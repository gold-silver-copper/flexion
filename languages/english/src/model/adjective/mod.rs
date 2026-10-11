//! adjective: 4 cells, 92 edits, 65 classes, 3311 tree nodes.

use flexion::{Class, Edit, Join, Path, PosModel, Walk};

mod classes;
mod edits;
mod tree;

/// Per cell, which model builds it when the caller gives principal parts:
/// 0 this one, `k + 1` part model `k`.
static SOURCES: [u8; 4] = [0, 0, 0, 0];

pub(crate) struct Adjective;

impl PosModel for Adjective {
    fn cells(&self) -> usize {
        4
    }
    fn inherent(&self) -> usize {
        0
    }
    fn lemma_cell(&self) -> Option<usize> {
        Some(3)
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
        None
    }
    fn restore_front(&self) -> Option<usize> {
        None
    }
    fn edit(&self, id: usize) -> Option<Edit<'_>> {
        edits::get(id).copied()
    }
    fn classes(&self) -> usize {
        65
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
}
