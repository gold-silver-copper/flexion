use super::super::Node;
use flexion::tables::NodeKind;

const R: NodeKind = NodeKind::Root;
const W: NodeKind = NodeKind::Word;
const I: NodeKind = NodeKind::Inner;

static KINDS: [NodeKind; 9] = [
    R, W, I, I, I, W, W, W, W,
];

static PARENTS: [u32; 9] = [
    u32::MAX, 0, 0, 0, 0, 2, 4, 3, 3,
];

/// Where each node's counts start; the last entry ends the last node's.
static STARTS: [u32; 10] = [
    0, 14, 15, 17, 21, 26, 27, 29, 31, 32,
];

static CLASSES: [u32; 32] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 6, 1,
    4, 9, 10, 12, 13, 2, 5, 7, 8, 11, 1, 7, 8, 9, 10, 12,
];

static WEIGHTS: [f32; 32] = [
    3.0, 1.0, 1.0, 1.0, 2.0, 1.0, 0.25, 0.67, 0.33, 0.25, 0.23, 1.0, 1.0, 1.0, 0.25, 1.0,
    2.0, 0.25, 0.23, 1.0, 1.0, 1.0, 1.0, 0.67, 0.33, 1.0, 1.0, 0.67, 0.33, 0.25, 0.23, 1.0,
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
