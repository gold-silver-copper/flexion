//! `flexion-train`: turn a language description plus a Kaikki Wiktionary dump
//! into a finished inflection crate.
//!
//! Pipeline: [`extract`] reads the dump into lexemes, [`learn`] turns them into
//! a pruned suffix tree over inflection classes, [`eval`] measures it on
//! held-out words, and [`emit`] writes the crate. Nothing in here knows any
//! language; everything language-specific comes from a [`spec::Language`].

pub mod cells;
pub mod edit;
pub mod emit;
pub mod eval;
pub mod extract;
pub mod kaikki;
pub mod learn;
pub mod marks;
pub mod pipeline;
pub mod spec;

use extract::{PosData, Word};
use flexion::Spelling;
use flexion::write::SpellingSpec;
use spec::Language;

/// The runtime spelling spec for a language, with the tables its marks
/// imply derived from Unicode.
pub fn spelling_spec(lang: &Language) -> SpellingSpec {
    let s = &lang.spelling;
    let d = marks::derive(s);
    SpellingSpec {
        fold_case: s.fold_case,
        mark_monosyllables: s.mark_monosyllables,
        stress_marks: s.stress_marks.to_string(),
        stress_table: d.stress_table,
        vowels: d.vowels,
        implicit: s.implicit.to_string(),
        fold_from: d.fold.iter().map(|(a, _)| *a).collect(),
        fold_to: d.fold.iter().map(|(_, b)| *b).collect(),
        strip_marks: s.strip_marks.to_string(),
        unmark_from: d.unmark.iter().map(|(a, _)| *a).collect(),
        unmark_to: d.unmark.iter().map(|(_, b)| *b).collect(),
        syllabic: s.syllabic.to_string(),
        mute: s.mute.to_string(),
        diphthongs: s.diphthongs.concat(),
        hiatus: d.hiatus,
    }
}

/// A runtime [`Spelling`] view of a spec.
pub fn view(s: &SpellingSpec) -> Spelling<'_> {
    Spelling::new(s.fold_case, s.mark_monosyllables, s.strings())
}

/// Which tenth of the data a word belongs to: a hash of its key, so the split
/// is deterministic and every entry of a word lands in the same tenth.
pub fn bucket(key: &str) -> u64 {
    fnv(key) % 10
}

/// A second, independent split of the words into hundredths, for training
/// on a fraction of them (learning curves).
pub fn percentile(key: &str) -> u64 {
    (fnv(key) / 10) % 100
}

fn fnv(key: &str) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in key.bytes() {
        h ^= u64::from(b);
        h = h.wrapping_mul(0x0100_0000_01b3);
    }
    h
}

/// Held-out words (scored, never trained on).
pub const TEST_BUCKET: u64 = 0;
/// Words used to choose hyperparameters.
pub const DEV_BUCKET: u64 = 1;

/// Per part of speech: its shape and its learner parameters.
pub struct PosPlan {
    pub shape: learn::Shape,
    pub params: learn::Params,
    /// The cells of the citation's principal parts, and per cell which of
    /// them builds it (see [`train_pos`]); empty for none.
    pub citation: Vec<usize>,
    pub sources: Vec<u8>,
}

pub fn shapes(lang: &Language) -> Result<Vec<learn::Shape>, String> {
    lang.parts
        .iter()
        .map(|p| {
            let cells = cells::Cells::new(lang, p)?;
            let lemma_cell = match &p.lemma {
                Some((g, v)) => Some(cells.find(g, v).ok_or("lemma cell not found")?),
                None => None,
            };
            let optional = (0..cells.total)
                .map(|c| cells.group_of(c).is_some_and(|g| g.group.optional))
                .chain([false])
                .collect();
            let groups = cells.groups.iter().map(|g| g.len()).chain([1]).collect();
            Ok(learn::Shape {
                cells: cells.total + 1,
                inherent: p.inherent.len(),
                lemma_cell: cells.total,
                groups,
                unscored: lemma_cell.into_iter().chain([cells.total]).collect(),
                optional,
                capitalized: p.capitalized,
            })
        })
        .collect()
}

