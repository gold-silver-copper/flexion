//! `cargo xtask bytes [--against <ref>]`: the generated crates' source, as
//! it ships (the files on disk, formatted), per crate, per file kind and per
//! part of speech; against a git revision's, the change.

use std::collections::BTreeMap;
use std::path::Path;

/// Each generated crate's source file under `root`, by its path from
/// `root`, with its bytes.
pub fn on_disk(root: &Path) -> Result<BTreeMap<String, u64>, String> {
    fn walk(root: &Path, dir: &Path, out: &mut BTreeMap<String, u64>) -> Result<(), String> {
        for entry in std::fs::read_dir(dir).map_err(|e| format!("{}: {e}", dir.display()))? {
            let entry = entry.map_err(|e| e.to_string())?;
            let path = entry.path();
            if path.is_dir() {
                walk(root, &path, out)?;
            } else if let Ok(from) = path.strip_prefix(root) {
                let name = from.to_string_lossy().to_string();
                if place(&name).is_some() {
                    let meta = entry.metadata().map_err(|e| e.to_string())?;
                    out.insert(name, meta.len());
                }
            }
        }
        Ok(())
    }
    let mut out = BTreeMap::new();
    walk(root, &root.join("languages"), &mut out)?;
    Ok(out)
}

/// The same at git revision `rev`.
fn at(root: &Path, rev: &str) -> Result<BTreeMap<String, u64>, String> {
    let out = std::process::Command::new("git")
        .args(["ls-tree", "-r", "-l", rev, "--", "languages/"])
        .current_dir(root)
        .output()
        .map_err(|e| format!("git: {e}"))?;
    if !out.status.success() {
        return Err(format!(
            "git ls-tree {rev}: {}",
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    Ok(String::from_utf8_lossy(&out.stdout)
        .lines()
        .filter_map(|l| {
            let (meta, path) = l.split_once('\t')?;
            let size = meta.split_whitespace().nth(3)?.parse().ok()?;
            place(path).map(|_| (path.to_string(), size))
        })
        .collect())
}

/// A source file's crate, part of speech (`(crate)` for the crate's own
/// files) and file name: `languages/<crate>/src/[<part>/]<file>.rs`.
pub fn place(path: &str) -> Option<(String, String, String)> {
    let mut parts = path.split('/');
    let (Some("languages"), Some(lang), Some("src")) = (parts.next(), parts.next(), parts.next())
    else {
        return None;
    };
    let rest: Vec<&str> = parts.collect();
    let (part, file) = match rest.as_slice() {
        [file] => ("(crate)", *file),
        [part, file] => (*part, *file),
        _ => return None,
    };
    file.ends_with(".rs")
        .then(|| (lang.to_string(), part.to_string(), file.to_string()))
}

/// Bytes in MB, as the READMEs print them.
#[allow(clippy::cast_precision_loss)]
pub fn mb(n: u64) -> String {
    format!("{:.2}", n as f64 / 1e6)
}

/// `cargo xtask bytes`: per crate, file kind and part of speech, the source
/// now, and against `rev`'s where given (the parts only where they differ).
pub fn report(root: &Path, rev: Option<&str>) -> Result<(), String> {
    let now = on_disk(root)?;
    let before = rev.map(|r| at(root, r)).transpose()?;
    // (crate, part, file) → (before, now).
    let mut all: BTreeMap<(String, String, String), (u64, u64)> = BTreeMap::new();
    for (side, files) in [(0, before.as_ref()), (1, Some(&now))] {
        for (path, n) in files.into_iter().flatten() {
            if let Some(at) = place(path) {
                let e = all.entry(at).or_default();
                if side == 0 {
                    e.0 += n;
                } else {
                    e.1 += n;
                }
            }
        }
    }
    let by = |key: &dyn Fn(&(String, String, String)) -> String| {
        let mut out: BTreeMap<String, (u64, u64)> = BTreeMap::new();
        for (k, (a, b)) in &all {
            let e = out.entry(key(k)).or_default();
            *e = (e.0 + a, e.1 + b);
        }
        let mut out: Vec<(String, (u64, u64))> = out.into_iter().collect();
        out.sort_by(|x, y| (y.1.1).cmp(&x.1.1).then_with(|| x.0.cmp(&y.0)));
        out
    };
    let line = |name: &str, (a, b): (u64, u64)| match rev {
        Some(_) => {
            #[allow(clippy::cast_possible_wrap)]
            let d = b as i64 - a as i64;
            format!("{name:<32} {:>8} {:>8} {d:>+10}", mb(a), mb(b))
        }
        None => format!("{name:<32} {:>8}", mb(b)),
    };
    let head = |what: &str| match rev {
        Some(r) => format!("{what:<32} {:>8} {:>8} {:>10}", r, "now", "bytes"),
        None => format!("{what:<32} {:>8}", "MB"),
    };
    println!("{}", head("crate"));
    let crates = by(&|k| k.0.clone());
    for (name, sizes) in &crates {
        println!("{}", line(name, *sizes));
    }
    let total = crates
        .iter()
        .fold((0, 0), |t, (_, (a, b))| (t.0 + a, t.1 + b));
    println!("{}", line("all", total));
    println!("\n{}", head("file kind"));
    for (name, sizes) in by(&|k| k.2.clone()) {
        println!("{}", line(&name, sizes));
    }
    let parts: Vec<(String, (u64, u64))> = by(&|k| format!("{} {}", k.0, k.1))
        .into_iter()
        .filter(|(_, (a, b))| rev.is_none() || a != b)
        .collect();
    if !parts.is_empty() {
        println!("\n{}", head("part of speech"));
        for (name, sizes) in parts {
            println!("{}", line(&name, sizes));
        }
    }
    Ok(())
}
