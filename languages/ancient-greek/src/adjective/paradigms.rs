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
     $(stem adjective_sg: $s0:tt;)?
     $(stem adjective: $s1:tt;)?
     $(stem adjective_f: $s2:tt;)?
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
    /// Like `τελειος`: 464 words.
    Τελειος,
    /// Like `προσηκων`: 328 words.
    Προσηκων,
    /// Like `ἀτελης`: 286 words.
    Ἀτελης,
    /// Like `ψῑλος`: 168 words.
    Ψῑλος,
    /// Like `προπᾱς`: 168 words.
    Προπᾱς,
    /// Like `ἀσημος`: 152 words.
    Ἀσημος,
    /// Like `ὑστερος`: 128 words.
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
    /// Like `ζων`: 29 words.
    Ζων,
    /// Like `λᾰμπρος`: 26 words.
    Λᾰμπρος,
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
    /// Like `πενης`: 7 words.
    Πενης,
    /// Like `ἀπᾰτωρ`: 6 words.
    Ἀπᾰτωρ,
    /// Like `φῡς`: 6 words.
    Φῡς,
    /// Like `ἁπλους`: 6 words.
    Ἁπλους,
    /// Like `εὐελπῐς`: 6 words.
    Εὐελπῐς,
    /// Like `βοτηρ`: 6 words.
    Βοτηρ,
    /// Like `αἰθοψ`: 5 words.
    Αἰθοψ,
    /// Like `νομᾰς`: 5 words.
    Νομᾰς,
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
    /// Like `ἀλλος`: 3 words.
    Ἀλλος,
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
    /// Like `ὁλοκληρος`: 1 words.
    Ὁλοκληρος,
    /// Like `δνοφερος`: 1 words.
    Δνοφερος,
    /// Like `γενομενω`: 1 words.
    Γενομενω,
    /// Like `γενομενοιν`: 1 words.
    Γενομενοιν,
    /// Like `γενομεναιν`: 1 words.
    Γενομεναιν,
    /// Like `μελᾰνθρῐξ`: 1 words.
    Μελᾰνθρῐξ,
    /// Like `ὑπαρχομενη`: 1 words.
    Ὑπαρχομενη,
    /// Like `ὑπαρχον`: 1 words.
    Ὑπαρχον,
    /// Like `ὑπαρχομενον`: 1 words.
    Ὑπαρχομενον,
    /// Like `ψησας`: 1 words.
    Ψησας,
    /// Like `χερηα`: 1 words.
    Χερηα,
    /// Like `ἀτηρος`: 1 words.
    Ἀτηρος,
    /// Like `ἀειμακαριστος`: 1 words.
    Ἀειμακαριστος,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 57] = [
        Declension::Μᾰλᾰκος,
        Declension::Ἐμπεδος,
        Declension::Τελειος,
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
        Declension::Ζων,
        Declension::Λᾰμπρος,
        Declension::Πλειων,
        Declension::Τρᾱχῠς,
        Declension::Τρῐπους,
        Declension::Ὁμηλῐξ,
        Declension::Ἀξῐοχρεως,
        Declension::Πενης,
        Declension::Ἀπᾰτωρ,
        Declension::Φῡς,
        Declension::Ἁπλους,
        Declension::Εὐελπῐς,
        Declension::Βοτηρ,
        Declension::Αἰθοψ,
        Declension::Νομᾰς,
        Declension::Πλειστος,
        Declension::Μελᾱς,
        Declension::Βαιος,
        Declension::Θηλῠς,
        Declension::Μωλῠς,
        Declension::Δους,
        Declension::Ἀπους,
        Declension::Δεξῐος,
        Declension::Ἀλλος,
        Declension::Ἀντῐπαις,
        Declension::Τοιοσδε,
        Declension::Μωνῠξ,
        Declension::Νηποινος,
        Declension::Ἱερος,
        Declension::Ὁλοκληρος,
        Declension::Δνοφερος,
        Declension::Γενομενω,
        Declension::Γενομενοιν,
        Declension::Γενομεναιν,
        Declension::Μελᾰνθρῐξ,
        Declension::Ὑπαρχομενη,
        Declension::Ὑπαρχον,
        Declension::Ὑπαρχομενον,
        Declension::Ψησας,
        Declension::Χερηα,
        Declension::Ἀτηρος,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 57] = [
    // Μᾰλᾰκος: 1,454 words (ἐρωμενος, παιδῐκος, γᾰλᾰκτῐκος, ὀλῐγος, λευκος, ὁλος).
    declension!("μᾰλᾰκος", 1454, "~ος";
    nom_m: "~ος", "~ω", "~οι";
    nom_f: "~η", "~ᾱ", "~αι";
    nom_n: "~ον", _, "~ᾰ";
    gen_m: "~ου", "~οιν", "~ων";
    gen_f: "~ης", "~αιν", "~ων";
    dat_m: "~ῳ", _, "~οις";
    dat_f: "~ῃ", _, "~αις";
    acc_m: "~ον", _, "~ους";
    acc_f: "~ην", _, "~ᾱς";
    voc_m: "~ε", _, _;
    voc_f: "~η", _, _;
    comparative: "~ωτερος";
    superlative: "~ωτᾰτος";
    adverb: "~ως";
    ),
    // Ἐμπεδος: 931 words (δῐᾰβολος, βᾰρβᾰρος, ἀνᾰπολογητος, πρωτοτοκος, μᾰγος, ἀκᾰθᾰρτος). Like Μᾰλᾰκος, except in 10 cells: nom_f sg, nom_f du, nom_f pl, gen_f sg, gen_f du, dat_f sg, ….
    declension!("ἐμπεδος", 931, "~ος", like Μᾰλᾰκος;
    nom_f: "~ος", "~ω", "~οι";
    gen_f: "~ου", "~οιν", _;
    dat_f: "~ῳ", _, "~οις";
    acc_f: "~ον", _, "~ους";
    voc_f: "~ε", _, _;
    ),
    // Τελειος: 464 words (μῑκρος, κῡρῐος, ἁγῐος, νεος, ἐρῠθρος, χλωρος). Like Μᾰλᾰκος, except in 5 cells: nom_f sg, gen_f sg, dat_f sg, acc_f sg, voc_f sg.
    declension!("τελειος", 464, "~ος", like Μᾰλᾰκος;
    nom_f: "~ᾱ", _, _;
    gen_f: "~ᾱς", _, _;
    dat_f: "~ᾳ", _, _;
    acc_f: "~ᾱν", _, _;
    voc_f: "~ᾱ", _, _;
    ),
    // Προσηκων: 328 words (κῠων, μελλων, πᾰρων, ὀζων, νεων, γερων).
    declension!("προσηκων", 328, "~ων";
    nom_m: "~ων", "~οντε", "~οντες";
    nom_f: "~ουσᾰ", "~ουσᾱ", "~ουσαι";
    nom_n: "~ον", _, "~οντᾰ";
    gen_m: "~οντος", "~οντοιν", "~οντων";
    gen_f: "~ουσης", "~ουσαιν", "~ουσων";
    dat_m: "~οντῐ", _, "~ουσῐ|~ουσῐν";
    dat_f: "~ουσῃ", _, "~ουσαις";
    acc_m: "~οντᾰ", _, "~οντᾰς";
    acc_f: "~ουσᾰν", _, "~ουσᾱς";
    voc_m: "~ων", _, _;
    voc_f: "~ουσᾰ", _, _;
    adverb: "~οντως";
    ),
    // Ἀτελης: 286 words (ὑγῐης, ἀβαρης, ψευδης, πληρης, ἀτελης, δηλητηρῐωδης). Like Ἐμπεδος, except in 22 cells: nom_m sg, nom_m du, nom_m pl, nom_f sg, nom_f du, nom_f pl, ….
    declension!("ἀτελης", 286, "~ης", like Ἐμπεδος;
    nom_m: "~ης", "~ει", "~εις";
    nom_f: "~ης", "~ει", "~εις";
    nom_n: "~ες", _, "~η";
    gen_m: "~ους", _, _;
    gen_f: "~ους", _, _;
    dat_m: "~ει", _, "~εσῐ|~εσῐν";
    dat_f: "~ει", _, "~εσῐ|~εσῐν";
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
    nom_m: "~ᾱς", "~ᾰντε", "~ᾰντες";
    nom_f: "~ᾱσᾰ", "~ᾱσᾱ", "~ᾱσαι";
    nom_n: "~ᾰν", _, "~ᾰντᾰ";
    gen_m: "~ᾰντος", "~ᾰντοιν", "~ᾰντων";
    gen_f: "~ᾱσης", "~ᾱσαιν", "~ᾱσων";
    dat_m: "~ᾰντῐ", _, "~ᾱσῐ|~ᾱσῐν";
    dat_f: "~ᾱσῃ", _, "~ᾱσαις";
    acc_m: "~ᾰντᾰ", _, "~ᾰντᾰς";
    acc_f: "~ᾱσᾰν", _, "~ᾱσᾱς";
    voc_m: "~ᾱς", _, _;
    voc_f: "~ᾱσᾰ", _, _;
    adverb: "~ᾰντως";
    ),
    // Ἀσημος: 152 words (ἀκᾰρπος, εὐχᾰρῐστος, πᾰρᾰκλητος, ἀγνωστος, πᾰλῐμψηστος, πρᾱος). Like Ἐμπεδος, except in 2 cells: comparative, superlative.
    declension!("ἀσημος", 152, "~ος", like Ἐμπεδος;
    comparative: "~οτερος";
    superlative: "~οτᾰτος";
    ),
    // Ὑστερος: 128 words (ἰδῐωτερος, προτερος, βᾰθῠτερος, πρεσβῠτερος, βεβαιοτερος, ὑστερος).
    declension!("ὑστερος", 128, "~ερος";
    nom_m: "~ερος", "~ερω", "~εροι";
    nom_f: "~ερᾱ", "~ερᾱ", "~εραι";
    nom_n: "~ερον", _, "~ερᾰ";
    gen_m: "~ερου", "~εροιν", "~ερων";
    gen_f: "~ερᾱς", "~εραιν", "~ερων";
    dat_m: "~ερῳ", _, "~εροις";
    dat_f: "~ερᾳ", _, "~εραις";
    acc_m: "~ερον", _, "~ερους";
    acc_f: "~ερᾱν", _, "~ερᾱς";
    voc_m: "~ερε", _, _;
    voc_f: "~ερᾱ", _, _;
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
    nom_m: "~ις", "~ντε", "~ντες";
    nom_f: "~ισᾰ", "~ισᾱ", "~ισαι";
    nom_n: "~ν", _, "~ντᾰ";
    gen_m: "~ντος", "~ντοιν", "~ντων";
    gen_f: "~ισης", "~ισαιν", "~ισων";
    dat_m: "~ντῐ", _, "~ισῐ|~ισῐν";
    dat_f: "~ισῃ", _, "~ισαις";
    acc_m: "~ντᾰ", _, "~ντᾰς";
    acc_f: "~ισᾰν", _, "~ισᾱς";
    voc_m: "~ις", _, _;
    voc_f: "~ισᾰ", _, _;
    adverb: "~ντως";
    ),
    // Ἀρκων: 58 words (ποιων, κυκλων, σημειων, βῐων, δηλων, γονῠπετων). Like Προσηκων, except in 11 cells: nom_m du, nom_m pl, nom_n sg, nom_n pl, gen_m sg, gen_m du, ….
    declension!("ἀρκων", 58, "~ων", like Προσηκων;
    nom_m: _, "~ουντε", "~ουντες";
    nom_n: "~ουν", _, "~ουντᾰ";
    gen_m: "~ουντος", "~ουντοιν", "~ουντων";
    dat_m: "~ουντῐ", _, _;
    acc_m: "~ουντᾰ", _, "~ουντᾰς";
    adverb: "~ουντως";
    ),
    // Εἰκως: 48 words (λελῠκως, κεχηνως, βεβηκως, εἰδως, πεπαιδευκως, ἠγᾰπηκως).
    declension!("εἰκως", 48, "~ως";
    nom_m: "~ως", "~οτε", "~οτες";
    nom_f: "~υιᾰ", "~υιᾱ", "~υιαι";
    nom_n: "~ος", _, "~οτᾰ";
    gen_m: "~οτος", "~οτοιν", "~οτων";
    gen_f: "~υιᾱς", "~υιαιν", "~υιων";
    dat_m: "~οτῐ", _, "~οσῐ|~οσῐν";
    dat_f: "~υιᾳ", _, "~υιαις";
    acc_m: "~οτᾰ", _, "~οτᾰς";
    acc_f: "~υιᾰν", _, "~υιᾱς";
    voc_m: "~ως", _, _;
    voc_f: "~υιᾰ", _, _;
    adverb: "~οτως";
    ),
    // Μνημων: 39 words (σωφρων, ἀκμων, βελτῑων, κᾰκοδαιμων, ὑπερφρων, εὐφρων).
    declension!("μνημων", 39, "~ων";
    nom_m: "~ων", "~ονε", "~ονες";
    nom_f: "~ων", "~ονε", "~ονες";
    nom_n: "~ον", _, "~ονᾰ";
    gen_m: "~ονος", "~ονοιν", "~ονων";
    gen_f: "~ονος", "~ονοιν", "~ονων";
    dat_m: "~ονῐ", _, "~οσῐ|~οσῐν";
    dat_f: "~ονῐ", _, "~οσῐ|~οσῐν";
    acc_m: "~ονᾰ", _, "~ονᾰς";
    acc_f: "~ονᾰ", _, "~ονᾰς";
    voc_m: "~ον", _, _;
    voc_f: "~ον", _, _;
    comparative: "~ονεστερος";
    superlative: "~ονεστᾰτος";
    adverb: "~ονως";
    ),
    // Φωνηεις: 30 words (ἀργῐνοεις, χᾰρῐεις, φωνηεις, αἱμᾰτοεις, ῥοδοεις, ἰχθυοεις). Like Πληγεις, except in 14 cells: nom_f sg, nom_f du, nom_f pl, gen_f sg, gen_f du, gen_f pl, ….
    declension!("φωνηεις", 30, "~ις", like Πληγεις;
    nom_f: "~σσᾰ", "~σσᾱ", "~σσαι";
    gen_f: "~σσης", "~σσαιν", "~σσων";
    dat_f: "~σσῃ", _, "~σσαις";
    acc_f: "~σσᾰν", _, "~σσᾱς";
    voc_m: "~ν", _, _;
    voc_f: "~σσᾰ", _, _;
    comparative: "~στερος";
    superlative: "~στᾰτος";
    ),
    // Ζων: 29 words (ὠν, νῑκων, τῑμων, δῐαιτων, θεων, αὐδων). Like Πληγεις, except in 14 cells: nom_m sg, nom_f sg, nom_f du, nom_f pl, gen_f sg, gen_f du, ….
    declension!("ζων", 29, "~ν", like Πληγεις;
    nom_m: "~ν", _, _;
    nom_f: "~σᾰ", "~σᾱ", "~σαι";
    gen_f: "~σης", "~σαιν", "~σων";
    dat_m: _, _, "~σῐ|~σῐν";
    dat_f: "~σῃ", _, "~σαις";
    acc_f: "~σᾰν", _, "~σᾱς";
    voc_m: "~ν", _, _;
    voc_f: "~σᾰ", _, _;
    ),
    // Λᾰμπρος: 26 words (ψῡχρος, ἀλβος, λυγρος, μᾰδᾰρος, νεαρος, ὑγρος). Like Τελειος, except in 2 cells: comparative, superlative.
    declension!("λᾰμπρος", 26, "~ος", like Τελειος;
    comparative: "~οτερος";
    superlative: "~οτᾰτος";
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
    // Τρᾱχῠς: 25 words (τᾰχῠς, βρᾰδῠς, πολῠς, βᾰθῠς, ὀξῠς, ὠκῠς). Like Ἀτελης, except in 22 cells: nom_m sg, nom_f sg, nom_f du, nom_f pl, nom_n sg, nom_n pl, ….
    declension!("τρᾱχῠς", 25, "~ῠς", like Ἀτελης;
    nom_m: "~ῠς", _, _;
    nom_f: "~ειᾰ", "~ειᾱ", "~ειαι";
    nom_n: "~ῠ", _, "~εᾰ";
    gen_m: "~εος", "~εοιν", "~εων";
    gen_f: "~ειᾱς", "~ειαιν", "~ειων";
    dat_f: "~ειᾳ", _, "~ειαις";
    acc_m: "~ῠν", _, _;
    acc_f: "~ειᾰν", _, "~ειᾱς";
    voc_m: "~ῠ", _, _;
    voc_f: "~ειᾰ", _, _;
    comparative: "~ῠτερος";
    superlative: "~ῠτᾰτος";
    adverb: "~εως";
    ),
    // Τρῐπους: 8 words (τετρᾰπους, ὀκτωπους, τρῐπους, ὀκτᾰπους, εἰλῐπους, ἐρῠθροπους). Like Ζων, except in 24 cells: nom_m sg, nom_m du, nom_m pl, nom_f sg, nom_f du, nom_f pl, ….
    declension!("τρῐπους", 8, "~υς", like Ζων;
    nom_m: "~υς", "~δε", "~δες";
    nom_f: "~υς", "~δε", "~δες";
    nom_n: "~υν", _, "~δᾰ";
    gen_m: "~δος", "~δοιν", "~δων";
    gen_f: "~δος", "~δοιν", "~δων";
    dat_m: "~δῐ", _, _;
    dat_f: "~δῐ", _, "~σῐ|~σῐν";
    acc_m: "~δᾰ", _, "~δᾰς";
    acc_f: "~δᾰ", _, "~δᾰς";
    voc_m: "~υς", _, _;
    voc_f: "~υς", _, _;
    adverb: "~δως";
    ),
    // Ὁμηλῐξ: 8 words (ἡλῐξ, κᾰλλῐθρῐξ, πτωξ, ὁμηλῐξ, χρῡσᾰμπῠξ, βλᾱξ).
    declension!("ὁμηλῐξ", 8, "~ξ";
    nom_m: "~ξ", "~κε", "~κες";
    nom_f: "~ξ", "~κε", "~κες";
    nom_n: "~ξ", _, "~κᾰ";
    gen_m: "~κος", "~κοιν", "~κων";
    gen_f: "~κος", "~κοιν", "~κων";
    dat_m: "~κῐ", _, "~ξῐ|~ξῐν";
    dat_f: "~κῐ", _, "~ξῐ|~ξῐν";
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
    // Πενης: 7 words (ἀγνως, εὐχᾰρῐς, πενης, ἑστως, ξῠνᾰφεστως, ἀφεστως).
    declension!("πενης", 7, "~ς";
    nom_m: "~ς", "~τε", "~τες";
    nom_f: "~ς", "~τε", "~τες";
    nom_n: "~", _, "~τᾰ";
    gen_m: "~τος", "~τοιν", "~των";
    gen_f: "~τος", "~τοιν", "~των";
    dat_m: "~τῐ", _, "~σῐ";
    dat_f: "~τῐ", _, "~σῐ";
    acc_m: "~ν", _, "~τᾰς";
    acc_f: "~ν", _, "~τᾰς";
    voc_m: "~", _, _;
    voc_f: "~", _, _;
    comparative: "~τωτερος";
    superlative: "~τωτᾰτος";
    adverb: "~τως";
    ),
    // Ἀπᾰτωρ: 6 words (ἀλεκτωρ, ἀπᾰτωρ, μεγᾰλητωρ, ἀγηνωρ, ὑψηνωρ, ἀμητωρ).
    declension!("ἀπᾰτωρ", 6, "~ω~";
    nom_m: "~ω~", "~ο~ε", "~ο~ες";
    nom_f: "~ω~", "~ο~ε", "~ο~ες";
    nom_n: "~ο~", _, "~ο~ᾰ";
    gen_m: "~ο~ος", "~ο~οιν", "~ο~ων";
    gen_f: "~ο~ος", "~ο~οιν", "~ο~ων";
    dat_m: "~ο~ῐ", _, "~ο~σῐ|~ο~σῐν";
    dat_f: "~ο~ῐ", _, "~ο~σῐ|~ο~σῐν";
    acc_m: "~ο~ᾰ", _, "~ο~ᾰς";
    acc_f: "~ο~ᾰ", _, "~ο~ᾰς";
    voc_m: "~ο~", _, _;
    voc_f: "~ο~", _, _;
    adverb: "~ο~ως";
    ),
    // Φῡς: 6 words (δεικνῡς, κᾰτᾰδῡς, ἐπῐμειγνῡς, ἀποδῡς, ἀποδεικνῡς, φῡς).
    declension!("φῡς", 6, "~ῡς";
    nom_m: "~ῡς", "~ῠντε", "~ῠντες";
    nom_f: "~ῡσᾰ", "~ῡσᾱ", "~ῡσαι";
    nom_n: "~ῠν", _, "~ῠντᾰ";
    gen_m: "~ῠντος", "~ῠντοιν", "~ῠντων";
    gen_f: "~ῡσης", "~ῡσαιν", "~ῡσων";
    dat_m: "~ῠντῐ", _, "~ῡσῐ|~ῡσῐν";
    dat_f: "~ῡσῃ", _, "~ῡσαις";
    acc_m: "~ῠντᾰ", _, "~ῠντᾰς";
    acc_f: "~ῡσᾰν", _, "~ῡσᾱς";
    voc_m: "~ῡς", _, _;
    voc_f: "~ῡσᾰ", _, _;
    adverb: "~ῠντως";
    ),
    // Ἁπλους: 6 words (δῐπλους, ἁπλους, χρῡσους, ἀργῠρους, χᾰλκους, πορφῠρους). Like Μᾰλᾰκος, except in 8 cells: nom_m sg, nom_m du, nom_n sg, nom_n pl, acc_m sg, voc_m sg, ….
    declension!("ἁπλους", 6, "~ους", like Μᾰλᾰκος;
    nom_m: "~ους", "~ω|~ω", _;
    nom_n: "~ουν", _, "~ᾱ";
    acc_m: "~ουν", _, _;
    voc_m: "~ου", _, _;
    comparative: "~εωτερος";
    superlative: "~εωτᾰτος";
    ),
    // Εὐελπῐς: 6 words (μεγᾰς, ἐϋς, εὐελπῐς, ἀνᾰλκῐς, φερᾰσπῐς, ἀπᾰτρῐς). Like Τρῐπους, except in 7 cells: nom_m sg, nom_f sg, nom_n sg, acc_m sg, acc_f sg, voc_m sg, ….
    declension!("εὐελπῐς", 6, "~ς", like Τρῐπους;
    nom_m: "~ς", _, _;
    nom_f: "~ς", _, _;
    nom_n: "~", _, _;
    acc_m: "~ν", _, _;
    acc_f: "~ν", _, _;
    voc_m: "~", _, _;
    voc_f: "~", _, _;
    ),
    // Βοτηρ: 6 words (ἀρρην, δᾰϊφρων, λῡομενη, βοτηρ, κᾰρτεροχειρ, ἑκτωρ). Like Εὐελπῐς, except in 20 cells: nom_m sg, nom_m du, nom_m pl, nom_f sg, nom_f du, nom_f pl, ….
    declension!("βοτηρ", 6, "~", like Εὐελπῐς;
    nom_m: "~", "~ε", "~ες";
    nom_f: "~", "~ε", "~ες";
    nom_n: _, _, "~ᾰ";
    gen_m: "~ος", "~οιν", "~ων";
    gen_f: "~ος", "~οιν", "~ων";
    dat_m: "~ῐ", _, _;
    dat_f: "~ῐ", _, _;
    acc_m: "~ᾰ", _, "~ᾰς";
    acc_f: "~ᾰ", _, "~ᾰς";
    adverb: "~ως";
    ),
    // Αἰθοψ: 5 words (οἰνοψ, μῠωψ, αἰθοψ, νωροψ, ἠνοψ).
    declension!("αἰθοψ", 5, "~ψ";
    nom_m: "~ψ", "~πε", "~πες";
    nom_f: "~ψ", "~πε", "~πες";
    nom_n: "~ψ", _, "~πᾰ";
    gen_m: "~πος", "~ποιν", "~πων";
    gen_f: "~πος", "~ποιν", "~πων";
    dat_m: "~πῐ", _, "~ψῐ|~ψῐν";
    dat_f: "~πῐ", _, "~ψῐ|~ψῐν";
    acc_m: "~πᾰ", _, "~πᾰς";
    acc_f: "~πᾰ", _, "~πᾰς";
    voc_m: "~ψ", _, _;
    voc_f: "~ψ", _, _;
    adverb: "~πως";
    ),
    // Νομᾰς: 5 words (δρομᾰς, νομᾰς, ἐπηλῠς, λογᾰς, πολυδειρᾰς). Like Εὐελπῐς, except in 4 cells: acc_m sg, acc_f sg, voc_m sg, voc_f sg.
    declension!("νομᾰς", 5, "~ς", like Εὐελπῐς;
    acc_m: "~δᾰ", _, _;
    acc_f: "~δᾰ", _, _;
    voc_m: "~ς", _, _;
    voc_f: "~ς", _, _;
    ),
    // Πλειστος: 4 words (πρωτος, πλειστος, κᾰλλῐστος, ἀγχιστος). Like Ἀρῐστος, except in 2 cells: superlative, adverb.
    declension!("πλειστος", 4, "~ος", like Ἀρῐστος;
    superlative: "~ῐστος";
    adverb: "~ον";
    ),
    // Μελᾱς: 4 words (μελᾱς, λευκομελᾱς, τᾰλᾱς, χλωρομελᾱς).
    declension!("μελᾱς", 4, "~ᾱς";
    nom_m: "~ᾱς", "~ᾰνε", "~ᾰνες";
    nom_f: "~αινᾰ", "~αινᾱ", "~αιναι";
    nom_n: "~ᾰν", _, "~ᾰνᾰ";
    gen_m: "~ᾰνος", "~ᾰνοιν", "~ᾰνων";
    gen_f: "~αινης", "~αιναιν", "~αινων";
    dat_m: "~ᾰνῐ", _, "~ᾰσῐ|~ᾰσῐν";
    dat_f: "~αινῃ", _, "~αιναις";
    acc_m: "~ᾰνᾰ", _, "~ᾰνᾰς";
    acc_f: "~αινᾰν", _, "~αινᾱς";
    voc_m: "~ᾰν", _, _;
    voc_f: "~αινᾰ", _, _;
    comparative: "~ᾰντερος";
    superlative: "~ᾰντᾰτος";
    adverb: "~ᾰνως";
    ),
    // Βαιος: 4 words (πᾰλαιος, βαιος, ἠβαιος, πᾰνημερῐος). Like Λᾰμπρος, except in 1 cell: adverb.
    declension!("βαιος", 4, "~ος", like Λᾰμπρος;
    adverb: "~ον";
    ),
    // Θηλῠς: 4 words (ἡμῐσῠς, θηλῠς, πρεσβῠς, ἀμφῐβρᾰχῠς). Like Τρᾱχῠς, except in 1 cell: nom_n pl.
    declension!("θηλῠς", 4, "~ῠς", like Τρᾱχῠς;
    nom_n: _, _, "~η|~εᾰ";
    ),
    // Μωλῠς: 4 words (νεκῠς, εὐϊχθῠς, πολῠϊχθῠς, μωλῠς). Like Τρᾱχῠς, except in 21 cells: nom_m du, nom_m pl, nom_f sg, nom_f du, nom_f pl, nom_n pl, ….
    declension!("μωλῠς", 4, "~ῠς", like Τρᾱχῠς;
    nom_m: _, "~ῠε|~ῡ", "~ῠες|~ῡς";
    nom_f: "~ῠς", "~ῠε|~ῡ", "~ῠες|~ῡς";
    nom_n: _, _, "~ῠᾰ";
    gen_m: "~ῠος", "~ῠοιν", "~ῠων";
    gen_f: "~ῠος", "~ῠοιν", "~ῠων";
    dat_m: "~ῠϊ|~υι", _, "~ῠσῐ|~ῠσῐν";
    dat_f: "~ῠϊ|~υι", _, "~ῠσῐ|~ῠσῐν";
    acc_m: _, _, "~ῡς|~ῠᾰς";
    acc_f: "~ῠν", _, "~ῡς|~ῠᾰς";
    voc_f: "~ῠ", _, _;
    adverb: "~ῠως";
    ),
    // Δους: 4 words (δους, βῐους, δῐδους, γνους). Like Πληγεις, except in 14 cells: nom_m sg, nom_f sg, nom_f du, nom_f pl, gen_f sg, gen_f du, ….
    declension!("δους", 4, "~υς", like Πληγεις;
    nom_m: "~υς", _, _;
    nom_f: "~υσᾰ", "~υσᾱ", "~υσαι";
    gen_f: "~υσης", "~υσαιν", "~υσων";
    dat_m: _, _, "~υσῐ|~υσῐν";
    dat_f: "~υσῃ", _, "~υσαις";
    acc_f: "~υσᾰν", _, "~υσᾱς";
    voc_m: "~υς", _, _;
    voc_f: "~υσᾰ", _, _;
    ),
    // Ἀπους: 4 words (πολυπους, ἀπους, χειμαρρους, δῐπους). Like Τρῐπους, except in 2 cells: acc_m sg, acc_f sg.
    declension!("ἀπους", 4, "~υς", like Τρῐπους;
    acc_m: "~υν", _, _;
    acc_f: "~υν", _, _;
    ),
    // Δεξῐος: 3 words (δεξῐος, ἀρῐστερος, φεριστος). Like Τελειος, except in 3 cells: comparative, superlative, adverb.
    declension!("δεξῐος", 3, "~ος", like Τελειος;
    comparative: derived;
    superlative: derived;
    adverb: "~ᾰ|~ως|~οθεν";
    ),
    // Ἀλλος: 3 words (ἀλλος, ταυτος, τηλῐκουτος). Like Ἀρῐστος, except in 2 cells: nom_n sg, adverb.
    declension!("ἀλλος", 3, "~ος", like Ἀρῐστος;
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
    // Μωνῠξ: 3 words (μωνῠξ, γᾰμψωνῠξ, κρᾰτερωνῠξ). Like Ὁμηλῐξ, except in 20 cells: nom_m du, nom_m pl, nom_f du, nom_f pl, nom_n pl, gen_m sg, ….
    declension!("μωνῠξ", 3, "~ξ", like Ὁμηλῐξ;
    nom_m: _, "~χε", "~χες";
    nom_f: _, "~χε", "~χες";
    nom_n: _, _, "~χᾰ";
    gen_m: "~χος", "~χοιν", "~χων";
    gen_f: "~χος", "~χοιν", "~χων";
    dat_m: "~χῐ", _, _;
    dat_f: "~χῐ", _, _;
    acc_m: "~χᾰ", _, "~χᾰς";
    acc_f: "~χᾰ", _, "~χᾰς";
    comparative: derived;
    superlative: derived;
    adverb: "~χως";
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
    // Ὁλοκληρος: 1 words (ὁλοκληρος).
    declension!("ὁλοκληρος", 1, "~κ~~ος";
    nom_m: "~~κ~ος", "~~κ~ω", "~~κ~οι";
    nom_f: "~~κ~ος", "~~κ~ω", "~~κ~οι";
    nom_n: "~~κ~ον", _, "~~κ~ᾰ";
    gen_m: "~~κ~ου", "~~κ~οιν", "~~κ~ων";
    gen_f: "~~κ~ου", "~~κ~οιν", "~~κ~ων";
    dat_m: "~~κ~ῳ", _, "~~κ~οις";
    dat_f: "~~κ~ῳ", _, "~~κ~οις";
    acc_m: "~~κ~ον", _, "~~κ~ους";
    acc_f: "~~κ~ον", _, "~~κ~ους";
    voc_m: "~~κ~ε", _, _;
    voc_f: "~~κ~ε", _, _;
    comparative: "~~κ~οτερος";
    superlative: "~~κ~οτᾰτος";
    adverb: "~~κ~ως";
    ),
    // Δνοφερος: 1 words (δνοφερος).
    declension!("δνοφερος", 1, "~ε~ος";
    nom_m: "~η~ος", "~η~ω", "~η~οι";
    nom_f: "~η~ᾱ", "~η~ᾱ", "~η~αι";
    nom_n: "~η~ον", _, "~η~ᾰ";
    gen_m: "~η~ου", "~η~οιν", "~η~ων";
    gen_f: "~η~ᾱς", "~η~αιν", "~η~ων";
    dat_m: "~η~ῳ", _, "~η~οις";
    dat_f: "~η~ᾳ", _, "~η~αις";
    acc_m: "~η~ον", _, "~η~ους";
    acc_f: "~η~ᾱν", _, "~η~ᾱς";
    voc_m: "~η~ε", _, _;
    voc_f: "~η~ᾱ", _, _;
    comparative: "~η~οτερος";
    superlative: "~η~οτᾰτος";
    adverb: "~η~ως";
    ),
    // Γενομενω: 1 words (γενομενω). Like Ἀρῐστος, except in 1 cell: adverb.
    declension!("γενομενω", 1, "~ω", like Ἀρῐστος;
    adverb: derived;
    ),
    // Γενομενοιν: 1 words (γενομενοιν). Like Γενομενω.
    declension!("γενομενοιν", 1, "~οιν", like Γενομενω;
    ),
    // Γενομεναιν: 1 words (γενομεναιν). Like Γενομενω.
    declension!("γενομεναιν", 1, "~αιν", like Γενομενω;
    ),
    // Μελᾰνθρῐξ: 1 words (μελᾰνθρῐξ).
    declension!("μελᾰνθρῐξ", 1, "~θ~ξ";
    nom_m: "~οθ~ξ", "~οτ~χε", "~οτ~χες";
    nom_f: "~οθ~ξ", "~οτ~χε", "~οτ~χες";
    nom_n: "~οθ~ξ", _, "~οτ~χᾰ";
    gen_m: "~οτ~χος", "~οτ~χοιν", "~οτ~χων";
    gen_f: "~οτ~χος", "~οτ~χοιν", "~οτ~χων";
    dat_m: "~οτ~χῐ", _, "~οθ~ξῐ|~οθ~ξῐν";
    dat_f: "~οτ~χῐ", _, "~οθ~ξῐ|~οθ~ξῐν";
    acc_m: "~οτ~χᾰ", _, "~οτ~χᾰς";
    acc_f: "~οτ~χᾰ", _, "~οτ~χᾰς";
    voc_m: "~οθ~ξ", _, _;
    voc_f: "~οθ~ξ", _, _;
    adverb: "~οτ~χως";
    ),
    // Ὑπαρχομενη: 1 words (ὑπαρχομενη).
    declension!("ὑπαρχομενη", 1, "~α~η";
    nom_m: "~ᾰ~ος", "~ᾰ~ω", "~ᾰ~οι";
    nom_f: "~ᾰ~η", "~ᾰ~ᾱ", "~ᾰ~αι";
    nom_n: "~ᾰ~ον", _, "~ᾰ~ᾰ";
    gen_m: "~ᾰ~ου", "~ᾰ~οιν", "~ᾰ~ων";
    gen_f: "~ᾰ~ης", "~ᾰ~αιν", "~ᾰ~ων";
    dat_m: "~ᾰ~ῳ", _, "~ᾰ~οις";
    dat_f: "~ᾰ~ῃ", _, "~ᾰ~αις";
    acc_m: "~ᾰ~ον", _, "~ᾰ~ους";
    acc_f: "~ᾰ~ην", _, "~ᾰ~ᾱς";
    voc_m: "~ᾰ~ε", _, _;
    voc_f: "~ᾰ~η", _, _;
    comparative: "~ᾰ~ωτερος";
    superlative: "~ᾰ~ωτᾰτος";
    adverb: "~ᾰ~ως";
    ),
    // Ὑπαρχον: 1 words (ὑπαρχον).
    declension!("ὑπαρχον", 1, "~α~~";
    nom_m: "~ᾰ~με~ος", "~ᾰ~με~ω", "~ᾰ~με~οι";
    nom_f: "~ᾰ~με~η", "~ᾰ~με~ᾱ", "~ᾰ~με~αι";
    nom_n: "~ᾰ~με~ον", _, "~ᾰ~με~ᾰ";
    gen_m: "~ᾰ~με~ου", "~ᾰ~με~οιν", "~ᾰ~με~ων";
    gen_f: "~ᾰ~με~ης", "~ᾰ~με~αιν", "~ᾰ~με~ων";
    dat_m: "~ᾰ~με~ῳ", _, "~ᾰ~με~οις";
    dat_f: "~ᾰ~με~ῃ", _, "~ᾰ~με~αις";
    acc_m: "~ᾰ~με~ον", _, "~ᾰ~με~ους";
    acc_f: "~ᾰ~με~ην", _, "~ᾰ~με~ᾱς";
    voc_m: "~ᾰ~με~ε", _, _;
    voc_f: "~ᾰ~με~η", _, _;
    comparative: "~ᾰ~με~ωτερος";
    superlative: "~ᾰ~με~ωτᾰτος";
    adverb: "~ᾰ~με~ως";
    ),
    // Ὑπαρχομενον: 1 words (ὑπαρχομενον). Like Ὑπαρχομενη.
    declension!("ὑπαρχομενον", 1, "~α~ον", like Ὑπαρχομενη;
    ),
    // Ψησας: 1 words (ψησας).
    declension!("ψησας", 1, "~ας";
    nom_m: "ἑ~ᾱς", "ἑ~ᾰντε", "ἑ~ᾰντες";
    nom_f: "ἑ~ᾱσᾰ", "ἑ~ᾱσᾱ", "ἑ~ᾱσαι";
    nom_n: "ἑ~ᾰν", _, "ἑ~ᾰντᾰ";
    gen_m: "ἑ~ᾰντος", "ἑ~ᾰντοιν", "ἑ~ᾰντων";
    gen_f: "ἑ~ᾱσης", "ἑ~ᾱσαιν", "ἑ~ᾱσων";
    dat_m: "ἑ~ᾰντῐ", _, "ἑ~ᾱσῐ|ἑ~ᾱσῐν";
    dat_f: "ἑ~ᾱσῃ", _, "ἑ~ᾱσαις";
    acc_m: "ἑ~ᾰντᾰ", _, "ἑ~ᾰντᾰς";
    acc_f: "ἑ~ᾱσᾰν", _, "ἑ~ᾱσᾱς";
    voc_m: "ἑ~ᾱς", _, _;
    voc_f: "ἑ~ᾱσᾰ", _, _;
    adverb: "ἑ~ᾰντως";
    ),
    // Χερηα: 1 words (χερηα). Like Γενομενω, except in 24 cells: nom_m sg, nom_m du, nom_m pl, nom_f sg, nom_f du, nom_f pl, ….
    declension!("χερηα", 1, "~ηα", like Γενομενω;
    nom_m: "~ηα", derived, "~ηες";
    nom_f: derived, derived, "~ηες";
    nom_n: derived, _, "~ηα|~εια";
    gen_m: derived, derived, derived;
    gen_f: derived, derived, derived;
    dat_m: "~ηϊ|~ηϊ", _, derived;
    dat_f: "~ηϊ|~ηϊ", _, derived;
    acc_m: "~ηα|~εια", _, derived;
    acc_f: "~ηα|~εια", _, derived;
    voc_m: derived, _, _;
    voc_f: derived, _, _;
    ),
    // Ἀτηρος: 1 words (ἀτηρος).
    declension!("ἀτηρος", 1, "~η~ος";
    nom_m: "~~εστος", "~~εστω", "~~εστοι";
    nom_f: "~~εστᾱ", "~~εστᾱ", "~~εσται";
    nom_n: "~~εστον", _, "~~εστᾰ";
    gen_m: "~~εστου", "~~εστοιν", "~~εστων";
    gen_f: "~~εστᾱς", "~~εσταιν", "~~εστων";
    dat_m: "~~εστῳ", _, "~~εστοις";
    dat_f: "~~εστᾳ", _, "~~εσταις";
    acc_m: "~~εστον", _, "~~εστους";
    acc_f: "~~εστᾱν", _, "~~εστᾱς";
    voc_m: "~~εστε", _, _;
    voc_f: "~~εστᾱ", _, _;
    comparative: "~~εστοτερος";
    superlative: "~~εστοτᾰτος";
    adverb: "~~εστως";
    ),
    // Ἀειμακαριστος: 1 words (ἀειμακαριστος).
    declension!("ἀειμακαριστος", 1, "ἀ~α~α~ι~ος";
    nom_m: "ᾰ̓~ᾰ~ᾰ~ῐ~ος", "ᾰ̓~ᾰ~ᾰ~ῐ~ω", "ᾰ̓~ᾰ~ᾰ~ῐ~οι";
    nom_f: "ᾰ̓~ᾰ~ᾰ~ῐ~η", "ᾰ̓~ᾰ~ᾰ~ῐ~ᾱ", "ᾰ̓~ᾰ~ᾰ~ῐ~αι";
    nom_n: "ᾰ̓~ᾰ~ᾰ~ῐ~ον", _, "ᾰ̓~ᾰ~ᾰ~ῐ~ᾰ";
    gen_m: "ᾰ̓~ᾰ~ᾰ~ῐ~ου", "ᾰ̓~ᾰ~ᾰ~ῐ~οιν", "ᾰ̓~ᾰ~ᾰ~ῐ~ων";
    gen_f: "ᾰ̓~ᾰ~ᾰ~ῐ~ης", "ᾰ̓~ᾰ~ᾰ~ῐ~αιν", "ᾰ̓~ᾰ~ᾰ~ῐ~ων";
    dat_m: "ᾰ̓~ᾰ~ᾰ~ῐ~ῳ", _, "ᾰ̓~ᾰ~ᾰ~ῐ~οις";
    dat_f: "ᾰ̓~ᾰ~ᾰ~ῐ~ῃ", _, "ᾰ̓~ᾰ~ᾰ~ῐ~αις";
    acc_m: "ᾰ̓~ᾰ~ᾰ~ῐ~ον", _, "ᾰ̓~ᾰ~ᾰ~ῐ~ους";
    acc_f: "ᾰ̓~ᾰ~ᾰ~ῐ~ην", _, "ᾰ̓~ᾰ~ᾰ~ῐ~ᾱς";
    voc_m: "ᾰ̓~ᾰ~ᾰ~ῐ~ε", _, _;
    voc_f: "ᾰ̓~ᾰ~ᾰ~ῐ~η", _, _;
    comparative: "ᾰ̓~ᾰ~ᾰ~ῐ~οτερος";
    superlative: "ᾰ̓~ᾰ~ᾰ~ῐ~οτᾰτος";
    adverb: "ᾰ̓~ᾰ~ᾰ~ῐ~ως";
    ),
];
