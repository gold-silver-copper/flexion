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

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `mano`: 16,641 words.
    Mano,
    /// Like `mercatanzia`: 15,709 words.
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
    /// Like `mezzasega`: 8 words.
    Mezzasega,
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
    pub const ALL: [Declension; 26] = [
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
        Declension::Mezzasega,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 26] = [
    // Mano: 16,641 words (zero, sabato, peso, centavo, libero, euro).
    declension!("mano", 16641, "~o", [noun_gender: Masculine];
    noun: "~o", "~i";
    ),
    // Mercatanzia: 15,709 words (aquila, acicula, aria, arietta, guerra, acacia).
    declension!("mercatanzia", 15709, "~a", [noun_gender: Feminine];
    noun: "~a", "~e";
    ),
    // Lume: 13,482 words (abate, lente, decade, seme, are, andante). Like Mano, except in 1 cell: noun sg.
    declension!("lume", 13482, "~e", like Mano;
    noun: "~e", _;
    ),
    // Autorità: 7,297 words (fa, abaca, abracadabra, quiz, nu, robot).
    declension!("autorità", 7297, "~";
    noun: "~", "~";
    ),
    // Testimonio: 3,698 words (radio, bancario, curio, scenario, pugio, sorcio). Like Mano, except in 1 cell: noun pl.
    declension!("testimonio", 3698, "~o", [noun_gender: Masculine], like Mano;
    noun: _, "~";
    ),
    // Programma: 2,282 words (abada, comma, plasma, miasma, papa, poeta). Like Mercatanzia.
    declension!("programma", 2282, "~a", like Mercatanzia;
    stem noun_pl: "~- like programma";
    ),
    // Mosca: 1,323 words (replica, musica, pesca, riga, vacca, fuga). Like Mercatanzia, except in 1 cell: noun pl.
    declension!("mosca", 1323, "~a", [noun_gender: Feminine], like Mercatanzia;
    noun: _, "~he";
    ),
    // Banco: 746 words (tago, guanaco, bongo, abaco, loco, mango). Like Mano, except in 1 cell: noun pl.
    declension!("banco", 746, "~o", [noun_gender: Masculine], like Mano;
    noun: _, "~hi";
    ),
    // Traccia: 290 words (faccia, pioggia, pronuncia, fascia, lancia, goccia). Like Capsa with `` after each pattern (`ia` after ~).
    declension!("traccia", 290, "~ia", [noun_gender: Feminine], like Capsa with "", "ia" after "~";
    ),
    // Capolista: 86 words (capotavola, capobranco, caporeparto, capolinea, bagnoschiuma, capogruppo). Like Mano with `~` after each pattern.
    declension!("capolista", 86, "~o~", like Mano with "~";
    ),
    // Gerarca: 35 words (transfuga, auriga, collega, monarca, tetrarca, gerarca). Like Mercatanzia, except in 1 cell: noun pl.
    declension!("gerarca", 35, "~a", like Mercatanzia;
    noun: _, "~hi";
    ),
    // Mezzaluna: 34 words (terracotta, belladonna, capasanta, mezzaluna, primadonna, terraferma). Like Mercatanzia with `~a` after each pattern (`~e` after e).
    declension!("mezzaluna", 34, "~a~a", [noun_gender: Feminine], like Mercatanzia with "~a", "~e" after "e";
    ),
    // Spoiler: 32 words (boy, croissant, fan, magazine, yard, manager). Like Autorità, except in 1 cell: noun pl.
    declension!("spoiler", 32, "~", like Autorità;
    noun: _, "~s";
    ),
    // Braccio: 22 words (uovo, labbro, braccio, centinaio, migliaio, ginocchio). Like Mano, except in 1 cell: noun pl.
    declension!("braccio", 22, "~o", [noun_gender: Masculine], like Mano;
    noun: _, "~a";
    ),
    // Bassofondo: 14 words (altoforno, bassofondo, camposanto, caposaldo, caposcarico, collotorto). Like Mano with `~i` after each pattern (`~o` after o).
    declension!("bassofondo", 14, "~o~o", [noun_gender: Masculine], like Mano with "~i", "~o" after "o";
    ),
    // Uomo: 13 words (homo, uomo, galantuomo, gentiluomo, superuomo, nobiluomo). Like Mano, except in 1 cell: noun pl.
    declension!("uomo", 13, "~o", [noun_gender: Masculine], like Mano;
    noun: _, "~ini";
    ),
    // Acquaforte: 10 words (mezzanotte, malafede, cassaforte, mezzanave, acquaforte, piazzaforte). Like Mercatanzia with `~e` after each pattern (`~i` after e).
    declension!("acquaforte", 10, "~a~e", like Mercatanzia with "~e", "~i" after "e";
    ),
    // Superficie: 8 words (moglie, superficie, effigie, aviosuperficie, ipersuperficie, isosuperficie). Like Lume, except in 1 cell: noun pl.
    declension!("superficie", 8, "~e", [noun_gender: Feminine], like Lume;
    noun: _, "~";
    ),
    // Mezzasega: 8 words (cappalunga, cassapanca, cartasuga, gambasecca, mezzasega, mezzatacca). Like Mercatanzia with `~a` after each pattern (`~he` after e).
    declension!("mezzasega", 8, "~a~a", [noun_gender: Feminine], like Mercatanzia with "~a", "~he" after "e";
    ),
    // Bimbaminkia: 6 words (cartacarbone, scuolaguida, spinacristi, zappacavallo, bimbaminkia, mostramercato). Like Mercatanzia with `~` after each pattern.
    declension!("bimbaminkia", 6, "~a~", [noun_gender: Feminine], like Mercatanzia with "~";
    ),
    // Capoccia: 4 words (maracaia, capoccia, correggia, porteria). Like Mercatanzia, except in 1 cell: noun pl.
    declension!("capoccia", 4, "~a", like Mercatanzia;
    noun: _, "~";
    ),
    // Heroon: 4 words (rhyton, heroon, megaron, abaton). Like Testimonio with `a` after each pattern (`n` after o).
    declension!("heroon", 4, "~on", [noun_gender: Masculine], like Testimonio with "a", "n" after "o";
    ),
    // Capsa: 3 words (tabula, fabula, capsa). Like Mercatanzia, except in 1 cell: noun sg.
    declension!("capsa", 3, "~", [noun_gender: Feminine], like Mercatanzia;
    noun: "~", _;
    ),
    // Gattomammone: 3 words (gattomammone, capocarceriere, capotimoniere). Like Mano with `~e` after each pattern (`~i` after i).
    declension!("gattomammone", 3, "~o~e", [noun_gender: Masculine], like Mano with "~e", "~i" after "i";
    ),
    // Capofficina: 3 words (capofficina, decanapulazione, vetrofusione). Like Capolista, except in 1 cell: noun sg.
    declension!("capofficina", 3, "~~", like Capolista;
    noun: "~~", _;
    ),
    // Doppiosenso: 3 words (doppiovetro, occhiogrosso, doppiosenso). Like Testimonio with `~i` after each pattern (`~o` after o).
    declension!("doppiosenso", 3, "~o~o", [noun_gender: Masculine], like Testimonio with "~i", "~o" after "o";
    ),
];
