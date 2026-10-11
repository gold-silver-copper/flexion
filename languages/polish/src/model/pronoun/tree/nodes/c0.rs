use super::super::Node;
use flexion::tables::NodeKind;

const R: NodeKind = NodeKind::Root;
const W: NodeKind = NodeKind::Word;
const I: NodeKind = NodeKind::Inner;

static KINDS: [NodeKind; 23] = [
    R, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I,
    I, I, I, I, I, I, W,
];

static PARENTS: [u32; 23] = [
    u32::MAX, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 3, 3, 7, 7, 7,
    4, 4, 4, 4, 5, 5, 15,
];

/// Where each node's counts start; the last entry ends the last node's.
static STARTS: [u32; 24] = [
    0, 21, 22, 24, 26, 30, 35, 36, 40, 41, 42, 43, 44, 45, 46, 47,
    49, 50, 51, 52, 53, 56, 57, 58,
];

static CLASSES: [u32; 58] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
    16, 17, 18, 19, 20, 3, 7, 8, 0, 11, 1, 4, 5, 10, 13, 14,
    16, 18, 19, 20, 6, 9, 15, 17, 12, 2, 8, 0, 11, 6, 9, 15,
    17, 4, 5, 1, 10, 16, 18, 19, 13, 15,
];

static WEIGHTS: [f32; 58] = [
    2.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 3.0, 1.0, 3.0, 1.0, 3.0, 2.0, 1.0, 0.25, 0.25,
    1.0, 0.25, 0.67, 0.33, 0.25, 1.0, 3.0, 1.0, 2.0, 3.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.25,
    1.0, 0.67, 0.33, 0.25, 1.0, 3.0, 0.25, 0.25, 2.0, 1.0, 1.0, 2.0, 3.0, 1.0, 3.0, 0.25,
    0.25, 1.0, 1.0, 1.0, 1.0, 1.0, 0.67, 0.33, 1.0, 0.25,
];

pub(super) fn get(i: usize) -> Option<(Node, &'static [u32], &'static [f32])> {
    let start = usize::try_from(*STARTS.get(i)?).ok()?;
    let end = usize::try_from(*STARTS.get(i.checked_add(1)?)?).ok()?;
    let node = Node {
        kind: *KINDS.get(i)?,
        parent: *PARENTS.get(i)?,
    };
    Some((node, CLASSES.get(start..end)?, WEIGHTS.get(start..end)?))
}
