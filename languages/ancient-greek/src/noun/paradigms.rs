//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first. A `%`
//! is the first letter of the stem piece after it (reduplication: `%ε~κα`).

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (`[gender: Neuter]`, if
/// they share them), the declension it is like (if any), then a row of patterns
/// per line (`derived`: the form comes from another cell's, see
/// `relations.rs`; `_`, or a row left out: the form is the declension's it is like,
/// or, in a declension like no other, derived):
/// `nominative` (sg, du, pl)
/// `genitive` (sg, du, pl)
/// `dative` (sg, du, pl)
/// `accusative` (sg, du, pl)
/// `vocative` (sg, du, pl)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, [$($f:ident: $v:ident),*])? $(, like $parent:ident)?;
     $(nominative: $c0:tt, $c1:tt, $c2:tt;)?
     $(genitive: $c3:tt, $c4:tt, $c5:tt;)?
     $(dative: $c6:tt, $c7:tt, $c8:tt;)?
     $(accusative: $c9:tt, $c10:tt, $c11:tt;)?
     $(vocative: $c12:tt, $c13:tt, $c14:tt;)?
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
        nominative: "~ος", "~ω", "~οι";
        genitive: "~ου", "~οιν", "~ων";
        dative: "~ῳ", _, "~οις";
        accusative: "~ον", _, "~ους";
        vocative: "~ε", _, _;
    ),
    // Μορῐον: 1,025 words (ἀστρον, παιδῐον, δενδρον, μῠστηρῐον, ἐργον, εἰδωλον). Like Τρᾰγος, except in 4 cells: nominative sg, nominative pl, accusative pl, vocative sg.
    declension!("μορῐον", 1025, "~ον", [gender: Neuter], like Τρᾰγος;
        nominative: "~ον", _, "~ᾰ";
        accusative: _, _, "~ᾰ";
        vocative: "~ον", _, _;
    ),
    // Οὐσῐᾱ: 546 words (δημοκρᾰτῐᾱ, χωρᾱ, μελῳδῐᾱ, φῐλῐᾱ, σοφῐᾱ, ἡγεμονῐᾱ).
    declension!("οὐσῐᾱ", 546, "~ᾱ", [gender: Feminine];
        nominative: "~ᾱ", "~ᾱ", "~αι";
        genitive: "~ᾱς", "~αιν", "~ων";
        dative: "~ᾳ", _, "~αις";
        accusative: "~ᾱν", _, "~ᾱς";
        vocative: "~ᾱ", _, _;
    ),
    // Θεμᾰ: 531 words (μελῐ, γρᾰμμᾰ, δερμᾰ, σῑγμᾰ, βαπτισμᾰ, στομᾰ).
    declension!("θεμᾰ", 531, "~";
        nominative: "~", "~τε", "~τᾰ";
        genitive: "~τος", "~τοιν", "~των";
        dative: "~τῐ", _, "~σῐ|~σῐν";
        accusative: "~", _, "~τᾰ";
        vocative: "~", _, _;
    ),
    // Νῠμφη: 518 words (νῑκη, δῑνη, κορη, γῠνη, ἐλεημοσῠνη, εἰρηνη). Like Οὐσῐᾱ, except in 5 cells: nominative sg, genitive sg, dative sg, accusative sg, vocative sg.
    declension!("νῠμφη", 518, "~η", [gender: Feminine], like Οὐσῐᾱ;
        nominative: "~η", _, _;
        genitive: "~ης", _, _;
        dative: "~ῃ", _, _;
        accusative: "~ην", _, _;
        vocative: "~η", _, _;
    ),
    // Μερῐσμος: 483 words (λαιμος, μᾰστος, θεος, καιρος, νεκρος, ὀφθᾰλμος). Like Τρᾰγος, except in 3 cells: genitive sg, genitive du, dative pl.
    declension!("μερῐσμος", 483, "~ος", [gender: Masculine], like Τρᾰγος;
        genitive: "~οῡ", "~οῑν", _;
        dative: _, _, "~οῑς";
    ),
    // Δῐαιρεσῐς: 433 words (πολῐς, γνωσῐς, κῠβερνησῐς, μορφωσῐς, φῠσῐς, κρῐσῐς).
    declension!("δῐαιρεσῐς", 433, "~ῐς", [gender: Feminine];
        nominative: "~ῐς", "~ει", "~εις";
        genitive: "~εως", "~εοιν", "~εων";
        dative: "~ει", _, "~εσῐ|~εσῐν";
        accusative: "~ῐν", _, "~εις";
        vocative: "~ῐ", _, _;
    ),
    // Κᾰτᾰσκευη: 302 words (μηχᾰνη, μουσῐκη, γρᾰμμᾰτῐκη, φωνη, κεφᾰλη, ἡδονη). Like Νῠμφη, except in 2 cells: genitive du, dative pl.
    declension!("κᾰτᾰσκευη", 302, "~η", [gender: Feminine], like Νῠμφη;
        genitive: _, "~αῑν", _;
        dative: _, _, "~αῑς";
    ),
    // Γενετης: 204 words (προφητης, κλεπτης, πολῑτης, ἱπποτοξοτης, τρωγλοδῠτης, σῠβᾰρῑτης). Like Νῠμφη, except in 3 cells: nominative sg, genitive sg, vocative sg.
    declension!("γενετης", 204, "~ης", [gender: Masculine], like Νῠμφη;
        nominative: "~ης", _, _;
        genitive: "~ου", _, _;
        vocative: "~ᾰ", _, _;
    ),
    // Κερκῐς: 185 words (παῑς, λεπῐς, πᾰτρῐς, ἑλληνῐς, περσῐς, γρᾰφῐς).
    declension!("κερκῐς", 185, "~ς", [gender: Feminine];
        nominative: "~ς", "~δε", "~δες";
        genitive: "~δος", "~δοιν", "~δων";
        dative: "~δῐ", _, "~σῐ|~σῐν";
        accusative: "~δᾰ", _, "~δᾰς";
        vocative: "~", _, _;
    ),
    // Κορᾰξ: 183 words (χᾰλῐξ, πτερῠξ, γλαῡξ, ἑλῐξ, κῐλῐξ, νῠξ).
    declension!("κορᾰξ", 183, "~ξ";
        nominative: "~ξ", "~κε", "~κες";
        genitive: "~κος", "~κοιν", "~κων";
        dative: "~κῐ", _, "~ξῐ|~ξῐν";
        accusative: "~κᾰ", _, "~κᾰς";
        vocative: "~ξ", _, _;
    ),
    // Κλεις: 165 words (μῠκης, κρης, σκληροτης, μᾰρτῠς, θης, κλεις). Like Κερκῐς, except in 1 cell: vocative sg.
    declension!("κλεις", 165, "~ς", like Κερκῐς;
        vocative: "~ς", _, _;
    ),
    // Τρᾰπεζᾰ: 133 words (θᾰλᾰσσᾰ, ῥῐζᾰ, θᾰλᾰττᾰ, δοξᾰ, γλωττᾰ, μελῐττᾰ). Like Νῠμφη, except in 3 cells: nominative sg, accusative sg, vocative sg.
    declension!("τρᾰπεζᾰ", 133, "~ᾰ", [gender: Feminine], like Νῠμφη;
        nominative: "~ᾰ", _, _;
        accusative: "~ᾰν", _, _;
        vocative: "~ᾰ", _, _;
    ),
    // Ἐπῐφᾰνειᾰ: 116 words (βᾰσῐλειᾰ, ἀσθενειᾰ, μᾰχαιρᾰ, βοηθειᾰ, συγγενειᾰ, ὀρθοεπειᾰ). Like Τρᾰπεζᾰ, except in 2 cells: genitive sg, dative sg.
    declension!("ἐπῐφᾰνειᾰ", 116, "~ᾰ", [gender: Feminine], like Τρᾰπεζᾰ;
        genitive: "~ᾱς", _, _;
        dative: "~ᾳ", _, _;
    ),
    // Γενος: 109 words (σκῠλος, κλεος, ἐθνος, πληθος, τελος, στηθος).
    declension!("γενος", 109, "~ος", [gender: Neuter];
        nominative: "~ος", "~ει", "~η";
        genitive: "~ους", "~οῑν", "~ων";
        dative: "~ει", _, "~εσῐ|~εσῐν";
        accusative: "~ος", _, "~η";
        vocative: "~ος", _, _;
    ),
    // Αἰων: 96 words (ἑλλην, αἰων, λεγεων, ἀγων, σᾱπων, λᾰκων).
    declension!("αἰων", 96, "~ν", [gender: Masculine];
        nominative: "~ν", "~νε", "~νες";
        genitive: "~νος", "~νοιν", "~νων";
        dative: "~νῐ", _, "~σῐ|~σῐν";
        accusative: "~νᾰ", _, "~νᾰς";
        vocative: "~ν", _, _;
    ),
    // Χορευτης: 87 words (ἐρᾰστης, ποιητης, βᾰπτῐστης, κρῐτης, κωλοβᾰθρῐστης, σοφῐστης). Like Κᾰτᾰσκευη, except in 3 cells: nominative sg, genitive sg, vocative sg.
    declension!("χορευτης", 87, "~ης", [gender: Masculine], like Κᾰτᾰσκευη;
        nominative: "~ης", _, _;
        genitive: "~οῡ", _, _;
        vocative: "~ᾰ", _, _;
    ),
    // Ἱππευς: 81 words (βᾰσῐλευς, ἀρχιερευς, γραμμᾰτευς, αἰολευς, ἀμφισσευς, ἀντῐκυρευς).
    declension!("ἱππευς", 81, "~ευς", [gender: Masculine];
        nominative: "~ευς", "~η", "~ης|~εῑς";
        genitive: "~εως", "~εοιν", "~εων";
        dative: "~εῑ", _, "~εῡσῐ|~εῡσῐν";
        accusative: "~εᾱ", _, "~εᾱς";
        vocative: "~εῡ", _, "~ης|~εῑς";
    ),
    // Χᾰρᾰκτηρ: 72 words (μᾰρτῠρ, πᾰνθηρ, ἰβηρ, χειρ, ἐορ, κρᾱτηρ).
    declension!("χᾰρᾰκτηρ", 72, "~";
        nominative: "~", "~ε", "~ες";
        genitive: "~ος", "~οιν", "~ων";
        dative: "~ῐ", _, "~σῐ|~σῐν";
        accusative: "~ᾰ", _, "~ᾰς";
        vocative: "~", _, _;
    ),
    // Κεραιᾱ: 71 words (προφητειᾱ, πορνειᾱ, παιδειᾱ, δουλειᾱ, ξῠλειᾱ, νηστειᾱ).
    declension!("κεραιᾱ", 71, "~~ᾱ", [gender: Feminine];
        nominative: "~~ᾱ", "~~ᾱ", "~ῑα~";
        genitive: "~~ᾱς", "~~αιν", "~~ων";
        dative: "~~ᾳ", _, "~~αις";
        accusative: "~~ᾱν", _, "~~ᾱς";
        vocative: "~~ᾱ", _, _;
    ),
    // Σημεῑον: 62 words (θεῑον, πανδοχεῑον, πᾰνδοκεῑον, στοιχεῑον, ἀγγεῑον, βᾰλᾰνεῑον).
    declension!("σημεῑον", 62, "~ῑον", [gender: Neuter];
        nominative: "~ῑον", "~ιω", "~ῑᾰ";
        genitive: "~ιου", "~ιοιν", "~ιων";
        dative: "~ιῳ", _, "~ιοις";
        accusative: "~ῑον", _, "~ῑᾰ";
        vocative: "~ῑον", _, _;
    ),
    // Κᾰλπῐς: 62 words (πελασγῐωτῐς, σπᾰρτῐᾱτῐς, σῠβᾰρῑτῐς, σμῠρῐς, ἀσπρῐς, εὐνῐς). Like Κερκῐς, except in 1 cell: accusative sg.
    declension!("κᾰλπῐς", 62, "~ς", like Κερκῐς;
        accusative: "~ν", _, _;
    ),
    // Φορᾱ: 54 words (παιδῐᾱ, πᾰτρῐᾱ, γενεᾱ, θεᾱ, στρᾰτῐᾱ, σκῐᾱ). Like Οὐσῐᾱ, except in 2 cells: genitive du, dative pl.
    declension!("φορᾱ", 54, "~ᾱ", [gender: Feminine], like Οὐσῐᾱ;
        genitive: _, "~αῑν", _;
        dative: _, _, "~αῑς";
    ),
    // Δᾰμᾰλης: 53 words (βελγης, δακης, μῐνῠης, ὀδρῠσης, πῐσῐδης, σᾰκης). Like Γενετης, except in 1 cell: vocative sg.
    declension!("δᾰμᾰλης", 53, "~ης", [gender: Masculine], like Γενετης;
        vocative: "~η", _, _;
    ),
    // Κοχλῐᾱς: 49 words (νεᾱνῐᾱς, γᾰλᾰξῐᾱς, καικιᾱς, κῠβᾱς, τᾰμῐᾱς, κοχλῐᾱς). Like Οὐσῐᾱ, except in 2 cells: nominative sg, genitive sg.
    declension!("κοχλῐᾱς", 49, "~ᾱς", [gender: Masculine], like Οὐσῐᾱ;
        nominative: "~ᾱς", _, _;
        genitive: "~ου", _, _;
    ),
    // Ὀργῐᾰ: 49 words (κᾰστᾰνειᾰ, κᾰστᾰνᾰ, ὀργῐᾰ, γαρσᾰνᾰ, αἱρεσῐᾰ, δῐονῡσῐᾰ). Like Μορῐον, except in 7 cells: nominative sg, nominative du, genitive sg, genitive du, dative sg, accusative sg, ….
    declension!("ὀργῐᾰ", 49, "~ᾰ", [gender: Neuter], like Μορῐον;
        nominative: "~ᾰ", derived, _;
        genitive: derived, derived, _;
        dative: derived, _, _;
        accusative: derived, _, _;
        vocative: derived, _, _;
    ),
    // Ζῠγον: 45 words (πτερον, ὀνομᾰστῐκον, ᾠον, ἑρπετον, ζῠγον, φῠτον). Like Μορῐον, except in 3 cells: genitive sg, genitive du, dative pl.
    declension!("ζῠγον", 45, "~ον", [gender: Neuter], like Μορῐον;
        genitive: "~οῡ", "~οῑν", _;
        dative: _, _, "~οῑς";
    ),
    // Φᾰος: 40 words (δᾰσος, πεος, σκᾰφος, φᾰος, πᾰχος, κᾰμπος). Like Γενος, except in 7 cells: nominative du, nominative pl, genitive sg, genitive du, genitive pl, dative sg, ….
    declension!("φᾰος", 40, "~ος", [gender: Neuter], like Γενος;
        nominative: _, "~εε", "~εᾰ";
        genitive: "~εος", "~εοιν", "~εων";
        dative: "~εϊ", _, _;
        accusative: _, _, "~εᾰ";
    ),
    // Ὀλῐνοι: 40 words (πᾰρᾰβολᾱνοι, ἰσοι, μοσσῡνοικοι, ἡνῐοχοι, σῐνδοι, ἰστροι). Like Ὀργῐᾰ, except in 3 cells: nominative sg, nominative pl, accusative pl.
    declension!("ὀλῐνοι", 40, "~οι", [gender: Masculine], like Ὀργῐᾰ;
        nominative: "~οι", _, "~οι";
        accusative: _, _, "~ους";
    ),
    // Κροσσαι: 39 words (ἑβδομηκοντᾰκωμηται, τεσσᾰρεσκαιδεκᾰτῑται, ὀᾰσῑται, ἰσκαι, κρουπεζαι, ὀζολαι). Like Γενετης, except in 0 cells: .
    declension!("κροσσαι", 39, "~αι", like Γενετης;
    ),
    // Κᾰνων: 34 words (ἡγεμων, τεκτων, μᾰκεδων, μαιων, μῠρμῐδων, παιων).
    declension!("κᾰνων", 34, "~ων";
        nominative: "~ων", "~ονε", "~ονες";
        genitive: "~ονος", "~ονοιν", "~ονων";
        dative: "~ονῐ", _, "~οσῐ|~οσῐν";
        accusative: "~ονᾰ", _, "~ονᾰς";
        vocative: "~ων", _, _;
    ),
    // Ῥητωρ: 33 words (κῠων, ἀλεκτωρ, καστωρ, ῥητωρ, δωτωρ, στατωρ).
    declension!("ῥητωρ", 33, "~ω~";
        nominative: "~ω~", "~ο~ε", "~ο~ες";
        genitive: "~ο~ος", "~ο~οιν", "~ο~ων";
        dative: "~ο~ῐ", _, "~ο~σῐ|~ο~σῐν";
        accusative: "~ο~ᾰ", _, "~ο~ᾰς";
        vocative: "~ο~", _, _;
    ),
    // Κολλοψ: 32 words (αἰθῐοψ, κῠκλωψ, δολοψ, αἰγῐλωψ, ἀγχιλωψ, ὀψ).
    declension!("κολλοψ", 32, "~ψ";
        nominative: "~ψ", "~πε", "~πες";
        genitive: "~πος", "~ποιν", "~πων";
        dative: "~πῐ", _, "~ψῐ|~ψῐν";
        accusative: "~πᾰ", _, "~πᾰς";
        vocative: "~ψ", _, _;
    ),
    // Ἀμβροσια: 30 words (ἱστορια, ἰδια, ἀπορια, ἀρτηρια, ἀμνηστια, ἀμβροσια).
    declension!("ἀμβροσια", 30, "~ια", [gender: Feminine];
        nominative: "~ῐᾱ", "~ῐᾱ", "~ῐαι";
        genitive: "~ῐᾱς", "~ῐαιν", "~ῐων";
        dative: "~ῐᾳ", _, "~ῐαις";
        accusative: "~ῐᾱν", _, "~ῐᾱς";
        vocative: "~ῐᾱ", _, _;
    ),
    // Στᾰχῠς: 30 words (λῐβῠς, λῐγῠς, γενῠς, ἐγχελῠς, ῥᾰπῠς, ῥᾰφῠς).
    declension!("στᾰχῠς", 30, "~ῠς";
        nominative: "~ῠς", "~ῠε|~ῡ", "~ῠες|~ῡς";
        genitive: "~ῠος", "~ῠοιν", "~ῠων";
        dative: "~ῠϊ|~υι", _, "~ῠσῐ|~ῠσῐν";
        accusative: "~ῠν", "~ῠε|~ῡ", "~ῡς|~ῠᾰς";
        vocative: "~ῠ", "~ῠε|~ῡ", "~ῠες|~ῡς";
    ),
    // Ναῡλος: 28 words (ταῡρος, δοῡλος, σαῡρος, πλοῡτος, καρδοῡχος, οὐολοῡσκος).
    declension!("ναῡλος", 28, "~ῡ~ος", [gender: Masculine];
        nominative: "~ῡ~ος", "~υ~ω", "~ῡ~οι";
        genitive: "~υ~ου", "~υ~οιν", "~υ~ων";
        dative: "~υ~ῳ", _, "~υ~οις";
        accusative: "~ῡ~ον", _, "~υ~ους";
        vocative: "~ῡ~ε", _, _;
    ),
    // Ῥωμαῑος: 26 words (ἰουδαῑος, φαρισαῑος, ῥωμαῑος, ἀνδρεῑος, ἑβραῑος, χᾰλδαῑος). Like Σημεῑον, except in 4 cells: nominative sg, nominative pl, accusative pl, vocative sg.
    declension!("ῥωμαῑος", 26, "~ῑος", [gender: Masculine], like Σημεῑον;
        nominative: "~ῑος", _, "~ῑοι";
        accusative: _, _, "~ιους";
        vocative: "~ῑε", _, _;
    ),
    // Ἀξων: 25 words (ἀρχιτεκτων, βιστων, χᾱων, λῠκᾱων, βρᾰχῑων, ἀκμων). Like Κᾰνων, except in 1 cell: vocative sg.
    declension!("ἀξων", 25, "~ων", [gender: Masculine], like Κᾰνων;
        vocative: "~ον", _, _;
    ),
    // Δᾰνᾰοι: 24 words (δᾰνᾰοι, τῐβᾰρηνοι, μεθοδῐκοι, πᾰλαιοι, ἐμπειρῐκοι, στοιχειωμᾰτῐκοι). Like Ὀλῐνοι, except in 1 cell: dative pl.
    declension!("δᾰνᾰοι", 24, "~οι", [gender: Masculine], like Ὀλῐνοι;
        dative: _, _, "~οῑς";
    ),
    // Λεων: 22 words (θερᾰπων, λεων, δρᾰκων, γερων, ὁρῐζων, χᾰμαιλεων).
    declension!("λεων", 22, "~ων", [gender: Masculine];
        nominative: "~ων", "~οντε", "~οντες";
        genitive: "~οντος", "~οντοιν", "~οντων";
        dative: "~οντῐ", _, "~ουσῐ|~ουσῐν";
        accusative: "~οντᾰ", _, "~οντᾰς";
        vocative: "~ον", _, _;
    ),
    // Σᾰρξ: 21 words (σᾰρξ, θρᾳξ, φρῠξ, λῠγξ, προιξ, σφηξ). Like Κορᾰξ, except in 1 cell: genitive du.
    declension!("σᾰρξ", 21, "~ξ", like Κορᾰξ;
        genitive: _, "~κοῑν", _;
    ),
    // Ὑποθεσις: 19 words (ὑποθεσις, ἀρσις, ὑβρις, ἀνεσις, ἀσκησις, ἀμαυρωσις). Like Δῐαιρεσῐς, except in 0 cells: .
    declension!("ὑποθεσις", 19, "~ις", [gender: Feminine], like Δῐαιρεσῐς;
    ),
    // Πους: 18 words (πους, ὀκτωπους, πολυπους, τρῐπους, ἀπους, δᾰσῠπους). Like Κερκῐς, except in 2 cells: nominative sg, vocative sg.
    declension!("πους", 18, "~υς", [gender: Masculine], like Κερκῐς;
        nominative: "~υς", _, _;
        vocative: "~υς", _, _;
    ),
    // Λᾰτρῐς: 16 words (ὀκρῐς, λᾰτρῐς, μηνῐς, πορτῐς, ἀρδῐς, κᾰπετῐς).
    declension!("λᾰτρῐς", 16, "~ῐς";
        nominative: "~ῐς", "~ῐε", "~ῐες";
        genitive: "~ῐος", "~ῐοιν", "~ῐων";
        dative: "~ῑ", _, "~ῐσῐ|~ῐσῐν";
        accusative: "~ῐν", _, "~ῑς|~ῐᾰς";
        vocative: "~ῐ", _, _;
    ),
    // Ἀστραγᾰλος: 15 words (ἰταλος, ἰθακος, ὑμνογραφος, ἱπποποταμος, ἰαμβος, ἀρακος).
    declension!("ἀστραγᾰλος", 15, "~α~ος";
        nominative: "~ᾰ~ος", "~ᾰ~ω", "~ᾰ~οι";
        genitive: "~ᾰ~ου", "~ᾰ~οιν", "~ᾰ~ων";
        dative: "~ᾰ~ῳ", _, "~ᾰ~οις";
        accusative: "~ᾰ~ον", _, "~ᾰ~ους";
        vocative: "~ᾰ~ε", _, _;
    ),
    // Περᾰς: 15 words (φως, τερᾰς, χᾰρῐς, κρεᾰς, πᾰγκρεᾰς, ἐρῠσῐπελᾰς). Like Θεμᾰ, except in 3 cells: nominative sg, accusative sg, vocative sg.
    declension!("περᾰς", 15, "~ς", like Θεμᾰ;
        nominative: "~ς", _, _;
        accusative: "~ς", _, _;
        vocative: "~ς", _, _;
    ),
    // Πνεῡμᾰ: 14 words (πνεῡμᾰ, καῡμᾰ, ῥεῡμᾰ, θαῡμᾰ, ζεῡγμᾰ, τραῡμᾰ).
    declension!("πνεῡμᾰ", 14, "~ῡ~", [gender: Neuter];
        nominative: "~ῡ~", "~υ~τε", "~υ~τᾰ";
        genitive: "~υ~τος", "~υ~τοιν", "~υ~των";
        dative: "~υ~τῐ", _, "~υ~σῐ|~υ~σῐν";
        accusative: "~ῡ~", _, "~υ~τᾰ";
        vocative: "~ῡ~", _, _;
    ),
    // Ἰκρῐα: 14 words (γενεσια, μουστακια, σαγματοπασμαγαδια, εὐληρα, βρῡτεα, ἰσα). Like Ὀργῐᾰ, except in 1 cell: nominative sg.
    declension!("ἰκρῐα", 14, "~α", [gender: Neuter], like Ὀργῐᾰ;
        nominative: "~α", _, _;
    ),
    // Νηδῡς: 14 words (βοῡς, δελφῡς, ὀφρῡς, γρᾱῡς, λῐγνῡς, ἐδητῡς). Like Στᾰχῠς, except in 4 cells: nominative sg, dative sg, accusative sg, vocative sg.
    declension!("νηδῡς", 14, "~ῡς", like Στᾰχῠς;
        nominative: "~ῡς", _, _;
        dative: "~ῠϊ|~υῑ", _, _;
        accusative: "~ῡν", _, _;
        vocative: "~ῡ", _, _;
    ),
    // Σηψ: 13 words (ὠψ, γρῡψ, θρῑψ, θωψ, κλωψ, σκωψ). Like Κολλοψ, except in 1 cell: genitive du.
    declension!("σηψ", 13, "~ψ", like Κολλοψ;
        genitive: _, "~ποῑν", _;
    ),
    // Ἀποστασις: 13 words (ὑπαρξις, ἀναβρωσις, ἀποταξις, ὑποταξις, ὑπερταξις, ἀναχωρησις).
    declension!("ἀποστασις", 13, "~α~ις", [gender: Feminine];
        nominative: "~ᾰ~ῐς", "~ᾰ~ει", "~ᾰ~εις";
        genitive: "~ᾰ~εως", "~ᾰ~εοιν", "~ᾰ~εων";
        dative: "~ᾰ~ει", _, "~ᾰ~εσῐ|~ᾰ~εσῐν";
        accusative: "~ᾰ~ῐν", _, "~ᾰ~εις";
        vocative: "~ᾰ~ῐ", _, _;
    ),
    // Ἀσαῑοι: 13 words (βοττῐαῑοι, ἀραμαῑοι, ἀρδιαῑοι, ἀσαῑοι, θᾰμᾰναῑοι, νῐψαῑοι). Like Ῥωμαῑος, except in 0 cells: .
    declension!("ἀσαῑοι", 13, "~ῑοι", [gender: Masculine], like Ῥωμαῑος;
    ),
    // Ἰδεα: 12 words (ἰδεα, ἀκρα, ὑδρα, ἀνθρωποποιϊα, ἰτεα, ὑστερα). Like Οὐσῐᾱ, except in 0 cells: .
    declension!("ἰδεα", 12, "~α", [gender: Feminine], like Οὐσῐᾱ;
    ),
    // Ἁλως: 12 words (νεως, γᾰλως, τᾰως, λᾰγως, ἑως, λεως).
    declension!("ἁλως", 12, "~ως";
        nominative: "~ως", "~ω", "~ῳ";
        genitive: "~ω", "~ῳν", "~ων";
        dative: "~ῳ", _, "~ῳς";
        accusative: "~ω|~ων", _, "~ως";
        vocative: "~ως", _, _;
    ),
    // Κηρ: 12 words (κηρ, ἐᾰρ, θενᾰρ, ὀνᾰρ, νεκτᾰρ, μελᾰν). Like Χᾰρᾰκτηρ, except in 3 cells: nominative pl, accusative sg, accusative pl.
    declension!("κηρ", 12, "~", [gender: Neuter], like Χᾰρᾰκτηρ;
        nominative: _, _, "~ᾰ";
        accusative: "~", _, "~ᾰ";
    ),
    // Ῥως: 12 words (ἑξηκοντᾰ, ὀσσε, ὑφεν, ῥᾰββῑ, ῥᾰββονῑ, ῥᾰββουνῑ).
    declension!("ῥως", 12, "~";
        nominative: "~", "~", "~";
        genitive: "~", "~", "~";
        dative: "~", _, "~";
        accusative: "~", _, "~";
        vocative: "~", _, _;
    ),
    // Στοῑχος: 12 words (χοῑρος, ἑταῑρος, στοῑχος, τοῑχος, σχοῑνος, ῥοῑζος).
    declension!("στοῑχος", 12, "~ῑ~ος", [gender: Masculine];
        nominative: "~ῑ~ος", "~ι~ω", "~ῑ~οι";
        genitive: "~ι~ου", "~ι~οιν", "~ι~ων";
        dative: "~ι~ῳ", _, "~ι~οις";
        accusative: "~ῑ~ον", _, "~ι~ους";
        vocative: "~ῑ~ε", _, _;
    ),
    // Ἀλωπεκιᾱ: 12 words (ὑποψιᾱ, ἀβλεψιᾱ, ἀκηδιᾱ, ἀτροφιᾱ, ἀναιμιᾱ, ἀλωπεκιᾱ). Like Ἀμβροσια, except in 0 cells: .
    declension!("ἀλωπεκιᾱ", 12, "~ιᾱ", [gender: Feminine], like Ἀμβροσια;
    ),
    // Πτερῠξ: 11 words (λᾰτᾰξ, ὀρτῠξ, μᾰστῑξ, τεττῑξ, λᾰλᾰξ, πομφολῠξ).
    declension!("πτερῠξ", 11, "~ξ";
        nominative: "~ξ", "~γε", "~γες";
        genitive: "~γος", "~γοιν", "~γων";
        dative: "~γῐ", _, "~ξῐ|~ξῐν";
        accusative: "~γᾰ", _, "~γᾰς";
        vocative: "~ξ", _, _;
    ),
    // Ἀραχνη: 11 words (ἀγαπη, ἀπατη, ἀναγκη, ἀραχνη, ἀγκαλη, ὑπογεγραμμενη).
    declension!("ἀραχνη", 11, "~α~η", [gender: Feminine];
        nominative: "~ᾰ~η", "~ᾰ~ᾱ", "~ᾰ~αι";
        genitive: "~ᾰ~ης", "~ᾰ~αιν", "~ᾰ~ων";
        dative: "~ᾰ~ῃ", _, "~ᾰ~αις";
        accusative: "~ᾰ~ην", _, "~ᾰ~ᾱς";
        vocative: "~ᾰ~η", _, _;
    ),
    // Τρωες: 11 words (πῑερες, τρωες, κηφηνες, ἁλτηρες, αἰγες, ἀνᾰξῠρῐδες). Like Ὀργῐᾰ, except in 4 cells: nominative sg, nominative pl, dative pl, accusative pl.
    declension!("τρωες", 11, "~ες", like Ὀργῐᾰ;
        nominative: "~ες", _, "~ες";
        dative: _, _, "~σῐ|~σῐν";
        accusative: _, _, "~ᾰς";
    ),
    // Ὑπομνημα: 11 words (ὑποστρωμα, ὑπομνημα, ὑποδημα, ἀγνοημα, ἁρμα, ἁλμα).
    declension!("ὑπομνημα", 11, "~α", [gender: Neuter];
        nominative: "~ᾰ", "~ᾰτε", "~ᾰτᾰ";
        genitive: "~ᾰτος", "~ᾰτοιν", "~ᾰτων";
        dative: "~ᾰτῐ", _, "~ᾰσῐ|~ᾰσῐν";
        accusative: "~ᾰ", _, "~ᾰτᾰ";
        vocative: "~ᾰ", _, _;
    ),
    // Ἀντονομασια: 10 words (ἀνθρακια, ἀναρχια, ἁμαρτια, ἱπποκρατια, ἀντονομασια, ἁρματοδρομια).
    declension!("ἀντονομασια", 10, "~α~ια", [gender: Feminine];
        nominative: "~ᾰ~ῐᾱ", "~ᾰ~ῐᾱ", "~ᾰ~ῐαι";
        genitive: "~ᾰ~ῐᾱς", "~ᾰ~ῐαιν", "~ᾰ~ῐων";
        dative: "~ᾰ~ῐᾳ", _, "~ᾰ~ῐαις";
        accusative: "~ᾰ~ῐᾱν", _, "~ᾰ~ῐᾱς";
        vocative: "~ᾰ~ῐᾱ", _, _;
    ),
    // Ἰδιον: 10 words (ἰδιον, ἰσχιον, ἀγγουριον, ἱστιον, ἰπνιον, ἰνιον).
    declension!("ἰδιον", 10, "~ιον", [gender: Neuter];
        nominative: "~ῐον", "~ῐω", "~ῐᾰ";
        genitive: "~ῐου", "~ῐοιν", "~ῐων";
        dative: "~ῐῳ", _, "~ῐοις";
        accusative: "~ῐον", _, "~ῐᾰ";
        vocative: "~ῐον", _, _;
    ),
    // Ἱεραρχης: 10 words (σαϊτης, ἰλαρχης, ἱλαστης, ἱεραρχης, ἀναλεκτης, ἀκροβατης). Like Ἀραχνη, except in 3 cells: nominative sg, genitive sg, vocative sg.
    declension!("ἱεραρχης", 10, "~α~ης", like Ἀραχνη;
        nominative: "~ᾰ~ης", _, _;
        genitive: "~ᾰ~ου", _, _;
        vocative: "~ᾰ~ᾰ", _, _;
    ),
    // Λευκη: 10 words (πευκη, δουλη, βλαυτη, σενδουκη, λευκη, ζευγλη).
    declension!("λευκη", 10, "~υ~η", [gender: Feminine];
        nominative: "~υ~η", "~υ~ᾱ", "~ῡ~αι";
        genitive: "~υ~ης", "~υ~αιν", "~υ~ων";
        dative: "~υ~ῃ", _, "~υ~αις";
        accusative: "~υ~ην", _, "~υ~ᾱς";
        vocative: "~υ~η", _, _;
    ),
    // Ἀκανθα: 10 words (ἁμαξα, ἱλαειρα, ἀκανθα, ὑπατεια, ἀσφαλεια, ἀγαθοποιϊα). Like Ἱεραρχης, except in 4 cells: nominative sg, genitive sg, dative sg, accusative sg.
    declension!("ἀκανθα", 10, "~α~α", [gender: Feminine], like Ἱεραρχης;
        nominative: "~ᾰ~ᾰ", _, _;
        genitive: "~ᾰ~ᾱς", _, _;
        dative: "~ᾰ~ᾳ", _, _;
        accusative: "~ᾰ~ᾰν", _, _;
    ),
    // Πεπερῐ: 9 words (σῐνᾱπῐ, κομμῐ, πεπερῐ, κιννᾰβᾰρῐ, στῐμμῐ, κῐκῐ). Like Δῐαιρεσῐς, except in 5 cells: nominative sg, nominative pl, accusative sg, accusative pl, vocative pl.
    declension!("πεπερῐ", 9, "~ῐ", [gender: Neuter], like Δῐαιρεσῐς;
        nominative: "~ῐ", _, "~η|~εᾰ";
        accusative: "~ῐ", _, "~η|~εᾰ";
        vocative: _, _, "~η|~εᾰ";
    ),
    // Φρην: 9 words (φρην, ὑμην, ποιμην, πῠθμην, ἀδην, αὐχην).
    declension!("φρην", 9, "~ην";
        nominative: "~ην", "~ενε", "~ενες";
        genitive: "~ενος", "~ενοιν", "~ενων";
        dative: "~ενῐ", _, "~εσῐ|~εσῐν";
        accusative: "~ενᾰ", _, "~ενᾰς";
        vocative: "~ην", _, _;
    ),
    // Κοιτη: 9 words (χαιτη, κοιτη, ἐρεικη, θοινη, βαιτη, μαινη).
    declension!("κοιτη", 9, "~ι~η", [gender: Feminine];
        nominative: "~ι~η", "~ι~ᾱ", "~ῑ~αι";
        genitive: "~ι~ης", "~ι~αιν", "~ι~ων";
        dative: "~ι~ῃ", _, "~ι~αις";
        accusative: "~ι~ην", _, "~ι~ᾱς";
        vocative: "~ι~η", _, _;
    ),
    // Θηρ: 9 words (κᾱρ, θηρ, ψᾱρ, φωρ, σηρ, φηρ). Like Χᾰρᾰκτηρ, except in 1 cell: genitive du.
    declension!("θηρ", 9, "~", like Χᾰρᾰκτηρ;
        genitive: _, "~οῑν", _;
    ),
    // Ἀλειτης: 9 words (ὠρειτης, παικτης, σωρειτης, σᾰμᾰρειτης, ἀρσενοκοιτης, μητροκοιτης). Like Κοιτη, except in 3 cells: nominative sg, genitive sg, vocative sg.
    declension!("ἀλειτης", 9, "~ι~ης", [gender: Masculine], like Κοιτη;
        nominative: "~ι~ης", _, _;
        genitive: "~ι~ου", _, _;
        vocative: "~ῑ~ᾰ", _, _;
    ),
    // Κωμῡς: 9 words (ἑλμῐνς, ὀρνῑς, κωμῡς, κορῠς, γελγῑς, δελλῑς).
    declension!("κωμῡς", 9, "~ς";
        nominative: "~ς", "~θε", "~θες";
        genitive: "~θος", "~θοιν", "~θων";
        dative: "~θῐ", _, "~σῐ|~σῐν";
        accusative: "~θᾰ", _, "~θᾰς";
        vocative: "~ς", _, _;
    ),
    // Πλᾰτεῑᾰ: 9 words (μυῑᾰ, πλᾰτεῑᾰ, γαῑᾰ, μαῑᾰ, βᾰρεῑᾰ, ὀξεῑᾰ).
    declension!("πλᾰτεῑᾰ", 9, "~ῑᾰ", [gender: Feminine];
        nominative: "~ῑᾰ", "~ιᾱ", "~ῑαι";
        genitive: "~ιᾱς", "~ιαιν", "~ιων";
        dative: "~ιᾳ", _, "~ιαις";
        accusative: "~ῑᾰν", _, "~ιᾱς";
        vocative: "~ῑᾰ", _, _;
    ),
    // Ἀσπις: 9 words (ἀσπις, ἀγρωστις, ἀκις, ἁλουργις, ἱππουρις, ἀμις).
    declension!("ἀσπις", 9, "~ις", [gender: Feminine];
        nominative: "~ῐς", "~ῐδε", "~ῐδες";
        genitive: "~ῐδος", "~ῐδοιν", "~ῐδων";
        dative: "~ῐδῐ", _, "~ῐσῐ|~ῐσῐν";
        accusative: "~ῐδᾰ", _, "~ῐδᾰς";
        vocative: "~ῐ", _, _;
    ),
    // Σησᾰμοειδες: 9 words (νευροειδες, μονοειδες, τρῐχομᾰνες, ἡμεροκᾰλλες, σησᾰμοειδες, ὠκῐμοειδες). Like Γενος, except in 6 cells: nominative sg, nominative du, genitive sg, dative sg, accusative sg, vocative sg.
    declension!("σησᾰμοειδες", 9, "~ες", [gender: Neuter], like Γενος;
        nominative: "~ες", "~εῑ", _;
        genitive: "~οῡς", _, _;
        dative: "~εῑ", _, _;
        accusative: "~ες", _, _;
        vocative: "~ες", _, _;
    ),
    // Γερᾰς: 9 words (ἁλς, ὑς, ὀϊς, γερᾰς, δεπᾰς, ψεφᾰς). Like Κηρ, except in 5 cells: nominative sg, dative sg, dative pl, accusative sg, vocative sg.
    declension!("γερᾰς", 9, "~ς", like Κηρ;
        nominative: "~ς", _, _;
        dative: "~ϊ", _, "~σῐ";
        accusative: "~ς", _, _;
        vocative: "~ς", _, _;
    ),
    // Νεῡρον: 8 words (προῡμνον, νεῡρον, βοῡστον, τεῡτλον, σοῡσον, λοῡσσον). Like Ναῡλος, except in 4 cells: nominative sg, nominative pl, accusative pl, vocative sg.
    declension!("νεῡρον", 8, "~ῡ~ον", [gender: Neuter], like Ναῡλος;
        nominative: "~ῡ~ον", _, "~ῡ~ᾰ";
        accusative: _, _, "~ῡ~ᾰ";
        vocative: "~ῡ~ον", _, _;
    ),
    // Ἀμεθυστος: 8 words (ἀργυρος, δρυτομος, ἀνδρογυνος, ἰθυφαλλος, ἀμεθυστος, ἀργυροτοξος).
    declension!("ἀμεθυστος", 8, "~υ~ος";
        nominative: "~ῠ~ος", "~ῠ~ω", "~ῠ~οι";
        genitive: "~ῠ~ου", "~ῠ~οιν", "~ῠ~ων";
        dative: "~ῠ~ῳ", _, "~ῠ~οις";
        accusative: "~ῠ~ον", _, "~ῠ~ους";
        vocative: "~ῠ~ε", _, _;
    ),
    // Ἀναθεμα: 8 words (ἁμαρτημα, ἱλασμα, ἀγαλμα, ἁρπαγμα, ἀναθεμα, ὑφασμα).
    declension!("ἀναθεμα", 8, "~α~α", [gender: Neuter];
        nominative: "~ᾰ~ᾰ", "~ᾰ~ᾰτε", "~ᾰ~ᾰτᾰ";
        genitive: "~ᾰ~ᾰτος", "~ᾰ~ᾰτοιν", "~ᾰ~ᾰτων";
        dative: "~ᾰ~ᾰτῐ", _, "~ᾰ~ᾰσῐ|~ᾰ~ᾰσῐν";
        accusative: "~ᾰ~ᾰ", _, "~ᾰ~ᾰτᾰ";
        vocative: "~ᾰ~ᾰ", _, _;
    ),
    // Ῥᾱξ: 7 words (φλοξ, αἰξ, σφᾰξ, ῥᾱξ, στρῐξ, ῥωξ). Like Πτερῠξ, except in 1 cell: genitive du.
    declension!("ῥᾱξ", 7, "~ξ", like Πτερῠξ;
        genitive: _, "~γοῑν", _;
    ),
    // Ἀληθεια: 7 words (ἀληθεια, ἀγνοια, ἀμελεια, ἀσεβεια, ἀρουρα, ἀδεια). Like Ἐπῐφᾰνειᾰ, except in 0 cells: .
    declension!("ἀληθεια", 7, "~α", [gender: Feminine], like Ἐπῐφᾰνειᾰ;
    ),
    // Ἰδιωτης: 7 words (ὑποκριτης, ἀγωνιστης, ἰδιωτης, ἀκοντιστης, ὑψιβρεμετης, ἰᾰμιδης).
    declension!("ἰδιωτης", 7, "~ι~ης";
        nominative: "~ῐ~ης", "~ῐ~ᾱ", "~ῐ~αι";
        genitive: "~ῐ~οῡ", "~ῐ~αῑν", "~ῐ~ων";
        dative: "~ῐ~ῃ", _, "~ῐ~αῑς";
        accusative: "~ῐ~ην", _, "~ῐ~ᾱς";
        vocative: "~ῐ~ᾰ", _, _;
    ),
    // Ἀδᾰμᾱς: 7 words (κορῠβᾱς, γῐγᾱς, ἐλεφᾱς, ἀδᾰμᾱς, λῠκᾰβᾱς, ἀλῐβᾱς).
    declension!("ἀδᾰμᾱς", 7, "~ᾱς", [gender: Masculine];
        nominative: "~ᾱς", "~ᾰντε", "~ᾰντες";
        genitive: "~ᾰντος", "~ᾰντοιν", "~ᾰντων";
        dative: "~ᾰντῐ", _, "~ᾱσῐ|~ᾱσῐν";
        accusative: "~ᾰντᾰ", _, "~ᾰντᾰς";
        vocative: "~ᾰν", _, _;
    ),
    // Ἠχω: 7 words (ἠχω, τυτω, κῐττω, πῑπω, μοτω, φειδω). Like Μερῐσμος, except in 5 cells: nominative sg, genitive sg, dative sg, accusative sg, vocative sg.
    declension!("ἠχω", 7, "~ω", [gender: Feminine], like Μερῐσμος;
        nominative: "~ω", _, _;
        genitive: "~οῡς", _, _;
        dative: "~οῑ", _, _;
        accusative: "~ω", _, _;
        vocative: "~οῑ", _, _;
    ),
    // Μωλῠ: 7 words (δᾰκρῠ, μωλῠ, νᾱπῠ, μῐσῠ, βρᾰθῠ, σωρῠ). Like Στᾰχῠς, except in 5 cells: nominative sg, nominative pl, accusative sg, accusative pl, vocative pl.
    declension!("μωλῠ", 7, "~ῠ", [gender: Neuter], like Στᾰχῠς;
        nominative: "~ῠ", _, "~ῠᾰ";
        accusative: "~ῠ", _, "~ῠᾰ";
        vocative: _, _, derived;
    ),
    // Χεῑμᾰ: 7 words (χεῑμᾰ, δεῑγμᾰ, σεῑσμᾰ, λεῑμμᾰ, λαῑτμᾰ, πεῑσμᾰ).
    declension!("χεῑμᾰ", 7, "~ῑ~", [gender: Neuter];
        nominative: "~ῑ~", "~ι~τε", "~ι~τᾰ";
        genitive: "~ι~τος", "~ι~τοιν", "~ι~των";
        dative: "~ι~τῐ", _, "~ι~σῐ|~ι~σῐν";
        accusative: "~ῑ~", _, "~ι~τᾰ";
        vocative: "~ῑ~", _, _;
    ),
    // Ναυτης: 7 words (ναυτης, θραυστης, κοκκοθραυστης, ἀργοναυτης, ἐμφῠτευτης, ψευστης). Like Λευκη, except in 3 cells: nominative sg, genitive sg, vocative sg.
    declension!("ναυτης", 7, "~υ~ης", [gender: Masculine], like Λευκη;
        nominative: "~υ~ης", _, _;
        genitive: "~υ~ου", _, _;
        vocative: "~ῡ~ᾰ", _, _;
    ),
    // Ἀναγωγη: 7 words (ἀστραπη, ἀνατροπη, ἀνατροφη, ἀναγωγη, ἀναστροφη, ἀναβολη). Like Ἀραχνη, except in 2 cells: genitive du, dative pl.
    declension!("ἀναγωγη", 7, "~α~η", [gender: Feminine], like Ἀραχνη;
        genitive: _, "~ᾰ~αῑν", _;
        dative: _, _, "~ᾰ~αῑς";
    ),
    // Βᾰσκᾱς: 7 words (πᾰξᾰμᾱς, μᾰρῐκᾱς, ἀττᾰγᾱς, μᾰσουχᾱς, βᾰσκᾱς, χαλιφᾱς). Like Φορᾱ, except in 3 cells: nominative sg, nominative pl, genitive sg.
    declension!("βᾰσκᾱς", 7, "~ᾱς", [gender: Masculine], like Φορᾱ;
        nominative: "~ᾱς", _, "~αῑ";
        genitive: "~ᾱ", _, _;
    ),
    // Πᾰτηρ: 6 words (ἀνηρ, μητηρ, πᾰτηρ, ἀστηρ, θῠγᾰτηρ, ἐνᾰτηρ).
    declension!("πᾰτηρ", 6, "~η~";
        nominative: "~η~", "~ε~ε", "~ε~ες";
        genitive: "~~ος", "~ε~οιν", "~ε~ων";
        dative: "~~ῐ", _, "~~ᾰσῐ";
        accusative: "~ε~ᾰ", _, "~ε~ᾰς";
        vocative: "~ε~", _, _;
    ),
    // Ἰλυς: 6 words (ἰχθυς, ἰσχυς, ἰτυς, ἀχλυς, ἀρκυς, ἰλυς). Like Νηδῡς, except in 0 cells: .
    declension!("ἰλυς", 6, "~υς", like Νηδῡς;
    ),
    // Μοῑρᾰ: 6 words (μοῑρᾰ, στεῑρᾰ, πεῑρᾰ, σφαῑρᾰ, σπεῑρᾰ, ἀγρεῑφνᾰ). Like Ἀλειτης, except in 4 cells: nominative sg, genitive sg, dative sg, accusative sg.
    declension!("μοῑρᾰ", 6, "~ῑ~ᾰ", [gender: Feminine], like Ἀλειτης;
        nominative: "~ῑ~ᾰ", _, _;
        genitive: "~ι~ᾱς", _, _;
        dative: "~ι~ᾳ", _, _;
        accusative: "~ῑ~ᾰν", _, _;
    ),
    // Σφην: 6 words (μην, χην, σπλην, κλων, σφην, ψην). Like Αἰων, except in 1 cell: genitive du.
    declension!("σφην", 6, "~ν", [gender: Masculine], like Αἰων;
        genitive: _, "~νοῑν", _;
    ),
    // Μητρως: 6 words (τρως, ἡρως, δμως, πᾰτρως, θως, μητρως).
    declension!("μητρως", 6, "~ως", [gender: Masculine];
        nominative: "~ως", "~ωε", "~ωες";
        genitive: "~ωος", "~ωοιν", "~ωων";
        dative: "~ῳ|~ωϊ", _, "~ωσῐ|~ωσῐν";
        accusative: "~ω", _, "~ωᾰς";
        vocative: "~ως", _, _;
    ),
    // Κῠᾰρ: 6 words (οὐθᾰρ, ἡπᾰρ, φρεᾱρ, στεᾱρ, δελεᾰρ, κῠᾰρ). Like Θεμᾰ, except in 3 cells: nominative sg, accusative sg, vocative sg.
    declension!("κῠᾰρ", 6, "~ρ", [gender: Neuter], like Θεμᾰ;
        nominative: "~ρ", _, _;
        accusative: "~ρ", _, _;
        vocative: "~ρ", _, _;
    ),
    // Ἀνισον: 6 words (ἀλφιτον, ἀνισον, ἀγκιστρον, ἀντιθετον, ἱστορικον, ἀριστερον).
    declension!("ἀνισον", 6, "~ι~ον", [gender: Neuter];
        nominative: "~ῐ~ον", "~ῐ~ω", "~ῐ~ᾰ";
        genitive: "~ῐ~ου", "~ῐ~οιν", "~ῐ~ων";
        dative: "~ῐ~ῳ", _, "~ῐ~οις";
        accusative: "~ῐ~ον", _, "~ῐ~ᾰ";
        vocative: "~ῐ~ον", _, _;
    ),
    // Ἀριθμος: 6 words (ἀριθμος, ἀφορισμος, ἱστορικος, ἁγνισμος, σιρος, ἀκιδωτος). Like Ἀνισον, except in 7 cells: nominative sg, nominative pl, genitive sg, genitive du, dative pl, accusative pl, ….
    declension!("ἀριθμος", 6, "~ι~ος", [gender: Masculine], like Ἀνισον;
        nominative: "~ῐ~ος", _, "~ῐ~οι";
        genitive: "~ῐ~οῡ", "~ῐ~οῑν", _;
        dative: _, _, "~ῐ~οῑς";
        accusative: _, _, "~ῐ~ους";
        vocative: "~ῐ~ε", _, _;
    ),
    // Πτῠξ: 6 words (ὀνῠξ, ψῑξ, σᾰρδονῠξ, στῐξ, βηξ, πτῠξ).
    declension!("πτῠξ", 6, "~ξ";
        nominative: "~ξ", "~χε", "~χες";
        genitive: "~χος", "~χοῑν", "~χων";
        dative: "~χῐ", _, "~ξῐ|~ξῐν";
        accusative: "~χᾰ", _, "~χᾰς";
        vocative: "~ξ", _, _;
    ),
    // Πηχῠς: 6 words (ἡμῐσῠς, υἱῠς, πελεκῠς, πρεσβῠς, πηχῠς, τρῐβρᾰχῠς). Like Δῐαιρεσῐς, except in 3 cells: nominative sg, accusative sg, vocative sg.
    declension!("πηχῠς", 6, "~ῠς", [gender: Masculine], like Δῐαιρεσῐς;
        nominative: "~ῠς", _, _;
        accusative: "~ῠν", _, _;
        vocative: "~ῠ", _, _;
    ),
    // Παῡλᾰ: 6 words (μοῡσᾰ, βοῡλλᾰ, πᾰνδοῡρᾰ, παῡλᾰ, γοῡρνᾰ, κοῡπᾰ). Like Λευκη, except in 3 cells: nominative sg, accusative sg, vocative sg.
    declension!("παῡλᾰ", 6, "~ῡ~ᾰ", [gender: Feminine], like Λευκη;
        nominative: "~ῡ~ᾰ", _, _;
        accusative: "~ῡ~ᾰν", _, _;
        vocative: "~ῡ~ᾰ", _, _;
    ),
    // Γεῑσον: 6 words (δεῑπνον, σπεῑρον, κλεῑθρον, σεῑστρον, ῥεῑθρον, γεῑσον). Like Στοῑχος, except in 4 cells: nominative sg, nominative pl, accusative pl, vocative sg.
    declension!("γεῑσον", 6, "~ῑ~ον", [gender: Neuter], like Στοῑχος;
        nominative: "~ῑ~ον", _, "~ῑ~ᾰ";
        accusative: _, _, "~ῑ~ᾰ";
        vocative: "~ῑ~ον", _, _;
    ),
    // Γλωχῑς: 6 words (ὠδῑς, γλωχῑς, στᾰμῑς, ἑρμῑς, ὀρκῡς, τελμῑς). Like Αἰων, except in 2 cells: nominative sg, vocative sg.
    declension!("γλωχῑς", 6, "~ς", like Αἰων;
        nominative: "~ς", _, _;
        vocative: "~ς", _, _;
    ),
    // Πᾰπᾱς: 6 words (πᾰπᾱς, γᾰλᾰτᾱς, κᾰδῡτᾱς, γενετᾱς, σᾰκτᾱς, πᾰππᾱς). Like Κοχλῐᾱς, except in 1 cell: vocative sg.
    declension!("πᾰπᾱς", 6, "~ᾱς", [gender: Masculine], like Κοχλῐᾱς;
        vocative: "~ᾰ", _, _;
    ),
    // Ἀναξ: 5 words (ἀνθραξ, ἀβαξ, ἀναξ, ὑραξ, ἁρπαξ).
    declension!("ἀναξ", 5, "~αξ", [gender: Masculine];
        nominative: "~ᾰξ", "~ᾰκε", "~ᾰκες";
        genitive: "~ᾰκος", "~ᾰκοιν", "~ᾰκων";
        dative: "~ᾰκῐ", _, "~ᾰξῐ|~ᾰξῐν";
        accusative: "~ᾰκᾰ", _, "~ᾰκᾰς";
        vocative: "~ᾰξ", _, _;
    ),
    // Ἀηρ: 5 words (γᾰστηρ, ἀηρ, φρᾱτηρ, αἰθηρ, ἀθηρ). Like Πᾰτηρ, except in 4 cells: genitive sg, dative sg, dative pl, vocative sg.
    declension!("ἀηρ", 5, "~η~", like Πᾰτηρ;
        genitive: "~ε~ος", _, _;
        dative: "~ε~ῐ", _, "~ε~σῐ|~ε~σῐν";
        vocative: "~η~", _, _;
    ),
    // Ἰδιος: 5 words (ἁγιος, ἰδιος, ἀνεψιος, ἁλιος, ἀπιος). Like Ἰδιον, except in 4 cells: nominative sg, nominative pl, accusative pl, vocative sg.
    declension!("ἰδιος", 5, "~ιος", [gender: Masculine], like Ἰδιον;
        nominative: "~ῐος", _, "~ῐοι";
        accusative: _, _, "~ῐους";
        vocative: "~ῐε", _, _;
    ),
    // Ἀστερισκος: 5 words (ἁλιαιετος, ἀνθρωπισκος, ἀργιλλος, ἀστερισκος, ἀγχινοος). Like Ἀριθμος, except in 3 cells: genitive sg, genitive du, dative pl.
    declension!("ἀστερισκος", 5, "~ι~ος", like Ἀριθμος;
        genitive: "~ῐ~ου", "~ῐ~οιν", _;
        dative: _, _, "~ῐ~οις";
    ),
    // Ἰσχας: 5 words (ἀρκας, ἱπποφοβας, ἁλας, ἰσχας, ἀελλας).
    declension!("ἰσχας", 5, "~ας";
        nominative: "~ᾰς", "~ᾰδε", "~ᾰδες";
        genitive: "~ᾰδος", "~ᾰδοιν", "~ᾰδων";
        dative: "~ᾰδῐ", _, "~ᾰσῐ|~ᾰσῐν";
        accusative: "~ᾰδᾰ", _, "~ᾰδᾰς";
        vocative: "~ᾰς", _, _;
    ),
    // Ἀνατασις: 5 words (ἀναστασις, ἀγανακτησις, ἀναταξις, ἀνατασις, ἀναβασις).
    declension!("ἀνατασις", 5, "~α~α~ις", [gender: Feminine];
        nominative: "~ᾰ~ᾰ~ῐς", "~ᾰ~ᾰ~ει", "~ᾰ~ᾰ~εις";
        genitive: "~ᾰ~ᾰ~εως", "~ᾰ~ᾰ~εοιν", "~ᾰ~ᾰ~εων";
        dative: "~ᾰ~ᾰ~ει", _, "~ᾰ~ᾰ~εσῐ|~ᾰ~ᾰ~εσῐν";
        accusative: "~ᾰ~ᾰ~ῐν", _, "~ᾰ~ᾰ~εις";
        vocative: "~ᾰ~ᾰ~ῐ", _, _;
    ),
    // Ἀντιθεσις: 5 words (ὑποκρισις, ἀντιθεσις, ἀντιπτωσις, ὑποκιστις, ἀφιξις).
    declension!("ἀντιθεσις", 5, "~ι~ις", [gender: Feminine];
        nominative: "~ῐ~ῐς", "~ῐ~ει", "~ῐ~εις";
        genitive: "~ῐ~εως", "~ῐ~εοιν", "~ῐ~εων";
        dative: "~ῐ~ει", _, "~ῐ~εσῐ|~ῐ~εσῐν";
        accusative: "~ῐ~ῐν", _, "~ῐ~εις";
        vocative: "~ῐ~ῐ", _, _;
    ),
    // Δαιμων: 5 words (δαιμων, μειζων, γειτων, ποτᾰμογειτων, σεισων).
    declension!("δαιμων", 5, "~ι~ων", [gender: Masculine];
        nominative: "~ι~ων", "~ι~ονε", "~ι~ονες";
        genitive: "~ι~ονος", "~ι~ονοιν", "~ι~ονων";
        dative: "~ι~ονῐ", _, "~ι~οσῐ|~ι~οσῐν";
        accusative: "~ι~ονᾰ", _, "~ι~ονᾰς";
        vocative: "~ῑ~ον", _, _;
    ),
    // Γεῡσῐς: 5 words (παῡσῐς, λοῡσῐς, ῥεῡσῐς, ζεῡξῐς, γεῡσῐς).
    declension!("γεῡσῐς", 5, "~ῡ~ῐς", [gender: Feminine];
        nominative: "~ῡ~ῐς", "~υ~ει", "~υ~εις";
        genitive: "~υ~εως", "~υ~εοιν", "~υ~εων";
        dative: "~υ~ει", _, "~υ~εσῐ|~υ~εσῐν";
        accusative: "~ῡ~ῐν", _, "~υ~εις";
        vocative: "~ῡ~ῐ", _, _;
    ),
    // Νοῡς: 5 words (νοῡς, πλοῡς, θῠγατρῐδοῡς, ῥοῡς, λουτροχοῡς).
    declension!("νοῡς", 5, "~οῡς", [gender: Masculine];
        nominative: "~οῡς", "~ω|~ω", "~οῑ";
        genitive: "~οῡ", "~οῑν", "~ων";
        dative: "~ῳ", _, "~οῑς";
        accusative: "~οῡν", "~ω|~ω", "~οῡς";
        vocative: "~οῡ", "~ω|~ω", _;
    ),
    // Λῐψ: 5 words (χᾰλῠψ, φλεψ, χερνῐψ, φᾰψ, λῐψ).
    declension!("λῐψ", 5, "~ψ";
        nominative: "~ψ", "~βε", "~βες";
        genitive: "~βος", "~βοῑν", "~βων";
        dative: "~βῐ", _, "~ψῐ|~ψῐν";
        accusative: "~βᾰ", _, "~βᾰς";
        vocative: "~ψ", _, _;
    ),
    // Ἀγκυλη: 5 words (ἀφροσυνη, ἀργυροθηκη, ὑδρομυλη, ἀγκυλη, ἱπποσυνη).
    declension!("ἀγκυλη", 5, "~υ~η", [gender: Feminine];
        nominative: "~ῠ~η", "~ῠ~ᾱ", "~ῠ~αι";
        genitive: "~ῠ~ης", "~ῠ~αιν", "~ῠ~ων";
        dative: "~ῠ~ῃ", _, "~ῠ~αις";
        accusative: "~ῠ~ην", _, "~ῠ~ᾱς";
        vocative: "~ῠ~η", _, _;
    ),
    // Τεῡχος: 5 words (τεῡχος, γλεῡκος, ψεῡδος, ζεῡγος, κεῡθος).
    declension!("τεῡχος", 5, "~ῡ~ος", [gender: Neuter];
        nominative: "~ῡ~ος", "~υ~ει", "~υ~η";
        genitive: "~υ~ους", "~υ~οῑν", "~υ~ων";
        dative: "~υ~ει", _, "~υ~εσῐ|~υ~εσῐν";
        accusative: "~ῡ~ος", _, "~υ~η";
        vocative: "~ῡ~ος", _, _;
    ),
    // Χλουνης: 5 words (τῠμβαυλης, χλουνης, σῐδευνης, χοραυλης, πρωταυλης). Like Ναυτης, except in 1 cell: vocative sg.
    declension!("χλουνης", 5, "~υ~ης", [gender: Masculine], like Ναυτης;
        vocative: "~υ~η", _, _;
    ),
    // Λεῑμᾰξ: 5 words (χοῑνῐξ, κᾰταῑτῠξ, μεῑρᾰξ, λεῑμᾰξ, βαῑβῠξ).
    declension!("λεῑμᾰξ", 5, "~ῑ~ξ";
        nominative: "~ῑ~ξ", "~ι~κε", "~ι~κες";
        genitive: "~ι~κος", "~ι~κοιν", "~ι~κων";
        dative: "~ι~κῐ", _, "~ι~ξῐ|~ι~ξῐν";
        accusative: "~ι~κᾰ", _, "~ι~κᾰς";
        vocative: "~ῑ~ξ", _, _;
    ),
    // Φῡσᾱς: 5 words (πελεκᾱς, πῡρᾰμοῡς, φῡσᾱς, πλᾰκοῡς, ἀλλᾱς).
    declension!("φῡσᾱς", 5, "~ς", [gender: Masculine];
        nominative: "~ς", "~ντε", "~ντες";
        genitive: "~ντος", "~ντοιν", "~ντων";
        dative: "~ντῐ", _, "~σῐ|~σῐν";
        accusative: "~ντᾰ", _, "~ντᾰς";
        vocative: "~ν", _, _;
    ),
    // Ἱσπανος: 4 words (ἰατρος, λαϊκος, ἱσπανος, ἀρχαγος).
    declension!("ἱσπανος", 4, "~α~ος", [gender: Masculine];
        nominative: "~ᾱ~ος", "~ᾱ~ω", "~ᾱ~οι";
        genitive: "~ᾱ~οῡ", "~ᾱ~οῑν", "~ᾱ~ων";
        dative: "~ᾱ~ῳ", _, "~ᾱ~οῑς";
        accusative: "~ᾱ~ον", _, "~ᾱ~ους";
        vocative: "~ᾱ~ε", _, _;
    ),
    // Ἁγνεια: 4 words (ἰουδαια, ἁγνεια, ἀνδρεια, ὑστεραια). Like Κεραιᾱ, except in 0 cells: .
    declension!("ἁγνεια", 4, "~~α", [gender: Feminine], like Κεραιᾱ;
    ),
    // Πῠρᾰ: 4 words (πῠρᾰ, πολῑτῐκᾰ, ἐσθλᾰ, τρωϊκᾰ). Like Ὀργῐᾰ, except in 1 cell: dative pl.
    declension!("πῠρᾰ", 4, "~ᾰ", [gender: Neuter], like Ὀργῐᾰ;
        dative: _, _, "~οῑς";
    ),
    // Λαυρᾱ: 4 words (σαυρᾱ, λαυρᾱ, κᾰρουχᾱ, βρουτζᾱ). Like Λευκη, except in 5 cells: nominative sg, genitive sg, dative sg, accusative sg, vocative sg.
    declension!("λαυρᾱ", 4, "~υ~ᾱ", [gender: Feminine], like Λευκη;
        nominative: "~υ~ᾱ", _, _;
        genitive: "~υ~ᾱς", _, _;
        dative: "~υ~ᾳ", _, _;
        accusative: "~υ~ᾱν", _, _;
        vocative: "~υ~ᾱ", _, _;
    ),
    // Ἀελλα: 4 words (ὑαινα, ὑποτεινουσα, ἀγχουσα, ἀελλα). Like Τρᾰπεζᾰ, except in 0 cells: .
    declension!("ἀελλα", 4, "~α", [gender: Feminine], like Τρᾰπεζᾰ;
    ),
    // Πλευμων: 4 words (πλευμων, πνευμων, μουσμων, ἰχνευμων).
    declension!("πλευμων", 4, "~υ~ων", [gender: Masculine];
        nominative: "~υ~ων", "~υ~ονε", "~υ~ονες";
        genitive: "~υ~ονος", "~υ~ονοιν", "~υ~ονων";
        dative: "~υ~ονῐ", _, "~υ~οσῐ|~υ~οσῐν";
        accusative: "~υ~ονᾰ", _, "~υ~ονᾰς";
        vocative: "~ῡ~ον", _, _;
    ),
    // Ἀκανθις: 4 words (ἁμαμηλις, ὑπογραφις, ἀκανθις, ἀσταφις).
    declension!("ἀκανθις", 4, "~α~ις", [gender: Feminine];
        nominative: "~ᾰ~ῐς", "~ᾰ~ῐδε", "~ᾰ~ῐδες";
        genitive: "~ᾰ~ῐδος", "~ᾰ~ῐδοιν", "~ᾰ~ῐδων";
        dative: "~ᾰ~ῐδῐ", _, "~ᾰ~ῐσῐ|~ᾰ~ῐσῐν";
        accusative: "~ᾰ~ῐδᾰ", _, "~ᾰ~ῐδᾰς";
        vocative: "~ᾰ~ῐ", _, _;
    ),
    // Ἀκροβυστια: 4 words (ἀσφυξια, ἀκροβυστια, ἀμβλυωπια, ἀσυμμετρια).
    declension!("ἀκροβυστια", 4, "~υ~ια", [gender: Feminine];
        nominative: "~ῠ~ῐᾱ", "~ῠ~ῐᾱ", "~ῠ~ῐαι";
        genitive: "~ῠ~ῐᾱς", "~ῠ~ῐαιν", "~ῠ~ῐων";
        dative: "~ῠ~ῐᾳ", _, "~ῠ~ῐαις";
        accusative: "~ῠ~ῐᾱν", _, "~ῠ~ῐᾱς";
        vocative: "~ῠ~ῐᾱ", _, _;
    ),
    // Ἱματιον: 4 words (ἱματιον, ἀκουσματιον, ἀνακτοριον, ἰναριον).
    declension!("ἱματιον", 4, "~α~ιον", [gender: Neuter];
        nominative: "~ᾰ~ῐον", "~ᾰ~ῐω", "~ᾰ~ῐᾰ";
        genitive: "~ᾰ~ῐου", "~ᾰ~ῐοιν", "~ᾰ~ῐων";
        dative: "~ᾰ~ῐῳ", _, "~ᾰ~ῐοις";
        accusative: "~ᾰ~ῐον", _, "~ᾰ~ῐᾰ";
        vocative: "~ᾰ~ῐον", _, _;
    ),
    // Ἀστυρον: 4 words (ἀσυνδετον, ἀστυρον, ὑδνοφυλλον, ἰχθυοπωλεῑον). Like Ἀμεθυστος, except in 4 cells: nominative sg, nominative pl, accusative pl, vocative sg.
    declension!("ἀστυρον", 4, "~υ~ον", [gender: Neuter], like Ἀμεθυστος;
        nominative: "~ῠ~ον", _, "~ῠ~ᾰ";
        accusative: _, _, "~ῠ~ᾰ";
        vocative: "~ῠ~ον", _, _;
    ),
    // Σικανος: 4 words (σικανος, ἀντιπατρος, ἀρχιδαμος, ἁλιβαπτος).
    declension!("σικανος", 4, "~ι~α~ος", [gender: Masculine];
        nominative: "~ῐ~ᾰ~ος", "~ῐ~ᾰ~ω", "~ῐ~ᾰ~οι";
        genitive: "~ῐ~ᾰ~ου", "~ῐ~ᾰ~οιν", "~ῐ~ᾰ~ων";
        dative: "~ῐ~ᾰ~ῳ", _, "~ῐ~ᾰ~οις";
        accusative: "~ῐ~ᾰ~ον", _, "~ῐ~ᾰ~ους";
        vocative: "~ῐ~ᾰ~ε", _, _;
    ),
    // Κηρῠξ: 4 words (κοκκῠξ, κηρῠξ, ψευδοκηρῠξ, ἱεροκηρῠξ).
    declension!("κηρῠξ", 4, "~ῠξ", [gender: Masculine];
        nominative: "~ῠξ", "~ῡκε", "~ῡκες";
        genitive: "~ῡκος", "~ῡκοιν", "~ῡκων";
        dative: "~ῡκῐ", _, "~ῡξῐ|~ῡξῐν";
        accusative: "~ῡκᾰ", _, "~ῡκᾰς";
        vocative: "~ῠξ", _, _;
    ),
    // Ζωᾱ: 4 words (γονᾱ, δρᾰχμᾱ, σφᾰγᾱ, ζωᾱ). Like Φορᾱ, except in 3 cells: genitive pl, dative pl, accusative pl.
    declension!("ζωᾱ", 4, "~ᾱ", [gender: Feminine], like Φορᾱ;
        genitive: _, _, "~ᾱων|~ᾱν";
        dative: _, _, "~αῑς|~αῑσῐ|~αῑσῐν";
        accusative: _, _, "~ᾰς";
    ),
    // Γαλη: 3 words (γη, μῡγᾰλη, γαλη). Like Κᾰτᾰσκευη, except in 1 cell: nominative pl.
    declension!("γαλη", 3, "~η", [gender: Feminine], like Κᾰτᾰσκευη;
        nominative: _, _, "~αῑ";
    ),
    // Ἀνακυκλωσις: 3 words (ἀποκαλυψις, ἀνακυκλωσις, ἀνακυκλησις).
    declension!("ἀνακυκλωσις", 3, "~α~υ~ις", [gender: Feminine];
        nominative: "~ᾰ~ῠ~ῐς", "~ᾰ~ῠ~ει", "~ᾰ~ῠ~εις";
        genitive: "~ᾰ~ῠ~εως", "~ᾰ~ῠ~εοιν", "~ᾰ~ῠ~εων";
        dative: "~ᾰ~ῠ~ει", _, "~ᾰ~ῠ~εσῐ|~ᾰ~ῠ~εσῐν";
        accusative: "~ᾰ~ῠ~ῐν", _, "~ᾰ~ῠ~εις";
        vocative: "~ᾰ~ῠ~ῐ", _, _;
    ),
    // Ἀμανιτης: 3 words (ἰσραηλιτης, ἱεραπολιτης, ἀμανιτης).
    declension!("ἀμανιτης", 3, "~α~ι~ης", [gender: Masculine];
        nominative: "~ᾱ~ῑ~ης", "~ᾱ~ῑ~ᾱ", "~ᾱ~ῑ~αι";
        genitive: "~ᾱ~ῑ~ου", "~ᾱ~ῑ~αιν", "~ᾱ~ῑ~ων";
        dative: "~ᾱ~ῑ~ῃ", _, "~ᾱ~ῑ~αις";
        accusative: "~ᾱ~ῑ~ην", _, "~ᾱ~ῑ~ᾱς";
        vocative: "~ᾱ~ῑ~ᾰ", _, _;
    ),
    // Ἀθανασια: 3 words (ἀμαζονομαχια, ἀταραξια, ἀθανασια).
    declension!("ἀθανασια", 3, "~α~α~ια", [gender: Feminine];
        nominative: "~ᾰ~ᾰ~ῐᾱ", "~ᾰ~ᾰ~ῐᾱ", "~ᾰ~ᾰ~ῐαι";
        genitive: "~ᾰ~ᾰ~ῐᾱς", "~ᾰ~ᾰ~ῐαιν", "~ᾰ~ᾰ~ῐων";
        dative: "~ᾰ~ᾰ~ῐᾳ", _, "~ᾰ~ᾰ~ῐαις";
        accusative: "~ᾰ~ᾰ~ῐᾱν", _, "~ᾰ~ᾰ~ῐᾱς";
        vocative: "~ᾰ~ᾰ~ῐᾱ", _, _;
    ),
    // Θῑς: 3 words (ῥῑς, ἰς, θῑς). Like Γλωχῑς, except in 1 cell: genitive du.
    declension!("θῑς", 3, "~ς", like Γλωχῑς;
        genitive: _, "~νοῑν", _;
    ),
    // Μῡς: 3 words (σῡς, δρῡς, μῡς). Like Νηδῡς, except in 2 cells: genitive du, dative sg.
    declension!("μῡς", 3, "~ῡς", like Νηδῡς;
        genitive: _, "~ῠοῑν", _;
        dative: "~ῠϊ|~υι", _, _;
    ),
    // Ἀγορα: 3 words (ἰα, ἀγορα, ἀρα). Like Φορᾱ, except in 0 cells: .
    declension!("ἀγορα", 3, "~α", [gender: Feminine], like Φορᾱ;
    ),
    // Νεῑκος: 3 words (τεῑχος, χεῑλος, νεῑκος).
    declension!("νεῑκος", 3, "~ῑ~ος", [gender: Neuter];
        nominative: "~ῑ~ος", "~ι~ει", "~ι~η";
        genitive: "~ι~ους", "~ι~οῑν", "~ι~ων";
        dative: "~ι~ει", _, "~ι~εσῐ|~ι~εσῐν";
        accusative: "~ῑ~ος", _, "~ι~η";
        vocative: "~ῑ~ος", _, _;
    ),
    // Ἀσις: 3 words (ἀκρις, ἀκοιτις, ἀσις). Like Λᾰτρῐς, except in 0 cells: .
    declension!("ἀσις", 3, "~ις", [gender: Feminine], like Λᾰτρῐς;
    ),
    // Πειθω: 3 words (λεχω, πειθω, κερδω). Like Ἠχω, except in 1 cell: genitive sg.
    declension!("πειθω", 3, "~ω", [gender: Feminine], like Ἠχω;
        genitive: "~οος|~οῡς", _, _;
    ),
    // Πεῑνᾰ: 3 words (πεῑσᾰ, χλαῑνᾰ, πεῑνᾰ). Like Μοῑρᾰ, except in 2 cells: genitive sg, dative sg.
    declension!("πεῑνᾰ", 3, "~ῑ~ᾰ", [gender: Feminine], like Μοῑρᾰ;
        genitive: "~ι~ης", _, _;
        dative: "~ι~ῃ", _, _;
    ),
    // Ἀβατον: 3 words (ἀνδραποδον, ἀσαρον, ἀβατον). Like Ἀστραγᾰλος, except in 4 cells: nominative sg, nominative pl, accusative pl, vocative sg.
    declension!("ἀβατον", 3, "~α~ον", [gender: Neuter], like Ἀστραγᾰλος;
        nominative: "~ᾰ~ον", _, "~ᾰ~ᾰ";
        accusative: _, _, "~ᾰ~ᾰ";
        vocative: "~ᾰ~ον", _, _;
    ),
    // Ἐγκᾰτᾰ: 3 words (ἐγκᾰτᾰ, κεδμᾰτᾰ, ἰχθῠημᾰτᾰ). Like Τρωες, except in 4 cells: nominative sg, nominative pl, genitive pl, accusative pl.
    declension!("ἐγκᾰτᾰ", 3, "~τᾰ", [gender: Neuter], like Τρωες;
        nominative: "~τᾰ", _, "~τᾰ";
        genitive: _, _, "~των";
        accusative: _, _, "~τᾰ";
    ),
    // Ἰασπις: 3 words (ἰασπις, ἰσατις, ἀγλαοφωτις). Like Ἀκανθις, except in 1 cell: accusative sg.
    declension!("ἰασπις", 3, "~α~ις", [gender: Feminine], like Ἀκανθις;
        accusative: "~ᾰ~ῐν", _, _;
    ),
    // Ὑπαλλαγη: 3 words (ἀνταλλαγη, ἀνασκαφη, ὑπαλλαγη).
    declension!("ὑπαλλαγη", 3, "~α~α~η", [gender: Feminine];
        nominative: "~ᾰ~ᾰ~η", "~ᾰ~ᾰ~ᾱ", "~ᾰ~ᾰ~αι";
        genitive: "~ᾰ~ᾰ~ης", "~ᾰ~ᾰ~αῑν", "~ᾰ~ᾰ~ων";
        dative: "~ᾰ~ᾰ~ῃ", _, "~ᾰ~ᾰ~αῑς";
        accusative: "~ᾰ~ᾰ~ην", _, "~ᾰ~ᾰ~ᾱς";
        vocative: "~ᾰ~ᾰ~η", _, _;
    ),
    // Λυρτος: 3 words (νεκυϊσμος, ἰχθυουλκος, λυρτος). Like Ἀμεθυστος, except in 3 cells: genitive sg, genitive du, dative pl.
    declension!("λυρτος", 3, "~υ~ος", [gender: Masculine], like Ἀμεθυστος;
        genitive: "~ῠ~οῡ", "~ῠ~οῑν", _;
        dative: _, _, "~ῠ~οῑς";
    ),
    // Δᾰσῠ: 3 words (ὀξῠ, δᾰσῠ, πλᾰτῠ). Like Φᾰος, except in 5 cells: nominative sg, nominative du, dative sg, accusative sg, vocative sg.
    declension!("δᾰσῠ", 3, "~ῠ", like Φᾰος;
        nominative: "~ῠ", "~εῑ", _;
        dative: "~εῑ", _, _;
        accusative: "~ῠ", _, _;
        vocative: "~ῠ", _, _;
    ),
    // Λαλλαι: 3 words (κᾰλανδαι, χαρωνῑται, λαλλαι). Like Ἀραχνη, except in 7 cells: nominative sg, nominative du, genitive sg, genitive du, dative sg, accusative sg, ….
    declension!("λαλλαι", 3, "~α~αι", like Ἀραχνη;
        nominative: "~α~αι", derived, _;
        genitive: derived, derived, _;
        dative: derived, _, _;
        accusative: derived, _, _;
        vocative: derived, _, _;
    ),
    // Βαλσαμον: 3 words (ἀρτολαγανον, βαλσαμον, ἀλαβαστρον).
    declension!("βαλσαμον", 3, "~α~α~ον", [gender: Neuter];
        nominative: "~ᾰ~ᾰ~ον", "~ᾰ~ᾰ~ω", "~ᾰ~ᾰ~ᾰ";
        genitive: "~ᾰ~ᾰ~ου", "~ᾰ~ᾰ~οιν", "~ᾰ~ᾰ~ων";
        dative: "~ᾰ~ᾰ~ῳ", _, "~ᾰ~ᾰ~οις";
        accusative: "~ᾰ~ᾰ~ον", _, "~ᾰ~ᾰ~ᾰ";
        vocative: "~ᾰ~ᾰ~ον", _, _;
    ),
    // Ἐντεᾰ: 3 words (ἐντεᾰ, δηνεᾰ, τειρεᾰ). Like Ὀργῐᾰ, except in 1 cell: dative pl.
    declension!("ἐντεᾰ", 3, "~ᾰ", [gender: Neuter], like Ὀργῐᾰ;
        dative: _, _, "~σῐ|~σῐν";
    ),
    // Παστα: 3 words (χεδροπα, παστα, ἀλμενιχιακα). Like Ἰκρῐα, except in 1 cell: dative pl.
    declension!("παστα", 3, "~α", [gender: Neuter], like Ἰκρῐα;
        dative: _, _, "~οῑς";
    ),
    // Γᾰμφηλαι: 3 words (γᾰμφηλαι, ταυρεασται, δοκηται). Like Ὀργῐᾰ, except in 4 cells: nominative sg, nominative pl, dative pl, accusative pl.
    declension!("γᾰμφηλαι", 3, "~αι", like Ὀργῐᾰ;
        nominative: "~αι", _, "~αι";
        dative: _, _, "~αῑς";
        accusative: _, _, "~ᾱς";
    ),
    // Θεωσῐς: 3 words (θεωσῐς, ἀλθησῐς, ζαχαρῐς). Like Δῐαιρεσῐς, except in 6 cells: nominative du, nominative pl, genitive du, dative sg, dative pl, accusative pl.
    declension!("θεωσῐς", 3, "~ῐς", [gender: Feminine], like Δῐαιρεσῐς;
        nominative: _, derived, "~εες";
        genitive: _, derived, _;
        dative: "~εϊ", _, "~εσῐ";
        accusative: _, _, "~εᾰς";
    ),
    // Ἀσφαλισις: 3 words (ἀνακουφισις, ἀνταποκρισις, ἀσφαλισις).
    declension!("ἀσφαλισις", 3, "~α~ι~ις", [gender: Feminine];
        nominative: "~ᾰ~ῐ~ῐς", "~ᾰ~ῐ~ει", "~ᾰ~ῐ~εις";
        genitive: "~ᾰ~ῐ~εως", "~ᾰ~ῐ~εοιν", "~ᾰ~ῐ~εων";
        dative: "~ᾰ~ῐ~ει", _, "~ᾰ~ῐ~εσῐ|~ᾰ~ῐ~εσῐν";
        accusative: "~ᾰ~ῐ~ῐν", _, "~ᾰ~ῐ~εις";
        vocative: "~ᾰ~ῐ~ῐ", _, _;
    ),
];
