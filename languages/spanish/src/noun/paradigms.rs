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
    /// Like `lata`: 26,934 words.
    Lata,
    /// Like `flor`: 3,814 words.
    Flor,
    /// Like `visión`: 2,405 words.
    Visión,
    /// Like `cocos`: 1,413 words.
    Cocos,
    /// Like `cén`: 231 words.
    Cén,
    /// Like `luz`: 201 words.
    Luz,
    /// Like `odín`: 197 words.
    Odín,
    /// Like `pás`: 169 words.
    Pás,
    /// Like `canon`: 33 words.
    Canon,
    /// Like `tún`: 29 words.
    Tún,
    /// Like `bolon`: 17 words.
    Bolon,
    /// Like `mitin`: 15 words.
    Mitin,
    /// Like `dumen`: 12 words.
    Dumen,
    /// Like `psi`: 7 words.
    Psi,
    /// Like `germen`: 6 words.
    Germen,
    /// Like `rry`: 5 words.
    Rry,
    /// Like `ivery`: 4 words.
    Ivery,
    /// Like `cheto`: 4 words.
    Cheto,
    /// Like `barman`: 3 words.
    Barman,
    /// Like `clac`: 3 words.
    Clac,
    /// Like `agen`: 3 words.
    Agen,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 21] = [
        Declension::Lata,
        Declension::Flor,
        Declension::Visión,
        Declension::Cocos,
        Declension::Cén,
        Declension::Luz,
        Declension::Odín,
        Declension::Pás,
        Declension::Canon,
        Declension::Tún,
        Declension::Bolon,
        Declension::Mitin,
        Declension::Dumen,
        Declension::Psi,
        Declension::Germen,
        Declension::Rry,
        Declension::Ivery,
        Declension::Cheto,
        Declension::Barman,
        Declension::Clac,
        Declension::Agen,
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
    // Lata: 26,934 words (pie, swap, deal, quiz, de, te).
    declension!("lata", 26934, "~";
        noun: "~", "~s";
    ),
    // Flor: 3,814 words (van, can, o, gas, país, grey).
    declension!("flor", 3814, "~";
        noun: "~", "~es";
    ),
    // Visión: 2,405 words (ibón, zircón, utilización, ubicación, ulceración, neón).
    declension!("visión", 2405, "~ó~";
        noun: "~ó~", "~o~es";
    ),
    // Cocos: 1,413 words (fa, gnosis, ime, sms, dna, stylus).
    declension!("cocos", 1413, "~";
        noun: "~", "~";
    ),
    // Cén: 231 words (ugandés, jején, ghanés, edén, kinés, eibarrés).
    declension!("cén", 231, "~é~";
        noun: "~é~", "~e~es";
    ),
    // Luz: 201 words (pez, voz, juez, luz, diez, vez).
    declension!("luz", 201, "~z";
        noun: "~z", "~ces";
    ),
    // Odín: 197 words (zepelín, orín, afín, kirguís, ukelín, theremín).
    declension!("odín", 197, "~í~";
        noun: "~í~", "~i~es";
    ),
    // Pás: 169 words (ruán, imán, afán, arán, edecán, peán).
    declension!("pás", 169, "~á~";
        noun: "~á~", "~a~es";
    ),
    // Canon: 33 words (baden, canon, taxon, brahman, nasion, margen).
    declension!("canon", 33, "~a~";
        noun: "~a~", "~á~es";
    ),
    // Tún: 29 words (atún, obús, kuskús, bús, mún, nrún).
    declension!("tún", 29, "~ú~";
        noun: "~ú~", "~u~es";
    ),
    // Bolon: 17 words (colon, poison, token, loden, joven, polen).
    declension!("bolon", 17, "~o~";
        noun: "~o~", "~ó~es";
    ),
    // Mitin: 15 words (crimen, origen, mitin, limen, virgen, pilsen).
    declension!("mitin", 15, "~i~";
        noun: "~i~", "~í~es";
    ),
    // Dumen: 12 words (lumen, acumen, rumen, numen, gluten, lumpen).
    declension!("dumen", 12, "~u~";
        noun: "~u~", "~ú~es";
    ),
    // Psi: 7 words (mi, pi, ni, xi, psi, fi).
    declension!("psi", 7, "~i";
        noun: "~i", "~íes";
    ),
    // Germen: 6 words (semen, theremin, germen, beicon, cemen, ntleman).
    declension!("germen", 6, "~e~";
        noun: "~e~", "~é~es";
    ),
    // Rry: 5 words (body, docureality, exgay, rry, rty).
    declension!("rry", 5, "~y";
        noun: "~y", "~ys|~ies";
    ),
    // Ivery: 4 words (lady, lity, ivery, dity).
    declension!("ivery", 4, "~y";
        noun: "~y", "~ies";
    ),
    // Cheto: 4 words (juanero, cheto, crático, cúbico).
    declension!("cheto", 4, "~o";
        noun: "~o", "~os|~as";
    ),
    // Barman: 3 words (barman, campin, dauntaun).
    declension!("barman", 3, "~a~";
        noun: "~a~", "~á~s|~á~es";
    ),
    // Clac: 3 words (clac, vac, sínic).
    declension!("clac", 3, "~c";
        noun: "~c", "~ques";
    ),
    // Agen: 3 words (agen, amen, asmon).
    declension!("agen", 3, "a~";
        noun: "a~", "á~es";
    ),
];
