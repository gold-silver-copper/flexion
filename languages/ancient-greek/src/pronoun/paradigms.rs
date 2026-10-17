//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes (`~ος`, `οἱος`: the stem and `ος`). A stem in two pieces (`~α~ου`, `ἑαυτου`) has
//! letters between them that other forms may drop or change.
//! Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (if they share them),
/// the declension it is like (if any), the stem slots it states with a stem and an
/// ending table (see `stems.rs`), then a row of patterns per line
/// (`derived`: the form comes from another cell's, see `relations.rs`; `_`,
/// or a row left out: the form is its stem slot's, or the declension's it is
/// like, or, in a declension like no other, derived):
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
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident $(as $view:ident)? $(with $affix:literal $(, $other:literal after $letters:literal)?)?)?;
     $(stem lemma: $s0:tt;)?
     $(stem nom_m_du: $s1:tt;)?
     $(stem nom_m_pl: $s2:tt;)?
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
            reads: view!($($($view)?)?),
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
            stems: ::std::borrow::Cow::Borrowed(concat!(
                $(flexion::cell!($s0),)? "\t",
                $(flexion::cell!($s1),)? "\t",
                $(flexion::cell!($s2),)? "\t",
            )),
            affix: affix!($($($affix $(, $other, $letters)?)?)?),
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

/// The cells a declension reads in the one it is like, by the view's name (see
/// the views below); none: the same cells.
macro_rules! view {
    () => {
        None
    };
    ($view:ident) => {
        Some(::std::borrow::Cow::Borrowed(&$view))
    };
}

