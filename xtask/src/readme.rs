//! `cargo xtask readme`: the root README, written from its text
//! (`readme.md`) and what it reads there from the builds and the crates:
//!
//! - `{{table results|errors|compatible|shipped}}`: the crates' report
//!   tables, a row per language and part of speech, from the builds kept in
//!   `target/build-cache` (see `flexion_train::saved`), each checked to be
//!   the build its crate's README was printed from;
//! - `{{table size}}`: each crate's source, from the files;
//! - `{{table costs}}`: the build and call costs measured by hand
//!   (`costs.tsv`), a row per language;
//! - `{{cell <language> | <part> | <column>}}`: one cell of a crate's report;
//! - `{{new word <language> | <part>}}`: a small part's score on a new word;
//! - `{{below the floor}}`: the parts below the floor beyond chance;
//! - `{{cells <language> | <part>}}`, `{{max words <language> | <part>}}`:
//!   from the config;
//! - `{{mb [<path>]}}`: the source under `languages/<path>` (all of it), in
//!   MB;
//! - `{{excerpt <path> | <start> | <lines> [| <width>]}}`: lines of a crate
//!   file, from the first (after the code block's last excerpt of that
//!   file) that starts with `<start>`, each cut at `<width>` characters;
//! - `{{languages}}`, `{{count}}`, `{{crates}}`, `{{develop}}`, `{{small}}`:
//!   the languages (from `flexion_configs::all`), their number, the
//!   workspace's crates, the development tenth and the size under which a
//!   part is small.

use crate::bytes;
use flexion_train::pipeline::Measured;
use flexion_train::report::{self, Tables};
use flexion_train::spec::Language;
use std::collections::BTreeMap;
use std::path::Path;

/// The README's text, with what it reads in `{{…}}`.
const TEMPLATE: &str = include_str!("readme.md");

/// The costs measured by hand, a row per language.
const COSTS: &str = include_str!("costs.tsv");

/// A language's build as the README reads it.
struct Built {
    lang: Language,
    tables: Tables,
    measured: Vec<Measured>,
}

/// Write `root`'s README from the builds kept in `cache` and the crates.
pub fn write(root: &Path, cache: &Path) -> Result<(), String> {
    check_crates(root)?;
    let built = flexion_configs::all()
        .into_iter()
        .map(|lang| load(root, cache, lang))
        .collect::<Result<Vec<_>, _>>()?;
    let text = render(root, &built)?;
    let path = root.join("README.md");
    std::fs::write(&path, text).map_err(|e| format!("{}: {e}", path.display()))
}

/// Every crate under `languages/` is a configured language's, and every
/// configured language has one.
fn check_crates(root: &Path) -> Result<(), String> {
    let dir = root.join("languages");
    let crates: std::collections::BTreeSet<String> = std::fs::read_dir(&dir)
        .map_err(|e| format!("{}: {e}", dir.display()))?
        .flatten()
        .filter(|e| e.path().is_dir())
        .filter_map(|e| e.file_name().into_string().ok())
        .collect();
    let configs: std::collections::BTreeSet<String> = flexion_configs::all()
        .iter()
        .map(|l| l.name.to_string())
        .collect();
    let stale: Vec<&String> = crates.difference(&configs).collect();
    let missing: Vec<&String> = configs.difference(&crates).collect();
    if stale.is_empty() && missing.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "languages/ has crates no config builds: {stale:?}; configs with no crate: {missing:?}"
        ))
    }
}

