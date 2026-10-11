//! The classes: per class its record and, per group of cells, which of
//! the group's shared edit vectors it has (`u16::MAX`: no form in that cell).

use flexion::{Class, NONE};

mod g0;
mod g1;
mod g2;
mod g3;
mod g4;
mod list;

pub(crate) use list::get;

pub(crate) struct ClassDef {
    pub(crate) class: Class<'static>,
    /// One shared edit vector per group of cells.
    pub(crate) subs: [u32; 5],
}

/// Each cell's group and its place in the group.
static CELL_GROUP: [(usize, usize); 148] = [(0, 0), (0, 1), (0, 2), (0, 3), (0, 4), (0, 5), (0, 6), (0, 7), (0, 8), (0, 9), (0, 10), (0, 11), (0, 12), (0, 13), (0, 14), (0, 15), (0, 16), (0, 17), (0, 18), (0, 19), (0, 20), (0, 21), (0, 22), (0, 23), (0, 24), (0, 25), (0, 26), (0, 27), (0, 28), (0, 29), (0, 30), (0, 31), (0, 32), (0, 33), (0, 34), (0, 35), (0, 36), (0, 37), (0, 38), (0, 39), (0, 40), (0, 41), (0, 42), (0, 43), (0, 44), (0, 45), (0, 46), (0, 47), (1, 0), (1, 1), (1, 2), (1, 3), (1, 4), (1, 5), (1, 6), (1, 7), (1, 8), (1, 9), (1, 10), (1, 11), (1, 12), (1, 13), (1, 14), (1, 15), (1, 16), (1, 17), (1, 18), (1, 19), (1, 20), (1, 21), (1, 22), (1, 23), (1, 24), (1, 25), (1, 26), (1, 27), (1, 28), (1, 29), (1, 30), (1, 31), (1, 32), (1, 33), (1, 34), (1, 35), (1, 36), (1, 37), (1, 38), (1, 39), (1, 40), (1, 41), (1, 42), (1, 43), (1, 44), (1, 45), (1, 46), (1, 47), (2, 0), (2, 1), (2, 2), (2, 3), (2, 4), (2, 5), (2, 6), (2, 7), (2, 8), (2, 9), (2, 10), (2, 11), (2, 12), (2, 13), (2, 14), (2, 15), (2, 16), (2, 17), (2, 18), (2, 19), (2, 20), (2, 21), (2, 22), (2, 23), (2, 24), (2, 25), (2, 26), (2, 27), (2, 28), (2, 29), (2, 30), (2, 31), (2, 32), (2, 33), (2, 34), (2, 35), (2, 36), (2, 37), (2, 38), (2, 39), (2, 40), (2, 41), (2, 42), (2, 43), (2, 44), (2, 45), (2, 46), (2, 47), (3, 0), (3, 1), (3, 2), (4, 0)];

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
        _ => return None,
    };
    Some(if e == u16::MAX { NONE } else { u32::from(e) })
}
