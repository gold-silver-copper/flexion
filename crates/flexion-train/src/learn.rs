//! Learning one part of speech's grammar from its dictionary words: its
//! paradigms, the choice of paradigm, its stress, and the forms no rule
//! gives.

use crate::choose::{self, Settings};
use crate::extract::Lexeme;
use crate::skeleton;
use flexion::pattern;
use flexion::{
    Accent, AccentPattern, Compound, Form, Grammar, Orthography, Paradigm, Place, Referral,
    StressClass, Written,
};
use std::borrow::Cow;
use std::collections::{BTreeMap, HashMap};

/// What the learner needs to know about a part of speech.
#[derive(Debug, Clone)]
pub struct Shape {
    pub cells: usize,
    pub lemma_cell: Option<usize>,
    /// Per cell, the cell a word without a form of its own there uses.
    pub fallback: Vec<Option<usize>>,
    pub capitalized: bool,
    /// How many values each inherent feature has.
    pub inherent: Vec<usize>,
}

/// A learned part of speech, ready to be printed.
#[derive(Debug, Clone)]
pub struct Learned<'s> {
    pub grammar: Grammar<'s, u16, u16, u8>,
    /// The inherent feature (its place among the part of speech's) the
    /// referrals depend on.
    pub feature: Option<usize>,
    /// Per paradigm, some of its words (its name first).
    pub members: Vec<Vec<String>>,
    /// Forms the grammar doesn't give, listed: `(word, cell, form)`.
    pub irregular: Vec<(String, usize, String)>,
    pub stats: Stats,
}

/// Counts for the report.
#[derive(Debug, Clone, Default)]
pub struct Stats {
    pub words: usize,
    pub paradigms: usize,
    pub accent_patterns: usize,
    pub irregular_forms: usize,
}

/// One dictionary word, read.
#[derive(Clone)]
struct Word {
    /// What it is looked up by: its letters, folded (`fox`).
    key: String,
    /// Its letters as the dictionary writes them (`fōx`).
    letters: String,
    /// Senses of its dictionary entry.
    senses: u16,
    /// How many of the dictionaries have the word.
    dictionaries: u8,
    /// Per inherent feature, its value (`u8::MAX` if the dictionary doesn't
    /// say).
    inherent: Vec<u8>,
    /// Per cell, its forms (letters and stress), the usual one first.
    cells: Vec<Vec<Written>>,
}

/// A word's analysis: its paradigm signature.
struct Analyzed {
    lemma: String,
    /// Per cell, the pattern of its usual form (with variants after `|`).
    cells: Vec<Option<String>>,
}

