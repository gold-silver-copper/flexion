//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's feature says):
/// `nominative_masculine` (anim, inan)
/// `nominative_feminine` (anim, inan)
/// `nominative_neuter` (anim, inan)
/// `nominative_virile` (anim, inan)
/// `nominative_nonvirile` (anim, inan)
/// `genitive_masculine` (anim, inan)
/// `genitive_feminine` (anim, inan)
/// `genitive_neuter` (anim, inan)
/// `genitive_virile` (anim, inan)
/// `genitive_nonvirile` (anim, inan)
/// `dative_masculine` (anim, inan)
/// `dative_feminine` (anim, inan)
/// `dative_neuter` (anim, inan)
/// `dative_virile` (anim, inan)
/// `dative_nonvirile` (anim, inan)
/// `accusative_masculine` (anim, inan)
/// `accusative_feminine` (anim, inan)
/// `accusative_neuter` (anim, inan)
/// `accusative_virile` (anim, inan)
/// `accusative_nonvirile` (anim, inan)
/// `instrumental_masculine` (anim, inan)
/// `instrumental_feminine` (anim, inan)
/// `instrumental_neuter` (anim, inan)
/// `instrumental_virile` (anim, inan)
/// `instrumental_nonvirile` (anim, inan)
/// `locative_masculine` (anim, inan)
/// `locative_feminine` (anim, inan)
/// `locative_neuter` (anim, inan)
/// `locative_virile` (anim, inan)
/// `locative_nonvirile` (anim, inan)
/// `comparative`
/// `superlative`
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal;
     nominative_masculine: $c0:tt, $c1:tt;
     nominative_feminine: $c2:tt, $c3:tt;
     nominative_neuter: $c4:tt, $c5:tt;
     nominative_virile: $c6:tt, $c7:tt;
     nominative_nonvirile: $c8:tt, $c9:tt;
     genitive_masculine: $c10:tt, $c11:tt;
     genitive_feminine: $c12:tt, $c13:tt;
     genitive_neuter: $c14:tt, $c15:tt;
     genitive_virile: $c16:tt, $c17:tt;
     genitive_nonvirile: $c18:tt, $c19:tt;
     dative_masculine: $c20:tt, $c21:tt;
     dative_feminine: $c22:tt, $c23:tt;
     dative_neuter: $c24:tt, $c25:tt;
     dative_virile: $c26:tt, $c27:tt;
     dative_nonvirile: $c28:tt, $c29:tt;
     accusative_masculine: $c30:tt, $c31:tt;
     accusative_feminine: $c32:tt, $c33:tt;
     accusative_neuter: $c34:tt, $c35:tt;
     accusative_virile: $c36:tt, $c37:tt;
     accusative_nonvirile: $c38:tt, $c39:tt;
     instrumental_masculine: $c40:tt, $c41:tt;
     instrumental_feminine: $c42:tt, $c43:tt;
     instrumental_neuter: $c44:tt, $c45:tt;
     instrumental_virile: $c46:tt, $c47:tt;
     instrumental_nonvirile: $c48:tt, $c49:tt;
     locative_masculine: $c50:tt, $c51:tt;
     locative_feminine: $c52:tt, $c53:tt;
     locative_neuter: $c54:tt, $c55:tt;
     locative_virile: $c56:tt, $c57:tt;
     locative_nonvirile: $c58:tt, $c59:tt;
     comparative: $c60:tt;
     superlative: $c61:tt;
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
                flexion::form!($c48),
                flexion::form!($c49),
                flexion::form!($c50),
                flexion::form!($c51),
                flexion::form!($c52),
                flexion::form!($c53),
                flexion::form!($c54),
                flexion::form!($c55),
                flexion::form!($c56),
                flexion::form!($c57),
                flexion::form!($c58),
                flexion::form!($c59),
                flexion::form!($c60),
                flexion::form!($c61),
            ]),
        }
    };
}

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `wolny`: 4,032 words.
    Wolny,
    /// Like `słaby`: 2,935 words.
    Słaby,
    /// Like `lekki`: 1,304 words.
    Lekki,
    /// Like `pierwszy`: 798 words.
    Pierwszy,
    /// Like `herbaciany`: 644 words.
    Herbaciany,
    /// Like `ciężki`: 588 words.
    Ciężki,
    /// Like `palący`: 584 words.
    Palący,
    /// Like `otwarty`: 456 words.
    Otwarty,
    /// Like `uczulony`: 440 words.
    Uczulony,
    /// Like `zajęczy`: 393 words.
    Zajęczy,
    /// Like `odpowiedni`: 221 words.
    Odpowiedni,
    /// Like `prosty`: 217 words.
    Prosty,
    /// Like `mały`: 136 words.
    Mały,
    /// Like `stały`: 129 words.
    Stały,
    /// Like `drugi`: 49 words.
    Drugi,
    /// Like `jasny`: 38 words.
    Jasny,
    /// Like `głęboki`: 30 words.
    Głęboki,
    /// Like `normalny`: 27 words.
    Normalny,
    /// Like `ścisły`: 26 words.
    Ścisły,
    /// Like `soboli`: 25 words.
    Soboli,
    /// Like `dobry`: 23 words.
    Dobry,
    /// Like `głuchy`: 16 words.
    Głuchy,
    /// Like `młody`: 14 words.
    Młody,
    /// Like `agresywny`: 14 words.
    Agresywny,
    /// Like `stary`: 12 words.
    Stary,
    /// Like `głupi`: 12 words.
    Głupi,
    /// Like `namolny`: 7 words.
    Namolny,
    /// Like `zwięzły`: 7 words.
    Zwięzły,
    /// Like `nowy`: 6 words.
    Nowy,
    /// Like `barani`: 5 words.
    Barani,
    /// Like `żyzny`: 5 words.
    Żyzny,
    /// Like `wart`: 4 words.
    Wart,
    /// Like `wązki`: 4 words.
    Wązki,
    /// Like `jawny`: 4 words.
    Jawny,
    /// Like `rad`: 4 words.
    Rad,
    /// Like `jaskrawy`: 3 words.
    Jaskrawy,
    /// Like `wielki`: 3 words.
    Wielki,
    /// Like `daleki`: 3 words.
    Daleki,
    /// Like `blady`: 3 words.
    Blady,
    /// Like `niedobry`: 3 words.
    Niedobry,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 40] = [
        Declension::Wolny,
        Declension::Słaby,
        Declension::Lekki,
        Declension::Pierwszy,
        Declension::Herbaciany,
        Declension::Ciężki,
        Declension::Palący,
        Declension::Otwarty,
        Declension::Uczulony,
        Declension::Zajęczy,
        Declension::Odpowiedni,
        Declension::Prosty,
        Declension::Mały,
        Declension::Stały,
        Declension::Drugi,
        Declension::Jasny,
        Declension::Głęboki,
        Declension::Normalny,
        Declension::Ścisły,
        Declension::Soboli,
        Declension::Dobry,
        Declension::Głuchy,
        Declension::Młody,
        Declension::Agresywny,
        Declension::Stary,
        Declension::Głupi,
        Declension::Namolny,
        Declension::Zwięzły,
        Declension::Nowy,
        Declension::Barani,
        Declension::Żyzny,
        Declension::Wart,
        Declension::Wązki,
        Declension::Jawny,
        Declension::Rad,
        Declension::Jaskrawy,
        Declension::Wielki,
        Declension::Daleki,
        Declension::Blady,
        Declension::Niedobry,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 40] = [
    // Wolny: 4,032 words (wolny, jenny, bezwstydny, homoseksualny, uszny, niechlujny).
    declension!("wolny", 4032, "~y";
        nominative_masculine: "~y", "~y";
        nominative_feminine: "~a", "~a";
        nominative_neuter: "~e", "~e";
        nominative_virile: "~i", "~i";
        nominative_nonvirile: "~e", "~e";
        genitive_masculine: "~ego", "~ego";
        genitive_feminine: "~ej", "~ej";
        genitive_neuter: "~ego", "~ego";
        genitive_virile: "~ych", "~ych";
        genitive_nonvirile: "~ych", "~ych";
        dative_masculine: "~emu", "~emu";
        dative_feminine: "~ej", "~ej";
        dative_neuter: "~emu", "~emu";
        dative_virile: "~ym", "~ym";
        dative_nonvirile: "~ym", "~ym";
        accusative_masculine: "~ego", "~y";
        accusative_feminine: "~ą", "~ą";
        accusative_neuter: "~e", "~e";
        accusative_virile: "~ych", "~ych";
        accusative_nonvirile: "~e", "~e";
        instrumental_masculine: "~ym", "~ym";
        instrumental_feminine: "~ą", "~ą";
        instrumental_neuter: "~ym", "~ym";
        instrumental_virile: "~ymi", "~ymi";
        instrumental_nonvirile: "~ymi", "~ymi";
        locative_masculine: "~ym", "~ym";
        locative_feminine: "~ej", "~ej";
        locative_neuter: "~ym", "~ym";
        locative_virile: "~ych", "~ych";
        locative_nonvirile: "~ych", "~ych";
        comparative: "~iejszy";
        superlative: "naj~iejszy";
    ),
    // Słaby: 2,935 words (międzynarodowy, różowy, brązowy, snowy, pomarańczowy, fioletowy).
    declension!("słaby", 2935, "~y";
        nominative_masculine: "~y", "~y";
        nominative_feminine: "~a", "~a";
        nominative_neuter: "~e", "~e";
        nominative_virile: "~i", "~i";
        nominative_nonvirile: "~e", "~e";
        genitive_masculine: "~ego", "~ego";
        genitive_feminine: "~ej", "~ej";
        genitive_neuter: "~ego", "~ego";
        genitive_virile: "~ych", "~ych";
        genitive_nonvirile: "~ych", "~ych";
        dative_masculine: "~emu", "~emu";
        dative_feminine: "~ej", "~ej";
        dative_neuter: "~emu", "~emu";
        dative_virile: "~ym", "~ym";
        dative_nonvirile: "~ym", "~ym";
        accusative_masculine: "~ego", "~y";
        accusative_feminine: "~ą", "~ą";
        accusative_neuter: "~e", "~e";
        accusative_virile: "~ych", "~ych";
        accusative_nonvirile: "~e", "~e";
        instrumental_masculine: "~ym", "~ym";
        instrumental_feminine: "~ą", "~ą";
        instrumental_neuter: "~ym", "~ym";
        instrumental_virile: "~ymi", "~ymi";
        instrumental_nonvirile: "~ymi", "~ymi";
        locative_masculine: "~ym", "~ym";
        locative_feminine: "~ej", "~ej";
        locative_neuter: "~ym", "~ym";
        locative_virile: "~ych", "~ych";
        locative_nonvirile: "~ych", "~ych";
        comparative: "~szy";
        superlative: "naj~szy";
    ),
    // Lekki: 1,304 words (polski, rosyjski, norweski, ruski, łaciński, morski).
    declension!("lekki", 1304, "~~ki";
        nominative_masculine: "~~ki", "~~ki";
        nominative_feminine: "~~ka", "~~ka";
        nominative_neuter: "~~kie", "~~kie";
        nominative_virile: "~~cy", "~~cy";
        nominative_nonvirile: "~~kie", "~~kie";
        genitive_masculine: "~~kiego", "~~kiego";
        genitive_feminine: "~~kiej", "~~kiej";
        genitive_neuter: "~~kiego", "~~kiego";
        genitive_virile: "~~kich", "~~kich";
        genitive_nonvirile: "~~kich", "~~kich";
        dative_masculine: "~~kiemu", "~~kiemu";
        dative_feminine: "~~kiej", "~~kiej";
        dative_neuter: "~~kiemu", "~~kiemu";
        dative_virile: "~~kim", "~~kim";
        dative_nonvirile: "~~kim", "~~kim";
        accusative_masculine: "~~kiego", "~~ki";
        accusative_feminine: "~~ką", "~~ką";
        accusative_neuter: "~~kie", "~~kie";
        accusative_virile: "~~kich", "~~kich";
        accusative_nonvirile: "~~kie", "~~kie";
        instrumental_masculine: "~~kim", "~~kim";
        instrumental_feminine: "~~ką", "~~ką";
        instrumental_neuter: "~~kim", "~~kim";
        instrumental_virile: "~~kimi", "~~kimi";
        instrumental_nonvirile: "~~kimi", "~~kimi";
        locative_masculine: "~~kim", "~~kim";
        locative_feminine: "~~kiej", "~~kiej";
        locative_neuter: "~~kim", "~~kim";
        locative_virile: "~~kich", "~~kich";
        locative_nonvirile: "~~kich", "~~kich";
        comparative: "~ż~zy";
        superlative: "naj~ż~zy";
    ),
    // Pierwszy: 798 words (pierwszy, pieszy, lepszy, teraźniejszy, rychlejszy, najrychlejszy).
    declension!("pierwszy", 798, "~zy";
        nominative_masculine: "~zy", "~zy";
        nominative_feminine: "~za", "~za";
        nominative_neuter: "~ze", "~ze";
        nominative_virile: "~i", "~i";
        nominative_nonvirile: "~ze", "~ze";
        genitive_masculine: "~zego", "~zego";
        genitive_feminine: "~zej", "~zej";
        genitive_neuter: "~zego", "~zego";
        genitive_virile: "~zych", "~zych";
        genitive_nonvirile: "~zych", "~zych";
        dative_masculine: "~zemu", "~zemu";
        dative_feminine: "~zej", "~zej";
        dative_neuter: "~zemu", "~zemu";
        dative_virile: "~zym", "~zym";
        dative_nonvirile: "~zym", "~zym";
        accusative_masculine: "~zego", "~zy";
        accusative_feminine: "~zą", "~zą";
        accusative_neuter: "~ze", "~ze";
        accusative_virile: "~zych", "~zych";
        accusative_nonvirile: "~ze", "~ze";
        instrumental_masculine: "~zym", "~zym";
        instrumental_feminine: "~zą", "~zą";
        instrumental_neuter: "~zym", "~zym";
        instrumental_virile: "~zymi", "~zymi";
        instrumental_nonvirile: "~zymi", "~zymi";
        locative_masculine: "~zym", "~zym";
        locative_feminine: "~zej", "~zej";
        locative_neuter: "~zym", "~zym";
        locative_virile: "~zych", "~zych";
        locative_nonvirile: "~zych", "~zych";
        comparative: "~iejszy";
        superlative: "naj~iejszy";
    ),
    // Herbaciany: 644 words (dany, nieprzygotowany, miedziany, przygotowany, pijany, słony).
    declension!("herbaciany", 644, "~ny";
        nominative_masculine: "~ny", "~ny";
        nominative_feminine: "~na", "~na";
        nominative_neuter: "~ne", "~ne";
        nominative_virile: "~ni", "~ni";
        nominative_nonvirile: "~ne", "~ne";
        genitive_masculine: "~nego", "~nego";
        genitive_feminine: "~nej", "~nej";
        genitive_neuter: "~nego", "~nego";
        genitive_virile: "~nych", "~nych";
        genitive_nonvirile: "~nych", "~nych";
        dative_masculine: "~nemu", "~nemu";
        dative_feminine: "~nej", "~nej";
        dative_neuter: "~nemu", "~nemu";
        dative_virile: "~nym", "~nym";
        dative_nonvirile: "~nym", "~nym";
        accusative_masculine: "~nego", "~ny";
        accusative_feminine: "~ną", "~ną";
        accusative_neuter: "~ne", "~ne";
        accusative_virile: "~nych", "~nych";
        accusative_nonvirile: "~ne", "~ne";
        instrumental_masculine: "~nym", "~nym";
        instrumental_feminine: "~ną", "~ną";
        instrumental_neuter: "~nym", "~nym";
        instrumental_virile: "~nymi", "~nymi";
        instrumental_nonvirile: "~nymi", "~nymi";
        locative_masculine: "~nym", "~nym";
        locative_feminine: "~nej", "~nej";
        locative_neuter: "~nym", "~nym";
        locative_virile: "~nych", "~nych";
        locative_nonvirile: "~nych", "~nych";
        comparative: "~ńszy";
        superlative: "naj~ńszy";
    ),
    // Ciężki: 588 words (słodki, jaki, malutki, niderlandzki, niemiecki, ciężki).
    declension!("ciężki", 588, "~ki";
        nominative_masculine: "~ki", "~ki";
        nominative_feminine: "~ka", "~ka";
        nominative_neuter: "~kie", "~kie";
        nominative_virile: "~cy", "~cy";
        nominative_nonvirile: "~kie", "~kie";
        genitive_masculine: "~kiego", "~kiego";
        genitive_feminine: "~kiej", "~kiej";
        genitive_neuter: "~kiego", "~kiego";
        genitive_virile: "~kich", "~kich";
        genitive_nonvirile: "~kich", "~kich";
        dative_masculine: "~kiemu", "~kiemu";
        dative_feminine: "~kiej", "~kiej";
        dative_neuter: "~kiemu", "~kiemu";
        dative_virile: "~kim", "~kim";
        dative_nonvirile: "~kim", "~kim";
        accusative_masculine: "~kiego", "~ki";
        accusative_feminine: "~ką", "~ką";
        accusative_neuter: "~kie", "~kie";
        accusative_virile: "~kich", "~kich";
        accusative_nonvirile: "~kie", "~kie";
        instrumental_masculine: "~kim", "~kim";
        instrumental_feminine: "~ką", "~ką";
        instrumental_neuter: "~kim", "~kim";
        instrumental_virile: "~kimi", "~kimi";
        instrumental_nonvirile: "~kimi", "~kimi";
        locative_masculine: "~kim", "~kim";
        locative_feminine: "~kiej", "~kiej";
        locative_neuter: "~kim", "~kim";
        locative_virile: "~kich", "~kich";
        locative_nonvirile: "~kich", "~kich";
        comparative: "~szy";
        superlative: "naj~szy";
    ),
    // Palący: 584 words (mulczujący, lśniący, gorący, bieżący, śpiący, wystarczający).
    declension!("palący", 584, "~ący";
        nominative_masculine: "~ący", "~ący";
        nominative_feminine: "~ąca", "~ąca";
        nominative_neuter: "~ące", "~ące";
        nominative_virile: "~ący", "~ący";
        nominative_nonvirile: "~ące", "~ące";
        genitive_masculine: "~ącego", "~ącego";
        genitive_feminine: "~ącej", "~ącej";
        genitive_neuter: "~ącego", "~ącego";
        genitive_virile: "~ących", "~ących";
        genitive_nonvirile: "~ących", "~ących";
        dative_masculine: "~ącemu", "~ącemu";
        dative_feminine: "~ącej", "~ącej";
        dative_neuter: "~ącemu", "~ącemu";
        dative_virile: "~ącym", "~ącym";
        dative_nonvirile: "~ącym", "~ącym";
        accusative_masculine: "~ącego", "~ący";
        accusative_feminine: "~ącą", "~ącą";
        accusative_neuter: "~ące", "~ące";
        accusative_virile: "~ących", "~ących";
        accusative_nonvirile: "~ące", "~ące";
        instrumental_masculine: "~ącym", "~ącym";
        instrumental_feminine: "~ącą", "~ącą";
        instrumental_neuter: "~ącym", "~ącym";
        instrumental_virile: "~ącymi", "~ącymi";
        instrumental_nonvirile: "~ącymi", "~ącymi";
        locative_masculine: "~ącym", "~ącym";
        locative_feminine: "~ącej", "~ącej";
        locative_neuter: "~ącym", "~ącym";
        locative_virile: "~ących", "~ących";
        locative_nonvirile: "~ących", "~ących";
        comparative: "~ętszy";
        superlative: "naj~ętszy";
    ),
    // Otwarty: 456 words (party, złoty, pity, żółty, luty, syty).
    declension!("otwarty", 456, "~ty";
        nominative_masculine: "~ty", "~ty";
        nominative_feminine: "~ta", "~ta";
        nominative_neuter: "~te", "~te";
        nominative_virile: "~ci", "~ci";
        nominative_nonvirile: "~te", "~te";
        genitive_masculine: "~tego", "~tego";
        genitive_feminine: "~tej", "~tej";
        genitive_neuter: "~tego", "~tego";
        genitive_virile: "~tych", "~tych";
        genitive_nonvirile: "~tych", "~tych";
        dative_masculine: "~temu", "~temu";
        dative_feminine: "~tej", "~tej";
        dative_neuter: "~temu", "~temu";
        dative_virile: "~tym", "~tym";
        dative_nonvirile: "~tym", "~tym";
        accusative_masculine: "~tego", "~ty";
        accusative_feminine: "~tą", "~tą";
        accusative_neuter: "~te", "~te";
        accusative_virile: "~tych", "~tych";
        accusative_nonvirile: "~te", "~te";
        instrumental_masculine: "~tym", "~tym";
        instrumental_feminine: "~tą", "~tą";
        instrumental_neuter: "~tym", "~tym";
        instrumental_virile: "~tymi", "~tymi";
        instrumental_nonvirile: "~tymi", "~tymi";
        locative_masculine: "~tym", "~tym";
        locative_feminine: "~tej", "~tej";
        locative_neuter: "~tym", "~tym";
        locative_virile: "~tych", "~tych";
        locative_nonvirile: "~tych", "~tych";
        comparative: "~tszy";
        superlative: "naj~tszy";
    ),
    // Uczulony: 440 words (spragniony, zmęczony, ograniczony, uzależniony, roztargniony, popierdolony).
    declension!("uczulony", 440, "~ony";
        nominative_masculine: "~ony", "~ony";
        nominative_feminine: "~ona", "~ona";
        nominative_neuter: "~one", "~one";
        nominative_virile: "~eni", "~eni";
        nominative_nonvirile: "~one", "~one";
        genitive_masculine: "~onego", "~onego";
        genitive_feminine: "~onej", "~onej";
        genitive_neuter: "~onego", "~onego";
        genitive_virile: "~onych", "~onych";
        genitive_nonvirile: "~onych", "~onych";
        dative_masculine: "~onemu", "~onemu";
        dative_feminine: "~onej", "~onej";
        dative_neuter: "~onemu", "~onemu";
        dative_virile: "~onym", "~onym";
        dative_nonvirile: "~onym", "~onym";
        accusative_masculine: "~onego", "~ony";
        accusative_feminine: "~oną", "~oną";
        accusative_neuter: "~one", "~one";
        accusative_virile: "~onych", "~onych";
        accusative_nonvirile: "~one", "~one";
        instrumental_masculine: "~onym", "~onym";
        instrumental_feminine: "~oną", "~oną";
        instrumental_neuter: "~onym", "~onym";
        instrumental_virile: "~onymi", "~onymi";
        instrumental_nonvirile: "~onymi", "~onymi";
        locative_masculine: "~onym", "~onym";
        locative_feminine: "~onej", "~onej";
        locative_neuter: "~onym", "~onym";
        locative_virile: "~onych", "~onych";
        locative_nonvirile: "~onych", "~onych";
        comparative: "~eńszy";
        superlative: "naj~eńszy";
    ),
    // Zajęczy: 393 words (zwierzęcy, obcy, świeży, uroczy, tajemniczy, rolniczy).
    declension!("zajęczy", 393, "~y";
        nominative_masculine: "~y", "~y";
        nominative_feminine: "~a", "~a";
        nominative_neuter: "~e", "~e";
        nominative_virile: "~y", "~y";
        nominative_nonvirile: "~e", "~e";
        genitive_masculine: "~ego", "~ego";
        genitive_feminine: "~ej", "~ej";
        genitive_neuter: "~ego", "~ego";
        genitive_virile: "~ych", "~ych";
        genitive_nonvirile: "~ych", "~ych";
        dative_masculine: "~emu", "~emu";
        dative_feminine: "~ej", "~ej";
        dative_neuter: "~emu", "~emu";
        dative_virile: "~ym", "~ym";
        dative_nonvirile: "~ym", "~ym";
        accusative_masculine: "~ego", "~y";
        accusative_feminine: "~ą", "~ą";
        accusative_neuter: "~e", "~e";
        accusative_virile: "~ych", "~ych";
        accusative_nonvirile: "~e", "~e";
        instrumental_masculine: "~ym", "~ym";
        instrumental_feminine: "~ą", "~ą";
        instrumental_neuter: "~ym", "~ym";
        instrumental_virile: "~ymi", "~ymi";
        instrumental_nonvirile: "~ymi", "~ymi";
        locative_masculine: "~ym", "~ym";
        locative_feminine: "~ej", "~ej";
        locative_neuter: "~ym", "~ym";
        locative_virile: "~ych", "~ych";
        locative_nonvirile: "~ych", "~ych";
        comparative: "~szy";
        superlative: "naj~szy";
    ),
    // Odpowiedni: 221 words (psi, babi, kuni, misi, osi, elfi).
    declension!("odpowiedni", 221, "~";
        nominative_masculine: "~", "~";
        nominative_feminine: "~a", "~a";
        nominative_neuter: "~e", "~e";
        nominative_virile: "~", "~";
        nominative_nonvirile: "~e", "~e";
        genitive_masculine: "~ego", "~ego";
        genitive_feminine: "~ej", "~ej";
        genitive_neuter: "~ego", "~ego";
        genitive_virile: "~ch", "~ch";
        genitive_nonvirile: "~ch", "~ch";
        dative_masculine: "~emu", "~emu";
        dative_feminine: "~ej", "~ej";
        dative_neuter: "~emu", "~emu";
        dative_virile: "~m", "~m";
        dative_nonvirile: "~m", "~m";
        accusative_masculine: "~ego", "~";
        accusative_feminine: "~ą", "~ą";
        accusative_neuter: "~e", "~e";
        accusative_virile: "~ch", "~ch";
        accusative_nonvirile: "~e", "~e";
        instrumental_masculine: "~m", "~m";
        instrumental_feminine: "~ą", "~ą";
        instrumental_neuter: "~m", "~m";
        instrumental_virile: "~mi", "~mi";
        instrumental_nonvirile: "~mi", "~mi";
        locative_masculine: "~m", "~m";
        locative_feminine: "~ej", "~ej";
        locative_neuter: "~m", "~m";
        locative_virile: "~ch", "~ch";
        locative_nonvirile: "~ch", "~ch";
        comparative: "~ejszy";
        superlative: "naj~ejszy";
    ),
    // Prosty: 217 words (prosty, pusty, przezroczysty, szósty, jedenasty, dwunasty).
    declension!("prosty", 217, "~sty";
        nominative_masculine: "~sty", "~sty";
        nominative_feminine: "~sta", "~sta";
        nominative_neuter: "~ste", "~ste";
        nominative_virile: "~ści", "~ści";
        nominative_nonvirile: "~ste", "~ste";
        genitive_masculine: "~stego", "~stego";
        genitive_feminine: "~stej", "~stej";
        genitive_neuter: "~stego", "~stego";
        genitive_virile: "~stych", "~stych";
        genitive_nonvirile: "~stych", "~stych";
        dative_masculine: "~stemu", "~stemu";
        dative_feminine: "~stej", "~stej";
        dative_neuter: "~stemu", "~stemu";
        dative_virile: "~stym", "~stym";
        dative_nonvirile: "~stym", "~stym";
        accusative_masculine: "~stego", "~sty";
        accusative_feminine: "~stą", "~stą";
        accusative_neuter: "~ste", "~ste";
        accusative_virile: "~stych", "~stych";
        accusative_nonvirile: "~ste", "~ste";
        instrumental_masculine: "~stym", "~stym";
        instrumental_feminine: "~stą", "~stą";
        instrumental_neuter: "~stym", "~stym";
        instrumental_virile: "~stymi", "~stymi";
        instrumental_nonvirile: "~stymi", "~stymi";
        locative_masculine: "~stym", "~stym";
        locative_feminine: "~stej", "~stej";
        locative_neuter: "~stym", "~stym";
        locative_virile: "~stych", "~stych";
        locative_nonvirile: "~stych", "~stych";
        comparative: "~stszy";
        superlative: "naj~stszy";
    ),
    // Mały: 136 words (ciepły, biały, oziębły, prostopadły, równoległy, mały).
    declension!("mały", 136, "~ły";
        nominative_masculine: "~ły", "~ły";
        nominative_feminine: "~ła", "~ła";
        nominative_neuter: "~łe", "~łe";
        nominative_virile: "~li", "~li";
        nominative_nonvirile: "~łe", "~łe";
        genitive_masculine: "~łego", "~łego";
        genitive_feminine: "~łej", "~łej";
        genitive_neuter: "~łego", "~łego";
        genitive_virile: "~łych", "~łych";
        genitive_nonvirile: "~łych", "~łych";
        dative_masculine: "~łemu", "~łemu";
        dative_feminine: "~łej", "~łej";
        dative_neuter: "~łemu", "~łemu";
        dative_virile: "~łym", "~łym";
        dative_nonvirile: "~łym", "~łym";
        accusative_masculine: "~łego", "~ły";
        accusative_feminine: "~łą", "~łą";
        accusative_neuter: "~łe", "~łe";
        accusative_virile: "~łych", "~łych";
        accusative_nonvirile: "~łe", "~łe";
        instrumental_masculine: "~łym", "~łym";
        instrumental_feminine: "~łą", "~łą";
        instrumental_neuter: "~łym", "~łym";
        instrumental_virile: "~łymi", "~łymi";
        instrumental_nonvirile: "~łymi", "~łymi";
        locative_masculine: "~łym", "~łym";
        locative_feminine: "~łej", "~łej";
        locative_neuter: "~łym", "~łym";
        locative_virile: "~łych", "~łych";
        locative_nonvirile: "~łych", "~łych";
        comparative: "~lejszy";
        superlative: "naj~lejszy";
    ),
    // Stały: 129 words (miły, cały, zgniły, spleśniały, były, dojrzały).
    declension!("stały", 129, "~ły";
        nominative_masculine: "~ły", "~ły";
        nominative_feminine: "~ła", "~ła";
        nominative_neuter: "~łe", "~łe";
        nominative_virile: "~li", "~li";
        nominative_nonvirile: "~łe", "~łe";
        genitive_masculine: "~łego", "~łego";
        genitive_feminine: "~łej", "~łej";
        genitive_neuter: "~łego", "~łego";
        genitive_virile: "~łych", "~łych";
        genitive_nonvirile: "~łych", "~łych";
        dative_masculine: "~łemu", "~łemu";
        dative_feminine: "~łej", "~łej";
        dative_neuter: "~łemu", "~łemu";
        dative_virile: "~łym", "~łym";
        dative_nonvirile: "~łym", "~łym";
        accusative_masculine: "~łego", "~ły";
        accusative_feminine: "~łą", "~łą";
        accusative_neuter: "~łe", "~łe";
        accusative_virile: "~łych", "~łych";
        accusative_nonvirile: "~łe", "~łe";
        instrumental_masculine: "~łym", "~łym";
        instrumental_feminine: "~łą", "~łą";
        instrumental_neuter: "~łym", "~łym";
        instrumental_virile: "~łymi", "~łymi";
        instrumental_nonvirile: "~łymi", "~łymi";
        locative_masculine: "~łym", "~łym";
        locative_feminine: "~łej", "~łej";
        locative_neuter: "~łym", "~łym";
        locative_virile: "~łych", "~łych";
        locative_nonvirile: "~łych", "~łych";
        comparative: "~lszy";
        superlative: "naj~lszy";
    ),
    // Drugi: 49 words (drogi, nagi, drugi, długi, mnogi, wrogi).
    declension!("drugi", 49, "~gi";
        nominative_masculine: "~gi", "~gi";
        nominative_feminine: "~ga", "~ga";
        nominative_neuter: "~gie", "~gie";
        nominative_virile: "~dzy", "~dzy";
        nominative_nonvirile: "~gie", "~gie";
        genitive_masculine: "~giego", "~giego";
        genitive_feminine: "~giej", "~giej";
        genitive_neuter: "~giego", "~giego";
        genitive_virile: "~gich", "~gich";
        genitive_nonvirile: "~gich", "~gich";
        dative_masculine: "~giemu", "~giemu";
        dative_feminine: "~giej", "~giej";
        dative_neuter: "~giemu", "~giemu";
        dative_virile: "~gim", "~gim";
        dative_nonvirile: "~gim", "~gim";
        accusative_masculine: "~giego", "~gi";
        accusative_feminine: "~gą", "~gą";
        accusative_neuter: "~gie", "~gie";
        accusative_virile: "~gich", "~gich";
        accusative_nonvirile: "~gie", "~gie";
        instrumental_masculine: "~gim", "~gim";
        instrumental_feminine: "~gą", "~gą";
        instrumental_neuter: "~gim", "~gim";
        instrumental_virile: "~gimi", "~gimi";
        instrumental_nonvirile: "~gimi", "~gimi";
        locative_masculine: "~gim", "~gim";
        locative_feminine: "~giej", "~giej";
        locative_neuter: "~gim", "~gim";
        locative_virile: "~gich", "~gich";
        locative_nonvirile: "~gich", "~gich";
        comparative: "~ższy";
        superlative: "naj~ższy";
    ),
    // Jasny: 38 words (ciasny, własny, jasny, równoczesny, jednoczesny, wczesny).
    declension!("jasny", 38, "~s~y";
        nominative_masculine: "~s~y", "~s~y";
        nominative_feminine: "~s~a", "~s~a";
        nominative_neuter: "~s~e", "~s~e";
        nominative_virile: "~ś~i", "~ś~i";
        nominative_nonvirile: "~s~e", "~s~e";
        genitive_masculine: "~s~ego", "~s~ego";
        genitive_feminine: "~s~ej", "~s~ej";
        genitive_neuter: "~s~ego", "~s~ego";
        genitive_virile: "~s~ych", "~s~ych";
        genitive_nonvirile: "~s~ych", "~s~ych";
        dative_masculine: "~s~emu", "~s~emu";
        dative_feminine: "~s~ej", "~s~ej";
        dative_neuter: "~s~emu", "~s~emu";
        dative_virile: "~s~ym", "~s~ym";
        dative_nonvirile: "~s~ym", "~s~ym";
        accusative_masculine: "~s~ego", "~s~y";
        accusative_feminine: "~s~ą", "~s~ą";
        accusative_neuter: "~s~e", "~s~e";
        accusative_virile: "~s~ych", "~s~ych";
        accusative_nonvirile: "~s~e", "~s~e";
        instrumental_masculine: "~s~ym", "~s~ym";
        instrumental_feminine: "~s~ą", "~s~ą";
        instrumental_neuter: "~s~ym", "~s~ym";
        instrumental_virile: "~s~ymi", "~s~ymi";
        instrumental_nonvirile: "~s~ymi", "~s~ymi";
        locative_masculine: "~s~ym", "~s~ym";
        locative_feminine: "~s~ej", "~s~ej";
        locative_neuter: "~s~ym", "~s~ym";
        locative_virile: "~s~ych", "~s~ych";
        locative_nonvirile: "~s~ych", "~s~ych";
        comparative: "~ś~iejszy";
        superlative: "naj~ś~iejszy";
    ),
    // Głęboki: 30 words (szeroki, głęboki, jednooki, zielonooki, bystrooki, skośnooki).
    declension!("głęboki", 30, "~oki";
        nominative_masculine: "~oki", "~oki";
        nominative_feminine: "~oka", "~oka";
        nominative_neuter: "~okie", "~okie";
        nominative_virile: "~ocy", "~ocy";
        nominative_nonvirile: "~okie", "~okie";
        genitive_masculine: "~okiego", "~okiego";
        genitive_feminine: "~okiej", "~okiej";
        genitive_neuter: "~okiego", "~okiego";
        genitive_virile: "~okich", "~okich";
        genitive_nonvirile: "~okich", "~okich";
        dative_masculine: "~okiemu", "~okiemu";
        dative_feminine: "~okiej", "~okiej";
        dative_neuter: "~okiemu", "~okiemu";
        dative_virile: "~okim", "~okim";
        dative_nonvirile: "~okim", "~okim";
        accusative_masculine: "~okiego", "~oki";
        accusative_feminine: "~oką", "~oką";
        accusative_neuter: "~okie", "~okie";
        accusative_virile: "~okich", "~okich";
        accusative_nonvirile: "~okie", "~okie";
        instrumental_masculine: "~okim", "~okim";
        instrumental_feminine: "~oką", "~oką";
        instrumental_neuter: "~okim", "~okim";
        instrumental_virile: "~okimi", "~okimi";
        instrumental_nonvirile: "~okimi", "~okimi";
        locative_masculine: "~okim", "~okim";
        locative_feminine: "~okiej", "~okiej";
        locative_neuter: "~okim", "~okim";
        locative_virile: "~okich", "~okich";
        locative_nonvirile: "~okich", "~okich";
        comparative: "~szy";
        superlative: "naj~szy";
    ),
    // Normalny: 27 words (nudny, niebezpieczny, neutralny, normalny, niezbędny, nikczemny).
    declension!("normalny", 27, "~~y";
        nominative_masculine: "~~y", "~~y";
        nominative_feminine: "~~a", "~~a";
        nominative_neuter: "~~e", "~~e";
        nominative_virile: "~~i", "~~i";
        nominative_nonvirile: "~~e", "~~e";
        genitive_masculine: "~~ego", "~~ego";
        genitive_feminine: "~~ej", "~~ej";
        genitive_neuter: "~~ego", "~~ego";
        genitive_virile: "~~ych", "~~ych";
        genitive_nonvirile: "~~ych", "~~ych";
        dative_masculine: "~~emu", "~~emu";
        dative_feminine: "~~ej", "~~ej";
        dative_neuter: "~~emu", "~~emu";
        dative_virile: "~~ym", "~~ym";
        dative_nonvirile: "~~ym", "~~ym";
        accusative_masculine: "~~ego", "~~y";
        accusative_feminine: "~~ą", "~~ą";
        accusative_neuter: "~~e", "~~e";
        accusative_virile: "~~ych", "~~ych";
        accusative_nonvirile: "~~e", "~~e";
        instrumental_masculine: "~~ym", "~~ym";
        instrumental_feminine: "~~ą", "~~ą";
        instrumental_neuter: "~~ym", "~~ym";
        instrumental_virile: "~~ymi", "~~ymi";
        instrumental_nonvirile: "~~ymi", "~~ymi";
        locative_masculine: "~~ym", "~~ym";
        locative_feminine: "~~ej", "~~ej";
        locative_neuter: "~~ym", "~~ym";
        locative_virile: "~~ych", "~~ych";
        locative_nonvirile: "~~ych", "~~ych";
        comparative: "~~iejszy";
        superlative: "~ajn~iejszy";
    ),
    // Ścisły: 26 words (dorosły, ścisły, wzniosły, wklęsły, rosły, wyniosły).
    declension!("ścisły", 26, "~sły";
        nominative_masculine: "~sły", "~sły";
        nominative_feminine: "~sła", "~sła";
        nominative_neuter: "~słe", "~słe";
        nominative_virile: "~śli", "~śli";
        nominative_nonvirile: "~słe", "~słe";
        genitive_masculine: "~słego", "~słego";
        genitive_feminine: "~słej", "~słej";
        genitive_neuter: "~słego", "~słego";
        genitive_virile: "~słych", "~słych";
        genitive_nonvirile: "~słych", "~słych";
        dative_masculine: "~słemu", "~słemu";
        dative_feminine: "~słej", "~słej";
        dative_neuter: "~słemu", "~słemu";
        dative_virile: "~słym", "~słym";
        dative_nonvirile: "~słym", "~słym";
        accusative_masculine: "~słego", "~sły";
        accusative_feminine: "~słą", "~słą";
        accusative_neuter: "~słe", "~słe";
        accusative_virile: "~słych", "~słych";
        accusative_nonvirile: "~słe", "~słe";
        instrumental_masculine: "~słym", "~słym";
        instrumental_feminine: "~słą", "~słą";
        instrumental_neuter: "~słym", "~słym";
        instrumental_virile: "~słymi", "~słymi";
        instrumental_nonvirile: "~słymi", "~słymi";
        locative_masculine: "~słym", "~słym";
        locative_feminine: "~słej", "~słej";
        locative_neuter: "~słym", "~słym";
        locative_virile: "~słych", "~słych";
        locative_nonvirile: "~słych", "~słych";
        comparative: "~ślejszy";
        superlative: "naj~ślejszy";
    ),
    // Soboli: 25 words (orli, muli, karli, ośli, krokodyli, pszczeli).
    declension!("soboli", 25, "~i";
        nominative_masculine: "~i", "~i";
        nominative_feminine: "~a", "~a";
        nominative_neuter: "~e", "~e";
        nominative_virile: "~i", "~i";
        nominative_nonvirile: "~e", "~e";
        genitive_masculine: "~ego", "~ego";
        genitive_feminine: "~ej", "~ej";
        genitive_neuter: "~ego", "~ego";
        genitive_virile: "~ich", "~ich";
        genitive_nonvirile: "~ich", "~ich";
        dative_masculine: "~emu", "~emu";
        dative_feminine: "~ej", "~ej";
        dative_neuter: "~emu", "~emu";
        dative_virile: "~im", "~im";
        dative_nonvirile: "~im", "~im";
        accusative_masculine: "~ego", "~i";
        accusative_feminine: "~ą", "~ą";
        accusative_neuter: "~e", "~e";
        accusative_virile: "~ich", "~ich";
        accusative_nonvirile: "~e", "~e";
        instrumental_masculine: "~im", "~im";
        instrumental_feminine: "~ą", "~ą";
        instrumental_neuter: "~im", "~im";
        instrumental_virile: "~imi", "~imi";
        instrumental_nonvirile: "~imi", "~imi";
        locative_masculine: "~im", "~im";
        locative_feminine: "~ej", "~ej";
        locative_neuter: "~im", "~im";
        locative_virile: "~ich", "~ich";
        locative_nonvirile: "~ich", "~ich";
        comparative: "~szy";
        superlative: "naj~szy";
    ),
    // Dobry: 23 words (bury, dobry, mokry, ostry, mądry, chrobry).
    declension!("dobry", 23, "~y";
        nominative_masculine: "~y", "~y";
        nominative_feminine: "~a", "~a";
        nominative_neuter: "~e", "~e";
        nominative_virile: "~zy", "~zy";
        nominative_nonvirile: "~e", "~e";
        genitive_masculine: "~ego", "~ego";
        genitive_feminine: "~ej", "~ej";
        genitive_neuter: "~ego", "~ego";
        genitive_virile: "~ych", "~ych";
        genitive_nonvirile: "~ych", "~ych";
        dative_masculine: "~emu", "~emu";
        dative_feminine: "~ej", "~ej";
        dative_neuter: "~emu", "~emu";
        dative_virile: "~ym", "~ym";
        dative_nonvirile: "~ym", "~ym";
        accusative_masculine: "~ego", "~y";
        accusative_feminine: "~ą", "~ą";
        accusative_neuter: "~e", "~e";
        accusative_virile: "~ych", "~ych";
        accusative_nonvirile: "~e", "~e";
        instrumental_masculine: "~ym", "~ym";
        instrumental_feminine: "~ą", "~ą";
        instrumental_neuter: "~ym", "~ym";
        instrumental_virile: "~ymi", "~ymi";
        instrumental_nonvirile: "~ymi", "~ymi";
        locative_masculine: "~ym", "~ym";
        locative_feminine: "~ej", "~ej";
        locative_neuter: "~ym", "~ym";
        locative_virile: "~ych", "~ych";
        locative_nonvirile: "~ych", "~ych";
        comparative: "~zejszy";
        superlative: "naj~zejszy";
    ),
    // Głuchy: 16 words (cichy, suchy, głuchy, kruchy, lichy, płochy).
    declension!("głuchy", 16, "~chy";
        nominative_masculine: "~chy", "~chy";
        nominative_feminine: "~cha", "~cha";
        nominative_neuter: "~che", "~che";
        nominative_virile: "~si", "~si";
        nominative_nonvirile: "~che", "~che";
        genitive_masculine: "~chego", "~chego";
        genitive_feminine: "~chej", "~chej";
        genitive_neuter: "~chego", "~chego";
        genitive_virile: "~chych", "~chych";
        genitive_nonvirile: "~chych", "~chych";
        dative_masculine: "~chemu", "~chemu";
        dative_feminine: "~chej", "~chej";
        dative_neuter: "~chemu", "~chemu";
        dative_virile: "~chym", "~chym";
        dative_nonvirile: "~chym", "~chym";
        accusative_masculine: "~chego", "~chy";
        accusative_feminine: "~chą", "~chą";
        accusative_neuter: "~che", "~che";
        accusative_virile: "~chych", "~chych";
        accusative_nonvirile: "~che", "~che";
        instrumental_masculine: "~chym", "~chym";
        instrumental_feminine: "~chą", "~chą";
        instrumental_neuter: "~chym", "~chym";
        instrumental_virile: "~chymi", "~chymi";
        instrumental_nonvirile: "~chymi", "~chymi";
        locative_masculine: "~chym", "~chym";
        locative_feminine: "~chej", "~chej";
        locative_neuter: "~chym", "~chym";
        locative_virile: "~chych", "~chych";
        locative_nonvirile: "~chych", "~chych";
        comparative: "~chszy";
        superlative: "naj~chszy";
    ),
    // Młody: 14 words (hardy, młody, twardy, chudy, rudy, złotorudy).
    declension!("młody", 14, "~y";
        nominative_masculine: "~y", "~y";
        nominative_feminine: "~a", "~a";
        nominative_neuter: "~e", "~e";
        nominative_virile: "~zi", "~zi";
        nominative_nonvirile: "~e", "~e";
        genitive_masculine: "~ego", "~ego";
        genitive_feminine: "~ej", "~ej";
        genitive_neuter: "~ego", "~ego";
        genitive_virile: "~ych", "~ych";
        genitive_nonvirile: "~ych", "~ych";
        dative_masculine: "~emu", "~emu";
        dative_feminine: "~ej", "~ej";
        dative_neuter: "~emu", "~emu";
        dative_virile: "~ym", "~ym";
        dative_nonvirile: "~ym", "~ym";
        accusative_masculine: "~ego", "~y";
        accusative_feminine: "~ą", "~ą";
        accusative_neuter: "~e", "~e";
        accusative_virile: "~ych", "~ych";
        accusative_nonvirile: "~e", "~e";
        instrumental_masculine: "~ym", "~ym";
        instrumental_feminine: "~ą", "~ą";
        instrumental_neuter: "~ym", "~ym";
        instrumental_virile: "~ymi", "~ymi";
        instrumental_nonvirile: "~ymi", "~ymi";
        locative_masculine: "~ym", "~ym";
        locative_feminine: "~ej", "~ej";
        locative_neuter: "~ym", "~ym";
        locative_virile: "~ych", "~ych";
        locative_nonvirile: "~ych", "~ych";
        comparative: "~szy";
        superlative: "naj~szy";
    ),
    // Agresywny: 14 words (agresywny, atrakcyjny, ambitny, aktualny, adekwatny, absolutny).
    declension!("agresywny", 14, "~~y";
        nominative_masculine: "~~y", "~~y";
        nominative_feminine: "~~a", "~~a";
        nominative_neuter: "~~e", "~~e";
        nominative_virile: "~~i", "~~i";
        nominative_nonvirile: "~~e", "~~e";
        genitive_masculine: "~~ego", "~~ego";
        genitive_feminine: "~~ej", "~~ej";
        genitive_neuter: "~~ego", "~~ego";
        genitive_virile: "~~ych", "~~ych";
        genitive_nonvirile: "~~ych", "~~ych";
        dative_masculine: "~~emu", "~~emu";
        dative_feminine: "~~ej", "~~ej";
        dative_neuter: "~~emu", "~~emu";
        dative_virile: "~~ym", "~~ym";
        dative_nonvirile: "~~ym", "~~ym";
        accusative_masculine: "~~ego", "~~y";
        accusative_feminine: "~~ą", "~~ą";
        accusative_neuter: "~~e", "~~e";
        accusative_virile: "~~ych", "~~ych";
        accusative_nonvirile: "~~e", "~~e";
        instrumental_masculine: "~~ym", "~~ym";
        instrumental_feminine: "~~ą", "~~ą";
        instrumental_neuter: "~~ym", "~~ym";
        instrumental_virile: "~~ymi", "~~ymi";
        instrumental_nonvirile: "~~ymi", "~~ymi";
        locative_masculine: "~~ym", "~~ym";
        locative_feminine: "~~ej", "~~ej";
        locative_neuter: "~~ym", "~~ym";
        locative_virile: "~~ych", "~~ych";
        locative_nonvirile: "~~ych", "~~ych";
        comparative: "~~iejszy";
        superlative: "n~ja~iejszy";
    ),
    // Stary: 12 words (chory, stary, szary, szczery, jasnoszary, spory).
    declension!("stary", 12, "~y";
        nominative_masculine: "~y", "~y";
        nominative_feminine: "~a", "~a";
        nominative_neuter: "~e", "~e";
        nominative_virile: "~zy", "~zy";
        nominative_nonvirile: "~e", "~e";
        genitive_masculine: "~ego", "~ego";
        genitive_feminine: "~ej", "~ej";
        genitive_neuter: "~ego", "~ego";
        genitive_virile: "~ych", "~ych";
        genitive_nonvirile: "~ych", "~ych";
        dative_masculine: "~emu", "~emu";
        dative_feminine: "~ej", "~ej";
        dative_neuter: "~emu", "~emu";
        dative_virile: "~ym", "~ym";
        dative_nonvirile: "~ym", "~ym";
        accusative_masculine: "~ego", "~y";
        accusative_feminine: "~ą", "~ą";
        accusative_neuter: "~e", "~e";
        accusative_virile: "~ych", "~ych";
        accusative_nonvirile: "~e", "~e";
        instrumental_masculine: "~ym", "~ym";
        instrumental_feminine: "~ą", "~ą";
        instrumental_neuter: "~ym", "~ym";
        instrumental_virile: "~ymi", "~ymi";
        instrumental_nonvirile: "~ymi", "~ymi";
        locative_masculine: "~ym", "~ym";
        locative_feminine: "~ej", "~ej";
        locative_neuter: "~ym", "~ym";
        locative_virile: "~ych", "~ych";
        locative_nonvirile: "~ych", "~ych";
        comparative: "~szy";
        superlative: "naj~szy";
    ),
    // Głupi: 12 words (dropi, capi, głupi, małpi, karpi, trupi).
    declension!("głupi", 12, "~i";
        nominative_masculine: "~i", "~i";
        nominative_feminine: "~ia", "~ia";
        nominative_neuter: "~ie", "~ie";
        nominative_virile: "~i", "~i";
        nominative_nonvirile: "~ie", "~ie";
        genitive_masculine: "~iego", "~iego";
        genitive_feminine: "~iej", "~iej";
        genitive_neuter: "~iego", "~iego";
        genitive_virile: "~ich", "~ich";
        genitive_nonvirile: "~ich", "~ich";
        dative_masculine: "~iemu", "~iemu";
        dative_feminine: "~iej", "~iej";
        dative_neuter: "~iemu", "~iemu";
        dative_virile: "~im", "~im";
        dative_nonvirile: "~im", "~im";
        accusative_masculine: "~iego", "~i";
        accusative_feminine: "~ią", "~ią";
        accusative_neuter: "~ie", "~ie";
        accusative_virile: "~ich", "~ich";
        accusative_nonvirile: "~ie", "~ie";
        instrumental_masculine: "~im", "~im";
        instrumental_feminine: "~ią", "~ią";
        instrumental_neuter: "~im", "~im";
        instrumental_virile: "~imi", "~imi";
        instrumental_nonvirile: "~imi", "~imi";
        locative_masculine: "~im", "~im";
        locative_feminine: "~iej", "~iej";
        locative_neuter: "~im", "~im";
        locative_virile: "~ich", "~ich";
        locative_nonvirile: "~ich", "~ich";
        comparative: "~szy";
        superlative: "naj~szy";
    ),
    // Namolny: 7 words (naturalny, nadobny, nachalny, natrętny, namiętny, namolny).
    declension!("namolny", 7, "~~y";
        nominative_masculine: "~~y", "~~y";
        nominative_feminine: "~~a", "~~a";
        nominative_neuter: "~~e", "~~e";
        nominative_virile: "~~i", "~~i";
        nominative_nonvirile: "~~e", "~~e";
        genitive_masculine: "~~ego", "~~ego";
        genitive_feminine: "~~ej", "~~ej";
        genitive_neuter: "~~ego", "~~ego";
        genitive_virile: "~~ych", "~~ych";
        genitive_nonvirile: "~~ych", "~~ych";
        dative_masculine: "~~emu", "~~emu";
        dative_feminine: "~~ej", "~~ej";
        dative_neuter: "~~emu", "~~emu";
        dative_virile: "~~ym", "~~ym";
        dative_nonvirile: "~~ym", "~~ym";
        accusative_masculine: "~~ego", "~~y";
        accusative_feminine: "~~ą", "~~ą";
        accusative_neuter: "~~e", "~~e";
        accusative_virile: "~~ych", "~~ych";
        accusative_nonvirile: "~~e", "~~e";
        instrumental_masculine: "~~ym", "~~ym";
        instrumental_feminine: "~~ą", "~~ą";
        instrumental_neuter: "~~ym", "~~ym";
        instrumental_virile: "~~ymi", "~~ymi";
        instrumental_nonvirile: "~~ymi", "~~ymi";
        locative_masculine: "~~ym", "~~ym";
        locative_feminine: "~~ej", "~~ej";
        locative_neuter: "~~ym", "~~ym";
        locative_virile: "~~ych", "~~ych";
        locative_nonvirile: "~~ych", "~~ych";
        comparative: "~~iejszy";
        superlative: "~jna~iejszy";
    ),
    // Zwięzły: 7 words (zwięzły, rozwiązły, niezły, rozlazły, spełzły, wypełzły).
    declension!("zwięzły", 7, "~zły";
        nominative_masculine: "~zły", "~zły";
        nominative_feminine: "~zła", "~zła";
        nominative_neuter: "~złe", "~złe";
        nominative_virile: "~źli", "~źli";
        nominative_nonvirile: "~złe", "~złe";
        genitive_masculine: "~złego", "~złego";
        genitive_feminine: "~złej", "~złej";
        genitive_neuter: "~złego", "~złego";
        genitive_virile: "~złych", "~złych";
        genitive_nonvirile: "~złych", "~złych";
        dative_masculine: "~złemu", "~złemu";
        dative_feminine: "~złej", "~złej";
        dative_neuter: "~złemu", "~złemu";
        dative_virile: "~złym", "~złym";
        dative_nonvirile: "~złym", "~złym";
        accusative_masculine: "~złego", "~zły";
        accusative_feminine: "~złą", "~złą";
        accusative_neuter: "~złe", "~złe";
        accusative_virile: "~złych", "~złych";
        accusative_nonvirile: "~złe", "~złe";
        instrumental_masculine: "~złym", "~złym";
        instrumental_feminine: "~złą", "~złą";
        instrumental_neuter: "~złym", "~złym";
        instrumental_virile: "~złymi", "~złymi";
        instrumental_nonvirile: "~złymi", "~złymi";
        locative_masculine: "~złym", "~złym";
        locative_feminine: "~złej", "~złej";
        locative_neuter: "~złym", "~złym";
        locative_virile: "~złych", "~złych";
        locative_nonvirile: "~złych", "~złych";
        comparative: "~źlejszy";
        superlative: "naj~źlejszy";
    ),
    // Nowy: 6 words (nowy, niecierpliwy, niesprawiedliwy, nieszczęśliwy, nieżyczliwy, nobliwy).
    declension!("nowy", 6, "~~y";
        nominative_masculine: "~~y", "~~y";
        nominative_feminine: "~~a", "~~a";
        nominative_neuter: "~~e", "~~e";
        nominative_virile: "~~i", "~~i";
        nominative_nonvirile: "~~e", "~~e";
        genitive_masculine: "~~ego", "~~ego";
        genitive_feminine: "~~ej", "~~ej";
        genitive_neuter: "~~ego", "~~ego";
        genitive_virile: "~~ych", "~~ych";
        genitive_nonvirile: "~~ych", "~~ych";
        dative_masculine: "~~emu", "~~emu";
        dative_feminine: "~~ej", "~~ej";
        dative_neuter: "~~emu", "~~emu";
        dative_virile: "~~ym", "~~ym";
        dative_nonvirile: "~~ym", "~~ym";
        accusative_masculine: "~~ego", "~~y";
        accusative_feminine: "~~ą", "~~ą";
        accusative_neuter: "~~e", "~~e";
        accusative_virile: "~~ych", "~~ych";
        accusative_nonvirile: "~~e", "~~e";
        instrumental_masculine: "~~ym", "~~ym";
        instrumental_feminine: "~~ą", "~~ą";
        instrumental_neuter: "~~ym", "~~ym";
        instrumental_virile: "~~ymi", "~~ymi";
        instrumental_nonvirile: "~~ymi", "~~ymi";
        locative_masculine: "~~ym", "~~ym";
        locative_feminine: "~~ej", "~~ej";
        locative_neuter: "~~ym", "~~ym";
        locative_virile: "~~ych", "~~ych";
        locative_nonvirile: "~~ych", "~~ych";
        comparative: "~~szy";
        superlative: "~ajn~szy";
    ),
    // Barani: 5 words (kani, tani, barani, łani, bociani).
    declension!("barani", 5, "~ni";
        nominative_masculine: "~ni", "~ni";
        nominative_feminine: "~nia", "~nia";
        nominative_neuter: "~nie", "~nie";
        nominative_virile: "~ni", "~ni";
        nominative_nonvirile: "~nie", "~nie";
        genitive_masculine: "~niego", "~niego";
        genitive_feminine: "~niej", "~niej";
        genitive_neuter: "~niego", "~niego";
        genitive_virile: "~nich", "~nich";
        genitive_nonvirile: "~nich", "~nich";
        dative_masculine: "~niemu", "~niemu";
        dative_feminine: "~niej", "~niej";
        dative_neuter: "~niemu", "~niemu";
        dative_virile: "~nim", "~nim";
        dative_nonvirile: "~nim", "~nim";
        accusative_masculine: "~niego", "~ni";
        accusative_feminine: "~nią", "~nią";
        accusative_neuter: "~nie", "~nie";
        accusative_virile: "~nich", "~nich";
        accusative_nonvirile: "~nie", "~nie";
        instrumental_masculine: "~nim", "~nim";
        instrumental_feminine: "~nią", "~nią";
        instrumental_neuter: "~nim", "~nim";
        instrumental_virile: "~nimi", "~nimi";
        instrumental_nonvirile: "~nimi", "~nimi";
        locative_masculine: "~nim", "~nim";
        locative_feminine: "~niej", "~niej";
        locative_neuter: "~nim", "~nim";
        locative_virile: "~nich", "~nich";
        locative_nonvirile: "~nich", "~nich";
        comparative: "~ńszy";
        superlative: "naj~ńszy";
    ),
    // Żyzny: 5 words (żelazny, przyjazny, żyzny, nieprzyjazny, nieżelazny).
    declension!("żyzny", 5, "~z~y";
        nominative_masculine: "~z~y", "~z~y";
        nominative_feminine: "~z~a", "~z~a";
        nominative_neuter: "~z~e", "~z~e";
        nominative_virile: "~ź~i", "~ź~i";
        nominative_nonvirile: "~z~e", "~z~e";
        genitive_masculine: "~z~ego", "~z~ego";
        genitive_feminine: "~z~ej", "~z~ej";
        genitive_neuter: "~z~ego", "~z~ego";
        genitive_virile: "~z~ych", "~z~ych";
        genitive_nonvirile: "~z~ych", "~z~ych";
        dative_masculine: "~z~emu", "~z~emu";
        dative_feminine: "~z~ej", "~z~ej";
        dative_neuter: "~z~emu", "~z~emu";
        dative_virile: "~z~ym", "~z~ym";
        dative_nonvirile: "~z~ym", "~z~ym";
        accusative_masculine: "~z~ego", "~z~y";
        accusative_feminine: "~z~ą", "~z~ą";
        accusative_neuter: "~z~e", "~z~e";
        accusative_virile: "~z~ych", "~z~ych";
        accusative_nonvirile: "~z~e", "~z~e";
        instrumental_masculine: "~z~ym", "~z~ym";
        instrumental_feminine: "~z~ą", "~z~ą";
        instrumental_neuter: "~z~ym", "~z~ym";
        instrumental_virile: "~z~ymi", "~z~ymi";
        instrumental_nonvirile: "~z~ymi", "~z~ymi";
        locative_masculine: "~z~ym", "~z~ym";
        locative_feminine: "~z~ej", "~z~ej";
        locative_neuter: "~z~ym", "~z~ym";
        locative_virile: "~z~ych", "~z~ych";
        locative_nonvirile: "~z~ych", "~z~ych";
        comparative: "~ź~iejszy";
        superlative: "naj~ź~iejszy";
    ),
    // Wart: 4 words (wart, syt, kontent, niewart).
    declension!("wart", 4, "~t";
        nominative_masculine: "~t", "~t";
        nominative_feminine: "~ta", "~ta";
        nominative_neuter: "~te", "~te";
        nominative_virile: "~ci", "~ci";
        nominative_nonvirile: "~te", "~te";
        genitive_masculine: "~tego", "~tego";
        genitive_feminine: "~tej", "~tej";
        genitive_neuter: "~tego", "~tego";
        genitive_virile: "~tych", "~tych";
        genitive_nonvirile: "~tych", "~tych";
        dative_masculine: "~temu", "~temu";
        dative_feminine: "~tej", "~tej";
        dative_neuter: "~temu", "~temu";
        dative_virile: "~tym", "~tym";
        dative_nonvirile: "~tym", "~tym";
        accusative_masculine: "~tego", "~t";
        accusative_feminine: "~tą", "~tą";
        accusative_neuter: "~te", "~te";
        accusative_virile: "~tych", "~tych";
        accusative_nonvirile: "~te", "~te";
        instrumental_masculine: "~tym", "~tym";
        instrumental_feminine: "~tą", "~tą";
        instrumental_neuter: "~tym", "~tym";
        instrumental_virile: "~tymi", "~tymi";
        instrumental_nonvirile: "~tymi", "~tymi";
        locative_masculine: "~tym", "~tym";
        locative_feminine: "~tej", "~tej";
        locative_neuter: "~tym", "~tym";
        locative_virile: "~tych", "~tych";
        locative_nonvirile: "~tych", "~tych";
        comparative: "~tszy";
        superlative: "naj~tszy";
    ),
    // Wązki: 4 words (sąsiedzki, wązki, dobrosąsiedzki, grudziądzki).
    declension!("wązki", 4, "~ą~ki";
        nominative_masculine: "~ą~ki", "~ą~ki";
        nominative_feminine: "~ą~ka", "~ą~ka";
        nominative_neuter: "~ą~kie", "~ą~kie";
        nominative_virile: "~ą~cy", "~ą~cy";
        nominative_nonvirile: "~ą~kie", "~ą~kie";
        genitive_masculine: "~ą~kiego", "~ą~kiego";
        genitive_feminine: "~ą~kiej", "~ą~kiej";
        genitive_neuter: "~ą~kiego", "~ą~kiego";
        genitive_virile: "~ą~kich", "~ą~kich";
        genitive_nonvirile: "~ą~kich", "~ą~kich";
        dative_masculine: "~ą~kiemu", "~ą~kiemu";
        dative_feminine: "~ą~kiej", "~ą~kiej";
        dative_neuter: "~ą~kiemu", "~ą~kiemu";
        dative_virile: "~ą~kim", "~ą~kim";
        dative_nonvirile: "~ą~kim", "~ą~kim";
        accusative_masculine: "~ą~kiego", "~ą~ki";
        accusative_feminine: "~ą~ką", "~ą~ką";
        accusative_neuter: "~ą~kie", "~ą~kie";
        accusative_virile: "~ą~kich", "~ą~kich";
        accusative_nonvirile: "~ą~kie", "~ą~kie";
        instrumental_masculine: "~ą~kim", "~ą~kim";
        instrumental_feminine: "~ą~ką", "~ą~ką";
        instrumental_neuter: "~ą~kim", "~ą~kim";
        instrumental_virile: "~ą~kimi", "~ą~kimi";
        instrumental_nonvirile: "~ą~kimi", "~ą~kimi";
        locative_masculine: "~ą~kim", "~ą~kim";
        locative_feminine: "~ą~kiej", "~ą~kiej";
        locative_neuter: "~ą~kim", "~ą~kim";
        locative_virile: "~ą~kich", "~ą~kich";
        locative_nonvirile: "~ą~kich", "~ą~kich";
        comparative: "~ężs~y";
        superlative: "naj~ężs~y";
    ),
    // Jawny: 4 words (jawny, jędrny, jowialny, jurny).
    declension!("jawny", 4, "~~y";
        nominative_masculine: "~~y", "~~y";
        nominative_feminine: "~~a", "~~a";
        nominative_neuter: "~~e", "~~e";
        nominative_virile: "~~i", "~~i";
        nominative_nonvirile: "~~e", "~~e";
        genitive_masculine: "~~ego", "~~ego";
        genitive_feminine: "~~ej", "~~ej";
        genitive_neuter: "~~ego", "~~ego";
        genitive_virile: "~~ych", "~~ych";
        genitive_nonvirile: "~~ych", "~~ych";
        dative_masculine: "~~emu", "~~emu";
        dative_feminine: "~~ej", "~~ej";
        dative_neuter: "~~emu", "~~emu";
        dative_virile: "~~ym", "~~ym";
        dative_nonvirile: "~~ym", "~~ym";
        accusative_masculine: "~~ego", "~~y";
        accusative_feminine: "~~ą", "~~ą";
        accusative_neuter: "~~e", "~~e";
        accusative_virile: "~~ych", "~~ych";
        accusative_nonvirile: "~~e", "~~e";
        instrumental_masculine: "~~ym", "~~ym";
        instrumental_feminine: "~~ą", "~~ą";
        instrumental_neuter: "~~ym", "~~ym";
        instrumental_virile: "~~ymi", "~~ymi";
        instrumental_nonvirile: "~~ymi", "~~ymi";
        locative_masculine: "~~ym", "~~ym";
        locative_feminine: "~~ej", "~~ej";
        locative_neuter: "~~ym", "~~ym";
        locative_virile: "~~ych", "~~ych";
        locative_nonvirile: "~~ych", "~~ych";
        comparative: "~~iejszy";
        superlative: "na~j~iejszy";
    ),
    // Rad: 4 words (rad, łaskaw, miłościw, córczyn).
    declension!("rad", 4, "~";
        nominative_masculine: "~", "~";
        nominative_feminine: "~a", "~a";
        nominative_neuter: "~e", "~e";
        nominative_virile: "~i", "~i";
        nominative_nonvirile: "~e", "~e";
        genitive_masculine: "~ego", "~ego";
        genitive_feminine: "~ej", "~ej";
        genitive_neuter: "~ego", "~ego";
        genitive_virile: "~ych", "~ych";
        genitive_nonvirile: "~ych", "~ych";
        dative_masculine: "~emu", "~emu";
        dative_feminine: "~ej", "~ej";
        dative_neuter: "~emu", "~emu";
        dative_virile: "~ym", "~ym";
        dative_nonvirile: "~ym", "~ym";
        accusative_masculine: "~ego", "~";
        accusative_feminine: "~ą", "~ą";
        accusative_neuter: "~e", "~e";
        accusative_virile: "~ych", "~ych";
        accusative_nonvirile: "~e", "~e";
        instrumental_masculine: "~ym", "~ym";
        instrumental_feminine: "~ą", "~ą";
        instrumental_neuter: "~ym", "~ym";
        instrumental_virile: "~ymi", "~ymi";
        instrumental_nonvirile: "~ymi", "~ymi";
        locative_masculine: "~ym", "~ym";
        locative_feminine: "~ej", "~ej";
        locative_neuter: "~ym", "~ym";
        locative_virile: "~ych", "~ych";
        locative_nonvirile: "~ych", "~ych";
        comparative: "~ejszy";
        superlative: "naj~ejszy";
    ),
    // Jaskrawy: 3 words (jaskrawy, jebliwy, jadliwy).
    declension!("jaskrawy", 3, "~~y";
        nominative_masculine: "~~y", "~~y";
        nominative_feminine: "~~a", "~~a";
        nominative_neuter: "~~e", "~~e";
        nominative_virile: "~~i", "~~i";
        nominative_nonvirile: "~~e", "~~e";
        genitive_masculine: "~~ego", "~~ego";
        genitive_feminine: "~~ej", "~~ej";
        genitive_neuter: "~~ego", "~~ego";
        genitive_virile: "~~ych", "~~ych";
        genitive_nonvirile: "~~ych", "~~ych";
        dative_masculine: "~~emu", "~~emu";
        dative_feminine: "~~ej", "~~ej";
        dative_neuter: "~~emu", "~~emu";
        dative_virile: "~~ym", "~~ym";
        dative_nonvirile: "~~ym", "~~ym";
        accusative_masculine: "~~ego", "~~y";
        accusative_feminine: "~~ą", "~~ą";
        accusative_neuter: "~~e", "~~e";
        accusative_virile: "~~ych", "~~ych";
        accusative_nonvirile: "~~e", "~~e";
        instrumental_masculine: "~~ym", "~~ym";
        instrumental_feminine: "~~ą", "~~ą";
        instrumental_neuter: "~~ym", "~~ym";
        instrumental_virile: "~~ymi", "~~ymi";
        instrumental_nonvirile: "~~ymi", "~~ymi";
        locative_masculine: "~~ym", "~~ym";
        locative_feminine: "~~ej", "~~ej";
        locative_neuter: "~~ym", "~~ym";
        locative_virile: "~~ych", "~~ych";
        locative_nonvirile: "~~ych", "~~ych";
        comparative: "~~szy";
        superlative: "na~j~szy";
    ),
    // Wielki: 3 words (wszelki, wielki, niewielki).
    declension!("wielki", 3, "~elki";
        nominative_masculine: "~elki", "~elki";
        nominative_feminine: "~elka", "~elka";
        nominative_neuter: "~elkie", "~elkie";
        nominative_virile: "~elcy", "~elcy";
        nominative_nonvirile: "~elkie", "~elkie";
        genitive_masculine: "~elkiego", "~elkiego";
        genitive_feminine: "~elkiej", "~elkiej";
        genitive_neuter: "~elkiego", "~elkiego";
        genitive_virile: "~elkich", "~elkich";
        genitive_nonvirile: "~elkich", "~elkich";
        dative_masculine: "~elkiemu", "~elkiemu";
        dative_feminine: "~elkiej", "~elkiej";
        dative_neuter: "~elkiemu", "~elkiemu";
        dative_virile: "~elkim", "~elkim";
        dative_nonvirile: "~elkim", "~elkim";
        accusative_masculine: "~elkiego", "~elki";
        accusative_feminine: "~elką", "~elką";
        accusative_neuter: "~elkie", "~elkie";
        accusative_virile: "~elkich", "~elkich";
        accusative_nonvirile: "~elkie", "~elkie";
        instrumental_masculine: "~elkim", "~elkim";
        instrumental_feminine: "~elką", "~elką";
        instrumental_neuter: "~elkim", "~elkim";
        instrumental_virile: "~elkimi", "~elkimi";
        instrumental_nonvirile: "~elkimi", "~elkimi";
        locative_masculine: "~elkim", "~elkim";
        locative_feminine: "~elkiej", "~elkiej";
        locative_neuter: "~elkim", "~elkim";
        locative_virile: "~elkich", "~elkich";
        locative_nonvirile: "~elkich", "~elkich";
        comparative: "~ększy";
        superlative: "naj~ększy";
    ),
    // Daleki: 3 words (daleki, niedaleki, kaleki).
    declension!("daleki", 3, "~eki";
        nominative_masculine: "~eki", "~eki";
        nominative_feminine: "~eka", "~eka";
        nominative_neuter: "~ekie", "~ekie";
        nominative_virile: "~ecy", "~ecy";
        nominative_nonvirile: "~ekie", "~ekie";
        genitive_masculine: "~ekiego", "~ekiego";
        genitive_feminine: "~ekiej", "~ekiej";
        genitive_neuter: "~ekiego", "~ekiego";
        genitive_virile: "~ekich", "~ekich";
        genitive_nonvirile: "~ekich", "~ekich";
        dative_masculine: "~ekiemu", "~ekiemu";
        dative_feminine: "~ekiej", "~ekiej";
        dative_neuter: "~ekiemu", "~ekiemu";
        dative_virile: "~ekim", "~ekim";
        dative_nonvirile: "~ekim", "~ekim";
        accusative_masculine: "~ekiego", "~eki";
        accusative_feminine: "~eką", "~eką";
        accusative_neuter: "~ekie", "~ekie";
        accusative_virile: "~ekich", "~ekich";
        accusative_nonvirile: "~ekie", "~ekie";
        instrumental_masculine: "~ekim", "~ekim";
        instrumental_feminine: "~eką", "~eką";
        instrumental_neuter: "~ekim", "~ekim";
        instrumental_virile: "~ekimi", "~ekimi";
        instrumental_nonvirile: "~ekimi", "~ekimi";
        locative_masculine: "~ekim", "~ekim";
        locative_feminine: "~ekiej", "~ekiej";
        locative_neuter: "~ekim", "~ekim";
        locative_virile: "~ekich", "~ekich";
        locative_nonvirile: "~ekich", "~ekich";
        comparative: "~szy";
        superlative: "naj~szy";
    ),
    // Blady: 3 words (gniady, blady, śniady).
    declension!("blady", 3, "~a~y";
        nominative_masculine: "~a~y", "~a~y";
        nominative_feminine: "~a~a", "~a~a";
        nominative_neuter: "~a~e", "~a~e";
        nominative_virile: "~a~zi", "~a~zi";
        nominative_nonvirile: "~a~e", "~a~e";
        genitive_masculine: "~a~ego", "~a~ego";
        genitive_feminine: "~a~ej", "~a~ej";
        genitive_neuter: "~a~ego", "~a~ego";
        genitive_virile: "~a~ych", "~a~ych";
        genitive_nonvirile: "~a~ych", "~a~ych";
        dative_masculine: "~a~emu", "~a~emu";
        dative_feminine: "~a~ej", "~a~ej";
        dative_neuter: "~a~emu", "~a~emu";
        dative_virile: "~a~ym", "~a~ym";
        dative_nonvirile: "~a~ym", "~a~ym";
        accusative_masculine: "~a~ego", "~a~y";
        accusative_feminine: "~a~ą", "~a~ą";
        accusative_neuter: "~a~e", "~a~e";
        accusative_virile: "~a~ych", "~a~ych";
        accusative_nonvirile: "~a~e", "~a~e";
        instrumental_masculine: "~a~ym", "~a~ym";
        instrumental_feminine: "~a~ą", "~a~ą";
        instrumental_neuter: "~a~ym", "~a~ym";
        instrumental_virile: "~a~ymi", "~a~ymi";
        instrumental_nonvirile: "~a~ymi", "~a~ymi";
        locative_masculine: "~a~ym", "~a~ym";
        locative_feminine: "~a~ej", "~a~ej";
        locative_neuter: "~a~ym", "~a~ym";
        locative_virile: "~a~ych", "~a~ych";
        locative_nonvirile: "~a~ych", "~a~ych";
        comparative: "~a~szy";
        superlative: "naj~a~szy";
    ),
    // Niedobry: 3 words (niedobry, arcydobry, przedobry).
    declension!("niedobry", 3, "~dobry";
        nominative_masculine: "~dobry", "~dobry";
        nominative_feminine: "~dobra", "~dobra";
        nominative_neuter: "~dobre", "~dobre";
        nominative_virile: "~dobrzy", "~dobrzy";
        nominative_nonvirile: "~dobre", "~dobre";
        genitive_masculine: "~dobrego", "~dobrego";
        genitive_feminine: "~dobrej", "~dobrej";
        genitive_neuter: "~dobrego", "~dobrego";
        genitive_virile: "~dobrych", "~dobrych";
        genitive_nonvirile: "~dobrych", "~dobrych";
        dative_masculine: "~dobremu", "~dobremu";
        dative_feminine: "~dobrej", "~dobrej";
        dative_neuter: "~dobremu", "~dobremu";
        dative_virile: "~dobrym", "~dobrym";
        dative_nonvirile: "~dobrym", "~dobrym";
        accusative_masculine: "~dobrego", "~dobry";
        accusative_feminine: "~dobrą", "~dobrą";
        accusative_neuter: "~dobre", "~dobre";
        accusative_virile: "~dobrych", "~dobrych";
        accusative_nonvirile: "~dobre", "~dobre";
        instrumental_masculine: "~dobrym", "~dobrym";
        instrumental_feminine: "~dobrą", "~dobrą";
        instrumental_neuter: "~dobrym", "~dobrym";
        instrumental_virile: "~dobrymi", "~dobrymi";
        instrumental_nonvirile: "~dobrymi", "~dobrymi";
        locative_masculine: "~dobrym", "~dobrym";
        locative_feminine: "~dobrej", "~dobrej";
        locative_neuter: "~dobrym", "~dobrym";
        locative_virile: "~dobrych", "~dobrych";
        locative_nonvirile: "~dobrych", "~dobrych";
        comparative: "~lepszy";
        superlative: "~najlepszy";
    ),
];
