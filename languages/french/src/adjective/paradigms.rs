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
    /// Like `sauvage`: 8,973 words.
    Sauvage,
    /// Like `fini`: 4,059 words.
    Fini,
    /// Like `bon`: 1,284 words.
    Bon,
    /// Like `trivial`: 888 words.
    Trivial,
    /// Like `chaleureux`: 585 words.
    Chaleureux,
    /// Like `vif`: 543 words.
    Vif,
    /// Like `précis`: 479 words.
    Précis,
    /// Like `gentil`: 367 words.
    Gentil,
    /// Like `dar`: 284 words.
    Dar,
    /// Like `régulier`: 222 words.
    Régulier,
    /// Like `fellateur`: 209 words.
    Fellateur,
    /// Like `rageur`: 140 words.
    Rageur,
    /// Like `net`: 51 words.
    Net,
    /// Like `banal`: 12 words.
    Banal,
    /// Like `public`: 10 words.
    Public,
    /// Like `beau`: 10 words.
    Beau,
    /// Like `gros`: 9 words.
    Gros,
    /// Like `aigu`: 7 words.
    Aigu,
    /// Like `long`: 3 words.
    Long,
    /// Like `franc`: 3 words.
    Franc,
    /// Like `faux`: 3 words.
    Faux,
    /// Like `bateau`: 3 words.
    Bateau,
    /// Like `hébreu`: 3 words.
    Hébreu,
    /// Like `précurseur`: 3 words.
    Précurseur,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 24] = [
        Declension::Sauvage,
        Declension::Fini,
        Declension::Bon,
        Declension::Trivial,
        Declension::Chaleureux,
        Declension::Vif,
        Declension::Précis,
        Declension::Gentil,
        Declension::Dar,
        Declension::Régulier,
        Declension::Fellateur,
        Declension::Rageur,
        Declension::Net,
        Declension::Banal,
        Declension::Public,
        Declension::Beau,
        Declension::Gros,
        Declension::Aigu,
        Declension::Long,
        Declension::Franc,
        Declension::Faux,
        Declension::Bateau,
        Declension::Hébreu,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 24] = [
    // Sauvage: 8,973 words (abaxial, abominable, absorbable, acceptable, accessible, accordable).
    declension!("sauvage", 8973, "~";
        masculine: "~", "~s";
        feminine: "~", "~s";
        before_vowel: "~";
    ),
    // Fini: 4,059 words (gratuit, aberrant, abject, abrupt, nu, absent).
    declension!("fini", 4059, "~";
        masculine: "~", "~s";
        feminine: "~e", "~es";
        before_vowel: "~";
    ),
    // Bon: 1,284 words (chien, moron, con, tien, mien, alsacien).
    declension!("bon", 1284, "~";
        masculine: "~", "~s";
        feminine: "~ne", "~nes";
        before_vowel: "~";
    ),
    // Trivial: 888 words (adjectival, abbatial, abdominal, abnormal, abyssal, plural).
    declension!("trivial", 888, "~l";
        masculine: "~l", "~ux";
        feminine: "~le", "~les";
        before_vowel: "~l";
    ),
    // Chaleureux: 585 words (doux, gueux, heureux, paresseux, nuageux, douteux).
    declension!("chaleureux", 585, "~x";
        masculine: "~x", "~x";
        feminine: "~se", "~ses";
        before_vowel: "~x";
    ),
    // Vif: 543 words (vif, neuf, massif, serf, naïf, sauf).
    declension!("vif", 543, "~f";
        masculine: "~f", "~fs";
        feminine: "~ve", "~ves";
        before_vowel: "~f";
    ),
    // Précis: 479 words (japonais, bourgeois, anglais, pers, français, gris).
    declension!("précis", 479, "~";
        masculine: "~", "~";
        feminine: "~e", "~es";
        before_vowel: "~";
    ),
    // Gentil: 367 words (duel, cruel, éventuel, pluriel, mortel, gentil).
    declension!("gentil", 367, "~";
        masculine: "~", "~s";
        feminine: "~le", "~les";
        before_vowel: "~";
    ),
    // Dar: 284 words (gratis, orange, bio, poison, dar, nickel).
    declension!("dar", 284, "~";
        masculine: "~", "~";
        feminine: "~", "~";
        before_vowel: "~";
    ),
    // Régulier: 222 words (secret, quiet, fier, premier, cher, léger).
    declension!("régulier", 222, "~e~";
        masculine: "~e~", "~e~s";
        feminine: "~è~e", "~è~es";
        before_vowel: "~e~";
    ),
    // Fellateur: 209 words (observateur, moteur, directeur, provocateur, transformateur, formateur).
    declension!("fellateur", 209, "~eu~";
        masculine: "~eu~", "~eu~s";
        feminine: "~~ice", "~~ices";
        before_vowel: "~eu~";
    ),
    // Rageur: 140 words (poseur, bricoleur, joueur, voyageur, voleur, songeur).
    declension!("rageur", 140, "~r";
        masculine: "~r", "~rs";
        feminine: "~se", "~ses";
        before_vowel: "~r";
    ),
    // Net: 51 words (violet, sot, net, chou, cadet, gringalet).
    declension!("net", 51, "~";
        masculine: "~", "~s";
        feminine: "~te", "~tes";
        before_vowel: "~";
    ),
    // Banal: 12 words (natal, banal, final, atonal, tribal, tonal).
    declension!("banal", 12, "~l";
        masculine: "~l", "~ls|~ux";
        feminine: "~le", "~les";
        before_vowel: "~l";
    ),
    // Public: 10 words (public, turc, ammoniac, caduc, laïc, parapublic).
    declension!("public", 10, "~c";
        masculine: "~c", "~cs";
        feminine: "~que", "~ques";
        before_vowel: "~c";
    ),
    // Beau: 10 words (beau, nouveau, puceau, tourangeau, manceau, quadrijumeau).
    declension!("beau", 10, "~au";
        masculine: "~au", "~aux";
        feminine: "~lle", "~lles";
        before_vowel: "~au";
    ),
    // Gros: 9 words (las, gras, bas, coulis, gros, épais).
    declension!("gros", 9, "~";
        masculine: "~", "~";
        feminine: "~se", "~ses";
        before_vowel: "~";
    ),
    // Aigu: 7 words (aigu, ambigu, contigu, subaigu, exigu, suraigu).
    declension!("aigu", 7, "~u";
        masculine: "~u", "~us";
        feminine: "~uë|~üe", "~uës|~ües";
        before_vowel: "~u";
    ),
    // Long: 3 words (long, oblong, barlong).
    declension!("long", 3, "~";
        masculine: "~", "~s";
        feminine: "~ue", "~ues";
        before_vowel: "~";
    ),
    // Franc: 3 words (franc, blanc, antiblanc).
    declension!("franc", 3, "~";
        masculine: "~", "~s";
        feminine: "~he", "~hes";
        before_vowel: "~";
    ),
    // Faux: 3 words (faux, roux, archifaux).
    declension!("faux", 3, "~x";
        masculine: "~x", "~x";
        feminine: "~sse", "~sses";
        before_vowel: "~x";
    ),
    // Bateau: 3 words (bateau, marteau, morvandiau).
    declension!("bateau", 3, "~";
        masculine: "~", "~x";
        feminine: "~", "~x";
        before_vowel: "~";
    ),
    // Hébreu: 3 words (hébreu, scrogneugneu, mantchou).
    declension!("hébreu", 3, "~";
        masculine: "~", "~x";
        feminine: "~e", "~es";
        before_vowel: "~";
    ),
    // Précurseur: 3 words (avant-coureur, précurseur, entremetteur).
    declension!("précurseur", 3, "~~ur";
        masculine: "~~ur", "~~urs";
        feminine: "~~use", "~~uses";
        before_vowel: "~~ur";
    ),
];