/// What a declension adds after each pattern of the one it is like (`with "~"`: `~` after every pattern).
macro_rules! affix {
    () => {
        None
    };
    ($affix:literal) => {
        affix!($affix, $affix, "")
    };
    ($affix:literal, $other:literal, $letters:literal) => {
        Some(flexion::Affix {
            pattern: ::std::borrow::Cow::Borrowed($affix),
            other: ::std::borrow::Cow::Borrowed($other),
            letters: ::std::borrow::Cow::Borrowed($letters),
        })
    };
}

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `οἱος`: 13 words.
    Οἱος,
    /// Like `ὁσος`: 8 words.
    Ὁσος,
    /// Like `αὑτου`: 5 words.
    Αὑτου,
    /// Like `αὐτος`: 5 words.
    Αὐτος,
    /// Like `σφεις`: 5 words.
    Σφεις,
    /// Like `σφω`: 4 words.
    Σφω,
    /// Like `ἐμαυτου`: 3 words.
    Ἐμαυτου,
    /// Like `ἁπᾱς`: 2 words.
    Ἁπᾱς,
    /// Like `ὁδε`: 2 words.
    Ὁδε,
    /// Like `ὁστῐς`: 2 words.
    Ὁστῐς,
    /// Like `ὁ`: 1 words.
    Ὁ,
    /// Like `σῠ`: 1 words.
    Σῠ,
    /// Like `τῐς`: 1 words.
    Τῐς,
    /// Like `ἱ`: 1 words.
    Ἱ,
    /// Like `οὑ`: 1 words.
    Οὑ,
    /// Like `ἑαυτου`: 1 words.
    Ἑαυτου,
    /// Like `ἀλληλων`: 1 words.
    Ἀλληλων,
    /// Like `ἐνῐοι`: 1 words.
    Ἐνῐοι,
    /// Like `ὁσᾰπλᾰσῐων`: 1 words.
    Ὁσᾰπλᾰσῐων,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 19] = [
        Declension::Οἱος,
        Declension::Ὁσος,
        Declension::Αὑτου,
        Declension::Αὐτος,
        Declension::Σφεις,
        Declension::Σφω,
        Declension::Ἐμαυτου,
        Declension::Ἁπᾱς,
        Declension::Ὁδε,
        Declension::Ὁστῐς,
        Declension::Ὁ,
        Declension::Σῠ,
        Declension::Τῐς,
        Declension::Ἱ,
        Declension::Οὑ,
        Declension::Ἑαυτου,
        Declension::Ἀλληλων,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 19] = [
    // Οἱος: 13 words (οὑτος, ποιος, ἀμφοτερος, οἱος, τοιος, ἑκᾰτερος).
    declension!("οἱος", 13, "~ος";
    nom_m: "~ος", "~ω", "~οι";
    nom_f: "~ᾱ", "~ᾱ", "~αι";
    nom_n: _, _, "~ᾰ";
    gen_m: "~ου", "~οιν", "~ων";
    gen_f: "~ᾱς", "~αιν", "~ων";
    dat_m: "~ῳ", _, "~οις";
    dat_f: "~ᾳ", _, "~αις";
    acc_m: "~ον", _, "~ους";
    acc_f: "~ᾱν", _, "~ᾱς";
    acc_n: "~ον", _, "~ᾰ";
    voc_m: "~ε", _, _;
    voc_n: "~ον", _, _;
    ),
    // Ὁσος: 8 words (ἑος, ἑκᾰστος, ὁσος, τοσος, ὁποστος, ὁποσος). Like Οἱος, except in 4 cells: nom_f sg, gen_f sg, dat_f sg, acc_f sg.
    declension!("ὁσος", 8, "~ος", like Οἱος;
    nom_f: "~η", _, _;
    gen_f: "~ης", _, _;
    dat_f: "~ῃ", _, _;
    acc_f: "~ην", _, _;
    ),
    // Αὑτου: 5 words (ἑωυτου, αὑτου, σεαυτου, σεωυτου, σαυτου). Like Ὁσος, except in 15 cells: nom_m sg, nom_m du, nom_m pl, nom_f sg, nom_f du, nom_f pl, ….
    declension!("αὑτου", 5, "~ου", like Ὁσος;
    nom_m: "~ου", derived, derived;
    nom_f: derived, derived, derived;
    nom_n: _, _, derived;
    gen_f: _, _, "~εων";
    dat_f: _, _, "~ῃσῐ";
    acc_m: _, "~ω", _;
    acc_f: _, "~ᾱ", _;
    acc_n: "~ο", "~ω", _;
    voc_m: derived, _, _;
    voc_n: derived, _, _;
    ),
    // Αὐτος: 5 words (αὐτος, ἐκεινος, ταυτος, τοιουτος, τοσουτος). Like Ὁσος, except in 3 cells: nom_n sg, acc_n sg, voc_n sg.
    declension!("αὐτος", 5, "~ος", like Ὁσος;
    nom_n: "~ο|~ον", _, _;
    acc_n: "~ο|~ον", _, _;
    voc_n: derived, _, _;
    ),
    // Σφεις: 5 words (οὐδεις, ἡμεις, ὑμεις, μηδεις, σφεις). Like Αὑτου, except in 21 cells: nom_m sg, nom_m pl, nom_f pl, nom_n pl, gen_m sg, gen_m du, ….
    declension!("σφεις", 5, "~εις", like Αὑτου;
    nom_m: "~εις", _, "~εις";
    nom_f: _, _, "~εις";
    nom_n: _, _, "~εις";
    gen_m: derived, derived, _;
    gen_f: derived, derived, "~ων";
    dat_m: derived, _, "~ῑν";
    dat_f: derived, _, "~ῑν";
    acc_m: derived, derived, "~ᾱς";
    acc_f: derived, derived, _;
    acc_n: derived, derived, "~ᾱς";
    ),
    // Σφω: 4 words (ἐγω, νω, σφω, ἀμφω). Like Σφεις, except in 15 cells: nom_m sg, nom_m du, nom_m pl, nom_f du, nom_f pl, nom_n pl, ….
    declension!("σφω", 4, "~ω", like Σφεις;
    nom_m: "~ω", "~ω", derived;
    nom_f: _, "~ω", derived;
    nom_n: _, _, derived;
    gen_m: _, "~ῳν", derived;
    gen_f: _, "~ῳν", derived;
    dat_m: _, _, derived;
    dat_f: _, _, derived;
    acc_m: _, _, derived;
    acc_f: _, _, derived;
    acc_n: _, _, derived;
    ),
    // Ἐμαυτου: 3 words (ἐμαυτου, ἐμεωυτου, ἐμωυτου). Like Σφω, except in 12 cells: nom_m sg, nom_m du, nom_f du, gen_m sg, gen_m du, gen_f sg, ….
    declension!("ἐμαυτου", 3, "~ου", like Σφω;
    nom_m: "~ου", derived, _;
    nom_f: _, derived, _;
    gen_m: "~ου", derived, _;
    gen_f: "~ης", derived, _;
    dat_m: "~ῳ", _, _;
    dat_f: "~ῃ", _, _;
    acc_m: "~ον", _, _;
    acc_f: "~ην", _, _;
    acc_n: "~ον", _, _;
    ),
    // Ἁπᾱς: 2 words (πᾱς, ἁπᾱς). Like Σφεις, except in 21 cells: nom_m sg, nom_m pl, nom_f sg, nom_f pl, nom_n pl, gen_m sg, ….
    declension!("ἁπᾱς", 2, "~ᾱς", like Σφεις;
    nom_m: "~ᾱς", _, "~ᾰντες";
    nom_f: "~ᾱσᾰ", _, "~ᾱσαι";
    nom_n: _, _, "~ᾰντᾰ";
    gen_m: "~ᾰντος", _, "~ᾰντων";
    gen_f: "~ᾱσης", _, "~ᾱσων";
    dat_m: "~ᾰντῐ", _, "~ᾱσῐ|~ᾱσῐν";
    dat_f: "~ᾱσῃ", _, "~ᾱσαις";
    acc_m: "~ᾰντᾰ", _, "~ᾰντᾰς";
    acc_f: "~ᾱσᾰν", _, "~ᾱσᾱς";
    acc_n: "~ᾰν", _, "~ᾰντᾰ";
    voc_m: "~ᾱς", _, _;
    voc_n: "~ᾰν", _, _;
    ),
    // Ὁδε: 2 words (ὁς, ὁδε). Like Ὁ with `~` after each pattern, except in 5 cells: gen_f sg, dat_m pl, dat_f pl, acc_m pl, acc_f pl.
    declension!("ὁδε", 2, "ὁ~", like Ὁ with "~";
    gen_f: "τησ~", _, _;
    dat_m: _, _, "τοισ~";
    dat_f: _, _, "ταισ~";
    acc_m: _, _, "τουσ~";
    acc_f: _, _, "τᾱσ~";
    ),
    // Ὁστῐς: 2 words (ὁστῐς, ὁσπερ). Like Ὁδε, except in 20 cells: nom_m sg, nom_m du, nom_f du, nom_n pl, gen_m sg, gen_m du, ….
    declension!("ὁστῐς", 2, "ὁσ~", like Ὁδε;
    nom_m: "ὁσ~", "ὡ~", _;
    nom_f: _, "ἁ~", _;
    nom_n: _, _, "ἁ~";
    gen_m: "οὑ~", "οἱν~", "ὡν~";
    gen_f: "ἡσ~", "αἱν~", "ὡν~";
    dat_m: "ᾡ~", _, "οἱσ~";
    dat_f: "ᾑ~", _, "αἱσ~";
    acc_m: "ὁν~", _, "οὑσ~";
    acc_f: "ἡν~", _, "ἁσ~";
    acc_n: "ὁ~", _, "ἁ~";
    ),
    // Ὁ: 1 words (ὁ).
    declension!("ὁ", 1, "ὁ";
    nom_m: "ὁ", "τω", "οἱ";
    nom_f: "ἡ", "τω", "αἱ";
    nom_n: _, _, "τᾰ";
    gen_m: "του", "τοιν", "των";
    gen_f: "της", "τοιν", "των";
    dat_m: "τῳ", _, "τοις";
    dat_f: "τῃ", _, "ταις";
    acc_m: "τον", _, "τους";
    acc_f: "την", _, "τᾱς";
    acc_n: "το", _, "τᾰ";
    ),
    // Σῠ: 1 words (σῠ).
    declension!("σῠ", 1, "σῠ";
    nom_m: "σῠ", _, _;
    nom_f: "σῠ", _, _;
    nom_n: "σῠ", _, _;
    gen_m: "σου|σου", _, _;
    gen_f: "σου|σου", _, _;
    dat_m: "σοι|σοι", _, _;
    dat_f: "σοι|σοι", _, _;
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
    nom_n: _, _, "~ῐνᾰ|~ῐνᾰ|ἀ~τᾰ";
    gen_m: "~ῐνος|~ῐνος|~ου", "~ῐνοιν|~ῐνοιν", "~ῐνων|~ῐνων";
    gen_f: "~ῐνος|~ῐνος|~ου", "~ῐνοιν|~ῐνοιν", "~ῐνων|~ῐνων";
    dat_m: "~ῐνῐ|~ῐνῐ|~ῳ", _, "~ῐσῐ|~ῐσῐν|~ῐσῐ|~ῐσῐν";
    dat_f: "~ῐνῐ|~ῐνῐ|~ῳ", _, "~ῐσῐ|~ῐσῐν|~ῐσῐ|~ῐσῐν";
    acc_m: "~ῐνᾰ|~ῐνᾰ", _, "~ῐνᾰς|~ῐνᾰς";
    acc_f: "~ῐνᾰ|~ῐνᾰ", _, "~ῐνᾰς|~ῐνᾰς";
    acc_n: "~ῐ", _, "~ῐνᾰ|~ῐνᾰ|ἀ~τᾰ";
    ),
    // Ἱ: 1 words (ἱ). Like Σῠ, except in 13 cells: nom_m sg, nom_f sg, nom_n sg, gen_m sg, gen_f sg, dat_m sg, ….
    declension!("ἱ", 1, "ἱ", like Σῠ;
    nom_m: "ἱ|ἱ", _, _;
    nom_f: "ἱ|ἱ", _, _;
    nom_n: "ἱ|ἱ", _, _;
    gen_m: "οὑ|οὑ", _, _;
    gen_f: "οὑ|οὑ", _, _;
    dat_m: "ἱν|ἱν", _, _;
    dat_f: "ἱν|ἱν", _, _;
    acc_m: derived, _, _;
    acc_f: derived, _, _;
    acc_n: derived, _, _;
    voc_m: derived, _, _;
    voc_f: derived, _, _;
    voc_n: derived, _, _;
    ),
    // Οὑ: 1 words (οὑ). Like Ἱ, except in 18 cells: nom_m sg, nom_m pl, nom_f sg, nom_f pl, nom_n sg, nom_n pl, ….
    declension!("οὑ", 1, "οὑ", like Ἱ;
    nom_m: "οὑ", _, "σφεις";
    nom_f: derived, _, "σφεις";
    nom_n: derived, _, "σφεις";
    gen_m: _, _, "σφων";
    gen_f: _, _, "σφων";
    dat_m: "οἱ|οἱ", _, "σφῐσῐ|σφῐσῐν";
    dat_f: "οἱ|οἱ", _, "σφῐσῐ|σφῐσῐν";
    acc_m: "ἑ|ἑ", _, "σφᾱς";
    acc_f: "ἑ|ἑ", _, "σφᾱς";
    acc_n: "ἑ|ἑ", _, "σφᾱς";
    ),
    // Ἑαυτου: 1 words (ἑαυτου).
    declension!("ἑαυτου", 1, "~α~ου";
    nom_m: "~α~ου", _, _;
    gen_m: "~ᾱ~ου", "~ᾱ~οιν", "~ᾱ~ων";
    gen_f: "~ᾱ~ης", "~ᾱ~αιν", "~ᾱ~ων";
    dat_m: "~ᾱ~ῳ", _, "~ᾱ~οις|~ᾱ~οισῐ|~ᾱ~οισῐν";
    dat_f: "~ᾱ~ῃ", _, "~ᾱ~αις";
    acc_m: "~ᾱ~ον", "~ᾱ~ω", "~ᾱ~ους";
    acc_f: "~ᾱ~ην", "~ᾱ~ᾱ", "~ᾱ~ᾱς";
    acc_n: "~ᾱ~ο|~ᾱ~ον", "~ᾱ~ω", "~ᾱ~ᾰ";
    ),
    // Ἀλληλων: 1 words (ἀλληλων). Like Αὑτου, except in 10 cells: nom_m sg, gen_m sg, gen_f sg, gen_f pl, dat_m sg, dat_f sg, ….
    declension!("ἀλληλων", 1, "~ων", like Αὑτου;
    nom_m: "~ων", _, _;
    gen_m: derived, _, _;
    gen_f: derived, _, "~ων";
    dat_m: derived, _, _;
    dat_f: derived, _, "~αις";
    acc_m: derived, _, _;
    acc_f: derived, _, _;
    acc_n: derived, _, _;
    ),
    // Ἐνῐοι: 1 words (ἐνῐοι). Like Σφεις, except in 8 cells: nom_m sg, nom_m pl, nom_f pl, nom_n pl, dat_m pl, dat_f pl, ….
    declension!("ἐνῐοι", 1, "~οι", like Σφεις;
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
    nom_m: "~ων", "~ονε", "~ονες";
    nom_f: "~ων", "~ονε", "~ονες";
    nom_n: _, _, "~ονᾰ";
    gen_m: "~ονος", "~ονοιν", "~ονων";
    gen_f: "~ονος", "~ονοιν", "~ονων";
    dat_m: "~ονῐ", _, "~οσῐ|~οσῐν";
    dat_f: "~ονῐ", _, "~οσῐ|~οσῐν";
    acc_m: "~ονᾰ", _, "~ονᾰς";
    acc_f: "~ονᾰ", _, "~ονᾰς";
    acc_n: "~ον", _, "~ονᾰ";
    voc_m: "~ον", _, _;
    voc_f: "~ον", _, _;
    voc_n: "~ον", _, _;
    ),
];
