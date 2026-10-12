//! How the language is written: its vowels, and the marks it reads and
//! writes.

use flexion::Orthography;

pub(crate) const ORTHOGRAPHY: Orthography<'static> = Orthography {
    lowercase: true,
    vowels: "aeiouyāēīōūȳ",
    stress_marks: "",
    stressed: &[],
    implicit: "",
    dropped: "\u{306}\u{304}",
    mark_monosyllables: false,
    folded: &[
        ('j', 'i'),
        ('ā', 'a'),
        ('ē', 'e'),
        ('ī', 'i'),
        ('ō', 'o'),
        ('ū', 'u'),
        ('ȳ', 'y'),
    ],
    prosody: None,
};
