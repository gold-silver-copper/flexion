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
    /// Like `ὁσος`: 15 words.
    Ὁσος,
    /// Like `οἱος`: 7 words.
    Οἱος,
    /// Like `ἑαυτοῡ`: 6 words.
    Ἑαυτοῡ,
    /// Like `τοῑος`: 4 words.
    Τοῑος,
    /// Like `σφω`: 4 words.
    Σφω,
    /// Like `ἐμαυτοῡ`: 3 words.
    Ἐμαυτοῡ,
    /// Like `ἡμεῑς`: 2 words.
    Ἡμεῑς,
    /// Like `ὁ`: 1 words.
    Ὁ,
    /// Like `σῠ`: 1 words.
    Σῠ,
    /// Like `τῐς`: 1 words.
    Τῐς,
    /// Like `ὁς`: 1 words.
    Ὁς,
    /// Like `ὁστῐς`: 1 words.
    Ὁστῐς,
    /// Like `οὐδεις`: 1 words.
    Οὐδεις,
    /// Like `ἱ`: 1 words.
    Ἱ,
    /// Like `πᾱς`: 1 words.
    Πᾱς,
    /// Like `οὑ`: 1 words.
    Οὑ,
    /// Like `ἁπας`: 1 words.
    Ἁπας,
    /// Like `ὁδε`: 1 words.
    Ὁδε,
    /// Like `μηδεις`: 1 words.
    Μηδεις,
    /// Like `ἀλληλων`: 1 words.
    Ἀλληλων,
    /// Like `ὁσπερ`: 1 words.
    Ὁσπερ,
    /// Like `σφεῑς`: 1 words.
    Σφεῑς,
    /// Like `ἐνῐοι`: 1 words.
    Ἐνῐοι,
    /// Like `ὁσᾰπλᾰσῐων`: 1 words.
    Ὁσᾰπλᾰσῐων,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 24] = [
        Declension::Ὁσος,
        Declension::Οἱος,
        Declension::Ἑαυτοῡ,
        Declension::Τοῑος,
        Declension::Σφω,
        Declension::Ἐμαυτοῡ,
        Declension::Ἡμεῑς,
        Declension::Ὁ,
        Declension::Σῠ,
        Declension::Τῐς,
        Declension::Ὁς,
        Declension::Ὁστῐς,
        Declension::Οὐδεις,
        Declension::Ἱ,
        Declension::Πᾱς,
        Declension::Οὑ,
        Declension::Ἁπας,
        Declension::Ὁδε,
        Declension::Μηδεις,
        Declension::Ἀλληλων,
        Declension::Ὁσπερ,
        Declension::Σφεῑς,
        Declension::Ἐνῐοι,
        Declension::Ὁσᾰπλᾰσῐων,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 24] = [
    // Ὁσος: 15 words (αὐτος, οὑτος, ἑος, ἐκεῑνος, ἑκᾰστος, ταὐτος).
    declension!("ὁσος", 15, "~ος";
        nominative_masculine: "~ος", _, "~οι";
        nominative_feminine: "~η", _, "~αι";
        nominative_neuter: _, _, "~ᾰ";
        genitive_masculine: "~ου", "~οιν", "~ων";
        genitive_feminine: "~ης", "~αιν", "~ων";
        dative_masculine: "~ῳ", _, "~οις";
        dative_feminine: "~ῃ", _, "~αις";
        accusative_masculine: "~ον", "~ω", "~ους";
        accusative_feminine: "~ην", "~ᾱ", "~ᾱς";
        accusative_neuter: "~ον", _, "~ᾰ";
        vocative_masculine: "~ε", _, _;
    ),
    // Οἱος: 7 words (ἀμφοτερος, οἱος, ἑκᾰτερος, οὐδετερος, ὁποτερος, μηδετερος). Like Ὁσος, except in 4 cells: nominative_feminine sg, genitive_feminine sg, dative_feminine sg, accusative_feminine sg.
    declension!("οἱος", 7, "~ος", like Ὁσος;
        nominative_feminine: "~ᾱ", _, _;
        genitive_feminine: "~ᾱς", _, _;
        dative_feminine: "~ᾳ", _, _;
        accusative_feminine: "~ᾱν", _, _;
    ),
    // Ἑαυτοῡ: 6 words (ἑαυτοῡ, ἑωυτοῡ, αὑτοῡ, σεαυτοῡ, σεωυτοῡ, σαυτοῡ). Like Ὁσος, except in 13 cells: nominative_masculine sg, nominative_masculine pl, nominative_feminine sg, nominative_feminine pl, nominative_neuter pl, genitive_masculine sg, ….
    declension!("ἑαυτοῡ", 6, "~οῡ", like Ὁσος;
        nominative_masculine: "~οῡ", _, derived;
        nominative_feminine: derived, _, derived;
        nominative_neuter: _, _, derived;
        genitive_masculine: "~οῡ", "~οῑν", _;
        genitive_feminine: _, "~αῑν", "~εων";
        dative_masculine: _, _, "~οῑς";
        dative_feminine: _, _, "~ῃσῐ";
        accusative_neuter: "~ο", _, _;
        vocative_masculine: derived, _, _;
    ),
    // Τοῑος: 4 words (ποῑος, τοῑος, πᾰντοῑος, ὁποῑος).
    declension!("τοῑος", 4, "~ῑος";
        nominative_masculine: "~ῑος", _, "~ῑοι";
        nominative_feminine: "~ιᾱ", _, "~ῑαι";
        nominative_neuter: _, _, "~ῑᾰ";
        genitive_masculine: "~ιου", "~ιοιν", "~ιων";
        genitive_feminine: "~ιᾱς", "~ιαιν", "~ιων";
        dative_masculine: "~ιῳ", _, "~ιοις";
        dative_feminine: "~ιᾳ", _, "~ιαις";
        accusative_masculine: "~ῑον", "~ιω", "~ιους";
        accusative_feminine: "~ιᾱν", "~ιᾱ", "~ιᾱς";
        accusative_neuter: "~ῑον", _, "~ῑᾰ";
        vocative_masculine: "~ῑε", _, _;
    ),
    // Σφω: 4 words (ἐγω, νω, σφω, ἀμφω). Like Ἑαυτοῡ, except in 18 cells: nominative_masculine sg, genitive_masculine sg, genitive_masculine du, genitive_masculine pl, genitive_feminine sg, genitive_feminine du, ….
    declension!("σφω", 4, "~ω", like Ἑαυτοῡ;
        nominative_masculine: "~ω", _, _;
        genitive_masculine: derived, "~ῳν", derived;
        genitive_feminine: derived, "~ῳν", derived;
        dative_masculine: derived, _, derived;
        dative_feminine: derived, _, derived;
        accusative_masculine: derived, _, derived;
        accusative_feminine: derived, "~ω", derived;
        accusative_neuter: derived, _, derived;
    ),
    // Ἐμαυτοῡ: 3 words (ἐμαυτοῡ, ἐμεωυτοῡ, ἐμωυτοῡ). Like Ἑαυτοῡ, except in 12 cells: genitive_masculine du, genitive_masculine pl, genitive_feminine du, genitive_feminine pl, dative_masculine pl, dative_feminine pl, ….
    declension!("ἐμαυτοῡ", 3, "~οῡ", like Ἑαυτοῡ;
        genitive_masculine: _, derived, derived;
        genitive_feminine: _, derived, derived;
        dative_masculine: _, _, derived;
        dative_feminine: _, _, derived;
        accusative_masculine: _, derived, derived;
        accusative_feminine: _, derived, derived;
        accusative_neuter: "~ον", _, derived;
    ),
    // Ἡμεῑς: 2 words (ἡμεῑς, ὑμεῑς). Like Σφω, except in 15 cells: nominative_masculine sg, nominative_masculine pl, nominative_feminine pl, nominative_neuter pl, genitive_masculine du, genitive_masculine pl, ….
    declension!("ἡμεῑς", 2, "~εῑς", like Σφω;
        nominative_masculine: "~εῑς", _, "~εῑς";
        nominative_feminine: _, _, "~εῑς";
        nominative_neuter: _, _, "~εῑς";
        genitive_masculine: _, derived, "~ων";
        genitive_feminine: _, derived, "~ων";
        dative_masculine: _, _, "~ῑν";
        dative_feminine: _, _, "~ῑν";
        accusative_masculine: _, derived, "~ᾱς";
        accusative_feminine: _, derived, "~ᾱς";
        accusative_neuter: _, _, "~ᾱς";
    ),
    // Ὁ: 1 words (ὁ).
    declension!("ὁ", 1, "ὁ";
        nominative_masculine: "ὁ", _, "οἱ";
        nominative_feminine: "ἡ", _, "αἱ";
        nominative_neuter: _, _, "τᾰ";
        genitive_masculine: "τοῡ", "τοῑν", "των";
        genitive_feminine: "της", "τοῑν", "των";
        dative_masculine: "τῳ", _, "τοῑς";
        dative_feminine: "τῃ", _, "ταῑς";
        accusative_masculine: "τον", "τω", "τους";
        accusative_feminine: "την", "τω", "τᾱς";
        accusative_neuter: "το", _, "τᾰ";
    ),
    // Σῠ: 1 words (σῠ).
    declension!("σῠ", 1, "σῠ";
        nominative_masculine: "σῠ", _, _;
        nominative_feminine: "σῠ", _, _;
        nominative_neuter: "σῠ", _, _;
        genitive_masculine: "σοῡ|σου", _, _;
        genitive_feminine: "σοῡ|σου", _, _;
        genitive_neuter: "σοῡ|σου", _, _;
        dative_masculine: "σοι|σοι", _, _;
        dative_feminine: "σοι|σοι", _, _;
        dative_neuter: "σοι|σοι", _, _;
        accusative_masculine: "σε|σε", _, _;
        accusative_feminine: "σε|σε", _, _;
        accusative_neuter: "σε|σε", _, _;
        vocative_masculine: "τᾱν", _, _;
        vocative_feminine: "τᾱν", _, _;
        vocative_neuter: "τᾱν", _, _;
    ),
    // Τῐς: 1 words (τῐς).
    declension!("τῐς", 1, "~ῐς";
        nominative_masculine: "~ῐς", "~ῐνε|~ῐνε", "~ῐνες|~ῐνες";
        nominative_feminine: "~ῐς", "~ῐνε|~ῐνε", "~ῐνες|~ῐνες";
        nominative_neuter: _, "~ῐνε|~ῐνε", "~ῐνᾰ|~ῐνᾰ|ἀ~τᾰ";
        genitive_masculine: "~ῐνος|~ῐνος|~ου", "~ῐνοιν|~ῐνοῑν", "~ῐνων|~ῐνων";
        genitive_feminine: "~ῐνος|~ῐνος|~ου", "~ῐνοιν|~ῐνοῑν", "~ῐνων|~ῐνων";
        genitive_neuter: "~ῐνος|~ῐνος|~ου", "~ῐνοιν|~ῐνοῑν", "~ῐνων|~ῐνων";
        dative_masculine: "~ῐνῐ|~ῐνῐ|~ῳ", "~ῐνοιν|~ῐνοῑν", "~ῐσῐ|~ῐσῐν|~ῐσῐ|~ῐσῐν";
        dative_feminine: "~ῐνῐ|~ῐνῐ|~ῳ", "~ῐνοιν|~ῐνοῑν", "~ῐσῐ|~ῐσῐν|~ῐσῐ|~ῐσῐν";
        dative_neuter: "~ῐνῐ|~ῐνῐ|~ῳ", "~ῐνοιν|~ῐνοῑν", "~ῐσῐ|~ῐσῐν|~ῐσῐ|~ῐσῐν";
        accusative_masculine: "~ῐνᾰ|~ῐνᾰ", "~ῐνε|~ῐνε", "~ῐνᾰς|~ῐνᾰς";
        accusative_feminine: "~ῐνᾰ|~ῐνᾰ", "~ῐνε|~ῐνε", "~ῐνᾰς|~ῐνᾰς";
        accusative_neuter: "~ῐ", "~ῐνε|~ῐνε", "~ῐνᾰ|~ῐνᾰ|ἀ~τᾰ";
    ),
    // Ὁς: 1 words (ὁς). Like Ὁ, except in 20 cells: nominative_masculine sg, nominative_neuter pl, genitive_masculine sg, genitive_masculine du, genitive_masculine pl, genitive_feminine sg, ….
    declension!("ὁς", 1, "ὁς", like Ὁ;
        nominative_masculine: "ὁς", _, _;
        nominative_neuter: _, _, "ἁ";
        genitive_masculine: "οὑ", "οἱν", "ὡν";
        genitive_feminine: "ἡς", "αἱν", "ὡν";
        dative_masculine: "ᾡ", _, "οἱς";
        dative_feminine: "ᾑ", _, "αἱς";
        accusative_masculine: "ὁν", "ὡ", "οὑς";
        accusative_feminine: "ἡν", "ἁ", "ἁς";
        accusative_neuter: "ὁ", _, "ἁ";
    ),
    // Ὁστῐς: 1 words (ὁστῐς).
    declension!("ὁστῐς", 1, "ὁσ~ῐς";
        nominative_masculine: "ὁσ~ῐς", _, "οἱ~ῐνες";
        nominative_feminine: "ἡ~ῐς", _, "αἱ~ῐνες";
        nominative_neuter: _, _, "ἁ~ῐνᾰ|ἁ~τᾰ";
        genitive_masculine: "οὑ~ῐνος|ὁ~ου", "οἱν~ῐνοιν", "ὡν~ῐνων|ὁ~ων";
        genitive_feminine: "ἡσ~ῐνος", "αἱν~ῐνοιν", "ὡν~ῐνων|ὁ~ων";
        genitive_neuter: "οὑ~ῐνος|ὁ~ου", _, "ὡν~ῐνων|ὁ~ων";
        dative_masculine: "ᾡ~ῐνῐ|ὁ~ῳ", _, "οἱσ~ῐσῐ|οἱσ~ῐσῐν|ὁ~οις";
        dative_feminine: "ᾑ~ῐνῐ", _, "αἱσ~ῐσῐ|αἱσ~ῐσῐν";
        dative_neuter: "ᾡ~ῐνῐ|ὁ~ῳ", _, "οἱσ~ῐσῐ|οἱσ~ῐσῐν|ὁ~οις";
        accusative_masculine: "ὁν~ῐνᾰ", "ὡ~ῐνε", "οὑσ~ῐνᾰς";
        accusative_feminine: "ἡν~ῐνᾰ", "ἁ~ῐνε", "ἁσ~ῐνᾰς";
        accusative_neuter: _, _, "ἁ~ῐνᾰ|ἁ~τᾰ";
    ),
    // Οὐδεις: 1 words (οὐδεις).
    declension!("οὐδεις", 1, "~ις";
        nominative_masculine: "~ις", _, "~νες";
        nominative_feminine: "~μιᾰ", _, "~μιαι";
        nominative_neuter: _, _, "~νᾰ";
        genitive_masculine: "~νος", "~νοιν", "~νων";
        genitive_feminine: "~μιᾱς", "~μιαιν", "~μιων";
        dative_masculine: "~νι", _, "~σι|~σιν";
        dative_feminine: "~μιᾳ", _, "~μιαις";
        dative_neuter: _, _, "~σι|~σιν";
        accusative_masculine: "~νᾰ", "~νε", "~νᾰς";
        accusative_feminine: "~μιᾰν", "~μιᾱ", "~μῐᾱς";
        accusative_neuter: "~ν", _, "~νᾰ";
        vocative_masculine: "~ις", _, _;
    ),
    // Ἱ: 1 words (ἱ). Like Σῠ, except in 15 cells: nominative_masculine sg, nominative_feminine sg, nominative_neuter sg, genitive_masculine sg, genitive_feminine sg, genitive_neuter sg, ….
    declension!("ἱ", 1, "ἱ", like Σῠ;
        nominative_masculine: "ἱ|ἱ", _, _;
        nominative_feminine: "ἱ|ἱ", _, _;
        nominative_neuter: "ἱ|ἱ", _, _;
        genitive_masculine: "οὑ|οὑ", _, _;
        genitive_feminine: "οὑ|οὑ", _, _;
        genitive_neuter: "οὑ|οὑ", _, _;
        dative_masculine: "ἱν|ἱν", _, _;
        dative_feminine: "ἱν|ἱν", _, _;
        dative_neuter: "ἱν|ἱν", _, _;
        accusative_masculine: derived, _, _;
        accusative_feminine: derived, _, _;
        accusative_neuter: derived, _, _;
        vocative_masculine: derived, _, _;
        vocative_feminine: derived, _, _;
        vocative_neuter: derived, _, _;
    ),
    // Πᾱς: 1 words (πᾱς).
    declension!("πᾱς", 1, "~ᾱς";
        nominative_masculine: "~ᾱς", _, "~ᾰντες";
        nominative_feminine: "~ᾱσᾰ", _, "~ᾱσαι";
        nominative_neuter: _, _, "~ᾰντᾰ";
        genitive_masculine: "~ᾰντος", "~ᾰντοιν", "~ᾰντων";
        genitive_feminine: "~ᾱσης", "~ᾱσαιν", "~ᾱσων";
        dative_masculine: "~ᾰντῐ", _, "~ᾱσῐ|~ᾱσῐν";
        dative_feminine: "~ᾱσῃ", _, "~ᾱσαις";
        dative_neuter: _, _, "~ᾱσῐ|~ᾱσῐν";
        accusative_masculine: "~ᾰντᾰ", "~ᾰντε", "~ᾰντᾰς";
        accusative_feminine: "~ᾱσᾰν", "~ᾱσᾱ", "~ᾱσᾱς";
        accusative_neuter: "~ᾱν", _, "~ᾰντᾰ";
        vocative_masculine: "~ᾱς", _, _;
    ),
    // Οὑ: 1 words (οὑ). Like Ἱ, except in 20 cells: nominative_masculine sg, nominative_masculine pl, nominative_feminine sg, nominative_feminine pl, nominative_neuter sg, nominative_neuter pl, ….
    declension!("οὑ", 1, "οὑ", like Ἱ;
        nominative_masculine: "οὑ", _, "σφεῑς";
        nominative_feminine: derived, _, "σφεῑς";
        nominative_neuter: derived, _, "σφεῑς";
        genitive_masculine: _, _, "σφων";
        genitive_feminine: _, _, "σφων";
        dative_masculine: "οἱ|οἱ", _, "σφῐσῐ|σφῐσῐν";
        dative_feminine: "οἱ|οἱ", _, "σφῐσῐ|σφῐσῐν";
        dative_neuter: "οἱ|οἱ", _, "σφῐσῐ|σφῐσῐν";
        accusative_masculine: "ἑ|ἑ", _, "σφᾱς";
        accusative_feminine: "ἑ|ἑ", _, "σφᾱς";
        accusative_neuter: "ἑ|ἑ", _, "σφᾱς";
    ),
    // Ἁπας: 1 words (ἁπας). Like Πᾱς, except in 5 cells: genitive_masculine du, genitive_feminine du, accusative_masculine du, accusative_feminine du, accusative_neuter sg.
    declension!("ἁπας", 1, "~ας", like Πᾱς;
        genitive_masculine: _, derived, _;
        genitive_feminine: _, derived, _;
        accusative_masculine: _, derived, _;
        accusative_feminine: _, derived, _;
        accusative_neuter: "~ᾰν", _, _;
    ),
    // Ὁδε: 1 words (ὁδε).
    declension!("ὁδε", 1, "ὁ~";
        nominative_masculine: "ὁ~", _, "οἱ~";
        nominative_feminine: "ἡ~", _, "αἱ~";
        nominative_neuter: _, _, "τᾰ~";
        genitive_masculine: "τοῡ~", "τοῑν~", "των~";
        genitive_feminine: "τησ~", "τοῑν~|ταῑν~", "των~";
        dative_masculine: "τῳ~", _, "τοῑσ~";
        dative_feminine: "τῃ~", "τοῑν~|ταῑν~", "ταῑσ~";
        accusative_masculine: "τον~", "τω~", "τουσ~";
        accusative_feminine: "την~", "τω~", "τᾱσ~";
        accusative_neuter: "το~", _, "τᾰ~";
    ),
    // Μηδεις: 1 words (μηδεις). Like Ἐμαυτοῡ, except in 9 cells: nominative_masculine sg, nominative_feminine sg, genitive_masculine sg, genitive_feminine sg, dative_masculine sg, dative_feminine sg, ….
    declension!("μηδεις", 1, "~ις", like Ἐμαυτοῡ;
        nominative_masculine: "~ις", _, _;
        nominative_feminine: "~μῐᾰ", _, _;
        genitive_masculine: "~νος", _, _;
        genitive_feminine: "~μῐᾱς", _, _;
        dative_masculine: "~νῐ", _, _;
        dative_feminine: "~μῐᾳ", _, _;
        accusative_masculine: "~νᾰ", _, _;
        accusative_feminine: "~μῐᾰν", _, _;
        accusative_neuter: "~ν", _, _;
    ),
    // Ἀλληλων: 1 words (ἀλληλων). Like Σφω, except in 11 cells: nominative_masculine sg, genitive_masculine du, genitive_masculine pl, genitive_feminine du, genitive_feminine pl, dative_masculine pl, ….
    declension!("ἀλληλων", 1, "~ων", like Σφω;
        nominative_masculine: "~ων", _, _;
        genitive_masculine: _, "~οιν", "~ων";
        genitive_feminine: _, "~αιν", "~ων";
        dative_masculine: _, _, "~οις";
        dative_feminine: _, _, "~αις";
        accusative_masculine: _, _, "~ους";
        accusative_feminine: _, "~ᾱ", "~ᾱς";
        accusative_neuter: _, _, "~ᾰ";
    ),
    // Ὁσπερ: 1 words (ὁσπερ). Like Ὁδε, except in 21 cells: nominative_masculine sg, nominative_neuter pl, genitive_masculine sg, genitive_masculine du, genitive_masculine pl, genitive_feminine sg, ….
    declension!("ὁσπερ", 1, "ὁσ~", like Ὁδε;
        nominative_masculine: "ὁσ~", _, _;
        nominative_neuter: _, _, "ἁ~";
        genitive_masculine: "οὑ~", "οἱν~", "ὡν~";
        genitive_feminine: "ἡσ~", "αἱν~", "ὡν~";
        dative_masculine: "ᾡ~", _, "οἱσ~";
        dative_feminine: "ᾑ~", derived, "αἱσ~";
        accusative_masculine: "ὁν~", "ὡ~", "οὑσ~";
        accusative_feminine: "ἡν~", "ἁ~", "ἁσ~";
        accusative_neuter: "ὁ~", _, "ἁ~";
    ),
    // Σφεῑς: 1 words (σφεῑς). Like Ἡμεῑς, except in 3 cells: dative_masculine pl, dative_feminine pl, dative_neuter pl.
    declension!("σφεῑς", 1, "~εῑς", like Ἡμεῑς;
        dative_masculine: _, _, "~ῐσῐ|~ῐσῐν";
        dative_feminine: _, _, "~ῐσῐ|~ῐσῐν";
        dative_neuter: _, _, "~ῐσῐ|~ῐσῐν";
    ),
    // Ἐνῐοι: 1 words (ἐνῐοι). Like Ἡμεῑς, except in 8 cells: nominative_masculine sg, nominative_masculine pl, nominative_feminine pl, nominative_neuter pl, dative_masculine pl, dative_feminine pl, ….
    declension!("ἐνῐοι", 1, "~οι", like Ἡμεῑς;
        nominative_masculine: "~οι", _, "~οι";
        nominative_feminine: _, _, "~αι";
        nominative_neuter: _, _, "~ᾰ";
        dative_masculine: _, _, "~οις";
        dative_feminine: _, _, "~αις";
        accusative_masculine: _, _, "~ους";
        accusative_neuter: _, _, "~ᾰ";
    ),
    // Ὁσᾰπλᾰσῐων: 1 words (ὁσᾰπλᾰσῐων).
    declension!("ὁσᾰπλᾰσῐων", 1, "~ων";
        nominative_masculine: "~ων", _, "~ονες";
        nominative_feminine: "~ων", _, "~ονες";
        nominative_neuter: _, _, "~ονᾰ";
        genitive_masculine: "~ονος", "~ονοιν", "~ονων";
        genitive_feminine: "~ονος", "~ονοιν", "~ονων";
        dative_masculine: "~ονῐ", _, "~οσῐ|~οσῐν";
        dative_feminine: "~ονῐ", _, "~οσῐ|~οσῐν";
        dative_neuter: _, _, "~οσῐ|~οσῐν";
        accusative_masculine: "~ονᾰ", "~ονε", "~ονᾰς";
        accusative_feminine: "~ονᾰ", "~ονε", "~ονᾰς";
        accusative_neuter: "~ον", _, "~ονᾰ";
        vocative_masculine: "~ον", _, _;
        vocative_feminine: "~ον", _, _;
    ),
];
