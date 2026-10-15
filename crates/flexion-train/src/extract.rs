//! Reading a dump into lexemes: for each Wiktionary entry of a configured part
//! of speech, its plain forms per cell in listing order, its inherent
//! features, and its training weight.

use crate::cells::Cells;
use crate::kaikki::{self, Entry};
use crate::spec::{FormSource, Language, PartOfSpeech};
use flexion::Orthography;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use unicode_normalization::UnicodeNormalization;

/// The longest lemma or form read, in bytes (see the format's edit record).
const MAX_WORD: usize = 255;

/// One Wiktionary entry, reduced to what the learner needs.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Lexeme {
    /// The headword, case-folded when the language folds case.
    pub lemma: String,
    /// The headword written in full, as the dictionary writes it (with stress,
    /// macrons, ё): the table's lemma cell, else the headword.
    pub full: String,
    /// The lookup key: `full`'s letters, without stress marks.
    pub key: String,
    /// Attested forms per cell, as written, in listing order.
    pub cells: Vec<Vec<String>>,
    /// One value index per inherent feature (`u8::MAX` if unknown).
    pub inherent: Vec<u8>,
    /// Position in the dump, which orders homographs the way Wiktionary does.
    pub order: usize,
    /// How many senses the entry has: common words have many.
    #[serde(default)]
    pub senses: u16,
    /// Where the entry comes from: 0 for the main dump, `i + 1` for the
    /// language's edition `i`.
    #[serde(default)]
    pub source: u8,
    /// A sense of it is vulgar or offensive: not a word to name a paradigm
    /// after.
    #[serde(default)]
    pub vulgar: bool,
    /// What the entry is: a word of its own, or another word's form or
    /// spelling (for scoring them apart).
    #[serde(default)]
    pub of: Of,
    /// The dictionary editors' own analysis of the word. **For scoring
    /// only**: the learner never reads it (the grammar must find classes,
    /// stems and features itself); `xtask` compares what was learned with it.
    #[serde(default, skip_serializing_if = "Editors::is_empty")]
    pub editors: Editors,
}

/// What the dictionary's editors wrote about a word's inflection: its
/// head-line and inflection templates with their arguments (Latin `la-conj`
/// `3.sigm tāgō tetig tāct`, Russian `ru-conj` `6a`, Greek `grc-conj` per
/// tense stem) and the tags of its senses (`deponent`). Carried for scoring
/// only; see [`Lexeme::editors`].
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Editors {
    /// `(template, arguments)`, in the entry's order: head-line templates,
    /// then inflection templates. Arguments are `(name, value)`, empty and
    /// long (over [`EDITOR_ARGUMENT`] bytes: notes) ones left out.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub templates: Vec<(String, Vec<(String, String)>)>,
    /// Every tag of every sense, sorted, once each.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
}

/// The longest template argument kept in [`Editors`], in bytes.
pub const EDITOR_ARGUMENT: usize = 48;

impl Editors {
    pub fn is_empty(&self) -> bool {
        self.templates.is_empty() && self.tags.is_empty()
    }

    /// Read from a dump entry.
    pub fn of(entry: &Entry) -> Editors {
        let args = |a: &std::collections::BTreeMap<String, serde_json::Value>| {
            a.iter()
                .filter_map(|(k, v)| {
                    let v = match v {
                        serde_json::Value::String(s) => s.clone(),
                        serde_json::Value::Number(n) => n.to_string(),
                        _ => return None,
                    };
                    (!v.is_empty() && v.len() <= EDITOR_ARGUMENT).then(|| (k.clone(), v))
                })
                .collect::<Vec<_>>()
        };
        let templates = entry
            .head_templates
            .iter()
            .filter(|t| t.name != "head")
            .map(|t| (t.name.clone(), args(&t.args)))
            .chain(
                entry
                    .inflection_templates
                    .iter()
                    .map(|t| (t.name.clone(), args(&t.args))),
            )
            .collect();
        let tags: BTreeSet<String> = entry
            .senses
            .iter()
            .flat_map(|s| s.tags.iter().cloned())
            .collect();
        Editors {
            templates,
            tags: tags.into_iter().collect(),
        }
    }

    /// The arguments of the templates named `name`, in order.
    pub fn template<'e>(&'e self, name: &'e str) -> impl Iterator<Item = &'e [(String, String)]> {
        self.templates
            .iter()
            .filter(move |(n, _)| n == name)
            .map(|(_, a)| a.as_slice())
    }

    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }
}

/// Every lexeme of one part of speech.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct PosData {
    pub lexemes: Vec<Lexeme>,
}