pub fn learn<'s>(
    orthography: Orthography<'s>,
    shape: &Shape,
    lexemes: &[Lexeme],
    settings: Settings,
) -> Learned<'s> {
    // One entry per word: the first the dictionaries give (the main
    // dictionary's, in its order).
    let key = |l: &Lexeme| orthography.key(&orthography.read(&l.full).letters);
    let mut dictionaries: HashMap<String, std::collections::BTreeSet<u8>> = HashMap::new();
    for l in lexemes {
        dictionaries.entry(key(l)).or_default().insert(l.source);
    }
    let mut by_order: Vec<&Lexeme> = lexemes.iter().collect();
    by_order.sort_by_key(|l| (l.source, l.order));
    let mut seen = std::collections::HashSet::new();
    let lexemes: Vec<Lexeme> = by_order
        .into_iter()
        .filter(|l| seen.insert(key(l)))
        .cloned()
        .collect();
    let lexemes = lexemes.as_slice();
    let words: Vec<Word> = lexemes
        .iter()
        .map(|l| Word {
            key: key(l),
            letters: orthography.read(&l.full).letters,
            senses: l.senses,
            dictionaries: dictionaries
                .get(&key(l))
                .map_or(1, |s| u8::try_from(s.len()).unwrap_or(1)),
            inherent: l.inherent.clone(),
            cells: (0..shape.cells)
                .map(|c| {
                    l.cells
                        .get(c)
                        .map(|fs| fs.iter().map(|f| orthography.read(f)).collect())
                        .unwrap_or_default()
                })
                .collect(),
        })
        .collect();
    // Words written in parts that inflect each part are the grammar of
    // their parts, not paradigms of their own.
    let compounds: Vec<(String, Compound)> = words.iter().filter_map(compound).collect();
    let compound_choice =
        (!compounds.is_empty()).then(|| choose::learn(&compounds, settings, |_, _| true));
    let parted: std::collections::HashSet<&str> = words
        .iter()
        .filter(|w| compound(w).is_some_and(|c| c.1 == Compound::Each))
        .map(|w| w.key.as_str())
        .collect();
    let (all_words, all_lexemes) = (words.as_slice(), lexemes);
    let words: Vec<Word> = all_words
        .iter()
        .filter(|w| !parted.contains(w.key.as_str()))
        .cloned()
        .collect();
    let found = referrals(&words, shape);
    let mut analyses: Vec<Analyzed> = words.iter().map(|w| analyze(w, shape)).collect();
    let mut feature_examples: Vec<(String, u8)> = Vec::new();
    if let Some((f, referrals)) = &found {
        for (w, a) in words.iter().zip(analyses.iter_mut()) {
            let value = w.inherent.get(*f).copied().filter(|&v| v != u8::MAX);
            let value = value.or_else(|| infer(w, referrals));
            if let Some(v) = value {
                feature_examples.push((w.key.clone(), v));
                refer(w, a, referrals, v);
            }
        }
    }
    let feature_choice = found
        .as_ref()
        .map(|_| choose::learn(&feature_examples, settings, |_, _| true));
    let (paradigms, assignment) = paradigms(&words, &analyses, shape);
    let paradigm_examples: Vec<(String, u16)> = words
        .iter()
        .zip(&assignment)
        .map(|(w, &p)| (w.key.clone(), p))
        .collect();
    let fits = |word: &str, p: u16| {
        paradigms
            .get(usize::from(p))
            .is_some_and(|x| x.stems(&orthography, word).is_some())
    };
    let paradigm_choice = choose::learn(&paradigm_examples, settings, fits);
    let (accent, stress_labels) = if orthography.stress_marks.is_empty() {
        (None, HashMap::new())
    } else {
        let (a, labels) = accent(
            &orthography,
            &words,
            &assignment,
            &paradigms,
            shape,
            settings,
        );
        (Some(a), labels)
    };
    // Spellings a key doesn't show and the word's paradigm doesn't write.
    let mut spellings: Vec<(Cow<'s, str>, Cow<'s, str>)> = Vec::new();
    for (w, &p) in words.iter().zip(&assignment) {
        if w.letters == w.key || spellings.last().is_some_and(|l| l.0 == w.key) {
            continue;
        }
        let written = paradigms.get(usize::from(p)).and_then(|par| {
            let stems = par.stems(&orthography, &w.key)?;
            Some(pattern::fill(&par.lemma, &stems))
        });
        if written.as_deref() != Some(w.letters.as_str()) {
            spellings.push((Cow::Owned(w.key.clone()), Cow::Owned(w.letters.clone())));
        }
    }
    spellings.sort();
    spellings.dedup_by(|a, b| a.0 == b.0);
    let could_hide = |key: &str| {
        key.chars()
            .any(|c| orthography.folded.iter().any(|e| e.1 == c))
    };
    let hiding = words.iter().filter(|w| could_hide(&w.key)).count();
    #[allow(clippy::cast_precision_loss)]
    let hidden = spellings.len() as f32 / hiding.max(1) as f32;
    let mut members: Vec<Vec<String>> = vec![Vec::new(); paradigms.len()];
    for (w, &p) in words.iter().zip(&assignment) {
        if let Some(m) = members.get_mut(usize::from(p))
            && m.len() < 8
        {
            m.push(w.letters.clone());
        }
    }
    let mut grammar = Grammar {
        orthography,
        cells: shape.cells,
        lemma_cell: shape.lemma_cell,
        capitalized: shape.capitalized,
        paradigms: Cow::Owned(paradigms),
        paradigm: paradigm_choice,
        referrals: Cow::Owned(found.as_ref().map(|f| f.1.clone()).unwrap_or_default()),
        feature: feature_choice,
        compound: compound_choice,
        accent,
        laws: None,
        spellings: Cow::Owned(spellings),
        hidden,
        irregular: Cow::Owned(Vec::new()),
        smoothing: settings.smoothing,
        sharpness: 1.0,
    };
    // A word the grammar gets wrong is listed in its choices: its paradigm,
    // its stress pattern and stem vowel. Forms still wrong are irregular.
    let wrong: std::collections::BTreeSet<String> = irregular(&grammar, all_lexemes, all_words)
        .into_iter()
        .map(|(k, _, _)| k)
        .collect();
    let mut paradigm_of: HashMap<&str, u16> = HashMap::new();
    for (w, &p) in words.iter().zip(&assignment) {
        paradigm_of.entry(w.key.as_str()).or_insert(p);
    }
    for key in &wrong {
        if let Some(&p) = paradigm_of.get(key.as_str()) {
            list(&mut grammar.paradigm, key, p);
        }
        if let (Some(a), Some(&class)) = (grammar.accent.as_mut(), stress_labels.get(key)) {
            list(&mut a.class, key, class);
        }
    }
    tidy(&mut grammar.paradigm);
    if let Some(a) = grammar.accent.as_mut() {
        tidy(&mut a.class);
    }
    let irregular = irregular(&grammar, all_lexemes, all_words);
    grammar.irregular = Cow::Owned(
        irregular
            .iter()
            .map(|(w, c, f)| {
                (
                    Cow::Owned(w.clone()),
                    u16::try_from(*c).unwrap_or(u16::MAX),
                    Cow::Owned(f.clone()),
                )
            })
            .collect(),
    );
    let stats = Stats {
        words: all_words.len(),
        paradigms: grammar.paradigms.len(),
        accent_patterns: grammar.accent.as_ref().map_or(0, |a| a.patterns.len()),
        irregular_forms: irregular.len(),
    };
    Learned {
        grammar,
        feature: found.map(|f| f.0),
        members,
        irregular,
        stats,
    }
}

