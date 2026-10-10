//! The binary model format shared by `flexion-train` (writer) and this crate
//! (reader). Every multi-byte integer is little-endian `u32`. Every read is
//! bounds-checked: a truncated or corrupt blob makes [`crate::Model::from_bytes`]
//! return `None` or a lookup return nothing, never a panic.
//!
//! ```text
//! header (88 bytes)
//!   0  magic "FLX1"            4  version
//!   8  strings offset          12 strings length
//!   16 number of parts of speech
//!   20 spelling flags (bit 0: fold case, bit 1: mark stress on monosyllables)
//!   24 stress mark    32 vowels    40 implicitly stressed letters
//!   48 fold-from      56 fold-to   64 marks stripped from input
//!   72 unmark-from    80 unmark-to (letters written plain when marks are off)
//!      (each a string reference: offset + length into the string area)
//! part-of-speech headers, 60 bytes each, right after the header: cells,
//!   inherent features, then offset and count of edits, classes, nodes and
//!   distribution entries, the lemma cell, the walk's feature word
//!   (bits 0–7: 1 + the depth at which a syllable-count token is inserted,
//!   0 for none; bits 8–15: the largest syllable bucket), the offset of the
//!   first-character array, the offset of the class groups section, and the
//!   index of the part of speech that restores this one's lemmas ([`NONE`]:
//!   lemmas are taken as typed)
//! records (edits, classes, nodes, distribution entries) at the offsets the
//!   part-of-speech headers give
//! string area: UTF-8 bytes
//! ```
//!
//! Records:
//! - edit, 32 bytes: prefix added, suffix stripped, suffix added (three string
//!   references), stress kind (`u8`), stress index (`u8`), then (each `u8`,
//!   saturating) the stripped suffix's length in characters and the vowels
//!   in the prefix, the stripped suffix and the added suffix, two bytes
//!   padding;
//! - class groups section: the number of groups, each group's number of
//!   cells (they sum to the part of speech's cells), then per group the number
//!   of its sub-vectors followed by the sub-vectors themselves (one edit index
//!   per cell of the group, [`NONE`] when there is no form there). Classes
//!   share sub-vectors, which keeps big paradigms small;
//! - class, `4 * groups + 8 + inherent` bytes rounded up to 4: one sub-vector
//!   index per group, a string reference to the class's longest stripped
//!   suffix, one value index per inherent feature ([`UNKNOWN`] when unknown);
//! - node, 16 bytes: the label's offset in the string area, first child,
//!   first distribution entry (all `u32`), number of children, number of
//!   distribution entries (both `u16`);
//! - one `u32` per node, in node order: its label's first character (low 21
//!   bits) and the label's length in bytes (high 11 bits), so a child search
//!   touches one small dense array. Nodes are laid out depth-first by child
//!   block, so a walk's successive levels sit close together;
//! - distribution entry, 8 bytes: class index, probability (`f32` bits).
//!   Entries of one node are sorted by descending probability.

pub const MAGIC: &[u8; 4] = b"FLX1";
pub const VERSION: u32 = 4;
pub const HEADER_LEN: usize = 88;
pub const POS_HEADER_LEN: usize = 60;
pub const EDIT_LEN: usize = 32;
pub const NODE_LEN: usize = 16;
pub const DIST_LEN: usize = 8;
/// No edit (the class has no form in this cell), or no lemma cell.
pub const NONE: u32 = u32::MAX;
/// Unknown inherent-feature value.
pub const UNKNOWN: u8 = u8::MAX;
/// The symbol that ends every key in the tree: the start of the word, since
/// keys are walked from the last letter backwards.
pub const BOUNDARY: char = '\u{1}';

/// The first syllable-count token; bucket `k` is `char::from_u32(SYLLABLE_TOKEN + k)`.
pub const SYLLABLE_TOKEN: u32 = 0x10;
/// Bits of a first-character entry that hold the character.
pub const CHAR_BITS: u32 = 21;

pub const FLAG_FOLD_CASE: u32 = 1;
pub const FLAG_MARK_MONOSYLLABLES: u32 = 2;

/// Bytes a class record occupies.
pub const fn class_len(cells: usize, inherent: usize) -> usize {
    let raw = cells
        .saturating_mul(4)
        .saturating_add(8)
        .saturating_add(inherent);
    raw.saturating_add(3) & !3
}

/// How a form's stress is placed, relative to the edit that built it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Stress {
    /// No stress mark.
    Unmarked,
    /// On the same vowel as the lemma's stress.
    SameAsLemma,
    /// On the n-th vowel of the added suffix.
    Added(u8),
    /// On the n-th vowel of the kept stem, counted from its end (0 = last).
    KeptFromEnd(u8),
    /// On the n-th vowel of the kept stem, counted from its start.
    KeptFromStart(u8),
    /// On the n-th vowel of the added prefix.
    Prefix(u8),
}

impl Stress {
    pub const fn encode(self) -> (u8, u8) {
        match self {
            Stress::Unmarked => (0, 0),
            Stress::SameAsLemma => (1, 0),
            Stress::Added(i) => (2, i),
            Stress::KeptFromEnd(i) => (3, i),
            Stress::KeptFromStart(i) => (4, i),
            Stress::Prefix(i) => (5, i),
        }
    }

    pub const fn decode(kind: u8, index: u8) -> Stress {
        match kind {
            1 => Stress::SameAsLemma,
            2 => Stress::Added(index),
            3 => Stress::KeptFromEnd(index),
            4 => Stress::KeptFromStart(index),
            5 => Stress::Prefix(index),
            _ => Stress::Unmarked,
        }
    }
}

/// Bounds-checked little-endian reads.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Bytes<'a>(pub(crate) &'a [u8]);

impl<'a> Bytes<'a> {
    pub(crate) fn u32(self, at: usize) -> Option<u32> {
        let end = at.checked_add(4)?;
        let b = self.0.get(at..end)?;
        Some(u32::from_le_bytes([
            *b.first()?,
            *b.get(1)?,
            *b.get(2)?,
            *b.get(3)?,
        ]))
    }

    pub(crate) fn usize(self, at: usize) -> Option<usize> {
        usize::try_from(self.u32(at)?).ok()
    }

    pub(crate) fn u8(self, at: usize) -> Option<u8> {
        self.0.get(at).copied()
    }

    pub(crate) fn f32(self, at: usize) -> Option<f32> {
        Some(f32::from_bits(self.u32(at)?))
    }

    /// A string reference at `at`, resolved against the string area.
    pub(crate) fn str(self, strings: &'a str, at: usize) -> Option<&'a str> {
        let off = self.usize(at)?;
        let len = self.usize(at.checked_add(4)?)?;
        strings.get(off..off.checked_add(len)?)
    }
}
