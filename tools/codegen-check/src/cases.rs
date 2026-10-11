// The fixture models the check generates code for, shared by `build.rs`
// and the test: `(module, language, joins, start restoration, sound laws,
// derived lexicon)`.

const CASES: &[(&str, &str, bool, bool, bool, bool)] = &[
    ("english", "english", false, false, false, false),
    ("russian", "russian", false, false, false, false),
    ("russian_laws", "russian", false, false, true, false),
    ("russian_derived", "russian", false, false, false, true),
    ("latin", "latin", false, false, false, false),
    ("latin_front", "latin", false, true, false, false),
    ("latin_derived", "latin", false, false, false, true),
    ("polish", "polish", false, false, false, false),
    ("polish_laws", "polish", false, false, true, false),
    ("german", "german", false, false, false, false),
    ("german_joins", "german", true, false, false, false),
    ("german_laws", "german", false, false, true, false),
    ("ancient_greek", "ancient-greek", false, false, false, false),
    ("ancient_greek_laws", "ancient-greek", false, false, true, false),
    ("ancient_greek_derived", "ancient-greek", false, false, false, true),
    ("sanskrit", "sanskrit", false, false, false, false),
    ("spanish", "spanish", false, false, false, false),
    ("french", "french", false, false, false, false),
    ("italian", "italian", false, false, false, false),
];

/// Train `language` on its fixture with default parameters (and the join
/// model, start restoration, sound laws or a derived lexicon where asked).
fn train_fixture(
    language: &str,
    joins: bool,
    front: bool,
    laws: bool,
    derive: bool,
) -> Result<
    (
        flexion_train::spec::Language,
        Vec<flexion_train::extract::PosData>,
        flexion::Built,
    ),
    String,
> {
    let lang = flexion_configs::by_name(language).ok_or("no such language")?;
    let spec = flexion_train::spelling_spec(&lang);
    let sp = flexion_train::view(&spec);
    let fixture = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../configs/tests/fixtures")
        .join(format!("{language}.jsonl"));
    let data = flexion_train::extract::extract(&lang, &sp, &fixture)?;
    let params = flexion_train::learn::Params {
        joins,
        front_restore: front,
        phonology: laws,
        derive_lexicon: derive,
        ..Default::default()
    };
    let plans: Vec<flexion_train::PosPlan> = flexion_train::shapes(&lang)?
        .into_iter()
        .map(|shape| flexion_train::PosPlan {
            shape,
            params,
            citation: Vec::new(),
            sources: Vec::new(),
        })
        .collect();
    let (built, _) = flexion_train::train(&lang, &data, &plans, &|_| true);
    let built = built.ok_or("the tables make no model")?;
    Ok((lang, data, built))
}

/// A hand-built model whose edits set off a sound law (`k → c`, `g → dz`
/// before a softening ending), as the fixtures are too small to learn one:
/// nouns with cells [lemma, locative].
fn handmade() -> Result<flexion::Built, String> {
    use flexion::tables::{
        ClassSpec, EditSpec, NodeKind, NodeSpec, PhonologySpec, PosTables, RuleSpec,
    };
    let trigger = flexion::TRIGGER_FIRST;
    let class = |edits: Vec<u32>, strip: &str| ClassSpec {
        edits,
        strip: strip.into(),
        front: String::new(),
        inherent: Vec::new(),
        example: String::new(),
    };
    let nouns = PosTables {
        cells: 2,
        lemma_cell: Some(0),
        edits: vec![
            EditSpec::suffix("", ""),
            EditSpec::suffix("a", &format!("{trigger}e")),
            EditSpec::suffix("", "u"),
        ],
        classes: vec![class(vec![0, 1], "a"), class(vec![0, 2], "")],
        // root → "a" (the softening class).
        nodes: vec![
            NodeSpec {
                label: String::new(),
                children: 1,
                kind: NodeKind::Root,
                counts: vec![(1, 1.0)],
            },
            NodeSpec {
                label: "a".into(),
                children: 0,
                kind: NodeKind::Inner,
                counts: vec![(0, 9.0), (1, 1.0)],
            },
        ],
        phonology: Some(PhonologySpec {
            classes: vec![
                ("S0".into(), vec!["k".into(), "g".into()]),
                ("TRIGGER".into(), vec![trigger.to_string()]),
            ],
            maps: vec![("M0".into(), vec![("k".into(), "c".into()), ("g".into(), "dz".into())])],
            rules: vec![
                RuleSpec {
                    name: "end_0".into(),
                    target: "{x:S0}".into(),
                    replacement: "{x|M0}".into(),
                    left: String::new(),
                    right: trigger.to_string(),
                },
                RuleSpec {
                    name: "cleanup".into(),
                    target: "{t:TRIGGER}".into(),
                    ..Default::default()
                },
            ],
        }),
        ..Default::default()
    };
    flexion::Built::new(&flexion::tables::SpellingSpec::default(), &[nouns])
        .ok_or_else(|| "the hand-built model makes none".to_string())
}