/// `lang`'s build kept in `cache` (one is kept per language), checked to be
/// the one its crate's README reports.
fn load(root: &Path, cache: &Path, lang: Language) -> Result<Built, String> {
    let key = std::fs::read_dir(cache)
        .map_err(|e| format!("{}: {e}", cache.display()))?
        .flatten()
        .filter_map(|e| e.file_name().into_string().ok())
        .find_map(|n| {
            let hex = n.strip_prefix(lang.name)?.strip_prefix('-')?;
            let hex = hex.strip_suffix(".json.gz")?;
            (hex.len() == 16).then(|| u64::from_str_radix(hex, 16).ok())?
        })
        .ok_or(format!(
            "no build of {} kept: cargo xtask build {}",
            lang.name, lang.name
        ))?;
    let script = flexion_train::script::Script::new(&lang.spelling);
    let (learned, measured) = flexion_train::saved::load(cache, lang.name, key, &script)?
        .ok_or(format!("no build of {} kept", lang.name))?;
    let readme = root.join("languages").join(lang.name).join("README.md");
    let printed =
        std::fs::read_to_string(&readme).map_err(|e| format!("{}: {e}", readme.display()))?;
    let report = report::report(&lang, &learned, &measured);
    if !printed.contains(report.trim_end()) {
        return Err(format!(
            "{} isn't printed from the build kept: cargo xtask build {}",
            readme.display(),
            lang.name
        ));
    }
    let tables = report::tables(&lang, &learned, &measured);
    Ok(Built {
        lang,
        tables,
        measured,
    })
}

/// The template with every `{{…}}` read.
fn render(root: &Path, built: &[Built]) -> Result<String, String> {
    let mut out = String::new();
    // Per file, the line after the code block's last excerpt of it.
    let mut cursors: BTreeMap<String, usize> = BTreeMap::new();
    for line in TEMPLATE.lines() {
        if line.starts_with("```") {
            cursors.clear();
        }
        let mut rest = line;
        while let Some((before, after)) = rest.split_once("{{") {
            let (inner, after) = after
                .split_once("}}")
                .ok_or(format!("an unclosed {{{{ in: {line}"))?;
            out.push_str(before);
            let text = directive(root, built, inner, &mut cursors)
                .map_err(|e| format!("{{{{{inner}}}}}: {e}"))?;
            out.push_str(&text);
            rest = after;
        }
        out.push_str(rest);
        out.push('\n');
    }
    Ok(out)
}

/// What `{{inner}}` reads.
fn directive(
    root: &Path,
    built: &[Built],
    inner: &str,
    cursors: &mut BTreeMap<String, usize>,
) -> Result<String, String> {
    const NAMES: [&str; 13] = [
        "below the floor",
        "max words",
        "new word",
        "languages",
        "develop",
        "excerpt",
        "crates",
        "count",
        "cells",
        "small",
        "table",
        "cell",
        "mb",
    ];
    let inner = inner.trim();
    let (name, rest) = NAMES
        .iter()
        .find_map(|n| {
            let rest = inner.strip_prefix(*n)?;
            (rest.is_empty() || rest.starts_with(' ')).then_some((*n, rest.trim()))
        })
        .ok_or("no such directive")?;
    let args: Vec<&str> = if rest.is_empty() {
        Vec::new()
    } else {
        rest.split(" | ").map(str::trim).collect()
    };
    let arg = |i: usize| args.get(i).copied().ok_or("too few arguments");
    let lang = |i: usize| -> Result<&Built, String> {
        let name = arg(i)?;
        built
            .iter()
            .find(|b| b.lang.name == name)
            .ok_or(format!("no language {name}"))
    };
    match name {
        "table" => table(root, built, arg(0)?),
        "cell" => {
            let (b, part, column) = (lang(0)?, arg(1)?, arg(2)?);
            let cell = b
                .tables
                .cell(part, column)
                .ok_or(format!("no {part} {column}"))?;
            Ok(prose(cell))
        }
        "new word" => {
            let (b, part) = (lang(0)?, arg(1)?);
            let small = b
                .measured
                .iter()
                .find(|m| m.name == part)
                .and_then(|m| m.small.as_ref())
                .ok_or(format!("{part} isn't small"))?;
            Ok(format!("{:.2}%", 100.0 * small.left_out.accuracy()))
        }
        "below the floor" => Ok(below_floor(built)),
        "cells" => {
            let b = lang(0)?;
            let part = spec(&b.lang, arg(1)?)?;
            let cells = flexion_train::cells::Cells::new(&b.lang, part)?;
            Ok(thousands(cells.total))
        }
        "max words" => {
            let b = lang(0)?;
            let part = spec(&b.lang, arg(1)?)?;
            part.max_words
                .map(thousands)
                .ok_or("learns from every word".into())
        }
        "mb" => {
            let under = root.join("languages").join(args.first().unwrap_or(&""));
            let sizes = bytes::on_disk(root)?;
            let n: u64 = sizes
                .iter()
                .filter(|(path, _)| root.join(path).starts_with(&under))
                .map(|(_, n)| n)
                .sum();
            if n == 0 {
                return Err("no source there".into());
            }
            Ok(bytes::mb(n))
        }
        "excerpt" => excerpt(root, &args, cursors),
        "languages" => {
            let links: Vec<String> = built
                .iter()
                .map(|b| format!("[{}](languages/{})", b.lang.display, b.lang.name))
                .collect();
            Ok(match links.split_last() {
                Some((last, [])) => last.clone(),
                Some((last, rest)) => format!("{} and {last}", rest.join(", ")),
                None => String::new(),
            })
        }
        "count" => Ok(number(built.len())),
        "crates" => crates(root),
        "develop" => Ok(flexion_train::eval::DEVELOP.to_string()),
        "small" => Ok(flexion_train::pipeline::SMALL.to_string()),
        _ => Err("no such directive".into()),
    }
}

