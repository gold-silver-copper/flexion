//! A build's learning kept on disk: per part of speech, the shipped grammar
//! ([`Learned`], all of it but its timings) and what was measured of it
//! ([`Measured`]), everything the printer and the report read. Kept under a
//! key that hashes:
//!
//! - the data the build learns from, the extracted files (see
//!   [`crate::extract::load`]);
//! - the language's config, what learning reads of it: all but the crate's
//!   own text (its display name, version, description, `extra.rs`, README
//!   intro), the nonce words and the hooks (they run when the dumps are
//!   extracted, so what they do is in the data);
//! - the settings;
//! - the source of everything that computes what is kept: the trainer, the
//!   runtime, `sound-law` and the lock file, but not the printer
//!   ([`crate::print`]) or the report ([`crate::report`]), which only format
//!   it.
//!
//! So a change to the printer or the report alone finds the build kept and
//! reprints it, and any other change learns afresh.
//!
//! The printer computes one thing learning reads: each cell's name
//! ([`crate::print::cell_names`], through [`crate::shape`]). Rather than the
//! printer's source, the key hashes what it gives, every part's
//! [`crate::learn::Shape`]: a printer change that renames cells learns
//! afresh, any other reprints.

use crate::Hashing;
use crate::choose::Settings;
use crate::learn::Learned;
use crate::pipeline::Measured;
use crate::script::Script;
use crate::spec::Language;
use flate2::{Compression, read::GzDecoder, write::GzEncoder};
use std::fmt::Write as _;
use std::io::{BufReader, BufWriter, Read as _};
use std::path::{Path, PathBuf};

/// Each file's path from here, with its text.
macro_rules! sources {
    ($($path:literal),* $(,)?) => {
        &[$(($path, include_str!($path))),*]
    };
}

/// The source of everything that computes what a build keeps: a change to
/// any of it is a new key. Every source file of the trainer, the runtime and
/// `sound-law` is here but the printer's (see the test below).
const SOURCE: &[(&str, &str)] = sources![
    "baseline.rs",
    "bits.rs",
    "ceiling.rs",
    "cells.rs",
    "choose.rs",
    "eval.rs",
    "extract.rs",
    "kaikki.rs",
    "laws.rs",
    "laws/harmony.rs",
    "learn.rs",
    "length.rs",
    "lib.rs",
    "measure.rs",
    "par.rs",
    "pipeline.rs",
    "relations.rs",
    "saved.rs",
    "script.rs",
    "skeleton.rs",
    "spec.rs",
    "stage.rs",
    "stems.rs",
    "trace.rs",
    "../../flexion/src/choice.rs",
    "../../flexion/src/grammar.rs",
    "../../flexion/src/lib.rs",
    "../../flexion/src/orthography.rs",
    "../../flexion/src/paradigm.rs",
    "../../flexion/src/pattern.rs",
    "../../flexion/src/stems.rs",
    "../../flexion/src/stress.rs",
    "../../sound-law/src/compile.rs",
    "../../sound-law/src/engine.rs",
    "../../sound-law/src/lib.rs",
    "../../sound-law/src/rust.rs",
    "../../../Cargo.lock",
];

/// What a build keeps: per part of speech, its shipped grammar and what was
/// measured of it.
pub type Kept<'s> = (Vec<Learned<'s>>, Vec<Measured>);

/// The key of `lang`'s build with `settings`, from the extracted `data`
/// files it learns from (a missing one, the editors' sidecar, counts as
/// missing).
pub fn key(lang: &Language, settings: Settings, data: &[&Path]) -> Result<u64, String> {
    let mut hash = Hashing::new();
    for (path, text) in SOURCE {
        write!(hash, "{path}\u{1f}{text}\u{1f}").map_err(|e| e.to_string())?;
    }
    for path in data {
        let bytes = match std::fs::File::open(path) {
            Ok(file) => std::io::copy(&mut BufReader::new(file), &mut hash)
                .map_err(|e| format!("{}: {e}", path.display()))?,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => u64::MAX,
            Err(e) => return Err(format!("{}: {e}", path.display())),
        };
        write!(hash, "\u{1f}{bytes}\u{1f}").map_err(|e| e.to_string())?;
    }
    // Named one by one, so that a new field is placed in the key or out of it.
    let Language {
        name,
        display: _,
        wiktionary,
        version: _,
        description: _,
        features,
        parts,
        spelling,
        tags,
        hooks: _,
        extra: _,
        readme_intro: _,
        nonce: _,
        editions,
    } = lang;
    write!(
        hash,
        "{name:?}{wiktionary:?}{features:?}{parts:?}{spelling:?}{tags:?}{editions:?}{settings:?}"
    )
    .map_err(|e| e.to_string())?;
    for part in parts {
        let shape = crate::shape(lang, part)?;
        write!(hash, "{shape:?}").map_err(|e| e.to_string())?;
    }
    Ok(hash.finish())
}

