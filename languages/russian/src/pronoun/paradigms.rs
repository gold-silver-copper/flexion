//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's feature says):
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
    ($name:literal, $words:literal, $lemma:literal;
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
            forms: ::std::borrow::Cow::Borrowed(&[
                flexion::form!($c0),
                flexion::form!($c1),
                flexion::form!($c2),
                flexion::form!($c3),
                flexion::form!($c4),
                flexion::form!($c5),
                flexion::form!($c6),
                flexion::form!($c7),
                flexion::form!($c8),
                flexion::form!($c9),
                flexion::form!($c10),
                flexion::form!($c11),
                flexion::form!($c12),
                flexion::form!($c13),
                flexion::form!($c14),
                flexion::form!($c15),
                flexion::form!($c16),
                flexion::form!($c17),
                flexion::form!($c18),
                flexion::form!($c19),
                flexion::form!($c20),
                flexion::form!($c21),
                flexion::form!($c22),
                flexion::form!($c23),
                flexion::form!($c24),
                flexion::form!($c25),
                flexion::form!($c26),
                flexion::form!($c27),
                flexion::form!($c28),
                flexion::form!($c29),
                flexion::form!($c30),
                flexion::form!($c31),
                flexion::form!($c32),
                flexion::form!($c33),
                flexion::form!($c34),
                flexion::form!($c35),
                flexion::form!($c36),
                flexion::form!($c37),
                flexion::form!($c38),
                flexion::form!($c39),
                flexion::form!($c40),
                flexion::form!($c41),
                flexion::form!($c42),
                flexion::form!($c43),
                flexion::form!($c44),
                flexion::form!($c45),
                flexion::form!($c46),
                flexion::form!($c47),
            ]),
        }
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
    // Иной: 4 words (иной, любой, таковой, каковой).
    declension!("иной", 4, "~ой";
        nominative_masculine: "~ой", "~ой";
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
        accusative_masculine: "~ого", "~ой";
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
    // Некий: 4 words (некий, всякий, этакий, эдакий).
    declension!("некий", 4, "~ий";
        nominative_masculine: "~ий", "~ий";
        nominative_feminine: "~ая", "~ая";
        nominative_neuter: "~ое", "~ое";
        nominative_plural: "~ие", "~ие";
        genitive_masculine: "~ого", "~ого";
        genitive_feminine: "~ой", "~ой";
        genitive_neuter: "~ого", "~ого";
        genitive_plural: "~их", "~их";
        dative_masculine: "~ому", "~ому";
        dative_feminine: "~ой", "~ой";
        dative_neuter: "~ому", "~ому";
        dative_plural: "~им", "~им";
        accusative_masculine: "~ого", "~ий";
        accusative_feminine: "~ую", "~ую";
        accusative_neuter: "~ое", "~ое";
        accusative_plural: "~их", "~ие";
        instrumental_masculine: "~им", "~им";
        instrumental_feminine: "~ой|~ою", "~ой|~ою";
        instrumental_neuter: "~им", "~им";
        instrumental_plural: "~ими", "~ими";
        prepositional_masculine: "~ом", "~ом";
        prepositional_feminine: "~ой", "~ой";
        prepositional_neuter: "~ом", "~ом";
        prepositional_plural: "~их", "~их";
    ),
    // Другой: 3 words (другой, кое-какой, никакой).
    declension!("другой", 3, "~ой";
        nominative_masculine: "~ой", "~ой";
        nominative_feminine: "~ая", "~ая";
        nominative_neuter: "~ое", "~ое";
        nominative_plural: "~ие", "~ие";
        genitive_masculine: "~ого", "~ого";
        genitive_feminine: "~ой", "~ой";
        genitive_neuter: "~ого", "~ого";
        genitive_plural: "~их", "~их";
        dative_masculine: "~ому", "~ому";
        dative_feminine: "~ой", "~ой";
        dative_neuter: "~ому", "~ому";
        dative_plural: "~им", "~им";
        accusative_masculine: "~ого", "~ой";
        accusative_feminine: "~ую", "~ую";
        accusative_neuter: "~ое", "~ое";
        accusative_plural: "~их", "~ие";
        instrumental_masculine: "~им", "~им";
        instrumental_feminine: "~ой|~ою", "~ой|~ою";
        instrumental_neuter: "~им", "~им";
        instrumental_plural: "~ими", "~ими";
        prepositional_masculine: "~ом", "~ом";
        prepositional_feminine: "~ой", "~ой";
        prepositional_neuter: "~ом", "~ом";
        prepositional_plural: "~их", "~их";
    ),
    // Наш: 2 words (ваш, наш).
    declension!("наш", 2, "~";
        nominative_masculine: "~", "~";
        nominative_feminine: "~а", "~а";
        nominative_neuter: "~е", "~е";
        nominative_plural: "~и", "~и";
        genitive_masculine: "~его", "~его";
        genitive_feminine: "~ей", "~ей";
        genitive_neuter: "~его", "~его";
        genitive_plural: "~их", "~их";
        dative_masculine: "~ему", "~ему";
        dative_feminine: "~ей", "~ей";
        dative_neuter: "~ему", "~ему";
        dative_plural: "~им", "~им";
        accusative_masculine: "~его", "~";
        accusative_feminine: "~у", "~у";
        accusative_neuter: "~е", "~е";
        accusative_plural: "~их", "~и";
        instrumental_masculine: "~им", "~им";
        instrumental_feminine: "~ей|~ею", "~ей|~ею";
        instrumental_neuter: "~им", "~им";
        instrumental_plural: "~ими", "~ими";
        prepositional_masculine: "~ем", "~ем";
        prepositional_feminine: "~ей", "~ей";
        prepositional_neuter: "~ем", "~ем";
        prepositional_plural: "~их", "~их";
    ),
    // Этот: 1 words (этот).
    declension!("этот", 1, "~от";
        nominative_masculine: "~от", "~от";
        nominative_feminine: "~а", "~а";
        nominative_neuter: "~о", "~о";
        nominative_plural: "~и", "~и";
        genitive_masculine: "~ого", "~ого";
        genitive_feminine: "~ой", "~ой";
        genitive_neuter: "~ого", "~ого";
        genitive_plural: "~их", "~их";
        dative_masculine: "~ому", "~ому";
        dative_feminine: "~ой", "~ой";
        dative_neuter: "~ому", "~ому";
        dative_plural: "~им", "~им";
        accusative_masculine: "~ого", "~от";
        accusative_feminine: "~у", "~у";
        accusative_neuter: "~о", "~о";
        accusative_plural: "~их", "~и";
        instrumental_masculine: "~им", "~им";
        instrumental_feminine: "~ой|~ою", "~ой|~ою";
        instrumental_neuter: "~им", "~им";
        instrumental_plural: "~ими", "~ими";
        prepositional_masculine: "~ом", "~ом";
        prepositional_feminine: "~ой", "~ой";
        prepositional_neuter: "~ом", "~ом";
        prepositional_plural: "~их", "~их";
    ),
    // Сам: 1 words (сам).
    declension!("сам", 1, "~";
        nominative_masculine: "~", "~";
        nominative_feminine: "~а", "~а";
        nominative_neuter: "~о", "~о";
        nominative_plural: "~и", "~и";
        genitive_masculine: "~ого", "~ого";
        genitive_feminine: "~ой", "~ой";
        genitive_neuter: "~ого", "~ого";
        genitive_plural: "~их", "~их";
        dative_masculine: "~ому", "~ому";
        dative_feminine: "~ой", "~ой";
        dative_neuter: "~ому", "~ому";
        dative_plural: "~им", "~им";
        accusative_masculine: "~ого", "~";
        accusative_feminine: "~у|~оё", "~у|~оё";
        accusative_neuter: "~о", "~о";
        accusative_plural: "~их", "~и";
        instrumental_masculine: "~им", "~им";
        instrumental_feminine: "~ой|~ою", "~ой|~ою";
        instrumental_neuter: "~им", "~им";
        instrumental_plural: "~ими", "~ими";
        prepositional_masculine: "~ом", "~ом";
        prepositional_feminine: "~ой", "~ой";
        prepositional_neuter: "~ом", "~ом";
        prepositional_plural: "~их", "~их";
    ),
    // Тот: 1 words (тот).
    declension!("тот", 1, "~от";
        nominative_masculine: "~от", "~от";
        nominative_feminine: "~а", "~а";
        nominative_neuter: "~о", "~о";
        nominative_plural: "~е", "~е";
        genitive_masculine: "~ого", "~ого";
        genitive_feminine: "~ой", "~ой";
        genitive_neuter: "~ого", "~ого";
        genitive_plural: "~ех", "~ех";
        dative_masculine: "~ому", "~ому";
        dative_feminine: "~ой", "~ой";
        dative_neuter: "~ому", "~ому";
        dative_plural: "~ем", "~ем";
        accusative_masculine: "~ого", "~от";
        accusative_feminine: "~у", "~у";
        accusative_neuter: "~о", "~о";
        accusative_plural: "~ех", "~е";
        instrumental_masculine: "~ем", "~ем";
        instrumental_feminine: "~ой|~ою", "~ой|~ою";
        instrumental_neuter: "~ем", "~ем";
        instrumental_plural: "~еми", "~еми";
        prepositional_masculine: "~ом", "~ом";
        prepositional_feminine: "~ой", "~ой";
        prepositional_neuter: "~ом", "~ом";
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
