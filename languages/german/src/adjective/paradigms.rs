//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's feature says):
/// `strong_nominative` (m, f, n, pl)
/// `strong_genitive` (m, f, n, pl)
/// `strong_dative` (m, f, n, pl)
/// `strong_accusative` (m, f, n, pl)
/// `weak_nominative` (m, f, n, pl)
/// `weak_genitive` (m, f, n, pl)
/// `weak_dative` (m, f, n, pl)
/// `weak_accusative` (m, f, n, pl)
/// `mixed_nominative` (m, f, n, pl)
/// `mixed_genitive` (m, f, n, pl)
/// `mixed_dative` (m, f, n, pl)
/// `mixed_accusative` (m, f, n, pl)
/// `comparative_strong_nominative` (m, f, n, pl)
/// `comparative_strong_genitive` (m, f, n, pl)
/// `comparative_strong_dative` (m, f, n, pl)
/// `comparative_strong_accusative` (m, f, n, pl)
/// `comparative_weak_nominative` (m, f, n, pl)
/// `comparative_weak_genitive` (m, f, n, pl)
/// `comparative_weak_dative` (m, f, n, pl)
/// `comparative_weak_accusative` (m, f, n, pl)
/// `comparative_mixed_nominative` (m, f, n, pl)
/// `comparative_mixed_genitive` (m, f, n, pl)
/// `comparative_mixed_dative` (m, f, n, pl)
/// `comparative_mixed_accusative` (m, f, n, pl)
/// `superlative_strong_nominative` (m, f, n, pl)
/// `superlative_strong_genitive` (m, f, n, pl)
/// `superlative_strong_dative` (m, f, n, pl)
/// `superlative_strong_accusative` (m, f, n, pl)
/// `superlative_weak_nominative` (m, f, n, pl)
/// `superlative_weak_genitive` (m, f, n, pl)
/// `superlative_weak_dative` (m, f, n, pl)
/// `superlative_weak_accusative` (m, f, n, pl)
/// `superlative_mixed_nominative` (m, f, n, pl)
/// `superlative_mixed_genitive` (m, f, n, pl)
/// `superlative_mixed_dative` (m, f, n, pl)
/// `superlative_mixed_accusative` (m, f, n, pl)
/// `predicative` (pos, comp, sup)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal;
     strong_nominative: $c0:tt, $c1:tt, $c2:tt, $c3:tt;
     strong_genitive: $c4:tt, $c5:tt, $c6:tt, $c7:tt;
     strong_dative: $c8:tt, $c9:tt, $c10:tt, $c11:tt;
     strong_accusative: $c12:tt, $c13:tt, $c14:tt, $c15:tt;
     weak_nominative: $c16:tt, $c17:tt, $c18:tt, $c19:tt;
     weak_genitive: $c20:tt, $c21:tt, $c22:tt, $c23:tt;
     weak_dative: $c24:tt, $c25:tt, $c26:tt, $c27:tt;
     weak_accusative: $c28:tt, $c29:tt, $c30:tt, $c31:tt;
     mixed_nominative: $c32:tt, $c33:tt, $c34:tt, $c35:tt;
     mixed_genitive: $c36:tt, $c37:tt, $c38:tt, $c39:tt;
     mixed_dative: $c40:tt, $c41:tt, $c42:tt, $c43:tt;
     mixed_accusative: $c44:tt, $c45:tt, $c46:tt, $c47:tt;
     comparative_strong_nominative: $c48:tt, $c49:tt, $c50:tt, $c51:tt;
     comparative_strong_genitive: $c52:tt, $c53:tt, $c54:tt, $c55:tt;
     comparative_strong_dative: $c56:tt, $c57:tt, $c58:tt, $c59:tt;
     comparative_strong_accusative: $c60:tt, $c61:tt, $c62:tt, $c63:tt;
     comparative_weak_nominative: $c64:tt, $c65:tt, $c66:tt, $c67:tt;
     comparative_weak_genitive: $c68:tt, $c69:tt, $c70:tt, $c71:tt;
     comparative_weak_dative: $c72:tt, $c73:tt, $c74:tt, $c75:tt;
     comparative_weak_accusative: $c76:tt, $c77:tt, $c78:tt, $c79:tt;
     comparative_mixed_nominative: $c80:tt, $c81:tt, $c82:tt, $c83:tt;
     comparative_mixed_genitive: $c84:tt, $c85:tt, $c86:tt, $c87:tt;
     comparative_mixed_dative: $c88:tt, $c89:tt, $c90:tt, $c91:tt;
     comparative_mixed_accusative: $c92:tt, $c93:tt, $c94:tt, $c95:tt;
     superlative_strong_nominative: $c96:tt, $c97:tt, $c98:tt, $c99:tt;
     superlative_strong_genitive: $c100:tt, $c101:tt, $c102:tt, $c103:tt;
     superlative_strong_dative: $c104:tt, $c105:tt, $c106:tt, $c107:tt;
     superlative_strong_accusative: $c108:tt, $c109:tt, $c110:tt, $c111:tt;
     superlative_weak_nominative: $c112:tt, $c113:tt, $c114:tt, $c115:tt;
     superlative_weak_genitive: $c116:tt, $c117:tt, $c118:tt, $c119:tt;
     superlative_weak_dative: $c120:tt, $c121:tt, $c122:tt, $c123:tt;
     superlative_weak_accusative: $c124:tt, $c125:tt, $c126:tt, $c127:tt;
     superlative_mixed_nominative: $c128:tt, $c129:tt, $c130:tt, $c131:tt;
     superlative_mixed_genitive: $c132:tt, $c133:tt, $c134:tt, $c135:tt;
     superlative_mixed_dative: $c136:tt, $c137:tt, $c138:tt, $c139:tt;
     superlative_mixed_accusative: $c140:tt, $c141:tt, $c142:tt, $c143:tt;
     predicative: $c144:tt, $c145:tt, $c146:tt;
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
                flexion::form!($c62),
                flexion::form!($c63),
                flexion::form!($c64),
                flexion::form!($c65),
                flexion::form!($c66),
                flexion::form!($c67),
                flexion::form!($c68),
                flexion::form!($c69),
                flexion::form!($c70),
                flexion::form!($c71),
                flexion::form!($c72),
                flexion::form!($c73),
                flexion::form!($c74),
                flexion::form!($c75),
                flexion::form!($c76),
                flexion::form!($c77),
                flexion::form!($c78),
                flexion::form!($c79),
                flexion::form!($c80),
                flexion::form!($c81),
                flexion::form!($c82),
                flexion::form!($c83),
                flexion::form!($c84),
                flexion::form!($c85),
                flexion::form!($c86),
                flexion::form!($c87),
                flexion::form!($c88),
                flexion::form!($c89),
                flexion::form!($c90),
                flexion::form!($c91),
                flexion::form!($c92),
                flexion::form!($c93),
                flexion::form!($c94),
                flexion::form!($c95),
                flexion::form!($c96),
                flexion::form!($c97),
                flexion::form!($c98),
                flexion::form!($c99),
                flexion::form!($c100),
                flexion::form!($c101),
                flexion::form!($c102),
                flexion::form!($c103),
                flexion::form!($c104),
                flexion::form!($c105),
                flexion::form!($c106),
                flexion::form!($c107),
                flexion::form!($c108),
                flexion::form!($c109),
                flexion::form!($c110),
                flexion::form!($c111),
                flexion::form!($c112),
                flexion::form!($c113),
                flexion::form!($c114),
                flexion::form!($c115),
                flexion::form!($c116),
                flexion::form!($c117),
                flexion::form!($c118),
                flexion::form!($c119),
                flexion::form!($c120),
                flexion::form!($c121),
                flexion::form!($c122),
                flexion::form!($c123),
                flexion::form!($c124),
                flexion::form!($c125),
                flexion::form!($c126),
                flexion::form!($c127),
                flexion::form!($c128),
                flexion::form!($c129),
                flexion::form!($c130),
                flexion::form!($c131),
                flexion::form!($c132),
                flexion::form!($c133),
                flexion::form!($c134),
                flexion::form!($c135),
                flexion::form!($c136),
                flexion::form!($c137),
                flexion::form!($c138),
                flexion::form!($c139),
                flexion::form!($c140),
                flexion::form!($c141),
                flexion::form!($c142),
                flexion::form!($c143),
                flexion::form!($c144),
                flexion::form!($c145),
                flexion::form!($c146),
            ]),
        }
    };
}

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `veraltet`: 8,389 words.
    Veraltet,
    /// Like `glatt`: 2,293 words.
    Glatt,
    /// Like `erkoren`: 386 words.
    Erkoren,
    /// Like `affig`: 301 words.
    Affig,
    /// Like `muffig`: 177 words.
    Muffig,
    /// Like `böse`: 130 words.
    Böse,
    /// Like `edel`: 90 words.
    Edel,
    /// Like `aufrecht`: 88 words.
    Aufrecht,
    /// Like `maßlos`: 60 words.
    Maßlos,
    /// Like `überfinanziert`: 40 words.
    Überfinanziert,
    /// Like `scharf`: 37 words.
    Scharf,
    /// Like `unsicher`: 25 words.
    Unsicher,
    /// Like `träge`: 24 words.
    Träge,
    /// Like `karg`: 19 words.
    Karg,
    /// Like `schwarz`: 17 words.
    Schwarz,
    /// Like `zeitnah`: 15 words.
    Zeitnah,
    /// Like `gedacht`: 13 words.
    Gedacht,
    /// Like `verschieden`: 12 words.
    Verschieden,
    /// Like `fade`: 10 words.
    Fade,
    /// Like `rot`: 10 words.
    Rot,
    /// Like `unedel`: 9 words.
    Unedel,
    /// Like `nug`: 8 words.
    Nug,
    /// Like `terer`: 8 words.
    Terer,
    /// Like `kurz`: 7 words.
    Kurz,
    /// Like `grob`: 6 words.
    Grob,
    /// Like `amyloid`: 5 words.
    Amyloid,
    /// Like `hoch`: 5 words.
    Hoch,
    /// Like `amtlich`: 5 words.
    Amtlich,
    /// Like `naß`: 5 words.
    Naß,
    /// Like `abgeschlossen`: 4 words.
    Abgeschlossen,
    /// Like `lila`: 3 words.
    Lila,
    /// Like `aper`: 3 words.
    Aper,
    /// Like `affabel`: 3 words.
    Affabel,
    /// Like `aschearm`: 3 words.
    Aschearm,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 34] = [
        Declension::Veraltet,
        Declension::Glatt,
        Declension::Erkoren,
        Declension::Affig,
        Declension::Muffig,
        Declension::Böse,
        Declension::Edel,
        Declension::Aufrecht,
        Declension::Maßlos,
        Declension::Überfinanziert,
        Declension::Scharf,
        Declension::Unsicher,
        Declension::Träge,
        Declension::Karg,
        Declension::Schwarz,
        Declension::Zeitnah,
        Declension::Gedacht,
        Declension::Verschieden,
        Declension::Fade,
        Declension::Rot,
        Declension::Unedel,
        Declension::Nug,
        Declension::Terer,
        Declension::Kurz,
        Declension::Grob,
        Declension::Amyloid,
        Declension::Hoch,
        Declension::Amtlich,
        Declension::Naß,
        Declension::Abgeschlossen,
        Declension::Lila,
        Declension::Aper,
        Declension::Affabel,
        Declension::Aschearm,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 34] = [
    // Veraltet: 8,389 words (plural, dick, hell, urban, global, mental).
    declension!("veraltet", 8389, "~";
        strong_nominative: "~er", "~e", "~es", "~e";
        strong_genitive: "~en", "~er", "~en", "~er";
        strong_dative: "~em", "~er", "~em", "~en";
        strong_accusative: "~en", "~e", "~es", "~e";
        weak_nominative: "~e", "~e", "~e", "~en";
        weak_genitive: "~en", "~en", "~en", "~en";
        weak_dative: "~en", "~en", "~en", "~en";
        weak_accusative: "~en", "~e", "~e", "~en";
        mixed_nominative: "~er", "~e", "~es", "~en";
        mixed_genitive: "~en", "~en", "~en", "~en";
        mixed_dative: "~en", "~en", "~en", "~en";
        mixed_accusative: "~en", "~e", "~es", "~en";
        comparative_strong_nominative: "~erer", "~ere", "~eres", "~ere";
        comparative_strong_genitive: "~eren", "~erer", "~eren", "~erer";
        comparative_strong_dative: "~erem", "~erer", "~erem", "~eren";
        comparative_strong_accusative: "~eren", "~ere", "~eres", "~ere";
        comparative_weak_nominative: "~ere", "~ere", "~ere", "~eren";
        comparative_weak_genitive: "~eren", "~eren", "~eren", "~eren";
        comparative_weak_dative: "~eren", "~eren", "~eren", "~eren";
        comparative_weak_accusative: "~eren", "~ere", "~ere", "~eren";
        comparative_mixed_nominative: "~erer", "~ere", "~eres", "~eren";
        comparative_mixed_genitive: "~eren", "~eren", "~eren", "~eren";
        comparative_mixed_dative: "~eren", "~eren", "~eren", "~eren";
        comparative_mixed_accusative: "~eren", "~ere", "~eres", "~eren";
        superlative_strong_nominative: "~ster", "~ste", "~stes", "~ste";
        superlative_strong_genitive: "~sten", "~ster", "~sten", "~ster";
        superlative_strong_dative: "~stem", "~ster", "~stem", "~sten";
        superlative_strong_accusative: "~sten", "~ste", "~stes", "~ste";
        superlative_weak_nominative: "~ste", "~ste", "~ste", "~sten";
        superlative_weak_genitive: "~sten", "~sten", "~sten", "~sten";
        superlative_weak_dative: "~sten", "~sten", "~sten", "~sten";
        superlative_weak_accusative: "~sten", "~ste", "~ste", "~sten";
        superlative_mixed_nominative: "~ster", "~ste", "~stes", "~sten";
        superlative_mixed_genitive: "~sten", "~sten", "~sten", "~sten";
        superlative_mixed_dative: "~sten", "~sten", "~sten", "~sten";
        superlative_mixed_accusative: "~sten", "~ste", "~stes", "~sten";
        predicative: "~", "~er", "am ~sten";
    ),
    // Glatt: 2,293 words (frei, flatulent, feist, new, pedant, blond).
    declension!("glatt", 2293, "~";
        strong_nominative: "~er", "~e", "~es", "~e";
        strong_genitive: "~en", "~er", "~en", "~er";
        strong_dative: "~em", "~er", "~em", "~en";
        strong_accusative: "~en", "~e", "~es", "~e";
        weak_nominative: "~e", "~e", "~e", "~en";
        weak_genitive: "~en", "~en", "~en", "~en";
        weak_dative: "~en", "~en", "~en", "~en";
        weak_accusative: "~en", "~e", "~e", "~en";
        mixed_nominative: "~er", "~e", "~es", "~en";
        mixed_genitive: "~en", "~en", "~en", "~en";
        mixed_dative: "~en", "~en", "~en", "~en";
        mixed_accusative: "~en", "~e", "~es", "~en";
        comparative_strong_nominative: "~erer", "~ere", "~eres", "~ere";
        comparative_strong_genitive: "~eren", "~erer", "~eren", "~erer";
        comparative_strong_dative: "~erem", "~erer", "~erem", "~eren";
        comparative_strong_accusative: "~eren", "~ere", "~eres", "~ere";
        comparative_weak_nominative: "~ere", "~ere", "~ere", "~eren";
        comparative_weak_genitive: "~eren", "~eren", "~eren", "~eren";
        comparative_weak_dative: "~eren", "~eren", "~eren", "~eren";
        comparative_weak_accusative: "~eren", "~ere", "~ere", "~eren";
        comparative_mixed_nominative: "~erer", "~ere", "~eres", "~eren";
        comparative_mixed_genitive: "~eren", "~eren", "~eren", "~eren";
        comparative_mixed_dative: "~eren", "~eren", "~eren", "~eren";
        comparative_mixed_accusative: "~eren", "~ere", "~eres", "~eren";
        superlative_strong_nominative: "~ester", "~este", "~estes", "~este";
        superlative_strong_genitive: "~esten", "~ester", "~esten", "~ester";
        superlative_strong_dative: "~estem", "~ester", "~estem", "~esten";
        superlative_strong_accusative: "~esten", "~este", "~estes", "~este";
        superlative_weak_nominative: "~este", "~este", "~este", "~esten";
        superlative_weak_genitive: "~esten", "~esten", "~esten", "~esten";
        superlative_weak_dative: "~esten", "~esten", "~esten", "~esten";
        superlative_weak_accusative: "~esten", "~este", "~este", "~esten";
        superlative_mixed_nominative: "~ester", "~este", "~estes", "~esten";
        superlative_mixed_genitive: "~esten", "~esten", "~esten", "~esten";
        superlative_mixed_dative: "~esten", "~esten", "~esten", "~esten";
        superlative_mixed_accusative: "~esten", "~este", "~estes", "~esten";
        predicative: "~", "~er", "am ~esten";
    ),
    // Erkoren: 386 words (gelaufen, gegessen, getrunken, gesprochen, gekommen, geboren).
    declension!("erkoren", 386, "~";
        strong_nominative: "~er", "~e", "~es", "~e";
        strong_genitive: "~en", "~er", "~en", "~er";
        strong_dative: "~em", "~er", "~em", "~en";
        strong_accusative: "~en", "~e", "~es", "~e";
        weak_nominative: "~e", "~e", "~e", "~en";
        weak_genitive: "~en", "~en", "~en", "~en";
        weak_dative: "~en", "~en", "~en", "~en";
        weak_accusative: "~en", "~e", "~e", "~en";
        mixed_nominative: "~er", "~e", "~es", "~en";
        mixed_genitive: "~en", "~en", "~en", "~en";
        mixed_dative: "~en", "~en", "~en", "~en";
        mixed_accusative: "~en", "~e", "~es", "~en";
        comparative_strong_nominative: "~erer", "~ere", "~eres", "~ere";
        comparative_strong_genitive: "~eren", "~erer", "~eren", "~erer";
        comparative_strong_dative: "~erem", "~erer", "~erem", "~eren";
        comparative_strong_accusative: "~eren", "~ere", "~eres", "~ere";
        comparative_weak_nominative: "~ere", "~ere", "~ere", "~eren";
        comparative_weak_genitive: "~eren", "~eren", "~eren", "~eren";
        comparative_weak_dative: "~eren", "~eren", "~eren", "~eren";
        comparative_weak_accusative: "~eren", "~ere", "~ere", "~eren";
        comparative_mixed_nominative: "~erer", "~ere", "~eres", "~eren";
        comparative_mixed_genitive: "~eren", "~eren", "~eren", "~eren";
        comparative_mixed_dative: "~eren", "~eren", "~eren", "~eren";
        comparative_mixed_accusative: "~eren", "~ere", "~eres", "~eren";
        superlative_strong_nominative: "~ster", "~ste", "~stes", "~ste";
        superlative_strong_genitive: "~sten", "~ster", "~sten", "~ster";
        superlative_strong_dative: "~stem", "~ster", "~stem", "~sten";
        superlative_strong_accusative: "~sten", "~ste", "~stes", "~ste";
        superlative_weak_nominative: "~ste", "~ste", "~ste", "~sten";
        superlative_weak_genitive: "~sten", "~sten", "~sten", "~sten";
        superlative_weak_dative: "~sten", "~sten", "~sten", "~sten";
        superlative_weak_accusative: "~sten", "~ste", "~ste", "~sten";
        superlative_mixed_nominative: "~ster", "~ste", "~stes", "~sten";
        superlative_mixed_genitive: "~sten", "~sten", "~sten", "~sten";
        superlative_mixed_dative: "~sten", "~sten", "~sten", "~sten";
        superlative_mixed_accusative: "~sten", "~ste", "~stes", "~sten";
        predicative: "ist ~|es ist ~|sind ~", "ist ~er", "ist am ~sten";
    ),
    // Affig: 301 words (abnormal, abgünstig, aktiv, aufmerksam, allgemein, abhängig).
    declension!("affig", 301, "~~";
        strong_nominative: "~~er", "~~e", "~~es", "~~e";
        strong_genitive: "~~en", "~~er", "~~en", "~~er";
        strong_dative: "~~em", "~~er", "~~em", "~~en";
        strong_accusative: "~~en", "~~e", "~~es", "~~e";
        weak_nominative: "~~e", "~~e", "~~e", "~~en";
        weak_genitive: "~~en", "~~en", "~~en", "~~en";
        weak_dative: "~~en", "~~en", "~~en", "~~en";
        weak_accusative: "~~en", "~~e", "~~e", "~~en";
        mixed_nominative: "~~er", "~~e", "~~es", "~~en";
        mixed_genitive: "~~en", "~~en", "~~en", "~~en";
        mixed_dative: "~~en", "~~en", "~~en", "~~en";
        mixed_accusative: "~~en", "~~e", "~~es", "~~en";
        comparative_strong_nominative: "~~erer", "~~ere", "~~eres", "~~ere";
        comparative_strong_genitive: "~~eren", "~~erer", "~~eren", "~~erer";
        comparative_strong_dative: "~~erem", "~~erer", "~~erem", "~~eren";
        comparative_strong_accusative: "~~eren", "~~ere", "~~eres", "~~ere";
        comparative_weak_nominative: "~~ere", "~~ere", "~~ere", "~~eren";
        comparative_weak_genitive: "~~eren", "~~eren", "~~eren", "~~eren";
        comparative_weak_dative: "~~eren", "~~eren", "~~eren", "~~eren";
        comparative_weak_accusative: "~~eren", "~~ere", "~~ere", "~~eren";
        comparative_mixed_nominative: "~~erer", "~~ere", "~~eres", "~~eren";
        comparative_mixed_genitive: "~~eren", "~~eren", "~~eren", "~~eren";
        comparative_mixed_dative: "~~eren", "~~eren", "~~eren", "~~eren";
        comparative_mixed_accusative: "~~eren", "~~ere", "~~eres", "~~eren";
        superlative_strong_nominative: "~~ster", "~~ste", "~~stes", "~~ste";
        superlative_strong_genitive: "~~sten", "~~ster", "~~sten", "~~ster";
        superlative_strong_dative: "~~stem", "~~ster", "~~stem", "~~sten";
        superlative_strong_accusative: "~~sten", "~~ste", "~~stes", "~~ste";
        superlative_weak_nominative: "~~ste", "~~ste", "~~ste", "~~sten";
        superlative_weak_genitive: "~~sten", "~~sten", "~~sten", "~~sten";
        superlative_weak_dative: "~~sten", "~~sten", "~~sten", "~~sten";
        superlative_weak_accusative: "~~sten", "~~ste", "~~ste", "~~sten";
        superlative_mixed_nominative: "~~ster", "~~ste", "~~stes", "~~sten";
        superlative_mixed_genitive: "~~sten", "~~sten", "~~sten", "~~sten";
        superlative_mixed_dative: "~~sten", "~~sten", "~~sten", "~~sten";
        superlative_mixed_accusative: "~~sten", "~~ste", "~~stes", "~~sten";
        predicative: "~~", "~~er", "~m a~sten";
    ),
    // Muffig: 177 words (mutig, mobil, minim, modern, modular, männlich).
    declension!("muffig", 177, "~~";
        strong_nominative: "~~er", "~~e", "~~es", "~~e";
        strong_genitive: "~~en", "~~er", "~~en", "~~er";
        strong_dative: "~~em", "~~er", "~~em", "~~en";
        strong_accusative: "~~en", "~~e", "~~es", "~~e";
        weak_nominative: "~~e", "~~e", "~~e", "~~en";
        weak_genitive: "~~en", "~~en", "~~en", "~~en";
        weak_dative: "~~en", "~~en", "~~en", "~~en";
        weak_accusative: "~~en", "~~e", "~~e", "~~en";
        mixed_nominative: "~~er", "~~e", "~~es", "~~en";
        mixed_genitive: "~~en", "~~en", "~~en", "~~en";
        mixed_dative: "~~en", "~~en", "~~en", "~~en";
        mixed_accusative: "~~en", "~~e", "~~es", "~~en";
        comparative_strong_nominative: "~~erer", "~~ere", "~~eres", "~~ere";
        comparative_strong_genitive: "~~eren", "~~erer", "~~eren", "~~erer";
        comparative_strong_dative: "~~erem", "~~erer", "~~erem", "~~eren";
        comparative_strong_accusative: "~~eren", "~~ere", "~~eres", "~~ere";
        comparative_weak_nominative: "~~ere", "~~ere", "~~ere", "~~eren";
        comparative_weak_genitive: "~~eren", "~~eren", "~~eren", "~~eren";
        comparative_weak_dative: "~~eren", "~~eren", "~~eren", "~~eren";
        comparative_weak_accusative: "~~eren", "~~ere", "~~ere", "~~eren";
        comparative_mixed_nominative: "~~erer", "~~ere", "~~eres", "~~eren";
        comparative_mixed_genitive: "~~eren", "~~eren", "~~eren", "~~eren";
        comparative_mixed_dative: "~~eren", "~~eren", "~~eren", "~~eren";
        comparative_mixed_accusative: "~~eren", "~~ere", "~~eres", "~~eren";
        superlative_strong_nominative: "~~ster", "~~ste", "~~stes", "~~ste";
        superlative_strong_genitive: "~~sten", "~~ster", "~~sten", "~~ster";
        superlative_strong_dative: "~~stem", "~~ster", "~~stem", "~~sten";
        superlative_strong_accusative: "~~sten", "~~ste", "~~stes", "~~ste";
        superlative_weak_nominative: "~~ste", "~~ste", "~~ste", "~~sten";
        superlative_weak_genitive: "~~sten", "~~sten", "~~sten", "~~sten";
        superlative_weak_dative: "~~sten", "~~sten", "~~sten", "~~sten";
        superlative_weak_accusative: "~~sten", "~~ste", "~~ste", "~~sten";
        superlative_mixed_nominative: "~~ster", "~~ste", "~~stes", "~~sten";
        superlative_mixed_genitive: "~~sten", "~~sten", "~~sten", "~~sten";
        superlative_mixed_dative: "~~sten", "~~sten", "~~sten", "~~sten";
        superlative_mixed_accusative: "~~sten", "~~ste", "~~stes", "~~sten";
        predicative: "~~", "~~er", "a~ m~sten";
    ),
    // Böse: 130 words (orange, safe, lose, woke, nice, müde).
    declension!("böse", 130, "~";
        strong_nominative: "~r", "~", "~s", "~";
        strong_genitive: "~n", "~r", "~n", "~r";
        strong_dative: "~m", "~r", "~m", "~n";
        strong_accusative: "~n", "~", "~s", "~";
        weak_nominative: "~", "~", "~", "~n";
        weak_genitive: "~n", "~n", "~n", "~n";
        weak_dative: "~n", "~n", "~n", "~n";
        weak_accusative: "~n", "~", "~", "~n";
        mixed_nominative: "~r", "~", "~s", "~n";
        mixed_genitive: "~n", "~n", "~n", "~n";
        mixed_dative: "~n", "~n", "~n", "~n";
        mixed_accusative: "~n", "~", "~s", "~n";
        comparative_strong_nominative: "~rer", "~re", "~res", "~re";
        comparative_strong_genitive: "~ren", "~rer", "~ren", "~rer";
        comparative_strong_dative: "~rem", "~rer", "~rem", "~ren";
        comparative_strong_accusative: "~ren", "~re", "~res", "~re";
        comparative_weak_nominative: "~re", "~re", "~re", "~ren";
        comparative_weak_genitive: "~ren", "~ren", "~ren", "~ren";
        comparative_weak_dative: "~ren", "~ren", "~ren", "~ren";
        comparative_weak_accusative: "~ren", "~re", "~re", "~ren";
        comparative_mixed_nominative: "~rer", "~re", "~res", "~ren";
        comparative_mixed_genitive: "~ren", "~ren", "~ren", "~ren";
        comparative_mixed_dative: "~ren", "~ren", "~ren", "~ren";
        comparative_mixed_accusative: "~ren", "~re", "~res", "~ren";
        superlative_strong_nominative: "~ster", "~ste", "~stes", "~ste";
        superlative_strong_genitive: "~sten", "~ster", "~sten", "~ster";
        superlative_strong_dative: "~stem", "~ster", "~stem", "~sten";
        superlative_strong_accusative: "~sten", "~ste", "~stes", "~ste";
        superlative_weak_nominative: "~ste", "~ste", "~ste", "~sten";
        superlative_weak_genitive: "~sten", "~sten", "~sten", "~sten";
        superlative_weak_dative: "~sten", "~sten", "~sten", "~sten";
        superlative_weak_accusative: "~sten", "~ste", "~ste", "~sten";
        superlative_mixed_nominative: "~ster", "~ste", "~stes", "~sten";
        superlative_mixed_genitive: "~sten", "~sten", "~sten", "~sten";
        superlative_mixed_dative: "~sten", "~sten", "~sten", "~sten";
        superlative_mixed_accusative: "~sten", "~ste", "~stes", "~sten";
        predicative: "~", "~r", "am ~sten";
    ),
    // Edel: 90 words (übel, nobel, dunkel, simpel, sauer, teuer).
    declension!("edel", 90, "~e~";
        strong_nominative: "~~er", "~~e", "~~es", "~~e";
        strong_genitive: "~~en", "~~er", "~~en", "~~er";
        strong_dative: "~~em", "~~er", "~~em", "~~en";
        strong_accusative: "~~en", "~~e", "~~es", "~~e";
        weak_nominative: "~~e", "~~e", "~~e", "~~en";
        weak_genitive: "~~en", "~~en", "~~en", "~~en";
        weak_dative: "~~en", "~~en", "~~en", "~~en";
        weak_accusative: "~~en", "~~e", "~~e", "~~en";
        mixed_nominative: "~~er", "~~e", "~~es", "~~en";
        mixed_genitive: "~~en", "~~en", "~~en", "~~en";
        mixed_dative: "~~en", "~~en", "~~en", "~~en";
        mixed_accusative: "~~en", "~~e", "~~es", "~~en";
        comparative_strong_nominative: "~~erer", "~~ere", "~~eres", "~~ere";
        comparative_strong_genitive: "~~eren", "~~erer", "~~eren", "~~erer";
        comparative_strong_dative: "~~erem", "~~erer", "~~erem", "~~eren";
        comparative_strong_accusative: "~~eren", "~~ere", "~~eres", "~~ere";
        comparative_weak_nominative: "~~ere", "~~ere", "~~ere", "~~eren";
        comparative_weak_genitive: "~~eren", "~~eren", "~~eren", "~~eren";
        comparative_weak_dative: "~~eren", "~~eren", "~~eren", "~~eren";
        comparative_weak_accusative: "~~eren", "~~ere", "~~ere", "~~eren";
        comparative_mixed_nominative: "~~erer", "~~ere", "~~eres", "~~eren";
        comparative_mixed_genitive: "~~eren", "~~eren", "~~eren", "~~eren";
        comparative_mixed_dative: "~~eren", "~~eren", "~~eren", "~~eren";
        comparative_mixed_accusative: "~~eren", "~~ere", "~~eres", "~~eren";
        superlative_strong_nominative: "~e~ster", "~e~ste", "~e~stes", "~e~ste";
        superlative_strong_genitive: "~e~sten", "~e~ster", "~e~sten", "~e~ster";
        superlative_strong_dative: "~e~stem", "~e~ster", "~e~stem", "~e~sten";
        superlative_strong_accusative: "~e~sten", "~e~ste", "~e~stes", "~e~ste";
        superlative_weak_nominative: "~e~ste", "~e~ste", "~e~ste", "~e~sten";
        superlative_weak_genitive: "~e~sten", "~e~sten", "~e~sten", "~e~sten";
        superlative_weak_dative: "~e~sten", "~e~sten", "~e~sten", "~e~sten";
        superlative_weak_accusative: "~e~sten", "~e~ste", "~e~ste", "~e~sten";
        superlative_mixed_nominative: "~e~ster", "~e~ste", "~e~stes", "~e~sten";
        superlative_mixed_genitive: "~e~sten", "~e~sten", "~e~sten", "~e~sten";
        superlative_mixed_dative: "~e~sten", "~e~sten", "~e~sten", "~e~sten";
        superlative_mixed_accusative: "~e~sten", "~e~ste", "~e~stes", "~e~sten";
        predicative: "~e~", "~~er", "am ~e~sten";
    ),
    // Aufrecht: 88 words (aberrant, abrupt, absurd, abundant, arid, arrogant).
    declension!("aufrecht", 88, "~~";
        strong_nominative: "~~er", "~~e", "~~es", "~~e";
        strong_genitive: "~~en", "~~er", "~~en", "~~er";
        strong_dative: "~~em", "~~er", "~~em", "~~en";
        strong_accusative: "~~en", "~~e", "~~es", "~~e";
        weak_nominative: "~~e", "~~e", "~~e", "~~en";
        weak_genitive: "~~en", "~~en", "~~en", "~~en";
        weak_dative: "~~en", "~~en", "~~en", "~~en";
        weak_accusative: "~~en", "~~e", "~~e", "~~en";
        mixed_nominative: "~~er", "~~e", "~~es", "~~en";
        mixed_genitive: "~~en", "~~en", "~~en", "~~en";
        mixed_dative: "~~en", "~~en", "~~en", "~~en";
        mixed_accusative: "~~en", "~~e", "~~es", "~~en";
        comparative_strong_nominative: "~~erer", "~~ere", "~~eres", "~~ere";
        comparative_strong_genitive: "~~eren", "~~erer", "~~eren", "~~erer";
        comparative_strong_dative: "~~erem", "~~erer", "~~erem", "~~eren";
        comparative_strong_accusative: "~~eren", "~~ere", "~~eres", "~~ere";
        comparative_weak_nominative: "~~ere", "~~ere", "~~ere", "~~eren";
        comparative_weak_genitive: "~~eren", "~~eren", "~~eren", "~~eren";
        comparative_weak_dative: "~~eren", "~~eren", "~~eren", "~~eren";
        comparative_weak_accusative: "~~eren", "~~ere", "~~ere", "~~eren";
        comparative_mixed_nominative: "~~erer", "~~ere", "~~eres", "~~eren";
        comparative_mixed_genitive: "~~eren", "~~eren", "~~eren", "~~eren";
        comparative_mixed_dative: "~~eren", "~~eren", "~~eren", "~~eren";
        comparative_mixed_accusative: "~~eren", "~~ere", "~~eres", "~~eren";
        superlative_strong_nominative: "~~ester", "~~este", "~~estes", "~~este";
        superlative_strong_genitive: "~~esten", "~~ester", "~~esten", "~~ester";
        superlative_strong_dative: "~~estem", "~~ester", "~~estem", "~~esten";
        superlative_strong_accusative: "~~esten", "~~este", "~~estes", "~~este";
        superlative_weak_nominative: "~~este", "~~este", "~~este", "~~esten";
        superlative_weak_genitive: "~~esten", "~~esten", "~~esten", "~~esten";
        superlative_weak_dative: "~~esten", "~~esten", "~~esten", "~~esten";
        superlative_weak_accusative: "~~esten", "~~este", "~~este", "~~esten";
        superlative_mixed_nominative: "~~ester", "~~este", "~~estes", "~~esten";
        superlative_mixed_genitive: "~~esten", "~~esten", "~~esten", "~~esten";
        superlative_mixed_dative: "~~esten", "~~esten", "~~esten", "~~esten";
        superlative_mixed_accusative: "~~esten", "~~este", "~~estes", "~~esten";
        predicative: "~~", "~~er", "~m a~esten";
    ),
    // Maßlos: 60 words (manifest, mies, mau, mild, matt, morbid).
    declension!("maßlos", 60, "~~";
        strong_nominative: "~~er", "~~e", "~~es", "~~e";
        strong_genitive: "~~en", "~~er", "~~en", "~~er";
        strong_dative: "~~em", "~~er", "~~em", "~~en";
        strong_accusative: "~~en", "~~e", "~~es", "~~e";
        weak_nominative: "~~e", "~~e", "~~e", "~~en";
        weak_genitive: "~~en", "~~en", "~~en", "~~en";
        weak_dative: "~~en", "~~en", "~~en", "~~en";
        weak_accusative: "~~en", "~~e", "~~e", "~~en";
        mixed_nominative: "~~er", "~~e", "~~es", "~~en";
        mixed_genitive: "~~en", "~~en", "~~en", "~~en";
        mixed_dative: "~~en", "~~en", "~~en", "~~en";
        mixed_accusative: "~~en", "~~e", "~~es", "~~en";
        comparative_strong_nominative: "~~erer", "~~ere", "~~eres", "~~ere";
        comparative_strong_genitive: "~~eren", "~~erer", "~~eren", "~~erer";
        comparative_strong_dative: "~~erem", "~~erer", "~~erem", "~~eren";
        comparative_strong_accusative: "~~eren", "~~ere", "~~eres", "~~ere";
        comparative_weak_nominative: "~~ere", "~~ere", "~~ere", "~~eren";
        comparative_weak_genitive: "~~eren", "~~eren", "~~eren", "~~eren";
        comparative_weak_dative: "~~eren", "~~eren", "~~eren", "~~eren";
        comparative_weak_accusative: "~~eren", "~~ere", "~~ere", "~~eren";
        comparative_mixed_nominative: "~~erer", "~~ere", "~~eres", "~~eren";
        comparative_mixed_genitive: "~~eren", "~~eren", "~~eren", "~~eren";
        comparative_mixed_dative: "~~eren", "~~eren", "~~eren", "~~eren";
        comparative_mixed_accusative: "~~eren", "~~ere", "~~eres", "~~eren";
        superlative_strong_nominative: "~~ester", "~~este", "~~estes", "~~este";
        superlative_strong_genitive: "~~esten", "~~ester", "~~esten", "~~ester";
        superlative_strong_dative: "~~estem", "~~ester", "~~estem", "~~esten";
        superlative_strong_accusative: "~~esten", "~~este", "~~estes", "~~este";
        superlative_weak_nominative: "~~este", "~~este", "~~este", "~~esten";
        superlative_weak_genitive: "~~esten", "~~esten", "~~esten", "~~esten";
        superlative_weak_dative: "~~esten", "~~esten", "~~esten", "~~esten";
        superlative_weak_accusative: "~~esten", "~~este", "~~este", "~~esten";
        superlative_mixed_nominative: "~~ester", "~~este", "~~estes", "~~esten";
        superlative_mixed_genitive: "~~esten", "~~esten", "~~esten", "~~esten";
        superlative_mixed_dative: "~~esten", "~~esten", "~~esten", "~~esten";
        superlative_mixed_accusative: "~~esten", "~~este", "~~estes", "~~esten";
        predicative: "~~", "~~er", "a~ m~esten";
    ),
    // Überfinanziert: 40 words (interessiert, diskutiert, abandonniert, desillusioniert, fragmentiert, deaktiviert).
    declension!("überfinanziert", 40, "~~";
        strong_nominative: "~~er", "~~e", "~~es", "~~e";
        strong_genitive: "~~en", "~~er", "~~en", "~~er";
        strong_dative: "~~em", "~~er", "~~em", "~~en";
        strong_accusative: "~~en", "~~e", "~~es", "~~e";
        weak_nominative: "~~e", "~~e", "~~e", "~~en";
        weak_genitive: "~~en", "~~en", "~~en", "~~en";
        weak_dative: "~~en", "~~en", "~~en", "~~en";
        weak_accusative: "~~en", "~~e", "~~e", "~~en";
        mixed_nominative: "~~er", "~~e", "~~es", "~~en";
        mixed_genitive: "~~en", "~~en", "~~en", "~~en";
        mixed_dative: "~~en", "~~en", "~~en", "~~en";
        mixed_accusative: "~~en", "~~e", "~~es", "~~en";
        comparative_strong_nominative: "~~erer", "~~ere", "~~eres", "~~ere";
        comparative_strong_genitive: "~~eren", "~~erer", "~~eren", "~~erer";
        comparative_strong_dative: "~~erem", "~~erer", "~~erem", "~~eren";
        comparative_strong_accusative: "~~eren", "~~ere", "~~eres", "~~ere";
        comparative_weak_nominative: "~~ere", "~~ere", "~~ere", "~~eren";
        comparative_weak_genitive: "~~eren", "~~eren", "~~eren", "~~eren";
        comparative_weak_dative: "~~eren", "~~eren", "~~eren", "~~eren";
        comparative_weak_accusative: "~~eren", "~~ere", "~~ere", "~~eren";
        comparative_mixed_nominative: "~~erer", "~~ere", "~~eres", "~~eren";
        comparative_mixed_genitive: "~~eren", "~~eren", "~~eren", "~~eren";
        comparative_mixed_dative: "~~eren", "~~eren", "~~eren", "~~eren";
        comparative_mixed_accusative: "~~eren", "~~ere", "~~eres", "~~eren";
        superlative_strong_nominative: "~~ster", "~~ste", "~~stes", "~~ste";
        superlative_strong_genitive: "~~sten", "~~ster", "~~sten", "~~ster";
        superlative_strong_dative: "~~stem", "~~ster", "~~stem", "~~sten";
        superlative_strong_accusative: "~~sten", "~~ste", "~~stes", "~~ste";
        superlative_weak_nominative: "~~ste", "~~ste", "~~ste", "~~sten";
        superlative_weak_genitive: "~~sten", "~~sten", "~~sten", "~~sten";
        superlative_weak_dative: "~~sten", "~~sten", "~~sten", "~~sten";
        superlative_weak_accusative: "~~sten", "~~ste", "~~ste", "~~sten";
        superlative_mixed_nominative: "~~ster", "~~ste", "~~stes", "~~sten";
        superlative_mixed_genitive: "~~sten", "~~sten", "~~sten", "~~sten";
        superlative_mixed_dative: "~~sten", "~~sten", "~~sten", "~~sten";
        superlative_mixed_accusative: "~~sten", "~~ste", "~~stes", "~~sten";
        predicative: "ist ~~", "ist ~~er", "ist ~m a~sten";
    ),
    // Scharf: 37 words (warm, stark, lang, scharf, schwach, krank).
    declension!("scharf", 37, "~a~";
        strong_nominative: "~a~er", "~a~e", "~a~es", "~a~e";
        strong_genitive: "~a~en", "~a~er", "~a~en", "~a~er";
        strong_dative: "~a~em", "~a~er", "~a~em", "~a~en";
        strong_accusative: "~a~en", "~a~e", "~a~es", "~a~e";
        weak_nominative: "~a~e", "~a~e", "~a~e", "~a~en";
        weak_genitive: "~a~en", "~a~en", "~a~en", "~a~en";
        weak_dative: "~a~en", "~a~en", "~a~en", "~a~en";
        weak_accusative: "~a~en", "~a~e", "~a~e", "~a~en";
        mixed_nominative: "~a~er", "~a~e", "~a~es", "~a~en";
        mixed_genitive: "~a~en", "~a~en", "~a~en", "~a~en";
        mixed_dative: "~a~en", "~a~en", "~a~en", "~a~en";
        mixed_accusative: "~a~en", "~a~e", "~a~es", "~a~en";
        comparative_strong_nominative: "~ä~erer", "~ä~ere", "~ä~eres", "~ä~ere";
        comparative_strong_genitive: "~ä~eren", "~ä~erer", "~ä~eren", "~ä~erer";
        comparative_strong_dative: "~ä~erem", "~ä~erer", "~ä~erem", "~ä~eren";
        comparative_strong_accusative: "~ä~eren", "~ä~ere", "~ä~eres", "~ä~ere";
        comparative_weak_nominative: "~ä~ere", "~ä~ere", "~ä~ere", "~ä~eren";
        comparative_weak_genitive: "~ä~eren", "~ä~eren", "~ä~eren", "~ä~eren";
        comparative_weak_dative: "~ä~eren", "~ä~eren", "~ä~eren", "~ä~eren";
        comparative_weak_accusative: "~ä~eren", "~ä~ere", "~ä~ere", "~ä~eren";
        comparative_mixed_nominative: "~ä~erer", "~ä~ere", "~ä~eres", "~ä~eren";
        comparative_mixed_genitive: "~ä~eren", "~ä~eren", "~ä~eren", "~ä~eren";
        comparative_mixed_dative: "~ä~eren", "~ä~eren", "~ä~eren", "~ä~eren";
        comparative_mixed_accusative: "~ä~eren", "~ä~ere", "~ä~eres", "~ä~eren";
        superlative_strong_nominative: "~ä~ster", "~ä~ste", "~ä~stes", "~ä~ste";
        superlative_strong_genitive: "~ä~sten", "~ä~ster", "~ä~sten", "~ä~ster";
        superlative_strong_dative: "~ä~stem", "~ä~ster", "~ä~stem", "~ä~sten";
        superlative_strong_accusative: "~ä~sten", "~ä~ste", "~ä~stes", "~ä~ste";
        superlative_weak_nominative: "~ä~ste", "~ä~ste", "~ä~ste", "~ä~sten";
        superlative_weak_genitive: "~ä~sten", "~ä~sten", "~ä~sten", "~ä~sten";
        superlative_weak_dative: "~ä~sten", "~ä~sten", "~ä~sten", "~ä~sten";
        superlative_weak_accusative: "~ä~sten", "~ä~ste", "~ä~ste", "~ä~sten";
        superlative_mixed_nominative: "~ä~ster", "~ä~ste", "~ä~stes", "~ä~sten";
        superlative_mixed_genitive: "~ä~sten", "~ä~sten", "~ä~sten", "~ä~sten";
        superlative_mixed_dative: "~ä~sten", "~ä~sten", "~ä~sten", "~ä~sten";
        superlative_mixed_accusative: "~ä~sten", "~ä~ste", "~ä~stes", "~ä~sten";
        predicative: "~a~", "~ä~er", "am ~ä~sten";
    ),
    // Unsicher: 25 words (sicher, clever, düster, hager, selbstsicher, unsicher).
    declension!("unsicher", 25, "~~";
        strong_nominative: "~~er", "~~e", "~~es", "~~e";
        strong_genitive: "~~en", "~~er", "~~en", "~~er";
        strong_dative: "~~em", "~~er", "~~em", "~~en";
        strong_accusative: "~~en", "~~e", "~~es", "~~e";
        weak_nominative: "~~e", "~~e", "~~e", "~~en";
        weak_genitive: "~~en", "~~en", "~~en", "~~en";
        weak_dative: "~~en", "~~en", "~~en", "~~en";
        weak_accusative: "~~en", "~~e", "~~e", "~~en";
        mixed_nominative: "~~er", "~~e", "~~es", "~~en";
        mixed_genitive: "~~en", "~~en", "~~en", "~~en";
        mixed_dative: "~~en", "~~en", "~~en", "~~en";
        mixed_accusative: "~~en", "~~e", "~~es", "~~en";
        comparative_strong_nominative: "~~erer|~r~er", "~~ere|~r~e", "~~eres|~r~es", "~~ere|~r~e";
        comparative_strong_genitive: "~~eren|~r~en", "~~erer|~r~er", "~~eren|~r~en", "~~erer|~r~er";
        comparative_strong_dative: "~~erem|~r~em", "~~erer|~r~er", "~~erem|~r~em", "~~eren|~r~en";
        comparative_strong_accusative: "~~eren|~r~en", "~~ere|~r~e", "~~eres|~r~es", "~~ere|~r~e";
        comparative_weak_nominative: "~~ere|~r~e", "~~ere|~r~e", "~~ere|~r~e", "~~eren|~r~en";
        comparative_weak_genitive: "~~eren|~r~en", "~~eren|~r~en", "~~eren|~r~en", "~~eren|~r~en";
        comparative_weak_dative: "~~eren|~r~en", "~~eren|~r~en", "~~eren|~r~en", "~~eren|~r~en";
        comparative_weak_accusative: "~~eren|~r~en", "~~ere|~r~e", "~~ere|~r~e", "~~eren|~r~en";
        comparative_mixed_nominative: "~~erer|~r~er", "~~ere|~r~e", "~~eres|~r~es", "~~eren|~r~en";
        comparative_mixed_genitive: "~~eren|~r~en", "~~eren|~r~en", "~~eren|~r~en", "~~eren|~r~en";
        comparative_mixed_dative: "~~eren|~r~en", "~~eren|~r~en", "~~eren|~r~en", "~~eren|~r~en";
        comparative_mixed_accusative: "~~eren|~r~en", "~~ere|~r~e", "~~eres|~r~es", "~~eren|~r~en";
        superlative_strong_nominative: "~~ster", "~~ste", "~~stes", "~~ste";
        superlative_strong_genitive: "~~sten", "~~ster", "~~sten", "~~ster";
        superlative_strong_dative: "~~stem", "~~ster", "~~stem", "~~sten";
        superlative_strong_accusative: "~~sten", "~~ste", "~~stes", "~~ste";
        superlative_weak_nominative: "~~ste", "~~ste", "~~ste", "~~sten";
        superlative_weak_genitive: "~~sten", "~~sten", "~~sten", "~~sten";
        superlative_weak_dative: "~~sten", "~~sten", "~~sten", "~~sten";
        superlative_weak_accusative: "~~sten", "~~ste", "~~ste", "~~sten";
        superlative_mixed_nominative: "~~ster", "~~ste", "~~stes", "~~sten";
        superlative_mixed_genitive: "~~sten", "~~sten", "~~sten", "~~sten";
        superlative_mixed_dative: "~~sten", "~~sten", "~~sten", "~~sten";
        superlative_mixed_accusative: "~~sten", "~~ste", "~~stes", "~~sten";
        predicative: "~~", "~~er|~r~", "am ~~sten";
    ),
    // Träge: 24 words (broke, malle, fake, knorke, äußere, zähe).
    declension!("träge", 24, "~~";
        strong_nominative: "~~r", "~~", "~~s", "~~";
        strong_genitive: "~~n", "~~r", "~~n", "~~r";
        strong_dative: "~~m", "~~r", "~~m", "~~n";
        strong_accusative: "~~n", "~~", "~~s", "~~";
        weak_nominative: "~~", "~~", "~~", "~~n";
        weak_genitive: "~~n", "~~n", "~~n", "~~n";
        weak_dative: "~~n", "~~n", "~~n", "~~n";
        weak_accusative: "~~n", "~~", "~~", "~~n";
        mixed_nominative: "~~r", "~~", "~~s", "~~n";
        mixed_genitive: "~~n", "~~n", "~~n", "~~n";
        mixed_dative: "~~n", "~~n", "~~n", "~~n";
        mixed_accusative: "~~n", "~~", "~~s", "~~n";
        comparative_strong_nominative: "~~rer", "~~re", "~~res", "~~re";
        comparative_strong_genitive: "~~ren", "~~rer", "~~ren", "~~rer";
        comparative_strong_dative: "~~rem", "~~rer", "~~rem", "~~ren";
        comparative_strong_accusative: "~~ren", "~~re", "~~res", "~~re";
        comparative_weak_nominative: "~~re", "~~re", "~~re", "~~ren";
        comparative_weak_genitive: "~~ren", "~~ren", "~~ren", "~~ren";
        comparative_weak_dative: "~~ren", "~~ren", "~~ren", "~~ren";
        comparative_weak_accusative: "~~ren", "~~re", "~~re", "~~ren";
        comparative_mixed_nominative: "~~rer", "~~re", "~~res", "~~ren";
        comparative_mixed_genitive: "~~ren", "~~ren", "~~ren", "~~ren";
        comparative_mixed_dative: "~~ren", "~~ren", "~~ren", "~~ren";
        comparative_mixed_accusative: "~~ren", "~~re", "~~res", "~~ren";
        superlative_strong_nominative: "~st~r", "~st~", "~st~s", "~st~";
        superlative_strong_genitive: "~st~n", "~st~r", "~st~n", "~st~r";
        superlative_strong_dative: "~st~m", "~st~r", "~st~m", "~st~n";
        superlative_strong_accusative: "~st~n", "~st~", "~st~s", "~st~";
        superlative_weak_nominative: "~st~", "~st~", "~st~", "~st~n";
        superlative_weak_genitive: "~st~n", "~st~n", "~st~n", "~st~n";
        superlative_weak_dative: "~st~n", "~st~n", "~st~n", "~st~n";
        superlative_weak_accusative: "~st~n", "~st~", "~st~", "~st~n";
        superlative_mixed_nominative: "~st~r", "~st~", "~st~s", "~st~n";
        superlative_mixed_genitive: "~st~n", "~st~n", "~st~n", "~st~n";
        superlative_mixed_dative: "~st~n", "~st~n", "~st~n", "~st~n";
        superlative_mixed_accusative: "~st~n", "~st~", "~st~s", "~st~n";
        predicative: "~~", "~~r", "am ~st~n";
    ),
    // Karg: 19 words (bang, schmal, karg, wochenlang, jahrelang, geraum).
    declension!("karg", 19, "~a~";
        strong_nominative: "~a~er", "~a~e", "~a~es", "~a~e";
        strong_genitive: "~a~en", "~a~er", "~a~en", "~a~er";
        strong_dative: "~a~em", "~a~er", "~a~em", "~a~en";
        strong_accusative: "~a~en", "~a~e", "~a~es", "~a~e";
        weak_nominative: "~a~e", "~a~e", "~a~e", "~a~en";
        weak_genitive: "~a~en", "~a~en", "~a~en", "~a~en";
        weak_dative: "~a~en", "~a~en", "~a~en", "~a~en";
        weak_accusative: "~a~en", "~a~e", "~a~e", "~a~en";
        mixed_nominative: "~a~er", "~a~e", "~a~es", "~a~en";
        mixed_genitive: "~a~en", "~a~en", "~a~en", "~a~en";
        mixed_dative: "~a~en", "~a~en", "~a~en", "~a~en";
        mixed_accusative: "~a~en", "~a~e", "~a~es", "~a~en";
        comparative_strong_nominative: "~a~erer", "~a~ere", "~a~eres", "~a~ere";
        comparative_strong_genitive: "~a~eren", "~a~erer", "~a~eren", "~a~erer";
        comparative_strong_dative: "~a~erem", "~a~erer", "~a~erem", "~a~eren";
        comparative_strong_accusative: "~a~eren", "~a~ere", "~a~eres", "~a~ere";
        comparative_weak_nominative: "~a~ere", "~a~ere", "~a~ere", "~a~eren";
        comparative_weak_genitive: "~a~eren", "~a~eren", "~a~eren", "~a~eren";
        comparative_weak_dative: "~a~eren", "~a~eren", "~a~eren", "~a~eren";
        comparative_weak_accusative: "~a~eren", "~a~ere", "~a~ere", "~a~eren";
        comparative_mixed_nominative: "~a~erer", "~a~ere", "~a~eres", "~a~eren";
        comparative_mixed_genitive: "~a~eren", "~a~eren", "~a~eren", "~a~eren";
        comparative_mixed_dative: "~a~eren", "~a~eren", "~a~eren", "~a~eren";
        comparative_mixed_accusative: "~a~eren", "~a~ere", "~a~eres", "~a~eren";
        superlative_strong_nominative: "~a~ster", "~a~ste", "~a~stes", "~a~ste";
        superlative_strong_genitive: "~a~sten", "~a~ster", "~a~sten", "~a~ster";
        superlative_strong_dative: "~a~stem", "~a~ster", "~a~stem", "~a~sten";
        superlative_strong_accusative: "~a~sten", "~a~ste", "~a~stes", "~a~ste";
        superlative_weak_nominative: "~a~ste", "~a~ste", "~a~ste", "~a~sten";
        superlative_weak_genitive: "~a~sten", "~a~sten", "~a~sten", "~a~sten";
        superlative_weak_dative: "~a~sten", "~a~sten", "~a~sten", "~a~sten";
        superlative_weak_accusative: "~a~sten", "~a~ste", "~a~ste", "~a~sten";
        superlative_mixed_nominative: "~a~ster", "~a~ste", "~a~stes", "~a~sten";
        superlative_mixed_genitive: "~a~sten", "~a~sten", "~a~sten", "~a~sten";
        superlative_mixed_dative: "~a~sten", "~a~sten", "~a~sten", "~a~sten";
        superlative_mixed_accusative: "~a~sten", "~a~ste", "~a~stes", "~a~sten";
        predicative: "~a~", "~a~er", "am ~a~sten";
    ),
    // Schwarz: 17 words (hart, kalt, schwarz, galt, pechschwarz, schweinekalt).
    declension!("schwarz", 17, "~a~";
        strong_nominative: "~a~er", "~a~e", "~a~es", "~a~e";
        strong_genitive: "~a~en", "~a~er", "~a~en", "~a~er";
        strong_dative: "~a~em", "~a~er", "~a~em", "~a~en";
        strong_accusative: "~a~en", "~a~e", "~a~es", "~a~e";
        weak_nominative: "~a~e", "~a~e", "~a~e", "~a~en";
        weak_genitive: "~a~en", "~a~en", "~a~en", "~a~en";
        weak_dative: "~a~en", "~a~en", "~a~en", "~a~en";
        weak_accusative: "~a~en", "~a~e", "~a~e", "~a~en";
        mixed_nominative: "~a~er", "~a~e", "~a~es", "~a~en";
        mixed_genitive: "~a~en", "~a~en", "~a~en", "~a~en";
        mixed_dative: "~a~en", "~a~en", "~a~en", "~a~en";
        mixed_accusative: "~a~en", "~a~e", "~a~es", "~a~en";
        comparative_strong_nominative: "~ä~erer", "~ä~ere", "~ä~eres", "~ä~ere";
        comparative_strong_genitive: "~ä~eren", "~ä~erer", "~ä~eren", "~ä~erer";
        comparative_strong_dative: "~ä~erem", "~ä~erer", "~ä~erem", "~ä~eren";
        comparative_strong_accusative: "~ä~eren", "~ä~ere", "~ä~eres", "~ä~ere";
        comparative_weak_nominative: "~ä~ere", "~ä~ere", "~ä~ere", "~ä~eren";
        comparative_weak_genitive: "~ä~eren", "~ä~eren", "~ä~eren", "~ä~eren";
        comparative_weak_dative: "~ä~eren", "~ä~eren", "~ä~eren", "~ä~eren";
        comparative_weak_accusative: "~ä~eren", "~ä~ere", "~ä~ere", "~ä~eren";
        comparative_mixed_nominative: "~ä~erer", "~ä~ere", "~ä~eres", "~ä~eren";
        comparative_mixed_genitive: "~ä~eren", "~ä~eren", "~ä~eren", "~ä~eren";
        comparative_mixed_dative: "~ä~eren", "~ä~eren", "~ä~eren", "~ä~eren";
        comparative_mixed_accusative: "~ä~eren", "~ä~ere", "~ä~eres", "~ä~eren";
        superlative_strong_nominative: "~ä~ester", "~ä~este", "~ä~estes", "~ä~este";
        superlative_strong_genitive: "~ä~esten", "~ä~ester", "~ä~esten", "~ä~ester";
        superlative_strong_dative: "~ä~estem", "~ä~ester", "~ä~estem", "~ä~esten";
        superlative_strong_accusative: "~ä~esten", "~ä~este", "~ä~estes", "~ä~este";
        superlative_weak_nominative: "~ä~este", "~ä~este", "~ä~este", "~ä~esten";
        superlative_weak_genitive: "~ä~esten", "~ä~esten", "~ä~esten", "~ä~esten";
        superlative_weak_dative: "~ä~esten", "~ä~esten", "~ä~esten", "~ä~esten";
        superlative_weak_accusative: "~ä~esten", "~ä~este", "~ä~este", "~ä~esten";
        superlative_mixed_nominative: "~ä~ester", "~ä~este", "~ä~estes", "~ä~esten";
        superlative_mixed_genitive: "~ä~esten", "~ä~esten", "~ä~esten", "~ä~esten";
        superlative_mixed_dative: "~ä~esten", "~ä~esten", "~ä~esten", "~ä~esten";
        superlative_mixed_accusative: "~ä~esten", "~ä~este", "~ä~estes", "~ä~esten";
        predicative: "~a~", "~ä~er", "am ~ä~esten";
    ),
    // Zeitnah: 15 words (nah, bodennah, körpernah, zeitnah, regierungsnah, citynah).
    declension!("zeitnah", 15, "~a~";
        strong_nominative: "~a~er", "~a~e", "~a~es", "~a~e";
        strong_genitive: "~a~en", "~a~er", "~a~en", "~a~er";
        strong_dative: "~a~em", "~a~er", "~a~em", "~a~en";
        strong_accusative: "~a~en", "~a~e", "~a~es", "~a~e";
        weak_nominative: "~a~e", "~a~e", "~a~e", "~a~en";
        weak_genitive: "~a~en", "~a~en", "~a~en", "~a~en";
        weak_dative: "~a~en", "~a~en", "~a~en", "~a~en";
        weak_accusative: "~a~en", "~a~e", "~a~e", "~a~en";
        mixed_nominative: "~a~er", "~a~e", "~a~es", "~a~en";
        mixed_genitive: "~a~en", "~a~en", "~a~en", "~a~en";
        mixed_dative: "~a~en", "~a~en", "~a~en", "~a~en";
        mixed_accusative: "~a~en", "~a~e", "~a~es", "~a~en";
        comparative_strong_nominative: "~ä~erer", "~ä~ere", "~ä~eres", "~ä~ere";
        comparative_strong_genitive: "~ä~eren", "~ä~erer", "~ä~eren", "~ä~erer";
        comparative_strong_dative: "~ä~erem", "~ä~erer", "~ä~erem", "~ä~eren";
        comparative_strong_accusative: "~ä~eren", "~ä~ere", "~ä~eres", "~ä~ere";
        comparative_weak_nominative: "~ä~ere", "~ä~ere", "~ä~ere", "~ä~eren";
        comparative_weak_genitive: "~ä~eren", "~ä~eren", "~ä~eren", "~ä~eren";
        comparative_weak_dative: "~ä~eren", "~ä~eren", "~ä~eren", "~ä~eren";
        comparative_weak_accusative: "~ä~eren", "~ä~ere", "~ä~ere", "~ä~eren";
        comparative_mixed_nominative: "~ä~erer", "~ä~ere", "~ä~eres", "~ä~eren";
        comparative_mixed_genitive: "~ä~eren", "~ä~eren", "~ä~eren", "~ä~eren";
        comparative_mixed_dative: "~ä~eren", "~ä~eren", "~ä~eren", "~ä~eren";
        comparative_mixed_accusative: "~ä~eren", "~ä~ere", "~ä~eres", "~ä~eren";
        superlative_strong_nominative: "~äc~ster", "~äc~ste", "~äc~stes", "~äc~ste";
        superlative_strong_genitive: "~äc~sten", "~äc~ster", "~äc~sten", "~äc~ster";
        superlative_strong_dative: "~äc~stem", "~äc~ster", "~äc~stem", "~äc~sten";
        superlative_strong_accusative: "~äc~sten", "~äc~ste", "~äc~stes", "~äc~ste";
        superlative_weak_nominative: "~äc~ste", "~äc~ste", "~äc~ste", "~äc~sten";
        superlative_weak_genitive: "~äc~sten", "~äc~sten", "~äc~sten", "~äc~sten";
        superlative_weak_dative: "~äc~sten", "~äc~sten", "~äc~sten", "~äc~sten";
        superlative_weak_accusative: "~äc~sten", "~äc~ste", "~äc~ste", "~äc~sten";
        superlative_mixed_nominative: "~äc~ster", "~äc~ste", "~äc~stes", "~äc~sten";
        superlative_mixed_genitive: "~äc~sten", "~äc~sten", "~äc~sten", "~äc~sten";
        superlative_mixed_dative: "~äc~sten", "~äc~sten", "~äc~sten", "~äc~sten";
        superlative_mixed_accusative: "~äc~sten", "~äc~ste", "~äc~stes", "~äc~sten";
        predicative: "~a~", "~ä~er", "am ~äc~sten";
    ),
    // Gedacht: 13 words (gedacht, gemacht, gekocht, besucht, geschwächt, eingemacht).
    declension!("gedacht", 13, "~~";
        strong_nominative: "~~er", "~~e", "~~es", "~~e";
        strong_genitive: "~~en", "~~er", "~~en", "~~er";
        strong_dative: "~~em", "~~er", "~~em", "~~en";
        strong_accusative: "~~en", "~~e", "~~es", "~~e";
        weak_nominative: "~~e", "~~e", "~~e", "~~en";
        weak_genitive: "~~en", "~~en", "~~en", "~~en";
        weak_dative: "~~en", "~~en", "~~en", "~~en";
        weak_accusative: "~~en", "~~e", "~~e", "~~en";
        mixed_nominative: "~~er", "~~e", "~~es", "~~en";
        mixed_genitive: "~~en", "~~en", "~~en", "~~en";
        mixed_dative: "~~en", "~~en", "~~en", "~~en";
        mixed_accusative: "~~en", "~~e", "~~es", "~~en";
        comparative_strong_nominative: "~~erer", "~~ere", "~~eres", "~~ere";
        comparative_strong_genitive: "~~eren", "~~erer", "~~eren", "~~erer";
        comparative_strong_dative: "~~erem", "~~erer", "~~erem", "~~eren";
        comparative_strong_accusative: "~~eren", "~~ere", "~~eres", "~~ere";
        comparative_weak_nominative: "~~ere", "~~ere", "~~ere", "~~eren";
        comparative_weak_genitive: "~~eren", "~~eren", "~~eren", "~~eren";
        comparative_weak_dative: "~~eren", "~~eren", "~~eren", "~~eren";
        comparative_weak_accusative: "~~eren", "~~ere", "~~ere", "~~eren";
        comparative_mixed_nominative: "~~erer", "~~ere", "~~eres", "~~eren";
        comparative_mixed_genitive: "~~eren", "~~eren", "~~eren", "~~eren";
        comparative_mixed_dative: "~~eren", "~~eren", "~~eren", "~~eren";
        comparative_mixed_accusative: "~~eren", "~~ere", "~~eres", "~~eren";
        superlative_strong_nominative: "~~ester", "~~este", "~~estes", "~~este";
        superlative_strong_genitive: "~~esten", "~~ester", "~~esten", "~~ester";
        superlative_strong_dative: "~~estem", "~~ester", "~~estem", "~~esten";
        superlative_strong_accusative: "~~esten", "~~este", "~~estes", "~~este";
        superlative_weak_nominative: "~~este", "~~este", "~~este", "~~esten";
        superlative_weak_genitive: "~~esten", "~~esten", "~~esten", "~~esten";
        superlative_weak_dative: "~~esten", "~~esten", "~~esten", "~~esten";
        superlative_weak_accusative: "~~esten", "~~este", "~~este", "~~esten";
        superlative_mixed_nominative: "~~ester", "~~este", "~~estes", "~~esten";
        superlative_mixed_genitive: "~~esten", "~~esten", "~~esten", "~~esten";
        superlative_mixed_dative: "~~esten", "~~esten", "~~esten", "~~esten";
        superlative_mixed_accusative: "~~esten", "~~este", "~~estes", "~~esten";
        predicative: "ist ~~", "ist ~~er", "ist ~m a~esten";
    ),
    // Verschieden: 12 words (willkommen, verschieden, eben, überlegen, gelassen, belesen).
    declension!("verschieden", 12, "~e~";
        strong_nominative: "~e~er", "~e~e", "~e~es", "~e~e";
        strong_genitive: "~e~en", "~e~er", "~e~en", "~e~er";
        strong_dative: "~e~em", "~e~er", "~e~em", "~e~en";
        strong_accusative: "~e~en", "~e~e", "~e~es", "~e~e";
        weak_nominative: "~e~e", "~e~e", "~e~e", "~e~en";
        weak_genitive: "~e~en", "~e~en", "~e~en", "~e~en";
        weak_dative: "~e~en", "~e~en", "~e~en", "~e~en";
        weak_accusative: "~e~en", "~e~e", "~e~e", "~e~en";
        mixed_nominative: "~e~er", "~e~e", "~e~es", "~e~en";
        mixed_genitive: "~e~en", "~e~en", "~e~en", "~e~en";
        mixed_dative: "~e~en", "~e~en", "~e~en", "~e~en";
        mixed_accusative: "~e~en", "~e~e", "~e~es", "~e~en";
        comparative_strong_nominative: "~e~erer|~~erer", "~e~ere|~~ere", "~e~eres|~~eres", "~e~ere|~~ere";
        comparative_strong_genitive: "~e~eren|~~eren", "~e~erer|~~erer", "~e~eren|~~eren", "~e~erer|~~erer";
        comparative_strong_dative: "~e~erem|~~erem", "~e~erer|~~erer", "~e~erem|~~erem", "~e~eren|~~eren";
        comparative_strong_accusative: "~e~eren|~~eren", "~e~ere|~~ere", "~e~eres|~~eres", "~e~ere|~~ere";
        comparative_weak_nominative: "~e~ere|~~ere", "~e~ere|~~ere", "~e~ere|~~ere", "~e~eren|~~eren";
        comparative_weak_genitive: "~e~eren|~~eren", "~e~eren|~~eren", "~e~eren|~~eren", "~e~eren|~~eren";
        comparative_weak_dative: "~e~eren|~~eren", "~e~eren|~~eren", "~e~eren|~~eren", "~e~eren|~~eren";
        comparative_weak_accusative: "~e~eren|~~eren", "~e~ere|~~ere", "~e~ere|~~ere", "~e~eren|~~eren";
        comparative_mixed_nominative: "~e~erer|~~erer", "~e~ere|~~ere", "~e~eres|~~eres", "~e~eren|~~eren";
        comparative_mixed_genitive: "~e~eren|~~eren", "~e~eren|~~eren", "~e~eren|~~eren", "~e~eren|~~eren";
        comparative_mixed_dative: "~e~eren|~~eren", "~e~eren|~~eren", "~e~eren|~~eren", "~e~eren|~~eren";
        comparative_mixed_accusative: "~e~eren|~~eren", "~e~ere|~~ere", "~e~eres|~~eres", "~e~eren|~~eren";
        superlative_strong_nominative: "~e~ster", "~e~ste", "~e~stes", "~e~ste";
        superlative_strong_genitive: "~e~sten", "~e~ster", "~e~sten", "~e~ster";
        superlative_strong_dative: "~e~stem", "~e~ster", "~e~stem", "~e~sten";
        superlative_strong_accusative: "~e~sten", "~e~ste", "~e~stes", "~e~ste";
        superlative_weak_nominative: "~e~ste", "~e~ste", "~e~ste", "~e~sten";
        superlative_weak_genitive: "~e~sten", "~e~sten", "~e~sten", "~e~sten";
        superlative_weak_dative: "~e~sten", "~e~sten", "~e~sten", "~e~sten";
        superlative_weak_accusative: "~e~sten", "~e~ste", "~e~ste", "~e~sten";
        superlative_mixed_nominative: "~e~ster", "~e~ste", "~e~stes", "~e~sten";
        superlative_mixed_genitive: "~e~sten", "~e~sten", "~e~sten", "~e~sten";
        superlative_mixed_dative: "~e~sten", "~e~sten", "~e~sten", "~e~sten";
        superlative_mixed_accusative: "~e~sten", "~e~ste", "~e~stes", "~e~sten";
        predicative: "~e~", "~e~er|~~er", "am ~e~sten";
    ),
    // Fade: 10 words (fade, ladylike, schnurgerade, helle, malade, dröge).
    declension!("fade", 10, "~~";
        strong_nominative: "~~r", "~~", "~~s", "~~";
        strong_genitive: "~~n", "~~r", "~~n", "~~r";
        strong_dative: "~~m", "~~r", "~~m", "~~n";
        strong_accusative: "~~n", "~~", "~~s", "~~";
        weak_nominative: "~~", "~~", "~~", "~~n";
        weak_genitive: "~~n", "~~n", "~~n", "~~n";
        weak_dative: "~~n", "~~n", "~~n", "~~n";
        weak_accusative: "~~n", "~~", "~~", "~~n";
        mixed_nominative: "~~r", "~~", "~~s", "~~n";
        mixed_genitive: "~~n", "~~n", "~~n", "~~n";
        mixed_dative: "~~n", "~~n", "~~n", "~~n";
        mixed_accusative: "~~n", "~~", "~~s", "~~n";
        comparative_strong_nominative: "~~rer", "~~re", "~~res", "~~re";
        comparative_strong_genitive: "~~ren", "~~rer", "~~ren", "~~rer";
        comparative_strong_dative: "~~rem", "~~rer", "~~rem", "~~ren";
        comparative_strong_accusative: "~~ren", "~~re", "~~res", "~~re";
        comparative_weak_nominative: "~~re", "~~re", "~~re", "~~ren";
        comparative_weak_genitive: "~~ren", "~~ren", "~~ren", "~~ren";
        comparative_weak_dative: "~~ren", "~~ren", "~~ren", "~~ren";
        comparative_weak_accusative: "~~ren", "~~re", "~~re", "~~ren";
        comparative_mixed_nominative: "~~rer", "~~re", "~~res", "~~ren";
        comparative_mixed_genitive: "~~ren", "~~ren", "~~ren", "~~ren";
        comparative_mixed_dative: "~~ren", "~~ren", "~~ren", "~~ren";
        comparative_mixed_accusative: "~~ren", "~~re", "~~res", "~~ren";
        superlative_strong_nominative: "~~ster", "~~ste", "~~stes", "~~ste";
        superlative_strong_genitive: "~~sten", "~~ster", "~~sten", "~~ster";
        superlative_strong_dative: "~~stem", "~~ster", "~~stem", "~~sten";
        superlative_strong_accusative: "~~sten", "~~ste", "~~stes", "~~ste";
        superlative_weak_nominative: "~~ste", "~~ste", "~~ste", "~~sten";
        superlative_weak_genitive: "~~sten", "~~sten", "~~sten", "~~sten";
        superlative_weak_dative: "~~sten", "~~sten", "~~sten", "~~sten";
        superlative_weak_accusative: "~~sten", "~~ste", "~~ste", "~~sten";
        superlative_mixed_nominative: "~~ster", "~~ste", "~~stes", "~~sten";
        superlative_mixed_genitive: "~~sten", "~~sten", "~~sten", "~~sten";
        superlative_mixed_dative: "~~sten", "~~sten", "~~sten", "~~sten";
        superlative_mixed_accusative: "~~sten", "~~ste", "~~stes", "~~sten";
        predicative: "~~", "~~r", "am ~~sten";
    ),
    // Rot: 10 words (rot, groß, gross, riesengroß, roth, flächengroß).
    declension!("rot", 10, "~o~";
        strong_nominative: "~o~er", "~o~e", "~o~es", "~o~e";
        strong_genitive: "~o~en", "~o~er", "~o~en", "~o~er";
        strong_dative: "~o~em", "~o~er", "~o~em", "~o~en";
        strong_accusative: "~o~en", "~o~e", "~o~es", "~o~e";
        weak_nominative: "~o~e", "~o~e", "~o~e", "~o~en";
        weak_genitive: "~o~en", "~o~en", "~o~en", "~o~en";
        weak_dative: "~o~en", "~o~en", "~o~en", "~o~en";
        weak_accusative: "~o~en", "~o~e", "~o~e", "~o~en";
        mixed_nominative: "~o~er", "~o~e", "~o~es", "~o~en";
        mixed_genitive: "~o~en", "~o~en", "~o~en", "~o~en";
        mixed_dative: "~o~en", "~o~en", "~o~en", "~o~en";
        mixed_accusative: "~o~en", "~o~e", "~o~es", "~o~en";
        comparative_strong_nominative: "~ö~erer", "~ö~ere", "~ö~eres", "~ö~ere";
        comparative_strong_genitive: "~ö~eren", "~ö~erer", "~ö~eren", "~ö~erer";
        comparative_strong_dative: "~ö~erem", "~ö~erer", "~ö~erem", "~ö~eren";
        comparative_strong_accusative: "~ö~eren", "~ö~ere", "~ö~eres", "~ö~ere";
        comparative_weak_nominative: "~ö~ere", "~ö~ere", "~ö~ere", "~ö~eren";
        comparative_weak_genitive: "~ö~eren", "~ö~eren", "~ö~eren", "~ö~eren";
        comparative_weak_dative: "~ö~eren", "~ö~eren", "~ö~eren", "~ö~eren";
        comparative_weak_accusative: "~ö~eren", "~ö~ere", "~ö~ere", "~ö~eren";
        comparative_mixed_nominative: "~ö~erer", "~ö~ere", "~ö~eres", "~ö~eren";
        comparative_mixed_genitive: "~ö~eren", "~ö~eren", "~ö~eren", "~ö~eren";
        comparative_mixed_dative: "~ö~eren", "~ö~eren", "~ö~eren", "~ö~eren";
        comparative_mixed_accusative: "~ö~eren", "~ö~ere", "~ö~eres", "~ö~eren";
        superlative_strong_nominative: "~ö~ter", "~ö~te", "~ö~tes", "~ö~te";
        superlative_strong_genitive: "~ö~ten", "~ö~ter", "~ö~ten", "~ö~ter";
        superlative_strong_dative: "~ö~tem", "~ö~ter", "~ö~tem", "~ö~ten";
        superlative_strong_accusative: "~ö~ten", "~ö~te", "~ö~tes", "~ö~te";
        superlative_weak_nominative: "~ö~te", "~ö~te", "~ö~te", "~ö~ten";
        superlative_weak_genitive: "~ö~ten", "~ö~ten", "~ö~ten", "~ö~ten";
        superlative_weak_dative: "~ö~ten", "~ö~ten", "~ö~ten", "~ö~ten";
        superlative_weak_accusative: "~ö~ten", "~ö~te", "~ö~te", "~ö~ten";
        superlative_mixed_nominative: "~ö~ter", "~ö~te", "~ö~tes", "~ö~ten";
        superlative_mixed_genitive: "~ö~ten", "~ö~ten", "~ö~ten", "~ö~ten";
        superlative_mixed_dative: "~ö~ten", "~ö~ten", "~ö~ten", "~ö~ten";
        superlative_mixed_accusative: "~ö~ten", "~ö~te", "~ö~tes", "~ö~ten";
        predicative: "~o~", "~ö~er", "am ~ö~ten";
    ),
    // Unedel: 9 words (integer, schofel, illuster, makaber, speiübel, unedel).
    declension!("unedel", 9, "~e~";
        strong_nominative: "~e~er", "~e~e", "~e~es", "~e~e";
        strong_genitive: "~e~en", "~e~er", "~e~en", "~e~er";
        strong_dative: "~e~em", "~e~er", "~e~em", "~e~en";
        strong_accusative: "~e~en", "~e~e", "~e~es", "~e~e";
        weak_nominative: "~e~e", "~e~e", "~e~e", "~e~en";
        weak_genitive: "~e~en", "~e~en", "~e~en", "~e~en";
        weak_dative: "~e~en", "~e~en", "~e~en", "~e~en";
        weak_accusative: "~e~en", "~e~e", "~e~e", "~e~en";
        mixed_nominative: "~e~er", "~e~e", "~e~es", "~e~en";
        mixed_genitive: "~e~en", "~e~en", "~e~en", "~e~en";
        mixed_dative: "~e~en", "~e~en", "~e~en", "~e~en";
        mixed_accusative: "~e~en", "~e~e", "~e~es", "~e~en";
        comparative_strong_nominative: "~~erer", "~~ere", "~~eres", "~~ere";
        comparative_strong_genitive: "~~eren", "~~erer", "~~eren", "~~erer";
        comparative_strong_dative: "~~erem", "~~erer", "~~erem", "~~eren";
        comparative_strong_accusative: "~~eren", "~~ere", "~~eres", "~~ere";
        comparative_weak_nominative: "~~ere", "~~ere", "~~ere", "~~eren";
        comparative_weak_genitive: "~~eren", "~~eren", "~~eren", "~~eren";
        comparative_weak_dative: "~~eren", "~~eren", "~~eren", "~~eren";
        comparative_weak_accusative: "~~eren", "~~ere", "~~ere", "~~eren";
        comparative_mixed_nominative: "~~erer", "~~ere", "~~eres", "~~eren";
        comparative_mixed_genitive: "~~eren", "~~eren", "~~eren", "~~eren";
        comparative_mixed_dative: "~~eren", "~~eren", "~~eren", "~~eren";
        comparative_mixed_accusative: "~~eren", "~~ere", "~~eres", "~~eren";
        superlative_strong_nominative: "~e~ster", "~e~ste", "~e~stes", "~e~ste";
        superlative_strong_genitive: "~e~sten", "~e~ster", "~e~sten", "~e~ster";
        superlative_strong_dative: "~e~stem", "~e~ster", "~e~stem", "~e~sten";
        superlative_strong_accusative: "~e~sten", "~e~ste", "~e~stes", "~e~ste";
        superlative_weak_nominative: "~e~ste", "~e~ste", "~e~ste", "~e~sten";
        superlative_weak_genitive: "~e~sten", "~e~sten", "~e~sten", "~e~sten";
        superlative_weak_dative: "~e~sten", "~e~sten", "~e~sten", "~e~sten";
        superlative_weak_accusative: "~e~sten", "~e~ste", "~e~ste", "~e~sten";
        superlative_mixed_nominative: "~e~ster", "~e~ste", "~e~stes", "~e~sten";
        superlative_mixed_genitive: "~e~sten", "~e~sten", "~e~sten", "~e~sten";
        superlative_mixed_dative: "~e~sten", "~e~sten", "~e~sten", "~e~sten";
        superlative_mixed_accusative: "~e~sten", "~e~ste", "~e~stes", "~e~sten";
        predicative: "~e~", "~~er", "am ~e~sten";
    ),
    // Nug: 8 words (dumm, jung, klug, unklug, blutjung, neunmalklug).
    declension!("nug", 8, "~u~";
        strong_nominative: "~u~er", "~u~e", "~u~es", "~u~e";
        strong_genitive: "~u~en", "~u~er", "~u~en", "~u~er";
        strong_dative: "~u~em", "~u~er", "~u~em", "~u~en";
        strong_accusative: "~u~en", "~u~e", "~u~es", "~u~e";
        weak_nominative: "~u~e", "~u~e", "~u~e", "~u~en";
        weak_genitive: "~u~en", "~u~en", "~u~en", "~u~en";
        weak_dative: "~u~en", "~u~en", "~u~en", "~u~en";
        weak_accusative: "~u~en", "~u~e", "~u~e", "~u~en";
        mixed_nominative: "~u~er", "~u~e", "~u~es", "~u~en";
        mixed_genitive: "~u~en", "~u~en", "~u~en", "~u~en";
        mixed_dative: "~u~en", "~u~en", "~u~en", "~u~en";
        mixed_accusative: "~u~en", "~u~e", "~u~es", "~u~en";
        comparative_strong_nominative: "~ü~erer", "~ü~ere", "~ü~eres", "~ü~ere";
        comparative_strong_genitive: "~ü~eren", "~ü~erer", "~ü~eren", "~ü~erer";
        comparative_strong_dative: "~ü~erem", "~ü~erer", "~ü~erem", "~ü~eren";
        comparative_strong_accusative: "~ü~eren", "~ü~ere", "~ü~eres", "~ü~ere";
        comparative_weak_nominative: "~ü~ere", "~ü~ere", "~ü~ere", "~ü~eren";
        comparative_weak_genitive: "~ü~eren", "~ü~eren", "~ü~eren", "~ü~eren";
        comparative_weak_dative: "~ü~eren", "~ü~eren", "~ü~eren", "~ü~eren";
        comparative_weak_accusative: "~ü~eren", "~ü~ere", "~ü~ere", "~ü~eren";
        comparative_mixed_nominative: "~ü~erer", "~ü~ere", "~ü~eres", "~ü~eren";
        comparative_mixed_genitive: "~ü~eren", "~ü~eren", "~ü~eren", "~ü~eren";
        comparative_mixed_dative: "~ü~eren", "~ü~eren", "~ü~eren", "~ü~eren";
        comparative_mixed_accusative: "~ü~eren", "~ü~ere", "~ü~eres", "~ü~eren";
        superlative_strong_nominative: "~ü~ster", "~ü~ste", "~ü~stes", "~ü~ste";
        superlative_strong_genitive: "~ü~sten", "~ü~ster", "~ü~sten", "~ü~ster";
        superlative_strong_dative: "~ü~stem", "~ü~ster", "~ü~stem", "~ü~sten";
        superlative_strong_accusative: "~ü~sten", "~ü~ste", "~ü~stes", "~ü~ste";
        superlative_weak_nominative: "~ü~ste", "~ü~ste", "~ü~ste", "~ü~sten";
        superlative_weak_genitive: "~ü~sten", "~ü~sten", "~ü~sten", "~ü~sten";
        superlative_weak_dative: "~ü~sten", "~ü~sten", "~ü~sten", "~ü~sten";
        superlative_weak_accusative: "~ü~sten", "~ü~ste", "~ü~ste", "~ü~sten";
        superlative_mixed_nominative: "~ü~ster", "~ü~ste", "~ü~stes", "~ü~sten";
        superlative_mixed_genitive: "~ü~sten", "~ü~sten", "~ü~sten", "~ü~sten";
        superlative_mixed_dative: "~ü~sten", "~ü~sten", "~ü~sten", "~ü~sten";
        superlative_mixed_accusative: "~ü~sten", "~ü~ste", "~ü~stes", "~ü~sten";
        predicative: "~u~", "~ü~er", "am ~ü~sten";
    ),
    // Terer: 8 words (nächster, oberer, erstbester, kantscher, tlerer, terer).
    declension!("terer", 8, "~~r";
        strong_nominative: "~~r", "~~", "~~s", "~~";
        strong_genitive: "~~n", "~~r", "~~n", "~~r";
        strong_dative: "~~m", "~~r", "~~m", "~~n";
        strong_accusative: "~~n", "~~", "~~s", "~~";
        weak_nominative: "~~", "~~", "~~", "~~n";
        weak_genitive: "~~n", "~~n", "~~n", "~~n";
        weak_dative: "~~n", "~~n", "~~n", "~~n";
        weak_accusative: "~~n", "~~", "~~", "~~n";
        mixed_nominative: "~~r", "~~", "~~s", "~~n";
        mixed_genitive: "~~n", "~~n", "~~n", "~~n";
        mixed_dative: "~~n", "~~n", "~~n", "~~n";
        mixed_accusative: "~~n", "~~", "~~s", "~~n";
        comparative_strong_nominative: "~~rer", "~~re", "~~res", "~~re";
        comparative_strong_genitive: "~~ren", "~~rer", "~~ren", "~~rer";
        comparative_strong_dative: "~~rem", "~~rer", "~~rem", "~~ren";
        comparative_strong_accusative: "~~ren", "~~re", "~~res", "~~re";
        comparative_weak_nominative: "~~re", "~~re", "~~re", "~~ren";
        comparative_weak_genitive: "~~ren", "~~ren", "~~ren", "~~ren";
        comparative_weak_dative: "~~ren", "~~ren", "~~ren", "~~ren";
        comparative_weak_accusative: "~~ren", "~~re", "~~re", "~~ren";
        comparative_mixed_nominative: "~~rer", "~~re", "~~res", "~~ren";
        comparative_mixed_genitive: "~~ren", "~~ren", "~~ren", "~~ren";
        comparative_mixed_dative: "~~ren", "~~ren", "~~ren", "~~ren";
        comparative_mixed_accusative: "~~ren", "~~re", "~~res", "~~ren";
        superlative_strong_nominative: "~st~r", "~st~", "~st~s", "~st~";
        superlative_strong_genitive: "~st~n", "~st~r", "~st~n", "~st~r";
        superlative_strong_dative: "~st~m", "~st~r", "~st~m", "~st~n";
        superlative_strong_accusative: "~st~n", "~st~", "~st~s", "~st~";
        superlative_weak_nominative: "~st~", "~st~", "~st~", "~st~n";
        superlative_weak_genitive: "~st~n", "~st~n", "~st~n", "~st~n";
        superlative_weak_dative: "~st~n", "~st~n", "~st~n", "~st~n";
        superlative_weak_accusative: "~st~n", "~st~", "~st~", "~st~n";
        superlative_mixed_nominative: "~st~r", "~st~", "~st~s", "~st~n";
        superlative_mixed_genitive: "~st~n", "~st~n", "~st~n", "~st~n";
        superlative_mixed_dative: "~st~n", "~st~n", "~st~n", "~st~n";
        superlative_mixed_accusative: "~st~n", "~st~", "~st~s", "~st~n";
        predicative: "~~", "~~r", "am ~st~n";
    ),
    // Kurz: 7 words (kurz, gesund, schnurz, ungesund, megakurz, kurtz).
    declension!("kurz", 7, "~u~";
        strong_nominative: "~u~er", "~u~e", "~u~es", "~u~e";
        strong_genitive: "~u~en", "~u~er", "~u~en", "~u~er";
        strong_dative: "~u~em", "~u~er", "~u~em", "~u~en";
        strong_accusative: "~u~en", "~u~e", "~u~es", "~u~e";
        weak_nominative: "~u~e", "~u~e", "~u~e", "~u~en";
        weak_genitive: "~u~en", "~u~en", "~u~en", "~u~en";
        weak_dative: "~u~en", "~u~en", "~u~en", "~u~en";
        weak_accusative: "~u~en", "~u~e", "~u~e", "~u~en";
        mixed_nominative: "~u~er", "~u~e", "~u~es", "~u~en";
        mixed_genitive: "~u~en", "~u~en", "~u~en", "~u~en";
        mixed_dative: "~u~en", "~u~en", "~u~en", "~u~en";
        mixed_accusative: "~u~en", "~u~e", "~u~es", "~u~en";
        comparative_strong_nominative: "~ü~erer", "~ü~ere", "~ü~eres", "~ü~ere";
        comparative_strong_genitive: "~ü~eren", "~ü~erer", "~ü~eren", "~ü~erer";
        comparative_strong_dative: "~ü~erem", "~ü~erer", "~ü~erem", "~ü~eren";
        comparative_strong_accusative: "~ü~eren", "~ü~ere", "~ü~eres", "~ü~ere";
        comparative_weak_nominative: "~ü~ere", "~ü~ere", "~ü~ere", "~ü~eren";
        comparative_weak_genitive: "~ü~eren", "~ü~eren", "~ü~eren", "~ü~eren";
        comparative_weak_dative: "~ü~eren", "~ü~eren", "~ü~eren", "~ü~eren";
        comparative_weak_accusative: "~ü~eren", "~ü~ere", "~ü~ere", "~ü~eren";
        comparative_mixed_nominative: "~ü~erer", "~ü~ere", "~ü~eres", "~ü~eren";
        comparative_mixed_genitive: "~ü~eren", "~ü~eren", "~ü~eren", "~ü~eren";
        comparative_mixed_dative: "~ü~eren", "~ü~eren", "~ü~eren", "~ü~eren";
        comparative_mixed_accusative: "~ü~eren", "~ü~ere", "~ü~eres", "~ü~eren";
        superlative_strong_nominative: "~ü~ester", "~ü~este", "~ü~estes", "~ü~este";
        superlative_strong_genitive: "~ü~esten", "~ü~ester", "~ü~esten", "~ü~ester";
        superlative_strong_dative: "~ü~estem", "~ü~ester", "~ü~estem", "~ü~esten";
        superlative_strong_accusative: "~ü~esten", "~ü~este", "~ü~estes", "~ü~este";
        superlative_weak_nominative: "~ü~este", "~ü~este", "~ü~este", "~ü~esten";
        superlative_weak_genitive: "~ü~esten", "~ü~esten", "~ü~esten", "~ü~esten";
        superlative_weak_dative: "~ü~esten", "~ü~esten", "~ü~esten", "~ü~esten";
        superlative_weak_accusative: "~ü~esten", "~ü~este", "~ü~este", "~ü~esten";
        superlative_mixed_nominative: "~ü~ester", "~ü~este", "~ü~estes", "~ü~esten";
        superlative_mixed_genitive: "~ü~esten", "~ü~esten", "~ü~esten", "~ü~esten";
        superlative_mixed_dative: "~ü~esten", "~ü~esten", "~ü~esten", "~ü~esten";
        superlative_mixed_accusative: "~ü~esten", "~ü~este", "~ü~estes", "~ü~esten";
        predicative: "~u~", "~ü~er", "am ~ü~esten";
    ),
    // Grob: 6 words (grob, fromm, aerob, lammfromm, storr, exaerob).
    declension!("grob", 6, "~o~";
        strong_nominative: "~o~er", "~o~e", "~o~es", "~o~e";
        strong_genitive: "~o~en", "~o~er", "~o~en", "~o~er";
        strong_dative: "~o~em", "~o~er", "~o~em", "~o~en";
        strong_accusative: "~o~en", "~o~e", "~o~es", "~o~e";
        weak_nominative: "~o~e", "~o~e", "~o~e", "~o~en";
        weak_genitive: "~o~en", "~o~en", "~o~en", "~o~en";
        weak_dative: "~o~en", "~o~en", "~o~en", "~o~en";
        weak_accusative: "~o~en", "~o~e", "~o~e", "~o~en";
        mixed_nominative: "~o~er", "~o~e", "~o~es", "~o~en";
        mixed_genitive: "~o~en", "~o~en", "~o~en", "~o~en";
        mixed_dative: "~o~en", "~o~en", "~o~en", "~o~en";
        mixed_accusative: "~o~en", "~o~e", "~o~es", "~o~en";
        comparative_strong_nominative: "~ö~erer", "~ö~ere", "~ö~eres", "~ö~ere";
        comparative_strong_genitive: "~ö~eren", "~ö~erer", "~ö~eren", "~ö~erer";
        comparative_strong_dative: "~ö~erem", "~ö~erer", "~ö~erem", "~ö~eren";
        comparative_strong_accusative: "~ö~eren", "~ö~ere", "~ö~eres", "~ö~ere";
        comparative_weak_nominative: "~ö~ere", "~ö~ere", "~ö~ere", "~ö~eren";
        comparative_weak_genitive: "~ö~eren", "~ö~eren", "~ö~eren", "~ö~eren";
        comparative_weak_dative: "~ö~eren", "~ö~eren", "~ö~eren", "~ö~eren";
        comparative_weak_accusative: "~ö~eren", "~ö~ere", "~ö~ere", "~ö~eren";
        comparative_mixed_nominative: "~ö~erer", "~ö~ere", "~ö~eres", "~ö~eren";
        comparative_mixed_genitive: "~ö~eren", "~ö~eren", "~ö~eren", "~ö~eren";
        comparative_mixed_dative: "~ö~eren", "~ö~eren", "~ö~eren", "~ö~eren";
        comparative_mixed_accusative: "~ö~eren", "~ö~ere", "~ö~eres", "~ö~eren";
        superlative_strong_nominative: "~ö~ster", "~ö~ste", "~ö~stes", "~ö~ste";
        superlative_strong_genitive: "~ö~sten", "~ö~ster", "~ö~sten", "~ö~ster";
        superlative_strong_dative: "~ö~stem", "~ö~ster", "~ö~stem", "~ö~sten";
        superlative_strong_accusative: "~ö~sten", "~ö~ste", "~ö~stes", "~ö~ste";
        superlative_weak_nominative: "~ö~ste", "~ö~ste", "~ö~ste", "~ö~sten";
        superlative_weak_genitive: "~ö~sten", "~ö~sten", "~ö~sten", "~ö~sten";
        superlative_weak_dative: "~ö~sten", "~ö~sten", "~ö~sten", "~ö~sten";
        superlative_weak_accusative: "~ö~sten", "~ö~ste", "~ö~ste", "~ö~sten";
        superlative_mixed_nominative: "~ö~ster", "~ö~ste", "~ö~stes", "~ö~sten";
        superlative_mixed_genitive: "~ö~sten", "~ö~sten", "~ö~sten", "~ö~sten";
        superlative_mixed_dative: "~ö~sten", "~ö~sten", "~ö~sten", "~ö~sten";
        superlative_mixed_accusative: "~ö~sten", "~ö~ste", "~ö~stes", "~ö~sten";
        predicative: "~o~", "~ö~er", "am ~ö~sten";
    ),
    // Amyloid: 5 words (ambivalent, amyloid, ambitiös, amüsant, ambitioniert).
    declension!("amyloid", 5, "~~";
        strong_nominative: "~~er", "~~e", "~~es", "~~e";
        strong_genitive: "~~en", "~~er", "~~en", "~~er";
        strong_dative: "~~em", "~~er", "~~em", "~~en";
        strong_accusative: "~~en", "~~e", "~~es", "~~e";
        weak_nominative: "~~e", "~~e", "~~e", "~~en";
        weak_genitive: "~~en", "~~en", "~~en", "~~en";
        weak_dative: "~~en", "~~en", "~~en", "~~en";
        weak_accusative: "~~en", "~~e", "~~e", "~~en";
        mixed_nominative: "~~er", "~~e", "~~es", "~~en";
        mixed_genitive: "~~en", "~~en", "~~en", "~~en";
        mixed_dative: "~~en", "~~en", "~~en", "~~en";
        mixed_accusative: "~~en", "~~e", "~~es", "~~en";
        comparative_strong_nominative: "~~erer", "~~ere", "~~eres", "~~ere";
        comparative_strong_genitive: "~~eren", "~~erer", "~~eren", "~~erer";
        comparative_strong_dative: "~~erem", "~~erer", "~~erem", "~~eren";
        comparative_strong_accusative: "~~eren", "~~ere", "~~eres", "~~ere";
        comparative_weak_nominative: "~~ere", "~~ere", "~~ere", "~~eren";
        comparative_weak_genitive: "~~eren", "~~eren", "~~eren", "~~eren";
        comparative_weak_dative: "~~eren", "~~eren", "~~eren", "~~eren";
        comparative_weak_accusative: "~~eren", "~~ere", "~~ere", "~~eren";
        comparative_mixed_nominative: "~~erer", "~~ere", "~~eres", "~~eren";
        comparative_mixed_genitive: "~~eren", "~~eren", "~~eren", "~~eren";
        comparative_mixed_dative: "~~eren", "~~eren", "~~eren", "~~eren";
        comparative_mixed_accusative: "~~eren", "~~ere", "~~eres", "~~eren";
        superlative_strong_nominative: "~~ester", "~~este", "~~estes", "~~este";
        superlative_strong_genitive: "~~esten", "~~ester", "~~esten", "~~ester";
        superlative_strong_dative: "~~estem", "~~ester", "~~estem", "~~esten";
        superlative_strong_accusative: "~~esten", "~~este", "~~estes", "~~este";
        superlative_weak_nominative: "~~este", "~~este", "~~este", "~~esten";
        superlative_weak_genitive: "~~esten", "~~esten", "~~esten", "~~esten";
        superlative_weak_dative: "~~esten", "~~esten", "~~esten", "~~esten";
        superlative_weak_accusative: "~~esten", "~~este", "~~este", "~~esten";
        superlative_mixed_nominative: "~~ester", "~~este", "~~estes", "~~esten";
        superlative_mixed_genitive: "~~esten", "~~esten", "~~esten", "~~esten";
        superlative_mixed_dative: "~~esten", "~~esten", "~~esten", "~~esten";
        superlative_mixed_accusative: "~~esten", "~~este", "~~estes", "~~esten";
        predicative: "~~", "~~er", "~ am~esten";
    ),
    // Hoch: 5 words (hoch, himmelhoch, ranghoch, wolkenhoch, hüfthoch).
    declension!("hoch", 5, "~oc~";
        strong_nominative: "~o~er", "~o~e", "~o~es", "~o~e";
        strong_genitive: "~o~en", "~o~er", "~o~en", "~o~er";
        strong_dative: "~o~em", "~o~er", "~o~em", "~o~en";
        strong_accusative: "~o~en", "~o~e", "~o~es", "~o~e";
        weak_nominative: "~o~e", "~o~e", "~o~e", "~o~en";
        weak_genitive: "~o~en", "~o~en", "~o~en", "~o~en";
        weak_dative: "~o~en", "~o~en", "~o~en", "~o~en";
        weak_accusative: "~o~en", "~o~e", "~o~e", "~o~en";
        mixed_nominative: "~o~er", "~o~e", "~o~es", "~o~en";
        mixed_genitive: "~o~en", "~o~en", "~o~en", "~o~en";
        mixed_dative: "~o~en", "~o~en", "~o~en", "~o~en";
        mixed_accusative: "~o~en", "~o~e", "~o~es", "~o~en";
        comparative_strong_nominative: "~ö~erer", "~ö~ere", "~ö~eres", "~ö~ere";
        comparative_strong_genitive: "~ö~eren", "~ö~erer", "~ö~eren", "~ö~erer";
        comparative_strong_dative: "~ö~erem", "~ö~erer", "~ö~erem", "~ö~eren";
        comparative_strong_accusative: "~ö~eren", "~ö~ere", "~ö~eres", "~ö~ere";
        comparative_weak_nominative: "~ö~ere", "~ö~ere", "~ö~ere", "~ö~eren";
        comparative_weak_genitive: "~ö~eren", "~ö~eren", "~ö~eren", "~ö~eren";
        comparative_weak_dative: "~ö~eren", "~ö~eren", "~ö~eren", "~ö~eren";
        comparative_weak_accusative: "~ö~eren", "~ö~ere", "~ö~ere", "~ö~eren";
        comparative_mixed_nominative: "~ö~erer", "~ö~ere", "~ö~eres", "~ö~eren";
        comparative_mixed_genitive: "~ö~eren", "~ö~eren", "~ö~eren", "~ö~eren";
        comparative_mixed_dative: "~ö~eren", "~ö~eren", "~ö~eren", "~ö~eren";
        comparative_mixed_accusative: "~ö~eren", "~ö~ere", "~ö~eres", "~ö~eren";
        superlative_strong_nominative: "~öc~ster", "~öc~ste", "~öc~stes", "~öc~ste";
        superlative_strong_genitive: "~öc~sten", "~öc~ster", "~öc~sten", "~öc~ster";
        superlative_strong_dative: "~öc~stem", "~öc~ster", "~öc~stem", "~öc~sten";
        superlative_strong_accusative: "~öc~sten", "~öc~ste", "~öc~stes", "~öc~ste";
        superlative_weak_nominative: "~öc~ste", "~öc~ste", "~öc~ste", "~öc~sten";
        superlative_weak_genitive: "~öc~sten", "~öc~sten", "~öc~sten", "~öc~sten";
        superlative_weak_dative: "~öc~sten", "~öc~sten", "~öc~sten", "~öc~sten";
        superlative_weak_accusative: "~öc~sten", "~öc~ste", "~öc~ste", "~öc~sten";
        superlative_mixed_nominative: "~öc~ster", "~öc~ste", "~öc~stes", "~öc~sten";
        superlative_mixed_genitive: "~öc~sten", "~öc~sten", "~öc~sten", "~öc~sten";
        superlative_mixed_dative: "~öc~sten", "~öc~sten", "~öc~sten", "~öc~sten";
        superlative_mixed_accusative: "~öc~sten", "~öc~ste", "~öc~stes", "~öc~sten";
        predicative: "~oc~", "~ö~er", "am ~öc~sten";
    ),
    // Amtlich: 5 words (amerikanisch, amtlich, amharisch, amisch, amphibolisch).
    declension!("amtlich", 5, "~~";
        strong_nominative: "~~er", "~~e", "~~es", "~~e";
        strong_genitive: "~~en", "~~er", "~~en", "~~er";
        strong_dative: "~~em", "~~er", "~~em", "~~en";
        strong_accusative: "~~en", "~~e", "~~es", "~~e";
        weak_nominative: "~~e", "~~e", "~~e", "~~en";
        weak_genitive: "~~en", "~~en", "~~en", "~~en";
        weak_dative: "~~en", "~~en", "~~en", "~~en";
        weak_accusative: "~~en", "~~e", "~~e", "~~en";
        mixed_nominative: "~~er", "~~e", "~~es", "~~en";
        mixed_genitive: "~~en", "~~en", "~~en", "~~en";
        mixed_dative: "~~en", "~~en", "~~en", "~~en";
        mixed_accusative: "~~en", "~~e", "~~es", "~~en";
        comparative_strong_nominative: "~~erer", "~~ere", "~~eres", "~~ere";
        comparative_strong_genitive: "~~eren", "~~erer", "~~eren", "~~erer";
        comparative_strong_dative: "~~erem", "~~erer", "~~erem", "~~eren";
        comparative_strong_accusative: "~~eren", "~~ere", "~~eres", "~~ere";
        comparative_weak_nominative: "~~ere", "~~ere", "~~ere", "~~eren";
        comparative_weak_genitive: "~~eren", "~~eren", "~~eren", "~~eren";
        comparative_weak_dative: "~~eren", "~~eren", "~~eren", "~~eren";
        comparative_weak_accusative: "~~eren", "~~ere", "~~ere", "~~eren";
        comparative_mixed_nominative: "~~erer", "~~ere", "~~eres", "~~eren";
        comparative_mixed_genitive: "~~eren", "~~eren", "~~eren", "~~eren";
        comparative_mixed_dative: "~~eren", "~~eren", "~~eren", "~~eren";
        comparative_mixed_accusative: "~~eren", "~~ere", "~~eres", "~~eren";
        superlative_strong_nominative: "~~ster", "~~ste", "~~stes", "~~ste";
        superlative_strong_genitive: "~~sten", "~~ster", "~~sten", "~~ster";
        superlative_strong_dative: "~~stem", "~~ster", "~~stem", "~~sten";
        superlative_strong_accusative: "~~sten", "~~ste", "~~stes", "~~ste";
        superlative_weak_nominative: "~~ste", "~~ste", "~~ste", "~~sten";
        superlative_weak_genitive: "~~sten", "~~sten", "~~sten", "~~sten";
        superlative_weak_dative: "~~sten", "~~sten", "~~sten", "~~sten";
        superlative_weak_accusative: "~~sten", "~~ste", "~~ste", "~~sten";
        superlative_mixed_nominative: "~~ster", "~~ste", "~~stes", "~~sten";
        superlative_mixed_genitive: "~~sten", "~~sten", "~~sten", "~~sten";
        superlative_mixed_dative: "~~sten", "~~sten", "~~sten", "~~sten";
        superlative_mixed_accusative: "~~sten", "~~ste", "~~stes", "~~sten";
        predicative: "~~", "~~er", "~ am~sten";
    ),
    // Naß: 5 words (naß, gewiß, ungewiß, kraß, blaß).
    declension!("naß", 5, "~ß";
        strong_nominative: "~sser", "~sse", "~sses", "~sse";
        strong_genitive: "~ssen", "~sser", "~ssen", "~sser";
        strong_dative: "~ssem", "~sser", "~ssem", "~ssen";
        strong_accusative: "~ssen", "~sse", "~sses", "~sse";
        weak_nominative: "~sse", "~sse", "~sse", "~ssen";
        weak_genitive: "~ssen", "~ssen", "~ssen", "~ssen";
        weak_dative: "~ssen", "~ssen", "~ssen", "~ssen";
        weak_accusative: "~ssen", "~sse", "~sse", "~ssen";
        mixed_nominative: "~sser", "~sse", "~sses", "~ssen";
        mixed_genitive: "~ssen", "~ssen", "~ssen", "~ssen";
        mixed_dative: "~ssen", "~ssen", "~ssen", "~ssen";
        mixed_accusative: "~ssen", "~sse", "~sses", "~ssen";
        comparative_strong_nominative: "~sserer", "~ssere", "~sseres", "~ssere";
        comparative_strong_genitive: "~sseren", "~sserer", "~sseren", "~sserer";
        comparative_strong_dative: "~sserem", "~sserer", "~sserem", "~sseren";
        comparative_strong_accusative: "~sseren", "~ssere", "~sseres", "~ssere";
        comparative_weak_nominative: "~ssere", "~ssere", "~ssere", "~sseren";
        comparative_weak_genitive: "~sseren", "~sseren", "~sseren", "~sseren";
        comparative_weak_dative: "~sseren", "~sseren", "~sseren", "~sseren";
        comparative_weak_accusative: "~sseren", "~ssere", "~ssere", "~sseren";
        comparative_mixed_nominative: "~sserer", "~ssere", "~sseres", "~sseren";
        comparative_mixed_genitive: "~sseren", "~sseren", "~sseren", "~sseren";
        comparative_mixed_dative: "~sseren", "~sseren", "~sseren", "~sseren";
        comparative_mixed_accusative: "~sseren", "~ssere", "~sseres", "~sseren";
        superlative_strong_nominative: "~ssester", "~sseste", "~ssestes", "~sseste";
        superlative_strong_genitive: "~ssesten", "~ssester", "~ssesten", "~ssester";
        superlative_strong_dative: "~ssestem", "~ssester", "~ssestem", "~ssesten";
        superlative_strong_accusative: "~ssesten", "~sseste", "~ssestes", "~sseste";
        superlative_weak_nominative: "~sseste", "~sseste", "~sseste", "~ssesten";
        superlative_weak_genitive: "~ssesten", "~ssesten", "~ssesten", "~ssesten";
        superlative_weak_dative: "~ssesten", "~ssesten", "~ssesten", "~ssesten";
        superlative_weak_accusative: "~ssesten", "~sseste", "~sseste", "~ssesten";
        superlative_mixed_nominative: "~ssester", "~sseste", "~ssestes", "~ssesten";
        superlative_mixed_genitive: "~ssesten", "~ssesten", "~ssesten", "~ssesten";
        superlative_mixed_dative: "~ssesten", "~ssesten", "~ssesten", "~ssesten";
        superlative_mixed_accusative: "~ssesten", "~sseste", "~ssestes", "~ssesten";
        predicative: "~ß", "~sser", "am ~ssesten";
    ),
    // Abgeschlossen: 4 words (angemessen, abgeschlossen, abgedroschen, aufgedunsen).
    declension!("abgeschlossen", 4, "~~e~";
        strong_nominative: "~~e~er", "~~e~e", "~~e~es", "~~e~e";
        strong_genitive: "~~e~en", "~~e~er", "~~e~en", "~~e~er";
        strong_dative: "~~e~em", "~~e~er", "~~e~em", "~~e~en";
        strong_accusative: "~~e~en", "~~e~e", "~~e~es", "~~e~e";
        weak_nominative: "~~e~e", "~~e~e", "~~e~e", "~~e~en";
        weak_genitive: "~~e~en", "~~e~en", "~~e~en", "~~e~en";
        weak_dative: "~~e~en", "~~e~en", "~~e~en", "~~e~en";
        weak_accusative: "~~e~en", "~~e~e", "~~e~e", "~~e~en";
        mixed_nominative: "~~e~er", "~~e~e", "~~e~es", "~~e~en";
        mixed_genitive: "~~e~en", "~~e~en", "~~e~en", "~~e~en";
        mixed_dative: "~~e~en", "~~e~en", "~~e~en", "~~e~en";
        mixed_accusative: "~~e~en", "~~e~e", "~~e~es", "~~e~en";
        comparative_strong_nominative: "~~e~erer|~~~erer", "~~e~ere|~~~ere", "~~e~eres|~~~eres", "~~e~ere|~~~ere";
        comparative_strong_genitive: "~~e~eren|~~~eren", "~~e~erer|~~~erer", "~~e~eren|~~~eren", "~~e~erer|~~~erer";
        comparative_strong_dative: "~~e~erem|~~~erem", "~~e~erer|~~~erer", "~~e~erem|~~~erem", "~~e~eren|~~~eren";
        comparative_strong_accusative: "~~e~eren|~~~eren", "~~e~ere|~~~ere", "~~e~eres|~~~eres", "~~e~ere|~~~ere";
        comparative_weak_nominative: "~~e~ere|~~~ere", "~~e~ere|~~~ere", "~~e~ere|~~~ere", "~~e~eren|~~~eren";
        comparative_weak_genitive: "~~e~eren|~~~eren", "~~e~eren|~~~eren", "~~e~eren|~~~eren", "~~e~eren|~~~eren";
        comparative_weak_dative: "~~e~eren|~~~eren", "~~e~eren|~~~eren", "~~e~eren|~~~eren", "~~e~eren|~~~eren";
        comparative_weak_accusative: "~~e~eren|~~~eren", "~~e~ere|~~~ere", "~~e~ere|~~~ere", "~~e~eren|~~~eren";
        comparative_mixed_nominative: "~~e~erer|~~~erer", "~~e~ere|~~~ere", "~~e~eres|~~~eres", "~~e~eren|~~~eren";
        comparative_mixed_genitive: "~~e~eren|~~~eren", "~~e~eren|~~~eren", "~~e~eren|~~~eren", "~~e~eren|~~~eren";
        comparative_mixed_dative: "~~e~eren|~~~eren", "~~e~eren|~~~eren", "~~e~eren|~~~eren", "~~e~eren|~~~eren";
        comparative_mixed_accusative: "~~e~eren|~~~eren", "~~e~ere|~~~ere", "~~e~eres|~~~eres", "~~e~eren|~~~eren";
        superlative_strong_nominative: "~~e~ster", "~~e~ste", "~~e~stes", "~~e~ste";
        superlative_strong_genitive: "~~e~sten", "~~e~ster", "~~e~sten", "~~e~ster";
        superlative_strong_dative: "~~e~stem", "~~e~ster", "~~e~stem", "~~e~sten";
        superlative_strong_accusative: "~~e~sten", "~~e~ste", "~~e~stes", "~~e~ste";
        superlative_weak_nominative: "~~e~ste", "~~e~ste", "~~e~ste", "~~e~sten";
        superlative_weak_genitive: "~~e~sten", "~~e~sten", "~~e~sten", "~~e~sten";
        superlative_weak_dative: "~~e~sten", "~~e~sten", "~~e~sten", "~~e~sten";
        superlative_weak_accusative: "~~e~sten", "~~e~ste", "~~e~ste", "~~e~sten";
        superlative_mixed_nominative: "~~e~ster", "~~e~ste", "~~e~stes", "~~e~sten";
        superlative_mixed_genitive: "~~e~sten", "~~e~sten", "~~e~sten", "~~e~sten";
        superlative_mixed_dative: "~~e~sten", "~~e~sten", "~~e~sten", "~~e~sten";
        superlative_mixed_accusative: "~~e~sten", "~~e~ste", "~~e~stes", "~~e~sten";
        predicative: "~~e~", "~~e~er|~~~er", "~m a~e~sten";
    ),
    // Lila: 3 words (rosa, lila, schweinchenrosa).
    declension!("lila", 3, "~";
        strong_nominative: "~ner", "~ne", "~nes", "~ne";
        strong_genitive: "~nen", "~ner", "~nen", "~ner";
        strong_dative: "~nem", "~ner", "~nem", "~nen";
        strong_accusative: "~nen", "~ne", "~nes", "~ne";
        weak_nominative: "~ne", "~ne", "~ne", "~nen";
        weak_genitive: "~nen", "~nen", "~nen", "~nen";
        weak_dative: "~nen", "~nen", "~nen", "~nen";
        weak_accusative: "~nen", "~ne", "~ne", "~nen";
        mixed_nominative: "~ner", "~ne", "~nes", "~nen";
        mixed_genitive: "~nen", "~nen", "~nen", "~nen";
        mixed_dative: "~nen", "~nen", "~nen", "~nen";
        mixed_accusative: "~nen", "~ne", "~nes", "~nen";
        comparative_strong_nominative: "~nerer", "~nere", "~neres", "~nere";
        comparative_strong_genitive: "~neren", "~nerer", "~neren", "~nerer";
        comparative_strong_dative: "~nerem", "~nerer", "~nerem", "~neren";
        comparative_strong_accusative: "~neren", "~nere", "~neres", "~nere";
        comparative_weak_nominative: "~nere", "~nere", "~nere", "~neren";
        comparative_weak_genitive: "~neren", "~neren", "~neren", "~neren";
        comparative_weak_dative: "~neren", "~neren", "~neren", "~neren";
        comparative_weak_accusative: "~neren", "~nere", "~nere", "~neren";
        comparative_mixed_nominative: "~nerer", "~nere", "~neres", "~neren";
        comparative_mixed_genitive: "~neren", "~neren", "~neren", "~neren";
        comparative_mixed_dative: "~neren", "~neren", "~neren", "~neren";
        comparative_mixed_accusative: "~neren", "~nere", "~neres", "~neren";
        superlative_strong_nominative: "~nster", "~nste", "~nstes", "~nste";
        superlative_strong_genitive: "~nsten", "~nster", "~nsten", "~nster";
        superlative_strong_dative: "~nstem", "~nster", "~nstem", "~nsten";
        superlative_strong_accusative: "~nsten", "~nste", "~nstes", "~nste";
        superlative_weak_nominative: "~nste", "~nste", "~nste", "~nsten";
        superlative_weak_genitive: "~nsten", "~nsten", "~nsten", "~nsten";
        superlative_weak_dative: "~nsten", "~nsten", "~nsten", "~nsten";
        superlative_weak_accusative: "~nsten", "~nste", "~nste", "~nsten";
        superlative_mixed_nominative: "~nster", "~nste", "~nstes", "~nsten";
        superlative_mixed_genitive: "~nsten", "~nsten", "~nsten", "~nsten";
        superlative_mixed_dative: "~nsten", "~nsten", "~nsten", "~nsten";
        superlative_mixed_accusative: "~nsten", "~nste", "~nstes", "~nsten";
        predicative: "~", "~ner", "am ~nsten";
    ),
    // Aper: 3 words (aper, abhörsicher, auslaufsicher).
    declension!("aper", 3, "~~~";
        strong_nominative: "~~~er", "~~~e", "~~~es", "~~~e";
        strong_genitive: "~~~en", "~~~er", "~~~en", "~~~er";
        strong_dative: "~~~em", "~~~er", "~~~em", "~~~en";
        strong_accusative: "~~~en", "~~~e", "~~~es", "~~~e";
        weak_nominative: "~~~e", "~~~e", "~~~e", "~~~en";
        weak_genitive: "~~~en", "~~~en", "~~~en", "~~~en";
        weak_dative: "~~~en", "~~~en", "~~~en", "~~~en";
        weak_accusative: "~~~en", "~~~e", "~~~e", "~~~en";
        mixed_nominative: "~~~er", "~~~e", "~~~es", "~~~en";
        mixed_genitive: "~~~en", "~~~en", "~~~en", "~~~en";
        mixed_dative: "~~~en", "~~~en", "~~~en", "~~~en";
        mixed_accusative: "~~~en", "~~~e", "~~~es", "~~~en";
        comparative_strong_nominative: "~~~erer|~~r~er", "~~~ere|~~r~e", "~~~eres|~~r~es", "~~~ere|~~r~e";
        comparative_strong_genitive: "~~~eren|~~r~en", "~~~erer|~~r~er", "~~~eren|~~r~en", "~~~erer|~~r~er";
        comparative_strong_dative: "~~~erem|~~r~em", "~~~erer|~~r~er", "~~~erem|~~r~em", "~~~eren|~~r~en";
        comparative_strong_accusative: "~~~eren|~~r~en", "~~~ere|~~r~e", "~~~eres|~~r~es", "~~~ere|~~r~e";
        comparative_weak_nominative: "~~~ere|~~r~e", "~~~ere|~~r~e", "~~~ere|~~r~e", "~~~eren|~~r~en";
        comparative_weak_genitive: "~~~eren|~~r~en", "~~~eren|~~r~en", "~~~eren|~~r~en", "~~~eren|~~r~en";
        comparative_weak_dative: "~~~eren|~~r~en", "~~~eren|~~r~en", "~~~eren|~~r~en", "~~~eren|~~r~en";
        comparative_weak_accusative: "~~~eren|~~r~en", "~~~ere|~~r~e", "~~~ere|~~r~e", "~~~eren|~~r~en";
        comparative_mixed_nominative: "~~~erer|~~r~er", "~~~ere|~~r~e", "~~~eres|~~r~es", "~~~eren|~~r~en";
        comparative_mixed_genitive: "~~~eren|~~r~en", "~~~eren|~~r~en", "~~~eren|~~r~en", "~~~eren|~~r~en";
        comparative_mixed_dative: "~~~eren|~~r~en", "~~~eren|~~r~en", "~~~eren|~~r~en", "~~~eren|~~r~en";
        comparative_mixed_accusative: "~~~eren|~~r~en", "~~~ere|~~r~e", "~~~eres|~~r~es", "~~~eren|~~r~en";
        superlative_strong_nominative: "~~~ster", "~~~ste", "~~~stes", "~~~ste";
        superlative_strong_genitive: "~~~sten", "~~~ster", "~~~sten", "~~~ster";
        superlative_strong_dative: "~~~stem", "~~~ster", "~~~stem", "~~~sten";
        superlative_strong_accusative: "~~~sten", "~~~ste", "~~~stes", "~~~ste";
        superlative_weak_nominative: "~~~ste", "~~~ste", "~~~ste", "~~~sten";
        superlative_weak_genitive: "~~~sten", "~~~sten", "~~~sten", "~~~sten";
        superlative_weak_dative: "~~~sten", "~~~sten", "~~~sten", "~~~sten";
        superlative_weak_accusative: "~~~sten", "~~~ste", "~~~ste", "~~~sten";
        superlative_mixed_nominative: "~~~ster", "~~~ste", "~~~stes", "~~~sten";
        superlative_mixed_genitive: "~~~sten", "~~~sten", "~~~sten", "~~~sten";
        superlative_mixed_dative: "~~~sten", "~~~sten", "~~~sten", "~~~sten";
        superlative_mixed_accusative: "~~~sten", "~~~ste", "~~~stes", "~~~sten";
        predicative: "~~~", "~~~er|~~r~", "~m a~~sten";
    ),
    // Affabel: 3 words (akzeptabel, abominabel, affabel).
    declension!("affabel", 3, "~~e~";
        strong_nominative: "~~~er", "~~~e", "~~~es", "~~~e";
        strong_genitive: "~~~en", "~~~er", "~~~en", "~~~er";
        strong_dative: "~~~em", "~~~er", "~~~em", "~~~en";
        strong_accusative: "~~~en", "~~~e", "~~~es", "~~~e";
        weak_nominative: "~~~e", "~~~e", "~~~e", "~~~en";
        weak_genitive: "~~~en", "~~~en", "~~~en", "~~~en";
        weak_dative: "~~~en", "~~~en", "~~~en", "~~~en";
        weak_accusative: "~~~en", "~~~e", "~~~e", "~~~en";
        mixed_nominative: "~~~er", "~~~e", "~~~es", "~~~en";
        mixed_genitive: "~~~en", "~~~en", "~~~en", "~~~en";
        mixed_dative: "~~~en", "~~~en", "~~~en", "~~~en";
        mixed_accusative: "~~~en", "~~~e", "~~~es", "~~~en";
        comparative_strong_nominative: "~~~erer", "~~~ere", "~~~eres", "~~~ere";
        comparative_strong_genitive: "~~~eren", "~~~erer", "~~~eren", "~~~erer";
        comparative_strong_dative: "~~~erem", "~~~erer", "~~~erem", "~~~eren";
        comparative_strong_accusative: "~~~eren", "~~~ere", "~~~eres", "~~~ere";
        comparative_weak_nominative: "~~~ere", "~~~ere", "~~~ere", "~~~eren";
        comparative_weak_genitive: "~~~eren", "~~~eren", "~~~eren", "~~~eren";
        comparative_weak_dative: "~~~eren", "~~~eren", "~~~eren", "~~~eren";
        comparative_weak_accusative: "~~~eren", "~~~ere", "~~~ere", "~~~eren";
        comparative_mixed_nominative: "~~~erer", "~~~ere", "~~~eres", "~~~eren";
        comparative_mixed_genitive: "~~~eren", "~~~eren", "~~~eren", "~~~eren";
        comparative_mixed_dative: "~~~eren", "~~~eren", "~~~eren", "~~~eren";
        comparative_mixed_accusative: "~~~eren", "~~~ere", "~~~eres", "~~~eren";
        superlative_strong_nominative: "~~e~ster", "~~e~ste", "~~e~stes", "~~e~ste";
        superlative_strong_genitive: "~~e~sten", "~~e~ster", "~~e~sten", "~~e~ster";
        superlative_strong_dative: "~~e~stem", "~~e~ster", "~~e~stem", "~~e~sten";
        superlative_strong_accusative: "~~e~sten", "~~e~ste", "~~e~stes", "~~e~ste";
        superlative_weak_nominative: "~~e~ste", "~~e~ste", "~~e~ste", "~~e~sten";
        superlative_weak_genitive: "~~e~sten", "~~e~sten", "~~e~sten", "~~e~sten";
        superlative_weak_dative: "~~e~sten", "~~e~sten", "~~e~sten", "~~e~sten";
        superlative_weak_accusative: "~~e~sten", "~~e~ste", "~~e~ste", "~~e~sten";
        superlative_mixed_nominative: "~~e~ster", "~~e~ste", "~~e~stes", "~~e~sten";
        superlative_mixed_genitive: "~~e~sten", "~~e~sten", "~~e~sten", "~~e~sten";
        superlative_mixed_dative: "~~e~sten", "~~e~sten", "~~e~sten", "~~e~sten";
        superlative_mixed_accusative: "~~e~sten", "~~e~ste", "~~e~stes", "~~e~sten";
        predicative: "~~e~", "~~~er", "~m a~e~sten";
    ),
    // Aschearm: 3 words (alkoholarm, anregungsarm, aschearm).
    declension!("aschearm", 3, "~~a~";
        strong_nominative: "~~a~er", "~~a~e", "~~a~es", "~~a~e";
        strong_genitive: "~~a~en", "~~a~er", "~~a~en", "~~a~er";
        strong_dative: "~~a~em", "~~a~er", "~~a~em", "~~a~en";
        strong_accusative: "~~a~en", "~~a~e", "~~a~es", "~~a~e";
        weak_nominative: "~~a~e", "~~a~e", "~~a~e", "~~a~en";
        weak_genitive: "~~a~en", "~~a~en", "~~a~en", "~~a~en";
        weak_dative: "~~a~en", "~~a~en", "~~a~en", "~~a~en";
        weak_accusative: "~~a~en", "~~a~e", "~~a~e", "~~a~en";
        mixed_nominative: "~~a~er", "~~a~e", "~~a~es", "~~a~en";
        mixed_genitive: "~~a~en", "~~a~en", "~~a~en", "~~a~en";
        mixed_dative: "~~a~en", "~~a~en", "~~a~en", "~~a~en";
        mixed_accusative: "~~a~en", "~~a~e", "~~a~es", "~~a~en";
        comparative_strong_nominative: "~~ä~erer", "~~ä~ere", "~~ä~eres", "~~ä~ere";
        comparative_strong_genitive: "~~ä~eren", "~~ä~erer", "~~ä~eren", "~~ä~erer";
        comparative_strong_dative: "~~ä~erem", "~~ä~erer", "~~ä~erem", "~~ä~eren";
        comparative_strong_accusative: "~~ä~eren", "~~ä~ere", "~~ä~eres", "~~ä~ere";
        comparative_weak_nominative: "~~ä~ere", "~~ä~ere", "~~ä~ere", "~~ä~eren";
        comparative_weak_genitive: "~~ä~eren", "~~ä~eren", "~~ä~eren", "~~ä~eren";
        comparative_weak_dative: "~~ä~eren", "~~ä~eren", "~~ä~eren", "~~ä~eren";
        comparative_weak_accusative: "~~ä~eren", "~~ä~ere", "~~ä~ere", "~~ä~eren";
        comparative_mixed_nominative: "~~ä~erer", "~~ä~ere", "~~ä~eres", "~~ä~eren";
        comparative_mixed_genitive: "~~ä~eren", "~~ä~eren", "~~ä~eren", "~~ä~eren";
        comparative_mixed_dative: "~~ä~eren", "~~ä~eren", "~~ä~eren", "~~ä~eren";
        comparative_mixed_accusative: "~~ä~eren", "~~ä~ere", "~~ä~eres", "~~ä~eren";
        superlative_strong_nominative: "~~ä~ster", "~~ä~ste", "~~ä~stes", "~~ä~ste";
        superlative_strong_genitive: "~~ä~sten", "~~ä~ster", "~~ä~sten", "~~ä~ster";
        superlative_strong_dative: "~~ä~stem", "~~ä~ster", "~~ä~stem", "~~ä~sten";
        superlative_strong_accusative: "~~ä~sten", "~~ä~ste", "~~ä~stes", "~~ä~ste";
        superlative_weak_nominative: "~~ä~ste", "~~ä~ste", "~~ä~ste", "~~ä~sten";
        superlative_weak_genitive: "~~ä~sten", "~~ä~sten", "~~ä~sten", "~~ä~sten";
        superlative_weak_dative: "~~ä~sten", "~~ä~sten", "~~ä~sten", "~~ä~sten";
        superlative_weak_accusative: "~~ä~sten", "~~ä~ste", "~~ä~ste", "~~ä~sten";
        superlative_mixed_nominative: "~~ä~ster", "~~ä~ste", "~~ä~stes", "~~ä~sten";
        superlative_mixed_genitive: "~~ä~sten", "~~ä~sten", "~~ä~sten", "~~ä~sten";
        superlative_mixed_dative: "~~ä~sten", "~~ä~sten", "~~ä~sten", "~~ä~sten";
        superlative_mixed_accusative: "~~ä~sten", "~~ä~ste", "~~ä~stes", "~~ä~sten";
        predicative: "~~a~", "~~ä~er", "~m a~ä~sten";
    ),
];
