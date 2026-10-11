use super::super::Node;
use flexion::tables::NodeKind;

const R: NodeKind = NodeKind::Root;
const W: NodeKind = NodeKind::Word;
const I: NodeKind = NodeKind::Inner;

static KINDS: [NodeKind; 10] = [
    R, I, I, I, I, I, I, I, I, W,
];

static PARENTS: [u32; 10] = [
    u32::MAX, 0, 0, 0, 3, 3, 3, 5, 4, 4,
];

/// Where each node's counts start; the last entry ends the last node's.
static STARTS: [u32; 11] = [
    0, 21, 26, 30, 42, 50, 53, 54, 56, 58, 62,
];

static CLASSES: [u32; 62] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
    16, 17, 18, 19, 20, 3, 17, 18, 19, 20, 4, 5, 6, 7, 0, 1,
    2, 8, 9, 10, 11, 12, 13, 14, 15, 16, 0, 1, 9, 10, 11, 12,
    13, 14, 8, 15, 16, 2, 15, 16, 0, 1, 9, 10, 11, 12,
];

static WEIGHTS: [f32; 62] = [
    0.67, 0.33, 1.0, 10.52, 0.53, 0.27, 0.13, 0.07, 1.0, 0.13, 0.07, 0.03, 0.02, 0.83, 0.42, 0.67,
    0.33, 0.26, 0.13, 0.06, 0.03, 10.52, 0.26, 0.13, 0.06, 0.03, 0.53, 0.27, 0.13, 0.07, 0.67, 0.33,
    1.0, 1.0, 0.13, 0.07, 0.03, 0.02, 0.83, 0.42, 0.67, 0.33, 0.67, 0.33, 0.13, 0.07, 0.03, 0.02,
    0.83, 0.42, 1.0, 0.67, 0.33, 1.0, 0.67, 0.33, 0.67, 0.33, 0.13, 0.07, 0.03, 0.02,
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
