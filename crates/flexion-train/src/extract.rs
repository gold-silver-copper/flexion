//! Reading a dump into lexemes: for each Wiktionary entry of a configured part
//! of speech, its plain forms per cell in listing order, its inherent
//! features, and its training weight.

use crate::cells::Cells;
use crate::kaikki::{self, Entry};
use crate::spec::{FormSource, Language};
use flexion::Spelling;
use std::collections::BTreeMap;
use std::path::Path;

/// One Wiktionary entry, reduced to what the learner needs.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Lexeme {
    /// The headword, case-folded when the language folds case.
    pub lemma: String,
    /// The headword's letters as the runtime reads them (no stress marks).
    pub base: String,
    /// The lookup key (`base` folded).
    pub key: String,
    /// The lemma's stressed vowel, from its stressed citation form.
    pub lemma_stress: Option<usize>,
    /// Plain attested forms per cell, in listing order. The lemma cell holds
    /// the (stressed) lemma itself when the table doesn't list it.
    pub cells: Vec<Vec<String>>,
    /// One value index per inherent feature (`u8::MAX` if unknown).
    pub inherent: Vec<u8>,
    /// 1 for a standard entry, less for an entry whose every sense is a
    /// register variant (slang, dialectal…).
    pub weight: f64,
    /// Position in the dump, which orders homographs the way Wiktionary does.
    pub order: usize,
}

/// Every lexeme of one part of speech.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PosData {
    pub lexemes: Vec<Lexeme>,
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
    let cells: Vec<Cells> = lang
        .parts
        .iter()
        .map(|p| Cells::new(lang, p))
        .collect::<Result<_, _>>()?;
    let mut out: Vec<PosData> = lang
        .parts
        .iter()
        .map(|_| PosData {
            lexemes: Vec::new(),
        })
        .collect();
    let mut order = 0usize;
    let stats = kaikki::for_each_entry(dump, |entry| {
        order += 1;
        if entry.lang_code != lang.wiktionary {
            return;
        }
        for (pi, part) in lang.parts.iter().enumerate() {
            if !part.wiktionary.contains(&entry.pos.as_str()) {
                continue;
            }
            if !part.templates.is_empty() {
                let template = entry
                    .forms
                    .iter()
                    .find(|f| f.tags.iter().any(|t| t == "inflection-template"));
                if !template.is_some_and(|t| part.templates.contains(&t.form.as_str())) {
                    continue;
                }
            }
            let (Some(c), Some(sink)) = (cells.get(pi), out.get_mut(pi)) else {
                continue;
            };
            if let Some(lex) = lexeme(lang, spelling, pi, c, &entry, order) {
                sink.lexemes.push(lex);
            }
        }
    })?;
    eprintln!(
        "read {} lines ({} unparsable); lexemes: {}",
        stats.lines,
        stats.failures,
        lang.parts
            .iter()
            .zip(&out)
            .map(|(p, d)| format!("{} {}", p.name, d.lexemes.len()))
            .collect::<Vec<_>>()
            .join(", ")
    );
    Ok(out)
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
    if !lang.hooks.lemma_ok(part.name, &lemma) {
        return None;
    }
    let policy = &lang.tags;
    let tables = part.forms == FormSource::Tables && entry.forms.iter().any(|f| f.source.is_some());
    let mut forms: Vec<Vec<String>> = vec![Vec::new(); cells.total];
    let mut inherent_tags: Vec<String> = lang.hooks.inherent_tags(entry);
    for f in &entry.forms {
        let has = |t: &str| f.tags.iter().any(|x| x == t);
        if f.source.is_none() && has("canonical") {
            inherent_tags.extend(f.tags.iter().cloned());
        }
        if tables && f.source.is_none() {
            continue;
        }
        if policy.meta.iter().any(|t| has(t))
            || policy.sentinels.contains(&f.form.trim())
            || policy.exclude.iter().any(|t| has(t))
            || policy.soft.iter().any(|t| has(t))
        {
            continue;
        }
        let form = fold(f.form.trim());
        if !lang.hooks.form_ok(part.name, &lemma, &form) {
            continue;
        }
        for cell in cells.match_tags(&f.tags) {
            if let Some(slot) = forms.get_mut(cell)
                && !slot.contains(&form)
            {
                slot.push(form.clone());
            }
        }
    }
    let lemma_cell = cells.find(part.lemma.0, &part.lemma.1)?;
    if !forms
        .iter()
        .enumerate()
        .any(|(i, f)| i != lemma_cell && !f.is_empty())
    {
        return None;
    }
    // The lemma's own stressed form: the table's lemma cell, else the
    // canonical headword form, else the headword.
    let canonical = entry
        .forms
        .iter()
        .find(|f| f.source.is_none() && f.tags.iter().any(|t| t == "canonical"))
        .map(|f| fold(f.form.trim()));
    let read = spelling.read(&lemma);
    let stressed = forms
        .get(lemma_cell)
        .and_then(|f| f.first().cloned())
        .filter(|f| spelling.read(f).base == read.base)
        .or(canonical.filter(|c| spelling.read(c).base == read.base));
    let lemma_stress = stressed.as_ref().and_then(|s| spelling.read(s).stress);
    if let Some(slot) = forms.get_mut(lemma_cell)
        && slot.is_empty()
    {
        slot.push(stressed.unwrap_or_else(|| lemma.clone()));
    }

    for f in &entry.forms {
        if f.source.is_some() && cells.match_tags(&f.tags).contains(&lemma_cell) {
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
        lemma,
        lemma_stress,
        cells: forms,
        inherent,
        weight: if soft_entry {
            policy.soft_entry_weight
        } else {
            1.0
        },
        order,
    })
}
