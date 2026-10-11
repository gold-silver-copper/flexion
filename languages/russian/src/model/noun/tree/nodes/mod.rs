//! Every tree node with a distribution, in the order the tree functions
//! number them.

use super::Node;

mod c0;
mod c1;
mod c2;
mod c3;
mod c4;
mod c5;
mod c6;
mod c7;

pub(crate) fn get(i: usize) -> Option<&'static Node> {
    match i / 20000 {
        0 => c0::ITEMS.get(i % 20000),
        1 => c1::ITEMS.get(i % 20000),
        2 => c2::ITEMS.get(i % 20000),
        3 => c3::ITEMS.get(i % 20000),
        4 => c4::ITEMS.get(i % 20000),
        5 => c5::ITEMS.get(i % 20000),
        6 => c6::ITEMS.get(i % 20000),
        7 => c7::ITEMS.get(i % 20000),
        _ => None,
    }
}
