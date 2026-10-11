use super::super::Node;
use flexion::tables::NodeKind;

const R: NodeKind = NodeKind::Root;
const W: NodeKind = NodeKind::Word;
const I: NodeKind = NodeKind::Inner;

static KINDS: [NodeKind; 11] = [
    R, I, I, W, I, I, W, I, I, W, W,
];

static PARENTS: [u32; 11] = [
    u32::MAX, 0, 0, 2, 2, 2, 2, 2, 2, 4, 4,
];

/// Where each node's counts start; the last entry ends the last node's.
static STARTS: [u32; 12] = [
    0, 9, 11, 17, 18, 21, 23, 24, 25, 26, 27, 28,
];

static CLASSES: [u32; 28] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 2, 3, 0, 1, 4, 6, 7,
    8, 7, 4, 6, 0, 0, 1, 8, 0, 8, 4, 0,
];

static WEIGHTS: [f32; 28] = [
    22.75, 2.75, 2.0, 1.0, 1.0, 1.0, 1.25, 1.0, 2.25, 2.0, 1.0, 22.75, 2.75, 1.0, 1.25, 1.0,
    2.25, 1.0, 1.0, 1.25, 0.25, 5.25, 2.75, 0.25, 10.0, 2.0, 1.0, 0.25,
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
