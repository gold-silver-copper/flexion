//! Reading a dump into lexemes: for each Wiktionary entry of a configured part
//! of speech, its plain forms per cell in listing order, its inherent
//! features, and its training weight.

use crate::cells::Cells;
use crate::kaikki::{self, Entry};
use crate::spec::{FormSource, Language};
use flexion::Spelling;
use std::collections::BTreeMap;
use std::path::Path;

/// The longest lemma or form read, in bytes (see the format's edit record).
const MAX_WORD: usize = 255;

/// One Wiktionary entry, reduced to what the learner needs.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Lexeme {
    /// The headword, case-folded when the language folds case.
    pub lemma: String,
    /// The headword's letters as the runtime reads them (no stress marks).
    pub base: String,
    /// The citation form's letters: the headword written in full (Latin
    /// macrons, Russian ё). Every cell but the citation cell is an edit of
    /// these; the citation cell is the edit that restores them from `base`.
    pub full: String,
    /// The lookup key (`base` folded).
    pub key: String,
    /// The lemma's stressed vowel, from its stressed citation form.
    pub lemma_stress: Option<usize>,
    /// The kind of the lemma's stress mark.
    #[serde(default)]
    pub lemma_kind: u8,
    /// Plain attested forms per cell, in listing order, then the citation
    /// form as one extra, last cell.
    pub cells: Vec<Vec<String>>,
    /// One value index per inherent feature (`u8::MAX` if unknown).
    pub inherent: Vec<u8>,
    /// 1 for a standard entry, less for an entry whose every sense is a
    /// register variant (slang, dialectal…).
    pub weight: f64,
    /// Position in the dump, which orders homographs the way Wiktionary does.
    pub order: usize,
    /// Where the entry comes from: 0 for the main dump, `i + 1` for the
    /// language's edition `i`.
    #[serde(default)]
    pub source: u8,
}

/// Every lexeme of one part of speech.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PosData {
    pub lexemes: Vec<Lexeme>,
    /// What each edition added, in the order the language lists them.
    #[serde(default)]
    pub editions: Vec<EditionStats>,
}

/// What one edition gave a part of speech, and how well it agrees with the
/// main dump on the words both have.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct EditionStats {
    pub code: String,
    /// Entries read.
    pub entries: usize,
    /// Words (lookup keys) the main dump doesn't have.
    pub new_words: usize,
    /// Words both have.
    pub shared_words: usize,
    /// Entries kept for training.
    pub kept: usize,
    /// On shared words, cells both fill; of those, how many have the
    /// edition's first form among the main dump's forms, exactly and in
    /// letters (stress and length marks aside).
    pub compared: usize,
    pub same: usize,
    pub same_letters: usize,
}

impl PosData {
    /// The main dump's entries only: for numbers comparable with a build
    /// that reads no edition.
    pub fn main_only(&self) -> PosData {
        PosData {
            lexemes: self
                .lexemes
                .iter()
                .filter(|l| l.source == 0)
                .cloned()
                .collect(),
            editions: Vec::new(),
        }
    }
}

/// Lexemes grouped by lookup key: what a caller asking for that word means.
pub struct Word<'a> {
    pub key: &'a str,
    pub lexemes: Vec<&'a Lexeme>,
}

impl Word<'_> {
    /// The primary attested form of a cell: the first-listed form of the
    /// first entry (in dump order) that attests the cell.
    pub fn primary(&self, cell: usize) -> Option<&str> {
        self.lexemes
            .iter()
            .find_map(|l| l.cells.get(cell).and_then(|c| c.first()))
            .map(String::as_str)
    }

    /// Every plain attested form of a cell, across entries.
    pub fn attested(&self, cell: usize) -> Vec<&str> {
        let mut out: Vec<&str> = Vec::new();
        for l in &self.lexemes {
            for f in l.cells.get(cell).into_iter().flatten() {
                if !out.contains(&f.as_str()) {
                    out.push(f);
                }
            }
        }
        out
    }

    pub fn lemma(&self) -> &str {
        self.lexemes.first().map_or("", |l| l.lemma.as_str())
    }
}

