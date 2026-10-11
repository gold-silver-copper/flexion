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
mod g7;
mod g8;
mod g9;
mod list;

pub(crate) use list::get;

pub(crate) struct ClassDef {
    pub(crate) class: Class<'static>,
    /// One shared edit vector per group of cells.
    pub(crate) subs: [u32; 10],
}

/// Each cell's group and its place in the group.
static CELL_GROUP: [(usize, usize); 50] = [(0, 0), (1, 0), (1, 1), (1, 2), (1, 3), (1, 4), (1, 5), (2, 0), (2, 1), (2, 2), (2, 3), (2, 4), (2, 5), (2, 6), (2, 7), (2, 8), (2, 9), (2, 10), (2, 11), (2, 12), (2, 13), (2, 14), (3, 0), (3, 1), (3, 2), (3, 3), (3, 4), (3, 5), (3, 6), (3, 7), (3, 8), (3, 9), (3, 10), (3, 11), (3, 12), (3, 13), (3, 14), (4, 0), (4, 1), (4, 2), (4, 3), (4, 4), (4, 5), (5, 0), (6, 0), (6, 1), (7, 0), (7, 1), (8, 0), (9, 0)];

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
        7 => g7::get(sub)?.get(off)?,
        8 => g8::get(sub)?.get(off)?,
        9 => g9::get(sub)?.get(off)?,
        _ => return None,
    };
    Some(if e == u16::MAX { NONE } else { u32::from(e) })
}
