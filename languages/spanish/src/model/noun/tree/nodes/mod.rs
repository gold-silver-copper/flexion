//! Every tree node with a distribution, in the order the tree functions
//! number them.

use super::Node;

mod c0;

pub(crate) fn get(i: usize) -> Option<&'static Node> {
    match i / 20000 {
        0 => c0::ITEMS.get(i % 20000),
        _ => None,
    }
}
