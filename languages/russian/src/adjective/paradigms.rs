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
/// `short_adjective` (m, f, n, pl)
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
     short_adjective: $c48:tt, $c49:tt, $c50:tt, $c51:tt;
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
                flexion::form!($c48),
                flexion::form!($c49),
                flexion::form!($c50),
                flexion::form!($c51),
            ]),
        }
    };
}

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `щий`: 9,800 words.
    Щий,
    /// Like `естный`: 6,554 words.
    Естный,
    /// Like `нкий`: 5,817 words.
    Нкий,
    /// Like `арелый`: 5,265 words.
    Арелый,
    /// Like `жающийся`: 2,575 words.
    Жающийся,
    /// Like `танный`: 1,732 words.
    Танный,
    /// Like `виальный`: 1,412 words.
    Виальный,
    /// Like `знённый`: 390 words.
    Знённый,
    /// Like `принуждённый`: 379 words.
    Принуждённый,
    /// Like `мой`: 291 words.
    Мой,
    /// Like `казной`: 266 words.
    Казной,
    /// Like `сий`: 177 words.
    Сий,
    /// Like `жайный`: 167 words.
    Жайный,
    /// Like `дний`: 105 words.
    Дний,
    /// Like `окий`: 102 words.
    Окий,
    /// Like `дственный`: 94 words.
    Дственный,
    /// Like `тов`: 67 words.
    Тов,
    /// Like `хонький`: 63 words.
    Хонький,
    /// Like `ухой`: 51 words.
    Ухой,
    /// Like `мкин`: 13 words.
    Мкин,
    /// Like `бойкий`: 12 words.
    Бойкий,
    /// Like `азд`: 12 words.
    Азд,
    /// Like `цый`: 12 words.
    Цый,
    /// Like `зелёный`: 8 words.
    Зелёный,
    /// Like `ний`: 8 words.
    Ний,
    /// Like `твёрдый`: 5 words.
    Твёрдый,
    /// Like `хитрый`: 5 words.
    Хитрый,
    /// Like `жен`: 5 words.
    Жен,
    /// Like `теплонагревательный`: 5 words.
    Теплонагревательный,
    /// Like `чёрный`: 3 words.
    Чёрный,
    /// Like `злой`: 3 words.
    Злой,
    /// Like `роший`: 3 words.
    Роший,
    /// Like `шеий`: 3 words.
    Шеий,
    /// Like `утенек`: 3 words.
    Утенек,
    /// Like `рёбый`: 3 words.
    Рёбый,
    /// Like `кодилий`: 3 words.
    Кодилий,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 36] = [
        Declension::Щий,
        Declension::Естный,
        Declension::Нкий,
        Declension::Арелый,
        Declension::Жающийся,
        Declension::Танный,
        Declension::Виальный,
        Declension::Знённый,
        Declension::Принуждённый,
        Declension::Мой,
        Declension::Казной,
        Declension::Сий,
        Declension::Жайный,
        Declension::Дний,
        Declension::Окий,
        Declension::Дственный,
        Declension::Тов,
        Declension::Хонький,
        Declension::Ухой,
        Declension::Мкин,
        Declension::Бойкий,
        Declension::Азд,
        Declension::Цый,
        Declension::Зелёный,
        Declension::Ний,
        Declension::Твёрдый,
        Declension::Хитрый,
        Declension::Жен,
        Declension::Теплонагревательный,
        Declension::Чёрный,
        Declension::Злой,
        Declension::Роший,
        Declension::Шеий,
        Declension::Утенек,
        Declension::Рёбый,
        Declension::Кодилий,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 36] = [
    // Щий: 9,800 words (увядший, млекопитающий, длиннейший, младший, бывший, вяжущий).
    declension!("щий", 9800, "~ий";
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
    // Естный: 6,554 words (южный, человечный, четвероюродный, федеративный, длинный, ядерный).
    declension!("естный", 6554, "~~ый";
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
    // Нкий: 5,817 words (январский, февральский, августовский, июльский, июньский, австралийский).
    declension!("нкий", 5817, "~~ий";
        nominative_masculine: "~~ий", "~~ий";
        nominative_feminine: "~~ая", "~~ая";
        nominative_neuter: "~~ое", "~~ое";
        nominative_plural: "~~ие", "~~ие";
        genitive_masculine: "~~ого", "~~ого";
        genitive_feminine: "~~ой", "~~ой";
        genitive_neuter: "~~ого", "~~ого";
        genitive_plural: "~~их", "~~их";
        dative_masculine: "~~ому", "~~ому";
        dative_feminine: "~~ой", "~~ой";
        dative_neuter: "~~ому", "~~ому";
        dative_plural: "~~им", "~~им";
        accusative_masculine: "~~ого", "~~ий";
        accusative_feminine: "~~ую", "~~ую";
        accusative_neuter: "~~ое", "~~ое";
        accusative_plural: "~~их", "~~ие";
        instrumental_masculine: "~~им", "~~им";
        instrumental_feminine: "~~ой", "~~ой";
        instrumental_neuter: "~~им", "~~им";
        instrumental_plural: "~~ими", "~~ими";
        prepositional_masculine: "~~ом", "~~ом";
        prepositional_feminine: "~~ой", "~~ой";
        prepositional_neuter: "~~ом", "~~ом";
        prepositional_plural: "~~их", "~~их";
        short_adjective: "~о~", "~~а", "~~о", "~~и";
    ),
    // Арелый: 5,265 words (ядовитый, быстрый, нулёвый, учёный, оранжевый, здоровый).
    declension!("арелый", 5265, "~ый";
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
    // Жающийся: 2,575 words (учащийся, увлекающийся, льющийся, вьющийся, являющийся, являвшийся).
    declension!("жающийся", 2575, "~ий~";
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
        short_adjective: "~е~", "~~а", "~~о", "~~ы";
    ),
    // Танный: 1,732 words (ученный, сделанный, уверенный, квалифицированный, непреднамеренный, сдержанный).
    declension!("танный", 1732, "~ный";
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
    // Виальный: 1,412 words (федеральный, творительный, идеальный, школьный, значительный, официальный).
    declension!("виальный", 1412, "~ь~ый";
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
    // Знённый: 390 words (увлечённый, влюблённый, ожесточённый, включённый, введённый, оценённый).
    declension!("знённый", 390, "~ё~ный";
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
    // Принуждённый: 379 words (несдержанный, недисциплинированный, непредвиденный, многочисленный, отчаянный, самоуверенный).
    declension!("принуждённый", 379, "~ный";
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
        short_adjective: "~", "~на", "~но", "~ны";
    ),
    // Мой: 291 words (былой, младой, золотой, нулевой, рядовой, языковой).
    declension!("мой", 291, "~ой";
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
        short_adjective: "~", "~а", "~о", "~ы";
    ),
    // Казной: 266 words (зубной, земной, цветной, ушной, земляной, дневной).
    declension!("казной", 266, "~~ой";
        nominative_masculine: "~~ой", "~~ой";
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
        accusative_masculine: "~~ого", "~~ой";
        accusative_feminine: "~~ую", "~~ую";
        accusative_neuter: "~~ое", "~~ое";
        accusative_plural: "~~ых", "~~ые";
        instrumental_masculine: "~~ым", "~~ым";
        instrumental_feminine: "~~ой", "~~ой";
        instrumental_neuter: "~~ым", "~~ым";
        instrumental_plural: "~~ыми", "~~ыми";
        prepositional_masculine: "~~ом", "~~ом";
        prepositional_feminine: "~~ой", "~~ой";
        prepositional_neuter: "~~ом", "~~ом";
        prepositional_plural: "~~ых", "~~ых";
        short_adjective: "~е~", "~~а", "~~о", "~~ы";
    ),
    // Сий: 177 words (птичий, щучий, овечий, бычий, олений, человечий).
    declension!("сий", 177, "~ий";
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
        short_adjective: "~", "~а", "~е", "~и";
    ),
    // Жайный: 167 words (аварийный, семейный, линейный, знойный, чрезвычайный, швейный).
    declension!("жайный", 167, "~й~ый";
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
    // Дний: 105 words (вчерашний, зимний, нынешний, здешний, ихний, четырёхлетний).
    declension!("дний", 105, "~~ий";
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
    // Окий: 102 words (многий, убогий, четверорукий, четвероногий, юркий, златорогий).
    declension!("окий", 102, "~ий";
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
        short_adjective: "~", "~а", "~о", "~и";
    ),
    // Дственный: 94 words (царственный, нравственный, явственный, убийственный, множественный, безответственный).
    declension!("дственный", 94, "~ный";
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
        short_adjective: "~ен|~", "~на", "~но", "~ны";
    ),
    // Тов: 67 words (хуев, ааронов, иванов, кесарев, считавши, фомин).
    declension!("тов", 67, "~";
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
        short_adjective: "~", "~а", "~о", "~ы";
    ),
    // Хонький: 63 words (здоровёшенький, зеленёхонький, зеленёшенький, хиленький, хитренький, шустренький).
    declension!("хонький", 63, "~ь~ий";
        nominative_masculine: "~ь~ий", "~ь~ий";
        nominative_feminine: "~ь~ая", "~ь~ая";
        nominative_neuter: "~ь~ое", "~ь~ое";
        nominative_plural: "~ь~ие", "~ь~ие";
        genitive_masculine: "~ь~ого", "~ь~ого";
        genitive_feminine: "~ь~ой", "~ь~ой";
        genitive_neuter: "~ь~ого", "~ь~ого";
        genitive_plural: "~ь~их", "~ь~их";
        dative_masculine: "~ь~ому", "~ь~ому";
        dative_feminine: "~ь~ой", "~ь~ой";
        dative_neuter: "~ь~ому", "~ь~ому";
        dative_plural: "~ь~им", "~ь~им";
        accusative_masculine: "~ь~ого", "~ь~ий";
        accusative_feminine: "~ь~ую", "~ь~ую";
        accusative_neuter: "~ь~ое", "~ь~ое";
        accusative_plural: "~ь~их", "~ь~ие";
        instrumental_masculine: "~ь~им", "~ь~им";
        instrumental_feminine: "~ь~ой|~ь~ою", "~ь~ой|~ь~ою";
        instrumental_neuter: "~ь~им", "~ь~им";
        instrumental_plural: "~ь~ими", "~ь~ими";
        prepositional_masculine: "~ь~ом", "~ь~ом";
        prepositional_feminine: "~ь~ой", "~ь~ой";
        prepositional_neuter: "~ь~ом", "~ь~ом";
        prepositional_plural: "~ь~их", "~ь~их";
        short_adjective: "~е~", "~ь~а", "~ь~о", "~ь~и";
    ),
    // Ухой: 51 words (большой, ямской, тверской, шутовской, княжой, франтовской).
    declension!("ухой", 51, "~ой";
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
        short_adjective: "~", "~а", "~о", "~и";
    ),
    // Мкин: 13 words (дядин, мин, нин, китин, ушкин, пин).
    declension!("мкин", 13, "~";
        nominative_masculine: "~", "~";
        nominative_feminine: "~а", "~а";
        nominative_neuter: "~о", "~о";
        nominative_plural: "~ы", "~ы";
        genitive_masculine: "~ого", "~ого";
        genitive_feminine: "~ой", "~ой";
        genitive_neuter: "~ого", "~ого";
        genitive_plural: "~ых", "~ых";
        dative_masculine: "~ому", "~ому";
        dative_feminine: "~ой", "~ой";
        dative_neuter: "~ому", "~ому";
        dative_plural: "~ым", "~ым";
        accusative_masculine: "~ого", "~";
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
        short_adjective: "~", "~а", "~о", "~ы";
    ),
    // Бойкий: 12 words (бойкий, зимостойкий, носостойкий, стойкий, озостойкий, ростойкий).
    declension!("бойкий", 12, "~й~ий";
        nominative_masculine: "~й~ий", "~й~ий";
        nominative_feminine: "~й~ая", "~й~ая";
        nominative_neuter: "~й~ое", "~й~ое";
        nominative_plural: "~й~ие", "~й~ие";
        genitive_masculine: "~й~ого", "~й~ого";
        genitive_feminine: "~й~ой", "~й~ой";
        genitive_neuter: "~й~ого", "~й~ого";
        genitive_plural: "~й~их", "~й~их";
        dative_masculine: "~й~ому", "~й~ому";
        dative_feminine: "~й~ой", "~й~ой";
        dative_neuter: "~й~ому", "~й~ому";
        dative_plural: "~й~им", "~й~им";
        accusative_masculine: "~й~ого", "~й~ий";
        accusative_feminine: "~й~ую", "~й~ую";
        accusative_neuter: "~й~ое", "~й~ое";
        accusative_plural: "~й~их", "~й~ие";
        instrumental_masculine: "~й~им", "~й~им";
        instrumental_feminine: "~й~ой|~й~ою", "~й~ой|~й~ою";
        instrumental_neuter: "~й~им", "~й~им";
        instrumental_plural: "~й~ими", "~й~ими";
        prepositional_masculine: "~й~ом", "~й~ом";
        prepositional_feminine: "~й~ой", "~й~ой";
        prepositional_neuter: "~й~ом", "~й~ом";
        prepositional_plural: "~й~их", "~й~их";
        short_adjective: "~е~", "~й~а", "~й~о", "~й~и";
    ),
    // Азд: 12 words (мал, рад, узок, япон, рок, лик).
    declension!("азд", 12, "~";
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
        instrumental_feminine: "~ой", "~ой";
        instrumental_neuter: "~ым", "~ым";
        instrumental_plural: "~ыми", "~ыми";
        prepositional_masculine: "~ом", "~ом";
        prepositional_feminine: "~ой", "~ой";
        prepositional_neuter: "~ом", "~ом";
        prepositional_plural: "~ых", "~ых";
        short_adjective: "~", "~а", "~о", "~ы";
    ),
    // Цый: 12 words (еднолицый, цый, лицый, хлолицый, гроволицый, лтолицый).
    declension!("цый", 12, "~ый";
        nominative_masculine: "~ый", "~ый";
        nominative_feminine: "~ая", "~ая";
        nominative_neuter: "~ее", "~ее";
        nominative_plural: "~ые", "~ые";
        genitive_masculine: "~его", "~его";
        genitive_feminine: "~ей", "~ей";
        genitive_neuter: "~его", "~его";
        genitive_plural: "~ых", "~ых";
        dative_masculine: "~ему", "~ему";
        dative_feminine: "~ей", "~ей";
        dative_neuter: "~ему", "~ему";
        dative_plural: "~ым", "~ым";
        accusative_masculine: "~его", "~ый";
        accusative_feminine: "~ую", "~ую";
        accusative_neuter: "~ее", "~ее";
        accusative_plural: "~ых", "~ые";
        instrumental_masculine: "~ым", "~ым";
        instrumental_feminine: "~ей|~ею", "~ей|~ею";
        instrumental_neuter: "~ым", "~ым";
        instrumental_plural: "~ыми", "~ыми";
        prepositional_masculine: "~ем", "~ем";
        prepositional_feminine: "~ей", "~ей";
        prepositional_neuter: "~ем", "~ем";
        prepositional_plural: "~ых", "~ых";
        short_adjective: "~", "~а", "~е", "~ы";
    ),
    // Зелёный: 8 words (дешёвый, зелёный, сёлый, чёный, есёлый, тёжный).
    declension!("зелёный", 8, "~ё~ый";
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
        short_adjective: "~е~", "~е~а", "~е~о", "~е~ы";
    ),
    // Ний: 8 words (игрений, ний, рий, сосий, -синий, дальний).
    declension!("ний", 8, "~ий";
        nominative_masculine: "~ий", "~ий";
        nominative_feminine: "~яя", "~яя";
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
        accusative_feminine: "~юю", "~юю";
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
        short_adjective: "~ь", "~я", "~е", "~и";
    ),
    // Твёрдый: 5 words (жёлтый, твёрдый, мёртвый, тяжёлый, пёстрый).
    declension!("твёрдый", 5, "~ё~ый";
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
        short_adjective: "~ё~", "~е~а", "~е~о", "~е~ы|~ё~ы";
    ),
    // Хитрый: 5 words (хитрый, шустрый, вумный, мный, стрый).
    declension!("хитрый", 5, "~~ый";
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
        short_adjective: "~ё~", "~~а", "~~о", "~~ы|~~ы";
    ),
    // Жен: 5 words (длинен, жен, сен, ешен, ден).
    declension!("жен", 5, "~е~";
        nominative_masculine: "~е~", "~е~";
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
        accusative_masculine: "~~ого", "~е~";
        accusative_feminine: "~~ую", "~~ую";
        accusative_neuter: "~~ое", "~~ое";
        accusative_plural: "~~ых", "~~ые";
        instrumental_masculine: "~~ым", "~~ым";
        instrumental_feminine: "~~ой", "~~ой";
        instrumental_neuter: "~~ым", "~~ым";
        instrumental_plural: "~~ыми", "~~ыми";
        prepositional_masculine: "~~ом", "~~ом";
        prepositional_feminine: "~~ой", "~~ой";
        prepositional_neuter: "~~ом", "~~ом";
        prepositional_plural: "~~ых", "~~ых";
        short_adjective: "~е~", "~~а", "~~о|~~о", "~~ы|~~ы";
    ),
    // Теплонагревательный: 5 words (теплонагревательный, малоуютный, хлыстовый, гтевый, одно-разговорный).
    declension!("теплонагревательный", 5, "~~ый";
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
        accusative_masculine: "~~ый", "~~ый";
        accusative_feminine: "~~ую", "~~ую";
        accusative_neuter: "~~ое", "~~ое";
        accusative_plural: "~~ые", "~~ые";
        instrumental_masculine: "~~ым", "~~ым";
        instrumental_feminine: "~~ой", "~~ой";
        instrumental_neuter: "~~ым", "~~ым";
        instrumental_plural: "~~ыми", "~~ыми";
        prepositional_masculine: "~~ом", "~~ом";
        prepositional_feminine: "~~ой", "~~ой";
        prepositional_neuter: "~~ом", "~~ом";
        prepositional_plural: "~~ых", "~~ых";
        short_adjective: "~е~", "~~а", "~~о", "~~ы";
    ),
    // Чёрный: 3 words (чёрный, тёмный, тёплый).
    declension!("чёрный", 3, "~ё~~ый";
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
    // Злой: 3 words (злой, шной, мешной).
    declension!("злой", 3, "~~ой";
        nominative_masculine: "~~ой", "~~ой";
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
        accusative_masculine: "~~ого", "~~ой";
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
        short_adjective: "~о~", "~~а", "~~о", "~~ы";
    ),
    // Роший: 3 words (горячий, роший, хороший).
    declension!("роший", 3, "~ий";
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
        short_adjective: "~", "~а", "~о", "~и";
    ),
    // Шеий: 3 words (шеий, стошеий, язношеий).
    declension!("шеий", 3, "~ий";
        nominative_masculine: "~ий", "~ий";
        nominative_feminine: "~яя", "~яя";
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
        accusative_feminine: "~юю", "~юю";
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
        short_adjective: "~й", "~я", "~е", "~и";
    ),
    // Утенек: 3 words (утенек, дёхонек, дёшенек).
    declension!("утенек", 3, "~е~";
        nominative_masculine: "~е~", "~е~";
        nominative_feminine: "~ь~ая", "~ь~ая";
        nominative_neuter: "~ь~ое", "~ь~ое";
        nominative_plural: "~ь~ие", "~ь~ие";
        genitive_masculine: "~ь~ого", "~ь~ого";
        genitive_feminine: "~ь~ой", "~ь~ой";
        genitive_neuter: "~ь~ого", "~ь~ого";
        genitive_plural: "~ь~их", "~ь~их";
        dative_masculine: "~ь~ому", "~ь~ому";
        dative_feminine: "~ь~ой", "~ь~ой";
        dative_neuter: "~ь~ому", "~ь~ому";
        dative_plural: "~ь~им", "~ь~им";
        accusative_masculine: "~ь~ого", "~е~";
        accusative_feminine: "~ь~ую", "~ь~ую";
        accusative_neuter: "~ь~ое", "~ь~ое";
        accusative_plural: "~ь~их", "~ь~ие";
        instrumental_masculine: "~ь~им", "~ь~им";
        instrumental_feminine: "~ь~ой", "~ь~ой";
        instrumental_neuter: "~ь~им", "~ь~им";
        instrumental_plural: "~ь~ими", "~ь~ими";
        prepositional_masculine: "~ь~ом", "~ь~ом";
        prepositional_feminine: "~ь~ой", "~ь~ой";
        prepositional_neuter: "~ь~ом", "~ь~ом";
        prepositional_plural: "~ь~их", "~ь~их";
        short_adjective: "~е~", "~ь~а", "~ь~о", "~ь~и";
    ),
    // Рёбый: 3 words (пешливый, ажёный, рёбый).
    declension!("рёбый", 3, "~ый";
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
        accusative_masculine: "~ого", "~ого|~ый";
        accusative_feminine: "~ую", "~ую";
        accusative_neuter: "~ое", "~ое";
        accusative_plural: "~ых", "~ых";
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
    // Кодилий: 3 words (кодилий, хухолий, лий).
    declension!("кодилий", 3, "~ий";
        nominative_masculine: "~ий", "~ий";
        nominative_feminine: "~яя", "~яя";
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
        accusative_feminine: "~юю", "~юю";
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
        short_adjective: "~ен", "~я", "~е", "~и";
    ),
];
