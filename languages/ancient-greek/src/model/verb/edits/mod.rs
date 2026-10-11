//! Every edit: `Edit::new(front, prefix, strip, add, stress, mark, [stripped
//! letters, vowels in prefix, strip and add, front letters, front vowels])`.

use flexion::Edit;

mod c0;
mod c1;
mod c2;
mod c3;
mod c4;
mod c5;
mod c6;

pub(crate) fn get(i: usize) -> Option<&'static Edit<'static>> {
    match i / 20000 {
        0 => c0::ITEMS.get(i % 20000),
        1 => c1::ITEMS.get(i % 20000),
        2 => c2::ITEMS.get(i % 20000),
        3 => c3::ITEMS.get(i % 20000),
        4 => c4::ITEMS.get(i % 20000),
        5 => c5::ITEMS.get(i % 20000),
        6 => c6::ITEMS.get(i % 20000),
        _ => None,
    }
}
