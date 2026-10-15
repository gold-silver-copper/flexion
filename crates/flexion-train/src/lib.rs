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
pub mod measure;
pub mod par;
pub mod pipeline;
pub mod print;
pub mod relations;
pub mod script;
pub mod skeleton;
pub mod spec;
pub mod stems;
pub mod trace;

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
        names: print::cell_names(&cells),
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
