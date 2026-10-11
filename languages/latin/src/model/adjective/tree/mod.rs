//! The suffix tree: a word's symbols (its letters from the last one back,
//! the walk's tokens, then the start of the word) choose the arms; each node
//! on the way adds its distribution over classes, `(class, probability)`,
//! and its total mass. Returns whether the walk reached the word's own node.

mod t0;
mod t1;
mod t2;
mod t3;
mod t4;
mod t5;
mod t6;
mod t7;
mod t8;

pub(crate) use t0::n0 as root;
