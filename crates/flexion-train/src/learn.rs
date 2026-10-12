//! Learning one part of speech's grammar from its dictionary words: its
//! paradigms, the choice of paradigm, its stress, and the forms no rule
//! gives.

use crate::HashMap;
use crate::choose::{self, Settings};
use crate::extract::Lexeme;
use crate::skeleton;
use flexion::Finals;
use flexion::pattern;
use flexion::{
    Accent, AccentPattern, Compound, DERIVED, Grammar, Orthography, Paradigm, Place, Relation,
    StressClass, Written,
};
use std::borrow::Cow;
use std::collections::BTreeMap;

/// What the learner needs to know about a part of speech.
#[derive(Debug, Clone)]
pub struct Shape {
    pub cells: usize,
    pub lemma_cell: Option<usize>,
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
    /// Per paradigm, the cells its words have forms in.
    pub seen: Vec<Vec<bool>>,
    /// The sound laws at the stem's edge, with their evidence.
    pub laws: Vec<crate::laws::Law>,
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
    /// A sense of it is vulgar or offensive.
    vulgar: bool,
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
    let mut dictionaries: HashMap<String, std::collections::BTreeSet<u8>> = HashMap::default();
    for l in lexemes {
        dictionaries.entry(key(l)).or_default().insert(l.source);
    }
    let mut by_order: Vec<&Lexeme> = lexemes.iter().collect();
    by_order.sort_by_key(|l| (l.source, l.order));
    let mut seen = crate::HashSet::default();
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
            vulgar: l.vulgar,
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
    let parted: crate::HashSet<&str> = words
        .iter()
        .filter(|w| compound(w).is_some_and(|c| c.1 == Compound::Each))
        .map(|w| w.key.as_str())
        .collect();
    let (all_words, all_lexemes) = (words.as_slice(), lexemes);
    // Words that separate a particle are the grammar of the rest of them:
    // their bases are learned with the other words.
    let (prefixes, bases) = prefixes(all_words);
    let separated: crate::HashSet<&str> = bases.iter().map(|(k, _)| k.as_str()).collect();
    let mut words: Vec<Word> = all_words
        .iter()
        .filter(|w| !parted.contains(w.key.as_str()) && !separated.contains(w.key.as_str()))
        .cloned()
        .collect();
    // A base the dictionary has takes the forms its particle verbs show in
    // the cells it lacks (`aufzunehmen` shows `nehmen`'s `zunehmen`).
    let mut at: HashMap<String, usize> = words
        .iter()
        .enumerate()
        .map(|(i, w)| (w.key.clone(), i))
        .collect();
    for (_, base) in bases {
        match at.get(&base.key).and_then(|&i| words.get_mut(i)) {
            Some(known) => {
                for (mine, theirs) in known.cells.iter_mut().zip(base.cells) {
                    if mine.is_empty() {
                        *mine = theirs;
                    }
                }
            }
            None => {
                at.insert(base.key.clone(), words.len());
                words.push(base);
            }
        }
    }
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
    // Each cell's relation to the cell its form is best told from; the
    // referrals (by the word's feature) take their cells' place.
    let usual: Vec<Vec<Option<&str>>> = words
        .iter()
        .map(|w| {
            w.cells
                .iter()
                .map(|fs| fs.first().map(|f| f.letters.as_str()))
                .collect()
        })
        .collect();
    let root = shape.lemma_cell.unwrap_or(0);
    let mut relations = crate::relations::learn(&usual, shape.cells, root, settings);
    if let Some((_, referrals)) = &found {
        relations.retain(|r| !referrals.iter().any(|q| q.cell == r.cell));
        relations.extend(referrals.iter().cloned());
    }
    relations.sort_by_key(|r| r.cell);
    let (paradigms, assignment) = paradigms(&words, &analyses, shape, &relations);
    // Paradigms that differ only by a change where stem meets ending are
    // one paradigm and a sound law.
    let law_words: Vec<crate::laws::Word<'_>> = words
        .iter()
        .zip(&analyses)
        .map(|(w, a)| crate::laws::Word {
            letters: &w.letters,
            forms: w
                .cells
                .iter()
                .zip(&a.cells)
                .map(|(fs, p)| {
                    fs.first()
                        .filter(|_| p.as_deref() != Some(DERIVED))
                        .map(|f| f.letters.as_str())
                })
                .collect(),
        })
        .collect();
    let (laws, program, mut paradigms, assignment) =
        crate::laws::learn(&orthography, &law_words, paradigms, assignment);
    inherent(&words, &assignment, &mut paradigms);
    let mut seen = vec![vec![false; shape.cells]; paradigms.len()];
    for (w, &p) in words.iter().zip(&assignment) {
        for (c, fs) in w.cells.iter().enumerate() {
            if let Some(slot) = seen.get_mut(usize::from(p)).and_then(|s| s.get_mut(c)) {
                *slot |= !fs.is_empty();
            }
        }
    }
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
        (None, HashMap::default())
    } else {
        let (a, labels) = accent(
            &orthography,
            &words,
            &assignment,
            &paradigms,
            program.as_ref(),
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
    // How a letter typed without its mark is written, by what follows it;
    // the words it doesn't spell are listed.
    let marks = marks(&orthography, &words, settings);
    if let Some(marks) = &marks {
        spellings = words
            .iter()
            .filter(|w| {
                flexion::restore(&orthography, marks, settings.smoothing, &w.key, &w.key).0
                    != w.letters
            })
            .map(|w| (Cow::Owned(w.key.clone()), Cow::Owned(w.letters.clone())))
            .collect();
        spellings.sort();
        spellings.dedup_by(|a, b| a.0 == b.0);
    }
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
        relations: Cow::Owned(relations),
        feature: found
            .as_ref()
            .and_then(|f| u8::try_from(f.0).ok())
            .zip(feature_choice),
        compound: compound_choice,
        prefixes,
        accent,
        laws: program,
        spellings: Cow::Owned(spellings),
        marks,
        irregular: flexion::Irregular::default(),
        smoothing: settings.smoothing,
        sharpness: 1.0,
    };
    // A word the grammar gets wrong is listed in its choices: its paradigm,
    // its stress pattern and stem vowel. Forms still wrong are irregular.
    let wrong: std::collections::BTreeSet<String> =
        irregular(&grammar, all_lexemes, all_words, |_| true)
            .into_iter()
            .map(|(k, _, _)| k)
            .collect();
    let mut paradigm_of: HashMap<&str, u16> = HashMap::default();
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
    // Words made of others (`hineinlassen`, `диван-кровать`) are checked
    // once the words they are made of have their irregular forms.
    let made = |w: &Word| {
        w.key.contains('-')
            || grammar
                .prefixes
                .as_ref()
                .and_then(|p| p.split(&w.key))
                .is_some_and(|(_, separable, inseparable)| separable + inseparable > 0.0)
    };
    // A word's listed forms are its principal parts: the forms no relation
    // tells from another listed one. The forms told wrong are listed too.
    let wrong = irregular(&grammar, all_lexemes, all_words, |w| !made(w));
    if let Cow::Owned(relations) = &mut grammar.relations {
        for r in relations.iter_mut() {
            r.listed = tells_listed(r, &wrong, all_words, grammar.smoothing);
        }
    }
    let mut irregular = roots(&grammar, &wrong);
    grammar.irregular = flexion::Irregular::new(&irregular);
    for _ in 0..ROUNDS {
        let listed: crate::HashSet<String> = irregular.iter().map(|(k, _, _)| k.clone()).collect();
        let more = self::irregular(&grammar, all_lexemes, all_words, |w| {
            !made(w) && listed.contains(&w.key)
        });
        if more.is_empty() {
            break;
        }
        irregular.extend(more);
        irregular.sort();
        grammar.irregular = flexion::Irregular::new(&irregular);
    }
    irregular.extend(self::irregular(&grammar, all_lexemes, all_words, made));
    irregular.sort();
    grammar.irregular = flexion::Irregular::new(&irregular);
    if let Cow::Owned(paradigms) = &mut grammar.paradigms {
        inherit(paradigms);
    }
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
        seen,
        laws,
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

/// Make each paradigm like the earlier (commoner) one it shares the most
/// cells with, when that is at least half of them: its own table keeps only
/// the cells that differ. Every paradigm still has the same forms.
fn inherit(paradigms: &mut [Paradigm<'_>]) {
    let cells: Vec<Vec<String>> = paradigms
        .iter()
        .map(|p| p.table.split('\t').map(str::to_string).collect())
        .collect();
    let pieces: Vec<usize> = paradigms
        .iter()
        .map(|p| pattern::pieces(&p.lemma))
        .collect();
    for (i, paradigm) in paradigms.iter_mut().enumerate() {
        let (Some(mine), Some(&my_pieces)) = (cells.get(i), pieces.get(i)) else {
            continue;
        };
        let shared = |j: usize| {
            cells.get(j).map_or(0, |theirs| {
                mine.iter()
                    .zip(theirs)
                    .filter(|(a, b)| a == b && !a.is_empty())
                    .count()
            })
        };
        let best = (0..i)
            .filter(|&j| pieces.get(j) == Some(&my_pieces))
            .map(|j| (shared(j), j))
            .max_by(|a, b| a.0.cmp(&b.0).then(b.1.cmp(&a.1)));
        let filled = mine.iter().filter(|c| !c.is_empty()).count();
        let Some((n, j)) = best.filter(|&(n, _)| n > 0 && 2 * n >= filled) else {
            continue;
        };
        let Some(theirs) = cells.get(j) else { continue };
        let _ = n;
        let table: String = mine
            .iter()
            .zip(theirs.iter().map(Some).chain(std::iter::repeat(None)))
            .take(mine.len().saturating_sub(1))
            .map(|(m, t)| {
                if t == Some(m) {
                    "\t".to_string()
                } else {
                    format!("{m}\t")
                }
            })
            .collect();
        paradigm.parent = u16::try_from(j).ok();
        paradigm.table = Cow::Owned(table);
    }
}

/// The letters of a word's usual form in cell `c`.
fn first_form(w: &Word, c: usize) -> Option<&str> {
    w.cells
        .get(c)
        .and_then(|fs| fs.first())
        .map(|f| f.letters.as_str())
}

/// The shortest and longest prefix, in letters.
const PREFIX_LETTERS: std::ops::RangeInclusive<usize> = 2..=12;

/// Prefixes words inflect behind, and the words made of one with their
/// bases: `(word's key, base)`. Only where words separate prefixes: a
/// dictionary that never writes one apart has none to learn (a prefix it
/// keeps in front is no more than the start of a word).
///
/// A word separates a prefix when a form of it puts the prefix after the
/// rest, as a word of its own (`räuchre aus`). It keeps one in front when
/// every form begins with it and the rest is a word whose forms, with what
/// the cell's inseparable prefixes take off (the participle's `ge`), are the
/// rest of its forms (`verhindern`, `hindern`); the cells' drops are learned
/// from such pairs. A word's base has its forms without the prefix (and with
/// what was taken off put back).
fn prefixes<'a>(words: &[Word]) -> (Option<flexion::Prefixes<'a>>, Vec<(String, Word)>) {
    use flexion::Separation;
    let cells = words.first().map_or(0, |w| w.cells.len());
    let form = first_form;
    let by_key: HashMap<&str, &Word> = words.iter().map(|w| (w.key.as_str(), w)).collect();
    // Separable: a form with the prefix after the rest.
    let mut separable: HashMap<&str, String> = HashMap::default();
    for w in words {
        let prefix = w.cells.iter().filter_map(|fs| fs.first()).find_map(|f| {
            let (_, tail) = f.letters.rsplit_once(' ')?;
            (!tail.is_empty() && w.letters.starts_with(tail) && w.letters.len() > tail.len())
                .then_some(tail)
        });
        if let Some(p) = prefix {
            separable.insert(w.key.as_str(), p.to_string());
        }
    }
    // What an inseparable prefix takes off each cell: from words made of a
    // prefix and a word of the dictionary.
    let split = |w: &Word, n: usize| -> Option<(String, String)> {
        let at = w.letters.char_indices().nth(n)?.0;
        let (p, rest) = w.letters.split_at(at);
        (rest.chars().count() >= 3).then(|| (p.to_string(), rest.to_string()))
    };
    let mut drops: Vec<BTreeMap<String, u32>> = vec![BTreeMap::new(); cells];
    for w in words
        .iter()
        .filter(|w| !separable.contains_key(w.key.as_str()))
    {
        for n in PREFIX_LETTERS {
            let Some((p, rest)) = split(w, n) else { break };
            let Some(base) = by_key.get(rest.as_str()) else {
                continue;
            };
            for (c, votes) in drops.iter_mut().enumerate() {
                let (Some(mine), Some(theirs)) = (form(w, c), form(base, c)) else {
                    continue;
                };
                let Some(tail) = mine.strip_prefix(p.as_str()) else {
                    continue;
                };
                if let Some(dropped) = theirs.strip_suffix(tail) {
                    *votes.entry(dropped.to_string()).or_default() += 1;
                }
            }
        }
    }
    let drops: Vec<String> = drops
        .iter()
        .map(|v| {
            v.iter()
                .max_by_key(|e| *e.1)
                .map(|e| e.0.clone())
                .unwrap_or_default()
        })
        .collect();
    let drop = |c: usize| drops.get(c).map_or("", String::as_str);
    // Inseparable: every form begins with the prefix and, with what the cell
    // takes off put back, its rest is the base's form where the dictionary
    // has the base.
    let keeps = |w: &Word, p: &str, rest: &str| {
        let base = by_key.get(rest);
        let mut seen = 0;
        for c in 0..cells {
            let Some(mine) = form(w, c) else { continue };
            let Some(tail) = mine.strip_prefix(p) else {
                return false;
            };
            if !drop(c).is_empty() && tail.starts_with(drop(c)) {
                return false;
            }
            if let Some(theirs) = base.and_then(|b| form(b, c))
                && theirs != format!("{}{tail}", drop(c))
            {
                return false;
            }
            seen += 1;
        }
        seen > 0
    };
    let mut inseparable: HashMap<&str, String> = HashMap::default();
    let mut proven: BTreeMap<String, u32> = BTreeMap::new();
    for w in words
        .iter()
        .filter(|w| !separable.contains_key(w.key.as_str()))
    {
        for n in PREFIX_LETTERS {
            let Some((p, rest)) = split(w, n) else { break };
            if by_key.contains_key(rest.as_str()) && keeps(w, &p, &rest) {
                *proven.entry(p).or_default() += 1;
                break;
            }
        }
    }
    // A prefix needs MIN_WORDS words that separate it or that keep it, with a
    // base the dictionary has.
    let mut counted: BTreeMap<String, (u32, u32)> = BTreeMap::new();
    for p in separable.values() {
        counted.entry(p.clone()).or_default().0 += 1;
    }
    for (p, n) in &proven {
        if *n >= MIN_WORDS {
            counted.entry(p.clone()).or_default();
        }
    }
    counted.retain(|p, (s, _)| *s >= MIN_WORDS || proven.get(p).is_some_and(|&n| n >= MIN_WORDS));
    if !counted.values().any(|(s, _)| *s >= MIN_WORDS) {
        return (None, Vec::new());
    }
    // Every word that begins with a prefix: what it does with it.
    let longest = |key: &str| {
        counted
            .keys()
            .filter(|p| key.starts_with(p.as_str()) && key.chars().count() >= p.chars().count() + 3)
            .max_by_key(|p| p.len())
            .cloned()
    };
    let mut bases: Vec<(String, Word)> = Vec::new();
    let mut what: Vec<(String, String, Separation)> = Vec::new();
    for w in words {
        let Some(p) = longest(&w.key) else { continue };
        let n = p.chars().count();
        let rest: String = w.letters.chars().skip(n).collect();
        let prefix: String = w.letters.chars().take(n).collect();
        let separation = if separable.get(w.key.as_str()).is_some_and(|s| *s == prefix) {
            Separation::Separable
        } else if keeps(w, &prefix, &rest) {
            inseparable.insert(w.key.as_str(), prefix.clone());
            Separation::Inseparable
        } else {
            Separation::Neither
        };
        if separation != Separation::Neither {
            let mut base = w.clone();
            base.letters = rest;
            base.key = w.key.chars().skip(n).collect();
            for (c, forms) in base.cells.iter_mut().enumerate() {
                for f in forms.iter_mut() {
                    let rest = match separation {
                        Separation::Separable => f
                            .letters
                            .strip_suffix(prefix.as_str())
                            .and_then(|s| s.strip_suffix(' '))
                            .or_else(|| f.letters.strip_prefix(prefix.as_str()))
                            .map(str::to_string),
                        _ => f
                            .letters
                            .strip_prefix(prefix.as_str())
                            .map(|t| format!("{}{t}", drop(c))),
                    };
                    if let Some(rest) = rest {
                        f.letters = rest;
                        f.stress = None;
                    }
                }
            }
            bases.push((w.key.clone(), base));
        }
        what.push((p, w.key.clone(), separation));
    }
    // Where a separable prefix goes, cell by cell.
    let mut votes: Vec<[u32; 2]> = vec![[0; 2]; cells];
    for w in words {
        let Some(p) = separable.get(w.key.as_str()) else {
            continue;
        };
        for (c, v) in votes.iter_mut().enumerate() {
            match form(w, c) {
                Some(f) if f.ends_with(&format!(" {p}")) => v[1] += 1,
                Some(f) if f.starts_with(p.as_str()) => v[0] += 1,
                _ => {}
            }
        }
    }
    // Each prefix's counts; words that go against most are listed.
    let mut prefixes: Vec<flexion::Prefix<'a>> = Vec::new();
    let mut listed: Vec<(Cow<'a, str>, Separation)> = Vec::new();
    for p in counted.keys() {
        let mine: Vec<&(String, String, Separation)> = what.iter().filter(|e| e.0 == *p).collect();
        let count = |s: Separation| {
            u32::try_from(mine.iter().filter(|e| e.2 == s).count()).unwrap_or(u32::MAX)
        };
        let (sep, insep, neither) = (
            count(Separation::Separable),
            count(Separation::Inseparable),
            count(Separation::Neither),
        );
        let most = [
            (sep, Separation::Separable),
            (insep, Separation::Inseparable),
            (neither, Separation::Neither),
        ]
        .into_iter()
        .max_by_key(|e| e.0)
        .map_or(Separation::Neither, |e| e.1);
        for (_, key, s) in &mine {
            if *s != most {
                listed.push((Cow::Owned(key.clone()), *s));
            }
        }
        prefixes.push(flexion::Prefix {
            letters: Cow::Owned(p.clone()),
            separable: sep,
            inseparable: insep,
            words: sep + insep + neither,
        });
    }
    listed.sort();
    listed.dedup_by(|a, b| a.0 == b.0);
    let placement = votes
        .iter()
        .map(|v| {
            if v[1] > v[0] {
                flexion::Placement::After
            } else {
                flexion::Placement::Before
            }
        })
        .collect();
    let prefixes = flexion::Prefixes {
        prefixes: Cow::Owned(prefixes),
        listed: Cow::Owned(listed),
        placement: Cow::Owned(placement),
        drops: Cow::Owned(drops.into_iter().map(Cow::Owned).collect()),
    };
    (Some(prefixes), bases)
}

/// How a letter typed without its mark is written (see
/// [`flexion::Grammar::marks`]): learned from every letter of the words that
/// a mark could stand on, by the letter before it and the letters from it
/// on. `None`
/// where no word writes such a mark.
fn marks<'a>(
    orthography: &Orthography<'_>,
    words: &[Word],
    settings: Settings,
) -> Option<flexion::Choice<'a, char>> {
    let markable = |c: char| orthography.folded.iter().any(|e| e.1 == c);
    let mut examples: Vec<(String, char)> = Vec::new();
    let mut marked = false;
    for w in words {
        let mut before = flexion::WORD_END;
        for ((i, k), l) in w.key.char_indices().zip(w.letters.chars()) {
            if markable(k)
                && let Some(rest) = w.key.get(i..)
            {
                marked |= l != k;
                examples.push((format!("{before}{rest}{}", flexion::WORD_END), l));
            }
            before = k;
        }
    }
    if !marked {
        return None;
    }
    let mut choice = choose::learn_starts(&examples, settings, |around, m| {
        around
            .chars()
            .nth(1)
            .is_some_and(|c| orthography.fold(m) == c)
    });
    // What the rules get wrong is a word's own spelling, listed with it.
    choice.exceptions = Cow::Owned(Vec::new());
    Some(choice)
}

