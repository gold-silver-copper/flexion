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
    /// Like `mano`: 16,634 words.
    Mano,
    /// Like `mercatanzia`: 15,701 words.
    Mercatanzia,
    /// Like `lume`: 13,482 words.
    Lume,
    /// Like `autorità`: 7,297 words.
    Autorità,
    /// Like `testimonio`: 3,698 words.
    Testimonio,
    /// Like `programma`: 2,282 words.
    Programma,
    /// Like `mosca`: 1,323 words.
    Mosca,
    /// Like `banco`: 746 words.
    Banco,
    /// Like `traccia`: 290 words.
    Traccia,
    /// Like `capolista`: 86 words.
    Capolista,
    /// Like `gerarca`: 35 words.
    Gerarca,
    /// Like `mezzaluna`: 34 words.
    Mezzaluna,
    /// Like `spoiler`: 32 words.
    Spoiler,
    /// Like `braccio`: 22 words.
    Braccio,
    /// Like `bassofondo`: 14 words.
    Bassofondo,
    /// Like `uomo`: 13 words.
    Uomo,
    /// Like `acquaforte`: 10 words.
    Acquaforte,
    /// Like `superficie`: 8 words.
    Superficie,
    /// Like `frogia`: 8 words.
    Frogia,
    /// Like `mezzasega`: 8 words.
    Mezzasega,
    /// Like `capodanno`: 7 words.
    Capodanno,
    /// Like `bimbaminkia`: 6 words.
    Bimbaminkia,
    /// Like `capoccia`: 4 words.
    Capoccia,
    /// Like `heroon`: 4 words.
    Heroon,
    /// Like `capsa`: 3 words.
    Capsa,
    /// Like `gattomammone`: 3 words.
    Gattomammone,
    /// Like `capofficina`: 3 words.
    Capofficina,
    /// Like `doppiosenso`: 3 words.
    Doppiosenso,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 28] = [
        Declension::Mano,
        Declension::Mercatanzia,
        Declension::Lume,
        Declension::Autorità,
        Declension::Testimonio,
        Declension::Programma,
        Declension::Mosca,
        Declension::Banco,
        Declension::Traccia,
        Declension::Capolista,
        Declension::Gerarca,
        Declension::Mezzaluna,
        Declension::Spoiler,
        Declension::Braccio,
        Declension::Bassofondo,
        Declension::Uomo,
        Declension::Acquaforte,
        Declension::Superficie,
        Declension::Frogia,
        Declension::Mezzasega,
        Declension::Capodanno,
        Declension::Bimbaminkia,
        Declension::Capoccia,
        Declension::Heroon,
        Declension::Capsa,
        Declension::Gattomammone,
        Declension::Capofficina,
        Declension::Doppiosenso,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 28] = [
    // Mano: 16,634 words (zero, sabato, peso, centavo, libero, euro).
    declension!("mano", 16634, "~o";
        noun: "~o", "~i";
    ),
    // Mercatanzia: 15,701 words (aquila, acicula, aria, arietta, guerra, acacia).
    declension!("mercatanzia", 15701, "~a";
        noun: "~a", "~e";
    ),
    // Lume: 13,482 words (abate, lente, decade, seme, are, andante).
    declension!("lume", 13482, "~e";
        noun: "~e", "~i";
    ),
    // Autorità: 7,297 words (fa, abaca, abracadabra, quiz, nu, robot).
    declension!("autorità", 7297, "~";
        noun: "~", "~";
    ),
    // Testimonio: 3,698 words (radio, bancario, curio, scenario, pugio, sorcio).
    declension!("testimonio", 3698, "~o";
        noun: "~o", "~";
    ),
    // Programma: 2,282 words (abada, comma, plasma, miasma, papa, poeta).
    declension!("programma", 2282, "~a";
        noun: "~a", "~i|~e";
    ),
    // Mosca: 1,323 words (replica, musica, pesca, riga, vacca, fuga).
    declension!("mosca", 1323, "~a";
        noun: "~a", "~he";
    ),
    // Banco: 746 words (tago, guanaco, bongo, abaco, loco, mango).
    declension!("banco", 746, "~o";
        noun: "~o", "~hi";
    ),
    // Traccia: 290 words (faccia, pioggia, pronuncia, fascia, lancia, goccia).
    declension!("traccia", 290, "~ia";
        noun: "~ia", "~e";
    ),
    // Capolista: 86 words (capotavola, capobranco, caporeparto, capolinea, bagnoschiuma, capogruppo).
    declension!("capolista", 86, "~o~";
        noun: "~o~", "~i~";
    ),
    // Gerarca: 35 words (transfuga, auriga, collega, monarca, tetrarca, gerarca).
    declension!("gerarca", 35, "~a";
        noun: "~a", "~hi";
    ),
    // Mezzaluna: 34 words (terracotta, belladonna, capasanta, mezzaluna, primadonna, terraferma).
    declension!("mezzaluna", 34, "~a~a";
        noun: "~a~a", "~e~e";
    ),
    // Spoiler: 32 words (boy, croissant, fan, magazine, yard, manager).
    declension!("spoiler", 32, "~";
        noun: "~", "~s";
    ),
    // Braccio: 22 words (uovo, labbro, braccio, centinaio, migliaio, ginocchio).
    declension!("braccio", 22, "~o";
        noun: "~o", "~a";
    ),
    // Bassofondo: 14 words (altoforno, bassofondo, camposanto, caposaldo, caposcarico, collotorto).
    declension!("bassofondo", 14, "~o~o";
        noun: "~o~o", "~i~i";
    ),
    // Uomo: 13 words (homo, uomo, galantuomo, gentiluomo, superuomo, nobiluomo).
    declension!("uomo", 13, "~o";
        noun: "~o", "~ini";
    ),
    // Acquaforte: 10 words (mezzanotte, malafede, cassaforte, mezzanave, acquaforte, piazzaforte).
    declension!("acquaforte", 10, "~a~e";
        noun: "~a~e", "~e~i";
    ),
    // Superficie: 8 words (moglie, superficie, effigie, aviosuperficie, ipersuperficie, isosuperficie).
    declension!("superficie", 8, "~e";
        noun: "~e", "~";
    ),
    // Frogia: 8 words (valigia, ciliegia, frogia, guarentigia, bragia, cervogia).
    declension!("frogia", 8, "~ia";
        noun: "~ia", "~ie|~e";
    ),
    // Mezzasega: 8 words (cappalunga, cassapanca, cartasuga, gambasecca, mezzasega, mezzatacca).
    declension!("mezzasega", 8, "~a~a";
        noun: "~a~a", "~e~he";
    ),
    // Capodanno: 7 words (pomodoro, caciocavallo, capodanno, altopiano, capomastro, capogatto).
    declension!("capodanno", 7, "~o~o";
        noun: "~o~o", "~o~i|~i~i";
    ),
    // Bimbaminkia: 6 words (cartacarbone, scuolaguida, spinacristi, zappacavallo, bimbaminkia, mostramercato).
    declension!("bimbaminkia", 6, "~a~";
        noun: "~a~", "~e~";
    ),
    // Capoccia: 4 words (maracaia, capoccia, correggia, porteria).
    declension!("capoccia", 4, "~a";
        noun: "~a", "~";
    ),
    // Heroon: 4 words (rhyton, heroon, megaron, abaton).
    declension!("heroon", 4, "~on";
        noun: "~on", "~a";
    ),
    // Capsa: 3 words (tabula, fabula, capsa).
    declension!("capsa", 3, "~";
        noun: "~", "~e";
    ),
    // Gattomammone: 3 words (gattomammone, capocarceriere, capotimoniere).
    declension!("gattomammone", 3, "~o~e";
        noun: "~o~e", "~i~i";
    ),
    // Capofficina: 3 words (capofficina, decanapulazione, vetrofusione).
    declension!("capofficina", 3, "~~";
        noun: "~~", "~i~";
    ),
    // Doppiosenso: 3 words (doppiovetro, occhiogrosso, doppiosenso).
    declension!("doppiosenso", 3, "~o~o";
        noun: "~o~o", "~~i";
    ),
];
