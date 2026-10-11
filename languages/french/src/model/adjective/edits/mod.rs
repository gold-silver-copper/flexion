//! Every edit: `Edit::new(front, prefix, strip, add, stress, mark, [stripped
//! letters, vowels in prefix, strip and add, front letters, front vowels])`.

use flexion::Edit;

mod c0;

pub(crate) fn get(i: usize) -> Option<&'static Edit<'static>> {
    match i / 20000 {
        0 => c0::ITEMS.get(i % 20000),
        _ => None,
    }
}
