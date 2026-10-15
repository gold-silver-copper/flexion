//! The conjugations, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first. A `%`
//! is the first letter of the stem piece after it (reduplication: `%ε~κα`),
//! as the copy laws in `laws.rs` change it. A `ˇ` ends
//! the patterns of a conjugation whose stems alternate: the laws in `laws.rs`
//! for alternating stems change them (`~nˇ`: `katu`, `kadun`).

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
    ($name:literal, $words:literal, $lemma:literal $(, [$($f:ident: $v:ident),*])? $(, like $parent:ident $(as $view:ident)? $(with $affix:literal $(, $other:literal after $letters:literal)?)?)?;
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
            affix: affix!($($($affix $(, $other, $letters)?)?)?),
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

/// What a conjugation adds after each pattern of the one it is like (`with "~ь", "~я"
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

/// An inherent feature's value, as `(feature, value)` by their places.
macro_rules! feature {
    (aspect: $v:ident) => {
        (0, crate::Aspect::$v as u8)
    };
}

/// Every conjugation learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Conjugation {
    /// Like `играть`: 11,634 words.
    Играть,
    /// Like `набираться`: 8,293 words.
    Набираться,
    /// Like `вибрировать`: 3,326 words.
    Вибрировать,
    /// Like `стыковаться`: 2,209 words.
    Стыковаться,
    /// Like `ударить`: 1,983 words.
    Ударить,
    /// Like `палить`: 1,320 words.
    Палить,
    /// Like `числиться`: 1,171 words.
    Числиться,
    /// Like `раздуть`: 927 words.
    Раздуть,
    /// Like `завибрировать`: 879 words.
    Завибрировать,
    /// Like `рвануть`: 688 words.
    Рвануть,
    /// Like `выделить`: 614 words.
    Выделить,
    /// Like `усилиться`: 566 words.
    Усилиться,
    /// Like `тянуть`: 540 words.
    Тянуть,
    /// Like `обнажиться`: 501 words.
    Обнажиться,
    /// Like `любиться`: 454 words.
    Любиться,
    /// Like `обернуться`: 392 words.
    Обернуться,
    /// Like `обрушиться`: 343 words.
    Обрушиться,
    /// Like `восстановить`: 330 words.
    Восстановить,
    /// Like `разводить`: 327 words.
    Разводить,
    /// Like `долбить`: 323 words.
    Долбить,
    /// Like `хватить`: 297 words.
    Хватить,
    /// Like `расправиться`: 296 words.
    Расправиться,
    /// Like `представить`: 290 words.
    Представить,
    /// Like `расходиться`: 283 words.
    Расходиться,
    /// Like `закатиться`: 232 words.
    Закатиться,
    /// Like `тянуться`: 230 words.
    Тянуться,
    /// Like `залипнуть`: 203 words.
    Залипнуть,
    /// Like `сносить`: 186 words.
    Сносить,
    /// Like `отметить`: 164 words.
    Отметить,
    /// Like `распуститься`: 163 words.
    Распуститься,
    /// Like `стучать`: 152 words.
    Стучать,
    /// Like `носиться`: 138 words.
    Носиться,
    /// Like `забросить`: 125 words.
    Забросить,
    /// Like `засосать`: 121 words.
    Засосать,
    /// Like `встретиться`: 120 words.
    Встретиться,
    /// Like `оказать`: 119 words.
    Оказать,
    /// Like `топтать`: 117 words.
    Топтать,
    /// Like `развеять`: 113 words.
    Развеять,
    /// Like `возместить`: 113 words.
    Возместить,
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
    /// Like `защитить`: 78 words.
    Защитить,
    /// Like `отдавать`: 77 words.
    Отдавать,
    /// Like `развеяться`: 73 words.
    Развеяться,
    /// Like `коптить`: 72 words.
    Коптить,
    /// Like `сычевать`: 71 words.
    Сычевать,
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
    /// Like `драть`: 51 words.
    Драть,
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
    /// Like `трещать`: 43 words.
    Трещать,
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
    /// Like `понять`: 20 words.
    Понять,
    /// Like `отнять`: 19 words.
    Отнять,
    /// Like `лечь`: 19 words.
    Лечь,
    /// Like `плескаться`: 19 words.
    Плескаться,
    /// Like `трепетать`: 19 words.
    Трепетать,
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
    /// Like `лезть`: 17 words.
    Лезть,
    /// Like `жать`: 17 words.
    Жать,
    /// Like `взъебать`: 17 words.
    Взъебать,
    /// Like `колыхаться`: 17 words.
    Колыхаться,
    /// Like `гнать`: 17 words.
    Гнать,
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
    /// Like `просвистать`: 12 words.
    Просвистать,
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
    /// Like `назвать`: 10 words.
    Назвать,
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
    /// Like `стонать`: 6 words.
    Стонать,
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
    pub const ALL: [Conjugation; 341] = [
        Conjugation::Играть,
        Conjugation::Набираться,
        Conjugation::Вибрировать,
        Conjugation::Стыковаться,
        Conjugation::Ударить,
        Conjugation::Палить,
        Conjugation::Числиться,
        Conjugation::Раздуть,
        Conjugation::Завибрировать,
        Conjugation::Рвануть,
        Conjugation::Выделить,
        Conjugation::Усилиться,
        Conjugation::Тянуть,
        Conjugation::Обнажиться,
        Conjugation::Любиться,
        Conjugation::Обернуться,
        Conjugation::Обрушиться,
        Conjugation::Восстановить,
        Conjugation::Разводить,
        Conjugation::Долбить,
        Conjugation::Хватить,
        Conjugation::Расправиться,
        Conjugation::Представить,
        Conjugation::Расходиться,
        Conjugation::Закатиться,
        Conjugation::Тянуться,
        Conjugation::Залипнуть,
        Conjugation::Сносить,
        Conjugation::Отметить,
        Conjugation::Распуститься,
        Conjugation::Стучать,
        Conjugation::Носиться,
        Conjugation::Забросить,
        Conjugation::Засосать,
        Conjugation::Встретиться,
        Conjugation::Оказать,
        Conjugation::Топтать,
        Conjugation::Развеять,
        Conjugation::Возместить,
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
        Conjugation::Защитить,
        Conjugation::Отдавать,
        Conjugation::Развеяться,
        Conjugation::Коптить,
        Conjugation::Сычевать,
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
        Conjugation::Драть,
        Conjugation::Идти,
        Conjugation::Глядеться,
        Conjugation::Развести,
        Conjugation::Разбить,
        Conjugation::Клевать,
        Conjugation::Снести,
        Conjugation::Трещать,
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
        Conjugation::Понять,
        Conjugation::Отнять,
        Conjugation::Лечь,
        Conjugation::Плескаться,
        Conjugation::Трепетать,
        Conjugation::Предназначить,
        Conjugation::Просмотреть,
        Conjugation::Въесться,
        Conjugation::Потчевать,
        Conjugation::Устояться,
        Conjugation::Лезть,
        Conjugation::Жать,
        Conjugation::Взъебать,
        Conjugation::Колыхаться,
        Conjugation::Гнать,
        Conjugation::Вгрызться,
        Conjugation::Постричь,
        Conjugation::Разобрать,
        Conjugation::Выровнять,
        Conjugation::Трепать,
        Conjugation::Заволочься,
        Conjugation::Насесть,
        Conjugation::Смеяться,
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
        Conjugation::Просвистать,
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
        Conjugation::Стелиться,
        Conjugation::Заспать,
        Conjugation::Ушибиться,
        Conjugation::Жаться,
        Conjugation::Пакоститься,
        Conjugation::Дожить,
        Conjugation::Зассать,
        Conjugation::Зачесться,
        Conjugation::Назвать,
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
        Conjugation::Стереть,
        Conjugation::Выспаться,
        Conjugation::Размять,
        Conjugation::Толочься,
        Conjugation::Скрестись,
        Conjugation::Зажечься,
        Conjugation::Мести,
        Conjugation::Разойтись,
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
        Conjugation::Стонать,
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

pub(super) static CONJUGATIONS: [Paradigm<'static>; 341] = [
    // Играть: 11,634 words (знать, играть, презирать, делать, уступать, скучать).
    conjugation!("играть", 11634, "~ть";
    stem non_past: "~- like играть";
    stem past: "~- like играть";
    infinitive: "~ть";
    ),
    // Набираться: 8,293 words (отпочковываться, изъясняться, облокачиваться, кончаться, являться, сморкаться). Like Играть with `~ь` after each pattern (`~я` after й, л, м, т, ь), except in 3 cells: participle pres_pass, participle past_pass, gerund past.
    conjugation!("набираться", 8293, "~ть~я", like Играть with "~ь", "~я" after "йлмть";
    participle: _, _, derived, derived;
    gerund: _, "~вши~ь";
    ),
    // Вибрировать: 3,326 words (мульчировать, координировать, использовать, чувствовать, почувствовать, целовать).
    conjugation!("вибрировать", 3326, "~овать";
    stem non_past: "~у- like играть";
    stem past: "~ова- like играть";
    infinitive: "~овать";
    ),
    // Стыковаться: 2,209 words (тусоваться, использоваться, чувствоваться, почувствоваться, целоваться, абсорбироваться). Like Вибрировать with `~ь` after each pattern (`~я` after й, л, м, т, ь), except in 3 cells: participle pres_pass, participle past_pass, gerund past.
    conjugation!("стыковаться", 2209, "~овать~я", like Вибрировать with "~ь", "~я" after "йлмть";
    participle: _, _, derived, derived;
    gerund: _, "~овавши~ь";
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
    // Палить: 1,320 words (говорить, поговорить, изменить, благодарить, винить, отменить). Like Ударить, except in 3 cells: imperative sg, imperative pl, participle past_pass.
    conjugation!("палить", 1320, "~ить", like Ударить;
    imperative: "~и", "~ите";
    participle: _, _, _, "~ённый";
    ),
    // Числиться: 1,171 words (изъясниться, соединиться, присоединиться, объясниться, валиться, приземлиться). Like Палить with `~ь` after each pattern (`~я` after й, л, м, т, ь), except in 3 cells: participle pres_pass, participle past_pass, gerund past.
    conjugation!("числиться", 1171, "~ить~я", like Палить with "~ь", "~я" after "йлмть";
    participle: _, _, derived, derived;
    gerund: _, "~ивши~ь";
    ),
    // Раздуть: 927 words (деревенеть, взять, одеревенеть, деревянеть, болеть, краснеть). Like Играть.
    conjugation!("раздуть", 927, "~ть", like Играть;
    stem past: "~- like раздуть";
    ),
    // Завибрировать: 879 words (завибрировать, фехтовать, действовать, путешествовать, кайфовать, следовать).
    conjugation!("завибрировать", 879, "~атьˇ";
    stem non_past: "~у- like завибрировать";
    infinitive: "~атьˇ";
    past: "~алˇ", "~алаˇ", "~алоˇ", "~алиˇ";
    participle: _, "~авшийˇ", _, _;
    gerund: _, "~авˇ|~авшиˇ";
    ),
    // Рвануть: 688 words (взбзднуть, хохотнуть, блевануть, пугнуть, гнуть, ткнуть).
    conjugation!("рвануть", 688, "~уть", [aspect: Perfective];
    stem non_past: "~- like рвануть";
    stem past: "~у- like раздуть";
    infinitive: "~уть";
    ),
    // Выделить: 614 words (учить, курить, покурить, кончить, закончить, варить). Like Палить.
    conjugation!("выделить", 614, "~ить", like Палить;
    stem past: "~- like ударить";
    ),
    // Усилиться: 566 words (ссориться, пялиться, ускориться, стариться, состариться, целиться). Like Ударить with `~ь` after each pattern (`~я` after й, л, м, т, ь), except in 3 cells: participle pres_pass, participle past_pass, gerund past.
    conjugation!("усилиться", 566, "~ить~я", like Ударить with "~ь", "~я" after "йлмть";
    participle: _, _, derived, derived;
    gerund: _, "~ивши~ь|~я~ь";
    ),
    // Тянуть: 540 words (выковырнуть, прыгнуть, пёрнуть, капнуть, шлёпнуть, взглянуть). Like Рвануть.
    conjugation!("тянуть", 540, "~уть", [aspect: Perfective], like Рвануть;
    stem non_past: "~- like тянуть";
    ),
    // Обнажиться: 501 words (учиться, кончиться, страшиться, случиться, научиться, уменьшиться). Like Служить with `~ь` after each pattern (`~я` after й, л, м, т, ь), except in 3 cells: participle pres_pass, participle past_pass, gerund past.
    conjugation!("обнажиться", 501, "~ить~я", like Служить with "~ь", "~я" after "йлмть";
    participle: _, _, derived, derived;
    gerund: _, "~ивши~ь";
    ),
    // Любиться: 454 words (появиться, торопиться, стремиться, влюбиться, удивиться, остановиться). Like Восстановить with `~ь` after each pattern (`~я` after й, л, м, т, ь), except in 3 cells: participle pres_pass, participle past_pass, gerund past.
    conjugation!("любиться", 454, "~ить~я", like Восстановить with "~ь", "~я" after "йлмть";
    participle: _, _, derived, derived;
    gerund: _, "~ивши~ь";
    ),
    // Обернуться: 392 words (рехнуться, повернуться, проснуться, заткнуться, вернуться, улыбнуться). Like Рвануть with `~я` after each pattern (`~ь` after а, е, и, о, у), except in 2 cells: participle past_pass, gerund past.
    conjugation!("обернуться", 392, "~уть~я", [aspect: Perfective], like Рвануть with "~я", "~ь" after "аеиоу";
    participle: _, _, _, derived;
    gerund: _, "~увши~ь";
    ),
    // Обрушиться: 343 words (увеличиться, размножиться, кочевряжиться, дурачиться, спешиться, мучиться). Like Предназначить with `~я` after each pattern (`~ь` after а, е, и, о, у), except in 3 cells: participle pres_pass, participle past_pass, gerund past.
    conjugation!("обрушиться", 343, "~ить~я", like Предназначить with "~я", "~ь" after "аеиоу";
    participle: _, _, derived, derived;
    gerund: _, "~ивши~ь";
    ),
    // Восстановить: 330 words (уступить, любить, полюбить, возлюбить, купить, заломить). Like Палить, except in 2 cells: non_past_1 sg, participle past_pass.
    conjugation!("восстановить", 330, "~ить", like Палить;
    non_past_1: "~лю", _;
    participle: _, _, _, "~ленный";
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
    // Долбить: 323 words (вступить, наступить, дымить, удивить, бомбить, выступить). Like Восстановить, except in 1 cell: participle past_pass.
    conjugation!("долбить", 323, "~ить", like Восстановить;
    participle: _, _, _, "~лённый";
    ),
    // Хватить: 297 words (светить, кутить, крутить, платить, шутить, пошутить).
    conjugation!("хватить", 297, "~тить";
    infinitive: "~тить";
    non_past_1: "~чу", "~тим";
    non_past_2: "~тишь", "~тите";
    non_past_3: "~тит", "~тят";
    past: "~тил", "~тила", "~тило", "~тили";
    imperative: "~ти", "~тите";
    participle: "~тящий", "~тивший", "~тимый", "~ченный";
    gerund: "~тя", "~тив|~тивши";
    ),
    // Расправиться: 296 words (нравиться, понравиться, готовиться, направиться, знакомиться, познакомиться). Like Трафить with `~ь` after each pattern (`~я` after й, л, м, т, ь), except in 2 cells: participle past_pass, gerund past.
    conjugation!("расправиться", 296, "~ить~я", like Трафить with "~ь", "~я" after "йлмть";
    participle: _, _, _, derived;
    gerund: _, "~ивши~ь";
    ),
    // Представить: 290 words (забавить, восславить, готовить, грабить, заставить, править). Like Ударить, except in 2 cells: non_past_1 sg, participle past_pass.
    conjugation!("представить", 290, "~ить", like Ударить;
    non_past_1: "~лю", _;
    participle: _, _, _, "~ленный";
    ),
    // Расходиться: 283 words (находиться, трудиться, садиться, насладиться, разводиться, родиться). Like Разводить with `~ь` after each pattern (`~я` after й, л, м, т, ь), except in 3 cells: participle pres_pass, participle past_pass, gerund past.
    conjugation!("расходиться", 283, "~дить~я", like Разводить with "~ь", "~я" after "йлмть";
    participle: _, _, derived, derived;
    gerund: _, "~дивши~ь";
    ),
    // Закатиться: 232 words (облокотиться, расплатиться, катиться, портиться, суетиться, светиться). Like Хватить with `~ь` after each pattern (`~я` after й, л, м, т, ь), except in 3 cells: participle pres_pass, participle past_pass, gerund past.
    conjugation!("закатиться", 232, "~тить~я", like Хватить with "~ь", "~я" after "йлмть";
    participle: _, _, derived, derived;
    gerund: _, "~тивши~ь";
    ),
    // Тянуться: 230 words (грохнуться, чокнуться, оглянуться, тянуться, потянуться, откликнуться). Like Тянуть with `~я` after each pattern (`~ь` after а, е, и, о, у), except in 2 cells: participle past_pass, gerund past.
    conjugation!("тянуться", 230, "~уть~я", [aspect: Perfective], like Тянуть with "~я", "~ь" after "аеиоу";
    participle: _, _, _, derived;
    gerund: _, "~увши~ь";
    ),
    // Залипнуть: 203 words (распухнуть, сдохнуть, замёрзнуть, высохнуть, погибнуть, окрепнуть).
    conjugation!("залипнуть", 203, "~нуть", [aspect: Perfective];
    stem non_past: "~н- like тянуть";
    stem past: "~- like залипнуть";
    infinitive: "~нуть";
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
    // Отметить: 164 words (ответить, встретить, заметить, утратить, тратить, похитить). Like Хватить, except in 2 cells: imperative sg, imperative pl.
    conjugation!("отметить", 164, "~тить", like Хватить;
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
    // Стучать: 152 words (держать, кричать, жужжать, мычать, лежать, подержать).
    conjugation!("стучать", 152, "~ать";
    stem non_past: "~- like стучать";
    stem past: "~а- like играть";
    infinitive: "~ать";
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
    // Засосать: 121 words (врать, ебать, ржать, заржать, соврать, рвать). Like Стучать, except in 1 cell: participle pres_pass.
    conjugation!("засосать", 121, "~ать", like Стучать;
    stem non_past: "~- like засосать";
    participle: _, _, derived, _;
    ),
    // Встретиться: 120 words (охотиться, заботиться, встретиться, пятиться, отметиться, позаботиться). Like Отметить with `~ь` after each pattern (`~я` after й, л, м, т, ь), except in 3 cells: participle pres_pass, participle past_pass, gerund past.
    conjugation!("встретиться", 120, "~тить~я", like Отметить with "~ь", "~я" after "йлмть";
    participle: _, _, derived, derived;
    gerund: _, "~тивши~ь";
    ),
    // Оказать: 119 words (сказать, рассказать, промазать, показать, связать, резать).
    conjugation!("оказать", 119, "~зать";
    stem non_past: "~ж- like оказать";
    stem past: "~за- like играть";
    infinitive: "~зать";
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
    // Возместить: 113 words (простить, нарастить, поместить, переместить, возместить, известить).
    conjugation!("возместить", 113, "~стить", [aspect: Perfective];
    infinitive: "~стить";
    non_past_1: "~щу", "~стим";
    non_past_2: "~стишь", "~стите";
    non_past_3: "~стит", "~стят";
    past: "~стил", "~стила", "~стило", "~стили";
    imperative: "~сти", "~стите";
    participle: "~стящий", "~стивший", "~стимый", "~щённый";
    gerund: "~стя", "~стив|~стивши";
    ),
    // Перестроиться: 110 words (беспокоиться, успокоиться, приклеиться, строиться, устроиться, настроиться). Like Стоить with `~ь` after each pattern (`~я` after й, л, м, т, ь), except in 3 cells: participle pres_pass, participle past_pass, gerund past.
    conjugation!("перестроиться", 110, "~ить~я", like Стоить with "~ь", "~я" after "йлмть";
    participle: _, _, derived, derived;
    gerund: _, "~ивши~ь";
    ),
    // Держаться: 108 words (воздержаться, мчаться, держаться, послышаться, слышаться, содержаться). Like Стучать with `~ь` after each pattern (`~я` after й, л, м, т, ь), except in 3 cells: participle pres_pass, participle past_pass, gerund past.
    conjugation!("держаться", 108, "~ать~я", like Стучать with "~ь", "~я" after "йлмть";
    participle: _, _, derived, derived;
    gerund: _, "~авши~ь";
    ),
    // Сорваться: 103 words (ебаться, стебаться, усраться, обосраться, дождаться, заебаться). Like Засосать with `~я` after each pattern (`~ь` after а, е, и, у, я), except in 4 cells: past n, past pl, participle past_pass, gerund past.
    conjugation!("сорваться", 103, "~ать~я", like Засосать with "~я", "~ь" after "аеиуя";
    past: _, _, "~ало~ь|~ало~ь", "~али~ь|~али~ь";
    participle: _, _, _, derived;
    gerund: _, "~авши~ь";
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
    // Наладиться: 95 words (ладиться, наладиться, отвадиться, сгрудиться, заладиться, повадиться). Like Наладить with `~ь` after each pattern (`~я` after й, л, м, т, ь), except in 3 cells: participle pres_pass, participle past_pass, gerund past.
    conjugation!("наладиться", 95, "~дить~я", like Наладить with "~ь", "~я" after "йлмть";
    participle: _, _, derived, derived;
    gerund: _, "~дивши~ь";
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
    conjugation!("подкинуть", 88, "~уть", [aspect: Perfective], like Тянуть;
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
    // Выразиться: 81 words (сразиться, возиться, выразиться, выгрузиться, отразиться, грузиться). Like Вывозить with `~ь` after each pattern (`~я` after й, л, м, т, ь), except in 3 cells: participle pres_pass, participle past_pass, gerund past.
    conjugation!("выразиться", 81, "~зить~я", like Вывозить with "~ь", "~я" after "йлмть";
    participle: _, _, derived, derived;
    gerund: _, "~зивши~ь";
    ),
    // Стать: 81 words (стать, устать, застрять, перестать, встать, надеть). Like Залипнуть, except in 2 cells: infinitive, participle pres_act.
    conjugation!("стать", 81, "~ть", [aspect: Perfective], like Залипнуть;
    stem non_past: "~н- like подкинуть";
    stem past: "~- like раздуть";
    infinitive: "~ть";
    participle: derived, _, _, _;
    ),
    // Зарядить: 79 words (щадить, пощадить, опередить, зарядить, отрядить, снарядить). Like Разводить, except in 1 cell: participle past_pass.
    conjugation!("зарядить", 79, "~дить", like Разводить;
    participle: _, _, _, "~жённый";
    ),
    // Защитить: 78 words (запретить, прекратить, крестить, посетить, защитить, сократить). Like Хватить, except in 2 cells: non_past_1 sg, participle past_pass.
    conjugation!("защитить", 78, "~тить", like Хватить;
    non_past_1: "~щу", _;
    participle: _, _, _, "~щённый";
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
    // Развеяться: 73 words (надеяться, каяться, покаяться, маяться, измаяться, отчаяться). Like Развеять with `~ь` after each pattern (`~я` after й, л, м, т, ь), except in 3 cells: participle pres_pass, participle past_pass, gerund past.
    conjugation!("развеяться", 73, "~ять~я", like Развеять with "~ь", "~я" after "йлмть";
    participle: _, _, derived, derived;
    gerund: _, "~явши~ь";
    ),
    // Коптить: 72 words (коптить, сплотить, золотить, закоптить, кипятить, вскипятить). Like Хватить, except in 1 cell: participle past_pass.
    conjugation!("коптить", 72, "~тить", like Хватить;
    participle: _, _, _, "~чённый";
    ),
    // Сычевать: 71 words (бичевать, воевать, завоевать, корчевать, отвоевать, повоевать). Like Вибрировать, except in 1 cell: infinitive.
    conjugation!("сычевать", 71, "~евать", like Вибрировать;
    stem past: "~- like сычевать";
    infinitive: "~евать";
    ),
    // Стушеваться: 71 words (стушеваться, тушеваться, разбушеваться, зарубцеваться, кольцеваться, вытанцеваться). Like Сычевать with `~ь` after each pattern (`~я` after й, л, м, т, ь), except in 3 cells: participle pres_pass, participle past_pass, gerund past.
    conjugation!("стушеваться", 71, "~евать~я", like Сычевать with "~ь", "~я" after "йлмть";
    participle: _, _, derived, derived;
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
    // Сблизиться: 66 words (приблизиться, сконфузиться, близиться, снизиться, сузиться, сблизиться). Like Елозить with `~ь` after each pattern (`~я` after й, л, м, т, ь), except in 3 cells: participle pres_pass, participle past_pass, gerund past.
    conjugation!("сблизиться", 66, "~зить~я", like Елозить with "~ь", "~я" after "йлмть";
    participle: _, _, derived, derived;
    gerund: _, "~зивши~ь";
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
    // Сунуться: 57 words (двинуться, прикинуться, кинуться, дотронуться, тронуться, ринуться). Like Подкинуть with `~я` after each pattern (`~ь` after а, е, и, о, у), except in 2 cells: participle past_pass, gerund past.
    conjugation!("сунуться", 57, "~уть~я", [aspect: Perfective], like Подкинуть with "~я", "~ь" after "аеиоу";
    participle: _, _, _, derived;
    gerund: _, "~увши~ь";
    ),
    // Колоть: 55 words (полоть, колоть, пороть, выполоть, прополоть, выпороть).
    conjugation!("колоть", 55, "~оть";
    stem non_past: "~- like колоть";
    stem past: "~о- like раздуть";
    infinitive: "~оть";
    ),
    // Раскрыться: 55 words (мыться, помыться, скрыться, открыться, умыться, срыться). Like Открыть with `~ь` after each pattern (`~я` after й, л, м, т, ь), except in 3 cells: participle pres_pass, participle past_pass, gerund past.
    conjugation!("раскрыться", 55, "~ыть~я", like Открыть with "~ь", "~я" after "йлмть";
    participle: _, _, derived, derived;
    gerund: _, "~ывши~ь";
    ),
    // Заглохнуть: 54 words (достигнуть, проникнуть, возникнуть, затихнуть, подвергнуть, расторгнуть). Like Залипнуть, except in 1 cell: participle pres_act.
    conjugation!("заглохнуть", 54, "~нуть", [aspect: Perfective], like Залипнуть;
    stem past: "~- like заглохнуть";
    participle: derived, _, _, _;
    ),
    // Влечься: 53 words (отречься, увлечься, растечься, отвлечься, стечься, влечься). Like Обтечь with `~ь` after each pattern (`~я` after й, к, м, т, ь), except in 2 cells: participle pres_pass, participle past_pass.
    conjugation!("влечься", 53, "~ечь~я", like Обтечь with "~ь", "~я" after "йкмть";
    participle: _, _, derived, derived;
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
    // Отказаться: 52 words (казаться, оказаться, показаться, отказаться, обязаться, высказаться). Like Оказать with `~ь` after each pattern (`~я` after й, л, м, т, ь), except in 2 cells: participle past_pass, gerund past.
    conjugation!("отказаться", 52, "~зать~я", like Оказать with "~ь", "~я" after "йлмть";
    participle: _, _, _, derived;
    gerund: _, "~завши~ь";
    ),
    // Пасть: 52 words (класть, пропасть, напасть, красть, украсть, пасть).
    conjugation!("пасть", 52, "~сть";
    stem non_past: "~д- like пасть";
    infinitive: "~сть";
    past: "~л", "~ла", "~ло", "~ли";
    participle: _, "~вший", _, "~денный";
    gerund: _, "~в|~вши";
    ),
    // Отдаваться: 51 words (издаваться, оставаться, сдаваться, расставаться, удаваться, раздаваться). Like Отдавать with `~ь` after each pattern (`~я` after й, л, м, т, ь), except in 2 cells: participle pres_pass, gerund past.
    conjugation!("отдаваться", 51, "~вать~я", [aspect: Imperfective], like Отдавать with "~ь", "~я" after "йлмть";
    participle: _, _, derived, _;
    gerund: _, "~вавши~ь";
    ),
    // Драть: 51 words (брать, избрать, собрать, удрать, стлать, постлать).
    conjugation!("драть", 51, "~~ать";
    stem non_past: "~е~- like пасть";
    stem past: "~~а- like играть";
    infinitive: "~~ать";
    ),
    // Идти: 51 words (идти, пасти, спасти, ползти, припасти, упасти). Like Засосать, except in 8 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("идти", 51, "~ти", like Засосать;
    stem non_past: "~- like пасть";
    infinitive: "~ти";
    past: "~", "~ла", "~ло", "~ли";
    participle: _, "~ший", _, "~ённый";
    gerund: _, "~ши";
    ),
    // Глядеться: 48 words (увидеться, видеться, привидеться, вглядеться, наглядеться, оглядеться). Like Сидеть with `~ь` after each pattern (`~я` after й, л, м, т, ь), except in 3 cells: participle pres_pass, participle past_pass, gerund past.
    conjugation!("глядеться", 48, "~деть~я", like Сидеть with "~ь", "~я" after "йлмть";
    participle: _, _, derived, derived;
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
    // Трещать: 43 words (принадлежать, бренчать, дребезжать, трещать, верещать, содержать).
    conjugation!("трещать", 43, "~е~ать";
    stem non_past: "~е~- like стучать";
    infinitive: "~е~ать";
    past: "~е~ал", "~е~ала", "~е~ало", "~е~али";
    participle: _, "~е~авший", _, "~ё~анный";
    gerund: _, "~е~ав";
    ),
    // Колоться: 42 words (бороться, приколоться, расколоться, колоться, напороться, побороться). Like Колоть with `~ь` after each pattern (`~я` after й, л, м, т, ь), except in 2 cells: participle past_pass, gerund past.
    conjugation!("колоться", 42, "~оть~я", like Колоть with "~ь", "~я" after "йлмть";
    participle: _, _, _, derived;
    gerund: _, "~овши~ь";
    ),
    // Топтаться: 42 words (метаться, топтаться, расхохотаться, заметаться, шептаться, потоптаться). Like Топтать with `~ь` after each pattern (`~я` after й, л, м, т, ь), except in 2 cells: participle past_pass, gerund past.
    conjugation!("топтаться", 42, "~тать~я", like Топтать with "~ь", "~я" after "йлмть";
    participle: _, _, _, derived;
    gerund: _, "~тавши~ь";
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
    // Сбиться: 39 words (влиться, сбиться, развиться, взвиться, разбиться, отбиться). Like Разбить with `~я` after each pattern (`~ь` after а, е, и, о, ю), except in 2 cells: participle past_pass, gerund past.
    conjugation!("сбиться", 39, "~~ить~я", [aspect: Perfective], like Разбить with "~я", "~ь" after "аеиою";
    participle: _, _, _, derived;
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
    // Служить: 37 words (сообщить, обобщить, приобщить, истощить, разобщить, переборщить). Like Палить.
    conjugation!("служить", 37, "~ить", like Палить;
    stem non_past: "~- like стучать";
    ),
    // Обратиться: 37 words (превратиться, обратиться, прекратиться, возвратиться, смутиться, возмутиться). Like Защитить with `~ь` after each pattern (`~я` after й, л, м, т, ь), except in 3 cells: participle pres_pass, participle past_pass, gerund past.
    conjugation!("обратиться", 37, "~тить~я", [aspect: Perfective], like Защитить with "~ь", "~я" after "йлмть";
    participle: _, _, derived, derived;
    gerund: _, "~тивши~ь|~тя~ь";
    ),
    // Гореть: 37 words (сгореть, смотреть, гореть, посмотреть, велеть, загореть). Like Прошипеть, except in 1 cell: non_past_1 sg.
    conjugation!("гореть", 37, "~еть", like Прошипеть;
    non_past_1: "~ю", _;
    ),
    // Загореться: 36 words (рассмотреться, смотреться, загореться, предусмотреться, просмотреться, присмотреться). Like Гореть with `~ь` after each pattern (`~я` after й, л, м, т, ь), except in 2 cells: participle pres_pass, gerund past.
    conjugation!("загореться", 36, "~еть~я", like Гореть with "~ь", "~я" after "йлмть";
    participle: _, _, derived, _;
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
    // Трепаться: 34 words (колебаться, поколебаться, трепаться, истрепаться, заколебаться, затрепаться). Like Щипать with `~ь` after each pattern (`~я` after й, л, м, т, ь), except in 3 cells: participle pres_pass, participle past_pass, gerund past.
    conjugation!("трепаться", 34, "~ать~я", like Щипать with "~ь", "~я" after "йлмть";
    participle: _, _, derived, derived;
    gerund: _, "~авши~ь";
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
    // Резаться: 32 words (врезаться, резаться, мазаться, нарезаться, размазаться, измазаться). Like Оказать with `~я` after each pattern (`~ь` after а, е, и, о, у), except in 4 cells: imperative sg, imperative pl, participle past_pass, gerund past.
    conjugation!("резаться", 32, "~зать~я", like Оказать with "~я", "~ь" after "аеиоу";
    imperative: "~жь~я", "~жьте~ь";
    participle: _, _, _, derived;
    gerund: _, "~завши~ь";
    ),
    // Отвоеваться: 32 words (отвоеваться, навоеваться, довоеваться, развоеваться, изгореваться, нагореваться). Like Отвоевать with `~ь` after each pattern (`~я` after й, л, м, т, ь), except in 3 cells: participle pres_pass, participle past_pass, gerund past.
    conjugation!("отвоеваться", 32, "~евать~я", like Отвоевать with "~ь", "~я" after "йлмть";
    participle: _, _, derived, derived;
    gerund: _, "~евавши~ь";
    ),
    // Трястись: 31 words (трястись, пастись, сотрястись, спастись, затрястись, стрястись). Like Идти with `~ь` after each pattern (`~я` after ~, й, м, т, ь), except in 2 cells: participle pres_pass, participle past_pass.
    conjugation!("трястись", 31, "~ти~ь", like Идти with "~ь", "~я" after "~ймть";
    participle: _, _, derived, derived;
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
    // Отпустить: 27 words (вырастить, вымостить, обесчестить, бесчестить, запостить, постить). Like Возместить, except in 1 cell: participle past_pass.
    conjugation!("отпустить", 27, "~стить", like Возместить;
    participle: _, _, _, "~щенный";
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
    // Просвистеть: 26 words (блестеть, свистеть, хрустеть, заблестеть, насвистеть, посвистеть). Like Возместить, except in 2 cells: infinitive, participle past_pass.
    conjugation!("просвистеть", 26, "~стеть", like Возместить;
    stem past: "~сте- like прошипеть";
    infinitive: "~стеть";
    participle: _, _, _, derived;
    ),
    // Терпеться: 26 words (терпеться, натерпеться, обтерпеться, притерпеться, раскипеться, расшуметься). Like Прошипеть with `~ь` after each pattern (`~я` after й, л, м, т, ь), except in 2 cells: participle pres_pass, gerund past.
    conjugation!("терпеться", 26, "~еть~я", like Прошипеть with "~ь", "~я" after "йлмть";
    participle: _, _, derived, _;
    gerund: _, "~евши~ь";
    ),
    // Плеваться: 26 words (плеваться, расплеваться, наклеваться, наплеваться, облеваться, обплеваться). Like Клевать with `~ь` after each pattern (`~я` after й, л, м, т, ь), except in 2 cells: participle past_pass, gerund past.
    conjugation!("плеваться", 26, "~евать~я", like Клевать with "~ь", "~я" after "йлмть";
    participle: _, _, _, derived;
    gerund: _, "~евавши~ь";
    ),
    // Стричься: 26 words (напрячься, достичься, стричься, остричься, обстричься, подстричься). Like Постричь with `~ь` after each pattern (`~я` after г, й, м, т, ь), except in 1 cell: participle past_pass.
    conjugation!("стричься", 26, "~чь~я", [aspect: Perfective], like Постричь with "~ь", "~я" after "гймть";
    participle: _, _, _, derived;
    ),
    // Упереться: 26 words (запереться, упереться, опереться, простереться, тереться, потереться). Like Тереть with `~ь` after each pattern (`~я` after ~, й, м, т, ь), except in 1 cell: participle past_pass.
    conjugation!("упереться", 26, "~е~еть~я", like Тереть with "~ь", "~я" after "~ймть";
    participle: _, _, _, derived;
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
    // Разлететься: 24 words (вертеться, разлететься, завертеться, отвертеться, довертеться, навертеться). Like Бухтеть with `~ь` after each pattern (`~я` after й, л, м, т, ь), except in 3 cells: participle pres_pass, participle past_pass, gerund past.
    conjugation!("разлететься", 24, "~теть~я", like Бухтеть with "~ь", "~я" after "йлмть";
    participle: _, _, derived, derived;
    gerund: _, "~тевши~ь";
    ),
    // Расплыться: 24 words (житься, прижиться, расплыться, ужиться, разжиться, сжиться). Like Дожить with `~я` after each pattern (`~ь` after а, е, и, о, у), except in 3 cells: past pl, participle past_pass, gerund past.
    conjugation!("расплыться", 24, "~ть~я", like Дожить with "~я", "~ь" after "аеиоу";
    past: _, _, _, "~ли~ь";
    participle: _, _, _, derived;
    gerund: _, "~вши~ь";
    ),
    // Вторгнуться: 24 words (вторгнуться, подвергнуться, проникнуться, слипнуться, выдохнуться, смеркнуться). Like Залипнуть with `~ь` after each pattern (`~я` after ~, й, м, т, ь).
    conjugation!("вторгнуться", 24, "~нуть~я", [aspect: Perfective], like Залипнуть with "~ь", "~я" after "~ймть";
    ),
    // Проскочить: 23 words (тащить, вытащить, затащить, потащить, притащить, стащить). Like Служить.
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
    // Отдаться: 21 words (издаться, удаться, сдаться, раздаться, выдаться, даться). Like Сдать with `~я` after each pattern (`~ь` after а, е, и), except in 4 cells: non_past_3 sg, past n, participle past_pass, gerund past.
    conjugation!("отдаться", 21, "~ть~я", [aspect: Perfective], like Сдать with "~я", "~ь" after "аеи";
    non_past_3: "~~тся", _;
    past: _, _, "~ло~ь|~ло~ь", _;
    participle: _, _, _, derived;
    gerund: _, "~вши~ь";
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
    // Унестись: 21 words (произнестись, донестись, отнестись, пронестись, понестись, перенестись). Like Снести with `~ь` after each pattern (`~я` after ~, й, м, т, ь), except in 1 cell: participle past_pass.
    conjugation!("унестись", 21, "~е~ти~ь", [aspect: Perfective], like Снести with "~ь", "~я" after "~ймть";
    participle: _, _, _, derived;
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
    // Расстаться: 20 words (остаться, расстаться, одеться, раздеться, переодеться, достаться). Like Стать with `~я` after each pattern (`~ь` after а, е, и, о, у), except in 2 cells: participle past_pass, gerund past.
    conjugation!("расстаться", 20, "~ть~я", [aspect: Perfective], like Стать with "~я", "~ь" after "аеиоу";
    participle: _, _, _, derived;
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
    // Понять: 20 words (понять, занять, нанять, унять, донять, перенять).
    conjugation!("понять", 20, "~ятьˇ", [aspect: Perfective];
    stem non_past: "~йм- like понять";
    stem past: "~я- like понять";
    infinitive: "~ятьˇ";
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
    // Трепетать: 19 words (клеветать, свистать, трепетать, роптать, скрежетать, затрепетать). Like Обыскать, except in 3 cells: infinitive, participle pres_pass, participle past_pass.
    conjugation!("трепетать", 19, "~тать", like Обыскать;
    stem past: "~та- like прошипеть";
    infinitive: "~тать";
    participle: _, _, derived, derived;
    ),
    // Предназначить: 18 words (встопорщить, вывощить, наплющить, натопорщить, повытащить, поморщить). Like Проскочить, except in 2 cells: imperative sg, imperative pl.
    conjugation!("предназначить", 18, "~ить", [aspect: Perfective], like Проскочить;
    imperative: "~ь", "~ьте";
    ),
    // Просмотреть: 18 words (презреть, рассмотреть, предусмотреть, осмотреть, подсмотреть, присмотреть). Like Гореть, except in 3 cells: participle pres_act, participle pres_pass, gerund pres.
    conjugation!("просмотреть", 18, "~еть", [aspect: Perfective], like Гореть;
    stem past: "~е- like играть";
    participle: derived, _, derived, _;
    gerund: derived, _;
    ),
    // Въесться: 18 words (наесться, въесться, объесться, приесться, взъесться, заесться). Like Отдаться, except in 7 cells: infinitive, non_past_3 pl, past m, past n, past pl, imperative sg, ….
    conjugation!("въесться", 18, "~~ться", like Отдаться;
    infinitive: "~~ться";
    non_past_3: _, "~дят~я";
    past: "~л~я", _, "~ло~ь", "~ли~ь";
    imperative: "~шь~я", "~шьте~ь";
    ),
    // Потчевать: 18 words (потчевать, выкорчевать, вымежевать, запотчевать, напотчевать, отпотчевать). Like Сычевать.
    conjugation!("потчевать", 18, "~евать", like Сычевать;
    stem past: "~ева- like играть";
    ),
    // Устояться: 17 words (бояться, состояться, забояться, побояться, устояться, застояться). Like Стоять with `~ь` after each pattern (`~я` after й, л, м, т, ь), except in 1 cell: gerund past.
    conjugation!("устояться", 17, "~ять~я", like Стоять with "~ь", "~я" after "йлмть";
    gerund: _, "~явши~ь";
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
    // Колыхаться: 17 words (колыхаться, набрехаться, напахаться, отбрехаться, размахаться, выпахаться). Like Пыхать with `~ь` after each pattern (`~я` after й, л, м, т, ь), except in 2 cells: participle past_pass, gerund past.
    conjugation!("колыхаться", 17, "~хать~я", like Пыхать with "~ь", "~я" after "йлмть";
    participle: _, _, _, derived;
    gerund: _, "~хавши~ь";
    ),
    // Гнать: 17 words (гнать, вызвать, догнать, выгнать, погнать, пригнать).
    conjugation!("гнать", 17, "~~ать";
    stem non_past: "~о~- like просмотреть";
    stem past: "~~а- like играть";
    infinitive: "~~ать";
    ),
    // Вгрызться: 17 words (вгрызться, нагрызться, отгрызться, погрызться, разлезться, прогрызться). Like Грызть with `~ь` after each pattern (`~я` after ~, й, м, т, ь), except in 2 cells: participle pres_pass, participle past_pass.
    conjugation!("вгрызться", 17, "~ть~я", like Грызть with "~ь", "~я" after "~ймть";
    participle: _, _, derived, derived;
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
    // Заволочься: 16 words (заволочься, обволочься, взволочься, волочься, вволочься, доволочься). Like Обволочь with `~ь` after each pattern (`~я` after й, к, м, т, ь), except in 1 cell: participle past_pass.
    conjugation!("заволочься", 16, "~чь~я", like Обволочь with "~ь", "~я" after "йкмть";
    participle: _, _, _, derived;
    ),
    // Насесть: 15 words (сесть, засесть, присесть, осесть, подсесть, пересесть).
    conjugation!("насесть", 15, "~есть", [aspect: Perfective];
    stem non_past: "~яд- like подкинуть";
    stem past: "~е- like прошипеть";
    infinitive: "~есть";
    ),
    // Смеяться: 15 words (смеяться, засмеяться, рассмеяться, посмеяться, досмеяться, надсмеяться). Like Осмеять with `~я` after each pattern (`~ь` after а, е, и, о, ю), except in 4 cells: participle pres_act, participle past_pass, gerund pres, gerund past.
    conjugation!("смеяться", 15, "~ять~я", like Осмеять with "~я", "~ь" after "аеиою";
    participle: "~ющий~я", _, _, derived;
    gerund: "~я~ь", "~явши~ь";
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
    // Прискакать: 14 words (скакать, отскакать, заскакать, поскакать, наскакать, прискакать). Like Плакать, except in 1 cell: participle past_pass.
    conjugation!("прискакать", 14, "~кать", like Плакать;
    stem non_past: "~ч- like оказать";
    participle: _, _, _, derived;
    ),
    // Молоться: 14 words (вымолоться, молоться, замолоться, измолоться, намолоться, обмолоться). Like Молоть with `~ь` after each pattern (`~я` after й, м, т, ь), except in 3 cells: past m, participle past_pass, gerund past.
    conjugation!("молоться", 14, "~о~оть~я", like Молоть with "~ь", "~я" after "ймть";
    past: "~оло~~я", _, _, _;
    participle: _, _, _, derived;
    gerund: _, "~о~овши~ь";
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
    // Мяться: 13 words (замяться, мяться, помяться, примяться, вымяться, домяться). Like Помять with `~ь` after each pattern (`~я` after й, л, м, т, ь), except in 4 cells: participle pres_act, participle past_pass, gerund pres, gerund past.
    conjugation!("мяться", 13, "~ять~я", like Помять with "~ь", "~я" after "йлмть";
    participle: derived, _, _, derived;
    gerund: derived, "~явши~ь";
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
    // Тыкаться: 13 words (расплакаться, тыкаться, плакаться, доскакаться, исплакаться, наплакаться). Like Плакать with `~я` after each pattern (`~ь` after а, е, и, о, у), except in 2 cells: participle past_pass, gerund past.
    conjugation!("тыкаться", 13, "~кать~я", like Плакать with "~я", "~ь" after "аеиоу";
    participle: _, _, _, derived;
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
    // Просвистать: 12 words (насвистать, посвистать, просвистать, высвистать, засвистать, освистать). Like Трепетать, except in 1 cell: infinitive.
    conjugation!("просвистать", 12, "~стать", like Трепетать;
    stem past: "~ста- like играть";
    infinitive: "~стать";
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
    // Брить: 12 words (брить, побрить, выбрить, обрить, добрить, забрить). Like Забить.
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
    // Стынуть: 12 words (стынуть, застынуть, остынуть, простынуть, постынуть, настынуть). Like Стать, except in 3 cells: infinitive, participle pres_act, participle past_pass.
    conjugation!("стынуть", 12, "~нуть", like Стать;
    infinitive: "~нуть";
    participle: "~нущий", _, _, derived;
    ),
    // Бриться: 12 words (бриться, побриться, перебриться, выбриться, обриться, пробриться). Like Брить with `~ь` after each pattern (`~я` after й, л, м, т, ь), except in 3 cells: participle pres_pass, participle past_pass, gerund past.
    conjugation!("бриться", 12, "~ить~я", like Брить with "~ь", "~я" after "йлмть";
    participle: _, _, derived, derived;
    gerund: _, "~ивши~ь";
    ),
    // Морщить: 12 words (упорядочить, морщить, выпучить, сморщить, таращить, вытаращить). Like Предназначить, except in 1 cell: imperative sg.
    conjugation!("морщить", 12, "~ить", like Предназначить;
    imperative: "~и|~ь", _;
    ),
    // Сослаться: 12 words (сослаться, выслаться, дослаться, заслаться, ниспослаться, отослаться). Like Отослать with `~ь` after each pattern (`~я` after й, м, т, ь), except in 3 cells: past m, participle past_pass, gerund past.
    conjugation!("сослаться", 12, "~с~ать~я", [aspect: Perfective], like Отослать with "~ь", "~я" after "ймть";
    past: "~сла~~я", _, _, _;
    participle: _, _, _, derived;
    gerund: _, "~с~авши~ь";
    ),
    // Пересидеть: 12 words (просидеть, отсидеть, усидеть, насидеть, пересидеть, засидеть). Like Сидеть, except in 4 cells: participle pres_act, participle pres_pass, participle past_pass, gerund pres.
    conjugation!("пересидеть", 12, "~деть", [aspect: Perfective], like Сидеть;
    participle: derived, _, derived, "~женный";
    gerund: derived, _;
    ),
    // Спеться: 12 words (петься, спеться, допеться, распеться, напеться, воспеться). Like Перепеть with `~ь` after each pattern (`~я` after й, л, м, т, ь), except in 2 cells: participle past_pass, gerund past.
    conjugation!("спеться", 12, "~еть~я", like Перепеть with "~ь", "~я" after "йлмть";
    participle: _, _, _, derived;
    gerund: _, "~евши~ь";
    ),
    // Стереться: 12 words (стереться, отпереться, подтереться, втереться, впереться, обтереться). Like Стереть with `~ь` after each pattern (`~я` after ~, й, м, т, ь), except in 2 cells: participle past_pass, gerund past.
    conjugation!("стереться", 12, "~~е~еть~я", [aspect: Perfective], like Стереть with "~ь", "~я" after "~ймть";
    participle: _, _, _, derived;
    gerund: _, "~~ё~ши~ь";
    ),
    // Подвигнуться: 12 words (свергнуться, разверзнуться, низвергнуться, подвигнуться, сверзнуться, исторгнуться). Like Залипнуть with `~ь` after each pattern (`~я` after м, т, ь), except in 3 cells: past m, participle pres_act, participle past_act.
    conjugation!("подвигнуться", 12, "~нуть~я", [aspect: Perfective], like Залипнуть with "~ь", "~я" after "мть";
    past: derived, _, _, _;
    participle: derived, "~нувший~я|~ший~я", _, _;
    ),
    // Стелиться: 12 words (зиждиться, расстелиться, стелиться, созиждиться, взъемлиться, постелиться). Like Стелить with `~ь` after each pattern (`~я` after й, л, м, т, ь), except in 3 cells: participle pres_pass, participle past_pass, gerund past.
    conjugation!("стелиться", 12, "~ить~я", like Стелить with "~ь", "~я" after "йлмть";
    participle: _, _, derived, derived;
    gerund: _, "~ивши~ь";
    ),
    // Заспать: 11 words (спать, поспать, проспать, переспать, заспать, отоспать). Like Стучать, except in 5 cells: non_past_1 sg, non_past_3 pl, participle pres_act, participle pres_pass, gerund pres.
    conjugation!("заспать", 11, "~ать", like Стучать;
    non_past_1: "~лю", _;
    non_past_3: _, "~ят";
    participle: derived, _, derived, _;
    gerund: derived, _;
    ),
    // Ушибиться: 11 words (ошибиться, ушибиться, зашибиться, прошибиться, вышибиться, пришибиться). Like Зашибить with `~ь` after each pattern (`~я` after ~, й, м, т, ь), except in 2 cells: participle past_pass, gerund past.
    conjugation!("ушибиться", 11, "~ить~я", [aspect: Perfective], like Зашибить with "~ь", "~я" after "~ймть";
    participle: _, _, _, derived;
    gerund: _, "~ивши~ь";
    ),
    // Жаться: 11 words (прижаться, жаться, пожаться, прожаться, ужаться, выжаться). Like Жать with `~ь` after each pattern (`~я` after й, л, м, т, ь), except in 2 cells: participle past_pass, gerund past.
    conjugation!("жаться", 11, "~ать~я", like Жать with "~ь", "~я" after "йлмть";
    participle: _, _, _, derived;
    gerund: _, "~авши~ь";
    ),
    // Пакоститься: 11 words (заневеститься, засовеститься, невеститься, скапуститься, испакоститься, чихвоститься). Like Распуститься, except in 2 cells: imperative sg, imperative pl.
    conjugation!("пакоститься", 11, "~~титься", like Распуститься;
    imperative: "~~ться", "~~тьтесь";
    ),
    // Дожить: 11 words (выжить, прожить, пережить, дожить, нажить, изжить). Like Наплыть, except in 6 cells: past m, past n, past pl, participle pres_act, participle past_pass, gerund pres.
    conjugation!("дожить", 11, "~ть", [aspect: Perfective], like Наплыть;
    past: "~л|~л", _, "~ло|~ло", "~ли|~ли";
    participle: derived, _, _, "~тый|~тый";
    gerund: derived, _;
    ),
    // Зассать: 11 words (ссать, сцать, обоссать, зассать, поссать, проссать). Like Засосать, except in 8 cells: non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg, imperative sg, imperative pl, ….
    conjugation!("зассать", 11, "~ать", like Засосать;
    non_past_1: _, "~ым";
    non_past_2: "~ышь", "~ыте";
    non_past_3: "~ыт", _;
    imperative: "~ы", "~ыте";
    participle: derived, _, _, _;
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
    // Назвать: 10 words (звать, позвать, назвать, призвать, прозвать, созвать). Like Гнать.
    conjugation!("назвать", 10, "~~ать", like Гнать;
    stem non_past: "~о~- like засосать";
    ),
    // Стоять: 10 words (стоять, настоять, постоять, состоять, противостоять, предстоять). Like Развеять, except in 8 cells: non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg, non_past_3 pl, participle pres_act, ….
    conjugation!("стоять", 10, "~ять", like Развеять;
    non_past_1: _, "~им";
    non_past_2: "~ишь", "~ите";
    non_past_3: "~ит", "~ят";
    participle: "~ящий", _, derived, derived;
    ),
    // Беречься: 10 words (беречься, остеречься, поостеречься, постеречься, стеречься, уберечься). Like Сберечь with `~ь` after each pattern (`~я` after г, й, м, т, ь), except in 2 cells: participle pres_pass, participle past_pass.
    conjugation!("беречься", 10, "~ечь~я", like Сберечь with "~ь", "~я" after "гймть";
    participle: _, _, derived, derived;
    ),
    // Вырвать: 10 words (выебать, вырвать, выждать, высосать, выжрать, выткать). Like Засосать, except in 2 cells: participle pres_act, gerund pres.
    conjugation!("вырвать", 10, "~ать", [aspect: Perfective], like Засосать;
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
    // Забыться: 10 words (забыться, сбыться, добыться, избыться, перезабыться, подзабыться). Like Забыть with `~я` after each pattern (`~ь` after а, е, и, о, у), except in 2 cells: participle past_pass, gerund past.
    conjugation!("забыться", 10, "~ыть~я", [aspect: Perfective], like Забыть with "~я", "~ь" after "аеиоу";
    participle: _, _, _, derived;
    gerund: _, "~ывши~ь";
    ),
    // Нестись: 10 words (нестись, везтись, повезтись, свезтись, перевезтись, наебтись). Like Везти with `~ь` after each pattern (`~я` after ~, й, м, т, ь), except in 2 cells: participle pres_pass, participle past_pass.
    conjugation!("нестись", 10, "~е~ти~ь", like Везти with "~ь", "~я" after "~ймть";
    participle: _, _, derived, derived;
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
    // Свянуть: 10 words (обвянуть, повянуть, привянуть, призавянуть, отвянуть, подвянуть). Like Стынуть, except in 3 cells: participle pres_act, participle past_act, gerund past.
    conjugation!("свянуть", 10, "~нуть", [aspect: Perfective], like Стынуть;
    participle: derived, "~нувший", _, _;
    gerund: _, "~нувши";
    ),
    // Жеваться: 10 words (нажеваться, жеваться, дожеваться, зажеваться, пережеваться, прожеваться). Like Стушеваться, except in 4 cells: non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg.
    conjugation!("жеваться", 10, "~евать~я", like Стушеваться;
    non_past_1: _, "~уём~я";
    non_past_2: "~уёшь~я", "~уёте~ь";
    non_past_3: "~уёт~я", _;
    ),
    // Обидеться: 10 words (обидеться, навидеться, разобидеться, изобидеться, завидеться, ненавидеться). Like Видеть with `~ь` after each pattern (`~я` after й, л, м, т, ь), except in 3 cells: participle pres_pass, participle past_pass, gerund past.
    conjugation!("обидеться", 10, "~деть~я", like Видеть with "~ь", "~я" after "йлмть";
    participle: _, _, derived, derived;
    gerund: _, "~девши~ь";
    ),
    // Вырваться: 10 words (вырваться, выебаться, высраться, выткаться, закармливаться, высосаться). Like Стонать with `~ь` after each pattern (`~я` after й, л, м, т, ь), except in 1 cell: gerund past.
    conjugation!("вырваться", 10, "~ать~я", like Стонать with "~ь", "~я" after "йлмть";
    gerund: _, "~авши~ь";
    ),
    // Стереть: 9 words (стереть, подпереть, спереть, обтереть, втереть, впереть).
    conjugation!("стереть", 9, "~~е~еть", [aspect: Perfective];
    stem non_past: "~о~~- like перевернуть";
    infinitive: "~~е~еть";
    past: "~~ё~", "~~ё~ла", "~~ё~ло", "~~ё~ли";
    participle: _, "~~ё~ший", _, "~~ё~тый";
    gerund: _, "~~е~ев|~~ё~ши";
    ),
    // Выспаться: 9 words (выспаться, отоспаться, доспаться, заспаться, наспаться, проспаться). Like Заспать with `~ь` after each pattern (`~я` after й, л, м, т, ь), except in 2 cells: participle past_pass, gerund past.
    conjugation!("выспаться", 9, "~ать~я", [aspect: Perfective], like Заспать with "~ь", "~я" after "йлмть";
    participle: _, _, _, derived;
    gerund: _, "~авши~ь";
    ),
    // Размять: 9 words (смять, измять, размять, подмять, вмять, обмять).
    conjugation!("размять", 9, "~~ять", [aspect: Perfective];
    stem non_past: "~о~н- like перевернуть";
    stem past: "~~я- like раздуть";
    infinitive: "~~ять";
    ),
    // Толочься: 9 words (толочься, затолочься, потолочься, истолочься, дотолочься, утолочься). Like Толочь with `~ь` after each pattern (`~я` after й, к, м, т, ь), except in 1 cell: participle past_pass.
    conjugation!("толочься", 9, "~очь~я", like Толочь with "~ь", "~я" after "йкмть";
    participle: _, _, _, derived;
    ),
    // Скрестись: 9 words (скрестись, грестись, огрестись, догрестись, угрестись, погрестись). Like Отгрестись, except in 3 cells: participle pres_act, gerund pres, gerund past.
    conjugation!("скрестись", 9, "~е~тись", like Отгрестись;
    participle: "~ебущий~я", _, _, _;
    gerund: "~ебя~ь", "~ёбши~ь";
    ),
    // Зажечься: 9 words (зажечься, ожечься, нажечься, жечься, возжечься, пережечься). Like Прожечь with `~ь` after each pattern (`~я` after г, й, м, т, ь), except in 1 cell: participle past_pass.
    conjugation!("зажечься", 9, "~ечь~я", like Прожечь with "~ь", "~я" after "гймть";
    participle: _, _, _, derived;
    ),
    // Мести: 9 words (гнести, плести, мести, цвести, расцвести, рассвести). Like Сплести, except in 1 cell: gerund past.
    conjugation!("мести", 9, "~ести", like Сплести;
    stem non_past: "~ет- like засосать";
    gerund: _, "~ётши";
    ),
    // Разойтись: 9 words (найтись, прийтись, обойтись, пройтись, разойтись, сойтись). Like Зайти with `~ь` after each pattern (`~я` after й, л, м, т, ь), except in 2 cells: participle past_pass, gerund past.
    conjugation!("разойтись", 9, "~йти~ь", [aspect: Perfective], like Зайти with "~ь", "~я" after "йлмть";
    participle: _, _, _, derived;
    gerund: _, "~шедши~ь";
    ),
    // Выпить: 8 words (выпить, вышить, выбить, вылить, подвыпить, вывить). Like Забить, except in 6 cells: non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg, participle pres_act, gerund pres.
    conjugation!("выпить", 8, "~ить", [aspect: Perfective], like Забить;
    non_past_1: _, "~ьем";
    non_past_2: "~ьешь", "~ьете";
    non_past_3: "~ьет", _;
    participle: derived, _, _, _;
    gerund: derived, _;
    ),
    // Разогнать: 8 words (подогнать, обогнать, отогнать, разогнать, согнать, вогнать). Like Разобрать.
    conjugation!("разогнать", 8, "~о~~ать", [aspect: Perfective], like Разобрать;
    stem non_past: "~~о~- like просмотреть";
    ),
    // Смяться: 8 words (измяться, смяться, размяться, вмяться, обмяться, подмяться). Like Размять with `~ь` after each pattern (`~я` after й, л, т, ь), except in 3 cells: non_past_1 pl, participle past_pass, gerund past.
    conjugation!("смяться", 8, "~~ять~я", [aspect: Perfective], like Размять with "~ь", "~я" after "йлть";
    non_past_1: _, "~омнё~~я";
    participle: _, _, _, derived;
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
    // Выпариться: 8 words (закупориться, выпариться, замусориться, умайориться, откупориться, раскупориться). Like Закупорить with `~ь` after each pattern (`~я` after й, л, м, т, ь), except in 3 cells: imperative sg, participle past_pass, gerund past.
    conjugation!("выпариться", 8, "~ить~я", [aspect: Perfective], like Закупорить with "~ь", "~я" after "йлмть";
    imperative: "~ь~я|~и~ь", _;
    participle: _, _, _, derived;
    gerund: _, "~я~ь|~ивши~ь";
    ),
    // Отсвистеться: 8 words (насвистеться, рассвистеться, пересвистеться, свистеться, высвистеться, досвистеться). Like Распуститься, except in 1 cell: infinitive.
    conjugation!("отсвистеться", 8, "~~теться", like Распуститься;
    stem past: "~~те- like распуститься";
    infinitive: "~~теться";
    ),
    // Разогнаться: 8 words (отозваться, разогнаться, взогнаться, вогнаться, обозваться, отогнаться). Like Разогнать with `~я` after each pattern (`~ь` after а, е, и, ю), except in 4 cells: past n, past pl, participle past_pass, gerund past.
    conjugation!("разогнаться", 8, "~о~~ать~я", [aspect: Perfective], like Разогнать with "~я", "~ь" after "аеию";
    past: _, _, "~о~~ало~ь|~о~~ало~ь", "~о~~али~ь|~о~~али~ь";
    participle: _, _, _, derived;
    gerund: _, "~о~~авши~ь";
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
    // Вынести: 7 words (вынести, вывезти, выпасти, вытрясти, повывезти, повынести). Like Лезть, except in 5 cells: infinitive, participle pres_act, participle past_pass, gerund pres, gerund past.
    conjugation!("вынести", 7, "~ти", [aspect: Perfective], like Лезть;
    stem non_past: "~- like заглохнуть";
    infinitive: "~ти";
    participle: derived, _, _, "~енный";
    gerund: derived, "~я";
    ),
    // Нашептать: 7 words (шептать, прошептать, зашептать, нашептать, пошептать, перешептать).
    conjugation!("нашептать", 7, "~е~тать";
    stem non_past: "~е~ч- like оказать";
    infinitive: "~е~тать";
    past: "~е~тал", "~е~тала", "~е~тало", "~е~тали";
    participle: _, "~е~тавший", _, "~ё~танный";
    gerund: _, "~е~тав|~е~тавши";
    ),
    // Прятаться: 7 words (прятаться, спрятаться, запрятаться, припрятаться, упрятаться, напрятаться). Like Топтать with `~я` after each pattern (`~ь` after а, е, и, о, у), except in 4 cells: imperative sg, imperative pl, participle past_pass, gerund past.
    conjugation!("прятаться", 7, "~тать~я", like Топтать with "~я", "~ь" after "аеиоу";
    imperative: "~чь~я", "~чьте~ь";
    participle: _, _, _, derived;
    gerund: _, "~тавши~ь";
    ),
    // Закупорить: 7 words (выверить, закупорить, вызволить, расстопорить, застопорить, заякорить). Like Ударить, except in 4 cells: imperative sg, participle pres_act, participle pres_pass, gerund pres.
    conjugation!("закупорить", 7, "~ить", [aspect: Perfective], like Ударить;
    imperative: "~и|~ь", _;
    participle: derived, _, derived, _;
    gerund: derived, _;
    ),
    // Залгаться: 7 words (залгаться, изолгаться, отолгаться, оболгаться, разолгаться, налгаться). Like Стричься, except in 8 cells: infinitive, past m, past f, past n, past pl, participle pres_act, ….
    conjugation!("залгаться", 7, "~гать~я", [aspect: Perfective], like Стричься;
    infinitive: "~гать~я";
    past: "~гал~я", "~гала~ь", "~гало~ь", "~гали~ь";
    participle: derived, "~гавший~я", _, _;
    gerund: _, "~гавши~ь";
    ),
    // Насытиться: 7 words (насытиться, пресытиться, пересытиться, похититься, ухититься, сытиться). Like Насытить with `~ь` after each pattern (`~я` after й, л, м, т, ь), except in 3 cells: participle pres_pass, participle past_pass, gerund past.
    conjugation!("насытиться", 7, "~тить~я", like Насытить with "~ь", "~я" after "йлмть";
    participle: _, _, derived, derived;
    gerund: _, "~тивши~ь";
    ),
    // Стонать: 6 words (жаждать, высрать, возжаждать, страждать, кровожаждать, состраждать). Like Вырвать, except in 3 cells: participle pres_act, participle past_pass, gerund pres.
    conjugation!("стонать", 6, "~ать", like Вырвать;
    participle: "~ущий", _, _, derived;
    gerund: "~ая", _;
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
    // Морщиться: 6 words (таращиться, морщиться, поморщиться, сморщиться, топорщиться, упорядочиться). Like Морщить with `~я` after each pattern (`~ь` after а, е, и, о, у), except in 3 cells: participle pres_pass, participle past_pass, gerund past.
    conjugation!("морщиться", 6, "~ить~я", like Морщить with "~я", "~ь" after "аеиоу";
    participle: _, _, derived, derived;
    gerund: _, "~ивши~ь";
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
    // Сжаться: 6 words (сжаться, вжаться, отжаться, обжаться, поджаться, разжаться). Like Отжать with `~ь` after each pattern (`~я` after й, л, м, т, ь), except in 2 cells: participle past_pass, gerund past.
    conjugation!("сжаться", 6, "~~ать~я", [aspect: Perfective], like Отжать with "~ь", "~я" after "йлмть";
    participle: _, _, _, derived;
    gerund: _, "~~авши~ь";
    ),
    // Соваться: 6 words (коваться, основаться, соваться, подковаться, вковаться, рассоваться). Like Сковать with `~ь` after each pattern (`~я` after й, л, м, т, ь), except in 2 cells: participle past_pass, gerund past.
    conjugation!("соваться", 6, "~овать~я", like Сковать with "~ь", "~я" after "йлмть";
    participle: _, _, _, derived;
    gerund: _, "~овавши~ь";
    ),
    // Разъехаться: 6 words (съехаться, разъехаться, проехаться, поразъехаться, подъехаться, ехаться). Like Поехать with `~я` after each pattern (`~ь` after а, е, и, о, у), except in 1 cell: gerund past.
    conjugation!("разъехаться", 6, "~хать~я", like Поехать with "~я", "~ь" after "аеиоу";
    gerund: _, "~хавши~ь";
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
    // Плестись: 6 words (плестись, местись, сместись, цвестись, гнестись, доместись). Like Сплестись, except in 3 cells: participle pres_act, gerund pres, gerund past.
    conjugation!("плестись", 6, "~е~тись", like Сплестись;
    participle: "~етущий~я", _, _, _;
    gerund: "~етя~ь", "~ётши~ь";
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
    // Замереть: 6 words (запереть, умереть, помереть, замереть, примереть, позапереть). Like Затереть.
    conjugation!("замереть", 6, "~е~еть", [aspect: Perfective], like Затереть;
    stem past: "~е~- like замереть";
    ),
    // Мочься: 6 words (разнемочься, выпрячься, выстричься, мочься, занемочься, перемочься). Like Помочь with `~ь` after each pattern (`~я` after г, й, м, т, ь), except in 1 cell: participle past_pass.
    conjugation!("мочься", 6, "~чь~я", like Помочь with "~ь", "~я" after "гймть";
    participle: _, _, _, derived;
    ),
    // Взорать: 6 words (глаголать, проглаголать, обкармливать, разорать, взорать, доорать). Like Вырвать, except in 3 cells: non_past_1 sg, non_past_3 pl, participle past_pass.
    conjugation!("взорать", 6, "~ать", like Вырвать;
    non_past_1: "~ю", _;
    non_past_3: _, "~ют";
    participle: _, _, _, "~енный";
    ),
    // Висеть: 5 words (висеть, зависеть, повисеть, перевисеть, провисеть). Like Сносить, except in 3 cells: infinitive, participle pres_pass, participle past_pass.
    conjugation!("висеть", 5, "~сеть", like Сносить;
    stem past: "~се- like прошипеть";
    infinitive: "~сеть";
    participle: _, _, derived, derived;
    ),
    // Выбрать: 5 words (выбрать, выдрать, перевыбрать, повыдрать, повыбрать). Like Драть, except in 3 cells: participle pres_act, participle pres_pass, gerund pres.
    conjugation!("выбрать", 5, "~~ать", [aspect: Perfective], like Драть;
    stem non_past: "~е~- like заглохнуть";
    participle: derived, _, derived, _;
    gerund: derived, _;
    ),
    // Отжать: 5 words (сжать, обжать, разжать, отжать, вжать). Like Гнать.
    conjugation!("отжать", 5, "~~ать", [aspect: Perfective], like Гнать;
    stem non_past: "~о~м- like перевернуть";
    stem past: "~~а- like раздуть";
    ),
    // Отстоять: 5 words (отстоять, выстоять, простоять, застоять, достоять). Like Стоять, except in 2 cells: participle pres_act, gerund pres.
    conjugation!("отстоять", 5, "~ять", [aspect: Perfective], like Стоять;
    stem past: "~я- like играть";
    participle: derived, _, _, _;
    gerund: derived, _;
    ),
    // Выставиться: 5 words (понадобиться, осведомиться, выставиться, занадобиться, уведомиться). Like Выставить with `~я` after each pattern (`~ь` after а, е, и, о, ю), except in 2 cells: participle past_pass, gerund past.
    conjugation!("выставиться", 5, "~ить~я", [aspect: Perfective], like Выставить with "~я", "~ь" after "аеиою";
    participle: _, _, _, derived;
    gerund: _, "~ивши~ь";
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
    // Выдвинуться: 5 words (высунуться, выдвинуться, грянуться, откашлянуться, самовыдвинуться). Like Подкинуть with `~я` after each pattern (`~ь` after а, е, и, о, у), except in 3 cells: imperative sg, participle past_pass, gerund past.
    conjugation!("выдвинуться", 5, "~уть~я", [aspect: Perfective], like Подкинуть with "~я", "~ь" after "аеиоу";
    imperative: "~и~ь", _;
    participle: _, _, _, derived;
    gerund: _, "~увши~ь";
    ),
    // Пробежаться: 5 words (разбежаться, сбежаться, пробежаться, поразбежаться, оббежаться). Like Набежать with `~ь` after each pattern (`~я` after й, л, м, т, ь), except in 2 cells: participle pres_act, gerund past.
    conjugation!("пробежаться", 5, "~жать~я", [aspect: Perfective], like Набежать with "~ь", "~я" after "йлмть";
    participle: derived, _, _, _;
    gerund: _, "~жавши~ь";
    ),
    // Обжечься: 5 words (обжечься, разжечься, поджечься, сжечься, изжечься). Like Разжечь with `~ь` after each pattern (`~я` after г, й, м, т, ь), except in 1 cell: participle past_pass.
    conjugation!("обжечься", 5, "~~ечь~я", [aspect: Perfective], like Разжечь with "~ь", "~я" after "гймть";
    participle: _, _, _, derived;
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
    // Распоясаться: 5 words (распоясаться, выплясаться, подпоясаться, вытесаться, опоясаться). Like Чесаться, except in 4 cells: imperative sg, imperative pl, participle pres_act, gerund pres.
    conjugation!("распоясаться", 5, "~~аться", [aspect: Perfective], like Чесаться;
    imperative: "~шь~я", "~шьте~ь";
    participle: derived, _, _, _;
    gerund: derived, _;
    ),
    // Трепетаться: 5 words (вострепетаться, трепетаться, оклеветаться, затрепетаться, потрепетаться). Like Трепетать with `~ь` after each pattern (`~я` after й, л, м, т, ь), except in 1 cell: gerund past.
    conjugation!("трепетаться", 5, "~тать~я", like Трепетать with "~ь", "~я" after "йлмть";
    gerund: _, "~тавши~ь";
    ),
    // Ссаться: 5 words (уссаться, обоссаться, ссаться, сцаться, обосцаться). Like Зассать with `~ь` after each pattern (`~я` after й, л, м, т, ь), except in 4 cells: participle pres_act, participle past_pass, gerund pres, gerund past.
    conjugation!("ссаться", 5, "~ать~я", like Зассать with "~ь", "~я" after "йлмть";
    participle: "~ущий~я", _, _, derived;
    gerund: "~а~ь", "~авши~ь";
    ),
    // Срастись: 5 words (разрастись, срастись, зарастись, врастись, растись). Like Дорасти with `~ь` after each pattern (`~я` after ~, й, м, т, ь).
    conjugation!("срастись", 5, "~а~ти~ь", like Дорасти with "~ь", "~я" after "~ймть";
    ),
    // Вестись: 5 words (вестись, соблюстись, ебстись, блюстись, поебстись). Like Распасться, except in 3 cells: infinitive, participle past_act, gerund past.
    conjugation!("вестись", 5, "~~тись", like Распасться;
    infinitive: "~~тись";
    participle: _, "~дший~я", _, _;
    gerund: _, "~дши~ь";
    ),
    // Вывиться: 5 words (выбиться, вылиться, выпиться, вывиться, повыбиться). Like Выпить with `~я` after each pattern (`~ь` after а, е, и, о, ю), except in 3 cells: non_past_2 pl, participle past_pass, gerund past.
    conjugation!("вывиться", 5, "~ить~я", [aspect: Perfective], like Выпить with "~я", "~ь" after "аеиою";
    non_past_2: _, "~ейте~ь";
    participle: _, _, _, derived;
    gerund: _, "~ивши~ь";
    ),
    // Выползти: 5 words (вылезти, выползти, повылезти, повыползти, выебти). Like Вынести.
    conjugation!("выползти", 5, "~ти", [aspect: Perfective], like Вынести;
    stem past: "~- like грызть";
    ),
    // Изобрести: 4 words (изобрести, приобрести, обрести, соплести). Like Сплести, except in 2 cells: participle past_act, gerund past.
    conjugation!("изобрести", 4, "~ести", [aspect: Perfective], like Сплести;
    participle: _, "~етший", _, _;
    gerund: _, "~етя|~етши";
    ),
    // Принять: 4 words (принять, воспринять, предпринять, напринять).
    conjugation!("принять", 4, "~нять", [aspect: Perfective];
    stem non_past: "~м- like заглохнуть";
    stem past: "~ня- like раздуть";
    infinitive: "~нять";
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
    // Высечь: 4 words (высечь, вытечь, выволочь, выпечь). Like Обволочь, except in 6 cells: non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg, participle pres_act, participle past_pass.
    conjugation!("высечь", 4, "~чь", [aspect: Perfective], like Обволочь;
    non_past_1: _, "~чем";
    non_past_2: "~чешь", "~чете";
    non_past_3: "~чет", _;
    participle: derived, _, _, "~ченный";
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
    // Улечься: 4 words (улечься, поулечься, разлечься, приулечься). Like Лечь with `ся` after each pattern (`сь` after а, е, и, о, у).
    conjugation!("улечься", 4, "~ечься", [aspect: Perfective], like Лечь with "ся", "сь" after "аеиоу";
    ),
    // Вынестись: 4 words (вынестись, вывезтись, вытрястись, отверзтись). Like Вынести with `~ь` after each pattern (`~я` after ~, й, м, т, ь), except in 1 cell: participle past_pass.
    conjugation!("вынестись", 4, "~ти~ь", [aspect: Perfective], like Вынести with "~ь", "~я" after "~ймть";
    participle: _, _, _, derived;
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
    // Растереться: 4 words (растереться, истереться, поистереться, распереться). Like Истереть with `~ь` after each pattern (`~я` after ~, й, м, т, ь), except in 2 cells: participle past_pass, gerund past.
    conjugation!("растереться", 4, "~с~е~еть~я", [aspect: Perfective], like Истереть with "~ь", "~я" after "~ймть";
    participle: _, _, _, derived;
    gerund: _, "~с~ё~ши~ь";
    ),
    // Изъяться: 4 words (изъяться, подъяться, разъяться, отъяться). Like Изъять with `~ь` after each pattern (`~я` after й, л, м, т, ь), except in 2 cells: participle past_pass, gerund past.
    conjugation!("изъяться", 4, "~ъять~я", [aspect: Perfective], like Изъять with "~ь", "~я" after "йлмть";
    participle: _, _, _, derived;
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
    // Помочь: 3 words (выстричь, выпрячь, вылечь). Like Постричь, except in 5 cells: non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg, participle pres_act.
    conjugation!("помочь", 3, "~чь", [aspect: Perfective], like Постричь;
    non_past_1: _, "~жем";
    non_past_2: "~жешь", "~жете";
    non_past_3: "~жет", _;
    participle: derived, _, _, _;
    ),
    // Хотеть: 3 words (хотеть, захотеть, расхотеть). Like Бухтеть, except in 6 cells: non_past_2 sg, non_past_3 sg, participle pres_act, participle pres_pass, participle past_pass, gerund pres.
    conjugation!("хотеть", 3, "~теть", like Бухтеть;
    non_past_2: "~чешь", _;
    non_past_3: "~чет", _;
    participle: derived, _, derived, derived;
    gerund: derived, _;
    ),
    // Хотеться: 3 words (хотеться, захотеться, расхотеться). Like Хотеть with `~ь` after each pattern (`~я` after й, л, м, т, ь), except in 1 cell: gerund past.
    conjugation!("хотеться", 3, "~теть~я", like Хотеть with "~ь", "~я" after "йлмть";
    gerund: _, "~тевши~ь";
    ),
    // Вырасти: 3 words (вырасти, повырасти, подвырасти). Like Дорасти, except in 2 cells: participle pres_act, gerund pres.
    conjugation!("вырасти", 3, "~а~ти", [aspect: Perfective], like Дорасти;
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
    conjugation!("опоясать", 3, "~сать", [aspect: Perfective], like Выписать;
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
    // Приняться: 3 words (приняться, восприняться, предприняться). Like Принять with `~ь` after each pattern (`~я` after й, л, м, т, ь), except in 2 cells: participle past_pass, gerund past.
    conjugation!("приняться", 3, "~нять~я", [aspect: Perfective], like Принять with "~ь", "~я" after "йлмть";
    participle: _, _, _, derived;
    gerund: _, "~нявши~ь";
    ),
    // Вывести: 3 words (вывести, выбрести, повывести). Like Выпасть, except in 3 cells: infinitive, participle past_act, gerund past.
    conjugation!("вывести", 3, "~сти", [aspect: Perfective], like Выпасть;
    infinitive: "~сти";
    participle: _, "~дший", _, _;
    gerund: _, "~дя";
    ),
    // Выбраться: 3 words (выбраться, выдраться, повыбраться). Like Выбрать with `~ь` after each pattern (`~я` after й, л, м, т, ь), except in 2 cells: participle past_pass, gerund past.
    conjugation!("выбраться", 3, "~~ать~я", [aspect: Perfective], like Выбрать with "~ь", "~я" after "йлмть";
    participle: _, _, _, derived;
    gerund: _, "~~авши~ь";
    ),
    // Вылезть: 3 words (вылезть, выгрызть, отверзть). Like Лезть, except in 3 cells: imperative sg, participle pres_act, gerund pres.
    conjugation!("вылезть", 3, "~ть", [aspect: Perfective], like Лезть;
    imperative: "~и", _;
    participle: derived, _, _, _;
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
    // Обмереть: 3 words (отпереть, обмереть, отмереть). Like Стереть.
    conjugation!("обмереть", 3, "~~е~еть", [aspect: Perfective], like Стереть;
    stem past: "~~е~- like обмереть";
    ),
    // Понудить: 3 words (принудить, понудить, попринудить). Like Высвободить, except in 3 cells: imperative sg, imperative pl, participle past_pass.
    conjugation!("понудить", 3, "~дить", [aspect: Perfective], like Высвободить;
    imperative: "~дь", "~дьте";
    participle: _, _, _, "~ждённый";
    ),
    // Отозвать: 3 words (отозвать, обозвать, подозвать). Like Разогнать.
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
    // Осечься: 3 words (пересечься, осечься, сечься). Like Засечь with `~я` after each pattern (`~ь` after а, е, и, о, у), except in 1 cell: participle past_pass.
    conjugation!("осечься", 3, "~ечь~я", like Засечь with "~я", "~ь" after "аеиоу";
    participle: _, _, _, derived;
    ),
    // Изыти: 3 words (снити, изыти, внити). Like Вывести, except in 7 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("изыти", 3, "~ти", [aspect: Perfective], like Вывести;
    infinitive: "~ти";
    past: "~шел", "~шла", "~шло", "~шли";
    participle: _, "~шедший", _, derived;
    ),
    // Наморщиться: 3 words (встопорщиться, наморщиться, изморщиться). Like Закупорить with `~я` after each pattern (`~ь` after а, е, и, о), except in 4 cells: non_past_1 sg, imperative sg, participle past_pass, gerund past.
    conjugation!("наморщиться", 3, "~ить~я", [aspect: Perfective], like Закупорить with "~я", "~ь" after "аеио";
    non_past_1: "~у~ь", _;
    imperative: "~ь~я|~и~ь", _;
    participle: _, _, _, derived;
    gerund: _, "~ивши~ь";
    ),
    // Чтиться: 3 words (чтиться, почтиться, узриться). Like Палить with `~я` after each pattern (`~ь` after а, е, и, о, я), except in 4 cells: non_past_1 sg, participle pres_pass, participle past_pass, gerund past.
    conjugation!("чтиться", 3, "~ить~я", like Палить with "~я", "~ь" after "аеиоя";
    non_past_1: "~у~ь", _;
    participle: _, _, derived, derived;
    gerund: _, "~ивши~ь|~я~ь";
    ),
    // Взбресть: 3 words (взбресть, добресть, набресть). Like Развести, except in 3 cells: infinitive, participle past_pass, gerund past.
    conjugation!("взбресть", 3, "~есть", [aspect: Perfective], like Развести;
    infinitive: "~есть";
    participle: _, _, _, derived;
    gerund: _, "~едя";
    ),
    // Выпечься: 3 words (выволочься, выпечься, высечься). Like Высечь with `~ь` after each pattern (`~я` after й, к, м, т, ь), except in 1 cell: participle past_pass.
    conjugation!("выпечься", 3, "~чь~я", [aspect: Perfective], like Высечь with "~ь", "~я" after "йкмть";
    participle: _, _, _, derived;
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
    // Испиться: 3 words (распиться, расшиться, испиться). Like Испить with `~я` after each pattern (`~ь` after а, е, и, о, ю), except in 2 cells: participle past_pass, gerund past.
    conjugation!("испиться", 3, "~с~ить~я", [aspect: Perfective], like Испить with "~я", "~ь" after "аеиою";
    participle: _, _, _, derived;
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
    // Закодить: 3 words (повыводить, закодить, накодить). Like Высвободить.
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