/// `lang`'s part of speech `name`.
fn spec<'l>(
    lang: &'l Language,
    name: &str,
) -> Result<&'l flexion_train::spec::PartOfSpeech, String> {
    lang.parts
        .iter()
        .find(|p| p.name == name)
        .ok_or(format!("{} has no {name}", lang.name))
}

/// A table of the README, its rows each language's in turn.
fn table(root: &Path, built: &[Built], name: &str) -> Result<String, String> {
    let named = |columns: &[&str]| -> Vec<String> {
        ["language", "part of speech"]
            .iter()
            .chain(columns)
            .map(ToString::to_string)
            .collect()
    };
    // Each language's rows of one of its tables, the cells `columns` read.
    let rows = |pick: &dyn Fn(&Tables) -> Option<&report::Table>,
                columns: &dyn Fn(&report::Table) -> Vec<String>|
     -> Vec<Vec<String>> {
        built
            .iter()
            .filter_map(|b| Some((b, pick(&b.tables)?)))
            .flat_map(|(b, t)| {
                let picked = columns(t);
                t.rows.iter().map(move |(part, _)| {
                    [b.lang.name.to_string(), part.clone()]
                        .into_iter()
                        .chain(picked.iter().map(|c| {
                            let cell = t.cell(part, c).unwrap_or("");
                            // The floor's unseen cells are left to the crates.
                            if c == "floor (unseen)" {
                                cell.split(" (").next().unwrap_or("").to_string()
                            } else {
                                cell.to_string()
                            }
                        }))
                        .collect()
                })
            })
            .collect()
    };
    let whole = |pick: &dyn Fn(&Tables) -> Option<&report::Table>| -> Result<String, String> {
        let first = built
            .iter()
            .find_map(|b| pick(&b.tables))
            .ok_or("no language has it")?;
        let columns: Vec<&str> = first.columns.iter().map(String::as_str).collect();
        let rows = rows(pick, &|t| t.columns.clone());
        Ok(report::markdown(&named(&columns), 2, rows.into_iter()))
    };
    let text = match name {
        "results" => {
            // The crate's column, and the README's name for it.
            let columns = [
                ("top-1", "top-1 from the headword"),
                ("lemma in full", "lemma in full"),
                ("principal parts given", "principal parts given"),
                ("features given", "features given"),
                ("floor (unseen)", "floor"),
                ("against the floor", "against the floor"),
                (
                    "on a training word / new root",
                    "on a training word / new root",
                ),
                ("any listed form", "any listed form"),
                ("homograph ceiling", "homograph ceiling"),
                ("lemma spelling", "lemma spelling"),
                ("own / form-of / alt-of", "own / form-of / alt-of"),
                ("phrases", "phrases"),
                ("irregular lines", "irregular lines"),
                ("calibration error", "calibration error"),
            ];
            let names: Vec<&str> = columns.iter().map(|(_, n)| *n).collect();
            let rows = rows(&|t| Some(&t.results), &|_| {
                columns.iter().map(|(c, _)| (*c).to_string()).collect()
            });
            report::markdown(&named(&names), 2, rows.into_iter())
        }
        "errors" => whole(&|t| Some(&t.errors))?,
        "compatible" => whole(&|t| t.compatible.as_ref())?,
        "shipped" => whole(&|t| Some(&t.shipped))?,
        "size" => {
            let mut crates: BTreeMap<String, u64> = BTreeMap::new();
            for (path, n) in bytes::on_disk(root)? {
                if let Some(at) = bytes::place(&path) {
                    *crates.entry(at.0).or_default() += n;
                }
            }
            let mut crates: Vec<(String, u64)> = crates.into_iter().collect();
            crates.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
            let rows = crates.into_iter().map(|(c, n)| vec![c, bytes::mb(n)]);
            report::markdown(&["crate".into(), "MB".into()], 1, rows)
        }
        "costs" => costs(built)?,
        _ => return Err(format!("no table {name}")),
    };
    Ok(text.trim_end().to_string())
}

