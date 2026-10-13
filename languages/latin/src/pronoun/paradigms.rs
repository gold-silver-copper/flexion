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
/// they share them), the declension it is like (if any), then a row of patterns
/// per line (`derived`: the form comes from another cell's, see
/// `relations.rs`; `_`, or a row left out: the form is the declension's it is like,
/// or, in a declension like no other, derived):
/// `nom_m` (sg, pl)
/// `nom_f` (sg, pl)
/// `nom_n` (sg, pl)
/// `gen_m` (sg, pl)
/// `gen_f` (sg, pl)
/// `gen_n` (sg, pl)
/// `dat_m` (sg, pl)
/// `dat_f` (sg, pl)
/// `dat_n` (sg, pl)
/// `acc_m` (sg, pl)
/// `acc_f` (sg, pl)
/// `acc_n` (sg, pl)
/// `abl_m` (sg, pl)
/// `abl_f` (sg, pl)
/// `abl_n` (sg, pl)
/// `voc_m` (sg, pl)
/// `voc_f` (sg, pl)
/// `voc_n` (sg, pl)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident)?;
     $(nom_m: $c0:tt, $c1:tt;)?
     $(nom_f: $c2:tt, $c3:tt;)?
     $(nom_n: $c4:tt, $c5:tt;)?
     $(gen_m: $c6:tt, $c7:tt;)?
     $(gen_f: $c8:tt, $c9:tt;)?
     $(gen_n: $c10:tt, $c11:tt;)?
     $(dat_m: $c12:tt, $c13:tt;)?
     $(dat_f: $c14:tt, $c15:tt;)?
     $(dat_n: $c16:tt, $c17:tt;)?
     $(acc_m: $c18:tt, $c19:tt;)?
     $(acc_f: $c20:tt, $c21:tt;)?
     $(acc_n: $c22:tt, $c23:tt;)?
     $(abl_m: $c24:tt, $c25:tt;)?
     $(abl_f: $c26:tt, $c27:tt;)?
     $(abl_n: $c28:tt, $c29:tt;)?
     $(voc_m: $c30:tt, $c31:tt;)?
     $(voc_f: $c32:tt, $c33:tt;)?
     $(voc_n: $c34:tt, $c35:tt;)?
    ) => {
        Paradigm {
            name: ::std::borrow::Cow::Borrowed($name),
            words: $words,
            lemma: ::std::borrow::Cow::Borrowed($lemma),
            parent: like!($($parent)?),
            inherent: ::std::borrow::Cow::Borrowed(&[]),
            table: ::std::borrow::Cow::Borrowed(concat!(
                $(flexion::cell!($c0),)? "\t",
                $(flexion::cell!($c1),)? "\t",
                $(flexion::cell!($c2),)? "\t",
                $(flexion::cell!($c3),)? "\t",
                $(flexion::cell!($c4),)? "\t",
                $(flexion::cell!($c5),)? "\t",
                $(flexion::cell!($c6),)? "\t",
                $(flexion::cell!($c7),)? "\t",
                $(flexion::cell!($c8),)? "\t",
                $(flexion::cell!($c9),)? "\t",
                $(flexion::cell!($c10),)? "\t",
                $(flexion::cell!($c11),)? "\t",
                $(flexion::cell!($c12),)? "\t",
                $(flexion::cell!($c13),)? "\t",
                $(flexion::cell!($c14),)? "\t",
                $(flexion::cell!($c15),)? "\t",
                $(flexion::cell!($c16),)? "\t",
                $(flexion::cell!($c17),)? "\t",
                $(flexion::cell!($c18),)? "\t",
                $(flexion::cell!($c19),)? "\t",
                $(flexion::cell!($c20),)? "\t",
                $(flexion::cell!($c21),)? "\t",
                $(flexion::cell!($c22),)? "\t",
                $(flexion::cell!($c23),)? "\t",
                $(flexion::cell!($c24),)? "\t",
                $(flexion::cell!($c25),)? "\t",
                $(flexion::cell!($c26),)? "\t",
                $(flexion::cell!($c27),)? "\t",
                $(flexion::cell!($c28),)? "\t",
                $(flexion::cell!($c29),)? "\t",
                $(flexion::cell!($c30),)? "\t",
                $(flexion::cell!($c31),)? "\t",
                $(flexion::cell!($c32),)? "\t",
                $(flexion::cell!($c33),)? "\t",
                $(flexion::cell!($c34),)? "\t",
                $(flexion::cell!($c35),)? "\t",
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
    /// Like `quis`: 30 words.
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
    /// Like `nēmō`: 1 words.
    Nēmō,
    /// Like `quī`: 1 words.
    Quī,
    /// Like `ecquī`: 1 words.
    Ecquī,
    /// Like `suī`: 1 words.
    Suī,
    /// Like `ambŏ`: 1 words.
    Ambŏ,
    /// Like `complūsculī`: 1 words.
    Complūsculī,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 14] = [
        Declension::Quis,
        Declension::Ipse,
        Declension::Uter,
        Declension::Alius,
        Declension::Utervīs,
        Declension::Complūrēs,
        Declension::Complūra,
        Declension::Is,
        Declension::Nēmō,
        Declension::Quī,
        Declension::Ecquī,
        Declension::Suī,
        Declension::Ambŏ,
        Declension::Complūsculī,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 14] = [
    // Quis: 30 words (hic, quis, quīdam, aliquī, hice, quisquis).
    declension!("quis", 30, "~i~";
    nom_m: "~i~", "~īs~e";
    nom_f: "~ae~", "~ae~";
    nom_n: "~u~", "~ae~";
    gen_m: "~ĭus~e", "~ōrun~";
    gen_f: "~ĭus~e", "~ārun~";
    dat_m: "~ī~", "~īs~e";
    dat_f: "~ī~", _;
    acc_m: "~un~", "~ōs~e";
    acc_f: "~an~", "~ās~e";
    acc_n: "~u~", _;
    abl_m: "~ō~", _;
    abl_f: "~ā~", _;
    ),
    // Ipse: 12 words (ille, iste, ipse, quisque, ūnusquisque, quīcumque).
    declension!("ipse", 12, "~e";
    nom_m: "~e", "~ī";
    nom_f: "~a", "~ae";
    nom_n: "~ud", "~a";
    gen_m: "~ĭus", "~ōrum";
    gen_f: "~ĭus", "~ārum";
    dat_m: "~ī", "~īs";
    dat_f: "~ī", _;
    acc_m: "~um", "~ōs";
    acc_f: "~am", "~ās";
    acc_n: "~ud", _;
    abl_m: "~ō", _;
    abl_f: "~ā", _;
    ),
    // Uter: 12 words (alter, noster, īdem, uter, vester, egomet).
    declension!("uter", 12, "~e~";
    nom_m: "~e~", "~~ī";
    nom_f: "~~a", "~~ae";
    nom_n: "~~um", "~~a";
    gen_m: "~~ī", "~~ōrum";
    gen_f: "~~ae", "~~ārum";
    dat_m: "~~ō", "~~īs";
    dat_f: "~~ae", _;
    acc_m: "~~um", "~~ōs";
    acc_f: "~~am", "~~ās";
    acc_n: "~~um", _;
    abl_m: "~~ō", _;
    abl_f: "~~ā", _;
    voc_m: "~e~", _;
    ),
    // Alius: 8 words (alius, tuus, meus, suus, ūllus, cuius). Like Ipse, except in 8 cells: nom_m sg, nom_n sg, gen_m sg, gen_f sg, dat_m sg, dat_f sg, ….
    declension!("alius", 8, "~us", like Ipse;
    nom_m: "~us", _;
    nom_n: "~um", _;
    gen_m: "~ī", _;
    gen_f: "~ae", _;
    dat_m: "~ō", _;
    dat_f: "~ae", _;
    acc_n: "~um", _;
    voc_m: "~e", _;
    ),
    // Utervīs: 8 words (tūtemet, temet, uterque, ipsemet, utervīs, utercumque).
    declension!("utervīs", 8, "~e~~";
    nom_m: "~e~~", "~~ī~";
    nom_f: "~~a~", "~~ae~";
    nom_n: "~~um~", "~~a~";
    gen_m: "~~ĭus~", "~~ōrum~";
    gen_f: "~~ĭus~", "~~ārum~";
    dat_m: "~~ī~", "~~īs~";
    dat_f: "~~ī~", _;
    acc_m: "~~um~", "~~ōs~";
    acc_f: "~~am~", "~~ās~";
    acc_n: "~~um~", _;
    abl_m: "~~ō~", _;
    abl_f: "~~ā~", _;
    voc_m: "~e~~", _;
    ),
    // Complūrēs: 2 words (complūrēs, conplūrēs).
    declension!("complūrēs", 2, "~ēs";
    nom_m: "~ēs", "~ēs";
    nom_f: _, "~ēs";
    nom_n: _, "~a|~ia";
    gen_m: _, "~ium|~um";
    gen_f: _, "~ium|~um";
    gen_n: _, "~ium|~um";
    dat_m: _, "~ibus";
    acc_m: _, "~īs|~ēs";
    acc_f: _, "~īs|~ēs";
    acc_n: _, "~a|~ia";
    voc_n: _, "~a|~ia";
    ),
    // Complūra: 2 words (conplūra, complūra). Like Complūrēs, except in 10 cells: nom_m sg, nom_m pl, nom_f pl, gen_m pl, gen_f pl, gen_n pl, ….
    declension!("complūra", 2, "~a", like Complūrēs;
    nom_m: "~a", "~a|~ia";
    nom_f: _, "~a|~ia";
    gen_m: _, "~ium";
    gen_f: _, "~ium";
    gen_n: _, derived;
    acc_m: _, "~a|~ia";
    acc_f: _, "~a|~ia";
    voc_m: _, "~a|~ia";
    voc_f: _, "~a|~ia";
    ),
    // Is: 1 words (is).
    declension!("is", 1, "is";
    nom_m: "is", "iī|eī|ī";
    nom_f: "ea", "eae";
    nom_n: "id", "ea";
    gen_m: "eius|ejus", "eōrum|eum";
    gen_f: "eius|ejus", "eārum";
    gen_n: "eius|ejus", "eōrum|eum";
    dat_m: "eī|ēī", "iīs|eīs|īs|ībus";
    dat_f: "eī|ēī|eae", "iīs|eīs|īs|eābus";
    dat_n: "eī|ēī", "iīs|eīs|īs";
    acc_m: "eum", "eōs";
    acc_f: "eam", "eās";
    acc_n: "id", _;
    abl_m: "eō", "iīs|eīs|īs";
    abl_f: "eā", "iīs|eīs|īs";
    abl_n: _, "iīs|eīs|īs";
    ),
    // Nēmō: 1 words (nēmō).
    declension!("nēmō", 1, "~ēmō";
    nom_m: "~ēmō", _;
    nom_f: "~ēmō", _;
    nom_n: "~ēmō", _;
    gen_m: "~ūllĭus|~ēminis", _;
    gen_f: "~ūllĭus|~ēminis", _;
    gen_n: "~ūllĭus|~ēminis", _;
    dat_m: "~ēminī", _;
    dat_f: "~ēminī", _;
    acc_m: "~ēminem", _;
    acc_f: "~ēminem", _;
    acc_n: "~ēminem", _;
    abl_m: "~ūllō|~ēmine", _;
    abl_f: "~ūllā|~ēmine", _;
    ),
    // Quī: 1 words (quī).
    declension!("quī", 1, "q~ī";
    nom_m: "q~ī|q~ei", "q~ī|q~ei";
    nom_f: "q~ae|q~ai", "q~ae";
    nom_n: "q~od", "q~ae";
    gen_m: "c~ius|q~oius", "q~ōrum";
    gen_f: "c~ius|q~oius", "q~ārum";
    gen_n: "c~ius|q~oius", _;
    dat_m: "c~i|q~oi|q~oī|q~oei|q~oiī|q~oiei", "q~ibus|q~īs|q~eis";
    dat_f: "c~i|q~oi|q~oī|q~oei|q~oiī|q~oiei", "q~ibus|q~īs|q~eis";
    dat_n: "c~i|q~oi|q~oī|q~oei|q~oiī|q~oiei", "q~ibus|q~īs|q~eis";
    acc_m: "q~em", "q~ōs";
    acc_f: "q~am", "q~ās";
    acc_n: "q~od", _;
    abl_m: "q~ō|q~ī", "q~ibus|q~īs|q~eis";
    abl_f: "q~ā|q~ī", "q~ibus|q~īs|q~eis";
    abl_n: "q~ō|q~ī", "q~ibus|q~īs|q~eis";
    ),
    // Ecquī: 1 words (ecquī).
    declension!("ecquī", 1, "~q~ī";
    nom_m: "~q~ī", "~q~ī";
    nom_f: "~q~a", "~q~ae";
    nom_n: "~q~od", "~q~a";
    gen_m: "~c~ius", "~q~ōrum";
    gen_f: "~c~ius", "~q~ārum";
    dat_m: "~c~i", "~q~īs";
    dat_f: "~c~i", _;
    acc_m: "~q~em", "~q~ōs";
    acc_f: "~q~am", "~q~ās";
    acc_n: "~q~od", _;
    abl_m: "~q~ō", _;
    abl_f: "~q~ā", _;
    ),
    // Suī: 1 words (suī). Like Nēmō, except in 14 cells: nom_m sg, nom_f sg, nom_n sg, gen_m sg, gen_f sg, gen_n sg, ….
    declension!("suī", 1, "~uī", like Nēmō;
    nom_m: "~uī", _;
    nom_f: derived, _;
    nom_n: derived, _;
    gen_m: "~uī", _;
    gen_f: "~uī", _;
    gen_n: derived, _;
    dat_m: "~ibĭ", _;
    dat_f: "~ibĭ", _;
    acc_m: "~ē|~ēsē", _;
    acc_f: "~ē|~ēsē", _;
    acc_n: "~ē|~ēsē", _;
    abl_m: "~ē|~ēsē", _;
    abl_f: "~ē|~ēsē", _;
    abl_n: "~ē|~ēsē", _;
    ),
    // Ambŏ: 1 words (ambŏ). Like Complūrēs, except in 13 cells: nom_m sg, nom_m pl, nom_f pl, nom_n pl, gen_m pl, gen_f pl, ….
    declension!("ambŏ", 1, "~ŏ", like Complūrēs;
    nom_m: "~ŏ", "~ō";
    nom_f: _, "~ae";
    nom_n: _, "~ō";
    gen_m: _, "~ōrum";
    gen_f: _, "~ārum";
    gen_n: _, derived;
    dat_m: _, "~ōbus";
    dat_f: _, "~ābus";
    acc_m: _, "~ōs|~ō";
    acc_f: _, "~ās";
    acc_n: _, derived;
    voc_n: _, derived;
    ),
    // Complūsculī: 1 words (complūsculī). Like Ambŏ, except in 6 cells: nom_m sg, nom_m pl, nom_n pl, dat_m pl, dat_f pl, acc_m pl.
    declension!("complūsculī", 1, "~ī", like Ambŏ;
    nom_m: "~ī", "~ī";
    nom_n: _, "~a";
    dat_m: _, "~īs";
    dat_f: _, derived;
    acc_m: _, "~ōs";
    ),
];
