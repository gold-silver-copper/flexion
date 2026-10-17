//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes. A stem in two pieces (`~est~`, `kevyesti`) has
//! letters between them that other forms may drop or change.
//! Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (if they share them),
/// the declension it is like (if any), the stem slots it states with a stem and an
/// ending table (see `stems.rs`), then a row of patterns per line
/// (`derived`: the form comes from another cell's, see `relations.rs`; `_`,
/// or a row left out: the form is its stem slot's, or the declension's it is
/// like, or, in a declension like no other, derived):
/// `adverb` (pos, comp, sup)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident $(as $view:ident)?)?;
     $(stem pos: $s0:tt;)?
     $(stem comp: $s1:tt;)?
     $(stem sup: $s2:tt;)?
     $(adverb: $c0:tt, $c1:tt, $c2:tt;)?
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
            )),
            stems: ::std::borrow::Cow::Borrowed(concat!(
                $(flexion::cell!($s0),)? "\t",
                $(flexion::cell!($s1),)? "\t",
                $(flexion::cell!($s2),)? "\t",
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
    /// Like `kevyesti`: 1,094 words.
    Kevyesti,
    /// Like `karkeasti`: 509 words.
    Karkeasti,
    /// Like `kiihkeästi`: 279 words.
    Kiihkeästi,
    /// Like `paksusti`: 77 words.
    Paksusti,
    /// Like `iisisti`: 55 words.
    Iisisti,
    /// Like `kovasti`: 53 words.
    Kovasti,
    /// Like `suoraan`: 33 words.
    Suoraan,
    /// Like `herkästi`: 29 words.
    Herkästi,
    /// Like `tiiviisti`: 6 words.
    Tiiviisti,
    /// Like `oikein`: 4 words.
    Oikein,
    /// Like `edellä`: 4 words.
    Edellä,
    /// Like `kaukaa`: 3 words.
    Kaukaa,
    /// Like `etäälle`: 3 words.
    Etäälle,
    /// Like `lujaa`: 3 words.
    Lujaa,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 14] = [
        Declension::Kevyesti,
        Declension::Karkeasti,
        Declension::Kiihkeästi,
        Declension::Paksusti,
        Declension::Iisisti,
        Declension::Kovasti,
        Declension::Suoraan,
        Declension::Herkästi,
        Declension::Tiiviisti,
        Declension::Oikein,
        Declension::Edellä,
        Declension::Kaukaa,
        Declension::Etäälle,
        Declension::Lujaa,
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
    // Kevyesti: 1,094 words (rauhallisesti, vihaisesti, kuuliaisesti, geologisesti, geopoliittisesti, verisesti).
    declension!("kevyesti", 1094, "~est~";
    adverb: "~est~", "~emm~n", "~~mmin";
    ),
    // Karkeasti: 509 words (seuraavasti, oikeasti, raa'asti, kodikkaasti, jatkuvasti, mielenkiinnottomasti).
    declension!("karkeasti", 509, "~ast~";
    adverb: "~ast~", "~amm~n", "~~mmin";
    ),
    // Kiihkeästi: 279 words (välittömästi, järkevästi, leveästi, epämiehekkäästi, ehkäisevästi, himmeästi).
    declension!("kiihkeästi", 279, "~äst~";
    adverb: "~äst~", "~ämm~n", "~~mmin";
    ),
    // Paksusti: 77 words (oudosti, huonosti, rajusti, epäreilusti, helposti, kömpelösti).
    declension!("paksusti", 77, "~st~";
    adverb: "~st~", "~mm~n", "~~mmin";
    ),
    // Iisisti: 55 words (epäsiististi, modernisti, iisisti, epänormaalisti, abstraktisti, banaalisti).
    declension!("iisisti", 55, "~~st~";
    adverb: "~~st~", "~~mm~n", "~e~mm~n";
    ),
    // Kovasti: 53 words (julmasti, tuhmasti, kivasti, laajasti, rumasti, hurjasti). Like Kevyesti, except in 1 cell: adverb pos.
    declension!("kovasti", 53, "~ast~", like Kevyesti;
    adverb: "~ast~", _, _;
    ),
    // Suoraan: 33 words (alas, pian, ulkona, ennen, takaa, taakse).
    declension!("suoraan", 33, "~~";
    adverb: "~~", "~emm~", "~imm~";
    ),
    // Herkästi: 29 words (kylmästi, jyrkästi, epäselvästi, tyhmästi, nöyrästi, epäkypsästi). Like Kevyesti, except in 1 cell: adverb pos.
    declension!("herkästi", 29, "~äst~", like Kevyesti;
    adverb: "~äst~", _, _;
    ),
    // Tiiviisti: 6 words (kauniisti, ilmatiiviisti, tiiviisti, kalliisti, alttiisti, raittiisti).
    declension!("tiiviisti", 6, "~~~sti";
    adverb: "~~~sti", "~~~mmin", "~e~mm~n";
    ),
    // Oikein: 4 words (väärin, oikein, usein, aikaisin).
    declension!("oikein", 4, "~~~";
    adverb: "~~~", "~amm~~", "~~mmi~";
    ),
    // Edellä: 4 words (edellä, keskellä, keskelle, keskeltä).
    declension!("edellä", 4, "~e~";
    adverb: "~e~", "~emmä~", "~immä~";
    ),
    // Kaukaa: 3 words (takana, kaukaa, kaukana).
    declension!("kaukaa", 3, "~k~";
    adverb: "~k~", "~emp~", "~imp~";
    ),
    // Etäälle: 3 words (etäältä, etäälle, etäällä).
    declension!("etäälle", 3, "~~";
    adverb: "~~", "~mmä~", _;
    ),
    // Lujaa: 3 words (kovaa, sisään, lujaa). Like Kaukaa, except in 1 cell: adverb pos.
    declension!("lujaa", 3, "~~", like Kaukaa;
    adverb: "~~", _, _;
    ),
];
