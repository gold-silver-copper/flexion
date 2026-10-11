//! Every class, with a word of it and its first forms.

use super::ClassDef;

mod c0;

pub(crate) fn get(i: usize) -> Option<&'static ClassDef> {
    match i / 20000 {
        0 => c0::ITEMS.get(i % 20000),
        _ => None,
    }
}
