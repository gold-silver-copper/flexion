use super::super::Node;
use flexion::tables::NodeKind;

const R: NodeKind = NodeKind::Root;
const W: NodeKind = NodeKind::Word;
const I: NodeKind = NodeKind::Inner;

static KINDS: [NodeKind; 25] = [
    R, W, I, I, W, I, I, W, W, W, W, W, W, W, W, W,
    I, W, W, W, W, W, W, W, W,
];

static PARENTS: [u32; 25] = [
    u32::MAX, 0, 0, 0, 0, 0, 0, 0, 2, 2, 3, 5, 5, 3, 3, 5,
    5, 5, 5, 5, 5, 5, 5, 5, 5,
];

/// Where each node's counts start; the last entry ends the last node's.
static STARTS: [u32; 26] = [
    0, 17, 19, 21, 26, 28, 31, 33, 34, 36, 38, 39, 41, 43, 46, 47,
    50, 51, 53, 55, 57, 59, 61, 63, 65, 67,
];

static CLASSES: [u32; 67] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
    16, 12, 13, 3, 4, 2, 7, 9, 10, 11, 14, 15, 0, 1, 5, 6,
    8, 16, 4, 3, 4, 3, 7, 0, 1, 0, 1, 9, 10, 11, 2, 5,
    0, 1, 5, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0,
    1, 0, 1,
];

static WEIGHTS: [f32; 67] = [
    6.77, 16.13, 1.0, 1.67, 0.83, 3.75, 10.0, 1.0, 2.25, 0.67, 0.33, 0.9, 0.67, 0.33, 0.67, 0.33,
    1.0, 0.67, 0.33, 1.67, 0.83, 1.0, 1.0, 0.67, 0.33, 0.9, 0.67, 0.33, 6.77, 16.13, 3.75, 10.0,
    2.25, 1.0, 0.67, 0.33, 0.17, 0.08, 1.0, 0.67, 0.33, 0.17, 0.08, 0.67, 0.33, 0.9, 1.0, 1.0,
    0.6, 0.3, 2.75, 0.67, 0.33, 0.67, 0.33, 0.67, 0.33, 0.67, 0.33, 0.67, 0.33, 0.67, 0.33, 0.67,
    0.33, 0.67, 0.33,
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