/// The costs measured by hand, a row per language (– where none is).
fn costs(built: &[Built]) -> Result<String, String> {
    let mut lines = COSTS.lines().map(|l| l.split('\t').collect::<Vec<_>>());
    let header = lines.next().ok_or("costs.tsv is empty")?;
    let rows: BTreeMap<&str, Vec<&str>> = lines
        .filter_map(|f| Some((*f.first()?, f.get(1..)?.to_vec())))
        .collect();
    if let Some(stray) = rows
        .keys()
        .find(|l| !built.iter().any(|b| b.lang.name == **l))
    {
        return Err(format!("costs.tsv has {stray}, no configured language"));
    }
    let columns: Vec<String> = header.iter().map(ToString::to_string).collect();
    let width = columns.len().saturating_sub(1);
    let table = built.iter().map(|b| {
        let cells = rows.get(b.lang.name);
        std::iter::once(b.lang.name.to_string())
            .chain((0..width).map(|i| {
                cells
                    .and_then(|c| c.get(i))
                    .map_or("–".to_string(), ToString::to_string)
            }))
            .collect()
    });
    Ok(report::markdown(&columns, 1, table))
}

/// The parts of speech whose grammar is below the floor beyond chance:
/// `German nouns (−1.02), …`.
fn below_floor(built: &[Built]) -> String {
    let parts: Vec<String> = built
        .iter()
        .flat_map(|b| b.measured.iter().map(move |m| (b, m)))
        .filter(|(_, m)| m.headword.forms > 0)
        .filter_map(|(b, m)| {
            let f = &m.against_floor;
            // As the table rounds it: an interval up to −0.00 is no loss.
            let (change, high) = (format!("{:+.2}", 100.0 * (f.b - f.a)), 100.0 * f.high);
            ((high * 100.0).round() < 0.0).then(|| {
                format!(
                    "{} {}s ({})",
                    b.lang.display,
                    m.name.replace('_', " "),
                    change.replace('-', "−")
                )
            })
        })
        .collect();
    match parts.split_last() {
        Some((last, [])) => last.clone(),
        Some((last, rest)) => format!("{} and {last}", rest.join(", ")),
        None => "none".into(),
    }
}

/// A table cell as prose: unbolded, its bracket (an interval, a count) left
/// to the table, a count with its thousands marked.
fn prose(cell: &str) -> String {
    let cell = cell.replace("**", "");
    let cell = match cell.split_once(" (") {
        Some((value, _)) if cell.ends_with(')') => value.to_string(),
        _ => cell,
    };
    cell.parse::<usize>().map_or(cell, thousands)
}