pub fn words(data: &PosData) -> Vec<Word<'_>> {
    let mut by_key: BTreeMap<&str, Vec<&Lexeme>> = BTreeMap::new();
    for l in &data.lexemes {
        by_key.entry(&l.key).or_default().push(l);
    }
    by_key
        .into_iter()
        .map(|(key, mut lexemes)| {
            lexemes.sort_by_key(|l| l.order);
            Word { key, lexemes }
        })
        .collect()
}

/// Read every configured part of speech from a dump in one pass.
pub fn extract(
    lang: &Language,
    spelling: &Spelling<'_>,
    dump: &Path,
) -> Result<Vec<PosData>, String> {
    let cells = all_cells(lang)?;
    let mut out: Vec<PosData> = lang
        .parts
        .iter()
        .map(|_| PosData {
            lexemes: Vec::new(),
            editions: Vec::new(),
        })
        .collect();
    let mut order = 0usize;
    let stats = kaikki::for_each_entry(dump, |entry| {
        order += 1;
        read_entry(lang, spelling, &cells, &mut out, &entry, order, None);
    })?;
    eprintln!(
        "read {} lines ({} unparsable); lexemes: {}",
        stats.lines,
        stats.failures,
        summary(lang, &out)
    );
    Ok(out)
}

/// Read edition `index` of the language (see [`Language::editions`]) from its
/// dump into `data`, which holds the main dump's lexemes, merging as the
/// edition says.
pub fn add_edition(
    lang: &Language,
    spelling: &Spelling<'_>,
    data: &mut [PosData],
    index: usize,
    dump: &Path,
) -> Result<(), String> {
    let edition = lang
        .editions
        .get(index)
        .ok_or(format!("{}: no edition {index}", lang.name))?;
    let source = u8::try_from(index + 1).map_err(|_| "too many editions")?;
    let cells = all_cells(lang)?;
    let mut read: Vec<PosData> = lang
        .parts
        .iter()
        .map(|_| PosData {
            lexemes: Vec::new(),
            editions: Vec::new(),
        })
        .collect();
    // Entries order after every main-dump entry, so a word's main entries
    // stay its first.
    let mut order = data
        .iter()
        .flat_map(|d| d.lexemes.iter().map(|l| l.order))
        .max()
        .unwrap_or(0);
    let stats = kaikki::for_each_entry(dump, |entry| {
        order += 1;
        read_entry(
            lang,
            spelling,
            &cells,
            &mut read,
            &entry,
            order,
            Some((source, edition.code)),
        );
    })?;
    for ((d, r), c) in data.iter_mut().zip(read).zip(&cells) {
        let mut s = EditionStats {
            code: edition.code.to_string(),
            entries: r.lexemes.len(),
            ..EditionStats::default()
        };
        let mut main: BTreeMap<&str, Vec<&Lexeme>> = BTreeMap::new();
        for l in d.lexemes.iter().filter(|l| l.source == 0) {
            main.entry(&l.key).or_default().push(l);
        }
        let mut new_keys: std::collections::BTreeSet<&str> = std::collections::BTreeSet::new();
        let mut shared_keys: std::collections::BTreeSet<&str> = std::collections::BTreeSet::new();
        let mut kept = Vec::new();
        for l in &r.lexemes {
            let Some(m) = main.get(l.key.as_str()) else {
                new_keys.insert(&l.key);
                kept.push(l.clone());
                continue;
            };
            shared_keys.insert(&l.key);
            let letters = |f: &str| spelling.unmarked(&spelling.read(f).base);
            for (cell, forms) in l.cells.iter().enumerate().take(c.total) {
                let Some(first) = forms.first() else {
                    continue;
                };
                let theirs: Vec<&String> = m
                    .iter()
                    .flat_map(|x| x.cells.get(cell).into_iter().flatten())
                    .collect();
                if theirs.is_empty() {
                    continue;
                }
                s.compared += 1;
                if theirs.contains(&first) {
                    s.same += 1;
                }
                let mine = letters(first);
                if theirs.iter().any(|t| letters(t) == mine) {
                    s.same_letters += 1;
                }
            }
            if edition.merge == crate::spec::Merge::All {
                kept.push(l.clone());
            }
        }
        s.new_words = new_keys.len();
        s.shared_words = shared_keys.len();
        s.kept = kept.len();
        d.lexemes.extend(kept);
        d.editions.push(s);
    }
    eprintln!(
        "{}: read {} lines ({} unparsable); lexemes now: {}",
        edition.code,
        stats.lines,
        stats.failures,
        summary(lang, data)
    );
    for (p, d) in lang.parts.iter().zip(data.iter()) {
        if let Some(s) = d.editions.last() {
            eprintln!(
                "  {}: {} entries, {} new words, {} shared ({} cells compared: {:.1}% same, {:.1}% same letters), {} kept",
                p.name,
                s.entries,
                s.new_words,
                s.shared_words,
                s.compared,
                100.0 * s.same as f64 / s.compared.max(1) as f64,
                100.0 * s.same_letters as f64 / s.compared.max(1) as f64,
                s.kept
            );
        }
    }
    Ok(())
}

