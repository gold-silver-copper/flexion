//! The conjugations, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first. A `%`
//! is the first letter of the stem piece after it (reduplication: `%ε~κα`),
//! as the copy laws in `laws.rs` change it.

use flexion::Paradigm;

/// Writes one conjugation: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (`[gender: Neuter]`, if
/// they share them), the conjugation it is like (if any), the stem slots
/// it states with a stem and an ending table (`stem aorist: "~σ- like λυω"`:
/// the slot's cells are `λυω`'s table's patterns with `~σ` for their `~`; see
/// `stems.rs`), then a row of patterns per line (`derived`: the form comes
/// from another cell's, see `relations.rs`; `_`, or a row left out: the
/// form is its stem slot's, or the conjugation's it is like, or, in a conjugation like
/// no other, derived):
/// `infinitive`
/// `non_past_1` (sg, pl)
/// `non_past_2` (sg, pl)
/// `non_past_3` (sg, pl)
/// `past` (m, f, n, pl)
/// `imperative` (sg, pl)
/// `participle` (pres_act, past_act, pres_pass, past_pass)
/// `gerund` (pres, past)
macro_rules! conjugation {
    ($name:literal, $words:literal, $lemma:literal $(, [$($f:ident: $v:ident),*])? $(, like $parent:ident $(as $view:ident)?)?;
     $(stem infinitive: $s0:tt;)?
     $(stem non_past: $s1:tt;)?
     $(stem past: $s2:tt;)?
     $(infinitive: $c0:tt;)?
     $(non_past_1: $c1:tt, $c2:tt;)?
     $(non_past_2: $c3:tt, $c4:tt;)?
     $(non_past_3: $c5:tt, $c6:tt;)?
     $(past: $c7:tt, $c8:tt, $c9:tt, $c10:tt;)?
     $(imperative: $c11:tt, $c12:tt;)?
     $(participle: $c13:tt, $c14:tt, $c15:tt, $c16:tt;)?
     $(gerund: $c17:tt, $c18:tt;)?
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
                $(flexion::cell!($c14),)? "\t",
                $(flexion::cell!($c15),)? "\t",
                $(flexion::cell!($c16),)? "\t",
                $(flexion::cell!($c17),)? "\t",
                $(flexion::cell!($c18),)? "\t",
            )),
            stems: ::std::borrow::Cow::Borrowed(concat!(
                $(flexion::cell!($s0),)? "\t",
                $(flexion::cell!($s1),)? "\t",
                $(flexion::cell!($s2),)? "\t",
            )),
        }
    };
}

/// The conjugation a conjugation is like, by its name.
macro_rules! like {
    () => {
        None
    };
    ($parent:ident) => {
        Some(Conjugation::$parent as u16)
    };
}

/// The cells a conjugation reads in the one it is like, by the view's name (see
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
    (aspect: $v:ident) => {
        (0, crate::Aspect::$v as u8)
    };
}

/// A view: per cell (in the cells' order), the cell of the conjugation it is
/// like that it reads (`act_as_pass`: one value read as another).
const ACT_AS_PASS: [u16; 19] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 15, 14, 15, 16, 17, 18,
];

/// A view: per cell (in the cells' order), the cell of the conjugation it is
/// like that it reads (`pres_as_past`: one value read as another).
const PRES_AS_PAST: [u16; 19] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 16, 14, 15, 16, 17, 18,
];

/// A view: per cell (in the cells' order), the cell of the conjugation it is
/// like that it reads (`past_as_pres`: one value read as another).
const PAST_AS_PRES: [u16; 19] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 15, 17, 18,
];

/// Every conjugation learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Conjugation {
    /// Like `играть`: 11,638 words.
    Играть,
    /// Like `набираться`: 8,293 words.
    Набираться,
    /// Like `вибрировать`: 4,162 words.
    Вибрировать,
    /// Like `стыковаться`: 2,209 words.
    Стыковаться,
    /// Like `ударить`: 1,983 words.
    Ударить,
    /// Like `палить`: 1,330 words.
    Палить,
    /// Like `числиться`: 1,171 words.
    Числиться,
    /// Like `раздуть`: 925 words.
    Раздуть,
    /// Like `рвануть`: 685 words.
    Рвануть,
    /// Like `выделить`: 604 words.
    Выделить,
    /// Like `усилиться`: 566 words.
    Усилиться,
    /// Like `тянуть`: 543 words.
    Тянуть,
    /// Like `обнажиться`: 501 words.
    Обнажиться,
    /// Like `любиться`: 454 words.
    Любиться,
    /// Like `обернуться`: 392 words.
    Обернуться,
    /// Like `восстановить`: 366 words.
    Восстановить,
    /// Like `обрушиться`: 343 words.
    Обрушиться,
    /// Like `разводить`: 327 words.
    Разводить,
    /// Like `расправиться`: 296 words.
    Расправиться,
    /// Like `хватить`: 296 words.
    Хватить,
    /// Like `представить`: 290 words.
    Представить,
    /// Like `долбить`: 287 words.
    Долбить,
    /// Like `расходиться`: 283 words.
    Расходиться,
    /// Like `закатиться`: 232 words.
    Закатиться,
    /// Like `тянуться`: 230 words.
    Тянуться,
    /// Like `залипнуть`: 203 words.
    Залипнуть,
    /// Like `стучать`: 192 words.
    Стучать,
    /// Like `защитить`: 191 words.
    Защитить,
    /// Like `сносить`: 186 words.
    Сносить,
    /// Like `отметить`: 165 words.
    Отметить,
    /// Like `распуститься`: 163 words.
    Распуститься,
    /// Like `носиться`: 138 words.
    Носиться,
    /// Like `забросить`: 125 words.
    Забросить,
    /// Like `встретиться`: 120 words.
    Встретиться,
    /// Like `оказать`: 119 words.
    Оказать,
    /// Like `засосать`: 118 words.
    Засосать,
    /// Like `топтать`: 117 words.
    Топтать,
    /// Like `развеять`: 113 words.
    Развеять,
    /// Like `сычевать`: 113 words.
    Сычевать,
    /// Like `перестроиться`: 110 words.
    Перестроиться,
    /// Like `держаться`: 108 words.
    Держаться,
    /// Like `сорваться`: 103 words.
    Сорваться,
    /// Like `трафить`: 103 words.
    Трафить,
    /// Like `прошипеть`: 103 words.
    Прошипеть,
    /// Like `вывозить`: 102 words.
    Вывозить,
    /// Like `наладиться`: 95 words.
    Наладиться,
    /// Like `переброситься`: 93 words.
    Переброситься,
    /// Like `елозить`: 90 words.
    Елозить,
    /// Like `подкинуть`: 88 words.
    Подкинуть,
    /// Like `открыть`: 83 words.
    Открыть,
    /// Like `наладить`: 83 words.
    Наладить,
    /// Like `выразиться`: 81 words.
    Выразиться,
    /// Like `стать`: 81 words.
    Стать,
    /// Like `зарядить`: 79 words.
    Зарядить,
    /// Like `отдавать`: 77 words.
    Отдавать,
    /// Like `развеяться`: 73 words.
    Развеяться,
    /// Like `коптить`: 72 words.
    Коптить,
    /// Like `стушеваться`: 71 words.
    Стушеваться,
    /// Like `забить`: 68 words.
    Забить,
    /// Like `сблизиться`: 66 words.
    Сблизиться,
    /// Like `бухтеть`: 65 words.
    Бухтеть,
    /// Like `стоить`: 62 words.
    Стоить,
    /// Like `зевать`: 61 words.
    Зевать,
    /// Like `обтечь`: 60 words.
    Обтечь,
    /// Like `возбудить`: 59 words.
    Возбудить,
    /// Like `отвоевать`: 58 words.
    Отвоевать,
    /// Like `сидеть`: 57 words.
    Сидеть,
    /// Like `перевернуть`: 57 words.
    Перевернуть,
    /// Like `виться`: 57 words.
    Виться,
    /// Like `сунуться`: 57 words.
    Сунуться,
    /// Like `колоть`: 55 words.
    Колоть,
    /// Like `раскрыться`: 55 words.
    Раскрыться,
    /// Like `заглохнуть`: 54 words.
    Заглохнуть,
    /// Like `влечься`: 53 words.
    Влечься,
    /// Like `чесаться`: 53 words.
    Чесаться,
    /// Like `насытить`: 53 words.
    Насытить,
    /// Like `отказаться`: 52 words.
    Отказаться,
    /// Like `пасть`: 52 words.
    Пасть,
    /// Like `отдаваться`: 51 words.
    Отдаваться,
    /// Like `идти`: 51 words.
    Идти,
    /// Like `глядеться`: 48 words.
    Глядеться,
    /// Like `развести`: 48 words.
    Развести,
    /// Like `разбить`: 46 words.
    Разбить,
    /// Like `клевать`: 45 words.
    Клевать,
    /// Like `снести`: 44 words.
    Снести,
    /// Like `драть`: 43 words.
    Драть,
    /// Like `колоться`: 42 words.
    Колоться,
    /// Like `топтаться`: 42 words.
    Топтаться,
    /// Like `пыхать`: 42 words.
    Пыхать,
    /// Like `сдать`: 41 words.
    Сдать,
    /// Like `выписать`: 40 words.
    Выписать,
    /// Like `отобразить`: 39 words.
    Отобразить,
    /// Like `сбиться`: 39 words.
    Сбиться,
    /// Like `обволочь`: 39 words.
    Обволочь,
    /// Like `служить`: 37 words.
    Служить,
    /// Like `обратиться`: 37 words.
    Обратиться,
    /// Like `гореть`: 37 words.
    Гореть,
    /// Like `загореться`: 36 words.
    Загореться,
    /// Like `щипать`: 35 words.
    Щипать,
    /// Like `трепаться`: 34 words.
    Трепаться,
    /// Like `сплести`: 34 words.
    Сплести,
    /// Like `расчесать`: 33 words.
    Расчесать,
    /// Like `станцевать`: 33 words.
    Станцевать,
    /// Like `резаться`: 32 words.
    Резаться,
    /// Like `отвоеваться`: 32 words.
    Отвоеваться,
    /// Like `трястись`: 31 words.
    Трястись,
    /// Like `обыскать`: 31 words.
    Обыскать,
    /// Like `сковать`: 30 words.
    Сковать,
    /// Like `искусить`: 30 words.
    Искусить,
    /// Like `распасться`: 29 words.
    Распасться,
    /// Like `отпустить`: 27 words.
    Отпустить,
    /// Like `набраться`: 27 words.
    Набраться,
    /// Like `грести`: 27 words.
    Грести,
    /// Like `завестись`: 27 words.
    Завестись,
    /// Like `зайти`: 27 words.
    Зайти,
    /// Like `плакать`: 26 words.
    Плакать,
    /// Like `просвистеть`: 26 words.
    Просвистеть,
    /// Like `терпеться`: 26 words.
    Терпеться,
    /// Like `плеваться`: 26 words.
    Плеваться,
    /// Like `стричься`: 26 words.
    Стричься,
    /// Like `упереться`: 26 words.
    Упереться,
    /// Like `тухнуть`: 25 words.
    Тухнуть,
    /// Like `наплыть`: 25 words.
    Наплыть,
    /// Like `разлететься`: 24 words.
    Разлететься,
    /// Like `расплыться`: 24 words.
    Расплыться,
    /// Like `вторгнуться`: 24 words.
    Вторгнуться,
    /// Like `проскочить`: 23 words.
    Проскочить,
    /// Like `заесть`: 23 words.
    Заесть,
    /// Like `затереть`: 22 words.
    Затереть,
    /// Like `перепеть`: 22 words.
    Перепеть,
    /// Like `отдаться`: 21 words.
    Отдаться,
    /// Like `дорасти`: 21 words.
    Дорасти,
    /// Like `забыть`: 21 words.
    Забыть,
    /// Like `унестись`: 21 words.
    Унестись,
    /// Like `набежать`: 20 words.
    Набежать,
    /// Like `расстаться`: 20 words.
    Расстаться,
    /// Like `грызть`: 20 words.
    Грызть,
    /// Like `зашибить`: 20 words.
    Зашибить,
    /// Like `занять`: 19 words.
    Занять,
    /// Like `отнять`: 19 words.
    Отнять,
    /// Like `лечь`: 19 words.
    Лечь,
    /// Like `плескаться`: 19 words.
    Плескаться,
    /// Like `предназначить`: 18 words.
    Предназначить,
    /// Like `просмотреть`: 18 words.
    Просмотреть,
    /// Like `въесться`: 18 words.
    Въесться,
    /// Like `потчевать`: 18 words.
    Потчевать,
    /// Like `устояться`: 17 words.
    Устояться,
    /// Like `трепетать`: 17 words.
    Трепетать,
    /// Like `лезть`: 17 words.
    Лезть,
    /// Like `жать`: 17 words.
    Жать,
    /// Like `взъебать`: 17 words.
    Взъебать,
    /// Like `колыхаться`: 17 words.
    Колыхаться,
    /// Like `вгрызться`: 17 words.
    Вгрызться,
    /// Like `постричь`: 17 words.
    Постричь,
    /// Like `разобрать`: 16 words.
    Разобрать,
    /// Like `выровнять`: 16 words.
    Выровнять,
    /// Like `трепать`: 16 words.
    Трепать,
    /// Like `заволочься`: 16 words.
    Заволочься,
    /// Like `насесть`: 15 words.
    Насесть,
    /// Like `смеяться`: 15 words.
    Смеяться,
    /// Like `гнать`: 15 words.
    Гнать,
    /// Like `помять`: 15 words.
    Помять,
    /// Like `гнить`: 15 words.
    Гнить,
    /// Like `сберечь`: 15 words.
    Сберечь,
    /// Like `честь`: 15 words.
    Честь,
    /// Like `стелить`: 15 words.
    Стелить,
    /// Like `видеть`: 14 words.
    Видеть,
    /// Like `везти`: 14 words.
    Везти,
    /// Like `молоть`: 14 words.
    Молоть,
    /// Like `прискакать`: 14 words.
    Прискакать,
    /// Like `молоться`: 14 words.
    Молоться,
    /// Like `просвистать`: 14 words.
    Просвистать,
    /// Like `прожечь`: 14 words.
    Прожечь,
    /// Like `глохнуть`: 13 words.
    Глохнуть,
    /// Like `подобраться`: 13 words.
    Подобраться,
    /// Like `засечь`: 13 words.
    Засечь,
    /// Like `толочь`: 13 words.
    Толочь,
    /// Like `мяться`: 13 words.
    Мяться,
    /// Like `сплестись`: 13 words.
    Сплестись,
    /// Like `нагореть`: 13 words.
    Нагореть,
    /// Like `тыкаться`: 13 words.
    Тыкаться,
    /// Like `отгрестись`: 13 words.
    Отгрестись,
    /// Like `поехать`: 12 words.
    Поехать,
    /// Like `подняться`: 12 words.
    Подняться,
    /// Like `отослать`: 12 words.
    Отослать,
    /// Like `брить`: 12 words.
    Брить,
    /// Like `уняться`: 12 words.
    Уняться,
    /// Like `стынуть`: 12 words.
    Стынуть,
    /// Like `бриться`: 12 words.
    Бриться,
    /// Like `морщить`: 12 words.
    Морщить,
    /// Like `сослаться`: 12 words.
    Сослаться,
    /// Like `пересидеть`: 12 words.
    Пересидеть,
    /// Like `спеться`: 12 words.
    Спеться,
    /// Like `стереться`: 12 words.
    Стереться,
    /// Like `подвигнуться`: 12 words.
    Подвигнуться,
    /// Like `назвать`: 12 words.
    Назвать,
    /// Like `стелиться`: 12 words.
    Стелиться,
    /// Like `заспать`: 11 words.
    Заспать,
    /// Like `ушибиться`: 11 words.
    Ушибиться,
    /// Like `жаться`: 11 words.
    Жаться,
    /// Like `пакоститься`: 11 words.
    Пакоститься,
    /// Like `дожить`: 11 words.
    Дожить,
    /// Like `зассать`: 11 words.
    Зассать,
    /// Like `зачесться`: 11 words.
    Зачесться,
    /// Like `стоять`: 10 words.
    Стоять,
    /// Like `беречься`: 10 words.
    Беречься,
    /// Like `вырвать`: 10 words.
    Вырвать,
    /// Like `плескать`: 10 words.
    Плескать,
    /// Like `забыться`: 10 words.
    Забыться,
    /// Like `нестись`: 10 words.
    Нестись,
    /// Like `гнаться`: 10 words.
    Гнаться,
    /// Like `назваться`: 10 words.
    Назваться,
    /// Like `избороздить`: 10 words.
    Избороздить,
    /// Like `свянуть`: 10 words.
    Свянуть,
    /// Like `жеваться`: 10 words.
    Жеваться,
    /// Like `обидеться`: 10 words.
    Обидеться,
    /// Like `вырваться`: 10 words.
    Вырваться,
    /// Like `стонать`: 9 words.
    Стонать,
    /// Like `стереть`: 9 words.
    Стереть,
    /// Like `выспаться`: 9 words.
    Выспаться,
    /// Like `размять`: 9 words.
    Размять,
    /// Like `толочься`: 9 words.
    Толочься,
    /// Like `скрестись`: 9 words.
    Скрестись,
    /// Like `зажечься`: 9 words.
    Зажечься,
    /// Like `мести`: 9 words.
    Мести,
    /// Like `разойтись`: 9 words.
    Разойтись,
    /// Like `застлать`: 8 words.
    Застлать,
    /// Like `выпить`: 8 words.
    Выпить,
    /// Like `разогнать`: 8 words.
    Разогнать,
    /// Like `смяться`: 8 words.
    Смяться,
    /// Like `осмеять`: 8 words.
    Осмеять,
    /// Like `выклевать`: 8 words.
    Выклевать,
    /// Like `разреветься`: 8 words.
    Разреветься,
    /// Like `выпариться`: 8 words.
    Выпариться,
    /// Like `отсвистеться`: 8 words.
    Отсвистеться,
    /// Like `разогнаться`: 8 words.
    Разогнаться,
    /// Like `вести`: 8 words.
    Вести,
    /// Like `месть`: 8 words.
    Месть,
    /// Like `вынести`: 7 words.
    Вынести,
    /// Like `нашептать`: 7 words.
    Нашептать,
    /// Like `прятаться`: 7 words.
    Прятаться,
    /// Like `закупорить`: 7 words.
    Закупорить,
    /// Like `залгаться`: 7 words.
    Залгаться,
    /// Like `насытиться`: 7 words.
    Насытиться,
    /// Like `достичь`: 6 words.
    Достичь,
    /// Like `разжечь`: 6 words.
    Разжечь,
    /// Like `морщиться`: 6 words.
    Морщиться,
    /// Like `измыслить`: 6 words.
    Измыслить,
    /// Like `рассесться`: 6 words.
    Рассесться,
    /// Like `сжаться`: 6 words.
    Сжаться,
    /// Like `соваться`: 6 words.
    Соваться,
    /// Like `разъехаться`: 6 words.
    Разъехаться,
    /// Like `сыпаться`: 6 words.
    Сыпаться,
    /// Like `хлестать`: 6 words.
    Хлестать,
    /// Like `плестись`: 6 words.
    Плестись,
    /// Like `выпасть`: 6 words.
    Выпасть,
    /// Like `исторгнуть`: 6 words.
    Исторгнуть,
    /// Like `зачертить`: 6 words.
    Зачертить,
    /// Like `исхлестаться`: 6 words.
    Исхлестаться,
    /// Like `обидеть`: 6 words.
    Обидеть,
    /// Like `вымести`: 6 words.
    Вымести,
    /// Like `замереть`: 6 words.
    Замереть,
    /// Like `мочься`: 6 words.
    Мочься,
    /// Like `взорать`: 6 words.
    Взорать,
    /// Like `висеть`: 5 words.
    Висеть,
    /// Like `выбрать`: 5 words.
    Выбрать,
    /// Like `отжать`: 5 words.
    Отжать,
    /// Like `отстоять`: 5 words.
    Отстоять,
    /// Like `выставиться`: 5 words.
    Выставиться,
    /// Like `одолеть`: 5 words.
    Одолеть,
    /// Like `выставить`: 5 words.
    Выставить,
    /// Like `изъять`: 5 words.
    Изъять,
    /// Like `выдвинуться`: 5 words.
    Выдвинуться,
    /// Like `пробежаться`: 5 words.
    Пробежаться,
    /// Like `обжечься`: 5 words.
    Обжечься,
    /// Like `волгнуть`: 5 words.
    Волгнуть,
    /// Like `стлаться`: 5 words.
    Стлаться,
    /// Like `распоясаться`: 5 words.
    Распоясаться,
    /// Like `трепетаться`: 5 words.
    Трепетаться,
    /// Like `ссаться`: 5 words.
    Ссаться,
    /// Like `срастись`: 5 words.
    Срастись,
    /// Like `вестись`: 5 words.
    Вестись,
    /// Like `вывиться`: 5 words.
    Вывиться,
    /// Like `выползти`: 5 words.
    Выползти,
    /// Like `изобрести`: 4 words.
    Изобрести,
    /// Like `принять`: 4 words.
    Принять,
    /// Like `услышать`: 4 words.
    Услышать,
    /// Like `выпереть`: 4 words.
    Выпереть,
    /// Like `высечь`: 4 words.
    Высечь,
    /// Like `напрячь`: 4 words.
    Напрячь,
    /// Like `выдвинуть`: 4 words.
    Выдвинуть,
    /// Like `внимать`: 4 words.
    Внимать,
    /// Like `бечь`: 4 words.
    Бечь,
    /// Like `улечься`: 4 words.
    Улечься,
    /// Like `вынестись`: 4 words.
    Вынестись,
    /// Like `истереть`: 4 words.
    Истереть,
    /// Like `испить`: 4 words.
    Испить,
    /// Like `поколебать`: 4 words.
    Поколебать,
    /// Like `насочинять`: 4 words.
    Насочинять,
    /// Like `растереться`: 4 words.
    Растереться,
    /// Like `изъяться`: 4 words.
    Изъяться,
    /// Like `уеть`: 4 words.
    Уеть,
    /// Like `доебсти`: 4 words.
    Доебсти,
    /// Like `помочь`: 3 words.
    Помочь,
    /// Like `хотеть`: 3 words.
    Хотеть,
    /// Like `хотеться`: 3 words.
    Хотеться,
    /// Like `вырасти`: 3 words.
    Вырасти,
    /// Like `высвободить`: 3 words.
    Высвободить,
    /// Like `начаться`: 3 words.
    Начаться,
    /// Like `тереть`: 3 words.
    Тереть,
    /// Like `опоясать`: 3 words.
    Опоясать,
    /// Like `вянуть`: 3 words.
    Вянуть,
    /// Like `заклясть`: 3 words.
    Заклясть,
    /// Like `клясться`: 3 words.
    Клясться,
    /// Like `приняться`: 3 words.
    Приняться,
    /// Like `вывести`: 3 words.
    Вывести,
    /// Like `выбраться`: 3 words.
    Выбраться,
    /// Like `вылезть`: 3 words.
    Вылезть,
    /// Like `счесть`: 3 words.
    Счесть,
    /// Like `обмереть`: 3 words.
    Обмереть,
    /// Like `понудить`: 3 words.
    Понудить,
    /// Like `отозвать`: 3 words.
    Отозвать,
    /// Like `помутить`: 3 words.
    Помутить,
    /// Like `обглодать`: 3 words.
    Обглодать,
    /// Like `выгрести`: 3 words.
    Выгрести,
    /// Like `приобрестись`: 3 words.
    Приобрестись,
    /// Like `пролежать`: 3 words.
    Пролежать,
    /// Like `осечься`: 3 words.
    Осечься,
    /// Like `изыти`: 3 words.
    Изыти,
    /// Like `наморщиться`: 3 words.
    Наморщиться,
    /// Like `чтиться`: 3 words.
    Чтиться,
    /// Like `взбресть`: 3 words.
    Взбресть,
    /// Like `выпечься`: 3 words.
    Выпечься,
    /// Like `мястись`: 3 words.
    Мястись,
    /// Like `испиться`: 3 words.
    Испиться,
    /// Like `измождать`: 3 words.
    Измождать,
    /// Like `позаводить`: 3 words.
    Позаводить,
    /// Like `закодить`: 3 words.
    Закодить,
    /// Like `харассить`: 3 words.
    Харассить,
    /// Like `рещи`: 1 words.
    Рещи,
}

