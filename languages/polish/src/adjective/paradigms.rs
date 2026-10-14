//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first. A `%`
//! is the first letter of the stem piece after it (reduplication: `%ε~κα`),
//! as the copy laws in `laws.rs` change it. A `ˇ` ends
//! the patterns of a declension whose stems alternate: the laws in `laws.rs`
//! for alternating stems change them (`~nˇ`: `katu`, `kadun`).

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
    /// Like `wolny`: 8,195 words.
    Wolny,
    /// Like `słaby`: 1,412 words.
    Słaby,
    /// Like `lekki`: 1,310 words.
    Lekki,
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
    /// Like `rad`: 8 words.
    Rad,
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
        Declension::Lekki,
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
        Declension::Rad,
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
    nom_m: _, "~y";
    nom_f: "~a", _;
    nom_n: "~e", _;
    nom_vir: "~i", _;
    gen_m: "~ego", _;
    gen_f: "~ej", _;
    gen_vir: "~ych", _;
    dat_m: "~emu", _;
    dat_vir: _, "~ym";
    dat_nvir: "~ym", "~ym";
    acc_m: _, "~y";
    acc_f: _, "~ą";
    ins_m: "~ym", "~ym";
    ins_f: "~ą", "~ą";
    ins_n: "~ym", "~ym";
    ins_vir: _, "~ymi";
    ins_nvir: "~ymi", "~ymi";
    loc_m: "~ym", "~ym";
    loc_n: "~ym", "~ym";
    comparative: "~iejszy";
    ),
    // Słaby: 1,412 words (mulczujący, różowy, żółty, dany, chory, stary). Like Wolny, except in 1 cell: comparative.
    declension!("słaby", 1412, "~y", like Wolny;
    comparative: "~szy";
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
    dat_vir: _, "~~kim";
    dat_nvir: "~~kim", "~~kim";
    acc_m: _, "~~ki";
    acc_f: _, "~~ką";
    ins_m: "~~kim", "~~kim";
    ins_f: "~~ką", "~~ką";
    ins_n: "~~kim", "~~kim";
    ins_vir: _, "~~kimi";
    ins_nvir: "~~kimi", "~~kimi";
    loc_m: "~~kim", "~~kim";
    loc_n: "~~kim", "~~kim";
    comparative: "~ż~zy";
    ),
    // Party: 1,218 words (party, złoty, hardy, pity, pusty, ciasny). Like Wolny, except in 27 cells: nom_m inan, nom_f anim, nom_n anim, nom_vir anim, gen_m anim, gen_f anim, ….
    declension!("party", 1218, "~yˇ", like Wolny;
    nom_m: _, "~yˇ";
    nom_f: "~aˇ", _;
    nom_n: "~eˇ", _;
    nom_vir: "~iˇ", _;
    gen_m: "~egoˇ", _;
    gen_f: "~ejˇ", _;
    gen_vir: "~ychˇ", _;
    dat_m: "~emuˇ", _;
    dat_vir: _, "~ymˇ";
    dat_nvir: "~ymˇ", "~ymˇ";
    acc_m: _, "~yˇ";
    acc_f: _, "~ąˇ";
    ins_m: "~ymˇ", "~ymˇ";
    ins_f: "~ąˇ", "~ąˇ";
    ins_n: "~ymˇ", "~ymˇ";
    ins_vir: _, "~ymiˇ";
    ins_nvir: "~ymiˇ", "~ymiˇ";
    loc_m: "~ymˇ", "~ymˇ";
    loc_n: "~ymˇ", "~ymˇ";
    comparative: "~iejszyˇ";
    ),
    // Ciężki: 618 words (słodki, jaki, wszelki, malutki, niderlandzki, niemiecki). Like Słaby, except in 26 cells: nom_m inan, nom_f anim, nom_n anim, nom_vir anim, gen_m anim, gen_f anim, ….
    declension!("ciężki", 618, "~ki", like Słaby;
    nom_m: _, "~ki";
    nom_f: "~ka", _;
    nom_n: "~kie", _;
    nom_vir: "~cy", _;
    gen_m: "~kiego", _;
    gen_f: "~kiej", _;
    gen_vir: "~kich", _;
    dat_m: "~kiemu", _;
    dat_vir: _, "~kim";
    dat_nvir: "~kim", "~kim";
    acc_m: _, "~ki";
    acc_f: _, "~ką";
    ins_m: "~kim", "~kim";
    ins_f: "~ką", "~ką";
    ins_n: "~kim", "~kim";
    ins_vir: _, "~kimi";
    ins_nvir: "~kimi", "~kimi";
    loc_m: "~kim", "~kim";
    loc_n: "~kim", "~kim";
    ),
    // Odpowiedni: 238 words (psi, kani, babi, kuni, tani, dropi). Like Wolny, except in 19 cells: nom_m inan, nom_vir anim, gen_vir anim, dat_vir inan, dat_nvir anim, dat_nvir inan, ….
    declension!("odpowiedni", 238, "~", like Wolny;
    nom_m: _, "~";
    nom_vir: "~", _;
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
    comparative: "~ejszy";
    ),
    // Drugi: 49 words (drogi, nagi, drugi, długi, mnogi, wrogi). Like Wolny, except in 27 cells: nom_m inan, nom_f anim, nom_n anim, nom_vir anim, gen_m anim, gen_f anim, ….
    declension!("drugi", 49, "~gi", like Wolny;
    nom_m: _, "~gi";
    nom_f: "~ga", _;
    nom_n: "~gie", _;
    nom_vir: "~dzy", _;
    gen_m: "~giego", _;
    gen_f: "~giej", _;
    gen_vir: "~gich", _;
    dat_m: "~giemu", _;
    dat_vir: _, "~gim";
    dat_nvir: "~gim", "~gim";
    acc_m: _, "~gi";
    acc_f: _, "~gą";
    ins_m: "~gim", "~gim";
    ins_f: "~gą", "~gą";
    ins_n: "~gim", "~gim";
    ins_vir: _, "~gimi";
    ins_nvir: "~gimi", "~gimi";
    loc_m: "~gim", "~gim";
    loc_n: "~gim", "~gim";
    comparative: "~ższy";
    ),
    // Mały: 30 words (zwykły, ciekły, niezwykły, ścisły, nikły, smukły). Like Wolny, except in 27 cells: nom_m inan, nom_f anim, nom_n anim, nom_vir anim, gen_m anim, gen_f anim, ….
    declension!("mały", 30, "~ły", like Wolny;
    nom_m: _, "~ły";
    nom_f: "~ła", _;
    nom_n: "~łe", _;
    nom_vir: "~li", _;
    gen_m: "~łego", _;
    gen_f: "~łej", _;
    gen_vir: "~łych", _;
    dat_m: "~łemu", _;
    dat_vir: _, "~łym";
    dat_nvir: "~łym", "~łym";
    acc_m: _, "~ły";
    acc_f: _, "~łą";
    ins_m: "~łym", "~łym";
    ins_f: "~łą", "~łą";
    ins_n: "~łym", "~łym";
    ins_vir: _, "~łymi";
    ins_nvir: "~łymi", "~łymi";
    loc_m: "~łym", "~łym";
    loc_n: "~łym", "~łym";
    comparative: "~lejszy";
    ),
    // Otwarty: 29 words (odkryty, szyty, uszyty, umyty, myty, zmyty). Like Wolny, except in 28 cells: nom_m inan, nom_f anim, nom_n anim, nom_vir anim, gen_m anim, gen_f anim, ….
    declension!("otwarty", 29, "~ty", like Wolny;
    nom_m: _, "~ty";
    nom_f: "~ta", _;
    nom_n: "~te", _;
    nom_vir: "~ci", _;
    gen_m: "~tego", _;
    gen_f: "~tej", _;
    gen_vir: "~tych", _;
    dat_m: "~temu", _;
    dat_vir: _, "~tym";
    dat_nvir: "~tym", "~tym";
    acc_m: _, "~ty";
    acc_f: _, "~tą";
    ins_m: "~tym", "~tym";
    ins_f: "~tą", "~tą";
    ins_n: "~tym", "~tym";
    ins_vir: _, "~tymi";
    ins_nvir: "~tymi", "~tymi";
    loc_m: "~tym", "~tym";
    loc_n: "~tym", "~tym";
    comparative: "~tszy";
    superlative: "naj~tszy";
    ),
    // Soboli: 25 words (orli, muli, karli, ośli, krokodyli, pszczeli). Like Wolny, except in 18 cells: nom_m inan, gen_vir anim, dat_vir inan, dat_nvir anim, dat_nvir inan, acc_m inan, ….
    declension!("soboli", 25, "~i", like Wolny;
    nom_m: _, "~i";
    gen_vir: "~ich", _;
    dat_vir: _, "~im";
    dat_nvir: "~im", "~im";
    acc_m: _, "~i";
    ins_m: "~im", "~im";
    ins_n: "~im", "~im";
    ins_vir: _, "~imi";
    ins_nvir: "~imi", "~imi";
    loc_m: "~im", "~im";
    loc_n: "~im", "~im";
    comparative: derived;
    ),
    // Prosty: 23 words (prosty, młody, twardy, częsty, chudy, tłusty). Like Party, except in 1 cell: comparative.
    declension!("prosty", 23, "~yˇ", like Party;
    comparative: "~szyˇ";
    ),
    // Stały: 19 words (miły, były, dojrzały, stały, doskonały, czuły). Like Mały, except in 1 cell: comparative.
    declension!("stały", 19, "~ły", like Mały;
    comparative: "~lszy";
    ),
    // Herbaciany: 14 words (słony, jasnozielony, ciemnoczerwony, biało-czerwony, podczerwony, infraczerwony). Like Wolny, except in 27 cells: nom_m inan, nom_f anim, nom_n anim, nom_vir anim, gen_m anim, gen_f anim, ….
    declension!("herbaciany", 14, "~ny", like Wolny;
    nom_m: _, "~ny";
    nom_f: "~na", _;
    nom_n: "~ne", _;
    nom_vir: "~ni", _;
    gen_m: "~nego", _;
    gen_f: "~nej", _;
    gen_vir: "~nych", _;
    dat_m: "~nemu", _;
    dat_vir: _, "~nym";
    dat_nvir: "~nym", "~nym";
    acc_m: _, "~ny";
    acc_f: _, "~ną";
    ins_m: "~nym", "~nym";
    ins_f: "~ną", "~ną";
    ins_n: "~nym", "~nym";
    ins_vir: _, "~nymi";
    ins_nvir: "~nymi", "~nymi";
    loc_m: "~nym", "~nym";
    loc_n: "~nym", "~nym";
    comparative: "~ńszy";
    ),
    // Zajęczy: 12 words (bosy, łasy, rusy, krasy, kusy, kosy). Like Słaby, except in 2 cells: nom_vir anim, superlative.
    declension!("zajęczy", 12, "~y", like Słaby;
    nom_vir: "~y", _;
    superlative: "naj~szy";
    ),
    // Rad: 8 words (wart, rad, syt, łaskaw, miłościw, kontent). Like Wolny, except in 3 cells: nom_m inan, acc_m inan, comparative.
    declension!("rad", 8, "~", like Wolny;
    nom_m: _, "~";
    acc_m: _, "~";
    comparative: derived;
    ),
    // Ostry: 6 words (ostry, mądry, bystry, chytry, szczodry, przykry). Like Wolny, except in 2 cells: nom_vir anim, comparative.
    declension!("ostry", 6, "~y", like Wolny;
    nom_vir: "~zy", _;
    comparative: "~zejszy";
    ),
    // Zwięzły: 6 words (zwięzły, rozwiązły, niezły, spełzły, wypełzły, sczezły). Like Wolny, except in 27 cells: nom_m inan, nom_f anim, nom_n anim, nom_vir anim, gen_m anim, gen_f anim, ….
    declension!("zwięzły", 6, "~zły", like Wolny;
    nom_m: _, "~zły";
    nom_f: "~zła", _;
    nom_n: "~złe", _;
    nom_vir: "~źli", _;
    gen_m: "~złego", _;
    gen_f: "~złej", _;
    gen_vir: "~złych", _;
    dat_m: "~złemu", _;
    dat_vir: _, "~złym";
    dat_nvir: "~złym", "~złym";
    acc_m: _, "~zły";
    acc_f: _, "~złą";
    ins_m: "~złym", "~złym";
    ins_f: "~złą", "~złą";
    ins_n: "~złym", "~złym";
    ins_vir: _, "~złymi";
    ins_nvir: "~złymi", "~złymi";
    loc_m: "~złym", "~złym";
    loc_n: "~złym", "~złym";
    comparative: "~źlejszy";
    ),
    // Uczulony: 5 words (ulubiony, szalony, uczony, zasłużony, uniżony). Like Wolny, except in 27 cells: nom_m inan, nom_f anim, nom_n anim, nom_vir anim, gen_m anim, gen_f anim, ….
    declension!("uczulony", 5, "~ony", like Wolny;
    nom_m: _, "~ony";
    nom_f: "~ona", _;
    nom_n: "~one", _;
    nom_vir: "~eni", _;
    gen_m: "~onego", _;
    gen_f: "~onej", _;
    gen_vir: "~onych", _;
    dat_m: "~onemu", _;
    dat_vir: _, "~onym";
    dat_nvir: "~onym", "~onym";
    acc_m: _, "~ony";
    acc_f: _, "~oną";
    ins_m: "~onym", "~onym";
    ins_f: "~oną", "~oną";
    ins_n: "~onym", "~onym";
    ins_vir: _, "~onymi";
    ins_nvir: "~onymi", "~onymi";
    loc_m: "~onym", "~onym";
    loc_n: "~onym", "~onym";
    comparative: "~eńszy";
    ),
    // Wązki: 4 words (sąsiedzki, wązki, dobrosąsiedzki, grudziądzki). Like Lekki, except in 27 cells: nom_m inan, nom_f anim, nom_n anim, nom_vir anim, gen_m anim, gen_f anim, ….
    declension!("wązki", 4, "~ą~ki", like Lekki;
    nom_m: _, "~ą~ki";
    nom_f: "~ą~ka", _;
    nom_n: "~ą~kie", _;
    nom_vir: "~ą~cy", _;
    gen_m: "~ą~kiego", _;
    gen_f: "~ą~kiej", _;
    gen_vir: "~ą~kich", _;
    dat_m: "~ą~kiemu", _;
    dat_vir: _, "~ą~kim";
    dat_nvir: "~ą~kim", "~ą~kim";
    acc_m: _, "~ą~ki";
    acc_f: _, "~ą~ką";
    ins_m: "~ą~kim", "~ą~kim";
    ins_f: "~ą~ką", "~ą~ką";
    ins_n: "~ą~kim", "~ą~kim";
    ins_vir: _, "~ą~kimi";
    ins_nvir: "~ą~kimi", "~ą~kimi";
    loc_m: "~ą~kim", "~ą~kim";
    loc_n: "~ą~kim", "~ą~kim";
    comparative: "~ężs~y";
    ),
    // Żyzny: 3 words (przyjazny, żyzny, nieprzyjazny). Like Lekki, except in 27 cells: nom_m inan, nom_f anim, nom_n anim, nom_vir anim, gen_m anim, gen_f anim, ….
    declension!("żyzny", 3, "~z~y", like Lekki;
    nom_m: _, "~z~y";
    nom_f: "~z~a", _;
    nom_n: "~z~e", _;
    nom_vir: "~ź~i", _;
    gen_m: "~z~ego", _;
    gen_f: "~z~ej", _;
    gen_vir: "~z~ych", _;
    dat_m: "~z~emu", _;
    dat_vir: _, "~z~ym";
    dat_nvir: "~z~ym", "~z~ym";
    acc_m: _, "~z~y";
    acc_f: _, "~z~ą";
    ins_m: "~z~ym", "~z~ym";
    ins_f: "~z~ą", "~z~ą";
    ins_n: "~z~ym", "~z~ym";
    ins_vir: _, "~z~ymi";
    ins_nvir: "~z~ymi", "~z~ymi";
    loc_m: "~z~ym", "~z~ym";
    loc_n: "~z~ym", "~z~ym";
    comparative: "~ź~iejszy";
    ),
];
