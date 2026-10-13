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
    /// Like `ἐμπεδος`: 791 words.
    Ἐμπεδος,
    /// Like `ἰσος`: 750 words.
    Ἰσος,
    /// Like `μᾰλᾰκος`: 670 words.
    Μᾰλᾰκος,
    /// Like `ἀτελης`: 254 words.
    Ἀτελης,
    /// Like `τελειος`: 235 words.
    Τελειος,
    /// Like `προσηκων`: 193 words.
    Προσηκων,
    /// Like `λᾰμπρος`: 139 words.
    Λᾰμπρος,
    /// Like `προπᾱς`: 125 words.
    Προπᾱς,
    /// Like `ψῑλος`: 113 words.
    Ψῑλος,
    /// Like `ὑστερος`: 111 words.
    Ὑστερος,
    /// Like `ἀσημος`: 105 words.
    Ἀσημος,
    /// Like `πληγεις`: 90 words.
    Πληγεις,
    /// Like `ἡκῐστος`: 89 words.
    Ἡκῐστος,
    /// Like `οἰκεῑος`: 76 words.
    Οἰκεῑος,
    /// Like `ἰων`: 75 words.
    Ἰων,
    /// Like `ἀρκων`: 55 words.
    Ἀρκων,
    /// Like `εἰκως`: 46 words.
    Εἰκως,
    /// Like `μαυρος`: 46 words.
    Μαυρος,
    /// Like `πλειων`: 34 words.
    Πλειων,
    /// Like `ζων`: 33 words.
    Ζων,
    /// Like `φωνηεις`: 28 words.
    Φωνηεις,
    /// Like `ἀγνως`: 25 words.
    Ἀγνως,
    /// Like `ἀκαρπος`: 25 words.
    Ἀκαρπος,
    /// Like `τρᾱχῠς`: 24 words.
    Τρᾱχῠς,
    /// Like `βαινων`: 23 words.
    Βαινων,
    /// Like `ἀρχικος`: 21 words.
    Ἀρχικος,
    /// Like `ἡσσων`: 20 words.
    Ἡσσων,
    /// Like `κᾰκοηθης`: 19 words.
    Κᾰκοηθης,
    /// Like `παυων`: 19 words.
    Παυων,
    /// Like `ἀδιψος`: 19 words.
    Ἀδιψος,
    /// Like `δειξᾱς`: 17 words.
    Δειξᾱς,
    /// Like `ἀριστος`: 16 words.
    Ἀριστος,
    /// Like `ἐρημος`: 15 words.
    Ἐρημος,
    /// Like `ὑπατος`: 15 words.
    Ὑπατος,
    /// Like `τρῐπους`: 13 words.
    Τρῐπους,
    /// Like `ἀγαθος`: 13 words.
    Ἀγαθος,
    /// Like `ἀβατος`: 12 words.
    Ἀβατος,
    /// Like `ἀρτιος`: 12 words.
    Ἀρτιος,
    /// Like `τοιοσδε`: 12 words.
    Τοιοσδε,
    /// Like `δῐκαιος`: 11 words.
    Δῐκαιος,
    /// Like `φαῡλος`: 11 words.
    Φαῡλος,
    /// Like `ὑποτακτικος`: 11 words.
    Ὑποτακτικος,
    /// Like `γαῡρος`: 10 words.
    Γαῡρος,
    /// Like `ἀμαθης`: 10 words.
    Ἀμαθης,
    /// Like `ἀμυμων`: 10 words.
    Ἀμυμων,
    /// Like `φᾰνερος`: 9 words.
    Φᾰνερος,
    /// Like `ἀξιωτερος`: 9 words.
    Ἀξιωτερος,
    /// Like `ἀξιωτατος`: 9 words.
    Ἀξιωτατος,
    /// Like `κᾰλλῐθρῐξ`: 9 words.
    Κᾰλλῐθρῐξ,
    /// Like `ζευξᾱς`: 9 words.
    Ζευξᾱς,
    /// Like `δαϊζων`: 9 words.
    Δαϊζων,
    /// Like `ἰσοτατος`: 9 words.
    Ἰσοτατος,
    /// Like `κολοβος`: 8 words.
    Κολοβος,
    /// Like `θειων`: 8 words.
    Θειων,
    /// Like `ἀκυρος`: 8 words.
    Ἀκυρος,
    /// Like `ἀσυμπτωτος`: 7 words.
    Ἀσυμπτωτος,
    /// Like `ἀσιτος`: 7 words.
    Ἀσιτος,
    /// Like `ἀκρατος`: 7 words.
    Ἀκρατος,
    /// Like `ἀξῐοχρεως`: 7 words.
    Ἀξῐοχρεως,
    /// Like `ἀκαθαρτος`: 6 words.
    Ἀκαθαρτος,
    /// Like `ἰδιος`: 6 words.
    Ἰδιος,
    /// Like `ἀπατωρ`: 6 words.
    Ἀπατωρ,
    /// Like `ἀρας`: 6 words.
    Ἀρας,
    /// Like `φῡς`: 6 words.
    Φῡς,
    /// Like `δρυϊνος`: 6 words.
    Δρυϊνος,
    /// Like `ἀγωνιστικος`: 5 words.
    Ἀγωνιστικος,
    /// Like `ἀγραμματος`: 5 words.
    Ἀγραμματος,
    /// Like `ἀσυνδετος`: 5 words.
    Ἀσυνδετος,
    /// Like `ἀδυνατος`: 5 words.
    Ἀδυνατος,
    /// Like `αἰθοψ`: 5 words.
    Αἰθοψ,
    /// Like `δους`: 5 words.
    Δους,
    /// Like `ἁπλοῡς`: 5 words.
    Ἁπλοῡς,
    /// Like `ἀντιπαις`: 5 words.
    Ἀντιπαις,
    /// Like `ἀντιπαλος`: 5 words.
    Ἀντιπαλος,
    /// Like `ἀοριστος`: 4 words.
    Ἀοριστος,
    /// Like `πλεῑστος`: 4 words.
    Πλεῑστος,
    /// Like `μελᾱς`: 4 words.
    Μελᾱς,
    /// Like `ἀπους`: 4 words.
    Ἀπους,
    /// Like `χειρων`: 4 words.
    Χειρων,
    /// Like `ἀθανατος`: 4 words.
    Ἀθανατος,
    /// Like `πεπτεως`: 4 words.
    Πεπτεως,
    /// Like `ἀδαμᾱς`: 4 words.
    Ἀδαμᾱς,
    /// Like `ἰχθυηρος`: 4 words.
    Ἰχθυηρος,
    /// Like `βαιος`: 3 words.
    Βαιος,
    /// Like `θηλῠς`: 3 words.
    Θηλῠς,
    /// Like `δῐᾱνταῑος`: 3 words.
    Δῐᾱνταῑος,
    /// Like `ἀτυχης`: 3 words.
    Ἀτυχης,
    /// Like `μωνῠξ`: 3 words.
    Μωνῠξ,
    /// Like `ἱκανωτερος`: 3 words.
    Ἱκανωτερος,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 89] = [
        Declension::Ἐμπεδος,
        Declension::Ἰσος,
        Declension::Μᾰλᾰκος,
        Declension::Ἀτελης,
        Declension::Τελειος,
        Declension::Προσηκων,
        Declension::Λᾰμπρος,
        Declension::Προπᾱς,
        Declension::Ψῑλος,
        Declension::Ὑστερος,
        Declension::Ἀσημος,
        Declension::Πληγεις,
        Declension::Ἡκῐστος,
        Declension::Οἰκεῑος,
        Declension::Ἰων,
        Declension::Ἀρκων,
        Declension::Εἰκως,
        Declension::Μαυρος,
        Declension::Πλειων,
        Declension::Ζων,
        Declension::Φωνηεις,
        Declension::Ἀγνως,
        Declension::Ἀκαρπος,
        Declension::Τρᾱχῠς,
        Declension::Βαινων,
        Declension::Ἀρχικος,
        Declension::Ἡσσων,
        Declension::Κᾰκοηθης,
        Declension::Παυων,
        Declension::Ἀδιψος,
        Declension::Δειξᾱς,
        Declension::Ἀριστος,
        Declension::Ἐρημος,
        Declension::Ὑπατος,
        Declension::Τρῐπους,
        Declension::Ἀγαθος,
        Declension::Ἀβατος,
        Declension::Ἀρτιος,
        Declension::Τοιοσδε,
        Declension::Δῐκαιος,
        Declension::Φαῡλος,
        Declension::Ὑποτακτικος,
        Declension::Γαῡρος,
        Declension::Ἀμαθης,
        Declension::Ἀμυμων,
        Declension::Φᾰνερος,
        Declension::Ἀξιωτερος,
        Declension::Ἀξιωτατος,
        Declension::Κᾰλλῐθρῐξ,
        Declension::Ζευξᾱς,
        Declension::Δαϊζων,
        Declension::Ἰσοτατος,
        Declension::Κολοβος,
        Declension::Θειων,
        Declension::Ἀκυρος,
        Declension::Ἀσυμπτωτος,
        Declension::Ἀσιτος,
        Declension::Ἀκρατος,
        Declension::Ἀξῐοχρεως,
        Declension::Ἀκαθαρτος,
        Declension::Ἰδιος,
        Declension::Ἀπατωρ,
        Declension::Ἀρας,
        Declension::Φῡς,
        Declension::Δρυϊνος,
        Declension::Ἀγωνιστικος,
        Declension::Ἀγραμματος,
        Declension::Ἀσυνδετος,
        Declension::Ἀδυνατος,
        Declension::Αἰθοψ,
        Declension::Δους,
        Declension::Ἁπλοῡς,
        Declension::Ἀντιπαις,
        Declension::Ἀντιπαλος,
        Declension::Ἀοριστος,
        Declension::Πλεῑστος,
        Declension::Μελᾱς,
        Declension::Ἀπους,
        Declension::Χειρων,
        Declension::Ἀθανατος,
        Declension::Πεπτεως,
        Declension::Ἀδαμᾱς,
        Declension::Ἰχθυηρος,
        Declension::Βαιος,
        Declension::Θηλῠς,
        Declension::Δῐᾱνταῑος,
        Declension::Ἀτυχης,
        Declension::Μωνῠξ,
        Declension::Ἱκανωτερος,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 89] = [
    // Ἐμπεδος: 791 words (δῐᾰβολος, βᾰρβᾰρος, πρωτοτοκος, μᾰγος, θεομᾰχος, εὐχᾰρῐστος).
    declension!("ἐμπεδος", 791, "~ος";
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
    // Ἰσος: 750 words (ἐρωμενος, ὀλῐγος, ὁλος, ἐσχᾰτος, ἀλλος, ξενος). Like Ἐμπεδος, except in 10 cells: nom_f sg, nom_f du, nom_f pl, gen_f sg, gen_f du, dat_f sg, ….
    declension!("ἰσος", 750, "~ος", like Ἐμπεδος;
    nom_f: "~η", "~ᾱ", "~αι";
    gen_f: "~ης", "~αιν", _;
    dat_f: "~ῃ", _, "~αις";
    acc_f: "~ην", _, "~ᾱς";
    voc_f: "~η", _, _;
    ),
    // Μᾰλᾰκος: 670 words (παιδῐκος, γᾰλᾰκτῐκος, λευκος, πνευματικος, κρῠπτος, κᾰκος). Like Ἰσος, except in 5 cells: gen_m sg, gen_m du, gen_f du, dat_m pl, dat_f pl.
    declension!("μᾰλᾰκος", 670, "~ος", like Ἰσος;
    gen_m: "~οῡ", "~οῑν", _;
    gen_f: _, "~αῑν", _;
    dat_m: _, _, "~οῑς";
    dat_f: _, _, "~αῑς";
    ),
    // Ἀτελης: 254 words (ὑγιης, ἀβαρης, ψευδης, ἀτελης, δηλητηρῐωδης, σῑγμοειδης). Like Μᾰλᾰκος, except in 24 cells: nom_m sg, nom_m du, nom_m pl, nom_f sg, nom_f du, nom_f pl, ….
    declension!("ἀτελης", 254, "~ης", like Μᾰλᾰκος;
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
    // Τελειος: 235 words (κῡρῐος, νεος, ἑτερος, ὁμοιος, πλουσῐος, βῐαιος). Like Ἰσος, except in 5 cells: nom_f sg, gen_f sg, dat_f sg, acc_f sg, voc_f sg.
    declension!("τελειος", 235, "~ος", like Ἰσος;
    nom_f: "~ᾱ", _, _;
    gen_f: "~ᾱς", _, _;
    dat_f: "~ᾳ", _, _;
    acc_f: "~ᾱν", _, _;
    voc_f: "~ᾱ", _, _;
    ),
    // Προσηκων: 193 words (κῠων, μελλων, ὀζων, νεων, γερων, ἀρχων).
    declension!("προσηκων", 193, "~ων";
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
    // Λᾰμπρος: 139 words (μῑκρος, ἐρῠθρος, χλωρος, νεκρος, κᾰθᾰρος, δεξῐος). Like Μᾰλᾰκος, except in 7 cells: nom_f sg, gen_f sg, dat_f sg, acc_f sg, voc_f sg, comparative, ….
    declension!("λᾰμπρος", 139, "~ος", like Μᾰλᾰκος;
    nom_f: "~ᾱ", _, _;
    gen_f: "~ᾱς", _, _;
    dat_f: "~ᾳ", _, _;
    acc_f: "~ᾱν", _, _;
    voc_f: "~ᾱ", _, _;
    comparative: "~οτερος";
    superlative: "~οτᾰτος";
    ),
    // Προπᾱς: 125 words (λῡσᾱς, γῐγᾱς, βῐβᾱς, φωνησᾱς, νῠξᾱς, αὐδησᾱς).
    declension!("προπᾱς", 125, "~ᾱς";
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
    // Ψῑλος: 113 words (ἀδελφος, ἰουδαϊκος, ξᾰνθος, βᾰκτρῐᾱνος, ἁγνος, ὀρθος). Like Μᾰλᾰκος, except in 2 cells: comparative, superlative.
    declension!("ψῑλος", 113, "~ος", like Μᾰλᾰκος;
    comparative: "~οτερος";
    superlative: "~οτᾰτος";
    ),
    // Ὑστερος: 111 words (προτερος, βᾰθῠτερος, πρεσβῠτερος, βεβαιοτερος, ὑστερος, στοιχειωδεστερος).
    declension!("ὑστερος", 111, "~ερος";
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
    // Ἀσημος: 105 words (πᾰρᾰκλητος, πρᾱος, ἐπικουρος, χερσος, ἱλαος, τῠρᾰννος). Like Ἐμπεδος, except in 2 cells: comparative, superlative.
    declension!("ἀσημος", 105, "~ος", like Ἐμπεδος;
    comparative: "~οτερος";
    superlative: "~οτᾰτος";
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
    // Ἡκῐστος: 89 words (μεγῐστος, ἡκῐστος, στοιχειωδεστᾰτος, κᾰκῐστος, τᾰχῐστος, βελτῐστος). Like Ἰσος, except in 3 cells: comparative, superlative, adverb.
    declension!("ἡκῐστος", 89, "~ος", like Ἰσος;
    comparative: derived;
    superlative: derived;
    adverb: "~ᾰ";
    ),
    // Οἰκεῑος: 76 words (ἀρχαῑος, ῥωμαῑος, ἀνδρεῑος, ἑβραῑος, ἀργεῑος, θηβαῑος).
    declension!("οἰκεῑος", 76, "~ῑος";
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
    // Ἰων: 75 words (πᾰρων, ἀπων, λᾰβων, πᾰρελθων, ἑκων, εἰπων). Like Προσηκων, except in 6 cells: nom_f sg, nom_f pl, dat_m pl, dat_n pl, acc_f sg, voc_f sg.
    declension!("ἰων", 75, "~ων", like Προσηκων;
    nom_f: "~οῡσᾰ", _, "~οῡσαι";
    dat_m: _, _, "~οῡσῐ|~οῡσῐν";
    dat_n: _, _, "~οῡσῐ|~οῡσῐν";
    acc_f: "~οῡσᾰν", _, _;
    voc_f: "~οῡσᾰ", _, _;
    ),
    // Ἀρκων: 55 words (ποιων, κυκλων, σημειων, βῐων, δηλων, γονῠπετων). Like Ἰων, except in 11 cells: nom_m du, nom_m pl, nom_n sg, nom_n pl, gen_m sg, gen_m du, ….
    declension!("ἀρκων", 55, "~ων", like Ἰων;
    nom_m: _, "~οῡντε", "~οῡντες";
    nom_n: "~οῡν", _, "~οῡντᾰ";
    gen_m: "~οῡντος", "~ουντοιν", "~ουντων";
    dat_m: "~οῡντῐ", _, _;
    acc_m: "~οῡντᾰ", _, "~οῡντᾰς";
    adverb: "~ουντως";
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
    // Μαυρος: 46 words (μαυρος, σῠνεργος, ἀγωγος, γεωργος, σῠνᾰπτος, ἐνεργος). Like Ἀσημος, except in 6 cells: gen_m sg, gen_m du, gen_f sg, gen_f du, dat_m pl, dat_f pl.
    declension!("μαυρος", 46, "~ος", like Ἀσημος;
    gen_m: "~οῡ", "~οῑν", _;
    gen_f: "~οῡ", "~οῑν", _;
    dat_m: _, _, "~οῑς";
    dat_f: _, _, "~οῑς";
    ),
    // Πλειων: 34 words (σωφρων, ἀκμων, βελτῑων, κᾰκοδαιμων, μειων, ὑπερφρων).
    declension!("πλειων", 34, "~ων";
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
    // Φωνηεις: 28 words (χᾰρῐεις, φωνηεις, αἱμᾰτοεις, ῥοδοεις, ἰχθυοεις, φθογγηεις). Like Πληγεις, except in 16 cells: nom_f sg, nom_f du, nom_f pl, gen_f sg, gen_f du, gen_f pl, ….
    declension!("φωνηεις", 28, "~ις", like Πληγεις;
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
    // Ἀγνως: 25 words (μεγᾰς, δρομᾰς, ἐϋς, ἀγνως, ὀκρῐς, νομᾰς).
    declension!("ἀγνως", 25, "~ς";
    nom_m: "~ς", "~δε", "~δες";
    nom_f: "~ς", "~δε", "~δες";
    nom_n: "~", _, "~δᾰ";
    gen_m: "~δος", "~δοιν", "~δων";
    gen_f: "~δος", "~δοιν", "~δων";
    dat_m: "~δῐ", _, "~σῐ";
    dat_f: "~δῐ", _, "~σῐ";
    acc_m: "~δᾰ", _, "~δᾰς";
    acc_f: "~δᾰ", _, "~δᾰς";
    voc_m: "~ς", _, _;
    voc_f: "~ς", _, _;
    adverb: "~δως";
    ),
    // Ἀκαρπος: 25 words (ἀναπολογητος, ἀκαρπος, ἀδηφαγος, ἀναπαιστος, ἀνανδρος, ἀγαθοποιος).
    declension!("ἀκαρπος", 25, "~α~ος";
    nom_m: "~ᾰ~ος", "~ᾰ~ω", "~ᾰ~οι";
    nom_f: "~ᾰ~ος", "~ᾰ~ω", "~ᾰ~οι";
    nom_n: "~ᾰ~ον", _, "~ᾰ~ᾰ";
    gen_m: "~ᾰ~ου", "~ᾰ~οιν", "~ᾰ~ων";
    gen_f: "~ᾰ~ου", "~ᾰ~οιν", "~ᾰ~ων";
    dat_m: "~ᾰ~ῳ", _, "~ᾰ~οις";
    dat_f: "~ᾰ~ῳ", _, "~ᾰ~οις";
    acc_m: "~ᾰ~ον", _, "~ᾰ~ους";
    acc_f: "~ᾰ~ον", _, "~ᾰ~ους";
    voc_m: "~ᾰ~ε", _, _;
    voc_f: "~ᾰ~ε", _, _;
    comparative: "~ᾰ~οτερος";
    superlative: "~ᾰ~οτᾰτος";
    adverb: "~ᾰ~ως";
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
    // Ἀρχικος: 21 words (ἀριστερος, ἀττικος, ἀποπληκτικος, ὑλικος, ἱστορικος, ὑποθετικος).
    declension!("ἀρχικος", 21, "~ι~ος";
    nom_m: "~ῐ~ος", "~ῐ~ω", "~ῐ~οι";
    nom_f: "~ῐ~η", "~ῐ~ᾱ", "~ῐ~αι";
    nom_n: "~ῐ~ον", _, "~ῐ~ᾰ";
    gen_m: "~ῐ~οῡ", "~ῐ~οῑν", "~ῐ~ων";
    gen_f: "~ῐ~ης", "~ῐ~αῑν", "~ῐ~ων";
    dat_m: "~ῐ~ῳ", _, "~ῐ~οῑς";
    dat_f: "~ῐ~ῃ", _, "~ῐ~αῑς";
    acc_m: "~ῐ~ον", _, "~ῐ~ους";
    acc_f: "~ῐ~ην", _, "~ῐ~ᾱς";
    voc_m: "~ῐ~ε", _, _;
    voc_f: "~ῐ~η", _, _;
    comparative: "~ῐ~ωτερος";
    superlative: "~ῐ~ωτᾰτος";
    adverb: "~ῐ~ως";
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
    // Ἀδιψος: 19 words (ἀδικος, ἀγριελαιος, ἀντιλογος, ἀοριστος, ἀμαυροστικτος, ἀξιολογος). Like Ἀρχικος, except in 13 cells: nom_f sg, nom_f du, nom_f pl, gen_m sg, gen_m du, gen_f sg, ….
    declension!("ἀδιψος", 19, "~ι~ος", like Ἀρχικος;
    nom_f: "~ῐ~ος", "~ῐ~ω", "~ῐ~οι";
    gen_m: "~ῐ~ου", "~ῐ~οιν", _;
    gen_f: "~ῐ~ου", "~ῐ~οιν", _;
    dat_m: _, _, "~ῐ~οις";
    dat_f: "~ῐ~ῳ", _, "~ῐ~οις";
    acc_f: "~ῐ~ον", _, "~ῐ~ους";
    voc_f: "~ῐ~ε", _, _;
    ),
    // Δειξᾱς: 17 words (ἀγειρᾱς, ἀειρᾱς, δεισᾱς, μεινᾱς, κτεινᾱς, σπεισᾱς).
    declension!("δειξᾱς", 17, "~ι~ᾱς";
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
    // Ἀριστος: 16 words (ὑψιστος, ἀερινος, ἀριστος, ἀτειχιστος, ἀξιουμενος, ὑμενινος). Like Ἀρχικος, except in 5 cells: gen_m sg, gen_m du, gen_f du, dat_m pl, dat_f pl.
    declension!("ἀριστος", 16, "~ι~ος", like Ἀρχικος;
    gen_m: "~ῐ~ου", "~ῐ~οιν", _;
    gen_f: _, "~ῐ~αιν", _;
    dat_m: _, _, "~ῐ~οις";
    dat_f: _, _, "~ῐ~αις";
    ),
    // Ἐρημος: 15 words (ἀλεξανδρῑνος, λᾰτῑνος, πεμπτος, τετᾰρτος, τρῐτος, λῐσπος). Like Ἰσος, except in 2 cells: comparative, superlative.
    declension!("ἐρημος", 15, "~ος", like Ἰσος;
    comparative: "~οτερος";
    superlative: "~οτᾰτος";
    ),
    // Ὑπατος: 15 words (ὑπατος, ὑπογεγραμμενος, ἀνεσκαμμενος, ἀγαθοποιησομενος, ἀγγειλαμενος, ἱσταμενος). Like Ἀκαρπος, except in 12 cells: nom_f sg, nom_f du, nom_f pl, gen_f sg, gen_f du, dat_f sg, ….
    declension!("ὑπατος", 15, "~α~ος", like Ἀκαρπος;
    nom_f: "~ᾰ~η", "~ᾰ~ᾱ", "~ᾰ~αι";
    gen_f: "~ᾰ~ης", "~ᾰ~αιν", _;
    dat_f: "~ᾰ~ῃ", _, "~ᾰ~αις";
    acc_f: "~ᾰ~ην", _, "~ᾰ~ᾱς";
    voc_f: "~ᾰ~η", _, _;
    comparative: "~ᾰ~ωτερος";
    superlative: "~ᾰ~ωτᾰτος";
    ),
    // Τρῐπους: 13 words (τετρᾰπους, ὀκτωπους, τρῐπους, ἰθυς, ἀμφιβραχυς, ὀκτᾰπους). Like Ἀγνως, except in 8 cells: nom_m sg, nom_f sg, nom_n sg, dat_m pl, dat_f pl, dat_n pl, ….
    declension!("τρῐπους", 13, "~υς", like Ἀγνως;
    nom_m: "~υς", _, _;
    nom_f: "~υς", _, _;
    nom_n: "~υν", _, _;
    dat_m: _, _, "~σῐ|~σῐν";
    dat_f: _, _, "~σῐ|~σῐν";
    dat_n: _, _, "~σῐ|~σῐν";
    voc_m: "~υς", _, _;
    voc_f: "~υς", _, _;
    ),
    // Ἀγαθος: 13 words (ἀγαθος, ἀγαπητος, ῥωμαϊκος, ἱλαρος, ἀλᾰπαδνος, ἱκανος). Like Ὑπατος, except in 5 cells: gen_m sg, gen_m du, gen_f du, dat_m pl, dat_f pl.
    declension!("ἀγαθος", 13, "~α~ος", like Ὑπατος;
    gen_m: "~ᾰ~οῡ", "~ᾰ~οῑν", _;
    gen_f: _, "~ᾰ~αῑν", _;
    dat_m: _, _, "~ᾰ~οῑς";
    dat_f: _, _, "~ᾰ~αῑς";
    ),
    // Ἀβατος: 12 words (ἀγραφος, ἀκεφαλος, ἀγναφος, ἀνωμαλος, ἀναλογος, ἀβατος). Like Ἀκαρπος, except in 2 cells: comparative, superlative.
    declension!("ἀβατος", 12, "~α~ος", like Ἀκαρπος;
    comparative: "~ᾰ~ωτερος";
    superlative: "~ᾰ~ωτᾰτος";
    ),
    // Ἀρτιος: 12 words (ἁγιος, ἀγριος, ἀξιος, ἀρτιος, ἀλλοτριος, ἁλιος).
    declension!("ἀρτιος", 12, "~ιος";
    nom_m: "~ῐος", "~ῐω", "~ῐοι";
    nom_f: "~ῐᾱ", "~ῐᾱ", "~ῐαι";
    nom_n: "~ῐον", _, "~ῐᾰ";
    gen_m: "~ῐου", "~ῐοιν", "~ῐων";
    gen_f: "~ῐᾱς", "~ῐαιν", "~ῐων";
    dat_m: "~ῐῳ", _, "~ῐοις";
    dat_f: "~ῐᾳ", _, "~ῐαις";
    acc_m: "~ῐον", _, "~ῐους";
    acc_f: "~ῐᾱν", _, "~ῐᾱς";
    voc_m: "~ῐε", _, _;
    voc_f: "~ῐᾱ", _, _;
    comparative: "~ῐωτερος";
    superlative: "~ῐωτᾰτος";
    adverb: "~ῐως";
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
    // Ὑποτακτικος: 11 words (ἀμφαριστερος, ὑπατικος, ἀναλογητικος, ἀναγιγνωσκομενος, ὑποτακτικος, ἱμαντινος).
    declension!("ὑποτακτικος", 11, "~α~ι~ος";
    nom_m: "~ᾰ~ῐ~ος", "~ᾰ~ῐ~ω", "~ᾰ~ῐ~οι";
    nom_f: "~ᾰ~ῐ~η", "~ᾰ~ῐ~ᾱ", "~ᾰ~ῐ~αι";
    nom_n: "~ᾰ~ῐ~ον", _, "~ᾰ~ῐ~ᾰ";
    gen_m: "~ᾰ~ῐ~οῡ", "~ᾰ~ῐ~οῑν", "~ᾰ~ῐ~ων";
    gen_f: "~ᾰ~ῐ~ης", "~ᾰ~ῐ~αῑν", "~ᾰ~ῐ~ων";
    dat_m: "~ᾰ~ῐ~ῳ", _, "~ᾰ~ῐ~οῑς";
    dat_f: "~ᾰ~ῐ~ῃ", _, "~ᾰ~ῐ~αῑς";
    acc_m: "~ᾰ~ῐ~ον", _, "~ᾰ~ῐ~ους";
    acc_f: "~ᾰ~ῐ~ην", _, "~ᾰ~ῐ~ᾱς";
    voc_m: "~ᾰ~ῐ~ε", _, _;
    voc_f: "~ᾰ~ῐ~η", _, _;
    comparative: "~ᾰ~ῐ~ωτερος";
    superlative: "~ᾰ~ῐ~ωτᾰτος";
    adverb: "~ᾰ~ῐ~ως";
    ),
    // Γαῡρος: 10 words (πᾰνοῡργος, λαῡρος, θοῡρος, γαῡρος, κᾰκοῡργος, σκηπτοῡχος). Like Φαῡλος, except in 10 cells: nom_f sg, nom_f du, nom_f pl, gen_f sg, gen_f du, dat_f sg, ….
    declension!("γαῡρος", 10, "~ῡ~ος", like Φαῡλος;
    nom_f: "~ῡ~ος", "~υ~ω", "~ῡ~οι";
    gen_f: "~υ~ου", "~υ~οιν", _;
    dat_f: "~υ~ῳ", _, "~υ~οις";
    acc_f: "~ῡ~ον", _, "~υ~ους";
    voc_f: "~ῡ~ε", _, _;
    ),
    // Ἀμαθης: 10 words (ἀπαθης, ἀμαθης, ἀσφαλης, ἀφανης, ἀσαφης, ἀραχνοειδης). Like Ἀγαθος, except in 24 cells: nom_m sg, nom_m du, nom_m pl, nom_f sg, nom_f du, nom_f pl, ….
    declension!("ἀμαθης", 10, "~α~ης", like Ἀγαθος;
    nom_m: "~ᾰ~ης", "~ᾰ~εῑ", "~ᾰ~εῑς";
    nom_f: "~ᾰ~ης", "~ᾰ~εῑ", "~ᾰ~εῑς";
    nom_n: "~ᾰ~ες", _, "~ᾰ~η";
    gen_m: "~ᾰ~οῡς", _, _;
    gen_f: "~ᾰ~οῡς", "~ᾰ~οῑν", _;
    dat_m: "~ᾰ~εῑ", _, "~ᾰ~εσῐ|~ᾰ~εσῐν";
    dat_f: "~ᾰ~εῑ", _, "~ᾰ~εσῐ|~ᾰ~εσῐν";
    dat_n: _, _, "~ᾰ~εσῐ|~ᾰ~εσῐν";
    acc_m: "~ᾰ~η", _, "~ᾰ~εῑς";
    acc_f: "~ᾰ~η", _, "~ᾰ~εῑς";
    voc_m: "~ᾰ~ες", _, _;
    voc_f: "~ᾰ~ες", _, _;
    comparative: "~ᾰ~εστερος";
    superlative: "~ᾰ~εστᾰτος";
    ),
    // Ἀμυμων: 10 words (ἀμυμων, παιδευσων, ἀγγαρευσων, φευγων, ἀγορευσων, πολῑτευσων).
    declension!("ἀμυμων", 10, "~υ~ων";
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
    // Φᾰνερος: 9 words (στερεος, σᾰρωνῐκος, ἐτεος, πολῐος, σκολῐος, πελῐος). Like Λᾰμπρος, except in 2 cells: comparative, superlative.
    declension!("φᾰνερος", 9, "~ος", like Λᾰμπρος;
    comparative: "~ωτερος";
    superlative: "~ωτᾰτος";
    ),
    // Ἀξιωτερος: 9 words (ἰδιωτερος, ἀκλιτωτερος, ἀπιστοτερος, ἀγριωτερος, ἀξιωτερος, ἀθλιωτερος). Like Ἀριστος, except in 8 cells: nom_f sg, gen_f sg, dat_f sg, acc_f sg, voc_f sg, comparative, ….
    declension!("ἀξιωτερος", 9, "~ι~ος", like Ἀριστος;
    nom_f: "~ῐ~ᾱ", _, _;
    gen_f: "~ῐ~ᾱς", _, _;
    dat_f: "~ῐ~ᾳ", _, _;
    acc_f: "~ῐ~ᾱν", _, _;
    voc_f: "~ῐ~ᾱ", _, _;
    comparative: derived;
    superlative: derived;
    adverb: "~ῐ~ον";
    ),
    // Ἀξιωτατος: 9 words (ἰδιωτατος, ἀκλιτωτατος, ἀλλοτριωτατος, ἀγριωτατος, ἀξιωτατος, ἀθλιωτατος).
    declension!("ἀξιωτατος", 9, "~ι~α~ος";
    nom_m: "~ῐ~ᾰ~ος", "~ῐ~ᾰ~ω", "~ῐ~ᾰ~οι";
    nom_f: "~ῐ~ᾰ~η", "~ῐ~ᾰ~ᾱ", "~ῐ~ᾰ~αι";
    nom_n: "~ῐ~ᾰ~ον", _, "~ῐ~ᾰ~ᾰ";
    gen_m: "~ῐ~ᾰ~ου", "~ῐ~ᾰ~οιν", "~ῐ~ᾰ~ων";
    gen_f: "~ῐ~ᾰ~ης", "~ῐ~ᾰ~αιν", "~ῐ~ᾰ~ων";
    dat_m: "~ῐ~ᾰ~ῳ", _, "~ῐ~ᾰ~οις";
    dat_f: "~ῐ~ᾰ~ῃ", _, "~ῐ~ᾰ~αις";
    acc_m: "~ῐ~ᾰ~ον", _, "~ῐ~ᾰ~ους";
    acc_f: "~ῐ~ᾰ~ην", _, "~ῐ~ᾰ~ᾱς";
    voc_m: "~ῐ~ᾰ~ε", _, _;
    voc_f: "~ῐ~ᾰ~η", _, _;
    adverb: "~ῐ~ᾰ~ᾰ";
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
    // Ζευξᾱς: 9 words (παιδευσᾱς, ἀγγαρευσᾱς, στρᾰτευσᾱς, πλευσᾱς, ἀπουρᾱς, ὀρουσᾱς).
    declension!("ζευξᾱς", 9, "~υ~ᾱς";
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
    // Δαϊζων: 9 words (ἀγαθοεργεων, ἀγαθοποιων, ἀγαθοποιησων, ἀκμαζων, ἁμαρτων, ὑπαρχων).
    declension!("δαϊζων", 9, "~α~ων";
    nom_m: "~ᾰ~ων", "~ᾰ~οντε", "~ᾰ~οντες";
    nom_f: "~ᾰ~ουσᾰ", "~ᾰ~ουσᾱ", "~ᾰ~ουσαι";
    nom_n: "~ᾰ~ον", _, "~ᾰ~οντᾰ";
    gen_m: "~ᾰ~οντος", "~ᾰ~οντοιν", "~ᾰ~οντων";
    gen_f: "~ᾰ~ουσης", "~ᾰ~ουσαιν", "~ᾰ~ουσων";
    dat_m: "~ᾰ~οντῐ", _, "~ᾰ~ουσῐ|~ᾰ~ουσῐν";
    dat_f: "~ᾰ~ουσῃ", _, "~ᾰ~ουσαις";
    dat_n: _, _, "~ᾰ~ουσῐ|~ᾰ~ουσῐν";
    acc_m: "~ᾰ~οντᾰ", _, "~ᾰ~οντᾰς";
    acc_f: "~ᾰ~ουσᾰν", _, "~ᾰ~ουσᾱς";
    voc_m: "~ᾰ~ων", _, _;
    voc_f: "~ᾰ~ουσᾰ", _, _;
    adverb: "~ᾰ~οντως";
    ),
    // Ἰσοτατος: 9 words (ἀηδεστατος, ἀνδρειοτατος, ἀργοτατος, ἰσαιτατος, ἱερωτατος, ἀδελφοτατος). Like Ὑπατος, except in 3 cells: comparative, superlative, adverb.
    declension!("ἰσοτατος", 9, "~α~ος", like Ὑπατος;
    comparative: derived;
    superlative: derived;
    adverb: "~ᾰ~ᾰ";
    ),
    // Κολοβος: 8 words (πᾰθῐκος, κολοβος, ἀκῐρος, πᾰρδᾰκος, ἐπῐπεδῐκος, ἐκκοπρωτικος). Like Μαυρος, except in 2 cells: comparative, superlative.
    declension!("κολοβος", 8, "~ος", like Μαυρος;
    comparative: "~ωτερος";
    superlative: "~ωτᾰτος";
    ),
    // Θειων: 8 words (θειων, ναιων, κλαιων, οἰκειων, κᾰκκειων, πολεμησειων).
    declension!("θειων", 8, "~ιων";
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
    // Ἀκυρος: 8 words (ἰσχυρος, ἀσυλος, θυϊνος, ὑπερθυμος, ἀπυγος, ἀχρυσος). Like Γαῡρος, except in 17 cells: nom_m du, nom_f du, gen_m sg, gen_m du, gen_m pl, gen_f sg, ….
    declension!("ἀκυρος", 8, "~υ~ος", like Γαῡρος;
    nom_m: _, "~ῡ~ω", _;
    nom_f: _, "~ῡ~ω", _;
    gen_m: "~ῡ~ου", "~ῡ~οιν", "~ῡ~ων";
    gen_f: "~ῡ~ου", "~ῡ~οιν", "~ῡ~ων";
    dat_m: "~ῡ~ῳ", _, "~ῡ~οις";
    dat_f: "~ῡ~ῳ", _, "~ῡ~οις";
    acc_m: _, _, "~ῡ~ους";
    acc_f: _, _, "~ῡ~ους";
    comparative: "~ῡ~οτερος";
    superlative: "~ῡ~οτᾰτος";
    adverb: "~ῡ~ως";
    ),
    // Ἀσυμπτωτος: 7 words (ἀνυποδητος, ἀμεθυστος, ἀρρυθμος, ἀργυροηλος, ὑποκυκλος, ἀγρυπνος).
    declension!("ἀσυμπτωτος", 7, "~υ~ος";
    nom_m: "~ῠ~ος", "~ῠ~ω", "~ῠ~οι";
    nom_f: "~ῠ~ος", "~ῠ~ω", "~ῠ~οι";
    nom_n: "~ῠ~ον", _, "~ῠ~ᾰ";
    gen_m: "~ῠ~ου", "~ῠ~οιν", "~ῠ~ων";
    gen_f: "~ῠ~ου", "~ῠ~οιν", "~ῠ~ων";
    dat_m: "~ῠ~ῳ", _, "~ῠ~οις";
    dat_f: "~ῠ~ῳ", _, "~ῠ~οις";
    acc_m: "~ῠ~ον", _, "~ῠ~ους";
    acc_f: "~ῠ~ον", _, "~ῠ~ους";
    voc_m: "~ῠ~ε", _, _;
    voc_f: "~ῠ~ε", _, _;
    comparative: "~ῠ~οτερος";
    superlative: "~ῠ~οτᾰτος";
    adverb: "~ῠ~ως";
    ),
    // Ἀσιτος: 7 words (ἀνικητος, ἀλιτρος, ἀφικομενος, ἀκριθος, ἀσιτος, ἀκριβεστερος).
    declension!("ἀσιτος", 7, "~ι~ος";
    nom_m: "~ῑ~ος", "~ῑ~ω", "~ῑ~οι";
    nom_f: "~ῑ~ος", "~ῑ~ω", "~ῑ~οι";
    nom_n: "~ῑ~ον", _, "~ῑ~ᾰ";
    gen_m: "~ῑ~ου", "~ῑ~οιν", "~ῑ~ων";
    gen_f: "~ῑ~ου", "~ῑ~οιν", "~ῑ~ων";
    dat_m: "~ῑ~ῳ", _, "~ῑ~οις";
    dat_f: "~ῑ~ῳ", _, "~ῑ~οις";
    acc_m: "~ῑ~ον", _, "~ῑ~ους";
    acc_f: "~ῑ~ον", _, "~ῑ~ους";
    voc_m: "~ῑ~ε", _, _;
    voc_f: "~ῑ~ε", _, _;
    comparative: "~ῑ~οτερος";
    superlative: "~ῑ~οτᾰτος";
    adverb: "~ῑ~ως";
    ),
    // Ἀκρατος: 7 words (λαϊκος, ἀγενεαλογητος, λαϊνος, ἀκρατος, ἱερακοπροσωπος, ἱσπανος).
    declension!("ἀκρατος", 7, "~α~ος";
    nom_m: "~ᾱ~ος", "~ᾱ~ω", "~ᾱ~οι";
    nom_f: "~ᾱ~ος", "~ᾱ~ω", "~ᾱ~οι";
    nom_n: "~ᾱ~ον", _, "~ᾱ~ᾰ";
    gen_m: "~ᾱ~ου", "~ᾱ~οιν", "~ᾱ~ων";
    gen_f: "~ᾱ~ου", "~ᾱ~οιν", "~ᾱ~ων";
    dat_m: "~ᾱ~ῳ", _, "~ᾱ~οις";
    dat_f: "~ᾱ~ῳ", _, "~ᾱ~οις";
    acc_m: "~ᾱ~ον", _, "~ᾱ~ους";
    acc_f: "~ᾱ~ον", _, "~ᾱ~ους";
    voc_m: "~ᾱ~ε", _, _;
    voc_f: "~ᾱ~ε", _, _;
    comparative: "~ᾱ~οτερος";
    superlative: "~ᾱ~οτᾰτος";
    adverb: "~ᾱ~ως";
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
    // Ἀκαθαρτος: 6 words (ἀκαθαρτος, ἀκαταληπτος, ἀκᾰτασκευαστος, ἀταλαντος, ἀναλφαβητος, ἀταρακτος).
    declension!("ἀκαθαρτος", 6, "~α~α~ος";
    nom_m: "~ᾰ~ᾰ~ος", "~ᾰ~ᾰ~ω", "~ᾰ~ᾰ~οι";
    nom_f: "~ᾰ~ᾰ~ος", "~ᾰ~ᾰ~ω", "~ᾰ~ᾰ~οι";
    nom_n: "~ᾰ~ᾰ~ον", _, "~ᾰ~ᾰ~ᾰ";
    gen_m: "~ᾰ~ᾰ~ου", "~ᾰ~ᾰ~οιν", "~ᾰ~ᾰ~ων";
    gen_f: "~ᾰ~ᾰ~ου", "~ᾰ~ᾰ~οιν", "~ᾰ~ᾰ~ων";
    dat_m: "~ᾰ~ᾰ~ῳ", _, "~ᾰ~ᾰ~οις";
    dat_f: "~ᾰ~ᾰ~ῳ", _, "~ᾰ~ᾰ~οις";
    acc_m: "~ᾰ~ᾰ~ον", _, "~ᾰ~ᾰ~ους";
    acc_f: "~ᾰ~ᾰ~ον", _, "~ᾰ~ᾰ~ους";
    voc_m: "~ᾰ~ᾰ~ε", _, _;
    voc_f: "~ᾰ~ᾰ~ε", _, _;
    comparative: "~ᾰ~ᾰ~οτερος";
    superlative: "~ᾰ~ᾰ~οτᾰτος";
    adverb: "~ᾰ~ᾰ~ως";
    ),
    // Ἰδιος: 6 words (ἰδιος, ὑποχονδριος, ἀποφωλιος, ἀεκουσιος, ἀκηριος, ὑπαιθριος). Like Ἀρτιος, except in 10 cells: nom_f sg, nom_f du, nom_f pl, gen_f sg, gen_f du, dat_f sg, ….
    declension!("ἰδιος", 6, "~ιος", like Ἀρτιος;
    nom_f: "~ῐος", "~ῐω", "~ῐοι";
    gen_f: "~ῐου", "~ῐοιν", _;
    dat_f: "~ῐῳ", _, "~ῐοις";
    acc_f: "~ῐον", _, "~ῐους";
    voc_f: "~ῐε", _, _;
    ),
    // Ἀπατωρ: 6 words (ἀλεκτωρ, ἀπατωρ, μεγᾰλητωρ, ἀγηνωρ, ὑψηνωρ, ἀμητωρ).
    declension!("ἀπατωρ", 6, "~ω~";
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
    // Ἀρας: 6 words (ἱστας, ἀγγειλας, ἀρας, ἀκουσας, ἀποβας, ἀϊξας). Like Προπᾱς, except in 0 cells: .
    declension!("ἀρας", 6, "~ας", like Προπᾱς;
    ),
    // Φῡς: 6 words (δεικνῡς, ἀργυροῡς, κᾰτᾰδῡς, ἐπῐμειγνῡς, ἀποδῡς, φῡς).
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
    // Δρυϊνος: 6 words (οἰσυϊνος, δρυϊνος, ἀργυρεος, συϊνος, ἀτυζομενος, ἀποδεικνυμενος). Like Ἀσυμπτωτος, except in 12 cells: nom_f sg, nom_f du, nom_f pl, gen_f sg, gen_f du, dat_f sg, ….
    declension!("δρυϊνος", 6, "~υ~ος", like Ἀσυμπτωτος;
    nom_f: "~ῠ~η", "~ῠ~ᾱ", "~ῠ~αι";
    gen_f: "~ῠ~ης", "~ῠ~αιν", _;
    dat_f: "~ῠ~ῃ", _, "~ῠ~αις";
    acc_f: "~ῠ~ην", _, "~ῠ~ᾱς";
    voc_f: "~ῠ~η", _, _;
    comparative: "~ῠ~ωτερος";
    superlative: "~ῠ~ωτᾰτος";
    ),
    // Ἀγωνιστικος: 5 words (ὑβριστικος, ἀντιθετικος, ἀντιλογητικος, ἀξιοπιστος, ἀγωνιστικος).
    declension!("ἀγωνιστικος", 5, "~ι~ι~ος";
    nom_m: "~ῐ~ῐ~ος", "~ῐ~ῐ~ω", "~ῐ~ῐ~οι";
    nom_f: "~ῐ~ῐ~η", "~ῐ~ῐ~ᾱ", "~ῐ~ῐ~αι";
    nom_n: "~ῐ~ῐ~ον", _, "~ῐ~ῐ~ᾰ";
    gen_m: "~ῐ~ῐ~οῡ", "~ῐ~ῐ~οῑν", "~ῐ~ῐ~ων";
    gen_f: "~ῐ~ῐ~ης", "~ῐ~ῐ~αῑν", "~ῐ~ῐ~ων";
    dat_m: "~ῐ~ῐ~ῳ", _, "~ῐ~ῐ~οῑς";
    dat_f: "~ῐ~ῐ~ῃ", _, "~ῐ~ῐ~αῑς";
    acc_m: "~ῐ~ῐ~ον", _, "~ῐ~ῐ~ους";
    acc_f: "~ῐ~ῐ~ην", _, "~ῐ~ῐ~ᾱς";
    voc_m: "~ῐ~ῐ~ε", _, _;
    voc_f: "~ῐ~ῐ~η", _, _;
    comparative: "~ῐ~ῐ~ωτερος";
    superlative: "~ῐ~ῐ~ωτᾰτος";
    adverb: "~ῐ~ῐ~ως";
    ),
    // Ἀγραμματος: 5 words (ἀνανταποδοτος, ἀγραμματος, ἀδαπανος, ἀμανδαλος, ἀκανθοφαγος). Like Ἀκαθαρτος, except in 2 cells: comparative, superlative.
    declension!("ἀγραμματος", 5, "~α~α~ος", like Ἀκαθαρτος;
    comparative: "~ᾰ~ᾰ~ωτερος";
    superlative: "~ᾰ~ᾰ~ωτᾰτος";
    ),
    // Ἀσυνδετος: 5 words (ἀνδρογυνος, ἀσυνδετος, ἰχθυοτροφος, ἀστυνομος, ἀπολυτος). Like Ἀσυμπτωτος, except in 2 cells: comparative, superlative.
    declension!("ἀσυνδετος", 5, "~υ~ος", like Ἀσυμπτωτος;
    comparative: "~ῠ~ωτερος";
    superlative: "~ῠ~ωτᾰτος";
    ),
    // Ἀδυνατος: 5 words (ἰχθυοφαγος, ἰχθυφαγος, ἀδυνατος, ἀτυραννευτος, ἰσοδυναμος).
    declension!("ἀδυνατος", 5, "~υ~α~ος";
    nom_m: "~ῠ~ᾰ~ος", "~ῠ~ᾰ~ω", "~ῠ~ᾰ~οι";
    nom_f: "~ῠ~ᾰ~ος", "~ῠ~ᾰ~ω", "~ῠ~ᾰ~οι";
    nom_n: "~ῠ~ᾰ~ον", _, "~ῠ~ᾰ~ᾰ";
    gen_m: "~ῠ~ᾰ~ου", "~ῠ~ᾰ~οιν", "~ῠ~ᾰ~ων";
    gen_f: "~ῠ~ᾰ~ου", "~ῠ~ᾰ~οιν", "~ῠ~ᾰ~ων";
    dat_m: "~ῠ~ᾰ~ῳ", _, "~ῠ~ᾰ~οις";
    dat_f: "~ῠ~ᾰ~ῳ", _, "~ῠ~ᾰ~οις";
    acc_m: "~ῠ~ᾰ~ον", _, "~ῠ~ᾰ~ους";
    acc_f: "~ῠ~ᾰ~ον", _, "~ῠ~ᾰ~ους";
    voc_m: "~ῠ~ᾰ~ε", _, _;
    voc_f: "~ῠ~ᾰ~ε", _, _;
    comparative: "~ῠ~ᾰ~ωτερος";
    superlative: "~ῠ~ᾰ~ωτᾰτος";
    adverb: "~ῠ~ᾰ~ως";
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
    // Δους: 5 words (δους, βῐους, δῐδους, γνους, ἀποδεικνυς). Like Πληγεις, except in 15 cells: nom_m sg, nom_f sg, nom_f du, nom_f pl, gen_f sg, gen_f du, ….
    declension!("δους", 5, "~υς", like Πληγεις;
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
    // Ἁπλοῡς: 5 words (δῐπλοῡς, ἁπλοῡς, χρῡσοῡς, χᾰλκοῡς, πορφῠροῡς). Like Μᾰλᾰκος, except in 16 cells: nom_m sg, nom_m du, nom_m pl, nom_f pl, nom_n sg, nom_n du, ….
    declension!("ἁπλοῡς", 5, "~οῡς", like Μᾰλᾰκος;
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
    // Ἀντιπαις: 5 words (ἀργινοεις, ἀντιπαις, ἀξιωθεις, ἀποτειχισθεις, ἀποκριθεις).
    declension!("ἀντιπαις", 5, "~ι~ις";
    nom_m: "~ῐ~ις", "~ῐ~ντε", "~ῐ~ντες";
    nom_f: "~ῐ~ῑσᾰ", "~ῐ~ισᾱ", "~ῐ~ῑσαι";
    nom_n: "~ῐ~ν", _, "~ῐ~ντᾰ";
    gen_m: "~ῐ~ντος", "~ῐ~ντοιν", "~ῐ~ντων";
    gen_f: "~ῐ~ισης", "~ῐ~ισαιν", "~ῐ~ισων";
    dat_m: "~ῐ~ντῐ", _, "~ῐ~ῑσῐ|~ῐ~ῑσῐν";
    dat_f: "~ῐ~ισῃ", _, "~ῐ~ισαις";
    dat_n: _, _, "~ῐ~ῑσῐ|~ῐ~ῑσῐν";
    acc_m: "~ῐ~ντᾰ", _, "~ῐ~ντᾰς";
    acc_f: "~ῐ~ῑσᾰν", _, "~ῐ~ισᾱς";
    voc_m: "~ῐ~ις", _, _;
    voc_f: "~ῐ~ῑσᾰ", _, _;
    adverb: "~ῐ~ντως";
    ),
    // Ἀντιπαλος: 5 words (ὑψιμελαθρος, ἀντιπαλος, ὑψικαρηνος, ἀμφικεφαλος, ἁλιβαπτος). Like Ἀξιωτατος, except in 13 cells: nom_f sg, nom_f du, nom_f pl, gen_f sg, gen_f du, dat_f sg, ….
    declension!("ἀντιπαλος", 5, "~ι~α~ος", like Ἀξιωτατος;
    nom_f: "~ῐ~ᾰ~ος", "~ῐ~ᾰ~ω", "~ῐ~ᾰ~οι";
    gen_f: "~ῐ~ᾰ~ου", "~ῐ~ᾰ~οιν", _;
    dat_f: "~ῐ~ᾰ~ῳ", _, "~ῐ~ᾰ~οις";
    acc_f: "~ῐ~ᾰ~ον", _, "~ῐ~ᾰ~ους";
    voc_f: "~ῐ~ᾰ~ε", _, _;
    comparative: "~ῐ~ᾰ~οτερος";
    superlative: "~ῐ~ᾰ~οτᾰτος";
    adverb: "~ῐ~ᾰ~ως";
    ),
    // Ἀοριστος: 4 words (ἁβροδιαιτος, ἰσοδιαιτος, ὑποκιρρος, ἀκινδῡνος). Like Ἀδιψος, except in 2 cells: comparative, superlative.
    declension!("ἀοριστος", 4, "~ι~ος", like Ἀδιψος;
    comparative: "~ῐ~οτερος";
    superlative: "~ῐ~οτᾰτος";
    ),
    // Πλεῑστος: 4 words (πρωτος, πλεῑστος, κᾰλλῐστος, ἀγχιστος). Like Ἡκῐστος, except in 2 cells: superlative, adverb.
    declension!("πλεῑστος", 4, "~ος", like Ἡκῐστος;
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
    // Ἀθανατος: 4 words (ἀθανατος, ἀναγκαζομενος, ἀνᾰχασσαμενος, ἱκανωτατος). Like Ἀγραμματος, except in 10 cells: nom_f sg, nom_f du, nom_f pl, gen_f sg, gen_f du, dat_f sg, ….
    declension!("ἀθανατος", 4, "~α~α~ος", like Ἀγραμματος;
    nom_f: "~ᾰ~ᾰ~η", "~ᾰ~ᾰ~ᾱ", "~ᾰ~ᾰ~αι";
    gen_f: "~ᾰ~ᾰ~ης", "~ᾰ~ᾰ~αιν", _;
    dat_f: "~ᾰ~ᾰ~ῃ", _, "~ᾰ~ᾰ~αις";
    acc_f: "~ᾰ~ᾰ~ην", _, "~ᾰ~ᾰ~ᾱς";
    voc_f: "~ᾰ~ᾰ~η", _, _;
    ),
    // Πεπτεως: 4 words (πεπτεως, πεπτηως, τετληως, κεκοτηως). Like Εἰκως, except in 4 cells: nom_f sg, nom_f pl, acc_f sg, voc_f sg.
    declension!("πεπτεως", 4, "~ως", like Εἰκως;
    nom_f: "~υιᾰ", _, "~υιαι";
    acc_f: "~υιᾰν", _, _;
    voc_f: "~υιᾰ", _, _;
    ),
    // Ἀδαμᾱς: 4 words (ἀδαμᾱς, ὑπαρξᾱς, ἀντῐασᾱς, ἀναπλησᾱς).
    declension!("ἀδαμᾱς", 4, "~α~ᾱς";
    nom_m: "~ᾰ~ᾱς", "~ᾰ~ᾰντε", "~ᾰ~ᾰντες";
    nom_f: "~ᾰ~ᾱσᾰ", "~ᾰ~ᾱσᾱ", "~ᾰ~ᾱσαι";
    nom_n: "~ᾰ~ᾰν", _, "~ᾰ~ᾰντᾰ";
    gen_m: "~ᾰ~ᾰντος", "~ᾰ~ᾰντοιν", "~ᾰ~ᾰντων";
    gen_f: "~ᾰ~ᾱσης", "~ᾰ~ᾱσαιν", "~ᾰ~ᾱσων";
    dat_m: "~ᾰ~ᾰντῐ", _, "~ᾰ~ᾱσῐ|~ᾰ~ᾱσῐν";
    dat_f: "~ᾰ~ᾱσῃ", _, "~ᾰ~ᾱσαις";
    dat_n: _, _, "~ᾰ~ᾱσῐ|~ᾰ~ᾱσῐν";
    acc_m: "~ᾰ~ᾰντᾰ", _, "~ᾰ~ᾰντᾰς";
    acc_f: "~ᾰ~ᾱσᾰν", _, "~ᾰ~ᾱσᾱς";
    voc_m: "~ᾰ~ᾱς", _, _;
    voc_f: "~ᾰ~ᾱσᾰ", _, _;
    adverb: "~ᾰ~ᾰντως";
    ),
    // Ἰχθυηρος: 4 words (νεκυϊκος, ἰχθυηρος, ἰχθυϊκος, ἀποκᾰλυπτῐκος). Like Δρυϊνος, except in 5 cells: gen_m sg, gen_m du, gen_f du, dat_m pl, dat_f pl.
    declension!("ἰχθυηρος", 4, "~υ~ος", like Δρυϊνος;
    gen_m: "~ῠ~οῡ", "~ῠ~οῑν", _;
    gen_f: _, "~ῠ~αῑν", _;
    dat_m: _, _, "~ῠ~οῑς";
    dat_f: _, _, "~ῠ~αῑς";
    ),
    // Βαιος: 3 words (πᾰλαιος, βαιος, ἠβαιος). Like Λᾰμπρος, except in 1 cell: adverb.
    declension!("βαιος", 3, "~ος", like Λᾰμπρος;
    adverb: "~ον";
    ),
    // Θηλῠς: 3 words (ἡμῐσῠς, θηλῠς, πρεσβῠς). Like Τρᾱχῠς, except in 11 cells: nom_m du, nom_m pl, nom_f sg, nom_f pl, nom_n pl, dat_m sg, ….
    declension!("θηλῠς", 3, "~ῠς", like Τρᾱχῠς;
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
    // Δῐᾱνταῑος: 3 words (χῠδαῑος, ᾰμορβαῑος, δῐᾱνταῑος). Like Οἰκεῑος, except in 10 cells: nom_f sg, nom_f du, nom_f pl, gen_f sg, gen_f du, dat_f sg, ….
    declension!("δῐᾱνταῑος", 3, "~ῑος", like Οἰκεῑος;
    nom_f: "~ῑος", "~ιω", "~ῑοι";
    gen_f: "~ιου", "~ιοιν", _;
    dat_f: "~ιῳ", _, "~ιοις";
    acc_f: "~ῑον", _, "~ιους";
    voc_f: "~ῑε", _, _;
    ),
    // Ἀτυχης: 3 words (ἰχθυοειδης, ἀτυχης, ἀρυταινοειδης). Like Ἰχθυηρος, except in 24 cells: nom_m sg, nom_m du, nom_m pl, nom_f sg, nom_f du, nom_f pl, ….
    declension!("ἀτυχης", 3, "~υ~ης", like Ἰχθυηρος;
    nom_m: "~ῠ~ης", "~ῠ~εῑ", "~ῠ~εῑς";
    nom_f: "~ῠ~ης", "~ῠ~εῑ", "~ῠ~εῑς";
    nom_n: "~ῠ~ες", _, "~ῠ~η";
    gen_m: "~ῠ~οῡς", _, _;
    gen_f: "~ῠ~οῡς", "~ῠ~οῑν", _;
    dat_m: "~ῠ~εῑ", _, "~ῠ~εσῐ|~ῠ~εσῐν";
    dat_f: "~ῠ~εῑ", _, "~ῠ~εσῐ|~ῠ~εσῐν";
    dat_n: _, _, "~ῠ~εσῐ|~ῠ~εσῐν";
    acc_m: "~ῠ~η", _, "~ῠ~εῑς";
    acc_f: "~ῠ~η", _, "~ῠ~εῑς";
    voc_m: "~ῠ~ες", _, _;
    voc_f: "~ῠ~ες", _, _;
    comparative: "~ῠ~εστερος";
    superlative: "~ῠ~εστᾰτος";
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
    // Ἱκανωτερος: 3 words (ἀναγκαιοτερος, ἱκανωτερος, ἀμαθεστερος). Like Ἰσοτατος, except in 6 cells: nom_f sg, gen_f sg, dat_f sg, acc_f sg, voc_f sg, adverb.
    declension!("ἱκανωτερος", 3, "~α~ος", like Ἰσοτατος;
    nom_f: "~ᾰ~ᾱ", _, _;
    gen_f: "~ᾰ~ᾱς", _, _;
    dat_f: "~ᾰ~ᾳ", _, _;
    acc_f: "~ᾰ~ᾱν", _, _;
    voc_f: "~ᾰ~ᾱ", _, _;
    adverb: "~ᾰ~ον";
    ),
];
