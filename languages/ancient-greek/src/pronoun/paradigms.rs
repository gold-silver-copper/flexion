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
    /// Like `ὁσος`: 16 words.
    Ὁσος,
    /// Like `οἱος`: 6 words.
    Οἱος,
    /// Like `ἑαυτοῡ`: 6 words.
    Ἑαυτοῡ,
    /// Like `τοῑος`: 4 words.
    Τοῑος,
    /// Like `ἐμαυτοῡ`: 3 words.
    Ἐμαυτοῡ,
    /// Like `σφω`: 2 words.
    Σφω,
    /// Like `ὁ`: 1 words.
    Ὁ,
    /// Like `ἐγω`: 1 words.
    Ἐγω,
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
    /// Like `ἡμεῑς`: 1 words.
    Ἡμεῑς,
    /// Like `ὑμεῑς`: 1 words.
    Ὑμεῑς,
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
    /// Like `ἀμφω`: 1 words.
    Ἀμφω,
    /// Like `ἐνῐοι`: 1 words.
    Ἐνῐοι,
    /// Like `ὁσᾰπλᾰσῐων`: 1 words.
    Ὁσᾰπλᾰσῐων,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 27] = [
        Declension::Ὁσος,
        Declension::Οἱος,
        Declension::Ἑαυτοῡ,
        Declension::Τοῑος,
        Declension::Ἐμαυτοῡ,
        Declension::Σφω,
        Declension::Ὁ,
        Declension::Ἐγω,
        Declension::Σῠ,
        Declension::Τῐς,
        Declension::Ὁς,
        Declension::Ὁστῐς,
        Declension::Οὐδεις,
        Declension::Ἱ,
        Declension::Ἡμεῑς,
        Declension::Ὑμεῑς,
        Declension::Πᾱς,
        Declension::Οὑ,
        Declension::Ἁπας,
        Declension::Ὁδε,
        Declension::Μηδεις,
        Declension::Ἀλληλων,
        Declension::Ὁσπερ,
        Declension::Σφεῑς,
        Declension::Ἀμφω,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 27] = [
    // Ὁσος: 16 words (αὐτος, οὑτος, ἑος, ἐκεῑνος, ἑκᾰστος, ταὐτος).
    declension!("ὁσος", 16, "~ος";
        nominative_masculine: "~ος", "~ω", "~οι";
        nominative_feminine: "~η", "~ᾱ", "~αι";
        nominative_neuter: "~ον", "~ω", "~ᾰ";
        genitive_masculine: "~ου", "~οιν", "~ων";
        genitive_feminine: "~ης", "~αιν", "~ων";
        genitive_neuter: "~ου", "~οιν", "~ων";
        dative_masculine: "~ῳ", "~οιν", "~οις";
        dative_feminine: "~ῃ", "~αιν", "~αις";
        dative_neuter: "~ῳ", "~οιν", "~οις";
        accusative_masculine: "~ον", "~ω", "~ους";
        accusative_feminine: "~ην", "~ᾱ", "~ᾱς";
        accusative_neuter: "~ον", "~ω", "~ᾰ";
        vocative_masculine: "~ε", "~ω", "~οι";
        vocative_feminine: "~η", "~ᾱ", "~αι";
        vocative_neuter: "~ον", "~ω", "~ᾰ";
    ),
    // Οἱος: 6 words (οἱος, ἑκᾰτερος, οὐδετερος, ὁποτερος, μηδετερος, αὐτοτερος). Like Ὁσος, except in 5 cells: nominative_feminine sg, genitive_feminine sg, dative_feminine sg, accusative_feminine sg, vocative_feminine sg.
    declension!("οἱος", 6, "~ος", like Ὁσος;
        nominative_masculine: _, _, _;
        nominative_feminine: "~ᾱ", _, _;
        nominative_neuter: _, _, _;
        genitive_masculine: _, _, _;
        genitive_feminine: "~ᾱς", _, _;
        genitive_neuter: _, _, _;
        dative_masculine: _, _, _;
        dative_feminine: "~ᾳ", _, _;
        dative_neuter: _, _, _;
        accusative_masculine: _, _, _;
        accusative_feminine: "~ᾱν", _, _;
        accusative_neuter: _, _, _;
        vocative_masculine: _, _, _;
        vocative_feminine: "~ᾱ", _, _;
        vocative_neuter: _, _, _;
    ),
    // Ἑαυτοῡ: 6 words (ἑαυτοῡ, ἑωυτοῡ, αὑτοῡ, σεαυτοῡ, σεωυτοῡ, σαυτοῡ).
    declension!("ἑαυτοῡ", 6, "~οῡ";
        nominative_masculine: "~οῡ", derived, derived;
        nominative_feminine: derived, derived, derived;
        nominative_neuter: derived, derived, derived;
        genitive_masculine: "~οῡ", "~οῑν", "~ων";
        genitive_feminine: "~ης", "~αῑν", "~εων";
        genitive_neuter: "~οῡ", "~οῑν", "~ων";
        dative_masculine: "~ῳ", "~οῑν", "~οῑς";
        dative_feminine: "~ῃ", "~αῑν", "~ῃσῐ";
        dative_neuter: "~ῳ", "~οῑν", "~οῑς";
        accusative_masculine: "~ον", "~ω", "~ους";
        accusative_feminine: "~ην", "~ᾱ", "~ᾱς";
        accusative_neuter: "~ο", "~ω", "~ᾰ";
        vocative_masculine: derived, derived, derived;
        vocative_feminine: derived, derived, derived;
        vocative_neuter: derived, derived, derived;
    ),
    // Τοῑος: 4 words (ποῑος, τοῑος, πᾰντοῑος, ὁποῑος).
    declension!("τοῑος", 4, "~ῑος";
        nominative_masculine: "~ῑος", "~ιω", "~ῑοι";
        nominative_feminine: "~ιᾱ", "~ιᾱ", "~ῑαι";
        nominative_neuter: "~ῑον", "~ιω", "~ῑᾰ";
        genitive_masculine: "~ιου", "~ιοιν", "~ιων";
        genitive_feminine: "~ιᾱς", "~ιαιν", "~ιων";
        genitive_neuter: "~ιου", "~ιοιν", "~ιων";
        dative_masculine: "~ιῳ", "~ιοιν", "~ιοις";
        dative_feminine: "~ιᾳ", "~ιαιν", "~ιαις";
        dative_neuter: "~ιῳ", "~ιοιν", "~ιοις";
        accusative_masculine: "~ῑον", "~ιω", "~ιους";
        accusative_feminine: "~ιᾱν", "~ιᾱ", "~ιᾱς";
        accusative_neuter: "~ῑον", "~ιω", "~ῑᾰ";
        vocative_masculine: "~ῑε", "~ιω", "~ῑοι";
        vocative_feminine: "~ιᾱ", "~ιᾱ", "~ῑαι";
        vocative_neuter: "~ῑον", "~ιω", "~ῑᾰ";
    ),
    // Ἐμαυτοῡ: 3 words (ἐμαυτοῡ, ἐμεωυτοῡ, ἐμωυτοῡ). Like Ἑαυτοῡ, except in 19 cells: genitive_masculine du, genitive_masculine pl, genitive_feminine du, genitive_feminine pl, genitive_neuter du, genitive_neuter pl, ….
    declension!("ἐμαυτοῡ", 3, "~οῡ", like Ἑαυτοῡ;
        nominative_masculine: _, _, _;
        nominative_feminine: _, _, _;
        nominative_neuter: _, _, _;
        genitive_masculine: _, derived, derived;
        genitive_feminine: _, derived, derived;
        genitive_neuter: _, derived, derived;
        dative_masculine: _, derived, derived;
        dative_feminine: _, derived, derived;
        dative_neuter: _, derived, derived;
        accusative_masculine: _, derived, derived;
        accusative_feminine: _, derived, derived;
        accusative_neuter: "~ον", derived, derived;
        vocative_masculine: _, _, _;
        vocative_feminine: _, _, _;
        vocative_neuter: _, _, _;
    ),
    // Σφω: 2 words (νω, σφω).
    declension!("σφω", 2, "~ω";
        nominative_masculine: "~ω", "~ω", derived;
        nominative_feminine: derived, "~ω", derived;
        nominative_neuter: derived, "~ω", derived;
        genitive_masculine: derived, "~ῳν", derived;
        genitive_feminine: derived, "~ῳν", derived;
        genitive_neuter: derived, "~ῳν", derived;
        dative_masculine: derived, "~ῳν", derived;
        dative_feminine: derived, "~ῳν", derived;
        dative_neuter: derived, "~ῳν", derived;
        accusative_masculine: derived, "~ω", derived;
        accusative_feminine: derived, "~ω", derived;
        accusative_neuter: derived, "~ω", derived;
        vocative_masculine: derived, "~ω", derived;
        vocative_feminine: derived, "~ω", derived;
        vocative_neuter: derived, "~ω", derived;
    ),
    // Ὁ: 1 words (ὁ).
    declension!("ὁ", 1, "ὁ";
        nominative_masculine: "ὁ", "τω", "οἱ";
        nominative_feminine: "ἡ", "τω", "αἱ";
        nominative_neuter: "το", "τω", "τᾰ";
        genitive_masculine: "τοῡ", "τοῑν", "των";
        genitive_feminine: "της", "τοῑν", "των";
        genitive_neuter: "τοῡ", "τοῑν", "των";
        dative_masculine: "τῳ", "τοῑν", "τοῑς";
        dative_feminine: "τῃ", "τοῑν", "ταῑς";
        dative_neuter: "τῳ", "τοῑν", "τοῑς";
        accusative_masculine: "τον", "τω", "τους";
        accusative_feminine: "την", "τω", "τᾱς";
        accusative_neuter: "το", "τω", "τᾰ";
        vocative_masculine: derived, derived, derived;
        vocative_feminine: derived, derived, derived;
        vocative_neuter: derived, derived, derived;
    ),
    // Ἐγω: 1 words (ἐγω).
    declension!("ἐγω", 1, "ἐγω";
        nominative_masculine: "ἐγω", derived, derived;
        nominative_feminine: "ἐγω", derived, derived;
        nominative_neuter: "ἐγω", derived, derived;
        genitive_masculine: "ἐμοῡ|μου", derived, derived;
        genitive_feminine: "ἐμοῡ|μου", derived, derived;
        genitive_neuter: "ἐμοῡ|μου", derived, derived;
        dative_masculine: "ἐμοι|μοι", derived, derived;
        dative_feminine: "ἐμοι|μοι", derived, derived;
        dative_neuter: "ἐμοι|μοι", derived, derived;
        accusative_masculine: "ἐμε|με", derived, derived;
        accusative_feminine: "ἐμε|με", derived, derived;
        accusative_neuter: "ἐμε|με", derived, derived;
        vocative_masculine: derived, derived, derived;
        vocative_feminine: derived, derived, derived;
        vocative_neuter: derived, derived, derived;
    ),
    // Σῠ: 1 words (σῠ). Like Ἐγω, except in 15 cells: nominative_masculine sg, nominative_feminine sg, nominative_neuter sg, genitive_masculine sg, genitive_feminine sg, genitive_neuter sg, ….
    declension!("σῠ", 1, "σῠ", like Ἐγω;
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
        nominative_neuter: "~ῐ", "~ῐνε|~ῐνε", "~ῐνᾰ|~ῐνᾰ|ᾰ̓~τᾰ";
        genitive_masculine: "~ῐνος|~ῐνος|~ου", "~ῐνοιν|~ῐνοῑν", "~ῐνων|~ῐνων";
        genitive_feminine: "~ῐνος|~ῐνος|~ου", "~ῐνοιν|~ῐνοῑν", "~ῐνων|~ῐνων";
        genitive_neuter: "~ῐνος|~ῐνος|~ου", "~ῐνοιν|~ῐνοῑν", "~ῐνων|~ῐνων";
        dative_masculine: "~ῐνῐ|~ῐνῐ|~ῳ", "~ῐνοιν|~ῐνοῑν", "~ῐσῐ|~ῐσῐν|~ῐσῐ|~ῐσῐν";
        dative_feminine: "~ῐνῐ|~ῐνῐ|~ῳ", "~ῐνοιν|~ῐνοῑν", "~ῐσῐ|~ῐσῐν|~ῐσῐ|~ῐσῐν";
        dative_neuter: "~ῐνῐ|~ῐνῐ|~ῳ", "~ῐνοιν|~ῐνοῑν", "~ῐσῐ|~ῐσῐν|~ῐσῐ|~ῐσῐν";
        accusative_masculine: "~ῐνᾰ|~ῐνᾰ", "~ῐνε|~ῐνε", "~ῐνᾰς|~ῐνᾰς";
        accusative_feminine: "~ῐνᾰ|~ῐνᾰ", "~ῐνε|~ῐνε", "~ῐνᾰς|~ῐνᾰς";
        accusative_neuter: "~ῐ", "~ῐνε|~ῐνε", "~ῐνᾰ|~ῐνᾰ|ᾰ̓~τᾰ";
        vocative_masculine: derived, derived, derived;
        vocative_feminine: derived, derived, derived;
        vocative_neuter: derived, derived, derived;
    ),
    // Ὁς: 1 words (ὁς).
    declension!("ὁς", 1, "ὁς";
        nominative_masculine: "ὁς", "ὡ", "οἱ";
        nominative_feminine: "ἡ", "ᾱ̔", "αἱ";
        nominative_neuter: "ὁ", "ὡ", "ᾰ̔";
        genitive_masculine: "οὑ", "οἱν", "ὡν";
        genitive_feminine: "ἡς", "αἱν", "ὡν";
        genitive_neuter: "οὑ", "οἱν", "ὡν";
        dative_masculine: "ᾡ", "οἱν", "οἱς";
        dative_feminine: "ᾑ", "αἱν", "αἱς";
        dative_neuter: "ᾡ", "οἱν", "οἱς";
        accusative_masculine: "ὁν", "ὡ", "οὑς";
        accusative_feminine: "ἡν", "ᾱ̔", "ᾱ̔ς";
        accusative_neuter: "ὁ", "ὡ", "ᾰ̔";
        vocative_masculine: derived, derived, derived;
        vocative_feminine: derived, derived, derived;
        vocative_neuter: derived, derived, derived;
    ),
    // Ὁστῐς: 1 words (ὁστῐς).
    declension!("ὁστῐς", 1, "ὁσ~ῐς";
        nominative_masculine: "ὁσ~ῐς", "ὡ~ῐνε", "οἱ~ῐνες";
        nominative_feminine: "ἡ~ῐς", "ᾱ̔~ῐνε", "αἱ~ῐνες";
        nominative_neuter: derived, "ὡ~ῐνε", "ᾰ̔~ῐνᾰ|ᾰ̔~τᾰ";
        genitive_masculine: "οὑ~ῐνος|ὁ~ου", "οἱν~ῐνοιν", "ὡν~ῐνων|ὁ~ων";
        genitive_feminine: "ἡσ~ῐνος", "αἱν~ῐνοιν", "ὡν~ῐνων|ὁ~ων";
        genitive_neuter: "οὑ~ῐνος|ὁ~ου", "οἱν~ῐνοιν", "ὡν~ῐνων|ὁ~ων";
        dative_masculine: "ᾡ~ῐνῐ|ὁ~ῳ", "οἱν~ῐνοιν", "οἱσ~ῐσῐ|οἱσ~ῐσῐν|ὁ~οις";
        dative_feminine: "ᾑ~ῐνῐ", "αἱν~ῐνοιν", "αἱσ~ῐσῐ|αἱσ~ῐσῐν";
        dative_neuter: "ᾡ~ῐνῐ|ὁ~ῳ", "οἱν~ῐνοιν", "οἱσ~ῐσῐ|οἱσ~ῐσῐν|ὁ~οις";
        accusative_masculine: "ὁν~ῐνᾰ", "ὡ~ῐνε", "οὑσ~ῐνᾰς";
        accusative_feminine: "ἡν~ῐνᾰ", "ᾱ̔~ῐνε", "ᾱ̔σ~ῐνᾰς";
        accusative_neuter: derived, "ὡ~ῐνε", "ᾰ̔~ῐνᾰ|ᾰ̔~τᾰ";
        vocative_masculine: derived, derived, derived;
        vocative_feminine: derived, derived, derived;
        vocative_neuter: derived, derived, derived;
    ),
    // Οὐδεις: 1 words (οὐδεις).
    declension!("οὐδεις", 1, "~ις";
        nominative_masculine: "~ις", "~νε", "~νες";
        nominative_feminine: "~μιᾰ", "~μιᾱ", "~μιαι";
        nominative_neuter: "~ν", "~νε", "~νᾰ";
        genitive_masculine: "~νος", "~νοιν", "~νων";
        genitive_feminine: "~μιᾱς", "~μιαιν", "~μιων";
        genitive_neuter: "~νος", "~νοιν", "~νων";
        dative_masculine: "~νι", "~νοιν", "~σι|~σιν";
        dative_feminine: "~μιᾳ", "~μιαιν", "~μιαις";
        dative_neuter: "~νι", "~νοιν", "~σι|~σιν";
        accusative_masculine: "~νᾰ", "~νε", "~νᾰς";
        accusative_feminine: "~μιᾰν", "~μιᾱ", "~μῐᾱς";
        accusative_neuter: "~ν", "~νε", "~νᾰ";
        vocative_masculine: "~ις", "~νε", "~νες";
        vocative_feminine: "~μιᾰ", "~μιᾱ", "~μιαι";
        vocative_neuter: "~ν", "~νε", "~νᾰ";
    ),
    // Ἱ: 1 words (ἱ). Like Ἐγω, except in 12 cells: nominative_masculine sg, nominative_feminine sg, nominative_neuter sg, genitive_masculine sg, genitive_feminine sg, genitive_neuter sg, ….
    declension!("ἱ", 1, "ἱ", like Ἐγω;
        nominative_masculine: "ῑ̔|ῑ̔", _, _;
        nominative_feminine: "ῑ̔|ῑ̔", _, _;
        nominative_neuter: "ῑ̔|ῑ̔", _, _;
        genitive_masculine: "οὑ|οῡ̔", _, _;
        genitive_feminine: "οὑ|οῡ̔", _, _;
        genitive_neuter: "οὑ|οῡ̔", _, _;
        dative_masculine: "ῐ̔ν|ῐ̔ν", _, _;
        dative_feminine: "ῐ̔ν|ῐ̔ν", _, _;
        dative_neuter: "ῐ̔ν|ῐ̔ν", _, _;
        accusative_masculine: derived, _, _;
        accusative_feminine: derived, _, _;
        accusative_neuter: derived, _, _;
        vocative_masculine: _, _, _;
        vocative_feminine: _, _, _;
        vocative_neuter: _, _, _;
    ),
    // Ἡμεῑς: 1 words (ἡμεῑς). Like Ἐμαυτοῡ, except in 22 cells: nominative_masculine sg, nominative_masculine pl, nominative_feminine pl, nominative_neuter pl, genitive_masculine sg, genitive_masculine pl, ….
    declension!("ἡμεῑς", 1, "ἡ~εῑς", like Ἐμαυτοῡ;
        nominative_masculine: "ἡ~εῑς", _, "ἡ~εῑς|ᾰ̓~μες|ᾰ̓~μες";
        nominative_feminine: _, _, "ἡ~εῑς|ᾰ̓~μες|ᾰ̓~μες";
        nominative_neuter: _, _, "ἡ~εῑς|ᾰ̓~μες|ᾰ̓~μες";
        genitive_masculine: derived, _, "ἡ~ων|ᾰ̓~μεων|ᾰ̓~μεουν";
        genitive_feminine: derived, _, "ἡ~ων|ᾰ̓~μεων|ᾰ̓~μεουν";
        genitive_neuter: derived, _, "ἡ~ων|ᾰ̓~μεων|ᾰ̓~μεουν";
        dative_masculine: derived, _, "ἡ~ῑν|ᾰ̓~μῐ|ᾰ̓~μῐν|ᾰ̓~μῐν";
        dative_feminine: derived, _, "ἡ~ῑν|ᾰ̓~μῐ|ᾰ̓~μῐν|ᾰ̓~μῐν";
        dative_neuter: derived, _, "ἡ~ῑν|ᾰ̓~μῐ|ᾰ̓~μῐν|ᾰ̓~μῐν";
        accusative_masculine: derived, _, "ἡ~ᾱς|ᾰ̓~με|ᾰ̓~με";
        accusative_feminine: derived, _, "ἡ~ᾱς|ᾰ̓~με|ᾰ̓~με";
        accusative_neuter: derived, _, "ἡ~ᾱς|ᾰ̓~με|ᾰ̓~με";
        vocative_masculine: _, _, _;
        vocative_feminine: _, _, _;
        vocative_neuter: _, _, _;
    ),
    // Ὑμεῑς: 1 words (ὑμεῑς). Like Ἡμεῑς, except in 16 cells: nominative_masculine sg, nominative_masculine pl, nominative_feminine pl, nominative_neuter pl, genitive_masculine pl, genitive_feminine pl, ….
    declension!("ὑμεῑς", 1, "ὑ~εῑς", like Ἡμεῑς;
        nominative_masculine: "ὑ~εῑς", _, "ῡ̔~εῑς";
        nominative_feminine: _, _, "ῡ̔~εῑς";
        nominative_neuter: _, _, "ῡ̔~εῑς";
        genitive_masculine: _, _, "ῡ̔~ων";
        genitive_feminine: _, _, "ῡ̔~ων";
        genitive_neuter: _, _, "ῡ̔~ων";
        dative_masculine: _, _, "ῡ̔~ῑν";
        dative_feminine: _, _, "ῡ̔~ῑν";
        dative_neuter: _, _, "ῡ̔~ῑν";
        accusative_masculine: _, _, "ῡ̔~ᾱς";
        accusative_feminine: _, _, "ῡ̔~ᾱς";
        accusative_neuter: _, _, "ῡ̔~ᾱς";
        vocative_masculine: _, _, "ῡ̔~εῑς";
        vocative_feminine: _, _, "ῡ̔~εῑς";
        vocative_neuter: _, _, "ῡ̔~εῑς";
    ),
    // Πᾱς: 1 words (πᾱς).
    declension!("πᾱς", 1, "~ᾱς";
        nominative_masculine: "~ᾱς", "~ᾰντε", "~ᾰντες";
        nominative_feminine: "~ᾱσᾰ", "~ᾱσᾱ", "~ᾱσαι";
        nominative_neuter: "~ᾱν", "~ᾰντε", "~ᾰντᾰ";
        genitive_masculine: "~ᾰντος", "~ᾰντοιν", "~ᾰντων";
        genitive_feminine: "~ᾱσης", "~ᾱσαιν", "~ᾱσων";
        genitive_neuter: "~ᾰντος", "~ᾰντοιν", "~ᾰντων";
        dative_masculine: "~ᾰντῐ", "~ᾰντοιν", "~ᾱσῐ|~ᾱσῐν";
        dative_feminine: "~ᾱσῃ", "~ᾱσαιν", "~ᾱσαις";
        dative_neuter: "~ᾰντῐ", "~ᾰντοιν", "~ᾱσῐ|~ᾱσῐν";
        accusative_masculine: "~ᾰντᾰ", "~ᾰντε", "~ᾰντᾰς";
        accusative_feminine: "~ᾱσᾰν", "~ᾱσᾱ", "~ᾱσᾱς";
        accusative_neuter: "~ᾱν", "~ᾰντε", "~ᾰντᾰ";
        vocative_masculine: "~ᾱς", "~ᾰντε", "~ᾰντες";
        vocative_feminine: "~ᾱσᾰ", "~ᾱσᾱ", "~ᾱσαι";
        vocative_neuter: "~ᾱν", "~ᾰντε", "~ᾰντᾰ";
    ),
    // Οὑ: 1 words (οὑ).
    declension!("οὑ", 1, "οὑ";
        nominative_masculine: "οὑ", derived, "σφεῑς";
        nominative_feminine: derived, derived, "σφεῑς";
        nominative_neuter: derived, derived, "σφεῑς";
        genitive_masculine: "οὑ|οὑ", derived, "σφων";
        genitive_feminine: "οὑ|οὑ", derived, "σφων";
        genitive_neuter: "οὑ|οὑ", derived, "σφων";
        dative_masculine: "οἱ|οἱ", derived, "σφῐσῐ|σφῐσῐν";
        dative_feminine: "οἱ|οἱ", derived, "σφῐσῐ|σφῐσῐν";
        dative_neuter: "οἱ|οἱ", derived, "σφῐσῐ|σφῐσῐν";
        accusative_masculine: "ἑ|ἑ", derived, "σφᾱς";
        accusative_feminine: "ἑ|ἑ", derived, "σφᾱς";
        accusative_neuter: "ἑ|ἑ", derived, "σφᾱς";
        vocative_masculine: derived, derived, "σφεῑς";
        vocative_feminine: derived, derived, "σφεῑς";
        vocative_neuter: derived, derived, "σφεῑς";
    ),
    // Ἁπας: 1 words (ἁπας).
    declension!("ἁπας", 1, "ἁ~ας";
        nominative_masculine: "ᾰ̔~ᾱς", derived, "ᾰ̔~ᾰντες";
        nominative_feminine: "ᾰ̔~ᾱσᾰ", derived, "ᾰ̔~ᾱσαι";
        nominative_neuter: "ᾰ̔~ᾰν", derived, "ᾰ̔~ᾰντᾰ";
        genitive_masculine: "ᾰ̔~ᾰντος", derived, "ᾰ̔~ᾰντων";
        genitive_feminine: "ᾰ̔~ᾱσης", derived, "ᾰ̔~ᾱσων";
        genitive_neuter: "ᾰ̔~ᾰντος", derived, "ᾰ̔~ᾰντων";
        dative_masculine: "ᾰ̔~ᾰντῐ", derived, "ᾰ̔~ᾱσῐ|ᾰ̔~ᾱσῐν";
        dative_feminine: "ᾰ̔~ᾱσῃ", derived, "ᾰ̔~ᾱσαις";
        dative_neuter: "ᾰ̔~ᾰντῐ", derived, "ᾰ̔~ᾱσῐ|ᾰ̔~ᾱσῐν";
        accusative_masculine: "ᾰ̔~ᾰντᾰ", derived, "ᾰ̔~ᾰντᾰς";
        accusative_feminine: "ᾰ̔~ᾱσᾰν", derived, "ᾰ̔~ᾱσᾱς";
        accusative_neuter: "ᾰ̔~ᾰν", derived, "ᾰ̔~ᾰντᾰ";
        vocative_masculine: "ᾰ̔~ᾱς", derived, "ᾰ̔~ᾰντες";
        vocative_feminine: "ᾰ̔~ᾱσᾰ", derived, "ᾰ̔~ᾱσαι";
        vocative_neuter: "ᾰ̔~ᾰν", derived, "ᾰ̔~ᾰντᾰ";
    ),
    // Ὁδε: 1 words (ὁδε).
    declension!("ὁδε", 1, "ὁ~";
        nominative_masculine: "ὁ~", "τω~", "οἱ~";
        nominative_feminine: "ἡ~", "τω~", "αἱ~";
        nominative_neuter: "το~", "τω~", "τᾰ~";
        genitive_masculine: "τοῡ~", "τοῑν~", "των~";
        genitive_feminine: "τησ~", "τοῑν~|ταῑν~", "των~";
        genitive_neuter: "τοῡ~", "τοῑν~", "των~";
        dative_masculine: "τῳ~", "τοῑν~", "τοῑσ~";
        dative_feminine: "τῃ~", "τοῑν~|ταῑν~", "ταῑσ~";
        dative_neuter: "τῳ~", "τοῑν~", "τοῑσ~";
        accusative_masculine: "τον~", "τω~", "τουσ~";
        accusative_feminine: "την~", "τω~", "τᾱσ~";
        accusative_neuter: "το~", "τω~", "τᾰ~";
        vocative_masculine: derived, derived, derived;
        vocative_feminine: derived, derived, derived;
        vocative_neuter: derived, derived, derived;
    ),
    // Μηδεις: 1 words (μηδεις). Like Ἐμαυτοῡ, except in 12 cells: nominative_masculine sg, nominative_feminine sg, nominative_neuter sg, genitive_masculine sg, genitive_feminine sg, genitive_neuter sg, ….
    declension!("μηδεις", 1, "~ις", like Ἐμαυτοῡ;
        nominative_masculine: "~ις", _, _;
        nominative_feminine: "~μῐᾰ", _, _;
        nominative_neuter: "~ν", _, _;
        genitive_masculine: "~νος", _, _;
        genitive_feminine: "~μῐᾱς", _, _;
        genitive_neuter: "~νος", _, _;
        dative_masculine: "~νῐ", _, _;
        dative_feminine: "~μῐᾳ", _, _;
        dative_neuter: "~νῐ", _, _;
        accusative_masculine: "~νᾰ", _, _;
        accusative_feminine: "~μῐᾰν", _, _;
        accusative_neuter: "~ν", _, _;
        vocative_masculine: _, _, _;
        vocative_feminine: _, _, _;
        vocative_neuter: _, _, _;
    ),
    // Ἀλληλων: 1 words (ἀλληλων). Like Ἡμεῑς, except in 22 cells: nominative_masculine sg, nominative_masculine pl, nominative_feminine pl, nominative_neuter pl, genitive_masculine du, genitive_masculine pl, ….
    declension!("ἀλληλων", 1, "ἀ~ων", like Ἡμεῑς;
        nominative_masculine: "ἀ~ων", _, derived;
        nominative_feminine: _, _, derived;
        nominative_neuter: _, _, derived;
        genitive_masculine: _, "ᾰ̓~οιν", "ᾰ̓~ων";
        genitive_feminine: _, "ᾰ̓~αιν", "ᾰ̓~ων";
        genitive_neuter: _, "ᾰ̓~οιν", "ᾰ̓~ων";
        dative_masculine: _, "ᾰ̓~οιν", "ᾰ̓~οις";
        dative_feminine: _, "ᾰ̓~αιν", "ᾰ̓~αις";
        dative_neuter: _, "ᾰ̓~οιν", "ᾰ̓~οις";
        accusative_masculine: _, "ᾰ̓~ω", "ᾰ̓~ους";
        accusative_feminine: _, "ᾰ̓~ᾱ", "ᾰ̓~ᾱς";
        accusative_neuter: _, "ᾰ̓~ω", "ᾰ̓~ᾰ";
        vocative_masculine: _, _, _;
        vocative_feminine: _, _, _;
        vocative_neuter: _, _, _;
    ),
    // Ὁσπερ: 1 words (ὁσπερ).
    declension!("ὁσπερ", 1, "ὁσ~";
        nominative_masculine: "ὁσ~", "ὡ~", "οἱ~";
        nominative_feminine: "ἡ~", "ᾱ̔~", "αἱ~";
        nominative_neuter: "ὁ~", "ὡ~", "ᾰ̔~";
        genitive_masculine: "οὑ~", "οἱν~", "ὡν~";
        genitive_feminine: "ἡσ~", "αἱν~", "ὡν~";
        genitive_neuter: "οὑ~", "οἱν~", "ὡν~";
        dative_masculine: "ᾡ~", "οἱν~", "οἱσ~";
        dative_feminine: "ᾑ~", "αἱν~", "αἱσ~";
        dative_neuter: "ᾡ~", "οἱν~", "οἱσ~";
        accusative_masculine: "ὁν~", "ὡ~", "οὑσ~";
        accusative_feminine: "ἡν~", "ᾱ̔~", "ᾱ̔σ~";
        accusative_neuter: "ὁ~", "ὡ~", "ᾰ̔~";
        vocative_masculine: derived, derived, derived;
        vocative_feminine: derived, derived, derived;
        vocative_neuter: derived, derived, derived;
    ),
    // Σφεῑς: 1 words (σφεῑς). Like Ἡμεῑς, except in 13 cells: nominative_masculine sg, nominative_masculine pl, nominative_feminine pl, nominative_neuter pl, genitive_masculine pl, genitive_feminine pl, ….
    declension!("σφεῑς", 1, "~εῑς", like Ἡμεῑς;
        nominative_masculine: "~εῑς", _, "~εῑς";
        nominative_feminine: _, _, "~εῑς";
        nominative_neuter: _, _, "~εῑς";
        genitive_masculine: _, _, "~ων";
        genitive_feminine: _, _, "~ων";
        genitive_neuter: _, _, "~ων";
        dative_masculine: _, _, "~ῐσῐ|~ῐσῐν";
        dative_feminine: _, _, "~ῐσῐ|~ῐσῐν";
        dative_neuter: _, _, "~ῐσῐ|~ῐσῐν";
        accusative_masculine: _, _, "~ᾱς";
        accusative_feminine: _, _, "~ᾱς";
        accusative_neuter: _, _, "~ᾱς";
        vocative_masculine: _, _, _;
        vocative_feminine: _, _, _;
        vocative_neuter: _, _, _;
    ),
    // Ἀμφω: 1 words (ἀμφω). Like Ἐμαυτοῡ, except in 13 cells: nominative_masculine sg, genitive_masculine sg, genitive_feminine sg, genitive_neuter sg, dative_masculine sg, dative_feminine sg, ….
    declension!("ἀμφω", 1, "ἀ~", like Ἐμαυτοῡ;
        nominative_masculine: "ἀ~", _, _;
        nominative_feminine: _, _, _;
        nominative_neuter: _, _, _;
        genitive_masculine: derived, _, _;
        genitive_feminine: derived, _, _;
        genitive_neuter: derived, _, _;
        dative_masculine: derived, _, _;
        dative_feminine: derived, _, _;
        dative_neuter: derived, _, _;
        accusative_masculine: derived, _, _;
        accusative_feminine: derived, _, _;
        accusative_neuter: derived, _, _;
        vocative_masculine: _, "ᾰ̓~", _;
        vocative_feminine: _, "ᾰ̓~", _;
        vocative_neuter: _, "ᾰ̓~", _;
    ),
    // Ἐνῐοι: 1 words (ἐνῐοι). Like Σφεῑς, except in 12 cells: nominative_masculine sg, nominative_masculine pl, nominative_feminine pl, nominative_neuter pl, dative_masculine pl, dative_feminine pl, ….
    declension!("ἐνῐοι", 1, "~οι", like Σφεῑς;
        nominative_masculine: "~οι", _, "~οι";
        nominative_feminine: _, _, "~αι";
        nominative_neuter: _, _, "~ᾰ";
        genitive_masculine: _, _, _;
        genitive_feminine: _, _, _;
        genitive_neuter: _, _, _;
        dative_masculine: _, _, "~οις";
        dative_feminine: _, _, "~αις";
        dative_neuter: _, _, "~οις";
        accusative_masculine: _, _, "~ους";
        accusative_feminine: _, _, _;
        accusative_neuter: _, _, "~ᾰ";
        vocative_masculine: _, _, "~οι";
        vocative_feminine: _, _, "~αι";
        vocative_neuter: _, _, "~ᾰ";
    ),
    // Ὁσᾰπλᾰσῐων: 1 words (ὁσᾰπλᾰσῐων).
    declension!("ὁσᾰπλᾰσῐων", 1, "~ων";
        nominative_masculine: "~ων", "~ονε", "~ονες";
        nominative_feminine: "~ων", "~ονε", "~ονες";
        nominative_neuter: "~ον", "~ονε", "~ονᾰ";
        genitive_masculine: "~ονος", "~ονοιν", "~ονων";
        genitive_feminine: "~ονος", "~ονοιν", "~ονων";
        genitive_neuter: "~ονος", "~ονοιν", "~ονων";
        dative_masculine: "~ονῐ", "~ονοιν", "~οσῐ|~οσῐν";
        dative_feminine: "~ονῐ", "~ονοιν", "~οσῐ|~οσῐν";
        dative_neuter: "~ονῐ", "~ονοιν", "~οσῐ|~οσῐν";
        accusative_masculine: "~ονᾰ", "~ονε", "~ονᾰς";
        accusative_feminine: "~ονᾰ", "~ονε", "~ονᾰς";
        accusative_neuter: "~ον", "~ονε", "~ονᾰ";
        vocative_masculine: "~ον", "~ονε", "~ονες";
        vocative_feminine: "~ον", "~ονε", "~ονες";
        vocative_neuter: "~ον", "~ονε", "~ονᾰ";
    ),
];
