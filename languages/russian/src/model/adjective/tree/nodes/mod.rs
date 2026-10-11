//! Every tree node with a distribution, in the order the tree functions
//! number them, and its counts.

use super::Node;

mod c0;
mod c1;

/// Node `i` and its counts: its classes and their weights.
pub(crate) fn get(i: usize) -> Option<(Node, &'static [u32], &'static [f32])> {
    match i / 20000 {
        0 => c0::get(i % 20000),
        1 => c1::get(i % 20000),
        _ => None,
    }
}
