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
/// they share them), the conjugation it is like (if any), then a row of patterns
/// per line (`derived`: the form comes from another cell's, see
/// `relations.rs`; `_`, or a row left out: the form is the conjugation's it is like,
/// or, in a conjugation like no other, derived):
/// `infinitive`
/// `non_past_1` (sg, pl)
/// `non_past_2` (sg, pl)
/// `non_past_3` (sg, pl)
/// `past` (m, f, n, pl)
/// `imperative` (sg, pl)
/// `participle` (pres_act, past_act, pres_pass, past_pass)
/// `gerund` (pres, past)
macro_rules! conjugation {
    ($name:literal, $words:literal, $lemma:literal $(, [$($f:ident: $v:ident),*])? $(, like $parent:ident)?;
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

/// An inherent feature's value, as `(feature, value)` by their places.
macro_rules! feature {
    (aspect: $v:ident) => {
        (0, crate::Aspect::$v as u8)
    };
}

/// Every conjugation learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Conjugation {
    /// Like `играть`: 11,635 words.
    Играть,
    /// Like `набираться`: 8,293 words.
    Набираться,
    /// Like `вибрировать`: 4,162 words.
    Вибрировать,
    /// Like `стыковаться`: 2,209 words.
    Стыковаться,
    /// Like `ударить`: 1,983 words.
    Ударить,
    /// Like `палить`: 1,318 words.
    Палить,
    /// Like `числиться`: 1,171 words.
    Числиться,
    /// Like `раздуть`: 928 words.
    Раздуть,
    /// Like `рвануть`: 685 words.
    Рвануть,
    /// Like `выделить`: 616 words.
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
    /// Like `обрушиться`: 343 words.
    Обрушиться,
    /// Like `восстановить`: 336 words.
    Восстановить,
    /// Like `разводить`: 327 words.
    Разводить,
    /// Like `долбить`: 317 words.
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
    /// Like `стучать`: 192 words.
    Стучать,
    /// Like `защитить`: 191 words.
    Защитить,
    /// Like `сносить`: 186 words.
    Сносить,
    /// Like `отметить`: 164 words.
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
    /// Like `перестроиться`: 110 words.
    Перестроиться,
    /// Like `держаться`: 108 words.
    Держаться,
    /// Like `сычевать`: 108 words.
    Сычевать,
    /// Like `кишеть`: 105 words.
    Кишеть,
    /// Like `сорваться`: 103 words.
    Сорваться,
    /// Like `трафить`: 103 words.
    Трафить,
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
    /// Like `бухтеть`: 68 words.
    Бухтеть,
    /// Like `забить`: 68 words.
    Забить,
    /// Like `сблизиться`: 66 words.
    Сблизиться,
    /// Like `отвоевать`: 63 words.
    Отвоевать,
    /// Like `стоить`: 62 words.
    Стоить,
    /// Like `зевать`: 61 words.
    Зевать,
    /// Like `обтечь`: 60 words.
    Обтечь,
    /// Like `возбудить`: 59 words.
    Возбудить,
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
    /// Like `идти`: 53 words.
    Идти,
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
    /// Like `глядеться`: 48 words.
    Глядеться,
    /// Like `развести`: 48 words.
    Развести,
    /// Like `разбить`: 46 words.
    Разбить,
    /// Like `снести`: 45 words.
    Снести,
    /// Like `клевать`: 45 words.
    Клевать,
    /// Like `колоться`: 42 words.
    Колоться,
    /// Like `топтаться`: 42 words.
    Топтаться,
    /// Like `драть`: 42 words.
    Драть,
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
    /// Like `сволочь`: 39 words.
    Сволочь,
    /// Like `служить`: 37 words.
    Служить,
    /// Like `обратиться`: 37 words.
    Обратиться,
    /// Like `трястись`: 36 words.
    Трястись,
    /// Like `загореться`: 36 words.
    Загореться,
    /// Like `гореть`: 35 words.
    Гореть,
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
    /// Like `обыскать`: 31 words.
    Обыскать,
    /// Like `сковать`: 30 words.
    Сковать,
    /// Like `грести`: 30 words.
    Грести,
    /// Like `искусить`: 30 words.
    Искусить,
    /// Like `распасться`: 29 words.
    Распасться,
    /// Like `вывестись`: 28 words.
    Вывестись,
    /// Like `отпустить`: 27 words.
    Отпустить,
    /// Like `зайти`: 27 words.
    Зайти,
    /// Like `набраться`: 27 words.
    Набраться,
    /// Like `плакать`: 26 words.
    Плакать,
    /// Like `просвистеть`: 26 words.
    Просвистеть,
    /// Like `упереться`: 26 words.
    Упереться,
    /// Like `терпеться`: 26 words.
    Терпеться,
    /// Like `плеваться`: 26 words.
    Плеваться,
    /// Like `стричься`: 26 words.
    Стричься,
    /// Like `наплыть`: 25 words.
    Наплыть,
    /// Like `тухнуть`: 25 words.
    Тухнуть,
    /// Like `разлететься`: 24 words.
    Разлететься,
    /// Like `расплыться`: 24 words.
    Расплыться,
    /// Like `вторгнуться`: 24 words.
    Вторгнуться,
    /// Like `отнять`: 24 words.
    Отнять,
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
    /// Like `лезть`: 20 words.
    Лезть,
    /// Like `зашибить`: 20 words.
    Зашибить,
    /// Like `занять`: 19 words.
    Занять,
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
    /// Like `жать`: 17 words.
    Жать,
    /// Like `вгрызться`: 17 words.
    Вгрызться,
    /// Like `взъебать`: 17 words.
    Взъебать,
    /// Like `колыхаться`: 17 words.
    Колыхаться,
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
    /// Like `честь`: 15 words.
    Честь,
    /// Like `насесть`: 15 words.
    Насесть,
    /// Like `смеяться`: 15 words.
    Смеяться,
    /// Like `гнать`: 15 words.
    Гнать,
    /// Like `помять`: 15 words.
    Помять,
    /// Like `зиждить`: 15 words.
    Зиждить,
    /// Like `гнить`: 15 words.
    Гнить,
    /// Like `сберечь`: 15 words.
    Сберечь,
    /// Like `видеть`: 14 words.
    Видеть,
    /// Like `прожечь`: 14 words.
    Прожечь,
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
    /// Like `тыкаться`: 13 words.
    Тыкаться,
    /// Like `отгрестись`: 13 words.
    Отгрестись,
    /// Like `нагореть`: 13 words.
    Нагореть,
    /// Like `вызвать`: 12 words.
    Вызвать,
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
    /// Like `стелиться`: 12 words.
    Стелиться,
    /// Like `подвигнуться`: 12 words.
    Подвигнуться,
    /// Like `поехать`: 12 words.
    Поехать,
    /// Like `дожить`: 11 words.
    Дожить,
    /// Like `ушибиться`: 11 words.
    Ушибиться,
    /// Like `зачесться`: 11 words.
    Зачесться,
    /// Like `жаться`: 11 words.
    Жаться,
    /// Like `пакоститься`: 11 words.
    Пакоститься,
    /// Like `зассать`: 11 words.
    Зассать,
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
    /// Like `разойтись`: 9 words.
    Разойтись,
    /// Like `стонать`: 9 words.
    Стонать,
    /// Like `стереть`: 9 words.
    Стереть,
    /// Like `выспаться`: 9 words.
    Выспаться,
    /// Like `застлать`: 9 words.
    Застлать,
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
    /// Like `выпить`: 8 words.
    Выпить,
    /// Like `разогнать`: 8 words.
    Разогнать,
    /// Like `отозваться`: 8 words.
    Отозваться,
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
    /// Like `месть`: 8 words.
    Месть,
    /// Like `отсвистеться`: 8 words.
    Отсвистеться,
    /// Like `вести`: 7 words.
    Вести,
    /// Like `вынести`: 7 words.
    Вынести,
    /// Like `нашептать`: 7 words.
    Нашептать,
    /// Like `прятаться`: 7 words.
    Прятаться,
    /// Like `закупорить`: 7 words.
    Закупорить,
    /// Like `вымести`: 7 words.
    Вымести,
    /// Like `залгаться`: 7 words.
    Залгаться,
    /// Like `насытиться`: 7 words.
    Насытиться,
    /// Like `спать`: 6 words.
    Спать,
    /// Like `достичь`: 6 words.
    Достичь,
    /// Like `разжечь`: 6 words.
    Разжечь,
    /// Like `замереть`: 6 words.
    Замереть,
    /// Like `морщиться`: 6 words.
    Морщиться,
    /// Like `измыслить`: 6 words.
    Измыслить,
    /// Like `обидеть`: 6 words.
    Обидеть,
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
    /// Like `мочься`: 6 words.
    Мочься,
    /// Like `исхлестаться`: 6 words.
    Исхлестаться,
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
    /// Like `заспать`: 5 words.
    Заспать,
    /// Like `выставиться`: 5 words.
    Выставиться,
    /// Like `одолеть`: 5 words.
    Одолеть,
    /// Like `выставить`: 5 words.
    Выставить,
    /// Like `выдвинуться`: 5 words.
    Выдвинуться,
    /// Like `пробежаться`: 5 words.
    Пробежаться,
    /// Like `обжечься`: 5 words.
    Обжечься,
    /// Like `приобрестись`: 5 words.
    Приобрестись,
    /// Like `волгнуть`: 5 words.
    Волгнуть,
    /// Like `стлаться`: 5 words.
    Стлаться,
    /// Like `распоясаться`: 5 words.
    Распоясаться,
    /// Like `вылиться`: 5 words.
    Вылиться,
    /// Like `трепетаться`: 5 words.
    Трепетаться,
    /// Like `ссаться`: 5 words.
    Ссаться,
    /// Like `вылезти`: 5 words.
    Вылезти,
    /// Like `срастись`: 5 words.
    Срастись,
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
    /// Like `уеть`: 4 words.
    Уеть,
    /// Like `доебсти`: 4 words.
    Доебсти,
    /// Like `поколебать`: 4 words.
    Поколебать,
    /// Like `насочинять`: 4 words.
    Насочинять,
    /// Like `растереться`: 4 words.
    Растереться,
    /// Like `изъяться`: 4 words.
    Изъяться,
    /// Like `помочь`: 3 words.
    Помочь,
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
    /// Like `пролежать`: 3 words.
    Пролежать,
    /// Like `осечься`: 3 words.
    Осечься,
    /// Like `наморщиться`: 3 words.
    Наморщиться,
    /// Like `чтиться`: 3 words.
    Чтиться,
    /// Like `взбресть`: 3 words.
    Взбресть,
    /// Like `выпечься`: 3 words.
    Выпечься,
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
    pub const ALL: [Conjugation; 334] = [
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
        Conjugation::Перестроиться,
        Conjugation::Держаться,
        Conjugation::Сычевать,
        Conjugation::Кишеть,
        Conjugation::Сорваться,
        Conjugation::Трафить,
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
        Conjugation::Бухтеть,
        Conjugation::Забить,
        Conjugation::Сблизиться,
        Conjugation::Отвоевать,
        Conjugation::Стоить,
        Conjugation::Зевать,
        Conjugation::Обтечь,
        Conjugation::Возбудить,
        Conjugation::Сидеть,
        Conjugation::Перевернуть,
        Conjugation::Виться,
        Conjugation::Сунуться,
        Conjugation::Колоть,
        Conjugation::Раскрыться,
        Conjugation::Заглохнуть,
        Conjugation::Идти,
        Conjugation::Влечься,
        Conjugation::Чесаться,
        Conjugation::Насытить,
        Conjugation::Отказаться,
        Conjugation::Пасть,
        Conjugation::Отдаваться,
        Conjugation::Глядеться,
        Conjugation::Развести,
        Conjugation::Разбить,
        Conjugation::Снести,
        Conjugation::Клевать,
        Conjugation::Колоться,
        Conjugation::Топтаться,
        Conjugation::Драть,
        Conjugation::Пыхать,
        Conjugation::Сдать,
        Conjugation::Выписать,
        Conjugation::Отобразить,
        Conjugation::Сбиться,
        Conjugation::Сволочь,
        Conjugation::Служить,
        Conjugation::Обратиться,
        Conjugation::Трястись,
        Conjugation::Загореться,
        Conjugation::Гореть,
        Conjugation::Щипать,
        Conjugation::Трепаться,
        Conjugation::Сплести,
        Conjugation::Расчесать,
        Conjugation::Станцевать,
        Conjugation::Резаться,
        Conjugation::Отвоеваться,
        Conjugation::Обыскать,
        Conjugation::Сковать,
        Conjugation::Грести,
        Conjugation::Искусить,
        Conjugation::Распасться,
        Conjugation::Вывестись,
        Conjugation::Отпустить,
        Conjugation::Зайти,
        Conjugation::Набраться,
        Conjugation::Плакать,
        Conjugation::Просвистеть,
        Conjugation::Упереться,
        Conjugation::Терпеться,
        Conjugation::Плеваться,
        Conjugation::Стричься,
        Conjugation::Наплыть,
        Conjugation::Тухнуть,
        Conjugation::Разлететься,
        Conjugation::Расплыться,
        Conjugation::Вторгнуться,
        Conjugation::Отнять,
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
        Conjugation::Лезть,
        Conjugation::Зашибить,
        Conjugation::Занять,
        Conjugation::Лечь,
        Conjugation::Плескаться,
        Conjugation::Предназначить,
        Conjugation::Просмотреть,
        Conjugation::Въесться,
        Conjugation::Потчевать,
        Conjugation::Устояться,
        Conjugation::Трепетать,
        Conjugation::Жать,
        Conjugation::Вгрызться,
        Conjugation::Взъебать,
        Conjugation::Колыхаться,
        Conjugation::Постричь,
        Conjugation::Разобрать,
        Conjugation::Выровнять,
        Conjugation::Трепать,
        Conjugation::Заволочься,
        Conjugation::Честь,
        Conjugation::Насесть,
        Conjugation::Смеяться,
        Conjugation::Гнать,
        Conjugation::Помять,
        Conjugation::Зиждить,
        Conjugation::Гнить,
        Conjugation::Сберечь,
        Conjugation::Видеть,
        Conjugation::Прожечь,
        Conjugation::Везти,
        Conjugation::Молоть,
        Conjugation::Прискакать,
        Conjugation::Молоться,
        Conjugation::Просвистать,
        Conjugation::Глохнуть,
        Conjugation::Подобраться,
        Conjugation::Засечь,
        Conjugation::Толочь,
        Conjugation::Мяться,
        Conjugation::Сплестись,
        Conjugation::Тыкаться,
        Conjugation::Отгрестись,
        Conjugation::Нагореть,
        Conjugation::Вызвать,
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
        Conjugation::Стелиться,
        Conjugation::Подвигнуться,
        Conjugation::Поехать,
        Conjugation::Дожить,
        Conjugation::Ушибиться,
        Conjugation::Зачесться,
        Conjugation::Жаться,
        Conjugation::Пакоститься,
        Conjugation::Зассать,
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
        Conjugation::Разойтись,
        Conjugation::Стонать,
        Conjugation::Стереть,
        Conjugation::Выспаться,
        Conjugation::Застлать,
        Conjugation::Размять,
        Conjugation::Толочься,
        Conjugation::Скрестись,
        Conjugation::Зажечься,
        Conjugation::Мести,
        Conjugation::Выпить,
        Conjugation::Разогнать,
        Conjugation::Отозваться,
        Conjugation::Смяться,
        Conjugation::Осмеять,
        Conjugation::Выклевать,
        Conjugation::Разреветься,
        Conjugation::Выпариться,
        Conjugation::Месть,
        Conjugation::Отсвистеться,
        Conjugation::Вести,
        Conjugation::Вынести,
        Conjugation::Нашептать,
        Conjugation::Прятаться,
        Conjugation::Закупорить,
        Conjugation::Вымести,
        Conjugation::Залгаться,
        Conjugation::Насытиться,
        Conjugation::Спать,
        Conjugation::Достичь,
        Conjugation::Разжечь,
        Conjugation::Замереть,
        Conjugation::Морщиться,
        Conjugation::Измыслить,
        Conjugation::Обидеть,
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
        Conjugation::Мочься,
        Conjugation::Исхлестаться,
        Conjugation::Взорать,
        Conjugation::Висеть,
        Conjugation::Выбрать,
        Conjugation::Отжать,
        Conjugation::Отстоять,
        Conjugation::Заспать,
        Conjugation::Выставиться,
        Conjugation::Одолеть,
        Conjugation::Выставить,
        Conjugation::Выдвинуться,
        Conjugation::Пробежаться,
        Conjugation::Обжечься,
        Conjugation::Приобрестись,
        Conjugation::Волгнуть,
        Conjugation::Стлаться,
        Conjugation::Распоясаться,
        Conjugation::Вылиться,
        Conjugation::Трепетаться,
        Conjugation::Ссаться,
        Conjugation::Вылезти,
        Conjugation::Срастись,
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
        Conjugation::Уеть,
        Conjugation::Доебсти,
        Conjugation::Поколебать,
        Conjugation::Насочинять,
        Conjugation::Растереться,
        Conjugation::Изъяться,
        Conjugation::Помочь,
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
        Conjugation::Счесть,
        Conjugation::Обмереть,
        Conjugation::Понудить,
        Conjugation::Отозвать,
        Conjugation::Помутить,
        Conjugation::Обглодать,
        Conjugation::Пролежать,
        Conjugation::Осечься,
        Conjugation::Наморщиться,
        Conjugation::Чтиться,
        Conjugation::Взбресть,
        Conjugation::Выпечься,
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

pub(super) static CONJUGATIONS: [Paradigm<'static>; 334] = [
    // Играть: 11,635 words (знать, играть, презирать, делать, уступать, скучать).
    conjugation!("играть", 11635, "~ть";
    infinitive: "~ть";
    non_past_1: "~ю", "~ем";
    non_past_2: "~ешь", "~ете";
    non_past_3: "~ет", "~ют";
    past: "~л", "~ла", "~ло", "~ли";
    imperative: "~й", "~йте";
    participle: "~ющий", "~вший", "~емый", "~нный";
    gerund: "~я", "~в|~вши";
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
    infinitive: "~овать";
    non_past_1: "~ую", "~уем";
    non_past_2: "~уешь", "~уете";
    non_past_3: "~ует", "~уют";
    past: "~овал", "~овала", "~овало", "~овали";
    imperative: "~уй", "~уйте";
    participle: "~ующий", "~овавший", "~уемый", "~ованный";
    gerund: "~уя", "~овав|~овавши";
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
    infinitive: "~ить";
    non_past_1: "~ю", "~им";
    non_past_2: "~ишь", "~ите";
    non_past_3: "~ит", "~ят";
    past: "~ил", "~ила", "~ило", "~или";
    imperative: "~ь", "~ьте";
    participle: "~ящий", "~ивший", "~имый", "~енный";
    gerund: "~я", "~ив|~ивши";
    ),
    // Палить: 1,318 words (говорить, поговорить, изменить, благодарить, винить, отменить). Like Ударить, except in 3 cells: imperative sg, imperative pl, participle past_pass.
    conjugation!("палить", 1318, "~ить", like Ударить;
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
    // Раздуть: 928 words (деревенеть, взять, одеревенеть, деревянеть, болеть, краснеть). Like Играть, except in 1 cell: participle past_pass.
    conjugation!("раздуть", 928, "~ть", like Играть;
    participle: _, _, _, "~тый";
    ),
    // Рвануть: 685 words (взбзднуть, хохотнуть, блевануть, пугнуть, гнуть, ткнуть).
    conjugation!("рвануть", 685, "~уть", [aspect: Perfective];
    infinitive: "~уть";
    non_past_1: "~у", "~ём";
    non_past_2: "~ёшь", "~ёте";
    non_past_3: "~ёт", "~ут";
    past: "~ул", "~ула", "~уло", "~ули";
    imperative: "~и", "~ите";
    participle: "~ущий", "~увший", _, "~утый";
    gerund: _, "~ув|~увши";
    ),
    // Выделить: 616 words (учить, курить, покурить, кончить, закончить, варить). Like Палить, except in 1 cell: participle past_pass.
    conjugation!("выделить", 616, "~ить", like Палить;
    participle: _, _, _, "~енный";
    ),
    // Усилиться: 566 words (ссориться, пялиться, ускориться, стариться, состариться, целиться). Like Числиться, except in 3 cells: imperative sg, imperative pl, gerund past.
    conjugation!("усилиться", 566, "~ить~я", like Числиться;
    imperative: "~ь~я", "~ьте~ь";
    gerund: _, "~ивши~ь|~я~ь";
    ),
    // Тянуть: 543 words (выковырнуть, прыгнуть, пёрнуть, капнуть, шлёпнуть, взглянуть). Like Рвануть, except in 4 cells: non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg.
    conjugation!("тянуть", 543, "~уть", [aspect: Perfective], like Рвануть;
    non_past_1: _, "~ем";
    non_past_2: "~ешь", "~ете";
    non_past_3: "~ет", _;
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
    // Обрушиться: 343 words (увеличиться, размножиться, кочевряжиться, дурачиться, спешиться, мучиться). Like Обнажиться, except in 2 cells: imperative sg, imperative pl.
    conjugation!("обрушиться", 343, "~ить~я", like Обнажиться;
    imperative: "~ь~я", "~ьте~ь";
    ),
    // Восстановить: 336 words (уступить, любить, полюбить, возлюбить, купить, заломить). Like Палить, except in 2 cells: non_past_1 sg, participle past_pass.
    conjugation!("восстановить", 336, "~ить", like Палить;
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
    // Долбить: 317 words (вступить, наступить, дымить, удивить, бомбить, выступить). Like Восстановить, except in 1 cell: participle past_pass.
    conjugation!("долбить", 317, "~ить", like Восстановить;
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
    // Расправиться: 296 words (нравиться, понравиться, готовиться, направиться, знакомиться, познакомиться). Like Усилиться, except in 2 cells: non_past_1 sg, gerund past.
    conjugation!("расправиться", 296, "~ить~я", like Усилиться;
    non_past_1: "~лю~ь", _;
    gerund: _, "~ивши~ь";
    ),
    // Представить: 290 words (забавить, восславить, готовить, грабить, заставить, править). Like Ударить, except in 2 cells: non_past_1 sg, participle past_pass.
    conjugation!("представить", 290, "~ить", like Ударить;
    non_past_1: "~лю", _;
    participle: _, _, _, "~ленный";
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
    infinitive: "~нуть";
    non_past_1: "~ну", "~нем";
    non_past_2: "~нешь", "~нете";
    non_past_3: "~нет", "~нут";
    past: "~", "~ла", "~ло", "~ли";
    imperative: "~ни", "~ните";
    participle: "~нущий", "~ший", _, _;
    gerund: _, "~ши";
    ),
    // Стучать: 192 words (держать, кричать, жужжать, мычать, принадлежать, лежать).
    conjugation!("стучать", 192, "~ать";
    infinitive: "~ать";
    non_past_1: "~у", "~им";
    non_past_2: "~ишь", "~ите";
    non_past_3: "~ит", "~ат";
    past: "~ал", "~ала", "~ало", "~али";
    imperative: "~и", "~ите";
    participle: "~ащий", "~авший", "~имый", "~анный";
    gerund: "~а", "~ав|~авши";
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
    // Отметить: 164 words (ответить, встретить, заметить, утратить, тратить, похитить). Like Хватить, except in 2 cells: imperative sg, imperative pl.
    conjugation!("отметить", 164, "~тить", like Хватить;
    imperative: "~ть", "~тьте";
    ),
    // Распуститься: 163 words (креститься, спуститься, сгуститься, опуститься, поместиться, проститься).
    conjugation!("распуститься", 163, "~~титься";
    infinitive: "~~титься";
    non_past_1: "~щу~ь", "~~тимся";
    non_past_2: "~~тишься", "~~титесь";
    non_past_3: "~~тится", "~~тятся";
    past: "~~тился", "~~тилась", "~~тилось", "~~тились";
    imperative: "~~тись", "~~титесь";
    participle: "~~тящийся", "~~тившийся", _, _;
    gerund: "~~тясь", "~~тившись";
    ),
    // Носиться: 138 words (соотноситься, согласиться, относиться, напроситься, произноситься, доноситься).
    conjugation!("носиться", 138, "~~иться";
    infinitive: "~~иться";
    non_past_1: "~шу~ь", "~~имся";
    non_past_2: "~~ишься", "~~итесь";
    non_past_3: "~~ится", "~~ятся";
    past: "~~ился", "~~илась", "~~илось", "~~ились";
    imperative: "~~ись", "~~итесь";
    participle: "~~ящийся", "~~ившийся", _, _;
    gerund: "~~ясь", "~~ившись";
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
    infinitive: "~зать";
    non_past_1: "~жу", "~жем";
    non_past_2: "~жешь", "~жете";
    non_past_3: "~жет", "~жут";
    past: "~зал", "~зала", "~зало", "~зали";
    imperative: "~жи", "~жите";
    participle: "~жущий", "~завший", _, "~занный";
    gerund: "~жа", "~зав|~завши";
    ),
    // Засосать: 118 words (врать, ебать, ржать, заржать, соврать, рвать). Like Стучать, except in 8 cells: non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg, non_past_3 pl, participle pres_act, ….
    conjugation!("засосать", 118, "~ать", like Стучать;
    non_past_1: _, "~ём";
    non_past_2: "~ёшь", "~ёте";
    non_past_3: "~ёт", "~ут";
    participle: "~ущий", _, derived, _;
    gerund: "~я", _;
    ),
    // Топтать: 117 words (хохотать, захохотать, прятать, спрятать, щекотать, лепетать).
    conjugation!("топтать", 117, "~тать";
    infinitive: "~тать";
    non_past_1: "~чу", "~чем";
    non_past_2: "~чешь", "~чете";
    non_past_3: "~чет", "~чут";
    past: "~тал", "~тала", "~тало", "~тали";
    imperative: "~чи", "~чите";
    participle: "~чущий", "~тавший", _, "~танный";
    gerund: "~ча", "~тав|~тавши";
    ),
    // Развеять: 113 words (баять, лаять, чуять, сеять, блеять, посеять). Like Играть, except in 8 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("развеять", 113, "~ять", like Играть;
    infinitive: "~ять";
    past: "~ял", "~яла", "~яло", "~яли";
    participle: _, "~явший", _, "~янный";
    gerund: _, "~яв|~явши";
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
    // Сычевать: 108 words (бичевать, воевать, кочевать, завоевать, ночевать, корчевать). Like Вибрировать, except in 8 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("сычевать", 108, "~евать", like Вибрировать;
    infinitive: "~евать";
    past: "~евал", "~евала", "~евало", "~евали";
    participle: _, "~евавший", _, "~ёванный";
    gerund: _, "~евав|~евавши";
    ),
    // Кишеть: 105 words (загреметь, храпеть, кипеть, терпеть, скрипеть, греметь). Like Восстановить, except in 8 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("кишеть", 105, "~еть", like Восстановить;
    infinitive: "~еть";
    past: "~ел", "~ела", "~ело", "~ели";
    participle: _, "~евший", _, derived;
    gerund: _, "~ев|~евши";
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
    // Подкинуть: 88 words (покинуть, тронуть, кинуть, клюнуть, плюнуть, сунуть). Like Тянуть, except in 3 cells: imperative sg, imperative pl, participle pres_act.
    conjugation!("подкинуть", 88, "~уть", [aspect: Perfective], like Тянуть;
    imperative: "~ь", "~ьте";
    participle: derived, _, _, _;
    ),
    // Открыть: 83 words (открыть, изныть, мыть, помыть, закрыть, рыть).
    conjugation!("открыть", 83, "~ыть";
    infinitive: "~ыть";
    non_past_1: "~ою", "~оем";
    non_past_2: "~оешь", "~оете";
    non_past_3: "~оет", "~оют";
    past: "~ыл", "~ыла", "~ыло", "~ыли";
    imperative: "~ой", "~ойте";
    participle: "~оющий", "~ывший", "~оемый", "~ытый";
    gerund: "~оя", "~ыв|~ывши";
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
    // Стать: 81 words (стать, устать, застрять, перестать, встать, надеть). Like Залипнуть, except in 8 cells: infinitive, past m, imperative sg, imperative pl, participle pres_act, participle past_act, ….
    conjugation!("стать", 81, "~ть", [aspect: Perfective], like Залипнуть;
    infinitive: "~ть";
    past: "~л", _, _, _;
    imperative: "~нь", "~ньте";
    participle: derived, "~вший", _, "~тый";
    gerund: _, "~в|~вши";
    ),
    // Зарядить: 79 words (щадить, пощадить, опередить, зарядить, отрядить, снарядить). Like Разводить, except in 1 cell: participle past_pass.
    conjugation!("зарядить", 79, "~дить", like Разводить;
    participle: _, _, _, "~жённый";
    ),
    // Отдавать: 77 words (давать, воссоздавать, создавать, издавать, уставать, предавать).
    conjugation!("отдавать", 77, "~вать";
    infinitive: "~вать";
    non_past_1: "~ю", "~ём";
    non_past_2: "~ёшь", "~ёте";
    non_past_3: "~ёт", "~ют";
    past: "~вал", "~вала", "~вало", "~вали";
    imperative: "~вай", "~вайте";
    participle: "~ющий", "~вавший", "~ваемый", _;
    gerund: "~вая", "~вав|~вавши";
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
    // Бухтеть: 68 words (лететь, хотеть, полететь, захотеть, взлететь, вертеть). Like Хватить, except in 7 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("бухтеть", 68, "~теть", like Хватить;
    infinitive: "~теть";
    past: "~тел", "~тела", "~тело", "~тели";
    participle: _, "~тевший", _, _;
    gerund: _, "~тев|~тевши";
    ),
    // Забить: 68 words (пить, бить, лить, вить, шить, побить).
    conjugation!("забить", 68, "~ить";
    infinitive: "~ить";
    non_past_1: "~ью", "~ьём";
    non_past_2: "~ьёшь", "~ьёте";
    non_past_3: "~ьёт", "~ьют";
    past: "~ил", "~ила", "~ило", "~или";
    imperative: "~ей", "~ейте";
    participle: "~ьющий", "~ивший", _, "~итый";
    gerund: "~ья", "~ив|~ивши";
    ),
    // Сблизиться: 66 words (приблизиться, сконфузиться, близиться, снизиться, сузиться, сблизиться). Like Выразиться, except in 2 cells: imperative sg, imperative pl.
    conjugation!("сблизиться", 66, "~зить~я", like Выразиться;
    imperative: "~зь~я", "~зьте~ь";
    ),
    // Отвоевать: 63 words (мухлевать, горевать, малевать, полевать, царевать, чаевать).
    conjugation!("отвоевать", 63, "~евать";
    infinitive: "~евать";
    non_past_1: "~юю", "~юем";
    non_past_2: "~юешь", "~юете";
    non_past_3: "~юет", "~юют";
    past: "~евал", "~евала", "~евало", "~евали";
    imperative: "~юй", "~юйте";
    participle: "~юющий", "~евавший", "~юемый", "~ёванный";
    gerund: "~юя", "~евав|~евавши";
    ),
    // Стоить: 62 words (беспокоить, строить, стоить, устроить, построить, усвоить). Like Ударить, except in 2 cells: imperative sg, imperative pl.
    conjugation!("стоить", 62, "~ить", like Ударить;
    imperative: "~й", "~йте";
    ),
    // Зевать: 61 words (зевать, оседлать, хлебать, седлать, верстать, сверстать).
    conjugation!("зевать", 61, "~е~ть";
    infinitive: "~е~ть";
    non_past_1: "~е~ю", "~е~ем";
    non_past_2: "~е~ешь", "~е~ете";
    non_past_3: "~е~ет", "~е~ют";
    past: "~е~л", "~е~ла", "~е~ло", "~е~ли";
    imperative: "~е~й", "~е~йте";
    participle: "~е~ющий", "~е~вший", "~е~емый", "~ё~нный";
    gerund: "~е~я", "~е~в|~е~вши";
    ),
    // Обтечь: 60 words (печь, испечь, спечь, стечь, течь, развлечь).
    conjugation!("обтечь", 60, "~ечь", [aspect: Perfective];
    infinitive: "~ечь";
    non_past_1: "~еку", "~ечём";
    non_past_2: "~ечёшь", "~ечёте";
    non_past_3: "~ечёт", "~екут";
    past: "~ёк", "~екла", "~екло", "~екли";
    imperative: "~еки", "~еките";
    participle: "~екущий", "~ёкший", "~екомый", "~ечённый";
    gerund: _, "~ёкши";
    ),
    // Возбудить: 59 words (освободить, победить, предупредить, повредить, руководить, обсудить). Like Разводить, except in 1 cell: participle past_pass.
    conjugation!("возбудить", 59, "~дить", like Разводить;
    participle: _, _, _, "~ждённый";
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
    infinitive: "~е~уть";
    non_past_1: "~е~у", "~е~ём";
    non_past_2: "~е~ёшь", "~е~ёте";
    non_past_3: "~е~ёт", "~е~ут";
    past: "~е~ул", "~е~ула", "~е~уло", "~е~ули";
    imperative: "~е~и", "~е~ите";
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
    // Сунуться: 57 words (двинуться, прикинуться, кинуться, дотронуться, тронуться, ринуться). Like Тянуться, except in 3 cells: imperative sg, imperative pl, participle pres_act.
    conjugation!("сунуться", 57, "~уть~я", [aspect: Perfective], like Тянуться;
    imperative: "~ь~я", "~ьте~ь";
    participle: derived, _, _, _;
    ),
    // Колоть: 55 words (полоть, колоть, пороть, выполоть, прополоть, выпороть).
    conjugation!("колоть", 55, "~оть";
    infinitive: "~оть";
    non_past_1: "~ю", "~ем";
    non_past_2: "~ешь", "~ете";
    non_past_3: "~ет", "~ют";
    past: "~ол", "~ола", "~оло", "~оли";
    imperative: "~и", "~ите";
    participle: "~ющий", "~овший", _, "~отый";
    gerund: "~я", "~ов|~овши";
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
    // Заглохнуть: 54 words (достигнуть, проникнуть, возникнуть, затихнуть, подвергнуть, расторгнуть). Like Залипнуть, except in 4 cells: participle pres_act, participle past_act, participle past_pass, gerund past.
    conjugation!("заглохнуть", 54, "~нуть", [aspect: Perfective], like Залипнуть;
    participle: derived, "~ший|~нувший", _, "~нутый";
    gerund: _, "~нув|~ши|~нувши";
    ),
    // Идти: 53 words (идти, пасти, спасти, ползти, припасти, упасти). Like Засосать, except in 9 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("идти", 53, "~ти", like Засосать;
    infinitive: "~ти";
    past: "~", "~ла", "~ло", "~ли";
    participle: _, "~ший", "~омый", "~ённый";
    gerund: _, "~ши";
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
    infinitive: "~~аться";
    non_past_1: "~шу~ь", "~шем~я";
    non_past_2: "~шешь~я", "~шете~ь";
    non_past_3: "~шет~я", "~шут~я";
    past: "~~ался", "~~алась", "~~алось", "~~ались";
    imperative: "~ши~ь", "~шите~ь";
    participle: "~шущий~я", "~~авшийся", _, _;
    gerund: "~ша~ь", "~~авшись";
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
    infinitive: "~сть";
    non_past_1: "~ду", "~дём";
    non_past_2: "~дёшь", "~дёте";
    non_past_3: "~дёт", "~дут";
    past: "~л", "~ла", "~ло", "~ли";
    imperative: "~ди", "~дите";
    participle: "~дущий", "~вший", "~домый", "~денный";
    gerund: "~дя", "~в|~вши";
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
    // Глядеться: 48 words (увидеться, видеться, привидеться, вглядеться, наглядеться, оглядеться). Like Расходиться, except in 7 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("глядеться", 48, "~деть~я", like Расходиться;
    infinitive: "~деть~я";
    past: "~дел~я", "~дела~ь", "~дело~ь", "~дели~ь";
    participle: _, "~девший~я", _, _;
    gerund: _, "~девши~ь";
    ),
    // Развести: 48 words (провести, перевести, увести, завести, произвести, извести).
    conjugation!("развести", 48, "~ести", [aspect: Perfective];
    infinitive: "~ести";
    non_past_1: "~еду", "~едём";
    non_past_2: "~едёшь", "~едёте";
    non_past_3: "~едёт", "~едут";
    past: "~ёл", "~ела", "~ело", "~ели";
    imperative: "~еди", "~едите";
    participle: _, "~едший", _, "~едённый";
    gerund: _, "~едя|~едши";
    ),
    // Разбить: 46 words (сшить, разбить, сбить, свить, развить, разлить).
    conjugation!("разбить", 46, "~~ить", [aspect: Perfective];
    infinitive: "~~ить";
    non_past_1: "~о~ью", "~о~ьём";
    non_past_2: "~о~ьёшь", "~о~ьёте";
    non_past_3: "~о~ьёт", "~о~ьют";
    past: "~~ил", "~~ила", "~~ило", "~~или";
    imperative: "~~ей", "~~ейте";
    participle: _, "~~ивший", _, "~~итый";
    gerund: _, "~~ив|~~ивши";
    ),
    // Снести: 45 words (изнести, принести, привезти, произнести, донести, перенести). Like Перевернуть, except in 8 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("снести", 45, "~е~ти", [aspect: Perfective], like Перевернуть;
    infinitive: "~е~ти";
    past: "~ё~", "~е~ла", "~е~ло", "~е~ли";
    participle: _, "~ё~ший", _, "~е~ённый";
    gerund: _, "~е~я|~ё~ши";
    ),
    // Клевать: 45 words (блевать, плевать, клевать, жевать, пожевать, наплевать). Like Отвоевать, except in 5 cells: non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg, participle pres_pass.
    conjugation!("клевать", 45, "~евать", like Отвоевать;
    non_past_1: _, "~юём";
    non_past_2: "~юёшь", "~юёте";
    non_past_3: "~юёт", _;
    participle: _, _, derived, _;
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
    // Драть: 42 words (брать, избрать, собрать, удрать, забрать, убрать).
    conjugation!("драть", 42, "~~ать";
    infinitive: "~~ать";
    non_past_1: "~е~у", "~е~ём";
    non_past_2: "~е~ёшь", "~е~ёте";
    non_past_3: "~е~ёт", "~е~ут";
    past: "~~ал", "~~ала", "~~ало", "~~али";
    imperative: "~е~и", "~е~ите";
    participle: "~е~ущий", "~~авший", "~е~омый", "~~анный";
    gerund: "~е~я", "~~ав|~~авши";
    ),
    // Пыхать: 42 words (въехать, махать, брехать, объехать, подъехать, пахать).
    conjugation!("пыхать", 42, "~хать";
    infinitive: "~хать";
    non_past_1: "~шу", "~шем";
    non_past_2: "~шешь", "~шете";
    non_past_3: "~шет", "~шут";
    past: "~хал", "~хала", "~хало", "~хали";
    imperative: "~ши", "~шите";
    participle: "~шущий", "~хавший", _, "~ханный";
    gerund: "~ша", "~хав|~хавши";
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
    // Выписать: 40 words (писать, написать, подписать, приписать, описать, записать). Like Пыхать, except in 8 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("выписать", 40, "~сать", like Пыхать;
    infinitive: "~сать";
    past: "~сал", "~сала", "~сало", "~сали";
    participle: _, "~савший", _, "~санный";
    gerund: _, "~сав|~савши";
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
    // Сволочь: 39 words (мочь, смочь, помочь, сволочь, волочь, приволочь).
    conjugation!("сволочь", 39, "~чь";
    infinitive: "~чь";
    non_past_1: "~ку", "~чём";
    non_past_2: "~чёшь", "~чёте";
    non_past_3: "~чёт", "~кут";
    past: "~к", "~кла", "~кло", "~кли";
    imperative: "~ки", "~ките";
    participle: "~кущий", "~кший", _, "~чённый";
    gerund: _, "~кши";
    ),
    // Служить: 37 words (сообщить, обобщить, приобщить, истощить, разобщить, переборщить). Like Палить, except in 4 cells: non_past_1 sg, non_past_3 pl, participle pres_act, gerund pres.
    conjugation!("служить", 37, "~ить", like Палить;
    non_past_1: "~у", _;
    non_past_3: _, "~ат";
    participle: "~ащий", _, _, _;
    gerund: "~а", _;
    ),
    // Обратиться: 37 words (превратиться, обратиться, прекратиться, возвратиться, смутиться, возмутиться). Like Закатиться, except in 2 cells: non_past_1 sg, gerund past.
    conjugation!("обратиться", 37, "~тить~я", [aspect: Perfective], like Закатиться;
    non_past_1: "~щу~ь", _;
    gerund: _, "~тивши~ь|~тя~ь";
    ),
    // Трястись: 36 words (трястись, пастись, сотрястись, спастись, соблюстись, затрястись). Like Сорваться, except in 7 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("трястись", 36, "~ти~ь", like Сорваться;
    infinitive: "~ти~ь";
    past: "~~я", "~ла~ь", "~ло~ь", "~ли~ь";
    participle: _, "~ший~я", _, _;
    gerund: _, "~ши~ь";
    ),
    // Загореться: 36 words (рассмотреться, смотреться, загореться, предусмотреться, просмотреться, присмотреться). Like Числиться, except in 7 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("загореться", 36, "~еть~я", like Числиться;
    infinitive: "~еть~я";
    past: "~ел~я", "~ела~ь", "~ело~ь", "~ели~ь";
    participle: _, "~евший~я", _, _;
    gerund: _, "~евши~ь";
    ),
    // Гореть: 35 words (сгореть, смотреть, гореть, посмотреть, велеть, загореть). Like Кишеть, except in 1 cell: non_past_1 sg.
    conjugation!("гореть", 35, "~еть", like Кишеть;
    non_past_1: "~ю", _;
    ),
    // Щипать: 35 words (засыпать, дремать, задремать, сыпать, насыпать, высыпать).
    conjugation!("щипать", 35, "~ать";
    infinitive: "~ать";
    non_past_1: "~лю", "~лем";
    non_past_2: "~лешь", "~лете";
    non_past_3: "~лет", "~лют";
    past: "~ал", "~ала", "~ало", "~али";
    imperative: "~ли", "~лите";
    participle: "~лющий", "~авший", "~аемый", "~анный";
    gerund: "~ля", "~ав|~авши";
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
    infinitive: "~ести";
    non_past_1: "~ету", "~етём";
    non_past_2: "~етёшь", "~етёте";
    non_past_3: "~етёт", "~етут";
    past: "~ёл", "~ела", "~ело", "~ели";
    imperative: "~ети", "~етите";
    participle: _, "~ётший", _, "~етённый";
    gerund: _, "~етя|~ётши";
    ),
    // Расчесать: 33 words (чесать, тесать, расчесать, почесать, причесать, обтесать).
    conjugation!("расчесать", 33, "~есать";
    infinitive: "~есать";
    non_past_1: "~ешу", "~ешем";
    non_past_2: "~ешешь", "~ешете";
    non_past_3: "~ешет", "~ешут";
    past: "~есал", "~есала", "~есало", "~есали";
    imperative: "~еши", "~ешите";
    participle: "~ешущий", "~есавший", _, "~ёсанный";
    gerund: "~еша", "~есав|~есавши";
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
    // Обыскать: 31 words (искать, полоскать, отыскать, поискать, разыскать, обыскать).
    conjugation!("обыскать", 31, "~скать";
    infinitive: "~скать";
    non_past_1: "~щу", "~щем";
    non_past_2: "~щешь", "~щете";
    non_past_3: "~щет", "~щут";
    past: "~скал", "~скала", "~скало", "~скали";
    imperative: "~щи", "~щите";
    participle: "~щущий", "~скавший", "~скаемый", "~сканный";
    gerund: "~ща", "~скав|~скавши";
    ),
    // Сковать: 30 words (ковать, подковать, совать, основать, сновать, приковать). Like Вибрировать, except in 5 cells: non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg, participle pres_pass.
    conjugation!("сковать", 30, "~овать", like Вибрировать;
    non_past_1: _, "~уём";
    non_past_2: "~уёшь", "~уёте";
    non_past_3: "~уёт", _;
    participle: _, _, derived, _;
    ),
    // Грести: 30 words (грести, погрести, скрести, сгрести, разгрести, поскрести).
    conjugation!("грести", 30, "~ести";
    infinitive: "~ести";
    non_past_1: "~ебу", "~ебём";
    non_past_2: "~ебёшь", "~ебёте";
    non_past_3: "~ебёт", "~ебут";
    past: "~ёб", "~ебла", "~ебло", "~ебли";
    imperative: "~еби", "~ебите";
    participle: "~ебущий", "~ёбший", "~ебомый", "~ебённый";
    gerund: "~ебя", "~ёбши";
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
    // Вывестись: 28 words (вестись, развестись, произвестись, свестись, довестись, привестись).
    conjugation!("вывестись", 28, "~е~тись", [aspect: Perfective];
    infinitive: "~е~тись";
    non_past_1: "~еду~ь", "~едём~я";
    non_past_2: "~едёшь~я", "~едёте~ь";
    non_past_3: "~едёт~я", "~едут~я";
    past: "~ёл~я", "~ела~ь", "~ело~ь", "~ели~ь";
    imperative: "~еди~ь", "~едите~ь";
    participle: _, "~едший~я", _, _;
    gerund: _, "~едя~ь";
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
    // Зайти: 27 words (произойти, найти, пойти, дойти, выйти, прийти).
    conjugation!("зайти", 27, "~йти", [aspect: Perfective];
    infinitive: "~йти";
    non_past_1: "~йду", "~йдём";
    non_past_2: "~йдёшь", "~йдёте";
    non_past_3: "~йдёт", "~йдут";
    past: "~шёл", "~шла", "~шло", "~шли";
    imperative: "~йди", "~йдите";
    participle: _, "~шедший", _, "~йдённый";
    gerund: _, "~йдя|~шедши";
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
    // Плакать: 26 words (плакать, тыкать, оплакать, кликать, заплакать, поплакать).
    conjugation!("плакать", 26, "~кать";
    infinitive: "~кать";
    non_past_1: "~чу", "~чем";
    non_past_2: "~чешь", "~чете";
    non_past_3: "~чет", "~чут";
    past: "~кал", "~кала", "~кало", "~кали";
    imperative: "~чь", "~чьте";
    participle: "~чущий", "~кавший", _, "~канный";
    gerund: "~ча", "~кав|~кавши";
    ),
    // Просвистеть: 26 words (блестеть, свистеть, хрустеть, заблестеть, насвистеть, посвистеть). Like Отпустить, except in 8 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("просвистеть", 26, "~стеть", like Отпустить;
    infinitive: "~стеть";
    past: "~стел", "~стела", "~стело", "~стели";
    participle: _, "~стевший", _, derived;
    gerund: _, "~стев|~стевши";
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
    // Наплыть: 25 words (жить, плыть, отплыть, ожить, зажить, пожить).
    conjugation!("наплыть", 25, "~ть";
    infinitive: "~ть";
    non_past_1: "~ву", "~вём";
    non_past_2: "~вёшь", "~вёте";
    non_past_3: "~вёт", "~вут";
    past: "~л", "~ла", "~ло", "~ли";
    imperative: "~ви", "~вите";
    participle: "~вущий", "~вший", _, _;
    gerund: "~вя", "~в|~вши";
    ),
    // Тухнуть: 25 words (исчезнуть, привыкнуть, крепнуть, гибнуть, дрыхнуть, гаснуть). Like Залипнуть, except in 3 cells: past m, participle past_act, gerund past.
    conjugation!("тухнуть", 25, "~нуть", like Залипнуть;
    past: "~|~нул", _, _, _;
    participle: _, "~нувший", _, _;
    gerund: _, "~нув|~нувши";
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
    // Отнять: 24 words (поднять, снять, обнять, отнять, приподнять, подъять).
    conjugation!("отнять", 24, "~ять", [aspect: Perfective];
    infinitive: "~ять";
    non_past_1: "~иму", "~имем";
    non_past_2: "~имешь", "~имете";
    non_past_3: "~имет", "~имут";
    past: "~ял", "~яла", "~яло", "~яли";
    imperative: "~ими", "~имите";
    participle: _, "~явший", _, "~ятый";
    gerund: _, "~яв|~явши";
    ),
    // Проскочить: 23 words (тащить, вытащить, затащить, потащить, притащить, стащить). Like Служить, except in 1 cell: participle past_pass.
    conjugation!("проскочить", 23, "~ить", [aspect: Perfective], like Служить;
    participle: _, _, _, "~енный";
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
    infinitive: "~е~еть";
    non_past_1: "~~у", "~~ём";
    non_past_2: "~~ёшь", "~~ёте";
    non_past_3: "~~ёт", "~~ут";
    past: "~ё~", "~ё~ла", "~ё~ло", "~ё~ли";
    imperative: "~~и", "~~ите";
    participle: _, "~ё~ший", _, "~ё~тый";
    gerund: _, "~е~ев|~ё~ши";
    ),
    // Перепеть: 22 words (петь, допеть, реветь, запеть, напеть, пропеть).
    conjugation!("перепеть", 22, "~еть";
    infinitive: "~еть";
    non_past_1: "~ою", "~оём";
    non_past_2: "~оёшь", "~оёте";
    non_past_3: "~оёт", "~оют";
    past: "~ел", "~ела", "~ело", "~ели";
    imperative: "~ой", "~ойте";
    participle: "~оющий", "~евший", _, "~етый";
    gerund: "~оя", "~ев|~евши";
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
    infinitive: "~а~ти";
    non_past_1: "~а~ту", "~а~тём";
    non_past_2: "~а~тёшь", "~а~тёте";
    non_past_3: "~а~тёт", "~а~тут";
    past: "~о~", "~о~ла", "~о~ло", "~о~ли";
    imperative: "~а~ти", "~а~тите";
    participle: "~а~тущий", "~о~ший", _, _;
    gerund: "~а~тя", "~о~ши";
    ),
    // Забыть: 21 words (быть, прибыть, забыть, добыть, побыть, сбыть).
    conjugation!("забыть", 21, "~ыть", [aspect: Perfective];
    infinitive: "~ыть";
    non_past_1: "~уду", "~удем";
    non_past_2: "~удешь", "~удете";
    non_past_3: "~удет", "~удут";
    past: "~ыл", "~ыла", "~ыло", "~ыли";
    imperative: "~удь", "~удьте";
    participle: _, "~ывший", _, "~ытый";
    gerund: _, "~ыв|~ывши";
    ),
    // Унестись: 21 words (произнестись, донестись, отнестись, пронестись, понестись, перенестись). Like Набраться, except in 9 cells: infinitive, past m, past f, past n, past pl, participle pres_act, ….
    conjugation!("унестись", 21, "~е~ти~ь", [aspect: Perfective], like Набраться;
    infinitive: "~е~ти~ь";
    past: "~ё~~я", "~е~ла~ь", "~е~ло~ь", "~е~ли~ь";
    participle: derived, "~ё~ший~я", _, _;
    gerund: derived, "~е~я~ь|~ё~ши~ь";
    ),
    // Набежать: 20 words (бежать, побежать, избежать, выбежать, убежать, сбежать).
    conjugation!("набежать", 20, "~жать", [aspect: Perfective];
    infinitive: "~жать";
    non_past_1: "~гу", "~жим";
    non_past_2: "~жишь", "~жите";
    non_past_3: "~жит", "~гут";
    past: "~жал", "~жала", "~жало", "~жали";
    imperative: "~ги", "~гите";
    participle: "~гущий", "~жавший", _, _;
    gerund: _, "~жав|~жавши";
    ),
    // Расстаться: 20 words (остаться, расстаться, одеться, раздеться, переодеться, достаться). Like Вторгнуться, except in 7 cells: infinitive, past m, imperative sg, imperative pl, participle pres_act, participle past_act, ….
    conjugation!("расстаться", 20, "~ть~я", [aspect: Perfective], like Вторгнуться;
    infinitive: "~ть~я";
    past: "~л~я", _, _, _;
    imperative: "~нь~я", "~ньте~ь";
    participle: derived, "~вший~я", _, _;
    gerund: _, "~вши~ь";
    ),
    // Грызть: 20 words (грызть, обгрызть, огрызть, догрызть, загрызть, изгрызть). Like Идти, except in 2 cells: infinitive, participle past_pass.
    conjugation!("грызть", 20, "~ть", like Идти;
    infinitive: "~ть";
    participle: _, _, _, "~енный";
    ),
    // Лезть: 20 words (залезть, лезть, полезть, вылезть, влезть, слезть). Like Грызть, except in 8 cells: non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg, imperative sg, imperative pl, ….
    conjugation!("лезть", 20, "~ть", [aspect: Perfective], like Грызть;
    non_past_1: _, "~ем";
    non_past_2: "~ешь", "~ете";
    non_past_3: "~ет", _;
    imperative: "~ь", "~ьте";
    participle: _, _, derived, derived;
    ),
    // Зашибить: 20 words (вышибить, зашибить, ушибить, сшибить, прошибить, пришибить). Like Идти, except in 7 cells: infinitive, participle pres_act, participle past_act, participle pres_pass, participle past_pass, gerund pres, ….
    conjugation!("зашибить", 20, "~ить", [aspect: Perfective], like Идти;
    infinitive: "~ить";
    participle: derived, "~ивший", derived, "~ленный";
    gerund: derived, "~ив|~ивши";
    ),
    // Занять: 19 words (понять, занять, нанять, унять, донять, перенять).
    conjugation!("занять", 19, "~нять", [aspect: Perfective];
    infinitive: "~нять";
    non_past_1: "~йму", "~ймём";
    non_past_2: "~ймёшь", "~ймёте";
    non_past_3: "~ймёт", "~ймут";
    past: "~нял", "~няла", "~няло", "~няли";
    imperative: "~йми", "~ймите";
    participle: _, "~нявший", _, "~нятый";
    gerund: _, "~няв|~нявши";
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
    infinitive: "~~каться";
    non_past_1: "~щу~ь", "~щем~я";
    non_past_2: "~щешь~я", "~щете~ь";
    non_past_3: "~щет~я", "~щут~я";
    past: "~~кался", "~~калась", "~~калось", "~~кались";
    imperative: "~щи~ь", "~щите~ь";
    participle: "~щущий~я", "~~кавшийся", _, _;
    gerund: "~ща~ь", "~~кавшись";
    ),
    // Предназначить: 18 words (встопорщить, вывощить, наплющить, натопорщить, повытащить, поморщить). Like Проскочить, except in 2 cells: imperative sg, imperative pl.
    conjugation!("предназначить", 18, "~ить", [aspect: Perfective], like Проскочить;
    imperative: "~ь", "~ьте";
    ),
    // Просмотреть: 18 words (презреть, рассмотреть, предусмотреть, осмотреть, подсмотреть, присмотреть). Like Гореть, except in 4 cells: participle pres_act, participle pres_pass, participle past_pass, gerund pres.
    conjugation!("просмотреть", 18, "~еть", [aspect: Perfective], like Гореть;
    participle: derived, _, derived, "~енный";
    gerund: derived, _;
    ),
    // Въесться: 18 words (наесться, въесться, объесться, приесться, взъесться, заесться). Like Отдаться, except in 7 cells: infinitive, non_past_3 pl, past m, past n, past pl, imperative sg, ….
    conjugation!("въесться", 18, "~~ться", like Отдаться;
    infinitive: "~~ться";
    non_past_3: _, "~дят~я";
    past: "~л~я", _, "~ло~ь", "~ли~ь";
    imperative: "~шь~я", "~шьте~ь";
    ),
    // Потчевать: 18 words (потчевать, выкорчевать, вымежевать, запотчевать, напотчевать, отпотчевать). Like Сычевать, except in 1 cell: participle past_pass.
    conjugation!("потчевать", 18, "~евать", like Сычевать;
    participle: _, _, _, "~еванный";
    ),
    // Устояться: 17 words (бояться, состояться, забояться, побояться, устояться, застояться). Like Развеяться, except in 6 cells: non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg, non_past_3 pl, participle pres_act.
    conjugation!("устояться", 17, "~ять~я", like Развеяться;
    non_past_1: _, "~им~я";
    non_past_2: "~ишь~я", "~ите~ь";
    non_past_3: "~ит~я", "~ят~я";
    participle: "~ящий~я", _, _, _;
    ),
    // Трепетать: 17 words (клеветать, трепетать, роптать, скрежетать, затрепетать, оклеветать). Like Обыскать, except in 9 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("трепетать", 17, "~тать", like Обыскать;
    infinitive: "~тать";
    past: "~тал", "~тала", "~тало", "~тали";
    participle: _, "~тавший", derived, derived;
    gerund: _, "~тав|~тавши";
    ),
    // Жать: 17 words (нажать, выжать, жать, пожать, зажать, прижать).
    conjugation!("жать", 17, "~ать";
    infinitive: "~ать";
    non_past_1: "~му", "~мём";
    non_past_2: "~мёшь", "~мёте";
    non_past_3: "~мёт", "~мут";
    past: "~ал", "~ала", "~ало", "~али";
    imperative: "~ми", "~мите";
    participle: "~мущий", "~авший", _, "~атый";
    gerund: "~мя", "~ав|~авши";
    ),
    // Вгрызться: 17 words (вгрызться, нагрызться, отгрызться, погрызться, разлезться, прогрызться). Like Трястись, except in 1 cell: infinitive.
    conjugation!("вгрызться", 17, "~ть~я", like Трястись;
    infinitive: "~ть~я";
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
    infinitive: "~о~~ать";
    non_past_1: "~~е~у", "~~е~ём";
    non_past_2: "~~е~ёшь", "~~е~ёте";
    non_past_3: "~~е~ёт", "~~е~ут";
    past: "~о~~ал", "~о~~ала", "~о~~ало", "~о~~али";
    imperative: "~~е~и", "~~е~ите";
    participle: _, "~о~~авший", _, "~о~~анный";
    gerund: _, "~о~~ав|~о~~авши";
    ),
    // Выровнять: 16 words (выровнять, приравнять, прировнять, сравнять, сровнять, уравнять). Like Отнять, except in 9 cells: non_past_1 sg, non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg, non_past_3 pl, ….
    conjugation!("выровнять", 16, "~ять", [aspect: Perfective], like Отнять;
    non_past_1: "~яю", "~яем";
    non_past_2: "~яешь", "~яете";
    non_past_3: "~яет", "~яют";
    imperative: "~яй", "~яйте";
    participle: _, _, _, "~енный";
    ),
    // Трепать: 16 words (трепать, потрепать, растрепать, истрепать, встрепать, затрепать).
    conjugation!("трепать", 16, "~е~ать";
    infinitive: "~е~ать";
    non_past_1: "~е~лю", "~е~лем";
    non_past_2: "~е~лешь", "~е~лете";
    non_past_3: "~е~лет", "~е~лют";
    past: "~е~ал", "~е~ала", "~е~ало", "~е~али";
    imperative: "~е~ли", "~е~лите";
    participle: "~е~лющий", "~е~авший", _, "~ё~анный";
    gerund: "~е~ля", "~е~ав|~е~авши";
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
    // Честь: 15 words (прочесть, честь, предпочесть, вычесть, учесть, почесть).
    conjugation!("честь", 15, "~есть";
    infinitive: "~есть";
    non_past_1: "~ту", "~тём";
    non_past_2: "~тёшь", "~тёте";
    non_past_3: "~тёт", "~тут";
    past: "~ёл", "~ла", "~ло", "~ли";
    imperative: "~ти", "~тите";
    participle: _, _, _, "~тённый";
    gerund: _, "~тя|~ётши";
    ),
    // Насесть: 15 words (сесть, засесть, присесть, осесть, подсесть, пересесть).
    conjugation!("насесть", 15, "~есть", [aspect: Perfective];
    infinitive: "~есть";
    non_past_1: "~яду", "~ядем";
    non_past_2: "~ядешь", "~ядете";
    non_past_3: "~ядет", "~ядут";
    past: "~ел", "~ела", "~ело", "~ели";
    imperative: "~ядь", "~ядьте";
    participle: _, "~евший", _, _;
    gerund: _, "~ев|~евши";
    ),
    // Смеяться: 15 words (смеяться, засмеяться, рассмеяться, посмеяться, досмеяться, надсмеяться). Like Развеяться, except in 4 cells: non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg.
    conjugation!("смеяться", 15, "~ять~я", like Развеяться;
    non_past_1: _, "~ём~я";
    non_past_2: "~ёшь~я", "~ёте~ь";
    non_past_3: "~ёт~я", _;
    ),
    // Гнать: 15 words (гнать, догнать, выгнать, погнать, пригнать, изгнать).
    conjugation!("гнать", 15, "~~ать";
    infinitive: "~~ать";
    non_past_1: "~о~ю", "~о~им";
    non_past_2: "~о~ишь", "~о~ите";
    non_past_3: "~о~ит", "~о~ят";
    past: "~~ал", "~~ала", "~~ало", "~~али";
    imperative: "~о~и", "~о~ите";
    participle: _, "~~авший", _, "~~анный";
    gerund: _, "~~ав|~~авши";
    ),
    // Помять: 15 words (мять, помять, распять, замять, домять, вымять).
    conjugation!("помять", 15, "~ять";
    infinitive: "~ять";
    non_past_1: "~ну", "~нём";
    non_past_2: "~нёшь", "~нёте";
    non_past_3: "~нёт", "~нут";
    past: "~ял", "~яла", "~яло", "~яли";
    imperative: "~ни", "~ните";
    participle: "~нущий", "~явший", _, "~ятый";
    gerund: "~ня", "~яв|~явши";
    ),
    // Зиждить: 15 words (стелить, постелить, застелить, расстелить, устелить, зиждить). Like Выделить, except in 6 cells: non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg, non_past_3 pl, participle pres_act.
    conjugation!("зиждить", 15, "~ить", like Выделить;
    non_past_1: _, "~ем";
    non_past_2: "~ешь", "~ете";
    non_past_3: "~ет", "~ют";
    participle: "~ющий", _, _, _;
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
    infinitive: "~ечь";
    non_past_1: "~егу", "~ежём";
    non_past_2: "~ежёшь", "~ежёте";
    non_past_3: "~ежёт", "~егут";
    past: "~ёг", "~егла", "~егло", "~егли";
    imperative: "~еги", "~егите";
    participle: "~егущий", "~ёгший", "~егомый", "~ежённый";
    gerund: _, "~ёгши";
    ),
    // Видеть: 14 words (видеть, увидеть, ненавидеть, предвидеть, возненавидеть, завидеть). Like Сидеть, except in 3 cells: imperative sg, imperative pl, participle past_pass.
    conjugation!("видеть", 14, "~деть", like Сидеть;
    imperative: "~дь", "~дьте";
    participle: _, _, _, "~денный";
    ),
    // Прожечь: 14 words (жечь, зажечь, выжечь, пожечь, прижечь, прожечь). Like Постричь, except in 5 cells: infinitive, past m, participle past_act, participle past_pass, gerund past.
    conjugation!("прожечь", 14, "~ечь", like Постричь;
    infinitive: "~ечь";
    past: "~ёг", _, _, _;
    participle: _, "~ёгший", _, "~жённый";
    gerund: _, "~ёгши";
    ),
    // Везти: 14 words (нести, везти, заебти, поебти, взъебти, уебти). Like Снести, except in 4 cells: participle pres_act, participle pres_pass, gerund pres, gerund past.
    conjugation!("везти", 14, "~е~ти", like Снести;
    participle: "~е~ущий", _, "~е~омый", _;
    gerund: "~е~я", "~ё~ши";
    ),
    // Молоть: 14 words (молоть, смолоть, домолоть, вымолоть, замолоть, намолоть).
    conjugation!("молоть", 14, "~о~оть";
    infinitive: "~о~оть";
    non_past_1: "~е~ю", "~е~ем";
    non_past_2: "~е~ешь", "~е~ете";
    non_past_3: "~е~ет", "~е~ют";
    past: "~о~ол", "~о~ола", "~о~оло", "~о~оли";
    imperative: "~е~и", "~е~ите";
    participle: "~е~ющий", "~о~овший", _, "~о~отый";
    gerund: "~е~я", "~о~ов|~о~овши";
    ),
    // Прискакать: 14 words (скакать, отскакать, заскакать, поскакать, наскакать, прискакать). Like Плакать, except in 3 cells: imperative sg, imperative pl, participle past_pass.
    conjugation!("прискакать", 14, "~кать", like Плакать;
    imperative: "~чи", "~чите";
    participle: _, _, _, derived;
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
    // Просвистать: 14 words (свистать, насвистать, посвистать, дристать, просвистать, высвистать). Like Трепетать, except in 8 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("просвистать", 14, "~стать", like Трепетать;
    infinitive: "~стать";
    past: "~стал", "~стала", "~стало", "~стали";
    participle: _, "~ставший", _, "~станный";
    gerund: _, "~став|~ставши";
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
    // Толочь: 13 words (толочь, столочь, потолочь, истолочь, растолочь, втолочь). Like Сволочь, except in 4 cells: infinitive, past m, participle past_act, gerund past.
    conjugation!("толочь", 13, "~очь", like Сволочь;
    infinitive: "~очь";
    past: "~ок", _, _, _;
    participle: _, "~окший", _, _;
    gerund: _, "~окши";
    ),
    // Мяться: 13 words (замяться, мяться, помяться, примяться, вымяться, домяться).
    conjugation!("мяться", 13, "~ять~я";
    infinitive: "~ять~я";
    non_past_1: "~ну~ь", "~нём~я";
    non_past_2: "~нёшь~я", "~нёте~ь";
    non_past_3: "~нёт~я", "~нут~я";
    past: "~ял~я", "~яла~ь", "~яло~ь", "~яли~ь";
    imperative: "~ни~ь", "~ните~ь";
    participle: _, "~явший~я", _, _;
    gerund: _, "~явши~ь";
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
    // Нагореть: 13 words (нагореть, отгореть, отзвенеть, подзагореть, позвенеть, пообгореть). Like Просмотреть, except in 1 cell: gerund past.
    conjugation!("нагореть", 13, "~еть", like Просмотреть;
    gerund: _, "~евши|~ев";
    ),
    // Вызвать: 12 words (звать, позвать, вызвать, назвать, призвать, прозвать). Like Гнать, except in 8 cells: non_past_1 sg, non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg, non_past_3 pl, ….
    conjugation!("вызвать", 12, "~~ать", like Гнать;
    non_past_1: "~о~у", "~о~ём";
    non_past_2: "~о~ёшь", "~о~ёте";
    non_past_3: "~о~ёт", "~о~ут";
    participle: "~о~ущий", _, _, _;
    gerund: "~о~я", _;
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
    infinitive: "~с~ать";
    non_past_1: "~ш~ю", "~ш~ём";
    non_past_2: "~ш~ёшь", "~ш~ёте";
    non_past_3: "~ш~ёт", "~ш~ют";
    past: "~с~ал", "~с~ала", "~с~ало", "~с~али";
    imperative: "~ш~и", "~ш~ите";
    participle: _, "~с~авший", _, "~с~анный";
    gerund: _, "~с~ав|~с~авши";
    ),
    // Брить: 12 words (брить, побрить, выбрить, обрить, добрить, забрить). Like Забить, except in 9 cells: non_past_1 sg, non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg, non_past_3 pl, ….
    conjugation!("брить", 12, "~ить", like Забить;
    non_past_1: "~ею", "~еем";
    non_past_2: "~еешь", "~еете";
    non_past_3: "~еет", "~еют";
    participle: "~еющий", _, "~еемый", _;
    gerund: "~ея", _;
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
    // Стелиться: 12 words (зиждиться, расстелиться, стелиться, созиждиться, взъемлиться, постелиться). Like Числиться, except in 6 cells: non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg, non_past_3 pl, participle pres_act.
    conjugation!("стелиться", 12, "~ить~я", like Числиться;
    non_past_1: _, "~ем~я";
    non_past_2: "~ешь~я", "~ете~ь";
    non_past_3: "~ет~я", "~ют~я";
    participle: "~ющий~я", _, _, _;
    ),
    // Подвигнуться: 12 words (свергнуться, разверзнуться, низвергнуться, подвигнуться, сверзнуться, исторгнуться). Like Вторгнуться, except in 3 cells: past m, participle pres_act, participle past_act.
    conjugation!("подвигнуться", 12, "~нуть~я", [aspect: Perfective], like Вторгнуться;
    past: derived, _, _, _;
    participle: derived, "~нувший~я|~ший~я", _, _;
    ),
    // Поехать: 12 words (поехать, ехать, приехать, уехать, проехать, выехать).
    conjugation!("поехать", 12, "~хать";
    infinitive: "~хать";
    non_past_1: "~ду", "~дем";
    non_past_2: "~дешь", "~дете";
    non_past_3: "~дет", "~дут";
    past: "~хал", "~хала", "~хало", "~хали";
    imperative: "~зжай", "~зжайте";
    participle: _, "~хавший", _, _;
    gerund: _, "~хав|~хавши";
    ),
    // Дожить: 11 words (выжить, прожить, пережить, дожить, нажить, изжить). Like Наплыть, except in 6 cells: past m, past n, past pl, participle pres_act, participle past_pass, gerund pres.
    conjugation!("дожить", 11, "~ть", [aspect: Perfective], like Наплыть;
    past: "~л|~л", _, "~ло|~ло", "~ли|~ли";
    participle: derived, _, _, "~тый|~тый";
    gerund: derived, _;
    ),
    // Ушибиться: 11 words (ошибиться, ушибиться, зашибиться, прошибиться, вышибиться, пришибиться). Like Трястись, except in 5 cells: infinitive, participle pres_act, participle past_act, gerund pres, gerund past.
    conjugation!("ушибиться", 11, "~ить~я", [aspect: Perfective], like Трястись;
    infinitive: "~ить~я";
    participle: derived, "~ивший~я", _, _;
    gerund: derived, "~ивши~ь";
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
    // Зассать: 11 words (ссать, сцать, обоссать, зассать, поссать, проссать). Like Засосать, except in 8 cells: non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg, imperative sg, imperative pl, ….
    conjugation!("зассать", 11, "~ать", like Засосать;
    non_past_1: _, "~ым";
    non_past_2: "~ышь", "~ыте";
    non_past_3: "~ыт", _;
    imperative: "~ы", "~ыте";
    participle: derived, _, _, _;
    gerund: derived, _;
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
    // Вырвать: 10 words (выебать, вырвать, выждать, высосать, выжрать, выткать). Like Засосать, except in 6 cells: non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg, participle pres_act, gerund pres.
    conjugation!("вырвать", 10, "~ать", [aspect: Perfective], like Засосать;
    non_past_1: _, "~ем";
    non_past_2: "~ешь", "~ете";
    non_past_3: "~ет", _;
    participle: derived, _, _, _;
    gerund: derived, _;
    ),
    // Плескать: 10 words (плескать, рукоплескать, расплескать, зарукоплескать, наплескать, оплескать).
    conjugation!("плескать", 10, "~ескать";
    infinitive: "~ескать";
    non_past_1: "~ещу", "~ещем";
    non_past_2: "~ещешь", "~ещете";
    non_past_3: "~ещет", "~ещут";
    past: "~ескал", "~ескала", "~ескало", "~ескали";
    imperative: "~ещи", "~ещите";
    participle: "~ещущий", "~ескавший", "~ескаемый", "~ёсканный";
    gerund: "~еща", "~ескав|~ескавши";
    ),
    // Забыться: 10 words (забыться, сбыться, добыться, избыться, перезабыться, подзабыться).
    conjugation!("забыться", 10, "~ыть~я", [aspect: Perfective];
    infinitive: "~ыть~я";
    non_past_1: "~уду~ь", "~удем~я";
    non_past_2: "~удешь~я", "~удете~ь";
    non_past_3: "~удет~я", "~удут~я";
    past: "~ыл~я", "~ыла~ь", "~ыло~ь", "~ыли~ь";
    imperative: "~удь~я", "~удьте~ь";
    participle: _, "~ывший~я", _, _;
    gerund: _, "~ывши~ь";
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
    infinitive: "~здить";
    non_past_1: "~зжу", "~здим";
    non_past_2: "~здишь", "~здите";
    non_past_3: "~здит", "~здят";
    past: "~здил", "~здила", "~здило", "~здили";
    imperative: "~зди", "~здите";
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
    // Стонать: 9 words (стонать, жаждать, застонать, простонать, высрать, возжаждать). Like Вырвать, except in 3 cells: participle pres_act, participle past_pass, gerund pres.
    conjugation!("стонать", 9, "~ать", like Вырвать;
    participle: "~ущий", _, _, derived;
    gerund: "~ая", _;
    ),
    // Стереть: 9 words (стереть, подпереть, спереть, обтереть, втереть, впереть).
    conjugation!("стереть", 9, "~~е~еть", [aspect: Perfective];
    infinitive: "~~е~еть";
    non_past_1: "~о~~у", "~о~~ём";
    non_past_2: "~о~~ёшь", "~о~~ёте";
    non_past_3: "~о~~ёт", "~о~~ут";
    past: "~~ё~", "~~ё~ла", "~~ё~ло", "~~ё~ли";
    imperative: "~о~~и", "~о~~ите";
    participle: _, "~~ё~ший", _, "~~ё~тый";
    gerund: _, "~~е~ев|~~ё~ши";
    ),
    // Выспаться: 9 words (выспаться, отоспаться, доспаться, заспаться, наспаться, проспаться). Like Держаться, except in 4 cells: non_past_1 sg, non_past_3 pl, participle pres_act, gerund pres.
    conjugation!("выспаться", 9, "~ать~я", [aspect: Perfective], like Держаться;
    non_past_1: "~лю~ь", _;
    non_past_3: _, "~ят~я";
    participle: derived, _, _, _;
    gerund: derived, _;
    ),
    // Застлать: 9 words (стлать, постлать, застлать, устлать, выстлать, достлать). Like Гнать, except in 8 cells: non_past_1 sg, non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg, non_past_3 pl, ….
    conjugation!("застлать", 9, "~~ать", like Гнать;
    non_past_1: "~е~ю", "~е~ем";
    non_past_2: "~е~ешь", "~е~ете";
    non_past_3: "~е~ет", "~е~ют";
    imperative: "~е~и", "~е~ите";
    ),
    // Размять: 9 words (смять, измять, размять, подмять, вмять, обмять).
    conjugation!("размять", 9, "~~ять", [aspect: Perfective];
    infinitive: "~~ять";
    non_past_1: "~о~ну", "~о~нём";
    non_past_2: "~о~нёшь", "~о~нёте";
    non_past_3: "~о~нёт", "~о~нут";
    past: "~~ял", "~~яла", "~~яло", "~~яли";
    imperative: "~о~ни", "~о~ните";
    participle: _, "~~явший", _, "~~ятый";
    gerund: _, "~~яв|~~явши";
    ),
    // Толочься: 9 words (толочься, затолочься, потолочься, истолочься, дотолочься, утолочься). Like Заволочься, except in 4 cells: infinitive, past m, participle past_act, gerund past.
    conjugation!("толочься", 9, "~очь~я", like Заволочься;
    infinitive: "~очь~я";
    past: "~ок~я", _, _, _;
    participle: _, "~окший~я", _, _;
    gerund: _, "~окши~ь";
    ),
    // Скрестись: 9 words (скрестись, грестись, огрестись, догрестись, угрестись, погрестись). Like Отгрестись, except in 3 cells: participle pres_act, gerund pres, gerund past.
    conjugation!("скрестись", 9, "~е~тись", like Отгрестись;
    participle: "~ебущий~я", _, _, _;
    gerund: "~ебя~ь", "~ёбши~ь";
    ),
    // Зажечься: 9 words (зажечься, ожечься, нажечься, жечься, возжечься, пережечься). Like Стричься, except in 4 cells: infinitive, past m, participle past_act, gerund past.
    conjugation!("зажечься", 9, "~ечь~я", like Стричься;
    infinitive: "~ечь~я";
    past: "~ёг~я", _, _, _;
    participle: _, "~ёгший~я", _, _;
    gerund: _, "~ёгши~ь";
    ),
    // Мести: 9 words (гнести, плести, мести, цвести, расцвести, рассвести). Like Сплести, except in 3 cells: participle pres_act, gerund pres, gerund past.
    conjugation!("мести", 9, "~ести", like Сплести;
    participle: "~етущий", _, _, _;
    gerund: "~етя", "~ётши";
    ),
    // Выпить: 8 words (выпить, вышить, выбить, вылить, подвыпить, вывить). Like Забить, except in 6 cells: non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg, participle pres_act, gerund pres.
    conjugation!("выпить", 8, "~ить", [aspect: Perfective], like Забить;
    non_past_1: _, "~ьем";
    non_past_2: "~ьешь", "~ьете";
    non_past_3: "~ьет", _;
    participle: derived, _, _, _;
    gerund: derived, _;
    ),
    // Разогнать: 8 words (подогнать, обогнать, отогнать, разогнать, согнать, вогнать). Like Разобрать, except in 8 cells: non_past_1 sg, non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg, non_past_3 pl, ….
    conjugation!("разогнать", 8, "~о~~ать", [aspect: Perfective], like Разобрать;
    non_past_1: "~~о~ю", "~~о~им";
    non_past_2: "~~о~ишь", "~~о~ите";
    non_past_3: "~~о~ит", "~~о~ят";
    imperative: "~~о~и", "~~о~ите";
    ),
    // Отозваться: 8 words (отозваться, разогнаться, взогнаться, вогнаться, обозваться, отогнаться). Like Подобраться, except in 9 cells: non_past_1 sg, non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg, non_past_3 pl, ….
    conjugation!("отозваться", 8, "~о~~ать~я", [aspect: Perfective], like Подобраться;
    non_past_1: "~~о~ю~ь", "~~о~им~я";
    non_past_2: "~~о~ишь~я", "~~о~ите~ь";
    non_past_3: "~~о~ит~я", "~~о~ят~я";
    past: "~о~~ал~я", _, _, _;
    imperative: "~~о~и~ь", "~~о~ите~ь";
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
    // Выклевать: 8 words (выклевать, выплевать, выблевать, вымалевать, заштемпелевать, проштемпелевать). Like Отвоевать, except in 4 cells: participle pres_act, participle pres_pass, participle past_pass, gerund pres.
    conjugation!("выклевать", 8, "~евать", [aspect: Perfective], like Отвоевать;
    participle: derived, _, derived, "~еванный";
    gerund: derived, _;
    ),
    // Разреветься: 8 words (выреветься, нареветься, обреветься, прореветься, разреветься, изреветься). Like Ушибиться, except in 7 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("разреветься", 8, "~еть~я", [aspect: Perfective], like Ушибиться;
    infinitive: "~еть~я";
    past: "~ел~я", "~ела~ь", "~ело~ь", "~ели~ь";
    participle: _, "~евший~я", _, _;
    gerund: _, "~евши~ь";
    ),
    // Выпариться: 8 words (закупориться, выпариться, замусориться, умайориться, откупориться, раскупориться). Like Усилиться, except in 4 cells: imperative sg, participle pres_act, gerund pres, gerund past.
    conjugation!("выпариться", 8, "~ить~я", [aspect: Perfective], like Усилиться;
    imperative: "~ь~я|~и~ь", _;
    participle: derived, _, _, _;
    gerund: derived, "~я~ь|~ивши~ь";
    ),
    // Месть: 8 words (месть, приплесть, приобресть, выцвесть, выгнесть, процвесть). Like Сплести, except in 3 cells: infinitive, participle past_act, gerund past.
    conjugation!("месть", 8, "~есть", like Сплести;
    infinitive: "~есть";
    participle: _, "~етший", _, _;
    gerund: _, "~етя";
    ),
    // Отсвистеться: 8 words (насвистеться, рассвистеться, пересвистеться, свистеться, высвистеться, досвистеться). Like Распуститься, except in 7 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("отсвистеться", 8, "~~теться", like Распуститься;
    infinitive: "~~теться";
    past: "~~телся", "~~телась", "~~телось", "~~телись";
    participle: _, "~~тевшийся", _, _;
    gerund: _, "~~тевшись";
    ),
    // Вести: 7 words (вести, блюсти, брести, соблюсти, грясти, густи). Like Пасть, except in 5 cells: infinitive, participle past_act, participle pres_pass, participle past_pass, gerund past.
    conjugation!("вести", 7, "~сти", like Пасть;
    infinitive: "~сти";
    participle: _, "~дший", derived, "~дённый";
    gerund: _, "~дши";
    ),
    // Вынести: 7 words (вынести, вывезти, выпасти, вытрясти, повывезти, повынести). Like Лезть, except in 7 cells: infinitive, imperative sg, imperative pl, participle pres_act, participle past_pass, gerund pres, ….
    conjugation!("вынести", 7, "~ти", [aspect: Perfective], like Лезть;
    infinitive: "~ти";
    imperative: "~и", "~ите";
    participle: derived, _, _, "~енный";
    gerund: derived, "~я";
    ),
    // Нашептать: 7 words (шептать, прошептать, зашептать, нашептать, пошептать, перешептать).
    conjugation!("нашептать", 7, "~е~тать";
    infinitive: "~е~тать";
    non_past_1: "~е~чу", "~е~чем";
    non_past_2: "~е~чешь", "~е~чете";
    non_past_3: "~е~чет", "~е~чут";
    past: "~е~тал", "~е~тала", "~е~тало", "~е~тали";
    imperative: "~е~чи", "~е~чите";
    participle: "~е~чущий", "~е~тавший", _, "~ё~танный";
    gerund: "~е~ча", "~е~тав|~е~тавши";
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
    // Вымести: 7 words (выцвести, вымести, ебсти, выплести, повыцвести, мясти). Like Честь, except in 9 cells: infinitive, non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg, past m, ….
    conjugation!("вымести", 7, "~сти", like Честь;
    infinitive: "~сти";
    non_past_1: _, "~тем";
    non_past_2: "~тешь", "~тете";
    non_past_3: "~тет", _;
    past: "~л", _, _, _;
    participle: _, "~тший", _, "~тенный";
    gerund: _, "~тя";
    ),
    // Залгаться: 7 words (залгаться, изолгаться, отолгаться, оболгаться, разолгаться, налгаться). Like Стричься, except in 8 cells: infinitive, past m, past f, past n, past pl, participle pres_act, ….
    conjugation!("залгаться", 7, "~гать~я", [aspect: Perfective], like Стричься;
    infinitive: "~гать~я";
    past: "~гал~я", "~гала~ь", "~гало~ь", "~гали~ь";
    participle: derived, "~гавший~я", _, _;
    gerund: _, "~гавши~ь";
    ),
    // Насытиться: 7 words (насытиться, пресытиться, пересытиться, похититься, ухититься, сытиться). Like Встретиться, except in 1 cell: non_past_1 sg.
    conjugation!("насытиться", 7, "~тить~я", like Встретиться;
    non_past_1: "~щу~ь", _;
    ),
    // Спать: 6 words (спать, поспать, отоспать, выспать, доспать, недоспать). Like Стучать, except in 6 cells: non_past_1 sg, non_past_3 pl, participle pres_act, participle pres_pass, participle past_pass, gerund pres.
    conjugation!("спать", 6, "~ать", like Стучать;
    non_past_1: "~лю", _;
    non_past_3: _, "~ят";
    participle: "~ящий", _, derived, derived;
    gerund: "~я", _;
    ),
    // Достичь: 6 words (достичь, постичь, настичь, застичь, перестичь, пристичь).
    conjugation!("достичь", 6, "~чь", [aspect: Perfective];
    infinitive: "~чь";
    non_past_1: "~гну", "~гнем";
    non_past_2: "~гнешь", "~гнете";
    non_past_3: "~гнет", "~гнут";
    past: "~г", "~гла", "~гло", "~гли";
    imperative: "~гни", "~гните";
    participle: _, "~гший|~гнувший", _, "~гнутый";
    gerund: _, "~гнув|~гши|~гнувши";
    ),
    // Разжечь: 6 words (сжечь, поджечь, отжечь, обжечь, разжечь, изжечь).
    conjugation!("разжечь", 6, "~~ечь", [aspect: Perfective];
    infinitive: "~~ечь";
    non_past_1: "~о~гу", "~о~жём";
    non_past_2: "~о~жёшь", "~о~жёте";
    non_past_3: "~о~жёт", "~о~гут";
    past: "~~ёг", "~о~гла", "~о~гло", "~о~гли";
    imperative: "~о~ги", "~о~гите";
    participle: _, "~~ёгший", _, "~о~жённый";
    gerund: _, "~~ёгши";
    ),
    // Замереть: 6 words (запереть, умереть, помереть, замереть, примереть, позапереть). Like Затереть, except in 7 cells: past m, past f, past n, past pl, participle past_act, participle past_pass, ….
    conjugation!("замереть", 6, "~е~еть", [aspect: Perfective], like Затереть;
    past: "~е~", "~е~ла", "~е~ло", "~е~ли";
    participle: _, "~е~ший", _, "~е~тый";
    gerund: _, "~е~ев";
    ),
    // Морщиться: 6 words (таращиться, морщиться, поморщиться, сморщиться, топорщиться, упорядочиться). Like Обрушиться, except in 1 cell: imperative sg.
    conjugation!("морщиться", 6, "~ить~я", like Обрушиться;
    imperative: "~и~ь|~ь~я", _;
    ),
    // Измыслить: 6 words (злоумыслить, умыслить, замыслить, измыслить, вымыслить, обмыслить).
    conjugation!("измыслить", 6, "~с~ить", [aspect: Perfective];
    infinitive: "~с~ить";
    non_past_1: "~с~ю", "~с~им";
    non_past_2: "~с~ишь", "~с~ите";
    non_past_3: "~с~ит", "~с~ят";
    past: "~с~ил", "~с~ила", "~с~ило", "~с~или";
    imperative: "~с~и", "~с~ите";
    participle: _, "~с~ивший", _, "~ш~енный";
    gerund: _, "~с~ив|~с~ивши";
    ),
    // Обидеть: 6 words (обидеть, высидеть, изобидеть, разобидеть, забидеть, пообидеть). Like Пересидеть, except in 2 cells: imperative sg, imperative pl.
    conjugation!("обидеть", 6, "~деть", [aspect: Perfective], like Пересидеть;
    imperative: "~дь", "~дьте";
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
    // Разъехаться: 6 words (съехаться, разъехаться, проехаться, поразъехаться, подъехаться, ехаться).
    conjugation!("разъехаться", 6, "~хать~я";
    infinitive: "~хать~я";
    non_past_1: "~ду~ь", "~дем~я";
    non_past_2: "~дешь~я", "~дете~ь";
    non_past_3: "~дет~я", "~дут~я";
    past: "~хал~я", "~хала~ь", "~хало~ь", "~хали~ь";
    imperative: "~зжай~я", "~зжайте~ь";
    participle: _, "~хавший~я", _, _;
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
    // Выпасть: 6 words (выпасть, выкрасть, выпрясть, повыпасть, выкласть, повыкрасть). Like Пасть, except in 7 cells: non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg, participle pres_act, participle pres_pass, ….
    conjugation!("выпасть", 6, "~сть", [aspect: Perfective], like Пасть;
    non_past_1: _, "~дем";
    non_past_2: "~дешь", "~дете";
    non_past_3: "~дет", _;
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
    // Мочься: 6 words (разнемочься, выпрячься, выстричься, мочься, занемочься, перемочься). Like Стричься, except in 5 cells: non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg, participle pres_act.
    conjugation!("мочься", 6, "~чь~я", like Стричься;
    non_past_1: _, "~жем~я";
    non_past_2: "~жешь~я", "~жете~ь";
    non_past_3: "~жет~я", _;
    participle: derived, _, _, _;
    ),
    // Исхлестаться: 6 words (насвистаться, рассвистаться, хвостаться, расхлестаться, исхлестаться, нахлестаться). Like Плескаться, except in 7 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("исхлестаться", 6, "~~таться", like Плескаться;
    infinitive: "~~таться";
    past: "~~тался", "~~талась", "~~талось", "~~тались";
    participle: _, "~~тавшийся", _, _;
    gerund: _, "~~тавшись";
    ),
    // Взорать: 6 words (глаголать, проглаголать, обкармливать, разорать, взорать, доорать). Like Вырвать, except in 3 cells: non_past_1 sg, non_past_3 pl, participle past_pass.
    conjugation!("взорать", 6, "~ать", like Вырвать;
    non_past_1: "~ю", _;
    non_past_3: _, "~ют";
    participle: _, _, _, "~енный";
    ),
    // Висеть: 5 words (висеть, зависеть, повисеть, перевисеть, провисеть). Like Сносить, except in 9 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("висеть", 5, "~сеть", like Сносить;
    infinitive: "~сеть";
    past: "~сел", "~села", "~село", "~сели";
    participle: _, "~севший", derived, derived;
    gerund: _, "~сев|~севши";
    ),
    // Выбрать: 5 words (выбрать, выдрать, перевыбрать, повыдрать, повыбрать). Like Застлать, except in 2 cells: non_past_1 sg, non_past_3 pl.
    conjugation!("выбрать", 5, "~~ать", [aspect: Perfective], like Застлать;
    non_past_1: "~е~у", _;
    non_past_3: _, "~е~ут";
    ),
    // Отжать: 5 words (сжать, обжать, разжать, отжать, вжать). Like Гнать, except in 9 cells: non_past_1 sg, non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg, non_past_3 pl, ….
    conjugation!("отжать", 5, "~~ать", [aspect: Perfective], like Гнать;
    non_past_1: "~о~му", "~о~мём";
    non_past_2: "~о~мёшь", "~о~мёте";
    non_past_3: "~о~мёт", "~о~мут";
    imperative: "~о~ми", "~о~мите";
    participle: _, _, _, "~~атый";
    ),
    // Отстоять: 5 words (отстоять, выстоять, простоять, застоять, достоять). Like Стоять, except in 3 cells: participle pres_act, participle past_pass, gerund pres.
    conjugation!("отстоять", 5, "~ять", [aspect: Perfective], like Стоять;
    participle: derived, _, _, "~янный";
    gerund: derived, _;
    ),
    // Заспать: 5 words (проспать, переспать, заспать, наспать, приспать). Like Спать, except in 3 cells: participle pres_act, participle past_pass, gerund pres.
    conjugation!("заспать", 5, "~ать", [aspect: Perfective], like Спать;
    participle: derived, _, _, "~анный";
    gerund: derived, _;
    ),
    // Выставиться: 5 words (понадобиться, осведомиться, выставиться, занадобиться, уведомиться). Like Расправиться, except in 3 cells: imperative sg, participle pres_act, gerund pres.
    conjugation!("выставиться", 5, "~ить~я", [aspect: Perfective], like Расправиться;
    imperative: "~и~ь|~ь~я", _;
    participle: derived, _, _, _;
    gerund: derived, _;
    ),
    // Одолеть: 5 words (преодолеть, одолеть, запечатлеть, напечатлеть, отпечатлеть). Like Просмотреть, except in 9 cells: non_past_1 sg, non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg, non_past_3 pl, ….
    conjugation!("одолеть", 5, "~еть", [aspect: Perfective], like Просмотреть;
    non_past_1: "~ею", "~еем";
    non_past_2: "~еешь", "~еете";
    non_past_3: "~еет", "~еют";
    imperative: "~ей", "~ейте";
    participle: _, _, _, "~ённый";
    ),
    // Выставить: 5 words (выставить, выправить, осведомить, уведомить, предуведомить). Like Закупорить, except in 2 cells: non_past_1 sg, participle past_pass.
    conjugation!("выставить", 5, "~ить", [aspect: Perfective], like Закупорить;
    non_past_1: "~лю", _;
    participle: _, _, _, "~ленный";
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
    // Приобрестись: 5 words (приобрестись, обрестись, вплестись, выплестись, выместись). Like Сплестись, except in 1 cell: participle past_act.
    conjugation!("приобрестись", 5, "~е~тись", [aspect: Perfective], like Сплестись;
    participle: _, "~етший~я", _, _;
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
    // Вылиться: 5 words (выбиться, вылиться, выпиться, вывиться, повыбиться). Like Бриться, except in 8 cells: non_past_1 sg, non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg, non_past_3 pl, ….
    conjugation!("вылиться", 5, "~ить~я", [aspect: Perfective], like Бриться;
    non_past_1: "~ью~ь", "~ьем~я";
    non_past_2: "~ьешь~я", "~ейте~ь";
    non_past_3: "~ьет~я", "~ьют~я";
    participle: derived, _, _, _;
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
    // Вылезти: 5 words (вылезти, выползти, повылезти, повыползти, выебти). Like Вынести, except in 1 cell: gerund past.
    conjugation!("вылезти", 5, "~ти", [aspect: Perfective], like Вынести;
    gerund: _, "~ши";
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
    // Изобрести: 4 words (изобрести, приобрести, обрести, соплести). Like Сплести, except in 2 cells: participle past_act, gerund past.
    conjugation!("изобрести", 4, "~ести", [aspect: Perfective], like Сплести;
    participle: _, "~етший", _, _;
    gerund: _, "~етя|~етши";
    ),
    // Принять: 4 words (принять, воспринять, предпринять, напринять). Like Занять, except in 8 cells: non_past_1 sg, non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg, non_past_3 pl, ….
    conjugation!("принять", 4, "~нять", [aspect: Perfective], like Занять;
    non_past_1: "~му", "~мем";
    non_past_2: "~мешь", "~мете";
    non_past_3: "~мет", "~мут";
    imperative: "~ми", "~мите";
    ),
    // Услышать: 4 words (услышать, вымолчать, выбрюзжать, дослышать). Like Заспать, except in 4 cells: non_past_1 sg, non_past_3 pl, imperative sg, imperative pl.
    conjugation!("услышать", 4, "~ать", [aspect: Perfective], like Заспать;
    non_past_1: "~у", _;
    non_past_3: _, "~ат";
    imperative: "~ь", "~ьте";
    ),
    // Выпереть: 4 words (вымереть, вытереть, выпереть, повымереть). Like Замереть, except in 5 cells: non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg, gerund past.
    conjugation!("выпереть", 4, "~е~еть", [aspect: Perfective], like Замереть;
    non_past_1: _, "~~ем";
    non_past_2: "~~ешь", "~~ете";
    non_past_3: "~~ет", _;
    gerund: _, "~е~ев|~е~ши";
    ),
    // Высечь: 4 words (высечь, вытечь, выволочь, выпечь). Like Сволочь, except in 6 cells: non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg, participle pres_act, participle past_pass.
    conjugation!("высечь", 4, "~чь", [aspect: Perfective], like Сволочь;
    non_past_1: _, "~чем";
    non_past_2: "~чешь", "~чете";
    non_past_3: "~чет", _;
    participle: derived, _, _, "~ченный";
    ),
    // Напрячь: 4 words (запрячь, напрячь, распрячь, впрячь).
    conjugation!("напрячь", 4, "~ячь", [aspect: Perfective];
    infinitive: "~ячь";
    non_past_1: "~ягу", "~яжём";
    non_past_2: "~яжёшь", "~яжёте";
    non_past_3: "~яжёт", "~ягут";
    past: "~ёг|~яг", "~ягла", "~ягло", "~ягли";
    imperative: "~яги", "~ягите";
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
    // Бечь: 4 words (бечь, убечь, избечь, побечь). Like Сберечь, except in 6 cells: non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg, participle pres_pass, participle past_pass.
    conjugation!("бечь", 4, "~ечь", like Сберечь;
    non_past_1: _, "~егём";
    non_past_2: "~егёшь", "~егёте";
    non_past_3: "~егёт", _;
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
    // Вынестись: 4 words (вынестись, вывезтись, вытрястись, отверзтись). Like Трястись, except in 7 cells: non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg, participle pres_act, gerund pres, ….
    conjugation!("вынестись", 4, "~ти~ь", [aspect: Perfective], like Трястись;
    non_past_1: _, "~ем~я";
    non_past_2: "~ешь~я", "~ете~ь";
    non_past_3: "~ет~я", _;
    participle: derived, _, _, _;
    gerund: derived, "~я~ь";
    ),
    // Истереть: 4 words (растереть, распереть, истереть, поистереть).
    conjugation!("истереть", 4, "~с~е~еть", [aspect: Perfective];
    infinitive: "~с~е~еть";
    non_past_1: "~зо~~у", "~зо~~ём";
    non_past_2: "~зо~~ёшь", "~зо~~ёте";
    non_past_3: "~зо~~ёт", "~зо~~ут";
    past: "~с~ё~", "~с~ё~ла", "~с~ё~ло", "~с~ё~ли";
    imperative: "~зо~~и", "~зо~~ите";
    participle: _, "~с~ё~ший", _, "~с~ё~тый";
    gerund: _, "~с~е~ев|~с~ё~ши";
    ),
    // Испить: 4 words (расшить, испить, распить, исшить). Like Измыслить, except in 9 cells: non_past_1 sg, non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg, non_past_3 pl, ….
    conjugation!("испить", 4, "~с~ить", [aspect: Perfective], like Измыслить;
    non_past_1: "~зо~ью", "~зо~ьём";
    non_past_2: "~зо~ьёшь", "~зо~ьёте";
    non_past_3: "~зо~ьёт", "~зо~ьют";
    imperative: "~с~ей", "~с~ейте";
    participle: _, _, _, "~с~итый";
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
    // Поколебать: 4 words (колебать, поколебать, зыбать, выщипать). Like Щипать, except in 2 cells: participle pres_pass, participle past_pass.
    conjugation!("поколебать", 4, "~ать", like Щипать;
    participle: _, _, "~лемый", "~ленный";
    ),
    // Насочинять: 4 words (насочинять, обрусять, постеснять, оязыковлять). Like Выровнять, except in 4 cells: participle pres_act, participle pres_pass, participle past_pass, gerund pres.
    conjugation!("насочинять", 4, "~ять", like Выровнять;
    participle: "~яющий", _, "~яемый", "~ённый";
    gerund: "~яя", _;
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
    // Помочь: 3 words (выстричь, выпрячь, вылечь). Like Постричь, except in 5 cells: non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg, participle pres_act.
    conjugation!("помочь", 3, "~чь", [aspect: Perfective], like Постричь;
    non_past_1: _, "~жем";
    non_past_2: "~жешь", "~жете";
    non_past_3: "~жет", _;
    participle: derived, _, _, _;
    ),
    // Хотеться: 3 words (хотеться, захотеться, расхотеться). Like Разлететься, except in 4 cells: non_past_2 sg, non_past_3 sg, participle pres_act, gerund pres.
    conjugation!("хотеться", 3, "~теть~я", like Разлететься;
    non_past_2: "~чешь~я", _;
    non_past_3: "~чет~я", _;
    participle: derived, _, _, _;
    gerund: derived, _;
    ),
    // Вырасти: 3 words (вырасти, повырасти, подвырасти). Like Дорасти, except in 6 cells: non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg, participle pres_act, gerund pres.
    conjugation!("вырасти", 3, "~а~ти", [aspect: Perfective], like Дорасти;
    non_past_1: _, "~а~тем";
    non_past_2: "~а~тешь", "~а~тете";
    non_past_3: "~а~тет", _;
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
    // Тереть: 3 words (тереть, мереть, переть). Like Затереть, except in 2 cells: participle pres_act, gerund past.
    conjugation!("тереть", 3, "~е~еть", [aspect: Imperfective], like Затереть;
    participle: "~~ущий", _, _, _;
    gerund: _, "~ё~ши";
    ),
    // Опоясать: 3 words (опоясать, запоясать, распоясать). Like Выписать, except in 4 cells: imperative sg, imperative pl, participle pres_act, gerund pres.
    conjugation!("опоясать", 3, "~сать", [aspect: Perfective], like Выписать;
    imperative: "~шь", "~шьте";
    participle: derived, _, _, _;
    gerund: derived, _;
    ),
    // Вянуть: 3 words (вянуть, завянуть, увянуть). Like Стынуть, except in 2 cells: participle past_act, gerund past.
    conjugation!("вянуть", 3, "~нуть", like Стынуть;
    participle: _, "~нувший", _, _;
    gerund: _, "~нув|~нувши";
    ),
    // Заклясть: 3 words (проклясть, клясть, заклясть). Like Помять, except in 9 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("заклясть", 3, "~сть", like Помять;
    infinitive: "~сть";
    past: "~л", "~ла", "~ло", "~ли";
    participle: _, "~вший", "~нимый", "~тый";
    gerund: _, "~в|~вши";
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
    // Выбраться: 3 words (выбраться, выдраться, повыбраться). Like Стлаться, except in 4 cells: non_past_1 sg, non_past_3 pl, participle pres_act, gerund pres.
    conjugation!("выбраться", 3, "~~ать~я", [aspect: Perfective], like Стлаться;
    non_past_1: "~е~у~ь", _;
    non_past_3: _, "~е~ут~я";
    participle: derived, _, _, _;
    gerund: derived, _;
    ),
    // Счесть: 3 words (счесть, обчесть, отчесть).
    conjugation!("счесть", 3, "~~есть", [aspect: Perfective];
    infinitive: "~~есть";
    non_past_1: "~о~ту", "~о~тём";
    non_past_2: "~о~тёшь", "~о~тёте";
    non_past_3: "~о~тёт", "~о~тут";
    past: "~~ёл", "~о~ла", "~о~ло", "~о~ли";
    imperative: "~о~ти", "~о~тите";
    participle: _, _, _, "~о~тённый";
    gerund: _, "~о~тя|~~ётши";
    ),
    // Обмереть: 3 words (отпереть, обмереть, отмереть). Like Стереть, except in 7 cells: past m, past f, past n, past pl, participle past_act, participle past_pass, ….
    conjugation!("обмереть", 3, "~~е~еть", [aspect: Perfective], like Стереть;
    past: "~~е~", "~~е~ла", "~~е~ло", "~~е~ли";
    participle: _, "~~е~ший|~~е~ший", _, "~~е~тый";
    gerund: _, "~~е~ев|~~е~ши|~~е~ши";
    ),
    // Понудить: 3 words (принудить, понудить, попринудить). Like Высвободить, except in 3 cells: imperative sg, imperative pl, participle past_pass.
    conjugation!("понудить", 3, "~дить", [aspect: Perfective], like Высвободить;
    imperative: "~дь", "~дьте";
    participle: _, _, _, "~ждённый";
    ),
    // Отозвать: 3 words (отозвать, обозвать, подозвать). Like Разогнать, except in 6 cells: non_past_1 sg, non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg, non_past_3 pl.
    conjugation!("отозвать", 3, "~о~~ать", [aspect: Perfective], like Разогнать;
    non_past_1: "~~о~у", "~~о~ём";
    non_past_2: "~~о~ёшь", "~~о~ёте";
    non_past_3: "~~о~ёт", "~~о~ут";
    ),
    // Помутить: 3 words (замутить, помутить, намутить). Like Зачертить, except in 1 cell: participle past_pass.
    conjugation!("помутить", 3, "~тить", [aspect: Perfective], like Зачертить;
    participle: _, _, _, "~тнённый";
    ),
    // Обглодать: 3 words (глодать, обглодать, сглодать).
    conjugation!("обглодать", 3, "~дать";
    infinitive: "~дать";
    non_past_1: "~жу|~даю", "~жем|~даем";
    non_past_2: "~жешь|~даешь", "~жете|~даете";
    non_past_3: "~жет|~дает", "~жут|~дают";
    past: "~дал", "~дала", "~дало", "~дали";
    imperative: "~жи|~дай", "~жите|~дайте";
    participle: "~жущий", "~давший", "~даемый", "~данный";
    gerund: "~жа", "~дав|~давши";
    ),
    // Пролежать: 3 words (пролежать, отлежать, облежать). Like Взъебать, except in 8 cells: non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg, non_past_3 pl, participle pres_act, ….
    conjugation!("пролежать", 3, "~е~ать", like Взъебать;
    non_past_1: _, "~е~им";
    non_past_2: "~е~ишь", "~е~ите";
    non_past_3: "~е~ит", "~е~ат";
    participle: "~е~ащий", _, "~е~имый", _;
    gerund: "~е~а", _;
    ),
    // Осечься: 3 words (пересечься, осечься, сечься). Like Влечься, except in 3 cells: past m, participle past_act, gerund past.
    conjugation!("осечься", 3, "~ечь~я", like Влечься;
    past: "~ёк~я|~ек~я", _, _, _;
    participle: _, "~екший~я|~ёкший~я", _, _;
    gerund: _, "~екши~ь|~ёкши~ь";
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
    // Взбресть: 3 words (взбресть, добресть, набресть). Like Развести, except in 3 cells: infinitive, participle past_pass, gerund past.
    conjugation!("взбресть", 3, "~есть", [aspect: Perfective], like Развести;
    infinitive: "~есть";
    participle: _, _, _, derived;
    gerund: _, "~едя";
    ),
    // Выпечься: 3 words (выволочься, выпечься, высечься). Like Заволочься, except in 5 cells: non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg, participle pres_act.
    conjugation!("выпечься", 3, "~чь~я", [aspect: Perfective], like Заволочься;
    non_past_1: _, "~чем~я";
    non_past_2: "~чешь~я", "~чете~ь";
    non_past_3: "~чет~я", _;
    participle: derived, _, _, _;
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
    infinitive: "~ать";
    non_past_1: "~аю", "~аем";
    non_past_2: "~аешь", "~аете";
    non_past_3: "~ает", "~ают";
    past: "~ал", "~ала", "~ало", "~али";
    imperative: "~ай", "~айте";
    participle: "~ающий", "~авший", "~аемый", "~ённый";
    gerund: "~ая", "~ав|~авши";
    ),
    // Позаводить: 3 words (самовоспроизводить, напереводить, позаводить).
    conjugation!("позаводить", 3, "~одить", [aspect: Perfective];
    infinitive: "~одить";
    non_past_1: "~ожу", "~одим";
    non_past_2: "~одишь", "~одите";
    non_past_3: "~одит", "~одят";
    past: "~одил", "~одила", "~одило", "~одили";
    imperative: "~оди", "~одите";
    participle: _, "~одивший", _, "~едённый";
    gerund: _, "~одив|~одивши";
    ),
    // Закодить: 3 words (повыводить, закодить, накодить). Like Высвободить, except in 1 cell: participle past_pass.
    conjugation!("закодить", 3, "~дить", [aspect: Perfective], like Высвободить;
    participle: _, _, _, "~денный";
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
