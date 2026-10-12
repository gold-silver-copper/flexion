//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's feature says):
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
    ($name:literal, $words:literal, $lemma:literal;
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
            forms: ::std::borrow::Cow::Borrowed(&[
                flexion::form!($c0),
                flexion::form!($c1),
                flexion::form!($c2),
                flexion::form!($c3),
                flexion::form!($c4),
                flexion::form!($c5),
                flexion::form!($c6),
                flexion::form!($c7),
                flexion::form!($c8),
                flexion::form!($c9),
                flexion::form!($c10),
                flexion::form!($c11),
                flexion::form!($c12),
                flexion::form!($c13),
                flexion::form!($c14),
                flexion::form!($c15),
                flexion::form!($c16),
                flexion::form!($c17),
                flexion::form!($c18),
                flexion::form!($c19),
                flexion::form!($c20),
                flexion::form!($c21),
                flexion::form!($c22),
                flexion::form!($c23),
                flexion::form!($c24),
                flexion::form!($c25),
                flexion::form!($c26),
                flexion::form!($c27),
                flexion::form!($c28),
                flexion::form!($c29),
                flexion::form!($c30),
                flexion::form!($c31),
                flexion::form!($c32),
                flexion::form!($c33),
                flexion::form!($c34),
                flexion::form!($c35),
            ]),
        }
    };
}

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `quis`: 24 words.
    Quis,
    /// Like `uter`: 15 words.
    Uter,
    /// Like `ipse`: 12 words.
    Ipse,
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
        Declension::Uter,
        Declension::Ipse,
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
    // Quis: 24 words (hic, quis, aliquī, hice, quisquis, quidquid).
    declension!("quis", 24, "~i~";
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
        vocative_masculine: "~e~", "~~ī";
        vocative_feminine: "~~a", "~~ae";
        vocative_neuter: "~~um", "~~a";
    ),
    // Uter: 15 words (alter, noster, īdem, uter, vester, egomet).
    declension!("uter", 15, "~e~";
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
        vocative_masculine: "~e", "~ī";
        vocative_feminine: "~a", "~ae";
        vocative_neuter: "~um", "~a";
    ),
    // Alius: 8 words (alius, tuus, meus, suus, ūllus, cuius).
    declension!("alius", 8, "~us";
        nominative_masculine: "~us", "~ī";
        nominative_feminine: "~a", "~ae";
        nominative_neuter: "~um", "~a";
        genitive_masculine: "~ī", "~ōrum";
        genitive_feminine: "~ae", "~ārum";
        genitive_neuter: "~ī", "~ōrum";
        dative_masculine: "~ō", "~īs";
        dative_feminine: "~ae", "~īs";
        dative_neuter: "~ō", "~īs";
        accusative_masculine: "~um", "~ōs";
        accusative_feminine: "~am", "~ās";
        accusative_neuter: "~um", "~a";
        ablative_masculine: "~ō", "~īs";
        ablative_feminine: "~ā", "~īs";
        ablative_neuter: "~ō", "~īs";
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
        nominative_masculine: "~us", "~ēs";
        nominative_feminine: "~a", "~ēs";
        nominative_neuter: "~um", "~a|~ia";
        genitive_masculine: "~ī", "~ium|~um";
        genitive_feminine: "~ae", "~ium|~um";
        genitive_neuter: "~ī", "~ium|~um";
        dative_masculine: "~ō", "~ibus";
        dative_feminine: "~ae", "~ibus";
        dative_neuter: "~ō", "~ibus";
        accusative_masculine: "~um", "~īs|~ēs";
        accusative_feminine: "~am", "~īs|~ēs";
        accusative_neuter: "~um", "~a|~ia";
        ablative_masculine: "~ō", "~ibus";
        ablative_feminine: "~ā", "~ibus";
        ablative_neuter: "~ō", "~ibus";
        vocative_masculine: "~e", "~ēs";
        vocative_feminine: "~a", "~ēs";
        vocative_neuter: "~um", "~a|~ia";
    ),
    // Complūra: 2 words (conplūra, complūra).
    declension!("complūra", 2, "~a";
        nominative_masculine: "~us", "~a|~ia";
        nominative_feminine: "~a", "~a|~ia";
        nominative_neuter: "~um", "~a|~ia";
        genitive_masculine: "~ī", "~ium";
        genitive_feminine: "~ae", "~ium";
        genitive_neuter: "~ī", "~ium";
        dative_masculine: "~ō", "~ibus";
        dative_feminine: "~ae", "~ibus";
        dative_neuter: "~ō", "~ibus";
        accusative_masculine: "~um", "~a|~ia";
        accusative_feminine: "~am", "~a|~ia";
        accusative_neuter: "~um", "~a|~ia";
        ablative_masculine: "~ō", "~ibus";
        ablative_feminine: "~ā", "~ibus";
        ablative_neuter: "~ō", "~ibus";
        vocative_masculine: "~e", "~a|~ia";
        vocative_feminine: "~a", "~a|~ia";
        vocative_neuter: "~um", "~a|~ia";
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
        vocative_masculine: "id", "ea";
        vocative_feminine: "id", "ea";
        vocative_neuter: "id", "ea";
    ),
    // Sē: 1 words (sē).
    declension!("sē", 1, "~ē";
        nominative_masculine: "~e", "~ī";
        nominative_feminine: "~a", "~ae";
        nominative_neuter: "~ud", "~a";
        genitive_masculine: "~uī", "~ōrum";
        genitive_feminine: "~uī", "~ārum";
        genitive_neuter: "~uī", "~ōrum";
        dative_masculine: "~ibī", "~īs";
        dative_feminine: "~ibī", "~īs";
        dative_neuter: "~ibī", "~īs";
        accusative_masculine: "~ē|~ēsē", "~ōs";
        accusative_feminine: "~ē|~ēsē", "~ās";
        accusative_neuter: "~ē|~ēsē", "~a";
        ablative_masculine: "~ē|~ēsē", "~īs";
        ablative_feminine: "~ē|~ēsē", "~īs";
        ablative_neuter: "~ē|~ēsē", "~īs";
        vocative_masculine: "~e", "~ī";
        vocative_feminine: "~a", "~ae";
        vocative_neuter: "~um", "~a";
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
    // Nēmō: 1 words (nēmō).
    declension!("nēmō", 1, "~ēmō";
        nominative_masculine: "~ēmō", "~ī";
        nominative_feminine: "~ēmō", "~ae";
        nominative_neuter: "~ēmō", "~a";
        genitive_masculine: "~ūllīus|~ēminis", "~ōrum";
        genitive_feminine: "~ūllīus|~ēminis", "~ārum";
        genitive_neuter: "~ūllīus|~ēminis", "~ōrum";
        dative_masculine: "~ēminī", "~īs";
        dative_feminine: "~ēminī", "~īs";
        dative_neuter: "~ēminī", "~īs";
        accusative_masculine: "~ēminem", "~ōs";
        accusative_feminine: "~ēminem", "~ās";
        accusative_neuter: "~ēminem", "~a";
        ablative_masculine: "~ūllō|~ēmine", "~īs";
        ablative_feminine: "~ūllā|~ēmine", "~īs";
        ablative_neuter: "~ō", "~īs";
        vocative_masculine: "~e", "~ī";
        vocative_feminine: "~a", "~ae";
        vocative_neuter: "~um", "~a";
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
        vocative_masculine: "~e", "~ī";
        vocative_feminine: "~a", "~ae";
        vocative_neuter: "~um", "~a";
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
        vocative_masculine: "~e~~", "~~ī~";
        vocative_feminine: "~~a~", "~~ae~";
        vocative_neuter: "~~um~", "~~a~";
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
        vocative_masculine: "~e~", "~~ī";
        vocative_feminine: "~~a", "~~ae";
        vocative_neuter: "~~um", "~~a";
    ),
    // Suī: 1 words (suī).
    declension!("suī", 1, "~uī";
        nominative_masculine: "~e", "~ī";
        nominative_feminine: "~a", "~ae";
        nominative_neuter: "~ud", "~a";
        genitive_masculine: "~uī", "~ōrum";
        genitive_feminine: "~uī", "~ārum";
        genitive_neuter: "~uī", "~ōrum";
        dative_masculine: "~ibī", "~īs";
        dative_feminine: "~ibī", "~īs";
        dative_neuter: "~ibī", "~īs";
        accusative_masculine: "~ē|~ēsē", "~ōs";
        accusative_feminine: "~ē|~ēsē", "~ās";
        accusative_neuter: "~ē|~ēsē", "~a";
        ablative_masculine: "~ē|~ēsē", "~īs";
        ablative_feminine: "~ē|~ēsē", "~īs";
        ablative_neuter: "~ē|~ēsē", "~īs";
        vocative_masculine: "~e", "~ī";
        vocative_feminine: "~a", "~ae";
        vocative_neuter: "~um", "~a";
    ),
    // Ambō: 1 words (ambō).
    declension!("ambō", 1, "~ō";
        nominative_masculine: "~us", "~ō";
        nominative_feminine: "~a", "~ae";
        nominative_neuter: "~um", "~ō";
        genitive_masculine: "~ī", "~ōrum";
        genitive_feminine: "~ae", "~ārum";
        genitive_neuter: "~ī", "~ōrum";
        dative_masculine: "~ō", "~ōbus";
        dative_feminine: "~ae", "~ābus";
        dative_neuter: "~ō", "~ōbus";
        accusative_masculine: "~um", "~ōs|~ō";
        accusative_feminine: "~am", "~ās";
        accusative_neuter: "~um", "~ō";
        ablative_masculine: "~ō", "~ōbus";
        ablative_feminine: "~ā", "~ābus";
        ablative_neuter: "~ō", "~ōbus";
        vocative_masculine: "~e", "~ō";
        vocative_feminine: "~a", "~ae";
        vocative_neuter: "~um", "~ō";
    ),
    // Sēsē: 1 words (sēsē).
    declension!("sēsē", 1, "~ēsē";
        nominative_masculine: "~e", "~ī";
        nominative_feminine: "~a", "~ae";
        nominative_neuter: "~ud", "~a";
        genitive_masculine: "~uī", "~ōrum";
        genitive_feminine: "~uī", "~ārum";
        genitive_neuter: "~uī", "~ōrum";
        dative_masculine: "~ibī", "~īs";
        dative_feminine: "~ibī", "~īs";
        dative_neuter: "~ibī", "~īs";
        accusative_masculine: "~ē|~ēsē", "~ōs";
        accusative_feminine: "~ē|~ēsē", "~ās";
        accusative_neuter: "~ē|~ēsē", "~a";
        ablative_masculine: "~ē|~ēsē", "~īs";
        ablative_feminine: "~ē|~ēsē", "~īs";
        ablative_neuter: "~ē|~ēsē", "~īs";
        vocative_masculine: "~e", "~ī";
        vocative_feminine: "~a", "~ae";
        vocative_neuter: "~um", "~a";
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
        vocative_masculine: "~e", "~ī";
        vocative_feminine: "~a", "~ae";
        vocative_neuter: "~um", "~a";
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
        vocative_masculine: "~e~", "~~ī";
        vocative_feminine: "~~a", "~~ae";
        vocative_neuter: "~~um", "~~a";
    ),
    // Complūsculī: 1 words (complūsculī).
    declension!("complūsculī", 1, "~ī";
        nominative_masculine: "~us", "~ī";
        nominative_feminine: "~a", "~ae";
        nominative_neuter: "~um", "~a";
        genitive_masculine: "~ī", "~ōrum";
        genitive_feminine: "~ae", "~ārum";
        genitive_neuter: "~ī", "~ōrum";
        dative_masculine: "~ō", "~īs";
        dative_feminine: "~ae", "~īs";
        dative_neuter: "~ō", "~īs";
        accusative_masculine: "~um", "~ōs";
        accusative_feminine: "~am", "~ās";
        accusative_neuter: "~um", "~a";
        ablative_masculine: "~ō", "~īs";
        ablative_feminine: "~ā", "~īs";
        ablative_neuter: "~ō", "~īs";
        vocative_masculine: "~e", "~ī";
        vocative_feminine: "~a", "~ae";
        vocative_neuter: "~um", "~a";
    ),
    // Ecquīnam: 1 words (ecquīnam).
    declension!("ecquīnam", 1, "ecq~īnam";
        nominative_masculine: "ecq~īnam|ecq~einam|ecq~isnam", "ecq~īnam|ecq~einam";
        nominative_feminine: "ecq~aenam", "ecq~aenam";
        nominative_neuter: "ecq~odnam", "ecq~aenam";
        genitive_masculine: "~īus", "~ōrum";
        genitive_feminine: "~īus", "~ārum";
        genitive_neuter: "~īus", "~ōrum";
        dative_masculine: "ecc~inam|ecq~oinam|ecq~oī|q~oeinam|ecq~oiī|q~oieinam", "~īs";
        dative_feminine: "ecc~inam|ecq~oinam|ecq~oī|q~oeinam|ecq~oiī|q~oieinam", "~īs";
        dative_neuter: "ecc~inam|ecq~oinam|ecq~oī|q~oeinam|ecq~oiī|q~oieinam", "~īs";
        accusative_masculine: "ecq~emnam", "ecq~ōsnam";
        accusative_feminine: "ecq~amnam", "ecq~āsnam";
        accusative_neuter: "ecq~odnam", "ecq~aenam";
        ablative_masculine: "ecq~ōnam|ecq~īnam", "~īs";
        ablative_feminine: "ecq~ānam|ecq~īnam", "~īs";
        ablative_neuter: "ecq~ōnam|ecq~īnam", "~īs";
        vocative_masculine: "~e", "~ī";
        vocative_feminine: "~a", "~ae";
        vocative_neuter: "~um", "~a";
    ),
];
