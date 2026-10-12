//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's feature says):
/// `feminine` (sg)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal;
     feminine: $c0:tt;
    ) => {
        Paradigm {
            name: ::std::borrow::Cow::Borrowed($name),
            words: $words,
            lemma: ::std::borrow::Cow::Borrowed($lemma),
            forms: ::std::borrow::Cow::Borrowed(&[flexion::form!($c0)]),
        }
    };
}

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `sauvage`: 9,267 words.
    Sauvage,
    /// Like `fini`: 5,434 words.
    Fini,
    /// Like `bon`: 1,284 words.
    Bon,
    /// Like `chaleureux`: 583 words.
    Chaleureux,
    /// Like `vif`: 543 words.
    Vif,
    /// Like `gentil`: 367 words.
    Gentil,
    /// Like `régulier`: 220 words.
    Régulier,
    /// Like `fellateur`: 209 words.
    Fellateur,
    /// Like `rageur`: 140 words.
    Rageur,
    /// Like `net`: 51 words.
    Net,
    /// Like `public`: 10 words.
    Public,
    /// Like `gros`: 9 words.
    Gros,
    /// Like `beau`: 9 words.
    Beau,
    /// Like `aigu`: 7 words.
    Aigu,
    /// Like `long`: 3 words.
    Long,
    /// Like `franc`: 3 words.
    Franc,
    /// Like `faux`: 3 words.
    Faux,
    /// Like `précurseur`: 3 words.
    Précurseur,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 18] = [
        Declension::Sauvage,
        Declension::Fini,
        Declension::Bon,
        Declension::Chaleureux,
        Declension::Vif,
        Declension::Gentil,
        Declension::Régulier,
        Declension::Fellateur,
        Declension::Rageur,
        Declension::Net,
        Declension::Public,
        Declension::Gros,
        Declension::Beau,
        Declension::Aigu,
        Declension::Long,
        Declension::Franc,
        Declension::Faux,
        Declension::Précurseur,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 18] = [
    // Sauvage: 9,267 words (gratis, abaxial, abominable, absorbable, acceptable, accessible).
    declension!("sauvage", 9267, "~";
        feminine: "~";
    ),
    // Fini: 5,434 words (gratuit, adjectival, abbatial, abdominal, aberrant, abject).
    declension!("fini", 5434, "~";
        feminine: "~e";
    ),
    // Bon: 1,284 words (chien, moron, con, tien, mien, alsacien).
    declension!("bon", 1284, "~";
        feminine: "~ne";
    ),
    // Chaleureux: 583 words (gueux, heureux, paresseux, nuageux, douteux, merdeux).
    declension!("chaleureux", 583, "~x";
        feminine: "~se";
    ),
    // Vif: 543 words (vif, neuf, massif, serf, naïf, sauf).
    declension!("vif", 543, "~f";
        feminine: "~ve";
    ),
    // Gentil: 367 words (duel, cruel, éventuel, pluriel, mortel, gentil).
    declension!("gentil", 367, "~";
        feminine: "~le";
    ),
    // Régulier: 220 words (secret, quiet, fier, premier, cher, léger).
    declension!("régulier", 220, "~e~";
        feminine: "~è~e";
    ),
    // Fellateur: 209 words (observateur, moteur, directeur, provocateur, transformateur, formateur).
    declension!("fellateur", 209, "~eu~";
        feminine: "~~ice";
    ),
    // Rageur: 140 words (poseur, bricoleur, joueur, voyageur, voleur, songeur).
    declension!("rageur", 140, "~r";
        feminine: "~se";
    ),
    // Net: 51 words (violet, sot, net, chou, cadet, gringalet).
    declension!("net", 51, "~";
        feminine: "~te";
    ),
    // Public: 10 words (public, turc, ammoniac, caduc, laïc, parapublic).
    declension!("public", 10, "~c";
        feminine: "~que";
    ),
    // Gros: 9 words (las, gras, bas, coulis, gros, épais).
    declension!("gros", 9, "~";
        feminine: "~se";
    ),
    // Beau: 9 words (beau, nouveau, puceau, tourangeau, manceau, quadrijumeau).
    declension!("beau", 9, "~au";
        feminine: "~lle";
    ),
    // Aigu: 7 words (aigu, ambigu, contigu, subaigu, exigu, suraigu).
    declension!("aigu", 7, "~u";
        feminine: "~uë|~üe";
    ),
    // Long: 3 words (long, oblong, barlong).
    declension!("long", 3, "~";
        feminine: "~ue";
    ),
    // Franc: 3 words (franc, blanc, antiblanc).
    declension!("franc", 3, "~";
        feminine: "~he";
    ),
    // Faux: 3 words (faux, roux, archifaux).
    declension!("faux", 3, "~x";
        feminine: "~sse";
    ),
    // Précurseur: 3 words (avant-coureur, précurseur, entremetteur).
    declension!("précurseur", 3, "~~ur";
        feminine: "~~use";
    ),
];
