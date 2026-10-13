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
    let lemma_cell = match &part.lemma {
        Some((g, v)) => Some(
            cells
                .find(g, v)
                .ok_or(format!("{}: no lemma cell {g} {v:?}", part.name))?,
        ),
        None => None,
    };
    // Each cell's group and feature values, on dimensions shared by name.
    let mut features: Vec<&str> = Vec::new();
    for g in &cells.groups {
        for f in &g.features {
            if !features.contains(&f.name) {
                features.push(f.name);
            }
        }
    }
    let mut coordinates = vec![Vec::new(); cells.total];
    for (gi, g) in cells.groups.iter().enumerate() {
        for cell in g.offset..g.offset + g.len() {
            let values = g.values(cell);
            let mut at = vec![u16::MAX; features.len() + 1];
            if let Some(slot) = at.first_mut() {
                *slot = u16::try_from(gi).unwrap_or(u16::MAX);
            }
            for (f, v) in g.features.iter().zip(values) {
                if let Some(slot) = features
                    .iter()
                    .position(|&n| n == f.name)
                    .and_then(|i| at.get_mut(i + 1))
                {
                    *slot = u16::try_from(v).unwrap_or(u16::MAX);
                }
            }
            if let Some(slot) = coordinates.get_mut(cell) {
                *slot = at;
            }
        }
    }
    Ok(learn::Shape {
        coordinates,
        cells: cells.total,
        lemma_cell,
        capitalized: part.capitalized,
        inherent: part
            .inherent
            .iter()
            .map(|f| lang.feature(f).map_or(0, |f| f.values.len()))
            .collect(),
    })
}
