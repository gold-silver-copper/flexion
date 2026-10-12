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
/// `nominative` (sg, du, pl)
/// `genitive` (sg, du, pl)
/// `dative` (sg, du, pl)
/// `accusative` (sg, du, pl)
/// `vocative` (sg, du, pl)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, [$($f:ident: $v:ident),*])? $(, like $parent:ident)?;
     nominative: $c0:tt, $c1:tt, $c2:tt;
     genitive: $c3:tt, $c4:tt, $c5:tt;
     dative: $c6:tt, $c7:tt, $c8:tt;
     accusative: $c9:tt, $c10:tt, $c11:tt;
     vocative: $c12:tt, $c13:tt, $c14:tt;
    ) => {
        Paradigm {
            name: ::std::borrow::Cow::Borrowed($name),
            words: $words,
            lemma: ::std::borrow::Cow::Borrowed($lemma),
            parent: like!($($parent)?),
            inherent: ::std::borrow::Cow::Borrowed(&[$($(feature!($f: $v)),*)?]),
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
    /// Like `τρᾰγος`: 1,194 words.
    Τρᾰγος,
    /// Like `μορῐον`: 1,012 words.
    Μορῐον,
    /// Like `θεμᾰ`: 670 words.
    Θεμᾰ,
    /// Like `οὐσῐᾱ`: 542 words.
    Οὐσῐᾱ,
    /// Like `νῠμφη`: 519 words.
    Νῠμφη,
    /// Like `μερῐσμος`: 463 words.
    Μερῐσμος,
    /// Like `δῐαιρεσῐς`: 429 words.
    Δῐαιρεσῐς,
    /// Like `κᾰτᾰσκευη`: 274 words.
    Κᾰτᾰσκευη,
    /// Like `γενετης`: 196 words.
    Γενετης,
    /// Like `κερκῐς`: 186 words.
    Κερκῐς,
    /// Like `κορᾰξ`: 174 words.
    Κορᾰξ,
    /// Like `κλεις`: 156 words.
    Κλεις,
    /// Like `τρᾰπεζᾰ`: 132 words.
    Τρᾰπεζᾰ,
    /// Like `ἐπῐφᾰνειᾰ`: 114 words.
    Ἐπῐφᾰνειᾰ,
    /// Like `γενος`: 97 words.
    Γενος,
    /// Like `αἰων`: 90 words.
    Αἰων,
    /// Like `χορευτης`: 84 words.
    Χορευτης,
    /// Like `γρᾰφευς`: 74 words.
    Γρᾰφευς,
    /// Like `χᾰρᾰκτηρ`: 70 words.
    Χᾰρᾰκτηρ,
    /// Like `κεραιᾱ`: 69 words.
    Κεραιᾱ,
    /// Like `σημεῑον`: 60 words.
    Σημεῑον,
    /// Like `κᾰλπῐς`: 60 words.
    Κᾰλπῐς,
    /// Like `φορᾱ`: 55 words.
    Φορᾱ,
    /// Like `δᾰμᾰλης`: 52 words.
    Δᾰμᾰλης,
    /// Like `ὀργῐᾰ`: 51 words.
    Ὀργῐᾰ,
    /// Like `κοχλῐᾱς`: 49 words.
    Κοχλῐᾱς,
    /// Like `ζῠγον`: 45 words.
    Ζῠγον,
    /// Like `ὀλῐνοι`: 40 words.
    Ὀλῐνοι,
    /// Like `κροσσαι`: 39 words.
    Κροσσαι,
    /// Like `φᾰος`: 37 words.
    Φᾰος,
    /// Like `κᾰνων`: 32 words.
    Κᾰνων,
    /// Like `κολλοψ`: 32 words.
    Κολλοψ,
    /// Like `κῑων`: 32 words.
    Κῑων,
    /// Like `ναῡλος`: 28 words.
    Ναῡλος,
    /// Like `στᾰχῠς`: 28 words.
    Στᾰχῠς,
    /// Like `δᾰνᾰοι`: 25 words.
    Δᾰνᾰοι,
    /// Like `ῥωμαῑος`: 24 words.
    Ῥωμαῑος,
    /// Like `ἀρετη`: 23 words.
    Ἀρετη,
    /// Like `ἀκρον`: 22 words.
    Ἀκρον,
    /// Like `ἀντλος`: 22 words.
    Ἀντλος,
    /// Like `ἀξων`: 21 words.
    Ἀξων,
    /// Like `λεων`: 20 words.
    Λεων,
    /// Like `ἀμβροσια`: 19 words.
    Ἀμβροσια,
    /// Like `σᾰρξ`: 17 words.
    Σᾰρξ,
    /// Like `μῡς`: 16 words.
    Μῡς,
    /// Like `πους`: 16 words.
    Πους,
    /// Like `λᾰτρῐς`: 15 words.
    Λᾰτρῐς,
    /// Like `πνεῡμᾰ`: 14 words.
    Πνεῡμᾰ,
    /// Like `περᾰς`: 13 words.
    Περᾰς,
    /// Like `ἀστος`: 13 words.
    Ἀστος,
    /// Like `ἀστηρ`: 13 words.
    Ἀστηρ,
    /// Like `ἀσαῑοι`: 13 words.
    Ἀσαῑοι,
    /// Like `λᾰγως`: 12 words.
    Λᾰγως,
    /// Like `σηψ`: 12 words.
    Σηψ,
    /// Like `κηρ`: 12 words.
    Κηρ,
    /// Like `ἀμαυρωσις`: 12 words.
    Ἀμαυρωσις,
    /// Like `στοῑχος`: 12 words.
    Στοῑχος,
    /// Like `ἀποστασις`: 12 words.
    Ἀποστασις,
    /// Like `ἰκρῐα`: 11 words.
    Ἰκρῐα,
    /// Like `ἀγος`: 11 words.
    Ἀγος,
    /// Like `πτερῠξ`: 10 words.
    Πτερῠξ,
    /// Like `χᾰρᾰξ`: 10 words.
    Χᾰρᾰξ,
    /// Like `ἀληθεια`: 10 words.
    Ἀληθεια,
    /// Like `ὑψος`: 10 words.
    Ὑψος,
    /// Like `ἀγκων`: 10 words.
    Ἀγκων,
    /// Like `θηρ`: 10 words.
    Θηρ,
    /// Like `τρωες`: 10 words.
    Τρωες,
    /// Like `λευκη`: 10 words.
    Λευκη,
    /// Like `ἠχου`: 10 words.
    Ἠχου,
    /// Like `πεπερῐ`: 9 words.
    Πεπερῐ,
    /// Like `κοιτη`: 9 words.
    Κοιτη,
    /// Like `ἀλειτης`: 9 words.
    Ἀλειτης,
    /// Like `ἀμη`: 9 words.
    Ἀμη,
    /// Like `πλᾰτεῑᾰ`: 9 words.
    Πλᾰτεῑᾰ,
    /// Like `σησᾰμοειδες`: 9 words.
    Σησᾰμοειδες,
    /// Like `θῑς`: 9 words.
    Θῑς,
    /// Like `ἀλωπεκιᾱ`: 9 words.
    Ἀλωπεκιᾱ,
    /// Like `πᾰτηρ`: 8 words.
    Πᾰτηρ,
    /// Like `νεῡρον`: 8 words.
    Νεῡρον,
    /// Like `ἠχω`: 8 words.
    Ἠχω,
    /// Like `ἀφεσῐς`: 8 words.
    Ἀφεσῐς,
    /// Like `ῥᾱξ`: 7 words.
    Ῥᾱξ,
    /// Like `φρην`: 7 words.
    Φρην,
    /// Like `ἀντονομασια`: 7 words.
    Ἀντονομασια,
    /// Like `μωλῠ`: 7 words.
    Μωλῠ,
    /// Like `ὑδρα`: 7 words.
    Ὑδρα,
    /// Like `χεῑμᾰ`: 7 words.
    Χεῑμᾰ,
    /// Like `ἀναγωγη`: 7 words.
    Ἀναγωγη,
    /// Like `ἀδᾰμᾱς`: 7 words.
    Ἀδᾰμᾱς,
    /// Like `ἀναθεμα`: 7 words.
    Ἀναθεμα,
    /// Like `βᾰσκᾱς`: 7 words.
    Βᾰσκᾱς,
    /// Like `ἀργεστης`: 7 words.
    Ἀργεστης,
    /// Like `ἀραχνη`: 6 words.
    Ἀραχνη,
    /// Like `μοῑρᾰ`: 6 words.
    Μοῑρᾰ,
    /// Like `ἀγορα`: 6 words.
    Ἀγορα,
    /// Like `ἀλῐβᾱς`: 6 words.
    Ἀλῐβᾱς,
    /// Like `μητρως`: 6 words.
    Μητρως,
    /// Like `ἱστος`: 6 words.
    Ἱστος,
    /// Like `ἀμεθυστος`: 6 words.
    Ἀμεθυστος,
    /// Like `κῠᾰρ`: 6 words.
    Κῠᾰρ,
    /// Like `ὑπᾰκοη`: 6 words.
    Ὑπᾰκοη,
    /// Like `ναυτης`: 6 words.
    Ναυτης,
    /// Like `ὑποθεσις`: 6 words.
    Ὑποθεσις,
    /// Like `ἀσπις`: 6 words.
    Ἀσπις,
    /// Like `πηχῠς`: 6 words.
    Πηχῠς,
    /// Like `ὑπερῳον`: 6 words.
    Ὑπερῳον,
    /// Like `παῡλᾰ`: 6 words.
    Παῡλᾰ,
    /// Like `ἡρωϊς`: 6 words.
    Ἡρωϊς,
    /// Like `γεῑσον`: 6 words.
    Γεῑσον,
    /// Like `πᾰπᾱς`: 6 words.
    Πᾰπᾱς,
    /// Like `ἀλκαιᾱ`: 6 words.
    Ἀλκαιᾱ,
    /// Like `γερᾰς`: 6 words.
    Γερᾰς,
    /// Like `σφην`: 5 words.
    Σφην,
    /// Like `ἀνατασις`: 5 words.
    Ἀνατασις,
    /// Like `κωμῡς`: 5 words.
    Κωμῡς,
    /// Like `ἀνισον`: 5 words.
    Ἀνισον,
    /// Like `γεῡσῐς`: 5 words.
    Γεῡσῐς,
    /// Like `νοῡς`: 5 words.
    Νοῡς,
    /// Like `λῐψ`: 5 words.
    Λῐψ,
    /// Like `δαιμων`: 5 words.
    Δαιμων,
    /// Like `ἀστραγᾰλος`: 5 words.
    Ἀστραγᾰλος,
    /// Like `τεῡχος`: 5 words.
    Τεῡχος,
    /// Like `χλουνης`: 5 words.
    Χλουνης,
    /// Like `ἀγρευς`: 5 words.
    Ἀγρευς,
    /// Like `λεῑμᾰξ`: 5 words.
    Λεῑμᾰξ,
    /// Like `λαυρᾱ`: 4 words.
    Λαυρᾱ,
    /// Like `ἰσηγορια`: 4 words.
    Ἰσηγορια,
    /// Like `πλευμων`: 4 words.
    Πλευμων,
    /// Like `ὑπομνημα`: 4 words.
    Ὑπομνημα,
    /// Like `ὑπαρξις`: 4 words.
    Ὑπαρξις,
    /// Like `ἀκροβυστια`: 4 words.
    Ἀκροβυστια,
    /// Like `ἀποτελεσμα`: 4 words.
    Ἀποτελεσμα,
    /// Like `ἰσον`: 4 words.
    Ἰσον,
    /// Like `ἀστερισκος`: 4 words.
    Ἀστερισκος,
    /// Like `κηρῠξ`: 4 words.
    Κηρῠξ,
    /// Like `ἀκοντιον`: 4 words.
    Ἀκοντιον,
    /// Like `γαλη`: 3 words.
    Γαλη,
    /// Like `ἀνακυκλωσις`: 3 words.
    Ἀνακυκλωσις,
    /// Like `πῠρᾰ`: 3 words.
    Πῠρᾰ,
    /// Like `ἱκετης`: 3 words.
    Ἱκετης,
    /// Like `ἰδιον`: 3 words.
    Ἰδιον,
    /// Like `ὑποστρατηγος`: 3 words.
    Ὑποστρατηγος,
    /// Like `δηξ`: 3 words.
    Δηξ,
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
    /// Like `ἱπποκαμπος`: 3 words.
    Ἱπποκαμπος,
    /// Like `ἀντιθεσις`: 3 words.
    Ἀντιθεσις,
    /// Like `ἀριθμος`: 3 words.
    Ἀριθμος,
    /// Like `ἀγκυλη`: 3 words.
    Ἀγκυλη,
    /// Like `ἁμμα`: 3 words.
    Ἁμμα,
    /// Like `δᾰσῠ`: 3 words.
    Δᾰσῠ,
    /// Like `λαλλαι`: 3 words.
    Λαλλαι,
    /// Like `ἐντεᾰ`: 3 words.
    Ἐντεᾰ,
    /// Like `παστα`: 3 words.
    Παστα,
    /// Like `γᾰμφηλαι`: 3 words.
    Γᾰμφηλαι,
    /// Like `θεωσῐς`: 3 words.
    Θεωσῐς,
    /// Like `ὑποψιᾱ`: 3 words.
    Ὑποψιᾱ,
    /// Like `ζωᾱ`: 3 words.
    Ζωᾱ,
    /// Like `ἀσφαλισις`: 3 words.
    Ἀσφαλισις,
    /// Like `βᾰρῠτης`: 2 words.
    Βᾰρῠτης,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 163] = [
        Declension::Τρᾰγος,
        Declension::Μορῐον,
        Declension::Θεμᾰ,
        Declension::Οὐσῐᾱ,
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
        Declension::Γρᾰφευς,
        Declension::Χᾰρᾰκτηρ,
        Declension::Κεραιᾱ,
        Declension::Σημεῑον,
        Declension::Κᾰλπῐς,
        Declension::Φορᾱ,
        Declension::Δᾰμᾰλης,
        Declension::Ὀργῐᾰ,
        Declension::Κοχλῐᾱς,
        Declension::Ζῠγον,
        Declension::Ὀλῐνοι,
        Declension::Κροσσαι,
        Declension::Φᾰος,
        Declension::Κᾰνων,
        Declension::Κολλοψ,
        Declension::Κῑων,
        Declension::Ναῡλος,
        Declension::Στᾰχῠς,
        Declension::Δᾰνᾰοι,
        Declension::Ῥωμαῑος,
        Declension::Ἀρετη,
        Declension::Ἀκρον,
        Declension::Ἀντλος,
        Declension::Ἀξων,
        Declension::Λεων,
        Declension::Ἀμβροσια,
        Declension::Σᾰρξ,
        Declension::Μῡς,
        Declension::Πους,
        Declension::Λᾰτρῐς,
        Declension::Πνεῡμᾰ,
        Declension::Περᾰς,
        Declension::Ἀστος,
        Declension::Ἀστηρ,
        Declension::Ἀσαῑοι,
        Declension::Λᾰγως,
        Declension::Σηψ,
        Declension::Κηρ,
        Declension::Ἀμαυρωσις,
        Declension::Στοῑχος,
        Declension::Ἀποστασις,
        Declension::Ἰκρῐα,
        Declension::Ἀγος,
        Declension::Πτερῠξ,
        Declension::Χᾰρᾰξ,
        Declension::Ἀληθεια,
        Declension::Ὑψος,
        Declension::Ἀγκων,
        Declension::Θηρ,
        Declension::Τρωες,
        Declension::Λευκη,
        Declension::Ἠχου,
        Declension::Πεπερῐ,
        Declension::Κοιτη,
        Declension::Ἀλειτης,
        Declension::Ἀμη,
        Declension::Πλᾰτεῑᾰ,
        Declension::Σησᾰμοειδες,
        Declension::Θῑς,
        Declension::Ἀλωπεκιᾱ,
        Declension::Πᾰτηρ,
        Declension::Νεῡρον,
        Declension::Ἠχω,
        Declension::Ἀφεσῐς,
        Declension::Ῥᾱξ,
        Declension::Φρην,
        Declension::Ἀντονομασια,
        Declension::Μωλῠ,
        Declension::Ὑδρα,
        Declension::Χεῑμᾰ,
        Declension::Ἀναγωγη,
        Declension::Ἀδᾰμᾱς,
        Declension::Ἀναθεμα,
        Declension::Βᾰσκᾱς,
        Declension::Ἀργεστης,
        Declension::Ἀραχνη,
        Declension::Μοῑρᾰ,
        Declension::Ἀγορα,
        Declension::Ἀλῐβᾱς,
        Declension::Μητρως,
        Declension::Ἱστος,
        Declension::Ἀμεθυστος,
        Declension::Κῠᾰρ,
        Declension::Ὑπᾰκοη,
        Declension::Ναυτης,
        Declension::Ὑποθεσις,
        Declension::Ἀσπις,
        Declension::Πηχῠς,
        Declension::Ὑπερῳον,
        Declension::Παῡλᾰ,
        Declension::Ἡρωϊς,
        Declension::Γεῑσον,
        Declension::Πᾰπᾱς,
        Declension::Ἀλκαιᾱ,
        Declension::Γερᾰς,
        Declension::Σφην,
        Declension::Ἀνατασις,
        Declension::Κωμῡς,
        Declension::Ἀνισον,
        Declension::Γεῡσῐς,
        Declension::Νοῡς,
        Declension::Λῐψ,
        Declension::Δαιμων,
        Declension::Ἀστραγᾰλος,
        Declension::Τεῡχος,
        Declension::Χλουνης,
        Declension::Ἀγρευς,
        Declension::Λεῑμᾰξ,
        Declension::Λαυρᾱ,
        Declension::Ἰσηγορια,
        Declension::Πλευμων,
        Declension::Ὑπομνημα,
        Declension::Ὑπαρξις,
        Declension::Ἀκροβυστια,
        Declension::Ἀποτελεσμα,
        Declension::Ἰσον,
        Declension::Ἀστερισκος,
        Declension::Κηρῠξ,
        Declension::Ἀκοντιον,
        Declension::Γαλη,
        Declension::Ἀνακυκλωσις,
        Declension::Πῠρᾰ,
        Declension::Ἱκετης,
        Declension::Ἰδιον,
        Declension::Ὑποστρατηγος,
        Declension::Δηξ,
        Declension::Νεῑκος,
        Declension::Ἀσις,
        Declension::Πειθω,
        Declension::Πεῑνᾰ,
        Declension::Ἀβατον,
        Declension::Ἱπποκαμπος,
        Declension::Ἀντιθεσις,
        Declension::Ἀριθμος,
        Declension::Ἀγκυλη,
        Declension::Ἁμμα,
        Declension::Δᾰσῠ,
        Declension::Λαλλαι,
        Declension::Ἐντεᾰ,
        Declension::Παστα,
        Declension::Γᾰμφηλαι,
        Declension::Θεωσῐς,
        Declension::Ὑποψιᾱ,
        Declension::Ζωᾱ,
        Declension::Ἀσφαλισις,
        Declension::Βᾰρῠτης,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 163] = [
    // Τρᾰγος: 1,194 words (λῠκος, κοσμος, ἐρωμενος, σπογγος, ἀγγελος, πῐθηκος).
    declension!("τρᾰγος", 1194, "~ος";
        nominative: "~ος", "~ω", "~οι";
        genitive: "~ου", "~οιν", "~ων";
        dative: "~ῳ", "~οιν", "~οις";
        accusative: "~ον", "~ω", "~ους";
        vocative: "~ε", "~ω", "~οι";
    ),
    // Μορῐον: 1,012 words (παιδῐον, δενδρον, μῠστηρῐον, ἐργον, εἰδωλον, σῠνεδρῐον). Like Τρᾰγος, except in 5 cells: nominative sg, nominative pl, accusative pl, vocative sg, vocative pl.
    declension!("μορῐον", 1012, "~ον", [gender: Neuter], like Τρᾰγος;
        nominative: "~ον", _, "~ᾰ";
        genitive: _, _, _;
        dative: _, _, _;
        accusative: _, _, "~ᾰ";
        vocative: "~ον", _, "~ᾰ";
    ),
    // Θεμᾰ: 670 words (μελῐ, γρᾰμμᾰ, δερμᾰ, σῑγμᾰ, βαπτισμᾰ, στομᾰ).
    declension!("θεμᾰ", 670, "~";
        nominative: "~", "~τε", "~τᾰ";
        genitive: "~τος", "~τοιν", "~των";
        dative: "~τῐ", "~τοιν", "~σῐ|~σῐν";
        accusative: "~", "~τε", "~τᾰ";
        vocative: "~", "~τε", "~τᾰ";
    ),
    // Οὐσῐᾱ: 542 words (δημοκρᾰτῐᾱ, χωρᾱ, μελῳδῐᾱ, φῐλῐᾱ, σοφῐᾱ, ἡγεμονῐᾱ).
    declension!("οὐσῐᾱ", 542, "~ᾱ", [gender: Feminine];
        nominative: "~ᾱ", "~ᾱ", "~αι";
        genitive: "~ᾱς", "~αιν", "~ων";
        dative: "~ᾳ", "~αιν", "~αις";
        accusative: "~ᾱν", "~ᾱ", "~ᾱς";
        vocative: "~ᾱ", "~ᾱ", "~αι";
    ),
    // Νῠμφη: 519 words (νῑκη, δῑνη, κορη, γῠνη, ἐλεημοσῠνη, εἰρηνη). Like Οὐσῐᾱ, except in 5 cells: nominative sg, genitive sg, dative sg, accusative sg, vocative sg.
    declension!("νῠμφη", 519, "~η", [gender: Feminine], like Οὐσῐᾱ;
        nominative: "~η", _, _;
        genitive: "~ης", _, _;
        dative: "~ῃ", _, _;
        accusative: "~ην", _, _;
        vocative: "~η", _, _;
    ),
    // Μερῐσμος: 463 words (λαιμος, μᾰστος, θεος, καιρος, νεκρος, ὀφθᾰλμος). Like Τρᾰγος, except in 4 cells: genitive sg, genitive du, dative du, dative pl.
    declension!("μερῐσμος", 463, "~ος", [gender: Masculine], like Τρᾰγος;
        nominative: _, _, _;
        genitive: "~οῡ", "~οῑν", _;
        dative: _, "~οῑν", "~οῑς";
        accusative: _, _, _;
        vocative: _, _, _;
    ),
    // Δῐαιρεσῐς: 429 words (πολῐς, γνωσῐς, κῠβερνησῐς, μορφωσῐς, φῠσῐς, κρῐσῐς).
    declension!("δῐαιρεσῐς", 429, "~ῐς", [gender: Feminine];
        nominative: "~ῐς", "~ει", "~εις";
        genitive: "~εως", "~εοιν", "~εων";
        dative: "~ει", "~εοιν", "~εσῐ|~εσῐν";
        accusative: "~ῐν", "~ει", "~εις";
        vocative: "~ῐ", "~ει", "~εις";
    ),
    // Κᾰτᾰσκευη: 274 words (μηχᾰνη, μουσῐκη, γρᾰμμᾰτῐκη, φωνη, κεφᾰλη, ἡδονη). Like Νῠμφη, except in 3 cells: genitive du, dative du, dative pl.
    declension!("κᾰτᾰσκευη", 274, "~η", [gender: Feminine], like Νῠμφη;
        nominative: _, _, _;
        genitive: _, "~αῑν", _;
        dative: _, "~αῑν", "~αῑς";
        accusative: _, _, _;
        vocative: _, _, _;
    ),
    // Γενετης: 196 words (προφητης, κλεπτης, πολῑτης, τρωγλοδῠτης, σῠβᾰρῑτης, χᾰρτης). Like Νῠμφη, except in 3 cells: nominative sg, genitive sg, vocative sg.
    declension!("γενετης", 196, "~ης", [gender: Masculine], like Νῠμφη;
        nominative: "~ης", _, _;
        genitive: "~ου", _, _;
        dative: _, _, _;
        accusative: _, _, _;
        vocative: "~ᾰ", _, _;
    ),
    // Κερκῐς: 186 words (παῑς, λεπῐς, πᾰτρῐς, ἑλληνῐς, περσῐς, ἑλμῐνς).
    declension!("κερκῐς", 186, "~ς";
        nominative: "~ς", "~δε", "~δες";
        genitive: "~δος", "~δοιν", "~δων";
        dative: "~δῐ", "~δοιν", "~σῐ|~σῐν";
        accusative: "~δᾰ", "~δε", "~δᾰς";
        vocative: "~", "~δε", "~δες";
    ),
    // Κορᾰξ: 174 words (ὀνῠξ, πτερῠξ, γλαῡξ, ἑλῐξ, κῐλῐξ, νῠξ).
    declension!("κορᾰξ", 174, "~ξ";
        nominative: "~ξ", "~κε", "~κες";
        genitive: "~κος", "~κοιν", "~κων";
        dative: "~κῐ", "~κοιν", "~ξῐ|~ξῐν";
        accusative: "~κᾰ", "~κε", "~κᾰς";
        vocative: "~ξ", "~κε", "~κες";
    ),
    // Κλεις: 156 words (μῠκης, κρης, σκληροτης, θης, κλεις, χλᾰμῠς). Like Κερκῐς, except in 1 cell: vocative sg.
    declension!("κλεις", 156, "~ς", like Κερκῐς;
        nominative: _, _, _;
        genitive: _, _, _;
        dative: _, _, _;
        accusative: _, _, _;
        vocative: "~ς", _, _;
    ),
    // Τρᾰπεζᾰ: 132 words (θᾰλᾰσσᾰ, ῥῐζᾰ, θᾰλᾰττᾰ, δοξᾰ, γλωττᾰ, μελῐττᾰ). Like Νῠμφη, except in 3 cells: nominative sg, accusative sg, vocative sg.
    declension!("τρᾰπεζᾰ", 132, "~ᾰ", [gender: Feminine], like Νῠμφη;
        nominative: "~ᾰ", _, _;
        genitive: _, _, _;
        dative: _, _, _;
        accusative: "~ᾰν", _, _;
        vocative: "~ᾰ", _, _;
    ),
    // Ἐπῐφᾰνειᾰ: 114 words (βᾰσῐλειᾰ, ἀσθενειᾰ, μᾰχαιρᾰ, βοηθειᾰ, συγγενειᾰ, ὀρθοεπειᾰ). Like Τρᾰπεζᾰ, except in 2 cells: genitive sg, dative sg.
    declension!("ἐπῐφᾰνειᾰ", 114, "~ᾰ", [gender: Feminine], like Τρᾰπεζᾰ;
        nominative: _, _, _;
        genitive: "~ᾱς", _, _;
        dative: "~ᾳ", _, _;
        accusative: _, _, _;
        vocative: _, _, _;
    ),
    // Γενος: 97 words (σκῠλος, κλεος, ἐθνος, πληθος, τελος, στηθος).
    declension!("γενος", 97, "~ος", [gender: Neuter];
        nominative: "~ος", "~ει", "~η";
        genitive: "~ους", "~οῑν", "~ων";
        dative: "~ει", "~οῑν", "~εσῐ|~εσῐν";
        accusative: "~ος", "~ει", "~η";
        vocative: "~ος", "~ει", "~η";
    ),
    // Αἰων: 90 words (ἑλλην, λεγεων, σᾱπων, λᾰκων, ἀκαρνᾱν, ἠδων).
    declension!("αἰων", 90, "~ν", [gender: Masculine];
        nominative: "~ν", "~νε", "~νες";
        genitive: "~νος", "~νοιν", "~νων";
        dative: "~νῐ", "~νοιν", "~σῐ|~σῐν";
        accusative: "~νᾰ", "~νε", "~νᾰς";
        vocative: "~ν", "~νε", "~νες";
    ),
    // Χορευτης: 84 words (ἐρᾰστης, ποιητης, βᾰπτῐστης, κρῐτης, κωλοβᾰθρῐστης, σοφῐστης). Like Κᾰτᾰσκευη, except in 3 cells: nominative sg, genitive sg, vocative sg.
    declension!("χορευτης", 84, "~ης", [gender: Masculine], like Κᾰτᾰσκευη;
        nominative: "~ης", _, _;
        genitive: "~οῡ", _, _;
        dative: _, _, _;
        accusative: _, _, _;
        vocative: "~ᾰ", _, _;
    ),
    // Γρᾰφευς: 74 words (βᾰσῐλευς, ἀρχιερευς, γραμμᾰτευς, αἰολευς, ἀμφισσευς, ἀντῐκυρευς).
    declension!("γρᾰφευς", 74, "~ευς", [gender: Masculine];
        nominative: "~ευς", "~η", "~ης|~εῑς";
        genitive: "~εως", "~εοιν", "~εων";
        dative: "~εῑ", "~εοιν", "~εῡσῐ|~εῡσῐν";
        accusative: "~εᾱ", "~η", "~εᾱς";
        vocative: "~εῡ", "~η", "~ης|~εῑς";
    ),
    // Χᾰρᾰκτηρ: 70 words (αἰων, μᾰρτῠρ, πᾰνθηρ, ἰβηρ, χειρ, ἐορ).
    declension!("χᾰρᾰκτηρ", 70, "~";
        nominative: "~", "~ε", "~ες";
        genitive: "~ος", "~οιν", "~ων";
        dative: "~ῐ", "~οιν", "~σῐ|~σῐν";
        accusative: "~ᾰ", "~ε", "~ᾰς";
        vocative: "~", "~ε", "~ες";
    ),
    // Κεραιᾱ: 69 words (προφητειᾱ, πορνειᾱ, παιδειᾱ, δουλειᾱ, ξῠλειᾱ, νηστειᾱ).
    declension!("κεραιᾱ", 69, "~~ᾱ", [gender: Feminine];
        nominative: "~~ᾱ", "~~ᾱ", "~ῑα~";
        genitive: "~~ᾱς", "~~αιν", "~~ων";
        dative: "~~ᾳ", "~~αιν", "~~αις";
        accusative: "~~ᾱν", "~~ᾱ", "~~ᾱς";
        vocative: "~~ᾱ", "~~ᾱ", "~ῑα~";
    ),
    // Σημεῑον: 60 words (θεῑον, πανδοχεῑον, πᾰνδοκεῑον, στοιχεῑον, βᾰλᾰνεῑον, σημεῑον).
    declension!("σημεῑον", 60, "~ῑον", [gender: Neuter];
        nominative: "~ῑον", "~ιω", "~ῑᾰ";
        genitive: "~ιου", "~ιοιν", "~ιων";
        dative: "~ιῳ", "~ιοιν", "~ιοις";
        accusative: "~ῑον", "~ιω", "~ῑᾰ";
        vocative: "~ῑον", "~ιω", "~ῑᾰ";
    ),
    // Κᾰλπῐς: 60 words (πελασγῐωτῐς, σπᾰρτῐᾱτῐς, σῠβᾰρῑτῐς, σμῠρῐς, ἀσπρῐς, εὐνῐς). Like Κερκῐς, except in 1 cell: accusative sg.
    declension!("κᾰλπῐς", 60, "~ς", like Κερκῐς;
        nominative: _, _, _;
        genitive: _, _, _;
        dative: _, _, _;
        accusative: "~ν", _, _;
        vocative: _, _, _;
    ),
    // Φορᾱ: 55 words (παιδῐᾱ, πᾰτρῐᾱ, γενεᾱ, θεᾱ, στρᾰτῐᾱ, σκῐᾱ). Like Οὐσῐᾱ, except in 3 cells: genitive du, dative du, dative pl.
    declension!("φορᾱ", 55, "~ᾱ", [gender: Feminine], like Οὐσῐᾱ;
        nominative: _, _, _;
        genitive: _, "~αῑν", _;
        dative: _, "~αῑν", "~αῑς";
        accusative: _, _, _;
        vocative: _, _, _;
    ),
    // Δᾰμᾰλης: 52 words (βελγης, δακης, μῐνῠης, ὀδρῠσης, πῐσῐδης, σᾰκης). Like Γενετης, except in 1 cell: vocative sg.
    declension!("δᾰμᾰλης", 52, "~ης", [gender: Masculine], like Γενετης;
        nominative: _, _, _;
        genitive: _, _, _;
        dative: _, _, _;
        accusative: _, _, _;
        vocative: "~η", _, _;
    ),
    // Ὀργῐᾰ: 51 words (ἑξηκοντᾰ, κᾰστᾰνειᾰ, κᾰστᾰνᾰ, ὀργῐᾰ, ἐγκᾰτᾰ, κεδμᾰτᾰ).
    declension!("ὀργῐᾰ", 51, "~ᾰ", [gender: Neuter];
        nominative: "~ᾰ", derived, "~ᾰ";
        genitive: derived, derived, "~ων";
        dative: derived, derived, "~οις";
        accusative: derived, derived, "~ᾰ";
        vocative: derived, derived, "~ᾰ";
    ),
    // Κοχλῐᾱς: 49 words (νεᾱνῐᾱς, γᾰλᾰξῐᾱς, καικιᾱς, κῠβᾱς, τᾰμῐᾱς, κοχλῐᾱς). Like Οὐσῐᾱ, except in 2 cells: nominative sg, genitive sg.
    declension!("κοχλῐᾱς", 49, "~ᾱς", [gender: Masculine], like Οὐσῐᾱ;
        nominative: "~ᾱς", _, _;
        genitive: "~ου", _, _;
        dative: _, _, _;
        accusative: _, _, _;
        vocative: _, _, _;
    ),
    // Ζῠγον: 45 words (πτερον, ὀνομᾰστῐκον, ᾠον, ἑρπετον, ζῠγον, φῠτον). Like Μορῐον, except in 4 cells: genitive sg, genitive du, dative du, dative pl.
    declension!("ζῠγον", 45, "~ον", [gender: Neuter], like Μορῐον;
        nominative: _, _, _;
        genitive: "~οῡ", "~οῑν", _;
        dative: _, "~οῑν", "~οῑς";
        accusative: _, _, _;
        vocative: _, _, _;
    ),
    // Ὀλῐνοι: 40 words (πᾰρᾰβολᾱνοι, μοσσῡνοικοι, ἡνῐοχοι, σῐνδοι, ὀλῐνοι, ταυλαντῐοι). Like Τρᾰγος, except in 0 cells: .
    declension!("ὀλῐνοι", 40, "~οι", [gender: Masculine], like Τρᾰγος;
        nominative: _, _, _;
        genitive: _, _, _;
        dative: _, _, _;
        accusative: _, _, _;
        vocative: _, _, _;
    ),
    // Κροσσαι: 39 words (ἑβδομηκοντᾰκωμηται, τεσσᾰρεσκαιδεκᾰτῑται, ὀᾰσῑται, ἰσκαι, κρουπεζαι, ὀζολαι). Like Γενετης, except in 0 cells: .
    declension!("κροσσαι", 39, "~αι", like Γενετης;
        nominative: _, _, _;
        genitive: _, _, _;
        dative: _, _, _;
        accusative: _, _, _;
        vocative: _, _, _;
    ),
    // Φᾰος: 37 words (δᾰσος, πεος, σκᾰφος, φᾰος, πᾰχος, κᾰμπος).
    declension!("φᾰος", 37, "~ος", [gender: Neuter];
        nominative: "~ος", "~εε", "~εᾰ";
        genitive: "~εος", "~εοιν", "~εων";
        dative: "~εῐ̈", "~εοιν", "~εσῐ|~εσῐν";
        accusative: "~ος", "~εε", "~εᾰ";
        vocative: "~ος", "~εε", "~εᾰ";
    ),
    // Κᾰνων: 32 words (ἡγεμων, τεκτων, μᾰκεδων, μαιων, μῠρμῐδων, παιων).
    declension!("κᾰνων", 32, "~ων";
        nominative: "~ων", "~ονε", "~ονες";
        genitive: "~ονος", "~ονοιν", "~ονων";
        dative: "~ονῐ", "~ονοιν", "~οσῐ|~οσῐν";
        accusative: "~ονᾰ", "~ονε", "~ονᾰς";
        vocative: "~ων", "~ονε", "~ονες";
    ),
    // Κολλοψ: 32 words (αἰθῐοψ, κῠκλωψ, δολοψ, αἰγῐλωψ, ἀγχιλωψ, ὀψ).
    declension!("κολλοψ", 32, "~ψ";
        nominative: "~ψ", "~πε", "~πες";
        genitive: "~πος", "~ποιν", "~πων";
        dative: "~πῐ", "~ποιν", "~ψῐ|~ψῐν";
        accusative: "~πᾰ", "~πε", "~πᾰς";
        vocative: "~ψ", "~πε", "~πες";
    ),
    // Κῑων: 32 words (κῠων, καστωρ, ῥητωρ, δωτωρ, στατωρ, κοσμοκρᾰτωρ).
    declension!("κῑων", 32, "~ω~";
        nominative: "~ω~", "~ο~ε", "~ο~ες";
        genitive: "~ο~ος", "~ο~οιν", "~ο~ων";
        dative: "~ο~ῐ", "~ο~οιν", "~ο~σῐ|~ο~σῐν";
        accusative: "~ο~ᾰ", "~ο~ε", "~ο~ᾰς";
        vocative: "~ο~", "~ο~ε", "~ο~ες";
    ),
    // Ναῡλος: 28 words (ταῡρος, δοῡλος, σαῡρος, πλοῡτος, καρδοῡχος, οὐολοῡσκος).
    declension!("ναῡλος", 28, "~ῡ~ος", [gender: Masculine];
        nominative: "~ῡ~ος", "~υ~ω", "~ῡ~οι";
        genitive: "~υ~ου", "~υ~οιν", "~υ~ων";
        dative: "~υ~ῳ", "~υ~οιν", "~υ~οις";
        accusative: "~ῡ~ον", "~υ~ω", "~υ~ους";
        vocative: "~ῡ~ε", "~υ~ω", "~ῡ~οι";
    ),
    // Στᾰχῠς: 28 words (λῐβῠς, λῐγῠς, γενῠς, ἐγχελῠς, ῥᾰπῠς, ῥᾰφῠς).
    declension!("στᾰχῠς", 28, "~ῠς";
        nominative: "~ῠς", "~ῠε|~ῡ", "~ῠες|~ῡς";
        genitive: "~ῠος", "~ῠοιν", "~ῠων";
        dative: "~ῠῐ̈|~υι", "~ῠοιν", "~ῠσῐ|~ῠσῐν";
        accusative: "~ῠν", "~ῠε|~ῡ", "~ῡς|~ῠᾰς";
        vocative: "~ῠ", "~ῠε|~ῡ", "~ῠες|~ῡς";
    ),
    // Δᾰνᾰοι: 25 words (δᾰνᾰοι, τῐβᾰρηνοι, μεθοδῐκοι, πᾰλαιοι, ἐμπειρῐκοι, στοιχειωμᾰτῐκοι). Like Μερῐσμος, except in 0 cells: .
    declension!("δᾰνᾰοι", 25, "~οι", [gender: Masculine], like Μερῐσμος;
        nominative: _, _, _;
        genitive: _, _, _;
        dative: _, _, _;
        accusative: _, _, _;
        vocative: _, _, _;
    ),
    // Ῥωμαῑος: 24 words (ἰουδαῑος, φαρισαῑος, ῥωμαῑος, ἑβραῑος, χᾰλδαῑος, θεῑος). Like Σημεῑον, except in 5 cells: nominative sg, nominative pl, accusative pl, vocative sg, vocative pl.
    declension!("ῥωμαῑος", 24, "~ῑος", [gender: Masculine], like Σημεῑον;
        nominative: "~ῑος", _, "~ῑοι";
        genitive: _, _, _;
        dative: _, _, _;
        accusative: _, _, "~ιους";
        vocative: "~ῑε", _, "~ῑοι";
    ),
    // Ἀρετη: 23 words (ἀρχη, ἀδελφη, ἀκη, ἀοιδη, ἀρετη, ἀκμη).
    declension!("ἀρετη", 23, "ἀ~η", [gender: Feminine];
        nominative: "ᾰ̓~η", "ᾰ̓~ᾱ", "ᾰ̓~αι";
        genitive: "ᾰ̓~ης", "ᾰ̓~αῑν", "ᾰ̓~ων";
        dative: "ᾰ̓~ῃ", "ᾰ̓~αῑν", "ᾰ̓~αῑς";
        accusative: "ᾰ̓~ην", "ᾰ̓~ᾱ", "ᾰ̓~ᾱς";
        vocative: "ᾰ̓~η", "ᾰ̓~ᾱ", "ᾰ̓~αι";
    ),
    // Ἀκρον: 22 words (ἀστρον, ἀρθρον, ἀκρον, ἀγγεῑον, ἀλευρον, ἀροτρον).
    declension!("ἀκρον", 22, "ἀ~ον", [gender: Neuter];
        nominative: "ᾰ̓~ον", "ᾰ̓~ω", "ᾰ̓~ᾰ";
        genitive: "ᾰ̓~ου", "ᾰ̓~οιν", "ᾰ̓~ων";
        dative: "ᾰ̓~ῳ", "ᾰ̓~οιν", "ᾰ̓~οις";
        accusative: "ᾰ̓~ον", "ᾰ̓~ω", "ᾰ̓~ᾰ";
        vocative: "ᾰ̓~ον", "ᾰ̓~ω", "ᾰ̓~ᾰ";
    ),
    // Ἀντλος: 22 words (ἀνθρωπος, ἀποστολος, ἀρκτος, ἀνδρεῑος, ἀργεῑος, ἀνεμος). Like Ἀκρον, except in 5 cells: nominative sg, nominative pl, accusative pl, vocative sg, vocative pl.
    declension!("ἀντλος", 22, "ἀ~ος", like Ἀκρον;
        nominative: "ᾰ̓~ος", _, "ᾰ̓~οι";
        genitive: _, _, _;
        dative: _, _, _;
        accusative: _, _, "ᾰ̓~ους";
        vocative: "ᾰ̓~ε", _, "ᾰ̓~οι";
    ),
    // Ἀξων: 21 words (ἀρχιτεκτων, βιστων, χᾱων, λῠκᾱων, βρᾰχῑων, κᾰκοδαιμων). Like Κᾰνων, except in 1 cell: vocative sg.
    declension!("ἀξων", 21, "~ων", [gender: Masculine], like Κᾰνων;
        nominative: _, _, _;
        genitive: _, _, _;
        dative: _, _, _;
        accusative: _, _, _;
        vocative: "~ον", _, _;
    ),
    // Λεων: 20 words (θερᾰπων, λεων, δρᾰκων, γερων, ὁρῐζων, χᾰμαιλεων).
    declension!("λεων", 20, "~ων", [gender: Masculine];
        nominative: "~ων", "~οντε", "~οντες";
        genitive: "~οντος", "~οντοιν", "~οντων";
        dative: "~οντῐ", "~οντοιν", "~ουσῐ|~ουσῐν";
        accusative: "~οντᾰ", "~οντε", "~οντᾰς";
        vocative: "~ον", "~οντε", "~οντες";
    ),
    // Ἀμβροσια: 19 words (ἀπορια, ἀρτηρια, ἀμνηστια, ἀμβροσια, ἀσωτια, ἀγεννησια).
    declension!("ἀμβροσια", 19, "ἀ~ια";
        nominative: "ᾰ̓~ῐᾱ", "ᾰ̓~ῐᾱ", "ᾰ̓~ῐαι";
        genitive: "ᾰ̓~ῐᾱς", "ᾰ̓~ῐαιν", "ᾰ̓~ῐων";
        dative: "ᾰ̓~ῐᾳ", "ᾰ̓~ῐαιν", "ᾰ̓~ῐαις";
        accusative: "ᾰ̓~ῐᾱν", "ᾰ̓~ῐᾱ", "ᾰ̓~ῐᾱς";
        vocative: "ᾰ̓~ῐᾱ", "ᾰ̓~ῐᾱ", "ᾰ̓~ῐαι";
    ),
    // Σᾰρξ: 17 words (σᾰρξ, θρᾳξ, φρῠξ, προιξ, σφηξ, πλᾰξ). Like Κορᾰξ, except in 2 cells: genitive du, dative du.
    declension!("σᾰρξ", 17, "~ξ", like Κορᾰξ;
        nominative: _, _, _;
        genitive: _, "~κοῑν", _;
        dative: _, "~κοῑν", _;
        accusative: _, _, _;
        vocative: _, _, _;
    ),
    // Μῡς: 16 words (σῡς, δελφῡς, δρῡς, ὀφρῡς, μῡς, γρᾱῡς). Like Στᾰχῠς, except in 4 cells: nominative sg, dative sg, accusative sg, vocative sg.
    declension!("μῡς", 16, "~ῡς", like Στᾰχῠς;
        nominative: "~ῡς", _, _;
        genitive: _, _, _;
        dative: "~ῠῐ̈|~υῑ", _, _;
        accusative: "~ῡν", _, _;
        vocative: "~ῡ", _, _;
    ),
    // Πους: 16 words (πους, ὀκτωπους, πολυπους, τρῐπους, δᾰσῠπους, λᾰγωπους). Like Κερκῐς, except in 2 cells: nominative sg, vocative sg.
    declension!("πους", 16, "~υς", [gender: Masculine], like Κερκῐς;
        nominative: "~υς", _, _;
        genitive: _, _, _;
        dative: _, _, _;
        accusative: _, _, _;
        vocative: "~υς", _, _;
    ),
    // Λᾰτρῐς: 15 words (ὀκρῐς, λᾰτρῐς, μηνῐς, πορτῐς, κᾰπετῐς, ῥῐζῐς).
    declension!("λᾰτρῐς", 15, "~ῐς";
        nominative: "~ῐς", "~ῐε", "~ῐες";
        genitive: "~ῐος", "~ῐοιν", "~ῐων";
        dative: "~ῑ", "~ῐοιν", "~ῐσῐ|~ῐσῐν";
        accusative: "~ῐν", "~ῐε", "~ῑς|~ῐᾰς";
        vocative: "~ῐ", "~ῐε", "~ῐες";
    ),
    // Πνεῡμᾰ: 14 words (πνεῡμᾰ, καῡμᾰ, ῥεῡμᾰ, θαῡμᾰ, ζεῡγμᾰ, τραῡμᾰ).
    declension!("πνεῡμᾰ", 14, "~ῡ~", [gender: Neuter];
        nominative: "~ῡ~", "~υ~τε", "~υ~τᾰ";
        genitive: "~υ~τος", "~υ~τοιν", "~υ~των";
        dative: "~υ~τῐ", "~υ~τοιν", "~υ~σῐ|~υ~σῐν";
        accusative: "~ῡ~", "~υ~τε", "~υ~τᾰ";
        vocative: "~ῡ~", "~υ~τε", "~υ~τᾰ";
    ),
    // Περᾰς: 13 words (φως, τερᾰς, κρεᾰς, πᾰγκρεᾰς, ἐρῠσῐπελᾰς, περᾰς). Like Θεμᾰ, except in 3 cells: nominative sg, accusative sg, vocative sg.
    declension!("περᾰς", 13, "~ς", [gender: Neuter], like Θεμᾰ;
        nominative: "~ς", _, _;
        genitive: _, _, _;
        dative: _, _, _;
        accusative: "~ς", _, _;
        vocative: "~ς", _, _;
    ),
    // Ἀστος: 13 words (ἀγρος, ἀδελφος, ἀοιδος, ἀλφος, ἀμνος, ἀρνος). Like Ἀντλος, except in 4 cells: genitive sg, genitive du, dative du, dative pl.
    declension!("ἀστος", 13, "ἀ~ος", like Ἀντλος;
        nominative: _, _, _;
        genitive: "ᾰ̓~οῡ", "ᾰ̓~οῑν", _;
        dative: _, "ᾰ̓~οῑν", "ᾰ̓~οῑς";
        accusative: _, _, _;
        vocative: _, _, _;
    ),
    // Ἀστηρ: 13 words (ἀνηρ, ἀστηρ, ἀλεκτωρ, ἀφθᾰ, ἀλωπηξ, ἀροτηρ).
    declension!("ἀστηρ", 13, "ἀ~";
        nominative: "ᾰ̓~", "ᾰ̓~ε", "ᾰ̓~ες";
        genitive: "ᾰ̓~ος", "ᾰ̓~οιν", "ᾰ̓~ων";
        dative: "ᾰ̓~ῐ", "ᾰ̓~οιν", "ᾰ̓~σῐ";
        accusative: "ᾰ̓~ᾰ", "ᾰ̓~ε", "ᾰ̓~ᾰς";
        vocative: "ᾰ̓~", "ᾰ̓~ε", "ᾰ̓~ες";
    ),
    // Ἀσαῑοι: 13 words (βοττῐαῑοι, ἀραμαῑοι, ἀρδιαῑοι, ἀσαῑοι, θᾰμᾰναῑοι, νῐψαῑοι). Like Ῥωμαῑος, except in 0 cells: .
    declension!("ἀσαῑοι", 13, "~ῑοι", [gender: Masculine], like Ῥωμαῑος;
        nominative: _, _, _;
        genitive: _, _, _;
        dative: _, _, _;
        accusative: _, _, _;
        vocative: _, _, _;
    ),
    // Λᾰγως: 12 words (νεως, γᾰλως, τᾰως, λᾰγως, ἑως, λεως).
    declension!("λᾰγως", 12, "~ως";
        nominative: "~ως", "~ω", "~ῳ";
        genitive: "~ω", "~ῳν", "~ων";
        dative: "~ῳ", "~ῳν", "~ῳς";
        accusative: "~ω|~ων", "~ω", "~ως";
        vocative: "~ως", "~ω", "~ῳ";
    ),
    // Σηψ: 12 words (ὠψ, γρῡψ, θρῑψ, θωψ, κλωψ, σκωψ). Like Κολλοψ, except in 2 cells: genitive du, dative du.
    declension!("σηψ", 12, "~ψ", like Κολλοψ;
        nominative: _, _, _;
        genitive: _, "~ποῑν", _;
        dative: _, "~ποῑν", _;
        accusative: _, _, _;
        vocative: _, _, _;
    ),
    // Κηρ: 12 words (κηρ, ἐᾰρ, θενᾰρ, ὀνᾰρ, νεκτᾰρ, μελᾰν). Like Χᾰρᾰκτηρ, except in 4 cells: nominative pl, accusative sg, accusative pl, vocative pl.
    declension!("κηρ", 12, "~", [gender: Neuter], like Χᾰρᾰκτηρ;
        nominative: _, _, "~ᾰ";
        genitive: _, _, _;
        dative: _, _, _;
        accusative: "~", _, "~ᾰ";
        vocative: _, _, "~ᾰ";
    ),
    // Ἀμαυρωσις: 12 words (ἀρσις, ἀνεσις, ἀσκησις, ἀμαυρωσις, ἀκροπολις, ἀποδειξις).
    declension!("ἀμαυρωσις", 12, "ἀ~ις", [gender: Feminine];
        nominative: "ᾰ̓~ῐς", "ᾰ̓~ει", "ᾰ̓~εις";
        genitive: "ᾰ̓~εως", "ᾰ̓~εοιν", "ᾰ̓~εων";
        dative: "ᾰ̓~ει", "ᾰ̓~εοιν", "ᾰ̓~εσῐ|ᾰ̓~εσῐν";
        accusative: "ᾰ̓~ῐν", "ᾰ̓~ει", "ᾰ̓~εις";
        vocative: "ᾰ̓~ῐ", "ᾰ̓~ει", "ᾰ̓~εις";
    ),
    // Στοῑχος: 12 words (χοῑρος, ἑταῑρος, στοῑχος, τοῑχος, σχοῑνος, ῥοῑζος).
    declension!("στοῑχος", 12, "~ῑ~ος", [gender: Masculine];
        nominative: "~ῑ~ος", "~ι~ω", "~ῑ~οι";
        genitive: "~ι~ου", "~ι~οιν", "~ι~ων";
        dative: "~ι~ῳ", "~ι~οιν", "~ι~οις";
        accusative: "~ῑ~ον", "~ι~ω", "~ι~ους";
        vocative: "~ῑ~ε", "~ι~ω", "~ῑ~οι";
    ),
    // Ἀποστασις: 12 words (ἀναβρωσις, ἀποταξις, ἀναχωρησις, ἀποβασις, ἀκανθις, ἀποστασις).
    declension!("ἀποστασις", 12, "ἀ~α~ις", [gender: Feminine];
        nominative: "ᾰ̓~ᾰ~ῐς", "ᾰ̓~ᾰ~ει", "ᾰ̓~ᾰ~εις";
        genitive: "ᾰ̓~ᾰ~εως", "ᾰ̓~ᾰ~εοιν", "ᾰ̓~ᾰ~εων";
        dative: "ᾰ̓~ᾰ~ει", "ᾰ̓~ᾰ~εοιν", "ᾰ̓~ᾰ~εσῐ|ᾰ̓~ᾰ~εσῐν";
        accusative: "ᾰ̓~ᾰ~ῐν", "ᾰ̓~ᾰ~ει", "ᾰ̓~ᾰ~εις";
        vocative: "ᾰ̓~ᾰ~ῐ", "ᾰ̓~ᾰ~ει", "ᾰ̓~ᾰ~εις";
    ),
    // Ἰκρῐα: 11 words (γενεσια, μουστακια, σαγματοπασμαγαδια, εὐληρα, βρῡτεα, κανθηλια). Like Ὀργῐᾰ, except in 1 cell: nominative sg.
    declension!("ἰκρῐα", 11, "~α", [gender: Neuter], like Ὀργῐᾰ;
        nominative: "~α", _, _;
        genitive: _, _, _;
        dative: _, _, _;
        accusative: _, _, _;
        vocative: _, _, _;
    ),
    // Ἀγος: 11 words (ἀνθος, ἀγος, ἀγκος, ἀλγος, ἀχος, ἀφενος).
    declension!("ἀγος", 11, "ἀ~ος", [gender: Neuter];
        nominative: "ᾰ̓~ος", "ᾰ̓~ει", "ᾰ̓~η";
        genitive: "ᾰ̓~ους", "ᾰ̓~οῑν", "ᾰ̓~ων";
        dative: "ᾰ̓~ει", "ᾰ̓~οῑν", "ᾰ̓~εσῐ|ᾰ̓~εσῐν";
        accusative: "ᾰ̓~ος", "ᾰ̓~ει", "ᾰ̓~η";
        vocative: "ᾰ̓~ος", "ᾰ̓~ει", "ᾰ̓~η";
    ),
    // Πτερῠξ: 10 words (λᾰτᾰξ, ὀρτῠξ, μᾰστῑξ, τεττῑξ, λᾰλᾰξ, πομφολῠξ).
    declension!("πτερῠξ", 10, "~ξ";
        nominative: "~ξ", "~γε", "~γες";
        genitive: "~γος", "~γοιν", "~γων";
        dative: "~γῐ", "~γοιν", "~ξῐ|~ξῐν";
        accusative: "~γᾰ", "~γε", "~γᾰς";
        vocative: "~ξ", "~γε", "~γες";
    ),
    // Χᾰρᾰξ: 10 words (χᾰλῐξ, περδῑξ, ψῑξ, φῠλᾰξ, σκῠλᾰξ, κᾰλῠξ). Like Κορᾰξ, except in 6 cells: nominative pl, genitive sg, dative sg, dative pl, accusative sg, accusative pl.
    declension!("χᾰρᾰξ", 10, "~ξ", [gender: Masculine], like Κορᾰξ;
        nominative: _, _, derived;
        genitive: derived, _, _;
        dative: derived, _, "~ξῐν";
        accusative: derived, _, derived;
        vocative: _, _, _;
    ),
    // Ἀληθεια: 10 words (ἀληθεια, ἀγνοια, ἀμελεια, ἀσεβεια, ἀγχουσα, ἀρουρα).
    declension!("ἀληθεια", 10, "ἀ~α";
        nominative: "ᾰ̓~ᾰ", "ᾰ̓~ᾱ", "ᾰ̓~αι";
        genitive: "ᾰ̓~ᾱς", "ᾰ̓~αιν", "ᾰ̓~ων";
        dative: "ᾰ̓~ᾳ", "ᾰ̓~αιν", "ᾰ̓~αις";
        accusative: "ᾰ̓~ᾰν", "ᾰ̓~ᾱ", "ᾰ̓~ᾱς";
        vocative: "ᾰ̓~ᾰ", "ᾰ̓~ᾱ", "ᾰ̓~αι";
    ),
    // Ὑψος: 10 words (ὑπνος, ὑψος, ὑδρος, ὑμνος, ὑσσωπος, ὑσκλος).
    declension!("ὑψος", 10, "ὑ~ος";
        nominative: "ῠ̔~ος", "ῠ̔~ω", "ῠ̔~οι";
        genitive: "ῠ̔~ου", "ῠ̔~οιν", "ῠ̔~ων";
        dative: "ῠ̔~ῳ", "ῠ̔~οιν", "ῠ̔~οις";
        accusative: "ῠ̔~ον", "ῠ̔~ω", "ῠ̔~ους";
        vocative: "ῠ̔~ε", "ῠ̔~ω", "ῠ̔~οι";
    ),
    // Ἀγκων: 10 words (ἀγων, ἀκμων, ἀδην, ἀρχων, ἀμβων, ἀκων).
    declension!("ἀγκων", 10, "ἀ~ν";
        nominative: "ᾰ̓~ν", "ᾰ̓~νε", "ᾰ̓~νες";
        genitive: "ᾰ̓~νος", "ᾰ̓~νοιν", "ᾰ̓~νων";
        dative: "ᾰ̓~νῐ", "ᾰ̓~νοιν", "ᾰ̓~σῐ";
        accusative: "ᾰ̓~νᾰ", "ᾰ̓~νε", "ᾰ̓~νᾰς";
        vocative: "ᾰ̓~ν", "ᾰ̓~νε", "ᾰ̓~νες";
    ),
    // Θηρ: 10 words (κᾱρ, χην, θηρ, ψᾱρ, φωρ, σηρ). Like Χᾰρᾰκτηρ, except in 2 cells: genitive du, dative du.
    declension!("θηρ", 10, "~", like Χᾰρᾰκτηρ;
        nominative: _, _, _;
        genitive: _, "~οῑν", _;
        dative: _, "~οῑν", _;
        accusative: _, _, _;
        vocative: _, _, _;
    ),
    // Τρωες: 10 words (πῑερες, τρωες, κηφηνες, αἰγες, ἀνᾰξῠρῐδες, ὠρῠγγες). Like Ὀργῐᾰ, except in 5 cells: nominative sg, nominative pl, dative pl, accusative pl, vocative pl.
    declension!("τρωες", 10, "~ες", like Ὀργῐᾰ;
        nominative: "~ες", _, "~ες";
        genitive: _, _, _;
        dative: _, _, "~σῐ";
        accusative: _, _, "~ᾰς";
        vocative: _, _, "~ες";
    ),
    // Λευκη: 10 words (πευκη, δουλη, βλαυτη, σενδουκη, λευκη, ζευγλη).
    declension!("λευκη", 10, "~υ~η", [gender: Feminine];
        nominative: "~υ~η", "~υ~ᾱ", "~ῡ~αι";
        genitive: "~υ~ης", "~υ~αιν", "~υ~ων";
        dative: "~υ~ῃ", "~υ~αιν", "~υ~αις";
        accusative: "~υ~ην", "~υ~ᾱ", "~υ~ᾱς";
        vocative: "~υ~η", "~υ~ᾱ", "~ῡ~αι";
    ),
    // Ἠχου: 10 words (ῥᾰββῑ, ῥᾰββονῑ, ῥᾰββουνῑ, κορβᾱν, βροτοῡ, βροτῳ).
    declension!("ἠχου", 10, "~";
        nominative: "~", derived, derived;
        genitive: "~", derived, derived;
        dative: "~", derived, derived;
        accusative: "~", derived, derived;
        vocative: "~", derived, derived;
    ),
    // Πεπερῐ: 9 words (σῐνᾱπῐ, κομμῐ, πεπερῐ, κιννᾰβᾰρῐ, στῐμμῐ, κῐκῐ). Like Δῐαιρεσῐς, except in 5 cells: nominative sg, nominative pl, accusative sg, accusative pl, vocative pl.
    declension!("πεπερῐ", 9, "~ῐ", [gender: Neuter], like Δῐαιρεσῐς;
        nominative: "~ῐ", _, "~η|~εᾰ";
        genitive: _, _, _;
        dative: _, _, _;
        accusative: "~ῐ", _, "~η|~εᾰ";
        vocative: _, _, "~η|~εᾰ";
    ),
    // Κοιτη: 9 words (χαιτη, κοιτη, ἐρεικη, θοινη, βαιτη, μαινη).
    declension!("κοιτη", 9, "~ι~η", [gender: Feminine];
        nominative: "~ι~η", "~ι~ᾱ", "~ῑ~αι";
        genitive: "~ι~ης", "~ι~αιν", "~ι~ων";
        dative: "~ι~ῃ", "~ι~αιν", "~ι~αις";
        accusative: "~ι~ην", "~ι~ᾱ", "~ι~ᾱς";
        vocative: "~ι~η", "~ι~ᾱ", "~ῑ~αι";
    ),
    // Ἀλειτης: 9 words (ὠρειτης, παικτης, σωρειτης, σᾰμᾰρειτης, ἀρσενοκοιτης, μητροκοιτης). Like Κοιτη, except in 3 cells: nominative sg, genitive sg, vocative sg.
    declension!("ἀλειτης", 9, "~ι~ης", [gender: Masculine], like Κοιτη;
        nominative: "~ι~ης", _, _;
        genitive: "~ι~ου", _, _;
        dative: _, _, _;
        accusative: _, _, _;
        vocative: "~ῑ~ᾰ", _, _;
    ),
    // Ἀμη: 9 words (ἀλκη, ἀποθηκη, ἀλοη, ἀμη, ἀρτοθηκη, ἀρμενοθηκη). Like Ἀρετη, except in 3 cells: genitive du, dative du, dative pl.
    declension!("ἀμη", 9, "ἀ~η", [gender: Feminine], like Ἀρετη;
        nominative: _, _, _;
        genitive: _, "ᾰ̓~αιν", _;
        dative: _, "ᾰ̓~αιν", "ᾰ̓~αις";
        accusative: _, _, _;
        vocative: _, _, _;
    ),
    // Πλᾰτεῑᾰ: 9 words (μυῑᾰ, πλᾰτεῑᾰ, γαῑᾰ, μαῑᾰ, βᾰρεῑᾰ, ὀξεῑᾰ).
    declension!("πλᾰτεῑᾰ", 9, "~ῑᾰ", [gender: Feminine];
        nominative: "~ῑᾰ", "~ιᾱ", "~ῑαι";
        genitive: "~ιᾱς", "~ιαιν", "~ιων";
        dative: "~ιᾳ", "~ιαιν", "~ιαις";
        accusative: "~ῑᾰν", "~ιᾱ", "~ιᾱς";
        vocative: "~ῑᾰ", "~ιᾱ", "~ῑαι";
    ),
    // Σησᾰμοειδες: 9 words (νευροειδες, μονοειδες, τρῐχομᾰνες, ἡμεροκᾰλλες, σησᾰμοειδες, ὠκῐμοειδες).
    declension!("σησᾰμοειδες", 9, "~ες", [gender: Neuter];
        nominative: "~ες", "~εῑ", "~η";
        genitive: "~οῡς", "~οῑν", "~ων";
        dative: "~εῑ", "~οῑν", "~εσῐ|~εσῐν";
        accusative: "~ες", "~εῑ", "~η";
        vocative: "~ες", "~εῑ", "~η";
    ),
    // Θῑς: 9 words (ῥῑς, ἰς, θῑς, ὠδῑς, γλωχῑς, στᾰμῑς). Like Αἰων, except in 2 cells: nominative sg, vocative sg.
    declension!("θῑς", 9, "~ς", like Αἰων;
        nominative: "~ς", _, _;
        genitive: _, _, _;
        dative: _, _, _;
        accusative: _, _, _;
        vocative: "~ς", _, _;
    ),
    // Ἀλωπεκιᾱ: 9 words (ἀβλεψιᾱ, ἀκηδιᾱ, ἀτροφιᾱ, ἀναιμιᾱ, ἀλωπεκιᾱ, ἀπουσιᾱ). Like Ἀμβροσια, except in 0 cells: .
    declension!("ἀλωπεκιᾱ", 9, "ἀ~ιᾱ", [gender: Feminine], like Ἀμβροσια;
        nominative: _, _, _;
        genitive: _, _, _;
        dative: _, _, _;
        accusative: _, _, _;
        vocative: _, _, _;
    ),
    // Πᾰτηρ: 8 words (μητηρ, πᾰτηρ, γᾰστηρ, θῠγᾰτηρ, φρᾱτηρ, ἐνᾰτηρ).
    declension!("πᾰτηρ", 8, "~η~";
        nominative: "~η~", "~ε~ε", "~ε~ες";
        genitive: "~~ος", "~ε~οιν", "~ε~ων";
        dative: "~~ῐ", "~ε~οιν", "~~ᾰσῐ";
        accusative: "~ε~ᾰ", "~ε~ε", "~ε~ᾰς";
        vocative: "~ε~", "~ε~ε", "~ε~ες";
    ),
    // Νεῡρον: 8 words (προῡμνον, νεῡρον, βοῡστον, τεῡτλον, σοῡσον, λοῡσσον). Like Ναῡλος, except in 5 cells: nominative sg, nominative pl, accusative pl, vocative sg, vocative pl.
    declension!("νεῡρον", 8, "~ῡ~ον", [gender: Neuter], like Ναῡλος;
        nominative: "~ῡ~ον", _, "~ῡ~ᾰ";
        genitive: _, _, _;
        dative: _, _, _;
        accusative: _, _, "~ῡ~ᾰ";
        vocative: "~ῡ~ον", _, "~ῡ~ᾰ";
    ),
    // Ἠχω: 8 words (ἠχω, τυτω, κῐττω, πῑπω, μοτω, ὁδω). Like Μερῐσμος, except in 5 cells: nominative sg, genitive sg, dative sg, accusative sg, vocative sg.
    declension!("ἠχω", 8, "~ω", [gender: Feminine], like Μερῐσμος;
        nominative: "~ω", _, _;
        genitive: "~οῡς", _, _;
        dative: "~οῑ", _, _;
        accusative: "~ω", _, _;
        vocative: "~οῑ", _, _;
    ),
    // Ἀφεσῐς: 8 words (ἀρδῐς, ἀρῐς, ἀκουσῐς, ἀλγησῐς, ἀποκρῐσῐς, ἀποσῐωπησῐς). Like Ἀμαυρωσις, except in 0 cells: .
    declension!("ἀφεσῐς", 8, "ἀ~ῐς", [gender: Feminine], like Ἀμαυρωσις;
        nominative: _, _, _;
        genitive: _, _, _;
        dative: _, _, _;
        accusative: _, _, _;
        vocative: _, _, _;
    ),
    // Ῥᾱξ: 7 words (φλοξ, αἰξ, σφᾰξ, ῥᾱξ, στρῐξ, ῥωξ). Like Πτερῠξ, except in 2 cells: genitive du, dative du.
    declension!("ῥᾱξ", 7, "~ξ", like Πτερῠξ;
        nominative: _, _, _;
        genitive: _, "~γοῑν", _;
        dative: _, "~γοῑν", _;
        accusative: _, _, _;
        vocative: _, _, _;
    ),
    // Φρην: 7 words (φρην, ποιμην, πῠθμην, αὐχην, λῐμην, ἀζην).
    declension!("φρην", 7, "~ην";
        nominative: "~ην", "~ενε", "~ενες";
        genitive: "~ενος", "~ενοιν", "~ενων";
        dative: "~ενῐ", "~ενοιν", "~εσῐ|~εσῐν";
        accusative: "~ενᾰ", "~ενε", "~ενᾰς";
        vocative: "~ην", "~ενε", "~ενες";
    ),
    // Ἀντονομασια: 7 words (ἀνθρακια, ἀναρχια, ἀντονομασια, ἀνωμαλια, ἀχρηματια, ἀμαθια).
    declension!("ἀντονομασια", 7, "ἀ~α~ια", [gender: Feminine];
        nominative: "ᾰ̓~ᾰ~ῐᾱ", "ᾰ̓~ᾰ~ῐᾱ", "ᾰ̓~ᾰ~ῐαι";
        genitive: "ᾰ̓~ᾰ~ῐᾱς", "ᾰ̓~ᾰ~ῐαιν", "ᾰ̓~ᾰ~ῐων";
        dative: "ᾰ̓~ᾰ~ῐᾳ", "ᾰ̓~ᾰ~ῐαιν", "ᾰ̓~ᾰ~ῐαις";
        accusative: "ᾰ̓~ᾰ~ῐᾱν", "ᾰ̓~ᾰ~ῐᾱ", "ᾰ̓~ᾰ~ῐᾱς";
        vocative: "ᾰ̓~ᾰ~ῐᾱ", "ᾰ̓~ᾰ~ῐᾱ", "ᾰ̓~ᾰ~ῐαι";
    ),
    // Μωλῠ: 7 words (δᾰκρῠ, μωλῠ, νᾱπῠ, μῐσῠ, βρᾰθῠ, σωρῠ). Like Στᾰχῠς, except in 5 cells: nominative sg, nominative pl, accusative sg, accusative pl, vocative pl.
    declension!("μωλῠ", 7, "~ῠ", [gender: Neuter], like Στᾰχῠς;
        nominative: "~ῠ", _, "~ῠᾰ";
        genitive: _, _, _;
        dative: _, _, _;
        accusative: "~ῠ", _, "~ῠᾰ";
        vocative: _, _, "~ῠᾰ";
    ),
    // Ὑδρα: 7 words (ὑαινα, ὑδρα, ὑστερα, ὑπερῳα, ὑποτεινουσα, ὑστεραια).
    declension!("ὑδρα", 7, "ὑ~α", [gender: Feminine];
        nominative: "ῠ̔~ᾱ", "ῠ̔~ᾱ", "ῠ̔~αι";
        genitive: "ῠ̔~ᾱς", "ῠ̔~αιν", "ῠ̔~ων";
        dative: "ῠ̔~ᾳ", "ῠ̔~αιν", "ῠ̔~αις";
        accusative: "ῠ̔~ᾱν", "ῠ̔~ᾱ", "ῠ̔~ᾱς";
        vocative: "ῠ̔~ᾱ", "ῠ̔~ᾱ", "ῠ̔~αι";
    ),
    // Χεῑμᾰ: 7 words (χεῑμᾰ, δεῑγμᾰ, σεῑσμᾰ, λεῑμμᾰ, λαῑτμᾰ, πεῑσμᾰ).
    declension!("χεῑμᾰ", 7, "~ῑ~", [gender: Neuter];
        nominative: "~ῑ~", "~ι~τε", "~ι~τᾰ";
        genitive: "~ι~τος", "~ι~τοιν", "~ι~των";
        dative: "~ι~τῐ", "~ι~τοιν", "~ι~σῐ|~ι~σῐν";
        accusative: "~ῑ~", "~ι~τε", "~ι~τᾰ";
        vocative: "~ῑ~", "~ι~τε", "~ι~τᾰ";
    ),
    // Ἀναγωγη: 7 words (ἀστραπη, ἀνατροπη, ἀνατροφη, ἀναγωγη, ἀναστροφη, ἀναβολη).
    declension!("ἀναγωγη", 7, "ἀ~α~η", [gender: Feminine];
        nominative: "ᾰ̓~ᾰ~η", "ᾰ̓~ᾰ~ᾱ", "ᾰ̓~ᾰ~αι";
        genitive: "ᾰ̓~ᾰ~ης", "ᾰ̓~ᾰ~αῑν", "ᾰ̓~ᾰ~ων";
        dative: "ᾰ̓~ᾰ~ῃ", "ᾰ̓~ᾰ~αῑν", "ᾰ̓~ᾰ~αῑς";
        accusative: "ᾰ̓~ᾰ~ην", "ᾰ̓~ᾰ~ᾱ", "ᾰ̓~ᾰ~ᾱς";
        vocative: "ᾰ̓~ᾰ~η", "ᾰ̓~ᾰ~ᾱ", "ᾰ̓~ᾰ~αι";
    ),
    // Ἀδᾰμᾱς: 7 words (ἀπους, ἀδᾰμᾱς, ἀδελφοτης, ἀδηλοτης, ἀμαυροτης, ἀλῠς).
    declension!("ἀδᾰμᾱς", 7, "ἀ~ς";
        nominative: "ᾰ̓~ς", "ᾰ̓~τε", "ᾰ̓~τες";
        genitive: "ᾰ̓~τος", "ᾰ̓~τοιν", "ᾰ̓~των";
        dative: "ᾰ̓~τῐ", "ᾰ̓~τοιν", "ᾰ̓~σῐ|ᾰ̓~σῐν";
        accusative: "ᾰ̓~τᾰ", "ᾰ̓~τε", "ᾰ̓~τᾰς";
        vocative: "ᾰ̓~ς", "ᾰ̓~τε", "ᾰ̓~τες";
    ),
    // Ἀναθεμα: 7 words (ἀκανθα, ἀσφαλεια, ἀκρατεια, ἀγαλμα, ἀναθεμα, ἀναστημα).
    declension!("ἀναθεμα", 7, "ἀ~α~α";
        nominative: "ᾰ̓~ᾰ~ᾰ", "ᾰ̓~ᾰ~ᾰτε", "ᾰ̓~ᾰ~ᾰτᾰ";
        genitive: "ᾰ̓~ᾰ~ᾰτος", "ᾰ̓~ᾰ~ᾰτοιν", "ᾰ̓~ᾰ~ᾰτων";
        dative: "ᾰ̓~ᾰ~ᾰτῐ", "ᾰ̓~ᾰ~ᾰτοιν", "ᾰ̓~ᾰ~ᾰσῐ|ᾰ̓~ᾰ~ᾰσῐν";
        accusative: "ᾰ̓~ᾰ~ᾰ", "ᾰ̓~ᾰ~ᾰτε", "ᾰ̓~ᾰ~ᾰτᾰ";
        vocative: "ᾰ̓~ᾰ~ᾰ", "ᾰ̓~ᾰ~ᾰτε", "ᾰ̓~ᾰ~ᾰτᾰ";
    ),
    // Βᾰσκᾱς: 7 words (πᾰξᾰμᾱς, μᾰρῐκᾱς, ἀττᾰγᾱς, μᾰσουχᾱς, βᾰσκᾱς, χαλιφᾱς). Like Φορᾱ, except in 4 cells: nominative sg, nominative pl, genitive sg, vocative pl.
    declension!("βᾰσκᾱς", 7, "~ᾱς", [gender: Masculine], like Φορᾱ;
        nominative: "~ᾱς", _, "~αῑ";
        genitive: "~ᾱ", _, _;
        dative: _, _, _;
        accusative: _, _, _;
        vocative: _, _, "~αῑ";
    ),
    // Ἀργεστης: 7 words (ἀητης, ἀργοναυτης, ἀποδημητης, ἀβροτονῑτης, ἀρχῐτελωνης, ἀγορητης). Like Ἀμη, except in 3 cells: nominative sg, genitive sg, vocative sg.
    declension!("ἀργεστης", 7, "ἀ~ης", [gender: Masculine], like Ἀμη;
        nominative: "ᾰ̓~ης", _, _;
        genitive: "ᾰ̓~ου", _, _;
        dative: _, _, _;
        accusative: _, _, _;
        vocative: "ᾰ̓~ᾰ", _, _;
    ),
    // Ἀραχνη: 6 words (ἀγαπη, ἀπατη, ἀναγκη, ἀραχνη, ἀγκαλη, ἀφακη). Like Ἀναγωγη, except in 3 cells: genitive du, dative du, dative pl.
    declension!("ἀραχνη", 6, "ἀ~α~η", [gender: Feminine], like Ἀναγωγη;
        nominative: _, _, _;
        genitive: _, "ᾰ̓~ᾰ~αιν", _;
        dative: _, "ᾰ̓~ᾰ~αιν", "ᾰ̓~ᾰ~αις";
        accusative: _, _, _;
        vocative: _, _, _;
    ),
    // Μοῑρᾰ: 6 words (μοῑρᾰ, στεῑρᾰ, πεῑρᾰ, σφαῑρᾰ, σπεῑρᾰ, ἀγρεῑφνᾰ). Like Ἀλειτης, except in 4 cells: nominative sg, genitive sg, dative sg, accusative sg.
    declension!("μοῑρᾰ", 6, "~ῑ~ᾰ", [gender: Feminine], like Ἀλειτης;
        nominative: "~ῑ~ᾰ", _, _;
        genitive: "~ι~ᾱς", _, _;
        dative: "~ι~ᾳ", _, _;
        accusative: "~ῑ~ᾰν", _, _;
        vocative: _, _, _;
    ),
    // Ἀγορα: 6 words (ἀκρα, ἀγορα, ἀγρα, ἀνδρεια, ἀρα, ἀλεα). Like Ἀληθεια, except in 3 cells: nominative sg, accusative sg, vocative sg.
    declension!("ἀγορα", 6, "ἀ~α", [gender: Feminine], like Ἀληθεια;
        nominative: "ᾰ̓~ᾱ", _, _;
        genitive: _, _, _;
        dative: _, _, _;
        accusative: "ᾰ̓~ᾱν", _, _;
        vocative: "ᾰ̓~ᾱ", _, _;
    ),
    // Ἀλῐβᾱς: 6 words (κορῠβᾱς, γῐγᾱς, ἐλεφᾱς, λῠκᾰβᾱς, ἀλῐβᾱς, γᾰρᾰμᾱς).
    declension!("ἀλῐβᾱς", 6, "~ᾱς", [gender: Masculine];
        nominative: "~ᾱς", "~ᾰντε", "~ᾰντες";
        genitive: "~ᾰντος", "~ᾰντοιν", "~ᾰντων";
        dative: "~ᾰντῐ", "~ᾰντοιν", "~ᾱσῐ|~ᾱσῐν";
        accusative: "~ᾰντᾰ", "~ᾰντε", "~ᾰντᾰς";
        vocative: "~ᾰν", "~ᾰντε", "~ᾰντες";
    ),
    // Μητρως: 6 words (τρως, ἡρως, δμως, πᾰτρως, θως, μητρως).
    declension!("μητρως", 6, "~ως", [gender: Masculine];
        nominative: "~ως", "~ωε", "~ωες";
        genitive: "~ωος", "~ωοιν", "~ωων";
        dative: "~ῳ", "~ωοιν", "~ωσῐ|~ωσῐν";
        accusative: "~ω", "~ωε", "~ωᾰς";
        vocative: "~ως", "~ωε", "~ωες";
    ),
    // Ἱστος: 6 words (ἱππος, ἱπποκενταυρος, ἱστος, ἱπποδρομος, ἱπποκανθαρος, ἱππουρος).
    declension!("ἱστος", 6, "ἱ~ος", [gender: Masculine];
        nominative: "ῐ̔~ος", "ῐ̔~ω", "ῐ̔~οι";
        genitive: "ῐ̔~ου", "ῐ̔~οιν", "ῐ̔~ων";
        dative: "ῐ̔~ῳ", "ῐ̔~οιν", "ῐ̔~οις";
        accusative: "ῐ̔~ον", "ῐ̔~ω", "ῐ̔~ους";
        vocative: "ῐ̔~ε", "ῐ̔~ω", "ῐ̔~οι";
    ),
    // Ἀμεθυστος: 6 words (ἀργυρος, ἀνδρογυνος, ἀμεθυστος, ἀργυροτοξος, ἀργυροκοπος, ἀστυνομος).
    declension!("ἀμεθυστος", 6, "ἀ~υ~ος";
        nominative: "ᾰ̓~ῠ~ος", "ᾰ̓~ῠ~ω", "ᾰ̓~ῠ~οι";
        genitive: "ᾰ̓~ῠ~ου", "ᾰ̓~ῠ~οιν", "ᾰ̓~ῠ~ων";
        dative: "ᾰ̓~ῠ~ῳ", "ᾰ̓~ῠ~οιν", "ᾰ̓~ῠ~οις";
        accusative: "ᾰ̓~ῠ~ον", "ᾰ̓~ῠ~ω", "ᾰ̓~ῠ~ους";
        vocative: "ᾰ̓~ῠ~ε", "ᾰ̓~ῠ~ω", "ᾰ̓~ῠ~οι";
    ),
    // Κῠᾰρ: 6 words (οὐθᾰρ, ἡπᾰρ, φρεᾱρ, στεᾱρ, δελεᾰρ, κῠᾰρ). Like Θεμᾰ, except in 3 cells: nominative sg, accusative sg, vocative sg.
    declension!("κῠᾰρ", 6, "~ρ", [gender: Neuter], like Θεμᾰ;
        nominative: "~ρ", _, _;
        genitive: _, _, _;
        dative: _, _, _;
        accusative: "~ρ", _, _;
        vocative: "~ρ", _, _;
    ),
    // Ὑπᾰκοη: 6 words (ὑφη, ὑπομονη, ὑποθηκη, ὑποδοχη, ὑρχη, ὑπᾰκοη).
    declension!("ὑπᾰκοη", 6, "ὑ~η", [gender: Feminine];
        nominative: "ῠ̔~η", "ῠ̔~ᾱ", "ῠ̔~αι";
        genitive: "ῠ̔~ης", "ῠ̔~αῑν", "ῠ̔~ων";
        dative: "ῠ̔~ῃ", "ῠ̔~αῑν", "ῠ̔~αῑς";
        accusative: "ῠ̔~ην", "ῠ̔~ᾱ", "ῠ̔~ᾱς";
        vocative: "ῠ̔~η", "ῠ̔~ᾱ", "ῠ̔~αι";
    ),
    // Ναυτης: 6 words (ναυτης, θραυστης, κοκκοθραυστης, ἐμφῠτευτης, ψευστης, πῠραυστης). Like Λευκη, except in 3 cells: nominative sg, genitive sg, vocative sg.
    declension!("ναυτης", 6, "~υ~ης", [gender: Masculine], like Λευκη;
        nominative: "~υ~ης", _, _;
        genitive: "~υ~ου", _, _;
        dative: _, _, _;
        accusative: _, _, _;
        vocative: "~ῡ~ᾰ", _, _;
    ),
    // Ὑποθεσις: 6 words (ὑποθεσις, ὑβρις, ὑφηγησις, ὑποσχεσις, ὑφαιρεσις, ὑποληψις).
    declension!("ὑποθεσις", 6, "ὑ~ις", [gender: Feminine];
        nominative: "ῠ̔~ῐς", "ῠ̔~ει", "ῠ̔~εις";
        genitive: "ῠ̔~εως", "ῠ̔~εοιν", "ῠ̔~εων";
        dative: "ῠ̔~ει", "ῠ̔~εοιν", "ῠ̔~εσῐ|ῠ̔~εσῐν";
        accusative: "ῠ̔~ῐν", "ῠ̔~ει", "ῠ̔~εις";
        vocative: "ῠ̔~ῐ", "ῠ̔~ει", "ῠ̔~εις";
    ),
    // Ἀσπις: 6 words (ἀσπις, ἀγρωστις, ἀκις, ἀμις, ἀκροστῐχις, ἀνθεμις).
    declension!("ἀσπις", 6, "ἀ~ις", [gender: Feminine];
        nominative: "ᾰ̓~ῐς", "ᾰ̓~ῐδε", "ᾰ̓~ῐδες";
        genitive: "ᾰ̓~ῐδος", "ᾰ̓~ῐδοιν", "ᾰ̓~ῐδων";
        dative: "ᾰ̓~ῐδῐ", "ᾰ̓~ῐδοιν", "ᾰ̓~ῐσῐ|ᾰ̓~ῐσῐν";
        accusative: "ᾰ̓~ῐδᾰ", "ᾰ̓~ῐδε", "ᾰ̓~ῐδᾰς";
        vocative: "ᾰ̓~ῐ", "ᾰ̓~ῐδε", "ᾰ̓~ῐδες";
    ),
    // Πηχῠς: 6 words (ἡμῐσῠς, υἱῠς, πελεκῠς, πρεσβῠς, πηχῠς, τρῐβρᾰχῠς). Like Δῐαιρεσῐς, except in 3 cells: nominative sg, accusative sg, vocative sg.
    declension!("πηχῠς", 6, "~ῠς", [gender: Masculine], like Δῐαιρεσῐς;
        nominative: "~ῠς", _, _;
        genitive: _, _, _;
        dative: _, _, _;
        accusative: "~ῠν", _, _;
        vocative: "~ῠ", _, _;
    ),
    // Ὑπερῳον: 6 words (ὑπογειον, ὑπερῳον, ὑπνον, ὑπηκοον, ὑδνον, ὑποκαυστον). Like Ὑψος, except in 5 cells: nominative sg, nominative pl, accusative pl, vocative sg, vocative pl.
    declension!("ὑπερῳον", 6, "ὑ~ον", [gender: Neuter], like Ὑψος;
        nominative: "ῠ̔~ον", _, "ῠ̔~ᾰ";
        genitive: _, _, _;
        dative: _, _, _;
        accusative: _, _, "ῠ̔~ᾰ";
        vocative: "ῠ̔~ον", _, "ῠ̔~ᾰ";
    ),
    // Παῡλᾰ: 6 words (μοῡσᾰ, βοῡλλᾰ, πᾰνδοῡρᾰ, παῡλᾰ, γοῡρνᾰ, κοῡπᾰ). Like Λευκη, except in 3 cells: nominative sg, accusative sg, vocative sg.
    declension!("παῡλᾰ", 6, "~ῡ~ᾰ", [gender: Feminine], like Λευκη;
        nominative: "~ῡ~ᾰ", _, _;
        genitive: _, _, _;
        dative: _, _, _;
        accusative: "~ῡ~ᾰν", _, _;
        vocative: "~ῡ~ᾰ", _, _;
    ),
    // Ἡρωϊς: 6 words (ὀϊς, ἡρωϊς, ἐχενηϊς, ληϊς, ἀχερωϊς, αἱμορροϊς).
    declension!("ἡρωϊς", 6, "~ϊς";
        nominative: "~ῐ̈ς", "~ῐ̈δε", "~ῐ̈δες";
        genitive: "~ῐ̈δος", "~ῐ̈δοιν", "~ῐ̈δων";
        dative: "~ῐ̈δῐ", "~ῐ̈δοιν", "~ῐ̈σῐ|~ῐ̈σῐν";
        accusative: "~ῐ̈δᾰ", "~ῐ̈δε", "~ῐ̈δᾰς";
        vocative: "~ῐ̈", "~ῐ̈δε", "~ῐ̈δες";
    ),
    // Γεῑσον: 6 words (δεῑπνον, σπεῑρον, κλεῑθρον, σεῑστρον, ῥεῑθρον, γεῑσον). Like Στοῑχος, except in 5 cells: nominative sg, nominative pl, accusative pl, vocative sg, vocative pl.
    declension!("γεῑσον", 6, "~ῑ~ον", [gender: Neuter], like Στοῑχος;
        nominative: "~ῑ~ον", _, "~ῑ~ᾰ";
        genitive: _, _, _;
        dative: _, _, _;
        accusative: _, _, "~ῑ~ᾰ";
        vocative: "~ῑ~ον", _, "~ῑ~ᾰ";
    ),
    // Πᾰπᾱς: 6 words (πᾰπᾱς, γᾰλᾰτᾱς, κᾰδῡτᾱς, γενετᾱς, σᾰκτᾱς, πᾰππᾱς). Like Κοχλῐᾱς, except in 1 cell: vocative sg.
    declension!("πᾰπᾱς", 6, "~ᾱς", [gender: Masculine], like Κοχλῐᾱς;
        nominative: _, _, _;
        genitive: _, _, _;
        dative: _, _, _;
        accusative: _, _, _;
        vocative: "~ᾰ", _, _;
    ),
    // Ἀλκαιᾱ: 6 words (ἀνῑᾱ, ἀγγᾰρειᾱ, ἀμφῐβολῐᾱ, ἀκρῐτομῡθιᾱ, ἀλκαιᾱ, ἀλκεᾱ). Like Ἀγορα, except in 0 cells: .
    declension!("ἀλκαιᾱ", 6, "ἀ~ᾱ", [gender: Feminine], like Ἀγορα;
        nominative: _, _, _;
        genitive: _, _, _;
        dative: _, _, _;
        accusative: _, _, _;
        vocative: _, _, _;
    ),
    // Γερᾰς: 6 words (γερᾰς, δεπᾰς, ψεφᾰς, σφελᾰς, βουκερᾰς, σκεπᾰς). Like Κηρ, except in 5 cells: nominative sg, dative sg, dative pl, accusative sg, vocative sg.
    declension!("γερᾰς", 6, "~ς", [gender: Neuter], like Κηρ;
        nominative: "~ς", _, _;
        genitive: _, _, _;
        dative: "~ῐ̈", _, "~σῐ";
        accusative: "~ς", _, _;
        vocative: "~ς", _, _;
    ),
    // Σφην: 5 words (μην, σπλην, κλων, σφην, ψην). Like Αἰων, except in 2 cells: genitive du, dative du.
    declension!("σφην", 5, "~ν", [gender: Masculine], like Αἰων;
        nominative: _, _, _;
        genitive: _, "~νοῑν", _;
        dative: _, "~νοῑν", _;
        accusative: _, _, _;
        vocative: _, _, _;
    ),
    // Ἀνατασις: 5 words (ἀναστασις, ἀγανακτησις, ἀναταξις, ἀνατασις, ἀναβασις).
    declension!("ἀνατασις", 5, "ἀ~α~α~ις", [gender: Feminine];
        nominative: "ᾰ̓~ᾰ~ᾰ~ῐς", "ᾰ̓~ᾰ~ᾰ~ει", "ᾰ̓~ᾰ~ᾰ~εις";
        genitive: "ᾰ̓~ᾰ~ᾰ~εως", "ᾰ̓~ᾰ~ᾰ~εοιν", "ᾰ̓~ᾰ~ᾰ~εων";
        dative: "ᾰ̓~ᾰ~ᾰ~ει", "ᾰ̓~ᾰ~ᾰ~εοιν", "ᾰ̓~ᾰ~ᾰ~εσῐ|ᾰ̓~ᾰ~ᾰ~εσῐν";
        accusative: "ᾰ̓~ᾰ~ᾰ~ῐν", "ᾰ̓~ᾰ~ᾰ~ει", "ᾰ̓~ᾰ~ᾰ~εις";
        vocative: "ᾰ̓~ᾰ~ᾰ~ῐ", "ᾰ̓~ᾰ~ᾰ~ει", "ᾰ̓~ᾰ~ᾰ~εις";
    ),
    // Κωμῡς: 5 words (μᾰρτῠς, νομᾰς, κωμῡς, φῠγᾰς, ἐμῠς). Like Κλεις, except in 6 cells: nominative pl, genitive sg, dative sg, dative pl, accusative sg, accusative pl.
    declension!("κωμῡς", 5, "~ς", [gender: Masculine], like Κλεις;
        nominative: _, _, derived;
        genitive: derived, _, _;
        dative: derived, _, "~σῐν";
        accusative: derived, _, derived;
        vocative: _, _, _;
    ),
    // Ἀνισον: 5 words (ἀλφιτον, ἀνισον, ἀγκιστρον, ἀντιθετον, ἀριστερον).
    declension!("ἀνισον", 5, "ἀ~ι~ον", [gender: Neuter];
        nominative: "ᾰ̓~ῐ~ον", "ᾰ̓~ῐ~ω", "ᾰ̓~ῐ~ᾰ";
        genitive: "ᾰ̓~ῐ~ου", "ᾰ̓~ῐ~οιν", "ᾰ̓~ῐ~ων";
        dative: "ᾰ̓~ῐ~ῳ", "ᾰ̓~ῐ~οιν", "ᾰ̓~ῐ~οις";
        accusative: "ᾰ̓~ῐ~ον", "ᾰ̓~ῐ~ω", "ᾰ̓~ῐ~ᾰ";
        vocative: "ᾰ̓~ῐ~ον", "ᾰ̓~ῐ~ω", "ᾰ̓~ῐ~ᾰ";
    ),
    // Γεῡσῐς: 5 words (παῡσῐς, λοῡσῐς, ῥεῡσῐς, ζεῡξῐς, γεῡσῐς).
    declension!("γεῡσῐς", 5, "~ῡ~ῐς", [gender: Feminine];
        nominative: "~ῡ~ῐς", "~υ~ει", "~υ~εις";
        genitive: "~υ~εως", "~υ~εοιν", "~υ~εων";
        dative: "~υ~ει", "~υ~εοιν", "~υ~εσῐ|~υ~εσῐν";
        accusative: "~ῡ~ῐν", "~υ~ει", "~υ~εις";
        vocative: "~ῡ~ῐ", "~υ~ει", "~υ~εις";
    ),
    // Νοῡς: 5 words (νοῡς, πλοῡς, θῠγατρῐδοῡς, ῥοῡς, λουτροχοῡς).
    declension!("νοῡς", 5, "~οῡς", [gender: Masculine];
        nominative: "~οῡς", "~ω|~ω", "~οῑ";
        genitive: "~οῡ", "~οῑν", "~ων";
        dative: "~ῳ", "~οῑν", "~οῑς";
        accusative: "~οῡν", "~ω|~ω", "~οῡς";
        vocative: "~οῡ", "~ω|~ω", "~οῑ";
    ),
    // Λῐψ: 5 words (χᾰλῠψ, φλεψ, χερνῐψ, φᾰψ, λῐψ).
    declension!("λῐψ", 5, "~ψ";
        nominative: "~ψ", "~βε", "~βες";
        genitive: "~βος", "~βοῑν", "~βων";
        dative: "~βῐ", "~βοῑν", "~ψῐ|~ψῐν";
        accusative: "~βᾰ", "~βε", "~βᾰς";
        vocative: "~ψ", "~βε", "~βες";
    ),
    // Δαιμων: 5 words (δαιμων, μειζων, γειτων, ποτᾰμογειτων, σεισων).
    declension!("δαιμων", 5, "~ι~ων", [gender: Masculine];
        nominative: "~ι~ων", "~ι~ονε", "~ι~ονες";
        genitive: "~ι~ονος", "~ι~ονοιν", "~ι~ονων";
        dative: "~ι~ονῐ", "~ι~ονοιν", "~ι~οσῐ|~ι~οσῐν";
        accusative: "~ι~ονᾰ", "~ι~ονε", "~ι~ονᾰς";
        vocative: "~ῑ~ον", "~ι~ονε", "~ι~ονες";
    ),
    // Ἀστραγᾰλος: 5 words (ἀρακος, ἀστραγᾰλος, ἀστακος, ἀραβος, ἀμαθος).
    declension!("ἀστραγᾰλος", 5, "ἀ~α~ος";
        nominative: "ᾰ̓~ᾰ~ος", "ᾰ̓~ᾰ~ω", "ᾰ̓~ᾰ~οι";
        genitive: "ᾰ̓~ᾰ~ου", "ᾰ̓~ᾰ~οιν", "ᾰ̓~ᾰ~ων";
        dative: "ᾰ̓~ᾰ~ῳ", "ᾰ̓~ᾰ~οιν", "ᾰ̓~ᾰ~οις";
        accusative: "ᾰ̓~ᾰ~ον", "ᾰ̓~ᾰ~ω", "ᾰ̓~ᾰ~ους";
        vocative: "ᾰ̓~ᾰ~ε", "ᾰ̓~ᾰ~ω", "ᾰ̓~ᾰ~οι";
    ),
    // Τεῡχος: 5 words (τεῡχος, γλεῡκος, ψεῡδος, ζεῡγος, κεῡθος).
    declension!("τεῡχος", 5, "~ῡ~ος", [gender: Neuter];
        nominative: "~ῡ~ος", "~υ~ει", "~υ~η";
        genitive: "~υ~ους", "~υ~οῑν", "~υ~ων";
        dative: "~υ~ει", "~υ~οῑν", "~υ~εσῐ|~υ~εσῐν";
        accusative: "~ῡ~ος", "~υ~ει", "~υ~η";
        vocative: "~ῡ~ος", "~υ~ει", "~υ~η";
    ),
    // Χλουνης: 5 words (τῠμβαυλης, χλουνης, σῐδευνης, χοραυλης, πρωταυλης). Like Ναυτης, except in 1 cell: vocative sg.
    declension!("χλουνης", 5, "~υ~ης", [gender: Masculine], like Ναυτης;
        nominative: _, _, _;
        genitive: _, _, _;
        dative: _, _, _;
        accusative: _, _, _;
        vocative: "~υ~η", _, _;
    ),
    // Ἀγρευς: 5 words (ἀμφορευς, ἀποστολευς, ἀγρευς, ἀλθευς, ἀμφῐφορευς).
    declension!("ἀγρευς", 5, "ἀ~ευς", [gender: Masculine];
        nominative: "ᾰ̓~ευς", "ᾰ̓~η", "ᾰ̓~ης|ᾰ̓~εῑς";
        genitive: "ᾰ̓~εως", "ᾰ̓~εοιν", "ᾰ̓~εων";
        dative: "ᾰ̓~εῑ", "ᾰ̓~εοιν", "ᾰ̓~εῡσῐ|ᾰ̓~εῡσῐν";
        accusative: "ᾰ̓~εᾱ", "ᾰ̓~η", "ᾰ̓~εᾱς";
        vocative: "ᾰ̓~εῡ", "ᾰ̓~η", "ᾰ̓~ης|ᾰ̓~εῑς";
    ),
    // Λεῑμᾰξ: 5 words (χοῑνῐξ, κᾰταῑτῠξ, μεῑρᾰξ, λεῑμᾰξ, βαῑβῠξ).
    declension!("λεῑμᾰξ", 5, "~ῑ~ξ";
        nominative: "~ῑ~ξ", "~ι~κε", "~ι~κες";
        genitive: "~ι~κος", "~ι~κοιν", "~ι~κων";
        dative: "~ι~κῐ", "~ι~κοιν", "~ι~ξῐ|~ι~ξῐν";
        accusative: "~ι~κᾰ", "~ι~κε", "~ι~κᾰς";
        vocative: "~ῑ~ξ", "~ι~κε", "~ι~κες";
    ),
    // Λαυρᾱ: 4 words (σαυρᾱ, λαυρᾱ, κᾰρουχᾱ, βρουτζᾱ). Like Λευκη, except in 5 cells: nominative sg, genitive sg, dative sg, accusative sg, vocative sg.
    declension!("λαυρᾱ", 4, "~υ~ᾱ", [gender: Feminine], like Λευκη;
        nominative: "~υ~ᾱ", _, _;
        genitive: "~υ~ᾱς", _, _;
        dative: "~υ~ᾳ", _, _;
        accusative: "~υ~ᾱν", _, _;
        vocative: "~υ~ᾱ", _, _;
    ),
    // Ἰσηγορια: 4 words (ἰδια, ἰσονομια, ἰσοψηφια, ἰσηγορια).
    declension!("ἰσηγορια", 4, "ἰ~ια", [gender: Feminine];
        nominative: "ῐ̓~ῐᾱ", "ῐ̓~ῐᾱ", "ῐ̓~ῐαι";
        genitive: "ῐ̓~ῐᾱς", "ῐ̓~ῐαιν", "ῐ̓~ῐων";
        dative: "ῐ̓~ῐᾳ", "ῐ̓~ῐαιν", "ῐ̓~ῐαις";
        accusative: "ῐ̓~ῐᾱν", "ῐ̓~ῐᾱ", "ῐ̓~ῐᾱς";
        vocative: "ῐ̓~ῐᾱ", "ῐ̓~ῐᾱ", "ῐ̓~ῐαι";
    ),
    // Πλευμων: 4 words (πλευμων, πνευμων, μουσμων, ἰχνευμων).
    declension!("πλευμων", 4, "~υ~ων", [gender: Masculine];
        nominative: "~υ~ων", "~υ~ονε", "~υ~ονες";
        genitive: "~υ~ονος", "~υ~ονοιν", "~υ~ονων";
        dative: "~υ~ονῐ", "~υ~ονοιν", "~υ~οσῐ|~υ~οσῐν";
        accusative: "~υ~ονᾰ", "~υ~ονε", "~υ~ονᾰς";
        vocative: "~ῡ~ον", "~υ~ονε", "~υ~ονες";
    ),
    // Ὑπομνημα: 4 words (ὑποστρωμα, ὑπομνημα, ὑποδημα, ὑσμα).
    declension!("ὑπομνημα", 4, "ὑ~α", [gender: Neuter];
        nominative: "ῠ̔~ᾰ", "ῠ̔~ᾰτε", "ῠ̔~ᾰτᾰ";
        genitive: "ῠ̔~ᾰτος", "ῠ̔~ᾰτοιν", "ῠ̔~ᾰτων";
        dative: "ῠ̔~ᾰτῐ", "ῠ̔~ᾰτοιν", "ῠ̔~ᾰσῐ|ῠ̔~ᾰσῐν";
        accusative: "ῠ̔~ᾰ", "ῠ̔~ᾰτε", "ῠ̔~ᾰτᾰ";
        vocative: "ῠ̔~ᾰ", "ῠ̔~ᾰτε", "ῠ̔~ᾰτᾰ";
    ),
    // Ὑπαρξις: 4 words (ὑπαρξις, ὑποταξις, ὑπερταξις, ὑπογραφις).
    declension!("ὑπαρξις", 4, "ὑ~α~ις", [gender: Feminine];
        nominative: "ῠ̔~ᾰ~ῐς", "ῠ̔~ᾰ~ει", "ῠ̔~ᾰ~εις";
        genitive: "ῠ̔~ᾰ~εως", "ῠ̔~ᾰ~εοιν", "ῠ̔~ᾰ~εων";
        dative: "ῠ̔~ᾰ~ει", "ῠ̔~ᾰ~εοιν", "ῠ̔~ᾰ~εσῐ|ῠ̔~ᾰ~εσῐν";
        accusative: "ῠ̔~ᾰ~ῐν", "ῠ̔~ᾰ~ει", "ῠ̔~ᾰ~εις";
        vocative: "ῠ̔~ᾰ~ῐ", "ῠ̔~ᾰ~ει", "ῠ̔~ᾰ~εις";
    ),
    // Ἀκροβυστια: 4 words (ἀσφυξια, ἀκροβυστια, ἀμβλυωπια, ἀσυμμετρια).
    declension!("ἀκροβυστια", 4, "ἀ~υ~ια", [gender: Feminine];
        nominative: "ᾰ̓~ῠ~ῐᾱ", "ᾰ̓~ῠ~ῐᾱ", "ᾰ̓~ῠ~ῐαι";
        genitive: "ᾰ̓~ῠ~ῐᾱς", "ᾰ̓~ῠ~ῐαιν", "ᾰ̓~ῠ~ῐων";
        dative: "ᾰ̓~ῠ~ῐᾳ", "ᾰ̓~ῠ~ῐαιν", "ᾰ̓~ῠ~ῐαις";
        accusative: "ᾰ̓~ῠ~ῐᾱν", "ᾰ̓~ῠ~ῐᾱ", "ᾰ̓~ῠ~ῐᾱς";
        vocative: "ᾰ̓~ῠ~ῐᾱ", "ᾰ̓~ῠ~ῐᾱ", "ᾰ̓~ῠ~ῐαι";
    ),
    // Ἀποτελεσμα: 4 words (ἀγνοημα, ἀκουσμα, ἀσκημα, ἀποτελεσμα).
    declension!("ἀποτελεσμα", 4, "ἀ~α", [gender: Neuter];
        nominative: "ᾰ̓~ᾰ", "ᾰ̓~ᾰτε", "ᾰ̓~ᾰτᾰ";
        genitive: "ᾰ̓~ᾰτος", "ᾰ̓~ᾰτοιν", "ᾰ̓~ᾰτων";
        dative: "ᾰ̓~ᾰτῐ", "ᾰ̓~ᾰτοιν", "ᾰ̓~ᾰσῐ|ᾰ̓~ᾰσῐν";
        accusative: "ᾰ̓~ᾰ", "ᾰ̓~ᾰτε", "ᾰ̓~ᾰτᾰ";
        vocative: "ᾰ̓~ᾰ", "ᾰ̓~ᾰτε", "ᾰ̓~ᾰτᾰ";
    ),
    // Ἰσον: 4 words (ἰχθυδῐον, ἰσον, ἰτον, ἰπνον).
    declension!("ἰσον", 4, "ἰ~ον", [gender: Neuter];
        nominative: "ῐ̓~ον", "ῐ̓~ω", "ῐ̓~ᾰ";
        genitive: "ῐ̓~ου", "ῐ̓~οιν", "ῐ̓~ων";
        dative: "ῐ̓~ῳ", "ῐ̓~οιν", "ῐ̓~οις";
        accusative: "ῐ̓~ον", "ῐ̓~ω", "ῐ̓~ᾰ";
        vocative: "ῐ̓~ον", "ῐ̓~ω", "ῐ̓~ᾰ";
    ),
    // Ἀστερισκος: 4 words (ἀνθρωπισκος, ἀργιλλος, ἀστερισκος, ἀγχινοος). Like Ἀνισον, except in 5 cells: nominative sg, nominative pl, accusative pl, vocative sg, vocative pl.
    declension!("ἀστερισκος", 4, "ἀ~ι~ος", like Ἀνισον;
        nominative: "ᾰ̓~ῐ~ος", _, "ᾰ̓~ῐ~οι";
        genitive: _, _, _;
        dative: _, _, _;
        accusative: _, _, "ᾰ̓~ῐ~ους";
        vocative: "ᾰ̓~ῐ~ε", _, "ᾰ̓~ῐ~οι";
    ),
    // Κηρῠξ: 4 words (κοκκῠξ, κηρῠξ, ψευδοκηρῠξ, ἱεροκηρῠξ).
    declension!("κηρῠξ", 4, "~ῠξ", [gender: Masculine];
        nominative: "~ῠξ", "~ῡκε", "~ῡκες";
        genitive: "~ῡκος", "~ῡκοιν", "~ῡκων";
        dative: "~ῡκῐ", "~ῡκοιν", "~ῡξῐ|~ῡξῐν";
        accusative: "~ῡκᾰ", "~ῡκε", "~ῡκᾰς";
        vocative: "~ῠξ", "~ῡκε", "~ῡκες";
    ),
    // Ἀκοντιον: 4 words (ἀγγουριον, ἀπιον, ἀκοντιον, ἀλετριον).
    declension!("ἀκοντιον", 4, "ἀ~ιον", [gender: Neuter];
        nominative: "ᾰ̓~ῐον", "ᾰ̓~ῐω", "ᾰ̓~ῐᾰ";
        genitive: "ᾰ̓~ῐου", "ᾰ̓~ῐοιν", "ᾰ̓~ῐων";
        dative: "ᾰ̓~ῐῳ", "ᾰ̓~ῐοιν", "ᾰ̓~ῐοις";
        accusative: "ᾰ̓~ῐον", "ᾰ̓~ῐω", "ᾰ̓~ῐᾰ";
        vocative: "ᾰ̓~ῐον", "ᾰ̓~ῐω", "ᾰ̓~ῐᾰ";
    ),
    // Γαλη: 3 words (γη, μῡγᾰλη, γαλη). Like Κᾰτᾰσκευη, except in 2 cells: nominative pl, vocative pl.
    declension!("γαλη", 3, "~η", [gender: Feminine], like Κᾰτᾰσκευη;
        nominative: _, _, "~αῑ";
        genitive: _, _, _;
        dative: _, _, _;
        accusative: _, _, _;
        vocative: _, _, "~αῑ";
    ),
    // Ἀνακυκλωσις: 3 words (ἀποκαλυψις, ἀνακυκλωσις, ἀνακυκλησις).
    declension!("ἀνακυκλωσις", 3, "ἀ~α~υ~ις", [gender: Feminine];
        nominative: "ᾰ̓~ᾰ~ῠ~ῐς", "ᾰ̓~ᾰ~ῠ~ει", "ᾰ̓~ᾰ~ῠ~εις";
        genitive: "ᾰ̓~ᾰ~ῠ~εως", "ᾰ̓~ᾰ~ῠ~εοιν", "ᾰ̓~ᾰ~ῠ~εων";
        dative: "ᾰ̓~ᾰ~ῠ~ει", "ᾰ̓~ᾰ~ῠ~εοιν", "ᾰ̓~ᾰ~ῠ~εσῐ|ᾰ̓~ᾰ~ῠ~εσῐν";
        accusative: "ᾰ̓~ᾰ~ῠ~ῐν", "ᾰ̓~ᾰ~ῠ~ει", "ᾰ̓~ᾰ~ῠ~εις";
        vocative: "ᾰ̓~ᾰ~ῠ~ῐ", "ᾰ̓~ᾰ~ῠ~ει", "ᾰ̓~ᾰ~ῠ~εις";
    ),
    // Πῠρᾰ: 3 words (πῠρᾰ, πολῑτῐκᾰ, ἐσθλᾰ). Like Ὀργῐᾰ, except in 1 cell: dative pl.
    declension!("πῠρᾰ", 3, "~ᾰ", [gender: Neuter], like Ὀργῐᾰ;
        nominative: _, _, _;
        genitive: _, _, _;
        dative: _, _, "~οῑς";
        accusative: _, _, _;
        vocative: _, _, _;
    ),
    // Ἱκετης: 3 words (ἱπποτοξοτης, ἱπποτης, ἱκετης).
    declension!("ἱκετης", 3, "ἱ~ης", [gender: Masculine];
        nominative: "ῐ̔~ης", "ῐ̔~ᾱ", "ῐ̔~αι";
        genitive: "ῐ̔~ου", "ῐ̔~αιν", "ῐ̔~ων";
        dative: "ῐ̔~ῃ", "ῐ̔~αιν", "ῐ̔~αις";
        accusative: "ῐ̔~ην", "ῐ̔~ᾱ", "ῐ̔~ᾱς";
        vocative: "ῐ̔~ᾰ", "ῐ̔~ᾱ", "ῐ̔~αι";
    ),
    // Ἰδιον: 3 words (ἰδιον, ἰσχιον, ἰπνιον).
    declension!("ἰδιον", 3, "ἰ~ιον", [gender: Neuter];
        nominative: "ῐ̓~ῐον", "ῐ̓~ῐω", "ῐ̓~ῐᾰ";
        genitive: "ῐ̓~ῐου", "ῐ̓~ῐοιν", "ῐ̓~ῐων";
        dative: "ῐ̓~ῐῳ", "ῐ̓~ῐοιν", "ῐ̓~ῐοις";
        accusative: "ῐ̓~ῐον", "ῐ̓~ῐω", "ῐ̓~ῐᾰ";
        vocative: "ῐ̓~ῐον", "ῐ̓~ῐω", "ῐ̓~ῐᾰ";
    ),
    // Ὑποστρατηγος: 3 words (ὑμνογραφος, ὑοφθαλμος, ὑποστρατηγος).
    declension!("ὑποστρατηγος", 3, "ὑ~α~ος", [gender: Masculine];
        nominative: "ῠ̔~ᾰ~ος", "ῠ̔~ᾰ~ω", "ῠ̔~ᾰ~οι";
        genitive: "ῠ̔~ᾰ~ου", "ῠ̔~ᾰ~οιν", "ῠ̔~ᾰ~ων";
        dative: "ῠ̔~ᾰ~ῳ", "ῠ̔~ᾰ~οιν", "ῠ̔~ᾰ~οις";
        accusative: "ῠ̔~ᾰ~ον", "ῠ̔~ᾰ~ω", "ῠ̔~ᾰ~ους";
        vocative: "ῠ̔~ᾰ~ε", "ῠ̔~ᾰ~ω", "ῠ̔~ᾰ~οι";
    ),
    // Δηξ: 3 words (λῠγξ, δηξ, βλᾱξ). Like Χᾰρᾰξ, except in 2 cells: genitive du, dative du.
    declension!("δηξ", 3, "~ξ", [gender: Masculine], like Χᾰρᾰξ;
        nominative: _, _, _;
        genitive: _, "~κοῑν", _;
        dative: _, "~κοῑν", _;
        accusative: _, _, _;
        vocative: _, _, _;
    ),
    // Νεῑκος: 3 words (τεῑχος, χεῑλος, νεῑκος).
    declension!("νεῑκος", 3, "~ῑ~ος", [gender: Neuter];
        nominative: "~ῑ~ος", "~ι~ει", "~ι~η";
        genitive: "~ι~ους", "~ι~οῑν", "~ι~ων";
        dative: "~ι~ει", "~ι~οῑν", "~ι~εσῐ|~ι~εσῐν";
        accusative: "~ῑ~ος", "~ι~ει", "~ι~η";
        vocative: "~ῑ~ος", "~ι~ει", "~ι~η";
    ),
    // Ἀσις: 3 words (ἀκρις, ἀκοιτις, ἀσις).
    declension!("ἀσις", 3, "ἀ~ις", [gender: Feminine];
        nominative: "ᾰ̓~ῐς", "ᾰ̓~ῐε", "ᾰ̓~ῐες";
        genitive: "ᾰ̓~ῐος", "ᾰ̓~ῐοιν", "ᾰ̓~ῐων";
        dative: "ᾰ̓~ῑ", "ᾰ̓~ῐοιν", "ᾰ̓~ῐσῐ|ᾰ̓~ῐσῐν";
        accusative: "ᾰ̓~ῐν", "ᾰ̓~ῐε", "ᾰ̓~ῑς|ᾰ̓~ῐᾰς";
        vocative: "ᾰ̓~ῐ", "ᾰ̓~ῐε", "ᾰ̓~ῐες";
    ),
    // Πειθω: 3 words (λεχω, πειθω, κερδω). Like Ἠχω, except in 1 cell: genitive sg.
    declension!("πειθω", 3, "~ω", [gender: Feminine], like Ἠχω;
        nominative: _, _, _;
        genitive: "~οος|~οῡς", _, _;
        dative: _, _, _;
        accusative: _, _, _;
        vocative: _, _, _;
    ),
    // Πεῑνᾰ: 3 words (πεῑσᾰ, χλαῑνᾰ, πεῑνᾰ). Like Μοῑρᾰ, except in 2 cells: genitive sg, dative sg.
    declension!("πεῑνᾰ", 3, "~ῑ~ᾰ", [gender: Feminine], like Μοῑρᾰ;
        nominative: _, _, _;
        genitive: "~ι~ης", _, _;
        dative: "~ι~ῃ", _, _;
        accusative: _, _, _;
        vocative: _, _, _;
    ),
    // Ἀβατον: 3 words (ἀνδραποδον, ἀσαρον, ἀβατον). Like Ἀστραγᾰλος, except in 5 cells: nominative sg, nominative pl, accusative pl, vocative sg, vocative pl.
    declension!("ἀβατον", 3, "ἀ~α~ον", [gender: Neuter], like Ἀστραγᾰλος;
        nominative: "ᾰ̓~ᾰ~ον", _, "ᾰ̓~ᾰ~ᾰ";
        genitive: _, _, _;
        dative: _, _, _;
        accusative: _, _, "ᾰ̓~ᾰ~ᾰ";
        vocative: "ᾰ̓~ᾰ~ον", _, "ᾰ̓~ᾰ~ᾰ";
    ),
    // Ἱπποκαμπος: 3 words (ἱπποποταμος, ἱπποκαμπος, ἱμαντοδεσμος).
    declension!("ἱπποκαμπος", 3, "ἱ~α~ος", [gender: Masculine];
        nominative: "ῐ̔~ᾰ~ος", "ῐ̔~ᾰ~ω", "ῐ̔~ᾰ~οι";
        genitive: "ῐ̔~ᾰ~ου", "ῐ̔~ᾰ~οιν", "ῐ̔~ᾰ~ων";
        dative: "ῐ̔~ᾰ~ῳ", "ῐ̔~ᾰ~οιν", "ῐ̔~ᾰ~οις";
        accusative: "ῐ̔~ᾰ~ον", "ῐ̔~ᾰ~ω", "ῐ̔~ᾰ~ους";
        vocative: "ῐ̔~ᾰ~ε", "ῐ̔~ᾰ~ω", "ῐ̔~ᾰ~οι";
    ),
    // Ἀντιθεσις: 3 words (ἀντιθεσις, ἀντιπτωσις, ἀφιξις).
    declension!("ἀντιθεσις", 3, "ἀ~ι~ις", [gender: Feminine];
        nominative: "ᾰ̓~ῐ~ῐς", "ᾰ̓~ῐ~ει", "ᾰ̓~ῐ~εις";
        genitive: "ᾰ̓~ῐ~εως", "ᾰ̓~ῐ~εοιν", "ᾰ̓~ῐ~εων";
        dative: "ᾰ̓~ῐ~ει", "ᾰ̓~ῐ~εοιν", "ᾰ̓~ῐ~εσῐ|ᾰ̓~ῐ~εσῐν";
        accusative: "ᾰ̓~ῐ~ῐν", "ᾰ̓~ῐ~ει", "ᾰ̓~ῐ~εις";
        vocative: "ᾰ̓~ῐ~ῐ", "ᾰ̓~ῐ~ει", "ᾰ̓~ῐ~εις";
    ),
    // Ἀριθμος: 3 words (ἀριθμος, ἀφορισμος, ἀκιδωτος). Like Ἀστερισκος, except in 4 cells: genitive sg, genitive du, dative du, dative pl.
    declension!("ἀριθμος", 3, "ἀ~ι~ος", [gender: Masculine], like Ἀστερισκος;
        nominative: _, _, _;
        genitive: "ᾰ̓~ῐ~οῡ", "ᾰ̓~ῐ~οῑν", _;
        dative: _, "ᾰ̓~ῐ~οῑν", "ᾰ̓~ῐ~οῑς";
        accusative: _, _, _;
        vocative: _, _, _;
    ),
    // Ἀγκυλη: 3 words (ἀφροσυνη, ἀργυροθηκη, ἀγκυλη).
    declension!("ἀγκυλη", 3, "ἀ~υ~η", [gender: Feminine];
        nominative: "ᾰ̓~ῠ~η", "ᾰ̓~ῠ~ᾱ", "ᾰ̓~ῠ~αι";
        genitive: "ᾰ̓~ῠ~ης", "ᾰ̓~ῠ~αιν", "ᾰ̓~ῠ~ων";
        dative: "ᾰ̓~ῠ~ῃ", "ᾰ̓~ῠ~αιν", "ᾰ̓~ῠ~αις";
        accusative: "ᾰ̓~ῠ~ην", "ᾰ̓~ῠ~ᾱ", "ᾰ̓~ῠ~ᾱς";
        vocative: "ᾰ̓~ῠ~η", "ᾰ̓~ῠ~ᾱ", "ᾰ̓~ῠ~αι";
    ),
    // Ἁμμα: 3 words (ἁρμα, ἁλμα, ἁμμα).
    declension!("ἁμμα", 3, "ἁ~α", [gender: Neuter];
        nominative: "ᾰ̔~ᾰ", "ᾰ̔~ᾰτε", "ᾰ̔~ᾰτᾰ";
        genitive: "ᾰ̔~ᾰτος", "ᾰ̔~ᾰτοιν", "ᾰ̔~ᾰτων";
        dative: "ᾰ̔~ᾰτῐ", "ᾰ̔~ᾰτοιν", "ᾰ̔~ᾰσῐ|ᾰ̔~ᾰσῐν";
        accusative: "ᾰ̔~ᾰ", "ᾰ̔~ᾰτε", "ᾰ̔~ᾰτᾰ";
        vocative: "ᾰ̔~ᾰ", "ᾰ̔~ᾰτε", "ᾰ̔~ᾰτᾰ";
    ),
    // Δᾰσῠ: 3 words (ὀξῠ, δᾰσῠ, πλᾰτῠ). Like Φᾰος, except in 7 cells: nominative sg, nominative du, dative sg, accusative sg, accusative du, vocative sg, ….
    declension!("δᾰσῠ", 3, "~ῠ", like Φᾰος;
        nominative: "~ῠ", "~εῑ", _;
        genitive: _, _, _;
        dative: "~εῑ", _, _;
        accusative: "~ῠ", "~εῑ", _;
        vocative: "~ῠ", "~εῑ", _;
    ),
    // Λαλλαι: 3 words (κᾰλανδαι, χαρωνῑται, λαλλαι).
    declension!("λαλλαι", 3, "~α~αι";
        nominative: "~α~αι", derived, "~ᾰ~αι";
        genitive: derived, derived, "~ᾰ~ων";
        dative: derived, derived, "~ᾰ~αις";
        accusative: derived, derived, "~ᾰ~ᾱς";
        vocative: derived, derived, "~ᾰ~αι";
    ),
    // Ἐντεᾰ: 3 words (ἐντεᾰ, δηνεᾰ, τειρεᾰ). Like Ὀργῐᾰ, except in 1 cell: dative pl.
    declension!("ἐντεᾰ", 3, "~ᾰ", [gender: Neuter], like Ὀργῐᾰ;
        nominative: _, _, _;
        genitive: _, _, _;
        dative: _, _, "~σῐ|~σῐν";
        accusative: _, _, _;
        vocative: _, _, _;
    ),
    // Παστα: 3 words (χεδροπα, παστα, ἀλμενιχιακα). Like Ἰκρῐα, except in 1 cell: dative pl.
    declension!("παστα", 3, "~α", [gender: Neuter], like Ἰκρῐα;
        nominative: _, _, _;
        genitive: _, _, _;
        dative: _, _, "~οῑς";
        accusative: _, _, _;
        vocative: _, _, _;
    ),
    // Γᾰμφηλαι: 3 words (γᾰμφηλαι, ταυρεασται, δοκηται). Like Ὀργῐᾰ, except in 5 cells: nominative sg, nominative pl, dative pl, accusative pl, vocative pl.
    declension!("γᾰμφηλαι", 3, "~αι", like Ὀργῐᾰ;
        nominative: "~αι", _, "~αι";
        genitive: _, _, _;
        dative: _, _, "~αῑς";
        accusative: _, _, "~ᾱς";
        vocative: _, _, "~αι";
    ),
    // Θεωσῐς: 3 words (θεωσῐς, ἀλθησῐς, ζαχαρῐς).
    declension!("θεωσῐς", 3, "~ῐς", [gender: Feminine];
        nominative: "~ῐς", derived, "~εες";
        genitive: "~εως", derived, "~εων";
        dative: "~εῐ̈", derived, "~εσῐ";
        accusative: "~ῐν", derived, "~εᾰς";
        vocative: "~ῐ", derived, "~εες";
    ),
    // Ὑποψιᾱ: 3 words (ὑποψιᾱ, ὑπεροψιᾱ, ὑδροφοβιᾱ).
    declension!("ὑποψιᾱ", 3, "ὑ~ιᾱ", [gender: Feminine];
        nominative: "ῠ̔~ῐᾱ", "ῠ̔~ῐᾱ", "ῠ̔~ῐαι";
        genitive: "ῠ̔~ῐᾱς", "ῠ̔~ῐαιν", "ῠ̔~ῐων";
        dative: "ῠ̔~ῐᾳ", "ῠ̔~ῐαιν", "ῠ̔~ῐαις";
        accusative: "ῠ̔~ῐᾱν", "ῠ̔~ῐᾱ", "ῠ̔~ῐᾱς";
        vocative: "ῠ̔~ῐᾱ", "ῠ̔~ῐᾱ", "ῠ̔~ῐαι";
    ),
    // Ζωᾱ: 3 words (γονᾱ, σφᾰγᾱ, ζωᾱ). Like Φορᾱ, except in 4 cells: genitive pl, dative pl, accusative sg, accusative pl.
    declension!("ζωᾱ", 3, "~ᾱ", [gender: Feminine], like Φορᾱ;
        nominative: _, _, _;
        genitive: _, _, "~ᾱων|~ᾱν";
        dative: _, _, "~αῑς|~αῑσῐ|~αῑσῐν";
        accusative: derived, _, "~ᾱ̆ς";
        vocative: _, _, _;
    ),
    // Ἀσφαλισις: 3 words (ἀνακουφισις, ἀνταποκρισις, ἀσφαλισις).
    declension!("ἀσφαλισις", 3, "ἀ~α~ι~ις", [gender: Feminine];
        nominative: "ᾰ̓~ᾰ~ῐ~ῐς", "ᾰ̓~ᾰ~ῐ~ει", "ᾰ̓~ᾰ~ῐ~εις";
        genitive: "ᾰ̓~ᾰ~ῐ~εως", "ᾰ̓~ᾰ~ῐ~εοιν", "ᾰ̓~ᾰ~ῐ~εων";
        dative: "ᾰ̓~ᾰ~ῐ~ει", "ᾰ̓~ᾰ~ῐ~εοιν", "ᾰ̓~ᾰ~ῐ~εσῐ|ᾰ̓~ᾰ~ῐ~εσῐν";
        accusative: "ᾰ̓~ᾰ~ῐ~ῐν", "ᾰ̓~ᾰ~ῐ~ει", "ᾰ̓~ᾰ~ῐ~εις";
        vocative: "ᾰ̓~ᾰ~ῐ~ῐ", "ᾰ̓~ᾰ~ῐ~ει", "ᾰ̓~ᾰ~ῐ~εις";
    ),
    // Βᾰρῠτης: 2 words (χᾰρῐς, δαις). Like Περᾰς, except in 4 cells: nominative pl, accusative sg, accusative pl, vocative pl.
    declension!("βᾰρῠτης", 2, "~ς", like Περᾰς;
        nominative: _, _, "~τες";
        genitive: _, _, _;
        dative: _, _, _;
        accusative: "~τᾰ", _, "~τᾰς";
        vocative: _, _, "~τες";
    ),
];
