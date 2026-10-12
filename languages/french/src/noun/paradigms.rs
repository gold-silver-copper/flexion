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
    /// Like `trait`: 40,467 words.
    Trait,
    /// Like `bois`: 1,573 words.
    Bois,
    /// Like `marteau`: 324 words.
    Marteau,
    /// Like `journal`: 152 words.
    Journal,
    /// Like `blanc-seing`: 16 words.
    Blancseing,
    /// Like `caddy`: 15 words.
    Caddy,
    /// Like `ksar`: 14 words.
    Ksar,
    /// Like `superwoman`: 12 words.
    Superwoman,
    /// Like `bonhomme`: 11 words.
    Bonhomme,
    /// Like `émail`: 9 words.
    Émail,
    /// Like `impedimentum`: 8 words.
    Impedimentum,
    /// Like `atrium`: 8 words.
    Atrium,
    /// Like `dandy`: 5 words.
    Dandy,
    /// Like `match`: 5 words.
    Match,
    /// Like `nævus`: 5 words.
    Nævus,
    /// Like `nova`: 4 words.
    Nova,
    /// Like `putto`: 4 words.
    Putto,
    /// Like `aïeul`: 4 words.
    Aïeul,
    /// Like `de`: 3 words.
    De,
    /// Like `dipinto`: 3 words.
    Dipinto,
    /// Like `dr`: 3 words.
    Dr,
    /// Like `aller-retour`: 3 words.
    Allerretour,
    /// Like `djeune`: 3 words.
    Djeune,
    /// Like `bisaïeul`: 3 words.
    Bisaïeul,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 24] = [
        Declension::Trait,
        Declension::Bois,
        Declension::Marteau,
        Declension::Journal,
        Declension::Blancseing,
        Declension::Caddy,
        Declension::Ksar,
        Declension::Superwoman,
        Declension::Bonhomme,
        Declension::Émail,
        Declension::Impedimentum,
        Declension::Atrium,
        Declension::Dandy,
        Declension::Match,
        Declension::Nævus,
        Declension::Nova,
        Declension::Putto,
        Declension::Aïeul,
        Declension::De,
        Declension::Dipinto,
        Declension::Dr,
        Declension::Allerretour,
        Declension::Djeune,
        Declension::Bisaïeul,
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
    // Trait: 40,467 words (livre, book, chien, pie, connotation, minute).
    declension!("trait", 40467, "~";
        noun: "~", "~s";
    ),
    // Bois: 1,573 words (fa, abraxas, faq, ce, os, bm).
    declension!("bois", 1573, "~";
        noun: "~", "~";
    ),
    // Marteau: 324 words (ypréau, taureau, gâteau, jeu, dieu, bureau).
    declension!("marteau", 324, "~";
        noun: "~", "~x";
    ),
    // Journal: 152 words (abbatial, dual, rational, animal, oral, capital).
    declension!("journal", 152, "~l";
        noun: "~l", "~ux";
    ),
    // Blancseing: 16 words (tam-tam, ticket-restaurant, nouveau-né, auto-tamponneuse, garde-côte, aide-soignant).
    declension!("blanc-seing", 16, "~~";
        noun: "~~", "~~s|~s~s";
    ),
    // Caddy: 15 words (hobby, party, lavatory, buggy, lady, whisky).
    declension!("caddy", 15, "~y";
        noun: "~y", "~ies|~ys";
    ),
    // Ksar: 14 words (businesswoman, land, yachtsman, recordman, ksar, croque-madame).
    declension!("ksar", 14, "~a~";
        noun: "~a~", "~a~s|~e~";
    ),
    // Superwoman: 12 words (businessman, gentleman, cameraman, salaryman, superwoman, tennisman).
    declension!("superwoman", 12, "~a~";
        noun: "~a~", "~e~|~a~s";
    ),
    // Bonhomme: 11 words (bonhomme, arrière-grand-mère, gentilhomme, arrière-petit-enfant, arrière-grand-parent, arrière-petite-fille).
    declension!("bonhomme", 11, "~~";
        noun: "~~", "~s~s";
    ),
    // Émail: 9 words (bail, émail, travail, ventail, corail, vitrail).
    declension!("émail", 9, "~il";
        noun: "~il", "~ux";
    ),
    // Impedimentum: 8 words (maximum, minimum, quantum, scriptorium, impedimentum, dolium).
    declension!("impedimentum", 8, "~um";
        noun: "~um", "~a";
    ),
    // Atrium: 8 words (atrium, podium, emporium, ultimatum, referendum, tepidarium).
    declension!("atrium", 8, "~um";
        noun: "~um", "~ums|~a";
    ),
    // Dandy: 5 words (cherry, penalty, dandy, lorry, pénalty).
    declension!("dandy", 5, "~y";
        noun: "~y", "~ys|~ies";
    ),
    // Match: 5 words (match, brunch, boombox, venator, bérabich).
    declension!("match", 5, "~";
        noun: "~", "~es";
    ),
    // Nævus: 5 words (naevus, uræus, balanus, nævus, nucléus).
    declension!("nævus", 5, "~us";
        noun: "~us", "~i";
    ),
    // Nova: 4 words (pizza, nova, supernova, khrouchtchevka).
    declension!("nova", 4, "~a";
        noun: "~a", "~as";
    ),
    // Putto: 4 words (ghetto, pianissimo, putto, impresario).
    declension!("putto", 4, "~o";
        noun: "~o", "~os|~i";
    ),
    // Aïeul: 4 words (fractal, étal, aïeul, airial).
    declension!("aïeul", 4, "~l";
        noun: "~l", "~ls|~ux";
    ),
    // De: 3 words (de, me, ve).
    declension!("de", 3, "~e";
        noun: "~e", "~ᵉˢ";
    ),
    // Dipinto: 3 words (scenario, dipinto, zingaro).
    declension!("dipinto", 3, "~o";
        noun: "~o", "~i";
    ),
    // Dr: 3 words (dr, mr, sr).
    declension!("dr", 3, "~r";
        noun: "~r", "~ʳˢ";
    ),
    // Allerretour: 3 words (aller-retour, jupe-crayon, réveil-matin).
    declension!("aller-retour", 3, "~~";
        noun: "~~", "~~";
    ),
    // Djeune: 3 words (érouve, djeune, afrodescendante).
    declension!("djeune", 3, "~e";
        noun: "~e", "~es|~s";
    ),
    // Bisaïeul: 3 words (trisaïeul, bisaïeul, quadrisaïeul).
    declension!("bisaïeul", 3, "~l";
        noun: "~l", "~x|~ls";
    ),
];
