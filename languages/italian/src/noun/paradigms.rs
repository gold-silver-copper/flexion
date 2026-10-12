//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's feature says):
/// `noun` (sg, pl)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal;
     noun: $c0:tt, $c1:tt;
    ) => {
        Paradigm {
            name: ::std::borrow::Cow::Borrowed($name),
            words: $words,
            lemma: ::std::borrow::Cow::Borrowed($lemma),
            forms: ::std::borrow::Cow::Borrowed(&[flexion::form!($c0), flexion::form!($c1)]),
        }
    };
}

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `mano`: 8,849 words.
    Mano,
    /// Like `catanzia`: 8,076 words.
    Catanzia,
    /// Like `lume`: 7,480 words.
    Lume,
    /// Like `rità`: 5,381 words.
    Rità,
    /// Like `timonio`: 2,056 words.
    Timonio,
    /// Like `evangelista`: 1,346 words.
    Evangelista,
    /// Like `esca`: 688 words.
    Esca,
    /// Like `cco`: 397 words.
    Cco,
    /// Like `fascia`: 141 words.
    Fascia,
    /// Like `capolista`: 88 words.
    Capolista,
    /// Like `mezzaluna`: 32 words.
    Mezzaluna,
    /// Like `boy`: 27 words.
    Boy,
    /// Like `garca`: 27 words.
    Garca,
    /// Like `ccio`: 16 words.
    Ccio,
    /// Like `bassofondo`: 12 words.
    Bassofondo,
    /// Like `uomo`: 11 words.
    Uomo,
    /// Like `zzaforte`: 10 words.
    Zzaforte,
    /// Like `mezzasega`: 7 words.
    Mezzasega,
    /// Like `olaguida`: 6 words.
    Olaguida,
    /// Like `gattomammone`: 5 words.
    Gattomammone,
    /// Like `ficie`: 4 words.
    Ficie,
    /// Like `liegia`: 4 words.
    Liegia,
    /// Like `otorto`: 4 words.
    Otorto,
    /// Like `capofficina`: 3 words.
    Capofficina,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 24] = [
        Declension::Mano,
        Declension::Catanzia,
        Declension::Lume,
        Declension::Rità,
        Declension::Timonio,
        Declension::Evangelista,
        Declension::Esca,
        Declension::Cco,
        Declension::Fascia,
        Declension::Capolista,
        Declension::Mezzaluna,
        Declension::Boy,
        Declension::Garca,
        Declension::Ccio,
        Declension::Bassofondo,
        Declension::Uomo,
        Declension::Zzaforte,
        Declension::Mezzasega,
        Declension::Olaguida,
        Declension::Gattomammone,
        Declension::Ficie,
        Declension::Liegia,
        Declension::Otorto,
        Declension::Capofficina,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 24] = [
    // Mano: 8,849 words (zero, peso, euro, moro, dodo, limo).
    declension!("mano", 8849, "~o";
        noun: "~o", "~i";
    ),
    // Catanzia: 8,076 words (aquila, aria, henna, tuba, zebra, lama).
    declension!("catanzia", 8076, "~a";
        noun: "~a", "~e";
    ),
    // Lume: 7,480 words (seme, are, cane, male, face, sale).
    declension!("lume", 7480, "~e";
        noun: "~e", "~i";
    ),
    // Rità: 5,381 words (fa, quiz, nu, van, die, star).
    declension!("rità", 5381, "~";
        noun: "~", "~";
    ),
    // Timonio: 2,056 words (radio, curio, olio, saio, agio, studio).
    declension!("timonio", 2056, "~o";
        noun: "~o", "~";
    ),
    // Evangelista: 1,346 words (miasma, papa, poeta, arma, tema, profeta).
    declension!("evangelista", 1346, "~a";
        noun: "~a", "~i|~e";
    ),
    // Esca: 688 words (riga, fuga, nuca, oca, giga, toga).
    declension!("esca", 688, "~a";
        noun: "~a", "~he";
    ),
    // Cco: 397 words (tago, loco, lago, ago, ubriaco, arco).
    declension!("cco", 397, "~o";
        noun: "~o", "~hi";
    ),
    // Fascia: 141 words (fascia, arancia, guancia, rovescia, striscia, angoscia).
    declension!("fascia", 141, "~ia";
        noun: "~ia", "~e";
    ),
    // Capolista: 88 words (capotavola, capobranco, caporeparto, capolinea, capogruppo, capotribù).
    declension!("capolista", 88, "~o~";
        noun: "~o~", "~i~";
    ),
    // Mezzaluna: 32 words (terracotta, mezzaluna, primadonna, terraferma, malalingua, acquasanta).
    declension!("mezzaluna", 32, "~a~a";
        noun: "~a~a", "~e~e";
    ),
    // Boy: 27 words (boy, fan, yard, blanco, yuppie, indio).
    declension!("boy", 27, "~";
        noun: "~", "~s";
    ),
    // Garca: 27 words (transfuga, auriga, gerarca, duca, patriarca, stratega).
    declension!("garca", 27, "~a";
        noun: "~a", "~hi";
    ),
    // Ccio: 16 words (uovo, migliaio, ginocchio, paio, ovo, riso).
    declension!("ccio", 16, "~o";
        noun: "~o", "~a";
    ),
    // Bassofondo: 12 words (altoforno, bassofondo, caposaldo, caposcarico, capofreddo, mezzopunto).
    declension!("bassofondo", 12, "~o~o";
        noun: "~o~o", "~i~i";
    ),
    // Uomo: 11 words (homo, uomo, omo, huomo, antuomo, tiluomo).
    declension!("uomo", 11, "~o";
        noun: "~o", "~ini";
    ),
    // Zzaforte: 10 words (mezzanotte, malafede, mezzanave, acquaforte, casamobile, casaforte).
    declension!("zzaforte", 10, "~a~e";
        noun: "~a~e", "~e~i";
    ),
    // Mezzasega: 7 words (cartasuga, mezzasega, mezzatacca, palunga, sapanca, basecca).
    declension!("mezzasega", 7, "~a~a";
        noun: "~a~a", "~e~he";
    ),
    // Olaguida: 6 words (cartacarbone, azza-copertina, olaguida, ppacavallo, mbaminkia, tramercato).
    declension!("olaguida", 6, "~a~";
        noun: "~a~", "~e~";
    ),
    // Gattomammone: 5 words (gattomammone, capocarceriere, capotimoniere, mmo-equivalente, mmo-mole).
    declension!("gattomammone", 5, "~o~e";
        noun: "~o~e", "~i~i";
    ),
    // Ficie: 4 words (glie, ficie, figie, superficie).
    declension!("ficie", 4, "~e";
        noun: "~e", "~";
    ),
    // Liegia: 4 words (liegia, entigia, vogia, lovaligia).
    declension!("liegia", 4, "~ia";
        noun: "~ia", "~ie|~e";
    ),
    // Otorto: 4 words (osanto, otorto, oviro, obuono).
    declension!("otorto", 4, "o~o";
        noun: "o~o", "i~i";
    ),
    // Capofficina: 3 words (capofficina, vetrofusione, napulazione).
    declension!("capofficina", 3, "~~";
        noun: "~~", "~i~";
    ),
];
