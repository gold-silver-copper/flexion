//! How the language is written: its vowels, and the marks it reads and
//! writes.

use flexion::Orthography;

pub(crate) const ORTHOGRAPHY: Orthography<'static> = Orthography {
    lowercase: true,
    vowels: "аеиоуыэюяё",
    stress_marks: "",
    stressed: &[],
    implicit: "",
    dropped: "\u{301}\u{300}",
    mark_monosyllables: false,
    folded: &[],
    prosody: None,
};
