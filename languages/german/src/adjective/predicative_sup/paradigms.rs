//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's feature says):
/// `superlative_strong_nominative` (m, f, n, pl)
/// `superlative_strong_genitive` (m, f, n, pl)
/// `superlative_strong_dative` (m, f, n, pl)
/// `superlative_strong_accusative` (m, f, n, pl)
/// `superlative_weak_nominative` (m, f, n, pl)
/// `superlative_weak_genitive` (m, f, n, pl)
/// `superlative_weak_dative` (m, f, n, pl)
/// `superlative_weak_accusative` (m, f, n, pl)
/// `superlative_mixed_nominative` (m, f, n, pl)
/// `superlative_mixed_genitive` (m, f, n, pl)
/// `superlative_mixed_dative` (m, f, n, pl)
/// `superlative_mixed_accusative` (m, f, n, pl)
/// `predicative` (sup)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal;
     superlative_strong_nominative: $c0:tt, $c1:tt, $c2:tt, $c3:tt;
     superlative_strong_genitive: $c4:tt, $c5:tt, $c6:tt, $c7:tt;
     superlative_strong_dative: $c8:tt, $c9:tt, $c10:tt, $c11:tt;
     superlative_strong_accusative: $c12:tt, $c13:tt, $c14:tt, $c15:tt;
     superlative_weak_nominative: $c16:tt, $c17:tt, $c18:tt, $c19:tt;
     superlative_weak_genitive: $c20:tt, $c21:tt, $c22:tt, $c23:tt;
     superlative_weak_dative: $c24:tt, $c25:tt, $c26:tt, $c27:tt;
     superlative_weak_accusative: $c28:tt, $c29:tt, $c30:tt, $c31:tt;
     superlative_mixed_nominative: $c32:tt, $c33:tt, $c34:tt, $c35:tt;
     superlative_mixed_genitive: $c36:tt, $c37:tt, $c38:tt, $c39:tt;
     superlative_mixed_dative: $c40:tt, $c41:tt, $c42:tt, $c43:tt;
     superlative_mixed_accusative: $c44:tt, $c45:tt, $c46:tt, $c47:tt;
     predicative: $c48:tt;
    ) => {
        Paradigm {
            name: ::std::borrow::Cow::Borrowed($name),
            words: $words,
            lemma: ::std::borrow::Cow::Borrowed($lemma),
            forms: ::std::borrow::Cow::Borrowed(&[
                flexion::form!($c0),
                flexion::form!($c1),
                flexion::form!($c2),
                flexion::form!($c3),
                flexion::form!($c4),
                flexion::form!($c5),
                flexion::form!($c6),
                flexion::form!($c7),
                flexion::form!($c8),
                flexion::form!($c9),
                flexion::form!($c10),
                flexion::form!($c11),
                flexion::form!($c12),
                flexion::form!($c13),
                flexion::form!($c14),
                flexion::form!($c15),
                flexion::form!($c16),
                flexion::form!($c17),
                flexion::form!($c18),
                flexion::form!($c19),
                flexion::form!($c20),
                flexion::form!($c21),
                flexion::form!($c22),
                flexion::form!($c23),
                flexion::form!($c24),
                flexion::form!($c25),
                flexion::form!($c26),
                flexion::form!($c27),
                flexion::form!($c28),
                flexion::form!($c29),
                flexion::form!($c30),
                flexion::form!($c31),
                flexion::form!($c32),
                flexion::form!($c33),
                flexion::form!($c34),
                flexion::form!($c35),
                flexion::form!($c36),
                flexion::form!($c37),
                flexion::form!($c38),
                flexion::form!($c39),
                flexion::form!($c40),
                flexion::form!($c41),
                flexion::form!($c42),
                flexion::form!($c43),
                flexion::form!($c44),
                flexion::form!($c45),
                flexion::form!($c46),
                flexion::form!($c47),
                flexion::form!($c48),
            ]),
        }
    };
}

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `am veraltetsten`: 4,898 words.
    Amveraltetsten,
    /// Like `am ausgelassensten`: 404 words.
    Amausgelassensten,
    /// Like `am muffigsten`: 233 words.
    Ammuffigsten,
    /// Like `am freiesten`: 74 words.
    Amfreiesten,
    /// Like `am stumpfsten`: 24 words.
    Amstumpfsten,
    /// Like `am amtlichsten`: 12 words.
    Amamtlichsten,
    /// Like `am kargsten`: 6 words.
    Amkargsten,
    /// Like `am rötesten`: 3 words.
    Amrötesten,
    /// Like `am mauesten`: 3 words.
    Ammauesten,
    /// Like `am aalglattesten`: 3 words.
    Amaalglattesten,
    /// Like `am besten besucht`: 2 words.
    Ambestenbesucht,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 11] = [
        Declension::Amveraltetsten,
        Declension::Amausgelassensten,
        Declension::Ammuffigsten,
        Declension::Amfreiesten,
        Declension::Amstumpfsten,
        Declension::Amamtlichsten,
        Declension::Amkargsten,
        Declension::Amrötesten,
        Declension::Ammauesten,
        Declension::Amaalglattesten,
        Declension::Ambestenbesucht,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 11] = [
    // Amveraltetsten: 4,898 words (am orangesten, am dicksten, am flatulentesten, am feistesten, am hellsten, am urbansten).
    declension!("am veraltetsten", 4898, "am ~n";
        superlative_strong_nominative: "~r", "~", "~s", "~";
        superlative_strong_genitive: "~n", "~r", "~n", "~r";
        superlative_strong_dative: "~m", "~r", "~m", "~n";
        superlative_strong_accusative: "~n", "~", "~s", "~";
        superlative_weak_nominative: "~", "~", "~", "~n";
        superlative_weak_genitive: "~n", "~n", "~n", "~n";
        superlative_weak_dative: "~n", "~n", "~n", "~n";
        superlative_weak_accusative: "~n", "~", "~", "~n";
        superlative_mixed_nominative: "~r", "~", "~s", "~n";
        superlative_mixed_genitive: "~n", "~n", "~n", "~n";
        superlative_mixed_dative: "~n", "~n", "~n", "~n";
        superlative_mixed_accusative: "~n", "~", "~s", "~n";
        predicative: "am ~n";
    ),
    // Amausgelassensten: 404 words (am aberrantesten, am abnormalsten, am abruptesten, am absurdesten, am abundantesten, am aridesten).
    declension!("am ausgelassensten", 404, "~m a~n";
        superlative_strong_nominative: "~~r", "~~", "~~s", "~~";
        superlative_strong_genitive: "~~n", "~~r", "~~n", "~~r";
        superlative_strong_dative: "~~m", "~~r", "~~m", "~~n";
        superlative_strong_accusative: "~~n", "~~", "~~s", "~~";
        superlative_weak_nominative: "~~", "~~", "~~", "~~n";
        superlative_weak_genitive: "~~n", "~~n", "~~n", "~~n";
        superlative_weak_dative: "~~n", "~~n", "~~n", "~~n";
        superlative_weak_accusative: "~~n", "~~", "~~", "~~n";
        superlative_mixed_nominative: "~~r", "~~", "~~s", "~~n";
        superlative_mixed_genitive: "~~n", "~~n", "~~n", "~~n";
        superlative_mixed_dative: "~~n", "~~n", "~~n", "~~n";
        superlative_mixed_accusative: "~~n", "~~", "~~s", "~~n";
        predicative: "~m a~n";
    ),
    // Ammuffigsten: 233 words (am mutigsten, am mobilsten, am manifestesten, am miesesten, am müdesten, am minimsten).
    declension!("am muffigsten", 233, "a~ m~n";
        superlative_strong_nominative: "~~r", "~~", "~~s", "~~";
        superlative_strong_genitive: "~~n", "~~r", "~~n", "~~r";
        superlative_strong_dative: "~~m", "~~r", "~~m", "~~n";
        superlative_strong_accusative: "~~n", "~~", "~~s", "~~";
        superlative_weak_nominative: "~~", "~~", "~~", "~~n";
        superlative_weak_genitive: "~~n", "~~n", "~~n", "~~n";
        superlative_weak_dative: "~~n", "~~n", "~~n", "~~n";
        superlative_weak_accusative: "~~n", "~~", "~~", "~~n";
        superlative_mixed_nominative: "~~r", "~~", "~~s", "~~n";
        superlative_mixed_genitive: "~~n", "~~n", "~~n", "~~n";
        superlative_mixed_dative: "~~n", "~~n", "~~n", "~~n";
        superlative_mixed_accusative: "~~n", "~~", "~~s", "~~n";
        predicative: "a~ m~n";
    ),
    // Amfreiesten: 74 words (am freiesten, am newesten, am linkesten, am blauesten, am neuesten, am fadesten).
    declension!("am freiesten", 74, "am ~e~n";
        superlative_strong_nominative: "~e~r|~~r", "~e~|~~", "~e~s|~~s", "~e~|~~";
        superlative_strong_genitive: "~e~n|~~n", "~e~r|~~r", "~e~n|~~n", "~e~r|~~r";
        superlative_strong_dative: "~e~m|~~m", "~e~r|~~r", "~e~m|~~m", "~e~n|~~n";
        superlative_strong_accusative: "~e~n|~~n", "~e~|~~", "~e~s|~~s", "~e~|~~";
        superlative_weak_nominative: "~e~|~~", "~e~|~~", "~e~|~~", "~e~n|~~n";
        superlative_weak_genitive: "~e~n|~~n", "~e~n|~~n", "~e~n|~~n", "~e~n|~~n";
        superlative_weak_dative: "~e~n|~~n", "~e~n|~~n", "~e~n|~~n", "~e~n|~~n";
        superlative_weak_accusative: "~e~n|~~n", "~e~|~~", "~e~|~~", "~e~n|~~n";
        superlative_mixed_nominative: "~e~r|~~r", "~e~|~~", "~e~s|~~s", "~e~n|~~n";
        superlative_mixed_genitive: "~e~n|~~n", "~e~n|~~n", "~e~n|~~n", "~e~n|~~n";
        superlative_mixed_dative: "~e~n|~~n", "~e~n|~~n", "~e~n|~~n", "~e~n|~~n";
        superlative_mixed_accusative: "~e~n|~~n", "~e~|~~", "~e~s|~~s", "~e~n|~~n";
        predicative: "am ~e~n|am ~~n";
    ),
    // Amstumpfsten: 24 words (am toughsten, am plumpsten, am knorksten, am rohsten, am zähsten, am schlanksten).
    declension!("am stumpfsten", 24, "am ~~n";
        superlative_strong_nominative: "~~r|~e~r", "~~|~e~", "~~s|~e~s", "~~|~e~";
        superlative_strong_genitive: "~~n|~e~n", "~~r|~e~r", "~~n|~e~n", "~~r|~e~r";
        superlative_strong_dative: "~~m|~e~m", "~~r|~e~r", "~~m|~e~m", "~~n|~e~n";
        superlative_strong_accusative: "~~n|~e~n", "~~|~e~", "~~s|~e~s", "~~|~e~";
        superlative_weak_nominative: "~~|~e~", "~~|~e~", "~~|~e~", "~~n|~e~n";
        superlative_weak_genitive: "~~n|~e~n", "~~n|~e~n", "~~n|~e~n", "~~n|~e~n";
        superlative_weak_dative: "~~n|~e~n", "~~n|~e~n", "~~n|~e~n", "~~n|~e~n";
        superlative_weak_accusative: "~~n|~e~n", "~~|~e~", "~~|~e~", "~~n|~e~n";
        superlative_mixed_nominative: "~~r|~e~r", "~~|~e~", "~~s|~e~s", "~~n|~e~n";
        superlative_mixed_genitive: "~~n|~e~n", "~~n|~e~n", "~~n|~e~n", "~~n|~e~n";
        superlative_mixed_dative: "~~n|~e~n", "~~n|~e~n", "~~n|~e~n", "~~n|~e~n";
        superlative_mixed_accusative: "~~n|~e~n", "~~|~e~", "~~s|~e~s", "~~n|~e~n";
        predicative: "am ~~n|am ~e~n";
    ),
    // Amamtlichsten: 12 words (am ambivalentesten, am amyloidesten, am amerikanischsten, am ambitiösesten, am amüsantesten, am ambitioniertesten).
    declension!("am amtlichsten", 12, "~ am~n";
        superlative_strong_nominative: "~~r", "~~", "~~s", "~~";
        superlative_strong_genitive: "~~n", "~~r", "~~n", "~~r";
        superlative_strong_dative: "~~m", "~~r", "~~m", "~~n";
        superlative_strong_accusative: "~~n", "~~", "~~s", "~~";
        superlative_weak_nominative: "~~", "~~", "~~", "~~n";
        superlative_weak_genitive: "~~n", "~~n", "~~n", "~~n";
        superlative_weak_dative: "~~n", "~~n", "~~n", "~~n";
        superlative_weak_accusative: "~~n", "~~", "~~", "~~n";
        superlative_mixed_nominative: "~~r", "~~", "~~s", "~~n";
        superlative_mixed_genitive: "~~n", "~~n", "~~n", "~~n";
        superlative_mixed_dative: "~~n", "~~n", "~~n", "~~n";
        superlative_mixed_accusative: "~~n", "~~", "~~s", "~~n";
        predicative: "~ am~n";
    ),
    // Amkargsten: 6 words (am bangsten, am nassesten, am schmalsten, am blassesten, am kargsten, am geraumsten).
    declension!("am kargsten", 6, "am ~a~n";
        superlative_strong_nominative: "~a~r|~ä~r", "~a~|~ä~", "~a~s|~ä~s", "~a~|~ä~";
        superlative_strong_genitive: "~a~n|~ä~n", "~a~r|~ä~r", "~a~n|~ä~n", "~a~r|~ä~r";
        superlative_strong_dative: "~a~m|~ä~m", "~a~r|~ä~r", "~a~m|~ä~m", "~a~n|~ä~n";
        superlative_strong_accusative: "~a~n|~ä~n", "~a~|~ä~", "~a~s|~ä~s", "~a~|~ä~";
        superlative_weak_nominative: "~a~|~ä~", "~a~|~ä~", "~a~|~ä~", "~a~n|~ä~n";
        superlative_weak_genitive: "~a~n|~ä~n", "~a~n|~ä~n", "~a~n|~ä~n", "~a~n|~ä~n";
        superlative_weak_dative: "~a~n|~ä~n", "~a~n|~ä~n", "~a~n|~ä~n", "~a~n|~ä~n";
        superlative_weak_accusative: "~a~n|~ä~n", "~a~|~ä~", "~a~|~ä~", "~a~n|~ä~n";
        superlative_mixed_nominative: "~a~r|~ä~r", "~a~|~ä~", "~a~s|~ä~s", "~a~n|~ä~n";
        superlative_mixed_genitive: "~a~n|~ä~n", "~a~n|~ä~n", "~a~n|~ä~n", "~a~n|~ä~n";
        superlative_mixed_dative: "~a~n|~ä~n", "~a~n|~ä~n", "~a~n|~ä~n", "~a~n|~ä~n";
        superlative_mixed_accusative: "~a~n|~ä~n", "~a~|~ä~", "~a~s|~ä~s", "~a~n|~ä~n";
        predicative: "am ~a~n|am ~ä~n";
    ),
    // Amrötesten: 3 words (am rötesten, am frömmsten, am röthesten).
    declension!("am rötesten", 3, "am ~ö~n";
        superlative_strong_nominative: "~ö~r|~o~r", "~ö~|~o~", "~ö~s|~o~s", "~ö~|~o~";
        superlative_strong_genitive: "~ö~n|~o~n", "~ö~r|~o~r", "~ö~n|~o~n", "~ö~r|~o~r";
        superlative_strong_dative: "~ö~m|~o~m", "~ö~r|~o~r", "~ö~m|~o~m", "~ö~n|~o~n";
        superlative_strong_accusative: "~ö~n|~o~n", "~ö~|~o~", "~ö~s|~o~s", "~ö~|~o~";
        superlative_weak_nominative: "~ö~|~o~", "~ö~|~o~", "~ö~|~o~", "~ö~n|~o~n";
        superlative_weak_genitive: "~ö~n|~o~n", "~ö~n|~o~n", "~ö~n|~o~n", "~ö~n|~o~n";
        superlative_weak_dative: "~ö~n|~o~n", "~ö~n|~o~n", "~ö~n|~o~n", "~ö~n|~o~n";
        superlative_weak_accusative: "~ö~n|~o~n", "~ö~|~o~", "~ö~|~o~", "~ö~n|~o~n";
        superlative_mixed_nominative: "~ö~r|~o~r", "~ö~|~o~", "~ö~s|~o~s", "~ö~n|~o~n";
        superlative_mixed_genitive: "~ö~n|~o~n", "~ö~n|~o~n", "~ö~n|~o~n", "~ö~n|~o~n";
        superlative_mixed_dative: "~ö~n|~o~n", "~ö~n|~o~n", "~ö~n|~o~n", "~ö~n|~o~n";
        superlative_mixed_accusative: "~ö~n|~o~n", "~ö~|~o~", "~ö~s|~o~s", "~ö~n|~o~n";
        predicative: "am ~ö~n|am ~o~n";
    ),
    // Ammauesten: 3 words (am mauesten, am morschesten, am medienscheuesten).
    declension!("am mauesten", 3, "a~ m~e~n";
        superlative_strong_nominative: "~~e~r|~~~r", "~~e~|~~~", "~~e~s|~~~s", "~~e~|~~~";
        superlative_strong_genitive: "~~e~n|~~~n", "~~e~r|~~~r", "~~e~n|~~~n", "~~e~r|~~~r";
        superlative_strong_dative: "~~e~m|~~~m", "~~e~r|~~~r", "~~e~m|~~~m", "~~e~n|~~~n";
        superlative_strong_accusative: "~~e~n|~~~n", "~~e~|~~~", "~~e~s|~~~s", "~~e~|~~~";
        superlative_weak_nominative: "~~e~|~~~", "~~e~|~~~", "~~e~|~~~", "~~e~n|~~~n";
        superlative_weak_genitive: "~~e~n|~~~n", "~~e~n|~~~n", "~~e~n|~~~n", "~~e~n|~~~n";
        superlative_weak_dative: "~~e~n|~~~n", "~~e~n|~~~n", "~~e~n|~~~n", "~~e~n|~~~n";
        superlative_weak_accusative: "~~e~n|~~~n", "~~e~|~~~", "~~e~|~~~", "~~e~n|~~~n";
        superlative_mixed_nominative: "~~e~r|~~~r", "~~e~|~~~", "~~e~s|~~~s", "~~e~n|~~~n";
        superlative_mixed_genitive: "~~e~n|~~~n", "~~e~n|~~~n", "~~e~n|~~~n", "~~e~n|~~~n";
        superlative_mixed_dative: "~~e~n|~~~n", "~~e~n|~~~n", "~~e~n|~~~n", "~~e~n|~~~n";
        superlative_mixed_accusative: "~~e~n|~~~n", "~~e~|~~~", "~~e~s|~~~s", "~~e~n|~~~n";
        predicative: "a~ m~e~n|a~ m~~n";
    ),
    // Amaalglattesten: 3 words (am aalglattesten, am aasigsten, am aargauischsten).
    declension!("am aalglattesten", 3, "~m ~a~n";
        superlative_strong_nominative: "~~~r", "~~~", "~~~s", "~~~";
        superlative_strong_genitive: "~~~n", "~~~r", "~~~n", "~~~r";
        superlative_strong_dative: "~~~m", "~~~r", "~~~m", "~~~n";
        superlative_strong_accusative: "~~~n", "~~~", "~~~s", "~~~";
        superlative_weak_nominative: "~~~", "~~~", "~~~", "~~~n";
        superlative_weak_genitive: "~~~n", "~~~n", "~~~n", "~~~n";
        superlative_weak_dative: "~~~n", "~~~n", "~~~n", "~~~n";
        superlative_weak_accusative: "~~~n", "~~~", "~~~", "~~~n";
        superlative_mixed_nominative: "~~~r", "~~~", "~~~s", "~~~n";
        superlative_mixed_genitive: "~~~n", "~~~n", "~~~n", "~~~n";
        superlative_mixed_dative: "~~~n", "~~~n", "~~~n", "~~~n";
        superlative_mixed_accusative: "~~~n", "~~~", "~~~s", "~~~n";
        predicative: "~m ~a~n";
    ),
    // Ambestenbesucht: 2 words (am besten besucht, am besten gelaunt).
    declension!("am besten besucht", 2, "am ~en ~";
        superlative_strong_nominative: "~~er", "~~e", "~~es", "~~e";
        superlative_strong_genitive: "~~en", "~~er", "~~en", "~~er";
        superlative_strong_dative: "~~em", "~~er", "~~em", "~~en";
        superlative_strong_accusative: "~~en", "~~e", "~~es", "~~e";
        superlative_weak_nominative: "~~e", "~~e", "~~e", "~~en";
        superlative_weak_genitive: "~~en", "~~en", "~~en", "~~en";
        superlative_weak_dative: "~~en", "~~en", "~~en", "~~en";
        superlative_weak_accusative: "~~en", "~~e", "~~e", "~~en";
        superlative_mixed_nominative: "~~er", "~~e", "~~es", "~~en";
        superlative_mixed_genitive: "~~en", "~~en", "~~en", "~~en";
        superlative_mixed_dative: "~~en", "~~en", "~~en", "~~en";
        superlative_mixed_accusative: "~~en", "~~e", "~~es", "~~en";
        predicative: "am ~en ~";
    ),
];
