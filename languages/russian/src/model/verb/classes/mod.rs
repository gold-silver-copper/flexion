//! The classes: per class its record and, per group of cells, which of
//! the group's shared edit vectors it has (`u16::MAX`: no form in that cell).

use flexion::{Class, NONE};

mod g0;
mod g1;
mod g2;
mod g3;
mod g4;
mod g5;
mod g6;
mod list;

pub(crate) use list::get;

pub(crate) struct ClassDef {
    pub(crate) class: Class<'static>,
    /// One shared edit vector per group of cells.
    pub(crate) subs: [u32; 7],
}

/// Each cell's group and its place in the group.
static CELL_GROUP: [(usize, usize); 20] = [(0, 0), (1, 0), (1, 1), (1, 2), (1, 3), (1, 4), (1, 5), (2, 0), (2, 1), (2, 2), (2, 3), (3, 0), (3, 1), (4, 0), (4, 1), (4, 2), (4, 3), (5, 0), (5, 1), (6, 0)];

/// Class `class`'s edit for `cell` (`NONE`: no form).
pub(crate) fn edit(class: usize, cell: usize) -> Option<u32> {
    let c = get(class)?;
    let &(g, off) = CELL_GROUP.get(cell)?;
    let sub = usize::try_from(*c.subs.get(g)?).ok()?;
    let e = *match g {
        0 => g0::get(sub)?.get(off)?,
        1 => g1::get(sub)?.get(off)?,
        2 => g2::get(sub)?.get(off)?,
        3 => g3::get(sub)?.get(off)?,
        4 => g4::get(sub)?.get(off)?,
        5 => g5::get(sub)?.get(off)?,
        6 => g6::get(sub)?.get(off)?,
        _ => return None,
    };
    Some(if e == u16::MAX { NONE } else { u32::from(e) })
}
