//! The conjugations, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one conjugation: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's feature says):
/// `infinitive`
/// `non_past_first` (sg, pl)
/// `non_past_second` (sg, pl)
/// `non_past_third` (sg, pl)
/// `past` (m, f, n, pl)
/// `imperative` (sg, pl)
/// `participle` (pres_act, past_act, pres_pass, past_pass)
/// `gerund` (pres, past)
macro_rules! conjugation {
    ($name:literal, $words:literal, $lemma:literal;
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
            ]),
        }
    };
}

/// Every conjugation learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Conjugation {
    /// Like `играть`: 11,621 words.
    Играть,
    /// Like `набираться`: 8,289 words.
    Набираться,
    /// Like `вибрировать`: 4,162 words.
    Вибрировать,
    /// Like `стыковаться`: 2,209 words.
    Стыковаться,
    /// Like `числиться`: 1,171 words.
    Числиться,
    /// Like `хуярить`: 1,129 words.
    Хуярить,
    /// Like `палить`: 1,053 words.
    Палить,
    /// Like `раздуть`: 928 words.
    Раздуть,
    /// Like `рвануть`: 679 words.
    Рвануть,
    /// Like `предназначить`: 619 words.
    Предназначить,
    /// Like `усилиться`: 566 words.
    Усилиться,
    /// Like `тянуть`: 548 words.
    Тянуть,
    /// Like `выделить`: 520 words.
    Выделить,
    /// Like `обнажиться`: 501 words.
    Обнажиться,
    /// Like `прицепиться`: 454 words.
    Прицепиться,
    /// Like `обернуться`: 391 words.
    Обернуться,
    /// Like `служить`: 358 words.
    Служить,
    /// Like `обрушиться`: 343 words.
    Обрушиться,
    /// Like `восстановить`: 329 words.
    Восстановить,
    /// Like `разводить`: 327 words.
    Разводить,
    /// Like `долбить`: 324 words.
    Долбить,
    /// Like `расправиться`: 296 words.
    Расправиться,
    /// Like `хватить`: 296 words.
    Хватить,
    /// Like `представить`: 290 words.
    Представить,
    /// Like `проскочить`: 287 words.
    Проскочить,
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
    /// Like `распуститься`: 163 words.
    Распуститься,
    /// Like `отметить`: 153 words.
    Отметить,
    /// Like `стучать`: 152 words.
    Стучать,
    /// Like `возместить`: 152 words.
    Возместить,
    /// Like `носиться`: 138 words.
    Носиться,
    /// Like `забросить`: 125 words.
    Забросить,
    /// Like `встретиться`: 120 words.
    Встретиться,
    /// Like `засосать`: 118 words.
    Засосать,
    /// Like `развеять`: 113 words.
    Развеять,
    /// Like `перестроиться`: 110 words.
    Перестроиться,
    /// Like `держаться`: 108 words.
    Держаться,
    /// Like `топтать`: 107 words.
    Топтать,
    /// Like `сорваться`: 103 words.
    Сорваться,
    /// Like `вывозить`: 102 words.
    Вывозить,
    /// Like `трафить`: 102 words.
    Трафить,
    /// Like `сычевать`: 101 words.
    Сычевать,
    /// Like `наладиться`: 95 words.
    Наладиться,
    /// Like `переброситься`: 93 words.
    Переброситься,
    /// Like `кишеть`: 91 words.
    Кишеть,
    /// Like `елозить`: 90 words.
    Елозить,
    /// Like `стоить`: 89 words.
    Стоить,
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
    /// Like `оказать`: 79 words.
    Оказать,
    /// Like `отдавать`: 77 words.
    Отдавать,
    /// Like `отпустить`: 75 words.
    Отпустить,
    /// Like `развеяться`: 73 words.
    Развеяться,
    /// Like `коптить`: 72 words.
    Коптить,
    /// Like `бухтеть`: 72 words.
    Бухтеть,
    /// Like `стушеваться`: 71 words.
    Стушеваться,
    /// Like `отвоевать`: 70 words.
    Отвоевать,
    /// Like `забить`: 68 words.
    Забить,
    /// Like `сблизиться`: 66 words.
    Сблизиться,
    /// Like `сидеть`: 66 words.
    Сидеть,
    /// Like `клепать`: 60 words.
    Клепать,
    /// Like `обтечь`: 60 words.
    Обтечь,
    /// Like `возбудить`: 59 words.
    Возбудить,
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
    /// Like `стать`: 53 words.
    Стать,
    /// Like `влечься`: 53 words.
    Влечься,
    /// Like `чесаться`: 53 words.
    Чесаться,
    /// Like `отказаться`: 52 words.
    Отказаться,
    /// Like `пасть`: 52 words.
    Пасть,
    /// Like `отдаваться`: 51 words.
    Отдаваться,
    /// Like `глядеться`: 48 words.
    Глядеться,
    /// Like `разбить`: 46 words.
    Разбить,
    /// Like `снести`: 45 words.
    Снести,
    /// Like `трещать`: 43 words.
    Трещать,
    /// Like `развести`: 42 words.
    Развести,
    /// Like `колоться`: 42 words.
    Колоться,
    /// Like `топтаться`: 42 words.
    Топтаться,
    /// Like `драть`: 42 words.
    Драть,
    /// Like `сдать`: 41 words.
    Сдать,
    /// Like `резать`: 40 words.
    Резать,
    /// Like `выписать`: 39 words.
    Выписать,
    /// Like `отобразить`: 39 words.
    Отобразить,
    /// Like `сбиться`: 39 words.
    Сбиться,
    /// Like `защитить`: 39 words.
    Защитить,
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
    /// Like `расчесать`: 34 words.
    Расчесать,
    /// Like `пыхать`: 34 words.
    Пыхать,
    /// Like `клевать`: 33 words.
    Клевать,
    /// Like `станцевать`: 33 words.
    Станцевать,
    /// Like `сплести`: 32 words.
    Сплести,
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
    /// Like `виснуть`: 25 words.
    Виснуть,
    /// Like `разлететься`: 24 words.
    Разлететься,
    /// Like `расплыться`: 24 words.
    Расплыться,
    /// Like `вторгнуться`: 24 words.
    Вторгнуться,
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
    /// Like `щипать`: 21 words.
    Щипать,
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
    /// Like `отнять`: 19 words.
    Отнять,
    /// Like `лечь`: 19 words.
    Лечь,
    /// Like `плескаться`: 19 words.
    Плескаться,
    /// Like `просмотреть`: 18 words.
    Просмотреть,
    /// Like `сволочь`: 18 words.
    Сволочь,
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
    /// Like `уебать`: 17 words.
    Уебать,
    /// Like `колыхаться`: 17 words.
    Колыхаться,
    /// Like `поехать`: 16 words.
    Поехать,
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
    /// Like `напеть`: 15 words.
    Напеть,
    /// Like `видеть`: 15 words.
    Видеть,
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
    /// Like `прожечь`: 14 words.
    Прожечь,
    /// Like `нести`: 14 words.
    Нести,
    /// Like `молоть`: 14 words.
    Молоть,
    /// Like `засыпать`: 14 words.
    Засыпать,
    /// Like `застыть`: 14 words.
    Застыть,
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
    /// Like `помочь`: 13 words.
    Помочь,
    /// Like `толочь`: 13 words.
    Толочь,
    /// Like `сопрячь`: 13 words.
    Сопрячь,
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
    /// Like `изобрести`: 12 words.
    Изобрести,
    /// Like `уняться`: 12 words.
    Уняться,
    /// Like `жевать`: 12 words.
    Жевать,
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
    /// Like `насытить`: 12 words.
    Насытить,
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
    /// Like `ссать`: 11 words.
    Ссать,
    /// Like `стоять`: 10 words.
    Стоять,
    /// Like `беречься`: 10 words.
    Беречься,
    /// Like `упрятать`: 10 words.
    Упрятать,
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
    /// Like `высрать`: 9 words.
    Высрать,
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
    /// Like `почать`: 7 words.
    Почать,
    /// Like `нашептать`: 7 words.
    Нашептать,
    /// Like `прятаться`: 7 words.
    Прятаться,
    /// Like `реветь`: 7 words.
    Реветь,
    /// Like `закупорить`: 7 words.
    Закупорить,
    /// Like `ебсти`: 7 words.
    Ебсти,
    /// Like `залгаться`: 7 words.
    Залгаться,
    /// Like `лгать`: 7 words.
    Лгать,
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
    /// Like `мести`: 5 words.
    Мести,
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
    /// Like `изъять`: 5 words.
    Изъять,
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
    /// Like `бухвостить`: 5 words.
    Бухвостить,
    /// Like `вылиться`: 5 words.
    Вылиться,
    /// Like `трепетаться`: 5 words.
    Трепетаться,
    /// Like `уссаться`: 5 words.
    Уссаться,
    /// Like `вылезти`: 5 words.
    Вылезти,
    /// Like `срастись`: 5 words.
    Срастись,
    /// Like `ехать`: 4 words.
    Ехать,
    /// Like `цвести`: 4 words.
    Цвести,
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
    /// Like `выебсти`: 4 words.
    Выебсти,
    /// Like `поколебать`: 4 words.
    Поколебать,
    /// Like `насочинять`: 4 words.
    Насочинять,
    /// Like `растереться`: 4 words.
    Растереться,
    /// Like `изъяться`: 4 words.
    Изъяться,
    /// Like `хотеться`: 3 words.
    Хотеться,
    /// Like `вырасти`: 3 words.
    Вырасти,
    /// Like `высвободить`: 3 words.
    Высвободить,
    /// Like `начаться`: 3 words.
    Начаться,
    /// Like `переть`: 3 words.
    Переть,
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
    pub const ALL: [Conjugation; 348] = [
        Conjugation::Играть,
        Conjugation::Набираться,
        Conjugation::Вибрировать,
        Conjugation::Стыковаться,
        Conjugation::Числиться,
        Conjugation::Хуярить,
        Conjugation::Палить,
        Conjugation::Раздуть,
        Conjugation::Рвануть,
        Conjugation::Предназначить,
        Conjugation::Усилиться,
        Conjugation::Тянуть,
        Conjugation::Выделить,
        Conjugation::Обнажиться,
        Conjugation::Прицепиться,
        Conjugation::Обернуться,
        Conjugation::Служить,
        Conjugation::Обрушиться,
        Conjugation::Восстановить,
        Conjugation::Разводить,
        Conjugation::Долбить,
        Conjugation::Расправиться,
        Conjugation::Хватить,
        Conjugation::Представить,
        Conjugation::Проскочить,
        Conjugation::Расходиться,
        Conjugation::Закатиться,
        Conjugation::Тянуться,
        Conjugation::Залипнуть,
        Conjugation::Сносить,
        Conjugation::Распуститься,
        Conjugation::Отметить,
        Conjugation::Стучать,
        Conjugation::Возместить,
        Conjugation::Носиться,
        Conjugation::Забросить,
        Conjugation::Встретиться,
        Conjugation::Засосать,
        Conjugation::Развеять,
        Conjugation::Перестроиться,
        Conjugation::Держаться,
        Conjugation::Топтать,
        Conjugation::Сорваться,
        Conjugation::Вывозить,
        Conjugation::Трафить,
        Conjugation::Сычевать,
        Conjugation::Наладиться,
        Conjugation::Переброситься,
        Conjugation::Кишеть,
        Conjugation::Елозить,
        Conjugation::Стоить,
        Conjugation::Подкинуть,
        Conjugation::Открыть,
        Conjugation::Наладить,
        Conjugation::Выразиться,
        Conjugation::Зарядить,
        Conjugation::Оказать,
        Conjugation::Отдавать,
        Conjugation::Отпустить,
        Conjugation::Развеяться,
        Conjugation::Коптить,
        Conjugation::Бухтеть,
        Conjugation::Стушеваться,
        Conjugation::Отвоевать,
        Conjugation::Забить,
        Conjugation::Сблизиться,
        Conjugation::Сидеть,
        Conjugation::Клепать,
        Conjugation::Обтечь,
        Conjugation::Возбудить,
        Conjugation::Виться,
        Conjugation::Перевернуть,
        Conjugation::Сунуться,
        Conjugation::Раскрыться,
        Conjugation::Колоть,
        Conjugation::Заглохнуть,
        Conjugation::Идти,
        Conjugation::Стать,
        Conjugation::Влечься,
        Conjugation::Чесаться,
        Conjugation::Отказаться,
        Conjugation::Пасть,
        Conjugation::Отдаваться,
        Conjugation::Глядеться,
        Conjugation::Разбить,
        Conjugation::Снести,
        Conjugation::Трещать,
        Conjugation::Развести,
        Conjugation::Колоться,
        Conjugation::Топтаться,
        Conjugation::Драть,
        Conjugation::Сдать,
        Conjugation::Резать,
        Conjugation::Выписать,
        Conjugation::Отобразить,
        Conjugation::Сбиться,
        Conjugation::Защитить,
        Conjugation::Обратиться,
        Conjugation::Трястись,
        Conjugation::Загореться,
        Conjugation::Гореть,
        Conjugation::Трепаться,
        Conjugation::Расчесать,
        Conjugation::Пыхать,
        Conjugation::Клевать,
        Conjugation::Станцевать,
        Conjugation::Сплести,
        Conjugation::Резаться,
        Conjugation::Отвоеваться,
        Conjugation::Обыскать,
        Conjugation::Сковать,
        Conjugation::Грести,
        Conjugation::Искусить,
        Conjugation::Распасться,
        Conjugation::Вывестись,
        Conjugation::Зайти,
        Conjugation::Набраться,
        Conjugation::Плакать,
        Conjugation::Просвистеть,
        Conjugation::Упереться,
        Conjugation::Терпеться,
        Conjugation::Плеваться,
        Conjugation::Стричься,
        Conjugation::Наплыть,
        Conjugation::Виснуть,
        Conjugation::Разлететься,
        Conjugation::Расплыться,
        Conjugation::Вторгнуться,
        Conjugation::Заесть,
        Conjugation::Затереть,
        Conjugation::Отдаться,
        Conjugation::Дорасти,
        Conjugation::Забыть,
        Conjugation::Унестись,
        Conjugation::Щипать,
        Conjugation::Набежать,
        Conjugation::Расстаться,
        Conjugation::Грызть,
        Conjugation::Лезть,
        Conjugation::Зашибить,
        Conjugation::Занять,
        Conjugation::Отнять,
        Conjugation::Лечь,
        Conjugation::Плескаться,
        Conjugation::Просмотреть,
        Conjugation::Сволочь,
        Conjugation::Потчевать,
        Conjugation::Въесться,
        Conjugation::Устояться,
        Conjugation::Трепетать,
        Conjugation::Жать,
        Conjugation::Вгрызться,
        Conjugation::Уебать,
        Conjugation::Колыхаться,
        Conjugation::Поехать,
        Conjugation::Разобрать,
        Conjugation::Выровнять,
        Conjugation::Трепать,
        Conjugation::Заволочься,
        Conjugation::Честь,
        Conjugation::Насесть,
        Conjugation::Напеть,
        Conjugation::Видеть,
        Conjugation::Смеяться,
        Conjugation::Гнать,
        Conjugation::Постричь,
        Conjugation::Помять,
        Conjugation::Зиждить,
        Conjugation::Гнить,
        Conjugation::Сберечь,
        Conjugation::Прожечь,
        Conjugation::Нести,
        Conjugation::Молоть,
        Conjugation::Засыпать,
        Conjugation::Застыть,
        Conjugation::Прискакать,
        Conjugation::Молоться,
        Conjugation::Просвистать,
        Conjugation::Глохнуть,
        Conjugation::Подобраться,
        Conjugation::Засечь,
        Conjugation::Помочь,
        Conjugation::Толочь,
        Conjugation::Сопрячь,
        Conjugation::Сплестись,
        Conjugation::Тыкаться,
        Conjugation::Мяться,
        Conjugation::Отгрестись,
        Conjugation::Нагореть,
        Conjugation::Вызвать,
        Conjugation::Подняться,
        Conjugation::Отослать,
        Conjugation::Брить,
        Conjugation::Изобрести,
        Conjugation::Уняться,
        Conjugation::Жевать,
        Conjugation::Стынуть,
        Conjugation::Бриться,
        Conjugation::Морщить,
        Conjugation::Сослаться,
        Conjugation::Пересидеть,
        Conjugation::Спеться,
        Conjugation::Стереться,
        Conjugation::Стелиться,
        Conjugation::Подвигнуться,
        Conjugation::Насытить,
        Conjugation::Дожить,
        Conjugation::Ушибиться,
        Conjugation::Зачесться,
        Conjugation::Жаться,
        Conjugation::Пакоститься,
        Conjugation::Ссать,
        Conjugation::Стоять,
        Conjugation::Беречься,
        Conjugation::Упрятать,
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
        Conjugation::Высрать,
        Conjugation::Стереть,
        Conjugation::Выспаться,
        Conjugation::Застлать,
        Conjugation::Размять,
        Conjugation::Толочься,
        Conjugation::Скрестись,
        Conjugation::Зажечься,
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
        Conjugation::Почать,
        Conjugation::Нашептать,
        Conjugation::Прятаться,
        Conjugation::Реветь,
        Conjugation::Закупорить,
        Conjugation::Ебсти,
        Conjugation::Залгаться,
        Conjugation::Лгать,
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
        Conjugation::Мести,
        Conjugation::Висеть,
        Conjugation::Выбрать,
        Conjugation::Отжать,
        Conjugation::Отстоять,
        Conjugation::Заспать,
        Conjugation::Выставиться,
        Conjugation::Одолеть,
        Conjugation::Выставить,
        Conjugation::Изъять,
        Conjugation::Выдвинуться,
        Conjugation::Пробежаться,
        Conjugation::Обжечься,
        Conjugation::Приобрестись,
        Conjugation::Волгнуть,
        Conjugation::Стлаться,
        Conjugation::Распоясаться,
        Conjugation::Бухвостить,
        Conjugation::Вылиться,
        Conjugation::Трепетаться,
        Conjugation::Уссаться,
        Conjugation::Вылезти,
        Conjugation::Срастись,
        Conjugation::Ехать,
        Conjugation::Цвести,
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
        Conjugation::Выебсти,
        Conjugation::Поколебать,
        Conjugation::Насочинять,
        Conjugation::Растереться,
        Conjugation::Изъяться,
        Conjugation::Хотеться,
        Conjugation::Вырасти,
        Conjugation::Высвободить,
        Conjugation::Начаться,
        Conjugation::Переть,
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

pub(super) static CONJUGATIONS: [Paradigm<'static>; 348] = [
    // Играть: 11,621 words (знать, играть, презирать, делать, уступать, скучать).
    conjugation!("играть", 11621, "~ть";
        infinitive: "~ть";
        non_past_first: "~ю", "~ем";
        non_past_second: "~ешь", "~ете";
        non_past_third: "~ет", "~ют";
        past: "~л", "~ла", "~ло", "~ли";
        imperative: "~й", "~йте";
        participle: "~ющий", "~вший", "~емый", "~нный";
        gerund: "~я", "~в|~вши";
    ),
    // Набираться: 8,289 words (отпочковываться, изъясняться, облокачиваться, кончаться, являться, сморкаться).
    conjugation!("набираться", 8289, "~ть~я";
        infinitive: "~ть~я";
        non_past_first: "~ю~ь", "~ем~я";
        non_past_second: "~ешь~я", "~ете~ь";
        non_past_third: "~ет~я", "~ют~я";
        past: "~л~я", "~ла~ь", "~ло~ь", "~ли~ь";
        imperative: "~й~я", "~йте~ь";
        participle: "~ющий~я", "~вший~я", "~е~емый", "~ё~нный";
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
        participle: "~ующий~я", "~овавший~я", "~е~емый", "~ё~нный";
        gerund: "~уя~ь", "~овавши~ь";
    ),
    // Числиться: 1,171 words (изъясниться, соединиться, присоединиться, объясниться, валиться, приземлиться).
    conjugation!("числиться", 1171, "~ить~я";
        infinitive: "~ить~я";
        non_past_first: "~ю~ь", "~им~я";
        non_past_second: "~ишь~я", "~ите~ь";
        non_past_third: "~ит~я", "~ят~я";
        past: "~ил~я", "~ила~ь", "~ило~ь", "~или~ь";
        imperative: "~и~ь", "~ите~ь";
        participle: "~ящий~я", "~ивший~я", "~е~емый", "~ё~нный";
        gerund: "~я~ь", "~ивши~ь";
    ),
    // Хуярить: 1,129 words (жалить, ужалить, ударить, парить, жарить, верить).
    conjugation!("хуярить", 1129, "~ить";
        infinitive: "~ить";
        non_past_first: "~ю", "~им";
        non_past_second: "~ишь", "~ите";
        non_past_third: "~ит", "~ят";
        past: "~ил", "~ила", "~ило", "~или";
        imperative: "~ь", "~ьте";
        participle: "~ящий", "~ивший", "~имый", "~енный";
        gerund: "~я", "~ив|~ивши";
    ),
    // Палить: 1,053 words (говорить, поговорить, изменить, благодарить, винить, отменить).
    conjugation!("палить", 1053, "~ить";
        infinitive: "~ить";
        non_past_first: "~ю", "~им";
        non_past_second: "~ишь", "~ите";
        non_past_third: "~ит", "~ят";
        past: "~ил", "~ила", "~ило", "~или";
        imperative: "~и", "~ите";
        participle: "~ящий", "~ивший", "~имый", "~ённый";
        gerund: "~я", "~ив|~ивши";
    ),
    // Раздуть: 928 words (деревенеть, взять, одеревенеть, деревянеть, болеть, краснеть).
    conjugation!("раздуть", 928, "~ть";
        infinitive: "~ть";
        non_past_first: "~ю", "~ем";
        non_past_second: "~ешь", "~ете";
        non_past_third: "~ет", "~ют";
        past: "~л", "~ла", "~ло", "~ли";
        imperative: "~й", "~йте";
        participle: "~ющий", "~вший", "~емый", "~тый";
        gerund: "~я", "~в|~вши";
    ),
    // Рвануть: 679 words (взбзднуть, хохотнуть, блевануть, пугнуть, гнуть, ткнуть).
    conjugation!("рвануть", 679, "~уть";
        infinitive: "~уть";
        non_past_first: "~у", "~ём";
        non_past_second: "~ёшь", "~ёте";
        non_past_third: "~ёт", "~ут";
        past: "~ул", "~ула", "~уло", "~ули";
        imperative: "~и", "~ите";
        participle: "~ущий", "~увший", "~омый", "~утый";
        gerund: "~я", "~ув|~увши";
    ),
    // Предназначить: 619 words (увеличить, значить, тревожить, противоречить, обнаружить, разрушить).
    conjugation!("предназначить", 619, "~ить";
        infinitive: "~ить";
        non_past_first: "~у", "~им";
        non_past_second: "~ишь", "~ите";
        non_past_third: "~ит", "~ат";
        past: "~ил", "~ила", "~ило", "~или";
        imperative: "~ь", "~ьте";
        participle: "~ащий", "~ивший", "~имый", "~енный";
        gerund: "~а", "~ив|~ивши";
    ),
    // Усилиться: 566 words (ссориться, пялиться, ускориться, стариться, состариться, целиться).
    conjugation!("усилиться", 566, "~ить~я";
        infinitive: "~ить~я";
        non_past_first: "~ю~ь", "~им~я";
        non_past_second: "~ишь~я", "~ите~ь";
        non_past_third: "~ит~я", "~ят~я";
        past: "~ил~я", "~ила~ь", "~ило~ь", "~или~ь";
        imperative: "~ь~я", "~ьте~ь";
        participle: "~ящий~я", "~ивший~я", "~е~емый", "~ё~нный";
        gerund: "~я~ь", "~ивши~ь|~я~ь";
    ),
    // Тянуть: 548 words (выковырнуть, прыгнуть, пёрнуть, капнуть, шлёпнуть, взглянуть).
    conjugation!("тянуть", 548, "~уть";
        infinitive: "~уть";
        non_past_first: "~у", "~ем";
        non_past_second: "~ешь", "~ете";
        non_past_third: "~ет", "~ут";
        past: "~ул", "~ула", "~уло", "~ули";
        imperative: "~и", "~ите";
        participle: "~ущий", "~увший", "~имый", "~утый";
        gerund: "~ая", "~ув|~увши";
    ),
    // Выделить: 520 words (курить, покурить, помнить, запомнить, варить, чинить).
    conjugation!("выделить", 520, "~ить";
        infinitive: "~ить";
        non_past_first: "~ю", "~им";
        non_past_second: "~ишь", "~ите";
        non_past_third: "~ит", "~ят";
        past: "~ил", "~ила", "~ило", "~или";
        imperative: "~и", "~ите";
        participle: "~ящий", "~ивший", "~имый", "~енный";
        gerund: "~я", "~ив|~ивши";
    ),
    // Обнажиться: 501 words (учиться, кончиться, страшиться, случиться, научиться, уменьшиться).
    conjugation!("обнажиться", 501, "~ить~я";
        infinitive: "~ить~я";
        non_past_first: "~у~ь", "~им~я";
        non_past_second: "~ишь~я", "~ите~ь";
        non_past_third: "~ит~я", "~ат~я";
        past: "~ил~я", "~ила~ь", "~ило~ь", "~или~ь";
        imperative: "~и~ь", "~ите~ь";
        participle: "~ащий~я", "~ивший~я", "~е~емый", "~ё~нный";
        gerund: "~а~ь", "~ивши~ь";
    ),
    // Прицепиться: 454 words (появиться, торопиться, стремиться, влюбиться, удивиться, остановиться).
    conjugation!("прицепиться", 454, "~ить~я";
        infinitive: "~ить~я";
        non_past_first: "~лю~ь", "~им~я";
        non_past_second: "~ишь~я", "~ите~ь";
        non_past_third: "~ит~я", "~ят~я";
        past: "~ил~я", "~ила~ь", "~ило~ь", "~или~ь";
        imperative: "~и~ь", "~ите~ь";
        participle: "~ящий~я", "~ивший~я", "~е~емый", "~ё~нный";
        gerund: "~я~ь", "~ивши~ь";
    ),
    // Обернуться: 391 words (рехнуться, повернуться, проснуться, заткнуться, вернуться, улыбнуться).
    conjugation!("обернуться", 391, "~уть~я";
        infinitive: "~уть~я";
        non_past_first: "~у~ь", "~ём~я";
        non_past_second: "~ёшь~я", "~ёте~ь";
        non_past_third: "~ёт~я", "~ут~я";
        past: "~ул~я", "~ула~ь", "~уло~ь", "~ули~ь";
        imperative: "~и~ь", "~ите~ь";
        participle: "~ущий~я", "~увший~я", "~е~емый", "~ё~нный";
        gerund: "~я~ь", "~увши~ь";
    ),
    // Служить: 358 words (оглушить, разрешить, совершить, спешить, окружить, решить).
    conjugation!("служить", 358, "~ить";
        infinitive: "~ить";
        non_past_first: "~у", "~им";
        non_past_second: "~ишь", "~ите";
        non_past_third: "~ит", "~ат";
        past: "~ил", "~ила", "~ило", "~или";
        imperative: "~и", "~ите";
        participle: "~ащий", "~ивший", "~имый", "~ённый";
        gerund: "~а", "~ив|~ивши";
    ),
    // Обрушиться: 343 words (увеличиться, размножиться, кочевряжиться, дурачиться, спешиться, мучиться).
    conjugation!("обрушиться", 343, "~ить~я";
        infinitive: "~ить~я";
        non_past_first: "~у~ь", "~им~я";
        non_past_second: "~ишь~я", "~ите~ь";
        non_past_third: "~ит~я", "~ат~я";
        past: "~ил~я", "~ила~ь", "~ило~ь", "~или~ь";
        imperative: "~ь~я", "~ьте~ь";
        participle: "~ащий~я", "~ивший~я", "~е~емый", "~ё~нный";
        gerund: "~а~ь", "~ивши~ь";
    ),
    // Восстановить: 329 words (уступить, любить, полюбить, возлюбить, купить, заломить).
    conjugation!("восстановить", 329, "~ить";
        infinitive: "~ить";
        non_past_first: "~лю", "~им";
        non_past_second: "~ишь", "~ите";
        non_past_third: "~ит", "~ят";
        past: "~ил", "~ила", "~ило", "~или";
        imperative: "~и", "~ите";
        participle: "~ящий", "~ивший", "~имый", "~ленный";
        gerund: "~я", "~ив|~ивши";
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
    // Долбить: 324 words (вступить, наступить, дымить, удивить, бомбить, выступить).
    conjugation!("долбить", 324, "~ить";
        infinitive: "~ить";
        non_past_first: "~лю", "~им";
        non_past_second: "~ишь", "~ите";
        non_past_third: "~ит", "~ят";
        past: "~ил", "~ила", "~ило", "~или";
        imperative: "~и", "~ите";
        participle: "~ящий", "~ивший", "~имый", "~лённый";
        gerund: "~я", "~ив|~ивши";
    ),
    // Расправиться: 296 words (нравиться, понравиться, готовиться, направиться, знакомиться, познакомиться).
    conjugation!("расправиться", 296, "~ить~я";
        infinitive: "~ить~я";
        non_past_first: "~лю~ь", "~им~я";
        non_past_second: "~ишь~я", "~ите~ь";
        non_past_third: "~ит~я", "~ят~я";
        past: "~ил~я", "~ила~ь", "~ило~ь", "~или~ь";
        imperative: "~ь~я", "~ьте~ь";
        participle: "~ящий~я", "~ивший~я", "~е~емый", "~ё~нный";
        gerund: "~я~ь", "~ивши~ь";
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
    // Представить: 290 words (забавить, восславить, готовить, грабить, заставить, править).
    conjugation!("представить", 290, "~ить";
        infinitive: "~ить";
        non_past_first: "~лю", "~им";
        non_past_second: "~ишь", "~ите";
        non_past_third: "~ит", "~ят";
        past: "~ил", "~ила", "~ило", "~или";
        imperative: "~ь", "~ьте";
        participle: "~ящий", "~ивший", "~имый", "~ленный";
        gerund: "~я", "~ив|~ивши";
    ),
    // Проскочить: 287 words (учить, кончить, закончить, предложить, выключить, научить).
    conjugation!("проскочить", 287, "~ить";
        infinitive: "~ить";
        non_past_first: "~у", "~им";
        non_past_second: "~ишь", "~ите";
        non_past_third: "~ит", "~ат";
        past: "~ил", "~ила", "~ило", "~или";
        imperative: "~и", "~ите";
        participle: "~ащий", "~ивший", "~имый", "~енный";
        gerund: "~а", "~ив|~ивши";
    ),
    // Расходиться: 283 words (находиться, трудиться, садиться, насладиться, разводиться, родиться).
    conjugation!("расходиться", 283, "~дить~я";
        infinitive: "~дить~я";
        non_past_first: "~жу~ь", "~дим~я";
        non_past_second: "~дишь~я", "~дите~ь";
        non_past_third: "~дит~я", "~дят~я";
        past: "~дил~я", "~дила~ь", "~дило~ь", "~дили~ь";
        imperative: "~ди~ь", "~дите~ь";
        participle: "~дящий~я", "~дивший~я", "~е~емый", "~ё~нный";
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
        participle: "~тящий~я", "~тивший~я", "~е~емый", "~ё~нный";
        gerund: "~тя~ь", "~тивши~ь";
    ),
    // Тянуться: 230 words (грохнуться, чокнуться, оглянуться, тянуться, потянуться, откликнуться).
    conjugation!("тянуться", 230, "~уть~я";
        infinitive: "~уть~я";
        non_past_first: "~у~ь", "~ем~я";
        non_past_second: "~ешь~я", "~ете~ь";
        non_past_third: "~ет~я", "~ут~я";
        past: "~ул~я", "~ула~ь", "~уло~ь", "~ули~ь";
        imperative: "~и~ь", "~ите~ь";
        participle: "~ущий~я", "~увший~я", "~е~емый", "~ё~нный";
        gerund: "~ая~ь", "~увши~ь";
    ),
    // Залипнуть: 203 words (распухнуть, сдохнуть, замёрзнуть, высохнуть, погибнуть, окрепнуть).
    conjugation!("залипнуть", 203, "~нуть";
        infinitive: "~нуть";
        non_past_first: "~ну", "~нем";
        non_past_second: "~нешь", "~нете";
        non_past_third: "~нет", "~нут";
        past: "~", "~ла", "~ло", "~ли";
        imperative: "~ни", "~ните";
        participle: "~нущий", "~ший", "~нимый", "~нутый";
        gerund: "~ня", "~ши";
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
    // Распуститься: 163 words (креститься, спуститься, сгуститься, опуститься, поместиться, проститься).
    conjugation!("распуститься", 163, "~~титься";
        infinitive: "~~титься";
        non_past_first: "~щу~ь", "~~тимся";
        non_past_second: "~~тишься", "~~титесь";
        non_past_third: "~~тится", "~~тятся";
        past: "~~тился", "~~тилась", "~~тилось", "~~тились";
        imperative: "~~тись", "~~титесь";
        participle: "~~тящийся", "~~тившийся", "~е~емый", "~ё~нный";
        gerund: "~~тясь", "~~тившись";
    ),
    // Отметить: 153 words (ответить, встретить, заметить, утратить, тратить, отметить).
    conjugation!("отметить", 153, "~тить";
        infinitive: "~тить";
        non_past_first: "~чу", "~тим";
        non_past_second: "~тишь", "~тите";
        non_past_third: "~тит", "~тят";
        past: "~тил", "~тила", "~тило", "~тили";
        imperative: "~ть", "~тьте";
        participle: "~тящий", "~тивший", "~тимый", "~ченный";
        gerund: "~тя", "~тив|~тивши";
    ),
    // Стучать: 152 words (держать, кричать, жужжать, мычать, лежать, подержать).
    conjugation!("стучать", 152, "~ать";
        infinitive: "~ать";
        non_past_first: "~у", "~им";
        non_past_second: "~ишь", "~ите";
        non_past_third: "~ит", "~ат";
        past: "~ал", "~ала", "~ало", "~али";
        imperative: "~и", "~ите";
        participle: "~ащий", "~авший", "~имый", "~анный";
        gerund: "~а", "~ав|~авши";
    ),
    // Возместить: 152 words (крестить, простить, нарастить, поместить, растить, переместить).
    conjugation!("возместить", 152, "~стить";
        infinitive: "~стить";
        non_past_first: "~щу", "~стим";
        non_past_second: "~стишь", "~стите";
        non_past_third: "~стит", "~стят";
        past: "~стил", "~стила", "~стило", "~стили";
        imperative: "~сти", "~стите";
        participle: "~стящий", "~стивший", "~стимый", "~щённый";
        gerund: "~стя", "~стив|~стивши";
    ),
    // Носиться: 138 words (соотноситься, согласиться, относиться, напроситься, произноситься, доноситься).
    conjugation!("носиться", 138, "~~иться";
        infinitive: "~~иться";
        non_past_first: "~шу~ь", "~~имся";
        non_past_second: "~~ишься", "~~итесь";
        non_past_third: "~~ится", "~~ятся";
        past: "~~ился", "~~илась", "~~илось", "~~ились";
        imperative: "~~ись", "~~итесь";
        participle: "~~ящийся", "~~ившийся", "~е~емый", "~ё~нный";
        gerund: "~~ясь", "~~ившись";
    ),
    // Забросить: 125 words (красить, бросить, сбросить, украсить, повесить, весить).
    conjugation!("забросить", 125, "~сить";
        infinitive: "~сить";
        non_past_first: "~шу", "~сим";
        non_past_second: "~сишь", "~сите";
        non_past_third: "~сит", "~сят";
        past: "~сил", "~сила", "~сило", "~сили";
        imperative: "~сь", "~сьте";
        participle: "~сящий", "~сивший", "~симый", "~шенный";
        gerund: "~ся", "~сив|~сивши";
    ),
    // Встретиться: 120 words (охотиться, заботиться, встретиться, пятиться, отметиться, позаботиться).
    conjugation!("встретиться", 120, "~тить~я";
        infinitive: "~тить~я";
        non_past_first: "~чу~ь", "~тим~я";
        non_past_second: "~тишь~я", "~тите~ь";
        non_past_third: "~тит~я", "~тят~я";
        past: "~тил~я", "~тила~ь", "~тило~ь", "~тили~ь";
        imperative: "~ть~я", "~тьте~ь";
        participle: "~тящий~я", "~тивший~я", "~е~емый", "~ё~нный";
        gerund: "~тя~ь", "~тивши~ь";
    ),
    // Засосать: 118 words (врать, ебать, ржать, заржать, соврать, рвать).
    conjugation!("засосать", 118, "~ать";
        infinitive: "~ать";
        non_past_first: "~у", "~ём";
        non_past_second: "~ёшь", "~ёте";
        non_past_third: "~ёт", "~ут";
        past: "~ал", "~ала", "~ало", "~али";
        imperative: "~и", "~ите";
        participle: "~ущий", "~авший", "~имый", "~анный";
        gerund: "~я", "~ав|~авши";
    ),
    // Развеять: 113 words (баять, лаять, чуять, сеять, блеять, посеять).
    conjugation!("развеять", 113, "~ять";
        infinitive: "~ять";
        non_past_first: "~ю", "~ем";
        non_past_second: "~ешь", "~ете";
        non_past_third: "~ет", "~ют";
        past: "~ял", "~яла", "~яло", "~яли";
        imperative: "~й", "~йте";
        participle: "~ющий", "~явший", "~емый", "~янный";
        gerund: "~я", "~яв|~явши";
    ),
    // Перестроиться: 110 words (беспокоиться, успокоиться, приклеиться, строиться, устроиться, настроиться).
    conjugation!("перестроиться", 110, "~ить~я";
        infinitive: "~ить~я";
        non_past_first: "~ю~ь", "~им~я";
        non_past_second: "~ишь~я", "~ите~ь";
        non_past_third: "~ит~я", "~ят~я";
        past: "~ил~я", "~ила~ь", "~ило~ь", "~или~ь";
        imperative: "~й~я", "~йте~ь";
        participle: "~ящий~я", "~ивший~я", "~е~емый", "~ё~нный";
        gerund: "~я~ь", "~ивши~ь";
    ),
    // Держаться: 108 words (воздержаться, мчаться, держаться, послышаться, слышаться, содержаться).
    conjugation!("держаться", 108, "~ать~я";
        infinitive: "~ать~я";
        non_past_first: "~у~ь", "~им~я";
        non_past_second: "~ишь~я", "~ите~ь";
        non_past_third: "~ит~я", "~ат~я";
        past: "~ал~я", "~ала~ь", "~ало~ь", "~али~ь";
        imperative: "~и~ь", "~ите~ь";
        participle: "~ащий~я", "~авший~я", "~е~емый", "~ё~нный";
        gerund: "~а~ь", "~авши~ь";
    ),
    // Топтать: 107 words (хохотать, захохотать, щекотать, лепетать, бормотать, хлопотать).
    conjugation!("топтать", 107, "~тать";
        infinitive: "~тать";
        non_past_first: "~чу", "~чем";
        non_past_second: "~чешь", "~чете";
        non_past_third: "~чет", "~чут";
        past: "~тал", "~тала", "~тало", "~тали";
        imperative: "~чи", "~чите";
        participle: "~чущий", "~тавший", "~тимый", "~танный";
        gerund: "~ча", "~тав|~тавши";
    ),
    // Сорваться: 103 words (ебаться, стебаться, усраться, обосраться, дождаться, заебаться).
    conjugation!("сорваться", 103, "~ать~я";
        infinitive: "~ать~я";
        non_past_first: "~у~ь", "~ём~я";
        non_past_second: "~ёшь~я", "~ёте~ь";
        non_past_third: "~ёт~я", "~ут~я";
        past: "~ал~я", "~ала~ь", "~ало~ь|~ало~ь", "~али~ь|~али~ь";
        imperative: "~и~ь", "~ите~ь";
        participle: "~ущий~я", "~авший~я", "~е~емый", "~ё~нный";
        gerund: "~я~ь", "~авши~ь";
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
    // Трафить: 102 words (суесловить, буравить, брасопить, дыбить, шароёбить, швартовить).
    conjugation!("трафить", 102, "~ить";
        infinitive: "~ить";
        non_past_first: "~лю", "~им";
        non_past_second: "~ишь", "~ите";
        non_past_third: "~ит", "~ят";
        past: "~ил", "~ила", "~ило", "~или";
        imperative: "~ь", "~ьте";
        participle: "~ящий", "~ивший", "~имый", "~ленный";
        gerund: "~я", "~ивши|~ив";
    ),
    // Сычевать: 101 words (бичевать, кочевать, ночевать, корчевать, бушевать, переночевать).
    conjugation!("сычевать", 101, "~евать";
        infinitive: "~евать";
        non_past_first: "~ую", "~уем";
        non_past_second: "~уешь", "~уете";
        non_past_third: "~ует", "~уют";
        past: "~евал", "~евала", "~евало", "~евали";
        imperative: "~уй", "~уйте";
        participle: "~ующий", "~евавший", "~уемый", "~ёванный";
        gerund: "~уя", "~евав|~евавши";
    ),
    // Наладиться: 95 words (ладиться, наладиться, отвадиться, сгрудиться, заладиться, повадиться).
    conjugation!("наладиться", 95, "~дить~я";
        infinitive: "~дить~я";
        non_past_first: "~жу~ь", "~дим~я";
        non_past_second: "~дишь~я", "~дите~ь";
        non_past_third: "~дит~я", "~дят~я";
        past: "~дил~я", "~дила~ь", "~дило~ь", "~дили~ь";
        imperative: "~дь~я", "~дьте~ь";
        participle: "~дящий~я", "~дивший~я", "~е~емый", "~ё~нный";
        gerund: "~дя~ь", "~дивши~ь";
    ),
    // Переброситься: 93 words (выситься, броситься, возвыситься, повыситься, свеситься, наброситься).
    conjugation!("переброситься", 93, "~~иться";
        infinitive: "~~иться";
        non_past_first: "~шу~ь", "~~имся";
        non_past_second: "~~ишься", "~~итесь";
        non_past_third: "~~ится", "~~ятся";
        past: "~~ился", "~~илась", "~~илось", "~~ились";
        imperative: "~~ься", "~~ьтесь";
        participle: "~~ящийся", "~~ившийся", "~е~емый", "~ё~нный";
        gerund: "~~ясь", "~~ившись";
    ),
    // Кишеть: 91 words (загреметь, храпеть, кипеть, терпеть, скрипеть, греметь).
    conjugation!("кишеть", 91, "~еть";
        infinitive: "~еть";
        non_past_first: "~лю", "~им";
        non_past_second: "~ишь", "~ите";
        non_past_third: "~ит", "~ят";
        past: "~ел", "~ела", "~ело", "~ели";
        imperative: "~и", "~ите";
        participle: "~ящий", "~евший", "~имый", "~енный";
        gerund: "~я", "~ев|~евши";
    ),
    // Елозить: 90 words (лазить, заморозить, морозить, конфузить, унизить, снизить).
    conjugation!("елозить", 90, "~зить";
        infinitive: "~зить";
        non_past_first: "~жу", "~зим";
        non_past_second: "~зишь", "~зите";
        non_past_third: "~зит", "~зят";
        past: "~зил", "~зила", "~зило", "~зили";
        imperative: "~зь", "~зьте";
        participle: "~зящий", "~зивший", "~зимый", "~женный";
        gerund: "~зя", "~зив|~зивши";
    ),
    // Стоить: 89 words (беспокоить, строить, стоить, клеить, устроить, построить).
    conjugation!("стоить", 89, "~ить";
        infinitive: "~ить";
        non_past_first: "~ю", "~им";
        non_past_second: "~ишь", "~ите";
        non_past_third: "~ит", "~ят";
        past: "~ил", "~ила", "~ило", "~или";
        imperative: "~й", "~йте";
        participle: "~ящий", "~ивший", "~имый", "~енный";
        gerund: "~я", "~ив|~ивши";
    ),
    // Подкинуть: 88 words (покинуть, тронуть, кинуть, клюнуть, плюнуть, сунуть).
    conjugation!("подкинуть", 88, "~уть";
        infinitive: "~уть";
        non_past_first: "~у", "~ем";
        non_past_second: "~ешь", "~ете";
        non_past_third: "~ет", "~ут";
        past: "~ул", "~ула", "~уло", "~ули";
        imperative: "~ь", "~ьте";
        participle: "~ущий", "~увший", "~емый", "~утый";
        gerund: "~я", "~ув|~увши";
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
    // Наладить: 83 words (гладить, погладить, лихорадить, бредить, ладить, наладить).
    conjugation!("наладить", 83, "~дить";
        infinitive: "~дить";
        non_past_first: "~жу", "~дим";
        non_past_second: "~дишь", "~дите";
        non_past_third: "~дит", "~дят";
        past: "~дил", "~дила", "~дило", "~дили";
        imperative: "~дь", "~дьте";
        participle: "~дящий", "~дивший", "~димый", "~женный";
        gerund: "~дя", "~див|~дивши";
    ),
    // Выразиться: 81 words (сразиться, возиться, выразиться, выгрузиться, отразиться, грузиться).
    conjugation!("выразиться", 81, "~зить~я";
        infinitive: "~зить~я";
        non_past_first: "~жу~ь", "~зим~я";
        non_past_second: "~зишь~я", "~зите~ь";
        non_past_third: "~зит~я", "~зят~я";
        past: "~зил~я", "~зила~ь", "~зило~ь", "~зили~ь";
        imperative: "~зи~ь", "~зите~ь";
        participle: "~зящий~я", "~зивший~я", "~е~емый", "~ё~нный";
        gerund: "~зя~ь", "~зивши~ь";
    ),
    // Зарядить: 79 words (щадить, пощадить, опередить, зарядить, отрядить, снарядить).
    conjugation!("зарядить", 79, "~дить";
        infinitive: "~дить";
        non_past_first: "~жу", "~дим";
        non_past_second: "~дишь", "~дите";
        non_past_third: "~дит", "~дят";
        past: "~дил", "~дила", "~дило", "~дили";
        imperative: "~ди", "~дите";
        participle: "~дящий", "~дивший", "~димый", "~жённый";
        gerund: "~дя", "~див|~дивши";
    ),
    // Оказать: 79 words (сказать, рассказать, показать, связать, лизать, доказать).
    conjugation!("оказать", 79, "~зать";
        infinitive: "~зать";
        non_past_first: "~жу", "~жем";
        non_past_second: "~жешь", "~жете";
        non_past_third: "~жет", "~жут";
        past: "~зал", "~зала", "~зало", "~зали";
        imperative: "~жи", "~жите";
        participle: "~жущий", "~завший", "~даемый", "~занный";
        gerund: "~жа", "~зав|~завши";
    ),
    // Отдавать: 77 words (давать, воссоздавать, создавать, издавать, уставать, предавать).
    conjugation!("отдавать", 77, "~вать";
        infinitive: "~вать";
        non_past_first: "~ю", "~ём";
        non_past_second: "~ёшь", "~ёте";
        non_past_third: "~ёт", "~ют";
        past: "~вал", "~вала", "~вало", "~вали";
        imperative: "~вай", "~вайте";
        participle: "~ющий", "~вавший", "~ваемый", "~янный";
        gerund: "~вая", "~вав|~вавши";
    ),
    // Отпустить: 75 words (допустить, пропустить, чистить, вырастить, пустить, упустить).
    conjugation!("отпустить", 75, "~стить";
        infinitive: "~стить";
        non_past_first: "~щу", "~стим";
        non_past_second: "~стишь", "~стите";
        non_past_third: "~стит", "~стят";
        past: "~стил", "~стила", "~стило", "~стили";
        imperative: "~сти", "~стите";
        participle: "~стящий", "~стивший", "~стимый", "~щенный";
        gerund: "~стя", "~стив|~стивши";
    ),
    // Развеяться: 73 words (надеяться, каяться, покаяться, маяться, измаяться, отчаяться).
    conjugation!("развеяться", 73, "~ять~я";
        infinitive: "~ять~я";
        non_past_first: "~ю~ь", "~ем~я";
        non_past_second: "~ешь~я", "~ете~ь";
        non_past_third: "~ет~я", "~ют~я";
        past: "~ял~я", "~яла~ь", "~яло~ь", "~яли~ь";
        imperative: "~й~я", "~йте~ь";
        participle: "~ющий~я", "~явший~я", "~е~емый", "~ё~нный";
        gerund: "~я~ь", "~явши~ь";
    ),
    // Коптить: 72 words (коптить, сплотить, золотить, закоптить, кипятить, вскипятить).
    conjugation!("коптить", 72, "~тить";
        infinitive: "~тить";
        non_past_first: "~чу", "~тим";
        non_past_second: "~тишь", "~тите";
        non_past_third: "~тит", "~тят";
        past: "~тил", "~тила", "~тило", "~тили";
        imperative: "~ти", "~тите";
        participle: "~тящий", "~тивший", "~тимый", "~чённый";
        gerund: "~тя", "~тив|~тивши";
    ),
    // Бухтеть: 72 words (лететь, хотеть, полететь, захотеть, взлететь, вертеть).
    conjugation!("бухтеть", 72, "~теть";
        infinitive: "~теть";
        non_past_first: "~чу", "~тим";
        non_past_second: "~тишь", "~тите";
        non_past_third: "~тит", "~тят";
        past: "~тел", "~тела", "~тело", "~тели";
        imperative: "~ти", "~тите";
        participle: "~тящий", "~тевший", "~тимый", "~ченный";
        gerund: "~тя", "~тев|~тевши";
    ),
    // Стушеваться: 71 words (стушеваться, тушеваться, разбушеваться, зарубцеваться, кольцеваться, вытанцеваться).
    conjugation!("стушеваться", 71, "~евать~я";
        infinitive: "~евать~я";
        non_past_first: "~ую~ь", "~уем~я";
        non_past_second: "~уешь~я", "~уете~ь";
        non_past_third: "~ует~я", "~уют~я";
        past: "~евал~я", "~евала~ь", "~евало~ь", "~евали~ь";
        imperative: "~уй~я", "~уйте~ь";
        participle: "~ующий~я", "~евавший~я", "~е~емый", "~ё~нный";
        gerund: "~уя~ь", "~евавши~ь";
    ),
    // Отвоевать: 70 words (мухлевать, воевать, завоевать, горевать, отвоевать, повоевать).
    conjugation!("отвоевать", 70, "~евать";
        infinitive: "~евать";
        non_past_first: "~юю", "~юем";
        non_past_second: "~юешь", "~юете";
        non_past_third: "~юет", "~юют";
        past: "~евал", "~евала", "~евало", "~евали";
        imperative: "~юй", "~юйте";
        participle: "~юющий", "~евавший", "~юемый", "~ёванный";
        gerund: "~юя", "~евав|~евавши";
    ),
    // Забить: 68 words (пить, бить, лить, вить, шить, побить).
    conjugation!("забить", 68, "~ить";
        infinitive: "~ить";
        non_past_first: "~ью", "~ьём";
        non_past_second: "~ьёшь", "~ьёте";
        non_past_third: "~ьёт", "~ьют";
        past: "~ил", "~ила", "~ило", "~или";
        imperative: "~ей", "~ейте";
        participle: "~ьющий", "~ивший", "~еемый", "~итый";
        gerund: "~ья", "~ив|~ивши";
    ),
    // Сблизиться: 66 words (приблизиться, сконфузиться, близиться, снизиться, сузиться, сблизиться).
    conjugation!("сблизиться", 66, "~зить~я";
        infinitive: "~зить~я";
        non_past_first: "~жу~ь", "~зим~я";
        non_past_second: "~зишь~я", "~зите~ь";
        non_past_third: "~зит~я", "~зят~я";
        past: "~зил~я", "~зила~ь", "~зило~ь", "~зили~ь";
        imperative: "~зь~я", "~зьте~ь";
        participle: "~зящий~я", "~зивший~я", "~е~емый", "~ё~нный";
        gerund: "~зя~ь", "~зивши~ь";
    ),
    // Сидеть: 66 words (бздеть, набздеть, сидеть, пиздеть, пердеть, выглядеть).
    conjugation!("сидеть", 66, "~деть";
        infinitive: "~деть";
        non_past_first: "~жу", "~дим";
        non_past_second: "~дишь", "~дите";
        non_past_third: "~дит", "~дят";
        past: "~дел", "~дела", "~дело", "~дели";
        imperative: "~ди", "~дите";
        participle: "~дящий", "~девший", "~димый", "~жённый";
        gerund: "~дя", "~дев|~девши";
    ),
    // Клепать: 60 words (зевать, оседлать, хлебать, седлать, верстать, сверстать).
    conjugation!("клепать", 60, "~е~ть";
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
    conjugation!("обтечь", 60, "~ечь";
        infinitive: "~ечь";
        non_past_first: "~еку", "~ечём";
        non_past_second: "~ечёшь", "~ечёте";
        non_past_third: "~ечёт", "~екут";
        past: "~ёк", "~екла", "~екло", "~екли";
        imperative: "~еки", "~еките";
        participle: "~екущий", "~ёкший", "~екомый", "~ечённый";
        gerund: "~я", "~ёкши";
    ),
    // Возбудить: 59 words (освободить, победить, предупредить, повредить, руководить, обсудить).
    conjugation!("возбудить", 59, "~дить";
        infinitive: "~дить";
        non_past_first: "~жу", "~дим";
        non_past_second: "~дишь", "~дите";
        non_past_third: "~дит", "~дят";
        past: "~дил", "~дила", "~дило", "~дили";
        imperative: "~ди", "~дите";
        participle: "~дящий", "~дивший", "~димый", "~ждённый";
        gerund: "~дя", "~див|~дивши";
    ),
    // Виться: 57 words (литься, биться, добиться, напиться, налиться, пробиться).
    conjugation!("виться", 57, "~ить~я";
        infinitive: "~ить~я";
        non_past_first: "~ью~ь", "~ьём~я";
        non_past_second: "~ьёшь~я", "~ьёте~ь";
        non_past_third: "~ьёт~я", "~ьют~я";
        past: "~ил~я|~ил~я", "~ила~ь", "~ило~ь|~ило~ь", "~или~ь|~или~ь";
        imperative: "~ей~я", "~ейте~ь";
        participle: "~ьющий~я", "~ивший~я", "~е~емый", "~ё~нный";
        gerund: "~ья~ь", "~ивши~ь";
    ),
    // Перевернуть: 57 words (перевернуть, повернуть, взметнуть, свернуть, завернуть, подчеркнуть).
    conjugation!("перевернуть", 57, "~е~уть";
        infinitive: "~е~уть";
        non_past_first: "~е~у", "~е~ём";
        non_past_second: "~е~ёшь", "~е~ёте";
        non_past_third: "~е~ёт", "~е~ут";
        past: "~е~ул", "~е~ула", "~е~уло", "~е~ули";
        imperative: "~е~и", "~е~ите";
        participle: "~е~ущий", "~е~увший", "~е~омый", "~ё~утый";
        gerund: "~е~я", "~е~ув|~е~увши";
    ),
    // Сунуться: 57 words (двинуться, прикинуться, кинуться, дотронуться, тронуться, ринуться).
    conjugation!("сунуться", 57, "~уть~я";
        infinitive: "~уть~я";
        non_past_first: "~у~ь", "~ем~я";
        non_past_second: "~ешь~я", "~ете~ь";
        non_past_third: "~ет~я", "~ут~я";
        past: "~ул~я", "~ула~ь", "~уло~ь", "~ули~ь";
        imperative: "~ь~я", "~ьте~ь";
        participle: "~ущий~я", "~увший~я", "~е~емый", "~ё~нный";
        gerund: "~ая~ь", "~увши~ь";
    ),
    // Раскрыться: 55 words (мыться, помыться, скрыться, открыться, умыться, срыться).
    conjugation!("раскрыться", 55, "~ыть~я";
        infinitive: "~ыть~я";
        non_past_first: "~ою~ь", "~оем~я";
        non_past_second: "~оешь~я", "~оете~ь";
        non_past_third: "~оет~я", "~оют~я";
        past: "~ыл~я", "~ыла~ь", "~ыло~ь", "~ыли~ь";
        imperative: "~ой~я", "~ойте~ь";
        participle: "~оющий~я", "~ывший~я", "~е~емый", "~ё~нный";
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
        participle: "~ющий", "~овший", "~имый", "~отый";
        gerund: "~я", "~ов|~овши";
    ),
    // Заглохнуть: 54 words (достигнуть, проникнуть, возникнуть, затихнуть, подвергнуть, расторгнуть).
    conjugation!("заглохнуть", 54, "~нуть";
        infinitive: "~нуть";
        non_past_first: "~ну", "~нем";
        non_past_second: "~нешь", "~нете";
        non_past_third: "~нет", "~нут";
        past: "~", "~ла", "~ло", "~ли";
        imperative: "~ни", "~ните";
        participle: "~нущий", "~ший|~нувший", "~нимый", "~нутый";
        gerund: "~ня", "~нув|~ши|~нувши";
    ),
    // Идти: 53 words (идти, пасти, спасти, ползти, припасти, упасти).
    conjugation!("идти", 53, "~ти";
        infinitive: "~ти";
        non_past_first: "~у", "~ём";
        non_past_second: "~ёшь", "~ёте";
        non_past_third: "~ёт", "~ут";
        past: "~", "~ла", "~ло", "~ли";
        imperative: "~и", "~ите";
        participle: "~ущий", "~ший", "~омый", "~ённый";
        gerund: "~я", "~ши";
    ),
    // Стать: 53 words (стать, устать, застрять, перестать, встать, надеть).
    conjugation!("стать", 53, "~ть";
        infinitive: "~ть";
        non_past_first: "~ну", "~нем";
        non_past_second: "~нешь", "~нете";
        non_past_third: "~нет", "~нут";
        past: "~л", "~ла", "~ло", "~ли";
        imperative: "~нь", "~ньте";
        participle: "~нущий", "~вший", "~емый", "~тый";
        gerund: "~я", "~в|~вши";
    ),
    // Влечься: 53 words (отречься, увлечься, растечься, отвлечься, стечься, влечься).
    conjugation!("влечься", 53, "~ечь~я";
        infinitive: "~ечь~я";
        non_past_first: "~еку~ь", "~ечём~я";
        non_past_second: "~ечёшь~я", "~ечёте~ь";
        non_past_third: "~ечёт~я", "~екут~я";
        past: "~ёк~я", "~екла~ь", "~екло~ь", "~екли~ь";
        imperative: "~еки~ь", "~еките~ь";
        participle: "~екущий~я", "~ёкший~я", "~е~емый", "~ё~нный";
        gerund: "~я~ь", "~ёкши~ь";
    ),
    // Чесаться: 53 words (подписаться, записаться, писаться, расписаться, вписаться, описаться).
    conjugation!("чесаться", 53, "~~аться";
        infinitive: "~~аться";
        non_past_first: "~шу~ь", "~шем~я";
        non_past_second: "~шешь~я", "~шете~ь";
        non_past_third: "~шет~я", "~шут~я";
        past: "~~ался", "~~алась", "~~алось", "~~ались";
        imperative: "~ши~ь", "~шите~ь";
        participle: "~шущий~я", "~~авшийся", "~е~емый", "~ё~нный";
        gerund: "~ша~ь", "~~авшись";
    ),
    // Отказаться: 52 words (казаться, оказаться, показаться, отказаться, обязаться, высказаться).
    conjugation!("отказаться", 52, "~зать~я";
        infinitive: "~зать~я";
        non_past_first: "~жу~ь", "~жем~я";
        non_past_second: "~жешь~я", "~жете~ь";
        non_past_third: "~жет~я", "~жут~я";
        past: "~зал~я", "~зала~ь", "~зало~ь", "~зали~ь";
        imperative: "~жи~ь", "~жите~ь";
        participle: "~жущий~я", "~завший~я", "~е~емый", "~ё~нный";
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
    conjugation!("отдаваться", 51, "~вать~я";
        infinitive: "~вать~я";
        non_past_first: "~ю~ь", "~ём~я";
        non_past_second: "~ёшь~я", "~ёте~ь";
        non_past_third: "~ёт~я", "~ют~я";
        past: "~вал~я", "~вала~ь", "~вало~ь", "~вали~ь";
        imperative: "~вай~я", "~вайте~ь";
        participle: "~ющий~я", "~вавший~я", "~е~емый", "~ё~нный";
        gerund: "~вая~ь", "~вавши~ь";
    ),
    // Глядеться: 48 words (увидеться, видеться, привидеться, вглядеться, наглядеться, оглядеться).
    conjugation!("глядеться", 48, "~деть~я";
        infinitive: "~деть~я";
        non_past_first: "~жу~ь", "~дим~я";
        non_past_second: "~дишь~я", "~дите~ь";
        non_past_third: "~дит~я", "~дят~я";
        past: "~дел~я", "~дела~ь", "~дело~ь", "~дели~ь";
        imperative: "~ди~ь", "~дите~ь";
        participle: "~дящий~я", "~девший~я", "~е~емый", "~ё~нный";
        gerund: "~дя~ь", "~девши~ь";
    ),
    // Разбить: 46 words (сшить, разбить, сбить, свить, развить, разлить).
    conjugation!("разбить", 46, "~~ить";
        infinitive: "~~ить";
        non_past_first: "~о~ью", "~о~ьём";
        non_past_second: "~о~ьёшь", "~о~ьёте";
        non_past_third: "~о~ьёт", "~о~ьют";
        past: "~~ил", "~~ила", "~~ило", "~~или";
        imperative: "~~ей", "~~ейте";
        participle: "~ющий~я", "~~ивший", "~е~емый", "~~итый";
        gerund: "~я~ь", "~~ив|~~ивши";
    ),
    // Снести: 45 words (изнести, принести, привезти, произнести, донести, перенести).
    conjugation!("снести", 45, "~е~ти";
        infinitive: "~е~ти";
        non_past_first: "~е~у", "~е~ём";
        non_past_second: "~е~ёшь", "~е~ёте";
        non_past_third: "~е~ёт", "~е~ут";
        past: "~ё~", "~е~ла", "~е~ло", "~е~ли";
        imperative: "~е~и", "~е~ите";
        participle: "~е~ущий", "~ё~ший", "~е~омый", "~е~ённый";
        gerund: "~е~я", "~е~я|~ё~ши";
    ),
    // Трещать: 43 words (принадлежать, бренчать, дребезжать, трещать, верещать, содержать).
    conjugation!("трещать", 43, "~е~ать";
        infinitive: "~е~ать";
        non_past_first: "~е~у", "~е~им";
        non_past_second: "~е~ишь", "~е~ите";
        non_past_third: "~е~ит", "~е~ат";
        past: "~е~ал", "~е~ала", "~е~ало", "~е~али";
        imperative: "~е~и", "~е~ите";
        participle: "~е~ащий", "~е~авший", "~е~имый", "~ё~анный";
        gerund: "~е~а", "~е~ав";
    ),
    // Развести: 42 words (провести, перевести, увести, завести, произвести, извести).
    conjugation!("развести", 42, "~ести";
        infinitive: "~ести";
        non_past_first: "~еду", "~едём";
        non_past_second: "~едёшь", "~едёте";
        non_past_third: "~едёт", "~едут";
        past: "~ёл", "~ела", "~ело", "~ели";
        imperative: "~еди", "~едите";
        participle: "~етущий", "~едший", "~ебомый", "~едённый";
        gerund: "~етя", "~едя|~едши";
    ),
    // Колоться: 42 words (бороться, приколоться, расколоться, колоться, напороться, побороться).
    conjugation!("колоться", 42, "~оть~я";
        infinitive: "~оть~я";
        non_past_first: "~ю~ь", "~ем~я";
        non_past_second: "~ешь~я", "~ете~ь";
        non_past_third: "~ет~я", "~ют~я";
        past: "~ол~я", "~ола~ь", "~оло~ь", "~оли~ь";
        imperative: "~и~ь", "~ите~ь";
        participle: "~ющий~я", "~овший~я", "~е~емый", "~ё~нный";
        gerund: "~я~ь", "~овши~ь";
    ),
    // Топтаться: 42 words (метаться, топтаться, расхохотаться, заметаться, шептаться, потоптаться).
    conjugation!("топтаться", 42, "~тать~я";
        infinitive: "~тать~я";
        non_past_first: "~чу~ь", "~чем~я";
        non_past_second: "~чешь~я", "~чете~ь";
        non_past_third: "~чет~я", "~чут~я";
        past: "~тал~я", "~тала~ь", "~тало~ь", "~тали~ь";
        imperative: "~чи~ь", "~чите~ь";
        participle: "~чущий~я", "~тавший~я", "~е~емый", "~ё~нный";
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
    // Сдать: 41 words (дать, воссоздать, создать, издать, предать, выдать).
    conjugation!("сдать", 41, "~ть";
        infinitive: "~ть";
        non_past_first: "~м", "~дим";
        non_past_second: "~шь", "~дите";
        non_past_third: "~ст", "~дут";
        past: "~л|~л", "~ла", "~ло", "~ли|~ли";
        imperative: "~й", "~йте";
        participle: "~ющий", "~вший", "~емый", "~нный";
        gerund: "~я", "~в|~вши";
    ),
    // Резать: 40 words (промазать, резать, отрезать, смазать, мазать, помазать).
    conjugation!("резать", 40, "~зать";
        infinitive: "~зать";
        non_past_first: "~жу", "~жем";
        non_past_second: "~жешь", "~жете";
        non_past_third: "~жет", "~жут";
        past: "~зал", "~зала", "~зало", "~зали";
        imperative: "~жь", "~жьте";
        participle: "~жущий", "~завший", "~даемый", "~занный";
        gerund: "~жа", "~зав|~завши";
    ),
    // Выписать: 39 words (писать, написать, подписать, приписать, описать, записать).
    conjugation!("выписать", 39, "~сать";
        infinitive: "~сать";
        non_past_first: "~шу", "~шем";
        non_past_second: "~шешь", "~шете";
        non_past_third: "~шет", "~шут";
        past: "~сал", "~сала", "~сало", "~сали";
        imperative: "~ши", "~шите";
        participle: "~шущий", "~савший", "~симый", "~санный";
        gerund: "~ша", "~сав|~савши";
    ),
    // Отобразить: 39 words (вообразить, отразить, исказить, разить, сразить, водрузить).
    conjugation!("отобразить", 39, "~зить";
        infinitive: "~зить";
        non_past_first: "~жу", "~зим";
        non_past_second: "~зишь", "~зите";
        non_past_third: "~зит", "~зят";
        past: "~зил", "~зила", "~зило", "~зили";
        imperative: "~зи", "~зите";
        participle: "~зящий", "~зивший", "~зимый", "~жённый";
        gerund: "~зя", "~зив|~зивши";
    ),
    // Сбиться: 39 words (влиться, сбиться, развиться, взвиться, разбиться, отбиться).
    conjugation!("сбиться", 39, "~~ить~я";
        infinitive: "~~ить~я";
        non_past_first: "~о~ью~ь", "~о~ьём~я";
        non_past_second: "~о~ьёшь~я", "~о~ьёте~ь";
        non_past_third: "~о~ьёт~я", "~о~ьют~я";
        past: "~~ил~я", "~~ила~ь", "~~ило~ь", "~~или~ь";
        imperative: "~~ей~я", "~~ейте~ь";
        participle: "~е~ущий~я", "~~ивший~я", "~~ить~я", "~о~~анный";
        gerund: "~е~я~ь", "~~ивши~ь";
    ),
    // Защитить: 39 words (запретить, прекратить, посетить, защитить, сократить, воспретить).
    conjugation!("защитить", 39, "~тить";
        infinitive: "~тить";
        non_past_first: "~щу", "~тим";
        non_past_second: "~тишь", "~тите";
        non_past_third: "~тит", "~тят";
        past: "~тил", "~тила", "~тило", "~тили";
        imperative: "~ти", "~тите";
        participle: "~тящий", "~тивший", "~тимый", "~щённый";
        gerund: "~тя", "~тив|~тивши";
    ),
    // Обратиться: 37 words (превратиться, обратиться, прекратиться, возвратиться, смутиться, возмутиться).
    conjugation!("обратиться", 37, "~тить~я";
        infinitive: "~тить~я";
        non_past_first: "~щу~ь", "~тим~я";
        non_past_second: "~тишь~я", "~тите~ь";
        non_past_third: "~тит~я", "~тят~я";
        past: "~тил~я", "~тила~ь", "~тило~ь", "~тили~ь";
        imperative: "~ти~ь", "~тите~ь";
        participle: "~тящий~я", "~тивший~я", "~е~емый", "~ё~нный";
        gerund: "~тя~ь", "~тивши~ь|~тя~ь";
    ),
    // Трястись: 36 words (трястись, пастись, сотрястись, спастись, соблюстись, затрястись).
    conjugation!("трястись", 36, "~ти~ь";
        infinitive: "~ти~ь";
        non_past_first: "~у~ь", "~ём~я";
        non_past_second: "~ёшь~я", "~ёте~ь";
        non_past_third: "~ёт~я", "~ут~я";
        past: "~~я", "~ла~ь", "~ло~ь", "~ли~ь";
        imperative: "~и~ь", "~ите~ь";
        participle: "~ущий~я", "~ший~я", "~е~емый", "~ё~нный";
        gerund: "~я~ь", "~ши~ь";
    ),
    // Загореться: 36 words (рассмотреться, смотреться, загореться, предусмотреться, просмотреться, присмотреться).
    conjugation!("загореться", 36, "~еть~я";
        infinitive: "~еть~я";
        non_past_first: "~ю~ь", "~им~я";
        non_past_second: "~ишь~я", "~ите~ь";
        non_past_third: "~ит~я", "~ят~я";
        past: "~ел~я", "~ела~ь", "~ело~ь", "~ели~ь";
        imperative: "~и~ь", "~ите~ь";
        participle: "~ящий~я", "~евший~я", "~е~емый", "~ё~нный";
        gerund: "~я~ь", "~евши~ь";
    ),
    // Гореть: 35 words (сгореть, смотреть, гореть, посмотреть, велеть, загореть).
    conjugation!("гореть", 35, "~еть";
        infinitive: "~еть";
        non_past_first: "~ю", "~им";
        non_past_second: "~ишь", "~ите";
        non_past_third: "~ит", "~ят";
        past: "~ел", "~ела", "~ело", "~ели";
        imperative: "~и", "~ите";
        participle: "~ящий", "~евший", "~имый", "~енный";
        gerund: "~я", "~ев|~евши";
    ),
    // Трепаться: 34 words (колебаться, поколебаться, трепаться, истрепаться, заколебаться, затрепаться).
    conjugation!("трепаться", 34, "~ать~я";
        infinitive: "~ать~я";
        non_past_first: "~лю~ь", "~лем~я";
        non_past_second: "~лешь~я", "~лете~ь";
        non_past_third: "~лет~я", "~лют~я";
        past: "~ал~я", "~ала~ь", "~ало~ь", "~али~ь";
        imperative: "~ли~ь", "~лите~ь";
        participle: "~лющий~я", "~авший~я", "~е~емый", "~ё~нный";
        gerund: "~ля~ь", "~авши~ь";
    ),
    // Расчесать: 34 words (чесать, тесать, расчесать, почесать, причесать, обтесать).
    conjugation!("расчесать", 34, "~есать";
        infinitive: "~есать";
        non_past_first: "~ешу", "~ешем";
        non_past_second: "~ешешь", "~ешете";
        non_past_third: "~ешет", "~ешут";
        past: "~есал", "~есала", "~есало", "~есали";
        imperative: "~еши", "~ешите";
        participle: "~ешущий", "~есавший", "~емый", "~ёсанный";
        gerund: "~еша", "~есав|~есавши";
    ),
    // Пыхать: 34 words (махать, брехать, пахать, помахать, отмахать, замахать).
    conjugation!("пыхать", 34, "~хать";
        infinitive: "~хать";
        non_past_first: "~шу", "~шем";
        non_past_second: "~шешь", "~шете";
        non_past_third: "~шет", "~шут";
        past: "~хал", "~хала", "~хало", "~хали";
        imperative: "~ши", "~шите";
        participle: "~шущий", "~хавший", "~симый", "~ханный";
        gerund: "~ша", "~хав|~хавши";
    ),
    // Клевать: 33 words (блевать, плевать, клевать, наплевать, заплевать, поплевать).
    conjugation!("клевать", 33, "~евать";
        infinitive: "~евать";
        non_past_first: "~юю", "~юём";
        non_past_second: "~юёшь", "~юёте";
        non_past_third: "~юёт", "~юют";
        past: "~евал", "~евала", "~евало", "~евали";
        imperative: "~юй", "~юйте";
        participle: "~юющий", "~евавший", "~юемый", "~ёванный";
        gerund: "~юя", "~евав|~евавши";
    ),
    // Станцевать: 33 words (танцевать, потанцевать, облицевать, вальцевать, станцевать, взлупцевать).
    conjugation!("станцевать", 33, "~евать";
        infinitive: "~евать";
        non_past_first: "~ую", "~уем";
        non_past_second: "~уешь", "~уете";
        non_past_third: "~ует", "~уют";
        past: "~евал", "~евала", "~евало", "~евали";
        imperative: "~уй", "~уйте";
        participle: "~ующий", "~евавший", "~уемый", "~ованный";
        gerund: "~уя", "~евав|~евавши";
    ),
    // Сплести: 32 words (сплести, расплести, подмести, взгнести, помести, смести).
    conjugation!("сплести", 32, "~ести";
        infinitive: "~ести";
        non_past_first: "~ету", "~етём";
        non_past_second: "~етёшь", "~етёте";
        non_past_third: "~етёт", "~етут";
        past: "~ёл", "~ела", "~ело", "~ели";
        imperative: "~ети", "~етите";
        participle: "~етущий", "~ётший", "~ебомый", "~етённый";
        gerund: "~етя", "~етя|~ётши";
    ),
    // Резаться: 32 words (врезаться, резаться, мазаться, нарезаться, размазаться, измазаться).
    conjugation!("резаться", 32, "~зать~я";
        infinitive: "~зать~я";
        non_past_first: "~жу~ь", "~жем~я";
        non_past_second: "~жешь~я", "~жете~ь";
        non_past_third: "~жет~я", "~жут~я";
        past: "~зал~я", "~зала~ь", "~зало~ь", "~зали~ь";
        imperative: "~жь~я", "~жьте~ь";
        participle: "~жущий~я", "~завший~я", "~е~емый", "~ё~нный";
        gerund: "~жа~ь", "~завши~ь";
    ),
    // Отвоеваться: 32 words (отвоеваться, навоеваться, довоеваться, развоеваться, изгореваться, нагореваться).
    conjugation!("отвоеваться", 32, "~евать~я";
        infinitive: "~евать~я";
        non_past_first: "~юю~ь", "~юем~я";
        non_past_second: "~юешь~я", "~юете~ь";
        non_past_third: "~юет~я", "~юют~я";
        past: "~евал~я", "~евала~ь", "~евало~ь", "~евали~ь";
        imperative: "~юй~я", "~юйте~ь";
        participle: "~юющий~я", "~евавший~я", "~е~емый", "~ё~нный";
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
    // Сковать: 30 words (ковать, подковать, совать, основать, сновать, приковать).
    conjugation!("сковать", 30, "~овать";
        infinitive: "~овать";
        non_past_first: "~ую", "~уём";
        non_past_second: "~уёшь", "~уёте";
        non_past_third: "~уёт", "~уют";
        past: "~овал", "~овала", "~овало", "~овали";
        imperative: "~уй", "~уйте";
        participle: "~ующий", "~овавший", "~уемый", "~ованный";
        gerund: "~уя", "~овав|~овавши";
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
    // Искусить: 30 words (пригласить, оросить, огласить, провозгласить, воскресить, вопросить).
    conjugation!("искусить", 30, "~сить";
        infinitive: "~сить";
        non_past_first: "~шу", "~сим";
        non_past_second: "~сишь", "~сите";
        non_past_third: "~сит", "~сят";
        past: "~сил", "~сила", "~сило", "~сили";
        imperative: "~си", "~сите";
        participle: "~сящий", "~сивший", "~симый", "~шённый";
        gerund: "~ся", "~сив|~сивши";
    ),
    // Распасться: 29 words (красться, попасться, распасться, подкрасться, вкрасться, спасться).
    conjugation!("распасться", 29, "~~ться";
        infinitive: "~~ться";
        non_past_first: "~ду~ь", "~дём~я";
        non_past_second: "~дёшь~я", "~дёте~ь";
        non_past_third: "~дёт~я", "~дут~я";
        past: "~л~я", "~ла~ь", "~ло~ь", "~ли~ь";
        imperative: "~ди~ь", "~дите~ь";
        participle: "~дущий~я", "~вший~я", "~е~емый", "~ё~нный";
        gerund: "~дя~ь", "~вши~ь";
    ),
    // Вывестись: 28 words (вестись, развестись, произвестись, свестись, довестись, привестись).
    conjugation!("вывестись", 28, "~е~тись";
        infinitive: "~е~тись";
        non_past_first: "~еду~ь", "~едём~я";
        non_past_second: "~едёшь~я", "~едёте~ь";
        non_past_third: "~едёт~я", "~едут~я";
        past: "~ёл~я", "~ела~ь", "~ело~ь", "~ели~ь";
        imperative: "~еди~ь", "~едите~ь";
        participle: "~етущий~я", "~едший~я", "~е~емый", "~ё~нный";
        gerund: "~етя~ь", "~едя~ь";
    ),
    // Зайти: 27 words (произойти, найти, пойти, дойти, выйти, прийти).
    conjugation!("зайти", 27, "~йти";
        infinitive: "~йти";
        non_past_first: "~йду", "~йдём";
        non_past_second: "~йдёшь", "~йдёте";
        non_past_third: "~йдёт", "~йдут";
        past: "~шёл", "~шла", "~шло", "~шли";
        imperative: "~йди", "~йдите";
        participle: "~ющий", "~шедший", "~емый", "~йдённый";
        gerund: "~я", "~йдя|~шедши";
    ),
    // Набраться: 27 words (драться, собраться, придраться, добраться, браться, убраться).
    conjugation!("набраться", 27, "~~ать~я";
        infinitive: "~~ать~я";
        non_past_first: "~е~у~ь", "~е~ём~я";
        non_past_second: "~е~ёшь~я", "~е~ёте~ь";
        non_past_third: "~е~ёт~я", "~е~ут~я";
        past: "~~ал~я|~~ал~я", "~~ала~ь", "~~ало~ь|~~ало~ь", "~~али~ь|~~али~ь";
        imperative: "~е~и~ь", "~е~ите~ь";
        participle: "~е~ущий~я", "~~авший~я", "~~ать~я", "~о~~анный";
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
        participle: "~чущий", "~кавший", "~тимый", "~канный";
        gerund: "~ча", "~кав|~кавши";
    ),
    // Просвистеть: 26 words (блестеть, свистеть, хрустеть, заблестеть, насвистеть, посвистеть).
    conjugation!("просвистеть", 26, "~стеть";
        infinitive: "~стеть";
        non_past_first: "~щу", "~стим";
        non_past_second: "~стишь", "~стите";
        non_past_third: "~стит", "~стят";
        past: "~стел", "~стела", "~стело", "~стели";
        imperative: "~сти", "~стите";
        participle: "~стящий", "~стевший", "~стимый", "~щённый";
        gerund: "~стя", "~стев|~стевши";
    ),
    // Упереться: 26 words (запереться, упереться, опереться, простереться, тереться, потереться).
    conjugation!("упереться", 26, "~е~еть~я";
        infinitive: "~е~еть~я";
        non_past_first: "~~у~ь", "~~ём~я";
        non_past_second: "~~ёшь~я", "~~ёте~ь";
        non_past_third: "~~ёт~я", "~~ут~я";
        past: "~ё~~я", "~ё~ла~ь", "~ё~ло~ь", "~ё~ли~ь";
        imperative: "~~и~ь", "~~ите~ь";
        participle: "~~ущий~я", "~ё~ший~я", "~е~еть~я", "~о~~анный";
        gerund: "~е~я~ь", "~ё~ши~ь";
    ),
    // Терпеться: 26 words (терпеться, натерпеться, обтерпеться, притерпеться, раскипеться, расшуметься).
    conjugation!("терпеться", 26, "~еть~я";
        infinitive: "~еть~я";
        non_past_first: "~лю~ь", "~им~я";
        non_past_second: "~ишь~я", "~ите~ь";
        non_past_third: "~ит~я", "~ят~я";
        past: "~ел~я", "~ела~ь", "~ело~ь", "~ели~ь";
        imperative: "~и~ь", "~ите~ь";
        participle: "~ящий~я", "~евший~я", "~е~емый", "~ё~нный";
        gerund: "~я~ь", "~евши~ь";
    ),
    // Плеваться: 26 words (плеваться, расплеваться, наклеваться, наплеваться, облеваться, обплеваться).
    conjugation!("плеваться", 26, "~евать~я";
        infinitive: "~евать~я";
        non_past_first: "~юю~ь", "~юём~я";
        non_past_second: "~юёшь~я", "~юёте~ь";
        non_past_third: "~юёт~я", "~юют~я";
        past: "~евал~я", "~евала~ь", "~евало~ь", "~евали~ь";
        imperative: "~юй~я", "~юйте~ь";
        participle: "~юющий~я", "~евавший~я", "~е~емый", "~ё~нный";
        gerund: "~юя~ь", "~евавши~ь";
    ),
    // Стричься: 26 words (напрячься, достичься, стричься, остричься, обстричься, подстричься).
    conjugation!("стричься", 26, "~чь~я";
        infinitive: "~чь~я";
        non_past_first: "~гу~ь", "~жём~я";
        non_past_second: "~жёшь~я", "~жёте~ь";
        non_past_third: "~жёт~я", "~гут~я";
        past: "~г~я", "~гла~ь", "~гло~ь", "~гли~ь";
        imperative: "~ги~ь", "~гите~ь";
        participle: "~гущий~я", "~гший~я", "~е~емый", "~ё~нный";
        gerund: "~я~ь", "~гши~ь";
    ),
    // Наплыть: 25 words (жить, плыть, отплыть, ожить, зажить, пожить).
    conjugation!("наплыть", 25, "~ть";
        infinitive: "~ть";
        non_past_first: "~ву", "~вём";
        non_past_second: "~вёшь", "~вёте";
        non_past_third: "~вёт", "~вут";
        past: "~л", "~ла", "~ло", "~ли";
        imperative: "~ви", "~вите";
        participle: "~вущий", "~вший", "~емый", "~тый";
        gerund: "~вя", "~в|~вши";
    ),
    // Виснуть: 25 words (исчезнуть, привыкнуть, крепнуть, гибнуть, дрыхнуть, гаснуть).
    conjugation!("виснуть", 25, "~нуть";
        infinitive: "~нуть";
        non_past_first: "~ну", "~нем";
        non_past_second: "~нешь", "~нете";
        non_past_third: "~нет", "~нут";
        past: "~|~нул", "~ла", "~ло", "~ли";
        imperative: "~ни", "~ните";
        participle: "~нущий", "~нувший", "~нимый", "~нутый";
        gerund: "~ня", "~нув|~нувши";
    ),
    // Разлететься: 24 words (вертеться, разлететься, завертеться, отвертеться, довертеться, навертеться).
    conjugation!("разлететься", 24, "~теть~я";
        infinitive: "~теть~я";
        non_past_first: "~чу~ь", "~тим~я";
        non_past_second: "~тишь~я", "~тите~ь";
        non_past_third: "~тит~я", "~тят~я";
        past: "~тел~я", "~тела~ь", "~тело~ь", "~тели~ь";
        imperative: "~ти~ь", "~тите~ь";
        participle: "~тящий~я", "~тевший~я", "~е~емый", "~ё~нный";
        gerund: "~тя~ь", "~тевши~ь";
    ),
    // Расплыться: 24 words (житься, прижиться, расплыться, ужиться, разжиться, сжиться).
    conjugation!("расплыться", 24, "~ть~я";
        infinitive: "~ть~я";
        non_past_first: "~ву~ь", "~вём~я";
        non_past_second: "~вёшь~я", "~вёте~ь";
        non_past_third: "~вёт~я", "~вут~я";
        past: "~л~я|~л~я", "~ла~ь", "~ло~ь|~ло~ь", "~ли~ь";
        imperative: "~ви~ь", "~вите~ь";
        participle: "~ющий~я", "~вший~я", "~е~емый", "~ё~нный";
        gerund: "~я~ь", "~вши~ь";
    ),
    // Вторгнуться: 24 words (вторгнуться, подвергнуться, проникнуться, слипнуться, выдохнуться, смеркнуться).
    conjugation!("вторгнуться", 24, "~нуть~я";
        infinitive: "~нуть~я";
        non_past_first: "~ну~ь", "~нем~я";
        non_past_second: "~нешь~я", "~нете~ь";
        non_past_third: "~нет~я", "~нут~я";
        past: "~~я", "~ла~ь", "~ло~ь", "~ли~ь";
        imperative: "~ни~ь", "~ните~ь";
        participle: "~нущий~я", "~ший~я", "~е~емый", "~ё~нный";
        gerund: "~ня~ь", "~ши~ь";
    ),
    // Заесть: 23 words (есть, съесть, поесть, надоесть, выесть, разъесть).
    conjugation!("заесть", 23, "~сть";
        infinitive: "~сть";
        non_past_first: "~м", "~дим";
        non_past_second: "~шь", "~дите";
        non_past_third: "~ст", "~дят";
        past: "~л", "~ла", "~ло", "~ли";
        imperative: "~шь", "~шьте";
        participle: "~дящий", "~вший", "~домый", "~денный";
        gerund: "~дя", "~в|~вши";
    ),
    // Затереть: 22 words (упереть, распростереть, потереть, протереть, простереть, утереть).
    conjugation!("затереть", 22, "~е~еть";
        infinitive: "~е~еть";
        non_past_first: "~~у", "~~ём";
        non_past_second: "~~ёшь", "~~ёте";
        non_past_third: "~~ёт", "~~ут";
        past: "~ё~", "~ё~ла", "~ё~ло", "~ё~ли";
        imperative: "~~и", "~~ите";
        participle: "~~ущий", "~ё~ший", "~е~омый", "~ё~тый";
        gerund: "~е~я", "~е~ев|~ё~ши";
    ),
    // Отдаться: 21 words (издаться, удаться, сдаться, раздаться, выдаться, даться).
    conjugation!("отдаться", 21, "~ть~я";
        infinitive: "~ть~я";
        non_past_first: "~м~я", "~дим~я";
        non_past_second: "~шь~я", "~дите~ь";
        non_past_third: "~~тся", "~дут~я";
        past: "~л~я|~л~я", "~ла~ь", "~ло~ь|~ло~ь", "~ли~ь|~ли~ь";
        imperative: "~й~я", "~йте~ь";
        participle: "~ющий~я", "~вший~я", "~е~емый", "~ё~нный";
        gerund: "~я~ь", "~вши~ь";
    ),
    // Дорасти: 21 words (расти, возрасти, порасти, зарасти, нарасти, подрасти).
    conjugation!("дорасти", 21, "~а~ти";
        infinitive: "~а~ти";
        non_past_first: "~а~ту", "~а~тём";
        non_past_second: "~а~тёшь", "~а~тёте";
        non_past_third: "~а~тёт", "~а~тут";
        past: "~о~", "~о~ла", "~о~ло", "~о~ли";
        imperative: "~а~ти", "~а~тите";
        participle: "~а~тущий", "~о~ший", "~е~емый", "~о~тённый";
        gerund: "~а~тя", "~о~ши";
    ),
    // Забыть: 21 words (быть, прибыть, забыть, добыть, побыть, сбыть).
    conjugation!("забыть", 21, "~ыть";
        infinitive: "~ыть";
        non_past_first: "~уду", "~удем";
        non_past_second: "~удешь", "~удете";
        non_past_third: "~удет", "~удут";
        past: "~ыл", "~ыла", "~ыло", "~ыли";
        imperative: "~удь", "~удьте";
        participle: "~оющий", "~ывший", "~оемый", "~ытый";
        gerund: "~оя", "~ыв|~ывши";
    ),
    // Унестись: 21 words (произнестись, донестись, отнестись, пронестись, понестись, перенестись).
    conjugation!("унестись", 21, "~е~ти~ь";
        infinitive: "~е~ти~ь";
        non_past_first: "~е~у~ь", "~е~ём~я";
        non_past_second: "~е~ёшь~я", "~е~ёте~ь";
        non_past_third: "~е~ёт~я", "~е~ут~я";
        past: "~ё~~я", "~е~ла~ь", "~е~ло~ь", "~е~ли~ь";
        imperative: "~е~и~ь", "~е~ите~ь";
        participle: "~е~ущий~я", "~ё~ший~я", "~е~ти~ь", "~о~~анный";
        gerund: "~е~я~ь", "~е~я~ь|~ё~ши~ь";
    ),
    // Щипать: 21 words (дремать, задремать, щипать, заколебать, уписывать, щепать).
    conjugation!("щипать", 21, "~ать";
        infinitive: "~ать";
        non_past_first: "~лю", "~лем";
        non_past_second: "~лешь", "~лете";
        non_past_third: "~лет", "~лют";
        past: "~ал", "~ала", "~ало", "~али";
        imperative: "~ли", "~лите";
        participle: "~лющий", "~авший", "~аемый", "~анный";
        gerund: "~ля", "~ав|~авши";
    ),
    // Набежать: 20 words (бежать, побежать, избежать, выбежать, убежать, сбежать).
    conjugation!("набежать", 20, "~жать";
        infinitive: "~жать";
        non_past_first: "~гу", "~жим";
        non_past_second: "~жишь", "~жите";
        non_past_third: "~жит", "~гут";
        past: "~жал", "~жала", "~жало", "~жали";
        imperative: "~ги", "~гите";
        participle: "~гущий", "~жавший", "~емый", "~женный";
        gerund: "~я", "~жав|~жавши";
    ),
    // Расстаться: 20 words (остаться, расстаться, одеться, раздеться, переодеться, достаться).
    conjugation!("расстаться", 20, "~ть~я";
        infinitive: "~ть~я";
        non_past_first: "~ну~ь", "~нем~я";
        non_past_second: "~нешь~я", "~нете~ь";
        non_past_third: "~нет~я", "~нут~я";
        past: "~л~я", "~ла~ь", "~ло~ь", "~ли~ь";
        imperative: "~нь~я", "~ньте~ь";
        participle: "~ющий~я", "~вший~я", "~е~емый", "~ё~нный";
        gerund: "~я~ь", "~вши~ь";
    ),
    // Грызть: 20 words (грызть, обгрызть, огрызть, догрызть, загрызть, изгрызть).
    conjugation!("грызть", 20, "~ть";
        infinitive: "~ть";
        non_past_first: "~у", "~ём";
        non_past_second: "~ёшь", "~ёте";
        non_past_third: "~ёт", "~ут";
        past: "~", "~ла", "~ло", "~ли";
        imperative: "~и", "~ите";
        participle: "~ущий", "~ший", "~омый", "~енный";
        gerund: "~я", "~ши";
    ),
    // Лезть: 20 words (залезть, лезть, полезть, вылезть, влезть, слезть).
    conjugation!("лезть", 20, "~ть";
        infinitive: "~ть";
        non_past_first: "~у", "~ем";
        non_past_second: "~ешь", "~ете";
        non_past_third: "~ет", "~ут";
        past: "~", "~ла", "~ло", "~ли";
        imperative: "~ь", "~ьте";
        participle: "~ущий", "~ший", "~омый", "~енный";
        gerund: "~я", "~ши";
    ),
    // Зашибить: 20 words (вышибить, зашибить, ушибить, сшибить, прошибить, пришибить).
    conjugation!("зашибить", 20, "~ить";
        infinitive: "~ить";
        non_past_first: "~у", "~ём";
        non_past_second: "~ёшь", "~ёте";
        non_past_third: "~ёт", "~ут";
        past: "~", "~ла", "~ло", "~ли";
        imperative: "~и", "~ите";
        participle: "~ащий", "~ивший", "~имый", "~ленный";
        gerund: "~а", "~ив|~ивши";
    ),
    // Занять: 19 words (понять, занять, нанять, унять, донять, перенять).
    conjugation!("занять", 19, "~нять";
        infinitive: "~нять";
        non_past_first: "~йму", "~ймём";
        non_past_second: "~ймёшь", "~ймёте";
        non_past_third: "~ймёт", "~ймут";
        past: "~нял", "~няла", "~няло", "~няли";
        imperative: "~йми", "~ймите";
        participle: "~ющий", "~нявший", "~емый", "~нятый";
        gerund: "~я", "~няв|~нявши";
    ),
    // Отнять: 19 words (поднять, снять, обнять, отнять, приподнять, разнять).
    conjugation!("отнять", 19, "~ять";
        infinitive: "~ять";
        non_past_first: "~иму", "~имем";
        non_past_second: "~имешь", "~имете";
        non_past_third: "~имет", "~имут";
        past: "~ял", "~яла", "~яло", "~яли";
        imperative: "~ими", "~имите";
        participle: "~нущий", "~явший", "~емый", "~ятый";
        gerund: "~ня", "~яв|~явши";
    ),
    // Лечь: 19 words (лечь, прилечь, залечь, облечь, пролечь, налечь).
    conjugation!("лечь", 19, "~ечь";
        infinitive: "~ечь";
        non_past_first: "~ягу", "~яжем";
        non_past_second: "~яжешь", "~яжете";
        non_past_third: "~яжет", "~ягут";
        past: "~ёг", "~егла", "~егло", "~егли";
        imperative: "~яг", "~ягте";
        participle: "~егущий", "~ёгший", "~егомый", "~ежённый";
        gerund: "~я", "~ёгши";
    ),
    // Плескаться: 19 words (плескаться, отыскаться, выплескаться, доискаться, обыскаться, полоскаться).
    conjugation!("плескаться", 19, "~~каться";
        infinitive: "~~каться";
        non_past_first: "~щу~ь", "~щем~я";
        non_past_second: "~щешь~я", "~щете~ь";
        non_past_third: "~щет~я", "~щут~я";
        past: "~~кался", "~~калась", "~~калось", "~~кались";
        imperative: "~щи~ь", "~щите~ь";
        participle: "~щущий~я", "~~кавшийся", "~е~емый", "~ё~нный";
        gerund: "~ща~ь", "~~кавшись";
    ),
    // Просмотреть: 18 words (презреть, рассмотреть, предусмотреть, осмотреть, подсмотреть, присмотреть).
    conjugation!("просмотреть", 18, "~еть";
        infinitive: "~еть";
        non_past_first: "~ю", "~им";
        non_past_second: "~ишь", "~ите";
        non_past_third: "~ит", "~ят";
        past: "~ел", "~ела", "~ело", "~ели";
        imperative: "~и", "~ите";
        participle: "~ящий", "~евший", "~имый", "~енный";
        gerund: "~я", "~ев|~евши";
    ),
    // Сволочь: 18 words (сволочь, волочь, приволочь, поволочь, заволочь, обволочь).
    conjugation!("сволочь", 18, "~чь";
        infinitive: "~чь";
        non_past_first: "~ку", "~чём";
        non_past_second: "~чёшь", "~чёте";
        non_past_third: "~чёт", "~кут";
        past: "~к", "~кла", "~кло", "~кли";
        imperative: "~ки", "~ките";
        participle: "~кущий", "~кший", "~тимый", "~чённый";
        gerund: "~тя", "~кши";
    ),
    // Потчевать: 18 words (потчевать, выкорчевать, вымежевать, запотчевать, напотчевать, отпотчевать).
    conjugation!("потчевать", 18, "~евать";
        infinitive: "~евать";
        non_past_first: "~ую", "~уем";
        non_past_second: "~уешь", "~уете";
        non_past_third: "~ует", "~уют";
        past: "~евал", "~евала", "~евало", "~евали";
        imperative: "~уй", "~уйте";
        participle: "~ующий", "~евавший", "~уемый", "~еванный";
        gerund: "~уя", "~евав|~евавши";
    ),
    // Въесться: 18 words (наесться, въесться, объесться, приесться, взъесться, заесться).
    conjugation!("въесться", 18, "~~ться";
        infinitive: "~~ться";
        non_past_first: "~м~я", "~дим~я";
        non_past_second: "~шь~я", "~дите~ь";
        non_past_third: "~~тся", "~дят~я";
        past: "~л~я", "~ла~ь", "~ло~ь", "~ли~ь";
        imperative: "~шь~я", "~шьте~ь";
        participle: "~дящий~я", "~вший~я", "~е~емый", "~ё~нный";
        gerund: "~дя~ь", "~вши~ь";
    ),
    // Устояться: 17 words (бояться, состояться, забояться, побояться, устояться, застояться).
    conjugation!("устояться", 17, "~ять~я";
        infinitive: "~ять~я";
        non_past_first: "~ю~ь", "~им~я";
        non_past_second: "~ишь~я", "~ите~ь";
        non_past_third: "~ит~я", "~ят~я";
        past: "~ял~я", "~яла~ь", "~яло~ь", "~яли~ь";
        imperative: "~й~я", "~йте~ь";
        participle: "~ящий~я", "~явший~я", "~е~емый", "~ё~нный";
        gerund: "~я~ь", "~явши~ь";
    ),
    // Трепетать: 17 words (клеветать, трепетать, роптать, скрежетать, затрепетать, оклеветать).
    conjugation!("трепетать", 17, "~тать";
        infinitive: "~тать";
        non_past_first: "~щу", "~щем";
        non_past_second: "~щешь", "~щете";
        non_past_third: "~щет", "~щут";
        past: "~тал", "~тала", "~тало", "~тали";
        imperative: "~щи", "~щите";
        participle: "~щущий", "~тавший", "~скаемый", "~танный";
        gerund: "~ща", "~тав|~тавши";
    ),
    // Жать: 17 words (нажать, выжать, жать, пожать, зажать, прижать).
    conjugation!("жать", 17, "~ать";
        infinitive: "~ать";
        non_past_first: "~му", "~мём";
        non_past_second: "~мёшь", "~мёте";
        non_past_third: "~мёт", "~мут";
        past: "~ал", "~ала", "~ало", "~али";
        imperative: "~ми", "~мите";
        participle: "~мущий", "~авший", "~имый", "~атый";
        gerund: "~мя", "~ав|~авши";
    ),
    // Вгрызться: 17 words (вгрызться, нагрызться, отгрызться, погрызться, разлезться, прогрызться).
    conjugation!("вгрызться", 17, "~ть~я";
        infinitive: "~ть~я";
        non_past_first: "~у~ь", "~ём~я";
        non_past_second: "~ёшь~я", "~ёте~ь";
        non_past_third: "~ёт~я", "~ут~я";
        past: "~~я", "~ла~ь", "~ло~ь", "~ли~ь";
        imperative: "~и~ь", "~ите~ь";
        participle: "~ущий~я", "~ший~я", "~е~емый", "~ё~нный";
        gerund: "~я~ь", "~ши~ь";
    ),
    // Уебать: 17 words (заебать, проебать, поебать, разъебать, наебать, объебать).
    conjugation!("уебать", 17, "~е~ать";
        infinitive: "~е~ать";
        non_past_first: "~е~у", "~е~ём";
        non_past_second: "~е~ёшь", "~е~ёте";
        non_past_third: "~е~ёт", "~е~ут";
        past: "~е~ал", "~е~ала", "~е~ало", "~е~али";
        imperative: "~е~и", "~е~ите";
        participle: "~е~ущий", "~е~авший", "~е~имый", "~ё~анный";
        gerund: "~е~я", "~е~ав|~е~авши";
    ),
    // Колыхаться: 17 words (колыхаться, набрехаться, напахаться, отбрехаться, размахаться, выпахаться).
    conjugation!("колыхаться", 17, "~хать~я";
        infinitive: "~хать~я";
        non_past_first: "~шу~ь", "~шем~я";
        non_past_second: "~шешь~я", "~шете~ь";
        non_past_third: "~шет~я", "~шут~я";
        past: "~хал~я", "~хала~ь", "~хало~ь", "~хали~ь";
        imperative: "~ши~ь", "~шите~ь";
        participle: "~шущий~я", "~хавший~я", "~е~емый", "~ё~нный";
        gerund: "~ша~ь", "~хавши~ь";
    ),
    // Поехать: 16 words (поехать, приехать, уехать, въехать, проехать, выехать).
    conjugation!("поехать", 16, "~хать";
        infinitive: "~хать";
        non_past_first: "~ду", "~дем";
        non_past_second: "~дешь", "~дете";
        non_past_third: "~дет", "~дут";
        past: "~хал", "~хала", "~хало", "~хали";
        imperative: "~зжай", "~зжайте";
        participle: "~дущий", "~хавший", "~домый", "~ханный";
        gerund: "~дучи", "~хав|~хавши";
    ),
    // Разобрать: 16 words (вобрать, разобрать, подобрать, отобрать, ободрать, содрать).
    conjugation!("разобрать", 16, "~о~~ать";
        infinitive: "~о~~ать";
        non_past_first: "~~е~у", "~~е~ём";
        non_past_second: "~~е~ёшь", "~~е~ёте";
        non_past_third: "~~е~ёт", "~~е~ут";
        past: "~о~~ал", "~о~~ала", "~о~~ало", "~о~~али";
        imperative: "~~е~и", "~~е~ите";
        participle: "~е~ущий~я", "~о~~авший", "~о~~ать", "~о~~анный";
        gerund: "~е~я~ь", "~о~~ав|~о~~авши";
    ),
    // Выровнять: 16 words (выровнять, приравнять, прировнять, сравнять, сровнять, уравнять).
    conjugation!("выровнять", 16, "~ять";
        infinitive: "~ять";
        non_past_first: "~яю", "~яем";
        non_past_second: "~яешь", "~яете";
        non_past_third: "~яет", "~яют";
        past: "~ял", "~яла", "~яло", "~яли";
        imperative: "~яй", "~яйте";
        participle: "~яющий", "~явший", "~яемый", "~енный";
        gerund: "~яя", "~яв|~явши";
    ),
    // Трепать: 16 words (трепать, потрепать, растрепать, истрепать, встрепать, затрепать).
    conjugation!("трепать", 16, "~е~ать";
        infinitive: "~е~ать";
        non_past_first: "~е~лю", "~е~лем";
        non_past_second: "~е~лешь", "~е~лете";
        non_past_third: "~е~лет", "~е~лют";
        past: "~е~ал", "~е~ала", "~е~ало", "~е~али";
        imperative: "~е~ли", "~е~лите";
        participle: "~е~лющий", "~е~авший", "~е~имый", "~ё~анный";
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
        participle: "~кущий~я", "~кший~я", "~е~емый", "~ё~нный";
        gerund: "~я~ь", "~кши~ь";
    ),
    // Честь: 15 words (прочесть, честь, предпочесть, вычесть, учесть, почесть).
    conjugation!("честь", 15, "~есть";
        infinitive: "~есть";
        non_past_first: "~ту", "~тём";
        non_past_second: "~тёшь", "~тёте";
        non_past_third: "~тёт", "~тут";
        past: "~ёл", "~ла", "~ло", "~ли";
        imperative: "~ти", "~тите";
        participle: "~ющий", "~етший", "~емый", "~тённый";
        gerund: "~я", "~тя|~ётши";
    ),
    // Насесть: 15 words (сесть, засесть, присесть, осесть, подсесть, пересесть).
    conjugation!("насесть", 15, "~есть";
        infinitive: "~есть";
        non_past_first: "~яду", "~ядем";
        non_past_second: "~ядешь", "~ядете";
        non_past_third: "~ядет", "~ядут";
        past: "~ел", "~ела", "~ело", "~ели";
        imperative: "~ядь", "~ядьте";
        participle: "~ящий", "~евший", "~имый", "~етённый";
        gerund: "~я", "~ев|~евши";
    ),
    // Напеть: 15 words (петь, допеть, запеть, напеть, пропеть, воспеть).
    conjugation!("напеть", 15, "~еть";
        infinitive: "~еть";
        non_past_first: "~ою", "~оём";
        non_past_second: "~оёшь", "~оёте";
        non_past_third: "~оёт", "~оют";
        past: "~ел", "~ела", "~ело", "~ели";
        imperative: "~ой", "~ойте";
        participle: "~оющий", "~евший", "~имый", "~етый";
        gerund: "~оя", "~ев|~евши";
    ),
    // Видеть: 15 words (видеть, увидеть, ненавидеть, предвидеть, возненавидеть, завидеть).
    conjugation!("видеть", 15, "~деть";
        infinitive: "~деть";
        non_past_first: "~жу", "~дим";
        non_past_second: "~дишь", "~дите";
        non_past_third: "~дит", "~дят";
        past: "~дел", "~дела", "~дело", "~дели";
        imperative: "~дь", "~дьте";
        participle: "~дящий", "~девший", "~димый", "~денный";
        gerund: "~дя", "~дев|~девши";
    ),
    // Смеяться: 15 words (смеяться, засмеяться, рассмеяться, посмеяться, досмеяться, надсмеяться).
    conjugation!("смеяться", 15, "~ять~я";
        infinitive: "~ять~я";
        non_past_first: "~ю~ь", "~ём~я";
        non_past_second: "~ёшь~я", "~ёте~ь";
        non_past_third: "~ёт~я", "~ют~я";
        past: "~ял~я", "~яла~ь", "~яло~ь", "~яли~ь";
        imperative: "~й~я", "~йте~ь";
        participle: "~ющий~я", "~явший~я", "~е~емый", "~ё~нный";
        gerund: "~я~ь", "~явши~ь";
    ),
    // Гнать: 15 words (гнать, догнать, выгнать, погнать, пригнать, изгнать).
    conjugation!("гнать", 15, "~~ать";
        infinitive: "~~ать";
        non_past_first: "~о~ю", "~о~им";
        non_past_second: "~о~ишь", "~о~ите";
        non_past_third: "~о~ит", "~о~ят";
        past: "~~ал", "~~ала", "~~ало", "~~али";
        imperative: "~о~и", "~о~ите";
        participle: "~о~ущий", "~~авший", "~е~омый", "~~анный";
        gerund: "~о~я", "~~ав|~~авши";
    ),
    // Постричь: 15 words (пренебречь, стричь, подстричь, остричь, постричь, обстричь).
    conjugation!("постричь", 15, "~чь";
        infinitive: "~чь";
        non_past_first: "~гу", "~жём";
        non_past_second: "~жёшь", "~жёте";
        non_past_third: "~жёт", "~гут";
        past: "~г", "~гла", "~гло", "~гли";
        imperative: "~ги", "~гите";
        participle: "~гущий", "~гший", "~димый", "~женный";
        gerund: "~дя", "~гши";
    ),
    // Помять: 15 words (мять, помять, распять, замять, домять, вымять).
    conjugation!("помять", 15, "~ять";
        infinitive: "~ять";
        non_past_first: "~ну", "~нём";
        non_past_second: "~нёшь", "~нёте";
        non_past_third: "~нёт", "~нут";
        past: "~ял", "~яла", "~яло", "~яли";
        imperative: "~ни", "~ните";
        participle: "~нущий", "~явший", "~емый", "~ятый";
        gerund: "~ня", "~яв|~явши";
    ),
    // Зиждить: 15 words (стелить, постелить, застелить, расстелить, устелить, зиждить).
    conjugation!("зиждить", 15, "~ить";
        infinitive: "~ить";
        non_past_first: "~ю", "~ем";
        non_past_second: "~ешь", "~ете";
        non_past_third: "~ет", "~ют";
        past: "~ил", "~ила", "~ило", "~или";
        imperative: "~и", "~ите";
        participle: "~ющий", "~ивший", "~имый", "~енный";
        gerund: "~я", "~ив|~ивши";
    ),
    // Гнить: 15 words (гнить, сгнить, прогнить, загнить, догнить, изгнить).
    conjugation!("гнить", 15, "~ть";
        infinitive: "~ть";
        non_past_first: "~ю", "~ём";
        non_past_second: "~ёшь", "~ёте";
        non_past_third: "~ёт", "~ют";
        past: "~л", "~ла", "~ло", "~ли";
        imperative: "~й", "~йте";
        participle: "~ющий", "~вший", "~емый", "~нный";
        gerund: "~я", "~в|~вши";
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
        gerund: "~я", "~ёгши";
    ),
    // Прожечь: 14 words (жечь, зажечь, выжечь, пожечь, прижечь, прожечь).
    conjugation!("прожечь", 14, "~ечь";
        infinitive: "~ечь";
        non_past_first: "~гу", "~жём";
        non_past_second: "~жёшь", "~жёте";
        non_past_third: "~жёт", "~гут";
        past: "~ёг", "~гла", "~гло", "~гли";
        imperative: "~ги", "~гите";
        participle: "~гущий", "~ёгший", "~егомый", "~жённый";
        gerund: "~дя", "~ёгши";
    ),
    // Нести: 14 words (нести, везти, заебти, поебти, взъебти, уебти).
    conjugation!("нести", 14, "~е~ти";
        infinitive: "~е~ти";
        non_past_first: "~е~у", "~е~ём";
        non_past_second: "~е~ёшь", "~е~ёте";
        non_past_third: "~е~ёт", "~е~ут";
        past: "~ё~", "~е~ла", "~е~ло", "~е~ли";
        imperative: "~е~и", "~е~ите";
        participle: "~е~ущий", "~ё~ший", "~е~омый", "~е~ённый";
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
        participle: "~е~ющий", "~о~овший", "~е~емый", "~о~отый";
        gerund: "~е~я", "~о~ов|~о~овши";
    ),
    // Засыпать: 14 words (засыпать, сыпать, насыпать, высыпать, посыпать, рассыпать).
    conjugation!("засыпать", 14, "~ать";
        infinitive: "~ать";
        non_past_first: "~лю", "~лем|~ем";
        non_past_second: "~лешь|~ешь", "~лете|~ете";
        non_past_third: "~лет|~ет", "~лют";
        past: "~ал", "~ала", "~ало", "~али";
        imperative: "~ь", "~ьте";
        participle: "~лющий", "~авший", "~лемый", "~анный";
        gerund: "~ля", "~ав|~авши";
    ),
    // Застыть: 14 words (стыть, простыть, остыть, застыть, постыть, выстыть).
    conjugation!("застыть", 14, "~ть";
        infinitive: "~ть";
        non_past_first: "~ну", "~нем";
        non_past_second: "~нешь", "~нете";
        non_past_third: "~нет", "~нут";
        past: "~л", "~ла", "~ло", "~ли";
        imperative: "~нь", "~ньте";
        participle: "~нущий", "~вший", "~емый", "~тый";
        gerund: "~я", "~в|~вши";
    ),
    // Прискакать: 14 words (скакать, отскакать, заскакать, поскакать, наскакать, прискакать).
    conjugation!("прискакать", 14, "~кать";
        infinitive: "~кать";
        non_past_first: "~чу", "~чем";
        non_past_second: "~чешь", "~чете";
        non_past_third: "~чет", "~чут";
        past: "~кал", "~кала", "~кало", "~кали";
        imperative: "~чи", "~чите";
        participle: "~чущий", "~кавший", "~тимый", "~канный";
        gerund: "~ча", "~кав|~кавши";
    ),
    // Молоться: 14 words (вымолоться, молоться, замолоться, измолоться, намолоться, обмолоться).
    conjugation!("молоться", 14, "~о~оть~я";
        infinitive: "~о~оть~я";
        non_past_first: "~е~ю~ь", "~е~ем~я";
        non_past_second: "~е~ешь~я", "~е~ете~ь";
        non_past_third: "~е~ет~я", "~е~ют~я";
        past: "~о~ол~я", "~о~ола~ь", "~о~оло~ь", "~о~оли~ь";
        imperative: "~е~и~ь", "~е~ите~ь";
        participle: "~е~ющий~я", "~о~овший~я", "~о~оть~я", "~о~~анный";
        gerund: "~е~я~ь", "~о~овши~ь";
    ),
    // Просвистать: 14 words (свистать, насвистать, посвистать, дристать, просвистать, высвистать).
    conjugation!("просвистать", 14, "~стать";
        infinitive: "~стать";
        non_past_first: "~щу", "~щем";
        non_past_second: "~щешь", "~щете";
        non_past_third: "~щет", "~щут";
        past: "~стал", "~стала", "~стало", "~стали";
        imperative: "~щи", "~щите";
        participle: "~щущий", "~ставший", "~скаемый", "~станный";
        gerund: "~ща", "~став|~ставши";
    ),
    // Глохнуть: 13 words (пухнуть, сохнуть, пахнуть, глохнуть, дрогнуть, липнуть).
    conjugation!("глохнуть", 13, "~нуть";
        infinitive: "~нуть";
        non_past_first: "~ну", "~нем";
        non_past_second: "~нешь", "~нете";
        non_past_third: "~нет", "~нут";
        past: "~|~нул", "~ла", "~ло", "~ли";
        imperative: "~ни", "~ните";
        participle: "~нущий", "~ший|~нувший", "~нимый", "~нутый";
        gerund: "~ня", "~нув|~ши|~нувши";
    ),
    // Подобраться: 13 words (разобраться, взобраться, подобраться, разодраться, взодраться, вобраться).
    conjugation!("подобраться", 13, "~о~~ать~я";
        infinitive: "~о~~ать~я";
        non_past_first: "~~е~у~ь", "~~е~ём~я";
        non_past_second: "~~е~ёшь~я", "~~е~ёте~ь";
        non_past_third: "~~е~ёт~я", "~~е~ут~я";
        past: "~о~~ал~я|~о~~ал~я", "~о~~ала~ь", "~о~~ало~ь|~о~~ало~ь", "~о~~али~ь|~о~~али~ь";
        imperative: "~~е~и~ь", "~~е~ите~ь";
        participle: "~о~~ать~я", "~о~~авший~я", "~о~~ать~я", "~о~~ать~я";
        gerund: "~о~~ать~я", "~о~~авши~ь";
    ),
    // Засечь: 13 words (усечь, сечь, изречь, пересечь, пресечь, наречь).
    conjugation!("засечь", 13, "~ечь";
        infinitive: "~ечь";
        non_past_first: "~еку", "~ечём";
        non_past_second: "~ечёшь", "~ечёте";
        non_past_third: "~ечёт", "~екут";
        past: "~ёк|~ек", "~екла", "~екло", "~екли";
        imperative: "~еки", "~еките";
        participle: "~екущий", "~екший|~ёкший", "~екомый", "~ечённый";
        gerund: "~я", "~екши|~ёкши";
    ),
    // Помочь: 13 words (мочь, смочь, помочь, выстричь, изнемочь, возмочь).
    conjugation!("помочь", 13, "~чь";
        infinitive: "~чь";
        non_past_first: "~гу", "~жем";
        non_past_second: "~жешь", "~жете";
        non_past_third: "~жет", "~гут";
        past: "~г", "~гла", "~гло", "~гли";
        imperative: "~ги", "~гите";
        participle: "~гущий", "~гший", "~даемый", "~женный";
        gerund: "~жа", "~гши";
    ),
    // Толочь: 13 words (толочь, столочь, потолочь, истолочь, растолочь, втолочь).
    conjugation!("толочь", 13, "~очь";
        infinitive: "~очь";
        non_past_first: "~ку", "~чём";
        non_past_second: "~чёшь", "~чёте";
        non_past_third: "~чёт", "~кут";
        past: "~ок", "~кла", "~кло", "~кли";
        imperative: "~ки", "~ките";
        participle: "~кущий", "~окший", "~тимый", "~чённый";
        gerund: "~тя", "~окши";
    ),
    // Сопрячь: 13 words (спрячь, сопрячь, перенапрячь, отпрячь, перепрячь, подпрячь).
    conjugation!("сопрячь", 13, "~чь";
        infinitive: "~чь";
        non_past_first: "~гу", "~жём";
        non_past_second: "~жёшь", "~жёте";
        non_past_third: "~жёт", "~гут";
        past: "~г", "~гла", "~гло", "~гли";
        imperative: "~ги", "~гите";
        participle: "~гущий", "~гший", "~димый", "~жённый";
        gerund: "~дя", "~гши";
    ),
    // Сплестись: 13 words (поплестись, переплестись, заплестись, сплестись, проплестись, доплестись).
    conjugation!("сплестись", 13, "~е~тись";
        infinitive: "~е~тись";
        non_past_first: "~ету~ь", "~етём~я";
        non_past_second: "~етёшь~я", "~етёте~ь";
        non_past_third: "~етёт~я", "~етут~я";
        past: "~ёл~я", "~ела~ь", "~ело~ь", "~ели~ь";
        imperative: "~ети~ь", "~етите~ь";
        participle: "~етущий~я", "~ётший~я", "~е~емый", "~ё~нный";
        gerund: "~етя~ь", "~етя~ь";
    ),
    // Тыкаться: 13 words (расплакаться, тыкаться, плакаться, доскакаться, исплакаться, наплакаться).
    conjugation!("тыкаться", 13, "~кать~я";
        infinitive: "~кать~я";
        non_past_first: "~чу~ь", "~чем~я";
        non_past_second: "~чешь~я", "~чете~ь";
        non_past_third: "~чет~я", "~чут~я";
        past: "~кал~я", "~кала~ь", "~кало~ь", "~кали~ь";
        imperative: "~чь~я", "~чьте~ь";
        participle: "~чущий~я", "~кавший~я", "~е~емый", "~ё~нный";
        gerund: "~ча~ь", "~кавши~ь";
    ),
    // Мяться: 13 words (замяться, мяться, помяться, примяться, вымяться, домяться).
    conjugation!("мяться", 13, "~ять~я";
        infinitive: "~ять~я";
        non_past_first: "~ну~ь", "~нём~я";
        non_past_second: "~нёшь~я", "~нёте~ь";
        non_past_third: "~нёт~я", "~нут~я";
        past: "~ял~я", "~яла~ь", "~яло~ь", "~яли~ь";
        imperative: "~ни~ь", "~ните~ь";
        participle: "~нущий~я", "~явший~я", "~е~емый", "~ё~нный";
        gerund: "~ня~ь", "~явши~ь";
    ),
    // Отгрестись: 13 words (сгрестись, выгрестись, выскрестись, прискрестись, доскрестись, заскрестись).
    conjugation!("отгрестись", 13, "~е~тись";
        infinitive: "~е~тись";
        non_past_first: "~ебу~ь", "~ебём~я";
        non_past_second: "~ебёшь~я", "~ебёте~ь";
        non_past_third: "~ебёт~я", "~ебут~я";
        past: "~ёб~я", "~ебла~ь", "~ебло~ь", "~ебли~ь";
        imperative: "~еби~ь", "~ебите~ь";
        participle: "~ебущий~я", "~ёбший~я", "~е~емый", "~ё~нный";
        gerund: "~ебя~ь", "~ебя~ь";
    ),
    // Нагореть: 13 words (нагореть, отгореть, отзвенеть, подзагореть, позвенеть, пообгореть).
    conjugation!("нагореть", 13, "~еть";
        infinitive: "~еть";
        non_past_first: "~ю", "~им";
        non_past_second: "~ишь", "~ите";
        non_past_third: "~ит", "~ят";
        past: "~ел", "~ела", "~ело", "~ели";
        imperative: "~и", "~ите";
        participle: "~ящий", "~евший", "~имый", "~енный";
        gerund: "~я", "~евши|~ев";
    ),
    // Вызвать: 12 words (звать, позвать, вызвать, назвать, призвать, прозвать).
    conjugation!("вызвать", 12, "~~ать";
        infinitive: "~~ать";
        non_past_first: "~о~у", "~о~ём";
        non_past_second: "~о~ёшь", "~о~ёте";
        non_past_third: "~о~ёт", "~о~ут";
        past: "~~ал", "~~ала", "~~ало", "~~али";
        imperative: "~о~и", "~о~ите";
        participle: "~о~ущий", "~~авший", "~е~омый", "~~анный";
        gerund: "~о~я", "~~ав|~~авши";
    ),
    // Подняться: 12 words (подняться, обняться, сняться, приподняться, отняться, засняться).
    conjugation!("подняться", 12, "~ять~я";
        infinitive: "~ять~я";
        non_past_first: "~иму~ь", "~имем~я";
        non_past_second: "~имешь~я", "~имете~ь";
        non_past_third: "~имет~я", "~имут~я";
        past: "~ял~я|~ял~я", "~яла~ь", "~яло~ь|~яло~ь", "~яли~ь|~яли~ь";
        imperative: "~ими~ь", "~имите~ь";
        participle: "~ющий~я", "~явший~я", "~е~емый", "~ё~нный";
        gerund: "~я~ь", "~явши~ь";
    ),
    // Отослать: 12 words (послать, выслать, прислать, услать, подослать, сослать).
    conjugation!("отослать", 12, "~с~ать";
        infinitive: "~с~ать";
        non_past_first: "~ш~ю", "~ш~ём";
        non_past_second: "~ш~ёшь", "~ш~ёте";
        non_past_third: "~ш~ёт", "~ш~ют";
        past: "~с~ал", "~с~ала", "~с~ало", "~с~али";
        imperative: "~ш~и", "~ш~ите";
        participle: "~ющий~я", "~с~авший", "~е~емый", "~с~анный";
        gerund: "~я~ь", "~с~ав|~с~авши";
    ),
    // Брить: 12 words (брить, побрить, выбрить, обрить, добрить, забрить).
    conjugation!("брить", 12, "~ить";
        infinitive: "~ить";
        non_past_first: "~ею", "~еем";
        non_past_second: "~еешь", "~еете";
        non_past_third: "~еет", "~еют";
        past: "~ил", "~ила", "~ило", "~или";
        imperative: "~ей", "~ейте";
        participle: "~еющий", "~ивший", "~еемый", "~итый";
        gerund: "~ея", "~ив|~ивши";
    ),
    // Изобрести: 12 words (изобрести, приобрести, обрести, домести, доцвести, промести).
    conjugation!("изобрести", 12, "~ести";
        infinitive: "~ести";
        non_past_first: "~ету", "~етём";
        non_past_second: "~етёшь", "~етёте";
        non_past_third: "~етёт", "~етут";
        past: "~ёл", "~ела", "~ело", "~ели";
        imperative: "~ети", "~етите";
        participle: "~етущий", "~етший", "~ебомый", "~етённый";
        gerund: "~етя", "~етя|~етши";
    ),
    // Уняться: 12 words (заняться, наняться, поняться, уняться, доняться, переняться).
    conjugation!("уняться", 12, "~нять~я";
        infinitive: "~нять~я";
        non_past_first: "~йму~ь", "~ймём~я";
        non_past_second: "~ймёшь~я", "~ймёте~ь";
        non_past_third: "~ймёт~я", "~ймут~я";
        past: "~нял~я|~нял~я", "~няла~ь", "~няло~ь", "~няли~ь";
        imperative: "~йми~ь", "~ймите~ь";
        participle: "~ющий~я", "~нявший~я", "~е~емый", "~ё~нный";
        gerund: "~я~ь", "~нявши~ь";
    ),
    // Жевать: 12 words (жевать, пожевать, прожевать, сжевать, дожевать, зажевать).
    conjugation!("жевать", 12, "~евать";
        infinitive: "~евать";
        non_past_first: "~ую", "~уём";
        non_past_second: "~уёшь", "~уёте";
        non_past_third: "~уёт", "~уют";
        past: "~евал", "~евала", "~евало", "~евали";
        imperative: "~уй", "~уйте";
        participle: "~ующий", "~евавший", "~уемый", "~ёванный";
        gerund: "~уя", "~евав|~евавши";
    ),
    // Стынуть: 12 words (стынуть, застынуть, остынуть, простынуть, постынуть, настынуть).
    conjugation!("стынуть", 12, "~нуть";
        infinitive: "~нуть";
        non_past_first: "~ну", "~нем";
        non_past_second: "~нешь", "~нете";
        non_past_third: "~нет", "~нут";
        past: "~л", "~ла", "~ло", "~ли";
        imperative: "~нь", "~ньте";
        participle: "~нущий", "~вший", "~нимый", "~нутый";
        gerund: "~ня", "~в|~вши";
    ),
    // Бриться: 12 words (бриться, побриться, перебриться, выбриться, обриться, пробриться).
    conjugation!("бриться", 12, "~ить~я";
        infinitive: "~ить~я";
        non_past_first: "~ею~ь", "~еем~я";
        non_past_second: "~еешь~я", "~еете~ь";
        non_past_third: "~еет~я", "~еют~я";
        past: "~ил~я", "~ила~ь", "~ило~ь", "~или~ь";
        imperative: "~ей~я", "~ейте~ь";
        participle: "~еющий~я", "~ивший~я", "~е~емый", "~ё~нный";
        gerund: "~ея~ь", "~ивши~ь";
    ),
    // Морщить: 12 words (упорядочить, морщить, выпучить, сморщить, таращить, вытаращить).
    conjugation!("морщить", 12, "~ить";
        infinitive: "~ить";
        non_past_first: "~у", "~им";
        non_past_second: "~ишь", "~ите";
        non_past_third: "~ит", "~ат";
        past: "~ил", "~ила", "~ило", "~или";
        imperative: "~и|~ь", "~ьте";
        participle: "~ащий", "~ивший", "~имый", "~енный";
        gerund: "~а", "~ив|~ивши";
    ),
    // Сослаться: 12 words (сослаться, выслаться, дослаться, заслаться, ниспослаться, отослаться).
    conjugation!("сослаться", 12, "~с~ать~я";
        infinitive: "~с~ать~я";
        non_past_first: "~ш~ю~ь", "~ш~ём~я";
        non_past_second: "~ш~ёшь~я", "~ш~ёте~ь";
        non_past_third: "~ш~ёт~я", "~ш~ют~я";
        past: "~с~ал~я", "~с~ала~ь", "~с~ало~ь", "~с~али~ь";
        imperative: "~ш~и~ь", "~ш~ите~ь";
        participle: "~е~ущий~я", "~с~авший~я", "~с~ать~я", "~о~~анный";
        gerund: "~е~я~ь", "~с~авши~ь";
    ),
    // Пересидеть: 12 words (просидеть, отсидеть, усидеть, насидеть, пересидеть, засидеть).
    conjugation!("пересидеть", 12, "~деть";
        infinitive: "~деть";
        non_past_first: "~жу", "~дим";
        non_past_second: "~дишь", "~дите";
        non_past_third: "~дит", "~дят";
        past: "~дел", "~дела", "~дело", "~дели";
        imperative: "~ди", "~дите";
        participle: "~дящий", "~девший", "~димый", "~женный";
        gerund: "~дя", "~дев|~девши";
    ),
    // Спеться: 12 words (петься, спеться, допеться, распеться, напеться, воспеться).
    conjugation!("спеться", 12, "~еть~я";
        infinitive: "~еть~я";
        non_past_first: "~ою~ь", "~оём~я";
        non_past_second: "~оёшь~я", "~оёте~ь";
        non_past_third: "~оёт~я", "~оют~я";
        past: "~ел~я", "~ела~ь", "~ело~ь", "~ели~ь";
        imperative: "~ой~я", "~ойте~ь";
        participle: "~оющий~я", "~евший~я", "~е~емый", "~ё~нный";
        gerund: "~оя~ь", "~евши~ь";
    ),
    // Стереться: 12 words (стереться, отпереться, подтереться, втереться, впереться, обтереться).
    conjugation!("стереться", 12, "~~е~еть~я";
        infinitive: "~~е~еть~я";
        non_past_first: "~о~~у~ь", "~о~~ём~я";
        non_past_second: "~о~~ёшь~я", "~о~~ёте~ь";
        non_past_third: "~о~~ёт~я", "~о~~ут~я";
        past: "~~ё~~я", "~~ё~ла~ь", "~~ё~ло~ь", "~~ё~ли~ь";
        imperative: "~о~~и~ь", "~о~~ите~ь";
        participle: "~~е~еть~я", "~~ё~ший~я", "~~е~еть~я", "~~е~еть~я";
        gerund: "~~е~еть~я", "~~ё~ши~ь";
    ),
    // Стелиться: 12 words (зиждиться, расстелиться, стелиться, созиждиться, взъемлиться, постелиться).
    conjugation!("стелиться", 12, "~ить~я";
        infinitive: "~ить~я";
        non_past_first: "~ю~ь", "~ем~я";
        non_past_second: "~ешь~я", "~ете~ь";
        non_past_third: "~ет~я", "~ют~я";
        past: "~ил~я", "~ила~ь", "~ило~ь", "~или~ь";
        imperative: "~и~ь", "~ите~ь";
        participle: "~ющий~я", "~ивший~я", "~е~емый", "~ё~нный";
        gerund: "~я~ь", "~ивши~ь";
    ),
    // Подвигнуться: 12 words (свергнуться, разверзнуться, низвергнуться, подвигнуться, сверзнуться, исторгнуться).
    conjugation!("подвигнуться", 12, "~нуть~я";
        infinitive: "~нуть~я";
        non_past_first: "~ну~ь", "~нем~я";
        non_past_second: "~нешь~я", "~нете~ь";
        non_past_third: "~нет~я", "~нут~я";
        past: "~~я", "~ла~ь", "~ло~ь", "~ли~ь";
        imperative: "~ни~ь", "~ните~ь";
        participle: "~нущий~я", "~нувший~я|~ший~я", "~е~емый", "~ё~нный";
        gerund: "~ня~ь", "~ши~ь";
    ),
    // Насытить: 12 words (похитить, предвосхитить, насытить, расхитить, перенасытить, пересытить).
    conjugation!("насытить", 12, "~тить";
        infinitive: "~тить";
        non_past_first: "~щу", "~тим";
        non_past_second: "~тишь", "~тите";
        non_past_third: "~тит", "~тят";
        past: "~тил", "~тила", "~тило", "~тили";
        imperative: "~ть", "~тьте";
        participle: "~тящий", "~тивший", "~тимый", "~щенный";
        gerund: "~тя", "~тив|~тивши";
    ),
    // Дожить: 11 words (выжить, прожить, пережить, дожить, нажить, изжить).
    conjugation!("дожить", 11, "~ть";
        infinitive: "~ть";
        non_past_first: "~ву", "~вём";
        non_past_second: "~вёшь", "~вёте";
        non_past_third: "~вёт", "~вут";
        past: "~л|~л", "~ла", "~ло|~ло", "~ли|~ли";
        imperative: "~ви", "~вите";
        participle: "~вущий", "~вший", "~емый", "~тый|~тый";
        gerund: "~вя", "~в|~вши";
    ),
    // Ушибиться: 11 words (ошибиться, ушибиться, зашибиться, прошибиться, вышибиться, пришибиться).
    conjugation!("ушибиться", 11, "~ить~я";
        infinitive: "~ить~я";
        non_past_first: "~у~ь", "~ём~я";
        non_past_second: "~ёшь~я", "~ёте~ь";
        non_past_third: "~ёт~я", "~ут~я";
        past: "~~я", "~ла~ь", "~ло~ь", "~ли~ь";
        imperative: "~и~ь", "~ите~ь";
        participle: "~ащий~я", "~ивший~я", "~е~емый", "~ё~нный";
        gerund: "~а~ь", "~ивши~ь";
    ),
    // Зачесться: 11 words (учесться, расчесться, зачесться, почесться, обчесться, причесться).
    conjugation!("зачесться", 11, "~е~ться";
        infinitive: "~е~ться";
        non_past_first: "~ту~ь", "~тём~я";
        non_past_second: "~тёшь~я", "~тёте~ь";
        non_past_third: "~тёт~я", "~тут~я";
        past: "~ёл~я", "~ла~ь", "~ло~ь", "~ли~ь";
        imperative: "~ти~ь", "~тите~ь";
        participle: "~ющий~я", "~евший~я", "~е~емый", "~ё~нный";
        gerund: "~я~ь", "~тя~ь";
    ),
    // Жаться: 11 words (прижаться, жаться, пожаться, прожаться, ужаться, выжаться).
    conjugation!("жаться", 11, "~ать~я";
        infinitive: "~ать~я";
        non_past_first: "~му~ь", "~мём~я";
        non_past_second: "~мёшь~я", "~мёте~ь";
        non_past_third: "~мёт~я", "~мут~я";
        past: "~ал~я", "~ала~ь", "~ало~ь", "~али~ь";
        imperative: "~ми~ь", "~мите~ь";
        participle: "~мущий~я", "~авший~я", "~е~емый", "~ё~нный";
        gerund: "~мя~ь", "~авши~ь";
    ),
    // Пакоститься: 11 words (заневеститься, засовеститься, невеститься, скапуститься, испакоститься, чихвоститься).
    conjugation!("пакоститься", 11, "~~титься";
        infinitive: "~~титься";
        non_past_first: "~щу~ь", "~~тимся";
        non_past_second: "~~тишься", "~~титесь";
        non_past_third: "~~тится", "~~тятся";
        past: "~~тился", "~~тилась", "~~тилось", "~~тились";
        imperative: "~~ться", "~~тьтесь";
        participle: "~~тящийся", "~~тившийся", "~е~емый", "~ё~нный";
        gerund: "~~тясь", "~~тившись";
    ),
    // Ссать: 11 words (ссать, сцать, обоссать, зассать, поссать, проссать).
    conjugation!("ссать", 11, "~ать";
        infinitive: "~ать";
        non_past_first: "~у", "~ым";
        non_past_second: "~ышь", "~ыте";
        non_past_third: "~ыт", "~ут";
        past: "~ал", "~ала", "~ало", "~али";
        imperative: "~ы", "~ыте";
        participle: "~ущий", "~авший", "~имый", "~анный";
        gerund: "~я", "~ав|~авши";
    ),
    // Стоять: 10 words (стоять, настоять, постоять, состоять, противостоять, предстоять).
    conjugation!("стоять", 10, "~ять";
        infinitive: "~ять";
        non_past_first: "~ю", "~им";
        non_past_second: "~ишь", "~ите";
        non_past_third: "~ит", "~ят";
        past: "~ял", "~яла", "~яло", "~яли";
        imperative: "~й", "~йте";
        participle: "~ящий", "~явший", "~емый", "~янный";
        gerund: "~я", "~яв|~явши";
    ),
    // Беречься: 10 words (беречься, остеречься, поостеречься, постеречься, стеречься, уберечься).
    conjugation!("беречься", 10, "~ечь~я";
        infinitive: "~ечь~я";
        non_past_first: "~егу~ь", "~ежём~я";
        non_past_second: "~ежёшь~я", "~ежёте~ь";
        non_past_third: "~ежёт~я", "~егут~я";
        past: "~ёг~я", "~егла~ь", "~егло~ь", "~егли~ь";
        imperative: "~еги~ь", "~егите~ь";
        participle: "~егущий~я", "~ёгший~я", "~е~емый", "~ё~нный";
        gerund: "~я~ь", "~ёгши~ь";
    ),
    // Упрятать: 10 words (прятать, спрятать, упрятать, запрятать, припрятать, напрятать).
    conjugation!("упрятать", 10, "~тать";
        infinitive: "~тать";
        non_past_first: "~чу", "~чем";
        non_past_second: "~чешь", "~чете";
        non_past_third: "~чет", "~чут";
        past: "~тал", "~тала", "~тало", "~тали";
        imperative: "~чь", "~чьте";
        participle: "~чущий", "~тавший", "~тимый", "~танный";
        gerund: "~ча", "~тав|~тавши";
    ),
    // Вырвать: 10 words (выебать, вырвать, выждать, высосать, выжрать, выткать).
    conjugation!("вырвать", 10, "~ать";
        infinitive: "~ать";
        non_past_first: "~у", "~ем";
        non_past_second: "~ешь", "~ете";
        non_past_third: "~ет", "~ут";
        past: "~ал", "~ала", "~ало", "~али";
        imperative: "~и", "~ите";
        participle: "~ущий", "~авший", "~имый", "~анный";
        gerund: "~ая", "~ав|~авши";
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
    conjugation!("забыться", 10, "~ыть~я";
        infinitive: "~ыть~я";
        non_past_first: "~уду~ь", "~удем~я";
        non_past_second: "~удешь~я", "~удете~ь";
        non_past_third: "~удет~я", "~удут~я";
        past: "~ыл~я", "~ыла~ь", "~ыло~ь", "~ыли~ь";
        imperative: "~удь~я", "~удьте~ь";
        participle: "~оющий~я", "~ывший~я", "~е~емый", "~ё~нный";
        gerund: "~оя~ь", "~ывши~ь";
    ),
    // Нестись: 10 words (нестись, везтись, повезтись, свезтись, перевезтись, наебтись).
    conjugation!("нестись", 10, "~е~ти~ь";
        infinitive: "~е~ти~ь";
        non_past_first: "~е~у~ь", "~е~ём~я";
        non_past_second: "~е~ёшь~я", "~е~ёте~ь";
        non_past_third: "~е~ёт~я", "~е~ут~я";
        past: "~ё~~я", "~е~ла~ь", "~е~ло~ь", "~е~ли~ь";
        imperative: "~е~и~ь", "~е~ите~ь";
        participle: "~е~ущий~я", "~ё~ший~я", "~е~ти~ь", "~о~~анный";
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
        participle: "~о~ящий~я", "~~авший~я", "~~ать~я", "~о~~анный";
        gerund: "~о~я~ь", "~~авши~ь";
    ),
    // Назваться: 10 words (вызваться, назваться, зваться, дозваться, призваться, самоназваться).
    conjugation!("назваться", 10, "~~ать~я";
        infinitive: "~~ать~я";
        non_past_first: "~о~у~ь", "~о~ём~я";
        non_past_second: "~о~ёшь~я", "~о~ёте~ь";
        non_past_third: "~о~ёт~я", "~о~ут~я";
        past: "~~ал~я|~~ал~я", "~~ала~ь", "~~ало~ь|~~ало~ь", "~~али~ь|~~али~ь";
        imperative: "~о~и~ь", "~о~ите~ь";
        participle: "~о~ущий~я", "~~авший~я", "~~ать~я", "~о~~анный";
        gerund: "~о~я~ь", "~~авши~ь";
    ),
    // Избороздить: 10 words (нагромоздить, взгромоздить, избороздить, взбороздить, загромоздить, пригвоздить).
    conjugation!("избороздить", 10, "~здить";
        infinitive: "~здить";
        non_past_first: "~зжу", "~здим";
        non_past_second: "~здишь", "~здите";
        non_past_third: "~здит", "~здят";
        past: "~здил", "~здила", "~здило", "~здили";
        imperative: "~зди", "~здите";
        participle: "~дящий", "~здивший", "~димый", "~ждённый";
        gerund: "~дя", "~здив|~здивши";
    ),
    // Свянуть: 10 words (обвянуть, повянуть, привянуть, призавянуть, отвянуть, подвянуть).
    conjugation!("свянуть", 10, "~нуть";
        infinitive: "~нуть";
        non_past_first: "~ну", "~нем";
        non_past_second: "~нешь", "~нете";
        non_past_third: "~нет", "~нут";
        past: "~л", "~ла", "~ло", "~ли";
        imperative: "~нь", "~ньте";
        participle: "~нущий", "~нувший", "~нимый", "~нутый";
        gerund: "~ня", "~нувши";
    ),
    // Жеваться: 10 words (нажеваться, жеваться, дожеваться, зажеваться, пережеваться, прожеваться).
    conjugation!("жеваться", 10, "~евать~я";
        infinitive: "~евать~я";
        non_past_first: "~ую~ь", "~уём~я";
        non_past_second: "~уёшь~я", "~уёте~ь";
        non_past_third: "~уёт~я", "~уют~я";
        past: "~евал~я", "~евала~ь", "~евало~ь", "~евали~ь";
        imperative: "~уй~я", "~уйте~ь";
        participle: "~ующий~я", "~евавший~я", "~е~емый", "~ё~нный";
        gerund: "~уя~ь", "~евавши~ь";
    ),
    // Обидеться: 10 words (обидеться, навидеться, разобидеться, изобидеться, завидеться, ненавидеться).
    conjugation!("обидеться", 10, "~деть~я";
        infinitive: "~деть~я";
        non_past_first: "~жу~ь", "~дим~я";
        non_past_second: "~дишь~я", "~дите~ь";
        non_past_third: "~дит~я", "~дят~я";
        past: "~дел~я", "~дела~ь", "~дело~ь", "~дели~ь";
        imperative: "~дь~я", "~дьте~ь";
        participle: "~дящий~я", "~девший~я", "~е~емый", "~ё~нный";
        gerund: "~дя~ь", "~девши~ь";
    ),
    // Вырваться: 10 words (вырваться, выебаться, высраться, выткаться, закармливаться, высосаться).
    conjugation!("вырваться", 10, "~ать~я";
        infinitive: "~ать~я";
        non_past_first: "~у~ь", "~ем~я";
        non_past_second: "~ешь~я", "~ете~ь";
        non_past_third: "~ет~я", "~ут~я";
        past: "~ал~я", "~ала~ь", "~ало~ь", "~али~ь";
        imperative: "~и~ь", "~ите~ь";
        participle: "~ущий~я", "~авший~я", "~е~емый", "~ё~нный";
        gerund: "~ая~ь", "~авши~ь";
    ),
    // Разойтись: 9 words (найтись, прийтись, обойтись, пройтись, разойтись, сойтись).
    conjugation!("разойтись", 9, "~йти~ь";
        infinitive: "~йти~ь";
        non_past_first: "~йду~ь", "~йдём~я";
        non_past_second: "~йдёшь~я", "~йдёте~ь";
        non_past_third: "~йдёт~я", "~йдут~я";
        past: "~шёл~я", "~шла~ь", "~шло~ь", "~шли~ь";
        imperative: "~йди~ь", "~йдите~ь";
        participle: "~ющий~я", "~шедший~я", "~е~емый", "~ё~нный";
        gerund: "~я~ь", "~шедши~ь";
    ),
    // Высрать: 9 words (стонать, жаждать, застонать, простонать, высрать, возжаждать).
    conjugation!("высрать", 9, "~ать";
        infinitive: "~ать";
        non_past_first: "~у", "~ем";
        non_past_second: "~ешь", "~ете";
        non_past_third: "~ет", "~ут";
        past: "~ал", "~ала", "~ало", "~али";
        imperative: "~и", "~ите";
        participle: "~ущий", "~авший", "~имый", "~анный";
        gerund: "~ая", "~ав|~авши";
    ),
    // Стереть: 9 words (стереть, подпереть, спереть, обтереть, втереть, впереть).
    conjugation!("стереть", 9, "~~е~еть";
        infinitive: "~~е~еть";
        non_past_first: "~о~~у", "~о~~ём";
        non_past_second: "~о~~ёшь", "~о~~ёте";
        non_past_third: "~о~~ёт", "~о~~ут";
        past: "~~ё~", "~~ё~ла", "~~ё~ло", "~~ё~ли";
        imperative: "~о~~и", "~о~~ите";
        participle: "~е~ущий~я", "~~ё~ший", "~~е~еть", "~~ё~тый";
        gerund: "~е~я~ь", "~~е~ев|~~ё~ши";
    ),
    // Выспаться: 9 words (выспаться, отоспаться, доспаться, заспаться, наспаться, проспаться).
    conjugation!("выспаться", 9, "~ать~я";
        infinitive: "~ать~я";
        non_past_first: "~лю~ь", "~им~я";
        non_past_second: "~ишь~я", "~ите~ь";
        non_past_third: "~ит~я", "~ят~я";
        past: "~ал~я", "~ала~ь", "~ало~ь", "~али~ь";
        imperative: "~и~ь", "~ите~ь";
        participle: "~ащий~я", "~авший~я", "~е~емый", "~ё~нный";
        gerund: "~а~ь", "~авши~ь";
    ),
    // Застлать: 9 words (стлать, постлать, застлать, устлать, выстлать, достлать).
    conjugation!("застлать", 9, "~~ать";
        infinitive: "~~ать";
        non_past_first: "~е~ю", "~е~ем";
        non_past_second: "~е~ешь", "~е~ете";
        non_past_third: "~е~ет", "~е~ют";
        past: "~~ал", "~~ала", "~~ало", "~~али";
        imperative: "~е~и", "~е~ите";
        participle: "~е~ущий", "~~авший", "~е~омый", "~~анный";
        gerund: "~е~я", "~~ав|~~авши";
    ),
    // Размять: 9 words (смять, измять, размять, подмять, вмять, обмять).
    conjugation!("размять", 9, "~~ять";
        infinitive: "~~ять";
        non_past_first: "~о~ну", "~о~нём";
        non_past_second: "~о~нёшь", "~о~нёте";
        non_past_third: "~о~нёт", "~о~нут";
        past: "~~ял", "~~яла", "~~яло", "~~яли";
        imperative: "~о~ни", "~о~ните";
        participle: "~ющий~я", "~~явший", "~е~емый", "~~ятый";
        gerund: "~я~ь", "~~яв|~~явши";
    ),
    // Толочься: 9 words (толочься, затолочься, потолочься, истолочься, дотолочься, утолочься).
    conjugation!("толочься", 9, "~очь~я";
        infinitive: "~очь~я";
        non_past_first: "~ку~ь", "~чём~я";
        non_past_second: "~чёшь~я", "~чёте~ь";
        non_past_third: "~чёт~я", "~кут~я";
        past: "~ок~я", "~кла~ь", "~кло~ь", "~кли~ь";
        imperative: "~ки~ь", "~ките~ь";
        participle: "~кущий~я", "~окший~я", "~е~емый", "~ё~нный";
        gerund: "~я~ь", "~окши~ь";
    ),
    // Скрестись: 9 words (скрестись, грестись, огрестись, догрестись, угрестись, погрестись).
    conjugation!("скрестись", 9, "~е~тись";
        infinitive: "~е~тись";
        non_past_first: "~ебу~ь", "~ебём~я";
        non_past_second: "~ебёшь~я", "~ебёте~ь";
        non_past_third: "~ебёт~я", "~ебут~я";
        past: "~ёб~я", "~ебла~ь", "~ебло~ь", "~ебли~ь";
        imperative: "~еби~ь", "~ебите~ь";
        participle: "~ебущий~я", "~ёбший~я", "~е~емый", "~ё~нный";
        gerund: "~ебя~ь", "~ёбши~ь";
    ),
    // Зажечься: 9 words (зажечься, ожечься, нажечься, жечься, возжечься, пережечься).
    conjugation!("зажечься", 9, "~ечь~я";
        infinitive: "~ечь~я";
        non_past_first: "~гу~ь", "~жём~я";
        non_past_second: "~жёшь~я", "~жёте~ь";
        non_past_third: "~жёт~я", "~гут~я";
        past: "~ёг~я", "~гла~ь", "~гло~ь", "~гли~ь";
        imperative: "~ги~ь", "~гите~ь";
        participle: "~гущий~я", "~ёгший~я", "~е~емый", "~ё~нный";
        gerund: "~я~ь", "~ёгши~ь";
    ),
    // Выпить: 8 words (выпить, вышить, выбить, вылить, подвыпить, вывить).
    conjugation!("выпить", 8, "~ить";
        infinitive: "~ить";
        non_past_first: "~ью", "~ьем";
        non_past_second: "~ьешь", "~ьете";
        non_past_third: "~ьет", "~ьют";
        past: "~ил", "~ила", "~ило", "~или";
        imperative: "~ей", "~ейте";
        participle: "~ьющий", "~ивший", "~еемый", "~итый";
        gerund: "~ья", "~ив|~ивши";
    ),
    // Разогнать: 8 words (подогнать, обогнать, отогнать, разогнать, согнать, вогнать).
    conjugation!("разогнать", 8, "~о~~ать";
        infinitive: "~о~~ать";
        non_past_first: "~~о~ю", "~~о~им";
        non_past_second: "~~о~ишь", "~~о~ите";
        non_past_third: "~~о~ит", "~~о~ят";
        past: "~о~~ал", "~о~~ала", "~о~~ало", "~о~~али";
        imperative: "~~о~и", "~~о~ите";
        participle: "~е~ущий~я", "~о~~авший", "~о~~ать", "~о~~анный";
        gerund: "~е~я~ь", "~о~~ав|~о~~авши";
    ),
    // Отозваться: 8 words (отозваться, разогнаться, взогнаться, вогнаться, обозваться, отогнаться).
    conjugation!("отозваться", 8, "~о~~ать~я";
        infinitive: "~о~~ать~я";
        non_past_first: "~~о~ю~ь", "~~о~им~я";
        non_past_second: "~~о~ишь~я", "~~о~ите~ь";
        non_past_third: "~~о~ит~я", "~~о~ят~я";
        past: "~о~~ал~я", "~о~~ала~ь", "~о~~ало~ь|~о~~ало~ь", "~о~~али~ь|~о~~али~ь";
        imperative: "~~о~и~ь", "~~о~ите~ь";
        participle: "~о~~ать~я", "~о~~авший~я", "~о~~ать~я", "~о~~ать~я";
        gerund: "~о~~ать~я", "~о~~авши~ь";
    ),
    // Смяться: 8 words (измяться, смяться, размяться, вмяться, обмяться, подмяться).
    conjugation!("смяться", 8, "~~ять~я";
        infinitive: "~~ять~я";
        non_past_first: "~о~ну~ь", "~о~нём~я";
        non_past_second: "~о~нёшь~я", "~о~нёте~ь";
        non_past_third: "~о~нёт~я", "~о~нут~я";
        past: "~~ял~я", "~~яла~ь", "~~яло~ь", "~~яли~ь";
        imperative: "~о~ни~ь", "~о~ните~ь";
        participle: "~е~ущий~я", "~~явший~я", "~~ять~я", "~о~~анный";
        gerund: "~е~я~ь", "~~явши~ь";
    ),
    // Осмеять: 8 words (осмеять, засмеять, обсмеять, пересмеять, просмеять, ссять).
    conjugation!("осмеять", 8, "~ять";
        infinitive: "~ять";
        non_past_first: "~ю", "~ём";
        non_past_second: "~ёшь", "~ёте";
        non_past_third: "~ёт", "~ют";
        past: "~ял", "~яла", "~яло", "~яли";
        imperative: "~й", "~йте";
        participle: "~ющий", "~явший", "~емый", "~янный";
        gerund: "~я", "~яв|~явши";
    ),
    // Выклевать: 8 words (выклевать, выплевать, выблевать, вымалевать, заштемпелевать, проштемпелевать).
    conjugation!("выклевать", 8, "~евать";
        infinitive: "~евать";
        non_past_first: "~юю", "~юем";
        non_past_second: "~юешь", "~юете";
        non_past_third: "~юет", "~юют";
        past: "~евал", "~евала", "~евало", "~евали";
        imperative: "~юй", "~юйте";
        participle: "~юющий", "~евавший", "~юемый", "~еванный";
        gerund: "~юя", "~евав|~евавши";
    ),
    // Разреветься: 8 words (выреветься, нареветься, обреветься, прореветься, разреветься, изреветься).
    conjugation!("разреветься", 8, "~еть~я";
        infinitive: "~еть~я";
        non_past_first: "~у~ь", "~ём~я";
        non_past_second: "~ёшь~я", "~ёте~ь";
        non_past_third: "~ёт~я", "~ут~я";
        past: "~ел~я", "~ела~ь", "~ело~ь", "~ели~ь";
        imperative: "~и~ь", "~ите~ь";
        participle: "~ящий~я", "~евший~я", "~е~емый", "~ё~нный";
        gerund: "~я~ь", "~евши~ь";
    ),
    // Выпариться: 8 words (закупориться, выпариться, замусориться, умайориться, откупориться, раскупориться).
    conjugation!("выпариться", 8, "~ить~я";
        infinitive: "~ить~я";
        non_past_first: "~ю~ь", "~им~я";
        non_past_second: "~ишь~я", "~ите~ь";
        non_past_third: "~ит~я", "~ят~я";
        past: "~ил~я", "~ила~ь", "~ило~ь", "~или~ь";
        imperative: "~ь~я|~и~ь", "~ьте~ь";
        participle: "~ящий~я", "~ивший~я", "~е~емый", "~ё~нный";
        gerund: "~я~ь", "~я~ь|~ивши~ь";
    ),
    // Месть: 8 words (месть, приплесть, приобресть, выцвесть, выгнесть, процвесть).
    conjugation!("месть", 8, "~есть";
        infinitive: "~есть";
        non_past_first: "~ету", "~етём";
        non_past_second: "~етёшь", "~етёте";
        non_past_third: "~етёт", "~етут";
        past: "~ёл", "~ела", "~ело", "~ели";
        imperative: "~ети", "~етите";
        participle: "~етущий", "~етший", "~имый", "~етённый";
        gerund: "~етя", "~етя";
    ),
    // Отсвистеться: 8 words (насвистеться, рассвистеться, пересвистеться, свистеться, высвистеться, досвистеться).
    conjugation!("отсвистеться", 8, "~~теться";
        infinitive: "~~теться";
        non_past_first: "~щу~ь", "~~тимся";
        non_past_second: "~~тишься", "~~титесь";
        non_past_third: "~~тится", "~~тятся";
        past: "~~телся", "~~телась", "~~телось", "~~телись";
        imperative: "~~тись", "~~титесь";
        participle: "~~тящийся", "~~тевшийся", "~е~емый", "~ё~нный";
        gerund: "~~тясь", "~~тевшись";
    ),
    // Вести: 7 words (вести, блюсти, брести, соблюсти, грясти, густи).
    conjugation!("вести", 7, "~сти";
        infinitive: "~сти";
        non_past_first: "~ду", "~дём";
        non_past_second: "~дёшь", "~дёте";
        non_past_third: "~дёт", "~дут";
        past: "~л", "~ла", "~ло", "~ли";
        imperative: "~ди", "~дите";
        participle: "~дущий", "~дший", "~домый", "~дённый";
        gerund: "~дя", "~дши";
    ),
    // Вынести: 7 words (вынести, вывезти, выпасти, вытрясти, повывезти, повынести).
    conjugation!("вынести", 7, "~ти";
        infinitive: "~ти";
        non_past_first: "~у", "~ем";
        non_past_second: "~ешь", "~ете";
        non_past_third: "~ет", "~ут";
        past: "~", "~ла", "~ло", "~ли";
        imperative: "~и", "~ите";
        participle: "~ущий", "~ший", "~омый", "~енный";
        gerund: "~я", "~я";
    ),
    // Почать: 7 words (начать, зачать, почать, учать, предначать, распочать).
    conjugation!("почать", 7, "~ать";
        infinitive: "~ать";
        non_past_first: "~ну", "~нём";
        non_past_second: "~нёшь", "~нёте";
        non_past_third: "~нёт", "~нут";
        past: "~ал", "~ала", "~ало", "~али";
        imperative: "~ни", "~ните";
        participle: "~мущий", "~авший", "~имый", "~атый";
        gerund: "~мя", "~ав|~авши";
    ),
    // Нашептать: 7 words (шептать, прошептать, зашептать, нашептать, пошептать, перешептать).
    conjugation!("нашептать", 7, "~е~тать";
        infinitive: "~е~тать";
        non_past_first: "~е~чу", "~е~чем";
        non_past_second: "~е~чешь", "~е~чете";
        non_past_third: "~е~чет", "~е~чут";
        past: "~е~тал", "~е~тала", "~е~тало", "~е~тали";
        imperative: "~е~чи", "~е~чите";
        participle: "~е~чущий", "~е~тавший", "~е~емый", "~ё~танный";
        gerund: "~е~ча", "~е~тав|~е~тавши";
    ),
    // Прятаться: 7 words (прятаться, спрятаться, запрятаться, припрятаться, упрятаться, напрятаться).
    conjugation!("прятаться", 7, "~тать~я";
        infinitive: "~тать~я";
        non_past_first: "~чу~ь", "~чем~я";
        non_past_second: "~чешь~я", "~чете~ь";
        non_past_third: "~чет~я", "~чут~я";
        past: "~тал~я", "~тала~ь", "~тало~ь", "~тали~ь";
        imperative: "~чь~я", "~чьте~ь";
        participle: "~чущий~я", "~тавший~я", "~е~емый", "~ё~нный";
        gerund: "~ча~ь", "~тавши~ь";
    ),
    // Реветь: 7 words (реветь, взреветь, зареветь, отреветь, пореветь, прореветь).
    conjugation!("реветь", 7, "~еть";
        infinitive: "~еть";
        non_past_first: "~у", "~ём";
        non_past_second: "~ёшь", "~ёте";
        non_past_third: "~ёт", "~ут";
        past: "~ел", "~ела", "~ело", "~ели";
        imperative: "~и", "~ите";
        participle: "~ущий", "~евший", "~имый", "~енный";
        gerund: "~я", "~ев|~евши";
    ),
    // Закупорить: 7 words (выверить, закупорить, вызволить, расстопорить, застопорить, заякорить).
    conjugation!("закупорить", 7, "~ить";
        infinitive: "~ить";
        non_past_first: "~ю", "~им";
        non_past_second: "~ишь", "~ите";
        non_past_third: "~ит", "~ят";
        past: "~ил", "~ила", "~ило", "~или";
        imperative: "~и|~ь", "~ьте";
        participle: "~ящий", "~ивший", "~имый", "~енный";
        gerund: "~я", "~ив|~ивши";
    ),
    // Ебсти: 7 words (выцвести, вымести, ебсти, выплести, повыцвести, мясти).
    conjugation!("ебсти", 7, "~сти";
        infinitive: "~сти";
        non_past_first: "~ту", "~тем";
        non_past_second: "~тешь", "~тете";
        non_past_third: "~тет", "~тут";
        past: "~л", "~ла", "~ло", "~ли";
        imperative: "~ти", "~тите";
        participle: "~дущий", "~тший", "~емый", "~тенный";
        gerund: "~дя", "~тя";
    ),
    // Залгаться: 7 words (залгаться, изолгаться, отолгаться, оболгаться, разолгаться, налгаться).
    conjugation!("залгаться", 7, "~гать~я";
        infinitive: "~гать~я";
        non_past_first: "~гу~ь", "~жём~я";
        non_past_second: "~жёшь~я", "~жёте~ь";
        non_past_third: "~жёт~я", "~гут~я";
        past: "~гал~я", "~гала~ь", "~гало~ь", "~гали~ь";
        imperative: "~ги~ь", "~гите~ь";
        participle: "~гущий~я", "~гавший~я", "~е~емый", "~ё~нный";
        gerund: "~я~ь", "~гавши~ь";
    ),
    // Лгать: 7 words (лгать, солгать, налгать, оболгать, прилгать, полгать).
    conjugation!("лгать", 7, "~гать";
        infinitive: "~гать";
        non_past_first: "~гу", "~жём";
        non_past_second: "~жёшь", "~жёте";
        non_past_third: "~жёт", "~гут";
        past: "~гал", "~гала", "~гало", "~гали";
        imperative: "~ги", "~гите";
        participle: "~гущий", "~гавший", "~емый", "~ганный";
        gerund: "~я", "~гав|~гавши";
    ),
    // Насытиться: 7 words (насытиться, пресытиться, пересытиться, похититься, ухититься, сытиться).
    conjugation!("насытиться", 7, "~тить~я";
        infinitive: "~тить~я";
        non_past_first: "~щу~ь", "~тим~я";
        non_past_second: "~тишь~я", "~тите~ь";
        non_past_third: "~тит~я", "~тят~я";
        past: "~тил~я", "~тила~ь", "~тило~ь", "~тили~ь";
        imperative: "~ть~я", "~тьте~ь";
        participle: "~тящий~я", "~тивший~я", "~е~емый", "~ё~нный";
        gerund: "~тя~ь", "~тивши~ь";
    ),
    // Спать: 6 words (спать, поспать, отоспать, выспать, доспать, недоспать).
    conjugation!("спать", 6, "~ать";
        infinitive: "~ать";
        non_past_first: "~лю", "~им";
        non_past_second: "~ишь", "~ите";
        non_past_third: "~ит", "~ят";
        past: "~ал", "~ала", "~ало", "~али";
        imperative: "~и", "~ите";
        participle: "~ящий", "~авший", "~имый", "~анный";
        gerund: "~я", "~ав|~авши";
    ),
    // Достичь: 6 words (достичь, постичь, настичь, застичь, перестичь, пристичь).
    conjugation!("достичь", 6, "~чь";
        infinitive: "~чь";
        non_past_first: "~гну", "~гнем";
        non_past_second: "~гнешь", "~гнете";
        non_past_third: "~гнет", "~гнут";
        past: "~г", "~гла", "~гло", "~гли";
        imperative: "~гни", "~гните";
        participle: "~гущий", "~гший|~гнувший", "~емый", "~гнутый";
        gerund: "~я", "~гнув|~гши|~гнувши";
    ),
    // Разжечь: 6 words (сжечь, поджечь, отжечь, обжечь, разжечь, изжечь).
    conjugation!("разжечь", 6, "~~ечь";
        infinitive: "~~ечь";
        non_past_first: "~о~гу", "~о~жём";
        non_past_second: "~о~жёшь", "~о~жёте";
        non_past_third: "~о~жёт", "~о~гут";
        past: "~~ёг", "~о~гла", "~о~гло", "~о~гли";
        imperative: "~о~ги", "~о~гите";
        participle: "~ющий~я", "~~ёгший", "~е~емый", "~о~жённый";
        gerund: "~я~ь", "~~ёгши";
    ),
    // Замереть: 6 words (запереть, умереть, помереть, замереть, примереть, позапереть).
    conjugation!("замереть", 6, "~е~еть";
        infinitive: "~е~еть";
        non_past_first: "~~у", "~~ём";
        non_past_second: "~~ёшь", "~~ёте";
        non_past_third: "~~ёт", "~~ут";
        past: "~е~", "~е~ла", "~е~ло", "~е~ли";
        imperative: "~~и", "~~ите";
        participle: "~~ущий", "~е~ший", "~е~емый", "~е~тый";
        gerund: "~е~я", "~е~ев";
    ),
    // Морщиться: 6 words (таращиться, морщиться, поморщиться, сморщиться, топорщиться, упорядочиться).
    conjugation!("морщиться", 6, "~ить~я";
        infinitive: "~ить~я";
        non_past_first: "~у~ь", "~им~я";
        non_past_second: "~ишь~я", "~ите~ь";
        non_past_third: "~ит~я", "~ат~я";
        past: "~ил~я", "~ила~ь", "~ило~ь", "~или~ь";
        imperative: "~и~ь|~ь~я", "~ьте~ь";
        participle: "~ащий~я", "~ивший~я", "~е~емый", "~ё~нный";
        gerund: "~а~ь", "~ивши~ь";
    ),
    // Измыслить: 6 words (злоумыслить, умыслить, замыслить, измыслить, вымыслить, обмыслить).
    conjugation!("измыслить", 6, "~с~ить";
        infinitive: "~с~ить";
        non_past_first: "~с~ю", "~с~им";
        non_past_second: "~с~ишь", "~с~ите";
        non_past_third: "~с~ит", "~с~ят";
        past: "~с~ил", "~с~ила", "~с~ило", "~с~или";
        imperative: "~с~и", "~с~ите";
        participle: "~ющий~я", "~с~ивший", "~е~емый", "~ш~енный";
        gerund: "~я~ь", "~с~ив|~с~ивши";
    ),
    // Обидеть: 6 words (обидеть, высидеть, изобидеть, разобидеть, забидеть, пообидеть).
    conjugation!("обидеть", 6, "~деть";
        infinitive: "~деть";
        non_past_first: "~жу", "~дим";
        non_past_second: "~дишь", "~дите";
        non_past_third: "~дит", "~дят";
        past: "~дел", "~дела", "~дело", "~дели";
        imperative: "~дь", "~дьте";
        participle: "~дящий", "~девший", "~димый", "~женный";
        gerund: "~дя", "~дев|~девши";
    ),
    // Рассесться: 6 words (усесться, рассесться, осесться, ссесться, надсесться, обсесться).
    conjugation!("рассесться", 6, "~е~ться";
        infinitive: "~е~ться";
        non_past_first: "~яду~ь", "~ядем~я";
        non_past_second: "~ядешь~я", "~ядете~ь";
        non_past_third: "~ядет~я", "~ядут~я";
        past: "~ел~я", "~ела~ь", "~ело~ь", "~ели~ь";
        imperative: "~ядь~я", "~ядьте~ь";
        participle: "~ящий~я", "~евший~я", "~е~емый", "~ё~нный";
        gerund: "~я~ь", "~евши~ь";
    ),
    // Сжаться: 6 words (сжаться, вжаться, отжаться, обжаться, поджаться, разжаться).
    conjugation!("сжаться", 6, "~~ать~я";
        infinitive: "~~ать~я";
        non_past_first: "~о~му~ь", "~о~мём~я";
        non_past_second: "~о~мёшь~я", "~о~мёте~ь";
        non_past_third: "~о~мёт~я", "~о~мут~я";
        past: "~~ал~я", "~~ала~ь", "~~ало~ь", "~~али~ь";
        imperative: "~о~ми~ь", "~о~мите~ь";
        participle: "~е~ущий~я", "~~авший~я", "~~ать~я", "~о~~анный";
        gerund: "~е~я~ь", "~~авши~ь";
    ),
    // Соваться: 6 words (коваться, основаться, соваться, подковаться, вковаться, рассоваться).
    conjugation!("соваться", 6, "~овать~я";
        infinitive: "~овать~я";
        non_past_first: "~ую~ь", "~уём~я";
        non_past_second: "~уёшь~я", "~уёте~ь";
        non_past_third: "~уёт~я", "~уют~я";
        past: "~овал~я", "~овала~ь", "~овало~ь", "~овали~ь";
        imperative: "~уй~я", "~уйте~ь";
        participle: "~ующий~я", "~овавший~я", "~е~емый", "~ё~нный";
        gerund: "~уя~ь", "~овавши~ь";
    ),
    // Разъехаться: 6 words (съехаться, разъехаться, проехаться, поразъехаться, подъехаться, ехаться).
    conjugation!("разъехаться", 6, "~хать~я";
        infinitive: "~хать~я";
        non_past_first: "~ду~ь", "~дем~я";
        non_past_second: "~дешь~я", "~дете~ь";
        non_past_third: "~дет~я", "~дут~я";
        past: "~хал~я", "~хала~ь", "~хало~ь", "~хали~ь";
        imperative: "~зжай~я", "~зжайте~ь";
        participle: "~шущий~я", "~хавший~я", "~е~емый", "~ё~нный";
        gerund: "~ша~ь", "~хавши~ь";
    ),
    // Сыпаться: 6 words (рассыпаться, посыпаться, сыпаться, осыпаться, высыпаться, отсыпаться).
    conjugation!("сыпаться", 6, "~ать~я";
        infinitive: "~ать~я";
        non_past_first: "~лю~ь", "~лем~я|~ем~я";
        non_past_second: "~лешь~я|~ешь~я", "~лете~ь|~ете~ь";
        non_past_third: "~лет~я|~ет~я", "~лют~я";
        past: "~ал~я", "~ала~ь", "~ало~ь", "~али~ь";
        imperative: "~ь~я", "~ьте~ь";
        participle: "~лющий~я", "~авший~я", "~е~емый", "~ё~нный";
        gerund: "~ля~ь", "~авши~ь";
    ),
    // Хлестать: 6 words (хлестать, нахлестать, похлестать, расхлестать, схлестать, захлестать).
    conjugation!("хлестать", 6, "~естать";
        infinitive: "~естать";
        non_past_first: "~ещу", "~ещем";
        non_past_second: "~ещешь", "~ещете";
        non_past_third: "~ещет", "~ещут";
        past: "~естал", "~естала", "~естало", "~естали";
        imperative: "~ещи", "~ещите";
        participle: "~ещущий", "~еставший", "~ескаемый", "~ёстанный";
        gerund: "~еща", "~естав|~еставши";
    ),
    // Плестись: 6 words (плестись, местись, сместись, цвестись, гнестись, доместись).
    conjugation!("плестись", 6, "~е~тись";
        infinitive: "~е~тись";
        non_past_first: "~ету~ь", "~етём~я";
        non_past_second: "~етёшь~я", "~етёте~ь";
        non_past_third: "~етёт~я", "~етут~я";
        past: "~ёл~я", "~ела~ь", "~ело~ь", "~ели~ь";
        imperative: "~ети~ь", "~етите~ь";
        participle: "~етущий~я", "~ётший~я", "~е~емый", "~ё~нный";
        gerund: "~етя~ь", "~ётши~ь";
    ),
    // Выпасть: 6 words (выпасть, выкрасть, выпрясть, повыпасть, выкласть, повыкрасть).
    conjugation!("выпасть", 6, "~сть";
        infinitive: "~сть";
        non_past_first: "~ду", "~дем";
        non_past_second: "~дешь", "~дете";
        non_past_third: "~дет", "~дут";
        past: "~л", "~ла", "~ло", "~ли";
        imperative: "~ди", "~дите";
        participle: "~дущий", "~вший", "~домый", "~денный";
        gerund: "~дя", "~в|~вши";
    ),
    // Исторгнуть: 6 words (ниспровергнуть, исторгнуть, разверзнуть, отверзнуть, нисповергнуть, развергнуть).
    conjugation!("исторгнуть", 6, "~нуть";
        infinitive: "~нуть";
        non_past_first: "~ну", "~нем";
        non_past_second: "~нешь", "~нете";
        non_past_third: "~нет", "~нут";
        past: "~", "~ла", "~ло", "~ли";
        imperative: "~ни", "~ните";
        participle: "~нущий", "~ший|~нувший", "~нимый", "~нутый";
        gerund: "~ня", "~ши|~нув|~нувши";
    ),
    // Зачертить: 6 words (вкатить, вскатить, докатить, дочертить, зачертить, подчертить).
    conjugation!("зачертить", 6, "~тить";
        infinitive: "~тить";
        non_past_first: "~чу", "~тим";
        non_past_second: "~тишь", "~тите";
        non_past_third: "~тит", "~тят";
        past: "~тил", "~тила", "~тило", "~тили";
        imperative: "~ти", "~тите";
        participle: "~тящий", "~тивший", "~тимый", "~танный";
        gerund: "~тя", "~тив|~тивши";
    ),
    // Мочься: 6 words (разнемочься, выпрячься, выстричься, мочься, занемочься, перемочься).
    conjugation!("мочься", 6, "~чь~я";
        infinitive: "~чь~я";
        non_past_first: "~гу~ь", "~жем~я";
        non_past_second: "~жешь~я", "~жете~ь";
        non_past_third: "~жет~я", "~гут~я";
        past: "~г~я", "~гла~ь", "~гло~ь", "~гли~ь";
        imperative: "~ги~ь", "~гите~ь";
        participle: "~гущий~я", "~гший~я", "~е~емый", "~ё~нный";
        gerund: "~жа~ь", "~гши~ь";
    ),
    // Исхлестаться: 6 words (насвистаться, рассвистаться, хвостаться, расхлестаться, исхлестаться, нахлестаться).
    conjugation!("исхлестаться", 6, "~~таться";
        infinitive: "~~таться";
        non_past_first: "~щу~ь", "~щем~я";
        non_past_second: "~щешь~я", "~щете~ь";
        non_past_third: "~щет~я", "~щут~я";
        past: "~~тался", "~~талась", "~~талось", "~~тались";
        imperative: "~щи~ь", "~щите~ь";
        participle: "~щущий~я", "~~тавшийся", "~е~емый", "~ё~нный";
        gerund: "~ща~ь", "~~тавшись";
    ),
    // Взорать: 6 words (глаголать, проглаголать, обкармливать, разорать, взорать, доорать).
    conjugation!("взорать", 6, "~ать";
        infinitive: "~ать";
        non_past_first: "~ю", "~ем";
        non_past_second: "~ешь", "~ете";
        non_past_third: "~ет", "~ют";
        past: "~ал", "~ала", "~ало", "~али";
        imperative: "~и", "~ите";
        participle: "~ущий", "~авший", "~имый", "~енный";
        gerund: "~ая", "~ав|~авши";
    ),
    // Мести: 5 words (гнести, плести, мести, рассвести, наплести).
    conjugation!("мести", 5, "~ести";
        infinitive: "~ести";
        non_past_first: "~ету", "~етём";
        non_past_second: "~етёшь", "~етёте";
        non_past_third: "~етёт", "~етут";
        past: "~ёл", "~ела", "~ело", "~ели";
        imperative: "~ети", "~етите";
        participle: "~етущий", "~ётший", "~ебомый", "~етённый";
        gerund: "~етя", "~ётши";
    ),
    // Висеть: 5 words (висеть, зависеть, повисеть, перевисеть, провисеть).
    conjugation!("висеть", 5, "~сеть";
        infinitive: "~сеть";
        non_past_first: "~шу", "~сим";
        non_past_second: "~сишь", "~сите";
        non_past_third: "~сит", "~сят";
        past: "~сел", "~села", "~село", "~сели";
        imperative: "~си", "~сите";
        participle: "~сящий", "~севший", "~симый", "~шенный";
        gerund: "~ся", "~сев|~севши";
    ),
    // Выбрать: 5 words (выбрать, выдрать, перевыбрать, повыдрать, повыбрать).
    conjugation!("выбрать", 5, "~~ать";
        infinitive: "~~ать";
        non_past_first: "~е~у", "~е~ем";
        non_past_second: "~е~ешь", "~е~ете";
        non_past_third: "~е~ет", "~е~ут";
        past: "~~ал", "~~ала", "~~ало", "~~али";
        imperative: "~е~и", "~е~ите";
        participle: "~е~ущий", "~~авший", "~е~омый", "~~анный";
        gerund: "~е~я", "~~ав|~~авши";
    ),
    // Отжать: 5 words (сжать, обжать, разжать, отжать, вжать).
    conjugation!("отжать", 5, "~~ать";
        infinitive: "~~ать";
        non_past_first: "~о~му", "~о~мём";
        non_past_second: "~о~мёшь", "~о~мёте";
        non_past_third: "~о~мёт", "~о~мут";
        past: "~~ал", "~~ала", "~~ало", "~~али";
        imperative: "~о~ми", "~о~мите";
        participle: "~е~ущий", "~~авший", "~е~омый", "~~атый";
        gerund: "~е~я", "~~ав|~~авши";
    ),
    // Отстоять: 5 words (отстоять, выстоять, простоять, застоять, достоять).
    conjugation!("отстоять", 5, "~ять";
        infinitive: "~ять";
        non_past_first: "~ю", "~им";
        non_past_second: "~ишь", "~ите";
        non_past_third: "~ит", "~ят";
        past: "~ял", "~яла", "~яло", "~яли";
        imperative: "~й", "~йте";
        participle: "~ящий", "~явший", "~емый", "~янный";
        gerund: "~я", "~яв|~явши";
    ),
    // Заспать: 5 words (проспать, переспать, заспать, наспать, приспать).
    conjugation!("заспать", 5, "~ать";
        infinitive: "~ать";
        non_past_first: "~лю", "~им";
        non_past_second: "~ишь", "~ите";
        non_past_third: "~ит", "~ят";
        past: "~ал", "~ала", "~ало", "~али";
        imperative: "~и", "~ите";
        participle: "~ящий", "~авший", "~имый", "~анный";
        gerund: "~я", "~ав|~авши";
    ),
    // Выставиться: 5 words (понадобиться, осведомиться, выставиться, занадобиться, уведомиться).
    conjugation!("выставиться", 5, "~ить~я";
        infinitive: "~ить~я";
        non_past_first: "~лю~ь", "~им~я";
        non_past_second: "~ишь~я", "~ите~ь";
        non_past_third: "~ит~я", "~ят~я";
        past: "~ил~я", "~ила~ь", "~ило~ь", "~или~ь";
        imperative: "~и~ь|~ь~я", "~ьте~ь";
        participle: "~ящий~я", "~ивший~я", "~е~емый", "~ё~нный";
        gerund: "~я~ь", "~ивши~ь";
    ),
    // Одолеть: 5 words (преодолеть, одолеть, запечатлеть, напечатлеть, отпечатлеть).
    conjugation!("одолеть", 5, "~еть";
        infinitive: "~еть";
        non_past_first: "~ею", "~еем";
        non_past_second: "~еешь", "~еете";
        non_past_third: "~еет", "~еют";
        past: "~ел", "~ела", "~ело", "~ели";
        imperative: "~ей", "~ейте";
        participle: "~ящий", "~евший", "~имый", "~ённый";
        gerund: "~я", "~ев|~евши";
    ),
    // Выставить: 5 words (выставить, выправить, осведомить, уведомить, предуведомить).
    conjugation!("выставить", 5, "~ить";
        infinitive: "~ить";
        non_past_first: "~лю", "~им";
        non_past_second: "~ишь", "~ите";
        non_past_third: "~ит", "~ят";
        past: "~ил", "~ила", "~ило", "~или";
        imperative: "~и|~ь", "~ьте";
        participle: "~ящий", "~ивший", "~имый", "~ленный";
        gerund: "~я", "~ив|~ивши";
    ),
    // Изъять: 5 words (подъять, изъять, разъять, отъять, съять).
    conjugation!("изъять", 5, "~ъять";
        infinitive: "~ъять";
        non_past_first: "~ыму", "~ымем";
        non_past_second: "~ымешь", "~ымете";
        non_past_third: "~ымет", "~ымут";
        past: "~ъял", "~ъяла", "~ъяло", "~ъяли";
        imperative: "~ыми", "~ымите";
        participle: "~ющий", "~ъявший", "~емый", "~ъятый";
        gerund: "~я", "~ъяв|~ъявши";
    ),
    // Выдвинуться: 5 words (высунуться, выдвинуться, грянуться, откашлянуться, самовыдвинуться).
    conjugation!("выдвинуться", 5, "~уть~я";
        infinitive: "~уть~я";
        non_past_first: "~у~ь", "~ем~я";
        non_past_second: "~ешь~я", "~ете~ь";
        non_past_third: "~ет~я", "~ут~я";
        past: "~ул~я", "~ула~ь", "~уло~ь", "~ули~ь";
        imperative: "~и~ь", "~ьте~ь";
        participle: "~ущий~я", "~увший~я", "~е~емый", "~ё~нный";
        gerund: "~ая~ь", "~увши~ь";
    ),
    // Пробежаться: 5 words (разбежаться, сбежаться, пробежаться, поразбежаться, оббежаться).
    conjugation!("пробежаться", 5, "~жать~я";
        infinitive: "~жать~я";
        non_past_first: "~гу~ь", "~жим~я";
        non_past_second: "~жишь~я", "~жите~ь";
        non_past_third: "~жит~я", "~гут~я";
        past: "~жал~я", "~жала~ь", "~жало~ь", "~жали~ь";
        imperative: "~ги~ь", "~гите~ь";
        participle: "~гущий~я", "~жавший~я", "~е~емый", "~ё~нный";
        gerund: "~я~ь", "~жавши~ь";
    ),
    // Обжечься: 5 words (обжечься, разжечься, поджечься, сжечься, изжечься).
    conjugation!("обжечься", 5, "~~ечь~я";
        infinitive: "~~ечь~я";
        non_past_first: "~о~гу~ь", "~о~жём~я";
        non_past_second: "~о~жёшь~я", "~о~жёте~ь";
        non_past_third: "~о~жёт~я", "~о~гут~я";
        past: "~~ёг~я", "~о~гла~ь", "~о~гло~ь", "~о~гли~ь";
        imperative: "~о~ги~ь", "~о~гите~ь";
        participle: "~е~ущий~я", "~~ёгший~я", "~~ечь~я", "~о~~анный";
        gerund: "~е~я~ь", "~~ёгши~ь";
    ),
    // Приобрестись: 5 words (приобрестись, обрестись, вплестись, выплестись, выместись).
    conjugation!("приобрестись", 5, "~е~тись";
        infinitive: "~е~тись";
        non_past_first: "~ету~ь", "~етём~я";
        non_past_second: "~етёшь~я", "~етёте~ь";
        non_past_third: "~етёт~я", "~етут~я";
        past: "~ёл~я", "~ела~ь", "~ело~ь", "~ели~ь";
        imperative: "~ети~ь", "~етите~ь";
        participle: "~етущий~я", "~етший~я", "~е~емый", "~ё~нный";
        gerund: "~етя~ь", "~етя~ь";
    ),
    // Волгнуть: 5 words (волгнуть, брюзгнуть, горкнуть, грузнуть, осмеркнуть).
    conjugation!("волгнуть", 5, "~нуть";
        infinitive: "~нуть";
        non_past_first: "~ну", "~нем";
        non_past_second: "~нешь", "~нете";
        non_past_third: "~нет", "~нут";
        past: "~нул", "~ла", "~ло", "~ли";
        imperative: "~ни", "~ните";
        participle: "~нущий", "~нувший", "~нимый", "~нутый";
        gerund: "~ня", "~нув|~нувши";
    ),
    // Стлаться: 5 words (стлаться, подостлаться, постлаться, простлаться, устлаться).
    conjugation!("стлаться", 5, "~~ать~я";
        infinitive: "~~ать~я";
        non_past_first: "~е~ю~ь", "~е~ем~я";
        non_past_second: "~е~ешь~я", "~е~ете~ь";
        non_past_third: "~е~ет~я", "~е~ют~я";
        past: "~~ал~я", "~~ала~ь", "~~ало~ь", "~~али~ь";
        imperative: "~е~и~ь", "~е~ите~ь";
        participle: "~е~ющий~я", "~~авший~я", "~~ать~я", "~о~~анный";
        gerund: "~е~я~ь", "~~авши~ь";
    ),
    // Распоясаться: 5 words (распоясаться, выплясаться, подпоясаться, вытесаться, опоясаться).
    conjugation!("распоясаться", 5, "~~аться";
        infinitive: "~~аться";
        non_past_first: "~шу~ь", "~шем~я";
        non_past_second: "~шешь~я", "~шете~ь";
        non_past_third: "~шет~я", "~шут~я";
        past: "~~ался", "~~алась", "~~алось", "~~ались";
        imperative: "~шь~я", "~шьте~ь";
        participle: "~шущий~я", "~~авшийся", "~е~емый", "~ё~нный";
        gerund: "~ша~ь", "~~авшись";
    ),
    // Бухвостить: 5 words (чихвостить, ростить, чехвостить, бухвостить, шилохвостить).
    conjugation!("бухвостить", 5, "~стить";
        infinitive: "~стить";
        non_past_first: "~щу", "~стим";
        non_past_second: "~стишь", "~стите";
        non_past_third: "~стит", "~стят";
        past: "~стил", "~стила", "~стило", "~стили";
        imperative: "~сть", "~стьте";
        participle: "~стящий", "~стивший", "~стимый", "~щенный";
        gerund: "~стя", "~стив|~стивши";
    ),
    // Вылиться: 5 words (выбиться, вылиться, выпиться, вывиться, повыбиться).
    conjugation!("вылиться", 5, "~ить~я";
        infinitive: "~ить~я";
        non_past_first: "~ью~ь", "~ьем~я";
        non_past_second: "~ьешь~я", "~ейте~ь";
        non_past_third: "~ьет~я", "~ьют~я";
        past: "~ил~я", "~ила~ь", "~ило~ь", "~или~ь";
        imperative: "~ей~я", "~ейте~ь";
        participle: "~ьющий~я", "~ивший~я", "~е~емый", "~ё~нный";
        gerund: "~ья~ь", "~ивши~ь";
    ),
    // Трепетаться: 5 words (вострепетаться, трепетаться, оклеветаться, затрепетаться, потрепетаться).
    conjugation!("трепетаться", 5, "~тать~я";
        infinitive: "~тать~я";
        non_past_first: "~щу~ь", "~щем~я";
        non_past_second: "~щешь~я", "~щете~ь";
        non_past_third: "~щет~я", "~щут~я";
        past: "~тал~я", "~тала~ь", "~тало~ь", "~тали~ь";
        imperative: "~щи~ь", "~щите~ь";
        participle: "~щущий~я", "~тавший~я", "~е~емый", "~ё~нный";
        gerund: "~ща~ь", "~тавши~ь";
    ),
    // Уссаться: 5 words (уссаться, обоссаться, ссаться, сцаться, обосцаться).
    conjugation!("уссаться", 5, "~ать~я";
        infinitive: "~ать~я";
        non_past_first: "~у~ь", "~ым~я";
        non_past_second: "~ышь~я", "~ыте~ь";
        non_past_third: "~ыт~я", "~ут~я";
        past: "~ал~я", "~ала~ь", "~ало~ь", "~али~ь";
        imperative: "~ы~ь", "~ыте~ь";
        participle: "~ущий~я", "~авший~я", "~е~емый", "~ё~нный";
        gerund: "~а~ь", "~авши~ь";
    ),
    // Вылезти: 5 words (вылезти, выползти, повылезти, повыползти, выебти).
    conjugation!("вылезти", 5, "~ти";
        infinitive: "~ти";
        non_past_first: "~у", "~ем";
        non_past_second: "~ешь", "~ете";
        non_past_third: "~ет", "~ут";
        past: "~", "~ла", "~ло", "~ли";
        imperative: "~и", "~ите";
        participle: "~ущий", "~ший", "~омый", "~енный";
        gerund: "~я", "~ши";
    ),
    // Срастись: 5 words (разрастись, срастись, зарастись, врастись, растись).
    conjugation!("срастись", 5, "~а~ти~ь";
        infinitive: "~а~ти~ь";
        non_past_first: "~а~ту~ь", "~а~тём~я";
        non_past_second: "~а~тёшь~я", "~а~тёте~ь";
        non_past_third: "~а~тёт~я", "~а~тут~я";
        past: "~о~~я", "~о~ла~ь", "~о~ло~ь", "~о~ли~ь";
        imperative: "~а~ти~ь", "~а~тите~ь";
        participle: "~а~тущий~я", "~о~ший~я", "~а~ти~ь", "~о~~анный";
        gerund: "~а~тя~ь", "~о~ши~ь";
    ),
    // Ехать: 4 words (ехать, взъехать, повыехать, поразъехать).
    conjugation!("ехать", 4, "~хать";
        infinitive: "~хать";
        non_past_first: "~ду", "~дем";
        non_past_second: "~дешь", "~дете";
        non_past_third: "~дет", "~дут";
        past: "~хал", "~хала", "~хало", "~хали";
        imperative: "по~зжай", "по~зжайте";
        participle: "~дущий", "~хавший", "~домый", "~ханный";
        gerund: "~дучи", "~хав|~хавши";
    ),
    // Цвести: 4 words (цвести, расцвести, зацвести, отцвести).
    conjugation!("цвести", 4, "~ести";
        infinitive: "~ести";
        non_past_first: "~ету", "~етём";
        non_past_second: "~етёшь", "~етёте";
        non_past_third: "~етёт", "~етут";
        past: "~ёл", "~ела", "~ело", "~ели";
        imperative: "~ети", "~етите";
        participle: "~етущий", "~етший|~ётший", "~ебомый", "~етённый";
        gerund: "~етя", "~етши|~ётши";
    ),
    // Принять: 4 words (принять, воспринять, предпринять, напринять).
    conjugation!("принять", 4, "~нять";
        infinitive: "~нять";
        non_past_first: "~му", "~мем";
        non_past_second: "~мешь", "~мете";
        non_past_third: "~мет", "~мут";
        past: "~нял", "~няла", "~няло", "~няли";
        imperative: "~ми", "~мите";
        participle: "~мущий", "~нявший", "~емый", "~нятый";
        gerund: "~мя", "~няв|~нявши";
    ),
    // Услышать: 4 words (услышать, вымолчать, выбрюзжать, дослышать).
    conjugation!("услышать", 4, "~ать";
        infinitive: "~ать";
        non_past_first: "~у", "~им";
        non_past_second: "~ишь", "~ите";
        non_past_third: "~ит", "~ат";
        past: "~ал", "~ала", "~ало", "~али";
        imperative: "~ь", "~ьте";
        participle: "~ащий", "~авший", "~имый", "~анный";
        gerund: "~а", "~ав|~авши";
    ),
    // Выпереть: 4 words (вымереть, вытереть, выпереть, повымереть).
    conjugation!("выпереть", 4, "~е~еть";
        infinitive: "~е~еть";
        non_past_first: "~~у", "~~ем";
        non_past_second: "~~ешь", "~~ете";
        non_past_third: "~~ет", "~~ут";
        past: "~е~", "~е~ла", "~е~ло", "~е~ли";
        imperative: "~~и", "~~ите";
        participle: "~~ущий", "~е~ший", "~е~емый", "~е~тый";
        gerund: "~е~я", "~е~ев|~е~ши";
    ),
    // Высечь: 4 words (высечь, вытечь, выволочь, выпечь).
    conjugation!("высечь", 4, "~чь";
        infinitive: "~чь";
        non_past_first: "~ку", "~чем";
        non_past_second: "~чешь", "~чете";
        non_past_third: "~чет", "~кут";
        past: "~к", "~кла", "~кло", "~кли";
        imperative: "~ки", "~ките";
        participle: "~кущий", "~кший", "~тимый", "~ченный";
        gerund: "~ча", "~кши";
    ),
    // Напрячь: 4 words (запрячь, напрячь, распрячь, впрячь).
    conjugation!("напрячь", 4, "~ячь";
        infinitive: "~ячь";
        non_past_first: "~ягу", "~яжём";
        non_past_second: "~яжёшь", "~яжёте";
        non_past_third: "~яжёт", "~ягут";
        past: "~ёг|~яг", "~ягла", "~ягло", "~ягли";
        imperative: "~яги", "~ягите";
        participle: "~егущий", "~ягший", "~егомый", "~яжённый";
        gerund: "~я", "~ягши";
    ),
    // Выдвинуть: 4 words (выдвинуть, высунуть, кашлянуть, выплюнуть).
    conjugation!("выдвинуть", 4, "~уть";
        infinitive: "~уть";
        non_past_first: "~у", "~ем";
        non_past_second: "~ешь", "~ете";
        non_past_third: "~ет", "~ут";
        past: "~ул", "~ула", "~уло", "~ули";
        imperative: "~и|~ь", "~ьте";
        participle: "~ущий", "~увший", "~имый", "~утый";
        gerund: "~я", "~ув|~увши";
    ),
    // Внимать: 4 words (внимать, приимать, восприимать, предприимать).
    conjugation!("внимать", 4, "~и~ать";
        infinitive: "~и~ать";
        non_past_first: "~е~лю|~е~лю|~и~аю", "~е~лем|~и~аем";
        non_past_second: "~е~лешь|~и~аешь", "~е~лете|~и~аете";
        non_past_third: "~е~лет|~и~ает", "~е~лют|~и~ают";
        past: "~и~ал", "~и~ала", "~и~ало", "~и~али";
        imperative: "~е~ли|~е~ли|~и~ай", "~е~лите|~е~лите|~и~айте";
        participle: "~е~лющий|~и~ающий", "~и~авший", "~е~лемый|~и~аемый", "~е~ленный";
        gerund: "~е~ля|~е~ля|~и~ая", "~и~ав|~и~авши";
    ),
    // Бечь: 4 words (бечь, убечь, избечь, побечь).
    conjugation!("бечь", 4, "~ечь";
        infinitive: "~ечь";
        non_past_first: "~егу", "~егём";
        non_past_second: "~егёшь", "~егёте";
        non_past_third: "~егёт", "~егут";
        past: "~ёг", "~егла", "~егло", "~егли";
        imperative: "~еги", "~егите";
        participle: "~егущий", "~ёгший", "~егомый", "~ежённый";
        gerund: "~я", "~ёгши";
    ),
    // Улечься: 4 words (улечься, поулечься, разлечься, приулечься).
    conjugation!("улечься", 4, "~ечься";
        infinitive: "~ечься";
        non_past_first: "~ягусь", "~яжемся";
        non_past_second: "~яжешься", "~яжетесь";
        non_past_third: "~яжется", "~ягутся";
        past: "~ёгся", "~еглась", "~еглось", "~еглись";
        imperative: "~ягся", "~ягтесь";
        participle: "~ющий", "~ёгшийся", "~емый", "~нный";
        gerund: "~я", "~ёгшись";
    ),
    // Вынестись: 4 words (вынестись, вывезтись, вытрястись, отверзтись).
    conjugation!("вынестись", 4, "~ти~ь";
        infinitive: "~ти~ь";
        non_past_first: "~у~ь", "~ем~я";
        non_past_second: "~ешь~я", "~ете~ь";
        non_past_third: "~ет~я", "~ут~я";
        past: "~~я", "~ла~ь", "~ло~ь", "~ли~ь";
        imperative: "~и~ь", "~ите~ь";
        participle: "~ущий~я", "~ший~я", "~е~емый", "~ё~нный";
        gerund: "~я~ь", "~я~ь";
    ),
    // Истереть: 4 words (растереть, распереть, истереть, поистереть).
    conjugation!("истереть", 4, "~с~е~еть";
        infinitive: "~с~е~еть";
        non_past_first: "~зо~~у", "~зо~~ём";
        non_past_second: "~зо~~ёшь", "~зо~~ёте";
        non_past_third: "~зо~~ёт", "~зо~~ут";
        past: "~с~ё~", "~с~ё~ла", "~с~ё~ло", "~с~ё~ли";
        imperative: "~зо~~и", "~зо~~ите";
        participle: "~е~ущий~я", "~с~ё~ший", "~с~е~еть", "~с~ё~тый";
        gerund: "~е~я~ь", "~с~е~ев|~с~ё~ши";
    ),
    // Испить: 4 words (расшить, испить, распить, исшить).
    conjugation!("испить", 4, "~с~ить";
        infinitive: "~с~ить";
        non_past_first: "~зо~ью", "~зо~ьём";
        non_past_second: "~зо~ьёшь", "~зо~ьёте";
        non_past_third: "~зо~ьёт", "~зо~ьют";
        past: "~с~ил", "~с~ила", "~с~ило", "~с~или";
        imperative: "~с~ей", "~с~ейте";
        participle: "~ющий~я", "~с~ивший", "~е~емый", "~с~итый";
        gerund: "~я~ь", "~с~ив|~с~ивши";
    ),
    // Уеть: 4 words (выеть, поеть, заеть, уеть).
    conjugation!("уеть", 4, "~еть";
        infinitive: "~еть";
        non_past_first: "~ебу", "~ебём";
        non_past_second: "~ебёшь", "~ебёте";
        non_past_third: "~ебёт", "~ебут";
        past: "~ёб", "~ебла", "~ебло", "~ебли";
        imperative: "~еби", "~ебите";
        participle: "~ящий", "~ёбший", "~имый", "~ебённый";
        gerund: "~я", "~ёбши";
    ),
    // Выебсти: 4 words (уебсти, выебсти, поебсти, доебсти).
    conjugation!("выебсти", 4, "~е~сти";
        infinitive: "~е~сти";
        non_past_first: "~е~у", "~е~ём";
        non_past_second: "~е~ёшь", "~е~ёте";
        non_past_third: "~е~ёт", "~е~ут";
        past: "~ё~", "~е~ла", "~е~ло", "~е~ли";
        imperative: "~е~и", "~е~ите";
        participle: "~е~ущий", "~ё~ший", "~е~омый", "~е~ённый";
        gerund: "~е~я", "~ё~ши";
    ),
    // Поколебать: 4 words (колебать, поколебать, зыбать, выщипать).
    conjugation!("поколебать", 4, "~ать";
        infinitive: "~ать";
        non_past_first: "~лю", "~лем";
        non_past_second: "~лешь", "~лете";
        non_past_third: "~лет", "~лют";
        past: "~ал", "~ала", "~ало", "~али";
        imperative: "~ли", "~лите";
        participle: "~лющий", "~авший", "~лемый", "~ленный";
        gerund: "~ля", "~ав|~авши";
    ),
    // Насочинять: 4 words (насочинять, обрусять, постеснять, оязыковлять).
    conjugation!("насочинять", 4, "~ять";
        infinitive: "~ять";
        non_past_first: "~яю", "~яем";
        non_past_second: "~яешь", "~яете";
        non_past_third: "~яет", "~яют";
        past: "~ял", "~яла", "~яло", "~яли";
        imperative: "~яй", "~яйте";
        participle: "~яющий", "~явший", "~яемый", "~ённый";
        gerund: "~яя", "~яв|~явши";
    ),
    // Растереться: 4 words (растереться, истереться, поистереться, распереться).
    conjugation!("растереться", 4, "~с~е~еть~я";
        infinitive: "~с~е~еть~я";
        non_past_first: "~зо~~у~ь", "~зо~~ём~я";
        non_past_second: "~зо~~ёшь~я", "~зо~~ёте~ь";
        non_past_third: "~зо~~ёт~я", "~зо~~ут~я";
        past: "~с~ё~~я", "~с~ё~ла~ь", "~с~ё~ло~ь", "~с~ё~ли~ь";
        imperative: "~зо~~и~ь", "~зо~~ите~ь";
        participle: "~с~е~еть~я", "~с~ё~ший~я", "~с~е~еть~я", "~с~е~еть~я";
        gerund: "~с~е~еть~я", "~с~ё~ши~ь";
    ),
    // Изъяться: 4 words (изъяться, подъяться, разъяться, отъяться).
    conjugation!("изъяться", 4, "~ъять~я";
        infinitive: "~ъять~я";
        non_past_first: "~ыму~ь", "~ымем~я";
        non_past_second: "~ымешь~я", "~ымете~ь";
        non_past_third: "~ымет~я", "~ымут~я";
        past: "~ъял~я", "~ъяла~ь", "~ъяло~ь", "~ъяли~ь";
        imperative: "~ыми~ь", "~ымите~ь";
        participle: "~ющий~я", "~ъявший~я", "~е~емый", "~ё~нный";
        gerund: "~я~ь", "~ъявши~ь";
    ),
    // Хотеться: 3 words (хотеться, захотеться, расхотеться).
    conjugation!("хотеться", 3, "~теть~я";
        infinitive: "~теть~я";
        non_past_first: "~чу~ь", "~тим~я";
        non_past_second: "~чешь~я", "~тите~ь";
        non_past_third: "~чет~я", "~тят~я";
        past: "~тел~я", "~тела~ь", "~тело~ь", "~тели~ь";
        imperative: "~ти~ь", "~тите~ь";
        participle: "~тящий~я", "~тевший~я", "~е~емый", "~ё~нный";
        gerund: "~тя~ь", "~тевши~ь";
    ),
    // Вырасти: 3 words (вырасти, повырасти, подвырасти).
    conjugation!("вырасти", 3, "~а~ти";
        infinitive: "~а~ти";
        non_past_first: "~а~ту", "~а~тем";
        non_past_second: "~а~тешь", "~а~тете";
        non_past_third: "~а~тет", "~а~тут";
        past: "~о~", "~о~ла", "~о~ло", "~о~ли";
        imperative: "~а~ти", "~а~тите";
        participle: "~а~тущий", "~о~ший", "~е~емый", "~о~тённый";
        gerund: "~а~тя", "~о~ши";
    ),
    // Высвободить: 3 words (вынудить, высвободить, выродить).
    conjugation!("высвободить", 3, "~дить";
        infinitive: "~дить";
        non_past_first: "~жу", "~дим";
        non_past_second: "~дишь", "~дите";
        non_past_third: "~дит", "~дят";
        past: "~дил", "~дила", "~дило", "~дили";
        imperative: "~ди", "~дите";
        participle: "~дящий", "~дивший", "~димый", "~жденный";
        gerund: "~дя", "~див|~дивши";
    ),
    // Начаться: 3 words (начаться, початься, зачаться).
    conjugation!("начаться", 3, "~ать~я";
        infinitive: "~ать~я";
        non_past_first: "~ну~ь", "~нём~я";
        non_past_second: "~нёшь~я", "~нёте~ь";
        non_past_third: "~нёт~я", "~нут~я";
        past: "~ал~я", "~ала~ь", "~ало~ь", "~али~ь";
        imperative: "~ни~ь", "~ните~ь";
        participle: "~ащий~я", "~авший~я", "~е~емый", "~ё~нный";
        gerund: "~а~ь", "~авши~ь";
    ),
    // Переть: 3 words (тереть, мереть, переть).
    conjugation!("переть", 3, "~е~еть";
        infinitive: "~е~еть";
        non_past_first: "~~у", "~~ём";
        non_past_second: "~~ёшь", "~~ёте";
        non_past_third: "~~ёт", "~~ут";
        past: "~ё~", "~ё~ла", "~ё~ло", "~ё~ли";
        imperative: "~~и", "~~ите";
        participle: "~~ущий", "~ё~ший", "~е~омый", "~ё~тый";
        gerund: "~е~я", "~ё~ши";
    ),
    // Опоясать: 3 words (опоясать, запоясать, распоясать).
    conjugation!("опоясать", 3, "~сать";
        infinitive: "~сать";
        non_past_first: "~шу", "~шем";
        non_past_second: "~шешь", "~шете";
        non_past_third: "~шет", "~шут";
        past: "~сал", "~сала", "~сало", "~сали";
        imperative: "~шь", "~шьте";
        participle: "~шущий", "~савший", "~домый", "~санный";
        gerund: "~ша", "~сав|~савши";
    ),
    // Вянуть: 3 words (вянуть, завянуть, увянуть).
    conjugation!("вянуть", 3, "~нуть";
        infinitive: "~нуть";
        non_past_first: "~ну", "~нем";
        non_past_second: "~нешь", "~нете";
        non_past_third: "~нет", "~нут";
        past: "~л", "~ла", "~ло", "~ли";
        imperative: "~нь", "~ньте";
        participle: "~нущий", "~нувший", "~нимый", "~нутый";
        gerund: "~ня", "~нув|~нувши";
    ),
    // Заклясть: 3 words (проклясть, клясть, заклясть).
    conjugation!("заклясть", 3, "~сть";
        infinitive: "~сть";
        non_past_first: "~ну", "~нём";
        non_past_second: "~нёшь", "~нёте";
        non_past_third: "~нёт", "~нут";
        past: "~л", "~ла", "~ло", "~ли";
        imperative: "~ни", "~ните";
        participle: "~нущий", "~вший", "~нимый", "~тый";
        gerund: "~ня", "~в|~вши";
    ),
    // Клясться: 3 words (клясться, поклясться, заклясться).
    conjugation!("клясться", 3, "~~ться";
        infinitive: "~~ться";
        non_past_first: "~ну~ь", "~нём~я";
        non_past_second: "~нёшь~я", "~нёте~ь";
        non_past_third: "~нёт~я", "~нут~я";
        past: "~л~я|~л~я", "~ла~ь", "~ло~ь|~ло~ь", "~ли~ь|~ли~ь";
        imperative: "~ни~ь", "~ните~ь";
        participle: "~нущий~я", "~вший~я", "~е~емый", "~ё~нный";
        gerund: "~ня~ь", "~вши~ь";
    ),
    // Приняться: 3 words (приняться, восприняться, предприняться).
    conjugation!("приняться", 3, "~нять~я";
        infinitive: "~нять~я";
        non_past_first: "~му~ь", "~мем~я";
        non_past_second: "~мешь~я", "~мете~ь";
        non_past_third: "~мет~я", "~мут~я";
        past: "~нял~я", "~няла~ь", "~няло~ь", "~няли~ь";
        imperative: "~ми~ь", "~мите~ь";
        participle: "~мущий~я", "~нявший~я", "~е~емый", "~ё~нный";
        gerund: "~мя~ь", "~нявши~ь";
    ),
    // Вывести: 3 words (вывести, выбрести, повывести).
    conjugation!("вывести", 3, "~сти";
        infinitive: "~сти";
        non_past_first: "~ду", "~дем";
        non_past_second: "~дешь", "~дете";
        non_past_third: "~дет", "~дут";
        past: "~л", "~ла", "~ло", "~ли";
        imperative: "~ди", "~дите";
        participle: "~дущий", "~дший", "~домый", "~денный";
        gerund: "~дя", "~дя";
    ),
    // Выбраться: 3 words (выбраться, выдраться, повыбраться).
    conjugation!("выбраться", 3, "~~ать~я";
        infinitive: "~~ать~я";
        non_past_first: "~е~у~ь", "~е~ем~я";
        non_past_second: "~е~ешь~я", "~е~ете~ь";
        non_past_third: "~е~ет~я", "~е~ут~я";
        past: "~~ал~я", "~~ала~ь", "~~ало~ь", "~~али~ь";
        imperative: "~е~и~ь", "~е~ите~ь";
        participle: "~е~ющий~я", "~~авший~я", "~~ать~я", "~о~~анный";
        gerund: "~е~я~ь", "~~авши~ь";
    ),
    // Счесть: 3 words (счесть, обчесть, отчесть).
    conjugation!("счесть", 3, "~~есть";
        infinitive: "~~есть";
        non_past_first: "~о~ту", "~о~тём";
        non_past_second: "~о~тёшь", "~о~тёте";
        non_past_third: "~о~тёт", "~о~тут";
        past: "~~ёл", "~о~ла", "~о~ло", "~о~ли";
        imperative: "~о~ти", "~о~тите";
        participle: "~а~тущий", "~о~ший", "~е~емый", "~о~тённый";
        gerund: "~а~тя", "~о~тя|~~ётши";
    ),
    // Обмереть: 3 words (отпереть, обмереть, отмереть).
    conjugation!("обмереть", 3, "~~е~еть";
        infinitive: "~~е~еть";
        non_past_first: "~о~~у", "~о~~ём";
        non_past_second: "~о~~ёшь", "~о~~ёте";
        non_past_third: "~о~~ёт", "~о~~ут";
        past: "~~е~", "~~е~ла", "~~е~ло", "~~е~ли";
        imperative: "~о~~и", "~о~~ите";
        participle: "~е~ущий~я", "~~е~ший|~~е~ший", "~~е~еть", "~~е~тый";
        gerund: "~е~я~ь", "~~е~ев|~~е~ши|~~е~ши";
    ),
    // Понудить: 3 words (принудить, понудить, попринудить).
    conjugation!("понудить", 3, "~дить";
        infinitive: "~дить";
        non_past_first: "~жу", "~дим";
        non_past_second: "~дишь", "~дите";
        non_past_third: "~дит", "~дят";
        past: "~дил", "~дила", "~дило", "~дили";
        imperative: "~дь", "~дьте";
        participle: "~дящий", "~дивший", "~димый", "~ждённый";
        gerund: "~дя", "~див|~дивши";
    ),
    // Отозвать: 3 words (отозвать, обозвать, подозвать).
    conjugation!("отозвать", 3, "~о~~ать";
        infinitive: "~о~~ать";
        non_past_first: "~~о~у", "~~о~ём";
        non_past_second: "~~о~ёшь", "~~о~ёте";
        non_past_third: "~~о~ёт", "~~о~ут";
        past: "~о~~ал", "~о~~ала", "~о~~ало", "~о~~али";
        imperative: "~~о~и", "~~о~ите";
        participle: "~е~ущий~я", "~о~~авший", "~о~~ать", "~о~~анный";
        gerund: "~е~я~ь", "~о~~ав|~о~~авши";
    ),
    // Помутить: 3 words (замутить, помутить, намутить).
    conjugation!("помутить", 3, "~тить";
        infinitive: "~тить";
        non_past_first: "~чу", "~тим";
        non_past_second: "~тишь", "~тите";
        non_past_third: "~тит", "~тят";
        past: "~тил", "~тила", "~тило", "~тили";
        imperative: "~ти", "~тите";
        participle: "~тящий", "~тивший", "~тимый", "~тнённый";
        gerund: "~тя", "~тив|~тивши";
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
    // Осечься: 3 words (пересечься, осечься, сечься).
    conjugation!("осечься", 3, "~ечь~я";
        infinitive: "~ечь~я";
        non_past_first: "~еку~ь", "~ечём~я";
        non_past_second: "~ечёшь~я", "~ечёте~ь";
        non_past_third: "~ечёт~я", "~екут~я";
        past: "~ёк~я|~ек~я", "~екла~ь", "~екло~ь", "~екли~ь";
        imperative: "~еки~ь", "~еките~ь";
        participle: "~екущий~я", "~екший~я|~ёкший~я", "~е~емый", "~ё~нный";
        gerund: "~я~ь", "~екши~ь|~ёкши~ь";
    ),
    // Наморщиться: 3 words (встопорщиться, наморщиться, изморщиться).
    conjugation!("наморщиться", 3, "~ить~я";
        infinitive: "~ить~я";
        non_past_first: "~у~ь", "~им~я";
        non_past_second: "~ишь~я", "~ите~ь";
        non_past_third: "~ит~я", "~ят~я";
        past: "~ил~я", "~ила~ь", "~ило~ь", "~или~ь";
        imperative: "~ь~я|~и~ь", "~ьте~ь";
        participle: "~ящий~я", "~ивший~я", "~е~емый", "~ё~нный";
        gerund: "~я~ь", "~ивши~ь";
    ),
    // Чтиться: 3 words (чтиться, почтиться, узриться).
    conjugation!("чтиться", 3, "~ить~я";
        infinitive: "~ить~я";
        non_past_first: "~у~ь", "~им~я";
        non_past_second: "~ишь~я", "~ите~ь";
        non_past_third: "~ит~я", "~ят~я";
        past: "~ил~я", "~ила~ь", "~ило~ь", "~или~ь";
        imperative: "~и~ь", "~ите~ь";
        participle: "~ящий~я", "~ивший~я", "~е~емый", "~ё~нный";
        gerund: "~я~ь", "~ивши~ь|~я~ь";
    ),
    // Взбресть: 3 words (взбресть, добресть, набресть).
    conjugation!("взбресть", 3, "~есть";
        infinitive: "~есть";
        non_past_first: "~еду", "~едём";
        non_past_second: "~едёшь", "~едёте";
        non_past_third: "~едёт", "~едут";
        past: "~ёл", "~ела", "~ело", "~ели";
        imperative: "~еди", "~едите";
        participle: "~етущий", "~едший", "~имый", "~етённый";
        gerund: "~етя", "~едя";
    ),
    // Выпечься: 3 words (выволочься, выпечься, высечься).
    conjugation!("выпечься", 3, "~чь~я";
        infinitive: "~чь~я";
        non_past_first: "~ку~ь", "~чем~я";
        non_past_second: "~чешь~я", "~чете~ь";
        non_past_third: "~чет~я", "~кут~я";
        past: "~к~я", "~кла~ь", "~кло~ь", "~кли~ь";
        imperative: "~ки~ь", "~ките~ь";
        participle: "~кущий~я", "~кший~я", "~е~емый", "~ё~нный";
        gerund: "~ча~ь", "~кши~ь";
    ),
    // Испиться: 3 words (распиться, расшиться, испиться).
    conjugation!("испиться", 3, "~с~ить~я";
        infinitive: "~с~ить~я";
        non_past_first: "~зо~ью~ь", "~зо~ьём~я";
        non_past_second: "~зо~ьёшь~я", "~зо~ьёте~ь";
        non_past_third: "~зо~ьёт~я", "~зо~ьют~я";
        past: "~с~ил~я", "~с~ила~ь", "~с~ило~ь", "~с~или~ь";
        imperative: "~с~ей~я", "~с~ейте~ь";
        participle: "~е~ущий~я", "~с~ивший~я", "~с~ить~я", "~о~~анный";
        gerund: "~е~я~ь", "~с~ивши~ь";
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
    conjugation!("позаводить", 3, "~одить";
        infinitive: "~одить";
        non_past_first: "~ожу", "~одим";
        non_past_second: "~одишь", "~одите";
        non_past_third: "~одит", "~одят";
        past: "~одил", "~одила", "~одило", "~одили";
        imperative: "~оди", "~одите";
        participle: "~ющий", "~одивший", "~емый", "~едённый";
        gerund: "~я", "~одив|~одивши";
    ),
    // Закодить: 3 words (повыводить, закодить, накодить).
    conjugation!("закодить", 3, "~дить";
        infinitive: "~дить";
        non_past_first: "~жу", "~дим";
        non_past_second: "~дишь", "~дите";
        non_past_third: "~дит", "~дят";
        past: "~дил", "~дила", "~дило", "~дили";
        imperative: "~ди", "~дите";
        participle: "~дящий", "~дивший", "~димый", "~денный";
        gerund: "~дя", "~див|~дивши";
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
    // Рещи: 1 words (рещи).
    conjugation!("рещи", 1, "~ещи";
        infinitive: "~ещи";
        non_past_first: "~еку", "~ечём";
        non_past_second: "~ечёшь", "~ечёте";
        non_past_third: "~ечёт", "~екут";
        past: "~ёк", "~екла", "~екло", "~екли";
        imperative: "~еки", "~еките";
        participle: "~екущий", "~ёкший", "~екомый", "~ечённый";
        gerund: "~я", "~ёкши";
    ),
];