/// Of the words with each value of a feature, the share whose form of a
/// cell must be one other cell's form for the cell to be a referral.
const REFERRAL_SHARE: f64 = 0.5;

/// Referrals, and the inherent feature they depend on: cells whose form is,
/// for most words with each value of the feature, the form of one earlier
/// cell, a different cell for different values (the accusative is the genitive of
/// animate nouns and the nominative of inanimate ones).
fn referrals<'s>(words: &[Word], shape: &Shape) -> Option<(usize, Vec<Relation<'s>>)> {
    let form = first_form;
    let mut best: Option<(usize, Vec<Relation<'s>>)> = None;
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
                            .any(|r: &Relation<'_>| usize::from(r.cell) == t)
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
                found.push(Relation {
                    cell: u16::try_from(cell).unwrap_or(u16::MAX),
                    from: Cow::Owned(to),
                    changes: Cow::Borrowed(&[]),
                    change: None,
                    listed: false,
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
fn infer(w: &Word, referrals: &[Relation<'_>]) -> Option<u8> {
    let form = |c: u16| {
        w.cells
            .get(usize::from(c))
            .and_then(|fs| fs.first())
            .map(|f| &f.letters)
    };
    let values = referrals.first()?.from.len();
    let fitting: Vec<usize> = (0..values)
        .filter(|&v| {
            referrals.iter().all(|r| {
                r.from
                    .get(v)
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
fn refer(w: &Word, a: &mut Analyzed, referrals: &[Relation<'_>], v: u8) {
    let form = |c: u16| {
        w.cells
            .get(usize::from(c))
            .and_then(|fs| fs.first())
            .map(|f| &f.letters)
    };
    for r in referrals {
        let Some(&t) = r.from.get(usize::from(v)) else {
            continue;
        };
        if form(r.cell).is_some()
            && form(r.cell) == form(t)
            && let Some(slot) = a.cells.get_mut(usize::from(r.cell))
        {
            *slot = Some(DERIVED.to_string());
        }
    }
}

/// A word's patterns over its own skeleton.
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
    let cells: Vec<Option<String>> = slots
        .iter()
        .map(|idx| {
            let patterns: Vec<&str> = idx
                .iter()
                .filter_map(|&i| a.forms.get(i).and_then(|p| p.as_deref()))
                .collect();
            (!patterns.is_empty()).then(|| patterns.join("|"))
        })
        .collect();
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
/// then one), then the most common. A cell no word of a paradigm has is
/// derived from another (see [`crate::relations`]), where one tells it.
fn paradigms<'s>(
    words: &[Word],
    analyses: &[Analyzed],
    shape: &Shape,
    relations: &[Relation<'_>],
) -> (Vec<Paradigm<'s>>, Vec<u16>) {
    // Distinct signatures, and their words.
    let mut index: HashMap<(String, Vec<Option<String>>), usize> = HashMap::default();
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
    let mut by_ending: HashMap<(usize, String), u32> = HashMap::default();
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
    let mut by_lemma: HashMap<&str, Vec<usize>> = HashMap::default();
    for (i, (lemma, _)) in signatures.iter().enumerate() {
        by_lemma.entry(lemma.as_str()).or_default().push(i);
    }
    for list in by_lemma.values_mut() {
        list.sort_by_key(|&s| std::cmp::Reverse(fullness(s)));
    }
    let mut by_last: HashMap<&str, Vec<&str>> = HashMap::default();
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
                            (Some(m), Some(p)) if p == DERIVED => m == DERIVED,
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
    let mut members: HashMap<usize, u32> = HashMap::default();
    for &c in &chosen {
        *members.entry(c).or_default() += 1;
    }
    let supported = |s: usize| members.get(&s).copied().unwrap_or(0) >= MIN_WORDS;
    let mut supported_by_lemma: HashMap<&str, Vec<usize>> = HashMap::default();
    for (i, (lemma, _)) in signatures.iter().enumerate() {
        if supported(i) {
            supported_by_lemma
                .entry(lemma.as_str())
                .or_default()
                .push(i);
        }
    }
    let mut supported_last: HashMap<&str, Vec<&str>> = HashMap::default();
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
                            (Some(p), Some(_)) if p == DERIVED => mine
                                .and_then(|a| a.cells.get(*cell))
                                .is_some_and(|m| m.as_deref() == Some(DERIVED)),
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
    let mut variants: HashMap<(usize, usize), BTreeMap<String, u32>> = HashMap::default();
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
        let table: String = cells
            .iter()
            .enumerate()
            .map(|(cell, p)| {
                let usual_p = match p {
                    Some(p) => p.clone(),
                    None if relations.iter().any(|r| usize::from(r.cell) == cell) => {
                        DERIVED.to_string()
                    }
                    None => lemma.clone(),
                };
                // The usual pattern, with the alternatives most words list.
                let full = variants
                    .get(&(s, cell))
                    .and_then(|v| v.iter().max_by_key(|e| *e.1))
                    .filter(|e| usual(e.0) == usual_p && 2 * *e.1 > n)
                    .map_or(usual_p, |(p, _)| p.clone());
                format!("{full}\t")
            })
            .collect();
        out.push(Paradigm {
            name: Cow::Owned(name),
            words: n,
            lemma: Cow::Owned(lemma.clone()),
            parent: None,
            inherent: Cow::Borrowed(&[]),
            table: Cow::Owned(table),
        });
    }
    let assignment = chosen
        .iter()
        .map(|c| id_of.get(c).copied().unwrap_or(0))
        .collect();
    (out, assignment)
}

/// Per paradigm, the inherent feature values (its gender) nineteen in
/// twenty of its words that have one share (at least [`MIN_WORDS`]).
fn inherent(words: &[Word], assignment: &[u16], paradigms: &mut [Paradigm<'_>]) {
    let mut counts: BTreeMap<(u16, u8, u8), u32> = BTreeMap::new();
    for (w, &p) in words.iter().zip(assignment) {
        for (f, &v) in w.inherent.iter().enumerate() {
            if let (Ok(f), true) = (u8::try_from(f), v != u8::MAX) {
                *counts.entry((p, f, v)).or_default() += 1;
            }
        }
    }
    let features = words.first().map_or(0, |w| w.inherent.len());
    for (p, paradigm) in paradigms.iter_mut().enumerate() {
        let Ok(p) = u16::try_from(p) else { continue };
        let mut values: Vec<(u8, u8)> = Vec::new();
        for f in (0..features).filter_map(|f| u8::try_from(f).ok()) {
            let of = counts.range((p, f, 0)..=(p, f, u8::MAX));
            let total: u32 = of.clone().map(|(_, n)| n).sum();
            if let Some((&(_, _, v), &n)) = of.max_by_key(|(_, n)| **n)
                && 20 * n >= 19 * total
                && n >= MIN_WORDS
            {
                values.push((f, v));
            }
        }
        paradigm.inherent = Cow::Owned(values);
    }
}

/// How many times the forms derivations from listed forms get wrong are
/// listed, at most.
const ROUNDS: usize = 3;

/// Whether relation `r` tells the forms of the words whose form of the cell
/// it comes from is `wrong` (sorted) right as often as their paradigm does.
fn tells_listed(
    r: &Relation<'_>,
    wrong: &[(String, usize, String)],
    words: &[Word],
    smoothing: f32,
) -> bool {
    let (Some(from), cell) = (r.source(), usize::from(r.cell)) else {
        return false;
    };
    let is_wrong = |key: &str, c: usize| {
        wrong
            .binary_search_by(|(k, x, _)| (k.as_str(), *x).cmp(&(key, c)))
            .is_ok()
    };
    let (mut told, mut built) = (0usize, 0usize);
    for w in words.iter().filter(|w| is_wrong(&w.key, from)) {
        let (Some(source), Some(want)) = (first_form(w, from), first_form(w, cell)) else {
            continue;
        };
        told += usize::from(r.told(source, smoothing).is_some_and(|t| t.0 == want));
        built += usize::from(!is_wrong(&w.key, cell));
    }
    told >= built
}

/// Of `listed` forms (`(word, cell, form)`, sorted), those no relation
/// tells from another listed form of the word, or one told so in turn.
fn roots(
    grammar: &Grammar<'_, u16, u16, u8>,
    listed: &[(String, usize, String)],
) -> Vec<(String, usize, String)> {
    let order = crate::relations::order(&grammar.relations, grammar.cells);
    let mut out = Vec::with_capacity(listed.len());
    for word in listed.chunk_by(|a, b| a.0 == b.0) {
        let Some((key, _, _)) = word.first() else {
            continue;
        };
        let forms: BTreeMap<usize, &String> = word.iter().map(|(_, c, f)| (*c, f)).collect();
        // Per cell, the letters the word has there: listed, or told.
        let mut have: BTreeMap<usize, String> = BTreeMap::new();
        for &c in &order {
            let told = grammar.relation(c).filter(|r| r.listed).and_then(|r| {
                let source = have.get(&r.source()?)?;
                r.told(source, grammar.smoothing).map(|t| t.0)
            });
            match forms.get(&c) {
                Some(form) => {
                    let letters = grammar.orthography.read(form).letters;
                    if told.as_ref() != Some(&letters) {
                        out.push((key.clone(), c, (*form).clone()));
                    }
                    have.insert(c, letters);
                }
                None => {
                    if let Some(t) = told {
                        have.insert(c, t);
                    }
                }
            }
        }
    }
    out.sort();
    out
}

/// The fewest words a paradigm or a stress pattern is learned from.
const MIN_WORDS: u32 = 3;

/// A paradigm's name: its commonest word that no sense makes vulgar or
/// offensive, judged by how many dictionaries have it and how many senses
/// its entry has (the dumps give no frequencies; then the shortest, then
/// the first alphabetically), of its plain words (letters only) of at least
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
                (!a.vulgar, a.dictionaries, a.senses)
                    .cmp(&(!b.vulgar, b.dictionaries, b.senses))
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
    laws: Option<&sound_law::Program<'_>>,
    shape: &Shape,
    settings: Settings,
) -> (Accent<'s, u16>, HashMap<String, u16>) {
    // Per word: per cell, its accent, letters and where the ending starts;
    // and where its lemma accents its stem.
    type Fact = (flexion::Stress, String, usize);
    let mut facts: Vec<(u16, Vec<Option<Fact>>, Option<usize>)> = Vec::with_capacity(words.len());
    for (w, &p) in words.iter().zip(assignment) {
        let Some(par) = paradigms.get(usize::from(p)) else {
            facts.push((p, vec![None; shape.cells], None));
            continue;
        };
        let stems = par.stems(orthography, &w.letters).unwrap_or_default();
        let form = |c: usize| w.cells.get(c).and_then(|fs| fs.first());
        let cells: Vec<Option<Fact>> = (0..shape.cells)
            .map(|c| {
                let f = form(c)?;
                let (_, ending) = par.built(paradigms, laws, c, &stems).into_iter().next()?;
                Some((f.stress?, f.letters.clone(), ending))
            })
            .collect();
        // The lemma's accent; else where the cells accented before their
        // ending have it most often.
        let lemma = shape
            .lemma_cell
            .and_then(form)
            .filter(|f| f.stress.is_some());
        let stem = match lemma {
            Some(f) => f
                .stress
                .map(|s| flexion::stress_position(orthography, &f.letters, s)),
            None => {
                let mut votes: BTreeMap<usize, u32> = BTreeMap::new();
                for (s, letters, ending) in cells.iter().flatten() {
                    if vowel_at(orthography, letters, s.vowel) < *ending {
                        *votes
                            .entry(flexion::stress_position(orthography, letters, *s))
                            .or_default() += 1;
                    }
                }
                votes.iter().max_by_key(|e| *e.1).map(|e| *e.0)
            }
        };
        facts.push((p, cells, stem));
    }
    let explains = |fact: &Fact, finals: Finals, place: Place, stem: Option<usize>| {
        (place != Place::Stem || stem.is_some())
            && flexion::stress_place(
                orthography,
                &fact.1,
                fact.2,
                finals,
                place,
                stem.unwrap_or(0),
            ) == Some(fact.0)
    };
    let explained = |fact: &Fact, long: bool, stem: Option<usize>| {
        [false, true].iter().any(|&circumflex| {
            Place::ALL
                .iter()
                .any(|&place| explains(fact, Finals { circumflex, long }, place, stem))
        })
    };
    // Cells whose final -αι, -οι is long: where reading it long explains
    // more words' accents.
    let long_finals: Vec<u16> = (0..shape.cells)
        .filter(|&c| {
            let (mut short, mut long) = (0u32, 0u32);
            for (_, cells, stem) in &facts {
                if let Some(Some(fact)) = cells.get(c) {
                    short += u32::from(explained(fact, false, *stem));
                    long += u32::from(explained(fact, true, *stem));
                }
            }
            long > short
        })
        .filter_map(|c| u16::try_from(c).ok())
        .collect();
    let is_long = |c: usize| u16::try_from(c).is_ok_and(|c| long_finals.binary_search(&c).is_ok());
    // Per paradigm, the cells whose ending's last syllable, accented, most
    // often takes the circumflex.
    let mut votes: HashMap<(u16, usize), (u32, u32)> = HashMap::default();
    for (p, cells, stem) in &facts {
        for (c, fact) in cells.iter().enumerate() {
            let Some(fact) = fact else { continue };
            let long = is_long(c);
            let vote = votes.entry((*p, c)).or_default();
            let any = |circumflex: bool| {
                Place::ALL
                    .iter()
                    .any(|&place| explains(fact, Finals { circumflex, long }, place, *stem))
            };
            let (acute, circ) = (any(false), any(true));
            // Only an accent the choice of kind decides counts.
            if acute != circ {
                vote.0 += u32::from(acute);
                vote.1 += u32::from(circ);
            }
        }
    }
    let mut circumflex: Vec<Vec<u16>> = vec![Vec::new(); paradigms.len()];
    for (&(p, c), &(acute, circ)) in &votes {
        if circ > acute
            && let (Some(cells), Ok(c)) = (circumflex.get_mut(usize::from(p)), u16::try_from(c))
        {
            cells.push(c);
        }
    }
    for cells in &mut circumflex {
        cells.sort_unstable();
    }
    let finals = |p: u16, c: usize| Finals {
        circumflex: circumflex
            .get(usize::from(p))
            .is_some_and(|cells| u16::try_from(c).is_ok_and(|c| cells.binary_search(&c).is_ok())),
        long: is_long(c),
    };
    // Per word: per cell, the places that give its accent (a bit per place
    // of `Place::ALL`).
    struct Seen {
        places: Vec<Option<u8>>,
        stem: Option<usize>,
    }
    let bit = |p: Place| 1u8 << Place::ALL.iter().position(|&x| x == p).unwrap_or(0);
    let seen: Vec<Seen> = facts
        .iter()
        .map(|(p, cells, stem)| Seen {
            places: cells
                .iter()
                .enumerate()
                .map(|(c, fact)| {
                    let fact = fact.as_ref()?;
                    Some(
                        Place::ALL
                            .iter()
                            .filter(|&&place| explains(fact, finals(*p, c), place, *stem))
                            .fold(0u8, |m, &place| m | bit(place)),
                    )
                })
                .collect(),
            stem: *stem,
        })
        .collect();
    // Per cell, how many words each place fits: of the places that fit a
    // word, it is read as the one most words share.
    let mut usual_place: Vec<[u32; 3]> = vec![[0; 3]; shape.cells];
    for s in &seen {
        for (c, p) in s.places.iter().enumerate() {
            if let (Some(m), Some(counts)) = (p, usual_place.get_mut(c)) {
                for (i, n) in counts.iter_mut().enumerate() {
                    if m & (1 << i) != 0 {
                        *n += 1;
                    }
                }
            }
        }
    }
    let read = |c: usize, mask: Option<u8>| -> Place {
        let counts = usual_place.get(c).copied().unwrap_or([0; 3]);
        Place::ALL
            .iter()
            .enumerate()
            .filter(|(i, _)| mask.is_none_or(|m| m & (1 << i) != 0))
            .max_by_key(|(i, _)| (counts.get(*i).copied().unwrap_or(0), std::cmp::Reverse(*i)))
            .map_or(Place::Stem, |(_, &p)| p)
    };
    let fits = |pat: &[Place], places: &[Option<u8>]| {
        pat.iter()
            .zip(places)
            .all(|(&pl, p)| p.is_none_or(|m| m & bit(pl) != 0))
    };
    let agreement = |pat: &[Place], places: &[Option<u8>]| {
        pat.iter()
            .zip(places)
            .filter(|(pl, p)| p.is_some_and(|m| m & bit(**pl) != 0))
            .count()
    };
    // Patterns: each word's places, its cells without a form read as most
    // words have them; kept if MIN_WORDS words have it.
    let mut vectors: BTreeMap<Vec<Place>, u32> = BTreeMap::new();
    for s in seen.iter().filter(|s| s.places.iter().any(Option::is_some)) {
        let pat: Vec<Place> = s
            .places
            .iter()
            .enumerate()
            .map(|(c, &m)| read(c, m))
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
    let chosen: Vec<usize> = of_word
        .iter()
        .map(|k| {
            k.and_then(|k| renumber.get(&k))
                .map_or(usize::MAX, |&k| usize::from(k))
        })
        .collect();
    let accent_patterns: Vec<AccentPattern<'s>> = order
        .iter()
        .filter_map(|&k| patterns.get(k))
        .enumerate()
        .map(|(i, (pat, n))| AccentPattern {
            name: Cow::Owned(name(words, &chosen, i)),
            words: *n,
            places: Cow::Owned(AccentPattern::table(pat)),
        })
        .collect();
    // Classes: a pattern and the stem's stressed vowel, from the lemma's end.
    let mut class_of: Vec<Option<StressClass<'s>>> = Vec::with_capacity(words.len());
    for ((w, k), s) in words.iter().zip(&of_word).zip(&seen) {
        let class = k.and_then(|k| renumber.get(&k)).map(|&pattern| {
            let n = flexion::stress_positions(orthography, &w.letters);
            let vowel = s
                .stem
                .and_then(|k| n.checked_sub(1)?.checked_sub(k))
                .and_then(|v| u8::try_from(v).ok());
            StressClass {
                name: Cow::Borrowed(""),
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
    let mut classes: Vec<StressClass<'s>> = counts
        .into_iter()
        .map(|((pattern, vowel), words)| StressClass {
            name: Cow::Borrowed(""),
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
    let chosen: Vec<usize> = class_of
        .iter()
        .map(|c| {
            c.as_ref()
                .and_then(|c| id.get(&(c.pattern, c.vowel)))
                .map_or(usize::MAX, |&i| usize::from(i))
        })
        .collect();
    for (i, class) in classes.iter_mut().enumerate() {
        class.name = Cow::Owned(name(words, &chosen, i));
    }
    let mut labels: HashMap<String, u16> = HashMap::default();
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
    // A class whose stem syllable the word hasn't got can't be its.
    let class = choose::learn(&examples, settings, |word, c| {
        classes.get(usize::from(c)).is_some_and(|c| {
            c.vowel
                .is_none_or(|v| usize::from(v) < flexion::stress_positions(orthography, word))
        })
    });
    let accent = Accent {
        patterns: Cow::Owned(accent_patterns),
        classes: Cow::Owned(classes),
        class,
        circumflex: Cow::Owned(circumflex.into_iter().map(Cow::Owned).collect()),
        long_finals: Cow::Owned(long_finals),
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
/// the dictionary writes it and typed plain: `(key, cell, form)`, sorted;
/// of the words `only` accepts. Of homographs, the first word's forms.
fn irregular(
    grammar: &Grammar<'_, u16, u16, u8>,
    lexemes: &[Lexeme],
    words: &[Word],
    only: impl Fn(&Word) -> bool,
) -> Vec<(String, usize, String)> {
    let mut first: BTreeMap<&str, usize> = BTreeMap::new();
    for (i, w) in words.iter().enumerate() {
        first.entry(w.key.as_str()).or_insert(i);
    }
    first.retain(|_, &mut i| words.get(i).is_some_and(&only));
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