impl Conjugation {
    /// Every conjugation, in order.
    pub const ALL: [Conjugation; 340] = [
        Conjugation::Играть,
        Conjugation::Набираться,
        Conjugation::Вибрировать,
        Conjugation::Стыковаться,
        Conjugation::Ударить,
        Conjugation::Палить,
        Conjugation::Числиться,
        Conjugation::Раздуть,
        Conjugation::Рвануть,
        Conjugation::Выделить,
        Conjugation::Усилиться,
        Conjugation::Тянуть,
        Conjugation::Обнажиться,
        Conjugation::Любиться,
        Conjugation::Обернуться,
        Conjugation::Восстановить,
        Conjugation::Обрушиться,
        Conjugation::Разводить,
        Conjugation::Расправиться,
        Conjugation::Хватить,
        Conjugation::Представить,
        Conjugation::Долбить,
        Conjugation::Расходиться,
        Conjugation::Закатиться,
        Conjugation::Тянуться,
        Conjugation::Залипнуть,
        Conjugation::Стучать,
        Conjugation::Защитить,
        Conjugation::Сносить,
        Conjugation::Отметить,
        Conjugation::Распуститься,
        Conjugation::Носиться,
        Conjugation::Забросить,
        Conjugation::Встретиться,
        Conjugation::Оказать,
        Conjugation::Засосать,
        Conjugation::Топтать,
        Conjugation::Развеять,
        Conjugation::Сычевать,
        Conjugation::Перестроиться,
        Conjugation::Держаться,
        Conjugation::Сорваться,
        Conjugation::Трафить,
        Conjugation::Прошипеть,
        Conjugation::Вывозить,
        Conjugation::Наладиться,
        Conjugation::Переброситься,
        Conjugation::Елозить,
        Conjugation::Подкинуть,
        Conjugation::Открыть,
        Conjugation::Наладить,
        Conjugation::Выразиться,
        Conjugation::Стать,
        Conjugation::Зарядить,
        Conjugation::Отдавать,
        Conjugation::Развеяться,
        Conjugation::Коптить,
        Conjugation::Стушеваться,
        Conjugation::Забить,
        Conjugation::Сблизиться,
        Conjugation::Бухтеть,
        Conjugation::Стоить,
        Conjugation::Зевать,
        Conjugation::Обтечь,
        Conjugation::Возбудить,
        Conjugation::Отвоевать,
        Conjugation::Сидеть,
        Conjugation::Перевернуть,
        Conjugation::Виться,
        Conjugation::Сунуться,
        Conjugation::Колоть,
        Conjugation::Раскрыться,
        Conjugation::Заглохнуть,
        Conjugation::Влечься,
        Conjugation::Чесаться,
        Conjugation::Насытить,
        Conjugation::Отказаться,
        Conjugation::Пасть,
        Conjugation::Отдаваться,
        Conjugation::Идти,
        Conjugation::Глядеться,
        Conjugation::Развести,
        Conjugation::Разбить,
        Conjugation::Клевать,
        Conjugation::Снести,
        Conjugation::Драть,
        Conjugation::Колоться,
        Conjugation::Топтаться,
        Conjugation::Пыхать,
        Conjugation::Сдать,
        Conjugation::Выписать,
        Conjugation::Отобразить,
        Conjugation::Сбиться,
        Conjugation::Обволочь,
        Conjugation::Служить,
        Conjugation::Обратиться,
        Conjugation::Гореть,
        Conjugation::Загореться,
        Conjugation::Щипать,
        Conjugation::Трепаться,
        Conjugation::Сплести,
        Conjugation::Расчесать,
        Conjugation::Станцевать,
        Conjugation::Резаться,
        Conjugation::Отвоеваться,
        Conjugation::Трястись,
        Conjugation::Обыскать,
        Conjugation::Сковать,
        Conjugation::Искусить,
        Conjugation::Распасться,
        Conjugation::Отпустить,
        Conjugation::Набраться,
        Conjugation::Грести,
        Conjugation::Завестись,
        Conjugation::Зайти,
        Conjugation::Плакать,
        Conjugation::Просвистеть,
        Conjugation::Терпеться,
        Conjugation::Плеваться,
        Conjugation::Стричься,
        Conjugation::Упереться,
        Conjugation::Тухнуть,
        Conjugation::Наплыть,
        Conjugation::Разлететься,
        Conjugation::Расплыться,
        Conjugation::Вторгнуться,
        Conjugation::Проскочить,
        Conjugation::Заесть,
        Conjugation::Затереть,
        Conjugation::Перепеть,
        Conjugation::Отдаться,
        Conjugation::Дорасти,
        Conjugation::Забыть,
        Conjugation::Унестись,
        Conjugation::Набежать,
        Conjugation::Расстаться,
        Conjugation::Грызть,
        Conjugation::Зашибить,
        Conjugation::Занять,
        Conjugation::Отнять,
        Conjugation::Лечь,
        Conjugation::Плескаться,
        Conjugation::Предназначить,
        Conjugation::Просмотреть,
        Conjugation::Въесться,
        Conjugation::Потчевать,
        Conjugation::Устояться,
        Conjugation::Трепетать,
        Conjugation::Лезть,
        Conjugation::Жать,
        Conjugation::Взъебать,
        Conjugation::Колыхаться,
        Conjugation::Вгрызться,
        Conjugation::Постричь,
        Conjugation::Разобрать,
        Conjugation::Выровнять,
        Conjugation::Трепать,
        Conjugation::Заволочься,
        Conjugation::Насесть,
        Conjugation::Смеяться,
        Conjugation::Гнать,
        Conjugation::Помять,
        Conjugation::Гнить,
        Conjugation::Сберечь,
        Conjugation::Честь,
        Conjugation::Стелить,
        Conjugation::Видеть,
        Conjugation::Везти,
        Conjugation::Молоть,
        Conjugation::Прискакать,
        Conjugation::Молоться,
        Conjugation::Просвистать,
        Conjugation::Прожечь,
        Conjugation::Глохнуть,
        Conjugation::Подобраться,
        Conjugation::Засечь,
        Conjugation::Толочь,
        Conjugation::Мяться,
        Conjugation::Сплестись,
        Conjugation::Нагореть,
        Conjugation::Тыкаться,
        Conjugation::Отгрестись,
        Conjugation::Поехать,
        Conjugation::Подняться,
        Conjugation::Отослать,
        Conjugation::Брить,
        Conjugation::Уняться,
        Conjugation::Стынуть,
        Conjugation::Бриться,
        Conjugation::Морщить,
        Conjugation::Сослаться,
        Conjugation::Пересидеть,
        Conjugation::Спеться,
        Conjugation::Стереться,
        Conjugation::Подвигнуться,
        Conjugation::Назвать,
        Conjugation::Стелиться,
        Conjugation::Заспать,
        Conjugation::Ушибиться,
        Conjugation::Жаться,
        Conjugation::Пакоститься,
        Conjugation::Дожить,
        Conjugation::Зассать,
        Conjugation::Зачесться,
        Conjugation::Стоять,
        Conjugation::Беречься,
        Conjugation::Вырвать,
        Conjugation::Плескать,
        Conjugation::Забыться,
        Conjugation::Нестись,
        Conjugation::Гнаться,
        Conjugation::Назваться,
        Conjugation::Избороздить,
        Conjugation::Свянуть,
        Conjugation::Жеваться,
        Conjugation::Обидеться,
        Conjugation::Вырваться,
        Conjugation::Стонать,
        Conjugation::Стереть,
        Conjugation::Выспаться,
        Conjugation::Размять,
        Conjugation::Толочься,
        Conjugation::Скрестись,
        Conjugation::Зажечься,
        Conjugation::Мести,
        Conjugation::Разойтись,
        Conjugation::Застлать,
        Conjugation::Выпить,
        Conjugation::Разогнать,
        Conjugation::Смяться,
        Conjugation::Осмеять,
        Conjugation::Выклевать,
        Conjugation::Разреветься,
        Conjugation::Выпариться,
        Conjugation::Отсвистеться,
        Conjugation::Разогнаться,
        Conjugation::Вести,
        Conjugation::Месть,
        Conjugation::Вынести,
        Conjugation::Нашептать,
        Conjugation::Прятаться,
        Conjugation::Закупорить,
        Conjugation::Залгаться,
        Conjugation::Насытиться,
        Conjugation::Достичь,
        Conjugation::Разжечь,
        Conjugation::Морщиться,
        Conjugation::Измыслить,
        Conjugation::Рассесться,
        Conjugation::Сжаться,
        Conjugation::Соваться,
        Conjugation::Разъехаться,
        Conjugation::Сыпаться,
        Conjugation::Хлестать,
        Conjugation::Плестись,
        Conjugation::Выпасть,
        Conjugation::Исторгнуть,
        Conjugation::Зачертить,
        Conjugation::Исхлестаться,
        Conjugation::Обидеть,
        Conjugation::Вымести,
        Conjugation::Замереть,
        Conjugation::Мочься,
        Conjugation::Взорать,
        Conjugation::Висеть,
        Conjugation::Выбрать,
        Conjugation::Отжать,
        Conjugation::Отстоять,
        Conjugation::Выставиться,
        Conjugation::Одолеть,
        Conjugation::Выставить,
        Conjugation::Изъять,
        Conjugation::Выдвинуться,
        Conjugation::Пробежаться,
        Conjugation::Обжечься,
        Conjugation::Волгнуть,
        Conjugation::Стлаться,
        Conjugation::Распоясаться,
        Conjugation::Трепетаться,
        Conjugation::Ссаться,
        Conjugation::Срастись,
        Conjugation::Вестись,
        Conjugation::Вывиться,
        Conjugation::Выползти,
        Conjugation::Изобрести,
        Conjugation::Принять,
        Conjugation::Услышать,
        Conjugation::Выпереть,
        Conjugation::Высечь,
        Conjugation::Напрячь,
        Conjugation::Выдвинуть,
        Conjugation::Внимать,
        Conjugation::Бечь,
        Conjugation::Улечься,
        Conjugation::Вынестись,
        Conjugation::Истереть,
        Conjugation::Испить,
        Conjugation::Поколебать,
        Conjugation::Насочинять,
        Conjugation::Растереться,
        Conjugation::Изъяться,
        Conjugation::Уеть,
        Conjugation::Доебсти,
        Conjugation::Помочь,
        Conjugation::Хотеть,
        Conjugation::Хотеться,
        Conjugation::Вырасти,
        Conjugation::Высвободить,
        Conjugation::Начаться,
        Conjugation::Тереть,
        Conjugation::Опоясать,
        Conjugation::Вянуть,
        Conjugation::Заклясть,
        Conjugation::Клясться,
        Conjugation::Приняться,
        Conjugation::Вывести,
        Conjugation::Выбраться,
        Conjugation::Вылезть,
        Conjugation::Счесть,
        Conjugation::Обмереть,
        Conjugation::Понудить,
        Conjugation::Отозвать,
        Conjugation::Помутить,
        Conjugation::Обглодать,
        Conjugation::Выгрести,
        Conjugation::Приобрестись,
        Conjugation::Пролежать,
        Conjugation::Осечься,
        Conjugation::Изыти,
        Conjugation::Наморщиться,
        Conjugation::Чтиться,
        Conjugation::Взбресть,
        Conjugation::Выпечься,
        Conjugation::Мястись,
        Conjugation::Испиться,
        Conjugation::Измождать,
        Conjugation::Позаводить,
        Conjugation::Закодить,
        Conjugation::Харассить,
        Conjugation::Рещи,
    ];

    /// Its table.
    pub fn paradigm(self) -> Option<&'static Paradigm<'static>> {
        CONJUGATIONS.get(self as usize)
    }
}

impl flexion::Label for Conjugation {
    fn index(self) -> usize {
        self as usize
    }
    fn from_index(index: usize) -> Option<Self> {
        Self::ALL.get(index).copied()
    }
}