/// How a word written in parts inflects, keyed by its first part: each
/// part when a form changes its first part. `None` for a word of one part.
fn compound(w: &Word) -> Option<(String, Compound)> {
    let (first, _) = w.letters.split_once('-')?;
    let (first_key, _) = w.key.split_once('-')?;
    let changes = w
        .cells
        .iter()
        .filter_map(|forms| forms.first())
        .filter_map(|f| f.letters.split_once('-'))
        .any(|(f, _)| f != first);
    Some((
        first_key.to_string(),
        if changes {
            Compound::Each
        } else {
            Compound::Last
        },
    ))
}

/// What the trainer writes for a referred cell while it learns paradigms.
const REFERRED: &str = "=";

/// Of the words with each value of a feature, the share whose form of a
/// cell must be one other cell's form for the cell to be a referral.
const REFERRAL_SHARE: f64 = 0.5;

/// Referrals, and the inherent feature they depend on: cells whose form is,
/// for most words with each value of the feature, the form of one earlier
/// cell, a different cell for different values (the accusative is the genitive of
/// animate nouns and the nominative of inanimate ones).
fn referrals(words: &[Word], shape: &Shape) -> Option<(usize, Vec<Referral<'static>>)> {
    fn form(w: &Word, c: usize) -> Option<&str> {
        w.cells
            .get(c)
            .and_then(|fs| fs.first())
            .map(|f| f.letters.as_str())
    }
    let mut best: Option<(usize, Vec<Referral<'static>>)> = None;
    for (f, &values) in shape.inherent.iter().enumerate() {
        let mut found = Vec::new();
        for cell in 0..shape.cells {
            let mut to = Vec::with_capacity(values);
            for v in 0..values {
                let with: Vec<&Word> = words
                    .iter()
                    .filter(|w| {
                        w.inherent.get(f).is_some_and(|&x| usize::from(x) == v)
                            && form(w, cell).is_some()
                    })
                    .collect();
                // An earlier cell, itself formed by the paradigm.
                let target = (0..cell)
                    .filter(|&t| {
                        !found
                            .iter()
                            .any(|r: &Referral<'_>| usize::from(r.cell) == t)
                    })
                    .map(|t| {
                        (
                            with.iter()
                                .filter(|w| form(w, t).is_some() && form(w, t) == form(w, cell))
                                .count(),
                            t,
                        )
                    })
                    .max_by_key(|&(n, t)| (n, std::cmp::Reverse(t)));
                #[allow(clippy::cast_precision_loss)]
                let share = |n: usize| n as f64 / with.len().max(1) as f64;
                match target {
                    Some((n, t)) if with.len() >= 50 && share(n) >= REFERRAL_SHARE => {
                        to.push(u16::try_from(t).unwrap_or(u16::MAX));
                    }
                    _ => break,
                }
            }
            let differ = to.iter().any(|t| Some(t) != to.first());
            if to.len() == values && differ {
                found.push(Referral {
                    cell: u16::try_from(cell).unwrap_or(u16::MAX),
                    to: Cow::Owned(to),
                });
            }
        }
        if best.as_ref().is_none_or(|b| found.len() > b.1.len()) && !found.is_empty() {
            best = Some((f, found));
        }
    }
    best
}

/// The value of a word the dictionary gives none for, from its forms: the
/// one whose referrals its forms follow, if just one.
fn infer(w: &Word, referrals: &[Referral<'_>]) -> Option<u8> {
    let form = |c: u16| {
        w.cells
            .get(usize::from(c))
            .and_then(|fs| fs.first())
            .map(|f| &f.letters)
    };
    let values = referrals.first()?.to.len();
    let fitting: Vec<usize> = (0..values)
        .filter(|&v| {
            referrals.iter().all(|r| {
                r.to.get(v)
                    .is_some_and(|&t| form(r.cell).is_none() || form(r.cell) == form(t))
            })
        })
        .collect();
    match fitting.as_slice() {
        [v] => u8::try_from(*v).ok(),
        _ => None,
    }
}

/// Mark the cells of `a` the word, of value `v`, forms by referral.
fn refer(w: &Word, a: &mut Analyzed, referrals: &[Referral<'_>], v: u8) {
    let form = |c: u16| {
        w.cells
            .get(usize::from(c))
            .and_then(|fs| fs.first())
            .map(|f| &f.letters)
    };
    for r in referrals {
        let Some(&t) = r.to.get(usize::from(v)) else {
            continue;
        };
        if form(r.cell).is_some()
            && form(r.cell) == form(t)
            && let Some(slot) = a.cells.get_mut(usize::from(r.cell))
        {
            *slot = Some(REFERRED.to_string());
        }
    }
}

/// A word's patterns over its own skeleton. Cells it has no form in that
/// fall back to another cell take that cell's pattern.
fn analyze(w: &Word, shape: &Shape) -> Analyzed {
    let mut forms: Vec<&str> = Vec::new();
    let mut slots: Vec<Vec<usize>> = vec![Vec::new(); shape.cells];
    for (c, fs) in w.cells.iter().enumerate() {
        for f in fs {
            if let Some(slot) = slots.get_mut(c) {
                slot.push(forms.len());
            }
            forms.push(&f.letters);
        }
    }
    let a = skeleton::analyze(&w.letters, &forms);
    let mut cells: Vec<Option<String>> = slots
        .iter()
        .map(|idx| {
            let patterns: Vec<&str> = idx
                .iter()
                .filter_map(|&i| a.forms.get(i).and_then(|p| p.as_deref()))
                .collect();
            (!patterns.is_empty()).then(|| patterns.join("|"))
        })
        .collect();
    for c in 0..shape.cells {
        if cells.get(c).is_some_and(Option::is_none)
            && let Some(Some(f)) = shape.fallback.get(c)
            && let Some(p) = cells.get(*f).cloned().flatten()
            && let Some(slot) = cells.get_mut(c)
        {
            *slot = Some(p);
        }
    }
    Analyzed {
        lemma: a.lemma,
        cells,
    }
}

/// The usual form's pattern of a cell pattern (`~ой|~ою` → `~ой`).
fn usual(p: &str) -> &str {
    pattern::variants(p).next().unwrap_or(p)
}

/// Paradigms, most words first, and each word's paradigm.
///
/// Every distinct signature (lemma pattern and the usual pattern of each
/// cell a word has) is a candidate. A word goes to the fullest candidate
/// that builds every form it has; of equally full ones, to the one most
/// common among words that end like it (its last three letters, then two,
/// then one), then the most common. Cells no word of a paradigm has take the
/// pattern of the most similar paradigm that has them.
fn paradigms<'s>(
    words: &[Word],
    analyses: &[Analyzed],
    shape: &Shape,
) -> (Vec<Paradigm<'s>>, Vec<u16>) {
    // Distinct signatures, and their words.
    let mut index: HashMap<(String, Vec<Option<String>>), usize> = HashMap::new();
    let mut signatures: Vec<(String, Vec<Option<String>>)> = Vec::new();
    let mut own: Vec<usize> = Vec::with_capacity(words.len());
    for a in analyses {
        let sig = (
            a.lemma.clone(),
            a.cells
                .iter()
                .map(|c| c.as_deref().map(|p| usual(p).to_string()))
                .collect(),
        );
        let id = *index.entry(sig.clone()).or_insert_with(|| {
            signatures.push(sig);
            signatures.len() - 1
        });
        own.push(id);
    }
    let fullness = |s: usize| {
        signatures
            .get(s)
            .map_or(0, |(_, c)| c.iter().flatten().count())
    };
    // Words per signature by ending.
    let mut by_ending: HashMap<(usize, String), u32> = HashMap::new();
    let mut size: Vec<u32> = vec![0; signatures.len()];
    for (w, &s) in words.iter().zip(&own) {
        if let Some(n) = size.get_mut(s) {
            *n += 1;
        }
        for n in 1..=3 {
            *by_ending.entry((s, last(&w.letters, n))).or_default() += 1;
        }
    }
    // Signatures by lemma pattern, fullest first; lemma patterns by their
    // last literal (what a word must end in to fit them).
    let mut by_lemma: HashMap<&str, Vec<usize>> = HashMap::new();
    for (i, (lemma, _)) in signatures.iter().enumerate() {
        by_lemma.entry(lemma.as_str()).or_default().push(i);
    }
    for list in by_lemma.values_mut() {
        list.sort_by_key(|&s| std::cmp::Reverse(fullness(s)));
    }
    let mut by_last: HashMap<&str, Vec<&str>> = HashMap::new();
    for &lp in by_lemma.keys() {
        let last_literal = lp.rsplit('~').next().unwrap_or("");
        by_last.entry(last_literal).or_default().push(lp);
    }
    let complete = |s: usize| fullness(s) == shape.cells;
    let mut chosen: Vec<usize> = Vec::with_capacity(words.len());
    for (w, &s) in words.iter().zip(&own) {
        // A word with every form keeps its own paradigm.
        if complete(s) {
            chosen.push(s);
            continue;
        }
        let mine = signatures.get(s);
        let mut best = (fullness(s), [0u32; 3], size.get(s).copied().unwrap_or(0), s);
        let fitting = choose::endings(&w.letters)
            .chain(std::iter::once(""))
            .filter_map(|e| by_last.get(e))
            .flatten();
        for &lp in fitting {
            let Some(stems) = pattern::bind(lp, &w.letters) else {
                continue;
            };
            for &cand in by_lemma.get(lp).into_iter().flatten() {
                // Only fuller paradigms complete it.
                if fullness(cand) <= fullness(s) || fullness(cand) < best.0 {
                    break;
                }
                let Some((_, cells)) = signatures.get(cand) else {
                    continue;
                };
                // It must build every form the word has.
                let builds = mine.is_some_and(|(_, mc)| {
                    mc.iter()
                        .zip(cells)
                        .enumerate()
                        .all(|(c, (m, p))| match (m, p) {
                            (None, _) => true,
                            (Some(_), None) => false,
                            (Some(m), Some(p)) if p == REFERRED => m == REFERRED,
                            (Some(_), Some(p)) => w
                                .cells
                                .get(c)
                                .and_then(|fs| fs.first())
                                .is_none_or(|f| pattern::fill(p, &stems) == f.letters),
                        })
                });
                if !builds {
                    continue;
                }
                let ends = [3, 2, 1].map(|n| {
                    by_ending
                        .get(&(cand, last(&w.letters, n)))
                        .copied()
                        .unwrap_or(0)
                });
                let key = (
                    fullness(cand),
                    ends,
                    size.get(cand).copied().unwrap_or(0),
                    cand,
                );
                if (key.0, key.1, key.2) > (best.0, best.1, best.2) {
                    best = key;
                }
            }
        }
        chosen.push(best.3);
    }
    // A paradigm needs MIN_WORDS words; a word of a rarer one inflects like
    // the paradigm that builds the most of its forms (its other forms are
    // then listed as irregular).
    let mut members: HashMap<usize, u32> = HashMap::new();
    for &c in &chosen {
        *members.entry(c).or_default() += 1;
    }
    let supported = |s: usize| members.get(&s).copied().unwrap_or(0) >= MIN_WORDS;
    let mut supported_by_lemma: HashMap<&str, Vec<usize>> = HashMap::new();
    for (i, (lemma, _)) in signatures.iter().enumerate() {
        if supported(i) {
            supported_by_lemma
                .entry(lemma.as_str())
                .or_default()
                .push(i);
        }
    }
    let mut supported_last: HashMap<&str, Vec<&str>> = HashMap::new();
    for &lp in supported_by_lemma.keys() {
        supported_last
            .entry(lp.rsplit('~').next().unwrap_or(""))
            .or_default()
            .push(lp);
    }
    for (i, w) in words.iter().enumerate() {
        let Some(&c) = chosen.get(i) else {
            continue;
        };
        if supported(c) {
            continue;
        }
        let mine = analyses.get(i);
        let mut best: Option<((usize, u32), usize)> = None;
        let fitting = choose::endings(&w.letters)
            .chain(std::iter::once(""))
            .filter_map(|e| supported_last.get(e))
            .flatten();
        for &lp in fitting {
            let Some(stems) = pattern::bind(lp, &w.letters) else {
                continue;
            };
            for &cand in supported_by_lemma.get(lp).into_iter().flatten() {
                let Some((_, cells)) = signatures.get(cand) else {
                    continue;
                };
                let built = cells
                    .iter()
                    .enumerate()
                    .filter(|(cell, p)| {
                        let want = w.cells.get(*cell).and_then(|fs| fs.first());
                        match (p, want) {
                            (Some(p), Some(_)) if p == REFERRED => mine
                                .and_then(|a| a.cells.get(*cell))
                                .is_some_and(|m| m.as_deref() == Some(REFERRED)),
                            (Some(p), Some(f)) => pattern::fill(usual(p), &stems) == f.letters,
                            _ => false,
                        }
                    })
                    .count();
                let key = (built, members.get(&cand).copied().unwrap_or(0));
                if best.is_none_or(|(b, _)| key > b) {
                    best = Some((key, cand));
                }
            }
        }
        if let (Some((_, cand)), Some(slot)) = (best, chosen.get_mut(i)) {
            *slot = cand;
        }
    }
    // Paradigms: the chosen signatures, most words first.
    let mut count: BTreeMap<usize, u32> = BTreeMap::new();
    for &c in &chosen {
        *count.entry(c).or_default() += 1;
    }
    let mut order: Vec<(usize, u32)> = count.into_iter().collect();
    order.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    let id_of: HashMap<usize, u16> = order
        .iter()
        .enumerate()
        .map(|(i, &(s, _))| (s, u16::try_from(i).unwrap_or(u16::MAX)))
        .collect();
    // Variants: the alternatives most of a paradigm's words list in a cell.
    let mut variants: HashMap<(usize, usize), BTreeMap<String, u32>> = HashMap::new();
    for (a, &c) in analyses.iter().zip(&chosen) {
        for (cell, p) in a.cells.iter().enumerate() {
            if let Some(p) = p {
                *variants
                    .entry((c, cell))
                    .or_default()
                    .entry(p.clone())
                    .or_default() += 1;
            }
        }
    }
    let mut out: Vec<Paradigm<'s>> = Vec::with_capacity(order.len());
    for &(s, n) in &order {
        let Some((lemma, cells)) = signatures.get(s) else {
            continue;
        };
        let name = name(words, &chosen, s);
        let forms: Vec<Form<'s>> = cells
            .iter()
            .enumerate()
            .map(|(cell, p)| {
                let usual_p = match p {
                    Some(p) => p.clone(),
                    None => borrow_cell(&signatures, &order, s, cell, shape)
                        .unwrap_or_else(|| lemma.clone()),
                };
                // The usual pattern, with the alternatives most words list.
                let full = variants
                    .get(&(s, cell))
                    .and_then(|v| v.iter().max_by_key(|e| *e.1))
                    .filter(|e| usual(e.0) == usual_p && 2 * *e.1 > n)
                    .map_or(usual_p, |(p, _)| p.clone());
                if full == REFERRED {
                    Form::Referred
                } else {
                    Form::Pattern(Cow::Owned(full))
                }
            })
            .collect();
        out.push(Paradigm {
            name: Cow::Owned(name),
            words: n,
            lemma: Cow::Owned(lemma.clone()),
            forms: Cow::Owned(forms),
        });
    }
    let assignment = chosen
        .iter()
        .map(|c| id_of.get(c).copied().unwrap_or(0))
        .collect();
    (out, assignment)
}

/// A pattern for `cell`, which no word of signature `s` has: from the chosen
/// paradigm with the same lemma pattern and number of stem pieces that agrees
/// with `s` on the most cells and has the cell (the cell's fallback first).
fn borrow_cell(
    signatures: &[(String, Vec<Option<String>>)],
    order: &[(usize, u32)],
    s: usize,
    cell: usize,
    shape: &Shape,
) -> Option<String> {
    let (lemma, cells) = signatures.get(s)?;
    if let Some(Some(f)) = shape.fallback.get(cell)
        && let Some(Some(p)) = cells.get(*f)
    {
        return Some(p.clone());
    }
    let pieces = pattern::pieces(lemma);
    order
        .iter()
        .filter_map(|&(o, n)| {
            let (l, c) = signatures.get(o)?;
            let p = c.get(cell)?.as_ref()?;
            (pattern::pieces(l) == pieces).then_some(())?;
            let agree = cells
                .iter()
                .zip(c)
                .filter(|(a, b)| a.is_some() && a == b)
                .count();
            Some(((l == lemma, agree, n), p.clone()))
        })
        .max_by(|a, b| a.0.cmp(&b.0))
        .map(|(_, p)| p)
}

/// The fewest words a paradigm or a stress pattern is learned from.
const MIN_WORDS: u32 = 3;

/// A paradigm's name: its commonest word, judged by how many dictionaries
/// have it and how many senses its entry has (then the shortest, then the
/// first alphabetically), of its plain words (letters only) of at least
/// three letters if it has any.
fn name(words: &[Word], chosen: &[usize], s: usize) -> String {
    let mine = || {
        words
            .iter()
            .zip(chosen)
            .filter(|&(_, &c)| c == s)
            .map(|(w, _)| w)
    };
    let pick = |plain: bool| {
        mine()
            .filter(|w| {
                !plain
                    || (w.letters.chars().count() >= 3
                        && w.letters.chars().all(char::is_alphabetic))
            })
            .max_by(|a, b| {
                (a.dictionaries, a.senses)
                    .cmp(&(b.dictionaries, b.senses))
                    .then_with(|| b.letters.chars().count().cmp(&a.letters.chars().count()))
                    .then_with(|| b.letters.cmp(&a.letters))
            })
            .map(|w| w.letters.clone())
    };
    pick(true).or_else(|| pick(false)).unwrap_or_default()
}

/// The last `n` letters of `word`.
fn last(word: &str, n: usize) -> String {
    let len = word.chars().count();
    word.chars().skip(len.saturating_sub(n)).collect()
}

/// The words' stress: a pattern per word (per cell, stem or ending), and the
/// stem's stressed vowel.
fn accent<'s>(
    orthography: &Orthography<'_>,
    words: &[Word],
    assignment: &[u16],
    paradigms: &[Paradigm<'s>],
    shape: &Shape,
    settings: Settings,
) -> (Accent<'s, u16>, HashMap<String, u16>) {
    // Per word: per cell, the places that give its stress; and its stem vowel.
    struct Seen {
        places: Vec<Option<(bool, bool)>>,
        stem_vowel: Option<u8>,
        complete: bool,
    }
    let mut seen: Vec<Seen> = Vec::with_capacity(words.len());
    for (w, &p) in words.iter().zip(assignment) {
        let Some(par) = paradigms.get(usize::from(p)) else {
            seen.push(Seen {
                places: vec![None; shape.cells],
                stem_vowel: None,
                complete: false,
            });
            continue;
        };
        let stems = par.stems(orthography, &w.letters).unwrap_or_default();
        // The stem's vowel: where cells stressed before their ending are.
        let mut stem_votes: BTreeMap<u8, u32> = BTreeMap::new();
        let mut facts: Vec<Option<(flexion::Stress, String, usize)>> = Vec::new();
        for c in 0..shape.cells {
            let fact = w.cells.get(c).and_then(|fs| fs.first()).and_then(|f| {
                let s = f.stress?;
                let pat = par.pattern(c).map(usual)?;
                let ending = pattern::ending_start(pat, &stems);
                Some((s, f.letters.clone(), ending))
            });
            if let Some((s, letters, ending)) = &fact {
                let at = vowel_at(orthography, letters, s.vowel);
                if at < *ending {
                    *stem_votes.entry(s.vowel).or_default() += 1;
                }
            }
            facts.push(fact);
        }
        let stem_vowel = stem_votes.iter().max_by_key(|e| *e.1).map(|e| *e.0);
        let places: Vec<Option<(bool, bool)>> = facts
            .iter()
            .map(|fact| {
                fact.as_ref().map(|(s, letters, ending)| {
                    let e = flexion::stress_place(orthography, letters, *ending, Place::Ending, 0)
                        == Some(*s);
                    let st = match stem_vowel {
                        Some(k) => {
                            flexion::stress_place(orthography, letters, *ending, Place::Stem, k)
                                == Some(*s)
                        }
                        None => vowel_at(orthography, letters, s.vowel) < *ending,
                    };
                    (st, e)
                })
            })
            .collect();
        let complete = places
            .iter()
            .enumerate()
            .all(|(c, p)| p.is_some() || shape.fallback.get(c).is_some_and(Option::is_some));
        seen.push(Seen {
            places,
            stem_vowel,
            complete,
        });
    }
    // Patterns: the stress of complete words, each cell on the ending where
    // the ending gives its stress; kept if MIN_WORDS words have it.
    let fits = |pat: &[Place], places: &[Option<(bool, bool)>]| {
        pat.iter().zip(places).all(|(pl, p)| match (pl, p) {
            (_, None) => true,
            (Place::Stem, Some((s, _))) => *s,
            (Place::Ending, Some((_, e))) => *e,
        })
    };
    let agreement = |pat: &[Place], places: &[Option<(bool, bool)>]| {
        pat.iter()
            .zip(places)
            .filter(|(pl, p)| match (pl, p) {
                (Place::Stem, Some((st, _))) => *st,
                (Place::Ending, Some((_, e))) => *e,
                _ => false,
            })
            .count()
    };
    let mut vectors: BTreeMap<Vec<Place>, u32> = BTreeMap::new();
    for s in seen.iter().filter(|s| s.complete) {
        let pat: Vec<Place> = s
            .places
            .iter()
            .map(|p| match p {
                Some((_, true)) => Place::Ending,
                _ => Place::Stem,
            })
            .collect();
        *vectors.entry(pat).or_default() += 1;
    }
    let mut candidates: Vec<(Vec<Place>, u32)> = vectors
        .into_iter()
        .filter(|&(_, n)| n >= MIN_WORDS)
        .collect();
    candidates.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    // Each word: the commonest pattern that fits it, else the one agreeing
    // with it on the most cells (its other cells' forms are then listed).
    let mut patterns: Vec<(Vec<Place>, u32)> =
        candidates.iter().map(|(p, _)| (p.clone(), 0)).collect();
    let mut of_word: Vec<Option<usize>> = vec![None; words.len()];
    for (i, s) in seen.iter().enumerate() {
        if s.places.iter().all(Option::is_none) {
            continue;
        }
        let k = candidates
            .iter()
            .position(|(pat, _)| fits(pat, &s.places))
            .or_else(|| {
                candidates
                    .iter()
                    .enumerate()
                    .max_by_key(|(_, (pat, n))| (agreement(pat, &s.places), *n))
                    .map(|(k, _)| k)
            });
        if let Some(k) = k {
            if let Some((_, n)) = patterns.get_mut(k) {
                *n += 1;
            }
            if let Some(slot) = of_word.get_mut(i) {
                *slot = Some(k);
            }
        }
    }
    // Most common first.
    let mut order: Vec<usize> = (0..patterns.len()).collect();
    order.sort_by(|&a, &b| {
        let na = patterns.get(a).map_or(0, |p| p.1);
        let nb = patterns.get(b).map_or(0, |p| p.1);
        nb.cmp(&na).then_with(|| a.cmp(&b))
    });
    let renumber: HashMap<usize, u16> = order
        .iter()
        .enumerate()
        .map(|(new, &old)| (old, u16::try_from(new).unwrap_or(u16::MAX)))
        .collect();
    let accent_patterns: Vec<AccentPattern<'s>> = order
        .iter()
        .filter_map(|&k| patterns.get(k))
        .enumerate()
        .map(|(i, (pat, n))| AccentPattern {
            name: Cow::Owned(format!("{}", i + 1)),
            words: *n,
            places: Cow::Owned(pat.clone()),
        })
        .collect();
    // Classes: a pattern and the stem's stressed vowel, from the lemma's end.
    let mut class_of: Vec<Option<StressClass>> = Vec::with_capacity(words.len());
    for ((w, k), s) in words.iter().zip(&of_word).zip(&seen) {
        let class = k.and_then(|k| renumber.get(&k)).map(|&pattern| {
            let n = orthography.vowel_count(&w.key);
            let vowel = s
                .stem_vowel
                .and_then(|k| n.checked_sub(1)?.checked_sub(usize::from(k)))
                .and_then(|v| u8::try_from(v).ok());
            StressClass {
                pattern,
                vowel,
                words: 0,
            }
        });
        class_of.push(class);
    }
    let mut counts: BTreeMap<(u16, Option<u8>), u32> = BTreeMap::new();
    for c in class_of.iter().flatten() {
        *counts.entry((c.pattern, c.vowel)).or_default() += 1;
    }
    let mut classes: Vec<StressClass> = counts
        .into_iter()
        .map(|((pattern, vowel), words)| StressClass {
            pattern,
            vowel,
            words,
        })
        .collect();
    classes.sort_by(|a, b| {
        b.words
            .cmp(&a.words)
            .then_with(|| (a.pattern, a.vowel).cmp(&(b.pattern, b.vowel)))
    });
    let id: HashMap<(u16, Option<u8>), u16> = classes
        .iter()
        .enumerate()
        .map(|(i, c)| ((c.pattern, c.vowel), u16::try_from(i).unwrap_or(u16::MAX)))
        .collect();
    let mut labels: HashMap<String, u16> = HashMap::new();
    let examples: Vec<(String, u16)> = words
        .iter()
        .zip(&class_of)
        .filter_map(|(w, c)| {
            let c = c.as_ref()?;
            Some((w.key.clone(), *id.get(&(c.pattern, c.vowel))?))
        })
        .collect();
    for (w, c) in &examples {
        labels.entry(w.clone()).or_insert(*c);
    }
    // A class whose stem vowel the word hasn't got can't be its.
    let class = choose::learn(&examples, settings, |word, c| {
        classes.get(usize::from(c)).is_some_and(|c| {
            c.vowel
                .is_none_or(|v| usize::from(v) < orthography.vowel_count(word))
        })
    });
    let accent = Accent {
        patterns: Cow::Owned(accent_patterns),
        classes: Cow::Owned(classes),
        class,
    };
    (accent, labels)
}

/// List `word` in `choice` as taking `value` (call [`tidy`] after).
fn list<L: flexion::Label + Ord>(choice: &mut flexion::Choice<'_, L>, word: &str, value: L) {
    choice
        .exceptions
        .to_mut()
        .push((Cow::Owned(word.to_string()), value));
}

/// Sort a choice's listed words and drop repeats.
fn tidy<L: flexion::Label + Ord>(choice: &mut flexion::Choice<'_, L>) {
    let list = choice.exceptions.to_mut();
    list.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));
    list.dedup();
}

/// The byte where the `vowel`-th vowel of `letters` is.
fn vowel_at(orthography: &Orthography<'_>, letters: &str, vowel: u8) -> usize {
    letters
        .char_indices()
        .filter(|&(_, c)| orthography.is_vowel(c))
        .nth(usize::from(vowel))
        .map_or(letters.len(), |(i, _)| i)
}

/// The dictionary's forms the grammar doesn't give, for the word typed as
/// the dictionary writes it and typed plain: `(key, cell, form)`, sorted.
/// Of homographs, the first word's forms.
fn irregular(
    grammar: &Grammar<'_, u16, u16, u8>,
    lexemes: &[Lexeme],
    words: &[Word],
) -> Vec<(String, usize, String)> {
    let mut first: BTreeMap<&str, usize> = BTreeMap::new();
    for (i, w) in words.iter().enumerate() {
        first.entry(w.key.as_str()).or_insert(i);
    }
    let firsts: Vec<usize> = first.into_values().collect();
    let chunk = firsts.len().div_ceil(THREADS).max(1);
    let found: Vec<Vec<(String, usize, String)>> = std::thread::scope(|scope| {
        let handles: Vec<_> = firsts
            .chunks(chunk)
            .map(|part| {
                scope.spawn(move || {
                    let mut out = Vec::new();
                    for &i in part {
                        let (Some(l), Some(w)) = (lexemes.get(i), words.get(i)) else {
                            continue;
                        };
                        let plain = grammar
                            .orthography
                            .write(&w.key, None, flexion::Casing::Lower);
                        for (cell, forms) in l.cells.iter().enumerate() {
                            let Some(want) = forms.first() else {
                                continue;
                            };
                            let right = [(l.full.as_str(), true), (plain.as_str(), false)]
                                .iter()
                                .all(|&(input, full)| {
                                    let known = flexion::Known {
                                        full,
                                        ..flexion::Known::default()
                                    };
                                    crate::eval::same_form(
                                        &grammar.orthography,
                                        &grammar.best(input, cell, &known),
                                        want,
                                    )
                                });
                            if !right {
                                let read = grammar.orthography.read(want);
                                let written = grammar.orthography.write(
                                    &read.letters,
                                    read.stress,
                                    flexion::Casing::Lower,
                                );
                                out.push((w.key.clone(), cell, written));
                            }
                        }
                    }
                    out
                })
            })
            .collect();
        handles.into_iter().filter_map(|h| h.join().ok()).collect()
    });
    let mut out: Vec<(String, usize, String)> = found.into_iter().flatten().collect();
    out.sort();
    out
}

/// Threads the learner uses at most.
const THREADS: usize = 8;
