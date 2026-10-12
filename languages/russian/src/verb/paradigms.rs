//! The conjugations, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one conjugation: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (`[gender: Neuter]`, if
/// they share them), the conjugation it is like (if any), then a row of patterns
/// per line (`derived`: the form comes from another cell's, see
/// `relations.rs`; `_`: the form is the conjugation's it is like):
/// `infinitive`
/// `non_past_first` (sg, pl)
/// `non_past_second` (sg, pl)
/// `non_past_third` (sg, pl)
/// `past` (m, f, n, pl)
/// `imperative` (sg, pl)
/// `participle` (pres_act, past_act, pres_pass, past_pass)
/// `gerund` (pres, past)
macro_rules! conjugation {
    ($name:literal, $words:literal, $lemma:literal $(, [$($f:ident: $v:ident),*])? $(, like $parent:ident)?;
     infinitive: $c0:tt;
     non_past_first: $c1:tt, $c2:tt;
     non_past_second: $c3:tt, $c4:tt;
     non_past_third: $c5:tt, $c6:tt;
     past: $c7:tt, $c8:tt, $c9:tt, $c10:tt;
     imperative: $c11:tt, $c12:tt;
     participle: $c13:tt, $c14:tt, $c15:tt, $c16:tt;
     gerund: $c17:tt, $c18:tt;
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
                flexion::cell!($c14), "\t",
                flexion::cell!($c15), "\t",
                flexion::cell!($c16), "\t",
                flexion::cell!($c17), "\t",
                flexion::cell!($c18), "\t",
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
    /// Like `расправиться`: 296 words.
    Расправиться,
    /// Like `хватить`: 296 words.
    Хватить,
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
    /// Like `засосать`: 118 words.
    Засосать,
    /// Like `оказать`: 118 words.
    Оказать,
    /// Like `топтать`: 116 words.
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
    /// Like `зарядить`: 79 words.
    Зарядить,
    /// Like `стать`: 78 words.
    Стать,
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
    /// Like `виться`: 57 words.
    Виться,
    /// Like `перевернуть`: 57 words.
    Перевернуть,
    /// Like `сунуться`: 57 words.
    Сунуться,
    /// Like `раскрыться`: 55 words.
    Раскрыться,
    /// Like `колоть`: 55 words.
    Колоть,
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
    /// Like `трепаться`: 34 words.
    Трепаться,
    /// Like `сплести`: 34 words.
    Сплести,
    /// Like `расчесать`: 33 words.
    Расчесать,
    /// Like `станцевать`: 33 words.
    Станцевать,
    /// Like `щипать`: 33 words.
    Щипать,
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
    /// Like `перепеть`: 20 words.
    Перепеть,
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
    /// Like `потчевать`: 18 words.
    Потчевать,
    /// Like `въесться`: 18 words.
    Въесться,
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
    /// Like `постричь`: 15 words.
    Постричь,
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
    /// Like `сплестись`: 13 words.
    Сплестись,
    /// Like `тыкаться`: 13 words.
    Тыкаться,
    /// Like `мяться`: 13 words.
    Мяться,
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
    /// Like `поехать`: 10 words.
    Поехать,
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
    /// Like `засыпать`: 2 words.
    Засыпать,
    /// Like `сопрячь`: 2 words.
    Сопрячь,
    /// Like `реветь`: 2 words.
    Реветь,
    /// Like `лгать`: 2 words.
    Лгать,
    /// Like `ехать`: 2 words.
    Ехать,
    /// Like `резать`: 1 words.
    Резать,
    /// Like `застыть`: 1 words.
    Застыть,
    /// Like `упрятать`: 1 words.
    Упрятать,
    /// Like `рещи`: 1 words.
    Рещи,
}

impl Conjugation {
    /// Every conjugation, in order.
    pub const ALL: [Conjugation; 342] = [
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
        Conjugation::Расправиться,
        Conjugation::Хватить,
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
        Conjugation::Засосать,
        Conjugation::Оказать,
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
        Conjugation::Зарядить,
        Conjugation::Стать,
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
        Conjugation::Виться,
        Conjugation::Перевернуть,
        Conjugation::Сунуться,
        Conjugation::Раскрыться,
        Conjugation::Колоть,
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
        Conjugation::Трепаться,
        Conjugation::Сплести,
        Conjugation::Расчесать,
        Conjugation::Станцевать,
        Conjugation::Щипать,
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
        Conjugation::Отдаться,
        Conjugation::Дорасти,
        Conjugation::Забыть,
        Conjugation::Унестись,
        Conjugation::Набежать,
        Conjugation::Расстаться,
        Conjugation::Грызть,
        Conjugation::Лезть,
        Conjugation::Зашибить,
        Conjugation::Перепеть,
        Conjugation::Занять,
        Conjugation::Лечь,
        Conjugation::Плескаться,
        Conjugation::Предназначить,
        Conjugation::Просмотреть,
        Conjugation::Потчевать,
        Conjugation::Въесться,
        Conjugation::Устояться,
        Conjugation::Трепетать,
        Conjugation::Жать,
        Conjugation::Вгрызться,
        Conjugation::Взъебать,
        Conjugation::Колыхаться,
        Conjugation::Разобрать,
        Conjugation::Выровнять,
        Conjugation::Трепать,
        Conjugation::Заволочься,
        Conjugation::Честь,
        Conjugation::Насесть,
        Conjugation::Смеяться,
        Conjugation::Гнать,
        Conjugation::Постричь,
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
        Conjugation::Сплестись,
        Conjugation::Тыкаться,
        Conjugation::Мяться,
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
        Conjugation::Дожить,
        Conjugation::Ушибиться,
        Conjugation::Зачесться,
        Conjugation::Жаться,
        Conjugation::Пакоститься,
        Conjugation::Зассать,
        Conjugation::Поехать,
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
        Conjugation::Засыпать,
        Conjugation::Сопрячь,
        Conjugation::Реветь,
        Conjugation::Лгать,
        Conjugation::Ехать,
        Conjugation::Резать,
        Conjugation::Застыть,
        Conjugation::Упрятать,
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

pub(super) static CONJUGATIONS: [Paradigm<'static>; 342] = [
    // Играть: 11,635 words (знать, играть, презирать, делать, уступать, скучать).
    conjugation!("играть", 11635, "~ть";
        infinitive: "~ть";
        non_past_first: "~ю", "~ем";
        non_past_second: "~ешь", "~ете";
        non_past_third: "~ет", "~ют";
        past: "~л", "~ла", "~ло", "~ли";
        imperative: "~й", "~йте";
        participle: "~ющий", "~вший", "~емый", "~нный";
        gerund: "~я", "~в|~вши";
    ),
    // Набираться: 8,293 words (отпочковываться, изъясняться, облокачиваться, кончаться, являться, сморкаться).
    conjugation!("набираться", 8293, "~ть~я";
        infinitive: "~ть~я";
        non_past_first: "~ю~ь", "~ем~я";
        non_past_second: "~ешь~я", "~ете~ь";
        non_past_third: "~ет~я", "~ют~я";
        past: "~л~я", "~ла~ь", "~ло~ь", "~ли~ь";
        imperative: "~й~я", "~йте~ь";
        participle: "~ющий~я", "~вший~я", derived, derived;
        gerund: "~я~ь", "~вши~ь";
    ),
    // Вибрировать: 4,162 words (мульчировать, завибрировать, координировать, использовать, чувствовать, почувствовать).
    conjugation!("вибрировать", 4162, "~овать";
        infinitive: "~овать";
        non_past_first: "~ую", "~уем";
        non_past_second: "~уешь", "~уете";
        non_past_third: "~ует", "~уют";
        past: "~овал", "~овала", "~овало", "~овали";
        imperative: "~уй", "~уйте";
        participle: "~ующий", "~овавший", "~уемый", "~ованный";
        gerund: "~уя", "~овав|~овавши";
    ),
    // Стыковаться: 2,209 words (тусоваться, использоваться, чувствоваться, почувствоваться, целоваться, абсорбироваться).
    conjugation!("стыковаться", 2209, "~овать~я";
        infinitive: "~овать~я";
        non_past_first: "~ую~ь", "~уем~я";
        non_past_second: "~уешь~я", "~уете~ь";
        non_past_third: "~ует~я", "~уют~я";
        past: "~овал~я", "~овала~ь", "~овало~ь", "~овали~ь";
        imperative: "~уй~я", "~уйте~ь";
        participle: "~ующий~я", "~овавший~я", derived, derived;
        gerund: "~уя~ь", "~овавши~ь";
    ),
    // Ударить: 1,983 words (жалить, ужалить, помнить, увеличить, запомнить, значить).
    conjugation!("ударить", 1983, "~ить";
        infinitive: "~ить";
        non_past_first: "~ю", "~им";
        non_past_second: "~ишь", "~ите";
        non_past_third: "~ит", "~ят";
        past: "~ил", "~ила", "~ило", "~или";
        imperative: "~ь", "~ьте";
        participle: "~ящий", "~ивший", "~имый", "~енный";
        gerund: "~я", "~ив|~ивши";
    ),
    // Палить: 1,318 words (говорить, поговорить, изменить, благодарить, винить, отменить). Like Ударить, except in 3 cells: imperative sg, imperative pl, participle past_pass.
    conjugation!("палить", 1318, "~ить", like Ударить;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: "~и", "~ите";
        participle: _, _, _, "~ённый";
        gerund: _, _;
    ),
    // Числиться: 1,171 words (изъясниться, соединиться, присоединиться, объясниться, валиться, приземлиться).
    conjugation!("числиться", 1171, "~ить~я";
        infinitive: "~ить~я";
        non_past_first: "~ю~ь", "~им~я";
        non_past_second: "~ишь~я", "~ите~ь";
        non_past_third: "~ит~я", "~ят~я";
        past: "~ил~я", "~ила~ь", "~ило~ь", "~или~ь";
        imperative: "~и~ь", "~ите~ь";
        participle: "~ящий~я", "~ивший~я", derived, derived;
        gerund: "~я~ь", "~ивши~ь";
    ),
    // Раздуть: 928 words (деревенеть, взять, одеревенеть, деревянеть, болеть, краснеть). Like Играть, except in 1 cell: participle past_pass.
    conjugation!("раздуть", 928, "~ть", like Играть;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: _, _;
        participle: _, _, _, "~тый";
        gerund: _, _;
    ),
    // Рвануть: 685 words (взбзднуть, хохотнуть, блевануть, пугнуть, гнуть, ткнуть).
    conjugation!("рвануть", 685, "~уть", [aspect: Perfective];
        infinitive: "~уть";
        non_past_first: "~у", "~ём";
        non_past_second: "~ёшь", "~ёте";
        non_past_third: "~ёт", "~ут";
        past: "~ул", "~ула", "~уло", "~ули";
        imperative: "~и", "~ите";
        participle: "~ущий", "~увший", derived, "~утый";
        gerund: derived, "~ув|~увши";
    ),
    // Выделить: 616 words (учить, курить, покурить, кончить, закончить, варить). Like Палить, except in 1 cell: participle past_pass.
    conjugation!("выделить", 616, "~ить", like Палить;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: _, _;
        participle: _, _, _, "~енный";
        gerund: _, _;
    ),
    // Усилиться: 566 words (ссориться, пялиться, ускориться, стариться, состариться, целиться). Like Числиться, except in 3 cells: imperative sg, imperative pl, gerund past.
    conjugation!("усилиться", 566, "~ить~я", like Числиться;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: "~ь~я", "~ьте~ь";
        participle: _, _, _, _;
        gerund: _, "~ивши~ь|~я~ь";
    ),
    // Тянуть: 543 words (выковырнуть, прыгнуть, пёрнуть, капнуть, шлёпнуть, взглянуть). Like Рвануть, except in 4 cells: non_past_first pl, non_past_second sg, non_past_second pl, non_past_third sg.
    conjugation!("тянуть", 543, "~уть", [aspect: Perfective], like Рвануть;
        infinitive: _;
        non_past_first: _, "~ем";
        non_past_second: "~ешь", "~ете";
        non_past_third: "~ет", _;
        past: _, _, _, _;
        imperative: _, _;
        participle: _, _, _, _;
        gerund: _, _;
    ),
    // Обнажиться: 501 words (учиться, кончиться, страшиться, случиться, научиться, уменьшиться). Like Числиться, except in 4 cells: non_past_first sg, non_past_third pl, participle pres_act, gerund pres.
    conjugation!("обнажиться", 501, "~ить~я", like Числиться;
        infinitive: _;
        non_past_first: "~у~ь", _;
        non_past_second: _, _;
        non_past_third: _, "~ат~я";
        past: _, _, _, _;
        imperative: _, _;
        participle: "~ащий~я", _, _, _;
        gerund: "~а~ь", _;
    ),
    // Любиться: 454 words (появиться, торопиться, стремиться, влюбиться, удивиться, остановиться). Like Числиться, except in 1 cell: non_past_first sg.
    conjugation!("любиться", 454, "~ить~я", like Числиться;
        infinitive: _;
        non_past_first: "~лю~ь", _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: _, _;
        participle: _, _, _, _;
        gerund: _, _;
    ),
    // Обернуться: 392 words (рехнуться, повернуться, проснуться, заткнуться, вернуться, улыбнуться).
    conjugation!("обернуться", 392, "~уть~я", [aspect: Perfective];
        infinitive: "~уть~я";
        non_past_first: "~у~ь", "~ём~я";
        non_past_second: "~ёшь~я", "~ёте~ь";
        non_past_third: "~ёт~я", "~ут~я";
        past: "~ул~я", "~ула~ь", "~уло~ь", "~ули~ь";
        imperative: "~и~ь", "~ите~ь";
        participle: "~ущий~я", "~увший~я", derived, derived;
        gerund: derived, "~увши~ь";
    ),
    // Обрушиться: 343 words (увеличиться, размножиться, кочевряжиться, дурачиться, спешиться, мучиться). Like Обнажиться, except in 2 cells: imperative sg, imperative pl.
    conjugation!("обрушиться", 343, "~ить~я", like Обнажиться;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: "~ь~я", "~ьте~ь";
        participle: _, _, _, _;
        gerund: _, _;
    ),
    // Восстановить: 336 words (уступить, любить, полюбить, возлюбить, купить, заломить). Like Палить, except in 2 cells: non_past_first sg, participle past_pass.
    conjugation!("восстановить", 336, "~ить", like Палить;
        infinitive: _;
        non_past_first: "~лю", _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: _, _;
        participle: _, _, _, "~ленный";
        gerund: _, _;
    ),
    // Разводить: 327 words (происходить, приходить, ходить, проводить, находить, ездить).
    conjugation!("разводить", 327, "~дить";
        infinitive: "~дить";
        non_past_first: "~жу", "~дим";
        non_past_second: "~дишь", "~дите";
        non_past_third: "~дит", "~дят";
        past: "~дил", "~дила", "~дило", "~дили";
        imperative: "~ди", "~дите";
        participle: "~дящий", "~дивший", "~димый", "~женный";
        gerund: "~дя", "~див|~дивши";
    ),
    // Долбить: 317 words (вступить, наступить, дымить, удивить, бомбить, выступить). Like Восстановить, except in 1 cell: participle past_pass.
    conjugation!("долбить", 317, "~ить", like Восстановить;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: _, _;
        participle: _, _, _, "~лённый";
        gerund: _, _;
    ),
    // Расправиться: 296 words (нравиться, понравиться, готовиться, направиться, знакомиться, познакомиться). Like Усилиться, except in 2 cells: non_past_first sg, gerund past.
    conjugation!("расправиться", 296, "~ить~я", like Усилиться;
        infinitive: _;
        non_past_first: "~лю~ь", _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: _, _;
        participle: _, _, _, _;
        gerund: _, "~ивши~ь";
    ),
    // Хватить: 296 words (светить, кутить, крутить, платить, шутить, пошутить).
    conjugation!("хватить", 296, "~тить";
        infinitive: "~тить";
        non_past_first: "~чу", "~тим";
        non_past_second: "~тишь", "~тите";
        non_past_third: "~тит", "~тят";
        past: "~тил", "~тила", "~тило", "~тили";
        imperative: "~ти", "~тите";
        participle: "~тящий", "~тивший", "~тимый", "~ченный";
        gerund: "~тя", "~тив|~тивши";
    ),
    // Представить: 290 words (забавить, восславить, готовить, грабить, заставить, править). Like Ударить, except in 2 cells: non_past_first sg, participle past_pass.
    conjugation!("представить", 290, "~ить", like Ударить;
        infinitive: _;
        non_past_first: "~лю", _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: _, _;
        participle: _, _, _, "~ленный";
        gerund: _, _;
    ),
    // Расходиться: 283 words (находиться, трудиться, садиться, насладиться, разводиться, родиться).
    conjugation!("расходиться", 283, "~дить~я";
        infinitive: "~дить~я";
        non_past_first: "~жу~ь", "~дим~я";
        non_past_second: "~дишь~я", "~дите~ь";
        non_past_third: "~дит~я", "~дят~я";
        past: "~дил~я", "~дила~ь", "~дило~ь", "~дили~ь";
        imperative: "~ди~ь", "~дите~ь";
        participle: "~дящий~я", "~дивший~я", derived, derived;
        gerund: "~дя~ь", "~дивши~ь";
    ),
    // Закатиться: 232 words (облокотиться, расплатиться, катиться, портиться, суетиться, светиться).
    conjugation!("закатиться", 232, "~тить~я";
        infinitive: "~тить~я";
        non_past_first: "~чу~ь", "~тим~я";
        non_past_second: "~тишь~я", "~тите~ь";
        non_past_third: "~тит~я", "~тят~я";
        past: "~тил~я", "~тила~ь", "~тило~ь", "~тили~ь";
        imperative: "~ти~ь", "~тите~ь";
        participle: "~тящий~я", "~тивший~я", derived, derived;
        gerund: "~тя~ь", "~тивши~ь";
    ),
    // Тянуться: 230 words (грохнуться, чокнуться, оглянуться, тянуться, потянуться, откликнуться). Like Обернуться, except in 4 cells: non_past_first pl, non_past_second sg, non_past_second pl, non_past_third sg.
    conjugation!("тянуться", 230, "~уть~я", [aspect: Perfective], like Обернуться;
        infinitive: _;
        non_past_first: _, "~ем~я";
        non_past_second: "~ешь~я", "~ете~ь";
        non_past_third: "~ет~я", _;
        past: _, _, _, _;
        imperative: _, _;
        participle: _, _, _, _;
        gerund: _, _;
    ),
    // Залипнуть: 203 words (распухнуть, сдохнуть, замёрзнуть, высохнуть, погибнуть, окрепнуть).
    conjugation!("залипнуть", 203, "~нуть", [aspect: Perfective];
        infinitive: "~нуть";
        non_past_first: "~ну", "~нем";
        non_past_second: "~нешь", "~нете";
        non_past_third: "~нет", "~нут";
        past: "~", "~ла", "~ло", "~ли";
        imperative: "~ни", "~ните";
        participle: "~нущий", "~ший", derived, derived;
        gerund: derived, "~ши";
    ),
    // Стучать: 192 words (держать, кричать, жужжать, мычать, принадлежать, лежать).
    conjugation!("стучать", 192, "~ать";
        infinitive: "~ать";
        non_past_first: "~у", "~им";
        non_past_second: "~ишь", "~ите";
        non_past_third: "~ит", "~ат";
        past: "~ал", "~ала", "~ало", "~али";
        imperative: "~и", "~ите";
        participle: "~ащий", "~авший", "~имый", "~анный";
        gerund: "~а", "~ав|~авши";
    ),
    // Защитить: 191 words (запретить, прекратить, крестить, простить, нарастить, посетить). Like Хватить, except in 2 cells: non_past_first sg, participle past_pass.
    conjugation!("защитить", 191, "~тить", like Хватить;
        infinitive: _;
        non_past_first: "~щу", _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: _, _;
        participle: _, _, _, "~щённый";
        gerund: _, _;
    ),
    // Сносить: 186 words (носить, спросить, приносить, гласить, переносить, косить).
    conjugation!("сносить", 186, "~сить";
        infinitive: "~сить";
        non_past_first: "~шу", "~сим";
        non_past_second: "~сишь", "~сите";
        non_past_third: "~сит", "~сят";
        past: "~сил", "~сила", "~сило", "~сили";
        imperative: "~си", "~сите";
        participle: "~сящий", "~сивший", "~симый", "~шенный";
        gerund: "~ся", "~сив|~сивши";
    ),
    // Отметить: 165 words (ответить, встретить, заметить, утратить, тратить, похитить). Like Хватить, except in 2 cells: imperative sg, imperative pl.
    conjugation!("отметить", 165, "~тить", like Хватить;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: "~ть", "~тьте";
        participle: _, _, _, _;
        gerund: _, _;
    ),
    // Распуститься: 163 words (креститься, спуститься, сгуститься, опуститься, поместиться, проститься).
    conjugation!("распуститься", 163, "~~титься";
        infinitive: "~~титься";
        non_past_first: "~щу~ь", "~~тимся";
        non_past_second: "~~тишься", "~~титесь";
        non_past_third: "~~тится", "~~тятся";
        past: "~~тился", "~~тилась", "~~тилось", "~~тились";
        imperative: "~~тись", "~~титесь";
        participle: "~~тящийся", "~~тившийся", derived, derived;
        gerund: "~~тясь", "~~тившись";
    ),
    // Носиться: 138 words (соотноситься, согласиться, относиться, напроситься, произноситься, доноситься).
    conjugation!("носиться", 138, "~~иться";
        infinitive: "~~иться";
        non_past_first: "~шу~ь", "~~имся";
        non_past_second: "~~ишься", "~~итесь";
        non_past_third: "~~ится", "~~ятся";
        past: "~~ился", "~~илась", "~~илось", "~~ились";
        imperative: "~~ись", "~~итесь";
        participle: "~~ящийся", "~~ившийся", derived, derived;
        gerund: "~~ясь", "~~ившись";
    ),
    // Забросить: 125 words (красить, бросить, сбросить, украсить, повесить, весить). Like Сносить, except in 2 cells: imperative sg, imperative pl.
    conjugation!("забросить", 125, "~сить", like Сносить;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: "~сь", "~сьте";
        participle: _, _, _, _;
        gerund: _, _;
    ),
    // Встретиться: 120 words (охотиться, заботиться, встретиться, пятиться, отметиться, позаботиться). Like Закатиться, except in 2 cells: imperative sg, imperative pl.
    conjugation!("встретиться", 120, "~тить~я", like Закатиться;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: "~ть~я", "~тьте~ь";
        participle: _, _, _, _;
        gerund: _, _;
    ),
    // Засосать: 118 words (врать, ебать, ржать, заржать, соврать, рвать). Like Стучать, except in 8 cells: non_past_first pl, non_past_second sg, non_past_second pl, non_past_third sg, non_past_third pl, participle pres_act, ….
    conjugation!("засосать", 118, "~ать", like Стучать;
        infinitive: _;
        non_past_first: _, "~ём";
        non_past_second: "~ёшь", "~ёте";
        non_past_third: "~ёт", "~ут";
        past: _, _, _, _;
        imperative: _, _;
        participle: "~ущий", _, derived, _;
        gerund: "~я", _;
    ),
    // Оказать: 118 words (сказать, рассказать, промазать, показать, связать, резать).
    conjugation!("оказать", 118, "~зать";
        infinitive: "~зать";
        non_past_first: "~жу", "~жем";
        non_past_second: "~жешь", "~жете";
        non_past_third: "~жет", "~жут";
        past: "~зал", "~зала", "~зало", "~зали";
        imperative: "~жи", "~жите";
        participle: "~жущий", "~завший", derived, "~занный";
        gerund: "~жа", "~зав|~завши";
    ),
    // Топтать: 116 words (хохотать, захохотать, прятать, спрятать, щекотать, лепетать).
    conjugation!("топтать", 116, "~тать";
        infinitive: "~тать";
        non_past_first: "~чу", "~чем";
        non_past_second: "~чешь", "~чете";
        non_past_third: "~чет", "~чут";
        past: "~тал", "~тала", "~тало", "~тали";
        imperative: "~чи", "~чите";
        participle: "~чущий", "~тавший", derived, "~танный";
        gerund: "~ча", "~тав|~тавши";
    ),
    // Развеять: 113 words (баять, лаять, чуять, сеять, блеять, посеять). Like Играть, except in 8 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("развеять", 113, "~ять", like Играть;
        infinitive: "~ять";
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: "~ял", "~яла", "~яло", "~яли";
        imperative: _, _;
        participle: _, "~явший", _, "~янный";
        gerund: _, "~яв|~явши";
    ),
    // Перестроиться: 110 words (беспокоиться, успокоиться, приклеиться, строиться, устроиться, настроиться). Like Числиться, except in 2 cells: imperative sg, imperative pl.
    conjugation!("перестроиться", 110, "~ить~я", like Числиться;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: "~й~я", "~йте~ь";
        participle: _, _, _, _;
        gerund: _, _;
    ),
    // Держаться: 108 words (воздержаться, мчаться, держаться, послышаться, слышаться, содержаться). Like Обнажиться, except in 7 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("держаться", 108, "~ать~я", like Обнажиться;
        infinitive: "~ать~я";
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: "~ал~я", "~ала~ь", "~ало~ь", "~али~ь";
        imperative: _, _;
        participle: _, "~авший~я", _, _;
        gerund: _, "~авши~ь";
    ),
    // Сычевать: 108 words (бичевать, воевать, кочевать, завоевать, ночевать, корчевать). Like Вибрировать, except in 8 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("сычевать", 108, "~евать", like Вибрировать;
        infinitive: "~евать";
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: "~евал", "~евала", "~евало", "~евали";
        imperative: _, _;
        participle: _, "~евавший", _, "~ёванный";
        gerund: _, "~евав|~евавши";
    ),
    // Кишеть: 105 words (загреметь, храпеть, кипеть, терпеть, скрипеть, греметь). Like Восстановить, except in 8 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("кишеть", 105, "~еть", like Восстановить;
        infinitive: "~еть";
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: "~ел", "~ела", "~ело", "~ели";
        imperative: _, _;
        participle: _, "~евший", _, derived;
        gerund: _, "~ев|~евши";
    ),
    // Сорваться: 103 words (ебаться, стебаться, усраться, обосраться, дождаться, заебаться). Like Обернуться, except in 8 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("сорваться", 103, "~ать~я", like Обернуться;
        infinitive: "~ать~я";
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: "~ал~я", "~ала~ь", "~ало~ь|~ало~ь", "~али~ь|~али~ь";
        imperative: _, _;
        participle: _, "~авший~я", _, _;
        gerund: "~я~ь", "~авши~ь";
    ),
    // Трафить: 103 words (суесловить, буравить, брасопить, дыбить, шароёбить, стрёмить). Like Представить, except in 2 cells: participle pres_pass, gerund past.
    conjugation!("трафить", 103, "~ить", [aspect: Imperfective], like Представить;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: _, _;
        participle: _, _, derived, _;
        gerund: _, "~ивши|~ив";
    ),
    // Вывозить: 102 words (возить, привозить, скользить, грозить, выразить, возразить).
    conjugation!("вывозить", 102, "~зить";
        infinitive: "~зить";
        non_past_first: "~жу", "~зим";
        non_past_second: "~зишь", "~зите";
        non_past_third: "~зит", "~зят";
        past: "~зил", "~зила", "~зило", "~зили";
        imperative: "~зи", "~зите";
        participle: "~зящий", "~зивший", "~зимый", "~женный";
        gerund: "~зя", "~зив|~зивши";
    ),
    // Наладиться: 95 words (ладиться, наладиться, отвадиться, сгрудиться, заладиться, повадиться). Like Расходиться, except in 2 cells: imperative sg, imperative pl.
    conjugation!("наладиться", 95, "~дить~я", like Расходиться;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: "~дь~я", "~дьте~ь";
        participle: _, _, _, _;
        gerund: _, _;
    ),
    // Переброситься: 93 words (выситься, броситься, возвыситься, повыситься, свеситься, наброситься). Like Носиться, except in 2 cells: imperative sg, imperative pl.
    conjugation!("переброситься", 93, "~~иться", like Носиться;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: "~~ься", "~~ьтесь";
        participle: _, _, _, _;
        gerund: _, _;
    ),
    // Елозить: 90 words (лазить, заморозить, морозить, конфузить, унизить, снизить). Like Вывозить, except in 2 cells: imperative sg, imperative pl.
    conjugation!("елозить", 90, "~зить", like Вывозить;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: "~зь", "~зьте";
        participle: _, _, _, _;
        gerund: _, _;
    ),
    // Подкинуть: 88 words (покинуть, тронуть, кинуть, клюнуть, плюнуть, сунуть). Like Тянуть, except in 3 cells: imperative sg, imperative pl, participle pres_act.
    conjugation!("подкинуть", 88, "~уть", [aspect: Perfective], like Тянуть;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: "~ь", "~ьте";
        participle: derived, _, _, _;
        gerund: _, _;
    ),
    // Открыть: 83 words (открыть, изныть, мыть, помыть, закрыть, рыть).
    conjugation!("открыть", 83, "~ыть";
        infinitive: "~ыть";
        non_past_first: "~ою", "~оем";
        non_past_second: "~оешь", "~оете";
        non_past_third: "~оет", "~оют";
        past: "~ыл", "~ыла", "~ыло", "~ыли";
        imperative: "~ой", "~ойте";
        participle: "~оющий", "~ывший", "~оемый", "~ытый";
        gerund: "~оя", "~ыв|~ывши";
    ),
    // Наладить: 83 words (гладить, погладить, лихорадить, бредить, ладить, наладить). Like Разводить, except in 2 cells: imperative sg, imperative pl.
    conjugation!("наладить", 83, "~дить", like Разводить;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: "~дь", "~дьте";
        participle: _, _, _, _;
        gerund: _, _;
    ),
    // Выразиться: 81 words (сразиться, возиться, выразиться, выгрузиться, отразиться, грузиться).
    conjugation!("выразиться", 81, "~зить~я";
        infinitive: "~зить~я";
        non_past_first: "~жу~ь", "~зим~я";
        non_past_second: "~зишь~я", "~зите~ь";
        non_past_third: "~зит~я", "~зят~я";
        past: "~зил~я", "~зила~ь", "~зило~ь", "~зили~ь";
        imperative: "~зи~ь", "~зите~ь";
        participle: "~зящий~я", "~зивший~я", derived, derived;
        gerund: "~зя~ь", "~зивши~ь";
    ),
    // Зарядить: 79 words (щадить, пощадить, опередить, зарядить, отрядить, снарядить). Like Разводить, except in 1 cell: participle past_pass.
    conjugation!("зарядить", 79, "~дить", like Разводить;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: _, _;
        participle: _, _, _, "~жённый";
        gerund: _, _;
    ),
    // Стать: 78 words (стать, устать, застрять, перестать, встать, надеть). Like Залипнуть, except in 8 cells: infinitive, past m, imperative sg, imperative pl, participle pres_act, participle past_act, ….
    conjugation!("стать", 78, "~ть", [aspect: Perfective], like Залипнуть;
        infinitive: "~ть";
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: "~л", _, _, _;
        imperative: "~нь", "~ньте";
        participle: derived, "~вший", _, "~тый";
        gerund: _, "~в|~вши";
    ),
    // Отдавать: 77 words (давать, воссоздавать, создавать, издавать, уставать, предавать).
    conjugation!("отдавать", 77, "~вать";
        infinitive: "~вать";
        non_past_first: "~ю", "~ём";
        non_past_second: "~ёшь", "~ёте";
        non_past_third: "~ёт", "~ют";
        past: "~вал", "~вала", "~вало", "~вали";
        imperative: "~вай", "~вайте";
        participle: "~ющий", "~вавший", "~ваемый", derived;
        gerund: "~вая", "~вав|~вавши";
    ),
    // Развеяться: 73 words (надеяться, каяться, покаяться, маяться, измаяться, отчаяться). Like Набираться, except in 7 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("развеяться", 73, "~ять~я", like Набираться;
        infinitive: "~ять~я";
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: "~ял~я", "~яла~ь", "~яло~ь", "~яли~ь";
        imperative: _, _;
        participle: _, "~явший~я", _, _;
        gerund: _, "~явши~ь";
    ),
    // Коптить: 72 words (коптить, сплотить, золотить, закоптить, кипятить, вскипятить). Like Хватить, except in 1 cell: participle past_pass.
    conjugation!("коптить", 72, "~тить", like Хватить;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: _, _;
        participle: _, _, _, "~чённый";
        gerund: _, _;
    ),
    // Стушеваться: 71 words (стушеваться, тушеваться, разбушеваться, зарубцеваться, кольцеваться, вытанцеваться). Like Стыковаться, except in 7 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("стушеваться", 71, "~евать~я", like Стыковаться;
        infinitive: "~евать~я";
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: "~евал~я", "~евала~ь", "~евало~ь", "~евали~ь";
        imperative: _, _;
        participle: _, "~евавший~я", _, _;
        gerund: _, "~евавши~ь";
    ),
    // Бухтеть: 68 words (лететь, хотеть, полететь, захотеть, взлететь, вертеть). Like Хватить, except in 7 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("бухтеть", 68, "~теть", like Хватить;
        infinitive: "~теть";
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: "~тел", "~тела", "~тело", "~тели";
        imperative: _, _;
        participle: _, "~тевший", _, _;
        gerund: _, "~тев|~тевши";
    ),
    // Забить: 68 words (пить, бить, лить, вить, шить, побить).
    conjugation!("забить", 68, "~ить";
        infinitive: "~ить";
        non_past_first: "~ью", "~ьём";
        non_past_second: "~ьёшь", "~ьёте";
        non_past_third: "~ьёт", "~ьют";
        past: "~ил", "~ила", "~ило", "~или";
        imperative: "~ей", "~ейте";
        participle: "~ьющий", "~ивший", derived, "~итый";
        gerund: "~ья", "~ив|~ивши";
    ),
    // Сблизиться: 66 words (приблизиться, сконфузиться, близиться, снизиться, сузиться, сблизиться). Like Выразиться, except in 2 cells: imperative sg, imperative pl.
    conjugation!("сблизиться", 66, "~зить~я", like Выразиться;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: "~зь~я", "~зьте~ь";
        participle: _, _, _, _;
        gerund: _, _;
    ),
    // Отвоевать: 63 words (мухлевать, горевать, малевать, полевать, царевать, чаевать).
    conjugation!("отвоевать", 63, "~евать";
        infinitive: "~евать";
        non_past_first: "~юю", "~юем";
        non_past_second: "~юешь", "~юете";
        non_past_third: "~юет", "~юют";
        past: "~евал", "~евала", "~евало", "~евали";
        imperative: "~юй", "~юйте";
        participle: "~юющий", "~евавший", "~юемый", "~ёванный";
        gerund: "~юя", "~евав|~евавши";
    ),
    // Стоить: 62 words (беспокоить, строить, стоить, устроить, построить, усвоить). Like Ударить, except in 2 cells: imperative sg, imperative pl.
    conjugation!("стоить", 62, "~ить", like Ударить;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: "~й", "~йте";
        participle: _, _, _, _;
        gerund: _, _;
    ),
    // Зевать: 61 words (зевать, оседлать, хлебать, седлать, верстать, сверстать).
    conjugation!("зевать", 61, "~е~ть";
        infinitive: "~е~ть";
        non_past_first: "~е~ю", "~е~ем";
        non_past_second: "~е~ешь", "~е~ете";
        non_past_third: "~е~ет", "~е~ют";
        past: "~е~л", "~е~ла", "~е~ло", "~е~ли";
        imperative: "~е~й", "~е~йте";
        participle: "~е~ющий", "~е~вший", "~е~емый", "~ё~нный";
        gerund: "~е~я", "~е~в|~е~вши";
    ),
    // Обтечь: 60 words (печь, испечь, спечь, стечь, течь, развлечь).
    conjugation!("обтечь", 60, "~ечь", [aspect: Perfective];
        infinitive: "~ечь";
        non_past_first: "~еку", "~ечём";
        non_past_second: "~ечёшь", "~ечёте";
        non_past_third: "~ечёт", "~екут";
        past: "~ёк", "~екла", "~екло", "~екли";
        imperative: "~еки", "~еките";
        participle: "~екущий", "~ёкший", "~екомый", "~ечённый";
        gerund: derived, "~ёкши";
    ),
    // Возбудить: 59 words (освободить, победить, предупредить, повредить, руководить, обсудить). Like Разводить, except in 1 cell: participle past_pass.
    conjugation!("возбудить", 59, "~дить", like Разводить;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: _, _;
        participle: _, _, _, "~ждённый";
        gerund: _, _;
    ),
    // Сидеть: 57 words (бздеть, набздеть, сидеть, пиздеть, пердеть, выглядеть). Like Зарядить, except in 7 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("сидеть", 57, "~деть", like Зарядить;
        infinitive: "~деть";
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: "~дел", "~дела", "~дело", "~дели";
        imperative: _, _;
        participle: _, "~девший", _, _;
        gerund: _, "~дев|~девши";
    ),
    // Виться: 57 words (литься, биться, добиться, напиться, налиться, пробиться).
    conjugation!("виться", 57, "~ить~я";
        infinitive: "~ить~я";
        non_past_first: "~ью~ь", "~ьём~я";
        non_past_second: "~ьёшь~я", "~ьёте~ь";
        non_past_third: "~ьёт~я", "~ьют~я";
        past: "~ил~я|~ил~я", "~ила~ь", "~ило~ь|~ило~ь", "~или~ь|~или~ь";
        imperative: "~ей~я", "~ейте~ь";
        participle: "~ьющий~я", "~ивший~я", derived, derived;
        gerund: "~ья~ь", "~ивши~ь";
    ),
    // Перевернуть: 57 words (перевернуть, повернуть, взметнуть, свернуть, завернуть, подчеркнуть).
    conjugation!("перевернуть", 57, "~е~уть", [aspect: Perfective];
        infinitive: "~е~уть";
        non_past_first: "~е~у", "~е~ём";
        non_past_second: "~е~ёшь", "~е~ёте";
        non_past_third: "~е~ёт", "~е~ут";
        past: "~е~ул", "~е~ула", "~е~уло", "~е~ули";
        imperative: "~е~и", "~е~ите";
        participle: derived, "~е~увший", derived, "~ё~утый";
        gerund: derived, "~е~ув|~е~увши";
    ),
    // Сунуться: 57 words (двинуться, прикинуться, кинуться, дотронуться, тронуться, ринуться). Like Тянуться, except in 3 cells: imperative sg, imperative pl, participle pres_act.
    conjugation!("сунуться", 57, "~уть~я", [aspect: Perfective], like Тянуться;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: "~ь~я", "~ьте~ь";
        participle: derived, _, _, _;
        gerund: _, _;
    ),
    // Раскрыться: 55 words (мыться, помыться, скрыться, открыться, умыться, срыться).
    conjugation!("раскрыться", 55, "~ыть~я";
        infinitive: "~ыть~я";
        non_past_first: "~ою~ь", "~оем~я";
        non_past_second: "~оешь~я", "~оете~ь";
        non_past_third: "~оет~я", "~оют~я";
        past: "~ыл~я", "~ыла~ь", "~ыло~ь", "~ыли~ь";
        imperative: "~ой~я", "~ойте~ь";
        participle: "~оющий~я", "~ывший~я", derived, derived;
        gerund: "~оя~ь", "~ывши~ь";
    ),
    // Колоть: 55 words (полоть, колоть, пороть, выполоть, прополоть, выпороть).
    conjugation!("колоть", 55, "~оть";
        infinitive: "~оть";
        non_past_first: "~ю", "~ем";
        non_past_second: "~ешь", "~ете";
        non_past_third: "~ет", "~ют";
        past: "~ол", "~ола", "~оло", "~оли";
        imperative: "~и", "~ите";
        participle: "~ющий", "~овший", derived, "~отый";
        gerund: "~я", "~ов|~овши";
    ),
    // Заглохнуть: 54 words (достигнуть, проникнуть, возникнуть, затихнуть, подвергнуть, расторгнуть). Like Залипнуть, except in 4 cells: participle pres_act, participle past_act, participle past_pass, gerund past.
    conjugation!("заглохнуть", 54, "~нуть", [aspect: Perfective], like Залипнуть;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: _, _;
        participle: derived, "~ший|~нувший", _, "~нутый";
        gerund: _, "~нув|~ши|~нувши";
    ),
    // Идти: 53 words (идти, пасти, спасти, ползти, припасти, упасти). Like Засосать, except in 9 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("идти", 53, "~ти", like Засосать;
        infinitive: "~ти";
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: "~", "~ла", "~ло", "~ли";
        imperative: _, _;
        participle: _, "~ший", "~омый", "~ённый";
        gerund: _, "~ши";
    ),
    // Влечься: 53 words (отречься, увлечься, растечься, отвлечься, стечься, влечься).
    conjugation!("влечься", 53, "~ечь~я";
        infinitive: "~ечь~я";
        non_past_first: "~еку~ь", "~ечём~я";
        non_past_second: "~ечёшь~я", "~ечёте~ь";
        non_past_third: "~ечёт~я", "~екут~я";
        past: "~ёк~я", "~екла~ь", "~екло~ь", "~екли~ь";
        imperative: "~еки~ь", "~еките~ь";
        participle: "~екущий~я", "~ёкший~я", derived, derived;
        gerund: derived, "~ёкши~ь";
    ),
    // Чесаться: 53 words (подписаться, записаться, писаться, расписаться, вписаться, описаться).
    conjugation!("чесаться", 53, "~~аться";
        infinitive: "~~аться";
        non_past_first: "~шу~ь", "~шем~я";
        non_past_second: "~шешь~я", "~шете~ь";
        non_past_third: "~шет~я", "~шут~я";
        past: "~~ался", "~~алась", "~~алось", "~~ались";
        imperative: "~ши~ь", "~шите~ь";
        participle: "~шущий~я", "~~авшийся", derived, derived;
        gerund: "~ша~ь", "~~авшись";
    ),
    // Насытить: 53 words (допустить, пропустить, чистить, пустить, упустить, почистить). Like Отметить, except in 2 cells: non_past_first sg, participle past_pass.
    conjugation!("насытить", 53, "~тить", like Отметить;
        infinitive: _;
        non_past_first: "~щу", _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: _, _;
        participle: _, _, _, "~щенный";
        gerund: _, _;
    ),
    // Отказаться: 52 words (казаться, оказаться, показаться, отказаться, обязаться, высказаться).
    conjugation!("отказаться", 52, "~зать~я";
        infinitive: "~зать~я";
        non_past_first: "~жу~ь", "~жем~я";
        non_past_second: "~жешь~я", "~жете~ь";
        non_past_third: "~жет~я", "~жут~я";
        past: "~зал~я", "~зала~ь", "~зало~ь", "~зали~ь";
        imperative: "~жи~ь", "~жите~ь";
        participle: "~жущий~я", "~завший~я", derived, derived;
        gerund: "~жа~ь", "~завши~ь";
    ),
    // Пасть: 52 words (класть, пропасть, напасть, красть, украсть, пасть).
    conjugation!("пасть", 52, "~сть";
        infinitive: "~сть";
        non_past_first: "~ду", "~дём";
        non_past_second: "~дёшь", "~дёте";
        non_past_third: "~дёт", "~дут";
        past: "~л", "~ла", "~ло", "~ли";
        imperative: "~ди", "~дите";
        participle: "~дущий", "~вший", "~домый", "~денный";
        gerund: "~дя", "~в|~вши";
    ),
    // Отдаваться: 51 words (издаваться, оставаться, сдаваться, расставаться, удаваться, раздаваться).
    conjugation!("отдаваться", 51, "~вать~я", [aspect: Imperfective];
        infinitive: "~вать~я";
        non_past_first: "~ю~ь", "~ём~я";
        non_past_second: "~ёшь~я", "~ёте~ь";
        non_past_third: "~ёт~я", "~ют~я";
        past: "~вал~я", "~вала~ь", "~вало~ь", "~вали~ь";
        imperative: "~вай~я", "~вайте~ь";
        participle: "~ющий~я", "~вавший~я", derived, derived;
        gerund: "~вая~ь", "~вавши~ь";
    ),
    // Глядеться: 48 words (увидеться, видеться, привидеться, вглядеться, наглядеться, оглядеться). Like Расходиться, except in 7 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("глядеться", 48, "~деть~я", like Расходиться;
        infinitive: "~деть~я";
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: "~дел~я", "~дела~ь", "~дело~ь", "~дели~ь";
        imperative: _, _;
        participle: _, "~девший~я", _, _;
        gerund: _, "~девши~ь";
    ),
    // Развести: 48 words (провести, перевести, увести, завести, произвести, извести).
    conjugation!("развести", 48, "~ести", [aspect: Perfective];
        infinitive: "~ести";
        non_past_first: "~еду", "~едём";
        non_past_second: "~едёшь", "~едёте";
        non_past_third: "~едёт", "~едут";
        past: "~ёл", "~ела", "~ело", "~ели";
        imperative: "~еди", "~едите";
        participle: derived, "~едший", derived, "~едённый";
        gerund: derived, "~едя|~едши";
    ),
    // Разбить: 46 words (сшить, разбить, сбить, свить, развить, разлить).
    conjugation!("разбить", 46, "~~ить", [aspect: Perfective];
        infinitive: "~~ить";
        non_past_first: "~о~ью", "~о~ьём";
        non_past_second: "~о~ьёшь", "~о~ьёте";
        non_past_third: "~о~ьёт", "~о~ьют";
        past: "~~ил", "~~ила", "~~ило", "~~или";
        imperative: "~~ей", "~~ейте";
        participle: derived, "~~ивший", derived, "~~итый";
        gerund: derived, "~~ив|~~ивши";
    ),
    // Снести: 45 words (изнести, принести, привезти, произнести, донести, перенести). Like Перевернуть, except in 8 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("снести", 45, "~е~ти", [aspect: Perfective], like Перевернуть;
        infinitive: "~е~ти";
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: "~ё~", "~е~ла", "~е~ло", "~е~ли";
        imperative: _, _;
        participle: _, "~ё~ший", _, "~е~ённый";
        gerund: _, "~е~я|~ё~ши";
    ),
    // Клевать: 45 words (блевать, плевать, клевать, жевать, пожевать, наплевать). Like Отвоевать, except in 5 cells: non_past_first pl, non_past_second sg, non_past_second pl, non_past_third sg, participle pres_pass.
    conjugation!("клевать", 45, "~евать", like Отвоевать;
        infinitive: _;
        non_past_first: _, "~юём";
        non_past_second: "~юёшь", "~юёте";
        non_past_third: "~юёт", _;
        past: _, _, _, _;
        imperative: _, _;
        participle: _, _, derived, _;
        gerund: _, _;
    ),
    // Колоться: 42 words (бороться, приколоться, расколоться, колоться, напороться, побороться). Like Набираться, except in 9 cells: infinitive, past m, past f, past n, past pl, imperative sg, ….
    conjugation!("колоться", 42, "~оть~я", like Набираться;
        infinitive: "~оть~я";
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: "~ол~я", "~ола~ь", "~оло~ь", "~оли~ь";
        imperative: "~и~ь", "~ите~ь";
        participle: _, "~овший~я", _, _;
        gerund: _, "~овши~ь";
    ),
    // Топтаться: 42 words (метаться, топтаться, расхохотаться, заметаться, шептаться, потоптаться).
    conjugation!("топтаться", 42, "~тать~я";
        infinitive: "~тать~я";
        non_past_first: "~чу~ь", "~чем~я";
        non_past_second: "~чешь~я", "~чете~ь";
        non_past_third: "~чет~я", "~чут~я";
        past: "~тал~я", "~тала~ь", "~тало~ь", "~тали~ь";
        imperative: "~чи~ь", "~чите~ь";
        participle: "~чущий~я", "~тавший~я", derived, derived;
        gerund: "~ча~ь", "~тавши~ь";
    ),
    // Драть: 42 words (брать, избрать, собрать, удрать, забрать, убрать).
    conjugation!("драть", 42, "~~ать";
        infinitive: "~~ать";
        non_past_first: "~е~у", "~е~ём";
        non_past_second: "~е~ёшь", "~е~ёте";
        non_past_third: "~е~ёт", "~е~ут";
        past: "~~ал", "~~ала", "~~ало", "~~али";
        imperative: "~е~и", "~е~ите";
        participle: "~е~ущий", "~~авший", "~е~омый", "~~анный";
        gerund: "~е~я", "~~ав|~~авши";
    ),
    // Пыхать: 42 words (въехать, махать, брехать, объехать, подъехать, пахать).
    conjugation!("пыхать", 42, "~хать";
        infinitive: "~хать";
        non_past_first: "~шу", "~шем";
        non_past_second: "~шешь", "~шете";
        non_past_third: "~шет", "~шут";
        past: "~хал", "~хала", "~хало", "~хали";
        imperative: "~ши", "~шите";
        participle: "~шущий", "~хавший", derived, "~ханный";
        gerund: "~ша", "~хав|~хавши";
    ),
    // Сдать: 41 words (дать, воссоздать, создать, издать, предать, выдать).
    conjugation!("сдать", 41, "~ть", [aspect: Perfective];
        infinitive: "~ть";
        non_past_first: "~м", "~дим";
        non_past_second: "~шь", "~дите";
        non_past_third: "~ст", "~дут";
        past: "~л|~л", "~ла", "~ло", "~ли|~ли";
        imperative: "~й", "~йте";
        participle: derived, "~вший", derived, "~нный";
        gerund: derived, "~в|~вши";
    ),
    // Выписать: 40 words (писать, написать, подписать, приписать, описать, записать). Like Пыхать, except in 8 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("выписать", 40, "~сать", like Пыхать;
        infinitive: "~сать";
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: "~сал", "~сала", "~сало", "~сали";
        imperative: _, _;
        participle: _, "~савший", _, "~санный";
        gerund: _, "~сав|~савши";
    ),
    // Отобразить: 39 words (вообразить, отразить, исказить, разить, сразить, водрузить). Like Вывозить, except in 1 cell: participle past_pass.
    conjugation!("отобразить", 39, "~зить", like Вывозить;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: _, _;
        participle: _, _, _, "~жённый";
        gerund: _, _;
    ),
    // Сбиться: 39 words (влиться, сбиться, развиться, взвиться, разбиться, отбиться).
    conjugation!("сбиться", 39, "~~ить~я", [aspect: Perfective];
        infinitive: "~~ить~я";
        non_past_first: "~о~ью~ь", "~о~ьём~я";
        non_past_second: "~о~ьёшь~я", "~о~ьёте~ь";
        non_past_third: "~о~ьёт~я", "~о~ьют~я";
        past: "~~ил~я", "~~ила~ь", "~~ило~ь", "~~или~ь";
        imperative: "~~ей~я", "~~ейте~ь";
        participle: derived, "~~ивший~я", derived, derived;
        gerund: derived, "~~ивши~ь";
    ),
    // Сволочь: 39 words (мочь, смочь, помочь, сволочь, волочь, приволочь).
    conjugation!("сволочь", 39, "~чь";
        infinitive: "~чь";
        non_past_first: "~ку", "~чём";
        non_past_second: "~чёшь", "~чёте";
        non_past_third: "~чёт", "~кут";
        past: "~к", "~кла", "~кло", "~кли";
        imperative: "~ки", "~ките";
        participle: "~кущий", "~кший", derived, "~чённый";
        gerund: derived, "~кши";
    ),
    // Служить: 37 words (сообщить, обобщить, приобщить, истощить, разобщить, переборщить). Like Палить, except in 4 cells: non_past_first sg, non_past_third pl, participle pres_act, gerund pres.
    conjugation!("служить", 37, "~ить", like Палить;
        infinitive: _;
        non_past_first: "~у", _;
        non_past_second: _, _;
        non_past_third: _, "~ат";
        past: _, _, _, _;
        imperative: _, _;
        participle: "~ащий", _, _, _;
        gerund: "~а", _;
    ),
    // Обратиться: 37 words (превратиться, обратиться, прекратиться, возвратиться, смутиться, возмутиться). Like Закатиться, except in 2 cells: non_past_first sg, gerund past.
    conjugation!("обратиться", 37, "~тить~я", [aspect: Perfective], like Закатиться;
        infinitive: _;
        non_past_first: "~щу~ь", _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: _, _;
        participle: _, _, _, _;
        gerund: _, "~тивши~ь|~тя~ь";
    ),
    // Трястись: 36 words (трястись, пастись, сотрястись, спастись, соблюстись, затрястись). Like Сорваться, except in 7 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("трястись", 36, "~ти~ь", like Сорваться;
        infinitive: "~ти~ь";
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: "~~я", "~ла~ь", "~ло~ь", "~ли~ь";
        imperative: _, _;
        participle: _, "~ший~я", _, _;
        gerund: _, "~ши~ь";
    ),
    // Загореться: 36 words (рассмотреться, смотреться, загореться, предусмотреться, просмотреться, присмотреться). Like Числиться, except in 7 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("загореться", 36, "~еть~я", like Числиться;
        infinitive: "~еть~я";
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: "~ел~я", "~ела~ь", "~ело~ь", "~ели~ь";
        imperative: _, _;
        participle: _, "~евший~я", _, _;
        gerund: _, "~евши~ь";
    ),
    // Гореть: 35 words (сгореть, смотреть, гореть, посмотреть, велеть, загореть). Like Кишеть, except in 1 cell: non_past_first sg.
    conjugation!("гореть", 35, "~еть", like Кишеть;
        infinitive: _;
        non_past_first: "~ю", _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: _, _;
        participle: _, _, _, _;
        gerund: _, _;
    ),
    // Трепаться: 34 words (колебаться, поколебаться, трепаться, истрепаться, заколебаться, затрепаться).
    conjugation!("трепаться", 34, "~ать~я";
        infinitive: "~ать~я";
        non_past_first: "~лю~ь", "~лем~я";
        non_past_second: "~лешь~я", "~лете~ь";
        non_past_third: "~лет~я", "~лют~я";
        past: "~ал~я", "~ала~ь", "~ало~ь", "~али~ь";
        imperative: "~ли~ь", "~лите~ь";
        participle: "~лющий~я", "~авший~я", derived, derived;
        gerund: "~ля~ь", "~авши~ь";
    ),
    // Сплести: 34 words (сплести, расплести, подмести, взгнести, помести, смести).
    conjugation!("сплести", 34, "~ести", [aspect: Perfective];
        infinitive: "~ести";
        non_past_first: "~ету", "~етём";
        non_past_second: "~етёшь", "~етёте";
        non_past_third: "~етёт", "~етут";
        past: "~ёл", "~ела", "~ело", "~ели";
        imperative: "~ети", "~етите";
        participle: derived, "~ётший", derived, "~етённый";
        gerund: derived, "~етя|~ётши";
    ),
    // Расчесать: 33 words (чесать, тесать, расчесать, почесать, причесать, обтесать).
    conjugation!("расчесать", 33, "~есать";
        infinitive: "~есать";
        non_past_first: "~ешу", "~ешем";
        non_past_second: "~ешешь", "~ешете";
        non_past_third: "~ешет", "~ешут";
        past: "~есал", "~есала", "~есало", "~есали";
        imperative: "~еши", "~ешите";
        participle: "~ешущий", "~есавший", derived, "~ёсанный";
        gerund: "~еша", "~есав|~есавши";
    ),
    // Станцевать: 33 words (танцевать, потанцевать, облицевать, вальцевать, станцевать, взлупцевать). Like Сычевать, except in 1 cell: participle past_pass.
    conjugation!("станцевать", 33, "~евать", like Сычевать;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: _, _;
        participle: _, _, _, "~ованный";
        gerund: _, _;
    ),
    // Щипать: 33 words (засыпать, дремать, задремать, насыпать, посыпать, щипать).
    conjugation!("щипать", 33, "~ать";
        infinitive: "~ать";
        non_past_first: "~лю", "~лем";
        non_past_second: "~лешь", "~лете";
        non_past_third: "~лет", "~лют";
        past: "~ал", "~ала", "~ало", "~али";
        imperative: "~ли", "~лите";
        participle: "~лющий", "~авший", "~аемый", "~анный";
        gerund: "~ля", "~ав|~авши";
    ),
    // Резаться: 32 words (врезаться, резаться, мазаться, нарезаться, размазаться, измазаться). Like Отказаться, except in 2 cells: imperative sg, imperative pl.
    conjugation!("резаться", 32, "~зать~я", like Отказаться;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: "~жь~я", "~жьте~ь";
        participle: _, _, _, _;
        gerund: _, _;
    ),
    // Отвоеваться: 32 words (отвоеваться, навоеваться, довоеваться, развоеваться, изгореваться, нагореваться).
    conjugation!("отвоеваться", 32, "~евать~я";
        infinitive: "~евать~я";
        non_past_first: "~юю~ь", "~юем~я";
        non_past_second: "~юешь~я", "~юете~ь";
        non_past_third: "~юет~я", "~юют~я";
        past: "~евал~я", "~евала~ь", "~евало~ь", "~евали~ь";
        imperative: "~юй~я", "~юйте~ь";
        participle: "~юющий~я", "~евавший~я", derived, derived;
        gerund: "~юя~ь", "~евавши~ь";
    ),
    // Обыскать: 31 words (искать, полоскать, отыскать, поискать, разыскать, обыскать).
    conjugation!("обыскать", 31, "~скать";
        infinitive: "~скать";
        non_past_first: "~щу", "~щем";
        non_past_second: "~щешь", "~щете";
        non_past_third: "~щет", "~щут";
        past: "~скал", "~скала", "~скало", "~скали";
        imperative: "~щи", "~щите";
        participle: "~щущий", "~скавший", "~скаемый", "~сканный";
        gerund: "~ща", "~скав|~скавши";
    ),
    // Сковать: 30 words (ковать, подковать, совать, основать, сновать, приковать). Like Вибрировать, except in 5 cells: non_past_first pl, non_past_second sg, non_past_second pl, non_past_third sg, participle pres_pass.
    conjugation!("сковать", 30, "~овать", like Вибрировать;
        infinitive: _;
        non_past_first: _, "~уём";
        non_past_second: "~уёшь", "~уёте";
        non_past_third: "~уёт", _;
        past: _, _, _, _;
        imperative: _, _;
        participle: _, _, derived, _;
        gerund: _, _;
    ),
    // Грести: 30 words (грести, погрести, скрести, сгрести, разгрести, поскрести).
    conjugation!("грести", 30, "~ести";
        infinitive: "~ести";
        non_past_first: "~ебу", "~ебём";
        non_past_second: "~ебёшь", "~ебёте";
        non_past_third: "~ебёт", "~ебут";
        past: "~ёб", "~ебла", "~ебло", "~ебли";
        imperative: "~еби", "~ебите";
        participle: "~ебущий", "~ёбший", "~ебомый", "~ебённый";
        gerund: "~ебя", "~ёбши";
    ),
    // Искусить: 30 words (пригласить, оросить, огласить, провозгласить, воскресить, вопросить). Like Сносить, except in 1 cell: participle past_pass.
    conjugation!("искусить", 30, "~сить", like Сносить;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: _, _;
        participle: _, _, _, "~шённый";
        gerund: _, _;
    ),
    // Распасться: 29 words (красться, попасться, распасться, подкрасться, вкрасться, спасться).
    conjugation!("распасться", 29, "~~ться";
        infinitive: "~~ться";
        non_past_first: "~ду~ь", "~дём~я";
        non_past_second: "~дёшь~я", "~дёте~ь";
        non_past_third: "~дёт~я", "~дут~я";
        past: "~л~я", "~ла~ь", "~ло~ь", "~ли~ь";
        imperative: "~ди~ь", "~дите~ь";
        participle: "~дущий~я", "~вший~я", derived, derived;
        gerund: "~дя~ь", "~вши~ь";
    ),
    // Вывестись: 28 words (вестись, развестись, произвестись, свестись, довестись, привестись).
    conjugation!("вывестись", 28, "~е~тись", [aspect: Perfective];
        infinitive: "~е~тись";
        non_past_first: "~еду~ь", "~едём~я";
        non_past_second: "~едёшь~я", "~едёте~ь";
        non_past_third: "~едёт~я", "~едут~я";
        past: "~ёл~я", "~ела~ь", "~ело~ь", "~ели~ь";
        imperative: "~еди~ь", "~едите~ь";
        participle: derived, "~едший~я", derived, derived;
        gerund: derived, "~едя~ь";
    ),
    // Отпустить: 27 words (вырастить, вымостить, обесчестить, бесчестить, запостить, постить).
    conjugation!("отпустить", 27, "~стить";
        infinitive: "~стить";
        non_past_first: "~щу", "~стим";
        non_past_second: "~стишь", "~стите";
        non_past_third: "~стит", "~стят";
        past: "~стил", "~стила", "~стило", "~стили";
        imperative: "~сти", "~стите";
        participle: "~стящий", "~стивший", "~стимый", "~щенный";
        gerund: "~стя", "~стив|~стивши";
    ),
    // Зайти: 27 words (произойти, найти, пойти, дойти, выйти, прийти).
    conjugation!("зайти", 27, "~йти", [aspect: Perfective];
        infinitive: "~йти";
        non_past_first: "~йду", "~йдём";
        non_past_second: "~йдёшь", "~йдёте";
        non_past_third: "~йдёт", "~йдут";
        past: "~шёл", "~шла", "~шло", "~шли";
        imperative: "~йди", "~йдите";
        participle: derived, "~шедший", derived, "~йдённый";
        gerund: derived, "~йдя|~шедши";
    ),
    // Набраться: 27 words (драться, собраться, придраться, добраться, браться, убраться).
    conjugation!("набраться", 27, "~~ать~я";
        infinitive: "~~ать~я";
        non_past_first: "~е~у~ь", "~е~ём~я";
        non_past_second: "~е~ёшь~я", "~е~ёте~ь";
        non_past_third: "~е~ёт~я", "~е~ут~я";
        past: "~~ал~я|~~ал~я", "~~ала~ь", "~~ало~ь|~~ало~ь", "~~али~ь|~~али~ь";
        imperative: "~е~и~ь", "~е~ите~ь";
        participle: "~е~ущий~я", "~~авший~я", derived, derived;
        gerund: "~е~я~ь", "~~авши~ь";
    ),
    // Плакать: 26 words (плакать, тыкать, оплакать, кликать, заплакать, поплакать).
    conjugation!("плакать", 26, "~кать";
        infinitive: "~кать";
        non_past_first: "~чу", "~чем";
        non_past_second: "~чешь", "~чете";
        non_past_third: "~чет", "~чут";
        past: "~кал", "~кала", "~кало", "~кали";
        imperative: "~чь", "~чьте";
        participle: "~чущий", "~кавший", derived, "~канный";
        gerund: "~ча", "~кав|~кавши";
    ),
    // Просвистеть: 26 words (блестеть, свистеть, хрустеть, заблестеть, насвистеть, посвистеть). Like Отпустить, except in 8 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("просвистеть", 26, "~стеть", like Отпустить;
        infinitive: "~стеть";
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: "~стел", "~стела", "~стело", "~стели";
        imperative: _, _;
        participle: _, "~стевший", _, derived;
        gerund: _, "~стев|~стевши";
    ),
    // Упереться: 26 words (запереться, упереться, опереться, простереться, тереться, потереться).
    conjugation!("упереться", 26, "~е~еть~я";
        infinitive: "~е~еть~я";
        non_past_first: "~~у~ь", "~~ём~я";
        non_past_second: "~~ёшь~я", "~~ёте~ь";
        non_past_third: "~~ёт~я", "~~ут~я";
        past: "~ё~~я", "~ё~ла~ь", "~ё~ло~ь", "~ё~ли~ь";
        imperative: "~~и~ь", "~~ите~ь";
        participle: "~~ущий~я", "~ё~ший~я", derived, derived;
        gerund: derived, "~ё~ши~ь";
    ),
    // Терпеться: 26 words (терпеться, натерпеться, обтерпеться, притерпеться, раскипеться, расшуметься). Like Загореться, except in 1 cell: non_past_first sg.
    conjugation!("терпеться", 26, "~еть~я", like Загореться;
        infinitive: _;
        non_past_first: "~лю~ь", _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: _, _;
        participle: _, _, _, _;
        gerund: _, _;
    ),
    // Плеваться: 26 words (плеваться, расплеваться, наклеваться, наплеваться, облеваться, обплеваться). Like Отвоеваться, except in 4 cells: non_past_first pl, non_past_second sg, non_past_second pl, non_past_third sg.
    conjugation!("плеваться", 26, "~евать~я", like Отвоеваться;
        infinitive: _;
        non_past_first: _, "~юём~я";
        non_past_second: "~юёшь~я", "~юёте~ь";
        non_past_third: "~юёт~я", _;
        past: _, _, _, _;
        imperative: _, _;
        participle: _, _, _, _;
        gerund: _, _;
    ),
    // Стричься: 26 words (напрячься, достичься, стричься, остричься, обстричься, подстричься).
    conjugation!("стричься", 26, "~чь~я", [aspect: Perfective];
        infinitive: "~чь~я";
        non_past_first: "~гу~ь", "~жём~я";
        non_past_second: "~жёшь~я", "~жёте~ь";
        non_past_third: "~жёт~я", "~гут~я";
        past: "~г~я", "~гла~ь", "~гло~ь", "~гли~ь";
        imperative: "~ги~ь", "~гите~ь";
        participle: "~гущий~я", "~гший~я", derived, derived;
        gerund: derived, "~гши~ь";
    ),
    // Наплыть: 25 words (жить, плыть, отплыть, ожить, зажить, пожить).
    conjugation!("наплыть", 25, "~ть";
        infinitive: "~ть";
        non_past_first: "~ву", "~вём";
        non_past_second: "~вёшь", "~вёте";
        non_past_third: "~вёт", "~вут";
        past: "~л", "~ла", "~ло", "~ли";
        imperative: "~ви", "~вите";
        participle: "~вущий", "~вший", derived, derived;
        gerund: "~вя", "~в|~вши";
    ),
    // Тухнуть: 25 words (исчезнуть, привыкнуть, крепнуть, гибнуть, дрыхнуть, гаснуть). Like Залипнуть, except in 3 cells: past m, participle past_act, gerund past.
    conjugation!("тухнуть", 25, "~нуть", like Залипнуть;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: "~|~нул", _, _, _;
        imperative: _, _;
        participle: _, "~нувший", _, _;
        gerund: _, "~нув|~нувши";
    ),
    // Разлететься: 24 words (вертеться, разлететься, завертеться, отвертеться, довертеться, навертеться). Like Закатиться, except in 7 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("разлететься", 24, "~теть~я", like Закатиться;
        infinitive: "~теть~я";
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: "~тел~я", "~тела~ь", "~тело~ь", "~тели~ь";
        imperative: _, _;
        participle: _, "~тевший~я", _, _;
        gerund: _, "~тевши~ь";
    ),
    // Расплыться: 24 words (житься, прижиться, расплыться, ужиться, разжиться, сжиться).
    conjugation!("расплыться", 24, "~ть~я";
        infinitive: "~ть~я";
        non_past_first: "~ву~ь", "~вём~я";
        non_past_second: "~вёшь~я", "~вёте~ь";
        non_past_third: "~вёт~я", "~вут~я";
        past: "~л~я|~л~я", "~ла~ь", "~ло~ь|~ло~ь", "~ли~ь";
        imperative: "~ви~ь", "~вите~ь";
        participle: derived, "~вший~я", derived, derived;
        gerund: derived, "~вши~ь";
    ),
    // Вторгнуться: 24 words (вторгнуться, подвергнуться, проникнуться, слипнуться, выдохнуться, смеркнуться).
    conjugation!("вторгнуться", 24, "~нуть~я", [aspect: Perfective];
        infinitive: "~нуть~я";
        non_past_first: "~ну~ь", "~нем~я";
        non_past_second: "~нешь~я", "~нете~ь";
        non_past_third: "~нет~я", "~нут~я";
        past: "~~я", "~ла~ь", "~ло~ь", "~ли~ь";
        imperative: "~ни~ь", "~ните~ь";
        participle: "~нущий~я", "~ший~я", derived, derived;
        gerund: derived, "~ши~ь";
    ),
    // Отнять: 24 words (поднять, снять, обнять, отнять, приподнять, подъять).
    conjugation!("отнять", 24, "~ять", [aspect: Perfective];
        infinitive: "~ять";
        non_past_first: "~иму", "~имем";
        non_past_second: "~имешь", "~имете";
        non_past_third: "~имет", "~имут";
        past: "~ял", "~яла", "~яло", "~яли";
        imperative: "~ими", "~имите";
        participle: derived, "~явший", derived, "~ятый";
        gerund: derived, "~яв|~явши";
    ),
    // Проскочить: 23 words (тащить, вытащить, затащить, потащить, притащить, стащить). Like Служить, except in 1 cell: participle past_pass.
    conjugation!("проскочить", 23, "~ить", [aspect: Perfective], like Служить;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: _, _;
        participle: _, _, _, "~енный";
        gerund: _, _;
    ),
    // Заесть: 23 words (есть, съесть, поесть, надоесть, выесть, разъесть). Like Пасть, except in 9 cells: non_past_first sg, non_past_first pl, non_past_second sg, non_past_second pl, non_past_third sg, non_past_third pl, ….
    conjugation!("заесть", 23, "~сть", [aspect: Perfective], like Пасть;
        infinitive: _;
        non_past_first: "~м", "~дим";
        non_past_second: "~шь", "~дите";
        non_past_third: "~ст", "~дят";
        past: _, _, _, _;
        imperative: "~шь", "~шьте";
        participle: "~дящий", _, _, _;
        gerund: _, _;
    ),
    // Затереть: 22 words (упереть, распростереть, потереть, протереть, простереть, утереть).
    conjugation!("затереть", 22, "~е~еть", [aspect: Perfective];
        infinitive: "~е~еть";
        non_past_first: "~~у", "~~ём";
        non_past_second: "~~ёшь", "~~ёте";
        non_past_third: "~~ёт", "~~ут";
        past: "~ё~", "~ё~ла", "~ё~ло", "~ё~ли";
        imperative: "~~и", "~~ите";
        participle: derived, "~ё~ший", derived, "~ё~тый";
        gerund: derived, "~е~ев|~ё~ши";
    ),
    // Отдаться: 21 words (издаться, удаться, сдаться, раздаться, выдаться, даться). Like Расплыться, except in 9 cells: non_past_first sg, non_past_first pl, non_past_second sg, non_past_second pl, non_past_third sg, non_past_third pl, ….
    conjugation!("отдаться", 21, "~ть~я", [aspect: Perfective], like Расплыться;
        infinitive: _;
        non_past_first: "~м~я", "~дим~я";
        non_past_second: "~шь~я", "~дите~ь";
        non_past_third: "~~тся", "~дут~я";
        past: _, _, _, "~ли~ь|~ли~ь";
        imperative: "~й~я", "~йте~ь";
        participle: _, _, _, _;
        gerund: _, _;
    ),
    // Дорасти: 21 words (расти, возрасти, порасти, зарасти, нарасти, подрасти).
    conjugation!("дорасти", 21, "~а~ти", [aspect: Perfective];
        infinitive: "~а~ти";
        non_past_first: "~а~ту", "~а~тём";
        non_past_second: "~а~тёшь", "~а~тёте";
        non_past_third: "~а~тёт", "~а~тут";
        past: "~о~", "~о~ла", "~о~ло", "~о~ли";
        imperative: "~а~ти", "~а~тите";
        participle: "~а~тущий", "~о~ший", derived, derived;
        gerund: "~а~тя", "~о~ши";
    ),
    // Забыть: 21 words (быть, прибыть, забыть, добыть, побыть, сбыть).
    conjugation!("забыть", 21, "~ыть", [aspect: Perfective];
        infinitive: "~ыть";
        non_past_first: "~уду", "~удем";
        non_past_second: "~удешь", "~удете";
        non_past_third: "~удет", "~удут";
        past: "~ыл", "~ыла", "~ыло", "~ыли";
        imperative: "~удь", "~удьте";
        participle: derived, "~ывший", derived, "~ытый";
        gerund: derived, "~ыв|~ывши";
    ),
    // Унестись: 21 words (произнестись, донестись, отнестись, пронестись, понестись, перенестись). Like Набраться, except in 9 cells: infinitive, past m, past f, past n, past pl, participle pres_act, ….
    conjugation!("унестись", 21, "~е~ти~ь", [aspect: Perfective], like Набраться;
        infinitive: "~е~ти~ь";
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: "~ё~~я", "~е~ла~ь", "~е~ло~ь", "~е~ли~ь";
        imperative: _, _;
        participle: derived, "~ё~ший~я", _, _;
        gerund: derived, "~е~я~ь|~ё~ши~ь";
    ),
    // Набежать: 20 words (бежать, побежать, избежать, выбежать, убежать, сбежать).
    conjugation!("набежать", 20, "~жать", [aspect: Perfective];
        infinitive: "~жать";
        non_past_first: "~гу", "~жим";
        non_past_second: "~жишь", "~жите";
        non_past_third: "~жит", "~гут";
        past: "~жал", "~жала", "~жало", "~жали";
        imperative: "~ги", "~гите";
        participle: "~гущий", "~жавший", derived, derived;
        gerund: derived, "~жав|~жавши";
    ),
    // Расстаться: 20 words (остаться, расстаться, одеться, раздеться, переодеться, достаться). Like Вторгнуться, except in 7 cells: infinitive, past m, imperative sg, imperative pl, participle pres_act, participle past_act, ….
    conjugation!("расстаться", 20, "~ть~я", [aspect: Perfective], like Вторгнуться;
        infinitive: "~ть~я";
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: "~л~я", _, _, _;
        imperative: "~нь~я", "~ньте~ь";
        participle: derived, "~вший~я", _, _;
        gerund: _, "~вши~ь";
    ),
    // Грызть: 20 words (грызть, обгрызть, огрызть, догрызть, загрызть, изгрызть). Like Идти, except in 2 cells: infinitive, participle past_pass.
    conjugation!("грызть", 20, "~ть", like Идти;
        infinitive: "~ть";
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: _, _;
        participle: _, _, _, "~енный";
        gerund: _, _;
    ),
    // Лезть: 20 words (залезть, лезть, полезть, вылезть, влезть, слезть). Like Грызть, except in 8 cells: non_past_first pl, non_past_second sg, non_past_second pl, non_past_third sg, imperative sg, imperative pl, ….
    conjugation!("лезть", 20, "~ть", [aspect: Perfective], like Грызть;
        infinitive: _;
        non_past_first: _, "~ем";
        non_past_second: "~ешь", "~ете";
        non_past_third: "~ет", _;
        past: _, _, _, _;
        imperative: "~ь", "~ьте";
        participle: _, _, derived, derived;
        gerund: _, _;
    ),
    // Зашибить: 20 words (вышибить, зашибить, ушибить, сшибить, прошибить, пришибить). Like Идти, except in 7 cells: infinitive, participle pres_act, participle past_act, participle pres_pass, participle past_pass, gerund pres, ….
    conjugation!("зашибить", 20, "~ить", [aspect: Perfective], like Идти;
        infinitive: "~ить";
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: _, _;
        participle: derived, "~ивший", derived, "~ленный";
        gerund: derived, "~ив|~ивши";
    ),
    // Перепеть: 20 words (петь, допеть, запеть, напеть, пропеть, взреветь).
    conjugation!("перепеть", 20, "~еть", [aspect: Perfective];
        infinitive: "~еть";
        non_past_first: "~ою", "~оём";
        non_past_second: "~оёшь", "~оёте";
        non_past_third: "~оёт", "~оют";
        past: "~ел", "~ела", "~ело", "~ели";
        imperative: "~ой", "~ойте";
        participle: "~оющий", "~евший", derived, "~етый";
        gerund: "~оя", "~ев|~евши";
    ),
    // Занять: 19 words (понять, занять, нанять, унять, донять, перенять).
    conjugation!("занять", 19, "~нять", [aspect: Perfective];
        infinitive: "~нять";
        non_past_first: "~йму", "~ймём";
        non_past_second: "~ймёшь", "~ймёте";
        non_past_third: "~ймёт", "~ймут";
        past: "~нял", "~няла", "~няло", "~няли";
        imperative: "~йми", "~ймите";
        participle: derived, "~нявший", derived, "~нятый";
        gerund: derived, "~няв|~нявши";
    ),
    // Лечь: 19 words (лечь, прилечь, залечь, облечь, пролечь, налечь).
    conjugation!("лечь", 19, "~ечь", [aspect: Perfective];
        infinitive: "~ечь";
        non_past_first: "~ягу", "~яжем";
        non_past_second: "~яжешь", "~яжете";
        non_past_third: "~яжет", "~ягут";
        past: "~ёг", "~егла", "~егло", "~егли";
        imperative: "~яг", "~ягте";
        participle: derived, "~ёгший", derived, derived;
        gerund: derived, "~ёгши";
    ),
    // Плескаться: 19 words (плескаться, отыскаться, выплескаться, доискаться, обыскаться, полоскаться).
    conjugation!("плескаться", 19, "~~каться";
        infinitive: "~~каться";
        non_past_first: "~щу~ь", "~щем~я";
        non_past_second: "~щешь~я", "~щете~ь";
        non_past_third: "~щет~я", "~щут~я";
        past: "~~кался", "~~калась", "~~калось", "~~кались";
        imperative: "~щи~ь", "~щите~ь";
        participle: "~щущий~я", "~~кавшийся", derived, derived;
        gerund: "~ща~ь", "~~кавшись";
    ),
    // Предназначить: 18 words (встопорщить, вывощить, наплющить, натопорщить, повытащить, поморщить). Like Проскочить, except in 2 cells: imperative sg, imperative pl.
    conjugation!("предназначить", 18, "~ить", [aspect: Perfective], like Проскочить;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: "~ь", "~ьте";
        participle: _, _, _, _;
        gerund: _, _;
    ),
    // Просмотреть: 18 words (презреть, рассмотреть, предусмотреть, осмотреть, подсмотреть, присмотреть). Like Гореть, except in 4 cells: participle pres_act, participle pres_pass, participle past_pass, gerund pres.
    conjugation!("просмотреть", 18, "~еть", [aspect: Perfective], like Гореть;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: _, _;
        participle: derived, _, derived, "~енный";
        gerund: derived, _;
    ),
    // Потчевать: 18 words (потчевать, выкорчевать, вымежевать, запотчевать, напотчевать, отпотчевать). Like Сычевать, except in 1 cell: participle past_pass.
    conjugation!("потчевать", 18, "~евать", like Сычевать;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: _, _;
        participle: _, _, _, "~еванный";
        gerund: _, _;
    ),
    // Въесться: 18 words (наесться, въесться, объесться, приесться, взъесться, заесться). Like Отдаться, except in 8 cells: infinitive, non_past_third pl, past m, past n, past pl, imperative sg, ….
    conjugation!("въесться", 18, "~~ться", like Отдаться;
        infinitive: "~~ться";
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, "~дят~я";
        past: "~л~я", _, "~ло~ь", "~ли~ь";
        imperative: "~шь~я", "~шьте~ь";
        participle: "~дящий~я", _, _, _;
        gerund: _, _;
    ),
    // Устояться: 17 words (бояться, состояться, забояться, побояться, устояться, застояться). Like Развеяться, except in 6 cells: non_past_first pl, non_past_second sg, non_past_second pl, non_past_third sg, non_past_third pl, participle pres_act.
    conjugation!("устояться", 17, "~ять~я", like Развеяться;
        infinitive: _;
        non_past_first: _, "~им~я";
        non_past_second: "~ишь~я", "~ите~ь";
        non_past_third: "~ит~я", "~ят~я";
        past: _, _, _, _;
        imperative: _, _;
        participle: "~ящий~я", _, _, _;
        gerund: _, _;
    ),
    // Трепетать: 17 words (клеветать, трепетать, роптать, скрежетать, затрепетать, оклеветать). Like Обыскать, except in 9 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("трепетать", 17, "~тать", like Обыскать;
        infinitive: "~тать";
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: "~тал", "~тала", "~тало", "~тали";
        imperative: _, _;
        participle: _, "~тавший", derived, derived;
        gerund: _, "~тав|~тавши";
    ),
    // Жать: 17 words (нажать, выжать, жать, пожать, зажать, прижать).
    conjugation!("жать", 17, "~ать";
        infinitive: "~ать";
        non_past_first: "~му", "~мём";
        non_past_second: "~мёшь", "~мёте";
        non_past_third: "~мёт", "~мут";
        past: "~ал", "~ала", "~ало", "~али";
        imperative: "~ми", "~мите";
        participle: "~мущий", "~авший", derived, "~атый";
        gerund: "~мя", "~ав|~авши";
    ),
    // Вгрызться: 17 words (вгрызться, нагрызться, отгрызться, погрызться, разлезться, прогрызться). Like Трястись, except in 1 cell: infinitive.
    conjugation!("вгрызться", 17, "~ть~я", like Трястись;
        infinitive: "~ть~я";
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: _, _;
        participle: _, _, _, _;
        gerund: _, _;
    ),
    // Взъебать: 17 words (заебать, проебать, поебать, разъебать, наебать, объебать). Like Драть, except in 9 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("взъебать", 17, "~е~ать", like Драть;
        infinitive: "~е~ать";
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: "~е~ал", "~е~ала", "~е~ало", "~е~али";
        imperative: _, _;
        participle: _, "~е~авший", derived, "~ё~анный";
        gerund: _, "~е~ав|~е~авши";
    ),
    // Колыхаться: 17 words (колыхаться, набрехаться, напахаться, отбрехаться, размахаться, выпахаться). Like Чесаться, except in 7 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("колыхаться", 17, "~хать~я", like Чесаться;
        infinitive: "~хать~я";
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: "~хал~я", "~хала~ь", "~хало~ь", "~хали~ь";
        imperative: _, _;
        participle: _, "~хавший~я", _, _;
        gerund: _, "~хавши~ь";
    ),
    // Разобрать: 16 words (вобрать, разобрать, подобрать, отобрать, ободрать, содрать).
    conjugation!("разобрать", 16, "~о~~ать", [aspect: Perfective];
        infinitive: "~о~~ать";
        non_past_first: "~~е~у", "~~е~ём";
        non_past_second: "~~е~ёшь", "~~е~ёте";
        non_past_third: "~~е~ёт", "~~е~ут";
        past: "~о~~ал", "~о~~ала", "~о~~ало", "~о~~али";
        imperative: "~~е~и", "~~е~ите";
        participle: derived, "~о~~авший", derived, "~о~~анный";
        gerund: derived, "~о~~ав|~о~~авши";
    ),
    // Выровнять: 16 words (выровнять, приравнять, прировнять, сравнять, сровнять, уравнять). Like Отнять, except in 9 cells: non_past_first sg, non_past_first pl, non_past_second sg, non_past_second pl, non_past_third sg, non_past_third pl, ….
    conjugation!("выровнять", 16, "~ять", [aspect: Perfective], like Отнять;
        infinitive: _;
        non_past_first: "~яю", "~яем";
        non_past_second: "~яешь", "~яете";
        non_past_third: "~яет", "~яют";
        past: _, _, _, _;
        imperative: "~яй", "~яйте";
        participle: _, _, _, "~енный";
        gerund: _, _;
    ),
    // Трепать: 16 words (трепать, потрепать, растрепать, истрепать, встрепать, затрепать).
    conjugation!("трепать", 16, "~е~ать";
        infinitive: "~е~ать";
        non_past_first: "~е~лю", "~е~лем";
        non_past_second: "~е~лешь", "~е~лете";
        non_past_third: "~е~лет", "~е~лют";
        past: "~е~ал", "~е~ала", "~е~ало", "~е~али";
        imperative: "~е~ли", "~е~лите";
        participle: "~е~лющий", "~е~авший", derived, "~ё~анный";
        gerund: "~е~ля", "~е~ав|~е~авши";
    ),
    // Заволочься: 16 words (заволочься, обволочься, взволочься, волочься, вволочься, доволочься).
    conjugation!("заволочься", 16, "~чь~я";
        infinitive: "~чь~я";
        non_past_first: "~ку~ь", "~чём~я";
        non_past_second: "~чёшь~я", "~чёте~ь";
        non_past_third: "~чёт~я", "~кут~я";
        past: "~к~я", "~кла~ь", "~кло~ь", "~кли~ь";
        imperative: "~ки~ь", "~ките~ь";
        participle: "~кущий~я", "~кший~я", derived, derived;
        gerund: derived, "~кши~ь";
    ),
    // Честь: 15 words (прочесть, честь, предпочесть, вычесть, учесть, почесть).
    conjugation!("честь", 15, "~есть";
        infinitive: "~есть";
        non_past_first: "~ту", "~тём";
        non_past_second: "~тёшь", "~тёте";
        non_past_third: "~тёт", "~тут";
        past: "~ёл", "~ла", "~ло", "~ли";
        imperative: "~ти", "~тите";
        participle: derived, derived, derived, "~тённый";
        gerund: derived, "~тя|~ётши";
    ),
    // Насесть: 15 words (сесть, засесть, присесть, осесть, подсесть, пересесть).
    conjugation!("насесть", 15, "~есть", [aspect: Perfective];
        infinitive: "~есть";
        non_past_first: "~яду", "~ядем";
        non_past_second: "~ядешь", "~ядете";
        non_past_third: "~ядет", "~ядут";
        past: "~ел", "~ела", "~ело", "~ели";
        imperative: "~ядь", "~ядьте";
        participle: derived, "~евший", derived, derived;
        gerund: derived, "~ев|~евши";
    ),
    // Смеяться: 15 words (смеяться, засмеяться, рассмеяться, посмеяться, досмеяться, надсмеяться). Like Развеяться, except in 4 cells: non_past_first pl, non_past_second sg, non_past_second pl, non_past_third sg.
    conjugation!("смеяться", 15, "~ять~я", like Развеяться;
        infinitive: _;
        non_past_first: _, "~ём~я";
        non_past_second: "~ёшь~я", "~ёте~ь";
        non_past_third: "~ёт~я", _;
        past: _, _, _, _;
        imperative: _, _;
        participle: _, _, _, _;
        gerund: _, _;
    ),
    // Гнать: 15 words (гнать, догнать, выгнать, погнать, пригнать, изгнать).
    conjugation!("гнать", 15, "~~ать";
        infinitive: "~~ать";
        non_past_first: "~о~ю", "~о~им";
        non_past_second: "~о~ишь", "~о~ите";
        non_past_third: "~о~ит", "~о~ят";
        past: "~~ал", "~~ала", "~~ало", "~~али";
        imperative: "~о~и", "~о~ите";
        participle: derived, "~~авший", derived, "~~анный";
        gerund: derived, "~~ав|~~авши";
    ),
    // Постричь: 15 words (пренебречь, стричь, подстричь, остричь, постричь, обстричь).
    conjugation!("постричь", 15, "~чь";
        infinitive: "~чь";
        non_past_first: "~гу", "~жём";
        non_past_second: "~жёшь", "~жёте";
        non_past_third: "~жёт", "~гут";
        past: "~г", "~гла", "~гло", "~гли";
        imperative: "~ги", "~гите";
        participle: "~гущий", "~гший", derived, "~женный";
        gerund: derived, "~гши";
    ),
    // Помять: 15 words (мять, помять, распять, замять, домять, вымять).
    conjugation!("помять", 15, "~ять";
        infinitive: "~ять";
        non_past_first: "~ну", "~нём";
        non_past_second: "~нёшь", "~нёте";
        non_past_third: "~нёт", "~нут";
        past: "~ял", "~яла", "~яло", "~яли";
        imperative: "~ни", "~ните";
        participle: "~нущий", "~явший", derived, "~ятый";
        gerund: "~ня", "~яв|~явши";
    ),
    // Зиждить: 15 words (стелить, постелить, застелить, расстелить, устелить, зиждить). Like Выделить, except in 6 cells: non_past_first pl, non_past_second sg, non_past_second pl, non_past_third sg, non_past_third pl, participle pres_act.
    conjugation!("зиждить", 15, "~ить", like Выделить;
        infinitive: _;
        non_past_first: _, "~ем";
        non_past_second: "~ешь", "~ете";
        non_past_third: "~ет", "~ют";
        past: _, _, _, _;
        imperative: _, _;
        participle: "~ющий", _, _, _;
        gerund: _, _;
    ),
    // Гнить: 15 words (гнить, сгнить, прогнить, загнить, догнить, изгнить). Like Играть, except in 8 cells: non_past_first pl, non_past_second sg, non_past_second pl, non_past_third sg, participle pres_act, participle pres_pass, ….
    conjugation!("гнить", 15, "~ть", like Играть;
        infinitive: _;
        non_past_first: _, "~ём";
        non_past_second: "~ёшь", "~ёте";
        non_past_third: "~ёт", _;
        past: _, _, _, _;
        imperative: _, _;
        participle: derived, _, derived, derived;
        gerund: derived, _;
    ),
    // Сберечь: 15 words (беречь, стеречь, предостеречь, приберечь, сберечь, уберечь).
    conjugation!("сберечь", 15, "~ечь";
        infinitive: "~ечь";
        non_past_first: "~егу", "~ежём";
        non_past_second: "~ежёшь", "~ежёте";
        non_past_third: "~ежёт", "~егут";
        past: "~ёг", "~егла", "~егло", "~егли";
        imperative: "~еги", "~егите";
        participle: "~егущий", "~ёгший", "~егомый", "~ежённый";
        gerund: derived, "~ёгши";
    ),
    // Видеть: 14 words (видеть, увидеть, ненавидеть, предвидеть, возненавидеть, завидеть). Like Сидеть, except in 3 cells: imperative sg, imperative pl, participle past_pass.
    conjugation!("видеть", 14, "~деть", like Сидеть;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: "~дь", "~дьте";
        participle: _, _, _, "~денный";
        gerund: _, _;
    ),
    // Прожечь: 14 words (жечь, зажечь, выжечь, пожечь, прижечь, прожечь). Like Постричь, except in 5 cells: infinitive, past m, participle past_act, participle past_pass, gerund past.
    conjugation!("прожечь", 14, "~ечь", like Постричь;
        infinitive: "~ечь";
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: "~ёг", _, _, _;
        imperative: _, _;
        participle: _, "~ёгший", _, "~жённый";
        gerund: _, "~ёгши";
    ),
    // Везти: 14 words (нести, везти, заебти, поебти, взъебти, уебти). Like Снести, except in 4 cells: participle pres_act, participle pres_pass, gerund pres, gerund past.
    conjugation!("везти", 14, "~е~ти", like Снести;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: _, _;
        participle: "~е~ущий", _, "~е~омый", _;
        gerund: "~е~я", "~ё~ши";
    ),
    // Молоть: 14 words (молоть, смолоть, домолоть, вымолоть, замолоть, намолоть).
    conjugation!("молоть", 14, "~о~оть";
        infinitive: "~о~оть";
        non_past_first: "~е~ю", "~е~ем";
        non_past_second: "~е~ешь", "~е~ете";
        non_past_third: "~е~ет", "~е~ют";
        past: "~о~ол", "~о~ола", "~о~оло", "~о~оли";
        imperative: "~е~и", "~е~ите";
        participle: "~е~ющий", "~о~овший", derived, "~о~отый";
        gerund: "~е~я", "~о~ов|~о~овши";
    ),
    // Прискакать: 14 words (скакать, отскакать, заскакать, поскакать, наскакать, прискакать). Like Плакать, except in 3 cells: imperative sg, imperative pl, participle past_pass.
    conjugation!("прискакать", 14, "~кать", like Плакать;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: "~чи", "~чите";
        participle: _, _, _, derived;
        gerund: _, _;
    ),
    // Молоться: 14 words (вымолоться, молоться, замолоться, измолоться, намолоться, обмолоться).
    conjugation!("молоться", 14, "~о~оть~я";
        infinitive: "~о~оть~я";
        non_past_first: "~е~ю~ь", "~е~ем~я";
        non_past_second: "~е~ешь~я", "~е~ете~ь";
        non_past_third: "~е~ет~я", "~е~ют~я";
        past: "~о~ол~я", "~о~ола~ь", "~о~оло~ь", "~о~оли~ь";
        imperative: "~е~и~ь", "~е~ите~ь";
        participle: "~е~ющий~я", "~о~овший~я", derived, derived;
        gerund: "~е~я~ь", "~о~овши~ь";
    ),
    // Просвистать: 14 words (свистать, насвистать, посвистать, дристать, просвистать, высвистать). Like Трепетать, except in 8 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("просвистать", 14, "~стать", like Трепетать;
        infinitive: "~стать";
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: "~стал", "~стала", "~стало", "~стали";
        imperative: _, _;
        participle: _, "~ставший", _, "~станный";
        gerund: _, "~став|~ставши";
    ),
    // Глохнуть: 13 words (пухнуть, сохнуть, пахнуть, глохнуть, дрогнуть, липнуть). Like Тухнуть, except in 2 cells: participle past_act, gerund past.
    conjugation!("глохнуть", 13, "~нуть", [aspect: Imperfective], like Тухнуть;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: _, _;
        participle: _, "~ший|~нувший", _, _;
        gerund: _, "~нув|~ши|~нувши";
    ),
    // Подобраться: 13 words (разобраться, взобраться, подобраться, разодраться, взодраться, вобраться).
    conjugation!("подобраться", 13, "~о~~ать~я", [aspect: Perfective];
        infinitive: "~о~~ать~я";
        non_past_first: "~~е~у~ь", "~~е~ём~я";
        non_past_second: "~~е~ёшь~я", "~~е~ёте~ь";
        non_past_third: "~~е~ёт~я", "~~е~ут~я";
        past: "~о~~ал~я|~о~~ал~я", "~о~~ала~ь", "~о~~ало~ь|~о~~ало~ь", "~о~~али~ь|~о~~али~ь";
        imperative: "~~е~и~ь", "~~е~ите~ь";
        participle: derived, "~о~~авший~я", derived, derived;
        gerund: derived, "~о~~авши~ь";
    ),
    // Засечь: 13 words (усечь, сечь, изречь, пересечь, пресечь, наречь). Like Обтечь, except in 4 cells: past m, participle past_act, participle pres_pass, gerund past.
    conjugation!("засечь", 13, "~ечь", like Обтечь;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: "~ёк|~ек", _, _, _;
        imperative: _, _;
        participle: _, "~екший|~ёкший", derived, _;
        gerund: _, "~екши|~ёкши";
    ),
    // Толочь: 13 words (толочь, столочь, потолочь, истолочь, растолочь, втолочь). Like Сволочь, except in 4 cells: infinitive, past m, participle past_act, gerund past.
    conjugation!("толочь", 13, "~очь", like Сволочь;
        infinitive: "~очь";
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: "~ок", _, _, _;
        imperative: _, _;
        participle: _, "~окший", _, _;
        gerund: _, "~окши";
    ),
    // Сплестись: 13 words (поплестись, переплестись, заплестись, сплестись, проплестись, доплестись).
    conjugation!("сплестись", 13, "~е~тись", [aspect: Perfective];
        infinitive: "~е~тись";
        non_past_first: "~ету~ь", "~етём~я";
        non_past_second: "~етёшь~я", "~етёте~ь";
        non_past_third: "~етёт~я", "~етут~я";
        past: "~ёл~я", "~ела~ь", "~ело~ь", "~ели~ь";
        imperative: "~ети~ь", "~етите~ь";
        participle: derived, "~ётший~я", derived, derived;
        gerund: derived, "~етя~ь";
    ),
    // Тыкаться: 13 words (расплакаться, тыкаться, плакаться, доскакаться, исплакаться, наплакаться). Like Топтаться, except in 9 cells: infinitive, past m, past f, past n, past pl, imperative sg, ….
    conjugation!("тыкаться", 13, "~кать~я", like Топтаться;
        infinitive: "~кать~я";
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: "~кал~я", "~кала~ь", "~кало~ь", "~кали~ь";
        imperative: "~чь~я", "~чьте~ь";
        participle: _, "~кавший~я", _, _;
        gerund: _, "~кавши~ь";
    ),
    // Мяться: 13 words (замяться, мяться, помяться, примяться, вымяться, домяться).
    conjugation!("мяться", 13, "~ять~я";
        infinitive: "~ять~я";
        non_past_first: "~ну~ь", "~нём~я";
        non_past_second: "~нёшь~я", "~нёте~ь";
        non_past_third: "~нёт~я", "~нут~я";
        past: "~ял~я", "~яла~ь", "~яло~ь", "~яли~ь";
        imperative: "~ни~ь", "~ните~ь";
        participle: "~нущий~я", "~явший~я", derived, derived;
        gerund: "~ня~ь", "~явши~ь";
    ),
    // Отгрестись: 13 words (сгрестись, выгрестись, выскрестись, прискрестись, доскрестись, заскрестись).
    conjugation!("отгрестись", 13, "~е~тись", [aspect: Perfective];
        infinitive: "~е~тись";
        non_past_first: "~ебу~ь", "~ебём~я";
        non_past_second: "~ебёшь~я", "~ебёте~ь";
        non_past_third: "~ебёт~я", "~ебут~я";
        past: "~ёб~я", "~ебла~ь", "~ебло~ь", "~ебли~ь";
        imperative: "~еби~ь", "~ебите~ь";
        participle: derived, "~ёбший~я", derived, derived;
        gerund: derived, "~ебя~ь";
    ),
    // Нагореть: 13 words (нагореть, отгореть, отзвенеть, подзагореть, позвенеть, пообгореть). Like Просмотреть, except in 1 cell: gerund past.
    conjugation!("нагореть", 13, "~еть", like Просмотреть;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: _, _;
        participle: _, _, _, _;
        gerund: _, "~евши|~ев";
    ),
    // Вызвать: 12 words (звать, позвать, вызвать, назвать, призвать, прозвать). Like Гнать, except in 8 cells: non_past_first sg, non_past_first pl, non_past_second sg, non_past_second pl, non_past_third sg, non_past_third pl, ….
    conjugation!("вызвать", 12, "~~ать", like Гнать;
        infinitive: _;
        non_past_first: "~о~у", "~о~ём";
        non_past_second: "~о~ёшь", "~о~ёте";
        non_past_third: "~о~ёт", "~о~ут";
        past: _, _, _, _;
        imperative: _, _;
        participle: "~о~ущий", _, _, _;
        gerund: "~о~я", _;
    ),
    // Подняться: 12 words (подняться, обняться, сняться, приподняться, отняться, засняться).
    conjugation!("подняться", 12, "~ять~я", [aspect: Perfective];
        infinitive: "~ять~я";
        non_past_first: "~иму~ь", "~имем~я";
        non_past_second: "~имешь~я", "~имете~ь";
        non_past_third: "~имет~я", "~имут~я";
        past: "~ял~я|~ял~я", "~яла~ь", "~яло~ь|~яло~ь", "~яли~ь|~яли~ь";
        imperative: "~ими~ь", "~имите~ь";
        participle: derived, "~явший~я", derived, derived;
        gerund: derived, "~явши~ь";
    ),
    // Отослать: 12 words (послать, выслать, прислать, услать, подослать, сослать).
    conjugation!("отослать", 12, "~с~ать", [aspect: Perfective];
        infinitive: "~с~ать";
        non_past_first: "~ш~ю", "~ш~ём";
        non_past_second: "~ш~ёшь", "~ш~ёте";
        non_past_third: "~ш~ёт", "~ш~ют";
        past: "~с~ал", "~с~ала", "~с~ало", "~с~али";
        imperative: "~ш~и", "~ш~ите";
        participle: derived, "~с~авший", derived, "~с~анный";
        gerund: derived, "~с~ав|~с~авши";
    ),
    // Брить: 12 words (брить, побрить, выбрить, обрить, добрить, забрить). Like Забить, except in 9 cells: non_past_first sg, non_past_first pl, non_past_second sg, non_past_second pl, non_past_third sg, non_past_third pl, ….
    conjugation!("брить", 12, "~ить", like Забить;
        infinitive: _;
        non_past_first: "~ею", "~еем";
        non_past_second: "~еешь", "~еете";
        non_past_third: "~еет", "~еют";
        past: _, _, _, _;
        imperative: _, _;
        participle: "~еющий", _, "~еемый", _;
        gerund: "~ея", _;
    ),
    // Уняться: 12 words (заняться, наняться, поняться, уняться, доняться, переняться).
    conjugation!("уняться", 12, "~нять~я", [aspect: Perfective];
        infinitive: "~нять~я";
        non_past_first: "~йму~ь", "~ймём~я";
        non_past_second: "~ймёшь~я", "~ймёте~ь";
        non_past_third: "~ймёт~я", "~ймут~я";
        past: "~нял~я|~нял~я", "~няла~ь", "~няло~ь", "~няли~ь";
        imperative: "~йми~ь", "~ймите~ь";
        participle: derived, "~нявший~я", derived, derived;
        gerund: derived, "~нявши~ь";
    ),
    // Стынуть: 12 words (стынуть, застынуть, остынуть, простынуть, постынуть, настынуть). Like Стать, except in 3 cells: infinitive, participle pres_act, participle past_pass.
    conjugation!("стынуть", 12, "~нуть", like Стать;
        infinitive: "~нуть";
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: _, _;
        participle: "~нущий", _, _, derived;
        gerund: _, _;
    ),
    // Бриться: 12 words (бриться, побриться, перебриться, выбриться, обриться, пробриться).
    conjugation!("бриться", 12, "~ить~я";
        infinitive: "~ить~я";
        non_past_first: "~ею~ь", "~еем~я";
        non_past_second: "~еешь~я", "~еете~ь";
        non_past_third: "~еет~я", "~еют~я";
        past: "~ил~я", "~ила~ь", "~ило~ь", "~или~ь";
        imperative: "~ей~я", "~ейте~ь";
        participle: "~еющий~я", "~ивший~я", derived, derived;
        gerund: "~ея~ь", "~ивши~ь";
    ),
    // Морщить: 12 words (упорядочить, морщить, выпучить, сморщить, таращить, вытаращить). Like Предназначить, except in 1 cell: imperative sg.
    conjugation!("морщить", 12, "~ить", like Предназначить;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: "~и|~ь", _;
        participle: _, _, _, _;
        gerund: _, _;
    ),
    // Сослаться: 12 words (сослаться, выслаться, дослаться, заслаться, ниспослаться, отослаться).
    conjugation!("сослаться", 12, "~с~ать~я", [aspect: Perfective];
        infinitive: "~с~ать~я";
        non_past_first: "~ш~ю~ь", "~ш~ём~я";
        non_past_second: "~ш~ёшь~я", "~ш~ёте~ь";
        non_past_third: "~ш~ёт~я", "~ш~ют~я";
        past: "~с~ал~я", "~с~ала~ь", "~с~ало~ь", "~с~али~ь";
        imperative: "~ш~и~ь", "~ш~ите~ь";
        participle: derived, "~с~авший~я", derived, derived;
        gerund: derived, "~с~авши~ь";
    ),
    // Пересидеть: 12 words (просидеть, отсидеть, усидеть, насидеть, пересидеть, засидеть). Like Сидеть, except in 4 cells: participle pres_act, participle pres_pass, participle past_pass, gerund pres.
    conjugation!("пересидеть", 12, "~деть", [aspect: Perfective], like Сидеть;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: _, _;
        participle: derived, _, derived, "~женный";
        gerund: derived, _;
    ),
    // Спеться: 12 words (петься, спеться, допеться, распеться, напеться, воспеться).
    conjugation!("спеться", 12, "~еть~я";
        infinitive: "~еть~я";
        non_past_first: "~ою~ь", "~оём~я";
        non_past_second: "~оёшь~я", "~оёте~ь";
        non_past_third: "~оёт~я", "~оют~я";
        past: "~ел~я", "~ела~ь", "~ело~ь", "~ели~ь";
        imperative: "~ой~я", "~ойте~ь";
        participle: "~оющий~я", "~евший~я", derived, derived;
        gerund: "~оя~ь", "~евши~ь";
    ),
    // Стереться: 12 words (стереться, отпереться, подтереться, втереться, впереться, обтереться).
    conjugation!("стереться", 12, "~~е~еть~я", [aspect: Perfective];
        infinitive: "~~е~еть~я";
        non_past_first: "~о~~у~ь", "~о~~ём~я";
        non_past_second: "~о~~ёшь~я", "~о~~ёте~ь";
        non_past_third: "~о~~ёт~я", "~о~~ут~я";
        past: "~~ё~~я", "~~ё~ла~ь", "~~ё~ло~ь", "~~ё~ли~ь";
        imperative: "~о~~и~ь", "~о~~ите~ь";
        participle: derived, "~~ё~ший~я", derived, derived;
        gerund: derived, "~~ё~ши~ь";
    ),
    // Стелиться: 12 words (зиждиться, расстелиться, стелиться, созиждиться, взъемлиться, постелиться). Like Числиться, except in 6 cells: non_past_first pl, non_past_second sg, non_past_second pl, non_past_third sg, non_past_third pl, participle pres_act.
    conjugation!("стелиться", 12, "~ить~я", like Числиться;
        infinitive: _;
        non_past_first: _, "~ем~я";
        non_past_second: "~ешь~я", "~ете~ь";
        non_past_third: "~ет~я", "~ют~я";
        past: _, _, _, _;
        imperative: _, _;
        participle: "~ющий~я", _, _, _;
        gerund: _, _;
    ),
    // Подвигнуться: 12 words (свергнуться, разверзнуться, низвергнуться, подвигнуться, сверзнуться, исторгнуться). Like Вторгнуться, except in 3 cells: past m, participle pres_act, participle past_act.
    conjugation!("подвигнуться", 12, "~нуть~я", [aspect: Perfective], like Вторгнуться;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: derived, _, _, _;
        imperative: _, _;
        participle: derived, "~нувший~я|~ший~я", _, _;
        gerund: _, _;
    ),
    // Дожить: 11 words (выжить, прожить, пережить, дожить, нажить, изжить). Like Наплыть, except in 6 cells: past m, past n, past pl, participle pres_act, participle past_pass, gerund pres.
    conjugation!("дожить", 11, "~ть", [aspect: Perfective], like Наплыть;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: "~л|~л", _, "~ло|~ло", "~ли|~ли";
        imperative: _, _;
        participle: derived, _, _, "~тый|~тый";
        gerund: derived, _;
    ),
    // Ушибиться: 11 words (ошибиться, ушибиться, зашибиться, прошибиться, вышибиться, пришибиться). Like Трястись, except in 5 cells: infinitive, participle pres_act, participle past_act, gerund pres, gerund past.
    conjugation!("ушибиться", 11, "~ить~я", [aspect: Perfective], like Трястись;
        infinitive: "~ить~я";
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: _, _;
        participle: derived, "~ивший~я", _, _;
        gerund: derived, "~ивши~ь";
    ),
    // Зачесться: 11 words (учесться, расчесться, зачесться, почесться, обчесться, причесться).
    conjugation!("зачесться", 11, "~е~ться";
        infinitive: "~е~ться";
        non_past_first: "~ту~ь", "~тём~я";
        non_past_second: "~тёшь~я", "~тёте~ь";
        non_past_third: "~тёт~я", "~тут~я";
        past: "~ёл~я", "~ла~ь", "~ло~ь", "~ли~ь";
        imperative: "~ти~ь", "~тите~ь";
        participle: derived, derived, derived, derived;
        gerund: derived, "~тя~ь";
    ),
    // Жаться: 11 words (прижаться, жаться, пожаться, прожаться, ужаться, выжаться).
    conjugation!("жаться", 11, "~ать~я";
        infinitive: "~ать~я";
        non_past_first: "~му~ь", "~мём~я";
        non_past_second: "~мёшь~я", "~мёте~ь";
        non_past_third: "~мёт~я", "~мут~я";
        past: "~ал~я", "~ала~ь", "~ало~ь", "~али~ь";
        imperative: "~ми~ь", "~мите~ь";
        participle: "~мущий~я", "~авший~я", derived, derived;
        gerund: "~мя~ь", "~авши~ь";
    ),
    // Пакоститься: 11 words (заневеститься, засовеститься, невеститься, скапуститься, испакоститься, чихвоститься). Like Распуститься, except in 2 cells: imperative sg, imperative pl.
    conjugation!("пакоститься", 11, "~~титься", like Распуститься;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: "~~ться", "~~тьтесь";
        participle: _, _, _, _;
        gerund: _, _;
    ),
    // Зассать: 11 words (ссать, сцать, обоссать, зассать, поссать, проссать). Like Засосать, except in 8 cells: non_past_first pl, non_past_second sg, non_past_second pl, non_past_third sg, imperative sg, imperative pl, ….
    conjugation!("зассать", 11, "~ать", like Засосать;
        infinitive: _;
        non_past_first: _, "~ым";
        non_past_second: "~ышь", "~ыте";
        non_past_third: "~ыт", _;
        past: _, _, _, _;
        imperative: "~ы", "~ыте";
        participle: derived, _, _, _;
        gerund: derived, _;
    ),
    // Поехать: 10 words (поехать, приехать, уехать, проехать, выехать, переехать).
    conjugation!("поехать", 10, "~хать", [aspect: Perfective];
        infinitive: "~хать";
        non_past_first: "~ду", "~дем";
        non_past_second: "~дешь", "~дете";
        non_past_third: "~дет", "~дут";
        past: "~хал", "~хала", "~хало", "~хали";
        imperative: "~зжай", "~зжайте";
        participle: derived, "~хавший", derived, derived;
        gerund: derived, "~хав|~хавши";
    ),
    // Стоять: 10 words (стоять, настоять, постоять, состоять, противостоять, предстоять). Like Развеять, except in 8 cells: non_past_first pl, non_past_second sg, non_past_second pl, non_past_third sg, non_past_third pl, participle pres_act, ….
    conjugation!("стоять", 10, "~ять", like Развеять;
        infinitive: _;
        non_past_first: _, "~им";
        non_past_second: "~ишь", "~ите";
        non_past_third: "~ит", "~ят";
        past: _, _, _, _;
        imperative: _, _;
        participle: "~ящий", _, derived, derived;
        gerund: _, _;
    ),
    // Беречься: 10 words (беречься, остеречься, поостеречься, постеречься, стеречься, уберечься).
    conjugation!("беречься", 10, "~ечь~я";
        infinitive: "~ечь~я";
        non_past_first: "~егу~ь", "~ежём~я";
        non_past_second: "~ежёшь~я", "~ежёте~ь";
        non_past_third: "~ежёт~я", "~егут~я";
        past: "~ёг~я", "~егла~ь", "~егло~ь", "~егли~ь";
        imperative: "~еги~ь", "~егите~ь";
        participle: "~егущий~я", "~ёгший~я", derived, derived;
        gerund: derived, "~ёгши~ь";
    ),
    // Вырвать: 10 words (выебать, вырвать, выждать, высосать, выжрать, выткать). Like Засосать, except in 6 cells: non_past_first pl, non_past_second sg, non_past_second pl, non_past_third sg, participle pres_act, gerund pres.
    conjugation!("вырвать", 10, "~ать", [aspect: Perfective], like Засосать;
        infinitive: _;
        non_past_first: _, "~ем";
        non_past_second: "~ешь", "~ете";
        non_past_third: "~ет", _;
        past: _, _, _, _;
        imperative: _, _;
        participle: derived, _, _, _;
        gerund: derived, _;
    ),
    // Плескать: 10 words (плескать, рукоплескать, расплескать, зарукоплескать, наплескать, оплескать).
    conjugation!("плескать", 10, "~ескать";
        infinitive: "~ескать";
        non_past_first: "~ещу", "~ещем";
        non_past_second: "~ещешь", "~ещете";
        non_past_third: "~ещет", "~ещут";
        past: "~ескал", "~ескала", "~ескало", "~ескали";
        imperative: "~ещи", "~ещите";
        participle: "~ещущий", "~ескавший", "~ескаемый", "~ёсканный";
        gerund: "~еща", "~ескав|~ескавши";
    ),
    // Забыться: 10 words (забыться, сбыться, добыться, избыться, перезабыться, подзабыться).
    conjugation!("забыться", 10, "~ыть~я", [aspect: Perfective];
        infinitive: "~ыть~я";
        non_past_first: "~уду~ь", "~удем~я";
        non_past_second: "~удешь~я", "~удете~ь";
        non_past_third: "~удет~я", "~удут~я";
        past: "~ыл~я", "~ыла~ь", "~ыло~ь", "~ыли~ь";
        imperative: "~удь~я", "~удьте~ь";
        participle: derived, "~ывший~я", derived, derived;
        gerund: derived, "~ывши~ь";
    ),
    // Нестись: 10 words (нестись, везтись, повезтись, свезтись, перевезтись, наебтись). Like Унестись, except in 3 cells: participle pres_act, gerund pres, gerund past.
    conjugation!("нестись", 10, "~е~ти~ь", like Унестись;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: _, _;
        participle: "~е~ущий~я", _, _, _;
        gerund: "~е~я~ь", "~ё~ши~ь";
    ),
    // Гнаться: 10 words (гнаться, погнаться, угнаться, загнаться, догнаться, пригнаться).
    conjugation!("гнаться", 10, "~~ать~я";
        infinitive: "~~ать~я";
        non_past_first: "~о~ю~ь", "~о~им~я";
        non_past_second: "~о~ишь~я", "~о~ите~ь";
        non_past_third: "~о~ит~я", "~о~ят~я";
        past: "~~ал~я", "~~ала~ь", "~~ало~ь|~~ало~ь", "~~али~ь|~~али~ь";
        imperative: "~о~и~ь", "~о~ите~ь";
        participle: "~о~ящий~я", "~~авший~я", derived, derived;
        gerund: "~о~я~ь", "~~авши~ь";
    ),
    // Назваться: 10 words (вызваться, назваться, зваться, дозваться, призваться, самоназваться). Like Гнаться, except in 8 cells: non_past_first sg, non_past_first pl, non_past_second sg, non_past_second pl, non_past_third sg, non_past_third pl, ….
    conjugation!("назваться", 10, "~~ать~я", like Гнаться;
        infinitive: _;
        non_past_first: "~о~у~ь", "~о~ём~я";
        non_past_second: "~о~ёшь~я", "~о~ёте~ь";
        non_past_third: "~о~ёт~я", "~о~ут~я";
        past: "~~ал~я|~~ал~я", _, _, _;
        imperative: _, _;
        participle: "~о~ущий~я", _, _, _;
        gerund: _, _;
    ),
    // Избороздить: 10 words (нагромоздить, взгромоздить, избороздить, взбороздить, загромоздить, пригвоздить).
    conjugation!("избороздить", 10, "~здить", [aspect: Perfective];
        infinitive: "~здить";
        non_past_first: "~зжу", "~здим";
        non_past_second: "~здишь", "~здите";
        non_past_third: "~здит", "~здят";
        past: "~здил", "~здила", "~здило", "~здили";
        imperative: "~зди", "~здите";
        participle: derived, "~здивший", derived, "~ждённый";
        gerund: derived, "~здив|~здивши";
    ),
    // Свянуть: 10 words (обвянуть, повянуть, привянуть, призавянуть, отвянуть, подвянуть). Like Стынуть, except in 3 cells: participle pres_act, participle past_act, gerund past.
    conjugation!("свянуть", 10, "~нуть", [aspect: Perfective], like Стынуть;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: _, _;
        participle: derived, "~нувший", _, _;
        gerund: _, "~нувши";
    ),
    // Жеваться: 10 words (нажеваться, жеваться, дожеваться, зажеваться, пережеваться, прожеваться). Like Стушеваться, except in 4 cells: non_past_first pl, non_past_second sg, non_past_second pl, non_past_third sg.
    conjugation!("жеваться", 10, "~евать~я", like Стушеваться;
        infinitive: _;
        non_past_first: _, "~уём~я";
        non_past_second: "~уёшь~я", "~уёте~ь";
        non_past_third: "~уёт~я", _;
        past: _, _, _, _;
        imperative: _, _;
        participle: _, _, _, _;
        gerund: _, _;
    ),
    // Обидеться: 10 words (обидеться, навидеться, разобидеться, изобидеться, завидеться, ненавидеться). Like Глядеться, except in 2 cells: imperative sg, imperative pl.
    conjugation!("обидеться", 10, "~деть~я", like Глядеться;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: "~дь~я", "~дьте~ь";
        participle: _, _, _, _;
        gerund: _, _;
    ),
    // Вырваться: 10 words (вырваться, выебаться, высраться, выткаться, закармливаться, высосаться). Like Держаться, except in 7 cells: non_past_first pl, non_past_second sg, non_past_second pl, non_past_third sg, non_past_third pl, participle pres_act, ….
    conjugation!("вырваться", 10, "~ать~я", like Держаться;
        infinitive: _;
        non_past_first: _, "~ем~я";
        non_past_second: "~ешь~я", "~ете~ь";
        non_past_third: "~ет~я", "~ут~я";
        past: _, _, _, _;
        imperative: _, _;
        participle: "~ущий~я", _, _, _;
        gerund: "~ая~ь", _;
    ),
    // Разойтись: 9 words (найтись, прийтись, обойтись, пройтись, разойтись, сойтись).
    conjugation!("разойтись", 9, "~йти~ь", [aspect: Perfective];
        infinitive: "~йти~ь";
        non_past_first: "~йду~ь", "~йдём~я";
        non_past_second: "~йдёшь~я", "~йдёте~ь";
        non_past_third: "~йдёт~я", "~йдут~я";
        past: "~шёл~я", "~шла~ь", "~шло~ь", "~шли~ь";
        imperative: "~йди~ь", "~йдите~ь";
        participle: derived, "~шедший~я", derived, derived;
        gerund: derived, "~шедши~ь";
    ),
    // Стонать: 9 words (стонать, жаждать, застонать, простонать, высрать, возжаждать). Like Вырвать, except in 3 cells: participle pres_act, participle past_pass, gerund pres.
    conjugation!("стонать", 9, "~ать", like Вырвать;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: _, _;
        participle: "~ущий", _, _, derived;
        gerund: "~ая", _;
    ),
    // Стереть: 9 words (стереть, подпереть, спереть, обтереть, втереть, впереть).
    conjugation!("стереть", 9, "~~е~еть", [aspect: Perfective];
        infinitive: "~~е~еть";
        non_past_first: "~о~~у", "~о~~ём";
        non_past_second: "~о~~ёшь", "~о~~ёте";
        non_past_third: "~о~~ёт", "~о~~ут";
        past: "~~ё~", "~~ё~ла", "~~ё~ло", "~~ё~ли";
        imperative: "~о~~и", "~о~~ите";
        participle: derived, "~~ё~ший", derived, "~~ё~тый";
        gerund: derived, "~~е~ев|~~ё~ши";
    ),
    // Выспаться: 9 words (выспаться, отоспаться, доспаться, заспаться, наспаться, проспаться). Like Держаться, except in 4 cells: non_past_first sg, non_past_third pl, participle pres_act, gerund pres.
    conjugation!("выспаться", 9, "~ать~я", [aspect: Perfective], like Держаться;
        infinitive: _;
        non_past_first: "~лю~ь", _;
        non_past_second: _, _;
        non_past_third: _, "~ят~я";
        past: _, _, _, _;
        imperative: _, _;
        participle: derived, _, _, _;
        gerund: derived, _;
    ),
    // Застлать: 9 words (стлать, постлать, застлать, устлать, выстлать, достлать). Like Гнать, except in 8 cells: non_past_first sg, non_past_first pl, non_past_second sg, non_past_second pl, non_past_third sg, non_past_third pl, ….
    conjugation!("застлать", 9, "~~ать", like Гнать;
        infinitive: _;
        non_past_first: "~е~ю", "~е~ем";
        non_past_second: "~е~ешь", "~е~ете";
        non_past_third: "~е~ет", "~е~ют";
        past: _, _, _, _;
        imperative: "~е~и", "~е~ите";
        participle: _, _, _, _;
        gerund: _, _;
    ),
    // Размять: 9 words (смять, измять, размять, подмять, вмять, обмять).
    conjugation!("размять", 9, "~~ять", [aspect: Perfective];
        infinitive: "~~ять";
        non_past_first: "~о~ну", "~о~нём";
        non_past_second: "~о~нёшь", "~о~нёте";
        non_past_third: "~о~нёт", "~о~нут";
        past: "~~ял", "~~яла", "~~яло", "~~яли";
        imperative: "~о~ни", "~о~ните";
        participle: derived, "~~явший", derived, "~~ятый";
        gerund: derived, "~~яв|~~явши";
    ),
    // Толочься: 9 words (толочься, затолочься, потолочься, истолочься, дотолочься, утолочься). Like Заволочься, except in 4 cells: infinitive, past m, participle past_act, gerund past.
    conjugation!("толочься", 9, "~очь~я", like Заволочься;
        infinitive: "~очь~я";
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: "~ок~я", _, _, _;
        imperative: _, _;
        participle: _, "~окший~я", _, _;
        gerund: _, "~окши~ь";
    ),
    // Скрестись: 9 words (скрестись, грестись, огрестись, догрестись, угрестись, погрестись). Like Отгрестись, except in 3 cells: participle pres_act, gerund pres, gerund past.
    conjugation!("скрестись", 9, "~е~тись", like Отгрестись;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: _, _;
        participle: "~ебущий~я", _, _, _;
        gerund: "~ебя~ь", "~ёбши~ь";
    ),
    // Зажечься: 9 words (зажечься, ожечься, нажечься, жечься, возжечься, пережечься). Like Стричься, except in 4 cells: infinitive, past m, participle past_act, gerund past.
    conjugation!("зажечься", 9, "~ечь~я", like Стричься;
        infinitive: "~ечь~я";
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: "~ёг~я", _, _, _;
        imperative: _, _;
        participle: _, "~ёгший~я", _, _;
        gerund: _, "~ёгши~ь";
    ),
    // Мести: 9 words (гнести, плести, мести, цвести, расцвести, рассвести). Like Сплести, except in 3 cells: participle pres_act, gerund pres, gerund past.
    conjugation!("мести", 9, "~ести", like Сплести;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: _, _;
        participle: "~етущий", _, _, _;
        gerund: "~етя", "~ётши";
    ),
    // Выпить: 8 words (выпить, вышить, выбить, вылить, подвыпить, вывить). Like Забить, except in 6 cells: non_past_first pl, non_past_second sg, non_past_second pl, non_past_third sg, participle pres_act, gerund pres.
    conjugation!("выпить", 8, "~ить", [aspect: Perfective], like Забить;
        infinitive: _;
        non_past_first: _, "~ьем";
        non_past_second: "~ьешь", "~ьете";
        non_past_third: "~ьет", _;
        past: _, _, _, _;
        imperative: _, _;
        participle: derived, _, _, _;
        gerund: derived, _;
    ),
    // Разогнать: 8 words (подогнать, обогнать, отогнать, разогнать, согнать, вогнать). Like Разобрать, except in 8 cells: non_past_first sg, non_past_first pl, non_past_second sg, non_past_second pl, non_past_third sg, non_past_third pl, ….
    conjugation!("разогнать", 8, "~о~~ать", [aspect: Perfective], like Разобрать;
        infinitive: _;
        non_past_first: "~~о~ю", "~~о~им";
        non_past_second: "~~о~ишь", "~~о~ите";
        non_past_third: "~~о~ит", "~~о~ят";
        past: _, _, _, _;
        imperative: "~~о~и", "~~о~ите";
        participle: _, _, _, _;
        gerund: _, _;
    ),
    // Отозваться: 8 words (отозваться, разогнаться, взогнаться, вогнаться, обозваться, отогнаться). Like Подобраться, except in 9 cells: non_past_first sg, non_past_first pl, non_past_second sg, non_past_second pl, non_past_third sg, non_past_third pl, ….
    conjugation!("отозваться", 8, "~о~~ать~я", [aspect: Perfective], like Подобраться;
        infinitive: _;
        non_past_first: "~~о~ю~ь", "~~о~им~я";
        non_past_second: "~~о~ишь~я", "~~о~ите~ь";
        non_past_third: "~~о~ит~я", "~~о~ят~я";
        past: "~о~~ал~я", _, _, _;
        imperative: "~~о~и~ь", "~~о~ите~ь";
        participle: _, _, _, _;
        gerund: _, _;
    ),
    // Смяться: 8 words (измяться, смяться, размяться, вмяться, обмяться, подмяться).
    conjugation!("смяться", 8, "~~ять~я", [aspect: Perfective];
        infinitive: "~~ять~я";
        non_past_first: "~о~ну~ь", "~о~нём~я";
        non_past_second: "~о~нёшь~я", "~о~нёте~ь";
        non_past_third: "~о~нёт~я", "~о~нут~я";
        past: "~~ял~я", "~~яла~ь", "~~яло~ь", "~~яли~ь";
        imperative: "~о~ни~ь", "~о~ните~ь";
        participle: derived, "~~явший~я", derived, derived;
        gerund: derived, "~~явши~ь";
    ),
    // Осмеять: 8 words (осмеять, засмеять, обсмеять, пересмеять, просмеять, ссять). Like Развеять, except in 7 cells: non_past_first pl, non_past_second sg, non_past_second pl, non_past_third sg, participle pres_act, participle pres_pass, ….
    conjugation!("осмеять", 8, "~ять", like Развеять;
        infinitive: _;
        non_past_first: _, "~ём";
        non_past_second: "~ёшь", "~ёте";
        non_past_third: "~ёт", _;
        past: _, _, _, _;
        imperative: _, _;
        participle: derived, _, derived, _;
        gerund: derived, _;
    ),
    // Выклевать: 8 words (выклевать, выплевать, выблевать, вымалевать, заштемпелевать, проштемпелевать). Like Отвоевать, except in 4 cells: participle pres_act, participle pres_pass, participle past_pass, gerund pres.
    conjugation!("выклевать", 8, "~евать", [aspect: Perfective], like Отвоевать;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: _, _;
        participle: derived, _, derived, "~еванный";
        gerund: derived, _;
    ),
    // Разреветься: 8 words (выреветься, нареветься, обреветься, прореветься, разреветься, изреветься). Like Ушибиться, except in 7 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("разреветься", 8, "~еть~я", [aspect: Perfective], like Ушибиться;
        infinitive: "~еть~я";
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: "~ел~я", "~ела~ь", "~ело~ь", "~ели~ь";
        imperative: _, _;
        participle: _, "~евший~я", _, _;
        gerund: _, "~евши~ь";
    ),
    // Выпариться: 8 words (закупориться, выпариться, замусориться, умайориться, откупориться, раскупориться). Like Усилиться, except in 4 cells: imperative sg, participle pres_act, gerund pres, gerund past.
    conjugation!("выпариться", 8, "~ить~я", [aspect: Perfective], like Усилиться;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: "~ь~я|~и~ь", _;
        participle: derived, _, _, _;
        gerund: derived, "~я~ь|~ивши~ь";
    ),
    // Месть: 8 words (месть, приплесть, приобресть, выцвесть, выгнесть, процвесть). Like Сплести, except in 3 cells: infinitive, participle past_act, gerund past.
    conjugation!("месть", 8, "~есть", like Сплести;
        infinitive: "~есть";
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: _, _;
        participle: _, "~етший", _, _;
        gerund: _, "~етя";
    ),
    // Отсвистеться: 8 words (насвистеться, рассвистеться, пересвистеться, свистеться, высвистеться, досвистеться). Like Распуститься, except in 7 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("отсвистеться", 8, "~~теться", like Распуститься;
        infinitive: "~~теться";
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: "~~телся", "~~телась", "~~телось", "~~телись";
        imperative: _, _;
        participle: _, "~~тевшийся", _, _;
        gerund: _, "~~тевшись";
    ),
    // Вести: 7 words (вести, блюсти, брести, соблюсти, грясти, густи). Like Пасть, except in 5 cells: infinitive, participle past_act, participle pres_pass, participle past_pass, gerund past.
    conjugation!("вести", 7, "~сти", like Пасть;
        infinitive: "~сти";
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: _, _;
        participle: _, "~дший", derived, "~дённый";
        gerund: _, "~дши";
    ),
    // Вынести: 7 words (вынести, вывезти, выпасти, вытрясти, повывезти, повынести). Like Лезть, except in 7 cells: infinitive, imperative sg, imperative pl, participle pres_act, participle past_pass, gerund pres, ….
    conjugation!("вынести", 7, "~ти", [aspect: Perfective], like Лезть;
        infinitive: "~ти";
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: "~и", "~ите";
        participle: derived, _, _, "~енный";
        gerund: derived, "~я";
    ),
    // Нашептать: 7 words (шептать, прошептать, зашептать, нашептать, пошептать, перешептать).
    conjugation!("нашептать", 7, "~е~тать";
        infinitive: "~е~тать";
        non_past_first: "~е~чу", "~е~чем";
        non_past_second: "~е~чешь", "~е~чете";
        non_past_third: "~е~чет", "~е~чут";
        past: "~е~тал", "~е~тала", "~е~тало", "~е~тали";
        imperative: "~е~чи", "~е~чите";
        participle: "~е~чущий", "~е~тавший", derived, "~ё~танный";
        gerund: "~е~ча", "~е~тав|~е~тавши";
    ),
    // Прятаться: 7 words (прятаться, спрятаться, запрятаться, припрятаться, упрятаться, напрятаться). Like Топтаться, except in 2 cells: imperative sg, imperative pl.
    conjugation!("прятаться", 7, "~тать~я", like Топтаться;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: "~чь~я", "~чьте~ь";
        participle: _, _, _, _;
        gerund: _, _;
    ),
    // Закупорить: 7 words (выверить, закупорить, вызволить, расстопорить, застопорить, заякорить). Like Ударить, except in 4 cells: imperative sg, participle pres_act, participle pres_pass, gerund pres.
    conjugation!("закупорить", 7, "~ить", [aspect: Perfective], like Ударить;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: "~и|~ь", _;
        participle: derived, _, derived, _;
        gerund: derived, _;
    ),
    // Вымести: 7 words (выцвести, вымести, ебсти, выплести, повыцвести, мясти). Like Честь, except in 9 cells: infinitive, non_past_first pl, non_past_second sg, non_past_second pl, non_past_third sg, past m, ….
    conjugation!("вымести", 7, "~сти", like Честь;
        infinitive: "~сти";
        non_past_first: _, "~тем";
        non_past_second: "~тешь", "~тете";
        non_past_third: "~тет", _;
        past: "~л", _, _, _;
        imperative: _, _;
        participle: _, "~тший", _, "~тенный";
        gerund: _, "~тя";
    ),
    // Залгаться: 7 words (залгаться, изолгаться, отолгаться, оболгаться, разолгаться, налгаться). Like Стричься, except in 8 cells: infinitive, past m, past f, past n, past pl, participle pres_act, ….
    conjugation!("залгаться", 7, "~гать~я", [aspect: Perfective], like Стричься;
        infinitive: "~гать~я";
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: "~гал~я", "~гала~ь", "~гало~ь", "~гали~ь";
        imperative: _, _;
        participle: derived, "~гавший~я", _, _;
        gerund: _, "~гавши~ь";
    ),
    // Насытиться: 7 words (насытиться, пресытиться, пересытиться, похититься, ухититься, сытиться). Like Встретиться, except in 1 cell: non_past_first sg.
    conjugation!("насытиться", 7, "~тить~я", like Встретиться;
        infinitive: _;
        non_past_first: "~щу~ь", _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: _, _;
        participle: _, _, _, _;
        gerund: _, _;
    ),
    // Спать: 6 words (спать, поспать, отоспать, выспать, доспать, недоспать). Like Стучать, except in 6 cells: non_past_first sg, non_past_third pl, participle pres_act, participle pres_pass, participle past_pass, gerund pres.
    conjugation!("спать", 6, "~ать", like Стучать;
        infinitive: _;
        non_past_first: "~лю", _;
        non_past_second: _, _;
        non_past_third: _, "~ят";
        past: _, _, _, _;
        imperative: _, _;
        participle: "~ящий", _, derived, derived;
        gerund: "~я", _;
    ),
    // Достичь: 6 words (достичь, постичь, настичь, застичь, перестичь, пристичь).
    conjugation!("достичь", 6, "~чь", [aspect: Perfective];
        infinitive: "~чь";
        non_past_first: "~гну", "~гнем";
        non_past_second: "~гнешь", "~гнете";
        non_past_third: "~гнет", "~гнут";
        past: "~г", "~гла", "~гло", "~гли";
        imperative: "~гни", "~гните";
        participle: derived, "~гший|~гнувший", derived, "~гнутый";
        gerund: derived, "~гнув|~гши|~гнувши";
    ),
    // Разжечь: 6 words (сжечь, поджечь, отжечь, обжечь, разжечь, изжечь).
    conjugation!("разжечь", 6, "~~ечь", [aspect: Perfective];
        infinitive: "~~ечь";
        non_past_first: "~о~гу", "~о~жём";
        non_past_second: "~о~жёшь", "~о~жёте";
        non_past_third: "~о~жёт", "~о~гут";
        past: "~~ёг", "~о~гла", "~о~гло", "~о~гли";
        imperative: "~о~ги", "~о~гите";
        participle: derived, "~~ёгший", derived, "~о~жённый";
        gerund: derived, "~~ёгши";
    ),
    // Замереть: 6 words (запереть, умереть, помереть, замереть, примереть, позапереть). Like Затереть, except in 7 cells: past m, past f, past n, past pl, participle past_act, participle past_pass, ….
    conjugation!("замереть", 6, "~е~еть", [aspect: Perfective], like Затереть;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: "~е~", "~е~ла", "~е~ло", "~е~ли";
        imperative: _, _;
        participle: _, "~е~ший", _, "~е~тый";
        gerund: _, "~е~ев";
    ),
    // Морщиться: 6 words (таращиться, морщиться, поморщиться, сморщиться, топорщиться, упорядочиться). Like Обрушиться, except in 1 cell: imperative sg.
    conjugation!("морщиться", 6, "~ить~я", like Обрушиться;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: "~и~ь|~ь~я", _;
        participle: _, _, _, _;
        gerund: _, _;
    ),
    // Измыслить: 6 words (злоумыслить, умыслить, замыслить, измыслить, вымыслить, обмыслить).
    conjugation!("измыслить", 6, "~с~ить", [aspect: Perfective];
        infinitive: "~с~ить";
        non_past_first: "~с~ю", "~с~им";
        non_past_second: "~с~ишь", "~с~ите";
        non_past_third: "~с~ит", "~с~ят";
        past: "~с~ил", "~с~ила", "~с~ило", "~с~или";
        imperative: "~с~и", "~с~ите";
        participle: derived, "~с~ивший", derived, "~ш~енный";
        gerund: derived, "~с~ив|~с~ивши";
    ),
    // Обидеть: 6 words (обидеть, высидеть, изобидеть, разобидеть, забидеть, пообидеть). Like Пересидеть, except in 2 cells: imperative sg, imperative pl.
    conjugation!("обидеть", 6, "~деть", [aspect: Perfective], like Пересидеть;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: "~дь", "~дьте";
        participle: _, _, _, _;
        gerund: _, _;
    ),
    // Рассесться: 6 words (усесться, рассесться, осесться, ссесться, надсесться, обсесться). Like Разреветься, except in 9 cells: infinitive, non_past_first sg, non_past_first pl, non_past_second sg, non_past_second pl, non_past_third sg, ….
    conjugation!("рассесться", 6, "~е~ться", [aspect: Perfective], like Разреветься;
        infinitive: "~е~ться";
        non_past_first: "~яду~ь", "~ядем~я";
        non_past_second: "~ядешь~я", "~ядете~ь";
        non_past_third: "~ядет~я", "~ядут~я";
        past: _, _, _, _;
        imperative: "~ядь~я", "~ядьте~ь";
        participle: _, _, _, _;
        gerund: _, _;
    ),
    // Сжаться: 6 words (сжаться, вжаться, отжаться, обжаться, поджаться, разжаться).
    conjugation!("сжаться", 6, "~~ать~я", [aspect: Perfective];
        infinitive: "~~ать~я";
        non_past_first: "~о~му~ь", "~о~мём~я";
        non_past_second: "~о~мёшь~я", "~о~мёте~ь";
        non_past_third: "~о~мёт~я", "~о~мут~я";
        past: "~~ал~я", "~~ала~ь", "~~ало~ь", "~~али~ь";
        imperative: "~о~ми~ь", "~о~мите~ь";
        participle: derived, "~~авший~я", derived, derived;
        gerund: derived, "~~авши~ь";
    ),
    // Соваться: 6 words (коваться, основаться, соваться, подковаться, вковаться, рассоваться). Like Стыковаться, except in 4 cells: non_past_first pl, non_past_second sg, non_past_second pl, non_past_third sg.
    conjugation!("соваться", 6, "~овать~я", like Стыковаться;
        infinitive: _;
        non_past_first: _, "~уём~я";
        non_past_second: "~уёшь~я", "~уёте~ь";
        non_past_third: "~уёт~я", _;
        past: _, _, _, _;
        imperative: _, _;
        participle: _, _, _, _;
        gerund: _, _;
    ),
    // Разъехаться: 6 words (съехаться, разъехаться, проехаться, поразъехаться, подъехаться, ехаться).
    conjugation!("разъехаться", 6, "~хать~я";
        infinitive: "~хать~я";
        non_past_first: "~ду~ь", "~дем~я";
        non_past_second: "~дешь~я", "~дете~ь";
        non_past_third: "~дет~я", "~дут~я";
        past: "~хал~я", "~хала~ь", "~хало~ь", "~хали~ь";
        imperative: "~зжай~я", "~зжайте~ь";
        participle: derived, "~хавший~я", derived, derived;
        gerund: derived, "~хавши~ь";
    ),
    // Сыпаться: 6 words (рассыпаться, посыпаться, сыпаться, осыпаться, высыпаться, отсыпаться). Like Трепаться, except in 6 cells: non_past_first pl, non_past_second sg, non_past_second pl, non_past_third sg, imperative sg, imperative pl.
    conjugation!("сыпаться", 6, "~ать~я", like Трепаться;
        infinitive: _;
        non_past_first: _, "~лем~я|~ем~я";
        non_past_second: "~лешь~я|~ешь~я", "~лете~ь|~ете~ь";
        non_past_third: "~лет~я|~ет~я", _;
        past: _, _, _, _;
        imperative: "~ь~я", "~ьте~ь";
        participle: _, _, _, _;
        gerund: _, _;
    ),
    // Хлестать: 6 words (хлестать, нахлестать, похлестать, расхлестать, схлестать, захлестать). Like Плескать, except in 9 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("хлестать", 6, "~естать", like Плескать;
        infinitive: "~естать";
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: "~естал", "~естала", "~естало", "~естали";
        imperative: _, _;
        participle: _, "~еставший", derived, "~ёстанный";
        gerund: _, "~естав|~еставши";
    ),
    // Плестись: 6 words (плестись, местись, сместись, цвестись, гнестись, доместись). Like Сплестись, except in 3 cells: participle pres_act, gerund pres, gerund past.
    conjugation!("плестись", 6, "~е~тись", like Сплестись;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: _, _;
        participle: "~етущий~я", _, _, _;
        gerund: "~етя~ь", "~ётши~ь";
    ),
    // Выпасть: 6 words (выпасть, выкрасть, выпрясть, повыпасть, выкласть, повыкрасть). Like Пасть, except in 7 cells: non_past_first pl, non_past_second sg, non_past_second pl, non_past_third sg, participle pres_act, participle pres_pass, ….
    conjugation!("выпасть", 6, "~сть", [aspect: Perfective], like Пасть;
        infinitive: _;
        non_past_first: _, "~дем";
        non_past_second: "~дешь", "~дете";
        non_past_third: "~дет", _;
        past: _, _, _, _;
        imperative: _, _;
        participle: derived, _, derived, _;
        gerund: derived, _;
    ),
    // Исторгнуть: 6 words (ниспровергнуть, исторгнуть, разверзнуть, отверзнуть, нисповергнуть, развергнуть). Like Заглохнуть, except in 2 cells: past m, gerund past.
    conjugation!("исторгнуть", 6, "~нуть", [aspect: Perfective], like Заглохнуть;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: derived, _, _, _;
        imperative: _, _;
        participle: _, _, _, _;
        gerund: _, "~ши|~нув|~нувши";
    ),
    // Зачертить: 6 words (вкатить, вскатить, докатить, дочертить, зачертить, подчертить). Like Хватить, except in 4 cells: participle pres_act, participle pres_pass, participle past_pass, gerund pres.
    conjugation!("зачертить", 6, "~тить", [aspect: Perfective], like Хватить;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: _, _;
        participle: derived, _, derived, "~танный";
        gerund: derived, _;
    ),
    // Мочься: 6 words (разнемочься, выпрячься, выстричься, мочься, занемочься, перемочься). Like Стричься, except in 5 cells: non_past_first pl, non_past_second sg, non_past_second pl, non_past_third sg, participle pres_act.
    conjugation!("мочься", 6, "~чь~я", like Стричься;
        infinitive: _;
        non_past_first: _, "~жем~я";
        non_past_second: "~жешь~я", "~жете~ь";
        non_past_third: "~жет~я", _;
        past: _, _, _, _;
        imperative: _, _;
        participle: derived, _, _, _;
        gerund: _, _;
    ),
    // Исхлестаться: 6 words (насвистаться, рассвистаться, хвостаться, расхлестаться, исхлестаться, нахлестаться). Like Плескаться, except in 7 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("исхлестаться", 6, "~~таться", like Плескаться;
        infinitive: "~~таться";
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: "~~тался", "~~талась", "~~талось", "~~тались";
        imperative: _, _;
        participle: _, "~~тавшийся", _, _;
        gerund: _, "~~тавшись";
    ),
    // Взорать: 6 words (глаголать, проглаголать, обкармливать, разорать, взорать, доорать). Like Вырвать, except in 3 cells: non_past_first sg, non_past_third pl, participle past_pass.
    conjugation!("взорать", 6, "~ать", like Вырвать;
        infinitive: _;
        non_past_first: "~ю", _;
        non_past_second: _, _;
        non_past_third: _, "~ют";
        past: _, _, _, _;
        imperative: _, _;
        participle: _, _, _, "~енный";
        gerund: _, _;
    ),
    // Висеть: 5 words (висеть, зависеть, повисеть, перевисеть, провисеть). Like Сносить, except in 9 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("висеть", 5, "~сеть", like Сносить;
        infinitive: "~сеть";
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: "~сел", "~села", "~село", "~сели";
        imperative: _, _;
        participle: _, "~севший", derived, derived;
        gerund: _, "~сев|~севши";
    ),
    // Выбрать: 5 words (выбрать, выдрать, перевыбрать, повыдрать, повыбрать). Like Застлать, except in 2 cells: non_past_first sg, non_past_third pl.
    conjugation!("выбрать", 5, "~~ать", [aspect: Perfective], like Застлать;
        infinitive: _;
        non_past_first: "~е~у", _;
        non_past_second: _, _;
        non_past_third: _, "~е~ут";
        past: _, _, _, _;
        imperative: _, _;
        participle: _, _, _, _;
        gerund: _, _;
    ),
    // Отжать: 5 words (сжать, обжать, разжать, отжать, вжать). Like Гнать, except in 9 cells: non_past_first sg, non_past_first pl, non_past_second sg, non_past_second pl, non_past_third sg, non_past_third pl, ….
    conjugation!("отжать", 5, "~~ать", [aspect: Perfective], like Гнать;
        infinitive: _;
        non_past_first: "~о~му", "~о~мём";
        non_past_second: "~о~мёшь", "~о~мёте";
        non_past_third: "~о~мёт", "~о~мут";
        past: _, _, _, _;
        imperative: "~о~ми", "~о~мите";
        participle: _, _, _, "~~атый";
        gerund: _, _;
    ),
    // Отстоять: 5 words (отстоять, выстоять, простоять, застоять, достоять). Like Стоять, except in 3 cells: participle pres_act, participle past_pass, gerund pres.
    conjugation!("отстоять", 5, "~ять", [aspect: Perfective], like Стоять;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: _, _;
        participle: derived, _, _, "~янный";
        gerund: derived, _;
    ),
    // Заспать: 5 words (проспать, переспать, заспать, наспать, приспать). Like Спать, except in 3 cells: participle pres_act, participle past_pass, gerund pres.
    conjugation!("заспать", 5, "~ать", [aspect: Perfective], like Спать;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: _, _;
        participle: derived, _, _, "~анный";
        gerund: derived, _;
    ),
    // Выставиться: 5 words (понадобиться, осведомиться, выставиться, занадобиться, уведомиться). Like Расправиться, except in 3 cells: imperative sg, participle pres_act, gerund pres.
    conjugation!("выставиться", 5, "~ить~я", [aspect: Perfective], like Расправиться;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: "~и~ь|~ь~я", _;
        participle: derived, _, _, _;
        gerund: derived, _;
    ),
    // Одолеть: 5 words (преодолеть, одолеть, запечатлеть, напечатлеть, отпечатлеть). Like Просмотреть, except in 9 cells: non_past_first sg, non_past_first pl, non_past_second sg, non_past_second pl, non_past_third sg, non_past_third pl, ….
    conjugation!("одолеть", 5, "~еть", [aspect: Perfective], like Просмотреть;
        infinitive: _;
        non_past_first: "~ею", "~еем";
        non_past_second: "~еешь", "~еете";
        non_past_third: "~еет", "~еют";
        past: _, _, _, _;
        imperative: "~ей", "~ейте";
        participle: _, _, _, "~ённый";
        gerund: _, _;
    ),
    // Выставить: 5 words (выставить, выправить, осведомить, уведомить, предуведомить). Like Закупорить, except in 2 cells: non_past_first sg, participle past_pass.
    conjugation!("выставить", 5, "~ить", [aspect: Perfective], like Закупорить;
        infinitive: _;
        non_past_first: "~лю", _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: _, _;
        participle: _, _, _, "~ленный";
        gerund: _, _;
    ),
    // Выдвинуться: 5 words (высунуться, выдвинуться, грянуться, откашлянуться, самовыдвинуться). Like Сунуться, except in 1 cell: imperative sg.
    conjugation!("выдвинуться", 5, "~уть~я", [aspect: Perfective], like Сунуться;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: "~и~ь", _;
        participle: _, _, _, _;
        gerund: _, _;
    ),
    // Пробежаться: 5 words (разбежаться, сбежаться, пробежаться, поразбежаться, оббежаться).
    conjugation!("пробежаться", 5, "~жать~я", [aspect: Perfective];
        infinitive: "~жать~я";
        non_past_first: "~гу~ь", "~жим~я";
        non_past_second: "~жишь~я", "~жите~ь";
        non_past_third: "~жит~я", "~гут~я";
        past: "~жал~я", "~жала~ь", "~жало~ь", "~жали~ь";
        imperative: "~ги~ь", "~гите~ь";
        participle: derived, "~жавший~я", derived, derived;
        gerund: derived, "~жавши~ь";
    ),
    // Обжечься: 5 words (обжечься, разжечься, поджечься, сжечься, изжечься).
    conjugation!("обжечься", 5, "~~ечь~я", [aspect: Perfective];
        infinitive: "~~ечь~я";
        non_past_first: "~о~гу~ь", "~о~жём~я";
        non_past_second: "~о~жёшь~я", "~о~жёте~ь";
        non_past_third: "~о~жёт~я", "~о~гут~я";
        past: "~~ёг~я", "~о~гла~ь", "~о~гло~ь", "~о~гли~ь";
        imperative: "~о~ги~ь", "~о~гите~ь";
        participle: derived, "~~ёгший~я", derived, derived;
        gerund: derived, "~~ёгши~ь";
    ),
    // Приобрестись: 5 words (приобрестись, обрестись, вплестись, выплестись, выместись). Like Сплестись, except in 1 cell: participle past_act.
    conjugation!("приобрестись", 5, "~е~тись", [aspect: Perfective], like Сплестись;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: _, _;
        participle: _, "~етший~я", _, _;
        gerund: _, _;
    ),
    // Волгнуть: 5 words (волгнуть, брюзгнуть, горкнуть, грузнуть, осмеркнуть). Like Тухнуть, except in 1 cell: past m.
    conjugation!("волгнуть", 5, "~нуть", like Тухнуть;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: "~нул", _, _, _;
        imperative: _, _;
        participle: _, _, _, _;
        gerund: _, _;
    ),
    // Стлаться: 5 words (стлаться, подостлаться, постлаться, простлаться, устлаться). Like Молоться, except in 7 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("стлаться", 5, "~~ать~я", like Молоться;
        infinitive: "~~ать~я";
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: "~~ал~я", "~~ала~ь", "~~ало~ь", "~~али~ь";
        imperative: _, _;
        participle: _, "~~авший~я", _, _;
        gerund: _, "~~авши~ь";
    ),
    // Распоясаться: 5 words (распоясаться, выплясаться, подпоясаться, вытесаться, опоясаться). Like Чесаться, except in 4 cells: imperative sg, imperative pl, participle pres_act, gerund pres.
    conjugation!("распоясаться", 5, "~~аться", [aspect: Perfective], like Чесаться;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: "~шь~я", "~шьте~ь";
        participle: derived, _, _, _;
        gerund: derived, _;
    ),
    // Вылиться: 5 words (выбиться, вылиться, выпиться, вывиться, повыбиться). Like Бриться, except in 8 cells: non_past_first sg, non_past_first pl, non_past_second sg, non_past_second pl, non_past_third sg, non_past_third pl, ….
    conjugation!("вылиться", 5, "~ить~я", [aspect: Perfective], like Бриться;
        infinitive: _;
        non_past_first: "~ью~ь", "~ьем~я";
        non_past_second: "~ьешь~я", "~ейте~ь";
        non_past_third: "~ьет~я", "~ьют~я";
        past: _, _, _, _;
        imperative: _, _;
        participle: derived, _, _, _;
        gerund: derived, _;
    ),
    // Трепетаться: 5 words (вострепетаться, трепетаться, оклеветаться, затрепетаться, потрепетаться). Like Плескаться, except in 7 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("трепетаться", 5, "~тать~я", like Плескаться;
        infinitive: "~тать~я";
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: "~тал~я", "~тала~ь", "~тало~ь", "~тали~ь";
        imperative: _, _;
        participle: _, "~тавший~я", _, _;
        gerund: _, "~тавши~ь";
    ),
    // Ссаться: 5 words (уссаться, обоссаться, ссаться, сцаться, обосцаться). Like Вырваться, except in 7 cells: non_past_first pl, non_past_second sg, non_past_second pl, non_past_third sg, imperative sg, imperative pl, ….
    conjugation!("ссаться", 5, "~ать~я", like Вырваться;
        infinitive: _;
        non_past_first: _, "~ым~я";
        non_past_second: "~ышь~я", "~ыте~ь";
        non_past_third: "~ыт~я", _;
        past: _, _, _, _;
        imperative: "~ы~ь", "~ыте~ь";
        participle: _, _, _, _;
        gerund: "~а~ь", _;
    ),
    // Вылезти: 5 words (вылезти, выползти, повылезти, повыползти, выебти). Like Вынести, except in 1 cell: gerund past.
    conjugation!("вылезти", 5, "~ти", [aspect: Perfective], like Вынести;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: _, _;
        participle: _, _, _, _;
        gerund: _, "~ши";
    ),
    // Срастись: 5 words (разрастись, срастись, зарастись, врастись, растись).
    conjugation!("срастись", 5, "~а~ти~ь";
        infinitive: "~а~ти~ь";
        non_past_first: "~а~ту~ь", "~а~тём~я";
        non_past_second: "~а~тёшь~я", "~а~тёте~ь";
        non_past_third: "~а~тёт~я", "~а~тут~я";
        past: "~о~~я", "~о~ла~ь", "~о~ло~ь", "~о~ли~ь";
        imperative: "~а~ти~ь", "~а~тите~ь";
        participle: "~а~тущий~я", "~о~ший~я", derived, derived;
        gerund: "~а~тя~ь", "~о~ши~ь";
    ),
    // Изобрести: 4 words (изобрести, приобрести, обрести, соплести). Like Сплести, except in 2 cells: participle past_act, gerund past.
    conjugation!("изобрести", 4, "~ести", [aspect: Perfective], like Сплести;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: _, _;
        participle: _, "~етший", _, _;
        gerund: _, "~етя|~етши";
    ),
    // Принять: 4 words (принять, воспринять, предпринять, напринять). Like Занять, except in 8 cells: non_past_first sg, non_past_first pl, non_past_second sg, non_past_second pl, non_past_third sg, non_past_third pl, ….
    conjugation!("принять", 4, "~нять", [aspect: Perfective], like Занять;
        infinitive: _;
        non_past_first: "~му", "~мем";
        non_past_second: "~мешь", "~мете";
        non_past_third: "~мет", "~мут";
        past: _, _, _, _;
        imperative: "~ми", "~мите";
        participle: _, _, _, _;
        gerund: _, _;
    ),
    // Услышать: 4 words (услышать, вымолчать, выбрюзжать, дослышать). Like Заспать, except in 4 cells: non_past_first sg, non_past_third pl, imperative sg, imperative pl.
    conjugation!("услышать", 4, "~ать", [aspect: Perfective], like Заспать;
        infinitive: _;
        non_past_first: "~у", _;
        non_past_second: _, _;
        non_past_third: _, "~ат";
        past: _, _, _, _;
        imperative: "~ь", "~ьте";
        participle: _, _, _, _;
        gerund: _, _;
    ),
    // Выпереть: 4 words (вымереть, вытереть, выпереть, повымереть). Like Замереть, except in 5 cells: non_past_first pl, non_past_second sg, non_past_second pl, non_past_third sg, gerund past.
    conjugation!("выпереть", 4, "~е~еть", [aspect: Perfective], like Замереть;
        infinitive: _;
        non_past_first: _, "~~ем";
        non_past_second: "~~ешь", "~~ете";
        non_past_third: "~~ет", _;
        past: _, _, _, _;
        imperative: _, _;
        participle: _, _, _, _;
        gerund: _, "~е~ев|~е~ши";
    ),
    // Высечь: 4 words (высечь, вытечь, выволочь, выпечь). Like Сволочь, except in 6 cells: non_past_first pl, non_past_second sg, non_past_second pl, non_past_third sg, participle pres_act, participle past_pass.
    conjugation!("высечь", 4, "~чь", [aspect: Perfective], like Сволочь;
        infinitive: _;
        non_past_first: _, "~чем";
        non_past_second: "~чешь", "~чете";
        non_past_third: "~чет", _;
        past: _, _, _, _;
        imperative: _, _;
        participle: derived, _, _, "~ченный";
        gerund: _, _;
    ),
    // Напрячь: 4 words (запрячь, напрячь, распрячь, впрячь).
    conjugation!("напрячь", 4, "~ячь", [aspect: Perfective];
        infinitive: "~ячь";
        non_past_first: "~ягу", "~яжём";
        non_past_second: "~яжёшь", "~яжёте";
        non_past_third: "~яжёт", "~ягут";
        past: "~ёг|~яг", "~ягла", "~ягло", "~ягли";
        imperative: "~яги", "~ягите";
        participle: derived, "~ягший", derived, "~яжённый";
        gerund: derived, "~ягши";
    ),
    // Выдвинуть: 4 words (выдвинуть, высунуть, кашлянуть, выплюнуть). Like Подкинуть, except in 1 cell: imperative sg.
    conjugation!("выдвинуть", 4, "~уть", [aspect: Perfective], like Подкинуть;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: "~и|~ь", _;
        participle: _, _, _, _;
        gerund: _, _;
    ),
    // Внимать: 4 words (внимать, приимать, восприимать, предприимать).
    conjugation!("внимать", 4, "~и~ать", [aspect: Imperfective];
        infinitive: "~и~ать";
        non_past_first: "~е~лю|~е~лю|~и~аю", "~е~лем|~и~аем";
        non_past_second: "~е~лешь|~и~аешь", "~е~лете|~и~аете";
        non_past_third: "~е~лет|~и~ает", "~е~лют|~и~ают";
        past: "~и~ал", "~и~ала", "~и~ало", "~и~али";
        imperative: "~е~ли|~е~ли|~и~ай", "~е~лите|~е~лите|~и~айте";
        participle: "~е~лющий|~и~ающий", "~и~авший", "~е~лемый|~и~аемый", "~е~ленный";
        gerund: "~е~ля|~е~ля|~и~ая", "~и~ав|~и~авши";
    ),
    // Бечь: 4 words (бечь, убечь, избечь, побечь). Like Сберечь, except in 6 cells: non_past_first pl, non_past_second sg, non_past_second pl, non_past_third sg, participle pres_pass, participle past_pass.
    conjugation!("бечь", 4, "~ечь", like Сберечь;
        infinitive: _;
        non_past_first: _, "~егём";
        non_past_second: "~егёшь", "~егёте";
        non_past_third: "~егёт", _;
        past: _, _, _, _;
        imperative: _, _;
        participle: _, _, derived, derived;
        gerund: _, _;
    ),
    // Улечься: 4 words (улечься, поулечься, разлечься, приулечься).
    conjugation!("улечься", 4, "~ечься", [aspect: Perfective];
        infinitive: "~ечься";
        non_past_first: "~ягусь", "~яжемся";
        non_past_second: "~яжешься", "~яжетесь";
        non_past_third: "~яжется", "~ягутся";
        past: "~ёгся", "~еглась", "~еглось", "~еглись";
        imperative: "~ягся", "~ягтесь";
        participle: derived, "~ёгшийся", derived, derived;
        gerund: derived, "~ёгшись";
    ),
    // Вынестись: 4 words (вынестись, вывезтись, вытрястись, отверзтись). Like Трястись, except in 7 cells: non_past_first pl, non_past_second sg, non_past_second pl, non_past_third sg, participle pres_act, gerund pres, ….
    conjugation!("вынестись", 4, "~ти~ь", [aspect: Perfective], like Трястись;
        infinitive: _;
        non_past_first: _, "~ем~я";
        non_past_second: "~ешь~я", "~ете~ь";
        non_past_third: "~ет~я", _;
        past: _, _, _, _;
        imperative: _, _;
        participle: derived, _, _, _;
        gerund: derived, "~я~ь";
    ),
    // Истереть: 4 words (растереть, распереть, истереть, поистереть).
    conjugation!("истереть", 4, "~с~е~еть", [aspect: Perfective];
        infinitive: "~с~е~еть";
        non_past_first: "~зо~~у", "~зо~~ём";
        non_past_second: "~зо~~ёшь", "~зо~~ёте";
        non_past_third: "~зо~~ёт", "~зо~~ут";
        past: "~с~ё~", "~с~ё~ла", "~с~ё~ло", "~с~ё~ли";
        imperative: "~зо~~и", "~зо~~ите";
        participle: derived, "~с~ё~ший", derived, "~с~ё~тый";
        gerund: derived, "~с~е~ев|~с~ё~ши";
    ),
    // Испить: 4 words (расшить, испить, распить, исшить). Like Измыслить, except in 9 cells: non_past_first sg, non_past_first pl, non_past_second sg, non_past_second pl, non_past_third sg, non_past_third pl, ….
    conjugation!("испить", 4, "~с~ить", [aspect: Perfective], like Измыслить;
        infinitive: _;
        non_past_first: "~зо~ью", "~зо~ьём";
        non_past_second: "~зо~ьёшь", "~зо~ьёте";
        non_past_third: "~зо~ьёт", "~зо~ьют";
        past: _, _, _, _;
        imperative: "~с~ей", "~с~ейте";
        participle: _, _, _, "~с~итый";
        gerund: _, _;
    ),
    // Уеть: 4 words (выеть, поеть, заеть, уеть). Like Грести, except in 4 cells: infinitive, participle pres_act, participle pres_pass, gerund pres.
    conjugation!("уеть", 4, "~еть", [aspect: Perfective], like Грести;
        infinitive: "~еть";
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: _, _;
        participle: derived, _, derived, _;
        gerund: derived, _;
    ),
    // Доебсти: 4 words (уебсти, выебсти, поебсти, доебсти). Like Снести, except in 2 cells: infinitive, gerund past.
    conjugation!("доебсти", 4, "~е~сти", [aspect: Perfective], like Снести;
        infinitive: "~е~сти";
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: _, _;
        participle: _, _, _, _;
        gerund: _, "~ё~ши";
    ),
    // Поколебать: 4 words (колебать, поколебать, зыбать, выщипать). Like Щипать, except in 2 cells: participle pres_pass, participle past_pass.
    conjugation!("поколебать", 4, "~ать", like Щипать;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: _, _;
        participle: _, _, "~лемый", "~ленный";
        gerund: _, _;
    ),
    // Насочинять: 4 words (насочинять, обрусять, постеснять, оязыковлять). Like Выровнять, except in 4 cells: participle pres_act, participle pres_pass, participle past_pass, gerund pres.
    conjugation!("насочинять", 4, "~ять", like Выровнять;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: _, _;
        participle: "~яющий", _, "~яемый", "~ённый";
        gerund: "~яя", _;
    ),
    // Растереться: 4 words (растереться, истереться, поистереться, распереться).
    conjugation!("растереться", 4, "~с~е~еть~я", [aspect: Perfective];
        infinitive: "~с~е~еть~я";
        non_past_first: "~зо~~у~ь", "~зо~~ём~я";
        non_past_second: "~зо~~ёшь~я", "~зо~~ёте~ь";
        non_past_third: "~зо~~ёт~я", "~зо~~ут~я";
        past: "~с~ё~~я", "~с~ё~ла~ь", "~с~ё~ло~ь", "~с~ё~ли~ь";
        imperative: "~зо~~и~ь", "~зо~~ите~ь";
        participle: derived, "~с~ё~ший~я", derived, derived;
        gerund: derived, "~с~ё~ши~ь";
    ),
    // Изъяться: 4 words (изъяться, подъяться, разъяться, отъяться).
    conjugation!("изъяться", 4, "~ъять~я", [aspect: Perfective];
        infinitive: "~ъять~я";
        non_past_first: "~ыму~ь", "~ымем~я";
        non_past_second: "~ымешь~я", "~ымете~ь";
        non_past_third: "~ымет~я", "~ымут~я";
        past: "~ъял~я", "~ъяла~ь", "~ъяло~ь", "~ъяли~ь";
        imperative: "~ыми~ь", "~ымите~ь";
        participle: derived, "~ъявший~я", derived, derived;
        gerund: derived, "~ъявши~ь";
    ),
    // Помочь: 3 words (выстричь, выпрячь, вылечь). Like Постричь, except in 5 cells: non_past_first pl, non_past_second sg, non_past_second pl, non_past_third sg, participle pres_act.
    conjugation!("помочь", 3, "~чь", [aspect: Perfective], like Постричь;
        infinitive: _;
        non_past_first: _, "~жем";
        non_past_second: "~жешь", "~жете";
        non_past_third: "~жет", _;
        past: _, _, _, _;
        imperative: _, _;
        participle: derived, _, _, _;
        gerund: _, _;
    ),
    // Хотеться: 3 words (хотеться, захотеться, расхотеться). Like Разлететься, except in 4 cells: non_past_second sg, non_past_third sg, participle pres_act, gerund pres.
    conjugation!("хотеться", 3, "~теть~я", like Разлететься;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: "~чешь~я", _;
        non_past_third: "~чет~я", _;
        past: _, _, _, _;
        imperative: _, _;
        participle: derived, _, _, _;
        gerund: derived, _;
    ),
    // Вырасти: 3 words (вырасти, повырасти, подвырасти). Like Дорасти, except in 6 cells: non_past_first pl, non_past_second sg, non_past_second pl, non_past_third sg, participle pres_act, gerund pres.
    conjugation!("вырасти", 3, "~а~ти", [aspect: Perfective], like Дорасти;
        infinitive: _;
        non_past_first: _, "~а~тем";
        non_past_second: "~а~тешь", "~а~тете";
        non_past_third: "~а~тет", _;
        past: _, _, _, _;
        imperative: _, _;
        participle: derived, _, _, _;
        gerund: derived, _;
    ),
    // Высвободить: 3 words (вынудить, высвободить, выродить). Like Разводить, except in 4 cells: participle pres_act, participle pres_pass, participle past_pass, gerund pres.
    conjugation!("высвободить", 3, "~дить", [aspect: Perfective], like Разводить;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: _, _;
        participle: derived, _, derived, "~жденный";
        gerund: derived, _;
    ),
    // Начаться: 3 words (начаться, початься, зачаться). Like Выспаться, except in 8 cells: non_past_first sg, non_past_first pl, non_past_second sg, non_past_second pl, non_past_third sg, non_past_third pl, ….
    conjugation!("начаться", 3, "~ать~я", [aspect: Perfective], like Выспаться;
        infinitive: _;
        non_past_first: "~ну~ь", "~нём~я";
        non_past_second: "~нёшь~я", "~нёте~ь";
        non_past_third: "~нёт~я", "~нут~я";
        past: _, _, _, _;
        imperative: "~ни~ь", "~ните~ь";
        participle: _, _, _, _;
        gerund: _, _;
    ),
    // Тереть: 3 words (тереть, мереть, переть). Like Затереть, except in 2 cells: participle pres_act, gerund past.
    conjugation!("тереть", 3, "~е~еть", [aspect: Imperfective], like Затереть;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: _, _;
        participle: "~~ущий", _, _, _;
        gerund: _, "~ё~ши";
    ),
    // Опоясать: 3 words (опоясать, запоясать, распоясать). Like Выписать, except in 4 cells: imperative sg, imperative pl, participle pres_act, gerund pres.
    conjugation!("опоясать", 3, "~сать", [aspect: Perfective], like Выписать;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: "~шь", "~шьте";
        participle: derived, _, _, _;
        gerund: derived, _;
    ),
    // Вянуть: 3 words (вянуть, завянуть, увянуть). Like Стынуть, except in 2 cells: participle past_act, gerund past.
    conjugation!("вянуть", 3, "~нуть", like Стынуть;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: _, _;
        participle: _, "~нувший", _, _;
        gerund: _, "~нув|~нувши";
    ),
    // Заклясть: 3 words (проклясть, клясть, заклясть). Like Помять, except in 9 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("заклясть", 3, "~сть", like Помять;
        infinitive: "~сть";
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: "~л", "~ла", "~ло", "~ли";
        imperative: _, _;
        participle: _, "~вший", "~нимый", "~тый";
        gerund: _, "~в|~вши";
    ),
    // Клясться: 3 words (клясться, поклясться, заклясться). Like Мяться, except in 7 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("клясться", 3, "~~ться", like Мяться;
        infinitive: "~~ться";
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: "~л~я|~л~я", "~ла~ь", "~ло~ь|~ло~ь", "~ли~ь|~ли~ь";
        imperative: _, _;
        participle: _, "~вший~я", _, _;
        gerund: _, "~вши~ь";
    ),
    // Приняться: 3 words (приняться, восприняться, предприняться). Like Уняться, except in 9 cells: non_past_first sg, non_past_first pl, non_past_second sg, non_past_second pl, non_past_third sg, non_past_third pl, ….
    conjugation!("приняться", 3, "~нять~я", [aspect: Perfective], like Уняться;
        infinitive: _;
        non_past_first: "~му~ь", "~мем~я";
        non_past_second: "~мешь~я", "~мете~ь";
        non_past_third: "~мет~я", "~мут~я";
        past: "~нял~я", _, _, _;
        imperative: "~ми~ь", "~мите~ь";
        participle: _, _, _, _;
        gerund: _, _;
    ),
    // Вывести: 3 words (вывести, выбрести, повывести). Like Выпасть, except in 3 cells: infinitive, participle past_act, gerund past.
    conjugation!("вывести", 3, "~сти", [aspect: Perfective], like Выпасть;
        infinitive: "~сти";
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: _, _;
        participle: _, "~дший", _, _;
        gerund: _, "~дя";
    ),
    // Выбраться: 3 words (выбраться, выдраться, повыбраться). Like Стлаться, except in 4 cells: non_past_first sg, non_past_third pl, participle pres_act, gerund pres.
    conjugation!("выбраться", 3, "~~ать~я", [aspect: Perfective], like Стлаться;
        infinitive: _;
        non_past_first: "~е~у~ь", _;
        non_past_second: _, _;
        non_past_third: _, "~е~ут~я";
        past: _, _, _, _;
        imperative: _, _;
        participle: derived, _, _, _;
        gerund: derived, _;
    ),
    // Счесть: 3 words (счесть, обчесть, отчесть).
    conjugation!("счесть", 3, "~~есть", [aspect: Perfective];
        infinitive: "~~есть";
        non_past_first: "~о~ту", "~о~тём";
        non_past_second: "~о~тёшь", "~о~тёте";
        non_past_third: "~о~тёт", "~о~тут";
        past: "~~ёл", "~о~ла", "~о~ло", "~о~ли";
        imperative: "~о~ти", "~о~тите";
        participle: derived, derived, derived, "~о~тённый";
        gerund: derived, "~о~тя|~~ётши";
    ),
    // Обмереть: 3 words (отпереть, обмереть, отмереть). Like Стереть, except in 7 cells: past m, past f, past n, past pl, participle past_act, participle past_pass, ….
    conjugation!("обмереть", 3, "~~е~еть", [aspect: Perfective], like Стереть;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: "~~е~", "~~е~ла", "~~е~ло", "~~е~ли";
        imperative: _, _;
        participle: _, "~~е~ший|~~е~ший", _, "~~е~тый";
        gerund: _, "~~е~ев|~~е~ши|~~е~ши";
    ),
    // Понудить: 3 words (принудить, понудить, попринудить). Like Высвободить, except in 3 cells: imperative sg, imperative pl, participle past_pass.
    conjugation!("понудить", 3, "~дить", [aspect: Perfective], like Высвободить;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: "~дь", "~дьте";
        participle: _, _, _, "~ждённый";
        gerund: _, _;
    ),
    // Отозвать: 3 words (отозвать, обозвать, подозвать). Like Разогнать, except in 6 cells: non_past_first sg, non_past_first pl, non_past_second sg, non_past_second pl, non_past_third sg, non_past_third pl.
    conjugation!("отозвать", 3, "~о~~ать", [aspect: Perfective], like Разогнать;
        infinitive: _;
        non_past_first: "~~о~у", "~~о~ём";
        non_past_second: "~~о~ёшь", "~~о~ёте";
        non_past_third: "~~о~ёт", "~~о~ут";
        past: _, _, _, _;
        imperative: _, _;
        participle: _, _, _, _;
        gerund: _, _;
    ),
    // Помутить: 3 words (замутить, помутить, намутить). Like Зачертить, except in 1 cell: participle past_pass.
    conjugation!("помутить", 3, "~тить", [aspect: Perfective], like Зачертить;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: _, _;
        participle: _, _, _, "~тнённый";
        gerund: _, _;
    ),
    // Обглодать: 3 words (глодать, обглодать, сглодать).
    conjugation!("обглодать", 3, "~дать";
        infinitive: "~дать";
        non_past_first: "~жу|~даю", "~жем|~даем";
        non_past_second: "~жешь|~даешь", "~жете|~даете";
        non_past_third: "~жет|~дает", "~жут|~дают";
        past: "~дал", "~дала", "~дало", "~дали";
        imperative: "~жи|~дай", "~жите|~дайте";
        participle: "~жущий", "~давший", "~даемый", "~данный";
        gerund: "~жа", "~дав|~давши";
    ),
    // Пролежать: 3 words (пролежать, отлежать, облежать). Like Взъебать, except in 8 cells: non_past_first pl, non_past_second sg, non_past_second pl, non_past_third sg, non_past_third pl, participle pres_act, ….
    conjugation!("пролежать", 3, "~е~ать", like Взъебать;
        infinitive: _;
        non_past_first: _, "~е~им";
        non_past_second: "~е~ишь", "~е~ите";
        non_past_third: "~е~ит", "~е~ат";
        past: _, _, _, _;
        imperative: _, _;
        participle: "~е~ащий", _, "~е~имый", _;
        gerund: "~е~а", _;
    ),
    // Осечься: 3 words (пересечься, осечься, сечься). Like Влечься, except in 3 cells: past m, participle past_act, gerund past.
    conjugation!("осечься", 3, "~ечь~я", like Влечься;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: "~ёк~я|~ек~я", _, _, _;
        imperative: _, _;
        participle: _, "~екший~я|~ёкший~я", _, _;
        gerund: _, "~екши~ь|~ёкши~ь";
    ),
    // Наморщиться: 3 words (встопорщиться, наморщиться, изморщиться). Like Выпариться, except in 2 cells: non_past_first sg, gerund past.
    conjugation!("наморщиться", 3, "~ить~я", [aspect: Perfective], like Выпариться;
        infinitive: _;
        non_past_first: "~у~ь", _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: _, _;
        participle: _, _, _, _;
        gerund: _, "~ивши~ь";
    ),
    // Чтиться: 3 words (чтиться, почтиться, узриться). Like Числиться, except in 2 cells: non_past_first sg, gerund past.
    conjugation!("чтиться", 3, "~ить~я", like Числиться;
        infinitive: _;
        non_past_first: "~у~ь", _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: _, _;
        participle: _, _, _, _;
        gerund: _, "~ивши~ь|~я~ь";
    ),
    // Взбресть: 3 words (взбресть, добресть, набресть). Like Развести, except in 3 cells: infinitive, participle past_pass, gerund past.
    conjugation!("взбресть", 3, "~есть", [aspect: Perfective], like Развести;
        infinitive: "~есть";
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: _, _;
        participle: _, _, _, derived;
        gerund: _, "~едя";
    ),
    // Выпечься: 3 words (выволочься, выпечься, высечься). Like Заволочься, except in 5 cells: non_past_first pl, non_past_second sg, non_past_second pl, non_past_third sg, participle pres_act.
    conjugation!("выпечься", 3, "~чь~я", [aspect: Perfective], like Заволочься;
        infinitive: _;
        non_past_first: _, "~чем~я";
        non_past_second: "~чешь~я", "~чете~ь";
        non_past_third: "~чет~я", _;
        past: _, _, _, _;
        imperative: _, _;
        participle: derived, _, _, _;
        gerund: _, _;
    ),
    // Испиться: 3 words (распиться, расшиться, испиться).
    conjugation!("испиться", 3, "~с~ить~я", [aspect: Perfective];
        infinitive: "~с~ить~я";
        non_past_first: "~зо~ью~ь", "~зо~ьём~я";
        non_past_second: "~зо~ьёшь~я", "~зо~ьёте~ь";
        non_past_third: "~зо~ьёт~я", "~зо~ьют~я";
        past: "~с~ил~я", "~с~ила~ь", "~с~ило~ь", "~с~или~ь";
        imperative: "~с~ей~я", "~с~ейте~ь";
        participle: derived, "~с~ивший~я", derived, derived;
        gerund: derived, "~с~ивши~ь";
    ),
    // Измождать: 3 words (измождать, повозвращать, прокажать).
    conjugation!("измождать", 3, "~ать";
        infinitive: "~ать";
        non_past_first: "~аю", "~аем";
        non_past_second: "~аешь", "~аете";
        non_past_third: "~ает", "~ают";
        past: "~ал", "~ала", "~ало", "~али";
        imperative: "~ай", "~айте";
        participle: "~ающий", "~авший", "~аемый", "~ённый";
        gerund: "~ая", "~ав|~авши";
    ),
    // Позаводить: 3 words (самовоспроизводить, напереводить, позаводить).
    conjugation!("позаводить", 3, "~одить", [aspect: Perfective];
        infinitive: "~одить";
        non_past_first: "~ожу", "~одим";
        non_past_second: "~одишь", "~одите";
        non_past_third: "~одит", "~одят";
        past: "~одил", "~одила", "~одило", "~одили";
        imperative: "~оди", "~одите";
        participle: derived, "~одивший", derived, "~едённый";
        gerund: derived, "~одив|~одивши";
    ),
    // Закодить: 3 words (повыводить, закодить, накодить). Like Высвободить, except in 1 cell: participle past_pass.
    conjugation!("закодить", 3, "~дить", [aspect: Perfective], like Высвободить;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: _, _;
        participle: _, _, _, "~денный";
        gerund: _, _;
    ),
    // Харассить: 3 words (оглонассить, харассить, глонассить).
    conjugation!("харассить", 3, "~ссить";
        infinitive: "~ссить";
        non_past_first: "~шу", "~ссим";
        non_past_second: "~ссишь", "~ссите";
        non_past_third: "~ссит", "~ссят";
        past: "~ссил", "~ссила", "~ссило", "~ссили";
        imperative: "~ссь", "~ссьте";
        participle: "~ссящий", "~ссивший", "~ссимый", "~шенный";
        gerund: "~сся", "~ссив|~ссивши";
    ),
    // Засыпать: 2 words (сыпать, высыпать). Like Щипать, except in 7 cells: non_past_first pl, non_past_second sg, non_past_second pl, non_past_third sg, imperative sg, imperative pl, ….
    conjugation!("засыпать", 2, "~ать", like Щипать;
        infinitive: _;
        non_past_first: _, "~лем|~ем";
        non_past_second: "~лешь|~ешь", "~лете|~ете";
        non_past_third: "~лет|~ет", _;
        past: _, _, _, _;
        imperative: "~ь", "~ьте";
        participle: _, _, "~лемый", _;
        gerund: _, _;
    ),
    // Сопрячь: 2 words (запречь, отпречь). Like Постричь, except in 2 cells: participle pres_act, participle past_pass.
    conjugation!("сопрячь", 2, "~чь", like Постричь;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: _, _;
        participle: derived, _, _, "~жённый";
        gerund: _, _;
    ),
    // Реветь: 2 words (реветь, простреть). Like Засосать, except in 8 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("реветь", 2, "~еть", like Засосать;
        infinitive: "~еть";
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: "~ел", "~ела", "~ело", "~ели";
        imperative: _, _;
        participle: _, "~евший", _, derived;
        gerund: _, "~ев|~евши";
    ),
    // Лгать: 2 words (оболгать, перелгать). Like Сопрячь, except in 8 cells: infinitive, past m, past f, past n, past pl, participle past_act, ….
    conjugation!("лгать", 2, "~гать", like Сопрячь;
        infinitive: "~гать";
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: "~гал", "~гала", "~гало", "~гали";
        imperative: _, _;
        participle: _, "~гавший", _, "~ганный";
        gerund: _, "~гав|~гавши";
    ),
    // Ехать: 2 words (ехать, повыехать). Like Поехать, except in 4 cells: imperative sg, imperative pl, participle pres_act, gerund pres.
    conjugation!("ехать", 2, "~хать", like Поехать;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: "по~зжай", "по~зжайте";
        participle: "~дущий", _, _, _;
        gerund: "~дучи", _;
    ),
    // Резать: 1 words (вынизать). Like Оказать, except in 2 cells: imperative sg, imperative pl.
    conjugation!("резать", 1, "~зать", like Оказать;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: "~жь", "~жьте";
        participle: _, _, _, _;
        gerund: _, _;
    ),
    // Застыть: 1 words (стыть). Like Стынуть, except in 1 cell: infinitive.
    conjugation!("застыть", 1, "~ть", like Стынуть;
        infinitive: "~ть";
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: _, _;
        participle: _, _, _, _;
        gerund: _, _;
    ),
    // Упрятать: 1 words (выбормотать). Like Топтать, except in 2 cells: imperative sg, imperative pl.
    conjugation!("упрятать", 1, "~тать", like Топтать;
        infinitive: _;
        non_past_first: _, _;
        non_past_second: _, _;
        non_past_third: _, _;
        past: _, _, _, _;
        imperative: "~чь", "~чьте";
        participle: _, _, _, _;
        gerund: _, _;
    ),
    // Рещи: 1 words (рещи). Like Обтечь, except in 7 cells: infinitive, non_past_first sg, non_past_first pl, non_past_second sg, non_past_second pl, non_past_third sg, ….
    conjugation!("рещи", 1, "~ещи", like Обтечь;
        infinitive: "~ещи";
        non_past_first: derived, derived;
        non_past_second: derived, derived;
        non_past_third: derived, derived;
        past: _, _, _, _;
        imperative: _, _;
        participle: _, _, _, _;
        gerund: _, _;
    ),
];
