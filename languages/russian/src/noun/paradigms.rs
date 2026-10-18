//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes (`~а`, `полнота`: the stem and `а`). A stem in two pieces (`~~а`, `стрелка`) has
//! letters between them that other forms may drop or change.
//! Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (if they share them),
/// the declension it is like (if any), the stem slots it states with a stem and an
/// ending table
/// (`stem pl: \"~- like приём\"`: the slot's cells are `приём`'s table's patterns
/// with `~` for their `~`) (see `stems.rs`), then a row of patterns per line
/// (`derived`: the form comes from another cell's, see `relations.rs`; `_`,
/// or a row left out: the form is its stem slot's, or the declension's it is
/// like, or, in a declension like no other, derived):
/// `nom` (sg, pl)
/// `gen_` (sg, pl)
/// `dat` (sg, pl)
/// `acc` (sg, pl)
/// `ins` (sg, pl)
/// `prep` (sg, pl)
/// `noun_second_case` (part, loc)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, [$($f:ident: $v:ident),*])? $(, like $parent:ident $(as $view:ident)? $(with $affix:literal $(, $other:literal after $letters:literal)?)?)?;
     $(stem lemma: $s0:tt;)?
     $(stem pl: $s1:tt;)?
     $(stem gen_sg: $s2:tt;)?
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
            reads: view!($($($view)?)?),
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
            stems: ::std::borrow::Cow::Borrowed(concat!(
                $(flexion::cell!($s0),)? "\t",
                $(flexion::cell!($s1),)? "\t",
                $(flexion::cell!($s2),)? "\t",
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

/// What a declension adds after each pattern of the one it is like (`with "", "о" after "~"`: ``, or `о`
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
    /// Like `приём`: 37,921 words.
    Приём,
    /// Like `полнота`: 12,531 words.
    Полнота,
    /// Like `скопление`: 9,052 words.
    Скопление,
    /// Like `склонность`: 7,529 words.
    Склонность,
    /// Like `стрелка`: 6,666 words.
    Стрелка,
    /// Like `химия`: 6,581 words.
    Химия,
    /// Like `какао`: 3,419 words.
    Какао,
    /// Like `хлопанье`: 2,660 words.
    Хлопанье,
    /// Like `дорожка`: 2,519 words.
    Дорожка,
    /// Like `сходство`: 2,323 words.
    Сходство,
    /// Like `агнец`: 2,159 words.
    Агнец,
    /// Like `медведь`: 1,979 words.
    Медведь,
    /// Like `щиток`: 1,057 words.
    Щиток,
    /// Like `товарищ`: 945 words.
    Товарищ,
    /// Like `змей`: 658 words.
    Змей,
    /// Like `гений`: 649 words.
    Гений,
    /// Like `барашек`: 492 words.
    Барашек,
    /// Like `чайка`: 474 words.
    Чайка,
    /// Like `накладная`: 415 words.
    Накладная,
    /// Like `мещанин`: 396 words.
    Мещанин,
    /// Like `ложе`: 327 words.
    Ложе,
    /// Like `зелёный`: 278 words.
    Зелёный,
    /// Like `ключ`: 277 words.
    Ключ,
    /// Like `люлька`: 263 words.
    Люлька,
    /// Like `баня`: 253 words.
    Баня,
    /// Like `аниец`: 250 words.
    Аниец,
    /// Like `червь`: 232 words.
    Червь,
    /// Like `шоколадница`: 218 words.
    Шоколадница,
    /// Like `домашние`: 215 words.
    Домашние,
    /// Like `постав`: 206 words.
    Постав,
    /// Like `счёты`: 203 words.
    Счёты,
    /// Like `русский`: 200 words.
    Русский,
    /// Like `батарея`: 195 words.
    Батарея,
    /// Like `отец`: 179 words.
    Отец,
    /// Like `второе`: 179 words.
    Второе,
    /// Like `умелец`: 174 words.
    Умелец,
    /// Like `молочко`: 153 words.
    Молочко,
    /// Like `латы`: 150 words.
    Латы,
    /// Like `сватья`: 149 words.
    Сватья,
    /// Like `мостки`: 144 words.
    Мостки,
    /// Like `фигня`: 128 words.
    Фигня,
    /// Like `орлёнок`: 125 words.
    Орлёнок,
    /// Like `подворотня`: 120 words.
    Подворотня,
    /// Like `гривна`: 114 words.
    Гривна,
    /// Like `ясли`: 113 words.
    Ясли,
    /// Like `кормчий`: 107 words.
    Кормчий,
    /// Like `камень`: 107 words.
    Камень,
    /// Like `нападки`: 100 words.
    Нападки,
    /// Like `гусёк`: 89 words.
    Гусёк,
    /// Like `выдача`: 85 words.
    Выдача,
    /// Like `спальня`: 85 words.
    Спальня,
    /// Like `достопримечательность`: 81 words.
    Достопримечательность,
    /// Like `штрафной`: 79 words.
    Штрафной,
    /// Like `кулички`: 75 words.
    Кулички,
    /// Like `дитё`: 74 words.
    Дитё,
    /// Like `абзац`: 63 words.
    Абзац,
    /// Like `ядро`: 59 words.
    Ядро,
    /// Like `внучонок`: 57 words.
    Внучонок,
    /// Like `бельё`: 56 words.
    Бельё,
    /// Like `раз`: 54 words.
    Раз,
    /// Like `свеча`: 54 words.
    Свеча,
    /// Like `недра`: 52 words.
    Недра,
    /// Like `каракуля`: 45 words.
    Каракуля,
    /// Like `котище`: 40 words.
    Котище,
    /// Like `делёж`: 38 words.
    Делёж,
    /// Like `съезжая`: 37 words.
    Съезжая,
    /// Like `помочь`: 37 words.
    Помочь,
    /// Like `дрязги`: 33 words.
    Дрязги,
    /// Like `горючее`: 32 words.
    Горючее,
    /// Like `шов`: 30 words.
    Шов,
    /// Like `член`: 30 words.
    Член,
    /// Like `кешью`: 30 words.
    Кешью,
    /// Like `порубка`: 30 words.
    Порубка,
    /// Like `ладья`: 29 words.
    Ладья,
    /// Like `деревня`: 29 words.
    Деревня,
    /// Like `верея`: 29 words.
    Верея,
    /// Like `бой`: 28 words.
    Бой,
    /// Like `егерь`: 28 words.
    Егерь,
    /// Like `голец`: 24 words.
    Голец,
    /// Like `звезда`: 24 words.
    Звезда,
    /// Like `божество`: 23 words.
    Божество,
    /// Like `ребро`: 23 words.
    Ребро,
    /// Like `котёл`: 22 words.
    Котёл,
    /// Like `устье`: 21 words.
    Устье,
    /// Like `деньги`: 21 words.
    Деньги,
    /// Like `тельце`: 21 words.
    Тельце,
    /// Like `маракуйя`: 21 words.
    Маракуйя,
    /// Like `клин`: 20 words.
    Клин,
    /// Like `сенцы`: 19 words.
    Сенцы,
    /// Like `меринос`: 18 words.
    Меринос,
    /// Like `боёк`: 18 words.
    Боёк,
    /// Like `детище`: 17 words.
    Детище,
    /// Like `кокцинеллида`: 16 words.
    Кокцинеллида,
    /// Like `колёсико`: 16 words.
    Колёсико,
    /// Like `анкара`: 16 words.
    Анкара,
    /// Like `перо`: 16 words.
    Перо,
    /// Like `день`: 15 words.
    День,
    /// Like `бойня`: 15 words.
    Бойня,
    /// Like `побои`: 15 words.
    Побои,
    /// Like `зло`: 15 words.
    Зло,
    /// Like `дух`: 14 words.
    Дух,
    /// Like `губоцветные`: 14 words.
    Губоцветные,
    /// Like `котёночек`: 14 words.
    Котёночек,
    /// Like `сердце`: 13 words.
    Сердце,
    /// Like `гнездо`: 13 words.
    Гнездо,
    /// Like `племя`: 13 words.
    Племя,
    /// Like `плюхино`: 13 words.
    Плюхино,
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
    /// Like `хлопья`: 10 words.
    Хлопья,
    /// Like `кукла`: 9 words.
    Кукла,
    /// Like `перешеек`: 9 words.
    Перешеек,
    /// Like `череда`: 9 words.
    Череда,
    /// Like `усадьба`: 9 words.
    Усадьба,
    /// Like `герц`: 8 words.
    Герц,
    /// Like `калашников`: 8 words.
    Калашников,
    /// Like `плечо`: 8 words.
    Плечо,
    /// Like `горе`: 8 words.
    Горе,
    /// Like `лапоть`: 7 words.
    Лапоть,
    /// Like `сиверко`: 7 words.
    Сиверко,
    /// Like `передняя`: 7 words.
    Передняя,
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
    /// Like `любовь`: 7 words.
    Любовь,
    /// Like `млекопитающее`: 7 words.
    Млекопитающее,
    /// Like `татарин`: 6 words.
    Татарин,
    /// Like `чечётка`: 6 words.
    Чечётка,
    /// Like `зеленя`: 6 words.
    Зеленя,
    /// Like `исайя`: 6 words.
    Исайя,
    /// Like `юноша`: 5 words.
    Юноша,
    /// Like `огонь`: 5 words.
    Огонь,
    /// Like `боец`: 5 words.
    Боец,
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
    /// Like `кухня`: 5 words.
    Кухня,
    /// Like `семя`: 4 words.
    Семя,
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
    /// Like `тамада`: 4 words.
    Тамада,
    /// Like `глаз`: 3 words.
    Глаз,
    /// Like `земля`: 3 words.
    Земля,
    /// Like `мать`: 3 words.
    Мать,
    /// Like `путь`: 3 words.
    Путь,
    /// Like `лёд`: 3 words.
    Лёд,
    /// Like `сколько`: 3 words.
    Сколько,
    /// Like `князь`: 3 words.
    Князь,
    /// Like `кто`: 3 words.
    Кто,
    /// Like `дрянь`: 3 words.
    Дрянь,
    /// Like `ничто`: 3 words.
    Ничто,
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
    /// Like `бий`: 3 words.
    Бий,
    /// Like `талдыченье`: 3 words.
    Талдыченье,
    /// Like `оладий`: 1 words.
    Оладий,
    /// Like `проводины`: 1 words.
    Проводины,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 181] = [
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
        Declension::Щиток,
        Declension::Товарищ,
        Declension::Змей,
        Declension::Гений,
        Declension::Барашек,
        Declension::Чайка,
        Declension::Накладная,
        Declension::Мещанин,
        Declension::Ложе,
        Declension::Зелёный,
        Declension::Ключ,
        Declension::Люлька,
        Declension::Баня,
        Declension::Аниец,
        Declension::Червь,
        Declension::Шоколадница,
        Declension::Домашние,
        Declension::Постав,
        Declension::Счёты,
        Declension::Русский,
        Declension::Батарея,
        Declension::Отец,
        Declension::Второе,
        Declension::Умелец,
        Declension::Молочко,
        Declension::Латы,
        Declension::Сватья,
        Declension::Мостки,
        Declension::Фигня,
        Declension::Орлёнок,
        Declension::Подворотня,
        Declension::Гривна,
        Declension::Ясли,
        Declension::Кормчий,
        Declension::Камень,
        Declension::Нападки,
        Declension::Гусёк,
        Declension::Выдача,
        Declension::Спальня,
        Declension::Достопримечательность,
        Declension::Штрафной,
        Declension::Кулички,
        Declension::Дитё,
        Declension::Абзац,
        Declension::Ядро,
        Declension::Внучонок,
        Declension::Бельё,
        Declension::Раз,
        Declension::Свеча,
        Declension::Недра,
        Declension::Каракуля,
        Declension::Котище,
        Declension::Делёж,
        Declension::Съезжая,
        Declension::Помочь,
        Declension::Дрязги,
        Declension::Горючее,
        Declension::Шов,
        Declension::Член,
        Declension::Кешью,
        Declension::Порубка,
        Declension::Ладья,
        Declension::Деревня,
        Declension::Верея,
        Declension::Бой,
        Declension::Егерь,
        Declension::Голец,
        Declension::Звезда,
        Declension::Божество,
        Declension::Ребро,
        Declension::Котёл,
        Declension::Устье,
        Declension::Деньги,
        Declension::Тельце,
        Declension::Маракуйя,
        Declension::Клин,
        Declension::Сенцы,
        Declension::Меринос,
        Declension::Боёк,
        Declension::Детище,
        Declension::Кокцинеллида,
        Declension::Колёсико,
        Declension::Анкара,
        Declension::Перо,
        Declension::День,
        Declension::Бойня,
        Declension::Побои,
        Declension::Зло,
        Declension::Дух,
        Declension::Губоцветные,
        Declension::Котёночек,
        Declension::Сердце,
        Declension::Гнездо,
        Declension::Племя,
        Declension::Плюхино,
        Declension::Фекалии,
        Declension::Кольцо,
        Declension::Рыльце,
        Declension::Мощи,
        Declension::Валенок,
        Declension::Счёт,
        Declension::Прения,
        Declension::Хлопья,
        Declension::Кукла,
        Declension::Перешеек,
        Declension::Череда,
        Declension::Усадьба,
        Declension::Герц,
        Declension::Калашников,
        Declension::Плечо,
        Declension::Горе,
        Declension::Лапоть,
        Declension::Сиверко,
        Declension::Передняя,
        Declension::Стекло,
        Declension::Заём,
        Declension::Весна,
        Declension::Осётр,
        Declension::Полгода,
        Declension::Любовь,
        Declension::Млекопитающее,
        Declension::Татарин,
        Declension::Чечётка,
        Declension::Зеленя,
        Declension::Исайя,
        Declension::Юноша,
        Declension::Огонь,
        Declension::Боец,
        Declension::Козлы,
        Declension::Репей,
        Declension::Хозяин,
        Declension::Бега,
        Declension::Ботиночек,
        Declension::Полкроны,
        Declension::Тиса,
        Declension::Кухня,
        Declension::Семя,
        Declension::Небо,
        Declension::Судно,
        Declension::Улей,
        Declension::Учащийся,
        Declension::Лишай,
        Declension::Капли,
        Declension::Пресмыкающееся,
        Declension::Ничья,
        Declension::Яйки,
        Declension::Сынуля,
        Declension::Карло,
        Declension::Дитятко,
        Declension::Тамада,
        Declension::Глаз,
        Declension::Земля,
        Declension::Мать,
        Declension::Путь,
        Declension::Лёд,
        Declension::Сколько,
        Declension::Князь,
        Declension::Кто,
        Declension::Дрянь,
        Declension::Ничто,
        Declension::Епитимья,
        Declension::Дверца,
        Declension::Солнышко,
        Declension::Получеловек,
        Declension::Морячка,
        Declension::Бий,
        Declension::Талдыченье,
        Declension::Оладий,
        Declension::Проводины,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 181] = [
    // Приём: 37,921 words (указ, зонт, кот, зонтик, бортник, бочаг).
    declension!("приём", 37921, "~", [gender: Masculine];
    stem pl: "~- like приём";
    stem gen_sg: "~- like приём";
    nom: "~", _;
    noun_second_case: derived, _;
    ),
    // Полнота: 12,531 words (собака, буява, былица, масленица, царица, вата). Like Приём, except in 2 cells: gen_ pl, noun_second_case loc.
    declension!("полнота", 12531, "~а", [gender: Feminine], like Приём;
    stem lemma: "~- like полнота";
    stem gen_sg: "~- like полнота";
    gen_: _, "~";
    noun_second_case: _, derived;
    ),
    // Скопление: 9,052 words (произношение, лезвие, распятие, деревенение, лесонасаждение, ведение).
    declension!("скопление", 9052, "~е", [gender: Neuter, animacy: Inanimate];
    stem lemma: "~е- like скопление";
    stem pl: "~- like скопление";
    stem gen_sg: "~- like скопление";
    ),
    // Склонность: 7,529 words (знать, весь, гласность, деревянность, дверь, ртуть).
    declension!("склонность", 7529, "~ь", [gender: Feminine, animacy: Inanimate];
    stem pl: "~- like склонность";
    stem gen_sg: "~- like склонность";
    nom: "~ь", _;
    ),
    // Стрелка: 6,666 words (водка, цыпка, жилка, деревянка, лесопилка, доска).
    declension!("стрелка", 6666, "~~а", [gender: Feminine];
    stem lemma: "~~- like полнота";
    stem gen_sg: "~~- like стрелка";
    nom: _, "~~и";
    gen_: _, "~о~";
    dat: _, "~~ам";
    ins: _, "~~ами";
    prep: _, "~~ах";
    ),
    // Химия: 6,581 words (анархия, лесохимия, библия, биология, академия, армия). Like Склонность, except in 3 cells: gen_ pl, ins sg, noun_second_case loc.
    declension!("химия", 6581, "~я", [gender: Feminine, animacy: Inanimate], like Склонность;
    stem lemma: "~- like химия";
    gen_: _, "~й";
    ins: "~ей|~ею", _;
    noun_second_case: _, derived;
    ),
    // Какао: 3,419 words (пальто, все, брачущиеся, брачующиеся, адажио, какао).
    declension!("какао", 3419, "~";
    stem gen_sg: "~- like какао";
    nom: "~", "~";
    gen_: _, "~";
    dat: _, "~";
    ins: _, "~";
    prep: _, "~";
    ),
    // Хлопанье: 2,660 words (запястье, воскресенье, варенье, воспитанье, отчаянье, уединенье).
    declension!("хлопанье", 2660, "~ье", [gender: Neuter, animacy: Inanimate];
    stem gen_sg: "~ь- like медведь";
    nom: "~ье", "~ья";
    gen_: _, "~ий";
    dat: _, "~ьям";
    ins: _, "~ьями";
    prep: _, "~ьях";
    noun_second_case: derived, _;
    ),
    // Дорожка: 2,519 words (кошка, быличка, палочка, ручка, мошка, бабочка). Like Стрелка, except in 1 cell: gen_ pl.
    declension!("дорожка", 2519, "~~а", like Стрелка;
    gen_: _, "~е~";
    ),
    // Сходство: 2,323 words (буево, бёрдо, око, лето, жало, горло). Like Ключ with `` after each pattern (`о` after ~), except in 2 cells: nom pl, gen_ pl.
    declension!("сходство", 2323, "~о", [gender: Neuter, animacy: Inanimate], like Ключ with "", "о" after "~";
    nom: _, "~а";
    gen_: _, "~";
    ),
    // Агнец: 2,159 words (левантинец, древоточец, марганец, самозванец, разночинец, ньюйоркец). Like Стрелка, except in 2 cells: nom sg, acc sg.
    declension!("агнец", 2159, "~е~", [gender: Masculine], like Стрелка;
    stem pl: "~~- like агнец";
    stem gen_sg: "~~- like агнец";
    nom: "~е~", _;
    acc: derived, _;
    ),
    // Медведь: 1,979 words (сшиватель, апрель, июнь, июль, олень, автомобиль). Like Скопление, except in 3 cells: nom sg, acc sg, noun_second_case part.
    declension!("медведь", 1979, "~ь", [gender: Masculine], like Скопление;
    stem pl: "~- like склонность";
    stem gen_sg: "~- like медведь";
    nom: "~ь", _;
    acc: derived, _;
    noun_second_case: derived, _;
    ),
    // Щиток: 1,057 words (щенок, бесёнок, лесок, замок, носок, подросток). Like Агнец, except in 4 cells: nom sg, nom pl, gen_ pl, noun_second_case part.
    declension!("щиток", 1057, "~о~", [gender: Masculine], like Агнец;
    stem gen_sg: "~~- like приём";
    nom: "~о~", "~~и";
    gen_: _, "~~ов";
    noun_second_case: derived, _;
    ),
    // Товарищ: 945 words (харч, туш, треш, раж, товарищ, дервиш). Like Приём, except in 1 cell: noun_second_case loc.
    declension!("товарищ", 945, "~", [gender: Masculine], like Приём;
    stem pl: "~- like товарищ";
    stem gen_sg: "~- like агнец";
    noun_second_case: _, derived;
    ),
    // Змей: 658 words (май, гей, вертухай, елей, герой, случай). Like Медведь, except in 2 cells: nom sg, gen_ pl.
    declension!("змей", 658, "~й", [gender: Masculine], like Медведь;
    nom: "~й", _;
    gen_: _, "~ев";
    ),
    // Гений: 649 words (калий, меркурий, гений, гравий, бериллий, гелий). Like Змей.
    declension!("гений", 649, "~й", [gender: Masculine], like Змей;
    stem gen_sg: "~- like скопление";
    ),
    // Барашек: 492 words (горошек, излишек, камешек, гривенничек, кусочек, орешек). Like Щиток, except in 2 cells: nom sg, noun_second_case loc.
    declension!("барашек", 492, "~е~", [gender: Masculine], like Щиток;
    nom: "~е~", _;
    noun_second_case: _, derived;
    ),
    // Чайка: 474 words (копейка, скамейка, перестройка, ищейка, лайка, австрийка).
    declension!("чайка", 474, "~й~а", [gender: Feminine];
    stem lemma: "~й~- like полнота";
    stem gen_sg: "~й~- like стрелка";
    nom: _, "~й~и";
    gen_: _, "~е~";
    dat: _, "~й~ам";
    ins: _, "~й~ами";
    prep: _, "~й~ах";
    ),
    // Накладная: 415 words (деревообделочная, детская, гостиная, ванная, кладовая, уборная).
    declension!("накладная", 415, "~ая", [gender: Feminine];
    stem pl: "~ы- like накладная";
    stem gen_sg: "~о- like накладная";
    nom: "~ая", _;
    acc: "~ую", _;
    ),
    // Мещанин: 396 words (господин, англичанин, россиянин, опочанин, гражданин, новоангличанин). Like Сходство, except in 2 cells: nom sg, nom pl.
    declension!("мещанин", 396, "~ин", [gender: Masculine, animacy: Animate], like Сходство;
    stem gen_sg: "~ин- like сходство";
    nom: "~ин", "~е";
    ),
    // Ложе: 327 words (солнце, вече, училище, кладбище, ложе, пристанище). Like Товарищ with `` after each pattern (`е` after ~), except in 2 cells: nom pl, gen_ pl.
    declension!("ложе", 327, "~е", [animacy: Inanimate], like Товарищ with "", "е" after "~";
    nom: _, "~а";
    gen_: _, "~";
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
    // Ключ: 277 words (борщ, карандаш, врач, сыч, нож, меч). Like Товарищ.
    declension!("ключ", 277, "~", [gender: Masculine], like Товарищ;
    stem gen_sg: "~- like сходство";
    ),
    // Люлька: 263 words (пенька, пиписька, кухонька, батенька, калька, свистулька).
    declension!("люлька", 263, "~ь~а";
    stem lemma: "~ь~- like полнота";
    stem gen_sg: "~ь~- like стрелка";
    nom: _, "~ь~и";
    gen_: _, "~е~";
    dat: _, "~ь~ам";
    ins: _, "~ь~ами";
    prep: _, "~ь~ах";
    ),
    // Баня: 253 words (братыня, неделя, буря, няня, княгиня, тётя). Like Химия, except in 1 cell: gen_ pl.
    declension!("баня", 253, "~я", like Химия;
    stem gen_sg: "~- like баня";
    gen_: _, "~ь";
    ),
    // Аниец: 250 words (австриец, австралиец, ариец, белогвардеец, китаец, византиец). Like Чайка, except in 2 cells: nom sg, acc sg.
    declension!("аниец", 250, "~е~", [gender: Masculine, animacy: Animate], like Чайка;
    stem pl: "~й~- like агнец";
    stem gen_sg: "~й~- like агнец";
    nom: "~е~", _;
    acc: derived, _;
    ),
    // Червь: 232 words (царь, рубль, кремль, дождь, январь, февраль). Like Медведь, except in 1 cell: noun_second_case part.
    declension!("червь", 232, "~ь", [gender: Masculine], like Медведь;
    stem gen_sg: "~- like червь";
    noun_second_case: derived, _;
    ),
    // Шоколадница: 218 words (пицца, кожа, мульча, грыжа, мрежа, туча). Like Полнота, except in 1 cell: ins sg.
    declension!("шоколадница", 218, "~а", like Полнота;
    ins: "~ей|~ею", _;
    ),
    // Домашние: 215 words (немногие, сколькие, данные, чаевые, присные, месячные).
    declension!("домашние", 215, "~е";
    stem pl: "~- like накладная";
    nom: "~е", _;
    ),
    // Постав: 206 words (дом, лес, город, пояс, адрес, бок). Like Приём, except in 1 cell: nom pl.
    declension!("постав", 206, "~", [gender: Masculine], like Приём;
    nom: _, "~а";
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
    // Русский: 200 words (русский, сербскохорватский, английский, рабочий, норвежский, китайский). Like Зелёный, except in 2 cells: nom sg, ins sg.
    declension!("русский", 200, "~ий", [gender: Masculine], like Зелёный;
    stem pl: "~и- like накладная";
    nom: "~ий", _;
    ins: "~им", _;
    ),
    // Батарея: 195 words (статуя, стая, шея, идея, затея, фея). Like Баня, except in 1 cell: gen_ pl.
    declension!("батарея", 195, "~я", [gender: Feminine], like Баня;
    gen_: _, "~й";
    ),
    // Отец: 179 words (отец, свинец, ветер, купец, творец, дворец). Like Барашек.
    declension!("отец", 179, "~е~", [gender: Masculine], like Барашек;
    stem pl: "~~- like приём";
    ),
    // Второе: 179 words (былое, насекомое, существительное, числительное, многое, животное). Like Зелёный.
    declension!("второе", 179, "~ое", [gender: Neuter], like Зелёный;
    stem lemma: "~ое- like скопление";
    ),
    // Умелец: 174 words (палец, комсомолец, земледелец, бразилец, владелец, анголец). Like Люлька, except in 2 cells: nom sg, acc sg.
    declension!("умелец", 174, "~е~", [gender: Masculine], like Люлька;
    stem pl: "~ь~- like агнец";
    stem gen_sg: "~ь~- like агнец";
    nom: "~е~", _;
    acc: derived, _;
    ),
    // Молочко: 153 words (семечко, колено, батько, яичко, тепло, окошко). Like Барашек, except in 2 cells: nom sg, gen_ pl.
    declension!("молочко", 153, "~~о", [animacy: Inanimate], like Барашек;
    nom: "~~о", _;
    gen_: _, "~е~";
    ),
    // Латы: 150 words (цимбалы, каникулы, ножницы, шахматы, именины, бакенбарды). Like Счёты, except in 1 cell: gen_ pl.
    declension!("латы", 150, "~ы", [gender: Feminine, animacy: Inanimate], like Счёты;
    gen_: _, "~";
    ),
    // Сватья: 149 words (гостья, авиаэскадрилья, эскадрилья, келья, тортилья, колдунья). Like Хлопанье, except in 1 cell: nom pl.
    declension!("сватья", 149, "~ья", [gender: Feminine], like Хлопанье;
    stem lemma: "~ь- like химия";
    stem gen_sg: "~ь- like баня";
    nom: _, "~ьи";
    ),
    // Мостки: 144 words (древпластики, духи, щипчики, мозги, падонки, очки). Like Товарищ, except in 2 cells: nom sg, gen_ pl.
    declension!("мостки", 144, "~и", [gender: Masculine, animacy: Inanimate], like Товарищ;
    stem gen_sg: "~и- like какао";
    nom: "~и", _;
    gen_: _, "~ов";
    ),
    // Фигня: 128 words (ступня, стезя, простыня, тля, хуйня, броня). Like Баня.
    declension!("фигня", 128, "~я", [gender: Feminine], like Баня;
    stem pl: "~- like склонность";
    stem gen_sg: "~- like заря";
    ),
    // Орлёнок: 125 words (телёнок, октябрёнок, поросёнок, жеребёнок, ягнёнок, козлёнок).
    declension!("орлёнок", 125, "~ёнок", [gender: Masculine, animacy: Animate];
    stem pl: "~ят- like сходство";
    stem gen_sg: "~ёнк- like сходство";
    nom: "~ёнок", _;
    ),
    // Подворотня: 120 words (вишня, песня, бушня, пригоршня, башня, пекарня). Like Дорожка, except in 3 cells: dat pl, ins pl, prep pl.
    declension!("подворотня", 120, "~~я", [gender: Feminine, animacy: Inanimate], like Дорожка;
    stem lemma: "~~- like химия";
    stem gen_sg: "~~- like баня";
    dat: _, "~~ям";
    ins: _, "~~ями";
    prep: _, "~~ях";
    ),
    // Гривна: 114 words (овца, сосна, домна, царевна, гривна, кошма). Like Дорожка, except in 1 cell: nom pl.
    declension!("гривна", 114, "~~а", [gender: Feminine], like Дорожка;
    stem gen_sg: "~~- like полнота";
    nom: _, "~~ы";
    ),
    // Ясли: 113 words (люди, кости, черви, внутренности, вожжи, ясли). Like Медведь, except in 5 cells: nom sg, gen_ sg, dat sg, ins sg, prep sg.
    declension!("ясли", 113, "~и", [animacy: Inanimate], like Медведь;
    nom: "~и", _;
    gen_: derived, _;
    dat: derived, _;
    ins: derived, _;
    prep: derived, _;
    ),
    // Кормчий: 107 words (лесничий, посторонний, нищий, военнослужащий, домашний, желающий). Like Русский.
    declension!("кормчий", 107, "~ий", [gender: Masculine, animacy: Animate], like Русский;
    stem gen_sg: "~- like кормчий";
    ),
    // Камень: 107 words (бредень, оборотень, камень, парень, полдень, кашель). Like Подворотня, except in 3 cells: nom sg, acc sg, noun_second_case part.
    declension!("камень", 107, "~е~ь", [gender: Masculine], like Подворотня;
    stem pl: "~~- like склонность";
    stem gen_sg: "~~- like медведь";
    nom: "~е~ь", _;
    acc: derived, _;
    noun_second_case: derived, _;
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
    // Гусёк: 89 words (мотылёк, хорёк, кошелёк, королёк, огонёк, уголёк). Like Умелец, except in 3 cells: nom sg, nom pl, gen_ pl.
    declension!("гусёк", 89, "~ё~", [gender: Masculine], like Умелец;
    stem gen_sg: "~ь~- like сходство";
    nom: "~ё~", "~ь~и";
    gen_: _, "~ь~ов";
    ),
    // Выдача: 85 words (тёща, пища, вельможа, гуща, роща, толща). Like Шоколадница, except in 1 cell: nom pl.
    declension!("выдача", 85, "~а", like Шоколадница;
    stem gen_sg: "~- like баня";
    nom: _, "~и";
    ),
    // Спальня: 85 words (лесопильня, спальня, читальня, колокольня, коптильня, наковальня). Like Люлька, except in 3 cells: dat pl, ins pl, prep pl.
    declension!("спальня", 85, "~ь~я", [gender: Feminine, animacy: Inanimate], like Люлька;
    stem lemma: "~ь~- like химия";
    stem gen_sg: "~ь~- like баня";
    dat: _, "~ь~ям";
    ins: _, "~ь~ями";
    prep: _, "~ь~ях";
    ),
    // Достопримечательность: 81 words (лошадь, сельдь, мышь, свекровь, достопримечательность, моль). Like Склонность, except in 2 cells: acc sg, noun_second_case loc.
    declension!("достопримечательность", 81, "~ь", [gender: Feminine, animacy: Animate], like Склонность;
    acc: "~ь", _;
    noun_second_case: _, derived;
    ),
    // Штрафной: 79 words (больной, голубой, часовой, золотой, вестовой, немой). Like Зелёный, except in 1 cell: nom sg.
    declension!("штрафной", 79, "~ой", [gender: Masculine], like Зелёный;
    nom: "~ой", _;
    ),
    // Кулички: 75 words (сумерки, бачки, шашки, кусачки, детишки, корточки). Like Нападки, except in 1 cell: gen_ pl.
    declension!("кулички", 75, "~~и", like Нападки;
    gen_: _, "~е~";
    ),
    // Дитё: 74 words (мумиё, враньё, жильё, остриё, бытиё, чутьё). Like Червь, except in 1 cell: nom pl.
    declension!("дитё", 74, "~ё", [gender: Neuter], like Червь;
    stem lemma: "~ё- like скопление";
    nom: _, "~я";
    ),
    // Абзац: 63 words (месяц, принц, абзац, кварц, плац, полумесяц). Like Приём, except in 1 cell: ins sg.
    declension!("абзац", 63, "~", [gender: Masculine], like Приём;
    stem pl: "~- like агнец";
    ins: "~ем", _;
    ),
    // Ядро: 59 words (брашно, говно, деревцо, ядро, масло, вёдро). Like Молочко, except in 1 cell: nom pl.
    declension!("ядро", 59, "~~о", [gender: Neuter, animacy: Inanimate], like Молочко;
    nom: _, "~~а";
    ),
    // Внучонок: 57 words (медвежонок, индюшонок, крольчонок, внучонок, волчонок, мышонок).
    declension!("внучонок", 57, "~онок", [gender: Masculine, animacy: Animate];
    stem pl: "~ат- like сходство";
    stem gen_sg: "~онк- like сходство";
    nom: "~онок", _;
    ),
    // Бельё: 56 words (копьё, питьё, ружьё, битьё, сырьё, бельё). Like Хлопанье, except in 2 cells: nom sg, noun_second_case part.
    declension!("бельё", 56, "~ьё", [gender: Neuter, animacy: Inanimate], like Хлопанье;
    stem pl: "~- like жнивьё";
    stem gen_sg: "~ь- like червь";
    nom: "~ьё", _;
    noun_second_case: derived, _;
    ),
    // Раз: 54 words (солдат, цыган, раз, гусар, вольт, погон). Like Приём, except in 2 cells: gen_ pl, noun_second_case loc.
    declension!("раз", 54, "~", [gender: Masculine], like Приём;
    gen_: _, "~";
    noun_second_case: _, derived;
    ),
    // Свеча: 54 words (левша, ханжа, вожжа, свеча, юноша, саранча). Like Выдача.
    declension!("свеча", 54, "~а", like Выдача;
    stem pl: "~- like товарищ";
    stem gen_sg: "~- like стрелка";
    ),
    // Недра: 52 words (ребята, врата, дрова, уста, внучата, правнучата). Like Латы, except in 1 cell: nom sg.
    declension!("недра", 52, "~а", like Латы;
    stem pl: "~- like сходство";
    nom: "~а", _;
    ),
    // Каракуля: 45 words (постеля, батя, дядя, доля, ходуля, сводня). Like Баня.
    declension!("каракуля", 45, "~я", like Баня;
    stem pl: "~- like склонность";
    ),
    // Котище: 40 words (дружище, когтище, волчище, хуище, козлище, третье). Like Товарищ with `` after each pattern (`е` after ~), except in 1 cell: gen_ pl.
    declension!("котище", 40, "~е", [gender: Masculine], like Товарищ with "", "е" after "~";
    gen_: _, "~";
    ),
    // Делёж: 38 words (падёж, пиздёж, чертёж, грабёж, платёж, мухлёж).
    declension!("делёж", 38, "~ё~", [gender: Masculine, animacy: Inanimate];
    stem pl: "~е~- like товарищ";
    stem gen_sg: "~е~- like сходство";
    nom: "~ё~", _;
    ),
    // Съезжая: 37 words (прихожая, телеведущая, неверующая, купчая, составляющая, заведующая). Like Русский, except in 2 cells: nom sg, acc sg.
    declension!("съезжая", 37, "~ая", [gender: Feminine], like Русский;
    stem gen_sg: "~е- like накладная";
    nom: "~ая", _;
    acc: "~ую", _;
    ),
    // Помочь: 37 words (тушь, нощь, пещь, помощь, немощь, мощь). Like Склонность.
    declension!("помочь", 37, "~ь", [gender: Feminine, animacy: Inanimate], like Склонность;
    stem pl: "~- like товарищ";
    ),
    // Дрязги: 33 words (брюки, ладоши, лыжи, вериги, пики, брызги). Like Выдача, except in 2 cells: nom sg, ins sg.
    declension!("дрязги", 33, "~и", [animacy: Inanimate], like Выдача;
    nom: "~и", _;
    ins: "~ой", _;
    ),
    // Горючее: 32 words (настоящее, будущее, горючее, грядущее, прочее, болеутоляющее). Like Кормчий, except in 1 cell: nom sg.
    declension!("горючее", 32, "~ее", [gender: Neuter, animacy: Inanimate], like Кормчий;
    nom: "~ее", _;
    ),
    // Шов: 30 words (рот, свёкор, лоб, сон, хохол, угол). Like Щиток.
    declension!("шов", 30, "~о~", [gender: Masculine], like Щиток;
    stem pl: "~~- like приём";
    ),
    // Член: 30 words (клон, тип, член, кат, кекс, бас). Like Приём, except in 2 cells: acc sg, noun_second_case loc.
    declension!("член", 30, "~", [gender: Masculine, animacy: Inanimate], like Приём;
    stem pl: "~- like член";
    acc: "~а|~", _;
    noun_second_case: _, derived;
    ),
    // Кешью: 30 words (дитя, порося, кешью, чапаев, гэта, мюсли). Like Какао, except in 2 cells: acc sg, acc pl.
    declension!("кешью", 30, "~", like Какао;
    acc: "~", "~";
    ),
    // Порубка: 30 words (скоблянка, меньшевичка, связистка, отклейка, порубка, самосброска). Like Свеча, except in 1 cell: gen_ pl.
    declension!("порубка", 30, "~а", [gender: Feminine], like Свеча;
    gen_: _, "~а";
    ),
    // Ладья: 29 words (семья, ладья, статья, судья, скамья, голья). Like Сватья, except in 1 cell: gen_ pl.
    declension!("ладья", 29, "~ья", like Сватья;
    stem gen_sg: "~ь- like заря";
    gen_: _, "~ей";
    ),
    // Деревня: 29 words (торговля, сабля, капля, туфля, чапля, цапля). Like Подворотня, except in 1 cell: gen_ pl.
    declension!("деревня", 29, "~~я", [gender: Feminine], like Подворотня;
    gen_: _, "~е~ь";
    ),
    // Верея: 29 words (змея, колея, чешуя, швея, струя, алия). Like Батарея.
    declension!("верея", 29, "~я", [gender: Feminine], like Батарея;
    stem gen_sg: "~- like заря";
    ),
    // Бой: 28 words (чай, хуй, бой, рой, слой, край). Like Змей, except in 1 cell: gen_ pl.
    declension!("бой", 28, "~й", [gender: Masculine], like Змей;
    gen_: _, "~ёв";
    ),
    // Егерь: 28 words (учитель, якорь, флигель, лагерь, тополь, соболь). Like Медведь, except in 1 cell: nom pl.
    declension!("егерь", 28, "~ь", [gender: Masculine], like Медведь;
    nom: _, "~я";
    ),
    // Голец: 24 words (лев, телец, стрелец, жилец, делец, малец). Like Умелец.
    declension!("голец", 24, "~е~", [gender: Masculine], like Умелец;
    stem pl: "~ь~- like приём";
    stem gen_sg: "~ь~- like сходство";
    ),
    // Звезда: 24 words (звезда, пчела, жена, сестра, медсестра, береста).
    declension!("звезда", 24, "~е~а", [gender: Feminine];
    stem lemma: "~е~- like полнота";
    stem pl: "~ё~- like полнота";
    stem gen_sg: "~е~- like полнота";
    ),
    // Божество: 23 words (существо, лицо, божество, чадо, чмо, падло). Like Глаз with `` after each pattern (`о` after ~), except in 2 cells: gen_ pl, acc sg.
    declension!("божество", 23, "~о", [gender: Neuter, animacy: Animate], like Глаз with "", "о" after "~";
    gen_: _, "~";
    acc: "~о", _;
    ),
    // Ребро: 23 words (сребро, серебро, ведро, ребро, весло, зерно).
    declension!("ребро", 23, "~е~~о", [gender: Neuter, animacy: Inanimate];
    stem gen_sg: "~е~~- like сходство";
    nom: "~е~~о", "~ё~~а";
    gen_: _, "~ё~е~";
    dat: _, "~ё~~ам";
    ins: _, "~ё~~ами";
    prep: _, "~ё~~ах";
    ),
    // Котёл: 22 words (пёс, козёл, орёл, овёс, осёл, костёр). Like Отец, except in 1 cell: nom sg.
    declension!("котёл", 22, "~ё~", [gender: Masculine], like Отец;
    nom: "~ё~", _;
    ),
    // Устье: 21 words (платье, устье, подмастерье, подполье, уголье, отребье). Like Змей, except in 1 cell: nom sg.
    declension!("устье", 21, "~е", [gender: Neuter, animacy: Inanimate], like Змей;
    stem pl: "~- like устье";
    nom: "~е", _;
    ),
    // Деньги: 21 words (деньги, четвереньки, грабельки, хахоньки, ахоньки, ясельки). Like Кулички, except in 5 cells: nom sg, nom pl, dat pl, ins pl, prep pl.
    declension!("деньги", 21, "~ь~и", [gender: Feminine, animacy: Inanimate], like Кулички;
    nom: "~ь~и", "~ь~и";
    dat: _, "~ь~ам";
    ins: _, "~ь~ами";
    prep: _, "~ь~ах";
    ),
    // Тельце: 21 words (болотце, тельце, коленце, ведёрце, весельце, дульце). Like Агнец, except in 2 cells: nom sg, nom pl.
    declension!("тельце", 21, "~~е", [gender: Neuter, animacy: Inanimate], like Агнец;
    nom: "~~е", "~~а";
    ),
    // Маракуйя: 21 words (аллилуйя, секвойя, паранойя, папайя, пирайя, майя). Like Баня, except in 1 cell: gen_ pl.
    declension!("маракуйя", 21, "~я", [gender: Feminine], like Баня;
    gen_: _, "~";
    ),
    // Клин: 20 words (брат, лист, муж, клин, стул, струп). Like Приём.
    declension!("клин", 20, "~", [gender: Masculine], like Приём;
    stem pl: "~ь- like устье";
    ),
    // Сенцы: 19 words (японцы, плоскогубцы, плоскозубцы, шлёпанцы, сланцы, святцы). Like Счёты.
    declension!("сенцы", 19, "~ы", [gender: Masculine], like Счёты;
    stem pl: "~- like агнец";
    ),
    // Меринос: 18 words (квакер, чернорубашечник, виолончелист, солитер, кастрат, бахмат). Like Член, except in 1 cell: acc sg.
    declension!("меринос", 18, "~", [gender: Masculine], like Член;
    stem pl: "~- like меринос";
    acc: "~", _;
    ),
    // Боёк: 18 words (паёк, чаёк, ручеёк, кофеёк, боёк, буёк). Like Аниец, except in 3 cells: nom sg, nom pl, gen_ pl.
    declension!("боёк", 18, "~ё~", [gender: Masculine], like Аниец;
    stem gen_sg: "~й~- like сходство";
    nom: "~ё~", "~й~и";
    gen_: _, "~й~ов";
    ),
    // Детище: 17 words (чудовище, чудище, детище, посмешище, уёбище, страшилище). Like Товарищ with `` after each pattern (`е` after ~), except in 3 cells: nom pl, gen_ pl, acc sg.
    declension!("детище", 17, "~е", [animacy: Animate], like Товарищ with "", "е" after "~";
    nom: _, "~а";
    gen_: _, "~";
    acc: "~е", _;
    ),
    // Кокцинеллида: 16 words (ебала, венецианка, ламиина, иссида, кокцинеллида, алеохарина). Like Полнота.
    declension!("кокцинеллида", 16, "~а", [animacy: Animate], like Полнота;
    stem pl: "~- like кокцинеллида";
    ),
    // Колёсико: 16 words (колёсико, древко, очко, брюшко, личико, пивко). Like Ключ with `` after each pattern (`о` after ~), except in 1 cell: gen_ pl.
    declension!("колёсико", 16, "~о", like Ключ with "", "о" after "~";
    gen_: _, "~ов";
    ),
    // Анкара: 16 words (ковурма, дисна, полумгла, мста, ангара, анкара). Like Полнота, except in 1 cell: gen_ pl.
    declension!("анкара", 16, "~а", [gender: Feminine], like Полнота;
    gen_: _, "~а";
    ),
    // Перо: 16 words (дерево, перо, крыло, сено, авиазвено, звено). Like Клин with `` after each pattern (`о` after ~), except in 1 cell: noun_second_case loc.
    declension!("перо", 16, "~о", [gender: Neuter, animacy: Inanimate], like Клин with "", "о" after "~";
    noun_second_case: _, derived;
    ),
    // День: 15 words (день, ремень, пень, кремень, плетень, трудодень). Like Камень, except in 2 cells: ins sg, noun_second_case loc.
    declension!("день", 15, "~е~ь", [gender: Masculine], like Камень;
    ins: "~~ём", _;
    noun_second_case: _, "~~ю";
    ),
    // Бойня: 15 words (кофейня, бойня, водогрейня, воскобойня, двойня, маслобойня). Like Чайка, except in 3 cells: dat pl, ins pl, prep pl.
    declension!("бойня", 15, "~й~я", [gender: Feminine, animacy: Inanimate], like Чайка;
    stem lemma: "~й~- like химия";
    stem gen_sg: "~й~- like баня";
    dat: _, "~й~ям";
    ins: _, "~й~ями";
    prep: _, "~й~ях";
    ),
    // Побои: 15 words (обои, побои, помои, прелиминарии, покои, фотообои). Like Ясли, except in 1 cell: gen_ pl.
    declension!("побои", 15, "~и", [gender: Masculine], like Ясли;
    gen_: _, "~ев";
    ),
    // Зло: 15 words (окно, зло, сукно, волокно, жалко, тягло). Like Ядро, except in 1 cell: gen_ pl.
    declension!("зло", 15, "~~о", [gender: Neuter, animacy: Inanimate], like Ядро;
    gen_: _, "~о~";
    ),
    // Дух: 14 words (северо-восток, юго-восток, лопух, пыжовник, гулявник, ужовник). Like Приём, except in 1 cell: nom pl.
    declension!("дух", 14, "~", [gender: Masculine], like Приём;
    nom: _, "~и";
    ),
    // Губоцветные: 14 words (вересковые, водяниковые, верескоцветные, буковые, букоцветные, мальпигиевые).
    declension!("губоцветные", 14, "~е", [gender: Feminine, animacy: Inanimate];
    nom: "~е", _;
    gen_: "~х", _;
    dat: "~м", _;
    ins: "~ми", _;
    prep: "~х", _;
    ),
    // Котёночек: 14 words (цыплёночек, котёночек, жеребёночек, козлёночек, утёночек, ягнёночек).
    declension!("котёночек", 14, "~ёноче~", [gender: Masculine];
    stem gen_sg: "~ёноч~- like сходство";
    nom: "~ёноче~", "~ят~и";
    gen_: _, "~ято~";
    dat: _, "~ят~ам";
    ins: _, "~ят~ами";
    prep: _, "~ят~ах";
    ),
    // Сердце: 13 words (сердце, деревце, полотенце, блюдце, оконце, волоконце). Like Тельце, except in 1 cell: gen_ pl.
    declension!("сердце", 13, "~~е", [gender: Neuter, animacy: Inanimate], like Тельце;
    gen_: _, "~е~";
    ),
    // Гнездо: 13 words (озеро, колесо, село, чело, решето, гнездо). Like Делёж, except in 1 cell: nom sg.
    declension!("гнездо", 13, "~е~о", [gender: Neuter, animacy: Inanimate], like Делёж;
    stem pl: "~ё~- like сходство";
    nom: "~е~о", _;
    ),
    // Племя: 13 words (имя, время, знамя, бремя, вымя, пламя).
    declension!("племя", 13, "~я", [gender: Neuter, animacy: Inanimate];
    nom: "~я", "~ена";
    gen_: "~ени", "~ён";
    dat: "~ени", "~енам";
    ins: "~енем", "~енами";
    prep: "~ени", "~енах";
    ),
    // Плюхино: 13 words (мотовство, плюхино, плюхново, шереметьево, карабаново, балабаново).
    declension!("плюхино", 13, "~о", [gender: Neuter, animacy: Inanimate];
    nom: "~о", "~а|~о";
    gen_: "~а|~о", "~о";
    dat: "~у|~о", "~ам|~о";
    ins: "~ом|~о", "~ами|~о";
    prep: "~е|~о", "~ах|~о";
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
    // Рыльце: 12 words (зеркальце, щупальце, дельце, рыльце, сальце, мыльце). Like Кольцо, except in 1 cell: nom sg.
    declension!("рыльце", 12, "~ь~е", [gender: Neuter, animacy: Inanimate], like Кольцо;
    stem gen_sg: "~ь~- like агнец";
    nom: "~ь~е", _;
    ),
    // Мощи: 11 words (щи, клещи, мощи, беруши, марши, кириши). Like Счёты, except in 1 cell: nom sg.
    declension!("мощи", 11, "~и", like Счёты;
    stem pl: "~- like товарищ";
    nom: "~и", _;
    ),
    // Валенок: 11 words (ботинок, чулок, турок, валенок, глазок, зубок). Like Щиток, except in 2 cells: gen_ pl, noun_second_case loc.
    declension!("валенок", 11, "~о~", [gender: Masculine], like Щиток;
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
    // Прения: 11 words (прения, условия, переживания, усилия, воспоминания, взаимоотношения). Like Фекалии, except in 1 cell: nom sg.
    declension!("прения", 11, "~я", [gender: Neuter, animacy: Inanimate], like Фекалии;
    stem pl: "~- like скопление";
    nom: "~я", _;
    ),
    // Хлопья: 10 words (хлопья, лохмотья, объедья, брусья, коренья, стручья). Like Прения.
    declension!("хлопья", 10, "~я", [animacy: Inanimate], like Прения;
    stem pl: "~- like устье";
    ),
    // Кукла: 9 words (мгла, кукла, мошна, княжна, свёкла, свекла). Like Гривна, except in 1 cell: gen_ pl.
    declension!("кукла", 9, "~~а", like Гривна;
    gen_: _, "~о~";
    ),
    // Перешеек: 9 words (перешеек, закраек, зашеек, опоек, отроек, ошеек). Like Боёк, except in 1 cell: nom sg.
    declension!("перешеек", 9, "~е~", [gender: Masculine], like Боёк;
    nom: "~е~", _;
    ),
    // Череда: 9 words (щека, середа, железа, слеза, пелена, череда). Like Звезда, except in 4 cells: nom pl, dat pl, ins pl, prep pl.
    declension!("череда", 9, "~е~а", [gender: Feminine], like Звезда;
    nom: _, "~е~ы";
    dat: _, "~е~ам";
    ins: _, "~е~ами";
    prep: _, "~е~ах";
    ),
    // Усадьба: 9 words (тьма, сурьма, тюрьма, полутьма, пыльца, сольца). Like Люлька, except in 1 cell: nom pl.
    declension!("усадьба", 9, "~ь~а", [gender: Feminine, animacy: Inanimate], like Люлька;
    stem gen_sg: "~ь~- like полнота";
    nom: _, "~ь~ы";
    ),
    // Герц: 8 words (герц, гигагерц, килогерц, мегагерц, миллигерц, кольбац). Like Раз.
    declension!("герц", 8, "~", [gender: Masculine, animacy: Inanimate], like Раз;
    stem gen_sg: "~- like агнец";
    ),
    // Калашников: 8 words (калашников, басков, топтыгин, обещалкин, сердечкин, собакин). Like Зелёный, except in 5 cells: nom sg, nom pl, gen_ sg, dat sg, prep sg.
    declension!("калашников", 8, "~", [gender: Masculine], like Зелёный;
    nom: "~", "~ы";
    gen_: "~а", _;
    dat: "~у", _;
    prep: "~е", _;
    ),
    // Плечо: 8 words (яблоко, молоко, млеко, веко, плечо, брюхо). Like Ключ with `` after each pattern (`о` after ~), except in 1 cell: gen_ pl.
    declension!("плечо", 8, "~о", [animacy: Inanimate], like Ключ with "", "о" after "~";
    gen_: _, "~";
    ),
    // Горе: 8 words (море, горе, поле, перекати-поле, муде, биополе). Like Егерь, except in 1 cell: nom sg.
    declension!("горе", 8, "~е", [gender: Neuter, animacy: Inanimate], like Егерь;
    nom: "~е", _;
    ),
    // Лапоть: 7 words (локоть, дёготь, ноготь, уголь, коготь, лапоть). Like Камень, except in 1 cell: nom sg.
    declension!("лапоть", 7, "~о~ь", [gender: Masculine, animacy: Inanimate], like Камень;
    nom: "~о~ь", _;
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
    // Стекло: 7 words (стекло, бронестекло, стегно, пекло, наностекло, оргстекло). Like Ребро, except in 1 cell: gen_ pl.
    declension!("стекло", 7, "~е~~о", [gender: Neuter, animacy: Inanimate], like Ребро;
    gen_: _, "~ё~о~";
    ),
    // Заём: 7 words (заём, наём, поднаём, перенаём, госзаём, сберзаём). Like Боёк.
    declension!("заём", 7, "~ё~", [gender: Masculine, animacy: Inanimate], like Боёк;
    stem pl: "~й~- like приём";
    ),
    // Весна: 7 words (десна, весна, метла, ветла, гидрометла, блесна). Like Ребро, except in 1 cell: nom pl.
    declension!("весна", 7, "~е~~а", [gender: Feminine, animacy: Inanimate], like Ребро;
    stem lemma: "~е~~- like полнота";
    stem gen_sg: "~е~~- like полнота";
    nom: _, "~ё~~ы";
    ),
    // Осётр: 7 words (ксёндз, чёлн, осётр, черёд, перёд, клёст). Like Делёж.
    declension!("осётр", 7, "~ё~", [gender: Masculine], like Делёж;
    stem pl: "~е~- like приём";
    ),
    // Полгода: 7 words (полчаса, полгода, полметра, полмиллиона, полкилометра, поллитра).
    declension!("полгода", 7, "~~а", [animacy: Inanimate];
    stem pl: "~у~- like приём";
    stem gen_sg: "~у~- like сходство";
    nom: "~~а", _;
    ),
    // Любовь: 7 words (церковь, рожь, вошь, ложь, любовь, нелюбовь). Like Лапоть, except in 4 cells: gen_ sg, dat sg, ins sg, prep sg.
    declension!("любовь", 7, "~о~ь", [gender: Feminine], like Лапоть;
    gen_: "~~и", _;
    dat: "~~и", _;
    ins: "~о~ью", _;
    prep: "~~и", _;
    ),
    // Млекопитающее: 7 words (млекопитающее, окружающее, обезболивающее, дальнейшее, иглокожее, кишечнодышащее). Like Горючее, except in 1 cell: acc sg.
    declension!("млекопитающее", 7, "~ее", [gender: Neuter], like Горючее;
    acc: "~ее", _;
    ),
    // Татарин: 6 words (болгарин, татарин, хазарин, угрин, долганин, хариджанин). Like Мещанин.
    declension!("татарин", 6, "~ин", [gender: Masculine, animacy: Animate], like Мещанин;
    stem pl: "~- like полнота";
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
    // Юноша: 5 words (чукча, раджа, обжа, корча, магараджа). Like Выдача.
    declension!("юноша", 5, "~а", like Выдача;
    stem pl: "~- like товарищ";
    ),
    // Огонь: 5 words (огонь, угорь, ломоть, артогонь, стоп-огонь). Like Лапоть, except in 1 cell: noun_second_case part.
    declension!("огонь", 5, "~о~ь", [gender: Masculine], like Лапоть;
    stem gen_sg: "~~- like червь";
    noun_second_case: derived, _;
    ),
    // Боец: 5 words (боец, хуец, политбоец, гаец, экс-боец). Like Перешеек.
    declension!("боец", 5, "~е~", [gender: Masculine], like Перешеек;
    stem pl: "~й~- like приём";
    ),
    // Козлы: 5 words (ножны, бубны, козлы, люберцы, ливны). Like Кулички, except in 2 cells: nom sg, nom pl.
    declension!("козлы", 5, "~~ы", [gender: Feminine, animacy: Inanimate], like Кулички;
    nom: "~~ы", "~~ы";
    ),
    // Репей: 5 words (соловей, воробей, муравей, ручей, репей). Like Бельё, except in 3 cells: nom sg, nom pl, gen_ pl.
    declension!("репей", 5, "~ей", [gender: Masculine], like Бельё;
    nom: "~ей", "~ьи";
    gen_: _, "~ьёв";
    ),
    // Хозяин: 5 words (хозяин, домохозяин, квартирохозяин, сохозяин, судохозяин). Like Мещанин.
    declension!("хозяин", 5, "~ин", [gender: Masculine, animacy: Animate], like Мещанин;
    stem pl: "~ев- like сходство";
    ),
    // Бега: 5 words (бега, торока, потроха, холода, севера). Like Недра, except in 1 cell: gen_ pl.
    declension!("бега", 5, "~а", [gender: Masculine, animacy: Inanimate], like Недра;
    gen_: _, "~ов";
    ),
    // Ботиночек: 5 words (ботиночек, затылочек, катерочек, валеночек, чулочек). Like Барашек, except in 1 cell: gen_ pl.
    declension!("ботиночек", 5, "~е~", [gender: Masculine, animacy: Inanimate], like Барашек;
    gen_: _, "~е~";
    ),
    // Полкроны: 5 words (полкроны, полстраны, полшколы, ползалупы, полпланеты). Like Полгода, except in 5 cells: nom sg, gen_ sg, gen_ pl, dat sg, ins sg.
    declension!("полкроны", 5, "~~ы", like Полгода;
    nom: "~~ы", _;
    gen_: "~у~ы", "~у~";
    dat: "~у~е", _;
    ins: "~у~ой", _;
    ),
    // Тиса: 5 words (линда, шила, тиса, драва, пуста). Like Недра, except in 3 cells: dat pl, ins pl, prep pl.
    declension!("тиса", 5, "~а", [gender: Feminine], like Недра;
    dat: _, "~а";
    ins: _, "~а";
    prep: _, "~а";
    ),
    // Кухня: 5 words (кухня, куфня, кормокухня, авиакухня, автокухня). Like Подворотня, except in 1 cell: gen_ pl.
    declension!("кухня", 5, "~~я", [gender: Feminine, animacy: Inanimate], like Подворотня;
    gen_: _, "~о~ь";
    ),
    // Семя: 4 words (темя, семя, стремя, льносемя). Like Племя, except in 1 cell: gen_ pl.
    declension!("семя", 4, "~я", [gender: Neuter, animacy: Inanimate], like Племя;
    gen_: _, "~ян";
    ),
    // Небо: 4 words (чудо, небо, древо, худо). Like Сходство.
    declension!("небо", 4, "~о", [gender: Neuter, animacy: Inanimate], like Сходство;
    stem pl: "~ес- like сходство";
    ),
    // Судно: 4 words (судно, агитсудно, авиасудно, фальшдно). Like Бега, except in 1 cell: nom sg.
    declension!("судно", 4, "~но", [gender: Neuter, animacy: Inanimate], like Бега;
    stem gen_sg: "~н- like сходство";
    nom: "~но", _;
    ),
    // Улей: 4 words (улей, жеребей, чирей, ирей). Like Репей, except in 2 cells: gen_ pl, noun_second_case part.
    declension!("улей", 4, "~ей", [gender: Masculine, animacy: Inanimate], like Репей;
    stem gen_sg: "~ь- like медведь";
    gen_: _, "~ьев";
    noun_second_case: derived, _;
    ),
    // Учащийся: 4 words (учащийся, трудящийся, вольноопределяющийся, праздношатающийся). Like Кормчий with `~` after each pattern.
    declension!("учащийся", 4, "~ий~", [gender: Masculine, animacy: Animate], like Кормчий with "~";
    ),
    // Лишай: 4 words (холуй, лишай, бугай, валуй). Like Бой, except in 1 cell: noun_second_case part.
    declension!("лишай", 4, "~й", [gender: Masculine], like Бой;
    stem gen_sg: "~- like червь";
    noun_second_case: derived, _;
    ),
    // Капли: 4 words (грабли, капли, чапли, велотуфли). Like Нападки, except in 4 cells: gen_ pl, dat pl, ins pl, prep pl.
    declension!("капли", 4, "~~и", [gender: Feminine, animacy: Inanimate], like Нападки;
    gen_: _, "~е~ь";
    dat: _, "~~ям";
    ins: _, "~~ями";
    prep: _, "~~ях";
    ),
    // Пресмыкающееся: 4 words (пресмыкающееся, случившееся, сбывшееся, несбывшееся). Like Млекопитающее with `~` after each pattern.
    declension!("пресмыкающееся", 4, "~ее~", [gender: Neuter], like Млекопитающее with "~";
    ),
    // Ничья: 4 words (ничья, третья, девичья, моя). Like Съезжая, except in 1 cell: nom pl.
    declension!("ничья", 4, "~я", [gender: Feminine], like Съезжая;
    stem lemma: "~- like химия";
    nom: _, "~и";
    ),
    // Яйки: 4 words (обойки, валуйки, яйки, прощайки). Like Кулички, except in 5 cells: nom sg, nom pl, dat pl, ins pl, prep pl.
    declension!("яйки", 4, "~й~и", [animacy: Inanimate], like Кулички;
    nom: "~й~и", "~й~и";
    dat: _, "~й~ам";
    ins: _, "~й~ами";
    prep: _, "~й~ах";
    ),
    // Сынуля: 4 words (сынуля, саня, меля, игруля). Like Баня.
    declension!("сынуля", 4, "~я", [gender: Masculine, animacy: Animate], like Баня;
    stem pl: "~- like сынуля";
    ),
    // Карло: 4 words (ракло, шарыпово, запевало, карло). Like Раз with `` after each pattern (`о` after ~), except in 1 cell: gen_ pl.
    declension!("карло", 4, "~о", like Раз with "", "о" after "~";
    gen_: _, "~";
    ),
    // Дитятко: 4 words (дитятко, старичонко, золотко, мужичонко). Like Сиверко, except in 1 cell: acc sg.
    declension!("дитятко", 4, "~~о", [animacy: Animate], like Сиверко;
    acc: "~~о", _;
    ),
    // Тамада: 4 words (тамада, фавела, яга, нигга). Like Полнота, except in 1 cell: acc pl.
    declension!("тамада", 4, "~а", like Полнота;
    acc: _, "~";
    ),
    // Глаз: 3 words (глаз, телеглаз, киноглаз). Like Сходство, except in 1 cell: nom sg.
    declension!("глаз", 3, "~", [gender: Masculine, animacy: Inanimate], like Сходство;
    nom: "~", _;
    ),
    // Земля: 3 words (земля, петля, полупетля). Like Деревня.
    declension!("земля", 3, "~~я", [gender: Feminine, animacy: Inanimate], like Деревня;
    stem gen_sg: "~~- like заря";
    ),
    // Мать: 3 words (дочь, мать, яжмать).
    declension!("мать", 3, "~ь", [gender: Feminine, animacy: Animate];
    stem lemma: "~ь- like скопление";
    stem pl: "~ер- like склонность";
    gen_: "~ери", _;
    dat: "~ери", _;
    ins: "~ерью", _;
    prep: "~ери", _;
    ),
    // Путь: 3 words (путь, автопуть, севморпуть). Like Склонность, except in 2 cells: ins sg, noun_second_case loc.
    declension!("путь", 3, "~ь", [gender: Masculine, animacy: Inanimate], like Склонность;
    ins: "~ём", _;
    noun_second_case: _, derived;
    ),
    // Лёд: 3 words (лёд, лён, снеголёд). Like Гусёк, except in 1 cell: noun_second_case part.
    declension!("лёд", 3, "~ё~", [gender: Masculine, animacy: Inanimate], like Гусёк;
    stem pl: "~ь~- like приём";
    stem gen_sg: "~ь~- like приём";
    noun_second_case: derived, _;
    ),
    // Сколько: 3 words (сколько, столько, несколько). Like Домашние, except in 6 cells: nom sg, nom pl, gen_ pl, dat pl, ins pl, prep pl.
    declension!("сколько", 3, "~о", like Домашние;
    nom: "~о", "~о";
    gen_: _, "~их|~их";
    dat: _, "~им|~им";
    ins: _, "~ими|~ими";
    prep: _, "~их|~их";
    ),
    // Князь: 3 words (зять, деверь, князь). Like Медведь.
    declension!("князь", 3, "~ь", [gender: Masculine, animacy: Animate], like Медведь;
    stem pl: "~- like жнивьё";
    ),
    // Кто: 3 words (кто, кое-кто, никто). Like Зелёный, except in 7 cells: nom sg, nom pl, gen_ pl, dat pl, ins sg, ins pl, ….
    declension!("кто", 3, "~то", [gender: Masculine], like Зелёный;
    nom: "~то", "~то";
    gen_: _, "~ого";
    dat: _, "~ому";
    ins: "~ем", "~ем";
    prep: _, "~ом";
    ),
    // Дрянь: 3 words (смерть, дрянь, мразь). Like Склонность, except in 1 cell: noun_second_case loc.
    declension!("дрянь", 3, "~ь", [gender: Feminine, animacy: Inanimate], like Склонность;
    stem pl: "~- like дрянь";
    noun_second_case: _, derived;
    ),
    // Ничто: 3 words (что, ничто, кое-что). Like Кто, except in 6 cells: gen_ sg, gen_ pl, dat sg, dat pl, prep sg, prep pl.
    declension!("ничто", 3, "~то", [gender: Neuter], like Кто;
    gen_: "~его", "~его";
    dat: "~ему", "~ему";
    prep: derived, derived;
    ),
    // Епитимья: 3 words (ектенья, паремья, епитимья). Like Сватья.
    declension!("епитимья", 3, "~ья", [gender: Feminine, animacy: Inanimate], like Сватья;
    stem gen_sg: "~ь- like заря";
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
    stem pl: "~~юд- like получеловек";
    stem gen_sg: "~че~овек- like сходство";
    nom: "~че~овек", _;
    ),
    // Морячка: 3 words (морячка, рыбачка, тестюшка). Like Дорожка, except in 1 cell: acc pl.
    declension!("морячка", 3, "~~а", [animacy: Animate], like Дорожка;
    acc: _, "~~и";
    ),
    // Бий: 3 words (бабай, бабий, бий). Like Змей, except in 3 cells: acc sg, acc pl, prep sg.
    declension!("бий", 3, "~й", [gender: Masculine, animacy: Animate], like Змей;
    acc: "~й", "~и";
    prep: "~и|~е", _;
    ),
    // Талдыченье: 3 words (средостение, благоусмотренье, талдыченье). Like Устье.
    declension!("талдыченье", 3, "~е", [gender: Neuter, animacy: Inanimate], like Устье;
    stem pl: "~- like скопление";
    ),
    // Оладий: 1 words (жеребий). Like Улей, except in 1 cell: nom sg.
    declension!("оладий", 1, "~ий", like Улей;
    nom: "~ий", _;
    ),
    // Проводины: 1 words (проводины). Like Мощи with `` after each pattern (`ны` after и), except in 3 cells: nom pl, gen_ pl, ins pl.
    declension!("проводины", 1, "~ины", like Мощи with "", "ны" after "и";
    nom: _, "~ы";
    gen_: _, "~ов";
    ins: _, "~ами";
    ),
];
