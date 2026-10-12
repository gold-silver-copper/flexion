//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's feature says):
/// `predicative` (pos)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal;
     predicative: $c0:tt;
    ) => {
        Paradigm {
            name: ::std::borrow::Cow::Borrowed($name),
            words: $words,
            lemma: ::std::borrow::Cow::Borrowed($lemma),
            forms: ::std::borrow::Cow::Borrowed(&[flexion::form!($c0)]),
        }
    };
}

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `veraltet`: 14,401 words.
    Veraltet,
    /// Like `überfinanziert`: 358 words.
    Überfinanziert,
    /// Like `erkoren`: 35 words.
    Erkoren,
    /// Like `sagend`: 26 words.
    Sagend,
    /// Like `eingetragen`: 9 words.
    Eingetragen,
    /// Like `tuend`: 8 words.
    Tuend,
    /// Like `sichernd`: 3 words.
    Sichernd,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 7] = [
        Declension::Veraltet,
        Declension::Überfinanziert,
        Declension::Erkoren,
        Declension::Sagend,
        Declension::Eingetragen,
        Declension::Tuend,
        Declension::Sichernd,
    ];

    /// Its table.
    pub fn paradigm(self) -> Option<&'static Paradigm<'static>> {
        DECLENSIONS.get(self as usize)
    }
}

impl flexion::Label for Declension {
    fn index(self) -> usize {
        self as usize
    }
    fn from_index(index: usize) -> Option<Self> {
        Self::ALL.get(index).copied()
    }
}

pub(super) static DECLENSIONS: [Paradigm<'static>; 7] = [
    // Veraltet: 14,401 words (frei, synonym, abaxial, abdominal, aberrant, abnormal).
    declension!("veraltet", 14401, "~";
        predicative: "~";
    ),
    // Überfinanziert: 358 words (gelaufen, gegessen, getrunken, gesprochen, gekommen, geboren).
    declension!("überfinanziert", 358, "~";
        predicative: "ist ~|es ist ~|sind ~";
    ),
    // Erkoren: 35 words (denkend, neigend, nehmend, erwachend, diskutierend, diskutiert).
    declension!("erkoren", 35, "~~";
        predicative: "ist ~~|es ist ~~|sin~ d~";
    ),
    // Sagend: 26 words (seiend, springend, sollend, sagend, sehend, sperrend).
    declension!("sagend", 26, "~~";
        predicative: "i~t s~|e~ ist s~|~ind s~";
    ),
    // Eingetragen: 9 words (eingetragen, eingeladen, eingebaut, einleitend, eingemacht, einladend).
    declension!("eingetragen", 9, "~~~";
        predicative: "ist ~~~|~s ~st ei~|sind ~~~";
    ),
    // Tuend: 8 words (tuend, trinkend, tötend, teilnehmend, träumend, tobend).
    declension!("tuend", 8, "~~";
        predicative: "is~ t~|es is~ t~|sind ~~";
    ),
    // Sichernd: 3 words (sichernd, sicherstellend, sichergestellt).
    declension!("sichernd", 3, "~~~";
        predicative: "i~t s~~|e~ ~st si~|~~nd si~";
    ),
];