fn all_cells(lang: &Language) -> Result<Vec<Cells<'_>>, String> {
    lang.parts.iter().map(|p| Cells::new(lang, p)).collect()
}

fn summary(lang: &Language, data: &[PosData]) -> String {
    lang.parts
        .iter()
        .zip(data)
        .map(|(p, d)| format!("{} {}", p.name, d.lexemes.len()))
        .collect::<Vec<_>>()
        .join(", ")
}

/// Read one entry into every part of speech it belongs to. `edition` is
/// `(source, code)` for an entry of another edition.
fn read_entry(
    lang: &Language,
    spelling: &Spelling<'_>,
    cells: &[Cells],
    out: &mut [PosData],
    entry: &Entry,
    order: usize,
    edition: Option<(u8, &str)>,
) {
    if entry.lang_code != lang.wiktionary {
        return;
    }
    for (pi, part) in lang.parts.iter().enumerate() {
        if !part.wiktionary.contains(&entry.pos.as_str()) {
            continue;
        }
        let mut repaired = entry.clone();
        match edition {
            Some((_, code)) => {
                if !lang.hooks.adapt(code, part.name, &mut repaired) {
                    continue;
                }
            }
            None => {
                if !part.templates.is_empty() {
                    let template = entry
                        .forms
                        .iter()
                        .find(|f| f.tags.iter().any(|t| t == "inflection-template"));
                    // A template name ending in `*` matches by prefix.
                    let matches = |t: &str| {
                        part.templates.iter().any(|p| match p.strip_suffix('*') {
                            Some(prefix) => t.starts_with(prefix),
                            None => t == *p,
                        })
                    };
                    if !template.is_some_and(|t| matches(&t.form)) {
                        continue;
                    }
                }
                lang.hooks.repair(part.name, &mut repaired);
            }
        }
        let (Some(c), Some(sink)) = (cells.get(pi), out.get_mut(pi)) else {
            continue;
        };
        if let Some(mut lex) = lexeme(lang, spelling, pi, c, &repaired, order) {
            lex.source = edition.map_or(0, |(s, _)| s);
            sink.lexemes.push(lex);
        }
    }
}

