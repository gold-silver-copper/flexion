//! How the language is written: its vowels, and the marks it reads and
//! writes.

use flexion::Orthography;

pub(crate) const ORTHOGRAPHY: Orthography<'static> = Orthography {
    lowercase: true,
    vowels: "aeiouyóąę",
    stress_marks: "",
    stressed: &[],
    implicit: "",
    dropped: "",
    mark_monosyllables: false,
    folded: &[],
    prosody: None,
};
