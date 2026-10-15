//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes (`~ος`, `μᾰλᾰκος`: the stem and `ος`). A stem in two pieces (`~ω~`, `ἀπᾰτωρ`) has
//! letters between them that other forms may drop or change.
//! Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (if they share them),
/// the declension it is like (if any), the stem slots it states with a stem and an
/// ending table
/// (`stem lemma: \"~- like μᾰλᾰκος\"`: the slot's cells are `μᾰλᾰκος`'s table's patterns
/// with `~` for their `~`) (see `stems.rs`), then a row of patterns per line
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
/// `comparative`
/// `superlative`
/// `adverb`
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident $(as $view:ident)?)?;
     $(stem lemma: $s0:tt;)?
     $(stem rest: $s1:tt;)?
     $(stem rest_: $s2:tt;)?
     $(stem adjective_dat_pl: $s3:tt;)?
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
     $(comparative: $c45:tt;)?
     $(superlative: $c46:tt;)?
     $(adverb: $c47:tt;)?
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
                $(flexion::cell!($c45),)? "\t",
                $(flexion::cell!($c46),)? "\t",
                $(flexion::cell!($c47),)? "\t",
            )),
            stems: ::std::borrow::Cow::Borrowed(concat!(
                $(flexion::cell!($s0),)? "\t",
                $(flexion::cell!($s1),)? "\t",
                $(flexion::cell!($s2),)? "\t",
                $(flexion::cell!($s3),)? "\t",
            )),
            affix: None,
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

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `μᾰλᾰκος`: 1,454 words.
    Μᾰλᾰκος,
    /// Like `ἐμπεδος`: 931 words.
    Ἐμπεδος,
    /// Like `λᾰμπρος`: 464 words.
    Λᾰμπρος,
    /// Like `προσηκων`: 327 words.
    Προσηκων,
    /// Like `ἀτελης`: 286 words.
    Ἀτελης,
    /// Like `ψῑλος`: 168 words.
    Ψῑλος,
    /// Like `προπᾱς`: 168 words.
    Προπᾱς,
    /// Like `ἀσημος`: 152 words.
    Ἀσημος,
    /// Like `ὑστερος`: 127 words.
    Ὑστερος,
    /// Like `ἀρῐστος`: 112 words.
    Ἀρῐστος,
    /// Like `πληγεις`: 90 words.
    Πληγεις,
    /// Like `ἀρκων`: 58 words.
    Ἀρκων,
    /// Like `εἰκως`: 48 words.
    Εἰκως,
    /// Like `μνημων`: 39 words.
    Μνημων,
    /// Like `φωνηεις`: 30 words.
    Φωνηεις,
    /// Like `λᾰμπρος`: 26 words.
    Λᾰμπρος2,
    /// Like `ζων`: 26 words.
    Ζων,
    /// Like `πλειων`: 25 words.
    Πλειων,
    /// Like `τρᾱχῠς`: 25 words.
    Τρᾱχῠς,
    /// Like `τρῐπους`: 8 words.
    Τρῐπους,
    /// Like `ὁμηλῐξ`: 8 words.
    Ὁμηλῐξ,
    /// Like `ἀξῐοχρεως`: 7 words.
    Ἀξῐοχρεως,
    /// Like `εὐχᾰρῐς`: 7 words.
    Εὐχᾰρῐς,
    /// Like `ἀπᾰτωρ`: 6 words.
    Ἀπᾰτωρ,
    /// Like `φῡς`: 6 words.
    Φῡς,
    /// Like `δῐπλους`: 6 words.
    Δῐπλους,
    /// Like `εὐελπῐς`: 6 words.
    Εὐελπῐς,
    /// Like `αἰθοψ`: 5 words.
    Αἰθοψ,
    /// Like `νομᾰς`: 5 words.
    Νομᾰς,
    /// Like `βοτηρ`: 5 words.
    Βοτηρ,
    /// Like `πλειστος`: 4 words.
    Πλειστος,
    /// Like `μελᾱς`: 4 words.
    Μελᾱς,
    /// Like `βαιος`: 4 words.
    Βαιος,
    /// Like `θηλῠς`: 4 words.
    Θηλῠς,
    /// Like `μωλῠς`: 4 words.
    Μωλῠς,
    /// Like `δους`: 4 words.
    Δους,
    /// Like `ἀπους`: 4 words.
    Ἀπους,
    /// Like `δεξῐος`: 3 words.
    Δεξῐος,
    /// Like `ταυτος`: 3 words.
    Ταυτος,
    /// Like `ἀντῐπαις`: 3 words.
    Ἀντῐπαις,
    /// Like `τοιοσδε`: 3 words.
    Τοιοσδε,
    /// Like `μωνῠξ`: 3 words.
    Μωνῠξ,
    /// Like `νηποινος`: 3 words.
    Νηποινος,
    /// Like `ἱερος`: 1 words.
    Ἱερος,
    /// Like `χερηα`: 1 words.
    Χερηα,
    /// Like `ἀειμακαριστος`: 1 words.
    Ἀειμακαριστος,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 46] = [
        Declension::Μᾰλᾰκος,
        Declension::Ἐμπεδος,
        Declension::Λᾰμπρος,
        Declension::Προσηκων,
        Declension::Ἀτελης,
        Declension::Ψῑλος,
        Declension::Προπᾱς,
        Declension::Ἀσημος,
        Declension::Ὑστερος,
        Declension::Ἀρῐστος,
        Declension::Πληγεις,
        Declension::Ἀρκων,
        Declension::Εἰκως,
        Declension::Μνημων,
        Declension::Φωνηεις,
        Declension::Λᾰμπρος2,
        Declension::Ζων,
        Declension::Πλειων,
        Declension::Τρᾱχῠς,
        Declension::Τρῐπους,
        Declension::Ὁμηλῐξ,
        Declension::Ἀξῐοχρεως,
        Declension::Εὐχᾰρῐς,
        Declension::Ἀπᾰτωρ,
        Declension::Φῡς,
        Declension::Δῐπλους,
        Declension::Εὐελπῐς,
        Declension::Αἰθοψ,
        Declension::Νομᾰς,
        Declension::Βοτηρ,
        Declension::Πλειστος,
        Declension::Μελᾱς,
        Declension::Βαιος,
        Declension::Θηλῠς,
        Declension::Μωλῠς,
        Declension::Δους,
        Declension::Ἀπους,
        Declension::Δεξῐος,
        Declension::Ταυτος,
        Declension::Ἀντῐπαις,
        Declension::Τοιοσδε,
        Declension::Μωνῠξ,
        Declension::Νηποινος,
        Declension::Ἱερος,
        Declension::Χερηα,
        Declension::Ἀειμακαριστος,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 46] = [
    // Μᾰλᾰκος: 1,454 words (ἐρωμενος, παιδῐκος, γᾰλᾰκτῐκος, ὀλῐγος, λευκος, ὁλος).
    declension!("μᾰλᾰκος", 1454, "~ος";
    stem lemma: "~- like μᾰλᾰκος";
    stem rest: "~- like μᾰλᾰκος";
    stem rest_: "~- like μᾰλᾰκος";
    stem adjective_dat_pl: "~- like μᾰλᾰκος";
    nom_n: _, derived, _;
    gen_n: derived, derived, derived;
    dat_m: _, derived, _;
    dat_f: _, derived, _;
    dat_n: derived, derived, derived;
    acc_m: _, derived, _;
    acc_f: _, derived, _;
    acc_n: derived, derived, derived;
    voc_m: _, derived, derived;
    voc_f: _, derived, derived;
    voc_n: derived, derived, derived;
    ),
    // Ἐμπεδος: 931 words (δῐᾰβολος, βᾰρβᾰρος, ἀνᾰπολογητος, πρωτοτοκος, μᾰγος, ἀκᾰθᾰρτος). Like Μᾰλᾰκος, except in 10 cells: nom_f sg, nom_f du, nom_f pl, gen_f sg, gen_f du, dat_f sg, ….
    declension!("ἐμπεδος", 931, "~ος", like Μᾰλᾰκος;
    nom_f: "~ος", "~ω", "~οι";
    gen_f: "~ου", "~οιν", _;
    dat_f: "~ῳ", _, "~οις";
    acc_f: "~ον", _, "~ους";
    voc_f: "~ε", _, _;
    ),
    // Λᾰμπρος: 464 words (μῑκρος, κῡρῐος, ἁγῐος, νεος, ἐρῠθρος, χλωρος). Like Μᾰλᾰκος, except in 4 cells: dat_f du, acc_f du, voc_f du, voc_f pl.
    declension!("λᾰμπρος", 464, "~ος", like Μᾰλᾰκος;
    stem rest_: "~- like τελειος";
    dat_f: _, derived, _;
    acc_f: _, derived, _;
    voc_f: _, derived, derived;
    ),
    // Προσηκων: 327 words (κῠων, μελλων, πᾰρων, ὀζων, νεων, γερων).
    declension!("προσηκων", 327, "~ων";
    stem rest: "~οντ- like προσηκων";
    stem rest_: "~ουσ- like προσηκων";
    stem adjective_dat_pl: "~ουσ- like προσηκων";
    nom_m: "~ων", _, _;
    nom_n: "~ον", derived, _;
    gen_n: derived, derived, derived;
    dat_m: _, derived, _;
    dat_f: _, derived, _;
    dat_n: derived, derived, derived;
    acc_m: "~οντᾰ", derived, _;
    acc_f: _, derived, _;
    acc_n: _, derived, derived;
    voc_m: "~ων", derived, derived;
    voc_f: _, derived, derived;
    voc_n: _, derived, derived;
    ),
    // Ἀτελης: 286 words (ὑγῐης, ἀβαρης, ψευδης, πληρης, ἀτελης, δηλητηρῐωδης). Like Ἐμπεδος, except in 21 cells: nom_m sg, nom_m du, nom_m pl, nom_f sg, nom_f du, nom_f pl, ….
    declension!("ἀτελης", 286, "~ης", like Ἐμπεδος;
    stem adjective_dat_pl: "~εσῐ- like ἀτελης";
    nom_m: "~ης", "~ει", "~εις";
    nom_f: "~ης", "~ει", "~εις";
    nom_n: "~ες", _, "~η";
    gen_m: "~ους", _, _;
    gen_f: "~ους", _, _;
    dat_m: "~ει", _, _;
    dat_f: "~ει", _, _;
    dat_n: _, _, derived;
    acc_m: "~η", _, "~εις";
    acc_f: "~η", _, "~εις";
    voc_m: "~ες", _, _;
    voc_f: "~ες", _, _;
    comparative: "~εστερος";
    superlative: "~εστᾰτος";
    ),
    // Ψῑλος: 168 words (ἀλεξανδρῑνος, ἀδελφος, ὑψῐστος, ἰουδαϊκος, ξᾰνθος, σωστος). Like Μᾰλᾰκος, except in 2 cells: comparative, superlative.
    declension!("ψῑλος", 168, "~ος", like Μᾰλᾰκος;
    comparative: "~οτερος";
    superlative: "~οτᾰτος";
    ),
    // Προπᾱς: 168 words (ἀδᾰμᾱς, λῡσᾱς, ἱστᾱς, γῐγᾱς, βῐβᾱς, φωνησᾱς).
    declension!("προπᾱς", 168, "~ᾱς";
    stem rest: "~ᾰντ- like προσηκων";
    stem rest_: "~ᾱσ- like προσηκων";
    stem adjective_dat_pl: "~ᾱσ- like προσηκων";
    nom_m: "~ᾱς", _, _;
    nom_n: "~ᾰν", derived, _;
    gen_n: derived, derived, derived;
    dat_m: _, derived, _;
    dat_f: _, derived, _;
    dat_n: derived, derived, derived;
    acc_m: "~ᾰντᾰ", derived, _;
    acc_f: _, derived, _;
    acc_n: _, derived, derived;
    voc_m: "~ᾱς", derived, derived;
    voc_f: _, derived, derived;
    voc_n: _, derived, derived;
    ),
    // Ἀσημος: 152 words (ἀκᾰρπος, εὐχᾰρῐστος, πᾰρᾰκλητος, ἀγνωστος, πᾰλῐμψηστος, πρᾱος). Like Ἐμπεδος, except in 2 cells: comparative, superlative.
    declension!("ἀσημος", 152, "~ος", like Ἐμπεδος;
    comparative: "~οτερος";
    superlative: "~οτᾰτος";
    ),
    // Ὑστερος: 127 words (ἰδῐωτερος, προτερος, βᾰθῠτερος, πρεσβῠτερος, βεβαιοτερος, ὑστερος).
    declension!("ὑστερος", 127, "~ερος";
    stem lemma: "~ερ- like μᾰλᾰκος";
    stem rest_: "~ερ- like τελειος";
    stem adjective_dat_pl: "~ερ- like μᾰλᾰκος";
    nom_m: _, "~ερω", "~εροι";
    nom_n: _, _, "~ερᾰ";
    gen_m: "~ερου", "~εροιν", "~ερων";
    dat_m: "~ερῳ", _, _;
    dat_f: _, derived, _;
    dat_n: _, _, derived;
    acc_m: _, _, "~ερους";
    acc_f: _, derived, _;
    acc_n: derived, _, _;
    voc_f: _, derived, derived;
    voc_n: derived, _, _;
    comparative: "~ερος";
    superlative: "~ᾰτος";
    adverb: "~ερον";
    ),
    // Ἀρῐστος: 112 words (ἰδῐωτᾰτος, μεγῐστος, ἀρῐστος, ἡκῐστος, στοιχειωδεστᾰτος, κᾰκῐστος). Like Μᾰλᾰκος, except in 3 cells: comparative, superlative, adverb.
    declension!("ἀρῐστος", 112, "~ος", like Μᾰλᾰκος;
    comparative: derived;
    superlative: derived;
    adverb: "~ᾰ";
    ),
    // Πληγεις: 90 words (σῠγγρᾰφεις, θεις, κοιμηθεις, νῠγεις, αὐδηθεις, κᾰλῠφεις).
    declension!("πληγεις", 90, "~ις";
    stem rest: "~ντ- like προσηκων";
    stem rest_: "~ισ- like προσηκων";
    stem adjective_dat_pl: "~ισ- like προσηκων";
    nom_m: "~ις", _, _;
    nom_n: "~ν", derived, _;
    gen_n: derived, derived, derived;
    dat_m: _, derived, _;
    dat_f: _, derived, _;
    dat_n: derived, derived, derived;
    acc_m: "~ντᾰ", derived, _;
    acc_f: _, derived, _;
    acc_n: _, derived, derived;
    voc_m: "~ις", derived, derived;
    voc_f: _, derived, derived;
    voc_n: _, derived, derived;
    ),
    // Ἀρκων: 58 words (ποιων, κυκλων, σημειων, βῐων, δηλων, γονῠπετων). Like Προσηκων, except in 16 cells: nom_n sg, nom_n du, gen_n sg, gen_n du, gen_n pl, dat_m du, ….
    declension!("ἀρκων", 58, "~ων", like Προσηκων;
    stem rest: "~ουντ- like προσηκων";
    nom_n: "~ουν", derived, _;
    gen_n: derived, derived, derived;
    dat_m: _, derived, _;
    dat_n: derived, derived, _;
    acc_m: "~ουντᾰ", derived, _;
    acc_n: _, derived, derived;
    voc_m: _, derived, derived;
    voc_n: _, derived, derived;
    ),
    // Εἰκως: 48 words (λελῠκως, κεχηνως, βεβηκως, εἰδως, πεπαιδευκως, ἠγᾰπηκως).
    declension!("εἰκως", 48, "~ως";
    stem rest: "~οτ- like προσηκων";
    stem rest_: "~υι- like εἰκως";
    nom_m: "~ως", _, _;
    nom_n: "~ος", derived, _;
    gen_n: derived, derived, derived;
    dat_m: _, derived, "~οσῐ|~οσῐν";
    dat_f: _, derived, "~υιαις";
    dat_n: derived, derived, _;
    acc_m: "~οτᾰ", derived, _;
    acc_f: _, derived, _;
    acc_n: _, derived, derived;
    voc_m: "~ως", derived, derived;
    voc_f: _, derived, derived;
    voc_n: _, derived, derived;
    ),
    // Μνημων: 39 words (σωφρων, ἀκμων, βελτῑων, κᾰκοδαιμων, ὑπερφρων, εὐφρων).
    declension!("μνημων", 39, "~ων";
    stem adjective_dat_pl: "~οσῐ- like ἀτελης";
    nom_m: "~ων", "~ονε", "~ονες";
    nom_f: "~ων", "~ονε", "~ονες";
    nom_n: "~ον", _, "~ονᾰ";
    gen_m: "~ονος", "~ονοιν", "~ονων";
    gen_f: "~ονος", "~ονοιν", "~ονων";
    dat_m: "~ονῐ", _, _;
    dat_f: "~ονῐ", _, _;
    dat_n: _, _, derived;
    acc_m: "~ονᾰ", _, "~ονᾰς";
    acc_f: "~ονᾰ", _, "~ονᾰς";
    voc_m: "~ον", _, _;
    voc_f: "~ον", _, _;
    comparative: "~ονεστερος";
    superlative: "~ονεστᾰτος";
    adverb: "~ονως";
    ),
    // Φωνηεις: 30 words (ἀργῐνοεις, χᾰρῐεις, φωνηεις, αἱμᾰτοεις, ῥοδοεις, ἰχθυοεις). Like Πληγεις, except in 8 cells: dat_f du, dat_f pl, acc_f du, voc_m sg, voc_f du, voc_f pl, ….
    declension!("φωνηεις", 30, "~ις", like Πληγεις;
    stem rest_: "~σσ- like προσηκων";
    dat_f: _, derived, "~σσαις";
    acc_f: _, derived, _;
    voc_m: "~ν", _, _;
    voc_f: _, derived, derived;
    comparative: "~στερος";
    superlative: "~στᾰτος";
    ),
    // Λᾰμπρος2: 26 words (ψῡχρος, ἀλβος, λυγρος, μᾰδᾰρος, νεαρος, ὑγρος). Like Λᾰμπρος, except in 2 cells: comparative, superlative.
    declension!("λᾰμπρος", 26, "~ος", like Λᾰμπρος;
    comparative: "~οτερος";
    superlative: "~οτᾰτος";
    ),
    // Ζων: 26 words (ὠν, νῑκων, τῑμων, δῐαιτων, θεων, αὐδων). Like Πληγεις, except in 7 cells: nom_m sg, dat_f du, dat_n pl, acc_f du, voc_m sg, voc_f du, ….
    declension!("ζων", 26, "~ν", like Πληγεις;
    stem rest_: "~σ- like προσηκων";
    stem adjective_dat_pl: "~σ- like προσηκων";
    nom_m: "~ν", _, _;
    dat_f: _, derived, _;
    dat_n: _, _, derived;
    acc_f: _, derived, _;
    voc_m: "~ν", _, _;
    voc_f: _, derived, derived;
    ),
    // Πλειων: 25 words (βρᾰχῑων, μειων, μᾰσσων, μειζων, πλειων, ἐλᾰσσων). Like Μνημων, except in 10 cells: nom_m pl, nom_f pl, nom_n pl, acc_m sg, acc_m pl, acc_f sg, ….
    declension!("πλειων", 25, "~ων", like Μνημων;
    nom_m: _, _, "~ονες|~ους";
    nom_f: _, _, "~ονες|~ους";
    nom_n: _, _, "~ονᾰ|~ω";
    acc_m: "~ονᾰ|~ω", _, "~ονᾰς|~ους";
    acc_f: "~ονᾰ|~ω", _, "~ονᾰς|~ους";
    comparative: derived;
    superlative: "~στος";
    adverb: "~ον";
    ),
    // Τρᾱχῠς: 25 words (τᾰχῠς, βρᾰδῠς, πολῠς, βᾰθῠς, ὀξῠς, ὠκῠς). Like Ἀτελης, except in 14 cells: nom_n pl, gen_m sg, gen_m du, gen_m pl, dat_f du, dat_f pl, ….
    declension!("τρᾱχῠς", 25, "~ῠς", like Ἀτελης;
    stem lemma: "~ῠ- like τρᾱχῠς";
    stem rest_: "~ει- like εἰκως";
    nom_n: _, _, "~εᾰ";
    gen_m: "~εος", "~εοιν", "~εων";
    dat_f: _, derived, "~ειαις";
    acc_f: _, derived, _;
    acc_n: derived, _, _;
    voc_f: _, derived, derived;
    voc_n: derived, _, _;
    comparative: "~ῠτερος";
    superlative: "~ῠτᾰτος";
    adverb: "~εως";
    ),
    // Τρῐπους: 8 words (τετρᾰπους, ὀκτωπους, τρῐπους, ὀκτᾰπους, εἰλῐπους, ἐρῠθροπους). Like Ζων, except in 29 cells: nom_m sg, nom_f sg, nom_f du, nom_f pl, nom_n sg, nom_n du, ….
    declension!("τρῐπους", 8, "~υς", like Ζων;
    stem rest: "~δ- like προσηκων";
    stem adjective_dat_pl: "~σῐ- like ἀτελης";
    nom_m: "~υς", _, _;
    nom_f: "~υς", "~δε", "~δες";
    nom_n: "~υν", derived, _;
    gen_f: "~δος", "~δοιν", "~δων";
    gen_n: derived, derived, derived;
    dat_m: _, derived, _;
    dat_f: "~δῐ", _, _;
    dat_n: derived, derived, derived;
    acc_m: "~δᾰ", derived, _;
    acc_f: "~δᾰ", _, "~δᾰς";
    acc_n: _, derived, derived;
    voc_m: "~υς", derived, derived;
    voc_f: "~υς", _, _;
    voc_n: _, derived, derived;
    ),
    // Ὁμηλῐξ: 8 words (ἡλῐξ, κᾰλλῐθρῐξ, πτωξ, ὁμηλῐξ, χρῡσᾰμπῠξ, βλᾱξ).
    declension!("ὁμηλῐξ", 8, "~ξ";
    stem adjective_dat_pl: "~ξῐ- like ἀτελης";
    nom_m: "~ξ", "~κε", "~κες";
    nom_f: "~ξ", "~κε", "~κες";
    nom_n: "~ξ", _, "~κᾰ";
    gen_m: "~κος", "~κοιν", "~κων";
    gen_f: "~κος", "~κοιν", "~κων";
    dat_m: "~κῐ", _, _;
    dat_f: "~κῐ", _, _;
    dat_n: _, _, derived;
    acc_m: "~κᾰ", _, "~κᾰς";
    acc_f: "~κᾰ", _, "~κᾰς";
    voc_m: "~ξ", _, _;
    voc_f: "~ξ", _, _;
    comparative: "~κοτερος";
    superlative: "~κοτᾰτος";
    adverb: "~κως";
    ),
    // Ἀξῐοχρεως: 7 words (ἱλεως, λεπτογεως, ἀξῐοχρεως, σως, μονοκερως, πλεως). Like Ἐμπεδος, except in 17 cells: nom_m sg, nom_m pl, nom_f sg, nom_f pl, nom_n sg, gen_m sg, ….
    declension!("ἀξῐοχρεως", 7, "~ως", like Ἐμπεδος;
    nom_m: "~ως", _, "~ῳ";
    nom_f: "~ως", _, "~ῳ";
    nom_n: "~ων", _, _;
    gen_m: "~ω", "~ῳν", _;
    gen_f: "~ω", "~ῳν", _;
    dat_m: _, _, "~ῳς";
    dat_f: _, _, "~ῳς";
    acc_m: "~ω|~ων", _, "~ως";
    acc_f: "~ω|~ων", _, "~ως";
    voc_m: "~ως", _, _;
    voc_f: "~ως", _, _;
    ),
    // Εὐχᾰρῐς: 7 words (ἀγνως, εὐχᾰρῐς, πενης, ἑστως, ξῠνᾰφεστως, ἀφεστως).
    declension!("εὐχᾰρῐς", 7, "~ς";
    stem lemma: "~- like τρᾱχῠς";
    nom_m: _, "~τε", "~τες";
    nom_f: "~ς", "~τε", "~τες";
    nom_n: _, _, "~τᾰ";
    gen_m: "~τος", "~τοιν", "~των";
    gen_f: "~τος", "~τοιν", "~των";
    dat_m: "~τῐ", _, "~σῐ";
    dat_f: "~τῐ", _, "~σῐ";
    acc_m: _, _, "~τᾰς";
    acc_f: "~ν", _, "~τᾰς";
    acc_n: derived, _, _;
    voc_f: "~", _, _;
    voc_n: derived, _, _;
    comparative: "~τωτερος";
    superlative: "~τωτᾰτος";
    adverb: "~τως";
    ),
    // Ἀπᾰτωρ: 6 words (ἀλεκτωρ, ἀπᾰτωρ, μεγᾰλητωρ, ἀγηνωρ, ὑψηνωρ, ἀμητωρ).
    declension!("ἀπᾰτωρ", 6, "~ω~";
    stem rest: "~ο~- like προσηκων";
    stem adjective_dat_pl: "~ο~σῐ- like ἀτελης";
    nom_m: "~ω~", _, _;
    nom_f: "~ω~", "~ο~ε", "~ο~ες";
    nom_n: "~ο~", derived, _;
    gen_f: "~ο~ος", "~ο~οιν", "~ο~ων";
    gen_n: derived, derived, derived;
    dat_m: _, derived, _;
    dat_f: "~ο~ῐ", _, _;
    dat_n: derived, derived, derived;
    acc_m: "~ο~ᾰ", derived, _;
    acc_f: "~ο~ᾰ", _, "~ο~ᾰς";
    acc_n: _, derived, derived;
    voc_m: "~ο~", derived, derived;
    voc_f: "~ο~", _, _;
    voc_n: _, derived, derived;
    ),
    // Φῡς: 6 words (δεικνῡς, κᾰτᾰδῡς, ἐπῐμειγνῡς, ἀποδῡς, ἀποδεικνῡς, φῡς).
    declension!("φῡς", 6, "~ῡς";
    stem rest: "~ῠντ- like προσηκων";
    stem rest_: "~ῡσ- like προσηκων";
    stem adjective_dat_pl: "~ῡσ- like προσηκων";
    nom_m: "~ῡς", _, _;
    nom_n: "~ῠν", derived, _;
    gen_n: derived, derived, derived;
    dat_m: _, derived, _;
    dat_f: _, derived, _;
    dat_n: derived, derived, derived;
    acc_m: "~ῠντᾰ", derived, _;
    acc_f: _, derived, _;
    acc_n: _, derived, derived;
    voc_m: "~ῡς", derived, derived;
    voc_f: _, derived, derived;
    voc_n: _, derived, derived;
    ),
    // Δῐπλους: 6 words (δῐπλους, ἁπλους, χρῡσους, ἀργῠρους, χᾰλκους, πορφῠρους). Like Μᾰλᾰκος, except in 8 cells: nom_m sg, nom_m du, nom_n sg, nom_n pl, acc_m sg, voc_m sg, ….
    declension!("δῐπλους", 6, "~ους", like Μᾰλᾰκος;
    nom_m: "~ους", "~ω|~ω", _;
    nom_n: "~ουν", _, "~ᾱ";
    acc_m: "~ουν", _, _;
    voc_m: "~ου", _, _;
    comparative: "~εωτερος";
    superlative: "~εωτᾰτος";
    ),
    // Εὐελπῐς: 6 words (μεγᾰς, ἐϋς, εὐελπῐς, ἀνᾰλκῐς, φερᾰσπῐς, ἀπᾰτρῐς). Like Τρῐπους, except in 5 cells: nom_f sg, acc_f sg, acc_n sg, voc_f sg, voc_n sg.
    declension!("εὐελπῐς", 6, "~ς", like Τρῐπους;
    stem lemma: "~- like τρᾱχῠς";
    nom_f: "~ς", _, _;
    acc_f: "~ν", _, _;
    acc_n: derived, _, _;
    voc_f: "~", _, _;
    voc_n: derived, _, _;
    ),
    // Αἰθοψ: 5 words (οἰνοψ, μῠωψ, αἰθοψ, νωροψ, ἠνοψ).
    declension!("αἰθοψ", 5, "~ψ";
    stem rest: "~π- like προσηκων";
    stem adjective_dat_pl: "~ψῐ- like ἀτελης";
    nom_m: "~ψ", _, _;
    nom_f: "~ψ", "~πε", "~πες";
    nom_n: "~ψ", derived, _;
    gen_f: "~πος", "~ποιν", "~πων";
    gen_n: derived, derived, derived;
    dat_m: _, derived, _;
    dat_f: "~πῐ", _, _;
    dat_n: derived, derived, derived;
    acc_m: "~πᾰ", derived, _;
    acc_f: "~πᾰ", _, "~πᾰς";
    acc_n: _, derived, derived;
    voc_m: "~ψ", derived, derived;
    voc_f: "~ψ", _, _;
    voc_n: _, derived, derived;
    ),
    // Νομᾰς: 5 words (δρομᾰς, νομᾰς, ἐπηλῠς, λογᾰς, πολυδειρᾰς). Like Εὐελπῐς, except in 4 cells: acc_m sg, acc_f sg, voc_m sg, voc_f sg.
    declension!("νομᾰς", 5, "~ς", like Εὐελπῐς;
    acc_m: "~δᾰ", _, _;
    acc_f: "~δᾰ", _, _;
    voc_m: "~ς", _, _;
    voc_f: "~ς", _, _;
    ),
    // Βοτηρ: 5 words (ἀρρην, δᾰϊφρων, βοτηρ, κᾰρτεροχειρ, ἑκτωρ). Like Εὐελπῐς, except in 25 cells: nom_m sg, nom_f sg, nom_f du, nom_f pl, nom_n du, gen_f sg, ….
    declension!("βοτηρ", 5, "~", like Εὐελπῐς;
    stem rest: "~- like προσηκων";
    nom_m: "~", _, _;
    nom_f: "~", "~ε", "~ες";
    nom_n: _, derived, _;
    gen_f: "~ος", "~οιν", "~ων";
    gen_n: derived, derived, derived;
    dat_m: _, derived, _;
    dat_f: "~ῐ", _, _;
    dat_n: derived, derived, _;
    acc_m: "~ᾰ", derived, _;
    acc_f: "~ᾰ", _, "~ᾰς";
    acc_n: _, derived, derived;
    voc_m: _, derived, derived;
    voc_n: _, derived, derived;
    ),
    // Πλειστος: 4 words (πρωτος, πλειστος, κᾰλλῐστος, ἀγχιστος). Like Ἀρῐστος, except in 2 cells: superlative, adverb.
    declension!("πλειστος", 4, "~ος", like Ἀρῐστος;
    superlative: "~ῐστος";
    adverb: "~ον";
    ),
    // Μελᾱς: 4 words (μελᾱς, λευκομελᾱς, τᾰλᾱς, χλωρομελᾱς).
    declension!("μελᾱς", 4, "~ᾱς";
    stem rest_: "~αιν- like προσηκων";
    nom_m: "~ᾱς", "~ᾰνε", "~ᾰνες";
    nom_n: "~ᾰν", _, "~ᾰνᾰ";
    gen_m: "~ᾰνος", "~ᾰνοιν", "~ᾰνων";
    dat_m: "~ᾰνῐ", _, "~ᾰσῐ|~ᾰσῐν";
    dat_f: _, derived, "~αιναις";
    acc_m: "~ᾰνᾰ", _, "~ᾰνᾰς";
    acc_f: _, derived, _;
    voc_m: "~ᾰν", _, _;
    voc_f: _, derived, derived;
    comparative: "~ᾰντερος";
    superlative: "~ᾰντᾰτος";
    adverb: "~ᾰνως";
    ),
    // Βαιος: 4 words (πᾰλαιος, βαιος, ἠβαιος, πᾰνημερῐος). Like Λᾰμπρος2, except in 1 cell: adverb.
    declension!("βαιος", 4, "~ος", like Λᾰμπρος2;
    adverb: "~ον";
    ),
    // Θηλῠς: 4 words (ἡμῐσῠς, θηλῠς, πρεσβῠς, ἀμφῐβρᾰχῠς). Like Τρᾱχῠς, except in 1 cell: nom_n pl.
    declension!("θηλῠς", 4, "~ῠς", like Τρᾱχῠς;
    nom_n: _, _, "~η|~εᾰ";
    ),
    // Μωλῠς: 4 words (νεκῠς, εὐϊχθῠς, πολῠϊχθῠς, μωλῠς). Like Τρᾱχῠς, except in 20 cells: nom_m du, nom_m pl, nom_f sg, nom_f du, nom_f pl, nom_n pl, ….
    declension!("μωλῠς", 4, "~ῠς", like Τρᾱχῠς;
    stem adjective_dat_pl: "~ῠσῐ- like ἀτελης";
    nom_m: _, "~ῠε|~ῡ", "~ῠες|~ῡς";
    nom_f: "~ῠς", "~ῠε|~ῡ", "~ῠες|~ῡς";
    nom_n: _, _, "~ῠᾰ";
    gen_m: "~ῠος", "~ῠοιν", "~ῠων";
    gen_f: "~ῠος", "~ῠοιν", "~ῠων";
    dat_m: "~ῠϊ|~υι", _, _;
    dat_f: "~ῠϊ|~υι", _, _;
    dat_n: _, _, derived;
    acc_m: _, _, "~ῡς|~ῠᾰς";
    acc_f: "~ῠν", _, "~ῡς|~ῠᾰς";
    voc_f: "~ῠ", _, _;
    adverb: "~ῠως";
    ),
    // Δους: 4 words (δους, βῐους, δῐδους, γνους). Like Πληγεις, except in 7 cells: nom_m sg, dat_f du, dat_n pl, acc_f du, voc_m sg, voc_f du, ….
    declension!("δους", 4, "~υς", like Πληγεις;
    stem rest_: "~υσ- like προσηκων";
    stem adjective_dat_pl: "~υσ- like προσηκων";
    nom_m: "~υς", _, _;
    dat_f: _, derived, _;
    dat_n: _, _, derived;
    acc_f: _, derived, _;
    voc_m: "~υς", _, _;
    voc_f: _, derived, derived;
    ),
    // Ἀπους: 4 words (πολυπους, ἀπους, χειμαρρους, δῐπους). Like Τρῐπους, except in 2 cells: acc_m sg, acc_f sg.
    declension!("ἀπους", 4, "~υς", like Τρῐπους;
    acc_m: "~υν", _, _;
    acc_f: "~υν", _, _;
    ),
    // Δεξῐος: 3 words (δεξῐος, ἀρῐστερος, φεριστος). Like Λᾰμπρος, except in 3 cells: comparative, superlative, adverb.
    declension!("δεξῐος", 3, "~ος", like Λᾰμπρος;
    comparative: derived;
    superlative: derived;
    adverb: "~ᾰ|~ως|~οθεν";
    ),
    // Ταυτος: 3 words (ἀλλος, ταυτος, τηλῐκουτος). Like Ἀρῐστος, except in 2 cells: nom_n sg, adverb.
    declension!("ταυτος", 3, "~ος", like Ἀρῐστος;
    nom_n: "~ο", _, _;
    adverb: "~ο";
    ),
    // Ἀντῐπαις: 3 words (ὀκρῐς, ἀντῐπαις, ἀλκῠονῐς). Like Εὐελπῐς, except in 2 cells: acc_m sg, acc_f sg.
    declension!("ἀντῐπαις", 3, "~ς", like Εὐελπῐς;
    acc_m: "~δᾰ", _, _;
    acc_f: "~δᾰ", _, _;
    ),
    // Τοιοσδε: 3 words (τοιοσδε, τοσοσδε, τηλῐκοσδε).
    declension!("τοιοσδε", 3, "~οσ~";
    nom_m: "~οσ~", "~ω~", "~οι~";
    nom_f: "~η~", "~ᾱ~", "~αι~";
    nom_n: "~ον~", _, "~ᾰ~";
    gen_m: "~ου~", "~οιν~", "~ων~";
    gen_f: "~ησ~", "~αιν~", "~ων~";
    dat_m: "~ῳ~", _, "~οισ~";
    dat_f: "~ῃ~", _, "~αισ~";
    acc_m: "~ον~", _, "~ουσ~";
    acc_f: "~ην~", _, "~ᾱσ~";
    ),
    // Μωνῠξ: 3 words (μωνῠξ, γᾰμψωνῠξ, κρᾰτερωνῠξ). Like Ὁμηλῐξ, except in 25 cells: nom_f du, nom_f pl, nom_n du, gen_f sg, gen_f du, gen_f pl, ….
    declension!("μωνῠξ", 3, "~ξ", like Ὁμηλῐξ;
    stem rest: "~χ- like προσηκων";
    nom_f: _, "~χε", "~χες";
    nom_n: _, derived, _;
    gen_f: "~χος", "~χοιν", "~χων";
    gen_n: derived, derived, derived;
    dat_m: _, derived, _;
    dat_f: "~χῐ", _, _;
    dat_n: derived, derived, _;
    acc_m: "~χᾰ", derived, _;
    acc_f: "~χᾰ", _, "~χᾰς";
    acc_n: _, derived, derived;
    voc_m: _, derived, derived;
    voc_n: _, derived, derived;
    comparative: derived;
    superlative: derived;
    ),
    // Νηποινος: 3 words (νηποινος, τᾰλαυρῑνος, ἀλῐᾰστος). Like Ἀσημος, except in 1 cell: adverb.
    declension!("νηποινος", 3, "~ος", like Ἀσημος;
    adverb: "~ον";
    ),
    // Ἱερος: 1 words (ἱερος).
    declension!("ἱερος", 1, "ἱ~ος";
    nom_m: "ῐ̔~ος", "ῐ̔~ω", "ῐ̔~οι";
    nom_f: "ῐ̔~ᾱ", "ῐ̔~ᾱ", "ῐ̔~αι";
    nom_n: "ῐ̔~ον", _, "ῐ̔~ᾰ";
    gen_m: "ῐ̔~ου", "ῐ̔~οιν", "ῐ̔~ων";
    gen_f: "ῐ̔~ᾱς", "ῐ̔~αιν", "ῐ̔~ων";
    dat_m: "ῐ̔~ῳ", _, "ῐ̔~οις";
    dat_f: "ῐ̔~ᾳ", _, "ῐ̔~αις";
    acc_m: "ῐ̔~ον", _, "ῐ̔~ους";
    acc_f: "ῐ̔~ᾱν", _, "ῐ̔~ᾱς";
    voc_m: "ῐ̔~ε", _, _;
    voc_f: "ῐ̔~ᾱ", _, _;
    comparative: "ῐ̔~ωτερος";
    superlative: "ῐ̔~ωτᾰτος";
    adverb: "ῐ̔~ως";
    ),
    // Χερηα: 1 words (χερηα).
    declension!("χερηα", 1, "~ηα";
    nom_m: "~ηα", _, "~ηες";
    nom_f: _, _, "~ηες";
    nom_n: _, _, "~ηα|~εια";
    dat_m: "~ηϊ|~ηϊ", _, _;
    dat_f: "~ηϊ|~ηϊ", _, _;
    acc_m: "~ηα|~εια", _, _;
    acc_f: "~ηα|~εια", _, _;
    ),
    // Ἀειμακαριστος: 1 words (ἀειμακαριστος).
    declension!("ἀειμακαριστος", 1, "ἀ~α~α~ι~ος";
    stem lemma: "~ᾰ~ᾰ~ῐ~- like ἀειμακαριστος";
    stem rest_: "~ᾰ~ᾰ~ῐ~- like ἀειμακαριστος";
    nom_m: _, "ᾰ̓~ᾰ~ᾰ~ῐ~ω", "ᾰ̓~ᾰ~ᾰ~ῐ~οι";
    nom_n: _, _, "ᾰ̓~ᾰ~ᾰ~ῐ~ᾰ";
    gen_m: "ᾰ̓~ᾰ~ᾰ~ῐ~ου", "ᾰ̓~ᾰ~ᾰ~ῐ~οιν", "ᾰ̓~ᾰ~ᾰ~ῐ~ων";
    dat_m: "ᾰ̓~ᾰ~ᾰ~ῐ~ῳ", _, "ᾰ̓~ᾰ~ᾰ~ῐ~οις";
    dat_f: _, derived, "ᾰ̓~ᾰ~ᾰ~ῐ~αις";
    acc_m: _, _, "ᾰ̓~ᾰ~ᾰ~ῐ~ους";
    acc_f: _, derived, _;
    acc_n: derived, _, _;
    voc_f: _, derived, derived;
    voc_n: derived, _, _;
    comparative: "ᾰ̓~ᾰ~ᾰ~ῐ~οτερος";
    superlative: "ᾰ̓~ᾰ~ᾰ~ῐ~οτᾰτος";
    adverb: "ᾰ̓~ᾰ~ᾰ~ῐ~ως";
    ),
];
