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
    /// Like `medio`: 46,322 words.
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
    /// Like `body`: 8 words.
    Body,
    /// Like `germen`: 8 words.
    Germen,
    /// Like `psi`: 7 words.
    Psi,
    /// Like `lobby`: 5 words.
    Lobby,
    /// Like `bicúbico`: 4 words.
    Bicúbico,
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
    pub const ALL: [Declension; 23] = [
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
        Declension::Body,
        Declension::Germen,
        Declension::Psi,
        Declension::Lobby,
        Declension::Bicúbico,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 23] = [
    // Medio: 46,322 words (pie, swap, deal, abada, abate, quiz).
    declension!("medio", 46322, "~";
        noun: "~", "~s";
    ),
    // Flor: 6,068 words (abdominal, abductor, aboriginal, raptor, accidental, plural).
    declension!("flor", 6068, "~";
        noun: "~", "~es";
    ),
    // Lastón: 4,740 words (barón, varón, chingón, corazón, millón, pronunciación).
    declension!("lastón", 4740, "~ó~";
        noun: "~ó~", "~o~es";
    ),
    // Análisis: 1,707 words (fa, gnosis, ime, menos, sms, paraguas).
    declension!("análisis", 1707, "~";
        noun: "~", "~";
    ),
    // Almacén: 339 words (japonés, luxemburgués, inglés, francés, ugandés, javanés).
    declension!("almacén", 339, "~é~";
        noun: "~é~", "~e~es";
    ),
    // Comodín: 337 words (zepelín, jardín, jazmín, querubín, pudín, delfín).
    declension!("comodín", 337, "~í~";
        noun: "~í~", "~i~es";
    ),
    // Matriz: 288 words (pez, voz, juez, luz, diez, vez).
    declension!("matriz", 288, "~z";
        noun: "~z", "~ces";
    ),
    // Bacán: 273 words (tulipán, catalán, zaguán, restorán, carácter, guardián).
    declension!("bacán", 273, "~á~";
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
    declension!("volumen", 17, "~u~";
        noun: "~u~", "~ú~es";
    ),
    // Body: 8 words (cherry, body, party, ferry, brandy, bully).
    declension!("body", 8, "~y";
        noun: "~y", "~ys|~ies";
    ),
    // Germen: 8 words (gentleman, semen, theremin, germen, penetron, beicon).
    declension!("germen", 8, "~e~";
        noun: "~e~", "~é~es";
    ),
    // Psi: 7 words (mi, pi, ni, xi, psi, fi).
    declension!("psi", 7, "~i";
        noun: "~i", "~íes";
    ),
    // Lobby: 5 words (reality, lady, delivery, lobby, estadity).
    declension!("lobby", 5, "~y";
        noun: "~y", "~ies";
    ),
    // Bicúbico: 4 words (sanjuanero, concheto, pornocrático, bicúbico).
    declension!("bicúbico", 4, "~o";
        noun: "~o", "~os|~as";
    ),
    // Copín: 4 words (vasín, vinín, huevín, copín).
    declension!("copín", 4, "~í~";
        noun: "~í~", "~i~os";
    ),
    // Barman: 3 words (barman, campin, dauntaun).
    declension!("barman", 3, "~a~";
        noun: "~a~", "~á~s|~á~es";
    ),
    // Fijodalgo: 3 words (hijodalgo, hijadalgo, fijodalgo).
    declension!("fijodalgo", 3, "~~";
        noun: "~~", "~s~";
    ),
    // Régimen: 3 words (espécimen, régimen, narcorrégimen).
    declension!("régimen", 3, "~é~i~";
        noun: "~é~i~", "~e~í~es";
    ),
    // Clac: 3 words (clac, vivac, alcasínic).
    declension!("clac", 3, "~c";
        noun: "~c", "~ques";
    ),
];
