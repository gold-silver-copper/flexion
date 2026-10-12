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
/// `before_vowel`
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal;
     masculine: $c0:tt, $c1:tt;
     feminine: $c2:tt, $c3:tt;
     before_vowel: $c4:tt;
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
    /// Like `uvage`: 6,771 words.
    Uvage,
    /// Like `aud`: 3,455 words.
    Aud,
    /// Like `bon`: 1,170 words.
    Bon,
    /// Like `vial`: 686 words.
    Vial,
    /// Like `aleureux`: 516 words.
    Aleureux,
    /// Like `cis`: 435 words.
    Cis,
    /// Like `vif`: 400 words.
    Vif,
    /// Like `dar`: 278 words.
    Dar,
    /// Like `gentil`: 251 words.
    Gentil,
    /// Like `gulier`: 207 words.
    Gulier,
    /// Like `émoteur`: 178 words.
    Émoteur,
    /// Like `ladeur`: 133 words.
    Ladeur,
    /// Like `net`: 50 words.
    Net,
    /// Like `nal`: 11 words.
    Nal,
    /// Like `gros`: 9 words.
    Gros,
    /// Like `beau`: 9 words.
    Beau,
    /// Like `public`: 7 words.
    Public,
    /// Like `aigu`: 5 words.
    Aigu,
    /// Like `hébreu`: 3 words.
    Hébreu,
    /// Like `curseur`: 3 words.
    Curseur,
    /// Like `cheur`: 3 words.
    Cheur,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 21] = [
        Declension::Uvage,
        Declension::Aud,
        Declension::Bon,
        Declension::Vial,
        Declension::Aleureux,
        Declension::Cis,
        Declension::Vif,
        Declension::Dar,
        Declension::Gentil,
        Declension::Gulier,
        Declension::Émoteur,
        Declension::Ladeur,
        Declension::Net,
        Declension::Nal,
        Declension::Gros,
        Declension::Beau,
        Declension::Public,
        Declension::Aigu,
        Declension::Hébreu,
        Declension::Curseur,
        Declension::Cheur,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 21] = [
    // Uvage: 6,771 words (abaxial, viable, minuscule, beige, black, mauve).
    declension!("uvage", 6771, "~";
        masculine: "~", "~s";
        feminine: "~", "~s";
        before_vowel: "~";
    ),
    // Aud: 3,455 words (gratuit, nu, second, flatulent, bot, standard).
    declension!("aud", 3455, "~";
        masculine: "~", "~s";
        feminine: "~e", "~es";
        before_vowel: "~";
    ),
    // Bon: 1,170 words (con, tien, mien, bon, italien, breton).
    declension!("bon", 1170, "~";
        masculine: "~", "~s";
        feminine: "~ne", "~nes";
        before_vowel: "~";
    ),
    // Vial: 686 words (plural, fiscal, global, mental, musical, dual).
    declension!("vial", 686, "~l";
        masculine: "~l", "~ux";
        feminine: "~le", "~les";
        before_vowel: "~l";
    ),
    // Aleureux: 516 words (faux, doux, roux, gueux, heureux, nuageux).
    declension!("aleureux", 516, "~x";
        masculine: "~x", "~x";
        feminine: "~se", "~ses";
        before_vowel: "~x";
    ),
    // Cis: 435 words (japonais, pers, gris, gaulois, hongrois, pris).
    declension!("cis", 435, "~";
        masculine: "~", "~";
        feminine: "~e", "~es";
        before_vowel: "~";
    ),
    // Vif: 400 words (vif, neuf, massif, serf, naïf, sauf).
    declension!("vif", 400, "~f";
        masculine: "~f", "~fs";
        feminine: "~ve", "~ves";
        before_vowel: "~f";
    ),
    // Dar: 278 words (gratis, bio, poison, dar, nickel, x).
    declension!("dar", 278, "~";
        masculine: "~", "~";
        feminine: "~", "~";
        before_vowel: "~";
    ),
    // Gentil: 251 words (duel, cruel, éventuel, pluriel, gentil, matériel).
    declension!("gentil", 251, "~";
        masculine: "~", "~s";
        feminine: "~le", "~les";
        before_vowel: "~";
    ),
    // Gulier: 207 words (secret, quiet, fier, premier, cher, léger).
    declension!("gulier", 207, "~e~";
        masculine: "~e~", "~e~s";
        feminine: "~è~e", "~è~es";
        before_vowel: "~e~";
    ),
    // Émoteur: 178 words (formateur, éjaculateur, vecteur, destructeur, migrateur, spéculateur).
    declension!("émoteur", 178, "~eu~";
        masculine: "~eu~", "~eu~s";
        feminine: "~~ice", "~~ices";
        before_vowel: "~eu~";
    ),
    // Ladeur: 133 words (poseur, bricoleur, joueur, voyageur, voleur, songeur).
    declension!("ladeur", 133, "~r";
        masculine: "~r", "~rs";
        feminine: "~se", "~ses";
        before_vowel: "~r";
    ),
    // Net: 50 words (violet, sot, net, chou, gringalet, coi).
    declension!("net", 50, "~";
        masculine: "~", "~s";
        feminine: "~te", "~tes";
        before_vowel: "~";
    ),
    // Nal: 11 words (final, tonal, cérémonial, nymphal, tombal, tal).
    declension!("nal", 11, "~l";
        masculine: "~l", "~ls|~ux";
        feminine: "~le", "~les";
        before_vowel: "~l";
    ),
    // Gros: 9 words (las, gras, bas, gros, épais, maous).
    declension!("gros", 9, "~";
        masculine: "~", "~";
        feminine: "~se", "~ses";
        before_vowel: "~";
    ),
    // Beau: 9 words (beau, puceau, tourangeau, manceau, esquimau, uveau).
    declension!("beau", 9, "~au";
        masculine: "~au", "~aux";
        feminine: "~lle", "~lles";
        before_vowel: "~au";
    ),
    // Public: 7 words (public, turc, laïc, vioc, maufranc, moniac).
    declension!("public", 7, "~c";
        masculine: "~c", "~cs";
        feminine: "~que", "~ques";
        before_vowel: "~c";
    ),
    // Aigu: 5 words (aigu, bégu, bigu, tigu, igu).
    declension!("aigu", 5, "~u";
        masculine: "~u", "~us";
        feminine: "~uë|~üe", "~uës|~ües";
        before_vowel: "~u";
    ),
    // Hébreu: 3 words (hébreu, scrogneugneu, mantchou).
    declension!("hébreu", 3, "~";
        masculine: "~", "~x";
        feminine: "~e", "~es";
        before_vowel: "~";
    ),
    // Curseur: 3 words (avant-coureur, curseur, tremetteur).
    declension!("curseur", 3, "~~ur";
        masculine: "~~ur", "~~urs";
        feminine: "~~use", "~~uses";
        before_vowel: "~~ur";
    ),
    // Cheur: 3 words (vengeur, enchanteur, cheur).
    declension!("cheur", 3, "~u~";
        masculine: "~u~", "~u~s";
        feminine: "~~esse", "~~esses";
        before_vowel: "~u~";
    ),
];