/// Read every configured part of speech from a dump in one pass.
pub fn extract(
    lang: &Language,
    spelling: &Orthography<'_>,
    dump: &Path,
) -> Result<Vec<PosData>, String> {
    let (mut out, stats) = read_dump(lang, spelling, dump, 0, None)?;
    eprintln!(
        "read {} lines ({} unparsable); lexemes: {}",
        stats.lines,
        stats.failures,
        summary(lang, &out)
    );
    foreign(lang, spelling, &mut out)?;
    Ok(out)
}

/// An entry whose table is another word's is rare noise; a convention of
/// the dictionary's (Greek verbs cited contracted, `φοβῶ` for `φοβέω`'s
/// table) is common. Below this share of a part of speech's entries, its
/// foreign tables are dropped (see [`foreign`]).
const FOREIGN: f64 = 0.02;

/// Entries whose table is another word's: none of the forms in its
/// lemma's cell has the headword's letters, whatever marks either writes
/// (Polish `zaklepywać` with `zaklepywywać`'s table; not Latin `homo` with
/// `homō̆`, its length unsure). Where they are fewer than [`FOREIGN`] of a part
/// of speech's entries they are noise and go; else a convention, and they
/// stay. Each one is printed either way.
pub fn foreign(
    lang: &Language,
    spelling: &Orthography<'_>,
    data: &mut [PosData],
) -> Result<(), String> {
    let key = |s: &str| -> String {
        use unicode_normalization::{UnicodeNormalization, char::is_combining_mark};
        let key = spelling.key(&spelling.read(s).letters);
        key.nfd().filter(|&c| !is_combining_mark(c)).collect()
    };
    for (part, d) in lang.parts.iter().zip(data.iter_mut()) {
        let Some(lemma_cell) = crate::shape(lang, part)?.lemma_cell else {
            continue;
        };
        let foreign = |l: &Lexeme| {
            l.cells.get(lemma_cell).is_some_and(|forms| {
                !forms.is_empty() && !forms.iter().any(|f| key(f) == key(&l.lemma))
            })
        };
        let flagged: Vec<String> = d
            .lexemes
            .iter()
            .filter(|l| foreign(l))
            .map(|l| {
                let table = l
                    .cells
                    .get(lemma_cell)
                    .and_then(|f| f.first())
                    .map_or("", String::as_str);
                format!("{} ({table})", l.lemma)
            })
            .collect();
        if flagged.is_empty() {
            continue;
        }
        #[allow(clippy::cast_precision_loss)]
        let share = flagged.len() as f64 / d.lexemes.len().max(1) as f64;
        let dropped = share < FOREIGN;
        eprintln!(
            "  {}: {} of {} entries have another word's table ({:.2}%): {}; {}",
            part.name,
            flagged.len(),
            d.lexemes.len(),
            100.0 * share,
            if dropped {
                "dropped"
            } else {
                "kept, a convention"
            },
            flagged.join(", ")
        );
        if dropped {
            d.lexemes.retain(|l| !foreign(l));
        }
    }
    Ok(())
}

/// Read edition `index` of the language (see [`Language::editions`]) from its
/// dump into `data`, which holds the main dump's lexemes, merging as the
/// edition says.
pub fn add_edition(
    lang: &Language,
    spelling: &Orthography<'_>,
    data: &mut [PosData],
    index: usize,
    dump: &Path,
) -> Result<(), String> {
    let edition = lang
        .editions
        .get(index)
        .ok_or(format!("{}: no edition {index}", lang.name))?;
    let source = u8::try_from(index + 1).map_err(|_| "too many editions")?;
    // Entries order after every main-dump entry, so a word's main entries
    // stay its first.
    let order = data
        .iter()
        .flat_map(|d| d.lexemes.iter().map(|l| l.order))
        .max()
        .unwrap_or(0);
    let (read, stats) = read_dump(lang, spelling, dump, order, Some((source, edition.code)))?;
    // How the edition agrees with the main dump on the words both have:
    // of the cells both fill, how many have its first form among the main
    // dump's, exactly and in letters (stress and length marks aside).
    let mut report = Vec::new();
    for ((d, r), p) in data.iter_mut().zip(read).zip(&lang.parts) {
        let mut main: BTreeMap<&str, Vec<&Lexeme>> = BTreeMap::new();
        for l in d.lexemes.iter().filter(|l| l.source == 0) {
            main.entry(&l.key).or_default().push(l);
        }
        let (mut new_words, mut shared) = (BTreeSet::new(), BTreeSet::new());
        let (mut compared, mut same, mut same_letters) = (0, 0, 0);
        let mut kept = Vec::new();
        for l in &r.lexemes {
            let Some(m) = main.get(l.key.as_str()) else {
                new_words.insert(&l.key);
                kept.push(l.clone());
                continue;
            };
            shared.insert(&l.key);
            let letters = |f: &str| spelling.read(f).letters;
            for (cell, forms) in l.cells.iter().enumerate() {
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
                compared += 1;
                same += usize::from(theirs.contains(&first));
                let mine = letters(first);
                same_letters += usize::from(theirs.iter().any(|t| letters(t) == mine));
            }
            if edition.merge == crate::spec::Merge::All {
                kept.push(l.clone());
            }
        }
        let share = |n: usize| 100.0 * n as f64 / compared.max(1) as f64;
        report.push(format!(
            "  {}: {} entries, {} new words, {} shared ({compared} cells compared: {:.1}% same, {:.1}% same letters), {} kept",
            p.name,
            r.lexemes.len(),
            new_words.len(),
            shared.len(),
            share(same),
            share(same_letters),
            kept.len()
        ));
        d.lexemes.extend(kept);
    }
    eprintln!(
        "{}: read {} lines ({} unparsable); lexemes now: {}\n{}",
        edition.code,
        stats.lines,
        stats.failures,
        summary(lang, data),
        report.join("\n")
    );
    foreign(lang, spelling, data)
}

