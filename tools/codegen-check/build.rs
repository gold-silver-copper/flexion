//! Generate code for each fixture model into `OUT_DIR/<module>.rs`.

include!("src/cases.rs");

fn main() -> Result<(), String> {
    let out = std::path::PathBuf::from(std::env::var("OUT_DIR").map_err(|e| e.to_string())?);
    println!("cargo::rerun-if-changed=../../configs/tests/fixtures");
    println!("cargo::rerun-if-changed=src/cases.rs");
    for &(module, language, joins, front, laws, derive) in CASES {
        let (lang, _, built) = train_fixture(language, joins, front, laws, derive)?;
        let code =
            flexion_train::codegen::single_file(&flexion_train::codegen::generate(&lang, &built));
        let path = out.join(format!("{module}.rs"));
        std::fs::write(&path, code).map_err(|e| format!("{}: {e}", path.display()))?;
    }
    let lang = flexion_configs::by_name("english").ok_or("no english")?;
    let code =
        flexion_train::codegen::single_file(&flexion_train::codegen::generate(&lang, &handmade()?));
    let path = out.join("handmade.rs");
    std::fs::write(&path, code).map_err(|e| format!("{}: {e}", path.display()))?;
    Ok(())
}
