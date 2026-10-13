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
/// `nom` (sg, du, pl)
/// `gen_` (sg, du, pl)
/// `dat` (sg, du, pl)
/// `acc` (sg, du, pl)
/// `voc` (sg, du, pl)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, [$($f:ident: $v:ident),*])? $(, like $parent:ident)?;
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

/// An inherent feature's value, as `(feature, value)` by their places.
macro_rules! feature {
    (gender: $v:ident) => {
        (0, crate::Gender::$v as u8)
    };
}

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `τρᾰγος`: 1,219 words.
    Τρᾰγος,
    /// Like `μορῐον`: 1,051 words.
    Μορῐον,
    /// Like `οὐσῐᾱ`: 626 words.
    Οὐσῐᾱ,
    /// Like `νῠμφη`: 533 words.
    Νῠμφη,
    /// Like `ἀρῐθμος`: 507 words.
    Ἀρῐθμος,
    /// Like `δῐαιρεσῐς`: 491 words.
    Δῐαιρεσῐς,
    /// Like `θεμᾰ`: 454 words.
    Θεμᾰ,
    /// Like `κᾰτᾰσκευη`: 313 words.
    Κᾰτᾰσκευη,
    /// Like `ἰδῐωτης`: 219 words.
    Ἰδῐωτης,
    /// Like `κερκῐς`: 198 words.
    Κερκῐς,
    /// Like `κορᾰξ`: 194 words.
    Κορᾰξ,
    /// Like `κλεις`: 176 words.
    Κλεις,
    /// Like `τρᾰπεζᾰ`: 144 words.
    Τρᾰπεζᾰ,
    /// Like `ἐπῐφᾰνειᾰ`: 128 words.
    Ἐπῐφᾰνειᾰ,
    /// Like `γενος`: 109 words.
    Γενος,
    /// Like `αἰων`: 98 words.
    Αἰων,
    /// Like `χορευτης`: 94 words.
    Χορευτης,
    /// Like `ἱππευς`: 86 words.
    Ἱππευς,
    /// Like `κεραιᾱ`: 80 words.
    Κεραιᾱ,
    /// Like `χᾰρᾰκτηρ`: 77 words.
    Χᾰρᾰκτηρ,
    /// Like `κᾰλπῐς`: 67 words.
    Κᾰλπῐς,
    /// Like `σημεῑον`: 64 words.
    Σημεῑον,
    /// Like `φορᾱ`: 62 words.
    Φορᾱ,
    /// Like `δᾰμᾰλης`: 57 words.
    Δᾰμᾰλης,
    /// Like `κοχλῐᾱς`: 49 words.
    Κοχλῐᾱς,
    /// Like `ὀργῐᾰ`: 49 words.
    Ὀργῐᾰ,
    /// Like `ζῠγον`: 47 words.
    Ζῠγον,
    /// Like `φᾰος`: 40 words.
    Φᾰος,
    /// Like `ὀλῐνοι`: 40 words.
    Ὀλῐνοι,
    /// Like `κᾰνων`: 39 words.
    Κᾰνων,
    /// Like `κροσσαι`: 39 words.
    Κροσσαι,
    /// Like `ῥητωρ`: 34 words.
    Ῥητωρ,
    /// Like `κολλοψ`: 32 words.
    Κολλοψ,
    /// Like `στᾰχῠς`: 32 words.
    Στᾰχῠς,
    /// Like `ναῡλος`: 28 words.
    Ναῡλος,
    /// Like `ἀξων`: 27 words.
    Ἀξων,
    /// Like `ῥωμαῑος`: 26 words.
    Ῥωμαῑος,
    /// Like `δᾰνᾰοι`: 24 words.
    Δᾰνᾰοι,
    /// Like `λεων`: 22 words.
    Λεων,
    /// Like `σᾰρξ`: 21 words.
    Σᾰρξ,
    /// Like `λᾰτρῐς`: 19 words.
    Λᾰτρῐς,
    /// Like `πους`: 19 words.
    Πους,
    /// Like `νηδῡς`: 18 words.
    Νηδῡς,
    /// Like `περᾰς`: 16 words.
    Περᾰς,
    /// Like `πτερῠξ`: 14 words.
    Πτερῠξ,
    /// Like `πνεῡμᾰ`: 14 words.
    Πνεῡμᾰ,
    /// Like `ἰκρῐα`: 14 words.
    Ἰκρῐα,
    /// Like `σηψ`: 13 words.
    Σηψ,
    /// Like `ῥως`: 13 words.
    Ῥως,
    /// Like `ἀσαῑοι`: 13 words.
    Ἀσαῑοι,
    /// Like `ἁλως`: 12 words.
    Ἁλως,
    /// Like `κηρ`: 12 words.
    Κηρ,
    /// Like `στοῑχος`: 12 words.
    Στοῑχος,
    /// Like `πεπερῐ`: 11 words.
    Πεπερῐ,
    /// Like `τρωες`: 11 words.
    Τρωες,
    /// Like `λευκη`: 10 words.
    Λευκη,
    /// Like `ἱππομᾰνες`: 10 words.
    Ἱππομᾰνες,
    /// Like `φρην`: 9 words.
    Φρην,
    /// Like `κοιτη`: 9 words.
    Κοιτη,
    /// Like `ἱμᾱς`: 9 words.
    Ἱμᾱς,
    /// Like `θηρ`: 9 words.
    Θηρ,
    /// Like `ἀλειτης`: 9 words.
    Ἀλειτης,
    /// Like `πλᾰτεῑᾰ`: 9 words.
    Πλᾰτεῑᾰ,
    /// Like `γερᾰς`: 9 words.
    Γερᾰς,
    /// Like `νεῡρον`: 8 words.
    Νεῡρον,
    /// Like `μωλῠ`: 8 words.
    Μωλῠ,
    /// Like `κῠᾰρ`: 8 words.
    Κῠᾰρ,
    /// Like `ἀθῠρᾱς`: 8 words.
    Ἀθῠρᾱς,
    /// Like `ῥᾱξ`: 7 words.
    Ῥᾱξ,
    /// Like `πᾰτηρ`: 7 words.
    Πᾰτηρ,
    /// Like `ἠχω`: 7 words.
    Ἠχω,
    /// Like `χεῑμᾰ`: 7 words.
    Χεῑμᾰ,
    /// Like `ναυτης`: 7 words.
    Ναυτης,
    /// Like `πηχῠς`: 7 words.
    Πηχῠς,
    /// Like `γλωχῑς`: 7 words.
    Γλωχῑς,
    /// Like `μοῑρᾰ`: 6 words.
    Μοῑρᾰ,
    /// Like `σφην`: 6 words.
    Σφην,
    /// Like `μητρως`: 6 words.
    Μητρως,
    /// Like `παῡλᾰ`: 6 words.
    Παῡλᾰ,
    /// Like `γεῑσον`: 6 words.
    Γεῑσον,
    /// Like `πᾰπᾱς`: 6 words.
    Πᾰπᾱς,
    /// Like `ἀηρ`: 5 words.
    Ἀηρ,
    /// Like `δαιμων`: 5 words.
    Δαιμων,
    /// Like `κωμῡς`: 5 words.
    Κωμῡς,
    /// Like `γεῡσῐς`: 5 words.
    Γεῡσῐς,
    /// Like `νοῡς`: 5 words.
    Νοῡς,
    /// Like `λῐψ`: 5 words.
    Λῐψ,
    /// Like `τεῡχος`: 5 words.
    Τεῡχος,
    /// Like `χλουνης`: 5 words.
    Χλουνης,
    /// Like `ἀγλῑς`: 5 words.
    Ἀγλῑς,
    /// Like `λεῑμᾰξ`: 5 words.
    Λεῑμᾰξ,
    /// Like `φῡσᾱς`: 5 words.
    Φῡσᾱς,
    /// Like `πῠρᾰ`: 4 words.
    Πῠρᾰ,
    /// Like `λαυρᾱ`: 4 words.
    Λαυρᾱ,
    /// Like `ὀνῠξ`: 4 words.
    Ὀνῠξ,
    /// Like `πλευμων`: 4 words.
    Πλευμων,
    /// Like `πτῠξ`: 4 words.
    Πτῠξ,
    /// Like `κηρῠξ`: 4 words.
    Κηρῠξ,
    /// Like `δᾰσῠ`: 4 words.
    Δᾰσῠ,
    /// Like `ζωᾱ`: 4 words.
    Ζωᾱ,
    /// Like `γαλη`: 3 words.
    Γαλη,
    /// Like `θῑς`: 3 words.
    Θῑς,
    /// Like `μῡς`: 3 words.
    Μῡς,
    /// Like `νεῑκος`: 3 words.
    Νεῑκος,
    /// Like `πειθω`: 3 words.
    Πειθω,
    /// Like `πεῑνᾰ`: 3 words.
    Πεῑνᾰ,
    /// Like `ἐγκᾰτᾰ`: 3 words.
    Ἐγκᾰτᾰ,
    /// Like `λαλλαι`: 3 words.
    Λαλλαι,
    /// Like `ἐντεᾰ`: 3 words.
    Ἐντεᾰ,
    /// Like `παστα`: 3 words.
    Παστα,
    /// Like `γᾰμφηλαι`: 3 words.
    Γᾰμφηλαι,
    /// Like `ἐκπλους`: 3 words.
    Ἐκπλους,
    /// Like `θεωσῐς`: 3 words.
    Θεωσῐς,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 113] = [
        Declension::Τρᾰγος,
        Declension::Μορῐον,
        Declension::Οὐσῐᾱ,
        Declension::Νῠμφη,
        Declension::Ἀρῐθμος,
        Declension::Δῐαιρεσῐς,
        Declension::Θεμᾰ,
        Declension::Κᾰτᾰσκευη,
        Declension::Ἰδῐωτης,
        Declension::Κερκῐς,
        Declension::Κορᾰξ,
        Declension::Κλεις,
        Declension::Τρᾰπεζᾰ,
        Declension::Ἐπῐφᾰνειᾰ,
        Declension::Γενος,
        Declension::Αἰων,
        Declension::Χορευτης,
        Declension::Ἱππευς,
        Declension::Κεραιᾱ,
        Declension::Χᾰρᾰκτηρ,
        Declension::Κᾰλπῐς,
        Declension::Σημεῑον,
        Declension::Φορᾱ,
        Declension::Δᾰμᾰλης,
        Declension::Κοχλῐᾱς,
        Declension::Ὀργῐᾰ,
        Declension::Ζῠγον,
        Declension::Φᾰος,
        Declension::Ὀλῐνοι,
        Declension::Κᾰνων,
        Declension::Κροσσαι,
        Declension::Ῥητωρ,
        Declension::Κολλοψ,
        Declension::Στᾰχῠς,
        Declension::Ναῡλος,
        Declension::Ἀξων,
        Declension::Ῥωμαῑος,
        Declension::Δᾰνᾰοι,
        Declension::Λεων,
        Declension::Σᾰρξ,
        Declension::Λᾰτρῐς,
        Declension::Πους,
        Declension::Νηδῡς,
        Declension::Περᾰς,
        Declension::Πτερῠξ,
        Declension::Πνεῡμᾰ,
        Declension::Ἰκρῐα,
        Declension::Σηψ,
        Declension::Ῥως,
        Declension::Ἀσαῑοι,
        Declension::Ἁλως,
        Declension::Κηρ,
        Declension::Στοῑχος,
        Declension::Πεπερῐ,
        Declension::Τρωες,
        Declension::Λευκη,
        Declension::Ἱππομᾰνες,
        Declension::Φρην,
        Declension::Κοιτη,
        Declension::Ἱμᾱς,
        Declension::Θηρ,
        Declension::Ἀλειτης,
        Declension::Πλᾰτεῑᾰ,
        Declension::Γερᾰς,
        Declension::Νεῡρον,
        Declension::Μωλῠ,
        Declension::Κῠᾰρ,
        Declension::Ἀθῠρᾱς,
        Declension::Ῥᾱξ,
        Declension::Πᾰτηρ,
        Declension::Ἠχω,
        Declension::Χεῑμᾰ,
        Declension::Ναυτης,
        Declension::Πηχῠς,
        Declension::Γλωχῑς,
        Declension::Μοῑρᾰ,
        Declension::Σφην,
        Declension::Μητρως,
        Declension::Παῡλᾰ,
        Declension::Γεῑσον,
        Declension::Πᾰπᾱς,
        Declension::Ἀηρ,
        Declension::Δαιμων,
        Declension::Κωμῡς,
        Declension::Γεῡσῐς,
        Declension::Νοῡς,
        Declension::Λῐψ,
        Declension::Τεῡχος,
        Declension::Χλουνης,
        Declension::Ἀγλῑς,
        Declension::Λεῑμᾰξ,
        Declension::Φῡσᾱς,
        Declension::Πῠρᾰ,
        Declension::Λαυρᾱ,
        Declension::Ὀνῠξ,
        Declension::Πλευμων,
        Declension::Πτῠξ,
        Declension::Κηρῠξ,
        Declension::Δᾰσῠ,
        Declension::Ζωᾱ,
        Declension::Γαλη,
        Declension::Θῑς,
        Declension::Μῡς,
        Declension::Νεῑκος,
        Declension::Πειθω,
        Declension::Πεῑνᾰ,
        Declension::Ἐγκᾰτᾰ,
        Declension::Λαλλαι,
        Declension::Ἐντεᾰ,
        Declension::Παστα,
        Declension::Γᾰμφηλαι,
        Declension::Ἐκπλους,
        Declension::Θεωσῐς,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 113] = [
    // Τρᾰγος: 1,219 words (λῠκος, κοσμος, ἱππος, ἐρωμενος, σπογγος, ἀγγελος).
    declension!("τρᾰγος", 1219, "~ος";
    nom: "~ος", "~ω", "~οι";
    gen_: "~ου", "~οιν", "~ων";
    dat: "~ῳ", _, "~οις";
    acc: "~ον", _, "~ους";
    voc: "~ε", _, _;
    ),
    // Μορῐον: 1,051 words (ἀστρον, παιδῐον, δενδρον, μῠστηρῐον, ἐργον, εἰδωλον). Like Τρᾰγος, except in 4 cells: nom sg, nom pl, acc pl, voc sg.
    declension!("μορῐον", 1051, "~ον", [gender: Neuter], like Τρᾰγος;
    nom: "~ον", _, "~ᾰ";
    acc: _, _, "~ᾰ";
    voc: "~ον", _, _;
    ),
    // Οὐσῐᾱ: 626 words (δημοκρᾰτῐᾱ, χωρᾱ, μελῳδῐᾱ, φῐλῐᾱ, σοφῐᾱ, ἡγεμονῐᾱ).
    declension!("οὐσῐᾱ", 626, "~ᾱ", [gender: Feminine];
    nom: "~ᾱ", "~ᾱ", "~αι";
    gen_: "~ᾱς", "~αιν", "~ων";
    dat: "~ᾳ", _, "~αις";
    acc: "~ᾱν", _, "~ᾱς";
    voc: "~ᾱ", _, _;
    ),
    // Νῠμφη: 533 words (νῑκη, δῑνη, κορη, ἀγᾰπη, γῠνη, ἐλεημοσῠνη). Like Οὐσῐᾱ, except in 5 cells: nom sg, gen_ sg, dat sg, acc sg, voc sg.
    declension!("νῠμφη", 533, "~η", [gender: Feminine], like Οὐσῐᾱ;
    nom: "~η", _, _;
    gen_: "~ης", _, _;
    dat: "~ῃ", _, _;
    acc: "~ην", _, _;
    voc: "~η", _, _;
    ),
    // Ἀρῐθμος: 507 words (λαιμος, μᾰστος, θεος, ἰᾱτρος, καιρος, νεκρος). Like Τρᾰγος, except in 3 cells: gen_ sg, gen_ du, dat pl.
    declension!("ἀρῐθμος", 507, "~ος", [gender: Masculine], like Τρᾰγος;
    gen_: "~οῡ", "~οῑν", _;
    dat: _, _, "~οῑς";
    ),
    // Δῐαιρεσῐς: 491 words (πολῐς, γνωσῐς, κῠβερνησῐς, ἀποκᾰλῠψῐς, μορφωσῐς, φῠσῐς).
    declension!("δῐαιρεσῐς", 491, "~ῐς", [gender: Feminine];
    nom: "~ῐς", "~ει", "~εις";
    gen_: "~εως", "~εοιν", "~εων";
    dat: "~ει", _, "~εσῐ|~εσῐν";
    acc: "~ῐν", _, "~εις";
    voc: "~ῐ", _, _;
    ),
    // Θεμᾰ: 454 words (μελῐ, γρᾰμμᾰ, δερμᾰ, σῑγμᾰ, βαπτισμᾰ, στομᾰ).
    declension!("θεμᾰ", 454, "~";
    nom: "~", "~τε", "~τᾰ";
    gen_: "~τος", "~τοιν", "~των";
    dat: "~τῐ", _, "~σῐ|~σῐν";
    acc: "~", _, "~τᾰ";
    voc: "~", _, _;
    ),
    // Κᾰτᾰσκευη: 313 words (μηχᾰνη, μουσῐκη, γρᾰμμᾰτῐκη, φωνη, κεφᾰλη, ἡδονη). Like Νῠμφη, except in 2 cells: gen_ du, dat pl.
    declension!("κᾰτᾰσκευη", 313, "~η", [gender: Feminine], like Νῠμφη;
    gen_: _, "~αῑν", _;
    dat: _, _, "~αῑς";
    ),
    // Ἰδῐωτης: 219 words (προφητης, κλεπτης, ἰσρᾱηλῑτης, πολῑτης, ἱπποτοξοτης, τρωγλοδῠτης). Like Νῠμφη, except in 3 cells: nom sg, gen_ sg, voc sg.
    declension!("ἰδῐωτης", 219, "~ης", [gender: Masculine], like Νῠμφη;
    nom: "~ης", _, _;
    gen_: "~ου", _, _;
    voc: "~ᾰ", _, _;
    ),
    // Κερκῐς: 198 words (παῑς, λεπῐς, πᾰτρῐς, ἑλληνῐς, περσῐς, γρᾰφῐς).
    declension!("κερκῐς", 198, "~ς", [gender: Feminine];
    nom: "~ς", "~δε", "~δες";
    gen_: "~δος", "~δοιν", "~δων";
    dat: "~δῐ", _, "~σῐ|~σῐν";
    acc: "~δᾰ", _, "~δᾰς";
    voc: "~", _, _;
    ),
    // Κορᾰξ: 194 words (χᾰλῐξ, ἀνθρᾰξ, ἀβᾰξ, πτερῠξ, γλαῡξ, ἑλῐξ).
    declension!("κορᾰξ", 194, "~ξ";
    nom: "~ξ", "~κε", "~κες";
    gen_: "~κος", "~κοιν", "~κων";
    dat: "~κῐ", _, "~ξῐ|~ξῐν";
    acc: "~κᾰ", _, "~κᾰς";
    voc: "~ξ", _, _;
    ),
    // Κλεις: 176 words (μῠκης, κρης, σκληροτης, μᾰρτῠς, θης, κλεις). Like Κερκῐς, except in 1 cell: voc sg.
    declension!("κλεις", 176, "~ς", like Κερκῐς;
    voc: "~ς", _, _;
    ),
    // Τρᾰπεζᾰ: 144 words (θᾰλᾰσσᾰ, ῥῐζᾰ, θᾰλᾰττᾰ, δοξᾰ, γλωττᾰ, μελῐττᾰ). Like Νῠμφη, except in 3 cells: nom sg, acc sg, voc sg.
    declension!("τρᾰπεζᾰ", 144, "~ᾰ", [gender: Feminine], like Νῠμφη;
    nom: "~ᾰ", _, _;
    acc: "~ᾰν", _, _;
    voc: "~ᾰ", _, _;
    ),
    // Ἐπῐφᾰνειᾰ: 128 words (βᾰσῐλειᾰ, ἀσθενειᾰ, μᾰχαιρᾰ, ἀληθειᾰ, βοηθειᾰ, συγγενειᾰ). Like Τρᾰπεζᾰ, except in 2 cells: gen_ sg, dat sg.
    declension!("ἐπῐφᾰνειᾰ", 128, "~ᾰ", [gender: Feminine], like Τρᾰπεζᾰ;
    gen_: "~ᾱς", _, _;
    dat: "~ᾳ", _, _;
    ),
    // Γενος: 109 words (σκῠλος, κλεος, ἐθνος, πληθος, τελος, στηθος).
    declension!("γενος", 109, "~ος", [gender: Neuter];
    nom: "~ος", "~ει", "~η";
    gen_: "~ους", "~οῑν", "~ων";
    dat: "~ει", _, "~εσῐ|~εσῐν";
    acc: "~ος", _, "~η";
    voc: "~ος", _, _;
    ),
    // Αἰων: 98 words (ἑλλην, αἰων, λεγεων, ἀγων, σᾱπων, λᾰκων).
    declension!("αἰων", 98, "~ν", [gender: Masculine];
    nom: "~ν", "~νε", "~νες";
    gen_: "~νος", "~νοιν", "~νων";
    dat: "~νῐ", _, "~σῐ|~σῐν";
    acc: "~νᾰ", _, "~νᾰς";
    voc: "~ν", _, _;
    ),
    // Χορευτης: 94 words (ἐρᾰστης, ποιητης, ὑποκρῐτης, βᾰπτῐστης, κρῐτης, κωλοβᾰθρῐστης). Like Κᾰτᾰσκευη, except in 3 cells: nom sg, gen_ sg, voc sg.
    declension!("χορευτης", 94, "~ης", [gender: Masculine], like Κᾰτᾰσκευη;
    nom: "~ης", _, _;
    gen_: "~οῡ", _, _;
    voc: "~ᾰ", _, _;
    ),
    // Ἱππευς: 86 words (βᾰσῐλευς, ἀρχιερευς, γραμμᾰτευς, ἀλεξᾰνδρευς, αἰολευς, ἀμφισσευς).
    declension!("ἱππευς", 86, "~ευς", [gender: Masculine];
    nom: "~ευς", "~η", "~ης|~εῑς";
    gen_: "~εως", "~εοιν", "~εων";
    dat: "~εῑ", _, "~εῡσῐ|~εῡσῐν";
    acc: "~εᾱ", _, "~εᾱς";
    voc: "~εῡ", _, "~ης|~εῑς";
    ),
    // Κεραιᾱ: 80 words (ἰουδαιᾱ, προφητειᾱ, πορνειᾱ, παιδειᾱ, δουλειᾱ, ξῠλειᾱ).
    declension!("κεραιᾱ", 80, "~~ᾱ", [gender: Feminine];
    nom: "~~ᾱ", "~~ᾱ", "~ῑα~";
    gen_: "~~ᾱς", "~~αιν", "~~ων";
    dat: "~~ᾳ", _, "~~αις";
    acc: "~~ᾱν", _, "~~ᾱς";
    voc: "~~ᾱ", _, _;
    ),
    // Χᾰρᾰκτηρ: 77 words (μᾰρτῠρ, πᾰνθηρ, ἰβηρ, χειρ, ἐορ, κρᾱτηρ).
    declension!("χᾰρᾰκτηρ", 77, "~";
    nom: "~", "~ε", "~ες";
    gen_: "~ος", "~οιν", "~ων";
    dat: "~ῐ", _, "~σῐ|~σῐν";
    acc: "~ᾰ", _, "~ᾰς";
    voc: "~", _, _;
    ),
    // Κᾰλπῐς: 67 words (πελασγῐωτῐς, σπᾰρτῐᾱτῐς, σῠβᾰρῑτῐς, σμῠρῐς, ἀσπρῐς, εὐνῐς). Like Κερκῐς, except in 1 cell: acc sg.
    declension!("κᾰλπῐς", 67, "~ς", like Κερκῐς;
    acc: "~ν", _, _;
    ),
    // Σημεῑον: 64 words (θεῑον, πανδοχεῑον, πᾰνδοκεῑον, στοιχεῑον, ἀγγεῑον, βᾰλᾰνεῑον).
    declension!("σημεῑον", 64, "~ῑον", [gender: Neuter];
    nom: "~ῑον", "~ιω", "~ῑᾰ";
    gen_: "~ιου", "~ιοιν", "~ιων";
    dat: "~ιῳ", _, "~ιοις";
    acc: "~ῑον", _, "~ῑᾰ";
    voc: "~ῑον", _, _;
    ),
    // Φορᾱ: 62 words (παιδῐᾱ, ἀνθρᾰκῐᾱ, πᾰτρῐᾱ, γενεᾱ, θεᾱ, στρᾰτῐᾱ). Like Οὐσῐᾱ, except in 2 cells: gen_ du, dat pl.
    declension!("φορᾱ", 62, "~ᾱ", [gender: Feminine], like Οὐσῐᾱ;
    gen_: _, "~αῑν", _;
    dat: _, _, "~αῑς";
    ),
    // Δᾰμᾰλης: 57 words (βελγης, δακης, μῐνῠης, ὀδρῠσης, πῐσῐδης, σᾰκης). Like Ἰδῐωτης, except in 1 cell: voc sg.
    declension!("δᾰμᾰλης", 57, "~ης", [gender: Masculine], like Ἰδῐωτης;
    voc: "~η", _, _;
    ),
    // Κοχλῐᾱς: 49 words (νεᾱνῐᾱς, γᾰλᾰξῐᾱς, καικιᾱς, κῠβᾱς, τᾰμῐᾱς, κοχλῐᾱς). Like Οὐσῐᾱ, except in 2 cells: nom sg, gen_ sg.
    declension!("κοχλῐᾱς", 49, "~ᾱς", [gender: Masculine], like Οὐσῐᾱ;
    nom: "~ᾱς", _, _;
    gen_: "~ου", _, _;
    ),
    // Ὀργῐᾰ: 49 words (κᾰστᾰνειᾰ, κᾰστᾰνᾰ, ὀργῐᾰ, γαρσᾰνᾰ, αἱρεσῐᾰ, δῐονῡσῐᾰ). Like Μορῐον, except in 7 cells: nom sg, nom du, gen_ sg, gen_ du, dat sg, acc sg, ….
    declension!("ὀργῐᾰ", 49, "~ᾰ", [gender: Neuter], like Μορῐον;
    nom: "~ᾰ", derived, _;
    gen_: derived, derived, _;
    dat: derived, _, _;
    acc: derived, _, _;
    voc: derived, _, _;
    ),
    // Ζῠγον: 47 words (πτερον, ὀνομᾰστῐκον, ᾠον, ἑρπετον, ζῠγον, φῠτον). Like Μορῐον, except in 3 cells: gen_ sg, gen_ du, dat pl.
    declension!("ζῠγον", 47, "~ον", [gender: Neuter], like Μορῐον;
    gen_: "~οῡ", "~οῑν", _;
    dat: _, _, "~οῑς";
    ),
    // Φᾰος: 40 words (δᾰσος, πεος, σκᾰφος, φᾰος, πᾰχος, κᾰμπος). Like Γενος, except in 7 cells: nom du, nom pl, gen_ sg, gen_ du, gen_ pl, dat sg, ….
    declension!("φᾰος", 40, "~ος", [gender: Neuter], like Γενος;
    nom: _, "~εε", "~εᾰ";
    gen_: "~εος", "~εοιν", "~εων";
    dat: "~εϊ", _, _;
    acc: _, _, "~εᾰ";
    ),
    // Ὀλῐνοι: 40 words (πᾰρᾰβολᾱνοι, ἰσοι, μοσσῡνοικοι, ἡνῐοχοι, σῐνδοι, ἰστροι). Like Ὀργῐᾰ, except in 3 cells: nom sg, nom pl, acc pl.
    declension!("ὀλῐνοι", 40, "~οι", [gender: Masculine], like Ὀργῐᾰ;
    nom: "~οι", _, "~οι";
    acc: _, _, "~ους";
    ),
    // Κᾰνων: 39 words (ἡγεμων, τεκτων, ἀμᾰζων, ἀλκῠων, μᾰκεδων, μαιων).
    declension!("κᾰνων", 39, "~ων";
    nom: "~ων", "~ονε", "~ονες";
    gen_: "~ονος", "~ονοιν", "~ονων";
    dat: "~ονῐ", _, "~οσῐ|~οσῐν";
    acc: "~ονᾰ", _, "~ονᾰς";
    voc: "~ων", _, _;
    ),
    // Κροσσαι: 39 words (ἑβδομηκοντᾰκωμηται, τεσσᾰρεσκαιδεκᾰτῑται, ὀᾰσῑται, ἰσκαι, κρουπεζαι, ὀζολαι). Like Ἰδῐωτης, except in 0 cells: .
    declension!("κροσσαι", 39, "~αι", like Ἰδῐωτης;
    ),
    // Ῥητωρ: 34 words (κῠων, ἀλεκτωρ, καστωρ, ῥητωρ, δωτωρ, στατωρ).
    declension!("ῥητωρ", 34, "~ω~";
    nom: "~ω~", "~ο~ε", "~ο~ες";
    gen_: "~ο~ος", "~ο~οιν", "~ο~ων";
    dat: "~ο~ῐ", _, "~ο~σῐ|~ο~σῐν";
    acc: "~ο~ᾰ", _, "~ο~ᾰς";
    voc: "~ο~", _, _;
    ),
    // Κολλοψ: 32 words (αἰθῐοψ, κῠκλωψ, δολοψ, αἰγῐλωψ, ἀγχιλωψ, ὀψ).
    declension!("κολλοψ", 32, "~ψ";
    nom: "~ψ", "~πε", "~πες";
    gen_: "~πος", "~ποιν", "~πων";
    dat: "~πῐ", _, "~ψῐ|~ψῐν";
    acc: "~πᾰ", _, "~πᾰς";
    voc: "~ψ", _, _;
    ),
    // Στᾰχῠς: 32 words (λῐβῠς, λῐγῠς, γενῠς, ἐγχελῠς, ῥᾰπῠς, ῥᾰφῠς).
    declension!("στᾰχῠς", 32, "~ῠς";
    nom: "~ῠς", "~ῠε|~ῡ", "~ῠες|~ῡς";
    gen_: "~ῠος", "~ῠοιν", "~ῠων";
    dat: "~ῠϊ|~υι", _, "~ῠσῐ|~ῠσῐν";
    acc: "~ῠν", "~ῠε|~ῡ", "~ῡς|~ῠᾰς";
    voc: "~ῠ", "~ῠε|~ῡ", "~ῠες|~ῡς";
    ),
    // Ναῡλος: 28 words (ταῡρος, δοῡλος, σαῡρος, πλοῡτος, καρδοῡχος, οὐολοῡσκος).
    declension!("ναῡλος", 28, "~ῡ~ος", [gender: Masculine];
    nom: "~ῡ~ος", "~υ~ω", "~ῡ~οι";
    gen_: "~υ~ου", "~υ~οιν", "~υ~ων";
    dat: "~υ~ῳ", _, "~υ~οις";
    acc: "~ῡ~ον", _, "~υ~ους";
    voc: "~ῡ~ε", _, _;
    ),
    // Ἀξων: 27 words (ἀρχιτεκτων, βιστων, χᾱων, ἰᾱων, λῠκᾱων, βρᾰχῑων). Like Κᾰνων, except in 1 cell: voc sg.
    declension!("ἀξων", 27, "~ων", [gender: Masculine], like Κᾰνων;
    voc: "~ον", _, _;
    ),
    // Ῥωμαῑος: 26 words (ἰουδαῑος, φαρισαῑος, ῥωμαῑος, ἀνδρεῑος, ἑβραῑος, χᾰλδαῑος). Like Σημεῑον, except in 4 cells: nom sg, nom pl, acc pl, voc sg.
    declension!("ῥωμαῑος", 26, "~ῑος", [gender: Masculine], like Σημεῑον;
    nom: "~ῑος", _, "~ῑοι";
    acc: _, _, "~ιους";
    voc: "~ῑε", _, _;
    ),
    // Δᾰνᾰοι: 24 words (δᾰνᾰοι, τῐβᾰρηνοι, μεθοδῐκοι, πᾰλαιοι, ἐμπειρῐκοι, στοιχειωμᾰτῐκοι). Like Ὀλῐνοι, except in 1 cell: dat pl.
    declension!("δᾰνᾰοι", 24, "~οι", [gender: Masculine], like Ὀλῐνοι;
    dat: _, _, "~οῑς";
    ),
    // Λεων: 22 words (θερᾰπων, λεων, δρᾰκων, γερων, ὁρῐζων, χᾰμαιλεων).
    declension!("λεων", 22, "~ων", [gender: Masculine];
    nom: "~ων", "~οντε", "~οντες";
    gen_: "~οντος", "~οντοιν", "~οντων";
    dat: "~οντῐ", _, "~ουσῐ|~ουσῐν";
    acc: "~οντᾰ", _, "~οντᾰς";
    voc: "~ον", _, _;
    ),
    // Σᾰρξ: 21 words (σᾰρξ, θρᾳξ, φρῠξ, λῠγξ, προιξ, σφηξ). Like Κορᾰξ, except in 1 cell: gen_ du.
    declension!("σᾰρξ", 21, "~ξ", like Κορᾰξ;
    gen_: _, "~κοῑν", _;
    ),
    // Λᾰτρῐς: 19 words (ἀκρῐς, ὀκρῐς, λᾰτρῐς, μηνῐς, πορτῐς, ἀρδῐς).
    declension!("λᾰτρῐς", 19, "~ῐς";
    nom: "~ῐς", "~ῐε", "~ῐες";
    gen_: "~ῐος", "~ῐοιν", "~ῐων";
    dat: "~ῑ", _, "~ῐσῐ|~ῐσῐν";
    acc: "~ῐν", _, "~ῑς|~ῐᾰς";
    voc: "~ῐ", _, _;
    ),
    // Πους: 19 words (πους, ὀκτωπους, πολυπους, τρῐπους, ἀπους, δᾰσῠπους). Like Κερκῐς, except in 2 cells: nom sg, voc sg.
    declension!("πους", 19, "~υς", [gender: Masculine], like Κερκῐς;
    nom: "~υς", _, _;
    voc: "~υς", _, _;
    ),
    // Νηδῡς: 18 words (ἰχθῡς, βοῡς, δελφῡς, ὀφρῡς, γρᾱῡς, λῐγνῡς). Like Στᾰχῠς, except in 4 cells: nom sg, dat sg, acc sg, voc sg.
    declension!("νηδῡς", 18, "~ῡς", like Στᾰχῠς;
    nom: "~ῡς", _, _;
    dat: "~ῠϊ|~υῑ", _, _;
    acc: "~ῡν", _, _;
    voc: "~ῡ", _, _;
    ),
    // Περᾰς: 16 words (φως, τερᾰς, χᾰρῐς, κρεᾰς, πᾰγκρεᾰς, ἐρῠσῐπελᾰς). Like Θεμᾰ, except in 3 cells: nom sg, acc sg, voc sg.
    declension!("περᾰς", 16, "~ς", like Θεμᾰ;
    nom: "~ς", _, _;
    acc: "~ς", _, _;
    voc: "~ς", _, _;
    ),
    // Πτερῠξ: 14 words (ἰᾱπῠξ, λᾰτᾰξ, ὀρτῠξ, μᾰστῑξ, τεττῑξ, λᾰλᾰξ).
    declension!("πτερῠξ", 14, "~ξ";
    nom: "~ξ", "~γε", "~γες";
    gen_: "~γος", "~γοιν", "~γων";
    dat: "~γῐ", _, "~ξῐ|~ξῐν";
    acc: "~γᾰ", _, "~γᾰς";
    voc: "~ξ", _, _;
    ),
    // Πνεῡμᾰ: 14 words (πνεῡμᾰ, καῡμᾰ, ῥεῡμᾰ, θαῡμᾰ, ζεῡγμᾰ, τραῡμᾰ).
    declension!("πνεῡμᾰ", 14, "~ῡ~", [gender: Neuter];
    nom: "~ῡ~", "~υ~τε", "~υ~τᾰ";
    gen_: "~υ~τος", "~υ~τοιν", "~υ~των";
    dat: "~υ~τῐ", _, "~υ~σῐ|~υ~σῐν";
    acc: "~ῡ~", _, "~υ~τᾰ";
    voc: "~ῡ~", _, _;
    ),
    // Ἰκρῐα: 14 words (γενεσια, μουστακια, σαγματοπασμαγαδια, εὐληρα, βρῡτεα, ἰσα). Like Ὀργῐᾰ, except in 1 cell: nom sg.
    declension!("ἰκρῐα", 14, "~α", [gender: Neuter], like Ὀργῐᾰ;
    nom: "~α", _, _;
    ),
    // Σηψ: 13 words (ὠψ, γρῡψ, θρῑψ, θωψ, κλωψ, σκωψ). Like Κολλοψ, except in 1 cell: gen_ du.
    declension!("σηψ", 13, "~ψ", like Κολλοψ;
    gen_: _, "~ποῑν", _;
    ),
    // Ῥως: 13 words (ἑξηκοντᾰ, ὀσσε, ὑφεν, ῥᾰββῑ, ῥᾰββονῑ, ῥᾰββουνῑ).
    declension!("ῥως", 13, "~";
    nom: "~", "~", "~";
    gen_: "~", "~", "~";
    dat: "~", _, "~";
    acc: "~", _, "~";
    voc: "~", _, _;
    ),
    // Ἀσαῑοι: 13 words (βοττῐαῑοι, ἀραμαῑοι, ἀρδιαῑοι, ἀσαῑοι, θᾰμᾰναῑοι, νῐψαῑοι). Like Ῥωμαῑος, except in 0 cells: .
    declension!("ἀσαῑοι", 13, "~ῑοι", [gender: Masculine], like Ῥωμαῑος;
    ),
    // Ἁλως: 12 words (νεως, γᾰλως, τᾰως, λᾰγως, ἑως, λεως).
    declension!("ἁλως", 12, "~ως";
    nom: "~ως", "~ω", "~ῳ";
    gen_: "~ω", "~ῳν", "~ων";
    dat: "~ῳ", _, "~ῳς";
    acc: "~ω|~ων", _, "~ως";
    voc: "~ως", _, _;
    ),
    // Κηρ: 12 words (κηρ, ἐᾰρ, θενᾰρ, ὀνᾰρ, νεκτᾰρ, μελᾰν). Like Χᾰρᾰκτηρ, except in 3 cells: nom pl, acc sg, acc pl.
    declension!("κηρ", 12, "~", [gender: Neuter], like Χᾰρᾰκτηρ;
    nom: _, _, "~ᾰ";
    acc: "~", _, "~ᾰ";
    ),
    // Στοῑχος: 12 words (χοῑρος, ἑταῑρος, στοῑχος, τοῑχος, σχοῑνος, ῥοῑζος).
    declension!("στοῑχος", 12, "~ῑ~ος", [gender: Masculine];
    nom: "~ῑ~ος", "~ι~ω", "~ῑ~οι";
    gen_: "~ι~ου", "~ι~οιν", "~ι~ων";
    dat: "~ι~ῳ", _, "~ι~οις";
    acc: "~ῑ~ον", _, "~ι~ους";
    voc: "~ῑ~ε", _, _;
    ),
    // Πεπερῐ: 11 words (σῐνᾱπῐ, κομμῐ, πεπερῐ, κιννᾰβᾰρῐ, ἀμῐ, στῐμμῐ). Like Δῐαιρεσῐς, except in 5 cells: nom sg, nom pl, acc sg, acc pl, voc pl.
    declension!("πεπερῐ", 11, "~ῐ", [gender: Neuter], like Δῐαιρεσῐς;
    nom: "~ῐ", _, "~η|~εᾰ";
    acc: "~ῐ", _, "~η|~εᾰ";
    voc: _, _, "~η|~εᾰ";
    ),
    // Τρωες: 11 words (πῑερες, τρωες, κηφηνες, ἁλτηρες, αἰγες, ἀνᾰξῠρῐδες). Like Ὀργῐᾰ, except in 4 cells: nom sg, nom pl, dat pl, acc pl.
    declension!("τρωες", 11, "~ες", like Ὀργῐᾰ;
    nom: "~ες", _, "~ες";
    dat: _, _, "~σῐ|~σῐν";
    acc: _, _, "~ᾰς";
    ),
    // Λευκη: 10 words (πευκη, δουλη, βλαυτη, σενδουκη, λευκη, ζευγλη).
    declension!("λευκη", 10, "~υ~η", [gender: Feminine];
    nom: "~υ~η", "~υ~ᾱ", "~ῡ~αι";
    gen_: "~υ~ης", "~υ~αιν", "~υ~ων";
    dat: "~υ~ῃ", _, "~υ~αις";
    acc: "~υ~ην", _, "~υ~ᾱς";
    voc: "~υ~η", _, _;
    ),
    // Ἱππομᾰνες: 10 words (νευροειδες, μονοειδες, τρῐχομᾰνες, ἡμεροκᾰλλες, σησᾰμοειδες, ὠκῐμοειδες). Like Γενος, except in 6 cells: nom sg, nom du, gen_ sg, dat sg, acc sg, voc sg.
    declension!("ἱππομᾰνες", 10, "~ες", [gender: Neuter], like Γενος;
    nom: "~ες", "~εῑ", _;
    gen_: "~οῡς", _, _;
    dat: "~εῑ", _, _;
    acc: "~ες", _, _;
    voc: "~ες", _, _;
    ),
    // Φρην: 9 words (φρην, ὑμην, ποιμην, πῠθμην, ἀδην, αὐχην).
    declension!("φρην", 9, "~ην";
    nom: "~ην", "~ενε", "~ενες";
    gen_: "~ενος", "~ενοιν", "~ενων";
    dat: "~ενῐ", _, "~εσῐ|~εσῐν";
    acc: "~ενᾰ", _, "~ενᾰς";
    voc: "~ην", _, _;
    ),
    // Κοιτη: 9 words (χαιτη, κοιτη, ἐρεικη, θοινη, βαιτη, μαινη).
    declension!("κοιτη", 9, "~ι~η", [gender: Feminine];
    nom: "~ι~η", "~ι~ᾱ", "~ῑ~αι";
    gen_: "~ι~ης", "~ι~αιν", "~ι~ων";
    dat: "~ι~ῃ", _, "~ι~αις";
    acc: "~ι~ην", _, "~ι~ᾱς";
    voc: "~ι~η", _, _;
    ),
    // Ἱμᾱς: 9 words (κορῠβᾱς, γῐγᾱς, ἐλεφᾱς, ἀδᾰμᾱς, ἀνδρῐᾱς, λῠκᾰβᾱς).
    declension!("ἱμᾱς", 9, "~ᾱς", [gender: Masculine];
    nom: "~ᾱς", "~ᾰντε", "~ᾰντες";
    gen_: "~ᾰντος", "~ᾰντοιν", "~ᾰντων";
    dat: "~ᾰντῐ", _, "~ᾱσῐ|~ᾱσῐν";
    acc: "~ᾰντᾰ", _, "~ᾰντᾰς";
    voc: "~ᾰν", _, _;
    ),
    // Θηρ: 9 words (κᾱρ, θηρ, ψᾱρ, φωρ, σηρ, φηρ). Like Χᾰρᾰκτηρ, except in 1 cell: gen_ du.
    declension!("θηρ", 9, "~", like Χᾰρᾰκτηρ;
    gen_: _, "~οῑν", _;
    ),
    // Ἀλειτης: 9 words (ὠρειτης, παικτης, σωρειτης, σᾰμᾰρειτης, ἀρσενοκοιτης, μητροκοιτης). Like Κοιτη, except in 3 cells: nom sg, gen_ sg, voc sg.
    declension!("ἀλειτης", 9, "~ι~ης", [gender: Masculine], like Κοιτη;
    nom: "~ι~ης", _, _;
    gen_: "~ι~ου", _, _;
    voc: "~ῑ~ᾰ", _, _;
    ),
    // Πλᾰτεῑᾰ: 9 words (μυῑᾰ, πλᾰτεῑᾰ, γαῑᾰ, μαῑᾰ, βᾰρεῑᾰ, ὀξεῑᾰ).
    declension!("πλᾰτεῑᾰ", 9, "~ῑᾰ", [gender: Feminine];
    nom: "~ῑᾰ", "~ιᾱ", "~ῑαι";
    gen_: "~ιᾱς", "~ιαιν", "~ιων";
    dat: "~ιᾳ", _, "~ιαις";
    acc: "~ῑᾰν", _, "~ιᾱς";
    voc: "~ῑᾰ", _, _;
    ),
    // Γερᾰς: 9 words (ἁλς, ὑς, ὀϊς, γερᾰς, δεπᾰς, ψεφᾰς). Like Κηρ, except in 5 cells: nom sg, dat sg, dat pl, acc sg, voc sg.
    declension!("γερᾰς", 9, "~ς", like Κηρ;
    nom: "~ς", _, _;
    dat: "~ϊ", _, "~σῐ";
    acc: "~ς", _, _;
    voc: "~ς", _, _;
    ),
    // Νεῡρον: 8 words (προῡμνον, νεῡρον, βοῡστον, τεῡτλον, σοῡσον, λοῡσσον). Like Ναῡλος, except in 4 cells: nom sg, nom pl, acc pl, voc sg.
    declension!("νεῡρον", 8, "~ῡ~ον", [gender: Neuter], like Ναῡλος;
    nom: "~ῡ~ον", _, "~ῡ~ᾰ";
    acc: _, _, "~ῡ~ᾰ";
    voc: "~ῡ~ον", _, _;
    ),
    // Μωλῠ: 8 words (δᾰκρῠ, μωλῠ, νᾱπῠ, ἀσχῠ, μῐσῠ, βρᾰθῠ). Like Στᾰχῠς, except in 5 cells: nom sg, nom pl, acc sg, acc pl, voc pl.
    declension!("μωλῠ", 8, "~ῠ", [gender: Neuter], like Στᾰχῠς;
    nom: "~ῠ", _, "~ῠᾰ";
    acc: "~ῠ", _, "~ῠᾰ";
    voc: _, _, derived;
    ),
    // Κῠᾰρ: 8 words (οὐθᾰρ, ἡπᾰρ, φρεᾱρ, ἀλειφᾰρ, στεᾱρ, δελεᾰρ). Like Θεμᾰ, except in 3 cells: nom sg, acc sg, voc sg.
    declension!("κῠᾰρ", 8, "~ρ", [gender: Neuter], like Θεμᾰ;
    nom: "~ρ", _, _;
    acc: "~ρ", _, _;
    voc: "~ρ", _, _;
    ),
    // Ἀθῠρᾱς: 8 words (πᾰξᾰμᾱς, μᾰρῐκᾱς, ἀττᾰγᾱς, μᾰσουχᾱς, βᾰσκᾱς, χαλιφᾱς). Like Φορᾱ, except in 3 cells: nom sg, nom pl, gen_ sg.
    declension!("ἀθῠρᾱς", 8, "~ᾱς", [gender: Masculine], like Φορᾱ;
    nom: "~ᾱς", _, "~αῑ";
    gen_: "~ᾱ", _, _;
    ),
    // Ῥᾱξ: 7 words (φλοξ, αἰξ, σφᾰξ, ῥᾱξ, στρῐξ, ῥωξ). Like Πτερῠξ, except in 1 cell: gen_ du.
    declension!("ῥᾱξ", 7, "~ξ", like Πτερῠξ;
    gen_: _, "~γοῑν", _;
    ),
    // Πᾰτηρ: 7 words (ἀνηρ, μητηρ, πᾰτηρ, ἀστηρ, θῠγᾰτηρ, ἐνᾰτηρ).
    declension!("πᾰτηρ", 7, "~η~";
    nom: "~η~", "~ε~ε", "~ε~ες";
    gen_: "~~ος", "~ε~οιν", "~ε~ων";
    dat: "~~ῐ", _, "~~ᾰσῐ";
    acc: "~ε~ᾰ", _, "~ε~ᾰς";
    voc: "~ε~", _, _;
    ),
    // Ἠχω: 7 words (ἠχω, τυτω, κῐττω, πῑπω, μοτω, φειδω). Like Ἀρῐθμος, except in 5 cells: nom sg, gen_ sg, dat sg, acc sg, voc sg.
    declension!("ἠχω", 7, "~ω", [gender: Feminine], like Ἀρῐθμος;
    nom: "~ω", _, _;
    gen_: "~οῡς", _, _;
    dat: "~οῑ", _, _;
    acc: "~ω", _, _;
    voc: "~οῑ", _, _;
    ),
    // Χεῑμᾰ: 7 words (χεῑμᾰ, δεῑγμᾰ, σεῑσμᾰ, λεῑμμᾰ, λαῑτμᾰ, πεῑσμᾰ).
    declension!("χεῑμᾰ", 7, "~ῑ~", [gender: Neuter];
    nom: "~ῑ~", "~ι~τε", "~ι~τᾰ";
    gen_: "~ι~τος", "~ι~τοιν", "~ι~των";
    dat: "~ι~τῐ", _, "~ι~σῐ|~ι~σῐν";
    acc: "~ῑ~", _, "~ι~τᾰ";
    voc: "~ῑ~", _, _;
    ),
    // Ναυτης: 7 words (ναυτης, θραυστης, κοκκοθραυστης, ἀργοναυτης, ἐμφῠτευτης, ψευστης). Like Λευκη, except in 3 cells: nom sg, gen_ sg, voc sg.
    declension!("ναυτης", 7, "~υ~ης", [gender: Masculine], like Λευκη;
    nom: "~υ~ης", _, _;
    gen_: "~υ~ου", _, _;
    voc: "~ῡ~ᾰ", _, _;
    ),
    // Πηχῠς: 7 words (ἡμῐσῠς, υἱῠς, πελεκῠς, πρεσβῠς, πηχῠς, ἀμφῐβρᾰχῠς). Like Δῐαιρεσῐς, except in 3 cells: nom sg, acc sg, voc sg.
    declension!("πηχῠς", 7, "~ῠς", [gender: Masculine], like Δῐαιρεσῐς;
    nom: "~ῠς", _, _;
    acc: "~ῠν", _, _;
    voc: "~ῠ", _, _;
    ),
    // Γλωχῑς: 7 words (ἀκτῑς, ὠδῑς, γλωχῑς, στᾰμῑς, ἑρμῑς, ὀρκῡς). Like Αἰων, except in 2 cells: nom sg, voc sg.
    declension!("γλωχῑς", 7, "~ς", like Αἰων;
    nom: "~ς", _, _;
    voc: "~ς", _, _;
    ),
    // Μοῑρᾰ: 6 words (μοῑρᾰ, στεῑρᾰ, πεῑρᾰ, σφαῑρᾰ, σπεῑρᾰ, ἀγρεῑφνᾰ). Like Ἀλειτης, except in 4 cells: nom sg, gen_ sg, dat sg, acc sg.
    declension!("μοῑρᾰ", 6, "~ῑ~ᾰ", [gender: Feminine], like Ἀλειτης;
    nom: "~ῑ~ᾰ", _, _;
    gen_: "~ι~ᾱς", _, _;
    dat: "~ι~ᾳ", _, _;
    acc: "~ῑ~ᾰν", _, _;
    ),
    // Σφην: 6 words (μην, χην, σπλην, κλων, σφην, ψην). Like Αἰων, except in 1 cell: gen_ du.
    declension!("σφην", 6, "~ν", [gender: Masculine], like Αἰων;
    gen_: _, "~νοῑν", _;
    ),
    // Μητρως: 6 words (τρως, ἡρως, δμως, πᾰτρως, θως, μητρως).
    declension!("μητρως", 6, "~ως", [gender: Masculine];
    nom: "~ως", "~ωε", "~ωες";
    gen_: "~ωος", "~ωοιν", "~ωων";
    dat: "~ῳ|~ωϊ", _, "~ωσῐ|~ωσῐν";
    acc: "~ω", _, "~ωᾰς";
    voc: "~ως", _, _;
    ),
    // Παῡλᾰ: 6 words (μοῡσᾰ, βοῡλλᾰ, πᾰνδοῡρᾰ, παῡλᾰ, γοῡρνᾰ, κοῡπᾰ). Like Λευκη, except in 3 cells: nom sg, acc sg, voc sg.
    declension!("παῡλᾰ", 6, "~ῡ~ᾰ", [gender: Feminine], like Λευκη;
    nom: "~ῡ~ᾰ", _, _;
    acc: "~ῡ~ᾰν", _, _;
    voc: "~ῡ~ᾰ", _, _;
    ),
    // Γεῑσον: 6 words (δεῑπνον, σπεῑρον, κλεῑθρον, σεῑστρον, ῥεῑθρον, γεῑσον). Like Στοῑχος, except in 4 cells: nom sg, nom pl, acc pl, voc sg.
    declension!("γεῑσον", 6, "~ῑ~ον", [gender: Neuter], like Στοῑχος;
    nom: "~ῑ~ον", _, "~ῑ~ᾰ";
    acc: _, _, "~ῑ~ᾰ";
    voc: "~ῑ~ον", _, _;
    ),
    // Πᾰπᾱς: 6 words (πᾰπᾱς, γᾰλᾰτᾱς, κᾰδῡτᾱς, γενετᾱς, σᾰκτᾱς, πᾰππᾱς). Like Κοχλῐᾱς, except in 1 cell: voc sg.
    declension!("πᾰπᾱς", 6, "~ᾱς", [gender: Masculine], like Κοχλῐᾱς;
    voc: "~ᾰ", _, _;
    ),
    // Ἀηρ: 5 words (γᾰστηρ, ἀηρ, φρᾱτηρ, αἰθηρ, ἀθηρ). Like Πᾰτηρ, except in 4 cells: gen_ sg, dat sg, dat pl, voc sg.
    declension!("ἀηρ", 5, "~η~", like Πᾰτηρ;
    gen_: "~ε~ος", _, _;
    dat: "~ε~ῐ", _, "~ε~σῐ|~ε~σῐν";
    voc: "~η~", _, _;
    ),
    // Δαιμων: 5 words (δαιμων, μειζων, γειτων, ποτᾰμογειτων, σεισων).
    declension!("δαιμων", 5, "~ι~ων", [gender: Masculine];
    nom: "~ι~ων", "~ι~ονε", "~ι~ονες";
    gen_: "~ι~ονος", "~ι~ονοιν", "~ι~ονων";
    dat: "~ι~ονῐ", _, "~ι~οσῐ|~ι~οσῐν";
    acc: "~ι~ονᾰ", _, "~ι~ονᾰς";
    voc: "~ῑ~ον", _, _;
    ),
    // Κωμῡς: 5 words (ἑλμῐνς, κωμῡς, κορῠς, πειρῐνς, ἀγνῡς).
    declension!("κωμῡς", 5, "~ς";
    nom: "~ς", "~θε", "~θες";
    gen_: "~θος", "~θοιν", "~θων";
    dat: "~θῐ", _, "~σῐ|~σῐν";
    acc: "~θᾰ", _, "~θᾰς";
    voc: "~ς", _, _;
    ),
    // Γεῡσῐς: 5 words (παῡσῐς, λοῡσῐς, ῥεῡσῐς, ζεῡξῐς, γεῡσῐς).
    declension!("γεῡσῐς", 5, "~ῡ~ῐς", [gender: Feminine];
    nom: "~ῡ~ῐς", "~υ~ει", "~υ~εις";
    gen_: "~υ~εως", "~υ~εοιν", "~υ~εων";
    dat: "~υ~ει", _, "~υ~εσῐ|~υ~εσῐν";
    acc: "~ῡ~ῐν", _, "~υ~εις";
    voc: "~ῡ~ῐ", _, _;
    ),
    // Νοῡς: 5 words (νοῡς, πλοῡς, θῠγατρῐδοῡς, ῥοῡς, λουτροχοῡς).
    declension!("νοῡς", 5, "~οῡς", [gender: Masculine];
    nom: "~οῡς", "~ω|~ω", "~οῑ";
    gen_: "~οῡ", "~οῑν", "~ων";
    dat: "~ῳ", _, "~οῑς";
    acc: "~οῡν", "~ω|~ω", "~οῡς";
    voc: "~οῡ", "~ω|~ω", _;
    ),
    // Λῐψ: 5 words (χᾰλῠψ, φλεψ, χερνῐψ, φᾰψ, λῐψ).
    declension!("λῐψ", 5, "~ψ";
    nom: "~ψ", "~βε", "~βες";
    gen_: "~βος", "~βοῑν", "~βων";
    dat: "~βῐ", _, "~ψῐ|~ψῐν";
    acc: "~βᾰ", _, "~βᾰς";
    voc: "~ψ", _, _;
    ),
    // Τεῡχος: 5 words (τεῡχος, γλεῡκος, ψεῡδος, ζεῡγος, κεῡθος).
    declension!("τεῡχος", 5, "~ῡ~ος", [gender: Neuter];
    nom: "~ῡ~ος", "~υ~ει", "~υ~η";
    gen_: "~υ~ους", "~υ~οῑν", "~υ~ων";
    dat: "~υ~ει", _, "~υ~εσῐ|~υ~εσῐν";
    acc: "~ῡ~ος", _, "~υ~η";
    voc: "~ῡ~ος", _, _;
    ),
    // Χλουνης: 5 words (τῠμβαυλης, χλουνης, σῐδευνης, χοραυλης, πρωταυλης). Like Ναυτης, except in 1 cell: voc sg.
    declension!("χλουνης", 5, "~υ~ης", [gender: Masculine], like Ναυτης;
    voc: "~υ~η", _, _;
    ),
    // Ἀγλῑς: 5 words (ὀρνῑς, γελγῑς, ἀγλῑς, δελλῑς, μερμῑς). Like Κωμῡς, except in 2 cells: acc sg, voc sg.
    declension!("ἀγλῑς", 5, "~ς", like Κωμῡς;
    acc: "~ν", _, _;
    voc: "~", _, _;
    ),
    // Λεῑμᾰξ: 5 words (χοῑνῐξ, κᾰταῑτῠξ, μεῑρᾰξ, λεῑμᾰξ, βαῑβῠξ).
    declension!("λεῑμᾰξ", 5, "~ῑ~ξ";
    nom: "~ῑ~ξ", "~ι~κε", "~ι~κες";
    gen_: "~ι~κος", "~ι~κοιν", "~ι~κων";
    dat: "~ι~κῐ", _, "~ι~ξῐ|~ι~ξῐν";
    acc: "~ι~κᾰ", _, "~ι~κᾰς";
    voc: "~ῑ~ξ", _, _;
    ),
    // Φῡσᾱς: 5 words (πελεκᾱς, πῡρᾰμοῡς, φῡσᾱς, πλᾰκοῡς, ἀλλᾱς).
    declension!("φῡσᾱς", 5, "~ς", [gender: Masculine];
    nom: "~ς", "~ντε", "~ντες";
    gen_: "~ντος", "~ντοιν", "~ντων";
    dat: "~ντῐ", _, "~σῐ|~σῐν";
    acc: "~ντᾰ", _, "~ντᾰς";
    voc: "~ν", _, _;
    ),
    // Πῠρᾰ: 4 words (πῠρᾰ, πολῑτῐκᾰ, ἐσθλᾰ, τρωϊκᾰ). Like Ὀργῐᾰ, except in 1 cell: dat pl.
    declension!("πῠρᾰ", 4, "~ᾰ", [gender: Neuter], like Ὀργῐᾰ;
    dat: _, _, "~οῑς";
    ),
    // Λαυρᾱ: 4 words (σαυρᾱ, λαυρᾱ, κᾰρουχᾱ, βρουτζᾱ). Like Λευκη, except in 5 cells: nom sg, gen_ sg, dat sg, acc sg, voc sg.
    declension!("λαυρᾱ", 4, "~υ~ᾱ", [gender: Feminine], like Λευκη;
    nom: "~υ~ᾱ", _, _;
    gen_: "~υ~ᾱς", _, _;
    dat: "~υ~ᾳ", _, _;
    acc: "~υ~ᾱν", _, _;
    voc: "~υ~ᾱ", _, _;
    ),
    // Ὀνῠξ: 4 words (ὀνῠξ, σᾰρδονῠξ, ὑστρῐξ, ἀδδῐξ).
    declension!("ὀνῠξ", 4, "~ξ";
    nom: "~ξ", "~χε", "~χες";
    gen_: "~χος", "~χοιν", "~χων";
    dat: "~χῐ", _, "~ξῐ|~ξῐν";
    acc: "~χᾰ", _, "~χᾰς";
    voc: "~ξ", _, _;
    ),
    // Πλευμων: 4 words (πλευμων, πνευμων, μουσμων, ἰχνευμων).
    declension!("πλευμων", 4, "~υ~ων", [gender: Masculine];
    nom: "~υ~ων", "~υ~ονε", "~υ~ονες";
    gen_: "~υ~ονος", "~υ~ονοιν", "~υ~ονων";
    dat: "~υ~ονῐ", _, "~υ~οσῐ|~υ~οσῐν";
    acc: "~υ~ονᾰ", _, "~υ~ονᾰς";
    voc: "~ῡ~ον", _, _;
    ),
    // Πτῠξ: 4 words (ψῑξ, στῐξ, βηξ, πτῠξ). Like Ὀνῠξ, except in 1 cell: gen_ du.
    declension!("πτῠξ", 4, "~ξ", like Ὀνῠξ;
    gen_: _, "~χοῑν", _;
    ),
    // Κηρῠξ: 4 words (κοκκῠξ, κηρῠξ, ψευδοκηρῠξ, ἱεροκηρῠξ).
    declension!("κηρῠξ", 4, "~ῠξ", [gender: Masculine];
    nom: "~ῠξ", "~ῡκε", "~ῡκες";
    gen_: "~ῡκος", "~ῡκοιν", "~ῡκων";
    dat: "~ῡκῐ", _, "~ῡξῐ|~ῡξῐν";
    acc: "~ῡκᾰ", _, "~ῡκᾰς";
    voc: "~ῠξ", _, _;
    ),
    // Δᾰσῠ: 4 words (ἀστῠ, ὀξῠ, δᾰσῠ, πλᾰτῠ). Like Φᾰος, except in 5 cells: nom sg, nom du, dat sg, acc sg, voc sg.
    declension!("δᾰσῠ", 4, "~ῠ", like Φᾰος;
    nom: "~ῠ", "~εῑ", _;
    dat: "~εῑ", _, _;
    acc: "~ῠ", _, _;
    voc: "~ῠ", _, _;
    ),
    // Ζωᾱ: 4 words (γονᾱ, δρᾰχμᾱ, σφᾰγᾱ, ζωᾱ). Like Φορᾱ, except in 3 cells: gen_ pl, dat pl, acc pl.
    declension!("ζωᾱ", 4, "~ᾱ", [gender: Feminine], like Φορᾱ;
    gen_: _, _, "~ᾱων|~ᾱν";
    dat: _, _, "~αῑς|~αῑσῐ|~αῑσῐν";
    acc: _, _, "~ᾰς";
    ),
    // Γαλη: 3 words (γη, μῡγᾰλη, γαλη). Like Κᾰτᾰσκευη, except in 1 cell: nom pl.
    declension!("γαλη", 3, "~η", [gender: Feminine], like Κᾰτᾰσκευη;
    nom: _, _, "~αῑ";
    ),
    // Θῑς: 3 words (ῥῑς, ἰς, θῑς). Like Γλωχῑς, except in 1 cell: gen_ du.
    declension!("θῑς", 3, "~ς", like Γλωχῑς;
    gen_: _, "~νοῑν", _;
    ),
    // Μῡς: 3 words (σῡς, δρῡς, μῡς). Like Νηδῡς, except in 2 cells: gen_ du, dat sg.
    declension!("μῡς", 3, "~ῡς", like Νηδῡς;
    gen_: _, "~ῠοῑν", _;
    dat: "~ῠϊ|~υι", _, _;
    ),
    // Νεῑκος: 3 words (τεῑχος, χεῑλος, νεῑκος).
    declension!("νεῑκος", 3, "~ῑ~ος", [gender: Neuter];
    nom: "~ῑ~ος", "~ι~ει", "~ι~η";
    gen_: "~ι~ους", "~ι~οῑν", "~ι~ων";
    dat: "~ι~ει", _, "~ι~εσῐ|~ι~εσῐν";
    acc: "~ῑ~ος", _, "~ι~η";
    voc: "~ῑ~ος", _, _;
    ),
    // Πειθω: 3 words (λεχω, πειθω, κερδω). Like Ἠχω, except in 1 cell: gen_ sg.
    declension!("πειθω", 3, "~ω", [gender: Feminine], like Ἠχω;
    gen_: "~οος|~οῡς", _, _;
    ),
    // Πεῑνᾰ: 3 words (πεῑσᾰ, χλαῑνᾰ, πεῑνᾰ). Like Μοῑρᾰ, except in 2 cells: gen_ sg, dat sg.
    declension!("πεῑνᾰ", 3, "~ῑ~ᾰ", [gender: Feminine], like Μοῑρᾰ;
    gen_: "~ι~ης", _, _;
    dat: "~ι~ῃ", _, _;
    ),
    // Ἐγκᾰτᾰ: 3 words (ἐγκᾰτᾰ, κεδμᾰτᾰ, ἰχθῠημᾰτᾰ). Like Τρωες, except in 4 cells: nom sg, nom pl, gen_ pl, acc pl.
    declension!("ἐγκᾰτᾰ", 3, "~τᾰ", [gender: Neuter], like Τρωες;
    nom: "~τᾰ", _, "~τᾰ";
    gen_: _, _, "~των";
    acc: _, _, "~τᾰ";
    ),
    // Λαλλαι: 3 words (κᾰλανδαι, χαρωνῑται, λαλλαι).
    declension!("λαλλαι", 3, "~α~αι";
    nom: "~α~αι", _, "~ᾰ~αι";
    gen_: _, _, "~ᾰ~ων";
    dat: _, _, "~ᾰ~αις";
    acc: _, _, "~ᾰ~ᾱς";
    ),
    // Ἐντεᾰ: 3 words (ἐντεᾰ, δηνεᾰ, τειρεᾰ). Like Ὀργῐᾰ, except in 1 cell: dat pl.
    declension!("ἐντεᾰ", 3, "~ᾰ", [gender: Neuter], like Ὀργῐᾰ;
    dat: _, _, "~σῐ|~σῐν";
    ),
    // Παστα: 3 words (χεδροπα, παστα, ἀλμενιχιακα). Like Ἰκρῐα, except in 1 cell: dat pl.
    declension!("παστα", 3, "~α", [gender: Neuter], like Ἰκρῐα;
    dat: _, _, "~οῑς";
    ),
    // Γᾰμφηλαι: 3 words (γᾰμφηλαι, ταυρεασται, δοκηται). Like Ὀργῐᾰ, except in 4 cells: nom sg, nom pl, dat pl, acc pl.
    declension!("γᾰμφηλαι", 3, "~αι", like Ὀργῐᾰ;
    nom: "~αι", _, "~αι";
    dat: _, _, "~αῑς";
    acc: _, _, "~ᾱς";
    ),
    // Ἐκπλους: 3 words (πᾰραπλους, ἐκπλους, ἀγχῐνους). Like Τρᾰγος, except in 3 cells: nom sg, acc sg, voc sg.
    declension!("ἐκπλους", 3, "~ους", [gender: Masculine], like Τρᾰγος;
    nom: "~ους", _, _;
    acc: "~ουν", _, _;
    voc: "~ου", _, _;
    ),
    // Θεωσῐς: 3 words (θεωσῐς, ἀλθησῐς, ζαχαρῐς). Like Δῐαιρεσῐς, except in 6 cells: nom du, nom pl, gen_ du, dat sg, dat pl, acc pl.
    declension!("θεωσῐς", 3, "~ῐς", [gender: Feminine], like Δῐαιρεσῐς;
    nom: _, derived, "~εες";
    gen_: _, derived, _;
    dat: "~εϊ", _, "~εσῐ";
    acc: _, _, "~εᾰς";
    ),
];
