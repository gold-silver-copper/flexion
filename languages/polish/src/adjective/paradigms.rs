//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes (`~y`, `wolny`: the stem and `y`). A stem in two pieces (`~~ki`, `niski`) has
//! letters between them that other forms may drop or change.
//! Alternatives are separated by `|`, the usual one first. A `ˇ` ends
//! the patterns of a declension whose stems alternate: the laws in `laws.rs`
//! for alternating stems change them (`~yˇ`: `party`).

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (if they share them),
/// the declension it is like (if any), the stem slots it states with a stem and an
/// ending table
/// (`stem lemma: \"~- like wolny\"`: the slot's cells are `wolny`'s table's patterns
/// with `~` for their `~`) (see `stems.rs`), then a row of patterns per line
/// (`derived`: the form comes from another cell's, see `relations.rs`; `_`,
/// or a row left out: the form is its stem slot's, or the declension's it is
/// like, or, in a declension like no other, derived):
/// `nom_m` (anim, inan)
/// `nom_f` (anim, inan)
/// `nom_n` (anim, inan)
/// `nom_vir` (anim, inan)
/// `nom_nvir` (anim, inan)
/// `gen_m` (anim, inan)
/// `gen_f` (anim, inan)
/// `gen_n` (anim, inan)
/// `gen_vir` (anim, inan)
/// `gen_nvir` (anim, inan)
/// `dat_m` (anim, inan)
/// `dat_f` (anim, inan)
/// `dat_n` (anim, inan)
/// `dat_vir` (anim, inan)
/// `dat_nvir` (anim, inan)
/// `acc_m` (anim, inan)
/// `acc_f` (anim, inan)
/// `acc_n` (anim, inan)
/// `acc_vir` (anim, inan)
/// `acc_nvir` (anim, inan)
/// `ins_m` (anim, inan)
/// `ins_f` (anim, inan)
/// `ins_n` (anim, inan)
/// `ins_vir` (anim, inan)
/// `ins_nvir` (anim, inan)
/// `loc_m` (anim, inan)
/// `loc_f` (anim, inan)
/// `loc_n` (anim, inan)
/// `loc_vir` (anim, inan)
/// `loc_nvir` (anim, inan)
/// `comparative`
/// `superlative`
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident $(as $view:ident)? $(with $affix:literal $(, $other:literal after $letters:literal)?)?)?;
     $(stem lemma: $s0:tt;)?
     $(stem adjective_nom_vir_anim: $s1:tt;)?
     $(nom_m: $c0:tt, $c1:tt;)?
     $(nom_f: $c2:tt, $c3:tt;)?
     $(nom_n: $c4:tt, $c5:tt;)?
     $(nom_vir: $c6:tt, $c7:tt;)?
     $(nom_nvir: $c8:tt, $c9:tt;)?
     $(gen_m: $c10:tt, $c11:tt;)?
     $(gen_f: $c12:tt, $c13:tt;)?
     $(gen_n: $c14:tt, $c15:tt;)?
     $(gen_vir: $c16:tt, $c17:tt;)?
     $(gen_nvir: $c18:tt, $c19:tt;)?
     $(dat_m: $c20:tt, $c21:tt;)?
     $(dat_f: $c22:tt, $c23:tt;)?
     $(dat_n: $c24:tt, $c25:tt;)?
     $(dat_vir: $c26:tt, $c27:tt;)?
     $(dat_nvir: $c28:tt, $c29:tt;)?
     $(acc_m: $c30:tt, $c31:tt;)?
     $(acc_f: $c32:tt, $c33:tt;)?
     $(acc_n: $c34:tt, $c35:tt;)?
     $(acc_vir: $c36:tt, $c37:tt;)?
     $(acc_nvir: $c38:tt, $c39:tt;)?
     $(ins_m: $c40:tt, $c41:tt;)?
     $(ins_f: $c42:tt, $c43:tt;)?
     $(ins_n: $c44:tt, $c45:tt;)?
     $(ins_vir: $c46:tt, $c47:tt;)?
     $(ins_nvir: $c48:tt, $c49:tt;)?
     $(loc_m: $c50:tt, $c51:tt;)?
     $(loc_f: $c52:tt, $c53:tt;)?
     $(loc_n: $c54:tt, $c55:tt;)?
     $(loc_vir: $c56:tt, $c57:tt;)?
     $(loc_nvir: $c58:tt, $c59:tt;)?
     $(comparative: $c60:tt;)?
     $(superlative: $c61:tt;)?
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
                $(flexion::cell!($c36),)? "\t",
                $(flexion::cell!($c37),)? "\t",
                $(flexion::cell!($c38),)? "\t",
                $(flexion::cell!($c39),)? "\t",
                $(flexion::cell!($c40),)? "\t",
                $(flexion::cell!($c41),)? "\t",
                $(flexion::cell!($c42),)? "\t",
                $(flexion::cell!($c43),)? "\t",
                $(flexion::cell!($c44),)? "\t",
                $(flexion::cell!($c45),)? "\t",
                $(flexion::cell!($c46),)? "\t",
                $(flexion::cell!($c47),)? "\t",
                $(flexion::cell!($c48),)? "\t",
                $(flexion::cell!($c49),)? "\t",
                $(flexion::cell!($c50),)? "\t",
                $(flexion::cell!($c51),)? "\t",
                $(flexion::cell!($c52),)? "\t",
                $(flexion::cell!($c53),)? "\t",
                $(flexion::cell!($c54),)? "\t",
                $(flexion::cell!($c55),)? "\t",
                $(flexion::cell!($c56),)? "\t",
                $(flexion::cell!($c57),)? "\t",
                $(flexion::cell!($c58),)? "\t",
                $(flexion::cell!($c59),)? "\t",
                $(flexion::cell!($c60),)? "\t",
                $(flexion::cell!($c61),)? "\t",
            )),
            stems: ::std::borrow::Cow::Borrowed(concat!(
                $(flexion::cell!($s0),)? "\t",
                $(flexion::cell!($s1),)? "\t",
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

/// What a declension adds after each pattern of the one it is like (`with "", "y" after "~"`: ``, or `y`
/// where that pattern ends in `~`).
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
    /// Like `wolny`: 8,195 words.
    Wolny,
    /// Like `słaby`: 1,412 words.
    Słaby,
    /// Like `niski`: 1,310 words.
    Niski,
    /// Like `party`: 1,218 words.
    Party,
    /// Like `ciężki`: 618 words.
    Ciężki,
    /// Like `odpowiedni`: 238 words.
    Odpowiedni,
    /// Like `drugi`: 49 words.
    Drugi,
    /// Like `mały`: 30 words.
    Mały,
    /// Like `otwarty`: 29 words.
    Otwarty,
    /// Like `soboli`: 25 words.
    Soboli,
    /// Like `prosty`: 23 words.
    Prosty,
    /// Like `stały`: 19 words.
    Stały,
    /// Like `herbaciany`: 14 words.
    Herbaciany,
    /// Like `zajęczy`: 12 words.
    Zajęczy,
    /// Like `wart`: 8 words.
    Wart,
    /// Like `ostry`: 6 words.
    Ostry,
    /// Like `zwięzły`: 6 words.
    Zwięzły,
    /// Like `uczulony`: 5 words.
    Uczulony,
    /// Like `wązki`: 4 words.
    Wązki,
    /// Like `żyzny`: 3 words.
    Żyzny,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 20] = [
        Declension::Wolny,
        Declension::Słaby,
        Declension::Niski,
        Declension::Party,
        Declension::Ciężki,
        Declension::Odpowiedni,
        Declension::Drugi,
        Declension::Mały,
        Declension::Otwarty,
        Declension::Soboli,
        Declension::Prosty,
        Declension::Stały,
        Declension::Herbaciany,
        Declension::Zajęczy,
        Declension::Wart,
        Declension::Ostry,
        Declension::Zwięzły,
        Declension::Uczulony,
        Declension::Wązki,
        Declension::Żyzny,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 20] = [
    // Wolny: 8,195 words (wolny, jenny, bury, bezwstydny, homoseksualny, uszny).
    declension!("wolny", 8195, "~y";
    stem lemma: "~- like wolny";
    stem adjective_nom_vir_anim: "~i- like wolny";
    nom_f: _, derived;
    nom_n: _, derived;
    nom_vir: _, derived;
    nom_nvir: derived, derived;
    gen_m: _, derived;
    gen_f: _, derived;
    gen_n: derived, derived;
    gen_vir: _, derived;
    gen_nvir: derived, derived;
    dat_m: _, derived;
    dat_f: derived, derived;
    dat_n: derived, derived;
    acc_m: derived, _;
    acc_n: derived, derived;
    acc_vir: derived, derived;
    acc_nvir: derived, derived;
    loc_f: derived, derived;
    loc_vir: derived, derived;
    loc_nvir: derived, derived;
    ),
    // Słaby: 1,412 words (mulczujący, różowy, żółty, dany, chory, stary). Like Wart with `` after each pattern (`y` after ~), except in 1 cell: comparative.
    declension!("słaby", 1412, "~y", like Wart with "", "y" after "~";
    comparative: "~szy";
    ),
    // Niski: 1,310 words (polski, rosyjski, norweski, ruski, łaciński, morski).
    declension!("niski", 1310, "~~ki";
    stem lemma: "~~k- like lekki";
    nom_f: _, derived;
    nom_n: _, derived;
    nom_vir: "~~cy", _;
    nom_nvir: derived, derived;
    gen_m: _, derived;
    gen_f: _, derived;
    gen_n: derived, derived;
    gen_vir: _, derived;
    gen_nvir: derived, derived;
    dat_m: _, derived;
    dat_f: derived, derived;
    dat_n: derived, derived;
    acc_m: derived, _;
    acc_n: derived, derived;
    acc_vir: derived, derived;
    acc_nvir: derived, derived;
    loc_f: derived, derived;
    loc_vir: derived, derived;
    loc_nvir: derived, derived;
    comparative: "~ż~zy";
    ),
    // Party: 1,218 words (party, złoty, hardy, pity, pusty, ciasny). Like Wolny with `ˇ` after each pattern.
    declension!("party", 1218, "~yˇ", like Wolny with "ˇ";
    ),
    // Ciężki: 618 words (słodki, jaki, wszelki, malutki, niderlandzki, niemiecki). Like Słaby, except in 30 cells: nom_f inan, nom_n inan, nom_vir anim, nom_nvir anim, nom_nvir inan, gen_m inan, ….
    declension!("ciężki", 618, "~ki", like Słaby;
    stem lemma: "~k- like lekki";
    nom_f: _, derived;
    nom_n: _, derived;
    nom_vir: "~cy", _;
    nom_nvir: derived, derived;
    gen_m: _, derived;
    gen_f: _, derived;
    gen_n: derived, derived;
    gen_vir: _, derived;
    gen_nvir: derived, derived;
    dat_m: _, derived;
    dat_f: derived, derived;
    dat_n: derived, derived;
    acc_m: derived, _;
    acc_n: derived, derived;
    acc_vir: derived, derived;
    acc_nvir: derived, derived;
    loc_f: derived, derived;
    loc_vir: derived, derived;
    loc_nvir: derived, derived;
    ),
    // Odpowiedni: 238 words (psi, kani, babi, kuni, tani, dropi). Like Wolny, except in 18 cells: nom_m inan, nom_vir inan, gen_vir anim, dat_vir inan, dat_nvir anim, dat_nvir inan, ….
    declension!("odpowiedni", 238, "~", like Wolny;
    stem adjective_nom_vir_anim: "~- like wolny";
    nom_m: _, "~";
    nom_vir: _, derived;
    gen_vir: "~ch", _;
    dat_vir: _, "~m";
    dat_nvir: "~m", "~m";
    acc_m: _, "~";
    ins_m: "~m", "~m";
    ins_n: "~m", "~m";
    ins_vir: _, "~mi";
    ins_nvir: "~mi", "~mi";
    loc_m: "~m", "~m";
    loc_n: "~m", "~m";
    ),
    // Drugi: 49 words (drogi, nagi, drugi, długi, mnogi, wrogi). Like Wolny, except in 31 cells: nom_f inan, nom_n inan, nom_vir anim, nom_nvir anim, nom_nvir inan, gen_m inan, ….
    declension!("drugi", 49, "~gi", like Wolny;
    stem lemma: "~g- like lekki";
    nom_f: _, derived;
    nom_n: _, derived;
    nom_vir: "~dzy", _;
    nom_nvir: derived, derived;
    gen_m: _, derived;
    gen_f: _, derived;
    gen_n: derived, derived;
    gen_vir: _, derived;
    gen_nvir: derived, derived;
    dat_m: _, derived;
    dat_f: derived, derived;
    dat_n: derived, derived;
    acc_m: derived, _;
    acc_n: derived, derived;
    acc_vir: derived, derived;
    acc_nvir: derived, derived;
    loc_f: derived, derived;
    loc_vir: derived, derived;
    loc_nvir: derived, derived;
    comparative: "~ższy";
    ),
    // Mały: 30 words (zwykły, ciekły, niezwykły, ścisły, nikły, smukły). Like Wolny, except in 30 cells: nom_f inan, nom_n inan, nom_vir inan, nom_nvir anim, nom_nvir inan, gen_m inan, ….
    declension!("mały", 30, "~ły", like Wolny;
    stem lemma: "~ł- like wolny";
    stem adjective_nom_vir_anim: "~l- like mały";
    nom_f: _, derived;
    nom_n: _, derived;
    nom_vir: _, derived;
    nom_nvir: derived, derived;
    gen_m: _, derived;
    gen_f: _, derived;
    gen_n: derived, derived;
    gen_vir: _, derived;
    gen_nvir: derived, derived;
    dat_m: _, derived;
    dat_f: derived, derived;
    dat_n: derived, derived;
    acc_m: derived, _;
    acc_n: derived, derived;
    acc_vir: derived, derived;
    acc_nvir: derived, derived;
    loc_f: derived, derived;
    loc_vir: derived, derived;
    loc_nvir: derived, derived;
    ),
    // Otwarty: 29 words (odkryty, szyty, uszyty, umyty, myty, zmyty). Like Wolny, except in 32 cells: nom_f inan, nom_n inan, nom_vir anim, nom_nvir anim, nom_nvir inan, gen_m inan, ….
    declension!("otwarty", 29, "~ty", like Wolny;
    stem lemma: "~t- like wolny";
    nom_f: _, derived;
    nom_n: _, derived;
    nom_vir: "~ci", _;
    nom_nvir: derived, derived;
    gen_m: _, derived;
    gen_f: _, derived;
    gen_n: derived, derived;
    gen_vir: _, derived;
    gen_nvir: derived, derived;
    dat_m: _, derived;
    dat_f: derived, derived;
    dat_n: derived, derived;
    acc_m: derived, _;
    acc_n: derived, derived;
    acc_vir: derived, derived;
    acc_nvir: derived, derived;
    loc_f: derived, derived;
    loc_vir: derived, derived;
    loc_nvir: derived, derived;
    comparative: "~tszy";
    superlative: "naj~tszy";
    ),
    // Soboli: 25 words (orli, muli, karli, ośli, krokodyli, pszczeli). Like Wart with `` after each pattern (`i` after ~), except in 15 cells: gen_vir anim, dat_vir inan, dat_nvir anim, dat_nvir inan, ins_m anim, ins_m inan, ….
    declension!("soboli", 25, "~i", like Wart with "", "i" after "~";
    gen_vir: "~ich", _;
    dat_vir: _, "~im";
    dat_nvir: "~im", "~im";
    ins_m: "~im", "~im";
    ins_n: "~im", "~im";
    ins_vir: _, "~imi";
    ins_nvir: "~imi", "~imi";
    loc_m: "~im", "~im";
    loc_n: "~im", "~im";
    ),
    // Prosty: 23 words (prosty, młody, twardy, częsty, chudy, tłusty). Like Wolny with `ˇ` after each pattern, except in 1 cell: comparative.
    declension!("prosty", 23, "~yˇ", like Wolny with "ˇ";
    comparative: "~szyˇ";
    ),
    // Stały: 19 words (miły, były, dojrzały, stały, doskonały, czuły). Like Mały, except in 1 cell: nom_vir inan.
    declension!("stały", 19, "~ły", like Mały;
    stem adjective_nom_vir_anim: "~l- like słaby";
    nom_vir: _, derived;
    ),
    // Herbaciany: 14 words (słony, jasnozielony, ciemnoczerwony, biało-czerwony, podczerwony, infraczerwony). Like Wolny, except in 31 cells: nom_f inan, nom_n inan, nom_vir anim, nom_nvir anim, nom_nvir inan, gen_m inan, ….
    declension!("herbaciany", 14, "~ny", like Wolny;
    stem lemma: "~n- like wolny";
    nom_f: _, derived;
    nom_n: _, derived;
    nom_vir: "~ni", _;
    nom_nvir: derived, derived;
    gen_m: _, derived;
    gen_f: _, derived;
    gen_n: derived, derived;
    gen_vir: _, derived;
    gen_nvir: derived, derived;
    dat_m: _, derived;
    dat_f: derived, derived;
    dat_n: derived, derived;
    acc_m: derived, _;
    acc_n: derived, derived;
    acc_vir: derived, derived;
    acc_nvir: derived, derived;
    loc_f: derived, derived;
    loc_vir: derived, derived;
    loc_nvir: derived, derived;
    comparative: "~ńszy";
    ),
    // Zajęczy: 12 words (bosy, łasy, rusy, krasy, kusy, kosy). Like Wart with `` after each pattern (`y` after ~), except in 3 cells: nom_vir anim, comparative, superlative.
    declension!("zajęczy", 12, "~y", like Wart with "", "y" after "~";
    nom_vir: "~y", _;
    comparative: "~szy";
    superlative: "naj~szy";
    ),
    // Wart: 8 words (wart, rad, syt, łaskaw, miłościw, kontent). Like Wolny, except in 3 cells: nom_m inan, acc_m inan, comparative.
    declension!("wart", 8, "~", like Wolny;
    nom_m: _, "~";
    acc_m: _, "~";
    comparative: derived;
    ),
    // Ostry: 6 words (ostry, mądry, bystry, chytry, szczodry, przykry). Like Wart with `` after each pattern (`y` after ~), except in 2 cells: nom_vir anim, comparative.
    declension!("ostry", 6, "~y", like Wart with "", "y" after "~";
    nom_vir: "~zy", _;
    comparative: "~zejszy";
    ),
    // Zwięzły: 6 words (zwięzły, rozwiązły, niezły, spełzły, wypełzły, sczezły). Like Wolny, except in 30 cells: nom_f inan, nom_n inan, nom_vir inan, nom_nvir anim, nom_nvir inan, gen_m inan, ….
    declension!("zwięzły", 6, "~zły", like Wolny;
    stem lemma: "~zł- like wolny";
    stem adjective_nom_vir_anim: "~źl- like mały";
    nom_f: _, derived;
    nom_n: _, derived;
    nom_vir: _, derived;
    nom_nvir: derived, derived;
    gen_m: _, derived;
    gen_f: _, derived;
    gen_n: derived, derived;
    gen_vir: _, derived;
    gen_nvir: derived, derived;
    dat_m: _, derived;
    dat_f: derived, derived;
    dat_n: derived, derived;
    acc_m: derived, _;
    acc_n: derived, derived;
    acc_vir: derived, derived;
    acc_nvir: derived, derived;
    loc_f: derived, derived;
    loc_vir: derived, derived;
    loc_nvir: derived, derived;
    ),
    // Uczulony: 5 words (ulubiony, szalony, uczony, zasłużony, uniżony). Like Wolny, except in 31 cells: nom_f inan, nom_n inan, nom_vir anim, nom_nvir anim, nom_nvir inan, gen_m inan, ….
    declension!("uczulony", 5, "~ony", like Wolny;
    stem lemma: "~on- like wolny";
    nom_f: _, derived;
    nom_n: _, derived;
    nom_vir: "~eni", _;
    nom_nvir: derived, derived;
    gen_m: _, derived;
    gen_f: _, derived;
    gen_n: derived, derived;
    gen_vir: _, derived;
    gen_nvir: derived, derived;
    dat_m: _, derived;
    dat_f: derived, derived;
    dat_n: derived, derived;
    acc_m: derived, _;
    acc_n: derived, derived;
    acc_vir: derived, derived;
    acc_nvir: derived, derived;
    loc_f: derived, derived;
    loc_vir: derived, derived;
    loc_nvir: derived, derived;
    comparative: "~eńszy";
    ),
    // Wązki: 4 words (sąsiedzki, wązki, dobrosąsiedzki, grudziądzki). Like Niski, except in 31 cells: nom_f inan, nom_n inan, nom_vir anim, nom_nvir anim, nom_nvir inan, gen_m inan, ….
    declension!("wązki", 4, "~ą~ki", like Niski;
    stem lemma: "~ą~k- like lekki";
    nom_f: _, derived;
    nom_n: _, derived;
    nom_vir: "~ą~cy", _;
    nom_nvir: derived, derived;
    gen_m: _, derived;
    gen_f: _, derived;
    gen_n: derived, derived;
    gen_vir: _, derived;
    gen_nvir: derived, derived;
    dat_m: _, derived;
    dat_f: derived, derived;
    dat_n: derived, derived;
    acc_m: derived, _;
    acc_n: derived, derived;
    acc_vir: derived, derived;
    acc_nvir: derived, derived;
    loc_f: derived, derived;
    loc_vir: derived, derived;
    loc_nvir: derived, derived;
    comparative: "~ężs~y";
    ),
    // Żyzny: 3 words (przyjazny, żyzny, nieprzyjazny). Like Niski, except in 30 cells: nom_f inan, nom_n inan, nom_vir inan, nom_nvir anim, nom_nvir inan, gen_m inan, ….
    declension!("żyzny", 3, "~z~y", like Niski;
    stem lemma: "~z~- like wolny";
    stem adjective_nom_vir_anim: "~ź~i- like wolny";
    nom_f: _, derived;
    nom_n: _, derived;
    nom_vir: _, derived;
    nom_nvir: derived, derived;
    gen_m: _, derived;
    gen_f: _, derived;
    gen_n: derived, derived;
    gen_vir: _, derived;
    gen_nvir: derived, derived;
    dat_m: _, derived;
    dat_f: derived, derived;
    dat_n: derived, derived;
    acc_m: derived, _;
    acc_n: derived, derived;
    acc_vir: derived, derived;
    acc_nvir: derived, derived;
    loc_f: derived, derived;
    loc_vir: derived, derived;
    loc_nvir: derived, derived;
    ),
];
