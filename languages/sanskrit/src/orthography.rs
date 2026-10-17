//! How the language is written: its vowels, and the marks it reads and
//! writes.

use flexion::Orthography;

pub(crate) const ORTHOGRAPHY: Orthography<'static> = Orthography {
    lowercase: false,
    vowels: "अआइईउऊऋऌएऐओऔॠॡ",
    stress_marks: "",
    stressed: &[],
    implicit: "",
    dropped: "\u{951}\u{952}",
    mark_monosyllables: false,
    folded: &[],
    prosody: None,
    initial: "",
};
