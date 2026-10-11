//! adjective: 49 cells, 1873 edits, 628 classes, 2430 tree nodes.

use flexion::{Class, Edit, Join, Path, PosModel, Walk};

mod classes;
mod edits;
mod tree;
mod lexicon;
mod phonology;

/// Per cell, which model builds it when the caller gives principal parts:
/// 0 this one, `k + 1` part model `k`.
static SOURCES: [u8; 49] = [0, 1, 0, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 0, 0, 0, 1, 0];

pub(crate) struct Adjective;

impl PosModel for Adjective {
    fn cells(&self) -> usize {
        49
    }
    fn inherent(&self) -> usize {
        0
    }
    fn lemma_cell(&self) -> Option<usize> {
        Some(48)
    }
    fn capitalized(&self) -> bool {
        false
    }
    fn walk(&self) -> Walk {
        Walk {
            syllable_depth: Some(0),
            syllable_max: 3,
            initial_depth: None,
        }
    }
    fn restore(&self) -> Option<usize> {
        Some(7)
    }
    fn restore_front(&self) -> Option<usize> {
        Some(8)
    }
    fn edit(&self, id: usize) -> Option<Edit<'_>> {
        edits::get(id).copied()
    }
    fn classes(&self) -> usize {
        628
    }
    fn class(&self, class: usize) -> Option<Class<'_>> {
        classes::get(class).map(|c| c.class)
    }
    fn class_edit(&self, class: usize, cell: usize) -> Option<u32> {
        classes::edit(class, cell)
    }
    fn path<'s>(&'s self, symbols: &[char], out: &mut Path<'s>) -> bool {
        tree::root(symbols, out)
    }
    fn has_joins(&self) -> bool {
        false
    }
    fn join(&self, _prefix: &str) -> Option<Join<'_>> {
        None
    }
    fn join_edit(&self, _class: usize, _cell: usize) -> Option<u32> {
        None
    }
    fn part_models(&self) -> &[(usize, usize)] {
        &[(3, 13)]
    }
    fn source(&self, cell: usize) -> u8 {
        SOURCES.get(cell).copied().unwrap_or(0)
    }
    fn lexicon(&self, key: &str) -> Option<&str> {
        lexicon::find(key)
    }
    fn lexicon_from(&self, i: usize) -> Option<(usize, &[usize])> {
        match i {
            0 => Some((2, &[609, 610, 611, 612, 613, 614, 615, 616, 617, 618, 619, 620, 621, 622, 623])),
            _ => None,
        }
    }
    fn phonology(&self) -> Option<&flexion::sound_law::Program> {
        phonology::program()
    }
}

/// Whether the sound laws compile.
pub(crate) fn sound_laws_compile() -> bool {
    phonology::program().is_some()
}
