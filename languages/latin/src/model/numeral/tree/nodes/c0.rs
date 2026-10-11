use super::super::Node;
use flexion::tables::NodeKind;

const R: NodeKind = NodeKind::Root;
const W: NodeKind = NodeKind::Word;
const I: NodeKind = NodeKind::Inner;

static KINDS: [NodeKind; 13] = [
    R, W, I, I, I, I, W, I, I, W, W, I, I,
];

static PARENTS: [u32; 13] = [
    u32::MAX, 0, 0, 0, 0, 0, 4, 5, 5, 8, 8, 7, 7,
];

/// Where each node's counts start; the last entry ends the last node's.
static STARTS: [u32; 14] = [
    0, 14, 15, 16, 17, 21, 28, 30, 32, 36, 37, 39, 41, 42,
];

static CLASSES: [u32; 42] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 9, 5,
    2, 3, 4, 6, 7, 0, 1, 8, 10, 11, 12, 13, 6, 7, 1, 8,
    0, 1, 12, 13, 1, 12, 13, 1, 8, 1,
];

static WEIGHTS: [f32; 42] = [
    21.67, 64.33, 8.25, 0.67, 0.33, 2.0, 0.17, 0.08, 6.25, 0.25, 0.67, 0.33, 0.67, 0.33, 0.25, 2.0,
    8.25, 0.67, 0.33, 0.17, 0.08, 21.67, 64.33, 6.25, 0.67, 0.33, 0.67, 0.33, 0.17, 0.08, 42.25, 6.25,
    21.67, 11.83, 0.67, 0.33, 1.0, 0.67, 0.33, 3.0, 6.25, 38.0,
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