/// Read a dump's entries into every part of speech, numbered on from
/// `order`. `edition` is `(source, code)` for another edition's dump.
fn read_dump(
    lang: &Language,
    spelling: &Orthography<'_>,
    dump: &Path,
    mut order: usize,
    edition: Option<(u8, &str)>,
) -> Result<(Vec<PosData>, kaikki::Stats), String> {
    let cells = lang
        .parts
        .iter()
        .map(|p| Cells::new(lang, p))
        .collect::<Result<Vec<_>, _>>()?;
    let mut out = vec![PosData::default(); lang.parts.len()];
    let stats = kaikki::for_each_entry(dump, |entry| {
        order += 1;
        read_entry(lang, spelling, &cells, &mut out, &entry, order, edition);
    })?;
    Ok((out, stats))
}

fn summary(lang: &Language, data: &[PosData]) -> String {
    lang.parts
        .iter()
        .zip(data)
        .map(|(p, d)| format!("{} {}", p.name, d.lexemes.len()))
        .collect::<Vec<_>>()
        .join(", ")
}

/// Read one entry into every part of speech it belongs to.
fn read_entry(
    lang: &Language,
    spelling: &Orthography<'_>,
    cells: &[Cells],
    out: &mut [PosData],
    entry: &Entry,
    order: usize,
    edition: Option<(u8, &str)>,
) {
    if entry.lang_code != lang.wiktionary {
        return;
    }
    for ((part, c), sink) in lang.parts.iter().zip(cells).zip(out) {
        if !part.wiktionary.contains(&entry.pos.as_str()) {
            continue;
        }
        let mut repaired = entry.clone();
        let read = match edition {
            Some((_, code)) => lang.hooks.adapt(code, part.name, &mut repaired),
            None => {
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
                let read = part.templates.is_empty() || template.is_some_and(|t| matches(&t.form));
                if read {
                    // An entry may have more than one table (Polish: a Middle
                    // Polish one after the modern one): each table's forms
                    // come after its template's name; those of a template
                    // the part of speech doesn't read go.
                    if !part.templates.is_empty() {
                        let mut reading = true;
                        repaired.forms.retain(|f| {
                            if f.tags.iter().any(|t| t == "inflection-template") {
                                reading = matches(&f.form);
                            }
                            f.source.is_none() || reading
                        });
                    }
                    lang.hooks.repair(part.name, &mut repaired);
                }
                read
            }
        };
        if !read {
            continue;
        }
        if let Some(mut lex) = lexeme(lang, spelling, part, c, &repaired, order) {
            lex.source = edition.map_or(0, |(s, _)| s);
            sink.lexemes.push(lex);
        }
    }
}

