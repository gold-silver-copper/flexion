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
/// `nominative` (sg, pl)
/// `genitive` (sg, pl)
/// `dative` (sg, pl)
/// `accusative` (sg, pl)
/// `instrumental` (sg, pl)
/// `prepositional` (sg, pl)
/// `noun_second_case` (part, loc)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, [$($f:ident: $v:ident),*])? $(, like $parent:ident)?;
     nominative: $c0:tt, $c1:tt;
     genitive: $c2:tt, $c3:tt;
     dative: $c4:tt, $c5:tt;
     accusative: $c6:tt, $c7:tt;
     instrumental: $c8:tt, $c9:tt;
     prepositional: $c10:tt, $c11:tt;
     noun_second_case: $c12:tt, $c13:tt;
    ) => {
        Paradigm {
            name: ::std::borrow::Cow::Borrowed($name),
            words: $words,
            lemma: ::std::borrow::Cow::Borrowed($lemma),
            parent: like!($($parent)?),
            inherent: ::std::borrow::Cow::Borrowed(&[$($(feature!($f: $v)),*)?]),
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

/// An inherent feature's value, as `(feature, value)` by their places.
macro_rules! feature {
    (gender: $v:ident) => {
        (0, crate::Gender::$v as u8)
    };
    (animacy: $v:ident) => {
        (1, crate::Animacy::$v as u8)
    };
}

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `приём`: 37,696 words.
    Приём,
    /// Like `полнота`: 12,550 words.
    Полнота,
    /// Like `скопление`: 9,055 words.
    Скопление,
    /// Like `склонность`: 7,532 words.
    Склонность,
    /// Like `стрелка`: 6,677 words.
    Стрелка,
    /// Like `химия`: 6,582 words.
    Химия,
    /// Like `какао`: 3,419 words.
    Какао,
    /// Like `хлопанье`: 2,662 words.
    Хлопанье,
    /// Like `дорожка`: 2,522 words.
    Дорожка,
    /// Like `сходство`: 2,331 words.
    Сходство,
    /// Like `агнец`: 2,159 words.
    Агнец,
    /// Like `медведь`: 1,983 words.
    Медведь,
    /// Like `рожок`: 1,052 words.
    Рожок,
    /// Like `товарищ`: 932 words.
    Товарищ,
    /// Like `гений`: 651 words.
    Гений,
    /// Like `змей`: 650 words.
    Змей,
    /// Like `барашек`: 639 words.
    Барашек,
    /// Like `чайка`: 474 words.
    Чайка,
    /// Like `накладная`: 414 words.
    Накладная,
    /// Like `постав`: 397 words.
    Постав,
    /// Like `господин`: 395 words.
    Господин,
    /// Like `ложе`: 327 words.
    Ложе,
    /// Like `зелёный`: 278 words.
    Зелёный,
    /// Like `ключ`: 276 words.
    Ключ,
    /// Like `люлька`: 264 words.
    Люлька,
    /// Like `баня`: 253 words.
    Баня,
    /// Like `аниец`: 250 words.
    Аниец,
    /// Like `червь`: 232 words.
    Червь,
    /// Like `шоколадница`: 214 words.
    Шоколадница,
    /// Like `счёты`: 203 words.
    Счёты,
    /// Like `русский`: 201 words.
    Русский,
    /// Like `батарея`: 194 words.
    Батарея,
    /// Like `лососёвые`: 192 words.
    Лососёвые,
    /// Like `умелец`: 174 words.
    Умелец,
    /// Like `второе`: 160 words.
    Второе,
    /// Like `сватья`: 151 words.
    Сватья,
    /// Like `латы`: 150 words.
    Латы,
    /// Like `молочко`: 148 words.
    Молочко,
    /// Like `мостки`: 147 words.
    Мостки,
    /// Like `заря`: 127 words.
    Заря,
    /// Like `орлёнок`: 124 words.
    Орлёнок,
    /// Like `подворотня`: 120 words.
    Подворотня,
    /// Like `жнивьё`: 115 words.
    Жнивьё,
    /// Like `ясли`: 113 words.
    Ясли,
    /// Like `кормчий`: 107 words.
    Кормчий,
    /// Like `камень`: 107 words.
    Камень,
    /// Like `гривна`: 106 words.
    Гривна,
    /// Like `нападки`: 100 words.
    Нападки,
    /// Like `выдача`: 89 words.
    Выдача,
    /// Like `гусёк`: 89 words.
    Гусёк,
    /// Like `спальня`: 85 words.
    Спальня,
    /// Like `достопримечательность`: 79 words.
    Достопримечательность,
    /// Like `штрафной`: 79 words.
    Штрафной,
    /// Like `абзац`: 76 words.
    Абзац,
    /// Like `кулички`: 74 words.
    Кулички,
    /// Like `дух`: 70 words.
    Дух,
    /// Like `ядро`: 63 words.
    Ядро,
    /// Like `внучонок`: 57 words.
    Внучонок,
    /// Like `раз`: 53 words.
    Раз,
    /// Like `недра`: 52 words.
    Недра,
    /// Like `свеча`: 45 words.
    Свеча,
    /// Like `каракуля`: 45 words.
    Каракуля,
    /// Like `котище`: 40 words.
    Котище,
    /// Like `делёж`: 38 words.
    Делёж,
    /// Like `съезжая`: 37 words.
    Съезжая,
    /// Like `помочь`: 36 words.
    Помочь,
    /// Like `отец`: 33 words.
    Отец,
    /// Like `бой`: 33 words.
    Бой,
    /// Like `горючее`: 33 words.
    Горючее,
    /// Like `дрязги`: 31 words.
    Дрязги,
    /// Like `шов`: 30 words.
    Шов,
    /// Like `ладья`: 30 words.
    Ладья,
    /// Like `член`: 30 words.
    Член,
    /// Like `деревня`: 29 words.
    Деревня,
    /// Like `верея`: 29 words.
    Верея,
    /// Like `кешью`: 29 words.
    Кешью,
    /// Like `порубка`: 29 words.
    Порубка,
    /// Like `егерь`: 27 words.
    Егерь,
    /// Like `пол-литра`: 26 words.
    Поллитра,
    /// Like `голец`: 24 words.
    Голец,
    /// Like `домашние`: 24 words.
    Домашние,
    /// Like `звезда`: 23 words.
    Звезда,
    /// Like `лицо`: 23 words.
    Лицо,
    /// Like `котёл`: 22 words.
    Котёл,
    /// Like `ребро`: 22 words.
    Ребро,
    /// Like `маракуйя`: 22 words.
    Маракуйя,
    /// Like `деньги`: 21 words.
    Деньги,
    /// Like `устье`: 21 words.
    Устье,
    /// Like `клин`: 19 words.
    Клин,
    /// Like `сенцы`: 19 words.
    Сенцы,
    /// Like `ископаемое`: 18 words.
    Ископаемое,
    /// Like `боёк`: 18 words.
    Боёк,
    /// Like `меринос`: 18 words.
    Меринос,
    /// Like `детище`: 17 words.
    Детище,
    /// Like `колёсико`: 16 words.
    Колёсико,
    /// Like `анкара`: 15 words.
    Анкара,
    /// Like `день`: 15 words.
    День,
    /// Like `зло`: 15 words.
    Зло,
    /// Like `бойня`: 15 words.
    Бойня,
    /// Like `побои`: 15 words.
    Побои,
    /// Like `перо`: 14 words.
    Перо,
    /// Like `кокцинеллида`: 14 words.
    Кокцинеллида,
    /// Like `губоцветные`: 14 words.
    Губоцветные,
    /// Like `перешеек`: 14 words.
    Перешеек,
    /// Like `сердце`: 13 words.
    Сердце,
    /// Like `гнездо`: 13 words.
    Гнездо,
    /// Like `котёночек`: 13 words.
    Котёночек,
    /// Like `фекалии`: 12 words.
    Фекалии,
    /// Like `кольцо`: 12 words.
    Кольцо,
    /// Like `рыльце`: 12 words.
    Рыльце,
    /// Like `мощи`: 11 words.
    Мощи,
    /// Like `валенок`: 11 words.
    Валенок,
    /// Like `счёт`: 11 words.
    Счёт,
    /// Like `прения`: 11 words.
    Прения,
    /// Like `тельце`: 11 words.
    Тельце,
    /// Like `племя`: 10 words.
    Племя,
    /// Like `хлопья`: 10 words.
    Хлопья,
    /// Like `болотце`: 10 words.
    Болотце,
    /// Like `кукла`: 9 words.
    Кукла,
    /// Like `плечо`: 8 words.
    Плечо,
    /// Like `горе`: 8 words.
    Горе,
    /// Like `герц`: 8 words.
    Герц,
    /// Like `калашников`: 8 words.
    Калашников,
    /// Like `плюхино`: 8 words.
    Плюхино,
    /// Like `пол-юбки`: 8 words.
    Полюбки,
    /// Like `ложь`: 7 words.
    Ложь,
    /// Like `лапоть`: 7 words.
    Лапоть,
    /// Like `череда`: 7 words.
    Череда,
    /// Like `сиверко`: 7 words.
    Сиверко,
    /// Like `передняя`: 7 words.
    Передняя,
    /// Like `семя`: 7 words.
    Семя,
    /// Like `стекло`: 7 words.
    Стекло,
    /// Like `заём`: 7 words.
    Заём,
    /// Like `весна`: 7 words.
    Весна,
    /// Like `осётр`: 7 words.
    Осётр,
    /// Like `полгода`: 7 words.
    Полгода,
    /// Like `дитё`: 7 words.
    Дитё,
    /// Like `пол-июля`: 7 words.
    Полиюля,
    /// Like `пол-игры`: 7 words.
    Полигры,
    /// Like `кто`: 6 words.
    Кто,
    /// Like `млекопитающее`: 6 words.
    Млекопитающее,
    /// Like `татарин`: 6 words.
    Татарин,
    /// Like `чечётка`: 6 words.
    Чечётка,
    /// Like `зеленя`: 6 words.
    Зеленя,
    /// Like `исайя`: 6 words.
    Исайя,
    /// Like `пол-лета`: 6 words.
    Поллета,
    /// Like `юноша`: 5 words.
    Юноша,
    /// Like `огонь`: 5 words.
    Огонь,
    /// Like `кухня`: 5 words.
    Кухня,
    /// Like `козлы`: 5 words.
    Козлы,
    /// Like `репей`: 5 words.
    Репей,
    /// Like `хозяин`: 5 words.
    Хозяин,
    /// Like `бега`: 5 words.
    Бега,
    /// Like `ботиночек`: 5 words.
    Ботиночек,
    /// Like `полкроны`: 5 words.
    Полкроны,
    /// Like `тиса`: 5 words.
    Тиса,
    /// Like `копьё`: 4 words.
    Копьё,
    /// Like `зуй`: 4 words.
    Зуй,
    /// Like `небо`: 4 words.
    Небо,
    /// Like `судно`: 4 words.
    Судно,
    /// Like `улей`: 4 words.
    Улей,
    /// Like `учащийся`: 4 words.
    Учащийся,
    /// Like `лишай`: 4 words.
    Лишай,
    /// Like `капли`: 4 words.
    Капли,
    /// Like `пресмыкающееся`: 4 words.
    Пресмыкающееся,
    /// Like `остриё`: 4 words.
    Остриё,
    /// Like `ничья`: 4 words.
    Ничья,
    /// Like `яйки`: 4 words.
    Яйки,
    /// Like `сынуля`: 4 words.
    Сынуля,
    /// Like `карло`: 4 words.
    Карло,
    /// Like `дитятко`: 4 words.
    Дитятко,
    /// Like `усадьба`: 3 words.
    Усадьба,
    /// Like `дичь`: 3 words.
    Дичь,
    /// Like `глаз`: 3 words.
    Глаз,
    /// Like `земля`: 3 words.
    Земля,
    /// Like `путь`: 3 words.
    Путь,
    /// Like `лёд`: 3 words.
    Лёд,
    /// Like `сколько`: 3 words.
    Сколько,
    /// Like `дрянь`: 3 words.
    Дрянь,
    /// Like `епитимья`: 3 words.
    Епитимья,
    /// Like `дверца`: 3 words.
    Дверца,
    /// Like `солнышко`: 3 words.
    Солнышко,
    /// Like `получеловек`: 3 words.
    Получеловек,
    /// Like `морячка`: 3 words.
    Морячка,
    /// Like `пол-леса`: 3 words.
    Поллеса,
    /// Like `мастерская`: 1 words.
    Мастерская,
    /// Like `чужое`: 1 words.
    Чужое,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 187] = [
        Declension::Приём,
        Declension::Полнота,
        Declension::Скопление,
        Declension::Склонность,
        Declension::Стрелка,
        Declension::Химия,
        Declension::Какао,
        Declension::Хлопанье,
        Declension::Дорожка,
        Declension::Сходство,
        Declension::Агнец,
        Declension::Медведь,
        Declension::Рожок,
        Declension::Товарищ,
        Declension::Гений,
        Declension::Змей,
        Declension::Барашек,
        Declension::Чайка,
        Declension::Накладная,
        Declension::Постав,
        Declension::Господин,
        Declension::Ложе,
        Declension::Зелёный,
        Declension::Ключ,
        Declension::Люлька,
        Declension::Баня,
        Declension::Аниец,
        Declension::Червь,
        Declension::Шоколадница,
        Declension::Счёты,
        Declension::Русский,
        Declension::Батарея,
        Declension::Лососёвые,
        Declension::Умелец,
        Declension::Второе,
        Declension::Сватья,
        Declension::Латы,
        Declension::Молочко,
        Declension::Мостки,
        Declension::Заря,
        Declension::Орлёнок,
        Declension::Подворотня,
        Declension::Жнивьё,
        Declension::Ясли,
        Declension::Кормчий,
        Declension::Камень,
        Declension::Гривна,
        Declension::Нападки,
        Declension::Выдача,
        Declension::Гусёк,
        Declension::Спальня,
        Declension::Достопримечательность,
        Declension::Штрафной,
        Declension::Абзац,
        Declension::Кулички,
        Declension::Дух,
        Declension::Ядро,
        Declension::Внучонок,
        Declension::Раз,
        Declension::Недра,
        Declension::Свеча,
        Declension::Каракуля,
        Declension::Котище,
        Declension::Делёж,
        Declension::Съезжая,
        Declension::Помочь,
        Declension::Отец,
        Declension::Бой,
        Declension::Горючее,
        Declension::Дрязги,
        Declension::Шов,
        Declension::Ладья,
        Declension::Член,
        Declension::Деревня,
        Declension::Верея,
        Declension::Кешью,
        Declension::Порубка,
        Declension::Егерь,
        Declension::Поллитра,
        Declension::Голец,
        Declension::Домашние,
        Declension::Звезда,
        Declension::Лицо,
        Declension::Котёл,
        Declension::Ребро,
        Declension::Маракуйя,
        Declension::Деньги,
        Declension::Устье,
        Declension::Клин,
        Declension::Сенцы,
        Declension::Ископаемое,
        Declension::Боёк,
        Declension::Меринос,
        Declension::Детище,
        Declension::Колёсико,
        Declension::Анкара,
        Declension::День,
        Declension::Зло,
        Declension::Бойня,
        Declension::Побои,
        Declension::Перо,
        Declension::Кокцинеллида,
        Declension::Губоцветные,
        Declension::Перешеек,
        Declension::Сердце,
        Declension::Гнездо,
        Declension::Котёночек,
        Declension::Фекалии,
        Declension::Кольцо,
        Declension::Рыльце,
        Declension::Мощи,
        Declension::Валенок,
        Declension::Счёт,
        Declension::Прения,
        Declension::Тельце,
        Declension::Племя,
        Declension::Хлопья,
        Declension::Болотце,
        Declension::Кукла,
        Declension::Плечо,
        Declension::Горе,
        Declension::Герц,
        Declension::Калашников,
        Declension::Плюхино,
        Declension::Полюбки,
        Declension::Ложь,
        Declension::Лапоть,
        Declension::Череда,
        Declension::Сиверко,
        Declension::Передняя,
        Declension::Семя,
        Declension::Стекло,
        Declension::Заём,
        Declension::Весна,
        Declension::Осётр,
        Declension::Полгода,
        Declension::Дитё,
        Declension::Полиюля,
        Declension::Полигры,
        Declension::Кто,
        Declension::Млекопитающее,
        Declension::Татарин,
        Declension::Чечётка,
        Declension::Зеленя,
        Declension::Исайя,
        Declension::Поллета,
        Declension::Юноша,
        Declension::Огонь,
        Declension::Кухня,
        Declension::Козлы,
        Declension::Репей,
        Declension::Хозяин,
        Declension::Бега,
        Declension::Ботиночек,
        Declension::Полкроны,
        Declension::Тиса,
        Declension::Копьё,
        Declension::Зуй,
        Declension::Небо,
        Declension::Судно,
        Declension::Улей,
        Declension::Учащийся,
        Declension::Лишай,
        Declension::Капли,
        Declension::Пресмыкающееся,
        Declension::Остриё,
        Declension::Ничья,
        Declension::Яйки,
        Declension::Сынуля,
        Declension::Карло,
        Declension::Дитятко,
        Declension::Усадьба,
        Declension::Дичь,
        Declension::Глаз,
        Declension::Земля,
        Declension::Путь,
        Declension::Лёд,
        Declension::Сколько,
        Declension::Дрянь,
        Declension::Епитимья,
        Declension::Дверца,
        Declension::Солнышко,
        Declension::Получеловек,
        Declension::Морячка,
        Declension::Поллеса,
        Declension::Мастерская,
        Declension::Чужое,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 187] = [
    // Приём: 37,696 words (указ, зонт, кот, зонтик, бортник, бочаг).
    declension!("приём", 37696, "~", [gender: Masculine];
        nominative: "~", "~ы";
        genitive: "~а", "~ов";
        dative: "~у", "~ам";
        accusative: derived, derived;
        instrumental: "~ом", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~у", "~у";
    ),
    // Полнота: 12,550 words (собака, буява, былица, масленица, царица, вата).
    declension!("полнота", 12550, "~а", [gender: Feminine];
        nominative: "~а", "~ы";
        genitive: "~ы", "~";
        dative: "~е", "~ам";
        accusative: "~у", derived;
        instrumental: "~ой|~ою", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: derived, derived;
    ),
    // Скопление: 9,055 words (произношение, лезвие, распятие, деревенение, лесонасаждение, ведение).
    declension!("скопление", 9055, "~е", [gender: Neuter, animacy: Inanimate];
        nominative: "~е", "~я";
        genitive: "~я", "~й";
        dative: "~ю", "~ям";
        accusative: derived, derived;
        instrumental: "~ем", "~ями";
        prepositional: "~и", "~ях";
        noun_second_case: derived, derived;
    ),
    // Склонность: 7,532 words (знать, весь, гласность, деревянность, дверь, ртуть). Like Скопление, except in 7 cells: nominative sg, nominative pl, genitive sg, genitive pl, dative sg, instrumental sg, ….
    declension!("склонность", 7532, "~ь", [gender: Feminine, animacy: Inanimate], like Скопление;
        nominative: "~ь", "~и";
        genitive: "~и", "~ей";
        dative: "~и", _;
        accusative: _, _;
        instrumental: "~ью", _;
        prepositional: _, _;
        noun_second_case: _, "~и";
    ),
    // Стрелка: 6,677 words (водка, цыпка, жилка, деревянка, лесопилка, доска).
    declension!("стрелка", 6677, "~~а", [gender: Feminine];
        nominative: "~~а", "~~и";
        genitive: "~~и", "~о~";
        dative: "~~е", "~~ам";
        accusative: "~~у", derived;
        instrumental: "~~ой|~~ою", "~~ами";
        prepositional: "~~е", "~~ах";
        noun_second_case: derived, derived;
    ),
    // Химия: 6,582 words (анархия, лесохимия, библия, биология, академия, армия). Like Склонность, except in 5 cells: nominative sg, genitive pl, accusative sg, instrumental sg, noun_second_case loc.
    declension!("химия", 6582, "~я", [gender: Feminine, animacy: Inanimate], like Склонность;
        nominative: "~я", _;
        genitive: _, "~й";
        dative: _, _;
        accusative: "~ю", _;
        instrumental: "~ей|~ею", _;
        prepositional: _, _;
        noun_second_case: _, derived;
    ),
    // Какао: 3,419 words (пальто, все, брачущиеся, брачующиеся, адажио, какао).
    declension!("какао", 3419, "~";
        nominative: "~", "~";
        genitive: "~", "~";
        dative: "~", "~";
        accusative: derived, derived;
        instrumental: "~", "~";
        prepositional: "~", "~";
        noun_second_case: derived, derived;
    ),
    // Хлопанье: 2,662 words (запястье, воскресенье, варенье, воспитанье, отчаянье, уединенье).
    declension!("хлопанье", 2662, "~ье", [gender: Neuter, animacy: Inanimate];
        nominative: "~ье", "~ья";
        genitive: "~ья", "~ий";
        dative: "~ью", "~ьям";
        accusative: derived, derived;
        instrumental: "~ьем", "~ьями";
        prepositional: "~ье", "~ьях";
        noun_second_case: derived, derived;
    ),
    // Дорожка: 2,522 words (кошка, быличка, палочка, ручка, мошка, бабочка). Like Стрелка, except in 1 cell: genitive pl.
    declension!("дорожка", 2522, "~~а", like Стрелка;
        nominative: _, _;
        genitive: _, "~е~";
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Сходство: 2,331 words (буево, бёрдо, око, лето, жало, горло). Like Приём, except in 5 cells: nominative sg, nominative pl, genitive pl, noun_second_case part, noun_second_case loc.
    declension!("сходство", 2331, "~о", [gender: Neuter, animacy: Inanimate], like Приём;
        nominative: "~о", "~а";
        genitive: _, "~";
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: derived, derived;
    ),
    // Агнец: 2,159 words (левантинец, древоточец, марганец, самозванец, разночинец, ньюйоркец). Like Стрелка, except in 7 cells: nominative sg, nominative pl, genitive sg, genitive pl, dative sg, accusative sg, ….
    declension!("агнец", 2159, "~е~", [gender: Masculine], like Стрелка;
        nominative: "~е~", "~~ы";
        genitive: "~~а", "~~ев";
        dative: "~~у", _;
        accusative: derived, _;
        instrumental: "~~ем", _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Медведь: 1,983 words (сшиватель, апрель, июнь, июль, олень, автомобиль). Like Скопление, except in 5 cells: nominative sg, nominative pl, genitive pl, prepositional sg, noun_second_case part.
    declension!("медведь", 1983, "~ь", [gender: Masculine], like Скопление;
        nominative: "~ь", "~и";
        genitive: _, "~ей";
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: "~е", _;
        noun_second_case: "~ю", _;
    ),
    // Рожок: 1,052 words (щенок, бесёнок, лесок, замок, носок, подросток). Like Агнец, except in 6 cells: nominative sg, nominative pl, genitive pl, instrumental sg, noun_second_case part, noun_second_case loc.
    declension!("рожок", 1052, "~о~", [gender: Masculine], like Агнец;
        nominative: "~о~", "~~и";
        genitive: _, "~~ов";
        dative: _, _;
        accusative: _, _;
        instrumental: "~~ом", _;
        prepositional: _, _;
        noun_second_case: "~~у", "~~у";
    ),
    // Товарищ: 932 words (харч, туш, треш, товарищ, дервиш, путч). Like Сходство, except in 4 cells: nominative sg, nominative pl, genitive pl, instrumental sg.
    declension!("товарищ", 932, "~", [gender: Masculine], like Сходство;
        nominative: "~", "~и";
        genitive: _, "~ей";
        dative: _, _;
        accusative: _, _;
        instrumental: "~ем", _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Гений: 651 words (калий, меркурий, гений, гравий, бериллий, гелий). Like Медведь, except in 3 cells: nominative sg, genitive pl, prepositional sg.
    declension!("гений", 651, "~й", [gender: Masculine], like Медведь;
        nominative: "~й", _;
        genitive: _, "~ев";
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: "~и", _;
        noun_second_case: _, _;
    ),
    // Змей: 650 words (май, гей, вертухай, елей, герой, случай). Like Гений, except in 1 cell: prepositional sg.
    declension!("змей", 650, "~й", [gender: Masculine], like Гений;
        nominative: _, _;
        genitive: _, _;
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: "~е", _;
        noun_second_case: _, _;
    ),
    // Барашек: 639 words (отец, горошек, свинец, купец, творец, дворец). Like Рожок, except in 2 cells: nominative sg, noun_second_case loc.
    declension!("барашек", 639, "~е~", [gender: Masculine], like Рожок;
        nominative: "~е~", _;
        genitive: _, _;
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, derived;
    ),
    // Чайка: 474 words (копейка, скамейка, перестройка, ищейка, лайка, австрийка).
    declension!("чайка", 474, "~й~а", [gender: Feminine];
        nominative: "~й~а", "~й~и";
        genitive: "~й~и", "~е~";
        dative: "~й~е", "~й~ам";
        accusative: "~й~у", derived;
        instrumental: "~й~ой|~й~ою", "~й~ами";
        prepositional: "~й~е", "~й~ах";
        noun_second_case: derived, derived;
    ),
    // Накладная: 414 words (деревообделочная, детская, гостиная, ванная, кладовая, уборная).
    declension!("накладная", 414, "~ая", [gender: Feminine];
        nominative: "~ая", "~ые";
        genitive: "~ой", "~ых";
        dative: "~ой", "~ым";
        accusative: "~ую", derived;
        instrumental: "~ой|~ою", "~ыми";
        prepositional: "~ой", "~ых";
        noun_second_case: derived, derived;
    ),
    // Постав: 397 words (дом, лес, город, пояс, гидраргирум, адрес). Like Приём, except in 1 cell: nominative pl.
    declension!("постав", 397, "~", [gender: Masculine], like Приём;
        nominative: _, "~а";
        genitive: _, _;
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Господин: 395 words (господин, англичанин, россиянин, опочанин, гражданин, новоангличанин). Like Сходство, except in 6 cells: nominative sg, nominative pl, genitive sg, dative sg, instrumental sg, prepositional sg.
    declension!("господин", 395, "~ин", [gender: Masculine, animacy: Animate], like Сходство;
        nominative: "~ин", "~е";
        genitive: "~ина", _;
        dative: "~ину", _;
        accusative: _, _;
        instrumental: "~ином", _;
        prepositional: "~ине", _;
        noun_second_case: _, _;
    ),
    // Ложе: 327 words (солнце, вече, училище, кладбище, ложе, пристанище). Like Сходство, except in 2 cells: nominative sg, instrumental sg.
    declension!("ложе", 327, "~е", [animacy: Inanimate], like Сходство;
        nominative: "~е", _;
        genitive: _, _;
        dative: _, _;
        accusative: _, _;
        instrumental: "~ем", _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Зелёный: 278 words (белый, чёрный, гласный, беспризорный, коридорный, левый). Like Накладная, except in 6 cells: nominative sg, genitive sg, dative sg, accusative sg, instrumental sg, prepositional sg.
    declension!("зелёный", 278, "~ый", [gender: Masculine], like Накладная;
        nominative: "~ый", _;
        genitive: "~ого", _;
        dative: "~ому", _;
        accusative: derived, _;
        instrumental: "~ым", _;
        prepositional: "~ом", _;
        noun_second_case: _, _;
    ),
    // Ключ: 276 words (борщ, карандаш, врач, сыч, нож, меч). Like Товарищ, except in 1 cell: instrumental sg.
    declension!("ключ", 276, "~", [gender: Masculine], like Товарищ;
        nominative: _, _;
        genitive: _, _;
        dative: _, _;
        accusative: _, _;
        instrumental: "~ом", _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Люлька: 264 words (пенька, пиписька, кухонька, батенька, калька, свистулька).
    declension!("люлька", 264, "~ь~а";
        nominative: "~ь~а", "~ь~и";
        genitive: "~ь~и", "~е~";
        dative: "~ь~е", "~ь~ам";
        accusative: "~ь~у", derived;
        instrumental: "~ь~ой|~ь~ою", "~ь~ами";
        prepositional: "~ь~е", "~ь~ах";
        noun_second_case: derived, derived;
    ),
    // Баня: 253 words (братыня, неделя, буря, няня, княгиня, тётя). Like Химия, except in 3 cells: genitive pl, dative sg, prepositional sg.
    declension!("баня", 253, "~я", like Химия;
        nominative: _, _;
        genitive: _, "~ь";
        dative: "~е", _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: "~е", _;
        noun_second_case: _, _;
    ),
    // Аниец: 250 words (австриец, австралиец, ариец, белогвардеец, китаец, византиец). Like Чайка, except in 7 cells: nominative sg, nominative pl, genitive sg, genitive pl, dative sg, accusative sg, ….
    declension!("аниец", 250, "~е~", [gender: Masculine, animacy: Animate], like Чайка;
        nominative: "~е~", "~й~ы";
        genitive: "~й~а", "~й~ев";
        dative: "~й~у", _;
        accusative: derived, _;
        instrumental: "~й~ем", _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Червь: 232 words (царь, рубль, кремль, дождь, январь, февраль). Like Медведь, except in 1 cell: instrumental sg.
    declension!("червь", 232, "~ь", [gender: Masculine], like Медведь;
        nominative: _, _;
        genitive: _, _;
        dative: _, _;
        accusative: _, _;
        instrumental: "~ём", _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Шоколадница: 214 words (пицца, мульча, грыжа, мрежа, туча, цареубийца). Like Полнота, except in 1 cell: instrumental sg.
    declension!("шоколадница", 214, "~а", like Полнота;
        nominative: _, _;
        genitive: _, _;
        dative: _, _;
        accusative: _, _;
        instrumental: "~ей|~ею", _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Счёты: 203 words (часы, роды, древоеды, щипцы, боеприпасы, французы). Like Приём, except in 7 cells: nominative sg, genitive sg, dative sg, instrumental sg, prepositional sg, noun_second_case part, ….
    declension!("счёты", 203, "~ы", [gender: Masculine], like Приём;
        nominative: "~ы", _;
        genitive: derived, _;
        dative: derived, _;
        accusative: _, _;
        instrumental: derived, _;
        prepositional: derived, _;
        noun_second_case: derived, derived;
    ),
    // Русский: 201 words (русский, сербскохорватский, английский, рабочий, норвежский, китайский). Like Зелёный, except in 7 cells: nominative sg, nominative pl, genitive pl, dative pl, instrumental sg, instrumental pl, ….
    declension!("русский", 201, "~ий", [gender: Masculine], like Зелёный;
        nominative: "~ий", "~ие";
        genitive: _, "~их";
        dative: _, "~им";
        accusative: _, _;
        instrumental: "~им", "~ими";
        prepositional: _, "~их";
        noun_second_case: _, _;
    ),
    // Батарея: 194 words (статуя, стая, шея, идея, затея, фея). Like Баня, except in 1 cell: genitive pl.
    declension!("батарея", 194, "~я", [gender: Feminine], like Баня;
        nominative: _, _;
        genitive: _, "~й";
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Лососёвые: 192 words (данные, чаевые, присные, месячные, наличные, родные). Like Счёты, except in 6 cells: nominative sg, nominative pl, genitive pl, dative pl, instrumental pl, prepositional pl.
    declension!("лососёвые", 192, "~е", like Счёты;
        nominative: "~е", "~е";
        genitive: _, "~х";
        dative: _, "~м";
        accusative: _, _;
        instrumental: _, "~ми";
        prepositional: _, "~х";
        noun_second_case: _, _;
    ),
    // Умелец: 174 words (палец, комсомолец, земледелец, бразилец, владелец, анголец). Like Люлька, except in 7 cells: nominative sg, nominative pl, genitive sg, genitive pl, dative sg, accusative sg, ….
    declension!("умелец", 174, "~е~", [gender: Masculine], like Люлька;
        nominative: "~е~", "~ь~ы";
        genitive: "~ь~а", "~ь~ев";
        dative: "~ь~у", _;
        accusative: derived, _;
        instrumental: "~ь~ем", _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Второе: 160 words (былое, существительное, числительное, многое, прилагательное, шампанское). Like Зелёный, except in 1 cell: nominative sg.
    declension!("второе", 160, "~ое", [gender: Neuter, animacy: Inanimate], like Зелёный;
        nominative: "~ое", _;
        genitive: _, _;
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Сватья: 151 words (гостья, авиаэскадрилья, эскадрилья, келья, тортилья, колдунья). Like Хлопанье, except in 6 cells: nominative sg, nominative pl, genitive sg, dative sg, accusative sg, instrumental sg.
    declension!("сватья", 151, "~ья", [gender: Feminine], like Хлопанье;
        nominative: "~ья", "~ьи";
        genitive: "~ьи", _;
        dative: "~ье", _;
        accusative: "~ью", _;
        instrumental: "~ьей|~ьею", _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Латы: 150 words (цимбалы, каникулы, ножницы, шахматы, именины, бакенбарды). Like Счёты, except in 1 cell: genitive pl.
    declension!("латы", 150, "~ы", [gender: Feminine, animacy: Inanimate], like Счёты;
        nominative: _, _;
        genitive: _, "~";
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Молочко: 148 words (семечко, колено, батько, яичко, окошко, зёрнышко). Like Барашек, except in 3 cells: nominative sg, genitive pl, noun_second_case part.
    declension!("молочко", 148, "~~о", [animacy: Inanimate], like Барашек;
        nominative: "~~о", _;
        genitive: _, "~е~";
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: derived, _;
    ),
    // Мостки: 147 words (древпластики, духи, щипчики, мозги, падонки, очки). Like Товарищ, except in 6 cells: nominative sg, genitive sg, genitive pl, dative sg, instrumental sg, prepositional sg.
    declension!("мостки", 147, "~и", [gender: Masculine, animacy: Inanimate], like Товарищ;
        nominative: "~и", _;
        genitive: "~и", "~ов";
        dative: "~и", _;
        accusative: _, _;
        instrumental: "~и", _;
        prepositional: "~и", _;
        noun_second_case: _, _;
    ),
    // Заря: 127 words (ступня, стезя, простыня, тля, хуйня, броня). Like Баня, except in 2 cells: genitive pl, instrumental sg.
    declension!("заря", 127, "~я", [gender: Feminine], like Баня;
        nominative: _, _;
        genitive: _, "~ей";
        dative: _, _;
        accusative: _, _;
        instrumental: "~ёй|~ёю", _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Орлёнок: 124 words (телёнок, октябрёнок, поросёнок, жеребёнок, ягнёнок, козлёнок).
    declension!("орлёнок", 124, "~ёнок", [gender: Masculine, animacy: Animate];
        nominative: "~ёнок", "~ята";
        genitive: "~ёнка", "~ят";
        dative: "~ёнку", "~ятам";
        accusative: derived, derived;
        instrumental: "~ёнком", "~ятами";
        prepositional: "~ёнке", "~ятах";
        noun_second_case: derived, derived;
    ),
    // Подворотня: 120 words (вишня, песня, бушня, пригоршня, башня, пекарня). Like Дорожка, except in 6 cells: nominative sg, dative pl, accusative sg, instrumental sg, instrumental pl, prepositional pl.
    declension!("подворотня", 120, "~~я", [gender: Feminine, animacy: Inanimate], like Дорожка;
        nominative: "~~я", _;
        genitive: _, _;
        dative: _, "~~ям";
        accusative: "~~ю", _;
        instrumental: "~~ей|~~ею", "~~ями";
        prepositional: _, "~~ях";
        noun_second_case: _, _;
    ),
    // Жнивьё: 115 words (питьё, ружьё, битьё, враньё, сырьё, жильё). Like Хлопанье, except in 3 cells: nominative sg, genitive pl, instrumental sg.
    declension!("жнивьё", 115, "~ьё", [gender: Neuter, animacy: Inanimate], like Хлопанье;
        nominative: "~ьё", _;
        genitive: _, "~ей";
        dative: _, _;
        accusative: _, _;
        instrumental: "~ьём", _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Ясли: 113 words (люди, кости, черви, внутренности, вожжи, ясли). Like Склонность, except in 6 cells: nominative sg, genitive sg, dative sg, instrumental sg, prepositional sg, noun_second_case loc.
    declension!("ясли", 113, "~и", [animacy: Inanimate], like Склонность;
        nominative: "~и", _;
        genitive: derived, _;
        dative: derived, _;
        accusative: _, _;
        instrumental: derived, _;
        prepositional: derived, _;
        noun_second_case: _, derived;
    ),
    // Кормчий: 107 words (лесничий, посторонний, нищий, военнослужащий, домашний, желающий). Like Русский, except in 3 cells: genitive sg, dative sg, prepositional sg.
    declension!("кормчий", 107, "~ий", [gender: Masculine, animacy: Animate], like Русский;
        nominative: _, _;
        genitive: "~его", _;
        dative: "~ему", _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: "~ем", _;
        noun_second_case: _, _;
    ),
    // Камень: 107 words (бредень, оборотень, камень, парень, полдень, кашель). Like Подворотня, except in 7 cells: nominative sg, genitive sg, genitive pl, dative sg, accusative sg, instrumental sg, ….
    declension!("камень", 107, "~е~ь", [gender: Masculine], like Подворотня;
        nominative: "~е~ь", _;
        genitive: "~~я", "~~ей";
        dative: "~~ю", _;
        accusative: derived, _;
        instrumental: "~~ем", _;
        prepositional: _, _;
        noun_second_case: "~~ю", _;
    ),
    // Гривна: 106 words (сосна, домна, царевна, гривна, кошма, плюсна). Like Дорожка, except in 2 cells: nominative pl, genitive sg.
    declension!("гривна", 106, "~~а", [gender: Feminine], like Дорожка;
        nominative: _, "~~ы";
        genitive: "~~ы", _;
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Нападки: 100 words (лесопосадки, плавки, сутки, сливки, колготки, поминки). Like Стрелка, except in 6 cells: nominative sg, genitive sg, dative sg, accusative sg, instrumental sg, prepositional sg.
    declension!("нападки", 100, "~~и", [gender: Feminine, animacy: Inanimate], like Стрелка;
        nominative: "~~и", _;
        genitive: derived, _;
        dative: derived, _;
        accusative: derived, _;
        instrumental: derived, _;
        prepositional: derived, _;
        noun_second_case: _, _;
    ),
    // Выдача: 89 words (кожа, тёща, пища, вельможа, рожа, ложа). Like Шоколадница, except in 2 cells: nominative pl, genitive sg.
    declension!("выдача", 89, "~а", like Шоколадница;
        nominative: _, "~и";
        genitive: "~и", _;
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Гусёк: 89 words (мотылёк, хорёк, кошелёк, королёк, огонёк, уголёк). Like Умелец, except in 5 cells: nominative sg, nominative pl, genitive pl, instrumental sg, noun_second_case part.
    declension!("гусёк", 89, "~ё~", [gender: Masculine], like Умелец;
        nominative: "~ё~", "~ь~и";
        genitive: _, "~ь~ов";
        dative: _, _;
        accusative: _, _;
        instrumental: "~ь~ом", _;
        prepositional: _, _;
        noun_second_case: "~ь~у", _;
    ),
    // Спальня: 85 words (лесопильня, спальня, читальня, колокольня, коптильня, наковальня). Like Люлька, except in 6 cells: nominative sg, dative pl, accusative sg, instrumental sg, instrumental pl, prepositional pl.
    declension!("спальня", 85, "~ь~я", [gender: Feminine, animacy: Inanimate], like Люлька;
        nominative: "~ь~я", _;
        genitive: _, _;
        dative: _, "~ь~ям";
        accusative: "~ь~ю", _;
        instrumental: "~ь~ей|~ь~ею", "~ь~ями";
        prepositional: _, "~ь~ях";
        noun_second_case: _, _;
    ),
    // Достопримечательность: 79 words (лошадь, дочь, мать, сельдь, свекровь, достопримечательность). Like Склонность, except in 2 cells: accusative sg, noun_second_case loc.
    declension!("достопримечательность", 79, "~ь", [gender: Feminine, animacy: Animate], like Склонность;
        nominative: _, _;
        genitive: _, _;
        dative: _, _;
        accusative: "~ь", _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, derived;
    ),
    // Штрафной: 79 words (больной, голубой, часовой, золотой, вестовой, немой). Like Зелёный, except in 1 cell: nominative sg.
    declension!("штрафной", 79, "~ой", [gender: Masculine], like Зелёный;
        nominative: "~ой", _;
        genitive: _, _;
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Абзац: 76 words (месяц, раж, принц, абзац, кош, кварц). Like Приём, except in 3 cells: genitive pl, instrumental sg, noun_second_case part.
    declension!("абзац", 76, "~", [gender: Masculine], like Приём;
        nominative: _, _;
        genitive: _, "~ев";
        dative: _, _;
        accusative: _, _;
        instrumental: "~ем", _;
        prepositional: _, _;
        noun_second_case: derived, _;
    ),
    // Кулички: 74 words (сумерки, бачки, шашки, кусачки, детишки, корточки). Like Нападки, except in 1 cell: genitive pl.
    declension!("кулички", 74, "~~и", like Нападки;
        nominative: _, _;
        genitive: _, "~е~";
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Дух: 70 words (гулаг, воздух, гальбик, восток, урюк, маниок). Like Приём, except in 1 cell: nominative pl.
    declension!("дух", 70, "~", [gender: Masculine, animacy: Inanimate], like Приём;
        nominative: _, "~и";
        genitive: _, _;
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Ядро: 63 words (брашно, говно, деревцо, ядро, масло, вёдро). Like Молочко, except in 1 cell: nominative pl.
    declension!("ядро", 63, "~~о", [gender: Neuter, animacy: Inanimate], like Молочко;
        nominative: _, "~~а";
        genitive: _, _;
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Внучонок: 57 words (медвежонок, индюшонок, крольчонок, внучонок, волчонок, мышонок).
    declension!("внучонок", 57, "~онок", [gender: Masculine, animacy: Animate];
        nominative: "~онок", "~ата";
        genitive: "~онка", "~ат";
        dative: "~онку", "~атам";
        accusative: derived, derived;
        instrumental: "~онком", "~атами";
        prepositional: "~онке", "~атах";
        noun_second_case: derived, derived;
    ),
    // Раз: 53 words (солдат, цыган, раз, гусар, вольт, погон). Like Приём, except in 2 cells: genitive pl, noun_second_case loc.
    declension!("раз", 53, "~", [gender: Masculine], like Приём;
        nominative: _, _;
        genitive: _, "~";
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, derived;
    ),
    // Недра: 52 words (ребята, врата, дрова, уста, внучата, правнучата). Like Латы, except in 2 cells: nominative sg, nominative pl.
    declension!("недра", 52, "~а", like Латы;
        nominative: "~а", "~а";
        genitive: _, _;
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Свеча: 45 words (левша, ханжа, вожжа, свеча, юноша, саранча). Like Выдача, except in 2 cells: genitive pl, instrumental sg.
    declension!("свеча", 45, "~а", like Выдача;
        nominative: _, _;
        genitive: _, "~ей";
        dative: _, _;
        accusative: _, _;
        instrumental: "~ой|~ою", _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Каракуля: 45 words (постеля, батя, дядя, доля, ходуля, сводня). Like Баня, except in 1 cell: genitive pl.
    declension!("каракуля", 45, "~я", like Баня;
        nominative: _, _;
        genitive: _, "~ей";
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Котище: 40 words (дружище, когтище, волчище, хуище, козлище, третье). Like Ложе, except in 1 cell: nominative pl.
    declension!("котище", 40, "~е", [gender: Masculine], like Ложе;
        nominative: _, "~и";
        genitive: _, _;
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Делёж: 38 words (падёж, пиздёж, чертёж, грабёж, платёж, мухлёж).
    declension!("делёж", 38, "~ё~", [gender: Masculine, animacy: Inanimate];
        nominative: "~ё~", "~е~и";
        genitive: "~е~а", "~е~ей";
        dative: "~е~у", "~е~ам";
        accusative: derived, derived;
        instrumental: "~е~ом", "~е~ами";
        prepositional: "~е~е", "~е~ах";
        noun_second_case: derived, derived;
    ),
    // Съезжая: 37 words (прихожая, телеведущая, неверующая, купчая, составляющая, заведующая). Like Русский, except in 6 cells: nominative sg, genitive sg, dative sg, accusative sg, instrumental sg, prepositional sg.
    declension!("съезжая", 37, "~ая", [gender: Feminine], like Русский;
        nominative: "~ая", _;
        genitive: "~ей", _;
        dative: "~ей", _;
        accusative: "~ую", _;
        instrumental: "~ей|~ею", _;
        prepositional: "~ей", _;
        noun_second_case: _, _;
    ),
    // Помочь: 36 words (мочь, тушь, нощь, пещь, помощь, немощь). Like Склонность, except in 3 cells: dative pl, instrumental pl, prepositional pl.
    declension!("помочь", 36, "~ь", [gender: Feminine, animacy: Inanimate], like Склонность;
        nominative: _, _;
        genitive: _, _;
        dative: _, "~ам";
        accusative: _, _;
        instrumental: _, "~ами";
        prepositional: _, "~ах";
        noun_second_case: _, _;
    ),
    // Отец: 33 words (ветер, узел, пепел, дятел, овен, домысел). Like Барашек, except in 1 cell: nominative pl.
    declension!("отец", 33, "~е~", [gender: Masculine], like Барашек;
        nominative: _, "~~ы";
        genitive: _, _;
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Бой: 33 words (чай, бой, рой, слой, край, буй). Like Змей, except in 1 cell: genitive pl.
    declension!("бой", 33, "~й", [gender: Masculine, animacy: Inanimate], like Змей;
        nominative: _, _;
        genitive: _, "~ёв";
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Горючее: 33 words (настоящее, будущее, горючее, грядущее, прочее, болеутоляющее). Like Кормчий, except in 1 cell: nominative sg.
    declension!("горючее", 33, "~ее", [gender: Neuter, animacy: Inanimate], like Кормчий;
        nominative: "~ее", _;
        genitive: _, _;
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Дрязги: 31 words (брюки, ладоши, лыжи, вериги, пики, брызги). Like Выдача, except in 2 cells: nominative sg, instrumental sg.
    declension!("дрязги", 31, "~и", [gender: Feminine, animacy: Inanimate], like Выдача;
        nominative: "~и", _;
        genitive: _, _;
        dative: _, _;
        accusative: _, _;
        instrumental: "~ой", _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Шов: 30 words (рот, свёкор, лоб, сон, хохол, угол). Like Рожок, except in 2 cells: nominative pl, noun_second_case part.
    declension!("шов", 30, "~о~", [gender: Masculine], like Рожок;
        nominative: _, "~~ы";
        genitive: _, _;
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: derived, _;
    ),
    // Ладья: 30 words (семья, ладья, статья, судья, скамья, голья). Like Сватья, except in 2 cells: genitive pl, instrumental sg.
    declension!("ладья", 30, "~ья", like Сватья;
        nominative: _, _;
        genitive: _, "~ей";
        dative: _, _;
        accusative: _, _;
        instrumental: "~ьёй|~ьёю", _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Член: 30 words (клон, тип, член, кат, кекс, бас). Like Приём, except in 4 cells: accusative sg, accusative pl, noun_second_case part, noun_second_case loc.
    declension!("член", 30, "~", [gender: Masculine, animacy: Inanimate], like Приём;
        nominative: _, _;
        genitive: _, _;
        dative: _, _;
        accusative: "~а|~", "~ов|~ы";
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: derived, derived;
    ),
    // Деревня: 29 words (торговля, сабля, капля, туфля, чапля, цапля). Like Подворотня, except in 1 cell: genitive pl.
    declension!("деревня", 29, "~~я", [gender: Feminine], like Подворотня;
        nominative: _, _;
        genitive: _, "~е~ь";
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Верея: 29 words (змея, колея, чешуя, швея, струя, алия). Like Батарея, except in 1 cell: instrumental sg.
    declension!("верея", 29, "~я", [gender: Feminine], like Батарея;
        nominative: _, _;
        genitive: _, _;
        dative: _, _;
        accusative: _, _;
        instrumental: "~ёй|~ёю", _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Кешью: 29 words (порося, кешью, чапаев, гэта, мюсли, пол-москвы). Like Какао, except in 2 cells: accusative sg, accusative pl.
    declension!("кешью", 29, "~", like Какао;
        nominative: _, _;
        genitive: _, _;
        dative: _, _;
        accusative: "~", "~";
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Порубка: 29 words (скоблянка, меньшевичка, связистка, отклейка, порубка, самосброска). Like Свеча, except in 1 cell: genitive pl.
    declension!("порубка", 29, "~а", [gender: Feminine], like Свеча;
        nominative: _, _;
        genitive: _, "~а";
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Егерь: 27 words (учитель, якорь, флигель, лагерь, тополь, соболь). Like Медведь, except in 2 cells: nominative pl, noun_second_case part.
    declension!("егерь", 27, "~ь", [gender: Masculine], like Медведь;
        nominative: _, "~я";
        genitive: _, _;
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: derived, _;
    ),
    // Поллитра: 26 words (пол-очка, пол-литра, пол-оборота, пол-августа, пол-автобуса, пол-аршина).
    declension!("пол-литра", 26, "~-~а";
        nominative: "~-~а", "~у~ы";
        genitive: "~у~а", "~у~ов";
        dative: "~у~у", "~у~ам";
        accusative: derived, derived;
        instrumental: "~у~ом", "~у~ами";
        prepositional: "~у~е", "~у~ах";
        noun_second_case: derived, derived;
    ),
    // Голец: 24 words (лев, телец, стрелец, жилец, делец, малец). Like Умелец, except in 2 cells: genitive pl, instrumental sg.
    declension!("голец", 24, "~е~", [gender: Masculine], like Умелец;
        nominative: _, _;
        genitive: _, "~ь~ов";
        dative: _, _;
        accusative: _, _;
        instrumental: "~ь~ом", _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Домашние: 24 words (немногие, сколькие, брюхоногие, близкие, древние, домашние). Like Горючее, except in 1 cell: accusative sg.
    declension!("домашние", 24, "~ие", like Горючее;
        nominative: _, _;
        genitive: _, _;
        dative: _, _;
        accusative: "~ее", _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Звезда: 23 words (звезда, пчела, жена, сестра, медсестра, береста).
    declension!("звезда", 23, "~е~а", [gender: Feminine];
        nominative: "~е~а", "~ё~ы";
        genitive: "~е~ы", "~ё~";
        dative: "~е~е", "~ё~ам";
        accusative: "~е~у", derived;
        instrumental: "~е~ой|~е~ою", "~ё~ами";
        prepositional: "~е~е", "~ё~ах";
        noun_second_case: derived, derived;
    ),
    // Лицо: 23 words (существо, лицо, божество, чадо, чмо, падло). Like Сходство, except in 1 cell: accusative sg.
    declension!("лицо", 23, "~о", [gender: Neuter, animacy: Animate], like Сходство;
        nominative: _, _;
        genitive: _, _;
        dative: _, _;
        accusative: "~о", _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Котёл: 22 words (пёс, козёл, орёл, овёс, осёл, костёр). Like Отец, except in 2 cells: nominative sg, noun_second_case part.
    declension!("котёл", 22, "~ё~", [gender: Masculine], like Отец;
        nominative: "~ё~", _;
        genitive: _, _;
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: derived, _;
    ),
    // Ребро: 22 words (сребро, серебро, ведро, ребро, весло, зерно).
    declension!("ребро", 22, "~е~~о", [gender: Neuter, animacy: Inanimate];
        nominative: "~е~~о", "~ё~~а";
        genitive: "~е~~а", "~ё~е~";
        dative: "~е~~у", "~ё~~ам";
        accusative: derived, derived;
        instrumental: "~е~~ом", "~ё~~ами";
        prepositional: "~е~~е", "~ё~~ах";
        noun_second_case: derived, derived;
    ),
    // Маракуйя: 22 words (аллилуйя, секвойя, паранойя, папайя, пирайя, майя). Like Баня, except in 1 cell: genitive pl.
    declension!("маракуйя", 22, "~я", [gender: Feminine], like Баня;
        nominative: _, _;
        genitive: _, "~";
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Деньги: 21 words (деньги, четвереньки, грабельки, хахоньки, ахоньки, ясельки). Like Кулички, except in 5 cells: nominative sg, nominative pl, dative pl, instrumental pl, prepositional pl.
    declension!("деньги", 21, "~ь~и", [gender: Feminine, animacy: Inanimate], like Кулички;
        nominative: "~ь~и", "~ь~и";
        genitive: _, _;
        dative: _, "~ь~ам";
        accusative: _, _;
        instrumental: _, "~ь~ами";
        prepositional: _, "~ь~ах";
        noun_second_case: _, _;
    ),
    // Устье: 21 words (платье, устье, подмастерье, подполье, уголье, отребье). Like Скопление, except in 2 cells: genitive pl, prepositional sg.
    declension!("устье", 21, "~е", [animacy: Inanimate], like Скопление;
        nominative: _, _;
        genitive: _, "~ев";
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: "~е", _;
        noun_second_case: _, _;
    ),
    // Клин: 19 words (брат, лист, муж, клин, стул, струп). Like Приём, except in 6 cells: nominative pl, genitive pl, dative pl, instrumental pl, prepositional pl, noun_second_case part.
    declension!("клин", 19, "~", [gender: Masculine], like Приём;
        nominative: _, "~ья";
        genitive: _, "~ьев";
        dative: _, "~ьям";
        accusative: _, _;
        instrumental: _, "~ьями";
        prepositional: _, "~ьях";
        noun_second_case: derived, _;
    ),
    // Сенцы: 19 words (японцы, плоскогубцы, плоскозубцы, шлёпанцы, сланцы, святцы). Like Счёты, except in 1 cell: genitive pl.
    declension!("сенцы", 19, "~ы", [gender: Masculine], like Счёты;
        nominative: _, _;
        genitive: _, "~ев";
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Ископаемое: 18 words (насекомое, животное, земноводное, парнокопытное, ракообразное, позвоночное). Like Второе, except in 1 cell: accusative sg.
    declension!("ископаемое", 18, "~ое", [gender: Neuter, animacy: Animate], like Второе;
        nominative: _, _;
        genitive: _, _;
        dative: _, _;
        accusative: "~ое", _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Боёк: 18 words (паёк, чаёк, ручеёк, кофеёк, боёк, буёк). Like Аниец, except in 4 cells: nominative sg, nominative pl, genitive pl, instrumental sg.
    declension!("боёк", 18, "~ё~", [gender: Masculine], like Аниец;
        nominative: "~ё~", "~й~и";
        genitive: _, "~й~ов";
        dative: _, _;
        accusative: _, _;
        instrumental: "~й~ом", _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Меринос: 18 words (квакер, чернорубашечник, виолончелист, солитер, кастрат, бахмат). Like Член, except in 2 cells: accusative sg, accusative pl.
    declension!("меринос", 18, "~", [gender: Masculine], like Член;
        nominative: _, _;
        genitive: _, _;
        dative: _, _;
        accusative: "~", "~ы";
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Детище: 17 words (чудовище, чудище, детище, посмешище, уёбище, страшилище). Like Ложе, except in 1 cell: accusative sg.
    declension!("детище", 17, "~е", [animacy: Animate], like Ложе;
        nominative: _, _;
        genitive: _, _;
        dative: _, _;
        accusative: "~е", _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Колёсико: 16 words (колёсико, древко, очко, брюшко, личико, пивко). Like Сходство, except in 2 cells: nominative pl, genitive pl.
    declension!("колёсико", 16, "~о", like Сходство;
        nominative: _, "~и";
        genitive: _, "~ов";
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Анкара: 15 words (ковурма, дисна, полумгла, мста, ангара, анкара). Like Полнота, except in 1 cell: genitive pl.
    declension!("анкара", 15, "~а", [gender: Feminine], like Полнота;
        nominative: _, _;
        genitive: _, "~а";
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // День: 15 words (день, ремень, пень, кремень, плетень, трудодень). Like Камень, except in 3 cells: instrumental sg, noun_second_case part, noun_second_case loc.
    declension!("день", 15, "~е~ь", [gender: Masculine], like Камень;
        nominative: _, _;
        genitive: _, _;
        dative: _, _;
        accusative: _, _;
        instrumental: "~~ём", _;
        prepositional: _, _;
        noun_second_case: derived, "~~ю";
    ),
    // Зло: 15 words (окно, зло, сукно, волокно, жалко, тягло). Like Ядро, except in 1 cell: genitive pl.
    declension!("зло", 15, "~~о", [gender: Neuter, animacy: Inanimate], like Ядро;
        nominative: _, _;
        genitive: _, "~о~";
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Бойня: 15 words (кофейня, бойня, водогрейня, воскобойня, двойня, маслобойня). Like Чайка, except in 6 cells: nominative sg, dative pl, accusative sg, instrumental sg, instrumental pl, prepositional pl.
    declension!("бойня", 15, "~й~я", [gender: Feminine, animacy: Inanimate], like Чайка;
        nominative: "~й~я", _;
        genitive: _, _;
        dative: _, "~й~ям";
        accusative: "~й~ю", _;
        instrumental: "~й~ей|~й~ею", "~й~ями";
        prepositional: _, "~й~ях";
        noun_second_case: _, _;
    ),
    // Побои: 15 words (обои, побои, помои, прелиминарии, покои, фотообои). Like Ясли, except in 1 cell: genitive pl.
    declension!("побои", 15, "~и", [gender: Masculine], like Ясли;
        nominative: _, _;
        genitive: _, "~ев";
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Перо: 14 words (дерево, перо, крыло, сено, авиазвено, звено). Like Клин, except in 2 cells: nominative sg, noun_second_case loc.
    declension!("перо", 14, "~о", [gender: Neuter, animacy: Inanimate], like Клин;
        nominative: "~о", _;
        genitive: _, _;
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, derived;
    ),
    // Кокцинеллида: 14 words (ебала, ламиина, иссида, кокцинеллида, алеохарина, циксиида). Like Полнота, except in 1 cell: accusative pl.
    declension!("кокцинеллида", 14, "~а", [animacy: Animate], like Полнота;
        nominative: _, _;
        genitive: _, _;
        dative: _, _;
        accusative: _, "~ы";
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Губоцветные: 14 words (вересковые, водяниковые, верескоцветные, буковые, букоцветные, мальпигиевые).
    declension!("губоцветные", 14, "~е", [gender: Feminine, animacy: Inanimate];
        nominative: "~е", derived;
        genitive: "~х", derived;
        dative: "~м", derived;
        accusative: derived, derived;
        instrumental: "~ми", derived;
        prepositional: "~х", derived;
        noun_second_case: derived, derived;
    ),
    // Перешеек: 14 words (боец, перешеек, хуец, закраек, зашеек, опоек). Like Боёк, except in 1 cell: nominative sg.
    declension!("перешеек", 14, "~е~", [gender: Masculine], like Боёк;
        nominative: "~е~", _;
        genitive: _, _;
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Сердце: 13 words (сердце, деревце, полотенце, блюдце, оконце, волоконце). Like Ядро, except in 2 cells: nominative sg, instrumental sg.
    declension!("сердце", 13, "~~е", [gender: Neuter, animacy: Inanimate], like Ядро;
        nominative: "~~е", _;
        genitive: _, _;
        dative: _, _;
        accusative: _, _;
        instrumental: "~~ем", _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Гнездо: 13 words (озеро, колесо, село, чело, решето, гнездо). Like Делёж, except in 6 cells: nominative sg, nominative pl, genitive pl, dative pl, instrumental pl, prepositional pl.
    declension!("гнездо", 13, "~е~о", [gender: Neuter, animacy: Inanimate], like Делёж;
        nominative: "~е~о", "~ё~а";
        genitive: _, "~ё~";
        dative: _, "~ё~ам";
        accusative: _, _;
        instrumental: _, "~ё~ами";
        prepositional: _, "~ё~ах";
        noun_second_case: _, _;
    ),
    // Котёночек: 13 words (цыплёночек, котёночек, жеребёночек, козлёночек, утёночек, ягнёночек).
    declension!("котёночек", 13, "~ёноче~", [gender: Masculine];
        nominative: "~ёноче~", "~ят~и";
        genitive: "~ёноч~а", "~ято~";
        dative: "~ёноч~у", "~ят~ам";
        accusative: derived, derived;
        instrumental: "~ёноч~ом", "~ят~ами";
        prepositional: "~ёноч~е", "~ят~ах";
        noun_second_case: derived, derived;
    ),
    // Фекалии: 12 words (химикалии, фекалии, авиалинии, гениталии, сатурналии, реалии). Like Ясли, except in 1 cell: genitive pl.
    declension!("фекалии", 12, "~и", [gender: Feminine], like Ясли;
        nominative: _, _;
        genitive: _, "~й";
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Кольцо: 12 words (письмо, дерьмо, кольцо, крыльцо, сельцо, автокольцо). Like Голец, except in 3 cells: nominative sg, nominative pl, genitive pl.
    declension!("кольцо", 12, "~ь~о", [gender: Neuter, animacy: Inanimate], like Голец;
        nominative: "~ь~о", "~ь~а";
        genitive: _, "~е~";
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Рыльце: 12 words (зеркальце, щупальце, дельце, рыльце, сальце, мыльце). Like Кольцо, except in 2 cells: nominative sg, instrumental sg.
    declension!("рыльце", 12, "~ь~е", [gender: Neuter, animacy: Inanimate], like Кольцо;
        nominative: "~ь~е", _;
        genitive: _, _;
        dative: _, _;
        accusative: _, _;
        instrumental: "~ь~ем", _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Мощи: 11 words (щи, клещи, мощи, беруши, марши, кириши). Like Счёты, except in 3 cells: nominative sg, nominative pl, genitive pl.
    declension!("мощи", 11, "~и", like Счёты;
        nominative: "~и", "~и";
        genitive: _, "~ей";
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Валенок: 11 words (ботинок, чулок, турок, валенок, глазок, зубок). Like Молочко, except in 2 cells: nominative sg, genitive pl.
    declension!("валенок", 11, "~о~", [gender: Masculine], like Молочко;
        nominative: "~о~", _;
        genitive: _, "~о~";
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Счёт: 11 words (мёд, счёт, шёлк, жёрнов, жёлоб, аэрожёлоб). Like Делёж, except in 7 cells: nominative pl, genitive sg, genitive pl, dative sg, instrumental sg, prepositional sg, ….
    declension!("счёт", 11, "~ё~", [gender: Masculine, animacy: Inanimate], like Делёж;
        nominative: _, "~е~а";
        genitive: "~ё~а", "~е~ов";
        dative: "~ё~у", _;
        accusative: _, _;
        instrumental: "~ё~ом", _;
        prepositional: "~ё~е", _;
        noun_second_case: _, "~е~у";
    ),
    // Прения: 11 words (прения, условия, переживания, усилия, воспоминания, взаимоотношения). Like Фекалии, except in 2 cells: nominative sg, nominative pl.
    declension!("прения", 11, "~я", [gender: Neuter, animacy: Inanimate], like Фекалии;
        nominative: "~я", "~я";
        genitive: _, _;
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Тельце: 11 words (тельце, весельце, дульце, жальце, зальце, одеяльце). Like Рыльце, except in 1 cell: genitive pl.
    declension!("тельце", 11, "~ь~е", [gender: Neuter, animacy: Inanimate], like Рыльце;
        nominative: _, _;
        genitive: _, "~ь~ев|~е~";
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Племя: 10 words (имя, время, знамя, вымя, пламя, племя).
    declension!("племя", 10, "~я", [gender: Neuter, animacy: Inanimate];
        nominative: "~я", "~ена";
        genitive: "~ени", "~ён";
        dative: "~ени", "~енам";
        accusative: derived, derived;
        instrumental: "~енем", "~енами";
        prepositional: "~ени", "~енах";
        noun_second_case: derived, derived;
    ),
    // Хлопья: 10 words (хлопья, лохмотья, объедья, брусья, коренья, стручья). Like Прения, except in 1 cell: genitive pl.
    declension!("хлопья", 10, "~я", [animacy: Inanimate], like Прения;
        nominative: _, _;
        genitive: _, "~ев";
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Болотце: 10 words (болотце, коленце, ведёрце, корытце, долотце, донце). Like Сердце, except in 1 cell: genitive pl.
    declension!("болотце", 10, "~~е", [gender: Neuter, animacy: Inanimate], like Сердце;
        nominative: _, _;
        genitive: _, "~~ев|~е~";
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Кукла: 9 words (мгла, кукла, мошна, княжна, свёкла, свекла). Like Гривна, except in 1 cell: genitive pl.
    declension!("кукла", 9, "~~а", like Гривна;
        nominative: _, _;
        genitive: _, "~о~";
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Плечо: 8 words (яблоко, молоко, млеко, веко, плечо, брюхо). Like Сходство, except in 1 cell: nominative pl.
    declension!("плечо", 8, "~о", [animacy: Inanimate], like Сходство;
        nominative: _, "~и";
        genitive: _, _;
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Горе: 8 words (море, горе, поле, перекати-поле, муде, биополе). Like Егерь, except in 1 cell: nominative sg.
    declension!("горе", 8, "~е", [gender: Neuter, animacy: Inanimate], like Егерь;
        nominative: "~е", _;
        genitive: _, _;
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Герц: 8 words (герц, гигагерц, килогерц, мегагерц, миллигерц, кольбац). Like Товарищ, except in 2 cells: nominative pl, genitive pl.
    declension!("герц", 8, "~", [gender: Masculine, animacy: Inanimate], like Товарищ;
        nominative: _, "~ы";
        genitive: _, "~";
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Калашников: 8 words (калашников, басков, топтыгин, обещалкин, сердечкин, собакин). Like Зелёный, except in 5 cells: nominative sg, nominative pl, genitive sg, dative sg, prepositional sg.
    declension!("калашников", 8, "~", [gender: Masculine], like Зелёный;
        nominative: "~", "~ы";
        genitive: "~а", _;
        dative: "~у", _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: "~е", _;
        noun_second_case: _, _;
    ),
    // Плюхино: 8 words (мотовство, плюхино, плюхново, кабаново, клементьево, бахтино).
    declension!("плюхино", 8, "~о", [gender: Neuter, animacy: Inanimate];
        nominative: "~о", "~а|~о";
        genitive: "~а|~о", "~о";
        dative: "~у|~о", "~ам|~о";
        accusative: derived, derived;
        instrumental: "~ом|~о", "~ами|~о";
        prepositional: "~е|~о", "~ах|~о";
        noun_second_case: derived, derived;
    ),
    // Полюбки: 8 words (пол-ёмкости, пол-линии, пол-ложки, пол-области, пол-юбки, пол-осьмухи).
    declension!("пол-юбки", 8, "~-~и";
        nominative: "~-~и", "~у~и";
        genitive: "~у~и", "~у~ей";
        dative: "~у~и", "~у~ям";
        accusative: derived, derived;
        instrumental: "~у~ью", "~у~ями";
        prepositional: "~у~и", "~у~ях";
        noun_second_case: derived, derived;
    ),
    // Ложь: 7 words (церковь, рожь, вошь, ложь, любовь, нелюбовь). Like Подворотня, except in 6 cells: nominative sg, genitive pl, dative sg, accusative sg, instrumental sg, prepositional sg.
    declension!("ложь", 7, "~о~ь", [gender: Feminine], like Подворотня;
        nominative: "~о~ь", _;
        genitive: _, "~~ей";
        dative: "~~и", _;
        accusative: derived, _;
        instrumental: "~о~ью", _;
        prepositional: "~~и", _;
        noun_second_case: _, _;
    ),
    // Лапоть: 7 words (локоть, дёготь, ноготь, уголь, коготь, лапоть). Like Камень, except in 2 cells: nominative sg, noun_second_case part.
    declension!("лапоть", 7, "~о~ь", [gender: Masculine, animacy: Inanimate], like Камень;
        nominative: "~о~ь", _;
        genitive: _, _;
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: derived, _;
    ),
    // Череда: 7 words (середа, железа, слеза, пелена, череда, желна). Like Звезда, except in 4 cells: nominative pl, dative pl, instrumental pl, prepositional pl.
    declension!("череда", 7, "~е~а", [gender: Feminine], like Звезда;
        nominative: _, "~е~ы";
        genitive: _, _;
        dative: _, "~е~ам";
        accusative: _, _;
        instrumental: _, "~е~ами";
        prepositional: _, "~е~ах";
        noun_second_case: _, _;
    ),
    // Сиверко: 7 words (ведёрко, сиверко, железко, весёлко, одеялко, морозко). Like Молочко, except in 1 cell: genitive pl.
    declension!("сиверко", 7, "~~о", like Молочко;
        nominative: _, _;
        genitive: _, "~о~";
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Передняя: 7 words (передняя, преисподняя, ближняя, замужняя, незамужняя, несовершеннолетняя). Like Съезжая, except in 2 cells: nominative sg, accusative sg.
    declension!("передняя", 7, "~яя", [gender: Feminine], like Съезжая;
        nominative: "~яя", _;
        genitive: _, _;
        dative: _, _;
        accusative: "~юю", _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Семя: 7 words (бремя, темя, семя, стремя, беремя, льносемя). Like Племя, except in 1 cell: genitive pl.
    declension!("семя", 7, "~я", [gender: Neuter, animacy: Inanimate], like Племя;
        nominative: _, _;
        genitive: _, "~ян";
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Стекло: 7 words (стекло, бронестекло, стегно, пекло, наностекло, оргстекло). Like Ребро, except in 1 cell: genitive pl.
    declension!("стекло", 7, "~е~~о", [gender: Neuter, animacy: Inanimate], like Ребро;
        nominative: _, _;
        genitive: _, "~ё~о~";
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Заём: 7 words (заём, наём, поднаём, перенаём, госзаём, сберзаём). Like Боёк, except in 1 cell: nominative pl.
    declension!("заём", 7, "~ё~", [gender: Masculine, animacy: Inanimate], like Боёк;
        nominative: _, "~й~ы";
        genitive: _, _;
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Весна: 7 words (десна, весна, метла, ветла, гидрометла, блесна). Like Ребро, except in 6 cells: nominative sg, nominative pl, genitive sg, dative sg, accusative sg, instrumental sg.
    declension!("весна", 7, "~е~~а", [gender: Feminine, animacy: Inanimate], like Ребро;
        nominative: "~е~~а", "~ё~~ы";
        genitive: "~е~~ы", _;
        dative: "~е~~е", _;
        accusative: "~е~~у", _;
        instrumental: "~е~~ой|~е~~ою", _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Осётр: 7 words (ксёндз, чёлн, осётр, черёд, перёд, клёст). Like Делёж, except in 2 cells: nominative pl, genitive pl.
    declension!("осётр", 7, "~ё~", [gender: Masculine], like Делёж;
        nominative: _, "~е~ы";
        genitive: _, "~е~ов";
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Полгода: 7 words (полчаса, полгода, полметра, полмиллиона, полкилометра, поллитра). Like Поллитра, except in 1 cell: nominative sg.
    declension!("полгода", 7, "~~а", [animacy: Inanimate], like Поллитра;
        nominative: "~~а", _;
        genitive: _, _;
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Дитё: 7 words (мумиё, дитё, хулиганьё, гадьё, мудачьё, татарьё). Like Егерь, except in 3 cells: nominative sg, accusative sg, instrumental sg.
    declension!("дитё", 7, "~ё", like Егерь;
        nominative: "~ё", _;
        genitive: _, _;
        dative: _, _;
        accusative: "~ё", _;
        instrumental: "~ём", _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Полиюля: 7 words (пол-апреля, пол-имения, пол-июля, пол-июня, пол-лагеря, пол-октября). Like Полюбки, except in 5 cells: nominative sg, genitive sg, dative sg, instrumental sg, prepositional sg.
    declension!("пол-июля", 7, "~-~я", like Полюбки;
        nominative: "~-~я", _;
        genitive: "~у~я", _;
        dative: "~у~ю", _;
        accusative: _, _;
        instrumental: "~у~ем", _;
        prepositional: "~у~е", _;
        noun_second_case: _, _;
    ),
    // Полигры: 7 words (пол-игры, пол-избы, пол-луковицы, пол-юрты, пол-ягоды, пол-луны). Like Поллитра, except in 5 cells: nominative sg, genitive sg, genitive pl, dative sg, instrumental sg.
    declension!("пол-игры", 7, "~-~ы", like Поллитра;
        nominative: "~-~ы", _;
        genitive: "~у~ы", "~у~";
        dative: "~у~е", _;
        accusative: _, _;
        instrumental: "~у~ой", _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Кто: 6 words (что, кто, кое-кто, никто, ничто, кое-что). Like Зелёный, except in 7 cells: nominative sg, nominative pl, genitive pl, dative pl, instrumental sg, instrumental pl, ….
    declension!("кто", 6, "~то", like Зелёный;
        nominative: "~то", "~то";
        genitive: _, "~ого";
        dative: _, "~ому";
        accusative: _, _;
        instrumental: "~ем", "~ем";
        prepositional: _, "~ом";
        noun_second_case: _, _;
    ),
    // Млекопитающее: 6 words (млекопитающее, окружающее, обезболивающее, иглокожее, кишечнодышащее, простейшее). Like Домашние, except in 0 cells: .
    declension!("млекопитающее", 6, "~ее", [gender: Neuter, animacy: Animate], like Домашние;
        nominative: _, _;
        genitive: _, _;
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Татарин: 6 words (болгарин, татарин, хазарин, угрин, долганин, хариджанин). Like Господин, except in 1 cell: nominative pl.
    declension!("татарин", 6, "~ин", [gender: Masculine, animacy: Animate], like Господин;
        nominative: _, "~ы";
        genitive: _, _;
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Чечётка: 6 words (чечётка, метёлка, кержанка, разведёнка, скалолазка, удмуртка). Like Стрелка, except in 1 cell: accusative pl.
    declension!("чечётка", 6, "~~а", [gender: Feminine, animacy: Inanimate], like Стрелка;
        nominative: _, _;
        genitive: _, _;
        dative: _, _;
        accusative: _, "~о~";
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Зеленя: 6 words (зеленя, ебеня, труселя, прохоря, чигиря, уборя). Like Прения, except in 1 cell: genitive pl.
    declension!("зеленя", 6, "~я", [gender: Masculine, animacy: Inanimate], like Прения;
        nominative: _, _;
        genitive: _, "~ей";
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Исайя: 6 words (исайя, хориоидея, заурея, стеаторея, ципрея, персея). Like Батарея, except in 1 cell: accusative pl.
    declension!("исайя", 6, "~я", like Батарея;
        nominative: _, _;
        genitive: _, _;
        dative: _, _;
        accusative: _, "~й";
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Поллета: 6 words (пол-лета, пол-лица, пол-лота, пол-окна, пол-училища, пол-ящика). Like Поллитра, except in 2 cells: nominative pl, genitive pl.
    declension!("пол-лета", 6, "~-~а", like Поллитра;
        nominative: _, "~у~а";
        genitive: _, "~у~";
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Юноша: 5 words (чукча, раджа, обжа, корча, магараджа). Like Выдача, except in 1 cell: genitive pl.
    declension!("юноша", 5, "~а", like Выдача;
        nominative: _, _;
        genitive: _, "~ей";
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Огонь: 5 words (огонь, угорь, ломоть, артогонь, стоп-огонь). Like Лапоть, except in 1 cell: instrumental sg.
    declension!("огонь", 5, "~о~ь", [gender: Masculine], like Лапоть;
        nominative: _, _;
        genitive: _, _;
        dative: _, _;
        accusative: _, _;
        instrumental: "~~ём", _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Кухня: 5 words (кухня, куфня, кормокухня, авиакухня, автокухня). Like Подворотня, except in 1 cell: genitive pl.
    declension!("кухня", 5, "~~я", [gender: Feminine, animacy: Inanimate], like Подворотня;
        nominative: _, _;
        genitive: _, "~о~ь";
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Козлы: 5 words (ножны, бубны, козлы, люберцы, ливны). Like Кулички, except in 2 cells: nominative sg, nominative pl.
    declension!("козлы", 5, "~~ы", [gender: Feminine, animacy: Inanimate], like Кулички;
        nominative: "~~ы", "~~ы";
        genitive: _, _;
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Репей: 5 words (соловей, воробей, муравей, ручей, репей). Like Жнивьё, except in 3 cells: nominative sg, nominative pl, genitive pl.
    declension!("репей", 5, "~ей", [gender: Masculine], like Жнивьё;
        nominative: "~ей", "~ьи";
        genitive: _, "~ьёв";
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Хозяин: 5 words (хозяин, домохозяин, квартирохозяин, сохозяин, судохозяин). Like Господин, except in 5 cells: nominative pl, genitive pl, dative pl, instrumental pl, prepositional pl.
    declension!("хозяин", 5, "~ин", [gender: Masculine, animacy: Animate], like Господин;
        nominative: _, "~ева";
        genitive: _, "~ев";
        dative: _, "~евам";
        accusative: _, _;
        instrumental: _, "~евами";
        prepositional: _, "~евах";
        noun_second_case: _, _;
    ),
    // Бега: 5 words (бега, торока, потроха, холода, севера). Like Недра, except in 1 cell: genitive pl.
    declension!("бега", 5, "~а", [gender: Masculine, animacy: Inanimate], like Недра;
        nominative: _, _;
        genitive: _, "~ов";
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Ботиночек: 5 words (ботиночек, затылочек, катерочек, валеночек, чулочек). Like Молочко, except in 1 cell: nominative sg.
    declension!("ботиночек", 5, "~е~", [gender: Masculine, animacy: Inanimate], like Молочко;
        nominative: "~е~", _;
        genitive: _, _;
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Полкроны: 5 words (полкроны, полстраны, полшколы, ползалупы, полпланеты). Like Полигры, except in 1 cell: nominative sg.
    declension!("полкроны", 5, "~~ы", like Полигры;
        nominative: "~~ы", _;
        genitive: _, _;
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Тиса: 5 words (линда, шила, тиса, драва, пуста). Like Недра, except in 3 cells: dative pl, instrumental pl, prepositional pl.
    declension!("тиса", 5, "~а", [gender: Feminine], like Недра;
        nominative: _, _;
        genitive: _, _;
        dative: _, "~а";
        accusative: _, _;
        instrumental: _, "~а";
        prepositional: _, "~а";
        noun_second_case: _, _;
    ),
    // Копьё: 4 words (копьё, тряпьё, тупьё, репьё). Like Жнивьё, except in 1 cell: genitive pl.
    declension!("копьё", 4, "~ьё", [gender: Neuter, animacy: Inanimate], like Жнивьё;
        nominative: _, _;
        genitive: _, "~ий";
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Зуй: 4 words (хуй, зуй, пидрахуй, суй). Like Бой, except in 4 cells: accusative sg, accusative pl, noun_second_case part, noun_second_case loc.
    declension!("зуй", 4, "~й", [gender: Masculine], like Бой;
        nominative: _, _;
        genitive: _, _;
        dative: _, _;
        accusative: "~я", "~ёв";
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: derived, "~ю";
    ),
    // Небо: 4 words (чудо, небо, древо, худо). Like Сходство, except in 5 cells: nominative pl, genitive pl, dative pl, instrumental pl, prepositional pl.
    declension!("небо", 4, "~о", [gender: Neuter, animacy: Inanimate], like Сходство;
        nominative: _, "~еса";
        genitive: _, "~ес";
        dative: _, "~есам";
        accusative: _, _;
        instrumental: _, "~есами";
        prepositional: _, "~есах";
        noun_second_case: _, _;
    ),
    // Судно: 4 words (судно, агитсудно, авиасудно, фальшдно). Like Бега, except in 5 cells: nominative sg, genitive sg, dative sg, instrumental sg, prepositional sg.
    declension!("судно", 4, "~но", [gender: Neuter, animacy: Inanimate], like Бега;
        nominative: "~но", _;
        genitive: "~на", _;
        dative: "~ну", _;
        accusative: _, _;
        instrumental: "~ном", _;
        prepositional: "~не", _;
        noun_second_case: _, _;
    ),
    // Улей: 4 words (улей, жеребей, чирей, ирей). Like Репей, except in 2 cells: genitive pl, instrumental sg.
    declension!("улей", 4, "~ей", [gender: Masculine, animacy: Inanimate], like Репей;
        nominative: _, _;
        genitive: _, "~ьев";
        dative: _, _;
        accusative: _, _;
        instrumental: "~ьем", _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Учащийся: 4 words (учащийся, трудящийся, вольноопределяющийся, праздношатающийся).
    declension!("учащийся", 4, "~ий~", [gender: Masculine, animacy: Animate];
        nominative: "~ий~", "~ие~";
        genitive: "~его~", "~их~";
        dative: "~ему~", "~им~";
        accusative: derived, derived;
        instrumental: "~им~", "~ими~";
        prepositional: "~ем~", "~их~";
        noun_second_case: derived, derived;
    ),
    // Лишай: 4 words (холуй, лишай, бугай, валуй). Like Бой, except in 2 cells: instrumental sg, noun_second_case part.
    declension!("лишай", 4, "~й", [gender: Masculine], like Бой;
        nominative: _, _;
        genitive: _, _;
        dative: _, _;
        accusative: _, _;
        instrumental: "~ём", _;
        prepositional: _, _;
        noun_second_case: derived, _;
    ),
    // Капли: 4 words (грабли, капли, чапли, велотуфли). Like Нападки, except in 4 cells: genitive pl, dative pl, instrumental pl, prepositional pl.
    declension!("капли", 4, "~~и", [gender: Feminine, animacy: Inanimate], like Нападки;
        nominative: _, _;
        genitive: _, "~е~ь";
        dative: _, "~~ям";
        accusative: _, _;
        instrumental: _, "~~ями";
        prepositional: _, "~~ях";
        noun_second_case: _, _;
    ),
    // Пресмыкающееся: 4 words (пресмыкающееся, случившееся, сбывшееся, несбывшееся). Like Учащийся, except in 2 cells: nominative sg, accusative sg.
    declension!("пресмыкающееся", 4, "~ее~", [gender: Neuter], like Учащийся;
        nominative: "~ее~", _;
        genitive: _, _;
        dative: _, _;
        accusative: "~ее~", _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Остриё: 4 words (остриё, бытиё, востриё, лезвеё). Like Дитё, except in 2 cells: genitive pl, accusative sg.
    declension!("остриё", 4, "~ё", [gender: Neuter, animacy: Inanimate], like Дитё;
        nominative: _, _;
        genitive: _, "~ёв";
        dative: _, _;
        accusative: derived, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Ничья: 4 words (ничья, третья, девичья, моя). Like Съезжая, except in 3 cells: nominative sg, nominative pl, accusative sg.
    declension!("ничья", 4, "~я", [gender: Feminine], like Съезжая;
        nominative: "~я", "~и";
        genitive: _, _;
        dative: _, _;
        accusative: "~ю", _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Яйки: 4 words (обойки, валуйки, яйки, прощайки). Like Кулички, except in 5 cells: nominative sg, nominative pl, dative pl, instrumental pl, prepositional pl.
    declension!("яйки", 4, "~й~и", [animacy: Inanimate], like Кулички;
        nominative: "~й~и", "~й~и";
        genitive: _, _;
        dative: _, "~й~ам";
        accusative: _, _;
        instrumental: _, "~й~ами";
        prepositional: _, "~й~ах";
        noun_second_case: _, _;
    ),
    // Сынуля: 4 words (сынуля, саня, меля, игруля). Like Баня, except in 1 cell: accusative pl.
    declension!("сынуля", 4, "~я", [gender: Masculine, animacy: Animate], like Баня;
        nominative: _, _;
        genitive: _, _;
        dative: _, _;
        accusative: _, "~и";
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Карло: 4 words (ракло, шарыпово, запевало, карло). Like Сходство, except in 1 cell: nominative pl.
    declension!("карло", 4, "~о", like Сходство;
        nominative: _, "~ы";
        genitive: _, _;
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Дитятко: 4 words (дитятко, старичонко, золотко, мужичонко). Like Сиверко, except in 1 cell: accusative sg.
    declension!("дитятко", 4, "~~о", [animacy: Animate], like Сиверко;
        nominative: _, _;
        genitive: _, _;
        dative: _, _;
        accusative: "~~о", _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Усадьба: 3 words (тьма, тюрьма, потьма). Like Люлька, except in 2 cells: nominative pl, genitive sg.
    declension!("усадьба", 3, "~ь~а", [gender: Feminine, animacy: Inanimate], like Люлька;
        nominative: _, "~ь~ы";
        genitive: "~ь~ы", _;
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Дичь: 3 words (мышь, дичь, водомышь). Like Помочь, except in 2 cells: accusative sg, noun_second_case loc.
    declension!("дичь", 3, "~ь", [gender: Feminine, animacy: Animate], like Помочь;
        nominative: _, _;
        genitive: _, _;
        dative: _, _;
        accusative: "~ь", _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, derived;
    ),
    // Глаз: 3 words (глаз, телеглаз, киноглаз). Like Раз, except in 1 cell: nominative pl.
    declension!("глаз", 3, "~", [gender: Masculine, animacy: Inanimate], like Раз;
        nominative: _, "~а";
        genitive: _, _;
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Земля: 3 words (земля, петля, полупетля). Like Деревня, except in 1 cell: instrumental sg.
    declension!("земля", 3, "~~я", [gender: Feminine, animacy: Inanimate], like Деревня;
        nominative: _, _;
        genitive: _, _;
        dative: _, _;
        accusative: _, _;
        instrumental: "~~ёй|~~ёю", _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Путь: 3 words (путь, автопуть, севморпуть). Like Склонность, except in 2 cells: instrumental sg, noun_second_case loc.
    declension!("путь", 3, "~ь", [gender: Masculine, animacy: Inanimate], like Склонность;
        nominative: _, _;
        genitive: _, _;
        dative: _, _;
        accusative: _, _;
        instrumental: "~ём", _;
        prepositional: _, _;
        noun_second_case: _, derived;
    ),
    // Лёд: 3 words (лёд, лён, снеголёд). Like Голец, except in 2 cells: nominative sg, noun_second_case loc.
    declension!("лёд", 3, "~ё~", [gender: Masculine, animacy: Inanimate], like Голец;
        nominative: "~ё~", _;
        genitive: _, _;
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, "~ь~у";
    ),
    // Сколько: 3 words (сколько, столько, несколько). Like Счёты, except in 6 cells: nominative sg, nominative pl, genitive pl, dative pl, instrumental pl, prepositional pl.
    declension!("сколько", 3, "~о", like Счёты;
        nominative: "~о", "~о";
        genitive: _, "~их|~их";
        dative: _, "~им|~им";
        accusative: _, _;
        instrumental: _, "~ими|~ими";
        prepositional: _, "~их|~их";
        noun_second_case: _, _;
    ),
    // Дрянь: 3 words (смерть, дрянь, мразь). Like Склонность, except in 2 cells: accusative pl, noun_second_case loc.
    declension!("дрянь", 3, "~ь", [gender: Feminine, animacy: Inanimate], like Склонность;
        nominative: _, _;
        genitive: _, _;
        dative: _, _;
        accusative: _, "~ей|~и";
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, derived;
    ),
    // Епитимья: 3 words (ектенья, паремья, епитимья). Like Сватья, except in 1 cell: instrumental sg.
    declension!("епитимья", 3, "~ья", [gender: Feminine, animacy: Inanimate], like Сватья;
        nominative: _, _;
        genitive: _, _;
        dative: _, _;
        accusative: _, _;
        instrumental: "~ьёй|~ьёю", _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Дверца: 3 words (дверца, шпорца, мадерца). Like Гривна, except in 1 cell: instrumental sg.
    declension!("дверца", 3, "~~а", [gender: Feminine, animacy: Inanimate], like Гривна;
        nominative: _, _;
        genitive: _, _;
        dative: _, _;
        accusative: _, _;
        instrumental: "~~ей|~~ею", _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Солнышко: 3 words (солнышко, нещечко, студентишко). Like Молочко, except in 1 cell: accusative sg.
    declension!("солнышко", 3, "~~о", [animacy: Animate], like Молочко;
        nominative: _, _;
        genitive: _, _;
        dative: _, _;
        accusative: "~~о", _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Получеловек: 3 words (получеловек, обезьяночеловек, первочеловек).
    declension!("получеловек", 3, "~че~овек", [gender: Masculine, animacy: Animate];
        nominative: "~че~овек", "~~юди";
        genitive: "~че~овека", "~~юдей";
        dative: "~че~овеку", "~~юдям";
        accusative: derived, derived;
        instrumental: "~че~овеком", "~~юдьми";
        prepositional: "~че~овеке", "~~юдях";
        noun_second_case: derived, derived;
    ),
    // Морячка: 3 words (морячка, рыбачка, тестюшка). Like Дорожка, except in 1 cell: accusative pl.
    declension!("морячка", 3, "~~а", [animacy: Animate], like Дорожка;
        nominative: _, _;
        genitive: _, _;
        dative: _, _;
        accusative: _, "~~и";
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Поллеса: 3 words (пол-леса, пол-округа, пол-острова). Like Поллитра, except in 1 cell: nominative pl.
    declension!("пол-леса", 3, "~-~а", like Поллитра;
        nominative: _, "~у~а";
        genitive: _, _;
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
    // Мастерская: 1 words (узкая). Like Съезжая, except in 4 cells: genitive sg, dative sg, instrumental sg, prepositional sg.
    declension!("мастерская", 1, "~ая", like Съезжая;
        nominative: _, _;
        genitive: "~ой", _;
        dative: "~ой", _;
        accusative: _, _;
        instrumental: "~ой|~ою", _;
        prepositional: "~ой", _;
        noun_second_case: _, _;
    ),
    // Чужое: 1 words (неизвестное). Like Русский, except in 1 cell: nominative sg.
    declension!("чужое", 1, "~ое", like Русский;
        nominative: "~ое", _;
        genitive: _, _;
        dative: _, _;
        accusative: _, _;
        instrumental: _, _;
        prepositional: _, _;
        noun_second_case: _, _;
    ),
];