/// The restoration model of a part of speech, if it needs one: a
/// one-cell model from the lemma as typed to the lemma as written in full
/// (Latin `tago` → `tāgō`, Russian `еж` → `ёж`). `None` when every lemma is
/// already typed in full.
pub fn restoration(
    sp: &Spelling<'_>,
    words: &[Word],
    params: learn::Params,
) -> Option<flexion::write::PosTables> {
    if words
        .iter()
        .all(|w| w.lexemes.iter().all(|l| l.base == l.full))
    {
        return None;
    }
    let lexemes: Vec<extract::Lexeme> = words
        .iter()
        .flat_map(|w| w.lexemes.iter())
        .map(|l| extract::Lexeme {
            full: l.base.clone(),
            cells: vec![vec![l.full.clone()]],
            inherent: Vec::new(),
            lemma_stress: None,
            ..(*l).clone()
        })
        .collect();
    let data = extract::PosData {
        lexemes,
        editions: Vec::new(),
    };
    let restore_words = extract::words(&data);
    let shape = learn::Shape {
        cells: 1,
        inherent: 0,
        lemma_cell: 0,
        groups: vec![1],
        unscored: Vec::new(),
        optional: vec![false],
        capitalized: false,
    };
    let mut params = params;
    params.walk.syllable_depth = None;
    let (t, _) = learn::Learner::new(sp, shape, params).learn(&restore_words);
    Some(t)
}

/// One part of speech, trained: its main model, the model that restores its
/// typed lemmas, and the models that build cells from principal parts.
pub struct Trained {
    pub main: flexion::write::PosTables,
    pub restore: Option<flexion::write::PosTables>,
    /// `(the principal part's cell, its model)`.
    pub parts: Vec<(usize, flexion::write::PosTables)>,
}

/// Lay out trained parts of speech: mains first (so a part of speech keeps
/// its index), then restoration models, then part models, each main pointing
/// at its own.
pub fn assemble(parts: Vec<Trained>) -> Vec<flexion::write::PosTables> {
    let n = parts.len();
    let mut mains = Vec::with_capacity(n);
    let mut restores = Vec::new();
    let mut part_models = Vec::new();
    let n_restores = parts.iter().filter(|t| t.restore.is_some()).count();
    for t in parts {
        let mut main = t.main;
        if let Some(r) = t.restore {
            main.restore = Some(n + restores.len());
            restores.push(r);
        }
        main.part_models.clear();
        for (cell, model) in t.parts {
            main.part_models
                .push((cell, n + n_restores + part_models.len()));
            part_models.push(model);
        }
        mains.push(main);
    }
    mains.extend(restores);
    mains.extend(part_models);
    mains
}

/// The model that builds cells from principal part `part_cell`: every word's
/// form in that cell is the lemma, and the cells `keep` marks are its cells
/// (the others stay empty, which keeps the model small). `None` when no word
/// has the part.
pub fn part_model(
    sp: &Spelling<'_>,
    shape: &learn::Shape,
    params: learn::Params,
    words: &[Word],
    part_cell: usize,
    keep: &[bool],
) -> Option<flexion::write::PosTables> {
    let citation_cell = shape.lemma_cell;
    let mut lexemes = Vec::new();
    for w in words {
        for lex in &w.lexemes {
            let Some(form) = lex.cells.get(part_cell).and_then(|c| c.first()) else {
                continue;
            };
            let read = sp.read(form);
            let cells = lex
                .cells
                .iter()
                .enumerate()
                .map(|(c, forms)| {
                    if c == citation_cell {
                        vec![form.clone()]
                    } else if keep.get(c).copied().unwrap_or(false) {
                        forms.clone()
                    } else {
                        Vec::new()
                    }
                })
                .collect();
            lexemes.push(extract::Lexeme {
                lemma: form.clone(),
                base: read.base.to_string(),
                full: read.base.to_string(),
                key: read.key.to_string(),
                lemma_stress: read.stress,
                lemma_kind: read.kind,
                cells,
                ..(*lex).clone()
            });
        }
    }
    if lexemes.is_empty() {
        return None;
    }
    let data = extract::PosData {
        lexemes,
        editions: Vec::new(),
    };
    let part_words = extract::words(&data);
    let chooser = edit::Chooser::new(sp, &part_words, citation_cell);
    let params = learn::Params {
        joins: false,
        ..params
    };
    let (t, _) = learn::Learner::new(sp, shape.clone(), params)
        .with_chooser(&chooser)
        .learn(&part_words);
    Some(t)
}

