//! Learn a language's inflection grammar from Wiktionary (Kaikki dumps), and
//! write it as a Rust crate.

use std::sync::atomic::{AtomicU64, Ordering};

/// Hash maps and sets with a fixed hasher of this crate's own ([`Fnv`]): no
/// toolchain changes it, and nothing the trainer writes may depend on the
/// order it fills them in (a test reseeds it and prints the same crate).
pub type HashMap<K, V> = std::collections::HashMap<K, V, Fixed>;
pub type HashSet<K> = std::collections::HashSet<K, Fixed>;
/// See [`HashMap`].
pub type Fixed = std::hash::BuildHasherDefault<Fnv>;

/// FNV-1a over the bytes hashed, from [`SEED`], its result mixed (Murmur3's
/// finalizer) so that every bit of it counts.
#[derive(Debug, Clone, Copy)]
pub struct Fnv(u64);

/// Where every [`Fnv`] starts: FNV's offset basis, unless a test reseeds it
/// (see [`reseed`]).
static SEED: AtomicU64 = AtomicU64::new(0xcbf2_9ce4_8422_2325);

impl Default for Fnv {
    fn default() -> Fnv {
        Fnv(SEED.load(Ordering::Relaxed))
    }
}

impl std::hash::Hasher for Fnv {
    fn finish(&self) -> u64 {
        let mut h = self.0;
        h ^= h >> 33;
        h = h.wrapping_mul(0xff51_afd7_ed55_8ccd);
        h ^= h >> 33;
        h = h.wrapping_mul(0xc4ce_b9fe_1a85_ec53);
        h ^ (h >> 33)
    }

    fn write(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.0 ^= u64::from(b);
            self.0 = self.0.wrapping_mul(0x0100_0000_01b3);
        }
    }
}

/// Start every [`Fnv`] made from now on from `seed`: for the test that the
/// trainer's output doesn't depend on the order its hash maps are filled
/// in. Nothing else calls it.
#[doc(hidden)]
pub fn reseed(seed: u64) {
    SEED.store(seed, Ordering::Relaxed);
}

/// The hash a cache is kept under, written to as text or bytes: [`Fnv`] from FNV's
/// own offset basis, whatever a test reseeded.
pub(crate) struct Hashing(Fnv);

impl Hashing {
    pub(crate) fn new() -> Hashing {
        Hashing(Fnv(0xcbf2_9ce4_8422_2325))
    }

    pub(crate) fn finish(&self) -> u64 {
        std::hash::Hasher::finish(&self.0)
    }
}

impl std::fmt::Write for Hashing {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        std::hash::Hasher::write(&mut self.0, s.as_bytes());
        Ok(())
    }
}

