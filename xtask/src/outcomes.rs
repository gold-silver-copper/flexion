//! Per-form outcomes of a `try`, saved to a file (`--save`), and the paired
//! comparison of two saved runs (`compare`).

use flexion_train::eval::{Input, Outcome};
use flexion_train::extract::Lexeme;
use flexion_train::measure::{self, Paired};
use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::Path;

/// A name for how the words were typed, as saved.
pub fn mode(input: Input<'_>) -> String {
    match input {
        Input::Full => "full".into(),
        Input::Headword => "headword".into(),
        Input::Citation(_) => "principal-parts".into(),
        Input::Features => "features".into(),
        Input::Given(k) => format!("given-{k}"),
    }
}

/// Each scored word's name in a saved run: its key, and after a `#` its
/// place among the scored words with that key (homographs).
pub fn word_ids(words: &[&Lexeme]) -> Vec<String> {
    let mut seen: BTreeMap<&str, usize> = BTreeMap::new();
    words
        .iter()
        .map(|l| {
            let n = seen.entry(l.key.as_str()).or_default();
            *n += 1;
            format!("{}#{}", l.key, *n)
        })
        .collect()
}

/// The rows of a run, as written: a header, then per form `part of speech,
/// mode, word, cell, right (1/0), the right answer's place (1 first; `-`
/// if no candidate), the first candidate's probability, the right
/// answer's, and its bits`, tab-separated.
#[derive(Debug, Default)]
pub struct Saved {
    text: String,
}

const HEADER: &str = "pos\tmode\tword\tcell\tright\trank\ttop\tprobability\tbits\n";

impl Saved {
    pub fn add(&mut self, pos: &str, mode: &str, ids: &[String], outcomes: &[Outcome]) {
        for o in outcomes {
            let word = ids.get(o.word).map_or("?", String::as_str);
            let rank = o.rank.map_or("-".to_string(), |r| (r + 1).to_string());
            let _ = writeln!(
                self.text,
                "{pos}\t{mode}\t{word}\t{}\t{}\t{rank}\t{:.4}\t{:.4}\t{:.3}",
                o.cell,
                u8::from(o.right()),
                o.top,
                o.probability,
                o.bits
            );
        }
    }

    pub fn write(&self, path: &Path) -> Result<(), String> {
        std::fs::write(path, format!("{HEADER}{}", self.text))
            .map_err(|e| format!("{}: {e}", path.display()))
    }
}

/// `(pos, mode)` → word → cell → right.
type Run = BTreeMap<(String, String), BTreeMap<String, BTreeMap<usize, bool>>>;

fn read(path: &Path) -> Result<Run, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let mut run = Run::new();
    for line in text.lines().skip(1) {
        let f: Vec<&str> = line.split('\t').collect();
        let (Some(pos), Some(mode), Some(word), Some(cell), Some(right)) =
            (f.first(), f.get(1), f.get(2), f.get(3), f.get(4))
        else {
            continue;
        };
        let cell: usize = cell
            .parse()
            .map_err(|_| format!("{}: bad line {line}", path.display()))?;
        run.entry(((*pos).to_string(), (*mode).to_string()))
            .or_default()
            .entry((*word).to_string())
            .or_default()
            .insert(cell, *right == "1");
    }
    Ok(run)
}

/// `cargo xtask compare <a> <b>`: per part of speech and mode, top-1 of
/// each run on the forms both scored, the paired bootstrap 95% interval of
/// b − a (resampling words), and the sign test over words.
pub fn compare(a: &Path, b: &Path) -> Result<(), String> {
    if a.is_dir() && b.is_dir() {
        return sweeps(a, b);
    }
    let (ra, rb) = (read(a)?, read(b)?);
    println!(
        "{:<10} {:<16} {:>6} {:>7} {:>8} {:>8} {:>8} {:>18} {:>11} {:>9}",
        "pos",
        "mode",
        "words",
        "forms",
        "a",
        "b",
        "b − a",
        "95% interval",
        "better/worse",
        "sign p"
    );
    for (key, wa) in &ra {
        let Some(wb) = rb.get(key) else {
            continue;
        };
        let mut paired = Vec::new();
        for (word, ca) in wa {
            let Some(cb) = wb.get(word) else {
                continue;
            };
            let mut p = Paired {
                forms: 0,
                a: 0,
                b: 0,
            };
            for (cell, &x) in ca {
                if let Some(&y) = cb.get(cell) {
                    p.forms += 1;
                    p.a += usize::from(x);
                    p.b += usize::from(y);
                }
            }
            if p.forms > 0 {
                paired.push(p);
            }
        }
        let c = measure::compare(&paired);
        println!(
            "{:<10} {:<16} {:>6} {:>7} {:>7.2}% {:>7.2}% {:>+7.2} {:>+8.2} … {:>+6.2} {:>6}/{:<5} {:>9.2e}",
            key.0,
            key.1,
            c.words,
            c.forms,
            100.0 * c.a,
            100.0 * c.b,
            100.0 * (c.b - c.a),
            100.0 * c.low,
            100.0 * c.high,
            c.better,
            c.worse,
            c.p
        );
    }
    Ok(())
}

/// Two sweeps (`cargo xtask sweep`): every language both have, compared as
/// two runs are, then each language's time and peak memory in b against a.
fn sweeps(a: &Path, b: &Path) -> Result<(), String> {
    let summary = |dir: &Path| -> BTreeMap<String, (f64, f64)> {
        std::fs::read_to_string(dir.join("summary.tsv"))
            .unwrap_or_default()
            .lines()
            .skip(1)
            .filter_map(|l| {
                let f: Vec<&str> = l.split('\t').collect();
                Some((
                    (*f.first()?).to_string(),
                    (f.get(1)?.parse().ok()?, f.get(2)?.parse().ok()?),
                ))
            })
            .collect()
    };
    let (sa, sb) = (summary(a), summary(b));
    for (lang, &(secs_a, peak_a)) in &sa {
        let file = format!("{lang}.tsv");
        let (fa, fb) = (a.join(&file), b.join(&file));
        let Some(&(secs_b, peak_b)) = sb.get(lang) else {
            continue;
        };
        println!(
            "\n{lang}: time {secs_a:.0}s → {secs_b:.0}s (×{:.2}), peak {peak_a:.0} → {peak_b:.0} MB (×{:.2})",
            secs_b / secs_a.max(1.0),
            peak_b / peak_a.max(1.0)
        );
        if fa.exists() && fb.exists() {
            compare(&fa, &fb)?;
        }
    }
    Ok(())
}
