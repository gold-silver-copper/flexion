//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's feature says):
/// `masculine` (sg, pl)
/// `feminine` (sg, pl)
/// `superlative`
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal;
     masculine: $c0:tt, $c1:tt;
     feminine: $c2:tt, $c3:tt;
     superlative: $c4:tt;
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
            ]),
        }
    };
}

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `gliardo`: 7,307 words.
    Gliardo,
    /// Like `ggiore`: 5,218 words.
    Ggiore,
    /// Like `ntastico`: 2,994 words.
    Ntastico,
    /// Like `zzaio`: 689 words.
    Zzaio,
    /// Like `ari`: 583 words.
    Ari,
    /// Like `berista`: 423 words.
    Berista,
    /// Like `vago`: 367 words.
    Vago,
    /// Like `morzatore`: 328 words.
    Morzatore,
    /// Like `rsipelle`: 130 words.
    Rsipelle,
    /// Like `vescio`: 106 words.
    Vescio,
    /// Like `rbone`: 28 words.
    Rbone,
    /// Like `ampio`: 12 words.
    Ampio,
    /// Like `dona`: 12 words.
    Dona,
    /// Like `acre`: 9 words.
    Acre,
    /// Like `duco`: 4 words.
    Duco,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 15] = [
        Declension::Gliardo,
        Declension::Ggiore,
        Declension::Ntastico,
        Declension::Zzaio,
        Declension::Ari,
        Declension::Berista,
        Declension::Vago,
        Declension::Morzatore,
        Declension::Rsipelle,
        Declension::Vescio,
        Declension::Rbone,
        Declension::Ampio,
        Declension::Dona,
        Declension::Acre,
        Declension::Duco,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 15] = [
    // Gliardo: 7,307 words (peso, fero, moro, rato, vero, talo).
    declension!("gliardo", 7307, "~o";
        masculine: "~o", "~i";
        feminine: "~a", "~e";
        superlative: "~issimo";
    ),
    // Ggiore: 5,218 words (acclive, volatile, numerale, rude, locale, fine).
    declension!("ggiore", 5218, "~e";
        masculine: "~e", "~i";
        feminine: "~e", "~i";
        superlative: "~issimo";
    ),
    // Ntastico: 2,994 words (lesbico, ebraico, pubblico, zotico, utopistico, identico).
    declension!("ntastico", 2994, "~o";
        masculine: "~o", "~i";
        feminine: "~a", "~he";
        superlative: "~issimo";
    ),
    // Zzaio: 689 words (buio, utilitario, urinario, rodio, vario, medio).
    declension!("zzaio", 689, "~o";
        masculine: "~o", "~";
        feminine: "~a", "~e";
        superlative: "~ssimo";
    ),
    // Ari: 583 words (wireless, bio, baby, ac, beat, touch).
    declension!("ari", 583, "~~";
        masculine: "~~", "~~";
        feminine: "~~", "~~";
        superlative: "~issim~";
    ),
    // Berista: 423 words (idiota, zarista, yemenita, rasta, dalmata, autista).
    declension!("berista", 423, "~a";
        masculine: "~a", "~i";
        feminine: "~a", "~e";
        superlative: "~issimo";
    ),
    // Vago: 367 words (ubriaco, sago, zingaresco, pago, vago, mago).
    declension!("vago", 367, "~o";
        masculine: "~o", "~hi";
        feminine: "~a", "~he";
        superlative: "~hissimo";
    ),
    // Morzatore: 328 words (urlatore, elaboratore, smorzatore, localizzatore, rallentatore, elevatore).
    declension!("morzatore", 328, "~o~e";
        masculine: "~o~e", "~o~i";
        feminine: "~~ice", "~~ici";
        superlative: "~issim~";
    ),
    // Rsipelle: 130 words (cane, live, yankee, bene, champagne, noce).
    declension!("rsipelle", 130, "~e";
        masculine: "~e", "~e";
        feminine: "~e", "~e";
        superlative: "~issimo";
    ),
    // Vescio: 106 words (ghiaccio, inconscio, lercio, rancio, guercio, subconscio).
    declension!("vescio", 106, "~io";
        masculine: "~io", "~i";
        feminine: "~ia", "~e";
        superlative: "~issimo";
    ),
    // Rbone: 28 words (leggere, zuccone, sgobbone, guascone, buzzicone, gocciolone).
    declension!("rbone", 28, "~e";
        masculine: "~e", "~i";
        feminine: "~a", "~e";
        superlative: "~issimo";
    ),
    // Ampio: 12 words (empio, propio, ampio, sapio, appio, ciclopio).
    declension!("ampio", 12, "~~o";
        masculine: "~~o", "~~";
        feminine: "~~a", "~~e";
        superlative: "~l~ssimo";
    ),
    // Dona: 12 words (nullipara, occhiona, durona, iofa, sorella, nella).
    declension!("dona", 12, "~a";
        masculine: "~a", "~e";
        feminine: "~a", "~e";
        superlative: "~issimo";
    ),
    // Acre: 9 words (acre, funebre, cre, lebre, alubre, gubre).
    declension!("acre", 9, "~~e";
        masculine: "~~e", "~~i";
        feminine: "~~e", "~~i";
        superlative: "~e~rimo";
    ),
    // Duco: 4 words (rauco, glauco, duco, uco).
    declension!("duco", 4, "~o";
        masculine: "~o", "~hi";
        feminine: "~a", "~he";
        superlative: "~issimo";
    ),
];
