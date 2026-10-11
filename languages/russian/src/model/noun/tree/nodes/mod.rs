//! Every tree node with a distribution, in the order the tree functions
//! number them, and its counts.

use super::Node;

mod c0;
mod c1;
mod c2;
mod c3;
mod c4;
mod c5;
mod c6;
mod c7;

/// Node `i` and its counts: its classes and their weights.
pub(crate) fn get(i: usize) -> Option<(Node, &'static [u32], &'static [f32])> {
    match i / 20000 {
        0 => c0::get(i % 20000),
        1 => c1::get(i % 20000),
        2 => c2::get(i % 20000),
        3 => c3::get(i % 20000),
        4 => c4::get(i % 20000),
        5 => c5::get(i % 20000),
        6 => c6::get(i % 20000),
        7 => c7::get(i % 20000),
        _ => None,
    }
}
