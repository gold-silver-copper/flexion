//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first. A `%`
//! is the first letter of the stem piece after it (reduplication: `%ε~κα`),
//! as the copy laws in `laws.rs` change it.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (`[gender: Neuter]`, if
/// they share them), the declension it is like (if any), the stem slots
/// it states with a stem and an ending table (`stem aorist: "~σ- like λυω"`:
/// the slot's cells are `λυω`'s table's patterns with `~σ` for their `~`; see
/// `stems.rs`), then a row of patterns per line (`derived`: the form comes
/// from another cell's, see `relations.rs`; `_`, or a row left out: the
/// form is its stem slot's, or the declension's it is like, or, in a declension like
/// no other, derived):
/// `noun` (sg, pl)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, [$($f:ident: $v:ident),*])? $(, like $parent:ident $(as $view:ident)? $(with $affix:literal $(, $other:literal after $letters:literal)?)?)?;
     $(stem noun_sg: $s0:tt;)?
     $(stem noun_pl: $s1:tt;)?
     $(noun: $c0:tt, $c1:tt;)?
    ) => {
        Paradigm {
            name: ::std::borrow::Cow::Borrowed($name),
            words: $words,
            lemma: ::std::borrow::Cow::Borrowed($lemma),
            parent: like!($($parent)?),
            reads: view!($($($view)?)?),
            inherent: ::std::borrow::Cow::Borrowed(&[$($(feature!($f: $v)),*)?]),
            table: ::std::borrow::Cow::Borrowed(concat!(
                $(flexion::cell!($c0),)? "\t",
                $(flexion::cell!($c1),)? "\t",
            )),
            stems: ::std::borrow::Cow::Borrowed(concat!(
                $(flexion::cell!($s0),)? "\t",
                $(flexion::cell!($s1),)? "\t",
            )),
            affix: affix!($($($affix $(, $other, $letters)?)?)?),
        }
    };
}

/// The declension a declension is like, by its name.
macro_rules! like {
    () => {
        None
    };
    ($parent:ident) => {
        Some(Declension::$parent as u16)
    };
}

/// The cells a declension reads in the one it is like, by the view's name (see
/// the views below); none: the same cells.
macro_rules! view {
    () => {
        None
    };
    ($view:ident) => {
        Some(::std::borrow::Cow::Borrowed(&$view))
    };
}

/// What a declension adds after each pattern of the one it is like (`with "~ь", "~я"
/// after "йлмть"`: `~ь`, or `~я` where that pattern ends in `й`, `л`, `м`, `т` or `ь`).
macro_rules! affix {
    () => {
        None
    };
    ($affix:literal) => {
        affix!($affix, $affix, "")
    };
    ($affix:literal, $other:literal, $letters:literal) => {
        Some(flexion::Affix {
            pattern: ::std::borrow::Cow::Borrowed($affix),
            other: ::std::borrow::Cow::Borrowed($other),
            letters: ::std::borrow::Cow::Borrowed($letters),
        })
    };
}

/// An inherent feature's value, as `(feature, value)` by their places.
macro_rules! feature {
    (noun_gender: $v:ident) => {
        (0, crate::NounGender::$v as u8)
    };
}

