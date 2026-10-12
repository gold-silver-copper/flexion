//! Learn a language's inflection grammar from Wiktionary (Kaikki dumps), and
//! write it as a Rust crate.

pub mod cells;
pub mod choose;
pub mod eval;
pub mod extract;
pub mod kaikki;
pub mod learn;
pub mod pipeline;
pub mod print;
pub mod script;
pub mod spec;
pub mod skeleton;

use spec::{Language, PartOfSpeech};

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
