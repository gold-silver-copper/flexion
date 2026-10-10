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
pub mod pipeline;
pub mod spec;

use extract::{PosData, Word};
use flexion::Spelling;
use flexion::write::SpellingSpec;
use spec::Language;

/// The runtime spelling spec for a language.
pub fn spelling_spec(lang: &Language) -> SpellingSpec {
    let s = &lang.spelling;
    SpellingSpec {
        fold_case: s.fold_case,
        mark_monosyllables: s.mark_monosyllables,
        stress_mark: s.stress_mark,
        vowels: s.vowels.to_string(),
        implicit: s.implicit.to_string(),
        fold_from: s.fold.iter().map(|(a, _)| *a).collect(),
        fold_to: s.fold.iter().map(|(_, b)| *b).collect(),
        strip_marks: s.strip_marks.to_string(),
        unmark_from: s.unmark.iter().map(|(a, _)| *a).collect(),
        unmark_to: s.unmark.iter().map(|(_, b)| *b).collect(),
    }
}

/// A runtime [`Spelling`] view of a spec.
pub fn view(s: &SpellingSpec) -> Spelling<'_> {
    Spelling::new(
        s.fold_case,
        s.mark_monosyllables,
        s.stress_mark,
        &s.vowels,
        &s.implicit,
        &s.fold_from,
        &s.fold_to,
        &s.strip_marks,
        &s.unmark_from,
        &s.unmark_to,
    )
}

/// Which tenth of the data a word belongs to: a hash of its key, so the split
/// is deterministic and every entry of a word lands in the same tenth.
pub fn bucket(key: &str) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in key.bytes() {
        h ^= u64::from(b);
        h = h.wrapping_mul(0x0100_0000_01b3);
    }
    h % 10
}

/// Held-out words (scored, never trained on).
pub const TEST_BUCKET: u64 = 0;
/// Words used to choose hyperparameters.
pub const DEV_BUCKET: u64 = 1;

/// Per part of speech: its shape and its learner parameters.
pub struct PosPlan {
    pub shape: learn::Shape,
    pub params: learn::Params,
}

pub fn shapes(lang: &Language) -> Result<Vec<learn::Shape>, String> {
    lang.parts
        .iter()
        .map(|p| {
            let cells = cells::Cells::new(lang, p)?;
            let lemma_cell = cells
                .find(p.lemma.0, &p.lemma.1)
                .ok_or("lemma cell not found")?;
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
                unscored: vec![lemma_cell, cells.total],
                optional,
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
    let data = extract::PosData { lexemes };
    let restore_words = extract::words(&data);
    let shape = learn::Shape {
        cells: 1,
        inherent: 0,
        lemma_cell: 0,
        groups: vec![1],
        unscored: Vec::new(),
        optional: vec![false],
    };
    let mut params = params;
    params.walk.syllable_depth = None;
    let (t, _) = learn::Learner::new(sp, shape, params).learn(&restore_words);
    Some(t)
}

/// Lay out main tables and their restoration models: mains first (so a part
/// of speech keeps its index), restorations after, each main pointing at its.
pub fn assemble(
    parts: Vec<(flexion::write::PosTables, Option<flexion::write::PosTables>)>,
) -> Vec<flexion::write::PosTables> {
    let n = parts.len();
    let mut mains = Vec::with_capacity(n);
    let mut extras = Vec::new();
    for (mut main, restore) in parts {
        if let Some(r) = restore {
            main.restore = Some(n + extras.len());
            extras.push(r);
        }
        mains.push(main);
    }
    mains.extend(extras);
    mains
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
        let restore = restoration(&sp, &words, plan.params);
        let (t, s) = learn::Learner::new(&sp, plan.shape.clone(), plan.params).learn(&words);
        tables.push((t, restore));
        stats.push(s);
    }
    (flexion::write::write(&spec, &assemble(tables)), stats)
}
