// The fixture models the check generates code for, shared by `build.rs`
// and the test: `(module, language, joins, start restoration)`.

const CASES: &[(&str, &str, bool, bool)] = &[
    ("english", "english", false, false),
    ("russian", "russian", false, false),
    ("latin", "latin", false, false),
    ("latin_front", "latin", false, true),
    ("polish", "polish", false, false),
    ("german", "german", false, false),
    ("german_joins", "german", true, false),
    ("ancient_greek", "ancient-greek", false, false),
    ("sanskrit", "sanskrit", false, false),
    ("spanish", "spanish", false, false),
    ("french", "french", false, false),
    ("italian", "italian", false, false),
];

/// Train `language` on its fixture with default parameters (and the join
/// model or start restoration where asked).
fn train_fixture(
    language: &str,
    joins: bool,
    front: bool,
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
