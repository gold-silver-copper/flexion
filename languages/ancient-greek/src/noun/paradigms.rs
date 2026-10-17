//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes (`~ος`, `ἀρῐθμος`: the stem and `ος`). A stem in two pieces (`~ω~`, `ῥητωρ`) has
//! letters between them that other forms may drop or change.
//! Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (if they share them),
/// the declension it is like (if any), the stem slots it states with a stem and an
/// ending table
/// (`stem lemma: \"~ον- like σημειον\"`: the slot's cells are `σημειον`'s table's patterns
/// with `~ον` for their `~`) (see `stems.rs`), then a row of patterns per line
/// (`derived`: the form comes from another cell's, see `relations.rs`; `_`,
/// or a row left out: the form is its stem slot's, or the declension's it is
/// like, or, in a declension like no other, derived):
/// `nom` (sg, du, pl)
/// `gen_` (sg, du, pl)
/// `dat` (sg, du, pl)
/// `acc` (sg, du, pl)
/// `voc` (sg, du, pl)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, [$($f:ident: $v:ident),*])? $(, like $parent:ident $(as $view:ident)? $(with $affix:literal $(, $other:literal after $letters:literal)?)?)?;
     $(stem lemma: $s0:tt;)?
     $(stem gen_sg: $s1:tt;)?
     $(nom: $c0:tt, $c1:tt, $c2:tt;)?
     $(gen_: $c3:tt, $c4:tt, $c5:tt;)?
     $(dat: $c6:tt, $c7:tt, $c8:tt;)?
     $(acc: $c9:tt, $c10:tt, $c11:tt;)?
     $(voc: $c12:tt, $c13:tt, $c14:tt;)?
    ) => {
        Paradigm {
            name: ::std::borrow::Cow::Borrowed($name),
            words: $words,
            lemma: ::std::borrow::Cow::Borrowed($lemma),
            parent: like!($($parent)?),
            reads: view!($($($view)?)?),
            inherent: ::std::borrow::Cow::Borrowed(&[$($(feature!($f: $v)),*)?]),
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
            )),
            stems: ::std::borrow::Cow::Borrowed(concat!(
                $(flexion::cell!($s0),)? "\t",
                $(flexion::cell!($s1),)? "\t",
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

/// What a declension adds after each pattern of the one it is like (`with "", "ς" after "η"`: ``, or `ς`
/// where that pattern ends in `η`).
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

/// An inherent feature's value, as `(feature, value)` by their places.
macro_rules! feature {
    (gender: $v:ident) => {
        (0, crate::Gender::$v as u8)
    };
}

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `ἀρῐθμος`: 1,783 words.
    Ἀρῐθμος,
    /// Like `σημειον`: 1,175 words.
    Σημειον,
    /// Like `νῠμφη`: 867 words.
    Νῠμφη,
    /// Like `φορᾱ`: 772 words.
    Φορᾱ,
    /// Like `περῐφρονησῐς`: 494 words.
    Περῐφρονησῐς,
    /// Like `θεμᾰ`: 448 words.
    Θεμᾰ,
    /// Like `ἰδῐωτης`: 323 words.
    Ἰδῐωτης,
    /// Like `κορᾰξ`: 220 words.
    Κορᾰξ,
    /// Like `κερκῐς`: 196 words.
    Κερκῐς,
    /// Like `βᾰρῠτης`: 168 words.
    Βᾰρῠτης,
    /// Like `τρᾰπεζᾰ`: 151 words.
    Τρᾰπεζᾰ,
    /// Like `ἐπῐφᾰνειᾰ`: 145 words.
    Ἐπῐφᾰνειᾰ,
    /// Like `τευχος`: 118 words.
    Τευχος,
    /// Like `αἰων`: 104 words.
    Αἰων,
    /// Like `ἱππευς`: 86 words.
    Ἱππευς,
    /// Like `χᾰρᾰκτηρ`: 86 words.
    Χᾰρᾰκτηρ,
    /// Like `δᾰνᾰοι`: 77 words.
    Δᾰνᾰοι,
    /// Like `κᾰλπῐς`: 67 words.
    Κᾰλπῐς,
    /// Like `κρῐτης`: 66 words.
    Κρῐτης,
    /// Like `ὀργῐᾰ`: 56 words.
    Ὀργῐᾰ,
    /// Like `κοχλῐᾱς`: 48 words.
    Κοχλῐᾱς,
    /// Like `κολλοψ`: 48 words.
    Κολλοψ,
    /// Like `κροσσαι`: 41 words.
    Κροσσαι,
    /// Like `φᾰος`: 40 words.
    Φᾰος,
    /// Like `ἀξων`: 40 words.
    Ἀξων,
    /// Like `κᾰνων`: 36 words.
    Κᾰνων,
    /// Like `ῥητωρ`: 33 words.
    Ῥητωρ,
    /// Like `στᾰχῠς`: 32 words.
    Στᾰχῠς,
    /// Like `πτερῠξ`: 22 words.
    Πτερῠξ,
    /// Like `λεων`: 22 words.
    Λεων,
    /// Like `λᾰτρῐς`: 21 words.
    Λᾰτρῐς,
    /// Like `βᾰρῠτης`: 20 words.
    Βᾰρῠτης2,
    /// Like `πους`: 19 words.
    Πους,
    /// Like `νηδῡς`: 19 words.
    Νηδῡς,
    /// Like `ἰκρῐα`: 15 words.
    Ἰκρῐα,
    /// Like `ὑπᾰρ`: 13 words.
    Ὑπᾰρ,
    /// Like `ἁλως`: 12 words.
    Ἁλως,
    /// Like `θενᾰρ`: 12 words.
    Θενᾰρ,
    /// Like `περᾰς`: 11 words.
    Περᾰς,
    /// Like `πεπερῐ`: 11 words.
    Πεπερῐ,
    /// Like `ἀθῠρᾱς`: 11 words.
    Ἀθῠρᾱς,
    /// Like `ἱππομᾰνες`: 10 words.
    Ἱππομᾰνες,
    /// Like `τρωες`: 10 words.
    Τρωες,
    /// Like `φρην`: 9 words.
    Φρην,
    /// Like `ἱμᾱς`: 9 words.
    Ἱμᾱς,
    /// Like `πτῠξ`: 8 words.
    Πτῠξ,
    /// Like `μωλῠ`: 8 words.
    Μωλῠ,
    /// Like `κῠᾰρ`: 8 words.
    Κῠᾰρ,
    /// Like `νους`: 8 words.
    Νους,
    /// Like `ἠχω`: 7 words.
    Ἠχω,
    /// Like `δμως`: 6 words.
    Δμως,
    /// Like `γερᾰς`: 6 words.
    Γερᾰς,
    /// Like `πᾰτηρ`: 6 words.
    Πᾰτηρ,
    /// Like `ὀϊς`: 6 words.
    Ὀϊς,
    /// Like `δᾱμος`: 6 words.
    Δᾱμος,
    /// Like `λῐψ`: 5 words.
    Λῐψ,
    /// Like `κωμῡς`: 5 words.
    Κωμῡς,
    /// Like `ἀγλῑς`: 5 words.
    Ἀγλῑς,
    /// Like `φῡσᾱς`: 5 words.
    Φῡσᾱς,
    /// Like `ἀηρ`: 5 words.
    Ἀηρ,
    /// Like `κηρῠξ`: 5 words.
    Κηρῠξ,
    /// Like `κρεᾰς`: 5 words.
    Κρεᾰς,
    /// Like `πηχῠς`: 4 words.
    Πηχῠς,
    /// Like `δᾰσῠ`: 4 words.
    Δᾰσῠ,
    /// Like `πᾰπᾱς`: 4 words.
    Πᾰπᾱς,
    /// Like `ζωᾱ`: 4 words.
    Ζωᾱ,
    /// Like `τρῐηρης`: 4 words.
    Τρῐηρης,
    /// Like `ὀδους`: 3 words.
    Ὀδους,
    /// Like `υἱῠς`: 3 words.
    Υἱῠς,
    /// Like `πειθω`: 3 words.
    Πειθω,
    /// Like `ἐγκᾰτᾰ`: 3 words.
    Ἐγκᾰτᾰ,
    /// Like `λαλλαι`: 3 words.
    Λαλλαι,
    /// Like `ἐντεᾰ`: 3 words.
    Ἐντεᾰ,
    /// Like `θεωσῐς`: 3 words.
    Θεωσῐς,
    /// Like `δυσμαι`: 2 words.
    Δυσμαι,
    /// Like `ἱερος`: 1 words.
    Ἱερος,
    /// Like `ἱερον`: 1 words.
    Ἱερον,
    /// Like `ἱερα`: 1 words.
    Ἱερα,
    /// Like `μηριᾰ`: 1 words.
    Μηριᾰ,
    /// Like `βυσναιοι`: 1 words.
    Βυσναιοι,
    /// Like `ἐπισφυρῐᾰ`: 1 words.
    Ἐπισφυρῐᾰ,
    /// Like `πᾰραφερνᾰ`: 1 words.
    Πᾰραφερνᾰ,
    /// Like `βακχῐαδαι`: 1 words.
    Βακχῐαδαι,
    /// Like `βρογχῐᾰ`: 1 words.
    Βρογχῐᾰ,
    /// Like `χᾰριστῐᾰ`: 1 words.
    Χᾰριστῐᾰ,
    /// Like `ἑλληνοτᾰμιαι`: 1 words.
    Ἑλληνοτᾰμιαι,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 86] = [
        Declension::Ἀρῐθμος,
        Declension::Σημειον,
        Declension::Νῠμφη,
        Declension::Φορᾱ,
        Declension::Περῐφρονησῐς,
        Declension::Θεμᾰ,
        Declension::Ἰδῐωτης,
        Declension::Κορᾰξ,
        Declension::Κερκῐς,
        Declension::Βᾰρῠτης,
        Declension::Τρᾰπεζᾰ,
        Declension::Ἐπῐφᾰνειᾰ,
        Declension::Τευχος,
        Declension::Αἰων,
        Declension::Ἱππευς,
        Declension::Χᾰρᾰκτηρ,
        Declension::Δᾰνᾰοι,
        Declension::Κᾰλπῐς,
        Declension::Κρῐτης,
        Declension::Ὀργῐᾰ,
        Declension::Κοχλῐᾱς,
        Declension::Κολλοψ,
        Declension::Κροσσαι,
        Declension::Φᾰος,
        Declension::Ἀξων,
        Declension::Κᾰνων,
        Declension::Ῥητωρ,
        Declension::Στᾰχῠς,
        Declension::Πτερῠξ,
        Declension::Λεων,
        Declension::Λᾰτρῐς,
        Declension::Βᾰρῠτης2,
        Declension::Πους,
        Declension::Νηδῡς,
        Declension::Ἰκρῐα,
        Declension::Ὑπᾰρ,
        Declension::Ἁλως,
        Declension::Θενᾰρ,
        Declension::Περᾰς,
        Declension::Πεπερῐ,
        Declension::Ἀθῠρᾱς,
        Declension::Ἱππομᾰνες,
        Declension::Τρωες,
        Declension::Φρην,
        Declension::Ἱμᾱς,
        Declension::Πτῠξ,
        Declension::Μωλῠ,
        Declension::Κῠᾰρ,
        Declension::Νους,
        Declension::Ἠχω,
        Declension::Δμως,
        Declension::Γερᾰς,
        Declension::Πᾰτηρ,
        Declension::Ὀϊς,
        Declension::Δᾱμος,
        Declension::Λῐψ,
        Declension::Κωμῡς,
        Declension::Ἀγλῑς,
        Declension::Φῡσᾱς,
        Declension::Ἀηρ,
        Declension::Κηρῠξ,
        Declension::Κρεᾰς,
        Declension::Πηχῠς,
        Declension::Δᾰσῠ,
        Declension::Πᾰπᾱς,
        Declension::Ζωᾱ,
        Declension::Τρῐηρης,
        Declension::Ὀδους,
        Declension::Υἱῠς,
        Declension::Πειθω,
        Declension::Ἐγκᾰτᾰ,
        Declension::Λαλλαι,
        Declension::Ἐντεᾰ,
        Declension::Θεωσῐς,
        Declension::Δυσμαι,
        Declension::Ἱερος,
        Declension::Ἱερον,
        Declension::Ἱερα,
        Declension::Μηριᾰ,
        Declension::Βυσναιοι,
        Declension::Ἐπισφυρῐᾰ,
        Declension::Πᾰραφερνᾰ,
        Declension::Βακχῐαδαι,
        Declension::Βρογχῐᾰ,
        Declension::Χᾰριστῐᾰ,
        Declension::Ἑλληνοτᾰμιαι,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 86] = [
    // Ἀρῐθμος: 1,783 words (λῠκος, κοσμος, ἱππος, ἐρωμενος, σπογγος, ἀγγελος).
    declension!("ἀρῐθμος", 1783, "~ος";
    nom: "~ος", "~ω", "~οι";
    gen_: "~ου", "~οιν", "~ων";
    dat: "~ῳ", _, "~οις";
    acc: "~ον", _, "~ους";
    voc: "~ε", _, _;
    ),
    // Σημειον: 1,175 words (ἀστρον, παιδῐον, δενδρον, μῠστηρῐον, ἐργον, εἰδωλον). Like Ἀρῐθμος, except in 2 cells: nom pl, acc pl.
    declension!("σημειον", 1175, "~ον", [gender: Neuter], like Ἀρῐθμος;
    stem lemma: "~ον- like σημειον";
    nom: _, _, "~ᾰ";
    acc: _, _, "~ᾰ";
    ),
    // Νῠμφη: 867 words (νῑκη, μηχᾰνη, μουσῐκη, γη, δῑνη, κορη).
    declension!("νῠμφη", 867, "~η", [gender: Feminine];
    stem lemma: "~η- like σημειον";
    nom: _, "~ᾱ", "~αι";
    gen_: "~ης", "~αιν", "~ων";
    dat: "~ῃ", _, "~αις";
    acc: "~ην", _, "~ᾱς";
    ),
    // Φορᾱ: 772 words (δημοκρᾰτῐᾱ, χωρᾱ, μελῳδῐᾱ, παιδῐᾱ, φῐλῐᾱ, σοφῐᾱ). Like Νῠμφη, except in 3 cells: gen_ sg, dat sg, acc sg.
    declension!("φορᾱ", 772, "~ᾱ", [gender: Feminine], like Νῠμφη;
    stem lemma: "~ᾱ- like σημειον";
    gen_: "~ᾱς", _, _;
    dat: "~ᾳ", _, _;
    acc: "~ᾱν", _, _;
    ),
    // Περῐφρονησῐς: 494 words (πολῐς, γνωσῐς, κῠβερνησῐς, ἀποκᾰλῠψῐς, μορφωσῐς, φῠσῐς).
    declension!("περῐφρονησῐς", 494, "~ῐς", [gender: Feminine];
    stem lemma: "~ῐ- like δῐαιρεσῐς";
    nom: _, "~ει", "~εις";
    gen_: "~εως", "~εοιν", "~εων";
    dat: "~ει", _, "~εσῐ|~εσῐν";
    acc: "~ῐν", _, "~εις";
    ),
    // Θεμᾰ: 448 words (μελῐ, γρᾰμμᾰ, δερμᾰ, σῑγμᾰ, πνευμᾰ, βαπτισμᾰ).
    declension!("θεμᾰ", 448, "~", [gender: Neuter];
    stem lemma: "~- like σημειον";
    stem gen_sg: "~- like θεμᾰ";
    dat: _, derived, _;
    acc: _, derived, _;
    voc: _, derived, derived;
    ),
    // Ἰδῐωτης: 323 words (ἐρᾰστης, προφητης, ποιητης, ὑποκρῐτης, κλεπτης, ἰσρᾱηλῑτης). Like Νῠμφη with `` after each pattern (`ς` after η), except in 2 cells: gen_ sg, voc sg.
    declension!("ἰδῐωτης", 323, "~ης", [gender: Masculine], like Νῠμφη with "", "ς" after "η";
    gen_: "~ου", _, _;
    voc: "~ᾰ", _, _;
    ),
    // Κορᾰξ: 220 words (χᾰλῐξ, σᾰρξ, ἀνθρᾰξ, ἀβᾰξ, πτερῠξ, γλαυξ).
    declension!("κορᾰξ", 220, "~ξ";
    stem lemma: "~ξ- like σημειον";
    stem gen_sg: "~- like κορᾰξ";
    dat: _, derived, _;
    acc: _, derived, _;
    voc: _, derived, derived;
    ),
    // Κερκῐς: 196 words (παις, λεπῐς, πᾰτρῐς, ἑλληνῐς, περσῐς, γρᾰφῐς).
    declension!("κερκῐς", 196, "~ς", [gender: Feminine];
    stem lemma: "~- like δῐαιρεσῐς";
    nom: _, "~δε", "~δες";
    gen_: "~δος", "~δοιν", "~δων";
    dat: "~δῐ", _, "~σῐ|~σῐν";
    acc: "~δᾰ", _, "~δᾰς";
    ),
    // Βᾰρῠτης: 168 words (μῠκης, κρης, σκληροτης, μᾰρτῠς, θης, κλεις). Like Κερκῐς, except in 1 cell: voc sg.
    declension!("βᾰρῠτης", 168, "~ς", like Κερκῐς;
    voc: "~ς", _, _;
    ),
    // Τρᾰπεζᾰ: 151 words (θᾰλᾰσσᾰ, ῥῐζᾰ, θᾰλᾰττᾰ, δοξᾰ, γλωττᾰ, μελῐττᾰ). Like Νῠμφη, except in 1 cell: acc sg.
    declension!("τρᾰπεζᾰ", 151, "~ᾰ", [gender: Feminine], like Νῠμφη;
    stem lemma: "~ᾰ- like σημειον";
    acc: "~ᾰν", _, _;
    ),
    // Ἐπῐφᾰνειᾰ: 145 words (βᾰσῐλειᾰ, ἀσθενειᾰ, μᾰχαιρᾰ, ἀληθειᾰ, βοηθειᾰ, μοιρᾰ). Like Τρᾰπεζᾰ, except in 2 cells: gen_ sg, dat sg.
    declension!("ἐπῐφᾰνειᾰ", 145, "~ᾰ", [gender: Feminine], like Τρᾰπεζᾰ;
    gen_: "~ᾱς", _, _;
    dat: "~ᾳ", _, _;
    ),
    // Τευχος: 118 words (σκῠλος, κλεος, ἐθνος, πληθος, τελος, στηθος).
    declension!("τευχος", 118, "~ος", [gender: Neuter];
    stem lemma: "~ος- like σημειον";
    nom: _, "~ει", "~η";
    gen_: "~ους", "~οιν", "~ων";
    dat: "~ει", _, "~εσῐ|~εσῐν";
    acc: "~ος", _, "~η";
    ),
    // Αἰων: 104 words (ἑλλην, αἰων, λεγεων, ἀγων, σᾱπων, μην).
    declension!("αἰων", 104, "~ν", [gender: Masculine];
    stem lemma: "~ν- like σημειον";
    stem gen_sg: "~- like αἰων";
    dat: _, derived, _;
    acc: _, derived, _;
    voc: _, derived, derived;
    ),
    // Ἱππευς: 86 words (βᾰσῐλευς, ἀρχιερευς, γραμμᾰτευς, ἀλεξᾰνδρευς, αἰολευς, ἀμφισσευς). Like Περῐφρονησῐς, except in 5 cells: nom du, nom pl, dat pl, acc sg, acc pl.
    declension!("ἱππευς", 86, "~ευς", [gender: Masculine], like Περῐφρονησῐς;
    stem lemma: "~ευ- like δῐαιρεσῐς";
    nom: _, "~η", "~ης|~εις";
    dat: _, _, "~ευσῐ|~ευσῐν";
    acc: "~εᾱ", _, "~εᾱς";
    ),
    // Χᾰρᾰκτηρ: 86 words (μᾰρτῠρ, πᾰνθηρ, κᾱρ, ἰβηρ, χειρ, ἐορ).
    declension!("χᾰρᾰκτηρ", 86, "~";
    stem lemma: "~- like σημειον";
    stem gen_sg: "~- like χᾰρᾰκτηρ";
    dat: _, derived, _;
    acc: _, derived, _;
    voc: _, derived, derived;
    ),
    // Δᾰνᾰοι: 77 words (βοττῐαιοι, δᾰνᾰοι, τῐβᾰρηνοι, μεθοδῐκοι, πᾰλαιοι, πᾰρᾰβολᾱνοι). Like Ἀρῐθμος, except in 7 cells: nom sg, nom du, gen_ sg, gen_ du, dat sg, acc sg, ….
    declension!("δᾰνᾰοι", 77, "~οι", [gender: Masculine], like Ἀρῐθμος;
    nom: "~οι", derived, _;
    gen_: derived, derived, _;
    dat: derived, _, _;
    acc: derived, _, _;
    voc: derived, _, _;
    ),
    // Κᾰλπῐς: 67 words (πελασγῐωτῐς, σπᾰρτῐᾱτῐς, σῠβᾰρῑτῐς, σμῠρῐς, ἀσπρῐς, εὐνῐς). Like Κερκῐς, except in 1 cell: acc sg.
    declension!("κᾰλπῐς", 67, "~ς", like Κερκῐς;
    acc: "~ν", _, _;
    ),
    // Κρῐτης: 66 words (κρῐτης, βελγης, δακης, μῐνῠης, ὀδρῠσης, πῐσῐδης). Like Νῠμφη with `` after each pattern (`ς` after η), except in 2 cells: gen_ sg, voc sg.
    declension!("κρῐτης", 66, "~ης", [gender: Masculine], like Νῠμφη with "", "ς" after "η";
    gen_: "~ου", _, _;
    voc: "~η", _, _;
    ),
    // Ὀργῐᾰ: 56 words (πῠρᾰ, κᾰστᾰνειᾰ, κᾰστᾰνᾰ, ὀργῐᾰ, πολῑτῐκᾰ, γαρσᾰνᾰ). Like Δᾰνᾰοι, except in 2 cells: nom sg, voc pl.
    declension!("ὀργῐᾰ", 56, "~ᾰ", [gender: Neuter], like Δᾰνᾰοι;
    stem gen_sg: "~- like ὀργῐᾰ";
    nom: "~ᾰ", _, _;
    voc: _, _, derived;
    ),
    // Κοχλῐᾱς: 48 words (νεᾱνῐᾱς, γᾰλᾰξῐᾱς, καικιᾱς, κῠβᾱς, τᾰμῐᾱς, κοχλῐᾱς). Like Φορᾱ with `` after each pattern (`ς` after ᾱ), except in 3 cells: nom du, gen_ sg, voc sg.
    declension!("κοχλῐᾱς", 48, "~ᾱς", [gender: Masculine], like Φορᾱ with "", "ς" after "ᾱ";
    nom: _, "~ᾱ", _;
    gen_: "~ου", _, _;
    voc: "~ᾱ", _, _;
    ),
    // Κολλοψ: 48 words (αἰθῐοψ, ὠψ, κῠκλωψ, δολοψ, αἰγῐλωψ, ἀγχιλωψ).
    declension!("κολλοψ", 48, "~ψ";
    stem lemma: "~ψ- like σημειον";
    nom: _, "~πε", "~πες";
    gen_: "~πος", "~ποιν", "~πων";
    dat: "~πῐ", _, "~ψῐ|~ψῐν";
    acc: "~πᾰ", _, "~πᾰς";
    ),
    // Κροσσαι: 41 words (ἑβδομηκοντᾰκωμηται, τεσσᾰρεσκαιδεκᾰτῑται, ὀᾰσῑται, ἰσκαι, γᾰμφηλαι, κρουπεζαι). Like Ἰκρῐα with `` after each pattern (`ι` after α), except in 3 cells: nom pl, dat pl, acc pl.
    declension!("κροσσαι", 41, "~αι", like Ἰκρῐα with "", "ι" after "α";
    nom: _, _, "~αι";
    dat: _, _, "~αις";
    acc: _, _, "~ᾱς";
    ),
    // Φᾰος: 40 words (δᾰσος, πεος, σκᾰφος, φᾰος, πᾰχος, κᾰμπος). Like Τευχος, except in 7 cells: nom du, nom pl, gen_ sg, gen_ du, gen_ pl, dat sg, ….
    declension!("φᾰος", 40, "~ος", [gender: Neuter], like Τευχος;
    nom: _, "~εε", "~εᾰ";
    gen_: "~εος", "~εοιν", "~εων";
    dat: "~εϊ", _, _;
    acc: _, _, "~εᾰ";
    ),
    // Ἀξων: 40 words (κῠων, ἀρχιτεκτων, βιστων, χᾱων, ἰᾱων, λῠκᾱων).
    declension!("ἀξων", 40, "~ων", [gender: Masculine];
    stem gen_sg: "~ο- like αἰων";
    nom: "~ων", _, _;
    dat: _, derived, _;
    acc: _, derived, _;
    voc: "~ον", derived, derived;
    ),
    // Κᾰνων: 36 words (ἡγεμων, τεκτων, ἀμᾰζων, ἀλκῠων, μᾰκεδων, μῠρμῐδων). Like Ἀξων, except in 1 cell: voc sg.
    declension!("κᾰνων", 36, "~ων", like Ἀξων;
    voc: "~ων", _, _;
    ),
    // Ῥητωρ: 33 words (ἀλεκτωρ, καστωρ, ῥητωρ, δωτωρ, στατωρ, κοσμοκρᾰτωρ).
    declension!("ῥητωρ", 33, "~ω~";
    stem gen_sg: "~ο~- like χᾰρᾰκτηρ";
    nom: "~ω~", _, _;
    dat: _, derived, _;
    acc: _, derived, _;
    voc: "~ο~", derived, derived;
    ),
    // Στᾰχῠς: 32 words (λῐβῠς, λῐγῠς, γενῠς, ἐγχελῠς, ῥᾰπῠς, ῥᾰφῠς).
    declension!("στᾰχῠς", 32, "~ῠς";
    stem lemma: "~ῠ- like δῐαιρεσῐς";
    nom: _, "~ῠε|~ῡ", "~ῠες|~ῡς";
    gen_: "~ῠος", "~ῠοιν", "~ῠων";
    dat: "~ῠϊ|~υι", _, "~ῠσῐ|~ῠσῐν";
    acc: "~ῠν", _, "~ῡς|~ῠᾰς";
    ),
    // Πτερῠξ: 22 words (φλοξ, αἰξ, ἰᾱπῠξ, λᾰτᾰξ, ὀρτῠξ, σφᾰξ).
    declension!("πτερῠξ", 22, "~ξ";
    stem lemma: "~ξ- like σημειον";
    nom: _, "~γε", "~γες";
    gen_: "~γος", "~γοιν", "~γων";
    dat: "~γῐ", _, "~ξῐ|~ξῐν";
    acc: "~γᾰ", _, "~γᾰς";
    ),
    // Λεων: 22 words (θερᾰπων, λεων, δρᾰκων, γερων, ὁρῐζων, χᾰμαιλεων).
    declension!("λεων", 22, "~ων", [gender: Masculine];
    nom: "~ων", "~οντε", "~οντες";
    gen_: "~οντος", "~οντοιν", "~οντων";
    dat: "~οντῐ", _, "~ουσῐ|~ουσῐν";
    acc: "~οντᾰ", _, "~οντᾰς";
    voc: "~ον", _, _;
    ),
    // Λᾰτρῐς: 21 words (ἀκρῐς, ὀκρῐς, λᾰτρῐς, μηνῐς, πορτῐς, ἀρδῐς).
    declension!("λᾰτρῐς", 21, "~ῐς";
    stem lemma: "~ῐ- like δῐαιρεσῐς";
    nom: _, "~ῐε", "~ῐες";
    gen_: "~ῐος", "~ῐοιν", "~ῐων";
    dat: "~ῑ", _, "~ῐσῐ|~ῐσῐν";
    acc: "~ῐν", _, "~ῑς|~ῐᾰς";
    ),
    // Βᾰρῠτης2: 20 words (χᾰρῐς, ῥῑς, εἱλως, ἰς, ἱδρως, ἐρως). Like Θεμᾰ with `` after each pattern (`ς` after ~, ᾰ), except in 2 cells: nom pl, acc sg.
    declension!("βᾰρῠτης", 20, "~ς", like Θεμᾰ with "", "ς" after "~ᾰ";
    nom: _, _, "~τες";
    acc: "~τᾰ", _, _;
    ),
    // Πους: 19 words (πους, ὀκτωπους, πολυπους, τρῐπους, ἀπους, δᾰσῠπους). Like Κερκῐς.
    declension!("πους", 19, "~υς", [gender: Masculine], like Κερκῐς;
    stem lemma: "~υς- like σημειον";
    ),
    // Νηδῡς: 19 words (ἰχθῡς, σῡς, δελφῡς, δρῡς, ὀφρῡς, μῡς). Like Στᾰχῠς, except in 1 cell: acc sg.
    declension!("νηδῡς", 19, "~ῡς", like Στᾰχῠς;
    stem lemma: "~ῡ- like δῐαιρεσῐς";
    acc: "~ῡν", _, _;
    ),
    // Ἰκρῐα: 15 words (γενεσια, μουστακια, σαγματοπασμαγαδια, εὐληρα, βρῡτεα, κανθηλια). Like Ὀργῐᾰ, except in 1 cell: nom sg.
    declension!("ἰκρῐα", 15, "~α", [gender: Neuter], like Ὀργῐᾰ;
    nom: "~α", _, _;
    ),
    // Ὑπᾰρ: 13 words (ἑξηκοντᾰ, ὀσσε, ὑφεν, ῥᾰββῑ, ῥᾰββονῑ, ῥᾰββουνῑ).
    declension!("ὑπᾰρ", 13, "~";
    stem lemma: "~- like σημειον";
    nom: _, "~", "~";
    gen_: "~", "~", "~";
    dat: "~", _, "~";
    acc: "~", _, "~";
    ),
    // Ἁλως: 12 words (νεως, γᾰλως, τᾰως, λᾰγως, ἑως, λεως).
    declension!("ἁλως", 12, "~ως";
    stem lemma: "~ως- like σημειον";
    nom: _, "~ω", "~ῳ";
    gen_: "~ω", "~ῳν", "~ων";
    dat: "~ῳ", _, "~ῳς";
    acc: "~ω|~ων", _, "~ως";
    ),
    // Θενᾰρ: 12 words (κηρ, ἐᾰρ, θενᾰρ, ὀνᾰρ, νεκτᾰρ, μελᾰν). Like Χᾰρᾰκτηρ, except in 3 cells: nom pl, acc sg, acc pl.
    declension!("θενᾰρ", 12, "~", [gender: Neuter], like Χᾰρᾰκτηρ;
    nom: _, _, "~ᾰ";
    acc: "~", _, "~ᾰ";
    ),
    // Περᾰς: 11 words (φως, τερᾰς, πᾰγκρεᾰς, ἐρῠσῐπελᾰς, περᾰς, εἰκος). Like Θεμᾰ with `` after each pattern (`ς` after ~).
    declension!("περᾰς", 11, "~ς", [gender: Neuter], like Θεμᾰ with "", "ς" after "~";
    ),
    // Πεπερῐ: 11 words (σῐνᾱπῐ, κομμῐ, πεπερῐ, κιννᾰβᾰρῐ, ἀμῐ, στῐμμῐ). Like Περῐφρονησῐς, except in 4 cells: nom sg, nom pl, acc sg, acc pl.
    declension!("πεπερῐ", 11, "~ῐ", [gender: Neuter], like Περῐφρονησῐς;
    nom: "~ῐ", _, "~η|~εᾰ";
    acc: "~ῐ", _, "~η|~εᾰ";
    ),
    // Ἀθῠρᾱς: 11 words (γεννᾱς, γᾰλᾰτᾱς, πᾰξᾰμᾱς, γενετᾱς, μᾰρῐκᾱς, ἀττᾰγᾱς). Like Φορᾱ with `` after each pattern (`ς` after ᾱ), except in 3 cells: nom du, gen_ sg, voc sg.
    declension!("ἀθῠρᾱς", 11, "~ᾱς", [gender: Masculine], like Φορᾱ with "", "ς" after "ᾱ";
    nom: _, "~ᾱ", _;
    gen_: "~ᾱ", _, _;
    voc: "~ᾱ", _, _;
    ),
    // Ἱππομᾰνες: 10 words (νευροειδες, μονοειδες, τρῐχομᾰνες, ἡμεροκᾰλλες, σησᾰμοειδες, ὠκῐμοειδες). Like Τευχος, except in 1 cell: acc sg.
    declension!("ἱππομᾰνες", 10, "~ες", [gender: Neuter], like Τευχος;
    stem lemma: "~ες- like σημειον";
    acc: "~ες", _, _;
    ),
    // Τρωες: 10 words (πῑερες, τρωες, κηφηνες, ἁλτηρες, αἰγες, ὠρῠγγες). Like Δᾰνᾰοι, except in 4 cells: nom sg, nom pl, dat pl, acc pl.
    declension!("τρωες", 10, "~ες", like Δᾰνᾰοι;
    nom: "~ες", _, "~ες";
    dat: _, _, "~σῐ|~σῐν";
    acc: _, _, "~ᾰς";
    ),
    // Φρην: 9 words (φρην, ὑμην, ποιμην, πῠθμην, ἀδην, αὐχην).
    declension!("φρην", 9, "~ην";
    stem lemma: "~ην- like σημειον";
    stem gen_sg: "~ε- like αἰων";
    dat: _, derived, _;
    acc: _, derived, _;
    voc: _, derived, derived;
    ),
    // Ἱμᾱς: 9 words (κορῠβᾱς, γῐγᾱς, ἐλεφᾱς, ἀδᾰμᾱς, ἀνδρῐᾱς, λῠκᾰβᾱς).
    declension!("ἱμᾱς", 9, "~ᾱς", [gender: Masculine];
    nom: "~ᾱς", "~ᾰντε", "~ᾰντες";
    gen_: "~ᾰντος", "~ᾰντοιν", "~ᾰντων";
    dat: "~ᾰντῐ", _, "~ᾱσῐ|~ᾱσῐν";
    acc: "~ᾰντᾰ", _, "~ᾰντᾰς";
    voc: "~ᾰν", _, _;
    ),
    // Πτῠξ: 8 words (ὀνῠξ, ψῑξ, σᾰρδονῠξ, ὑστρῐξ, στῐξ, βηξ).
    declension!("πτῠξ", 8, "~ξ";
    stem lemma: "~ξ- like σημειον";
    nom: _, "~χε", "~χες";
    gen_: "~χος", "~χοιν", "~χων";
    dat: "~χῐ", _, "~ξῐ|~ξῐν";
    acc: "~χᾰ", _, "~χᾰς";
    ),
    // Μωλῠ: 8 words (δᾰκρῠ, μωλῠ, νᾱπῠ, ἀσχῠ, μῐσῠ, βρᾰθῠ). Like Στᾰχῠς, except in 4 cells: nom sg, nom pl, acc sg, acc pl.
    declension!("μωλῠ", 8, "~ῠ", [gender: Neuter], like Στᾰχῠς;
    nom: "~ῠ", _, "~ῠᾰ";
    acc: "~ῠ", _, "~ῠᾰ";
    ),
    // Κῠᾰρ: 8 words (οὐθᾰρ, ἡπᾰρ, φρεᾱρ, ἀλειφᾰρ, στεᾱρ, δελεᾰρ). Like Θεμᾰ with `` after each pattern (`ρ` after ~).
    declension!("κῠᾰρ", 8, "~ρ", [gender: Neuter], like Θεμᾰ with "", "ρ" after "~";
    ),
    // Νους: 8 words (νους, πλους, θῠγατρῐδους, ῥους, πᾰραπλους, ἐκπλους). Like Ἀρῐθμος, except in 2 cells: nom du, acc sg.
    declension!("νους", 8, "~ους", [gender: Masculine], like Ἀρῐθμος;
    stem lemma: "~ου- like δῐαιρεσῐς";
    nom: _, "~ω|~ω", _;
    acc: "~ουν", _, _;
    ),
    // Ἠχω: 7 words (ἠχω, τυτω, κῐττω, πῑπω, μοτω, φειδω). Like Ἀρῐθμος, except in 5 cells: nom sg, gen_ sg, dat sg, acc sg, voc sg.
    declension!("ἠχω", 7, "~ω", [gender: Feminine], like Ἀρῐθμος;
    nom: "~ω", _, _;
    gen_: "~ους", _, _;
    dat: "~οι", _, _;
    acc: "~ω", _, _;
    voc: "~οι", _, _;
    ),
    // Δμως: 6 words (τρως, ἡρως, δμως, πᾰτρως, θως, μητρως).
    declension!("δμως", 6, "~ως", [gender: Masculine];
    stem lemma: "~ως- like σημειον";
    nom: _, "~ωε", "~ωες";
    gen_: "~ωος", "~ωοιν", "~ωων";
    dat: "~ῳ|~ωϊ", _, "~ωσῐ|~ωσῐν";
    acc: "~ω", _, "~ωᾰς";
    ),
    // Γερᾰς: 6 words (γερᾰς, δεπᾰς, ψεφᾰς, σφελᾰς, βουκερᾰς, σκεπᾰς). Like Θενᾰρ with `` after each pattern (`ς` after ~), except in 2 cells: dat sg, dat pl.
    declension!("γερᾰς", 6, "~ς", [gender: Neuter], like Θενᾰρ with "", "ς" after "~";
    dat: "~ϊ", _, "~σῐ";
    ),
    // Πᾰτηρ: 6 words (ἀνηρ, μητηρ, πᾰτηρ, ἀστηρ, θῠγᾰτηρ, ἐνᾰτηρ).
    declension!("πᾰτηρ", 6, "~η~";
    nom: "~η~", "~ε~ε", "~ε~ες";
    gen_: "~~ος", "~ε~οιν", "~ε~ων";
    dat: "~~ῐ", _, "~~ᾰσῐ";
    acc: "~ε~ᾰ", _, "~ε~ᾰς";
    voc: "~ε~", _, _;
    ),
    // Ὀϊς: 6 words (ἁλς, βους, ὑς, ὀϊς, χους, κῑς). Like Χᾰρᾰκτηρ, except in 4 cells: dat sg, dat pl, acc sg, acc pl.
    declension!("ὀϊς", 6, "~ς", [gender: Masculine], like Χᾰρᾰκτηρ;
    stem lemma: "~- like δῐαιρεσῐς";
    dat: "~", _, "~σῐ";
    acc: "~ν", _, "~ς";
    ),
    // Δᾱμος: 6 words (ὀρᾰνος, ξεννος, δᾱμος, θῐος, ἁλῐος, ἀρχᾱγος). Like Ἀρῐθμος, except in 3 cells: gen_ sg, dat pl, acc pl.
    declension!("δᾱμος", 6, "~ος", [gender: Masculine], like Ἀρῐθμος;
    gen_: "~ω|~οιο", _, _;
    dat: _, _, "~οις|~οισῐ|~οισῐν";
    acc: _, _, "~ως";
    ),
    // Λῐψ: 5 words (χᾰλῠψ, φλεψ, χερνῐψ, φᾰψ, λῐψ).
    declension!("λῐψ", 5, "~ψ";
    stem lemma: "~ψ- like σημειον";
    nom: _, "~βε", "~βες";
    gen_: "~βος", "~βοιν", "~βων";
    dat: "~βῐ", _, "~ψῐ|~ψῐν";
    acc: "~βᾰ", _, "~βᾰς";
    ),
    // Κωμῡς: 5 words (ἑλμῐνς, κωμῡς, κορῠς, πειρῐνς, ἀγνῡς).
    declension!("κωμῡς", 5, "~ς";
    stem lemma: "~ς- like σημειον";
    nom: _, "~θε", "~θες";
    gen_: "~θος", "~θοιν", "~θων";
    dat: "~θῐ", _, "~σῐ|~σῐν";
    acc: "~θᾰ", _, "~θᾰς";
    ),
    // Ἀγλῑς: 5 words (ὀρνῑς, γελγῑς, ἀγλῑς, δελλῑς, μερμῑς). Like Κωμῡς, except in 2 cells: acc sg, voc sg.
    declension!("ἀγλῑς", 5, "~ς", like Κωμῡς;
    acc: "~ν", _, _;
    voc: "~", _, _;
    ),
    // Φῡσᾱς: 5 words (πελεκᾱς, πῡρᾰμους, φῡσᾱς, πλᾰκους, ἀλλᾱς).
    declension!("φῡσᾱς", 5, "~ς", [gender: Masculine];
    nom: "~ς", "~ντε", "~ντες";
    gen_: "~ντος", "~ντοιν", "~ντων";
    dat: "~ντῐ", _, "~σῐ|~σῐν";
    acc: "~ντᾰ", _, "~ντᾰς";
    voc: "~ν", _, _;
    ),
    // Ἀηρ: 5 words (γᾰστηρ, ἀηρ, φρᾱτηρ, αἰθηρ, ἀθηρ). Like Πᾰτηρ, except in 5 cells: dat du, acc du, voc sg, voc du, voc pl.
    declension!("ἀηρ", 5, "~η~", like Πᾰτηρ;
    stem gen_sg: "~ε~- like χᾰρᾰκτηρ";
    dat: _, derived, _;
    acc: _, derived, _;
    voc: "~η~", derived, derived;
    ),
    // Κηρῠξ: 5 words (κοκκῠξ, κηρῠξ, δοιδῠξ, ψευδοκηρῠξ, ἱεροκηρῠξ).
    declension!("κηρῠξ", 5, "~ῠξ", [gender: Masculine];
    stem lemma: "~ῠξ- like σημειον";
    stem gen_sg: "~ῡ- like κορᾰξ";
    dat: _, derived, _;
    acc: _, derived, _;
    voc: _, derived, derived;
    ),
    // Κρεᾰς: 5 words (κρεᾰς, σελᾰς, κνεφᾰς, γηρᾰς, κᾰλλῐκρεᾰς).
    declension!("κρεᾰς", 5, "~ᾰς", [gender: Neuter];
    stem lemma: "~ᾰς- like σημειον";
    nom: _, "~ᾱ", "~ᾱ";
    gen_: "~ως", "~οιν", "~ων";
    dat: "~ᾳ", _, "~ᾰσῐ";
    acc: "~ᾰς", _, "~ᾱ";
    ),
    // Πηχῠς: 4 words (πελεκῠς, πρεσβῠς, πηχῠς, ἀμφῐβρᾰχῠς). Like Περῐφρονησῐς, except in 1 cell: acc sg.
    declension!("πηχῠς", 4, "~ῠς", [gender: Masculine], like Περῐφρονησῐς;
    stem lemma: "~ῠ- like δῐαιρεσῐς";
    acc: "~ῠν", _, _;
    ),
    // Δᾰσῠ: 4 words (ἀστῠ, ὀξῠ, δᾰσῠ, πλᾰτῠ). Like Φᾰος, except in 3 cells: nom du, dat sg, acc sg.
    declension!("δᾰσῠ", 4, "~ῠ", like Φᾰος;
    stem lemma: "~ῠ- like σημειον";
    nom: _, "~ει", _;
    dat: "~ει", _, _;
    acc: "~ῠ", _, _;
    ),
    // Πᾰπᾱς: 4 words (πᾰπᾱς, κᾰδῡτᾱς, σᾰκτᾱς, πᾰππᾱς). Like Φορᾱ with `` after each pattern (`ς` after ᾱ), except in 3 cells: nom du, gen_ sg, voc sg.
    declension!("πᾰπᾱς", 4, "~ᾱς", [gender: Masculine], like Φορᾱ with "", "ς" after "ᾱ";
    nom: _, "~ᾱ", _;
    gen_: "~ου", _, _;
    voc: "~ᾰ", _, _;
    ),
    // Ζωᾱ: 4 words (γονᾱ, δρᾰχμᾱ, σφᾰγᾱ, ζωᾱ). Like Φορᾱ, except in 3 cells: gen_ pl, dat pl, acc pl.
    declension!("ζωᾱ", 4, "~ᾱ", [gender: Feminine], like Φορᾱ;
    gen_: _, _, "~ᾱων|~ᾱν";
    dat: _, _, "~αις|~αισῐ|~αισῐν";
    acc: _, _, "~ᾰς";
    ),
    // Τρῐηρης: 4 words (γηγενης, τρῐηρης, κᾰλλῐκλης, ἁρποκρᾰτης). Like Ἱππομᾰνες, except in 4 cells: nom sg, nom pl, acc sg, acc pl.
    declension!("τρῐηρης", 4, "~ης", like Ἱππομᾰνες;
    nom: "~ης", _, "~εις";
    acc: "~η", _, "~εις";
    ),
    // Ὀδους: 3 words (ὀδους, κῠνοδους, τρῐοδους). Like Φῡσᾱς, except in 2 cells: nom sg, dat pl.
    declension!("ὀδους", 3, "~υς", [gender: Masculine], like Φῡσᾱς;
    nom: "~υς", _, _;
    dat: _, _, "~υσῐ|~υσῐν";
    ),
    // Υἱῠς: 3 words (ἡμῐσῠς, υἱῠς, τρῐβρᾰχῠς). Like Πηχῠς, except in 1 cell: gen_ sg.
    declension!("υἱῠς", 3, "~ῠς", [gender: Masculine], like Πηχῠς;
    gen_: "~εος", _, _;
    ),
    // Πειθω: 3 words (λεχω, πειθω, κερδω). Like Ἠχω, except in 1 cell: gen_ sg.
    declension!("πειθω", 3, "~ω", [gender: Feminine], like Ἠχω;
    gen_: "~οος|~ους", _, _;
    ),
    // Ἐγκᾰτᾰ: 3 words (ἐγκᾰτᾰ, κεδμᾰτᾰ, ἰχθῠημᾰτᾰ). Like Τρωες, except in 4 cells: nom sg, nom pl, gen_ pl, acc pl.
    declension!("ἐγκᾰτᾰ", 3, "~τᾰ", [gender: Neuter], like Τρωες;
    nom: "~τᾰ", _, "~τᾰ";
    gen_: _, _, "~των";
    acc: _, _, "~τᾰ";
    ),
    // Λαλλαι: 3 words (κᾰλανδαι, χαρωνῑται, λαλλαι). Like Ἰκρῐα with `~αι` after each pattern (`~ᾱς` after ᾰ), except in 3 cells: nom pl, gen_ pl, dat pl.
    declension!("λαλλαι", 3, "~α~αι", like Ἰκρῐα with "~αι", "~ᾱς" after "ᾰ";
    nom: _, _, "~ᾰ~αι";
    gen_: _, _, "~ᾰ~ων";
    dat: _, _, "~ᾰ~αις";
    ),
    // Ἐντεᾰ: 3 words (ἐντεᾰ, δηνεᾰ, τειρεᾰ). Like Ὀργῐᾰ, except in 1 cell: dat pl.
    declension!("ἐντεᾰ", 3, "~ᾰ", [gender: Neuter], like Ὀργῐᾰ;
    dat: _, _, "~σῐ|~σῐν";
    ),
    // Θεωσῐς: 3 words (θεωσῐς, ἀλθησῐς, ζαχαρῐς). Like Περῐφρονησῐς, except in 6 cells: nom du, nom pl, gen_ du, dat sg, dat pl, acc pl.
    declension!("θεωσῐς", 3, "~ῐς", [gender: Feminine], like Περῐφρονησῐς;
    nom: _, derived, "~εες";
    gen_: _, derived, _;
    dat: "~εϊ", _, "~εσῐ";
    acc: _, _, "~εᾰς";
    ),
    // Δυσμαι: 2 words (προλυται, δυσμαι). Like Λαλλαι, except in 2 cells: nom sg, voc pl.
    declension!("δυσμαι", 2, "~υ~αι", like Λαλλαι;
    stem gen_sg: "~ῠ~- like κροσσαι";
    nom: "~υ~αι", _, _;
    voc: _, _, derived;
    ),
    // Ἱερος: 1 words (ἱερος).
    declension!("ἱερος", 1, "ἱ~ος";
    nom: "ῐ̔~ος", "ῐ̔~ω", "ῐ̔~οι";
    gen_: "ῐ̔~ου", "ῐ̔~οιν", "ῐ̔~ων";
    dat: "ῐ̔~ῳ", _, "ῐ̔~οις";
    acc: "ῐ̔~ον", _, "ῐ̔~ους";
    voc: "ῐ̔~ε", _, _;
    ),
    // Ἱερον: 1 words (ἱερον). Like Ἱερος, except in 2 cells: nom pl, acc pl.
    declension!("ἱερον", 1, "ἱ~ον", like Ἱερος;
    stem lemma: "~ον- like ἱερον";
    nom: _, _, "ῐ̔~ᾰ";
    acc: _, _, "ῐ̔~ᾰ";
    ),
    // Ἱερα: 1 words (ἱερα).
    declension!("ἱερα", 1, "ἱ~α";
    stem lemma: "~ᾱ- like ἱερον";
    nom: _, "ῐ̔~ᾱ", "ῐ̔~αι";
    gen_: "ῐ̔~ᾱς", "ῐ̔~αιν", "ῐ̔~ων";
    dat: "ῐ̔~ᾳ", _, "ῐ̔~αις";
    acc: "ῐ̔~ᾱν", _, "ῐ̔~ᾱς";
    ),
    // Μηριᾰ: 1 words (μηριᾰ). Like Δᾰνᾰοι, except in 2 cells: nom sg, voc pl.
    declension!("μηριᾰ", 1, "~ιᾰ", like Δᾰνᾰοι;
    stem gen_sg: "~ῐ- like ὀργῐᾰ";
    nom: "~ιᾰ", _, _;
    voc: _, _, derived;
    ),
    // Βυσναιοι: 1 words (βυσναιοι). Like Δυσμαι, except in 4 cells: nom sg, nom pl, dat pl, acc pl.
    declension!("βυσναιοι", 1, "~υ~οι", like Δυσμαι;
    nom: "~υ~οι", _, "~ῠ~οι";
    dat: _, _, "~ῠ~οις";
    acc: _, _, "~ῠ~ους";
    ),
    // Ἐπισφυρῐᾰ: 1 words (ἐπισφυρῐᾰ). Like Βυσναιοι, except in 2 cells: nom sg, voc pl.
    declension!("ἐπισφυρῐᾰ", 1, "~υ~ᾰ", like Βυσναιοι;
    stem gen_sg: "~ῠ~- like ὀργῐᾰ";
    nom: "~υ~ᾰ", _, _;
    voc: _, _, derived;
    ),
    // Πᾰραφερνᾰ: 1 words (πᾰραφερνᾰ). Like Ἰκρῐα with `~ᾰ` after each pattern, except in 2 cells: gen_ pl, dat pl.
    declension!("πᾰραφερνᾰ", 1, "~α~ᾰ", like Ἰκρῐα with "~ᾰ";
    gen_: _, _, "~ᾰ~ων";
    dat: _, _, "~ᾰ~οις";
    ),
    // Βακχῐαδαι: 1 words (βακχῐαδαι). Like Ἰκρῐα with `~α~αι` after each pattern (`~ᾰ~αι` after ᾰ), except in 3 cells: gen_ pl, dat pl, acc pl.
    declension!("βακχῐαδαι", 1, "~α~α~αι", like Ἰκρῐα with "~α~αι", "~ᾰ~αι" after "ᾰ";
    gen_: _, _, "~ᾰ~ᾰ~ων";
    dat: _, _, "~ᾰ~ᾰ~αις";
    acc: _, _, "~ᾰ~ᾰ~ᾱς";
    ),
    // Βρογχῐᾰ: 1 words (βρογχῐᾰ). Like Δᾰνᾰοι, except in 2 cells: nom sg, voc pl.
    declension!("βρογχῐᾰ", 1, "~ῐᾰ", like Δᾰνᾰοι;
    stem gen_sg: "~ι- like ὀργῐᾰ";
    nom: "~ῐᾰ", _, _;
    voc: _, _, derived;
    ),
    // Χᾰριστῐᾰ: 1 words (χᾰριστῐᾰ). Like Λαλλαι, except in 2 cells: nom sg, voc pl.
    declension!("χᾰριστῐᾰ", 1, "~ι~ᾰ", like Λαλλαι;
    stem gen_sg: "~ῐ~- like ὀργῐᾰ";
    nom: "~ι~ᾰ", _, _;
    voc: _, _, derived;
    ),
    // Ἑλληνοτᾰμιαι: 1 words (ἑλληνοτᾰμιαι). Like Μηριᾰ, except in 2 cells: nom sg, voc pl.
    declension!("ἑλληνοτᾰμιαι", 1, "~ιαι", like Μηριᾰ;
    stem gen_sg: "~ῐ- like κροσσαι";
    nom: "~ιαι", _, _;
    voc: _, _, derived;
    ),
];
