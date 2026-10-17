//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes (`~ος`, `μαρῑνος`: the stem and `ος`). A stem in two pieces (`~ω~`, `μεντωρ`) has
//! letters between them that other forms may drop or change.
//! Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (if they share them),
/// the declension it is like (if any), the stem slots it states with a stem and an
/// ending table
/// (`stem lemma: \"~η- like μεθωνη\"`: the slot's cells are `μεθωνη`'s table's patterns
/// with `~η` for their `~`) (see `stems.rs`), then a row of patterns per line
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
    /// Like `μαρῑνος`: 1,226 words.
    Μαρῑνος,
    /// Like `μεθωνη`: 318 words.
    Μεθωνη,
    /// Like `ἡρᾱ`: 300 words.
    Ἡρᾱ,
    /// Like `ἀρτεμβᾰρης`: 213 words.
    Ἀρτεμβᾰρης,
    /// Like `μεδῐολᾱνον`: 172 words.
    Μεδῐολᾱνον,
    /// Like `λᾰδων`: 156 words.
    Λᾰδων,
    /// Like `μῐθρᾱς`: 110 words.
    Μῐθρᾱς,
    /// Like `βεροιᾰ`: 107 words.
    Βεροιᾰ,
    /// Like `σῑναι`: 97 words.
    Σῑναι,
    /// Like `ἀρῐοβαρζανης`: 91 words.
    Ἀρῐοβαρζανης,
    /// Like `ὀλβᾰσᾰ`: 89 words.
    Ὀλβᾰσᾰ,
    /// Like `σουσᾰ`: 69 words.
    Σουσᾰ,
    /// Like `μεγαβατης`: 59 words.
    Μεγαβατης,
    /// Like `κᾰλλῐπολῐς`: 56 words.
    Κᾰλλῐπολῐς,
    /// Like `ζᾰγρευς`: 53 words.
    Ζᾰγρευς,
    /// Like `δωρῐς`: 52 words.
    Δωρῐς,
    /// Like `πᾰρῠσᾱτῐς`: 49 words.
    Πᾰρῠσᾱτῐς,
    /// Like `ἐφραιμ`: 42 words.
    Ἐφραιμ,
    /// Like `ψαυμῐς`: 38 words.
    Ψαυμῐς,
    /// Like `ὑπερῑων`: 37 words.
    Ὑπερῑων,
    /// Like `πῡθω`: 35 words.
    Πῡθω,
    /// Like `περῐκλης`: 34 words.
    Περῐκλης,
    /// Like `γλισας`: 30 words.
    Γλισας,
    /// Like `ἑλλᾰς`: 28 words.
    Ἑλλᾰς,
    /// Like `μεντωρ`: 27 words.
    Μεντωρ,
    /// Like `ἰουδᾱς`: 27 words.
    Ἰουδᾱς,
    /// Like `σολοι`: 26 words.
    Σολοι,
    /// Like `λᾱοκοων`: 20 words.
    Λᾱοκοων,
    /// Like `φᾰεθων`: 19 words.
    Φᾰεθων,
    /// Like `μῐμᾱς`: 18 words.
    Μῐμᾱς,
    /// Like `ἰτῠς`: 15 words.
    Ἰτῠς,
    /// Like `βῠζηρες`: 14 words.
    Βῠζηρες,
    /// Like `ἐρως`: 13 words.
    Ἐρως,
    /// Like `ζᾰρᾰξ`: 13 words.
    Ζᾰρᾰξ,
    /// Like `ἀθως`: 13 words.
    Ἀθως,
    /// Like `κᾰλῠδων`: 12 words.
    Κᾰλῠδων,
    /// Like `οὐρᾰνιωνες`: 12 words.
    Οὐρᾰνιωνες,
    /// Like `δωρᾰ`: 12 words.
    Δωρᾰ,
    /// Like `καισᾰρ`: 6 words.
    Καισᾰρ,
    /// Like `κῠρβεις`: 5 words.
    Κῠρβεις,
    /// Like `γαδατᾱς`: 5 words.
    Γαδατᾱς,
    /// Like `τῑμωνᾰξ`: 5 words.
    Τῑμωνᾰξ,
    /// Like `χεοψ`: 5 words.
    Χεοψ,
    /// Like `σᾰλᾰμῑς`: 4 words.
    Σᾰλᾰμῑς,
    /// Like `ἀργος`: 4 words.
    Ἀργος,
    /// Like `βαϊαι`: 4 words.
    Βαϊαι,
    /// Like `σελληεις`: 4 words.
    Σελληεις,
    /// Like `στῠξ`: 4 words.
    Στῠξ,
    /// Like `περιπλους`: 4 words.
    Περιπλους,
    /// Like `ἀβιλα`: 4 words.
    Ἀβιλα,
    /// Like `ὑμην`: 4 words.
    Ὑμην,
    /// Like `ζοαρᾰ`: 4 words.
    Ζοαρᾰ,
    /// Like `ἁλῡς`: 3 words.
    Ἁλῡς,
    /// Like `ποσειδων`: 3 words.
    Ποσειδων,
    /// Like `κλημης`: 3 words.
    Κλημης,
    /// Like `δῐονῡσοκολᾰκες`: 3 words.
    Δῐονῡσοκολᾰκες,
    /// Like `ἁρμᾰ`: 3 words.
    Ἁρμᾰ,
    /// Like `γλᾰφυραι`: 2 words.
    Γλᾰφυραι,
    /// Like `γοριλλαι`: 2 words.
    Γοριλλαι,
    /// Like `στυρᾰ`: 2 words.
    Στυρᾰ,
    /// Like `ἀπᾰτουρῐα`: 1 words.
    Ἀπᾰτουρῐα,
    /// Like `ἀρταξᾰτᾰ`: 1 words.
    Ἀρταξᾰτᾰ,
    /// Like `πανελληνες`: 1 words.
    Πανελληνες,
    /// Like `τραλλεις`: 1 words.
    Τραλλεις,
    /// Like `πλειαδες`: 1 words.
    Πλειαδες,
    /// Like `θῑνα`: 1 words.
    Θῑνα,
    /// Like `πᾰλης`: 1 words.
    Πᾰλης,
    /// Like `κῐλλυριοι`: 1 words.
    Κῐλλυριοι,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 68] = [
        Declension::Μαρῑνος,
        Declension::Μεθωνη,
        Declension::Ἡρᾱ,
        Declension::Ἀρτεμβᾰρης,
        Declension::Μεδῐολᾱνον,
        Declension::Λᾰδων,
        Declension::Μῐθρᾱς,
        Declension::Βεροιᾰ,
        Declension::Σῑναι,
        Declension::Ἀρῐοβαρζανης,
        Declension::Ὀλβᾰσᾰ,
        Declension::Σουσᾰ,
        Declension::Μεγαβατης,
        Declension::Κᾰλλῐπολῐς,
        Declension::Ζᾰγρευς,
        Declension::Δωρῐς,
        Declension::Πᾰρῠσᾱτῐς,
        Declension::Ἐφραιμ,
        Declension::Ψαυμῐς,
        Declension::Ὑπερῑων,
        Declension::Πῡθω,
        Declension::Περῐκλης,
        Declension::Γλισας,
        Declension::Ἑλλᾰς,
        Declension::Μεντωρ,
        Declension::Ἰουδᾱς,
        Declension::Σολοι,
        Declension::Λᾱοκοων,
        Declension::Φᾰεθων,
        Declension::Μῐμᾱς,
        Declension::Ἰτῠς,
        Declension::Βῠζηρες,
        Declension::Ἐρως,
        Declension::Ζᾰρᾰξ,
        Declension::Ἀθως,
        Declension::Κᾰλῠδων,
        Declension::Οὐρᾰνιωνες,
        Declension::Δωρᾰ,
        Declension::Καισᾰρ,
        Declension::Κῠρβεις,
        Declension::Γαδατᾱς,
        Declension::Τῑμωνᾰξ,
        Declension::Χεοψ,
        Declension::Σᾰλᾰμῑς,
        Declension::Ἀργος,
        Declension::Βαϊαι,
        Declension::Σελληεις,
        Declension::Στῠξ,
        Declension::Περιπλους,
        Declension::Ἀβιλα,
        Declension::Ὑμην,
        Declension::Ζοαρᾰ,
        Declension::Ἁλῡς,
        Declension::Ποσειδων,
        Declension::Κλημης,
        Declension::Δῐονῡσοκολᾰκες,
        Declension::Ἁρμᾰ,
        Declension::Γλᾰφυραι,
        Declension::Γοριλλαι,
        Declension::Στυρᾰ,
        Declension::Ἀπᾰτουρῐα,
        Declension::Ἀρταξᾰτᾰ,
        Declension::Πανελληνες,
        Declension::Τραλλεις,
        Declension::Πλειαδες,
        Declension::Θῑνα,
        Declension::Πᾰλης,
        Declension::Κῐλλυριοι,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 68] = [
    // Μαρῑνος: 1,226 words (κῠπρος, λεσβος, χρῑστος, ἐφεσος, παυλος, πετρος).
    declension!("μαρῑνος", 1226, "~ος";
    nom: "~ος", "~ω", "~οι";
    gen_: "~ου", "~οιν", "~ων";
    dat: "~ῳ", _, "~οις";
    acc: "~ον", _, "~ους";
    voc: "~ε", _, _;
    ),
    // Μεθωνη: 318 words (πᾰρᾰσκευη, γη, νῑκη, κρητη, λῐβῠη, κῡρηνη).
    declension!("μεθωνη", 318, "~η", [gender: Feminine];
    stem lemma: "~η- like μεθωνη";
    nom: _, "~ᾱ", "~αι";
    gen_: "~ης", "~αιν", "~ων";
    dat: "~ῃ", _, "~αις";
    acc: "~ην", _, "~ᾱς";
    ),
    // Ἡρᾱ: 300 words (μᾰκεδονῐᾱ, μᾰρῐᾱ, γᾰλῑλαιᾱ, ἀσῐᾱ, ἰουδαιᾱ, καππαδοκῐᾱ). Like Μεθωνη, except in 3 cells: gen_ sg, dat sg, acc sg.
    declension!("ἡρᾱ", 300, "~ᾱ", [gender: Feminine], like Μεθωνη;
    stem lemma: "~ᾱ- like μεθωνη";
    gen_: "~ᾱς", _, _;
    dat: "~ᾳ", _, _;
    acc: "~ᾱν", _, _;
    ),
    // Ἀρτεμβᾰρης: 213 words (ἰωᾰννης, ᾁδης, ἡρῳδης, μωϋσης, ἰορδανης, ἑρμης). Like Μεθωνη with `` after each pattern (`ς` after η), except in 2 cells: gen_ sg, voc sg.
    declension!("ἀρτεμβᾰρης", 213, "~ης", [gender: Masculine], like Μεθωνη with "", "ς" after "η";
    gen_: "~ου", _, _;
    voc: "~η", _, _;
    ),
    // Μεδῐολᾱνον: 172 words (βῡζᾰντῐον, ἀκτιον, ἀτρᾰμυττῐον, αἰγαιον, ἀντῐον, ἀριμενον). Like Μαρῑνος, except in 2 cells: nom pl, acc pl.
    declension!("μεδῐολᾱνον", 172, "~ον", [gender: Neuter], like Μαρῑνος;
    stem lemma: "~ον- like μεθωνη";
    nom: _, _, "~ᾰ";
    acc: _, _, "~ᾰ";
    ),
    // Λᾰδων: 156 words (ἑλλην, βᾰβῠλων, σῐμων, σῠμεων, σαμψων, σῑδων).
    declension!("λᾰδων", 156, "~ν";
    stem lemma: "~ν- like μεθωνη";
    stem gen_sg: "~- like λᾰδων";
    dat: _, derived, _;
    acc: _, derived, _;
    voc: _, derived, derived;
    ),
    // Μῐθρᾱς: 110 words (ζαχαριᾱς, ἀνδρεᾱς, ἱερεμιᾱς, ἠσᾰϊᾱς, ἀδρῐᾱς, ἡλιᾱς). Like Ἡρᾱ with `` after each pattern (`ς` after ᾱ), except in 3 cells: nom du, gen_ sg, voc sg.
    declension!("μῐθρᾱς", 110, "~ᾱς", [gender: Masculine], like Ἡρᾱ with "", "ς" after "ᾱ";
    nom: _, "~ᾱ", _;
    gen_: "~ου", _, _;
    voc: "~ᾱ", _, _;
    ),
    // Βεροιᾰ: 107 words (φῐλᾰδελφειᾰ, σᾰμᾰρειᾰ, λᾱοδῐκειᾰ, σελευκειᾰ, ἀντῐοχειᾰ, μοιρᾰ). Like Ἡρᾱ, except in 1 cell: acc sg.
    declension!("βεροιᾰ", 107, "~ᾰ", [gender: Feminine], like Ἡρᾱ;
    stem lemma: "~ᾰ- like μεθωνη";
    acc: "~ᾰν", _, _;
    ),
    // Σῑναι: 97 words (κολοσσαι, ἀθηναι, μῠκηναι, θηβαι, ἀχαρναι, ἀμῠκλαι). Like Μεθωνη, except in 7 cells: nom sg, nom du, gen_ sg, gen_ du, dat sg, acc sg, ….
    declension!("σῑναι", 97, "~αι", like Μεθωνη;
    nom: "~αι", derived, _;
    gen_: derived, derived, _;
    dat: derived, _, _;
    acc: derived, _, _;
    voc: derived, _, _;
    ),
    // Ἀρῐοβαρζανης: 91 words (ἀχαιμενης, ἀλκᾰμενης, ἀντῐσθενης, ἀρχῐμηδης, ἀρης, ἀρῐοβαρζανης).
    declension!("ἀρῐοβαρζανης", 91, "~ης", [gender: Masculine];
    nom: "~ης", "~ει", "~εις";
    gen_: "~ους", "~οιν", "~ων";
    dat: "~ει", _, "~εσῐ";
    acc: "~η", _, "~εις";
    voc: "~ες", _, _;
    ),
    // Ὀλβᾰσᾰ: 89 words (γεεννᾰ, γᾰζᾰ, μουσᾰ, προυσᾰ, αἰγῑνᾰ, ἀμφισσᾰ). Like Βεροιᾰ, except in 2 cells: gen_ sg, dat sg.
    declension!("ὀλβᾰσᾰ", 89, "~ᾰ", [gender: Feminine], like Βεροιᾰ;
    gen_: "~ης", _, _;
    dat: "~ῃ", _, _;
    ),
    // Σουσᾰ: 69 words (σοδομᾰ, ἀβδηρᾰ, ἀναιᾰ, καρνειᾰ, κῠθηρᾰ, ἀντῐκῠθηρᾰ). Like Ὀλβᾰσᾰ, except in 5 cells: nom du, nom pl, gen_ du, dat pl, acc pl.
    declension!("σουσᾰ", 69, "~ᾰ", [gender: Neuter], like Ὀλβᾰσᾰ;
    nom: _, derived, "~ᾰ";
    gen_: _, derived, _;
    dat: _, _, "~οις";
    acc: _, _, "~ᾰ";
    ),
    // Μεγαβατης: 59 words (εὐφρᾱτης, αἰητης, ἀλυαττης, βελλεροφοντης, ἐφῐαλτης, ἰθωμητης). Like Μεθωνη with `` after each pattern (`ς` after η), except in 2 cells: gen_ sg, voc sg.
    declension!("μεγαβατης", 59, "~ης", [gender: Masculine], like Μεθωνη with "", "ς" after "η";
    gen_: "~ου", _, _;
    voc: "~ᾰ", _, _;
    ),
    // Κᾰλλῐπολῐς: 56 words (ἱερᾱπολῐς, δεκᾰπολῐς, νῑκοπολῐς, σῠβᾰρῐς, ἁδριανουπολῐς, τραϊᾱνουπολῐς). Like Ἀρῐοβαρζανης, except in 4 cells: gen_ sg, gen_ du, gen_ pl, acc sg.
    declension!("κᾰλλῐπολῐς", 56, "~ῐς", like Ἀρῐοβαρζανης;
    stem lemma: "~ῐ- like ἀρτεμβᾰρης";
    gen_: "~εως", "~εοιν", "~εων";
    acc: "~ῐν", _, _;
    ),
    // Ζᾰγρευς: 53 words (ζευς, ἀχῐλλευς, αἰγευς, αἰγῐᾰλευς, ἀρῐστευς, ἀταρνευς). Like Κᾰλλῐπολῐς, except in 5 cells: nom du, nom pl, dat pl, acc sg, acc pl.
    declension!("ζᾰγρευς", 53, "~ευς", [gender: Masculine], like Κᾰλλῐπολῐς;
    stem lemma: "~ευ- like ἀρτεμβᾰρης";
    nom: _, "~η", "~ης";
    dat: _, _, "~ευσῐ";
    acc: "~εᾱ", _, "~εᾱς";
    ),
    // Δωρῐς: 52 words (λευϊς, ἀρτεμῐς, κολχῐς, χαλκῐς, αἰολῐς, ἀργολῐς).
    declension!("δωρῐς", 52, "~ς";
    stem lemma: "~- like ἀρτεμβᾰρης";
    nom: _, "~δε", "~δες";
    gen_: "~δος", "~δοιν", "~δων";
    dat: "~δῐ", _, "~σῐ";
    acc: "~δᾰ", _, "~δᾰς";
    ),
    // Πᾰρῠσᾱτῐς: 49 words (ἀδωνῐς, ἀλκηστῐς, μαιωτῐς, βᾰκῐς, χρῡσοθεμῐς, εὐπολῐς).
    declension!("πᾰρῠσᾱτῐς", 49, "~ῐς";
    stem lemma: "~ῐ- like ἀρτεμβᾰρης";
    nom: _, "~ῐδε", "~ῐδες";
    gen_: "~ῐδος", "~ῐδοιν", "~ῐδων";
    dat: "~ῐδῐ", _, "~ῐσῐ";
    acc: "~ῐν", _, "~ῐδᾰς";
    ),
    // Ἐφραιμ: 42 words (νᾱζᾰρεθ, βηθλεεμ, χανααν, βενιᾱμῑν, ἐφραιμ, φοινῐξ).
    declension!("ἐφραιμ", 42, "~";
    stem lemma: "~- like μεθωνη";
    gen_: "~", _, _;
    dat: "~", _, _;
    acc: "~", _, _;
    ),
    // Ψαυμῐς: 38 words (βαιτῐς, κρευσῐς, ναυκρᾰτῐς, νισιβῐς, φᾱσῐς, ῥοδωπῐς). Like Ἐφραιμ, except in 3 cells: gen_ sg, dat sg, acc sg.
    declension!("ψαυμῐς", 38, "~ῐς", like Ἐφραιμ;
    stem lemma: "~ῐ- like ἀρτεμβᾰρης";
    gen_: "~ῐος", _, _;
    dat: "~ῑ", _, _;
    acc: "~ῐν", _, _;
    ),
    // Ὑπερῑων: 37 words (ἀπολλῠων, ἀγαμεμνων, προκῠων, ἀμφῑων, ἀνδραιμων, ἀρῑων).
    declension!("ὑπερῑων", 37, "~ων", [gender: Masculine];
    stem gen_sg: "~ο- like λᾰδων";
    nom: "~ων", _, _;
    dat: _, derived, _;
    acc: _, derived, _;
    voc: "~ον", derived, derived;
    ),
    // Πῡθω: 35 words (ἱερῑχω, ἀργω, κᾰλῠψω, κλειω, κλωθω, ἠχω). Like Μαρῑνος, except in 5 cells: nom sg, gen_ sg, dat sg, acc sg, voc sg.
    declension!("πῡθω", 35, "~ω", [gender: Feminine], like Μαρῑνος;
    nom: "~ω", _, _;
    gen_: "~ους", _, _;
    dat: "~οι", _, _;
    acc: "~ω", _, _;
    voc: "~οι", _, _;
    ),
    // Περῐκλης: 34 words (ἀμεινοκλης, ἀνδροκλης, χᾰρῐκλης, ἐμπεδοκλης, ἐτεοκλης, εὐκλης). Like Ἐφραιμ, except in 5 cells: nom sg, gen_ sg, dat sg, acc sg, voc sg.
    declension!("περῐκλης", 34, "~ης", [gender: Masculine], like Ἐφραιμ;
    nom: "~ης", _, _;
    gen_: "~εους", _, _;
    dat: "~ει", _, _;
    acc: "~εᾱ", _, _;
    voc: "~εις", _, _;
    ),
    // Γλισας: 30 words (ἐμμαους, τρᾰπεζους, κερᾰσους, ἀμᾰθους, ἀνθεμους, ἐλαιους).
    declension!("γλισας", 30, "~ς";
    nom: "~ς", "~ντε", "~ντες";
    gen_: "~ντος", "~ντοιν", "~ντων";
    dat: "~ντῐ", _, "~σῐ";
    acc: "~ντᾰ", _, "~ντᾰς";
    voc: "~ν", _, _;
    ),
    // Ἑλλᾰς: 28 words (ἡρῳδῐᾰς, ἑλλᾰς, κῠκλᾰς, τρωᾰς, πᾰλλᾰς, ὑᾰς). Like Δωρῐς, except in 1 cell: voc sg.
    declension!("ἑλλᾰς", 28, "~ς", [gender: Feminine], like Δωρῐς;
    voc: "~ς", _, _;
    ),
    // Μεντωρ: 27 words (ἀκτωρ, ἀγηνωρ, ἀμῡντωρ, ἀντηνωρ, κᾰστωρ, κλεᾱνωρ).
    declension!("μεντωρ", 27, "~ω~", [gender: Masculine];
    stem gen_sg: "~ο~- like μεντωρ";
    nom: "~ω~", _, _;
    dat: _, derived, _;
    acc: _, derived, _;
    voc: "~ο~", derived, derived;
    ),
    // Ἰουδᾱς: 27 words (κλεοπᾱς, θωμᾱς, ἰωνᾱς, σᾰτᾰνᾱς, βαραββᾱς, ἀγρῐππᾱς). Like Ἡρᾱ with `` after each pattern (`ς` after ᾱ), except in 3 cells: nom du, gen_ sg, voc sg.
    declension!("ἰουδᾱς", 27, "~ᾱς", [gender: Masculine], like Ἡρᾱ with "", "ς" after "ᾱ";
    nom: _, "~ᾱ", _;
    gen_: "~ᾱ", _, _;
    voc: "~ᾱ", _, _;
    ),
    // Σολοι: 26 words (φῐλῐπποι, ἀρποι, δελφοι, λεοντῑνοι, πομπηϊοι, σολοι). Like Σῑναι, except in 4 cells: nom sg, nom pl, dat pl, acc pl.
    declension!("σολοι", 26, "~οι", [gender: Masculine], like Σῑναι;
    nom: "~οι", _, "~οι";
    dat: _, _, "~οις";
    acc: _, _, "~ους";
    ),
    // Λᾱοκοων: 20 words (σολομων, ἀντῐφων, χαιρεφων, κλειτοφων, κλεοφων, κτησῐφων). Like Γλισας, except in 1 cell: nom sg.
    declension!("λᾱοκοων", 20, "~ν", [gender: Masculine], like Γλισας;
    nom: "~ν", _, _;
    ),
    // Φᾰεθων: 19 words (ἀχερων, ἀνᾰκρεων, κρεων, δῐομεδων, δρᾰκων, εὐρῠμεδων).
    declension!("φᾰεθων", 19, "~ω~", [gender: Masculine];
    stem gen_sg: "~ο~τ- like φᾰεθων";
    nom: "~ω~", _, _;
    voc: "~ο~", _, _;
    ),
    // Μῐμᾱς: 18 words (τᾰρᾱς, βῡζᾱς, ἀκᾰμᾱς, ἀκρᾰγᾱς, αἰᾱς, ἀθᾰμᾱς).
    declension!("μῐμᾱς", 18, "~ᾱς", [gender: Masculine];
    nom: "~ᾱς", "~ᾰντε", "~ᾰντες";
    gen_: "~ᾰντος", "~ᾰντοιν", "~ᾰντων";
    dat: "~ᾰντῐ", _, "~ᾱσῐ";
    acc: "~ᾰντᾰ", _, "~ᾰντᾰς";
    voc: "~ᾰν", _, _;
    ),
    // Ἰτῠς: 15 words (κοτῠς, τηθῠς, ἰτῠς, ὀθρῠς, ῥᾰδᾰμανθῠς, νεφθῠς). Like Ἐφραιμ, except in 3 cells: gen_ sg, dat sg, acc sg.
    declension!("ἰτῠς", 15, "~ῠς", like Ἐφραιμ;
    stem lemma: "~ῠ- like ἀρτεμβᾰρης";
    gen_: "~ῠος", _, _;
    dat: "~ῠϊ|~υι", _, _;
    acc: "~ῠν", _, _;
    ),
    // Βῠζηρες: 14 words (στοιχᾰδες, βῠζηρες, σᾰπειρες, γᾰρᾰμᾰντες, ἰᾱζῠγες, ἀλμωπες). Like Σῑναι, except in 4 cells: nom sg, nom pl, dat pl, acc pl.
    declension!("βῠζηρες", 14, "~ες", like Σῑναι;
    nom: "~ες", _, "~ες";
    dat: _, _, "~σῐ|~σῐν";
    acc: _, _, "~ᾰς";
    ),
    // Ἐρως: 13 words (χᾰρης, ἐρως, νεπως, παχης, φερης, πιγρης). Like Ἁρμᾰ with `` after each pattern (`ς` after ~), except in 1 cell: acc sg.
    declension!("ἐρως", 13, "~ς", like Ἁρμᾰ with "", "ς" after "~";
    acc: "~τᾰ", _, _;
    ),
    // Ζᾰρᾰξ: 13 words (θρᾳξ, ἐρῠξ, οἰᾱξ, φαιᾱξ, οὐινδιξ, περτῐνᾱξ).
    declension!("ζᾰρᾰξ", 13, "~ξ", [gender: Masculine];
    stem lemma: "~ξ- like μεθωνη";
    nom: _, "~κε", "~κες";
    gen_: "~κος", "~κοιν", "~κων";
    dat: "~κῐ", _, "~ξῐ";
    acc: "~κᾰ", _, "~κᾰς";
    ),
    // Ἀθως: 13 words (κως, ἀθως, ἑως, βρῐᾰρεως, κεως, ἰναρως). Like Ἁρμᾰ with `ως` after each pattern, except in 3 cells: gen_ sg, dat sg, acc sg.
    declension!("ἀθως", 13, "~ως", like Ἁρμᾰ with "ως";
    gen_: "~ω", _, _;
    dat: "~ῳ", _, _;
    acc: "~ω|~ων", _, _;
    ),
    // Κᾰλῠδων: 12 words (κᾰλῠδων, καρχηδων, χαλκηδων, ἠϊων, χθων, γηρῠων). Like Ὑπερῑων, except in 1 cell: voc sg.
    declension!("κᾰλῠδων", 12, "~ων", like Ὑπερῑων;
    voc: "~ων", _, _;
    ),
    // Οὐρᾰνιωνες: 12 words (σῑδονες, οὐρᾰνιωνες, νᾰσᾰμωνες, χᾱονες, καυκωνες, ὀρεᾱνες). Like Βῠζηρες, except in 2 cells: nom sg, voc pl.
    declension!("οὐρᾰνιωνες", 12, "~νες", [gender: Masculine], like Βῠζηρες;
    stem gen_sg: "~- like οὐρᾰνιωνες";
    nom: "~νες", _, _;
    voc: _, _, derived;
    ),
    // Δωρᾰ: 12 words (γομορρᾰ, γᾱδειρᾰ, λευκτρᾰ, σταγειρᾰ, λῠστρᾱ, βοστρᾰ). Like Σουσᾰ, except in 2 cells: gen_ sg, dat sg.
    declension!("δωρᾰ", 12, "~ᾰ", like Σουσᾰ;
    gen_: "~ᾱς", _, _;
    dat: "~ᾳ", _, _;
    ),
    // Καισᾰρ: 6 words (ἀραρ, καισᾰρ, κηρ, λειγηρ, ἰβηρ, βαλδῐμερ).
    declension!("καισᾰρ", 6, "~";
    stem lemma: "~- like μεθωνη";
    stem gen_sg: "~- like μεντωρ";
    dat: _, derived, _;
    acc: _, derived, _;
    voc: _, derived, derived;
    ),
    // Κῠρβεις: 5 words (ἀλπεις, σᾰρδεις, στοιχειωσεις, κῠρβεις, πᾰλεις). Like Βῠζηρες, except in 3 cells: nom sg, nom pl, acc pl.
    declension!("κῠρβεις", 5, "~ις", like Βῠζηρες;
    nom: "~ις", _, "~ις";
    acc: _, _, "~ις";
    ),
    // Γαδατᾱς: 5 words (ἀμῡντᾱς, γαδατᾱς, ζᾰπᾰτᾱς, εὐφρατᾱς, νῑκητᾱς). Like Ἐφραιμ, except in 5 cells: nom sg, gen_ sg, dat sg, acc sg, voc sg.
    declension!("γαδατᾱς", 5, "~ᾱς", [gender: Masculine], like Ἐφραιμ;
    nom: "~ᾱς", _, _;
    gen_: "~ου", _, _;
    dat: "~ᾳ", _, _;
    acc: "~ᾱν", _, _;
    voc: "~ᾰ", _, _;
    ),
    // Τῑμωνᾰξ: 5 words (ἀστῠᾰνᾰξ, νῠξ, πλειστοᾰνᾰξ, τῑμωνᾰξ, ἱππωνᾰξ). Like Ἐφραιμ.
    declension!("τῑμωνᾰξ", 5, "~ξ", like Ἐφραιμ;
    stem lemma: "~ξ- like μεθωνη";
    stem gen_sg: "~κτ- like φᾰεθων";
    ),
    // Χεοψ: 5 words (κεκροψ, μεροψ, πελοψ, χεοψ, κῑνῠψ). Like Ἐφραιμ.
    declension!("χεοψ", 5, "~ψ", [gender: Masculine], like Ἐφραιμ;
    stem lemma: "~ψ- like μεθωνη";
    stem gen_sg: "~π- like φᾰεθων";
    ),
    // Σᾰλᾰμῑς: 4 words (σᾰλᾰμῑς, ἐλευσῑς, γορτῡς, τρᾱχῑς). Like Ἐρως.
    declension!("σᾰλᾰμῑς", 4, "~ς", [gender: Feminine], like Ἐρως;
    stem gen_sg: "~ν- like φᾰεθων";
    ),
    // Ἀργος: 4 words (ἀργος, ἐρεβος, ἑλος, κρᾰτος). Like Ἁρμᾰ with `ος` after each pattern, except in 2 cells: gen_ sg, dat sg.
    declension!("ἀργος", 4, "~ος", like Ἁρμᾰ with "ος";
    gen_: "~ους", _, _;
    dat: "~ει", _, _;
    ),
    // Βαϊαι: 4 words (βαϊαι, καρραι, κλαζομεναι, γαγαι). Like Ἀβιλα with `~αι` after each pattern (`~ᾱς` after ᾰ), except in 3 cells: nom pl, gen_ pl, dat pl.
    declension!("βαϊαι", 4, "~α~αι", [gender: Feminine], like Ἀβιλα with "~αι", "~ᾱς" after "ᾰ";
    nom: _, _, "~ᾰ~αι";
    gen_: _, _, "~ᾰ~ων";
    dat: _, _, "~ᾰ~αις";
    ),
    // Σελληεις: 4 words (σῐμοεις, σολοεις, σᾰτνῐοεις, σελληεις). Like Ἐφραιμ, except in 2 cells: nom sg, voc sg.
    declension!("σελληεις", 4, "~ις", [gender: Masculine], like Ἐφραιμ;
    stem gen_sg: "~ντ- like φᾰεθων";
    nom: "~ις", _, _;
    voc: "~ν", _, _;
    ),
    // Στῠξ: 4 words (ῥηξ, σφῐγξ, στῠξ, δᾰπῠξ).
    declension!("στῠξ", 4, "~ξ";
    stem lemma: "~ξ- like μεθωνη";
    nom: _, "~γε", "~γες";
    gen_: "~γος", "~γοιν", "~γων";
    dat: "~γῐ", _, "~ξῐ";
    acc: "~γᾰ", _, "~γᾰς";
    ),
    // Περιπλους: 4 words (πειρῐθους, περιπλους, πᾰνθους, ἀντῐνους). Like Μαρῑνος, except in 1 cell: acc sg.
    declension!("περιπλους", 4, "~ους", [gender: Masculine], like Μαρῑνος;
    stem lemma: "~ου- like ἀρτεμβᾰρης";
    acc: "~ουν", _, _;
    ),
    // Ἀβιλα: 4 words (σαρεπτα, ἀβιλα, θυατειρα, αἰλανα). Like Σολοι, except in 2 cells: nom sg, voc pl.
    declension!("ἀβιλα", 4, "~α", [gender: Neuter], like Σολοι;
    stem gen_sg: "~- like ἀβιλα";
    nom: "~α", _, _;
    voc: _, _, derived;
    ),
    // Ὑμην: 4 words (ὑμην, ὠλην, φῐλοποιμην, πᾰτηρ). Like Φᾰεθων.
    declension!("ὑμην", 4, "~η~", [gender: Masculine], like Φᾰεθων;
    stem lemma: "~η~- like μεθωνη";
    stem gen_sg: "~ε~- like φᾰεθων";
    ),
    // Ζοαρᾰ: 4 words (κουναξα, βακτρᾰ, βῠλαζωρᾰ, ζοαρᾰ). Like Ἀβιλα with `~ᾰ` after each pattern, except in 2 cells: gen_ pl, dat pl.
    declension!("ζοαρᾰ", 4, "~α~ᾰ", [gender: Neuter], like Ἀβιλα with "~ᾰ";
    gen_: _, _, "~ᾰ~ων";
    dat: _, _, "~ᾰ~οις";
    ),
    // Ἁλῡς: 3 words (ἐρῑνῡς, ἁλῡς, φορκῡς).
    declension!("ἁλῡς", 3, "~ῡς";
    stem lemma: "~ῡ- like ἀρτεμβᾰρης";
    nom: _, "~ῠε", "~ῠες";
    gen_: "~ῠος", "~ῠοιν", "~ῠων";
    dat: "~ῠϊ|~υι", _, "~ῠσῐ";
    acc: "~ῡν", _, "~ῡς";
    ),
    // Ποσειδων: 3 words (ἀπολλων, ποσειδων, ποσειδᾱων). Like Ἐφραιμ, except in 2 cells: nom sg, voc sg.
    declension!("ποσειδων", 3, "~ων", [gender: Masculine], like Ἐφραιμ;
    stem gen_sg: "~ων- like φᾰεθων";
    nom: "~ων", _, _;
    voc: "~ον", _, _;
    ),
    // Κλημης: 3 words (οὐαλης, πουδης, κλημης). Like Περῐκλης, except in 1 cell: voc sg.
    declension!("κλημης", 3, "~ης", [gender: Masculine], like Περῐκλης;
    stem gen_sg: "~εντ- like φᾰεθων";
    voc: "~εν", _, _;
    ),
    // Δῐονῡσοκολᾰκες: 3 words (δῐονῡσοκολᾰκες, πηληκες, ζαυηκες). Like Σῑναι, except in 5 cells: nom sg, nom pl, gen_ pl, dat pl, acc pl.
    declension!("δῐονῡσοκολᾰκες", 3, "~κες", [gender: Masculine], like Σῑναι;
    nom: "~κες", _, "~κες";
    gen_: _, _, "~κων";
    dat: _, _, "~ξῐ|~ξῐν";
    acc: _, _, "~κᾰς";
    ),
    // Ἁρμᾰ: 3 words (ἁρμᾰ, ζευγμᾰ, κλῠσμᾰ). Like Ἐφραιμ, except in 2 cells: gen_ sg, dat sg.
    declension!("ἁρμᾰ", 3, "~", [gender: Neuter], like Ἐφραιμ;
    gen_: "~τος", _, _;
    dat: "~τῐ", _, _;
    ),
    // Γλᾰφυραι: 2 words (συρᾱκουσαι, γλᾰφυραι). Like Βαϊαι, except in 2 cells: nom sg, voc pl.
    declension!("γλᾰφυραι", 2, "~υ~αι", like Βαϊαι;
    stem gen_sg: "~ῠ~- like σῑναι";
    nom: "~υ~αι", _, _;
    voc: _, _, derived;
    ),
    // Γοριλλαι: 2 words (γοριλλαι, πιθηκουσαι). Like Βαϊαι, except in 2 cells: nom sg, voc pl.
    declension!("γοριλλαι", 2, "~ι~αι", like Βαϊαι;
    stem gen_sg: "~ῐ~- like σῑναι";
    nom: "~ι~αι", _, _;
    voc: _, _, derived;
    ),
    // Στυρᾰ: 2 words (στυρᾰ, κᾰρυανδᾰ). Like Γλᾰφυραι, except in 2 cells: nom sg, voc pl.
    declension!("στυρᾰ", 2, "~υ~ᾰ", like Γλᾰφυραι;
    stem gen_sg: "~ῠ~- like ἀβιλα";
    nom: "~υ~ᾰ", _, _;
    voc: _, _, derived;
    ),
    // Ἀπᾰτουρῐα: 1 words (ἀπᾰτουρῐα). Like Βαϊαι, except in 2 cells: nom sg, voc pl.
    declension!("ἀπᾰτουρῐα", 1, "~ᾰ~ῐα", like Βαϊαι;
    stem gen_sg: "~α~ι- like ἀβιλα";
    nom: "~ᾰ~ῐα", _, _;
    voc: _, _, derived;
    ),
    // Ἀρταξᾰτᾰ: 1 words (ἀρταξᾰτᾰ). Like Βαϊαι, except in 2 cells: nom sg, voc pl.
    declension!("ἀρταξᾰτᾰ", 1, "~ᾰ~ᾰ", like Βαϊαι;
    stem gen_sg: "~α~- like ἀβιλα";
    nom: "~ᾰ~ᾰ", _, _;
    voc: _, _, derived;
    ),
    // Πανελληνες: 1 words (πανελληνες). Like Ἀβιλα with `~νες` after each pattern (`~νᾰς` after ᾰ), except in 3 cells: nom pl, gen_ pl, dat pl.
    declension!("πανελληνες", 1, "~α~νες", like Ἀβιλα with "~νες", "~νᾰς" after "ᾰ";
    nom: _, _, "~ᾰ~νες";
    gen_: _, _, "~ᾰ~νων";
    dat: _, _, "~ᾰ~σῐ|~ᾰ~σῐν";
    ),
    // Τραλλεις: 1 words (τραλλεις). Like Ἀβιλα with `~ις` after each pattern, except in 2 cells: gen_ pl, dat pl.
    declension!("τραλλεις", 1, "~α~ις", like Ἀβιλα with "~ις";
    gen_: _, _, "~ᾰ~ων";
    dat: _, _, "~ᾰ~σῐ|~ᾰ~σῐν";
    ),
    // Πλειαδες: 1 words (πλειαδες). Like Ἀβιλα with `δες` after each pattern (`δᾰς` after ᾰ), except in 3 cells: nom pl, gen_ pl, dat pl.
    declension!("πλειαδες", 1, "~αδες", like Ἀβιλα with "δες", "δᾰς" after "ᾰ";
    nom: _, _, "~ᾰδες";
    gen_: _, _, "~ᾰδων";
    dat: _, _, "~ᾰσῐ|~ᾰσῐν";
    ),
    // Θῑνα: 1 words (θῑνα). Like Ἐφραιμ, except in 2 cells: nom sg, voc sg.
    declension!("θῑνα", 1, "~α", like Ἐφραιμ;
    stem gen_sg: "~- like φᾰεθων";
    nom: "~α", _, _;
    voc: derived, _, _;
    ),
    // Πᾰλης: 1 words (πᾰλης). Like Σῑναι, except in 5 cells: nom sg, nom pl, gen_ pl, dat pl, acc pl.
    declension!("πᾰλης", 1, "~ης", like Σῑναι;
    nom: "~ης", _, "~ης";
    gen_: _, _, "~εων";
    dat: _, _, "~ευσῐ|~ευσῐν";
    acc: _, _, "~εᾱς";
    ),
    // Κῐλλυριοι: 1 words (κῐλλυριοι). Like Σῑναι, except in 5 cells: nom sg, nom pl, gen_ pl, dat pl, acc pl.
    declension!("κῐλλυριοι", 1, "~ιοι", like Σῑναι;
    nom: "~ιοι", _, "~ῐοι";
    gen_: _, _, "~ῐων";
    dat: _, _, "~ῐοις";
    acc: _, _, "~ῐους";
    ),
];