/// A view: per cell (in the cells' order), the cell of the declension it is
/// like that it reads (`pl_as_sg`: one value read as another).
const PL_AS_SG: [u16; 2] = [0, 0];

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `medio`: 46,334 words.
    Medio,
    /// Like `flor`: 6,068 words.
    Flor,
    /// Like `lastón`: 4,740 words.
    Lastón,
    /// Like `análisis`: 1,707 words.
    Análisis,
    /// Like `almacén`: 339 words.
    Almacén,
    /// Like `comodín`: 337 words.
    Comodín,
    /// Like `matriz`: 288 words.
    Matriz,
    /// Like `bacán`: 273 words.
    Bacán,
    /// Like `canon`: 46 words.
    Canon,
    /// Like `betún`: 38 words.
    Betún,
    /// Like `desorden`: 27 words.
    Desorden,
    /// Like `mitin`: 18 words.
    Mitin,
    /// Like `volumen`: 17 words.
    Volumen,
    /// Like `germen`: 8 words.
    Germen,
    /// Like `psi`: 7 words.
    Psi,
    /// Like `lobby`: 5 words.
    Lobby,
    /// Like `copín`: 4 words.
    Copín,
    /// Like `barman`: 3 words.
    Barman,
    /// Like `fijodalgo`: 3 words.
    Fijodalgo,
    /// Like `régimen`: 3 words.
    Régimen,
    /// Like `clac`: 3 words.
    Clac,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 21] = [
        Declension::Medio,
        Declension::Flor,
        Declension::Lastón,
        Declension::Análisis,
        Declension::Almacén,
        Declension::Comodín,
        Declension::Matriz,
        Declension::Bacán,
        Declension::Canon,
        Declension::Betún,
        Declension::Desorden,
        Declension::Mitin,
        Declension::Volumen,
        Declension::Germen,
        Declension::Psi,
        Declension::Lobby,
        Declension::Copín,
        Declension::Barman,
        Declension::Fijodalgo,
        Declension::Régimen,
        Declension::Clac,
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
    // Medio: 46,334 words (pie, swap, deal, abada, abate, quiz).
    declension!("medio", 46334, "~";
    noun: "~", "~s";
    ),
    // Flor: 6,068 words (abdominal, abductor, aboriginal, raptor, accidental, plural). Like Medio, except in 1 cell: noun pl.
    declension!("flor", 6068, "~", like Medio;
    noun: _, "~es";
    ),
    // Lastón: 4,740 words (barón, varón, chingón, corazón, millón, pronunciación).
    declension!("lastón", 4740, "~ó~";
    noun: "~ó~", "~o~es";
    ),
    // Análisis: 1,707 words (fa, gnosis, ime, menos, sms, paraguas). Like Medio.
    declension!("análisis", 1707, "~", like Medio as PL_AS_SG;
    ),
    // Almacén: 339 words (japonés, luxemburgués, inglés, francés, ugandés, javanés).
    declension!("almacén", 339, "~é~", [noun_gender: Masculine];
    noun: "~é~", "~e~es";
    ),
    // Comodín: 337 words (zepelín, jardín, jazmín, querubín, pudín, delfín).
    declension!("comodín", 337, "~í~", [noun_gender: Masculine];
    noun: "~í~", "~i~es";
    ),
    // Matriz: 288 words (pez, voz, juez, luz, diez, vez).
    declension!("matriz", 288, "~z";
    noun: "~z", "~ces";
    ),
    // Bacán: 273 words (tulipán, catalán, zaguán, restorán, carácter, guardián).
    declension!("bacán", 273, "~á~", [noun_gender: Masculine];
    noun: "~á~", "~a~es";
    ),
    // Canon: 46 words (baden, canon, taxon, gravamen, brahman, nasion).
    declension!("canon", 46, "~a~";
    noun: "~a~", "~á~es";
    ),
    // Betún: 38 words (autobús, microbús, común, atún, runrún, betún).
    declension!("betún", 38, "~ú~";
    noun: "~ú~", "~u~es";
    ),
    // Desorden: 27 words (abdomen, colon, poison, token, loden, joven).
    declension!("desorden", 27, "~o~";
    noun: "~o~", "~ó~es";
    ),
    // Mitin: 18 words (hooligan, crimen, origen, shamisen, mitin, limen).
    declension!("mitin", 18, "~i~";
    noun: "~i~", "~í~es";
    ),
    // Volumen: 17 words (lumen, acumen, rumen, numen, gluten, albumen).
    declension!("volumen", 17, "~u~", [noun_gender: Masculine];
    noun: "~u~", "~ú~es";
    ),
    // Germen: 8 words (gentleman, semen, theremin, germen, penetron, beicon).
    declension!("germen", 8, "~e~", [noun_gender: Masculine];
    noun: "~e~", "~é~es";
    ),
    // Psi: 7 words (mi, pi, ni, xi, psi, fi).
    declension!("psi", 7, "~i", [noun_gender: Feminine];
    noun: "~i", "~íes";
    ),
    // Lobby: 5 words (reality, lady, delivery, lobby, estadity).
    declension!("lobby", 5, "~y";
    noun: "~y", "~ies";
    ),
    // Copín: 4 words (vasín, vinín, huevín, copín). Like Comodín, except in 1 cell: noun pl.
    declension!("copín", 4, "~í~", [noun_gender: Masculine], like Comodín;
    noun: _, "~i~os";
    ),
    // Barman: 3 words (barman, campin, dauntaun). Like Canon, except in 1 cell: noun pl.
    declension!("barman", 3, "~a~", like Canon;
    noun: _, "~á~s|~á~es";
    ),
    // Fijodalgo: 3 words (hijodalgo, hijadalgo, fijodalgo). Like Medio with `~` after each pattern.
    declension!("fijodalgo", 3, "~~", like Medio with "~";
    ),
    // Régimen: 3 words (espécimen, régimen, narcorrégimen).
    declension!("régimen", 3, "~é~i~", [noun_gender: Masculine];
    noun: "~é~i~", "~e~í~es";
    ),
    // Clac: 3 words (clac, vivac, alcasínic).
    declension!("clac", 3, "~c";
    noun: "~c", "~ques";
    ),
];
