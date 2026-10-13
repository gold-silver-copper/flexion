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
/// `nom` (sg, pl)
/// `gen_` (sg, pl)
/// `dat` (sg, pl)
/// `acc` (sg, pl)
/// `ins` (sg, pl)
/// `prep` (sg, pl)
/// `noun_second_case` (part, loc)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, [$($f:ident: $v:ident),*])? $(, like $parent:ident $(as $view:ident)?)?;
     $(stem noun_nom_sg: $s0:tt;)?
     $(stem noun_pl: $s1:tt;)?
     $(stem noun_sg: $s2:tt;)?
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

/// An inherent feature's value, as `(feature, value)` by their places.
macro_rules! feature {
    (gender: $v:ident) => {
        (0, crate::Gender::$v as u8)
    };
    (animacy: $v:ident) => {
        (1, crate::Animacy::$v as u8)
    };
}

/// A view: per cell (in the cells' order), the cell of the declension it is
/// like that it reads (`loc_as_part`: one value read as another).
const LOC_AS_PART: [u16; 14] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 12];

/// A view: per cell (in the cells' order), the cell of the declension it is
/// like that it reads (`gen_as_prep`: one value read as another).
const GEN_AS_PREP: [u16; 14] = [0, 1, 10, 11, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];

/// A view: per cell (in the cells' order), the cell of the declension it is
/// like that it reads (`gen_as_nom`: one value read as another).
const GEN_AS_NOM: [u16; 14] = [0, 1, 0, 1, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];

/// A view: per cell (in the cells' order), the cell of the declension it is
/// like that it reads (`sg_as_pl`: one value read as another).
const SG_AS_PL: [u16; 14] = [1, 1, 3, 3, 5, 5, 7, 7, 9, 9, 11, 11, 12, 13];

/// A view: per cell (in the cells' order), the cell of the declension it is
/// like that it reads (`nom_as_gen`: one value read as another).
const NOM_AS_GEN: [u16; 14] = [2, 3, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];

/// A view: per cell (in the cells' order), the cell of the declension it is
/// like that it reads (`nom_as_prep`: one value read as another).
const NOM_AS_PREP: [u16; 14] = [10, 11, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];

/// A view: per cell (in the cells' order), the cell of the declension it is
/// like that it reads (`acc_as_prep`: one value read as another).
const ACC_AS_PREP: [u16; 14] = [0, 1, 2, 3, 4, 5, 10, 11, 8, 9, 10, 11, 12, 13];

/// A view: per cell (in the cells' order), the cell of the declension it is
/// like that it reads (`acc_as_nom`: one value read as another).
const ACC_AS_NOM: [u16; 14] = [0, 1, 2, 3, 4, 5, 0, 1, 8, 9, 10, 11, 12, 13];

/// A view: per cell (in the cells' order), the cell of the declension it is
/// like that it reads (`pl_as_sg`: one value read as another).
const PL_AS_SG: [u16; 14] = [0, 0, 2, 2, 4, 4, 6, 6, 8, 8, 10, 10, 12, 13];

/// A view: per cell (in the cells' order), the cell of the declension it is
/// like that it reads (`prep_as_acc`: one value read as another).
const PREP_AS_ACC: [u16; 14] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 6, 7, 12, 13];

