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
            ]),
        }
    };
}

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `dwieście`: 10 words.
    Dwieście,
    /// Like `pięćdziesiąt`: 5 words.
    Pięćdziesiąt,
    /// Like `pięć`: 3 words.
    Pięć,
    /// Like `pięćset`: 3 words.
    Pięćset,
    /// Like `dwa`: 2 words.
    Dwa,
    /// Like `tyle`: 2 words.
    Tyle,
    /// Like `obydwa`: 2 words.
    Obydwa,
    /// Like `trzydzieści`: 2 words.
    Trzydzieści,
    /// Like `jeden`: 1 words.
    Jeden,
    /// Like `sto`: 1 words.
    Sto,
    /// Like `cztery`: 1 words.
    Cztery,
    /// Like `siedem`: 1 words.
    Siedem,
    /// Like `osiem`: 1 words.
    Osiem,
    /// Like `dwadzieścia`: 1 words.
    Dwadzieścia,
    /// Like `szereg`: 1 words.
    Szereg,
    /// Like `siedemset`: 1 words.
    Siedemset,
    /// Like `osiemset`: 1 words.
    Osiemset,
    /// Like `wtóry`: 1 words.
    Wtóry,
    /// Like `pińcet`: 1 words.
    Pińcet,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 19] = [
        Declension::Dwieście,
        Declension::Pięćdziesiąt,
        Declension::Pięć,
        Declension::Pięćset,
        Declension::Dwa,
        Declension::Tyle,
        Declension::Obydwa,
        Declension::Trzydzieści,
        Declension::Jeden,
        Declension::Sto,
        Declension::Cztery,
        Declension::Siedem,
        Declension::Osiem,
        Declension::Dwadzieścia,
        Declension::Szereg,
        Declension::Siedemset,
        Declension::Osiemset,
        Declension::Wtóry,
        Declension::Pińcet,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 19] = [
    // Dwieście: 10 words (dwanaście, piętnaście, jedenaście, trzynaście, czternaście, szesnaście).
    declension!("dwieście", 10, "~ście";
        nominative_masculine: "~y", "~y";
        nominative_feminine: "~a", "~a";
        nominative_neuter: "~e", "~e";
        nominative_virile: "~stu", "~stu";
        nominative_nonvirile: "~aj", "~aj";
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
        accusative_virile: "~stu", "~stu";
        accusative_nonvirile: "~u", "~u";
        instrumental_masculine: "~ym", "~ym";
        instrumental_feminine: "~ą", "~ą";
        instrumental_neuter: "~ym", "~ym";
        instrumental_virile: "~ymi", "~ymi";
        instrumental_nonvirile: "~oma", "~oma";
        locative_masculine: "~ym", "~ym";
        locative_feminine: "~ej", "~ej";
        locative_neuter: "~ym", "~ym";
        locative_virile: "~ych", "~ych";
        locative_nonvirile: "~ych", "~ych";
    ),
    // Pięćdziesiąt: 5 words (sześćdziesiąt, dziewięćdziesiąt, pięćdziesiąt, osiemdziesiąt, siedemdziesiąt).
    declension!("pięćdziesiąt", 5, "~ąt";
        nominative_masculine: "~y", "~y";
        nominative_feminine: "~a", "~a";
        nominative_neuter: "~e", "~e";
        nominative_virile: "~ęciu", "~ęciu";
        nominative_nonvirile: "~aj", "~aj";
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
        accusative_virile: "~ęciu", "~ęciu";
        accusative_nonvirile: "~u", "~u";
        instrumental_masculine: "~ym", "~ym";
        instrumental_feminine: "~ą", "~ą";
        instrumental_neuter: "~ym", "~ym";
        instrumental_virile: "~ymi", "~ymi";
        instrumental_nonvirile: "~oma", "~oma";
        locative_masculine: "~ym", "~ym";
        locative_feminine: "~ej", "~ej";
        locative_neuter: "~ym", "~ym";
        locative_virile: "~ych", "~ych";
        locative_nonvirile: "~ych", "~ych";
    ),
    // Pięć: 3 words (pięć, dziewięć, dziesięć).
    declension!("pięć", 3, "~ć";
        nominative_masculine: "~y", "~y";
        nominative_feminine: "~a", "~a";
        nominative_neuter: "~e", "~e";
        nominative_virile: "~ciu", "~ciu";
        nominative_nonvirile: "~aj", "~aj";
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
        accusative_virile: "~ciu", "~ciu";
        accusative_nonvirile: "~u", "~u";
        instrumental_masculine: "~ym", "~ym";
        instrumental_feminine: "~ą", "~ą";
        instrumental_neuter: "~ym", "~ym";
        instrumental_virile: "~ymi", "~ymi";
        instrumental_nonvirile: "~oma", "~oma";
        locative_masculine: "~ym", "~ym";
        locative_feminine: "~ej", "~ej";
        locative_neuter: "~ym", "~ym";
        locative_virile: "~ych", "~ych";
        locative_nonvirile: "~ych", "~ych";
    ),
    // Pięćset: 3 words (dziewięćset, pięćset, sześćset).
    declension!("pięćset", 3, "~ć~";
        nominative_masculine: "~e~", "~e~";
        nominative_feminine: "~~a", "~~a";
        nominative_neuter: "~~o", "~~o";
        nominative_virile: "~ciu~", "~ciu~";
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
        accusative_masculine: "~~ego", "~e~";
        accusative_feminine: "~~ą", "~~ą";
        accusative_neuter: "~~o", "~~o";
        accusative_virile: "~ciu~", "~ciu~";
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
    ),
    // Dwa: 2 words (oba, dwa).
    declension!("dwa", 2, "~a";
        nominative_masculine: "~y", "~y";
        nominative_feminine: "~a", "~a";
        nominative_neuter: "~e", "~e";
        nominative_virile: "~u", "~u";
        nominative_nonvirile: "~aj|~a", "~aj|~a";
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
        accusative_virile: "~u", "~u";
        accusative_nonvirile: "~u|~óch|~a", "~u|~óch|~a";
        instrumental_masculine: "~ym", "~ym";
        instrumental_feminine: "~ą", "~ą";
        instrumental_neuter: "~ym", "~ym";
        instrumental_virile: "~ymi", "~ymi";
        instrumental_nonvirile: "~oma|~u", "~oma|~u";
        locative_masculine: "~ym", "~ym";
        locative_feminine: "~ej", "~ej";
        locative_neuter: "~ym", "~ym";
        locative_virile: "~ych", "~ych";
        locative_nonvirile: "~ych", "~ych";
    ),
    // Tyle: 2 words (wiele, tyle).
    declension!("tyle", 2, "~e";
        nominative_masculine: "~y", "~y";
        nominative_feminine: "~a", "~a";
        nominative_neuter: "~e", "~e";
        nominative_virile: "~u", "~u";
        nominative_nonvirile: "~aj", "~aj";
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
        accusative_virile: "~u", "~u";
        accusative_nonvirile: "~u", "~u";
        instrumental_masculine: "~ym", "~ym";
        instrumental_feminine: "~ą", "~ą";
        instrumental_neuter: "~ym", "~ym";
        instrumental_virile: "~ymi", "~ymi";
        instrumental_nonvirile: "~oma", "~oma";
        locative_masculine: "~ym", "~ym";
        locative_feminine: "~ej", "~ej";
        locative_neuter: "~ym", "~ym";
        locative_virile: "~ych", "~ych";
        locative_nonvirile: "~ych", "~ych";
    ),
    // Obydwa: 2 words (trzysta, obydwa).
    declension!("obydwa", 2, "~a";
        nominative_masculine: "~y", "~y";
        nominative_feminine: "~a", "~a";
        nominative_neuter: "~e", "~e";
        nominative_virile: "~u", "~u";
        nominative_nonvirile: "~aj", "~aj";
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
        accusative_virile: "~u", "~u";
        accusative_nonvirile: "~u", "~u";
        instrumental_masculine: "~ym", "~ym";
        instrumental_feminine: "~ą", "~ą";
        instrumental_neuter: "~ym", "~ym";
        instrumental_virile: "~ymi", "~ymi";
        instrumental_nonvirile: "~oma", "~oma";
        locative_masculine: "~ym", "~ym";
        locative_feminine: "~ej", "~ej";
        locative_neuter: "~ym", "~ym";
        locative_virile: "~ych", "~ych";
        locative_nonvirile: "~ych", "~ych";
    ),
    // Trzydzieści: 2 words (trzydzieści, czterdzieści).
    declension!("trzydzieści", 2, "~ści";
        nominative_masculine: "~y", "~y";
        nominative_feminine: "~a", "~a";
        nominative_neuter: "~e", "~e";
        nominative_virile: "~stu", "~stu";
        nominative_nonvirile: "~aj", "~aj";
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
        accusative_virile: "~stu", "~stu";
        accusative_nonvirile: "~u", "~u";
        instrumental_masculine: "~ym", "~ym";
        instrumental_feminine: "~ą", "~ą";
        instrumental_neuter: "~ym", "~ym";
        instrumental_virile: "~ymi", "~ymi";
        instrumental_nonvirile: "~oma", "~oma";
        locative_masculine: "~ym", "~ym";
        locative_feminine: "~ej", "~ej";
        locative_neuter: "~ym", "~ym";
        locative_virile: "~ych", "~ych";
        locative_nonvirile: "~ych", "~ych";
    ),
    // Jeden: 1 words (jeden).
    declension!("jeden", 1, "~e~";
        nominative_masculine: "~e~", "~e~";
        nominative_feminine: "~~a", "~~a";
        nominative_neuter: "~~o", "~~o";
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
        accusative_masculine: "~~ego", "~e~";
        accusative_feminine: "~~ą", "~~ą";
        accusative_neuter: "~~o", "~~o";
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
    ),
    // Sto: 1 words (sto).
    declension!("sto", 1, "~o";
        nominative_masculine: "~y", "~y";
        nominative_feminine: "~a", "~a";
        nominative_neuter: "~e", "~e";
        nominative_virile: "~u", "~u";
        nominative_nonvirile: "~aj", "~aj";
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
        accusative_virile: "~u", "~u";
        accusative_nonvirile: "~u", "~u";
        instrumental_masculine: "~ym", "~ym";
        instrumental_feminine: "~ą", "~ą";
        instrumental_neuter: "~ym", "~ym";
        instrumental_virile: "~ymi", "~ymi";
        instrumental_nonvirile: "~oma", "~oma";
        locative_masculine: "~ym", "~ym";
        locative_feminine: "~ej", "~ej";
        locative_neuter: "~ym", "~ym";
        locative_virile: "~ych", "~ych";
        locative_nonvirile: "~ych", "~ych";
    ),
    // Cztery: 1 words (cztery).
    declension!("cztery", 1, "~y";
        nominative_masculine: "~y", "~y";
        nominative_feminine: "~a", "~a";
        nominative_neuter: "~e", "~e";
        nominative_virile: "~ej", "~ej";
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
        accusative_virile: "~ech", "~ech";
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
    ),
    // Siedem: 1 words (siedem).
    declension!("siedem", 1, "~e~";
        nominative_masculine: "~e~", "~e~";
        nominative_feminine: "~~a", "~~a";
        nominative_neuter: "~~o", "~~o";
        nominative_virile: "~~iu", "~~iu";
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
        accusative_masculine: "~~ego", "~e~";
        accusative_feminine: "~~ą", "~~ą";
        accusative_neuter: "~~o", "~~o";
        accusative_virile: "~~iu", "~~iu";
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
    ),
    // Osiem: 1 words (osiem).
    declension!("osiem", 1, "~sie~";
        nominative_masculine: "~e~", "~e~";
        nominative_feminine: "~~a", "~~a";
        nominative_neuter: "~~o", "~~o";
        nominative_virile: "~ś~iu", "~ś~iu";
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
        accusative_masculine: "~~ego", "~e~";
        accusative_feminine: "~~ą", "~~ą";
        accusative_neuter: "~~o", "~~o";
        accusative_virile: "~ś~iu", "~ś~iu";
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
    ),
    // Dwadzieścia: 1 words (dwadzieścia).
    declension!("dwadzieścia", 1, "~a~ścia";
        nominative_masculine: "~e~", "~e~";
        nominative_feminine: "~~a", "~~a";
        nominative_neuter: "~~o", "~~o";
        nominative_virile: "~u~stu", "~u~stu";
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
        accusative_masculine: "~~ego", "~e~";
        accusative_feminine: "~~ą", "~~ą";
        accusative_neuter: "~~o", "~~o";
        accusative_virile: "~u~stu", "~u~stu";
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
    ),
    // Szereg: 1 words (szereg).
    declension!("szereg", 1, "~";
        nominative_masculine: "~", "~";
        nominative_feminine: "~", "~";
        nominative_neuter: "~", "~";
        nominative_virile: "~", "~";
        nominative_nonvirile: "~", "~";
        genitive_masculine: "~u", "~u";
        genitive_feminine: "~u", "~u";
        genitive_neuter: "~u", "~u";
        genitive_virile: "~u", "~u";
        genitive_nonvirile: "~u", "~u";
        dative_masculine: "~owi", "~owi";
        dative_feminine: "~owi", "~owi";
        dative_neuter: "~owi", "~owi";
        dative_virile: "~owi", "~owi";
        dative_nonvirile: "~owi", "~owi";
        accusative_masculine: "~", "~";
        accusative_feminine: "~", "~";
        accusative_neuter: "~", "~";
        accusative_virile: "~", "~";
        accusative_nonvirile: "~", "~";
        instrumental_masculine: "~iem", "~iem";
        instrumental_feminine: "~iem", "~iem";
        instrumental_neuter: "~iem", "~iem";
        instrumental_virile: "~iem", "~iem";
        instrumental_nonvirile: "~iem", "~iem";
        locative_masculine: "~u", "~u";
        locative_feminine: "~u", "~u";
        locative_neuter: "~u", "~u";
        locative_virile: "~u", "~u";
        locative_nonvirile: "~u", "~u";
    ),
    // Siedemset: 1 words (siedemset).
    declension!("siedemset", 1, "~e~~";
        nominative_masculine: "~e~~", "~e~~";
        nominative_feminine: "~e~~", "~e~~";
        nominative_neuter: "~e~~", "~e~~";
        nominative_virile: "~~iu~", "~~iu~";
        nominative_nonvirile: "~e~~", "~e~~";
        genitive_masculine: "~e~~", "~e~~";
        genitive_feminine: "~e~~", "~e~~";
        genitive_neuter: "~e~~", "~e~~";
        genitive_virile: "~e~~", "~e~~";
        genitive_nonvirile: "~e~~", "~e~~";
        dative_masculine: "~e~~", "~e~~";
        dative_feminine: "~e~~", "~e~~";
        dative_neuter: "~e~~", "~e~~";
        dative_virile: "~e~~", "~e~~";
        dative_nonvirile: "~e~~", "~e~~";
        accusative_masculine: "~e~~", "~e~~";
        accusative_feminine: "~e~~", "~e~~";
        accusative_neuter: "~e~~", "~e~~";
        accusative_virile: "~~iu~", "~~iu~";
        accusative_nonvirile: "~e~~", "~e~~";
        instrumental_masculine: "~e~~", "~e~~";
        instrumental_feminine: "~e~~", "~e~~";
        instrumental_neuter: "~e~~", "~e~~";
        instrumental_virile: "~e~~", "~e~~";
        instrumental_nonvirile: "~e~~", "~e~~";
        locative_masculine: "~e~~", "~e~~";
        locative_feminine: "~e~~", "~e~~";
        locative_neuter: "~e~~", "~e~~";
        locative_virile: "~e~~", "~e~~";
        locative_nonvirile: "~e~~", "~e~~";
    ),
    // Osiemset: 1 words (osiemset).
    declension!("osiemset", 1, "~sie~~";
        nominative_masculine: "~sie~~", "~sie~~";
        nominative_feminine: "~sie~~", "~sie~~";
        nominative_neuter: "~sie~~", "~sie~~";
        nominative_virile: "~ś~iu~", "~ś~iu~";
        nominative_nonvirile: "~sie~~", "~sie~~";
        genitive_masculine: "~sie~~", "~sie~~";
        genitive_feminine: "~sie~~", "~sie~~";
        genitive_neuter: "~sie~~", "~sie~~";
        genitive_virile: "~sie~~", "~sie~~";
        genitive_nonvirile: "~sie~~", "~sie~~";
        dative_masculine: "~sie~~", "~sie~~";
        dative_feminine: "~sie~~", "~sie~~";
        dative_neuter: "~sie~~", "~sie~~";
        dative_virile: "~sie~~", "~sie~~";
        dative_nonvirile: "~sie~~", "~sie~~";
        accusative_masculine: "~sie~~", "~sie~~";
        accusative_feminine: "~sie~~", "~sie~~";
        accusative_neuter: "~sie~~", "~sie~~";
        accusative_virile: "~ś~iu~", "~ś~iu~";
        accusative_nonvirile: "~sie~~", "~sie~~";
        instrumental_masculine: "~sie~~", "~sie~~";
        instrumental_feminine: "~sie~~", "~sie~~";
        instrumental_neuter: "~sie~~", "~sie~~";
        instrumental_virile: "~sie~~", "~sie~~";
        instrumental_nonvirile: "~sie~~", "~sie~~";
        locative_masculine: "~sie~~", "~sie~~";
        locative_feminine: "~sie~~", "~sie~~";
        locative_neuter: "~sie~~", "~sie~~";
        locative_virile: "~sie~~", "~sie~~";
        locative_nonvirile: "~sie~~", "~sie~~";
    ),
    // Wtóry: 1 words (wtóry).
    declension!("wtóry", 1, "~y";
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
    ),
    // Pińcet: 1 words (pińcet).
    declension!("pińcet", 1, "~~";
        nominative_masculine: "~e~", "~e~";
        nominative_feminine: "~~a", "~~a";
        nominative_neuter: "~~o", "~~o";
        nominative_virile: "~ius~", "~ius~";
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
        accusative_masculine: "~~ego", "~e~";
        accusative_feminine: "~~ą", "~~ą";
        accusative_neuter: "~~o", "~~o";
        accusative_virile: "~ius~", "~ius~";
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
    ),
];