/// Where `lang`'s build is kept under `key` in `dir`.
fn path(dir: &Path, lang: &str, key: u64) -> PathBuf {
    dir.join(format!("{lang}-{key:016x}.json.gz"))
}

/// `lang`'s build kept in `dir` under `key`, its grammars written in
/// `script` (see [`flexion::Grammar::orthography`]); `None` if none is kept.
pub fn load<'s>(
    dir: &Path,
    lang: &str,
    key: u64,
    script: &'s Script,
) -> Result<Option<Kept<'s>>, String> {
    let path = path(dir, lang, key);
    let Ok(file) = std::fs::File::open(&path) else {
        return Ok(None);
    };
    let mut bytes = Vec::new();
    GzDecoder::new(BufReader::new(file))
        .read_to_end(&mut bytes)
        .map_err(|e| format!("{}: {e}", path.display()))?;
    let (mut learned, measured): Kept<'s> =
        serde_json::from_slice(&bytes).map_err(|e| format!("{}: {e}", path.display()))?;
    for l in &mut learned {
        l.grammar.orthography = script.orthography();
    }
    Ok(Some((learned, measured)))
}

/// Keep `lang`'s build in `dir` under `key`, and drop the language's builds
/// kept under other keys.
pub fn save(
    dir: &Path,
    lang: &str,
    key: u64,
    script: &Script,
    learned: &[Learned<'_>],
    measured: &[Measured],
) -> Result<(), String> {
    // Loading sets the grammars' orthography from the script: only what
    // that gives back is kept.
    if learned
        .iter()
        .any(|l| l.grammar.orthography != script.orthography())
    {
        return Err("a grammar's orthography isn't its script's: not kept".into());
    }
    std::fs::create_dir_all(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    let path = path(dir, lang, key);
    // Written beside it and renamed, so that a build reads all of it or none.
    let partial = path.with_extension("partial");
    let file =
        std::fs::File::create(&partial).map_err(|e| format!("{}: {e}", partial.display()))?;
    let mut gz = GzEncoder::new(BufWriter::new(file), Compression::fast());
    serde_json::to_writer(&mut gz, &(learned, measured)).map_err(|e| e.to_string())?;
    gz.finish()
        .and_then(|mut w| std::io::Write::flush(&mut w))
        .map_err(|e| format!("{}: {e}", partial.display()))?;
    std::fs::rename(&partial, &path).map_err(|e| format!("{}: {e}", path.display()))?;
    let kept = path.file_name();
    for entry in std::fs::read_dir(dir).into_iter().flatten().flatten() {
        let name = entry.file_name();
        let other = name.to_str().and_then(|n| {
            let hex = n.strip_prefix(lang)?.strip_prefix('-')?;
            hex.strip_suffix(".json.gz")
        });
        if other.is_some_and(|h| h.len() == 16) && Some(name.as_os_str()) != kept {
            let _ = std::fs::remove_file(entry.path());
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::SOURCE;
    use std::path::Path;

    /// The trainer's files left out of the key: they only format.
    const PRINTER: &[&str] = &["print.rs", "report.rs"];

    /// The `.rs` files under `dir`, by their path from `prefix`.
    fn files(dir: &Path, prefix: &str, out: &mut Vec<String>) {
        for entry in std::fs::read_dir(dir).expect("source dir").flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            let path = entry.path();
            if path.is_dir() {
                files(&path, &format!("{prefix}{name}/"), out);
            } else if name.ends_with(".rs") {
                out.push(format!("{prefix}{name}"));
            }
        }
    }

    #[test]
    fn every_source_file_is_in_the_key_or_only_formats() {
        let here = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let mut all = Vec::new();
        files(&here, "", &mut all);
        files(
            &here.join("../../flexion/src"),
            "../../flexion/src/",
            &mut all,
        );
        files(
            &here.join("../../sound-law/src"),
            "../../sound-law/src/",
            &mut all,
        );
        // `sound-law`'s command-line tool computes nothing a build keeps.
        all.retain(|f| !f.starts_with("../../sound-law/src/bin/"));
        let missing: Vec<&String> = all
            .iter()
            .filter(|f| !SOURCE.iter().any(|(s, _)| s == f) && !PRINTER.contains(&f.as_str()))
            .collect();
        assert!(
            missing.is_empty(),
            "in no key and not the printer's: {missing:?}"
        );
    }
}
