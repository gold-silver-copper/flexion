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
/// they share them), the declension it is like (if any), the stem slots
/// it states with a stem and an ending table (`stem aorist: "~σ- like λυω"`:
/// the slot's cells are `λυω`'s table's patterns with `~σ` for their `~`; see
/// `stems.rs`), then a row of patterns per line (`derived`: the form comes
/// from another cell's, see `relations.rs`; `_`, or a row left out: the
/// form is its stem slot's, or the declension's it is like, or, in a declension like
/// no other, derived):
/// `nom` (sg, du, pl)
/// `gen_` (sg, du, pl)
/// `dat` (sg, du, pl)
/// `acc` (sg, du, pl)
/// `voc` (sg, du, pl)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, [$($f:ident: $v:ident),*])? $(, like $parent:ident $(as $view:ident)?)?;
     $(stem noun_sg: $s0:tt;)?
     $(stem noun: $s1:tt;)?
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

/// An inherent feature's value, as `(feature, value)` by their places.
macro_rules! feature {
    (gender: $v:ident) => {
        (0, crate::Gender::$v as u8)
    };
}

/// A view: per cell (in the cells' order), the cell of the declension it is
/// like that it reads (`sg_as_du`: one value read as another).
const SG_AS_DU: [u16; 15] = [1, 1, 2, 4, 4, 5, 7, 7, 8, 10, 10, 11, 13, 13, 14];

/// A view: per cell (in the cells' order), the cell of the declension it is
/// like that it reads (`pl_as_sg`: one value read as another).
const PL_AS_SG: [u16; 15] = [0, 1, 0, 3, 4, 3, 6, 7, 6, 9, 10, 9, 12, 13, 12];

/// A view: per cell (in the cells' order), the cell of the declension it is
/// like that it reads (`acc_as_voc`: one value read as another).
const ACC_AS_VOC: [u16; 15] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 12, 13, 14, 12, 13, 14];

