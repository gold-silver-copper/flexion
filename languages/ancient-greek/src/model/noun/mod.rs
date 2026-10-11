//! noun: 16 cells, 2764 edits, 1284 classes, 7440 tree nodes.

use flexion::{Class, Edit, Join, Path, PosModel, Walk};

mod classes;
mod edits;
mod tree;
mod phonology;

/// Per cell, which model builds it when the caller gives principal parts:
/// 0 this one, `k + 1` part model `k`.
static SOURCES: [u8; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

pub(crate) struct Noun;

impl PosModel for Noun {
    fn cells(&self) -> usize {
        16
    }
    fn inherent(&self) -> usize {
        1
    }
    fn lemma_cell(&self) -> Option<usize> {
        Some(15)
    }
    fn capitalized(&self) -> bool {
        false
    }
    fn walk(&self) -> Walk {
        Walk {
            syllable_depth: None,
            syllable_max: 3,
            initial_depth: None,
        }
    }
    fn restore(&self) -> Option<usize> {
        Some(5)
    }
    fn restore_front(&self) -> Option<usize> {
        Some(6)
    }
    fn edit(&self, id: usize) -> Option<Edit<'_>> {
        edits::get(id).copied()
    }
    fn classes(&self) -> usize {
        1284
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
        &[]
    }
    fn source(&self, cell: usize) -> u8 {
        SOURCES.get(cell).copied().unwrap_or(0)
    }
    fn lexicon(&self, _key: &str) -> Option<&str> {
        None
    }
    fn phonology(&self) -> Option<&flexion::sound_law::Program> {
        phonology::program()
    }
}

/// Whether the sound laws compile.
pub(crate) fn sound_laws_compile() -> bool {
    phonology::program().is_some()
}
