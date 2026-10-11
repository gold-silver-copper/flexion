use super::super::Node;
use flexion::tables::NodeKind;

const R: NodeKind = NodeKind::Root;
const W: NodeKind = NodeKind::Word;
const I: NodeKind = NodeKind::Inner;

static KINDS: [NodeKind; 33] = [
    R, I, I, I, I, I, W, I, I, I, I, W, W, I, I, I,
    W, I, I, I, I, I, W, W, I, I, I, I, W, I, I, W,
    W,
];

static PARENTS: [u32; 33] = [
    u32::MAX, 0, 0, 0, 0, 0, 1, 3, 3, 3, 4, 4, 7, 8, 8, 9,
    9, 9, 9, 9, 13, 13, 15, 19, 19, 19, 15, 15, 17, 17, 17, 24,
    30,
];

/// Where each node's counts start; the last entry ends the last node's.
static STARTS: [u32; 34] = [
    0, 16, 18, 20, 31, 33, 34, 35, 37, 40, 48, 49, 50, 51, 53, 54,
    57, 58, 61, 62, 65, 66, 68, 69, 70, 72, 73, 74, 76, 77, 78, 80,
    81, 82,
];

static CLASSES: [u32; 82] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
    7, 8, 0, 11, 1, 3, 2, 4, 5, 6, 9, 10, 12, 13, 15, 2,
    14, 2, 7, 3, 2, 2, 12, 15, 1, 2, 4, 5, 6, 9, 10, 13,
    2, 14, 3, 2, 15, 12, 2, 10, 13, 9, 1, 2, 6, 2, 2, 4,
    5, 2, 2, 15, 10, 4, 5, 2, 2, 2, 13, 2, 6, 2, 1, 2,
    5, 1,
];

static WEIGHTS: [f32; 82] = [
    1.0, 1.0, 34.58, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.25, 1.0, 4.44, 1.0, 1.0, 1.0,
    1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 21.91, 1.0, 1.0, 1.0, 1.0, 0.25, 4.44, 1.0, 1.0, 5.5,
    1.0, 3.25, 1.0, 1.0, 1.0, 3.25, 4.44, 1.0, 1.0, 17.66, 1.0, 1.0, 1.0, 1.0, 0.25, 1.0,
    5.5, 1.0, 1.0, 3.25, 1.0, 4.44, 2.73, 0.25, 1.0, 1.0, 1.0, 3.25, 1.0, 3.0, 5.25, 1.0,
    1.0, 2.0, 0.25, 1.0, 0.25, 1.0, 1.0, 1.0, 4.25, 1.48, 1.0, 0.25, 1.0, 1.25, 1.0, 2.0,
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
