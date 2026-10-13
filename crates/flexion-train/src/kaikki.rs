//! The Kaikki (wiktextract) JSONL schema, as far as the trainer reads it, and
//! a streaming reader that never loads a dump into memory.

use serde::Deserialize;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;

#[derive(Debug, Clone, Deserialize)]
pub struct Entry {
    pub word: String,
    #[serde(default)]
    pub pos: String,
    #[serde(default)]
    pub lang_code: String,
    #[serde(default)]
    pub forms: Vec<Form>,
    #[serde(default)]
    pub senses: Vec<Sense>,
    #[serde(default)]
    pub head_templates: Vec<HeadTemplate>,
    /// Tags of the word itself (other editions put gender and aspect here).
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub categories: Vec<Category>,
    /// Syllable breaks of the headword (`пры́`, `гать`), which some editions
    /// write with stress.
    #[serde(default)]
    pub hyphenations: Vec<Hyphenation>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Hyphenation {
    #[serde(default)]
    pub parts: Vec<String>,
}

/// A category the entry is in: a plain name, or an object with one.
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum Category {
    Name(String),
    Named { name: String },
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Form {
    pub form: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub source: Option<String>,
    /// Labels other editions leave untranslated (`Nominativ`).
    #[serde(default)]
    pub raw_tags: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Sense {
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HeadTemplate {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub expansion: String,
}

/// Stream every entry of a dump (`.jsonl` or `.jsonl.gz`). Lines that fail
/// to parse are counted; more than 0.1% of them is an error, since it means
/// the schema changed and words would silently vanish.
pub fn for_each_entry(path: &Path, mut f: impl FnMut(Entry)) -> Result<Stats, String> {
    let file = File::open(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let reader: Box<dyn Read> = if path.extension().is_some_and(|e| e == "gz") {
        Box::new(flate2::read::GzDecoder::new(file))
    } else {
        Box::new(file)
    };
    let mut stats = Stats::default();
    for line in BufReader::with_capacity(1 << 20, reader).lines() {
        let line = line.map_err(|e| format!("{}: {e}", path.display()))?;
        stats.lines += 1;
        match serde_json::from_str::<Entry>(&line) {
            Ok(e) => f(e),
            Err(_) => stats.failures += 1,
        }
    }
    if stats.failures * 1000 > stats.lines {
        return Err(format!(
            "{}: {} of {} lines failed to parse; the Kaikki schema probably changed",
            path.display(),
            stats.failures,
            stats.lines
        ));
    }
    Ok(stats)
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Stats {
    pub lines: usize,
    pub failures: usize,
}
