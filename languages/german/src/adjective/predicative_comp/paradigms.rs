//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's feature says):
/// `strong_nominative` (m, f, n, pl)
/// `strong_genitive` (m, f, n, pl)
/// `strong_dative` (m, f, n, pl)
/// `strong_accusative` (m, f, n, pl)
/// `weak_nominative` (m, f, n, pl)
/// `weak_genitive` (m, f, n, pl)
/// `weak_dative` (m, f, n, pl)
/// `weak_accusative` (m, f, n, pl)
/// `mixed_nominative` (m, f, n, pl)
/// `mixed_genitive` (m, f, n, pl)
/// `mixed_dative` (m, f, n, pl)
/// `mixed_accusative` (m, f, n, pl)
/// `comparative_strong_nominative` (m, f, n, pl)
/// `comparative_strong_genitive` (m, f, n, pl)
/// `comparative_strong_dative` (m, f, n, pl)
/// `comparative_strong_accusative` (m, f, n, pl)
/// `comparative_weak_nominative` (m, f, n, pl)
/// `comparative_weak_genitive` (m, f, n, pl)
/// `comparative_weak_dative` (m, f, n, pl)
/// `comparative_weak_accusative` (m, f, n, pl)
/// `comparative_mixed_nominative` (m, f, n, pl)
/// `comparative_mixed_genitive` (m, f, n, pl)
/// `comparative_mixed_dative` (m, f, n, pl)
/// `comparative_mixed_accusative` (m, f, n, pl)
/// `predicative` (comp)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal;
     strong_nominative: $c0:tt, $c1:tt, $c2:tt, $c3:tt;
     strong_genitive: $c4:tt, $c5:tt, $c6:tt, $c7:tt;
     strong_dative: $c8:tt, $c9:tt, $c10:tt, $c11:tt;
     strong_accusative: $c12:tt, $c13:tt, $c14:tt, $c15:tt;
     weak_nominative: $c16:tt, $c17:tt, $c18:tt, $c19:tt;
     weak_genitive: $c20:tt, $c21:tt, $c22:tt, $c23:tt;
     weak_dative: $c24:tt, $c25:tt, $c26:tt, $c27:tt;
     weak_accusative: $c28:tt, $c29:tt, $c30:tt, $c31:tt;
     mixed_nominative: $c32:tt, $c33:tt, $c34:tt, $c35:tt;
     mixed_genitive: $c36:tt, $c37:tt, $c38:tt, $c39:tt;
     mixed_dative: $c40:tt, $c41:tt, $c42:tt, $c43:tt;
     mixed_accusative: $c44:tt, $c45:tt, $c46:tt, $c47:tt;
     comparative_strong_nominative: $c48:tt, $c49:tt, $c50:tt, $c51:tt;
     comparative_strong_genitive: $c52:tt, $c53:tt, $c54:tt, $c55:tt;
     comparative_strong_dative: $c56:tt, $c57:tt, $c58:tt, $c59:tt;
     comparative_strong_accusative: $c60:tt, $c61:tt, $c62:tt, $c63:tt;
     comparative_weak_nominative: $c64:tt, $c65:tt, $c66:tt, $c67:tt;
     comparative_weak_genitive: $c68:tt, $c69:tt, $c70:tt, $c71:tt;
     comparative_weak_dative: $c72:tt, $c73:tt, $c74:tt, $c75:tt;
     comparative_weak_accusative: $c76:tt, $c77:tt, $c78:tt, $c79:tt;
     comparative_mixed_nominative: $c80:tt, $c81:tt, $c82:tt, $c83:tt;
     comparative_mixed_genitive: $c84:tt, $c85:tt, $c86:tt, $c87:tt;
     comparative_mixed_dative: $c88:tt, $c89:tt, $c90:tt, $c91:tt;
     comparative_mixed_accusative: $c92:tt, $c93:tt, $c94:tt, $c95:tt;
     predicative: $c96:tt;
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
                flexion::form!($c49),
                flexion::form!($c50),
                flexion::form!($c51),
                flexion::form!($c52),
                flexion::form!($c53),
                flexion::form!($c54),
                flexion::form!($c55),
                flexion::form!($c56),
                flexion::form!($c57),
                flexion::form!($c58),
                flexion::form!($c59),
                flexion::form!($c60),
                flexion::form!($c61),
                flexion::form!($c62),
                flexion::form!($c63),
                flexion::form!($c64),
                flexion::form!($c65),
                flexion::form!($c66),
                flexion::form!($c67),
                flexion::form!($c68),
                flexion::form!($c69),
                flexion::form!($c70),
                flexion::form!($c71),
                flexion::form!($c72),
                flexion::form!($c73),
                flexion::form!($c74),
                flexion::form!($c75),
                flexion::form!($c76),
                flexion::form!($c77),
                flexion::form!($c78),
                flexion::form!($c79),
                flexion::form!($c80),
                flexion::form!($c81),
                flexion::form!($c82),
                flexion::form!($c83),
                flexion::form!($c84),
                flexion::form!($c85),
                flexion::form!($c86),
                flexion::form!($c87),
                flexion::form!($c88),
                flexion::form!($c89),
                flexion::form!($c90),
                flexion::form!($c91),
                flexion::form!($c92),
                flexion::form!($c93),
                flexion::form!($c94),
                flexion::form!($c95),
                flexion::form!($c96),
            ]),
        }
    };
}

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `veralteter`: 5,534 words.
    Veralteter,
    /// Like `schärfer`: 45 words.
    Schärfer,
    /// Like `unsicherer`: 39 words.
    Unsicherer,
    /// Like `röter`: 11 words.
    Röter,
    /// Like `kürzer`: 11 words.
    Kürzer,
    /// Like `karger`: 8 words.
    Karger,
    /// Like `integrer`: 6 words.
    Integrer,
    /// Like `unedler`: 4 words.
    Unedler,
    /// Like `ärger`: 3 words.
    Ärger,
    /// Like `ist ausgeruhter`: 3 words.
    Istausgeruhter,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 10] = [
        Declension::Veralteter,
        Declension::Schärfer,
        Declension::Unsicherer,
        Declension::Röter,
        Declension::Kürzer,
        Declension::Karger,
        Declension::Integrer,
        Declension::Unedler,
        Declension::Ärger,
        Declension::Istausgeruhter,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 10] = [
    // Veralteter: 5,534 words (freier, aberranter, abnormaler, abrupter, oranger, dicker).
    declension!("veralteter", 5534, "~r";
        strong_nominative: "~r", "~", "~s", "~";
        strong_genitive: "~n", "~r", "~n", "~r";
        strong_dative: "~m", "~r", "~m", "~n";
        strong_accusative: "~n", "~", "~s", "~";
        weak_nominative: "~", "~", "~", "~n";
        weak_genitive: "~n", "~n", "~n", "~n";
        weak_dative: "~n", "~n", "~n", "~n";
        weak_accusative: "~n", "~", "~", "~n";
        mixed_nominative: "~r", "~", "~s", "~n";
        mixed_genitive: "~n", "~n", "~n", "~n";
        mixed_dative: "~n", "~n", "~n", "~n";
        mixed_accusative: "~n", "~", "~s", "~n";
        comparative_strong_nominative: "~rer", "~re", "~res", "~re";
        comparative_strong_genitive: "~ren", "~rer", "~ren", "~rer";
        comparative_strong_dative: "~rem", "~rer", "~rem", "~ren";
        comparative_strong_accusative: "~ren", "~re", "~res", "~re";
        comparative_weak_nominative: "~re", "~re", "~re", "~ren";
        comparative_weak_genitive: "~ren", "~ren", "~ren", "~ren";
        comparative_weak_dative: "~ren", "~ren", "~ren", "~ren";
        comparative_weak_accusative: "~ren", "~re", "~re", "~ren";
        comparative_mixed_nominative: "~rer", "~re", "~res", "~ren";
        comparative_mixed_genitive: "~ren", "~ren", "~ren", "~ren";
        comparative_mixed_dative: "~ren", "~ren", "~ren", "~ren";
        comparative_mixed_accusative: "~ren", "~re", "~res", "~ren";
        predicative: "~r";
    ),
    // Schärfer: 45 words (wärmer, härter, näher, kälter, schwärzer, stärker).
    declension!("schärfer", 45, "~ä~r";
        strong_nominative: "~a~r", "~a~", "~a~s", "~a~";
        strong_genitive: "~a~n", "~a~r", "~a~n", "~a~r";
        strong_dative: "~a~m", "~a~r", "~a~m", "~a~n";
        strong_accusative: "~a~n", "~a~", "~a~s", "~a~";
        weak_nominative: "~a~", "~a~", "~a~", "~a~n";
        weak_genitive: "~a~n", "~a~n", "~a~n", "~a~n";
        weak_dative: "~a~n", "~a~n", "~a~n", "~a~n";
        weak_accusative: "~a~n", "~a~", "~a~", "~a~n";
        mixed_nominative: "~a~r", "~a~", "~a~s", "~a~n";
        mixed_genitive: "~a~n", "~a~n", "~a~n", "~a~n";
        mixed_dative: "~a~n", "~a~n", "~a~n", "~a~n";
        mixed_accusative: "~a~n", "~a~", "~a~s", "~a~n";
        comparative_strong_nominative: "~ä~rer", "~ä~re", "~ä~res", "~ä~re";
        comparative_strong_genitive: "~ä~ren", "~ä~rer", "~ä~ren", "~ä~rer";
        comparative_strong_dative: "~ä~rem", "~ä~rer", "~ä~rem", "~ä~ren";
        comparative_strong_accusative: "~ä~ren", "~ä~re", "~ä~res", "~ä~re";
        comparative_weak_nominative: "~ä~re", "~ä~re", "~ä~re", "~ä~ren";
        comparative_weak_genitive: "~ä~ren", "~ä~ren", "~ä~ren", "~ä~ren";
        comparative_weak_dative: "~ä~ren", "~ä~ren", "~ä~ren", "~ä~ren";
        comparative_weak_accusative: "~ä~ren", "~ä~re", "~ä~re", "~ä~ren";
        comparative_mixed_nominative: "~ä~rer", "~ä~re", "~ä~res", "~ä~ren";
        comparative_mixed_genitive: "~ä~ren", "~ä~ren", "~ä~ren", "~ä~ren";
        comparative_mixed_dative: "~ä~ren", "~ä~ren", "~ä~ren", "~ä~ren";
        comparative_mixed_accusative: "~ä~ren", "~ä~re", "~ä~res", "~ä~ren";
        predicative: "~ä~r";
    ),
    // Unsicherer: 39 words (sicherer, aperer, willkommener, cleverer, verschiedener, ebener).
    declension!("unsicherer", 39, "~e~r";
        strong_nominative: "~e~r", "~e~", "~e~s", "~e~";
        strong_genitive: "~e~n", "~e~r", "~e~n", "~e~r";
        strong_dative: "~e~m", "~e~r", "~e~m", "~e~n";
        strong_accusative: "~e~n", "~e~", "~e~s", "~e~";
        weak_nominative: "~e~", "~e~", "~e~", "~e~n";
        weak_genitive: "~e~n", "~e~n", "~e~n", "~e~n";
        weak_dative: "~e~n", "~e~n", "~e~n", "~e~n";
        weak_accusative: "~e~n", "~e~", "~e~", "~e~n";
        mixed_nominative: "~e~r", "~e~", "~e~s", "~e~n";
        mixed_genitive: "~e~n", "~e~n", "~e~n", "~e~n";
        mixed_dative: "~e~n", "~e~n", "~e~n", "~e~n";
        mixed_accusative: "~e~n", "~e~", "~e~s", "~e~n";
        comparative_strong_nominative: "~e~rer|~~rer", "~e~re|~~re", "~e~res|~~res", "~e~re|~~re";
        comparative_strong_genitive: "~e~ren|~~ren", "~e~rer|~~rer", "~e~ren|~~ren", "~e~rer|~~rer";
        comparative_strong_dative: "~e~rem|~~rem", "~e~rer|~~rer", "~e~rem|~~rem", "~e~ren|~~ren";
        comparative_strong_accusative: "~e~ren|~~ren", "~e~re|~~re", "~e~res|~~res", "~e~re|~~re";
        comparative_weak_nominative: "~e~re|~~re", "~e~re|~~re", "~e~re|~~re", "~e~ren|~~ren";
        comparative_weak_genitive: "~e~ren|~~ren", "~e~ren|~~ren", "~e~ren|~~ren", "~e~ren|~~ren";
        comparative_weak_dative: "~e~ren|~~ren", "~e~ren|~~ren", "~e~ren|~~ren", "~e~ren|~~ren";
        comparative_weak_accusative: "~e~ren|~~ren", "~e~re|~~re", "~e~re|~~re", "~e~ren|~~ren";
        comparative_mixed_nominative: "~e~rer|~~rer", "~e~re|~~re", "~e~res|~~res", "~e~ren|~~ren";
        comparative_mixed_genitive: "~e~ren|~~ren", "~e~ren|~~ren", "~e~ren|~~ren", "~e~ren|~~ren";
        comparative_mixed_dative: "~e~ren|~~ren", "~e~ren|~~ren", "~e~ren|~~ren", "~e~ren|~~ren";
        comparative_mixed_accusative: "~e~ren|~~ren", "~e~re|~~re", "~e~res|~~res", "~e~ren|~~ren";
        predicative: "~e~r|~~r";
    ),
    // Röter: 11 words (röter, größer, grösser, höher, gröber, frömmer).
    declension!("röter", 11, "~ö~r";
        strong_nominative: "~o~r", "~o~", "~o~s", "~o~";
        strong_genitive: "~o~n", "~o~r", "~o~n", "~o~r";
        strong_dative: "~o~m", "~o~r", "~o~m", "~o~n";
        strong_accusative: "~o~n", "~o~", "~o~s", "~o~";
        weak_nominative: "~o~", "~o~", "~o~", "~o~n";
        weak_genitive: "~o~n", "~o~n", "~o~n", "~o~n";
        weak_dative: "~o~n", "~o~n", "~o~n", "~o~n";
        weak_accusative: "~o~n", "~o~", "~o~", "~o~n";
        mixed_nominative: "~o~r", "~o~", "~o~s", "~o~n";
        mixed_genitive: "~o~n", "~o~n", "~o~n", "~o~n";
        mixed_dative: "~o~n", "~o~n", "~o~n", "~o~n";
        mixed_accusative: "~o~n", "~o~", "~o~s", "~o~n";
        comparative_strong_nominative: "~ö~rer", "~ö~re", "~ö~res", "~ö~re";
        comparative_strong_genitive: "~ö~ren", "~ö~rer", "~ö~ren", "~ö~rer";
        comparative_strong_dative: "~ö~rem", "~ö~rer", "~ö~rem", "~ö~ren";
        comparative_strong_accusative: "~ö~ren", "~ö~re", "~ö~res", "~ö~re";
        comparative_weak_nominative: "~ö~re", "~ö~re", "~ö~re", "~ö~ren";
        comparative_weak_genitive: "~ö~ren", "~ö~ren", "~ö~ren", "~ö~ren";
        comparative_weak_dative: "~ö~ren", "~ö~ren", "~ö~ren", "~ö~ren";
        comparative_weak_accusative: "~ö~ren", "~ö~re", "~ö~re", "~ö~ren";
        comparative_mixed_nominative: "~ö~rer", "~ö~re", "~ö~res", "~ö~ren";
        comparative_mixed_genitive: "~ö~ren", "~ö~ren", "~ö~ren", "~ö~ren";
        comparative_mixed_dative: "~ö~ren", "~ö~ren", "~ö~ren", "~ö~ren";
        comparative_mixed_accusative: "~ö~ren", "~ö~re", "~ö~res", "~ö~ren";
        predicative: "~ö~r";
    ),
    // Kürzer: 11 words (kürzer, dümmer, jünger, gesünder, klüger, unklüger).
    declension!("kürzer", 11, "~ü~r";
        strong_nominative: "~u~r", "~u~", "~u~s", "~u~";
        strong_genitive: "~u~n", "~u~r", "~u~n", "~u~r";
        strong_dative: "~u~m", "~u~r", "~u~m", "~u~n";
        strong_accusative: "~u~n", "~u~", "~u~s", "~u~";
        weak_nominative: "~u~", "~u~", "~u~", "~u~n";
        weak_genitive: "~u~n", "~u~n", "~u~n", "~u~n";
        weak_dative: "~u~n", "~u~n", "~u~n", "~u~n";
        weak_accusative: "~u~n", "~u~", "~u~", "~u~n";
        mixed_nominative: "~u~r", "~u~", "~u~s", "~u~n";
        mixed_genitive: "~u~n", "~u~n", "~u~n", "~u~n";
        mixed_dative: "~u~n", "~u~n", "~u~n", "~u~n";
        mixed_accusative: "~u~n", "~u~", "~u~s", "~u~n";
        comparative_strong_nominative: "~ü~rer", "~ü~re", "~ü~res", "~ü~re";
        comparative_strong_genitive: "~ü~ren", "~ü~rer", "~ü~ren", "~ü~rer";
        comparative_strong_dative: "~ü~rem", "~ü~rer", "~ü~rem", "~ü~ren";
        comparative_strong_accusative: "~ü~ren", "~ü~re", "~ü~res", "~ü~re";
        comparative_weak_nominative: "~ü~re", "~ü~re", "~ü~re", "~ü~ren";
        comparative_weak_genitive: "~ü~ren", "~ü~ren", "~ü~ren", "~ü~ren";
        comparative_weak_dative: "~ü~ren", "~ü~ren", "~ü~ren", "~ü~ren";
        comparative_weak_accusative: "~ü~ren", "~ü~re", "~ü~re", "~ü~ren";
        comparative_mixed_nominative: "~ü~rer", "~ü~re", "~ü~res", "~ü~ren";
        comparative_mixed_genitive: "~ü~ren", "~ü~ren", "~ü~ren", "~ü~ren";
        comparative_mixed_dative: "~ü~ren", "~ü~ren", "~ü~ren", "~ü~ren";
        comparative_mixed_accusative: "~ü~ren", "~ü~re", "~ü~res", "~ü~ren";
        predicative: "~ü~r";
    ),
    // Karger: 8 words (banger, nasser, schmaler, blasser, karger, geraumer).
    declension!("karger", 8, "~a~r";
        strong_nominative: "~a~r", "~a~", "~a~s", "~a~";
        strong_genitive: "~a~n", "~a~r", "~a~n", "~a~r";
        strong_dative: "~a~m", "~a~r", "~a~m", "~a~n";
        strong_accusative: "~a~n", "~a~", "~a~s", "~a~";
        weak_nominative: "~a~", "~a~", "~a~", "~a~n";
        weak_genitive: "~a~n", "~a~n", "~a~n", "~a~n";
        weak_dative: "~a~n", "~a~n", "~a~n", "~a~n";
        weak_accusative: "~a~n", "~a~", "~a~", "~a~n";
        mixed_nominative: "~a~r", "~a~", "~a~s", "~a~n";
        mixed_genitive: "~a~n", "~a~n", "~a~n", "~a~n";
        mixed_dative: "~a~n", "~a~n", "~a~n", "~a~n";
        mixed_accusative: "~a~n", "~a~", "~a~s", "~a~n";
        comparative_strong_nominative: "~a~rer|~ä~rer", "~a~re|~ä~re", "~a~res|~ä~res", "~a~re|~ä~re";
        comparative_strong_genitive: "~a~ren|~ä~ren", "~a~rer|~ä~rer", "~a~ren|~ä~ren", "~a~rer|~ä~rer";
        comparative_strong_dative: "~a~rem|~ä~rem", "~a~rer|~ä~rer", "~a~rem|~ä~rem", "~a~ren|~ä~ren";
        comparative_strong_accusative: "~a~ren|~ä~ren", "~a~re|~ä~re", "~a~res|~ä~res", "~a~re|~ä~re";
        comparative_weak_nominative: "~a~re|~ä~re", "~a~re|~ä~re", "~a~re|~ä~re", "~a~ren|~ä~ren";
        comparative_weak_genitive: "~a~ren|~ä~ren", "~a~ren|~ä~ren", "~a~ren|~ä~ren", "~a~ren|~ä~ren";
        comparative_weak_dative: "~a~ren|~ä~ren", "~a~ren|~ä~ren", "~a~ren|~ä~ren", "~a~ren|~ä~ren";
        comparative_weak_accusative: "~a~ren|~ä~ren", "~a~re|~ä~re", "~a~re|~ä~re", "~a~ren|~ä~ren";
        comparative_mixed_nominative: "~a~rer|~ä~rer", "~a~re|~ä~re", "~a~res|~ä~res", "~a~ren|~ä~ren";
        comparative_mixed_genitive: "~a~ren|~ä~ren", "~a~ren|~ä~ren", "~a~ren|~ä~ren", "~a~ren|~ä~ren";
        comparative_mixed_dative: "~a~ren|~ä~ren", "~a~ren|~ä~ren", "~a~ren|~ä~ren", "~a~ren|~ä~ren";
        comparative_mixed_accusative: "~a~ren|~ä~ren", "~a~re|~ä~re", "~a~res|~ä~res", "~a~ren|~ä~ren";
        predicative: "~a~r|~ä~r";
    ),
    // Integrer: 6 words (integrer, illustrer, makabrer, besser besucht, besser erhalten, besser gelaunt).
    declension!("integrer", 6, "~r~";
        strong_nominative: "~~er", "~~e", "~~es", "~~e";
        strong_genitive: "~~en", "~~er", "~~en", "~~er";
        strong_dative: "~~em", "~~er", "~~em", "~~en";
        strong_accusative: "~~en", "~~e", "~~es", "~~e";
        weak_nominative: "~~e", "~~e", "~~e", "~~en";
        weak_genitive: "~~en", "~~en", "~~en", "~~en";
        weak_dative: "~~en", "~~en", "~~en", "~~en";
        weak_accusative: "~~en", "~~e", "~~e", "~~en";
        mixed_nominative: "~~er", "~~e", "~~es", "~~en";
        mixed_genitive: "~~en", "~~en", "~~en", "~~en";
        mixed_dative: "~~en", "~~en", "~~en", "~~en";
        mixed_accusative: "~~en", "~~e", "~~es", "~~en";
        comparative_strong_nominative: "~r~er", "~r~e", "~r~es", "~r~e";
        comparative_strong_genitive: "~r~en", "~r~er", "~r~en", "~r~er";
        comparative_strong_dative: "~r~em", "~r~er", "~r~em", "~r~en";
        comparative_strong_accusative: "~r~en", "~r~e", "~r~es", "~r~e";
        comparative_weak_nominative: "~r~e", "~r~e", "~r~e", "~r~en";
        comparative_weak_genitive: "~r~en", "~r~en", "~r~en", "~r~en";
        comparative_weak_dative: "~r~en", "~r~en", "~r~en", "~r~en";
        comparative_weak_accusative: "~r~en", "~r~e", "~r~e", "~r~en";
        comparative_mixed_nominative: "~r~er", "~r~e", "~r~es", "~r~en";
        comparative_mixed_genitive: "~r~en", "~r~en", "~r~en", "~r~en";
        comparative_mixed_dative: "~r~en", "~r~en", "~r~en", "~r~en";
        comparative_mixed_accusative: "~r~en", "~r~e", "~r~es", "~r~en";
        predicative: "~r~";
    ),
    // Unedler: 4 words (schofler, speiübler, unedler, ignobler).
    declension!("unedler", 4, "~~r";
        strong_nominative: "~e~r", "~e~", "~e~s", "~e~";
        strong_genitive: "~e~n", "~e~r", "~e~n", "~e~r";
        strong_dative: "~e~m", "~e~r", "~e~m", "~e~n";
        strong_accusative: "~e~n", "~e~", "~e~s", "~e~";
        weak_nominative: "~e~", "~e~", "~e~", "~e~n";
        weak_genitive: "~e~n", "~e~n", "~e~n", "~e~n";
        weak_dative: "~e~n", "~e~n", "~e~n", "~e~n";
        weak_accusative: "~e~n", "~e~", "~e~", "~e~n";
        mixed_nominative: "~e~r", "~e~", "~e~s", "~e~n";
        mixed_genitive: "~e~n", "~e~n", "~e~n", "~e~n";
        mixed_dative: "~e~n", "~e~n", "~e~n", "~e~n";
        mixed_accusative: "~e~n", "~e~", "~e~s", "~e~n";
        comparative_strong_nominative: "~~rer", "~~re", "~~res", "~~re";
        comparative_strong_genitive: "~~ren", "~~rer", "~~ren", "~~rer";
        comparative_strong_dative: "~~rem", "~~rer", "~~rem", "~~ren";
        comparative_strong_accusative: "~~ren", "~~re", "~~res", "~~re";
        comparative_weak_nominative: "~~re", "~~re", "~~re", "~~ren";
        comparative_weak_genitive: "~~ren", "~~ren", "~~ren", "~~ren";
        comparative_weak_dative: "~~ren", "~~ren", "~~ren", "~~ren";
        comparative_weak_accusative: "~~ren", "~~re", "~~re", "~~ren";
        comparative_mixed_nominative: "~~rer", "~~re", "~~res", "~~ren";
        comparative_mixed_genitive: "~~ren", "~~ren", "~~ren", "~~ren";
        comparative_mixed_dative: "~~ren", "~~ren", "~~ren", "~~ren";
        comparative_mixed_accusative: "~~ren", "~~re", "~~res", "~~ren";
        predicative: "~~r";
    ),
    // Ärger: 3 words (ärmer, älter, ärger).
    declension!("ärger", 3, "ä~r";
        strong_nominative: "a~r", "a~", "a~s", "a~";
        strong_genitive: "a~n", "a~r", "a~n", "a~r";
        strong_dative: "a~m", "a~r", "a~m", "a~n";
        strong_accusative: "a~n", "a~", "a~s", "a~";
        weak_nominative: "a~", "a~", "a~", "a~n";
        weak_genitive: "a~n", "a~n", "a~n", "a~n";
        weak_dative: "a~n", "a~n", "a~n", "a~n";
        weak_accusative: "a~n", "a~", "a~", "a~n";
        mixed_nominative: "a~r", "a~", "a~s", "a~n";
        mixed_genitive: "a~n", "a~n", "a~n", "a~n";
        mixed_dative: "a~n", "a~n", "a~n", "a~n";
        mixed_accusative: "a~n", "a~", "a~s", "a~n";
        comparative_strong_nominative: "ä~rer", "ä~re", "ä~res", "ä~re";
        comparative_strong_genitive: "ä~ren", "ä~rer", "ä~ren", "ä~rer";
        comparative_strong_dative: "ä~rem", "ä~rer", "ä~rem", "ä~ren";
        comparative_strong_accusative: "ä~ren", "ä~re", "ä~res", "ä~re";
        comparative_weak_nominative: "ä~re", "ä~re", "ä~re", "ä~ren";
        comparative_weak_genitive: "ä~ren", "ä~ren", "ä~ren", "ä~ren";
        comparative_weak_dative: "ä~ren", "ä~ren", "ä~ren", "ä~ren";
        comparative_weak_accusative: "ä~ren", "ä~re", "ä~re", "ä~ren";
        comparative_mixed_nominative: "ä~rer", "ä~re", "ä~res", "ä~ren";
        comparative_mixed_genitive: "ä~ren", "ä~ren", "ä~ren", "ä~ren";
        comparative_mixed_dative: "ä~ren", "ä~ren", "ä~ren", "ä~ren";
        comparative_mixed_accusative: "ä~ren", "ä~re", "ä~res", "ä~ren";
        predicative: "ä~r";
    ),
    // Istausgeruhter: 3 words (ist ausgeruhter, ist verschimmelter, ist abgemilderter).
    declension!("ist ausgeruhter", 3, "ist ~r";
        strong_nominative: "~r", "~", "~s", "~";
        strong_genitive: "~n", "~r", "~n", "~r";
        strong_dative: "~m", "~r", "~m", "~n";
        strong_accusative: "~n", "~", "~s", "~";
        weak_nominative: "~", "~", "~", "~n";
        weak_genitive: "~n", "~n", "~n", "~n";
        weak_dative: "~n", "~n", "~n", "~n";
        weak_accusative: "~n", "~", "~", "~n";
        mixed_nominative: "~r", "~", "~s", "~n";
        mixed_genitive: "~n", "~n", "~n", "~n";
        mixed_dative: "~n", "~n", "~n", "~n";
        mixed_accusative: "~n", "~", "~s", "~n";
        comparative_strong_nominative: "~rer", "~re", "~res", "~re";
        comparative_strong_genitive: "~ren", "~rer", "~ren", "~rer";
        comparative_strong_dative: "~rem", "~rer", "~rem", "~ren";
        comparative_strong_accusative: "~ren", "~re", "~res", "~re";
        comparative_weak_nominative: "~re", "~re", "~re", "~ren";
        comparative_weak_genitive: "~ren", "~ren", "~ren", "~ren";
        comparative_weak_dative: "~ren", "~ren", "~ren", "~ren";
        comparative_weak_accusative: "~ren", "~re", "~re", "~ren";
        comparative_mixed_nominative: "~rer", "~re", "~res", "~ren";
        comparative_mixed_genitive: "~ren", "~ren", "~ren", "~ren";
        comparative_mixed_dative: "~ren", "~ren", "~ren", "~ren";
        comparative_mixed_accusative: "~ren", "~re", "~res", "~ren";
        predicative: "ist ~r|es ist ~r|sind ~r";
    ),
];