pub(super) static CONJUGATIONS: [Paradigm<'static>; 340] = [
    // Играть: 11,638 words (знать, играть, презирать, делать, уступать, скучать).
    conjugation!("играть", 11638, "~ть";
    stem non_past: "~- like играть";
    stem past: "~- like играть";
    infinitive: "~ть";
    ),
    // Набираться: 8,293 words (отпочковываться, изъясняться, облокачиваться, кончаться, являться, сморкаться).
    conjugation!("набираться", 8293, "~ть~я";
    infinitive: "~ть~я";
    non_past_1: "~ю~ь", "~ем~я";
    non_past_2: "~ешь~я", "~ете~ь";
    non_past_3: "~ет~я", "~ют~я";
    past: "~л~я", "~ла~ь", "~ло~ь", "~ли~ь";
    imperative: "~й~я", "~йте~ь";
    participle: "~ющий~я", "~вший~я", _, _;
    gerund: "~я~ь", "~вши~ь";
    ),
    // Вибрировать: 4,162 words (мульчировать, завибрировать, координировать, использовать, чувствовать, почувствовать).
    conjugation!("вибрировать", 4162, "~овать";
    stem non_past: "~у- like играть";
    stem past: "~ова- like играть";
    infinitive: "~овать";
    ),
    // Стыковаться: 2,209 words (тусоваться, использоваться, чувствоваться, почувствоваться, целоваться, абсорбироваться).
    conjugation!("стыковаться", 2209, "~овать~я";
    infinitive: "~овать~я";
    non_past_1: "~ую~ь", "~уем~я";
    non_past_2: "~уешь~я", "~уете~ь";
    non_past_3: "~ует~я", "~уют~я";
    past: "~овал~я", "~овала~ь", "~овало~ь", "~овали~ь";
    imperative: "~уй~я", "~уйте~ь";
    participle: "~ующий~я", "~овавший~я", _, _;
    gerund: "~уя~ь", "~овавши~ь";
    ),
    // Ударить: 1,983 words (жалить, ужалить, помнить, увеличить, запомнить, значить).
    conjugation!("ударить", 1983, "~ить";
    stem past: "~- like ударить";
    infinitive: "~ить";
    non_past_1: "~ю", "~им";
    non_past_2: "~ишь", "~ите";
    non_past_3: "~ит", "~ят";
    imperative: "~ь", "~ьте";
    participle: "~ящий", _, "~имый", _;
    gerund: "~я", _;
    ),
    // Палить: 1,330 words (говорить, поговорить, изменить, благодарить, винить, отменить). Like Ударить, except in 3 cells: imperative sg, imperative pl, participle past_pass.
    conjugation!("палить", 1330, "~ить", like Ударить;
    imperative: "~и", "~ите";
    participle: _, _, _, "~ённый";
    ),
    // Числиться: 1,171 words (изъясниться, соединиться, присоединиться, объясниться, валиться, приземлиться).
    conjugation!("числиться", 1171, "~ить~я";
    infinitive: "~ить~я";
    non_past_1: "~ю~ь", "~им~я";
    non_past_2: "~ишь~я", "~ите~ь";
    non_past_3: "~ит~я", "~ят~я";
    past: "~ил~я", "~ила~ь", "~ило~ь", "~или~ь";
    imperative: "~и~ь", "~ите~ь";
    participle: "~ящий~я", "~ивший~я", _, _;
    gerund: "~я~ь", "~ивши~ь";
    ),
    // Раздуть: 925 words (деревенеть, взять, одеревенеть, деревянеть, болеть, краснеть). Like Играть, except in 0 cells: .
    conjugation!("раздуть", 925, "~ть", like Играть;
    stem past: "~- like раздуть";
    ),
    // Рвануть: 685 words (взбзднуть, хохотнуть, блевануть, пугнуть, гнуть, ткнуть).
    conjugation!("рвануть", 685, "~уть", [aspect: Perfective];
    stem non_past: "~- like рвануть";
    stem past: "~у- like раздуть";
    infinitive: "~уть";
    ),
    // Выделить: 604 words (учить, курить, покурить, кончить, закончить, варить). Like Палить, except in 0 cells: .
    conjugation!("выделить", 604, "~ить", like Палить;
    stem past: "~- like ударить";
    ),
    // Усилиться: 566 words (ссориться, пялиться, ускориться, стариться, состариться, целиться). Like Числиться, except in 3 cells: imperative sg, imperative pl, gerund past.
    conjugation!("усилиться", 566, "~ить~я", like Числиться;
    imperative: "~ь~я", "~ьте~ь";
    gerund: _, "~ивши~ь|~я~ь";
    ),
    // Тянуть: 543 words (выковырнуть, прыгнуть, пёрнуть, капнуть, шлёпнуть, взглянуть). Like Рвануть, except in 0 cells: .
    conjugation!("тянуть", 543, "~уть", [aspect: Perfective], like Рвануть;
    stem non_past: "~- like тянуть";
    ),
    // Обнажиться: 501 words (учиться, кончиться, страшиться, случиться, научиться, уменьшиться). Like Числиться, except in 4 cells: non_past_1 sg, non_past_3 pl, participle pres_act, gerund pres.
    conjugation!("обнажиться", 501, "~ить~я", like Числиться;
    non_past_1: "~у~ь", _;
    non_past_3: _, "~ат~я";
    participle: "~ащий~я", _, _, _;
    gerund: "~а~ь", _;
    ),
    // Любиться: 454 words (появиться, торопиться, стремиться, влюбиться, удивиться, остановиться). Like Числиться, except in 1 cell: non_past_1 sg.
    conjugation!("любиться", 454, "~ить~я", like Числиться;
    non_past_1: "~лю~ь", _;
    ),
    // Обернуться: 392 words (рехнуться, повернуться, проснуться, заткнуться, вернуться, улыбнуться).
    conjugation!("обернуться", 392, "~уть~я", [aspect: Perfective];
    infinitive: "~уть~я";
    non_past_1: "~у~ь", "~ём~я";
    non_past_2: "~ёшь~я", "~ёте~ь";
    non_past_3: "~ёт~я", "~ут~я";
    past: "~ул~я", "~ула~ь", "~уло~ь", "~ули~ь";
    imperative: "~и~ь", "~ите~ь";
    participle: "~ущий~я", "~увший~я", _, _;
    gerund: _, "~увши~ь";
    ),
    // Восстановить: 366 words (уступить, любить, полюбить, возлюбить, вступить, купить). Like Палить, except in 2 cells: non_past_1 sg, participle past_pass.
    conjugation!("восстановить", 366, "~ить", like Палить;
    non_past_1: "~лю", _;
    participle: _, _, _, "~ленный";
    ),
    // Обрушиться: 343 words (увеличиться, размножиться, кочевряжиться, дурачиться, спешиться, мучиться). Like Обнажиться, except in 2 cells: imperative sg, imperative pl.
    conjugation!("обрушиться", 343, "~ить~я", like Обнажиться;
    imperative: "~ь~я", "~ьте~ь";
    ),
    // Разводить: 327 words (происходить, приходить, ходить, проводить, находить, ездить).
    conjugation!("разводить", 327, "~дить";
    infinitive: "~дить";
    non_past_1: "~жу", "~дим";
    non_past_2: "~дишь", "~дите";
    non_past_3: "~дит", "~дят";
    past: "~дил", "~дила", "~дило", "~дили";
    imperative: "~ди", "~дите";
    participle: "~дящий", "~дивший", "~димый", "~женный";
    gerund: "~дя", "~див|~дивши";
    ),
    // Расправиться: 296 words (нравиться, понравиться, готовиться, направиться, знакомиться, познакомиться). Like Усилиться, except in 2 cells: non_past_1 sg, gerund past.
    conjugation!("расправиться", 296, "~ить~я", like Усилиться;
    non_past_1: "~лю~ь", _;
    gerund: _, "~ивши~ь";
    ),
    // Хватить: 296 words (светить, кутить, крутить, платить, шутить, пошутить).
    conjugation!("хватить", 296, "~тить";
    infinitive: "~тить";
    non_past_1: "~чу", "~тим";
    non_past_2: "~тишь", "~тите";
    non_past_3: "~тит", "~тят";
    past: "~тил", "~тила", "~тило", "~тили";
    imperative: "~ти", "~тите";
    participle: "~тящий", "~тивший", "~тимый", "~ченный";
    gerund: "~тя", "~тив|~тивши";
    ),
    // Представить: 290 words (забавить, восславить, готовить, грабить, заставить, править). Like Ударить, except in 2 cells: non_past_1 sg, participle past_pass.
    conjugation!("представить", 290, "~ить", like Ударить;
    non_past_1: "~лю", _;
    participle: _, _, _, "~ленный";
    ),
    // Долбить: 287 words (дымить, удивить, бомбить, употребить, оскорбить, осуществить). Like Восстановить, except in 1 cell: participle past_pass.
    conjugation!("долбить", 287, "~ить", like Восстановить;
    participle: _, _, _, "~лённый";
    ),
    // Расходиться: 283 words (находиться, трудиться, садиться, насладиться, разводиться, родиться).
    conjugation!("расходиться", 283, "~дить~я";
    infinitive: "~дить~я";
    non_past_1: "~жу~ь", "~дим~я";
    non_past_2: "~дишь~я", "~дите~ь";
    non_past_3: "~дит~я", "~дят~я";
    past: "~дил~я", "~дила~ь", "~дило~ь", "~дили~ь";
    imperative: "~ди~ь", "~дите~ь";
    participle: "~дящий~я", "~дивший~я", _, _;
    gerund: "~дя~ь", "~дивши~ь";
    ),
    // Закатиться: 232 words (облокотиться, расплатиться, катиться, портиться, суетиться, светиться).
    conjugation!("закатиться", 232, "~тить~я";
    infinitive: "~тить~я";
    non_past_1: "~чу~ь", "~тим~я";
    non_past_2: "~тишь~я", "~тите~ь";
    non_past_3: "~тит~я", "~тят~я";
    past: "~тил~я", "~тила~ь", "~тило~ь", "~тили~ь";
    imperative: "~ти~ь", "~тите~ь";
    participle: "~тящий~я", "~тивший~я", _, _;
    gerund: "~тя~ь", "~тивши~ь";
    ),
    // Тянуться: 230 words (грохнуться, чокнуться, оглянуться, тянуться, потянуться, откликнуться). Like Обернуться, except in 4 cells: non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg.
    conjugation!("тянуться", 230, "~уть~я", [aspect: Perfective], like Обернуться;
    non_past_1: _, "~ем~я";
    non_past_2: "~ешь~я", "~ете~ь";
    non_past_3: "~ет~я", _;
    ),
    // Залипнуть: 203 words (распухнуть, сдохнуть, замёрзнуть, высохнуть, погибнуть, окрепнуть).
    conjugation!("залипнуть", 203, "~нуть", [aspect: Perfective];
    stem non_past: "~н- like тянуть";
    stem past: "~- like залипнуть";
    infinitive: "~нуть";
    ),
    // Стучать: 192 words (держать, кричать, жужжать, мычать, принадлежать, лежать).
    conjugation!("стучать", 192, "~ать";
    stem non_past: "~- like стучать";
    stem past: "~а- like играть";
    infinitive: "~ать";
    ),
    // Защитить: 191 words (запретить, прекратить, крестить, простить, нарастить, посетить). Like Хватить, except in 2 cells: non_past_1 sg, participle past_pass.
    conjugation!("защитить", 191, "~тить", like Хватить;
    non_past_1: "~щу", _;
    participle: _, _, _, "~щённый";
    ),
    // Сносить: 186 words (носить, спросить, приносить, гласить, переносить, косить).
    conjugation!("сносить", 186, "~сить";
    infinitive: "~сить";
    non_past_1: "~шу", "~сим";
    non_past_2: "~сишь", "~сите";
    non_past_3: "~сит", "~сят";
    past: "~сил", "~сила", "~сило", "~сили";
    imperative: "~си", "~сите";
    participle: "~сящий", "~сивший", "~симый", "~шенный";
    gerund: "~ся", "~сив|~сивши";
    ),
    // Отметить: 165 words (ответить, встретить, заметить, утратить, тратить, похитить). Like Хватить, except in 2 cells: imperative sg, imperative pl.
    conjugation!("отметить", 165, "~тить", like Хватить;
    imperative: "~ть", "~тьте";
    ),
    // Распуститься: 163 words (креститься, спуститься, сгуститься, опуститься, поместиться, проститься).
    conjugation!("распуститься", 163, "~~титься";
    stem past: "~~ти- like распуститься";
    infinitive: "~~титься";
    non_past_1: "~щу~ь", "~~тимся";
    non_past_2: "~~тишься", "~~титесь";
    non_past_3: "~~тится", "~~тятся";
    imperative: "~~тись", "~~титесь";
    participle: "~~тящийся", _, _, _;
    gerund: "~~тясь", _;
    ),
    // Носиться: 138 words (соотноситься, согласиться, относиться, напроситься, произноситься, доноситься).
    conjugation!("носиться", 138, "~~иться";
    stem past: "~~и- like распуститься";
    infinitive: "~~иться";
    non_past_1: "~шу~ь", "~~имся";
    non_past_2: "~~ишься", "~~итесь";
    non_past_3: "~~ится", "~~ятся";
    imperative: "~~ись", "~~итесь";
    participle: "~~ящийся", _, _, _;
    gerund: "~~ясь", _;
    ),
    // Забросить: 125 words (красить, бросить, сбросить, украсить, повесить, весить). Like Сносить, except in 2 cells: imperative sg, imperative pl.
    conjugation!("забросить", 125, "~сить", like Сносить;
    imperative: "~сь", "~сьте";
    ),
    // Встретиться: 120 words (охотиться, заботиться, встретиться, пятиться, отметиться, позаботиться). Like Закатиться, except in 2 cells: imperative sg, imperative pl.
    conjugation!("встретиться", 120, "~тить~я", like Закатиться;
    imperative: "~ть~я", "~тьте~ь";
    ),
    // Оказать: 119 words (сказать, рассказать, промазать, показать, связать, резать).
    conjugation!("оказать", 119, "~зать";
    stem non_past: "~ж- like оказать";
    stem past: "~за- like играть";
    infinitive: "~зать";
    ),
    // Засосать: 118 words (врать, ебать, ржать, заржать, соврать, рвать). Like Стучать, except in 1 cell: participle pres_pass.
    conjugation!("засосать", 118, "~ать", like Стучать;
    stem non_past: "~- like засосать";
    participle: _, _, derived, _;
    ),
    // Топтать: 117 words (хохотать, захохотать, прятать, спрятать, щекотать, лепетать).
    conjugation!("топтать", 117, "~тать";
    stem non_past: "~ч- like оказать";
    stem past: "~та- like играть";
    infinitive: "~тать";
    ),
    // Развеять: 113 words (баять, лаять, чуять, сеять, блеять, посеять). Like Играть, except in 1 cell: infinitive.
    conjugation!("развеять", 113, "~ять", like Играть;
    stem past: "~я- like играть";
    infinitive: "~ять";
    ),
    // Сычевать: 113 words (бичевать, воевать, кочевать, завоевать, ночевать, корчевать). Like Вибрировать, except in 1 cell: infinitive.
    conjugation!("сычевать", 113, "~евать", like Вибрировать;
    stem past: "~- like сычевать";
    infinitive: "~евать";
    ),
    // Перестроиться: 110 words (беспокоиться, успокоиться, приклеиться, строиться, устроиться, настроиться). Like Числиться, except in 2 cells: imperative sg, imperative pl.
    conjugation!("перестроиться", 110, "~ить~я", like Числиться;
    imperative: "~й~я", "~йте~ь";
    ),
    // Держаться: 108 words (воздержаться, мчаться, держаться, послышаться, слышаться, содержаться). Like Обнажиться, except in 7 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("держаться", 108, "~ать~я", like Обнажиться;
    infinitive: "~ать~я";
    past: "~ал~я", "~ала~ь", "~ало~ь", "~али~ь";
    participle: _, "~авший~я", _, _;
    gerund: _, "~авши~ь";
    ),
    // Сорваться: 103 words (ебаться, стебаться, усраться, обосраться, дождаться, заебаться). Like Обернуться, except in 8 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("сорваться", 103, "~ать~я", like Обернуться;
    infinitive: "~ать~я";
    past: "~ал~я", "~ала~ь", "~ало~ь|~ало~ь", "~али~ь|~али~ь";
    participle: _, "~авший~я", _, _;
    gerund: "~я~ь", "~авши~ь";
    ),
    // Трафить: 103 words (суесловить, буравить, брасопить, дыбить, шароёбить, стрёмить). Like Представить, except in 2 cells: participle pres_pass, gerund past.
    conjugation!("трафить", 103, "~ить", [aspect: Imperfective], like Представить;
    participle: _, _, derived, _;
    gerund: _, "~ивши|~ив";
    ),
    // Прошипеть: 103 words (загреметь, храпеть, кипеть, терпеть, скрипеть, греметь). Like Восстановить, except in 2 cells: infinitive, participle past_pass.
    conjugation!("прошипеть", 103, "~еть", like Восстановить;
    stem past: "~е- like прошипеть";
    infinitive: "~еть";
    participle: _, _, _, derived;
    ),
    // Вывозить: 102 words (возить, привозить, скользить, грозить, выразить, возразить).
    conjugation!("вывозить", 102, "~зить";
    infinitive: "~зить";
    non_past_1: "~жу", "~зим";
    non_past_2: "~зишь", "~зите";
    non_past_3: "~зит", "~зят";
    past: "~зил", "~зила", "~зило", "~зили";
    imperative: "~зи", "~зите";
    participle: "~зящий", "~зивший", "~зимый", "~женный";
    gerund: "~зя", "~зив|~зивши";
    ),
    // Наладиться: 95 words (ладиться, наладиться, отвадиться, сгрудиться, заладиться, повадиться). Like Расходиться, except in 2 cells: imperative sg, imperative pl.
    conjugation!("наладиться", 95, "~дить~я", like Расходиться;
    imperative: "~дь~я", "~дьте~ь";
    ),
    // Переброситься: 93 words (выситься, броситься, возвыситься, повыситься, свеситься, наброситься). Like Носиться, except in 2 cells: imperative sg, imperative pl.
    conjugation!("переброситься", 93, "~~иться", like Носиться;
    imperative: "~~ься", "~~ьтесь";
    ),
    // Елозить: 90 words (лазить, заморозить, морозить, конфузить, унизить, снизить). Like Вывозить, except in 2 cells: imperative sg, imperative pl.
    conjugation!("елозить", 90, "~зить", like Вывозить;
    imperative: "~зь", "~зьте";
    ),
    // Подкинуть: 88 words (покинуть, тронуть, кинуть, клюнуть, плюнуть, сунуть). Like Тянуть, except in 1 cell: participle pres_act.
    conjugation!("подкинуть", 88, "~уть", [aspect: Perfective], like Тянуть as ACT_AS_PASS;
    stem non_past: "~- like подкинуть";
    participle: derived, _, _, _;
    ),
    // Открыть: 83 words (открыть, изныть, мыть, помыть, закрыть, рыть).
    conjugation!("открыть", 83, "~ыть";
    stem non_past: "~о- like играть";
    stem past: "~ы- like раздуть";
    infinitive: "~ыть";
    ),
    // Наладить: 83 words (гладить, погладить, лихорадить, бредить, ладить, наладить). Like Разводить, except in 2 cells: imperative sg, imperative pl.
    conjugation!("наладить", 83, "~дить", like Разводить;
    imperative: "~дь", "~дьте";
    ),
    // Выразиться: 81 words (сразиться, возиться, выразиться, выгрузиться, отразиться, грузиться).
    conjugation!("выразиться", 81, "~зить~я";
    infinitive: "~зить~я";
    non_past_1: "~жу~ь", "~зим~я";
    non_past_2: "~зишь~я", "~зите~ь";
    non_past_3: "~зит~я", "~зят~я";
    past: "~зил~я", "~зила~ь", "~зило~ь", "~зили~ь";
    imperative: "~зи~ь", "~зите~ь";
    participle: "~зящий~я", "~зивший~я", _, _;
    gerund: "~зя~ь", "~зивши~ь";
    ),
    // Стать: 81 words (стать, устать, застрять, перестать, встать, надеть). Like Залипнуть, except in 2 cells: infinitive, participle pres_act.
    conjugation!("стать", 81, "~ть", [aspect: Perfective], like Залипнуть as PRES_AS_PAST;
    stem non_past: "~н- like подкинуть";
    stem past: "~- like раздуть";
    infinitive: "~ть";
    participle: derived, _, _, _;
    ),
    // Зарядить: 79 words (щадить, пощадить, опередить, зарядить, отрядить, снарядить). Like Разводить, except in 1 cell: participle past_pass.
    conjugation!("зарядить", 79, "~дить", like Разводить;
    participle: _, _, _, "~жённый";
    ),
    // Отдавать: 77 words (давать, воссоздавать, создавать, издавать, уставать, предавать).
    conjugation!("отдавать", 77, "~вать";
    stem past: "~ва- like прошипеть";
    infinitive: "~вать";
    non_past_1: "~ю", "~ём";
    non_past_2: "~ёшь", "~ёте";
    non_past_3: "~ёт", "~ют";
    imperative: "~вай", "~вайте";
    participle: "~ющий", _, "~ваемый", _;
    gerund: "~вая", _;
    ),
    // Развеяться: 73 words (надеяться, каяться, покаяться, маяться, измаяться, отчаяться). Like Набираться, except in 7 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("развеяться", 73, "~ять~я", like Набираться;
    infinitive: "~ять~я";
    past: "~ял~я", "~яла~ь", "~яло~ь", "~яли~ь";
    participle: _, "~явший~я", _, _;
    gerund: _, "~явши~ь";
    ),
    // Коптить: 72 words (коптить, сплотить, золотить, закоптить, кипятить, вскипятить). Like Хватить, except in 1 cell: participle past_pass.
    conjugation!("коптить", 72, "~тить", like Хватить;
    participle: _, _, _, "~чённый";
    ),
    // Стушеваться: 71 words (стушеваться, тушеваться, разбушеваться, зарубцеваться, кольцеваться, вытанцеваться). Like Стыковаться, except in 7 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("стушеваться", 71, "~евать~я", like Стыковаться;
    infinitive: "~евать~я";
    past: "~евал~я", "~евала~ь", "~евало~ь", "~евали~ь";
    participle: _, "~евавший~я", _, _;
    gerund: _, "~евавши~ь";
    ),
    // Забить: 68 words (пить, бить, лить, вить, шить, побить).
    conjugation!("забить", 68, "~ить";
    stem past: "~и- like раздуть";
    infinitive: "~ить";
    non_past_1: "~ью", "~ьём";
    non_past_2: "~ьёшь", "~ьёте";
    non_past_3: "~ьёт", "~ьют";
    imperative: "~ей", "~ейте";
    participle: "~ьющий", _, _, _;
    gerund: "~ья", _;
    ),
    // Сблизиться: 66 words (приблизиться, сконфузиться, близиться, снизиться, сузиться, сблизиться). Like Выразиться, except in 2 cells: imperative sg, imperative pl.
    conjugation!("сблизиться", 66, "~зить~я", like Выразиться;
    imperative: "~зь~я", "~зьте~ь";
    ),
    // Бухтеть: 65 words (лететь, полететь, взлететь, вертеть, вылететь, прилететь). Like Хватить, except in 7 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("бухтеть", 65, "~теть", like Хватить;
    infinitive: "~теть";
    past: "~тел", "~тела", "~тело", "~тели";
    participle: _, "~тевший", _, _;
    gerund: _, "~тев|~тевши";
    ),
    // Стоить: 62 words (беспокоить, строить, стоить, устроить, построить, усвоить). Like Ударить, except in 2 cells: imperative sg, imperative pl.
    conjugation!("стоить", 62, "~ить", like Ударить;
    imperative: "~й", "~йте";
    ),
    // Зевать: 61 words (зевать, оседлать, хлебать, седлать, верстать, сверстать).
    conjugation!("зевать", 61, "~е~ть";
    stem non_past: "~е~- like играть";
    infinitive: "~е~ть";
    past: "~е~л", "~е~ла", "~е~ло", "~е~ли";
    participle: _, "~е~вший", _, "~ё~нный";
    gerund: _, "~е~в|~е~вши";
    ),
    // Обтечь: 60 words (печь, испечь, спечь, стечь, течь, развлечь).
    conjugation!("обтечь", 60, "~ечь", [aspect: Perfective];
    stem non_past: "~е- like обтечь";
    infinitive: "~ечь";
    past: "~ёк", "~екла", "~екло", "~екли";
    participle: _, "~ёкший", _, "~ечённый";
    gerund: _, "~ёкши";
    ),
    // Возбудить: 59 words (освободить, победить, предупредить, повредить, руководить, обсудить). Like Разводить, except in 1 cell: participle past_pass.
    conjugation!("возбудить", 59, "~дить", like Разводить;
    participle: _, _, _, "~ждённый";
    ),
    // Отвоевать: 58 words (мухлевать, горевать, малевать, полевать, царевать, чаевать).
    conjugation!("отвоевать", 58, "~евать";
    stem non_past: "~ю- like играть";
    stem past: "~- like сычевать";
    infinitive: "~евать";
    ),
    // Сидеть: 57 words (бздеть, набздеть, сидеть, пиздеть, пердеть, выглядеть). Like Зарядить, except in 7 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("сидеть", 57, "~деть", like Зарядить;
    infinitive: "~деть";
    past: "~дел", "~дела", "~дело", "~дели";
    participle: _, "~девший", _, _;
    gerund: _, "~дев|~девши";
    ),
    // Перевернуть: 57 words (перевернуть, повернуть, взметнуть, свернуть, завернуть, подчеркнуть).
    conjugation!("перевернуть", 57, "~е~уть", [aspect: Perfective];
    stem non_past: "~е~- like перевернуть";
    infinitive: "~е~уть";
    past: "~е~ул", "~е~ула", "~е~уло", "~е~ули";
    participle: _, "~е~увший", _, "~ё~утый";
    gerund: _, "~е~ув|~е~увши";
    ),
    // Виться: 57 words (литься, биться, добиться, напиться, налиться, пробиться).
    conjugation!("виться", 57, "~ить~я";
    infinitive: "~ить~я";
    non_past_1: "~ью~ь", "~ьём~я";
    non_past_2: "~ьёшь~я", "~ьёте~ь";
    non_past_3: "~ьёт~я", "~ьют~я";
    past: "~ил~я|~ил~я", "~ила~ь", "~ило~ь|~ило~ь", "~или~ь|~или~ь";
    imperative: "~ей~я", "~ейте~ь";
    participle: "~ьющий~я", "~ивший~я", _, _;
    gerund: "~ья~ь", "~ивши~ь";
    ),
    // Сунуться: 57 words (двинуться, прикинуться, кинуться, дотронуться, тронуться, ринуться). Like Тянуться, except in 2 cells: imperative sg, imperative pl.
    conjugation!("сунуться", 57, "~уть~я", [aspect: Perfective], like Тянуться as PRES_AS_PAST;
    imperative: "~ь~я", "~ьте~ь";
    ),
    // Колоть: 55 words (полоть, колоть, пороть, выполоть, прополоть, выпороть).
    conjugation!("колоть", 55, "~оть";
    stem non_past: "~- like колоть";
    stem past: "~о- like раздуть";
    infinitive: "~оть";
    ),
    // Раскрыться: 55 words (мыться, помыться, скрыться, открыться, умыться, срыться).
    conjugation!("раскрыться", 55, "~ыть~я";
    infinitive: "~ыть~я";
    non_past_1: "~ою~ь", "~оем~я";
    non_past_2: "~оешь~я", "~оете~ь";
    non_past_3: "~оет~я", "~оют~я";
    past: "~ыл~я", "~ыла~ь", "~ыло~ь", "~ыли~ь";
    imperative: "~ой~я", "~ойте~ь";
    participle: "~оющий~я", "~ывший~я", _, _;
    gerund: "~оя~ь", "~ывши~ь";
    ),
    // Заглохнуть: 54 words (достигнуть, проникнуть, возникнуть, затихнуть, подвергнуть, расторгнуть). Like Залипнуть, except in 0 cells: .
    conjugation!("заглохнуть", 54, "~нуть", [aspect: Perfective], like Залипнуть as PRES_AS_PAST;
    stem past: "~- like заглохнуть";
    ),
    // Влечься: 53 words (отречься, увлечься, растечься, отвлечься, стечься, влечься).
    conjugation!("влечься", 53, "~ечь~я";
    infinitive: "~ечь~я";
    non_past_1: "~еку~ь", "~ечём~я";
    non_past_2: "~ечёшь~я", "~ечёте~ь";
    non_past_3: "~ечёт~я", "~екут~я";
    past: "~ёк~я", "~екла~ь", "~екло~ь", "~екли~ь";
    imperative: "~еки~ь", "~еките~ь";
    participle: "~екущий~я", "~ёкший~я", _, _;
    gerund: _, "~ёкши~ь";
    ),
    // Чесаться: 53 words (подписаться, записаться, писаться, расписаться, вписаться, описаться).
    conjugation!("чесаться", 53, "~~аться";
    stem past: "~~а- like распуститься";
    infinitive: "~~аться";
    non_past_1: "~шу~ь", "~шем~я";
    non_past_2: "~шешь~я", "~шете~ь";
    non_past_3: "~шет~я", "~шут~я";
    imperative: "~ши~ь", "~шите~ь";
    participle: "~шущий~я", _, _, _;
    gerund: "~ша~ь", _;
    ),
    // Насытить: 53 words (допустить, пропустить, чистить, пустить, упустить, почистить). Like Отметить, except in 2 cells: non_past_1 sg, participle past_pass.
    conjugation!("насытить", 53, "~тить", like Отметить;
    non_past_1: "~щу", _;
    participle: _, _, _, "~щенный";
    ),
    // Отказаться: 52 words (казаться, оказаться, показаться, отказаться, обязаться, высказаться).
    conjugation!("отказаться", 52, "~зать~я";
    infinitive: "~зать~я";
    non_past_1: "~жу~ь", "~жем~я";
    non_past_2: "~жешь~я", "~жете~ь";
    non_past_3: "~жет~я", "~жут~я";
    past: "~зал~я", "~зала~ь", "~зало~ь", "~зали~ь";
    imperative: "~жи~ь", "~жите~ь";
    participle: "~жущий~я", "~завший~я", _, _;
    gerund: "~жа~ь", "~завши~ь";
    ),
    // Пасть: 52 words (класть, пропасть, напасть, красть, украсть, пасть).
    conjugation!("пасть", 52, "~сть";
    stem non_past: "~д- like пасть";
    infinitive: "~сть";
    past: "~л", "~ла", "~ло", "~ли";
    participle: _, "~вший", _, "~денный";
    gerund: _, "~в|~вши";
    ),
    // Отдаваться: 51 words (издаваться, оставаться, сдаваться, расставаться, удаваться, раздаваться).
    conjugation!("отдаваться", 51, "~вать~я", [aspect: Imperfective];
    infinitive: "~вать~я";
    non_past_1: "~ю~ь", "~ём~я";
    non_past_2: "~ёшь~я", "~ёте~ь";
    non_past_3: "~ёт~я", "~ют~я";
    past: "~вал~я", "~вала~ь", "~вало~ь", "~вали~ь";
    imperative: "~вай~я", "~вайте~ь";
    participle: "~ющий~я", "~вавший~я", _, _;
    gerund: "~вая~ь", "~вавши~ь";
    ),
    // Идти: 51 words (идти, пасти, спасти, ползти, припасти, упасти). Like Засосать, except in 8 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("идти", 51, "~ти", like Засосать;
    stem non_past: "~- like пасть";
    infinitive: "~ти";
    past: "~", "~ла", "~ло", "~ли";
    participle: _, "~ший", _, "~ённый";
    gerund: _, "~ши";
    ),
    // Глядеться: 48 words (увидеться, видеться, привидеться, вглядеться, наглядеться, оглядеться). Like Расходиться, except in 7 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("глядеться", 48, "~деть~я", like Расходиться;
    infinitive: "~деть~я";
    past: "~дел~я", "~дела~ь", "~дело~ь", "~дели~ь";
    participle: _, "~девший~я", _, _;
    gerund: _, "~девши~ь";
    ),
    // Развести: 48 words (провести, перевести, увести, завести, произвести, извести).
    conjugation!("развести", 48, "~ести", [aspect: Perfective];
    stem non_past: "~ед- like перевернуть";
    infinitive: "~ести";
    past: "~ёл", "~ела", "~ело", "~ели";
    participle: _, "~едший", _, "~едённый";
    gerund: _, "~едя|~едши";
    ),
    // Разбить: 46 words (сшить, разбить, сбить, свить, развить, разлить).
    conjugation!("разбить", 46, "~~ить", [aspect: Perfective];
    stem past: "~~и- like раздуть";
    infinitive: "~~ить";
    non_past_1: "~о~ью", "~о~ьём";
    non_past_2: "~о~ьёшь", "~о~ьёте";
    non_past_3: "~о~ьёт", "~о~ьют";
    imperative: "~~ей", "~~ейте";
    ),
    // Клевать: 45 words (блевать, плевать, клевать, жевать, пожевать, наплевать). Like Отвоевать, except in 1 cell: participle pres_pass.
    conjugation!("клевать", 45, "~евать", like Отвоевать;
    stem non_past: "~ю- like клевать";
    participle: _, _, derived, _;
    ),
    // Снести: 44 words (изнести, принести, привезти, произнести, донести, перенести). Like Перевернуть, except in 8 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("снести", 44, "~е~ти", [aspect: Perfective], like Перевернуть;
    infinitive: "~е~ти";
    past: "~ё~", "~е~ла", "~е~ло", "~е~ли";
    participle: _, "~ё~ший", _, "~е~ённый";
    gerund: _, "~е~я|~ё~ши";
    ),
    // Драть: 43 words (брать, избрать, собрать, удрать, стлать, забрать).
    conjugation!("драть", 43, "~~ать";
    stem non_past: "~е~- like пасть";
    stem past: "~~а- like играть";
    infinitive: "~~ать";
    ),
    // Колоться: 42 words (бороться, приколоться, расколоться, колоться, напороться, побороться). Like Набираться, except in 9 cells: infinitive, past m, past f, past n, past pl, imperative sg, ….
    conjugation!("колоться", 42, "~оть~я", like Набираться;
    infinitive: "~оть~я";
    past: "~ол~я", "~ола~ь", "~оло~ь", "~оли~ь";
    imperative: "~и~ь", "~ите~ь";
    participle: _, "~овший~я", _, _;
    gerund: _, "~овши~ь";
    ),
    // Топтаться: 42 words (метаться, топтаться, расхохотаться, заметаться, шептаться, потоптаться).
    conjugation!("топтаться", 42, "~тать~я";
    infinitive: "~тать~я";
    non_past_1: "~чу~ь", "~чем~я";
    non_past_2: "~чешь~я", "~чете~ь";
    non_past_3: "~чет~я", "~чут~я";
    past: "~тал~я", "~тала~ь", "~тало~ь", "~тали~ь";
    imperative: "~чи~ь", "~чите~ь";
    participle: "~чущий~я", "~тавший~я", _, _;
    gerund: "~ча~ь", "~тавши~ь";
    ),
    // Пыхать: 42 words (въехать, махать, брехать, объехать, подъехать, пахать).
    conjugation!("пыхать", 42, "~хать";
    stem non_past: "~ш- like оказать";
    stem past: "~ха- like играть";
    infinitive: "~хать";
    ),
    // Сдать: 41 words (дать, воссоздать, создать, издать, предать, выдать).
    conjugation!("сдать", 41, "~ть", [aspect: Perfective];
    infinitive: "~ть";
    non_past_1: "~м", "~дим";
    non_past_2: "~шь", "~дите";
    non_past_3: "~ст", "~дут";
    past: "~л|~л", "~ла", "~ло", "~ли|~ли";
    imperative: "~й", "~йте";
    participle: _, "~вший", _, "~нный";
    gerund: _, "~в|~вши";
    ),
    // Выписать: 40 words (писать, написать, подписать, приписать, описать, записать). Like Пыхать, except in 1 cell: infinitive.
    conjugation!("выписать", 40, "~сать", like Пыхать;
    stem past: "~са- like играть";
    infinitive: "~сать";
    ),
    // Отобразить: 39 words (вообразить, отразить, исказить, разить, сразить, водрузить). Like Вывозить, except in 1 cell: participle past_pass.
    conjugation!("отобразить", 39, "~зить", like Вывозить;
    participle: _, _, _, "~жённый";
    ),
    // Сбиться: 39 words (влиться, сбиться, развиться, взвиться, разбиться, отбиться).
    conjugation!("сбиться", 39, "~~ить~я", [aspect: Perfective];
    infinitive: "~~ить~я";
    non_past_1: "~о~ью~ь", "~о~ьём~я";
    non_past_2: "~о~ьёшь~я", "~о~ьёте~ь";
    non_past_3: "~о~ьёт~я", "~о~ьют~я";
    past: "~~ил~я", "~~ила~ь", "~~ило~ь", "~~или~ь";
    imperative: "~~ей~я", "~~ейте~ь";
    participle: _, "~~ивший~я", _, _;
    gerund: _, "~~ивши~ь";
    ),
    // Обволочь: 39 words (мочь, смочь, помочь, сволочь, волочь, приволочь).
    conjugation!("обволочь", 39, "~чь";
    infinitive: "~чь";
    non_past_1: "~ку", "~чём";
    non_past_2: "~чёшь", "~чёте";
    non_past_3: "~чёт", "~кут";
    past: "~к", "~кла", "~кло", "~кли";
    imperative: "~ки", "~ките";
    participle: "~кущий", "~кший", _, "~чённый";
    gerund: _, "~кши";
    ),
    // Служить: 37 words (сообщить, обобщить, приобщить, истощить, разобщить, переборщить). Like Палить, except in 0 cells: .
    conjugation!("служить", 37, "~ить", like Палить;
    stem non_past: "~- like стучать";
    ),
    // Обратиться: 37 words (превратиться, обратиться, прекратиться, возвратиться, смутиться, возмутиться). Like Закатиться, except in 2 cells: non_past_1 sg, gerund past.
    conjugation!("обратиться", 37, "~тить~я", [aspect: Perfective], like Закатиться;
    non_past_1: "~щу~ь", _;
    gerund: _, "~тивши~ь|~тя~ь";
    ),
    // Гореть: 37 words (сгореть, смотреть, гореть, посмотреть, велеть, загореть). Like Прошипеть, except in 1 cell: non_past_1 sg.
    conjugation!("гореть", 37, "~еть", like Прошипеть;
    non_past_1: "~ю", _;
    ),
    // Загореться: 36 words (рассмотреться, смотреться, загореться, предусмотреться, просмотреться, присмотреться). Like Числиться, except in 7 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("загореться", 36, "~еть~я", like Числиться;
    infinitive: "~еть~я";
    past: "~ел~я", "~ела~ь", "~ело~ь", "~ели~ь";
    participle: _, "~евший~я", _, _;
    gerund: _, "~евши~ь";
    ),
    // Щипать: 35 words (засыпать, дремать, задремать, сыпать, насыпать, высыпать).
    conjugation!("щипать", 35, "~ать";
    stem past: "~а- like играть";
    infinitive: "~ать";
    non_past_1: "~лю", "~лем";
    non_past_2: "~лешь", "~лете";
    non_past_3: "~лет", "~лют";
    imperative: "~ли", "~лите";
    participle: "~лющий", _, "~аемый", _;
    gerund: "~ля", _;
    ),
    // Трепаться: 34 words (колебаться, поколебаться, трепаться, истрепаться, заколебаться, затрепаться).
    conjugation!("трепаться", 34, "~ать~я";
    infinitive: "~ать~я";
    non_past_1: "~лю~ь", "~лем~я";
    non_past_2: "~лешь~я", "~лете~ь";
    non_past_3: "~лет~я", "~лют~я";
    past: "~ал~я", "~ала~ь", "~ало~ь", "~али~ь";
    imperative: "~ли~ь", "~лите~ь";
    participle: "~лющий~я", "~авший~я", _, _;
    gerund: "~ля~ь", "~авши~ь";
    ),
    // Сплести: 34 words (сплести, расплести, подмести, взгнести, помести, смести).
    conjugation!("сплести", 34, "~ести", [aspect: Perfective];
    stem non_past: "~ет- like перевернуть";
    infinitive: "~ести";
    past: "~ёл", "~ела", "~ело", "~ели";
    participle: _, "~ётший", _, "~етённый";
    gerund: _, "~етя|~ётши";
    ),
    // Расчесать: 33 words (чесать, тесать, расчесать, почесать, причесать, обтесать).
    conjugation!("расчесать", 33, "~есать";
    stem non_past: "~еш- like оказать";
    infinitive: "~есать";
    past: "~есал", "~есала", "~есало", "~есали";
    participle: _, "~есавший", _, "~ёсанный";
    gerund: _, "~есав|~есавши";
    ),
    // Станцевать: 33 words (танцевать, потанцевать, облицевать, вальцевать, станцевать, взлупцевать). Like Сычевать, except in 1 cell: participle past_pass.
    conjugation!("станцевать", 33, "~евать", like Сычевать;
    participle: _, _, _, "~ованный";
    ),
    // Резаться: 32 words (врезаться, резаться, мазаться, нарезаться, размазаться, измазаться). Like Отказаться, except in 2 cells: imperative sg, imperative pl.
    conjugation!("резаться", 32, "~зать~я", like Отказаться;
    imperative: "~жь~я", "~жьте~ь";
    ),
    // Отвоеваться: 32 words (отвоеваться, навоеваться, довоеваться, развоеваться, изгореваться, нагореваться).
    conjugation!("отвоеваться", 32, "~евать~я";
    infinitive: "~евать~я";
    non_past_1: "~юю~ь", "~юем~я";
    non_past_2: "~юешь~я", "~юете~ь";
    non_past_3: "~юет~я", "~юют~я";
    past: "~евал~я", "~евала~ь", "~евало~ь", "~евали~ь";
    imperative: "~юй~я", "~юйте~ь";
    participle: "~юющий~я", "~евавший~я", _, _;
    gerund: "~юя~ь", "~евавши~ь";
    ),
    // Трястись: 31 words (трястись, пастись, сотрястись, спастись, затрястись, стрястись). Like Сорваться, except in 7 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("трястись", 31, "~ти~ь", like Сорваться;
    infinitive: "~ти~ь";
    past: "~~я", "~ла~ь", "~ло~ь", "~ли~ь";
    participle: _, "~ший~я", _, _;
    gerund: _, "~ши~ь";
    ),
    // Обыскать: 31 words (искать, полоскать, отыскать, поискать, разыскать, обыскать).
    conjugation!("обыскать", 31, "~скать";
    stem non_past: "~- like обыскать";
    stem past: "~ска- like играть";
    infinitive: "~скать";
    ),
    // Сковать: 30 words (ковать, подковать, совать, основать, сновать, приковать). Like Вибрировать, except in 1 cell: participle pres_pass.
    conjugation!("сковать", 30, "~овать", like Вибрировать;
    stem non_past: "~у- like клевать";
    participle: _, _, derived, _;
    ),
    // Искусить: 30 words (пригласить, оросить, огласить, провозгласить, воскресить, вопросить). Like Сносить, except in 1 cell: participle past_pass.
    conjugation!("искусить", 30, "~сить", like Сносить;
    participle: _, _, _, "~шённый";
    ),
    // Распасться: 29 words (красться, попасться, распасться, подкрасться, вкрасться, спасться).
    conjugation!("распасться", 29, "~~ться";
    infinitive: "~~ться";
    non_past_1: "~ду~ь", "~дём~я";
    non_past_2: "~дёшь~я", "~дёте~ь";
    non_past_3: "~дёт~я", "~дут~я";
    past: "~л~я", "~ла~ь", "~ло~ь", "~ли~ь";
    imperative: "~ди~ь", "~дите~ь";
    participle: "~дущий~я", "~вший~я", _, _;
    gerund: "~дя~ь", "~вши~ь";
    ),
    // Отпустить: 27 words (вырастить, вымостить, обесчестить, бесчестить, запостить, постить).
    conjugation!("отпустить", 27, "~стить";
    infinitive: "~стить";
    non_past_1: "~щу", "~стим";
    non_past_2: "~стишь", "~стите";
    non_past_3: "~стит", "~стят";
    past: "~стил", "~стила", "~стило", "~стили";
    imperative: "~сти", "~стите";
    participle: "~стящий", "~стивший", "~стимый", "~щенный";
    gerund: "~стя", "~стив|~стивши";
    ),
    // Набраться: 27 words (драться, собраться, придраться, добраться, браться, убраться).
    conjugation!("набраться", 27, "~~ать~я";
    infinitive: "~~ать~я";
    non_past_1: "~е~у~ь", "~е~ём~я";
    non_past_2: "~е~ёшь~я", "~е~ёте~ь";
    non_past_3: "~е~ёт~я", "~е~ут~я";
    past: "~~ал~я|~~ал~я", "~~ала~ь", "~~ало~ь|~~ало~ь", "~~али~ь|~~али~ь";
    imperative: "~е~и~ь", "~е~ите~ь";
    participle: "~е~ущий~я", "~~авший~я", _, _;
    gerund: "~е~я~ь", "~~авши~ь";
    ),
    // Грести: 27 words (грести, погрести, скрести, сгрести, разгрести, поскрести).
    conjugation!("грести", 27, "~ести";
    stem non_past: "~еб- like пасть";
    infinitive: "~ести";
    past: "~ёб", "~ебла", "~ебло", "~ебли";
    participle: _, "~ёбший", _, "~ебённый";
    gerund: _, "~ёбши";
    ),
    // Завестись: 27 words (развестись, произвестись, свестись, довестись, привестись, ввестись).
    conjugation!("завестись", 27, "~е~тись", [aspect: Perfective];
    infinitive: "~е~тись";
    non_past_1: "~еду~ь", "~едём~я";
    non_past_2: "~едёшь~я", "~едёте~ь";
    non_past_3: "~едёт~я", "~едут~я";
    past: "~ёл~я", "~ела~ь", "~ело~ь", "~ели~ь";
    imperative: "~еди~ь", "~едите~ь";
    participle: _, "~едший~я", _, _;
    gerund: _, "~едя~ь|~едши~ь";
    ),
    // Зайти: 27 words (произойти, найти, пойти, дойти, выйти, прийти).
    conjugation!("зайти", 27, "~йти", [aspect: Perfective];
    stem non_past: "~йд- like перевернуть";
    infinitive: "~йти";
    past: "~шёл", "~шла", "~шло", "~шли";
    participle: _, "~шедший", _, "~йдённый";
    gerund: _, "~йдя|~шедши";
    ),
    // Плакать: 26 words (плакать, тыкать, оплакать, кликать, заплакать, поплакать).
    conjugation!("плакать", 26, "~кать";
    stem non_past: "~ч- like плакать";
    stem past: "~ка- like играть";
    infinitive: "~кать";
    ),
    // Просвистеть: 26 words (блестеть, свистеть, хрустеть, заблестеть, насвистеть, посвистеть). Like Отпустить, except in 2 cells: infinitive, participle past_pass.
    conjugation!("просвистеть", 26, "~стеть", like Отпустить;
    stem past: "~сте- like прошипеть";
    infinitive: "~стеть";
    participle: _, _, _, derived;
    ),
    // Терпеться: 26 words (терпеться, натерпеться, обтерпеться, притерпеться, раскипеться, расшуметься). Like Загореться, except in 1 cell: non_past_1 sg.
    conjugation!("терпеться", 26, "~еть~я", like Загореться;
    non_past_1: "~лю~ь", _;
    ),
    // Плеваться: 26 words (плеваться, расплеваться, наклеваться, наплеваться, облеваться, обплеваться). Like Отвоеваться, except in 4 cells: non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg.
    conjugation!("плеваться", 26, "~евать~я", like Отвоеваться;
    non_past_1: _, "~юём~я";
    non_past_2: "~юёшь~я", "~юёте~ь";
    non_past_3: "~юёт~я", _;
    ),
    // Стричься: 26 words (напрячься, достичься, стричься, остричься, обстричься, подстричься).
    conjugation!("стричься", 26, "~чь~я", [aspect: Perfective];
    infinitive: "~чь~я";
    non_past_1: "~гу~ь", "~жём~я";
    non_past_2: "~жёшь~я", "~жёте~ь";
    non_past_3: "~жёт~я", "~гут~я";
    past: "~г~я", "~гла~ь", "~гло~ь", "~гли~ь";
    imperative: "~ги~ь", "~гите~ь";
    participle: "~гущий~я", "~гший~я", _, _;
    gerund: _, "~гши~ь";
    ),
    // Упереться: 26 words (запереться, упереться, опереться, простереться, тереться, потереться).
    conjugation!("упереться", 26, "~е~еть~я";
    infinitive: "~е~еть~я";
    non_past_1: "~~у~ь", "~~ём~я";
    non_past_2: "~~ёшь~я", "~~ёте~ь";
    non_past_3: "~~ёт~я", "~~ут~я";
    past: "~ё~~я", "~ё~ла~ь", "~ё~ло~ь", "~ё~ли~ь";
    imperative: "~~и~ь", "~~ите~ь";
    participle: "~~ущий~я", "~ё~ший~я", _, _;
    gerund: _, "~ё~ши~ь";
    ),
    // Тухнуть: 25 words (исчезнуть, привыкнуть, крепнуть, гибнуть, дрыхнуть, гаснуть). Like Залипнуть, except in 3 cells: past m, participle past_act, gerund past.
    conjugation!("тухнуть", 25, "~нуть", like Залипнуть;
    past: "~|~нул", _, _, _;
    participle: _, "~нувший", _, _;
    gerund: _, "~нув|~нувши";
    ),
    // Наплыть: 25 words (жить, плыть, отплыть, ожить, зажить, пожить).
    conjugation!("наплыть", 25, "~ть";
    stem non_past: "~в- like засосать";
    stem past: "~- like прошипеть";
    infinitive: "~ть";
    ),
    // Разлететься: 24 words (вертеться, разлететься, завертеться, отвертеться, довертеться, навертеться). Like Закатиться, except in 7 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("разлететься", 24, "~теть~я", like Закатиться;
    infinitive: "~теть~я";
    past: "~тел~я", "~тела~ь", "~тело~ь", "~тели~ь";
    participle: _, "~тевший~я", _, _;
    gerund: _, "~тевши~ь";
    ),
    // Расплыться: 24 words (житься, прижиться, расплыться, ужиться, разжиться, сжиться).
    conjugation!("расплыться", 24, "~ть~я";
    infinitive: "~ть~я";
    non_past_1: "~ву~ь", "~вём~я";
    non_past_2: "~вёшь~я", "~вёте~ь";
    non_past_3: "~вёт~я", "~вут~я";
    past: "~л~я|~л~я", "~ла~ь", "~ло~ь|~ло~ь", "~ли~ь";
    imperative: "~ви~ь", "~вите~ь";
    participle: _, "~вший~я", _, _;
    gerund: _, "~вши~ь";
    ),
    // Вторгнуться: 24 words (вторгнуться, подвергнуться, проникнуться, слипнуться, выдохнуться, смеркнуться).
    conjugation!("вторгнуться", 24, "~нуть~я", [aspect: Perfective];
    infinitive: "~нуть~я";
    non_past_1: "~ну~ь", "~нем~я";
    non_past_2: "~нешь~я", "~нете~ь";
    non_past_3: "~нет~я", "~нут~я";
    past: "~~я", "~ла~ь", "~ло~ь", "~ли~ь";
    imperative: "~ни~ь", "~ните~ь";
    participle: "~нущий~я", "~ший~я", _, _;
    gerund: _, "~ши~ь";
    ),
    // Проскочить: 23 words (тащить, вытащить, затащить, потащить, притащить, стащить). Like Служить, except in 0 cells: .
    conjugation!("проскочить", 23, "~ить", [aspect: Perfective], like Служить;
    stem past: "~- like ударить";
    ),
    // Заесть: 23 words (есть, съесть, поесть, надоесть, выесть, разъесть). Like Пасть, except in 9 cells: non_past_1 sg, non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg, non_past_3 pl, ….
    conjugation!("заесть", 23, "~сть", [aspect: Perfective], like Пасть;
    non_past_1: "~м", "~дим";
    non_past_2: "~шь", "~дите";
    non_past_3: "~ст", "~дят";
    imperative: "~шь", "~шьте";
    participle: "~дящий", _, _, _;
    ),
    // Затереть: 22 words (упереть, распростереть, потереть, протереть, простереть, утереть).
    conjugation!("затереть", 22, "~е~еть", [aspect: Perfective];
    stem non_past: "~~- like перевернуть";
    infinitive: "~е~еть";
    past: "~ё~", "~ё~ла", "~ё~ло", "~ё~ли";
    participle: _, "~ё~ший", _, "~ё~тый";
    gerund: _, "~е~ев|~ё~ши";
    ),
    // Перепеть: 22 words (петь, допеть, реветь, запеть, напеть, пропеть).
    conjugation!("перепеть", 22, "~еть";
    stem non_past: "~о- like клевать";
    stem past: "~е- like раздуть";
    infinitive: "~еть";
    ),
    // Отдаться: 21 words (издаться, удаться, сдаться, раздаться, выдаться, даться). Like Расплыться, except in 9 cells: non_past_1 sg, non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg, non_past_3 pl, ….
    conjugation!("отдаться", 21, "~ть~я", [aspect: Perfective], like Расплыться;
    non_past_1: "~м~я", "~дим~я";
    non_past_2: "~шь~я", "~дите~ь";
    non_past_3: "~~тся", "~дут~я";
    past: _, _, _, "~ли~ь|~ли~ь";
    imperative: "~й~я", "~йте~ь";
    ),
    // Дорасти: 21 words (расти, возрасти, порасти, зарасти, нарасти, подрасти).
    conjugation!("дорасти", 21, "~а~ти", [aspect: Perfective];
    stem non_past: "~а~т- like засосать";
    stem past: "~о~- like залипнуть";
    infinitive: "~а~ти";
    ),
    // Забыть: 21 words (быть, прибыть, забыть, добыть, побыть, сбыть).
    conjugation!("забыть", 21, "~ыть", [aspect: Perfective];
    stem non_past: "~уд- like подкинуть";
    stem past: "~ы- like раздуть";
    infinitive: "~ыть";
    ),
    // Унестись: 21 words (произнестись, донестись, отнестись, пронестись, понестись, перенестись). Like Набраться, except in 8 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("унестись", 21, "~е~ти~ь", [aspect: Perfective], like Набраться as PRES_AS_PAST;
    infinitive: "~е~ти~ь";
    past: "~ё~~я", "~е~ла~ь", "~е~ло~ь", "~е~ли~ь";
    participle: _, "~ё~ший~я", _, _;
    gerund: derived, "~е~я~ь|~ё~ши~ь";
    ),
    // Набежать: 20 words (бежать, побежать, избежать, выбежать, убежать, сбежать).
    conjugation!("набежать", 20, "~жать", [aspect: Perfective];
    stem past: "~жа- like прошипеть";
    infinitive: "~жать";
    non_past_1: "~гу", "~жим";
    non_past_2: "~жишь", "~жите";
    non_past_3: "~жит", "~гут";
    imperative: "~ги", "~гите";
    participle: "~гущий", _, _, _;
    ),
    // Расстаться: 20 words (остаться, расстаться, одеться, раздеться, переодеться, достаться). Like Вторгнуться, except in 6 cells: infinitive, past m, imperative sg, imperative pl, participle past_act, gerund past.
    conjugation!("расстаться", 20, "~ть~я", [aspect: Perfective], like Вторгнуться as PRES_AS_PAST;
    infinitive: "~ть~я";
    past: "~л~я", _, _, _;
    imperative: "~нь~я", "~ньте~ь";
    participle: _, "~вший~я", _, _;
    gerund: _, "~вши~ь";
    ),
    // Грызть: 20 words (грызть, обгрызть, огрызть, догрызть, загрызть, изгрызть). Like Идти, except in 1 cell: infinitive.
    conjugation!("грызть", 20, "~ть", like Идти;
    stem past: "~- like грызть";
    infinitive: "~ть";
    ),
    // Зашибить: 20 words (вышибить, зашибить, ушибить, сшибить, прошибить, пришибить). Like Идти, except in 7 cells: infinitive, participle pres_act, participle past_act, participle pres_pass, participle past_pass, gerund pres, ….
    conjugation!("зашибить", 20, "~ить", [aspect: Perfective], like Идти;
    infinitive: "~ить";
    participle: derived, "~ивший", derived, "~ленный";
    gerund: derived, "~ив|~ивши";
    ),
    // Занять: 19 words (понять, занять, нанять, унять, донять, перенять).
    conjugation!("занять", 19, "~нять", [aspect: Perfective];
    stem non_past: "~йм- like перевернуть";
    stem past: "~ня- like раздуть";
    infinitive: "~нять";
    ),
    // Отнять: 19 words (поднять, снять, обнять, отнять, приподнять, разнять).
    conjugation!("отнять", 19, "~ять", [aspect: Perfective];
    stem non_past: "~им- like заглохнуть";
    stem past: "~я- like раздуть";
    infinitive: "~ять";
    ),
    // Лечь: 19 words (лечь, прилечь, залечь, облечь, пролечь, налечь).
    conjugation!("лечь", 19, "~ечь", [aspect: Perfective];
    infinitive: "~ечь";
    non_past_1: "~ягу", "~яжем";
    non_past_2: "~яжешь", "~яжете";
    non_past_3: "~яжет", "~ягут";
    past: "~ёг", "~егла", "~егло", "~егли";
    imperative: "~яг", "~ягте";
    participle: _, "~ёгший", _, _;
    gerund: _, "~ёгши";
    ),
    // Плескаться: 19 words (плескаться, отыскаться, выплескаться, доискаться, обыскаться, полоскаться).
    conjugation!("плескаться", 19, "~~каться";
    stem past: "~~ка- like распуститься";
    infinitive: "~~каться";
    non_past_1: "~щу~ь", "~щем~я";
    non_past_2: "~щешь~я", "~щете~ь";
    non_past_3: "~щет~я", "~щут~я";
    imperative: "~щи~ь", "~щите~ь";
    participle: "~щущий~я", _, _, _;
    gerund: "~ща~ь", _;
    ),
    // Предназначить: 18 words (встопорщить, вывощить, наплющить, натопорщить, повытащить, поморщить). Like Проскочить, except in 2 cells: imperative sg, imperative pl.
    conjugation!("предназначить", 18, "~ить", [aspect: Perfective], like Проскочить;
    imperative: "~ь", "~ьте";
    ),
    // Просмотреть: 18 words (презреть, рассмотреть, предусмотреть, осмотреть, подсмотреть, присмотреть). Like Гореть, except in 2 cells: participle pres_act, gerund pres.
    conjugation!("просмотреть", 18, "~еть", [aspect: Perfective], like Гореть as PRES_AS_PAST;
    stem past: "~е- like играть";
    participle: derived, _, _, _;
    gerund: derived, _;
    ),
    // Въесться: 18 words (наесться, въесться, объесться, приесться, взъесться, заесться). Like Отдаться, except in 7 cells: infinitive, non_past_3 pl, past m, past n, past pl, imperative sg, ….
    conjugation!("въесться", 18, "~~ться", like Отдаться;
    infinitive: "~~ться";
    non_past_3: _, "~дят~я";
    past: "~л~я", _, "~ло~ь", "~ли~ь";
    imperative: "~шь~я", "~шьте~ь";
    ),
    // Потчевать: 18 words (потчевать, выкорчевать, вымежевать, запотчевать, напотчевать, отпотчевать). Like Сычевать, except in 0 cells: .
    conjugation!("потчевать", 18, "~евать", like Сычевать;
    stem past: "~ева- like играть";
    ),
    // Устояться: 17 words (бояться, состояться, забояться, побояться, устояться, застояться). Like Развеяться, except in 6 cells: non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg, non_past_3 pl, participle pres_act.
    conjugation!("устояться", 17, "~ять~я", like Развеяться;
    non_past_1: _, "~им~я";
    non_past_2: "~ишь~я", "~ите~ь";
    non_past_3: "~ит~я", "~ят~я";
    participle: "~ящий~я", _, _, _;
    ),
    // Трепетать: 17 words (клеветать, трепетать, роптать, скрежетать, затрепетать, оклеветать). Like Обыскать, except in 3 cells: infinitive, participle pres_pass, participle past_pass.
    conjugation!("трепетать", 17, "~тать", like Обыскать;
    stem past: "~та- like прошипеть";
    infinitive: "~тать";
    participle: _, _, derived, derived;
    ),
    // Лезть: 17 words (залезть, лезть, полезть, влезть, слезть, пролезть). Like Грызть, except in 8 cells: non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg, imperative sg, imperative pl, ….
    conjugation!("лезть", 17, "~ть", like Грызть;
    non_past_1: _, "~ем";
    non_past_2: "~ешь", "~ете";
    non_past_3: "~ет", _;
    imperative: "~ь", "~ьте";
    participle: _, _, derived, derived;
    ),
    // Жать: 17 words (нажать, выжать, жать, пожать, зажать, прижать).
    conjugation!("жать", 17, "~ать";
    stem non_past: "~м- like засосать";
    stem past: "~а- like раздуть";
    infinitive: "~ать";
    ),
    // Взъебать: 17 words (заебать, проебать, поебать, разъебать, наебать, объебать). Like Драть, except in 9 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("взъебать", 17, "~е~ать", like Драть;
    infinitive: "~е~ать";
    past: "~е~ал", "~е~ала", "~е~ало", "~е~али";
    participle: _, "~е~авший", derived, "~ё~анный";
    gerund: _, "~е~ав|~е~авши";
    ),
    // Колыхаться: 17 words (колыхаться, набрехаться, напахаться, отбрехаться, размахаться, выпахаться). Like Чесаться, except in 7 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("колыхаться", 17, "~хать~я", like Чесаться;
    infinitive: "~хать~я";
    past: "~хал~я", "~хала~ь", "~хало~ь", "~хали~ь";
    participle: _, "~хавший~я", _, _;
    gerund: _, "~хавши~ь";
    ),
    // Вгрызться: 17 words (вгрызться, нагрызться, отгрызться, погрызться, разлезться, прогрызться). Like Трястись, except in 1 cell: infinitive.
    conjugation!("вгрызться", 17, "~ть~я", like Трястись;
    infinitive: "~ть~я";
    ),
    // Постричь: 17 words (пренебречь, стричь, подстричь, остричь, постричь, обстричь).
    conjugation!("постричь", 17, "~чь";
    infinitive: "~чь";
    non_past_1: "~гу", "~жём";
    non_past_2: "~жёшь", "~жёте";
    non_past_3: "~жёт", "~гут";
    past: "~г", "~гла", "~гло", "~гли";
    imperative: "~ги", "~гите";
    participle: "~гущий", "~гший", _, "~женный";
    gerund: _, "~гши";
    ),
    // Разобрать: 16 words (вобрать, разобрать, подобрать, отобрать, ободрать, содрать).
    conjugation!("разобрать", 16, "~о~~ать", [aspect: Perfective];
    stem non_past: "~~е~- like перевернуть";
    stem past: "~о~~а- like играть";
    infinitive: "~о~~ать";
    ),
    // Выровнять: 16 words (выровнять, приравнять, прировнять, сравнять, сровнять, уравнять). Like Отнять, except in 1 cell: participle past_pass.
    conjugation!("выровнять", 16, "~ять", [aspect: Perfective], like Отнять;
    stem non_past: "~я- like выровнять";
    participle: _, _, _, "~енный";
    ),
    // Трепать: 16 words (трепать, потрепать, растрепать, истрепать, встрепать, затрепать).
    conjugation!("трепать", 16, "~е~ать";
    stem non_past: "~е~л- like колоть";
    infinitive: "~е~ать";
    past: "~е~ал", "~е~ала", "~е~ало", "~е~али";
    participle: _, "~е~авший", _, "~ё~анный";
    gerund: _, "~е~ав|~е~авши";
    ),
    // Заволочься: 16 words (заволочься, обволочься, взволочься, волочься, вволочься, доволочься).
    conjugation!("заволочься", 16, "~чь~я";
    infinitive: "~чь~я";
    non_past_1: "~ку~ь", "~чём~я";
    non_past_2: "~чёшь~я", "~чёте~ь";
    non_past_3: "~чёт~я", "~кут~я";
    past: "~к~я", "~кла~ь", "~кло~ь", "~кли~ь";
    imperative: "~ки~ь", "~ките~ь";
    participle: "~кущий~я", "~кший~я", _, _;
    gerund: _, "~кши~ь";
    ),
    // Насесть: 15 words (сесть, засесть, присесть, осесть, подсесть, пересесть). Like Просмотреть, except in 1 cell: infinitive.
    conjugation!("насесть", 15, "~есть", [aspect: Perfective], like Просмотреть as PAST_AS_PRES;
    stem non_past: "~яд- like подкинуть";
    infinitive: "~есть";
    ),
    // Смеяться: 15 words (смеяться, засмеяться, рассмеяться, посмеяться, досмеяться, надсмеяться). Like Развеяться, except in 4 cells: non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg.
    conjugation!("смеяться", 15, "~ять~я", like Развеяться;
    non_past_1: _, "~ём~я";
    non_past_2: "~ёшь~я", "~ёте~ь";
    non_past_3: "~ёт~я", _;
    ),
    // Гнать: 15 words (гнать, догнать, выгнать, погнать, пригнать, изгнать).
    conjugation!("гнать", 15, "~~ать";
    stem non_past: "~о~- like просмотреть";
    stem past: "~~а- like играть";
    infinitive: "~~ать";
    ),
    // Помять: 15 words (мять, помять, распять, замять, домять, вымять).
    conjugation!("помять", 15, "~ять";
    stem non_past: "~н- like засосать";
    stem past: "~я- like раздуть";
    infinitive: "~ять";
    ),
    // Гнить: 15 words (гнить, сгнить, прогнить, загнить, догнить, изгнить). Like Играть, except in 8 cells: non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg, participle pres_act, participle pres_pass, ….
    conjugation!("гнить", 15, "~ть", like Играть;
    non_past_1: _, "~ём";
    non_past_2: "~ёшь", "~ёте";
    non_past_3: "~ёт", _;
    participle: derived, _, derived, derived;
    gerund: derived, _;
    ),
    // Сберечь: 15 words (беречь, стеречь, предостеречь, приберечь, сберечь, уберечь).
    conjugation!("сберечь", 15, "~ечь";
    stem non_past: "~е- like сберечь";
    infinitive: "~ечь";
    past: "~ёг", "~егла", "~егло", "~егли";
    participle: _, "~ёгший", _, "~ежённый";
    gerund: _, "~ёгши";
    ),
    // Честь: 15 words (прочесть, честь, предпочесть, вычесть, учесть, почесть).
    conjugation!("честь", 15, "~есть";
    stem non_past: "~т- like перевернуть";
    infinitive: "~есть";
    past: "~ёл", "~ла", "~ло", "~ли";
    participle: _, _, _, "~тённый";
    gerund: _, "~тя|~ётши";
    ),
    // Стелить: 15 words (стелить, постелить, застелить, расстелить, устелить, зиждить). Like Выделить, except in 6 cells: non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg, non_past_3 pl, participle pres_act.
    conjugation!("стелить", 15, "~ить", like Выделить;
    non_past_1: _, "~ем";
    non_past_2: "~ешь", "~ете";
    non_past_3: "~ет", "~ют";
    participle: "~ющий", _, _, _;
    ),
    // Видеть: 14 words (видеть, увидеть, ненавидеть, предвидеть, возненавидеть, завидеть). Like Сидеть, except in 2 cells: imperative sg, imperative pl.
    conjugation!("видеть", 14, "~деть", like Сидеть;
    stem past: "~де- like играть";
    imperative: "~дь", "~дьте";
    ),
    // Везти: 14 words (нести, везти, заебти, поебти, взъебти, уебти). Like Снести, except in 1 cell: gerund past.
    conjugation!("везти", 14, "~е~ти", like Снести;
    stem non_past: "~е~- like пасть";
    gerund: _, "~ё~ши";
    ),
    // Молоть: 14 words (молоть, смолоть, домолоть, вымолоть, замолоть, намолоть).
    conjugation!("молоть", 14, "~о~оть";
    stem non_past: "~е~- like колоть";
    stem past: "~о~о- like раздуть";
    infinitive: "~о~оть";
    ),
    // Прискакать: 14 words (скакать, отскакать, заскакать, поскакать, наскакать, прискакать). Like Плакать, except in 0 cells: .
    conjugation!("прискакать", 14, "~кать", like Плакать as PAST_AS_PRES;
    stem non_past: "~ч- like оказать";
    ),
    // Молоться: 14 words (вымолоться, молоться, замолоться, измолоться, намолоться, обмолоться).
    conjugation!("молоться", 14, "~о~оть~я";
    infinitive: "~о~оть~я";
    non_past_1: "~е~ю~ь", "~е~ем~я";
    non_past_2: "~е~ешь~я", "~е~ете~ь";
    non_past_3: "~е~ет~я", "~е~ют~я";
    past: "~оло~~я", "~о~ола~ь", "~о~оло~ь", "~о~оли~ь";
    imperative: "~е~и~ь", "~е~ите~ь";
    participle: "~е~ющий~я", "~о~овший~я", _, _;
    gerund: "~е~я~ь", "~о~овши~ь";
    ),
    // Просвистать: 14 words (свистать, насвистать, посвистать, дристать, просвистать, высвистать). Like Трепетать, except in 1 cell: infinitive.
    conjugation!("просвистать", 14, "~стать", like Трепетать;
    stem past: "~ста- like играть";
    infinitive: "~стать";
    ),
    // Прожечь: 14 words (жечь, зажечь, выжечь, пожечь, прижечь, прожечь). Like Постричь, except in 5 cells: infinitive, past m, participle past_act, participle past_pass, gerund past.
    conjugation!("прожечь", 14, "~ечь", like Постричь;
    infinitive: "~ечь";
    past: "~ёг", _, _, _;
    participle: _, "~ёгший", _, "~жённый";
    gerund: _, "~ёгши";
    ),
    // Глохнуть: 13 words (пухнуть, сохнуть, пахнуть, глохнуть, дрогнуть, липнуть). Like Тухнуть, except in 2 cells: participle past_act, gerund past.
    conjugation!("глохнуть", 13, "~нуть", [aspect: Imperfective], like Тухнуть;
    participle: _, "~ший|~нувший", _, _;
    gerund: _, "~нув|~ши|~нувши";
    ),
    // Подобраться: 13 words (разобраться, взобраться, подобраться, разодраться, взодраться, вобраться).
    conjugation!("подобраться", 13, "~о~~ать~я", [aspect: Perfective];
    infinitive: "~о~~ать~я";
    non_past_1: "~~е~у~ь", "~~е~ём~я";
    non_past_2: "~~е~ёшь~я", "~~е~ёте~ь";
    non_past_3: "~~е~ёт~я", "~~е~ут~я";
    past: "~о~~ал~я|~о~~ал~я", "~о~~ала~ь", "~о~~ало~ь|~о~~ало~ь", "~о~~али~ь|~о~~али~ь";
    imperative: "~~е~и~ь", "~~е~ите~ь";
    participle: _, "~о~~авший~я", _, _;
    gerund: _, "~о~~авши~ь";
    ),
    // Засечь: 13 words (усечь, сечь, изречь, пересечь, пресечь, наречь). Like Обтечь, except in 4 cells: past m, participle past_act, participle pres_pass, gerund past.
    conjugation!("засечь", 13, "~ечь", like Обтечь;
    past: "~ёк|~ек", _, _, _;
    participle: _, "~екший|~ёкший", derived, _;
    gerund: _, "~екши|~ёкши";
    ),
    // Толочь: 13 words (толочь, столочь, потолочь, истолочь, растолочь, втолочь). Like Обволочь, except in 4 cells: infinitive, past m, participle past_act, gerund past.
    conjugation!("толочь", 13, "~очь", like Обволочь;
    infinitive: "~очь";
    past: "~ок", _, _, _;
    participle: _, "~окший", _, _;
    gerund: _, "~окши";
    ),
    // Мяться: 13 words (замяться, мяться, помяться, примяться, вымяться, домяться). Like Развеяться, except in 9 cells: non_past_1 sg, non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg, non_past_3 pl, ….
    conjugation!("мяться", 13, "~ять~я", like Развеяться as PRES_AS_PAST;
    non_past_1: "~ну~ь", "~нём~я";
    non_past_2: "~нёшь~я", "~нёте~ь";
    non_past_3: "~нёт~я", "~нут~я";
    imperative: "~ни~ь", "~ните~ь";
    gerund: derived, _;
    ),
    // Сплестись: 13 words (поплестись, переплестись, заплестись, сплестись, проплестись, доплестись).
    conjugation!("сплестись", 13, "~е~тись", [aspect: Perfective];
    infinitive: "~е~тись";
    non_past_1: "~ету~ь", "~етём~я";
    non_past_2: "~етёшь~я", "~етёте~ь";
    non_past_3: "~етёт~я", "~етут~я";
    past: "~ёл~я", "~ела~ь", "~ело~ь", "~ели~ь";
    imperative: "~ети~ь", "~етите~ь";
    participle: _, "~ётший~я", _, _;
    gerund: _, "~етя~ь";
    ),
    // Нагореть: 13 words (нагореть, отгореть, отзвенеть, подзагореть, позвенеть, пообгореть). Like Просмотреть, except in 1 cell: gerund past.
    conjugation!("нагореть", 13, "~еть", like Просмотреть;
    gerund: _, "~евши|~ев";
    ),
    // Тыкаться: 13 words (расплакаться, тыкаться, плакаться, доскакаться, исплакаться, наплакаться). Like Топтаться, except in 9 cells: infinitive, past m, past f, past n, past pl, imperative sg, ….
    conjugation!("тыкаться", 13, "~кать~я", like Топтаться;
    infinitive: "~кать~я";
    past: "~кал~я", "~кала~ь", "~кало~ь", "~кали~ь";
    imperative: "~чь~я", "~чьте~ь";
    participle: _, "~кавший~я", _, _;
    gerund: _, "~кавши~ь";
    ),
    // Отгрестись: 13 words (сгрестись, выгрестись, выскрестись, прискрестись, доскрестись, заскрестись).
    conjugation!("отгрестись", 13, "~е~тись", [aspect: Perfective];
    infinitive: "~е~тись";
    non_past_1: "~ебу~ь", "~ебём~я";
    non_past_2: "~ебёшь~я", "~ебёте~ь";
    non_past_3: "~ебёт~я", "~ебут~я";
    past: "~ёб~я", "~ебла~ь", "~ебло~ь", "~ебли~ь";
    imperative: "~еби~ь", "~ебите~ь";
    participle: _, "~ёбший~я", _, _;
    gerund: _, "~ебя~ь";
    ),
    // Поехать: 12 words (поехать, ехать, приехать, уехать, проехать, выехать).
    conjugation!("поехать", 12, "~хать";
    stem past: "~ха- like прошипеть";
    infinitive: "~хать";
    non_past_1: "~ду", "~дем";
    non_past_2: "~дешь", "~дете";
    non_past_3: "~дет", "~дут";
    imperative: "~зжай", "~зжайте";
    ),
    // Подняться: 12 words (подняться, обняться, сняться, приподняться, отняться, засняться).
    conjugation!("подняться", 12, "~ять~я", [aspect: Perfective];
    infinitive: "~ять~я";
    non_past_1: "~иму~ь", "~имем~я";
    non_past_2: "~имешь~я", "~имете~ь";
    non_past_3: "~имет~я", "~имут~я";
    past: "~ял~я|~ял~я", "~яла~ь", "~яло~ь|~яло~ь", "~яли~ь|~яли~ь";
    imperative: "~ими~ь", "~имите~ь";
    participle: _, "~явший~я", _, _;
    gerund: _, "~явши~ь";
    ),
    // Отослать: 12 words (послать, выслать, прислать, услать, подослать, сослать).
    conjugation!("отослать", 12, "~с~ать", [aspect: Perfective];
    stem non_past: "~ш~- like отослать";
    stem past: "~с~а- like играть";
    infinitive: "~с~ать";
    ),
    // Брить: 12 words (брить, побрить, выбрить, обрить, добрить, забрить). Like Забить, except in 0 cells: .
    conjugation!("брить", 12, "~ить", like Забить;
    stem non_past: "~е- like играть";
    ),
    // Уняться: 12 words (заняться, наняться, поняться, уняться, доняться, переняться).
    conjugation!("уняться", 12, "~нять~я", [aspect: Perfective];
    infinitive: "~нять~я";
    non_past_1: "~йму~ь", "~ймём~я";
    non_past_2: "~ймёшь~я", "~ймёте~ь";
    non_past_3: "~ймёт~я", "~ймут~я";
    past: "~нял~я|~нял~я", "~няла~ь", "~няло~ь", "~няли~ь";
    imperative: "~йми~ь", "~ймите~ь";
    participle: _, "~нявший~я", _, _;
    gerund: _, "~нявши~ь";
    ),
    // Стынуть: 12 words (стынуть, застынуть, остынуть, простынуть, постынуть, настынуть). Like Стать, except in 2 cells: infinitive, participle pres_act.
    conjugation!("стынуть", 12, "~нуть", like Стать as PAST_AS_PRES;
    infinitive: "~нуть";
    participle: "~нущий", _, _, _;
    ),
    // Бриться: 12 words (бриться, побриться, перебриться, выбриться, обриться, пробриться).
    conjugation!("бриться", 12, "~ить~я";
    infinitive: "~ить~я";
    non_past_1: "~ею~ь", "~еем~я";
    non_past_2: "~еешь~я", "~еете~ь";
    non_past_3: "~еет~я", "~еют~я";
    past: "~ил~я", "~ила~ь", "~ило~ь", "~или~ь";
    imperative: "~ей~я", "~ейте~ь";
    participle: "~еющий~я", "~ивший~я", _, _;
    gerund: "~ея~ь", "~ивши~ь";
    ),
    // Морщить: 12 words (упорядочить, морщить, выпучить, сморщить, таращить, вытаращить). Like Предназначить, except in 1 cell: imperative sg.
    conjugation!("морщить", 12, "~ить", like Предназначить;
    imperative: "~и|~ь", _;
    ),
    // Сослаться: 12 words (сослаться, выслаться, дослаться, заслаться, ниспослаться, отослаться).
    conjugation!("сослаться", 12, "~с~ать~я", [aspect: Perfective];
    infinitive: "~с~ать~я";
    non_past_1: "~ш~ю~ь", "~ш~ём~я";
    non_past_2: "~ш~ёшь~я", "~ш~ёте~ь";
    non_past_3: "~ш~ёт~я", "~ш~ют~я";
    past: "~сла~~я", "~с~ала~ь", "~с~ало~ь", "~с~али~ь";
    imperative: "~ш~и~ь", "~ш~ите~ь";
    participle: _, "~с~авший~я", _, _;
    gerund: _, "~с~авши~ь";
    ),
    // Пересидеть: 12 words (просидеть, отсидеть, усидеть, насидеть, пересидеть, засидеть). Like Сидеть, except in 4 cells: participle pres_act, participle pres_pass, participle past_pass, gerund pres.
    conjugation!("пересидеть", 12, "~деть", [aspect: Perfective], like Сидеть;
    participle: derived, _, derived, "~женный";
    gerund: derived, _;
    ),
    // Спеться: 12 words (петься, спеться, допеться, распеться, напеться, воспеться).
    conjugation!("спеться", 12, "~еть~я";
    infinitive: "~еть~я";
    non_past_1: "~ою~ь", "~оём~я";
    non_past_2: "~оёшь~я", "~оёте~ь";
    non_past_3: "~оёт~я", "~оют~я";
    past: "~ел~я", "~ела~ь", "~ело~ь", "~ели~ь";
    imperative: "~ой~я", "~ойте~ь";
    participle: "~оющий~я", "~евший~я", _, _;
    gerund: "~оя~ь", "~евши~ь";
    ),
    // Стереться: 12 words (стереться, отпереться, подтереться, втереться, впереться, обтереться).
    conjugation!("стереться", 12, "~~е~еть~я", [aspect: Perfective];
    infinitive: "~~е~еть~я";
    non_past_1: "~о~~у~ь", "~о~~ём~я";
    non_past_2: "~о~~ёшь~я", "~о~~ёте~ь";
    non_past_3: "~о~~ёт~я", "~о~~ут~я";
    past: "~~ё~~я", "~~ё~ла~ь", "~~ё~ло~ь", "~~ё~ли~ь";
    imperative: "~о~~и~ь", "~о~~ите~ь";
    participle: _, "~~ё~ший~я", _, _;
    gerund: _, "~~ё~ши~ь";
    ),
    // Подвигнуться: 12 words (свергнуться, разверзнуться, низвергнуться, подвигнуться, сверзнуться, исторгнуться). Like Вторгнуться, except in 2 cells: past m, participle past_act.
    conjugation!("подвигнуться", 12, "~нуть~я", [aspect: Perfective], like Вторгнуться as PRES_AS_PAST;
    past: derived, _, _, _;
    participle: _, "~нувший~я|~ший~я", _, _;
    ),
    // Назвать: 12 words (звать, позвать, вызвать, назвать, призвать, прозвать). Like Гнать, except in 0 cells: .
    conjugation!("назвать", 12, "~~ать", like Гнать;
    stem non_past: "~о~- like засосать";
    ),
    // Стелиться: 12 words (зиждиться, расстелиться, стелиться, созиждиться, взъемлиться, постелиться). Like Числиться, except in 6 cells: non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg, non_past_3 pl, participle pres_act.
    conjugation!("стелиться", 12, "~ить~я", like Числиться;
    non_past_1: _, "~ем~я";
    non_past_2: "~ешь~я", "~ете~ь";
    non_past_3: "~ет~я", "~ют~я";
    participle: "~ющий~я", _, _, _;
    ),
    // Заспать: 11 words (спать, поспать, проспать, переспать, заспать, отоспать). Like Стучать, except in 5 cells: non_past_1 sg, non_past_3 pl, participle pres_act, participle pres_pass, gerund pres.
    conjugation!("заспать", 11, "~ать", like Стучать;
    non_past_1: "~лю", _;
    non_past_3: _, "~ят";
    participle: derived, _, derived, _;
    gerund: derived, _;
    ),
    // Ушибиться: 11 words (ошибиться, ушибиться, зашибиться, прошибиться, вышибиться, пришибиться). Like Трястись, except in 4 cells: infinitive, participle past_act, gerund pres, gerund past.
    conjugation!("ушибиться", 11, "~ить~я", [aspect: Perfective], like Трястись as PRES_AS_PAST;
    infinitive: "~ить~я";
    participle: _, "~ивший~я", _, _;
    gerund: derived, "~ивши~ь";
    ),
    // Жаться: 11 words (прижаться, жаться, пожаться, прожаться, ужаться, выжаться).
    conjugation!("жаться", 11, "~ать~я";
    infinitive: "~ать~я";
    non_past_1: "~му~ь", "~мём~я";
    non_past_2: "~мёшь~я", "~мёте~ь";
    non_past_3: "~мёт~я", "~мут~я";
    past: "~ал~я", "~ала~ь", "~ало~ь", "~али~ь";
    imperative: "~ми~ь", "~мите~ь";
    participle: "~мущий~я", "~авший~я", _, _;
    gerund: "~мя~ь", "~авши~ь";
    ),
    // Пакоститься: 11 words (заневеститься, засовеститься, невеститься, скапуститься, испакоститься, чихвоститься). Like Распуститься, except in 2 cells: imperative sg, imperative pl.
    conjugation!("пакоститься", 11, "~~титься", like Распуститься;
    imperative: "~~ться", "~~тьтесь";
    ),
    // Дожить: 11 words (выжить, прожить, пережить, дожить, нажить, изжить). Like Наплыть, except in 5 cells: past m, past n, past pl, participle past_pass, gerund pres.
    conjugation!("дожить", 11, "~ть", [aspect: Perfective], like Наплыть as PRES_AS_PAST;
    past: "~л|~л", _, "~ло|~ло", "~ли|~ли";
    participle: _, _, _, "~тый|~тый";
    gerund: derived, _;
    ),
    // Зассать: 11 words (ссать, сцать, обоссать, зассать, поссать, проссать). Like Засосать, except in 7 cells: non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg, imperative sg, imperative pl, ….
    conjugation!("зассать", 11, "~ать", like Засосать as ACT_AS_PASS;
    non_past_1: _, "~ым";
    non_past_2: "~ышь", "~ыте";
    non_past_3: "~ыт", _;
    imperative: "~ы", "~ыте";
    gerund: derived, _;
    ),
    // Зачесться: 11 words (учесться, расчесться, зачесться, почесться, обчесться, причесться).
    conjugation!("зачесться", 11, "~е~ться";
    infinitive: "~е~ться";
    non_past_1: "~ту~ь", "~тём~я";
    non_past_2: "~тёшь~я", "~тёте~ь";
    non_past_3: "~тёт~я", "~тут~я";
    past: "~ёл~я", "~ла~ь", "~ло~ь", "~ли~ь";
    imperative: "~ти~ь", "~тите~ь";
    gerund: _, "~тя~ь";
    ),
    // Стоять: 10 words (стоять, настоять, постоять, состоять, противостоять, предстоять). Like Развеять, except in 8 cells: non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg, non_past_3 pl, participle pres_act, ….
    conjugation!("стоять", 10, "~ять", like Развеять;
    non_past_1: _, "~им";
    non_past_2: "~ишь", "~ите";
    non_past_3: "~ит", "~ят";
    participle: "~ящий", _, derived, derived;
    ),
    // Беречься: 10 words (беречься, остеречься, поостеречься, постеречься, стеречься, уберечься).
    conjugation!("беречься", 10, "~ечь~я";
    infinitive: "~ечь~я";
    non_past_1: "~егу~ь", "~ежём~я";
    non_past_2: "~ежёшь~я", "~ежёте~ь";
    non_past_3: "~ежёт~я", "~егут~я";
    past: "~ёг~я", "~егла~ь", "~егло~ь", "~егли~ь";
    imperative: "~еги~ь", "~егите~ь";
    participle: "~егущий~я", "~ёгший~я", _, _;
    gerund: _, "~ёгши~ь";
    ),
    // Вырвать: 10 words (выебать, вырвать, выждать, высосать, выжрать, выткать). Like Засосать, except in 2 cells: participle pres_act, gerund pres.
    conjugation!("вырвать", 10, "~ать", [aspect: Perfective], like Засосать as ACT_AS_PASS;
    stem non_past: "~- like заглохнуть";
    participle: derived, _, _, _;
    gerund: derived, _;
    ),
    // Плескать: 10 words (плескать, рукоплескать, расплескать, зарукоплескать, наплескать, оплескать).
    conjugation!("плескать", 10, "~ескать";
    stem non_past: "~е- like обыскать";
    infinitive: "~ескать";
    past: "~ескал", "~ескала", "~ескало", "~ескали";
    participle: _, "~ескавший", _, "~ёсканный";
    gerund: _, "~ескав|~ескавши";
    ),
    // Забыться: 10 words (забыться, сбыться, добыться, избыться, перезабыться, подзабыться). Like Раскрыться, except in 9 cells: non_past_1 sg, non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg, non_past_3 pl, ….
    conjugation!("забыться", 10, "~ыть~я", [aspect: Perfective], like Раскрыться as PRES_AS_PAST;
    non_past_1: "~уду~ь", "~удем~я";
    non_past_2: "~удешь~я", "~удете~ь";
    non_past_3: "~удет~я", "~удут~я";
    imperative: "~удь~я", "~удьте~ь";
    gerund: derived, _;
    ),
    // Нестись: 10 words (нестись, везтись, повезтись, свезтись, перевезтись, наебтись). Like Унестись, except in 3 cells: participle pres_act, gerund pres, gerund past.
    conjugation!("нестись", 10, "~е~ти~ь", like Унестись;
    participle: "~е~ущий~я", _, _, _;
    gerund: "~е~я~ь", "~ё~ши~ь";
    ),
    // Гнаться: 10 words (гнаться, погнаться, угнаться, загнаться, догнаться, пригнаться).
    conjugation!("гнаться", 10, "~~ать~я";
    infinitive: "~~ать~я";
    non_past_1: "~о~ю~ь", "~о~им~я";
    non_past_2: "~о~ишь~я", "~о~ите~ь";
    non_past_3: "~о~ит~я", "~о~ят~я";
    past: "~~ал~я", "~~ала~ь", "~~ало~ь|~~ало~ь", "~~али~ь|~~али~ь";
    imperative: "~о~и~ь", "~о~ите~ь";
    participle: "~о~ящий~я", "~~авший~я", _, _;
    gerund: "~о~я~ь", "~~авши~ь";
    ),
    // Назваться: 10 words (вызваться, назваться, зваться, дозваться, призваться, самоназваться). Like Гнаться, except in 8 cells: non_past_1 sg, non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg, non_past_3 pl, ….
    conjugation!("назваться", 10, "~~ать~я", like Гнаться;
    non_past_1: "~о~у~ь", "~о~ём~я";
    non_past_2: "~о~ёшь~я", "~о~ёте~ь";
    non_past_3: "~о~ёт~я", "~о~ут~я";
    past: "~~ал~я|~~ал~я", _, _, _;
    participle: "~о~ущий~я", _, _, _;
    ),
    // Избороздить: 10 words (нагромоздить, взгромоздить, избороздить, взбороздить, загромоздить, пригвоздить).
    conjugation!("избороздить", 10, "~здить", [aspect: Perfective];
    stem non_past: "~з- like пересидеть";
    infinitive: "~здить";
    past: "~здил", "~здила", "~здило", "~здили";
    participle: _, "~здивший", _, "~ждённый";
    gerund: _, "~здив|~здивши";
    ),
    // Свянуть: 10 words (обвянуть, повянуть, привянуть, призавянуть, отвянуть, подвянуть). Like Стынуть, except in 2 cells: participle past_act, gerund past.
    conjugation!("свянуть", 10, "~нуть", [aspect: Perfective], like Стынуть as PRES_AS_PAST;
    participle: _, "~нувший", _, _;
    gerund: _, "~нувши";
    ),
    // Жеваться: 10 words (нажеваться, жеваться, дожеваться, зажеваться, пережеваться, прожеваться). Like Стушеваться, except in 4 cells: non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg.
    conjugation!("жеваться", 10, "~евать~я", like Стушеваться;
    non_past_1: _, "~уём~я";
    non_past_2: "~уёшь~я", "~уёте~ь";
    non_past_3: "~уёт~я", _;
    ),
    // Обидеться: 10 words (обидеться, навидеться, разобидеться, изобидеться, завидеться, ненавидеться). Like Глядеться, except in 2 cells: imperative sg, imperative pl.
    conjugation!("обидеться", 10, "~деть~я", like Глядеться;
    imperative: "~дь~я", "~дьте~ь";
    ),
    // Вырваться: 10 words (вырваться, выебаться, высраться, выткаться, закармливаться, высосаться). Like Держаться, except in 7 cells: non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg, non_past_3 pl, participle pres_act, ….
    conjugation!("вырваться", 10, "~ать~я", like Держаться;
    non_past_1: _, "~ем~я";
    non_past_2: "~ешь~я", "~ете~ь";
    non_past_3: "~ет~я", "~ут~я";
    participle: "~ущий~я", _, _, _;
    gerund: "~ая~ь", _;
    ),
    // Стонать: 9 words (стонать, жаждать, застонать, простонать, высрать, возжаждать). Like Вырвать, except in 2 cells: participle pres_act, gerund pres.
    conjugation!("стонать", 9, "~ать", like Вырвать as PAST_AS_PRES;
    participle: "~ущий", _, _, _;
    gerund: "~ая", _;
    ),
    // Стереть: 9 words (стереть, подпереть, спереть, обтереть, втереть, впереть).
    conjugation!("стереть", 9, "~~е~еть", [aspect: Perfective];
    stem non_past: "~о~~- like перевернуть";
    infinitive: "~~е~еть";
    past: "~~ё~", "~~ё~ла", "~~ё~ло", "~~ё~ли";
    participle: _, "~~ё~ший", _, "~~ё~тый";
    gerund: _, "~~е~ев|~~ё~ши";
    ),
    // Выспаться: 9 words (выспаться, отоспаться, доспаться, заспаться, наспаться, проспаться). Like Держаться, except in 3 cells: non_past_1 sg, non_past_3 pl, gerund pres.
    conjugation!("выспаться", 9, "~ать~я", [aspect: Perfective], like Держаться as PRES_AS_PAST;
    non_past_1: "~лю~ь", _;
    non_past_3: _, "~ят~я";
    gerund: derived, _;
    ),
    // Размять: 9 words (смять, измять, размять, подмять, вмять, обмять).
    conjugation!("размять", 9, "~~ять", [aspect: Perfective];
    stem non_past: "~о~н- like перевернуть";
    stem past: "~~я- like раздуть";
    infinitive: "~~ять";
    ),
    // Толочься: 9 words (толочься, затолочься, потолочься, истолочься, дотолочься, утолочься). Like Заволочься, except in 4 cells: infinitive, past m, participle past_act, gerund past.
    conjugation!("толочься", 9, "~очь~я", like Заволочься;
    infinitive: "~очь~я";
    past: "~ок~я", _, _, _;
    participle: _, "~окший~я", _, _;
    gerund: _, "~окши~ь";
    ),
    // Скрестись: 9 words (скрестись, грестись, огрестись, догрестись, угрестись, погрестись). Like Отгрестись, except in 2 cells: participle pres_act, gerund past.
    conjugation!("скрестись", 9, "~е~тись", like Отгрестись as PRES_AS_PAST;
    participle: "~ебущий~я", _, _, _;
    gerund: _, "~ёбши~ь";
    ),
    // Зажечься: 9 words (зажечься, ожечься, нажечься, жечься, возжечься, пережечься). Like Стричься, except in 4 cells: infinitive, past m, participle past_act, gerund past.
    conjugation!("зажечься", 9, "~ечь~я", like Стричься;
    infinitive: "~ечь~я";
    past: "~ёг~я", _, _, _;
    participle: _, "~ёгший~я", _, _;
    gerund: _, "~ёгши~ь";
    ),
    // Мести: 9 words (гнести, плести, мести, цвести, расцвести, рассвести). Like Сплести, except in 1 cell: gerund past.
    conjugation!("мести", 9, "~ести", like Сплести;
    stem non_past: "~ет- like засосать";
    gerund: _, "~ётши";
    ),
    // Разойтись: 9 words (найтись, прийтись, обойтись, пройтись, разойтись, сойтись).
    conjugation!("разойтись", 9, "~йти~ь", [aspect: Perfective];
    infinitive: "~йти~ь";
    non_past_1: "~йду~ь", "~йдём~я";
    non_past_2: "~йдёшь~я", "~йдёте~ь";
    non_past_3: "~йдёт~я", "~йдут~я";
    past: "~шёл~я", "~шла~ь", "~шло~ь", "~шли~ь";
    imperative: "~йди~ь", "~йдите~ь";
    participle: _, "~шедший~я", _, _;
    gerund: _, "~шедши~ь";
    ),
    // Застлать: 8 words (постлать, застлать, устлать, выстлать, достлать, настлать). Like Гнать, except in 0 cells: .
    conjugation!("застлать", 8, "~~ать", [aspect: Perfective], like Гнать;
    stem non_past: "~е~- like застлать";
    ),
    // Выпить: 8 words (выпить, вышить, выбить, вылить, подвыпить, вывить). Like Забить, except in 5 cells: non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg, gerund pres.
    conjugation!("выпить", 8, "~ить", [aspect: Perfective], like Забить as ACT_AS_PASS;
    non_past_1: _, "~ьем";
    non_past_2: "~ьешь", "~ьете";
    non_past_3: "~ьет", _;
    gerund: derived, _;
    ),
    // Разогнать: 8 words (подогнать, обогнать, отогнать, разогнать, согнать, вогнать). Like Разобрать, except in 0 cells: .
    conjugation!("разогнать", 8, "~о~~ать", [aspect: Perfective], like Разобрать;
    stem non_past: "~~о~- like просмотреть";
    ),
    // Смяться: 8 words (измяться, смяться, размяться, вмяться, обмяться, подмяться).
    conjugation!("смяться", 8, "~~ять~я", [aspect: Perfective];
    infinitive: "~~ять~я";
    non_past_1: "~о~ну~ь", "~омнё~~я";
    non_past_2: "~о~нёшь~я", "~о~нёте~ь";
    non_past_3: "~о~нёт~я", "~о~нут~я";
    past: "~~ял~я", "~~яла~ь", "~~яло~ь", "~~яли~ь";
    imperative: "~о~ни~ь", "~о~ните~ь";
    participle: _, "~~явший~я", _, _;
    gerund: _, "~~явши~ь";
    ),
    // Осмеять: 8 words (осмеять, засмеять, обсмеять, пересмеять, просмеять, ссять). Like Развеять, except in 7 cells: non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg, participle pres_act, participle pres_pass, ….
    conjugation!("осмеять", 8, "~ять", like Развеять;
    non_past_1: _, "~ём";
    non_past_2: "~ёшь", "~ёте";
    non_past_3: "~ёт", _;
    participle: derived, _, derived, _;
    gerund: derived, _;
    ),
    // Выклевать: 8 words (выклевать, выплевать, выблевать, вымалевать, заштемпелевать, проштемпелевать). Like Отвоевать, except in 3 cells: participle pres_act, participle pres_pass, gerund pres.
    conjugation!("выклевать", 8, "~евать", [aspect: Perfective], like Отвоевать;
    stem past: "~ева- like играть";
    participle: derived, _, derived, _;
    gerund: derived, _;
    ),
    // Разреветься: 8 words (выреветься, нареветься, обреветься, прореветься, разреветься, изреветься). Like Ушибиться, except in 7 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("разреветься", 8, "~еть~я", [aspect: Perfective], like Ушибиться;
    infinitive: "~еть~я";
    past: "~ел~я", "~ела~ь", "~ело~ь", "~ели~ь";
    participle: _, "~евший~я", _, _;
    gerund: _, "~евши~ь";
    ),
    // Выпариться: 8 words (закупориться, выпариться, замусориться, умайориться, откупориться, раскупориться). Like Усилиться, except in 3 cells: imperative sg, gerund pres, gerund past.
    conjugation!("выпариться", 8, "~ить~я", [aspect: Perfective], like Усилиться as PRES_AS_PAST;
    imperative: "~ь~я|~и~ь", _;
    gerund: derived, "~я~ь|~ивши~ь";
    ),
    // Отсвистеться: 8 words (насвистеться, рассвистеться, пересвистеться, свистеться, высвистеться, досвистеться). Like Распуститься, except in 1 cell: infinitive.
    conjugation!("отсвистеться", 8, "~~теться", like Распуститься;
    stem past: "~~те- like распуститься";
    infinitive: "~~теться";
    ),
    // Разогнаться: 8 words (отозваться, разогнаться, взогнаться, вогнаться, обозваться, отогнаться). Like Подобраться, except in 9 cells: non_past_1 sg, non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg, non_past_3 pl, ….
    conjugation!("разогнаться", 8, "~о~~ать~я", [aspect: Perfective], like Подобраться;
    non_past_1: "~~о~ю~ь", "~~о~им~я";
    non_past_2: "~~о~ишь~я", "~~о~ите~ь";
    non_past_3: "~~о~ит~я", "~~о~ят~я";
    past: "~о~~ал~я", _, _, _;
    imperative: "~~о~и~ь", "~~о~ите~ь";
    ),
    // Вести: 8 words (вести, блюсти, брести, соблюсти, грясти, ебсти). Like Пасть, except in 5 cells: infinitive, participle past_act, participle pres_pass, participle past_pass, gerund past.
    conjugation!("вести", 8, "~сти", like Пасть;
    infinitive: "~сти";
    participle: _, "~дший", derived, "~дённый";
    gerund: _, "~дши";
    ),
    // Месть: 8 words (месть, приплесть, приобресть, выцвесть, выгнесть, процвесть). Like Сплести, except in 3 cells: infinitive, participle past_act, gerund past.
    conjugation!("месть", 8, "~есть", like Сплести;
    infinitive: "~есть";
    participle: _, "~етший", _, _;
    gerund: _, "~етя";
    ),
    // Вынести: 7 words (вынести, вывезти, выпасти, вытрясти, повывезти, повынести). Like Лезть, except in 4 cells: infinitive, participle pres_act, participle past_pass, gerund pres.
    conjugation!("вынести", 7, "~ти", [aspect: Perfective], like Лезть as PAST_AS_PRES;
    stem non_past: "~- like заглохнуть";
    infinitive: "~ти";
    participle: derived, _, _, "~енный";
    gerund: derived, _;
    ),
    // Нашептать: 7 words (шептать, прошептать, зашептать, нашептать, пошептать, перешептать).
    conjugation!("нашептать", 7, "~е~тать";
    stem non_past: "~е~ч- like оказать";
    infinitive: "~е~тать";
    past: "~е~тал", "~е~тала", "~е~тало", "~е~тали";
    participle: _, "~е~тавший", _, "~ё~танный";
    gerund: _, "~е~тав|~е~тавши";
    ),
    // Прятаться: 7 words (прятаться, спрятаться, запрятаться, припрятаться, упрятаться, напрятаться). Like Топтаться, except in 2 cells: imperative sg, imperative pl.
    conjugation!("прятаться", 7, "~тать~я", like Топтаться;
    imperative: "~чь~я", "~чьте~ь";
    ),
    // Закупорить: 7 words (выверить, закупорить, вызволить, расстопорить, застопорить, заякорить). Like Ударить, except in 4 cells: imperative sg, participle pres_act, participle pres_pass, gerund pres.
    conjugation!("закупорить", 7, "~ить", [aspect: Perfective], like Ударить;
    imperative: "~и|~ь", _;
    participle: derived, _, derived, _;
    gerund: derived, _;
    ),
    // Залгаться: 7 words (залгаться, изолгаться, отолгаться, оболгаться, разолгаться, налгаться). Like Стричься, except in 7 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("залгаться", 7, "~гать~я", [aspect: Perfective], like Стричься as PRES_AS_PAST;
    infinitive: "~гать~я";
    past: "~гал~я", "~гала~ь", "~гало~ь", "~гали~ь";
    participle: _, "~гавший~я", _, _;
    gerund: _, "~гавши~ь";
    ),
    // Насытиться: 7 words (насытиться, пресытиться, пересытиться, похититься, ухититься, сытиться). Like Встретиться, except in 1 cell: non_past_1 sg.
    conjugation!("насытиться", 7, "~тить~я", like Встретиться;
    non_past_1: "~щу~ь", _;
    ),
    // Достичь: 6 words (достичь, постичь, настичь, застичь, перестичь, пристичь).
    conjugation!("достичь", 6, "~чь", [aspect: Perfective];
    stem non_past: "~гн- like заглохнуть";
    stem past: "~г- like заглохнуть";
    infinitive: "~чь";
    ),
    // Разжечь: 6 words (сжечь, поджечь, отжечь, обжечь, разжечь, изжечь).
    conjugation!("разжечь", 6, "~~ечь", [aspect: Perfective];
    stem non_past: "~о~- like разжечь";
    infinitive: "~~ечь";
    past: "~~ёг", "~о~гла", "~о~гло", "~о~гли";
    participle: _, "~~ёгший", _, "~о~жённый";
    gerund: _, "~~ёгши";
    ),
    // Морщиться: 6 words (таращиться, морщиться, поморщиться, сморщиться, топорщиться, упорядочиться). Like Обрушиться, except in 1 cell: imperative sg.
    conjugation!("морщиться", 6, "~ить~я", like Обрушиться;
    imperative: "~и~ь|~ь~я", _;
    ),
    // Измыслить: 6 words (злоумыслить, умыслить, замыслить, измыслить, вымыслить, обмыслить).
    conjugation!("измыслить", 6, "~с~ить", [aspect: Perfective];
    stem non_past: "~с~- like просмотреть";
    infinitive: "~с~ить";
    past: "~с~ил", "~с~ила", "~с~ило", "~с~или";
    participle: _, "~с~ивший", _, "~ш~енный";
    gerund: _, "~с~ив|~с~ивши";
    ),
    // Рассесться: 6 words (усесться, рассесться, осесться, ссесться, надсесться, обсесться). Like Разреветься, except in 9 cells: infinitive, non_past_1 sg, non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg, ….
    conjugation!("рассесться", 6, "~е~ться", [aspect: Perfective], like Разреветься;
    infinitive: "~е~ться";
    non_past_1: "~яду~ь", "~ядем~я";
    non_past_2: "~ядешь~я", "~ядете~ь";
    non_past_3: "~ядет~я", "~ядут~я";
    imperative: "~ядь~я", "~ядьте~ь";
    ),
    // Сжаться: 6 words (сжаться, вжаться, отжаться, обжаться, поджаться, разжаться).
    conjugation!("сжаться", 6, "~~ать~я", [aspect: Perfective];
    infinitive: "~~ать~я";
    non_past_1: "~о~му~ь", "~о~мём~я";
    non_past_2: "~о~мёшь~я", "~о~мёте~ь";
    non_past_3: "~о~мёт~я", "~о~мут~я";
    past: "~~ал~я", "~~ала~ь", "~~ало~ь", "~~али~ь";
    imperative: "~о~ми~ь", "~о~мите~ь";
    participle: _, "~~авший~я", _, _;
    gerund: _, "~~авши~ь";
    ),
    // Соваться: 6 words (коваться, основаться, соваться, подковаться, вковаться, рассоваться). Like Стыковаться, except in 4 cells: non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg.
    conjugation!("соваться", 6, "~овать~я", like Стыковаться;
    non_past_1: _, "~уём~я";
    non_past_2: "~уёшь~я", "~уёте~ь";
    non_past_3: "~уёт~я", _;
    ),
    // Разъехаться: 6 words (съехаться, разъехаться, проехаться, поразъехаться, подъехаться, ехаться). Like Колыхаться, except in 9 cells: non_past_1 sg, non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg, non_past_3 pl, ….
    conjugation!("разъехаться", 6, "~хать~я", like Колыхаться as PRES_AS_PAST;
    non_past_1: "~ду~ь", "~дем~я";
    non_past_2: "~дешь~я", "~дете~ь";
    non_past_3: "~дет~я", "~дут~я";
    imperative: "~зжай~я", "~зжайте~ь";
    gerund: derived, _;
    ),
    // Сыпаться: 6 words (рассыпаться, посыпаться, сыпаться, осыпаться, высыпаться, отсыпаться). Like Трепаться, except in 6 cells: non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg, imperative sg, imperative pl.
    conjugation!("сыпаться", 6, "~ать~я", like Трепаться;
    non_past_1: _, "~лем~я|~ем~я";
    non_past_2: "~лешь~я|~ешь~я", "~лете~ь|~ете~ь";
    non_past_3: "~лет~я|~ет~я", _;
    imperative: "~ь~я", "~ьте~ь";
    ),
    // Хлестать: 6 words (хлестать, нахлестать, похлестать, расхлестать, схлестать, захлестать). Like Плескать, except in 9 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("хлестать", 6, "~естать", like Плескать;
    infinitive: "~естать";
    past: "~естал", "~естала", "~естало", "~естали";
    participle: _, "~еставший", derived, "~ёстанный";
    gerund: _, "~естав|~еставши";
    ),
    // Плестись: 6 words (плестись, местись, сместись, цвестись, гнестись, доместись). Like Сплестись, except in 2 cells: participle pres_act, gerund past.
    conjugation!("плестись", 6, "~е~тись", like Сплестись as PRES_AS_PAST;
    participle: "~етущий~я", _, _, _;
    gerund: _, "~ётши~ь";
    ),
    // Выпасть: 6 words (выпасть, выкрасть, выпрясть, повыпасть, выкласть, повыкрасть). Like Пасть, except in 3 cells: participle pres_act, participle pres_pass, gerund pres.
    conjugation!("выпасть", 6, "~сть", [aspect: Perfective], like Пасть;
    stem non_past: "~д- like заглохнуть";
    participle: derived, _, derived, _;
    gerund: derived, _;
    ),
    // Исторгнуть: 6 words (ниспровергнуть, исторгнуть, разверзнуть, отверзнуть, нисповергнуть, развергнуть). Like Заглохнуть, except in 2 cells: past m, gerund past.
    conjugation!("исторгнуть", 6, "~нуть", [aspect: Perfective], like Заглохнуть;
    past: derived, _, _, _;
    gerund: _, "~ши|~нув|~нувши";
    ),
    // Зачертить: 6 words (вкатить, вскатить, докатить, дочертить, зачертить, подчертить). Like Хватить, except in 4 cells: participle pres_act, participle pres_pass, participle past_pass, gerund pres.
    conjugation!("зачертить", 6, "~тить", [aspect: Perfective], like Хватить;
    participle: derived, _, derived, "~танный";
    gerund: derived, _;
    ),
    // Исхлестаться: 6 words (насвистаться, рассвистаться, хвостаться, расхлестаться, исхлестаться, нахлестаться). Like Плескаться, except in 1 cell: infinitive.
    conjugation!("исхлестаться", 6, "~~таться", like Плескаться;
    stem past: "~~та- like распуститься";
    infinitive: "~~таться";
    ),
    // Обидеть: 6 words (обидеть, высидеть, изобидеть, разобидеть, забидеть, пообидеть). Like Пересидеть, except in 2 cells: imperative sg, imperative pl.
    conjugation!("обидеть", 6, "~деть", [aspect: Perfective], like Пересидеть;
    imperative: "~дь", "~дьте";
    ),
    // Вымести: 6 words (выцвести, вымести, выплести, повыцвести, мясти, выгнести). Like Честь, except in 5 cells: infinitive, past m, participle past_act, participle past_pass, gerund past.
    conjugation!("вымести", 6, "~сти", like Честь;
    stem non_past: "~т- like заглохнуть";
    infinitive: "~сти";
    past: "~л", _, _, _;
    participle: _, "~тший", _, "~тенный";
    gerund: _, "~тя";
    ),
    // Замереть: 6 words (запереть, умереть, помереть, замереть, примереть, позапереть). Like Затереть, except in 0 cells: .
    conjugation!("замереть", 6, "~е~еть", [aspect: Perfective], like Затереть;
    stem past: "~е~- like замереть";
    ),
    // Мочься: 6 words (разнемочься, выпрячься, выстричься, мочься, занемочься, перемочься). Like Стричься, except in 4 cells: non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg.
    conjugation!("мочься", 6, "~чь~я", like Стричься as PRES_AS_PAST;
    non_past_1: _, "~жем~я";
    non_past_2: "~жешь~я", "~жете~ь";
    non_past_3: "~жет~я", _;
    ),
    // Взорать: 6 words (глаголать, проглаголать, обкармливать, разорать, взорать, доорать). Like Вырвать, except in 1 cell: participle past_pass.
    conjugation!("взорать", 6, "~ать", like Вырвать;
    stem non_past: "~- like застлать";
    participle: _, _, _, "~енный";
    ),
    // Висеть: 5 words (висеть, зависеть, повисеть, перевисеть, провисеть). Like Сносить, except in 3 cells: infinitive, participle pres_pass, participle past_pass.
    conjugation!("висеть", 5, "~сеть", like Сносить;
    stem past: "~се- like прошипеть";
    infinitive: "~сеть";
    participle: _, _, derived, derived;
    ),
    // Выбрать: 5 words (выбрать, выдрать, перевыбрать, повыдрать, повыбрать). Like Застлать, except in 0 cells: .
    conjugation!("выбрать", 5, "~~ать", [aspect: Perfective], like Застлать;
    stem non_past: "~е~- like заглохнуть";
    ),
    // Отжать: 5 words (сжать, обжать, разжать, отжать, вжать). Like Гнать, except in 0 cells: .
    conjugation!("отжать", 5, "~~ать", [aspect: Perfective], like Гнать;
    stem non_past: "~о~м- like перевернуть";
    stem past: "~~а- like раздуть";
    ),
    // Отстоять: 5 words (отстоять, выстоять, простоять, застоять, достоять). Like Стоять, except in 1 cell: gerund pres.
    conjugation!("отстоять", 5, "~ять", [aspect: Perfective], like Стоять as PRES_AS_PAST;
    stem past: "~я- like играть";
    gerund: derived, _;
    ),
    // Выставиться: 5 words (понадобиться, осведомиться, выставиться, занадобиться, уведомиться). Like Расправиться, except in 2 cells: imperative sg, gerund pres.
    conjugation!("выставиться", 5, "~ить~я", [aspect: Perfective], like Расправиться as PRES_AS_PAST;
    imperative: "~и~ь|~ь~я", _;
    gerund: derived, _;
    ),
    // Одолеть: 5 words (преодолеть, одолеть, запечатлеть, напечатлеть, отпечатлеть). Like Просмотреть, except in 1 cell: participle past_pass.
    conjugation!("одолеть", 5, "~еть", [aspect: Perfective], like Просмотреть;
    stem non_past: "~е- like выровнять";
    participle: _, _, _, "~ённый";
    ),
    // Выставить: 5 words (выставить, выправить, осведомить, уведомить, предуведомить). Like Закупорить, except in 2 cells: non_past_1 sg, participle past_pass.
    conjugation!("выставить", 5, "~ить", [aspect: Perfective], like Закупорить;
    non_past_1: "~лю", _;
    participle: _, _, _, "~ленный";
    ),
    // Изъять: 5 words (подъять, изъять, разъять, отъять, съять).
    conjugation!("изъять", 5, "~ъять", [aspect: Perfective];
    stem non_past: "~ым- like заглохнуть";
    stem past: "~ъя- like раздуть";
    infinitive: "~ъять";
    ),
    // Выдвинуться: 5 words (высунуться, выдвинуться, грянуться, откашлянуться, самовыдвинуться). Like Сунуться, except in 1 cell: imperative sg.
    conjugation!("выдвинуться", 5, "~уть~я", [aspect: Perfective], like Сунуться;
    imperative: "~и~ь", _;
    ),
    // Пробежаться: 5 words (разбежаться, сбежаться, пробежаться, поразбежаться, оббежаться).
    conjugation!("пробежаться", 5, "~жать~я", [aspect: Perfective];
    infinitive: "~жать~я";
    non_past_1: "~гу~ь", "~жим~я";
    non_past_2: "~жишь~я", "~жите~ь";
    non_past_3: "~жит~я", "~гут~я";
    past: "~жал~я", "~жала~ь", "~жало~ь", "~жали~ь";
    imperative: "~ги~ь", "~гите~ь";
    participle: _, "~жавший~я", _, _;
    gerund: _, "~жавши~ь";
    ),
    // Обжечься: 5 words (обжечься, разжечься, поджечься, сжечься, изжечься).
    conjugation!("обжечься", 5, "~~ечь~я", [aspect: Perfective];
    infinitive: "~~ечь~я";
    non_past_1: "~о~гу~ь", "~о~жём~я";
    non_past_2: "~о~жёшь~я", "~о~жёте~ь";
    non_past_3: "~о~жёт~я", "~о~гут~я";
    past: "~~ёг~я", "~о~гла~ь", "~о~гло~ь", "~о~гли~ь";
    imperative: "~о~ги~ь", "~о~гите~ь";
    participle: _, "~~ёгший~я", _, _;
    gerund: _, "~~ёгши~ь";
    ),
    // Волгнуть: 5 words (волгнуть, брюзгнуть, горкнуть, грузнуть, осмеркнуть). Like Тухнуть, except in 1 cell: past m.
    conjugation!("волгнуть", 5, "~нуть", like Тухнуть;
    past: "~нул", _, _, _;
    ),
    // Стлаться: 5 words (стлаться, подостлаться, постлаться, простлаться, устлаться). Like Молоться, except in 7 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("стлаться", 5, "~~ать~я", like Молоться;
    infinitive: "~~ать~я";
    past: "~~ал~я", "~~ала~ь", "~~ало~ь", "~~али~ь";
    participle: _, "~~авший~я", _, _;
    gerund: _, "~~авши~ь";
    ),
    // Распоясаться: 5 words (распоясаться, выплясаться, подпоясаться, вытесаться, опоясаться). Like Чесаться, except in 3 cells: imperative sg, imperative pl, gerund pres.
    conjugation!("распоясаться", 5, "~~аться", [aspect: Perfective], like Чесаться as PRES_AS_PAST;
    imperative: "~шь~я", "~шьте~ь";
    gerund: derived, _;
    ),
    // Трепетаться: 5 words (вострепетаться, трепетаться, оклеветаться, затрепетаться, потрепетаться). Like Плескаться, except in 7 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("трепетаться", 5, "~тать~я", like Плескаться;
    infinitive: "~тать~я";
    past: "~тал~я", "~тала~ь", "~тало~ь", "~тали~ь";
    participle: _, "~тавший~я", _, _;
    gerund: _, "~тавши~ь";
    ),
    // Ссаться: 5 words (уссаться, обоссаться, ссаться, сцаться, обосцаться). Like Вырваться, except in 7 cells: non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg, imperative sg, imperative pl, ….
    conjugation!("ссаться", 5, "~ать~я", like Вырваться;
    non_past_1: _, "~ым~я";
    non_past_2: "~ышь~я", "~ыте~ь";
    non_past_3: "~ыт~я", _;
    imperative: "~ы~ь", "~ыте~ь";
    gerund: "~а~ь", _;
    ),
    // Срастись: 5 words (разрастись, срастись, зарастись, врастись, растись).
    conjugation!("срастись", 5, "~а~ти~ь";
    infinitive: "~а~ти~ь";
    non_past_1: "~а~ту~ь", "~а~тём~я";
    non_past_2: "~а~тёшь~я", "~а~тёте~ь";
    non_past_3: "~а~тёт~я", "~а~тут~я";
    past: "~о~~я", "~о~ла~ь", "~о~ло~ь", "~о~ли~ь";
    imperative: "~а~ти~ь", "~а~тите~ь";
    participle: "~а~тущий~я", "~о~ший~я", _, _;
    gerund: "~а~тя~ь", "~о~ши~ь";
    ),
    // Вестись: 5 words (вестись, соблюстись, ебстись, блюстись, поебстись). Like Распасться, except in 3 cells: infinitive, participle past_act, gerund past.
    conjugation!("вестись", 5, "~~тись", like Распасться;
    infinitive: "~~тись";
    participle: _, "~дший~я", _, _;
    gerund: _, "~дши~ь";
    ),
    // Вывиться: 5 words (выбиться, вылиться, выпиться, вывиться, повыбиться). Like Бриться, except in 7 cells: non_past_1 sg, non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg, non_past_3 pl, ….
    conjugation!("вывиться", 5, "~ить~я", [aspect: Perfective], like Бриться as PRES_AS_PAST;
    non_past_1: "~ью~ь", "~ьем~я";
    non_past_2: "~ьешь~я", "~ейте~ь";
    non_past_3: "~ьет~я", "~ьют~я";
    gerund: derived, _;
    ),
    // Выползти: 5 words (вылезти, выползти, повылезти, повыползти, выебти). Like Вынести, except in 0 cells: .
    conjugation!("выползти", 5, "~ти", [aspect: Perfective], like Вынести;
    stem past: "~- like грызть";
    ),
    // Изобрести: 4 words (изобрести, приобрести, обрести, соплести). Like Сплести, except in 2 cells: participle past_act, gerund past.
    conjugation!("изобрести", 4, "~ести", [aspect: Perfective], like Сплести;
    participle: _, "~етший", _, _;
    gerund: _, "~етя|~етши";
    ),
    // Принять: 4 words (принять, воспринять, предпринять, напринять). Like Занять, except in 0 cells: .
    conjugation!("принять", 4, "~нять", [aspect: Perfective], like Занять;
    stem non_past: "~м- like заглохнуть";
    ),
    // Услышать: 4 words (услышать, вымолчать, выбрюзжать, дослышать). Like Заспать, except in 4 cells: non_past_1 sg, non_past_3 pl, imperative sg, imperative pl.
    conjugation!("услышать", 4, "~ать", [aspect: Perfective], like Заспать;
    non_past_1: "~у", _;
    non_past_3: _, "~ат";
    imperative: "~ь", "~ьте";
    ),
    // Выпереть: 4 words (вымереть, вытереть, выпереть, повымереть). Like Замереть, except in 1 cell: gerund past.
    conjugation!("выпереть", 4, "~е~еть", [aspect: Perfective], like Замереть;
    stem non_past: "~~- like заглохнуть";
    gerund: _, "~е~ев|~е~ши";
    ),
    // Высечь: 4 words (высечь, вытечь, выволочь, выпечь). Like Обволочь, except in 5 cells: non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg, participle past_pass.
    conjugation!("высечь", 4, "~чь", [aspect: Perfective], like Обволочь as ACT_AS_PASS;
    non_past_1: _, "~чем";
    non_past_2: "~чешь", "~чете";
    non_past_3: "~чет", _;
    participle: _, _, _, "~ченный";
    ),
    // Напрячь: 4 words (запрячь, напрячь, распрячь, впрячь).
    conjugation!("напрячь", 4, "~ячь", [aspect: Perfective];
    stem non_past: "~я- like разжечь";
    infinitive: "~ячь";
    past: "~ёг|~яг", "~ягла", "~ягло", "~ягли";
    participle: _, "~ягший", _, "~яжённый";
    gerund: _, "~ягши";
    ),
    // Выдвинуть: 4 words (выдвинуть, высунуть, кашлянуть, выплюнуть). Like Подкинуть, except in 1 cell: imperative sg.
    conjugation!("выдвинуть", 4, "~уть", [aspect: Perfective], like Подкинуть;
    imperative: "~и|~ь", _;
    ),
    // Внимать: 4 words (внимать, приимать, восприимать, предприимать).
    conjugation!("внимать", 4, "~и~ать", [aspect: Imperfective];
    infinitive: "~и~ать";
    non_past_1: "~е~лю|~е~лю|~и~аю", "~е~лем|~и~аем";
    non_past_2: "~е~лешь|~и~аешь", "~е~лете|~и~аете";
    non_past_3: "~е~лет|~и~ает", "~е~лют|~и~ают";
    past: "~и~ал", "~и~ала", "~и~ало", "~и~али";
    imperative: "~е~ли|~е~ли|~и~ай", "~е~лите|~е~лите|~и~айте";
    participle: "~е~лющий|~и~ающий", "~и~авший", "~е~лемый|~и~аемый", "~е~ленный";
    gerund: "~е~ля|~е~ля|~и~ая", "~и~ав|~и~авши";
    ),
    // Бечь: 4 words (бечь, убечь, избечь, побечь). Like Сберечь, except in 2 cells: participle pres_pass, participle past_pass.
    conjugation!("бечь", 4, "~ечь", like Сберечь;
    stem non_past: "~ег- like рвануть";
    participle: _, _, derived, derived;
    ),
    // Улечься: 4 words (улечься, поулечься, разлечься, приулечься).
    conjugation!("улечься", 4, "~ечься", [aspect: Perfective];
    infinitive: "~ечься";
    non_past_1: "~ягусь", "~яжемся";
    non_past_2: "~яжешься", "~яжетесь";
    non_past_3: "~яжется", "~ягутся";
    past: "~ёгся", "~еглась", "~еглось", "~еглись";
    imperative: "~ягся", "~ягтесь";
    participle: _, "~ёгшийся", _, _;
    gerund: _, "~ёгшись";
    ),
    // Вынестись: 4 words (вынестись, вывезтись, вытрястись, отверзтись). Like Трястись, except in 6 cells: non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg, participle pres_act, gerund pres.
    conjugation!("вынестись", 4, "~ти~ь", [aspect: Perfective], like Трястись as PAST_AS_PRES;
    non_past_1: _, "~ем~я";
    non_past_2: "~ешь~я", "~ете~ь";
    non_past_3: "~ет~я", _;
    participle: derived, _, _, _;
    gerund: derived, _;
    ),
    // Истереть: 4 words (растереть, распереть, истереть, поистереть).
    conjugation!("истереть", 4, "~с~е~еть", [aspect: Perfective];
    stem non_past: "~зо~~- like перевернуть";
    infinitive: "~с~е~еть";
    past: "~с~ё~", "~с~ё~ла", "~с~ё~ло", "~с~ё~ли";
    participle: _, "~с~ё~ший", _, "~с~ё~тый";
    gerund: _, "~с~е~ев|~с~ё~ши";
    ),
    // Испить: 4 words (расшить, испить, распить, исшить). Like Измыслить, except in 8 cells: non_past_1 sg, non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg, non_past_3 pl, ….
    conjugation!("испить", 4, "~с~ить", [aspect: Perfective], like Измыслить;
    stem past: "~с~и- like раздуть";
    non_past_1: "~зо~ью", "~зо~ьём";
    non_past_2: "~зо~ьёшь", "~зо~ьёте";
    non_past_3: "~зо~ьёт", "~зо~ьют";
    imperative: "~с~ей", "~с~ейте";
    ),
    // Поколебать: 4 words (колебать, поколебать, зыбать, выщипать). Like Щипать, except in 2 cells: participle pres_pass, participle past_pass.
    conjugation!("поколебать", 4, "~ать", like Щипать;
    participle: _, _, "~лемый", "~ленный";
    ),
    // Насочинять: 4 words (насочинять, обрусять, постеснять, оязыковлять). Like Выровнять, except in 1 cell: participle past_pass.
    conjugation!("насочинять", 4, "~ять", like Выровнять;
    stem non_past: "~я- like играть";
    participle: _, _, _, "~ённый";
    ),
    // Растереться: 4 words (растереться, истереться, поистереться, распереться).
    conjugation!("растереться", 4, "~с~е~еть~я", [aspect: Perfective];
    infinitive: "~с~е~еть~я";
    non_past_1: "~зо~~у~ь", "~зо~~ём~я";
    non_past_2: "~зо~~ёшь~я", "~зо~~ёте~ь";
    non_past_3: "~зо~~ёт~я", "~зо~~ут~я";
    past: "~с~ё~~я", "~с~ё~ла~ь", "~с~ё~ло~ь", "~с~ё~ли~ь";
    imperative: "~зо~~и~ь", "~зо~~ите~ь";
    participle: _, "~с~ё~ший~я", _, _;
    gerund: _, "~с~ё~ши~ь";
    ),
    // Изъяться: 4 words (изъяться, подъяться, разъяться, отъяться).
    conjugation!("изъяться", 4, "~ъять~я", [aspect: Perfective];
    infinitive: "~ъять~я";
    non_past_1: "~ыму~ь", "~ымем~я";
    non_past_2: "~ымешь~я", "~ымете~ь";
    non_past_3: "~ымет~я", "~ымут~я";
    past: "~ъял~я", "~ъяла~ь", "~ъяло~ь", "~ъяли~ь";
    imperative: "~ыми~ь", "~ымите~ь";
    participle: _, "~ъявший~я", _, _;
    gerund: _, "~ъявши~ь";
    ),
    // Уеть: 4 words (выеть, поеть, заеть, уеть). Like Грести, except in 4 cells: infinitive, participle pres_act, participle pres_pass, gerund pres.
    conjugation!("уеть", 4, "~еть", [aspect: Perfective], like Грести;
    infinitive: "~еть";
    participle: derived, _, derived, _;
    gerund: derived, _;
    ),
    // Доебсти: 4 words (уебсти, выебсти, поебсти, доебсти). Like Снести, except in 2 cells: infinitive, gerund past.
    conjugation!("доебсти", 4, "~е~сти", [aspect: Perfective], like Снести;
    infinitive: "~е~сти";
    gerund: _, "~ё~ши";
    ),
    // Помочь: 3 words (выстричь, выпрячь, вылечь). Like Постричь, except in 4 cells: non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg.
    conjugation!("помочь", 3, "~чь", [aspect: Perfective], like Постричь as ACT_AS_PASS;
    non_past_1: _, "~жем";
    non_past_2: "~жешь", "~жете";
    non_past_3: "~жет", _;
    ),
    // Хотеть: 3 words (хотеть, захотеть, расхотеть). Like Бухтеть, except in 6 cells: non_past_2 sg, non_past_3 sg, participle pres_act, participle pres_pass, participle past_pass, gerund pres.
    conjugation!("хотеть", 3, "~теть", like Бухтеть;
    non_past_2: "~чешь", _;
    non_past_3: "~чет", _;
    participle: derived, _, derived, derived;
    gerund: derived, _;
    ),
    // Хотеться: 3 words (хотеться, захотеться, расхотеться). Like Разлететься, except in 3 cells: non_past_2 sg, non_past_3 sg, gerund pres.
    conjugation!("хотеться", 3, "~теть~я", like Разлететься as PRES_AS_PAST;
    non_past_2: "~чешь~я", _;
    non_past_3: "~чет~я", _;
    gerund: derived, _;
    ),
    // Вырасти: 3 words (вырасти, повырасти, подвырасти). Like Дорасти, except in 2 cells: participle pres_act, gerund pres.
    conjugation!("вырасти", 3, "~а~ти", [aspect: Perfective], like Дорасти as PRES_AS_PAST;
    stem non_past: "~а~т- like заглохнуть";
    participle: derived, _, _, _;
    gerund: derived, _;
    ),
    // Высвободить: 3 words (вынудить, высвободить, выродить). Like Разводить, except in 4 cells: participle pres_act, participle pres_pass, participle past_pass, gerund pres.
    conjugation!("высвободить", 3, "~дить", [aspect: Perfective], like Разводить;
    participle: derived, _, derived, "~жденный";
    gerund: derived, _;
    ),
    // Начаться: 3 words (начаться, початься, зачаться). Like Мяться, except in 7 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("начаться", 3, "~ать~я", [aspect: Perfective], like Мяться;
    infinitive: "~ать~я";
    past: "~ал~я", "~ала~ь", "~ало~ь", "~али~ь";
    participle: _, "~авший~я", _, _;
    gerund: _, "~авши~ь";
    ),
    // Тереть: 3 words (тереть, мереть, переть). Like Затереть, except in 1 cell: gerund past.
    conjugation!("тереть", 3, "~е~еть", [aspect: Imperfective], like Затереть;
    stem non_past: "~~- like рвануть";
    gerund: _, "~ё~ши";
    ),
    // Опоясать: 3 words (опоясать, запоясать, распоясать). Like Выписать, except in 2 cells: participle pres_act, gerund pres.
    conjugation!("опоясать", 3, "~сать", [aspect: Perfective], like Выписать as ACT_AS_PASS;
    stem non_past: "~ш- like подкинуть";
    participle: derived, _, _, _;
    gerund: derived, _;
    ),
    // Вянуть: 3 words (вянуть, завянуть, увянуть). Like Стынуть, except in 2 cells: participle past_act, gerund past.
    conjugation!("вянуть", 3, "~нуть", like Стынуть;
    participle: _, "~нувший", _, _;
    gerund: _, "~нув|~нувши";
    ),
    // Заклясть: 3 words (проклясть, клясть, заклясть). Like Помять, except in 2 cells: infinitive, participle pres_pass.
    conjugation!("заклясть", 3, "~сть", like Помять;
    stem past: "~- like раздуть";
    infinitive: "~сть";
    participle: _, _, "~нимый", _;
    ),
    // Клясться: 3 words (клясться, поклясться, заклясться). Like Мяться, except in 9 cells: infinitive, past m, past f, past n, past pl, participle pres_act, ….
    conjugation!("клясться", 3, "~~ться", like Мяться;
    infinitive: "~~ться";
    past: "~л~я|~л~я", "~ла~ь", "~ло~ь|~ло~ь", "~ли~ь|~ли~ь";
    participle: "~нущий~я", "~вший~я", _, _;
    gerund: "~ня~ь", "~вши~ь";
    ),
    // Приняться: 3 words (приняться, восприняться, предприняться). Like Уняться, except in 9 cells: non_past_1 sg, non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg, non_past_3 pl, ….
    conjugation!("приняться", 3, "~нять~я", [aspect: Perfective], like Уняться;
    non_past_1: "~му~ь", "~мем~я";
    non_past_2: "~мешь~я", "~мете~ь";
    non_past_3: "~мет~я", "~мут~я";
    past: "~нял~я", _, _, _;
    imperative: "~ми~ь", "~мите~ь";
    ),
    // Вывести: 3 words (вывести, выбрести, повывести). Like Выпасть, except in 3 cells: infinitive, participle past_act, gerund past.
    conjugation!("вывести", 3, "~сти", [aspect: Perfective], like Выпасть;
    infinitive: "~сти";
    participle: _, "~дший", _, _;
    gerund: _, "~дя";
    ),
    // Выбраться: 3 words (выбраться, выдраться, повыбраться). Like Стлаться, except in 3 cells: non_past_1 sg, non_past_3 pl, gerund pres.
    conjugation!("выбраться", 3, "~~ать~я", [aspect: Perfective], like Стлаться as PRES_AS_PAST;
    non_past_1: "~е~у~ь", _;
    non_past_3: _, "~е~ут~я";
    gerund: derived, _;
    ),
    // Вылезть: 3 words (вылезть, выгрызть, отверзть). Like Лезть, except in 2 cells: imperative sg, gerund pres.
    conjugation!("вылезть", 3, "~ть", [aspect: Perfective], like Лезть as PRES_AS_PAST;
    imperative: "~и", _;
    gerund: derived, _;
    ),
    // Счесть: 3 words (счесть, обчесть, отчесть).
    conjugation!("счесть", 3, "~~есть", [aspect: Perfective];
    stem non_past: "~о~т- like перевернуть";
    infinitive: "~~есть";
    past: "~~ёл", "~о~ла", "~о~ло", "~о~ли";
    participle: _, _, _, "~о~тённый";
    gerund: _, "~о~тя|~~ётши";
    ),
    // Обмереть: 3 words (отпереть, обмереть, отмереть). Like Стереть, except in 0 cells: .
    conjugation!("обмереть", 3, "~~е~еть", [aspect: Perfective], like Стереть;
    stem past: "~~е~- like обмереть";
    ),
    // Понудить: 3 words (принудить, понудить, попринудить). Like Высвободить, except in 3 cells: imperative sg, imperative pl, participle past_pass.
    conjugation!("понудить", 3, "~дить", [aspect: Perfective], like Высвободить;
    imperative: "~дь", "~дьте";
    participle: _, _, _, "~ждённый";
    ),
    // Отозвать: 3 words (отозвать, обозвать, подозвать). Like Разогнать, except in 0 cells: .
    conjugation!("отозвать", 3, "~о~~ать", [aspect: Perfective], like Разогнать;
    stem non_past: "~~о~- like перевернуть";
    ),
    // Помутить: 3 words (замутить, помутить, намутить). Like Зачертить, except in 1 cell: participle past_pass.
    conjugation!("помутить", 3, "~тить", [aspect: Perfective], like Зачертить;
    participle: _, _, _, "~тнённый";
    ),
    // Обглодать: 3 words (глодать, обглодать, сглодать).
    conjugation!("обглодать", 3, "~дать";
    stem past: "~да- like играть";
    infinitive: "~дать";
    non_past_1: "~жу|~даю", "~жем|~даем";
    non_past_2: "~жешь|~даешь", "~жете|~даете";
    non_past_3: "~жет|~дает", "~жут|~дают";
    imperative: "~жи|~дай", "~жите|~дайте";
    participle: "~жущий", _, "~даемый", _;
    gerund: "~жа", _;
    ),
    // Выгрести: 3 words (выгрести, выскрести, повыгрести).
    conjugation!("выгрести", 3, "~сти", [aspect: Perfective];
    stem non_past: "~б- like заглохнуть";
    stem past: "~б- like грызть";
    infinitive: "~сти";
    ),
    // Приобрестись: 3 words (приобрестись, обрестись, вплестись). Like Сплестись, except in 1 cell: participle past_act.
    conjugation!("приобрестись", 3, "~е~тись", [aspect: Perfective], like Сплестись;
    participle: _, "~етший~я", _, _;
    ),
    // Пролежать: 3 words (пролежать, отлежать, облежать). Like Взъебать, except in 0 cells: .
    conjugation!("пролежать", 3, "~е~ать", like Взъебать;
    stem non_past: "~е~- like стучать";
    ),
    // Осечься: 3 words (пересечься, осечься, сечься). Like Влечься, except in 3 cells: past m, participle past_act, gerund past.
    conjugation!("осечься", 3, "~ечь~я", like Влечься;
    past: "~ёк~я|~ек~я", _, _, _;
    participle: _, "~екший~я|~ёкший~я", _, _;
    gerund: _, "~екши~ь|~ёкши~ь";
    ),
    // Изыти: 3 words (снити, изыти, внити). Like Вывести, except in 6 cells: infinitive, past m, past f, past n, past pl, participle past_act.
    conjugation!("изыти", 3, "~ти", [aspect: Perfective], like Вывести as PAST_AS_PRES;
    infinitive: "~ти";
    past: "~шел", "~шла", "~шло", "~шли";
    participle: _, "~шедший", _, _;
    ),
    // Наморщиться: 3 words (встопорщиться, наморщиться, изморщиться). Like Выпариться, except in 2 cells: non_past_1 sg, gerund past.
    conjugation!("наморщиться", 3, "~ить~я", [aspect: Perfective], like Выпариться;
    non_past_1: "~у~ь", _;
    gerund: _, "~ивши~ь";
    ),
    // Чтиться: 3 words (чтиться, почтиться, узриться). Like Числиться, except in 2 cells: non_past_1 sg, gerund past.
    conjugation!("чтиться", 3, "~ить~я", like Числиться;
    non_past_1: "~у~ь", _;
    gerund: _, "~ивши~ь|~я~ь";
    ),
    // Взбресть: 3 words (взбресть, добресть, набресть). Like Развести, except in 2 cells: infinitive, gerund past.
    conjugation!("взбресть", 3, "~есть", [aspect: Perfective], like Развести as PAST_AS_PRES;
    infinitive: "~есть";
    gerund: _, "~едя";
    ),
    // Выпечься: 3 words (выволочься, выпечься, высечься). Like Заволочься, except in 4 cells: non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg.
    conjugation!("выпечься", 3, "~чь~я", [aspect: Perfective], like Заволочься as PRES_AS_PAST;
    non_past_1: _, "~чем~я";
    non_past_2: "~чешь~я", "~чете~ь";
    non_past_3: "~чет~я", _;
    ),
    // Мястись: 3 words (мястись, выплестись, выместись). Like Зачесться, except in 7 cells: infinitive, non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg, past m, ….
    conjugation!("мястись", 3, "~~тись", like Зачесться;
    infinitive: "~~тись";
    non_past_1: _, "~тем~я";
    non_past_2: "~тешь~я", "~тете~ь";
    non_past_3: "~тет~я", _;
    past: "~л~я", _, _, _;
    participle: _, "~тший~я", _, _;
    ),
    // Испиться: 3 words (распиться, расшиться, испиться).
    conjugation!("испиться", 3, "~с~ить~я", [aspect: Perfective];
    infinitive: "~с~ить~я";
    non_past_1: "~зо~ью~ь", "~зо~ьём~я";
    non_past_2: "~зо~ьёшь~я", "~зо~ьёте~ь";
    non_past_3: "~зо~ьёт~я", "~зо~ьют~я";
    past: "~с~ил~я", "~с~ила~ь", "~с~ило~ь", "~с~или~ь";
    imperative: "~с~ей~я", "~с~ейте~ь";
    participle: _, "~с~ивший~я", _, _;
    gerund: _, "~с~ивши~ь";
    ),
    // Измождать: 3 words (измождать, повозвращать, прокажать).
    conjugation!("измождать", 3, "~ать";
    stem non_past: "~а- like играть";
    infinitive: "~ать";
    past: "~ал", "~ала", "~ало", "~али";
    participle: _, "~авший", _, "~ённый";
    gerund: _, "~ав|~авши";
    ),
    // Позаводить: 3 words (самовоспроизводить, напереводить, позаводить).
    conjugation!("позаводить", 3, "~одить", [aspect: Perfective];
    stem non_past: "~о- like пересидеть";
    infinitive: "~одить";
    past: "~одил", "~одила", "~одило", "~одили";
    participle: _, "~одивший", _, "~едённый";
    gerund: _, "~одив|~одивши";
    ),
    // Закодить: 3 words (повыводить, закодить, накодить). Like Высвободить, except in 0 cells: .
    conjugation!("закодить", 3, "~дить", [aspect: Perfective], like Высвободить;
    stem past: "~д- like ударить";
    ),
    // Харассить: 3 words (оглонассить, харассить, глонассить).
    conjugation!("харассить", 3, "~ссить";
    infinitive: "~ссить";
    non_past_1: "~шу", "~ссим";
    non_past_2: "~ссишь", "~ссите";
    non_past_3: "~ссит", "~ссят";
    past: "~ссил", "~ссила", "~ссило", "~ссили";
    imperative: "~ссь", "~ссьте";
    participle: "~ссящий", "~ссивший", "~ссимый", "~шенный";
    gerund: "~сся", "~ссив|~ссивши";
    ),
    // Рещи: 1 words (рещи). Like Обтечь, except in 7 cells: infinitive, non_past_1 sg, non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg, ….
    conjugation!("рещи", 1, "~ещи", like Обтечь;
    infinitive: "~ещи";
    non_past_1: derived, derived;
    non_past_2: derived, derived;
    non_past_3: derived, derived;
    ),
];
