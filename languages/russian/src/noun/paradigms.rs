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
/// they share them), the declension it is like (if any), then a row of patterns
/// per line (`derived`: the form comes from another cell's, see
/// `relations.rs`; `_`, or a row left out: the form is the declension's it is like,
/// or, in a declension like no other, derived):
/// `nom` (sg, pl)
/// `gen_` (sg, pl)
/// `dat` (sg, pl)
/// `acc` (sg, pl)
/// `ins` (sg, pl)
/// `prep` (sg, pl)
/// `noun_second_case` (part, loc)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, [$($f:ident: $v:ident),*])? $(, like $parent:ident)?;
     $(nom: $c0:tt, $c1:tt;)?
     $(gen_: $c2:tt, $c3:tt;)?
     $(dat: $c4:tt, $c5:tt;)?
     $(acc: $c6:tt, $c7:tt;)?
     $(ins: $c8:tt, $c9:tt;)?
     $(prep: $c10:tt, $c11:tt;)?
     $(noun_second_case: $c12:tt, $c13:tt;)?
    ) => {
        Paradigm {
            name: ::std::borrow::Cow::Borrowed($name),
            words: $words,
            lemma: ::std::borrow::Cow::Borrowed($lemma),
            parent: like!($($parent)?),
            inherent: ::std::borrow::Cow::Borrowed(&[$($(feature!($f: $v)),*)?]),
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
    /// Like `накладная`: 415 words.
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
    /// Like `шоколадница`: 218 words.
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
    /// Like `второе`: 161 words.
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
    /// Like `гусёк`: 89 words.
    Гусёк,
    /// Like `выдача`: 85 words.
    Выдача,
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
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 185] = [
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
        Declension::Гусёк,
        Declension::Выдача,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 185] = [
    // Приём: 37,696 words (указ, зонт, кот, зонтик, бортник, бочаг).
    declension!("приём", 37696, "~", [gender: Masculine];
    nom: "~", "~ы";
    gen_: "~а", "~ов";
    dat: "~у", "~ам";
    ins: "~ом", "~ами";
    prep: "~е", "~ах";
    noun_second_case: _, "~у";
    ),
    // Полнота: 12,550 words (собака, буява, былица, масленица, царица, вата). Like Приём, except in 7 cells: nom sg, gen_ sg, gen_ pl, dat sg, acc sg, ins sg, ….
    declension!("полнота", 12550, "~а", [gender: Feminine], like Приём;
    nom: "~а", _;
    gen_: "~ы", "~";
    dat: "~е", _;
    acc: "~у", _;
    ins: "~ой|~ою", _;
    noun_second_case: _, derived;
    ),
    // Скопление: 9,055 words (произношение, лезвие, распятие, деревенение, лесонасаждение, ведение).
    declension!("скопление", 9055, "~е", [gender: Neuter, animacy: Inanimate];
    nom: "~е", "~я";
    gen_: "~я", "~й";
    dat: "~ю", "~ям";
    ins: "~ем", "~ями";
    prep: "~и", "~ях";
    ),
    // Склонность: 7,532 words (знать, весь, гласность, деревянность, дверь, ртуть). Like Скопление, except in 7 cells: nom sg, nom pl, gen_ sg, gen_ pl, dat sg, ins sg, ….
    declension!("склонность", 7532, "~ь", [gender: Feminine, animacy: Inanimate], like Скопление;
    nom: "~ь", "~и";
    gen_: "~и", "~ей";
    dat: "~и", _;
    ins: "~ью", _;
    noun_second_case: _, "~и";
    ),
    // Стрелка: 6,677 words (водка, цыпка, жилка, деревянка, лесопилка, доска).
    declension!("стрелка", 6677, "~~а", [gender: Feminine];
    nom: "~~а", "~~и";
    gen_: "~~и", "~о~";
    dat: "~~е", "~~ам";
    acc: "~~у", _;
    ins: "~~ой|~~ою", "~~ами";
    prep: "~~е", "~~ах";
    ),
    // Химия: 6,582 words (анархия, лесохимия, библия, биология, академия, армия). Like Склонность, except in 5 cells: nom sg, gen_ pl, acc sg, ins sg, noun_second_case loc.
    declension!("химия", 6582, "~я", [gender: Feminine, animacy: Inanimate], like Склонность;
    nom: "~я", _;
    gen_: _, "~й";
    acc: "~ю", _;
    ins: "~ей|~ею", _;
    noun_second_case: _, derived;
    ),
    // Какао: 3,419 words (пальто, все, брачущиеся, брачующиеся, адажио, какао).
    declension!("какао", 3419, "~";
    nom: "~", "~";
    gen_: "~", "~";
    dat: "~", "~";
    ins: "~", "~";
    prep: "~", "~";
    ),
    // Хлопанье: 2,662 words (запястье, воскресенье, варенье, воспитанье, отчаянье, уединенье).
    declension!("хлопанье", 2662, "~ье", [gender: Neuter, animacy: Inanimate];
    nom: "~ье", "~ья";
    gen_: "~ья", "~ий";
    dat: "~ью", "~ьям";
    ins: "~ьем", "~ьями";
    prep: "~ье", "~ьях";
    ),
    // Дорожка: 2,522 words (кошка, быличка, палочка, ручка, мошка, бабочка). Like Стрелка, except in 1 cell: gen_ pl.
    declension!("дорожка", 2522, "~~а", like Стрелка;
    gen_: _, "~е~";
    ),
    // Сходство: 2,331 words (буево, бёрдо, око, лето, жало, горло). Like Приём, except in 4 cells: nom sg, nom pl, gen_ pl, noun_second_case loc.
    declension!("сходство", 2331, "~о", [gender: Neuter, animacy: Inanimate], like Приём;
    nom: "~о", "~а";
    gen_: _, "~";
    noun_second_case: _, derived;
    ),
    // Агнец: 2,159 words (левантинец, древоточец, марганец, самозванец, разночинец, ньюйоркец). Like Стрелка, except in 7 cells: nom sg, nom pl, gen_ sg, gen_ pl, dat sg, acc sg, ….
    declension!("агнец", 2159, "~е~", [gender: Masculine], like Стрелка;
    nom: "~е~", "~~ы";
    gen_: "~~а", "~~ев";
    dat: "~~у", _;
    acc: derived, _;
    ins: "~~ем", _;
    ),
    // Медведь: 1,983 words (сшиватель, апрель, июнь, июль, олень, автомобиль). Like Скопление, except in 4 cells: nom sg, nom pl, gen_ pl, prep sg.
    declension!("медведь", 1983, "~ь", [gender: Masculine], like Скопление;
    nom: "~ь", "~и";
    gen_: _, "~ей";
    prep: "~е", _;
    ),
    // Рожок: 1,052 words (щенок, бесёнок, лесок, замок, носок, подросток). Like Агнец, except in 5 cells: nom sg, nom pl, gen_ pl, ins sg, noun_second_case loc.
    declension!("рожок", 1052, "~о~", [gender: Masculine], like Агнец;
    nom: "~о~", "~~и";
    gen_: _, "~~ов";
    ins: "~~ом", _;
    noun_second_case: _, "~~у";
    ),
    // Товарищ: 932 words (харч, туш, треш, товарищ, дервиш, путч). Like Приём, except in 4 cells: nom pl, gen_ pl, ins sg, noun_second_case loc.
    declension!("товарищ", 932, "~", [gender: Masculine], like Приём;
    nom: _, "~и";
    gen_: _, "~ей";
    ins: "~ем", _;
    noun_second_case: _, derived;
    ),
    // Гений: 651 words (калий, меркурий, гений, гравий, бериллий, гелий). Like Скопление, except in 3 cells: nom sg, nom pl, gen_ pl.
    declension!("гений", 651, "~й", [gender: Masculine], like Скопление;
    nom: "~й", "~и";
    gen_: _, "~ев";
    ),
    // Змей: 650 words (май, гей, вертухай, елей, герой, случай). Like Гений, except in 1 cell: prep sg.
    declension!("змей", 650, "~й", [gender: Masculine], like Гений;
    prep: "~е", _;
    ),
    // Барашек: 639 words (отец, горошек, свинец, купец, творец, дворец). Like Рожок, except in 2 cells: nom sg, noun_second_case loc.
    declension!("барашек", 639, "~е~", [gender: Masculine], like Рожок;
    nom: "~е~", _;
    noun_second_case: _, derived;
    ),
    // Чайка: 474 words (копейка, скамейка, перестройка, ищейка, лайка, австрийка).
    declension!("чайка", 474, "~й~а", [gender: Feminine];
    nom: "~й~а", "~й~и";
    gen_: "~й~и", "~е~";
    dat: "~й~е", "~й~ам";
    acc: "~й~у", _;
    ins: "~й~ой|~й~ою", "~й~ами";
    prep: "~й~е", "~й~ах";
    ),
    // Накладная: 415 words (деревообделочная, детская, гостиная, ванная, кладовая, уборная).
    declension!("накладная", 415, "~ая", [gender: Feminine];
    nom: "~ая", "~ые";
    gen_: "~ой", "~ых";
    dat: "~ой", "~ым";
    acc: "~ую", _;
    ins: "~ой|~ою", "~ыми";
    prep: "~ой", "~ых";
    ),
    // Постав: 397 words (дом, лес, город, пояс, гидраргирум, адрес). Like Приём, except in 1 cell: nom pl.
    declension!("постав", 397, "~", [gender: Masculine], like Приём;
    nom: _, "~а";
    ),
    // Господин: 395 words (господин, англичанин, россиянин, опочанин, гражданин, новоангличанин). Like Сходство, except in 6 cells: nom sg, nom pl, gen_ sg, dat sg, ins sg, prep sg.
    declension!("господин", 395, "~ин", [gender: Masculine, animacy: Animate], like Сходство;
    nom: "~ин", "~е";
    gen_: "~ина", _;
    dat: "~ину", _;
    ins: "~ином", _;
    prep: "~ине", _;
    ),
    // Ложе: 327 words (солнце, вече, училище, кладбище, ложе, пристанище). Like Сходство, except in 2 cells: nom sg, ins sg.
    declension!("ложе", 327, "~е", [animacy: Inanimate], like Сходство;
    nom: "~е", _;
    ins: "~ем", _;
    ),
    // Зелёный: 278 words (белый, чёрный, гласный, беспризорный, коридорный, левый). Like Накладная, except in 6 cells: nom sg, gen_ sg, dat sg, acc sg, ins sg, prep sg.
    declension!("зелёный", 278, "~ый", [gender: Masculine], like Накладная;
    nom: "~ый", _;
    gen_: "~ого", _;
    dat: "~ому", _;
    acc: derived, _;
    ins: "~ым", _;
    prep: "~ом", _;
    ),
    // Ключ: 276 words (борщ, карандаш, врач, сыч, нож, меч). Like Товарищ, except in 1 cell: ins sg.
    declension!("ключ", 276, "~", [gender: Masculine], like Товарищ;
    ins: "~ом", _;
    ),
    // Люлька: 264 words (пенька, пиписька, кухонька, батенька, калька, свистулька).
    declension!("люлька", 264, "~ь~а";
    nom: "~ь~а", "~ь~и";
    gen_: "~ь~и", "~е~";
    dat: "~ь~е", "~ь~ам";
    acc: "~ь~у", _;
    ins: "~ь~ой|~ь~ою", "~ь~ами";
    prep: "~ь~е", "~ь~ах";
    ),
    // Баня: 253 words (братыня, неделя, буря, няня, княгиня, тётя). Like Химия, except in 3 cells: gen_ pl, dat sg, prep sg.
    declension!("баня", 253, "~я", like Химия;
    gen_: _, "~ь";
    dat: "~е", _;
    prep: "~е", _;
    ),
    // Аниец: 250 words (австриец, австралиец, ариец, белогвардеец, китаец, византиец). Like Чайка, except in 7 cells: nom sg, nom pl, gen_ sg, gen_ pl, dat sg, acc sg, ….
    declension!("аниец", 250, "~е~", [gender: Masculine, animacy: Animate], like Чайка;
    nom: "~е~", "~й~ы";
    gen_: "~й~а", "~й~ев";
    dat: "~й~у", _;
    acc: derived, _;
    ins: "~й~ем", _;
    ),
    // Червь: 232 words (царь, рубль, кремль, дождь, январь, февраль). Like Медведь, except in 1 cell: ins sg.
    declension!("червь", 232, "~ь", [gender: Masculine], like Медведь;
    ins: "~ём", _;
    ),
    // Шоколадница: 218 words (пицца, кожа, мульча, грыжа, мрежа, туча). Like Полнота, except in 1 cell: ins sg.
    declension!("шоколадница", 218, "~а", like Полнота;
    ins: "~ей|~ею", _;
    ),
    // Счёты: 203 words (часы, роды, древоеды, щипцы, боеприпасы, французы). Like Приём, except in 6 cells: nom sg, gen_ sg, dat sg, ins sg, prep sg, noun_second_case loc.
    declension!("счёты", 203, "~ы", [gender: Masculine], like Приём;
    nom: "~ы", _;
    gen_: derived, _;
    dat: derived, _;
    ins: derived, _;
    prep: derived, _;
    noun_second_case: _, derived;
    ),
    // Русский: 201 words (русский, сербскохорватский, английский, рабочий, норвежский, китайский). Like Зелёный, except in 7 cells: nom sg, nom pl, gen_ pl, dat pl, ins sg, ins pl, ….
    declension!("русский", 201, "~ий", [gender: Masculine], like Зелёный;
    nom: "~ий", "~ие";
    gen_: _, "~их";
    dat: _, "~им";
    ins: "~им", "~ими";
    prep: _, "~их";
    ),
    // Батарея: 194 words (статуя, стая, шея, идея, затея, фея). Like Баня, except in 1 cell: gen_ pl.
    declension!("батарея", 194, "~я", [gender: Feminine], like Баня;
    gen_: _, "~й";
    ),
    // Лососёвые: 192 words (данные, чаевые, присные, месячные, наличные, родные). Like Счёты, except in 6 cells: nom sg, nom pl, gen_ pl, dat pl, ins pl, prep pl.
    declension!("лососёвые", 192, "~е", like Счёты;
    nom: "~е", "~е";
    gen_: _, "~х";
    dat: _, "~м";
    ins: _, "~ми";
    prep: _, "~х";
    ),
    // Умелец: 174 words (палец, комсомолец, земледелец, бразилец, владелец, анголец). Like Люлька, except in 7 cells: nom sg, nom pl, gen_ sg, gen_ pl, dat sg, acc sg, ….
    declension!("умелец", 174, "~е~", [gender: Masculine], like Люлька;
    nom: "~е~", "~ь~ы";
    gen_: "~ь~а", "~ь~ев";
    dat: "~ь~у", _;
    acc: derived, _;
    ins: "~ь~ем", _;
    ),
    // Второе: 161 words (былое, существительное, числительное, многое, прилагательное, шампанское). Like Зелёный, except in 1 cell: nom sg.
    declension!("второе", 161, "~ое", [gender: Neuter, animacy: Inanimate], like Зелёный;
    nom: "~ое", _;
    ),
    // Сватья: 151 words (гостья, авиаэскадрилья, эскадрилья, келья, тортилья, колдунья). Like Хлопанье, except in 6 cells: nom sg, nom pl, gen_ sg, dat sg, acc sg, ins sg.
    declension!("сватья", 151, "~ья", [gender: Feminine], like Хлопанье;
    nom: "~ья", "~ьи";
    gen_: "~ьи", _;
    dat: "~ье", _;
    acc: "~ью", _;
    ins: "~ьей|~ьею", _;
    ),
    // Латы: 150 words (цимбалы, каникулы, ножницы, шахматы, именины, бакенбарды). Like Счёты, except in 1 cell: gen_ pl.
    declension!("латы", 150, "~ы", [gender: Feminine, animacy: Inanimate], like Счёты;
    gen_: _, "~";
    ),
    // Молочко: 148 words (семечко, колено, батько, яичко, окошко, зёрнышко). Like Барашек, except in 2 cells: nom sg, gen_ pl.
    declension!("молочко", 148, "~~о", [animacy: Inanimate], like Барашек;
    nom: "~~о", _;
    gen_: _, "~е~";
    ),
    // Мостки: 147 words (древпластики, духи, щипчики, мозги, падонки, очки). Like Товарищ, except in 6 cells: nom sg, gen_ sg, gen_ pl, dat sg, ins sg, prep sg.
    declension!("мостки", 147, "~и", [gender: Masculine, animacy: Inanimate], like Товарищ;
    nom: "~и", _;
    gen_: "~и", "~ов";
    dat: "~и", _;
    ins: "~и", _;
    prep: "~и", _;
    ),
    // Заря: 127 words (ступня, стезя, простыня, тля, хуйня, броня). Like Баня, except in 2 cells: gen_ pl, ins sg.
    declension!("заря", 127, "~я", [gender: Feminine], like Баня;
    gen_: _, "~ей";
    ins: "~ёй|~ёю", _;
    ),
    // Орлёнок: 124 words (телёнок, октябрёнок, поросёнок, жеребёнок, ягнёнок, козлёнок).
    declension!("орлёнок", 124, "~ёнок", [gender: Masculine, animacy: Animate];
    nom: "~ёнок", "~ята";
    gen_: "~ёнка", "~ят";
    dat: "~ёнку", "~ятам";
    ins: "~ёнком", "~ятами";
    prep: "~ёнке", "~ятах";
    ),
    // Подворотня: 120 words (вишня, песня, бушня, пригоршня, башня, пекарня). Like Дорожка, except in 6 cells: nom sg, dat pl, acc sg, ins sg, ins pl, prep pl.
    declension!("подворотня", 120, "~~я", [gender: Feminine, animacy: Inanimate], like Дорожка;
    nom: "~~я", _;
    dat: _, "~~ям";
    acc: "~~ю", _;
    ins: "~~ей|~~ею", "~~ями";
    prep: _, "~~ях";
    ),
    // Жнивьё: 115 words (питьё, ружьё, битьё, враньё, сырьё, жильё). Like Хлопанье, except in 3 cells: nom sg, gen_ pl, ins sg.
    declension!("жнивьё", 115, "~ьё", [gender: Neuter, animacy: Inanimate], like Хлопанье;
    nom: "~ьё", _;
    gen_: _, "~ей";
    ins: "~ьём", _;
    ),
    // Ясли: 113 words (люди, кости, черви, внутренности, вожжи, ясли). Like Медведь, except in 5 cells: nom sg, gen_ sg, dat sg, ins sg, prep sg.
    declension!("ясли", 113, "~и", [animacy: Inanimate], like Медведь;
    nom: "~и", _;
    gen_: derived, _;
    dat: derived, _;
    ins: derived, _;
    prep: derived, _;
    ),
    // Кормчий: 107 words (лесничий, посторонний, нищий, военнослужащий, домашний, желающий). Like Русский, except in 3 cells: gen_ sg, dat sg, prep sg.
    declension!("кормчий", 107, "~ий", [gender: Masculine, animacy: Animate], like Русский;
    gen_: "~его", _;
    dat: "~ему", _;
    prep: "~ем", _;
    ),
    // Камень: 107 words (бредень, оборотень, камень, парень, полдень, кашель). Like Подворотня, except in 6 cells: nom sg, gen_ sg, gen_ pl, dat sg, acc sg, ins sg.
    declension!("камень", 107, "~е~ь", [gender: Masculine], like Подворотня;
    nom: "~е~ь", _;
    gen_: "~~я", "~~ей";
    dat: "~~ю", _;
    acc: derived, _;
    ins: "~~ем", _;
    ),
    // Гривна: 106 words (сосна, домна, царевна, гривна, кошма, плюсна). Like Дорожка, except in 2 cells: nom pl, gen_ sg.
    declension!("гривна", 106, "~~а", [gender: Feminine], like Дорожка;
    nom: _, "~~ы";
    gen_: "~~ы", _;
    ),
    // Нападки: 100 words (лесопосадки, плавки, сутки, сливки, колготки, поминки). Like Стрелка, except in 6 cells: nom sg, gen_ sg, dat sg, acc sg, ins sg, prep sg.
    declension!("нападки", 100, "~~и", [gender: Feminine, animacy: Inanimate], like Стрелка;
    nom: "~~и", _;
    gen_: derived, _;
    dat: derived, _;
    acc: derived, _;
    ins: derived, _;
    prep: derived, _;
    ),
    // Гусёк: 89 words (мотылёк, хорёк, кошелёк, королёк, огонёк, уголёк). Like Умелец, except in 4 cells: nom sg, nom pl, gen_ pl, ins sg.
    declension!("гусёк", 89, "~ё~", [gender: Masculine], like Умелец;
    nom: "~ё~", "~ь~и";
    gen_: _, "~ь~ов";
    ins: "~ь~ом", _;
    ),
    // Выдача: 85 words (тёща, пища, вельможа, гуща, роща, толща). Like Шоколадница, except in 2 cells: nom pl, gen_ sg.
    declension!("выдача", 85, "~а", like Шоколадница;
    nom: _, "~и";
    gen_: "~и", _;
    ),
    // Спальня: 85 words (лесопильня, спальня, читальня, колокольня, коптильня, наковальня). Like Люлька, except in 6 cells: nom sg, dat pl, acc sg, ins sg, ins pl, prep pl.
    declension!("спальня", 85, "~ь~я", [gender: Feminine, animacy: Inanimate], like Люлька;
    nom: "~ь~я", _;
    dat: _, "~ь~ям";
    acc: "~ь~ю", _;
    ins: "~ь~ей|~ь~ею", "~ь~ями";
    prep: _, "~ь~ях";
    ),
    // Достопримечательность: 79 words (лошадь, дочь, мать, сельдь, свекровь, достопримечательность). Like Склонность, except in 2 cells: acc sg, noun_second_case loc.
    declension!("достопримечательность", 79, "~ь", [gender: Feminine, animacy: Animate], like Склонность;
    acc: "~ь", _;
    noun_second_case: _, derived;
    ),
    // Штрафной: 79 words (больной, голубой, часовой, золотой, вестовой, немой). Like Зелёный, except in 1 cell: nom sg.
    declension!("штрафной", 79, "~ой", [gender: Masculine], like Зелёный;
    nom: "~ой", _;
    ),
    // Абзац: 76 words (месяц, раж, принц, абзац, кош, кварц). Like Приём, except in 2 cells: gen_ pl, ins sg.
    declension!("абзац", 76, "~", [gender: Masculine], like Приём;
    gen_: _, "~ев";
    ins: "~ем", _;
    ),
    // Кулички: 74 words (сумерки, бачки, шашки, кусачки, детишки, корточки). Like Нападки, except in 1 cell: gen_ pl.
    declension!("кулички", 74, "~~и", like Нападки;
    gen_: _, "~е~";
    ),
    // Дух: 70 words (гулаг, воздух, гальбик, восток, урюк, маниок). Like Приём, except in 1 cell: nom pl.
    declension!("дух", 70, "~", [gender: Masculine, animacy: Inanimate], like Приём;
    nom: _, "~и";
    ),
    // Ядро: 63 words (брашно, говно, деревцо, ядро, масло, вёдро). Like Молочко, except in 1 cell: nom pl.
    declension!("ядро", 63, "~~о", [gender: Neuter, animacy: Inanimate], like Молочко;
    nom: _, "~~а";
    ),
    // Внучонок: 57 words (медвежонок, индюшонок, крольчонок, внучонок, волчонок, мышонок).
    declension!("внучонок", 57, "~онок", [gender: Masculine, animacy: Animate];
    nom: "~онок", "~ата";
    gen_: "~онка", "~ат";
    dat: "~онку", "~атам";
    ins: "~онком", "~атами";
    prep: "~онке", "~атах";
    ),
    // Раз: 53 words (солдат, цыган, раз, гусар, вольт, погон). Like Приём, except in 2 cells: gen_ pl, noun_second_case loc.
    declension!("раз", 53, "~", [gender: Masculine], like Приём;
    gen_: _, "~";
    noun_second_case: _, derived;
    ),
    // Недра: 52 words (ребята, врата, дрова, уста, внучата, правнучата). Like Латы, except in 2 cells: nom sg, nom pl.
    declension!("недра", 52, "~а", like Латы;
    nom: "~а", "~а";
    ),
    // Свеча: 45 words (левша, ханжа, вожжа, свеча, юноша, саранча). Like Выдача, except in 2 cells: gen_ pl, ins sg.
    declension!("свеча", 45, "~а", like Выдача;
    gen_: _, "~ей";
    ins: "~ой|~ою", _;
    ),
    // Каракуля: 45 words (постеля, батя, дядя, доля, ходуля, сводня). Like Баня, except in 1 cell: gen_ pl.
    declension!("каракуля", 45, "~я", like Баня;
    gen_: _, "~ей";
    ),
    // Котище: 40 words (дружище, когтище, волчище, хуище, козлище, третье). Like Ложе, except in 1 cell: nom pl.
    declension!("котище", 40, "~е", [gender: Masculine], like Ложе;
    nom: _, "~и";
    ),
    // Делёж: 38 words (падёж, пиздёж, чертёж, грабёж, платёж, мухлёж).
    declension!("делёж", 38, "~ё~", [gender: Masculine, animacy: Inanimate];
    nom: "~ё~", "~е~и";
    gen_: "~е~а", "~е~ей";
    dat: "~е~у", "~е~ам";
    ins: "~е~ом", "~е~ами";
    prep: "~е~е", "~е~ах";
    ),
    // Съезжая: 37 words (прихожая, телеведущая, неверующая, купчая, составляющая, заведующая). Like Русский, except in 6 cells: nom sg, gen_ sg, dat sg, acc sg, ins sg, prep sg.
    declension!("съезжая", 37, "~ая", [gender: Feminine], like Русский;
    nom: "~ая", _;
    gen_: "~ей", _;
    dat: "~ей", _;
    acc: "~ую", _;
    ins: "~ей|~ею", _;
    prep: "~ей", _;
    ),
    // Помочь: 36 words (мочь, тушь, нощь, пещь, помощь, немощь). Like Склонность, except in 3 cells: dat pl, ins pl, prep pl.
    declension!("помочь", 36, "~ь", [gender: Feminine, animacy: Inanimate], like Склонность;
    dat: _, "~ам";
    ins: _, "~ами";
    prep: _, "~ах";
    ),
    // Отец: 33 words (ветер, узел, пепел, дятел, овен, домысел). Like Барашек, except in 1 cell: nom pl.
    declension!("отец", 33, "~е~", [gender: Masculine], like Барашек;
    nom: _, "~~ы";
    ),
    // Бой: 33 words (чай, бой, рой, слой, край, буй). Like Змей, except in 1 cell: gen_ pl.
    declension!("бой", 33, "~й", [gender: Masculine, animacy: Inanimate], like Змей;
    gen_: _, "~ёв";
    ),
    // Горючее: 33 words (настоящее, будущее, горючее, грядущее, прочее, болеутоляющее). Like Кормчий, except in 1 cell: nom sg.
    declension!("горючее", 33, "~ее", [gender: Neuter, animacy: Inanimate], like Кормчий;
    nom: "~ее", _;
    ),
    // Дрязги: 31 words (брюки, ладоши, лыжи, вериги, пики, брызги). Like Выдача, except in 2 cells: nom sg, ins sg.
    declension!("дрязги", 31, "~и", [gender: Feminine, animacy: Inanimate], like Выдача;
    nom: "~и", _;
    ins: "~ой", _;
    ),
    // Шов: 30 words (рот, свёкор, лоб, сон, хохол, угол). Like Рожок, except in 1 cell: nom pl.
    declension!("шов", 30, "~о~", [gender: Masculine], like Рожок;
    nom: _, "~~ы";
    ),
    // Ладья: 30 words (семья, ладья, статья, судья, скамья, голья). Like Сватья, except in 2 cells: gen_ pl, ins sg.
    declension!("ладья", 30, "~ья", like Сватья;
    gen_: _, "~ей";
    ins: "~ьёй|~ьёю", _;
    ),
    // Член: 30 words (клон, тип, член, кат, кекс, бас). Like Приём, except in 3 cells: acc sg, acc pl, noun_second_case loc.
    declension!("член", 30, "~", [gender: Masculine, animacy: Inanimate], like Приём;
    acc: "~а|~", "~ов|~ы";
    noun_second_case: _, derived;
    ),
    // Деревня: 29 words (торговля, сабля, капля, туфля, чапля, цапля). Like Подворотня, except in 1 cell: gen_ pl.
    declension!("деревня", 29, "~~я", [gender: Feminine], like Подворотня;
    gen_: _, "~е~ь";
    ),
    // Верея: 29 words (змея, колея, чешуя, швея, струя, алия). Like Батарея, except in 1 cell: ins sg.
    declension!("верея", 29, "~я", [gender: Feminine], like Батарея;
    ins: "~ёй|~ёю", _;
    ),
    // Кешью: 29 words (порося, кешью, чапаев, гэта, мюсли, пол-москвы). Like Какао, except in 2 cells: acc sg, acc pl.
    declension!("кешью", 29, "~", like Какао;
    acc: "~", "~";
    ),
    // Порубка: 29 words (скоблянка, меньшевичка, связистка, отклейка, порубка, самосброска). Like Свеча, except in 1 cell: gen_ pl.
    declension!("порубка", 29, "~а", [gender: Feminine], like Свеча;
    gen_: _, "~а";
    ),
    // Егерь: 27 words (учитель, якорь, флигель, лагерь, тополь, соболь). Like Медведь, except in 1 cell: nom pl.
    declension!("егерь", 27, "~ь", [gender: Masculine], like Медведь;
    nom: _, "~я";
    ),
    // Поллитра: 26 words (пол-очка, пол-литра, пол-оборота, пол-августа, пол-автобуса, пол-аршина).
    declension!("пол-литра", 26, "~-~а";
    nom: "~-~а", "~у~ы";
    gen_: "~у~а", "~у~ов";
    dat: "~у~у", "~у~ам";
    ins: "~у~ом", "~у~ами";
    prep: "~у~е", "~у~ах";
    ),
    // Голец: 24 words (лев, телец, стрелец, жилец, делец, малец). Like Умелец, except in 2 cells: gen_ pl, ins sg.
    declension!("голец", 24, "~е~", [gender: Masculine], like Умелец;
    gen_: _, "~ь~ов";
    ins: "~ь~ом", _;
    ),
    // Домашние: 24 words (немногие, сколькие, брюхоногие, близкие, древние, домашние). Like Горючее, except in 1 cell: acc sg.
    declension!("домашние", 24, "~ие", like Горючее;
    acc: "~ее", _;
    ),
    // Звезда: 23 words (звезда, пчела, жена, сестра, медсестра, береста).
    declension!("звезда", 23, "~е~а", [gender: Feminine];
    nom: "~е~а", "~ё~ы";
    gen_: "~е~ы", "~ё~";
    dat: "~е~е", "~ё~ам";
    acc: "~е~у", _;
    ins: "~е~ой|~е~ою", "~ё~ами";
    prep: "~е~е", "~ё~ах";
    ),
    // Лицо: 23 words (существо, лицо, божество, чадо, чмо, падло). Like Сходство, except in 1 cell: acc sg.
    declension!("лицо", 23, "~о", [gender: Neuter, animacy: Animate], like Сходство;
    acc: "~о", _;
    ),
    // Котёл: 22 words (пёс, козёл, орёл, овёс, осёл, костёр). Like Отец, except in 1 cell: nom sg.
    declension!("котёл", 22, "~ё~", [gender: Masculine], like Отец;
    nom: "~ё~", _;
    ),
    // Ребро: 22 words (сребро, серебро, ведро, ребро, весло, зерно).
    declension!("ребро", 22, "~е~~о", [gender: Neuter, animacy: Inanimate];
    nom: "~е~~о", "~ё~~а";
    gen_: "~е~~а", "~ё~е~";
    dat: "~е~~у", "~ё~~ам";
    ins: "~е~~ом", "~ё~~ами";
    prep: "~е~~е", "~ё~~ах";
    ),
    // Маракуйя: 22 words (аллилуйя, секвойя, паранойя, папайя, пирайя, майя). Like Баня, except in 1 cell: gen_ pl.
    declension!("маракуйя", 22, "~я", [gender: Feminine], like Баня;
    gen_: _, "~";
    ),
    // Деньги: 21 words (деньги, четвереньки, грабельки, хахоньки, ахоньки, ясельки). Like Кулички, except in 5 cells: nom sg, nom pl, dat pl, ins pl, prep pl.
    declension!("деньги", 21, "~ь~и", [gender: Feminine, animacy: Inanimate], like Кулички;
    nom: "~ь~и", "~ь~и";
    dat: _, "~ь~ам";
    ins: _, "~ь~ами";
    prep: _, "~ь~ах";
    ),
    // Устье: 21 words (платье, устье, подмастерье, подполье, уголье, отребье). Like Скопление, except in 2 cells: gen_ pl, prep sg.
    declension!("устье", 21, "~е", [animacy: Inanimate], like Скопление;
    gen_: _, "~ев";
    prep: "~е", _;
    ),
    // Клин: 19 words (брат, лист, муж, клин, стул, струп). Like Приём, except in 5 cells: nom pl, gen_ pl, dat pl, ins pl, prep pl.
    declension!("клин", 19, "~", [gender: Masculine], like Приём;
    nom: _, "~ья";
    gen_: _, "~ьев";
    dat: _, "~ьям";
    ins: _, "~ьями";
    prep: _, "~ьях";
    ),
    // Сенцы: 19 words (японцы, плоскогубцы, плоскозубцы, шлёпанцы, сланцы, святцы). Like Счёты, except in 1 cell: gen_ pl.
    declension!("сенцы", 19, "~ы", [gender: Masculine], like Счёты;
    gen_: _, "~ев";
    ),
    // Ископаемое: 18 words (насекомое, животное, земноводное, парнокопытное, ракообразное, позвоночное). Like Второе, except in 1 cell: acc sg.
    declension!("ископаемое", 18, "~ое", [gender: Neuter, animacy: Animate], like Второе;
    acc: "~ое", _;
    ),
    // Боёк: 18 words (паёк, чаёк, ручеёк, кофеёк, боёк, буёк). Like Аниец, except in 4 cells: nom sg, nom pl, gen_ pl, ins sg.
    declension!("боёк", 18, "~ё~", [gender: Masculine], like Аниец;
    nom: "~ё~", "~й~и";
    gen_: _, "~й~ов";
    ins: "~й~ом", _;
    ),
    // Меринос: 18 words (квакер, чернорубашечник, виолончелист, солитер, кастрат, бахмат). Like Член, except in 2 cells: acc sg, acc pl.
    declension!("меринос", 18, "~", [gender: Masculine], like Член;
    acc: "~", "~ы";
    ),
    // Детище: 17 words (чудовище, чудище, детище, посмешище, уёбище, страшилище). Like Ложе, except in 1 cell: acc sg.
    declension!("детище", 17, "~е", [animacy: Animate], like Ложе;
    acc: "~е", _;
    ),
    // Колёсико: 16 words (колёсико, древко, очко, брюшко, личико, пивко). Like Сходство, except in 2 cells: nom pl, gen_ pl.
    declension!("колёсико", 16, "~о", like Сходство;
    nom: _, "~и";
    gen_: _, "~ов";
    ),
    // Анкара: 15 words (ковурма, дисна, полумгла, мста, ангара, анкара). Like Полнота, except in 1 cell: gen_ pl.
    declension!("анкара", 15, "~а", [gender: Feminine], like Полнота;
    gen_: _, "~а";
    ),
    // День: 15 words (день, ремень, пень, кремень, плетень, трудодень). Like Камень, except in 2 cells: ins sg, noun_second_case loc.
    declension!("день", 15, "~е~ь", [gender: Masculine], like Камень;
    ins: "~~ём", _;
    noun_second_case: _, "~~ю";
    ),
    // Зло: 15 words (окно, зло, сукно, волокно, жалко, тягло). Like Ядро, except in 1 cell: gen_ pl.
    declension!("зло", 15, "~~о", [gender: Neuter, animacy: Inanimate], like Ядро;
    gen_: _, "~о~";
    ),
    // Бойня: 15 words (кофейня, бойня, водогрейня, воскобойня, двойня, маслобойня). Like Чайка, except in 6 cells: nom sg, dat pl, acc sg, ins sg, ins pl, prep pl.
    declension!("бойня", 15, "~й~я", [gender: Feminine, animacy: Inanimate], like Чайка;
    nom: "~й~я", _;
    dat: _, "~й~ям";
    acc: "~й~ю", _;
    ins: "~й~ей|~й~ею", "~й~ями";
    prep: _, "~й~ях";
    ),
    // Побои: 15 words (обои, побои, помои, прелиминарии, покои, фотообои). Like Ясли, except in 1 cell: gen_ pl.
    declension!("побои", 15, "~и", [gender: Masculine], like Ясли;
    gen_: _, "~ев";
    ),
    // Перо: 14 words (дерево, перо, крыло, сено, авиазвено, звено). Like Клин, except in 2 cells: nom sg, noun_second_case loc.
    declension!("перо", 14, "~о", [gender: Neuter, animacy: Inanimate], like Клин;
    nom: "~о", _;
    noun_second_case: _, derived;
    ),
    // Кокцинеллида: 14 words (ебала, ламиина, иссида, кокцинеллида, алеохарина, циксиида). Like Полнота, except in 1 cell: acc pl.
    declension!("кокцинеллида", 14, "~а", [animacy: Animate], like Полнота;
    acc: _, "~ы";
    ),
    // Губоцветные: 14 words (вересковые, водяниковые, верескоцветные, буковые, букоцветные, мальпигиевые).
    declension!("губоцветные", 14, "~е", [gender: Feminine, animacy: Inanimate];
    nom: "~е", _;
    gen_: "~х", _;
    dat: "~м", _;
    ins: "~ми", _;
    prep: "~х", _;
    ),
    // Перешеек: 14 words (боец, перешеек, хуец, закраек, зашеек, опоек). Like Боёк, except in 1 cell: nom sg.
    declension!("перешеек", 14, "~е~", [gender: Masculine], like Боёк;
    nom: "~е~", _;
    ),
    // Сердце: 13 words (сердце, деревце, полотенце, блюдце, оконце, волоконце). Like Ядро, except in 2 cells: nom sg, ins sg.
    declension!("сердце", 13, "~~е", [gender: Neuter, animacy: Inanimate], like Ядро;
    nom: "~~е", _;
    ins: "~~ем", _;
    ),
    // Гнездо: 13 words (озеро, колесо, село, чело, решето, гнездо). Like Делёж, except in 6 cells: nom sg, nom pl, gen_ pl, dat pl, ins pl, prep pl.
    declension!("гнездо", 13, "~е~о", [gender: Neuter, animacy: Inanimate], like Делёж;
    nom: "~е~о", "~ё~а";
    gen_: _, "~ё~";
    dat: _, "~ё~ам";
    ins: _, "~ё~ами";
    prep: _, "~ё~ах";
    ),
    // Котёночек: 13 words (цыплёночек, котёночек, жеребёночек, козлёночек, утёночек, ягнёночек).
    declension!("котёночек", 13, "~ёноче~", [gender: Masculine];
    nom: "~ёноче~", "~ят~и";
    gen_: "~ёноч~а", "~ято~";
    dat: "~ёноч~у", "~ят~ам";
    ins: "~ёноч~ом", "~ят~ами";
    prep: "~ёноч~е", "~ят~ах";
    ),
    // Фекалии: 12 words (химикалии, фекалии, авиалинии, гениталии, сатурналии, реалии). Like Ясли, except in 1 cell: gen_ pl.
    declension!("фекалии", 12, "~и", [gender: Feminine], like Ясли;
    gen_: _, "~й";
    ),
    // Кольцо: 12 words (письмо, дерьмо, кольцо, крыльцо, сельцо, автокольцо). Like Гусёк, except in 3 cells: nom sg, nom pl, gen_ pl.
    declension!("кольцо", 12, "~ь~о", [gender: Neuter, animacy: Inanimate], like Гусёк;
    nom: "~ь~о", "~ь~а";
    gen_: _, "~е~";
    ),
    // Рыльце: 12 words (зеркальце, щупальце, дельце, рыльце, сальце, мыльце). Like Кольцо, except in 2 cells: nom sg, ins sg.
    declension!("рыльце", 12, "~ь~е", [gender: Neuter, animacy: Inanimate], like Кольцо;
    nom: "~ь~е", _;
    ins: "~ь~ем", _;
    ),
    // Мощи: 11 words (щи, клещи, мощи, беруши, марши, кириши). Like Счёты, except in 3 cells: nom sg, nom pl, gen_ pl.
    declension!("мощи", 11, "~и", like Счёты;
    nom: "~и", "~и";
    gen_: _, "~ей";
    ),
    // Валенок: 11 words (ботинок, чулок, турок, валенок, глазок, зубок). Like Рожок, except in 2 cells: gen_ pl, noun_second_case loc.
    declension!("валенок", 11, "~о~", [gender: Masculine], like Рожок;
    gen_: _, "~о~";
    noun_second_case: _, derived;
    ),
    // Счёт: 11 words (мёд, счёт, шёлк, жёрнов, жёлоб, аэрожёлоб). Like Делёж, except in 7 cells: nom pl, gen_ sg, gen_ pl, dat sg, ins sg, prep sg, ….
    declension!("счёт", 11, "~ё~", [gender: Masculine, animacy: Inanimate], like Делёж;
    nom: _, "~е~а";
    gen_: "~ё~а", "~е~ов";
    dat: "~ё~у", _;
    ins: "~ё~ом", _;
    prep: "~ё~е", _;
    noun_second_case: _, "~е~у";
    ),
    // Прения: 11 words (прения, условия, переживания, усилия, воспоминания, взаимоотношения). Like Фекалии, except in 2 cells: nom sg, nom pl.
    declension!("прения", 11, "~я", [gender: Neuter, animacy: Inanimate], like Фекалии;
    nom: "~я", "~я";
    ),
    // Тельце: 11 words (тельце, весельце, дульце, жальце, зальце, одеяльце). Like Рыльце, except in 1 cell: gen_ pl.
    declension!("тельце", 11, "~ь~е", [gender: Neuter, animacy: Inanimate], like Рыльце;
    gen_: _, "~ь~ев|~е~";
    ),
    // Племя: 10 words (имя, время, знамя, вымя, пламя, племя).
    declension!("племя", 10, "~я", [gender: Neuter, animacy: Inanimate];
    nom: "~я", "~ена";
    gen_: "~ени", "~ён";
    dat: "~ени", "~енам";
    ins: "~енем", "~енами";
    prep: "~ени", "~енах";
    ),
    // Хлопья: 10 words (хлопья, лохмотья, объедья, брусья, коренья, стручья). Like Прения, except in 1 cell: gen_ pl.
    declension!("хлопья", 10, "~я", [animacy: Inanimate], like Прения;
    gen_: _, "~ев";
    ),
    // Болотце: 10 words (болотце, коленце, ведёрце, корытце, долотце, донце). Like Сердце, except in 1 cell: gen_ pl.
    declension!("болотце", 10, "~~е", [gender: Neuter, animacy: Inanimate], like Сердце;
    gen_: _, "~~ев|~е~";
    ),
    // Кукла: 9 words (мгла, кукла, мошна, княжна, свёкла, свекла). Like Гривна, except in 1 cell: gen_ pl.
    declension!("кукла", 9, "~~а", like Гривна;
    gen_: _, "~о~";
    ),
    // Плечо: 8 words (яблоко, молоко, млеко, веко, плечо, брюхо). Like Сходство, except in 1 cell: nom pl.
    declension!("плечо", 8, "~о", [animacy: Inanimate], like Сходство;
    nom: _, "~и";
    ),
    // Горе: 8 words (море, горе, поле, перекати-поле, муде, биополе). Like Егерь, except in 1 cell: nom sg.
    declension!("горе", 8, "~е", [gender: Neuter, animacy: Inanimate], like Егерь;
    nom: "~е", _;
    ),
    // Герц: 8 words (герц, гигагерц, килогерц, мегагерц, миллигерц, кольбац). Like Раз, except in 1 cell: ins sg.
    declension!("герц", 8, "~", [gender: Masculine, animacy: Inanimate], like Раз;
    ins: "~ем", _;
    ),
    // Калашников: 8 words (калашников, басков, топтыгин, обещалкин, сердечкин, собакин). Like Зелёный, except in 5 cells: nom sg, nom pl, gen_ sg, dat sg, prep sg.
    declension!("калашников", 8, "~", [gender: Masculine], like Зелёный;
    nom: "~", "~ы";
    gen_: "~а", _;
    dat: "~у", _;
    prep: "~е", _;
    ),
    // Плюхино: 8 words (мотовство, плюхино, плюхново, кабаново, клементьево, бахтино).
    declension!("плюхино", 8, "~о", [gender: Neuter, animacy: Inanimate];
    nom: "~о", "~а|~о";
    gen_: "~а|~о", "~о";
    dat: "~у|~о", "~ам|~о";
    ins: "~ом|~о", "~ами|~о";
    prep: "~е|~о", "~ах|~о";
    ),
    // Полюбки: 8 words (пол-ёмкости, пол-линии, пол-ложки, пол-области, пол-юбки, пол-осьмухи).
    declension!("пол-юбки", 8, "~-~и";
    nom: "~-~и", "~у~и";
    gen_: "~у~и", "~у~ей";
    dat: "~у~и", "~у~ям";
    ins: "~у~ью", "~у~ями";
    prep: "~у~и", "~у~ях";
    ),
    // Ложь: 7 words (церковь, рожь, вошь, ложь, любовь, нелюбовь). Like Камень, except in 5 cells: nom sg, gen_ sg, dat sg, ins sg, prep sg.
    declension!("ложь", 7, "~о~ь", [gender: Feminine], like Камень;
    nom: "~о~ь", _;
    gen_: "~~и", _;
    dat: "~~и", _;
    ins: "~о~ью", _;
    prep: "~~и", _;
    ),
    // Лапоть: 7 words (локоть, дёготь, ноготь, уголь, коготь, лапоть). Like Камень, except in 1 cell: nom sg.
    declension!("лапоть", 7, "~о~ь", [gender: Masculine, animacy: Inanimate], like Камень;
    nom: "~о~ь", _;
    ),
    // Череда: 7 words (середа, железа, слеза, пелена, череда, желна). Like Звезда, except in 4 cells: nom pl, dat pl, ins pl, prep pl.
    declension!("череда", 7, "~е~а", [gender: Feminine], like Звезда;
    nom: _, "~е~ы";
    dat: _, "~е~ам";
    ins: _, "~е~ами";
    prep: _, "~е~ах";
    ),
    // Сиверко: 7 words (ведёрко, сиверко, железко, весёлко, одеялко, морозко). Like Молочко, except in 1 cell: gen_ pl.
    declension!("сиверко", 7, "~~о", like Молочко;
    gen_: _, "~о~";
    ),
    // Передняя: 7 words (передняя, преисподняя, ближняя, замужняя, незамужняя, несовершеннолетняя). Like Съезжая, except in 2 cells: nom sg, acc sg.
    declension!("передняя", 7, "~яя", [gender: Feminine], like Съезжая;
    nom: "~яя", _;
    acc: "~юю", _;
    ),
    // Семя: 7 words (бремя, темя, семя, стремя, беремя, льносемя). Like Племя, except in 1 cell: gen_ pl.
    declension!("семя", 7, "~я", [gender: Neuter, animacy: Inanimate], like Племя;
    gen_: _, "~ян";
    ),
    // Стекло: 7 words (стекло, бронестекло, стегно, пекло, наностекло, оргстекло). Like Ребро, except in 1 cell: gen_ pl.
    declension!("стекло", 7, "~е~~о", [gender: Neuter, animacy: Inanimate], like Ребро;
    gen_: _, "~ё~о~";
    ),
    // Заём: 7 words (заём, наём, поднаём, перенаём, госзаём, сберзаём). Like Боёк, except in 1 cell: nom pl.
    declension!("заём", 7, "~ё~", [gender: Masculine, animacy: Inanimate], like Боёк;
    nom: _, "~й~ы";
    ),
    // Весна: 7 words (десна, весна, метла, ветла, гидрометла, блесна). Like Ребро, except in 6 cells: nom sg, nom pl, gen_ sg, dat sg, acc sg, ins sg.
    declension!("весна", 7, "~е~~а", [gender: Feminine, animacy: Inanimate], like Ребро;
    nom: "~е~~а", "~ё~~ы";
    gen_: "~е~~ы", _;
    dat: "~е~~е", _;
    acc: "~е~~у", _;
    ins: "~е~~ой|~е~~ою", _;
    ),
    // Осётр: 7 words (ксёндз, чёлн, осётр, черёд, перёд, клёст). Like Делёж, except in 2 cells: nom pl, gen_ pl.
    declension!("осётр", 7, "~ё~", [gender: Masculine], like Делёж;
    nom: _, "~е~ы";
    gen_: _, "~е~ов";
    ),
    // Полгода: 7 words (полчаса, полгода, полметра, полмиллиона, полкилометра, поллитра). Like Поллитра, except in 1 cell: nom sg.
    declension!("полгода", 7, "~~а", [animacy: Inanimate], like Поллитра;
    nom: "~~а", _;
    ),
    // Дитё: 7 words (мумиё, дитё, хулиганьё, гадьё, мудачьё, татарьё). Like Червь, except in 3 cells: nom sg, nom pl, acc sg.
    declension!("дитё", 7, "~ё", like Червь;
    nom: "~ё", "~я";
    acc: "~ё", _;
    ),
    // Полиюля: 7 words (пол-апреля, пол-имения, пол-июля, пол-июня, пол-лагеря, пол-октября). Like Полюбки, except in 5 cells: nom sg, gen_ sg, dat sg, ins sg, prep sg.
    declension!("пол-июля", 7, "~-~я", like Полюбки;
    nom: "~-~я", _;
    gen_: "~у~я", _;
    dat: "~у~ю", _;
    ins: "~у~ем", _;
    prep: "~у~е", _;
    ),
    // Полигры: 7 words (пол-игры, пол-избы, пол-луковицы, пол-юрты, пол-ягоды, пол-луны). Like Поллитра, except in 5 cells: nom sg, gen_ sg, gen_ pl, dat sg, ins sg.
    declension!("пол-игры", 7, "~-~ы", like Поллитра;
    nom: "~-~ы", _;
    gen_: "~у~ы", "~у~";
    dat: "~у~е", _;
    ins: "~у~ой", _;
    ),
    // Кто: 6 words (что, кто, кое-кто, никто, ничто, кое-что). Like Зелёный, except in 7 cells: nom sg, nom pl, gen_ pl, dat pl, ins sg, ins pl, ….
    declension!("кто", 6, "~то", like Зелёный;
    nom: "~то", "~то";
    gen_: _, "~ого";
    dat: _, "~ому";
    ins: "~ем", "~ем";
    prep: _, "~ом";
    ),
    // Млекопитающее: 6 words (млекопитающее, окружающее, обезболивающее, иглокожее, кишечнодышащее, простейшее). Like Домашние, except in 0 cells: .
    declension!("млекопитающее", 6, "~ее", [gender: Neuter, animacy: Animate], like Домашние;
    ),
    // Татарин: 6 words (болгарин, татарин, хазарин, угрин, долганин, хариджанин). Like Господин, except in 1 cell: nom pl.
    declension!("татарин", 6, "~ин", [gender: Masculine, animacy: Animate], like Господин;
    nom: _, "~ы";
    ),
    // Чечётка: 6 words (чечётка, метёлка, кержанка, разведёнка, скалолазка, удмуртка). Like Стрелка, except in 1 cell: acc pl.
    declension!("чечётка", 6, "~~а", [gender: Feminine, animacy: Inanimate], like Стрелка;
    acc: _, "~о~";
    ),
    // Зеленя: 6 words (зеленя, ебеня, труселя, прохоря, чигиря, уборя). Like Прения, except in 1 cell: gen_ pl.
    declension!("зеленя", 6, "~я", [gender: Masculine, animacy: Inanimate], like Прения;
    gen_: _, "~ей";
    ),
    // Исайя: 6 words (исайя, хориоидея, заурея, стеаторея, ципрея, персея). Like Батарея, except in 1 cell: acc pl.
    declension!("исайя", 6, "~я", like Батарея;
    acc: _, "~й";
    ),
    // Поллета: 6 words (пол-лета, пол-лица, пол-лота, пол-окна, пол-училища, пол-ящика). Like Поллитра, except in 2 cells: nom pl, gen_ pl.
    declension!("пол-лета", 6, "~-~а", like Поллитра;
    nom: _, "~у~а";
    gen_: _, "~у~";
    ),
    // Юноша: 5 words (чукча, раджа, обжа, корча, магараджа). Like Выдача, except in 1 cell: gen_ pl.
    declension!("юноша", 5, "~а", like Выдача;
    gen_: _, "~ей";
    ),
    // Огонь: 5 words (огонь, угорь, ломоть, артогонь, стоп-огонь). Like Лапоть, except in 1 cell: ins sg.
    declension!("огонь", 5, "~о~ь", [gender: Masculine], like Лапоть;
    ins: "~~ём", _;
    ),
    // Кухня: 5 words (кухня, куфня, кормокухня, авиакухня, автокухня). Like Подворотня, except in 1 cell: gen_ pl.
    declension!("кухня", 5, "~~я", [gender: Feminine, animacy: Inanimate], like Подворотня;
    gen_: _, "~о~ь";
    ),
    // Козлы: 5 words (ножны, бубны, козлы, люберцы, ливны). Like Кулички, except in 2 cells: nom sg, nom pl.
    declension!("козлы", 5, "~~ы", [gender: Feminine, animacy: Inanimate], like Кулички;
    nom: "~~ы", "~~ы";
    ),
    // Репей: 5 words (соловей, воробей, муравей, ручей, репей). Like Жнивьё, except in 3 cells: nom sg, nom pl, gen_ pl.
    declension!("репей", 5, "~ей", [gender: Masculine], like Жнивьё;
    nom: "~ей", "~ьи";
    gen_: _, "~ьёв";
    ),
    // Хозяин: 5 words (хозяин, домохозяин, квартирохозяин, сохозяин, судохозяин). Like Господин, except in 5 cells: nom pl, gen_ pl, dat pl, ins pl, prep pl.
    declension!("хозяин", 5, "~ин", [gender: Masculine, animacy: Animate], like Господин;
    nom: _, "~ева";
    gen_: _, "~ев";
    dat: _, "~евам";
    ins: _, "~евами";
    prep: _, "~евах";
    ),
    // Бега: 5 words (бега, торока, потроха, холода, севера). Like Недра, except in 1 cell: gen_ pl.
    declension!("бега", 5, "~а", [gender: Masculine, animacy: Inanimate], like Недра;
    gen_: _, "~ов";
    ),
    // Ботиночек: 5 words (ботиночек, затылочек, катерочек, валеночек, чулочек). Like Барашек, except in 1 cell: gen_ pl.
    declension!("ботиночек", 5, "~е~", [gender: Masculine, animacy: Inanimate], like Барашек;
    gen_: _, "~е~";
    ),
    // Полкроны: 5 words (полкроны, полстраны, полшколы, ползалупы, полпланеты). Like Полигры, except in 1 cell: nom sg.
    declension!("полкроны", 5, "~~ы", like Полигры;
    nom: "~~ы", _;
    ),
    // Тиса: 5 words (линда, шила, тиса, драва, пуста). Like Недра, except in 3 cells: dat pl, ins pl, prep pl.
    declension!("тиса", 5, "~а", [gender: Feminine], like Недра;
    dat: _, "~а";
    ins: _, "~а";
    prep: _, "~а";
    ),
    // Копьё: 4 words (копьё, тряпьё, тупьё, репьё). Like Жнивьё, except in 1 cell: gen_ pl.
    declension!("копьё", 4, "~ьё", [gender: Neuter, animacy: Inanimate], like Жнивьё;
    gen_: _, "~ий";
    ),
    // Зуй: 4 words (хуй, зуй, пидрахуй, суй). Like Бой, except in 3 cells: acc sg, acc pl, noun_second_case loc.
    declension!("зуй", 4, "~й", [gender: Masculine], like Бой;
    acc: "~я", "~ёв";
    noun_second_case: _, "~ю";
    ),
    // Небо: 4 words (чудо, небо, древо, худо). Like Сходство, except in 5 cells: nom pl, gen_ pl, dat pl, ins pl, prep pl.
    declension!("небо", 4, "~о", [gender: Neuter, animacy: Inanimate], like Сходство;
    nom: _, "~еса";
    gen_: _, "~ес";
    dat: _, "~есам";
    ins: _, "~есами";
    prep: _, "~есах";
    ),
    // Судно: 4 words (судно, агитсудно, авиасудно, фальшдно). Like Бега, except in 5 cells: nom sg, gen_ sg, dat sg, ins sg, prep sg.
    declension!("судно", 4, "~но", [gender: Neuter, animacy: Inanimate], like Бега;
    nom: "~но", _;
    gen_: "~на", _;
    dat: "~ну", _;
    ins: "~ном", _;
    prep: "~не", _;
    ),
    // Улей: 4 words (улей, жеребей, чирей, ирей). Like Репей, except in 2 cells: gen_ pl, ins sg.
    declension!("улей", 4, "~ей", [gender: Masculine, animacy: Inanimate], like Репей;
    gen_: _, "~ьев";
    ins: "~ьем", _;
    ),
    // Учащийся: 4 words (учащийся, трудящийся, вольноопределяющийся, праздношатающийся).
    declension!("учащийся", 4, "~ий~", [gender: Masculine, animacy: Animate];
    nom: "~ий~", "~ие~";
    gen_: "~его~", "~их~";
    dat: "~ему~", "~им~";
    ins: "~им~", "~ими~";
    prep: "~ем~", "~их~";
    ),
    // Лишай: 4 words (холуй, лишай, бугай, валуй). Like Бой, except in 1 cell: ins sg.
    declension!("лишай", 4, "~й", [gender: Masculine], like Бой;
    ins: "~ём", _;
    ),
    // Капли: 4 words (грабли, капли, чапли, велотуфли). Like Нападки, except in 4 cells: gen_ pl, dat pl, ins pl, prep pl.
    declension!("капли", 4, "~~и", [gender: Feminine, animacy: Inanimate], like Нападки;
    gen_: _, "~е~ь";
    dat: _, "~~ям";
    ins: _, "~~ями";
    prep: _, "~~ях";
    ),
    // Пресмыкающееся: 4 words (пресмыкающееся, случившееся, сбывшееся, несбывшееся). Like Учащийся, except in 2 cells: nom sg, acc sg.
    declension!("пресмыкающееся", 4, "~ее~", [gender: Neuter], like Учащийся;
    nom: "~ее~", _;
    acc: "~ее~", _;
    ),
    // Остриё: 4 words (остриё, бытиё, востриё, лезвеё). Like Дитё, except in 2 cells: gen_ pl, acc sg.
    declension!("остриё", 4, "~ё", [gender: Neuter, animacy: Inanimate], like Дитё;
    gen_: _, "~ёв";
    acc: derived, _;
    ),
    // Ничья: 4 words (ничья, третья, девичья, моя). Like Съезжая, except in 3 cells: nom sg, nom pl, acc sg.
    declension!("ничья", 4, "~я", [gender: Feminine], like Съезжая;
    nom: "~я", "~и";
    acc: "~ю", _;
    ),
    // Яйки: 4 words (обойки, валуйки, яйки, прощайки). Like Кулички, except in 5 cells: nom sg, nom pl, dat pl, ins pl, prep pl.
    declension!("яйки", 4, "~й~и", [animacy: Inanimate], like Кулички;
    nom: "~й~и", "~й~и";
    dat: _, "~й~ам";
    ins: _, "~й~ами";
    prep: _, "~й~ах";
    ),
    // Сынуля: 4 words (сынуля, саня, меля, игруля). Like Баня, except in 1 cell: acc pl.
    declension!("сынуля", 4, "~я", [gender: Masculine, animacy: Animate], like Баня;
    acc: _, "~и";
    ),
    // Карло: 4 words (ракло, шарыпово, запевало, карло). Like Сходство, except in 1 cell: nom pl.
    declension!("карло", 4, "~о", like Сходство;
    nom: _, "~ы";
    ),
    // Дитятко: 4 words (дитятко, старичонко, золотко, мужичонко). Like Сиверко, except in 1 cell: acc sg.
    declension!("дитятко", 4, "~~о", [animacy: Animate], like Сиверко;
    acc: "~~о", _;
    ),
    // Усадьба: 3 words (тьма, тюрьма, потьма). Like Люлька, except in 2 cells: nom pl, gen_ sg.
    declension!("усадьба", 3, "~ь~а", [gender: Feminine, animacy: Inanimate], like Люлька;
    nom: _, "~ь~ы";
    gen_: "~ь~ы", _;
    ),
    // Дичь: 3 words (мышь, дичь, водомышь). Like Помочь, except in 2 cells: acc sg, noun_second_case loc.
    declension!("дичь", 3, "~ь", [gender: Feminine, animacy: Animate], like Помочь;
    acc: "~ь", _;
    noun_second_case: _, derived;
    ),
    // Глаз: 3 words (глаз, телеглаз, киноглаз). Like Сходство, except in 1 cell: nom sg.
    declension!("глаз", 3, "~", [gender: Masculine, animacy: Inanimate], like Сходство;
    nom: "~", _;
    ),
    // Земля: 3 words (земля, петля, полупетля). Like Деревня, except in 1 cell: ins sg.
    declension!("земля", 3, "~~я", [gender: Feminine, animacy: Inanimate], like Деревня;
    ins: "~~ёй|~~ёю", _;
    ),
    // Путь: 3 words (путь, автопуть, севморпуть). Like Склонность, except in 2 cells: ins sg, noun_second_case loc.
    declension!("путь", 3, "~ь", [gender: Masculine, animacy: Inanimate], like Склонность;
    ins: "~ём", _;
    noun_second_case: _, derived;
    ),
    // Лёд: 3 words (лёд, лён, снеголёд). Like Гусёк, except in 2 cells: nom pl, noun_second_case loc.
    declension!("лёд", 3, "~ё~", [gender: Masculine, animacy: Inanimate], like Гусёк;
    nom: _, "~ь~ы";
    noun_second_case: _, "~ь~у";
    ),
    // Сколько: 3 words (сколько, столько, несколько). Like Счёты, except in 6 cells: nom sg, nom pl, gen_ pl, dat pl, ins pl, prep pl.
    declension!("сколько", 3, "~о", like Счёты;
    nom: "~о", "~о";
    gen_: _, "~их|~их";
    dat: _, "~им|~им";
    ins: _, "~ими|~ими";
    prep: _, "~их|~их";
    ),
    // Дрянь: 3 words (смерть, дрянь, мразь). Like Склонность, except in 2 cells: acc pl, noun_second_case loc.
    declension!("дрянь", 3, "~ь", [gender: Feminine, animacy: Inanimate], like Склонность;
    acc: _, "~ей|~и";
    noun_second_case: _, derived;
    ),
    // Епитимья: 3 words (ектенья, паремья, епитимья). Like Сватья, except in 1 cell: ins sg.
    declension!("епитимья", 3, "~ья", [gender: Feminine, animacy: Inanimate], like Сватья;
    ins: "~ьёй|~ьёю", _;
    ),
    // Дверца: 3 words (дверца, шпорца, мадерца). Like Гривна, except in 1 cell: ins sg.
    declension!("дверца", 3, "~~а", [gender: Feminine, animacy: Inanimate], like Гривна;
    ins: "~~ей|~~ею", _;
    ),
    // Солнышко: 3 words (солнышко, нещечко, студентишко). Like Молочко, except in 1 cell: acc sg.
    declension!("солнышко", 3, "~~о", [animacy: Animate], like Молочко;
    acc: "~~о", _;
    ),
    // Получеловек: 3 words (получеловек, обезьяночеловек, первочеловек).
    declension!("получеловек", 3, "~че~овек", [gender: Masculine, animacy: Animate];
    nom: "~че~овек", "~~юди";
    gen_: "~че~овека", "~~юдей";
    dat: "~че~овеку", "~~юдям";
    ins: "~че~овеком", "~~юдьми";
    prep: "~че~овеке", "~~юдях";
    ),
    // Морячка: 3 words (морячка, рыбачка, тестюшка). Like Дорожка, except in 1 cell: acc pl.
    declension!("морячка", 3, "~~а", [animacy: Animate], like Дорожка;
    acc: _, "~~и";
    ),
    // Поллеса: 3 words (пол-леса, пол-округа, пол-острова). Like Поллитра, except in 1 cell: nom pl.
    declension!("пол-леса", 3, "~-~а", like Поллитра;
    nom: _, "~у~а";
    ),
];
