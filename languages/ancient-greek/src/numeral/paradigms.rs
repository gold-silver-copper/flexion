//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (`[gender: Neuter]`, if
/// they share them), the declension it is like (if any), then a row of patterns
/// per line (`derived`: the form comes from another cell's, see
/// `relations.rs`; `_`: the form is the declension's it is like):
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
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident)?;
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
            parent: like!($($parent)?),
            inherent: ::std::borrow::Cow::Borrowed(&[]),
            table: ::std::borrow::Cow::Borrowed(concat!(
                flexion::cell!($c0), "\t",
                flexion::cell!($c1), "\t",
                flexion::cell!($c2), "\t",
                flexion::cell!($c3), "\t",
                flexion::cell!($c4), "\t",
                flexion::cell!($c5), "\t",
                flexion::cell!($c6), "\t",
                flexion::cell!($c7), "\t",
                flexion::cell!($c8), "\t",
                flexion::cell!($c9), "\t",
                flexion::cell!($c10), "\t",
                flexion::cell!($c11), "\t",
                flexion::cell!($c12), "\t",
                flexion::cell!($c13), "\t",
                flexion::cell!($c14), "\t",
                flexion::cell!($c15), "\t",
                flexion::cell!($c16), "\t",
                flexion::cell!($c17), "\t",
                flexion::cell!($c18), "\t",
                flexion::cell!($c19), "\t",
                flexion::cell!($c20), "\t",
                flexion::cell!($c21), "\t",
                flexion::cell!($c22), "\t",
                flexion::cell!($c23), "\t",
                flexion::cell!($c24), "\t",
                flexion::cell!($c25), "\t",
                flexion::cell!($c26), "\t",
                flexion::cell!($c27), "\t",
                flexion::cell!($c28), "\t",
                flexion::cell!($c29), "\t",
                flexion::cell!($c30), "\t",
                flexion::cell!($c31), "\t",
                flexion::cell!($c32), "\t",
                flexion::cell!($c33), "\t",
                flexion::cell!($c34), "\t",
                flexion::cell!($c35), "\t",
                flexion::cell!($c36), "\t",
                flexion::cell!($c37), "\t",
                flexion::cell!($c38), "\t",
                flexion::cell!($c39), "\t",
                flexion::cell!($c40), "\t",
                flexion::cell!($c41), "\t",
                flexion::cell!($c42), "\t",
                flexion::cell!($c43), "\t",
                flexion::cell!($c44), "\t",
            )),
        }
    };
}

