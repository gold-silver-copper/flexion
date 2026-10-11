use super::super::Node;
use flexion::tables::NodeKind;

const R: NodeKind = NodeKind::Root;
const W: NodeKind = NodeKind::Word;
const I: NodeKind = NodeKind::Inner;

static KINDS: [NodeKind; 29] = [
    R, I, I, I, I, I, I, W, I, W, W, I, I, I, I, I,
    I, I, W, W, I, W, W, I, I, W, I, I, W,
];

static PARENTS: [u32; 29] = [
    u32::MAX, 0, 0, 0, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3,
    3, 3, 4, 4, 6, 6, 8, 13, 15, 16, 14, 14, 14,
];

/// Where each node's counts start; the last entry ends the last node's.
static STARTS: [u32; 30] = [
    0, 21, 31, 37, 45, 47, 49, 52, 53, 55, 56, 57, 58, 59, 62, 65,
    67, 69, 70, 71, 72, 73, 75, 76, 77, 78, 79, 80, 81, 82,
];

static CLASSES: [u32; 82] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
    16, 17, 18, 19, 20, 1, 3, 4, 0, 11, 15, 16, 17, 19, 20, 0,
    5, 6, 9, 12, 16, 2, 7, 8, 10, 13, 14, 0, 18, 1, 4, 15,
    16, 11, 20, 0, 17, 0, 19, 3, 16, 0, 9, 5, 6, 12, 7, 10,
    14, 2, 0, 8, 18, 13, 1, 4, 11, 20, 0, 0, 12, 2, 18, 10,
    14, 7,
];

static WEIGHTS: [f32; 82] = [
    4.15, 1.0, 8.0, 1.0, 1.0, 1.0, 0.25, 0.25, 1.25, 1.5, 0.75, 3.0, 0.75, 0.5, 0.5, 1.0,
    1.25, 1.0, 0.25, 1.0, 1.0, 1.0, 1.0, 1.0, 1.15, 3.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0,
    1.0, 0.25, 1.5, 0.75, 0.25, 8.0, 0.25, 1.25, 0.75, 0.5, 0.5, 1.0, 0.25, 1.0, 1.0, 1.0,
    1.0, 3.0, 1.0, 0.9, 1.0, 0.25, 1.0, 1.0, 0.25, 2.0, 1.5, 1.0, 0.25, 0.75, 0.25, 0.75,
    0.5, 8.0, 1.0, 1.25, 0.25, 0.5, 1.0, 1.0, 3.0, 1.0, 0.9, 0.25, 0.75, 8.0, 0.25, 0.75,
    0.5, 0.25,
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
