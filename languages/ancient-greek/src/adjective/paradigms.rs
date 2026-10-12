//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's feature says):
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
    ($name:literal, $words:literal, $lemma:literal;
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
                flexion::form!($c15),
                flexion::form!($c16),
                flexion::form!($c17),
                flexion::form!($c18),
                flexion::form!($c19),
                flexion::form!($c20),
                flexion::form!($c21),
                flexion::form!($c22),
                flexion::form!($c23),
                flexion::form!($c24),
                flexion::form!($c25),
                flexion::form!($c26),
                flexion::form!($c27),
                flexion::form!($c28),
                flexion::form!($c29),
                flexion::form!($c30),
                flexion::form!($c31),
                flexion::form!($c32),
                flexion::form!($c33),
                flexion::form!($c34),
                flexion::form!($c35),
                flexion::form!($c36),
                flexion::form!($c37),
                flexion::form!($c38),
                flexion::form!($c39),
                flexion::form!($c40),
                flexion::form!($c41),
                flexion::form!($c42),
                flexion::form!($c43),
                flexion::form!($c44),
                flexion::form!($c45),
                flexion::form!($c46),
                flexion::form!($c47),
            ]),
        }
    };
}

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `ἐσχᾰτος`: 794 words.
    Ἐσχᾰτος,
    /// Like `ἐμπεδος`: 488 words.
    Ἐμπεδος,
    /// Like `μᾰλᾰκος`: 455 words.
    Μᾰλᾰκος,
    /// Like `ἀτακτος`: 311 words.
    Ἀτακτος,
    /// Like `ψῑλος`: 280 words.
    Ψῑλος,
    /// Like `ἀτελης`: 203 words.
    Ἀτελης,
    /// Like `προσηκων`: 195 words.
    Προσηκων,
    /// Like `τελειος`: 176 words.
    Τελειος,
    /// Like `προπᾱς`: 130 words.
    Προπᾱς,
    /// Like `πλεῑστος`: 94 words.
    Πλεῑστος,
    /// Like `λᾰμπρος`: 87 words.
    Λᾰμπρος,
    /// Like `ὑληεις`: 86 words.
    Ὑληεις,
    /// Like `ἡκῐστος`: 84 words.
    Ἡκῐστος,
    /// Like `κῐων`: 68 words.
    Κῐων,
    /// Like `οἰκεῑος`: 63 words.
    Οἰκεῑος,
    /// Like `φᾰνερος`: 55 words.
    Φᾰνερος,
    /// Like `δῐκαιος`: 53 words.
    Δῐκαιος,
    /// Like `ἀρκων`: 51 words.
    Ἀρκων,
    /// Like `κᾰκοηθης`: 48 words.
    Κᾰκοηθης,
    /// Like `εἰκως`: 48 words.
    Εἰκως,
    /// Like `ἐρημος`: 47 words.
    Ἐρημος,
    /// Like `ἀσημος`: 41 words.
    Ἀσημος,
    /// Like `μαυρος`: 40 words.
    Μαυρος,
    /// Like `ἐκλῠτηρῐον`: 35 words.
    Ἐκλῠτηρῐον,
    /// Like `ἀξῐοχρεως`: 33 words.
    Ἀξῐοχρεως,
    /// Like `πλειων`: 29 words.
    Πλειων,
    /// Like `ἀκρῐτος`: 27 words.
    Ἀκρῐτος,
    /// Like `ἀκρᾰτης`: 26 words.
    Ἀκρᾰτης,
    /// Like `ἀλλος`: 25 words.
    Ἀλλος,
    /// Like `φωνηεις`: 25 words.
    Φωνηεις,
    /// Like `τρᾱχῠς`: 24 words.
    Τρᾱχῠς,
    /// Like `βαινων`: 21 words.
    Βαινων,
    /// Like `ἀκαρπος`: 19 words.
    Ἀκαρπος,
    /// Like `τρῐπους`: 18 words.
    Τρῐπους,
    /// Like `ἡσσων`: 18 words.
    Ἡσσων,
    /// Like `παυων`: 17 words.
    Παυων,
    /// Like `ἀπατωρ`: 17 words.
    Ἀπατωρ,
    /// Like `ἀγων`: 16 words.
    Ἀγων,
    /// Like `δειξᾱς`: 16 words.
    Δειξᾱς,
    /// Like `ἀβατος`: 13 words.
    Ἀβατος,
    /// Like `ὑπαιθρος`: 13 words.
    Ὑπαιθρος,
    /// Like `ἀδιψος`: 12 words.
    Ἀδιψος,
    /// Like `ἀρχικος`: 12 words.
    Ἀρχικος,
    /// Like `ἀκροτερος`: 12 words.
    Ἀκροτερος,
    /// Like `ἀγνως`: 11 words.
    Ἀγνως,
    /// Like `φαῡλος`: 11 words.
    Φαῡλος,
    /// Like `ἀγοραῑος`: 10 words.
    Ἀγοραῑος,
    /// Like `γαῡρος`: 10 words.
    Γαῡρος,
    /// Like `πλωϊμος`: 10 words.
    Πλωϊμος,
    /// Like `ὑποθετικος`: 10 words.
    Ὑποθετικος,
    /// Like `κολοβος`: 9 words.
    Κολοβος,
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
    /// Like `ἀποδεικτῐκος`: 9 words.
    Ἀποδεικτῐκος,
    /// Like `ἀντιπαλος`: 9 words.
    Ἀντιπαλος,
    /// Like `ἀσυνδετος`: 8 words.
    Ἀσυνδετος,
    /// Like `θειων`: 8 words.
    Θειων,
    /// Like `ἀξιωτερος`: 8 words.
    Ἀξιωτερος,
    /// Like `ἀκρος`: 7 words.
    Ἀκρος,
    /// Like `ἀγραμματος`: 7 words.
    Ἀγραμματος,
    /// Like `ἀσιτος`: 7 words.
    Ἀσιτος,
    /// Like `ἁβρος`: 7 words.
    Ἁβρος,
    /// Like `φῡς`: 7 words.
    Φῡς,
    /// Like `ἀρξαμενος`: 7 words.
    Ἀρξαμενος,
    /// Like `ἰσος`: 7 words.
    Ἰσος,
    /// Like `ἀκαθαρτος`: 6 words.
    Ἀκαθαρτος,
    /// Like `ἀφροδῑσῐος`: 6 words.
    Ἀφροδῑσῐος,
    /// Like `ἀσυμπτωτος`: 6 words.
    Ἀσυμπτωτος,
    /// Like `ἀκμαζων`: 6 words.
    Ἀκμαζων,
    /// Like `ὑπεγγῠος`: 6 words.
    Ὑπεγγῠος,
    /// Like `ἀργος`: 5 words.
    Ἀργος,
    /// Like `ἀοριστος`: 5 words.
    Ἀοριστος,
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
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 105] = [
        Declension::Ἐσχᾰτος,
        Declension::Ἐμπεδος,
        Declension::Μᾰλᾰκος,
        Declension::Ἀτακτος,
        Declension::Ψῑλος,
        Declension::Ἀτελης,
        Declension::Προσηκων,
        Declension::Τελειος,
        Declension::Προπᾱς,
        Declension::Πλεῑστος,
        Declension::Λᾰμπρος,
        Declension::Ὑληεις,
        Declension::Ἡκῐστος,
        Declension::Κῐων,
        Declension::Οἰκεῑος,
        Declension::Φᾰνερος,
        Declension::Δῐκαιος,
        Declension::Ἀρκων,
        Declension::Κᾰκοηθης,
        Declension::Εἰκως,
        Declension::Ἐρημος,
        Declension::Ἀσημος,
        Declension::Μαυρος,
        Declension::Ἐκλῠτηρῐον,
        Declension::Ἀξῐοχρεως,
        Declension::Πλειων,
        Declension::Ἀκρῐτος,
        Declension::Ἀκρᾰτης,
        Declension::Ἀλλος,
        Declension::Φωνηεις,
        Declension::Τρᾱχῠς,
        Declension::Βαινων,
        Declension::Ἀκαρπος,
        Declension::Τρῐπους,
        Declension::Ἡσσων,
        Declension::Παυων,
        Declension::Ἀπατωρ,
        Declension::Ἀγων,
        Declension::Δειξᾱς,
        Declension::Ἀβατος,
        Declension::Ὑπαιθρος,
        Declension::Ἀδιψος,
        Declension::Ἀρχικος,
        Declension::Ἀκροτερος,
        Declension::Ἀγνως,
        Declension::Φαῡλος,
        Declension::Ἀγοραῑος,
        Declension::Γαῡρος,
        Declension::Πλωϊμος,
        Declension::Ὑποθετικος,
        Declension::Κολοβος,
        Declension::Ἀμαθης,
        Declension::Ὑψηλος,
        Declension::Κᾰλλῐθρῐξ,
        Declension::Ζευξᾱς,
        Declension::Παυσων,
        Declension::Ἀμπελοεις,
        Declension::Ἀριστος,
        Declension::Ἀποδεικτῐκος,
        Declension::Ἀντιπαλος,
        Declension::Ἀσυνδετος,
        Declension::Θειων,
        Declension::Ἀξιωτερος,
        Declension::Ἀκρος,
        Declension::Ἀγραμματος,
        Declension::Ἀσιτος,
        Declension::Ἁβρος,
        Declension::Φῡς,
        Declension::Ἀρξαμενος,
        Declension::Ἰσος,
        Declension::Ἀκαθαρτος,
        Declension::Ἀφροδῑσῐος,
        Declension::Ἀσυμπτωτος,
        Declension::Ἀκμαζων,
        Declension::Ὑπεγγῠος,
        Declension::Ἀργος,
        Declension::Ἀοριστος,
        Declension::Αἰθοψ,
        Declension::Δους,
        Declension::Ἀντιπαις,
        Declension::Ἀγωγος,
        Declension::Ἀνομωτᾰτος,
        Declension::Ὑπατος,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 105] = [
    // Ἐσχᾰτος: 794 words (ἐρωμενος, ἁγιος, ὀλῐγος, ὁλος, ἐσχᾰτος, ξενος).
    declension!("ἐσχᾰτος", 794, "~ος";
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
    // Ἐμπεδος: 488 words (δῐᾰβολος, βᾰρβᾰρος, πρωτοτοκος, μᾰγος, θεομᾰχος, δολοφονος).
    declension!("ἐμπεδος", 488, "~ος";
        nominative_masculine: "~ος", "~ω", "~οι";
        nominative_feminine: "~ος", "~ω", "~οι";
        nominative_neuter: "~ον", "~ω", "~ᾰ";
        genitive_masculine: "~ου", "~οιν", "~ων";
        genitive_feminine: "~ου", "~οιν", "~ων";
        genitive_neuter: "~ου", "~οιν", "~ων";
        dative_masculine: "~ῳ", "~οιν", "~οις";
        dative_feminine: "~ῳ", "~οιν", "~οις";
        dative_neuter: "~ῳ", "~οιν", "~οις";
        accusative_masculine: "~ον", "~ω", "~ους";
        accusative_feminine: "~ον", "~ω", "~ους";
        accusative_neuter: "~ον", "~ω", "~ᾰ";
        vocative_masculine: "~ε", "~ω", "~οι";
        vocative_feminine: "~ε", "~ω", "~οι";
        vocative_neuter: "~ον", "~ω", "~ᾰ";
        comparative: "~ωτερος";
        superlative: "~ωτᾰτος";
        adverb: "~ως";
    ),
    // Μᾰλᾰκος: 455 words (παιδῐκος, γᾰλᾰκτῐκος, κᾰκος, πᾰρᾰλῠτῐκος, λογῐκος, ἑλληνῐκος).
    declension!("μᾰλᾰκος", 455, "~ος";
        nominative_masculine: "~ος", "~ω", "~οι";
        nominative_feminine: "~η", "~ᾱ", "~αι";
        nominative_neuter: "~ον", "~ω", "~ᾰ";
        genitive_masculine: "~οῡ", "~οῑν", "~ων";
        genitive_feminine: "~ης", "~αῑν", "~ων";
        genitive_neuter: "~οῡ", "~οῑν", "~ων";
        dative_masculine: "~ῳ", "~οῑν", "~οῑς";
        dative_feminine: "~ῃ", "~αῑν", "~αῑς";
        dative_neuter: "~ῳ", "~οῑν", "~οῑς";
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
    // Ἀτακτος: 311 words (εὐχᾰρῐστος, ἀπροσδοκητος, ἀτελειωτος, ἡδονοπληκτος, πᾰρᾰκλητος, ἀβυσσος).
    declension!("ἀτακτος", 311, "~ος";
        nominative_masculine: "~ος", "~ω", "~οι";
        nominative_feminine: "~ος", "~ω", "~οι";
        nominative_neuter: "~ον", "~ω", "~ᾰ";
        genitive_masculine: "~ου", "~οιν", "~ων";
        genitive_feminine: "~ου", "~οιν", "~ων";
        genitive_neuter: "~ου", "~οιν", "~ων";
        dative_masculine: "~ῳ", "~οιν", "~οις";
        dative_feminine: "~ῳ", "~οιν", "~οις";
        dative_neuter: "~ῳ", "~οιν", "~οις";
        accusative_masculine: "~ον", "~ω", "~ους";
        accusative_feminine: "~ον", "~ω", "~ους";
        accusative_neuter: "~ον", "~ω", "~ᾰ";
        vocative_masculine: "~ε", "~ω", "~οι";
        vocative_feminine: "~ε", "~ω", "~οι";
        vocative_neuter: "~ον", "~ω", "~ᾰ";
        comparative: "~οτερος";
        superlative: "~οτᾰτος";
        adverb: "~ως";
    ),
    // Ψῑλος: 280 words (λευκος, πνευματικος, κρῠπτος, ἰουδαϊκος, ξᾰνθος, σκοτεινος).
    declension!("ψῑλος", 280, "~ος";
        nominative_masculine: "~ος", "~ω", "~οι";
        nominative_feminine: "~η", "~ᾱ", "~αι";
        nominative_neuter: "~ον", "~ω", "~ᾰ";
        genitive_masculine: "~οῡ", "~οῑν", "~ων";
        genitive_feminine: "~ης", "~αῑν", "~ων";
        genitive_neuter: "~οῡ", "~οῑν", "~ων";
        dative_masculine: "~ῳ", "~οῑν", "~οῑς";
        dative_feminine: "~ῃ", "~αῑν", "~αῑς";
        dative_neuter: "~ῳ", "~οῑν", "~οῑς";
        accusative_masculine: "~ον", "~ω", "~ους";
        accusative_feminine: "~ην", "~ᾱ", "~ᾱς";
        accusative_neuter: "~ον", "~ω", "~ᾰ";
        vocative_masculine: "~ε", "~ω", "~οι";
        vocative_feminine: "~η", "~ᾱ", "~αι";
        vocative_neuter: "~ον", "~ω", "~ᾰ";
        comparative: "~οτερος";
        superlative: "~οτᾰτος";
        adverb: "~ως";
    ),
    // Ἀτελης: 203 words (ὑγιης, ἀβαρης, ψευδης, ἀτελης, σῑγμοειδης, σῠγγενης).
    declension!("ἀτελης", 203, "~ης";
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
        comparative: "~ονεστερος";
        superlative: "~ονεστᾰτος";
        adverb: "~οντως";
    ),
    // Τελειος: 176 words (κῡρῐος, νεος, πλουσῐος, ὁσῐος, σαρδανῐος, δημοσῐος).
    declension!("τελειος", 176, "~ος";
        nominative_masculine: "~ος", "~ω", "~οι";
        nominative_feminine: "~ᾱ", "~ᾱ", "~αι";
        nominative_neuter: "~ον", "~ω", "~ᾰ";
        genitive_masculine: "~ου", "~οιν", "~ων";
        genitive_feminine: "~ᾱς", "~αιν", "~ων";
        genitive_neuter: "~ου", "~οιν", "~ων";
        dative_masculine: "~ῳ", "~οιν", "~οις";
        dative_feminine: "~ᾳ", "~αιν", "~αις";
        dative_neuter: "~ῳ", "~οιν", "~οις";
        accusative_masculine: "~ον", "~ω", "~ους";
        accusative_feminine: "~ᾱν", "~ᾱ", "~ᾱς";
        accusative_neuter: "~ον", "~ω", "~ᾰ";
        vocative_masculine: "~ε", "~ω", "~οι";
        vocative_feminine: "~ᾱ", "~ᾱ", "~αι";
        vocative_neuter: "~ον", "~ω", "~ᾰ";
        comparative: "~ωτερος";
        superlative: "~ωτᾰτος";
        adverb: "~ως";
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
        comparative: "~ᾰντερος";
        superlative: "~ᾰντᾰτος";
        adverb: "~ᾰντως";
    ),
    // Πλεῑστος: 94 words (προτερος, βᾰθῠτερος, πρεσβῠτερος, βεβαιοτερος, πλεῑστος, στοιχειωδεστερος).
    declension!("πλεῑστος", 94, "~ος";
        nominative_masculine: "~ος", "~ω", "~οι";
        nominative_feminine: "~ᾱ", "~ᾱ", "~αι";
        nominative_neuter: "~ον", "~ω", "~ᾰ";
        genitive_masculine: "~ου", "~οιν", "~ων";
        genitive_feminine: "~ᾱς", "~αιν", "~ων";
        genitive_neuter: "~ου", "~οιν", "~ων";
        dative_masculine: "~ῳ", "~οιν", "~οις";
        dative_feminine: "~ᾳ", "~αιν", "~αις";
        dative_neuter: "~ῳ", "~οιν", "~οις";
        accusative_masculine: "~ον", "~ω", "~ους";
        accusative_feminine: "~ᾱν", "~ᾱ", "~ᾱς";
        accusative_neuter: "~ον", "~ω", "~ᾰ";
        vocative_masculine: "~ε", "~ω", "~οι";
        vocative_feminine: "~ᾱ", "~ᾱ", "~αι";
        vocative_neuter: "~ον", "~ω", "~ᾰ";
        comparative: "~ωτερος";
        superlative: "~ωτᾰτος";
        adverb: "~ον";
    ),
    // Λᾰμπρος: 87 words (μῑκρος, ἐρῠθρος, χλωρος, νεκρος, δεξῐος, λεπρος).
    declension!("λᾰμπρος", 87, "~ος";
        nominative_masculine: "~ος", "~ω", "~οι";
        nominative_feminine: "~ᾱ", "~ᾱ", "~αι";
        nominative_neuter: "~ον", "~ω", "~ᾰ";
        genitive_masculine: "~οῡ", "~οῑν", "~ων";
        genitive_feminine: "~ᾱς", "~αῑν", "~ων";
        genitive_neuter: "~οῡ", "~οῑν", "~ων";
        dative_masculine: "~ῳ", "~οῑν", "~οῑς";
        dative_feminine: "~ᾳ", "~αῑν", "~αῑς";
        dative_neuter: "~ῳ", "~οῑν", "~οῑς";
        accusative_masculine: "~ον", "~ω", "~ους";
        accusative_feminine: "~ᾱν", "~ᾱ", "~ᾱς";
        accusative_neuter: "~ον", "~ω", "~ᾰ";
        vocative_masculine: "~ε", "~ω", "~οι";
        vocative_feminine: "~ᾱ", "~ᾱ", "~αι";
        vocative_neuter: "~ον", "~ω", "~ᾰ";
        comparative: "~οτερος";
        superlative: "~οτᾰτος";
        adverb: "~ως";
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
        comparative: "~στερος";
        superlative: "~στᾰτος";
        adverb: "~ντως";
    ),
    // Ἡκῐστος: 84 words (μεγῐστος, ἡκῐστος, στοιχειωδεστᾰτος, κᾰκῐστος, τᾰχῐστος, βελτῐστος).
    declension!("ἡκῐστος", 84, "~ος";
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
        adverb: "~ᾰ";
    ),
    // Κῐων: 68 words (πᾰρων, ἀπων, λᾰβων, πᾰρελθων, ἑκων, εἰπων).
    declension!("κῐων", 68, "~ων";
        nominative_masculine: "~ων", "~οντε", "~οντες";
        nominative_feminine: "~οῡσᾰ", "~ουσᾱ", "~οῡσαι";
        nominative_neuter: "~ον", "~οντε", "~οντᾰ";
        genitive_masculine: "~οντος", "~οντοιν", "~οντων";
        genitive_feminine: "~ουσης", "~ουσαιν", "~ουσων";
        genitive_neuter: "~οντος", "~οντοιν", "~οντων";
        dative_masculine: "~οντῐ", "~οντοιν", "~οῡσῐ|~οῡσῐν";
        dative_feminine: "~ουσῃ", "~ουσαιν", "~ουσαις";
        dative_neuter: "~οντῐ", "~οντοιν", "~οῡσῐ|~οῡσῐν";
        accusative_masculine: "~οντᾰ", "~οντε", "~οντᾰς";
        accusative_feminine: "~οῡσᾰν", "~ουσᾱ", "~ουσᾱς";
        accusative_neuter: "~ον", "~οντε", "~οντᾰ";
        vocative_masculine: "~ων", "~οντε", "~οντες";
        vocative_feminine: "~οῡσᾰ", "~ουσᾱ", "~οῡσαι";
        vocative_neuter: "~ον", "~οντε", "~οντᾰ";
        comparative: "~ονεστερος";
        superlative: "~ονεστᾰτος";
        adverb: "~οντως";
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
    // Φᾰνερος: 55 words (κᾰθᾰρος, στερεος, σᾰρωνῐκος, σοβᾰρος, χλῐᾰρος, στᾰθερος).
    declension!("φᾰνερος", 55, "~ος";
        nominative_masculine: "~ος", "~ω", "~οι";
        nominative_feminine: "~ᾱ", "~ᾱ", "~αι";
        nominative_neuter: "~ον", "~ω", "~ᾰ";
        genitive_masculine: "~οῡ", "~οῑν", "~ων";
        genitive_feminine: "~ᾱς", "~αῑν", "~ων";
        genitive_neuter: "~οῡ", "~οῑν", "~ων";
        dative_masculine: "~ῳ", "~οῑν", "~οῑς";
        dative_feminine: "~ᾳ", "~αῑν", "~αῑς";
        dative_neuter: "~ῳ", "~οῑν", "~οῑς";
        accusative_masculine: "~ον", "~ω", "~ους";
        accusative_feminine: "~ᾱν", "~ᾱ", "~ᾱς";
        accusative_neuter: "~ον", "~ω", "~ᾰ";
        vocative_masculine: "~ε", "~ω", "~οι";
        vocative_feminine: "~ᾱ", "~ᾱ", "~αι";
        vocative_neuter: "~ον", "~ω", "~ᾰ";
        comparative: "~ωτερος";
        superlative: "~ωτᾰτος";
        adverb: "~ως";
    ),
    // Δῐκαιος: 53 words (ἑτερος, ὁμοιος, βῐαιος, δῐκαιος, μᾰταιος, φεβρουαριος).
    declension!("δῐκαιος", 53, "~ος";
        nominative_masculine: "~ος", "~ω", "~οι";
        nominative_feminine: "~ᾱ", "~ᾱ", "~αι";
        nominative_neuter: "~ον", "~ω", "~ᾰ";
        genitive_masculine: "~ου", "~οιν", "~ων";
        genitive_feminine: "~ᾱς", "~αιν", "~ων";
        genitive_neuter: "~ου", "~οιν", "~ων";
        dative_masculine: "~ῳ", "~οιν", "~οις";
        dative_feminine: "~ᾳ", "~αιν", "~αις";
        dative_neuter: "~ῳ", "~οιν", "~οις";
        accusative_masculine: "~ον", "~ω", "~ους";
        accusative_feminine: "~ᾱν", "~ᾱ", "~ᾱς";
        accusative_neuter: "~ον", "~ω", "~ᾰ";
        vocative_masculine: "~ε", "~ω", "~οι";
        vocative_feminine: "~ᾱ", "~ᾱ", "~αι";
        vocative_neuter: "~ον", "~ω", "~ᾰ";
        comparative: "~οτερος";
        superlative: "~οτᾰτος";
        adverb: "~ως";
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
        comparative: "~ονεστερος";
        superlative: "~ονεστᾰτος";
        adverb: "~ουντως";
    ),
    // Κᾰκοηθης: 48 words (πληρης, δηλητηρῐωδης, σῠνηθης, νεοδᾱμωδης, τρῐηρης, δημωδης).
    declension!("κᾰκοηθης", 48, "~ης";
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
        comparative: "~ονεστερος";
        superlative: "~ονεστᾰτος";
        adverb: "~οτως";
    ),
    // Ἐρημος: 47 words (ἀλεξανδρῑνος, λᾰτῑνος, ἀσβεστος, δηλος, κατοικος, λῐσπος).
    declension!("ἐρημος", 47, "~ος";
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
        comparative: "~οτερος";
        superlative: "~οτᾰτος";
        adverb: "~ως";
    ),
    // Ἀσημος: 41 words (ἀποπληκτος, ἀγνωστος, ἀγνωτος, ἀντοικος, ἀπειρος, ἀμορφος).
    declension!("ἀσημος", 41, "ἀ~ος";
        nominative_masculine: "ᾰ\u{313}~ος", "ᾰ\u{313}~ω", "ᾰ\u{313}~οι";
        nominative_feminine: "ᾰ\u{313}~ος", "ᾰ\u{313}~ω", "ᾰ\u{313}~οι";
        nominative_neuter: "ᾰ\u{313}~ον", "ᾰ\u{313}~ω", "ᾰ\u{313}~ᾰ";
        genitive_masculine: "ᾰ\u{313}~ου", "ᾰ\u{313}~οιν", "ᾰ\u{313}~ων";
        genitive_feminine: "ᾰ\u{313}~ου", "ᾰ\u{313}~οιν", "ᾰ\u{313}~ων";
        genitive_neuter: "ᾰ\u{313}~ου", "ᾰ\u{313}~οιν", "ᾰ\u{313}~ων";
        dative_masculine: "ᾰ\u{313}~ῳ", "ᾰ\u{313}~οιν", "ᾰ\u{313}~οις";
        dative_feminine: "ᾰ\u{313}~ῳ", "ᾰ\u{313}~οιν", "ᾰ\u{313}~οις";
        dative_neuter: "ᾰ\u{313}~ῳ", "ᾰ\u{313}~οιν", "ᾰ\u{313}~οις";
        accusative_masculine: "ᾰ\u{313}~ον", "ᾰ\u{313}~ω", "ᾰ\u{313}~ους";
        accusative_feminine: "ᾰ\u{313}~ον", "ᾰ\u{313}~ω", "ᾰ\u{313}~ους";
        accusative_neuter: "ᾰ\u{313}~ον", "ᾰ\u{313}~ω", "ᾰ\u{313}~ᾰ";
        vocative_masculine: "ᾰ\u{313}~ε", "ᾰ\u{313}~ω", "ᾰ\u{313}~οι";
        vocative_feminine: "ᾰ\u{313}~ε", "ᾰ\u{313}~ω", "ᾰ\u{313}~οι";
        vocative_neuter: "ᾰ\u{313}~ον", "ᾰ\u{313}~ω", "ᾰ\u{313}~ᾰ";
        comparative: "ᾰ\u{313}~οτερος";
        superlative: "ᾰ\u{313}~οτᾰτος";
        adverb: "ᾰ\u{313}~ως";
    ),
    // Μαυρος: 40 words (μαυρος, σῠνεργος, γεωργος, σῠνᾰπτος, ἐνεργος, ἁμαρτωλος).
    declension!("μαυρος", 40, "~ος";
        nominative_masculine: "~ος", "~ω", "~οι";
        nominative_feminine: "~ος", "~ω", "~οι";
        nominative_neuter: "~ον", "~ω", "~ᾰ";
        genitive_masculine: "~οῡ", "~οῑν", "~ων";
        genitive_feminine: "~οῡ", "~οῑν", "~ων";
        genitive_neuter: "~οῡ", "~οῑν", "~ων";
        dative_masculine: "~ῳ", "~οῑν", "~οῑς";
        dative_feminine: "~ῳ", "~οῑν", "~οῑς";
        dative_neuter: "~ῳ", "~οῑν", "~οῑς";
        accusative_masculine: "~ον", "~ω", "~ους";
        accusative_feminine: "~ον", "~ω", "~ους";
        accusative_neuter: "~ον", "~ω", "~ᾰ";
        vocative_masculine: "~ε", "~ω", "~οι";
        vocative_feminine: "~ε", "~ω", "~οι";
        vocative_neuter: "~ον", "~ω", "~ᾰ";
        comparative: "~οτερος";
        superlative: "~οτᾰτος";
        adverb: "~ως";
    ),
    // Ἐκλῠτηρῐον: 35 words (ὠν, νῑκων, τῑμων, ἀρρην, δῐαιτων, θεων).
    declension!("ἐκλῠτηρῐον", 35, "~ν";
        nominative_masculine: "~ν", "~ντε", "~ντες";
        nominative_feminine: "~σᾰ", "~σᾱ", "~σαι";
        nominative_neuter: "~ν", "~ντε", "~ντᾰ";
        genitive_masculine: "~ντος", "~ντοιν", "~ντων";
        genitive_feminine: "~σης", "~σαιν", "~σων";
        genitive_neuter: "~ντος", "~ντοιν", "~ντων";
        dative_masculine: "~ντῐ", "~ντοιν", "~σῐ|~σῐν";
        dative_feminine: "~σῃ", "~σαιν", "~σαις";
        dative_neuter: "~ντῐ", "~ντοιν", "~σῐ|~σῐν";
        accusative_masculine: "~ντᾰ", "~ντε", "~ντᾰς";
        accusative_feminine: "~σᾰν", "~σᾱ", "~σᾱς";
        accusative_neuter: "~ν", "~ντε", "~ντᾰ";
        vocative_masculine: "~ν", "~ντε", "~ντες";
        vocative_feminine: "~σᾰ", "~σᾱ", "~σαι";
        vocative_neuter: "~ν", "~ντε", "~ντᾰ";
        comparative: "~στερος";
        superlative: "~στᾰτος";
        adverb: "~ντως";
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
        comparative: "~ωτερος";
        superlative: "~ωτᾰτος";
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
    // Ἀκρῐτος: 27 words (ἀθεος, ἀπορος, ἀστρολογος, ἀγᾰμος, ἀφοβος, ἀδεσποτος).
    declension!("ἀκρῐτος", 27, "ἀ~ος";
        nominative_masculine: "ᾰ\u{313}~ος", "ᾰ\u{313}~ω", "ᾰ\u{313}~οι";
        nominative_feminine: "ᾰ\u{313}~ος", "ᾰ\u{313}~ω", "ᾰ\u{313}~οι";
        nominative_neuter: "ᾰ\u{313}~ον", "ᾰ\u{313}~ω", "ᾰ\u{313}~ᾰ";
        genitive_masculine: "ᾰ\u{313}~ου", "ᾰ\u{313}~οιν", "ᾰ\u{313}~ων";
        genitive_feminine: "ᾰ\u{313}~ου", "ᾰ\u{313}~οιν", "ᾰ\u{313}~ων";
        genitive_neuter: "ᾰ\u{313}~ου", "ᾰ\u{313}~οιν", "ᾰ\u{313}~ων";
        dative_masculine: "ᾰ\u{313}~ῳ", "ᾰ\u{313}~οιν", "ᾰ\u{313}~οις";
        dative_feminine: "ᾰ\u{313}~ῳ", "ᾰ\u{313}~οιν", "ᾰ\u{313}~οις";
        dative_neuter: "ᾰ\u{313}~ῳ", "ᾰ\u{313}~οιν", "ᾰ\u{313}~οις";
        accusative_masculine: "ᾰ\u{313}~ον", "ᾰ\u{313}~ω", "ᾰ\u{313}~ους";
        accusative_feminine: "ᾰ\u{313}~ον", "ᾰ\u{313}~ω", "ᾰ\u{313}~ους";
        accusative_neuter: "ᾰ\u{313}~ον", "ᾰ\u{313}~ω", "ᾰ\u{313}~ᾰ";
        vocative_masculine: "ᾰ\u{313}~ε", "ᾰ\u{313}~ω", "ᾰ\u{313}~οι";
        vocative_feminine: "ᾰ\u{313}~ε", "ᾰ\u{313}~ω", "ᾰ\u{313}~οι";
        vocative_neuter: "ᾰ\u{313}~ον", "ᾰ\u{313}~ω", "ᾰ\u{313}~ᾰ";
        comparative: "ᾰ\u{313}~ωτερος";
        superlative: "ᾰ\u{313}~ωτᾰτος";
        adverb: "ᾰ\u{313}~ως";
    ),
    // Ἀκρᾰτης: 26 words (ἀσθενης, ἀληθης, ἀηθης, ἀπευθης, ἀμαυρης, ἀεικης).
    declension!("ἀκρᾰτης", 26, "ἀ~ης";
        nominative_masculine: "ᾰ\u{313}~ης", "ᾰ\u{313}~εῑ", "ᾰ\u{313}~εῑς";
        nominative_feminine: "ᾰ\u{313}~ης", "ᾰ\u{313}~εῑ", "ᾰ\u{313}~εῑς";
        nominative_neuter: "ᾰ\u{313}~ες", "ᾰ\u{313}~εῑ", "ᾰ\u{313}~η";
        genitive_masculine: "ᾰ\u{313}~οῡς", "ᾰ\u{313}~οῑν", "ᾰ\u{313}~ων";
        genitive_feminine: "ᾰ\u{313}~οῡς", "ᾰ\u{313}~οῑν", "ᾰ\u{313}~ων";
        genitive_neuter: "ᾰ\u{313}~οῡς", "ᾰ\u{313}~οῑν", "ᾰ\u{313}~ων";
        dative_masculine: "ᾰ\u{313}~εῑ", "ᾰ\u{313}~οῑν", "ᾰ\u{313}~εσῐ|ᾰ\u{313}~εσῐν";
        dative_feminine: "ᾰ\u{313}~εῑ", "ᾰ\u{313}~οῑν", "ᾰ\u{313}~εσῐ|ᾰ\u{313}~εσῐν";
        dative_neuter: "ᾰ\u{313}~εῑ", "ᾰ\u{313}~οῑν", "ᾰ\u{313}~εσῐ|ᾰ\u{313}~εσῐν";
        accusative_masculine: "ᾰ\u{313}~η", "ᾰ\u{313}~εῑ", "ᾰ\u{313}~εῑς";
        accusative_feminine: "ᾰ\u{313}~η", "ᾰ\u{313}~εῑ", "ᾰ\u{313}~εῑς";
        accusative_neuter: "ᾰ\u{313}~ες", "ᾰ\u{313}~εῑ", "ᾰ\u{313}~η";
        vocative_masculine: "ᾰ\u{313}~ες", "ᾰ\u{313}~εῑ", "ᾰ\u{313}~εῑς";
        vocative_feminine: "ᾰ\u{313}~ες", "ᾰ\u{313}~εῑ", "ᾰ\u{313}~εῑς";
        vocative_neuter: "ᾰ\u{313}~ες", "ᾰ\u{313}~εῑ", "ᾰ\u{313}~η";
        comparative: "ᾰ\u{313}~εστερος";
        superlative: "ᾰ\u{313}~εστᾰτος";
        adverb: "ᾰ\u{313}~ως";
    ),
    // Ἀλλος: 25 words (ἀλλος, ἀμβροτος, ἀνθρωπῐνος, ἀγγελλομενος, ἀγγελεομενος, ἀγγελθησομενος).
    declension!("ἀλλος", 25, "ἀ~ος";
        nominative_masculine: "ᾰ\u{313}~ος", "ᾰ\u{313}~ω", "ᾰ\u{313}~οι";
        nominative_feminine: "ᾰ\u{313}~η", "ᾰ\u{313}~ᾱ", "ᾰ\u{313}~αι";
        nominative_neuter: "ᾰ\u{313}~ον", "ᾰ\u{313}~ω", "ᾰ\u{313}~ᾰ";
        genitive_masculine: "ᾰ\u{313}~ου", "ᾰ\u{313}~οιν", "ᾰ\u{313}~ων";
        genitive_feminine: "ᾰ\u{313}~ης", "ᾰ\u{313}~αιν", "ᾰ\u{313}~ων";
        genitive_neuter: "ᾰ\u{313}~ου", "ᾰ\u{313}~οιν", "ᾰ\u{313}~ων";
        dative_masculine: "ᾰ\u{313}~ῳ", "ᾰ\u{313}~οιν", "ᾰ\u{313}~οις";
        dative_feminine: "ᾰ\u{313}~ῃ", "ᾰ\u{313}~αιν", "ᾰ\u{313}~αις";
        dative_neuter: "ᾰ\u{313}~ῳ", "ᾰ\u{313}~οιν", "ᾰ\u{313}~οις";
        accusative_masculine: "ᾰ\u{313}~ον", "ᾰ\u{313}~ω", "ᾰ\u{313}~ους";
        accusative_feminine: "ᾰ\u{313}~ην", "ᾰ\u{313}~ᾱ", "ᾰ\u{313}~ᾱς";
        accusative_neuter: "ᾰ\u{313}~ον", "ᾰ\u{313}~ω", "ᾰ\u{313}~ᾰ";
        vocative_masculine: "ᾰ\u{313}~ε", "ᾰ\u{313}~ω", "ᾰ\u{313}~οι";
        vocative_feminine: "ᾰ\u{313}~η", "ᾰ\u{313}~ᾱ", "ᾰ\u{313}~αι";
        vocative_neuter: "ᾰ\u{313}~ον", "ᾰ\u{313}~ω", "ᾰ\u{313}~ᾰ";
        comparative: "ᾰ\u{313}~ωτερος";
        superlative: "ᾰ\u{313}~ωτᾰτος";
        adverb: "ᾰ\u{313}~ως";
    ),
    // Φωνηεις: 25 words (χᾰρῐεις, φωνηεις, αἱμᾰτοεις, ῥοδοεις, ἰχθυοεις, φθογγηεις).
    declension!("φωνηεις", 25, "~ις";
        nominative_masculine: "~ις", "~ντε", "~ντες";
        nominative_feminine: "~σσᾰ", "~σσᾱ", "~σσαι";
        nominative_neuter: "~ν", "~ντε", "~ντᾰ";
        genitive_masculine: "~ντος", "~ντοιν", "~ντων";
        genitive_feminine: "~σσης", "~σσαιν", "~σσων";
        genitive_neuter: "~ντος", "~ντοιν", "~ντων";
        dative_masculine: "~ντῐ", "~ντοιν", "~ισῐ|~ισῐν";
        dative_feminine: "~σσῃ", "~σσαιν", "~σσαις";
        dative_neuter: "~ντῐ", "~ντοιν", "~ισῐ|~ισῐν";
        accusative_masculine: "~ντᾰ", "~ντε", "~ντᾰς";
        accusative_feminine: "~σσᾰν", "~σσᾱ", "~σσᾱς";
        accusative_neuter: "~ν", "~ντε", "~ντᾰ";
        vocative_masculine: "~ν", "~ντε", "~ντες";
        vocative_feminine: "~σσᾰ", "~σσᾱ", "~σσαι";
        vocative_neuter: "~ν", "~ντε", "~ντᾰ";
        comparative: "~στερος";
        superlative: "~στᾰτος";
        adverb: "~ντως";
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
        comparative: "ᾰ\u{313}~ᾰ~οτερος";
        superlative: "ᾰ\u{313}~ᾰ~οτᾰτος";
        adverb: "~ι~οντως";
    ),
    // Ἀκαρπος: 19 words (ἀναπολογητος, ἀκαρπος, ἀναπαιστος, ἀνανδρος, ἀγαθοποιος, ἀγναπτος).
    declension!("ἀκαρπος", 19, "ἀ~α~ος";
        nominative_masculine: "ᾰ\u{313}~ᾰ~ος", "ᾰ\u{313}~ᾰ~ω", "ᾰ\u{313}~ᾰ~οι";
        nominative_feminine: "ᾰ\u{313}~ᾰ~ος", "ᾰ\u{313}~ᾰ~ω", "ᾰ\u{313}~ᾰ~οι";
        nominative_neuter: "ᾰ\u{313}~ᾰ~ον", "ᾰ\u{313}~ᾰ~ω", "ᾰ\u{313}~ᾰ~ᾰ";
        genitive_masculine: "ᾰ\u{313}~ᾰ~ου", "ᾰ\u{313}~ᾰ~οιν", "ᾰ\u{313}~ᾰ~ων";
        genitive_feminine: "ᾰ\u{313}~ᾰ~ου", "ᾰ\u{313}~ᾰ~οιν", "ᾰ\u{313}~ᾰ~ων";
        genitive_neuter: "ᾰ\u{313}~ᾰ~ου", "ᾰ\u{313}~ᾰ~οιν", "ᾰ\u{313}~ᾰ~ων";
        dative_masculine: "ᾰ\u{313}~ᾰ~ῳ", "ᾰ\u{313}~ᾰ~οιν", "ᾰ\u{313}~ᾰ~οις";
        dative_feminine: "ᾰ\u{313}~ᾰ~ῳ", "ᾰ\u{313}~ᾰ~οιν", "ᾰ\u{313}~ᾰ~οις";
        dative_neuter: "ᾰ\u{313}~ᾰ~ῳ", "ᾰ\u{313}~ᾰ~οιν", "ᾰ\u{313}~ᾰ~οις";
        accusative_masculine: "ᾰ\u{313}~ᾰ~ον", "ᾰ\u{313}~ᾰ~ω", "ᾰ\u{313}~ᾰ~ους";
        accusative_feminine: "ᾰ\u{313}~ᾰ~ον", "ᾰ\u{313}~ᾰ~ω", "ᾰ\u{313}~ᾰ~ους";
        accusative_neuter: "ᾰ\u{313}~ᾰ~ον", "ᾰ\u{313}~ᾰ~ω", "ᾰ\u{313}~ᾰ~ᾰ";
        vocative_masculine: "ᾰ\u{313}~ᾰ~ε", "ᾰ\u{313}~ᾰ~ω", "ᾰ\u{313}~ᾰ~οι";
        vocative_feminine: "ᾰ\u{313}~ᾰ~ε", "ᾰ\u{313}~ᾰ~ω", "ᾰ\u{313}~ᾰ~οι";
        vocative_neuter: "ᾰ\u{313}~ᾰ~ον", "ᾰ\u{313}~ᾰ~ω", "ᾰ\u{313}~ᾰ~ᾰ";
        comparative: "ᾰ\u{313}~ᾰ~οτερος";
        superlative: "ᾰ\u{313}~ᾰ~οτᾰτος";
        adverb: "ᾰ\u{313}~ᾰ~ως";
    ),
    // Τρῐπους: 18 words (τετρᾰπους, ὀκτωπους, πολυπους, τρῐπους, ἀπους, ἰθυς).
    declension!("τρῐπους", 18, "~υς";
        nominative_masculine: "~υς", "~δε", "~δες";
        nominative_feminine: "~υς", "~δε", "~δες";
        nominative_neuter: "~υν", "~δε", "~δᾰ";
        genitive_masculine: "~δος", "~δοιν", "~δων";
        genitive_feminine: "~δος", "~δοιν", "~δων";
        genitive_neuter: "~δος", "~δοιν", "~δων";
        dative_masculine: "~δῐ", "~δοιν", "~σῐ";
        dative_feminine: "~δῐ", "~δοιν", "~σῐ";
        dative_neuter: "~δῐ", "~δοιν", "~σῐ";
        accusative_masculine: "~δᾰ", "~δε", "~δᾰς";
        accusative_feminine: "~δᾰ", "~δε", "~δᾰς";
        accusative_neuter: "~υν", "~δε", "~δᾰ";
        vocative_masculine: "~υς", "~δε", "~δες";
        vocative_feminine: "~υς", "~δε", "~δες";
        vocative_neuter: "~υν", "~δε", "~δᾰ";
        comparative: "~ωτερος";
        superlative: "~ωτᾰτος";
        adverb: "~δως";
    ),
    // Ἡσσων: 18 words (βρᾰχῑων, μᾰσσων, ἐλᾰσσων, κερδιων, ἐλαττων, θᾱττων).
    declension!("ἡσσων", 18, "~ων";
        nominative_masculine: "~ων", "~ονε", "~ονες|~ους";
        nominative_feminine: "~ων", "~ονε", "~ονες|~ους";
        nominative_neuter: "~ον", "~ονε", "~ονᾰ|~ω";
        genitive_masculine: "~ονος", "~ονοιν", "~ονων";
        genitive_feminine: "~ονος", "~ονοιν", "~ονων";
        genitive_neuter: "~ονος", "~ονοιν", "~ονων";
        dative_masculine: "~ονῐ", "~ονοιν", "~οσῐ|~οσῐν";
        dative_feminine: "~ονῐ", "~ονοιν", "~οσῐ|~οσῐν";
        dative_neuter: "~ονῐ", "~ονοιν", "~οσῐ|~οσῐν";
        accusative_masculine: "~ονᾰ|~ω", "~ονε", "~ονᾰς|~ους";
        accusative_feminine: "~ονᾰ|~ω", "~ονε", "~ονᾰς|~ους";
        accusative_neuter: "~ον", "~ονε", "~ονᾰ|~ω";
        vocative_masculine: "~ον", "~ονε", "~ονες|~ους";
        vocative_feminine: "~ον", "~ονε", "~ονες|~ους";
        vocative_neuter: "~ον", "~ονε", "~ονᾰ|~ω";
        comparative: "~ονεστερος";
        superlative: "~στος";
        adverb: "~ον";
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
        comparative: "~ωτερος";
        superlative: "~ωτᾰτος";
        adverb: "~υοντως";
    ),
    // Ἀπατωρ: 17 words (τοιοσδε, τοσοσδε, ἀπατωρ, μεγᾰλητωρ, γενομενᾰ, ὑψηνωρ).
    declension!("ἀπατωρ", 17, "~";
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
        comparative: "~ωτερος";
        superlative: "~ωτᾰτος";
        adverb: "~ως";
    ),
    // Ἀγων: 16 words (ἀρχων, ἀγων, ἀγγελλων, ἀγγελεων, ἀκεων, ἀκουων).
    declension!("ἀγων", 16, "ἀ~ων";
        nominative_masculine: "ᾰ\u{313}~ων", "ᾰ\u{313}~οντε", "ᾰ\u{313}~οντες";
        nominative_feminine: "ᾰ\u{313}~ουσᾰ", "ᾰ\u{313}~ουσᾱ", "ᾰ\u{313}~ουσαι";
        nominative_neuter: "ᾰ\u{313}~ον", "ᾰ\u{313}~οντε", "ᾰ\u{313}~οντᾰ";
        genitive_masculine: "ᾰ\u{313}~οντος", "ᾰ\u{313}~οντοιν", "ᾰ\u{313}~οντων";
        genitive_feminine: "ᾰ\u{313}~ουσης", "ᾰ\u{313}~ουσαιν", "ᾰ\u{313}~ουσων";
        genitive_neuter: "ᾰ\u{313}~οντος", "ᾰ\u{313}~οντοιν", "ᾰ\u{313}~οντων";
        dative_masculine: "ᾰ\u{313}~οντῐ", "ᾰ\u{313}~οντοιν", "ᾰ\u{313}~ουσῐ|ᾰ\u{313}~ουσῐν";
        dative_feminine: "ᾰ\u{313}~ουσῃ", "ᾰ\u{313}~ουσαιν", "ᾰ\u{313}~ουσαις";
        dative_neuter: "ᾰ\u{313}~οντῐ", "ᾰ\u{313}~οντοιν", "ᾰ\u{313}~ουσῐ|ᾰ\u{313}~ουσῐν";
        accusative_masculine: "ᾰ\u{313}~οντᾰ", "ᾰ\u{313}~οντε", "ᾰ\u{313}~οντᾰς";
        accusative_feminine: "ᾰ\u{313}~ουσᾰν", "ᾰ\u{313}~ουσᾱ", "ᾰ\u{313}~ουσᾱς";
        accusative_neuter: "ᾰ\u{313}~ον", "ᾰ\u{313}~οντε", "ᾰ\u{313}~οντᾰ";
        vocative_masculine: "ᾰ\u{313}~ων", "ᾰ\u{313}~οντε", "ᾰ\u{313}~οντες";
        vocative_feminine: "ᾰ\u{313}~ουσᾰ", "ᾰ\u{313}~ουσᾱ", "ᾰ\u{313}~ουσαι";
        vocative_neuter: "ᾰ\u{313}~ον", "ᾰ\u{313}~οντε", "ᾰ\u{313}~οντᾰ";
        comparative: "ᾰ\u{313}~οτερος";
        superlative: "ᾰ\u{313}~οτᾰτος";
        adverb: "ᾰ\u{313}~οντως";
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
        comparative: "ᾰ\u{313}~ᾰ~οτερος";
        superlative: "ᾰ\u{313}~ᾰ~οτᾰτος";
        adverb: "~ι~ᾰντως";
    ),
    // Ἀβατος: 13 words (ἀγραφος, ἀκεφαλος, ἀδηφαγος, ἀγναφος, ἀνωμαλος, ἀναλογος).
    declension!("ἀβατος", 13, "ἀ~α~ος";
        nominative_masculine: "ᾰ\u{313}~ᾰ~ος", "ᾰ\u{313}~ᾰ~ω", "ᾰ\u{313}~ᾰ~οι";
        nominative_feminine: "ᾰ\u{313}~ᾰ~ος", "ᾰ\u{313}~ᾰ~ω", "ᾰ\u{313}~ᾰ~οι";
        nominative_neuter: "ᾰ\u{313}~ᾰ~ον", "ᾰ\u{313}~ᾰ~ω", "ᾰ\u{313}~ᾰ~ᾰ";
        genitive_masculine: "ᾰ\u{313}~ᾰ~ου", "ᾰ\u{313}~ᾰ~οιν", "ᾰ\u{313}~ᾰ~ων";
        genitive_feminine: "ᾰ\u{313}~ᾰ~ου", "ᾰ\u{313}~ᾰ~οιν", "ᾰ\u{313}~ᾰ~ων";
        genitive_neuter: "ᾰ\u{313}~ᾰ~ου", "ᾰ\u{313}~ᾰ~οιν", "ᾰ\u{313}~ᾰ~ων";
        dative_masculine: "ᾰ\u{313}~ᾰ~ῳ", "ᾰ\u{313}~ᾰ~οιν", "ᾰ\u{313}~ᾰ~οις";
        dative_feminine: "ᾰ\u{313}~ᾰ~ῳ", "ᾰ\u{313}~ᾰ~οιν", "ᾰ\u{313}~ᾰ~οις";
        dative_neuter: "ᾰ\u{313}~ᾰ~ῳ", "ᾰ\u{313}~ᾰ~οιν", "ᾰ\u{313}~ᾰ~οις";
        accusative_masculine: "ᾰ\u{313}~ᾰ~ον", "ᾰ\u{313}~ᾰ~ω", "ᾰ\u{313}~ᾰ~ους";
        accusative_feminine: "ᾰ\u{313}~ᾰ~ον", "ᾰ\u{313}~ᾰ~ω", "ᾰ\u{313}~ᾰ~ους";
        accusative_neuter: "ᾰ\u{313}~ᾰ~ον", "ᾰ\u{313}~ᾰ~ω", "ᾰ\u{313}~ᾰ~ᾰ";
        vocative_masculine: "ᾰ\u{313}~ᾰ~ε", "ᾰ\u{313}~ᾰ~ω", "ᾰ\u{313}~ᾰ~οι";
        vocative_feminine: "ᾰ\u{313}~ᾰ~ε", "ᾰ\u{313}~ᾰ~ω", "ᾰ\u{313}~ᾰ~οι";
        vocative_neuter: "ᾰ\u{313}~ᾰ~ον", "ᾰ\u{313}~ᾰ~ω", "ᾰ\u{313}~ᾰ~ᾰ";
        comparative: "ᾰ\u{313}~ᾰ~ωτερος";
        superlative: "ᾰ\u{313}~ᾰ~ωτᾰτος";
        adverb: "ᾰ\u{313}~ᾰ~ως";
    ),
    // Ὑπαιθρος: 13 words (ὑπογειος, ὑπερῳος, ὑποσπονδος, ὑπηκοος, ὑμενοπτερος, ὑψηλοκρημνος).
    declension!("ὑπαιθρος", 13, "ὑ~ος";
        nominative_masculine: "ῠ\u{314}~ος", "ῠ\u{314}~ω", "ῠ\u{314}~οι";
        nominative_feminine: "ῠ\u{314}~ος", "ῠ\u{314}~ω", "ῠ\u{314}~οι";
        nominative_neuter: "ῠ\u{314}~ον", "ῠ\u{314}~ω", "ῠ\u{314}~ᾰ";
        genitive_masculine: "ῠ\u{314}~ου", "ῠ\u{314}~οιν", "ῠ\u{314}~ων";
        genitive_feminine: "ῠ\u{314}~ου", "ῠ\u{314}~οιν", "ῠ\u{314}~ων";
        genitive_neuter: "ῠ\u{314}~ου", "ῠ\u{314}~οιν", "ῠ\u{314}~ων";
        dative_masculine: "ῠ\u{314}~ῳ", "ῠ\u{314}~οιν", "ῠ\u{314}~οις";
        dative_feminine: "ῠ\u{314}~ῳ", "ῠ\u{314}~οιν", "ῠ\u{314}~οις";
        dative_neuter: "ῠ\u{314}~ῳ", "ῠ\u{314}~οιν", "ῠ\u{314}~οις";
        accusative_masculine: "ῠ\u{314}~ον", "ῠ\u{314}~ω", "ῠ\u{314}~ους";
        accusative_feminine: "ῠ\u{314}~ον", "ῠ\u{314}~ω", "ῠ\u{314}~ους";
        accusative_neuter: "ῠ\u{314}~ον", "ῠ\u{314}~ω", "ῠ\u{314}~ᾰ";
        vocative_masculine: "ῠ\u{314}~ε", "ῠ\u{314}~ω", "ῠ\u{314}~οι";
        vocative_feminine: "ῠ\u{314}~ε", "ῠ\u{314}~ω", "ῠ\u{314}~οι";
        vocative_neuter: "ῠ\u{314}~ον", "ῠ\u{314}~ω", "ῠ\u{314}~ᾰ";
        comparative: "ῠ\u{314}~οτερος";
        superlative: "ῠ\u{314}~οτᾰτος";
        adverb: "ῠ\u{314}~ως";
    ),
    // Ἀδιψος: 12 words (ἀδικος, ἀντιλογος, ἀξιολογος, ἀνισος, ἀμφιβολος, ἀκλιτος).
    declension!("ἀδιψος", 12, "ἀ~ι~ος";
        nominative_masculine: "ᾰ\u{313}~ῐ~ος", "ᾰ\u{313}~ῐ~ω", "ᾰ\u{313}~ῐ~οι";
        nominative_feminine: "ᾰ\u{313}~ῐ~ος", "ᾰ\u{313}~ῐ~ω", "ᾰ\u{313}~ῐ~οι";
        nominative_neuter: "ᾰ\u{313}~ῐ~ον", "ᾰ\u{313}~ῐ~ω", "ᾰ\u{313}~ῐ~ᾰ";
        genitive_masculine: "ᾰ\u{313}~ῐ~ου", "ᾰ\u{313}~ῐ~οιν", "ᾰ\u{313}~ῐ~ων";
        genitive_feminine: "ᾰ\u{313}~ῐ~ου", "ᾰ\u{313}~ῐ~οιν", "ᾰ\u{313}~ῐ~ων";
        genitive_neuter: "ᾰ\u{313}~ῐ~ου", "ᾰ\u{313}~ῐ~οιν", "ᾰ\u{313}~ῐ~ων";
        dative_masculine: "ᾰ\u{313}~ῐ~ῳ", "ᾰ\u{313}~ῐ~οιν", "ᾰ\u{313}~ῐ~οις";
        dative_feminine: "ᾰ\u{313}~ῐ~ῳ", "ᾰ\u{313}~ῐ~οιν", "ᾰ\u{313}~ῐ~οις";
        dative_neuter: "ᾰ\u{313}~ῐ~ῳ", "ᾰ\u{313}~ῐ~οιν", "ᾰ\u{313}~ῐ~οις";
        accusative_masculine: "ᾰ\u{313}~ῐ~ον", "ᾰ\u{313}~ῐ~ω", "ᾰ\u{313}~ῐ~ους";
        accusative_feminine: "ᾰ\u{313}~ῐ~ον", "ᾰ\u{313}~ῐ~ω", "ᾰ\u{313}~ῐ~ους";
        accusative_neuter: "ᾰ\u{313}~ῐ~ον", "ᾰ\u{313}~ῐ~ω", "ᾰ\u{313}~ῐ~ᾰ";
        vocative_masculine: "ᾰ\u{313}~ῐ~ε", "ᾰ\u{313}~ῐ~ω", "ᾰ\u{313}~ῐ~οι";
        vocative_feminine: "ᾰ\u{313}~ῐ~ε", "ᾰ\u{313}~ῐ~ω", "ᾰ\u{313}~ῐ~οι";
        vocative_neuter: "ᾰ\u{313}~ῐ~ον", "ᾰ\u{313}~ῐ~ω", "ᾰ\u{313}~ῐ~ᾰ";
        comparative: "ᾰ\u{313}~ῐ~ωτερος";
        superlative: "ᾰ\u{313}~ῐ~ωτᾰτος";
        adverb: "ᾰ\u{313}~ῐ~ως";
    ),
    // Ἀρχικος: 12 words (ἀριστερος, ἀττικος, ἀποπληκτικος, ἀμφορικος, ἀληθινος, ἀφαιρετικος).
    declension!("ἀρχικος", 12, "ἀ~ι~ος";
        nominative_masculine: "ᾰ\u{313}~ῐ~ος", "ᾰ\u{313}~ῐ~ω", "ᾰ\u{313}~ῐ~οι";
        nominative_feminine: "ᾰ\u{313}~ῐ~η", "ᾰ\u{313}~ῐ~ᾱ", "ᾰ\u{313}~ῐ~αι";
        nominative_neuter: "ᾰ\u{313}~ῐ~ον", "ᾰ\u{313}~ῐ~ω", "ᾰ\u{313}~ῐ~ᾰ";
        genitive_masculine: "ᾰ\u{313}~ῐ~οῡ", "ᾰ\u{313}~ῐ~οῑν", "ᾰ\u{313}~ῐ~ων";
        genitive_feminine: "ᾰ\u{313}~ῐ~ης", "ᾰ\u{313}~ῐ~αῑν", "ᾰ\u{313}~ῐ~ων";
        genitive_neuter: "ᾰ\u{313}~ῐ~οῡ", "ᾰ\u{313}~ῐ~οῑν", "ᾰ\u{313}~ῐ~ων";
        dative_masculine: "ᾰ\u{313}~ῐ~ῳ", "ᾰ\u{313}~ῐ~οῑν", "ᾰ\u{313}~ῐ~οῑς";
        dative_feminine: "ᾰ\u{313}~ῐ~ῃ", "ᾰ\u{313}~ῐ~αῑν", "ᾰ\u{313}~ῐ~αῑς";
        dative_neuter: "ᾰ\u{313}~ῐ~ῳ", "ᾰ\u{313}~ῐ~οῑν", "ᾰ\u{313}~ῐ~οῑς";
        accusative_masculine: "ᾰ\u{313}~ῐ~ον", "ᾰ\u{313}~ῐ~ω", "ᾰ\u{313}~ῐ~ους";
        accusative_feminine: "ᾰ\u{313}~ῐ~ην", "ᾰ\u{313}~ῐ~ᾱ", "ᾰ\u{313}~ῐ~ᾱς";
        accusative_neuter: "ᾰ\u{313}~ῐ~ον", "ᾰ\u{313}~ῐ~ω", "ᾰ\u{313}~ῐ~ᾰ";
        vocative_masculine: "ᾰ\u{313}~ῐ~ε", "ᾰ\u{313}~ῐ~ω", "ᾰ\u{313}~ῐ~οι";
        vocative_feminine: "ᾰ\u{313}~ῐ~η", "ᾰ\u{313}~ῐ~ᾱ", "ᾰ\u{313}~ῐ~αι";
        vocative_neuter: "ᾰ\u{313}~ῐ~ον", "ᾰ\u{313}~ῐ~ω", "ᾰ\u{313}~ῐ~ᾰ";
        comparative: "ᾰ\u{313}~ῐ~ωτερος";
        superlative: "ᾰ\u{313}~ῐ~ωτᾰτος";
        adverb: "ᾰ\u{313}~ῐ~ως";
    ),
    // Ἀκροτερος: 12 words (ἀτολμοτερος, ἀληθεστερος, ἀτερπεστερος, ἀηδεστερος, ἀδεεστερος, ἀληπτοτερος).
    declension!("ἀκροτερος", 12, "ἀ~ος";
        nominative_masculine: "ᾰ\u{313}~ος", "ᾰ\u{313}~ω", "ᾰ\u{313}~οι";
        nominative_feminine: "ᾰ\u{313}~ᾱ", "ᾰ\u{313}~ᾱ", "ᾰ\u{313}~αι";
        nominative_neuter: "ᾰ\u{313}~ον", "ᾰ\u{313}~ω", "ᾰ\u{313}~ᾰ";
        genitive_masculine: "ᾰ\u{313}~ου", "ᾰ\u{313}~οιν", "ᾰ\u{313}~ων";
        genitive_feminine: "ᾰ\u{313}~ᾱς", "ᾰ\u{313}~αιν", "ᾰ\u{313}~ων";
        genitive_neuter: "ᾰ\u{313}~ου", "ᾰ\u{313}~οιν", "ᾰ\u{313}~ων";
        dative_masculine: "ᾰ\u{313}~ῳ", "ᾰ\u{313}~οιν", "ᾰ\u{313}~οις";
        dative_feminine: "ᾰ\u{313}~ᾳ", "ᾰ\u{313}~αιν", "ᾰ\u{313}~αις";
        dative_neuter: "ᾰ\u{313}~ῳ", "ᾰ\u{313}~οιν", "ᾰ\u{313}~οις";
        accusative_masculine: "ᾰ\u{313}~ον", "ᾰ\u{313}~ω", "ᾰ\u{313}~ους";
        accusative_feminine: "ᾰ\u{313}~ᾱν", "ᾰ\u{313}~ᾱ", "ᾰ\u{313}~ᾱς";
        accusative_neuter: "ᾰ\u{313}~ον", "ᾰ\u{313}~ω", "ᾰ\u{313}~ᾰ";
        vocative_masculine: "ᾰ\u{313}~ε", "ᾰ\u{313}~ω", "ᾰ\u{313}~οι";
        vocative_feminine: "ᾰ\u{313}~ᾱ", "ᾰ\u{313}~ᾱ", "ᾰ\u{313}~αι";
        vocative_neuter: "ᾰ\u{313}~ον", "ᾰ\u{313}~ω", "ᾰ\u{313}~ᾰ";
        comparative: "ᾰ\u{313}~οτερος";
        superlative: "ᾰ\u{313}~οτᾰτος";
        adverb: "ᾰ\u{313}~ον";
    ),
    // Ἀγνως: 11 words (ἀλεκτωρ, ἀκμων, ἀφρων, ἀγνως, ἀμεινων, ἀγηνωρ).
    declension!("ἀγνως", 11, "ἀ~ω~";
        nominative_masculine: "ᾰ\u{313}~ω~", "ᾰ\u{313}~ο~ε", "ᾰ\u{313}~ο~ες";
        nominative_feminine: "ᾰ\u{313}~ω~", "ᾰ\u{313}~ο~ε", "ᾰ\u{313}~ο~ες";
        nominative_neuter: "ᾰ\u{313}~ο~", "ᾰ\u{313}~ο~ε", "ᾰ\u{313}~ο~ᾰ";
        genitive_masculine: "ᾰ\u{313}~ο~ος", "ᾰ\u{313}~ο~οιν", "ᾰ\u{313}~ο~ων";
        genitive_feminine: "ᾰ\u{313}~ο~ος", "ᾰ\u{313}~ο~οιν", "ᾰ\u{313}~ο~ων";
        genitive_neuter: "ᾰ\u{313}~ο~ος", "ᾰ\u{313}~ο~οιν", "ᾰ\u{313}~ο~ων";
        dative_masculine: "ᾰ\u{313}~ο~ῐ", "ᾰ\u{313}~ο~οιν", "ᾰ\u{313}~ο~σῐ";
        dative_feminine: "ᾰ\u{313}~ο~ῐ", "ᾰ\u{313}~ο~οιν", "ᾰ\u{313}~ο~σῐ";
        dative_neuter: "ᾰ\u{313}~ο~ῐ", "ᾰ\u{313}~ο~οιν", "ᾰ\u{313}~ο~σῐ";
        accusative_masculine: "ᾰ\u{313}~ο~ᾰ", "ᾰ\u{313}~ο~ε", "ᾰ\u{313}~ο~ᾰς";
        accusative_feminine: "ᾰ\u{313}~ο~ᾰ", "ᾰ\u{313}~ο~ε", "ᾰ\u{313}~ο~ᾰς";
        accusative_neuter: "ᾰ\u{313}~ο~", "ᾰ\u{313}~ο~ε", "ᾰ\u{313}~ο~ᾰ";
        vocative_masculine: "ᾰ\u{313}~ο~", "ᾰ\u{313}~ο~ε", "ᾰ\u{313}~ο~ες";
        vocative_feminine: "ᾰ\u{313}~ο~", "ᾰ\u{313}~ο~ε", "ᾰ\u{313}~ο~ες";
        vocative_neuter: "ᾰ\u{313}~ο~", "ᾰ\u{313}~ο~ε", "ᾰ\u{313}~ο~ᾰ";
        comparative: "ᾰ\u{313}~ᾰ~οτερος";
        superlative: "ᾰ\u{313}~ᾰ~οτᾰτος";
        adverb: "ᾰ\u{313}~ο~ως";
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
    // Ἀγοραῑος: 10 words (ἀρχαῑος, ἀνδρεῑος, ἀργεῑος, ἀστεῑος, ἀλλοῑος, ἀθηναῑος).
    declension!("ἀγοραῑος", 10, "ἀ~ῑος";
        nominative_masculine: "ᾰ\u{313}~ῑος", "ᾰ\u{313}~ιω", "ᾰ\u{313}~ῑοι";
        nominative_feminine: "ᾰ\u{313}~ιᾱ", "ᾰ\u{313}~ιᾱ", "ᾰ\u{313}~ῑαι";
        nominative_neuter: "ᾰ\u{313}~ῑον", "ᾰ\u{313}~ιω", "ᾰ\u{313}~ῑᾰ";
        genitive_masculine: "ᾰ\u{313}~ιου", "ᾰ\u{313}~ιοιν", "ᾰ\u{313}~ιων";
        genitive_feminine: "ᾰ\u{313}~ιᾱς", "ᾰ\u{313}~ιαιν", "ᾰ\u{313}~ιων";
        genitive_neuter: "ᾰ\u{313}~ιου", "ᾰ\u{313}~ιοιν", "ᾰ\u{313}~ιων";
        dative_masculine: "ᾰ\u{313}~ιῳ", "ᾰ\u{313}~ιοιν", "ᾰ\u{313}~ιοις";
        dative_feminine: "ᾰ\u{313}~ιᾳ", "ᾰ\u{313}~ιαιν", "ᾰ\u{313}~ιαις";
        dative_neuter: "ᾰ\u{313}~ιῳ", "ᾰ\u{313}~ιοιν", "ᾰ\u{313}~ιοις";
        accusative_masculine: "ᾰ\u{313}~ῑον", "ᾰ\u{313}~ιω", "ᾰ\u{313}~ιους";
        accusative_feminine: "ᾰ\u{313}~ιᾱν", "ᾰ\u{313}~ιᾱ", "ᾰ\u{313}~ιᾱς";
        accusative_neuter: "ᾰ\u{313}~ῑον", "ᾰ\u{313}~ιω", "ᾰ\u{313}~ῑᾰ";
        vocative_masculine: "ᾰ\u{313}~ῑε", "ᾰ\u{313}~ιω", "ᾰ\u{313}~ῑοι";
        vocative_feminine: "ᾰ\u{313}~ιᾱ", "ᾰ\u{313}~ιᾱ", "ᾰ\u{313}~ῑαι";
        vocative_neuter: "ᾰ\u{313}~ῑον", "ᾰ\u{313}~ιω", "ᾰ\u{313}~ῑᾰ";
        comparative: "ᾰ\u{313}~ιοτερος";
        superlative: "ᾰ\u{313}~ιοτᾰτος";
        adverb: "ᾰ\u{313}~ιως";
    ),
    // Γαῡρος: 10 words (πᾰνοῡργος, λαῡρος, θοῡρος, γαῡρος, κᾰκοῡργος, σκηπτοῡχος).
    declension!("γαῡρος", 10, "~ῡ~ος";
        nominative_masculine: "~ῡ~ος", "~υ~ω", "~ῡ~οι";
        nominative_feminine: "~ῡ~ος", "~υ~ω", "~ῡ~οι";
        nominative_neuter: "~ῡ~ον", "~υ~ω", "~ῡ~ᾰ";
        genitive_masculine: "~υ~ου", "~υ~οιν", "~υ~ων";
        genitive_feminine: "~υ~ου", "~υ~οιν", "~υ~ων";
        genitive_neuter: "~υ~ου", "~υ~οιν", "~υ~ων";
        dative_masculine: "~υ~ῳ", "~υ~οιν", "~υ~οις";
        dative_feminine: "~υ~ῳ", "~υ~οιν", "~υ~οις";
        dative_neuter: "~υ~ῳ", "~υ~οιν", "~υ~οις";
        accusative_masculine: "~ῡ~ον", "~υ~ω", "~υ~ους";
        accusative_feminine: "~ῡ~ον", "~υ~ω", "~υ~ους";
        accusative_neuter: "~ῡ~ον", "~υ~ω", "~ῡ~ᾰ";
        vocative_masculine: "~ῡ~ε", "~υ~ω", "~ῡ~οι";
        vocative_feminine: "~ῡ~ε", "~υ~ω", "~ῡ~οι";
        vocative_neuter: "~ῡ~ον", "~υ~ω", "~ῡ~ᾰ";
        comparative: "~υ~οτερος";
        superlative: "~υ~οτᾰτος";
        adverb: "~υ~ως";
    ),
    // Πλωϊμος: 10 words (θεϊκος, τρωϊκος, πλωϊμος, ἡρωϊκος, πρωϊμος, σφωϊτερος).
    declension!("πλωϊμος", 10, "~ϊ~ος";
        nominative_masculine: "~ῐ\u{308}~ος", "~ῐ\u{308}~ω", "~ῐ\u{308}~οι";
        nominative_feminine: "~ῐ\u{308}~η", "~ῐ\u{308}~ᾱ", "~ῐ\u{308}~αι";
        nominative_neuter: "~ῐ\u{308}~ον", "~ῐ\u{308}~ω", "~ῐ\u{308}~ᾰ";
        genitive_masculine: "~ῐ\u{308}~οῡ", "~ῐ\u{308}~οῑν", "~ῐ\u{308}~ων";
        genitive_feminine: "~ῐ\u{308}~ης", "~ῐ\u{308}~αῑν", "~ῐ\u{308}~ων";
        genitive_neuter: "~ῐ\u{308}~οῡ", "~ῐ\u{308}~οῑν", "~ῐ\u{308}~ων";
        dative_masculine: "~ῐ\u{308}~ῳ", "~ῐ\u{308}~οῑν", "~ῐ\u{308}~οῑς";
        dative_feminine: "~ῐ\u{308}~ῃ", "~ῐ\u{308}~αῑν", "~ῐ\u{308}~αῑς";
        dative_neuter: "~ῐ\u{308}~ῳ", "~ῐ\u{308}~οῑν", "~ῐ\u{308}~οῑς";
        accusative_masculine: "~ῐ\u{308}~ον", "~ῐ\u{308}~ω", "~ῐ\u{308}~ους";
        accusative_feminine: "~ῐ\u{308}~ην", "~ῐ\u{308}~ᾱ", "~ῐ\u{308}~ᾱς";
        accusative_neuter: "~ῐ\u{308}~ον", "~ῐ\u{308}~ω", "~ῐ\u{308}~ᾰ";
        vocative_masculine: "~ῐ\u{308}~ε", "~ῐ\u{308}~ω", "~ῐ\u{308}~οι";
        vocative_feminine: "~ῐ\u{308}~η", "~ῐ\u{308}~ᾱ", "~ῐ\u{308}~αι";
        vocative_neuter: "~ῐ\u{308}~ον", "~ῐ\u{308}~ω", "~ῐ\u{308}~ᾰ";
        comparative: "~ῐ\u{308}~ωτερος";
        superlative: "~ῐ\u{308}~ωτᾰτος";
        adverb: "~ῐ\u{308}~ως";
    ),
    // Ὑποθετικος: 10 words (ὑψιστος, ὑποθετικος, ὑπερθετικος, ὑπνωτικος, ὑμενινος, ὑψικομος).
    declension!("ὑποθετικος", 10, "ὑ~ι~ος";
        nominative_masculine: "ῠ\u{314}~ῐ~ος", "ῠ\u{314}~ῐ~ω", "ῠ\u{314}~ῐ~οι";
        nominative_feminine: "ῠ\u{314}~ῐ~η", "ῠ\u{314}~ῐ~ᾱ", "ῠ\u{314}~ῐ~αι";
        nominative_neuter: "ῠ\u{314}~ῐ~ον", "ῠ\u{314}~ῐ~ω", "ῠ\u{314}~ῐ~ᾰ";
        genitive_masculine: "ῠ\u{314}~ῐ~οῡ", "ῠ\u{314}~ῐ~οῑν", "ῠ\u{314}~ῐ~ων";
        genitive_feminine: "ῠ\u{314}~ῐ~ης", "ῠ\u{314}~ῐ~αῑν", "ῠ\u{314}~ῐ~ων";
        genitive_neuter: "ῠ\u{314}~ῐ~οῡ", "ῠ\u{314}~ῐ~οῑν", "ῠ\u{314}~ῐ~ων";
        dative_masculine: "ῠ\u{314}~ῐ~ῳ", "ῠ\u{314}~ῐ~οῑν", "ῠ\u{314}~ῐ~οῑς";
        dative_feminine: "ῠ\u{314}~ῐ~ῃ", "ῠ\u{314}~ῐ~αῑν", "ῠ\u{314}~ῐ~αῑς";
        dative_neuter: "ῠ\u{314}~ῐ~ῳ", "ῠ\u{314}~ῐ~οῑν", "ῠ\u{314}~ῐ~οῑς";
        accusative_masculine: "ῠ\u{314}~ῐ~ον", "ῠ\u{314}~ῐ~ω", "ῠ\u{314}~ῐ~ους";
        accusative_feminine: "ῠ\u{314}~ῐ~ην", "ῠ\u{314}~ῐ~ᾱ", "ῠ\u{314}~ῐ~ᾱς";
        accusative_neuter: "ῠ\u{314}~ῐ~ον", "ῠ\u{314}~ῐ~ω", "ῠ\u{314}~ῐ~ᾰ";
        vocative_masculine: "ῠ\u{314}~ῐ~ε", "ῠ\u{314}~ῐ~ω", "ῠ\u{314}~ῐ~οι";
        vocative_feminine: "ῠ\u{314}~ῐ~η", "ῠ\u{314}~ῐ~ᾱ", "ῠ\u{314}~ῐ~αι";
        vocative_neuter: "ῠ\u{314}~ῐ~ον", "ῠ\u{314}~ῐ~ω", "ῠ\u{314}~ῐ~ᾰ";
        comparative: "ῠ\u{314}~ῐ~ωτερος";
        superlative: "ῠ\u{314}~ῐ~ωτᾰτος";
        adverb: "ῠ\u{314}~ῐ~ως";
    ),
    // Κολοβος: 9 words (πᾰθῐκος, κολοβος, ψᾰθῠρος, ἀκῐρος, πᾰρδᾰκος, ἐπῐπεδῐκος).
    declension!("κολοβος", 9, "~ος";
        nominative_masculine: "~ος", "~ω", "~οι";
        nominative_feminine: "~ος", "~ω", "~οι";
        nominative_neuter: "~ον", "~ω", "~ᾰ";
        genitive_masculine: "~οῡ", "~οῑν", "~ων";
        genitive_feminine: "~οῡ", "~οῑν", "~ων";
        genitive_neuter: "~οῡ", "~οῑν", "~ων";
        dative_masculine: "~ῳ", "~οῑν", "~οῑς";
        dative_feminine: "~ῳ", "~οῑν", "~οῑς";
        dative_neuter: "~ῳ", "~οῑν", "~οῑς";
        accusative_masculine: "~ον", "~ω", "~ους";
        accusative_feminine: "~ον", "~ω", "~ους";
        accusative_neuter: "~ον", "~ω", "~ᾰ";
        vocative_masculine: "~ε", "~ω", "~οι";
        vocative_feminine: "~ε", "~ω", "~οι";
        vocative_neuter: "~ον", "~ω", "~ᾰ";
        comparative: "~ωτερος";
        superlative: "~ωτᾰτος";
        adverb: "~ως";
    ),
    // Ἀμαθης: 9 words (ἀπαθης, ἀμαθης, ἀσφαλης, ἀφανης, ἀσαφης, ἀραχνοειδης).
    declension!("ἀμαθης", 9, "ἀ~α~ης";
        nominative_masculine: "ᾰ\u{313}~ᾰ~ης", "ᾰ\u{313}~ᾰ~εῑ", "ᾰ\u{313}~ᾰ~εῑς";
        nominative_feminine: "ᾰ\u{313}~ᾰ~ης", "ᾰ\u{313}~ᾰ~εῑ", "ᾰ\u{313}~ᾰ~εῑς";
        nominative_neuter: "ᾰ\u{313}~ᾰ~ες", "ᾰ\u{313}~ᾰ~εῑ", "ᾰ\u{313}~ᾰ~η";
        genitive_masculine: "ᾰ\u{313}~ᾰ~οῡς", "ᾰ\u{313}~ᾰ~οῑν", "ᾰ\u{313}~ᾰ~ων";
        genitive_feminine: "ᾰ\u{313}~ᾰ~οῡς", "ᾰ\u{313}~ᾰ~οῑν", "ᾰ\u{313}~ᾰ~ων";
        genitive_neuter: "ᾰ\u{313}~ᾰ~οῡς", "ᾰ\u{313}~ᾰ~οῑν", "ᾰ\u{313}~ᾰ~ων";
        dative_masculine: "ᾰ\u{313}~ᾰ~εῑ", "ᾰ\u{313}~ᾰ~οῑν", "ᾰ\u{313}~ᾰ~εσῐ|ᾰ\u{313}~ᾰ~εσῐν";
        dative_feminine: "ᾰ\u{313}~ᾰ~εῑ", "ᾰ\u{313}~ᾰ~οῑν", "ᾰ\u{313}~ᾰ~εσῐ|ᾰ\u{313}~ᾰ~εσῐν";
        dative_neuter: "ᾰ\u{313}~ᾰ~εῑ", "ᾰ\u{313}~ᾰ~οῑν", "ᾰ\u{313}~ᾰ~εσῐ|ᾰ\u{313}~ᾰ~εσῐν";
        accusative_masculine: "ᾰ\u{313}~ᾰ~η", "ᾰ\u{313}~ᾰ~εῑ", "ᾰ\u{313}~ᾰ~εῑς";
        accusative_feminine: "ᾰ\u{313}~ᾰ~η", "ᾰ\u{313}~ᾰ~εῑ", "ᾰ\u{313}~ᾰ~εῑς";
        accusative_neuter: "ᾰ\u{313}~ᾰ~ες", "ᾰ\u{313}~ᾰ~εῑ", "ᾰ\u{313}~ᾰ~η";
        vocative_masculine: "ᾰ\u{313}~ᾰ~ες", "ᾰ\u{313}~ᾰ~εῑ", "ᾰ\u{313}~ᾰ~εῑς";
        vocative_feminine: "ᾰ\u{313}~ᾰ~ες", "ᾰ\u{313}~ᾰ~εῑ", "ᾰ\u{313}~ᾰ~εῑς";
        vocative_neuter: "ᾰ\u{313}~ᾰ~ες", "ᾰ\u{313}~ᾰ~εῑ", "ᾰ\u{313}~ᾰ~η";
        comparative: "ᾰ\u{313}~ᾰ~εστερος";
        superlative: "ᾰ\u{313}~ᾰ~εστᾰτος";
        adverb: "ᾰ\u{313}~ᾰ~ως";
    ),
    // Ὑψηλος: 9 words (ὑψηλος, ὑποδεδειγμενος, ὑστεραῑος, ὑπερβολῐκος, ὑπηοῑος, ὑποβοηθουμενος).
    declension!("ὑψηλος", 9, "ὑ~ος";
        nominative_masculine: "ῠ\u{314}~ος", "ῠ\u{314}~ω", "ῠ\u{314}~οι";
        nominative_feminine: "ῠ\u{314}~η", "ῠ\u{314}~ᾱ", "ῠ\u{314}~αι";
        nominative_neuter: "ῠ\u{314}~ον", "ῠ\u{314}~ω", "ῠ\u{314}~ᾰ";
        genitive_masculine: "ῠ\u{314}~οῡ", "ῠ\u{314}~οῑν", "ῠ\u{314}~ων";
        genitive_feminine: "ῠ\u{314}~ης", "ῠ\u{314}~αῑν", "ῠ\u{314}~ων";
        genitive_neuter: "ῠ\u{314}~οῡ", "ῠ\u{314}~οῑν", "ῠ\u{314}~ων";
        dative_masculine: "ῠ\u{314}~ῳ", "ῠ\u{314}~οῑν", "ῠ\u{314}~οῑς";
        dative_feminine: "ῠ\u{314}~ῃ", "ῠ\u{314}~αῑν", "ῠ\u{314}~αῑς";
        dative_neuter: "ῠ\u{314}~ῳ", "ῠ\u{314}~οῑν", "ῠ\u{314}~οῑς";
        accusative_masculine: "ῠ\u{314}~ον", "ῠ\u{314}~ω", "ῠ\u{314}~ους";
        accusative_feminine: "ῠ\u{314}~ην", "ῠ\u{314}~ᾱ", "ῠ\u{314}~ᾱς";
        accusative_neuter: "ῠ\u{314}~ον", "ῠ\u{314}~ω", "ῠ\u{314}~ᾰ";
        vocative_masculine: "ῠ\u{314}~ε", "ῠ\u{314}~ω", "ῠ\u{314}~οι";
        vocative_feminine: "ῠ\u{314}~η", "ῠ\u{314}~ᾱ", "ῠ\u{314}~αι";
        vocative_neuter: "ῠ\u{314}~ον", "ῠ\u{314}~ω", "ῠ\u{314}~ᾰ";
        comparative: "ῠ\u{314}~οτερος";
        superlative: "ῠ\u{314}~οτᾰτος";
        adverb: "ῠ\u{314}~ως";
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
        comparative: "~ωτερος";
        superlative: "~ωτᾰτος";
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
        comparative: "ᾰ\u{313}~ᾰ~οτερος";
        superlative: "ᾰ\u{313}~ᾰ~οτᾰτος";
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
        comparative: "~υ~οτερος";
        superlative: "~υ~οτᾰτος";
        adverb: "~υ~οντως";
    ),
    // Ἀμπελοεις: 9 words (ἀγγελθεις, ἀγγελεις, ἀπολῠθεις, ἀορτηθεις, ἀμπελοεις, ἀφεις).
    declension!("ἀμπελοεις", 9, "ἀ~ις";
        nominative_masculine: "ᾰ\u{313}~ις", "ᾰ\u{313}~ντε", "ᾰ\u{313}~ντες";
        nominative_feminine: "ᾰ\u{313}~ῑσᾰ", "ᾰ\u{313}~ισᾱ", "ᾰ\u{313}~ῑσαι";
        nominative_neuter: "ᾰ\u{313}~ν", "ᾰ\u{313}~ντε", "ᾰ\u{313}~ντᾰ";
        genitive_masculine: "ᾰ\u{313}~ντος", "ᾰ\u{313}~ντοιν", "ᾰ\u{313}~ντων";
        genitive_feminine: "ᾰ\u{313}~ισης", "ᾰ\u{313}~ισαιν", "ᾰ\u{313}~ισων";
        genitive_neuter: "ᾰ\u{313}~ντος", "ᾰ\u{313}~ντοιν", "ᾰ\u{313}~ντων";
        dative_masculine: "ᾰ\u{313}~ντῐ", "ᾰ\u{313}~ντοιν", "ᾰ\u{313}~ῑσῐ|ᾰ\u{313}~ῑσῐν";
        dative_feminine: "ᾰ\u{313}~ισῃ", "ᾰ\u{313}~ισαιν", "ᾰ\u{313}~ισαις";
        dative_neuter: "ᾰ\u{313}~ντῐ", "ᾰ\u{313}~ντοιν", "ᾰ\u{313}~ῑσῐ|ᾰ\u{313}~ῑσῐν";
        accusative_masculine: "ᾰ\u{313}~ντᾰ", "ᾰ\u{313}~ντε", "ᾰ\u{313}~ντᾰς";
        accusative_feminine: "ᾰ\u{313}~ῑσᾰν", "ᾰ\u{313}~ισᾱ", "ᾰ\u{313}~ισᾱς";
        accusative_neuter: "ᾰ\u{313}~ν", "ᾰ\u{313}~ντε", "ᾰ\u{313}~ντᾰ";
        vocative_masculine: "ᾰ\u{313}~ις", "ᾰ\u{313}~ντε", "ᾰ\u{313}~ντες";
        vocative_feminine: "ᾰ\u{313}~ῑσᾰ", "ᾰ\u{313}~ισᾱ", "ᾰ\u{313}~ῑσαι";
        vocative_neuter: "ᾰ\u{313}~ν", "ᾰ\u{313}~ντε", "ᾰ\u{313}~ντᾰ";
        comparative: "~ωτερος";
        superlative: "~ωτᾰτος";
        adverb: "ᾰ\u{313}~ντως";
    ),
    // Ἀριστος: 9 words (ἀριστος, ἀτειχιστος, ἀξιουμενος, ἀρκευθινος, ἀλευρινος, ἀμμινος).
    declension!("ἀριστος", 9, "ἀ~ι~ος";
        nominative_masculine: "ᾰ\u{313}~ῐ~ος", "ᾰ\u{313}~ῐ~ω", "ᾰ\u{313}~ῐ~οι";
        nominative_feminine: "ᾰ\u{313}~ῐ~η", "ᾰ\u{313}~ῐ~ᾱ", "ᾰ\u{313}~ῐ~αι";
        nominative_neuter: "ᾰ\u{313}~ῐ~ον", "ᾰ\u{313}~ῐ~ω", "ᾰ\u{313}~ῐ~ᾰ";
        genitive_masculine: "ᾰ\u{313}~ῐ~ου", "ᾰ\u{313}~ῐ~οιν", "ᾰ\u{313}~ῐ~ων";
        genitive_feminine: "ᾰ\u{313}~ῐ~ης", "ᾰ\u{313}~ῐ~αιν", "ᾰ\u{313}~ῐ~ων";
        genitive_neuter: "ᾰ\u{313}~ῐ~ου", "ᾰ\u{313}~ῐ~οιν", "ᾰ\u{313}~ῐ~ων";
        dative_masculine: "ᾰ\u{313}~ῐ~ῳ", "ᾰ\u{313}~ῐ~οιν", "ᾰ\u{313}~ῐ~οις";
        dative_feminine: "ᾰ\u{313}~ῐ~ῃ", "ᾰ\u{313}~ῐ~αιν", "ᾰ\u{313}~ῐ~αις";
        dative_neuter: "ᾰ\u{313}~ῐ~ῳ", "ᾰ\u{313}~ῐ~οιν", "ᾰ\u{313}~ῐ~οις";
        accusative_masculine: "ᾰ\u{313}~ῐ~ον", "ᾰ\u{313}~ῐ~ω", "ᾰ\u{313}~ῐ~ους";
        accusative_feminine: "ᾰ\u{313}~ῐ~ην", "ᾰ\u{313}~ῐ~ᾱ", "ᾰ\u{313}~ῐ~ᾱς";
        accusative_neuter: "ᾰ\u{313}~ῐ~ον", "ᾰ\u{313}~ῐ~ω", "ᾰ\u{313}~ῐ~ᾰ";
        vocative_masculine: "ᾰ\u{313}~ῐ~ε", "ᾰ\u{313}~ῐ~ω", "ᾰ\u{313}~ῐ~οι";
        vocative_feminine: "ᾰ\u{313}~ῐ~η", "ᾰ\u{313}~ῐ~ᾱ", "ᾰ\u{313}~ῐ~αι";
        vocative_neuter: "ᾰ\u{313}~ῐ~ον", "ᾰ\u{313}~ῐ~ω", "ᾰ\u{313}~ῐ~ᾰ";
        comparative: "ᾰ\u{313}~ῐ~ωτερος";
        superlative: "ᾰ\u{313}~ῐ~ωτᾰτος";
        adverb: "ᾰ\u{313}~ῐ~ως";
    ),
    // Ἀποδεικτῐκος: 9 words (ἀρκετος, ἀνᾰλῠτῐκος, ἀχαιῐκος, ἀρσενῐκος, ἀποδεικτῐκος, ἀποφᾰτῐκος).
    declension!("ἀποδεικτῐκος", 9, "ἀ~ος";
        nominative_masculine: "ᾰ\u{313}~ος", "ᾰ\u{313}~ω", "ᾰ\u{313}~οι";
        nominative_feminine: "ᾰ\u{313}~η", "ᾰ\u{313}~ᾱ", "ᾰ\u{313}~αι";
        nominative_neuter: "ᾰ\u{313}~ον", "ᾰ\u{313}~ω", "ᾰ\u{313}~ᾰ";
        genitive_masculine: "ᾰ\u{313}~οῡ", "ᾰ\u{313}~οῑν", "ᾰ\u{313}~ων";
        genitive_feminine: "ᾰ\u{313}~ης", "ᾰ\u{313}~αῑν", "ᾰ\u{313}~ων";
        genitive_neuter: "ᾰ\u{313}~οῡ", "ᾰ\u{313}~οῑν", "ᾰ\u{313}~ων";
        dative_masculine: "ᾰ\u{313}~ῳ", "ᾰ\u{313}~οῑν", "ᾰ\u{313}~οῑς";
        dative_feminine: "ᾰ\u{313}~ῃ", "ᾰ\u{313}~αῑν", "ᾰ\u{313}~αῑς";
        dative_neuter: "ᾰ\u{313}~ῳ", "ᾰ\u{313}~οῑν", "ᾰ\u{313}~οῑς";
        accusative_masculine: "ᾰ\u{313}~ον", "ᾰ\u{313}~ω", "ᾰ\u{313}~ους";
        accusative_feminine: "ᾰ\u{313}~ην", "ᾰ\u{313}~ᾱ", "ᾰ\u{313}~ᾱς";
        accusative_neuter: "ᾰ\u{313}~ον", "ᾰ\u{313}~ω", "ᾰ\u{313}~ᾰ";
        vocative_masculine: "ᾰ\u{313}~ε", "ᾰ\u{313}~ω", "ᾰ\u{313}~οι";
        vocative_feminine: "ᾰ\u{313}~η", "ᾰ\u{313}~ᾱ", "ᾰ\u{313}~αι";
        vocative_neuter: "ᾰ\u{313}~ον", "ᾰ\u{313}~ω", "ᾰ\u{313}~ᾰ";
        comparative: "ᾰ\u{313}~ωτερος";
        superlative: "ᾰ\u{313}~ωτᾰτος";
        adverb: "ᾰ\u{313}~ως";
    ),
    // Ἀντιπαλος: 9 words (ἀντιπαλος, ἀμφικεφαλος, ἀκλιτωτατος, ἀλλοτριωτατος, ἀγριωτατος, ἀξιωτατος).
    declension!("ἀντιπαλος", 9, "ἀ~ι~α~ος";
        nominative_masculine: "ᾰ\u{313}~ῐ~ᾰ~ος", "ᾰ\u{313}~ῐ~ᾰ~ω", "ᾰ\u{313}~ῐ~ᾰ~οι";
        nominative_feminine: "ᾰ\u{313}~ῐ~ᾰ~η", "ᾰ\u{313}~ῐ~ᾰ~ᾱ", "ᾰ\u{313}~ῐ~ᾰ~αι";
        nominative_neuter: "ᾰ\u{313}~ῐ~ᾰ~ον", "ᾰ\u{313}~ῐ~ᾰ~ω", "ᾰ\u{313}~ῐ~ᾰ~ᾰ";
        genitive_masculine: "ᾰ\u{313}~ῐ~ᾰ~ου", "ᾰ\u{313}~ῐ~ᾰ~οιν", "ᾰ\u{313}~ῐ~ᾰ~ων";
        genitive_feminine: "ᾰ\u{313}~ῐ~ᾰ~ης", "ᾰ\u{313}~ῐ~ᾰ~αιν", "ᾰ\u{313}~ῐ~ᾰ~ων";
        genitive_neuter: "ᾰ\u{313}~ῐ~ᾰ~ου", "ᾰ\u{313}~ῐ~ᾰ~οιν", "ᾰ\u{313}~ῐ~ᾰ~ων";
        dative_masculine: "ᾰ\u{313}~ῐ~ᾰ~ῳ", "ᾰ\u{313}~ῐ~ᾰ~οιν", "ᾰ\u{313}~ῐ~ᾰ~οις";
        dative_feminine: "ᾰ\u{313}~ῐ~ᾰ~ῃ", "ᾰ\u{313}~ῐ~ᾰ~αιν", "ᾰ\u{313}~ῐ~ᾰ~αις";
        dative_neuter: "ᾰ\u{313}~ῐ~ᾰ~ῳ", "ᾰ\u{313}~ῐ~ᾰ~οιν", "ᾰ\u{313}~ῐ~ᾰ~οις";
        accusative_masculine: "ᾰ\u{313}~ῐ~ᾰ~ον", "ᾰ\u{313}~ῐ~ᾰ~ω", "ᾰ\u{313}~ῐ~ᾰ~ους";
        accusative_feminine: "ᾰ\u{313}~ῐ~ᾰ~ην", "ᾰ\u{313}~ῐ~ᾰ~ᾱ", "ᾰ\u{313}~ῐ~ᾰ~ᾱς";
        accusative_neuter: "ᾰ\u{313}~ῐ~ᾰ~ον", "ᾰ\u{313}~ῐ~ᾰ~ω", "ᾰ\u{313}~ῐ~ᾰ~ᾰ";
        vocative_masculine: "ᾰ\u{313}~ῐ~ᾰ~ε", "ᾰ\u{313}~ῐ~ᾰ~ω", "ᾰ\u{313}~ῐ~ᾰ~οι";
        vocative_feminine: "ᾰ\u{313}~ῐ~ᾰ~η", "ᾰ\u{313}~ῐ~ᾰ~ᾱ", "ᾰ\u{313}~ῐ~ᾰ~αι";
        vocative_neuter: "ᾰ\u{313}~ῐ~ᾰ~ον", "ᾰ\u{313}~ῐ~ᾰ~ω", "ᾰ\u{313}~ῐ~ᾰ~ᾰ";
        comparative: "ᾰ\u{313}~ᾰ~ᾰ~ωτερος";
        superlative: "ᾰ\u{313}~ᾰ~ᾰ~ωτᾰτος";
        adverb: "ᾰ\u{313}~ῐ~ᾰ~ᾰ";
    ),
    // Ἀσυνδετος: 8 words (ἀνδρογυνος, ἀσυνδετος, ἀργυρεος, ἀτυζομενος, ἀστυνομος, ἀποκᾰλυπτῐκος).
    declension!("ἀσυνδετος", 8, "ἀ~υ~ος";
        nominative_masculine: "ᾰ\u{313}~ῠ~ος", "ᾰ\u{313}~ῠ~ω", "ᾰ\u{313}~ῠ~οι";
        nominative_feminine: "ᾰ\u{313}~ῠ~ος", "ᾰ\u{313}~ῠ~ω", "ᾰ\u{313}~ῠ~οι";
        nominative_neuter: "ᾰ\u{313}~ῠ~ον", "ᾰ\u{313}~ῠ~ω", "ᾰ\u{313}~ῠ~ᾰ";
        genitive_masculine: "ᾰ\u{313}~ῠ~ου", "ᾰ\u{313}~ῠ~οιν", "ᾰ\u{313}~ῠ~ων";
        genitive_feminine: "ᾰ\u{313}~ῠ~ου", "ᾰ\u{313}~ῠ~οιν", "ᾰ\u{313}~ῠ~ων";
        genitive_neuter: "ᾰ\u{313}~ῠ~ου", "ᾰ\u{313}~ῠ~οιν", "ᾰ\u{313}~ῠ~ων";
        dative_masculine: "ᾰ\u{313}~ῠ~ῳ", "ᾰ\u{313}~ῠ~οιν", "ᾰ\u{313}~ῠ~οις";
        dative_feminine: "ᾰ\u{313}~ῠ~ῳ", "ᾰ\u{313}~ῠ~οιν", "ᾰ\u{313}~ῠ~οις";
        dative_neuter: "ᾰ\u{313}~ῠ~ῳ", "ᾰ\u{313}~ῠ~οιν", "ᾰ\u{313}~ῠ~οις";
        accusative_masculine: "ᾰ\u{313}~ῠ~ον", "ᾰ\u{313}~ῠ~ω", "ᾰ\u{313}~ῠ~ους";
        accusative_feminine: "ᾰ\u{313}~ῠ~ον", "ᾰ\u{313}~ῠ~ω", "ᾰ\u{313}~ῠ~ους";
        accusative_neuter: "ᾰ\u{313}~ῠ~ον", "ᾰ\u{313}~ῠ~ω", "ᾰ\u{313}~ῠ~ᾰ";
        vocative_masculine: "ᾰ\u{313}~ῠ~ε", "ᾰ\u{313}~ῠ~ω", "ᾰ\u{313}~ῠ~οι";
        vocative_feminine: "ᾰ\u{313}~ῠ~ε", "ᾰ\u{313}~ῠ~ω", "ᾰ\u{313}~ῠ~οι";
        vocative_neuter: "ᾰ\u{313}~ῠ~ον", "ᾰ\u{313}~ῠ~ω", "ᾰ\u{313}~ῠ~ᾰ";
        comparative: "ᾰ\u{313}~ῠ~ωτερος";
        superlative: "ᾰ\u{313}~ῠ~ωτᾰτος";
        adverb: "ᾰ\u{313}~ῠ~ως";
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
        comparative: "~ιοτερος";
        superlative: "~ιοτᾰτος";
        adverb: "~ιοντως";
    ),
    // Ἀξιωτερος: 8 words (ἀκλιτωτερος, ἀπιστοτερος, ἀγριωτερος, ἀξιωτερος, ἀθλιωτερος, ἀλκιμωτερος).
    declension!("ἀξιωτερος", 8, "ἀ~ι~ος";
        nominative_masculine: "ᾰ\u{313}~ῐ~ος", "ᾰ\u{313}~ῐ~ω", "ᾰ\u{313}~ῐ~οι";
        nominative_feminine: "ᾰ\u{313}~ῐ~ᾱ", "ᾰ\u{313}~ῐ~ᾱ", "ᾰ\u{313}~ῐ~αι";
        nominative_neuter: "ᾰ\u{313}~ῐ~ον", "ᾰ\u{313}~ῐ~ω", "ᾰ\u{313}~ῐ~ᾰ";
        genitive_masculine: "ᾰ\u{313}~ῐ~ου", "ᾰ\u{313}~ῐ~οιν", "ᾰ\u{313}~ῐ~ων";
        genitive_feminine: "ᾰ\u{313}~ῐ~ᾱς", "ᾰ\u{313}~ῐ~αιν", "ᾰ\u{313}~ῐ~ων";
        genitive_neuter: "ᾰ\u{313}~ῐ~ου", "ᾰ\u{313}~ῐ~οιν", "ᾰ\u{313}~ῐ~ων";
        dative_masculine: "ᾰ\u{313}~ῐ~ῳ", "ᾰ\u{313}~ῐ~οιν", "ᾰ\u{313}~ῐ~οις";
        dative_feminine: "ᾰ\u{313}~ῐ~ᾳ", "ᾰ\u{313}~ῐ~αιν", "ᾰ\u{313}~ῐ~αις";
        dative_neuter: "ᾰ\u{313}~ῐ~ῳ", "ᾰ\u{313}~ῐ~οιν", "ᾰ\u{313}~ῐ~οις";
        accusative_masculine: "ᾰ\u{313}~ῐ~ον", "ᾰ\u{313}~ῐ~ω", "ᾰ\u{313}~ῐ~ους";
        accusative_feminine: "ᾰ\u{313}~ῐ~ᾱν", "ᾰ\u{313}~ῐ~ᾱ", "ᾰ\u{313}~ῐ~ᾱς";
        accusative_neuter: "ᾰ\u{313}~ῐ~ον", "ᾰ\u{313}~ῐ~ω", "ᾰ\u{313}~ῐ~ᾰ";
        vocative_masculine: "ᾰ\u{313}~ῐ~ε", "ᾰ\u{313}~ῐ~ω", "ᾰ\u{313}~ῐ~οι";
        vocative_feminine: "ᾰ\u{313}~ῐ~ᾱ", "ᾰ\u{313}~ῐ~ᾱ", "ᾰ\u{313}~ῐ~αι";
        vocative_neuter: "ᾰ\u{313}~ῐ~ον", "ᾰ\u{313}~ῐ~ω", "ᾰ\u{313}~ῐ~ᾰ";
        comparative: "ᾰ\u{313}~ῐ~ωτερος";
        superlative: "ᾰ\u{313}~ῐ~ωτᾰτος";
        adverb: "ᾰ\u{313}~ῐ~ον";
    ),
    // Ἀκρος: 7 words (ἀκρος, ἀλβος, ἀνομοιος, ἀρειος, ἀσπρος, ἀνθρωπειος).
    declension!("ἀκρος", 7, "ἀ~ος";
        nominative_masculine: "ᾰ\u{313}~ος", "ᾰ\u{313}~ω", "ᾰ\u{313}~οι";
        nominative_feminine: "ᾰ\u{313}~ᾱ", "ᾰ\u{313}~ᾱ", "ᾰ\u{313}~αι";
        nominative_neuter: "ᾰ\u{313}~ον", "ᾰ\u{313}~ω", "ᾰ\u{313}~ᾰ";
        genitive_masculine: "ᾰ\u{313}~ου", "ᾰ\u{313}~οιν", "ᾰ\u{313}~ων";
        genitive_feminine: "ᾰ\u{313}~ᾱς", "ᾰ\u{313}~αιν", "ᾰ\u{313}~ων";
        genitive_neuter: "ᾰ\u{313}~ου", "ᾰ\u{313}~οιν", "ᾰ\u{313}~ων";
        dative_masculine: "ᾰ\u{313}~ῳ", "ᾰ\u{313}~οιν", "ᾰ\u{313}~οις";
        dative_feminine: "ᾰ\u{313}~ᾳ", "ᾰ\u{313}~αιν", "ᾰ\u{313}~αις";
        dative_neuter: "ᾰ\u{313}~ῳ", "ᾰ\u{313}~οιν", "ᾰ\u{313}~οις";
        accusative_masculine: "ᾰ\u{313}~ον", "ᾰ\u{313}~ω", "ᾰ\u{313}~ους";
        accusative_feminine: "ᾰ\u{313}~ᾱν", "ᾰ\u{313}~ᾱ", "ᾰ\u{313}~ᾱς";
        accusative_neuter: "ᾰ\u{313}~ον", "ᾰ\u{313}~ω", "ᾰ\u{313}~ᾰ";
        vocative_masculine: "ᾰ\u{313}~ε", "ᾰ\u{313}~ω", "ᾰ\u{313}~οι";
        vocative_feminine: "ᾰ\u{313}~ᾱ", "ᾰ\u{313}~ᾱ", "ᾰ\u{313}~αι";
        vocative_neuter: "ᾰ\u{313}~ον", "ᾰ\u{313}~ω", "ᾰ\u{313}~ᾰ";
        comparative: "ᾰ\u{313}~οτερος";
        superlative: "ᾰ\u{313}~οτᾰτος";
        adverb: "ᾰ\u{313}~ως";
    ),
    // Ἀγραμματος: 7 words (ἀνανταποδοτος, ἀγραμματος, ἀδαπανος, ἀμανδαλος, ἀκανθοφαγος, ἀναγκαζομενος).
    declension!("ἀγραμματος", 7, "ἀ~α~α~ος";
        nominative_masculine: "ᾰ\u{313}~ᾰ~ᾰ~ος", "ᾰ\u{313}~ᾰ~ᾰ~ω", "ᾰ\u{313}~ᾰ~ᾰ~οι";
        nominative_feminine: "ᾰ\u{313}~ᾰ~ᾰ~ος", "ᾰ\u{313}~ᾰ~ᾰ~ω", "ᾰ\u{313}~ᾰ~ᾰ~οι";
        nominative_neuter: "ᾰ\u{313}~ᾰ~ᾰ~ον", "ᾰ\u{313}~ᾰ~ᾰ~ω", "ᾰ\u{313}~ᾰ~ᾰ~ᾰ";
        genitive_masculine: "ᾰ\u{313}~ᾰ~ᾰ~ου", "ᾰ\u{313}~ᾰ~ᾰ~οιν", "ᾰ\u{313}~ᾰ~ᾰ~ων";
        genitive_feminine: "ᾰ\u{313}~ᾰ~ᾰ~ου", "ᾰ\u{313}~ᾰ~ᾰ~οιν", "ᾰ\u{313}~ᾰ~ᾰ~ων";
        genitive_neuter: "ᾰ\u{313}~ᾰ~ᾰ~ου", "ᾰ\u{313}~ᾰ~ᾰ~οιν", "ᾰ\u{313}~ᾰ~ᾰ~ων";
        dative_masculine: "ᾰ\u{313}~ᾰ~ᾰ~ῳ", "ᾰ\u{313}~ᾰ~ᾰ~οιν", "ᾰ\u{313}~ᾰ~ᾰ~οις";
        dative_feminine: "ᾰ\u{313}~ᾰ~ᾰ~ῳ", "ᾰ\u{313}~ᾰ~ᾰ~οιν", "ᾰ\u{313}~ᾰ~ᾰ~οις";
        dative_neuter: "ᾰ\u{313}~ᾰ~ᾰ~ῳ", "ᾰ\u{313}~ᾰ~ᾰ~οιν", "ᾰ\u{313}~ᾰ~ᾰ~οις";
        accusative_masculine: "ᾰ\u{313}~ᾰ~ᾰ~ον", "ᾰ\u{313}~ᾰ~ᾰ~ω", "ᾰ\u{313}~ᾰ~ᾰ~ους";
        accusative_feminine: "ᾰ\u{313}~ᾰ~ᾰ~ον", "ᾰ\u{313}~ᾰ~ᾰ~ω", "ᾰ\u{313}~ᾰ~ᾰ~ους";
        accusative_neuter: "ᾰ\u{313}~ᾰ~ᾰ~ον", "ᾰ\u{313}~ᾰ~ᾰ~ω", "ᾰ\u{313}~ᾰ~ᾰ~ᾰ";
        vocative_masculine: "ᾰ\u{313}~ᾰ~ᾰ~ε", "ᾰ\u{313}~ᾰ~ᾰ~ω", "ᾰ\u{313}~ᾰ~ᾰ~οι";
        vocative_feminine: "ᾰ\u{313}~ᾰ~ᾰ~ε", "ᾰ\u{313}~ᾰ~ᾰ~ω", "ᾰ\u{313}~ᾰ~ᾰ~οι";
        vocative_neuter: "ᾰ\u{313}~ᾰ~ᾰ~ον", "ᾰ\u{313}~ᾰ~ᾰ~ω", "ᾰ\u{313}~ᾰ~ᾰ~ᾰ";
        comparative: "ᾰ\u{313}~ᾰ~ᾰ~ωτερος";
        superlative: "ᾰ\u{313}~ᾰ~ᾰ~ωτᾰτος";
        adverb: "ᾰ\u{313}~ᾰ~ᾰ~ως";
    ),
    // Ἀσιτος: 7 words (ἀνικητος, ἀλιτρος, ἀφικομενος, ἀκριθος, ἀσιτος, ἀκριβεστερος).
    declension!("ἀσιτος", 7, "ἀ~ι~ος";
        nominative_masculine: "ᾰ\u{313}~ῑ~ος", "ᾰ\u{313}~ῑ~ω", "ᾰ\u{313}~ῑ~οι";
        nominative_feminine: "ᾰ\u{313}~ῑ~ος", "ᾰ\u{313}~ῑ~ω", "ᾰ\u{313}~ῑ~οι";
        nominative_neuter: "ᾰ\u{313}~ῑ~ον", "ᾰ\u{313}~ῑ~ω", "ᾰ\u{313}~ῑ~ᾰ";
        genitive_masculine: "ᾰ\u{313}~ῑ~ου", "ᾰ\u{313}~ῑ~οιν", "ᾰ\u{313}~ῑ~ων";
        genitive_feminine: "ᾰ\u{313}~ῑ~ου", "ᾰ\u{313}~ῑ~οιν", "ᾰ\u{313}~ῑ~ων";
        genitive_neuter: "ᾰ\u{313}~ῑ~ου", "ᾰ\u{313}~ῑ~οιν", "ᾰ\u{313}~ῑ~ων";
        dative_masculine: "ᾰ\u{313}~ῑ~ῳ", "ᾰ\u{313}~ῑ~οιν", "ᾰ\u{313}~ῑ~οις";
        dative_feminine: "ᾰ\u{313}~ῑ~ῳ", "ᾰ\u{313}~ῑ~οιν", "ᾰ\u{313}~ῑ~οις";
        dative_neuter: "ᾰ\u{313}~ῑ~ῳ", "ᾰ\u{313}~ῑ~οιν", "ᾰ\u{313}~ῑ~οις";
        accusative_masculine: "ᾰ\u{313}~ῑ~ον", "ᾰ\u{313}~ῑ~ω", "ᾰ\u{313}~ῑ~ους";
        accusative_feminine: "ᾰ\u{313}~ῑ~ον", "ᾰ\u{313}~ῑ~ω", "ᾰ\u{313}~ῑ~ους";
        accusative_neuter: "ᾰ\u{313}~ῑ~ον", "ᾰ\u{313}~ῑ~ω", "ᾰ\u{313}~ῑ~ᾰ";
        vocative_masculine: "ᾰ\u{313}~ῑ~ε", "ᾰ\u{313}~ῑ~ω", "ᾰ\u{313}~ῑ~οι";
        vocative_feminine: "ᾰ\u{313}~ῑ~ε", "ᾰ\u{313}~ῑ~ω", "ᾰ\u{313}~ῑ~οι";
        vocative_neuter: "ᾰ\u{313}~ῑ~ον", "ᾰ\u{313}~ῑ~ω", "ᾰ\u{313}~ῑ~ᾰ";
        comparative: "ᾰ\u{313}~ῑ~οτερος";
        superlative: "ᾰ\u{313}~ῑ~οτᾰτος";
        adverb: "ᾰ\u{313}~ῑ~ως";
    ),
    // Ἁβρος: 7 words (ἁγνος, ἁδρος, ἁπλοος, ἁβρος, ἁδῐνος, ἁλισκομενος).
    declension!("ἁβρος", 7, "ἁ~ος";
        nominative_masculine: "ᾰ\u{314}~ος", "ᾰ\u{314}~ω", "ᾰ\u{314}~οι";
        nominative_feminine: "ᾰ\u{314}~η", "ᾰ\u{314}~ᾱ", "ᾰ\u{314}~αι";
        nominative_neuter: "ᾰ\u{314}~ον", "ᾰ\u{314}~ω", "ᾰ\u{314}~ᾰ";
        genitive_masculine: "ᾰ\u{314}~ου", "ᾰ\u{314}~οιν", "ᾰ\u{314}~ων";
        genitive_feminine: "ᾰ\u{314}~ης", "ᾰ\u{314}~αιν", "ᾰ\u{314}~ων";
        genitive_neuter: "ᾰ\u{314}~ου", "ᾰ\u{314}~οιν", "ᾰ\u{314}~ων";
        dative_masculine: "ᾰ\u{314}~ῳ", "ᾰ\u{314}~οιν", "ᾰ\u{314}~οις";
        dative_feminine: "ᾰ\u{314}~ῃ", "ᾰ\u{314}~αιν", "ᾰ\u{314}~αις";
        dative_neuter: "ᾰ\u{314}~ῳ", "ᾰ\u{314}~οιν", "ᾰ\u{314}~οις";
        accusative_masculine: "ᾰ\u{314}~ον", "ᾰ\u{314}~ω", "ᾰ\u{314}~ους";
        accusative_feminine: "ᾰ\u{314}~ην", "ᾰ\u{314}~ᾱ", "ᾰ\u{314}~ᾱς";
        accusative_neuter: "ᾰ\u{314}~ον", "ᾰ\u{314}~ω", "ᾰ\u{314}~ᾰ";
        vocative_masculine: "ᾰ\u{314}~ε", "ᾰ\u{314}~ω", "ᾰ\u{314}~οι";
        vocative_feminine: "ᾰ\u{314}~η", "ᾰ\u{314}~ᾱ", "ᾰ\u{314}~αι";
        vocative_neuter: "ᾰ\u{314}~ον", "ᾰ\u{314}~ω", "ᾰ\u{314}~ᾰ";
        comparative: "ᾰ\u{314}~ωτερος";
        superlative: "ᾰ\u{314}~ωτᾰτος";
        adverb: "ᾰ\u{314}~ως";
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
        comparative: "~ωτερος";
        superlative: "~ωτᾰτος";
        adverb: "~ῠντως";
    ),
    // Ἀρξαμενος: 7 words (ἀνεσκαμμενος, ἀγαθοποιησομενος, ἀγγειλαμενος, ἀρξαμενος, ἀπαγχομενος, ἀποκρῑναμενος).
    declension!("ἀρξαμενος", 7, "ἀ~α~ος";
        nominative_masculine: "ᾰ\u{313}~ᾰ~ος", "ᾰ\u{313}~ᾰ~ω", "ᾰ\u{313}~ᾰ~οι";
        nominative_feminine: "ᾰ\u{313}~ᾰ~η", "ᾰ\u{313}~ᾰ~ᾱ", "ᾰ\u{313}~ᾰ~αι";
        nominative_neuter: "ᾰ\u{313}~ᾰ~ον", "ᾰ\u{313}~ᾰ~ω", "ᾰ\u{313}~ᾰ~ᾰ";
        genitive_masculine: "ᾰ\u{313}~ᾰ~ου", "ᾰ\u{313}~ᾰ~οιν", "ᾰ\u{313}~ᾰ~ων";
        genitive_feminine: "ᾰ\u{313}~ᾰ~ης", "ᾰ\u{313}~ᾰ~αιν", "ᾰ\u{313}~ᾰ~ων";
        genitive_neuter: "ᾰ\u{313}~ᾰ~ου", "ᾰ\u{313}~ᾰ~οιν", "ᾰ\u{313}~ᾰ~ων";
        dative_masculine: "ᾰ\u{313}~ᾰ~ῳ", "ᾰ\u{313}~ᾰ~οιν", "ᾰ\u{313}~ᾰ~οις";
        dative_feminine: "ᾰ\u{313}~ᾰ~ῃ", "ᾰ\u{313}~ᾰ~αιν", "ᾰ\u{313}~ᾰ~αις";
        dative_neuter: "ᾰ\u{313}~ᾰ~ῳ", "ᾰ\u{313}~ᾰ~οιν", "ᾰ\u{313}~ᾰ~οις";
        accusative_masculine: "ᾰ\u{313}~ᾰ~ον", "ᾰ\u{313}~ᾰ~ω", "ᾰ\u{313}~ᾰ~ους";
        accusative_feminine: "ᾰ\u{313}~ᾰ~ην", "ᾰ\u{313}~ᾰ~ᾱ", "ᾰ\u{313}~ᾰ~ᾱς";
        accusative_neuter: "ᾰ\u{313}~ᾰ~ον", "ᾰ\u{313}~ᾰ~ω", "ᾰ\u{313}~ᾰ~ᾰ";
        vocative_masculine: "ᾰ\u{313}~ᾰ~ε", "ᾰ\u{313}~ᾰ~ω", "ᾰ\u{313}~ᾰ~οι";
        vocative_feminine: "ᾰ\u{313}~ᾰ~η", "ᾰ\u{313}~ᾰ~ᾱ", "ᾰ\u{313}~ᾰ~αι";
        vocative_neuter: "ᾰ\u{313}~ᾰ~ον", "ᾰ\u{313}~ᾰ~ω", "ᾰ\u{313}~ᾰ~ᾰ";
        comparative: "ᾰ\u{313}~ᾰ~ωτερος";
        superlative: "ᾰ\u{313}~ᾰ~ωτᾰτος";
        adverb: "ᾰ\u{313}~ᾰ~ως";
    ),
    // Ἰσος: 7 words (ἰσος, ἰσομετρητος, ἰσχαιμος, ἰσαιτερος, ἰδῐωτῐκος, ἰσοτερος).
    declension!("ἰσος", 7, "ἰ~ος";
        nominative_masculine: "ῐ\u{313}~ος", "ῐ\u{313}~ω", "ῐ\u{313}~οι";
        nominative_feminine: "ῐ\u{313}~ᾱ", "ῐ\u{313}~ᾱ", "ῐ\u{313}~αι";
        nominative_neuter: "ῐ\u{313}~ον", "ῐ\u{313}~ω", "ῐ\u{313}~ᾰ";
        genitive_masculine: "ῐ\u{313}~ου", "ῐ\u{313}~οιν", "ῐ\u{313}~ων";
        genitive_feminine: "ῐ\u{313}~ᾱς", "ῐ\u{313}~αιν", "ῐ\u{313}~ων";
        genitive_neuter: "ῐ\u{313}~ου", "ῐ\u{313}~οιν", "ῐ\u{313}~ων";
        dative_masculine: "ῐ\u{313}~ῳ", "ῐ\u{313}~οιν", "ῐ\u{313}~οις";
        dative_feminine: "ῐ\u{313}~ᾳ", "ῐ\u{313}~αιν", "ῐ\u{313}~αις";
        dative_neuter: "ῐ\u{313}~ῳ", "ῐ\u{313}~οιν", "ῐ\u{313}~οις";
        accusative_masculine: "ῐ\u{313}~ον", "ῐ\u{313}~ω", "ῐ\u{313}~ους";
        accusative_feminine: "ῐ\u{313}~ᾱν", "ῐ\u{313}~ᾱ", "ῐ\u{313}~ᾱς";
        accusative_neuter: "ῐ\u{313}~ον", "ῐ\u{313}~ω", "ῐ\u{313}~ᾰ";
        vocative_masculine: "ῐ\u{313}~ε", "ῐ\u{313}~ω", "ῐ\u{313}~οι";
        vocative_feminine: "ῐ\u{313}~ᾱ", "ῐ\u{313}~ᾱ", "ῐ\u{313}~αι";
        vocative_neuter: "ῐ\u{313}~ον", "ῐ\u{313}~ω", "ῐ\u{313}~ᾰ";
        comparative: "~ωτερος";
        superlative: "~ωτᾰτος";
        adverb: "ῐ\u{313}~ον";
    ),
    // Ἀκαθαρτος: 6 words (ἀκαθαρτος, ἀκαταληπτος, ἀκᾰτασκευαστος, ἀταλαντος, ἀναλφαβητος, ἀταρακτος).
    declension!("ἀκαθαρτος", 6, "ἀ~α~α~ος";
        nominative_masculine: "ᾰ\u{313}~ᾰ~ᾰ~ος", "ᾰ\u{313}~ᾰ~ᾰ~ω", "ᾰ\u{313}~ᾰ~ᾰ~οι";
        nominative_feminine: "ᾰ\u{313}~ᾰ~ᾰ~ος", "ᾰ\u{313}~ᾰ~ᾰ~ω", "ᾰ\u{313}~ᾰ~ᾰ~οι";
        nominative_neuter: "ᾰ\u{313}~ᾰ~ᾰ~ον", "ᾰ\u{313}~ᾰ~ᾰ~ω", "ᾰ\u{313}~ᾰ~ᾰ~ᾰ";
        genitive_masculine: "ᾰ\u{313}~ᾰ~ᾰ~ου", "ᾰ\u{313}~ᾰ~ᾰ~οιν", "ᾰ\u{313}~ᾰ~ᾰ~ων";
        genitive_feminine: "ᾰ\u{313}~ᾰ~ᾰ~ου", "ᾰ\u{313}~ᾰ~ᾰ~οιν", "ᾰ\u{313}~ᾰ~ᾰ~ων";
        genitive_neuter: "ᾰ\u{313}~ᾰ~ᾰ~ου", "ᾰ\u{313}~ᾰ~ᾰ~οιν", "ᾰ\u{313}~ᾰ~ᾰ~ων";
        dative_masculine: "ᾰ\u{313}~ᾰ~ᾰ~ῳ", "ᾰ\u{313}~ᾰ~ᾰ~οιν", "ᾰ\u{313}~ᾰ~ᾰ~οις";
        dative_feminine: "ᾰ\u{313}~ᾰ~ᾰ~ῳ", "ᾰ\u{313}~ᾰ~ᾰ~οιν", "ᾰ\u{313}~ᾰ~ᾰ~οις";
        dative_neuter: "ᾰ\u{313}~ᾰ~ᾰ~ῳ", "ᾰ\u{313}~ᾰ~ᾰ~οιν", "ᾰ\u{313}~ᾰ~ᾰ~οις";
        accusative_masculine: "ᾰ\u{313}~ᾰ~ᾰ~ον", "ᾰ\u{313}~ᾰ~ᾰ~ω", "ᾰ\u{313}~ᾰ~ᾰ~ους";
        accusative_feminine: "ᾰ\u{313}~ᾰ~ᾰ~ον", "ᾰ\u{313}~ᾰ~ᾰ~ω", "ᾰ\u{313}~ᾰ~ᾰ~ους";
        accusative_neuter: "ᾰ\u{313}~ᾰ~ᾰ~ον", "ᾰ\u{313}~ᾰ~ᾰ~ω", "ᾰ\u{313}~ᾰ~ᾰ~ᾰ";
        vocative_masculine: "ᾰ\u{313}~ᾰ~ᾰ~ε", "ᾰ\u{313}~ᾰ~ᾰ~ω", "ᾰ\u{313}~ᾰ~ᾰ~οι";
        vocative_feminine: "ᾰ\u{313}~ᾰ~ᾰ~ε", "ᾰ\u{313}~ᾰ~ᾰ~ω", "ᾰ\u{313}~ᾰ~ᾰ~οι";
        vocative_neuter: "ᾰ\u{313}~ᾰ~ᾰ~ον", "ᾰ\u{313}~ᾰ~ᾰ~ω", "ᾰ\u{313}~ᾰ~ᾰ~ᾰ";
        comparative: "ᾰ\u{313}~ᾰ~ᾰ~οτερος";
        superlative: "ᾰ\u{313}~ᾰ~ᾰ~οτᾰτος";
        adverb: "ᾰ\u{313}~ᾰ~ᾰ~ως";
    ),
    // Ἀφροδῑσῐος: 6 words (ἀσκῐος, ἀθροος, ἀνᾰλῠτεος, ἀφροδῑσῐος, ἀπολλωνῐος, ἀνθεκτεος).
    declension!("ἀφροδῑσῐος", 6, "ἀ~ος";
        nominative_masculine: "ᾰ\u{313}~ος", "ᾰ\u{313}~ω", "ᾰ\u{313}~οι";
        nominative_feminine: "ᾰ\u{313}~ᾱ", "ᾰ\u{313}~ᾱ", "ᾰ\u{313}~αι";
        nominative_neuter: "ᾰ\u{313}~ον", "ᾰ\u{313}~ω", "ᾰ\u{313}~ᾰ";
        genitive_masculine: "ᾰ\u{313}~ου", "ᾰ\u{313}~οιν", "ᾰ\u{313}~ων";
        genitive_feminine: "ᾰ\u{313}~ᾱς", "ᾰ\u{313}~αιν", "ᾰ\u{313}~ων";
        genitive_neuter: "ᾰ\u{313}~ου", "ᾰ\u{313}~οιν", "ᾰ\u{313}~ων";
        dative_masculine: "ᾰ\u{313}~ῳ", "ᾰ\u{313}~οιν", "ᾰ\u{313}~οις";
        dative_feminine: "ᾰ\u{313}~ᾳ", "ᾰ\u{313}~αιν", "ᾰ\u{313}~αις";
        dative_neuter: "ᾰ\u{313}~ῳ", "ᾰ\u{313}~οιν", "ᾰ\u{313}~οις";
        accusative_masculine: "ᾰ\u{313}~ον", "ᾰ\u{313}~ω", "ᾰ\u{313}~ους";
        accusative_feminine: "ᾰ\u{313}~ᾱν", "ᾰ\u{313}~ᾱ", "ᾰ\u{313}~ᾱς";
        accusative_neuter: "ᾰ\u{313}~ον", "ᾰ\u{313}~ω", "ᾰ\u{313}~ᾰ";
        vocative_masculine: "ᾰ\u{313}~ε", "ᾰ\u{313}~ω", "ᾰ\u{313}~οι";
        vocative_feminine: "ᾰ\u{313}~ᾱ", "ᾰ\u{313}~ᾱ", "ᾰ\u{313}~αι";
        vocative_neuter: "ᾰ\u{313}~ον", "ᾰ\u{313}~ω", "ᾰ\u{313}~ᾰ";
        comparative: "ᾰ\u{313}~ωτερος";
        superlative: "ᾰ\u{313}~ωτᾰτος";
        adverb: "ᾰ\u{313}~ως";
    ),
    // Ἀσυμπτωτος: 6 words (ἀνυποδητος, ἀμεθυστος, ἀρρυθμος, ἀργυροηλος, ἀγρυπνος, ἀσυμπτωτος).
    declension!("ἀσυμπτωτος", 6, "ἀ~υ~ος";
        nominative_masculine: "ᾰ\u{313}~ῠ~ος", "ᾰ\u{313}~ῠ~ω", "ᾰ\u{313}~ῠ~οι";
        nominative_feminine: "ᾰ\u{313}~ῠ~ος", "ᾰ\u{313}~ῠ~ω", "ᾰ\u{313}~ῠ~οι";
        nominative_neuter: "ᾰ\u{313}~ῠ~ον", "ᾰ\u{313}~ῠ~ω", "ᾰ\u{313}~ῠ~ᾰ";
        genitive_masculine: "ᾰ\u{313}~ῠ~ου", "ᾰ\u{313}~ῠ~οιν", "ᾰ\u{313}~ῠ~ων";
        genitive_feminine: "ᾰ\u{313}~ῠ~ου", "ᾰ\u{313}~ῠ~οιν", "ᾰ\u{313}~ῠ~ων";
        genitive_neuter: "ᾰ\u{313}~ῠ~ου", "ᾰ\u{313}~ῠ~οιν", "ᾰ\u{313}~ῠ~ων";
        dative_masculine: "ᾰ\u{313}~ῠ~ῳ", "ᾰ\u{313}~ῠ~οιν", "ᾰ\u{313}~ῠ~οις";
        dative_feminine: "ᾰ\u{313}~ῠ~ῳ", "ᾰ\u{313}~ῠ~οιν", "ᾰ\u{313}~ῠ~οις";
        dative_neuter: "ᾰ\u{313}~ῠ~ῳ", "ᾰ\u{313}~ῠ~οιν", "ᾰ\u{313}~ῠ~οις";
        accusative_masculine: "ᾰ\u{313}~ῠ~ον", "ᾰ\u{313}~ῠ~ω", "ᾰ\u{313}~ῠ~ους";
        accusative_feminine: "ᾰ\u{313}~ῠ~ον", "ᾰ\u{313}~ῠ~ω", "ᾰ\u{313}~ῠ~ους";
        accusative_neuter: "ᾰ\u{313}~ῠ~ον", "ᾰ\u{313}~ῠ~ω", "ᾰ\u{313}~ῠ~ᾰ";
        vocative_masculine: "ᾰ\u{313}~ῠ~ε", "ᾰ\u{313}~ῠ~ω", "ᾰ\u{313}~ῠ~οι";
        vocative_feminine: "ᾰ\u{313}~ῠ~ε", "ᾰ\u{313}~ῠ~ω", "ᾰ\u{313}~ῠ~οι";
        vocative_neuter: "ᾰ\u{313}~ῠ~ον", "ᾰ\u{313}~ῠ~ω", "ᾰ\u{313}~ῠ~ᾰ";
        comparative: "ᾰ\u{313}~ῠ~οτερος";
        superlative: "ᾰ\u{313}~ῠ~οτᾰτος";
        adverb: "ᾰ\u{313}~ῠ~ως";
    ),
    // Ἀκμαζων: 6 words (ἀγαθοεργεων, ἀγαθοποιων, ἀγαθοποιησων, ἀκμαζων, ἀναγγελλων, ἀνακρῐνων).
    declension!("ἀκμαζων", 6, "ἀ~α~ων";
        nominative_masculine: "ᾰ\u{313}~ᾰ~ων", "ᾰ\u{313}~ᾰ~οντε", "ᾰ\u{313}~ᾰ~οντες";
        nominative_feminine: "ᾰ\u{313}~ᾰ~ουσᾰ", "ᾰ\u{313}~ᾰ~ουσᾱ", "ᾰ\u{313}~ᾰ~ουσαι";
        nominative_neuter: "ᾰ\u{313}~ᾰ~ον", "ᾰ\u{313}~ᾰ~οντε", "ᾰ\u{313}~ᾰ~οντᾰ";
        genitive_masculine: "ᾰ\u{313}~ᾰ~οντος", "ᾰ\u{313}~ᾰ~οντοιν", "ᾰ\u{313}~ᾰ~οντων";
        genitive_feminine: "ᾰ\u{313}~ᾰ~ουσης", "ᾰ\u{313}~ᾰ~ουσαιν", "ᾰ\u{313}~ᾰ~ουσων";
        genitive_neuter: "ᾰ\u{313}~ᾰ~οντος", "ᾰ\u{313}~ᾰ~οντοιν", "ᾰ\u{313}~ᾰ~οντων";
        dative_masculine: "ᾰ\u{313}~ᾰ~οντῐ", "ᾰ\u{313}~ᾰ~οντοιν", "ᾰ\u{313}~ᾰ~ουσῐ|ᾰ\u{313}~ᾰ~ουσῐν";
        dative_feminine: "ᾰ\u{313}~ᾰ~ουσῃ", "ᾰ\u{313}~ᾰ~ουσαιν", "ᾰ\u{313}~ᾰ~ουσαις";
        dative_neuter: "ᾰ\u{313}~ᾰ~οντῐ", "ᾰ\u{313}~ᾰ~οντοιν", "ᾰ\u{313}~ᾰ~ουσῐ|ᾰ\u{313}~ᾰ~ουσῐν";
        accusative_masculine: "ᾰ\u{313}~ᾰ~οντᾰ", "ᾰ\u{313}~ᾰ~οντε", "ᾰ\u{313}~ᾰ~οντᾰς";
        accusative_feminine: "ᾰ\u{313}~ᾰ~ουσᾰν", "ᾰ\u{313}~ᾰ~ουσᾱ", "ᾰ\u{313}~ᾰ~ουσᾱς";
        accusative_neuter: "ᾰ\u{313}~ᾰ~ον", "ᾰ\u{313}~ᾰ~οντε", "ᾰ\u{313}~ᾰ~οντᾰ";
        vocative_masculine: "ᾰ\u{313}~ᾰ~ων", "ᾰ\u{313}~ᾰ~οντε", "ᾰ\u{313}~ᾰ~οντες";
        vocative_feminine: "ᾰ\u{313}~ᾰ~ουσᾰ", "ᾰ\u{313}~ᾰ~ουσᾱ", "ᾰ\u{313}~ᾰ~ουσαι";
        vocative_neuter: "ᾰ\u{313}~ᾰ~ον", "ᾰ\u{313}~ᾰ~οντε", "ᾰ\u{313}~ᾰ~οντᾰ";
        comparative: "ᾰ\u{313}~ᾰ~οτερος";
        superlative: "ᾰ\u{313}~ᾰ~οτᾰτος";
        adverb: "ᾰ\u{313}~ᾰ~οντως";
    ),
    // Ὑπεγγῠος: 6 words (ὑψοροφος, ὑποστεγος, ὑποχειρῐος, ὑποδρομος, ὑποτροφος, ὑπεγγῠος).
    declension!("ὑπεγγῠος", 6, "ὑ~ος";
        nominative_masculine: "ῠ\u{314}~ος", "ῠ\u{314}~ω", "ῠ\u{314}~οι";
        nominative_feminine: "ῠ\u{314}~ος", "ῠ\u{314}~ω", "ῠ\u{314}~οι";
        nominative_neuter: "ῠ\u{314}~ον", "ῠ\u{314}~ω", "ῠ\u{314}~ᾰ";
        genitive_masculine: "ῠ\u{314}~ου", "ῠ\u{314}~οιν", "ῠ\u{314}~ων";
        genitive_feminine: "ῠ\u{314}~ου", "ῠ\u{314}~οιν", "ῠ\u{314}~ων";
        genitive_neuter: "ῠ\u{314}~ου", "ῠ\u{314}~οιν", "ῠ\u{314}~ων";
        dative_masculine: "ῠ\u{314}~ῳ", "ῠ\u{314}~οιν", "ῠ\u{314}~οις";
        dative_feminine: "ῠ\u{314}~ῳ", "ῠ\u{314}~οιν", "ῠ\u{314}~οις";
        dative_neuter: "ῠ\u{314}~ῳ", "ῠ\u{314}~οιν", "ῠ\u{314}~οις";
        accusative_masculine: "ῠ\u{314}~ον", "ῠ\u{314}~ω", "ῠ\u{314}~ους";
        accusative_feminine: "ῠ\u{314}~ον", "ῠ\u{314}~ω", "ῠ\u{314}~ους";
        accusative_neuter: "ῠ\u{314}~ον", "ῠ\u{314}~ω", "ῠ\u{314}~ᾰ";
        vocative_masculine: "ῠ\u{314}~ε", "ῠ\u{314}~ω", "ῠ\u{314}~οι";
        vocative_feminine: "ῠ\u{314}~ε", "ῠ\u{314}~ω", "ῠ\u{314}~οι";
        vocative_neuter: "ῠ\u{314}~ον", "ῠ\u{314}~ω", "ῠ\u{314}~ᾰ";
        comparative: "ῠ\u{314}~ωτερος";
        superlative: "ῠ\u{314}~ωτᾰτος";
        adverb: "ῠ\u{314}~ως";
    ),
    // Ἀργος: 5 words (ἀδελφος, ἀοιδος, ἀργος, ἀνοικτος, ἀπᾰτηλος).
    declension!("ἀργος", 5, "ἀ~ος";
        nominative_masculine: "ᾰ\u{313}~ος", "ᾰ\u{313}~ω", "ᾰ\u{313}~οι";
        nominative_feminine: "ᾰ\u{313}~η", "ᾰ\u{313}~ᾱ", "ᾰ\u{313}~αι";
        nominative_neuter: "ᾰ\u{313}~ον", "ᾰ\u{313}~ω", "ᾰ\u{313}~ᾰ";
        genitive_masculine: "ᾰ\u{313}~οῡ", "ᾰ\u{313}~οῑν", "ᾰ\u{313}~ων";
        genitive_feminine: "ᾰ\u{313}~ης", "ᾰ\u{313}~αῑν", "ᾰ\u{313}~ων";
        genitive_neuter: "ᾰ\u{313}~οῡ", "ᾰ\u{313}~οῑν", "ᾰ\u{313}~ων";
        dative_masculine: "ᾰ\u{313}~ῳ", "ᾰ\u{313}~οῑν", "ᾰ\u{313}~οῑς";
        dative_feminine: "ᾰ\u{313}~ῃ", "ᾰ\u{313}~αῑν", "ᾰ\u{313}~αῑς";
        dative_neuter: "ᾰ\u{313}~ῳ", "ᾰ\u{313}~οῑν", "ᾰ\u{313}~οῑς";
        accusative_masculine: "ᾰ\u{313}~ον", "ᾰ\u{313}~ω", "ᾰ\u{313}~ους";
        accusative_feminine: "ᾰ\u{313}~ην", "ᾰ\u{313}~ᾱ", "ᾰ\u{313}~ᾱς";
        accusative_neuter: "ᾰ\u{313}~ον", "ᾰ\u{313}~ω", "ᾰ\u{313}~ᾰ";
        vocative_masculine: "ᾰ\u{313}~ε", "ᾰ\u{313}~ω", "ᾰ\u{313}~οι";
        vocative_feminine: "ᾰ\u{313}~η", "ᾰ\u{313}~ᾱ", "ᾰ\u{313}~αι";
        vocative_neuter: "ᾰ\u{313}~ον", "ᾰ\u{313}~ω", "ᾰ\u{313}~ᾰ";
        comparative: "ᾰ\u{313}~οτερος";
        superlative: "ᾰ\u{313}~οτᾰτος";
        adverb: "ᾰ\u{313}~ως";
    ),
    // Ἀοριστος: 5 words (ἀγριελαιος, ἀοριστος, ἀμαυροστικτος, ἀνᾰπονιπτος, ἀκινδῡνος).
    declension!("ἀοριστος", 5, "ἀ~ι~ος";
        nominative_masculine: "ᾰ\u{313}~ῐ~ος", "ᾰ\u{313}~ῐ~ω", "ᾰ\u{313}~ῐ~οι";
        nominative_feminine: "ᾰ\u{313}~ῐ~ος", "ᾰ\u{313}~ῐ~ω", "ᾰ\u{313}~ῐ~οι";
        nominative_neuter: "ᾰ\u{313}~ῐ~ον", "ᾰ\u{313}~ῐ~ω", "ᾰ\u{313}~ῐ~ᾰ";
        genitive_masculine: "ᾰ\u{313}~ῐ~ου", "ᾰ\u{313}~ῐ~οιν", "ᾰ\u{313}~ῐ~ων";
        genitive_feminine: "ᾰ\u{313}~ῐ~ου", "ᾰ\u{313}~ῐ~οιν", "ᾰ\u{313}~ῐ~ων";
        genitive_neuter: "ᾰ\u{313}~ῐ~ου", "ᾰ\u{313}~ῐ~οιν", "ᾰ\u{313}~ῐ~ων";
        dative_masculine: "ᾰ\u{313}~ῐ~ῳ", "ᾰ\u{313}~ῐ~οιν", "ᾰ\u{313}~ῐ~οις";
        dative_feminine: "ᾰ\u{313}~ῐ~ῳ", "ᾰ\u{313}~ῐ~οιν", "ᾰ\u{313}~ῐ~οις";
        dative_neuter: "ᾰ\u{313}~ῐ~ῳ", "ᾰ\u{313}~ῐ~οιν", "ᾰ\u{313}~ῐ~οις";
        accusative_masculine: "ᾰ\u{313}~ῐ~ον", "ᾰ\u{313}~ῐ~ω", "ᾰ\u{313}~ῐ~ους";
        accusative_feminine: "ᾰ\u{313}~ῐ~ον", "ᾰ\u{313}~ῐ~ω", "ᾰ\u{313}~ῐ~ους";
        accusative_neuter: "ᾰ\u{313}~ῐ~ον", "ᾰ\u{313}~ῐ~ω", "ᾰ\u{313}~ῐ~ᾰ";
        vocative_masculine: "ᾰ\u{313}~ῐ~ε", "ᾰ\u{313}~ῐ~ω", "ᾰ\u{313}~ῐ~οι";
        vocative_feminine: "ᾰ\u{313}~ῐ~ε", "ᾰ\u{313}~ῐ~ω", "ᾰ\u{313}~ῐ~οι";
        vocative_neuter: "ᾰ\u{313}~ῐ~ον", "ᾰ\u{313}~ῐ~ω", "ᾰ\u{313}~ῐ~ᾰ";
        comparative: "ᾰ\u{313}~ῐ~οτερος";
        superlative: "ᾰ\u{313}~ῐ~οτᾰτος";
        adverb: "ᾰ\u{313}~ῐ~ως";
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
        comparative: "~ωτερος";
        superlative: "~ωτᾰτος";
        adverb: "~πως";
    ),
    // Δους: 5 words (δους, βῐους, δῐδους, γνους, γενομενους).
    declension!("δους", 5, "~υς";
        nominative_masculine: "~υς", "~ντε", "~ντες";
        nominative_feminine: "~ῡσᾰ", "~υσᾱ", "~ῡσαι";
        nominative_neuter: "~ν", "~ντε", "~ντᾰ";
        genitive_masculine: "~ντος", "~ντοιν", "~ντων";
        genitive_feminine: "~υσης", "~υσαιν", "~υσων";
        genitive_neuter: "~ντος", "~ντοιν", "~ντων";
        dative_masculine: "~ντῐ", "~ντοιν", "~ῡσῐ|~ῡσῐν";
        dative_feminine: "~υσῃ", "~υσαιν", "~υσαις";
        dative_neuter: "~ντῐ", "~ντοιν", "~ῡσῐ|~ῡσῐν";
        accusative_masculine: "~ντᾰ", "~ντε", "~ντᾰς";
        accusative_feminine: "~ῡσᾰν", "~υσᾱ", "~υσᾱς";
        accusative_neuter: "~ν", "~ντε", "~ντᾰ";
        vocative_masculine: "~υς", "~ντε", "~ντες";
        vocative_feminine: "~ῡσᾰ", "~υσᾱ", "~ῡσαι";
        vocative_neuter: "~ν", "~ντε", "~ντᾰ";
        comparative: "~στερος";
        superlative: "~στᾰτος";
        adverb: "~ντως";
    ),
    // Ἀντιπαις: 5 words (ἀργινοεις, ἀντιπαις, ἀξιωθεις, ἀποτειχισθεις, ἀποκριθεις).
    declension!("ἀντιπαις", 5, "ἀ~ι~ις";
        nominative_masculine: "ᾰ\u{313}~ῐ~ις", "ᾰ\u{313}~ῐ~ντε", "ᾰ\u{313}~ῐ~ντες";
        nominative_feminine: "ᾰ\u{313}~ῐ~ῑσᾰ", "ᾰ\u{313}~ῐ~ισᾱ", "ᾰ\u{313}~ῐ~ῑσαι";
        nominative_neuter: "ᾰ\u{313}~ῐ~ν", "ᾰ\u{313}~ῐ~ντε", "ᾰ\u{313}~ῐ~ντᾰ";
        genitive_masculine: "ᾰ\u{313}~ῐ~ντος", "ᾰ\u{313}~ῐ~ντοιν", "ᾰ\u{313}~ῐ~ντων";
        genitive_feminine: "ᾰ\u{313}~ῐ~ισης", "ᾰ\u{313}~ῐ~ισαιν", "ᾰ\u{313}~ῐ~ισων";
        genitive_neuter: "ᾰ\u{313}~ῐ~ντος", "ᾰ\u{313}~ῐ~ντοιν", "ᾰ\u{313}~ῐ~ντων";
        dative_masculine: "ᾰ\u{313}~ῐ~ντῐ", "ᾰ\u{313}~ῐ~ντοιν", "ᾰ\u{313}~ῐ~ῑσῐ|ᾰ\u{313}~ῐ~ῑσῐν";
        dative_feminine: "ᾰ\u{313}~ῐ~ισῃ", "ᾰ\u{313}~ῐ~ισαιν", "ᾰ\u{313}~ῐ~ισαις";
        dative_neuter: "ᾰ\u{313}~ῐ~ντῐ", "ᾰ\u{313}~ῐ~ντοιν", "ᾰ\u{313}~ῐ~ῑσῐ|ᾰ\u{313}~ῐ~ῑσῐν";
        accusative_masculine: "ᾰ\u{313}~ῐ~ντᾰ", "ᾰ\u{313}~ῐ~ντε", "ᾰ\u{313}~ῐ~ντᾰς";
        accusative_feminine: "ᾰ\u{313}~ῐ~ῑσᾰν", "ᾰ\u{313}~ῐ~ισᾱ", "ᾰ\u{313}~ῐ~ισᾱς";
        accusative_neuter: "ᾰ\u{313}~ῐ~ν", "ᾰ\u{313}~ῐ~ντε", "ᾰ\u{313}~ῐ~ντᾰ";
        vocative_masculine: "ᾰ\u{313}~ῐ~ις", "ᾰ\u{313}~ῐ~ντε", "ᾰ\u{313}~ῐ~ντες";
        vocative_feminine: "ᾰ\u{313}~ῐ~ῑσᾰ", "ᾰ\u{313}~ῐ~ισᾱ", "ᾰ\u{313}~ῐ~ῑσαι";
        vocative_neuter: "ᾰ\u{313}~ῐ~ν", "ᾰ\u{313}~ῐ~ντε", "ᾰ\u{313}~ῐ~ντᾰ";
        comparative: "ᾰ\u{313}~ᾰ~οτερος";
        superlative: "ᾰ\u{313}~ᾰ~οτᾰτος";
        adverb: "ᾰ\u{313}~ῐ~ντως";
    ),
    // Ἀγωγος: 5 words (ἀγωγος, ἀλαος, ἀφνειος, ἀμενηνος, ἀμβλῠωπος).
    declension!("ἀγωγος", 5, "ἀ~ος";
        nominative_masculine: "ᾰ\u{313}~ος", "ᾰ\u{313}~ω", "ᾰ\u{313}~οι";
        nominative_feminine: "ᾰ\u{313}~ος", "ᾰ\u{313}~ω", "ᾰ\u{313}~οι";
        nominative_neuter: "ᾰ\u{313}~ον", "ᾰ\u{313}~ω", "ᾰ\u{313}~ᾰ";
        genitive_masculine: "ᾰ\u{313}~οῡ", "ᾰ\u{313}~οῑν", "ᾰ\u{313}~ων";
        genitive_feminine: "ᾰ\u{313}~οῡ", "ᾰ\u{313}~οῑν", "ᾰ\u{313}~ων";
        genitive_neuter: "ᾰ\u{313}~οῡ", "ᾰ\u{313}~οῑν", "ᾰ\u{313}~ων";
        dative_masculine: "ᾰ\u{313}~ῳ", "ᾰ\u{313}~οῑν", "ᾰ\u{313}~οῑς";
        dative_feminine: "ᾰ\u{313}~ῳ", "ᾰ\u{313}~οῑν", "ᾰ\u{313}~οῑς";
        dative_neuter: "ᾰ\u{313}~ῳ", "ᾰ\u{313}~οῑν", "ᾰ\u{313}~οῑς";
        accusative_masculine: "ᾰ\u{313}~ον", "ᾰ\u{313}~ω", "ᾰ\u{313}~ους";
        accusative_feminine: "ᾰ\u{313}~ον", "ᾰ\u{313}~ω", "ᾰ\u{313}~ους";
        accusative_neuter: "ᾰ\u{313}~ον", "ᾰ\u{313}~ω", "ᾰ\u{313}~ᾰ";
        vocative_masculine: "ᾰ\u{313}~ε", "ᾰ\u{313}~ω", "ᾰ\u{313}~οι";
        vocative_feminine: "ᾰ\u{313}~ε", "ᾰ\u{313}~ω", "ᾰ\u{313}~οι";
        vocative_neuter: "ᾰ\u{313}~ον", "ᾰ\u{313}~ω", "ᾰ\u{313}~ᾰ";
        comparative: "ᾰ\u{313}~οτερος";
        superlative: "ᾰ\u{313}~οτᾰτος";
        adverb: "ᾰ\u{313}~ως";
    ),
    // Ἀνομωτᾰτος: 5 words (ἀληθεστᾰτος, ἀφᾰνεστᾰτος, ἀνομωτᾰτος, ἀοκνοτᾰτος, ἀχρειοτᾰτος).
    declension!("ἀνομωτᾰτος", 5, "ἀ~ος";
        nominative_masculine: "ᾰ\u{313}~ος", "ᾰ\u{313}~ω", "ᾰ\u{313}~οι";
        nominative_feminine: "ᾰ\u{313}~η", "ᾰ\u{313}~ᾱ", "ᾰ\u{313}~αι";
        nominative_neuter: "ᾰ\u{313}~ον", "ᾰ\u{313}~ω", "ᾰ\u{313}~ᾰ";
        genitive_masculine: "ᾰ\u{313}~ου", "ᾰ\u{313}~οιν", "ᾰ\u{313}~ων";
        genitive_feminine: "ᾰ\u{313}~ης", "ᾰ\u{313}~αιν", "ᾰ\u{313}~ων";
        genitive_neuter: "ᾰ\u{313}~ου", "ᾰ\u{313}~οιν", "ᾰ\u{313}~ων";
        dative_masculine: "ᾰ\u{313}~ῳ", "ᾰ\u{313}~οιν", "ᾰ\u{313}~οις";
        dative_feminine: "ᾰ\u{313}~ῃ", "ᾰ\u{313}~αιν", "ᾰ\u{313}~αις";
        dative_neuter: "ᾰ\u{313}~ῳ", "ᾰ\u{313}~οιν", "ᾰ\u{313}~οις";
        accusative_masculine: "ᾰ\u{313}~ον", "ᾰ\u{313}~ω", "ᾰ\u{313}~ους";
        accusative_feminine: "ᾰ\u{313}~ην", "ᾰ\u{313}~ᾱ", "ᾰ\u{313}~ᾱς";
        accusative_neuter: "ᾰ\u{313}~ον", "ᾰ\u{313}~ω", "ᾰ\u{313}~ᾰ";
        vocative_masculine: "ᾰ\u{313}~ε", "ᾰ\u{313}~ω", "ᾰ\u{313}~οι";
        vocative_feminine: "ᾰ\u{313}~η", "ᾰ\u{313}~ᾱ", "ᾰ\u{313}~αι";
        vocative_neuter: "ᾰ\u{313}~ον", "ᾰ\u{313}~ω", "ᾰ\u{313}~ᾰ";
        comparative: "ᾰ\u{313}~ωτερος";
        superlative: "ᾰ\u{313}~ωτᾰτος";
        adverb: "ᾰ\u{313}~ᾰ";
    ),
    // Ὑπατος: 5 words (ὑπατος, ὑπογεγραμμενος, ὑπαρχομενος, ὑψηλοτατος, ὑπανδρος).
    declension!("ὑπατος", 5, "ὑ~α~ος";
        nominative_masculine: "ῠ\u{314}~ᾰ~ος", "ῠ\u{314}~ᾰ~ω", "ῠ\u{314}~ᾰ~οι";
        nominative_feminine: "ῠ\u{314}~ᾰ~η", "ῠ\u{314}~ᾰ~ᾱ", "ῠ\u{314}~ᾰ~αι";
        nominative_neuter: "ῠ\u{314}~ᾰ~ον", "ῠ\u{314}~ᾰ~ω", "ῠ\u{314}~ᾰ~ᾰ";
        genitive_masculine: "ῠ\u{314}~ᾰ~ου", "ῠ\u{314}~ᾰ~οιν", "ῠ\u{314}~ᾰ~ων";
        genitive_feminine: "ῠ\u{314}~ᾰ~ης", "ῠ\u{314}~ᾰ~αιν", "ῠ\u{314}~ᾰ~ων";
        genitive_neuter: "ῠ\u{314}~ᾰ~ου", "ῠ\u{314}~ᾰ~οιν", "ῠ\u{314}~ᾰ~ων";
        dative_masculine: "ῠ\u{314}~ᾰ~ῳ", "ῠ\u{314}~ᾰ~οιν", "ῠ\u{314}~ᾰ~οις";
        dative_feminine: "ῠ\u{314}~ᾰ~ῃ", "ῠ\u{314}~ᾰ~αιν", "ῠ\u{314}~ᾰ~αις";
        dative_neuter: "ῠ\u{314}~ᾰ~ῳ", "ῠ\u{314}~ᾰ~οιν", "ῠ\u{314}~ᾰ~οις";
        accusative_masculine: "ῠ\u{314}~ᾰ~ον", "ῠ\u{314}~ᾰ~ω", "ῠ\u{314}~ᾰ~ους";
        accusative_feminine: "ῠ\u{314}~ᾰ~ην", "ῠ\u{314}~ᾰ~ᾱ", "ῠ\u{314}~ᾰ~ᾱς";
        accusative_neuter: "ῠ\u{314}~ᾰ~ον", "ῠ\u{314}~ᾰ~ω", "ῠ\u{314}~ᾰ~ᾰ";
        vocative_masculine: "ῠ\u{314}~ᾰ~ε", "ῠ\u{314}~ᾰ~ω", "ῠ\u{314}~ᾰ~οι";
        vocative_feminine: "ῠ\u{314}~ᾰ~η", "ῠ\u{314}~ᾰ~ᾱ", "ῠ\u{314}~ᾰ~αι";
        vocative_neuter: "ῠ\u{314}~ᾰ~ον", "ῠ\u{314}~ᾰ~ω", "ῠ\u{314}~ᾰ~ᾰ";
        comparative: "ῠ\u{314}~ᾰ~ωτερος";
        superlative: "ῠ\u{314}~ᾰ~ωτᾰτος";
        adverb: "ῠ\u{314}~ᾰ~ως";
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
    // Ἀγαπητος: 4 words (ἀγαθος, ἀγαπητος, ἀλᾰπαδνος, ἀσπαστος).
    declension!("ἀγαπητος", 4, "ἀ~α~ος";
        nominative_masculine: "ᾰ\u{313}~ᾰ~ος", "ᾰ\u{313}~ᾰ~ω", "ᾰ\u{313}~ᾰ~οι";
        nominative_feminine: "ᾰ\u{313}~ᾰ~η", "ᾰ\u{313}~ᾰ~ᾱ", "ᾰ\u{313}~ᾰ~αι";
        nominative_neuter: "ᾰ\u{313}~ᾰ~ον", "ᾰ\u{313}~ᾰ~ω", "ᾰ\u{313}~ᾰ~ᾰ";
        genitive_masculine: "ᾰ\u{313}~ᾰ~οῡ", "ᾰ\u{313}~ᾰ~οῑν", "ᾰ\u{313}~ᾰ~ων";
        genitive_feminine: "ᾰ\u{313}~ᾰ~ης", "ᾰ\u{313}~ᾰ~αῑν", "ᾰ\u{313}~ᾰ~ων";
        genitive_neuter: "ᾰ\u{313}~ᾰ~οῡ", "ᾰ\u{313}~ᾰ~οῑν", "ᾰ\u{313}~ᾰ~ων";
        dative_masculine: "ᾰ\u{313}~ᾰ~ῳ", "ᾰ\u{313}~ᾰ~οῑν", "ᾰ\u{313}~ᾰ~οῑς";
        dative_feminine: "ᾰ\u{313}~ᾰ~ῃ", "ᾰ\u{313}~ᾰ~αῑν", "ᾰ\u{313}~ᾰ~αῑς";
        dative_neuter: "ᾰ\u{313}~ᾰ~ῳ", "ᾰ\u{313}~ᾰ~οῑν", "ᾰ\u{313}~ᾰ~οῑς";
        accusative_masculine: "ᾰ\u{313}~ᾰ~ον", "ᾰ\u{313}~ᾰ~ω", "ᾰ\u{313}~ᾰ~ους";
        accusative_feminine: "ᾰ\u{313}~ᾰ~ην", "ᾰ\u{313}~ᾰ~ᾱ", "ᾰ\u{313}~ᾰ~ᾱς";
        accusative_neuter: "ᾰ\u{313}~ᾰ~ον", "ᾰ\u{313}~ᾰ~ω", "ᾰ\u{313}~ᾰ~ᾰ";
        vocative_masculine: "ᾰ\u{313}~ᾰ~ε", "ᾰ\u{313}~ᾰ~ω", "ᾰ\u{313}~ᾰ~οι";
        vocative_feminine: "ᾰ\u{313}~ᾰ~η", "ᾰ\u{313}~ᾰ~ᾱ", "ᾰ\u{313}~ᾰ~αι";
        vocative_neuter: "ᾰ\u{313}~ᾰ~ον", "ᾰ\u{313}~ᾰ~ω", "ᾰ\u{313}~ᾰ~ᾰ";
        comparative: "ᾰ\u{313}~ᾰ~οτερος";
        superlative: "ᾰ\u{313}~ᾰ~οτᾰτος";
        adverb: "ᾰ\u{313}~ᾰ~ως";
    ),
    // Ἀρτιος: 4 words (ἀγριος, ἀξιος, ἀρτιος, ἀλλοτριος).
    declension!("ἀρτιος", 4, "ἀ~ιος";
        nominative_masculine: "ᾰ\u{313}~ῐος", "ᾰ\u{313}~ῐω", "ᾰ\u{313}~ῐοι";
        nominative_feminine: "ᾰ\u{313}~ῐᾱ", "ᾰ\u{313}~ῐᾱ", "ᾰ\u{313}~ῐαι";
        nominative_neuter: "ᾰ\u{313}~ῐον", "ᾰ\u{313}~ῐω", "ᾰ\u{313}~ῐᾰ";
        genitive_masculine: "ᾰ\u{313}~ῐου", "ᾰ\u{313}~ῐοιν", "ᾰ\u{313}~ῐων";
        genitive_feminine: "ᾰ\u{313}~ῐᾱς", "ᾰ\u{313}~ῐαιν", "ᾰ\u{313}~ῐων";
        genitive_neuter: "ᾰ\u{313}~ῐου", "ᾰ\u{313}~ῐοιν", "ᾰ\u{313}~ῐων";
        dative_masculine: "ᾰ\u{313}~ῐῳ", "ᾰ\u{313}~ῐοιν", "ᾰ\u{313}~ῐοις";
        dative_feminine: "ᾰ\u{313}~ῐᾳ", "ᾰ\u{313}~ῐαιν", "ᾰ\u{313}~ῐαις";
        dative_neuter: "ᾰ\u{313}~ῐῳ", "ᾰ\u{313}~ῐοιν", "ᾰ\u{313}~ῐοις";
        accusative_masculine: "ᾰ\u{313}~ῐον", "ᾰ\u{313}~ῐω", "ᾰ\u{313}~ῐους";
        accusative_feminine: "ᾰ\u{313}~ῐᾱν", "ᾰ\u{313}~ῐᾱ", "ᾰ\u{313}~ῐᾱς";
        accusative_neuter: "ᾰ\u{313}~ῐον", "ᾰ\u{313}~ῐω", "ᾰ\u{313}~ῐᾰ";
        vocative_masculine: "ᾰ\u{313}~ῐε", "ᾰ\u{313}~ῐω", "ᾰ\u{313}~ῐοι";
        vocative_feminine: "ᾰ\u{313}~ῐᾱ", "ᾰ\u{313}~ῐᾱ", "ᾰ\u{313}~ῐαι";
        vocative_neuter: "ᾰ\u{313}~ῐον", "ᾰ\u{313}~ῐω", "ᾰ\u{313}~ῐᾰ";
        comparative: "ᾰ\u{313}~ῐωτερος";
        superlative: "ᾰ\u{313}~ῐωτᾰτος";
        adverb: "ᾰ\u{313}~ῐως";
    ),
    // Ἀμαυρος: 4 words (ἀμαυρος, ἀραιος, ἀγαυος, ἀνθηρος).
    declension!("ἀμαυρος", 4, "ἀ~ος";
        nominative_masculine: "ᾰ\u{313}~ος", "ᾰ\u{313}~ω", "ᾰ\u{313}~οι";
        nominative_feminine: "ᾰ\u{313}~ᾱ", "ᾰ\u{313}~ᾱ", "ᾰ\u{313}~αι";
        nominative_neuter: "ᾰ\u{313}~ον", "ᾰ\u{313}~ω", "ᾰ\u{313}~ᾰ";
        genitive_masculine: "ᾰ\u{313}~οῡ", "ᾰ\u{313}~οῑν", "ᾰ\u{313}~ων";
        genitive_feminine: "ᾰ\u{313}~ᾱς", "ᾰ\u{313}~αῑν", "ᾰ\u{313}~ων";
        genitive_neuter: "ᾰ\u{313}~οῡ", "ᾰ\u{313}~οῑν", "ᾰ\u{313}~ων";
        dative_masculine: "ᾰ\u{313}~ῳ", "ᾰ\u{313}~οῑν", "ᾰ\u{313}~οῑς";
        dative_feminine: "ᾰ\u{313}~ᾳ", "ᾰ\u{313}~αῑν", "ᾰ\u{313}~αῑς";
        dative_neuter: "ᾰ\u{313}~ῳ", "ᾰ\u{313}~οῑν", "ᾰ\u{313}~οῑς";
        accusative_masculine: "ᾰ\u{313}~ον", "ᾰ\u{313}~ω", "ᾰ\u{313}~ους";
        accusative_feminine: "ᾰ\u{313}~ᾱν", "ᾰ\u{313}~ᾱ", "ᾰ\u{313}~ᾱς";
        accusative_neuter: "ᾰ\u{313}~ον", "ᾰ\u{313}~ω", "ᾰ\u{313}~ᾰ";
        vocative_masculine: "ᾰ\u{313}~ε", "ᾰ\u{313}~ω", "ᾰ\u{313}~οι";
        vocative_feminine: "ᾰ\u{313}~ᾱ", "ᾰ\u{313}~ᾱ", "ᾰ\u{313}~αι";
        vocative_neuter: "ᾰ\u{313}~ον", "ᾰ\u{313}~ω", "ᾰ\u{313}~ᾰ";
        comparative: "ᾰ\u{313}~οτερος";
        superlative: "ᾰ\u{313}~οτᾰτος";
        adverb: "ᾰ\u{313}~ως";
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
        comparative: "ᾰ\u{313}~ᾰ~οτερος";
        superlative: "ᾰ\u{313}~ᾰ~οτᾰτος";
        adverb: "~ῑ~ον";
    ),
    // Ἀγωνιστικος: 4 words (ἀντιθετικος, ἀντιλογητικος, ἀξιοπιστος, ἀγωνιστικος).
    declension!("ἀγωνιστικος", 4, "ἀ~ι~ι~ος";
        nominative_masculine: "ᾰ\u{313}~ῐ~ῐ~ος", "ᾰ\u{313}~ῐ~ῐ~ω", "ᾰ\u{313}~ῐ~ῐ~οι";
        nominative_feminine: "ᾰ\u{313}~ῐ~ῐ~η", "ᾰ\u{313}~ῐ~ῐ~ᾱ", "ᾰ\u{313}~ῐ~ῐ~αι";
        nominative_neuter: "ᾰ\u{313}~ῐ~ῐ~ον", "ᾰ\u{313}~ῐ~ῐ~ω", "ᾰ\u{313}~ῐ~ῐ~ᾰ";
        genitive_masculine: "ᾰ\u{313}~ῐ~ῐ~οῡ", "ᾰ\u{313}~ῐ~ῐ~οῑν", "ᾰ\u{313}~ῐ~ῐ~ων";
        genitive_feminine: "ᾰ\u{313}~ῐ~ῐ~ης", "ᾰ\u{313}~ῐ~ῐ~αῑν", "ᾰ\u{313}~ῐ~ῐ~ων";
        genitive_neuter: "ᾰ\u{313}~ῐ~ῐ~οῡ", "ᾰ\u{313}~ῐ~ῐ~οῑν", "ᾰ\u{313}~ῐ~ῐ~ων";
        dative_masculine: "ᾰ\u{313}~ῐ~ῐ~ῳ", "ᾰ\u{313}~ῐ~ῐ~οῑν", "ᾰ\u{313}~ῐ~ῐ~οῑς";
        dative_feminine: "ᾰ\u{313}~ῐ~ῐ~ῃ", "ᾰ\u{313}~ῐ~ῐ~αῑν", "ᾰ\u{313}~ῐ~ῐ~αῑς";
        dative_neuter: "ᾰ\u{313}~ῐ~ῐ~ῳ", "ᾰ\u{313}~ῐ~ῐ~οῑν", "ᾰ\u{313}~ῐ~ῐ~οῑς";
        accusative_masculine: "ᾰ\u{313}~ῐ~ῐ~ον", "ᾰ\u{313}~ῐ~ῐ~ω", "ᾰ\u{313}~ῐ~ῐ~ους";
        accusative_feminine: "ᾰ\u{313}~ῐ~ῐ~ην", "ᾰ\u{313}~ῐ~ῐ~ᾱ", "ᾰ\u{313}~ῐ~ῐ~ᾱς";
        accusative_neuter: "ᾰ\u{313}~ῐ~ῐ~ον", "ᾰ\u{313}~ῐ~ῐ~ω", "ᾰ\u{313}~ῐ~ῐ~ᾰ";
        vocative_masculine: "ᾰ\u{313}~ῐ~ῐ~ε", "ᾰ\u{313}~ῐ~ῐ~ω", "ᾰ\u{313}~ῐ~ῐ~οι";
        vocative_feminine: "ᾰ\u{313}~ῐ~ῐ~η", "ᾰ\u{313}~ῐ~ῐ~ᾱ", "ᾰ\u{313}~ῐ~ῐ~αι";
        vocative_neuter: "ᾰ\u{313}~ῐ~ῐ~ον", "ᾰ\u{313}~ῐ~ῐ~ω", "ᾰ\u{313}~ῐ~ῐ~ᾰ";
        comparative: "ᾰ\u{313}~ῐ~ῐ~ωτερος";
        superlative: "ᾰ\u{313}~ῐ~ῐ~ωτᾰτος";
        adverb: "ᾰ\u{313}~ῐ~ῐ~ως";
    ),
    // Δῐπλοῡς: 4 words (δῐπλοῡς, χρῡσοῡς, χᾰλκοῡς, πορφῠροῡς).
    declension!("δῐπλοῡς", 4, "~οῡς";
        nominative_masculine: "~οῡς", "~ω|~ω", "~οῑ";
        nominative_feminine: "~η", "~ᾱ", "~αῑ";
        nominative_neuter: "~οῡν", "~ω|~ω", "~ᾱ";
        genitive_masculine: "~οῡ", "~οῑν", "~ων";
        genitive_feminine: "~ης", "~αῑν", "~ων";
        genitive_neuter: "~οῡ", "~οῑν", "~ων";
        dative_masculine: "~ῳ", "~οῑν", "~οῑς";
        dative_feminine: "~ῃ", "~αῑν", "~αῑς";
        dative_neuter: "~ῳ", "~οῑν", "~οῑς";
        accusative_masculine: "~οῡν", "~ω|~ω", "~οῡς";
        accusative_feminine: "~ην", "~ᾱ", "~ᾱς";
        accusative_neuter: "~οῡν", "~ω|~ω", "~ᾱ";
        vocative_masculine: "~οῡ", "~ω|~ω", "~οῑ";
        vocative_feminine: "~η", "~ᾱ", "~αῑ";
        vocative_neuter: "~οῡν", "~ω|~ω", "~ᾱ";
        comparative: "~εωτερος";
        superlative: "~εωτᾰτος";
        adverb: "~ως";
    ),
    // Ἀγνοων: 4 words (ἀγγελων, ἀγνοων, ἀντερων, ἀδῐκων).
    declension!("ἀγνοων", 4, "ἀ~ων";
        nominative_masculine: "ᾰ\u{313}~ων", "ᾰ\u{313}~οῡντε", "ᾰ\u{313}~οῡντες";
        nominative_feminine: "ᾰ\u{313}~οῡσᾰ", "ᾰ\u{313}~ουσᾱ", "ᾰ\u{313}~οῡσαι";
        nominative_neuter: "ᾰ\u{313}~οῡν", "ᾰ\u{313}~οῡντε", "ᾰ\u{313}~οῡντᾰ";
        genitive_masculine: "ᾰ\u{313}~οῡντος", "ᾰ\u{313}~ουντοιν", "ᾰ\u{313}~ουντων";
        genitive_feminine: "ᾰ\u{313}~ουσης", "ᾰ\u{313}~ουσαιν", "ᾰ\u{313}~ουσων";
        genitive_neuter: "ᾰ\u{313}~οῡντος", "ᾰ\u{313}~ουντοιν", "ᾰ\u{313}~ουντων";
        dative_masculine: "ᾰ\u{313}~οῡντῐ", "ᾰ\u{313}~ουντοιν", "ᾰ\u{313}~οῡσῐ|ᾰ\u{313}~οῡσῐν";
        dative_feminine: "ᾰ\u{313}~ουσῃ", "ᾰ\u{313}~ουσαιν", "ᾰ\u{313}~ουσαις";
        dative_neuter: "ᾰ\u{313}~οῡντῐ", "ᾰ\u{313}~ουντοιν", "ᾰ\u{313}~οῡσῐ|ᾰ\u{313}~οῡσῐν";
        accusative_masculine: "ᾰ\u{313}~οῡντᾰ", "ᾰ\u{313}~οῡντε", "ᾰ\u{313}~οῡντᾰς";
        accusative_feminine: "ᾰ\u{313}~οῡσᾰν", "ᾰ\u{313}~ουσᾱ", "ᾰ\u{313}~ουσᾱς";
        accusative_neuter: "ᾰ\u{313}~οῡν", "ᾰ\u{313}~οῡντε", "ᾰ\u{313}~οῡντᾰ";
        vocative_masculine: "ᾰ\u{313}~ων", "ᾰ\u{313}~οῡντε", "ᾰ\u{313}~οῡντες";
        vocative_feminine: "ᾰ\u{313}~οῡσᾰ", "ᾰ\u{313}~ουσᾱ", "ᾰ\u{313}~οῡσαι";
        vocative_neuter: "ᾰ\u{313}~οῡν", "ᾰ\u{313}~οῡντε", "ᾰ\u{313}~οῡντᾰ";
        comparative: "~ωτερος";
        superlative: "~ωτᾰτος";
        adverb: "ᾰ\u{313}~ουντως";
    ),
    // Δρυϊνος: 4 words (νεκυϊκος, οἰσυϊνος, δρυϊνος, συϊνος).
    declension!("δρυϊνος", 4, "~υϊ~ος";
        nominative_masculine: "~ῠῐ\u{308}~ος", "~ῠῐ\u{308}~ω", "~ῠῐ\u{308}~οι";
        nominative_feminine: "~ῠῐ\u{308}~η", "~ῠῐ\u{308}~ᾱ", "~ῠῐ\u{308}~αι";
        nominative_neuter: "~ῠῐ\u{308}~ον", "~ῠῐ\u{308}~ω", "~ῠῐ\u{308}~ᾰ";
        genitive_masculine: "~ῠῐ\u{308}~ου", "~ῠῐ\u{308}~οιν", "~ῠῐ\u{308}~ων";
        genitive_feminine: "~ῠῐ\u{308}~ης", "~ῠῐ\u{308}~αιν", "~ῠῐ\u{308}~ων";
        genitive_neuter: "~ῠῐ\u{308}~ου", "~ῠῐ\u{308}~οιν", "~ῠῐ\u{308}~ων";
        dative_masculine: "~ῠῐ\u{308}~ῳ", "~ῠῐ\u{308}~οιν", "~ῠῐ\u{308}~οις";
        dative_feminine: "~ῠῐ\u{308}~ῃ", "~ῠῐ\u{308}~αιν", "~ῠῐ\u{308}~αις";
        dative_neuter: "~ῠῐ\u{308}~ῳ", "~ῠῐ\u{308}~οιν", "~ῠῐ\u{308}~οις";
        accusative_masculine: "~ῠῐ\u{308}~ον", "~ῠῐ\u{308}~ω", "~ῠῐ\u{308}~ους";
        accusative_feminine: "~ῠῐ\u{308}~ην", "~ῠῐ\u{308}~ᾱ", "~ῠῐ\u{308}~ᾱς";
        accusative_neuter: "~ῠῐ\u{308}~ον", "~ῠῐ\u{308}~ω", "~ῠῐ\u{308}~ᾰ";
        vocative_masculine: "~ῠῐ\u{308}~ε", "~ῠῐ\u{308}~ω", "~ῠῐ\u{308}~οι";
        vocative_feminine: "~ῠῐ\u{308}~η", "~ῠῐ\u{308}~ᾱ", "~ῠῐ\u{308}~αι";
        vocative_neuter: "~ῠῐ\u{308}~ον", "~ῠῐ\u{308}~ω", "~ῠῐ\u{308}~ᾰ";
        comparative: "~ῠῐ\u{308}~ωτερος";
        superlative: "~ῠῐ\u{308}~ωτᾰτος";
        adverb: "~ῠῐ\u{308}~ως";
    ),
    // Πεπτεως: 4 words (πεπτεως, πεπτηως, τετληως, κεκοτηως).
    declension!("πεπτεως", 4, "~ως";
        nominative_masculine: "~ως", "~οτε", "~οτες";
        nominative_feminine: "~υιᾰ", "~υιᾱ", "~υιαι";
        nominative_neuter: "~ος", "~οτε", "~οτᾰ";
        genitive_masculine: "~οτος", "~οτοιν", "~οτων";
        genitive_feminine: "~υιᾱς", "~υιαιν", "~υιων";
        genitive_neuter: "~οτος", "~οτοιν", "~οτων";
        dative_masculine: "~οτῐ", "~οτοιν", "~οσῐ|~οσῐν";
        dative_feminine: "~υιᾳ", "~υιαιν", "~υιαις";
        dative_neuter: "~οτῐ", "~οτοιν", "~οσῐ|~οσῐν";
        accusative_masculine: "~οτᾰ", "~οτε", "~οτᾰς";
        accusative_feminine: "~υιᾰν", "~υιᾱ", "~υιᾱς";
        accusative_neuter: "~ος", "~οτε", "~οτᾰ";
        vocative_masculine: "~ως", "~οτε", "~οτες";
        vocative_feminine: "~υιᾰ", "~υιᾱ", "~υιαι";
        vocative_neuter: "~ος", "~οτε", "~οτᾰ";
        comparative: "~ονεστερος";
        superlative: "~ονεστᾰτος";
        adverb: "~οτως";
    ),
    // Ἀργοτατος: 4 words (ἀηδεστατος, ἀνδρειοτατος, ἀργοτατος, ἀδελφοτατος).
    declension!("ἀργοτατος", 4, "ἀ~α~ος";
        nominative_masculine: "ᾰ\u{313}~ᾰ~ος", "ᾰ\u{313}~ᾰ~ω", "ᾰ\u{313}~ᾰ~οι";
        nominative_feminine: "ᾰ\u{313}~ᾰ~η", "ᾰ\u{313}~ᾰ~ᾱ", "ᾰ\u{313}~ᾰ~αι";
        nominative_neuter: "ᾰ\u{313}~ᾰ~ον", "ᾰ\u{313}~ᾰ~ω", "ᾰ\u{313}~ᾰ~ᾰ";
        genitive_masculine: "ᾰ\u{313}~ᾰ~ου", "ᾰ\u{313}~ᾰ~οιν", "ᾰ\u{313}~ᾰ~ων";
        genitive_feminine: "ᾰ\u{313}~ᾰ~ης", "ᾰ\u{313}~ᾰ~αιν", "ᾰ\u{313}~ᾰ~ων";
        genitive_neuter: "ᾰ\u{313}~ᾰ~ου", "ᾰ\u{313}~ᾰ~οιν", "ᾰ\u{313}~ᾰ~ων";
        dative_masculine: "ᾰ\u{313}~ᾰ~ῳ", "ᾰ\u{313}~ᾰ~οιν", "ᾰ\u{313}~ᾰ~οις";
        dative_feminine: "ᾰ\u{313}~ᾰ~ῃ", "ᾰ\u{313}~ᾰ~αιν", "ᾰ\u{313}~ᾰ~αις";
        dative_neuter: "ᾰ\u{313}~ᾰ~ῳ", "ᾰ\u{313}~ᾰ~οιν", "ᾰ\u{313}~ᾰ~οις";
        accusative_masculine: "ᾰ\u{313}~ᾰ~ον", "ᾰ\u{313}~ᾰ~ω", "ᾰ\u{313}~ᾰ~ους";
        accusative_feminine: "ᾰ\u{313}~ᾰ~ην", "ᾰ\u{313}~ᾰ~ᾱ", "ᾰ\u{313}~ᾰ~ᾱς";
        accusative_neuter: "ᾰ\u{313}~ᾰ~ον", "ᾰ\u{313}~ᾰ~ω", "ᾰ\u{313}~ᾰ~ᾰ";
        vocative_masculine: "ᾰ\u{313}~ᾰ~ε", "ᾰ\u{313}~ᾰ~ω", "ᾰ\u{313}~ᾰ~οι";
        vocative_feminine: "ᾰ\u{313}~ᾰ~η", "ᾰ\u{313}~ᾰ~ᾱ", "ᾰ\u{313}~ᾰ~αι";
        vocative_neuter: "ᾰ\u{313}~ᾰ~ον", "ᾰ\u{313}~ᾰ~ω", "ᾰ\u{313}~ᾰ~ᾰ";
        comparative: "ᾰ\u{313}~ᾰ~ωτερος";
        superlative: "ᾰ\u{313}~ᾰ~ωτᾰτος";
        adverb: "ᾰ\u{313}~ᾰ~ᾰ";
    ),
    // Ἀκυρος: 4 words (ἀσυλος, ἀπυγος, ἀχρυσος, ἀκυρος).
    declension!("ἀκυρος", 4, "ἀ~υ~ος";
        nominative_masculine: "ᾰ\u{313}~ῡ~ος", "ᾰ\u{313}~ῡ~ω", "ᾰ\u{313}~ῡ~οι";
        nominative_feminine: "ᾰ\u{313}~ῡ~ος", "ᾰ\u{313}~ῡ~ω", "ᾰ\u{313}~ῡ~οι";
        nominative_neuter: "ᾰ\u{313}~ῡ~ον", "ᾰ\u{313}~ῡ~ω", "ᾰ\u{313}~ῡ~ᾰ";
        genitive_masculine: "ᾰ\u{313}~ῡ~ου", "ᾰ\u{313}~ῡ~οιν", "ᾰ\u{313}~ῡ~ων";
        genitive_feminine: "ᾰ\u{313}~ῡ~ου", "ᾰ\u{313}~ῡ~οιν", "ᾰ\u{313}~ῡ~ων";
        genitive_neuter: "ᾰ\u{313}~ῡ~ου", "ᾰ\u{313}~ῡ~οιν", "ᾰ\u{313}~ῡ~ων";
        dative_masculine: "ᾰ\u{313}~ῡ~ῳ", "ᾰ\u{313}~ῡ~οιν", "ᾰ\u{313}~ῡ~οις";
        dative_feminine: "ᾰ\u{313}~ῡ~ῳ", "ᾰ\u{313}~ῡ~οιν", "ᾰ\u{313}~ῡ~οις";
        dative_neuter: "ᾰ\u{313}~ῡ~ῳ", "ᾰ\u{313}~ῡ~οιν", "ᾰ\u{313}~ῡ~οις";
        accusative_masculine: "ᾰ\u{313}~ῡ~ον", "ᾰ\u{313}~ῡ~ω", "ᾰ\u{313}~ῡ~ους";
        accusative_feminine: "ᾰ\u{313}~ῡ~ον", "ᾰ\u{313}~ῡ~ω", "ᾰ\u{313}~ῡ~ους";
        accusative_neuter: "ᾰ\u{313}~ῡ~ον", "ᾰ\u{313}~ῡ~ω", "ᾰ\u{313}~ῡ~ᾰ";
        vocative_masculine: "ᾰ\u{313}~ῡ~ε", "ᾰ\u{313}~ῡ~ω", "ᾰ\u{313}~ῡ~οι";
        vocative_feminine: "ᾰ\u{313}~ῡ~ε", "ᾰ\u{313}~ῡ~ω", "ᾰ\u{313}~ῡ~οι";
        vocative_neuter: "ᾰ\u{313}~ῡ~ον", "ᾰ\u{313}~ῡ~ω", "ᾰ\u{313}~ῡ~ᾰ";
        comparative: "ᾰ\u{313}~ῡ~οτερος";
        superlative: "ᾰ\u{313}~ῡ~οτᾰτος";
        adverb: "ᾰ\u{313}~ῡ~ως";
    ),
    // Ἰταμος: 4 words (ἰταμος, ἰσαιτατος, ἰσοτατος, ἰσωτατος).
    declension!("ἰταμος", 4, "ἰ~α~ος";
        nominative_masculine: "ῐ\u{313}~ᾰ~ος", "ῐ\u{313}~ᾰ~ω", "ῐ\u{313}~ᾰ~οι";
        nominative_feminine: "ῐ\u{313}~ᾰ~η", "ῐ\u{313}~ᾰ~ᾱ", "ῐ\u{313}~ᾰ~αι";
        nominative_neuter: "ῐ\u{313}~ᾰ~ον", "ῐ\u{313}~ᾰ~ω", "ῐ\u{313}~ᾰ~ᾰ";
        genitive_masculine: "ῐ\u{313}~ᾰ~ου", "ῐ\u{313}~ᾰ~οιν", "ῐ\u{313}~ᾰ~ων";
        genitive_feminine: "ῐ\u{313}~ᾰ~ης", "ῐ\u{313}~ᾰ~αιν", "ῐ\u{313}~ᾰ~ων";
        genitive_neuter: "ῐ\u{313}~ᾰ~ου", "ῐ\u{313}~ᾰ~οιν", "ῐ\u{313}~ᾰ~ων";
        dative_masculine: "ῐ\u{313}~ᾰ~ῳ", "ῐ\u{313}~ᾰ~οιν", "ῐ\u{313}~ᾰ~οις";
        dative_feminine: "ῐ\u{313}~ᾰ~ῃ", "ῐ\u{313}~ᾰ~αιν", "ῐ\u{313}~ᾰ~αις";
        dative_neuter: "ῐ\u{313}~ᾰ~ῳ", "ῐ\u{313}~ᾰ~οιν", "ῐ\u{313}~ᾰ~οις";
        accusative_masculine: "ῐ\u{313}~ᾰ~ον", "ῐ\u{313}~ᾰ~ω", "ῐ\u{313}~ᾰ~ους";
        accusative_feminine: "ῐ\u{313}~ᾰ~ην", "ῐ\u{313}~ᾰ~ᾱ", "ῐ\u{313}~ᾰ~ᾱς";
        accusative_neuter: "ῐ\u{313}~ᾰ~ον", "ῐ\u{313}~ᾰ~ω", "ῐ\u{313}~ᾰ~ᾰ";
        vocative_masculine: "ῐ\u{313}~ᾰ~ε", "ῐ\u{313}~ᾰ~ω", "ῐ\u{313}~ᾰ~οι";
        vocative_feminine: "ῐ\u{313}~ᾰ~η", "ῐ\u{313}~ᾰ~ᾱ", "ῐ\u{313}~ᾰ~αι";
        vocative_neuter: "ῐ\u{313}~ᾰ~ον", "ῐ\u{313}~ᾰ~ω", "ῐ\u{313}~ᾰ~ᾰ";
        comparative: "ᾰ\u{313}~ᾰ~οτερος";
        superlative: "ᾰ\u{313}~ᾰ~οτᾰτος";
        adverb: "ῐ\u{313}~ᾰ~ᾰ";
    ),
    // Πρωτος: 3 words (πρωτος, κᾰλλῐστος, ἀγχιστος).
    declension!("πρωτος", 3, "~ος";
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
        superlative: "~ῐστος";
        adverb: "~ον";
    ),
    // Βαιος: 3 words (πᾰλαιος, βαιος, ἠβαιος).
    declension!("βαιος", 3, "~ος";
        nominative_masculine: "~ος", "~ω", "~οι";
        nominative_feminine: "~ᾱ", "~ᾱ", "~αι";
        nominative_neuter: "~ον", "~ω", "~ᾰ";
        genitive_masculine: "~οῡ", "~οῑν", "~ων";
        genitive_feminine: "~ᾱς", "~αῑν", "~ων";
        genitive_neuter: "~οῡ", "~οῑν", "~ων";
        dative_masculine: "~ῳ", "~οῑν", "~οῑς";
        dative_feminine: "~ᾳ", "~αῑν", "~αῑς";
        dative_neuter: "~ῳ", "~οῑν", "~οῑς";
        accusative_masculine: "~ον", "~ω", "~ους";
        accusative_feminine: "~ᾱν", "~ᾱ", "~ᾱς";
        accusative_neuter: "~ον", "~ω", "~ᾰ";
        vocative_masculine: "~ε", "~ω", "~οι";
        vocative_feminine: "~ᾱ", "~ᾱ", "~αι";
        vocative_neuter: "~ον", "~ω", "~ᾰ";
        comparative: "~οτερος";
        superlative: "~οτᾰτος";
        adverb: "~ον";
    ),
    // Θηλῠς: 3 words (ἡμῐσῠς, θηλῠς, πρεσβῠς).
    declension!("θηλῠς", 3, "~ῠς";
        nominative_masculine: "~ῠς", "~ει", "~εις";
        nominative_feminine: "~ειᾰ", "~ειᾱ", "~ειαι";
        nominative_neuter: "~ῠ", "~ει", "~η|~εᾰ";
        genitive_masculine: "~εος", "~εοιν", "~εων";
        genitive_feminine: "~ειᾱς", "~ειαιν", "~ειων";
        genitive_neuter: "~εος", "~εοιν", "~εων";
        dative_masculine: "~ει", "~εοιν", "~εσῐ|~εσῐν";
        dative_feminine: "~ειᾳ", "~ειαιν", "~ειαις";
        dative_neuter: "~ει", "~εοιν", "~εσῐ|~εσῐν";
        accusative_masculine: "~ῠν", "~ει", "~εις";
        accusative_feminine: "~ειᾰν", "~ειᾱ", "~ειᾱς";
        accusative_neuter: "~ῠ", "~ει", "~η|~εᾰ";
        vocative_masculine: "~ῠ", "~ει", "~εις";
        vocative_feminine: "~ειᾰ", "~ειᾱ", "~ειαι";
        vocative_neuter: "~ῠ", "~ει", "~η|~εᾰ";
        comparative: "~ῠτερος";
        superlative: "~ῠτᾰτος";
        adverb: "~εως";
    ),
    // Δῐᾱνταῑος: 3 words (χῠδαῑος, ᾰμορβαῑος, δῐᾱνταῑος).
    declension!("δῐᾱνταῑος", 3, "~ῑος";
        nominative_masculine: "~ῑος", "~ιω", "~ῑοι";
        nominative_feminine: "~ῑος", "~ιω", "~ῑοι";
        nominative_neuter: "~ῑον", "~ιω", "~ῑᾰ";
        genitive_masculine: "~ιου", "~ιοιν", "~ιων";
        genitive_feminine: "~ιου", "~ιοιν", "~ιων";
        genitive_neuter: "~ιου", "~ιοιν", "~ιων";
        dative_masculine: "~ιῳ", "~ιοιν", "~ιοις";
        dative_feminine: "~ιῳ", "~ιοιν", "~ιοις";
        dative_neuter: "~ιῳ", "~ιοιν", "~ιοις";
        accusative_masculine: "~ῑον", "~ιω", "~ιους";
        accusative_feminine: "~ῑον", "~ιω", "~ιους";
        accusative_neuter: "~ῑον", "~ιω", "~ῑᾰ";
        vocative_masculine: "~ῑε", "~ιω", "~ῑοι";
        vocative_feminine: "~ῑε", "~ιω", "~ῑοι";
        vocative_neuter: "~ῑον", "~ιω", "~ῑᾰ";
        comparative: "~ιοτερος";
        superlative: "~ιοτᾰτος";
        adverb: "~ιως";
    ),
    // Ἰσοδυναμος: 3 words (ἰχθυοφαγος, ἰχθυφαγος, ἰσοδυναμος).
    declension!("ἰσοδυναμος", 3, "ἰ~υ~α~ος";
        nominative_masculine: "ῐ\u{313}~ῠ~ᾰ~ος", "ῐ\u{313}~ῠ~ᾰ~ω", "ῐ\u{313}~ῠ~ᾰ~οι";
        nominative_feminine: "ῐ\u{313}~ῠ~ᾰ~ος", "ῐ\u{313}~ῠ~ᾰ~ω", "ῐ\u{313}~ῠ~ᾰ~οι";
        nominative_neuter: "ῐ\u{313}~ῠ~ᾰ~ον", "ῐ\u{313}~ῠ~ᾰ~ω", "ῐ\u{313}~ῠ~ᾰ~ᾰ";
        genitive_masculine: "ῐ\u{313}~ῠ~ᾰ~ου", "ῐ\u{313}~ῠ~ᾰ~οιν", "ῐ\u{313}~ῠ~ᾰ~ων";
        genitive_feminine: "ῐ\u{313}~ῠ~ᾰ~ου", "ῐ\u{313}~ῠ~ᾰ~οιν", "ῐ\u{313}~ῠ~ᾰ~ων";
        genitive_neuter: "ῐ\u{313}~ῠ~ᾰ~ου", "ῐ\u{313}~ῠ~ᾰ~οιν", "ῐ\u{313}~ῠ~ᾰ~ων";
        dative_masculine: "ῐ\u{313}~ῠ~ᾰ~ῳ", "ῐ\u{313}~ῠ~ᾰ~οιν", "ῐ\u{313}~ῠ~ᾰ~οις";
        dative_feminine: "ῐ\u{313}~ῠ~ᾰ~ῳ", "ῐ\u{313}~ῠ~ᾰ~οιν", "ῐ\u{313}~ῠ~ᾰ~οις";
        dative_neuter: "ῐ\u{313}~ῠ~ᾰ~ῳ", "ῐ\u{313}~ῠ~ᾰ~οιν", "ῐ\u{313}~ῠ~ᾰ~οις";
        accusative_masculine: "ῐ\u{313}~ῠ~ᾰ~ον", "ῐ\u{313}~ῠ~ᾰ~ω", "ῐ\u{313}~ῠ~ᾰ~ους";
        accusative_feminine: "ῐ\u{313}~ῠ~ᾰ~ον", "ῐ\u{313}~ῠ~ᾰ~ω", "ῐ\u{313}~ῠ~ᾰ~ους";
        accusative_neuter: "ῐ\u{313}~ῠ~ᾰ~ον", "ῐ\u{313}~ῠ~ᾰ~ω", "ῐ\u{313}~ῠ~ᾰ~ᾰ";
        vocative_masculine: "ῐ\u{313}~ῠ~ᾰ~ε", "ῐ\u{313}~ῠ~ᾰ~ω", "ῐ\u{313}~ῠ~ᾰ~οι";
        vocative_feminine: "ῐ\u{313}~ῠ~ᾰ~ε", "ῐ\u{313}~ῠ~ᾰ~ω", "ῐ\u{313}~ῠ~ᾰ~οι";
        vocative_neuter: "ῐ\u{313}~ῠ~ᾰ~ον", "ῐ\u{313}~ῠ~ᾰ~ω", "ῐ\u{313}~ῠ~ᾰ~ᾰ";
        comparative: "ῐ\u{313}~ῠ~ᾰ~ωτερος";
        superlative: "ῐ\u{313}~ῠ~ᾰ~ωτᾰτος";
        adverb: "ῐ\u{313}~ῠ~ᾰ~ως";
    ),
    // Ὑστερος: 3 words (ὑστερος, ὑπερτερος, ὑψηλοτερος).
    declension!("ὑστερος", 3, "ὑ~ερος";
        nominative_masculine: "ῠ\u{314}~ερος", "ῠ\u{314}~ερω", "ῠ\u{314}~εροι";
        nominative_feminine: "ῠ\u{314}~ερᾱ", "ῠ\u{314}~ερᾱ", "ῠ\u{314}~εραι";
        nominative_neuter: "ῠ\u{314}~ερον", "ῠ\u{314}~ερω", "ῠ\u{314}~ερᾰ";
        genitive_masculine: "ῠ\u{314}~ερου", "ῠ\u{314}~εροιν", "ῠ\u{314}~ερων";
        genitive_feminine: "ῠ\u{314}~ερᾱς", "ῠ\u{314}~εραιν", "ῠ\u{314}~ερων";
        genitive_neuter: "ῠ\u{314}~ερου", "ῠ\u{314}~εροιν", "ῠ\u{314}~ερων";
        dative_masculine: "ῠ\u{314}~ερῳ", "ῠ\u{314}~εροιν", "ῠ\u{314}~εροις";
        dative_feminine: "ῠ\u{314}~ερᾳ", "ῠ\u{314}~εραιν", "ῠ\u{314}~εραις";
        dative_neuter: "ῠ\u{314}~ερῳ", "ῠ\u{314}~εροιν", "ῠ\u{314}~εροις";
        accusative_masculine: "ῠ\u{314}~ερον", "ῠ\u{314}~ερω", "ῠ\u{314}~ερους";
        accusative_feminine: "ῠ\u{314}~ερᾱν", "ῠ\u{314}~ερᾱ", "ῠ\u{314}~ερᾱς";
        accusative_neuter: "ῠ\u{314}~ερον", "ῠ\u{314}~ερω", "ῠ\u{314}~ερᾰ";
        vocative_masculine: "ῠ\u{314}~ερε", "ῠ\u{314}~ερω", "ῠ\u{314}~εροι";
        vocative_feminine: "ῠ\u{314}~ερᾱ", "ῠ\u{314}~ερᾱ", "ῠ\u{314}~εραι";
        vocative_neuter: "ῠ\u{314}~ερον", "ῠ\u{314}~ερω", "ῠ\u{314}~ερᾰ";
        comparative: "ῠ\u{314}~ερος";
        superlative: "ῠ\u{314}~ᾰτος";
        adverb: "ῠ\u{314}~ερον";
    ),
    // Ἀκηριος: 3 words (ἀποφωλιος, ἀεκουσιος, ἀκηριος).
    declension!("ἀκηριος", 3, "ἀ~ιος";
        nominative_masculine: "ᾰ\u{313}~ῐος", "ᾰ\u{313}~ῐω", "ᾰ\u{313}~ῐοι";
        nominative_feminine: "ᾰ\u{313}~ῐος", "ᾰ\u{313}~ῐω", "ᾰ\u{313}~ῐοι";
        nominative_neuter: "ᾰ\u{313}~ῐον", "ᾰ\u{313}~ῐω", "ᾰ\u{313}~ῐᾰ";
        genitive_masculine: "ᾰ\u{313}~ῐου", "ᾰ\u{313}~ῐοιν", "ᾰ\u{313}~ῐων";
        genitive_feminine: "ᾰ\u{313}~ῐου", "ᾰ\u{313}~ῐοιν", "ᾰ\u{313}~ῐων";
        genitive_neuter: "ᾰ\u{313}~ῐου", "ᾰ\u{313}~ῐοιν", "ᾰ\u{313}~ῐων";
        dative_masculine: "ᾰ\u{313}~ῐῳ", "ᾰ\u{313}~ῐοιν", "ᾰ\u{313}~ῐοις";
        dative_feminine: "ᾰ\u{313}~ῐῳ", "ᾰ\u{313}~ῐοιν", "ᾰ\u{313}~ῐοις";
        dative_neuter: "ᾰ\u{313}~ῐῳ", "ᾰ\u{313}~ῐοιν", "ᾰ\u{313}~ῐοις";
        accusative_masculine: "ᾰ\u{313}~ῐον", "ᾰ\u{313}~ῐω", "ᾰ\u{313}~ῐους";
        accusative_feminine: "ᾰ\u{313}~ῐον", "ᾰ\u{313}~ῐω", "ᾰ\u{313}~ῐους";
        accusative_neuter: "ᾰ\u{313}~ῐον", "ᾰ\u{313}~ῐω", "ᾰ\u{313}~ῐᾰ";
        vocative_masculine: "ᾰ\u{313}~ῐε", "ᾰ\u{313}~ῐω", "ᾰ\u{313}~ῐοι";
        vocative_feminine: "ᾰ\u{313}~ῐε", "ᾰ\u{313}~ῐω", "ᾰ\u{313}~ῐοι";
        vocative_neuter: "ᾰ\u{313}~ῐον", "ᾰ\u{313}~ῐω", "ᾰ\u{313}~ῐᾰ";
        comparative: "ᾰ\u{313}~ῐωτερος";
        superlative: "ᾰ\u{313}~ῐωτᾰτος";
        adverb: "ᾰ\u{313}~ῐως";
    ),
    // Ἀποκᾰταστᾰτῐκος: 3 words (ἀγανος, ἀμαλος, ἀποκᾰταστᾰτῐκος).
    declension!("ἀποκᾰταστᾰτῐκος", 3, "ἀ~α~ος";
        nominative_masculine: "ᾰ\u{313}~ᾰ~ος", "ᾰ\u{313}~ᾰ~ω", "ᾰ\u{313}~ᾰ~οι";
        nominative_feminine: "ᾰ\u{313}~ᾰ~η", "ᾰ\u{313}~ᾰ~ᾱ", "ᾰ\u{313}~ᾰ~αι";
        nominative_neuter: "ᾰ\u{313}~ᾰ~ον", "ᾰ\u{313}~ᾰ~ω", "ᾰ\u{313}~ᾰ~ᾰ";
        genitive_masculine: "ᾰ\u{313}~ᾰ~οῡ", "ᾰ\u{313}~ᾰ~οῑν", "ᾰ\u{313}~ᾰ~ων";
        genitive_feminine: "ᾰ\u{313}~ᾰ~ης", "ᾰ\u{313}~ᾰ~αῑν", "ᾰ\u{313}~ᾰ~ων";
        genitive_neuter: "ᾰ\u{313}~ᾰ~οῡ", "ᾰ\u{313}~ᾰ~οῑν", "ᾰ\u{313}~ᾰ~ων";
        dative_masculine: "ᾰ\u{313}~ᾰ~ῳ", "ᾰ\u{313}~ᾰ~οῑν", "ᾰ\u{313}~ᾰ~οῑς";
        dative_feminine: "ᾰ\u{313}~ᾰ~ῃ", "ᾰ\u{313}~ᾰ~αῑν", "ᾰ\u{313}~ᾰ~αῑς";
        dative_neuter: "ᾰ\u{313}~ᾰ~ῳ", "ᾰ\u{313}~ᾰ~οῑν", "ᾰ\u{313}~ᾰ~οῑς";
        accusative_masculine: "ᾰ\u{313}~ᾰ~ον", "ᾰ\u{313}~ᾰ~ω", "ᾰ\u{313}~ᾰ~ους";
        accusative_feminine: "ᾰ\u{313}~ᾰ~ην", "ᾰ\u{313}~ᾰ~ᾱ", "ᾰ\u{313}~ᾰ~ᾱς";
        accusative_neuter: "ᾰ\u{313}~ᾰ~ον", "ᾰ\u{313}~ᾰ~ω", "ᾰ\u{313}~ᾰ~ᾰ";
        vocative_masculine: "ᾰ\u{313}~ᾰ~ε", "ᾰ\u{313}~ᾰ~ω", "ᾰ\u{313}~ᾰ~οι";
        vocative_feminine: "ᾰ\u{313}~ᾰ~η", "ᾰ\u{313}~ᾰ~ᾱ", "ᾰ\u{313}~ᾰ~αι";
        vocative_neuter: "ᾰ\u{313}~ᾰ~ον", "ᾰ\u{313}~ᾰ~ω", "ᾰ\u{313}~ᾰ~ᾰ";
        comparative: "ᾰ\u{313}~ᾰ~ωτερος";
        superlative: "ᾰ\u{313}~ᾰ~ωτᾰτος";
        adverb: "ᾰ\u{313}~ᾰ~ως";
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
        comparative: "~ωτερος";
        superlative: "~ωτᾰτος";
        adverb: "~χως";
    ),
];
