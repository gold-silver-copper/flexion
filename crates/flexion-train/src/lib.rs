//! Learn a language's inflection grammar from Wiktionary (Kaikki dumps), and
//! write it as a Rust crate.

/// Hash maps and sets with a fixed hasher: the same input fills them in the
/// same order, so the trainer writes the same crate byte for byte.
pub type HashMap<K, V> = std::collections::HashMap<K, V, Fixed>;
pub type HashSet<K> = std::collections::HashSet<K, Fixed>;
/// See [`HashMap`].
pub type Fixed = std::hash::BuildHasherDefault<std::collections::hash_map::DefaultHasher>;

pub mod baseline;
pub mod cells;
pub mod choose;
pub mod eval;
pub mod extract;
pub mod kaikki;
pub mod learn;
pub mod pipeline;
pub mod print;
pub mod script;
pub mod skeleton;
pub mod spec;
pub mod systems;

use spec::{Language, PartOfSpeech};

/// The shape of a part of speech's cells, from its config.
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

pub fn shape(lang: &Language, part: &PartOfSpeech) -> Result<learn::Shape, String> {
    let cells = cells::Cells::new(lang, part)?;
    let lemma_cell = match &part.lemma {
        Some((g, v)) => Some(
            cells
                .find(g, v)
                .ok_or(format!("{}: no lemma cell {g} {v:?}", part.name))?,
        ),
        None => None,
    };
    Ok(learn::Shape {
        cells: cells.total,
        lemma_cell,
        fallback: (0..cells.total).map(|c| cells.fallback(c)).collect(),
        capitalized: part.capitalized,
        inherent: part
            .inherent
            .iter()
            .map(|f| lang.feature(f).map_or(0, |f| f.values.len()))
            .collect(),
    })
}
