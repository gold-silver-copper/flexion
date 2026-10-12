//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's feature says):
/// `nominative` (sg, du, pl)
/// `genitive` (sg, du, pl)
/// `dative` (sg, du, pl)
/// `accusative` (sg, du, pl)
/// `vocative` (sg, du, pl)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal;
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
            forms: ::std::borrow::Cow::Borrowed(&[
                flexion::form!($c0),
                flexion::form!($c1),
                flexion::form!($c2),
                flexion::form!($c3),
                flexion::form!($c4),
                flexion::form!($c5),
                flexion::form!($c6),
                flexion::form!($c7),
                flexion::form!($c8),
                flexion::form!($c9),
                flexion::form!($c10),
                flexion::form!($c11),
                flexion::form!($c12),
                flexion::form!($c13),
                flexion::form!($c14),
            ]),
        }
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
    /// Like `κορᾰξ`: 147 words.
    Κορᾰξ,
    /// Like `τρᾰπεζᾰ`: 132 words.
    Τρᾰπεζᾰ,
    /// Like `ἐπῐφᾰνειᾰ`: 114 words.
    Ἐπῐφᾰνειᾰ,
    /// Like `γενος`: 97 words.
    Γενος,
    /// Like `αἰων`: 92 words.
    Αἰων,
    /// Like `κλεις`: 90 words.
    Κλεις,
    /// Like `χορευτης`: 84 words.
    Χορευτης,
    /// Like `γρᾰφευς`: 74 words.
    Γρᾰφευς,
    /// Like `κεραιᾱ`: 69 words.
    Κεραιᾱ,
    /// Like `βᾰρῠτης`: 68 words.
    Βᾰρῠτης,
    /// Like `χᾰρᾰκτηρ`: 68 words.
    Χᾰρᾰκτηρ,
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
    /// Like `πτερῠξ`: 37 words.
    Πτερῠξ,
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
    /// Like `σᾰρξ`: 12 words.
    Σᾰρξ,
    /// Like `ῥᾱξ`: 12 words.
    Ῥᾱξ,
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
    /// Like `μᾰρῐκᾱς`: 7 words.
    Μᾰρῐκᾱς,
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
        Declension::Τρᾰπεζᾰ,
        Declension::Ἐπῐφᾰνειᾰ,
        Declension::Γενος,
        Declension::Αἰων,
        Declension::Κλεις,
        Declension::Χορευτης,
        Declension::Γρᾰφευς,
        Declension::Κεραιᾱ,
        Declension::Βᾰρῠτης,
        Declension::Χᾰρᾰκτηρ,
        Declension::Σημεῑον,
        Declension::Κᾰλπῐς,
        Declension::Φορᾱ,
        Declension::Δᾰμᾰλης,
        Declension::Ὀργῐᾰ,
        Declension::Κοχλῐᾱς,
        Declension::Ζῠγον,
        Declension::Ὀλῐνοι,
        Declension::Κροσσαι,
        Declension::Πτερῠξ,
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
        Declension::Μῡς,
        Declension::Πους,
        Declension::Λᾰτρῐς,
        Declension::Πνεῡμᾰ,
        Declension::Περᾰς,
        Declension::Ἀστος,
        Declension::Ἀστηρ,
        Declension::Ἀσαῑοι,
        Declension::Σᾰρξ,
        Declension::Ῥᾱξ,
        Declension::Λᾰγως,
        Declension::Σηψ,
        Declension::Κηρ,
        Declension::Ἀμαυρωσις,
        Declension::Στοῑχος,
        Declension::Ἀποστασις,
        Declension::Ἰκρῐα,
        Declension::Ἀγος,
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
        Declension::Φρην,
        Declension::Ἀντονομασια,
        Declension::Μωλῠ,
        Declension::Ὑδρα,
        Declension::Χεῑμᾰ,
        Declension::Ἀναγωγη,
        Declension::Ἀδᾰμᾱς,
        Declension::Ἀναθεμα,
        Declension::Μᾰρῐκᾱς,
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
    // Μορῐον: 1,012 words (παιδῐον, δενδρον, μῠστηρῐον, ἐργον, εἰδωλον, σῠνεδρῐον).
    declension!("μορῐον", 1012, "~ον";
        nominative: "~ον", "~ω", "~ᾰ";
        genitive: "~ου", "~οιν", "~ων";
        dative: "~ῳ", "~οιν", "~οις";
        accusative: "~ον", "~ω", "~ᾰ";
        vocative: "~ον", "~ω", "~ᾰ";
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
    declension!("οὐσῐᾱ", 542, "~ᾱ";
        nominative: "~ᾱ", "~ᾱ", "~αι";
        genitive: "~ᾱς", "~αιν", "~ων";
        dative: "~ᾳ", "~αιν", "~αις";
        accusative: "~ᾱν", "~ᾱ", "~ᾱς";
        vocative: "~ᾱ", "~ᾱ", "~αι";
    ),
    // Νῠμφη: 519 words (νῑκη, δῑνη, κορη, γῠνη, ἐλεημοσῠνη, εἰρηνη).
    declension!("νῠμφη", 519, "~η";
        nominative: "~η", "~ᾱ", "~αι";
        genitive: "~ης", "~αιν", "~ων";
        dative: "~ῃ", "~αιν", "~αις";
        accusative: "~ην", "~ᾱ", "~ᾱς";
        vocative: "~η", "~ᾱ", "~αι";
    ),
    // Μερῐσμος: 463 words (λαιμος, μᾰστος, θεος, καιρος, νεκρος, ὀφθᾰλμος).
    declension!("μερῐσμος", 463, "~ος";
        nominative: "~ος", "~ω", "~οι";
        genitive: "~οῡ", "~οῑν", "~ων";
        dative: "~ῳ", "~οῑν", "~οῑς";
        accusative: "~ον", "~ω", "~ους";
        vocative: "~ε", "~ω", "~οι";
    ),
    // Δῐαιρεσῐς: 429 words (πολῐς, γνωσῐς, κῠβερνησῐς, μορφωσῐς, φῠσῐς, κρῐσῐς).
    declension!("δῐαιρεσῐς", 429, "~ῐς";
        nominative: "~ῐς", "~ει", "~εις";
        genitive: "~εως", "~εοιν", "~εων";
        dative: "~ει", "~εοιν", "~εσῐ|~εσῐν";
        accusative: "~ῐν", "~ει", "~εις";
        vocative: "~ῐ", "~ει", "~εις";
    ),
    // Κᾰτᾰσκευη: 274 words (μηχᾰνη, μουσῐκη, γρᾰμμᾰτῐκη, φωνη, κεφᾰλη, ἡδονη).
    declension!("κᾰτᾰσκευη", 274, "~η";
        nominative: "~η", "~ᾱ", "~αι";
        genitive: "~ης", "~αῑν", "~ων";
        dative: "~ῃ", "~αῑν", "~αῑς";
        accusative: "~ην", "~ᾱ", "~ᾱς";
        vocative: "~η", "~ᾱ", "~αι";
    ),
    // Γενετης: 196 words (προφητης, κλεπτης, πολῑτης, τρωγλοδῠτης, σῠβᾰρῑτης, χᾰρτης).
    declension!("γενετης", 196, "~ης";
        nominative: "~ης", "~ᾱ", "~αι";
        genitive: "~ου", "~αιν", "~ων";
        dative: "~ῃ", "~αιν", "~αις";
        accusative: "~ην", "~ᾱ", "~ᾱς";
        vocative: "~ᾰ", "~ᾱ", "~αι";
    ),
    // Κερκῐς: 186 words (παῑς, λεπῐς, πᾰτρῐς, ἑλληνῐς, περσῐς, ἑλμῐνς).
    declension!("κερκῐς", 186, "~ς";
        nominative: "~ς", "~δε", "~δες";
        genitive: "~δος", "~δοιν", "~δων";
        dative: "~δῐ", "~δοιν", "~σῐ|~σῐν";
        accusative: "~δᾰ", "~δε", "~δᾰς";
        vocative: "~", "~δε", "~δες";
    ),
    // Κορᾰξ: 147 words (ὀνῠξ, γλαῡξ, ἑλῐξ, κῐλῐξ, νῠξ, φαιᾱξ).
    declension!("κορᾰξ", 147, "~ξ";
        nominative: "~ξ", "~κε", "~κες";
        genitive: "~κος", "~κοιν", "~κων";
        dative: "~κῐ", "~κοιν", "~ξῐ|~ξῐν";
        accusative: "~κᾰ", "~κε", "~κᾰς";
        vocative: "~ξ", "~κε", "~κες";
    ),
    // Τρᾰπεζᾰ: 132 words (θᾰλᾰσσᾰ, ῥῐζᾰ, θᾰλᾰττᾰ, δοξᾰ, γλωττᾰ, μελῐττᾰ).
    declension!("τρᾰπεζᾰ", 132, "~ᾰ";
        nominative: "~ᾰ", "~ᾱ", "~αι";
        genitive: "~ης", "~αιν", "~ων";
        dative: "~ῃ", "~αιν", "~αις";
        accusative: "~ᾰν", "~ᾱ", "~ᾱς";
        vocative: "~ᾰ", "~ᾱ", "~αι";
    ),
    // Ἐπῐφᾰνειᾰ: 114 words (βᾰσῐλειᾰ, ἀσθενειᾰ, μᾰχαιρᾰ, βοηθειᾰ, συγγενειᾰ, ὀρθοεπειᾰ).
    declension!("ἐπῐφᾰνειᾰ", 114, "~ᾰ";
        nominative: "~ᾰ", "~ᾱ", "~αι";
        genitive: "~ᾱς", "~αιν", "~ων";
        dative: "~ᾳ", "~αιν", "~αις";
        accusative: "~ᾰν", "~ᾱ", "~ᾱς";
        vocative: "~ᾰ", "~ᾱ", "~αι";
    ),
    // Γενος: 97 words (σκῠλος, κλεος, ἐθνος, πληθος, τελος, στηθος).
    declension!("γενος", 97, "~ος";
        nominative: "~ος", "~ει", "~η";
        genitive: "~ους", "~οῑν", "~ων";
        dative: "~ει", "~οῑν", "~εσῐ|~εσῐν";
        accusative: "~ος", "~ει", "~η";
        vocative: "~ος", "~ει", "~η";
    ),
    // Αἰων: 92 words (ἑλλην, αἰων, λεγεων, σᾱπων, λᾰκων, ἀκαρνᾱν).
    declension!("αἰων", 92, "~ν";
        nominative: "~ν", "~νε", "~νες";
        genitive: "~νος", "~νοιν", "~νων";
        dative: "~νῐ", "~νοιν", "~σῐ|~σῐν";
        accusative: "~νᾰ", "~νε", "~νᾰς";
        vocative: "~ν", "~νε", "~νες";
    ),
    // Κλεις: 90 words (κλεις, χλᾰμῠς, μονᾰς, τρωᾰς, δειρᾰς, λοπᾰς).
    declension!("κλεις", 90, "~ς";
        nominative: "~ς", "~δε", "~δες";
        genitive: "~δος", "~δοιν", "~δων";
        dative: "~δῐ", "~δοιν", "~σῐ|~σῐν";
        accusative: "~δᾰ", "~δε", "~δᾰς";
        vocative: "~ς", "~δε", "~δες";
    ),
    // Χορευτης: 84 words (ἐρᾰστης, ποιητης, βᾰπτῐστης, κρῐτης, κωλοβᾰθρῐστης, σοφῐστης).
    declension!("χορευτης", 84, "~ης";
        nominative: "~ης", "~ᾱ", "~αι";
        genitive: "~οῡ", "~αῑν", "~ων";
        dative: "~ῃ", "~αῑν", "~αῑς";
        accusative: "~ην", "~ᾱ", "~ᾱς";
        vocative: "~ᾰ", "~ᾱ", "~αι";
    ),
    // Γρᾰφευς: 74 words (βᾰσῐλευς, ἀρχιερευς, γραμμᾰτευς, αἰολευς, ἀμφισσευς, ἀντῐκυρευς).
    declension!("γρᾰφευς", 74, "~ευς";
        nominative: "~ευς", "~η", "~ης|~εῑς";
        genitive: "~εως", "~εοιν", "~εων";
        dative: "~εῑ", "~εοιν", "~εῡσῐ|~εῡσῐν";
        accusative: "~εᾱ", "~η", "~εᾱς";
        vocative: "~εῡ", "~η", "~ης|~εῑς";
    ),
    // Κεραιᾱ: 69 words (προφητειᾱ, πορνειᾱ, παιδειᾱ, δουλειᾱ, ξῠλειᾱ, νηστειᾱ).
    declension!("κεραιᾱ", 69, "~~ᾱ";
        nominative: "~~ᾱ", "~~ᾱ", "~ῑα~";
        genitive: "~~ᾱς", "~~αιν", "~~ων";
        dative: "~~ᾳ", "~~αιν", "~~αις";
        accusative: "~~ᾱν", "~~ᾱ", "~~ᾱς";
        vocative: "~~ᾱ", "~~ᾱ", "~ῑα~";
    ),
    // Βᾰρῠτης: 68 words (μῠκης, κρης, σκληροτης, χᾰρῐς, θης, ἀθεοτης).
    declension!("βᾰρῠτης", 68, "~ς";
        nominative: "~ς", "~τε", "~τες";
        genitive: "~τος", "~τοιν", "~των";
        dative: "~τῐ", "~τοιν", "~σῐ|~σῐν";
        accusative: "~τᾰ", "~τε", "~τᾰς";
        vocative: "~ς", "~τε", "~τες";
    ),
    // Χᾰρᾰκτηρ: 68 words (μᾰρτῠρ, πᾰνθηρ, ἰβηρ, χειρ, ἐορ, κρᾱτηρ).
    declension!("χᾰρᾰκτηρ", 68, "~";
        nominative: "~", "~ε", "~ες";
        genitive: "~ος", "~οιν", "~ων";
        dative: "~ῐ", "~οιν", "~σῐ|~σῐν";
        accusative: "~ᾰ", "~ε", "~ᾰς";
        vocative: "~", "~ε", "~ες";
    ),
    // Σημεῑον: 60 words (θεῑον, πανδοχεῑον, πᾰνδοκεῑον, στοιχεῑον, βᾰλᾰνεῑον, σημεῑον).
    declension!("σημεῑον", 60, "~ῑον";
        nominative: "~ῑον", "~ιω", "~ῑᾰ";
        genitive: "~ιου", "~ιοιν", "~ιων";
        dative: "~ιῳ", "~ιοιν", "~ιοις";
        accusative: "~ῑον", "~ιω", "~ῑᾰ";
        vocative: "~ῑον", "~ιω", "~ῑᾰ";
    ),
    // Κᾰλπῐς: 60 words (πελασγῐωτῐς, σπᾰρτῐᾱτῐς, σῠβᾰρῑτῐς, σμῠρῐς, ἀσπρῐς, εὐνῐς).
    declension!("κᾰλπῐς", 60, "~ς";
        nominative: "~ς", "~δε", "~δες";
        genitive: "~δος", "~δοιν", "~δων";
        dative: "~δῐ", "~δοιν", "~σῐ|~σῐν";
        accusative: "~ν", "~δε", "~δᾰς";
        vocative: "~", "~δε", "~δες";
    ),
    // Φορᾱ: 55 words (παιδῐᾱ, πᾰτρῐᾱ, γενεᾱ, θεᾱ, στρᾰτῐᾱ, σκῐᾱ).
    declension!("φορᾱ", 55, "~ᾱ";
        nominative: "~ᾱ", "~ᾱ", "~αι";
        genitive: "~ᾱς", "~αῑν", "~ων";
        dative: "~ᾳ", "~αῑν", "~αῑς";
        accusative: "~ᾱν", "~ᾱ", "~ᾱς";
        vocative: "~ᾱ", "~ᾱ", "~αι";
    ),
    // Δᾰμᾰλης: 52 words (βελγης, δακης, μῐνῠης, ὀδρῠσης, πῐσῐδης, σᾰκης).
    declension!("δᾰμᾰλης", 52, "~ης";
        nominative: "~ης", "~ᾱ", "~αι";
        genitive: "~ου", "~αιν", "~ων";
        dative: "~ῃ", "~αιν", "~αις";
        accusative: "~ην", "~ᾱ", "~ᾱς";
        vocative: "~η", "~ᾱ", "~αι";
    ),
    // Ὀργῐᾰ: 51 words (ἑξηκοντᾰ, κᾰστᾰνειᾰ, κᾰστᾰνᾰ, ὀργῐᾰ, ἐγκᾰτᾰ, κεδμᾰτᾰ).
    declension!("ὀργῐᾰ", 51, "~ᾰ";
        nominative: "~ᾰ", "~ᾱ", "~ᾰ";
        genitive: "~ης", "~αιν", "~ων";
        dative: "~ῃ", "~αιν", "~οις";
        accusative: "~ᾰν", "~ᾱ", "~ᾰ";
        vocative: "~ᾰ", "~ᾱ", "~ᾰ";
    ),
    // Κοχλῐᾱς: 49 words (νεᾱνῐᾱς, γᾰλᾰξῐᾱς, καικιᾱς, κῠβᾱς, τᾰμῐᾱς, κοχλῐᾱς).
    declension!("κοχλῐᾱς", 49, "~ᾱς";
        nominative: "~ᾱς", "~ᾱ", "~αι";
        genitive: "~ου", "~αιν", "~ων";
        dative: "~ᾳ", "~αιν", "~αις";
        accusative: "~ᾱν", "~ᾱ", "~ᾱς";
        vocative: "~ᾱ", "~ᾱ", "~αι";
    ),
    // Ζῠγον: 45 words (πτερον, ὀνομᾰστῐκον, ᾠον, ἑρπετον, ζῠγον, φῠτον).
    declension!("ζῠγον", 45, "~ον";
        nominative: "~ον", "~ω", "~ᾰ";
        genitive: "~οῡ", "~οῑν", "~ων";
        dative: "~ῳ", "~οῑν", "~οῑς";
        accusative: "~ον", "~ω", "~ᾰ";
        vocative: "~ον", "~ω", "~ᾰ";
    ),
    // Ὀλῐνοι: 40 words (πᾰρᾰβολᾱνοι, μοσσῡνοικοι, ἡνῐοχοι, σῐνδοι, ὀλῐνοι, ταυλαντῐοι).
    declension!("ὀλῐνοι", 40, "~οι";
        nominative: "~ος", "~ω", "~οι";
        genitive: "~ου", "~οιν", "~ων";
        dative: "~ῳ", "~οιν", "~οις";
        accusative: "~ον", "~ω", "~ους";
        vocative: "~ε", "~ω", "~οι";
    ),
    // Κροσσαι: 39 words (ἑβδομηκοντᾰκωμηται, τεσσᾰρεσκαιδεκᾰτῑται, ὀᾰσῑται, ἰσκαι, κρουπεζαι, ὀζολαι).
    declension!("κροσσαι", 39, "~αι";
        nominative: "~ης", "~ᾱ", "~αι";
        genitive: "~ου", "~αιν", "~ων";
        dative: "~ῃ", "~αιν", "~αις";
        accusative: "~ην", "~ᾱ", "~ᾱς";
        vocative: "~ᾰ", "~ᾱ", "~αι";
    ),
    // Πτερῠξ: 37 words (πτερῠξ, λᾰτᾰξ, μηνῐγξ, φᾰλᾰγξ, σπηλῠγξ, λᾰρῠγξ).
    declension!("πτερῠξ", 37, "~ξ";
        nominative: "~ξ", "~γε", "~γες";
        genitive: "~γος", "~γοιν", "~γων";
        dative: "~γῐ", "~γοιν", "~ξῐ|~ξῐν";
        accusative: "~γᾰ", "~γε", "~γᾰς";
        vocative: "~ξ", "~γε", "~γες";
    ),
    // Φᾰος: 37 words (δᾰσος, πεος, σκᾰφος, φᾰος, πᾰχος, κᾰμπος).
    declension!("φᾰος", 37, "~ος";
        nominative: "~ος", "~εε", "~εᾰ";
        genitive: "~εος", "~εοιν", "~εων";
        dative: "~εῐ\u{308}", "~εοιν", "~εσῐ|~εσῐν";
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
    declension!("ναῡλος", 28, "~ῡ~ος";
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
        dative: "~ῠῐ\u{308}|~υι", "~ῠοιν", "~ῠσῐ|~ῠσῐν";
        accusative: "~ῠν", "~ῠε|~ῡ", "~ῡς|~ῠᾰς";
        vocative: "~ῠ", "~ῠε|~ῡ", "~ῠες|~ῡς";
    ),
    // Δᾰνᾰοι: 25 words (δᾰνᾰοι, τῐβᾰρηνοι, μεθοδῐκοι, πᾰλαιοι, ἐμπειρῐκοι, στοιχειωμᾰτῐκοι).
    declension!("δᾰνᾰοι", 25, "~οι";
        nominative: "~ος", "~ω", "~οι";
        genitive: "~οῡ", "~οῑν", "~ων";
        dative: "~ῳ", "~οῑν", "~οῑς";
        accusative: "~ον", "~ω", "~ους";
        vocative: "~ε", "~ω", "~οι";
    ),
    // Ῥωμαῑος: 24 words (ἰουδαῑος, φαρισαῑος, ῥωμαῑος, ἑβραῑος, χᾰλδαῑος, θεῑος).
    declension!("ῥωμαῑος", 24, "~ῑος";
        nominative: "~ῑος", "~ιω", "~ῑοι";
        genitive: "~ιου", "~ιοιν", "~ιων";
        dative: "~ιῳ", "~ιοιν", "~ιοις";
        accusative: "~ῑον", "~ιω", "~ιους";
        vocative: "~ῑε", "~ιω", "~ῑοι";
    ),
    // Ἀρετη: 23 words (ἀρχη, ἀδελφη, ἀκη, ἀοιδη, ἀρετη, ἀκμη).
    declension!("ἀρετη", 23, "ἀ~η";
        nominative: "ᾰ\u{313}~η", "ᾰ\u{313}~ᾱ", "ᾰ\u{313}~αι";
        genitive: "ᾰ\u{313}~ης", "ᾰ\u{313}~αῑν", "ᾰ\u{313}~ων";
        dative: "ᾰ\u{313}~ῃ", "ᾰ\u{313}~αῑν", "ᾰ\u{313}~αῑς";
        accusative: "ᾰ\u{313}~ην", "ᾰ\u{313}~ᾱ", "ᾰ\u{313}~ᾱς";
        vocative: "ᾰ\u{313}~η", "ᾰ\u{313}~ᾱ", "ᾰ\u{313}~αι";
    ),
    // Ἀκρον: 22 words (ἀστρον, ἀρθρον, ἀκρον, ἀγγεῑον, ἀλευρον, ἀροτρον).
    declension!("ἀκρον", 22, "ἀ~ον";
        nominative: "ᾰ\u{313}~ον", "ᾰ\u{313}~ω", "ᾰ\u{313}~ᾰ";
        genitive: "ᾰ\u{313}~ου", "ᾰ\u{313}~οιν", "ᾰ\u{313}~ων";
        dative: "ᾰ\u{313}~ῳ", "ᾰ\u{313}~οιν", "ᾰ\u{313}~οις";
        accusative: "ᾰ\u{313}~ον", "ᾰ\u{313}~ω", "ᾰ\u{313}~ᾰ";
        vocative: "ᾰ\u{313}~ον", "ᾰ\u{313}~ω", "ᾰ\u{313}~ᾰ";
    ),
    // Ἀντλος: 22 words (ἀνθρωπος, ἀποστολος, ἀρκτος, ἀνδρεῑος, ἀργεῑος, ἀνεμος).
    declension!("ἀντλος", 22, "ἀ~ος";
        nominative: "ᾰ\u{313}~ος", "ᾰ\u{313}~ω", "ᾰ\u{313}~οι";
        genitive: "ᾰ\u{313}~ου", "ᾰ\u{313}~οιν", "ᾰ\u{313}~ων";
        dative: "ᾰ\u{313}~ῳ", "ᾰ\u{313}~οιν", "ᾰ\u{313}~οις";
        accusative: "ᾰ\u{313}~ον", "ᾰ\u{313}~ω", "ᾰ\u{313}~ους";
        vocative: "ᾰ\u{313}~ε", "ᾰ\u{313}~ω", "ᾰ\u{313}~οι";
    ),
    // Ἀξων: 21 words (ἀρχιτεκτων, βιστων, χᾱων, λῠκᾱων, βρᾰχῑων, κᾰκοδαιμων).
    declension!("ἀξων", 21, "~ων";
        nominative: "~ων", "~ονε", "~ονες";
        genitive: "~ονος", "~ονοιν", "~ονων";
        dative: "~ονῐ", "~ονοιν", "~οσῐ|~οσῐν";
        accusative: "~ονᾰ", "~ονε", "~ονᾰς";
        vocative: "~ον", "~ονε", "~ονες";
    ),
    // Λεων: 20 words (θερᾰπων, λεων, δρᾰκων, γερων, ὁρῐζων, χᾰμαιλεων).
    declension!("λεων", 20, "~ων";
        nominative: "~ων", "~οντε", "~οντες";
        genitive: "~οντος", "~οντοιν", "~οντων";
        dative: "~οντῐ", "~οντοιν", "~ουσῐ|~ουσῐν";
        accusative: "~οντᾰ", "~οντε", "~οντᾰς";
        vocative: "~ον", "~οντε", "~οντες";
    ),
    // Ἀμβροσια: 19 words (ἀπορια, ἀρτηρια, ἀμνηστια, ἀμβροσια, ἀσωτια, ἀγεννησια).
    declension!("ἀμβροσια", 19, "ἀ~ια";
        nominative: "ᾰ\u{313}~ῐᾱ", "ᾰ\u{313}~ῐᾱ", "ᾰ\u{313}~ῐαι";
        genitive: "ᾰ\u{313}~ῐᾱς", "ᾰ\u{313}~ῐαιν", "ᾰ\u{313}~ῐων";
        dative: "ᾰ\u{313}~ῐᾳ", "ᾰ\u{313}~ῐαιν", "ᾰ\u{313}~ῐαις";
        accusative: "ᾰ\u{313}~ῐᾱν", "ᾰ\u{313}~ῐᾱ", "ᾰ\u{313}~ῐᾱς";
        vocative: "ᾰ\u{313}~ῐᾱ", "ᾰ\u{313}~ῐᾱ", "ᾰ\u{313}~ῐαι";
    ),
    // Μῡς: 16 words (σῡς, δελφῡς, δρῡς, ὀφρῡς, μῡς, γρᾱῡς).
    declension!("μῡς", 16, "~ῡς";
        nominative: "~ῡς", "~ῠε|~ῡ", "~ῠες|~ῡς";
        genitive: "~ῠος", "~ῠοιν", "~ῠων";
        dative: "~ῠῐ\u{308}|~υῑ", "~ῠοιν", "~ῠσῐ|~ῠσῐν";
        accusative: "~ῡν", "~ῠε|~ῡ", "~ῡς|~ῠᾰς";
        vocative: "~ῡ", "~ῠε|~ῡ", "~ῠες|~ῡς";
    ),
    // Πους: 16 words (πους, ὀκτωπους, πολυπους, τρῐπους, δᾰσῠπους, λᾰγωπους).
    declension!("πους", 16, "~υς";
        nominative: "~υς", "~δε", "~δες";
        genitive: "~δος", "~δοιν", "~δων";
        dative: "~δῐ", "~δοιν", "~σῐ|~σῐν";
        accusative: "~δᾰ", "~δε", "~δᾰς";
        vocative: "~υς", "~δε", "~δες";
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
    declension!("πνεῡμᾰ", 14, "~ῡ~";
        nominative: "~ῡ~", "~υ~τε", "~υ~τᾰ";
        genitive: "~υ~τος", "~υ~τοιν", "~υ~των";
        dative: "~υ~τῐ", "~υ~τοιν", "~υ~σῐ|~υ~σῐν";
        accusative: "~ῡ~", "~υ~τε", "~υ~τᾰ";
        vocative: "~ῡ~", "~υ~τε", "~υ~τᾰ";
    ),
    // Περᾰς: 13 words (φως, τερᾰς, κρεᾰς, πᾰγκρεᾰς, ἐρῠσῐπελᾰς, περᾰς).
    declension!("περᾰς", 13, "~ς";
        nominative: "~ς", "~τε", "~τᾰ";
        genitive: "~τος", "~τοιν", "~των";
        dative: "~τῐ", "~τοιν", "~σῐ|~σῐν";
        accusative: "~ς", "~τε", "~τᾰ";
        vocative: "~ς", "~τε", "~τᾰ";
    ),
    // Ἀστος: 13 words (ἀγρος, ἀδελφος, ἀοιδος, ἀλφος, ἀμνος, ἀρνος).
    declension!("ἀστος", 13, "ἀ~ος";
        nominative: "ᾰ\u{313}~ος", "ᾰ\u{313}~ω", "ᾰ\u{313}~οι";
        genitive: "ᾰ\u{313}~οῡ", "ᾰ\u{313}~οῑν", "ᾰ\u{313}~ων";
        dative: "ᾰ\u{313}~ῳ", "ᾰ\u{313}~οῑν", "ᾰ\u{313}~οῑς";
        accusative: "ᾰ\u{313}~ον", "ᾰ\u{313}~ω", "ᾰ\u{313}~ους";
        vocative: "ᾰ\u{313}~ε", "ᾰ\u{313}~ω", "ᾰ\u{313}~οι";
    ),
    // Ἀστηρ: 13 words (ἀνηρ, ἀστηρ, ἀλεκτωρ, ἀφθᾰ, ἀλωπηξ, ἀροτηρ).
    declension!("ἀστηρ", 13, "ἀ~";
        nominative: "ᾰ\u{313}~", "ᾰ\u{313}~ε", "ᾰ\u{313}~ες";
        genitive: "ᾰ\u{313}~ος", "ᾰ\u{313}~οιν", "ᾰ\u{313}~ων";
        dative: "ᾰ\u{313}~ῐ", "ᾰ\u{313}~οιν", "ᾰ\u{313}~σῐ";
        accusative: "ᾰ\u{313}~ᾰ", "ᾰ\u{313}~ε", "ᾰ\u{313}~ᾰς";
        vocative: "ᾰ\u{313}~", "ᾰ\u{313}~ε", "ᾰ\u{313}~ες";
    ),
    // Ἀσαῑοι: 13 words (βοττῐαῑοι, ἀραμαῑοι, ἀρδιαῑοι, ἀσαῑοι, θᾰμᾰναῑοι, νῐψαῑοι).
    declension!("ἀσαῑοι", 13, "~ῑοι";
        nominative: "~ῑος", "~ιω", "~ῑοι";
        genitive: "~ιου", "~ιοιν", "~ιων";
        dative: "~ιῳ", "~ιοιν", "~ιοις";
        accusative: "~ῑον", "~ιω", "~ιους";
        vocative: "~ῑε", "~ιω", "~ῑοι";
    ),
    // Σᾰρξ: 12 words (σᾰρξ, θρᾳξ, προιξ, σφηξ, πλᾰξ, φρῑξ).
    declension!("σᾰρξ", 12, "~ξ";
        nominative: "~ξ", "~κε", "~κες";
        genitive: "~κος", "~κοῑν", "~κων";
        dative: "~κῐ", "~κοῑν", "~ξῐ|~ξῐν";
        accusative: "~κᾰ", "~κε", "~κᾰς";
        vocative: "~ξ", "~κε", "~κες";
    ),
    // Ῥᾱξ: 12 words (φλοξ, αἰξ, φρῠξ, σφᾰξ, ῥᾱξ, στρᾰγξ).
    declension!("ῥᾱξ", 12, "~ξ";
        nominative: "~ξ", "~γε", "~γες";
        genitive: "~γος", "~γοῑν", "~γων";
        dative: "~γῐ", "~γοῑν", "~ξῐ|~ξῐν";
        accusative: "~γᾰ", "~γε", "~γᾰς";
        vocative: "~ξ", "~γε", "~γες";
    ),
    // Λᾰγως: 12 words (νεως, γᾰλως, τᾰως, λᾰγως, ἑως, λεως).
    declension!("λᾰγως", 12, "~ως";
        nominative: "~ως", "~ω", "~ῳ";
        genitive: "~ω", "~ῳν", "~ων";
        dative: "~ῳ", "~ῳν", "~ῳς";
        accusative: "~ω|~ων", "~ω", "~ως";
        vocative: "~ως", "~ω", "~ῳ";
    ),
    // Σηψ: 12 words (ὠψ, γρῡψ, θρῑψ, θωψ, κλωψ, σκωψ).
    declension!("σηψ", 12, "~ψ";
        nominative: "~ψ", "~πε", "~πες";
        genitive: "~πος", "~ποῑν", "~πων";
        dative: "~πῐ", "~ποῑν", "~ψῐ|~ψῐν";
        accusative: "~πᾰ", "~πε", "~πᾰς";
        vocative: "~ψ", "~πε", "~πες";
    ),
    // Κηρ: 12 words (κηρ, ἐᾰρ, θενᾰρ, ὀνᾰρ, νεκτᾰρ, μελᾰν).
    declension!("κηρ", 12, "~";
        nominative: "~", "~ε", "~ᾰ";
        genitive: "~ος", "~οιν", "~ων";
        dative: "~ῐ", "~οιν", "~σῐ|~σῐν";
        accusative: "~", "~ε", "~ᾰ";
        vocative: "~", "~ε", "~ᾰ";
    ),
    // Ἀμαυρωσις: 12 words (ἀρσις, ἀνεσις, ἀσκησις, ἀμαυρωσις, ἀκροπολις, ἀποδειξις).
    declension!("ἀμαυρωσις", 12, "ἀ~ις";
        nominative: "ᾰ\u{313}~ῐς", "ᾰ\u{313}~ει", "ᾰ\u{313}~εις";
        genitive: "ᾰ\u{313}~εως", "ᾰ\u{313}~εοιν", "ᾰ\u{313}~εων";
        dative: "ᾰ\u{313}~ει", "ᾰ\u{313}~εοιν", "ᾰ\u{313}~εσῐ|ᾰ\u{313}~εσῐν";
        accusative: "ᾰ\u{313}~ῐν", "ᾰ\u{313}~ει", "ᾰ\u{313}~εις";
        vocative: "ᾰ\u{313}~ῐ", "ᾰ\u{313}~ει", "ᾰ\u{313}~εις";
    ),
    // Στοῑχος: 12 words (χοῑρος, ἑταῑρος, στοῑχος, τοῑχος, σχοῑνος, ῥοῑζος).
    declension!("στοῑχος", 12, "~ῑ~ος";
        nominative: "~ῑ~ος", "~ι~ω", "~ῑ~οι";
        genitive: "~ι~ου", "~ι~οιν", "~ι~ων";
        dative: "~ι~ῳ", "~ι~οιν", "~ι~οις";
        accusative: "~ῑ~ον", "~ι~ω", "~ι~ους";
        vocative: "~ῑ~ε", "~ι~ω", "~ῑ~οι";
    ),
    // Ἀποστασις: 12 words (ἀναβρωσις, ἀποταξις, ἀναχωρησις, ἀποβασις, ἀκανθις, ἀποστασις).
    declension!("ἀποστασις", 12, "ἀ~α~ις";
        nominative: "ᾰ\u{313}~ᾰ~ῐς", "ᾰ\u{313}~ᾰ~ει", "ᾰ\u{313}~ᾰ~εις";
        genitive: "ᾰ\u{313}~ᾰ~εως", "ᾰ\u{313}~ᾰ~εοιν", "ᾰ\u{313}~ᾰ~εων";
        dative: "ᾰ\u{313}~ᾰ~ει", "ᾰ\u{313}~ᾰ~εοιν", "ᾰ\u{313}~ᾰ~εσῐ|ᾰ\u{313}~ᾰ~εσῐν";
        accusative: "ᾰ\u{313}~ᾰ~ῐν", "ᾰ\u{313}~ᾰ~ει", "ᾰ\u{313}~ᾰ~εις";
        vocative: "ᾰ\u{313}~ᾰ~ῐ", "ᾰ\u{313}~ᾰ~ει", "ᾰ\u{313}~ᾰ~εις";
    ),
    // Ἰκρῐα: 11 words (γενεσια, μουστακια, σαγματοπασμαγαδια, εὐληρα, βρῡτεα, κανθηλια).
    declension!("ἰκρῐα", 11, "~α";
        nominative: "~ον", "~ω", "~ᾰ";
        genitive: "~ου", "~οιν", "~ων";
        dative: "~ῳ", "~οιν", "~οις";
        accusative: "~ον", "~ω", "~ᾰ";
        vocative: "~ον", "~ω", "~ᾰ";
    ),
    // Ἀγος: 11 words (ἀνθος, ἀγος, ἀγκος, ἀλγος, ἀχος, ἀφενος).
    declension!("ἀγος", 11, "ἀ~ος";
        nominative: "ᾰ\u{313}~ος", "ᾰ\u{313}~ει", "ᾰ\u{313}~η";
        genitive: "ᾰ\u{313}~ους", "ᾰ\u{313}~οῑν", "ᾰ\u{313}~ων";
        dative: "ᾰ\u{313}~ει", "ᾰ\u{313}~οῑν", "ᾰ\u{313}~εσῐ|ᾰ\u{313}~εσῐν";
        accusative: "ᾰ\u{313}~ος", "ᾰ\u{313}~ει", "ᾰ\u{313}~η";
        vocative: "ᾰ\u{313}~ος", "ᾰ\u{313}~ει", "ᾰ\u{313}~η";
    ),
    // Χᾰρᾰξ: 10 words (χᾰλῐξ, περδῑξ, ψῑξ, φῠλᾰξ, σκῠλᾰξ, κᾰλῠξ).
    declension!("χᾰρᾰξ", 10, "~ξ";
        nominative: "~ξ", "~κε", "~κες";
        genitive: "~κος", "~κοιν", "~κων";
        dative: "~κῐ", "~κοιν", "~ξῐν";
        accusative: "~κᾰ", "~κε", "~κᾰς";
        vocative: "~ξ", "~κε", "~κες";
    ),
    // Ἀληθεια: 10 words (ἀληθεια, ἀγνοια, ἀμελεια, ἀσεβεια, ἀγχουσα, ἀρουρα).
    declension!("ἀληθεια", 10, "ἀ~α";
        nominative: "ᾰ\u{313}~ᾰ", "ᾰ\u{313}~ᾱ", "ᾰ\u{313}~αι";
        genitive: "ᾰ\u{313}~ᾱς", "ᾰ\u{313}~αιν", "ᾰ\u{313}~ων";
        dative: "ᾰ\u{313}~ᾳ", "ᾰ\u{313}~αιν", "ᾰ\u{313}~αις";
        accusative: "ᾰ\u{313}~ᾰν", "ᾰ\u{313}~ᾱ", "ᾰ\u{313}~ᾱς";
        vocative: "ᾰ\u{313}~ᾰ", "ᾰ\u{313}~ᾱ", "ᾰ\u{313}~αι";
    ),
    // Ὑψος: 10 words (ὑπνος, ὑψος, ὑδρος, ὑμνος, ὑσσωπος, ὑσκλος).
    declension!("ὑψος", 10, "ὑ~ος";
        nominative: "ῠ\u{314}~ος", "ῠ\u{314}~ω", "ῠ\u{314}~οι";
        genitive: "ῠ\u{314}~ου", "ῠ\u{314}~οιν", "ῠ\u{314}~ων";
        dative: "ῠ\u{314}~ῳ", "ῠ\u{314}~οιν", "ῠ\u{314}~οις";
        accusative: "ῠ\u{314}~ον", "ῠ\u{314}~ω", "ῠ\u{314}~ους";
        vocative: "ῠ\u{314}~ε", "ῠ\u{314}~ω", "ῠ\u{314}~οι";
    ),
    // Ἀγκων: 10 words (ἀγων, ἀκμων, ἀδην, ἀρχων, ἀμβων, ἀκων).
    declension!("ἀγκων", 10, "ἀ~ν";
        nominative: "ᾰ\u{313}~ν", "ᾰ\u{313}~νε", "ᾰ\u{313}~νες";
        genitive: "ᾰ\u{313}~νος", "ᾰ\u{313}~νοιν", "ᾰ\u{313}~νων";
        dative: "ᾰ\u{313}~νῐ", "ᾰ\u{313}~νοιν", "ᾰ\u{313}~σῐ";
        accusative: "ᾰ\u{313}~νᾰ", "ᾰ\u{313}~νε", "ᾰ\u{313}~νᾰς";
        vocative: "ᾰ\u{313}~ν", "ᾰ\u{313}~νε", "ᾰ\u{313}~νες";
    ),
    // Θηρ: 10 words (κᾱρ, χην, θηρ, ψᾱρ, φωρ, σηρ).
    declension!("θηρ", 10, "~";
        nominative: "~", "~ε", "~ες";
        genitive: "~ος", "~οῑν", "~ων";
        dative: "~ῐ", "~οῑν", "~σῐ|~σῐν";
        accusative: "~ᾰ", "~ε", "~ᾰς";
        vocative: "~", "~ε", "~ες";
    ),
    // Τρωες: 10 words (πῑερες, τρωες, κηφηνες, αἰγες, ἀνᾰξῠρῐδες, ὠρῠγγες).
    declension!("τρωες", 10, "~ες";
        nominative: "~ες", "~εῑ", "~ες";
        genitive: "~οῡς", "~οῑν", "~ων";
        dative: "~εῑ", "~οῑν", "~σῐ";
        accusative: "~ες", "~εῑ", "~ᾰς";
        vocative: "~ες", "~εῑ", "~ες";
    ),
    // Λευκη: 10 words (πευκη, δουλη, βλαυτη, σενδουκη, λευκη, ζευγλη).
    declension!("λευκη", 10, "~υ~η";
        nominative: "~υ~η", "~υ~ᾱ", "~ῡ~αι";
        genitive: "~υ~ης", "~υ~αιν", "~υ~ων";
        dative: "~υ~ῃ", "~υ~αιν", "~υ~αις";
        accusative: "~υ~ην", "~υ~ᾱ", "~υ~ᾱς";
        vocative: "~υ~η", "~υ~ᾱ", "~ῡ~αι";
    ),
    // Ἠχου: 10 words (ῥᾰββῑ, ῥᾰββονῑ, ῥᾰββουνῑ, κορβᾱν, βροτοῡ, βροτῳ).
    declension!("ἠχου", 10, "~";
        nominative: "~", "~τε", "~τᾰ";
        genitive: "~", "~τοιν", "~των";
        dative: "~", "~τοιν", "~σῐ";
        accusative: "~", "~τε", "~τᾰ";
        vocative: "~", "~τε", "~τᾰ";
    ),
    // Πεπερῐ: 9 words (σῐνᾱπῐ, κομμῐ, πεπερῐ, κιννᾰβᾰρῐ, στῐμμῐ, κῐκῐ).
    declension!("πεπερῐ", 9, "~ῐ";
        nominative: "~ῐ", "~ει", "~η|~εᾰ";
        genitive: "~εως", "~εοιν", "~εων";
        dative: "~ει", "~εοιν", "~εσῐ|~εσῐν";
        accusative: "~ῐ", "~ει", "~η|~εᾰ";
        vocative: "~ῐ", "~ει", "~η|~εᾰ";
    ),
    // Κοιτη: 9 words (χαιτη, κοιτη, ἐρεικη, θοινη, βαιτη, μαινη).
    declension!("κοιτη", 9, "~ι~η";
        nominative: "~ι~η", "~ι~ᾱ", "~ῑ~αι";
        genitive: "~ι~ης", "~ι~αιν", "~ι~ων";
        dative: "~ι~ῃ", "~ι~αιν", "~ι~αις";
        accusative: "~ι~ην", "~ι~ᾱ", "~ι~ᾱς";
        vocative: "~ι~η", "~ι~ᾱ", "~ῑ~αι";
    ),
    // Ἀλειτης: 9 words (ὠρειτης, παικτης, σωρειτης, σᾰμᾰρειτης, ἀρσενοκοιτης, μητροκοιτης).
    declension!("ἀλειτης", 9, "~ι~ης";
        nominative: "~ι~ης", "~ι~ᾱ", "~ῑ~αι";
        genitive: "~ι~ου", "~ι~αιν", "~ι~ων";
        dative: "~ι~ῃ", "~ι~αιν", "~ι~αις";
        accusative: "~ι~ην", "~ι~ᾱ", "~ι~ᾱς";
        vocative: "~ῑ~ᾰ", "~ι~ᾱ", "~ῑ~αι";
    ),
    // Ἀμη: 9 words (ἀλκη, ἀποθηκη, ἀλοη, ἀμη, ἀρτοθηκη, ἀρμενοθηκη).
    declension!("ἀμη", 9, "ἀ~η";
        nominative: "ᾰ\u{313}~η", "ᾰ\u{313}~ᾱ", "ᾰ\u{313}~αι";
        genitive: "ᾰ\u{313}~ης", "ᾰ\u{313}~αιν", "ᾰ\u{313}~ων";
        dative: "ᾰ\u{313}~ῃ", "ᾰ\u{313}~αιν", "ᾰ\u{313}~αις";
        accusative: "ᾰ\u{313}~ην", "ᾰ\u{313}~ᾱ", "ᾰ\u{313}~ᾱς";
        vocative: "ᾰ\u{313}~η", "ᾰ\u{313}~ᾱ", "ᾰ\u{313}~αι";
    ),
    // Πλᾰτεῑᾰ: 9 words (μυῑᾰ, πλᾰτεῑᾰ, γαῑᾰ, μαῑᾰ, βᾰρεῑᾰ, ὀξεῑᾰ).
    declension!("πλᾰτεῑᾰ", 9, "~ῑᾰ";
        nominative: "~ῑᾰ", "~ιᾱ", "~ῑαι";
        genitive: "~ιᾱς", "~ιαιν", "~ιων";
        dative: "~ιᾳ", "~ιαιν", "~ιαις";
        accusative: "~ῑᾰν", "~ιᾱ", "~ιᾱς";
        vocative: "~ῑᾰ", "~ιᾱ", "~ῑαι";
    ),
    // Σησᾰμοειδες: 9 words (νευροειδες, μονοειδες, τρῐχομᾰνες, ἡμεροκᾰλλες, σησᾰμοειδες, ὠκῐμοειδες).
    declension!("σησᾰμοειδες", 9, "~ες";
        nominative: "~ες", "~εῑ", "~η";
        genitive: "~οῡς", "~οῑν", "~ων";
        dative: "~εῑ", "~οῑν", "~εσῐ|~εσῐν";
        accusative: "~ες", "~εῑ", "~η";
        vocative: "~ες", "~εῑ", "~η";
    ),
    // Θῑς: 9 words (ῥῑς, ἰς, θῑς, ὠδῑς, γλωχῑς, στᾰμῑς).
    declension!("θῑς", 9, "~ς";
        nominative: "~ς", "~νε", "~νες";
        genitive: "~νος", "~νοιν", "~νων";
        dative: "~νῐ", "~νοιν", "~σῐ|~σῐν";
        accusative: "~νᾰ", "~νε", "~νᾰς";
        vocative: "~ς", "~νε", "~νες";
    ),
    // Ἀλωπεκιᾱ: 9 words (ἀβλεψιᾱ, ἀκηδιᾱ, ἀτροφιᾱ, ἀναιμιᾱ, ἀλωπεκιᾱ, ἀπουσιᾱ).
    declension!("ἀλωπεκιᾱ", 9, "ἀ~ιᾱ";
        nominative: "ᾰ\u{313}~ῐᾱ", "ᾰ\u{313}~ῐᾱ", "ᾰ\u{313}~ῐαι";
        genitive: "ᾰ\u{313}~ῐᾱς", "ᾰ\u{313}~ῐαιν", "ᾰ\u{313}~ῐων";
        dative: "ᾰ\u{313}~ῐᾳ", "ᾰ\u{313}~ῐαιν", "ᾰ\u{313}~ῐαις";
        accusative: "ᾰ\u{313}~ῐᾱν", "ᾰ\u{313}~ῐᾱ", "ᾰ\u{313}~ῐᾱς";
        vocative: "ᾰ\u{313}~ῐᾱ", "ᾰ\u{313}~ῐᾱ", "ᾰ\u{313}~ῐαι";
    ),
    // Πᾰτηρ: 8 words (μητηρ, πᾰτηρ, γᾰστηρ, θῠγᾰτηρ, φρᾱτηρ, ἐνᾰτηρ).
    declension!("πᾰτηρ", 8, "~η~";
        nominative: "~η~", "~ε~ε", "~ε~ες";
        genitive: "~~ος", "~ε~οιν", "~ε~ων";
        dative: "~~ῐ", "~ε~οιν", "~~ᾰσῐ";
        accusative: "~ε~ᾰ", "~ε~ε", "~ε~ᾰς";
        vocative: "~ε~", "~ε~ε", "~ε~ες";
    ),
    // Νεῡρον: 8 words (προῡμνον, νεῡρον, βοῡστον, τεῡτλον, σοῡσον, λοῡσσον).
    declension!("νεῡρον", 8, "~ῡ~ον";
        nominative: "~ῡ~ον", "~υ~ω", "~ῡ~ᾰ";
        genitive: "~υ~ου", "~υ~οιν", "~υ~ων";
        dative: "~υ~ῳ", "~υ~οιν", "~υ~οις";
        accusative: "~ῡ~ον", "~υ~ω", "~ῡ~ᾰ";
        vocative: "~ῡ~ον", "~υ~ω", "~ῡ~ᾰ";
    ),
    // Ἠχω: 8 words (ἠχω, τυτω, κῐττω, πῑπω, μοτω, ὁδω).
    declension!("ἠχω", 8, "~ω";
        nominative: "~ω", "~ω", "~οι";
        genitive: "~οῡς", "~οῑν", "~ων";
        dative: "~οῑ", "~οῑν", "~οῑς";
        accusative: "~ω", "~ω", "~ους";
        vocative: "~οῑ", "~ω", "~οι";
    ),
    // Ἀφεσῐς: 8 words (ἀρδῐς, ἀρῐς, ἀκουσῐς, ἀλγησῐς, ἀποκρῐσῐς, ἀποσῐωπησῐς).
    declension!("ἀφεσῐς", 8, "ἀ~ῐς";
        nominative: "ᾰ\u{313}~ῐς", "ᾰ\u{313}~ει", "ᾰ\u{313}~εις";
        genitive: "ᾰ\u{313}~εως", "ᾰ\u{313}~εοιν", "ᾰ\u{313}~εων";
        dative: "ᾰ\u{313}~ει", "ᾰ\u{313}~εοιν", "ᾰ\u{313}~εσῐ|ᾰ\u{313}~εσῐν";
        accusative: "ᾰ\u{313}~ῐν", "ᾰ\u{313}~ει", "ᾰ\u{313}~εις";
        vocative: "ᾰ\u{313}~ῐ", "ᾰ\u{313}~ει", "ᾰ\u{313}~εις";
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
    declension!("ἀντονομασια", 7, "ἀ~α~ια";
        nominative: "ᾰ\u{313}~ᾰ~ῐᾱ", "ᾰ\u{313}~ᾰ~ῐᾱ", "ᾰ\u{313}~ᾰ~ῐαι";
        genitive: "ᾰ\u{313}~ᾰ~ῐᾱς", "ᾰ\u{313}~ᾰ~ῐαιν", "ᾰ\u{313}~ᾰ~ῐων";
        dative: "ᾰ\u{313}~ᾰ~ῐᾳ", "ᾰ\u{313}~ᾰ~ῐαιν", "ᾰ\u{313}~ᾰ~ῐαις";
        accusative: "ᾰ\u{313}~ᾰ~ῐᾱν", "ᾰ\u{313}~ᾰ~ῐᾱ", "ᾰ\u{313}~ᾰ~ῐᾱς";
        vocative: "ᾰ\u{313}~ᾰ~ῐᾱ", "ᾰ\u{313}~ᾰ~ῐᾱ", "ᾰ\u{313}~ᾰ~ῐαι";
    ),
    // Μωλῠ: 7 words (δᾰκρῠ, μωλῠ, νᾱπῠ, μῐσῠ, βρᾰθῠ, σωρῠ).
    declension!("μωλῠ", 7, "~ῠ";
        nominative: "~ῠ", "~ῠε|~ῡ", "~ῠᾰ";
        genitive: "~ῠος", "~ῠοιν", "~ῠων";
        dative: "~ῠῐ\u{308}|~υι", "~ῠοιν", "~ῠσῐ|~ῠσῐν";
        accusative: "~ῠ", "~ῠε|~ῡ", "~ῠᾰ";
        vocative: "~ῠ", "~ῠε|~ῡ", "~ῠᾰ";
    ),
    // Ὑδρα: 7 words (ὑαινα, ὑδρα, ὑστερα, ὑπερῳα, ὑποτεινουσα, ὑστεραια).
    declension!("ὑδρα", 7, "ὑ~α";
        nominative: "ῠ\u{314}~ᾱ", "ῠ\u{314}~ᾱ", "ῠ\u{314}~αι";
        genitive: "ῠ\u{314}~ᾱς", "ῠ\u{314}~αιν", "ῠ\u{314}~ων";
        dative: "ῠ\u{314}~ᾳ", "ῠ\u{314}~αιν", "ῠ\u{314}~αις";
        accusative: "ῠ\u{314}~ᾱν", "ῠ\u{314}~ᾱ", "ῠ\u{314}~ᾱς";
        vocative: "ῠ\u{314}~ᾱ", "ῠ\u{314}~ᾱ", "ῠ\u{314}~αι";
    ),
    // Χεῑμᾰ: 7 words (χεῑμᾰ, δεῑγμᾰ, σεῑσμᾰ, λεῑμμᾰ, λαῑτμᾰ, πεῑσμᾰ).
    declension!("χεῑμᾰ", 7, "~ῑ~";
        nominative: "~ῑ~", "~ι~τε", "~ι~τᾰ";
        genitive: "~ι~τος", "~ι~τοιν", "~ι~των";
        dative: "~ι~τῐ", "~ι~τοιν", "~ι~σῐ|~ι~σῐν";
        accusative: "~ῑ~", "~ι~τε", "~ι~τᾰ";
        vocative: "~ῑ~", "~ι~τε", "~ι~τᾰ";
    ),
    // Ἀναγωγη: 7 words (ἀστραπη, ἀνατροπη, ἀνατροφη, ἀναγωγη, ἀναστροφη, ἀναβολη).
    declension!("ἀναγωγη", 7, "ἀ~α~η";
        nominative: "ᾰ\u{313}~ᾰ~η", "ᾰ\u{313}~ᾰ~ᾱ", "ᾰ\u{313}~ᾰ~αι";
        genitive: "ᾰ\u{313}~ᾰ~ης", "ᾰ\u{313}~ᾰ~αῑν", "ᾰ\u{313}~ᾰ~ων";
        dative: "ᾰ\u{313}~ᾰ~ῃ", "ᾰ\u{313}~ᾰ~αῑν", "ᾰ\u{313}~ᾰ~αῑς";
        accusative: "ᾰ\u{313}~ᾰ~ην", "ᾰ\u{313}~ᾰ~ᾱ", "ᾰ\u{313}~ᾰ~ᾱς";
        vocative: "ᾰ\u{313}~ᾰ~η", "ᾰ\u{313}~ᾰ~ᾱ", "ᾰ\u{313}~ᾰ~αι";
    ),
    // Ἀδᾰμᾱς: 7 words (ἀπους, ἀδᾰμᾱς, ἀδελφοτης, ἀδηλοτης, ἀμαυροτης, ἀλῠς).
    declension!("ἀδᾰμᾱς", 7, "ἀ~ς";
        nominative: "ᾰ\u{313}~ς", "ᾰ\u{313}~τε", "ᾰ\u{313}~τες";
        genitive: "ᾰ\u{313}~τος", "ᾰ\u{313}~τοιν", "ᾰ\u{313}~των";
        dative: "ᾰ\u{313}~τῐ", "ᾰ\u{313}~τοιν", "ᾰ\u{313}~σῐ|ᾰ\u{313}~σῐν";
        accusative: "ᾰ\u{313}~τᾰ", "ᾰ\u{313}~τε", "ᾰ\u{313}~τᾰς";
        vocative: "ᾰ\u{313}~ς", "ᾰ\u{313}~τε", "ᾰ\u{313}~τες";
    ),
    // Ἀναθεμα: 7 words (ἀκανθα, ἀσφαλεια, ἀκρατεια, ἀγαλμα, ἀναθεμα, ἀναστημα).
    declension!("ἀναθεμα", 7, "ἀ~α~α";
        nominative: "ᾰ\u{313}~ᾰ~ᾰ", "ᾰ\u{313}~ᾰ~ᾰτε", "ᾰ\u{313}~ᾰ~ᾰτᾰ";
        genitive: "ᾰ\u{313}~ᾰ~ᾰτος", "ᾰ\u{313}~ᾰ~ᾰτοιν", "ᾰ\u{313}~ᾰ~ᾰτων";
        dative: "ᾰ\u{313}~ᾰ~ᾰτῐ", "ᾰ\u{313}~ᾰ~ᾰτοιν", "ᾰ\u{313}~ᾰ~ᾰσῐ|ᾰ\u{313}~ᾰ~ᾰσῐν";
        accusative: "ᾰ\u{313}~ᾰ~ᾰ", "ᾰ\u{313}~ᾰ~ᾰτε", "ᾰ\u{313}~ᾰ~ᾰτᾰ";
        vocative: "ᾰ\u{313}~ᾰ~ᾰ", "ᾰ\u{313}~ᾰ~ᾰτε", "ᾰ\u{313}~ᾰ~ᾰτᾰ";
    ),
    // Μᾰρῐκᾱς: 7 words (πᾰξᾰμᾱς, μᾰρῐκᾱς, ἀττᾰγᾱς, μᾰσουχᾱς, βᾰσκᾱς, χαλιφᾱς).
    declension!("μᾰρῐκᾱς", 7, "~ᾱς";
        nominative: "~ᾱς", "~ᾱ", "~αῑ";
        genitive: "~ᾱ", "~αῑν", "~ων";
        dative: "~ᾳ", "~αῑν", "~αῑς";
        accusative: "~ᾱν", "~ᾱ", "~ᾱς";
        vocative: "~ᾱ", "~ᾱ", "~αῑ";
    ),
    // Ἀργεστης: 7 words (ἀητης, ἀργοναυτης, ἀποδημητης, ἀβροτονῑτης, ἀρχῐτελωνης, ἀγορητης).
    declension!("ἀργεστης", 7, "ἀ~ης";
        nominative: "ᾰ\u{313}~ης", "ᾰ\u{313}~ᾱ", "ᾰ\u{313}~αι";
        genitive: "ᾰ\u{313}~ου", "ᾰ\u{313}~αιν", "ᾰ\u{313}~ων";
        dative: "ᾰ\u{313}~ῃ", "ᾰ\u{313}~αιν", "ᾰ\u{313}~αις";
        accusative: "ᾰ\u{313}~ην", "ᾰ\u{313}~ᾱ", "ᾰ\u{313}~ᾱς";
        vocative: "ᾰ\u{313}~ᾰ", "ᾰ\u{313}~ᾱ", "ᾰ\u{313}~αι";
    ),
    // Ἀραχνη: 6 words (ἀγαπη, ἀπατη, ἀναγκη, ἀραχνη, ἀγκαλη, ἀφακη).
    declension!("ἀραχνη", 6, "ἀ~α~η";
        nominative: "ᾰ\u{313}~ᾰ~η", "ᾰ\u{313}~ᾰ~ᾱ", "ᾰ\u{313}~ᾰ~αι";
        genitive: "ᾰ\u{313}~ᾰ~ης", "ᾰ\u{313}~ᾰ~αιν", "ᾰ\u{313}~ᾰ~ων";
        dative: "ᾰ\u{313}~ᾰ~ῃ", "ᾰ\u{313}~ᾰ~αιν", "ᾰ\u{313}~ᾰ~αις";
        accusative: "ᾰ\u{313}~ᾰ~ην", "ᾰ\u{313}~ᾰ~ᾱ", "ᾰ\u{313}~ᾰ~ᾱς";
        vocative: "ᾰ\u{313}~ᾰ~η", "ᾰ\u{313}~ᾰ~ᾱ", "ᾰ\u{313}~ᾰ~αι";
    ),
    // Μοῑρᾰ: 6 words (μοῑρᾰ, στεῑρᾰ, πεῑρᾰ, σφαῑρᾰ, σπεῑρᾰ, ἀγρεῑφνᾰ).
    declension!("μοῑρᾰ", 6, "~ῑ~ᾰ";
        nominative: "~ῑ~ᾰ", "~ι~ᾱ", "~ῑ~αι";
        genitive: "~ι~ᾱς", "~ι~αιν", "~ι~ων";
        dative: "~ι~ᾳ", "~ι~αιν", "~ι~αις";
        accusative: "~ῑ~ᾰν", "~ι~ᾱ", "~ι~ᾱς";
        vocative: "~ῑ~ᾰ", "~ι~ᾱ", "~ῑ~αι";
    ),
    // Ἀγορα: 6 words (ἀκρα, ἀγορα, ἀγρα, ἀνδρεια, ἀρα, ἀλεα).
    declension!("ἀγορα", 6, "ἀ~α";
        nominative: "ᾰ\u{313}~ᾱ", "ᾰ\u{313}~ᾱ", "ᾰ\u{313}~αι";
        genitive: "ᾰ\u{313}~ᾱς", "ᾰ\u{313}~αιν", "ᾰ\u{313}~ων";
        dative: "ᾰ\u{313}~ᾳ", "ᾰ\u{313}~αιν", "ᾰ\u{313}~αις";
        accusative: "ᾰ\u{313}~ᾱν", "ᾰ\u{313}~ᾱ", "ᾰ\u{313}~ᾱς";
        vocative: "ᾰ\u{313}~ᾱ", "ᾰ\u{313}~ᾱ", "ᾰ\u{313}~αι";
    ),
    // Ἀλῐβᾱς: 6 words (κορῠβᾱς, γῐγᾱς, ἐλεφᾱς, λῠκᾰβᾱς, ἀλῐβᾱς, γᾰρᾰμᾱς).
    declension!("ἀλῐβᾱς", 6, "~ᾱς";
        nominative: "~ᾱς", "~ᾰντε", "~ᾰντες";
        genitive: "~ᾰντος", "~ᾰντοιν", "~ᾰντων";
        dative: "~ᾰντῐ", "~ᾰντοιν", "~ᾱσῐ|~ᾱσῐν";
        accusative: "~ᾰντᾰ", "~ᾰντε", "~ᾰντᾰς";
        vocative: "~ᾰν", "~ᾰντε", "~ᾰντες";
    ),
    // Μητρως: 6 words (τρως, ἡρως, δμως, πᾰτρως, θως, μητρως).
    declension!("μητρως", 6, "~ως";
        nominative: "~ως", "~ωε", "~ωες";
        genitive: "~ωος", "~ωοιν", "~ωων";
        dative: "~ῳ", "~ωοιν", "~ωσῐ|~ωσῐν";
        accusative: "~ω", "~ωε", "~ωᾰς";
        vocative: "~ως", "~ωε", "~ωες";
    ),
    // Ἱστος: 6 words (ἱππος, ἱπποκενταυρος, ἱστος, ἱπποδρομος, ἱπποκανθαρος, ἱππουρος).
    declension!("ἱστος", 6, "ἱ~ος";
        nominative: "ῐ\u{314}~ος", "ῐ\u{314}~ω", "ῐ\u{314}~οι";
        genitive: "ῐ\u{314}~ου", "ῐ\u{314}~οιν", "ῐ\u{314}~ων";
        dative: "ῐ\u{314}~ῳ", "ῐ\u{314}~οιν", "ῐ\u{314}~οις";
        accusative: "ῐ\u{314}~ον", "ῐ\u{314}~ω", "ῐ\u{314}~ους";
        vocative: "ῐ\u{314}~ε", "ῐ\u{314}~ω", "ῐ\u{314}~οι";
    ),
    // Ἀμεθυστος: 6 words (ἀργυρος, ἀνδρογυνος, ἀμεθυστος, ἀργυροτοξος, ἀργυροκοπος, ἀστυνομος).
    declension!("ἀμεθυστος", 6, "ἀ~υ~ος";
        nominative: "ᾰ\u{313}~ῠ~ος", "ᾰ\u{313}~ῠ~ω", "ᾰ\u{313}~ῠ~οι";
        genitive: "ᾰ\u{313}~ῠ~ου", "ᾰ\u{313}~ῠ~οιν", "ᾰ\u{313}~ῠ~ων";
        dative: "ᾰ\u{313}~ῠ~ῳ", "ᾰ\u{313}~ῠ~οιν", "ᾰ\u{313}~ῠ~οις";
        accusative: "ᾰ\u{313}~ῠ~ον", "ᾰ\u{313}~ῠ~ω", "ᾰ\u{313}~ῠ~ους";
        vocative: "ᾰ\u{313}~ῠ~ε", "ᾰ\u{313}~ῠ~ω", "ᾰ\u{313}~ῠ~οι";
    ),
    // Κῠᾰρ: 6 words (οὐθᾰρ, ἡπᾰρ, φρεᾱρ, στεᾱρ, δελεᾰρ, κῠᾰρ).
    declension!("κῠᾰρ", 6, "~ρ";
        nominative: "~ρ", "~τε", "~τᾰ";
        genitive: "~τος", "~τοιν", "~των";
        dative: "~τῐ", "~τοιν", "~σῐ|~σῐν";
        accusative: "~ρ", "~τε", "~τᾰ";
        vocative: "~ρ", "~τε", "~τᾰ";
    ),
    // Ὑπᾰκοη: 6 words (ὑφη, ὑπομονη, ὑποθηκη, ὑποδοχη, ὑρχη, ὑπᾰκοη).
    declension!("ὑπᾰκοη", 6, "ὑ~η";
        nominative: "ῠ\u{314}~η", "ῠ\u{314}~ᾱ", "ῠ\u{314}~αι";
        genitive: "ῠ\u{314}~ης", "ῠ\u{314}~αῑν", "ῠ\u{314}~ων";
        dative: "ῠ\u{314}~ῃ", "ῠ\u{314}~αῑν", "ῠ\u{314}~αῑς";
        accusative: "ῠ\u{314}~ην", "ῠ\u{314}~ᾱ", "ῠ\u{314}~ᾱς";
        vocative: "ῠ\u{314}~η", "ῠ\u{314}~ᾱ", "ῠ\u{314}~αι";
    ),
    // Ναυτης: 6 words (ναυτης, θραυστης, κοκκοθραυστης, ἐμφῠτευτης, ψευστης, πῠραυστης).
    declension!("ναυτης", 6, "~υ~ης";
        nominative: "~υ~ης", "~υ~ᾱ", "~ῡ~αι";
        genitive: "~υ~ου", "~υ~αιν", "~υ~ων";
        dative: "~υ~ῃ", "~υ~αιν", "~υ~αις";
        accusative: "~υ~ην", "~υ~ᾱ", "~υ~ᾱς";
        vocative: "~ῡ~ᾰ", "~υ~ᾱ", "~ῡ~αι";
    ),
    // Ὑποθεσις: 6 words (ὑποθεσις, ὑβρις, ὑφηγησις, ὑποσχεσις, ὑφαιρεσις, ὑποληψις).
    declension!("ὑποθεσις", 6, "ὑ~ις";
        nominative: "ῠ\u{314}~ῐς", "ῠ\u{314}~ει", "ῠ\u{314}~εις";
        genitive: "ῠ\u{314}~εως", "ῠ\u{314}~εοιν", "ῠ\u{314}~εων";
        dative: "ῠ\u{314}~ει", "ῠ\u{314}~εοιν", "ῠ\u{314}~εσῐ|ῠ\u{314}~εσῐν";
        accusative: "ῠ\u{314}~ῐν", "ῠ\u{314}~ει", "ῠ\u{314}~εις";
        vocative: "ῠ\u{314}~ῐ", "ῠ\u{314}~ει", "ῠ\u{314}~εις";
    ),
    // Ἀσπις: 6 words (ἀσπις, ἀγρωστις, ἀκις, ἀμις, ἀκροστῐχις, ἀνθεμις).
    declension!("ἀσπις", 6, "ἀ~ις";
        nominative: "ᾰ\u{313}~ῐς", "ᾰ\u{313}~ῐδε", "ᾰ\u{313}~ῐδες";
        genitive: "ᾰ\u{313}~ῐδος", "ᾰ\u{313}~ῐδοιν", "ᾰ\u{313}~ῐδων";
        dative: "ᾰ\u{313}~ῐδῐ", "ᾰ\u{313}~ῐδοιν", "ᾰ\u{313}~ῐσῐ|ᾰ\u{313}~ῐσῐν";
        accusative: "ᾰ\u{313}~ῐδᾰ", "ᾰ\u{313}~ῐδε", "ᾰ\u{313}~ῐδᾰς";
        vocative: "ᾰ\u{313}~ῐ", "ᾰ\u{313}~ῐδε", "ᾰ\u{313}~ῐδες";
    ),
    // Πηχῠς: 6 words (ἡμῐσῠς, υἱῠς, πελεκῠς, πρεσβῠς, πηχῠς, τρῐβρᾰχῠς).
    declension!("πηχῠς", 6, "~ῠς";
        nominative: "~ῠς", "~ει", "~εις";
        genitive: "~εως", "~εοιν", "~εων";
        dative: "~ει", "~εοιν", "~εσῐ|~εσῐν";
        accusative: "~ῠν", "~ει", "~εις";
        vocative: "~ῠ", "~ει", "~εις";
    ),
    // Ὑπερῳον: 6 words (ὑπογειον, ὑπερῳον, ὑπνον, ὑπηκοον, ὑδνον, ὑποκαυστον).
    declension!("ὑπερῳον", 6, "ὑ~ον";
        nominative: "ῠ\u{314}~ον", "ῠ\u{314}~ω", "ῠ\u{314}~ᾰ";
        genitive: "ῠ\u{314}~ου", "ῠ\u{314}~οιν", "ῠ\u{314}~ων";
        dative: "ῠ\u{314}~ῳ", "ῠ\u{314}~οιν", "ῠ\u{314}~οις";
        accusative: "ῠ\u{314}~ον", "ῠ\u{314}~ω", "ῠ\u{314}~ᾰ";
        vocative: "ῠ\u{314}~ον", "ῠ\u{314}~ω", "ῠ\u{314}~ᾰ";
    ),
    // Παῡλᾰ: 6 words (μοῡσᾰ, βοῡλλᾰ, πᾰνδοῡρᾰ, παῡλᾰ, γοῡρνᾰ, κοῡπᾰ).
    declension!("παῡλᾰ", 6, "~ῡ~ᾰ";
        nominative: "~ῡ~ᾰ", "~υ~ᾱ", "~ῡ~αι";
        genitive: "~υ~ης", "~υ~αιν", "~υ~ων";
        dative: "~υ~ῃ", "~υ~αιν", "~υ~αις";
        accusative: "~ῡ~ᾰν", "~υ~ᾱ", "~υ~ᾱς";
        vocative: "~ῡ~ᾰ", "~υ~ᾱ", "~ῡ~αι";
    ),
    // Ἡρωϊς: 6 words (ὀϊς, ἡρωϊς, ἐχενηϊς, ληϊς, ἀχερωϊς, αἱμορροϊς).
    declension!("ἡρωϊς", 6, "~ϊς";
        nominative: "~ῐ\u{308}ς", "~ῐ\u{308}δε", "~ῐ\u{308}δες";
        genitive: "~ῐ\u{308}δος", "~ῐ\u{308}δοιν", "~ῐ\u{308}δων";
        dative: "~ῐ\u{308}δῐ", "~ῐ\u{308}δοιν", "~ῐ\u{308}σῐ|~ῐ\u{308}σῐν";
        accusative: "~ῐ\u{308}δᾰ", "~ῐ\u{308}δε", "~ῐ\u{308}δᾰς";
        vocative: "~ῐ\u{308}", "~ῐ\u{308}δε", "~ῐ\u{308}δες";
    ),
    // Γεῑσον: 6 words (δεῑπνον, σπεῑρον, κλεῑθρον, σεῑστρον, ῥεῑθρον, γεῑσον).
    declension!("γεῑσον", 6, "~ῑ~ον";
        nominative: "~ῑ~ον", "~ι~ω", "~ῑ~ᾰ";
        genitive: "~ι~ου", "~ι~οιν", "~ι~ων";
        dative: "~ι~ῳ", "~ι~οιν", "~ι~οις";
        accusative: "~ῑ~ον", "~ι~ω", "~ῑ~ᾰ";
        vocative: "~ῑ~ον", "~ι~ω", "~ῑ~ᾰ";
    ),
    // Πᾰπᾱς: 6 words (πᾰπᾱς, γᾰλᾰτᾱς, κᾰδῡτᾱς, γενετᾱς, σᾰκτᾱς, πᾰππᾱς).
    declension!("πᾰπᾱς", 6, "~ᾱς";
        nominative: "~ᾱς", "~ᾱ", "~αι";
        genitive: "~ου", "~αιν", "~ων";
        dative: "~ᾳ", "~αιν", "~αις";
        accusative: "~ᾱν", "~ᾱ", "~ᾱς";
        vocative: "~ᾰ", "~ᾱ", "~αι";
    ),
    // Ἀλκαιᾱ: 6 words (ἀνῑᾱ, ἀγγᾰρειᾱ, ἀμφῐβολῐᾱ, ἀκρῐτομῡθιᾱ, ἀλκαιᾱ, ἀλκεᾱ).
    declension!("ἀλκαιᾱ", 6, "ἀ~ᾱ";
        nominative: "ᾰ\u{313}~ᾱ", "ᾰ\u{313}~ᾱ", "ᾰ\u{313}~αι";
        genitive: "ᾰ\u{313}~ᾱς", "ᾰ\u{313}~αιν", "ᾰ\u{313}~ων";
        dative: "ᾰ\u{313}~ᾳ", "ᾰ\u{313}~αιν", "ᾰ\u{313}~αις";
        accusative: "ᾰ\u{313}~ᾱν", "ᾰ\u{313}~ᾱ", "ᾰ\u{313}~ᾱς";
        vocative: "ᾰ\u{313}~ᾱ", "ᾰ\u{313}~ᾱ", "ᾰ\u{313}~αι";
    ),
    // Γερᾰς: 6 words (γερᾰς, δεπᾰς, ψεφᾰς, σφελᾰς, βουκερᾰς, σκεπᾰς).
    declension!("γερᾰς", 6, "~ς";
        nominative: "~ς", "~ε", "~ᾰ";
        genitive: "~ος", "~οιν", "~ων";
        dative: "~ῐ\u{308}", "~οιν", "~σῐ";
        accusative: "~ς", "~ε", "~ᾰ";
        vocative: "~ς", "~ε", "~ᾰ";
    ),
    // Σφην: 5 words (μην, σπλην, κλων, σφην, ψην).
    declension!("σφην", 5, "~ν";
        nominative: "~ν", "~νε", "~νες";
        genitive: "~νος", "~νοῑν", "~νων";
        dative: "~νῐ", "~νοῑν", "~σῐ|~σῐν";
        accusative: "~νᾰ", "~νε", "~νᾰς";
        vocative: "~ν", "~νε", "~νες";
    ),
    // Ἀνατασις: 5 words (ἀναστασις, ἀγανακτησις, ἀναταξις, ἀνατασις, ἀναβασις).
    declension!("ἀνατασις", 5, "ἀ~α~α~ις";
        nominative: "ᾰ\u{313}~ᾰ~ᾰ~ῐς", "ᾰ\u{313}~ᾰ~ᾰ~ει", "ᾰ\u{313}~ᾰ~ᾰ~εις";
        genitive: "ᾰ\u{313}~ᾰ~ᾰ~εως", "ᾰ\u{313}~ᾰ~ᾰ~εοιν", "ᾰ\u{313}~ᾰ~ᾰ~εων";
        dative: "ᾰ\u{313}~ᾰ~ᾰ~ει", "ᾰ\u{313}~ᾰ~ᾰ~εοιν", "ᾰ\u{313}~ᾰ~ᾰ~εσῐ|ᾰ\u{313}~ᾰ~ᾰ~εσῐν";
        accusative: "ᾰ\u{313}~ᾰ~ᾰ~ῐν", "ᾰ\u{313}~ᾰ~ᾰ~ει", "ᾰ\u{313}~ᾰ~ᾰ~εις";
        vocative: "ᾰ\u{313}~ᾰ~ᾰ~ῐ", "ᾰ\u{313}~ᾰ~ᾰ~ει", "ᾰ\u{313}~ᾰ~ᾰ~εις";
    ),
    // Κωμῡς: 5 words (μᾰρτῠς, νομᾰς, κωμῡς, φῠγᾰς, ἐμῠς).
    declension!("κωμῡς", 5, "~ς";
        nominative: "~ς", "~δε", "~δες";
        genitive: "~δος", "~δοιν", "~δων";
        dative: "~δῐ", "~δοιν", "~σῐν";
        accusative: "~δᾰ", "~δε", "~δᾰς";
        vocative: "~ς", "~δε", "~δες";
    ),
    // Ἀνισον: 5 words (ἀλφιτον, ἀνισον, ἀγκιστρον, ἀντιθετον, ἀριστερον).
    declension!("ἀνισον", 5, "ἀ~ι~ον";
        nominative: "ᾰ\u{313}~ῐ~ον", "ᾰ\u{313}~ῐ~ω", "ᾰ\u{313}~ῐ~ᾰ";
        genitive: "ᾰ\u{313}~ῐ~ου", "ᾰ\u{313}~ῐ~οιν", "ᾰ\u{313}~ῐ~ων";
        dative: "ᾰ\u{313}~ῐ~ῳ", "ᾰ\u{313}~ῐ~οιν", "ᾰ\u{313}~ῐ~οις";
        accusative: "ᾰ\u{313}~ῐ~ον", "ᾰ\u{313}~ῐ~ω", "ᾰ\u{313}~ῐ~ᾰ";
        vocative: "ᾰ\u{313}~ῐ~ον", "ᾰ\u{313}~ῐ~ω", "ᾰ\u{313}~ῐ~ᾰ";
    ),
    // Γεῡσῐς: 5 words (παῡσῐς, λοῡσῐς, ῥεῡσῐς, ζεῡξῐς, γεῡσῐς).
    declension!("γεῡσῐς", 5, "~ῡ~ῐς";
        nominative: "~ῡ~ῐς", "~υ~ει", "~υ~εις";
        genitive: "~υ~εως", "~υ~εοιν", "~υ~εων";
        dative: "~υ~ει", "~υ~εοιν", "~υ~εσῐ|~υ~εσῐν";
        accusative: "~ῡ~ῐν", "~υ~ει", "~υ~εις";
        vocative: "~ῡ~ῐ", "~υ~ει", "~υ~εις";
    ),
    // Νοῡς: 5 words (νοῡς, πλοῡς, θῠγατρῐδοῡς, ῥοῡς, λουτροχοῡς).
    declension!("νοῡς", 5, "~οῡς";
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
    declension!("δαιμων", 5, "~ι~ων";
        nominative: "~ι~ων", "~ι~ονε", "~ι~ονες";
        genitive: "~ι~ονος", "~ι~ονοιν", "~ι~ονων";
        dative: "~ι~ονῐ", "~ι~ονοιν", "~ι~οσῐ|~ι~οσῐν";
        accusative: "~ι~ονᾰ", "~ι~ονε", "~ι~ονᾰς";
        vocative: "~ῑ~ον", "~ι~ονε", "~ι~ονες";
    ),
    // Ἀστραγᾰλος: 5 words (ἀρακος, ἀστραγᾰλος, ἀστακος, ἀραβος, ἀμαθος).
    declension!("ἀστραγᾰλος", 5, "ἀ~α~ος";
        nominative: "ᾰ\u{313}~ᾰ~ος", "ᾰ\u{313}~ᾰ~ω", "ᾰ\u{313}~ᾰ~οι";
        genitive: "ᾰ\u{313}~ᾰ~ου", "ᾰ\u{313}~ᾰ~οιν", "ᾰ\u{313}~ᾰ~ων";
        dative: "ᾰ\u{313}~ᾰ~ῳ", "ᾰ\u{313}~ᾰ~οιν", "ᾰ\u{313}~ᾰ~οις";
        accusative: "ᾰ\u{313}~ᾰ~ον", "ᾰ\u{313}~ᾰ~ω", "ᾰ\u{313}~ᾰ~ους";
        vocative: "ᾰ\u{313}~ᾰ~ε", "ᾰ\u{313}~ᾰ~ω", "ᾰ\u{313}~ᾰ~οι";
    ),
    // Τεῡχος: 5 words (τεῡχος, γλεῡκος, ψεῡδος, ζεῡγος, κεῡθος).
    declension!("τεῡχος", 5, "~ῡ~ος";
        nominative: "~ῡ~ος", "~υ~ει", "~υ~η";
        genitive: "~υ~ους", "~υ~οῑν", "~υ~ων";
        dative: "~υ~ει", "~υ~οῑν", "~υ~εσῐ|~υ~εσῐν";
        accusative: "~ῡ~ος", "~υ~ει", "~υ~η";
        vocative: "~ῡ~ος", "~υ~ει", "~υ~η";
    ),
    // Χλουνης: 5 words (τῠμβαυλης, χλουνης, σῐδευνης, χοραυλης, πρωταυλης).
    declension!("χλουνης", 5, "~υ~ης";
        nominative: "~υ~ης", "~υ~ᾱ", "~ῡ~αι";
        genitive: "~υ~ου", "~υ~αιν", "~υ~ων";
        dative: "~υ~ῃ", "~υ~αιν", "~υ~αις";
        accusative: "~υ~ην", "~υ~ᾱ", "~υ~ᾱς";
        vocative: "~υ~η", "~υ~ᾱ", "~ῡ~αι";
    ),
    // Ἀγρευς: 5 words (ἀμφορευς, ἀποστολευς, ἀγρευς, ἀλθευς, ἀμφῐφορευς).
    declension!("ἀγρευς", 5, "ἀ~ευς";
        nominative: "ᾰ\u{313}~ευς", "ᾰ\u{313}~η", "ᾰ\u{313}~ης|ᾰ\u{313}~εῑς";
        genitive: "ᾰ\u{313}~εως", "ᾰ\u{313}~εοιν", "ᾰ\u{313}~εων";
        dative: "ᾰ\u{313}~εῑ", "ᾰ\u{313}~εοιν", "ᾰ\u{313}~εῡσῐ|ᾰ\u{313}~εῡσῐν";
        accusative: "ᾰ\u{313}~εᾱ", "ᾰ\u{313}~η", "ᾰ\u{313}~εᾱς";
        vocative: "ᾰ\u{313}~εῡ", "ᾰ\u{313}~η", "ᾰ\u{313}~ης|ᾰ\u{313}~εῑς";
    ),
    // Λεῑμᾰξ: 5 words (χοῑνῐξ, κᾰταῑτῠξ, μεῑρᾰξ, λεῑμᾰξ, βαῑβῠξ).
    declension!("λεῑμᾰξ", 5, "~ῑ~ξ";
        nominative: "~ῑ~ξ", "~ι~κε", "~ι~κες";
        genitive: "~ι~κος", "~ι~κοιν", "~ι~κων";
        dative: "~ι~κῐ", "~ι~κοιν", "~ι~ξῐ|~ι~ξῐν";
        accusative: "~ι~κᾰ", "~ι~κε", "~ι~κᾰς";
        vocative: "~ῑ~ξ", "~ι~κε", "~ι~κες";
    ),
    // Λαυρᾱ: 4 words (σαυρᾱ, λαυρᾱ, κᾰρουχᾱ, βρουτζᾱ).
    declension!("λαυρᾱ", 4, "~υ~ᾱ";
        nominative: "~υ~ᾱ", "~υ~ᾱ", "~ῡ~αι";
        genitive: "~υ~ᾱς", "~υ~αιν", "~υ~ων";
        dative: "~υ~ᾳ", "~υ~αιν", "~υ~αις";
        accusative: "~υ~ᾱν", "~υ~ᾱ", "~υ~ᾱς";
        vocative: "~υ~ᾱ", "~υ~ᾱ", "~ῡ~αι";
    ),
    // Ἰσηγορια: 4 words (ἰδια, ἰσονομια, ἰσοψηφια, ἰσηγορια).
    declension!("ἰσηγορια", 4, "ἰ~ια";
        nominative: "ῐ\u{313}~ῐᾱ", "ῐ\u{313}~ῐᾱ", "ῐ\u{313}~ῐαι";
        genitive: "ῐ\u{313}~ῐᾱς", "ῐ\u{313}~ῐαιν", "ῐ\u{313}~ῐων";
        dative: "ῐ\u{313}~ῐᾳ", "ῐ\u{313}~ῐαιν", "ῐ\u{313}~ῐαις";
        accusative: "ῐ\u{313}~ῐᾱν", "ῐ\u{313}~ῐᾱ", "ῐ\u{313}~ῐᾱς";
        vocative: "ῐ\u{313}~ῐᾱ", "ῐ\u{313}~ῐᾱ", "ῐ\u{313}~ῐαι";
    ),
    // Πλευμων: 4 words (πλευμων, πνευμων, μουσμων, ἰχνευμων).
    declension!("πλευμων", 4, "~υ~ων";
        nominative: "~υ~ων", "~υ~ονε", "~υ~ονες";
        genitive: "~υ~ονος", "~υ~ονοιν", "~υ~ονων";
        dative: "~υ~ονῐ", "~υ~ονοιν", "~υ~οσῐ|~υ~οσῐν";
        accusative: "~υ~ονᾰ", "~υ~ονε", "~υ~ονᾰς";
        vocative: "~ῡ~ον", "~υ~ονε", "~υ~ονες";
    ),
    // Ὑπομνημα: 4 words (ὑποστρωμα, ὑπομνημα, ὑποδημα, ὑσμα).
    declension!("ὑπομνημα", 4, "ὑ~α";
        nominative: "ῠ\u{314}~ᾰ", "ῠ\u{314}~ᾰτε", "ῠ\u{314}~ᾰτᾰ";
        genitive: "ῠ\u{314}~ᾰτος", "ῠ\u{314}~ᾰτοιν", "ῠ\u{314}~ᾰτων";
        dative: "ῠ\u{314}~ᾰτῐ", "ῠ\u{314}~ᾰτοιν", "ῠ\u{314}~ᾰσῐ|ῠ\u{314}~ᾰσῐν";
        accusative: "ῠ\u{314}~ᾰ", "ῠ\u{314}~ᾰτε", "ῠ\u{314}~ᾰτᾰ";
        vocative: "ῠ\u{314}~ᾰ", "ῠ\u{314}~ᾰτε", "ῠ\u{314}~ᾰτᾰ";
    ),
    // Ὑπαρξις: 4 words (ὑπαρξις, ὑποταξις, ὑπερταξις, ὑπογραφις).
    declension!("ὑπαρξις", 4, "ὑ~α~ις";
        nominative: "ῠ\u{314}~ᾰ~ῐς", "ῠ\u{314}~ᾰ~ει", "ῠ\u{314}~ᾰ~εις";
        genitive: "ῠ\u{314}~ᾰ~εως", "ῠ\u{314}~ᾰ~εοιν", "ῠ\u{314}~ᾰ~εων";
        dative: "ῠ\u{314}~ᾰ~ει", "ῠ\u{314}~ᾰ~εοιν", "ῠ\u{314}~ᾰ~εσῐ|ῠ\u{314}~ᾰ~εσῐν";
        accusative: "ῠ\u{314}~ᾰ~ῐν", "ῠ\u{314}~ᾰ~ει", "ῠ\u{314}~ᾰ~εις";
        vocative: "ῠ\u{314}~ᾰ~ῐ", "ῠ\u{314}~ᾰ~ει", "ῠ\u{314}~ᾰ~εις";
    ),
    // Ἀκροβυστια: 4 words (ἀσφυξια, ἀκροβυστια, ἀμβλυωπια, ἀσυμμετρια).
    declension!("ἀκροβυστια", 4, "ἀ~υ~ια";
        nominative: "ᾰ\u{313}~ῠ~ῐᾱ", "ᾰ\u{313}~ῠ~ῐᾱ", "ᾰ\u{313}~ῠ~ῐαι";
        genitive: "ᾰ\u{313}~ῠ~ῐᾱς", "ᾰ\u{313}~ῠ~ῐαιν", "ᾰ\u{313}~ῠ~ῐων";
        dative: "ᾰ\u{313}~ῠ~ῐᾳ", "ᾰ\u{313}~ῠ~ῐαιν", "ᾰ\u{313}~ῠ~ῐαις";
        accusative: "ᾰ\u{313}~ῠ~ῐᾱν", "ᾰ\u{313}~ῠ~ῐᾱ", "ᾰ\u{313}~ῠ~ῐᾱς";
        vocative: "ᾰ\u{313}~ῠ~ῐᾱ", "ᾰ\u{313}~ῠ~ῐᾱ", "ᾰ\u{313}~ῠ~ῐαι";
    ),
    // Ἀποτελεσμα: 4 words (ἀγνοημα, ἀκουσμα, ἀσκημα, ἀποτελεσμα).
    declension!("ἀποτελεσμα", 4, "ἀ~α";
        nominative: "ᾰ\u{313}~ᾰ", "ᾰ\u{313}~ᾰτε", "ᾰ\u{313}~ᾰτᾰ";
        genitive: "ᾰ\u{313}~ᾰτος", "ᾰ\u{313}~ᾰτοιν", "ᾰ\u{313}~ᾰτων";
        dative: "ᾰ\u{313}~ᾰτῐ", "ᾰ\u{313}~ᾰτοιν", "ᾰ\u{313}~ᾰσῐ|ᾰ\u{313}~ᾰσῐν";
        accusative: "ᾰ\u{313}~ᾰ", "ᾰ\u{313}~ᾰτε", "ᾰ\u{313}~ᾰτᾰ";
        vocative: "ᾰ\u{313}~ᾰ", "ᾰ\u{313}~ᾰτε", "ᾰ\u{313}~ᾰτᾰ";
    ),
    // Ἰσον: 4 words (ἰχθυδῐον, ἰσον, ἰτον, ἰπνον).
    declension!("ἰσον", 4, "ἰ~ον";
        nominative: "ῐ\u{313}~ον", "ῐ\u{313}~ω", "ῐ\u{313}~ᾰ";
        genitive: "ῐ\u{313}~ου", "ῐ\u{313}~οιν", "ῐ\u{313}~ων";
        dative: "ῐ\u{313}~ῳ", "ῐ\u{313}~οιν", "ῐ\u{313}~οις";
        accusative: "ῐ\u{313}~ον", "ῐ\u{313}~ω", "ῐ\u{313}~ᾰ";
        vocative: "ῐ\u{313}~ον", "ῐ\u{313}~ω", "ῐ\u{313}~ᾰ";
    ),
    // Ἀστερισκος: 4 words (ἀνθρωπισκος, ἀργιλλος, ἀστερισκος, ἀγχινοος).
    declension!("ἀστερισκος", 4, "ἀ~ι~ος";
        nominative: "ᾰ\u{313}~ῐ~ος", "ᾰ\u{313}~ῐ~ω", "ᾰ\u{313}~ῐ~οι";
        genitive: "ᾰ\u{313}~ῐ~ου", "ᾰ\u{313}~ῐ~οιν", "ᾰ\u{313}~ῐ~ων";
        dative: "ᾰ\u{313}~ῐ~ῳ", "ᾰ\u{313}~ῐ~οιν", "ᾰ\u{313}~ῐ~οις";
        accusative: "ᾰ\u{313}~ῐ~ον", "ᾰ\u{313}~ῐ~ω", "ᾰ\u{313}~ῐ~ους";
        vocative: "ᾰ\u{313}~ῐ~ε", "ᾰ\u{313}~ῐ~ω", "ᾰ\u{313}~ῐ~οι";
    ),
    // Κηρῠξ: 4 words (κοκκῠξ, κηρῠξ, ψευδοκηρῠξ, ἱεροκηρῠξ).
    declension!("κηρῠξ", 4, "~ῠξ";
        nominative: "~ῠξ", "~ῡκε", "~ῡκες";
        genitive: "~ῡκος", "~ῡκοιν", "~ῡκων";
        dative: "~ῡκῐ", "~ῡκοιν", "~ῡξῐ|~ῡξῐν";
        accusative: "~ῡκᾰ", "~ῡκε", "~ῡκᾰς";
        vocative: "~ῠξ", "~ῡκε", "~ῡκες";
    ),
    // Ἀκοντιον: 4 words (ἀγγουριον, ἀπιον, ἀκοντιον, ἀλετριον).
    declension!("ἀκοντιον", 4, "ἀ~ιον";
        nominative: "ᾰ\u{313}~ῐον", "ᾰ\u{313}~ῐω", "ᾰ\u{313}~ῐᾰ";
        genitive: "ᾰ\u{313}~ῐου", "ᾰ\u{313}~ῐοιν", "ᾰ\u{313}~ῐων";
        dative: "ᾰ\u{313}~ῐῳ", "ᾰ\u{313}~ῐοιν", "ᾰ\u{313}~ῐοις";
        accusative: "ᾰ\u{313}~ῐον", "ᾰ\u{313}~ῐω", "ᾰ\u{313}~ῐᾰ";
        vocative: "ᾰ\u{313}~ῐον", "ᾰ\u{313}~ῐω", "ᾰ\u{313}~ῐᾰ";
    ),
    // Γαλη: 3 words (γη, μῡγᾰλη, γαλη).
    declension!("γαλη", 3, "~η";
        nominative: "~η", "~ᾱ", "~αῑ";
        genitive: "~ης", "~αῑν", "~ων";
        dative: "~ῃ", "~αῑν", "~αῑς";
        accusative: "~ην", "~ᾱ", "~ᾱς";
        vocative: "~η", "~ᾱ", "~αῑ";
    ),
    // Ἀνακυκλωσις: 3 words (ἀποκαλυψις, ἀνακυκλωσις, ἀνακυκλησις).
    declension!("ἀνακυκλωσις", 3, "ἀ~α~υ~ις";
        nominative: "ᾰ\u{313}~ᾰ~ῠ~ῐς", "ᾰ\u{313}~ᾰ~ῠ~ει", "ᾰ\u{313}~ᾰ~ῠ~εις";
        genitive: "ᾰ\u{313}~ᾰ~ῠ~εως", "ᾰ\u{313}~ᾰ~ῠ~εοιν", "ᾰ\u{313}~ᾰ~ῠ~εων";
        dative: "ᾰ\u{313}~ᾰ~ῠ~ει", "ᾰ\u{313}~ᾰ~ῠ~εοιν", "ᾰ\u{313}~ᾰ~ῠ~εσῐ|ᾰ\u{313}~ᾰ~ῠ~εσῐν";
        accusative: "ᾰ\u{313}~ᾰ~ῠ~ῐν", "ᾰ\u{313}~ᾰ~ῠ~ει", "ᾰ\u{313}~ᾰ~ῠ~εις";
        vocative: "ᾰ\u{313}~ᾰ~ῠ~ῐ", "ᾰ\u{313}~ᾰ~ῠ~ει", "ᾰ\u{313}~ᾰ~ῠ~εις";
    ),
    // Πῠρᾰ: 3 words (πῠρᾰ, πολῑτῐκᾰ, ἐσθλᾰ).
    declension!("πῠρᾰ", 3, "~ᾰ";
        nominative: "~ᾰ", "~ᾱ", "~ᾰ";
        genitive: "~ης", "~αιν", "~ων";
        dative: "~ῃ", "~αιν", "~οῑς";
        accusative: "~ᾰν", "~ᾱ", "~ᾰ";
        vocative: "~ᾰ", "~ᾱ", "~ᾰ";
    ),
    // Ἱκετης: 3 words (ἱπποτοξοτης, ἱπποτης, ἱκετης).
    declension!("ἱκετης", 3, "ἱ~ης";
        nominative: "ῐ\u{314}~ης", "ῐ\u{314}~ᾱ", "ῐ\u{314}~αι";
        genitive: "ῐ\u{314}~ου", "ῐ\u{314}~αιν", "ῐ\u{314}~ων";
        dative: "ῐ\u{314}~ῃ", "ῐ\u{314}~αιν", "ῐ\u{314}~αις";
        accusative: "ῐ\u{314}~ην", "ῐ\u{314}~ᾱ", "ῐ\u{314}~ᾱς";
        vocative: "ῐ\u{314}~ᾰ", "ῐ\u{314}~ᾱ", "ῐ\u{314}~αι";
    ),
    // Ἰδιον: 3 words (ἰδιον, ἰσχιον, ἰπνιον).
    declension!("ἰδιον", 3, "ἰ~ιον";
        nominative: "ῐ\u{313}~ῐον", "ῐ\u{313}~ῐω", "ῐ\u{313}~ῐᾰ";
        genitive: "ῐ\u{313}~ῐου", "ῐ\u{313}~ῐοιν", "ῐ\u{313}~ῐων";
        dative: "ῐ\u{313}~ῐῳ", "ῐ\u{313}~ῐοιν", "ῐ\u{313}~ῐοις";
        accusative: "ῐ\u{313}~ῐον", "ῐ\u{313}~ῐω", "ῐ\u{313}~ῐᾰ";
        vocative: "ῐ\u{313}~ῐον", "ῐ\u{313}~ῐω", "ῐ\u{313}~ῐᾰ";
    ),
    // Ὑποστρατηγος: 3 words (ὑμνογραφος, ὑοφθαλμος, ὑποστρατηγος).
    declension!("ὑποστρατηγος", 3, "ὑ~α~ος";
        nominative: "ῠ\u{314}~ᾰ~ος", "ῠ\u{314}~ᾰ~ω", "ῠ\u{314}~ᾰ~οι";
        genitive: "ῠ\u{314}~ᾰ~ου", "ῠ\u{314}~ᾰ~οιν", "ῠ\u{314}~ᾰ~ων";
        dative: "ῠ\u{314}~ᾰ~ῳ", "ῠ\u{314}~ᾰ~οιν", "ῠ\u{314}~ᾰ~οις";
        accusative: "ῠ\u{314}~ᾰ~ον", "ῠ\u{314}~ᾰ~ω", "ῠ\u{314}~ᾰ~ους";
        vocative: "ῠ\u{314}~ᾰ~ε", "ῠ\u{314}~ᾰ~ω", "ῠ\u{314}~ᾰ~οι";
    ),
    // Δηξ: 3 words (λῠγξ, δηξ, βλᾱξ).
    declension!("δηξ", 3, "~ξ";
        nominative: "~ξ", "~κε", "~κες";
        genitive: "~κος", "~κοῑν", "~κων";
        dative: "~κῐ", "~κοῑν", "~ξῐν";
        accusative: "~κᾰ", "~κε", "~κᾰς";
        vocative: "~ξ", "~κε", "~κες";
    ),
    // Νεῑκος: 3 words (τεῑχος, χεῑλος, νεῑκος).
    declension!("νεῑκος", 3, "~ῑ~ος";
        nominative: "~ῑ~ος", "~ι~ει", "~ι~η";
        genitive: "~ι~ους", "~ι~οῑν", "~ι~ων";
        dative: "~ι~ει", "~ι~οῑν", "~ι~εσῐ|~ι~εσῐν";
        accusative: "~ῑ~ος", "~ι~ει", "~ι~η";
        vocative: "~ῑ~ος", "~ι~ει", "~ι~η";
    ),
    // Ἀσις: 3 words (ἀκρις, ἀκοιτις, ἀσις).
    declension!("ἀσις", 3, "ἀ~ις";
        nominative: "ᾰ\u{313}~ῐς", "ᾰ\u{313}~ῐε", "ᾰ\u{313}~ῐες";
        genitive: "ᾰ\u{313}~ῐος", "ᾰ\u{313}~ῐοιν", "ᾰ\u{313}~ῐων";
        dative: "ᾰ\u{313}~ῑ", "ᾰ\u{313}~ῐοιν", "ᾰ\u{313}~ῐσῐ|ᾰ\u{313}~ῐσῐν";
        accusative: "ᾰ\u{313}~ῐν", "ᾰ\u{313}~ῐε", "ᾰ\u{313}~ῑς|ᾰ\u{313}~ῐᾰς";
        vocative: "ᾰ\u{313}~ῐ", "ᾰ\u{313}~ῐε", "ᾰ\u{313}~ῐες";
    ),
    // Πειθω: 3 words (λεχω, πειθω, κερδω).
    declension!("πειθω", 3, "~ω";
        nominative: "~ω", "~ω", "~οι";
        genitive: "~οος|~οῡς", "~οῑν", "~ων";
        dative: "~οῑ", "~οῑν", "~οῑς";
        accusative: "~ω", "~ω", "~ους";
        vocative: "~οῑ", "~ω", "~οι";
    ),
    // Πεῑνᾰ: 3 words (πεῑσᾰ, χλαῑνᾰ, πεῑνᾰ).
    declension!("πεῑνᾰ", 3, "~ῑ~ᾰ";
        nominative: "~ῑ~ᾰ", "~ι~ᾱ", "~ῑ~αι";
        genitive: "~ι~ης", "~ι~αιν", "~ι~ων";
        dative: "~ι~ῃ", "~ι~αιν", "~ι~αις";
        accusative: "~ῑ~ᾰν", "~ι~ᾱ", "~ι~ᾱς";
        vocative: "~ῑ~ᾰ", "~ι~ᾱ", "~ῑ~αι";
    ),
    // Ἀβατον: 3 words (ἀνδραποδον, ἀσαρον, ἀβατον).
    declension!("ἀβατον", 3, "ἀ~α~ον";
        nominative: "ᾰ\u{313}~ᾰ~ον", "ᾰ\u{313}~ᾰ~ω", "ᾰ\u{313}~ᾰ~ᾰ";
        genitive: "ᾰ\u{313}~ᾰ~ου", "ᾰ\u{313}~ᾰ~οιν", "ᾰ\u{313}~ᾰ~ων";
        dative: "ᾰ\u{313}~ᾰ~ῳ", "ᾰ\u{313}~ᾰ~οιν", "ᾰ\u{313}~ᾰ~οις";
        accusative: "ᾰ\u{313}~ᾰ~ον", "ᾰ\u{313}~ᾰ~ω", "ᾰ\u{313}~ᾰ~ᾰ";
        vocative: "ᾰ\u{313}~ᾰ~ον", "ᾰ\u{313}~ᾰ~ω", "ᾰ\u{313}~ᾰ~ᾰ";
    ),
    // Ἱπποκαμπος: 3 words (ἱπποποταμος, ἱπποκαμπος, ἱμαντοδεσμος).
    declension!("ἱπποκαμπος", 3, "ἱ~α~ος";
        nominative: "ῐ\u{314}~ᾰ~ος", "ῐ\u{314}~ᾰ~ω", "ῐ\u{314}~ᾰ~οι";
        genitive: "ῐ\u{314}~ᾰ~ου", "ῐ\u{314}~ᾰ~οιν", "ῐ\u{314}~ᾰ~ων";
        dative: "ῐ\u{314}~ᾰ~ῳ", "ῐ\u{314}~ᾰ~οιν", "ῐ\u{314}~ᾰ~οις";
        accusative: "ῐ\u{314}~ᾰ~ον", "ῐ\u{314}~ᾰ~ω", "ῐ\u{314}~ᾰ~ους";
        vocative: "ῐ\u{314}~ᾰ~ε", "ῐ\u{314}~ᾰ~ω", "ῐ\u{314}~ᾰ~οι";
    ),
    // Ἀντιθεσις: 3 words (ἀντιθεσις, ἀντιπτωσις, ἀφιξις).
    declension!("ἀντιθεσις", 3, "ἀ~ι~ις";
        nominative: "ᾰ\u{313}~ῐ~ῐς", "ᾰ\u{313}~ῐ~ει", "ᾰ\u{313}~ῐ~εις";
        genitive: "ᾰ\u{313}~ῐ~εως", "ᾰ\u{313}~ῐ~εοιν", "ᾰ\u{313}~ῐ~εων";
        dative: "ᾰ\u{313}~ῐ~ει", "ᾰ\u{313}~ῐ~εοιν", "ᾰ\u{313}~ῐ~εσῐ|ᾰ\u{313}~ῐ~εσῐν";
        accusative: "ᾰ\u{313}~ῐ~ῐν", "ᾰ\u{313}~ῐ~ει", "ᾰ\u{313}~ῐ~εις";
        vocative: "ᾰ\u{313}~ῐ~ῐ", "ᾰ\u{313}~ῐ~ει", "ᾰ\u{313}~ῐ~εις";
    ),
    // Ἀριθμος: 3 words (ἀριθμος, ἀφορισμος, ἀκιδωτος).
    declension!("ἀριθμος", 3, "ἀ~ι~ος";
        nominative: "ᾰ\u{313}~ῐ~ος", "ᾰ\u{313}~ῐ~ω", "ᾰ\u{313}~ῐ~οι";
        genitive: "ᾰ\u{313}~ῐ~οῡ", "ᾰ\u{313}~ῐ~οῑν", "ᾰ\u{313}~ῐ~ων";
        dative: "ᾰ\u{313}~ῐ~ῳ", "ᾰ\u{313}~ῐ~οῑν", "ᾰ\u{313}~ῐ~οῑς";
        accusative: "ᾰ\u{313}~ῐ~ον", "ᾰ\u{313}~ῐ~ω", "ᾰ\u{313}~ῐ~ους";
        vocative: "ᾰ\u{313}~ῐ~ε", "ᾰ\u{313}~ῐ~ω", "ᾰ\u{313}~ῐ~οι";
    ),
    // Ἀγκυλη: 3 words (ἀφροσυνη, ἀργυροθηκη, ἀγκυλη).
    declension!("ἀγκυλη", 3, "ἀ~υ~η";
        nominative: "ᾰ\u{313}~ῠ~η", "ᾰ\u{313}~ῠ~ᾱ", "ᾰ\u{313}~ῠ~αι";
        genitive: "ᾰ\u{313}~ῠ~ης", "ᾰ\u{313}~ῠ~αιν", "ᾰ\u{313}~ῠ~ων";
        dative: "ᾰ\u{313}~ῠ~ῃ", "ᾰ\u{313}~ῠ~αιν", "ᾰ\u{313}~ῠ~αις";
        accusative: "ᾰ\u{313}~ῠ~ην", "ᾰ\u{313}~ῠ~ᾱ", "ᾰ\u{313}~ῠ~ᾱς";
        vocative: "ᾰ\u{313}~ῠ~η", "ᾰ\u{313}~ῠ~ᾱ", "ᾰ\u{313}~ῠ~αι";
    ),
    // Ἁμμα: 3 words (ἁρμα, ἁλμα, ἁμμα).
    declension!("ἁμμα", 3, "ἁ~α";
        nominative: "ᾰ\u{314}~ᾰ", "ᾰ\u{314}~ᾰτε", "ᾰ\u{314}~ᾰτᾰ";
        genitive: "ᾰ\u{314}~ᾰτος", "ᾰ\u{314}~ᾰτοιν", "ᾰ\u{314}~ᾰτων";
        dative: "ᾰ\u{314}~ᾰτῐ", "ᾰ\u{314}~ᾰτοιν", "ᾰ\u{314}~ᾰσῐ|ᾰ\u{314}~ᾰσῐν";
        accusative: "ᾰ\u{314}~ᾰ", "ᾰ\u{314}~ᾰτε", "ᾰ\u{314}~ᾰτᾰ";
        vocative: "ᾰ\u{314}~ᾰ", "ᾰ\u{314}~ᾰτε", "ᾰ\u{314}~ᾰτᾰ";
    ),
    // Δᾰσῠ: 3 words (ὀξῠ, δᾰσῠ, πλᾰτῠ).
    declension!("δᾰσῠ", 3, "~ῠ";
        nominative: "~ῠ", "~εῑ", "~εᾰ";
        genitive: "~εος", "~εοιν", "~εων";
        dative: "~εῑ", "~εοιν", "~εσῐ|~εσῐν";
        accusative: "~ῠ", "~εῑ", "~εᾰ";
        vocative: "~ῠ", "~εῑ", "~εᾰ";
    ),
    // Λαλλαι: 3 words (κᾰλανδαι, χαρωνῑται, λαλλαι).
    declension!("λαλλαι", 3, "~α~αι";
        nominative: "~~ᾱ", "~~ᾱ", "~ᾰ~αι";
        genitive: "~~ᾱς", "~~αιν", "~ᾰ~ων";
        dative: "~~ᾳ", "~~αιν", "~ᾰ~αις";
        accusative: "~~ᾱν", "~~ᾱ", "~ᾰ~ᾱς";
        vocative: "~~ᾱ", "~~ᾱ", "~ᾰ~αι";
    ),
    // Ἐντεᾰ: 3 words (ἐντεᾰ, δηνεᾰ, τειρεᾰ).
    declension!("ἐντεᾰ", 3, "~ᾰ";
        nominative: "~ᾰ", "~ᾱ", "~ᾰ";
        genitive: "~ης", "~αιν", "~ων";
        dative: "~ῃ", "~αιν", "~σῐ|~σῐν";
        accusative: "~ᾰν", "~ᾱ", "~ᾰ";
        vocative: "~ᾰ", "~ᾱ", "~ᾰ";
    ),
    // Παστα: 3 words (χεδροπα, παστα, ἀλμενιχιακα).
    declension!("παστα", 3, "~α";
        nominative: "~ον", "~ω", "~ᾰ";
        genitive: "~οῡ", "~οῑν", "~ων";
        dative: "~ῳ", "~οῑν", "~οῑς";
        accusative: "~ον", "~ω", "~ᾰ";
        vocative: "~ον", "~ω", "~ᾰ";
    ),
    // Γᾰμφηλαι: 3 words (γᾰμφηλαι, ταυρεασται, δοκηται).
    declension!("γᾰμφηλαι", 3, "~αι";
        nominative: "~ης", "~ᾱ", "~αι";
        genitive: "~ου", "~αιν", "~ων";
        dative: "~ῃ", "~αιν", "~αῑς";
        accusative: "~ην", "~ᾱ", "~ᾱς";
        vocative: "~ᾰ", "~ᾱ", "~αι";
    ),
    // Θεωσῐς: 3 words (θεωσῐς, ἀλθησῐς, ζαχαρῐς).
    declension!("θεωσῐς", 3, "~ῐς";
        nominative: "~ῐς", "~ει", "~εες";
        genitive: "~εως", "~εοιν", "~εων";
        dative: "~εῐ\u{308}", "~εοιν", "~εσῐ";
        accusative: "~ῐν", "~ει", "~εᾰς";
        vocative: "~ῐ", "~ει", "~εες";
    ),
    // Ὑποψιᾱ: 3 words (ὑποψιᾱ, ὑπεροψιᾱ, ὑδροφοβιᾱ).
    declension!("ὑποψιᾱ", 3, "ὑ~ιᾱ";
        nominative: "ῠ\u{314}~ῐᾱ", "ῠ\u{314}~ῐᾱ", "ῠ\u{314}~ῐαι";
        genitive: "ῠ\u{314}~ῐᾱς", "ῠ\u{314}~ῐαιν", "ῠ\u{314}~ῐων";
        dative: "ῠ\u{314}~ῐᾳ", "ῠ\u{314}~ῐαιν", "ῠ\u{314}~ῐαις";
        accusative: "ῠ\u{314}~ῐᾱν", "ῠ\u{314}~ῐᾱ", "ῠ\u{314}~ῐᾱς";
        vocative: "ῠ\u{314}~ῐᾱ", "ῠ\u{314}~ῐᾱ", "ῠ\u{314}~ῐαι";
    ),
    // Ζωᾱ: 3 words (γονᾱ, σφᾰγᾱ, ζωᾱ).
    declension!("ζωᾱ", 3, "~ᾱ";
        nominative: "~ᾱ", "~ᾱ", "~αι";
        genitive: "~ᾱς", "~αῑν", "~ᾱων|~ᾱν";
        dative: "~ᾳ", "~αῑν", "~αῑς|~αῑσῐ|~αῑσῐν";
        accusative: "~ᾱν", "~ᾱ", "~ᾱ\u{306}ς";
        vocative: "~ᾱ", "~ᾱ", "~αι";
    ),
    // Ἀσφαλισις: 3 words (ἀνακουφισις, ἀνταποκρισις, ἀσφαλισις).
    declension!("ἀσφαλισις", 3, "ἀ~α~ι~ις";
        nominative: "ᾰ\u{313}~ᾰ~ῐ~ῐς", "ᾰ\u{313}~ᾰ~ῐ~ει", "ᾰ\u{313}~ᾰ~ῐ~εις";
        genitive: "ᾰ\u{313}~ᾰ~ῐ~εως", "ᾰ\u{313}~ᾰ~ῐ~εοιν", "ᾰ\u{313}~ᾰ~ῐ~εων";
        dative: "ᾰ\u{313}~ᾰ~ῐ~ει", "ᾰ\u{313}~ᾰ~ῐ~εοιν", "ᾰ\u{313}~ᾰ~ῐ~εσῐ|ᾰ\u{313}~ᾰ~ῐ~εσῐν";
        accusative: "ᾰ\u{313}~ᾰ~ῐ~ῐν", "ᾰ\u{313}~ᾰ~ῐ~ει", "ᾰ\u{313}~ᾰ~ῐ~εις";
        vocative: "ᾰ\u{313}~ᾰ~ῐ~ῐ", "ᾰ\u{313}~ᾰ~ῐ~ει", "ᾰ\u{313}~ᾰ~ῐ~εις";
    ),
];