/// A view: per cell (in the cells' order), the cell of the declension it is
/// like that it reads (`dat_as_voc`: one value read as another).
const DAT_AS_VOC: [u16; 15] = [0, 1, 2, 3, 4, 5, 12, 13, 14, 9, 10, 11, 12, 13, 14];

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `τρᾰγος`: 1,216 words.
    Τρᾰγος,
    /// Like `μορῐον`: 1,050 words.
    Μορῐον,
    /// Like `οὐσῐᾱ`: 626 words.
    Οὐσῐᾱ,
    /// Like `νῠμφη`: 532 words.
    Νῠμφη,
    /// Like `ἀρῐθμος`: 507 words.
    Ἀρῐθμος,
    /// Like `δῐαιρεσῐς`: 491 words.
    Δῐαιρεσῐς,
    /// Like `θεμᾰ`: 433 words.
    Θεμᾰ,
    /// Like `κᾰτᾰσκευη`: 313 words.
    Κᾰτᾰσκευη,
    /// Like `ἰδῐωτης`: 219 words.
    Ἰδῐωτης,
    /// Like `κερκῐς`: 196 words.
    Κερκῐς,
    /// Like `κορᾰξ`: 194 words.
    Κορᾰξ,
    /// Like `κρηπῑς`: 172 words.
    Κρηπῑς,
    /// Like `τρᾰπεζᾰ`: 144 words.
    Τρᾰπεζᾰ,
    /// Like `ἐπῐφᾰνειᾰ`: 128 words.
    Ἐπῐφᾰνειᾰ,
    /// Like `γενος`: 109 words.
    Γενος,
    /// Like `αἰων`: 98 words.
    Αἰων,
    /// Like `χορευτης`: 91 words.
    Χορευτης,
    /// Like `ἱππευς`: 86 words.
    Ἱππευς,
    /// Like `κεραιᾱ`: 80 words.
    Κεραιᾱ,
    /// Like `χᾰρᾰκτηρ`: 76 words.
    Χᾰρᾰκτηρ,
    /// Like `κᾰλπῐς`: 66 words.
    Κᾰλπῐς,
    /// Like `σημεῑον`: 64 words.
    Σημεῑον,
    /// Like `φορᾱ`: 62 words.
    Φορᾱ,
    /// Like `δᾰμᾰλης`: 57 words.
    Δᾰμᾰλης,
    /// Like `ὀργῐᾰ`: 52 words.
    Ὀργῐᾰ,
    /// Like `κοχλῐᾱς`: 49 words.
    Κοχλῐᾱς,
    /// Like `ζῠγον`: 47 words.
    Ζῠγον,
    /// Like `ὀλῐνοι`: 41 words.
    Ὀλῐνοι,
    /// Like `φᾰος`: 40 words.
    Φᾰος,
    /// Like `κροσσαι`: 39 words.
    Κροσσαι,
    /// Like `κᾰνων`: 39 words.
    Κᾰνων,
    /// Like `ῥητωρ`: 34 words.
    Ῥητωρ,
    /// Like `στᾰχῠς`: 32 words.
    Στᾰχῠς,
    /// Like `κολλοψ`: 32 words.
    Κολλοψ,
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
    /// Like `λᾰτρῐς`: 20 words.
    Λᾰτρῐς,
    /// Like `πους`: 19 words.
    Πους,
    /// Like `νηδῡς`: 15 words.
    Νηδῡς,
    /// Like `πτερῠξ`: 14 words.
    Πτερῠξ,
    /// Like `πνεῡμᾰ`: 14 words.
    Πνεῡμᾰ,
    /// Like `σηψ`: 13 words.
    Σηψ,
    /// Like `ἀσαῑοι`: 13 words.
    Ἀσαῑοι,
    /// Like `ῥως`: 13 words.
    Ῥως,
    /// Like `ἰκρῐα`: 12 words.
    Ἰκρῐα,
    /// Like `ἁλως`: 12 words.
    Ἁλως,
    /// Like `στοῑχος`: 12 words.
    Στοῑχος,
    /// Like `κηρ`: 12 words.
    Κηρ,
    /// Like `περᾰς`: 11 words.
    Περᾰς,
    /// Like `πεπερῐ`: 11 words.
    Πεπερῐ,
    /// Like `τρωες`: 11 words.
    Τρωες,
    /// Like `λευκη`: 10 words.
    Λευκη,
    /// Like `ἱππομᾰνες`: 10 words.
    Ἱππομᾰνες,
    /// Like `πλᾰτεῑᾰ`: 10 words.
    Πλᾰτεῑᾰ,
    /// Like `φρην`: 9 words.
    Φρην,
    /// Like `κοιτη`: 9 words.
    Κοιτη,
    /// Like `ἱμᾱς`: 9 words.
    Ἱμᾱς,
    /// Like `ἀλειτης`: 9 words.
    Ἀλειτης,
    /// Like `νεῡρον`: 8 words.
    Νεῡρον,
    /// Like `θηρ`: 8 words.
    Θηρ,
    /// Like `μωλῠ`: 8 words.
    Μωλῠ,
    /// Like `κῠᾰρ`: 8 words.
    Κῠᾰρ,
    /// Like `ἀθῠρᾱς`: 8 words.
    Ἀθῠρᾱς,
    /// Like `ῥᾱξ`: 7 words.
    Ῥᾱξ,
    /// Like `ἠχω`: 7 words.
    Ἠχω,
    /// Like `χεῑμᾰ`: 7 words.
    Χεῑμᾰ,
    /// Like `ναυτης`: 7 words.
    Ναυτης,
    /// Like `γλωχῑς`: 7 words.
    Γλωχῑς,
    /// Like `πᾰτηρ`: 7 words.
    Πᾰτηρ,
    /// Like `μοῑρᾰ`: 6 words.
    Μοῑρᾰ,
    /// Like `σφην`: 6 words.
    Σφην,
    /// Like `μητρως`: 6 words.
    Μητρως,
    /// Like `γεῑσον`: 6 words.
    Γεῑσον,
    /// Like `γερᾰς`: 6 words.
    Γερᾰς,
    /// Like `θης`: 6 words.
    Θης,
    /// Like `παῡλᾰ`: 6 words.
    Παῡλᾰ,
    /// Like `πᾰπᾱς`: 6 words.
    Πᾰπᾱς,
    /// Like `δαιμων`: 5 words.
    Δαιμων,
    /// Like `κωμῡς`: 5 words.
    Κωμῡς,
    /// Like `γεῡσῐς`: 5 words.
    Γεῡσῐς,
    /// Like `νοῡς`: 5 words.
    Νοῡς,
    /// Like `τεῡχος`: 5 words.
    Τεῡχος,
    /// Like `χλουνης`: 5 words.
    Χλουνης,
    /// Like `ἀγλῑς`: 5 words.
    Ἀγλῑς,
    /// Like `ἀηρ`: 5 words.
    Ἀηρ,
    /// Like `λεῑμᾰξ`: 5 words.
    Λεῑμᾰξ,
    /// Like `κρεᾰς`: 5 words.
    Κρεᾰς,
    /// Like `λῐψ`: 5 words.
    Λῐψ,
    /// Like `φῡσᾱς`: 5 words.
    Φῡσᾱς,
    /// Like `πῠρᾰ`: 4 words.
    Πῠρᾰ,
    /// Like `λαυρᾱ`: 4 words.
    Λαυρᾱ,
    /// Like `ὀνῠξ`: 4 words.
    Ὀνῠξ,
    /// Like `ἁλς`: 4 words.
    Ἁλς,
    /// Like `πλευμων`: 4 words.
    Πλευμων,
    /// Like `πτῠξ`: 4 words.
    Πτῠξ,
    /// Like `πηχῠς`: 4 words.
    Πηχῠς,
    /// Like `ζωᾱ`: 4 words.
    Ζωᾱ,
    /// Like `κηρῠξ`: 4 words.
    Κηρῠξ,
    /// Like `δᾰσῠ`: 4 words.
    Δᾰσῠ,
    /// Like `γαλη`: 3 words.
    Γαλη,
    /// Like `κρῐτης`: 3 words.
    Κρῐτης,
    /// Like `υἱῠς`: 3 words.
    Υἱῠς,
    /// Like `χοῡς`: 3 words.
    Χοῡς,
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
    /// Like `ὀδους`: 3 words.
    Ὀδους,
    /// Like `λαλλαι`: 3 words.
    Λαλλαι,
    /// Like `χειρ`: 3 words.
    Χειρ,
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
    /// Like `δυσμαι`: 2 words.
    Δυσμαι,
    /// Like `γλωσσᾰ`: 1 words.
    Γλωσσᾰ,
    /// Like `ἱερος`: 1 words.
    Ἱερος,
    /// Like `ἱερον`: 1 words.
    Ἱερον,
    /// Like `ἱερα`: 1 words.
    Ἱερα,
    /// Like `μηριᾰ`: 1 words.
    Μηριᾰ,
    /// Like `ἰᾱλεμος`: 1 words.
    Ἰᾱλεμος,
    /// Like `κῠνοχᾰλκη`: 1 words.
    Κῠνοχᾰλκη,
    /// Like `λιμεναρχης`: 1 words.
    Λιμεναρχης,
    /// Like `βυσναῑοι`: 1 words.
    Βυσναῑοι,
    /// Like `ἐπισφυρῐᾰ`: 1 words.
    Ἐπισφυρῐᾰ,
    /// Like `πᾰραφερνᾰ`: 1 words.
    Πᾰραφερνᾰ,
    /// Like `βακχῐαδαι`: 1 words.
    Βακχῐαδαι,
    /// Like `κοσμητης`: 1 words.
    Κοσμητης,
    /// Like `χᾰριστῐᾰ`: 1 words.
    Χᾰριστῐᾰ,
    /// Like `ὀρθογραφος`: 1 words.
    Ὀρθογραφος,
    /// Like `ἑλληνοτᾰμιαι`: 1 words.
    Ἑλληνοτᾰμιαι,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 138] = [
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
        Declension::Κρηπῑς,
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
        Declension::Ὀργῐᾰ,
        Declension::Κοχλῐᾱς,
        Declension::Ζῠγον,
        Declension::Ὀλῐνοι,
        Declension::Φᾰος,
        Declension::Κροσσαι,
        Declension::Κᾰνων,
        Declension::Ῥητωρ,
        Declension::Στᾰχῠς,
        Declension::Κολλοψ,
        Declension::Ναῡλος,
        Declension::Ἀξων,
        Declension::Ῥωμαῑος,
        Declension::Δᾰνᾰοι,
        Declension::Λεων,
        Declension::Σᾰρξ,
        Declension::Λᾰτρῐς,
        Declension::Πους,
        Declension::Νηδῡς,
        Declension::Πτερῠξ,
        Declension::Πνεῡμᾰ,
        Declension::Σηψ,
        Declension::Ἀσαῑοι,
        Declension::Ῥως,
        Declension::Ἰκρῐα,
        Declension::Ἁλως,
        Declension::Στοῑχος,
        Declension::Κηρ,
        Declension::Περᾰς,
        Declension::Πεπερῐ,
        Declension::Τρωες,
        Declension::Λευκη,
        Declension::Ἱππομᾰνες,
        Declension::Πλᾰτεῑᾰ,
        Declension::Φρην,
        Declension::Κοιτη,
        Declension::Ἱμᾱς,
        Declension::Ἀλειτης,
        Declension::Νεῡρον,
        Declension::Θηρ,
        Declension::Μωλῠ,
        Declension::Κῠᾰρ,
        Declension::Ἀθῠρᾱς,
        Declension::Ῥᾱξ,
        Declension::Ἠχω,
        Declension::Χεῑμᾰ,
        Declension::Ναυτης,
        Declension::Γλωχῑς,
        Declension::Πᾰτηρ,
        Declension::Μοῑρᾰ,
        Declension::Σφην,
        Declension::Μητρως,
        Declension::Γεῑσον,
        Declension::Γερᾰς,
        Declension::Θης,
        Declension::Παῡλᾰ,
        Declension::Πᾰπᾱς,
        Declension::Δαιμων,
        Declension::Κωμῡς,
        Declension::Γεῡσῐς,
        Declension::Νοῡς,
        Declension::Τεῡχος,
        Declension::Χλουνης,
        Declension::Ἀγλῑς,
        Declension::Ἀηρ,
        Declension::Λεῑμᾰξ,
        Declension::Κρεᾰς,
        Declension::Λῐψ,
        Declension::Φῡσᾱς,
        Declension::Πῠρᾰ,
        Declension::Λαυρᾱ,
        Declension::Ὀνῠξ,
        Declension::Ἁλς,
        Declension::Πλευμων,
        Declension::Πτῠξ,
        Declension::Πηχῠς,
        Declension::Ζωᾱ,
        Declension::Κηρῠξ,
        Declension::Δᾰσῠ,
        Declension::Γαλη,
        Declension::Κρῐτης,
        Declension::Υἱῠς,
        Declension::Χοῡς,
        Declension::Θῑς,
        Declension::Μῡς,
        Declension::Νεῑκος,
        Declension::Πειθω,
        Declension::Πεῑνᾰ,
        Declension::Ἐγκᾰτᾰ,
        Declension::Ὀδους,
        Declension::Λαλλαι,
        Declension::Χειρ,
        Declension::Ἐντεᾰ,
        Declension::Παστα,
        Declension::Γᾰμφηλαι,
        Declension::Ἐκπλους,
        Declension::Θεωσῐς,
        Declension::Δυσμαι,
        Declension::Γλωσσᾰ,
        Declension::Ἱερος,
        Declension::Ἱερον,
        Declension::Ἱερα,
        Declension::Μηριᾰ,
        Declension::Ἰᾱλεμος,
        Declension::Κῠνοχᾰλκη,
        Declension::Λιμεναρχης,
        Declension::Βυσναῑοι,
        Declension::Ἐπισφυρῐᾰ,
        Declension::Πᾰραφερνᾰ,
        Declension::Βακχῐαδαι,
        Declension::Κοσμητης,
        Declension::Χᾰριστῐᾰ,
        Declension::Ὀρθογραφος,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 138] = [
    // Τρᾰγος: 1,216 words (λῠκος, κοσμος, ἱππος, ἐρωμενος, σπογγος, ἀγγελος).
    declension!("τρᾰγος", 1216, "~ος";
    stem noun_sg: "~- like τρᾰγος";
    nom: _, "~ω", "~οι";
    gen_: "~ου", "~οιν", "~ων";
    dat: "~ῳ", _, "~οις";
    acc: "~ον", _, "~ους";
    ),
    // Μορῐον: 1,050 words (ἀστρον, παιδῐον, δενδρον, μῠστηρῐον, ἐργον, εἰδωλον). Like Τρᾰγος, except in 2 cells: nom pl, acc pl.
    declension!("μορῐον", 1050, "~ον", [gender: Neuter], like Τρᾰγος;
    stem noun_sg: "~ον- like μορῐον";
    nom: _, _, "~ᾰ";
    acc: _, _, "~ᾰ";
    ),
    // Οὐσῐᾱ: 626 words (δημοκρᾰτῐᾱ, χωρᾱ, μελῳδῐᾱ, φῐλῐᾱ, σοφῐᾱ, ἡγεμονῐᾱ).
    declension!("οὐσῐᾱ", 626, "~ᾱ", [gender: Feminine];
    stem noun_sg: "~ᾱ- like μορῐον";
    nom: _, "~ᾱ", "~αι";
    gen_: "~ᾱς", "~αιν", "~ων";
    dat: "~ᾳ", _, "~αις";
    acc: "~ᾱν", _, "~ᾱς";
    ),
    // Νῠμφη: 532 words (νῑκη, δῑνη, κορη, ἀγᾰπη, γῠνη, ἐλεημοσῠνη). Like Οὐσῐᾱ, except in 3 cells: gen_ sg, dat sg, acc sg.
    declension!("νῠμφη", 532, "~η", [gender: Feminine], like Οὐσῐᾱ;
    stem noun_sg: "~η- like μορῐον";
    gen_: "~ης", _, _;
    dat: "~ῃ", _, _;
    acc: "~ην", _, _;
    ),
    // Ἀρῐθμος: 507 words (λαιμος, μᾰστος, θεος, ἰᾱτρος, καιρος, νεκρος). Like Τρᾰγος, except in 3 cells: gen_ sg, gen_ du, dat pl.
    declension!("ἀρῐθμος", 507, "~ος", [gender: Masculine], like Τρᾰγος;
    gen_: "~οῡ", "~οῑν", _;
    dat: _, _, "~οῑς";
    ),
    // Δῐαιρεσῐς: 491 words (πολῐς, γνωσῐς, κῠβερνησῐς, ἀποκᾰλῠψῐς, μορφωσῐς, φῠσῐς).
    declension!("δῐαιρεσῐς", 491, "~ῐς", [gender: Feminine];
    stem noun_sg: "~ῐ- like δῐαιρεσῐς";
    nom: _, "~ει", "~εις";
    gen_: "~εως", "~εοιν", "~εων";
    dat: "~ει", _, "~εσῐ|~εσῐν";
    acc: "~ῐν", _, "~εις";
    ),
    // Θεμᾰ: 433 words (μελῐ, γρᾰμμᾰ, δερμᾰ, σῑγμᾰ, βαπτισμᾰ, στομᾰ).
    declension!("θεμᾰ", 433, "~";
    stem noun_sg: "~- like μορῐον";
    nom: _, "~τε", "~τᾰ";
    gen_: "~τος", "~τοιν", "~των";
    dat: "~τῐ", _, "~σῐ|~σῐν";
    acc: "~", _, "~τᾰ";
    ),
    // Κᾰτᾰσκευη: 313 words (μηχᾰνη, μουσῐκη, γρᾰμμᾰτῐκη, φωνη, κεφᾰλη, ἡδονη). Like Νῠμφη, except in 2 cells: gen_ du, dat pl.
    declension!("κᾰτᾰσκευη", 313, "~η", [gender: Feminine], like Νῠμφη;
    gen_: _, "~αῑν", _;
    dat: _, _, "~αῑς";
    ),
    // Ἰδῐωτης: 219 words (προφητης, κλεπτης, ἰσρᾱηλῑτης, πολῑτης, ἱπποτοξοτης, τρωγλοδῠτης). Like Νῠμφη, except in 1 cell: gen_ sg.
    declension!("ἰδῐωτης", 219, "~ης", [gender: Masculine], like Νῠμφη;
    stem noun_sg: "~- like ἰδῐωτης";
    gen_: "~ου", _, _;
    ),
    // Κερκῐς: 196 words (παῑς, λεπῐς, πᾰτρῐς, ἑλληνῐς, περσῐς, γρᾰφῐς).
    declension!("κερκῐς", 196, "~ς", [gender: Feminine];
    stem noun_sg: "~- like δῐαιρεσῐς";
    nom: _, "~δε", "~δες";
    gen_: "~δος", "~δοιν", "~δων";
    dat: "~δῐ", _, "~σῐ|~σῐν";
    acc: "~δᾰ", _, "~δᾰς";
    ),
    // Κορᾰξ: 194 words (χᾰλῐξ, ἀνθρᾰξ, ἀβᾰξ, πτερῠξ, γλαῡξ, ἑλῐξ).
    declension!("κορᾰξ", 194, "~ξ";
    stem noun_sg: "~ξ- like μορῐον";
    nom: _, "~κε", "~κες";
    gen_: "~κος", "~κοιν", "~κων";
    dat: "~κῐ", _, "~ξῐ|~ξῐν";
    acc: "~κᾰ", _, "~κᾰς";
    ),
    // Κρηπῑς: 172 words (μῠκης, σκληροτης, μᾰρτῠς, κλεις, ἁψῑς, χλᾰμῠς). Like Κερκῐς, except in 1 cell: voc sg.
    declension!("κρηπῑς", 172, "~ς", like Κερκῐς;
    voc: "~ς", _, _;
    ),
    // Τρᾰπεζᾰ: 144 words (θᾰλᾰσσᾰ, ῥῐζᾰ, θᾰλᾰττᾰ, δοξᾰ, γλωττᾰ, μελῐττᾰ). Like Νῠμφη, except in 1 cell: acc sg.
    declension!("τρᾰπεζᾰ", 144, "~ᾰ", [gender: Feminine], like Νῠμφη;
    stem noun_sg: "~ᾰ- like μορῐον";
    acc: "~ᾰν", _, _;
    ),
    // Ἐπῐφᾰνειᾰ: 128 words (βᾰσῐλειᾰ, ἀσθενειᾰ, μᾰχαιρᾰ, ἀληθειᾰ, βοηθειᾰ, συγγενειᾰ). Like Τρᾰπεζᾰ, except in 2 cells: gen_ sg, dat sg.
    declension!("ἐπῐφᾰνειᾰ", 128, "~ᾰ", [gender: Feminine], like Τρᾰπεζᾰ;
    gen_: "~ᾱς", _, _;
    dat: "~ᾳ", _, _;
    ),
    // Γενος: 109 words (σκῠλος, κλεος, ἐθνος, πληθος, τελος, στηθος).
    declension!("γενος", 109, "~ος", [gender: Neuter];
    stem noun_sg: "~ος- like μορῐον";
    nom: _, "~ει", "~η";
    gen_: "~ους", "~οῑν", "~ων";
    dat: "~ει", _, "~εσῐ|~εσῐν";
    acc: "~ος", _, "~η";
    ),
    // Αἰων: 98 words (ἑλλην, αἰων, λεγεων, ἀγων, σᾱπων, λᾰκων).
    declension!("αἰων", 98, "~ν", [gender: Masculine];
    stem noun_sg: "~ν- like μορῐον";
    nom: _, "~νε", "~νες";
    gen_: "~νος", "~νοιν", "~νων";
    dat: "~νῐ", _, "~σῐ|~σῐν";
    acc: "~νᾰ", _, "~νᾰς";
    ),
    // Χορευτης: 91 words (ἐρᾰστης, ποιητης, ὑποκρῐτης, βᾰπτῐστης, κωλοβᾰθρῐστης, σοφῐστης). Like Κᾰτᾰσκευη, except in 1 cell: gen_ sg.
    declension!("χορευτης", 91, "~ης", [gender: Masculine], like Κᾰτᾰσκευη;
    stem noun_sg: "~- like ἰδῐωτης";
    gen_: "~οῡ", _, _;
    ),
    // Ἱππευς: 86 words (βᾰσῐλευς, ἀρχιερευς, γραμμᾰτευς, ἀλεξᾰνδρευς, αἰολευς, ἀμφισσευς).
    declension!("ἱππευς", 86, "~ευς", [gender: Masculine];
    nom: "~ευς", "~η", "~ης|~εῑς";
    gen_: "~εως", "~εοιν", "~εων";
    dat: "~εῑ", _, "~εῡσῐ|~εῡσῐν";
    acc: "~εᾱ", _, "~εᾱς";
    voc: "~εῡ", _, _;
    ),
    // Κεραιᾱ: 80 words (ἰουδαιᾱ, προφητειᾱ, πορνειᾱ, παιδειᾱ, δουλειᾱ, ξῠλειᾱ).
    declension!("κεραιᾱ", 80, "~~ᾱ", [gender: Feminine];
    stem noun_sg: "~~ᾱ- like μορῐον";
    nom: _, "~~ᾱ", "~ῑα~";
    gen_: "~~ᾱς", "~~αιν", "~~ων";
    dat: "~~ᾳ", _, "~~αις";
    acc: "~~ᾱν", _, "~~ᾱς";
    ),
    // Χᾰρᾰκτηρ: 76 words (μᾰρτῠρ, πᾰνθηρ, ἰβηρ, ἐορ, κρᾱτηρ, ἀροτηρ).
    declension!("χᾰρᾰκτηρ", 76, "~", [gender: Masculine];
    stem noun_sg: "~- like μορῐον";
    nom: _, "~ε", "~ες";
    gen_: "~ος", "~οιν", "~ων";
    dat: "~ῐ", _, "~σῐ|~σῐν";
    acc: "~ᾰ", _, "~ᾰς";
    ),
    // Κᾰλπῐς: 66 words (πελασγῐωτῐς, σπᾰρτῐᾱτῐς, σῠβᾰρῑτῐς, σμῠρῐς, ἀσπρῐς, εὐνῐς). Like Κερκῐς, except in 1 cell: acc sg.
    declension!("κᾰλπῐς", 66, "~ς", like Κερκῐς;
    acc: "~ν", _, _;
    ),
    // Σημεῑον: 64 words (θεῑον, πανδοχεῑον, πᾰνδοκεῑον, στοιχεῑον, ἀγγεῑον, βᾰλᾰνεῑον).
    declension!("σημεῑον", 64, "~ῑον", [gender: Neuter];
    stem noun_sg: "~ῑον- like μορῐον";
    nom: _, "~ιω", "~ῑᾰ";
    gen_: "~ιου", "~ιοιν", "~ιων";
    dat: "~ιῳ", _, "~ιοις";
    acc: "~ῑον", _, "~ῑᾰ";
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
    // Ὀργῐᾰ: 52 words (κᾰστᾰνειᾰ, κᾰστᾰνᾰ, ὀργῐᾰ, γαρσᾰνᾰ, αἱρεσῐᾰ, δῐονῡσῐᾰ). Like Μορῐον, except in 4 cells: nom sg, nom du, gen_ sg, gen_ du.
    declension!("ὀργῐᾰ", 52, "~ᾰ", [gender: Neuter], like Μορῐον as SG_AS_DU;
    nom: "~ᾰ", derived, _;
    gen_: derived, derived, _;
    ),
    // Κοχλῐᾱς: 49 words (νεᾱνῐᾱς, γᾰλᾰξῐᾱς, καικιᾱς, κῠβᾱς, τᾰμῐᾱς, κοχλῐᾱς). Like Οὐσῐᾱ, except in 1 cell: gen_ sg.
    declension!("κοχλῐᾱς", 49, "~ᾱς", [gender: Masculine], like Οὐσῐᾱ;
    stem noun_sg: "~ᾱ- like δῐαιρεσῐς";
    gen_: "~ου", _, _;
    ),
    // Ζῠγον: 47 words (πτερον, ὀνομᾰστῐκον, ᾠον, ἑρπετον, ζῠγον, φῠτον). Like Μορῐον, except in 3 cells: gen_ sg, gen_ du, dat pl.
    declension!("ζῠγον", 47, "~ον", [gender: Neuter], like Μορῐον;
    gen_: "~οῡ", "~οῑν", _;
    dat: _, _, "~οῑς";
    ),
    // Ὀλῐνοι: 41 words (πᾰρᾰβολᾱνοι, ἰδῐοι, ἰσοι, μοσσῡνοικοι, ἡνῐοχοι, σῐνδοι). Like Ὀργῐᾰ, except in 3 cells: nom sg, nom pl, acc pl.
    declension!("ὀλῐνοι", 41, "~οι", [gender: Masculine], like Ὀργῐᾰ;
    nom: "~οι", _, "~οι";
    acc: _, _, "~ους";
    ),
    // Φᾰος: 40 words (δᾰσος, πεος, σκᾰφος, φᾰος, πᾰχος, κᾰμπος). Like Γενος, except in 7 cells: nom du, nom pl, gen_ sg, gen_ du, gen_ pl, dat sg, ….
    declension!("φᾰος", 40, "~ος", [gender: Neuter], like Γενος;
    nom: _, "~εε", "~εᾰ";
    gen_: "~εος", "~εοιν", "~εων";
    dat: "~εϊ", _, _;
    acc: _, _, "~εᾰ";
    ),
    // Κροσσαι: 39 words (ἑβδομηκοντᾰκωμηται, τεσσᾰρεσκαιδεκᾰτῑται, ὀᾰσῑται, ἰσκαι, κρουπεζαι, ὀζολαι). Like Ἰδῐωτης, except in 0 cells: .
    declension!("κροσσαι", 39, "~αι", like Ἰδῐωτης;
    ),
    // Κᾰνων: 39 words (ἡγεμων, τεκτων, ἀμᾰζων, ἀλκῠων, μᾰκεδων, μαιων).
    declension!("κᾰνων", 39, "~ων";
    stem noun_sg: "~ων- like μορῐον";
    nom: _, "~ονε", "~ονες";
    gen_: "~ονος", "~ονοιν", "~ονων";
    dat: "~ονῐ", _, "~οσῐ|~οσῐν";
    acc: "~ονᾰ", _, "~ονᾰς";
    ),
    // Ῥητωρ: 34 words (κῠων, ἀλεκτωρ, καστωρ, ῥητωρ, δωτωρ, στατωρ).
    declension!("ῥητωρ", 34, "~ω~";
    nom: "~ω~", "~ο~ε", "~ο~ες";
    gen_: "~ο~ος", "~ο~οιν", "~ο~ων";
    dat: "~ο~ῐ", _, "~ο~σῐ|~ο~σῐν";
    acc: "~ο~ᾰ", _, "~ο~ᾰς";
    voc: "~ο~", _, _;
    ),
    // Στᾰχῠς: 32 words (λῐβῠς, λῐγῠς, γενῠς, ἐγχελῠς, ῥᾰπῠς, ῥᾰφῠς).
    declension!("στᾰχῠς", 32, "~ῠς";
    stem noun_sg: "~ῠ- like δῐαιρεσῐς";
    nom: _, "~ῠε|~ῡ", "~ῠες|~ῡς";
    gen_: "~ῠος", "~ῠοιν", "~ῠων";
    dat: "~ῠϊ|~υι", _, "~ῠσῐ|~ῠσῐν";
    acc: "~ῠν", _, "~ῡς|~ῠᾰς";
    ),
    // Κολλοψ: 32 words (αἰθῐοψ, κῠκλωψ, δολοψ, αἰγῐλωψ, ἀγχιλωψ, ὀψ).
    declension!("κολλοψ", 32, "~ψ";
    stem noun_sg: "~ψ- like μορῐον";
    nom: _, "~πε", "~πες";
    gen_: "~πος", "~ποιν", "~πων";
    dat: "~πῐ", _, "~ψῐ|~ψῐν";
    acc: "~πᾰ", _, "~πᾰς";
    ),
    // Ναῡλος: 28 words (ταῡρος, δοῡλος, σαῡρος, πλοῡτος, καρδοῡχος, οὐολοῡσκος).
    declension!("ναῡλος", 28, "~ῡ~ος", [gender: Masculine];
    stem noun_sg: "~ῡ~- like τρᾰγος";
    nom: _, "~υ~ω", "~ῡ~οι";
    gen_: "~υ~ου", "~υ~οιν", "~υ~ων";
    dat: "~υ~ῳ", _, "~υ~οις";
    acc: "~ῡ~ον", _, "~υ~ους";
    ),
    // Ἀξων: 27 words (ἀρχιτεκτων, βιστων, χᾱων, ἰᾱων, λῠκᾱων, βρᾰχῑων). Like Κᾰνων, except in 1 cell: voc sg.
    declension!("ἀξων", 27, "~ων", [gender: Masculine], like Κᾰνων;
    voc: "~ον", _, _;
    ),
    // Ῥωμαῑος: 26 words (ἰουδαῑος, φαρισαῑος, ῥωμαῑος, ἀνδρεῑος, ἑβραῑος, χᾰλδαῑος). Like Σημεῑον, except in 2 cells: nom pl, acc pl.
    declension!("ῥωμαῑος", 26, "~ῑος", [gender: Masculine], like Σημεῑον;
    stem noun_sg: "~ῑ- like τρᾰγος";
    nom: _, _, "~ῑοι";
    acc: _, _, "~ιους";
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
    // Λᾰτρῐς: 20 words (ἀκρῐς, ὀκρῐς, λᾰτρῐς, μηνῐς, πορτῐς, ἀρδῐς).
    declension!("λᾰτρῐς", 20, "~ῐς";
    stem noun_sg: "~ῐ- like δῐαιρεσῐς";
    nom: _, "~ῐε", "~ῐες";
    gen_: "~ῐος", "~ῐοιν", "~ῐων";
    dat: "~ῑ", _, "~ῐσῐ|~ῐσῐν";
    acc: "~ῐν", _, "~ῑς|~ῐᾰς";
    ),
    // Πους: 19 words (πους, ὀκτωπους, πολυπους, τρῐπους, ἀπους, δᾰσῠπους). Like Κερκῐς, except in 0 cells: .
    declension!("πους", 19, "~υς", [gender: Masculine], like Κερκῐς;
    stem noun_sg: "~υς- like μορῐον";
    ),
    // Νηδῡς: 15 words (ἰχθῡς, δελφῡς, ὀφρῡς, λῐγνῡς, ἐδητῡς, ἰσχῡς). Like Στᾰχῠς, except in 2 cells: dat sg, acc sg.
    declension!("νηδῡς", 15, "~ῡς", like Στᾰχῠς;
    stem noun_sg: "~ῡ- like δῐαιρεσῐς";
    dat: "~ῠϊ|~υῑ", _, _;
    acc: "~ῡν", _, _;
    ),
    // Πτερῠξ: 14 words (ἰᾱπῠξ, λᾰτᾰξ, ὀρτῠξ, μᾰστῑξ, τεττῑξ, λᾰλᾰξ).
    declension!("πτερῠξ", 14, "~ξ";
    stem noun_sg: "~ξ- like μορῐον";
    nom: _, "~γε", "~γες";
    gen_: "~γος", "~γοιν", "~γων";
    dat: "~γῐ", _, "~ξῐ|~ξῐν";
    acc: "~γᾰ", _, "~γᾰς";
    ),
    // Πνεῡμᾰ: 14 words (πνεῡμᾰ, καῡμᾰ, ῥεῡμᾰ, θαῡμᾰ, ζεῡγμᾰ, τραῡμᾰ).
    declension!("πνεῡμᾰ", 14, "~ῡ~", [gender: Neuter];
    stem noun_sg: "~ῡ~- like μορῐον";
    nom: _, "~υ~τε", "~υ~τᾰ";
    gen_: "~υ~τος", "~υ~τοιν", "~υ~των";
    dat: "~υ~τῐ", _, "~υ~σῐ|~υ~σῐν";
    acc: "~ῡ~", _, "~υ~τᾰ";
    ),
    // Σηψ: 13 words (ὠψ, γρῡψ, θρῑψ, θωψ, κλωψ, σκωψ). Like Κολλοψ, except in 1 cell: gen_ du.
    declension!("σηψ", 13, "~ψ", like Κολλοψ;
    gen_: _, "~ποῑν", _;
    ),
    // Ἀσαῑοι: 13 words (βοττῐαῑοι, ἀραμαῑοι, ἀρδιαῑοι, ἀσαῑοι, θᾰμᾰναῑοι, νῐψαῑοι). Like Ῥωμαῑος, except in 0 cells: .
    declension!("ἀσαῑοι", 13, "~ῑοι", [gender: Masculine], like Ῥωμαῑος;
    ),
    // Ῥως: 13 words (ἑξηκοντᾰ, ὀσσε, ὑφεν, ῥᾰββῑ, ῥᾰββονῑ, ῥᾰββουνῑ). Like Θεμᾰ, except in 7 cells: nom du, gen_ sg, gen_ du, gen_ pl, dat sg, dat pl, ….
    declension!("ῥως", 13, "~", like Θεμᾰ as PL_AS_SG;
    nom: _, "~", _;
    gen_: "~", "~", "~";
    dat: "~", _, "~";
    voc: _, _, derived;
    ),
    // Ἰκρῐα: 12 words (γενεσια, μουστακια, σαγματοπασμαγαδια, εὐληρα, βρῡτεα, κανθηλια). Like Ὀργῐᾰ, except in 1 cell: nom sg.
    declension!("ἰκρῐα", 12, "~α", [gender: Neuter], like Ὀργῐᾰ;
    nom: "~α", _, _;
    ),
    // Ἁλως: 12 words (νεως, γᾰλως, τᾰως, λᾰγως, ἑως, λεως).
    declension!("ἁλως", 12, "~ως";
    stem noun_sg: "~ως- like μορῐον";
    nom: _, "~ω", "~ῳ";
    gen_: "~ω", "~ῳν", "~ων";
    dat: "~ῳ", _, "~ῳς";
    acc: "~ω|~ων", _, "~ως";
    ),
    // Στοῑχος: 12 words (χοῑρος, ἑταῑρος, στοῑχος, τοῑχος, σχοῑνος, ῥοῑζος).
    declension!("στοῑχος", 12, "~ῑ~ος", [gender: Masculine];
    stem noun_sg: "~ῑ~- like τρᾰγος";
    nom: _, "~ι~ω", "~ῑ~οι";
    gen_: "~ι~ου", "~ι~οιν", "~ι~ων";
    dat: "~ι~ῳ", _, "~ι~οις";
    acc: "~ῑ~ον", _, "~ι~ους";
    ),
    // Κηρ: 12 words (κηρ, ἐᾰρ, θενᾰρ, ὀνᾰρ, νεκτᾰρ, μελᾰν). Like Χᾰρᾰκτηρ, except in 2 cells: nom pl, acc pl.
    declension!("κηρ", 12, "~", [gender: Neuter], like Χᾰρᾰκτηρ as ACC_AS_VOC;
    nom: _, _, "~ᾰ";
    acc: _, _, "~ᾰ";
    ),
    // Περᾰς: 11 words (φως, τερᾰς, πᾰγκρεᾰς, ἐρῠσῐπελᾰς, περᾰς, εἰκος). Like Θεμᾰ, except in 1 cell: acc sg.
    declension!("περᾰς", 11, "~ς", [gender: Neuter], like Θεμᾰ;
    stem noun_sg: "~ς- like μορῐον";
    acc: "~ς", _, _;
    ),
    // Πεπερῐ: 11 words (σῐνᾱπῐ, κομμῐ, πεπερῐ, κιννᾰβᾰρῐ, ἀμῐ, στῐμμῐ). Like Δῐαιρεσῐς, except in 3 cells: nom sg, nom pl, acc pl.
    declension!("πεπερῐ", 11, "~ῐ", [gender: Neuter], like Δῐαιρεσῐς as ACC_AS_VOC;
    nom: "~ῐ", _, "~η|~εᾰ";
    acc: _, _, "~η|~εᾰ";
    ),
    // Τρωες: 11 words (πῑερες, τρωες, κηφηνες, ἁλτηρες, αἰγες, ἀνᾰξῠρῐδες). Like Ὀργῐᾰ, except in 4 cells: nom sg, nom pl, dat pl, acc pl.
    declension!("τρωες", 11, "~ες", like Ὀργῐᾰ;
    nom: "~ες", _, "~ες";
    dat: _, _, "~σῐ|~σῐν";
    acc: _, _, "~ᾰς";
    ),
    // Λευκη: 10 words (πευκη, δουλη, βλαυτη, σενδουκη, λευκη, ζευγλη).
    declension!("λευκη", 10, "~υ~η", [gender: Feminine];
    stem noun_sg: "~υ~η- like μορῐον";
    nom: _, "~υ~ᾱ", "~ῡ~αι";
    gen_: "~υ~ης", "~υ~αιν", "~υ~ων";
    dat: "~υ~ῃ", _, "~υ~αις";
    acc: "~υ~ην", _, "~υ~ᾱς";
    ),
    // Ἱππομᾰνες: 10 words (νευροειδες, μονοειδες, τρῐχομᾰνες, ἡμεροκᾰλλες, σησᾰμοειδες, ὠκῐμοειδες). Like Γενος, except in 4 cells: nom du, gen_ sg, dat sg, acc sg.
    declension!("ἱππομᾰνες", 10, "~ες", [gender: Neuter], like Γενος;
    stem noun_sg: "~ες- like μορῐον";
    nom: _, "~εῑ", _;
    gen_: "~οῡς", _, _;
    dat: "~εῑ", _, _;
    acc: "~ες", _, _;
    ),
    // Πλᾰτεῑᾰ: 10 words (μυῑᾰ, πλᾰτεῑᾰ, γαῑᾰ, μαῑᾰ, βᾰρεῑᾰ, ὀξεῑᾰ).
    declension!("πλᾰτεῑᾰ", 10, "~ῑᾰ";
    stem noun_sg: "~ῑᾰ- like μορῐον";
    nom: _, "~ιᾱ", "~ῑαι";
    gen_: "~ιᾱς", "~ιαιν", "~ιων";
    dat: "~ιᾳ", _, "~ιαις";
    acc: "~ῑᾰν", _, "~ιᾱς";
    ),
    // Φρην: 9 words (φρην, ὑμην, ποιμην, πῠθμην, ἀδην, αὐχην).
    declension!("φρην", 9, "~ην";
    stem noun_sg: "~ην- like μορῐον";
    nom: _, "~ενε", "~ενες";
    gen_: "~ενος", "~ενοιν", "~ενων";
    dat: "~ενῐ", _, "~εσῐ|~εσῐν";
    acc: "~ενᾰ", _, "~ενᾰς";
    ),
    // Κοιτη: 9 words (χαιτη, κοιτη, ἐρεικη, θοινη, βαιτη, μαινη).
    declension!("κοιτη", 9, "~ι~η", [gender: Feminine];
    stem noun_sg: "~ι~η- like μορῐον";
    nom: _, "~ι~ᾱ", "~ῑ~αι";
    gen_: "~ι~ης", "~ι~αιν", "~ι~ων";
    dat: "~ι~ῃ", _, "~ι~αις";
    acc: "~ι~ην", _, "~ι~ᾱς";
    ),
    // Ἱμᾱς: 9 words (κορῠβᾱς, γῐγᾱς, ἐλεφᾱς, ἀδᾰμᾱς, ἀνδρῐᾱς, λῠκᾰβᾱς).
    declension!("ἱμᾱς", 9, "~ᾱς", [gender: Masculine];
    nom: "~ᾱς", "~ᾰντε", "~ᾰντες";
    gen_: "~ᾰντος", "~ᾰντοιν", "~ᾰντων";
    dat: "~ᾰντῐ", _, "~ᾱσῐ|~ᾱσῐν";
    acc: "~ᾰντᾰ", _, "~ᾰντᾰς";
    voc: "~ᾰν", _, _;
    ),
    // Ἀλειτης: 9 words (ὠρειτης, παικτης, σωρειτης, σᾰμᾰρειτης, ἀρσενοκοιτης, μητροκοιτης). Like Κοιτη, except in 3 cells: nom sg, gen_ sg, voc sg.
    declension!("ἀλειτης", 9, "~ι~ης", [gender: Masculine], like Κοιτη;
    nom: "~ι~ης", _, _;
    gen_: "~ι~ου", _, _;
    voc: "~ῑ~ᾰ", _, _;
    ),
    // Νεῡρον: 8 words (προῡμνον, νεῡρον, βοῡστον, τεῡτλον, σοῡσον, λοῡσσον). Like Ναῡλος, except in 2 cells: nom pl, acc pl.
    declension!("νεῡρον", 8, "~ῡ~ον", [gender: Neuter], like Ναῡλος;
    stem noun_sg: "~ῡ~ον- like μορῐον";
    nom: _, _, "~ῡ~ᾰ";
    acc: _, _, "~ῡ~ᾰ";
    ),
    // Θηρ: 8 words (κᾱρ, θηρ, ψᾱρ, φωρ, σηρ, φηρ). Like Χᾰρᾰκτηρ, except in 1 cell: gen_ du.
    declension!("θηρ", 8, "~", like Χᾰρᾰκτηρ;
    gen_: _, "~οῑν", _;
    ),
    // Μωλῠ: 8 words (δᾰκρῠ, μωλῠ, νᾱπῠ, ἀσχῠ, μῐσῠ, βρᾰθῠ). Like Στᾰχῠς, except in 3 cells: nom sg, nom pl, acc pl.
    declension!("μωλῠ", 8, "~ῠ", [gender: Neuter], like Στᾰχῠς as ACC_AS_VOC;
    nom: "~ῠ", _, "~ῠᾰ";
    acc: _, _, "~ῠᾰ";
    ),
    // Κῠᾰρ: 8 words (οὐθᾰρ, ἡπᾰρ, φρεᾱρ, ἀλειφᾰρ, στεᾱρ, δελεᾰρ). Like Θεμᾰ, except in 1 cell: acc sg.
    declension!("κῠᾰρ", 8, "~ρ", [gender: Neuter], like Θεμᾰ;
    stem noun_sg: "~ρ- like μορῐον";
    acc: "~ρ", _, _;
    ),
    // Ἀθῠρᾱς: 8 words (πᾰξᾰμᾱς, μᾰρῐκᾱς, ἀττᾰγᾱς, μᾰσουχᾱς, βᾰσκᾱς, χαλιφᾱς). Like Φορᾱ, except in 2 cells: nom pl, gen_ sg.
    declension!("ἀθῠρᾱς", 8, "~ᾱς", [gender: Masculine], like Φορᾱ;
    stem noun_sg: "~ᾱ- like δῐαιρεσῐς";
    nom: _, _, "~αῑ";
    gen_: "~ᾱ", _, _;
    ),
    // Ῥᾱξ: 7 words (φλοξ, αἰξ, σφᾰξ, ῥᾱξ, στρῐξ, ῥωξ). Like Πτερῠξ, except in 1 cell: gen_ du.
    declension!("ῥᾱξ", 7, "~ξ", like Πτερῠξ;
    gen_: _, "~γοῑν", _;
    ),
    // Ἠχω: 7 words (ἠχω, τυτω, κῐττω, πῑπω, μοτω, φειδω). Like Ἀρῐθμος, except in 4 cells: gen_ sg, dat sg, acc sg, voc sg.
    declension!("ἠχω", 7, "~ω", [gender: Feminine], like Ἀρῐθμος as SG_AS_DU;
    gen_: "~οῡς", _, _;
    dat: "~οῑ", _, _;
    acc: "~ω", _, _;
    voc: "~οῑ", _, _;
    ),
    // Χεῑμᾰ: 7 words (χεῑμᾰ, δεῑγμᾰ, σεῑσμᾰ, λεῑμμᾰ, λαῑτμᾰ, πεῑσμᾰ).
    declension!("χεῑμᾰ", 7, "~ῑ~", [gender: Neuter];
    stem noun_sg: "~ῑ~- like μορῐον";
    nom: _, "~ι~τε", "~ι~τᾰ";
    gen_: "~ι~τος", "~ι~τοιν", "~ι~των";
    dat: "~ι~τῐ", _, "~ι~σῐ|~ι~σῐν";
    acc: "~ῑ~", _, "~ι~τᾰ";
    ),
    // Ναυτης: 7 words (ναυτης, θραυστης, κοκκοθραυστης, ἀργοναυτης, ἐμφῠτευτης, ψευστης). Like Λευκη, except in 3 cells: nom sg, gen_ sg, voc sg.
    declension!("ναυτης", 7, "~υ~ης", [gender: Masculine], like Λευκη;
    nom: "~υ~ης", _, _;
    gen_: "~υ~ου", _, _;
    voc: "~ῡ~ᾰ", _, _;
    ),
    // Γλωχῑς: 7 words (ἀκτῑς, ὠδῑς, γλωχῑς, στᾰμῑς, ἑρμῑς, ὀρκῡς). Like Αἰων, except in 0 cells: .
    declension!("γλωχῑς", 7, "~ς", like Αἰων;
    stem noun_sg: "~ς- like μορῐον";
    ),
    // Πᾰτηρ: 7 words (ἀνηρ, μητηρ, πᾰτηρ, ἀστηρ, θῠγᾰτηρ, ἐνᾰτηρ).
    declension!("πᾰτηρ", 7, "~η~";
    nom: "~η~", "~ε~ε", "~ε~ες";
    gen_: "~~ος", "~ε~οιν", "~ε~ων";
    dat: "~~ῐ", _, "~~ᾰσῐ";
    acc: "~ε~ᾰ", _, "~ε~ᾰς";
    voc: "~ε~", _, _;
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
    stem noun_sg: "~ως- like μορῐον";
    nom: _, "~ωε", "~ωες";
    gen_: "~ωος", "~ωοιν", "~ωων";
    dat: "~ῳ|~ωϊ", _, "~ωσῐ|~ωσῐν";
    acc: "~ω", _, "~ωᾰς";
    ),
    // Γεῑσον: 6 words (δεῑπνον, σπεῑρον, κλεῑθρον, σεῑστρον, ῥεῑθρον, γεῑσον). Like Στοῑχος, except in 2 cells: nom pl, acc pl.
    declension!("γεῑσον", 6, "~ῑ~ον", [gender: Neuter], like Στοῑχος;
    stem noun_sg: "~ῑ~ον- like μορῐον";
    nom: _, _, "~ῑ~ᾰ";
    acc: _, _, "~ῑ~ᾰ";
    ),
    // Γερᾰς: 6 words (γερᾰς, δεπᾰς, ψεφᾰς, σφελᾰς, βουκερᾰς, σκεπᾰς). Like Κηρ, except in 3 cells: dat sg, dat pl, acc sg.
    declension!("γερᾰς", 6, "~ς", [gender: Neuter], like Κηρ;
    stem noun_sg: "~ς- like μορῐον";
    dat: "~ϊ", _, "~σῐ";
    acc: "~ς", _, _;
    ),
    // Θης: 6 words (κρης, χᾰρῐς, θης, δαις, χρως, σης). Like Περᾰς, except in 5 cells: nom pl, gen_ du, dat pl, acc sg, acc pl.
    declension!("θης", 6, "~ς", like Περᾰς;
    nom: _, _, "~τες";
    gen_: _, "~τοῑν", _;
    dat: _, _, "~σῐ";
    acc: "~τᾰ", _, "~τᾰς";
    ),
    // Παῡλᾰ: 6 words (μοῡσᾰ, βοῡλλᾰ, πᾰνδοῡρᾰ, παῡλᾰ, γοῡρνᾰ, κοῡπᾰ). Like Λευκη, except in 1 cell: acc sg.
    declension!("παῡλᾰ", 6, "~ῡ~ᾰ", [gender: Feminine], like Λευκη;
    stem noun_sg: "~ῡ~ᾰ- like μορῐον";
    acc: "~ῡ~ᾰν", _, _;
    ),
    // Πᾰπᾱς: 6 words (πᾰπᾱς, γᾰλᾰτᾱς, κᾰδῡτᾱς, γενετᾱς, σᾰκτᾱς, πᾰππᾱς). Like Κοχλῐᾱς, except in 1 cell: voc sg.
    declension!("πᾰπᾱς", 6, "~ᾱς", [gender: Masculine], like Κοχλῐᾱς;
    voc: "~ᾰ", _, _;
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
    stem noun_sg: "~ς- like μορῐον";
    nom: _, "~θε", "~θες";
    gen_: "~θος", "~θοιν", "~θων";
    dat: "~θῐ", _, "~σῐ|~σῐν";
    acc: "~θᾰ", _, "~θᾰς";
    ),
    // Γεῡσῐς: 5 words (παῡσῐς, λοῡσῐς, ῥεῡσῐς, ζεῡξῐς, γεῡσῐς).
    declension!("γεῡσῐς", 5, "~ῡ~ῐς", [gender: Feminine];
    stem noun_sg: "~ῡ~ῐ- like δῐαιρεσῐς";
    nom: _, "~υ~ει", "~υ~εις";
    gen_: "~υ~εως", "~υ~εοιν", "~υ~εων";
    dat: "~υ~ει", _, "~υ~εσῐ|~υ~εσῐν";
    acc: "~ῡ~ῐν", _, "~υ~εις";
    ),
    // Νοῡς: 5 words (νοῡς, πλοῡς, θῠγατρῐδοῡς, ῥοῡς, λουτροχοῡς). Like Ἀρῐθμος, except in 4 cells: nom du, nom pl, acc sg, acc pl.
    declension!("νοῡς", 5, "~οῡς", [gender: Masculine], like Ἀρῐθμος;
    stem noun_sg: "~οῡ- like δῐαιρεσῐς";
    nom: _, "~ω|~ω", "~οῑ";
    acc: "~οῡν", _, "~οῡς";
    ),
    // Τεῡχος: 5 words (τεῡχος, γλεῡκος, ψεῡδος, ζεῡγος, κεῡθος).
    declension!("τεῡχος", 5, "~ῡ~ος", [gender: Neuter];
    stem noun_sg: "~ῡ~ος- like μορῐον";
    nom: _, "~υ~ει", "~υ~η";
    gen_: "~υ~ους", "~υ~οῑν", "~υ~ων";
    dat: "~υ~ει", _, "~υ~εσῐ|~υ~εσῐν";
    acc: "~ῡ~ος", _, "~υ~η";
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
    // Ἀηρ: 5 words (γᾰστηρ, ἀηρ, φρᾱτηρ, αἰθηρ, ἀθηρ). Like Πᾰτηρ, except in 4 cells: gen_ sg, dat sg, dat pl, voc sg.
    declension!("ἀηρ", 5, "~η~", like Πᾰτηρ;
    gen_: "~ε~ος", _, _;
    dat: "~ε~ῐ", _, "~ε~σῐ|~ε~σῐν";
    voc: "~η~", _, _;
    ),
    // Λεῑμᾰξ: 5 words (χοῑνῐξ, κᾰταῑτῠξ, μεῑρᾰξ, λεῑμᾰξ, βαῑβῠξ).
    declension!("λεῑμᾰξ", 5, "~ῑ~ξ";
    stem noun_sg: "~ῑ~ξ- like μορῐον";
    nom: _, "~ι~κε", "~ι~κες";
    gen_: "~ι~κος", "~ι~κοιν", "~ι~κων";
    dat: "~ι~κῐ", _, "~ι~ξῐ|~ι~ξῐν";
    acc: "~ι~κᾰ", _, "~ι~κᾰς";
    ),
    // Κρεᾰς: 5 words (κρεᾰς, σελᾰς, κνεφᾰς, γηρᾰς, κᾰλλῐκρεᾰς).
    declension!("κρεᾰς", 5, "~ᾰς", [gender: Neuter];
    stem noun_sg: "~ᾰς- like μορῐον";
    nom: _, "~ᾱ", "~ᾱ";
    gen_: "~ως", "~οῑν", "~ων";
    dat: "~ᾳ", _, "~ᾰσῐ";
    acc: "~ᾰς", _, "~ᾱ";
    ),
    // Λῐψ: 5 words (χᾰλῠψ, φλεψ, χερνῐψ, φᾰψ, λῐψ).
    declension!("λῐψ", 5, "~ψ";
    stem noun_sg: "~ψ- like μορῐον";
    nom: _, "~βε", "~βες";
    gen_: "~βος", "~βοῑν", "~βων";
    dat: "~βῐ", _, "~ψῐ|~ψῐν";
    acc: "~βᾰ", _, "~βᾰς";
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
    // Λαυρᾱ: 4 words (σαυρᾱ, λαυρᾱ, κᾰρουχᾱ, βρουτζᾱ). Like Λευκη, except in 4 cells: gen_ sg, dat sg, acc sg, voc sg.
    declension!("λαυρᾱ", 4, "~υ~ᾱ", [gender: Feminine], like Λευκη as SG_AS_DU;
    gen_: "~υ~ᾱς", _, _;
    dat: "~υ~ᾳ", _, _;
    acc: "~υ~ᾱν", _, _;
    voc: "~υ~ᾱ", _, _;
    ),
    // Ὀνῠξ: 4 words (ὀνῠξ, σᾰρδονῠξ, ὑστρῐξ, ἀδδῐξ).
    declension!("ὀνῠξ", 4, "~ξ";
    stem noun_sg: "~ξ- like μορῐον";
    nom: _, "~χε", "~χες";
    gen_: "~χος", "~χοιν", "~χων";
    dat: "~χῐ", _, "~ξῐ|~ξῐν";
    acc: "~χᾰ", _, "~χᾰς";
    ),
    // Ἁλς: 4 words (ἁλς, ὑς, ὀϊς, κῑς). Like Χᾰρᾰκτηρ, except in 3 cells: dat pl, acc sg, acc pl.
    declension!("ἁλς", 4, "~ς", [gender: Masculine], like Χᾰρᾰκτηρ as DAT_AS_VOC;
    stem noun_sg: "~- like δῐαιρεσῐς";
    dat: _, _, "~σῐ";
    acc: "~ν", _, "~ς";
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
    // Πηχῠς: 4 words (πελεκῠς, πρεσβῠς, πηχῠς, ἀμφῐβρᾰχῠς). Like Δῐαιρεσῐς, except in 1 cell: acc sg.
    declension!("πηχῠς", 4, "~ῠς", [gender: Masculine], like Δῐαιρεσῐς;
    stem noun_sg: "~ῠ- like δῐαιρεσῐς";
    acc: "~ῠν", _, _;
    ),
    // Ζωᾱ: 4 words (γονᾱ, δρᾰχμᾱ, σφᾰγᾱ, ζωᾱ). Like Φορᾱ, except in 3 cells: gen_ pl, dat pl, acc pl.
    declension!("ζωᾱ", 4, "~ᾱ", [gender: Feminine], like Φορᾱ;
    gen_: _, _, "~ᾱων|~ᾱν";
    dat: _, _, "~αῑς|~αῑσῐ|~αῑσῐν";
    acc: _, _, "~ᾰς";
    ),
    // Κηρῠξ: 4 words (κοκκῠξ, κηρῠξ, ψευδοκηρῠξ, ἱεροκηρῠξ).
    declension!("κηρῠξ", 4, "~ῠξ", [gender: Masculine];
    stem noun_sg: "~ῠξ- like μορῐον";
    nom: _, "~ῡκε", "~ῡκες";
    gen_: "~ῡκος", "~ῡκοιν", "~ῡκων";
    dat: "~ῡκῐ", _, "~ῡξῐ|~ῡξῐν";
    acc: "~ῡκᾰ", _, "~ῡκᾰς";
    ),
    // Δᾰσῠ: 4 words (ἀστῠ, ὀξῠ, δᾰσῠ, πλᾰτῠ). Like Φᾰος, except in 3 cells: nom du, dat sg, acc sg.
    declension!("δᾰσῠ", 4, "~ῠ", like Φᾰος;
    stem noun_sg: "~ῠ- like μορῐον";
    nom: _, "~εῑ", _;
    dat: "~εῑ", _, _;
    acc: "~ῠ", _, _;
    ),
    // Γαλη: 3 words (γη, μῡγᾰλη, γαλη). Like Κᾰτᾰσκευη, except in 1 cell: nom pl.
    declension!("γαλη", 3, "~η", [gender: Feminine], like Κᾰτᾰσκευη;
    nom: _, _, "~αῑ";
    ),
    // Κρῐτης: 3 words (κρῐτης, ἀμερμουμνης, γλωττοειδης). Like Χορευτης, except in 1 cell: voc sg.
    declension!("κρῐτης", 3, "~ης", [gender: Masculine], like Χορευτης;
    voc: "~η", _, _;
    ),
    // Υἱῠς: 3 words (ἡμῐσῠς, υἱῠς, τρῐβρᾰχῠς). Like Πηχῠς, except in 1 cell: gen_ sg.
    declension!("υἱῠς", 3, "~ῠς", [gender: Masculine], like Πηχῠς;
    gen_: "~εος", _, _;
    ),
    // Χοῡς: 3 words (βοῡς, γρᾱῡς, χοῡς). Like Θηρ, except in 4 cells: dat sg, dat pl, acc sg, acc pl.
    declension!("χοῡς", 3, "~ῡς", like Θηρ;
    stem noun_sg: "~ῡ- like δῐαιρεσῐς";
    dat: "~ϊ", _, "~υσῐ|~υσῐν";
    acc: "~ῡν", _, "~ῡς";
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
    stem noun_sg: "~ῑ~ος- like μορῐον";
    nom: _, "~ι~ει", "~ι~η";
    gen_: "~ι~ους", "~ι~οῑν", "~ι~ων";
    dat: "~ι~ει", _, "~ι~εσῐ|~ι~εσῐν";
    acc: "~ῑ~ος", _, "~ι~η";
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
    // Ὀδους: 3 words (ὀδους, κῠνοδους, τρῐοδους). Like Φῡσᾱς, except in 2 cells: nom sg, dat pl.
    declension!("ὀδους", 3, "~υς", [gender: Masculine], like Φῡσᾱς;
    nom: "~υς", _, _;
    dat: _, _, "~υσῐ|~υσῐν";
    ),
    // Λαλλαι: 3 words (κᾰλανδαι, χαρωνῑται, λαλλαι).
    declension!("λαλλαι", 3, "~α~αι";
    nom: "~α~αι", _, "~ᾰ~αι";
    gen_: _, _, "~ᾰ~ων";
    dat: _, _, "~ᾰ~αις";
    acc: _, _, "~ᾰ~ᾱς";
    ),
    // Χειρ: 3 words (χειρ, φθειρ, ῥηγμιν).
    declension!("χειρ", 3, "~ι~";
    stem noun_sg: "~ι~- like μορῐον";
    nom: _, "~ῑ~ε", "~ῑ~ες";
    gen_: "~ι~ος", "~ι~οῑν", "~ι~ων";
    dat: "~ι~ῐ", _, "~ι~σῐ";
    acc: "~ῑ~ᾰ", _, "~ῑ~ᾰς";
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
    // Ἐκπλους: 3 words (πᾰραπλους, ἐκπλους, ἀγχῐνους). Like Τρᾰγος, except in 1 cell: acc sg.
    declension!("ἐκπλους", 3, "~ους", [gender: Masculine], like Τρᾰγος;
    stem noun_sg: "~ου- like δῐαιρεσῐς";
    acc: "~ουν", _, _;
    ),
    // Θεωσῐς: 3 words (θεωσῐς, ἀλθησῐς, ζαχαρῐς). Like Δῐαιρεσῐς, except in 6 cells: nom du, nom pl, gen_ du, dat sg, dat pl, acc pl.
    declension!("θεωσῐς", 3, "~ῐς", [gender: Feminine], like Δῐαιρεσῐς;
    nom: _, derived, "~εες";
    gen_: _, derived, _;
    dat: "~εϊ", _, "~εσῐ";
    acc: _, _, "~εᾰς";
    ),
    // Δυσμαι: 2 words (προλυται, δυσμαι). Like Λαλλαι, except in 5 cells: nom sg, nom pl, gen_ pl, dat pl, acc pl.
    declension!("δυσμαι", 2, "~υ~αι", like Λαλλαι;
    nom: "~υ~αι", _, "~ῠ~αι";
    gen_: _, _, "~ῠ~ων";
    dat: _, _, "~ῠ~αῑς";
    acc: _, _, "~ῠ~ᾱς";
    ),
    // Γλωσσᾰ: 1 words (γλωσσᾰ).
    declension!("γλωσσᾰ", 1, "~σσᾰ";
    stem noun_sg: "~ττᾰ- like μορῐον";
    nom: _, "~ττᾱ", "~τται";
    gen_: "~ττης", "~τταιν", "~ττων";
    dat: "~ττῃ", _, "~τταις";
    acc: "~ττᾰν", _, "~ττᾱς";
    ),
    // Ἱερος: 1 words (ἱερος).
    declension!("ἱερος", 1, "ἱ~ος";
    nom: "ῐ̔~ος", "ῐ̔~ω", "ῐ̔~οι";
    gen_: "ῐ̔~οῡ", "ῐ̔~οῑν", "ῐ̔~ων";
    dat: "ῐ̔~ῳ", _, "ῐ̔~οῑς";
    acc: "ῐ̔~ον", _, "ῐ̔~ους";
    voc: "ῐ̔~ε", _, _;
    ),
    // Ἱερον: 1 words (ἱερον). Like Ἱερος, except in 2 cells: nom pl, acc pl.
    declension!("ἱερον", 1, "ἱ~ον", like Ἱερος;
    stem noun_sg: "~ον- like ἱερον";
    nom: _, _, "ῐ̔~ᾰ";
    acc: _, _, "ῐ̔~ᾰ";
    ),
    // Ἱερα: 1 words (ἱερα).
    declension!("ἱερα", 1, "ἱ~α";
    stem noun_sg: "~ᾱ- like ἱερον";
    nom: _, "ῐ̔~ᾱ", "ῐ̔~αι";
    gen_: "ῐ̔~ᾱς", "ῐ̔~αῑν", "ῐ̔~ων";
    dat: "ῐ̔~ᾳ", _, "ῐ̔~αῑς";
    acc: "ῐ̔~ᾱν", _, "ῐ̔~ᾱς";
    ),
    // Μηριᾰ: 1 words (μηριᾰ). Like Ὀργῐᾰ, except in 5 cells: nom sg, nom pl, gen_ pl, dat pl, acc pl.
    declension!("μηριᾰ", 1, "~ιᾰ", like Ὀργῐᾰ;
    nom: "~ιᾰ", _, "~ῐᾰ";
    gen_: _, _, "~ῐων";
    dat: _, _, "~ῐοις";
    acc: _, _, "~ῐᾰ";
    ),
    // Ἰᾱλεμος: 1 words (ἰᾱλεμος).
    declension!("ἰᾱλεμος", 1, "~ος";
    stem noun_sg: "~μ- like τρᾰγος";
    nom: _, "~μω", "~μοι";
    gen_: "~μου", "~μοιν", "~μων";
    dat: "~μῳ", _, "~μοις";
    acc: "~μον", _, "~μους";
    ),
    // Κῠνοχᾰλκη: 1 words (κῠνοχᾰλκη).
    declension!("κῠνοχᾰλκη", 1, "~~η";
    stem noun_sg: "~κ~η- like μορῐον";
    nom: _, "~κ~ᾱ", "~κ~αι";
    gen_: "~κ~ης", "~κ~αιν", "~κ~ων";
    dat: "~κ~ῃ", _, "~κ~αις";
    acc: "~κ~ην", _, "~κ~ᾱς";
    ),
    // Λιμεναρχης: 1 words (λιμεναρχης).
    declension!("λιμεναρχης", 1, "~ι~ε~α~ης";
    stem noun_sg: "~ῐ~η~ᾰ~η- like δῐαιρεσῐς";
    nom: _, "~ῐ~η~ᾰ~ᾱ", "~ῐ~η~ᾰ~αι";
    gen_: "~ῐ~η~ᾰ~ου", "~ῐ~η~ᾰ~αιν", "~ῐ~η~ᾰ~ων";
    dat: "~ῐ~η~ᾰ~ῃ", _, "~ῐ~η~ᾰ~αις";
    acc: "~ῐ~η~ᾰ~ην", _, "~ῐ~η~ᾰ~ᾱς";
    ),
    // Βυσναῑοι: 1 words (βυσναῑοι).
    declension!("βυσναῑοι", 1, "~υ~ῑο~";
    nom: "~υ~ῑο~", _, "~ῠ~ῑο~";
    gen_: _, _, "~ῠ~~ων";
    dat: _, _, "~ῠ~~οις";
    acc: _, _, "~ῠ~~ους";
    ),
    // Ἐπισφυρῐᾰ: 1 words (ἐπισφυρῐᾰ). Like Δυσμαι, except in 4 cells: nom sg, nom pl, dat pl, acc pl.
    declension!("ἐπισφυρῐᾰ", 1, "~υ~ᾰ", like Δυσμαι;
    nom: "~υ~ᾰ", _, "~ῠ~ᾰ";
    dat: _, _, "~ῠ~οις";
    acc: _, _, "~ῠ~ᾰ";
    ),
    // Πᾰραφερνᾰ: 1 words (πᾰραφερνᾰ). Like Λαλλαι, except in 4 cells: nom sg, nom pl, dat pl, acc pl.
    declension!("πᾰραφερνᾰ", 1, "~α~ᾰ", like Λαλλαι;
    nom: "~α~ᾰ", _, "~ᾰ~ᾰ";
    dat: _, _, "~ᾰ~οις";
    acc: _, _, "~ᾰ~ᾰ";
    ),
    // Βακχῐαδαι: 1 words (βακχῐαδαι). Like Βυσναῑοι, except in 5 cells: nom sg, nom pl, gen_ pl, dat pl, acc pl.
    declension!("βακχῐαδαι", 1, "~α~α~αι", like Βυσναῑοι;
    nom: "~α~α~αι", _, "~ᾰ~ᾰ~αι";
    gen_: _, _, "~ᾰ~ᾰ~ων";
    dat: _, _, "~ᾰ~ᾰ~αις";
    acc: _, _, "~ᾰ~ᾰ~ᾱς";
    ),
    // Κοσμητης: 1 words (κοσμητης). Like Κεραιᾱ, except in 4 cells: nom pl, gen_ sg, dat sg, acc sg.
    declension!("κοσμητης", 1, "~σ~ης", like Κεραιᾱ;
    stem noun_sg: "~~- like ἰδῐωτης";
    nom: _, _, "~~αι";
    gen_: "~~ου", _, _;
    dat: "~~ῃ", _, _;
    acc: "~~ην", _, _;
    ),
    // Χᾰριστῐᾰ: 1 words (χᾰριστῐᾰ). Like Λαλλαι, except in 5 cells: nom sg, nom pl, gen_ pl, dat pl, acc pl.
    declension!("χᾰριστῐᾰ", 1, "~ι~ᾰ", like Λαλλαι;
    nom: "~ι~ᾰ", _, "~ῐ~ᾰ";
    gen_: _, _, "~ῐ~ων";
    dat: _, _, "~ῐ~οις";
    acc: _, _, "~ῐ~ᾰ";
    ),
    // Ὀρθογραφος: 1 words (ὀρθογραφος).
    declension!("ὀρθογραφος", 1, "ὀρθ~α~ος";
    nom: "λογ~ᾰ~ος", "λογ~ᾰ~ω", "λογ~ᾰ~οι";
    gen_: "λογ~ᾰ~ου", "λογ~ᾰ~οιν", "λογ~ᾰ~ων";
    dat: "λογ~ᾰ~ῳ", _, "λογ~ᾰ~οις";
    acc: "λογ~ᾰ~ον", _, "λογ~ᾰ~ους";
    voc: "λογ~ᾰ~ε", _, _;
    ),
    // Ἑλληνοτᾰμιαι: 1 words (ἑλληνοτᾰμιαι). Like Μηριᾰ, except in 4 cells: nom sg, nom pl, dat pl, acc pl.
    declension!("ἑλληνοτᾰμιαι", 1, "~ιαι", like Μηριᾰ;
    nom: "~ιαι", _, "~ῐαι";
    dat: _, _, "~ῐαις";
    acc: _, _, "~ῐᾱς";
    ),
];
