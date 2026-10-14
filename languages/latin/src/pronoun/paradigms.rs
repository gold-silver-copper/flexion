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
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident $(as $view:ident)?)?;
     $(stem pronoun_m_sg: $s0:tt;)?
     $(stem pronoun_m: $s1:tt;)?
     $(stem pronoun_f: $s2:tt;)?
     $(stem pronoun_n_sg: $s3:tt;)?
     $(stem pronoun_gen_sg: $s4:tt;)?
     $(stem pronoun_gen_pl: $s5:tt;)?
     $(stem pronoun_pl: $s6:tt;)?
     $(stem pronoun_abl_sg: $s7:tt;)?
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

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `ecquīnam`: 28 words.
    Ecquīnam,
    /// Like `quīvīs`: 8 words.
    Quīvīs,
    /// Like `uter`: 8 words.
    Uter,
    /// Like `ecquis`: 6 words.
    Ecquis,
    /// Like `quidquid`: 6 words.
    Quidquid,
    /// Like `suus`: 5 words.
    Suus,
    /// Like `iste`: 5 words.
    Iste,
    /// Like `utervīs`: 4 words.
    Utervīs,
    /// Like `alius`: 3 words.
    Alius,
    /// Like `quis`: 3 words.
    Quis,
    /// Like `complūria`: 2 words.
    Complūria,
    /// Like `ambŏ`: 1 words.
    Ambŏ,
    /// Like `temet`: 1 words.
    Temet,
    /// Like `quicquidlibet`: 1 words.
    Quicquidlibet,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 14] = [
        Declension::Ecquīnam,
        Declension::Quīvīs,
        Declension::Uter,
        Declension::Ecquis,
        Declension::Quidquid,
        Declension::Suus,
        Declension::Iste,
        Declension::Utervīs,
        Declension::Alius,
        Declension::Quis,
        Declension::Complūria,
        Declension::Ambŏ,
        Declension::Temet,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 14] = [
    // Ecquīnam: 28 words (is, nēmō, quī, aliquī, suī, quisque).
    declension!("ecquīnam", 28, "~~";
    nom_m: "~~", "~~";
    nom_f: _, "~~";
    nom_n: _, "~~";
    dat_m: _, "~bī~";
    acc_m: _, "~~";
    acc_f: _, "~~";
    ),
    // Quīvīs: 8 words (quīdam, quīcumque, quīnam, quīlibet, quīvīs, quīcunque).
    declension!("quīvīs", 8, "q~ī~";
    nom_m: "q~ī~", "q~ī~";
    nom_f: "q~ae~", "q~ae~";
    nom_n: "q~id~", "q~ae~";
    gen_m: "c~ius~", "q~ōrun~";
    gen_f: "c~ius~", "q~ārun~";
    dat_m: "c~i~", "q~ibus~";
    dat_f: "c~i~", _;
    acc_m: "q~en~", "q~ōs~";
    acc_f: "q~an~", "q~ās~";
    acc_n: "q~id~", _;
    abl_m: "q~ō~", _;
    abl_f: "q~ā~", _;
    ),
    // Uter: 8 words (alter, noster, uter, vester, egomet, tūtemet).
    declension!("uter", 8, "~e~";
    stem pronoun_m_sg: "~e~- like uter";
    nom_m: _, "~~ī";
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
    ),
    // Ecquis: 6 words (aliquis, ecquis, nēquis, sīquis, numquis, nesciōquis). Like Ecquīnam, except in 18 cells: nom_m sg, nom_m pl, nom_f sg, nom_f pl, nom_n sg, nom_n pl, ….
    declension!("ecquis", 6, "~q~is", like Ecquīnam;
    nom_m: "~q~is", "~q~ī";
    nom_f: "~q~a", "~q~ae";
    nom_n: "~q~id", "~q~a";
    gen_m: "~c~ius", _;
    gen_f: "~c~ius", _;
    dat_m: "~c~i", derived;
    dat_f: "~c~i", _;
    acc_m: "~q~em", "~q~ōs";
    acc_f: "~q~am", "~q~ās";
    acc_n: "~q~od", _;
    abl_m: "~q~ō", _;
    abl_f: "~q~ā", _;
    ),
    // Quidquid: 6 words (hic, hice, quidquid, illic, istic, isthic).
    declension!("quidquid", 6, "~i~";
    nom_m: "~i~", "~īs~e";
    nom_f: "~ae~", "~ae~";
    nom_n: "~u~|~u~ce|~o~", "~ae~";
    gen_m: "~ĭus~e", "~ōrun~";
    gen_f: "~ĭus~e", "~ārun~";
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
    nom_m: "~us", "~ī";
    nom_f: "~a", "~ae";
    nom_n: "~um", "~a";
    gen_m: "~ī", "~ōrum";
    gen_f: "~ae", "~ārum";
    dat_m: "~ō", "~īs";
    dat_f: "~ae", _;
    acc_m: "~um", "~ōs";
    acc_f: "~am", "~ās";
    acc_n: "~um", _;
    abl_m: "~ō", _;
    abl_f: "~ā", _;
    voc_m: "~e", _;
    ),
    // Iste: 5 words (ille, iste, ipse, seipse, olle). Like Suus, except in 8 cells: nom_m sg, nom_n sg, gen_m sg, gen_f sg, dat_m sg, dat_f sg, ….
    declension!("iste", 5, "~e", like Suus;
    nom_m: "~e", _;
    nom_n: "~ud", _;
    gen_m: "~ĭus", _;
    gen_f: "~ĭus", _;
    dat_m: "~ī", _;
    dat_f: "~ī", _;
    acc_n: "~ud", _;
    voc_m: derived, _;
    ),
    // Utervīs: 4 words (uterque, utervīs, utercumque, uterlibet).
    declension!("utervīs", 4, "~e~~";
    stem pronoun_m_sg: "~e~~- like uter";
    nom_m: _, "~~ī~";
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
    ),
    // Alius: 3 words (alius, ūllus, nūllus). Like Suus, except in 4 cells: gen_m sg, gen_f sg, dat_m sg, dat_f sg.
    declension!("alius", 3, "~us", like Suus;
    gen_m: "~ĭus|~ī", _;
    gen_f: "~ĭus", _;
    dat_m: "~ī", _;
    dat_f: "~ī", _;
    ),
    // Quis: 3 words (quis, quisquis, quālis).
    declension!("quis", 3, "~is";
    stem pronoun_m_sg: "~is- like uter";
    nom_m: _, "~ēs";
    nom_f: "~is", "~ēs";
    nom_n: "~e", "~ia";
    gen_m: "~is", "~ium";
    gen_f: "~is", "~ium";
    dat_m: "~ī", "~ibus";
    dat_f: "~ī", _;
    acc_m: "~em", "~īs";
    acc_f: "~em", "~īs";
    acc_n: "~e", _;
    abl_m: "~ī", _;
    abl_f: "~ī", _;
    ),
    // Complūria: 2 words (complūria, conplūria). Like Quis, except in 18 cells: nom_m sg, nom_m pl, nom_f sg, nom_f pl, nom_n sg, nom_n pl, ….
    declension!("complūria", 2, "~ia", like Quis;
    nom_m: "~ia", "~a|~ia";
    nom_f: derived, "~a|~ia";
    nom_n: derived, "~a|~ia";
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
    ),
    // Ambŏ: 1 words (ambŏ). Like Complūria, except in 11 cells: nom_m sg, nom_m pl, nom_f pl, nom_n pl, gen_m pl, gen_f pl, ….
    declension!("ambŏ", 1, "~ŏ", like Complūria;
    nom_m: "~ŏ", "~ō";
    nom_f: _, "~ae";
    nom_n: _, "~ō";
    gen_m: _, "~ōrum";
    gen_f: _, "~ārum";
    dat_m: _, "~ōbus";
    dat_f: _, "~ābus";
    acc_m: _, "~ōs|~ō";
    acc_f: _, "~ās";
    abl_f: _, "~ābus";
    ),
    // Temet: 1 words (temet). Like Ecquis, except in 16 cells: nom_m pl, nom_f sg, nom_f pl, nom_n sg, nom_n pl, gen_m sg, ….
    declension!("temet", 1, "~e~", like Ecquis;
    stem pronoun_m_sg: "~ūte~- like uter";
    nom_m: _, derived;
    nom_f: "~ūte~", derived;
    nom_n: "~ūte~", derived;
    gen_m: "~uī~", _;
    gen_f: "~uī~", _;
    dat_m: "~ibi~", _;
    dat_f: "~ibi~", _;
    acc_m: "~e~", derived;
    acc_f: "~e~", derived;
    acc_n: "~e~", _;
    abl_m: "~e~", _;
    abl_f: "~e~", _;
    ),
    // Quicquidlibet: 1 words (quicquidlibet).
    declension!("quicquidlibet", 1, "q~icq~id~";
    nom_m: "q~idq~id~|q~icq~id~", "q~aeq~ae~";
    nom_f: "q~idq~id~|q~icq~id~", "q~aeq~ae~";
    nom_n: "q~idq~id~|q~icq~id~", "q~aeq~ae~";
    gen_m: "c~i~scuius~", "q~ōr~mquōrum~";
    gen_f: "c~i~scuius~", "q~ōr~mquōrum~";
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
