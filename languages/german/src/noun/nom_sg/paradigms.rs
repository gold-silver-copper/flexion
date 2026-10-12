//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's gender says):
/// `nominative` (sg)
/// `genitive` (sg)
/// `dative` (sg)
/// `accusative` (sg)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal;
     nominative: $c0:tt;
     genitive: $c1:tt;
     dative: $c2:tt;
     accusative: $c3:tt;
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
            ]),
        }
    };
}

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `platte`: 62,076 words.
    Platte,
    /// Like `leben`: 45,563 words.
    Leben,
    /// Like `blatt`: 29,183 words.
    Blatt,
    /// Like `pilot`: 2,166 words.
    Pilot,
    /// Like `herr`: 1,339 words.
    Herr,
    /// Like `grüner`: 975 words.
    Grüner,
    /// Like `brösel`: 512 words.
    Brösel,
    /// Like `linke`: 492 words.
    Linke,
    /// Like `zerwürfnis`: 432 words.
    Zerwürfnis,
    /// Like `ablaß`: 286 words.
    Ablaß,
    /// Like `zores`: 278 words.
    Zores,
    /// Like `name`: 202 words.
    Name,
    /// Like `inneres`: 112 words.
    Inneres,
    /// Like `blust`: 92 words.
    Blust,
    /// Like `xerophyt`: 26 words.
    Xerophyt,
    /// Like `fiduz`: 18 words.
    Fiduz,
    /// Like `böse`: 14 words.
    Böse,
    /// Like `herz`: 10 words.
    Herz,
    /// Like `krake`: 9 words.
    Krake,
    /// Like `druckbuchstabe`: 8 words.
    Druckbuchstabe,
    /// Like `kipppunkt`: 8 words.
    Kipppunkt,
    /// Like `äußeres`: 7 words.
    Äußeres,
    /// Like `bruderherz`: 7 words.
    Bruderherz,
    /// Like `lump`: 4 words.
    Lump,
    /// Like `einsasse`: 4 words.
    Einsasse,
    /// Like `freie`: 3 words.
    Freie,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 26] = [
        Declension::Platte,
        Declension::Leben,
        Declension::Blatt,
        Declension::Pilot,
        Declension::Herr,
        Declension::Grüner,
        Declension::Brösel,
        Declension::Linke,
        Declension::Zerwürfnis,
        Declension::Ablaß,
        Declension::Zores,
        Declension::Name,
        Declension::Inneres,
        Declension::Blust,
        Declension::Xerophyt,
        Declension::Fiduz,
        Declension::Böse,
        Declension::Herz,
        Declension::Krake,
        Declension::Druckbuchstabe,
        Declension::Kipppunkt,
        Declension::Äußeres,
        Declension::Bruderherz,
        Declension::Lump,
        Declension::Einsasse,
        Declension::Freie,
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
    // Platte: 62,076 words (a, april, f, y, o, eu).
    declension!("platte", 62076, "~";
        nominative: "~";
        genitive: "~";
        dative: referred;
        accusative: "~";
    ),
    // Leben: 45,563 words (september, november, german, dezember, ram, am).
    declension!("leben", 45563, "~";
        nominative: "~";
        genitive: "~s";
        dative: referred;
        accusative: "~";
    ),
    // Blatt: 29,183 words (haus, samstag, warenkorb, parlament, buch, fisch).
    declension!("blatt", 29183, "~";
        nominative: "~";
        genitive: "~es|~s";
        dative: referred;
        accusative: "~";
    ),
    // Pilot: 2,166 words (christ, präsident, narr, magyar, buddhist, gigant).
    declension!("pilot", 2166, "~";
        nominative: "~";
        genitive: "~en";
        dative: "~en";
        accusative: "~en";
    ),
    // Herr: 1,339 words (japanese, chinese, vietnamese, nepalese, togolese, hesse).
    declension!("herr", 1339, "~";
        nominative: "~";
        genitive: "~n";
        dative: "~n";
        accusative: "~n";
    ),
    // Grüner: 975 words (homosexueller, deutscher, windischer, süßer, federweißer, lieber).
    declension!("grüner", 975, "~r";
        nominative: "~r";
        genitive: "~n";
        dative: "~m";
        accusative: "~n";
    ),
    // Brösel: 512 words (yak, zucchini, scheit, jogurt, joghurt, kaugummi).
    declension!("brösel", 512, "~";
        nominative: "~";
        genitive: "~s";
        dative: "~";
        accusative: "~";
    ),
    // Linke: 492 words (arme, deutsche, homosexuelle, variable, obdachlose, alte).
    declension!("linke", 492, "~";
        nominative: "~";
        genitive: "~r";
        dative: referred;
        accusative: "~";
    ),
    // Zerwürfnis: 432 words (verhältnis, as, autobus, omnibus, bus, bambus).
    declension!("zerwürfnis", 432, "~";
        nominative: "~";
        genitive: "~ses";
        dative: referred;
        accusative: "~";
    ),
    // Ablaß: 286 words (kuß, haß, faß, schloß, schluß, guß).
    declension!("ablaß", 286, "~ß";
        nominative: "~ß";
        genitive: "~sses";
        dative: referred;
        accusative: "~ß";
    ),
    // Zores: 278 words (trüffel, e-mail, triangel, mail, shorts, jorts).
    declension!("zores", 278, "~";
        nominative: "~";
        genitive: "~";
        dative: "~";
        accusative: "~";
    ),
    // Name: 202 words (eigenname, familienname, name, kurzname, buchstabe, funke).
    declension!("name", 202, "~";
        nominative: "~";
        genitive: "~ns";
        dative: "~n";
        accusative: "~n";
    ),
    // Inneres: 112 words (adlerjunges, schwanenjunges, junges, neugeborenes, vielfaches, immaterielles).
    declension!("inneres", 112, "~s";
        nominative: "~s";
        genitive: "~n";
        dative: "~m";
        accusative: "~s";
    ),
    // Blust: 92 words (färberwegedorn, phonetiksystem, scherzkeks, beach, rechtkant, rindergulasch).
    declension!("blust", 92, "~";
        nominative: "~";
        genitive: "~es|~s";
        dative: "~";
        accusative: "~";
    ),
    // Xerophyt: 26 words (prinzess, karrierist, minorist, strafjustizakt, serviceweg, xerophyt).
    declension!("xerophyt", 26, "~";
        nominative: "~";
        genitive: "~en";
        dative: referred;
        accusative: "~";
    ),
    // Fiduz: 18 words (pferdefuhrwerk, fiduz, baustil, kornkreis, beutelbuch, garbenbund).
    declension!("fiduz", 18, "~";
        nominative: "~";
        genitive: "~es|~s";
        dative: "~e|~";
        accusative: "~";
    ),
    // Böse: 14 words (kabyle, böse, dipsomane, peschmerge, straßenjunge, normanne).
    declension!("böse", 14, "~";
        nominative: "~";
        genitive: "~n";
        dative: referred;
        accusative: "~";
    ),
    // Herz: 10 words (herz, schwesterherz, sportlerherz, sportherz, athletenherz, spenderherz).
    declension!("herz", 10, "~";
        nominative: "~";
        genitive: "~ens";
        dative: "~en";
        accusative: "~";
    ),
    // Krake: 9 words (anke, krake, bracke, banse, karnivore, carnivore).
    declension!("krake", 9, "~";
        nominative: "~";
        genitive: "~n|~";
        dative: referred;
        accusative: "~n|~";
    ),
    // Druckbuchstabe: 8 words (kleinbuchstabe, haufe, druckbuchstabe, großdruckbuchstabe, kursivbuchstabe, doppelbuchstabe).
    declension!("druckbuchstabe", 8, "~";
        nominative: "~";
        genitive: "~ns|~n";
        dative: referred;
        accusative: "~";
    ),
    // Kipppunkt: 8 words (dinucleotid, mononucleotid, polynucleotid, ribonucleotid, liebesakt, kipppunkt).
    declension!("kipppunkt", 8, "~";
        nominative: "~";
        genitive: "~e";
        dative: referred;
        accusative: "~";
    ),
    // Äußeres: 7 words (versinneres, äußeres, waldinneres, wortinneres, kircheninneres, inselinneres).
    declension!("äußeres", 7, "~es";
        nominative: "~es";
        genitive: "~en|~n";
        dative: "~em";
        accusative: "~es";
    ),
    // Bruderherz: 7 words (artischockenherz, bruderherz, menschenherz, frauenherz, pferdeherz, scheißherz).
    declension!("bruderherz", 7, "~";
        nominative: "~";
        genitive: "~ens|~es";
        dative: referred;
        accusative: "~";
    ),
    // Lump: 4 words (lump, tetragraph, vorführherr, apochromat).
    declension!("lump", 4, "~";
        nominative: "~";
        genitive: "~en|~s";
        dative: referred;
        accusative: "~en|~";
    ),
    // Einsasse: 4 words (einsasse, sicherheitsroboter, kalfaterer, kampfroboter).
    declension!("einsasse", 4, "~";
        nominative: "~";
        genitive: "~s";
        dative: referred;
        accusative: "~n";
    ),
    // Freie: 3 words (freie, aranese, sinnesphysiologe).
    declension!("freie", 3, "~";
        nominative: "~";
        genitive: "~n";
        dative: "~n";
        accusative: "~";
    ),
];
