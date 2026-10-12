//! Words whose spelling their key doesn't show, as the dictionary writes
//! them: a caller who types a word without its marks means this. Marks a
//! conjugation writes itself aren't listed.

use std::borrow::Cow;

/// 0 words.
pub(super) static SPELLINGS: &[(Cow<'static, str>, Cow<'static, str>)] = flexion::spellings! {};
