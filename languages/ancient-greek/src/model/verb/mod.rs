//! verb: 625 cells, 133383 edits, 6602 classes, 8241 tree nodes.

use flexion::{Class, Edit, Join, Path, PosModel, Walk};

mod classes;
mod edits;
mod tree;

/// Per cell, which model builds it when the caller gives principal parts:
/// 0 this one, `k + 1` part model `k`.
static SOURCES: [u8; 625] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 2, 2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 2, 2, 2, 2, 2, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 2, 2, 2, 2, 2, 0, 0, 5, 5, 5, 5, 5, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 3, 3, 3, 3, 3, 3, 0, 0, 4, 4, 4, 4, 4, 4, 4, 4, 0, 4, 4, 4, 4, 4, 4, 4, 3, 0, 3, 3, 3, 3, 3, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 3, 3, 3, 3, 3, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 3, 3, 3, 3, 3, 0, 0, 0, 4, 4, 4, 4, 4, 4, 0, 0, 0, 4, 4, 4, 4, 4, 4, 3, 0, 3, 3, 3, 3, 3, 3, 3, 4, 0, 4, 4, 4, 4, 4, 4, 3, 4, 0, 4, 4, 4, 4, 4, 4, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 3, 4, 4, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 3, 4, 4, 0, 0, 0, 0];

pub(crate) struct Verb;

impl PosModel for Verb {
    fn cells(&self) -> usize {
        625
    }
    fn inherent(&self) -> usize {
        0
    }
    fn lemma_cell(&self) -> Option<usize> {
        Some(624)
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
        Some(9)
    }
    fn restore_front(&self) -> Option<usize> {
        Some(10)
    }
    fn edit(&self, id: usize) -> Option<Edit<'_>> {
        edits::get(id).copied()
    }
    fn classes(&self) -> usize {
        6602
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
        &[(135, 13), (243, 14), (351, 15), (360, 16), (261, 17)]
    }
    fn source(&self, cell: usize) -> u8 {
        SOURCES.get(cell).copied().unwrap_or(0)
    }
    fn lexicon(&self, _key: &str) -> Option<&str> {
        None
    }
}
