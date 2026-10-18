//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes (`~nen`, `toinen`: the stem and `nen`). A stem in two pieces (`~k~`, `mikään`) has
//! letters between them that other forms may drop or change.
//! Alternatives are separated by `|`, the usual one first. A capital (`A`, `V`)
//! is a class letter: a harmony law in `laws.rs` writes it, by the
//! letter before it.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (if they share them),
/// the declension it is like (if any), the stem slots it states with a stem and an
/// ending table
/// (`stem nom_pl: \"~se- like toinen\"`: the slot's cells are `toinen`'s table's patterns
/// with `~se` for their `~`) (see `stems.rs`), then a row of patterns per line
/// (`derived`: the form comes from another cell's, see `relations.rs`; `_`,
/// or a row left out: the form is its stem slot's, or the declension's it is
/// like, or, in a declension like no other, derived):
/// `nom` (sg, pl)
/// `gen_` (sg, pl)
/// `par` (sg, pl)
/// `ine` (sg, pl)
/// `ela` (sg, pl)
/// `ill` (sg, pl)
/// `ade` (sg, pl)
/// `abl` (sg, pl)
/// `all` (sg, pl)
/// `ess` (sg, pl)
/// `tra` (sg, pl)
/// `abe` (sg, pl)
/// `ins` (sg, pl)
/// `com` (sg, pl)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident $(as $view:ident)?)?;
     $(stem nom_sg: $s0:tt;)?
     $(stem nom_pl: $s1:tt;)?
     $(stem gen_pl: $s2:tt;)?
     $(stem par_sg: $s3:tt;)?
     $(nom: $c0:tt, $c1:tt;)?
     $(gen_: $c2:tt, $c3:tt;)?
     $(par: $c4:tt, $c5:tt;)?
     $(ine: $c6:tt, $c7:tt;)?
     $(ela: $c8:tt, $c9:tt;)?
     $(ill: $c10:tt, $c11:tt;)?
     $(ade: $c12:tt, $c13:tt;)?
     $(abl: $c14:tt, $c15:tt;)?
     $(all: $c16:tt, $c17:tt;)?
     $(ess: $c18:tt, $c19:tt;)?
     $(tra: $c20:tt, $c21:tt;)?
     $(abe: $c22:tt, $c23:tt;)?
     $(ins: $c24:tt, $c25:tt;)?
     $(com: $c26:tt, $c27:tt;)?
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
            )),
            stems: ::std::borrow::Cow::Borrowed(concat!(
                $(flexion::cell!($s0),)? "\t",
                $(flexion::cell!($s1),)? "\t",
                $(flexion::cell!($s2),)? "\t",
                $(flexion::cell!($s3),)? "\t",
            )),
            affix: None,
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
    /// Like `mi`: 27 words.
    Mi,
    /// Like `mikään`: 8 words.
    Mikään,
    /// Like `toinen`: 7 words.
    Toinen,
    /// Like `meikä`: 7 words.
    Meikä,
    /// Like `he`: 5 words.
    He,
    /// Like `itseni`: 5 words.
    Itseni,
    /// Like `meikämanne`: 5 words.
    Meikämanne,
    /// Like `mä`: 4 words.
    Mä,
    /// Like `sama`: 3 words.
    Sama,
    /// Like `jompi`: 3 words.
    Jompi,
    /// Like `molemmat`: 1 words.
    Molemmat,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 11] = [
        Declension::Mi,
        Declension::Mikään,
        Declension::Toinen,
        Declension::Meikä,
        Declension::He,
        Declension::Itseni,
        Declension::Meikämanne,
        Declension::Mä,
        Declension::Sama,
        Declension::Jompi,
        Declension::Molemmat,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 11] = [
    // Mi: 27 words (mi, hän, yksi, tämä, joku, kuka).
    declension!("mi", 27, "~";
    nom: "~", "~t";
    gen_: "~n", "~n";
    par: "~tä", "~tA";
    ine: "~ssA", "~ssä";
    ela: "~stA", "~stä";
    ill: "~hin", "~hin";
    ade: "~llA", "~llä";
    abl: "~ltA", "~ltä";
    all: "~lle", "~lle";
    ess: "~nA", "~nä";
    tra: "~ksi", "~ksi";
    ),
    // Mikään: 8 words (joka, mikä, jokin, kukin, mikään, mikin).
    declension!("mikään", 8, "~k~";
    nom: "~k~", "~tk~";
    gen_: "~nk~", "~nk~";
    par: "~t~", "~t~";
    ine: "~ss~", "~ss~";
    ela: "~st~", "~st~";
    ill: "~hink~", "~hink~";
    ade: "~ll~", "~ll~";
    abl: "~lt~", "~lt~";
    all: "~llek~", "~llek~";
    ess: "~n~", "~n~";
    tra: "~ksik~", "~ksik~";
    ins: _, "~nk~";
    ),
    // Toinen: 7 words (toinen, jokainen, kumpainen, meikäläinen, teikäläinen, semmonen).
    declension!("toinen", 7, "~nen";
    stem nom_pl: "~se- like toinen";
    stem gen_pl: "~s- like toinen";
    stem par_sg: "~s- like toinen";
    nom: "~nen", _;
    ),
    // Meikä: 7 words (se, meitsi, meikä, meikäpoika, teitsi, meikätyttö). Like Mi, except in 14 cells: nom pl, gen_ pl, par sg, par pl, ine pl, ela pl, ….
    declension!("meikä", 7, "~", like Mi;
    nom: _, derived;
    gen_: _, derived;
    par: "~A", derived;
    ine: _, derived;
    ela: _, derived;
    ill: "~Vn", derived;
    ade: _, derived;
    abl: _, derived;
    all: _, derived;
    ess: _, derived;
    tra: _, derived;
    abe: "~ttA", _;
    ),
    // He: 5 words (te, he, me, toisemme, toisenne).
    declension!("he", 5, "~e";
    nom: "~än", "~e";
    gen_: "~Anen", "~eidAn";
    par: "~äntä", "~eitA";
    ine: "~AnessA", "~eissA";
    ela: "~AnestA", "~eistA";
    ill: "~Aneen", "~eihin";
    ade: "~AnellA", "~eillA";
    abl: "~AneltA", "~eiltA";
    all: "~Anelle", "~eille";
    ess: "~AnenA", "~einA";
    tra: "~Aneksi", "~eiksi";
    abe: "~AnettA", "~eittA";
    ),
    // Itseni: 5 words (itseni, itsesi, itsensä, itsemme, itsenne).
    declension!("itseni", 5, "~~~";
    nom: "~~~", _;
    gen_: "~~~", _;
    par: "~ä~~", _;
    ine: "~ssä~~", _;
    ela: "~stä~~", _;
    ill: "~e~~", _;
    ade: "~llä~~", _;
    abl: "~ltä~~", _;
    all: "~lle~~", _;
    ess: "~~än~", _;
    tra: "~kse~~", _;
    abe: "~ttä~~", _;
    ),
    // Meikämanne: 5 words (tuo, muu, yks, meikämanne, meikäukko).
    declension!("meikämanne", 5, "~";
    nom: "~", "~t";
    gen_: "~n", "~jen";
    par: "~a", "~ja";
    ine: "~ssa", "~issa";
    ela: "~sta", "~ista";
    ill: "~Vn", "~ihin";
    ade: "~lla", "~illa";
    abl: "~lta", "~ilta";
    all: "~lle", "~ille";
    ess: "~na", "~ina";
    tra: "~ksi", "~iksi";
    abe: "~tta", "~itta";
    ins: _, "~in";
    com: _, "~ine";
    ),
    // Mä: 4 words (minä, sinä, sä, mä). Like Meikä, except in 12 cells: nom sg, gen_ sg, par sg, ine sg, ela sg, ill sg, ….
    declension!("mä", 4, "~ä", like Meikä;
    nom: "~ä", _;
    gen_: "~un", _;
    par: "~uA", _;
    ine: "~ussA", _;
    ela: "~ustA", _;
    ill: "~uhun", _;
    ade: "~ullA", _;
    abl: "~ultA", _;
    all: "~ulle", _;
    ess: "~unA", _;
    tra: "~uksi", _;
    abe: derived, _;
    ),
    // Sama: 3 words (sama, muutama, aniharva).
    declension!("sama", 3, "~a";
    stem nom_pl: "~a- like sama";
    nom: "~a", _;
    gen_: _, "~ojen";
    par: "~aV", "~ojA";
    ine: _, "~oissA";
    ela: _, "~oistA";
    ill: "~aVn", "~oihin";
    ade: _, "~oillA";
    abl: _, "~oiltA";
    all: "~alle", "~oille";
    ess: "~anV", "~oinA";
    tra: "~aksi", "~oiksi";
    abe: _, "~oittA";
    ins: _, "~oin";
    com: _, "~oine";
    ),
    // Jompi: 3 words (kumpi, molempi, jompi).
    declension!("jompi", 3, "~pi";
    stem nom_pl: "~m- like jompi";
    nom: "~pi", _;
    gen_: _, "~pien";
    par: "~pAV", "~piA";
    ine: _, "~missa";
    ela: _, "~mista";
    ill: "~pAVn", "~piin";
    ade: _, "~milla";
    abl: _, "~milta";
    all: "~malle", "~mille";
    ess: "~pAnV", "~pinA";
    tra: "~maksi", "~miksi";
    abe: _, "~mitta";
    ins: _, "~min";
    com: _, "~pine";
    ),
    // Molemmat: 1 words (molemmat).
    declension!("molemmat", 1, "~mat";
    nom: "~mat", "~mat";
    gen_: _, "~pien";
    par: _, "~pia";
    ine: _, "~missa";
    ela: _, "~mista";
    ill: _, "~piin";
    ade: _, "~milla";
    abl: _, "~milta";
    all: _, "~mille";
    ess: _, "~pina";
    tra: _, "~miksi";
    abe: _, "~mitta";
    ins: _, "~min";
    com: _, "~pine";
    ),
];
