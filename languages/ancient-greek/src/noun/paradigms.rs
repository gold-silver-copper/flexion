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
    /// Like `τρᾰγος`: 1,193 words.
    Τρᾰγος,
    /// Like `μορῐον`: 1,025 words.
    Μορῐον,
    /// Like `οὐσῐᾱ`: 546 words.
    Οὐσῐᾱ,
    /// Like `θεμᾰ`: 531 words.
    Θεμᾰ,
    /// Like `νῠμφη`: 518 words.
    Νῠμφη,
    /// Like `μερῐσμος`: 483 words.
    Μερῐσμος,
    /// Like `δῐαιρεσῐς`: 433 words.
    Δῐαιρεσῐς,
    /// Like `κᾰτᾰσκευη`: 302 words.
    Κᾰτᾰσκευη,
    /// Like `γενετης`: 204 words.
    Γενετης,
    /// Like `κερκῐς`: 185 words.
    Κερκῐς,
    /// Like `κορᾰξ`: 183 words.
    Κορᾰξ,
    /// Like `κλεις`: 165 words.
    Κλεις,
    /// Like `τρᾰπεζᾰ`: 133 words.
    Τρᾰπεζᾰ,
    /// Like `ἐπῐφᾰνειᾰ`: 116 words.
    Ἐπῐφᾰνειᾰ,
    /// Like `γενος`: 109 words.
    Γενος,
    /// Like `αἰων`: 96 words.
    Αἰων,
    /// Like `χορευτης`: 87 words.
    Χορευτης,
    /// Like `ἱππευς`: 81 words.
    Ἱππευς,
    /// Like `χᾰρᾰκτηρ`: 72 words.
    Χᾰρᾰκτηρ,
    /// Like `κεραιᾱ`: 71 words.
    Κεραιᾱ,
    /// Like `σημεῑον`: 62 words.
    Σημεῑον,
    /// Like `κᾰλπῐς`: 62 words.
    Κᾰλπῐς,
    /// Like `φορᾱ`: 54 words.
    Φορᾱ,
    /// Like `δᾰμᾰλης`: 53 words.
    Δᾰμᾰλης,
    /// Like `κοχλῐᾱς`: 49 words.
    Κοχλῐᾱς,
    /// Like `ὀργῐᾰ`: 49 words.
    Ὀργῐᾰ,
    /// Like `ζῠγον`: 45 words.
    Ζῠγον,
    /// Like `φᾰος`: 40 words.
    Φᾰος,
    /// Like `ὀλῐνοι`: 40 words.
    Ὀλῐνοι,
    /// Like `κροσσαι`: 39 words.
    Κροσσαι,
    /// Like `κᾰνων`: 34 words.
    Κᾰνων,
    /// Like `ῥητωρ`: 33 words.
    Ῥητωρ,
    /// Like `κολλοψ`: 32 words.
    Κολλοψ,
    /// Like `ἀμβροσια`: 30 words.
    Ἀμβροσια,
    /// Like `στᾰχῠς`: 30 words.
    Στᾰχῠς,
    /// Like `ναῡλος`: 28 words.
    Ναῡλος,
    /// Like `ῥωμαῑος`: 26 words.
    Ῥωμαῑος,
    /// Like `ἀξων`: 25 words.
    Ἀξων,
    /// Like `δᾰνᾰοι`: 24 words.
    Δᾰνᾰοι,
    /// Like `λεων`: 22 words.
    Λεων,
    /// Like `σᾰρξ`: 21 words.
    Σᾰρξ,
    /// Like `ὑποθεσις`: 19 words.
    Ὑποθεσις,
    /// Like `πους`: 18 words.
    Πους,
    /// Like `λᾰτρῐς`: 16 words.
    Λᾰτρῐς,
    /// Like `ἀστραγᾰλος`: 15 words.
    Ἀστραγᾰλος,
    /// Like `περᾰς`: 15 words.
    Περᾰς,
    /// Like `πνεῡμᾰ`: 14 words.
    Πνεῡμᾰ,
    /// Like `ἰκρῐα`: 14 words.
    Ἰκρῐα,
    /// Like `νηδῡς`: 14 words.
    Νηδῡς,
    /// Like `σηψ`: 13 words.
    Σηψ,
    /// Like `ἀποστασις`: 13 words.
    Ἀποστασις,
    /// Like `ἀσαῑοι`: 13 words.
    Ἀσαῑοι,
    /// Like `ἰδεα`: 12 words.
    Ἰδεα,
    /// Like `ἁλως`: 12 words.
    Ἁλως,
    /// Like `κηρ`: 12 words.
    Κηρ,
    /// Like `ῥως`: 12 words.
    Ῥως,
    /// Like `στοῑχος`: 12 words.
    Στοῑχος,
    /// Like `ἀλωπεκιᾱ`: 12 words.
    Ἀλωπεκιᾱ,
    /// Like `πτερῠξ`: 11 words.
    Πτερῠξ,
    /// Like `ἀραχνη`: 11 words.
    Ἀραχνη,
    /// Like `τρωες`: 11 words.
    Τρωες,
    /// Like `ὑπομνημα`: 11 words.
    Ὑπομνημα,
    /// Like `ἀντονομασια`: 10 words.
    Ἀντονομασια,
    /// Like `ἰδιον`: 10 words.
    Ἰδιον,
    /// Like `ἱεραρχης`: 10 words.
    Ἱεραρχης,
    /// Like `λευκη`: 10 words.
    Λευκη,
    /// Like `ἀκανθα`: 10 words.
    Ἀκανθα,
    /// Like `πεπερῐ`: 9 words.
    Πεπερῐ,
    /// Like `φρην`: 9 words.
    Φρην,
    /// Like `κοιτη`: 9 words.
    Κοιτη,
    /// Like `θηρ`: 9 words.
    Θηρ,
    /// Like `ἀλειτης`: 9 words.
    Ἀλειτης,
    /// Like `κωμῡς`: 9 words.
    Κωμῡς,
    /// Like `πλᾰτεῑᾰ`: 9 words.
    Πλᾰτεῑᾰ,
    /// Like `ἀσπις`: 9 words.
    Ἀσπις,
    /// Like `σησᾰμοειδες`: 9 words.
    Σησᾰμοειδες,
    /// Like `γερᾰς`: 9 words.
    Γερᾰς,
    /// Like `νεῡρον`: 8 words.
    Νεῡρον,
    /// Like `ἀμεθυστος`: 8 words.
    Ἀμεθυστος,
    /// Like `ἀναθεμα`: 8 words.
    Ἀναθεμα,
    /// Like `ῥᾱξ`: 7 words.
    Ῥᾱξ,
    /// Like `ἀληθεια`: 7 words.
    Ἀληθεια,
    /// Like `ἰδιωτης`: 7 words.
    Ἰδιωτης,
    /// Like `ἀδᾰμᾱς`: 7 words.
    Ἀδᾰμᾱς,
    /// Like `ἠχω`: 7 words.
    Ἠχω,
    /// Like `μωλῠ`: 7 words.
    Μωλῠ,
    /// Like `χεῑμᾰ`: 7 words.
    Χεῑμᾰ,
    /// Like `ναυτης`: 7 words.
    Ναυτης,
    /// Like `ἀναγωγη`: 7 words.
    Ἀναγωγη,
    /// Like `βᾰσκᾱς`: 7 words.
    Βᾰσκᾱς,
    /// Like `πᾰτηρ`: 6 words.
    Πᾰτηρ,
    /// Like `ἰλυς`: 6 words.
    Ἰλυς,
    /// Like `μοῑρᾰ`: 6 words.
    Μοῑρᾰ,
    /// Like `σφην`: 6 words.
    Σφην,
    /// Like `μητρως`: 6 words.
    Μητρως,
    /// Like `κῠᾰρ`: 6 words.
    Κῠᾰρ,
    /// Like `ἀνισον`: 6 words.
    Ἀνισον,
    /// Like `ἀριθμος`: 6 words.
    Ἀριθμος,
    /// Like `πτῠξ`: 6 words.
    Πτῠξ,
    /// Like `πηχῠς`: 6 words.
    Πηχῠς,
    /// Like `παῡλᾰ`: 6 words.
    Παῡλᾰ,
    /// Like `γεῑσον`: 6 words.
    Γεῑσον,
    /// Like `γλωχῑς`: 6 words.
    Γλωχῑς,
    /// Like `πᾰπᾱς`: 6 words.
    Πᾰπᾱς,
    /// Like `ἀναξ`: 5 words.
    Ἀναξ,
    /// Like `ἀηρ`: 5 words.
    Ἀηρ,
    /// Like `ἰδιος`: 5 words.
    Ἰδιος,
    /// Like `ἀστερισκος`: 5 words.
    Ἀστερισκος,
    /// Like `ἰσχας`: 5 words.
    Ἰσχας,
    /// Like `ἀνατασις`: 5 words.
    Ἀνατασις,
    /// Like `ἀντιθεσις`: 5 words.
    Ἀντιθεσις,
    /// Like `δαιμων`: 5 words.
    Δαιμων,
    /// Like `γεῡσῐς`: 5 words.
    Γεῡσῐς,
    /// Like `νοῡς`: 5 words.
    Νοῡς,
    /// Like `λῐψ`: 5 words.
    Λῐψ,
    /// Like `ἀγκυλη`: 5 words.
    Ἀγκυλη,
    /// Like `τεῡχος`: 5 words.
    Τεῡχος,
    /// Like `χλουνης`: 5 words.
    Χλουνης,
    /// Like `λεῑμᾰξ`: 5 words.
    Λεῑμᾰξ,
    /// Like `φῡσᾱς`: 5 words.
    Φῡσᾱς,
    /// Like `ἱσπανος`: 4 words.
    Ἱσπανος,
    /// Like `ἁγνεια`: 4 words.
    Ἁγνεια,
    /// Like `πῠρᾰ`: 4 words.
    Πῠρᾰ,
    /// Like `λαυρᾱ`: 4 words.
    Λαυρᾱ,
    /// Like `ἀελλα`: 4 words.
    Ἀελλα,
    /// Like `πλευμων`: 4 words.
    Πλευμων,
    /// Like `ἀκανθις`: 4 words.
    Ἀκανθις,
    /// Like `ἀκροβυστια`: 4 words.
    Ἀκροβυστια,
    /// Like `ἱματιον`: 4 words.
    Ἱματιον,
    /// Like `ἀστυρον`: 4 words.
    Ἀστυρον,
    /// Like `σικανος`: 4 words.
    Σικανος,
    /// Like `κηρῠξ`: 4 words.
    Κηρῠξ,
    /// Like `ζωᾱ`: 4 words.
    Ζωᾱ,
    /// Like `γαλη`: 3 words.
    Γαλη,
    /// Like `ἀνακυκλωσις`: 3 words.
    Ἀνακυκλωσις,
    /// Like `ἀμανιτης`: 3 words.
    Ἀμανιτης,
    /// Like `ἀθανασια`: 3 words.
    Ἀθανασια,
    /// Like `θῑς`: 3 words.
    Θῑς,
    /// Like `μῡς`: 3 words.
    Μῡς,
    /// Like `ἀγορα`: 3 words.
    Ἀγορα,
    /// Like `νεῑκος`: 3 words.
    Νεῑκος,
    /// Like `ἀσις`: 3 words.
    Ἀσις,
    /// Like `πειθω`: 3 words.
    Πειθω,
    /// Like `πεῑνᾰ`: 3 words.
    Πεῑνᾰ,
    /// Like `ἀβατον`: 3 words.
    Ἀβατον,
    /// Like `ἐγκᾰτᾰ`: 3 words.
    Ἐγκᾰτᾰ,
    /// Like `ἰασπις`: 3 words.
    Ἰασπις,
    /// Like `ὑπαλλαγη`: 3 words.
    Ὑπαλλαγη,
    /// Like `λυρτος`: 3 words.
    Λυρτος,
    /// Like `δᾰσῠ`: 3 words.
    Δᾰσῠ,
    /// Like `λαλλαι`: 3 words.
    Λαλλαι,
    /// Like `βαλσαμον`: 3 words.
    Βαλσαμον,
    /// Like `ἐντεᾰ`: 3 words.
    Ἐντεᾰ,
    /// Like `παστα`: 3 words.
    Παστα,
    /// Like `γᾰμφηλαι`: 3 words.
    Γᾰμφηλαι,
    /// Like `θεωσῐς`: 3 words.
    Θεωσῐς,
    /// Like `ἀσφαλισις`: 3 words.
    Ἀσφαλισις,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 157] = [
        Declension::Τρᾰγος,
        Declension::Μορῐον,
        Declension::Οὐσῐᾱ,
        Declension::Θεμᾰ,
        Declension::Νῠμφη,
        Declension::Μερῐσμος,
        Declension::Δῐαιρεσῐς,
        Declension::Κᾰτᾰσκευη,
        Declension::Γενετης,
        Declension::Κερκῐς,
        Declension::Κορᾰξ,
        Declension::Κλεις,
        Declension::Τρᾰπεζᾰ,
        Declension::Ἐπῐφᾰνειᾰ,
        Declension::Γενος,
        Declension::Αἰων,
        Declension::Χορευτης,
        Declension::Ἱππευς,
        Declension::Χᾰρᾰκτηρ,
        Declension::Κεραιᾱ,
        Declension::Σημεῑον,
        Declension::Κᾰλπῐς,
        Declension::Φορᾱ,
        Declension::Δᾰμᾰλης,
        Declension::Κοχλῐᾱς,
        Declension::Ὀργῐᾰ,
        Declension::Ζῠγον,
        Declension::Φᾰος,
        Declension::Ὀλῐνοι,
        Declension::Κροσσαι,
        Declension::Κᾰνων,
        Declension::Ῥητωρ,
        Declension::Κολλοψ,
        Declension::Ἀμβροσια,
        Declension::Στᾰχῠς,
        Declension::Ναῡλος,
        Declension::Ῥωμαῑος,
        Declension::Ἀξων,
        Declension::Δᾰνᾰοι,
        Declension::Λεων,
        Declension::Σᾰρξ,
        Declension::Ὑποθεσις,
        Declension::Πους,
        Declension::Λᾰτρῐς,
        Declension::Ἀστραγᾰλος,
        Declension::Περᾰς,
        Declension::Πνεῡμᾰ,
        Declension::Ἰκρῐα,
        Declension::Νηδῡς,
        Declension::Σηψ,
        Declension::Ἀποστασις,
        Declension::Ἀσαῑοι,
        Declension::Ἰδεα,
        Declension::Ἁλως,
        Declension::Κηρ,
        Declension::Ῥως,
        Declension::Στοῑχος,
        Declension::Ἀλωπεκιᾱ,
        Declension::Πτερῠξ,
        Declension::Ἀραχνη,
        Declension::Τρωες,
        Declension::Ὑπομνημα,
        Declension::Ἀντονομασια,
        Declension::Ἰδιον,
        Declension::Ἱεραρχης,
        Declension::Λευκη,
        Declension::Ἀκανθα,
        Declension::Πεπερῐ,
        Declension::Φρην,
        Declension::Κοιτη,
        Declension::Θηρ,
        Declension::Ἀλειτης,
        Declension::Κωμῡς,
        Declension::Πλᾰτεῑᾰ,
        Declension::Ἀσπις,
        Declension::Σησᾰμοειδες,
        Declension::Γερᾰς,
        Declension::Νεῡρον,
        Declension::Ἀμεθυστος,
        Declension::Ἀναθεμα,
        Declension::Ῥᾱξ,
        Declension::Ἀληθεια,
        Declension::Ἰδιωτης,
        Declension::Ἀδᾰμᾱς,
        Declension::Ἠχω,
        Declension::Μωλῠ,
        Declension::Χεῑμᾰ,
        Declension::Ναυτης,
        Declension::Ἀναγωγη,
        Declension::Βᾰσκᾱς,
        Declension::Πᾰτηρ,
        Declension::Ἰλυς,
        Declension::Μοῑρᾰ,
        Declension::Σφην,
        Declension::Μητρως,
        Declension::Κῠᾰρ,
        Declension::Ἀνισον,
        Declension::Ἀριθμος,
        Declension::Πτῠξ,
        Declension::Πηχῠς,
        Declension::Παῡλᾰ,
        Declension::Γεῑσον,
        Declension::Γλωχῑς,
        Declension::Πᾰπᾱς,
        Declension::Ἀναξ,
        Declension::Ἀηρ,
        Declension::Ἰδιος,
        Declension::Ἀστερισκος,
        Declension::Ἰσχας,
        Declension::Ἀνατασις,
        Declension::Ἀντιθεσις,
        Declension::Δαιμων,
        Declension::Γεῡσῐς,
        Declension::Νοῡς,
        Declension::Λῐψ,
        Declension::Ἀγκυλη,
        Declension::Τεῡχος,
        Declension::Χλουνης,
        Declension::Λεῑμᾰξ,
        Declension::Φῡσᾱς,
        Declension::Ἱσπανος,
        Declension::Ἁγνεια,
        Declension::Πῠρᾰ,
        Declension::Λαυρᾱ,
        Declension::Ἀελλα,
        Declension::Πλευμων,
        Declension::Ἀκανθις,
        Declension::Ἀκροβυστια,
        Declension::Ἱματιον,
        Declension::Ἀστυρον,
        Declension::Σικανος,
        Declension::Κηρῠξ,
        Declension::Ζωᾱ,
        Declension::Γαλη,
        Declension::Ἀνακυκλωσις,
        Declension::Ἀμανιτης,
        Declension::Ἀθανασια,
        Declension::Θῑς,
        Declension::Μῡς,
        Declension::Ἀγορα,
        Declension::Νεῑκος,
        Declension::Ἀσις,
        Declension::Πειθω,
        Declension::Πεῑνᾰ,
        Declension::Ἀβατον,
        Declension::Ἐγκᾰτᾰ,
        Declension::Ἰασπις,
        Declension::Ὑπαλλαγη,
        Declension::Λυρτος,
        Declension::Δᾰσῠ,
        Declension::Λαλλαι,
        Declension::Βαλσαμον,
        Declension::Ἐντεᾰ,
        Declension::Παστα,
        Declension::Γᾰμφηλαι,
        Declension::Θεωσῐς,
        Declension::Ἀσφαλισις,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 157] = [
    // Τρᾰγος: 1,193 words (λῠκος, κοσμος, ἱππος, ἐρωμενος, σπογγος, ἀγγελος).
    declension!("τρᾰγος", 1193, "~ος";
    nom: "~ος", "~ω", "~οι";
    gen_: "~ου", "~οιν", "~ων";
    dat: "~ῳ", _, "~οις";
    acc: "~ον", _, "~ους";
    voc: "~ε", _, _;
    ),
    // Μορῐον: 1,025 words (ἀστρον, παιδῐον, δενδρον, μῠστηρῐον, ἐργον, εἰδωλον). Like Τρᾰγος, except in 4 cells: nom sg, nom pl, acc pl, voc sg.
    declension!("μορῐον", 1025, "~ον", [gender: Neuter], like Τρᾰγος;
    nom: "~ον", _, "~ᾰ";
    acc: _, _, "~ᾰ";
    voc: "~ον", _, _;
    ),
    // Οὐσῐᾱ: 546 words (δημοκρᾰτῐᾱ, χωρᾱ, μελῳδῐᾱ, φῐλῐᾱ, σοφῐᾱ, ἡγεμονῐᾱ).
    declension!("οὐσῐᾱ", 546, "~ᾱ", [gender: Feminine];
    nom: "~ᾱ", "~ᾱ", "~αι";
    gen_: "~ᾱς", "~αιν", "~ων";
    dat: "~ᾳ", _, "~αις";
    acc: "~ᾱν", _, "~ᾱς";
    voc: "~ᾱ", _, _;
    ),
    // Θεμᾰ: 531 words (μελῐ, γρᾰμμᾰ, δερμᾰ, σῑγμᾰ, βαπτισμᾰ, στομᾰ).
    declension!("θεμᾰ", 531, "~";
    nom: "~", "~τε", "~τᾰ";
    gen_: "~τος", "~τοιν", "~των";
    dat: "~τῐ", _, "~σῐ|~σῐν";
    acc: "~", _, "~τᾰ";
    voc: "~", _, _;
    ),
    // Νῠμφη: 518 words (νῑκη, δῑνη, κορη, γῠνη, ἐλεημοσῠνη, εἰρηνη). Like Οὐσῐᾱ, except in 5 cells: nom sg, gen_ sg, dat sg, acc sg, voc sg.
    declension!("νῠμφη", 518, "~η", [gender: Feminine], like Οὐσῐᾱ;
    nom: "~η", _, _;
    gen_: "~ης", _, _;
    dat: "~ῃ", _, _;
    acc: "~ην", _, _;
    voc: "~η", _, _;
    ),
    // Μερῐσμος: 483 words (λαιμος, μᾰστος, θεος, καιρος, νεκρος, ὀφθᾰλμος). Like Τρᾰγος, except in 3 cells: gen_ sg, gen_ du, dat pl.
    declension!("μερῐσμος", 483, "~ος", [gender: Masculine], like Τρᾰγος;
    gen_: "~οῡ", "~οῑν", _;
    dat: _, _, "~οῑς";
    ),
    // Δῐαιρεσῐς: 433 words (πολῐς, γνωσῐς, κῠβερνησῐς, μορφωσῐς, φῠσῐς, κρῐσῐς).
    declension!("δῐαιρεσῐς", 433, "~ῐς", [gender: Feminine];
    nom: "~ῐς", "~ει", "~εις";
    gen_: "~εως", "~εοιν", "~εων";
    dat: "~ει", _, "~εσῐ|~εσῐν";
    acc: "~ῐν", _, "~εις";
    voc: "~ῐ", _, _;
    ),
    // Κᾰτᾰσκευη: 302 words (μηχᾰνη, μουσῐκη, γρᾰμμᾰτῐκη, φωνη, κεφᾰλη, ἡδονη). Like Νῠμφη, except in 2 cells: gen_ du, dat pl.
    declension!("κᾰτᾰσκευη", 302, "~η", [gender: Feminine], like Νῠμφη;
    gen_: _, "~αῑν", _;
    dat: _, _, "~αῑς";
    ),
    // Γενετης: 204 words (προφητης, κλεπτης, πολῑτης, ἱπποτοξοτης, τρωγλοδῠτης, σῠβᾰρῑτης). Like Νῠμφη, except in 3 cells: nom sg, gen_ sg, voc sg.
    declension!("γενετης", 204, "~ης", [gender: Masculine], like Νῠμφη;
    nom: "~ης", _, _;
    gen_: "~ου", _, _;
    voc: "~ᾰ", _, _;
    ),
    // Κερκῐς: 185 words (παῑς, λεπῐς, πᾰτρῐς, ἑλληνῐς, περσῐς, γρᾰφῐς).
    declension!("κερκῐς", 185, "~ς", [gender: Feminine];
    nom: "~ς", "~δε", "~δες";
    gen_: "~δος", "~δοιν", "~δων";
    dat: "~δῐ", _, "~σῐ|~σῐν";
    acc: "~δᾰ", _, "~δᾰς";
    voc: "~", _, _;
    ),
    // Κορᾰξ: 183 words (χᾰλῐξ, πτερῠξ, γλαῡξ, ἑλῐξ, κῐλῐξ, νῠξ).
    declension!("κορᾰξ", 183, "~ξ";
    nom: "~ξ", "~κε", "~κες";
    gen_: "~κος", "~κοιν", "~κων";
    dat: "~κῐ", _, "~ξῐ|~ξῐν";
    acc: "~κᾰ", _, "~κᾰς";
    voc: "~ξ", _, _;
    ),
    // Κλεις: 165 words (μῠκης, κρης, σκληροτης, μᾰρτῠς, θης, κλεις). Like Κερκῐς, except in 1 cell: voc sg.
    declension!("κλεις", 165, "~ς", like Κερκῐς;
    voc: "~ς", _, _;
    ),
    // Τρᾰπεζᾰ: 133 words (θᾰλᾰσσᾰ, ῥῐζᾰ, θᾰλᾰττᾰ, δοξᾰ, γλωττᾰ, μελῐττᾰ). Like Νῠμφη, except in 3 cells: nom sg, acc sg, voc sg.
    declension!("τρᾰπεζᾰ", 133, "~ᾰ", [gender: Feminine], like Νῠμφη;
    nom: "~ᾰ", _, _;
    acc: "~ᾰν", _, _;
    voc: "~ᾰ", _, _;
    ),
    // Ἐπῐφᾰνειᾰ: 116 words (βᾰσῐλειᾰ, ἀσθενειᾰ, μᾰχαιρᾰ, βοηθειᾰ, συγγενειᾰ, ὀρθοεπειᾰ). Like Τρᾰπεζᾰ, except in 2 cells: gen_ sg, dat sg.
    declension!("ἐπῐφᾰνειᾰ", 116, "~ᾰ", [gender: Feminine], like Τρᾰπεζᾰ;
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
    // Αἰων: 96 words (ἑλλην, αἰων, λεγεων, ἀγων, σᾱπων, λᾰκων).
    declension!("αἰων", 96, "~ν", [gender: Masculine];
    nom: "~ν", "~νε", "~νες";
    gen_: "~νος", "~νοιν", "~νων";
    dat: "~νῐ", _, "~σῐ|~σῐν";
    acc: "~νᾰ", _, "~νᾰς";
    voc: "~ν", _, _;
    ),
    // Χορευτης: 87 words (ἐρᾰστης, ποιητης, βᾰπτῐστης, κρῐτης, κωλοβᾰθρῐστης, σοφῐστης). Like Κᾰτᾰσκευη, except in 3 cells: nom sg, gen_ sg, voc sg.
    declension!("χορευτης", 87, "~ης", [gender: Masculine], like Κᾰτᾰσκευη;
    nom: "~ης", _, _;
    gen_: "~οῡ", _, _;
    voc: "~ᾰ", _, _;
    ),
    // Ἱππευς: 81 words (βᾰσῐλευς, ἀρχιερευς, γραμμᾰτευς, αἰολευς, ἀμφισσευς, ἀντῐκυρευς).
    declension!("ἱππευς", 81, "~ευς", [gender: Masculine];
    nom: "~ευς", "~η", "~ης|~εῑς";
    gen_: "~εως", "~εοιν", "~εων";
    dat: "~εῑ", _, "~εῡσῐ|~εῡσῐν";
    acc: "~εᾱ", _, "~εᾱς";
    voc: "~εῡ", _, "~ης|~εῑς";
    ),
    // Χᾰρᾰκτηρ: 72 words (μᾰρτῠρ, πᾰνθηρ, ἰβηρ, χειρ, ἐορ, κρᾱτηρ).
    declension!("χᾰρᾰκτηρ", 72, "~";
    nom: "~", "~ε", "~ες";
    gen_: "~ος", "~οιν", "~ων";
    dat: "~ῐ", _, "~σῐ|~σῐν";
    acc: "~ᾰ", _, "~ᾰς";
    voc: "~", _, _;
    ),
    // Κεραιᾱ: 71 words (προφητειᾱ, πορνειᾱ, παιδειᾱ, δουλειᾱ, ξῠλειᾱ, νηστειᾱ).
    declension!("κεραιᾱ", 71, "~~ᾱ", [gender: Feminine];
    nom: "~~ᾱ", "~~ᾱ", "~ῑα~";
    gen_: "~~ᾱς", "~~αιν", "~~ων";
    dat: "~~ᾳ", _, "~~αις";
    acc: "~~ᾱν", _, "~~ᾱς";
    voc: "~~ᾱ", _, _;
    ),
    // Σημεῑον: 62 words (θεῑον, πανδοχεῑον, πᾰνδοκεῑον, στοιχεῑον, ἀγγεῑον, βᾰλᾰνεῑον).
    declension!("σημεῑον", 62, "~ῑον", [gender: Neuter];
    nom: "~ῑον", "~ιω", "~ῑᾰ";
    gen_: "~ιου", "~ιοιν", "~ιων";
    dat: "~ιῳ", _, "~ιοις";
    acc: "~ῑον", _, "~ῑᾰ";
    voc: "~ῑον", _, _;
    ),
    // Κᾰλπῐς: 62 words (πελασγῐωτῐς, σπᾰρτῐᾱτῐς, σῠβᾰρῑτῐς, σμῠρῐς, ἀσπρῐς, εὐνῐς). Like Κερκῐς, except in 1 cell: acc sg.
    declension!("κᾰλπῐς", 62, "~ς", like Κερκῐς;
    acc: "~ν", _, _;
    ),
    // Φορᾱ: 54 words (παιδῐᾱ, πᾰτρῐᾱ, γενεᾱ, θεᾱ, στρᾰτῐᾱ, σκῐᾱ). Like Οὐσῐᾱ, except in 2 cells: gen_ du, dat pl.
    declension!("φορᾱ", 54, "~ᾱ", [gender: Feminine], like Οὐσῐᾱ;
    gen_: _, "~αῑν", _;
    dat: _, _, "~αῑς";
    ),
    // Δᾰμᾰλης: 53 words (βελγης, δακης, μῐνῠης, ὀδρῠσης, πῐσῐδης, σᾰκης). Like Γενετης, except in 1 cell: voc sg.
    declension!("δᾰμᾰλης", 53, "~ης", [gender: Masculine], like Γενετης;
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
    // Ζῠγον: 45 words (πτερον, ὀνομᾰστῐκον, ᾠον, ἑρπετον, ζῠγον, φῠτον). Like Μορῐον, except in 3 cells: gen_ sg, gen_ du, dat pl.
    declension!("ζῠγον", 45, "~ον", [gender: Neuter], like Μορῐον;
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
    // Κροσσαι: 39 words (ἑβδομηκοντᾰκωμηται, τεσσᾰρεσκαιδεκᾰτῑται, ὀᾰσῑται, ἰσκαι, κρουπεζαι, ὀζολαι). Like Γενετης, except in 0 cells: .
    declension!("κροσσαι", 39, "~αι", like Γενετης;
    ),
    // Κᾰνων: 34 words (ἡγεμων, τεκτων, μᾰκεδων, μαιων, μῠρμῐδων, παιων).
    declension!("κᾰνων", 34, "~ων";
    nom: "~ων", "~ονε", "~ονες";
    gen_: "~ονος", "~ονοιν", "~ονων";
    dat: "~ονῐ", _, "~οσῐ|~οσῐν";
    acc: "~ονᾰ", _, "~ονᾰς";
    voc: "~ων", _, _;
    ),
    // Ῥητωρ: 33 words (κῠων, ἀλεκτωρ, καστωρ, ῥητωρ, δωτωρ, στατωρ).
    declension!("ῥητωρ", 33, "~ω~";
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
    // Ἀμβροσια: 30 words (ἱστορια, ἰδια, ἀπορια, ἀρτηρια, ἀμνηστια, ἀμβροσια).
    declension!("ἀμβροσια", 30, "~ια", [gender: Feminine];
    nom: "~ῐᾱ", "~ῐᾱ", "~ῐαι";
    gen_: "~ῐᾱς", "~ῐαιν", "~ῐων";
    dat: "~ῐᾳ", _, "~ῐαις";
    acc: "~ῐᾱν", _, "~ῐᾱς";
    voc: "~ῐᾱ", _, _;
    ),
    // Στᾰχῠς: 30 words (λῐβῠς, λῐγῠς, γενῠς, ἐγχελῠς, ῥᾰπῠς, ῥᾰφῠς).
    declension!("στᾰχῠς", 30, "~ῠς";
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
    // Ῥωμαῑος: 26 words (ἰουδαῑος, φαρισαῑος, ῥωμαῑος, ἀνδρεῑος, ἑβραῑος, χᾰλδαῑος). Like Σημεῑον, except in 4 cells: nom sg, nom pl, acc pl, voc sg.
    declension!("ῥωμαῑος", 26, "~ῑος", [gender: Masculine], like Σημεῑον;
    nom: "~ῑος", _, "~ῑοι";
    acc: _, _, "~ιους";
    voc: "~ῑε", _, _;
    ),
    // Ἀξων: 25 words (ἀρχιτεκτων, βιστων, χᾱων, λῠκᾱων, βρᾰχῑων, ἀκμων). Like Κᾰνων, except in 1 cell: voc sg.
    declension!("ἀξων", 25, "~ων", [gender: Masculine], like Κᾰνων;
    voc: "~ον", _, _;
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
    // Ὑποθεσις: 19 words (ὑποθεσις, ἀρσις, ὑβρις, ἀνεσις, ἀσκησις, ἀμαυρωσις). Like Δῐαιρεσῐς, except in 0 cells: .
    declension!("ὑποθεσις", 19, "~ις", [gender: Feminine], like Δῐαιρεσῐς;
    ),
    // Πους: 18 words (πους, ὀκτωπους, πολυπους, τρῐπους, ἀπους, δᾰσῠπους). Like Κερκῐς, except in 2 cells: nom sg, voc sg.
    declension!("πους", 18, "~υς", [gender: Masculine], like Κερκῐς;
    nom: "~υς", _, _;
    voc: "~υς", _, _;
    ),
    // Λᾰτρῐς: 16 words (ὀκρῐς, λᾰτρῐς, μηνῐς, πορτῐς, ἀρδῐς, κᾰπετῐς).
    declension!("λᾰτρῐς", 16, "~ῐς";
    nom: "~ῐς", "~ῐε", "~ῐες";
    gen_: "~ῐος", "~ῐοιν", "~ῐων";
    dat: "~ῑ", _, "~ῐσῐ|~ῐσῐν";
    acc: "~ῐν", _, "~ῑς|~ῐᾰς";
    voc: "~ῐ", _, _;
    ),
    // Ἀστραγᾰλος: 15 words (ἰταλος, ἰθακος, ὑμνογραφος, ἱπποποταμος, ἰαμβος, ἀρακος).
    declension!("ἀστραγᾰλος", 15, "~α~ος";
    nom: "~ᾰ~ος", "~ᾰ~ω", "~ᾰ~οι";
    gen_: "~ᾰ~ου", "~ᾰ~οιν", "~ᾰ~ων";
    dat: "~ᾰ~ῳ", _, "~ᾰ~οις";
    acc: "~ᾰ~ον", _, "~ᾰ~ους";
    voc: "~ᾰ~ε", _, _;
    ),
    // Περᾰς: 15 words (φως, τερᾰς, χᾰρῐς, κρεᾰς, πᾰγκρεᾰς, ἐρῠσῐπελᾰς). Like Θεμᾰ, except in 3 cells: nom sg, acc sg, voc sg.
    declension!("περᾰς", 15, "~ς", like Θεμᾰ;
    nom: "~ς", _, _;
    acc: "~ς", _, _;
    voc: "~ς", _, _;
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
    // Νηδῡς: 14 words (βοῡς, δελφῡς, ὀφρῡς, γρᾱῡς, λῐγνῡς, ἐδητῡς). Like Στᾰχῠς, except in 4 cells: nom sg, dat sg, acc sg, voc sg.
    declension!("νηδῡς", 14, "~ῡς", like Στᾰχῠς;
    nom: "~ῡς", _, _;
    dat: "~ῠϊ|~υῑ", _, _;
    acc: "~ῡν", _, _;
    voc: "~ῡ", _, _;
    ),
    // Σηψ: 13 words (ὠψ, γρῡψ, θρῑψ, θωψ, κλωψ, σκωψ). Like Κολλοψ, except in 1 cell: gen_ du.
    declension!("σηψ", 13, "~ψ", like Κολλοψ;
    gen_: _, "~ποῑν", _;
    ),
    // Ἀποστασις: 13 words (ὑπαρξις, ἀναβρωσις, ἀποταξις, ὑποταξις, ὑπερταξις, ἀναχωρησις).
    declension!("ἀποστασις", 13, "~α~ις", [gender: Feminine];
    nom: "~ᾰ~ῐς", "~ᾰ~ει", "~ᾰ~εις";
    gen_: "~ᾰ~εως", "~ᾰ~εοιν", "~ᾰ~εων";
    dat: "~ᾰ~ει", _, "~ᾰ~εσῐ|~ᾰ~εσῐν";
    acc: "~ᾰ~ῐν", _, "~ᾰ~εις";
    voc: "~ᾰ~ῐ", _, _;
    ),
    // Ἀσαῑοι: 13 words (βοττῐαῑοι, ἀραμαῑοι, ἀρδιαῑοι, ἀσαῑοι, θᾰμᾰναῑοι, νῐψαῑοι). Like Ῥωμαῑος, except in 0 cells: .
    declension!("ἀσαῑοι", 13, "~ῑοι", [gender: Masculine], like Ῥωμαῑος;
    ),
    // Ἰδεα: 12 words (ἰδεα, ἀκρα, ὑδρα, ἀνθρωποποιϊα, ἰτεα, ὑστερα). Like Οὐσῐᾱ, except in 0 cells: .
    declension!("ἰδεα", 12, "~α", [gender: Feminine], like Οὐσῐᾱ;
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
    // Ῥως: 12 words (ἑξηκοντᾰ, ὀσσε, ὑφεν, ῥᾰββῑ, ῥᾰββονῑ, ῥᾰββουνῑ).
    declension!("ῥως", 12, "~";
    nom: "~", "~", "~";
    gen_: "~", "~", "~";
    dat: "~", _, "~";
    acc: "~", _, "~";
    voc: "~", _, _;
    ),
    // Στοῑχος: 12 words (χοῑρος, ἑταῑρος, στοῑχος, τοῑχος, σχοῑνος, ῥοῑζος).
    declension!("στοῑχος", 12, "~ῑ~ος", [gender: Masculine];
    nom: "~ῑ~ος", "~ι~ω", "~ῑ~οι";
    gen_: "~ι~ου", "~ι~οιν", "~ι~ων";
    dat: "~ι~ῳ", _, "~ι~οις";
    acc: "~ῑ~ον", _, "~ι~ους";
    voc: "~ῑ~ε", _, _;
    ),
    // Ἀλωπεκιᾱ: 12 words (ὑποψιᾱ, ἀβλεψιᾱ, ἀκηδιᾱ, ἀτροφιᾱ, ἀναιμιᾱ, ἀλωπεκιᾱ). Like Ἀμβροσια, except in 0 cells: .
    declension!("ἀλωπεκιᾱ", 12, "~ιᾱ", [gender: Feminine], like Ἀμβροσια;
    ),
    // Πτερῠξ: 11 words (λᾰτᾰξ, ὀρτῠξ, μᾰστῑξ, τεττῑξ, λᾰλᾰξ, πομφολῠξ).
    declension!("πτερῠξ", 11, "~ξ";
    nom: "~ξ", "~γε", "~γες";
    gen_: "~γος", "~γοιν", "~γων";
    dat: "~γῐ", _, "~ξῐ|~ξῐν";
    acc: "~γᾰ", _, "~γᾰς";
    voc: "~ξ", _, _;
    ),
    // Ἀραχνη: 11 words (ἀγαπη, ἀπατη, ἀναγκη, ἀραχνη, ἀγκαλη, ὑπογεγραμμενη).
    declension!("ἀραχνη", 11, "~α~η", [gender: Feminine];
    nom: "~ᾰ~η", "~ᾰ~ᾱ", "~ᾰ~αι";
    gen_: "~ᾰ~ης", "~ᾰ~αιν", "~ᾰ~ων";
    dat: "~ᾰ~ῃ", _, "~ᾰ~αις";
    acc: "~ᾰ~ην", _, "~ᾰ~ᾱς";
    voc: "~ᾰ~η", _, _;
    ),
    // Τρωες: 11 words (πῑερες, τρωες, κηφηνες, ἁλτηρες, αἰγες, ἀνᾰξῠρῐδες). Like Ὀργῐᾰ, except in 4 cells: nom sg, nom pl, dat pl, acc pl.
    declension!("τρωες", 11, "~ες", like Ὀργῐᾰ;
    nom: "~ες", _, "~ες";
    dat: _, _, "~σῐ|~σῐν";
    acc: _, _, "~ᾰς";
    ),
    // Ὑπομνημα: 11 words (ὑποστρωμα, ὑπομνημα, ὑποδημα, ἀγνοημα, ἁρμα, ἁλμα).
    declension!("ὑπομνημα", 11, "~α", [gender: Neuter];
    nom: "~ᾰ", "~ᾰτε", "~ᾰτᾰ";
    gen_: "~ᾰτος", "~ᾰτοιν", "~ᾰτων";
    dat: "~ᾰτῐ", _, "~ᾰσῐ|~ᾰσῐν";
    acc: "~ᾰ", _, "~ᾰτᾰ";
    voc: "~ᾰ", _, _;
    ),
    // Ἀντονομασια: 10 words (ἀνθρακια, ἀναρχια, ἁμαρτια, ἱπποκρατια, ἀντονομασια, ἁρματοδρομια).
    declension!("ἀντονομασια", 10, "~α~ια", [gender: Feminine];
    nom: "~ᾰ~ῐᾱ", "~ᾰ~ῐᾱ", "~ᾰ~ῐαι";
    gen_: "~ᾰ~ῐᾱς", "~ᾰ~ῐαιν", "~ᾰ~ῐων";
    dat: "~ᾰ~ῐᾳ", _, "~ᾰ~ῐαις";
    acc: "~ᾰ~ῐᾱν", _, "~ᾰ~ῐᾱς";
    voc: "~ᾰ~ῐᾱ", _, _;
    ),
    // Ἰδιον: 10 words (ἰδιον, ἰσχιον, ἀγγουριον, ἱστιον, ἰπνιον, ἰνιον).
    declension!("ἰδιον", 10, "~ιον", [gender: Neuter];
    nom: "~ῐον", "~ῐω", "~ῐᾰ";
    gen_: "~ῐου", "~ῐοιν", "~ῐων";
    dat: "~ῐῳ", _, "~ῐοις";
    acc: "~ῐον", _, "~ῐᾰ";
    voc: "~ῐον", _, _;
    ),
    // Ἱεραρχης: 10 words (σαϊτης, ἰλαρχης, ἱλαστης, ἱεραρχης, ἀναλεκτης, ἀκροβατης). Like Ἀραχνη, except in 3 cells: nom sg, gen_ sg, voc sg.
    declension!("ἱεραρχης", 10, "~α~ης", like Ἀραχνη;
    nom: "~ᾰ~ης", _, _;
    gen_: "~ᾰ~ου", _, _;
    voc: "~ᾰ~ᾰ", _, _;
    ),
    // Λευκη: 10 words (πευκη, δουλη, βλαυτη, σενδουκη, λευκη, ζευγλη).
    declension!("λευκη", 10, "~υ~η", [gender: Feminine];
    nom: "~υ~η", "~υ~ᾱ", "~ῡ~αι";
    gen_: "~υ~ης", "~υ~αιν", "~υ~ων";
    dat: "~υ~ῃ", _, "~υ~αις";
    acc: "~υ~ην", _, "~υ~ᾱς";
    voc: "~υ~η", _, _;
    ),
    // Ἀκανθα: 10 words (ἁμαξα, ἱλαειρα, ἀκανθα, ὑπατεια, ἀσφαλεια, ἀγαθοποιϊα). Like Ἱεραρχης, except in 4 cells: nom sg, gen_ sg, dat sg, acc sg.
    declension!("ἀκανθα", 10, "~α~α", [gender: Feminine], like Ἱεραρχης;
    nom: "~ᾰ~ᾰ", _, _;
    gen_: "~ᾰ~ᾱς", _, _;
    dat: "~ᾰ~ᾳ", _, _;
    acc: "~ᾰ~ᾰν", _, _;
    ),
    // Πεπερῐ: 9 words (σῐνᾱπῐ, κομμῐ, πεπερῐ, κιννᾰβᾰρῐ, στῐμμῐ, κῐκῐ). Like Δῐαιρεσῐς, except in 5 cells: nom sg, nom pl, acc sg, acc pl, voc pl.
    declension!("πεπερῐ", 9, "~ῐ", [gender: Neuter], like Δῐαιρεσῐς;
    nom: "~ῐ", _, "~η|~εᾰ";
    acc: "~ῐ", _, "~η|~εᾰ";
    voc: _, _, "~η|~εᾰ";
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
    // Κωμῡς: 9 words (ἑλμῐνς, ὀρνῑς, κωμῡς, κορῠς, γελγῑς, δελλῑς).
    declension!("κωμῡς", 9, "~ς";
    nom: "~ς", "~θε", "~θες";
    gen_: "~θος", "~θοιν", "~θων";
    dat: "~θῐ", _, "~σῐ|~σῐν";
    acc: "~θᾰ", _, "~θᾰς";
    voc: "~ς", _, _;
    ),
    // Πλᾰτεῑᾰ: 9 words (μυῑᾰ, πλᾰτεῑᾰ, γαῑᾰ, μαῑᾰ, βᾰρεῑᾰ, ὀξεῑᾰ).
    declension!("πλᾰτεῑᾰ", 9, "~ῑᾰ", [gender: Feminine];
    nom: "~ῑᾰ", "~ιᾱ", "~ῑαι";
    gen_: "~ιᾱς", "~ιαιν", "~ιων";
    dat: "~ιᾳ", _, "~ιαις";
    acc: "~ῑᾰν", _, "~ιᾱς";
    voc: "~ῑᾰ", _, _;
    ),
    // Ἀσπις: 9 words (ἀσπις, ἀγρωστις, ἀκις, ἁλουργις, ἱππουρις, ἀμις).
    declension!("ἀσπις", 9, "~ις", [gender: Feminine];
    nom: "~ῐς", "~ῐδε", "~ῐδες";
    gen_: "~ῐδος", "~ῐδοιν", "~ῐδων";
    dat: "~ῐδῐ", _, "~ῐσῐ|~ῐσῐν";
    acc: "~ῐδᾰ", _, "~ῐδᾰς";
    voc: "~ῐ", _, _;
    ),
    // Σησᾰμοειδες: 9 words (νευροειδες, μονοειδες, τρῐχομᾰνες, ἡμεροκᾰλλες, σησᾰμοειδες, ὠκῐμοειδες). Like Γενος, except in 6 cells: nom sg, nom du, gen_ sg, dat sg, acc sg, voc sg.
    declension!("σησᾰμοειδες", 9, "~ες", [gender: Neuter], like Γενος;
    nom: "~ες", "~εῑ", _;
    gen_: "~οῡς", _, _;
    dat: "~εῑ", _, _;
    acc: "~ες", _, _;
    voc: "~ες", _, _;
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
    // Ἀμεθυστος: 8 words (ἀργυρος, δρυτομος, ἀνδρογυνος, ἰθυφαλλος, ἀμεθυστος, ἀργυροτοξος).
    declension!("ἀμεθυστος", 8, "~υ~ος";
    nom: "~ῠ~ος", "~ῠ~ω", "~ῠ~οι";
    gen_: "~ῠ~ου", "~ῠ~οιν", "~ῠ~ων";
    dat: "~ῠ~ῳ", _, "~ῠ~οις";
    acc: "~ῠ~ον", _, "~ῠ~ους";
    voc: "~ῠ~ε", _, _;
    ),
    // Ἀναθεμα: 8 words (ἁμαρτημα, ἱλασμα, ἀγαλμα, ἁρπαγμα, ἀναθεμα, ὑφασμα).
    declension!("ἀναθεμα", 8, "~α~α", [gender: Neuter];
    nom: "~ᾰ~ᾰ", "~ᾰ~ᾰτε", "~ᾰ~ᾰτᾰ";
    gen_: "~ᾰ~ᾰτος", "~ᾰ~ᾰτοιν", "~ᾰ~ᾰτων";
    dat: "~ᾰ~ᾰτῐ", _, "~ᾰ~ᾰσῐ|~ᾰ~ᾰσῐν";
    acc: "~ᾰ~ᾰ", _, "~ᾰ~ᾰτᾰ";
    voc: "~ᾰ~ᾰ", _, _;
    ),
    // Ῥᾱξ: 7 words (φλοξ, αἰξ, σφᾰξ, ῥᾱξ, στρῐξ, ῥωξ). Like Πτερῠξ, except in 1 cell: gen_ du.
    declension!("ῥᾱξ", 7, "~ξ", like Πτερῠξ;
    gen_: _, "~γοῑν", _;
    ),
    // Ἀληθεια: 7 words (ἀληθεια, ἀγνοια, ἀμελεια, ἀσεβεια, ἀρουρα, ἀδεια). Like Ἐπῐφᾰνειᾰ, except in 0 cells: .
    declension!("ἀληθεια", 7, "~α", [gender: Feminine], like Ἐπῐφᾰνειᾰ;
    ),
    // Ἰδιωτης: 7 words (ὑποκριτης, ἀγωνιστης, ἰδιωτης, ἀκοντιστης, ὑψιβρεμετης, ἰᾰμιδης).
    declension!("ἰδιωτης", 7, "~ι~ης";
    nom: "~ῐ~ης", "~ῐ~ᾱ", "~ῐ~αι";
    gen_: "~ῐ~οῡ", "~ῐ~αῑν", "~ῐ~ων";
    dat: "~ῐ~ῃ", _, "~ῐ~αῑς";
    acc: "~ῐ~ην", _, "~ῐ~ᾱς";
    voc: "~ῐ~ᾰ", _, _;
    ),
    // Ἀδᾰμᾱς: 7 words (κορῠβᾱς, γῐγᾱς, ἐλεφᾱς, ἀδᾰμᾱς, λῠκᾰβᾱς, ἀλῐβᾱς).
    declension!("ἀδᾰμᾱς", 7, "~ᾱς", [gender: Masculine];
    nom: "~ᾱς", "~ᾰντε", "~ᾰντες";
    gen_: "~ᾰντος", "~ᾰντοιν", "~ᾰντων";
    dat: "~ᾰντῐ", _, "~ᾱσῐ|~ᾱσῐν";
    acc: "~ᾰντᾰ", _, "~ᾰντᾰς";
    voc: "~ᾰν", _, _;
    ),
    // Ἠχω: 7 words (ἠχω, τυτω, κῐττω, πῑπω, μοτω, φειδω). Like Μερῐσμος, except in 5 cells: nom sg, gen_ sg, dat sg, acc sg, voc sg.
    declension!("ἠχω", 7, "~ω", [gender: Feminine], like Μερῐσμος;
    nom: "~ω", _, _;
    gen_: "~οῡς", _, _;
    dat: "~οῑ", _, _;
    acc: "~ω", _, _;
    voc: "~οῑ", _, _;
    ),
    // Μωλῠ: 7 words (δᾰκρῠ, μωλῠ, νᾱπῠ, μῐσῠ, βρᾰθῠ, σωρῠ). Like Στᾰχῠς, except in 5 cells: nom sg, nom pl, acc sg, acc pl, voc pl.
    declension!("μωλῠ", 7, "~ῠ", [gender: Neuter], like Στᾰχῠς;
    nom: "~ῠ", _, "~ῠᾰ";
    acc: "~ῠ", _, "~ῠᾰ";
    voc: _, _, derived;
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
    // Ἀναγωγη: 7 words (ἀστραπη, ἀνατροπη, ἀνατροφη, ἀναγωγη, ἀναστροφη, ἀναβολη). Like Ἀραχνη, except in 2 cells: gen_ du, dat pl.
    declension!("ἀναγωγη", 7, "~α~η", [gender: Feminine], like Ἀραχνη;
    gen_: _, "~ᾰ~αῑν", _;
    dat: _, _, "~ᾰ~αῑς";
    ),
    // Βᾰσκᾱς: 7 words (πᾰξᾰμᾱς, μᾰρῐκᾱς, ἀττᾰγᾱς, μᾰσουχᾱς, βᾰσκᾱς, χαλιφᾱς). Like Φορᾱ, except in 3 cells: nom sg, nom pl, gen_ sg.
    declension!("βᾰσκᾱς", 7, "~ᾱς", [gender: Masculine], like Φορᾱ;
    nom: "~ᾱς", _, "~αῑ";
    gen_: "~ᾱ", _, _;
    ),
    // Πᾰτηρ: 6 words (ἀνηρ, μητηρ, πᾰτηρ, ἀστηρ, θῠγᾰτηρ, ἐνᾰτηρ).
    declension!("πᾰτηρ", 6, "~η~";
    nom: "~η~", "~ε~ε", "~ε~ες";
    gen_: "~~ος", "~ε~οιν", "~ε~ων";
    dat: "~~ῐ", _, "~~ᾰσῐ";
    acc: "~ε~ᾰ", _, "~ε~ᾰς";
    voc: "~ε~", _, _;
    ),
    // Ἰλυς: 6 words (ἰχθυς, ἰσχυς, ἰτυς, ἀχλυς, ἀρκυς, ἰλυς). Like Νηδῡς, except in 0 cells: .
    declension!("ἰλυς", 6, "~υς", like Νηδῡς;
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
    // Κῠᾰρ: 6 words (οὐθᾰρ, ἡπᾰρ, φρεᾱρ, στεᾱρ, δελεᾰρ, κῠᾰρ). Like Θεμᾰ, except in 3 cells: nom sg, acc sg, voc sg.
    declension!("κῠᾰρ", 6, "~ρ", [gender: Neuter], like Θεμᾰ;
    nom: "~ρ", _, _;
    acc: "~ρ", _, _;
    voc: "~ρ", _, _;
    ),
    // Ἀνισον: 6 words (ἀλφιτον, ἀνισον, ἀγκιστρον, ἀντιθετον, ἱστορικον, ἀριστερον).
    declension!("ἀνισον", 6, "~ι~ον", [gender: Neuter];
    nom: "~ῐ~ον", "~ῐ~ω", "~ῐ~ᾰ";
    gen_: "~ῐ~ου", "~ῐ~οιν", "~ῐ~ων";
    dat: "~ῐ~ῳ", _, "~ῐ~οις";
    acc: "~ῐ~ον", _, "~ῐ~ᾰ";
    voc: "~ῐ~ον", _, _;
    ),
    // Ἀριθμος: 6 words (ἀριθμος, ἀφορισμος, ἱστορικος, ἁγνισμος, σιρος, ἀκιδωτος). Like Ἀνισον, except in 7 cells: nom sg, nom pl, gen_ sg, gen_ du, dat pl, acc pl, ….
    declension!("ἀριθμος", 6, "~ι~ος", [gender: Masculine], like Ἀνισον;
    nom: "~ῐ~ος", _, "~ῐ~οι";
    gen_: "~ῐ~οῡ", "~ῐ~οῑν", _;
    dat: _, _, "~ῐ~οῑς";
    acc: _, _, "~ῐ~ους";
    voc: "~ῐ~ε", _, _;
    ),
    // Πτῠξ: 6 words (ὀνῠξ, ψῑξ, σᾰρδονῠξ, στῐξ, βηξ, πτῠξ).
    declension!("πτῠξ", 6, "~ξ";
    nom: "~ξ", "~χε", "~χες";
    gen_: "~χος", "~χοῑν", "~χων";
    dat: "~χῐ", _, "~ξῐ|~ξῐν";
    acc: "~χᾰ", _, "~χᾰς";
    voc: "~ξ", _, _;
    ),
    // Πηχῠς: 6 words (ἡμῐσῠς, υἱῠς, πελεκῠς, πρεσβῠς, πηχῠς, τρῐβρᾰχῠς). Like Δῐαιρεσῐς, except in 3 cells: nom sg, acc sg, voc sg.
    declension!("πηχῠς", 6, "~ῠς", [gender: Masculine], like Δῐαιρεσῐς;
    nom: "~ῠς", _, _;
    acc: "~ῠν", _, _;
    voc: "~ῠ", _, _;
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
    // Γλωχῑς: 6 words (ὠδῑς, γλωχῑς, στᾰμῑς, ἑρμῑς, ὀρκῡς, τελμῑς). Like Αἰων, except in 2 cells: nom sg, voc sg.
    declension!("γλωχῑς", 6, "~ς", like Αἰων;
    nom: "~ς", _, _;
    voc: "~ς", _, _;
    ),
    // Πᾰπᾱς: 6 words (πᾰπᾱς, γᾰλᾰτᾱς, κᾰδῡτᾱς, γενετᾱς, σᾰκτᾱς, πᾰππᾱς). Like Κοχλῐᾱς, except in 1 cell: voc sg.
    declension!("πᾰπᾱς", 6, "~ᾱς", [gender: Masculine], like Κοχλῐᾱς;
    voc: "~ᾰ", _, _;
    ),
    // Ἀναξ: 5 words (ἀνθραξ, ἀβαξ, ἀναξ, ὑραξ, ἁρπαξ).
    declension!("ἀναξ", 5, "~αξ", [gender: Masculine];
    nom: "~ᾰξ", "~ᾰκε", "~ᾰκες";
    gen_: "~ᾰκος", "~ᾰκοιν", "~ᾰκων";
    dat: "~ᾰκῐ", _, "~ᾰξῐ|~ᾰξῐν";
    acc: "~ᾰκᾰ", _, "~ᾰκᾰς";
    voc: "~ᾰξ", _, _;
    ),
    // Ἀηρ: 5 words (γᾰστηρ, ἀηρ, φρᾱτηρ, αἰθηρ, ἀθηρ). Like Πᾰτηρ, except in 4 cells: gen_ sg, dat sg, dat pl, voc sg.
    declension!("ἀηρ", 5, "~η~", like Πᾰτηρ;
    gen_: "~ε~ος", _, _;
    dat: "~ε~ῐ", _, "~ε~σῐ|~ε~σῐν";
    voc: "~η~", _, _;
    ),
    // Ἰδιος: 5 words (ἁγιος, ἰδιος, ἀνεψιος, ἁλιος, ἀπιος). Like Ἰδιον, except in 4 cells: nom sg, nom pl, acc pl, voc sg.
    declension!("ἰδιος", 5, "~ιος", [gender: Masculine], like Ἰδιον;
    nom: "~ῐος", _, "~ῐοι";
    acc: _, _, "~ῐους";
    voc: "~ῐε", _, _;
    ),
    // Ἀστερισκος: 5 words (ἁλιαιετος, ἀνθρωπισκος, ἀργιλλος, ἀστερισκος, ἀγχινοος). Like Ἀριθμος, except in 3 cells: gen_ sg, gen_ du, dat pl.
    declension!("ἀστερισκος", 5, "~ι~ος", like Ἀριθμος;
    gen_: "~ῐ~ου", "~ῐ~οιν", _;
    dat: _, _, "~ῐ~οις";
    ),
    // Ἰσχας: 5 words (ἀρκας, ἱπποφοβας, ἁλας, ἰσχας, ἀελλας).
    declension!("ἰσχας", 5, "~ας";
    nom: "~ᾰς", "~ᾰδε", "~ᾰδες";
    gen_: "~ᾰδος", "~ᾰδοιν", "~ᾰδων";
    dat: "~ᾰδῐ", _, "~ᾰσῐ|~ᾰσῐν";
    acc: "~ᾰδᾰ", _, "~ᾰδᾰς";
    voc: "~ᾰς", _, _;
    ),
    // Ἀνατασις: 5 words (ἀναστασις, ἀγανακτησις, ἀναταξις, ἀνατασις, ἀναβασις).
    declension!("ἀνατασις", 5, "~α~α~ις", [gender: Feminine];
    nom: "~ᾰ~ᾰ~ῐς", "~ᾰ~ᾰ~ει", "~ᾰ~ᾰ~εις";
    gen_: "~ᾰ~ᾰ~εως", "~ᾰ~ᾰ~εοιν", "~ᾰ~ᾰ~εων";
    dat: "~ᾰ~ᾰ~ει", _, "~ᾰ~ᾰ~εσῐ|~ᾰ~ᾰ~εσῐν";
    acc: "~ᾰ~ᾰ~ῐν", _, "~ᾰ~ᾰ~εις";
    voc: "~ᾰ~ᾰ~ῐ", _, _;
    ),
    // Ἀντιθεσις: 5 words (ὑποκρισις, ἀντιθεσις, ἀντιπτωσις, ὑποκιστις, ἀφιξις).
    declension!("ἀντιθεσις", 5, "~ι~ις", [gender: Feminine];
    nom: "~ῐ~ῐς", "~ῐ~ει", "~ῐ~εις";
    gen_: "~ῐ~εως", "~ῐ~εοιν", "~ῐ~εων";
    dat: "~ῐ~ει", _, "~ῐ~εσῐ|~ῐ~εσῐν";
    acc: "~ῐ~ῐν", _, "~ῐ~εις";
    voc: "~ῐ~ῐ", _, _;
    ),
    // Δαιμων: 5 words (δαιμων, μειζων, γειτων, ποτᾰμογειτων, σεισων).
    declension!("δαιμων", 5, "~ι~ων", [gender: Masculine];
    nom: "~ι~ων", "~ι~ονε", "~ι~ονες";
    gen_: "~ι~ονος", "~ι~ονοιν", "~ι~ονων";
    dat: "~ι~ονῐ", _, "~ι~οσῐ|~ι~οσῐν";
    acc: "~ι~ονᾰ", _, "~ι~ονᾰς";
    voc: "~ῑ~ον", _, _;
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
    // Ἀγκυλη: 5 words (ἀφροσυνη, ἀργυροθηκη, ὑδρομυλη, ἀγκυλη, ἱπποσυνη).
    declension!("ἀγκυλη", 5, "~υ~η", [gender: Feminine];
    nom: "~ῠ~η", "~ῠ~ᾱ", "~ῠ~αι";
    gen_: "~ῠ~ης", "~ῠ~αιν", "~ῠ~ων";
    dat: "~ῠ~ῃ", _, "~ῠ~αις";
    acc: "~ῠ~ην", _, "~ῠ~ᾱς";
    voc: "~ῠ~η", _, _;
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
    // Ἱσπανος: 4 words (ἰατρος, λαϊκος, ἱσπανος, ἀρχαγος).
    declension!("ἱσπανος", 4, "~α~ος", [gender: Masculine];
    nom: "~ᾱ~ος", "~ᾱ~ω", "~ᾱ~οι";
    gen_: "~ᾱ~οῡ", "~ᾱ~οῑν", "~ᾱ~ων";
    dat: "~ᾱ~ῳ", _, "~ᾱ~οῑς";
    acc: "~ᾱ~ον", _, "~ᾱ~ους";
    voc: "~ᾱ~ε", _, _;
    ),
    // Ἁγνεια: 4 words (ἰουδαια, ἁγνεια, ἀνδρεια, ὑστεραια). Like Κεραιᾱ, except in 0 cells: .
    declension!("ἁγνεια", 4, "~~α", [gender: Feminine], like Κεραιᾱ;
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
    // Ἀελλα: 4 words (ὑαινα, ὑποτεινουσα, ἀγχουσα, ἀελλα). Like Τρᾰπεζᾰ, except in 0 cells: .
    declension!("ἀελλα", 4, "~α", [gender: Feminine], like Τρᾰπεζᾰ;
    ),
    // Πλευμων: 4 words (πλευμων, πνευμων, μουσμων, ἰχνευμων).
    declension!("πλευμων", 4, "~υ~ων", [gender: Masculine];
    nom: "~υ~ων", "~υ~ονε", "~υ~ονες";
    gen_: "~υ~ονος", "~υ~ονοιν", "~υ~ονων";
    dat: "~υ~ονῐ", _, "~υ~οσῐ|~υ~οσῐν";
    acc: "~υ~ονᾰ", _, "~υ~ονᾰς";
    voc: "~ῡ~ον", _, _;
    ),
    // Ἀκανθις: 4 words (ἁμαμηλις, ὑπογραφις, ἀκανθις, ἀσταφις).
    declension!("ἀκανθις", 4, "~α~ις", [gender: Feminine];
    nom: "~ᾰ~ῐς", "~ᾰ~ῐδε", "~ᾰ~ῐδες";
    gen_: "~ᾰ~ῐδος", "~ᾰ~ῐδοιν", "~ᾰ~ῐδων";
    dat: "~ᾰ~ῐδῐ", _, "~ᾰ~ῐσῐ|~ᾰ~ῐσῐν";
    acc: "~ᾰ~ῐδᾰ", _, "~ᾰ~ῐδᾰς";
    voc: "~ᾰ~ῐ", _, _;
    ),
    // Ἀκροβυστια: 4 words (ἀσφυξια, ἀκροβυστια, ἀμβλυωπια, ἀσυμμετρια).
    declension!("ἀκροβυστια", 4, "~υ~ια", [gender: Feminine];
    nom: "~ῠ~ῐᾱ", "~ῠ~ῐᾱ", "~ῠ~ῐαι";
    gen_: "~ῠ~ῐᾱς", "~ῠ~ῐαιν", "~ῠ~ῐων";
    dat: "~ῠ~ῐᾳ", _, "~ῠ~ῐαις";
    acc: "~ῠ~ῐᾱν", _, "~ῠ~ῐᾱς";
    voc: "~ῠ~ῐᾱ", _, _;
    ),
    // Ἱματιον: 4 words (ἱματιον, ἀκουσματιον, ἀνακτοριον, ἰναριον).
    declension!("ἱματιον", 4, "~α~ιον", [gender: Neuter];
    nom: "~ᾰ~ῐον", "~ᾰ~ῐω", "~ᾰ~ῐᾰ";
    gen_: "~ᾰ~ῐου", "~ᾰ~ῐοιν", "~ᾰ~ῐων";
    dat: "~ᾰ~ῐῳ", _, "~ᾰ~ῐοις";
    acc: "~ᾰ~ῐον", _, "~ᾰ~ῐᾰ";
    voc: "~ᾰ~ῐον", _, _;
    ),
    // Ἀστυρον: 4 words (ἀσυνδετον, ἀστυρον, ὑδνοφυλλον, ἰχθυοπωλεῑον). Like Ἀμεθυστος, except in 4 cells: nom sg, nom pl, acc pl, voc sg.
    declension!("ἀστυρον", 4, "~υ~ον", [gender: Neuter], like Ἀμεθυστος;
    nom: "~ῠ~ον", _, "~ῠ~ᾰ";
    acc: _, _, "~ῠ~ᾰ";
    voc: "~ῠ~ον", _, _;
    ),
    // Σικανος: 4 words (σικανος, ἀντιπατρος, ἀρχιδαμος, ἁλιβαπτος).
    declension!("σικανος", 4, "~ι~α~ος", [gender: Masculine];
    nom: "~ῐ~ᾰ~ος", "~ῐ~ᾰ~ω", "~ῐ~ᾰ~οι";
    gen_: "~ῐ~ᾰ~ου", "~ῐ~ᾰ~οιν", "~ῐ~ᾰ~ων";
    dat: "~ῐ~ᾰ~ῳ", _, "~ῐ~ᾰ~οις";
    acc: "~ῐ~ᾰ~ον", _, "~ῐ~ᾰ~ους";
    voc: "~ῐ~ᾰ~ε", _, _;
    ),
    // Κηρῠξ: 4 words (κοκκῠξ, κηρῠξ, ψευδοκηρῠξ, ἱεροκηρῠξ).
    declension!("κηρῠξ", 4, "~ῠξ", [gender: Masculine];
    nom: "~ῠξ", "~ῡκε", "~ῡκες";
    gen_: "~ῡκος", "~ῡκοιν", "~ῡκων";
    dat: "~ῡκῐ", _, "~ῡξῐ|~ῡξῐν";
    acc: "~ῡκᾰ", _, "~ῡκᾰς";
    voc: "~ῠξ", _, _;
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
    // Ἀνακυκλωσις: 3 words (ἀποκαλυψις, ἀνακυκλωσις, ἀνακυκλησις).
    declension!("ἀνακυκλωσις", 3, "~α~υ~ις", [gender: Feminine];
    nom: "~ᾰ~ῠ~ῐς", "~ᾰ~ῠ~ει", "~ᾰ~ῠ~εις";
    gen_: "~ᾰ~ῠ~εως", "~ᾰ~ῠ~εοιν", "~ᾰ~ῠ~εων";
    dat: "~ᾰ~ῠ~ει", _, "~ᾰ~ῠ~εσῐ|~ᾰ~ῠ~εσῐν";
    acc: "~ᾰ~ῠ~ῐν", _, "~ᾰ~ῠ~εις";
    voc: "~ᾰ~ῠ~ῐ", _, _;
    ),
    // Ἀμανιτης: 3 words (ἰσραηλιτης, ἱεραπολιτης, ἀμανιτης).
    declension!("ἀμανιτης", 3, "~α~ι~ης", [gender: Masculine];
    nom: "~ᾱ~ῑ~ης", "~ᾱ~ῑ~ᾱ", "~ᾱ~ῑ~αι";
    gen_: "~ᾱ~ῑ~ου", "~ᾱ~ῑ~αιν", "~ᾱ~ῑ~ων";
    dat: "~ᾱ~ῑ~ῃ", _, "~ᾱ~ῑ~αις";
    acc: "~ᾱ~ῑ~ην", _, "~ᾱ~ῑ~ᾱς";
    voc: "~ᾱ~ῑ~ᾰ", _, _;
    ),
    // Ἀθανασια: 3 words (ἀμαζονομαχια, ἀταραξια, ἀθανασια).
    declension!("ἀθανασια", 3, "~α~α~ια", [gender: Feminine];
    nom: "~ᾰ~ᾰ~ῐᾱ", "~ᾰ~ᾰ~ῐᾱ", "~ᾰ~ᾰ~ῐαι";
    gen_: "~ᾰ~ᾰ~ῐᾱς", "~ᾰ~ᾰ~ῐαιν", "~ᾰ~ᾰ~ῐων";
    dat: "~ᾰ~ᾰ~ῐᾳ", _, "~ᾰ~ᾰ~ῐαις";
    acc: "~ᾰ~ᾰ~ῐᾱν", _, "~ᾰ~ᾰ~ῐᾱς";
    voc: "~ᾰ~ᾰ~ῐᾱ", _, _;
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
    // Ἀγορα: 3 words (ἰα, ἀγορα, ἀρα). Like Φορᾱ, except in 0 cells: .
    declension!("ἀγορα", 3, "~α", [gender: Feminine], like Φορᾱ;
    ),
    // Νεῑκος: 3 words (τεῑχος, χεῑλος, νεῑκος).
    declension!("νεῑκος", 3, "~ῑ~ος", [gender: Neuter];
    nom: "~ῑ~ος", "~ι~ει", "~ι~η";
    gen_: "~ι~ους", "~ι~οῑν", "~ι~ων";
    dat: "~ι~ει", _, "~ι~εσῐ|~ι~εσῐν";
    acc: "~ῑ~ος", _, "~ι~η";
    voc: "~ῑ~ος", _, _;
    ),
    // Ἀσις: 3 words (ἀκρις, ἀκοιτις, ἀσις). Like Λᾰτρῐς, except in 0 cells: .
    declension!("ἀσις", 3, "~ις", [gender: Feminine], like Λᾰτρῐς;
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
    // Ἀβατον: 3 words (ἀνδραποδον, ἀσαρον, ἀβατον). Like Ἀστραγᾰλος, except in 4 cells: nom sg, nom pl, acc pl, voc sg.
    declension!("ἀβατον", 3, "~α~ον", [gender: Neuter], like Ἀστραγᾰλος;
    nom: "~ᾰ~ον", _, "~ᾰ~ᾰ";
    acc: _, _, "~ᾰ~ᾰ";
    voc: "~ᾰ~ον", _, _;
    ),
    // Ἐγκᾰτᾰ: 3 words (ἐγκᾰτᾰ, κεδμᾰτᾰ, ἰχθῠημᾰτᾰ). Like Τρωες, except in 4 cells: nom sg, nom pl, gen_ pl, acc pl.
    declension!("ἐγκᾰτᾰ", 3, "~τᾰ", [gender: Neuter], like Τρωες;
    nom: "~τᾰ", _, "~τᾰ";
    gen_: _, _, "~των";
    acc: _, _, "~τᾰ";
    ),
    // Ἰασπις: 3 words (ἰασπις, ἰσατις, ἀγλαοφωτις). Like Ἀκανθις, except in 1 cell: acc sg.
    declension!("ἰασπις", 3, "~α~ις", [gender: Feminine], like Ἀκανθις;
    acc: "~ᾰ~ῐν", _, _;
    ),
    // Ὑπαλλαγη: 3 words (ἀνταλλαγη, ἀνασκαφη, ὑπαλλαγη).
    declension!("ὑπαλλαγη", 3, "~α~α~η", [gender: Feminine];
    nom: "~ᾰ~ᾰ~η", "~ᾰ~ᾰ~ᾱ", "~ᾰ~ᾰ~αι";
    gen_: "~ᾰ~ᾰ~ης", "~ᾰ~ᾰ~αῑν", "~ᾰ~ᾰ~ων";
    dat: "~ᾰ~ᾰ~ῃ", _, "~ᾰ~ᾰ~αῑς";
    acc: "~ᾰ~ᾰ~ην", _, "~ᾰ~ᾰ~ᾱς";
    voc: "~ᾰ~ᾰ~η", _, _;
    ),
    // Λυρτος: 3 words (νεκυϊσμος, ἰχθυουλκος, λυρτος). Like Ἀμεθυστος, except in 3 cells: gen_ sg, gen_ du, dat pl.
    declension!("λυρτος", 3, "~υ~ος", [gender: Masculine], like Ἀμεθυστος;
    gen_: "~ῠ~οῡ", "~ῠ~οῑν", _;
    dat: _, _, "~ῠ~οῑς";
    ),
    // Δᾰσῠ: 3 words (ὀξῠ, δᾰσῠ, πλᾰτῠ). Like Φᾰος, except in 5 cells: nom sg, nom du, dat sg, acc sg, voc sg.
    declension!("δᾰσῠ", 3, "~ῠ", like Φᾰος;
    nom: "~ῠ", "~εῑ", _;
    dat: "~εῑ", _, _;
    acc: "~ῠ", _, _;
    voc: "~ῠ", _, _;
    ),
    // Λαλλαι: 3 words (κᾰλανδαι, χαρωνῑται, λαλλαι). Like Ἀραχνη, except in 7 cells: nom sg, nom du, gen_ sg, gen_ du, dat sg, acc sg, ….
    declension!("λαλλαι", 3, "~α~αι", like Ἀραχνη;
    nom: "~α~αι", derived, _;
    gen_: derived, derived, _;
    dat: derived, _, _;
    acc: derived, _, _;
    voc: derived, _, _;
    ),
    // Βαλσαμον: 3 words (ἀρτολαγανον, βαλσαμον, ἀλαβαστρον).
    declension!("βαλσαμον", 3, "~α~α~ον", [gender: Neuter];
    nom: "~ᾰ~ᾰ~ον", "~ᾰ~ᾰ~ω", "~ᾰ~ᾰ~ᾰ";
    gen_: "~ᾰ~ᾰ~ου", "~ᾰ~ᾰ~οιν", "~ᾰ~ᾰ~ων";
    dat: "~ᾰ~ᾰ~ῳ", _, "~ᾰ~ᾰ~οις";
    acc: "~ᾰ~ᾰ~ον", _, "~ᾰ~ᾰ~ᾰ";
    voc: "~ᾰ~ᾰ~ον", _, _;
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
    // Θεωσῐς: 3 words (θεωσῐς, ἀλθησῐς, ζαχαρῐς). Like Δῐαιρεσῐς, except in 6 cells: nom du, nom pl, gen_ du, dat sg, dat pl, acc pl.
    declension!("θεωσῐς", 3, "~ῐς", [gender: Feminine], like Δῐαιρεσῐς;
    nom: _, derived, "~εες";
    gen_: _, derived, _;
    dat: "~εϊ", _, "~εσῐ";
    acc: _, _, "~εᾰς";
    ),
    // Ἀσφαλισις: 3 words (ἀνακουφισις, ἀνταποκρισις, ἀσφαλισις).
    declension!("ἀσφαλισις", 3, "~α~ι~ις", [gender: Feminine];
    nom: "~ᾰ~ῐ~ῐς", "~ᾰ~ῐ~ει", "~ᾰ~ῐ~εις";
    gen_: "~ᾰ~ῐ~εως", "~ᾰ~ῐ~εοιν", "~ᾰ~ῐ~εων";
    dat: "~ᾰ~ῐ~ει", _, "~ᾰ~ῐ~εσῐ|~ᾰ~ῐ~εσῐν";
    acc: "~ᾰ~ῐ~ῐν", _, "~ᾰ~ῐ~εις";
    voc: "~ᾰ~ῐ~ῐ", _, _;
    ),
];