/// A view: per cell (in the cells' order), the cell of the declension it is
/// like that it reads (`prep_as_nom`: one value read as another).
const PREP_AS_NOM: [u16; 14] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 12, 13];

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `приём`: 37,684 words.
    Приём,
    /// Like `полнота`: 12,547 words.
    Полнота,
    /// Like `скопление`: 9,052 words.
    Скопление,
    /// Like `склонность`: 7,532 words.
    Склонность,
    /// Like `стрелка`: 6,672 words.
    Стрелка,
    /// Like `химия`: 6,582 words.
    Химия,
    /// Like `какао`: 3,419 words.
    Какао,
    /// Like `хлопанье`: 2,643 words.
    Хлопанье,
    /// Like `дорожка`: 2,522 words.
    Дорожка,
    /// Like `сходство`: 2,331 words.
    Сходство,
    /// Like `агнец`: 2,159 words.
    Агнец,
    /// Like `медведь`: 1,980 words.
    Медведь,
    /// Like `щиток`: 1,052 words.
    Щиток,
    /// Like `товарищ`: 932 words.
    Товарищ,
    /// Like `змей`: 650 words.
    Змей,
    /// Like `гений`: 649 words.
    Гений,
    /// Like `барашек`: 639 words.
    Барашек,
    /// Like `чайка`: 474 words.
    Чайка,
    /// Like `накладная`: 415 words.
    Накладная,
    /// Like `господин`: 395 words.
    Господин,
    /// Like `постав`: 391 words.
    Постав,
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
    /// Like `второе`: 179 words.
    Второе,
    /// Like `умелец`: 174 words.
    Умелец,
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
    /// Like `жнивьё`: 119 words.
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
    /// Like `штрафной`: 79 words.
    Штрафной,
    /// Like `абзац`: 76 words.
    Абзац,
    /// Like `достопримечательность`: 76 words.
    Достопримечательность,
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
    /// Like `устье`: 40 words.
    Устье,
    /// Like `делёж`: 38 words.
    Делёж,
    /// Like `съезжая`: 37 words.
    Съезжая,
    /// Like `помочь`: 36 words.
    Помочь,
    /// Like `бой`: 36 words.
    Бой,
    /// Like `отец`: 33 words.
    Отец,
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
    /// Like `кешью`: 30 words.
    Кешью,
    /// Like `деревня`: 29 words.
    Деревня,
    /// Like `верея`: 29 words.
    Верея,
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
    /// Like `тельце`: 21 words.
    Тельце,
    /// Like `клин`: 19 words.
    Клин,
    /// Like `сенцы`: 19 words.
    Сенцы,
    /// Like `меринос`: 18 words.
    Меринос,
    /// Like `боёк`: 18 words.
    Боёк,
    /// Like `детище`: 17 words.
    Детище,
    /// Like `колёсико`: 16 words.
    Колёсико,
    /// Like `кокцинеллида`: 16 words.
    Кокцинеллида,
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
    /// Like `племя`: 10 words.
    Племя,
    /// Like `хлопья`: 10 words.
    Хлопья,
    /// Like `кукла`: 9 words.
    Кукла,
    /// Like `череда`: 9 words.
    Череда,
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
    /// Like `лапоть`: 7 words.
    Лапоть,
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
    /// Like `пол-игры`: 7 words.
    Полигры,
    /// Like `любовь`: 7 words.
    Любовь,
    /// Like `пол-июля`: 7 words.
    Полиюля,
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
    /// Like `пол-осени`: 5 words.
    Полосени,
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
    /// Like `тамада`: 4 words.
    Тамада,
    /// Like `усадьба`: 3 words.
    Усадьба,
    /// Like `дичь`: 3 words.
    Дичь,
    /// Like `глаз`: 3 words.
    Глаз,
    /// Like `земля`: 3 words.
    Земля,
    /// Like `дочь`: 3 words.
    Дочь,
    /// Like `путь`: 3 words.
    Путь,
    /// Like `лёд`: 3 words.
    Лёд,
    /// Like `сколько`: 3 words.
    Сколько,
    /// Like `зять`: 3 words.
    Зять,
    /// Like `кто`: 3 words.
    Кто,
    /// Like `дрянь`: 3 words.
    Дрянь,
    /// Like `что`: 3 words.
    Что,
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
    /// Like `бабай`: 3 words.
    Бабай,
    /// Like `пол-леса`: 3 words.
    Поллеса,
    /// Like `пол-юбки`: 3 words.
    Полюбки,
    /// Like `средостение`: 3 words.
    Средостение,
    /// Like `оладий`: 1 words.
    Оладий,
    /// Like `электропроводимость`: 1 words.
    Электропроводимость,
    /// Like `обрядник`: 1 words.
    Обрядник,
    /// Like `вольтметр`: 1 words.
    Вольтметр,
    /// Like `гармонщик`: 1 words.
    Гармонщик,
    /// Like `протечка`: 1 words.
    Протечка,
    /// Like `паяльщик`: 1 words.
    Паяльщик,
    /// Like `полешко`: 1 words.
    Полешко,
    /// Like `даунсайзинг`: 1 words.
    Даунсайзинг,
    /// Like `копрофил`: 1 words.
    Копрофил,
    /// Like `полудурье`: 1 words.
    Полудурье,
    /// Like `проводины`: 1 words.
    Проводины,
    /// Like `ёко`: 1 words.
    Ёко,
    /// Like `амблинойя`: 1 words.
    Амблинойя,
    /// Like `разговень`: 1 words.
    Разговень,
    /// Like `супремасизм`: 1 words.
    Супремасизм,
    /// Like `воздухан`: 1 words.
    Воздухан,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 205] = [
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
        Declension::Господин,
        Declension::Постав,
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
        Declension::Второе,
        Declension::Умелец,
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
        Declension::Штрафной,
        Declension::Абзац,
        Declension::Достопримечательность,
        Declension::Кулички,
        Declension::Дух,
        Declension::Ядро,
        Declension::Внучонок,
        Declension::Раз,
        Declension::Недра,
        Declension::Свеча,
        Declension::Каракуля,
        Declension::Котище,
        Declension::Устье,
        Declension::Делёж,
        Declension::Съезжая,
        Declension::Помочь,
        Declension::Бой,
        Declension::Отец,
        Declension::Горючее,
        Declension::Дрязги,
        Declension::Шов,
        Declension::Ладья,
        Declension::Член,
        Declension::Кешью,
        Declension::Деревня,
        Declension::Верея,
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
        Declension::Тельце,
        Declension::Клин,
        Declension::Сенцы,
        Declension::Меринос,
        Declension::Боёк,
        Declension::Детище,
        Declension::Колёсико,
        Declension::Кокцинеллида,
        Declension::Анкара,
        Declension::День,
        Declension::Зло,
        Declension::Бойня,
        Declension::Побои,
        Declension::Перо,
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
        Declension::Племя,
        Declension::Хлопья,
        Declension::Кукла,
        Declension::Череда,
        Declension::Плечо,
        Declension::Горе,
        Declension::Герц,
        Declension::Калашников,
        Declension::Плюхино,
        Declension::Лапоть,
        Declension::Сиверко,
        Declension::Передняя,
        Declension::Семя,
        Declension::Стекло,
        Declension::Заём,
        Declension::Весна,
        Declension::Осётр,
        Declension::Полгода,
        Declension::Дитё,
        Declension::Полигры,
        Declension::Любовь,
        Declension::Полиюля,
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
        Declension::Полосени,
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
        Declension::Тамада,
        Declension::Усадьба,
        Declension::Дичь,
        Declension::Глаз,
        Declension::Земля,
        Declension::Дочь,
        Declension::Путь,
        Declension::Лёд,
        Declension::Сколько,
        Declension::Зять,
        Declension::Кто,
        Declension::Дрянь,
        Declension::Что,
        Declension::Епитимья,
        Declension::Дверца,
        Declension::Солнышко,
        Declension::Получеловек,
        Declension::Морячка,
        Declension::Бабай,
        Declension::Поллеса,
        Declension::Полюбки,
        Declension::Средостение,
        Declension::Оладий,
        Declension::Электропроводимость,
        Declension::Обрядник,
        Declension::Вольтметр,
        Declension::Гармонщик,
        Declension::Протечка,
        Declension::Паяльщик,
        Declension::Полешко,
        Declension::Даунсайзинг,
        Declension::Копрофил,
        Declension::Полудурье,
        Declension::Проводины,
        Declension::Ёко,
        Declension::Амблинойя,
        Declension::Разговень,
        Declension::Супремасизм,
        Declension::Воздухан,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 205] = [
    // Приём: 37,684 words (указ, зонт, кот, зонтик, бортник, бочаг).
    declension!("приём", 37684, "~", [gender: Masculine];
    stem noun_pl: "~- like приём";
    nom: "~", _;
    gen_: "~а", _;
    dat: "~у", _;
    ins: "~ом", _;
    prep: "~е", _;
    noun_second_case: _, "~у";
    ),
    // Полнота: 12,547 words (собака, буява, былица, масленица, царица, вата). Like Приём, except in 2 cells: gen_ pl, noun_second_case loc.
    declension!("полнота", 12547, "~а", [gender: Feminine], like Приём as LOC_AS_PART;
    stem noun_nom_sg: "~- like полнота";
    stem noun_sg: "~- like полнота";
    gen_: _, "~";
    noun_second_case: _, derived;
    ),
    // Скопление: 9,052 words (произношение, лезвие, распятие, деревенение, лесонасаждение, ведение).
    declension!("скопление", 9052, "~е", [gender: Neuter, animacy: Inanimate];
    stem noun_nom_sg: "~е- like скопление";
    stem noun_pl: "~- like скопление";
    stem noun_sg: "~- like скопление";
    ),
    // Склонность: 7,532 words (знать, весь, гласность, деревянность, дверь, ртуть). Like Скопление, except in 2 cells: nom sg, acc sg.
    declension!("склонность", 7532, "~ь", [gender: Feminine, animacy: Inanimate], like Скопление as GEN_AS_PREP;
    stem noun_pl: "~- like склонность";
    stem noun_sg: "~- like склонность";
    nom: "~ь", _;
    acc: derived, _;
    ),
    // Стрелка: 6,672 words (водка, цыпка, жилка, деревянка, лесопилка, доска).
    declension!("стрелка", 6672, "~~а", [gender: Feminine];
    stem noun_nom_sg: "~~- like полнота";
    stem noun_sg: "~~- like стрелка";
    nom: _, "~~и";
    gen_: _, "~о~";
    dat: _, "~~ам";
    ins: _, "~~ами";
    prep: _, "~~ах";
    ),
    // Химия: 6,582 words (анархия, лесохимия, библия, биология, академия, армия). Like Склонность, except in 2 cells: gen_ pl, ins sg.
    declension!("химия", 6582, "~я", [gender: Feminine, animacy: Inanimate], like Склонность as LOC_AS_PART;
    stem noun_nom_sg: "~- like химия";
    gen_: _, "~й";
    ins: "~ей|~ею", _;
    ),
    // Какао: 3,419 words (пальто, все, брачущиеся, брачующиеся, адажио, какао).
    declension!("какао", 3419, "~";
    stem noun_sg: "~- like какао";
    nom: "~", "~";
    gen_: _, "~";
    dat: _, "~";
    ins: _, "~";
    prep: _, "~";
    ),
    // Хлопанье: 2,643 words (запястье, воскресенье, варенье, воспитанье, отчаянье, уединенье).
    declension!("хлопанье", 2643, "~ье", [gender: Neuter, animacy: Inanimate];
    stem noun_sg: "~ь- like хлопанье";
    nom: "~ье", "~ья";
    gen_: _, "~ий";
    dat: _, "~ьям";
    ins: _, "~ьями";
    prep: _, "~ьях";
    ),
    // Дорожка: 2,522 words (кошка, быличка, палочка, ручка, мошка, бабочка). Like Стрелка, except in 1 cell: gen_ pl.
    declension!("дорожка", 2522, "~~а", like Стрелка;
    gen_: _, "~е~";
    ),
    // Сходство: 2,331 words (буево, бёрдо, око, лето, жало, горло). Like Приём, except in 1 cell: nom sg.
    declension!("сходство", 2331, "~о", [gender: Neuter, animacy: Inanimate], like Приём as LOC_AS_PART;
    stem noun_pl: "~- like сходство";
    nom: "~о", _;
    ),
    // Агнец: 2,159 words (левантинец, древоточец, марганец, самозванец, разночинец, ньюйоркец). Like Стрелка, except in 2 cells: nom sg, acc sg.
    declension!("агнец", 2159, "~е~", [gender: Masculine], like Стрелка as GEN_AS_NOM;
    stem noun_pl: "~~- like агнец";
    stem noun_sg: "~~- like агнец";
    nom: "~е~", _;
    acc: derived, _;
    ),
    // Медведь: 1,980 words (сшиватель, апрель, июнь, июль, олень, автомобиль). Like Склонность, except in 1 cell: noun_second_case loc.
    declension!("медведь", 1980, "~ь", [gender: Masculine], like Склонность as LOC_AS_PART;
    stem noun_sg: "~- like хлопанье";
    noun_second_case: _, derived;
    ),
    // Щиток: 1,052 words (щенок, бесёнок, лесок, замок, носок, подросток). Like Агнец, except in 5 cells: nom sg, nom pl, gen_ pl, ins sg, noun_second_case loc.
    declension!("щиток", 1052, "~о~", [gender: Masculine], like Агнец;
    nom: "~о~", "~~и";
    gen_: _, "~~ов";
    ins: "~~ом", _;
    noun_second_case: _, "~~у";
    ),
    // Товарищ: 932 words (харч, туш, треш, товарищ, дервиш, путч). Like Приём, except in 1 cell: noun_second_case loc.
    declension!("товарищ", 932, "~", [gender: Masculine], like Приём as LOC_AS_PART;
    stem noun_pl: "~- like товарищ";
    stem noun_sg: "~- like агнец";
    noun_second_case: _, derived;
    ),
    // Змей: 650 words (май, гей, вертухай, елей, герой, случай). Like Медведь, except in 2 cells: nom sg, gen_ pl.
    declension!("змей", 650, "~й", [gender: Masculine], like Медведь;
    nom: "~й", _;
    gen_: _, "~ев";
    ),
    // Гений: 649 words (калий, меркурий, гений, гравий, бериллий, гелий). Like Змей, except in 0 cells: .
    declension!("гений", 649, "~й", [gender: Masculine], like Змей;
    stem noun_sg: "~- like скопление";
    ),
    // Барашек: 639 words (отец, горошек, свинец, купец, творец, дворец). Like Щиток, except in 1 cell: nom sg.
    declension!("барашек", 639, "~е~", [gender: Masculine], like Щиток as LOC_AS_PART;
    nom: "~е~", _;
    ),
    // Чайка: 474 words (копейка, скамейка, перестройка, ищейка, лайка, австрийка).
    declension!("чайка", 474, "~й~а", [gender: Feminine];
    stem noun_nom_sg: "~й~- like полнота";
    stem noun_sg: "~й~- like стрелка";
    nom: _, "~й~и";
    gen_: _, "~е~";
    dat: _, "~й~ам";
    ins: _, "~й~ами";
    prep: _, "~й~ах";
    ),
    // Накладная: 415 words (деревообделочная, детская, гостиная, ванная, кладовая, уборная).
    declension!("накладная", 415, "~ая", [gender: Feminine];
    stem noun_pl: "~ы- like накладная";
    stem noun_sg: "~о- like накладная";
    nom: "~ая", _;
    acc: "~ую", _;
    ),
    // Господин: 395 words (господин, англичанин, россиянин, опочанин, гражданин, новоангличанин). Like Сходство, except in 2 cells: nom sg, nom pl.
    declension!("господин", 395, "~ин", [gender: Masculine, animacy: Animate], like Сходство;
    stem noun_sg: "~ин- like сходство";
    nom: "~ин", "~е";
    ),
    // Постав: 391 words (дом, лес, город, пояс, гидраргирум, адрес). Like Приём, except in 1 cell: nom pl.
    declension!("постав", 391, "~", [gender: Masculine], like Приём;
    nom: _, "~а";
    ),
    // Ложе: 327 words (солнце, вече, училище, кладбище, ложе, пристанище). Like Сходство, except in 1 cell: nom sg.
    declension!("ложе", 327, "~е", [animacy: Inanimate], like Сходство;
    stem noun_sg: "~- like агнец";
    nom: "~е", _;
    ),
    // Зелёный: 278 words (белый, чёрный, гласный, беспризорный, коридорный, левый). Like Накладная, except in 5 cells: nom sg, gen_ sg, dat sg, ins sg, prep sg.
    declension!("зелёный", 278, "~ый", [gender: Masculine], like Накладная as SG_AS_PL;
    nom: "~ый", _;
    gen_: "~ого", _;
    dat: "~ому", _;
    ins: "~ым", _;
    prep: "~ом", _;
    ),
    // Ключ: 276 words (борщ, карандаш, врач, сыч, нож, меч). Like Товарищ, except in 0 cells: .
    declension!("ключ", 276, "~", [gender: Masculine], like Товарищ;
    stem noun_sg: "~- like сходство";
    ),
    // Люлька: 264 words (пенька, пиписька, кухонька, батенька, калька, свистулька).
    declension!("люлька", 264, "~ь~а";
    stem noun_nom_sg: "~ь~- like полнота";
    stem noun_sg: "~ь~- like стрелка";
    nom: _, "~ь~и";
    gen_: _, "~е~";
    dat: _, "~ь~ам";
    ins: _, "~ь~ами";
    prep: _, "~ь~ах";
    ),
    // Баня: 253 words (братыня, неделя, буря, няня, княгиня, тётя). Like Химия, except in 1 cell: gen_ pl.
    declension!("баня", 253, "~я", like Химия;
    stem noun_sg: "~- like баня";
    gen_: _, "~ь";
    ),
    // Аниец: 250 words (австриец, австралиец, ариец, белогвардеец, китаец, византиец). Like Чайка, except in 2 cells: nom sg, acc sg.
    declension!("аниец", 250, "~е~", [gender: Masculine, animacy: Animate], like Чайка as GEN_AS_NOM;
    stem noun_pl: "~й~- like агнец";
    stem noun_sg: "~й~- like агнец";
    nom: "~е~", _;
    acc: derived, _;
    ),
    // Червь: 232 words (царь, рубль, кремль, дождь, январь, февраль). Like Медведь, except in 1 cell: ins sg.
    declension!("червь", 232, "~ь", [gender: Masculine], like Медведь;
    ins: "~ём", _;
    ),
    // Шоколадница: 218 words (пицца, кожа, мульча, грыжа, мрежа, туча). Like Полнота, except in 0 cells: .
    declension!("шоколадница", 218, "~а", like Полнота;
    stem noun_sg: "~- like шоколадница";
    ),
    // Счёты: 203 words (часы, роды, древоеды, щипцы, боеприпасы, французы). Like Приём, except in 5 cells: nom sg, gen_ sg, dat sg, ins sg, prep sg.
    declension!("счёты", 203, "~ы", [gender: Masculine], like Приём as LOC_AS_PART;
    nom: "~ы", _;
    gen_: derived, _;
    dat: derived, _;
    ins: derived, _;
    prep: derived, _;
    ),
    // Русский: 201 words (русский, сербскохорватский, английский, рабочий, норвежский, китайский). Like Зелёный, except in 2 cells: nom sg, ins sg.
    declension!("русский", 201, "~ий", [gender: Masculine], like Зелёный;
    stem noun_pl: "~и- like накладная";
    nom: "~ий", _;
    ins: "~им", _;
    ),
    // Батарея: 194 words (статуя, стая, шея, идея, затея, фея). Like Баня, except in 1 cell: gen_ pl.
    declension!("батарея", 194, "~я", [gender: Feminine], like Баня;
    gen_: _, "~й";
    ),
    // Лососёвые: 192 words (данные, чаевые, присные, месячные, наличные, родные). Like Счёты, except in 1 cell: nom sg.
    declension!("лососёвые", 192, "~е", like Счёты;
    stem noun_pl: "~- like накладная";
    nom: "~е", _;
    ),
    // Второе: 179 words (былое, насекомое, существительное, числительное, многое, животное). Like Зелёный, except in 0 cells: .
    declension!("второе", 179, "~ое", [gender: Neuter], like Зелёный;
    stem noun_nom_sg: "~ое- like скопление";
    ),
    // Умелец: 174 words (палец, комсомолец, земледелец, бразилец, владелец, анголец). Like Люлька, except in 2 cells: nom sg, acc sg.
    declension!("умелец", 174, "~е~", [gender: Masculine], like Люлька as GEN_AS_NOM;
    stem noun_pl: "~ь~- like агнец";
    stem noun_sg: "~ь~- like агнец";
    nom: "~е~", _;
    acc: derived, _;
    ),
    // Сватья: 151 words (гостья, авиаэскадрилья, эскадрилья, келья, тортилья, колдунья). Like Хлопанье, except in 1 cell: nom pl.
    declension!("сватья", 151, "~ья", [gender: Feminine], like Хлопанье as NOM_AS_GEN;
    stem noun_nom_sg: "~ь- like химия";
    stem noun_sg: "~ь- like баня";
    nom: _, "~ьи";
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
    // Мостки: 147 words (древпластики, духи, щипчики, мозги, падонки, очки). Like Товарищ, except in 1 cell: gen_ pl.
    declension!("мостки", 147, "~и", [gender: Masculine, animacy: Inanimate], like Товарищ as SG_AS_PL;
    stem noun_sg: "~и- like какао";
    gen_: _, "~ов";
    ),
    // Заря: 127 words (ступня, стезя, простыня, тля, хуйня, броня). Like Баня, except in 0 cells: .
    declension!("заря", 127, "~я", [gender: Feminine], like Баня;
    stem noun_pl: "~- like склонность";
    stem noun_sg: "~- like заря";
    ),
    // Орлёнок: 124 words (телёнок, октябрёнок, поросёнок, жеребёнок, ягнёнок, козлёнок).
    declension!("орлёнок", 124, "~ёнок", [gender: Masculine, animacy: Animate];
    stem noun_pl: "~ят- like сходство";
    stem noun_sg: "~ёнк- like сходство";
    nom: "~ёнок", _;
    ),
    // Подворотня: 120 words (вишня, песня, бушня, пригоршня, башня, пекарня). Like Дорожка, except in 3 cells: dat pl, ins pl, prep pl.
    declension!("подворотня", 120, "~~я", [gender: Feminine, animacy: Inanimate], like Дорожка;
    stem noun_nom_sg: "~~- like химия";
    stem noun_sg: "~~- like баня";
    dat: _, "~~ям";
    ins: _, "~~ями";
    prep: _, "~~ях";
    ),
    // Жнивьё: 119 words (копьё, питьё, ружьё, битьё, враньё, сырьё). Like Хлопанье, except in 2 cells: nom sg, ins sg.
    declension!("жнивьё", 119, "~ьё", [gender: Neuter, animacy: Inanimate], like Хлопанье;
    stem noun_pl: "~- like жнивьё";
    nom: "~ьё", _;
    ins: "~ьём", _;
    ),
    // Ясли: 113 words (люди, кости, черви, внутренности, вожжи, ясли). Like Медведь, except in 4 cells: gen_ sg, dat sg, ins sg, prep sg.
    declension!("ясли", 113, "~и", [animacy: Inanimate], like Медведь as SG_AS_PL;
    gen_: derived, _;
    dat: derived, _;
    ins: derived, _;
    prep: derived, _;
    ),
    // Кормчий: 107 words (лесничий, посторонний, нищий, военнослужащий, домашний, желающий). Like Русский, except in 0 cells: .
    declension!("кормчий", 107, "~ий", [gender: Masculine, animacy: Animate], like Русский;
    stem noun_sg: "~- like кормчий";
    ),
    // Камень: 107 words (бредень, оборотень, камень, парень, полдень, кашель). Like Подворотня, except in 2 cells: nom sg, acc sg.
    declension!("камень", 107, "~е~ь", [gender: Masculine], like Подворотня as GEN_AS_NOM;
    stem noun_pl: "~~- like склонность";
    stem noun_sg: "~~- like хлопанье";
    nom: "~е~ь", _;
    acc: derived, _;
    ),
    // Гривна: 106 words (сосна, домна, царевна, гривна, кошма, плюсна). Like Дорожка, except in 1 cell: nom pl.
    declension!("гривна", 106, "~~а", [gender: Feminine], like Дорожка;
    stem noun_sg: "~~- like полнота";
    nom: _, "~~ы";
    ),
    // Нападки: 100 words (лесопосадки, плавки, сутки, сливки, колготки, поминки). Like Стрелка, except in 4 cells: gen_ sg, dat sg, ins sg, prep sg.
    declension!("нападки", 100, "~~и", [gender: Feminine, animacy: Inanimate], like Стрелка as SG_AS_PL;
    gen_: derived, _;
    dat: derived, _;
    ins: derived, _;
    prep: derived, _;
    ),
    // Гусёк: 89 words (мотылёк, хорёк, кошелёк, королёк, огонёк, уголёк). Like Умелец, except in 3 cells: nom sg, nom pl, gen_ pl.
    declension!("гусёк", 89, "~ё~", [gender: Masculine], like Умелец;
    stem noun_sg: "~ь~- like сходство";
    nom: "~ё~", "~ь~и";
    gen_: _, "~ь~ов";
    ),
    // Выдача: 85 words (тёща, пища, вельможа, гуща, роща, толща). Like Шоколадница, except in 1 cell: nom pl.
    declension!("выдача", 85, "~а", like Шоколадница;
    stem noun_sg: "~- like баня";
    nom: _, "~и";
    ),
    // Спальня: 85 words (лесопильня, спальня, читальня, колокольня, коптильня, наковальня). Like Люлька, except in 3 cells: dat pl, ins pl, prep pl.
    declension!("спальня", 85, "~ь~я", [gender: Feminine, animacy: Inanimate], like Люлька;
    stem noun_nom_sg: "~ь~- like химия";
    stem noun_sg: "~ь~- like баня";
    dat: _, "~ь~ям";
    ins: _, "~ь~ями";
    prep: _, "~ь~ях";
    ),
    // Штрафной: 79 words (больной, голубой, часовой, золотой, вестовой, немой). Like Зелёный, except in 1 cell: nom sg.
    declension!("штрафной", 79, "~ой", [gender: Masculine], like Зелёный;
    nom: "~ой", _;
    ),
    // Абзац: 76 words (месяц, раж, принц, абзац, кош, кварц). Like Приём, except in 1 cell: ins sg.
    declension!("абзац", 76, "~", [gender: Masculine], like Приём;
    stem noun_pl: "~- like агнец";
    ins: "~ем", _;
    ),
    // Достопримечательность: 76 words (лошадь, сельдь, свекровь, достопримечательность, моль, блядь). Like Склонность, except in 1 cell: acc sg.
    declension!("достопримечательность", 76, "~ь", [gender: Feminine, animacy: Animate], like Склонность as LOC_AS_PART;
    acc: "~ь", _;
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
    stem noun_pl: "~ат- like сходство";
    stem noun_sg: "~онк- like сходство";
    nom: "~онок", _;
    ),
    // Раз: 53 words (солдат, цыган, раз, гусар, вольт, погон). Like Приём, except in 1 cell: gen_ pl.
    declension!("раз", 53, "~", [gender: Masculine], like Приём as LOC_AS_PART;
    gen_: _, "~";
    ),
    // Недра: 52 words (ребята, врата, дрова, уста, внучата, правнучата). Like Латы, except in 1 cell: nom sg.
    declension!("недра", 52, "~а", like Латы;
    stem noun_pl: "~- like сходство";
    nom: "~а", _;
    ),
    // Свеча: 45 words (левша, ханжа, вожжа, свеча, юноша, саранча). Like Выдача, except in 0 cells: .
    declension!("свеча", 45, "~а", like Выдача;
    stem noun_pl: "~- like товарищ";
    stem noun_sg: "~- like стрелка";
    ),
    // Каракуля: 45 words (постеля, батя, дядя, доля, ходуля, сводня). Like Баня, except in 0 cells: .
    declension!("каракуля", 45, "~я", like Баня;
    stem noun_pl: "~- like склонность";
    ),
    // Котище: 40 words (дружище, когтище, волчище, хуище, козлище, третье). Like Ложе, except in 1 cell: nom pl.
    declension!("котище", 40, "~е", [gender: Masculine], like Ложе;
    nom: _, "~и";
    ),
    // Устье: 40 words (платье, устье, подмастерье, либеральничанье, подполье, уголье). Like Змей, except in 0 cells: .
    declension!("устье", 40, "~е", [animacy: Inanimate], like Змей as NOM_AS_PREP;
    stem noun_pl: "~- like устье";
    ),
    // Делёж: 38 words (падёж, пиздёж, чертёж, грабёж, платёж, мухлёж).
    declension!("делёж", 38, "~ё~", [gender: Masculine, animacy: Inanimate];
    stem noun_pl: "~е~- like товарищ";
    stem noun_sg: "~е~- like сходство";
    nom: "~ё~", _;
    ),
    // Съезжая: 37 words (прихожая, телеведущая, неверующая, купчая, составляющая, заведующая). Like Русский, except in 2 cells: nom sg, acc sg.
    declension!("съезжая", 37, "~ая", [gender: Feminine], like Русский;
    stem noun_sg: "~е- like накладная";
    nom: "~ая", _;
    acc: "~ую", _;
    ),
    // Помочь: 36 words (мочь, тушь, нощь, пещь, помощь, немощь). Like Склонность, except in 0 cells: .
    declension!("помочь", 36, "~ь", [gender: Feminine, animacy: Inanimate], like Склонность;
    stem noun_pl: "~- like товарищ";
    ),
    // Бой: 36 words (чай, хуй, бой, рой, слой, край). Like Змей, except in 1 cell: gen_ pl.
    declension!("бой", 36, "~й", [gender: Masculine], like Змей;
    gen_: _, "~ёв";
    ),
    // Отец: 33 words (ветер, узел, пепел, дятел, овен, домысел). Like Барашек, except in 0 cells: .
    declension!("отец", 33, "~е~", [gender: Masculine], like Барашек;
    stem noun_pl: "~~- like приём";
    ),
    // Горючее: 33 words (настоящее, будущее, горючее, грядущее, прочее, болеутоляющее). Like Кормчий, except in 1 cell: nom sg.
    declension!("горючее", 33, "~ее", [gender: Neuter, animacy: Inanimate], like Кормчий;
    nom: "~ее", _;
    ),
    // Дрязги: 31 words (брюки, ладоши, лыжи, вериги, пики, брызги). Like Выдача, except in 1 cell: nom sg.
    declension!("дрязги", 31, "~и", [gender: Feminine, animacy: Inanimate], like Выдача;
    stem noun_sg: "~- like дрязги";
    nom: "~и", _;
    ),
    // Шов: 30 words (рот, свёкор, лоб, сон, хохол, угол). Like Щиток, except in 0 cells: .
    declension!("шов", 30, "~о~", [gender: Masculine], like Щиток;
    stem noun_pl: "~~- like приём";
    ),
    // Ладья: 30 words (семья, ладья, статья, судья, скамья, голья). Like Сватья, except in 1 cell: gen_ pl.
    declension!("ладья", 30, "~ья", like Сватья;
    stem noun_sg: "~ь- like заря";
    gen_: _, "~ей";
    ),
    // Член: 30 words (клон, тип, член, кат, кекс, бас). Like Приём, except in 0 cells: .
    declension!("член", 30, "~", [gender: Masculine, animacy: Inanimate], like Приём as LOC_AS_PART;
    stem noun_nom_sg: "~- like член";
    stem noun_pl: "~- like член";
    ),
    // Кешью: 30 words (дитя, порося, кешью, чапаев, гэта, мюсли). Like Какао, except in 0 cells: .
    declension!("кешью", 30, "~", like Какао as ACC_AS_PREP;
    ),
    // Деревня: 29 words (торговля, сабля, капля, туфля, чапля, цапля). Like Подворотня, except in 1 cell: gen_ pl.
    declension!("деревня", 29, "~~я", [gender: Feminine], like Подворотня;
    gen_: _, "~е~ь";
    ),
    // Верея: 29 words (змея, колея, чешуя, швея, струя, алия). Like Батарея, except in 0 cells: .
    declension!("верея", 29, "~я", [gender: Feminine], like Батарея;
    stem noun_sg: "~- like заря";
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
    stem noun_pl: "~у~- like приём";
    stem noun_sg: "~у~- like сходство";
    nom: "~-~а", _;
    ),
    // Голец: 24 words (лев, телец, стрелец, жилец, делец, малец). Like Умелец, except in 0 cells: .
    declension!("голец", 24, "~е~", [gender: Masculine], like Умелец;
    stem noun_pl: "~ь~- like приём";
    stem noun_sg: "~ь~- like сходство";
    ),
    // Домашние: 24 words (немногие, сколькие, брюхоногие, близкие, древние, домашние). Like Горючее, except in 1 cell: acc sg.
    declension!("домашние", 24, "~ие", like Горючее;
    acc: "~ее", _;
    ),
    // Звезда: 23 words (звезда, пчела, жена, сестра, медсестра, береста).
    declension!("звезда", 23, "~е~а", [gender: Feminine];
    stem noun_nom_sg: "~е~- like полнота";
    stem noun_pl: "~ё~- like полнота";
    stem noun_sg: "~е~- like полнота";
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
    stem noun_sg: "~е~~- like сходство";
    nom: "~е~~о", "~ё~~а";
    gen_: _, "~ё~е~";
    dat: _, "~ё~~ам";
    ins: _, "~ё~~ами";
    prep: _, "~ё~~ах";
    ),
    // Маракуйя: 22 words (аллилуйя, секвойя, паранойя, папайя, пирайя, майя). Like Баня, except in 1 cell: gen_ pl.
    declension!("маракуйя", 22, "~я", [gender: Feminine], like Баня;
    gen_: _, "~";
    ),
    // Деньги: 21 words (деньги, четвереньки, грабельки, хахоньки, ахоньки, ясельки). Like Люлька, except in 4 cells: gen_ sg, dat sg, ins sg, prep sg.
    declension!("деньги", 21, "~ь~и", [gender: Feminine, animacy: Inanimate], like Люлька as SG_AS_PL;
    gen_: derived, _;
    dat: derived, _;
    ins: derived, _;
    prep: derived, _;
    ),
    // Тельце: 21 words (болотце, тельце, коленце, ведёрце, весельце, дульце). Like Агнец, except in 1 cell: nom pl.
    declension!("тельце", 21, "~~е", [gender: Neuter, animacy: Inanimate], like Агнец as NOM_AS_PREP;
    nom: _, "~~а";
    ),
    // Клин: 19 words (брат, лист, муж, клин, стул, струп). Like Приём, except in 0 cells: .
    declension!("клин", 19, "~", [gender: Masculine], like Приём;
    stem noun_pl: "~ь- like устье";
    ),
    // Сенцы: 19 words (японцы, плоскогубцы, плоскозубцы, шлёпанцы, сланцы, святцы). Like Счёты, except in 0 cells: .
    declension!("сенцы", 19, "~ы", [gender: Masculine], like Счёты;
    stem noun_pl: "~- like агнец";
    ),
    // Меринос: 18 words (квакер, чернорубашечник, виолончелист, солитер, кастрат, бахмат). Like Член, except in 0 cells: .
    declension!("меринос", 18, "~", [gender: Masculine], like Член as ACC_AS_NOM;
    ),
    // Боёк: 18 words (паёк, чаёк, ручеёк, кофеёк, боёк, буёк). Like Аниец, except in 3 cells: nom sg, nom pl, gen_ pl.
    declension!("боёк", 18, "~ё~", [gender: Masculine], like Аниец;
    stem noun_sg: "~й~- like сходство";
    nom: "~ё~", "~й~и";
    gen_: _, "~й~ов";
    ),
    // Детище: 17 words (чудовище, чудище, детище, посмешище, уёбище, страшилище). Like Ложе, except in 1 cell: acc sg.
    declension!("детище", 17, "~е", [animacy: Animate], like Ложе;
    acc: "~е", _;
    ),
    // Колёсико: 16 words (колёсико, древко, очко, брюшко, личико, пивко). Like Дух, except in 1 cell: nom sg.
    declension!("колёсико", 16, "~о", like Дух as LOC_AS_PART;
    nom: "~о", _;
    ),
    // Кокцинеллида: 16 words (ебала, венецианка, ламиина, иссида, кокцинеллида, алеохарина). Like Полнота, except in 0 cells: .
    declension!("кокцинеллида", 16, "~а", [animacy: Animate], like Полнота;
    stem noun_pl: "~- like кокцинеллида";
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
    // Бойня: 15 words (кофейня, бойня, водогрейня, воскобойня, двойня, маслобойня). Like Чайка, except in 3 cells: dat pl, ins pl, prep pl.
    declension!("бойня", 15, "~й~я", [gender: Feminine, animacy: Inanimate], like Чайка;
    stem noun_nom_sg: "~й~- like химия";
    stem noun_sg: "~й~- like баня";
    dat: _, "~й~ям";
    ins: _, "~й~ями";
    prep: _, "~й~ях";
    ),
    // Побои: 15 words (обои, побои, помои, прелиминарии, покои, фотообои). Like Ясли, except in 1 cell: gen_ pl.
    declension!("побои", 15, "~и", [gender: Masculine], like Ясли;
    gen_: _, "~ев";
    ),
    // Перо: 14 words (дерево, перо, крыло, сено, авиазвено, звено). Like Клин, except in 1 cell: nom sg.
    declension!("перо", 14, "~о", [gender: Neuter, animacy: Inanimate], like Клин as LOC_AS_PART;
    nom: "~о", _;
    ),
    // Губоцветные: 14 words (вересковые, водяниковые, верескоцветные, буковые, букоцветные, мальпигиевые). Like Лососёвые, except in 5 cells: nom pl, gen_ sg, dat sg, ins sg, prep sg.
    declension!("губоцветные", 14, "~е", [gender: Feminine, animacy: Inanimate], like Лососёвые as PL_AS_SG;
    nom: _, derived;
    gen_: "~х", _;
    dat: "~м", _;
    ins: "~ми", _;
    prep: "~х", _;
    ),
    // Перешеек: 14 words (боец, перешеек, хуец, закраек, зашеек, опоек). Like Боёк, except in 1 cell: nom sg.
    declension!("перешеек", 14, "~е~", [gender: Masculine], like Боёк;
    nom: "~е~", _;
    ),
    // Сердце: 13 words (сердце, деревце, полотенце, блюдце, оконце, волоконце). Like Тельце, except in 1 cell: gen_ pl.
    declension!("сердце", 13, "~~е", [gender: Neuter, animacy: Inanimate], like Тельце;
    gen_: _, "~е~";
    ),
    // Гнездо: 13 words (озеро, колесо, село, чело, решето, гнездо). Like Делёж, except in 1 cell: nom sg.
    declension!("гнездо", 13, "~е~о", [gender: Neuter, animacy: Inanimate], like Делёж;
    stem noun_pl: "~ё~- like сходство";
    nom: "~е~о", _;
    ),
    // Котёночек: 13 words (цыплёночек, котёночек, жеребёночек, козлёночек, утёночек, ягнёночек).
    declension!("котёночек", 13, "~ёноче~", [gender: Masculine];
    stem noun_sg: "~ёноч~- like сходство";
    nom: "~ёноче~", "~ят~и";
    gen_: _, "~ято~";
    dat: _, "~ят~ам";
    ins: _, "~ят~ами";
    prep: _, "~ят~ах";
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
    stem noun_sg: "~ь~- like агнец";
    nom: "~ь~е", _;
    ),
    // Мощи: 11 words (щи, клещи, мощи, беруши, марши, кириши). Like Счёты, except in 1 cell: nom sg.
    declension!("мощи", 11, "~и", like Счёты;
    stem noun_pl: "~- like товарищ";
    nom: "~и", _;
    ),
    // Валенок: 11 words (ботинок, чулок, турок, валенок, глазок, зубок). Like Щиток, except in 1 cell: gen_ pl.
    declension!("валенок", 11, "~о~", [gender: Masculine], like Щиток as LOC_AS_PART;
    gen_: _, "~о~";
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
    stem noun_pl: "~- like скопление";
    nom: "~я", _;
    ),
    // Племя: 10 words (имя, время, знамя, вымя, пламя, племя).
    declension!("племя", 10, "~я", [gender: Neuter, animacy: Inanimate];
    nom: "~я", "~ена";
    gen_: "~ени", "~ён";
    dat: "~ени", "~енам";
    ins: "~енем", "~енами";
    prep: "~ени", "~енах";
    ),
    // Хлопья: 10 words (хлопья, лохмотья, объедья, брусья, коренья, стручья). Like Прения, except in 0 cells: .
    declension!("хлопья", 10, "~я", [animacy: Inanimate], like Прения;
    stem noun_pl: "~- like устье";
    ),
    // Кукла: 9 words (мгла, кукла, мошна, княжна, свёкла, свекла). Like Гривна, except in 1 cell: gen_ pl.
    declension!("кукла", 9, "~~а", like Гривна;
    gen_: _, "~о~";
    ),
    // Череда: 9 words (щека, середа, железа, слеза, пелена, череда). Like Звезда, except in 4 cells: nom pl, dat pl, ins pl, prep pl.
    declension!("череда", 9, "~е~а", [gender: Feminine], like Звезда;
    nom: _, "~е~ы";
    dat: _, "~е~ам";
    ins: _, "~е~ами";
    prep: _, "~е~ах";
    ),
    // Плечо: 8 words (яблоко, молоко, млеко, веко, плечо, брюхо). Like Сходство, except in 1 cell: nom pl.
    declension!("плечо", 8, "~о", [animacy: Inanimate], like Сходство;
    nom: _, "~и";
    ),
    // Горе: 8 words (море, горе, поле, перекати-поле, муде, биополе). Like Устье, except in 1 cell: gen_ pl.
    declension!("горе", 8, "~е", [gender: Neuter, animacy: Inanimate], like Устье;
    gen_: _, "~ей";
    ),
    // Герц: 8 words (герц, гигагерц, килогерц, мегагерц, миллигерц, кольбац). Like Раз, except in 0 cells: .
    declension!("герц", 8, "~", [gender: Masculine, animacy: Inanimate], like Раз;
    stem noun_sg: "~- like агнец";
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
    // Семя: 7 words (бремя, темя, семя, стремя, беремя, льносемя). Like Племя, except in 1 cell: gen_ pl.
    declension!("семя", 7, "~я", [gender: Neuter, animacy: Inanimate], like Племя;
    gen_: _, "~ян";
    ),
    // Стекло: 7 words (стекло, бронестекло, стегно, пекло, наностекло, оргстекло). Like Ребро, except in 1 cell: gen_ pl.
    declension!("стекло", 7, "~е~~о", [gender: Neuter, animacy: Inanimate], like Ребро;
    gen_: _, "~ё~о~";
    ),
    // Заём: 7 words (заём, наём, поднаём, перенаём, госзаём, сберзаём). Like Боёк, except in 0 cells: .
    declension!("заём", 7, "~ё~", [gender: Masculine, animacy: Inanimate], like Боёк;
    stem noun_pl: "~й~- like приём";
    ),
    // Весна: 7 words (десна, весна, метла, ветла, гидрометла, блесна). Like Ребро, except in 1 cell: nom pl.
    declension!("весна", 7, "~е~~а", [gender: Feminine, animacy: Inanimate], like Ребро as NOM_AS_GEN;
    stem noun_nom_sg: "~е~~- like полнота";
    stem noun_sg: "~е~~- like полнота";
    nom: _, "~ё~~ы";
    ),
    // Осётр: 7 words (ксёндз, чёлн, осётр, черёд, перёд, клёст). Like Делёж, except in 0 cells: .
    declension!("осётр", 7, "~ё~", [gender: Masculine], like Делёж;
    stem noun_pl: "~е~- like приём";
    ),
    // Полгода: 7 words (полчаса, полгода, полметра, полмиллиона, полкилометра, поллитра). Like Поллитра, except in 1 cell: nom sg.
    declension!("полгода", 7, "~~а", [animacy: Inanimate], like Поллитра;
    nom: "~~а", _;
    ),
    // Дитё: 7 words (мумиё, дитё, хулиганьё, гадьё, мудачьё, татарьё). Like Червь, except in 1 cell: nom pl.
    declension!("дитё", 7, "~ё", like Червь;
    stem noun_nom_sg: "~ё- like скопление";
    nom: _, "~я";
    ),
    // Полигры: 7 words (пол-игры, пол-избы, пол-луковицы, пол-юрты, пол-ягоды, пол-луны). Like Поллитра, except in 5 cells: nom sg, gen_ sg, gen_ pl, dat sg, ins sg.
    declension!("пол-игры", 7, "~-~ы", like Поллитра;
    nom: "~-~ы", _;
    gen_: "~у~ы", "~у~";
    dat: "~у~е", _;
    ins: "~у~ой", _;
    ),
    // Любовь: 7 words (церковь, рожь, вошь, ложь, любовь, нелюбовь). Like Лапоть, except in 4 cells: gen_ sg, dat sg, ins sg, prep sg.
    declension!("любовь", 7, "~о~ь", [gender: Feminine], like Лапоть;
    gen_: "~~и", _;
    dat: "~~и", _;
    ins: "~о~ью", _;
    prep: "~~и", _;
    ),
    // Полиюля: 7 words (пол-апреля, пол-имения, пол-июля, пол-июня, пол-лагеря, пол-октября).
    declension!("пол-июля", 7, "~-~я";
    stem noun_pl: "~у~- like склонность";
    stem noun_sg: "~у~- like хлопанье";
    nom: "~-~я", _;
    ),
    // Млекопитающее: 6 words (млекопитающее, окружающее, обезболивающее, иглокожее, кишечнодышащее, простейшее). Like Домашние, except in 0 cells: .
    declension!("млекопитающее", 6, "~ее", [gender: Neuter, animacy: Animate], like Домашние;
    ),
    // Татарин: 6 words (болгарин, татарин, хазарин, угрин, долганин, хариджанин). Like Господин, except in 0 cells: .
    declension!("татарин", 6, "~ин", [gender: Masculine, animacy: Animate], like Господин;
    stem noun_pl: "~- like полнота";
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
    // Поллета: 6 words (пол-лета, пол-лица, пол-лота, пол-окна, пол-училища, пол-ящика). Like Поллитра, except in 0 cells: .
    declension!("пол-лета", 6, "~-~а", like Поллитра;
    stem noun_pl: "~у~- like сходство";
    ),
    // Юноша: 5 words (чукча, раджа, обжа, корча, магараджа). Like Выдача, except in 0 cells: .
    declension!("юноша", 5, "~а", like Выдача;
    stem noun_pl: "~- like товарищ";
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
    // Хозяин: 5 words (хозяин, домохозяин, квартирохозяин, сохозяин, судохозяин). Like Господин, except in 0 cells: .
    declension!("хозяин", 5, "~ин", [gender: Masculine, animacy: Animate], like Господин;
    stem noun_pl: "~ев- like сходство";
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
    // Полосени: 5 words (пол-ёмкости, пол-линии, пол-области, пол-епархии, пол-осени). Like Полиюля, except in 1 cell: nom sg.
    declension!("пол-осени", 5, "~-~и", like Полиюля;
    stem noun_sg: "~у~- like достопримечательность";
    nom: "~-~и", _;
    ),
    // Небо: 4 words (чудо, небо, древо, худо). Like Сходство, except in 0 cells: .
    declension!("небо", 4, "~о", [gender: Neuter, animacy: Inanimate], like Сходство;
    stem noun_pl: "~ес- like сходство";
    ),
    // Судно: 4 words (судно, агитсудно, авиасудно, фальшдно). Like Бега, except in 1 cell: nom sg.
    declension!("судно", 4, "~но", [gender: Neuter, animacy: Inanimate], like Бега;
    stem noun_sg: "~н- like сходство";
    nom: "~но", _;
    ),
    // Улей: 4 words (улей, жеребей, чирей, ирей). Like Репей, except in 1 cell: gen_ pl.
    declension!("улей", 4, "~ей", [gender: Masculine, animacy: Inanimate], like Репей;
    stem noun_sg: "~ь- like хлопанье";
    gen_: _, "~ьев";
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
    // Пресмыкающееся: 4 words (пресмыкающееся, случившееся, сбывшееся, несбывшееся). Like Учащийся, except in 0 cells: .
    declension!("пресмыкающееся", 4, "~ее~", [gender: Neuter], like Учащийся;
    stem noun_nom_sg: "~ее~- like скопление";
    ),
    // Остриё: 4 words (остриё, бытиё, востриё, лезвеё). Like Дитё, except in 1 cell: acc sg.
    declension!("остриё", 4, "~ё", [gender: Neuter, animacy: Inanimate], like Дитё;
    stem noun_pl: "~- like остриё";
    acc: derived, _;
    ),
    // Ничья: 4 words (ничья, третья, девичья, моя). Like Съезжая, except in 1 cell: nom pl.
    declension!("ничья", 4, "~я", [gender: Feminine], like Съезжая;
    stem noun_nom_sg: "~- like химия";
    nom: _, "~и";
    ),
    // Яйки: 4 words (обойки, валуйки, яйки, прощайки). Like Чайка, except in 4 cells: gen_ sg, dat sg, ins sg, prep sg.
    declension!("яйки", 4, "~й~и", [animacy: Inanimate], like Чайка as SG_AS_PL;
    gen_: derived, _;
    dat: derived, _;
    ins: derived, _;
    prep: derived, _;
    ),
    // Сынуля: 4 words (сынуля, саня, меля, игруля). Like Баня, except in 0 cells: .
    declension!("сынуля", 4, "~я", [gender: Masculine, animacy: Animate], like Баня;
    stem noun_pl: "~- like сынуля";
    ),
    // Карло: 4 words (ракло, шарыпово, запевало, карло). Like Сходство, except in 0 cells: .
    declension!("карло", 4, "~о", like Сходство;
    stem noun_pl: "~- like полнота";
    ),
    // Дитятко: 4 words (дитятко, старичонко, золотко, мужичонко). Like Сиверко, except in 1 cell: acc sg.
    declension!("дитятко", 4, "~~о", [animacy: Animate], like Сиверко;
    acc: "~~о", _;
    ),
    // Тамада: 4 words (тамада, фавела, яга, нигга). Like Полнота, except in 1 cell: acc pl.
    declension!("тамада", 4, "~а", like Полнота;
    acc: _, "~";
    ),
    // Усадьба: 3 words (тьма, тюрьма, потьма). Like Люлька, except in 1 cell: nom pl.
    declension!("усадьба", 3, "~ь~а", [gender: Feminine, animacy: Inanimate], like Люлька;
    stem noun_sg: "~ь~- like полнота";
    nom: _, "~ь~ы";
    ),
    // Дичь: 3 words (мышь, дичь, водомышь). Like Помочь, except in 1 cell: acc sg.
    declension!("дичь", 3, "~ь", [gender: Feminine, animacy: Animate], like Помочь as LOC_AS_PART;
    acc: "~ь", _;
    ),
    // Глаз: 3 words (глаз, телеглаз, киноглаз). Like Сходство, except in 1 cell: nom sg.
    declension!("глаз", 3, "~", [gender: Masculine, animacy: Inanimate], like Сходство;
    nom: "~", _;
    ),
    // Земля: 3 words (земля, петля, полупетля). Like Деревня, except in 0 cells: .
    declension!("земля", 3, "~~я", [gender: Feminine, animacy: Inanimate], like Деревня;
    stem noun_sg: "~~- like заря";
    ),
    // Дочь: 3 words (дочь, мать, яжмать).
    declension!("дочь", 3, "~ь", [gender: Feminine, animacy: Animate];
    stem noun_nom_sg: "~ь- like скопление";
    stem noun_pl: "~ер- like склонность";
    stem noun_sg: "~ер- like достопримечательность";
    ),
    // Путь: 3 words (путь, автопуть, севморпуть). Like Склонность, except in 1 cell: ins sg.
    declension!("путь", 3, "~ь", [gender: Masculine, animacy: Inanimate], like Склонность as LOC_AS_PART;
    ins: "~ём", _;
    ),
    // Лёд: 3 words (лёд, лён, снеголёд). Like Гусёк, except in 1 cell: noun_second_case loc.
    declension!("лёд", 3, "~ё~", [gender: Masculine, animacy: Inanimate], like Гусёк;
    stem noun_pl: "~ь~- like приём";
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
    // Зять: 3 words (зять, деверь, князь). Like Медведь, except in 0 cells: .
    declension!("зять", 3, "~ь", [gender: Masculine, animacy: Animate], like Медведь;
    stem noun_pl: "~- like жнивьё";
    ),
    // Кто: 3 words (кто, кое-кто, никто). Like Зелёный, except in 4 cells: nom sg, nom pl, ins sg, ins pl.
    declension!("кто", 3, "~то", [gender: Masculine], like Зелёный as PL_AS_SG;
    nom: "~то", "~то";
    ins: "~ем", "~ем";
    ),
    // Дрянь: 3 words (смерть, дрянь, мразь). Like Склонность, except in 0 cells: .
    declension!("дрянь", 3, "~ь", [gender: Feminine, animacy: Inanimate], like Склонность as LOC_AS_PART;
    stem noun_pl: "~- like дрянь";
    ),
    // Что: 3 words (что, ничто, кое-что). Like Кто, except in 4 cells: gen_ sg, gen_ pl, dat sg, dat pl.
    declension!("что", 3, "~то", [gender: Neuter], like Кто as PREP_AS_ACC;
    gen_: "~его", "~его";
    dat: "~ему", "~ему";
    ),
    // Епитимья: 3 words (ектенья, паремья, епитимья). Like Сватья, except in 0 cells: .
    declension!("епитимья", 3, "~ья", [gender: Feminine, animacy: Inanimate], like Сватья;
    stem noun_sg: "~ь- like заря";
    ),
    // Дверца: 3 words (дверца, шпорца, мадерца). Like Гривна, except in 0 cells: .
    declension!("дверца", 3, "~~а", [gender: Feminine, animacy: Inanimate], like Гривна;
    stem noun_sg: "~~- like шоколадница";
    ),
    // Солнышко: 3 words (солнышко, нещечко, студентишко). Like Молочко, except in 1 cell: acc sg.
    declension!("солнышко", 3, "~~о", [animacy: Animate], like Молочко;
    acc: "~~о", _;
    ),
    // Получеловек: 3 words (получеловек, обезьяночеловек, первочеловек).
    declension!("получеловек", 3, "~че~овек", [gender: Masculine, animacy: Animate];
    stem noun_pl: "~~юд- like получеловек";
    stem noun_sg: "~че~овек- like сходство";
    nom: "~че~овек", _;
    ),
    // Морячка: 3 words (морячка, рыбачка, тестюшка). Like Дорожка, except in 1 cell: acc pl.
    declension!("морячка", 3, "~~а", [animacy: Animate], like Дорожка;
    acc: _, "~~и";
    ),
    // Бабай: 3 words (бабай, бабий, бий). Like Змей, except in 1 cell: prep sg.
    declension!("бабай", 3, "~й", [gender: Masculine, animacy: Animate], like Змей as ACC_AS_NOM;
    prep: "~и|~е", _;
    ),
    // Поллеса: 3 words (пол-леса, пол-округа, пол-острова). Like Поллитра, except in 1 cell: nom pl.
    declension!("пол-леса", 3, "~-~а", like Поллитра;
    nom: _, "~у~а";
    ),
    // Полюбки: 3 words (пол-ложки, пол-юбки, пол-осьмухи).
    declension!("пол-юбки", 3, "~-~~и";
    stem noun_sg: "~у~~- like дрязги";
    nom: "~-~~и", "~у~~и";
    gen_: _, "~у~о~";
    dat: _, "~у~~ам";
    ins: _, "~у~~ами";
    prep: _, "~у~~ах";
    ),
    // Средостение: 3 words (средостение, благоусмотренье, талдыченье). Like Устье, except in 0 cells: .
    declension!("средостение", 3, "~е", [gender: Neuter, animacy: Inanimate], like Устье;
    stem noun_pl: "~- like скопление";
    ),
    // Оладий: 1 words (жеребий). Like Улей, except in 1 cell: nom sg.
    declension!("оладий", 1, "~ий", like Улей;
    nom: "~ий", _;
    ),
    // Электропроводимость: 1 words (электропроводимость).
    declension!("электропроводимость", 1, "~им~ь";
    stem noun_pl: "~н~- like склонность";
    stem noun_sg: "~н~- like достопримечательность";
    nom: "~н~ь", _;
    ),
    // Обрядник: 1 words (обрядник).
    declension!("обрядник", 1, "обряд~";
    nom: "началь~", "началь~и";
    gen_: "началь~а", "началь~ов";
    dat: "началь~у", "началь~ам";
    ins: "началь~ом", "началь~ами";
    prep: "началь~е", "началь~ах";
    ),
    // Вольтметр: 1 words (вольтметр).
    declension!("вольтметр", 1, "~~";
    stem noun_pl: "~р~- like приём";
    stem noun_sg: "~р~- like сходство";
    nom: "~р~", _;
    ),
    // Гармонщик: 1 words (гармонщик).
    declension!("гармонщик", 1, "~щ~к";
    stem noun_pl: "~~ст- like приём";
    stem noun_sg: "~~ст- like сходство";
    nom: "~~ст", _;
    ),
    // Протечка: 1 words (протечка).
    declension!("протечка", 1, "про~~а";
    nom: "у~~а", "у~~и";
    gen_: "у~~и", "у~е~";
    dat: "у~~е", "у~~ам";
    acc: "у~~у", _;
    ins: "у~~ой|у~~ою", "у~~ами";
    prep: "у~~е", "у~~ах";
    ),
    // Паяльщик: 1 words (паяльщик).
    declension!("паяльщик", 1, "~к";
    stem noun_nom_sg: "~ц- like полнота";
    stem noun_pl: "~ц- like полнота";
    stem noun_sg: "~ц- like шоколадница";
    ),
    // Полешко: 1 words (полешко).
    declension!("полешко", 1, "~шко";
    stem noun_sg: "~нц- like агнец";
    nom: "~нце", "~нца";
    gen_: _, "~нец";
    dat: _, "~нцам";
    ins: _, "~нцами";
    prep: _, "~нцах";
    ),
    // Даунсайзинг: 1 words (даунсайзинг).
    declension!("даунсайзинг", 1, "даунсайз~г";
    stem noun_pl: "~кфляци- like даунсайзинг";
    stem noun_sg: "~кфляци- like даунсайзинг";
    nom: "шр~кфляция", _;
    acc: "шр~кфляцию", _;
    ),
    // Копрофил: 1 words (копрофил).
    declension!("копрофил", 1, "~";
    stem noun_nom_sg: "~и- like химия";
    nom: _, "~ии";
    gen_: "~ии", "~ий";
    dat: "~ии", "~иям";
    ins: "~ией|~иею", "~иями";
    prep: "~ии", "~иях";
    ),
    // Полудурье: 1 words (полудурье). Like Устье, except in 0 cells: .
    declension!("полудурье", 1, "~ье", like Устье;
    ),
    // Проводины: 1 words (проводины). Like Счёты, except in 1 cell: nom sg.
    declension!("проводины", 1, "~ины", like Счёты;
    nom: "~ины", _;
    ),
    // Ёко: 1 words (ёко). Like Сколько, except in 3 cells: gen_ pl, dat pl, ins pl.
    declension!("ёко", 1, "~", like Сколько as PREP_AS_NOM;
    stem noun_sg: "~о- like какао";
    gen_: _, "~о";
    dat: _, "~о";
    ins: _, "~о";
    ),
    // Амблинойя: 1 words (амблинойя). Like Маракуйя, except in 0 cells: .
    declension!("амблинойя", 1, "~йя", like Маракуйя;
    ),
    // Разговень: 1 words (разговень). Like Делёж, except in 1 cell: nom sg.
    declension!("разговень", 1, "~а~ь", like Делёж;
    stem noun_pl: "~е~- like склонность";
    stem noun_sg: "~е~- like хлопанье";
    nom: "~е~ь", _;
    ),
    // Супремасизм: 1 words (супремасизм).
    declension!("супремасизм", 1, "~с~";
    stem noun_pl: "~т~- like приём";
    stem noun_sg: "~т~- like сходство";
    nom: "~т~", _;
    ),
    // Воздухан: 1 words (воздухан).
    declension!("воздухан", 1, "~з~";
    stem noun_pl: "~х~- like приём";
    stem noun_sg: "~х~- like сходство";
    nom: "~х~", _;
    ),
];
