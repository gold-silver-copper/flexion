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
/// `nom_m` (sg, du, pl)
/// `nom_f` (sg, du, pl)
/// `nom_n` (sg, du, pl)
/// `gen_m` (sg, du, pl)
/// `gen_f` (sg, du, pl)
/// `gen_n` (sg, du, pl)
/// `dat_m` (sg, du, pl)
/// `dat_f` (sg, du, pl)
/// `dat_n` (sg, du, pl)
/// `acc_m` (sg, du, pl)
/// `acc_f` (sg, du, pl)
/// `acc_n` (sg, du, pl)
/// `voc_m` (sg, du, pl)
/// `voc_f` (sg, du, pl)
/// `voc_n` (sg, du, pl)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident)?;
     $(nom_m: $c0:tt, $c1:tt, $c2:tt;)?
     $(nom_f: $c3:tt, $c4:tt, $c5:tt;)?
     $(nom_n: $c6:tt, $c7:tt, $c8:tt;)?
     $(gen_m: $c9:tt, $c10:tt, $c11:tt;)?
     $(gen_f: $c12:tt, $c13:tt, $c14:tt;)?
     $(gen_n: $c15:tt, $c16:tt, $c17:tt;)?
     $(dat_m: $c18:tt, $c19:tt, $c20:tt;)?
     $(dat_f: $c21:tt, $c22:tt, $c23:tt;)?
     $(dat_n: $c24:tt, $c25:tt, $c26:tt;)?
     $(acc_m: $c27:tt, $c28:tt, $c29:tt;)?
     $(acc_f: $c30:tt, $c31:tt, $c32:tt;)?
     $(acc_n: $c33:tt, $c34:tt, $c35:tt;)?
     $(voc_m: $c36:tt, $c37:tt, $c38:tt;)?
     $(voc_f: $c39:tt, $c40:tt, $c41:tt;)?
     $(voc_n: $c42:tt, $c43:tt, $c44:tt;)?
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
    /// Like `ἁπᾱς`: 1 words.
    Ἁπᾱς,
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
        Declension::Ἁπᾱς,
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
    // Ὁσος: 15 words (αὐτος, οὑτος, ἑος, ἐκεῑνος, ἑκᾰστος, ταυτος).
    declension!("ὁσος", 15, "~ος";
    nom_m: "~ος", _, "~οι";
    nom_f: "~η", _, "~αι";
    nom_n: _, _, "~ᾰ";
    gen_m: "~ου", "~οιν", "~ων";
    gen_f: "~ης", "~αιν", "~ων";
    dat_m: "~ῳ", _, "~οις";
    dat_f: "~ῃ", _, "~αις";
    acc_m: "~ον", "~ω", "~ους";
    acc_f: "~ην", "~ᾱ", "~ᾱς";
    acc_n: "~ον", _, "~ᾰ";
    voc_m: "~ε", _, _;
    ),
    // Οἱος: 7 words (ἀμφοτερος, οἱος, ἑκᾰτερος, οὐδετερος, ὁποτερος, μηδετερος). Like Ὁσος, except in 4 cells: nom_f sg, gen_f sg, dat_f sg, acc_f sg.
    declension!("οἱος", 7, "~ος", like Ὁσος;
    nom_f: "~ᾱ", _, _;
    gen_f: "~ᾱς", _, _;
    dat_f: "~ᾳ", _, _;
    acc_f: "~ᾱν", _, _;
    ),
    // Ἑαυτοῡ: 6 words (ἑαυτοῡ, ἑωυτοῡ, αὑτοῡ, σεαυτοῡ, σεωυτοῡ, σαυτοῡ). Like Ὁσος, except in 13 cells: nom_m sg, nom_m pl, nom_f sg, nom_f pl, nom_n pl, gen_m sg, ….
    declension!("ἑαυτοῡ", 6, "~οῡ", like Ὁσος;
    nom_m: "~οῡ", _, derived;
    nom_f: derived, _, derived;
    nom_n: _, _, derived;
    gen_m: "~οῡ", "~οῑν", _;
    gen_f: _, "~αῑν", "~εων";
    dat_m: _, _, "~οῑς";
    dat_f: _, _, "~ῃσῐ";
    acc_n: "~ο", _, _;
    voc_m: derived, _, _;
    ),
    // Τοῑος: 4 words (ποῑος, τοῑος, πᾰντοῑος, ὁποῑος).
    declension!("τοῑος", 4, "~ῑος";
    nom_m: "~ῑος", _, "~ῑοι";
    nom_f: "~ιᾱ", _, "~ῑαι";
    nom_n: _, _, "~ῑᾰ";
    gen_m: "~ιου", "~ιοιν", "~ιων";
    gen_f: "~ιᾱς", "~ιαιν", "~ιων";
    dat_m: "~ιῳ", _, "~ιοις";
    dat_f: "~ιᾳ", _, "~ιαις";
    acc_m: "~ῑον", "~ιω", "~ιους";
    acc_f: "~ιᾱν", "~ιᾱ", "~ιᾱς";
    acc_n: "~ῑον", _, "~ῑᾰ";
    voc_m: "~ῑε", _, _;
    ),
    // Σφω: 4 words (ἐγω, νω, σφω, ἀμφω). Like Ἑαυτοῡ, except in 18 cells: nom_m sg, gen_m sg, gen_m du, gen_m pl, gen_f sg, gen_f du, ….
    declension!("σφω", 4, "~ω", like Ἑαυτοῡ;
    nom_m: "~ω", _, _;
    gen_m: derived, "~ῳν", derived;
    gen_f: derived, "~ῳν", derived;
    dat_m: derived, _, derived;
    dat_f: derived, _, derived;
    acc_m: derived, _, derived;
    acc_f: derived, "~ω", derived;
    acc_n: derived, _, derived;
    ),
    // Ἐμαυτοῡ: 3 words (ἐμαυτοῡ, ἐμεωυτοῡ, ἐμωυτοῡ). Like Ἑαυτοῡ, except in 12 cells: gen_m du, gen_m pl, gen_f du, gen_f pl, dat_m pl, dat_f pl, ….
    declension!("ἐμαυτοῡ", 3, "~οῡ", like Ἑαυτοῡ;
    gen_m: _, derived, derived;
    gen_f: _, derived, derived;
    dat_m: _, _, derived;
    dat_f: _, _, derived;
    acc_m: _, derived, derived;
    acc_f: _, derived, derived;
    acc_n: "~ον", _, derived;
    ),
    // Ἡμεῑς: 2 words (ἡμεῑς, ὑμεῑς). Like Σφω, except in 15 cells: nom_m sg, nom_m pl, nom_f pl, nom_n pl, gen_m du, gen_m pl, ….
    declension!("ἡμεῑς", 2, "~εῑς", like Σφω;
    nom_m: "~εῑς", _, "~εῑς";
    nom_f: _, _, "~εῑς";
    nom_n: _, _, "~εῑς";
    gen_m: _, derived, "~ων";
    gen_f: _, derived, "~ων";
    dat_m: _, _, "~ῑν";
    dat_f: _, _, "~ῑν";
    acc_m: _, derived, "~ᾱς";
    acc_f: _, derived, "~ᾱς";
    acc_n: _, _, "~ᾱς";
    ),
    // Ὁ: 1 words (ὁ).
    declension!("ὁ", 1, "ὁ";
    nom_m: "ὁ", _, "οἱ";
    nom_f: "ἡ", _, "αἱ";
    nom_n: _, _, "τᾰ";
    gen_m: "τοῡ", "τοῑν", "των";
    gen_f: "της", "τοῑν", "των";
    dat_m: "τῳ", _, "τοῑς";
    dat_f: "τῃ", _, "ταῑς";
    acc_m: "τον", "τω", "τους";
    acc_f: "την", "τω", "τᾱς";
    acc_n: "το", _, "τᾰ";
    ),
    // Σῠ: 1 words (σῠ).
    declension!("σῠ", 1, "σῠ";
    nom_m: "σῠ", _, _;
    nom_f: "σῠ", _, _;
    nom_n: "σῠ", _, _;
    gen_m: "σοῡ|σου", _, _;
    gen_f: "σοῡ|σου", _, _;
    gen_n: "σοῡ|σου", _, _;
    dat_m: "σοι|σοι", _, _;
    dat_f: "σοι|σοι", _, _;
    dat_n: "σοι|σοι", _, _;
    acc_m: "σε|σε", _, _;
    acc_f: "σε|σε", _, _;
    acc_n: "σε|σε", _, _;
    voc_m: "τᾱν", _, _;
    voc_f: "τᾱν", _, _;
    voc_n: "τᾱν", _, _;
    ),
    // Τῐς: 1 words (τῐς).
    declension!("τῐς", 1, "~ῐς";
    nom_m: "~ῐς", "~ῐνε|~ῐνε", "~ῐνες|~ῐνες";
    nom_f: "~ῐς", "~ῐνε|~ῐνε", "~ῐνες|~ῐνες";
    nom_n: _, "~ῐνε|~ῐνε", "~ῐνᾰ|~ῐνᾰ|ἀ~τᾰ";
    gen_m: "~ῐνος|~ῐνος|~ου", "~ῐνοιν|~ῐνοῑν", "~ῐνων|~ῐνων";
    gen_f: "~ῐνος|~ῐνος|~ου", "~ῐνοιν|~ῐνοῑν", "~ῐνων|~ῐνων";
    gen_n: "~ῐνος|~ῐνος|~ου", "~ῐνοιν|~ῐνοῑν", "~ῐνων|~ῐνων";
    dat_m: "~ῐνῐ|~ῐνῐ|~ῳ", "~ῐνοιν|~ῐνοῑν", "~ῐσῐ|~ῐσῐν|~ῐσῐ|~ῐσῐν";
    dat_f: "~ῐνῐ|~ῐνῐ|~ῳ", "~ῐνοιν|~ῐνοῑν", "~ῐσῐ|~ῐσῐν|~ῐσῐ|~ῐσῐν";
    dat_n: "~ῐνῐ|~ῐνῐ|~ῳ", "~ῐνοιν|~ῐνοῑν", "~ῐσῐ|~ῐσῐν|~ῐσῐ|~ῐσῐν";
    acc_m: "~ῐνᾰ|~ῐνᾰ", "~ῐνε|~ῐνε", "~ῐνᾰς|~ῐνᾰς";
    acc_f: "~ῐνᾰ|~ῐνᾰ", "~ῐνε|~ῐνε", "~ῐνᾰς|~ῐνᾰς";
    acc_n: "~ῐ", "~ῐνε|~ῐνε", "~ῐνᾰ|~ῐνᾰ|ἀ~τᾰ";
    ),
    // Ὁς: 1 words (ὁς). Like Ὁ, except in 20 cells: nom_m sg, nom_n pl, gen_m sg, gen_m du, gen_m pl, gen_f sg, ….
    declension!("ὁς", 1, "ὁς", like Ὁ;
    nom_m: "ὁς", _, _;
    nom_n: _, _, "ἁ";
    gen_m: "οὑ", "οἱν", "ὡν";
    gen_f: "ἡς", "αἱν", "ὡν";
    dat_m: "ᾡ", _, "οἱς";
    dat_f: "ᾑ", _, "αἱς";
    acc_m: "ὁν", "ὡ", "οὑς";
    acc_f: "ἡν", "ἁ", "ἁς";
    acc_n: "ὁ", _, "ἁ";
    ),
    // Ὁστῐς: 1 words (ὁστῐς).
    declension!("ὁστῐς", 1, "ὁσ~ῐς";
    nom_m: "ὁσ~ῐς", _, "οἱ~ῐνες";
    nom_f: "ἡ~ῐς", _, "αἱ~ῐνες";
    nom_n: _, _, "ἁ~ῐνᾰ|ἁ~τᾰ";
    gen_m: "οὑ~ῐνος|ὁ~ου", "οἱν~ῐνοιν", "ὡν~ῐνων|ὁ~ων";
    gen_f: "ἡσ~ῐνος", "αἱν~ῐνοιν", "ὡν~ῐνων|ὁ~ων";
    gen_n: "οὑ~ῐνος|ὁ~ου", _, "ὡν~ῐνων|ὁ~ων";
    dat_m: "ᾡ~ῐνῐ|ὁ~ῳ", _, "οἱσ~ῐσῐ|οἱσ~ῐσῐν|ὁ~οις";
    dat_f: "ᾑ~ῐνῐ", _, "αἱσ~ῐσῐ|αἱσ~ῐσῐν";
    dat_n: "ᾡ~ῐνῐ|ὁ~ῳ", _, "οἱσ~ῐσῐ|οἱσ~ῐσῐν|ὁ~οις";
    acc_m: "ὁν~ῐνᾰ", "ὡ~ῐνε", "οὑσ~ῐνᾰς";
    acc_f: "ἡν~ῐνᾰ", "ἁ~ῐνε", "ἁσ~ῐνᾰς";
    acc_n: _, _, "ἁ~ῐνᾰ|ἁ~τᾰ";
    ),
    // Οὐδεις: 1 words (οὐδεις).
    declension!("οὐδεις", 1, "~ις";
    nom_m: "~ις", _, "~νες";
    nom_f: "~μιᾰ", _, "~μιαι";
    nom_n: _, _, "~νᾰ";
    gen_m: "~νος", "~νοιν", "~νων";
    gen_f: "~μιᾱς", "~μιαιν", "~μιων";
    dat_m: "~νι", _, "~σι|~σιν";
    dat_f: "~μιᾳ", _, "~μιαις";
    dat_n: _, _, "~σι|~σιν";
    acc_m: "~νᾰ", "~νε", "~νᾰς";
    acc_f: "~μιᾰν", "~μιᾱ", "~μῐᾱς";
    acc_n: "~ν", _, "~νᾰ";
    voc_m: "~ις", _, _;
    ),
    // Ἱ: 1 words (ἱ). Like Σῠ, except in 15 cells: nom_m sg, nom_f sg, nom_n sg, gen_m sg, gen_f sg, gen_n sg, ….
    declension!("ἱ", 1, "ἱ", like Σῠ;
    nom_m: "ἱ|ἱ", _, _;
    nom_f: "ἱ|ἱ", _, _;
    nom_n: "ἱ|ἱ", _, _;
    gen_m: "οὑ|οὑ", _, _;
    gen_f: "οὑ|οὑ", _, _;
    gen_n: "οὑ|οὑ", _, _;
    dat_m: "ἱν|ἱν", _, _;
    dat_f: "ἱν|ἱν", _, _;
    dat_n: "ἱν|ἱν", _, _;
    acc_m: derived, _, _;
    acc_f: derived, _, _;
    acc_n: derived, _, _;
    voc_m: derived, _, _;
    voc_f: derived, _, _;
    voc_n: derived, _, _;
    ),
    // Πᾱς: 1 words (πᾱς).
    declension!("πᾱς", 1, "~ᾱς";
    nom_m: "~ᾱς", _, "~ᾰντες";
    nom_f: "~ᾱσᾰ", _, "~ᾱσαι";
    nom_n: _, _, "~ᾰντᾰ";
    gen_m: "~ᾰντος", "~ᾰντοιν", "~ᾰντων";
    gen_f: "~ᾱσης", "~ᾱσαιν", "~ᾱσων";
    dat_m: "~ᾰντῐ", _, "~ᾱσῐ|~ᾱσῐν";
    dat_f: "~ᾱσῃ", _, "~ᾱσαις";
    dat_n: _, _, "~ᾱσῐ|~ᾱσῐν";
    acc_m: "~ᾰντᾰ", "~ᾰντε", "~ᾰντᾰς";
    acc_f: "~ᾱσᾰν", "~ᾱσᾱ", "~ᾱσᾱς";
    acc_n: "~ᾱν", _, "~ᾰντᾰ";
    voc_m: "~ᾱς", _, _;
    ),
    // Οὑ: 1 words (οὑ). Like Ἱ, except in 20 cells: nom_m sg, nom_m pl, nom_f sg, nom_f pl, nom_n sg, nom_n pl, ….
    declension!("οὑ", 1, "οὑ", like Ἱ;
    nom_m: "οὑ", _, "σφεῑς";
    nom_f: derived, _, "σφεῑς";
    nom_n: derived, _, "σφεῑς";
    gen_m: _, _, "σφων";
    gen_f: _, _, "σφων";
    dat_m: "οἱ|οἱ", _, "σφῐσῐ|σφῐσῐν";
    dat_f: "οἱ|οἱ", _, "σφῐσῐ|σφῐσῐν";
    dat_n: "οἱ|οἱ", _, "σφῐσῐ|σφῐσῐν";
    acc_m: "ἑ|ἑ", _, "σφᾱς";
    acc_f: "ἑ|ἑ", _, "σφᾱς";
    acc_n: "ἑ|ἑ", _, "σφᾱς";
    ),
    // Ἁπᾱς: 1 words (ἁπᾱς). Like Πᾱς, except in 5 cells: gen_m du, gen_f du, acc_m du, acc_f du, acc_n sg.
    declension!("ἁπᾱς", 1, "~ᾱς", like Πᾱς;
    gen_m: _, derived, _;
    gen_f: _, derived, _;
    acc_m: _, derived, _;
    acc_f: _, derived, _;
    acc_n: "~ᾰν", _, _;
    ),
    // Ὁδε: 1 words (ὁδε).
    declension!("ὁδε", 1, "ὁ~";
    nom_m: "ὁ~", _, "οἱ~";
    nom_f: "ἡ~", _, "αἱ~";
    nom_n: _, _, "τᾰ~";
    gen_m: "τοῡ~", "τοῑν~", "των~";
    gen_f: "τησ~", "τοῑν~|ταῑν~", "των~";
    dat_m: "τῳ~", _, "τοῑσ~";
    dat_f: "τῃ~", "τοῑν~|ταῑν~", "ταῑσ~";
    acc_m: "τον~", "τω~", "τουσ~";
    acc_f: "την~", "τω~", "τᾱσ~";
    acc_n: "το~", _, "τᾰ~";
    ),
    // Μηδεις: 1 words (μηδεις). Like Ἐμαυτοῡ, except in 9 cells: nom_m sg, nom_f sg, gen_m sg, gen_f sg, dat_m sg, dat_f sg, ….
    declension!("μηδεις", 1, "~ις", like Ἐμαυτοῡ;
    nom_m: "~ις", _, _;
    nom_f: "~μῐᾰ", _, _;
    gen_m: "~νος", _, _;
    gen_f: "~μῐᾱς", _, _;
    dat_m: "~νῐ", _, _;
    dat_f: "~μῐᾳ", _, _;
    acc_m: "~νᾰ", _, _;
    acc_f: "~μῐᾰν", _, _;
    acc_n: "~ν", _, _;
    ),
    // Ἀλληλων: 1 words (ἀλληλων). Like Σφω, except in 11 cells: nom_m sg, gen_m du, gen_m pl, gen_f du, gen_f pl, dat_m pl, ….
    declension!("ἀλληλων", 1, "~ων", like Σφω;
    nom_m: "~ων", _, _;
    gen_m: _, "~οιν", "~ων";
    gen_f: _, "~αιν", "~ων";
    dat_m: _, _, "~οις";
    dat_f: _, _, "~αις";
    acc_m: _, _, "~ους";
    acc_f: _, "~ᾱ", "~ᾱς";
    acc_n: _, _, "~ᾰ";
    ),
    // Ὁσπερ: 1 words (ὁσπερ). Like Ὁδε, except in 21 cells: nom_m sg, nom_n pl, gen_m sg, gen_m du, gen_m pl, gen_f sg, ….
    declension!("ὁσπερ", 1, "ὁσ~", like Ὁδε;
    nom_m: "ὁσ~", _, _;
    nom_n: _, _, "ἁ~";
    gen_m: "οὑ~", "οἱν~", "ὡν~";
    gen_f: "ἡσ~", "αἱν~", "ὡν~";
    dat_m: "ᾡ~", _, "οἱσ~";
    dat_f: "ᾑ~", derived, "αἱσ~";
    acc_m: "ὁν~", "ὡ~", "οὑσ~";
    acc_f: "ἡν~", "ἁ~", "ἁσ~";
    acc_n: "ὁ~", _, "ἁ~";
    ),
    // Σφεῑς: 1 words (σφεῑς). Like Ἡμεῑς, except in 3 cells: dat_m pl, dat_f pl, dat_n pl.
    declension!("σφεῑς", 1, "~εῑς", like Ἡμεῑς;
    dat_m: _, _, "~ῐσῐ|~ῐσῐν";
    dat_f: _, _, "~ῐσῐ|~ῐσῐν";
    dat_n: _, _, "~ῐσῐ|~ῐσῐν";
    ),
    // Ἐνῐοι: 1 words (ἐνῐοι). Like Ἡμεῑς, except in 8 cells: nom_m sg, nom_m pl, nom_f pl, nom_n pl, dat_m pl, dat_f pl, ….
    declension!("ἐνῐοι", 1, "~οι", like Ἡμεῑς;
    nom_m: "~οι", _, "~οι";
    nom_f: _, _, "~αι";
    nom_n: _, _, "~ᾰ";
    dat_m: _, _, "~οις";
    dat_f: _, _, "~αις";
    acc_m: _, _, "~ους";
    acc_n: _, _, "~ᾰ";
    ),
    // Ὁσᾰπλᾰσῐων: 1 words (ὁσᾰπλᾰσῐων).
    declension!("ὁσᾰπλᾰσῐων", 1, "~ων";
    nom_m: "~ων", _, "~ονες";
    nom_f: "~ων", _, "~ονες";
    nom_n: _, _, "~ονᾰ";
    gen_m: "~ονος", "~ονοιν", "~ονων";
    gen_f: "~ονος", "~ονοιν", "~ονων";
    dat_m: "~ονῐ", _, "~οσῐ|~οσῐν";
    dat_f: "~ονῐ", _, "~οσῐ|~οσῐν";
    dat_n: _, _, "~οσῐ|~οσῐν";
    acc_m: "~ονᾰ", "~ονε", "~ονᾰς";
    acc_f: "~ονᾰ", "~ονε", "~ονᾰς";
    acc_n: "~ον", _, "~ονᾰ";
    voc_m: "~ον", _, _;
    voc_f: "~ον", _, _;
    ),
];
