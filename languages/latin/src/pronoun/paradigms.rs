//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes (`~us`, `suus`: the stem and `us`). A stem in two pieces (`~~`, `ecquīnam`) has
//! letters between them that other forms may drop or change.
//! Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (if they share them),
/// the declension it is like (if any), the stem slots it states with a stem and an
/// ending table
/// (`stem gen_sg: \"~ius~- like quīvīs\"`: the slot's cells are `quīvīs`'s table's patterns
/// with `~ius~` for their `~`) (see `stems.rs`), then a row of patterns per line
/// (`derived`: the form comes from another cell's, see `relations.rs`; `_`,
/// or a row left out: the form is its stem slot's, or the declension's it is
/// like, or, in a declension like no other, derived):
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
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident $(as $view:ident)? $(with $affix:literal $(, $other:literal after $letters:literal)?)?)?;
     $(stem lemma: $s0:tt;)?
     $(stem rest__: $s1:tt;)?
     $(stem rest: $s2:tt;)?
     $(stem rest___: $s3:tt;)?
     $(stem gen_sg: $s4:tt;)?
     $(stem gen_pl: $s5:tt;)?
     $(stem rest_: $s6:tt;)?
     $(stem abl_sg: $s7:tt;)?
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
            reads: view!($($($view)?)?),
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
            stems: ::std::borrow::Cow::Borrowed(concat!(
                $(flexion::cell!($s0),)? "\t",
                $(flexion::cell!($s1),)? "\t",
                $(flexion::cell!($s2),)? "\t",
                $(flexion::cell!($s3),)? "\t",
                $(flexion::cell!($s4),)? "\t",
                $(flexion::cell!($s5),)? "\t",
                $(flexion::cell!($s6),)? "\t",
                $(flexion::cell!($s7),)? "\t",
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

/// What a declension adds after each pattern of the one it is like (`with "~", "us~" after "ī"`: `~`, or `us~`
/// where that pattern ends in `ī`).
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

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `ecquīnam`: 29 words.
    Ecquīnam,
    /// Like `quīdam`: 8 words.
    Quīdam,
    /// Like `noster`: 8 words.
    Noster,
    /// Like `ecquis`: 6 words.
    Ecquis,
    /// Like `istic`: 6 words.
    Istic,
    /// Like `suus`: 5 words.
    Suus,
    /// Like `iste`: 5 words.
    Iste,
    /// Like `utervīs`: 4 words.
    Utervīs,
    /// Like `ūllus`: 3 words.
    Ūllus,
    /// Like `quālis`: 3 words.
    Quālis,
    /// Like `complūria`: 2 words.
    Complūria,
    /// Like `quicquidlibet`: 1 words.
    Quicquidlibet,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 12] = [
        Declension::Ecquīnam,
        Declension::Quīdam,
        Declension::Noster,
        Declension::Ecquis,
        Declension::Istic,
        Declension::Suus,
        Declension::Iste,
        Declension::Utervīs,
        Declension::Ūllus,
        Declension::Quālis,
        Declension::Complūria,
        Declension::Quicquidlibet,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 12] = [
    // Ecquīnam: 29 words (is, nēmō, quī, aliquī, suī, quisque).
    declension!("ecquīnam", 29, "~~";
    nom_m: "~~", "~~";
    nom_f: _, "~~";
    nom_n: _, "~~";
    dat_m: _, "~bī~";
    acc_m: _, "~~";
    acc_f: _, "~~";
    ),
    // Quīdam: 8 words (quīdam, quīcumque, quīnam, quīlibet, quīvīs, quīcunque).
    declension!("quīdam", 8, "q~ī~";
    stem gen_sg: "~ius~- like quīvīs";
    nom_m: "q~ī~", "q~ī~";
    nom_f: "q~ae~", "q~ae~";
    nom_n: _, "q~ae~";
    gen_m: _, "q~ōrun~";
    gen_f: _, "q~ārun~";
    gen_n: derived, _;
    dat_m: "c~i~", "q~ibus~";
    dat_f: "c~i~", _;
    acc_m: "q~en~", "q~ōs~";
    acc_f: "q~an~", "q~ās~";
    acc_n: "q~id~", _;
    abl_m: "q~ō~", _;
    abl_f: "q~ā~", _;
    ),
    // Noster: 8 words (alter, noster, uter, vester, egomet, tūtemet).
    declension!("noster", 8, "~e~";
    stem lemma: "~e~- like uter";
    stem rest: "~~- like uter";
    stem rest___: "~~- like uter";
    stem abl_sg: "~~- like uter";
    nom_m: _, "~~ī";
    gen_m: "~~ī", "~~ōrum";
    gen_f: "~~ae", "~~ārum";
    dat_m: "~~ō", "~~īs";
    dat_f: "~~ae", _;
    acc_n: _, derived;
    abl_n: derived, _;
    voc_f: derived, derived;
    voc_n: _, derived;
    ),
    // Ecquis: 6 words (aliquis, ecquis, nēquis, sīquis, numquis, nesciōquis). Like Ecquīnam, except in 15 cells: nom_m sg, nom_m pl, nom_n sg, gen_n sg, dat_m sg, dat_m pl, ….
    declension!("ecquis", 6, "~q~is", like Ecquīnam;
    stem rest: "~q~- like uter";
    stem gen_sg: "~c~ius- like quidquid";
    stem abl_sg: "~q~- like uter";
    nom_m: "~q~is", "~q~ī";
    nom_n: "~q~id", _;
    gen_n: derived, _;
    dat_m: "~c~i", derived;
    dat_f: "~c~i", _;
    acc_m: "~q~em", _;
    acc_f: "~q~am", _;
    acc_n: "~q~od", derived;
    abl_n: derived, _;
    voc_f: derived, derived;
    voc_n: _, derived;
    ),
    // Istic: 6 words (hic, hice, quidquid, illic, istic, isthic).
    declension!("istic", 6, "~i~";
    stem gen_sg: "~īus~e- like quidquid";
    nom_m: "~i~", "~īs~e";
    nom_f: "~ae~", "~ae~";
    nom_n: "~u~|~u~ce|~o~", "~ae~";
    gen_m: _, "~ōrun~";
    gen_f: _, "~ārun~";
    gen_n: derived, _;
    dat_m: "~ī~", "~īs~e";
    dat_f: "~ī~", _;
    acc_m: "~un~", "~ōs~e";
    acc_f: "~an~", "~ās~e";
    acc_n: "~u~|~u~ce|~o~", _;
    abl_m: "~ō~", _;
    abl_f: "~ā~", _;
    ),
    // Suus: 5 words (tuus, meus, suus, cuius, ipsimus).
    declension!("suus", 5, "~us";
    stem rest: "~- like uter";
    stem rest___: "~- like uter";
    stem abl_sg: "~- like uter";
    nom_m: "~us", "~ī";
    gen_m: "~ī", "~ōrum";
    gen_f: "~ae", "~ārum";
    dat_m: "~ō", "~īs";
    dat_f: "~ae", _;
    acc_n: _, derived;
    abl_n: derived, _;
    voc_m: "~e", _;
    voc_f: derived, derived;
    voc_n: _, derived;
    ),
    // Iste: 5 words (ille, iste, ipse, seipse, olle). Like Suus, except in 7 cells: nom_m sg, gen_n sg, dat_n sg, acc_n sg, voc_m sg, voc_m pl, ….
    declension!("iste", 5, "~e", like Suus;
    stem rest__: "~ī- like utervīs";
    stem gen_sg: "~īus- like quidquid";
    nom_m: "~e", _;
    gen_n: derived, _;
    dat_n: derived, _;
    acc_n: "~ud", _;
    voc_m: derived, derived;
    voc_n: derived, _;
    ),
    // Utervīs: 4 words (uterque, utervīs, utercumque, uterlibet). Like Noster with `~` after each pattern (`us~` after ī), except in 4 cells: nom_m pl, gen_f sg, dat_m sg, dat_f sg.
    declension!("utervīs", 4, "~e~~", like Noster with "~", "us~" after "ī";
    nom_m: _, "~~ī~";
    gen_f: "~~īus~", _;
    dat_m: "~~ī~", _;
    dat_f: "~~ī~", _;
    ),
    // Ūllus: 3 words (alius, ūllus, nūllus). Like Suus, except in 4 cells: gen_m sg, gen_f sg, dat_n sg, voc_m pl.
    declension!("ūllus", 3, "~us", like Suus;
    stem rest__: "~ī- like utervīs";
    gen_m: "~īus|~ī", _;
    gen_f: "~īus", _;
    dat_n: derived, _;
    voc_m: _, derived;
    ),
    // Quālis: 3 words (quis, quisquis, quālis).
    declension!("quālis", 3, "~is";
    stem lemma: "~is- like uter";
    stem gen_sg: "~is- like quidquid";
    stem abl_sg: "~ī- like quis";
    nom_m: _, "~ēs";
    nom_f: "~is", "~ēs";
    nom_n: _, "~ia";
    gen_m: _, "~ium";
    gen_f: _, "~ium";
    gen_n: derived, _;
    dat_m: "~ī", "~ibus";
    dat_f: "~ī", _;
    acc_m: "~em", "~īs";
    acc_f: "~em", "~īs";
    acc_n: "~e", _;
    abl_n: derived, _;
    voc_n: "~e", _;
    ),
    // Complūria: 2 words (complūria, conplūria). Like Quālis, except in 18 cells: nom_m sg, nom_m pl, nom_f sg, nom_f pl, nom_n pl, gen_m sg, ….
    declension!("complūria", 2, "~ia", like Quālis;
    nom_m: "~ia", "~a|~ia";
    nom_f: derived, "~a|~ia";
    nom_n: _, "~a|~ia";
    gen_m: derived, _;
    gen_f: derived, _;
    dat_m: derived, _;
    dat_f: derived, _;
    acc_m: derived, "~a|~ia";
    acc_f: derived, "~a|~ia";
    acc_n: derived, _;
    abl_m: derived, _;
    abl_f: derived, _;
    voc_m: derived, _;
    voc_n: derived, _;
    ),
    // Quicquidlibet: 1 words (quicquidlibet).
    declension!("quicquidlibet", 1, "q~icq~id~";
    stem gen_sg: "~i~scuius~- like quīvīs";
    nom_m: "q~idq~id~|q~icq~id~", "q~aeq~ae~";
    nom_f: "q~idq~id~|q~icq~id~", "q~aeq~ae~";
    nom_n: "q~idq~id~|q~icq~id~", "q~aeq~ae~";
    gen_m: _, "q~ōr~mquōrum~";
    gen_f: _, "q~ōr~mquōrum~";
    gen_n: derived, _;
    dat_m: "c~ic~i~", "q~ib~squibus~|q~īsq~īs~";
    dat_f: "c~ic~i~", _;
    acc_m: "q~idq~id~|q~icq~id~", "q~aeq~ae~";
    acc_f: "q~idq~id~|q~icq~id~", "q~aeq~ae~";
    acc_n: "q~idq~id~|q~icq~id~", _;
    abl_m: "q~ōq~ō~", _;
    abl_f: "q~ōq~ō~", _;
    voc_m: "q~idq~id~|q~icq~id~", _;
    ),
];
