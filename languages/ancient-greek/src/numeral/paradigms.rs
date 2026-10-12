//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's feature says):
/// `nominative_masculine` (sg, du, pl)
/// `nominative_feminine` (sg, du, pl)
/// `nominative_neuter` (sg, du, pl)
/// `genitive_masculine` (sg, du, pl)
/// `genitive_feminine` (sg, du, pl)
/// `genitive_neuter` (sg, du, pl)
/// `dative_masculine` (sg, du, pl)
/// `dative_feminine` (sg, du, pl)
/// `dative_neuter` (sg, du, pl)
/// `accusative_masculine` (sg, du, pl)
/// `accusative_feminine` (sg, du, pl)
/// `accusative_neuter` (sg, du, pl)
/// `vocative_masculine` (sg, du, pl)
/// `vocative_feminine` (sg, du, pl)
/// `vocative_neuter` (sg, du, pl)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal;
     nominative_masculine: $c0:tt, $c1:tt, $c2:tt;
     nominative_feminine: $c3:tt, $c4:tt, $c5:tt;
     nominative_neuter: $c6:tt, $c7:tt, $c8:tt;
     genitive_masculine: $c9:tt, $c10:tt, $c11:tt;
     genitive_feminine: $c12:tt, $c13:tt, $c14:tt;
     genitive_neuter: $c15:tt, $c16:tt, $c17:tt;
     dative_masculine: $c18:tt, $c19:tt, $c20:tt;
     dative_feminine: $c21:tt, $c22:tt, $c23:tt;
     dative_neuter: $c24:tt, $c25:tt, $c26:tt;
     accusative_masculine: $c27:tt, $c28:tt, $c29:tt;
     accusative_feminine: $c30:tt, $c31:tt, $c32:tt;
     accusative_neuter: $c33:tt, $c34:tt, $c35:tt;
     vocative_masculine: $c36:tt, $c37:tt, $c38:tt;
     vocative_feminine: $c39:tt, $c40:tt, $c41:tt;
     vocative_neuter: $c42:tt, $c43:tt, $c44:tt;
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
            ]),
        }
    };
}

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `χῑλῐοι`: 11 words.
    Χῑλῐοι,
    /// Like `τεσσᾰρες`: 2 words.
    Τεσσᾰρες,
    /// Like `δῠο`: 1 words.
    Δῠο,
    /// Like `τρεῑς`: 1 words.
    Τρεῑς,
    /// Like `δεκᾰτεσσᾰρες`: 1 words.
    Δεκᾰτεσσᾰρες,
    /// Like `εἱς`: 1 words.
    Εἱς,
    /// Like `δευτερος`: 1 words.
    Δευτερος,
    /// Like `πισῠρες`: 1 words.
    Πισῠρες,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 8] = [
        Declension::Χῑλῐοι,
        Declension::Τεσσᾰρες,
        Declension::Δῠο,
        Declension::Τρεῑς,
        Declension::Δεκᾰτεσσᾰρες,
        Declension::Εἱς,
        Declension::Δευτερος,
        Declension::Πισῠρες,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 8] = [
    // Χῑλῐοι: 11 words (δῐσχῑλῐοι, χῑλῐοι, τρῐσχῑλῐοι, τρῐᾱκοσῐοι, τετρᾰκοσῐοι, δῐᾱκοσῐοι).
    declension!("χῑλῐοι", 11, "~οι";
        nominative_masculine: "~ος", "~ω", "~οι";
        nominative_feminine: "~ᾱ", "~ᾱ", "~αι";
        nominative_neuter: "~ον", "~ω", "~ᾰ";
        genitive_masculine: "~ου", "~οιν", "~ων";
        genitive_feminine: "~ᾱς", "~αιν", "~ων";
        genitive_neuter: "~ου", "~οιν", "~ων";
        dative_masculine: "~ῳ", "~οιν", "~οις";
        dative_feminine: "~ᾳ", "~αιν", "~αις";
        dative_neuter: "~ῳ", "~οιν", "~οις";
        accusative_masculine: "~ον", "~ω", "~ους";
        accusative_feminine: "~ᾱν", "~ᾱ", "~ᾱς";
        accusative_neuter: "~ον", "~ω", "~ᾰ";
        vocative_masculine: "~ε", "~ω", "~οι";
        vocative_feminine: "~ᾱ", "~ᾱ", "~αι";
        vocative_neuter: "~ον", "~ω", "~ᾰ";
    ),
    // Τεσσᾰρες: 2 words (τεσσᾰρες, τεττᾰρες).
    declension!("τεσσᾰρες", 2, "~ες";
        nominative_masculine: "~ος", "~ω", "~ες";
        nominative_feminine: "~ᾱ", "~ᾱ", "~ες";
        nominative_neuter: "~ον", "~ω", "~ᾰ";
        genitive_masculine: "~ου", "~οιν", "~ων";
        genitive_feminine: "~ᾱς", "~αιν", "~ων";
        genitive_neuter: "~ου", "~οιν", "~ων";
        dative_masculine: "~ῳ", "~οιν", "~σῐ|~σῐν";
        dative_feminine: "~ᾳ", "~αιν", "~σῐ|~σῐν";
        dative_neuter: "~ῳ", "~οιν", "~σῐ|~σῐν";
        accusative_masculine: "~ον", "~ω", "~ᾰς";
        accusative_feminine: "~ᾱν", "~ᾱ", "~ᾰς";
        accusative_neuter: "~ον", "~ω", "~ᾰ";
        vocative_masculine: "~ε", "~ω", "~οι";
        vocative_feminine: "~ᾱ", "~ᾱ", "~αι";
        vocative_neuter: "~ον", "~ω", "~ᾰ";
    ),
    // Δῠο: 1 words (δῠο).
    declension!("δῠο", 1, "~ο";
        nominative_masculine: "~ος", "~ο", "~οι";
        nominative_feminine: "~ᾱ", "~ο", "~αι";
        nominative_neuter: "~ον", "~ο", "~ᾰ";
        genitive_masculine: "~ου", "~οῑν|~εῑν", "~ων";
        genitive_feminine: "~ᾱς", "~οῑν|~εῑν", "~ων";
        genitive_neuter: "~ου", "~οῑν|~εῑν", "~ων";
        dative_masculine: "~ῳ", "~οῑν|~εῑν|~σῐ|~σῐν", "~οις";
        dative_feminine: "~ᾳ", "~οῑν|~εῑν|~σῐ|~σῐν", "~αις";
        dative_neuter: "~ῳ", "~οῑν|~εῑν|~σῐ|~σῐν", "~οις";
        accusative_masculine: "~ον", "~ο", "~ους";
        accusative_feminine: "~ᾱν", "~ο", "~ᾱς";
        accusative_neuter: "~ον", "~ο", "~ᾰ";
        vocative_masculine: "~ε", "~ο", "~οι";
        vocative_feminine: "~ᾱ", "~ο", "~αι";
        vocative_neuter: "~ον", "~ο", "~ᾰ";
    ),
    // Τρεῑς: 1 words (τρεῑς).
    declension!("τρεῑς", 1, "~εῑς";
        nominative_masculine: "~ος", "~ω", "~εῑς";
        nominative_feminine: "~ᾱ", "~ᾱ", "~εῑς";
        nominative_neuter: "~ον", "~ω", "~ῐᾰ";
        genitive_masculine: "~ου", "~οιν", "~ῐων";
        genitive_feminine: "~ᾱς", "~αιν", "~ῐων";
        genitive_neuter: "~ου", "~οιν", "~ῐων";
        dative_masculine: "~ῳ", "~οιν", "~ῐσῐ|~ῐσῐν";
        dative_feminine: "~ᾳ", "~αιν", "~ῐσῐ|~ῐσῐν";
        dative_neuter: "~ῳ", "~οιν", "~ῐσῐ|~ῐσῐν";
        accusative_masculine: "~ον", "~ω", "~εῑς";
        accusative_feminine: "~ᾱν", "~ᾱ", "~εῑς";
        accusative_neuter: "~ον", "~ω", "~ῐᾰ";
        vocative_masculine: "~ε", "~ω", "~οι";
        vocative_feminine: "~ᾱ", "~ᾱ", "~αι";
        vocative_neuter: "~ον", "~ω", "~ᾰ";
    ),
    // Δεκᾰτεσσᾰρες: 1 words (δεκᾰτεσσᾰρες).
    declension!("δεκᾰτεσσᾰρες", 1, "~ᾰ~ᾰ~ες";
        nominative_masculine: "~ᾰ~ᾰ~ες", "~ᾰ~ᾰ~ες", "~α~α~ες";
        nominative_feminine: "~ᾰ~ᾰ~ες", "~ᾰ~ᾰ~ες", "~α~α~ες";
        nominative_neuter: "~ᾰ~ᾰ~ες", "~ᾰ~ᾰ~ες", "~α~α~ᾰ";
        genitive_masculine: "~ᾰ~ᾰ~ες", "~ᾰ~ᾰ~ες", "~α~α~ων";
        genitive_feminine: "~ᾰ~ᾰ~ες", "~ᾰ~ᾰ~ες", "~α~α~ων";
        genitive_neuter: "~ᾰ~ᾰ~ες", "~ᾰ~ᾰ~ες", "~α~α~ων";
        dative_masculine: "~ᾰ~ᾰ~ες", "~ᾰ~ᾰ~ες", "~α~α~σι|~α~α~σιν";
        dative_feminine: "~ᾰ~ᾰ~ες", "~ᾰ~ᾰ~ες", "~α~α~σι|~α~α~σιν";
        dative_neuter: "~ᾰ~ᾰ~ες", "~ᾰ~ᾰ~ες", "~α~α~σῐ|~α~α~σῐν";
        accusative_masculine: "~ᾰ~ᾰ~ες", "~ᾰ~ᾰ~ες", "~α~α~ᾰς";
        accusative_feminine: "~ᾰ~ᾰ~ες", "~ᾰ~ᾰ~ες", "~α~α~ᾰς";
        accusative_neuter: "~ᾰ~ᾰ~ες", "~ᾰ~ᾰ~ες", "~α~α~ᾰ";
        vocative_masculine: "~ᾰ~ᾰ~ες", "~ᾰ~ᾰ~ες", "~ῐ~ῠ~ες";
        vocative_feminine: "~ᾰ~ᾰ~ες", "~ᾰ~ᾰ~ες", "~ῐ~ῠ~ες";
        vocative_neuter: "~ᾰ~ᾰ~ες", "~ᾰ~ᾰ~ες", "~ῐ~ῠ~ᾰ";
    ),
    // Εἱς: 1 words (εἱς).
    declension!("εἱς", 1, "εἱς";
        nominative_masculine: "εἱς", "εἱς", "εἱς";
        nominative_feminine: "μῐᾰ", "εἱς", "εἱς";
        nominative_neuter: "ἑν", "εἱς", "εἱς";
        genitive_masculine: "ἑνος", "εἱς", "εἱς";
        genitive_feminine: "μῐᾱς", "εἱς", "εἱς";
        genitive_neuter: "ἑνος", "εἱς", "εἱς";
        dative_masculine: "ἑνῐ", "εἱς", "εἱς";
        dative_feminine: "μῐᾳ", "εἱς", "εἱς";
        dative_neuter: "ἑνῐ", "εἱς", "εἱς";
        accusative_masculine: "ἑνᾰ", "εἱς", "εἱς";
        accusative_feminine: "μῐᾰν", "εἱς", "εἱς";
        accusative_neuter: "ἑν", "εἱς", "εἱς";
        vocative_masculine: "εἱς", "εἱς", "εἱς";
        vocative_feminine: "μῐᾰ", "εἱς", "εἱς";
        vocative_neuter: "ἑν", "εἱς", "εἱς";
    ),
    // Δευτερος: 1 words (δευτερος).
    declension!("δευτερος", 1, "~ος";
        nominative_masculine: "~ος", "~ω", "~οι";
        nominative_feminine: "~ᾱ", "~ᾱ", "~αι";
        nominative_neuter: "~ον", "~ω", "~ᾰ";
        genitive_masculine: "~ου", "~οιν", "~ων";
        genitive_feminine: "~ᾱς", "~αιν", "~ων";
        genitive_neuter: "~ου", "~οιν", "~ων";
        dative_masculine: "~ῳ", "~οιν", "~οις";
        dative_feminine: "~ᾳ", "~αιν", "~αις";
        dative_neuter: "~ῳ", "~οιν", "~οις";
        accusative_masculine: "~ον", "~ω", "~ους";
        accusative_feminine: "~ᾱν", "~ᾱ", "~ᾱς";
        accusative_neuter: "~ον", "~ω", "~ᾰ";
        vocative_masculine: "~ε", "~ω", "~οι";
        vocative_feminine: "~ᾱ", "~ᾱ", "~αι";
        vocative_neuter: "~ον", "~ω", "~ᾰ";
    ),
    // Πισῠρες: 1 words (πισῠρες).
    declension!("πισῠρες", 1, "~ι~ῠ~ες";
        nominative_masculine: "~ι~ῠ~ες", "~ι~ῠ~ες", "~ῐ~ῠ~ες";
        nominative_feminine: "~ι~ῠ~ες", "~ι~ῠ~ες", "~ῐ~ῠ~ες";
        nominative_neuter: "~ι~ῠ~ες", "~ι~ῠ~ες", "~ῐ~ῠ~ᾰ";
        genitive_masculine: "~ι~ῠ~ες", "~ι~ῠ~ες", "~ῐ~ῠ~ων";
        genitive_feminine: "~ι~ῠ~ες", "~ι~ῠ~ες", "~ῐ~ῠ~ων";
        genitive_neuter: "~ι~ῠ~ες", "~ι~ῠ~ες", "~ῐ~ῠ~ων";
        dative_masculine: "~ι~ῠ~ες", "~ι~ῠ~ες", "~ι~υ~εσσι|~ι~υ~οισι|~ι~υ~ῃσι|~ι~υ~αις";
        dative_feminine: "~ι~ῠ~ες", "~ι~ῠ~ες", "~ι~υ~εσσι|~ι~υ~οισι|~ι~υ~ῃσι|~ι~υ~αις";
        dative_neuter: "~ι~ῠ~ες", "~ι~ῠ~ες", "~ι~υ~εσσι|~ι~υ~οισι";
        accusative_masculine: "~ι~ῠ~ες", "~ι~ῠ~ες", "~ῐ~ῠ~ᾰς";
        accusative_feminine: "~ι~ῠ~ες", "~ι~ῠ~ες", "~ῐ~ῠ~ᾰς";
        accusative_neuter: "~ι~ῠ~ες", "~ι~ῠ~ες", "~ῐ~ῠ~ᾰ";
        vocative_masculine: "~ι~ῠ~ες", "~ι~ῠ~ες", "~ῐ~ῠ~ες";
        vocative_feminine: "~ι~ῠ~ες", "~ι~ῠ~ες", "~ῐ~ῠ~ες";
        vocative_neuter: "~ι~ῠ~ες", "~ι~ῠ~ες", "~ῐ~ῠ~ᾰ";
    ),
];
