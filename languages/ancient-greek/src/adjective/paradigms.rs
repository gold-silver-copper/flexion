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
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident)?;
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

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `ἐμπεδος`: 888 words.
    Ἐμπεδος,
    /// Like `ἰσος`: 772 words.
    Ἰσος,
    /// Like `μᾰλᾰκος`: 728 words.
    Μᾰλᾰκος,
    /// Like `ἀτελης`: 267 words.
    Ἀτελης,
    /// Like `τελειος`: 250 words.
    Τελειος,
    /// Like `προσηκων`: 190 words.
    Προσηκων,
    /// Like `προπᾱς`: 140 words.
    Προπᾱς,
    /// Like `ὑστερος`: 127 words.
    Ὑστερος,
    /// Like `ἀσημος`: 126 words.
    Ἀσημος,
    /// Like `λᾰμπρος`: 123 words.
    Λᾰμπρος,
    /// Like `ψῑλος`: 112 words.
    Ψῑλος,
    /// Like `ἀρῐστος`: 112 words.
    Ἀρῐστος,
    /// Like `πληγεις`: 90 words.
    Πληγεις,
    /// Like `ἰων`: 78 words.
    Ἰων,
    /// Like `οἰκεῑος`: 77 words.
    Οἰκεῑος,
    /// Like `ἀρκων`: 58 words.
    Ἀρκων,
    /// Like `μαυρος`: 49 words.
    Μαυρος,
    /// Like `εἰκως`: 46 words.
    Εἰκως,
    /// Like `πλειων`: 40 words.
    Πλειων,
    /// Like `ζων`: 33 words.
    Ζων,
    /// Like `φᾰνερος`: 31 words.
    Φᾰνερος,
    /// Like `φωνηεις`: 30 words.
    Φωνηεις,
    /// Like `τρᾱχῠς`: 24 words.
    Τρᾱχῠς,
    /// Like `βαινων`: 23 words.
    Βαινων,
    /// Like `ἡσσων`: 20 words.
    Ἡσσων,
    /// Like `κᾰκοηθης`: 19 words.
    Κᾰκοηθης,
    /// Like `παυων`: 19 words.
    Παυων,
    /// Like `δειξᾱς`: 18 words.
    Δειξᾱς,
    /// Like `τοιοσδε`: 12 words.
    Τοιοσδε,
    /// Like `δῐκαιος`: 11 words.
    Δῐκαιος,
    /// Like `φαῡλος`: 11 words.
    Φαῡλος,
    /// Like `ἀγνως`: 11 words.
    Ἀγνως,
    /// Like `γαῡρος`: 10 words.
    Γαῡρος,
    /// Like `ζευξᾱς`: 10 words.
    Ζευξᾱς,
    /// Like `ἐρημος`: 9 words.
    Ἐρημος,
    /// Like `θειων`: 9 words.
    Θειων,
    /// Like `κᾰλλῐθρῐξ`: 9 words.
    Κᾰλλῐθρῐξ,
    /// Like `παυσων`: 9 words.
    Παυσων,
    /// Like `κολοβος`: 8 words.
    Κολοβος,
    /// Like `τρῐπους`: 8 words.
    Τρῐπους,
    /// Like `εὐελπῐς`: 7 words.
    Εὐελπῐς,
    /// Like `ἀξῐοχρεως`: 7 words.
    Ἀξῐοχρεως,
    /// Like `ἀπᾰτωρ`: 6 words.
    Ἀπᾰτωρ,
    /// Like `φῡς`: 6 words.
    Φῡς,
    /// Like `ἁπλοῡς`: 6 words.
    Ἁπλοῡς,
    /// Like `αἰθοψ`: 5 words.
    Αἰθοψ,
    /// Like `πλεῑστος`: 4 words.
    Πλεῑστος,
    /// Like `μελᾱς`: 4 words.
    Μελᾱς,
    /// Like `θηλῠς`: 4 words.
    Θηλῠς,
    /// Like `ἀπους`: 4 words.
    Ἀπους,
    /// Like `χειρων`: 4 words.
    Χειρων,
    /// Like `μωλῠς`: 4 words.
    Μωλῠς,
    /// Like `δους`: 4 words.
    Δους,
    /// Like `πεπτεως`: 4 words.
    Πεπτεως,
    /// Like `βαιος`: 3 words.
    Βαιος,
    /// Like `δῐᾱνταῑος`: 3 words.
    Δῐᾱνταῑος,
    /// Like `ἀντῐπαις`: 3 words.
    Ἀντῐπαις,
    /// Like `μωνῠξ`: 3 words.
    Μωνῠξ,
    /// Like `νηποινος`: 3 words.
    Νηποινος,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 59] = [
        Declension::Ἐμπεδος,
        Declension::Ἰσος,
        Declension::Μᾰλᾰκος,
        Declension::Ἀτελης,
        Declension::Τελειος,
        Declension::Προσηκων,
        Declension::Προπᾱς,
        Declension::Ὑστερος,
        Declension::Ἀσημος,
        Declension::Λᾰμπρος,
        Declension::Ψῑλος,
        Declension::Ἀρῐστος,
        Declension::Πληγεις,
        Declension::Ἰων,
        Declension::Οἰκεῑος,
        Declension::Ἀρκων,
        Declension::Μαυρος,
        Declension::Εἰκως,
        Declension::Πλειων,
        Declension::Ζων,
        Declension::Φᾰνερος,
        Declension::Φωνηεις,
        Declension::Τρᾱχῠς,
        Declension::Βαινων,
        Declension::Ἡσσων,
        Declension::Κᾰκοηθης,
        Declension::Παυων,
        Declension::Δειξᾱς,
        Declension::Τοιοσδε,
        Declension::Δῐκαιος,
        Declension::Φαῡλος,
        Declension::Ἀγνως,
        Declension::Γαῡρος,
        Declension::Ζευξᾱς,
        Declension::Ἐρημος,
        Declension::Θειων,
        Declension::Κᾰλλῐθρῐξ,
        Declension::Παυσων,
        Declension::Κολοβος,
        Declension::Τρῐπους,
        Declension::Εὐελπῐς,
        Declension::Ἀξῐοχρεως,
        Declension::Ἀπᾰτωρ,
        Declension::Φῡς,
        Declension::Ἁπλοῡς,
        Declension::Αἰθοψ,
        Declension::Πλεῑστος,
        Declension::Μελᾱς,
        Declension::Θηλῠς,
        Declension::Ἀπους,
        Declension::Χειρων,
        Declension::Μωλῠς,
        Declension::Δους,
        Declension::Πεπτεως,
        Declension::Βαιος,
        Declension::Δῐᾱνταῑος,
        Declension::Ἀντῐπαις,
        Declension::Μωνῠξ,
        Declension::Νηποινος,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 59] = [
    // Ἐμπεδος: 888 words (δῐᾰβολος, βᾰρβᾰρος, ἀνᾰπολογητος, πρωτοτοκος, μᾰγος, ἀκᾰθᾰρτος).
    declension!("ἐμπεδος", 888, "~ος";
    nom_m: "~ος", "~ω", "~οι";
    nom_f: "~ος", "~ω", "~οι";
    nom_n: "~ον", _, "~ᾰ";
    gen_m: "~ου", "~οιν", "~ων";
    gen_f: "~ου", "~οιν", "~ων";
    dat_m: "~ῳ", _, "~οις";
    dat_f: "~ῳ", _, "~οις";
    acc_m: "~ον", _, "~ους";
    acc_f: "~ον", _, "~ους";
    voc_m: "~ε", _, _;
    voc_f: "~ε", _, _;
    comparative: "~ωτερος";
    superlative: "~ωτᾰτος";
    adverb: "~ως";
    ),
    // Ἰσος: 772 words (ἐρωμενος, ὀλῐγος, ὁλος, ἐσχᾰτος, ἀλλος, ξενος). Like Ἐμπεδος, except in 10 cells: nom_f sg, nom_f du, nom_f pl, gen_f sg, gen_f du, dat_f sg, ….
    declension!("ἰσος", 772, "~ος", like Ἐμπεδος;
    nom_f: "~η", "~ᾱ", "~αι";
    gen_f: "~ης", "~αιν", _;
    dat_f: "~ῃ", _, "~αις";
    acc_f: "~ην", _, "~ᾱς";
    voc_f: "~η", _, _;
    ),
    // Μᾰλᾰκος: 728 words (παιδῐκος, γᾰλᾰκτῐκος, λευκος, πνευματικος, ἀγᾰθος, κρῠπτος). Like Ἰσος, except in 5 cells: gen_m sg, gen_m du, gen_f du, dat_m pl, dat_f pl.
    declension!("μᾰλᾰκος", 728, "~ος", like Ἰσος;
    gen_m: "~οῡ", "~οῑν", _;
    gen_f: _, "~αῑν", _;
    dat_m: _, _, "~οῑς";
    dat_f: _, _, "~αῑς";
    ),
    // Ἀτελης: 267 words (ὑγῐης, ἀβαρης, ψευδης, ἀτελης, δηλητηρῐωδης, σῑγμοειδης). Like Μᾰλᾰκος, except in 24 cells: nom_m sg, nom_m du, nom_m pl, nom_f sg, nom_f du, nom_f pl, ….
    declension!("ἀτελης", 267, "~ης", like Μᾰλᾰκος;
    nom_m: "~ης", "~εῑ", "~εῑς";
    nom_f: "~ης", "~εῑ", "~εῑς";
    nom_n: "~ες", _, "~η";
    gen_m: "~οῡς", _, _;
    gen_f: "~οῡς", "~οῑν", _;
    dat_m: "~εῑ", _, "~εσῐ|~εσῐν";
    dat_f: "~εῑ", _, "~εσῐ|~εσῐν";
    dat_n: _, _, "~εσῐ|~εσῐν";
    acc_m: "~η", _, "~εῑς";
    acc_f: "~η", _, "~εῑς";
    voc_m: "~ες", _, _;
    voc_f: "~ες", _, _;
    comparative: "~εστερος";
    superlative: "~εστᾰτος";
    ),
    // Τελειος: 250 words (κῡρῐος, ἁγῐος, νεος, ἑτερος, ὁμοιος, ἀγρῐος). Like Ἰσος, except in 5 cells: nom_f sg, gen_f sg, dat_f sg, acc_f sg, voc_f sg.
    declension!("τελειος", 250, "~ος", like Ἰσος;
    nom_f: "~ᾱ", _, _;
    gen_f: "~ᾱς", _, _;
    dat_f: "~ᾳ", _, _;
    acc_f: "~ᾱν", _, _;
    voc_f: "~ᾱ", _, _;
    ),
    // Προσηκων: 190 words (κῠων, μελλων, ὀζων, νεων, γερων, ἀρχων).
    declension!("προσηκων", 190, "~ων";
    nom_m: "~ων", "~οντε", "~οντες";
    nom_f: "~ουσᾰ", "~ουσᾱ", "~ουσαι";
    nom_n: "~ον", _, "~οντᾰ";
    gen_m: "~οντος", "~οντοιν", "~οντων";
    gen_f: "~ουσης", "~ουσαιν", "~ουσων";
    dat_m: "~οντῐ", _, "~ουσῐ|~ουσῐν";
    dat_f: "~ουσῃ", _, "~ουσαις";
    dat_n: _, _, "~ουσῐ|~ουσῐν";
    acc_m: "~οντᾰ", _, "~οντᾰς";
    acc_f: "~ουσᾰν", _, "~ουσᾱς";
    voc_m: "~ων", _, _;
    voc_f: "~ουσᾰ", _, _;
    adverb: "~οντως";
    ),
    // Προπᾱς: 140 words (ἀδᾰμᾱς, λῡσᾱς, ἱστᾱς, γῐγᾱς, βῐβᾱς, φωνησᾱς).
    declension!("προπᾱς", 140, "~ᾱς";
    nom_m: "~ᾱς", "~ᾰντε", "~ᾰντες";
    nom_f: "~ᾱσᾰ", "~ᾱσᾱ", "~ᾱσαι";
    nom_n: "~ᾰν", _, "~ᾰντᾰ";
    gen_m: "~ᾰντος", "~ᾰντοιν", "~ᾰντων";
    gen_f: "~ᾱσης", "~ᾱσαιν", "~ᾱσων";
    dat_m: "~ᾰντῐ", _, "~ᾱσῐ|~ᾱσῐν";
    dat_f: "~ᾱσῃ", _, "~ᾱσαις";
    dat_n: _, _, "~ᾱσῐ|~ᾱσῐν";
    acc_m: "~ᾰντᾰ", _, "~ᾰντᾰς";
    acc_f: "~ᾱσᾰν", _, "~ᾱσᾱς";
    voc_m: "~ᾱς", _, _;
    voc_f: "~ᾱσᾰ", _, _;
    adverb: "~ᾰντως";
    ),
    // Ὑστερος: 127 words (ἰδῐωτερος, προτερος, βᾰθῠτερος, πρεσβῠτερος, βεβαιοτερος, ὑστερος).
    declension!("ὑστερος", 127, "~ερος";
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
    // Ἀσημος: 126 words (ἀκᾰρπος, πᾰρᾰκλητος, ἀβυσσος, πρᾱος, ἐπικουρος, χερσος). Like Ἐμπεδος, except in 2 cells: comparative, superlative.
    declension!("ἀσημος", 126, "~ος", like Ἐμπεδος;
    comparative: "~οτερος";
    superlative: "~οτᾰτος";
    ),
    // Λᾰμπρος: 123 words (μῑκρος, ἐρῠθρος, χλωρος, νεκρος, δεξῐος, ἀρῐστερος). Like Μᾰλᾰκος, except in 7 cells: nom_f sg, gen_f sg, dat_f sg, acc_f sg, voc_f sg, comparative, ….
    declension!("λᾰμπρος", 123, "~ος", like Μᾰλᾰκος;
    nom_f: "~ᾱ", _, _;
    gen_f: "~ᾱς", _, _;
    dat_f: "~ᾳ", _, _;
    acc_f: "~ᾱν", _, _;
    voc_f: "~ᾱ", _, _;
    comparative: "~οτερος";
    superlative: "~οτᾰτος";
    ),
    // Ψῑλος: 112 words (ἀδελφος, ἰουδαϊκος, ξᾰνθος, βᾰκτρῐᾱνος, ἁγνος, ὀρθος). Like Μᾰλᾰκος, except in 2 cells: comparative, superlative.
    declension!("ψῑλος", 112, "~ος", like Μᾰλᾰκος;
    comparative: "~οτερος";
    superlative: "~οτᾰτος";
    ),
    // Ἀρῐστος: 112 words (ἰδῐωτᾰτος, μεγῐστος, ἀρῐστος, ἡκῐστος, στοιχειωδεστᾰτος, κᾰκῐστος). Like Ἰσος, except in 3 cells: comparative, superlative, adverb.
    declension!("ἀρῐστος", 112, "~ος", like Ἰσος;
    comparative: derived;
    superlative: derived;
    adverb: "~ᾰ";
    ),
    // Πληγεις: 90 words (σῠγγρᾰφεις, θεις, κοιμηθεις, νῠγεις, αὐδηθεις, κᾰλῠφεις).
    declension!("πληγεις", 90, "~ις";
    nom_m: "~ις", "~ντε", "~ντες";
    nom_f: "~ῑσᾰ", "~ισᾱ", "~ῑσαι";
    nom_n: "~ν", _, "~ντᾰ";
    gen_m: "~ντος", "~ντοιν", "~ντων";
    gen_f: "~ισης", "~ισαιν", "~ισων";
    dat_m: "~ντῐ", _, "~ῑσῐ|~ῑσῐν";
    dat_f: "~ισῃ", _, "~ισαις";
    dat_n: _, _, "~ῑσῐ|~ῑσῐν";
    acc_m: "~ντᾰ", _, "~ντᾰς";
    acc_f: "~ῑσᾰν", _, "~ισᾱς";
    voc_m: "~ις", _, _;
    voc_f: "~ῑσᾰ", _, _;
    adverb: "~ντως";
    ),
    // Ἰων: 78 words (πᾰρων, ἀπων, λᾰβων, πᾰρελθων, ἑκων, εἰπων). Like Προσηκων, except in 6 cells: nom_f sg, nom_f pl, dat_m pl, dat_n pl, acc_f sg, voc_f sg.
    declension!("ἰων", 78, "~ων", like Προσηκων;
    nom_f: "~οῡσᾰ", _, "~οῡσαι";
    dat_m: _, _, "~οῡσῐ|~οῡσῐν";
    dat_n: _, _, "~οῡσῐ|~οῡσῐν";
    acc_f: "~οῡσᾰν", _, _;
    voc_f: "~οῡσᾰ", _, _;
    ),
    // Οἰκεῑος: 77 words (ἀρχαῑος, ῥωμαῑος, ἀνδρεῑος, ἑβραῑος, ἀργεῑος, θηβαῑος).
    declension!("οἰκεῑος", 77, "~ῑος";
    nom_m: "~ῑος", "~ιω", "~ῑοι";
    nom_f: "~ιᾱ", "~ιᾱ", "~ῑαι";
    nom_n: "~ῑον", _, "~ῑᾰ";
    gen_m: "~ιου", "~ιοιν", "~ιων";
    gen_f: "~ιᾱς", "~ιαιν", "~ιων";
    dat_m: "~ιῳ", _, "~ιοις";
    dat_f: "~ιᾳ", _, "~ιαις";
    acc_m: "~ῑον", _, "~ιους";
    acc_f: "~ιᾱν", _, "~ιᾱς";
    voc_m: "~ῑε", _, _;
    voc_f: "~ιᾱ", _, _;
    comparative: "~ιοτερος";
    superlative: "~ιοτᾰτος";
    adverb: "~ιως";
    ),
    // Ἀρκων: 58 words (ποιων, κυκλων, σημειων, βῐων, δηλων, γονῠπετων). Like Ἰων, except in 11 cells: nom_m du, nom_m pl, nom_n sg, nom_n pl, gen_m sg, gen_m du, ….
    declension!("ἀρκων", 58, "~ων", like Ἰων;
    nom_m: _, "~οῡντε", "~οῡντες";
    nom_n: "~οῡν", _, "~οῡντᾰ";
    gen_m: "~οῡντος", "~ουντοιν", "~ουντων";
    dat_m: "~οῡντῐ", _, _;
    acc_m: "~οῡντᾰ", _, "~οῡντᾰς";
    adverb: "~ουντως";
    ),
    // Μαυρος: 49 words (μαυρος, σῠνεργος, ἀγωγος, γεωργος, σῠνᾰπτος, ἐνεργος). Like Ἀσημος, except in 6 cells: gen_m sg, gen_m du, gen_f sg, gen_f du, dat_m pl, dat_f pl.
    declension!("μαυρος", 49, "~ος", like Ἀσημος;
    gen_m: "~οῡ", "~οῑν", _;
    gen_f: "~οῡ", "~οῑν", _;
    dat_m: _, _, "~οῑς";
    dat_f: _, _, "~οῑς";
    ),
    // Εἰκως: 46 words (λελῠκως, κεχηνως, βεβηκως, εἰδως, πεπαιδευκως, ἠγᾰπηκως).
    declension!("εἰκως", 46, "~ως";
    nom_m: "~ως", "~οτε", "~οτες";
    nom_f: "~υῑᾰ", "~υιᾱ", "~υῑαι";
    nom_n: "~ος", _, "~οτᾰ";
    gen_m: "~οτος", "~οτοιν", "~οτων";
    gen_f: "~υιᾱς", "~υιαιν", "~υιων";
    dat_m: "~οτῐ", _, "~οσῐ|~οσῐν";
    dat_f: "~υιᾳ", _, "~υιαις";
    dat_n: _, _, "~οσῐ|~οσῐν";
    acc_m: "~οτᾰ", _, "~οτᾰς";
    acc_f: "~υῑᾰν", _, "~υιᾱς";
    voc_m: "~ως", _, _;
    voc_f: "~υῑᾰ", _, _;
    adverb: "~οτως";
    ),
    // Πλειων: 40 words (σωφρων, ἀκμων, βελτῑων, κᾰκοδαιμων, μειων, ὑπερφρων).
    declension!("πλειων", 40, "~ων";
    nom_m: "~ων", "~ονε", "~ονες";
    nom_f: "~ων", "~ονε", "~ονες";
    nom_n: "~ον", _, "~ονᾰ";
    gen_m: "~ονος", "~ονοιν", "~ονων";
    gen_f: "~ονος", "~ονοιν", "~ονων";
    dat_m: "~ονῐ", _, "~οσῐ|~οσῐν";
    dat_f: "~ονῐ", _, "~οσῐ|~οσῐν";
    dat_n: _, _, "~οσῐ|~οσῐν";
    acc_m: "~ονᾰ", _, "~ονᾰς";
    acc_f: "~ονᾰ", _, "~ονᾰς";
    voc_m: "~ον", _, _;
    voc_f: "~ον", _, _;
    comparative: "~ονεστερος";
    superlative: "~ονεστᾰτος";
    adverb: "~ονως";
    ),
    // Ζων: 33 words (ὠν, νῑκων, τῑμων, δῐαιτων, θεων, αὐδων). Like Πληγεις, except in 15 cells: nom_m sg, nom_f sg, nom_f du, nom_f pl, gen_f sg, gen_f du, ….
    declension!("ζων", 33, "~ν", like Πληγεις;
    nom_m: "~ν", _, _;
    nom_f: "~σᾰ", "~σᾱ", "~σαι";
    gen_f: "~σης", "~σαιν", "~σων";
    dat_m: _, _, "~σῐ|~σῐν";
    dat_f: "~σῃ", _, "~σαις";
    dat_n: _, _, "~σῐ|~σῐν";
    acc_f: "~σᾰν", _, "~σᾱς";
    voc_m: "~ν", _, _;
    voc_f: "~σᾰ", _, _;
    ),
    // Φᾰνερος: 31 words (κᾰθᾰρος, στερεος, σᾰρωνῐκος, κερᾰος, ἰος, ἱλᾰρος). Like Λᾰμπρος, except in 2 cells: comparative, superlative.
    declension!("φᾰνερος", 31, "~ος", like Λᾰμπρος;
    comparative: "~ωτερος";
    superlative: "~ωτᾰτος";
    ),
    // Φωνηεις: 30 words (ἀργῐνοεις, χᾰρῐεις, φωνηεις, αἱμᾰτοεις, ῥοδοεις, ἰχθυοεις). Like Πληγεις, except in 16 cells: nom_f sg, nom_f du, nom_f pl, gen_f sg, gen_f du, gen_f pl, ….
    declension!("φωνηεις", 30, "~ις", like Πληγεις;
    nom_f: "~σσᾰ", "~σσᾱ", "~σσαι";
    gen_f: "~σσης", "~σσαιν", "~σσων";
    dat_m: _, _, "~ισῐ|~ισῐν";
    dat_f: "~σσῃ", _, "~σσαις";
    dat_n: _, _, "~ισῐ|~ισῐν";
    acc_f: "~σσᾰν", _, "~σσᾱς";
    voc_m: "~ν", _, _;
    voc_f: "~σσᾰ", _, _;
    comparative: "~στερος";
    superlative: "~στᾰτος";
    ),
    // Τρᾱχῠς: 24 words (τᾰχῠς, βρᾰδῠς, πολῠς, βᾰθῠς, ὀξῠς, ὠκῠς). Like Ἀτελης, except in 22 cells: nom_m sg, nom_f sg, nom_f du, nom_f pl, nom_n sg, nom_n pl, ….
    declension!("τρᾱχῠς", 24, "~ῠς", like Ἀτελης;
    nom_m: "~ῠς", _, _;
    nom_f: "~εῑᾰ", "~ειᾱ", "~εῑαι";
    nom_n: "~ῠ", _, "~εᾰ";
    gen_m: "~εος", "~εοιν", "~εων";
    gen_f: "~ειᾱς", "~ειαιν", "~ειων";
    dat_f: "~ειᾳ", _, "~ειαις";
    acc_m: "~ῠν", _, _;
    acc_f: "~εῑᾰν", _, "~ειᾱς";
    voc_m: "~ῠ", _, _;
    voc_f: "~εῑᾰ", _, _;
    comparative: "~ῠτερος";
    superlative: "~ῠτᾰτος";
    adverb: "~εως";
    ),
    // Βαινων: 23 words (φαινων, ἀγειρων, μενεαινων, χαιρων, λειβων, ἀπολειπων).
    declension!("βαινων", 23, "~ι~ων";
    nom_m: "~ι~ων", "~ι~οντε", "~ι~οντες";
    nom_f: "~ι~ουσᾰ", "~ι~ουσᾱ", "~ι~ουσαι";
    nom_n: "~ῑ~ον", _, "~ι~οντᾰ";
    gen_m: "~ι~οντος", "~ι~οντοιν", "~ι~οντων";
    gen_f: "~ι~ουσης", "~ι~ουσαιν", "~ι~ουσων";
    dat_m: "~ι~οντῐ", _, "~ι~ουσῐ|~ι~ουσῐν";
    dat_f: "~ι~ουσῃ", _, "~ι~ουσαις";
    dat_n: _, _, "~ι~ουσῐ|~ι~ουσῐν";
    acc_m: "~ι~οντᾰ", _, "~ι~οντᾰς";
    acc_f: "~ι~ουσᾰν", _, "~ι~ουσᾱς";
    voc_m: "~ι~ων", _, _;
    voc_f: "~ι~ουσᾰ", _, _;
    adverb: "~ι~οντως";
    ),
    // Ἡσσων: 20 words (βρᾰχῑων, μᾰσσων, ἐλᾰσσων, κερδιων, ἐλαττων, θᾱττων). Like Πλειων, except in 14 cells: nom_m pl, nom_f pl, nom_n pl, acc_m sg, acc_m pl, acc_f sg, ….
    declension!("ἡσσων", 20, "~ων", like Πλειων;
    nom_m: _, _, "~ονες|~ους";
    nom_f: _, _, "~ονες|~ους";
    nom_n: _, _, "~ονᾰ|~ω";
    acc_m: "~ονᾰ|~ω", _, "~ονᾰς|~ους";
    acc_f: "~ονᾰ|~ω", _, "~ονᾰς|~ους";
    acc_n: _, _, "~ονᾰ|~ω";
    voc_m: _, _, "~ονες|~ους";
    voc_f: _, _, "~ονες|~ους";
    voc_n: _, _, "~ονᾰ|~ω";
    comparative: derived;
    superlative: "~στος";
    adverb: "~ον";
    ),
    // Κᾰκοηθης: 19 words (πληρης, σῠνηθης, τρῐηρης, ἀηθης, κᾰκοηθης, κᾰλοηθης). Like Ἀτελης, except in 12 cells: nom_m du, nom_m pl, nom_f du, nom_f pl, gen_m sg, gen_m du, ….
    declension!("κᾰκοηθης", 19, "~ης", like Ἀτελης;
    nom_m: _, "~ει", "~εις";
    nom_f: _, "~ει", "~εις";
    gen_m: "~ους", "~οιν", _;
    gen_f: "~ους", "~οιν", _;
    dat_m: "~ει", _, _;
    dat_f: "~ει", _, _;
    acc_m: _, _, "~εις";
    acc_f: _, _, "~εις";
    ),
    // Παυων: 19 words (περῐοδευων, παιδευων, ἀγγαρευων, στρᾰτευων, κελευων, ἀκουων).
    declension!("παυων", 19, "~υων";
    nom_m: "~υων", "~υοντε", "~υοντες";
    nom_f: "~υουσᾰ", "~υουσᾱ", "~υουσαι";
    nom_n: "~ῡον", _, "~υοντᾰ";
    gen_m: "~υοντος", "~υοντοιν", "~υοντων";
    gen_f: "~υουσης", "~υουσαιν", "~υουσων";
    dat_m: "~υοντῐ", _, "~υουσῐ|~υουσῐν";
    dat_f: "~υουσῃ", _, "~υουσαις";
    dat_n: _, _, "~υουσῐ|~υουσῐν";
    acc_m: "~υοντᾰ", _, "~υοντᾰς";
    acc_f: "~υουσᾰν", _, "~υουσᾱς";
    voc_m: "~υων", _, _;
    voc_f: "~υουσᾰ", _, _;
    adverb: "~υοντως";
    ),
    // Δειξᾱς: 18 words (ἀγγειλᾱς, ἀγειρᾱς, ἀειρᾱς, δεισᾱς, μεινᾱς, κτεινᾱς).
    declension!("δειξᾱς", 18, "~ι~ᾱς";
    nom_m: "~ι~ᾱς", "~ι~ᾰντε", "~ι~ᾰντες";
    nom_f: "~ι~ᾱσᾰ", "~ι~ᾱσᾱ", "~ι~ᾱσαι";
    nom_n: "~ῑ~ᾰν", _, "~ι~ᾰντᾰ";
    gen_m: "~ι~ᾰντος", "~ι~ᾰντοιν", "~ι~ᾰντων";
    gen_f: "~ι~ᾱσης", "~ι~ᾱσαιν", "~ι~ᾱσων";
    dat_m: "~ι~ᾰντῐ", _, "~ι~ᾱσῐ|~ι~ᾱσῐν";
    dat_f: "~ι~ᾱσῃ", _, "~ι~ᾱσαις";
    dat_n: _, _, "~ι~ᾱσῐ|~ι~ᾱσῐν";
    acc_m: "~ι~ᾰντᾰ", _, "~ι~ᾰντᾰς";
    acc_f: "~ι~ᾱσᾰν", _, "~ι~ᾱσᾱς";
    voc_m: "~ι~ᾱς", _, _;
    voc_f: "~ι~ᾱσᾰ", _, _;
    adverb: "~ι~ᾰντως";
    ),
    // Τοιοσδε: 12 words (ἀρρην, τοιοσδε, τοσοσδε, δᾰϊφρων, λῡομενη, βοτηρ). Like Ἐμπεδος, except in 21 cells: nom_m sg, nom_m du, nom_m pl, nom_f sg, nom_f du, nom_f pl, ….
    declension!("τοιοσδε", 12, "~", like Ἐμπεδος;
    nom_m: "~", "~ε", "~ες";
    nom_f: "~", "~ε", "~ες";
    nom_n: "~", _, _;
    gen_m: "~ος", _, _;
    gen_f: "~ος", _, _;
    dat_m: "~ῐ", _, "~σῐ";
    dat_f: "~ῐ", _, "~σῐ";
    acc_m: "~ᾰ", _, "~ᾰς";
    acc_f: "~ᾰ", _, "~ᾰς";
    voc_m: "~", _, _;
    voc_f: "~", _, _;
    comparative: derived;
    superlative: derived;
    ),
    // Δῐκαιος: 11 words (ἀκρος, ἀλβος, ἀργῳος, χῑος, ἀζυμος, γλῐσχρος). Like Τελειος, except in 2 cells: comparative, superlative.
    declension!("δῐκαιος", 11, "~ος", like Τελειος;
    comparative: "~οτερος";
    superlative: "~οτᾰτος";
    ),
    // Φαῡλος: 11 words (δοῡλος, φαῡλος, παῡρος, κοῡφος, σαῡλος, κραῡρος).
    declension!("φαῡλος", 11, "~ῡ~ος";
    nom_m: "~ῡ~ος", "~υ~ω", "~ῡ~οι";
    nom_f: "~υ~η", "~υ~ᾱ", "~ῡ~αι";
    nom_n: "~ῡ~ον", _, "~ῡ~ᾰ";
    gen_m: "~υ~ου", "~υ~οιν", "~υ~ων";
    gen_f: "~υ~ης", "~υ~αιν", "~υ~ων";
    dat_m: "~υ~ῳ", _, "~υ~οις";
    dat_f: "~υ~ῃ", _, "~υ~αις";
    acc_m: "~ῡ~ον", _, "~υ~ους";
    acc_f: "~υ~ην", _, "~υ~ᾱς";
    voc_m: "~ῡ~ε", _, _;
    voc_f: "~υ~η", _, _;
    comparative: "~υ~οτερος";
    superlative: "~υ~οτᾰτος";
    adverb: "~υ~ως";
    ),
    // Ἀγνως: 11 words (μεγᾰς, δρομᾰς, ἀγνως, νομᾰς, πενης, ἐπηλῠς).
    declension!("ἀγνως", 11, "~ς";
    nom_m: "~ς", "~δε", "~δες";
    nom_f: "~ς", "~δε", "~δες";
    nom_n: "~", _, "~δᾰ";
    gen_m: "~δος", "~δοιν", "~δων";
    gen_f: "~δος", "~δοιν", "~δων";
    dat_m: "~δῐ", _, "~σῐ|~σῐν";
    dat_f: "~δῐ", _, "~σῐ|~σῐν";
    dat_n: _, _, "~σῐ";
    acc_m: "~δᾰ", _, "~δᾰς";
    acc_f: "~δᾰ", _, "~δᾰς";
    voc_m: "~ς", _, _;
    voc_f: "~ς", _, _;
    adverb: "~δως";
    ),
    // Γαῡρος: 10 words (πᾰνοῡργος, λαῡρος, θοῡρος, γαῡρος, κᾰκοῡργος, σκηπτοῡχος). Like Φαῡλος, except in 10 cells: nom_f sg, nom_f du, nom_f pl, gen_f sg, gen_f du, dat_f sg, ….
    declension!("γαῡρος", 10, "~ῡ~ος", like Φαῡλος;
    nom_f: "~ῡ~ος", "~υ~ω", "~ῡ~οι";
    gen_f: "~υ~ου", "~υ~οιν", _;
    dat_f: "~υ~ῳ", _, "~υ~οις";
    acc_f: "~ῡ~ον", _, "~υ~ους";
    voc_f: "~ῡ~ε", _, _;
    ),
    // Ζευξᾱς: 10 words (παιδευσᾱς, ἀγγαρευσᾱς, στρᾰτευσᾱς, ἀκουσᾱς, πλευσᾱς, ἀπουρᾱς).
    declension!("ζευξᾱς", 10, "~υ~ᾱς";
    nom_m: "~υ~ᾱς", "~υ~ᾰντε", "~υ~ᾰντες";
    nom_f: "~υ~ᾱσᾰ", "~υ~ᾱσᾱ", "~υ~ᾱσαι";
    nom_n: "~ῡ~ᾰν", _, "~υ~ᾰντᾰ";
    gen_m: "~υ~ᾰντος", "~υ~ᾰντοιν", "~υ~ᾰντων";
    gen_f: "~υ~ᾱσης", "~υ~ᾱσαιν", "~υ~ᾱσων";
    dat_m: "~υ~ᾰντῐ", _, "~υ~ᾱσῐ|~υ~ᾱσῐν";
    dat_f: "~υ~ᾱσῃ", _, "~υ~ᾱσαις";
    dat_n: _, _, "~υ~ᾱσῐ|~υ~ᾱσῐν";
    acc_m: "~υ~ᾰντᾰ", _, "~υ~ᾰντᾰς";
    acc_f: "~υ~ᾱσᾰν", _, "~υ~ᾱσᾱς";
    voc_m: "~υ~ᾱς", _, _;
    voc_f: "~υ~ᾱσᾰ", _, _;
    adverb: "~υ~ᾰντως";
    ),
    // Ἐρημος: 9 words (ἀλεξανδρῑνος, λᾰτῑνος, λῐσπος, κᾰκομᾰζᾱλος, δελτος, λᾰγνος). Like Ἰσος, except in 2 cells: comparative, superlative.
    declension!("ἐρημος", 9, "~ος", like Ἰσος;
    comparative: "~οτερος";
    superlative: "~οτᾰτος";
    ),
    // Θειων: 9 words (θειων, ναιων, κλαιων, οἰκειων, κᾰκκειων, πολεμησειων).
    declension!("θειων", 9, "~ιων";
    nom_m: "~ιων", "~ιοντε", "~ιοντες";
    nom_f: "~ιουσᾰ", "~ιουσᾱ", "~ιουσαι";
    nom_n: "~ῑον", _, "~ιοντᾰ";
    gen_m: "~ιοντος", "~ιοντοιν", "~ιοντων";
    gen_f: "~ιουσης", "~ιουσαιν", "~ιουσων";
    dat_m: "~ιοντῐ", _, "~ιουσῐ|~ιουσῐν";
    dat_f: "~ιουσῃ", _, "~ιουσαις";
    dat_n: _, _, "~ιουσῐ|~ιουσῐν";
    acc_m: "~ιοντᾰ", _, "~ιοντᾰς";
    acc_f: "~ιουσᾰν", _, "~ιουσᾱς";
    voc_m: "~ιων", _, _;
    voc_f: "~ιουσᾰ", _, _;
    adverb: "~ιοντως";
    ),
    // Κᾰλλῐθρῐξ: 9 words (ἡλῐξ, κᾰλλῐθρῐξ, πτωξ, ὁμηλῐξ, χρῡσᾰμπῠξ, μελᾰνθρῐξ).
    declension!("κᾰλλῐθρῐξ", 9, "~ξ";
    nom_m: "~ξ", "~κε", "~κες";
    nom_f: "~ξ", "~κε", "~κες";
    nom_n: "~ξ", _, "~κᾰ";
    gen_m: "~κος", "~κοιν", "~κων";
    gen_f: "~κος", "~κοιν", "~κων";
    dat_m: "~κῐ", _, "~ξῐ|~ξῐν";
    dat_f: "~κῐ", _, "~ξῐ|~ξῐν";
    dat_n: _, _, "~ξῐ|~ξῐν";
    acc_m: "~κᾰ", _, "~κᾰς";
    acc_f: "~κᾰ", _, "~κᾰς";
    voc_m: "~ξ", _, _;
    voc_f: "~ξ", _, _;
    adverb: "~κως";
    ),
    // Παυσων: 9 words (παιδευσων, ἀγγαρευσων, φευγων, ἀγορευσων, πολῑτευσων, τευχων).
    declension!("παυσων", 9, "~υ~ων";
    nom_m: "~υ~ων", "~υ~οντε", "~υ~οντες";
    nom_f: "~υ~ουσᾰ", "~υ~ουσᾱ", "~υ~ουσαι";
    nom_n: "~ῡ~ον", _, "~υ~οντᾰ";
    gen_m: "~υ~οντος", "~υ~οντοιν", "~υ~οντων";
    gen_f: "~υ~ουσης", "~υ~ουσαιν", "~υ~ουσων";
    dat_m: "~υ~οντῐ", _, "~υ~ουσῐ|~υ~ουσῐν";
    dat_f: "~υ~ουσῃ", _, "~υ~ουσαις";
    dat_n: _, _, "~υ~ουσῐ|~υ~ουσῐν";
    acc_m: "~υ~οντᾰ", _, "~υ~οντᾰς";
    acc_f: "~υ~ουσᾰν", _, "~υ~ουσᾱς";
    voc_m: "~υ~ων", _, _;
    voc_f: "~υ~ουσᾰ", _, _;
    adverb: "~υ~οντως";
    ),
    // Κολοβος: 8 words (πᾰθῐκος, κολοβος, ἀκῐρος, πᾰρδᾰκος, ἐπῐπεδῐκος, ἐκκοπρωτικος). Like Μαυρος, except in 2 cells: comparative, superlative.
    declension!("κολοβος", 8, "~ος", like Μαυρος;
    comparative: "~ωτερος";
    superlative: "~ωτᾰτος";
    ),
    // Τρῐπους: 8 words (τετρᾰπους, ὀκτωπους, τρῐπους, ὀκτᾰπους, εἰλῐπους, ἐρῠθροπους). Like Ἀγνως, except in 6 cells: nom_m sg, nom_f sg, nom_n sg, dat_n pl, voc_m sg, voc_f sg.
    declension!("τρῐπους", 8, "~υς", like Ἀγνως;
    nom_m: "~υς", _, _;
    nom_f: "~υς", _, _;
    nom_n: "~υν", _, _;
    dat_n: _, _, "~σῐ|~σῐν";
    voc_m: "~υς", _, _;
    voc_f: "~υς", _, _;
    ),
    // Εὐελπῐς: 7 words (ἐϋς, εὐελπῐς, εὐχᾰρῐς, ἀνᾰλκῐς, φερᾰσπῐς, ἀπᾰτρῐς). Like Ἀγνως, except in 5 cells: dat_n pl, acc_m sg, acc_f sg, voc_m sg, voc_f sg.
    declension!("εὐελπῐς", 7, "~ς", like Ἀγνως;
    dat_n: _, _, "~σῐ|~σῐν";
    acc_m: "~ν", _, _;
    acc_f: "~ν", _, _;
    voc_m: "~", _, _;
    voc_f: "~", _, _;
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
    // Ἀπᾰτωρ: 6 words (ἀλεκτωρ, ἀπᾰτωρ, μεγᾰλητωρ, ἀγηνωρ, ὑψηνωρ, ἀμητωρ).
    declension!("ἀπᾰτωρ", 6, "~ω~";
    nom_m: "~ω~", "~ο~ε", "~ο~ες";
    nom_f: "~ω~", "~ο~ε", "~ο~ες";
    nom_n: "~ο~", _, "~ο~ᾰ";
    gen_m: "~ο~ος", "~ο~οιν", "~ο~ων";
    gen_f: "~ο~ος", "~ο~οιν", "~ο~ων";
    dat_m: "~ο~ῐ", _, "~ο~σῐ|~ο~σῐν";
    dat_f: "~ο~ῐ", _, "~ο~σῐ|~ο~σῐν";
    dat_n: _, _, "~ο~σῐ|~ο~σῐν";
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
    dat_n: _, _, "~ῡσῐ|~ῡσῐν";
    acc_m: "~ῠντᾰ", _, "~ῠντᾰς";
    acc_f: "~ῡσᾰν", _, "~ῡσᾱς";
    voc_m: "~ῡς", _, _;
    voc_f: "~ῡσᾰ", _, _;
    adverb: "~ῠντως";
    ),
    // Ἁπλοῡς: 6 words (δῐπλοῡς, ἁπλοῡς, χρῡσοῡς, ἀργῠροῡς, χᾰλκοῡς, πορφῠροῡς). Like Μᾰλᾰκος, except in 16 cells: nom_m sg, nom_m du, nom_m pl, nom_f pl, nom_n sg, nom_n du, ….
    declension!("ἁπλοῡς", 6, "~οῡς", like Μᾰλᾰκος;
    nom_m: "~οῡς", "~ω|~ω", "~οῑ";
    nom_f: _, _, "~αῑ";
    nom_n: "~οῡν", "~ω|~ω", "~ᾱ";
    acc_m: "~οῡν", "~ω|~ω", "~οῡς";
    acc_n: _, "~ω|~ω", _;
    voc_m: "~οῡ", "~ω|~ω", _;
    voc_n: _, "~ω|~ω", _;
    comparative: "~εωτερος";
    superlative: "~εωτᾰτος";
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
    dat_n: _, _, "~ψῐ|~ψῐν";
    acc_m: "~πᾰ", _, "~πᾰς";
    acc_f: "~πᾰ", _, "~πᾰς";
    voc_m: "~ψ", _, _;
    voc_f: "~ψ", _, _;
    adverb: "~πως";
    ),
    // Πλεῑστος: 4 words (πρωτος, πλεῑστος, κᾰλλῐστος, ἀγχιστος). Like Ἀρῐστος, except in 2 cells: superlative, adverb.
    declension!("πλεῑστος", 4, "~ος", like Ἀρῐστος;
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
    dat_n: _, _, "~ᾰσῐ|~ᾰσῐν";
    acc_m: "~ᾰνᾰ", _, "~ᾰνᾰς";
    acc_f: "~αινᾰν", _, "~αινᾱς";
    voc_m: "~ᾰν", _, _;
    voc_f: "~αινᾰ", _, _;
    comparative: "~ᾰντερος";
    superlative: "~ᾰντᾰτος";
    adverb: "~ᾰνως";
    ),
    // Θηλῠς: 4 words (ἡμῐσῠς, θηλῠς, πρεσβῠς, ἀμφῐβρᾰχῠς). Like Τρᾱχῠς, except in 11 cells: nom_m du, nom_m pl, nom_f sg, nom_f pl, nom_n pl, dat_m sg, ….
    declension!("θηλῠς", 4, "~ῠς", like Τρᾱχῠς;
    nom_m: _, "~ει", "~εις";
    nom_f: "~ειᾰ", _, "~ειαι";
    nom_n: _, _, "~η|~εᾰ";
    dat_m: "~ει", _, _;
    acc_m: _, _, "~εις";
    acc_f: "~ειᾰν", _, _;
    acc_n: _, _, "~η|~εᾰ";
    voc_f: "~ειᾰ", _, _;
    voc_n: _, _, "~η|~εᾰ";
    ),
    // Ἀπους: 4 words (πολυπους, ἀπους, χειμαρρους, δῐπους). Like Τρῐπους, except in 2 cells: acc_m sg, acc_f sg.
    declension!("ἀπους", 4, "~υς", like Τρῐπους;
    acc_m: "~υν", _, _;
    acc_f: "~υν", _, _;
    ),
    // Χειρων: 4 words (μειζων, χειρων, κρειττων, τρῐσκᾰκοδαιμων).
    declension!("χειρων", 4, "~ι~ων";
    nom_m: "~ι~ων", "~ι~ονε", "~ι~ονες|~ι~ους";
    nom_f: "~ι~ων", "~ι~ονε", "~ι~ονες|~ι~ους";
    nom_n: "~ῑ~ον", _, "~ι~ονᾰ|~ι~ω";
    gen_m: "~ι~ονος", "~ι~ονοιν", "~ι~ονων";
    gen_f: "~ι~ονος", "~ι~ονοιν", "~ι~ονων";
    dat_m: "~ι~ονῐ", _, "~ι~οσῐ|~ι~οσῐν";
    dat_f: "~ι~ονῐ", _, "~ι~οσῐ|~ι~οσῐν";
    dat_n: _, _, "~ι~οσῐ|~ι~οσῐν";
    acc_m: "~ι~ονᾰ|~ι~ω", _, "~ι~ονᾰς|~ι~ους";
    acc_f: "~ι~ονᾰ|~ι~ω", _, "~ι~ονᾰς|~ι~ους";
    acc_n: _, _, "~ι~ονᾰ|~ι~ω";
    voc_m: "~ῑ~ον", _, "~ι~ονες|~ι~ους";
    voc_f: "~ῑ~ον", _, "~ι~ονες|~ι~ους";
    voc_n: _, _, "~ι~ονᾰ|~ι~ω";
    adverb: "~ῑ~ον";
    ),
    // Μωλῠς: 4 words (νεκῠς, εὐϊχθῠς, πολῠϊχθῠς, μωλῠς).
    declension!("μωλῠς", 4, "~ῠς";
    nom_m: "~ῠς", "~ῠε|~ῡ", "~ῠες|~ῡς";
    nom_f: "~ῠς", "~ῠε|~ῡ", "~ῠες|~ῡς";
    nom_n: "~ῠ", "~ῠε|~ῡ", "~ῠᾰ";
    gen_m: "~ῠος", "~ῠοιν", "~ῠων";
    gen_f: "~ῠος", "~ῠοιν", "~ῠων";
    dat_m: "~ῠϊ|~υι", _, "~ῠσῐ|~ῠσῐν";
    dat_f: "~ῠϊ|~υι", _, "~ῠσῐ|~ῠσῐν";
    dat_n: "~ῠϊ|~υι", _, "~ῠσῐ|~ῠσῐν";
    acc_m: "~ῠν", "~ῠε|~ῡ", "~ῡς|~ῠᾰς";
    acc_f: "~ῠν", "~ῠε|~ῡ", "~ῡς|~ῠᾰς";
    acc_n: _, "~ῠε|~ῡ", _;
    voc_m: "~ῠ", "~ῠε|~ῡ", "~ῠες|~ῡς";
    voc_f: "~ῠ", "~ῠε|~ῡ", "~ῠες|~ῡς";
    voc_n: _, "~ῠε|~ῡ", _;
    comparative: "~ῠτερος";
    superlative: "~ῠτᾰτος";
    adverb: "~ῠως";
    ),
    // Δους: 4 words (δους, βῐους, δῐδους, γνους). Like Πληγεις, except in 15 cells: nom_m sg, nom_f sg, nom_f du, nom_f pl, gen_f sg, gen_f du, ….
    declension!("δους", 4, "~υς", like Πληγεις;
    nom_m: "~υς", _, _;
    nom_f: "~ῡσᾰ", "~υσᾱ", "~ῡσαι";
    gen_f: "~υσης", "~υσαιν", "~υσων";
    dat_m: _, _, "~ῡσῐ|~ῡσῐν";
    dat_f: "~υσῃ", _, "~υσαις";
    dat_n: _, _, "~ῡσῐ|~ῡσῐν";
    acc_f: "~ῡσᾰν", _, "~υσᾱς";
    voc_m: "~υς", _, _;
    voc_f: "~ῡσᾰ", _, _;
    ),
    // Πεπτεως: 4 words (πεπτεως, πεπτηως, τετληως, κεκοτηως). Like Εἰκως, except in 4 cells: nom_f sg, nom_f pl, acc_f sg, voc_f sg.
    declension!("πεπτεως", 4, "~ως", like Εἰκως;
    nom_f: "~υιᾰ", _, "~υιαι";
    acc_f: "~υιᾰν", _, _;
    voc_f: "~υιᾰ", _, _;
    ),
    // Βαιος: 3 words (πᾰλαιος, βαιος, ἠβαιος). Like Λᾰμπρος, except in 1 cell: adverb.
    declension!("βαιος", 3, "~ος", like Λᾰμπρος;
    adverb: "~ον";
    ),
    // Δῐᾱνταῑος: 3 words (χῠδαῑος, ᾰμορβαῑος, δῐᾱνταῑος). Like Οἰκεῑος, except in 10 cells: nom_f sg, nom_f du, nom_f pl, gen_f sg, gen_f du, dat_f sg, ….
    declension!("δῐᾱνταῑος", 3, "~ῑος", like Οἰκεῑος;
    nom_f: "~ῑος", "~ιω", "~ῑοι";
    gen_f: "~ιου", "~ιοιν", _;
    dat_f: "~ιῳ", _, "~ιοις";
    acc_f: "~ῑον", _, "~ιους";
    voc_f: "~ῑε", _, _;
    ),
    // Ἀντῐπαις: 3 words (ὀκρῐς, ἀντῐπαις, ἀλκῠονῐς). Like Εὐελπῐς, except in 2 cells: acc_m sg, acc_f sg.
    declension!("ἀντῐπαις", 3, "~ς", like Εὐελπῐς;
    acc_m: "~δᾰ", _, _;
    acc_f: "~δᾰ", _, _;
    ),
    // Μωνῠξ: 3 words (μωνῠξ, γᾰμψωνῠξ, κρᾰτερωνῠξ). Like Κᾰλλῐθρῐξ, except in 18 cells: nom_m du, nom_m pl, nom_f du, nom_f pl, nom_n pl, gen_m sg, ….
    declension!("μωνῠξ", 3, "~ξ", like Κᾰλλῐθρῐξ;
    nom_m: _, "~χε", "~χες";
    nom_f: _, "~χε", "~χες";
    nom_n: _, _, "~χᾰ";
    gen_m: "~χος", "~χοιν", "~χων";
    gen_f: "~χος", "~χοιν", "~χων";
    dat_m: "~χῐ", _, _;
    dat_f: "~χῐ", _, _;
    acc_m: "~χᾰ", _, "~χᾰς";
    acc_f: "~χᾰ", _, "~χᾰς";
    adverb: "~χως";
    ),
    // Νηποινος: 3 words (νηποινος, τᾰλαυρῑνος, ἀλῐᾰστος). Like Ἀσημος, except in 1 cell: adverb.
    declension!("νηποινος", 3, "~ος", like Ἀσημος;
    adverb: "~ον";
    ),
];
