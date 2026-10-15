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
/// `nom_pl` (anim, inan)
/// `gen_m` (anim, inan)
/// `gen_f` (anim, inan)
/// `gen_n` (anim, inan)
/// `gen_pl` (anim, inan)
/// `dat_m` (anim, inan)
/// `dat_f` (anim, inan)
/// `dat_n` (anim, inan)
/// `dat_pl` (anim, inan)
/// `acc_m` (anim, inan)
/// `acc_f` (anim, inan)
/// `acc_n` (anim, inan)
/// `acc_pl` (anim, inan)
/// `ins_m` (anim, inan)
/// `ins_f` (anim, inan)
/// `ins_n` (anim, inan)
/// `ins_pl` (anim, inan)
/// `prep_m` (anim, inan)
/// `prep_f` (anim, inan)
/// `prep_n` (anim, inan)
/// `prep_pl` (anim, inan)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident $(as $view:ident)? $(with $affix:literal $(, $other:literal after $letters:literal)?)?)?;
     $(stem pronoun: $s0:tt;)?
     $(nom_m: $c0:tt, $c1:tt;)?
     $(nom_f: $c2:tt, $c3:tt;)?
     $(nom_n: $c4:tt, $c5:tt;)?
     $(nom_pl: $c6:tt, $c7:tt;)?
     $(gen_m: $c8:tt, $c9:tt;)?
     $(gen_f: $c10:tt, $c11:tt;)?
     $(gen_n: $c12:tt, $c13:tt;)?
     $(gen_pl: $c14:tt, $c15:tt;)?
     $(dat_m: $c16:tt, $c17:tt;)?
     $(dat_f: $c18:tt, $c19:tt;)?
     $(dat_n: $c20:tt, $c21:tt;)?
     $(dat_pl: $c22:tt, $c23:tt;)?
     $(acc_m: $c24:tt, $c25:tt;)?
     $(acc_f: $c26:tt, $c27:tt;)?
     $(acc_n: $c28:tt, $c29:tt;)?
     $(acc_pl: $c30:tt, $c31:tt;)?
     $(ins_m: $c32:tt, $c33:tt;)?
     $(ins_f: $c34:tt, $c35:tt;)?
     $(ins_n: $c36:tt, $c37:tt;)?
     $(ins_pl: $c38:tt, $c39:tt;)?
     $(prep_m: $c40:tt, $c41:tt;)?
     $(prep_f: $c42:tt, $c43:tt;)?
     $(prep_n: $c44:tt, $c45:tt;)?
     $(prep_pl: $c46:tt, $c47:tt;)?
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
            )),
            stems: ::std::borrow::Cow::Borrowed(concat!(
                $(flexion::cell!($s0),)? "\t",
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

/// What a declension adds after each pattern of the one it is like (`with "~ь", "~я"
/// after "йлмть"`: `~ь`, or `~я` where that pattern ends in `й`, `л`, `м`, `т` or `ь`).
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
    /// Like `оный`: 7 words.
    Оный,
    /// Like `свой`: 5 words.
    Свой,
    /// Like `чей`: 4 words.
    Чей,
    /// Like `иной`: 4 words.
    Иной,
    /// Like `некий`: 4 words.
    Некий,
    /// Like `другой`: 3 words.
    Другой,
    /// Like `наш`: 2 words.
    Наш,
    /// Like `тот`: 2 words.
    Тот,
    /// Like `сам`: 1 words.
    Сам,
    /// Like `иже`: 1 words.
    Иже,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 10] = [
        Declension::Оный,
        Declension::Свой,
        Declension::Чей,
        Declension::Иной,
        Declension::Некий,
        Declension::Другой,
        Declension::Наш,
        Declension::Тот,
        Declension::Сам,
        Declension::Иже,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 10] = [
    // Оный: 7 words (самый, который, каждый, некоторый, подобный, оный).
    declension!("оный", 7, "~ый";
    nom_m: _, "~ый";
    nom_f: "~ая", _;
    nom_n: "~ое", _;
    nom_pl: "~ые", _;
    gen_m: "~ого", _;
    gen_f: _, "~ой";
    gen_pl: "~ых", _;
    dat_m: _, "~ому";
    dat_f: "~ой", "~ой";
    dat_n: "~ому", "~ому";
    dat_pl: _, "~ым";
    acc_m: _, "~ый";
    acc_f: "~ую", _;
    ins_m: "~ым", "~ым";
    ins_f: "~ой|~ою", "~ой|~ою";
    ins_n: "~ым", "~ым";
    ins_pl: _, "~ыми";
    prep_m: "~ом", _;
    prep_f: "~ой", "~ой";
    ),
    // Свой: 5 words (мой, твой, свой, кой, никой).
    declension!("свой", 5, "~й";
    nom_m: _, "~й";
    nom_f: "~я", _;
    nom_n: "~ё", _;
    nom_pl: "~и", _;
    gen_m: "~его", _;
    gen_f: _, "~ей";
    gen_pl: "~их", _;
    dat_m: _, "~ему";
    dat_f: "~ей", "~ей";
    dat_n: "~ему", "~ему";
    dat_pl: _, "~им";
    acc_m: _, "~й";
    acc_f: "~ю", _;
    ins_m: "~им", "~им";
    ins_f: "~ей|~ею", "~ей|~ею";
    ins_n: "~им", "~им";
    ins_pl: _, "~ими";
    prep_m: "~ём", _;
    prep_f: "~ей", "~ей";
    ),
    // Чей: 4 words (чей, сей, ничей, кое-чей).
    declension!("чей", 4, "~ей";
    nom_m: _, "~ей";
    nom_f: "~ья", _;
    nom_n: "~ьё", _;
    nom_pl: "~ьи", _;
    gen_m: "~ьего", _;
    gen_f: _, "~ьей";
    gen_pl: "~ьих", _;
    dat_m: _, "~ьему";
    dat_f: "~ьей", "~ьей";
    dat_n: "~ьему", "~ьему";
    dat_pl: _, "~ьим";
    acc_m: _, "~ей";
    acc_f: "~ью", _;
    ins_m: "~ьим", "~ьим";
    ins_f: "~ьей|~ьею", "~ьей|~ьею";
    ins_n: "~ьим", "~ьим";
    ins_pl: _, "~ьими";
    prep_m: "~ьём", _;
    prep_f: "~ьей", "~ьей";
    ),
    // Иной: 4 words (иной, любой, таковой, каковой). Like Оный, except in 2 cells: nom_m inan, acc_m inan.
    declension!("иной", 4, "~ой", like Оный;
    nom_m: _, "~ой";
    acc_m: _, "~ой";
    ),
    // Некий: 4 words (некий, всякий, этакий, эдакий). Like Оный, except in 16 cells: nom_m inan, nom_pl anim, gen_f anim, gen_f inan, gen_pl anim, dat_f anim, ….
    declension!("некий", 4, "~ий", like Оный;
    nom_m: _, "~ий";
    nom_pl: "~ие", _;
    gen_f: "~ой", derived;
    gen_pl: "~их", _;
    dat_f: derived, derived;
    dat_pl: _, "~им";
    acc_m: _, "~ий";
    ins_m: "~им", "~им";
    ins_n: "~им", "~им";
    ins_pl: _, "~ими";
    prep_f: derived, derived;
    ),
    // Другой: 3 words (другой, кое-какой, никакой). Like Иной, except in 8 cells: nom_pl anim, gen_pl anim, dat_pl inan, ins_m anim, ins_m inan, ins_n anim, ….
    declension!("другой", 3, "~ой", like Иной;
    nom_pl: "~ие", _;
    gen_pl: "~их", _;
    dat_pl: _, "~им";
    ins_m: "~им", "~им";
    ins_n: "~им", "~им";
    ins_pl: _, "~ими";
    ),
    // Наш: 2 words (ваш, наш). Like Свой, except in 6 cells: nom_m inan, nom_f anim, nom_n anim, acc_m inan, acc_f anim, prep_m anim.
    declension!("наш", 2, "~", like Свой;
    nom_m: _, "~";
    nom_f: "~а", _;
    nom_n: "~е", _;
    acc_m: _, "~";
    acc_f: "~у", _;
    prep_m: "~ем", _;
    ),
    // Тот: 2 words (этот, тот). Like Сам with `` after each pattern (`от` after ~), except in 9 cells: nom_pl anim, gen_pl anim, dat_pl inan, acc_f anim, ins_m anim, ins_m inan, ….
    declension!("тот", 2, "~от", like Сам with "", "от" after "~";
    nom_pl: "~е", _;
    gen_pl: "~ех", _;
    dat_pl: _, "~ем";
    acc_f: "~у", _;
    ins_m: "~ем", "~ем";
    ins_n: "~ем", "~ем";
    ins_pl: _, "~еми";
    ),
    // Сам: 1 words (сам). Like Другой, except in 6 cells: nom_m inan, nom_f anim, nom_n anim, nom_pl anim, acc_m inan, acc_f anim.
    declension!("сам", 1, "~", like Другой;
    nom_m: _, "~";
    nom_f: "~а", _;
    nom_n: "~о", _;
    nom_pl: "~и", _;
    acc_m: _, "~";
    acc_f: "~у|~оё", _;
    ),
    // Иже: 1 words (иже).
    declension!("иже", 1, "и~";
    nom_m: _, "и~";
    nom_f: "я~", _;
    nom_n: "е~", _;
    nom_pl: "и~|я~", _;
    gen_m: "его~|него~", _;
    gen_f: "ея~|нея~", _;
    gen_pl: "их~|них~", _;
    dat_m: "ему~|нему~", _;
    dat_f: "ей~|ней~", "ей~|ней~";
    dat_pl: "им~|ним~", _;
    acc_m: _, "и~|ни~";
    acc_f: "ю~|ню~", _;
    acc_n: "е~|не~", "е~|не~";
    acc_pl: _, "и~|я~|ни~|ня~";
    ins_f: "ею~|нею~", "ею~|нею~";
    ins_pl: "ими~|ними~", _;
    prep_m: "нем~", _;
    prep_f: "ней~", "ней~";
    prep_pl: "них~", "них~";
    ),
];
