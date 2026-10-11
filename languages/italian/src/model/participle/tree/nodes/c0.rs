use super::super::Node;
use flexion::tables::NodeKind;

const R: NodeKind = NodeKind::Root;
const W: NodeKind = NodeKind::Word;
const I: NodeKind = NodeKind::Inner;

static KINDS: [NodeKind; 14] = [
    R, I, I, I, I, I, I, I, I, I, I, I, W, W,
];

static PARENTS: [u32; 14] = [
    u32::MAX, 0, 0, 0, 1, 1, 2, 2, 2, 2, 2, 2, 3, 4,
];

/// Where each node's counts start; the last entry ends the last node's.
static STARTS: [u32; 15] = [
    0, 13, 18, 24, 26, 28, 31, 32, 33, 34, 35, 36, 37, 38, 39,
];

static CLASSES: [u32; 39] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 2, 6, 7,
    9, 11, 1, 3, 4, 5, 8, 10, 0, 12, 6, 9, 2, 7, 11, 4,
    5, 1, 3, 8, 10, 12, 6,
];

static WEIGHTS: [f32; 39] = [
    7954.8, 23.0, 5.0, 34.0, 8.0, 17.0, 1.0, 3.0, 5.0, 4.0, 2.0, 1.0, 0.25, 5.0, 1.0, 3.0,
    4.0, 1.0, 23.0, 34.0, 8.0, 17.0, 5.0, 2.0, 7954.8, 0.25, 1.0, 4.0, 5.0, 3.0, 1.0, 8.0,
    17.0, 23.0, 34.0, 5.0, 2.0, 0.25, 1.0,
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
