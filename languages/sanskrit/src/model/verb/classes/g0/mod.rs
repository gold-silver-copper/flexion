//! Group 0's shared edit vectors, one edit per cell of the group.

mod c0;

pub(crate) fn get(i: usize) -> Option<&'static [u16; 72]> {
    match i / 20000 {
        0 => c0::ITEMS.get(i % 20000),
        _ => None,
    }
}
