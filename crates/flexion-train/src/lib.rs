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
pub mod laws;
pub mod learn;
pub mod pipeline;
pub mod print;
pub mod relations;
pub mod script;
pub mod skeleton;
pub mod spec;

use spec::{Language, PartOfSpeech};

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
        cells: cells.total,
        lemma_cell,
        capitalized: part.capitalized,
        max_words: part.max_words,
        inherent: part
            .inherent
            .iter()
            .map(|f| lang.feature(f).map_or(0, |f| f.values.len()))
            .collect(),
    })
}