impl std::io::Write for Hashing {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        std::hash::Hasher::write(&mut self.0, bytes);
        Ok(bytes.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

pub mod baseline;
pub mod bits;
pub mod ceiling;
pub mod cells;
pub mod choose;
pub mod eval;
pub mod extract;
pub mod kaikki;
pub mod laws;
pub mod learn;
pub mod length;
pub mod marks;
pub mod measure;
pub mod par;
pub mod pipeline;
pub mod print;
pub mod relations;
pub mod report;
pub mod saved;
pub mod script;
pub mod skeleton;
pub mod spec;
pub mod stage;
pub mod stems;
pub mod trace;

use spec::{Language, PartOfSpeech};
use std::collections::BTreeSet;

/// The language's marks model (see [`marks`]), from the words of every
/// part of speech `keep` keeps (those trained on): the rules from their
/// lemmas, the roots from every form of theirs. A form-of page (Latin
/// `abditus`, a participle of `abdō`) votes on the roots with the word
/// whose table lists its headword, if `keep` keeps both: else a held-out
/// word's forms would teach the roots it is scored with. Without such a
/// word it doesn't vote.
pub fn marks(
    lang: &Language,
    orthography: &flexion::Orthography<'_>,
    data: &[extract::PosData],
    keep: impl Fn(&extract::Lexeme) -> bool,
    settings: choose::Settings,
) -> Result<Option<marks::LanguageMarks>, String> {
    use extract::Of;
    let read = |f: &str| orthography.read(f).letters;
    let lemmas: Vec<Option<usize>> = (lang.parts.iter())
        .map(|part| Ok(shape(lang, part)?.lemma_cell))
        .collect::<Result<_, String>>()?;
    // The words, numbered across the parts of speech.
    let words: Vec<(usize, &str, Option<usize>, &extract::Lexeme)> = (lang.parts.iter())
        .zip(data)
        .zip(&lemmas)
        .flat_map(|((part, d), &cell)| d.lexemes.iter().map(move |l| (part.name, cell, l)))
        .enumerate()
        .map(|(i, (part, cell, l))| (i, part, cell, l))
        .collect();
    // Each form-of page's word: the first whose table lists its headword
    // as a form (not as its own lemma: the noun `nexus` is no form of the
    // participle's).
    let mut owners: HashMap<String, Option<usize>> = (words.iter())
        .filter(|w| w.3.of == Of::Form)
        .map(|w| (orthography.key(&read(&w.3.full)), None))
        .collect();
    for &(i, _, cell, l) in words.iter().filter(|w| w.3.of != Of::Form) {
        let forms = (l.cells.iter().enumerate()).filter(|&(c, _)| Some(c) != cell);
        for f in forms.flat_map(|(_, fs)| fs) {
            if let Some(owner @ None) = owners.get_mut(&orthography.key(&read(f))) {
                *owner = Some(i);
            }
        }
    }
    let mut seen = HashSet::default();
    let mut spelled: Vec<(String, String, &str)> = Vec::new();
    let mut voters: Vec<marks::Voter<'_>> = Vec::new();
    for &(i, part, cell, l) in words.iter().filter(|w| keep(w.3)) {
        let letters = learn::spelled(orthography, cell, l);
        let key = orthography.key(&letters);
        let owner = match l.of {
            Of::Form => owners
                .get(&orthography.key(&read(&l.full)))
                .copied()
                .flatten()
                .filter(|&o| words.get(o).is_some_and(|w| keep(w.3))),
            _ => Some(i),
        };
        if let Some(owner) = owner {
            let forms = l.cells.iter().flatten().map(|f| read(f));
            let spellings: BTreeSet<String> =
                std::iter::once(letters.clone()).chain(forms).collect();
            voters.push(marks::Voter {
                owner,
                part,
                key: key.clone(),
                lemma: letters.clone(),
                spellings: spellings.into_iter().collect(),
            });
        }
        let word = (key, letters, part);
        if seen.insert(word.clone()) {
            spelled.push(word);
        }
    }
    let spelled: Vec<(&str, &str, &str)> = (spelled.iter())
        .map(|(k, l, p)| (k.as_str(), l.as_str(), *p))
        .collect();
    Ok(marks::learn(orthography, &spelled, &voters, settings))
}

/// The words a part of speech's grammar learns from, those a `keep` keeps:
/// its own, then those of the part it learns from too (see
/// [`spec::PartOfSpeech::learns_from`]) but those it has itself.
/// [`learn_part`] learns from nothing else, so every grammar of a part
/// learns from the same words the shipped one does.
#[derive(Clone)]
pub struct Words(Vec<extract::Lexeme>);

impl Words {
    pub fn of(
        lang: &Language,
        part: &PartOfSpeech,
        data: &[extract::PosData],
        keep: impl Fn(&extract::Lexeme) -> bool,
    ) -> Words {
        let lexemes = |name: &str| {
            (lang.parts.iter().zip(data))
                .find(|(p, _)| p.name == name)
                .map_or(&[][..], |(_, d)| d.lexemes.as_slice())
        };
        let own = lexemes(part.name);
        let keys: HashSet<&str> = own.iter().map(|l| l.key.as_str()).collect();
        let taught = part.learns_from.map_or(&[][..], lexemes);
        Words(
            (own.iter().filter(|l| keep(l)))
                .chain(
                    taught
                        .iter()
                        .filter(|l| keep(l) && !keys.contains(l.key.as_str())),
                )
                .cloned()
                .collect(),
        )
    }

    /// Those of these words `keep` keeps.
    #[must_use]
    pub fn only(&self, keep: impl Fn(&extract::Lexeme) -> bool) -> Words {
        Words(self.0.iter().filter(|l| keep(l)).cloned().collect())
    }

    /// Change each word (cells taken out of its table, to learn without
    /// them).
    pub fn edit(&mut self, edit: impl FnMut(&mut extract::Lexeme)) {
        self.0.iter_mut().for_each(edit);
    }
}

impl std::ops::Deref for Words {
    type Target = [extract::Lexeme];

    fn deref(&self) -> &[extract::Lexeme] {
        &self.0
    }
}

/// A part of speech's grammar (its `shape`), learned from `words` with the
/// language's marks model, if it has one: the one way every grammar is
/// learned.
pub fn learn_part<'s>(
    orthography: flexion::Orthography<'s>,
    shape: &learn::Shape,
    words: &Words,
    marks: Option<&marks::LanguageMarks>,
    settings: choose::Settings,
) -> learn::Learned<'s> {
    let shape = learn::Shape {
        marks: marks.map(|m| m.of(&shape.part)),
        ..shape.clone()
    };
    learn::learn(orthography, &shape, words, settings)
}

