//! Lists the source the trainer's caches are keyed by (see `src/saved.rs`):
//! every source file of the trainer, the runtime and `sound-law`, and the
//! lock file, but the printer's and the report's, which only format what is
//! kept, and `sound-law`'s command-line tool, which computes none of it.

use std::fmt::Write as _;
use std::path::{Path, PathBuf};

/// The trainer's files that only format.
const PRINTER: &[&str] = &["print.rs", "report.rs"];

/// The `.rs` files under `dir`, by their path from `prefix`.
fn files(dir: &Path, prefix: &str, out: &mut Vec<(String, PathBuf)>) -> std::io::Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().to_string();
        let path = entry.path();
        if path.is_dir() {
            files(&path, &format!("{prefix}{name}/"), out)?;
        } else if name.ends_with(".rs") {
            out.push((format!("{prefix}{name}"), path));
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let here = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?);
    let src = here.join("src");
    let runtime = here.join("../flexion/src");
    let laws = here.join("../sound-law/src");
    let lock = here.join("../../Cargo.lock");
    let mut all = Vec::new();
    files(&src, "", &mut all)?;
    files(&runtime, "../../flexion/src/", &mut all)?;
    files(&laws, "../../sound-law/src/", &mut all)?;
    all.retain(|(name, _)| {
        !PRINTER.contains(&name.as_str()) && !name.starts_with("../../sound-law/src/bin/")
    });
    all.push(("../../../Cargo.lock".to_string(), lock.clone()));
    all.sort();
    let mut out = String::from("&[\n");
    for (name, path) in &all {
        let path = path.canonicalize()?;
        writeln!(
            out,
            "    ({name:?}, include_str!({:?})),",
            path.display().to_string()
        )?;
    }
    out.push_str("]\n");
    std::fs::write(
        PathBuf::from(std::env::var("OUT_DIR")?).join("sources.rs"),
        out,
    )?;
    for dir in [&src, &runtime, &laws, &lock] {
        println!("cargo::rerun-if-changed={}", dir.display());
    }
    Ok(())
}
