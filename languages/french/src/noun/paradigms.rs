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
    /// Like `ait`: 27,460 words.
    Ait,
    /// Like `bois`: 1,312 words.
    Bois,
    /// Like `bonhomme`: 243 words.
    Bonhomme,
    /// Like `jeu`: 187 words.
    Jeu,
    /// Like `mal`: 112 words.
    Mal,
    /// Like `tit-gris`: 81 words.
    Titgris,
    /// Like `au-frère`: 16 words.
    Aufrère,
    /// Like `ddy`: 14 words.
    Ddy,
    /// Like `ksar`: 14 words.
    Ksar,
    /// Like `russien`: 13 words.
    Russien,
    /// Like `-bar`: 13 words.
    Bar,
    /// Like `woman`: 12 words.
    Woman,
    /// Like `émail`: 9 words.
    Émail,
    /// Like `dium`: 7 words.
    Dium,
    /// Like `pedimentum`: 7 words.
    Pedimentum,
    /// Like `f-lieu`: 7 words.
    Flieu,
    /// Like `al-garou`: 7 words.
    Algarou,
    /// Like `-de-cuir`: 6 words.
    Decuir,
    /// Like `evus`: 5 words.
    Evus,
    /// Like `tto`: 4 words.
    Tto,
    /// Like `zza`: 4 words.
    Zza,
    /// Like `ndy`: 4 words.
    Ndy,
    /// Like `au-fils`: 4 words.
    Aufils,
    /// Like `de`: 3 words.
    De,
    /// Like `dr`: 3 words.
    Dr,
    /// Like `djeune`: 3 words.
    Djeune,
    /// Like `aïeul`: 3 words.
    Aïeul,
    /// Like `ombox`: 3 words.
    Ombox,
    /// Like `ler-retour`: 3 words.
    Lerretour,
    /// Like `al-jupon`: 3 words.
    Aljupon,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 30] = [
        Declension::Ait,
        Declension::Bois,
        Declension::Bonhomme,
        Declension::Jeu,
        Declension::Mal,
        Declension::Titgris,
        Declension::Aufrère,
        Declension::Ddy,
        Declension::Ksar,
        Declension::Russien,
        Declension::Bar,
        Declension::Woman,
        Declension::Émail,
        Declension::Dium,
        Declension::Pedimentum,
        Declension::Flieu,
        Declension::Algarou,
        Declension::Decuir,
        Declension::Evus,
        Declension::Tto,
        Declension::Zza,
        Declension::Ndy,
        Declension::Aufils,
        Declension::De,
        Declension::Dr,
        Declension::Djeune,
        Declension::Aïeul,
        Declension::Ombox,
        Declension::Lerretour,
        Declension::Aljupon,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 30] = [
    // Ait: 27,460 words (book, pie, deal, eagle, able, nu).
    declension!("ait", 27460, "~";
        noun: "~", "~s";
    ),
    // Bois: 1,312 words (fa, faq, ce, os, bm, led).
    declension!("bois", 1312, "~";
        noun: "~", "~";
    ),
    // Bonhomme: 243 words (bonhomme, gentilhomme, arrière-grand-oncle, p-garou, ille-douce, c-tireur).
    declension!("bonhomme", 243, "~~";
        noun: "~~", "~s~s";
    ),
    // Jeu: 187 words (ypréau, gâteau, jeu, dieu, beau, pou).
    declension!("jeu", 187, "~";
        noun: "~", "~x";
    ),
    // Mal: 112 words (dual, oral, vocal, mal, hospital, pascal).
    declension!("mal", 112, "~l";
        noun: "~l", "~ux";
    ),
    // Titgris: 81 words (in-marie, l-de-sac, tien-gorge, lle-de-jour, lle-de-nuit, l-au-vent).
    declension!("tit-gris", 81, "~~";
        noun: "~~", "~s~";
    ),
    // Aufrère: 16 words (au-père, teau-pilon, teau-piolet, teau-piqueur, au-frère, seau-mouche).
    declension!("au-frère", 16, "~~";
        noun: "~~", "~x~s";
    ),
    // Ddy: 14 words (hobby, lady, whisky, wallaby, ferry, rty).
    declension!("ddy", 14, "~y";
        noun: "~y", "~ies|~ys";
    ),
    // Ksar: 14 words (land, yachtsman, ksar, yachtwoman, yachtman, wattman).
    declension!("ksar", 14, "~a~";
        noun: "~a~", "~a~s|~e~";
    ),
    // Russien: 13 words (homme-sandwich, m-tam, cket-restaurant, veau-né, tamponneuse, de-soignant).
    declension!("russien", 13, "~~";
        noun: "~~", "~~s|~s~s";
    ),
    // Bar: 13 words (-pourri, -boutant, -point, -blanc, -pub, -bar).
    declension!("-bar", 13, "~";
        noun: "~", "s~s";
    ),
    // Woman: 12 words (gentleman, tennisman, tenniswoman, sinessman, raman, ryman).
    declension!("woman", 12, "~a~";
        noun: "~a~", "~e~|~a~s";
    ),
    // Émail: 9 words (bail, émail, ventail, corail, vitrail, fermail).
    declension!("émail", 9, "~il";
        noun: "~il", "~ux";
    ),
    // Dium: 7 words (ultimatum, tepidarium, dium, orium, ferendum, férendum).
    declension!("dium", 7, "~um";
        noun: "~um", "~ums|~a";
    ),
    // Pedimentum: 7 words (cæcum, ximum, mum, ntum, riptorium, pedimentum).
    declension!("pedimentum", 7, "~um";
        noun: "~um", "~a|~ums";
    ),
    // Flieu: 7 words (f-lieu, quin-marteau, tit-neveu, petit-neveu, pa-gâteau, ouille-taureau).
    declension!("f-lieu", 7, "~~";
        noun: "~~", "~s~x";
    ),
    // Algarou: 7 words (chal-ferrant, éral-démocrate, cial-démocrate, al-risque, mal-garou, al-garou).
    declension!("al-garou", 7, "~l~";
        noun: "~l~", "~ux~s";
    ),
    // Decuir: 6 words (-en-ciel, -de-pied, -de-vin, -de-cuir, -sous-roche, -basket).
    declension!("-de-cuir", 6, "~";
        noun: "~", "s~";
    ),
    // Evus: 5 words (uræus, balanus, nævus, nucléus, evus).
    declension!("evus", 5, "~us";
        noun: "~us", "~i";
    ),
    // Tto: 4 words (ghetto, anissimo, tto, presario).
    declension!("tto", 4, "~o";
        noun: "~o", "~os|~i";
    ),
    // Zza: 4 words (nova, khrouchtchevka, zza, ïba).
    declension!("zza", 4, "~a";
        noun: "~a", "~as";
    ),
    // Ndy: 4 words (rry, alty, ndy, nalty).
    declension!("ndy", 4, "~y";
        noun: "~y", "~ys|~ies";
    ),
    // Aufils: 4 words (au-fils, teau-feu, teau-phare, teau-lavoir).
    declension!("au-fils", 4, "~~";
        noun: "~~", "~x~";
    ),
    // De: 3 words (de, me, ve).
    declension!("de", 3, "~e";
        noun: "~e", "~ᵉˢ";
    ),
    // Dr: 3 words (dr, mr, sr).
    declension!("dr", 3, "~r";
        noun: "~r", "~ʳˢ";
    ),
    // Djeune: 3 words (érouve, djeune, rodescendante).
    declension!("djeune", 3, "~e";
        noun: "~e", "~es|~s";
    ),
    // Aïeul: 3 words (étal, aïeul, ctal).
    declension!("aïeul", 3, "~l";
        noun: "~l", "~ls|~ux";
    ),
    // Ombox: 3 words (venator, ombox, rabich).
    declension!("ombox", 3, "~";
        noun: "~", "~es";
    ),
    // Lerretour: 3 words (ler-retour, pe-crayon, veil-matin).
    declension!("ler-retour", 3, "~~";
        noun: "~~", "~~";
    ),
    // Aljupon: 3 words (al-vapeur, al-jupon, al-diable).
    declension!("al-jupon", 3, "~l~";
        noun: "~l~", "~ux~";
    ),
];
