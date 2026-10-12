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
    /// Like `gagliardo`: 10,273 words.
    Gagliardo,
    /// Like `maggiore`: 7,517 words.
    Maggiore,
    /// Like `lubrico`: 5,290 words.
    Lubrico,
    /// Like `sezzaio`: 906 words.
    Sezzaio,
    /// Like `rossoblu`: 636 words.
    Rossoblu,
    /// Like `centrista`: 504 words.
    Centrista,
    /// Like `carico`: 438 words.
    Carico,
    /// Like `riparatore`: 392 words.
    Riparatore,
    /// Like `versipelle`: 144 words.
    Versipelle,
    /// Like `acconcio`: 132 words.
    Acconcio,
    /// Like `birbone`: 28 words.
    Birbone,
    /// Like `acre`: 13 words.
    Acre,
    /// Like `ampio`: 13 words.
    Ampio,
    /// Like `caduco`: 4 words.
    Caduco,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 14] = [
        Declension::Gagliardo,
        Declension::Maggiore,
        Declension::Lubrico,
        Declension::Sezzaio,
        Declension::Rossoblu,
        Declension::Centrista,
        Declension::Carico,
        Declension::Riparatore,
        Declension::Versipelle,
        Declension::Acconcio,
        Declension::Birbone,
        Declension::Acre,
        Declension::Ampio,
        Declension::Caduco,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 14] = [
    // Gagliardo: 10,273 words (peso, libero, fero, secondo, moro, gallo).
    declension!("gagliardo", 10273, "~o";
        masculine: "~o", "~i";
        feminine: "~a", "~e";
        superlative: "~issimo";
    ),
    // Maggiore: 7,517 words (acclive, andante, mobile, dilettante, finale, simile).
    declension!("maggiore", 7517, "~e";
        masculine: "~e", "~i";
        feminine: "~e", "~i";
        superlative: "~issimo";
    ),
    // Lubrico: 5,290 words (eccentrico, sacrifico, magnifico, lesbico, acetico, porco).
    declension!("lubrico", 5290, "~o";
        masculine: "~o", "~i";
        feminine: "~a", "~he";
        superlative: "~issimo";
    ),
    // Sezzaio: 906 words (monetario, bancario, ordinario, ovvio, buio, oratorio).
    declension!("sezzaio", 906, "~o";
        masculine: "~o", "~";
        feminine: "~a", "~e";
        superlative: "~ssimo";
    ),
    // Rossoblu: 636 words (gratis, wireless, standard, bio, baby, banana).
    declension!("rossoblu", 636, "~";
        masculine: "~", "~";
        feminine: "~", "~";
        superlative: "~ssimo";
    ),
    // Centrista: 504 words (idiota, antisemita, vietnamita, zarista, yemenita, socialista).
    declension!("centrista", 504, "~a";
        masculine: "~a", "~i";
        feminine: "~a", "~e";
        superlative: "~issimo";
    ),
    // Carico: 438 words (largo, ubriaco, bianco, fresco, sago, polacco).
    declension!("carico", 438, "~o";
        masculine: "~o", "~hi";
        feminine: "~a", "~he";
        superlative: "~hissimo";
    ),
    // Riparatore: 392 words (allenatore, conduttore, calcolatore, pescatore, lavoratore, motore).
    declension!("riparatore", 392, "~o~e";
        masculine: "~o~e", "~o~i";
        feminine: "~~ice", "~~ici";
        superlative: "~l~ssimo";
    ),
    // Versipelle: 144 words (cane, nature, live, online, sabre, single).
    declension!("versipelle", 144, "~e";
        masculine: "~e", "~e";
        feminine: "~e", "~e";
        superlative: "~issimo";
    ),
    // Acconcio: 132 words (ghiaccio, sudicio, bigio, accio, riccio, liscio).
    declension!("acconcio", 132, "~io";
        masculine: "~io", "~i";
        feminine: "~ia", "~e";
        superlative: "~issimo";
    ),
    // Birbone: 28 words (leggere, signore, poltrone, zuccone, chiacchierone, credulone).
    declension!("birbone", 28, "~e";
        masculine: "~e", "~i";
        feminine: "~a", "~e";
        superlative: "~issimo";
    ),
    // Acre: 13 words (acre, alacre, mediocre, celebre, insalubre, funebre).
    declension!("acre", 13, "~~e";
        masculine: "~~e", "~~i";
        feminine: "~~e", "~~i";
        superlative: "~e~rimo";
    ),
    // Ampio: 13 words (empio, doppio, propio, ampio, storpio, olimpio).
    declension!("ampio", 13, "~~o";
        masculine: "~~o", "~~";
        feminine: "~~a", "~~e";
        superlative: "~l~ssimo";
    ),
    // Caduco: 4 words (rauco, caduco, ciuco, glauco).
    declension!("caduco", 4, "~o";
        masculine: "~o", "~hi";
        feminine: "~a", "~he";
        superlative: "~issimo";
    ),
];
