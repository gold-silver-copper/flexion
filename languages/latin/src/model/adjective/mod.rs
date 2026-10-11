//! adjective: 39 cells, 9915 edits, 425 classes, 24248 tree nodes.

use flexion::{Class, Edit, Join, Path, PosModel, Walk};

mod classes;
mod edits;
mod tree;
mod lexicon;
mod joins;

/// Per cell, which model builds it when the caller gives principal parts:
/// 0 this one, `k + 1` part model `k`.
static SOURCES: [u8; 39] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

pub(crate) struct Adjective;

impl PosModel for Adjective {
    fn cells(&self) -> usize {
        39
    }
    fn inherent(&self) -> usize {
        0
    }
    fn lemma_cell(&self) -> Option<usize> {
        Some(38)
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
        Some(7)
    }
    fn restore_front(&self) -> Option<usize> {
        Some(8)
    }
    fn edit(&self, id: usize) -> Option<Edit<'_>> {
        edits::get(id).copied()
    }
    fn classes(&self) -> usize {
        425
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
        true
    }
    fn join(&self, prefix: &str) -> Option<Join<'_>> {
        joins::join(prefix)
    }
    fn join_edit(&self, class: usize, cell: usize) -> Option<u32> {
        joins::edit(class, cell)
    }
    fn part_models(&self) -> &[(usize, usize)] {
        &[]
    }
    fn source(&self, cell: usize) -> u8 {
        SOURCES.get(cell).copied().unwrap_or(0)
    }
    fn lexicon(&self, key: &str) -> Option<&str> {
        lexicon::find(key)
    }
}
