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
/// `short_adjective` (m, f, n, pl)
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
     short_adjective: $c48:tt, $c49:tt, $c50:tt, $c51:tt;
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
                flexion::cell!($c48), "\t",
                flexion::cell!($c49), "\t",
                flexion::cell!($c50), "\t",
                flexion::cell!($c51), "\t",
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
    /// Like `сведущий`: 24,315 words.
    Сведущий,
    /// Like `уместный`: 11,284 words.
    Уместный,
    /// Like `устарелый`: 9,274 words.
    Устарелый,
    /// Like `пылкий`: 8,180 words.
    Пылкий,
    /// Like `снижающийся`: 5,127 words.
    Снижающийся,
    /// Like `подверженный`: 3,679 words.
    Подверженный,
    /// Like `вольный`: 2,209 words.
    Вольный,
    /// Like `казнённый`: 987 words.
    Казнённый,
    /// Like `прямой`: 647 words.
    Прямой,
    /// Like `непринуждённый`: 453 words.
    Непринуждённый,
    /// Like `показной`: 449 words.
    Показной,
    /// Like `тайный`: 264 words.
    Тайный,
    /// Like `лисий`: 224 words.
    Лисий,
    /// Like `средний`: 177 words.
    Средний,
    /// Like `родственный`: 134 words.
    Родственный,
    /// Like `иродов`: 68 words.
    Иродов,
    /// Like `горький`: 68 words.
    Горький,
    /// Like `бойкий`: 21 words.
    Бойкий,
    /// Like `куцый`: 19 words.
    Куцый,
    /// Like `дядин`: 14 words.
    Дядин,
    /// Like `рад`: 12 words.
    Рад,
    /// Like `синий`: 10 words.
    Синий,
    /// Like `острый`: 9 words.
    Острый,
    /// Like `длинношеий`: 8 words.
    Длинношеий,
    /// Like `чёрный`: 6 words.
    Чёрный,
    /// Like `твёрдый`: 6 words.
    Твёрдый,
    /// Like `должен`: 5 words.
    Должен,
    /// Like `тяжкий`: 5 words.
    Тяжкий,
    /// Like `горячий`: 4 words.
    Горячий,
    /// Like `злой`: 4 words.
    Злой,
    /// Like `лёгкий`: 4 words.
    Лёгкий,
    /// Like `заёбанный`: 4 words.
    Заёбанный,
    /// Like `высокий`: 3 words.
    Высокий,
    /// Like `дешёвый`: 3 words.
    Дешёвый,
    /// Like `зелёный`: 3 words.
    Зелёный,
    /// Like `катин`: 3 words.
    Катин,
    /// Like `крутенек`: 3 words.
    Крутенек,
    /// Like `сланцевый`: 3 words.
    Сланцевый,
    /// Like `жерёбый`: 3 words.
    Жерёбый,
    /// Like `теплонагревательный`: 3 words.
    Теплонагревательный,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 40] = [
        Declension::Сведущий,
        Declension::Уместный,
        Declension::Устарелый,
        Declension::Пылкий,
        Declension::Снижающийся,
        Declension::Подверженный,
        Declension::Вольный,
        Declension::Казнённый,
        Declension::Прямой,
        Declension::Непринуждённый,
        Declension::Показной,
        Declension::Тайный,
        Declension::Лисий,
        Declension::Средний,
        Declension::Родственный,
        Declension::Иродов,
        Declension::Горький,
        Declension::Бойкий,
        Declension::Куцый,
        Declension::Дядин,
        Declension::Рад,
        Declension::Синий,
        Declension::Острый,
        Declension::Длинношеий,
        Declension::Чёрный,
        Declension::Твёрдый,
        Declension::Должен,
        Declension::Тяжкий,
        Declension::Горячий,
        Declension::Злой,
        Declension::Лёгкий,
        Declension::Заёбанный,
        Declension::Высокий,
        Declension::Дешёвый,
        Declension::Зелёный,
        Declension::Катин,
        Declension::Крутенек,
        Declension::Сланцевый,
        Declension::Жерёбый,
        Declension::Теплонагревательный,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 40] = [
    // Сведущий: 24,315 words (деревообрабатывающий, близорукий, великий, будущий, лесообрабатывающий, настоящий).
    declension!("сведущий", 24315, "~ий";
        nominative_masculine: "~ий", "~ий";
        nominative_feminine: "~ая", "~ая";
        nominative_neuter: "~ее", "~ее";
        nominative_plural: "~ие", "~ие";
        genitive_masculine: "~его", "~его";
        genitive_feminine: "~ей", "~ей";
        genitive_neuter: "~его", "~его";
        genitive_plural: "~их", "~их";
        dative_masculine: "~ему", "~ему";
        dative_feminine: "~ей", "~ей";
        dative_neuter: "~ему", "~ему";
        dative_plural: "~им", "~им";
        accusative_masculine: "~его", "~ий";
        accusative_feminine: "~ую", "~ую";
        accusative_neuter: "~ее", "~ее";
        accusative_plural: "~их", "~ие";
        instrumental_masculine: "~им", "~им";
        instrumental_feminine: "~ей|~ею", "~ей|~ею";
        instrumental_neuter: "~им", "~им";
        instrumental_plural: "~ими", "~ими";
        prepositional_masculine: "~ем", "~ем";
        prepositional_feminine: "~ей", "~ей";
        prepositional_neuter: "~ем", "~ем";
        prepositional_plural: "~их", "~их";
        short_adjective: "~", "~а", "~е", "~и";
    ),
    // Уместный: 11,284 words (браный, славный, крохотный, деревянный, древесный, лесоводственный).
    declension!("уместный", 11284, "~~ый";
        nominative_masculine: "~~ый", "~~ый";
        nominative_feminine: "~~ая", "~~ая";
        nominative_neuter: "~~ое", "~~ое";
        nominative_plural: "~~ые", "~~ые";
        genitive_masculine: "~~ого", "~~ого";
        genitive_feminine: "~~ой", "~~ой";
        genitive_neuter: "~~ого", "~~ого";
        genitive_plural: "~~ых", "~~ых";
        dative_masculine: "~~ому", "~~ому";
        dative_feminine: "~~ой", "~~ой";
        dative_neuter: "~~ому", "~~ому";
        dative_plural: "~~ым", "~~ым";
        accusative_masculine: "~~ого", "~~ый";
        accusative_feminine: "~~ую", "~~ую";
        accusative_neuter: "~~ое", "~~ое";
        accusative_plural: "~~ых", "~~ые";
        instrumental_masculine: "~~ым", "~~ым";
        instrumental_feminine: "~~ой|~~ою", "~~ой|~~ою";
        instrumental_neuter: "~~ым", "~~ым";
        instrumental_plural: "~~ыми", "~~ыми";
        prepositional_masculine: "~~ом", "~~ом";
        prepositional_feminine: "~~ой", "~~ой";
        prepositional_neuter: "~~ом", "~~ом";
        prepositional_plural: "~~ых", "~~ых";
        short_adjective: "~е~", "~~а", "~~о", "~~ы";
    ),
    // Устарелый: 9,274 words (слабый, ласковый, деревянистый, лесистый, лесотундровый, тундровый).
    declension!("устарелый", 9274, "~ый";
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
        short_adjective: "~", "~а", "~о", "~ы";
    ),
    // Пылкий: 8,180 words (русский, тайский, жалкий, лесоводческий, лесохимический, лесотехнический). Like Уместный, except in 23 cells: nominative_masculine anim, nominative_masculine inan, nominative_plural anim, nominative_plural inan, genitive_plural anim, genitive_plural inan, ….
    declension!("пылкий", 8180, "~~ий", like Уместный;
        nominative_masculine: "~~ий", "~~ий";
        nominative_feminine: _, _;
        nominative_neuter: _, _;
        nominative_plural: "~~ие", "~~ие";
        genitive_masculine: _, _;
        genitive_feminine: _, _;
        genitive_neuter: _, _;
        genitive_plural: "~~их", "~~их";
        dative_masculine: _, _;
        dative_feminine: _, _;
        dative_neuter: _, _;
        dative_plural: "~~им", "~~им";
        accusative_masculine: _, "~~ий";
        accusative_feminine: _, _;
        accusative_neuter: _, _;
        accusative_plural: "~~их", "~~ие";
        instrumental_masculine: "~~им", "~~им";
        instrumental_feminine: "~~ой", "~~ой";
        instrumental_neuter: "~~им", "~~им";
        instrumental_plural: "~~ими", "~~ими";
        prepositional_masculine: _, _;
        prepositional_feminine: _, _;
        prepositional_neuter: _, _;
        prepositional_plural: "~~их", "~~их";
        short_adjective: "~о~", _, _, "~~и";
    ),
    // Снижающийся: 5,127 words (снижающийся, невзрывающийся, неразорвавшийся, учащийся, трудящийся, полуразвалившийся).
    declension!("снижающийся", 5127, "~ий~";
        nominative_masculine: "~ий~", "~ий~";
        nominative_feminine: "~ая~", "~ая~";
        nominative_neuter: "~ее~", "~ее~";
        nominative_plural: "~ие~", "~ие~";
        genitive_masculine: "~его~", "~его~";
        genitive_feminine: "~ей~", "~ей~";
        genitive_neuter: "~его~", "~его~";
        genitive_plural: "~их~", "~их~";
        dative_masculine: "~ему~", "~ему~";
        dative_feminine: "~ей~", "~ей~";
        dative_neuter: "~ему~", "~ему~";
        dative_plural: "~им~", "~им~";
        accusative_masculine: "~его~", "~ий~";
        accusative_feminine: "~ую~", "~ую~";
        accusative_neuter: "~ее~", "~ее~";
        accusative_plural: "~их~", "~ие~";
        instrumental_masculine: "~им~", "~им~";
        instrumental_feminine: "~ей~|~ею~", "~ей~|~ею~";
        instrumental_neuter: "~им~", "~им~";
        instrumental_plural: "~ими~", "~ими~";
        prepositional_masculine: "~ем~", "~ем~";
        prepositional_feminine: "~ей~", "~ей~";
        prepositional_neuter: "~ем~", "~ем~";
        prepositional_plural: "~их~", "~их~";
        short_adjective: derived, derived, derived, derived;
    ),
    // Подверженный: 3,679 words (использованный, ученный, неприготовленный, непривязанный, неупакованный, неутрамбованный).
    declension!("подверженный", 3679, "~ный";
        nominative_masculine: "~ный", "~ный";
        nominative_feminine: "~ная", "~ная";
        nominative_neuter: "~ное", "~ное";
        nominative_plural: "~ные", "~ные";
        genitive_masculine: "~ного", "~ного";
        genitive_feminine: "~ной", "~ной";
        genitive_neuter: "~ного", "~ного";
        genitive_plural: "~ных", "~ных";
        dative_masculine: "~ному", "~ному";
        dative_feminine: "~ной", "~ной";
        dative_neuter: "~ному", "~ному";
        dative_plural: "~ным", "~ным";
        accusative_masculine: "~ного", "~ный";
        accusative_feminine: "~ную", "~ную";
        accusative_neuter: "~ное", "~ное";
        accusative_plural: "~ных", "~ные";
        instrumental_masculine: "~ным", "~ным";
        instrumental_feminine: "~ной|~ною", "~ной|~ною";
        instrumental_neuter: "~ным", "~ным";
        instrumental_plural: "~ными", "~ными";
        prepositional_masculine: "~ном", "~ном";
        prepositional_feminine: "~ной", "~ной";
        prepositional_neuter: "~ном", "~ном";
        prepositional_plural: "~ных", "~ных";
        short_adjective: "~", "~а", "~о", "~ы";
    ),
    // Вольный: 2,209 words (примирительный, буквальный, утомительный, охуительный, действительный, изумительный).
    declension!("вольный", 2209, "~ь~ый";
        nominative_masculine: "~ь~ый", "~ь~ый";
        nominative_feminine: "~ь~ая", "~ь~ая";
        nominative_neuter: "~ь~ое", "~ь~ое";
        nominative_plural: "~ь~ые", "~ь~ые";
        genitive_masculine: "~ь~ого", "~ь~ого";
        genitive_feminine: "~ь~ой", "~ь~ой";
        genitive_neuter: "~ь~ого", "~ь~ого";
        genitive_plural: "~ь~ых", "~ь~ых";
        dative_masculine: "~ь~ому", "~ь~ому";
        dative_feminine: "~ь~ой", "~ь~ой";
        dative_neuter: "~ь~ому", "~ь~ому";
        dative_plural: "~ь~ым", "~ь~ым";
        accusative_masculine: "~ь~ого", "~ь~ый";
        accusative_feminine: "~ь~ую", "~ь~ую";
        accusative_neuter: "~ь~ое", "~ь~ое";
        accusative_plural: "~ь~ых", "~ь~ые";
        instrumental_masculine: "~ь~ым", "~ь~ым";
        instrumental_feminine: "~ь~ой|~ь~ою", "~ь~ой|~ь~ою";
        instrumental_neuter: "~ь~ым", "~ь~ым";
        instrumental_plural: "~ь~ыми", "~ь~ыми";
        prepositional_masculine: "~ь~ом", "~ь~ом";
        prepositional_feminine: "~ь~ой", "~ь~ой";
        prepositional_neuter: "~ь~ом", "~ь~ом";
        prepositional_plural: "~ь~ых", "~ь~ых";
        short_adjective: "~е~", "~ь~а", "~ь~о", "~ь~ы";
    ),
    // Казнённый: 987 words (соединённый, обызвествлённый, одушевлённый, объединённый, приобретённый, неприкреплённый).
    declension!("казнённый", 987, "~ё~ный";
        nominative_masculine: "~ё~ный", "~ё~ный";
        nominative_feminine: "~ё~ная", "~ё~ная";
        nominative_neuter: "~ё~ное", "~ё~ное";
        nominative_plural: "~ё~ные", "~ё~ные";
        genitive_masculine: "~ё~ного", "~ё~ного";
        genitive_feminine: "~ё~ной", "~ё~ной";
        genitive_neuter: "~ё~ного", "~ё~ного";
        genitive_plural: "~ё~ных", "~ё~ных";
        dative_masculine: "~ё~ному", "~ё~ному";
        dative_feminine: "~ё~ной", "~ё~ной";
        dative_neuter: "~ё~ному", "~ё~ному";
        dative_plural: "~ё~ным", "~ё~ным";
        accusative_masculine: "~ё~ного", "~ё~ный";
        accusative_feminine: "~ё~ную", "~ё~ную";
        accusative_neuter: "~ё~ное", "~ё~ное";
        accusative_plural: "~ё~ных", "~ё~ные";
        instrumental_masculine: "~ё~ным", "~ё~ным";
        instrumental_feminine: "~ё~ной|~ё~ною", "~ё~ной|~ё~ною";
        instrumental_neuter: "~ё~ным", "~ё~ным";
        instrumental_plural: "~ё~ными", "~ё~ными";
        prepositional_masculine: "~ё~ном", "~ё~ном";
        prepositional_feminine: "~ё~ной", "~ё~ной";
        prepositional_neuter: "~ё~ном", "~ё~ном";
        prepositional_plural: "~ё~ных", "~ё~ных";
        short_adjective: "~ё~", "~е~а", "~е~о", "~е~ы";
    ),
    // Прямой: 647 words (большой, баской, морской, городской, лесовой, былой). Like Устарелый, except in 3 cells: nominative_masculine anim, nominative_masculine inan, accusative_masculine inan.
    declension!("прямой", 647, "~ой", like Устарелый;
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
        short_adjective: _, _, _, _;
    ),
    // Непринуждённый: 453 words (сверхъестественный, неодушевлённый, несдержанный, неожиданный, недисциплинированный, непредвиденный). Like Подверженный, except in 3 cells: short_adjective f, short_adjective n, short_adjective pl.
    declension!("непринуждённый", 453, "~ный", like Подверженный;
        nominative_masculine: _, _;
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
        accusative_masculine: _, _;
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
        short_adjective: _, "~на", "~но", "~ны";
    ),
    // Показной: 449 words (лесной, лесостепной, дровяной, губной, стальной, нефтяной). Like Уместный, except in 5 cells: nominative_masculine anim, nominative_masculine inan, accusative_masculine inan, instrumental_feminine anim, instrumental_feminine inan.
    declension!("показной", 449, "~~ой", like Уместный;
        nominative_masculine: "~~ой", "~~ой";
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
        accusative_masculine: _, "~~ой";
        accusative_feminine: _, _;
        accusative_neuter: _, _;
        accusative_plural: _, _;
        instrumental_masculine: _, _;
        instrumental_feminine: "~~ой", "~~ой";
        instrumental_neuter: _, _;
        instrumental_plural: _, _;
        prepositional_masculine: _, _;
        prepositional_feminine: _, _;
        prepositional_neuter: _, _;
        prepositional_plural: _, _;
        short_adjective: _, _, _, _;
    ),
    // Тайный: 264 words (покойный, спокойный, семейный, случайный, партийный, комедийный).
    declension!("тайный", 264, "~й~ый";
        nominative_masculine: "~й~ый", "~й~ый";
        nominative_feminine: "~й~ая", "~й~ая";
        nominative_neuter: "~й~ое", "~й~ое";
        nominative_plural: "~й~ые", "~й~ые";
        genitive_masculine: "~й~ого", "~й~ого";
        genitive_feminine: "~й~ой", "~й~ой";
        genitive_neuter: "~й~ого", "~й~ого";
        genitive_plural: "~й~ых", "~й~ых";
        dative_masculine: "~й~ому", "~й~ому";
        dative_feminine: "~й~ой", "~й~ой";
        dative_neuter: "~й~ому", "~й~ому";
        dative_plural: "~й~ым", "~й~ым";
        accusative_masculine: "~й~ого", "~й~ый";
        accusative_feminine: "~й~ую", "~й~ую";
        accusative_neuter: "~й~ое", "~й~ое";
        accusative_plural: "~й~ых", "~й~ые";
        instrumental_masculine: "~й~ым", "~й~ым";
        instrumental_feminine: "~й~ой|~й~ою", "~й~ой|~й~ою";
        instrumental_neuter: "~й~ым", "~й~ым";
        instrumental_plural: "~й~ыми", "~й~ыми";
        prepositional_masculine: "~й~ом", "~й~ом";
        prepositional_feminine: "~й~ой", "~й~ой";
        prepositional_neuter: "~й~ом", "~й~ом";
        prepositional_plural: "~й~ых", "~й~ых";
        short_adjective: "~е~", "~й~а", "~й~о", "~й~ы";
    ),
    // Лисий: 224 words (бабий, лесничий, третий, собачий, заячий, рыбий).
    declension!("лисий", 224, "~ий";
        nominative_masculine: "~ий", "~ий";
        nominative_feminine: "~ья", "~ья";
        nominative_neuter: "~ье", "~ье";
        nominative_plural: "~ьи", "~ьи";
        genitive_masculine: "~ьего", "~ьего";
        genitive_feminine: "~ьей", "~ьей";
        genitive_neuter: "~ьего", "~ьего";
        genitive_plural: "~ьих", "~ьих";
        dative_masculine: "~ьему", "~ьему";
        dative_feminine: "~ьей", "~ьей";
        dative_neuter: "~ьему", "~ьему";
        dative_plural: "~ьим", "~ьим";
        accusative_masculine: "~ьего", "~ий";
        accusative_feminine: "~ью", "~ью";
        accusative_neuter: "~ье", "~ье";
        accusative_plural: "~ьих", "~ьи";
        instrumental_masculine: "~ьим", "~ьим";
        instrumental_feminine: "~ьей|~ьею", "~ьей|~ьею";
        instrumental_neuter: "~ьим", "~ьим";
        instrumental_plural: "~ьими", "~ьими";
        prepositional_masculine: "~ьем", "~ьем";
        prepositional_feminine: "~ьей", "~ьей";
        prepositional_neuter: "~ьем", "~ьем";
        prepositional_plural: "~ьих", "~ьих";
        short_adjective: derived, derived, derived, derived;
    ),
    // Средний: 177 words (субботний, средний, последний, задний, предпоследний, посторонний).
    declension!("средний", 177, "~~ий";
        nominative_masculine: "~~ий", "~~ий";
        nominative_feminine: "~~яя", "~~яя";
        nominative_neuter: "~~ее", "~~ее";
        nominative_plural: "~~ие", "~~ие";
        genitive_masculine: "~~его", "~~его";
        genitive_feminine: "~~ей", "~~ей";
        genitive_neuter: "~~его", "~~его";
        genitive_plural: "~~их", "~~их";
        dative_masculine: "~~ему", "~~ему";
        dative_feminine: "~~ей", "~~ей";
        dative_neuter: "~~ему", "~~ему";
        dative_plural: "~~им", "~~им";
        accusative_masculine: "~~его", "~~ий";
        accusative_feminine: "~~юю", "~~юю";
        accusative_neuter: "~~ее", "~~ее";
        accusative_plural: "~~их", "~~ие";
        instrumental_masculine: "~~им", "~~им";
        instrumental_feminine: "~~ей", "~~ей";
        instrumental_neuter: "~~им", "~~им";
        instrumental_plural: "~~ими", "~~ими";
        prepositional_masculine: "~~ем", "~~ем";
        prepositional_feminine: "~~ей", "~~ей";
        prepositional_neuter: "~~ем", "~~ем";
        prepositional_plural: "~~их", "~~их";
        short_adjective: "~е~", "~~я", "~~е", "~~и";
    ),
    // Родственный: 134 words (государственный, собственный, медленный, священный, художественный, производственный). Like Непринуждённый, except in 1 cell: short_adjective m.
    declension!("родственный", 134, "~ный", like Непринуждённый;
        nominative_masculine: _, _;
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
        accusative_masculine: _, _;
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
        short_adjective: "~ен|~", _, _, _;
    ),
    // Иродов: 68 words (сукин, христов, адамов, хуев, булев, отцов).
    declension!("иродов", 68, "~";
        nominative_masculine: "~", "~";
        nominative_feminine: "~а", "~а";
        nominative_neuter: "~о", "~о";
        nominative_plural: "~ы", "~ы";
        genitive_masculine: "~а", "~а";
        genitive_feminine: "~ой", "~ой";
        genitive_neuter: "~а", "~а";
        genitive_plural: "~ых", "~ых";
        dative_masculine: "~у", "~у";
        dative_feminine: "~ой", "~ой";
        dative_neuter: "~у", "~у";
        dative_plural: "~ым", "~ым";
        accusative_masculine: "~а", "~";
        accusative_feminine: "~у", "~у";
        accusative_neuter: "~о", "~о";
        accusative_plural: "~ых", "~ы";
        instrumental_masculine: "~ым", "~ым";
        instrumental_feminine: "~ой|~ою", "~ой|~ою";
        instrumental_neuter: "~ым", "~ым";
        instrumental_plural: "~ыми", "~ыми";
        prepositional_masculine: "~ом", "~ом";
        prepositional_feminine: "~ой", "~ой";
        prepositional_neuter: "~ом", "~ом";
        prepositional_plural: "~ых", "~ых";
        short_adjective: derived, derived, derived, derived;
    ),
    // Горький: 68 words (горький, малюсенький, целёхонький, близёхонький, близёшенький, сладенький). Like Вольный, except in 20 cells: nominative_masculine anim, nominative_masculine inan, nominative_plural anim, nominative_plural inan, genitive_plural anim, genitive_plural inan, ….
    declension!("горький", 68, "~ь~ий", like Вольный;
        nominative_masculine: "~ь~ий", "~ь~ий";
        nominative_feminine: _, _;
        nominative_neuter: _, _;
        nominative_plural: "~ь~ие", "~ь~ие";
        genitive_masculine: _, _;
        genitive_feminine: _, _;
        genitive_neuter: _, _;
        genitive_plural: "~ь~их", "~ь~их";
        dative_masculine: _, _;
        dative_feminine: _, _;
        dative_neuter: _, _;
        dative_plural: "~ь~им", "~ь~им";
        accusative_masculine: _, "~ь~ий";
        accusative_feminine: _, _;
        accusative_neuter: _, _;
        accusative_plural: "~ь~их", "~ь~ие";
        instrumental_masculine: "~ь~им", "~ь~им";
        instrumental_feminine: _, _;
        instrumental_neuter: "~ь~им", "~ь~им";
        instrumental_plural: "~ь~ими", "~ь~ими";
        prepositional_masculine: _, _;
        prepositional_feminine: _, _;
        prepositional_neuter: _, _;
        prepositional_plural: "~ь~их", "~ь~их";
        short_adjective: _, _, _, "~ь~и";
    ),
    // Бойкий: 21 words (бойкий, клейкий, стойкий, износостойкий, огнестойкий, морозостойкий). Like Тайный, except in 20 cells: nominative_masculine anim, nominative_masculine inan, nominative_plural anim, nominative_plural inan, genitive_plural anim, genitive_plural inan, ….
    declension!("бойкий", 21, "~й~ий", like Тайный;
        nominative_masculine: "~й~ий", "~й~ий";
        nominative_feminine: _, _;
        nominative_neuter: _, _;
        nominative_plural: "~й~ие", "~й~ие";
        genitive_masculine: _, _;
        genitive_feminine: _, _;
        genitive_neuter: _, _;
        genitive_plural: "~й~их", "~й~их";
        dative_masculine: _, _;
        dative_feminine: _, _;
        dative_neuter: _, _;
        dative_plural: "~й~им", "~й~им";
        accusative_masculine: _, "~й~ий";
        accusative_feminine: _, _;
        accusative_neuter: _, _;
        accusative_plural: "~й~их", "~й~ие";
        instrumental_masculine: "~й~им", "~й~им";
        instrumental_feminine: _, _;
        instrumental_neuter: "~й~им", "~й~им";
        instrumental_plural: "~й~ими", "~й~ими";
        prepositional_masculine: _, _;
        prepositional_feminine: _, _;
        prepositional_neuter: _, _;
        prepositional_plural: "~й~их", "~й~их";
        short_adjective: _, _, _, "~й~и";
    ),
    // Куцый: 19 words (бледнолицый, куцый, круглолицый, пухлолицый, багроволицый, белолицый). Like Сведущий, except in 20 cells: nominative_masculine anim, nominative_masculine inan, nominative_plural anim, nominative_plural inan, genitive_plural anim, genitive_plural inan, ….
    declension!("куцый", 19, "~ый", like Сведущий;
        nominative_masculine: "~ый", "~ый";
        nominative_feminine: _, _;
        nominative_neuter: _, _;
        nominative_plural: "~ые", "~ые";
        genitive_masculine: _, _;
        genitive_feminine: _, _;
        genitive_neuter: _, _;
        genitive_plural: "~ых", "~ых";
        dative_masculine: _, _;
        dative_feminine: _, _;
        dative_neuter: _, _;
        dative_plural: "~ым", "~ым";
        accusative_masculine: _, "~ый";
        accusative_feminine: _, _;
        accusative_neuter: _, _;
        accusative_plural: "~ых", "~ые";
        instrumental_masculine: "~ым", "~ым";
        instrumental_feminine: _, _;
        instrumental_neuter: "~ым", "~ым";
        instrumental_plural: "~ыми", "~ыми";
        prepositional_masculine: _, _;
        prepositional_feminine: _, _;
        prepositional_neuter: _, _;
        prepositional_plural: "~ых", "~ых";
        short_adjective: _, _, _, "~ы";
    ),
    // Дядин: 14 words (мамин, женин, никитин, дядин, бабушкин, папин). Like Иродов, except in 9 cells: genitive_masculine anim, genitive_masculine inan, genitive_neuter anim, genitive_neuter inan, dative_masculine anim, dative_masculine inan, ….
    declension!("дядин", 14, "~", like Иродов;
        nominative_masculine: _, _;
        nominative_feminine: _, _;
        nominative_neuter: _, _;
        nominative_plural: _, _;
        genitive_masculine: "~ого", "~ого";
        genitive_feminine: _, _;
        genitive_neuter: "~ого", "~ого";
        genitive_plural: _, _;
        dative_masculine: "~ому", "~ому";
        dative_feminine: _, _;
        dative_neuter: "~ому", "~ому";
        dative_plural: _, _;
        accusative_masculine: "~ого", _;
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
        short_adjective: _, _, _, _;
    ),
    // Рад: 12 words (широк, мал, рад, велик, намерен, наслышан).
    declension!("рад", 12, "~";
        nominative_masculine: "~", "~";
        nominative_feminine: derived, derived;
        nominative_neuter: derived, derived;
        nominative_plural: derived, derived;
        genitive_masculine: derived, derived;
        genitive_feminine: derived, derived;
        genitive_neuter: derived, derived;
        genitive_plural: derived, derived;
        dative_masculine: derived, derived;
        dative_feminine: derived, derived;
        dative_neuter: derived, derived;
        dative_plural: derived, derived;
        accusative_masculine: derived, "~";
        accusative_feminine: derived, derived;
        accusative_neuter: derived, derived;
        accusative_plural: derived, derived;
        instrumental_masculine: derived, derived;
        instrumental_feminine: derived, derived;
        instrumental_neuter: derived, derived;
        instrumental_plural: derived, derived;
        prepositional_masculine: derived, derived;
        prepositional_feminine: derived, derived;
        prepositional_neuter: derived, derived;
        prepositional_plural: derived, derived;
        short_adjective: "~", "~а", "~о", "~ы";
    ),
    // Синий: 10 words (синий, лососий, тёмно-синий, крокодилий, сверхдальний, выхухолий). Like Сведущий, except in 6 cells: nominative_feminine anim, nominative_feminine inan, accusative_feminine anim, accusative_feminine inan, short_adjective m, short_adjective f.
    declension!("синий", 10, "~ий", like Сведущий;
        nominative_masculine: _, _;
        nominative_feminine: "~яя", "~яя";
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
        accusative_masculine: _, _;
        accusative_feminine: "~юю", "~юю";
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
        short_adjective: "~ь", "~я", _, _;
    ),
    // Острый: 9 words (острый, неумный, умный, хитрый, шустрый, нехитрый). Like Уместный, except in 3 cells: short_adjective m, short_adjective n, short_adjective pl.
    declension!("острый", 9, "~~ый", like Уместный;
        nominative_masculine: _, _;
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
        accusative_masculine: _, _;
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
        short_adjective: "~ё~", _, "~~о|~~о", "~~ы|~~ы";
    ),
    // Длинношеий: 8 words (длинношеий, голошеий, долгошеий, короткошеий, кривошеий, толстошеий). Like Синий, except in 1 cell: short_adjective m.
    declension!("длинношеий", 8, "~ий", like Синий;
        nominative_masculine: _, _;
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
        accusative_masculine: _, _;
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
        short_adjective: "~й", _, _, _;
    ),
    // Чёрный: 6 words (чёрный, тёмный, тёплый, полутёмный, нечёрный, белочёрный).
    declension!("чёрный", 6, "~ё~~ый";
        nominative_masculine: "~ё~~ый", "~ё~~ый";
        nominative_feminine: "~ё~~ая", "~ё~~ая";
        nominative_neuter: "~ё~~ое", "~ё~~ое";
        nominative_plural: "~ё~~ые", "~ё~~ые";
        genitive_masculine: "~ё~~ого", "~ё~~ого";
        genitive_feminine: "~ё~~ой", "~ё~~ой";
        genitive_neuter: "~ё~~ого", "~ё~~ого";
        genitive_plural: "~ё~~ых", "~ё~~ых";
        dative_masculine: "~ё~~ому", "~ё~~ому";
        dative_feminine: "~ё~~ой", "~ё~~ой";
        dative_neuter: "~ё~~ому", "~ё~~ому";
        dative_plural: "~ё~~ым", "~ё~~ым";
        accusative_masculine: "~ё~~ого", "~ё~~ый";
        accusative_feminine: "~ё~~ую", "~ё~~ую";
        accusative_neuter: "~ё~~ое", "~ё~~ое";
        accusative_plural: "~ё~~ых", "~ё~~ые";
        instrumental_masculine: "~ё~~ым", "~ё~~ым";
        instrumental_feminine: "~ё~~ой|~ё~~ою", "~ё~~ой|~ё~~ою";
        instrumental_neuter: "~ё~~ым", "~ё~~ым";
        instrumental_plural: "~ё~~ыми", "~ё~~ыми";
        prepositional_masculine: "~ё~~ом", "~ё~~ом";
        prepositional_feminine: "~ё~~ой", "~ё~~ой";
        prepositional_neuter: "~ё~~ом", "~ё~~ом";
        prepositional_plural: "~ё~~ых", "~ё~~ых";
        short_adjective: "~ё~е~", "~е~~а", "~е~~о", "~е~~ы";
    ),
    // Твёрдый: 6 words (жёлтый, твёрдый, мёртвый, тяжёлый, пёстрый, супертяжёлый).
    declension!("твёрдый", 6, "~ё~ый";
        nominative_masculine: "~ё~ый", "~ё~ый";
        nominative_feminine: "~ё~ая", "~ё~ая";
        nominative_neuter: "~ё~ое", "~ё~ое";
        nominative_plural: "~ё~ые", "~ё~ые";
        genitive_masculine: "~ё~ого", "~ё~ого";
        genitive_feminine: "~ё~ой", "~ё~ой";
        genitive_neuter: "~ё~ого", "~ё~ого";
        genitive_plural: "~ё~ых", "~ё~ых";
        dative_masculine: "~ё~ому", "~ё~ому";
        dative_feminine: "~ё~ой", "~ё~ой";
        dative_neuter: "~ё~ому", "~ё~ому";
        dative_plural: "~ё~ым", "~ё~ым";
        accusative_masculine: "~ё~ого", "~ё~ый";
        accusative_feminine: "~ё~ую", "~ё~ую";
        accusative_neuter: "~ё~ое", "~ё~ое";
        accusative_plural: "~ё~ых", "~ё~ые";
        instrumental_masculine: "~ё~ым", "~ё~ым";
        instrumental_feminine: "~ё~ой|~ё~ою", "~ё~ой|~ё~ою";
        instrumental_neuter: "~ё~ым", "~ё~ым";
        instrumental_plural: "~ё~ыми", "~ё~ыми";
        prepositional_masculine: "~ё~ом", "~ё~ом";
        prepositional_feminine: "~ё~ой", "~ё~ой";
        prepositional_neuter: "~ё~ом", "~ё~ом";
        prepositional_plural: "~ё~ых", "~ё~ых";
        short_adjective: "~ё~", "~е~а", "~е~о", "~е~ы";
    ),
    // Должен: 5 words (должен, тесен, грешен, длинен, годен).
    declension!("должен", 5, "~е~";
        nominative_masculine: derived, "~е~";
        nominative_feminine: derived, derived;
        nominative_neuter: derived, derived;
        nominative_plural: derived, derived;
        genitive_masculine: derived, derived;
        genitive_feminine: derived, derived;
        genitive_neuter: derived, derived;
        genitive_plural: derived, derived;
        dative_masculine: derived, derived;
        dative_feminine: derived, derived;
        dative_neuter: derived, derived;
        dative_plural: derived, derived;
        accusative_masculine: derived, derived;
        accusative_feminine: derived, derived;
        accusative_neuter: derived, derived;
        accusative_plural: derived, derived;
        instrumental_masculine: derived, derived;
        instrumental_feminine: derived, derived;
        instrumental_neuter: derived, derived;
        instrumental_plural: derived, derived;
        prepositional_masculine: derived, derived;
        prepositional_feminine: derived, derived;
        prepositional_neuter: derived, derived;
        prepositional_plural: derived, derived;
        short_adjective: "~е~", "~~а", "~~о|~~о", "~~ы|~~ы";
    ),
    // Тяжкий: 5 words (тяжкий, лёжкий, нелёжкий, нетяжкий, сторожкий). Like Пылкий, except in 3 cells: instrumental_feminine anim, instrumental_feminine inan, short_adjective m.
    declension!("тяжкий", 5, "~~ий", like Пылкий;
        nominative_masculine: _, _;
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
        accusative_masculine: _, _;
        accusative_feminine: _, _;
        accusative_neuter: _, _;
        accusative_plural: _, _;
        instrumental_masculine: _, _;
        instrumental_feminine: "~~ой|~~ою", "~~ой|~~ою";
        instrumental_neuter: _, _;
        instrumental_plural: _, _;
        prepositional_masculine: _, _;
        prepositional_feminine: _, _;
        prepositional_neuter: _, _;
        prepositional_plural: _, _;
        short_adjective: "~е~", _, _, _;
    ),
    // Горячий: 4 words (хороший, горячий, свежий, нехороший). Like Сведущий, except in 1 cell: short_adjective n.
    declension!("горячий", 4, "~ий", like Сведущий;
        nominative_masculine: _, _;
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
        accusative_masculine: _, _;
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
        short_adjective: _, _, "~о", _;
    ),
    // Злой: 4 words (злой, смешной, незлой, пресмешной). Like Показной, except in 3 cells: instrumental_feminine anim, instrumental_feminine inan, short_adjective m.
    declension!("злой", 4, "~~ой", like Показной;
        nominative_masculine: _, _;
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
        accusative_masculine: _, _;
        accusative_feminine: _, _;
        accusative_neuter: _, _;
        accusative_plural: _, _;
        instrumental_masculine: _, _;
        instrumental_feminine: "~~ой|~~ою", "~~ой|~~ою";
        instrumental_neuter: _, _;
        instrumental_plural: _, _;
        prepositional_masculine: _, _;
        prepositional_feminine: _, _;
        prepositional_neuter: _, _;
        prepositional_plural: _, _;
        short_adjective: "~о~", _, _, _;
    ),
    // Лёгкий: 4 words (лёгкий, жёсткий, нелёгкий, прелёгкий). Like Чёрный, except in 21 cells: nominative_masculine anim, nominative_masculine inan, nominative_plural anim, nominative_plural inan, genitive_plural anim, genitive_plural inan, ….
    declension!("лёгкий", 4, "~ё~~ий", like Чёрный;
        nominative_masculine: "~ё~~ий", "~ё~~ий";
        nominative_feminine: _, _;
        nominative_neuter: _, _;
        nominative_plural: "~ё~~ие", "~ё~~ие";
        genitive_masculine: _, _;
        genitive_feminine: _, _;
        genitive_neuter: _, _;
        genitive_plural: "~ё~~их", "~ё~~их";
        dative_masculine: _, _;
        dative_feminine: _, _;
        dative_neuter: _, _;
        dative_plural: "~ё~~им", "~ё~~им";
        accusative_masculine: _, "~ё~~ий";
        accusative_feminine: _, _;
        accusative_neuter: _, _;
        accusative_plural: "~ё~~их", "~ё~~ие";
        instrumental_masculine: "~ё~~им", "~ё~~им";
        instrumental_feminine: _, _;
        instrumental_neuter: "~ё~~им", "~ё~~им";
        instrumental_plural: "~ё~~ими", "~ё~~ими";
        prepositional_masculine: _, _;
        prepositional_feminine: _, _;
        prepositional_neuter: _, _;
        prepositional_plural: "~ё~~их", "~ё~~их";
        short_adjective: "~ё~о~", _, _, "~е~~и";
    ),
    // Заёбанный: 4 words (заёбанный, проёбанный, поёбанный, разъёбанный). Like Казнённый, except in 3 cells: short_adjective f, short_adjective n, short_adjective pl.
    declension!("заёбанный", 4, "~ё~ный", like Казнённый;
        nominative_masculine: _, _;
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
        accusative_masculine: _, _;
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
        short_adjective: _, "~ё~а|~е~а", "~ё~о", "~ё~ы";
    ),
    // Высокий: 3 words (противосвёртывающий, всемилостивейший, бывалошний). Like Устарелый, except in 20 cells: nominative_masculine anim, nominative_masculine inan, nominative_plural anim, nominative_plural inan, genitive_plural anim, genitive_plural inan, ….
    declension!("высокий", 3, "~ий", like Устарелый;
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
        short_adjective: _, _, _, "~и";
    ),
    // Дешёвый: 3 words (дешёвый, недешёвый, ультрадешёвый).
    declension!("дешёвый", 3, "~ешё~ый";
        nominative_masculine: "~ешё~ый", "~ешё~ый";
        nominative_feminine: "~ешё~ая", "~ешё~ая";
        nominative_neuter: "~ешё~ое", "~ешё~ое";
        nominative_plural: "~ешё~ые", "~ешё~ые";
        genitive_masculine: "~ешё~ого", "~ешё~ого";
        genitive_feminine: "~ешё~ой", "~ешё~ой";
        genitive_neuter: "~ешё~ого", "~ешё~ого";
        genitive_plural: "~ешё~ых", "~ешё~ых";
        dative_masculine: "~ешё~ому", "~ешё~ому";
        dative_feminine: "~ешё~ой", "~ешё~ой";
        dative_neuter: "~ешё~ому", "~ешё~ому";
        dative_plural: "~ешё~ым", "~ешё~ым";
        accusative_masculine: "~ешё~ого", "~ешё~ый";
        accusative_feminine: "~ешё~ую", "~ешё~ую";
        accusative_neuter: "~ешё~ое", "~ешё~ое";
        accusative_plural: "~ешё~ых", "~ешё~ые";
        instrumental_masculine: "~ешё~ым", "~ешё~ым";
        instrumental_feminine: "~ешё~ой|~ешё~ою", "~ешё~ой|~ешё~ою";
        instrumental_neuter: "~ешё~ым", "~ешё~ым";
        instrumental_plural: "~ешё~ыми", "~ешё~ыми";
        prepositional_masculine: "~ешё~ом", "~ешё~ом";
        prepositional_feminine: "~ешё~ой", "~ешё~ой";
        prepositional_neuter: "~ешё~ом", "~ешё~ом";
        prepositional_plural: "~ешё~ых", "~ешё~ых";
        short_adjective: "~ёше~", "~еше~а", "~ёше~о", "~ёше~ы";
    ),
    // Зелёный: 3 words (зелёный, весёлый, невесёлый). Like Твёрдый, except in 2 cells: short_adjective m, short_adjective pl.
    declension!("зелёный", 3, "~ё~ый", like Твёрдый;
        nominative_masculine: _, _;
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
        accusative_masculine: _, _;
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
        short_adjective: "~е~", _, _, "~е~ы|~е~ы";
    ),
    // Катин: 3 words (сашин, катин, танин). Like Иродов, except in 18 cells: nominative_neuter anim, nominative_neuter inan, genitive_neuter anim, genitive_neuter inan, dative_neuter anim, dative_neuter inan, ….
    declension!("катин", 3, "~", like Иродов;
        nominative_masculine: _, _;
        nominative_feminine: _, _;
        nominative_neuter: derived, derived;
        nominative_plural: _, _;
        genitive_masculine: _, _;
        genitive_feminine: _, _;
        genitive_neuter: derived, derived;
        genitive_plural: _, _;
        dative_masculine: _, _;
        dative_feminine: _, _;
        dative_neuter: derived, derived;
        dative_plural: _, _;
        accusative_masculine: _, "~а";
        accusative_feminine: _, _;
        accusative_neuter: derived, derived;
        accusative_plural: _, "~ых";
        instrumental_masculine: _, _;
        instrumental_feminine: "~ой", "~ой";
        instrumental_neuter: derived, derived;
        instrumental_plural: _, _;
        prepositional_masculine: "~е", "~е";
        prepositional_feminine: _, _;
        prepositional_neuter: derived, derived;
        prepositional_plural: _, _;
        short_adjective: _, _, _, _;
    ),
    // Крутенек: 3 words (крутенек, радёхонек, радёшенек). Like Должен, except in 5 cells: nominative_masculine anim, accusative_masculine inan, short_adjective f, short_adjective n, short_adjective pl.
    declension!("крутенек", 3, "~е~", like Должен;
        nominative_masculine: "~е~", _;
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
        accusative_masculine: _, "~е~";
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
        short_adjective: _, "~ь~а", "~ь~о", "~ь~и";
    ),
    // Сланцевый: 3 words (сланцевый, нахлыстовый, дёгтевый). Like Устарелый, except in 2 cells: accusative_masculine anim, accusative_plural anim.
    declension!("сланцевый", 3, "~ый", like Устарелый;
        nominative_masculine: _, _;
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
        accusative_masculine: "~ый", _;
        accusative_feminine: _, _;
        accusative_neuter: _, _;
        accusative_plural: "~ые", _;
        instrumental_masculine: _, _;
        instrumental_feminine: _, _;
        instrumental_neuter: _, _;
        instrumental_plural: _, _;
        prepositional_masculine: _, _;
        prepositional_feminine: _, _;
        prepositional_neuter: _, _;
        prepositional_plural: _, _;
        short_adjective: _, _, _, _;
    ),
    // Жерёбый: 3 words (успешливый, посажёный, жерёбый). Like Устарелый, except in 2 cells: accusative_masculine inan, accusative_plural inan.
    declension!("жерёбый", 3, "~ый", like Устарелый;
        nominative_masculine: _, _;
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
        accusative_masculine: _, "~ого|~ый";
        accusative_feminine: _, _;
        accusative_neuter: _, _;
        accusative_plural: _, "~ых";
        instrumental_masculine: _, _;
        instrumental_feminine: _, _;
        instrumental_neuter: _, _;
        instrumental_plural: _, _;
        prepositional_masculine: _, _;
        prepositional_feminine: _, _;
        prepositional_neuter: _, _;
        prepositional_plural: _, _;
        short_adjective: _, _, _, _;
    ),
    // Теплонагревательный: 3 words (народно-разговорный, теплонагревательный, малоуютный). Like Уместный, except in 4 cells: accusative_masculine anim, accusative_plural anim, instrumental_feminine anim, instrumental_feminine inan.
    declension!("теплонагревательный", 3, "~~ый", like Уместный;
        nominative_masculine: _, _;
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
        accusative_masculine: "~~ый", _;
        accusative_feminine: _, _;
        accusative_neuter: _, _;
        accusative_plural: "~~ые", _;
        instrumental_masculine: _, _;
        instrumental_feminine: "~~ой", "~~ой";
        instrumental_neuter: _, _;
        instrumental_plural: _, _;
        prepositional_masculine: _, _;
        prepositional_feminine: _, _;
        prepositional_neuter: _, _;
        prepositional_plural: _, _;
        short_adjective: _, _, _, _;
    ),
];
