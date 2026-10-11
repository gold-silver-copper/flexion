use super::super::Node;
use flexion::tables::NodeKind;

const R: NodeKind = NodeKind::Root;
const I: NodeKind = NodeKind::Inner;

static KINDS: [NodeKind; 11] = [
    R, I, I, I, I, I, I, I, I, I, I,
];

static PARENTS: [u32; 11] = [
    u32::MAX, 0, 0, 0, 3, 3, 1, 1, 4, 6, 7,
];

/// Where each node's counts start; the last entry ends the last node's.
static STARTS: [u32; 12] = [
    0, 9, 13, 14, 18, 21, 22, 24, 26, 27, 28, 29,
];

static CLASSES: [u32; 29] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 2, 3, 5, 8, 4, 0, 1,
    6, 7, 0, 6, 7, 1, 3, 8, 2, 5, 0, 3, 2,
];

static WEIGHTS: [f32; 29] = [
    1.0, 3.0, 2.0, 2.0, 1.0, 6.0, 0.25, 1.25, 1.0, 2.0, 2.0, 6.0, 1.0, 1.0, 1.0, 3.0,
    0.25, 1.25, 1.0, 0.25, 1.25, 2.0, 2.0, 1.0, 2.0, 6.0, 1.0, 2.0, 2.0,
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
