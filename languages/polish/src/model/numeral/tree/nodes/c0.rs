use super::super::Node;
use flexion::tables::NodeKind;

const R: NodeKind = NodeKind::Root;
const W: NodeKind = NodeKind::Word;
const I: NodeKind = NodeKind::Inner;

static KINDS: [NodeKind; 20] = [
    R, I, I, I, I, I, I, W, I, I, I, I, I, I, W, W,
    W, W, W, W,
];

static PARENTS: [u32; 20] = [
    u32::MAX, 0, 0, 0, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 5, 6,
    2, 2, 11, 13,
];

/// Where each node's counts start; the last entry ends the last node's.
static STARTS: [u32; 21] = [
    0, 24, 29, 44, 54, 58, 60, 62, 63, 65, 66, 69, 71, 72, 76, 77,
    78, 79, 80, 81, 82,
];

static CLASSES: [u32; 82] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
    16, 17, 18, 19, 20, 21, 22, 23, 3, 4, 5, 9, 20, 2, 8, 9,
    12, 3, 4, 5, 15, 11, 17, 18, 21, 13, 22, 23, 0, 1, 6, 7,
    10, 11, 13, 14, 16, 19, 3, 4, 5, 13, 8, 22, 15, 18, 12, 11,
    17, 9, 6, 13, 14, 1, 7, 0, 10, 11, 16, 19, 8, 15, 2, 23,
    7, 16,
];

static WEIGHTS: [f32; 82] = [
    2.0, 8.0, 1.0, 1.14, 0.57, 0.29, 1.0, 1.0, 1.0, 3.0, 5.0, 3.0, 1.0, 1.67, 0.33, 1.0,
    1.0, 0.25, 1.0, 1.0, 1.0, 1.0, 2.0, 0.25, 0.57, 0.29, 0.14, 1.0, 1.0, 1.0, 1.0, 2.0,
    1.0, 0.57, 0.29, 0.14, 1.0, 2.0, 0.25, 1.0, 1.0, 1.0, 2.0, 0.25, 2.0, 8.0, 1.0, 1.0,
    5.0, 1.0, 0.67, 0.33, 1.0, 1.0, 0.57, 0.29, 0.14, 1.0, 1.0, 2.0, 1.0, 1.0, 1.0, 2.0,
    0.25, 2.0, 1.0, 0.67, 0.33, 8.0, 1.0, 2.0, 5.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.25,
    1.0, 1.0,
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
