use crate::tables::{Built, ClassSpec, EditSpec, NodeSpec, PosTables, SpellingSpec};
use crate::{NONE, Options};

fn edit(strip: &str, add: &str) -> EditSpec {
    EditSpec::suffix(strip, add)
}

fn node(label: &str, children: u32, dist_len: u32) -> NodeSpec {
    NodeSpec {
        label: label.into(),
        children,
        dist_len,
    }
}

/// Nouns with cells [singular, plural]: +s everywhere, y → ies after "y",
/// and one memorized exception, "ox" → "oxen".
fn tiny() -> Built {
    let spelling = SpellingSpec {
        fold_case: true,
        // `ö` is `o` for lookup, so `dög` is the lexicon's `dog` written in full.
        fold_from: "ö".into(),
        fold_to: "o".into(),
        ..Default::default()
    };
    let nouns = PosTables {
        cells: 2,
        inherent: 0,
        lemma_cell: Some(0),
        groups: vec![],
        restore: None,
        restore_front: None,
        capitalized: false,
        joins: vec![],
        join_classes: vec![],
        part_models: vec![],
        sources: vec![],
        lexicon: vec![("dog".into(), "dög".into())],
        walk: crate::Walk::default(),
        edits: vec![
            edit("", ""),
            edit("", "s"),
            edit("y", "ies"),
            edit("", "en"),
        ],
        classes: vec![
            ClassSpec {
                edits: vec![0, 1],
                strip: String::new(),
                front: String::new(),
                inherent: vec![],
                example: String::new(),
            },
            ClassSpec {
                edits: vec![0, 2],
                strip: "y".into(),
                front: String::new(),
                inherent: vec![],
                example: String::new(),
            },
            ClassSpec {
                edits: vec![0, 3],
                strip: String::new(),
                front: String::new(),
                inherent: vec![],
                example: String::new(),
            },
            ClassSpec {
                edits: vec![0, NONE],
                strip: String::new(),
                front: String::new(),
                inherent: vec![],
                example: String::new(),
            },
        ],
        // root → "y" (child 1), "xo\u{1}" (child 2)
        nodes: vec![node("", 2, 2), node("xo\u{1}", 0, 1), node("y", 0, 2)],
        dist: vec![(0, 0.9), (1, 0.1), (2, 1.0), (1, 0.8), (0, 0.2)],
    };
    Built::new(&spelling, &[nouns]).expect("valid")
}

#[test]
fn walks_to_the_deepest_node_and_applies_edits() {
    let built = tiny();
    let model = built.model();
    let nouns = model.pos(0).expect("pos");
    let ask = |w: &str| nouns.candidates(&nouns.citation(w, &[1]), 1, Options::default());
    assert_eq!(ask("cat")[0].form, "cats");
    assert_eq!(ask("city")[0].form, "cities");
    assert!((ask("city")[0].confidence - 0.8).abs() < 1e-6);
    assert_eq!(ask("ox")[0].form, "oxen");
    // Casing is restored; a Title-case word finds the lowercase entry.
    assert_eq!(ask("Ox")[0].form, "Oxen");
    assert_eq!(ask("CAT")[0].form, "CATS");
}

#[test]
fn the_lexicon_writes_a_plain_lemma_in_full() {
    let built = tiny();
    let model = built.model();
    let nouns = model.pos(0).expect("pos");
    let ask = |w: &str| nouns.best(&nouns.citation(w, &[1]), 1, Options::default());
    assert_eq!(ask("dog"), "dögs");
    assert_eq!(ask("Dog"), "Dögs");
    assert_eq!(ask("dög"), "dögs");
    assert_eq!(ask("cat"), "cats");
}

#[test]
fn citations_pin_and_filter() {
    let built = tiny();
    let model = built.model();
    let nouns = model.pos(0).expect("pos");
    // A given part is output exactly.
    let c = nouns.citation("city, citys", &[1]);
    assert_eq!(nouns.candidates(&c, 1, Options::default())[0].form, "citys");
    // Malformed citations never panic.
    let c = nouns.citation(",, ,", &[1]);
    assert_eq!(nouns.candidates(&c, 1, Options::default())[0].form, "");
}

#[test]
fn tables_that_make_no_model_are_rejected() {
    let spelling = SpellingSpec::default();
    // No tree.
    assert!(Built::new(&spelling, &[PosTables::default()]).is_none());
    // A part model that points at itself.
    let mut nouns = PosTables {
        cells: 1,
        nodes: vec![node("", 0, 0)],
        ..Default::default()
    };
    assert!(Built::new(&spelling, std::slice::from_ref(&nouns)).is_some());
    nouns.part_models = vec![(0, 0)];
    assert!(Built::new(&spelling, &[nouns]).is_none());
}

/// Nouns read through two restoration models: the end model writes `-us` as
/// `-ūs`, the start model (over the word written backwards) writes `cons-`
/// as `cōns-`.
fn restored() -> Built {
    let spelling = SpellingSpec {
        fold_case: true,
        fold_from: "ōū".into(),
        fold_to: "ou".into(),
        ..Default::default()
    };
    let one = |edits: Vec<EditSpec>, classes: Vec<ClassSpec>, dist: Vec<(u32, f32)>| PosTables {
        cells: 1,
        inherent: 0,
        lemma_cell: Some(0),
        groups: vec![],
        restore: None,
        restore_front: None,
        capitalized: false,
        joins: vec![],
        join_classes: vec![],
        part_models: vec![],
        sources: vec![],
        lexicon: vec![],
        walk: crate::Walk::default(),
        edits,
        classes,
        nodes: vec![node("", 0, u32::try_from(dist.len()).unwrap_or(0))],
        dist,
    };
    let class = |edit: u32, strip: &str| ClassSpec {
        edits: vec![edit],
        strip: strip.into(),
        front: String::new(),
        inherent: vec![],
        example: String::new(),
    };
    let mut nouns = one(
        vec![edit("", ""), edit("", "s")],
        vec![ClassSpec {
            edits: vec![0, 1],
            strip: String::new(),
            front: String::new(),
            inherent: vec![],
            example: String::new(),
        }],
        vec![(0, 1.0)],
    );
    nouns.cells = 2;
    nouns.restore = Some(1);
    nouns.restore_front = Some(2);
    let end = one(vec![edit("us", "ūs")], vec![class(0, "us")], vec![(0, 1.0)]);
    // Backwards: `susnoc` → `susnōc`, or left as typed.
    let start = one(
        vec![edit("snoc", "snōc"), edit("", "")],
        vec![class(0, "snoc"), class(1, "")],
        vec![(0, 0.6), (1, 0.4)],
    );
    Built::new(&spelling, &[nouns, end, start]).expect("valid")
}

#[test]
fn the_start_model_restores_the_beginning_of_a_plain_lemma() {
    let built = restored();
    let model = built.model();
    let nouns = model.pos(0).expect("pos");
    let ask = |w: &str, cell| nouns.best(&nouns.citation(w, &[1]), cell, Options::default());
    assert_eq!(ask("consus", 0), "cōnsūs");
    assert_eq!(ask("consus", 1), "cōnsūss");
    // A principal part spells the word as written: neither model rewrites it.
    assert_eq!(ask("consus, consuss", 0), "consus");
}
