//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's feature says):
/// `predicative` (sup)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal;
     predicative: $c0:tt;
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
    /// Like `veraltet`: 3,832 words.
    Veraltet,
    /// Like `glatt`: 1,116 words.
    Glatt,
    /// Like `ausgelassen`: 317 words.
    Ausgelassen,
    /// Like `muffig`: 182 words.
    Muffig,
    /// Like `aufrecht`: 82 words.
    Aufrecht,
    /// Like `maßlos`: 53 words.
    Maßlos,
    /// Like `scharf`: 29 words.
    Scharf,
    /// Like `träge`: 16 words.
    Träge,
    /// Like `zeitnah`: 8 words.
    Zeitnah,
    /// Like `schwarz`: 6 words.
    Schwarz,
    /// Like `dumm`: 6 words.
    Dumm,
    /// Like `hoch`: 6 words.
    Hoch,
    /// Like `amtlich`: 6 words.
    Amtlich,
    /// Like `amyloid`: 5 words.
    Amyloid,
    /// Like `fade`: 5 words.
    Fade,
    /// Like `kurz`: 5 words.
    Kurz,
    /// Like `naß`: 5 words.
    Naß,
    /// Like `geraum`: 4 words.
    Geraum,
    /// Like `aschearm`: 3 words.
    Aschearm,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 19] = [
        Declension::Veraltet,
        Declension::Glatt,
        Declension::Ausgelassen,
        Declension::Muffig,
        Declension::Aufrecht,
        Declension::Maßlos,
        Declension::Scharf,
        Declension::Träge,
        Declension::Zeitnah,
        Declension::Schwarz,
        Declension::Dumm,
        Declension::Hoch,
        Declension::Amtlich,
        Declension::Amyloid,
        Declension::Fade,
        Declension::Kurz,
        Declension::Naß,
        Declension::Geraum,
        Declension::Aschearm,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 19] = [
    // Veraltet: 3,832 words (orange, dick, hell, urban, rational, irrational).
    declension!("veraltet", 3832, "~";
        predicative: "am ~sten";
    ),
    // Glatt: 1,116 words (frei, flatulent, feist, new, pedant, blond).
    declension!("glatt", 1116, "~";
        predicative: "am ~esten";
    ),
    // Ausgelassen: 317 words (abnormal, aper, abgünstig, aktiv, aufmerksam, allgemein).
    declension!("ausgelassen", 317, "~~";
        predicative: "~m a~sten";
    ),
    // Muffig: 182 words (mutig, mobil, müde, minim, modern, modular).
    declension!("muffig", 182, "~~";
        predicative: "a~ m~sten";
    ),
    // Aufrecht: 82 words (aberrant, abrupt, absurd, abundant, arid, arrogant).
    declension!("aufrecht", 82, "~~";
        predicative: "~m a~esten";
    ),
    // Maßlos: 53 words (manifest, mies, mau, mild, matt, morbid).
    declension!("maßlos", 53, "~~";
        predicative: "a~ m~esten";
    ),
    // Scharf: 29 words (warm, stark, lang, scharf, schwach, krank).
    declension!("scharf", 29, "~a~";
        predicative: "am ~ä~sten";
    ),
    // Träge: 16 words (broke, malle, knorke, zähe, irre, vage).
    declension!("träge", 16, "~~";
        predicative: "am ~st~n";
    ),
    // Zeitnah: 8 words (nah, körpernah, zeitnah, bahnhofsnah, citynah, äquatornah).
    declension!("zeitnah", 8, "~a~";
        predicative: "am ~äc~sten";
    ),
    // Schwarz: 6 words (hart, kalt, schwarz, nachtschwarz, nasskalt, rabenschwarz).
    declension!("schwarz", 6, "~a~";
        predicative: "am ~ä~esten";
    ),
    // Dumm: 6 words (dumm, jung, klug, unklug, neunmalklug, weltklug).
    declension!("dumm", 6, "~u~";
        predicative: "am ~ü~sten";
    ),
    // Hoch: 6 words (hoch, grob, fromm, ranghoch, hüfthoch, storr).
    declension!("hoch", 6, "~o~";
        predicative: "am ~ö~sten";
    ),
    // Amtlich: 6 words (amerikanisch, amtlich, amharisch, amisch, amphibolisch, amtsmüde).
    declension!("amtlich", 6, "~~";
        predicative: "~ am~sten";
    ),
    // Amyloid: 5 words (ambivalent, amyloid, ambitiös, amüsant, ambitioniert).
    declension!("amyloid", 5, "~~";
        predicative: "~ am~esten";
    ),
    // Fade: 5 words (fade, ladylike, helle, weitgehend, dröge).
    declension!("fade", 5, "~~";
        predicative: "am ~~sten|am ~st~n";
    ),
    // Kurz: 5 words (kurz, gesund, ungesund, kurtz, erbgesund).
    declension!("kurz", 5, "~u~";
        predicative: "am ~ü~esten";
    ),
    // Naß: 5 words (naß, gewiß, ungewiß, kraß, blaß).
    declension!("naß", 5, "~ß";
        predicative: "am ~ssesten";
    ),
    // Geraum: 4 words (schmal, geraum, erdnah, praxisnah).
    declension!("geraum", 4, "~a~";
        predicative: "am ~a~sten";
    ),
    // Aschearm: 3 words (alkoholarm, anregungsarm, aschearm).
    declension!("aschearm", 3, "~~a~";
        predicative: "~m a~ä~sten";
    ),
];