/// The declension a declension is like, by its name.
macro_rules! like {
    () => {
        None
    };
    ($parent:ident) => {
        Some(Declension::$parent as u16)
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
        nominative_masculine: "~οι", "~οι", "~οι";
        nominative_feminine: "~οι", "~οι", "~αι";
        nominative_neuter: "~οι", derived, "~ᾰ";
        genitive_masculine: "~οι", "~οι", "~ων";
        genitive_feminine: "~οι", "~οι", "~ων";
        genitive_neuter: "~οι", "~οι", "~ων";
        dative_masculine: "~οι", "~οι", "~οις";
        dative_feminine: "~οι", "~οι", "~αις";
        dative_neuter: "~οι", "~οι", "~οις";
        accusative_masculine: "~οι", derived, "~ους";
        accusative_feminine: "~οι", "~οι", "~ᾱς";
        accusative_neuter: "~οι", derived, "~ᾰ";
        vocative_masculine: "~οι", derived, "~οι";
        vocative_feminine: "~οι", "~οι", "~αι";
        vocative_neuter: "~οι", derived, "~ᾰ";
    ),
    // Τεσσᾰρες: 2 words (τεσσᾰρες, τεττᾰρες).
    declension!("τεσσᾰρες", 2, "~ες";
        nominative_masculine: "~ες", "~ες", "~ες";
        nominative_feminine: "~ες", "~ες", "~ες";
        nominative_neuter: "~ες", derived, "~ᾰ";
        genitive_masculine: "~ες", "~ες", "~ων";
        genitive_feminine: "~ες", "~ες", "~ων";
        genitive_neuter: "~ες", "~ες", "~ων";
        dative_masculine: "~ες", "~ες", "~σῐ|~σῐν";
        dative_feminine: "~ες", "~ες", "~σῐ|~σῐν";
        dative_neuter: "~ες", "~ες", "~σῐ|~σῐν";
        accusative_masculine: "~ες", derived, "~ᾰς";
        accusative_feminine: "~ες", "~ες", "~ᾰς";
        accusative_neuter: "~ες", derived, "~ᾰ";
        vocative_masculine: "~ες", derived, derived;
        vocative_feminine: "~ες", "~ες", derived;
        vocative_neuter: "~ες", derived, derived;
    ),
    // Δῠο: 1 words (δῠο).
    declension!("δῠο", 1, "~ο";
        nominative_masculine: "~ο", "~ο", "~ο";
        nominative_feminine: "~ο", "~ο", derived;
        nominative_neuter: "~ο", "~ο", derived;
        genitive_masculine: "~ο", "~οῑν|~εῑν", derived;
        genitive_feminine: "~ο", "~οῑν|~εῑν", derived;
        genitive_neuter: "~ο", "~οῑν|~εῑν", derived;
        dative_masculine: "~ο", "~οῑν|~εῑν|~σῐ|~σῐν", derived;
        dative_feminine: "~ο", "~οῑν|~εῑν|~σῐ|~σῐν", derived;
        dative_neuter: "~ο", "~οῑν|~εῑν|~σῐ|~σῐν", derived;
        accusative_masculine: "~ο", "~ο", derived;
        accusative_feminine: "~ο", "~ο", derived;
        accusative_neuter: "~ο", "~ο", derived;
        vocative_masculine: "~ο", "~ο", derived;
        vocative_feminine: "~ο", "~ο", derived;
        vocative_neuter: "~ο", "~ο", derived;
    ),
    // Τρεῑς: 1 words (τρεῑς).
    declension!("τρεῑς", 1, "~εῑς";
        nominative_masculine: "~εῑς", "~εῑς", "~εῑς";
        nominative_feminine: "~εῑς", "~εῑς", "~εῑς";
        nominative_neuter: "~εῑς", derived, "~ῐᾰ";
        genitive_masculine: "~εῑς", "~εῑς", "~ῐων";
        genitive_feminine: "~εῑς", "~εῑς", "~ῐων";
        genitive_neuter: "~εῑς", "~εῑς", "~ῐων";
        dative_masculine: "~εῑς", "~εῑς", "~ῐσῐ|~ῐσῐν";
        dative_feminine: "~εῑς", "~εῑς", "~ῐσῐ|~ῐσῐν";
        dative_neuter: "~εῑς", "~εῑς", "~ῐσῐ|~ῐσῐν";
        accusative_masculine: "~εῑς", derived, "~εῑς";
        accusative_feminine: "~εῑς", "~εῑς", "~εῑς";
        accusative_neuter: "~εῑς", derived, "~ῐᾰ";
        vocative_masculine: "~εῑς", derived, derived;
        vocative_feminine: "~εῑς", "~εῑς", derived;
        vocative_neuter: "~εῑς", derived, derived;
    ),
    // Δεκᾰτεσσᾰρες: 1 words (δεκᾰτεσσᾰρες).
    declension!("δεκᾰτεσσᾰρες", 1, "~ᾰ~ᾰ~ες";
        nominative_masculine: "~ᾰ~ᾰ~ες", "~ᾰ~ᾰ~ες", "~α~α~ες";
        nominative_feminine: "~ᾰ~ᾰ~ες", "~ᾰ~ᾰ~ες", "~α~α~ες";
        nominative_neuter: "~ᾰ~ᾰ~ες", derived, "~α~α~ᾰ";
        genitive_masculine: "~ᾰ~ᾰ~ες", "~ᾰ~ᾰ~ες", "~α~α~ων";
        genitive_feminine: "~ᾰ~ᾰ~ες", "~ᾰ~ᾰ~ες", "~α~α~ων";
        genitive_neuter: "~ᾰ~ᾰ~ες", "~ᾰ~ᾰ~ες", "~α~α~ων";
        dative_masculine: "~ᾰ~ᾰ~ες", "~ᾰ~ᾰ~ες", "~α~α~σι|~α~α~σιν";
        dative_feminine: "~ᾰ~ᾰ~ες", "~ᾰ~ᾰ~ες", "~α~α~σι|~α~α~σιν";
        dative_neuter: "~ᾰ~ᾰ~ες", "~ᾰ~ᾰ~ες", "~α~α~σῐ|~α~α~σῐν";
        accusative_masculine: "~ᾰ~ᾰ~ες", derived, "~α~α~ᾰς";
        accusative_feminine: "~ᾰ~ᾰ~ες", "~ᾰ~ᾰ~ες", "~α~α~ᾰς";
        accusative_neuter: "~ᾰ~ᾰ~ες", derived, "~α~α~ᾰ";
        vocative_masculine: "~ᾰ~ᾰ~ες", derived, derived;
        vocative_feminine: "~ᾰ~ᾰ~ες", "~ᾰ~ᾰ~ες", derived;
        vocative_neuter: "~ᾰ~ᾰ~ες", derived, derived;
    ),
    // Εἱς: 1 words (εἱς).
    declension!("εἱς", 1, "εἱς";
        nominative_masculine: "εἱς", "εἱς", "εἱς";
        nominative_feminine: "μῐᾰ", "εἱς", derived;
        nominative_neuter: "ἑν", derived, derived;
        genitive_masculine: "ἑνος", "εἱς", derived;
        genitive_feminine: "μῐᾱς", "εἱς", derived;
        genitive_neuter: "ἑνος", "εἱς", derived;
        dative_masculine: "ἑνῐ", "εἱς", derived;
        dative_feminine: "μῐᾳ", "εἱς", derived;
        dative_neuter: "ἑνῐ", "εἱς", derived;
        accusative_masculine: "ἑνᾰ", derived, derived;
        accusative_feminine: "μῐᾰν", "εἱς", derived;
        accusative_neuter: "ἑν", derived, derived;
        vocative_masculine: "εἱς", derived, derived;
        vocative_feminine: "μῐᾰ", "εἱς", derived;
        vocative_neuter: "ἑν", derived, derived;
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
        nominative_neuter: "~ι~ῠ~ες", derived, "~ῐ~ῠ~ᾰ";
        genitive_masculine: "~ι~ῠ~ες", "~ι~ῠ~ες", "~ῐ~ῠ~ων";
        genitive_feminine: "~ι~ῠ~ες", "~ι~ῠ~ες", "~ῐ~ῠ~ων";
        genitive_neuter: "~ι~ῠ~ες", "~ι~ῠ~ες", "~ῐ~ῠ~ων";
        dative_masculine: "~ι~ῠ~ες", "~ι~ῠ~ες", "~ι~υ~εσσι|~ι~υ~οισι|~ι~υ~ῃσι|~ι~υ~αις";
        dative_feminine: "~ι~ῠ~ες", "~ι~ῠ~ες", "~ι~υ~εσσι|~ι~υ~οισι|~ι~υ~ῃσι|~ι~υ~αις";
        dative_neuter: "~ι~ῠ~ες", "~ι~ῠ~ες", "~ι~υ~εσσι|~ι~υ~οισι";
        accusative_masculine: "~ι~ῠ~ες", derived, "~ῐ~ῠ~ᾰς";
        accusative_feminine: "~ι~ῠ~ες", "~ι~ῠ~ες", "~ῐ~ῠ~ᾰς";
        accusative_neuter: "~ι~ῠ~ες", derived, "~ῐ~ῠ~ᾰ";
        vocative_masculine: "~ι~ῠ~ες", derived, "~ῐ~ῠ~ες";
        vocative_feminine: "~ι~ῠ~ες", "~ι~ῠ~ες", "~ῐ~ῠ~ες";
        vocative_neuter: "~ι~ῠ~ες", derived, "~ῐ~ῠ~ᾰ";
    ),
];
