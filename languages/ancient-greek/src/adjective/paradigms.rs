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
     nominative_masculine: $c0:tt, $c1:tt, $c2:tt;
     nominative_feminine: $c3:tt, $c4:tt, $c5:tt;
     nominative_neuter: $c6:tt, $c7:tt, $c8:tt;
     genitive_masculine: $c9:tt, $c10:tt, $c11:tt;
     genitive_feminine: $c12:tt, $c13:tt, $c14:tt;
     genitive_neuter: $c15:tt, $c16:tt, $c17:tt;
     dative_masculine: $c18:tt, $c19:tt, $c20:tt;
     dative_feminine: $c21:tt, $c22:tt, $c23:tt;
     dative_neuter: $c24:tt, $c25:tt, $c26:tt;
     accusative_masculine: $c27:tt, $c28:tt, $c29:tt;
     accusative_feminine: $c30:tt, $c31:tt, $c32:tt;
     accusative_neuter: $c33:tt, $c34:tt, $c35:tt;
     vocative_masculine: $c36:tt, $c37:tt, $c38:tt;
     vocative_feminine: $c39:tt, $c40:tt, $c41:tt;
     vocative_neuter: $c42:tt, $c43:tt, $c44:tt;
     comparative: $c45:tt;
     superlative: $c46:tt;
     adverb: $c47:tt;
    ) => {
        Paradigm {
            name: ::std::borrow::Cow::Borrowed($name),
            words: $words,
            lemma: ::std::borrow::Cow::Borrowed($lemma),
            parent: like!($($parent)?),
            inherent: ::std::borrow::Cow::Borrowed(&[]),
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
                flexion::cell!($c15), "\t",
                flexion::cell!($c16), "\t",
                flexion::cell!($c17), "\t",
                flexion::cell!($c18), "\t",
                flexion::cell!($c19), "\t",
                flexion::cell!($c20), "\t",
                flexion::cell!($c21), "\t",
                flexion::cell!($c22), "\t",
                flexion::cell!($c23), "\t",
                flexion::cell!($c24), "\t",
                flexion::cell!($c25), "\t",
                flexion::cell!($c26), "\t",
                flexion::cell!($c27), "\t",
                flexion::cell!($c28), "\t",
                flexion::cell!($c29), "\t",
                flexion::cell!($c30), "\t",
                flexion::cell!($c31), "\t",
                flexion::cell!($c32), "\t",
                flexion::cell!($c33), "\t",
                flexion::cell!($c34), "\t",
                flexion::cell!($c35), "\t",
                flexion::cell!($c36), "\t",
                flexion::cell!($c37), "\t",
                flexion::cell!($c38), "\t",
                flexion::cell!($c39), "\t",
                flexion::cell!($c40), "\t",
                flexion::cell!($c41), "\t",
                flexion::cell!($c42), "\t",
                flexion::cell!($c43), "\t",
                flexion::cell!($c44), "\t",
                flexion::cell!($c45), "\t",
                flexion::cell!($c46), "\t",
                flexion::cell!($c47), "\t",
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
    /// Like `ἐσχᾰτος`: 831 words.
    Ἐσχᾰτος,
    /// Like `ἐμπεδος`: 710 words.
    Ἐμπεδος,
    /// Like `μᾰλᾰκος`: 649 words.
    Μᾰλᾰκος,
    /// Like `ἀτελης`: 234 words.
    Ἀτελης,
    /// Like `τελειος`: 221 words.
    Τελειος,
    /// Like `προσηκων`: 195 words.
    Προσηκων,
    /// Like `λᾰμπρος`: 135 words.
    Λᾰμπρος,
    /// Like `προπᾱς`: 130 words.
    Προπᾱς,
    /// Like `πλεῑστος`: 94 words.
    Πλεῑστος,
    /// Like `ἀτακτος`: 89 words.
    Ἀτακτος,
    /// Like `ψῑλος`: 86 words.
    Ψῑλος,
    /// Like `ὑληεις`: 86 words.
    Ὑληεις,
    /// Like `ἡκῐστος`: 84 words.
    Ἡκῐστος,
    /// Like `κῐων`: 68 words.
    Κῐων,
    /// Like `οἰκεῑος`: 63 words.
    Οἰκεῑος,
    /// Like `ἀρκων`: 51 words.
    Ἀρκων,
    /// Like `εἰκως`: 48 words.
    Εἰκως,
    /// Like `ἀσημος`: 44 words.
    Ἀσημος,
    /// Like `μαυρος`: 44 words.
    Μαυρος,
    /// Like `ἐκλῠτηρῐον`: 35 words.
    Ἐκλῠτηρῐον,
    /// Like `ἀξῐοχρεως`: 33 words.
    Ἀξῐοχρεως,
    /// Like `πλειων`: 29 words.
    Πλειων,
    /// Like `ἀκρᾰτης`: 26 words.
    Ἀκρᾰτης,
    /// Like `ἀλλος`: 25 words.
    Ἀλλος,
    /// Like `φωνηεις`: 25 words.
    Φωνηεις,
    /// Like `ἀκρῐτος`: 24 words.
    Ἀκρῐτος,
    /// Like `τρᾱχῠς`: 24 words.
    Τρᾱχῠς,
    /// Like `βαινων`: 21 words.
    Βαινων,
    /// Like `ἀκαρπος`: 21 words.
    Ἀκαρπος,
    /// Like `τρῐπους`: 18 words.
    Τρῐπους,
    /// Like `ἡσσων`: 18 words.
    Ἡσσων,
    /// Like `κᾰκοηθης`: 17 words.
    Κᾰκοηθης,
    /// Like `παυων`: 17 words.
    Παυων,
    /// Like `ἀγων`: 16 words.
    Ἀγων,
    /// Like `δειξᾱς`: 16 words.
    Δειξᾱς,
    /// Like `τοιοσδε`: 16 words.
    Τοιοσδε,
    /// Like `ἀδιψος`: 16 words.
    Ἀδιψος,
    /// Like `ὑπαιθρος`: 13 words.
    Ὑπαιθρος,
    /// Like `ἀσυνδετος`: 13 words.
    Ἀσυνδετος,
    /// Like `ἀγραμματος`: 13 words.
    Ἀγραμματος,
    /// Like `ἀγνως`: 12 words.
    Ἀγνως,
    /// Like `ἀρχικος`: 12 words.
    Ἀρχικος,
    /// Like `ἀκροτερος`: 12 words.
    Ἀκροτερος,
    /// Like `ἀποδεικτῐκος`: 12 words.
    Ἀποδεικτῐκος,
    /// Like `ἀβατος`: 11 words.
    Ἀβατος,
    /// Like `φαῡλος`: 11 words.
    Φαῡλος,
    /// Like `ἐρημος`: 10 words.
    Ἐρημος,
    /// Like `ἀγοραῑος`: 10 words.
    Ἀγοραῑος,
    /// Like `γαῡρος`: 10 words.
    Γαῡρος,
    /// Like `πλωϊμος`: 10 words.
    Πλωϊμος,
    /// Like `ὑποθετικος`: 10 words.
    Ὑποθετικος,
    /// Like `ἀμαθης`: 9 words.
    Ἀμαθης,
    /// Like `ὑψηλος`: 9 words.
    Ὑψηλος,
    /// Like `κᾰλλῐθρῐξ`: 9 words.
    Κᾰλλῐθρῐξ,
    /// Like `ζευξᾱς`: 9 words.
    Ζευξᾱς,
    /// Like `παυσων`: 9 words.
    Παυσων,
    /// Like `ἀμπελοεις`: 9 words.
    Ἀμπελοεις,
    /// Like `ἀριστος`: 9 words.
    Ἀριστος,
    /// Like `ἀντιπαλος`: 9 words.
    Ἀντιπαλος,
    /// Like `ἀκρος`: 9 words.
    Ἀκρος,
    /// Like `δῐκαιος`: 8 words.
    Δῐκαιος,
    /// Like `θειων`: 8 words.
    Θειων,
    /// Like `ἀξιωτερος`: 8 words.
    Ἀξιωτερος,
    /// Like `φᾰνερος`: 7 words.
    Φᾰνερος,
    /// Like `ἀσιτος`: 7 words.
    Ἀσιτος,
    /// Like `ἁπλοος`: 7 words.
    Ἁπλοος,
    /// Like `φῡς`: 7 words.
    Φῡς,
    /// Like `ἀρξαμενος`: 7 words.
    Ἀρξαμενος,
    /// Like `ἰσος`: 7 words.
    Ἰσος,
    /// Like `ἀκμαζων`: 6 words.
    Ἀκμαζων,
    /// Like `ὑπεγγῠος`: 6 words.
    Ὑπεγγῠος,
    /// Like `κολοβος`: 5 words.
    Κολοβος,
    /// Like `αἰθοψ`: 5 words.
    Αἰθοψ,
    /// Like `δους`: 5 words.
    Δους,
    /// Like `ἀντιπαις`: 5 words.
    Ἀντιπαις,
    /// Like `ἀγωγος`: 5 words.
    Ἀγωγος,
    /// Like `ἀνομωτᾰτος`: 5 words.
    Ἀνομωτᾰτος,
    /// Like `ὑπατος`: 5 words.
    Ὑπατος,
    /// Like `ἀφροδῑσῐος`: 4 words.
    Ἀφροδῑσῐος,
    /// Like `μελᾱς`: 4 words.
    Μελᾱς,
    /// Like `ἀγαπητος`: 4 words.
    Ἀγαπητος,
    /// Like `ἀρτιος`: 4 words.
    Ἀρτιος,
    /// Like `ἀμαυρος`: 4 words.
    Ἀμαυρος,
    /// Like `χειρων`: 4 words.
    Χειρων,
    /// Like `ἀγωνιστικος`: 4 words.
    Ἀγωνιστικος,
    /// Like `δῐπλοῡς`: 4 words.
    Δῐπλοῡς,
    /// Like `ἀγνοων`: 4 words.
    Ἀγνοων,
    /// Like `δρυϊνος`: 4 words.
    Δρυϊνος,
    /// Like `πεπτεως`: 4 words.
    Πεπτεως,
    /// Like `ἀργοτατος`: 4 words.
    Ἀργοτατος,
    /// Like `ἀκυρος`: 4 words.
    Ἀκυρος,
    /// Like `ἰταμος`: 4 words.
    Ἰταμος,
    /// Like `πρωτος`: 3 words.
    Πρωτος,
    /// Like `βαιος`: 3 words.
    Βαιος,
    /// Like `θηλῠς`: 3 words.
    Θηλῠς,
    /// Like `δῐᾱνταῑος`: 3 words.
    Δῐᾱνταῑος,
    /// Like `ἰσοδυναμος`: 3 words.
    Ἰσοδυναμος,
    /// Like `ὑστερος`: 3 words.
    Ὑστερος,
    /// Like `ἀκηριος`: 3 words.
    Ἀκηριος,
    /// Like `ἀποκᾰταστᾰτῐκος`: 3 words.
    Ἀποκᾰταστᾰτῐκος,
    /// Like `μωνῠξ`: 3 words.
    Μωνῠξ,
    /// Like `ἀργος`: 2 words.
    Ἀργος,
    /// Like `ἀσυμπτωτος`: 1 words.
    Ἀσυμπτωτος,
    /// Like `ἀοριστος`: 1 words.
    Ἀοριστος,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 104] = [
        Declension::Ἐσχᾰτος,
        Declension::Ἐμπεδος,
        Declension::Μᾰλᾰκος,
        Declension::Ἀτελης,
        Declension::Τελειος,
        Declension::Προσηκων,
        Declension::Λᾰμπρος,
        Declension::Προπᾱς,
        Declension::Πλεῑστος,
        Declension::Ἀτακτος,
        Declension::Ψῑλος,
        Declension::Ὑληεις,
        Declension::Ἡκῐστος,
        Declension::Κῐων,
        Declension::Οἰκεῑος,
        Declension::Ἀρκων,
        Declension::Εἰκως,
        Declension::Ἀσημος,
        Declension::Μαυρος,
        Declension::Ἐκλῠτηρῐον,
        Declension::Ἀξῐοχρεως,
        Declension::Πλειων,
        Declension::Ἀκρᾰτης,
        Declension::Ἀλλος,
        Declension::Φωνηεις,
        Declension::Ἀκρῐτος,
        Declension::Τρᾱχῠς,
        Declension::Βαινων,
        Declension::Ἀκαρπος,
        Declension::Τρῐπους,
        Declension::Ἡσσων,
        Declension::Κᾰκοηθης,
        Declension::Παυων,
        Declension::Ἀγων,
        Declension::Δειξᾱς,
        Declension::Τοιοσδε,
        Declension::Ἀδιψος,
        Declension::Ὑπαιθρος,
        Declension::Ἀσυνδετος,
        Declension::Ἀγραμματος,
        Declension::Ἀγνως,
        Declension::Ἀρχικος,
        Declension::Ἀκροτερος,
        Declension::Ἀποδεικτῐκος,
        Declension::Ἀβατος,
        Declension::Φαῡλος,
        Declension::Ἐρημος,
        Declension::Ἀγοραῑος,
        Declension::Γαῡρος,
        Declension::Πλωϊμος,
        Declension::Ὑποθετικος,
        Declension::Ἀμαθης,
        Declension::Ὑψηλος,
        Declension::Κᾰλλῐθρῐξ,
        Declension::Ζευξᾱς,
        Declension::Παυσων,
        Declension::Ἀμπελοεις,
        Declension::Ἀριστος,
        Declension::Ἀντιπαλος,
        Declension::Ἀκρος,
        Declension::Δῐκαιος,
        Declension::Θειων,
        Declension::Ἀξιωτερος,
        Declension::Φᾰνερος,
        Declension::Ἀσιτος,
        Declension::Ἁπλοος,
        Declension::Φῡς,
        Declension::Ἀρξαμενος,
        Declension::Ἰσος,
        Declension::Ἀκμαζων,
        Declension::Ὑπεγγῠος,
        Declension::Κολοβος,
        Declension::Αἰθοψ,
        Declension::Δους,
        Declension::Ἀντιπαις,
        Declension::Ἀγωγος,
        Declension::Ἀνομωτᾰτος,
        Declension::Ὑπατος,
        Declension::Ἀφροδῑσῐος,
        Declension::Μελᾱς,
        Declension::Ἀγαπητος,
        Declension::Ἀρτιος,
        Declension::Ἀμαυρος,
        Declension::Χειρων,
        Declension::Ἀγωνιστικος,
        Declension::Δῐπλοῡς,
        Declension::Ἀγνοων,
        Declension::Δρυϊνος,
        Declension::Πεπτεως,
        Declension::Ἀργοτατος,
        Declension::Ἀκυρος,
        Declension::Ἰταμος,
        Declension::Πρωτος,
        Declension::Βαιος,
        Declension::Θηλῠς,
        Declension::Δῐᾱνταῑος,
        Declension::Ἰσοδυναμος,
        Declension::Ὑστερος,
        Declension::Ἀκηριος,
        Declension::Ἀποκᾰταστᾰτῐκος,
        Declension::Μωνῠξ,
        Declension::Ἀργος,
        Declension::Ἀσυμπτωτος,
        Declension::Ἀοριστος,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 104] = [
    // Ἐσχᾰτος: 831 words (ἐρωμενος, ἁγιος, ὀλῐγος, ὁλος, ἐσχᾰτος, ξενος).
    declension!("ἐσχᾰτος", 831, "~ος";
        nominative_masculine: "~ος", "~ω", "~οι";
        nominative_feminine: "~η", "~ᾱ", "~αι";
        nominative_neuter: "~ον", "~ω", "~ᾰ";
        genitive_masculine: "~ου", "~οιν", "~ων";
        genitive_feminine: "~ης", "~αιν", "~ων";
        genitive_neuter: "~ου", "~οιν", "~ων";
        dative_masculine: "~ῳ", "~οιν", "~οις";
        dative_feminine: "~ῃ", "~αιν", "~αις";
        dative_neuter: "~ῳ", "~οιν", "~οις";
        accusative_masculine: "~ον", "~ω", "~ους";
        accusative_feminine: "~ην", "~ᾱ", "~ᾱς";
        accusative_neuter: "~ον", "~ω", "~ᾰ";
        vocative_masculine: "~ε", "~ω", "~οι";
        vocative_feminine: "~η", "~ᾱ", "~αι";
        vocative_neuter: "~ον", "~ω", "~ᾰ";
        comparative: "~ωτερος";
        superlative: "~ωτᾰτος";
        adverb: "~ως";
    ),
    // Ἐμπεδος: 710 words (δῐᾰβολος, βᾰρβᾰρος, πρωτοτοκος, μᾰγος, θεομᾰχος, εὐχᾰρῐστος). Like Ἐσχᾰτος, except in 14 cells: nominative_feminine sg, nominative_feminine du, nominative_feminine pl, genitive_feminine sg, genitive_feminine du, dative_feminine sg, ….
    declension!("ἐμπεδος", 710, "~ος", like Ἐσχᾰτος;
        nominative_masculine: _, _, _;
        nominative_feminine: "~ος", "~ω", "~οι";
        nominative_neuter: _, _, _;
        genitive_masculine: _, _, _;
        genitive_feminine: "~ου", "~οιν", _;
        genitive_neuter: _, _, _;
        dative_masculine: _, _, _;
        dative_feminine: "~ῳ", "~οιν", "~οις";
        dative_neuter: _, _, _;
        accusative_masculine: _, _, _;
        accusative_feminine: "~ον", "~ω", "~ους";
        accusative_neuter: _, _, _;
        vocative_masculine: _, _, _;
        vocative_feminine: "~ε", "~ω", "~οι";
        vocative_neuter: _, _, _;
        comparative: _;
        superlative: _;
        adverb: _;
    ),
    // Μᾰλᾰκος: 649 words (παιδῐκος, γᾰλᾰκτῐκος, λευκος, πνευματικος, κρῠπτος, κᾰκος). Like Ἐσχᾰτος, except in 11 cells: genitive_masculine sg, genitive_masculine du, genitive_feminine du, genitive_neuter sg, genitive_neuter du, dative_masculine du, ….
    declension!("μᾰλᾰκος", 649, "~ος", like Ἐσχᾰτος;
        nominative_masculine: _, _, _;
        nominative_feminine: _, _, _;
        nominative_neuter: _, _, _;
        genitive_masculine: "~οῡ", "~οῑν", _;
        genitive_feminine: _, "~αῑν", _;
        genitive_neuter: "~οῡ", "~οῑν", _;
        dative_masculine: _, "~οῑν", "~οῑς";
        dative_feminine: _, "~αῑν", "~αῑς";
        dative_neuter: _, "~οῑν", "~οῑς";
        accusative_masculine: _, _, _;
        accusative_feminine: _, _, _;
        accusative_neuter: _, _, _;
        vocative_masculine: _, _, _;
        vocative_feminine: _, _, _;
        vocative_neuter: _, _, _;
        comparative: _;
        superlative: _;
        adverb: _;
    ),
    // Ἀτελης: 234 words (ὑγιης, ἀβαρης, ψευδης, ἀτελης, δηλητηρῐωδης, σῑγμοειδης).
    declension!("ἀτελης", 234, "~ης";
        nominative_masculine: "~ης", "~εῑ", "~εῑς";
        nominative_feminine: "~ης", "~εῑ", "~εῑς";
        nominative_neuter: "~ες", "~εῑ", "~η";
        genitive_masculine: "~οῡς", "~οῑν", "~ων";
        genitive_feminine: "~οῡς", "~οῑν", "~ων";
        genitive_neuter: "~οῡς", "~οῑν", "~ων";
        dative_masculine: "~εῑ", "~οῑν", "~εσῐ|~εσῐν";
        dative_feminine: "~εῑ", "~οῑν", "~εσῐ|~εσῐν";
        dative_neuter: "~εῑ", "~οῑν", "~εσῐ|~εσῐν";
        accusative_masculine: "~η", "~εῑ", "~εῑς";
        accusative_feminine: "~η", "~εῑ", "~εῑς";
        accusative_neuter: "~ες", "~εῑ", "~η";
        vocative_masculine: "~ες", "~εῑ", "~εῑς";
        vocative_feminine: "~ες", "~εῑ", "~εῑς";
        vocative_neuter: "~ες", "~εῑ", "~η";
        comparative: "~εστερος";
        superlative: "~εστᾰτος";
        adverb: "~ως";
    ),
    // Τελειος: 221 words (κῡρῐος, νεος, ἑτερος, ὁμοιος, πλουσῐος, βῐαιος). Like Ἐσχᾰτος, except in 5 cells: nominative_feminine sg, genitive_feminine sg, dative_feminine sg, accusative_feminine sg, vocative_feminine sg.
    declension!("τελειος", 221, "~ος", like Ἐσχᾰτος;
        nominative_masculine: _, _, _;
        nominative_feminine: "~ᾱ", _, _;
        nominative_neuter: _, _, _;
        genitive_masculine: _, _, _;
        genitive_feminine: "~ᾱς", _, _;
        genitive_neuter: _, _, _;
        dative_masculine: _, _, _;
        dative_feminine: "~ᾳ", _, _;
        dative_neuter: _, _, _;
        accusative_masculine: _, _, _;
        accusative_feminine: "~ᾱν", _, _;
        accusative_neuter: _, _, _;
        vocative_masculine: _, _, _;
        vocative_feminine: "~ᾱ", _, _;
        vocative_neuter: _, _, _;
        comparative: _;
        superlative: _;
        adverb: _;
    ),
    // Προσηκων: 195 words (κῠων, μελλων, ὀζων, νεων, γερων, ὑπερφρων).
    declension!("προσηκων", 195, "~ων";
        nominative_masculine: "~ων", "~οντε", "~οντες";
        nominative_feminine: "~ουσᾰ", "~ουσᾱ", "~ουσαι";
        nominative_neuter: "~ον", "~οντε", "~οντᾰ";
        genitive_masculine: "~οντος", "~οντοιν", "~οντων";
        genitive_feminine: "~ουσης", "~ουσαιν", "~ουσων";
        genitive_neuter: "~οντος", "~οντοιν", "~οντων";
        dative_masculine: "~οντῐ", "~οντοιν", "~ουσῐ|~ουσῐν";
        dative_feminine: "~ουσῃ", "~ουσαιν", "~ουσαις";
        dative_neuter: "~οντῐ", "~οντοιν", "~ουσῐ|~ουσῐν";
        accusative_masculine: "~οντᾰ", "~οντε", "~οντᾰς";
        accusative_feminine: "~ουσᾰν", "~ουσᾱ", "~ουσᾱς";
        accusative_neuter: "~ον", "~οντε", "~οντᾰ";
        vocative_masculine: "~ων", "~οντε", "~οντες";
        vocative_feminine: "~ουσᾰ", "~ουσᾱ", "~ουσαι";
        vocative_neuter: "~ον", "~οντε", "~οντᾰ";
        comparative: derived;
        superlative: derived;
        adverb: "~οντως";
    ),
    // Λᾰμπρος: 135 words (μῑκρος, ἐρῠθρος, χλωρος, νεκρος, κᾰθᾰρος, δεξῐος). Like Μᾰλᾰκος, except in 7 cells: nominative_feminine sg, genitive_feminine sg, dative_feminine sg, accusative_feminine sg, vocative_feminine sg, comparative, ….
    declension!("λᾰμπρος", 135, "~ος", like Μᾰλᾰκος;
        nominative_masculine: _, _, _;
        nominative_feminine: "~ᾱ", _, _;
        nominative_neuter: _, _, _;
        genitive_masculine: _, _, _;
        genitive_feminine: "~ᾱς", _, _;
        genitive_neuter: _, _, _;
        dative_masculine: _, _, _;
        dative_feminine: "~ᾳ", _, _;
        dative_neuter: _, _, _;
        accusative_masculine: _, _, _;
        accusative_feminine: "~ᾱν", _, _;
        accusative_neuter: _, _, _;
        vocative_masculine: _, _, _;
        vocative_feminine: "~ᾱ", _, _;
        vocative_neuter: _, _, _;
        comparative: "~οτερος";
        superlative: "~οτᾰτος";
        adverb: _;
    ),
    // Προπᾱς: 130 words (ἀδαμᾱς, λῡσᾱς, γῐγᾱς, βῐβᾱς, φωνησᾱς, νῠξᾱς).
    declension!("προπᾱς", 130, "~ᾱς";
        nominative_masculine: "~ᾱς", "~ᾰντε", "~ᾰντες";
        nominative_feminine: "~ᾱσᾰ", "~ᾱσᾱ", "~ᾱσαι";
        nominative_neuter: "~ᾰν", "~ᾰντε", "~ᾰντᾰ";
        genitive_masculine: "~ᾰντος", "~ᾰντοιν", "~ᾰντων";
        genitive_feminine: "~ᾱσης", "~ᾱσαιν", "~ᾱσων";
        genitive_neuter: "~ᾰντος", "~ᾰντοιν", "~ᾰντων";
        dative_masculine: "~ᾰντῐ", "~ᾰντοιν", "~ᾱσῐ|~ᾱσῐν";
        dative_feminine: "~ᾱσῃ", "~ᾱσαιν", "~ᾱσαις";
        dative_neuter: "~ᾰντῐ", "~ᾰντοιν", "~ᾱσῐ|~ᾱσῐν";
        accusative_masculine: "~ᾰντᾰ", "~ᾰντε", "~ᾰντᾰς";
        accusative_feminine: "~ᾱσᾰν", "~ᾱσᾱ", "~ᾱσᾱς";
        accusative_neuter: "~ᾰν", "~ᾰντε", "~ᾰντᾰ";
        vocative_masculine: "~ᾱς", "~ᾰντε", "~ᾰντες";
        vocative_feminine: "~ᾱσᾰ", "~ᾱσᾱ", "~ᾱσαι";
        vocative_neuter: "~ᾰν", "~ᾰντε", "~ᾰντᾰ";
        comparative: derived;
        superlative: derived;
        adverb: "~ᾰντως";
    ),
    // Πλεῑστος: 94 words (προτερος, βᾰθῠτερος, πρεσβῠτερος, βεβαιοτερος, πλεῑστος, στοιχειωδεστερος). Like Τελειος, except in 3 cells: comparative, superlative, adverb.
    declension!("πλεῑστος", 94, "~ος", like Τελειος;
        nominative_masculine: _, _, _;
        nominative_feminine: _, _, _;
        nominative_neuter: _, _, _;
        genitive_masculine: _, _, _;
        genitive_feminine: _, _, _;
        genitive_neuter: _, _, _;
        dative_masculine: _, _, _;
        dative_feminine: _, _, _;
        dative_neuter: _, _, _;
        accusative_masculine: _, _, _;
        accusative_feminine: _, _, _;
        accusative_neuter: _, _, _;
        vocative_masculine: _, _, _;
        vocative_feminine: _, _, _;
        vocative_neuter: _, _, _;
        comparative: derived;
        superlative: derived;
        adverb: "~ον";
    ),
    // Ἀτακτος: 89 words (πᾰρᾰκλητος, πρᾱος, ἐπικουρος, χερσος, τῠρᾰννος, κᾰλλῐπῡγος). Like Ἐμπεδος, except in 2 cells: comparative, superlative.
    declension!("ἀτακτος", 89, "~ος", like Ἐμπεδος;
        nominative_masculine: _, _, _;
        nominative_feminine: _, _, _;
        nominative_neuter: _, _, _;
        genitive_masculine: _, _, _;
        genitive_feminine: _, _, _;
        genitive_neuter: _, _, _;
        dative_masculine: _, _, _;
        dative_feminine: _, _, _;
        dative_neuter: _, _, _;
        accusative_masculine: _, _, _;
        accusative_feminine: _, _, _;
        accusative_neuter: _, _, _;
        vocative_masculine: _, _, _;
        vocative_feminine: _, _, _;
        vocative_neuter: _, _, _;
        comparative: "~οτερος";
        superlative: "~οτᾰτος";
        adverb: _;
    ),
    // Ψῑλος: 86 words (ξᾰνθος, βᾰκτρῐᾱνος, ὀρθος, στεγνος, ψῑλος, φωτεινος). Like Μᾰλᾰκος, except in 2 cells: comparative, superlative.
    declension!("ψῑλος", 86, "~ος", like Μᾰλᾰκος;
        nominative_masculine: _, _, _;
        nominative_feminine: _, _, _;
        nominative_neuter: _, _, _;
        genitive_masculine: _, _, _;
        genitive_feminine: _, _, _;
        genitive_neuter: _, _, _;
        dative_masculine: _, _, _;
        dative_feminine: _, _, _;
        dative_neuter: _, _, _;
        accusative_masculine: _, _, _;
        accusative_feminine: _, _, _;
        accusative_neuter: _, _, _;
        vocative_masculine: _, _, _;
        vocative_feminine: _, _, _;
        vocative_neuter: _, _, _;
        comparative: "~οτερος";
        superlative: "~οτᾰτος";
        adverb: _;
    ),
    // Ὑληεις: 86 words (σῠγγρᾰφεις, θεις, κοιμηθεις, νῠγεις, αὐδηθεις, κᾰλῠφεις).
    declension!("ὑληεις", 86, "~ις";
        nominative_masculine: "~ις", "~ντε", "~ντες";
        nominative_feminine: "~ῑσᾰ", "~ισᾱ", "~ῑσαι";
        nominative_neuter: "~ν", "~ντε", "~ντᾰ";
        genitive_masculine: "~ντος", "~ντοιν", "~ντων";
        genitive_feminine: "~ισης", "~ισαιν", "~ισων";
        genitive_neuter: "~ντος", "~ντοιν", "~ντων";
        dative_masculine: "~ντῐ", "~ντοιν", "~ῑσῐ|~ῑσῐν";
        dative_feminine: "~ισῃ", "~ισαιν", "~ισαις";
        dative_neuter: "~ντῐ", "~ντοιν", "~ῑσῐ|~ῑσῐν";
        accusative_masculine: "~ντᾰ", "~ντε", "~ντᾰς";
        accusative_feminine: "~ῑσᾰν", "~ισᾱ", "~ισᾱς";
        accusative_neuter: "~ν", "~ντε", "~ντᾰ";
        vocative_masculine: "~ις", "~ντε", "~ντες";
        vocative_feminine: "~ῑσᾰ", "~ισᾱ", "~ῑσαι";
        vocative_neuter: "~ν", "~ντε", "~ντᾰ";
        comparative: derived;
        superlative: derived;
        adverb: "~ντως";
    ),
    // Ἡκῐστος: 84 words (μεγῐστος, ἡκῐστος, στοιχειωδεστᾰτος, κᾰκῐστος, τᾰχῐστος, βελτῐστος). Like Ἐσχᾰτος, except in 3 cells: comparative, superlative, adverb.
    declension!("ἡκῐστος", 84, "~ος", like Ἐσχᾰτος;
        nominative_masculine: _, _, _;
        nominative_feminine: _, _, _;
        nominative_neuter: _, _, _;
        genitive_masculine: _, _, _;
        genitive_feminine: _, _, _;
        genitive_neuter: _, _, _;
        dative_masculine: _, _, _;
        dative_feminine: _, _, _;
        dative_neuter: _, _, _;
        accusative_masculine: _, _, _;
        accusative_feminine: _, _, _;
        accusative_neuter: _, _, _;
        vocative_masculine: _, _, _;
        vocative_feminine: _, _, _;
        vocative_neuter: _, _, _;
        comparative: derived;
        superlative: derived;
        adverb: "~ᾰ";
    ),
    // Κῐων: 68 words (πᾰρων, ἀπων, λᾰβων, πᾰρελθων, ἑκων, εἰπων). Like Προσηκων, except in 7 cells: nominative_feminine sg, nominative_feminine pl, dative_masculine pl, dative_neuter pl, accusative_feminine sg, vocative_feminine sg, ….
    declension!("κῐων", 68, "~ων", like Προσηκων;
        nominative_masculine: _, _, _;
        nominative_feminine: "~οῡσᾰ", _, "~οῡσαι";
        nominative_neuter: _, _, _;
        genitive_masculine: _, _, _;
        genitive_feminine: _, _, _;
        genitive_neuter: _, _, _;
        dative_masculine: _, _, "~οῡσῐ|~οῡσῐν";
        dative_feminine: _, _, _;
        dative_neuter: _, _, "~οῡσῐ|~οῡσῐν";
        accusative_masculine: _, _, _;
        accusative_feminine: "~οῡσᾰν", _, _;
        accusative_neuter: _, _, _;
        vocative_masculine: _, _, _;
        vocative_feminine: "~οῡσᾰ", _, "~οῡσαι";
        vocative_neuter: _, _, _;
        comparative: _;
        superlative: _;
        adverb: _;
    ),
    // Οἰκεῑος: 63 words (ῥωμαῑος, ἑβραῑος, θηβαῑος, ἑκᾰταῑος, θεῑος, δρομαῑος).
    declension!("οἰκεῑος", 63, "~ῑος";
        nominative_masculine: "~ῑος", "~ιω", "~ῑοι";
        nominative_feminine: "~ιᾱ", "~ιᾱ", "~ῑαι";
        nominative_neuter: "~ῑον", "~ιω", "~ῑᾰ";
        genitive_masculine: "~ιου", "~ιοιν", "~ιων";
        genitive_feminine: "~ιᾱς", "~ιαιν", "~ιων";
        genitive_neuter: "~ιου", "~ιοιν", "~ιων";
        dative_masculine: "~ιῳ", "~ιοιν", "~ιοις";
        dative_feminine: "~ιᾳ", "~ιαιν", "~ιαις";
        dative_neuter: "~ιῳ", "~ιοιν", "~ιοις";
        accusative_masculine: "~ῑον", "~ιω", "~ιους";
        accusative_feminine: "~ιᾱν", "~ιᾱ", "~ιᾱς";
        accusative_neuter: "~ῑον", "~ιω", "~ῑᾰ";
        vocative_masculine: "~ῑε", "~ιω", "~ῑοι";
        vocative_feminine: "~ιᾱ", "~ιᾱ", "~ῑαι";
        vocative_neuter: "~ῑον", "~ιω", "~ῑᾰ";
        comparative: "~ιοτερος";
        superlative: "~ιοτᾰτος";
        adverb: "~ιως";
    ),
    // Ἀρκων: 51 words (ποιων, κυκλων, σημειων, βῐων, δηλων, γονῠπετων).
    declension!("ἀρκων", 51, "~ων";
        nominative_masculine: "~ων", "~οῡντε", "~οῡντες";
        nominative_feminine: "~οῡσᾰ", "~ουσᾱ", "~οῡσαι";
        nominative_neuter: "~οῡν", "~οῡντε", "~οῡντᾰ";
        genitive_masculine: "~οῡντος", "~ουντοιν", "~ουντων";
        genitive_feminine: "~ουσης", "~ουσαιν", "~ουσων";
        genitive_neuter: "~οῡντος", "~ουντοιν", "~ουντων";
        dative_masculine: "~οῡντῐ", "~ουντοιν", "~οῡσῐ|~οῡσῐν";
        dative_feminine: "~ουσῃ", "~ουσαιν", "~ουσαις";
        dative_neuter: "~οῡντῐ", "~ουντοιν", "~οῡσῐ|~οῡσῐν";
        accusative_masculine: "~οῡντᾰ", "~οῡντε", "~οῡντᾰς";
        accusative_feminine: "~οῡσᾰν", "~ουσᾱ", "~ουσᾱς";
        accusative_neuter: "~οῡν", "~οῡντε", "~οῡντᾰ";
        vocative_masculine: "~ων", "~οῡντε", "~οῡντες";
        vocative_feminine: "~οῡσᾰ", "~ουσᾱ", "~οῡσαι";
        vocative_neuter: "~οῡν", "~οῡντε", "~οῡντᾰ";
        comparative: derived;
        superlative: derived;
        adverb: "~ουντως";
    ),
    // Εἰκως: 48 words (λελῠκως, κεχηνως, βεβηκως, εἰδως, ἱλεως, πεπαιδευκως).
    declension!("εἰκως", 48, "~ως";
        nominative_masculine: "~ως", "~οτε", "~οτες";
        nominative_feminine: "~υῑᾰ", "~υιᾱ", "~υῑαι";
        nominative_neuter: "~ος", "~οτε", "~οτᾰ";
        genitive_masculine: "~οτος", "~οτοιν", "~οτων";
        genitive_feminine: "~υιᾱς", "~υιαιν", "~υιων";
        genitive_neuter: "~οτος", "~οτοιν", "~οτων";
        dative_masculine: "~οτῐ", "~οτοιν", "~οσῐ|~οσῐν";
        dative_feminine: "~υιᾳ", "~υιαιν", "~υιαις";
        dative_neuter: "~οτῐ", "~οτοιν", "~οσῐ|~οσῐν";
        accusative_masculine: "~οτᾰ", "~οτε", "~οτᾰς";
        accusative_feminine: "~υῑᾰν", "~υιᾱ", "~υιᾱς";
        accusative_neuter: "~ος", "~οτε", "~οτᾰ";
        vocative_masculine: "~ως", "~οτε", "~οτες";
        vocative_feminine: "~υῑᾰ", "~υιᾱ", "~υῑαι";
        vocative_neuter: "~ος", "~οτε", "~οτᾰ";
        comparative: derived;
        superlative: derived;
        adverb: "~οτως";
    ),
    // Ἀσημος: 44 words (ἀποπληκτος, ἀγνωστος, ἀπορος, ἀγνωτος, ἀντοικος, ἀπειρος).
    declension!("ἀσημος", 44, "ἀ~ος";
        nominative_masculine: "ᾰ̓~ος", "ᾰ̓~ω", "ᾰ̓~οι";
        nominative_feminine: "ᾰ̓~ος", "ᾰ̓~ω", "ᾰ̓~οι";
        nominative_neuter: "ᾰ̓~ον", "ᾰ̓~ω", "ᾰ̓~ᾰ";
        genitive_masculine: "ᾰ̓~ου", "ᾰ̓~οιν", "ᾰ̓~ων";
        genitive_feminine: "ᾰ̓~ου", "ᾰ̓~οιν", "ᾰ̓~ων";
        genitive_neuter: "ᾰ̓~ου", "ᾰ̓~οιν", "ᾰ̓~ων";
        dative_masculine: "ᾰ̓~ῳ", "ᾰ̓~οιν", "ᾰ̓~οις";
        dative_feminine: "ᾰ̓~ῳ", "ᾰ̓~οιν", "ᾰ̓~οις";
        dative_neuter: "ᾰ̓~ῳ", "ᾰ̓~οιν", "ᾰ̓~οις";
        accusative_masculine: "ᾰ̓~ον", "ᾰ̓~ω", "ᾰ̓~ους";
        accusative_feminine: "ᾰ̓~ον", "ᾰ̓~ω", "ᾰ̓~ους";
        accusative_neuter: "ᾰ̓~ον", "ᾰ̓~ω", "ᾰ̓~ᾰ";
        vocative_masculine: "ᾰ̓~ε", "ᾰ̓~ω", "ᾰ̓~οι";
        vocative_feminine: "ᾰ̓~ε", "ᾰ̓~ω", "ᾰ̓~οι";
        vocative_neuter: "ᾰ̓~ον", "ᾰ̓~ω", "ᾰ̓~ᾰ";
        comparative: "ᾰ̓~οτερος";
        superlative: "ᾰ̓~οτᾰτος";
        adverb: "ᾰ̓~ως";
    ),
    // Μαυρος: 44 words (μαυρος, πᾰθῐκος, σῠνεργος, γεωργος, σῠνᾰπτος, ἐνεργος). Like Ἀτακτος, except in 12 cells: genitive_masculine sg, genitive_masculine du, genitive_feminine sg, genitive_feminine du, genitive_neuter sg, genitive_neuter du, ….
    declension!("μαυρος", 44, "~ος", like Ἀτακτος;
        nominative_masculine: _, _, _;
        nominative_feminine: _, _, _;
        nominative_neuter: _, _, _;
        genitive_masculine: "~οῡ", "~οῑν", _;
        genitive_feminine: "~οῡ", "~οῑν", _;
        genitive_neuter: "~οῡ", "~οῑν", _;
        dative_masculine: _, "~οῑν", "~οῑς";
        dative_feminine: _, "~οῑν", "~οῑς";
        dative_neuter: _, "~οῑν", "~οῑς";
        accusative_masculine: _, _, _;
        accusative_feminine: _, _, _;
        accusative_neuter: _, _, _;
        vocative_masculine: _, _, _;
        vocative_feminine: _, _, _;
        vocative_neuter: _, _, _;
        comparative: _;
        superlative: _;
        adverb: _;
    ),
    // Ἐκλῠτηρῐον: 35 words (ὠν, νῑκων, τῑμων, ἀρρην, δῐαιτων, θεων). Like Ὑληεις, except in 19 cells: nominative_masculine sg, nominative_feminine sg, nominative_feminine du, nominative_feminine pl, genitive_feminine sg, genitive_feminine du, ….
    declension!("ἐκλῠτηρῐον", 35, "~ν", like Ὑληεις;
        nominative_masculine: "~ν", _, _;
        nominative_feminine: "~σᾰ", "~σᾱ", "~σαι";
        nominative_neuter: _, _, _;
        genitive_masculine: _, _, _;
        genitive_feminine: "~σης", "~σαιν", "~σων";
        genitive_neuter: _, _, _;
        dative_masculine: _, _, "~σῐ|~σῐν";
        dative_feminine: "~σῃ", "~σαιν", "~σαις";
        dative_neuter: _, _, "~σῐ|~σῐν";
        accusative_masculine: _, _, _;
        accusative_feminine: "~σᾰν", "~σᾱ", "~σᾱς";
        accusative_neuter: _, _, _;
        vocative_masculine: "~ν", _, _;
        vocative_feminine: "~σᾰ", "~σᾱ", "~σαι";
        vocative_neuter: _, _, _;
        comparative: _;
        superlative: _;
        adverb: _;
    ),
    // Ἀξῐοχρεως: 33 words (μεγᾰς, δρομᾰς, ἐϋς, ἱστας, ὀκρῐς, νομᾰς).
    declension!("ἀξῐοχρεως", 33, "~ς";
        nominative_masculine: "~ς", "~δε", "~δες";
        nominative_feminine: "~ς", "~δε", "~δες";
        nominative_neuter: "~", "~δε", "~δᾰ";
        genitive_masculine: "~δος", "~δοιν", "~δων";
        genitive_feminine: "~δος", "~δοιν", "~δων";
        genitive_neuter: "~δος", "~δοιν", "~δων";
        dative_masculine: "~δῐ", "~δοιν", "~σῐ";
        dative_feminine: "~δῐ", "~δοιν", "~σῐ";
        dative_neuter: "~δῐ", "~δοιν", "~σῐ";
        accusative_masculine: "~δᾰ", "~δε", "~δᾰς";
        accusative_feminine: "~δᾰ", "~δε", "~δᾰς";
        accusative_neuter: "~", "~δε", "~δᾰ";
        vocative_masculine: "~ς", "~δε", "~δες";
        vocative_feminine: "~ς", "~δε", "~δες";
        vocative_neuter: "~", "~δε", "~δᾰ";
        comparative: derived;
        superlative: derived;
        adverb: "~δως";
    ),
    // Πλειων: 29 words (σωφρων, βελτῑων, κᾰκοδαιμων, μειων, εὐφρων, πλειων).
    declension!("πλειων", 29, "~ων";
        nominative_masculine: "~ων", "~ονε", "~ονες";
        nominative_feminine: "~ων", "~ονε", "~ονες";
        nominative_neuter: "~ον", "~ονε", "~ονᾰ";
        genitive_masculine: "~ονος", "~ονοιν", "~ονων";
        genitive_feminine: "~ονος", "~ονοιν", "~ονων";
        genitive_neuter: "~ονος", "~ονοιν", "~ονων";
        dative_masculine: "~ονῐ", "~ονοιν", "~οσῐ|~οσῐν";
        dative_feminine: "~ονῐ", "~ονοιν", "~οσῐ|~οσῐν";
        dative_neuter: "~ονῐ", "~ονοιν", "~οσῐ|~οσῐν";
        accusative_masculine: "~ονᾰ", "~ονε", "~ονᾰς";
        accusative_feminine: "~ονᾰ", "~ονε", "~ονᾰς";
        accusative_neuter: "~ον", "~ονε", "~ονᾰ";
        vocative_masculine: "~ον", "~ονε", "~ονες";
        vocative_feminine: "~ον", "~ονε", "~ονες";
        vocative_neuter: "~ον", "~ονε", "~ονᾰ";
        comparative: "~ονεστερος";
        superlative: "~ονεστᾰτος";
        adverb: "~ονως";
    ),
    // Ἀκρᾰτης: 26 words (ἀσθενης, ἀληθης, ἀηθης, ἀπευθης, ἀμαυρης, ἀεικης).
    declension!("ἀκρᾰτης", 26, "ἀ~ης";
        nominative_masculine: "ᾰ̓~ης", "ᾰ̓~εῑ", "ᾰ̓~εῑς";
        nominative_feminine: "ᾰ̓~ης", "ᾰ̓~εῑ", "ᾰ̓~εῑς";
        nominative_neuter: "ᾰ̓~ες", "ᾰ̓~εῑ", "ᾰ̓~η";
        genitive_masculine: "ᾰ̓~οῡς", "ᾰ̓~οῑν", "ᾰ̓~ων";
        genitive_feminine: "ᾰ̓~οῡς", "ᾰ̓~οῑν", "ᾰ̓~ων";
        genitive_neuter: "ᾰ̓~οῡς", "ᾰ̓~οῑν", "ᾰ̓~ων";
        dative_masculine: "ᾰ̓~εῑ", "ᾰ̓~οῑν", "ᾰ̓~εσῐ|ᾰ̓~εσῐν";
        dative_feminine: "ᾰ̓~εῑ", "ᾰ̓~οῑν", "ᾰ̓~εσῐ|ᾰ̓~εσῐν";
        dative_neuter: "ᾰ̓~εῑ", "ᾰ̓~οῑν", "ᾰ̓~εσῐ|ᾰ̓~εσῐν";
        accusative_masculine: "ᾰ̓~η", "ᾰ̓~εῑ", "ᾰ̓~εῑς";
        accusative_feminine: "ᾰ̓~η", "ᾰ̓~εῑ", "ᾰ̓~εῑς";
        accusative_neuter: "ᾰ̓~ες", "ᾰ̓~εῑ", "ᾰ̓~η";
        vocative_masculine: "ᾰ̓~ες", "ᾰ̓~εῑ", "ᾰ̓~εῑς";
        vocative_feminine: "ᾰ̓~ες", "ᾰ̓~εῑ", "ᾰ̓~εῑς";
        vocative_neuter: "ᾰ̓~ες", "ᾰ̓~εῑ", "ᾰ̓~η";
        comparative: "ᾰ̓~εστερος";
        superlative: "ᾰ̓~εστᾰτος";
        adverb: "ᾰ̓~ως";
    ),
    // Ἀλλος: 25 words (ἀλλος, ἀμβροτος, ἀνθρωπῐνος, ἀγγελλομενος, ἀγγελεομενος, ἀγγελθησομενος). Like Ἀσημος, except in 16 cells: nominative_feminine sg, nominative_feminine du, nominative_feminine pl, genitive_feminine sg, genitive_feminine du, dative_feminine sg, ….
    declension!("ἀλλος", 25, "ἀ~ος", like Ἀσημος;
        nominative_masculine: _, _, _;
        nominative_feminine: "ᾰ̓~η", "ᾰ̓~ᾱ", "ᾰ̓~αι";
        nominative_neuter: _, _, _;
        genitive_masculine: _, _, _;
        genitive_feminine: "ᾰ̓~ης", "ᾰ̓~αιν", _;
        genitive_neuter: _, _, _;
        dative_masculine: _, _, _;
        dative_feminine: "ᾰ̓~ῃ", "ᾰ̓~αιν", "ᾰ̓~αις";
        dative_neuter: _, _, _;
        accusative_masculine: _, _, _;
        accusative_feminine: "ᾰ̓~ην", "ᾰ̓~ᾱ", "ᾰ̓~ᾱς";
        accusative_neuter: _, _, _;
        vocative_masculine: _, _, _;
        vocative_feminine: "ᾰ̓~η", "ᾰ̓~ᾱ", "ᾰ̓~αι";
        vocative_neuter: _, _, _;
        comparative: "ᾰ̓~ωτερος";
        superlative: "ᾰ̓~ωτᾰτος";
        adverb: _;
    ),
    // Φωνηεις: 25 words (χᾰρῐεις, φωνηεις, αἱμᾰτοεις, ῥοδοεις, ἰχθυοεις, φθογγηεις). Like Ὑληεις, except in 20 cells: nominative_feminine sg, nominative_feminine du, nominative_feminine pl, genitive_feminine sg, genitive_feminine du, genitive_feminine pl, ….
    declension!("φωνηεις", 25, "~ις", like Ὑληεις;
        nominative_masculine: _, _, _;
        nominative_feminine: "~σσᾰ", "~σσᾱ", "~σσαι";
        nominative_neuter: _, _, _;
        genitive_masculine: _, _, _;
        genitive_feminine: "~σσης", "~σσαιν", "~σσων";
        genitive_neuter: _, _, _;
        dative_masculine: _, _, "~ισῐ|~ισῐν";
        dative_feminine: "~σσῃ", "~σσαιν", "~σσαις";
        dative_neuter: _, _, "~ισῐ|~ισῐν";
        accusative_masculine: _, _, _;
        accusative_feminine: "~σσᾰν", "~σσᾱ", "~σσᾱς";
        accusative_neuter: _, _, _;
        vocative_masculine: "~ν", _, _;
        vocative_feminine: "~σσᾰ", "~σσᾱ", "~σσαι";
        vocative_neuter: _, _, _;
        comparative: "~στερος";
        superlative: "~στᾰτος";
        adverb: _;
    ),
    // Ἀκρῐτος: 24 words (ἀθεος, ἀστρολογος, ἀγᾰμος, ἀφοβος, ἀδεσποτος, ἀποθεος). Like Ἀσημος, except in 2 cells: comparative, superlative.
    declension!("ἀκρῐτος", 24, "ἀ~ος", like Ἀσημος;
        nominative_masculine: _, _, _;
        nominative_feminine: _, _, _;
        nominative_neuter: _, _, _;
        genitive_masculine: _, _, _;
        genitive_feminine: _, _, _;
        genitive_neuter: _, _, _;
        dative_masculine: _, _, _;
        dative_feminine: _, _, _;
        dative_neuter: _, _, _;
        accusative_masculine: _, _, _;
        accusative_feminine: _, _, _;
        accusative_neuter: _, _, _;
        vocative_masculine: _, _, _;
        vocative_feminine: _, _, _;
        vocative_neuter: _, _, _;
        comparative: "ᾰ̓~ωτερος";
        superlative: "ᾰ̓~ωτᾰτος";
        adverb: _;
    ),
    // Τρᾱχῠς: 24 words (τᾰχῠς, βρᾰδῠς, πολῠς, βᾰθῠς, ὀξῠς, ὠκῠς).
    declension!("τρᾱχῠς", 24, "~ῠς";
        nominative_masculine: "~ῠς", "~εῑ", "~εῑς";
        nominative_feminine: "~εῑᾰ", "~ειᾱ", "~εῑαι";
        nominative_neuter: "~ῠ", "~εῑ", "~εᾰ";
        genitive_masculine: "~εος", "~εοιν", "~εων";
        genitive_feminine: "~ειᾱς", "~ειαιν", "~ειων";
        genitive_neuter: "~εος", "~εοιν", "~εων";
        dative_masculine: "~εῑ", "~εοιν", "~εσῐ|~εσῐν";
        dative_feminine: "~ειᾳ", "~ειαιν", "~ειαις";
        dative_neuter: "~εῑ", "~εοιν", "~εσῐ|~εσῐν";
        accusative_masculine: "~ῠν", "~εῑ", "~εῑς";
        accusative_feminine: "~εῑᾰν", "~ειᾱ", "~ειᾱς";
        accusative_neuter: "~ῠ", "~εῑ", "~εᾰ";
        vocative_masculine: "~ῠ", "~εῑ", "~εῑς";
        vocative_feminine: "~εῑᾰ", "~ειᾱ", "~εῑαι";
        vocative_neuter: "~ῠ", "~εῑ", "~εᾰ";
        comparative: "~ῠτερος";
        superlative: "~ῠτᾰτος";
        adverb: "~εως";
    ),
    // Βαινων: 21 words (φαινων, ἀγειρων, μενεαινων, χαιρων, λειβων, ἀπολειπων).
    declension!("βαινων", 21, "~ι~ων";
        nominative_masculine: "~ι~ων", "~ι~οντε", "~ι~οντες";
        nominative_feminine: "~ι~ουσᾰ", "~ι~ουσᾱ", "~ι~ουσαι";
        nominative_neuter: "~ῑ~ον", "~ι~οντε", "~ι~οντᾰ";
        genitive_masculine: "~ι~οντος", "~ι~οντοιν", "~ι~οντων";
        genitive_feminine: "~ι~ουσης", "~ι~ουσαιν", "~ι~ουσων";
        genitive_neuter: "~ι~οντος", "~ι~οντοιν", "~ι~οντων";
        dative_masculine: "~ι~οντῐ", "~ι~οντοιν", "~ι~ουσῐ|~ι~ουσῐν";
        dative_feminine: "~ι~ουσῃ", "~ι~ουσαιν", "~ι~ουσαις";
        dative_neuter: "~ι~οντῐ", "~ι~οντοιν", "~ι~ουσῐ|~ι~ουσῐν";
        accusative_masculine: "~ι~οντᾰ", "~ι~οντε", "~ι~οντᾰς";
        accusative_feminine: "~ι~ουσᾰν", "~ι~ουσᾱ", "~ι~ουσᾱς";
        accusative_neuter: "~ῑ~ον", "~ι~οντε", "~ι~οντᾰ";
        vocative_masculine: "~ι~ων", "~ι~οντε", "~ι~οντες";
        vocative_feminine: "~ι~ουσᾰ", "~ι~ουσᾱ", "~ι~ουσαι";
        vocative_neuter: "~ῑ~ον", "~ι~οντε", "~ι~οντᾰ";
        comparative: derived;
        superlative: derived;
        adverb: "~ι~οντως";
    ),
    // Ἀκαρπος: 21 words (ἀναπολογητος, ἀκαρπος, ἀναπαιστος, ἀνανδρος, ἀγαθοποιος, ἀγναπτος).
    declension!("ἀκαρπος", 21, "ἀ~α~ος";
        nominative_masculine: "ᾰ̓~ᾰ~ος", "ᾰ̓~ᾰ~ω", "ᾰ̓~ᾰ~οι";
        nominative_feminine: "ᾰ̓~ᾰ~ος", "ᾰ̓~ᾰ~ω", "ᾰ̓~ᾰ~οι";
        nominative_neuter: "ᾰ̓~ᾰ~ον", "ᾰ̓~ᾰ~ω", "ᾰ̓~ᾰ~ᾰ";
        genitive_masculine: "ᾰ̓~ᾰ~ου", "ᾰ̓~ᾰ~οιν", "ᾰ̓~ᾰ~ων";
        genitive_feminine: "ᾰ̓~ᾰ~ου", "ᾰ̓~ᾰ~οιν", "ᾰ̓~ᾰ~ων";
        genitive_neuter: "ᾰ̓~ᾰ~ου", "ᾰ̓~ᾰ~οιν", "ᾰ̓~ᾰ~ων";
        dative_masculine: "ᾰ̓~ᾰ~ῳ", "ᾰ̓~ᾰ~οιν", "ᾰ̓~ᾰ~οις";
        dative_feminine: "ᾰ̓~ᾰ~ῳ", "ᾰ̓~ᾰ~οιν", "ᾰ̓~ᾰ~οις";
        dative_neuter: "ᾰ̓~ᾰ~ῳ", "ᾰ̓~ᾰ~οιν", "ᾰ̓~ᾰ~οις";
        accusative_masculine: "ᾰ̓~ᾰ~ον", "ᾰ̓~ᾰ~ω", "ᾰ̓~ᾰ~ους";
        accusative_feminine: "ᾰ̓~ᾰ~ον", "ᾰ̓~ᾰ~ω", "ᾰ̓~ᾰ~ους";
        accusative_neuter: "ᾰ̓~ᾰ~ον", "ᾰ̓~ᾰ~ω", "ᾰ̓~ᾰ~ᾰ";
        vocative_masculine: "ᾰ̓~ᾰ~ε", "ᾰ̓~ᾰ~ω", "ᾰ̓~ᾰ~οι";
        vocative_feminine: "ᾰ̓~ᾰ~ε", "ᾰ̓~ᾰ~ω", "ᾰ̓~ᾰ~οι";
        vocative_neuter: "ᾰ̓~ᾰ~ον", "ᾰ̓~ᾰ~ω", "ᾰ̓~ᾰ~ᾰ";
        comparative: "ᾰ̓~ᾰ~οτερος";
        superlative: "ᾰ̓~ᾰ~οτᾰτος";
        adverb: "ᾰ̓~ᾰ~ως";
    ),
    // Τρῐπους: 18 words (τετρᾰπους, ὀκτωπους, πολυπους, τρῐπους, ἀπους, ἰθυς). Like Ἀξῐοχρεως, except in 7 cells: nominative_masculine sg, nominative_feminine sg, nominative_neuter sg, accusative_neuter sg, vocative_masculine sg, vocative_feminine sg, ….
    declension!("τρῐπους", 18, "~υς", like Ἀξῐοχρεως;
        nominative_masculine: "~υς", _, _;
        nominative_feminine: "~υς", _, _;
        nominative_neuter: "~υν", _, _;
        genitive_masculine: _, _, _;
        genitive_feminine: _, _, _;
        genitive_neuter: _, _, _;
        dative_masculine: _, _, _;
        dative_feminine: _, _, _;
        dative_neuter: _, _, _;
        accusative_masculine: _, _, _;
        accusative_feminine: _, _, _;
        accusative_neuter: "~υν", _, _;
        vocative_masculine: "~υς", _, _;
        vocative_feminine: "~υς", _, _;
        vocative_neuter: "~υν", _, _;
        comparative: _;
        superlative: _;
        adverb: _;
    ),
    // Ἡσσων: 18 words (βρᾰχῑων, μᾰσσων, ἐλᾰσσων, κερδιων, ἐλαττων, θᾱττων). Like Πλειων, except in 14 cells: nominative_masculine pl, nominative_feminine pl, nominative_neuter pl, accusative_masculine sg, accusative_masculine pl, accusative_feminine sg, ….
    declension!("ἡσσων", 18, "~ων", like Πλειων;
        nominative_masculine: _, _, "~ονες|~ους";
        nominative_feminine: _, _, "~ονες|~ους";
        nominative_neuter: _, _, "~ονᾰ|~ω";
        genitive_masculine: _, _, _;
        genitive_feminine: _, _, _;
        genitive_neuter: _, _, _;
        dative_masculine: _, _, _;
        dative_feminine: _, _, _;
        dative_neuter: _, _, _;
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
    // Κᾰκοηθης: 17 words (πληρης, σῠνηθης, τρῐηρης, κᾰκοηθης, κᾰλοηθης, ποδᾰρκης).
    declension!("κᾰκοηθης", 17, "~ης";
        nominative_masculine: "~ης", "~ει", "~εις";
        nominative_feminine: "~ης", "~ει", "~εις";
        nominative_neuter: "~ες", "~ει", "~η";
        genitive_masculine: "~ους", "~οιν", "~ων";
        genitive_feminine: "~ους", "~οιν", "~ων";
        genitive_neuter: "~ους", "~οιν", "~ων";
        dative_masculine: "~ει", "~οιν", "~εσῐ|~εσῐν";
        dative_feminine: "~ει", "~οιν", "~εσῐ|~εσῐν";
        dative_neuter: "~ει", "~οιν", "~εσῐ|~εσῐν";
        accusative_masculine: "~η", "~ει", "~εις";
        accusative_feminine: "~η", "~ει", "~εις";
        accusative_neuter: "~ες", "~ει", "~η";
        vocative_masculine: "~ες", "~ει", "~εις";
        vocative_feminine: "~ες", "~ει", "~εις";
        vocative_neuter: "~ες", "~ει", "~η";
        comparative: "~εστερος";
        superlative: "~εστᾰτος";
        adverb: "~ως";
    ),
    // Παυων: 17 words (περῐοδευων, παιδευων, ἀγγαρευων, στρᾰτευων, κελευων, φῠτευων).
    declension!("παυων", 17, "~υων";
        nominative_masculine: "~υων", "~υοντε", "~υοντες";
        nominative_feminine: "~υουσᾰ", "~υουσᾱ", "~υουσαι";
        nominative_neuter: "~ῡον", "~υοντε", "~υοντᾰ";
        genitive_masculine: "~υοντος", "~υοντοιν", "~υοντων";
        genitive_feminine: "~υουσης", "~υουσαιν", "~υουσων";
        genitive_neuter: "~υοντος", "~υοντοιν", "~υοντων";
        dative_masculine: "~υοντῐ", "~υοντοιν", "~υουσῐ|~υουσῐν";
        dative_feminine: "~υουσῃ", "~υουσαιν", "~υουσαις";
        dative_neuter: "~υοντῐ", "~υοντοιν", "~υουσῐ|~υουσῐν";
        accusative_masculine: "~υοντᾰ", "~υοντε", "~υοντᾰς";
        accusative_feminine: "~υουσᾰν", "~υουσᾱ", "~υουσᾱς";
        accusative_neuter: "~ῡον", "~υοντε", "~υοντᾰ";
        vocative_masculine: "~υων", "~υοντε", "~υοντες";
        vocative_feminine: "~υουσᾰ", "~υουσᾱ", "~υουσαι";
        vocative_neuter: "~ῡον", "~υοντε", "~υοντᾰ";
        comparative: derived;
        superlative: derived;
        adverb: "~υοντως";
    ),
    // Ἀγων: 16 words (ἀρχων, ἀγων, ἀγγελλων, ἀγγελεων, ἀκεων, ἀκουων).
    declension!("ἀγων", 16, "ἀ~ων";
        nominative_masculine: "ᾰ̓~ων", "ᾰ̓~οντε", "ᾰ̓~οντες";
        nominative_feminine: "ᾰ̓~ουσᾰ", "ᾰ̓~ουσᾱ", "ᾰ̓~ουσαι";
        nominative_neuter: "ᾰ̓~ον", "ᾰ̓~οντε", "ᾰ̓~οντᾰ";
        genitive_masculine: "ᾰ̓~οντος", "ᾰ̓~οντοιν", "ᾰ̓~οντων";
        genitive_feminine: "ᾰ̓~ουσης", "ᾰ̓~ουσαιν", "ᾰ̓~ουσων";
        genitive_neuter: "ᾰ̓~οντος", "ᾰ̓~οντοιν", "ᾰ̓~οντων";
        dative_masculine: "ᾰ̓~οντῐ", "ᾰ̓~οντοιν", "ᾰ̓~ουσῐ|ᾰ̓~ουσῐν";
        dative_feminine: "ᾰ̓~ουσῃ", "ᾰ̓~ουσαιν", "ᾰ̓~ουσαις";
        dative_neuter: "ᾰ̓~οντῐ", "ᾰ̓~οντοιν", "ᾰ̓~ουσῐ|ᾰ̓~ουσῐν";
        accusative_masculine: "ᾰ̓~οντᾰ", "ᾰ̓~οντε", "ᾰ̓~οντᾰς";
        accusative_feminine: "ᾰ̓~ουσᾰν", "ᾰ̓~ουσᾱ", "ᾰ̓~ουσᾱς";
        accusative_neuter: "ᾰ̓~ον", "ᾰ̓~οντε", "ᾰ̓~οντᾰ";
        vocative_masculine: "ᾰ̓~ων", "ᾰ̓~οντε", "ᾰ̓~οντες";
        vocative_feminine: "ᾰ̓~ουσᾰ", "ᾰ̓~ουσᾱ", "ᾰ̓~ουσαι";
        vocative_neuter: "ᾰ̓~ον", "ᾰ̓~οντε", "ᾰ̓~οντᾰ";
        comparative: derived;
        superlative: derived;
        adverb: "ᾰ̓~οντως";
    ),
    // Δειξᾱς: 16 words (ἀγειρᾱς, ἀειρᾱς, δεισᾱς, μεινᾱς, κτεινᾱς, σπεισᾱς).
    declension!("δειξᾱς", 16, "~ι~ᾱς";
        nominative_masculine: "~ι~ᾱς", "~ι~ᾰντε", "~ι~ᾰντες";
        nominative_feminine: "~ι~ᾱσᾰ", "~ι~ᾱσᾱ", "~ι~ᾱσαι";
        nominative_neuter: "~ῑ~ᾰν", "~ι~ᾰντε", "~ι~ᾰντᾰ";
        genitive_masculine: "~ι~ᾰντος", "~ι~ᾰντοιν", "~ι~ᾰντων";
        genitive_feminine: "~ι~ᾱσης", "~ι~ᾱσαιν", "~ι~ᾱσων";
        genitive_neuter: "~ι~ᾰντος", "~ι~ᾰντοιν", "~ι~ᾰντων";
        dative_masculine: "~ι~ᾰντῐ", "~ι~ᾰντοιν", "~ι~ᾱσῐ|~ι~ᾱσῐν";
        dative_feminine: "~ι~ᾱσῃ", "~ι~ᾱσαιν", "~ι~ᾱσαις";
        dative_neuter: "~ι~ᾰντῐ", "~ι~ᾰντοιν", "~ι~ᾱσῐ|~ι~ᾱσῐν";
        accusative_masculine: "~ι~ᾰντᾰ", "~ι~ᾰντε", "~ι~ᾰντᾰς";
        accusative_feminine: "~ι~ᾱσᾰν", "~ι~ᾱσᾱ", "~ι~ᾱσᾱς";
        accusative_neuter: "~ῑ~ᾰν", "~ι~ᾰντε", "~ι~ᾰντᾰ";
        vocative_masculine: "~ι~ᾱς", "~ι~ᾰντε", "~ι~ᾰντες";
        vocative_feminine: "~ι~ᾱσᾰ", "~ι~ᾱσᾱ", "~ι~ᾱσαι";
        vocative_neuter: "~ῑ~ᾰν", "~ι~ᾰντε", "~ι~ᾰντᾰ";
        comparative: derived;
        superlative: derived;
        adverb: "~ι~ᾰντως";
    ),
    // Τοιοσδε: 16 words (τοιοσδε, τοσοσδε, μεγᾰλητωρ, γενομενᾰ, ὑψηνωρ, λῡομενη).
    declension!("τοιοσδε", 16, "~";
        nominative_masculine: "~", "~ε", "~ες";
        nominative_feminine: "~", "~ε", "~ες";
        nominative_neuter: "~", "~ε", "~ᾰ";
        genitive_masculine: "~ος", "~οιν", "~ων";
        genitive_feminine: "~ος", "~οιν", "~ων";
        genitive_neuter: "~ος", "~οιν", "~ων";
        dative_masculine: "~ῐ", "~οιν", "~σῐ";
        dative_feminine: "~ῐ", "~οιν", "~σῐ";
        dative_neuter: "~ῐ", "~οιν", "~σῐ";
        accusative_masculine: "~ᾰ", "~ε", "~ᾰς";
        accusative_feminine: "~ᾰ", "~ε", "~ᾰς";
        accusative_neuter: "~", "~ε", "~ᾰ";
        vocative_masculine: "~", "~ε", "~ες";
        vocative_feminine: "~", "~ε", "~ες";
        vocative_neuter: "~", "~ε", "~ᾰ";
        comparative: derived;
        superlative: derived;
        adverb: "~ως";
    ),
    // Ἀδιψος: 16 words (ἀδικος, ἀγριελαιος, ἀντιλογος, ἀοριστος, ἀμαυροστικτος, ἀξιολογος).
    declension!("ἀδιψος", 16, "ἀ~ι~ος";
        nominative_masculine: "ᾰ̓~ῐ~ος", "ᾰ̓~ῐ~ω", "ᾰ̓~ῐ~οι";
        nominative_feminine: "ᾰ̓~ῐ~ος", "ᾰ̓~ῐ~ω", "ᾰ̓~ῐ~οι";
        nominative_neuter: "ᾰ̓~ῐ~ον", "ᾰ̓~ῐ~ω", "ᾰ̓~ῐ~ᾰ";
        genitive_masculine: "ᾰ̓~ῐ~ου", "ᾰ̓~ῐ~οιν", "ᾰ̓~ῐ~ων";
        genitive_feminine: "ᾰ̓~ῐ~ου", "ᾰ̓~ῐ~οιν", "ᾰ̓~ῐ~ων";
        genitive_neuter: "ᾰ̓~ῐ~ου", "ᾰ̓~ῐ~οιν", "ᾰ̓~ῐ~ων";
        dative_masculine: "ᾰ̓~ῐ~ῳ", "ᾰ̓~ῐ~οιν", "ᾰ̓~ῐ~οις";
        dative_feminine: "ᾰ̓~ῐ~ῳ", "ᾰ̓~ῐ~οιν", "ᾰ̓~ῐ~οις";
        dative_neuter: "ᾰ̓~ῐ~ῳ", "ᾰ̓~ῐ~οιν", "ᾰ̓~ῐ~οις";
        accusative_masculine: "ᾰ̓~ῐ~ον", "ᾰ̓~ῐ~ω", "ᾰ̓~ῐ~ους";
        accusative_feminine: "ᾰ̓~ῐ~ον", "ᾰ̓~ῐ~ω", "ᾰ̓~ῐ~ους";
        accusative_neuter: "ᾰ̓~ῐ~ον", "ᾰ̓~ῐ~ω", "ᾰ̓~ῐ~ᾰ";
        vocative_masculine: "ᾰ̓~ῐ~ε", "ᾰ̓~ῐ~ω", "ᾰ̓~ῐ~οι";
        vocative_feminine: "ᾰ̓~ῐ~ε", "ᾰ̓~ῐ~ω", "ᾰ̓~ῐ~οι";
        vocative_neuter: "ᾰ̓~ῐ~ον", "ᾰ̓~ῐ~ω", "ᾰ̓~ῐ~ᾰ";
        comparative: "ᾰ̓~ῐ~ωτερος";
        superlative: "ᾰ̓~ῐ~ωτᾰτος";
        adverb: "ᾰ̓~ῐ~ως";
    ),
    // Ὑπαιθρος: 13 words (ὑπογειος, ὑπερῳος, ὑποσπονδος, ὑπηκοος, ὑμενοπτερος, ὑψηλοκρημνος).
    declension!("ὑπαιθρος", 13, "ὑ~ος";
        nominative_masculine: "ῠ̔~ος", "ῠ̔~ω", "ῠ̔~οι";
        nominative_feminine: "ῠ̔~ος", "ῠ̔~ω", "ῠ̔~οι";
        nominative_neuter: "ῠ̔~ον", "ῠ̔~ω", "ῠ̔~ᾰ";
        genitive_masculine: "ῠ̔~ου", "ῠ̔~οιν", "ῠ̔~ων";
        genitive_feminine: "ῠ̔~ου", "ῠ̔~οιν", "ῠ̔~ων";
        genitive_neuter: "ῠ̔~ου", "ῠ̔~οιν", "ῠ̔~ων";
        dative_masculine: "ῠ̔~ῳ", "ῠ̔~οιν", "ῠ̔~οις";
        dative_feminine: "ῠ̔~ῳ", "ῠ̔~οιν", "ῠ̔~οις";
        dative_neuter: "ῠ̔~ῳ", "ῠ̔~οιν", "ῠ̔~οις";
        accusative_masculine: "ῠ̔~ον", "ῠ̔~ω", "ῠ̔~ους";
        accusative_feminine: "ῠ̔~ον", "ῠ̔~ω", "ῠ̔~ους";
        accusative_neuter: "ῠ̔~ον", "ῠ̔~ω", "ῠ̔~ᾰ";
        vocative_masculine: "ῠ̔~ε", "ῠ̔~ω", "ῠ̔~οι";
        vocative_feminine: "ῠ̔~ε", "ῠ̔~ω", "ῠ̔~οι";
        vocative_neuter: "ῠ̔~ον", "ῠ̔~ω", "ῠ̔~ᾰ";
        comparative: "ῠ̔~οτερος";
        superlative: "ῠ̔~οτᾰτος";
        adverb: "ῠ̔~ως";
    ),
    // Ἀσυνδετος: 13 words (ἀνυποδητος, ἀνδρογυνος, ἀμεθυστος, ἀσυνδετος, ἀργυροηλος, ἀργυρεος).
    declension!("ἀσυνδετος", 13, "ἀ~υ~ος";
        nominative_masculine: "ᾰ̓~ῠ~ος", "ᾰ̓~ῠ~ω", "ᾰ̓~ῠ~οι";
        nominative_feminine: "ᾰ̓~ῠ~ος", "ᾰ̓~ῠ~ω", "ᾰ̓~ῠ~οι";
        nominative_neuter: "ᾰ̓~ῠ~ον", "ᾰ̓~ῠ~ω", "ᾰ̓~ῠ~ᾰ";
        genitive_masculine: "ᾰ̓~ῠ~ου", "ᾰ̓~ῠ~οιν", "ᾰ̓~ῠ~ων";
        genitive_feminine: "ᾰ̓~ῠ~ου", "ᾰ̓~ῠ~οιν", "ᾰ̓~ῠ~ων";
        genitive_neuter: "ᾰ̓~ῠ~ου", "ᾰ̓~ῠ~οιν", "ᾰ̓~ῠ~ων";
        dative_masculine: "ᾰ̓~ῠ~ῳ", "ᾰ̓~ῠ~οιν", "ᾰ̓~ῠ~οις";
        dative_feminine: "ᾰ̓~ῠ~ῳ", "ᾰ̓~ῠ~οιν", "ᾰ̓~ῠ~οις";
        dative_neuter: "ᾰ̓~ῠ~ῳ", "ᾰ̓~ῠ~οιν", "ᾰ̓~ῠ~οις";
        accusative_masculine: "ᾰ̓~ῠ~ον", "ᾰ̓~ῠ~ω", "ᾰ̓~ῠ~ους";
        accusative_feminine: "ᾰ̓~ῠ~ον", "ᾰ̓~ῠ~ω", "ᾰ̓~ῠ~ους";
        accusative_neuter: "ᾰ̓~ῠ~ον", "ᾰ̓~ῠ~ω", "ᾰ̓~ῠ~ᾰ";
        vocative_masculine: "ᾰ̓~ῠ~ε", "ᾰ̓~ῠ~ω", "ᾰ̓~ῠ~οι";
        vocative_feminine: "ᾰ̓~ῠ~ε", "ᾰ̓~ῠ~ω", "ᾰ̓~ῠ~οι";
        vocative_neuter: "ᾰ̓~ῠ~ον", "ᾰ̓~ῠ~ω", "ᾰ̓~ῠ~ᾰ";
        comparative: "ᾰ̓~ῠ~ωτερος";
        superlative: "ᾰ̓~ῠ~ωτᾰτος";
        adverb: "ᾰ̓~ῠ~ως";
    ),
    // Ἀγραμματος: 13 words (ἀκαθαρτος, ἀκαταληπτος, ἀνανταποδοτος, ἀκᾰτασκευαστος, ἀγραμματος, ἀδαπανος).
    declension!("ἀγραμματος", 13, "ἀ~α~α~ος";
        nominative_masculine: "ᾰ̓~ᾰ~ᾰ~ος", "ᾰ̓~ᾰ~ᾰ~ω", "ᾰ̓~ᾰ~ᾰ~οι";
        nominative_feminine: "ᾰ̓~ᾰ~ᾰ~ος", "ᾰ̓~ᾰ~ᾰ~ω", "ᾰ̓~ᾰ~ᾰ~οι";
        nominative_neuter: "ᾰ̓~ᾰ~ᾰ~ον", "ᾰ̓~ᾰ~ᾰ~ω", "ᾰ̓~ᾰ~ᾰ~ᾰ";
        genitive_masculine: "ᾰ̓~ᾰ~ᾰ~ου", "ᾰ̓~ᾰ~ᾰ~οιν", "ᾰ̓~ᾰ~ᾰ~ων";
        genitive_feminine: "ᾰ̓~ᾰ~ᾰ~ου", "ᾰ̓~ᾰ~ᾰ~οιν", "ᾰ̓~ᾰ~ᾰ~ων";
        genitive_neuter: "ᾰ̓~ᾰ~ᾰ~ου", "ᾰ̓~ᾰ~ᾰ~οιν", "ᾰ̓~ᾰ~ᾰ~ων";
        dative_masculine: "ᾰ̓~ᾰ~ᾰ~ῳ", "ᾰ̓~ᾰ~ᾰ~οιν", "ᾰ̓~ᾰ~ᾰ~οις";
        dative_feminine: "ᾰ̓~ᾰ~ᾰ~ῳ", "ᾰ̓~ᾰ~ᾰ~οιν", "ᾰ̓~ᾰ~ᾰ~οις";
        dative_neuter: "ᾰ̓~ᾰ~ᾰ~ῳ", "ᾰ̓~ᾰ~ᾰ~οιν", "ᾰ̓~ᾰ~ᾰ~οις";
        accusative_masculine: "ᾰ̓~ᾰ~ᾰ~ον", "ᾰ̓~ᾰ~ᾰ~ω", "ᾰ̓~ᾰ~ᾰ~ους";
        accusative_feminine: "ᾰ̓~ᾰ~ᾰ~ον", "ᾰ̓~ᾰ~ᾰ~ω", "ᾰ̓~ᾰ~ᾰ~ους";
        accusative_neuter: "ᾰ̓~ᾰ~ᾰ~ον", "ᾰ̓~ᾰ~ᾰ~ω", "ᾰ̓~ᾰ~ᾰ~ᾰ";
        vocative_masculine: "ᾰ̓~ᾰ~ᾰ~ε", "ᾰ̓~ᾰ~ᾰ~ω", "ᾰ̓~ᾰ~ᾰ~οι";
        vocative_feminine: "ᾰ̓~ᾰ~ᾰ~ε", "ᾰ̓~ᾰ~ᾰ~ω", "ᾰ̓~ᾰ~ᾰ~οι";
        vocative_neuter: "ᾰ̓~ᾰ~ᾰ~ον", "ᾰ̓~ᾰ~ᾰ~ω", "ᾰ̓~ᾰ~ᾰ~ᾰ";
        comparative: "ᾰ̓~ᾰ~ᾰ~ωτερος";
        superlative: "ᾰ̓~ᾰ~ᾰ~ωτᾰτος";
        adverb: "ᾰ̓~ᾰ~ᾰ~ως";
    ),
    // Ἀγνως: 12 words (ἀλεκτωρ, ἀκμων, ἀφρων, ἀγνως, ἀπατωρ, ἀμεινων).
    declension!("ἀγνως", 12, "ἀ~ω~";
        nominative_masculine: "ᾰ̓~ω~", "ᾰ̓~ο~ε", "ᾰ̓~ο~ες";
        nominative_feminine: "ᾰ̓~ω~", "ᾰ̓~ο~ε", "ᾰ̓~ο~ες";
        nominative_neuter: "ᾰ̓~ο~", "ᾰ̓~ο~ε", "ᾰ̓~ο~ᾰ";
        genitive_masculine: "ᾰ̓~ο~ος", "ᾰ̓~ο~οιν", "ᾰ̓~ο~ων";
        genitive_feminine: "ᾰ̓~ο~ος", "ᾰ̓~ο~οιν", "ᾰ̓~ο~ων";
        genitive_neuter: "ᾰ̓~ο~ος", "ᾰ̓~ο~οιν", "ᾰ̓~ο~ων";
        dative_masculine: "ᾰ̓~ο~ῐ", "ᾰ̓~ο~οιν", "ᾰ̓~ο~σῐ";
        dative_feminine: "ᾰ̓~ο~ῐ", "ᾰ̓~ο~οιν", "ᾰ̓~ο~σῐ";
        dative_neuter: "ᾰ̓~ο~ῐ", "ᾰ̓~ο~οιν", "ᾰ̓~ο~σῐ";
        accusative_masculine: "ᾰ̓~ο~ᾰ", "ᾰ̓~ο~ε", "ᾰ̓~ο~ᾰς";
        accusative_feminine: "ᾰ̓~ο~ᾰ", "ᾰ̓~ο~ε", "ᾰ̓~ο~ᾰς";
        accusative_neuter: "ᾰ̓~ο~", "ᾰ̓~ο~ε", "ᾰ̓~ο~ᾰ";
        vocative_masculine: "ᾰ̓~ο~", "ᾰ̓~ο~ε", "ᾰ̓~ο~ες";
        vocative_feminine: "ᾰ̓~ο~", "ᾰ̓~ο~ε", "ᾰ̓~ο~ες";
        vocative_neuter: "ᾰ̓~ο~", "ᾰ̓~ο~ε", "ᾰ̓~ο~ᾰ";
        comparative: derived;
        superlative: derived;
        adverb: "ᾰ̓~ο~ως";
    ),
    // Ἀρχικος: 12 words (ἀριστερος, ἀττικος, ἀποπληκτικος, ἀμφορικος, ἀληθινος, ἀφαιρετικος). Like Ἀδιψος, except in 22 cells: nominative_feminine sg, nominative_feminine du, nominative_feminine pl, genitive_masculine sg, genitive_masculine du, genitive_feminine sg, ….
    declension!("ἀρχικος", 12, "ἀ~ι~ος", like Ἀδιψος;
        nominative_masculine: _, _, _;
        nominative_feminine: "ᾰ̓~ῐ~η", "ᾰ̓~ῐ~ᾱ", "ᾰ̓~ῐ~αι";
        nominative_neuter: _, _, _;
        genitive_masculine: "ᾰ̓~ῐ~οῡ", "ᾰ̓~ῐ~οῑν", _;
        genitive_feminine: "ᾰ̓~ῐ~ης", "ᾰ̓~ῐ~αῑν", _;
        genitive_neuter: "ᾰ̓~ῐ~οῡ", "ᾰ̓~ῐ~οῑν", _;
        dative_masculine: _, "ᾰ̓~ῐ~οῑν", "ᾰ̓~ῐ~οῑς";
        dative_feminine: "ᾰ̓~ῐ~ῃ", "ᾰ̓~ῐ~αῑν", "ᾰ̓~ῐ~αῑς";
        dative_neuter: _, "ᾰ̓~ῐ~οῑν", "ᾰ̓~ῐ~οῑς";
        accusative_masculine: _, _, _;
        accusative_feminine: "ᾰ̓~ῐ~ην", "ᾰ̓~ῐ~ᾱ", "ᾰ̓~ῐ~ᾱς";
        accusative_neuter: _, _, _;
        vocative_masculine: _, _, _;
        vocative_feminine: "ᾰ̓~ῐ~η", "ᾰ̓~ῐ~ᾱ", "ᾰ̓~ῐ~αι";
        vocative_neuter: _, _, _;
        comparative: _;
        superlative: _;
        adverb: _;
    ),
    // Ἀκροτερος: 12 words (ἀτολμοτερος, ἀληθεστερος, ἀτερπεστερος, ἀηδεστερος, ἀδεεστερος, ἀληπτοτερος). Like Ἀλλος, except in 8 cells: nominative_feminine sg, genitive_feminine sg, dative_feminine sg, accusative_feminine sg, vocative_feminine sg, comparative, ….
    declension!("ἀκροτερος", 12, "ἀ~ος", like Ἀλλος;
        nominative_masculine: _, _, _;
        nominative_feminine: "ᾰ̓~ᾱ", _, _;
        nominative_neuter: _, _, _;
        genitive_masculine: _, _, _;
        genitive_feminine: "ᾰ̓~ᾱς", _, _;
        genitive_neuter: _, _, _;
        dative_masculine: _, _, _;
        dative_feminine: "ᾰ̓~ᾳ", _, _;
        dative_neuter: _, _, _;
        accusative_masculine: _, _, _;
        accusative_feminine: "ᾰ̓~ᾱν", _, _;
        accusative_neuter: _, _, _;
        vocative_masculine: _, _, _;
        vocative_feminine: "ᾰ̓~ᾱ", _, _;
        vocative_neuter: _, _, _;
        comparative: derived;
        superlative: derived;
        adverb: "ᾰ̓~ον";
    ),
    // Ἀποδεικτῐκος: 12 words (ἀργος, ἀνοικτος, ἀρκετος, ἀνᾰλῠτῐκος, ἀχαιῐκος, ἀρσενῐκος). Like Ἀλλος, except in 11 cells: genitive_masculine sg, genitive_masculine du, genitive_feminine du, genitive_neuter sg, genitive_neuter du, dative_masculine du, ….
    declension!("ἀποδεικτῐκος", 12, "ἀ~ος", like Ἀλλος;
        nominative_masculine: _, _, _;
        nominative_feminine: _, _, _;
        nominative_neuter: _, _, _;
        genitive_masculine: "ᾰ̓~οῡ", "ᾰ̓~οῑν", _;
        genitive_feminine: _, "ᾰ̓~αῑν", _;
        genitive_neuter: "ᾰ̓~οῡ", "ᾰ̓~οῑν", _;
        dative_masculine: _, "ᾰ̓~οῑν", "ᾰ̓~οῑς";
        dative_feminine: _, "ᾰ̓~αῑν", "ᾰ̓~αῑς";
        dative_neuter: _, "ᾰ̓~οῑν", "ᾰ̓~οῑς";
        accusative_masculine: _, _, _;
        accusative_feminine: _, _, _;
        accusative_neuter: _, _, _;
        vocative_masculine: _, _, _;
        vocative_feminine: _, _, _;
        vocative_neuter: _, _, _;
        comparative: _;
        superlative: _;
        adverb: _;
    ),
    // Ἀβατος: 11 words (ἀγραφος, ἀκεφαλος, ἀδηφαγος, ἀγναφος, ἀνωμαλος, ἀναλογος). Like Ἀκαρπος, except in 2 cells: comparative, superlative.
    declension!("ἀβατος", 11, "ἀ~α~ος", like Ἀκαρπος;
        nominative_masculine: _, _, _;
        nominative_feminine: _, _, _;
        nominative_neuter: _, _, _;
        genitive_masculine: _, _, _;
        genitive_feminine: _, _, _;
        genitive_neuter: _, _, _;
        dative_masculine: _, _, _;
        dative_feminine: _, _, _;
        dative_neuter: _, _, _;
        accusative_masculine: _, _, _;
        accusative_feminine: _, _, _;
        accusative_neuter: _, _, _;
        vocative_masculine: _, _, _;
        vocative_feminine: _, _, _;
        vocative_neuter: _, _, _;
        comparative: "ᾰ̓~ᾰ~ωτερος";
        superlative: "ᾰ̓~ᾰ~ωτᾰτος";
        adverb: _;
    ),
    // Φαῡλος: 11 words (δοῡλος, φαῡλος, παῡρος, κοῡφος, σαῡλος, κραῡρος).
    declension!("φαῡλος", 11, "~ῡ~ος";
        nominative_masculine: "~ῡ~ος", "~υ~ω", "~ῡ~οι";
        nominative_feminine: "~υ~η", "~υ~ᾱ", "~ῡ~αι";
        nominative_neuter: "~ῡ~ον", "~υ~ω", "~ῡ~ᾰ";
        genitive_masculine: "~υ~ου", "~υ~οιν", "~υ~ων";
        genitive_feminine: "~υ~ης", "~υ~αιν", "~υ~ων";
        genitive_neuter: "~υ~ου", "~υ~οιν", "~υ~ων";
        dative_masculine: "~υ~ῳ", "~υ~οιν", "~υ~οις";
        dative_feminine: "~υ~ῃ", "~υ~αιν", "~υ~αις";
        dative_neuter: "~υ~ῳ", "~υ~οιν", "~υ~οις";
        accusative_masculine: "~ῡ~ον", "~υ~ω", "~υ~ους";
        accusative_feminine: "~υ~ην", "~υ~ᾱ", "~υ~ᾱς";
        accusative_neuter: "~ῡ~ον", "~υ~ω", "~ῡ~ᾰ";
        vocative_masculine: "~ῡ~ε", "~υ~ω", "~ῡ~οι";
        vocative_feminine: "~υ~η", "~υ~ᾱ", "~ῡ~αι";
        vocative_neuter: "~ῡ~ον", "~υ~ω", "~ῡ~ᾰ";
        comparative: "~υ~οτερος";
        superlative: "~υ~οτᾰτος";
        adverb: "~υ~ως";
    ),
    // Ἐρημος: 10 words (ἀλεξανδρῑνος, λᾰτῑνος, λῐσπος, κᾰκομᾰζᾱλος, δελτος, λᾰγνος). Like Ἐσχᾰτος, except in 2 cells: comparative, superlative.
    declension!("ἐρημος", 10, "~ος", like Ἐσχᾰτος;
        nominative_masculine: _, _, _;
        nominative_feminine: _, _, _;
        nominative_neuter: _, _, _;
        genitive_masculine: _, _, _;
        genitive_feminine: _, _, _;
        genitive_neuter: _, _, _;
        dative_masculine: _, _, _;
        dative_feminine: _, _, _;
        dative_neuter: _, _, _;
        accusative_masculine: _, _, _;
        accusative_feminine: _, _, _;
        accusative_neuter: _, _, _;
        vocative_masculine: _, _, _;
        vocative_feminine: _, _, _;
        vocative_neuter: _, _, _;
        comparative: "~οτερος";
        superlative: "~οτᾰτος";
        adverb: _;
    ),
    // Ἀγοραῑος: 10 words (ἀρχαῑος, ἀνδρεῑος, ἀργεῑος, ἀστεῑος, ἀλλοῑος, ἀθηναῑος).
    declension!("ἀγοραῑος", 10, "ἀ~ῑος";
        nominative_masculine: "ᾰ̓~ῑος", "ᾰ̓~ιω", "ᾰ̓~ῑοι";
        nominative_feminine: "ᾰ̓~ιᾱ", "ᾰ̓~ιᾱ", "ᾰ̓~ῑαι";
        nominative_neuter: "ᾰ̓~ῑον", "ᾰ̓~ιω", "ᾰ̓~ῑᾰ";
        genitive_masculine: "ᾰ̓~ιου", "ᾰ̓~ιοιν", "ᾰ̓~ιων";
        genitive_feminine: "ᾰ̓~ιᾱς", "ᾰ̓~ιαιν", "ᾰ̓~ιων";
        genitive_neuter: "ᾰ̓~ιου", "ᾰ̓~ιοιν", "ᾰ̓~ιων";
        dative_masculine: "ᾰ̓~ιῳ", "ᾰ̓~ιοιν", "ᾰ̓~ιοις";
        dative_feminine: "ᾰ̓~ιᾳ", "ᾰ̓~ιαιν", "ᾰ̓~ιαις";
        dative_neuter: "ᾰ̓~ιῳ", "ᾰ̓~ιοιν", "ᾰ̓~ιοις";
        accusative_masculine: "ᾰ̓~ῑον", "ᾰ̓~ιω", "ᾰ̓~ιους";
        accusative_feminine: "ᾰ̓~ιᾱν", "ᾰ̓~ιᾱ", "ᾰ̓~ιᾱς";
        accusative_neuter: "ᾰ̓~ῑον", "ᾰ̓~ιω", "ᾰ̓~ῑᾰ";
        vocative_masculine: "ᾰ̓~ῑε", "ᾰ̓~ιω", "ᾰ̓~ῑοι";
        vocative_feminine: "ᾰ̓~ιᾱ", "ᾰ̓~ιᾱ", "ᾰ̓~ῑαι";
        vocative_neuter: "ᾰ̓~ῑον", "ᾰ̓~ιω", "ᾰ̓~ῑᾰ";
        comparative: "ᾰ̓~ιοτερος";
        superlative: "ᾰ̓~ιοτᾰτος";
        adverb: "ᾰ̓~ιως";
    ),
    // Γαῡρος: 10 words (πᾰνοῡργος, λαῡρος, θοῡρος, γαῡρος, κᾰκοῡργος, σκηπτοῡχος). Like Φαῡλος, except in 14 cells: nominative_feminine sg, nominative_feminine du, nominative_feminine pl, genitive_feminine sg, genitive_feminine du, dative_feminine sg, ….
    declension!("γαῡρος", 10, "~ῡ~ος", like Φαῡλος;
        nominative_masculine: _, _, _;
        nominative_feminine: "~ῡ~ος", "~υ~ω", "~ῡ~οι";
        nominative_neuter: _, _, _;
        genitive_masculine: _, _, _;
        genitive_feminine: "~υ~ου", "~υ~οιν", _;
        genitive_neuter: _, _, _;
        dative_masculine: _, _, _;
        dative_feminine: "~υ~ῳ", "~υ~οιν", "~υ~οις";
        dative_neuter: _, _, _;
        accusative_masculine: _, _, _;
        accusative_feminine: "~ῡ~ον", "~υ~ω", "~υ~ους";
        accusative_neuter: _, _, _;
        vocative_masculine: _, _, _;
        vocative_feminine: "~ῡ~ε", "~υ~ω", "~ῡ~οι";
        vocative_neuter: _, _, _;
        comparative: _;
        superlative: _;
        adverb: _;
    ),
    // Πλωϊμος: 10 words (θεϊκος, τρωϊκος, πλωϊμος, ἡρωϊκος, πρωϊμος, σφωϊτερος).
    declension!("πλωϊμος", 10, "~ϊ~ος";
        nominative_masculine: "~ῐ̈~ος", "~ῐ̈~ω", "~ῐ̈~οι";
        nominative_feminine: "~ῐ̈~η", "~ῐ̈~ᾱ", "~ῐ̈~αι";
        nominative_neuter: "~ῐ̈~ον", "~ῐ̈~ω", "~ῐ̈~ᾰ";
        genitive_masculine: "~ῐ̈~οῡ", "~ῐ̈~οῑν", "~ῐ̈~ων";
        genitive_feminine: "~ῐ̈~ης", "~ῐ̈~αῑν", "~ῐ̈~ων";
        genitive_neuter: "~ῐ̈~οῡ", "~ῐ̈~οῑν", "~ῐ̈~ων";
        dative_masculine: "~ῐ̈~ῳ", "~ῐ̈~οῑν", "~ῐ̈~οῑς";
        dative_feminine: "~ῐ̈~ῃ", "~ῐ̈~αῑν", "~ῐ̈~αῑς";
        dative_neuter: "~ῐ̈~ῳ", "~ῐ̈~οῑν", "~ῐ̈~οῑς";
        accusative_masculine: "~ῐ̈~ον", "~ῐ̈~ω", "~ῐ̈~ους";
        accusative_feminine: "~ῐ̈~ην", "~ῐ̈~ᾱ", "~ῐ̈~ᾱς";
        accusative_neuter: "~ῐ̈~ον", "~ῐ̈~ω", "~ῐ̈~ᾰ";
        vocative_masculine: "~ῐ̈~ε", "~ῐ̈~ω", "~ῐ̈~οι";
        vocative_feminine: "~ῐ̈~η", "~ῐ̈~ᾱ", "~ῐ̈~αι";
        vocative_neuter: "~ῐ̈~ον", "~ῐ̈~ω", "~ῐ̈~ᾰ";
        comparative: "~ῐ̈~ωτερος";
        superlative: "~ῐ̈~ωτᾰτος";
        adverb: "~ῐ̈~ως";
    ),
    // Ὑποθετικος: 10 words (ὑψιστος, ὑποθετικος, ὑπερθετικος, ὑπνωτικος, ὑμενινος, ὑψικομος).
    declension!("ὑποθετικος", 10, "ὑ~ι~ος";
        nominative_masculine: "ῠ̔~ῐ~ος", "ῠ̔~ῐ~ω", "ῠ̔~ῐ~οι";
        nominative_feminine: "ῠ̔~ῐ~η", "ῠ̔~ῐ~ᾱ", "ῠ̔~ῐ~αι";
        nominative_neuter: "ῠ̔~ῐ~ον", "ῠ̔~ῐ~ω", "ῠ̔~ῐ~ᾰ";
        genitive_masculine: "ῠ̔~ῐ~οῡ", "ῠ̔~ῐ~οῑν", "ῠ̔~ῐ~ων";
        genitive_feminine: "ῠ̔~ῐ~ης", "ῠ̔~ῐ~αῑν", "ῠ̔~ῐ~ων";
        genitive_neuter: "ῠ̔~ῐ~οῡ", "ῠ̔~ῐ~οῑν", "ῠ̔~ῐ~ων";
        dative_masculine: "ῠ̔~ῐ~ῳ", "ῠ̔~ῐ~οῑν", "ῠ̔~ῐ~οῑς";
        dative_feminine: "ῠ̔~ῐ~ῃ", "ῠ̔~ῐ~αῑν", "ῠ̔~ῐ~αῑς";
        dative_neuter: "ῠ̔~ῐ~ῳ", "ῠ̔~ῐ~οῑν", "ῠ̔~ῐ~οῑς";
        accusative_masculine: "ῠ̔~ῐ~ον", "ῠ̔~ῐ~ω", "ῠ̔~ῐ~ους";
        accusative_feminine: "ῠ̔~ῐ~ην", "ῠ̔~ῐ~ᾱ", "ῠ̔~ῐ~ᾱς";
        accusative_neuter: "ῠ̔~ῐ~ον", "ῠ̔~ῐ~ω", "ῠ̔~ῐ~ᾰ";
        vocative_masculine: "ῠ̔~ῐ~ε", "ῠ̔~ῐ~ω", "ῠ̔~ῐ~οι";
        vocative_feminine: "ῠ̔~ῐ~η", "ῠ̔~ῐ~ᾱ", "ῠ̔~ῐ~αι";
        vocative_neuter: "ῠ̔~ῐ~ον", "ῠ̔~ῐ~ω", "ῠ̔~ῐ~ᾰ";
        comparative: "ῠ̔~ῐ~ωτερος";
        superlative: "ῠ̔~ῐ~ωτᾰτος";
        adverb: "ῠ̔~ῐ~ως";
    ),
    // Ἀμαθης: 9 words (ἀπαθης, ἀμαθης, ἀσφαλης, ἀφανης, ἀσαφης, ἀραχνοειδης).
    declension!("ἀμαθης", 9, "ἀ~α~ης";
        nominative_masculine: "ᾰ̓~ᾰ~ης", "ᾰ̓~ᾰ~εῑ", "ᾰ̓~ᾰ~εῑς";
        nominative_feminine: "ᾰ̓~ᾰ~ης", "ᾰ̓~ᾰ~εῑ", "ᾰ̓~ᾰ~εῑς";
        nominative_neuter: "ᾰ̓~ᾰ~ες", "ᾰ̓~ᾰ~εῑ", "ᾰ̓~ᾰ~η";
        genitive_masculine: "ᾰ̓~ᾰ~οῡς", "ᾰ̓~ᾰ~οῑν", "ᾰ̓~ᾰ~ων";
        genitive_feminine: "ᾰ̓~ᾰ~οῡς", "ᾰ̓~ᾰ~οῑν", "ᾰ̓~ᾰ~ων";
        genitive_neuter: "ᾰ̓~ᾰ~οῡς", "ᾰ̓~ᾰ~οῑν", "ᾰ̓~ᾰ~ων";
        dative_masculine: "ᾰ̓~ᾰ~εῑ", "ᾰ̓~ᾰ~οῑν", "ᾰ̓~ᾰ~εσῐ|ᾰ̓~ᾰ~εσῐν";
        dative_feminine: "ᾰ̓~ᾰ~εῑ", "ᾰ̓~ᾰ~οῑν", "ᾰ̓~ᾰ~εσῐ|ᾰ̓~ᾰ~εσῐν";
        dative_neuter: "ᾰ̓~ᾰ~εῑ", "ᾰ̓~ᾰ~οῑν", "ᾰ̓~ᾰ~εσῐ|ᾰ̓~ᾰ~εσῐν";
        accusative_masculine: "ᾰ̓~ᾰ~η", "ᾰ̓~ᾰ~εῑ", "ᾰ̓~ᾰ~εῑς";
        accusative_feminine: "ᾰ̓~ᾰ~η", "ᾰ̓~ᾰ~εῑ", "ᾰ̓~ᾰ~εῑς";
        accusative_neuter: "ᾰ̓~ᾰ~ες", "ᾰ̓~ᾰ~εῑ", "ᾰ̓~ᾰ~η";
        vocative_masculine: "ᾰ̓~ᾰ~ες", "ᾰ̓~ᾰ~εῑ", "ᾰ̓~ᾰ~εῑς";
        vocative_feminine: "ᾰ̓~ᾰ~ες", "ᾰ̓~ᾰ~εῑ", "ᾰ̓~ᾰ~εῑς";
        vocative_neuter: "ᾰ̓~ᾰ~ες", "ᾰ̓~ᾰ~εῑ", "ᾰ̓~ᾰ~η";
        comparative: "ᾰ̓~ᾰ~εστερος";
        superlative: "ᾰ̓~ᾰ~εστᾰτος";
        adverb: "ᾰ̓~ᾰ~ως";
    ),
    // Ὑψηλος: 9 words (ὑψηλος, ὑποδεδειγμενος, ὑστεραῑος, ὑπερβολῐκος, ὑπηοῑος, ὑποβοηθουμενος). Like Ὑπαιθρος, except in 22 cells: nominative_feminine sg, nominative_feminine du, nominative_feminine pl, genitive_masculine sg, genitive_masculine du, genitive_feminine sg, ….
    declension!("ὑψηλος", 9, "ὑ~ος", like Ὑπαιθρος;
        nominative_masculine: _, _, _;
        nominative_feminine: "ῠ̔~η", "ῠ̔~ᾱ", "ῠ̔~αι";
        nominative_neuter: _, _, _;
        genitive_masculine: "ῠ̔~οῡ", "ῠ̔~οῑν", _;
        genitive_feminine: "ῠ̔~ης", "ῠ̔~αῑν", _;
        genitive_neuter: "ῠ̔~οῡ", "ῠ̔~οῑν", _;
        dative_masculine: _, "ῠ̔~οῑν", "ῠ̔~οῑς";
        dative_feminine: "ῠ̔~ῃ", "ῠ̔~αῑν", "ῠ̔~αῑς";
        dative_neuter: _, "ῠ̔~οῑν", "ῠ̔~οῑς";
        accusative_masculine: _, _, _;
        accusative_feminine: "ῠ̔~ην", "ῠ̔~ᾱ", "ῠ̔~ᾱς";
        accusative_neuter: _, _, _;
        vocative_masculine: _, _, _;
        vocative_feminine: "ῠ̔~η", "ῠ̔~ᾱ", "ῠ̔~αι";
        vocative_neuter: _, _, _;
        comparative: _;
        superlative: _;
        adverb: _;
    ),
    // Κᾰλλῐθρῐξ: 9 words (ἡλῐξ, κᾰλλῐθρῐξ, πτωξ, ὁμηλῐξ, χρῡσᾰμπῠξ, μελᾰνθρῐξ).
    declension!("κᾰλλῐθρῐξ", 9, "~ξ";
        nominative_masculine: "~ξ", "~κε", "~κες";
        nominative_feminine: "~ξ", "~κε", "~κες";
        nominative_neuter: "~ξ", "~κε", "~κᾰ";
        genitive_masculine: "~κος", "~κοιν", "~κων";
        genitive_feminine: "~κος", "~κοιν", "~κων";
        genitive_neuter: "~κος", "~κοιν", "~κων";
        dative_masculine: "~κῐ", "~κοιν", "~ξῐ|~ξῐν";
        dative_feminine: "~κῐ", "~κοιν", "~ξῐ|~ξῐν";
        dative_neuter: "~κῐ", "~κοιν", "~ξῐ|~ξῐν";
        accusative_masculine: "~κᾰ", "~κε", "~κᾰς";
        accusative_feminine: "~κᾰ", "~κε", "~κᾰς";
        accusative_neuter: "~ξ", "~κε", "~κᾰ";
        vocative_masculine: "~ξ", "~κε", "~κες";
        vocative_feminine: "~ξ", "~κε", "~κες";
        vocative_neuter: "~ξ", "~κε", "~κᾰ";
        comparative: derived;
        superlative: derived;
        adverb: "~κως";
    ),
    // Ζευξᾱς: 9 words (παιδευσᾱς, ἀγγαρευσᾱς, στρᾰτευσᾱς, πλευσᾱς, ἀπουρᾱς, ὀρουσᾱς).
    declension!("ζευξᾱς", 9, "~υ~ᾱς";
        nominative_masculine: "~υ~ᾱς", "~υ~ᾰντε", "~υ~ᾰντες";
        nominative_feminine: "~υ~ᾱσᾰ", "~υ~ᾱσᾱ", "~υ~ᾱσαι";
        nominative_neuter: "~ῡ~ᾰν", "~υ~ᾰντε", "~υ~ᾰντᾰ";
        genitive_masculine: "~υ~ᾰντος", "~υ~ᾰντοιν", "~υ~ᾰντων";
        genitive_feminine: "~υ~ᾱσης", "~υ~ᾱσαιν", "~υ~ᾱσων";
        genitive_neuter: "~υ~ᾰντος", "~υ~ᾰντοιν", "~υ~ᾰντων";
        dative_masculine: "~υ~ᾰντῐ", "~υ~ᾰντοιν", "~υ~ᾱσῐ|~υ~ᾱσῐν";
        dative_feminine: "~υ~ᾱσῃ", "~υ~ᾱσαιν", "~υ~ᾱσαις";
        dative_neuter: "~υ~ᾰντῐ", "~υ~ᾰντοιν", "~υ~ᾱσῐ|~υ~ᾱσῐν";
        accusative_masculine: "~υ~ᾰντᾰ", "~υ~ᾰντε", "~υ~ᾰντᾰς";
        accusative_feminine: "~υ~ᾱσᾰν", "~υ~ᾱσᾱ", "~υ~ᾱσᾱς";
        accusative_neuter: "~ῡ~ᾰν", "~υ~ᾰντε", "~υ~ᾰντᾰ";
        vocative_masculine: "~υ~ᾱς", "~υ~ᾰντε", "~υ~ᾰντες";
        vocative_feminine: "~υ~ᾱσᾰ", "~υ~ᾱσᾱ", "~υ~ᾱσαι";
        vocative_neuter: "~ῡ~ᾰν", "~υ~ᾰντε", "~υ~ᾰντᾰ";
        comparative: derived;
        superlative: derived;
        adverb: "~υ~ᾰντως";
    ),
    // Παυσων: 9 words (παιδευσων, ἀγγαρευσων, φευγων, ἀγορευσων, πολῑτευσων, τευχων).
    declension!("παυσων", 9, "~υ~ων";
        nominative_masculine: "~υ~ων", "~υ~οντε", "~υ~οντες";
        nominative_feminine: "~υ~ουσᾰ", "~υ~ουσᾱ", "~υ~ουσαι";
        nominative_neuter: "~ῡ~ον", "~υ~οντε", "~υ~οντᾰ";
        genitive_masculine: "~υ~οντος", "~υ~οντοιν", "~υ~οντων";
        genitive_feminine: "~υ~ουσης", "~υ~ουσαιν", "~υ~ουσων";
        genitive_neuter: "~υ~οντος", "~υ~οντοιν", "~υ~οντων";
        dative_masculine: "~υ~οντῐ", "~υ~οντοιν", "~υ~ουσῐ|~υ~ουσῐν";
        dative_feminine: "~υ~ουσῃ", "~υ~ουσαιν", "~υ~ουσαις";
        dative_neuter: "~υ~οντῐ", "~υ~οντοιν", "~υ~ουσῐ|~υ~ουσῐν";
        accusative_masculine: "~υ~οντᾰ", "~υ~οντε", "~υ~οντᾰς";
        accusative_feminine: "~υ~ουσᾰν", "~υ~ουσᾱ", "~υ~ουσᾱς";
        accusative_neuter: "~ῡ~ον", "~υ~οντε", "~υ~οντᾰ";
        vocative_masculine: "~υ~ων", "~υ~οντε", "~υ~οντες";
        vocative_feminine: "~υ~ουσᾰ", "~υ~ουσᾱ", "~υ~ουσαι";
        vocative_neuter: "~ῡ~ον", "~υ~οντε", "~υ~οντᾰ";
        comparative: derived;
        superlative: derived;
        adverb: "~υ~οντως";
    ),
    // Ἀμπελοεις: 9 words (ἀγγελθεις, ἀγγελεις, ἀπολῠθεις, ἀορτηθεις, ἀμπελοεις, ἀφεις).
    declension!("ἀμπελοεις", 9, "ἀ~ις";
        nominative_masculine: "ᾰ̓~ις", "ᾰ̓~ντε", "ᾰ̓~ντες";
        nominative_feminine: "ᾰ̓~ῑσᾰ", "ᾰ̓~ισᾱ", "ᾰ̓~ῑσαι";
        nominative_neuter: "ᾰ̓~ν", "ᾰ̓~ντε", "ᾰ̓~ντᾰ";
        genitive_masculine: "ᾰ̓~ντος", "ᾰ̓~ντοιν", "ᾰ̓~ντων";
        genitive_feminine: "ᾰ̓~ισης", "ᾰ̓~ισαιν", "ᾰ̓~ισων";
        genitive_neuter: "ᾰ̓~ντος", "ᾰ̓~ντοιν", "ᾰ̓~ντων";
        dative_masculine: "ᾰ̓~ντῐ", "ᾰ̓~ντοιν", "ᾰ̓~ῑσῐ|ᾰ̓~ῑσῐν";
        dative_feminine: "ᾰ̓~ισῃ", "ᾰ̓~ισαιν", "ᾰ̓~ισαις";
        dative_neuter: "ᾰ̓~ντῐ", "ᾰ̓~ντοιν", "ᾰ̓~ῑσῐ|ᾰ̓~ῑσῐν";
        accusative_masculine: "ᾰ̓~ντᾰ", "ᾰ̓~ντε", "ᾰ̓~ντᾰς";
        accusative_feminine: "ᾰ̓~ῑσᾰν", "ᾰ̓~ισᾱ", "ᾰ̓~ισᾱς";
        accusative_neuter: "ᾰ̓~ν", "ᾰ̓~ντε", "ᾰ̓~ντᾰ";
        vocative_masculine: "ᾰ̓~ις", "ᾰ̓~ντε", "ᾰ̓~ντες";
        vocative_feminine: "ᾰ̓~ῑσᾰ", "ᾰ̓~ισᾱ", "ᾰ̓~ῑσαι";
        vocative_neuter: "ᾰ̓~ν", "ᾰ̓~ντε", "ᾰ̓~ντᾰ";
        comparative: derived;
        superlative: derived;
        adverb: "ᾰ̓~ντως";
    ),
    // Ἀριστος: 9 words (ἀριστος, ἀτειχιστος, ἀξιουμενος, ἀρκευθινος, ἀλευρινος, ἀμμινος). Like Ἀρχικος, except in 11 cells: genitive_masculine sg, genitive_masculine du, genitive_feminine du, genitive_neuter sg, genitive_neuter du, dative_masculine du, ….
    declension!("ἀριστος", 9, "ἀ~ι~ος", like Ἀρχικος;
        nominative_masculine: _, _, _;
        nominative_feminine: _, _, _;
        nominative_neuter: _, _, _;
        genitive_masculine: "ᾰ̓~ῐ~ου", "ᾰ̓~ῐ~οιν", _;
        genitive_feminine: _, "ᾰ̓~ῐ~αιν", _;
        genitive_neuter: "ᾰ̓~ῐ~ου", "ᾰ̓~ῐ~οιν", _;
        dative_masculine: _, "ᾰ̓~ῐ~οιν", "ᾰ̓~ῐ~οις";
        dative_feminine: _, "ᾰ̓~ῐ~αιν", "ᾰ̓~ῐ~αις";
        dative_neuter: _, "ᾰ̓~ῐ~οιν", "ᾰ̓~ῐ~οις";
        accusative_masculine: _, _, _;
        accusative_feminine: _, _, _;
        accusative_neuter: _, _, _;
        vocative_masculine: _, _, _;
        vocative_feminine: _, _, _;
        vocative_neuter: _, _, _;
        comparative: _;
        superlative: _;
        adverb: _;
    ),
    // Ἀντιπαλος: 9 words (ἀντιπαλος, ἀμφικεφαλος, ἀκλιτωτατος, ἀλλοτριωτατος, ἀγριωτατος, ἀξιωτατος).
    declension!("ἀντιπαλος", 9, "ἀ~ι~α~ος";
        nominative_masculine: "ᾰ̓~ῐ~ᾰ~ος", "ᾰ̓~ῐ~ᾰ~ω", "ᾰ̓~ῐ~ᾰ~οι";
        nominative_feminine: "ᾰ̓~ῐ~ᾰ~η", "ᾰ̓~ῐ~ᾰ~ᾱ", "ᾰ̓~ῐ~ᾰ~αι";
        nominative_neuter: "ᾰ̓~ῐ~ᾰ~ον", "ᾰ̓~ῐ~ᾰ~ω", "ᾰ̓~ῐ~ᾰ~ᾰ";
        genitive_masculine: "ᾰ̓~ῐ~ᾰ~ου", "ᾰ̓~ῐ~ᾰ~οιν", "ᾰ̓~ῐ~ᾰ~ων";
        genitive_feminine: "ᾰ̓~ῐ~ᾰ~ης", "ᾰ̓~ῐ~ᾰ~αιν", "ᾰ̓~ῐ~ᾰ~ων";
        genitive_neuter: "ᾰ̓~ῐ~ᾰ~ου", "ᾰ̓~ῐ~ᾰ~οιν", "ᾰ̓~ῐ~ᾰ~ων";
        dative_masculine: "ᾰ̓~ῐ~ᾰ~ῳ", "ᾰ̓~ῐ~ᾰ~οιν", "ᾰ̓~ῐ~ᾰ~οις";
        dative_feminine: "ᾰ̓~ῐ~ᾰ~ῃ", "ᾰ̓~ῐ~ᾰ~αιν", "ᾰ̓~ῐ~ᾰ~αις";
        dative_neuter: "ᾰ̓~ῐ~ᾰ~ῳ", "ᾰ̓~ῐ~ᾰ~οιν", "ᾰ̓~ῐ~ᾰ~οις";
        accusative_masculine: "ᾰ̓~ῐ~ᾰ~ον", "ᾰ̓~ῐ~ᾰ~ω", "ᾰ̓~ῐ~ᾰ~ους";
        accusative_feminine: "ᾰ̓~ῐ~ᾰ~ην", "ᾰ̓~ῐ~ᾰ~ᾱ", "ᾰ̓~ῐ~ᾰ~ᾱς";
        accusative_neuter: "ᾰ̓~ῐ~ᾰ~ον", "ᾰ̓~ῐ~ᾰ~ω", "ᾰ̓~ῐ~ᾰ~ᾰ";
        vocative_masculine: "ᾰ̓~ῐ~ᾰ~ε", "ᾰ̓~ῐ~ᾰ~ω", "ᾰ̓~ῐ~ᾰ~οι";
        vocative_feminine: "ᾰ̓~ῐ~ᾰ~η", "ᾰ̓~ῐ~ᾰ~ᾱ", "ᾰ̓~ῐ~ᾰ~αι";
        vocative_neuter: "ᾰ̓~ῐ~ᾰ~ον", "ᾰ̓~ῐ~ᾰ~ω", "ᾰ̓~ῐ~ᾰ~ᾰ";
        comparative: derived;
        superlative: derived;
        adverb: "ᾰ̓~ῐ~ᾰ~ᾰ";
    ),
    // Ἀκρος: 9 words (ἀκρος, ἀλβος, ἀνομοιος, ἀρειος, ἀνᾰλῠτεος, ἀσπρος). Like Ἀκροτερος, except in 3 cells: comparative, superlative, adverb.
    declension!("ἀκρος", 9, "ἀ~ος", like Ἀκροτερος;
        nominative_masculine: _, _, _;
        nominative_feminine: _, _, _;
        nominative_neuter: _, _, _;
        genitive_masculine: _, _, _;
        genitive_feminine: _, _, _;
        genitive_neuter: _, _, _;
        dative_masculine: _, _, _;
        dative_feminine: _, _, _;
        dative_neuter: _, _, _;
        accusative_masculine: _, _, _;
        accusative_feminine: _, _, _;
        accusative_neuter: _, _, _;
        vocative_masculine: _, _, _;
        vocative_feminine: _, _, _;
        vocative_neuter: _, _, _;
        comparative: "ᾰ̓~οτερος";
        superlative: "ᾰ̓~οτᾰτος";
        adverb: "ᾰ̓~ως";
    ),
    // Δῐκαιος: 8 words (ἀργῳος, χῑος, ἀζυμος, γλῐσχρος, πατρῳος, ξενϝος). Like Τελειος, except in 2 cells: comparative, superlative.
    declension!("δῐκαιος", 8, "~ος", like Τελειος;
        nominative_masculine: _, _, _;
        nominative_feminine: _, _, _;
        nominative_neuter: _, _, _;
        genitive_masculine: _, _, _;
        genitive_feminine: _, _, _;
        genitive_neuter: _, _, _;
        dative_masculine: _, _, _;
        dative_feminine: _, _, _;
        dative_neuter: _, _, _;
        accusative_masculine: _, _, _;
        accusative_feminine: _, _, _;
        accusative_neuter: _, _, _;
        vocative_masculine: _, _, _;
        vocative_feminine: _, _, _;
        vocative_neuter: _, _, _;
        comparative: "~οτερος";
        superlative: "~οτᾰτος";
        adverb: _;
    ),
    // Θειων: 8 words (θειων, ναιων, κλαιων, οἰκειων, κᾰκκειων, πολεμησειων).
    declension!("θειων", 8, "~ιων";
        nominative_masculine: "~ιων", "~ιοντε", "~ιοντες";
        nominative_feminine: "~ιουσᾰ", "~ιουσᾱ", "~ιουσαι";
        nominative_neuter: "~ῑον", "~ιοντε", "~ιοντᾰ";
        genitive_masculine: "~ιοντος", "~ιοντοιν", "~ιοντων";
        genitive_feminine: "~ιουσης", "~ιουσαιν", "~ιουσων";
        genitive_neuter: "~ιοντος", "~ιοντοιν", "~ιοντων";
        dative_masculine: "~ιοντῐ", "~ιοντοιν", "~ιουσῐ|~ιουσῐν";
        dative_feminine: "~ιουσῃ", "~ιουσαιν", "~ιουσαις";
        dative_neuter: "~ιοντῐ", "~ιοντοιν", "~ιουσῐ|~ιουσῐν";
        accusative_masculine: "~ιοντᾰ", "~ιοντε", "~ιοντᾰς";
        accusative_feminine: "~ιουσᾰν", "~ιουσᾱ", "~ιουσᾱς";
        accusative_neuter: "~ῑον", "~ιοντε", "~ιοντᾰ";
        vocative_masculine: "~ιων", "~ιοντε", "~ιοντες";
        vocative_feminine: "~ιουσᾰ", "~ιουσᾱ", "~ιουσαι";
        vocative_neuter: "~ῑον", "~ιοντε", "~ιοντᾰ";
        comparative: derived;
        superlative: derived;
        adverb: "~ιοντως";
    ),
    // Ἀξιωτερος: 8 words (ἀκλιτωτερος, ἀπιστοτερος, ἀγριωτερος, ἀξιωτερος, ἀθλιωτερος, ἀλκιμωτερος). Like Ἀριστος, except in 8 cells: nominative_feminine sg, genitive_feminine sg, dative_feminine sg, accusative_feminine sg, vocative_feminine sg, comparative, ….
    declension!("ἀξιωτερος", 8, "ἀ~ι~ος", like Ἀριστος;
        nominative_masculine: _, _, _;
        nominative_feminine: "ᾰ̓~ῐ~ᾱ", _, _;
        nominative_neuter: _, _, _;
        genitive_masculine: _, _, _;
        genitive_feminine: "ᾰ̓~ῐ~ᾱς", _, _;
        genitive_neuter: _, _, _;
        dative_masculine: _, _, _;
        dative_feminine: "ᾰ̓~ῐ~ᾳ", _, _;
        dative_neuter: _, _, _;
        accusative_masculine: _, _, _;
        accusative_feminine: "ᾰ̓~ῐ~ᾱν", _, _;
        accusative_neuter: _, _, _;
        vocative_masculine: _, _, _;
        vocative_feminine: "ᾰ̓~ῐ~ᾱ", _, _;
        vocative_neuter: _, _, _;
        comparative: derived;
        superlative: derived;
        adverb: "ᾰ̓~ῐ~ον";
    ),
    // Φᾰνερος: 7 words (στερεος, ἐτεος, πολῐος, σκολῐος, πελῐος, ἐνεος). Like Λᾰμπρος, except in 2 cells: comparative, superlative.
    declension!("φᾰνερος", 7, "~ος", like Λᾰμπρος;
        nominative_masculine: _, _, _;
        nominative_feminine: _, _, _;
        nominative_neuter: _, _, _;
        genitive_masculine: _, _, _;
        genitive_feminine: _, _, _;
        genitive_neuter: _, _, _;
        dative_masculine: _, _, _;
        dative_feminine: _, _, _;
        dative_neuter: _, _, _;
        accusative_masculine: _, _, _;
        accusative_feminine: _, _, _;
        accusative_neuter: _, _, _;
        vocative_masculine: _, _, _;
        vocative_feminine: _, _, _;
        vocative_neuter: _, _, _;
        comparative: "~ωτερος";
        superlative: "~ωτᾰτος";
        adverb: _;
    ),
    // Ἀσιτος: 7 words (ἀνικητος, ἀλιτρος, ἀφικομενος, ἀκριθος, ἀσιτος, ἀκριβεστερος).
    declension!("ἀσιτος", 7, "ἀ~ι~ος";
        nominative_masculine: "ᾰ̓~ῑ~ος", "ᾰ̓~ῑ~ω", "ᾰ̓~ῑ~οι";
        nominative_feminine: "ᾰ̓~ῑ~ος", "ᾰ̓~ῑ~ω", "ᾰ̓~ῑ~οι";
        nominative_neuter: "ᾰ̓~ῑ~ον", "ᾰ̓~ῑ~ω", "ᾰ̓~ῑ~ᾰ";
        genitive_masculine: "ᾰ̓~ῑ~ου", "ᾰ̓~ῑ~οιν", "ᾰ̓~ῑ~ων";
        genitive_feminine: "ᾰ̓~ῑ~ου", "ᾰ̓~ῑ~οιν", "ᾰ̓~ῑ~ων";
        genitive_neuter: "ᾰ̓~ῑ~ου", "ᾰ̓~ῑ~οιν", "ᾰ̓~ῑ~ων";
        dative_masculine: "ᾰ̓~ῑ~ῳ", "ᾰ̓~ῑ~οιν", "ᾰ̓~ῑ~οις";
        dative_feminine: "ᾰ̓~ῑ~ῳ", "ᾰ̓~ῑ~οιν", "ᾰ̓~ῑ~οις";
        dative_neuter: "ᾰ̓~ῑ~ῳ", "ᾰ̓~ῑ~οιν", "ᾰ̓~ῑ~οις";
        accusative_masculine: "ᾰ̓~ῑ~ον", "ᾰ̓~ῑ~ω", "ᾰ̓~ῑ~ους";
        accusative_feminine: "ᾰ̓~ῑ~ον", "ᾰ̓~ῑ~ω", "ᾰ̓~ῑ~ους";
        accusative_neuter: "ᾰ̓~ῑ~ον", "ᾰ̓~ῑ~ω", "ᾰ̓~ῑ~ᾰ";
        vocative_masculine: "ᾰ̓~ῑ~ε", "ᾰ̓~ῑ~ω", "ᾰ̓~ῑ~οι";
        vocative_feminine: "ᾰ̓~ῑ~ε", "ᾰ̓~ῑ~ω", "ᾰ̓~ῑ~οι";
        vocative_neuter: "ᾰ̓~ῑ~ον", "ᾰ̓~ῑ~ω", "ᾰ̓~ῑ~ᾰ";
        comparative: "ᾰ̓~ῑ~οτερος";
        superlative: "ᾰ̓~ῑ~οτᾰτος";
        adverb: "ᾰ̓~ῑ~ως";
    ),
    // Ἁπλοος: 7 words (ἁγνος, ἁδρος, ἁπλοος, ἁβρος, ἁδῐνος, ἁλισκομενος).
    declension!("ἁπλοος", 7, "ἁ~ος";
        nominative_masculine: "ᾰ̔~ος", "ᾰ̔~ω", "ᾰ̔~οι";
        nominative_feminine: "ᾰ̔~η", "ᾰ̔~ᾱ", "ᾰ̔~αι";
        nominative_neuter: "ᾰ̔~ον", "ᾰ̔~ω", "ᾰ̔~ᾰ";
        genitive_masculine: "ᾰ̔~ου", "ᾰ̔~οιν", "ᾰ̔~ων";
        genitive_feminine: "ᾰ̔~ης", "ᾰ̔~αιν", "ᾰ̔~ων";
        genitive_neuter: "ᾰ̔~ου", "ᾰ̔~οιν", "ᾰ̔~ων";
        dative_masculine: "ᾰ̔~ῳ", "ᾰ̔~οιν", "ᾰ̔~οις";
        dative_feminine: "ᾰ̔~ῃ", "ᾰ̔~αιν", "ᾰ̔~αις";
        dative_neuter: "ᾰ̔~ῳ", "ᾰ̔~οιν", "ᾰ̔~οις";
        accusative_masculine: "ᾰ̔~ον", "ᾰ̔~ω", "ᾰ̔~ους";
        accusative_feminine: "ᾰ̔~ην", "ᾰ̔~ᾱ", "ᾰ̔~ᾱς";
        accusative_neuter: "ᾰ̔~ον", "ᾰ̔~ω", "ᾰ̔~ᾰ";
        vocative_masculine: "ᾰ̔~ε", "ᾰ̔~ω", "ᾰ̔~οι";
        vocative_feminine: "ᾰ̔~η", "ᾰ̔~ᾱ", "ᾰ̔~αι";
        vocative_neuter: "ᾰ̔~ον", "ᾰ̔~ω", "ᾰ̔~ᾰ";
        comparative: "ᾰ̔~ωτερος";
        superlative: "ᾰ̔~ωτᾰτος";
        adverb: "ᾰ̔~ως";
    ),
    // Φῡς: 7 words (δεικνῡς, ἁπλοῡς, ἀργυροῡς, κᾰτᾰδῡς, ἐπῐμειγνῡς, ἀποδῡς).
    declension!("φῡς", 7, "~ῡς";
        nominative_masculine: "~ῡς", "~ῠντε", "~ῠντες";
        nominative_feminine: "~ῡσᾰ", "~ῡσᾱ", "~ῡσαι";
        nominative_neuter: "~ῠν", "~ῠντε", "~ῠντᾰ";
        genitive_masculine: "~ῠντος", "~ῠντοιν", "~ῠντων";
        genitive_feminine: "~ῡσης", "~ῡσαιν", "~ῡσων";
        genitive_neuter: "~ῠντος", "~ῠντοιν", "~ῠντων";
        dative_masculine: "~ῠντῐ", "~ῠντοιν", "~ῡσῐ|~ῡσῐν";
        dative_feminine: "~ῡσῃ", "~ῡσαιν", "~ῡσαις";
        dative_neuter: "~ῠντῐ", "~ῠντοιν", "~ῡσῐ|~ῡσῐν";
        accusative_masculine: "~ῠντᾰ", "~ῠντε", "~ῠντᾰς";
        accusative_feminine: "~ῡσᾰν", "~ῡσᾱ", "~ῡσᾱς";
        accusative_neuter: "~ῠν", "~ῠντε", "~ῠντᾰ";
        vocative_masculine: "~ῡς", "~ῠντε", "~ῠντες";
        vocative_feminine: "~ῡσᾰ", "~ῡσᾱ", "~ῡσαι";
        vocative_neuter: "~ῠν", "~ῠντε", "~ῠντᾰ";
        comparative: derived;
        superlative: derived;
        adverb: "~ῠντως";
    ),
    // Ἀρξαμενος: 7 words (ἀνεσκαμμενος, ἀγαθοποιησομενος, ἀγγειλαμενος, ἀρξαμενος, ἀπαγχομενος, ἀποκρῑναμενος). Like Ἀβατος, except in 14 cells: nominative_feminine sg, nominative_feminine du, nominative_feminine pl, genitive_feminine sg, genitive_feminine du, dative_feminine sg, ….
    declension!("ἀρξαμενος", 7, "ἀ~α~ος", like Ἀβατος;
        nominative_masculine: _, _, _;
        nominative_feminine: "ᾰ̓~ᾰ~η", "ᾰ̓~ᾰ~ᾱ", "ᾰ̓~ᾰ~αι";
        nominative_neuter: _, _, _;
        genitive_masculine: _, _, _;
        genitive_feminine: "ᾰ̓~ᾰ~ης", "ᾰ̓~ᾰ~αιν", _;
        genitive_neuter: _, _, _;
        dative_masculine: _, _, _;
        dative_feminine: "ᾰ̓~ᾰ~ῃ", "ᾰ̓~ᾰ~αιν", "ᾰ̓~ᾰ~αις";
        dative_neuter: _, _, _;
        accusative_masculine: _, _, _;
        accusative_feminine: "ᾰ̓~ᾰ~ην", "ᾰ̓~ᾰ~ᾱ", "ᾰ̓~ᾰ~ᾱς";
        accusative_neuter: _, _, _;
        vocative_masculine: _, _, _;
        vocative_feminine: "ᾰ̓~ᾰ~η", "ᾰ̓~ᾰ~ᾱ", "ᾰ̓~ᾰ~αι";
        vocative_neuter: _, _, _;
        comparative: _;
        superlative: _;
        adverb: _;
    ),
    // Ἰσος: 7 words (ἰσος, ἰσομετρητος, ἰσχαιμος, ἰσαιτερος, ἰδῐωτῐκος, ἰσοτερος).
    declension!("ἰσος", 7, "ἰ~ος";
        nominative_masculine: "ῐ̓~ος", "ῐ̓~ω", "ῐ̓~οι";
        nominative_feminine: "ῐ̓~ᾱ", "ῐ̓~ᾱ", "ῐ̓~αι";
        nominative_neuter: "ῐ̓~ον", "ῐ̓~ω", "ῐ̓~ᾰ";
        genitive_masculine: "ῐ̓~ου", "ῐ̓~οιν", "ῐ̓~ων";
        genitive_feminine: "ῐ̓~ᾱς", "ῐ̓~αιν", "ῐ̓~ων";
        genitive_neuter: "ῐ̓~ου", "ῐ̓~οιν", "ῐ̓~ων";
        dative_masculine: "ῐ̓~ῳ", "ῐ̓~οιν", "ῐ̓~οις";
        dative_feminine: "ῐ̓~ᾳ", "ῐ̓~αιν", "ῐ̓~αις";
        dative_neuter: "ῐ̓~ῳ", "ῐ̓~οιν", "ῐ̓~οις";
        accusative_masculine: "ῐ̓~ον", "ῐ̓~ω", "ῐ̓~ους";
        accusative_feminine: "ῐ̓~ᾱν", "ῐ̓~ᾱ", "ῐ̓~ᾱς";
        accusative_neuter: "ῐ̓~ον", "ῐ̓~ω", "ῐ̓~ᾰ";
        vocative_masculine: "ῐ̓~ε", "ῐ̓~ω", "ῐ̓~οι";
        vocative_feminine: "ῐ̓~ᾱ", "ῐ̓~ᾱ", "ῐ̓~αι";
        vocative_neuter: "ῐ̓~ον", "ῐ̓~ω", "ῐ̓~ᾰ";
        comparative: derived;
        superlative: derived;
        adverb: "ῐ̓~ον";
    ),
    // Ἀκμαζων: 6 words (ἀγαθοεργεων, ἀγαθοποιων, ἀγαθοποιησων, ἀκμαζων, ἀναγγελλων, ἀνακρῐνων).
    declension!("ἀκμαζων", 6, "ἀ~α~ων";
        nominative_masculine: "ᾰ̓~ᾰ~ων", "ᾰ̓~ᾰ~οντε", "ᾰ̓~ᾰ~οντες";
        nominative_feminine: "ᾰ̓~ᾰ~ουσᾰ", "ᾰ̓~ᾰ~ουσᾱ", "ᾰ̓~ᾰ~ουσαι";
        nominative_neuter: "ᾰ̓~ᾰ~ον", "ᾰ̓~ᾰ~οντε", "ᾰ̓~ᾰ~οντᾰ";
        genitive_masculine: "ᾰ̓~ᾰ~οντος", "ᾰ̓~ᾰ~οντοιν", "ᾰ̓~ᾰ~οντων";
        genitive_feminine: "ᾰ̓~ᾰ~ουσης", "ᾰ̓~ᾰ~ουσαιν", "ᾰ̓~ᾰ~ουσων";
        genitive_neuter: "ᾰ̓~ᾰ~οντος", "ᾰ̓~ᾰ~οντοιν", "ᾰ̓~ᾰ~οντων";
        dative_masculine: "ᾰ̓~ᾰ~οντῐ", "ᾰ̓~ᾰ~οντοιν", "ᾰ̓~ᾰ~ουσῐ|ᾰ̓~ᾰ~ουσῐν";
        dative_feminine: "ᾰ̓~ᾰ~ουσῃ", "ᾰ̓~ᾰ~ουσαιν", "ᾰ̓~ᾰ~ουσαις";
        dative_neuter: "ᾰ̓~ᾰ~οντῐ", "ᾰ̓~ᾰ~οντοιν", "ᾰ̓~ᾰ~ουσῐ|ᾰ̓~ᾰ~ουσῐν";
        accusative_masculine: "ᾰ̓~ᾰ~οντᾰ", "ᾰ̓~ᾰ~οντε", "ᾰ̓~ᾰ~οντᾰς";
        accusative_feminine: "ᾰ̓~ᾰ~ουσᾰν", "ᾰ̓~ᾰ~ουσᾱ", "ᾰ̓~ᾰ~ουσᾱς";
        accusative_neuter: "ᾰ̓~ᾰ~ον", "ᾰ̓~ᾰ~οντε", "ᾰ̓~ᾰ~οντᾰ";
        vocative_masculine: "ᾰ̓~ᾰ~ων", "ᾰ̓~ᾰ~οντε", "ᾰ̓~ᾰ~οντες";
        vocative_feminine: "ᾰ̓~ᾰ~ουσᾰ", "ᾰ̓~ᾰ~ουσᾱ", "ᾰ̓~ᾰ~ουσαι";
        vocative_neuter: "ᾰ̓~ᾰ~ον", "ᾰ̓~ᾰ~οντε", "ᾰ̓~ᾰ~οντᾰ";
        comparative: derived;
        superlative: derived;
        adverb: "ᾰ̓~ᾰ~οντως";
    ),
    // Ὑπεγγῠος: 6 words (ὑψοροφος, ὑποστεγος, ὑποχειρῐος, ὑποδρομος, ὑποτροφος, ὑπεγγῠος). Like Ὑπαιθρος, except in 2 cells: comparative, superlative.
    declension!("ὑπεγγῠος", 6, "ὑ~ος", like Ὑπαιθρος;
        nominative_masculine: _, _, _;
        nominative_feminine: _, _, _;
        nominative_neuter: _, _, _;
        genitive_masculine: _, _, _;
        genitive_feminine: _, _, _;
        genitive_neuter: _, _, _;
        dative_masculine: _, _, _;
        dative_feminine: _, _, _;
        dative_neuter: _, _, _;
        accusative_masculine: _, _, _;
        accusative_feminine: _, _, _;
        accusative_neuter: _, _, _;
        vocative_masculine: _, _, _;
        vocative_feminine: _, _, _;
        vocative_neuter: _, _, _;
        comparative: "ῠ̔~ωτερος";
        superlative: "ῠ̔~ωτᾰτος";
        adverb: _;
    ),
    // Κολοβος: 5 words (κολοβος, ἀκῐρος, πᾰρδᾰκος, ἐκκοπρωτικος, ἐτος). Like Μαυρος, except in 2 cells: comparative, superlative.
    declension!("κολοβος", 5, "~ος", like Μαυρος;
        nominative_masculine: _, _, _;
        nominative_feminine: _, _, _;
        nominative_neuter: _, _, _;
        genitive_masculine: _, _, _;
        genitive_feminine: _, _, _;
        genitive_neuter: _, _, _;
        dative_masculine: _, _, _;
        dative_feminine: _, _, _;
        dative_neuter: _, _, _;
        accusative_masculine: _, _, _;
        accusative_feminine: _, _, _;
        accusative_neuter: _, _, _;
        vocative_masculine: _, _, _;
        vocative_feminine: _, _, _;
        vocative_neuter: _, _, _;
        comparative: "~ωτερος";
        superlative: "~ωτᾰτος";
        adverb: _;
    ),
    // Αἰθοψ: 5 words (οἰνοψ, μῠωψ, αἰθοψ, νωροψ, ἠνοψ).
    declension!("αἰθοψ", 5, "~ψ";
        nominative_masculine: "~ψ", "~πε", "~πες";
        nominative_feminine: "~ψ", "~πε", "~πες";
        nominative_neuter: "~ψ", "~πε", "~πᾰ";
        genitive_masculine: "~πος", "~ποιν", "~πων";
        genitive_feminine: "~πος", "~ποιν", "~πων";
        genitive_neuter: "~πος", "~ποιν", "~πων";
        dative_masculine: "~πῐ", "~ποιν", "~ψῐ|~ψῐν";
        dative_feminine: "~πῐ", "~ποιν", "~ψῐ|~ψῐν";
        dative_neuter: "~πῐ", "~ποιν", "~ψῐ|~ψῐν";
        accusative_masculine: "~πᾰ", "~πε", "~πᾰς";
        accusative_feminine: "~πᾰ", "~πε", "~πᾰς";
        accusative_neuter: "~ψ", "~πε", "~πᾰ";
        vocative_masculine: "~ψ", "~πε", "~πες";
        vocative_feminine: "~ψ", "~πε", "~πες";
        vocative_neuter: "~ψ", "~πε", "~πᾰ";
        comparative: derived;
        superlative: derived;
        adverb: "~πως";
    ),
    // Δους: 5 words (δους, βῐους, δῐδους, γνους, γενομενους). Like Ὑληεις, except in 19 cells: nominative_masculine sg, nominative_feminine sg, nominative_feminine du, nominative_feminine pl, genitive_feminine sg, genitive_feminine du, ….
    declension!("δους", 5, "~υς", like Ὑληεις;
        nominative_masculine: "~υς", _, _;
        nominative_feminine: "~ῡσᾰ", "~υσᾱ", "~ῡσαι";
        nominative_neuter: _, _, _;
        genitive_masculine: _, _, _;
        genitive_feminine: "~υσης", "~υσαιν", "~υσων";
        genitive_neuter: _, _, _;
        dative_masculine: _, _, "~ῡσῐ|~ῡσῐν";
        dative_feminine: "~υσῃ", "~υσαιν", "~υσαις";
        dative_neuter: _, _, "~ῡσῐ|~ῡσῐν";
        accusative_masculine: _, _, _;
        accusative_feminine: "~ῡσᾰν", "~υσᾱ", "~υσᾱς";
        accusative_neuter: _, _, _;
        vocative_masculine: "~υς", _, _;
        vocative_feminine: "~ῡσᾰ", "~υσᾱ", "~ῡσαι";
        vocative_neuter: _, _, _;
        comparative: _;
        superlative: _;
        adverb: _;
    ),
    // Ἀντιπαις: 5 words (ἀργινοεις, ἀντιπαις, ἀξιωθεις, ἀποτειχισθεις, ἀποκριθεις).
    declension!("ἀντιπαις", 5, "ἀ~ι~ις";
        nominative_masculine: "ᾰ̓~ῐ~ις", "ᾰ̓~ῐ~ντε", "ᾰ̓~ῐ~ντες";
        nominative_feminine: "ᾰ̓~ῐ~ῑσᾰ", "ᾰ̓~ῐ~ισᾱ", "ᾰ̓~ῐ~ῑσαι";
        nominative_neuter: "ᾰ̓~ῐ~ν", "ᾰ̓~ῐ~ντε", "ᾰ̓~ῐ~ντᾰ";
        genitive_masculine: "ᾰ̓~ῐ~ντος", "ᾰ̓~ῐ~ντοιν", "ᾰ̓~ῐ~ντων";
        genitive_feminine: "ᾰ̓~ῐ~ισης", "ᾰ̓~ῐ~ισαιν", "ᾰ̓~ῐ~ισων";
        genitive_neuter: "ᾰ̓~ῐ~ντος", "ᾰ̓~ῐ~ντοιν", "ᾰ̓~ῐ~ντων";
        dative_masculine: "ᾰ̓~ῐ~ντῐ", "ᾰ̓~ῐ~ντοιν", "ᾰ̓~ῐ~ῑσῐ|ᾰ̓~ῐ~ῑσῐν";
        dative_feminine: "ᾰ̓~ῐ~ισῃ", "ᾰ̓~ῐ~ισαιν", "ᾰ̓~ῐ~ισαις";
        dative_neuter: "ᾰ̓~ῐ~ντῐ", "ᾰ̓~ῐ~ντοιν", "ᾰ̓~ῐ~ῑσῐ|ᾰ̓~ῐ~ῑσῐν";
        accusative_masculine: "ᾰ̓~ῐ~ντᾰ", "ᾰ̓~ῐ~ντε", "ᾰ̓~ῐ~ντᾰς";
        accusative_feminine: "ᾰ̓~ῐ~ῑσᾰν", "ᾰ̓~ῐ~ισᾱ", "ᾰ̓~ῐ~ισᾱς";
        accusative_neuter: "ᾰ̓~ῐ~ν", "ᾰ̓~ῐ~ντε", "ᾰ̓~ῐ~ντᾰ";
        vocative_masculine: "ᾰ̓~ῐ~ις", "ᾰ̓~ῐ~ντε", "ᾰ̓~ῐ~ντες";
        vocative_feminine: "ᾰ̓~ῐ~ῑσᾰ", "ᾰ̓~ῐ~ισᾱ", "ᾰ̓~ῐ~ῑσαι";
        vocative_neuter: "ᾰ̓~ῐ~ν", "ᾰ̓~ῐ~ντε", "ᾰ̓~ῐ~ντᾰ";
        comparative: derived;
        superlative: derived;
        adverb: "ᾰ̓~ῐ~ντως";
    ),
    // Ἀγωγος: 5 words (ἀγωγος, ἀλαος, ἀφνειος, ἀμενηνος, ἀμβλῠωπος). Like Ἀσημος, except in 12 cells: genitive_masculine sg, genitive_masculine du, genitive_feminine sg, genitive_feminine du, genitive_neuter sg, genitive_neuter du, ….
    declension!("ἀγωγος", 5, "ἀ~ος", like Ἀσημος;
        nominative_masculine: _, _, _;
        nominative_feminine: _, _, _;
        nominative_neuter: _, _, _;
        genitive_masculine: "ᾰ̓~οῡ", "ᾰ̓~οῑν", _;
        genitive_feminine: "ᾰ̓~οῡ", "ᾰ̓~οῑν", _;
        genitive_neuter: "ᾰ̓~οῡ", "ᾰ̓~οῑν", _;
        dative_masculine: _, "ᾰ̓~οῑν", "ᾰ̓~οῑς";
        dative_feminine: _, "ᾰ̓~οῑν", "ᾰ̓~οῑς";
        dative_neuter: _, "ᾰ̓~οῑν", "ᾰ̓~οῑς";
        accusative_masculine: _, _, _;
        accusative_feminine: _, _, _;
        accusative_neuter: _, _, _;
        vocative_masculine: _, _, _;
        vocative_feminine: _, _, _;
        vocative_neuter: _, _, _;
        comparative: _;
        superlative: _;
        adverb: _;
    ),
    // Ἀνομωτᾰτος: 5 words (ἀληθεστᾰτος, ἀφᾰνεστᾰτος, ἀνομωτᾰτος, ἀοκνοτᾰτος, ἀχρειοτᾰτος). Like Ἀλλος, except in 3 cells: comparative, superlative, adverb.
    declension!("ἀνομωτᾰτος", 5, "ἀ~ος", like Ἀλλος;
        nominative_masculine: _, _, _;
        nominative_feminine: _, _, _;
        nominative_neuter: _, _, _;
        genitive_masculine: _, _, _;
        genitive_feminine: _, _, _;
        genitive_neuter: _, _, _;
        dative_masculine: _, _, _;
        dative_feminine: _, _, _;
        dative_neuter: _, _, _;
        accusative_masculine: _, _, _;
        accusative_feminine: _, _, _;
        accusative_neuter: _, _, _;
        vocative_masculine: _, _, _;
        vocative_feminine: _, _, _;
        vocative_neuter: _, _, _;
        comparative: derived;
        superlative: derived;
        adverb: "ᾰ̓~ᾰ";
    ),
    // Ὑπατος: 5 words (ὑπατος, ὑπογεγραμμενος, ὑπαρχομενος, ὑψηλοτατος, ὑπανδρος).
    declension!("ὑπατος", 5, "ὑ~α~ος";
        nominative_masculine: "ῠ̔~ᾰ~ος", "ῠ̔~ᾰ~ω", "ῠ̔~ᾰ~οι";
        nominative_feminine: "ῠ̔~ᾰ~η", "ῠ̔~ᾰ~ᾱ", "ῠ̔~ᾰ~αι";
        nominative_neuter: "ῠ̔~ᾰ~ον", "ῠ̔~ᾰ~ω", "ῠ̔~ᾰ~ᾰ";
        genitive_masculine: "ῠ̔~ᾰ~ου", "ῠ̔~ᾰ~οιν", "ῠ̔~ᾰ~ων";
        genitive_feminine: "ῠ̔~ᾰ~ης", "ῠ̔~ᾰ~αιν", "ῠ̔~ᾰ~ων";
        genitive_neuter: "ῠ̔~ᾰ~ου", "ῠ̔~ᾰ~οιν", "ῠ̔~ᾰ~ων";
        dative_masculine: "ῠ̔~ᾰ~ῳ", "ῠ̔~ᾰ~οιν", "ῠ̔~ᾰ~οις";
        dative_feminine: "ῠ̔~ᾰ~ῃ", "ῠ̔~ᾰ~αιν", "ῠ̔~ᾰ~αις";
        dative_neuter: "ῠ̔~ᾰ~ῳ", "ῠ̔~ᾰ~οιν", "ῠ̔~ᾰ~οις";
        accusative_masculine: "ῠ̔~ᾰ~ον", "ῠ̔~ᾰ~ω", "ῠ̔~ᾰ~ους";
        accusative_feminine: "ῠ̔~ᾰ~ην", "ῠ̔~ᾰ~ᾱ", "ῠ̔~ᾰ~ᾱς";
        accusative_neuter: "ῠ̔~ᾰ~ον", "ῠ̔~ᾰ~ω", "ῠ̔~ᾰ~ᾰ";
        vocative_masculine: "ῠ̔~ᾰ~ε", "ῠ̔~ᾰ~ω", "ῠ̔~ᾰ~οι";
        vocative_feminine: "ῠ̔~ᾰ~η", "ῠ̔~ᾰ~ᾱ", "ῠ̔~ᾰ~αι";
        vocative_neuter: "ῠ̔~ᾰ~ον", "ῠ̔~ᾰ~ω", "ῠ̔~ᾰ~ᾰ";
        comparative: "ῠ̔~ᾰ~ωτερος";
        superlative: "ῠ̔~ᾰ~ωτᾰτος";
        adverb: "ῠ̔~ᾰ~ως";
    ),
    // Ἀφροδῑσῐος: 4 words (ἀσκῐος, ἀθροος, ἀφροδῑσῐος, ἀνθεκτεος). Like Ἀκρος, except in 2 cells: comparative, superlative.
    declension!("ἀφροδῑσῐος", 4, "ἀ~ος", like Ἀκρος;
        nominative_masculine: _, _, _;
        nominative_feminine: _, _, _;
        nominative_neuter: _, _, _;
        genitive_masculine: _, _, _;
        genitive_feminine: _, _, _;
        genitive_neuter: _, _, _;
        dative_masculine: _, _, _;
        dative_feminine: _, _, _;
        dative_neuter: _, _, _;
        accusative_masculine: _, _, _;
        accusative_feminine: _, _, _;
        accusative_neuter: _, _, _;
        vocative_masculine: _, _, _;
        vocative_feminine: _, _, _;
        vocative_neuter: _, _, _;
        comparative: "ᾰ̓~ωτερος";
        superlative: "ᾰ̓~ωτᾰτος";
        adverb: _;
    ),
    // Μελᾱς: 4 words (μελᾱς, λευκομελᾱς, τᾰλᾱς, χλωρομελᾱς).
    declension!("μελᾱς", 4, "~ᾱς";
        nominative_masculine: "~ᾱς", "~ᾰνε", "~ᾰνες";
        nominative_feminine: "~αινᾰ", "~αινᾱ", "~αιναι";
        nominative_neuter: "~ᾰν", "~ᾰνε", "~ᾰνᾰ";
        genitive_masculine: "~ᾰνος", "~ᾰνοιν", "~ᾰνων";
        genitive_feminine: "~αινης", "~αιναιν", "~αινων";
        genitive_neuter: "~ᾰνος", "~ᾰνοιν", "~ᾰνων";
        dative_masculine: "~ᾰνῐ", "~ᾰνοιν", "~ᾰσῐ|~ᾰσῐν";
        dative_feminine: "~αινῃ", "~αιναιν", "~αιναις";
        dative_neuter: "~ᾰνῐ", "~ᾰνοιν", "~ᾰσῐ|~ᾰσῐν";
        accusative_masculine: "~ᾰνᾰ", "~ᾰνε", "~ᾰνᾰς";
        accusative_feminine: "~αινᾰν", "~αινᾱ", "~αινᾱς";
        accusative_neuter: "~ᾰν", "~ᾰνε", "~ᾰνᾰ";
        vocative_masculine: "~ᾰν", "~ᾰνε", "~ᾰνες";
        vocative_feminine: "~αινᾰ", "~αινᾱ", "~αιναι";
        vocative_neuter: "~ᾰν", "~ᾰνε", "~ᾰνᾰ";
        comparative: "~ᾰντερος";
        superlative: "~ᾰντᾰτος";
        adverb: "~ᾰνως";
    ),
    // Ἀγαπητος: 4 words (ἀγαθος, ἀγαπητος, ἀλᾰπαδνος, ἀσπαστος). Like Ἀρξαμενος, except in 13 cells: genitive_masculine sg, genitive_masculine du, genitive_feminine du, genitive_neuter sg, genitive_neuter du, dative_masculine du, ….
    declension!("ἀγαπητος", 4, "ἀ~α~ος", like Ἀρξαμενος;
        nominative_masculine: _, _, _;
        nominative_feminine: _, _, _;
        nominative_neuter: _, _, _;
        genitive_masculine: "ᾰ̓~ᾰ~οῡ", "ᾰ̓~ᾰ~οῑν", _;
        genitive_feminine: _, "ᾰ̓~ᾰ~αῑν", _;
        genitive_neuter: "ᾰ̓~ᾰ~οῡ", "ᾰ̓~ᾰ~οῑν", _;
        dative_masculine: _, "ᾰ̓~ᾰ~οῑν", "ᾰ̓~ᾰ~οῑς";
        dative_feminine: _, "ᾰ̓~ᾰ~αῑν", "ᾰ̓~ᾰ~αῑς";
        dative_neuter: _, "ᾰ̓~ᾰ~οῑν", "ᾰ̓~ᾰ~οῑς";
        accusative_masculine: _, _, _;
        accusative_feminine: _, _, _;
        accusative_neuter: _, _, _;
        vocative_masculine: _, _, _;
        vocative_feminine: _, _, _;
        vocative_neuter: _, _, _;
        comparative: "ᾰ̓~ᾰ~οτερος";
        superlative: "ᾰ̓~ᾰ~οτᾰτος";
        adverb: _;
    ),
    // Ἀρτιος: 4 words (ἀγριος, ἀξιος, ἀρτιος, ἀλλοτριος).
    declension!("ἀρτιος", 4, "ἀ~ιος";
        nominative_masculine: "ᾰ̓~ῐος", "ᾰ̓~ῐω", "ᾰ̓~ῐοι";
        nominative_feminine: "ᾰ̓~ῐᾱ", "ᾰ̓~ῐᾱ", "ᾰ̓~ῐαι";
        nominative_neuter: "ᾰ̓~ῐον", "ᾰ̓~ῐω", "ᾰ̓~ῐᾰ";
        genitive_masculine: "ᾰ̓~ῐου", "ᾰ̓~ῐοιν", "ᾰ̓~ῐων";
        genitive_feminine: "ᾰ̓~ῐᾱς", "ᾰ̓~ῐαιν", "ᾰ̓~ῐων";
        genitive_neuter: "ᾰ̓~ῐου", "ᾰ̓~ῐοιν", "ᾰ̓~ῐων";
        dative_masculine: "ᾰ̓~ῐῳ", "ᾰ̓~ῐοιν", "ᾰ̓~ῐοις";
        dative_feminine: "ᾰ̓~ῐᾳ", "ᾰ̓~ῐαιν", "ᾰ̓~ῐαις";
        dative_neuter: "ᾰ̓~ῐῳ", "ᾰ̓~ῐοιν", "ᾰ̓~ῐοις";
        accusative_masculine: "ᾰ̓~ῐον", "ᾰ̓~ῐω", "ᾰ̓~ῐους";
        accusative_feminine: "ᾰ̓~ῐᾱν", "ᾰ̓~ῐᾱ", "ᾰ̓~ῐᾱς";
        accusative_neuter: "ᾰ̓~ῐον", "ᾰ̓~ῐω", "ᾰ̓~ῐᾰ";
        vocative_masculine: "ᾰ̓~ῐε", "ᾰ̓~ῐω", "ᾰ̓~ῐοι";
        vocative_feminine: "ᾰ̓~ῐᾱ", "ᾰ̓~ῐᾱ", "ᾰ̓~ῐαι";
        vocative_neuter: "ᾰ̓~ῐον", "ᾰ̓~ῐω", "ᾰ̓~ῐᾰ";
        comparative: "ᾰ̓~ῐωτερος";
        superlative: "ᾰ̓~ῐωτᾰτος";
        adverb: "ᾰ̓~ῐως";
    ),
    // Ἀμαυρος: 4 words (ἀμαυρος, ἀραιος, ἀγαυος, ἀνθηρος). Like Ἀποδεικτῐκος, except in 7 cells: nominative_feminine sg, genitive_feminine sg, dative_feminine sg, accusative_feminine sg, vocative_feminine sg, comparative, ….
    declension!("ἀμαυρος", 4, "ἀ~ος", like Ἀποδεικτῐκος;
        nominative_masculine: _, _, _;
        nominative_feminine: "ᾰ̓~ᾱ", _, _;
        nominative_neuter: _, _, _;
        genitive_masculine: _, _, _;
        genitive_feminine: "ᾰ̓~ᾱς", _, _;
        genitive_neuter: _, _, _;
        dative_masculine: _, _, _;
        dative_feminine: "ᾰ̓~ᾳ", _, _;
        dative_neuter: _, _, _;
        accusative_masculine: _, _, _;
        accusative_feminine: "ᾰ̓~ᾱν", _, _;
        accusative_neuter: _, _, _;
        vocative_masculine: _, _, _;
        vocative_feminine: "ᾰ̓~ᾱ", _, _;
        vocative_neuter: _, _, _;
        comparative: "ᾰ̓~οτερος";
        superlative: "ᾰ̓~οτᾰτος";
        adverb: _;
    ),
    // Χειρων: 4 words (μειζων, χειρων, κρειττων, τρῐσκᾰκοδαιμων).
    declension!("χειρων", 4, "~ι~ων";
        nominative_masculine: "~ι~ων", "~ι~ονε", "~ι~ονες|~ι~ους";
        nominative_feminine: "~ι~ων", "~ι~ονε", "~ι~ονες|~ι~ους";
        nominative_neuter: "~ῑ~ον", "~ι~ονε", "~ι~ονᾰ|~ι~ω";
        genitive_masculine: "~ι~ονος", "~ι~ονοιν", "~ι~ονων";
        genitive_feminine: "~ι~ονος", "~ι~ονοιν", "~ι~ονων";
        genitive_neuter: "~ι~ονος", "~ι~ονοιν", "~ι~ονων";
        dative_masculine: "~ι~ονῐ", "~ι~ονοιν", "~ι~οσῐ|~ι~οσῐν";
        dative_feminine: "~ι~ονῐ", "~ι~ονοιν", "~ι~οσῐ|~ι~οσῐν";
        dative_neuter: "~ι~ονῐ", "~ι~ονοιν", "~ι~οσῐ|~ι~οσῐν";
        accusative_masculine: "~ι~ονᾰ|~ι~ω", "~ι~ονε", "~ι~ονᾰς|~ι~ους";
        accusative_feminine: "~ι~ονᾰ|~ι~ω", "~ι~ονε", "~ι~ονᾰς|~ι~ους";
        accusative_neuter: "~ῑ~ον", "~ι~ονε", "~ι~ονᾰ|~ι~ω";
        vocative_masculine: "~ῑ~ον", "~ι~ονε", "~ι~ονες|~ι~ους";
        vocative_feminine: "~ῑ~ον", "~ι~ονε", "~ι~ονες|~ι~ους";
        vocative_neuter: "~ῑ~ον", "~ι~ονε", "~ι~ονᾰ|~ι~ω";
        comparative: derived;
        superlative: derived;
        adverb: "~ῑ~ον";
    ),
    // Ἀγωνιστικος: 4 words (ἀντιθετικος, ἀντιλογητικος, ἀξιοπιστος, ἀγωνιστικος).
    declension!("ἀγωνιστικος", 4, "ἀ~ι~ι~ος";
        nominative_masculine: "ᾰ̓~ῐ~ῐ~ος", "ᾰ̓~ῐ~ῐ~ω", "ᾰ̓~ῐ~ῐ~οι";
        nominative_feminine: "ᾰ̓~ῐ~ῐ~η", "ᾰ̓~ῐ~ῐ~ᾱ", "ᾰ̓~ῐ~ῐ~αι";
        nominative_neuter: "ᾰ̓~ῐ~ῐ~ον", "ᾰ̓~ῐ~ῐ~ω", "ᾰ̓~ῐ~ῐ~ᾰ";
        genitive_masculine: "ᾰ̓~ῐ~ῐ~οῡ", "ᾰ̓~ῐ~ῐ~οῑν", "ᾰ̓~ῐ~ῐ~ων";
        genitive_feminine: "ᾰ̓~ῐ~ῐ~ης", "ᾰ̓~ῐ~ῐ~αῑν", "ᾰ̓~ῐ~ῐ~ων";
        genitive_neuter: "ᾰ̓~ῐ~ῐ~οῡ", "ᾰ̓~ῐ~ῐ~οῑν", "ᾰ̓~ῐ~ῐ~ων";
        dative_masculine: "ᾰ̓~ῐ~ῐ~ῳ", "ᾰ̓~ῐ~ῐ~οῑν", "ᾰ̓~ῐ~ῐ~οῑς";
        dative_feminine: "ᾰ̓~ῐ~ῐ~ῃ", "ᾰ̓~ῐ~ῐ~αῑν", "ᾰ̓~ῐ~ῐ~αῑς";
        dative_neuter: "ᾰ̓~ῐ~ῐ~ῳ", "ᾰ̓~ῐ~ῐ~οῑν", "ᾰ̓~ῐ~ῐ~οῑς";
        accusative_masculine: "ᾰ̓~ῐ~ῐ~ον", "ᾰ̓~ῐ~ῐ~ω", "ᾰ̓~ῐ~ῐ~ους";
        accusative_feminine: "ᾰ̓~ῐ~ῐ~ην", "ᾰ̓~ῐ~ῐ~ᾱ", "ᾰ̓~ῐ~ῐ~ᾱς";
        accusative_neuter: "ᾰ̓~ῐ~ῐ~ον", "ᾰ̓~ῐ~ῐ~ω", "ᾰ̓~ῐ~ῐ~ᾰ";
        vocative_masculine: "ᾰ̓~ῐ~ῐ~ε", "ᾰ̓~ῐ~ῐ~ω", "ᾰ̓~ῐ~ῐ~οι";
        vocative_feminine: "ᾰ̓~ῐ~ῐ~η", "ᾰ̓~ῐ~ῐ~ᾱ", "ᾰ̓~ῐ~ῐ~αι";
        vocative_neuter: "ᾰ̓~ῐ~ῐ~ον", "ᾰ̓~ῐ~ῐ~ω", "ᾰ̓~ῐ~ῐ~ᾰ";
        comparative: "ᾰ̓~ῐ~ῐ~ωτερος";
        superlative: "ᾰ̓~ῐ~ῐ~ωτᾰτος";
        adverb: "ᾰ̓~ῐ~ῐ~ως";
    ),
    // Δῐπλοῡς: 4 words (δῐπλοῡς, χρῡσοῡς, χᾰλκοῡς, πορφῠροῡς). Like Μᾰλᾰκος, except in 22 cells: nominative_masculine sg, nominative_masculine du, nominative_masculine pl, nominative_feminine pl, nominative_neuter sg, nominative_neuter du, ….
    declension!("δῐπλοῡς", 4, "~οῡς", like Μᾰλᾰκος;
        nominative_masculine: "~οῡς", "~ω|~ω", "~οῑ";
        nominative_feminine: _, _, "~αῑ";
        nominative_neuter: "~οῡν", "~ω|~ω", "~ᾱ";
        genitive_masculine: _, _, _;
        genitive_feminine: _, _, _;
        genitive_neuter: _, _, _;
        dative_masculine: _, _, _;
        dative_feminine: _, _, _;
        dative_neuter: _, _, _;
        accusative_masculine: "~οῡν", "~ω|~ω", "~οῡς";
        accusative_feminine: _, _, _;
        accusative_neuter: "~οῡν", "~ω|~ω", "~ᾱ";
        vocative_masculine: "~οῡ", "~ω|~ω", "~οῑ";
        vocative_feminine: _, _, "~αῑ";
        vocative_neuter: "~οῡν", "~ω|~ω", "~ᾱ";
        comparative: "~εωτερος";
        superlative: "~εωτᾰτος";
        adverb: _;
    ),
    // Ἀγνοων: 4 words (ἀγγελων, ἀγνοων, ἀντερων, ἀδῐκων).
    declension!("ἀγνοων", 4, "ἀ~ων";
        nominative_masculine: "ᾰ̓~ων", "ᾰ̓~οῡντε", "ᾰ̓~οῡντες";
        nominative_feminine: "ᾰ̓~οῡσᾰ", "ᾰ̓~ουσᾱ", "ᾰ̓~οῡσαι";
        nominative_neuter: "ᾰ̓~οῡν", "ᾰ̓~οῡντε", "ᾰ̓~οῡντᾰ";
        genitive_masculine: "ᾰ̓~οῡντος", "ᾰ̓~ουντοιν", "ᾰ̓~ουντων";
        genitive_feminine: "ᾰ̓~ουσης", "ᾰ̓~ουσαιν", "ᾰ̓~ουσων";
        genitive_neuter: "ᾰ̓~οῡντος", "ᾰ̓~ουντοιν", "ᾰ̓~ουντων";
        dative_masculine: "ᾰ̓~οῡντῐ", "ᾰ̓~ουντοιν", "ᾰ̓~οῡσῐ|ᾰ̓~οῡσῐν";
        dative_feminine: "ᾰ̓~ουσῃ", "ᾰ̓~ουσαιν", "ᾰ̓~ουσαις";
        dative_neuter: "ᾰ̓~οῡντῐ", "ᾰ̓~ουντοιν", "ᾰ̓~οῡσῐ|ᾰ̓~οῡσῐν";
        accusative_masculine: "ᾰ̓~οῡντᾰ", "ᾰ̓~οῡντε", "ᾰ̓~οῡντᾰς";
        accusative_feminine: "ᾰ̓~οῡσᾰν", "ᾰ̓~ουσᾱ", "ᾰ̓~ουσᾱς";
        accusative_neuter: "ᾰ̓~οῡν", "ᾰ̓~οῡντε", "ᾰ̓~οῡντᾰ";
        vocative_masculine: "ᾰ̓~ων", "ᾰ̓~οῡντε", "ᾰ̓~οῡντες";
        vocative_feminine: "ᾰ̓~οῡσᾰ", "ᾰ̓~ουσᾱ", "ᾰ̓~οῡσαι";
        vocative_neuter: "ᾰ̓~οῡν", "ᾰ̓~οῡντε", "ᾰ̓~οῡντᾰ";
        comparative: derived;
        superlative: derived;
        adverb: "ᾰ̓~ουντως";
    ),
    // Δρυϊνος: 4 words (νεκυϊκος, οἰσυϊνος, δρυϊνος, συϊνος).
    declension!("δρυϊνος", 4, "~υϊ~ος";
        nominative_masculine: "~ῠῐ̈~ος", "~ῠῐ̈~ω", "~ῠῐ̈~οι";
        nominative_feminine: "~ῠῐ̈~η", "~ῠῐ̈~ᾱ", "~ῠῐ̈~αι";
        nominative_neuter: "~ῠῐ̈~ον", "~ῠῐ̈~ω", "~ῠῐ̈~ᾰ";
        genitive_masculine: "~ῠῐ̈~ου", "~ῠῐ̈~οιν", "~ῠῐ̈~ων";
        genitive_feminine: "~ῠῐ̈~ης", "~ῠῐ̈~αιν", "~ῠῐ̈~ων";
        genitive_neuter: "~ῠῐ̈~ου", "~ῠῐ̈~οιν", "~ῠῐ̈~ων";
        dative_masculine: "~ῠῐ̈~ῳ", "~ῠῐ̈~οιν", "~ῠῐ̈~οις";
        dative_feminine: "~ῠῐ̈~ῃ", "~ῠῐ̈~αιν", "~ῠῐ̈~αις";
        dative_neuter: "~ῠῐ̈~ῳ", "~ῠῐ̈~οιν", "~ῠῐ̈~οις";
        accusative_masculine: "~ῠῐ̈~ον", "~ῠῐ̈~ω", "~ῠῐ̈~ους";
        accusative_feminine: "~ῠῐ̈~ην", "~ῠῐ̈~ᾱ", "~ῠῐ̈~ᾱς";
        accusative_neuter: "~ῠῐ̈~ον", "~ῠῐ̈~ω", "~ῠῐ̈~ᾰ";
        vocative_masculine: "~ῠῐ̈~ε", "~ῠῐ̈~ω", "~ῠῐ̈~οι";
        vocative_feminine: "~ῠῐ̈~η", "~ῠῐ̈~ᾱ", "~ῠῐ̈~αι";
        vocative_neuter: "~ῠῐ̈~ον", "~ῠῐ̈~ω", "~ῠῐ̈~ᾰ";
        comparative: "~ῠῐ̈~ωτερος";
        superlative: "~ῠῐ̈~ωτᾰτος";
        adverb: "~ῠῐ̈~ως";
    ),
    // Πεπτεως: 4 words (πεπτεως, πεπτηως, τετληως, κεκοτηως). Like Εἰκως, except in 5 cells: nominative_feminine sg, nominative_feminine pl, accusative_feminine sg, vocative_feminine sg, vocative_feminine pl.
    declension!("πεπτεως", 4, "~ως", like Εἰκως;
        nominative_masculine: _, _, _;
        nominative_feminine: "~υιᾰ", _, "~υιαι";
        nominative_neuter: _, _, _;
        genitive_masculine: _, _, _;
        genitive_feminine: _, _, _;
        genitive_neuter: _, _, _;
        dative_masculine: _, _, _;
        dative_feminine: _, _, _;
        dative_neuter: _, _, _;
        accusative_masculine: _, _, _;
        accusative_feminine: "~υιᾰν", _, _;
        accusative_neuter: _, _, _;
        vocative_masculine: _, _, _;
        vocative_feminine: "~υιᾰ", _, "~υιαι";
        vocative_neuter: _, _, _;
        comparative: _;
        superlative: _;
        adverb: _;
    ),
    // Ἀργοτατος: 4 words (ἀηδεστατος, ἀνδρειοτατος, ἀργοτατος, ἀδελφοτατος). Like Ἀρξαμενος, except in 3 cells: comparative, superlative, adverb.
    declension!("ἀργοτατος", 4, "ἀ~α~ος", like Ἀρξαμενος;
        nominative_masculine: _, _, _;
        nominative_feminine: _, _, _;
        nominative_neuter: _, _, _;
        genitive_masculine: _, _, _;
        genitive_feminine: _, _, _;
        genitive_neuter: _, _, _;
        dative_masculine: _, _, _;
        dative_feminine: _, _, _;
        dative_neuter: _, _, _;
        accusative_masculine: _, _, _;
        accusative_feminine: _, _, _;
        accusative_neuter: _, _, _;
        vocative_masculine: _, _, _;
        vocative_feminine: _, _, _;
        vocative_neuter: _, _, _;
        comparative: derived;
        superlative: derived;
        adverb: "ᾰ̓~ᾰ~ᾰ";
    ),
    // Ἀκυρος: 4 words (ἀσυλος, ἀπυγος, ἀχρυσος, ἀκυρος).
    declension!("ἀκυρος", 4, "ἀ~υ~ος";
        nominative_masculine: "ᾰ̓~ῡ~ος", "ᾰ̓~ῡ~ω", "ᾰ̓~ῡ~οι";
        nominative_feminine: "ᾰ̓~ῡ~ος", "ᾰ̓~ῡ~ω", "ᾰ̓~ῡ~οι";
        nominative_neuter: "ᾰ̓~ῡ~ον", "ᾰ̓~ῡ~ω", "ᾰ̓~ῡ~ᾰ";
        genitive_masculine: "ᾰ̓~ῡ~ου", "ᾰ̓~ῡ~οιν", "ᾰ̓~ῡ~ων";
        genitive_feminine: "ᾰ̓~ῡ~ου", "ᾰ̓~ῡ~οιν", "ᾰ̓~ῡ~ων";
        genitive_neuter: "ᾰ̓~ῡ~ου", "ᾰ̓~ῡ~οιν", "ᾰ̓~ῡ~ων";
        dative_masculine: "ᾰ̓~ῡ~ῳ", "ᾰ̓~ῡ~οιν", "ᾰ̓~ῡ~οις";
        dative_feminine: "ᾰ̓~ῡ~ῳ", "ᾰ̓~ῡ~οιν", "ᾰ̓~ῡ~οις";
        dative_neuter: "ᾰ̓~ῡ~ῳ", "ᾰ̓~ῡ~οιν", "ᾰ̓~ῡ~οις";
        accusative_masculine: "ᾰ̓~ῡ~ον", "ᾰ̓~ῡ~ω", "ᾰ̓~ῡ~ους";
        accusative_feminine: "ᾰ̓~ῡ~ον", "ᾰ̓~ῡ~ω", "ᾰ̓~ῡ~ους";
        accusative_neuter: "ᾰ̓~ῡ~ον", "ᾰ̓~ῡ~ω", "ᾰ̓~ῡ~ᾰ";
        vocative_masculine: "ᾰ̓~ῡ~ε", "ᾰ̓~ῡ~ω", "ᾰ̓~ῡ~οι";
        vocative_feminine: "ᾰ̓~ῡ~ε", "ᾰ̓~ῡ~ω", "ᾰ̓~ῡ~οι";
        vocative_neuter: "ᾰ̓~ῡ~ον", "ᾰ̓~ῡ~ω", "ᾰ̓~ῡ~ᾰ";
        comparative: "ᾰ̓~ῡ~οτερος";
        superlative: "ᾰ̓~ῡ~οτᾰτος";
        adverb: "ᾰ̓~ῡ~ως";
    ),
    // Ἰταμος: 4 words (ἰταμος, ἰσαιτατος, ἰσοτατος, ἰσωτατος).
    declension!("ἰταμος", 4, "ἰ~α~ος";
        nominative_masculine: "ῐ̓~ᾰ~ος", "ῐ̓~ᾰ~ω", "ῐ̓~ᾰ~οι";
        nominative_feminine: "ῐ̓~ᾰ~η", "ῐ̓~ᾰ~ᾱ", "ῐ̓~ᾰ~αι";
        nominative_neuter: "ῐ̓~ᾰ~ον", "ῐ̓~ᾰ~ω", "ῐ̓~ᾰ~ᾰ";
        genitive_masculine: "ῐ̓~ᾰ~ου", "ῐ̓~ᾰ~οιν", "ῐ̓~ᾰ~ων";
        genitive_feminine: "ῐ̓~ᾰ~ης", "ῐ̓~ᾰ~αιν", "ῐ̓~ᾰ~ων";
        genitive_neuter: "ῐ̓~ᾰ~ου", "ῐ̓~ᾰ~οιν", "ῐ̓~ᾰ~ων";
        dative_masculine: "ῐ̓~ᾰ~ῳ", "ῐ̓~ᾰ~οιν", "ῐ̓~ᾰ~οις";
        dative_feminine: "ῐ̓~ᾰ~ῃ", "ῐ̓~ᾰ~αιν", "ῐ̓~ᾰ~αις";
        dative_neuter: "ῐ̓~ᾰ~ῳ", "ῐ̓~ᾰ~οιν", "ῐ̓~ᾰ~οις";
        accusative_masculine: "ῐ̓~ᾰ~ον", "ῐ̓~ᾰ~ω", "ῐ̓~ᾰ~ους";
        accusative_feminine: "ῐ̓~ᾰ~ην", "ῐ̓~ᾰ~ᾱ", "ῐ̓~ᾰ~ᾱς";
        accusative_neuter: "ῐ̓~ᾰ~ον", "ῐ̓~ᾰ~ω", "ῐ̓~ᾰ~ᾰ";
        vocative_masculine: "ῐ̓~ᾰ~ε", "ῐ̓~ᾰ~ω", "ῐ̓~ᾰ~οι";
        vocative_feminine: "ῐ̓~ᾰ~η", "ῐ̓~ᾰ~ᾱ", "ῐ̓~ᾰ~αι";
        vocative_neuter: "ῐ̓~ᾰ~ον", "ῐ̓~ᾰ~ω", "ῐ̓~ᾰ~ᾰ";
        comparative: derived;
        superlative: derived;
        adverb: "ῐ̓~ᾰ~ᾰ";
    ),
    // Πρωτος: 3 words (πρωτος, κᾰλλῐστος, ἀγχιστος). Like Ἡκῐστος, except in 2 cells: superlative, adverb.
    declension!("πρωτος", 3, "~ος", like Ἡκῐστος;
        nominative_masculine: _, _, _;
        nominative_feminine: _, _, _;
        nominative_neuter: _, _, _;
        genitive_masculine: _, _, _;
        genitive_feminine: _, _, _;
        genitive_neuter: _, _, _;
        dative_masculine: _, _, _;
        dative_feminine: _, _, _;
        dative_neuter: _, _, _;
        accusative_masculine: _, _, _;
        accusative_feminine: _, _, _;
        accusative_neuter: _, _, _;
        vocative_masculine: _, _, _;
        vocative_feminine: _, _, _;
        vocative_neuter: _, _, _;
        comparative: _;
        superlative: "~ῐστος";
        adverb: "~ον";
    ),
    // Βαιος: 3 words (πᾰλαιος, βαιος, ἠβαιος). Like Λᾰμπρος, except in 1 cell: adverb.
    declension!("βαιος", 3, "~ος", like Λᾰμπρος;
        nominative_masculine: _, _, _;
        nominative_feminine: _, _, _;
        nominative_neuter: _, _, _;
        genitive_masculine: _, _, _;
        genitive_feminine: _, _, _;
        genitive_neuter: _, _, _;
        dative_masculine: _, _, _;
        dative_feminine: _, _, _;
        dative_neuter: _, _, _;
        accusative_masculine: _, _, _;
        accusative_feminine: _, _, _;
        accusative_neuter: _, _, _;
        vocative_masculine: _, _, _;
        vocative_feminine: _, _, _;
        vocative_neuter: _, _, _;
        comparative: _;
        superlative: _;
        adverb: "~ον";
    ),
    // Θηλῠς: 3 words (ἡμῐσῠς, θηλῠς, πρεσβῠς). Like Τρᾱχῠς, except in 19 cells: nominative_masculine du, nominative_masculine pl, nominative_feminine sg, nominative_feminine pl, nominative_neuter du, nominative_neuter pl, ….
    declension!("θηλῠς", 3, "~ῠς", like Τρᾱχῠς;
        nominative_masculine: _, "~ει", "~εις";
        nominative_feminine: "~ειᾰ", _, "~ειαι";
        nominative_neuter: _, "~ει", "~η|~εᾰ";
        genitive_masculine: _, _, _;
        genitive_feminine: _, _, _;
        genitive_neuter: _, _, _;
        dative_masculine: "~ει", _, _;
        dative_feminine: _, _, _;
        dative_neuter: "~ει", _, _;
        accusative_masculine: _, "~ει", "~εις";
        accusative_feminine: "~ειᾰν", _, _;
        accusative_neuter: _, "~ει", "~η|~εᾰ";
        vocative_masculine: _, "~ει", "~εις";
        vocative_feminine: "~ειᾰ", _, "~ειαι";
        vocative_neuter: _, "~ει", "~η|~εᾰ";
        comparative: _;
        superlative: _;
        adverb: _;
    ),
    // Δῐᾱνταῑος: 3 words (χῠδαῑος, ᾰμορβαῑος, δῐᾱνταῑος). Like Οἰκεῑος, except in 14 cells: nominative_feminine sg, nominative_feminine du, nominative_feminine pl, genitive_feminine sg, genitive_feminine du, dative_feminine sg, ….
    declension!("δῐᾱνταῑος", 3, "~ῑος", like Οἰκεῑος;
        nominative_masculine: _, _, _;
        nominative_feminine: "~ῑος", "~ιω", "~ῑοι";
        nominative_neuter: _, _, _;
        genitive_masculine: _, _, _;
        genitive_feminine: "~ιου", "~ιοιν", _;
        genitive_neuter: _, _, _;
        dative_masculine: _, _, _;
        dative_feminine: "~ιῳ", "~ιοιν", "~ιοις";
        dative_neuter: _, _, _;
        accusative_masculine: _, _, _;
        accusative_feminine: "~ῑον", "~ιω", "~ιους";
        accusative_neuter: _, _, _;
        vocative_masculine: _, _, _;
        vocative_feminine: "~ῑε", "~ιω", "~ῑοι";
        vocative_neuter: _, _, _;
        comparative: _;
        superlative: _;
        adverb: _;
    ),
    // Ἰσοδυναμος: 3 words (ἰχθυοφαγος, ἰχθυφαγος, ἰσοδυναμος).
    declension!("ἰσοδυναμος", 3, "ἰ~υ~α~ος";
        nominative_masculine: "ῐ̓~ῠ~ᾰ~ος", "ῐ̓~ῠ~ᾰ~ω", "ῐ̓~ῠ~ᾰ~οι";
        nominative_feminine: "ῐ̓~ῠ~ᾰ~ος", "ῐ̓~ῠ~ᾰ~ω", "ῐ̓~ῠ~ᾰ~οι";
        nominative_neuter: "ῐ̓~ῠ~ᾰ~ον", "ῐ̓~ῠ~ᾰ~ω", "ῐ̓~ῠ~ᾰ~ᾰ";
        genitive_masculine: "ῐ̓~ῠ~ᾰ~ου", "ῐ̓~ῠ~ᾰ~οιν", "ῐ̓~ῠ~ᾰ~ων";
        genitive_feminine: "ῐ̓~ῠ~ᾰ~ου", "ῐ̓~ῠ~ᾰ~οιν", "ῐ̓~ῠ~ᾰ~ων";
        genitive_neuter: "ῐ̓~ῠ~ᾰ~ου", "ῐ̓~ῠ~ᾰ~οιν", "ῐ̓~ῠ~ᾰ~ων";
        dative_masculine: "ῐ̓~ῠ~ᾰ~ῳ", "ῐ̓~ῠ~ᾰ~οιν", "ῐ̓~ῠ~ᾰ~οις";
        dative_feminine: "ῐ̓~ῠ~ᾰ~ῳ", "ῐ̓~ῠ~ᾰ~οιν", "ῐ̓~ῠ~ᾰ~οις";
        dative_neuter: "ῐ̓~ῠ~ᾰ~ῳ", "ῐ̓~ῠ~ᾰ~οιν", "ῐ̓~ῠ~ᾰ~οις";
        accusative_masculine: "ῐ̓~ῠ~ᾰ~ον", "ῐ̓~ῠ~ᾰ~ω", "ῐ̓~ῠ~ᾰ~ους";
        accusative_feminine: "ῐ̓~ῠ~ᾰ~ον", "ῐ̓~ῠ~ᾰ~ω", "ῐ̓~ῠ~ᾰ~ους";
        accusative_neuter: "ῐ̓~ῠ~ᾰ~ον", "ῐ̓~ῠ~ᾰ~ω", "ῐ̓~ῠ~ᾰ~ᾰ";
        vocative_masculine: "ῐ̓~ῠ~ᾰ~ε", "ῐ̓~ῠ~ᾰ~ω", "ῐ̓~ῠ~ᾰ~οι";
        vocative_feminine: "ῐ̓~ῠ~ᾰ~ε", "ῐ̓~ῠ~ᾰ~ω", "ῐ̓~ῠ~ᾰ~οι";
        vocative_neuter: "ῐ̓~ῠ~ᾰ~ον", "ῐ̓~ῠ~ᾰ~ω", "ῐ̓~ῠ~ᾰ~ᾰ";
        comparative: "ῐ̓~ῠ~ᾰ~ωτερος";
        superlative: "ῐ̓~ῠ~ᾰ~ωτᾰτος";
        adverb: "ῐ̓~ῠ~ᾰ~ως";
    ),
    // Ὑστερος: 3 words (ὑστερος, ὑπερτερος, ὑψηλοτερος).
    declension!("ὑστερος", 3, "ὑ~ερος";
        nominative_masculine: "ῠ̔~ερος", "ῠ̔~ερω", "ῠ̔~εροι";
        nominative_feminine: "ῠ̔~ερᾱ", "ῠ̔~ερᾱ", "ῠ̔~εραι";
        nominative_neuter: "ῠ̔~ερον", "ῠ̔~ερω", "ῠ̔~ερᾰ";
        genitive_masculine: "ῠ̔~ερου", "ῠ̔~εροιν", "ῠ̔~ερων";
        genitive_feminine: "ῠ̔~ερᾱς", "ῠ̔~εραιν", "ῠ̔~ερων";
        genitive_neuter: "ῠ̔~ερου", "ῠ̔~εροιν", "ῠ̔~ερων";
        dative_masculine: "ῠ̔~ερῳ", "ῠ̔~εροιν", "ῠ̔~εροις";
        dative_feminine: "ῠ̔~ερᾳ", "ῠ̔~εραιν", "ῠ̔~εραις";
        dative_neuter: "ῠ̔~ερῳ", "ῠ̔~εροιν", "ῠ̔~εροις";
        accusative_masculine: "ῠ̔~ερον", "ῠ̔~ερω", "ῠ̔~ερους";
        accusative_feminine: "ῠ̔~ερᾱν", "ῠ̔~ερᾱ", "ῠ̔~ερᾱς";
        accusative_neuter: "ῠ̔~ερον", "ῠ̔~ερω", "ῠ̔~ερᾰ";
        vocative_masculine: "ῠ̔~ερε", "ῠ̔~ερω", "ῠ̔~εροι";
        vocative_feminine: "ῠ̔~ερᾱ", "ῠ̔~ερᾱ", "ῠ̔~εραι";
        vocative_neuter: "ῠ̔~ερον", "ῠ̔~ερω", "ῠ̔~ερᾰ";
        comparative: "ῠ̔~ερος";
        superlative: "ῠ̔~ᾰτος";
        adverb: "ῠ̔~ερον";
    ),
    // Ἀκηριος: 3 words (ἀποφωλιος, ἀεκουσιος, ἀκηριος). Like Ἀρτιος, except in 14 cells: nominative_feminine sg, nominative_feminine du, nominative_feminine pl, genitive_feminine sg, genitive_feminine du, dative_feminine sg, ….
    declension!("ἀκηριος", 3, "ἀ~ιος", like Ἀρτιος;
        nominative_masculine: _, _, _;
        nominative_feminine: "ᾰ̓~ῐος", "ᾰ̓~ῐω", "ᾰ̓~ῐοι";
        nominative_neuter: _, _, _;
        genitive_masculine: _, _, _;
        genitive_feminine: "ᾰ̓~ῐου", "ᾰ̓~ῐοιν", _;
        genitive_neuter: _, _, _;
        dative_masculine: _, _, _;
        dative_feminine: "ᾰ̓~ῐῳ", "ᾰ̓~ῐοιν", "ᾰ̓~ῐοις";
        dative_neuter: _, _, _;
        accusative_masculine: _, _, _;
        accusative_feminine: "ᾰ̓~ῐον", "ᾰ̓~ῐω", "ᾰ̓~ῐους";
        accusative_neuter: _, _, _;
        vocative_masculine: _, _, _;
        vocative_feminine: "ᾰ̓~ῐε", "ᾰ̓~ῐω", "ᾰ̓~ῐοι";
        vocative_neuter: _, _, _;
        comparative: _;
        superlative: _;
        adverb: _;
    ),
    // Ἀποκᾰταστᾰτῐκος: 3 words (ἀγανος, ἀμαλος, ἀποκᾰταστᾰτῐκος). Like Ἀγαπητος, except in 2 cells: comparative, superlative.
    declension!("ἀποκᾰταστᾰτῐκος", 3, "ἀ~α~ος", like Ἀγαπητος;
        nominative_masculine: _, _, _;
        nominative_feminine: _, _, _;
        nominative_neuter: _, _, _;
        genitive_masculine: _, _, _;
        genitive_feminine: _, _, _;
        genitive_neuter: _, _, _;
        dative_masculine: _, _, _;
        dative_feminine: _, _, _;
        dative_neuter: _, _, _;
        accusative_masculine: _, _, _;
        accusative_feminine: _, _, _;
        accusative_neuter: _, _, _;
        vocative_masculine: _, _, _;
        vocative_feminine: _, _, _;
        vocative_neuter: _, _, _;
        comparative: "ᾰ̓~ᾰ~ωτερος";
        superlative: "ᾰ̓~ᾰ~ωτᾰτος";
        adverb: _;
    ),
    // Μωνῠξ: 3 words (μωνῠξ, γᾰμψωνῠξ, κρᾰτερωνῠξ).
    declension!("μωνῠξ", 3, "~ξ";
        nominative_masculine: "~ξ", "~χε", "~χες";
        nominative_feminine: "~ξ", "~χε", "~χες";
        nominative_neuter: "~ξ", "~χε", "~χᾰ";
        genitive_masculine: "~χος", "~χοιν", "~χων";
        genitive_feminine: "~χος", "~χοιν", "~χων";
        genitive_neuter: "~χος", "~χοιν", "~χων";
        dative_masculine: "~χῐ", "~χοιν", "~ξῐ|~ξῐν";
        dative_feminine: "~χῐ", "~χοιν", "~ξῐ|~ξῐν";
        dative_neuter: "~χῐ", "~χοιν", "~ξῐ|~ξῐν";
        accusative_masculine: "~χᾰ", "~χε", "~χᾰς";
        accusative_feminine: "~χᾰ", "~χε", "~χᾰς";
        accusative_neuter: "~ξ", "~χε", "~χᾰ";
        vocative_masculine: "~ξ", "~χε", "~χες";
        vocative_feminine: "~ξ", "~χε", "~χες";
        vocative_neuter: "~ξ", "~χε", "~χᾰ";
        comparative: derived;
        superlative: derived;
        adverb: "~χως";
    ),
    // Ἀργος: 2 words (ἀδελφος, ἀοιδος). Like Ἀποδεικτῐκος, except in 2 cells: comparative, superlative.
    declension!("ἀργος", 2, "ἀ~ος", like Ἀποδεικτῐκος;
        nominative_masculine: _, _, _;
        nominative_feminine: _, _, _;
        nominative_neuter: _, _, _;
        genitive_masculine: _, _, _;
        genitive_feminine: _, _, _;
        genitive_neuter: _, _, _;
        dative_masculine: _, _, _;
        dative_feminine: _, _, _;
        dative_neuter: _, _, _;
        accusative_masculine: _, _, _;
        accusative_feminine: _, _, _;
        accusative_neuter: _, _, _;
        vocative_masculine: _, _, _;
        vocative_feminine: _, _, _;
        vocative_neuter: _, _, _;
        comparative: "ᾰ̓~οτερος";
        superlative: "ᾰ̓~οτᾰτος";
        adverb: _;
    ),
    // Ἀσυμπτωτος: 1 words (ἀρρυθμος). Like Ἀσυνδετος, except in 2 cells: comparative, superlative.
    declension!("ἀσυμπτωτος", 1, "ἀ~υ~ος", like Ἀσυνδετος;
        nominative_masculine: _, _, _;
        nominative_feminine: _, _, _;
        nominative_neuter: _, _, _;
        genitive_masculine: _, _, _;
        genitive_feminine: _, _, _;
        genitive_neuter: _, _, _;
        dative_masculine: _, _, _;
        dative_feminine: _, _, _;
        dative_neuter: _, _, _;
        accusative_masculine: _, _, _;
        accusative_feminine: _, _, _;
        accusative_neuter: _, _, _;
        vocative_masculine: _, _, _;
        vocative_feminine: _, _, _;
        vocative_neuter: _, _, _;
        comparative: "ᾰ̓~ῠ~οτερος";
        superlative: "ᾰ̓~ῠ~οτᾰτος";
        adverb: _;
    ),
    // Ἀοριστος: 1 words (ἀκινδῡνος). Like Ἀδιψος, except in 2 cells: comparative, superlative.
    declension!("ἀοριστος", 1, "ἀ~ι~ος", like Ἀδιψος;
        nominative_masculine: _, _, _;
        nominative_feminine: _, _, _;
        nominative_neuter: _, _, _;
        genitive_masculine: _, _, _;
        genitive_feminine: _, _, _;
        genitive_neuter: _, _, _;
        dative_masculine: _, _, _;
        dative_feminine: _, _, _;
        dative_neuter: _, _, _;
        accusative_masculine: _, _, _;
        accusative_feminine: _, _, _;
        accusative_neuter: _, _, _;
        vocative_masculine: _, _, _;
        vocative_feminine: _, _, _;
        vocative_neuter: _, _, _;
        comparative: "ᾰ̓~ῐ~οτερος";
        superlative: "ᾰ̓~ῐ~οτᾰτος";
        adverb: _;
    ),
];
