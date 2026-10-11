//! The suffix tree: a word's symbols (its letters from the last one back,
//! the walk's tokens, then the start of the word) choose the arms; each node
//! on the way adds its distribution over classes. A node stores the classes
//! of the words under it, counted; its distribution is computed from them on
//! first use, smoothed toward its nearest ancestor's. The walk returns
//! whether it reached the word's own node.

use flexion::Path;
use flexion::tables::{NodeKind, Smoothed, smooth_node};
use std::sync::OnceLock;

mod nodes;
mod t0;
mod t1;
mod t2;

pub(crate) use t0::n0 as root;

/// Smoothing strength, the tail cut, and whether probabilities are rounded to
/// 1/65535 (see `flexion::tables::smooth_node`).
const ALPHA: f64 = 0.5;
const EPSILON: f64 = 0.002;
const NARROW: bool = true;

/// A node with a distribution: how it comes from its counts, the node it is
/// smoothed toward (`u32::MAX`: none), and its counts `(class, weight)`.
pub(crate) struct Node {
    kind: NodeKind,
    parent: u32,
    counts: &'static [(u32, f32)],
}

impl Node {
    pub(crate) const fn root(counts: &'static [(u32, f32)]) -> Node {
        Node { kind: NodeKind::Root, parent: u32::MAX, counts }
    }
    pub(crate) const fn word(counts: &'static [(u32, f32)]) -> Node {
        Node { kind: NodeKind::Word, parent: u32::MAX, counts }
    }
    pub(crate) const fn inner(parent: u32, counts: &'static [(u32, f32)]) -> Node {
        Node { kind: NodeKind::Inner, parent, counts }
    }
}

/// Each node's distribution, computed on first use.
static CACHE: [OnceLock<Smoothed>; 6300] = [const { OnceLock::new() }; 6300];

fn dist(i: usize) -> Option<&'static Smoothed> {
    let node = nodes::get(i)?;
    Some(CACHE.get(i)?.get_or_init(|| {
        let parent = (node.parent != u32::MAX)
            .then(|| usize::try_from(node.parent).ok())
            .flatten()
            .and_then(dist);
        let q = parent.map_or(&[][..], |p| p.entries.as_slice());
        smooth_node(node.kind, node.counts, q, ALPHA, EPSILON, NARROW)
    }))
}

/// Add node `i`'s distribution to the path.
pub(crate) fn push(out: &mut Path<'_>, i: usize) {
    if let Some(d) = dist(i).filter(|d| !d.entries.is_empty()) {
        out.push(&d.entries, d.mass);
    }
}