/// `n` with its thousands marked: `12,546`.
fn thousands(n: usize) -> String {
    let digits = n.to_string();
    let mut out = String::new();
    for (i, c) in digits.chars().enumerate() {
        if i > 0 && (digits.len() - i).is_multiple_of(3) {
            out.push(',');
        }
        out.push(c);
    }
    out
}

/// `n` in words, capitalized, up to twenty.
fn number(n: usize) -> String {
    const WORDS: [&str; 21] = [
        "No",
        "One",
        "Two",
        "Three",
        "Four",
        "Five",
        "Six",
        "Seven",
        "Eight",
        "Nine",
        "Ten",
        "Eleven",
        "Twelve",
        "Thirteen",
        "Fourteen",
        "Fifteen",
        "Sixteen",
        "Seventeen",
        "Eighteen",
        "Nineteen",
        "Twenty",
    ];
    WORDS
        .get(n)
        .map_or_else(|| n.to_string(), ToString::to_string)
}

/// Lines of a crate's file (see the module's doc).
fn excerpt(
    root: &Path,
    args: &[&str],
    cursors: &mut BTreeMap<String, usize>,
) -> Result<String, String> {
    let [path, start, lines, rest @ ..] = args else {
        return Err("needs a path, a start and a number of lines".into());
    };
    let lines: usize = lines.parse().map_err(|_| "bad number of lines")?;
    let width: Option<usize> = match rest {
        [] => None,
        [w] => Some(w.parse().map_err(|_| "bad width")?),
        _ => return Err("too many arguments".into()),
    };
    let file = root.join("languages").join(path);
    let text = std::fs::read_to_string(&file).map_err(|e| format!("{}: {e}", file.display()))?;
    let all: Vec<&str> = text.lines().collect();
    let from = cursors.get(*path).copied().unwrap_or(0);
    let at = all
        .iter()
        .enumerate()
        .skip(from)
        .find(|(_, l)| l.trim_start().starts_with(start))
        .map(|(i, _)| i)
        .ok_or(format!("no line starts with {start}"))?;
    let picked = all
        .get(at..at + lines)
        .ok_or(format!("fewer than {lines} lines from {start}"))?;
    cursors.insert((*path).to_string(), at + lines);
    let cut: Vec<String> = picked
        .iter()
        .map(|l| match width {
            Some(w) if l.chars().count() > w => {
                format!("{} …", l.chars().take(w).collect::<String>())
            }
            _ => (*l).to_string(),
        })
        .collect();
    Ok(cut.join("\n"))
}

/// The workspace's crates, each with its manifest's description.
fn crates(root: &Path) -> Result<String, String> {
    let dir = root.join("crates");
    let mut dirs: Vec<String> = std::fs::read_dir(&dir)
        .map_err(|e| format!("{}: {e}", dir.display()))?
        .flatten()
        .filter(|e| e.path().join("Cargo.toml").exists())
        .filter_map(|e| e.file_name().into_string().ok())
        .map(|n| format!("crates/{n}"))
        .collect();
    dirs.sort();
    dirs.push("xtask".into());
    dirs.iter()
        .map(|d| {
            let manifest = root.join(d).join("Cargo.toml");
            let text = std::fs::read_to_string(&manifest)
                .map_err(|e| format!("{}: {e}", manifest.display()))?;
            let description = text
                .lines()
                .find_map(|l| l.strip_prefix("description = \""))
                .and_then(|l| l.strip_suffix('"'))
                .ok_or(format!("{}: no description", manifest.display()))?;
            Ok(format!("- `{d}`: {description}."))
        })
        .collect::<Result<Vec<_>, String>>()
        .map(|l| l.join("\n"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_crate_is_a_configured_languages() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
        assert_eq!(check_crates(&root), Ok(()));
    }
}