fn lexeme(
    lang: &Language,
    spelling: &Spelling<'_>,
    pi: usize,
    cells: &Cells,
    entry: &Entry,
    order: usize,
) -> Option<Lexeme> {
    let part = lang.parts.get(pi)?;
    let fold = |s: &str| {
        if lang.spelling.fold_case {
            s.to_lowercase()
        } else {
            s.to_string()
        }
    };
    let lemma = fold(&entry.word);
    // The model writes an edit's strings' lengths in a byte, and no word
    // comes near 255 bytes.
    if lemma.len() > MAX_WORD || !lang.hooks.lemma_ok(part.name, &lemma) {
        return None;
    }
    let policy = &lang.tags;
    let tables = part.forms == FormSource::Tables && entry.forms.iter().any(|f| f.source.is_some());
    // One slot per cell, plus the citation form (the lemma written in full)
    // as a last, hidden cell.
    let citation_cell = cells.total;
    let mut forms: Vec<Vec<String>> = vec![Vec::new(); cells.total + 1];
    let mut inherent_tags: Vec<String> = lang.hooks.inherent_tags(entry);
    for f in &entry.forms {
        let has = |t: &str| f.tags.iter().any(|x| x == t);
        if f.source.is_none() && has("canonical") {
            inherent_tags.extend(f.tags.iter().cloned());
        }
        if policy.meta.iter().any(|t| has(t))
            || policy.sentinels.contains(&f.form.trim())
            || policy.exclude.iter().any(|t| has(t))
            || policy.soft.iter().any(|t| has(t))
        {
            continue;
        }
        let form = fold(f.form.trim());
        if form.len() > MAX_WORD || !lang.hooks.form_ok(part.name, &lemma, &form) {
            continue;
        }
        // With a table, table forms fill the table's groups and head-line
        // forms only the groups that come from the head line.
        let head = f.source.is_none();
        let wanted = |g: &crate::spec::Group| !tables || g.from_head == head;
        for cell in cells.match_tags_where(&f.tags, &wanted) {
            if let Some(slot) = forms.get_mut(cell)
                && !slot.contains(&form)
            {
                slot.push(form.clone());
            }
        }
    }
    let lemma_cell = match &part.lemma {
        Some((g, v)) => Some(cells.find(g, v)?),
        None => None,
    };
    if !forms
        .iter()
        .enumerate()
        .any(|(i, f)| Some(i) != lemma_cell && i != citation_cell && !f.is_empty())
    {
        return None;
    }
    // The citation form: the lemma as the dictionary writes it in full (with
    // stress, macrons, ё): the table's lemma cell, else the canonical
    // headword form, else the headword.
    let read = spelling.read(&lemma);
    let same_word = |f: &String| spelling.read(f).key == read.key;
    let canonical = entry
        .forms
        .iter()
        .find(|f| f.source.is_none() && f.tags.iter().any(|t| t == "canonical"))
        .map(|f| fold(f.form.trim()))
        .filter(same_word);
    let written = lemma_cell
        .and_then(|c| forms.get(c))
        .and_then(|f| f.first().cloned())
        .filter(same_word)
        .or(canonical)
        .unwrap_or_else(|| lemma.clone());
    let full = spelling.read(&written);
    let lemma_stress = full.stress;
    let lemma_kind = full.kind;
    let full_base = full.base.to_string();
    // Without a table, the lemma cell is the headword itself (English nouns
    // list only their plural).
    if !tables
        && let Some(slot) = lemma_cell.and_then(|c| forms.get_mut(c))
        && slot.is_empty()
    {
        slot.push(written.clone());
    }
    if let Some(slot) = forms.get_mut(citation_cell) {
        slot.push(written);
    }

    for f in &entry.forms {
        if f.source.is_some() && lemma_cell.is_some_and(|c| cells.match_tags(&f.tags).contains(&c))
        {
            inherent_tags.extend(f.tags.iter().cloned());
        }
    }
    let inherent = part
        .inherent
        .iter()
        .map(|name| {
            lang.feature(name)
                .and_then(|feat| {
                    feat.values.iter().position(|v| {
                        v.all.iter().all(|t| inherent_tags.iter().any(|x| x == t))
                            && !v.none.iter().any(|t| inherent_tags.iter().any(|x| x == t))
                    })
                })
                .and_then(|i| u8::try_from(i).ok())
                .unwrap_or(u8::MAX)
        })
        .collect();

    let soft_entry = !entry.senses.is_empty()
        && entry.senses.iter().all(|s| {
            s.tags.iter().any(|t| {
                policy.soft_sense.contains(&t.as_str())
                    || policy.soft.contains(&t.as_str())
                    || policy.exclude.contains(&t.as_str())
            })
        });
    Some(Lexeme {
        key: read.key.to_string(),
        base: read.base.to_string(),
        full: full_base,
        lemma,
        lemma_stress,
        lemma_kind,
        cells: forms,
        inherent,
        weight: if soft_entry {
            policy.soft_entry_weight
        } else {
            1.0
        },
        order,
        source: 0,
    })
}
