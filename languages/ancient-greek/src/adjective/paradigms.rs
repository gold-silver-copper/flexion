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
/// `nominative_masculine` (sg, du, pl)
/// `nominative_feminine` (sg, du, pl)
/// `nominative_neuter` (sg, du, pl)
/// `genitive_masculine` (sg, du, pl)
/// `genitive_feminine` (sg, du, pl)
/// `genitive_neuter` (sg, du, pl)
/// `dative_masculine` (sg, du, pl)
/// `dative_feminine` (sg, du, pl)
/// `dative_neuter` (sg, du, pl)
/// `accusative_masculine` (sg, du, pl)
/// `accusative_feminine` (sg, du, pl)
/// `accusative_neuter` (sg, du, pl)
/// `vocative_masculine` (sg, du, pl)
/// `vocative_feminine` (sg, du, pl)
/// `vocative_neuter` (sg, du, pl)
/// `comparative`
/// `superlative`
/// `adverb`
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident)?;
     $(nominative_masculine: $c0:tt, $c1:tt, $c2:tt;)?
     $(nominative_feminine: $c3:tt, $c4:tt, $c5:tt;)?
     $(nominative_neuter: $c6:tt, $c7:tt, $c8:tt;)?
     $(genitive_masculine: $c9:tt, $c10:tt, $c11:tt;)?
     $(genitive_feminine: $c12:tt, $c13:tt, $c14:tt;)?
     $(genitive_neuter: $c15:tt, $c16:tt, $c17:tt;)?
     $(dative_masculine: $c18:tt, $c19:tt, $c20:tt;)?
     $(dative_feminine: $c21:tt, $c22:tt, $c23:tt;)?
     $(dative_neuter: $c24:tt, $c25:tt, $c26:tt;)?
     $(accusative_masculine: $c27:tt, $c28:tt, $c29:tt;)?
     $(accusative_feminine: $c30:tt, $c31:tt, $c32:tt;)?
     $(accusative_neuter: $c33:tt, $c34:tt, $c35:tt;)?
     $(vocative_masculine: $c36:tt, $c37:tt, $c38:tt;)?
     $(vocative_feminine: $c39:tt, $c40:tt, $c41:tt;)?
     $(vocative_neuter: $c42:tt, $c43:tt, $c44:tt;)?
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
        nominative_masculine: "~ος", "~ω", "~οι";
        nominative_feminine: "~ος", "~ω", "~οι";
        nominative_neuter: "~ον", _, "~ᾰ";
        genitive_masculine: "~ου", "~οιν", "~ων";
        genitive_feminine: "~ου", "~οιν", "~ων";
        dative_masculine: "~ῳ", _, "~οις";
        dative_feminine: "~ῳ", _, "~οις";
        accusative_masculine: "~ον", _, "~ους";
        accusative_feminine: "~ον", _, "~ους";
        vocative_masculine: "~ε", _, _;
        vocative_feminine: "~ε", _, _;
        comparative: "~ωτερος";
        superlative: "~ωτᾰτος";
        adverb: "~ως";
    ),
    // Ἰσος: 750 words (ἐρωμενος, ὀλῐγος, ὁλος, ἐσχᾰτος, ἀλλος, ξενος). Like Ἐμπεδος, except in 10 cells: nominative_feminine sg, nominative_feminine du, nominative_feminine pl, genitive_feminine sg, genitive_feminine du, dative_feminine sg, ….
    declension!("ἰσος", 750, "~ος", like Ἐμπεδος;
        nominative_feminine: "~η", "~ᾱ", "~αι";
        genitive_feminine: "~ης", "~αιν", _;
        dative_feminine: "~ῃ", _, "~αις";
        accusative_feminine: "~ην", _, "~ᾱς";
        vocative_feminine: "~η", _, _;
    ),
    // Μᾰλᾰκος: 670 words (παιδῐκος, γᾰλᾰκτῐκος, λευκος, πνευματικος, κρῠπτος, κᾰκος). Like Ἰσος, except in 5 cells: genitive_masculine sg, genitive_masculine du, genitive_feminine du, dative_masculine pl, dative_feminine pl.
    declension!("μᾰλᾰκος", 670, "~ος", like Ἰσος;
        genitive_masculine: "~οῡ", "~οῑν", _;
        genitive_feminine: _, "~αῑν", _;
        dative_masculine: _, _, "~οῑς";
        dative_feminine: _, _, "~αῑς";
    ),
    // Ἀτελης: 254 words (ὑγιης, ἀβαρης, ψευδης, ἀτελης, δηλητηρῐωδης, σῑγμοειδης). Like Μᾰλᾰκος, except in 24 cells: nominative_masculine sg, nominative_masculine du, nominative_masculine pl, nominative_feminine sg, nominative_feminine du, nominative_feminine pl, ….
    declension!("ἀτελης", 254, "~ης", like Μᾰλᾰκος;
        nominative_masculine: "~ης", "~εῑ", "~εῑς";
        nominative_feminine: "~ης", "~εῑ", "~εῑς";
        nominative_neuter: "~ες", _, "~η";
        genitive_masculine: "~οῡς", _, _;
        genitive_feminine: "~οῡς", "~οῑν", _;
        dative_masculine: "~εῑ", _, "~εσῐ|~εσῐν";
        dative_feminine: "~εῑ", _, "~εσῐ|~εσῐν";
        dative_neuter: _, _, "~εσῐ|~εσῐν";
        accusative_masculine: "~η", _, "~εῑς";
        accusative_feminine: "~η", _, "~εῑς";
        vocative_masculine: "~ες", _, _;
        vocative_feminine: "~ες", _, _;
        comparative: "~εστερος";
        superlative: "~εστᾰτος";
    ),
    // Τελειος: 235 words (κῡρῐος, νεος, ἑτερος, ὁμοιος, πλουσῐος, βῐαιος). Like Ἰσος, except in 5 cells: nominative_feminine sg, genitive_feminine sg, dative_feminine sg, accusative_feminine sg, vocative_feminine sg.
    declension!("τελειος", 235, "~ος", like Ἰσος;
        nominative_feminine: "~ᾱ", _, _;
        genitive_feminine: "~ᾱς", _, _;
        dative_feminine: "~ᾳ", _, _;
        accusative_feminine: "~ᾱν", _, _;
        vocative_feminine: "~ᾱ", _, _;
    ),
    // Προσηκων: 193 words (κῠων, μελλων, ὀζων, νεων, γερων, ἀρχων).
    declension!("προσηκων", 193, "~ων";
        nominative_masculine: "~ων", "~οντε", "~οντες";
        nominative_feminine: "~ουσᾰ", "~ουσᾱ", "~ουσαι";
        nominative_neuter: "~ον", _, "~οντᾰ";
        genitive_masculine: "~οντος", "~οντοιν", "~οντων";
        genitive_feminine: "~ουσης", "~ουσαιν", "~ουσων";
        dative_masculine: "~οντῐ", _, "~ουσῐ|~ουσῐν";
        dative_feminine: "~ουσῃ", _, "~ουσαις";
        dative_neuter: _, _, "~ουσῐ|~ουσῐν";
        accusative_masculine: "~οντᾰ", _, "~οντᾰς";
        accusative_feminine: "~ουσᾰν", _, "~ουσᾱς";
        vocative_masculine: "~ων", _, _;
        vocative_feminine: "~ουσᾰ", _, _;
        adverb: "~οντως";
    ),
    // Λᾰμπρος: 139 words (μῑκρος, ἐρῠθρος, χλωρος, νεκρος, κᾰθᾰρος, δεξῐος). Like Μᾰλᾰκος, except in 7 cells: nominative_feminine sg, genitive_feminine sg, dative_feminine sg, accusative_feminine sg, vocative_feminine sg, comparative, ….
    declension!("λᾰμπρος", 139, "~ος", like Μᾰλᾰκος;
        nominative_feminine: "~ᾱ", _, _;
        genitive_feminine: "~ᾱς", _, _;
        dative_feminine: "~ᾳ", _, _;
        accusative_feminine: "~ᾱν", _, _;
        vocative_feminine: "~ᾱ", _, _;
        comparative: "~οτερος";
        superlative: "~οτᾰτος";
    ),
    // Προπᾱς: 125 words (λῡσᾱς, γῐγᾱς, βῐβᾱς, φωνησᾱς, νῠξᾱς, αὐδησᾱς).
    declension!("προπᾱς", 125, "~ᾱς";
        nominative_masculine: "~ᾱς", "~ᾰντε", "~ᾰντες";
        nominative_feminine: "~ᾱσᾰ", "~ᾱσᾱ", "~ᾱσαι";
        nominative_neuter: "~ᾰν", _, "~ᾰντᾰ";
        genitive_masculine: "~ᾰντος", "~ᾰντοιν", "~ᾰντων";
        genitive_feminine: "~ᾱσης", "~ᾱσαιν", "~ᾱσων";
        dative_masculine: "~ᾰντῐ", _, "~ᾱσῐ|~ᾱσῐν";
        dative_feminine: "~ᾱσῃ", _, "~ᾱσαις";
        dative_neuter: _, _, "~ᾱσῐ|~ᾱσῐν";
        accusative_masculine: "~ᾰντᾰ", _, "~ᾰντᾰς";
        accusative_feminine: "~ᾱσᾰν", _, "~ᾱσᾱς";
        vocative_masculine: "~ᾱς", _, _;
        vocative_feminine: "~ᾱσᾰ", _, _;
        adverb: "~ᾰντως";
    ),
    // Ψῑλος: 113 words (ἀδελφος, ἰουδαϊκος, ξᾰνθος, βᾰκτρῐᾱνος, ἁγνος, ὀρθος). Like Μᾰλᾰκος, except in 2 cells: comparative, superlative.
    declension!("ψῑλος", 113, "~ος", like Μᾰλᾰκος;
        comparative: "~οτερος";
        superlative: "~οτᾰτος";
    ),
    // Ὑστερος: 111 words (προτερος, βᾰθῠτερος, πρεσβῠτερος, βεβαιοτερος, ὑστερος, στοιχειωδεστερος).
    declension!("ὑστερος", 111, "~ερος";
        nominative_masculine: "~ερος", "~ερω", "~εροι";
        nominative_feminine: "~ερᾱ", "~ερᾱ", "~εραι";
        nominative_neuter: "~ερον", _, "~ερᾰ";
        genitive_masculine: "~ερου", "~εροιν", "~ερων";
        genitive_feminine: "~ερᾱς", "~εραιν", "~ερων";
        dative_masculine: "~ερῳ", _, "~εροις";
        dative_feminine: "~ερᾳ", _, "~εραις";
        accusative_masculine: "~ερον", _, "~ερους";
        accusative_feminine: "~ερᾱν", _, "~ερᾱς";
        vocative_masculine: "~ερε", _, _;
        vocative_feminine: "~ερᾱ", _, _;
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
        nominative_masculine: "~ις", "~ντε", "~ντες";
        nominative_feminine: "~ῑσᾰ", "~ισᾱ", "~ῑσαι";
        nominative_neuter: "~ν", _, "~ντᾰ";
        genitive_masculine: "~ντος", "~ντοιν", "~ντων";
        genitive_feminine: "~ισης", "~ισαιν", "~ισων";
        dative_masculine: "~ντῐ", _, "~ῑσῐ|~ῑσῐν";
        dative_feminine: "~ισῃ", _, "~ισαις";
        dative_neuter: _, _, "~ῑσῐ|~ῑσῐν";
        accusative_masculine: "~ντᾰ", _, "~ντᾰς";
        accusative_feminine: "~ῑσᾰν", _, "~ισᾱς";
        vocative_masculine: "~ις", _, _;
        vocative_feminine: "~ῑσᾰ", _, _;
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
        nominative_masculine: "~ῑος", "~ιω", "~ῑοι";
        nominative_feminine: "~ιᾱ", "~ιᾱ", "~ῑαι";
        nominative_neuter: "~ῑον", _, "~ῑᾰ";
        genitive_masculine: "~ιου", "~ιοιν", "~ιων";
        genitive_feminine: "~ιᾱς", "~ιαιν", "~ιων";
        dative_masculine: "~ιῳ", _, "~ιοις";
        dative_feminine: "~ιᾳ", _, "~ιαις";
        accusative_masculine: "~ῑον", _, "~ιους";
        accusative_feminine: "~ιᾱν", _, "~ιᾱς";
        vocative_masculine: "~ῑε", _, _;
        vocative_feminine: "~ιᾱ", _, _;
        comparative: "~ιοτερος";
        superlative: "~ιοτᾰτος";
        adverb: "~ιως";
    ),
    // Ἰων: 75 words (πᾰρων, ἀπων, λᾰβων, πᾰρελθων, ἑκων, εἰπων). Like Προσηκων, except in 6 cells: nominative_feminine sg, nominative_feminine pl, dative_masculine pl, dative_neuter pl, accusative_feminine sg, vocative_feminine sg.
    declension!("ἰων", 75, "~ων", like Προσηκων;
        nominative_feminine: "~οῡσᾰ", _, "~οῡσαι";
        dative_masculine: _, _, "~οῡσῐ|~οῡσῐν";
        dative_neuter: _, _, "~οῡσῐ|~οῡσῐν";
        accusative_feminine: "~οῡσᾰν", _, _;
        vocative_feminine: "~οῡσᾰ", _, _;
    ),
    // Ἀρκων: 55 words (ποιων, κυκλων, σημειων, βῐων, δηλων, γονῠπετων). Like Ἰων, except in 11 cells: nominative_masculine du, nominative_masculine pl, nominative_neuter sg, nominative_neuter pl, genitive_masculine sg, genitive_masculine du, ….
    declension!("ἀρκων", 55, "~ων", like Ἰων;
        nominative_masculine: _, "~οῡντε", "~οῡντες";
        nominative_neuter: "~οῡν", _, "~οῡντᾰ";
        genitive_masculine: "~οῡντος", "~ουντοιν", "~ουντων";
        dative_masculine: "~οῡντῐ", _, _;
        accusative_masculine: "~οῡντᾰ", _, "~οῡντᾰς";
        adverb: "~ουντως";
    ),
    // Εἰκως: 46 words (λελῠκως, κεχηνως, βεβηκως, εἰδως, πεπαιδευκως, ἠγᾰπηκως).
    declension!("εἰκως", 46, "~ως";
        nominative_masculine: "~ως", "~οτε", "~οτες";
        nominative_feminine: "~υῑᾰ", "~υιᾱ", "~υῑαι";
        nominative_neuter: "~ος", _, "~οτᾰ";
        genitive_masculine: "~οτος", "~οτοιν", "~οτων";
        genitive_feminine: "~υιᾱς", "~υιαιν", "~υιων";
        dative_masculine: "~οτῐ", _, "~οσῐ|~οσῐν";
        dative_feminine: "~υιᾳ", _, "~υιαις";
        dative_neuter: _, _, "~οσῐ|~οσῐν";
        accusative_masculine: "~οτᾰ", _, "~οτᾰς";
        accusative_feminine: "~υῑᾰν", _, "~υιᾱς";
        vocative_masculine: "~ως", _, _;
        vocative_feminine: "~υῑᾰ", _, _;
        adverb: "~οτως";
    ),
    // Μαυρος: 46 words (μαυρος, σῠνεργος, ἀγωγος, γεωργος, σῠνᾰπτος, ἐνεργος). Like Ἀσημος, except in 6 cells: genitive_masculine sg, genitive_masculine du, genitive_feminine sg, genitive_feminine du, dative_masculine pl, dative_feminine pl.
    declension!("μαυρος", 46, "~ος", like Ἀσημος;
        genitive_masculine: "~οῡ", "~οῑν", _;
        genitive_feminine: "~οῡ", "~οῑν", _;
        dative_masculine: _, _, "~οῑς";
        dative_feminine: _, _, "~οῑς";
    ),
    // Πλειων: 34 words (σωφρων, ἀκμων, βελτῑων, κᾰκοδαιμων, μειων, ὑπερφρων).
    declension!("πλειων", 34, "~ων";
        nominative_masculine: "~ων", "~ονε", "~ονες";
        nominative_feminine: "~ων", "~ονε", "~ονες";
        nominative_neuter: "~ον", _, "~ονᾰ";
        genitive_masculine: "~ονος", "~ονοιν", "~ονων";
        genitive_feminine: "~ονος", "~ονοιν", "~ονων";
        dative_masculine: "~ονῐ", _, "~οσῐ|~οσῐν";
        dative_feminine: "~ονῐ", _, "~οσῐ|~οσῐν";
        dative_neuter: _, _, "~οσῐ|~οσῐν";
        accusative_masculine: "~ονᾰ", _, "~ονᾰς";
        accusative_feminine: "~ονᾰ", _, "~ονᾰς";
        vocative_masculine: "~ον", _, _;
        vocative_feminine: "~ον", _, _;
        comparative: "~ονεστερος";
        superlative: "~ονεστᾰτος";
        adverb: "~ονως";
    ),
    // Ζων: 33 words (ὠν, νῑκων, τῑμων, δῐαιτων, θεων, αὐδων). Like Πληγεις, except in 15 cells: nominative_masculine sg, nominative_feminine sg, nominative_feminine du, nominative_feminine pl, genitive_feminine sg, genitive_feminine du, ….
    declension!("ζων", 33, "~ν", like Πληγεις;
        nominative_masculine: "~ν", _, _;
        nominative_feminine: "~σᾰ", "~σᾱ", "~σαι";
        genitive_feminine: "~σης", "~σαιν", "~σων";
        dative_masculine: _, _, "~σῐ|~σῐν";
        dative_feminine: "~σῃ", _, "~σαις";
        dative_neuter: _, _, "~σῐ|~σῐν";
        accusative_feminine: "~σᾰν", _, "~σᾱς";
        vocative_masculine: "~ν", _, _;
        vocative_feminine: "~σᾰ", _, _;
    ),
    // Φωνηεις: 28 words (χᾰρῐεις, φωνηεις, αἱμᾰτοεις, ῥοδοεις, ἰχθυοεις, φθογγηεις). Like Πληγεις, except in 16 cells: nominative_feminine sg, nominative_feminine du, nominative_feminine pl, genitive_feminine sg, genitive_feminine du, genitive_feminine pl, ….
    declension!("φωνηεις", 28, "~ις", like Πληγεις;
        nominative_feminine: "~σσᾰ", "~σσᾱ", "~σσαι";
        genitive_feminine: "~σσης", "~σσαιν", "~σσων";
        dative_masculine: _, _, "~ισῐ|~ισῐν";
        dative_feminine: "~σσῃ", _, "~σσαις";
        dative_neuter: _, _, "~ισῐ|~ισῐν";
        accusative_feminine: "~σσᾰν", _, "~σσᾱς";
        vocative_masculine: "~ν", _, _;
        vocative_feminine: "~σσᾰ", _, _;
        comparative: "~στερος";
        superlative: "~στᾰτος";
    ),
    // Ἀγνως: 25 words (μεγᾰς, δρομᾰς, ἐϋς, ἀγνως, ὀκρῐς, νομᾰς).
    declension!("ἀγνως", 25, "~ς";
        nominative_masculine: "~ς", "~δε", "~δες";
        nominative_feminine: "~ς", "~δε", "~δες";
        nominative_neuter: "~", _, "~δᾰ";
        genitive_masculine: "~δος", "~δοιν", "~δων";
        genitive_feminine: "~δος", "~δοιν", "~δων";
        dative_masculine: "~δῐ", _, "~σῐ";
        dative_feminine: "~δῐ", _, "~σῐ";
        accusative_masculine: "~δᾰ", _, "~δᾰς";
        accusative_feminine: "~δᾰ", _, "~δᾰς";
        vocative_masculine: "~ς", _, _;
        vocative_feminine: "~ς", _, _;
        adverb: "~δως";
    ),
    // Ἀκαρπος: 25 words (ἀναπολογητος, ἀκαρπος, ἀδηφαγος, ἀναπαιστος, ἀνανδρος, ἀγαθοποιος).
    declension!("ἀκαρπος", 25, "~α~ος";
        nominative_masculine: "~ᾰ~ος", "~ᾰ~ω", "~ᾰ~οι";
        nominative_feminine: "~ᾰ~ος", "~ᾰ~ω", "~ᾰ~οι";
        nominative_neuter: "~ᾰ~ον", _, "~ᾰ~ᾰ";
        genitive_masculine: "~ᾰ~ου", "~ᾰ~οιν", "~ᾰ~ων";
        genitive_feminine: "~ᾰ~ου", "~ᾰ~οιν", "~ᾰ~ων";
        dative_masculine: "~ᾰ~ῳ", _, "~ᾰ~οις";
        dative_feminine: "~ᾰ~ῳ", _, "~ᾰ~οις";
        accusative_masculine: "~ᾰ~ον", _, "~ᾰ~ους";
        accusative_feminine: "~ᾰ~ον", _, "~ᾰ~ους";
        vocative_masculine: "~ᾰ~ε", _, _;
        vocative_feminine: "~ᾰ~ε", _, _;
        comparative: "~ᾰ~οτερος";
        superlative: "~ᾰ~οτᾰτος";
        adverb: "~ᾰ~ως";
    ),
    // Τρᾱχῠς: 24 words (τᾰχῠς, βρᾰδῠς, πολῠς, βᾰθῠς, ὀξῠς, ὠκῠς). Like Ἀτελης, except in 22 cells: nominative_masculine sg, nominative_feminine sg, nominative_feminine du, nominative_feminine pl, nominative_neuter sg, nominative_neuter pl, ….
    declension!("τρᾱχῠς", 24, "~ῠς", like Ἀτελης;
        nominative_masculine: "~ῠς", _, _;
        nominative_feminine: "~εῑᾰ", "~ειᾱ", "~εῑαι";
        nominative_neuter: "~ῠ", _, "~εᾰ";
        genitive_masculine: "~εος", "~εοιν", "~εων";
        genitive_feminine: "~ειᾱς", "~ειαιν", "~ειων";
        dative_feminine: "~ειᾳ", _, "~ειαις";
        accusative_masculine: "~ῠν", _, _;
        accusative_feminine: "~εῑᾰν", _, "~ειᾱς";
        vocative_masculine: "~ῠ", _, _;
        vocative_feminine: "~εῑᾰ", _, _;
        comparative: "~ῠτερος";
        superlative: "~ῠτᾰτος";
        adverb: "~εως";
    ),
    // Βαινων: 23 words (φαινων, ἀγειρων, μενεαινων, χαιρων, λειβων, ἀπολειπων).
    declension!("βαινων", 23, "~ι~ων";
        nominative_masculine: "~ι~ων", "~ι~οντε", "~ι~οντες";
        nominative_feminine: "~ι~ουσᾰ", "~ι~ουσᾱ", "~ι~ουσαι";
        nominative_neuter: "~ῑ~ον", _, "~ι~οντᾰ";
        genitive_masculine: "~ι~οντος", "~ι~οντοιν", "~ι~οντων";
        genitive_feminine: "~ι~ουσης", "~ι~ουσαιν", "~ι~ουσων";
        dative_masculine: "~ι~οντῐ", _, "~ι~ουσῐ|~ι~ουσῐν";
        dative_feminine: "~ι~ουσῃ", _, "~ι~ουσαις";
        dative_neuter: _, _, "~ι~ουσῐ|~ι~ουσῐν";
        accusative_masculine: "~ι~οντᾰ", _, "~ι~οντᾰς";
        accusative_feminine: "~ι~ουσᾰν", _, "~ι~ουσᾱς";
        vocative_masculine: "~ι~ων", _, _;
        vocative_feminine: "~ι~ουσᾰ", _, _;
        adverb: "~ι~οντως";
    ),
    // Ἀρχικος: 21 words (ἀριστερος, ἀττικος, ἀποπληκτικος, ὑλικος, ἱστορικος, ὑποθετικος).
    declension!("ἀρχικος", 21, "~ι~ος";
        nominative_masculine: "~ῐ~ος", "~ῐ~ω", "~ῐ~οι";
        nominative_feminine: "~ῐ~η", "~ῐ~ᾱ", "~ῐ~αι";
        nominative_neuter: "~ῐ~ον", _, "~ῐ~ᾰ";
        genitive_masculine: "~ῐ~οῡ", "~ῐ~οῑν", "~ῐ~ων";
        genitive_feminine: "~ῐ~ης", "~ῐ~αῑν", "~ῐ~ων";
        dative_masculine: "~ῐ~ῳ", _, "~ῐ~οῑς";
        dative_feminine: "~ῐ~ῃ", _, "~ῐ~αῑς";
        accusative_masculine: "~ῐ~ον", _, "~ῐ~ους";
        accusative_feminine: "~ῐ~ην", _, "~ῐ~ᾱς";
        vocative_masculine: "~ῐ~ε", _, _;
        vocative_feminine: "~ῐ~η", _, _;
        comparative: "~ῐ~ωτερος";
        superlative: "~ῐ~ωτᾰτος";
        adverb: "~ῐ~ως";
    ),
    // Ἡσσων: 20 words (βρᾰχῑων, μᾰσσων, ἐλᾰσσων, κερδιων, ἐλαττων, θᾱττων). Like Πλειων, except in 14 cells: nominative_masculine pl, nominative_feminine pl, nominative_neuter pl, accusative_masculine sg, accusative_masculine pl, accusative_feminine sg, ….
    declension!("ἡσσων", 20, "~ων", like Πλειων;
        nominative_masculine: _, _, "~ονες|~ους";
        nominative_feminine: _, _, "~ονες|~ους";
        nominative_neuter: _, _, "~ονᾰ|~ω";
        accusative_masculine: "~ονᾰ|~ω", _, "~ονᾰς|~ους";
        accusative_feminine: "~ονᾰ|~ω", _, "~ονᾰς|~ους";
        accusative_neuter: _, _, "~ονᾰ|~ω";
        vocative_masculine: _, _, "~ονες|~ους";
        vocative_feminine: _, _, "~ονες|~ους";
        vocative_neuter: _, _, "~ονᾰ|~ω";
        comparative: derived;
        superlative: "~στος";
        adverb: "~ον";
    ),
    // Κᾰκοηθης: 19 words (πληρης, σῠνηθης, τρῐηρης, ἀηθης, κᾰκοηθης, κᾰλοηθης). Like Ἀτελης, except in 12 cells: nominative_masculine du, nominative_masculine pl, nominative_feminine du, nominative_feminine pl, genitive_masculine sg, genitive_masculine du, ….
    declension!("κᾰκοηθης", 19, "~ης", like Ἀτελης;
        nominative_masculine: _, "~ει", "~εις";
        nominative_feminine: _, "~ει", "~εις";
        genitive_masculine: "~ους", "~οιν", _;
        genitive_feminine: "~ους", "~οιν", _;
        dative_masculine: "~ει", _, _;
        dative_feminine: "~ει", _, _;
        accusative_masculine: _, _, "~εις";
        accusative_feminine: _, _, "~εις";
    ),
    // Παυων: 19 words (περῐοδευων, παιδευων, ἀγγαρευων, στρᾰτευων, κελευων, ἀκουων).
    declension!("παυων", 19, "~υων";
        nominative_masculine: "~υων", "~υοντε", "~υοντες";
        nominative_feminine: "~υουσᾰ", "~υουσᾱ", "~υουσαι";
        nominative_neuter: "~ῡον", _, "~υοντᾰ";
        genitive_masculine: "~υοντος", "~υοντοιν", "~υοντων";
        genitive_feminine: "~υουσης", "~υουσαιν", "~υουσων";
        dative_masculine: "~υοντῐ", _, "~υουσῐ|~υουσῐν";
        dative_feminine: "~υουσῃ", _, "~υουσαις";
        dative_neuter: _, _, "~υουσῐ|~υουσῐν";
        accusative_masculine: "~υοντᾰ", _, "~υοντᾰς";
        accusative_feminine: "~υουσᾰν", _, "~υουσᾱς";
        vocative_masculine: "~υων", _, _;
        vocative_feminine: "~υουσᾰ", _, _;
        adverb: "~υοντως";
    ),
    // Ἀδιψος: 19 words (ἀδικος, ἀγριελαιος, ἀντιλογος, ἀοριστος, ἀμαυροστικτος, ἀξιολογος). Like Ἀρχικος, except in 13 cells: nominative_feminine sg, nominative_feminine du, nominative_feminine pl, genitive_masculine sg, genitive_masculine du, genitive_feminine sg, ….
    declension!("ἀδιψος", 19, "~ι~ος", like Ἀρχικος;
        nominative_feminine: "~ῐ~ος", "~ῐ~ω", "~ῐ~οι";
        genitive_masculine: "~ῐ~ου", "~ῐ~οιν", _;
        genitive_feminine: "~ῐ~ου", "~ῐ~οιν", _;
        dative_masculine: _, _, "~ῐ~οις";
        dative_feminine: "~ῐ~ῳ", _, "~ῐ~οις";
        accusative_feminine: "~ῐ~ον", _, "~ῐ~ους";
        vocative_feminine: "~ῐ~ε", _, _;
    ),
    // Δειξᾱς: 17 words (ἀγειρᾱς, ἀειρᾱς, δεισᾱς, μεινᾱς, κτεινᾱς, σπεισᾱς).
    declension!("δειξᾱς", 17, "~ι~ᾱς";
        nominative_masculine: "~ι~ᾱς", "~ι~ᾰντε", "~ι~ᾰντες";
        nominative_feminine: "~ι~ᾱσᾰ", "~ι~ᾱσᾱ", "~ι~ᾱσαι";
        nominative_neuter: "~ῑ~ᾰν", _, "~ι~ᾰντᾰ";
        genitive_masculine: "~ι~ᾰντος", "~ι~ᾰντοιν", "~ι~ᾰντων";
        genitive_feminine: "~ι~ᾱσης", "~ι~ᾱσαιν", "~ι~ᾱσων";
        dative_masculine: "~ι~ᾰντῐ", _, "~ι~ᾱσῐ|~ι~ᾱσῐν";
        dative_feminine: "~ι~ᾱσῃ", _, "~ι~ᾱσαις";
        dative_neuter: _, _, "~ι~ᾱσῐ|~ι~ᾱσῐν";
        accusative_masculine: "~ι~ᾰντᾰ", _, "~ι~ᾰντᾰς";
        accusative_feminine: "~ι~ᾱσᾰν", _, "~ι~ᾱσᾱς";
        vocative_masculine: "~ι~ᾱς", _, _;
        vocative_feminine: "~ι~ᾱσᾰ", _, _;
        adverb: "~ι~ᾰντως";
    ),
    // Ἀριστος: 16 words (ὑψιστος, ἀερινος, ἀριστος, ἀτειχιστος, ἀξιουμενος, ὑμενινος). Like Ἀρχικος, except in 5 cells: genitive_masculine sg, genitive_masculine du, genitive_feminine du, dative_masculine pl, dative_feminine pl.
    declension!("ἀριστος", 16, "~ι~ος", like Ἀρχικος;
        genitive_masculine: "~ῐ~ου", "~ῐ~οιν", _;
        genitive_feminine: _, "~ῐ~αιν", _;
        dative_masculine: _, _, "~ῐ~οις";
        dative_feminine: _, _, "~ῐ~αις";
    ),
    // Ἐρημος: 15 words (ἀλεξανδρῑνος, λᾰτῑνος, πεμπτος, τετᾰρτος, τρῐτος, λῐσπος). Like Ἰσος, except in 2 cells: comparative, superlative.
    declension!("ἐρημος", 15, "~ος", like Ἰσος;
        comparative: "~οτερος";
        superlative: "~οτᾰτος";
    ),
    // Ὑπατος: 15 words (ὑπατος, ὑπογεγραμμενος, ἀνεσκαμμενος, ἀγαθοποιησομενος, ἀγγειλαμενος, ἱσταμενος). Like Ἀκαρπος, except in 12 cells: nominative_feminine sg, nominative_feminine du, nominative_feminine pl, genitive_feminine sg, genitive_feminine du, dative_feminine sg, ….
    declension!("ὑπατος", 15, "~α~ος", like Ἀκαρπος;
        nominative_feminine: "~ᾰ~η", "~ᾰ~ᾱ", "~ᾰ~αι";
        genitive_feminine: "~ᾰ~ης", "~ᾰ~αιν", _;
        dative_feminine: "~ᾰ~ῃ", _, "~ᾰ~αις";
        accusative_feminine: "~ᾰ~ην", _, "~ᾰ~ᾱς";
        vocative_feminine: "~ᾰ~η", _, _;
        comparative: "~ᾰ~ωτερος";
        superlative: "~ᾰ~ωτᾰτος";
    ),
    // Τρῐπους: 13 words (τετρᾰπους, ὀκτωπους, τρῐπους, ἰθυς, ἀμφιβραχυς, ὀκτᾰπους). Like Ἀγνως, except in 8 cells: nominative_masculine sg, nominative_feminine sg, nominative_neuter sg, dative_masculine pl, dative_feminine pl, dative_neuter pl, ….
    declension!("τρῐπους", 13, "~υς", like Ἀγνως;
        nominative_masculine: "~υς", _, _;
        nominative_feminine: "~υς", _, _;
        nominative_neuter: "~υν", _, _;
        dative_masculine: _, _, "~σῐ|~σῐν";
        dative_feminine: _, _, "~σῐ|~σῐν";
        dative_neuter: _, _, "~σῐ|~σῐν";
        vocative_masculine: "~υς", _, _;
        vocative_feminine: "~υς", _, _;
    ),
    // Ἀγαθος: 13 words (ἀγαθος, ἀγαπητος, ῥωμαϊκος, ἱλαρος, ἀλᾰπαδνος, ἱκανος). Like Ὑπατος, except in 5 cells: genitive_masculine sg, genitive_masculine du, genitive_feminine du, dative_masculine pl, dative_feminine pl.
    declension!("ἀγαθος", 13, "~α~ος", like Ὑπατος;
        genitive_masculine: "~ᾰ~οῡ", "~ᾰ~οῑν", _;
        genitive_feminine: _, "~ᾰ~αῑν", _;
        dative_masculine: _, _, "~ᾰ~οῑς";
        dative_feminine: _, _, "~ᾰ~αῑς";
    ),
    // Ἀβατος: 12 words (ἀγραφος, ἀκεφαλος, ἀγναφος, ἀνωμαλος, ἀναλογος, ἀβατος). Like Ἀκαρπος, except in 2 cells: comparative, superlative.
    declension!("ἀβατος", 12, "~α~ος", like Ἀκαρπος;
        comparative: "~ᾰ~ωτερος";
        superlative: "~ᾰ~ωτᾰτος";
    ),
    // Ἀρτιος: 12 words (ἁγιος, ἀγριος, ἀξιος, ἀρτιος, ἀλλοτριος, ἁλιος).
    declension!("ἀρτιος", 12, "~ιος";
        nominative_masculine: "~ῐος", "~ῐω", "~ῐοι";
        nominative_feminine: "~ῐᾱ", "~ῐᾱ", "~ῐαι";
        nominative_neuter: "~ῐον", _, "~ῐᾰ";
        genitive_masculine: "~ῐου", "~ῐοιν", "~ῐων";
        genitive_feminine: "~ῐᾱς", "~ῐαιν", "~ῐων";
        dative_masculine: "~ῐῳ", _, "~ῐοις";
        dative_feminine: "~ῐᾳ", _, "~ῐαις";
        accusative_masculine: "~ῐον", _, "~ῐους";
        accusative_feminine: "~ῐᾱν", _, "~ῐᾱς";
        vocative_masculine: "~ῐε", _, _;
        vocative_feminine: "~ῐᾱ", _, _;
        comparative: "~ῐωτερος";
        superlative: "~ῐωτᾰτος";
        adverb: "~ῐως";
    ),
    // Τοιοσδε: 12 words (ἀρρην, τοιοσδε, τοσοσδε, δᾰϊφρων, λῡομενη, βοτηρ). Like Ἐμπεδος, except in 21 cells: nominative_masculine sg, nominative_masculine du, nominative_masculine pl, nominative_feminine sg, nominative_feminine du, nominative_feminine pl, ….
    declension!("τοιοσδε", 12, "~", like Ἐμπεδος;
        nominative_masculine: "~", "~ε", "~ες";
        nominative_feminine: "~", "~ε", "~ες";
        nominative_neuter: "~", _, _;
        genitive_masculine: "~ος", _, _;
        genitive_feminine: "~ος", _, _;
        dative_masculine: "~ῐ", _, "~σῐ";
        dative_feminine: "~ῐ", _, "~σῐ";
        accusative_masculine: "~ᾰ", _, "~ᾰς";
        accusative_feminine: "~ᾰ", _, "~ᾰς";
        vocative_masculine: "~", _, _;
        vocative_feminine: "~", _, _;
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
        nominative_masculine: "~ῡ~ος", "~υ~ω", "~ῡ~οι";
        nominative_feminine: "~υ~η", "~υ~ᾱ", "~ῡ~αι";
        nominative_neuter: "~ῡ~ον", _, "~ῡ~ᾰ";
        genitive_masculine: "~υ~ου", "~υ~οιν", "~υ~ων";
        genitive_feminine: "~υ~ης", "~υ~αιν", "~υ~ων";
        dative_masculine: "~υ~ῳ", _, "~υ~οις";
        dative_feminine: "~υ~ῃ", _, "~υ~αις";
        accusative_masculine: "~ῡ~ον", _, "~υ~ους";
        accusative_feminine: "~υ~ην", _, "~υ~ᾱς";
        vocative_masculine: "~ῡ~ε", _, _;
        vocative_feminine: "~υ~η", _, _;
        comparative: "~υ~οτερος";
        superlative: "~υ~οτᾰτος";
        adverb: "~υ~ως";
    ),
    // Ὑποτακτικος: 11 words (ἀμφαριστερος, ὑπατικος, ἀναλογητικος, ἀναγιγνωσκομενος, ὑποτακτικος, ἱμαντινος).
    declension!("ὑποτακτικος", 11, "~α~ι~ος";
        nominative_masculine: "~ᾰ~ῐ~ος", "~ᾰ~ῐ~ω", "~ᾰ~ῐ~οι";
        nominative_feminine: "~ᾰ~ῐ~η", "~ᾰ~ῐ~ᾱ", "~ᾰ~ῐ~αι";
        nominative_neuter: "~ᾰ~ῐ~ον", _, "~ᾰ~ῐ~ᾰ";
        genitive_masculine: "~ᾰ~ῐ~οῡ", "~ᾰ~ῐ~οῑν", "~ᾰ~ῐ~ων";
        genitive_feminine: "~ᾰ~ῐ~ης", "~ᾰ~ῐ~αῑν", "~ᾰ~ῐ~ων";
        dative_masculine: "~ᾰ~ῐ~ῳ", _, "~ᾰ~ῐ~οῑς";
        dative_feminine: "~ᾰ~ῐ~ῃ", _, "~ᾰ~ῐ~αῑς";
        accusative_masculine: "~ᾰ~ῐ~ον", _, "~ᾰ~ῐ~ους";
        accusative_feminine: "~ᾰ~ῐ~ην", _, "~ᾰ~ῐ~ᾱς";
        vocative_masculine: "~ᾰ~ῐ~ε", _, _;
        vocative_feminine: "~ᾰ~ῐ~η", _, _;
        comparative: "~ᾰ~ῐ~ωτερος";
        superlative: "~ᾰ~ῐ~ωτᾰτος";
        adverb: "~ᾰ~ῐ~ως";
    ),
    // Γαῡρος: 10 words (πᾰνοῡργος, λαῡρος, θοῡρος, γαῡρος, κᾰκοῡργος, σκηπτοῡχος). Like Φαῡλος, except in 10 cells: nominative_feminine sg, nominative_feminine du, nominative_feminine pl, genitive_feminine sg, genitive_feminine du, dative_feminine sg, ….
    declension!("γαῡρος", 10, "~ῡ~ος", like Φαῡλος;
        nominative_feminine: "~ῡ~ος", "~υ~ω", "~ῡ~οι";
        genitive_feminine: "~υ~ου", "~υ~οιν", _;
        dative_feminine: "~υ~ῳ", _, "~υ~οις";
        accusative_feminine: "~ῡ~ον", _, "~υ~ους";
        vocative_feminine: "~ῡ~ε", _, _;
    ),
    // Ἀμαθης: 10 words (ἀπαθης, ἀμαθης, ἀσφαλης, ἀφανης, ἀσαφης, ἀραχνοειδης). Like Ἀγαθος, except in 24 cells: nominative_masculine sg, nominative_masculine du, nominative_masculine pl, nominative_feminine sg, nominative_feminine du, nominative_feminine pl, ….
    declension!("ἀμαθης", 10, "~α~ης", like Ἀγαθος;
        nominative_masculine: "~ᾰ~ης", "~ᾰ~εῑ", "~ᾰ~εῑς";
        nominative_feminine: "~ᾰ~ης", "~ᾰ~εῑ", "~ᾰ~εῑς";
        nominative_neuter: "~ᾰ~ες", _, "~ᾰ~η";
        genitive_masculine: "~ᾰ~οῡς", _, _;
        genitive_feminine: "~ᾰ~οῡς", "~ᾰ~οῑν", _;
        dative_masculine: "~ᾰ~εῑ", _, "~ᾰ~εσῐ|~ᾰ~εσῐν";
        dative_feminine: "~ᾰ~εῑ", _, "~ᾰ~εσῐ|~ᾰ~εσῐν";
        dative_neuter: _, _, "~ᾰ~εσῐ|~ᾰ~εσῐν";
        accusative_masculine: "~ᾰ~η", _, "~ᾰ~εῑς";
        accusative_feminine: "~ᾰ~η", _, "~ᾰ~εῑς";
        vocative_masculine: "~ᾰ~ες", _, _;
        vocative_feminine: "~ᾰ~ες", _, _;
        comparative: "~ᾰ~εστερος";
        superlative: "~ᾰ~εστᾰτος";
    ),
    // Ἀμυμων: 10 words (ἀμυμων, παιδευσων, ἀγγαρευσων, φευγων, ἀγορευσων, πολῑτευσων).
    declension!("ἀμυμων", 10, "~υ~ων";
        nominative_masculine: "~υ~ων", "~υ~οντε", "~υ~οντες";
        nominative_feminine: "~υ~ουσᾰ", "~υ~ουσᾱ", "~υ~ουσαι";
        nominative_neuter: "~ῡ~ον", _, "~υ~οντᾰ";
        genitive_masculine: "~υ~οντος", "~υ~οντοιν", "~υ~οντων";
        genitive_feminine: "~υ~ουσης", "~υ~ουσαιν", "~υ~ουσων";
        dative_masculine: "~υ~οντῐ", _, "~υ~ουσῐ|~υ~ουσῐν";
        dative_feminine: "~υ~ουσῃ", _, "~υ~ουσαις";
        dative_neuter: _, _, "~υ~ουσῐ|~υ~ουσῐν";
        accusative_masculine: "~υ~οντᾰ", _, "~υ~οντᾰς";
        accusative_feminine: "~υ~ουσᾰν", _, "~υ~ουσᾱς";
        vocative_masculine: "~υ~ων", _, _;
        vocative_feminine: "~υ~ουσᾰ", _, _;
        adverb: "~υ~οντως";
    ),
    // Φᾰνερος: 9 words (στερεος, σᾰρωνῐκος, ἐτεος, πολῐος, σκολῐος, πελῐος). Like Λᾰμπρος, except in 2 cells: comparative, superlative.
    declension!("φᾰνερος", 9, "~ος", like Λᾰμπρος;
        comparative: "~ωτερος";
        superlative: "~ωτᾰτος";
    ),
    // Ἀξιωτερος: 9 words (ἰδιωτερος, ἀκλιτωτερος, ἀπιστοτερος, ἀγριωτερος, ἀξιωτερος, ἀθλιωτερος). Like Ἀριστος, except in 8 cells: nominative_feminine sg, genitive_feminine sg, dative_feminine sg, accusative_feminine sg, vocative_feminine sg, comparative, ….
    declension!("ἀξιωτερος", 9, "~ι~ος", like Ἀριστος;
        nominative_feminine: "~ῐ~ᾱ", _, _;
        genitive_feminine: "~ῐ~ᾱς", _, _;
        dative_feminine: "~ῐ~ᾳ", _, _;
        accusative_feminine: "~ῐ~ᾱν", _, _;
        vocative_feminine: "~ῐ~ᾱ", _, _;
        comparative: derived;
        superlative: derived;
        adverb: "~ῐ~ον";
    ),
    // Ἀξιωτατος: 9 words (ἰδιωτατος, ἀκλιτωτατος, ἀλλοτριωτατος, ἀγριωτατος, ἀξιωτατος, ἀθλιωτατος).
    declension!("ἀξιωτατος", 9, "~ι~α~ος";
        nominative_masculine: "~ῐ~ᾰ~ος", "~ῐ~ᾰ~ω", "~ῐ~ᾰ~οι";
        nominative_feminine: "~ῐ~ᾰ~η", "~ῐ~ᾰ~ᾱ", "~ῐ~ᾰ~αι";
        nominative_neuter: "~ῐ~ᾰ~ον", _, "~ῐ~ᾰ~ᾰ";
        genitive_masculine: "~ῐ~ᾰ~ου", "~ῐ~ᾰ~οιν", "~ῐ~ᾰ~ων";
        genitive_feminine: "~ῐ~ᾰ~ης", "~ῐ~ᾰ~αιν", "~ῐ~ᾰ~ων";
        dative_masculine: "~ῐ~ᾰ~ῳ", _, "~ῐ~ᾰ~οις";
        dative_feminine: "~ῐ~ᾰ~ῃ", _, "~ῐ~ᾰ~αις";
        accusative_masculine: "~ῐ~ᾰ~ον", _, "~ῐ~ᾰ~ους";
        accusative_feminine: "~ῐ~ᾰ~ην", _, "~ῐ~ᾰ~ᾱς";
        vocative_masculine: "~ῐ~ᾰ~ε", _, _;
        vocative_feminine: "~ῐ~ᾰ~η", _, _;
        adverb: "~ῐ~ᾰ~ᾰ";
    ),
    // Κᾰλλῐθρῐξ: 9 words (ἡλῐξ, κᾰλλῐθρῐξ, πτωξ, ὁμηλῐξ, χρῡσᾰμπῠξ, μελᾰνθρῐξ).
    declension!("κᾰλλῐθρῐξ", 9, "~ξ";
        nominative_masculine: "~ξ", "~κε", "~κες";
        nominative_feminine: "~ξ", "~κε", "~κες";
        nominative_neuter: "~ξ", _, "~κᾰ";
        genitive_masculine: "~κος", "~κοιν", "~κων";
        genitive_feminine: "~κος", "~κοιν", "~κων";
        dative_masculine: "~κῐ", _, "~ξῐ|~ξῐν";
        dative_feminine: "~κῐ", _, "~ξῐ|~ξῐν";
        dative_neuter: _, _, "~ξῐ|~ξῐν";
        accusative_masculine: "~κᾰ", _, "~κᾰς";
        accusative_feminine: "~κᾰ", _, "~κᾰς";
        vocative_masculine: "~ξ", _, _;
        vocative_feminine: "~ξ", _, _;
        adverb: "~κως";
    ),
    // Ζευξᾱς: 9 words (παιδευσᾱς, ἀγγαρευσᾱς, στρᾰτευσᾱς, πλευσᾱς, ἀπουρᾱς, ὀρουσᾱς).
    declension!("ζευξᾱς", 9, "~υ~ᾱς";
        nominative_masculine: "~υ~ᾱς", "~υ~ᾰντε", "~υ~ᾰντες";
        nominative_feminine: "~υ~ᾱσᾰ", "~υ~ᾱσᾱ", "~υ~ᾱσαι";
        nominative_neuter: "~ῡ~ᾰν", _, "~υ~ᾰντᾰ";
        genitive_masculine: "~υ~ᾰντος", "~υ~ᾰντοιν", "~υ~ᾰντων";
        genitive_feminine: "~υ~ᾱσης", "~υ~ᾱσαιν", "~υ~ᾱσων";
        dative_masculine: "~υ~ᾰντῐ", _, "~υ~ᾱσῐ|~υ~ᾱσῐν";
        dative_feminine: "~υ~ᾱσῃ", _, "~υ~ᾱσαις";
        dative_neuter: _, _, "~υ~ᾱσῐ|~υ~ᾱσῐν";
        accusative_masculine: "~υ~ᾰντᾰ", _, "~υ~ᾰντᾰς";
        accusative_feminine: "~υ~ᾱσᾰν", _, "~υ~ᾱσᾱς";
        vocative_masculine: "~υ~ᾱς", _, _;
        vocative_feminine: "~υ~ᾱσᾰ", _, _;
        adverb: "~υ~ᾰντως";
    ),
    // Δαϊζων: 9 words (ἀγαθοεργεων, ἀγαθοποιων, ἀγαθοποιησων, ἀκμαζων, ἁμαρτων, ὑπαρχων).
    declension!("δαϊζων", 9, "~α~ων";
        nominative_masculine: "~ᾰ~ων", "~ᾰ~οντε", "~ᾰ~οντες";
        nominative_feminine: "~ᾰ~ουσᾰ", "~ᾰ~ουσᾱ", "~ᾰ~ουσαι";
        nominative_neuter: "~ᾰ~ον", _, "~ᾰ~οντᾰ";
        genitive_masculine: "~ᾰ~οντος", "~ᾰ~οντοιν", "~ᾰ~οντων";
        genitive_feminine: "~ᾰ~ουσης", "~ᾰ~ουσαιν", "~ᾰ~ουσων";
        dative_masculine: "~ᾰ~οντῐ", _, "~ᾰ~ουσῐ|~ᾰ~ουσῐν";
        dative_feminine: "~ᾰ~ουσῃ", _, "~ᾰ~ουσαις";
        dative_neuter: _, _, "~ᾰ~ουσῐ|~ᾰ~ουσῐν";
        accusative_masculine: "~ᾰ~οντᾰ", _, "~ᾰ~οντᾰς";
        accusative_feminine: "~ᾰ~ουσᾰν", _, "~ᾰ~ουσᾱς";
        vocative_masculine: "~ᾰ~ων", _, _;
        vocative_feminine: "~ᾰ~ουσᾰ", _, _;
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
        nominative_masculine: "~ιων", "~ιοντε", "~ιοντες";
        nominative_feminine: "~ιουσᾰ", "~ιουσᾱ", "~ιουσαι";
        nominative_neuter: "~ῑον", _, "~ιοντᾰ";
        genitive_masculine: "~ιοντος", "~ιοντοιν", "~ιοντων";
        genitive_feminine: "~ιουσης", "~ιουσαιν", "~ιουσων";
        dative_masculine: "~ιοντῐ", _, "~ιουσῐ|~ιουσῐν";
        dative_feminine: "~ιουσῃ", _, "~ιουσαις";
        dative_neuter: _, _, "~ιουσῐ|~ιουσῐν";
        accusative_masculine: "~ιοντᾰ", _, "~ιοντᾰς";
        accusative_feminine: "~ιουσᾰν", _, "~ιουσᾱς";
        vocative_masculine: "~ιων", _, _;
        vocative_feminine: "~ιουσᾰ", _, _;
        adverb: "~ιοντως";
    ),
    // Ἀκυρος: 8 words (ἰσχυρος, ἀσυλος, θυϊνος, ὑπερθυμος, ἀπυγος, ἀχρυσος). Like Γαῡρος, except in 17 cells: nominative_masculine du, nominative_feminine du, genitive_masculine sg, genitive_masculine du, genitive_masculine pl, genitive_feminine sg, ….
    declension!("ἀκυρος", 8, "~υ~ος", like Γαῡρος;
        nominative_masculine: _, "~ῡ~ω", _;
        nominative_feminine: _, "~ῡ~ω", _;
        genitive_masculine: "~ῡ~ου", "~ῡ~οιν", "~ῡ~ων";
        genitive_feminine: "~ῡ~ου", "~ῡ~οιν", "~ῡ~ων";
        dative_masculine: "~ῡ~ῳ", _, "~ῡ~οις";
        dative_feminine: "~ῡ~ῳ", _, "~ῡ~οις";
        accusative_masculine: _, _, "~ῡ~ους";
        accusative_feminine: _, _, "~ῡ~ους";
        comparative: "~ῡ~οτερος";
        superlative: "~ῡ~οτᾰτος";
        adverb: "~ῡ~ως";
    ),
    // Ἀσυμπτωτος: 7 words (ἀνυποδητος, ἀμεθυστος, ἀρρυθμος, ἀργυροηλος, ὑποκυκλος, ἀγρυπνος).
    declension!("ἀσυμπτωτος", 7, "~υ~ος";
        nominative_masculine: "~ῠ~ος", "~ῠ~ω", "~ῠ~οι";
        nominative_feminine: "~ῠ~ος", "~ῠ~ω", "~ῠ~οι";
        nominative_neuter: "~ῠ~ον", _, "~ῠ~ᾰ";
        genitive_masculine: "~ῠ~ου", "~ῠ~οιν", "~ῠ~ων";
        genitive_feminine: "~ῠ~ου", "~ῠ~οιν", "~ῠ~ων";
        dative_masculine: "~ῠ~ῳ", _, "~ῠ~οις";
        dative_feminine: "~ῠ~ῳ", _, "~ῠ~οις";
        accusative_masculine: "~ῠ~ον", _, "~ῠ~ους";
        accusative_feminine: "~ῠ~ον", _, "~ῠ~ους";
        vocative_masculine: "~ῠ~ε", _, _;
        vocative_feminine: "~ῠ~ε", _, _;
        comparative: "~ῠ~οτερος";
        superlative: "~ῠ~οτᾰτος";
        adverb: "~ῠ~ως";
    ),
    // Ἀσιτος: 7 words (ἀνικητος, ἀλιτρος, ἀφικομενος, ἀκριθος, ἀσιτος, ἀκριβεστερος).
    declension!("ἀσιτος", 7, "~ι~ος";
        nominative_masculine: "~ῑ~ος", "~ῑ~ω", "~ῑ~οι";
        nominative_feminine: "~ῑ~ος", "~ῑ~ω", "~ῑ~οι";
        nominative_neuter: "~ῑ~ον", _, "~ῑ~ᾰ";
        genitive_masculine: "~ῑ~ου", "~ῑ~οιν", "~ῑ~ων";
        genitive_feminine: "~ῑ~ου", "~ῑ~οιν", "~ῑ~ων";
        dative_masculine: "~ῑ~ῳ", _, "~ῑ~οις";
        dative_feminine: "~ῑ~ῳ", _, "~ῑ~οις";
        accusative_masculine: "~ῑ~ον", _, "~ῑ~ους";
        accusative_feminine: "~ῑ~ον", _, "~ῑ~ους";
        vocative_masculine: "~ῑ~ε", _, _;
        vocative_feminine: "~ῑ~ε", _, _;
        comparative: "~ῑ~οτερος";
        superlative: "~ῑ~οτᾰτος";
        adverb: "~ῑ~ως";
    ),
    // Ἀκρατος: 7 words (λαϊκος, ἀγενεαλογητος, λαϊνος, ἀκρατος, ἱερακοπροσωπος, ἱσπανος).
    declension!("ἀκρατος", 7, "~α~ος";
        nominative_masculine: "~ᾱ~ος", "~ᾱ~ω", "~ᾱ~οι";
        nominative_feminine: "~ᾱ~ος", "~ᾱ~ω", "~ᾱ~οι";
        nominative_neuter: "~ᾱ~ον", _, "~ᾱ~ᾰ";
        genitive_masculine: "~ᾱ~ου", "~ᾱ~οιν", "~ᾱ~ων";
        genitive_feminine: "~ᾱ~ου", "~ᾱ~οιν", "~ᾱ~ων";
        dative_masculine: "~ᾱ~ῳ", _, "~ᾱ~οις";
        dative_feminine: "~ᾱ~ῳ", _, "~ᾱ~οις";
        accusative_masculine: "~ᾱ~ον", _, "~ᾱ~ους";
        accusative_feminine: "~ᾱ~ον", _, "~ᾱ~ους";
        vocative_masculine: "~ᾱ~ε", _, _;
        vocative_feminine: "~ᾱ~ε", _, _;
        comparative: "~ᾱ~οτερος";
        superlative: "~ᾱ~οτᾰτος";
        adverb: "~ᾱ~ως";
    ),
    // Ἀξῐοχρεως: 7 words (ἱλεως, λεπτογεως, ἀξῐοχρεως, σως, μονοκερως, πλεως). Like Ἐμπεδος, except in 17 cells: nominative_masculine sg, nominative_masculine pl, nominative_feminine sg, nominative_feminine pl, nominative_neuter sg, genitive_masculine sg, ….
    declension!("ἀξῐοχρεως", 7, "~ως", like Ἐμπεδος;
        nominative_masculine: "~ως", _, "~ῳ";
        nominative_feminine: "~ως", _, "~ῳ";
        nominative_neuter: "~ων", _, _;
        genitive_masculine: "~ω", "~ῳν", _;
        genitive_feminine: "~ω", "~ῳν", _;
        dative_masculine: _, _, "~ῳς";
        dative_feminine: _, _, "~ῳς";
        accusative_masculine: "~ω|~ων", _, "~ως";
        accusative_feminine: "~ω|~ων", _, "~ως";
        vocative_masculine: "~ως", _, _;
        vocative_feminine: "~ως", _, _;
    ),
    // Ἀκαθαρτος: 6 words (ἀκαθαρτος, ἀκαταληπτος, ἀκᾰτασκευαστος, ἀταλαντος, ἀναλφαβητος, ἀταρακτος).
    declension!("ἀκαθαρτος", 6, "~α~α~ος";
        nominative_masculine: "~ᾰ~ᾰ~ος", "~ᾰ~ᾰ~ω", "~ᾰ~ᾰ~οι";
        nominative_feminine: "~ᾰ~ᾰ~ος", "~ᾰ~ᾰ~ω", "~ᾰ~ᾰ~οι";
        nominative_neuter: "~ᾰ~ᾰ~ον", _, "~ᾰ~ᾰ~ᾰ";
        genitive_masculine: "~ᾰ~ᾰ~ου", "~ᾰ~ᾰ~οιν", "~ᾰ~ᾰ~ων";
        genitive_feminine: "~ᾰ~ᾰ~ου", "~ᾰ~ᾰ~οιν", "~ᾰ~ᾰ~ων";
        dative_masculine: "~ᾰ~ᾰ~ῳ", _, "~ᾰ~ᾰ~οις";
        dative_feminine: "~ᾰ~ᾰ~ῳ", _, "~ᾰ~ᾰ~οις";
        accusative_masculine: "~ᾰ~ᾰ~ον", _, "~ᾰ~ᾰ~ους";
        accusative_feminine: "~ᾰ~ᾰ~ον", _, "~ᾰ~ᾰ~ους";
        vocative_masculine: "~ᾰ~ᾰ~ε", _, _;
        vocative_feminine: "~ᾰ~ᾰ~ε", _, _;
        comparative: "~ᾰ~ᾰ~οτερος";
        superlative: "~ᾰ~ᾰ~οτᾰτος";
        adverb: "~ᾰ~ᾰ~ως";
    ),
    // Ἰδιος: 6 words (ἰδιος, ὑποχονδριος, ἀποφωλιος, ἀεκουσιος, ἀκηριος, ὑπαιθριος). Like Ἀρτιος, except in 10 cells: nominative_feminine sg, nominative_feminine du, nominative_feminine pl, genitive_feminine sg, genitive_feminine du, dative_feminine sg, ….
    declension!("ἰδιος", 6, "~ιος", like Ἀρτιος;
        nominative_feminine: "~ῐος", "~ῐω", "~ῐοι";
        genitive_feminine: "~ῐου", "~ῐοιν", _;
        dative_feminine: "~ῐῳ", _, "~ῐοις";
        accusative_feminine: "~ῐον", _, "~ῐους";
        vocative_feminine: "~ῐε", _, _;
    ),
    // Ἀπατωρ: 6 words (ἀλεκτωρ, ἀπατωρ, μεγᾰλητωρ, ἀγηνωρ, ὑψηνωρ, ἀμητωρ).
    declension!("ἀπατωρ", 6, "~ω~";
        nominative_masculine: "~ω~", "~ο~ε", "~ο~ες";
        nominative_feminine: "~ω~", "~ο~ε", "~ο~ες";
        nominative_neuter: "~ο~", _, "~ο~ᾰ";
        genitive_masculine: "~ο~ος", "~ο~οιν", "~ο~ων";
        genitive_feminine: "~ο~ος", "~ο~οιν", "~ο~ων";
        dative_masculine: "~ο~ῐ", _, "~ο~σῐ|~ο~σῐν";
        dative_feminine: "~ο~ῐ", _, "~ο~σῐ|~ο~σῐν";
        dative_neuter: _, _, "~ο~σῐ|~ο~σῐν";
        accusative_masculine: "~ο~ᾰ", _, "~ο~ᾰς";
        accusative_feminine: "~ο~ᾰ", _, "~ο~ᾰς";
        vocative_masculine: "~ο~", _, _;
        vocative_feminine: "~ο~", _, _;
        adverb: "~ο~ως";
    ),
    // Ἀρας: 6 words (ἱστας, ἀγγειλας, ἀρας, ἀκουσας, ἀποβας, ἀϊξας). Like Προπᾱς, except in 0 cells: .
    declension!("ἀρας", 6, "~ας", like Προπᾱς;
    ),
    // Φῡς: 6 words (δεικνῡς, ἀργυροῡς, κᾰτᾰδῡς, ἐπῐμειγνῡς, ἀποδῡς, φῡς).
    declension!("φῡς", 6, "~ῡς";
        nominative_masculine: "~ῡς", "~ῠντε", "~ῠντες";
        nominative_feminine: "~ῡσᾰ", "~ῡσᾱ", "~ῡσαι";
        nominative_neuter: "~ῠν", _, "~ῠντᾰ";
        genitive_masculine: "~ῠντος", "~ῠντοιν", "~ῠντων";
        genitive_feminine: "~ῡσης", "~ῡσαιν", "~ῡσων";
        dative_masculine: "~ῠντῐ", _, "~ῡσῐ|~ῡσῐν";
        dative_feminine: "~ῡσῃ", _, "~ῡσαις";
        dative_neuter: _, _, "~ῡσῐ|~ῡσῐν";
        accusative_masculine: "~ῠντᾰ", _, "~ῠντᾰς";
        accusative_feminine: "~ῡσᾰν", _, "~ῡσᾱς";
        vocative_masculine: "~ῡς", _, _;
        vocative_feminine: "~ῡσᾰ", _, _;
        adverb: "~ῠντως";
    ),
    // Δρυϊνος: 6 words (οἰσυϊνος, δρυϊνος, ἀργυρεος, συϊνος, ἀτυζομενος, ἀποδεικνυμενος). Like Ἀσυμπτωτος, except in 12 cells: nominative_feminine sg, nominative_feminine du, nominative_feminine pl, genitive_feminine sg, genitive_feminine du, dative_feminine sg, ….
    declension!("δρυϊνος", 6, "~υ~ος", like Ἀσυμπτωτος;
        nominative_feminine: "~ῠ~η", "~ῠ~ᾱ", "~ῠ~αι";
        genitive_feminine: "~ῠ~ης", "~ῠ~αιν", _;
        dative_feminine: "~ῠ~ῃ", _, "~ῠ~αις";
        accusative_feminine: "~ῠ~ην", _, "~ῠ~ᾱς";
        vocative_feminine: "~ῠ~η", _, _;
        comparative: "~ῠ~ωτερος";
        superlative: "~ῠ~ωτᾰτος";
    ),
    // Ἀγωνιστικος: 5 words (ὑβριστικος, ἀντιθετικος, ἀντιλογητικος, ἀξιοπιστος, ἀγωνιστικος).
    declension!("ἀγωνιστικος", 5, "~ι~ι~ος";
        nominative_masculine: "~ῐ~ῐ~ος", "~ῐ~ῐ~ω", "~ῐ~ῐ~οι";
        nominative_feminine: "~ῐ~ῐ~η", "~ῐ~ῐ~ᾱ", "~ῐ~ῐ~αι";
        nominative_neuter: "~ῐ~ῐ~ον", _, "~ῐ~ῐ~ᾰ";
        genitive_masculine: "~ῐ~ῐ~οῡ", "~ῐ~ῐ~οῑν", "~ῐ~ῐ~ων";
        genitive_feminine: "~ῐ~ῐ~ης", "~ῐ~ῐ~αῑν", "~ῐ~ῐ~ων";
        dative_masculine: "~ῐ~ῐ~ῳ", _, "~ῐ~ῐ~οῑς";
        dative_feminine: "~ῐ~ῐ~ῃ", _, "~ῐ~ῐ~αῑς";
        accusative_masculine: "~ῐ~ῐ~ον", _, "~ῐ~ῐ~ους";
        accusative_feminine: "~ῐ~ῐ~ην", _, "~ῐ~ῐ~ᾱς";
        vocative_masculine: "~ῐ~ῐ~ε", _, _;
        vocative_feminine: "~ῐ~ῐ~η", _, _;
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
        nominative_masculine: "~ῠ~ᾰ~ος", "~ῠ~ᾰ~ω", "~ῠ~ᾰ~οι";
        nominative_feminine: "~ῠ~ᾰ~ος", "~ῠ~ᾰ~ω", "~ῠ~ᾰ~οι";
        nominative_neuter: "~ῠ~ᾰ~ον", _, "~ῠ~ᾰ~ᾰ";
        genitive_masculine: "~ῠ~ᾰ~ου", "~ῠ~ᾰ~οιν", "~ῠ~ᾰ~ων";
        genitive_feminine: "~ῠ~ᾰ~ου", "~ῠ~ᾰ~οιν", "~ῠ~ᾰ~ων";
        dative_masculine: "~ῠ~ᾰ~ῳ", _, "~ῠ~ᾰ~οις";
        dative_feminine: "~ῠ~ᾰ~ῳ", _, "~ῠ~ᾰ~οις";
        accusative_masculine: "~ῠ~ᾰ~ον", _, "~ῠ~ᾰ~ους";
        accusative_feminine: "~ῠ~ᾰ~ον", _, "~ῠ~ᾰ~ους";
        vocative_masculine: "~ῠ~ᾰ~ε", _, _;
        vocative_feminine: "~ῠ~ᾰ~ε", _, _;
        comparative: "~ῠ~ᾰ~ωτερος";
        superlative: "~ῠ~ᾰ~ωτᾰτος";
        adverb: "~ῠ~ᾰ~ως";
    ),
    // Αἰθοψ: 5 words (οἰνοψ, μῠωψ, αἰθοψ, νωροψ, ἠνοψ).
    declension!("αἰθοψ", 5, "~ψ";
        nominative_masculine: "~ψ", "~πε", "~πες";
        nominative_feminine: "~ψ", "~πε", "~πες";
        nominative_neuter: "~ψ", _, "~πᾰ";
        genitive_masculine: "~πος", "~ποιν", "~πων";
        genitive_feminine: "~πος", "~ποιν", "~πων";
        dative_masculine: "~πῐ", _, "~ψῐ|~ψῐν";
        dative_feminine: "~πῐ", _, "~ψῐ|~ψῐν";
        dative_neuter: _, _, "~ψῐ|~ψῐν";
        accusative_masculine: "~πᾰ", _, "~πᾰς";
        accusative_feminine: "~πᾰ", _, "~πᾰς";
        vocative_masculine: "~ψ", _, _;
        vocative_feminine: "~ψ", _, _;
        adverb: "~πως";
    ),
    // Δους: 5 words (δους, βῐους, δῐδους, γνους, ἀποδεικνυς). Like Πληγεις, except in 15 cells: nominative_masculine sg, nominative_feminine sg, nominative_feminine du, nominative_feminine pl, genitive_feminine sg, genitive_feminine du, ….
    declension!("δους", 5, "~υς", like Πληγεις;
        nominative_masculine: "~υς", _, _;
        nominative_feminine: "~ῡσᾰ", "~υσᾱ", "~ῡσαι";
        genitive_feminine: "~υσης", "~υσαιν", "~υσων";
        dative_masculine: _, _, "~ῡσῐ|~ῡσῐν";
        dative_feminine: "~υσῃ", _, "~υσαις";
        dative_neuter: _, _, "~ῡσῐ|~ῡσῐν";
        accusative_feminine: "~ῡσᾰν", _, "~υσᾱς";
        vocative_masculine: "~υς", _, _;
        vocative_feminine: "~ῡσᾰ", _, _;
    ),
    // Ἁπλοῡς: 5 words (δῐπλοῡς, ἁπλοῡς, χρῡσοῡς, χᾰλκοῡς, πορφῠροῡς). Like Μᾰλᾰκος, except in 16 cells: nominative_masculine sg, nominative_masculine du, nominative_masculine pl, nominative_feminine pl, nominative_neuter sg, nominative_neuter du, ….
    declension!("ἁπλοῡς", 5, "~οῡς", like Μᾰλᾰκος;
        nominative_masculine: "~οῡς", "~ω|~ω", "~οῑ";
        nominative_feminine: _, _, "~αῑ";
        nominative_neuter: "~οῡν", "~ω|~ω", "~ᾱ";
        accusative_masculine: "~οῡν", "~ω|~ω", "~οῡς";
        accusative_neuter: _, "~ω|~ω", _;
        vocative_masculine: "~οῡ", "~ω|~ω", _;
        vocative_neuter: _, "~ω|~ω", _;
        comparative: "~εωτερος";
        superlative: "~εωτᾰτος";
    ),
    // Ἀντιπαις: 5 words (ἀργινοεις, ἀντιπαις, ἀξιωθεις, ἀποτειχισθεις, ἀποκριθεις).
    declension!("ἀντιπαις", 5, "~ι~ις";
        nominative_masculine: "~ῐ~ις", "~ῐ~ντε", "~ῐ~ντες";
        nominative_feminine: "~ῐ~ῑσᾰ", "~ῐ~ισᾱ", "~ῐ~ῑσαι";
        nominative_neuter: "~ῐ~ν", _, "~ῐ~ντᾰ";
        genitive_masculine: "~ῐ~ντος", "~ῐ~ντοιν", "~ῐ~ντων";
        genitive_feminine: "~ῐ~ισης", "~ῐ~ισαιν", "~ῐ~ισων";
        dative_masculine: "~ῐ~ντῐ", _, "~ῐ~ῑσῐ|~ῐ~ῑσῐν";
        dative_feminine: "~ῐ~ισῃ", _, "~ῐ~ισαις";
        dative_neuter: _, _, "~ῐ~ῑσῐ|~ῐ~ῑσῐν";
        accusative_masculine: "~ῐ~ντᾰ", _, "~ῐ~ντᾰς";
        accusative_feminine: "~ῐ~ῑσᾰν", _, "~ῐ~ισᾱς";
        vocative_masculine: "~ῐ~ις", _, _;
        vocative_feminine: "~ῐ~ῑσᾰ", _, _;
        adverb: "~ῐ~ντως";
    ),
    // Ἀντιπαλος: 5 words (ὑψιμελαθρος, ἀντιπαλος, ὑψικαρηνος, ἀμφικεφαλος, ἁλιβαπτος). Like Ἀξιωτατος, except in 13 cells: nominative_feminine sg, nominative_feminine du, nominative_feminine pl, genitive_feminine sg, genitive_feminine du, dative_feminine sg, ….
    declension!("ἀντιπαλος", 5, "~ι~α~ος", like Ἀξιωτατος;
        nominative_feminine: "~ῐ~ᾰ~ος", "~ῐ~ᾰ~ω", "~ῐ~ᾰ~οι";
        genitive_feminine: "~ῐ~ᾰ~ου", "~ῐ~ᾰ~οιν", _;
        dative_feminine: "~ῐ~ᾰ~ῳ", _, "~ῐ~ᾰ~οις";
        accusative_feminine: "~ῐ~ᾰ~ον", _, "~ῐ~ᾰ~ους";
        vocative_feminine: "~ῐ~ᾰ~ε", _, _;
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
        nominative_masculine: "~ᾱς", "~ᾰνε", "~ᾰνες";
        nominative_feminine: "~αινᾰ", "~αινᾱ", "~αιναι";
        nominative_neuter: "~ᾰν", _, "~ᾰνᾰ";
        genitive_masculine: "~ᾰνος", "~ᾰνοιν", "~ᾰνων";
        genitive_feminine: "~αινης", "~αιναιν", "~αινων";
        dative_masculine: "~ᾰνῐ", _, "~ᾰσῐ|~ᾰσῐν";
        dative_feminine: "~αινῃ", _, "~αιναις";
        dative_neuter: _, _, "~ᾰσῐ|~ᾰσῐν";
        accusative_masculine: "~ᾰνᾰ", _, "~ᾰνᾰς";
        accusative_feminine: "~αινᾰν", _, "~αινᾱς";
        vocative_masculine: "~ᾰν", _, _;
        vocative_feminine: "~αινᾰ", _, _;
        comparative: "~ᾰντερος";
        superlative: "~ᾰντᾰτος";
        adverb: "~ᾰνως";
    ),
    // Ἀπους: 4 words (πολυπους, ἀπους, χειμαρρους, δῐπους). Like Τρῐπους, except in 2 cells: accusative_masculine sg, accusative_feminine sg.
    declension!("ἀπους", 4, "~υς", like Τρῐπους;
        accusative_masculine: "~υν", _, _;
        accusative_feminine: "~υν", _, _;
    ),
    // Χειρων: 4 words (μειζων, χειρων, κρειττων, τρῐσκᾰκοδαιμων).
    declension!("χειρων", 4, "~ι~ων";
        nominative_masculine: "~ι~ων", "~ι~ονε", "~ι~ονες|~ι~ους";
        nominative_feminine: "~ι~ων", "~ι~ονε", "~ι~ονες|~ι~ους";
        nominative_neuter: "~ῑ~ον", _, "~ι~ονᾰ|~ι~ω";
        genitive_masculine: "~ι~ονος", "~ι~ονοιν", "~ι~ονων";
        genitive_feminine: "~ι~ονος", "~ι~ονοιν", "~ι~ονων";
        dative_masculine: "~ι~ονῐ", _, "~ι~οσῐ|~ι~οσῐν";
        dative_feminine: "~ι~ονῐ", _, "~ι~οσῐ|~ι~οσῐν";
        dative_neuter: _, _, "~ι~οσῐ|~ι~οσῐν";
        accusative_masculine: "~ι~ονᾰ|~ι~ω", _, "~ι~ονᾰς|~ι~ους";
        accusative_feminine: "~ι~ονᾰ|~ι~ω", _, "~ι~ονᾰς|~ι~ους";
        accusative_neuter: _, _, "~ι~ονᾰ|~ι~ω";
        vocative_masculine: "~ῑ~ον", _, "~ι~ονες|~ι~ους";
        vocative_feminine: "~ῑ~ον", _, "~ι~ονες|~ι~ους";
        vocative_neuter: _, _, "~ι~ονᾰ|~ι~ω";
        adverb: "~ῑ~ον";
    ),
    // Ἀθανατος: 4 words (ἀθανατος, ἀναγκαζομενος, ἀνᾰχασσαμενος, ἱκανωτατος). Like Ἀγραμματος, except in 10 cells: nominative_feminine sg, nominative_feminine du, nominative_feminine pl, genitive_feminine sg, genitive_feminine du, dative_feminine sg, ….
    declension!("ἀθανατος", 4, "~α~α~ος", like Ἀγραμματος;
        nominative_feminine: "~ᾰ~ᾰ~η", "~ᾰ~ᾰ~ᾱ", "~ᾰ~ᾰ~αι";
        genitive_feminine: "~ᾰ~ᾰ~ης", "~ᾰ~ᾰ~αιν", _;
        dative_feminine: "~ᾰ~ᾰ~ῃ", _, "~ᾰ~ᾰ~αις";
        accusative_feminine: "~ᾰ~ᾰ~ην", _, "~ᾰ~ᾰ~ᾱς";
        vocative_feminine: "~ᾰ~ᾰ~η", _, _;
    ),
    // Πεπτεως: 4 words (πεπτεως, πεπτηως, τετληως, κεκοτηως). Like Εἰκως, except in 4 cells: nominative_feminine sg, nominative_feminine pl, accusative_feminine sg, vocative_feminine sg.
    declension!("πεπτεως", 4, "~ως", like Εἰκως;
        nominative_feminine: "~υιᾰ", _, "~υιαι";
        accusative_feminine: "~υιᾰν", _, _;
        vocative_feminine: "~υιᾰ", _, _;
    ),
    // Ἀδαμᾱς: 4 words (ἀδαμᾱς, ὑπαρξᾱς, ἀντῐασᾱς, ἀναπλησᾱς).
    declension!("ἀδαμᾱς", 4, "~α~ᾱς";
        nominative_masculine: "~ᾰ~ᾱς", "~ᾰ~ᾰντε", "~ᾰ~ᾰντες";
        nominative_feminine: "~ᾰ~ᾱσᾰ", "~ᾰ~ᾱσᾱ", "~ᾰ~ᾱσαι";
        nominative_neuter: "~ᾰ~ᾰν", _, "~ᾰ~ᾰντᾰ";
        genitive_masculine: "~ᾰ~ᾰντος", "~ᾰ~ᾰντοιν", "~ᾰ~ᾰντων";
        genitive_feminine: "~ᾰ~ᾱσης", "~ᾰ~ᾱσαιν", "~ᾰ~ᾱσων";
        dative_masculine: "~ᾰ~ᾰντῐ", _, "~ᾰ~ᾱσῐ|~ᾰ~ᾱσῐν";
        dative_feminine: "~ᾰ~ᾱσῃ", _, "~ᾰ~ᾱσαις";
        dative_neuter: _, _, "~ᾰ~ᾱσῐ|~ᾰ~ᾱσῐν";
        accusative_masculine: "~ᾰ~ᾰντᾰ", _, "~ᾰ~ᾰντᾰς";
        accusative_feminine: "~ᾰ~ᾱσᾰν", _, "~ᾰ~ᾱσᾱς";
        vocative_masculine: "~ᾰ~ᾱς", _, _;
        vocative_feminine: "~ᾰ~ᾱσᾰ", _, _;
        adverb: "~ᾰ~ᾰντως";
    ),
    // Ἰχθυηρος: 4 words (νεκυϊκος, ἰχθυηρος, ἰχθυϊκος, ἀποκᾰλυπτῐκος). Like Δρυϊνος, except in 5 cells: genitive_masculine sg, genitive_masculine du, genitive_feminine du, dative_masculine pl, dative_feminine pl.
    declension!("ἰχθυηρος", 4, "~υ~ος", like Δρυϊνος;
        genitive_masculine: "~ῠ~οῡ", "~ῠ~οῑν", _;
        genitive_feminine: _, "~ῠ~αῑν", _;
        dative_masculine: _, _, "~ῠ~οῑς";
        dative_feminine: _, _, "~ῠ~αῑς";
    ),
    // Βαιος: 3 words (πᾰλαιος, βαιος, ἠβαιος). Like Λᾰμπρος, except in 1 cell: adverb.
    declension!("βαιος", 3, "~ος", like Λᾰμπρος;
        adverb: "~ον";
    ),
    // Θηλῠς: 3 words (ἡμῐσῠς, θηλῠς, πρεσβῠς). Like Τρᾱχῠς, except in 11 cells: nominative_masculine du, nominative_masculine pl, nominative_feminine sg, nominative_feminine pl, nominative_neuter pl, dative_masculine sg, ….
    declension!("θηλῠς", 3, "~ῠς", like Τρᾱχῠς;
        nominative_masculine: _, "~ει", "~εις";
        nominative_feminine: "~ειᾰ", _, "~ειαι";
        nominative_neuter: _, _, "~η|~εᾰ";
        dative_masculine: "~ει", _, _;
        accusative_masculine: _, _, "~εις";
        accusative_feminine: "~ειᾰν", _, _;
        accusative_neuter: _, _, "~η|~εᾰ";
        vocative_feminine: "~ειᾰ", _, _;
        vocative_neuter: _, _, "~η|~εᾰ";
    ),
    // Δῐᾱνταῑος: 3 words (χῠδαῑος, ᾰμορβαῑος, δῐᾱνταῑος). Like Οἰκεῑος, except in 10 cells: nominative_feminine sg, nominative_feminine du, nominative_feminine pl, genitive_feminine sg, genitive_feminine du, dative_feminine sg, ….
    declension!("δῐᾱνταῑος", 3, "~ῑος", like Οἰκεῑος;
        nominative_feminine: "~ῑος", "~ιω", "~ῑοι";
        genitive_feminine: "~ιου", "~ιοιν", _;
        dative_feminine: "~ιῳ", _, "~ιοις";
        accusative_feminine: "~ῑον", _, "~ιους";
        vocative_feminine: "~ῑε", _, _;
    ),
    // Ἀτυχης: 3 words (ἰχθυοειδης, ἀτυχης, ἀρυταινοειδης). Like Ἰχθυηρος, except in 24 cells: nominative_masculine sg, nominative_masculine du, nominative_masculine pl, nominative_feminine sg, nominative_feminine du, nominative_feminine pl, ….
    declension!("ἀτυχης", 3, "~υ~ης", like Ἰχθυηρος;
        nominative_masculine: "~ῠ~ης", "~ῠ~εῑ", "~ῠ~εῑς";
        nominative_feminine: "~ῠ~ης", "~ῠ~εῑ", "~ῠ~εῑς";
        nominative_neuter: "~ῠ~ες", _, "~ῠ~η";
        genitive_masculine: "~ῠ~οῡς", _, _;
        genitive_feminine: "~ῠ~οῡς", "~ῠ~οῑν", _;
        dative_masculine: "~ῠ~εῑ", _, "~ῠ~εσῐ|~ῠ~εσῐν";
        dative_feminine: "~ῠ~εῑ", _, "~ῠ~εσῐ|~ῠ~εσῐν";
        dative_neuter: _, _, "~ῠ~εσῐ|~ῠ~εσῐν";
        accusative_masculine: "~ῠ~η", _, "~ῠ~εῑς";
        accusative_feminine: "~ῠ~η", _, "~ῠ~εῑς";
        vocative_masculine: "~ῠ~ες", _, _;
        vocative_feminine: "~ῠ~ες", _, _;
        comparative: "~ῠ~εστερος";
        superlative: "~ῠ~εστᾰτος";
    ),
    // Μωνῠξ: 3 words (μωνῠξ, γᾰμψωνῠξ, κρᾰτερωνῠξ). Like Κᾰλλῐθρῐξ, except in 18 cells: nominative_masculine du, nominative_masculine pl, nominative_feminine du, nominative_feminine pl, nominative_neuter pl, genitive_masculine sg, ….
    declension!("μωνῠξ", 3, "~ξ", like Κᾰλλῐθρῐξ;
        nominative_masculine: _, "~χε", "~χες";
        nominative_feminine: _, "~χε", "~χες";
        nominative_neuter: _, _, "~χᾰ";
        genitive_masculine: "~χος", "~χοιν", "~χων";
        genitive_feminine: "~χος", "~χοιν", "~χων";
        dative_masculine: "~χῐ", _, _;
        dative_feminine: "~χῐ", _, _;
        accusative_masculine: "~χᾰ", _, "~χᾰς";
        accusative_feminine: "~χᾰ", _, "~χᾰς";
        adverb: "~χως";
    ),
    // Ἱκανωτερος: 3 words (ἀναγκαιοτερος, ἱκανωτερος, ἀμαθεστερος). Like Ἰσοτατος, except in 6 cells: nominative_feminine sg, genitive_feminine sg, dative_feminine sg, accusative_feminine sg, vocative_feminine sg, adverb.
    declension!("ἱκανωτερος", 3, "~α~ος", like Ἰσοτατος;
        nominative_feminine: "~ᾰ~ᾱ", _, _;
        genitive_feminine: "~ᾰ~ᾱς", _, _;
        dative_feminine: "~ᾰ~ᾳ", _, _;
        accusative_feminine: "~ᾰ~ᾱν", _, _;
        vocative_feminine: "~ᾰ~ᾱ", _, _;
        adverb: "~ᾰ~ον";
    ),
];
