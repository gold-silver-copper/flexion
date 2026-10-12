//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (`[gender: Neuter]`, if
/// they share them), the declension it is like (if any), then a row of patterns
/// per line (`derived`: the form comes from another cell's, see
/// `relations.rs`; `_`: the form is the declension's it is like):
/// `nominative_masculine` (sg, pl)
/// `nominative_feminine` (sg, pl)
/// `nominative_neuter` (sg, pl)
/// `genitive_masculine` (sg, pl)
/// `genitive_feminine` (sg, pl)
/// `genitive_neuter` (sg, pl)
/// `dative_masculine` (sg, pl)
/// `dative_feminine` (sg, pl)
/// `dative_neuter` (sg, pl)
/// `accusative_masculine` (sg, pl)
/// `accusative_feminine` (sg, pl)
/// `accusative_neuter` (sg, pl)
/// `ablative_masculine` (sg, pl)
/// `ablative_feminine` (sg, pl)
/// `ablative_neuter` (sg, pl)
/// `vocative_masculine` (sg, pl)
/// `vocative_feminine` (sg, pl)
/// `vocative_neuter` (sg, pl)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident)?;
     nominative_masculine: $c0:tt, $c1:tt;
     nominative_feminine: $c2:tt, $c3:tt;
     nominative_neuter: $c4:tt, $c5:tt;
     genitive_masculine: $c6:tt, $c7:tt;
     genitive_feminine: $c8:tt, $c9:tt;
     genitive_neuter: $c10:tt, $c11:tt;
     dative_masculine: $c12:tt, $c13:tt;
     dative_feminine: $c14:tt, $c15:tt;
     dative_neuter: $c16:tt, $c17:tt;
     accusative_masculine: $c18:tt, $c19:tt;
     accusative_feminine: $c20:tt, $c21:tt;
     accusative_neuter: $c22:tt, $c23:tt;
     ablative_masculine: $c24:tt, $c25:tt;
     ablative_feminine: $c26:tt, $c27:tt;
     ablative_neuter: $c28:tt, $c29:tt;
     vocative_masculine: $c30:tt, $c31:tt;
     vocative_feminine: $c32:tt, $c33:tt;
     vocative_neuter: $c34:tt, $c35:tt;
    ) => {
        Paradigm {
            name: ::std::borrow::Cow::Borrowed($name),
            words: $words,
            lemma: ::std::borrow::Cow::Borrowed($lemma),
            parent: like!($($parent)?),
            inherent: ::std::borrow::Cow::Borrowed(&[]),
            table: ::std::borrow::Cow::Borrowed(concat!(
                flexion::cell!($c0), "\t",
                flexion::cell!($c1), "\t",
                flexion::cell!($c2), "\t",
                flexion::cell!($c3), "\t",
                flexion::cell!($c4), "\t",
                flexion::cell!($c5), "\t",
                flexion::cell!($c6), "\t",
                flexion::cell!($c7), "\t",
                flexion::cell!($c8), "\t",
                flexion::cell!($c9), "\t",
                flexion::cell!($c10), "\t",
                flexion::cell!($c11), "\t",
                flexion::cell!($c12), "\t",
                flexion::cell!($c13), "\t",
                flexion::cell!($c14), "\t",
                flexion::cell!($c15), "\t",
                flexion::cell!($c16), "\t",
                flexion::cell!($c17), "\t",
                flexion::cell!($c18), "\t",
                flexion::cell!($c19), "\t",
                flexion::cell!($c20), "\t",
                flexion::cell!($c21), "\t",
                flexion::cell!($c22), "\t",
                flexion::cell!($c23), "\t",
                flexion::cell!($c24), "\t",
                flexion::cell!($c25), "\t",
                flexion::cell!($c26), "\t",
                flexion::cell!($c27), "\t",
                flexion::cell!($c28), "\t",
                flexion::cell!($c29), "\t",
                flexion::cell!($c30), "\t",
                flexion::cell!($c31), "\t",
                flexion::cell!($c32), "\t",
                flexion::cell!($c33), "\t",
                flexion::cell!($c34), "\t",
                flexion::cell!($c35), "\t",
            )),
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

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `quis`: 27 words.
    Quis,
    /// Like `ipse`: 12 words.
    Ipse,
    /// Like `uter`: 12 words.
    Uter,
    /// Like `alius`: 8 words.
    Alius,
    /// Like `utervīs`: 8 words.
    Utervīs,
    /// Like `complūrēs`: 2 words.
    Complūrēs,
    /// Like `complūra`: 2 words.
    Complūra,
    /// Like `is`: 1 words.
    Is,
    /// Like `sē`: 1 words.
    Sē,
    /// Like `ea`: 1 words.
    Ea,
    /// Like `nēmō`: 1 words.
    Nēmō,
    /// Like `id`: 1 words.
    Id,
    /// Like `quī`: 1 words.
    Quī,
    /// Like `quīdam`: 1 words.
    Quīdam,
    /// Like `ecquī`: 1 words.
    Ecquī,
    /// Like `suī`: 1 words.
    Suī,
    /// Like `ambō`: 1 words.
    Ambō,
    /// Like `sēsē`: 1 words.
    Sēsē,
    /// Like `quīnam`: 1 words.
    Quīnam,
    /// Like `quīvīs`: 1 words.
    Quīvīs,
    /// Like `complūsculī`: 1 words.
    Complūsculī,
    /// Like `ecquīnam`: 1 words.
    Ecquīnam,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 22] = [
        Declension::Quis,
        Declension::Ipse,
        Declension::Uter,
        Declension::Alius,
        Declension::Utervīs,
        Declension::Complūrēs,
        Declension::Complūra,
        Declension::Is,
        Declension::Sē,
        Declension::Ea,
        Declension::Nēmō,
        Declension::Id,
        Declension::Quī,
        Declension::Quīdam,
        Declension::Ecquī,
        Declension::Suī,
        Declension::Ambō,
        Declension::Sēsē,
        Declension::Quīnam,
        Declension::Quīvīs,
        Declension::Complūsculī,
        Declension::Ecquīnam,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 22] = [
    // Quis: 27 words (hic, quis, aliquī, hice, quisquis, quidquid).
    declension!("quis", 27, "~i~";
        nominative_masculine: "~i~", "~īs~e";
        nominative_feminine: "~ae~", "~ae~";
        nominative_neuter: "~u~", "~ae~";
        genitive_masculine: "~īus~e", "~ōrun~";
        genitive_feminine: "~īus~e", "~ārun~";
        genitive_neuter: "~īus~e", "~ōrun~";
        dative_masculine: "~ī~", "~īs~e";
        dative_feminine: "~ī~", "~īs~e";
        dative_neuter: "~ī~", "~īs~e";
        accusative_masculine: "~un~", "~ōs~e";
        accusative_feminine: "~an~", "~ās~e";
        accusative_neuter: "~u~", "~ae~";
        ablative_masculine: "~ō~", "~īs~e";
        ablative_feminine: "~ā~", "~īs~e";
        ablative_neuter: "~ō~", "~īs~e";
        vocative_masculine: derived, derived;
        vocative_feminine: derived, derived;
        vocative_neuter: derived, derived;
    ),
    // Ipse: 12 words (ille, iste, ipse, quisque, ūnusquisque, quīcumque).
    declension!("ipse", 12, "~e";
        nominative_masculine: "~e", "~ī";
        nominative_feminine: "~a", "~ae";
        nominative_neuter: "~ud", "~a";
        genitive_masculine: "~īus", "~ōrum";
        genitive_feminine: "~īus", "~ārum";
        genitive_neuter: "~īus", "~ōrum";
        dative_masculine: "~ī", "~īs";
        dative_feminine: "~ī", "~īs";
        dative_neuter: "~ī", "~īs";
        accusative_masculine: "~um", "~ōs";
        accusative_feminine: "~am", "~ās";
        accusative_neuter: "~ud", "~a";
        ablative_masculine: "~ō", "~īs";
        ablative_feminine: "~ā", "~īs";
        ablative_neuter: "~ō", "~īs";
        vocative_masculine: derived, derived;
        vocative_feminine: derived, derived;
        vocative_neuter: derived, derived;
    ),
    // Uter: 12 words (alter, noster, īdem, uter, vester, egomet).
    declension!("uter", 12, "~e~";
        nominative_masculine: "~e~", "~~ī";
        nominative_feminine: "~~a", "~~ae";
        nominative_neuter: "~~um", "~~a";
        genitive_masculine: "~~ī", "~~ōrum";
        genitive_feminine: "~~ae", "~~ārum";
        genitive_neuter: "~~ī", "~~ōrum";
        dative_masculine: "~~ō", "~~īs";
        dative_feminine: "~~ae", "~~īs";
        dative_neuter: "~~ō", "~~īs";
        accusative_masculine: "~~um", "~~ōs";
        accusative_feminine: "~~am", "~~ās";
        accusative_neuter: "~~um", "~~a";
        ablative_masculine: "~~ō", "~~īs";
        ablative_feminine: "~~ā", "~~īs";
        ablative_neuter: "~~ō", "~~īs";
        vocative_masculine: "~e~", "~~ī";
        vocative_feminine: "~~a", "~~ae";
        vocative_neuter: "~~um", "~~a";
    ),
    // Alius: 8 words (alius, tuus, meus, suus, ūllus, cuius). Like Ipse, except in 15 cells: nominative_masculine sg, nominative_neuter sg, genitive_masculine sg, genitive_feminine sg, genitive_neuter sg, dative_masculine sg, ….
    declension!("alius", 8, "~us", like Ipse;
        nominative_masculine: "~us", _;
        nominative_feminine: _, _;
        nominative_neuter: "~um", _;
        genitive_masculine: "~ī", _;
        genitive_feminine: "~ae", _;
        genitive_neuter: "~ī", _;
        dative_masculine: "~ō", _;
        dative_feminine: "~ae", _;
        dative_neuter: "~ō", _;
        accusative_masculine: _, _;
        accusative_feminine: _, _;
        accusative_neuter: "~um", _;
        ablative_masculine: _, _;
        ablative_feminine: _, _;
        ablative_neuter: _, _;
        vocative_masculine: "~e", "~ī";
        vocative_feminine: "~a", "~ae";
        vocative_neuter: "~um", "~a";
    ),
    // Utervīs: 8 words (tūtemet, temet, uterque, ipsemet, utervīs, utercumque).
    declension!("utervīs", 8, "~e~~";
        nominative_masculine: "~e~~", "~~ī~";
        nominative_feminine: "~~a~", "~~ae~";
        nominative_neuter: "~~um~", "~~a~";
        genitive_masculine: "~~īus~", "~~ōrum~";
        genitive_feminine: "~~īus~", "~~ārum~";
        genitive_neuter: "~~īus~", "~~ōrum~";
        dative_masculine: "~~ī~", "~~īs~";
        dative_feminine: "~~ī~", "~~īs~";
        dative_neuter: "~~ī~", "~~īs~";
        accusative_masculine: "~~um~", "~~ōs~";
        accusative_feminine: "~~am~", "~~ās~";
        accusative_neuter: "~~um~", "~~a~";
        ablative_masculine: "~~ō~", "~~īs~";
        ablative_feminine: "~~ā~", "~~īs~";
        ablative_neuter: "~~ō~", "~~īs~";
        vocative_masculine: "~e~~", "~~ī~";
        vocative_feminine: "~~a~", "~~ae~";
        vocative_neuter: "~~um~", "~~a~";
    ),
    // Complūrēs: 2 words (complūrēs, conplūrēs).
    declension!("complūrēs", 2, "~ēs";
        nominative_masculine: "~ēs", "~ēs";
        nominative_feminine: derived, "~ēs";
        nominative_neuter: derived, "~a|~ia";
        genitive_masculine: derived, "~ium|~um";
        genitive_feminine: derived, "~ium|~um";
        genitive_neuter: derived, "~ium|~um";
        dative_masculine: derived, "~ibus";
        dative_feminine: derived, "~ibus";
        dative_neuter: derived, "~ibus";
        accusative_masculine: derived, "~īs|~ēs";
        accusative_feminine: derived, "~īs|~ēs";
        accusative_neuter: derived, "~a|~ia";
        ablative_masculine: derived, "~ibus";
        ablative_feminine: derived, "~ibus";
        ablative_neuter: derived, "~ibus";
        vocative_masculine: derived, "~ēs";
        vocative_feminine: derived, "~ēs";
        vocative_neuter: derived, "~a|~ia";
    ),
    // Complūra: 2 words (conplūra, complūra). Like Complūrēs, except in 10 cells: nominative_masculine sg, nominative_masculine pl, nominative_feminine pl, genitive_masculine pl, genitive_feminine pl, genitive_neuter pl, ….
    declension!("complūra", 2, "~a", like Complūrēs;
        nominative_masculine: "~a", "~a|~ia";
        nominative_feminine: _, "~a|~ia";
        nominative_neuter: _, _;
        genitive_masculine: _, "~ium";
        genitive_feminine: _, "~ium";
        genitive_neuter: _, "~ium";
        dative_masculine: _, _;
        dative_feminine: _, _;
        dative_neuter: _, _;
        accusative_masculine: _, "~a|~ia";
        accusative_feminine: _, "~a|~ia";
        accusative_neuter: _, _;
        ablative_masculine: _, _;
        ablative_feminine: _, _;
        ablative_neuter: _, _;
        vocative_masculine: _, "~a|~ia";
        vocative_feminine: _, "~a|~ia";
        vocative_neuter: _, _;
    ),
    // Is: 1 words (is).
    declension!("is", 1, "is";
        nominative_masculine: "is", "iī|eī|ī";
        nominative_feminine: "ea", "eae";
        nominative_neuter: "id", "ea";
        genitive_masculine: "eius|ejus", "eōrum|eum";
        genitive_feminine: "eius|ejus", "eārum";
        genitive_neuter: "eius|ejus", "eōrum|eum";
        dative_masculine: "eī|ēī", "iīs|eīs|īs|ībus";
        dative_feminine: "eī|ēī|eae", "iīs|eīs|īs|eābus";
        dative_neuter: "eī|ēī", "iīs|eīs|īs";
        accusative_masculine: "eum", "eōs";
        accusative_feminine: "eam", "eās";
        accusative_neuter: "id", "ea";
        ablative_masculine: "eō", "iīs|eīs|īs";
        ablative_feminine: "eā", "iīs|eīs|īs";
        ablative_neuter: "eō", "iīs|eīs|īs";
        vocative_masculine: derived, derived;
        vocative_feminine: derived, derived;
        vocative_neuter: derived, derived;
    ),
    // Sē: 1 words (sē).
    declension!("sē", 1, "~ē";
        nominative_masculine: "~ē", derived;
        nominative_feminine: derived, derived;
        nominative_neuter: derived, derived;
        genitive_masculine: "~uī", derived;
        genitive_feminine: "~uī", derived;
        genitive_neuter: "~uī", derived;
        dative_masculine: "~ibī", derived;
        dative_feminine: "~ibī", derived;
        dative_neuter: "~ibī", derived;
        accusative_masculine: "~ē|~ēsē", derived;
        accusative_feminine: "~ē|~ēsē", derived;
        accusative_neuter: "~ē|~ēsē", derived;
        ablative_masculine: "~ē|~ēsē", derived;
        ablative_feminine: "~ē|~ēsē", derived;
        ablative_neuter: "~ē|~ēsē", derived;
        vocative_masculine: derived, derived;
        vocative_feminine: derived, derived;
        vocative_neuter: derived, derived;
    ),
    // Ea: 1 words (ea).
    declension!("ea", 1, "ea";
        nominative_masculine: "ea", "eae";
        nominative_feminine: "ea", "eae";
        nominative_neuter: "ea", "eae";
        genitive_masculine: "eius|ejus", "eārum";
        genitive_feminine: "eius|ejus", "eārum";
        genitive_neuter: "eius|ejus", "eārum";
        dative_masculine: "eī|ēī|eae", "eīs|eābus|iīs|īs";
        dative_feminine: "eī|ēī|eae", "eīs|eābus|iīs|īs";
        dative_neuter: "eī|ēī|eae", "eīs|eābus|iīs|īs";
        accusative_masculine: "eam", "eās";
        accusative_feminine: "eam", "eās";
        accusative_neuter: "eam", "eās";
        ablative_masculine: "eā", "eīs|eābus|iīs|īs";
        ablative_feminine: "eā", "eīs|eābus|iīs|īs";
        ablative_neuter: "eā", "eīs|eābus|iīs|īs";
        vocative_masculine: "ea", "eae";
        vocative_feminine: "ea", "eae";
        vocative_neuter: "ea", "eae";
    ),
    // Nēmō: 1 words (nēmō). Like Sē, except in 15 cells: nominative_masculine sg, nominative_feminine sg, nominative_neuter sg, genitive_masculine sg, genitive_feminine sg, genitive_neuter sg, ….
    declension!("nēmō", 1, "~ēmō", like Sē;
        nominative_masculine: "~ēmō", _;
        nominative_feminine: "~ēmō", _;
        nominative_neuter: "~ēmō", _;
        genitive_masculine: "~ūllīus|~ēminis", _;
        genitive_feminine: "~ūllīus|~ēminis", _;
        genitive_neuter: "~ūllīus|~ēminis", _;
        dative_masculine: "~ēminī", _;
        dative_feminine: "~ēminī", _;
        dative_neuter: "~ēminī", _;
        accusative_masculine: "~ēminem", _;
        accusative_feminine: "~ēminem", _;
        accusative_neuter: "~ēminem", _;
        ablative_masculine: "~ūllō|~ēmine", _;
        ablative_feminine: "~ūllā|~ēmine", _;
        ablative_neuter: derived, _;
        vocative_masculine: _, _;
        vocative_feminine: _, _;
        vocative_neuter: _, _;
    ),
    // Id: 1 words (id).
    declension!("id", 1, "id";
        nominative_masculine: "id", "ea";
        nominative_feminine: "id", "ea";
        nominative_neuter: "id", "ea";
        genitive_masculine: "eius|ejus", "eōrum";
        genitive_feminine: "eius|ejus", "eōrum";
        genitive_neuter: "eius|ejus", "eōrum";
        dative_masculine: "eī|ēī", "eīs|iīs|īs";
        dative_feminine: "eī|ēī", "eīs|iīs|īs";
        dative_neuter: "eī|ēī", "eīs|iīs|īs";
        accusative_masculine: "id", "ea";
        accusative_feminine: "id", "ea";
        accusative_neuter: "id", "ea";
        ablative_masculine: "eō", "eīs|iīs|īs";
        ablative_feminine: "eō", "eīs|iīs|īs";
        ablative_neuter: "eō", "eīs|iīs|īs";
        vocative_masculine: "id", "ea";
        vocative_feminine: "id", "ea";
        vocative_neuter: "id", "ea";
    ),
    // Quī: 1 words (quī).
    declension!("quī", 1, "q~ī";
        nominative_masculine: "q~ī|q~ei", "q~ī|q~ei";
        nominative_feminine: "q~ae|q~ai", "q~ae";
        nominative_neuter: "q~od", "q~ae";
        genitive_masculine: "c~ius|q~oius", "q~ōrum";
        genitive_feminine: "c~ius|q~oius", "q~ārum";
        genitive_neuter: "c~ius|q~oius", "q~ōrum";
        dative_masculine: "c~i|q~oi|q~oī|q~oei|q~oiī|q~oiei", "q~ibus|q~īs|q~eis";
        dative_feminine: "c~i|q~oi|q~oī|q~oei|q~oiī|q~oiei", "q~ibus|q~īs|q~eis";
        dative_neuter: "c~i|q~oi|q~oī|q~oei|q~oiī|q~oiei", "q~ibus|q~īs|q~eis";
        accusative_masculine: "q~em", "q~ōs";
        accusative_feminine: "q~am", "q~ās";
        accusative_neuter: "q~od", "q~ae";
        ablative_masculine: "q~ō|q~ī", "q~ibus|q~īs|q~eis";
        ablative_feminine: "q~ā|q~ī", "q~ibus|q~īs|q~eis";
        ablative_neuter: "q~ō|q~ī", "q~ibus|q~īs|q~eis";
        vocative_masculine: derived, derived;
        vocative_feminine: derived, derived;
        vocative_neuter: derived, derived;
    ),
    // Quīdam: 1 words (quīdam).
    declension!("quīdam", 1, "q~ī~~";
        nominative_masculine: "q~ī~~", "q~ī~~";
        nominative_feminine: "q~ae~~", "q~ae~~";
        nominative_neuter: "q~i~d~", "q~ae~~";
        genitive_masculine: "c~ius~~", "q~ōrun~~|q~ōrum~~";
        genitive_feminine: "c~ius~~", "q~ārun~~|q~ārum~~";
        genitive_neuter: "c~ius~~", "q~ōrun~~|q~ōrum~~";
        dative_masculine: "c~i~~", "q~ibus~~";
        dative_feminine: "c~i~~", "q~ibus~~";
        dative_neuter: "c~i~~", "q~ibus~~";
        accusative_masculine: "q~en~~|q~em~~", "q~ōs~~";
        accusative_feminine: "q~an~~|q~am~~", "q~ās~~";
        accusative_neuter: "q~i~d~", "q~ae~~";
        ablative_masculine: "q~ō~~", "q~ibus~~";
        ablative_feminine: "q~ā~~", "q~ibus~~";
        ablative_neuter: "q~ō~~", "q~ibus~~";
        vocative_masculine: derived, derived;
        vocative_feminine: derived, derived;
        vocative_neuter: derived, derived;
    ),
    // Ecquī: 1 words (ecquī).
    declension!("ecquī", 1, "~q~ī";
        nominative_masculine: "~q~ī", "~q~ī";
        nominative_feminine: "~q~a", "~q~ae";
        nominative_neuter: "~q~od", "~q~a";
        genitive_masculine: "~c~ius", "~q~ōrum";
        genitive_feminine: "~c~ius", "~q~ārum";
        genitive_neuter: "~c~ius", "~q~ōrum";
        dative_masculine: "~c~i", "~q~īs";
        dative_feminine: "~c~i", "~q~īs";
        dative_neuter: "~c~i", "~q~īs";
        accusative_masculine: "~q~em", "~q~ōs";
        accusative_feminine: "~q~am", "~q~ās";
        accusative_neuter: "~q~od", "~q~a";
        ablative_masculine: "~q~ō", "~q~īs";
        ablative_feminine: "~q~ā", "~q~īs";
        ablative_neuter: "~q~ō", "~q~īs";
        vocative_masculine: derived, derived;
        vocative_feminine: derived, derived;
        vocative_neuter: derived, derived;
    ),
    // Suī: 1 words (suī). Like Sē, except in 1 cell: nominative_masculine sg.
    declension!("suī", 1, "~uī", like Sē;
        nominative_masculine: "~uī", _;
        nominative_feminine: _, _;
        nominative_neuter: _, _;
        genitive_masculine: _, _;
        genitive_feminine: _, _;
        genitive_neuter: _, _;
        dative_masculine: _, _;
        dative_feminine: _, _;
        dative_neuter: _, _;
        accusative_masculine: _, _;
        accusative_feminine: _, _;
        accusative_neuter: _, _;
        ablative_masculine: _, _;
        ablative_feminine: _, _;
        ablative_neuter: _, _;
        vocative_masculine: _, _;
        vocative_feminine: _, _;
        vocative_neuter: _, _;
    ),
    // Ambō: 1 words (ambō).
    declension!("ambō", 1, "~ō";
        nominative_masculine: "~ō", "~ō";
        nominative_feminine: derived, "~ae";
        nominative_neuter: derived, "~ō";
        genitive_masculine: derived, "~ōrum";
        genitive_feminine: derived, "~ārum";
        genitive_neuter: derived, "~ōrum";
        dative_masculine: derived, "~ōbus";
        dative_feminine: derived, "~ābus";
        dative_neuter: derived, "~ōbus";
        accusative_masculine: derived, "~ōs|~ō";
        accusative_feminine: derived, "~ās";
        accusative_neuter: derived, "~ō";
        ablative_masculine: derived, "~ōbus";
        ablative_feminine: derived, "~ābus";
        ablative_neuter: derived, "~ōbus";
        vocative_masculine: derived, "~ō";
        vocative_feminine: derived, "~ae";
        vocative_neuter: derived, "~ō";
    ),
    // Sēsē: 1 words (sēsē). Like Sē, except in 1 cell: nominative_masculine sg.
    declension!("sēsē", 1, "~ēsē", like Sē;
        nominative_masculine: "~ēsē", _;
        nominative_feminine: _, _;
        nominative_neuter: _, _;
        genitive_masculine: _, _;
        genitive_feminine: _, _;
        genitive_neuter: _, _;
        dative_masculine: _, _;
        dative_feminine: _, _;
        dative_neuter: _, _;
        accusative_masculine: _, _;
        accusative_feminine: _, _;
        accusative_neuter: _, _;
        ablative_masculine: _, _;
        ablative_feminine: _, _;
        ablative_neuter: _, _;
        vocative_masculine: _, _;
        vocative_feminine: _, _;
        vocative_neuter: _, _;
    ),
    // Quīnam: 1 words (quīnam).
    declension!("quīnam", 1, "q~īnam";
        nominative_masculine: "q~īnam", "q~īnam|q~einam";
        nominative_feminine: "q~aenam", "q~aenam";
        nominative_neuter: "q~odnam", "q~aenam";
        genitive_masculine: "c~iusnam|q~oiusnam", "q~ōrumnam";
        genitive_feminine: "c~iusnam|q~oiusnam", "q~ārumnam";
        genitive_neuter: "c~iusnam|q~oiusnam", "q~ōrumnam";
        dative_masculine: "c~inam|q~oinam|q~oī|q~oeinam|q~oiī|q~oieinam", "q~ibusnam";
        dative_feminine: "c~inam|q~oinam|q~oī|q~oeinam|q~oiī|q~oieinam", "q~ibusnam";
        dative_neuter: "c~inam|q~oinam|q~oī|q~oeinam|q~oiī|q~oieinam", "q~ibusnam";
        accusative_masculine: "q~emnam", "q~ōsnam";
        accusative_feminine: "q~amnam", "q~āsnam";
        accusative_neuter: "q~odnam", "q~aenam";
        ablative_masculine: "q~ōnam|q~īnam", "q~ibusnam";
        ablative_feminine: "q~ānam|q~īnam", "q~ibusnam";
        ablative_neuter: "q~ōnam|q~īnam", "q~ibusnam";
        vocative_masculine: derived, derived;
        vocative_feminine: derived, derived;
        vocative_neuter: derived, derived;
    ),
    // Quīvīs: 1 words (quīvīs).
    declension!("quīvīs", 1, "q~ī~";
        nominative_masculine: "q~ī~", "q~ī~|q~ei~";
        nominative_feminine: "q~ae~", "q~ae~";
        nominative_neuter: "q~id~", "q~ae~";
        genitive_masculine: "c~ius~|q~oius~", "q~ōrun~|q~ōrum~";
        genitive_feminine: "c~ius~|q~oius~", "q~ārun~|q~ārum~";
        genitive_neuter: "c~ius~|q~oius~", "q~ōrun~|q~ōrum~";
        dative_masculine: "c~i~|q~oi~", "q~ibus~|q~īs~|q~eis~";
        dative_feminine: "c~i~|q~oi~", "q~ibus~|q~īs~|q~eis~";
        dative_neuter: "c~i~|q~oi~", "q~ibus~|q~īs~|q~eis~";
        accusative_masculine: "q~en~|q~em~", "q~ōs~";
        accusative_feminine: "q~an~|q~am~", "q~ās~";
        accusative_neuter: "q~id~", "q~ae~";
        ablative_masculine: "q~ō~|q~ī~", "q~ibus~|q~īs~|q~eis~";
        ablative_feminine: "q~ā~|q~ī~", "q~ibus~|q~īs~|q~eis~";
        ablative_neuter: "q~ō~", "q~ibus~|q~īs~|q~eis~";
        vocative_masculine: derived, derived;
        vocative_feminine: derived, derived;
        vocative_neuter: derived, derived;
    ),
    // Complūsculī: 1 words (complūsculī). Like Ambō, except in 13 cells: nominative_masculine sg, nominative_masculine pl, nominative_neuter pl, dative_masculine pl, dative_feminine pl, dative_neuter pl, ….
    declension!("complūsculī", 1, "~ī", like Ambō;
        nominative_masculine: "~ī", "~ī";
        nominative_feminine: _, _;
        nominative_neuter: _, "~a";
        genitive_masculine: _, _;
        genitive_feminine: _, _;
        genitive_neuter: _, _;
        dative_masculine: _, "~īs";
        dative_feminine: _, "~īs";
        dative_neuter: _, "~īs";
        accusative_masculine: _, "~ōs";
        accusative_feminine: _, _;
        accusative_neuter: _, "~a";
        ablative_masculine: _, "~īs";
        ablative_feminine: _, "~īs";
        ablative_neuter: _, "~īs";
        vocative_masculine: _, "~ī";
        vocative_feminine: _, _;
        vocative_neuter: _, "~a";
    ),
    // Ecquīnam: 1 words (ecquīnam).
    declension!("ecquīnam", 1, "ecq~īnam";
        nominative_masculine: "ecq~īnam|ecq~einam|ecq~isnam", "ecq~īnam|ecq~einam";
        nominative_feminine: "ecq~aenam", "ecq~aenam";
        nominative_neuter: "ecq~odnam", "ecq~aenam";
        genitive_masculine: derived, derived;
        genitive_feminine: derived, derived;
        genitive_neuter: derived, derived;
        dative_masculine: "ecc~inam|ecq~oinam|ecq~oī|q~oeinam|ecq~oiī|q~oieinam", derived;
        dative_feminine: "ecc~inam|ecq~oinam|ecq~oī|q~oeinam|ecq~oiī|q~oieinam", derived;
        dative_neuter: "ecc~inam|ecq~oinam|ecq~oī|q~oeinam|ecq~oiī|q~oieinam", derived;
        accusative_masculine: "ecq~emnam", "ecq~ōsnam";
        accusative_feminine: "ecq~amnam", "ecq~āsnam";
        accusative_neuter: "ecq~odnam", "ecq~aenam";
        ablative_masculine: "ecq~ōnam|ecq~īnam", derived;
        ablative_feminine: "ecq~ānam|ecq~īnam", derived;
        ablative_neuter: "ecq~ōnam|ecq~īnam", derived;
        vocative_masculine: derived, derived;
        vocative_feminine: derived, derived;
        vocative_neuter: derived, derived;
    ),
];
