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
    /// Per cell, its place on each dimension the part of speech's cells vary
    /// on: its group (`present`, `future`), then each feature's value
    /// (`u16::MAX` where its group hasn't the feature).
    pub coordinates: Vec<Vec<u16>>,
    /// The most words to learn the structure from (see
    /// `PartOfSpeech::max_words`).
    pub max_words: Option<usize>,
    /// Per cell, its name (`gen_pl`), as the crate's constants are named.
    pub names: Vec<String>,
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
    /// How often the paradigm rules' productivity verdicts agree across the
    /// vocabularies they are judged on.
    pub stability: f64,
    /// The views paradigms may read their parents through (see [`View`]).
    pub views: Vec<View>,
}

/// Counts for the report.
#[derive(Debug, Clone, Default)]
pub struct Stats {
    pub words: usize,
    pub paradigms: usize,
    pub accent_patterns: usize,
    pub irregular_forms: usize,
    pub listed_stems: usize,
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
    /// The stem pieces.
    stems: Vec<String>,
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
    let unique: Vec<&Lexeme> = by_order
        .into_iter()
        .filter(|l| seen.insert(key(l)))
        .collect();
    // The grammar's structure (its paradigms, relations and laws, learned
    // form by form) is learned from at most so many words, the commonest;
    // every word then takes a paradigm and lists what the grammar gets
    // wrong, a word at a time. They come first, the rest after.
    let sample: crate::HashSet<usize> = match shape.max_words {
        Some(max) if unique.len() > max => {
            let mut by: Vec<usize> = (0..unique.len()).collect();
            by.sort_by_key(|&i| {
                unique
                    .get(i)
                    .map(|l| (std::cmp::Reverse(l.senses), l.source, l.order))
            });
            by.into_iter().take(max).collect()
        }
        _ => (0..unique.len()).collect(),
    };
    let (first, rest): (Vec<usize>, Vec<usize>) =
        (0..unique.len()).partition(|i| sample.contains(i));
    let learned_from = first.len();
    let every: Vec<Lexeme> = first
        .into_iter()
        .chain(rest)
        .filter_map(|i| unique.get(i).map(|&l| l.clone()))
        .collect();
    // A word is spelled as its table writes it (`ὑποκρῠ́πτω`, where the
    // headword says `ὑποκρύπτω`): its forms carry those marks.
    let letters = |l: &Lexeme| {
        let headword = orthography.read(&l.full).letters;
        shape
            .lemma_cell
            .and_then(|c| l.cells.get(c))
            .and_then(|fs| fs.first())
            .map(|f| orthography.read(f).letters)
            .filter(|t| orthography.key(t) == orthography.key(&headword))
            .unwrap_or(headword)
    };
    let everyone: Vec<Word> = every
        .iter()
        .map(|l| Word {
            key: key(l),
            letters: letters(l),
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
    let (words, rest_words) = everyone.split_at(learned_from.min(everyone.len()));
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
    let all_words = words;
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
    let mut analyses: Vec<Analyzed> = words.iter().map(analyze).collect();
    reduplicate(&mut analyses, shape.cells);
    let mut feature_examples: Vec<(String, u8)> = Vec::new();
    if let Some((f, referrals)) = &found {
        for (w, a) in words.iter().zip(analyses.iter_mut()) {
            let value = w.inherent.get(*f).copied().filter(|&v| v != u8::MAX);
            let value = value.or_else(|| infer(w, referrals));
            // Learned from the words whose forms it decides: those whose
            // cells it chooses between differ.
            let decides = referrals.iter().any(|r| {
                let forms: Option<Vec<&str>> = r
                    .from
                    .iter()
                    .map(|&c| first_form(w, usize::from(c)))
                    .collect();
                forms.is_some_and(|f| f.windows(2).any(|p| p.first() != p.get(1)))
            });
            if let Some(v) = value {
                if decides {
                    feature_examples.push((w.key.clone(), v));
                }
                refer(w, a, referrals, v);
            }
        }
    }
    let feature_choice = found
        .as_ref()
        .map(|_| choose::learn(&feature_examples, settings, |_, _| true));
    let usual: Vec<Vec<Option<&str>>> = words
        .iter()
        .map(|w| (0..shape.cells).map(|c| first_form(w, c)).collect())
        .collect();
    let (paradigms, assignment) = paradigms(&words, &analyses, shape);
    // Paradigms that differ only by a change where stem meets ending are
    // one paradigm and a sound law.
    let law_words: Vec<crate::laws::Word<'_>> = words
        .iter()
        .zip(&usual)
        .zip(&analyses)
        .map(|((w, forms), a)| crate::laws::Word {
            letters: &w.letters,
            forms: forms
                .iter()
                .zip(&a.cells)
                .map(|(f, p)| f.filter(|_| p.as_deref() != Some(DERIVED)))
                .collect(),
        })
        .collect();
    let (laws, program, paradigms, assignment) =
        crate::laws::learn(&orthography, &law_words, paradigms, assignment);
    let (mut paradigms, assignment) = rehome(
        &orthography,
        &law_words,
        paradigms,
        assignment,
        program.as_ref(),
    );
    inherent(&words, &assignment, &mut paradigms);
    // Each cell's relation to the cell its form is best told from, said
    // through the laws; the referrals (by the word's feature) take their
    // cells' place.
    let classes: Vec<(char, Vec<char>)> = laws
        .iter()
        .filter_map(|l| match &l.change {
            crate::laws::Change::Harmony { map, .. } => Some(map),
            _ => None,
        })
        .flatten()
        .fold(Vec::new(), |mut classes, &(class, _, value)| {
            match classes.iter_mut().find(|(c, _)| *c == class) {
                Some((_, values)) if !values.contains(&value) => values.push(value),
                Some(_) => {}
                None => classes.push((class, vec![value])),
            }
            classes
        });
    let root = shape.lemma_cell.unwrap_or(0);
    let through = program.as_ref().map(|p| (p, classes.as_slice()));
    let mut relations = crate::relations::learn(&usual, shape.cells, root, settings, through);
    if let Some((_, referrals)) = &found {
        relations.retain(|r| !referrals.iter().any(|q| q.cell == r.cell));
        relations.extend(referrals.iter().cloned());
    }
    // The cells no relation tells (too few words have them): by analogy.
    let told = crate::relations::analogies(
        &usual,
        &shape.coordinates,
        &relations,
        root,
        settings,
        through,
    );
    relations.extend(told);
    relations.sort_by_key(|r| r.cell);
    // The words the structure wasn't learned from: each takes the paradigm
    // that builds the most of its forms (none, written in parts that each
    // inflect).
    let rest_assignment = fit(&orthography, &paradigms, program.as_ref(), rest_words);
    let assigned: Vec<(&Word, u16)> = words
        .iter()
        .zip(assignment.iter().copied())
        .chain(
            rest_words
                .iter()
                .zip(&rest_assignment)
                .filter(|(w, _)| compound(w).is_none_or(|c| c.1 != Compound::Each))
                .filter_map(|(w, p)| Some((w, (*p)?))),
        )
        .collect();
    let mut alphabet: crate::HashSet<char> = crate::HashSet::default();
    for w in &words {
        for f in w.cells.iter().flatten() {
            alphabet.extend(f.letters.chars());
        }
    }
    let prices = crate::bits::Prices::new(alphabet.len(), shape.cells);
    derive(
        &mut paradigms,
        &relations,
        &assigned,
        &prices,
        settings.smoothing,
        program.as_ref(),
    );
    let mut seen = vec![vec![false; shape.cells]; paradigms.len()];
    for (w, &p) in words.iter().zip(&assignment) {
        for (c, fs) in w.cells.iter().enumerate() {
            if let Some(slot) = seen.get_mut(usize::from(p)).and_then(|s| s.get_mut(c)) {
                *slot |= !fs.is_empty();
            }
        }
    }
    // Read from the word's end to its start (`#lux`, not `pollux`).
    let anchored = |key: &str| format!("{}{key}", flexion::WORD_END);
    let paradigm_examples: Vec<(String, u16)> = assigned
        .iter()
        .map(|&(w, p)| (anchored(&w.key), p))
        .collect();
    let fits = |word: &str, p: u16| {
        let word = word.strip_prefix(flexion::WORD_END).unwrap_or(word);
        paradigms
            .get(usize::from(p))
            .is_some_and(|x| x.stems(&orthography, word).is_some())
    };
    let mut paradigm_choice = choose::learn(&paradigm_examples, settings, fits);
    // Which rules are productive, judged on the commonest words: in the
    // most dictionaries, with the most senses.
    let mut by_commonness: Vec<usize> = (0..assigned.len()).collect();
    by_commonness.sort_by_key(|&i| {
        assigned.get(i).map(|&(w, _)| {
            (
                std::cmp::Reverse(w.dictionaries),
                std::cmp::Reverse(w.senses),
                w.key.clone(),
            )
        })
    });
    let mut rank = vec![0; assigned.len()];
    for (r, &i) in by_commonness.iter().enumerate() {
        if let Some(slot) = rank.get_mut(i) {
            *slot = r;
        }
    }
    let stability = choose::judge(&mut paradigm_choice, &paradigm_examples, &rank);
    let (accent, stress_labels) = (!orthography.stress_marks.is_empty())
        .then(|| {
            let laws = program.as_ref();
            accent(
                &orthography,
                &words,
                &assignment,
                &paradigms,
                laws,
                shape,
                settings,
            )
        })
        .unzip();
    let stress_labels = stress_labels.unwrap_or_default();
    // How a letter typed without its mark is written, by what follows it.
    let marks = marks(&orthography, &words, settings);
    // Spellings a key doesn't show, listed: those the marks don't give (of
    // every word, those inflecting behind a prefix too, and those whose
    // headword leaves off marks its forms have); without marks, those the
    // word's paradigm doesn't write.
    let listed: Vec<&Word> = match &marks {
        Some(marks) => everyone
            .iter()
            .zip(&every)
            .filter(|(w, l)| {
                flexion::restore(&orthography, marks, settings.smoothing, &w.key, &w.key).0
                    != w.letters
                    || orthography.read(&l.full).letters != w.letters
            })
            .map(|(w, _)| w)
            .collect(),
        None => assigned
            .iter()
            .filter(|&&(w, p)| {
                let written = paradigms.get(usize::from(p)).and_then(|par| {
                    let stems = par.stems(&orthography, &w.key)?;
                    Some(pattern::fill(&par.lemma, &stems))
                });
                w.letters != w.key && written.as_deref() != Some(w.letters.as_str())
            })
            .map(|&(w, _)| w)
            .collect(),
    };
    let mut spellings: Vec<(Cow<'s, str>, Cow<'s, str>)> = listed
        .into_iter()
        .map(|w| (Cow::Owned(w.key.clone()), Cow::Owned(w.letters.clone())))
        .collect();
    spellings.sort();
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
        slots: Cow::Borrowed(&[]),
        tables: Cow::Borrowed(&[]),
        irregular: flexion::Irregular::default(),
        smoothing: settings.smoothing,
        sharpness: 1.0,
        trust: flexion::Trust::FULL,
    };
    // A word the grammar gets wrong is listed in its choices: its paradigm,
    // its stress pattern and stem vowel. Forms still wrong are irregular.
    let wrong: crate::HashSet<String> = irregular(&grammar, &every, &everyone, |_| true)
        .into_iter()
        .map(|(k, _, _)| k)
        .collect();
    for &(w, p) in &assigned {
        if !wrong.contains(&w.key) {
            continue;
        }
        let anchored = Cow::Owned(anchored(&w.key));
        grammar.paradigm.exceptions.to_mut().push((anchored, p));
        if let (Some(a), Some(&class)) = (grammar.accent.as_mut(), stress_labels.get(&w.key)) {
            a.class
                .exceptions
                .to_mut()
                .push((Cow::Owned(w.key.clone()), class));
        }
    }
    tidy(&mut grammar.paradigm);
    if let Some(a) = grammar.accent.as_mut() {
        tidy(&mut a.class);
    }
    // Words made of others (`hineinlassen`, `диван-кровать`) are checked
    // once the words they are made of have their irregular forms.
    let made: crate::HashSet<&str> = everyone
        .iter()
        .filter(|w| {
            w.key.contains('-')
                || grammar
                    .prefixes
                    .as_ref()
                    .and_then(|p| p.split(&w.key))
                    .is_some_and(|(_, separable, inseparable)| separable + inseparable > 0.0)
        })
        .map(|w| w.key.as_str())
        .collect();
    let made = |w: &Word| made.contains(w.key.as_str());
    // A word's listed forms are its principal parts: the forms no relation
    // tells from another listed one. The forms told wrong are listed too.
    let checked = misbuilt(&grammar, &every, &everyone, |w| !made(w));
    let wrong: Vec<(String, usize, String)> = checked
        .iter()
        .map(|(k, c, f, _)| (k.clone(), *c, f.clone()))
        .collect();
    // The stem slots: learned from the words' own analyses, and the forms
    // the grammar builds with the wrong letters.
    let keys: Vec<&str> = words.iter().map(|w| w.key.as_str()).collect();
    let root_of = crate::stems::roots(&keys);
    let mut letters_wrong: HashMap<&str, Vec<bool>> = HashMap::default();
    for (k, c, _, letters) in &checked {
        if *letters {
            let flags = letters_wrong
                .entry(k.as_str())
                .or_insert_with(|| vec![false; shape.cells]);
            if let Some(f) = flags.get_mut(*c) {
                *f = true;
            }
        }
    }
    let evidence: Vec<crate::stems::Evidence<'_>> = words
        .iter()
        .zip(&analyses)
        .zip(&root_of)
        .map(|((w, a), &root)| crate::stems::Evidence {
            root,
            cells: (0..shape.cells)
                .map(|c| {
                    let f = first_form(w, c)?;
                    let p = a.cells.get(c)?.as_deref()?;
                    (p != DERIVED).then(|| (f, self::usual(p)))
                })
                .collect(),
            wrong: letters_wrong
                .get(w.key.as_str())
                .cloned()
                .unwrap_or_default(),
        })
        .collect();
    let slot_of = crate::stems::slots(&evidence, shape.cells, grammar.laws.as_ref(), &prices);
    let built = crate::stems::tables(&grammar.paradigms, &slot_of, &shape.names);
    grammar.slots = Cow::Owned(built.slots);
    grammar.tables = Cow::Owned(built.tables);
    // A word the grammar builds with the wrong letters lists the stems that
    // build more of its forms right than they cost.
    // Every word that lists something is checked again once all is listed.
    let mut unsure: crate::HashSet<String> = wrong.iter().map(|(k, _, _)| k.clone()).collect();
    let candidates = listed_stems(&grammar, &everyone, &letters_wrong, &prices);
    // Each word's listing two ways: its forms (those no relation tells from
    // another listed one, then those told wrong), and the stems worth
    // listing for it with the forms still wrong; with the forms listed for
    // the words made of it (`hineinlassen` of `lassen`: they take its
    // stems). Each word and those made of it keep the cheaper (see
    // `crate::bits`).
    // Which relations tell listed words' forms: judged on the forms wrong
    // with every candidate stem listed, the same for both listings.
    grammar.irregular = flexion::Irregular::new(&[], &candidates);
    let judged = rebuilt(&grammar, &everyone, &candidates, wrong.clone());
    if let Cow::Owned(relations) = &mut grammar.relations {
        for r in relations.iter_mut() {
            r.listed = tells_listed(
                r,
                &judged,
                &everyone,
                grammar.smoothing,
                grammar.laws.as_ref(),
            );
        }
    }
    let words_of = (every.as_slice(), everyone.as_slice());
    let forms_only = listing(&mut grammar, &[], &wrong, words_of, &made, &mut unsure);
    let on_stems = listing(
        &mut grammar,
        &candidates,
        &wrong,
        words_of,
        &made,
        &mut unsure,
    );
    let prefixes = grammar.prefixes.clone();
    let base_of = |key: &str| -> String {
        let rest = prefixes
            .as_ref()
            .and_then(|p| p.split(key))
            .and_then(|(letters, ..)| key.char_indices().nth(letters))
            .and_then(|(at, _)| key.get(at..));
        rest.unwrap_or(key).to_string()
    };
    let stems = cheaper(
        &grammar,
        &forms_only,
        &on_stems,
        candidates,
        &prices,
        &base_of,
    );
    let irregular = listing(&mut grammar, &stems, &wrong, words_of, &made, &mut unsure);
    // The ending tables listed stems are built with; paradigms stated
    // through stem slots where that pays; the tables either uses are the
    // grammar's.
    let slot_index = |name: &str| grammar.slots.iter().position(|s| s.name == name);
    let listed_tables: crate::HashSet<(u16, String)> = stems
        .iter()
        .filter_map(|(_, slot, _, table)| {
            Some((u16::try_from(slot_index(slot)?).ok()?, table.clone()))
        })
        .collect();
    let all_tables = grammar.tables.to_vec();
    let mut used: Vec<usize> = Vec::new();
    let views = views(shape);
    if let Cow::Owned(paradigms) = &mut grammar.paradigms {
        inherit(paradigms, &grammar.relations, &views);
        used = crate::stems::state(
            paradigms,
            &grammar.slots,
            &all_tables,
            &listed_tables,
            &prices,
        );
    }
    grammar.tables = Cow::Owned(
        all_tables
            .into_iter()
            .enumerate()
            .filter(|(i, t)| {
                used.contains(i) || listed_tables.contains(&(t.slot, t.name.to_string()))
            })
            .map(|(_, t)| t)
            .collect(),
    );
    let stats = Stats {
        words: everyone.len(),
        paradigms: grammar.paradigms.len(),
        accent_patterns: grammar.accent.as_ref().map_or(0, |a| a.patterns.len()),
        irregular_forms: irregular.len(),
        listed_stems: stems.len(),
    };
    Learned {
        grammar,
        feature: found.map(|f| f.0),
        members,
        irregular,
        seen,
        laws,
        stats,
        stability,
        views,
    }
}

/// How a word written in parts inflects, keyed by its first part: each
/// part when a form changes its first part. `None` for a word of one part.
fn compound(w: &Word) -> Option<(String, Compound)> {
    let (first, _) = w.letters.split_once('-')?;
    let (first_key, _) = w.key.split_once('-')?;
    let firsts = w
        .cells
        .iter()
        .filter_map(|fs| fs.first()?.letters.split_once('-'));
    let how = match firsts.map(|(f, _)| f).any(|f| f != first) {
        true => Compound::Each,
        false => Compound::Last,
    };
    Some((first_key.to_string(), how))
}

/// A view: a cell order in which each cell reads another of the same
/// part of speech (one feature's value read as another's, `act_as_pass`),
/// named after the value names it swaps.
pub type View = (String, Vec<u16>);

/// The views one value of one of the cells' features read as another make
/// (see [`View`]): each cell with the first value reads the cell like it
/// but for that value.
fn views(shape: &Shape) -> Vec<View> {
    let at = |c: usize| shape.coordinates.get(c).map_or(&[][..], Vec::as_slice);
    let dims = shape.coordinates.iter().map(Vec::len).max().unwrap_or(0);
    let mut out: Vec<View> = Vec::new();
    for k in 0..dims {
        let mut values: Vec<u16> = (0..shape.cells)
            .filter_map(|c| at(c).get(k).copied())
            .filter(|&v| v != u16::MAX)
            .collect();
        values.sort_unstable();
        values.dedup();
        for &a in &values {
            for &b in values.iter().filter(|&&b| b != a) {
                let map: Vec<u16> = (0..shape.cells)
                    .map(|c| {
                        let mut target = at(c).to_vec();
                        if target.get(k) != Some(&a) {
                            return c;
                        }
                        if let Some(slot) = target.get_mut(k) {
                            *slot = b;
                        }
                        (0..shape.cells)
                            .find(|&d| at(d) == target.as_slice())
                            .unwrap_or(c)
                    })
                    .filter_map(|c| u16::try_from(c).ok())
                    .collect();
                // Named by the name parts the first cell it moves changes.
                let Some((from, to)) = map
                    .iter()
                    .enumerate()
                    .find(|&(c, &d)| usize::from(d) != c)
                    .and_then(|(c, &d)| {
                        Some((shape.names.get(c)?, shape.names.get(usize::from(d))?))
                    })
                else {
                    continue;
                };
                let parts = |n: &str| n.split('_').map(str::to_string).collect::<Vec<_>>();
                let (x, y) = (parts(from), parts(to));
                let name = x
                    .iter()
                    .zip(&y)
                    .find(|(p, q)| p != q)
                    .map_or_else(String::new, |(p, q)| format!("{p}_as_{q}"));
                if !name.is_empty() && !out.iter().any(|v| v.1 == map) {
                    out.push((name, map));
                }
            }
        }
    }
    out
}

/// Each paradigm made like the earlier paradigm (with as many stem pieces)
/// it shares the most cells with, read as it is or through a view (see
/// [`View`]), if that is at least half of its own: it then writes only the
/// cells that differ. Cells that are a copy of another (by a relation) are
/// derived.
fn inherit(paradigms: &mut [Paradigm<'_>], relations: &[Relation<'_>], views: &[View]) {
    let copied: Vec<(usize, usize, bool)> = relations
        .iter()
        .filter(|r| r.analogy.is_none() && r.changes.iter().all(|(old, new)| old == new))
        .filter_map(|r| Some((usize::from(r.cell), r.source()?, r.change.is_none())))
        .collect();
    let cells: Vec<Vec<String>> = paradigms
        .iter()
        .map(|p| {
            let written: Vec<&str> = p.table.split('\t').collect();
            // A copy with no changes copies every form; one read off a
            // change, the usual one only.
            let copy = |c: usize, m: &str| {
                let from = |&(cell, from, all): &(usize, usize, bool)| {
                    cell == c && written.get(from) == Some(&m) && (all || !m.contains('|'))
                };
                m != DERIVED && copied.iter().any(from)
            };
            let cells = written.iter().enumerate();
            cells
                .map(|(c, &m)| if copy(c, m) { DERIVED } else { m }.to_string())
                .collect()
        })
        .collect();
    let pieces: Vec<usize> = paradigms
        .iter()
        .map(|p| pattern::pieces(&p.lemma))
        .collect();
    for (i, paradigm) in paradigms.iter_mut().enumerate() {
        let (Some(mine), Some(&my_pieces)) = (cells.get(i), pieces.get(i)) else {
            continue;
        };
        // The parent's cell `c` reads, through view `v`.
        let read = |v: Option<usize>, c: usize| {
            v.and_then(|v| views.get(v)?.1.get(c))
                .map_or(c, |&d| usize::from(d))
        };
        // A derived cell is told from its own source: read through a view,
        // it is no other cell's.
        let like = |theirs: &[String], v: Option<usize>, c: usize, m: &String| {
            theirs.get(read(v, c)) == Some(m) && (v.is_none() || m != DERIVED)
        };
        let shared = |j: usize, v: Option<usize>| {
            cells.get(j).map_or(0, |theirs| {
                mine.iter()
                    .enumerate()
                    .filter(|&(c, a)| !a.is_empty() && like(theirs, v, c, a))
                    .count()
            })
        };
        let filled = mine.iter().filter(|c| !c.is_empty()).count();
        let ways = std::iter::once(None).chain((0..views.len()).map(Some));
        let parent = (0..i)
            .filter(|&j| pieces.get(j) == Some(&my_pieces))
            .flat_map(|j| ways.clone().map(move |v| (j, v)))
            .map(|(j, v)| (shared(j, v), v.is_none(), std::cmp::Reverse(j), v))
            .max_by_key(|&(n, plain, j, _)| (n, plain, j))
            .filter(|&(n, ..)| n > 0 && 2 * n >= filled)
            .map(|(_, _, std::cmp::Reverse(j), v)| (j, v));
        let view = parent.and_then(|(_, v)| v);
        let theirs = parent.and_then(|(j, _)| cells.get(j));
        // A cell like the parent's isn't written; like no other, nor are
        // its derived cells.
        let table: String = mine
            .iter()
            .enumerate()
            .take(mine.len().saturating_sub(1))
            .map(|(c, m)| match theirs {
                Some(t) if like(t, view, c, m) => "\t".to_string(),
                None if m == DERIVED => "\t".to_string(),
                _ => format!("{m}\t"),
            })
            .collect();
        paradigm.parent = parent.and_then(|(j, _)| u16::try_from(j).ok());
        paradigm.reads = view
            .and_then(|v| views.get(v))
            .map(|v| Cow::Owned(v.1.clone()));
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
/// bases: `(word's key, base)`. Only where words separate prefixes, or
/// keep them in front otherwise than written before the rest's forms (the
/// augment after a Greek preverb); elsewhere a prefix kept in front is no
/// more than the start of a word.
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
    let mut separable: HashMap<&str, &str> = HashMap::default();
    for w in words {
        let prefix = w.cells.iter().filter_map(|fs| fs.first()).find_map(|f| {
            let (_, tail) = f.letters.rsplit_once(' ')?;
            (!tail.is_empty() && w.letters.starts_with(tail) && w.letters.len() > tail.len())
                .then_some(tail)
        });
        if let Some(p) = prefix {
            separable.insert(w.key.as_str(), p);
        }
    }
    let unseparated = || {
        words
            .iter()
            .filter(|w| !separable.contains_key(w.key.as_str()))
    };
    // Words made of a prefix and a word of the dictionary.
    let mut pairs: Vec<(&Word, &str, &str, &Word)> = Vec::new();
    for w in unseparated() {
        for (p, pk, rest) in PREFIX_LETTERS.filter_map(|n| split(w, n)) {
            if let Some(&base) = by_key.get(rest) {
                pairs.push((w, p, pk, base));
            }
        }
    }
    // What an inseparable prefix takes off each cell (the participle's
    // `ge`).
    let mut drops: Vec<BTreeMap<String, u32>> = vec![BTreeMap::new(); cells];
    for (w, p, _, base) in &pairs {
        for (c, votes) in drops.iter_mut().enumerate() {
            let (Some(mine), Some(theirs)) = (form(w, c), form(base, c)) else {
                continue;
            };
            if let Some(dropped) = mine
                .strip_prefix(p)
                .and_then(|tail| theirs.strip_suffix(tail))
            {
                *votes.entry(dropped.to_string()).or_default() += 1;
            }
        }
    }
    let drops: Vec<String> = drops
        .iter()
        .map(|v| v.iter().max_by_key(|e| *e.1).map(|e| e.0.clone()))
        .map(Option::unwrap_or_default)
        .collect();
    let drop = |c: usize| drops.get(c).map_or("", String::as_str);
    // How a prefix meets the rest's form: by the form's first letter, what
    // the two become (Greek `ἀπο` and the augment's `ἐ`: `ἀπε`), as most
    // words have it; kept where that isn't just the prefix written before
    // the form, and MIN_WORDS words have it.
    let mut met: BTreeMap<(&str, char), BTreeMap<&str, crate::HashSet<&str>>> = BTreeMap::new();
    // Per prefix, the words that keep it in front of a form beginning
    // otherwise than their rest does (the augment of `ἀπ-έ-λυον`, `λύω`):
    // what a rule on a word's ending can't see.
    let mut augmented: BTreeMap<&str, crate::HashSet<&str>> = BTreeMap::new();
    for &(w, p, pk, base) in &pairs {
        for c in 0..cells {
            let (Some(mine), Some(theirs)) = (form(w, c), form(base, c)) else {
                continue;
            };
            let theirs = theirs.strip_prefix(drop(c)).unwrap_or(theirs);
            let Some(head) = theirs.chars().next() else {
                continue;
            };
            let tail = theirs.get(head.len_utf8()..).unwrap_or("");
            let Some(joined) = mine.strip_suffix(tail) else {
                continue;
            };
            // Something put before the rest's form: it begins otherwise
            // than the rest, and goes on as the rest begins.
            let first = base.letters.chars().next();
            if first != Some(head) && tail.chars().next() == first {
                augmented.entry(pk).or_default().insert(&w.key);
            }
            if joined != format!("{p}{head}") {
                let by = met.entry((pk, head)).or_default();
                by.entry(joined).or_default().insert(&w.key);
            }
        }
    }
    let augmenting = |pk: &str| {
        augmented
            .get(pk)
            .is_some_and(|w| w.len() >= MIN_WORDS as usize)
    };
    let junctions: Vec<(Cow<'a, str>, Cow<'a, str>, Cow<'a, str>)> = met
        .into_iter()
        .filter(|((pk, _), _)| augmenting(pk))
        .filter_map(|((pk, head), joined)| {
            let (joined, by) = joined.into_iter().max_by_key(|e| e.1.len())?;
            (by.len() >= MIN_WORDS as usize).then(|| {
                let own = |s: &str| Cow::Owned(s.to_string());
                (own(pk), own(&head.to_string()), own(joined))
            })
        })
        .collect();
    let joining = flexion::Prefixes {
        prefixes: Cow::Borrowed(&[]),
        listed: Cow::Borrowed(&[]),
        placement: Cow::Borrowed(&[]),
        drops: Cow::Borrowed(&[]),
        junctions: Cow::Borrowed(&junctions),
    };
    // A form of a word keeping prefix `p` (key `pk`) in front, without it:
    // the rest's form, with what the cell takes off put back.
    let unjoin = |pk: &str, p: &str, c: usize, mine: &str| -> Option<String> {
        let rest = joining.unjoin(pk, p, mine)?;
        (drop(c).is_empty() || !rest.starts_with(drop(c))).then(|| format!("{}{rest}", drop(c)))
    };
    // Inseparable: its forms (all, where the words separate prefixes; four
    // in five, where they join them), without the prefix, are the rest's
    // forms where the dictionary has the rest.
    let keeps = |w: &Word, p: &str, pk: &str, rest: &str| {
        let base = by_key.get(rest);
        let mine: Vec<(usize, &str)> = (0..cells).filter_map(|c| Some((c, form(w, c)?))).collect();
        let kept = mine.iter().filter(|&&(c, mine)| {
            let Some(unjoined) = unjoin(pk, p, c, mine) else {
                return false;
            };
            base.and_then(|b| form(b, c)).is_none_or(|theirs| {
                let t = theirs.strip_prefix(drop(c)).unwrap_or(theirs);
                joining.join(pk, p, t).0 == mine || theirs == unjoined
            })
        });
        let (kept, seen) = (kept.count(), mine.len());
        seen > 0
            && if augmenting(pk) {
                5 * kept >= 4 * seen
            } else {
                kept == seen
            }
    };
    let mut proven: BTreeMap<&str, u32> = BTreeMap::new();
    for w in unseparated() {
        let first = PREFIX_LETTERS
            .map_while(|n| split(w, n))
            .find(|&(p, pk, rest)| by_key.contains_key(rest) && keeps(w, p, pk, rest));
        if let Some((_, pk, _)) = first {
            *proven.entry(pk).or_default() += 1;
        }
    }
    // A prefix needs MIN_WORDS words that separate it or that keep it, with a
    // base the dictionary has.
    let mut separated: BTreeMap<&str, u32> = BTreeMap::new();
    for p in separable.values() {
        *separated.entry(p).or_default() += 1;
    }
    let separates = separated.values().any(|&n| n >= MIN_WORDS);
    let counted: std::collections::BTreeSet<&str> = separated
        .iter()
        .chain(&proven)
        .filter(|&(p, &n)| n >= MIN_WORDS && (separates || augmenting(p)))
        .map(|(p, _)| *p)
        .collect();
    if counted.is_empty() {
        return (None, Vec::new());
    }
    // Every word that begins with a prefix: what it does with it.
    let mut bases: Vec<(String, Word)> = Vec::new();
    let mut what: Vec<(&str, &str, Separation)> = Vec::new();
    for w in words {
        let longest = counted
            .iter()
            .filter(|p| w.key.starts_with(*p) && w.key.chars().count() >= p.chars().count() + 3)
            .max_by_key(|p| p.len());
        let Some(&pk) = longest else { continue };
        let n = pk.chars().count();
        let prefix: String = w.letters.chars().take(n).collect();
        let rest_key: String = w.key.chars().skip(n).collect();
        let separation = if separable.get(w.key.as_str()) == Some(&prefix.as_str()) {
            Separation::Separable
        } else if keeps(w, &prefix, pk, &rest_key) {
            Separation::Inseparable
        } else {
            Separation::Neither
        };
        what.push((pk, &w.key, separation));
        if separation == Separation::Neither {
            continue;
        }
        let mut base = w.clone();
        base.letters = w.letters.chars().skip(n).collect();
        base.key = rest_key;
        // Its forms without the prefix; a cell with one that isn't, none.
        for (c, forms) in base.cells.iter_mut().enumerate() {
            let rests: Option<Vec<String>> = forms
                .iter()
                .map(|f| match separation {
                    Separation::Separable => f
                        .letters
                        .strip_suffix(prefix.as_str())
                        .and_then(|s| s.strip_suffix(' '))
                        .or_else(|| f.letters.strip_prefix(prefix.as_str()))
                        .map(str::to_string),
                    _ => unjoin(pk, &prefix, c, &f.letters),
                })
                .collect();
            match rests {
                Some(rests) => {
                    for (f, rest) in forms.iter_mut().zip(rests) {
                        (f.letters, f.stress) = (rest, None);
                    }
                }
                None => forms.clear(),
            }
        }
        bases.push((w.key.clone(), base));
    }
    // Where a separable prefix goes, cell by cell.
    let placement = (0..cells)
        .map(|c| {
            let (mut before, mut after) = (0u32, 0u32);
            for w in words {
                let (Some(p), Some(f)) = (separable.get(w.key.as_str()), form(w, c)) else {
                    continue;
                };
                if f.ends_with(&format!(" {p}")) {
                    after += 1;
                } else if f.starts_with(p) {
                    before += 1;
                }
            }
            if after > before {
                flexion::Placement::After
            } else {
                flexion::Placement::Before
            }
        })
        .collect();
    // Each prefix's counts; words that go against most are listed.
    const KINDS: [Separation; 3] = [
        Separation::Separable,
        Separation::Inseparable,
        Separation::Neither,
    ];
    let mut prefixes: Vec<flexion::Prefix<'a>> = Vec::new();
    let mut listed: Vec<(Cow<'a, str>, Separation)> = Vec::new();
    for p in counted {
        let mine = what.iter().filter(|e| e.0 == p);
        let count = |s: Separation| mine.clone().filter(|e| e.2 == s).count();
        let n = KINDS.map(|s| u32::try_from(count(s)).unwrap_or(u32::MAX));
        let most = KINDS.into_iter().zip(n).max_by_key(|e| e.1);
        let most = most.map_or(Separation::Neither, |e| e.0);
        listed.extend(
            mine.filter(|e| e.2 != most)
                .map(|e| (Cow::Owned(e.1.to_string()), e.2)),
        );
        let [separable, inseparable, neither] = n;
        prefixes.push(flexion::Prefix {
            letters: Cow::Owned(p.to_string()),
            separable,
            inseparable,
            words: separable + inseparable + neither,
        });
    }
    listed.sort();
    let prefixes = flexion::Prefixes {
        prefixes: Cow::Owned(prefixes),
        listed: Cow::Owned(listed),
        placement: Cow::Owned(placement),
        drops: Cow::Owned(drops.into_iter().map(Cow::Owned).collect()),
        junctions: Cow::Owned(junctions),
    };
    (Some(prefixes), bases)
}

/// Word `w` split after its first `n` letters: the prefix's letters and
/// key, and the rest's key. `None` unless three letters or more are left,
/// and its key has as many letters as it.
fn split(w: &Word, n: usize) -> Option<(&str, &str, &str)> {
    let at = w.letters.char_indices().nth(n)?.0;
    let key_at = w.key.char_indices().nth(n)?.0;
    let (p, rest) = w.letters.split_at(at);
    let (pk, rest_key) = w.key.split_at(key_at);
    (rest.chars().count() >= 3 && w.key.chars().count() == w.letters.chars().count())
        .then_some((p, pk, rest_key))
}

/// How a letter typed without its mark is written (see
/// [`flexion::Grammar::marks`]): learned from every letter of the words that
/// a mark could stand on, by the letter before it and the letters from it
/// on. `None` where no word writes such a mark.
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
    // What the rules get wrong is a word's own spelling, listed with it.
    Some(choose::learn_starts(&examples, settings))
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
        let mut found: Vec<Relation<'s>> = Vec::new();
        for cell in 0..shape.cells {
            let to: Option<Vec<u16>> = (0..values)
                .map(|v| {
                    let with: Vec<&Word> = words
                        .iter()
                        .filter(|w| {
                            w.inherent.get(f).is_some_and(|&x| usize::from(x) == v)
                                && form(w, cell).is_some()
                        })
                        .collect();
                    // An earlier cell, itself formed by the paradigm.
                    let (n, t) = (0..cell)
                        .filter(|&t| !found.iter().any(|r| usize::from(r.cell) == t))
                        .map(|t| {
                            let same = with
                                .iter()
                                .filter(|w| form(w, t).is_some() && form(w, t) == form(w, cell));
                            (same.count(), t)
                        })
                        .max_by_key(|&(n, t)| (n, std::cmp::Reverse(t)))?;
                    #[allow(clippy::cast_precision_loss)]
                    let share = n as f64 / with.len().max(1) as f64;
                    (with.len() >= 50 && share >= REFERRAL_SHARE)
                        .then(|| u16::try_from(t).unwrap_or(u16::MAX))
                })
                .collect();
            if let Some(to) = to.filter(|to| to.iter().any(|t| Some(t) != to.first())) {
                let cell = u16::try_from(cell).unwrap_or(u16::MAX);
                found.push(crate::relations::copy(cell, to, None));
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
    let form = |c: u16| first_form(w, usize::from(c));
    let values = referrals.first()?.from.len();
    let mut fitting = (0..values).filter(|&v| {
        referrals.iter().all(|r| {
            r.from
                .get(v)
                .is_some_and(|&t| form(r.cell).is_none() || form(r.cell) == form(t))
        })
    });
    match (fitting.next(), fitting.next()) {
        (Some(v), None) => u8::try_from(v).ok(),
        _ => None,
    }
}

/// Mark the cells of `a` the word, of value `v`, forms by referral.
fn refer(w: &Word, a: &mut Analyzed, referrals: &[Relation<'_>], v: u8) {
    let form = |c: u16| first_form(w, usize::from(c));
    for r in referrals {
        if let Some(&t) = r.from.get(usize::from(v))
            && form(r.cell).is_some()
            && form(r.cell) == form(t)
            && let Some(slot) = a.cells.get_mut(usize::from(r.cell))
        {
            *slot = Some(DERIVED.to_string());
        }
    }
}

/// A word's patterns over its own skeleton.
fn analyze(w: &Word) -> Analyzed {
    let forms: Vec<&str> = w
        .cells
        .iter()
        .flatten()
        .map(|f| f.letters.as_str())
        .collect();
    let a = skeleton::analyze(&w.letters, &forms);
    let mut patterns = a.forms.into_iter();
    let cells = w
        .cells
        .iter()
        .map(|fs| {
            let mine: Vec<String> = patterns.by_ref().take(fs.len()).flatten().collect();
            (!mine.is_empty()).then(|| mine.join("|"))
        })
        .collect();
    Analyzed {
        lemma: a.lemma,
        cells,
        stems: a.stems,
    }
}

/// Per stem piece of `pattern` (chars) after a letter that may copy it, a
/// letter between them (`λ` of `λε~`, `λυ`): the piece's first letter, the
/// copying letter's place in the pattern, and the letter between.
fn before(pattern: &[char], stems: &[String]) -> Vec<(char, usize, char)> {
    let letter = |at: usize| {
        let c = *pattern.get(at)?;
        (c != '~' && c != pattern::COPY).then_some(c)
    };
    let pieces = pattern.iter().enumerate().filter(|&(_, &c)| c == '~');
    pieces
        .zip(stems)
        .filter_map(|((i, _), stem)| {
            let first = stem.chars().next()?;
            let at = i.checked_sub(2)?;
            letter(at)?;
            Some((first, at, letter(i.checked_sub(1)?)?))
        })
        .collect()
}

/// Reduplication in a cell: the first letters its words copy, and the
/// letters between a copy and its stem piece (`ε`), with how many words
/// have each.
#[derive(Default)]
struct Copying {
    letters: crate::HashSet<char>,
    between: HashMap<char, u32>,
}

impl Copying {
    /// Whether enough of its words have `between` after their copy.
    fn usual(&self, between: char) -> bool {
        self.between
            .get(&between)
            .is_some_and(|&n| n as usize >= crate::laws::MIN_WORDS)
    }
}

/// Reduplication in the cells that have it (words beginning with at least
/// three different letters copy them, a letter between): the letter before
/// a stem piece that copies its first letter is written as a copy
/// ([`pattern::COPY`]); so is a letter another is copied as (Greek `φ` as
/// `π`: `πεφίληκα`), where [`crate::laws::MIN_WORDS`] words copy it so and
/// fewer copy it as itself. A sound law then changes the copy (see
/// [`pattern::COPIED`]).
fn reduplicate(analyses: &mut [Analyzed], cells: usize) {
    // Each stem piece after a letter that may copy it: its word and cell,
    // its first letter, that letter, and the letter between.
    let mut found: Vec<(usize, usize, char, char, char)> = Vec::new();
    for (w, a) in analyses.iter().enumerate() {
        for (c, p) in a.cells.iter().enumerate() {
            let Some(p) = p else { continue };
            let chars: Vec<char> = usual(p).chars().collect();
            for (first, at, between) in before(&chars, &a.stems) {
                if let Some(&y) = chars.get(at) {
                    found.push((w, c, first, y, between));
                }
            }
        }
    }
    let mut cell: Vec<Copying> = (0..cells).map(|_| Copying::default()).collect();
    // The words copying each letter as itself.
    let mut plain: HashMap<char, crate::HashSet<usize>> = HashMap::default();
    for &(w, c, first, _, between) in found.iter().filter(|e| e.2 == e.3) {
        if let Some(copying) = cell.get_mut(c) {
            copying.letters.insert(first);
            *copying.between.entry(between).or_default() += 1;
            plain.entry(first).or_default().insert(w);
        }
    }
    let copies = |c: &Copying| c.letters.len() >= 3;
    // The letters each letter is copied as instead, and the words doing so.
    let mut mapped: HashMap<(char, char), crate::HashSet<usize>> = HashMap::default();
    for &(w, c, first, y, between) in found.iter().filter(|e| e.2 != e.3) {
        if cell.get(c).is_some_and(|k| copies(k) && k.usual(between)) {
            mapped.entry((first, y)).or_default().insert(w);
        }
    }
    let mapped: crate::HashSet<(char, char)> = mapped
        .into_iter()
        .filter(|((x, _), words)| {
            words.len() >= crate::laws::MIN_WORDS
                && words.len() > plain.get(x).map_or(0, crate::HashSet::len)
        })
        .map(|(pair, _)| pair)
        .collect();
    for a in analyses.iter_mut() {
        for (p, copying) in a.cells.iter_mut().zip(&cell) {
            let Some(p) = p.as_mut() else { continue };
            if !copies(copying) || p.contains(DERIVED) {
                continue;
            }
            let rewritten: Vec<String> = pattern::variants(p)
                .map(|v| {
                    let mut chars: Vec<char> = v.chars().collect();
                    // The letter itself, or one copied as it, before a
                    // usual letter between.
                    for (first, at, between) in before(&chars, &a.stems) {
                        if let Some(y) = chars.get_mut(at)
                            && (*y == first
                                || (mapped.contains(&(first, *y)) && copying.usual(between)))
                        {
                            *y = pattern::COPY;
                        }
                    }
                    chars.into_iter().collect()
                })
                .collect();
            *p = rewritten.join("|");
        }
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
/// derived from another (see [`crate::relations`]).
fn paradigms<'s>(
    words: &[Word],
    analyses: &[Analyzed],
    shape: &Shape,
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
    // Signatures by lemma pattern (of those `keep` takes), and lemma
    // patterns by their last literal (what a word must end in to fit them).
    let index = |keep: &dyn Fn(usize) -> bool| {
        let mut by_lemma: HashMap<&str, Vec<usize>> = HashMap::default();
        for (i, (lemma, _)) in signatures.iter().enumerate().filter(|e| keep(e.0)) {
            by_lemma.entry(lemma.as_str()).or_default().push(i);
        }
        let mut by_last: HashMap<&str, Vec<&str>> = HashMap::default();
        for &lp in by_lemma.keys() {
            by_last
                .entry(lp.rsplit('~').next().unwrap_or(""))
                .or_default()
                .push(lp);
        }
        (by_lemma, by_last)
    };
    // Whether signature `cand` builds word `i`'s form of a cell: derived
    // where the word's is.
    let builds = |cand: usize, i: usize, cell: usize, stems: &[&str]| {
        let p = signatures.get(cand).and_then(|s| s.1.get(cell)?.as_deref());
        let form = words.get(i).and_then(|w| first_form(w, cell));
        let mine = analyses.get(i).and_then(|a| a.cells.get(cell)?.as_deref());
        match (p, form) {
            (Some(DERIVED), _) => mine == Some(DERIVED),
            (Some(p), Some(f)) => pattern::fill(p, stems) == f,
            _ => false,
        }
    };
    let (mut by_lemma, by_last) = index(&|_| true);
    for list in by_lemma.values_mut() {
        list.sort_by_key(|&s| std::cmp::Reverse(fullness(s)));
    }
    let mut chosen: Vec<usize> = Vec::with_capacity(words.len());
    for (i, (w, &s)) in words.iter().zip(&own).enumerate() {
        let mine = signatures.get(s).map_or(&[][..], |m| m.1.as_slice());
        let mut best = (fullness(s), [0u32; 3], size.get(s).copied().unwrap_or(0), s);
        for (lp, stems) in fitting(&by_last, &w.letters) {
            for &cand in by_lemma.get(lp).into_iter().flatten() {
                // Only fuller paradigms complete it.
                if fullness(cand) <= fullness(s) || fullness(cand) < best.0 {
                    break;
                }
                // It must build every form the word has.
                let mut cells = mine.iter().enumerate().filter(|(_, m)| m.is_some());
                let every = cells.all(|(c, _)| builds(cand, i, c, &stems));
                if !every {
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
    // A paradigm of more than one word is kept where it pays for itself (see
    // `crate::bits`): from the one with fewest words up, its words go to the
    // paradigm that builds the most of their forms when listing the forms it
    // doesn't build costs less than stating the paradigm.
    let mut alphabet: crate::HashSet<char> = crate::HashSet::default();
    for w in words {
        for f in w.cells.iter().flatten() {
            alphabet.extend(f.letters.chars());
        }
    }
    let prices = crate::bits::Prices::new(alphabet.len(), shape.cells);
    let mut members: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for (i, &c) in chosen.iter().enumerate() {
        members.entry(c).or_default().push(i);
    }
    let mut order: Vec<usize> = members.keys().copied().collect();
    order.sort_by_key(|s| (members.get(s).map_or(0, Vec::len), *s));
    let mut kept: crate::HashSet<usize> = order.iter().copied().collect();
    for s in order {
        kept.remove(&s);
        let (by_lemma, by_last) = index(&|c| kept.contains(&c));
        let mine = members.get(&s).cloned().unwrap_or_default();
        // Each word's best other paradigm, and the forms it lists there.
        let mut moves: Vec<(usize, usize)> = Vec::with_capacity(mine.len());
        let mut near = true;
        let mut listed = 0.0;
        let mut stated = crate::bits::ITEM;
        for &i in &mine {
            let Some(w) = words.get(i) else { continue };
            let mut best: Option<((usize, usize), usize, Vec<String>)> = None;
            for (lp, stems) in fitting(&by_last, &w.letters) {
                for &cand in by_lemma.get(lp).into_iter().flatten() {
                    let built = (0..shape.cells)
                        .filter(|&c| builds(cand, i, c, &stems))
                        .count();
                    let key = (built, members.get(&cand).map_or(0, Vec::len));
                    if best.as_ref().is_none_or(|b| key > b.0) {
                        let owned = stems.iter().map(|x| (*x).to_string()).collect();
                        best = Some((key, cand, owned));
                    }
                }
            }
            let stems: Vec<&str> = best
                .as_ref()
                .map_or(Vec::new(), |b| b.2.iter().map(String::as_str).collect());
            for c in 0..shape.cells {
                let Some(form) = first_form(w, c) else {
                    continue;
                };
                if best.as_ref().is_none_or(|b| !builds(b.1, i, c, &stems)) {
                    listed += prices.listed(form);
                    // What the paradigm states there instead, once.
                    if i == mine.first().copied().unwrap_or(i) {
                        let p = signatures.get(s).and_then(|x| x.1.get(c)?.as_deref());
                        stated += prices.cell + p.map_or(0.0, |p| prices.text(p));
                    }
                }
            }
            let forms = (0..shape.cells)
                .filter(|&c| first_form(w, c).is_some())
                .count();
            if let Some(b) = best {
                // Most of its forms built there, for a word moved anyway.
                near &= 2 * b.0.0 >= forms;
                moves.push((i, b.1));
            }
        }
        // One word's own table generalizes to no other: where another
        // paradigm builds most of its forms, its word is listed there.
        let alone = mine.len() < 2 && near;
        if moves.len() == mine.len() && (listed < stated || alone) {
            for (i, cand) in moves {
                if let Some(slot) = chosen.get_mut(i) {
                    *slot = cand;
                }
                members.entry(cand).or_default().push(i);
            }
            members.remove(&s);
        } else {
            kept.insert(s);
        }
    }
    // Paradigms: the chosen signatures, most words first.
    let mut count: BTreeMap<usize, u32> = BTreeMap::new();
    for &c in &chosen {
        *count.entry(c).or_default() += 1;
    }
    let mut order: Vec<(usize, u32)> = count.into_iter().collect();
    order.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
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
    let out = order.iter().filter_map(|&(s, n)| {
        let (lemma, cells) = signatures.get(s)?;
        let table: String = cells
            .iter()
            .enumerate()
            .map(|(cell, p)| {
                // A cell none of its words has comes from another; the
                // lemma's own cell is the lemma.
                let usual_p = match p {
                    Some(p) => p.clone(),
                    None if shape.lemma_cell == Some(cell) => lemma.clone(),
                    None => DERIVED.to_string(),
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
        Some(Paradigm {
            name: Cow::Owned(name(words, &chosen, s)),
            words: n,
            lemma: Cow::Owned(lemma.clone()),
            parent: None,
            inherent: Cow::Borrowed(&[]),
            table: Cow::Owned(table),
            stems: Cow::Borrowed(""),
            reads: None,
        })
    });
    let id = |c: usize| order.iter().position(|e| e.0 == c);
    let assignment = chosen
        .iter()
        .map(|&c| id(c).and_then(|i| u16::try_from(i).ok()).unwrap_or(0))
        .collect();
    (out.collect(), assignment)
}

/// The lemma patterns of `by_last` (see [`paradigms`]) that `letters`
/// fits, longest ending first, with its stems.
fn fitting<'a>(
    by_last: &'a HashMap<&'a str, Vec<&'a str>>,
    letters: &'a str,
) -> impl Iterator<Item = (&'a str, pattern::Stems<'a>)> {
    choose::endings(letters)
        .chain(std::iter::once(""))
        .filter_map(|e| by_last.get(e))
        .flatten()
        .filter_map(|&lp| Some((lp, pattern::bind(lp, letters)?)))
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
        let values = (0..features).filter_map(|f| {
            let f = u8::try_from(f).ok()?;
            let of = counts.range((p, f, 0)..=(p, f, u8::MAX));
            let total: u32 = of.clone().map(|(_, n)| n).sum();
            let (&(_, _, v), &n) = of.max_by_key(|(_, n)| **n)?;
            (20 * n >= 19 * total && n >= MIN_WORDS).then_some((f, v))
        });
        paradigm.inherent = Cow::Owned(values.collect());
    }
}

/// The stems each word of `words` the grammar builds with the wrong letters
/// (`letters`: per word, per cell) may list, as [`crate::stems::listed`]
/// finds them: `(word, slot, stem, table)`, by name.
fn listed_stems(
    grammar: &Grammar<'_, u16, u16, u8>,
    words: &[Word],
    letters: &HashMap<&str, Vec<bool>>,
    prices: &crate::bits::Prices,
) -> Vec<flexion::ListedStem> {
    let mine: Vec<(&Word, &Vec<bool>)> = words
        .iter()
        .filter_map(|w| Some((w, letters.get(w.key.as_str())?)))
        .collect();
    let chunk = mine.len().div_ceil(THREADS).max(1);
    let mut out: Vec<flexion::ListedStem> = std::thread::scope(|scope| {
        let handles: Vec<_> = mine
            .chunks(chunk)
            .map(|part| {
                scope.spawn(move || {
                    let mut out = Vec::new();
                    for &(w, flags) in part {
                        let word = crate::stems::Wrong {
                            key: &w.key,
                            cells: (0..w.cells.len())
                                .map(|c| {
                                    let f = first_form(w, c)?;
                                    Some((f, flags.get(c).copied().unwrap_or(false)))
                                })
                                .collect(),
                        };
                        for (s, stem, t) in crate::stems::listed(grammar, &word, prices) {
                            let (Some(slot), Some(table)) =
                                (grammar.slots.get(s), grammar.tables.get(t))
                            else {
                                continue;
                            };
                            out.push((
                                w.key.clone(),
                                slot.name.to_string(),
                                stem,
                                table.name.to_string(),
                            ));
                        }
                    }
                    out
                })
            })
            .collect();
        handles
            .into_iter()
            .filter_map(|h| h.join().ok())
            .flatten()
            .collect()
    });
    out.sort();
    out
}

/// The forms listed for the words with `wrong` forms (`(key, cell, form)`,
/// sorted) when they list `stems`, and for the words `made` of others:
/// those no relation tells from another listed or built one, then those
/// told wrong, until none is (see [`settle`]); every word listing anything
/// added to `unsure`.
fn listing(
    grammar: &mut Grammar<'_, u16, u16, u8>,
    stems: &[flexion::ListedStem],
    wrong: &[(String, usize, String)],
    (lexemes, words): (&[Lexeme], &[Word]),
    made: &dyn Fn(&Word) -> bool,
    unsure: &mut crate::HashSet<String>,
) -> Vec<(String, usize, String)> {
    grammar.irregular = flexion::Irregular::new(&[], stems);
    let wrong = rebuilt(grammar, words, stems, wrong.to_vec());
    unsure.extend(wrong.iter().map(|(k, _, _)| k.clone()));
    let mut irregular = roots(grammar, &wrong);
    settle(grammar, &mut irregular, stems, lexemes, words, unsure);
    let made_wrong = self::irregular(grammar, lexemes, words, made);
    unsure.extend(made_wrong.iter().map(|(k, _, _)| k.clone()));
    irregular.extend(made_wrong);
    settle(grammar, &mut irregular, stems, lexemes, words, unsure);
    irregular
}

/// The stems of `stems` the words listing them keep: those whose listing on
/// them (`on_stems`: the forms still listed, `(word, cell, form)`) costs
/// fewer bits with the stems than the word's listing of `forms` only; a
/// word judged with the words made of it (`base`: the word a word is made
/// of, or itself), which take its stems. An ending table is kept where the
/// words built with it save more than stating it costs; a word using a
/// table not kept lists its forms.
fn cheaper(
    grammar: &Grammar<'_, u16, u16, u8>,
    forms: &[(String, usize, String)],
    on_stems: &[(String, usize, String)],
    stems: Vec<flexion::ListedStem>,
    prices: &crate::bits::Prices,
    base: &dyn Fn(&str) -> String,
) -> Vec<flexion::ListedStem> {
    let refs = crate::bits::log2(grammar.slots.len()) + crate::bits::log2(grammar.tables.len());
    let mut bits: HashMap<String, (f64, f64)> = HashMap::default();
    for (k, _, f) in forms {
        bits.entry(base(k)).or_default().0 += prices.listed(f);
    }
    for (k, _, f) in on_stems {
        bits.entry(base(k)).or_default().1 += prices.listed(f);
    }
    for (k, _, stem, _) in &stems {
        bits.entry(base(k)).or_default().1 += prices.text(stem) + refs;
    }
    // What stating each table costs: an item, and its patterns.
    let cost = |slot: &str, name: &str| -> f64 {
        let s = grammar.slots.iter().position(|s| s.name == slot);
        let table = grammar
            .tables
            .iter()
            .find(|t| Some(usize::from(t.slot)) == s && t.name == name);
        table.map_or(0.0, |t| {
            crate::bits::ITEM + t.table.split('\t').map(|p| prices.text(p)).sum::<f64>()
        })
    };
    let mut on: crate::HashSet<String> = bits
        .iter()
        .filter(|(_, b)| b.1 < b.0)
        .map(|(k, _)| k.clone())
        .collect();
    // Tables that don't pay go, from the one saving least, until all pay.
    loop {
        let mut saved: BTreeMap<(&str, &str), (f64, crate::HashSet<String>)> = BTreeMap::new();
        for (k, slot, _, name) in &stems {
            let group = base(k);
            if !on.contains(&group) {
                continue;
            }
            let entry = saved.entry((slot.as_str(), name.as_str())).or_default();
            if entry.1.insert(group.clone()) {
                entry.0 += bits.get(&group).map_or(0.0, |b| b.0 - b.1);
            }
        }
        let worst = saved
            .iter()
            .map(|(t, (s, groups))| (s - cost(t.0, t.1), t, groups))
            .filter(|(net, ..)| *net <= 0.0)
            .min_by(|a, b| a.0.total_cmp(&b.0).then_with(|| a.1.cmp(b.1)));
        let Some((_, _, groups)) = worst else { break };
        for g in groups.clone() {
            on.remove(&g);
        }
    }
    stems
        .into_iter()
        .filter(|(k, ..)| on.contains(&base(k)))
        .collect()
}

/// The forms `wrong` (`(key, cell, form)`, sorted) and those of the cells
/// listed `stems` build, as the grammar now builds them: those still built
/// with the wrong letters (a form only stressed wrong is checked later).
fn rebuilt(
    grammar: &Grammar<'_, u16, u16, u8>,
    words: &[Word],
    stems: &[flexion::ListedStem],
    wrong: Vec<(String, usize, String)>,
) -> Vec<(String, usize, String)> {
    let o = &grammar.orthography;
    let by_key: HashMap<&str, &Word> = words.iter().map(|w| (w.key.as_str(), w)).collect();
    let mut out: Vec<(String, usize, String)> = wrong
        .into_iter()
        .filter(|(k, c, f)| {
            grammar
                .stem_form(k, *c)
                .is_none_or(|b| b != o.read(f).letters)
        })
        .collect();
    for (key, slot, _, _) in stems {
        let (Some(w), Some(slot)) = (
            by_key.get(key.as_str()),
            grammar.slots.iter().find(|s| &s.name == slot),
        ) else {
            continue;
        };
        for &c in slot.cells.iter() {
            let c = usize::from(c);
            let Some(form) = w.cells.get(c).and_then(|fs| fs.first()) else {
                continue;
            };
            if grammar.stem_form(key, c).is_some_and(|b| b != form.letters) {
                let written = o.write(&form.letters, form.stress, flexion::Casing::Lower);
                out.push((key.clone(), c, written));
            }
        }
    }
    out.sort();
    out.dedup_by(|a, b| a.0 == b.0 && a.1 == b.1);
    out
}

/// How many times the forms derivations from listed forms get wrong are
/// listed, at most.
const ROUNDS: usize = 8;

/// Lists `irregular` in `grammar`, then, while the words of `keys` have
/// forms told wrong from their listed ones (or their accent misplaced),
/// those too.
fn settle(
    grammar: &mut Grammar<'_, u16, u16, u8>,
    irregular: &mut Vec<(String, usize, String)>,
    stems: &[flexion::ListedStem],
    lexemes: &[Lexeme],
    words: &[Word],
    keys: &crate::HashSet<String>,
) {
    for _ in 0..ROUNDS {
        irregular.sort();
        grammar.irregular = flexion::Irregular::new(irregular, stems);
        let more = self::irregular(grammar, lexemes, words, |w| keys.contains(&w.key));
        if more.is_empty() {
            return;
        }
        irregular.extend(more);
    }
    irregular.sort();
    grammar.irregular = flexion::Irregular::new(irregular, stems);
}

/// Whether relation `r` tells the forms of the words whose form of the cell
/// it comes from is `wrong` (sorted) right as often as their paradigm does.
fn tells_listed(
    r: &Relation<'_>,
    wrong: &[(String, usize, String)],
    words: &[Word],
    smoothing: f32,
    laws: Option<&sound_law::Program<'_>>,
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
        told += usize::from(r.told(source, smoothing, laws).is_some_and(|t| t.0 == want));
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
        let built = |c: usize| grammar.stem_form(key, c);
        for c in word_roots(grammar, &order, &forms, &built) {
            if let Some(&form) = forms.get(&c) {
                out.push((key.clone(), c, form.clone()));
            }
        }
    }
    out.sort();
    out
}

/// Of a word's listed `forms` (by cell), the cells no relation tells from
/// another listed form of the word, or from a form `built` on a stem it
/// lists, or one told so in turn (`order`: the cells, each after the one it
/// is told from).
fn word_roots(
    grammar: &Grammar<'_, u16, u16, u8>,
    order: &[usize],
    forms: &BTreeMap<usize, &String>,
    built: &dyn Fn(usize) -> Option<String>,
) -> Vec<usize> {
    let mut out = Vec::new();
    // Per cell, the letters the word has there: listed, built, or told.
    let mut have: BTreeMap<usize, String> = BTreeMap::new();
    for &c in order {
        let told = grammar.relation(c).filter(|r| r.listed).and_then(|r| {
            let source = have.get(&r.source()?)?;
            r.told(source, grammar.smoothing, grammar.laws.as_ref())
                .map(|t| t.0)
        });
        // A form built on a listed stem is the cell's unless a form is listed
        // (relations tell no cell a stem builds); it tells others as a
        // listed one.
        let on_stem = built(c);
        let listed = forms.get(&c).map(|&form| {
            let letters = grammar.orthography.read(form).letters;
            if on_stem.is_some() || told.as_ref() != Some(&letters) {
                out.push(c);
            }
            letters
        });
        if let Some(letters) = listed.or(on_stem).or(told) {
            have.insert(c, letters);
        }
    }
    out
}

/// The fewest words a paradigm or a stress pattern is learned from.
const MIN_WORDS: u32 = 3;

/// Paradigms the laws left with fewer than MIN_WORDS words go: each of their
/// words inflects like the paradigm (of those kept) building the most of its
/// forms, the others listed.
fn rehome<'s>(
    orthography: &Orthography<'_>,
    words: &[crate::laws::Word<'_>],
    paradigms: Vec<Paradigm<'s>>,
    mut assignment: Vec<u16>,
    laws: Option<&sound_law::Program<'_>>,
) -> (Vec<Paradigm<'s>>, Vec<u16>) {
    let kept: Vec<bool> = (0..paradigms.len())
        .map(|p| assignment.iter().filter(|&&a| usize::from(a) == p).count() >= MIN_WORDS as usize)
        .collect();
    for (w, slot) in words.iter().zip(assignment.iter_mut()) {
        if kept.get(usize::from(*slot)) == Some(&true) {
            continue;
        }
        let built = |par: &Paradigm<'_>| builds(orthography, &paradigms, laws, par, w);
        let best = paradigms
            .iter()
            .zip(&kept)
            .enumerate()
            .filter(|(_, (_, kept))| **kept)
            .map(|(p, (par, _))| (built(par), std::cmp::Reverse(p)))
            .max()
            .filter(|&(n, _)| n > 0);
        if let Some((_, std::cmp::Reverse(p))) = best
            && let Ok(p) = u16::try_from(p)
        {
            *slot = p;
        }
    }
    crate::laws::drop_empty(paradigms, assignment)
}

/// Each paradigm's cells its relation tells from the paradigm's own form
/// of the cell it comes from with its one change, derived, where the forms
/// of its words the
/// relation tells wrong cost fewer bits listed than the cell's pattern
/// stated (Finnish `talossani`: `talossa` and `ni`, for every noun).
fn derive(
    paradigms: &mut [Paradigm<'_>],
    relations: &[Relation<'_>],
    assigned: &[(&Word, u16)],
    prices: &crate::bits::Prices,
    smoothing: f32,
    laws: Option<&sound_law::Program<'_>>,
) {
    let mut members: Vec<Vec<&Word>> = vec![Vec::new(); paradigms.len()];
    for &(w, p) in assigned {
        if let Some(m) = members.get_mut(usize::from(p)) {
            m.push(w);
        }
    }
    for (paradigm, words) in paradigms.iter_mut().zip(&members) {
        let mut cells: Vec<String> = paradigm.table.split('\t').map(str::to_string).collect();
        // A relation with one change makes every word's form the same way:
        // a held-out word's too (one choosing by ending may not).
        for r in relations
            .iter()
            .filter(|r| r.analogy.is_none() && r.changes.len() == 1)
        {
            let (c, Some(s)) = (usize::from(r.cell), r.source()) else {
                continue;
            };
            let Some(pattern) = cells
                .get(c)
                .filter(|m| !m.is_empty() && m.as_str() != DERIVED && !m.contains('|'))
            else {
                continue;
            };
            // The forms it tells wrong, listed; judged on a word at least.
            let pairs: Vec<(&str, &str)> = words
                .iter()
                .filter_map(|w| Some((first_form(w, s)?, first_form(w, c)?)))
                .collect();
            let wrong: f64 = pairs
                .iter()
                .filter(|(from, form)| {
                    r.told(from, smoothing, laws)
                        .is_none_or(|(told, _)| told != *form)
                })
                .map(|(_, form)| prices.listed(form))
                .sum();
            if !pairs.is_empty()
                && wrong < prices.text(pattern)
                && let Some(slot) = cells.get_mut(c)
            {
                *slot = DERIVED.to_string();
            }
        }
        paradigm.table = Cow::Owned(cells.join("\t"));
    }
}

/// How many of `w`'s forms `par` builds right.
fn builds(
    orthography: &Orthography<'_>,
    paradigms: &[Paradigm<'_>],
    laws: Option<&sound_law::Program<'_>>,
    par: &Paradigm<'_>,
    w: &crate::laws::Word<'_>,
) -> usize {
    let Some(stems) = par.stems(orthography, w.letters) else {
        return 0;
    };
    w.forms
        .iter()
        .enumerate()
        .filter(|&(c, f)| {
            let built = || par.built(paradigms, laws, c, &stems);
            f.is_some_and(|f| built().first().is_some_and(|(b, _)| b == f))
        })
        .count()
}

/// Per word, the paradigm that builds the most of its forms (the first of
/// those that build as many), if any builds one.
fn fit(
    orthography: &Orthography<'_>,
    paradigms: &[Paradigm<'_>],
    laws: Option<&sound_law::Program<'_>>,
    words: &[Word],
) -> Vec<Option<u16>> {
    let best = |w: &Word| {
        let forms: Vec<Option<&str>> = (0..w.cells.len()).map(|c| first_form(w, c)).collect();
        let w = crate::laws::Word {
            letters: &w.letters,
            forms,
        };
        paradigms
            .iter()
            .enumerate()
            .map(|(p, par)| {
                (
                    builds(orthography, paradigms, laws, par, &w),
                    std::cmp::Reverse(p),
                )
            })
            .max()
            .filter(|&(n, _)| n > 0)
            .and_then(|(_, std::cmp::Reverse(p))| u16::try_from(p).ok())
    };
    let best = &best;
    let chunk = words.len().div_ceil(THREADS).max(1);
    std::thread::scope(|scope| {
        let handles: Vec<_> = words
            .chunks(chunk)
            .map(|part| scope.spawn(move || part.iter().map(best).collect::<Vec<_>>()))
            .collect();
        handles
            .into_iter()
            .filter_map(|h| h.join().ok())
            .flatten()
            .collect()
    })
}

/// A paradigm's name: its commonest word that no sense makes vulgar or
/// offensive, judged by how many dictionaries have it and how many senses
/// its entry has (the dumps give no frequencies; then the shortest, then
/// the first alphabetically), of its plain words (letters only) of at least
/// three letters if it has any.
fn name<T: PartialEq>(words: &[Word], chosen: &[T], s: T) -> String {
    use std::cmp::Reverse;
    let pick = |plain: bool| {
        let mine = words.iter().zip(chosen).filter(|&(_, c)| *c == s);
        mine.map(|(w, _)| w)
            .filter(|w| {
                !plain
                    || (w.letters.chars().count() >= 3
                        && w.letters.chars().all(char::is_alphabetic))
            })
            .max_by_key(|w| {
                let length = Reverse(w.letters.chars().count());
                (
                    !w.vulgar,
                    w.dictionaries,
                    w.senses,
                    length,
                    Reverse(&w.letters),
                )
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
    // Per word: its paradigm; per cell, its accent, letters and where the
    // ending starts; and where its lemma accents its stem.
    type Fact = (flexion::Stress, String, usize);
    let facts: Vec<(u16, Vec<Option<Fact>>, Option<usize>)> = words
        .iter()
        .zip(assignment)
        .map(|(w, &p)| {
            let Some(par) = paradigms.get(usize::from(p)) else {
                return (p, vec![None; shape.cells], None);
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
            let lemma = shape.lemma_cell.and_then(form);
            let stem = match lemma.and_then(|f| Some((f.stress?, &f.letters))) {
                Some((s, letters)) => Some(flexion::stress_position(orthography, letters, s)),
                None => {
                    let mut votes: BTreeMap<usize, u32> = BTreeMap::new();
                    for (s, letters, ending) in cells.iter().flatten() {
                        // The byte where the accented vowel is.
                        let mut vowels = letters
                            .char_indices()
                            .filter(|&(_, c)| orthography.is_vowel(c));
                        let at = vowels
                            .nth(usize::from(s.vowel))
                            .map_or(letters.len(), |(i, _)| i);
                        if at < *ending {
                            *votes
                                .entry(flexion::stress_position(orthography, letters, *s))
                                .or_default() += 1;
                        }
                    }
                    votes.iter().max_by_key(|e| *e.1).map(|e| *e.0)
                }
            };
            (p, cells, stem)
        })
        .collect();
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
    // Whether some place gives the accent, with finals read so.
    let explained = |fact: &Fact, circumflex: bool, long: bool, stem: Option<usize>| {
        let finals = Finals { circumflex, long };
        Place::ALL
            .iter()
            .any(|&place| explains(fact, finals, place, stem))
    };
    let either = |fact: &Fact, long: bool, stem: Option<usize>| {
        explained(fact, false, long, stem) || explained(fact, true, long, stem)
    };
    // Cells whose final -αι, -οι is long: where reading it long explains
    // more words' accents.
    let long_finals: Vec<u16> = (0..shape.cells)
        .filter(|&c| {
            let with = facts
                .iter()
                .filter_map(|(_, cells, stem)| Some((cells.get(c)?.as_ref()?, *stem)));
            let gain =
                with.map(|(f, s)| i32::from(either(f, true, s)) - i32::from(either(f, false, s)));
            gain.sum::<i32>() > 0
        })
        .filter_map(|c| u16::try_from(c).ok())
        .collect();
    let is_long = |c: usize| u16::try_from(c).is_ok_and(|c| long_finals.binary_search(&c).is_ok());
    // Per paradigm, the cells whose ending's last syllable, accented, most
    // often takes the circumflex: of the accents the choice of kind decides.
    let mut votes: BTreeMap<(u16, usize), i32> = BTreeMap::new();
    for (p, cells, stem) in &facts {
        for (c, fact) in cells.iter().enumerate() {
            let Some(fact) = fact else { continue };
            let acute = explained(fact, false, is_long(c), *stem);
            let circ = explained(fact, true, is_long(c), *stem);
            if acute != circ {
                *votes.entry((*p, c)).or_default() += if circ { 1 } else { -1 };
            }
        }
    }
    let mut circumflex: Vec<Vec<u16>> = vec![Vec::new(); paradigms.len()];
    for (&(p, c), &n) in &votes {
        if n > 0
            && let (Some(cells), Ok(c)) = (circumflex.get_mut(usize::from(p)), u16::try_from(c))
        {
            cells.push(c);
        }
    }
    let finals = |p: u16, c: usize| Finals {
        circumflex: circumflex
            .get(usize::from(p))
            .is_some_and(|cells| u16::try_from(c).is_ok_and(|c| cells.binary_search(&c).is_ok())),
        long: is_long(c),
    };
    // Per word, per cell, the places that give its accent (a bit per place
    // of `Place::ALL`).
    let bit = |p: Place| 1u8 << Place::ALL.iter().position(|&x| x == p).unwrap_or(0);
    let seen: Vec<Vec<Option<u8>>> = facts
        .iter()
        .map(|(p, cells, stem)| {
            let places = |(c, fact): (usize, &Option<Fact>)| {
                let fact = fact.as_ref()?;
                let fit = Place::ALL
                    .iter()
                    .filter(|&&place| explains(fact, finals(*p, c), place, *stem));
                Some(fit.fold(0u8, |m, &place| m | bit(place)))
            };
            cells.iter().enumerate().map(places).collect()
        })
        .collect();
    // Per cell, how many words each place fits: of the places that fit a
    // word, it is read as the one most words share.
    let mut usual_place: Vec<[u32; Place::ALL.len()]> = vec![[0; Place::ALL.len()]; shape.cells];
    for places in &seen {
        for (m, counts) in places.iter().zip(usual_place.iter_mut()) {
            for (i, n) in counts.iter_mut().enumerate() {
                *n += u32::from(m.is_some_and(|m| m & (1 << i) != 0));
            }
        }
    }
    let read = |c: usize, mask: Option<u8>| -> Place {
        let counts = usual_place.get(c).copied().unwrap_or([0; Place::ALL.len()]);
        Place::ALL
            .iter()
            .enumerate()
            .filter(|(i, _)| mask.is_none_or(|m| m & (1 << i) != 0))
            .max_by_key(|(i, _)| (counts.get(*i).copied().unwrap_or(0), std::cmp::Reverse(*i)))
            .map_or(Place::Stem, |(_, &p)| p)
    };
    // Patterns: each word's places, its cells without a form read as most
    // words have them; kept if MIN_WORDS words have it.
    let mut vectors: BTreeMap<Vec<Place>, u32> = BTreeMap::new();
    for places in seen.iter().filter(|s| s.iter().any(Option::is_some)) {
        let pat = places.iter().enumerate().map(|(c, &m)| read(c, m));
        *vectors.entry(pat.collect()).or_default() += 1;
    }
    let mut candidates: Vec<(Vec<Place>, u32)> = vectors
        .into_iter()
        .filter(|&(_, n)| n >= MIN_WORDS)
        .collect();
    candidates.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    // Each word: the commonest pattern that fits it, else the one agreeing
    // with it on the most cells (its other cells' forms are then listed).
    let of_word: Vec<Option<usize>> = seen
        .iter()
        .map(|places| {
            if places.iter().all(Option::is_none) {
                return None;
            }
            // Per cell, whether the pattern's place gives the word's accent
            // (`None` where the word has no accent there).
            let fit = |pat: &[Place]| -> Vec<Option<bool>> {
                let cells = pat.iter().zip(places);
                cells.map(|(&pl, p)| p.map(|m| m & bit(pl) != 0)).collect()
            };
            let agreement = |pat: &[Place]| fit(pat).into_iter().flatten().filter(|&f| f).count();
            let fits = |pat: &[Place]| fit(pat).into_iter().all(|f| f != Some(false));
            let ranked = candidates.iter().enumerate();
            let first = ranked.clone().find(|(_, (pat, _))| fits(pat));
            first
                .or_else(|| ranked.max_by_key(|(_, (pat, n))| (agreement(pat), *n)))
                .map(|(k, _)| k)
        })
        .collect();
    let uses = |k: usize| {
        let n = of_word.iter().filter(|&&x| x == Some(k)).count();
        u32::try_from(n).unwrap_or(u32::MAX)
    };
    // Most common first; patterns no word follows aren't kept.
    let mut order: Vec<usize> = (0..candidates.len()).filter(|&k| uses(k) > 0).collect();
    order.sort_by_key(|&k| (std::cmp::Reverse(uses(k)), k));
    let pattern_of: Vec<Option<u16>> = of_word
        .iter()
        .map(|&k| u16::try_from(order.iter().position(|&o| Some(o) == k)?).ok())
        .collect();
    let kept: Vec<(&Vec<Place>, u32)> = order
        .iter()
        .filter_map(|&k| Some((&candidates.get(k)?.0, uses(k))))
        .collect();
    // Each pattern is like the commoner one it shares the most cells with,
    // if that is at least half of them, and writes only the cells that
    // differ.
    let accent_patterns: Vec<AccentPattern<'s>> = kept
        .iter()
        .enumerate()
        .map(|(i, &(pat, n))| {
            let shared = |j: usize| {
                kept.get(j).map_or(0, |(q, _)| {
                    q.iter().zip(pat).filter(|(a, b)| a == b).count()
                })
            };
            let parent = (0..i)
                .map(|j| (shared(j), std::cmp::Reverse(j)))
                .max()
                .filter(|&(n, _)| 2 * n >= pat.len())
                .map(|(_, std::cmp::Reverse(j))| j);
            let theirs = parent.and_then(|j| kept.get(j)).map(|(q, _)| q);
            let places: String = pat
                .iter()
                .enumerate()
                .map(|(c, p)| match theirs.and_then(|q| q.get(c)) {
                    Some(q) if q == p => "\t".to_string(),
                    _ => format!("{p:?}\t"),
                })
                .collect();
            AccentPattern {
                name: Cow::Owned(name(words, &pattern_of, u16::try_from(i).ok())),
                words: n,
                parent: parent.and_then(|j| u16::try_from(j).ok()),
                places: Cow::Owned(places),
            }
        })
        .collect();
    // Classes: a pattern and the stem's stressed vowel, from the lemma's end.
    let class_of: Vec<Option<(u16, Option<u8>)>> = words
        .iter()
        .zip(&pattern_of)
        .zip(&facts)
        .map(|((w, &pattern), (_, _, stem))| {
            let n = flexion::stress_positions(orthography, &w.letters);
            let vowel = stem
                .and_then(|k| n.checked_sub(1)?.checked_sub(k))
                .and_then(|v| u8::try_from(v).ok());
            Some((pattern?, vowel))
        })
        .collect();
    let mut counts: BTreeMap<(u16, Option<u8>), u32> = BTreeMap::new();
    for &c in class_of.iter().flatten() {
        *counts.entry(c).or_default() += 1;
    }
    let mut counts: Vec<((u16, Option<u8>), u32)> = counts.into_iter().collect();
    counts.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    let class_id: Vec<Option<u16>> = class_of
        .iter()
        .map(|&c| u16::try_from(counts.iter().position(|e| Some(e.0) == c)?).ok())
        .collect();
    let classes: Vec<StressClass<'s>> = (0u16..)
        .zip(counts)
        .map(|(i, ((pattern, vowel), n))| StressClass {
            name: Cow::Owned(name(words, &class_id, Some(i))),
            pattern,
            vowel,
            words: n,
        })
        .collect();
    let examples: Vec<(String, u16)> = words
        .iter()
        .zip(&class_id)
        .filter_map(|(w, c)| Some((w.key.clone(), (*c)?)))
        .collect();
    let labels: HashMap<String, u16> = examples.iter().cloned().collect();
    // A class whose stem syllable the word hasn't got can't be its.
    let class = choose::learn(&examples, settings.detail(), |word, c| {
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

/// Sort a choice's listed words and drop repeats.
fn tidy<L: flexion::Label + Ord>(choice: &mut flexion::Choice<'_, L>) {
    let list = choice.exceptions.to_mut();
    list.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));
    list.dedup();
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
    misbuilt(grammar, lexemes, words, only)
        .into_iter()
        .map(|(k, c, f, _)| (k, c, f))
        .collect()
}

/// [`irregular`], each form with whether the grammar's letters are wrong
/// (not only its stress).
fn misbuilt(
    grammar: &Grammar<'_, u16, u16, u8>,
    lexemes: &[Lexeme],
    words: &[Word],
    only: impl Fn(&Word) -> bool,
) -> Vec<(String, usize, String, bool)> {
    let orthography = &grammar.orthography;
    let wrong = |l: &Lexeme, w: &Word| {
        let plain = orthography.write(&w.key, None, flexion::Casing::Lower);
        let tables = [(l.full.as_str(), true), (plain.as_str(), false)].map(|(input, full)| {
            let known = flexion::Known {
                full,
                ..flexion::Known::default()
            };
            grammar.table(input, &known)
        });
        let mut out = Vec::new();
        for (cell, forms) in l.cells.iter().enumerate() {
            let Some(want) = forms.first() else { continue };
            let tops = tables
                .iter()
                .map(|t| t.get(cell).and_then(|c| c.first()).map(|c| c.form.as_str()));
            let mut right = true;
            let mut letters = false;
            let read = orthography.read(want);
            for top in tops {
                right &= top.is_some_and(|t| crate::eval::same_form(orthography, t, want));
                letters |= top.is_none_or(|t| orthography.read(t).letters != read.letters);
            }
            if !right {
                let form = orthography.write(&read.letters, read.stress, flexion::Casing::Lower);
                out.push((w.key.clone(), cell, form, letters));
            }
        }
        out
    };
    let wrong = &wrong;
    let mine: Vec<(&Lexeme, &Word)> = lexemes.iter().zip(words).filter(|e| only(e.1)).collect();
    let chunk = mine.len().div_ceil(THREADS).max(1);
    let mut out: Vec<(String, usize, String, bool)> = std::thread::scope(|scope| {
        let handles: Vec<_> = mine
            .chunks(chunk)
            .map(|part| {
                scope.spawn(move || {
                    part.iter()
                        .flat_map(|&(l, w)| wrong(l, w))
                        .collect::<Vec<_>>()
                })
            })
            .collect();
        handles
            .into_iter()
            .filter_map(|h| h.join().ok())
            .flatten()
            .collect()
    });
    out.sort();
    out
}

/// Threads the learner uses at most.
const THREADS: usize = 8;
