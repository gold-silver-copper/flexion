use crate::format::{NONE, Stress};
use crate::write::{ClassSpec, EditSpec, NodeSpec, PosTables, SpellingSpec, write};
use crate::{Model, Options};

fn edit(strip: &str, add: &str) -> EditSpec {
    EditSpec {
        prefix: String::new(),
        strip: strip.into(),
        add: add.into(),
        stress: Stress::Unmarked,
    }
}

fn node(label: &str, first_child: u32, children: u32, dist: u32, dist_len: u32) -> NodeSpec {
    NodeSpec {
        label: label.into(),
        first_child,
        children,
        dist,
        dist_len,
    }
}

/// Nouns with cells [singular, plural]: +s everywhere, y → ies after "y",
/// and one memorized exception, "ox" → "oxen".
fn tiny() -> Vec<u8> {
    let spelling = SpellingSpec {
        fold_case: true,
        ..Default::default()
    };
    let nouns = PosTables {
        cells: 2,
        inherent: 0,
        lemma_cell: Some(0),
        groups: vec![],
        restore: None,
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
                inherent: vec![],
            },
            ClassSpec {
                edits: vec![0, 2],
                strip: "y".into(),
                inherent: vec![],
            },
            ClassSpec {
                edits: vec![0, 3],
                strip: String::new(),
                inherent: vec![],
            },
            ClassSpec {
                edits: vec![0, NONE],
                strip: String::new(),
                inherent: vec![],
            },
        ],
        // root → "y" (child 1), "xo\u{1}" (child 2)
        nodes: vec![
            node("", 1, 2, 0, 2),
            node("xo\u{1}", 0, 0, 2, 1),
            node("y", 0, 0, 3, 2),
        ],
        dist: vec![(0, 0.9), (1, 0.1), (2, 1.0), (1, 0.8), (0, 0.2)],
    };
    write(&spelling, &[nouns])
}

#[test]
fn walks_to_the_deepest_node_and_applies_edits() {
    let blob = tiny();
    let model = Model::from_bytes(&blob).expect("valid");
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
fn citations_pin_and_filter() {
    let blob = tiny();
    let model = Model::from_bytes(&blob).expect("valid");
    let nouns = model.pos(0).expect("pos");
    // A given part is output exactly.
    let c = nouns.citation("city, citys", &[1]);
    assert_eq!(nouns.candidates(&c, 1, Options::default())[0].form, "citys");
    // Malformed citations never panic.
    let c = nouns.citation(",, ,", &[1]);
    assert_eq!(nouns.candidates(&c, 1, Options::default())[0].form, "");
}

#[test]
fn corrupt_blobs_are_rejected() {
    let blob = tiny();
    for cut in [0, 3, 20, 71, blob.len() / 2] {
        assert!(Model::from_bytes(&blob[..cut]).is_none(), "cut at {cut}");
    }
}