/// The cells of a part of speech's principal parts after its lemma.
pub fn parts(lang: &Language, part: &PartOfSpeech) -> Result<Vec<usize>, String> {
    let cells = cells::Cells::new(lang, part)?;
    part.citation
        .iter()
        .map(|(g, v)| {
            cells
                .find(g, v)
                .ok_or(format!("{}: no principal part cell {g} {v:?}", part.name))
        })
        .collect()
}

/// The shape of a part of speech's cells, from its config.
pub fn shape(lang: &Language, part: &PartOfSpeech) -> Result<learn::Shape, String> {
    let cells = cells::Cells::new(lang, part)?;
    let lemma_cell = part
        .lemma
        .as_ref()
        .map(|(g, v)| {
            let cell = cells.find(g, v);
            cell.ok_or(format!("{}: no lemma cell {g} {v:?}", part.name))
        })
        .transpose()?;
    // Each cell's group and feature values, on dimensions shared by name.
    let mut features: Vec<&str> = Vec::new();
    for f in cells.groups.iter().flat_map(|g| &g.features) {
        if !features.contains(&f.name) {
            features.push(f.name);
        }
    }
    let mut coordinates = vec![Vec::new(); cells.total];
    for (group, g) in (0u16..).zip(&cells.groups) {
        for cell in g.offset..g.offset + g.len() {
            let values = g.values(cell);
            let value = |name: &str| {
                let v = g.features.iter().zip(&values).find(|(f, _)| f.name == name);
                v.map_or(u16::MAX, |(_, &v)| u16::try_from(v).unwrap_or(u16::MAX))
            };
            let at = std::iter::once(group).chain(features.iter().map(|n| value(n)));
            if let Some(slot) = coordinates.get_mut(cell) {
                *slot = at.collect();
            }
        }
    }
    Ok(learn::Shape {
        coordinates,
        names: print::cell_names(&cells),
        cells: cells.total,
        lemma_cell,
        capitalized: part.capitalized,
        max_words: part.max_words,
        marks: None,
        part: part.name.to_string(),
        parts: parts(lang, part)?,
        closed: part.closed(),
        stated_length: lang.spelling.stated_length,
        inherent: part
            .inherent
            .iter()
            .map(|f| lang.feature(f).map_or(0, |f| f.values.len()))
            .collect(),
    })
}
