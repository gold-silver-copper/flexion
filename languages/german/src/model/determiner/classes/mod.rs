//! The classes: per class its record and, per group of cells, which of
//! the group's shared edit vectors it has (`u16::MAX`: no form in that cell).

use flexion::{Class, NONE};

mod g0;
mod g1;
mod list;

pub(crate) use list::get;

pub(crate) struct ClassDef {
    pub(crate) class: Class<'static>,
    /// One shared edit vector per group of cells.
    pub(crate) subs: [u32; 2],
}

/// Each cell's group and its place in the group.
static CELL_GROUP: [(usize, usize); 17] = [(0, 0), (0, 1), (0, 2), (0, 3), (0, 4), (0, 5), (0, 6), (0, 7), (0, 8), (0, 9), (0, 10), (0, 11), (0, 12), (0, 13), (0, 14), (0, 15), (1, 0)];

/// Class `class`'s edit for `cell` (`NONE`: no form).
pub(crate) fn edit(class: usize, cell: usize) -> Option<u32> {
    let c = get(class)?;
    let &(g, off) = CELL_GROUP.get(cell)?;
    let sub = usize::try_from(*c.subs.get(g)?).ok()?;
    let e = *match g {
        0 => g0::get(sub)?.get(off)?,
        1 => g1::get(sub)?.get(off)?,
        _ => return None,
    };
    Some(if e == u16::MAX { NONE } else { u32::from(e) })
}
