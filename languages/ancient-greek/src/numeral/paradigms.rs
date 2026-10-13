//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first. A `%`
//! is the first letter of the stem piece after it (reduplication: `%ε~κα`),
//! as the copy laws in `laws.rs` change it.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (`[gender: Neuter]`, if
/// they share them), the declension it is like (if any), then a row of patterns
/// per line (`derived`: the form comes from another cell's, see
/// `relations.rs`; `_`, or a row left out: the form is the declension's it is like,
/// or, in a declension like no other, derived):
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
     $(nominative_masculine: $c0:tt, $c1:tt, $c2:tt;)?
     $(nominative_feminine: $c3:tt, $c4:tt, $c5:tt;)?
     $(nominative_neuter: $c6:tt, $c7:tt, $c8:tt;)?
     $(genitive_masculine: $c9:tt, $c10:tt, $c11:tt;)?
     $(genitive_feminine: $c12:tt, $c13:tt, $c14:tt;)?
     $(genitive_neuter: $c15:tt, $c16:tt, $c17:tt;)?
     $(dative_masculine: $c18:tt, $c19:tt, $c20:tt;)?
     $(dative_feminine: $c21:tt, $c22:tt, $c23:tt;)?
     $(dative_neuter: $c24:tt, $c25:tt, $c26:tt;)?
     $(accusative_masculine: $c27:tt, $c28:tt, $c29:tt;)?
     $(accusative_feminine: $c30:tt, $c31:tt, $c32:tt;)?
     $(accusative_neuter: $c33:tt, $c34:tt, $c35:tt;)?
     $(vocative_masculine: $c36:tt, $c37:tt, $c38:tt;)?
     $(vocative_feminine: $c39:tt, $c40:tt, $c41:tt;)?
     $(vocative_neuter: $c42:tt, $c43:tt, $c44:tt;)?
    ) => {
        Paradigm {
            name: ::std::borrow::Cow::Borrowed($name),
            words: $words,
            lemma: ::std::borrow::Cow::Borrowed($lemma),
            parent: like!($($parent)?),
            inherent: ::std::borrow::Cow::Borrowed(&[]),
            table: ::std::borrow::Cow::Borrowed(concat!(
                $(flexion::cell!($c0),)? "\t",
                $(flexion::cell!($c1),)? "\t",
                $(flexion::cell!($c2),)? "\t",
                $(flexion::cell!($c3),)? "\t",
                $(flexion::cell!($c4),)? "\t",
                $(flexion::cell!($c5),)? "\t",
                $(flexion::cell!($c6),)? "\t",
                $(flexion::cell!($c7),)? "\t",
                $(flexion::cell!($c8),)? "\t",
                $(flexion::cell!($c9),)? "\t",
                $(flexion::cell!($c10),)? "\t",
                $(flexion::cell!($c11),)? "\t",
                $(flexion::cell!($c12),)? "\t",
                $(flexion::cell!($c13),)? "\t",
                $(flexion::cell!($c14),)? "\t",
                $(flexion::cell!($c15),)? "\t",
                $(flexion::cell!($c16),)? "\t",
                $(flexion::cell!($c17),)? "\t",
                $(flexion::cell!($c18),)? "\t",
                $(flexion::cell!($c19),)? "\t",
                $(flexion::cell!($c20),)? "\t",
                $(flexion::cell!($c21),)? "\t",
                $(flexion::cell!($c22),)? "\t",
                $(flexion::cell!($c23),)? "\t",
                $(flexion::cell!($c24),)? "\t",
                $(flexion::cell!($c25),)? "\t",
                $(flexion::cell!($c26),)? "\t",
                $(flexion::cell!($c27),)? "\t",
                $(flexion::cell!($c28),)? "\t",
                $(flexion::cell!($c29),)? "\t",
                $(flexion::cell!($c30),)? "\t",
                $(flexion::cell!($c31),)? "\t",
                $(flexion::cell!($c32),)? "\t",
                $(flexion::cell!($c33),)? "\t",
                $(flexion::cell!($c34),)? "\t",
                $(flexion::cell!($c35),)? "\t",
                $(flexion::cell!($c36),)? "\t",
                $(flexion::cell!($c37),)? "\t",
                $(flexion::cell!($c38),)? "\t",
                $(flexion::cell!($c39),)? "\t",
                $(flexion::cell!($c40),)? "\t",
                $(flexion::cell!($c41),)? "\t",
                $(flexion::cell!($c42),)? "\t",
                $(flexion::cell!($c43),)? "\t",
                $(flexion::cell!($c44),)? "\t",
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
        nominative_masculine: "~οι", _, "~οι";
        nominative_feminine: _, _, "~αι";
        nominative_neuter: _, _, "~ᾰ";
        genitive_masculine: _, _, "~ων";
        dative_feminine: _, _, "~αις";
        dative_neuter: _, _, "~οις";
        accusative_masculine: _, _, "~ους";
        accusative_feminine: _, _, "~ᾱς";
    ),
    // Τεσσᾰρες: 2 words (τεσσᾰρες, τεττᾰρες). Like Χῑλῐοι, except in 8 cells: nominative_masculine sg, nominative_masculine pl, nominative_feminine pl, dative_masculine pl, dative_feminine pl, dative_neuter pl, ….
    declension!("τεσσᾰρες", 2, "~ες", like Χῑλῐοι;
        nominative_masculine: "~ες", _, "~ες";
        nominative_feminine: _, _, "~ες";
        dative_masculine: _, _, "~σῐ|~σῐν";
        dative_feminine: _, _, "~σῐ|~σῐν";
        dative_neuter: _, _, "~σῐ|~σῐν";
        accusative_masculine: _, _, "~ᾰς";
        accusative_feminine: _, _, "~ᾰς";
    ),
    // Δῠο: 1 words (δῠο). Like Χῑλῐοι, except in 17 cells: nominative_masculine sg, nominative_masculine du, nominative_masculine pl, nominative_feminine du, nominative_feminine pl, nominative_neuter pl, ….
    declension!("δῠο", 1, "~ο", like Χῑλῐοι;
        nominative_masculine: "~ο", "~ο", derived;
        nominative_feminine: _, "~ο", derived;
        nominative_neuter: _, _, derived;
        genitive_masculine: _, "~οῑν|~εῑν", derived;
        genitive_feminine: _, "~οῑν|~εῑν", _;
        genitive_neuter: _, "~οῑν|~εῑν", _;
        dative_masculine: _, "~οῑν|~εῑν|~σῐ|~σῐν", _;
        dative_feminine: _, "~οῑν|~εῑν|~σῐ|~σῐν", derived;
        dative_neuter: _, "~οῑν|~εῑν|~σῐ|~σῐν", derived;
        accusative_masculine: _, _, derived;
        accusative_feminine: _, _, derived;
    ),
    // Τρεῑς: 1 words (τρεῑς). Like Χῑλῐοι, except in 10 cells: nominative_masculine sg, nominative_masculine pl, nominative_feminine pl, nominative_neuter pl, genitive_masculine pl, dative_masculine pl, ….
    declension!("τρεῑς", 1, "~εῑς", like Χῑλῐοι;
        nominative_masculine: "~εῑς", _, "~εῑς";
        nominative_feminine: _, _, "~εῑς";
        nominative_neuter: _, _, "~ῐᾰ";
        genitive_masculine: _, _, "~ῐων";
        dative_masculine: _, _, "~ῐσῐ|~ῐσῐν";
        dative_feminine: _, _, "~ῐσῐ|~ῐσῐν";
        dative_neuter: _, _, "~ῐσῐ|~ῐσῐν";
        accusative_masculine: _, _, "~εῑς";
        accusative_feminine: _, _, "~εῑς";
    ),
    // Δεκᾰτεσσᾰρες: 1 words (δεκᾰτεσσᾰρες).
    declension!("δεκᾰτεσσᾰρες", 1, "~ᾰ~ᾰ~ες";
        nominative_masculine: "~ᾰ~ᾰ~ες", _, "~α~α~ες";
        nominative_feminine: _, _, "~α~α~ες";
        nominative_neuter: _, _, "~α~α~ᾰ";
        genitive_masculine: _, _, "~α~α~ων";
        dative_masculine: _, _, "~α~α~σι|~α~α~σιν";
        dative_feminine: _, _, "~α~α~σι|~α~α~σιν";
        dative_neuter: _, _, "~α~α~σῐ|~α~α~σῐν";
        accusative_masculine: _, _, "~α~α~ᾰς";
        accusative_feminine: _, _, "~α~α~ᾰς";
    ),
    // Εἱς: 1 words (εἱς).
    declension!("εἱς", 1, "εἱς";
        nominative_masculine: "εἱς", _, _;
        nominative_feminine: "μῐᾰ", _, _;
        nominative_neuter: "ἑν", _, _;
        genitive_masculine: "ἑνος", _, _;
        genitive_feminine: "μῐᾱς", _, _;
        dative_masculine: "ἑνῐ", _, _;
        dative_feminine: "μῐᾳ", _, _;
        accusative_masculine: "ἑνᾰ", _, _;
        accusative_feminine: "μῐᾰν", _, _;
        vocative_masculine: "εἱς", _, _;
    ),
    // Δευτερος: 1 words (δευτερος). Like Χῑλῐοι, except in 14 cells: nominative_masculine sg, nominative_masculine du, nominative_feminine sg, nominative_feminine du, nominative_neuter sg, genitive_masculine sg, ….
    declension!("δευτερος", 1, "~ος", like Χῑλῐοι;
        nominative_masculine: "~ος", "~ω", _;
        nominative_feminine: "~ᾱ", "~ᾱ", _;
        nominative_neuter: "~ον", _, _;
        genitive_masculine: "~ου", "~οιν", _;
        genitive_feminine: "~ᾱς", "~αιν", _;
        dative_masculine: "~ῳ", _, _;
        dative_feminine: "~ᾳ", _, _;
        accusative_masculine: "~ον", _, _;
        accusative_feminine: "~ᾱν", _, _;
        vocative_masculine: "~ε", _, _;
    ),
    // Πισῠρες: 1 words (πισῠρες). Like Δεκᾰτεσσᾰρες, except in 10 cells: nominative_masculine sg, nominative_masculine pl, nominative_feminine pl, nominative_neuter pl, genitive_masculine pl, dative_masculine pl, ….
    declension!("πισῠρες", 1, "~ι~ῠ~ες", like Δεκᾰτεσσᾰρες;
        nominative_masculine: "~ι~ῠ~ες", _, "~ῐ~ῠ~ες";
        nominative_feminine: _, _, "~ῐ~ῠ~ες";
        nominative_neuter: _, _, "~ῐ~ῠ~ᾰ";
        genitive_masculine: _, _, "~ῐ~ῠ~ων";
        dative_masculine: _, _, "~ι~υ~εσσι|~ι~υ~οισι|~ι~υ~ῃσι|~ι~υ~αις";
        dative_feminine: _, _, "~ι~υ~εσσι|~ι~υ~οισι|~ι~υ~ῃσι|~ι~υ~αις";
        dative_neuter: _, _, "~ι~υ~εσσι|~ι~υ~οισι";
        accusative_masculine: _, _, "~ῐ~ῠ~ᾰς";
        accusative_feminine: _, _, "~ῐ~ῠ~ᾰς";
    ),
];