/// Train one part of speech: the main model (reading pairs as `chooser`
/// says), its restoration model, and a part model for every principal part
/// some cell is built from (`sources`, see
/// [`flexion::write::PosTables::sources`]).
pub fn train_pos(
    sp: &Spelling<'_>,
    shape: &learn::Shape,
    params: learn::Params,
    words: &[Word],
    chooser: &edit::Chooser,
    citation: &[usize],
    sources: &[u8],
) -> (Trained, learn::Stats) {
    let restore = restoration(sp, words, params);
    let (mut main, stats) = learn::Learner::new(sp, shape.clone(), params)
        .with_chooser(chooser)
        .learn(words);
    let mut parts = Vec::new();
    let mut remap = vec![0u8; citation.len().saturating_add(1)];
    for (k, &cell) in citation.iter().enumerate() {
        let tag = u8::try_from(k.saturating_add(1)).unwrap_or(0);
        let keep: Vec<bool> = (0..shape.cells)
            .map(|c| sources.get(c) == Some(&tag))
            .collect();
        if !keep.contains(&true) {
            continue;
        }
        if let Some(t) = part_model(sp, shape, params, words, cell, &keep) {
            parts.push((cell, t));
            if let Some(slot) = remap.get_mut(k.saturating_add(1)) {
                *slot = u8::try_from(parts.len()).unwrap_or(0);
            }
        }
    }
    // Sources name citation parts; the blob numbers the part models built.
    main.sources = (0..shape.cells)
        .map(|c| {
            let s = usize::from(sources.get(c).copied().unwrap_or(0));
            remap.get(s).copied().unwrap_or(0)
        })
        .collect();
    (
        Trained {
            main,
            restore,
            parts,
        },
        stats,
    )
}

/// The lexicon of part of speech `pi` (see
/// [`spec::PartOfSpeech::lemmas_from`]): every form of the linked groups of
/// other parts of speech, among the words `keep` accepts, by lookup key. All
/// of them, not only those the model would get wrong: a word missing from
/// the lexicon must mean the other part of speech doesn't attest it, or the
/// model's confidence for the words it does attest would be too low. A key
/// attested with two different written forms gives nothing, and so does a
/// word of the part of speech's own (its own entry decides: the adjective
/// `latus`, wide, is no participle `lātus`).
pub fn lexicon(
    lang: &Language,
    sp: &Spelling<'_>,
    data: &[PosData],
    pi: usize,
    keep: &dyn Fn(&str) -> bool,
) -> Vec<(String, String)> {
    let Some(part) = lang.parts.get(pi) else {
        return Vec::new();
    };
    let mut found: std::collections::BTreeMap<String, Vec<String>> = Default::default();
    for (pos, group) in &part.lemmas_from {
        let Some(qi) = lang.parts.iter().position(|q| q.name == *pos) else {
            continue;
        };
        let (Some(q), Some(d)) = (lang.parts.get(qi), data.get(qi)) else {
            continue;
        };
        let Ok(cells) = cells::Cells::new(lang, q) else {
            continue;
        };
        let Some(g) = cells.groups.iter().find(|g| g.name == *group) else {
            continue;
        };
        for lex in d.lexemes.iter().filter(|l| keep(&l.key)) {
            for forms in lex.cells.iter().skip(g.offset).take(g.len()) {
                for f in forms {
                    let written = found.entry(sp.read(f).key.to_string()).or_default();
                    if !written.contains(f) {
                        written.push(f.clone());
                    }
                }
            }
        }
    }
    let own: std::collections::BTreeSet<&str> = data
        .get(pi)
        .map(|d| {
            d.lexemes
                .iter()
                .filter(|l| keep(&l.key))
                .map(|l| l.key.as_str())
                .collect()
        })
        .unwrap_or_default();
    found
        .into_iter()
        .filter(|(key, _)| !own.contains(key.as_str()))
        .filter_map(|(key, written)| match written.as_slice() {
            [one] => Some((key, one.clone())),
            _ => None,
        })
        .collect()
}

/// Train every part of speech on the words `keep` accepts and write a blob.
pub fn train(
    lang: &Language,
    data: &[PosData],
    plans: &[PosPlan],
    keep: &dyn Fn(&str) -> bool,
) -> (Vec<u8>, Vec<learn::Stats>) {
    let spec = spelling_spec(lang);
    let sp = view(&spec);
    let mut tables = Vec::new();
    let mut stats = Vec::new();
    for (d, plan) in data.iter().zip(plans) {
        let words: Vec<Word> = extract::words(d)
            .into_iter()
            .filter(|w| keep(w.key))
            .collect();
        let chooser = edit::Chooser::new(&sp, &words, plan.shape.lemma_cell);
        let (mut t, s) = train_pos(
            &sp,
            &plan.shape,
            plan.params,
            &words,
            &chooser,
            &plan.citation,
            &plan.sources,
        );
        t.main.lexicon = lexicon(lang, &sp, data, tables.len(), keep);
        tables.push(t);
        stats.push(s);
    }
    (flexion::write::write(&spec, &assemble(tables)), stats)
}