fn lexeme(
    lang: &Language,
    spelling: &Orthography<'_>,
    part: &PartOfSpeech,
    cells: &Cells,
    entry: &Entry,
    order: usize,
) -> Option<Lexeme> {
    let fold = |s: &str| {
        let s = if lang.spelling.fold_case {
            s.to_lowercase()
        } else {
            s.to_string()
        };
        initial_marks_only(&s, lang.spelling.initial_marks, lang.spelling.vowels)
    };
    let lemma = fold(&entry.word);
    // The model writes an edit's strings' lengths in a byte, and no word
    // comes near 255 bytes.
    if lemma.len() > MAX_WORD || !lang.hooks.lemma_ok(part.name, &lemma) {
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
        for cell in cells.match_tags(&f.tags, &wanted) {
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
        .any(|(i, f)| Some(i) != lemma_cell && !f.is_empty())
    {
        return None;
    }
    // The citation form: the lemma as the dictionary writes it in full (with
    // stress, macrons, ё): the table's lemma cell, else the canonical
    // headword form, else the headword.
    let key_of = |f: &str| spelling.key(&spelling.read(f).letters);
    let lemma_key = key_of(&lemma);
    let same_word = |f: &String| key_of(f) == lemma_key;
    // An inflected form with an entry of its own (`γέγονε`, the perfect's
    // third person singular of `γίγνομαι`) repeats its lemma's table: a
    // sense calls it a form of another word, and its table has it in that
    // form's cell. It is no word of its own. Marks lookup ignores don't
    // count, in the letter or written apart (`ᾰ̓μείνων` is `ἀμείνων`).
    let s = &lang.spelling;
    let marks = [s.stress_marks, s.fold_marks, s.unmark_marks, s.strip_marks];
    let bare = |f: &str| {
        let plain: String = f
            .nfd()
            .filter(|c| !marks.iter().any(|m| m.contains(*c)))
            .collect();
        key_of(&plain)
    };
    let headword = bare(&lemma);
    let has_headword = |c: usize| {
        Some(c) != lemma_cell
            && forms
                .get(c)
                .is_some_and(|fs| fs.iter().any(|f| bare(f) == headword))
    };
    if form_of_cells(entry, cells).into_iter().any(has_headword) {
        return None;
    }
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
    let key = key_of(&written);
    // Without a table, the lemma cell is the headword itself (English nouns
    // list only their plural).
    if !tables
        && let Some(slot) = lemma_cell.and_then(|c| forms.get_mut(c))
        && slot.is_empty()
    {
        slot.push(written.clone());
    }

    for f in &entry.forms {
        if f.source.is_some()
            && lemma_cell.is_some_and(|c| cells.match_tags(&f.tags, &|_| true).contains(&c))
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

    Some(Lexeme {
        key,
        full: written,
        lemma,
        cells: forms,
        inherent,
        order,
        senses: u16::try_from(entry.senses.len()).unwrap_or(u16::MAX),
        source: 0,
        vulgar: entry.senses.iter().any(|s| {
            s.tags
                .iter()
                .any(|t| matches!(t.as_str(), "vulgar" | "offensive" | "derogatory" | "slur"))
        }),
        editors: Editors::of(entry),
        of: Of::of_entry(entry),
    })
}

/// The cells in which, by its senses, an entry is a form of another word:
/// none unless every sense says it is one.
/// `word` with the marks of `marks` that stand after a consonant of their
/// word (not on its first letter nor its first vowels) dropped.
fn initial_marks_only(word: &str, marks: &str, vowels: &str) -> String {
    if marks.is_empty() || !word.nfd().any(|c| marks.contains(c)) {
        return word.to_string();
    }
    let (mut letters, mut consonant) = (0, false);
    let mut out = String::with_capacity(word.len());
    for c in word.nfd() {
        if unicode_normalization::char::is_combining_mark(c) {
            if !(marks.contains(c) && consonant && letters > 1) {
                out.push(c);
            }
            continue;
        }
        if c.is_alphabetic() {
            letters += 1;
            consonant |= !vowels.contains(c);
        } else {
            (letters, consonant) = (0, false);
        }
        out.push(c);
    }
    out.nfc().collect()
}

/// What a dictionary entry is: a word of its own, or, by every sense's tags,
/// another word's form (`form-of`: a participle, a comparative) or another
/// word's spelling (`alt-of`).
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum Of {
    #[default]
    Own,
    Form,
    Spelling,
}

impl Of {
    pub fn of_entry(entry: &Entry) -> Of {
        let every = |tag: &str| {
            !entry.senses.is_empty() && entry.senses.iter().all(|s| s.tags.iter().any(|t| t == tag))
        };
        if every("form-of") {
            Of::Form
        } else if every("alt-of") {
            Of::Spelling
        } else {
            Of::Own
        }
    }
}

fn form_of_cells(entry: &Entry, cells: &Cells) -> Vec<usize> {
    let form_of = |s: &kaikki::Sense| s.tags.iter().any(|t| t == "form-of");
    if entry.senses.is_empty() || !entry.senses.iter().all(form_of) {
        return Vec::new();
    }
    entry
        .senses
        .iter()
        .flat_map(|s| cells.match_tags(&s.tags, &|_| true))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::initial_marks_only;

    #[test]
    fn a_breathing_after_a_consonant_of_the_word_is_dropped() {
        let breathings = "\u{313}\u{314}";
        let keep = |w: &str| initial_marks_only(w, breathings, "αεηιουω");
        assert_eq!(keep("ὑπἔκρυπτον"), "ὑπέκρυπτον");
        assert_eq!(keep("αἰτέω"), "αἰτέω");
        assert_eq!(keep("ῥήτωρ"), "ῥήτωρ");
        assert_eq!(keep("ὁ ἀνήρ"), "ὁ ἀνήρ");
    }
}
