//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (`[gender: Neuter]`, if
/// they share them), the declension it is like (if any), then a row of patterns
/// per line (`derived`: the form comes from another cell's, see
/// `relations.rs`; `_`: the form is the declension's it is like):
/// `nominative_masculine` (anim, inan)
/// `nominative_feminine` (anim, inan)
/// `nominative_neuter` (anim, inan)
/// `nominative_plural` (anim, inan)
/// `genitive_masculine` (anim, inan)
/// `genitive_feminine` (anim, inan)
/// `genitive_neuter` (anim, inan)
/// `genitive_plural` (anim, inan)
/// `dative_masculine` (anim, inan)
/// `dative_feminine` (anim, inan)
/// `dative_neuter` (anim, inan)
/// `dative_plural` (anim, inan)
/// `accusative_masculine` (anim, inan)
/// `accusative_feminine` (anim, inan)
/// `accusative_neuter` (anim, inan)
/// `accusative_plural` (anim, inan)
/// `instrumental_masculine` (anim, inan)
/// `instrumental_feminine` (anim, inan)
/// `instrumental_neuter` (anim, inan)
/// `instrumental_plural` (anim, inan)
/// `prepositional_masculine` (anim, inan)
/// `prepositional_feminine` (anim, inan)
/// `prepositional_neuter` (anim, inan)
/// `prepositional_plural` (anim, inan)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident)?;
     nominative_masculine: $c0:tt, $c1:tt;
     nominative_feminine: $c2:tt, $c3:tt;
     nominative_neuter: $c4:tt, $c5:tt;
     nominative_plural: $c6:tt, $c7:tt;
     genitive_masculine: $c8:tt, $c9:tt;
     genitive_feminine: $c10:tt, $c11:tt;
     genitive_neuter: $c12:tt, $c13:tt;
     genitive_plural: $c14:tt, $c15:tt;
     dative_masculine: $c16:tt, $c17:tt;
     dative_feminine: $c18:tt, $c19:tt;
     dative_neuter: $c20:tt, $c21:tt;
     dative_plural: $c22:tt, $c23:tt;
     accusative_masculine: $c24:tt, $c25:tt;
     accusative_feminine: $c26:tt, $c27:tt;
     accusative_neuter: $c28:tt, $c29:tt;
     accusative_plural: $c30:tt, $c31:tt;
     instrumental_masculine: $c32:tt, $c33:tt;
     instrumental_feminine: $c34:tt, $c35:tt;
     instrumental_neuter: $c36:tt, $c37:tt;
     instrumental_plural: $c38:tt, $c39:tt;
     prepositional_masculine: $c40:tt, $c41:tt;
     prepositional_feminine: $c42:tt, $c43:tt;
     prepositional_neuter: $c44:tt, $c45:tt;
     prepositional_plural: $c46:tt, $c47:tt;
    ) => {
        Paradigm {
            name: ::std::borrow::Cow::Borrowed($name),
            words: $words,
            lemma: ::std::borrow::Cow::Borrowed($lemma),
            parent: like!($($parent)?),
            inherent: ::std::borrow::Cow::Borrowed(&[]),
            table: ::std::borrow::Cow::Borrowed(concat!(
                flexion::cell!($c0), "\t",
                flexion::cell!($c1), "\t",
                flexion::cell!($c2), "\t",
                flexion::cell!($c3), "\t",
                flexion::cell!($c4), "\t",
                flexion::cell!($c5), "\t",
                flexion::cell!($c6), "\t",
                flexion::cell!($c7), "\t",
                flexion::cell!($c8), "\t",
                flexion::cell!($c9), "\t",
                flexion::cell!($c10), "\t",
                flexion::cell!($c11), "\t",
                flexion::cell!($c12), "\t",
                flexion::cell!($c13), "\t",
                flexion::cell!($c14), "\t",
                flexion::cell!($c15), "\t",
                flexion::cell!($c16), "\t",
                flexion::cell!($c17), "\t",
                flexion::cell!($c18), "\t",
                flexion::cell!($c19), "\t",
                flexion::cell!($c20), "\t",
                flexion::cell!($c21), "\t",
                flexion::cell!($c22), "\t",
                flexion::cell!($c23), "\t",
                flexion::cell!($c24), "\t",
                flexion::cell!($c25), "\t",
                flexion::cell!($c26), "\t",
                flexion::cell!($c27), "\t",
                flexion::cell!($c28), "\t",
                flexion::cell!($c29), "\t",
                flexion::cell!($c30), "\t",
                flexion::cell!($c31), "\t",
                flexion::cell!($c32), "\t",
                flexion::cell!($c33), "\t",
                flexion::cell!($c34), "\t",
                flexion::cell!($c35), "\t",
                flexion::cell!($c36), "\t",
                flexion::cell!($c37), "\t",
                flexion::cell!($c38), "\t",
                flexion::cell!($c39), "\t",
                flexion::cell!($c40), "\t",
                flexion::cell!($c41), "\t",
                flexion::cell!($c42), "\t",
                flexion::cell!($c43), "\t",
                flexion::cell!($c44), "\t",
                flexion::cell!($c45), "\t",
                flexion::cell!($c46), "\t",
                flexion::cell!($c47), "\t",
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
    /// Like `этот`: 1 words.
    Этот,
    /// Like `сам`: 1 words.
    Сам,
    /// Like `тот`: 1 words.
    Тот,
    /// Like `иже`: 1 words.
    Иже,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 11] = [
        Declension::Оный,
        Declension::Свой,
        Declension::Чей,
        Declension::Иной,
        Declension::Некий,
        Declension::Другой,
        Declension::Наш,
        Declension::Этот,
        Declension::Сам,
        Declension::Тот,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 11] = [
    // Оный: 7 words (самый, который, каждый, некоторый, подобный, оный).
    declension!("оный", 7, "~ый";
        nominative_masculine: "~ый", "~ый";
        nominative_feminine: "~ая", "~ая";
        nominative_neuter: "~ое", "~ое";
        nominative_plural: "~ые", "~ые";
        genitive_masculine: "~ого", "~ого";
        genitive_feminine: "~ой", "~ой";
        genitive_neuter: "~ого", "~ого";
        genitive_plural: "~ых", "~ых";
        dative_masculine: "~ому", "~ому";
        dative_feminine: "~ой", "~ой";
        dative_neuter: "~ому", "~ому";
        dative_plural: "~ым", "~ым";
        accusative_masculine: "~ого", "~ый";
        accusative_feminine: "~ую", "~ую";
        accusative_neuter: "~ое", "~ое";
        accusative_plural: "~ых", "~ые";
        instrumental_masculine: "~ым", "~ым";
        instrumental_feminine: "~ой|~ою", "~ой|~ою";
        instrumental_neuter: "~ым", "~ым";
        instrumental_plural: "~ыми", "~ыми";
        prepositional_masculine: "~ом", "~ом";
        prepositional_feminine: "~ой", "~ой";
        prepositional_neuter: "~ом", "~ом";
        prepositional_plural: "~ых", "~ых";
    ),
    // Свой: 5 words (мой, твой, свой, кой, никой).
    declension!("свой", 5, "~й";
        nominative_masculine: "~й", "~й";
        nominative_feminine: "~я", "~я";
        nominative_neuter: "~ё", "~ё";
        nominative_plural: "~и", "~и";
        genitive_masculine: "~его", "~его";
        genitive_feminine: "~ей", "~ей";
        genitive_neuter: "~его", "~его";
        genitive_plural: "~их", "~их";
        dative_masculine: "~ему", "~ему";
        dative_feminine: "~ей", "~ей";
        dative_neuter: "~ему", "~ему";
        dative_plural: "~им", "~им";
        accusative_masculine: "~его", "~й";
        accusative_feminine: "~ю", "~ю";
        accusative_neuter: "~ё", "~ё";
        accusative_plural: "~их", "~и";
        instrumental_masculine: "~им", "~им";
        instrumental_feminine: "~ей|~ею", "~ей|~ею";
        instrumental_neuter: "~им", "~им";
        instrumental_plural: "~ими", "~ими";
        prepositional_masculine: "~ём", "~ём";
        prepositional_feminine: "~ей", "~ей";
        prepositional_neuter: "~ём", "~ём";
        prepositional_plural: "~их", "~их";
    ),
    // Чей: 4 words (чей, сей, ничей, кое-чей).
    declension!("чей", 4, "~ей";
        nominative_masculine: "~ей", "~ей";
        nominative_feminine: "~ья", "~ья";
        nominative_neuter: "~ьё", "~ьё";
        nominative_plural: "~ьи", "~ьи";
        genitive_masculine: "~ьего", "~ьего";
        genitive_feminine: "~ьей", "~ьей";
        genitive_neuter: "~ьего", "~ьего";
        genitive_plural: "~ьих", "~ьих";
        dative_masculine: "~ьему", "~ьему";
        dative_feminine: "~ьей", "~ьей";
        dative_neuter: "~ьему", "~ьему";
        dative_plural: "~ьим", "~ьим";
        accusative_masculine: "~ьего", "~ей";
        accusative_feminine: "~ью", "~ью";
        accusative_neuter: "~ьё", "~ьё";
        accusative_plural: "~ьих", "~ьи";
        instrumental_masculine: "~ьим", "~ьим";
        instrumental_feminine: "~ьей|~ьею", "~ьей|~ьею";
        instrumental_neuter: "~ьим", "~ьим";
        instrumental_plural: "~ьими", "~ьими";
        prepositional_masculine: "~ьём", "~ьём";
        prepositional_feminine: "~ьей", "~ьей";
        prepositional_neuter: "~ьём", "~ьём";
        prepositional_plural: "~ьих", "~ьих";
    ),
    // Иной: 4 words (иной, любой, таковой, каковой). Like Оный, except in 3 cells: nominative_masculine anim, nominative_masculine inan, accusative_masculine inan.
    declension!("иной", 4, "~ой", like Оный;
        nominative_masculine: "~ой", "~ой";
        nominative_feminine: _, _;
        nominative_neuter: _, _;
        nominative_plural: _, _;
        genitive_masculine: _, _;
        genitive_feminine: _, _;
        genitive_neuter: _, _;
        genitive_plural: _, _;
        dative_masculine: _, _;
        dative_feminine: _, _;
        dative_neuter: _, _;
        dative_plural: _, _;
        accusative_masculine: _, "~ой";
        accusative_feminine: _, _;
        accusative_neuter: _, _;
        accusative_plural: _, _;
        instrumental_masculine: _, _;
        instrumental_feminine: _, _;
        instrumental_neuter: _, _;
        instrumental_plural: _, _;
        prepositional_masculine: _, _;
        prepositional_feminine: _, _;
        prepositional_neuter: _, _;
        prepositional_plural: _, _;
    ),
    // Некий: 4 words (некий, всякий, этакий, эдакий). Like Оный, except in 19 cells: nominative_masculine anim, nominative_masculine inan, nominative_plural anim, nominative_plural inan, genitive_plural anim, genitive_plural inan, ….
    declension!("некий", 4, "~ий", like Оный;
        nominative_masculine: "~ий", "~ий";
        nominative_feminine: _, _;
        nominative_neuter: _, _;
        nominative_plural: "~ие", "~ие";
        genitive_masculine: _, _;
        genitive_feminine: _, _;
        genitive_neuter: _, _;
        genitive_plural: "~их", "~их";
        dative_masculine: _, _;
        dative_feminine: _, _;
        dative_neuter: _, _;
        dative_plural: "~им", "~им";
        accusative_masculine: _, "~ий";
        accusative_feminine: _, _;
        accusative_neuter: _, _;
        accusative_plural: "~их", "~ие";
        instrumental_masculine: "~им", "~им";
        instrumental_feminine: _, _;
        instrumental_neuter: "~им", "~им";
        instrumental_plural: "~ими", "~ими";
        prepositional_masculine: _, _;
        prepositional_feminine: _, _;
        prepositional_neuter: _, _;
        prepositional_plural: "~их", "~их";
    ),
    // Другой: 3 words (другой, кое-какой, никакой). Like Некий, except in 3 cells: nominative_masculine anim, nominative_masculine inan, accusative_masculine inan.
    declension!("другой", 3, "~ой", like Некий;
        nominative_masculine: "~ой", "~ой";
        nominative_feminine: _, _;
        nominative_neuter: _, _;
        nominative_plural: _, _;
        genitive_masculine: _, _;
        genitive_feminine: _, _;
        genitive_neuter: _, _;
        genitive_plural: _, _;
        dative_masculine: _, _;
        dative_feminine: _, _;
        dative_neuter: _, _;
        dative_plural: _, _;
        accusative_masculine: _, "~ой";
        accusative_feminine: _, _;
        accusative_neuter: _, _;
        accusative_plural: _, _;
        instrumental_masculine: _, _;
        instrumental_feminine: _, _;
        instrumental_neuter: _, _;
        instrumental_plural: _, _;
        prepositional_masculine: _, _;
        prepositional_feminine: _, _;
        prepositional_neuter: _, _;
        prepositional_plural: _, _;
    ),
    // Наш: 2 words (ваш, наш). Like Свой, except in 15 cells: nominative_masculine anim, nominative_masculine inan, nominative_feminine anim, nominative_feminine inan, nominative_neuter anim, nominative_neuter inan, ….
    declension!("наш", 2, "~", like Свой;
        nominative_masculine: "~", "~";
        nominative_feminine: "~а", "~а";
        nominative_neuter: "~е", "~е";
        nominative_plural: _, _;
        genitive_masculine: _, _;
        genitive_feminine: _, _;
        genitive_neuter: _, _;
        genitive_plural: _, _;
        dative_masculine: _, _;
        dative_feminine: _, _;
        dative_neuter: _, _;
        dative_plural: _, _;
        accusative_masculine: _, "~";
        accusative_feminine: "~у", "~у";
        accusative_neuter: "~е", "~е";
        accusative_plural: _, _;
        instrumental_masculine: _, _;
        instrumental_feminine: _, _;
        instrumental_neuter: _, _;
        instrumental_plural: _, _;
        prepositional_masculine: "~ем", "~ем";
        prepositional_feminine: _, _;
        prepositional_neuter: "~ем", "~ем";
        prepositional_plural: _, _;
    ),
    // Этот: 1 words (этот). Like Некий, except in 14 cells: nominative_masculine anim, nominative_masculine inan, nominative_feminine anim, nominative_feminine inan, nominative_neuter anim, nominative_neuter inan, ….
    declension!("этот", 1, "~от", like Некий;
        nominative_masculine: "~от", "~от";
        nominative_feminine: "~а", "~а";
        nominative_neuter: "~о", "~о";
        nominative_plural: "~и", "~и";
        genitive_masculine: _, _;
        genitive_feminine: _, _;
        genitive_neuter: _, _;
        genitive_plural: _, _;
        dative_masculine: _, _;
        dative_feminine: _, _;
        dative_neuter: _, _;
        dative_plural: _, _;
        accusative_masculine: _, "~от";
        accusative_feminine: "~у", "~у";
        accusative_neuter: "~о", "~о";
        accusative_plural: _, "~и";
        instrumental_masculine: _, _;
        instrumental_feminine: _, _;
        instrumental_neuter: _, _;
        instrumental_plural: _, _;
        prepositional_masculine: _, _;
        prepositional_feminine: _, _;
        prepositional_neuter: _, _;
        prepositional_plural: _, _;
    ),
    // Сам: 1 words (сам). Like Этот, except in 5 cells: nominative_masculine anim, nominative_masculine inan, accusative_masculine inan, accusative_feminine anim, accusative_feminine inan.
    declension!("сам", 1, "~", like Этот;
        nominative_masculine: "~", "~";
        nominative_feminine: _, _;
        nominative_neuter: _, _;
        nominative_plural: _, _;
        genitive_masculine: _, _;
        genitive_feminine: _, _;
        genitive_neuter: _, _;
        genitive_plural: _, _;
        dative_masculine: _, _;
        dative_feminine: _, _;
        dative_neuter: _, _;
        dative_plural: _, _;
        accusative_masculine: _, "~";
        accusative_feminine: "~у|~оё", "~у|~оё";
        accusative_neuter: _, _;
        accusative_plural: _, _;
        instrumental_masculine: _, _;
        instrumental_feminine: _, _;
        instrumental_neuter: _, _;
        instrumental_plural: _, _;
        prepositional_masculine: _, _;
        prepositional_feminine: _, _;
        prepositional_neuter: _, _;
        prepositional_plural: _, _;
    ),
    // Тот: 1 words (тот). Like Этот, except in 16 cells: nominative_plural anim, nominative_plural inan, genitive_plural anim, genitive_plural inan, dative_plural anim, dative_plural inan, ….
    declension!("тот", 1, "~от", like Этот;
        nominative_masculine: _, _;
        nominative_feminine: _, _;
        nominative_neuter: _, _;
        nominative_plural: "~е", "~е";
        genitive_masculine: _, _;
        genitive_feminine: _, _;
        genitive_neuter: _, _;
        genitive_plural: "~ех", "~ех";
        dative_masculine: _, _;
        dative_feminine: _, _;
        dative_neuter: _, _;
        dative_plural: "~ем", "~ем";
        accusative_masculine: _, _;
        accusative_feminine: _, _;
        accusative_neuter: _, _;
        accusative_plural: "~ех", "~е";
        instrumental_masculine: "~ем", "~ем";
        instrumental_feminine: _, _;
        instrumental_neuter: "~ем", "~ем";
        instrumental_plural: "~еми", "~еми";
        prepositional_masculine: _, _;
        prepositional_feminine: _, _;
        prepositional_neuter: _, _;
        prepositional_plural: "~ех", "~ех";
    ),
    // Иже: 1 words (иже).
    declension!("иже", 1, "и~";
        nominative_masculine: "и~", "и~";
        nominative_feminine: "я~", "я~";
        nominative_neuter: "е~", "е~";
        nominative_plural: "и~|я~", "и~|я~";
        genitive_masculine: "его~|него~", "его~|него~";
        genitive_feminine: "ея~|нея~", "ея~|нея~";
        genitive_neuter: "его~|него~", "его~|него~";
        genitive_plural: "их~|них~", "их~|них~";
        dative_masculine: "ему~|нему~", "ему~|нему~";
        dative_feminine: "ей~|ней~", "ей~|ней~";
        dative_neuter: "ему~|нему~", "ему~|нему~";
        dative_plural: "им~|ним~", "им~|ним~";
        accusative_masculine: "его~|него~", "и~|ни~";
        accusative_feminine: "ю~|ню~", "ю~|ню~";
        accusative_neuter: "е~|не~", "е~|не~";
        accusative_plural: "их~|них~", "и~|я~|ни~|ня~";
        instrumental_masculine: "им~|ним~", "им~|ним~";
        instrumental_feminine: "ею~|нею~", "ею~|нею~";
        instrumental_neuter: "им~|ним~", "им~|ним~";
        instrumental_plural: "ими~|ними~", "ими~|ними~";
        prepositional_masculine: "нем~", "нем~";
        prepositional_feminine: "ней~", "ней~";
        prepositional_neuter: "нем~", "нем~";
        prepositional_plural: "них~", "них~";
    ),
];
