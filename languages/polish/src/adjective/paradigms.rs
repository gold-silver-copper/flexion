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
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident $(as $view:ident)?)?;
     $(stem adjective: $s0:tt;)?
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
    /// Like `wolny`: 5,051 words.
    Wolny,
    /// Like `słaby`: 4,843 words.
    Słaby,
    /// Like `lekki`: 1,310 words.
    Lekki,
    /// Like `odpowiedni`: 1,238 words.
    Odpowiedni,
    /// Like `ciężki`: 618 words.
    Ciężki,
    /// Like `drugi`: 49 words.
    Drugi,
    /// Like `jasny`: 38 words.
    Jasny,
    /// Like `herbaciany`: 17 words.
    Herbaciany,
    /// Like `stały`: 15 words.
    Stały,
    /// Like `mały`: 10 words.
    Mały,
    /// Like `rad`: 8 words.
    Rad,
    /// Like `zwięzły`: 7 words.
    Zwięzły,
    /// Like `ostry`: 6 words.
    Ostry,
    /// Like `uczulony`: 5 words.
    Uczulony,
    /// Like `żyzny`: 5 words.
    Żyzny,
    /// Like `wązki`: 4 words.
    Wązki,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 16] = [
        Declension::Wolny,
        Declension::Słaby,
        Declension::Lekki,
        Declension::Odpowiedni,
        Declension::Ciężki,
        Declension::Drugi,
        Declension::Jasny,
        Declension::Herbaciany,
        Declension::Stały,
        Declension::Mały,
        Declension::Rad,
        Declension::Zwięzły,
        Declension::Ostry,
        Declension::Uczulony,
        Declension::Żyzny,
        Declension::Wązki,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 16] = [
    // Wolny: 5,051 words (wolny, jenny, bury, hardy, bezwstydny, homoseksualny).
    declension!("wolny", 5051, "~y";
    nom_m: _, "~y";
    nom_f: "~a", _;
    nom_n: "~e", _;
    nom_vir: "~i", _;
    gen_m: "~ego", _;
    gen_f: "~ej", _;
    gen_vir: "~ych", _;
    dat_m: "~emu", _;
    dat_vir: "~ym", _;
    acc_f: "~ą", _;
    ins_vir: "~ymi", _;
    comparative: "~iejszy";
    superlative: "naj~iejszy";
    ),
    // Słaby: 4,843 words (party, złoty, pity, prosty, międzynarodowy, różowy). Like Wolny, except in 2 cells: comparative, superlative.
    declension!("słaby", 4843, "~y", like Wolny;
    comparative: "~szy";
    superlative: "naj~szy";
    ),
    // Lekki: 1,310 words (polski, rosyjski, norweski, ruski, łaciński, morski).
    declension!("lekki", 1310, "~~ki";
    nom_m: _, "~~ki";
    nom_f: "~~ka", _;
    nom_n: "~~kie", _;
    nom_vir: "~~cy", _;
    gen_m: "~~kiego", _;
    gen_f: "~~kiej", _;
    gen_vir: "~~kich", _;
    dat_m: "~~kiemu", _;
    dat_vir: "~~kim", _;
    acc_f: "~~ką", _;
    ins_vir: "~~kimi", _;
    comparative: "~ż~zy";
    superlative: "naj~ż~zy";
    ),
    // Odpowiedni: 1,238 words (psi, kani, babi, mulczujący, kuni, tani). Like Wolny, except in 7 cells: nom_m inan, nom_vir anim, gen_vir anim, dat_vir anim, ins_vir anim, comparative, ….
    declension!("odpowiedni", 1238, "~", like Wolny;
    nom_m: _, "~";
    nom_vir: "~", _;
    gen_vir: "~ch", _;
    dat_vir: "~m", _;
    ins_vir: "~mi", _;
    comparative: "~ejszy";
    superlative: "naj~ejszy";
    ),
    // Ciężki: 618 words (słodki, jaki, wszelki, malutki, niderlandzki, niemiecki). Like Słaby, except in 11 cells: nom_m inan, nom_f anim, nom_n anim, nom_vir anim, gen_m anim, gen_f anim, ….
    declension!("ciężki", 618, "~ki", like Słaby;
    nom_m: _, "~ki";
    nom_f: "~ka", _;
    nom_n: "~kie", _;
    nom_vir: "~cy", _;
    gen_m: "~kiego", _;
    gen_f: "~kiej", _;
    gen_vir: "~kich", _;
    dat_m: "~kiemu", _;
    dat_vir: "~kim", _;
    acc_f: "~ką", _;
    ins_vir: "~kimi", _;
    ),
    // Drugi: 49 words (drogi, nagi, drugi, długi, mnogi, wrogi). Like Wolny, except in 13 cells: nom_m inan, nom_f anim, nom_n anim, nom_vir anim, gen_m anim, gen_f anim, ….
    declension!("drugi", 49, "~gi", like Wolny;
    nom_m: _, "~gi";
    nom_f: "~ga", _;
    nom_n: "~gie", _;
    nom_vir: "~dzy", _;
    gen_m: "~giego", _;
    gen_f: "~giej", _;
    gen_vir: "~gich", _;
    dat_m: "~giemu", _;
    dat_vir: "~gim", _;
    acc_f: "~gą", _;
    ins_vir: "~gimi", _;
    comparative: "~ższy";
    superlative: "naj~ższy";
    ),
    // Jasny: 38 words (ciasny, własny, jasny, równoczesny, jednoczesny, wczesny). Like Lekki, except in 13 cells: nom_m inan, nom_f anim, nom_n anim, nom_vir anim, gen_m anim, gen_f anim, ….
    declension!("jasny", 38, "~s~y", like Lekki;
    nom_m: _, "~s~y";
    nom_f: "~s~a", _;
    nom_n: "~s~e", _;
    nom_vir: "~ś~i", _;
    gen_m: "~s~ego", _;
    gen_f: "~s~ej", _;
    gen_vir: "~s~ych", _;
    dat_m: "~s~emu", _;
    dat_vir: "~s~ym", _;
    acc_f: "~s~ą", _;
    ins_vir: "~s~ymi", _;
    comparative: "~ś~iejszy";
    superlative: "naj~ś~iejszy";
    ),
    // Herbaciany: 17 words (słony, jasnozielony, ciemnoczerwony, biało-czerwony, podczerwony, infraczerwony). Like Wolny, except in 13 cells: nom_m inan, nom_f anim, nom_n anim, nom_vir anim, gen_m anim, gen_f anim, ….
    declension!("herbaciany", 17, "~ny", like Wolny;
    nom_m: _, "~ny";
    nom_f: "~na", _;
    nom_n: "~ne", _;
    nom_vir: "~ni", _;
    gen_m: "~nego", _;
    gen_f: "~nej", _;
    gen_vir: "~nych", _;
    dat_m: "~nemu", _;
    dat_vir: "~nym", _;
    acc_f: "~ną", _;
    ins_vir: "~nymi", _;
    comparative: "~ńszy";
    superlative: "naj~ńszy";
    ),
    // Stały: 15 words (miły, dojrzały, stały, doskonały, czuły, trwały). Like Wolny, except in 13 cells: nom_m inan, nom_f anim, nom_n anim, nom_vir anim, gen_m anim, gen_f anim, ….
    declension!("stały", 15, "~ły", like Wolny;
    nom_m: _, "~ły";
    nom_f: "~ła", _;
    nom_n: "~łe", _;
    nom_vir: "~li", _;
    gen_m: "~łego", _;
    gen_f: "~łej", _;
    gen_vir: "~łych", _;
    dat_m: "~łemu", _;
    dat_vir: "~łym", _;
    acc_f: "~łą", _;
    ins_vir: "~łymi", _;
    comparative: "~lszy";
    superlative: "naj~lszy";
    ),
    // Mały: 10 words (zwykły, niezwykły, ścisły, nikły, smukły, wściekły). Like Stały, except in 2 cells: comparative, superlative.
    declension!("mały", 10, "~ły", like Stały;
    comparative: "~lejszy";
    superlative: "naj~lejszy";
    ),
    // Rad: 8 words (wart, rad, syt, łaskaw, miłościw, kontent). Like Wolny, except in 3 cells: nom_m inan, comparative, superlative.
    declension!("rad", 8, "~", like Wolny;
    nom_m: _, "~";
    comparative: derived;
    superlative: derived;
    ),
    // Zwięzły: 7 words (zwięzły, rozwiązły, niezły, rozlazły, spełzły, wypełzły). Like Wolny, except in 13 cells: nom_m inan, nom_f anim, nom_n anim, nom_vir anim, gen_m anim, gen_f anim, ….
    declension!("zwięzły", 7, "~zły", like Wolny;
    nom_m: _, "~zły";
    nom_f: "~zła", _;
    nom_n: "~złe", _;
    nom_vir: "~źli", _;
    gen_m: "~złego", _;
    gen_f: "~złej", _;
    gen_vir: "~złych", _;
    dat_m: "~złemu", _;
    dat_vir: "~złym", _;
    acc_f: "~złą", _;
    ins_vir: "~złymi", _;
    comparative: "~źlejszy";
    superlative: "naj~źlejszy";
    ),
    // Ostry: 6 words (ostry, mądry, bystry, chytry, szczodry, przykry). Like Wolny, except in 3 cells: nom_vir anim, comparative, superlative.
    declension!("ostry", 6, "~y", like Wolny;
    nom_vir: "~zy", _;
    comparative: "~zejszy";
    superlative: "naj~zejszy";
    ),
    // Uczulony: 5 words (ulubiony, szalony, uczony, zasłużony, uniżony). Like Wolny, except in 13 cells: nom_m inan, nom_f anim, nom_n anim, nom_vir anim, gen_m anim, gen_f anim, ….
    declension!("uczulony", 5, "~ony", like Wolny;
    nom_m: _, "~ony";
    nom_f: "~ona", _;
    nom_n: "~one", _;
    nom_vir: "~eni", _;
    gen_m: "~onego", _;
    gen_f: "~onej", _;
    gen_vir: "~onych", _;
    dat_m: "~onemu", _;
    dat_vir: "~onym", _;
    acc_f: "~oną", _;
    ins_vir: "~onymi", _;
    comparative: "~eńszy";
    superlative: "naj~eńszy";
    ),
    // Żyzny: 5 words (żelazny, przyjazny, żyzny, nieprzyjazny, nieżelazny). Like Lekki, except in 13 cells: nom_m inan, nom_f anim, nom_n anim, nom_vir anim, gen_m anim, gen_f anim, ….
    declension!("żyzny", 5, "~z~y", like Lekki;
    nom_m: _, "~z~y";
    nom_f: "~z~a", _;
    nom_n: "~z~e", _;
    nom_vir: "~ź~i", _;
    gen_m: "~z~ego", _;
    gen_f: "~z~ej", _;
    gen_vir: "~z~ych", _;
    dat_m: "~z~emu", _;
    dat_vir: "~z~ym", _;
    acc_f: "~z~ą", _;
    ins_vir: "~z~ymi", _;
    comparative: "~ź~iejszy";
    superlative: "naj~ź~iejszy";
    ),
    // Wązki: 4 words (sąsiedzki, wązki, dobrosąsiedzki, grudziądzki). Like Lekki, except in 13 cells: nom_m inan, nom_f anim, nom_n anim, nom_vir anim, gen_m anim, gen_f anim, ….
    declension!("wązki", 4, "~ą~ki", like Lekki;
    nom_m: _, "~ą~ki";
    nom_f: "~ą~ka", _;
    nom_n: "~ą~kie", _;
    nom_vir: "~ą~cy", _;
    gen_m: "~ą~kiego", _;
    gen_f: "~ą~kiej", _;
    gen_vir: "~ą~kich", _;
    dat_m: "~ą~kiemu", _;
    dat_vir: "~ą~kim", _;
    acc_f: "~ą~ką", _;
    ins_vir: "~ą~kimi", _;
    comparative: "~ężs~y";
    superlative: "naj~ężs~y";
    ),
];
