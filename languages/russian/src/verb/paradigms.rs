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
    /// Like `играть`: 4,439 words.
    Играть,
    /// Like `обнажаться`: 3,108 words.
    Обнажаться,
    /// Like `вибрировать`: 2,951 words.
    Вибрировать,
    /// Like `ыковаться`: 1,612 words.
    Ыковаться,
    /// Like `хуярить`: 643 words.
    Хуярить,
    /// Like `чать`: 563 words.
    Чать,
    /// Like `диться`: 505 words.
    Диться,
    /// Like `палить`: 472 words.
    Палить,
    /// Like `рвануть`: 380 words.
    Рвануть,
    /// Like `назначить`: 377 words.
    Назначить,
    /// Like `тянуть`: 322 words.
    Тянуть,
    /// Like `илиться`: 319 words.
    Илиться,
    /// Like `обнажиться`: 223 words.
    Обнажиться,
    /// Like `выделить`: 218 words.
    Выделить,
    /// Like `лубиться`: 214 words.
    Лубиться,
    /// Like `обрушиться`: 210 words.
    Обрушиться,
    /// Like `ткнуться`: 199 words.
    Ткнуться,
    /// Like `лбить`: 179 words.
    Лбить,
    /// Like `ставиться`: 172 words.
    Ставиться,
    /// Like `ягчить`: 172 words.
    Ягчить,
    /// Like `ставить`: 166 words.
    Ставить,
    /// Like `залипнуть`: 156 words.
    Залипнуть,
    /// Like `становить`: 126 words.
    Становить,
    /// Like `тянуться`: 125 words.
    Тянуться,
    /// Like `водить`: 114 words.
    Водить,
    /// Like `хватить`: 111 words.
    Хватить,
    /// Like `гнездиться`: 103 words.
    Гнездиться,
    /// Like `отметить`: 102 words.
    Отметить,
    /// Like `сложить`: 96 words.
    Сложить,
    /// Like `намить`: 86 words.
    Намить,
    /// Like `стучать`: 71 words.
    Стучать,
    /// Like `крутиться`: 69 words.
    Крутиться,
    /// Like `третиться`: 68 words.
    Третиться,
    /// Like `сычевать`: 63 words.
    Сычевать,
    /// Like `гасить`: 61 words.
    Гасить,
    /// Like `перестроиться`: 59 words.
    Перестроиться,
    /// Like `ушеваться`: 55 words.
    Ушеваться,
    /// Like `елозить`: 53 words.
    Елозить,
    /// Like `очиститься`: 53 words.
    Очиститься,
    /// Like `озить`: 51 words.
    Озить,
    /// Like `птать`: 48 words.
    Птать,
    /// Like `перестроить`: 45 words.
    Перестроить,
    /// Like `таять`: 44 words.
    Таять,
    /// Like `осить`: 44 words.
    Осить,
    /// Like `глохнуть`: 44 words.
    Глохнуть,
    /// Like `местить`: 42 words.
    Местить,
    /// Like `ядить`: 41 words.
    Ядить,
    /// Like `разбить`: 41 words.
    Разбить,
    /// Like `устить`: 40 words.
    Устить,
    /// Like `разнести`: 39 words.
    Разнести,
    /// Like `ладить`: 38 words.
    Ладить,
    /// Like `ладиться`: 38 words.
    Ладиться,
    /// Like `ржаться`: 37 words.
    Ржаться,
    /// Like `носиться`: 36 words.
    Носиться,
    /// Like `сбиться`: 35 words.
    Сбиться,
    /// Like `возбудить`: 34 words.
    Возбудить,
    /// Like `возиться`: 34 words.
    Возиться,
    /// Like `кишеть`: 34 words.
    Кишеть,
    /// Like `развести`: 33 words.
    Развести,
    /// Like `зти`: 33 words.
    Зти,
    /// Like `инуть`: 32 words.
    Инуть,
    /// Like `левать`: 32 words.
    Левать,
    /// Like `лизиться`: 32 words.
    Лизиться,
    /// Like `переброситься`: 31 words.
    Переброситься,
    /// Like `каяться`: 31 words.
    Каяться,
    /// Like `птить`: 30 words.
    Птить,
    /// Like `ебать`: 28 words.
    Ебать,
    /// Like `хтеть`: 28 words.
    Хтеть,
    /// Like `сообразить`: 26 words.
    Сообразить,
    /// Like `раться`: 26 words.
    Раться,
    /// Like `сдавать`: 26 words.
    Сдавать,
    /// Like `атиться`: 25 words.
    Атиться,
    /// Like `щитить`: 24 words.
    Щитить,
    /// Like `клепать`: 23 words.
    Клепать,
    /// Like `вывестись`: 23 words.
    Вывестись,
    /// Like `ястись`: 22 words.
    Ястись,
    /// Like `крыть`: 20 words.
    Крыть,
    /// Like `гореться`: 20 words.
    Гореться,
    /// Like `шить`: 19 words.
    Шить,
    /// Like `виснуть`: 19 words.
    Виснуть,
    /// Like `поехать`: 18 words.
    Поехать,
    /// Like `виться`: 18 words.
    Виться,
    /// Like `лести`: 18 words.
    Лести,
    /// Like `сидеть`: 18 words.
    Сидеть,
    /// Like `насесть`: 17 words.
    Насесть,
    /// Like `ласить`: 17 words.
    Ласить,
    /// Like `сдвинуться`: 17 words.
    Сдвинуться,
    /// Like `унестись`: 17 words.
    Унестись,
    /// Like `гореть`: 17 words.
    Гореть,
    /// Like `оргнуться`: 17 words.
    Оргнуться,
    /// Like `пасть`: 17 words.
    Пасть,
    /// Like `рыться`: 16 words.
    Рыться,
    /// Like `плакать`: 16 words.
    Плакать,
    /// Like `вырвать`: 16 words.
    Вырвать,
    /// Like `даваться`: 16 words.
    Даваться,
    /// Like `вернуть`: 16 words.
    Вернуть,
    /// Like `глядеться`: 16 words.
    Глядеться,
    /// Like `азаться`: 15 words.
    Азаться,
    /// Like `жевать`: 15 words.
    Жевать,
    /// Like `сыпаться`: 15 words.
    Сыпаться,
    /// Like `разобрать`: 14 words.
    Разобрать,
    /// Like `пасться`: 14 words.
    Пасться,
    /// Like `птаться`: 14 words.
    Птаться,
    /// Like `язать`: 14 words.
    Язать,
    /// Like `сдать`: 13 words.
    Сдать,
    /// Like `пахать`: 13 words.
    Пахать,
    /// Like `попереть`: 13 words.
    Попереть,
    /// Like `лыхаться`: 13 words.
    Лыхаться,
    /// Like `нагореть`: 13 words.
    Нагореть,
    /// Like `тыть`: 13 words.
    Тыть,
    /// Like `рыскать`: 13 words.
    Рыскать,
    /// Like `напеть`: 12 words.
    Напеть,
    /// Like `прискакать`: 12 words.
    Прискакать,
    /// Like `ечь`: 12 words.
    Ечь,
    /// Like `драть`: 12 words.
    Драть,
    /// Like `воеваться`: 12 words.
    Воеваться,
    /// Like `истеть`: 12 words.
    Истеть,
    /// Like `браться`: 11 words.
    Браться,
    /// Like `зачесться`: 11 words.
    Зачесться,
    /// Like `рщить`: 11 words.
    Рщить,
    /// Like `честь`: 11 words.
    Честь,
    /// Like `вянуть`: 11 words.
    Вянуть,
    /// Like `рпеться`: 11 words.
    Рпеться,
    /// Like `писать`: 11 words.
    Писать,
    /// Like `ичься`: 11 words.
    Ичься,
    /// Like `вынести`: 10 words.
    Вынести,
    /// Like `подобраться`: 10 words.
    Подобраться,
    /// Like `помять`: 10 words.
    Помять,
    /// Like `влечься`: 10 words.
    Влечься,
    /// Like `лыться`: 10 words.
    Лыться,
    /// Like `спеться`: 10 words.
    Спеться,
    /// Like `опереться`: 10 words.
    Опереться,
    /// Like `лететься`: 10 words.
    Лететься,
    /// Like `тчевать`: 10 words.
    Тчевать,
    /// Like `видеть`: 9 words.
    Видеть,
    /// Like `тичь`: 9 words.
    Тичь,
    /// Like `петать`: 9 words.
    Петать,
    /// Like `обрести`: 9 words.
    Обрести,
    /// Like `сечь`: 9 words.
    Сечь,
    /// Like `ыпать`: 9 words.
    Ыпать,
    /// Like `чесаться`: 9 words.
    Чесаться,
    /// Like `статься`: 9 words.
    Статься,
    /// Like `уйти`: 9 words.
    Уйти,
    /// Like `резать`: 9 words.
    Резать,
    /// Like `жить`: 8 words.
    Жить,
    /// Like `стереть`: 8 words.
    Стереть,
    /// Like `добыть`: 8 words.
    Добыть,
    /// Like `врезаться`: 8 words.
    Врезаться,
    /// Like `размять`: 8 words.
    Размять,
    /// Like `равнять`: 8 words.
    Равнять,
    /// Like `мяться`: 8 words.
    Мяться,
    /// Like `зиждить`: 8 words.
    Зиждить,
    /// Like `пакоститься`: 8 words.
    Пакоститься,
    /// Like `грестись`: 8 words.
    Грестись,
    /// Like `лоть`: 8 words.
    Лоть,
    /// Like `оесть`: 7 words.
    Оесть,
    /// Like `сковать`: 7 words.
    Сковать,
    /// Like `одеть`: 7 words.
    Одеть,
    /// Like `разогнать`: 7 words.
    Разогнать,
    /// Like `чесать`: 7 words.
    Чесать,
    /// Like `лезть`: 7 words.
    Лезть,
    /// Like `выслать`: 7 words.
    Выслать,
    /// Like `отозваться`: 7 words.
    Отозваться,
    /// Like `смяться`: 7 words.
    Смяться,
    /// Like `прячь`: 7 words.
    Прячь,
    /// Like `ытить`: 7 words.
    Ытить,
    /// Like `выслаться`: 7 words.
    Выслаться,
    /// Like `речься`: 7 words.
    Речься,
    /// Like `ызть`: 7 words.
    Ызть,
    /// Like `опорить`: 7 words.
    Опорить,
    /// Like `разжечь`: 6 words.
    Разжечь,
    /// Like `вырасти`: 6 words.
    Вырасти,
    /// Like `даться`: 6 words.
    Даться,
    /// Like `вылить`: 6 words.
    Вылить,
    /// Like `жать`: 6 words.
    Жать,
    /// Like `сжаться`: 6 words.
    Сжаться,
    /// Like `шибить`: 6 words.
    Шибить,
    /// Like `плескаться`: 6 words.
    Плескаться,
    /// Like `тереться`: 6 words.
    Тереться,
    /// Like `стлать`: 6 words.
    Стлать,
    /// Like `хлестать`: 6 words.
    Хлестать,
    /// Like `обжечься`: 6 words.
    Обжечься,
    /// Like `исхлестаться`: 6 words.
    Исхлестаться,
    /// Like `ечься`: 6 words.
    Ечься,
    /// Like `стелиться`: 6 words.
    Стелиться,
    /// Like `вырваться`: 6 words.
    Вырваться,
    /// Like `лоться`: 6 words.
    Лоться,
    /// Like `выгрести`: 6 words.
    Выгрести,
    /// Like `рякнуть`: 6 words.
    Рякнуть,
    /// Like `лгнуть`: 6 words.
    Лгнуть,
    /// Like `вгрызться`: 6 words.
    Вгрызться,
    /// Like `смотреть`: 5 words.
    Смотреть,
    /// Like `бежать`: 5 words.
    Бежать,
    /// Like `жечь`: 5 words.
    Жечь,
    /// Like `идеть`: 5 words.
    Идеть,
    /// Like `мести`: 5 words.
    Мести,
    /// Like `стояться`: 5 words.
    Стояться,
    /// Like `вязать`: 5 words.
    Вязать,
    /// Like `отжать`: 5 words.
    Отжать,
    /// Like `видеться`: 5 words.
    Видеться,
    /// Like `пыхать`: 5 words.
    Пыхать,
    /// Like `сесться`: 5 words.
    Сесться,
    /// Like `изъять`: 5 words.
    Изъять,
    /// Like `обмереть`: 5 words.
    Обмереть,
    /// Like `унять`: 5 words.
    Унять,
    /// Like `лестись`: 5 words.
    Лестись,
    /// Like `тыкаться`: 5 words.
    Тыкаться,
    /// Like `зачертить`: 5 words.
    Зачертить,
    /// Like `бечь`: 5 words.
    Бечь,
    /// Like `няться`: 5 words.
    Няться,
    /// Like `рщиться`: 5 words.
    Рщиться,
    /// Like `ынуть`: 5 words.
    Ынуть,
    /// Like `обнять`: 5 words.
    Обнять,
    /// Like `трепать`: 5 words.
    Трепать,
    /// Like `вигнуться`: 5 words.
    Вигнуться,
    /// Like `усориться`: 5 words.
    Усориться,
    /// Like `лечь`: 4 words.
    Лечь,
    /// Like `нести`: 4 words.
    Нести,
    /// Like `плестись`: 4 words.
    Плестись,
    /// Like `жаться`: 4 words.
    Жаться,
    /// Like `вынестись`: 4 words.
    Вынестись,
    /// Like `леваться`: 4 words.
    Леваться,
    /// Like `ушибиться`: 4 words.
    Ушибиться,
    /// Like `испить`: 4 words.
    Испить,
    /// Like `уебать`: 4 words.
    Уебать,
    /// Like `бороздить`: 4 words.
    Бороздить,
    /// Like `хвостить`: 4 words.
    Хвостить,
    /// Like `усять`: 4 words.
    Усять,
    /// Like `изъяться`: 4 words.
    Изъяться,
    /// Like `ытиться`: 4 words.
    Ытиться,
    /// Like `оять`: 4 words.
    Оять,
    /// Like `рчать`: 4 words.
    Рчать,
    /// Like `еяться`: 4 words.
    Еяться,
    /// Like `стричь`: 4 words.
    Стричь,
    /// Like `домить`: 4 words.
    Домить,
    /// Like `измыслить`: 4 words.
    Измыслить,
    /// Like `петаться`: 4 words.
    Петаться,
    /// Like `ведомиться`: 4 words.
    Ведомиться,
    /// Like `переть`: 3 words.
    Переть,
    /// Like `нестись`: 3 words.
    Нестись,
    /// Like `чтить`: 3 words.
    Чтить,
    /// Like `счесть`: 3 words.
    Счесть,
    /// Like `соваться`: 3 words.
    Соваться,
    /// Like `отозвать`: 3 words.
    Отозвать,
    /// Like `выдвинуться`: 3 words.
    Выдвинуться,
    /// Like `уняться`: 3 words.
    Уняться,
    /// Like `истереть`: 3 words.
    Истереть,
    /// Like `лесить`: 3 words.
    Лесить,
    /// Like `колебать`: 3 words.
    Колебать,
    /// Like `выклевать`: 3 words.
    Выклевать,
    /// Like `выкласть`: 3 words.
    Выкласть,
    /// Like `оясаться`: 3 words.
    Оясаться,
    /// Like `растереться`: 3 words.
    Растереться,
    /// Like `испиться`: 3 words.
    Испиться,
    /// Like `срастись`: 3 words.
    Срастись,
    /// Like `закодить`: 3 words.
    Закодить,
    /// Like `харассить`: 3 words.
    Харассить,
    /// Like `ятать`: 3 words.
    Ятать,
    /// Like `ождать`: 3 words.
    Ождать,
    /// Like `ятаться`: 3 words.
    Ятаться,
    /// Like `одолеть`: 3 words.
    Одолеть,
    /// Like `лочься`: 3 words.
    Лочься,
    /// Like `быться`: 3 words.
    Быться,
    /// Like `ежаться`: 3 words.
    Ежаться,
    /// Like `истеться`: 3 words.
    Истеться,
    /// Like `заводить`: 3 words.
    Заводить,
    /// Like `рещи`: 1 words.
    Рещи,
    /// Like `ться`: 1 words.
    Ться,
    /// Like `чься`: 1 words.
    Чься,
}

impl Conjugation {
    /// Every conjugation, in order.
    pub const ALL: [Conjugation; 269] = [
        Conjugation::Играть,
        Conjugation::Обнажаться,
        Conjugation::Вибрировать,
        Conjugation::Ыковаться,
        Conjugation::Хуярить,
        Conjugation::Чать,
        Conjugation::Диться,
        Conjugation::Палить,
        Conjugation::Рвануть,
        Conjugation::Назначить,
        Conjugation::Тянуть,
        Conjugation::Илиться,
        Conjugation::Обнажиться,
        Conjugation::Выделить,
        Conjugation::Лубиться,
        Conjugation::Обрушиться,
        Conjugation::Ткнуться,
        Conjugation::Лбить,
        Conjugation::Ставиться,
        Conjugation::Ягчить,
        Conjugation::Ставить,
        Conjugation::Залипнуть,
        Conjugation::Становить,
        Conjugation::Тянуться,
        Conjugation::Водить,
        Conjugation::Хватить,
        Conjugation::Гнездиться,
        Conjugation::Отметить,
        Conjugation::Сложить,
        Conjugation::Намить,
        Conjugation::Стучать,
        Conjugation::Крутиться,
        Conjugation::Третиться,
        Conjugation::Сычевать,
        Conjugation::Гасить,
        Conjugation::Перестроиться,
        Conjugation::Ушеваться,
        Conjugation::Елозить,
        Conjugation::Очиститься,
        Conjugation::Озить,
        Conjugation::Птать,
        Conjugation::Перестроить,
        Conjugation::Таять,
        Conjugation::Осить,
        Conjugation::Глохнуть,
        Conjugation::Местить,
        Conjugation::Ядить,
        Conjugation::Разбить,
        Conjugation::Устить,
        Conjugation::Разнести,
        Conjugation::Ладить,
        Conjugation::Ладиться,
        Conjugation::Ржаться,
        Conjugation::Носиться,
        Conjugation::Сбиться,
        Conjugation::Возбудить,
        Conjugation::Возиться,
        Conjugation::Кишеть,
        Conjugation::Развести,
        Conjugation::Зти,
        Conjugation::Инуть,
        Conjugation::Левать,
        Conjugation::Лизиться,
        Conjugation::Переброситься,
        Conjugation::Каяться,
        Conjugation::Птить,
        Conjugation::Ебать,
        Conjugation::Хтеть,
        Conjugation::Сообразить,
        Conjugation::Раться,
        Conjugation::Сдавать,
        Conjugation::Атиться,
        Conjugation::Щитить,
        Conjugation::Клепать,
        Conjugation::Вывестись,
        Conjugation::Ястись,
        Conjugation::Крыть,
        Conjugation::Гореться,
        Conjugation::Шить,
        Conjugation::Виснуть,
        Conjugation::Поехать,
        Conjugation::Виться,
        Conjugation::Лести,
        Conjugation::Сидеть,
        Conjugation::Насесть,
        Conjugation::Ласить,
        Conjugation::Сдвинуться,
        Conjugation::Унестись,
        Conjugation::Гореть,
        Conjugation::Оргнуться,
        Conjugation::Пасть,
        Conjugation::Рыться,
        Conjugation::Плакать,
        Conjugation::Вырвать,
        Conjugation::Даваться,
        Conjugation::Вернуть,
        Conjugation::Глядеться,
        Conjugation::Азаться,
        Conjugation::Жевать,
        Conjugation::Сыпаться,
        Conjugation::Разобрать,
        Conjugation::Пасться,
        Conjugation::Птаться,
        Conjugation::Язать,
        Conjugation::Сдать,
        Conjugation::Пахать,
        Conjugation::Попереть,
        Conjugation::Лыхаться,
        Conjugation::Нагореть,
        Conjugation::Тыть,
        Conjugation::Рыскать,
        Conjugation::Напеть,
        Conjugation::Прискакать,
        Conjugation::Ечь,
        Conjugation::Драть,
        Conjugation::Воеваться,
        Conjugation::Истеть,
        Conjugation::Браться,
        Conjugation::Зачесться,
        Conjugation::Рщить,
        Conjugation::Честь,
        Conjugation::Вянуть,
        Conjugation::Рпеться,
        Conjugation::Писать,
        Conjugation::Ичься,
        Conjugation::Вынести,
        Conjugation::Подобраться,
        Conjugation::Помять,
        Conjugation::Влечься,
        Conjugation::Лыться,
        Conjugation::Спеться,
        Conjugation::Опереться,
        Conjugation::Лететься,
        Conjugation::Тчевать,
        Conjugation::Видеть,
        Conjugation::Тичь,
        Conjugation::Петать,
        Conjugation::Обрести,
        Conjugation::Сечь,
        Conjugation::Ыпать,
        Conjugation::Чесаться,
        Conjugation::Статься,
        Conjugation::Уйти,
        Conjugation::Резать,
        Conjugation::Жить,
        Conjugation::Стереть,
        Conjugation::Добыть,
        Conjugation::Врезаться,
        Conjugation::Размять,
        Conjugation::Равнять,
        Conjugation::Мяться,
        Conjugation::Зиждить,
        Conjugation::Пакоститься,
        Conjugation::Грестись,
        Conjugation::Лоть,
        Conjugation::Оесть,
        Conjugation::Сковать,
        Conjugation::Одеть,
        Conjugation::Разогнать,
        Conjugation::Чесать,
        Conjugation::Лезть,
        Conjugation::Выслать,
        Conjugation::Отозваться,
        Conjugation::Смяться,
        Conjugation::Прячь,
        Conjugation::Ытить,
        Conjugation::Выслаться,
        Conjugation::Речься,
        Conjugation::Ызть,
        Conjugation::Опорить,
        Conjugation::Разжечь,
        Conjugation::Вырасти,
        Conjugation::Даться,
        Conjugation::Вылить,
        Conjugation::Жать,
        Conjugation::Сжаться,
        Conjugation::Шибить,
        Conjugation::Плескаться,
        Conjugation::Тереться,
        Conjugation::Стлать,
        Conjugation::Хлестать,
        Conjugation::Обжечься,
        Conjugation::Исхлестаться,
        Conjugation::Ечься,
        Conjugation::Стелиться,
        Conjugation::Вырваться,
        Conjugation::Лоться,
        Conjugation::Выгрести,
        Conjugation::Рякнуть,
        Conjugation::Лгнуть,
        Conjugation::Вгрызться,
        Conjugation::Смотреть,
        Conjugation::Бежать,
        Conjugation::Жечь,
        Conjugation::Идеть,
        Conjugation::Мести,
        Conjugation::Стояться,
        Conjugation::Вязать,
        Conjugation::Отжать,
        Conjugation::Видеться,
        Conjugation::Пыхать,
        Conjugation::Сесться,
        Conjugation::Изъять,
        Conjugation::Обмереть,
        Conjugation::Унять,
        Conjugation::Лестись,
        Conjugation::Тыкаться,
        Conjugation::Зачертить,
        Conjugation::Бечь,
        Conjugation::Няться,
        Conjugation::Рщиться,
        Conjugation::Ынуть,
        Conjugation::Обнять,
        Conjugation::Трепать,
        Conjugation::Вигнуться,
        Conjugation::Усориться,
        Conjugation::Лечь,
        Conjugation::Нести,
        Conjugation::Плестись,
        Conjugation::Жаться,
        Conjugation::Вынестись,
        Conjugation::Леваться,
        Conjugation::Ушибиться,
        Conjugation::Испить,
        Conjugation::Уебать,
        Conjugation::Бороздить,
        Conjugation::Хвостить,
        Conjugation::Усять,
        Conjugation::Изъяться,
        Conjugation::Ытиться,
        Conjugation::Оять,
        Conjugation::Рчать,
        Conjugation::Еяться,
        Conjugation::Стричь,
        Conjugation::Домить,
        Conjugation::Измыслить,
        Conjugation::Петаться,
        Conjugation::Ведомиться,
        Conjugation::Переть,
        Conjugation::Нестись,
        Conjugation::Чтить,
        Conjugation::Счесть,
        Conjugation::Соваться,
        Conjugation::Отозвать,
        Conjugation::Выдвинуться,
        Conjugation::Уняться,
        Conjugation::Истереть,
        Conjugation::Лесить,
        Conjugation::Колебать,
        Conjugation::Выклевать,
        Conjugation::Выкласть,
        Conjugation::Оясаться,
        Conjugation::Растереться,
        Conjugation::Испиться,
        Conjugation::Срастись,
        Conjugation::Закодить,
        Conjugation::Харассить,
        Conjugation::Ятать,
        Conjugation::Ождать,
        Conjugation::Ятаться,
        Conjugation::Одолеть,
        Conjugation::Лочься,
        Conjugation::Быться,
        Conjugation::Ежаться,
        Conjugation::Истеться,
        Conjugation::Заводить,
        Conjugation::Рещи,
        Conjugation::Ться,
        Conjugation::Чься,
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

pub(super) static CONJUGATIONS: [Paradigm<'static>; 269] = [
    // Играть: 4,439 words (знать, играть, скучать, сделать, обедать, какать).
    conjugation!("играть", 4439, "~ть";
        infinitive: "~ть";
        non_past_first: "~ю", "~ем";
        non_past_second: "~ешь", "~ете";
        non_past_third: "~ет", "~ют";
        past: "~л", "~ла", "~ло", "~ли";
        imperative: "~й", "~йте";
        participle: "~ющий", "~вший", "~емый", "~нный";
        gerund: "~я", "~в|~вши";
    ),
    // Обнажаться: 3,108 words (являться, пытаться, хвастаться, обижаться, качаться, вращаться).
    conjugation!("обнажаться", 3108, "~ть~я";
        infinitive: "~ть~я";
        non_past_first: "~ю~ь", "~ем~я";
        non_past_second: "~ешь~я", "~ете~ь";
        non_past_third: "~ет~я", "~ют~я";
        past: "~л~я", "~ла~ь", "~ло~ь", "~ли~ь";
        imperative: "~й~я", "~йте~ь";
        participle: "~ющий~я", "~вший~я", "~е~емый", "~~итый";
        gerund: "~я~ь", "~вши~ь";
    ),
    // Вибрировать: 2,951 words (мульчировать, чувствовать, целовать, фехтовать, выковать, рисовать).
    conjugation!("вибрировать", 2951, "~овать";
        infinitive: "~овать";
        non_past_first: "~ую", "~уем";
        non_past_second: "~уешь", "~уете";
        non_past_third: "~ует", "~уют";
        past: "~овал", "~овала", "~овало", "~овали";
        imperative: "~уй", "~уйте";
        participle: "~ующий", "~овавший", "~уемый", "~ованный";
        gerund: "~уя", "~овав|~овавши";
    ),
    // Ыковаться: 1,612 words (тусоваться, чувствоваться, целоваться, жаловаться, ориентироваться, англизироваться).
    conjugation!("ыковаться", 1612, "~овать~я";
        infinitive: "~овать~я";
        non_past_first: "~ую~ь", "~уем~я";
        non_past_second: "~уешь~я", "~уете~ь";
        non_past_third: "~ует~я", "~уют~я";
        past: "~овал~я", "~овала~ь", "~овало~ь", "~овали~ь";
        imperative: "~уй~я", "~уйте~ь";
        participle: "~ующий~я", "~овавший~я", "~е~емый", "~~итый";
        gerund: "~уя~ь", "~овавши~ь";
    ),
    // Хуярить: 643 words (жалить, ужалить, жарить, халтурить, уволить, ускорить).
    conjugation!("хуярить", 643, "~ить";
        infinitive: "~ить";
        non_past_first: "~ю", "~им";
        non_past_second: "~ишь", "~ите";
        non_past_third: "~ит", "~ят";
        past: "~ил", "~ила", "~ило", "~или";
        imperative: "~ь", "~ьте";
        participle: "~ящий", "~ивший", "~имый", "~енный";
        gerund: "~я", "~ив|~ивши";
    ),
    // Чать: 563 words (взять, иметь, греть, хуеть, уметь, спеть).
    conjugation!("чать", 563, "~ть";
        infinitive: "~ть";
        non_past_first: "~ю", "~ем";
        non_past_second: "~ешь", "~ете";
        non_past_third: "~ет", "~ют";
        past: "~л", "~ла", "~ло", "~ли";
        imperative: "~й", "~йте";
        participle: "~ющий", "~вший", "~емый", "~тый";
        gerund: "~я", "~в|~вши";
    ),
    // Диться: 505 words (валиться, молиться, говориться, очутиться, длиться, твориться).
    conjugation!("диться", 505, "~ить~я";
        infinitive: "~ить~я";
        non_past_first: "~ю~ь", "~им~я";
        non_past_second: "~ишь~я", "~ите~ь";
        non_past_third: "~ит~я", "~ят~я";
        past: "~ил~я", "~ила~ь", "~ило~ь", "~или~ь";
        imperative: "~и~ь", "~ите~ь";
        participle: "~ящий~я", "~ивший~я", "~е~емый", "~~итый";
        gerund: "~я~ь", "~ивши~ь";
    ),
    // Палить: 472 words (говорить, благодарить, винить, хранить, озарить, делить).
    conjugation!("палить", 472, "~ить";
        infinitive: "~ить";
        non_past_first: "~ю", "~им";
        non_past_second: "~ишь", "~ите";
        non_past_third: "~ит", "~ят";
        past: "~ил", "~ила", "~ило", "~или";
        imperative: "~и", "~ите";
        participle: "~ящий", "~ивший", "~имый", "~ённый";
        gerund: "~я", "~ив|~ивши";
    ),
    // Рвануть: 380 words (хохотнуть, гнуть, ткнуть, икнуть, отдохнуть, чихнуть).
    conjugation!("рвануть", 380, "~уть";
        infinitive: "~уть";
        non_past_first: "~у", "~ём";
        non_past_second: "~ёшь", "~ёте";
        non_past_third: "~ёт", "~ут";
        past: "~ул", "~ула", "~уло", "~ули";
        imperative: "~и", "~ите";
        participle: "~ущий", "~увший", "~омый", "~утый";
        gerund: "~я", "~ув|~увши";
    ),
    // Назначить: 377 words (увеличить, значить, уничтожить, утешить, канючить, мучить).
    conjugation!("назначить", 377, "~ить";
        infinitive: "~ить";
        non_past_first: "~у", "~им";
        non_past_second: "~ишь", "~ите";
        non_past_third: "~ит", "~ат";
        past: "~ил", "~ила", "~ило", "~или";
        imperative: "~ь", "~ьте";
        participle: "~ащий", "~ивший", "~имый", "~енный";
        gerund: "~а", "~ив|~ивши";
    ),
    // Тянуть: 322 words (выковырнуть, капнуть, шлёпнуть, взглянуть, крикнуть, тянуть).
    conjugation!("тянуть", 322, "~уть";
        infinitive: "~уть";
        non_past_first: "~у", "~ем";
        non_past_second: "~ешь", "~ете";
        non_past_third: "~ет", "~ут";
        past: "~ул", "~ула", "~уло", "~ули";
        imperative: "~и", "~ите";
        participle: "~ущий", "~увший", "~омый", "~утый";
        gerund: "~ая", "~ув|~увши";
    ),
    // Илиться: 319 words (пялиться, ускориться, целиться, силиться, хмуриться, щуриться).
    conjugation!("илиться", 319, "~ить~я";
        infinitive: "~ить~я";
        non_past_first: "~ю~ь", "~им~я";
        non_past_second: "~ишь~я", "~ите~ь";
        non_past_third: "~ит~я", "~ят~я";
        past: "~ил~я", "~ила~ь", "~ило~ь", "~или~ь";
        imperative: "~ь~я", "~ьте~ь";
        participle: "~ящий~я", "~ивший~я", "~е~емый", "~~итый";
        gerund: "~я~ь", "~ивши~ь|~я~ь";
    ),
    // Обнажиться: 223 words (ложиться, кружиться, ухудшиться, улучшиться, решиться, лечиться).
    conjugation!("обнажиться", 223, "~ить~я";
        infinitive: "~ить~я";
        non_past_first: "~у~ь", "~им~я";
        non_past_second: "~ишь~я", "~ите~ь";
        non_past_third: "~ит~я", "~ат~я";
        past: "~ил~я", "~ила~ь", "~ило~ь", "~или~ь";
        imperative: "~и~ь", "~ите~ь";
        participle: "~ащий~я", "~ивший~я", "~е~емый", "~~итый";
        gerund: "~а~ь", "~ивши~ь";
    ),
    // Выделить: 218 words (чинить, молить, хвалить, хоронить, мыслить, исполнить).
    conjugation!("выделить", 218, "~ить";
        infinitive: "~ить";
        non_past_first: "~ю", "~им";
        non_past_second: "~ишь", "~ите";
        non_past_third: "~ит", "~ят";
        past: "~ил", "~ила", "~ило", "~или";
        imperative: "~и", "~ите";
        participle: "~ящий", "~ивший", "~имый", "~енный";
        gerund: "~я", "~ив|~ивши";
    ),
    // Лубиться: 214 words (влюбиться, явиться, вцепиться, дымиться, давиться, копиться).
    conjugation!("лубиться", 214, "~ить~я";
        infinitive: "~ить~я";
        non_past_first: "~лю~ь", "~им~я";
        non_past_second: "~ишь~я", "~ите~ь";
        non_past_third: "~ит~я", "~ят~я";
        past: "~ил~я", "~ила~ь", "~ило~ь", "~или~ь";
        imperative: "~и~ь", "~ите~ь";
        participle: "~ящий~я", "~ивший~я", "~е~емый", "~~итый";
        gerund: "~я~ь", "~ивши~ь";
    ),
    // Обрушиться: 210 words (дурачиться, мучиться, замучиться, обрушиться, значиться, рушиться).
    conjugation!("обрушиться", 210, "~ить~я";
        infinitive: "~ить~я";
        non_past_first: "~у~ь", "~им~я";
        non_past_second: "~ишь~я", "~ите~ь";
        non_past_third: "~ит~я", "~ат~я";
        past: "~ил~я", "~ила~ь", "~ило~ь", "~или~ь";
        imperative: "~ь~я", "~ьте~ь";
        participle: "~ащий~я", "~ивший~я", "~е~емый", "~~итый";
        gerund: "~а~ь", "~ивши~ь";
    ),
    // Ткнуться: 199 words (улыбнуться, качнуться, гнуться, очнуться, рвануться, распахнуться).
    conjugation!("ткнуться", 199, "~уть~я";
        infinitive: "~уть~я";
        non_past_first: "~у~ь", "~ём~я";
        non_past_second: "~ёшь~я", "~ёте~ь";
        non_past_third: "~ёт~я", "~ут~я";
        past: "~ул~я", "~ула~ь", "~уло~ь", "~ули~ь";
        imperative: "~и~ь", "~ите~ь";
        participle: "~ущий~я", "~увший~я", "~е~емый", "~~итый";
        gerund: "~я~ь", "~увши~ь";
    ),
    // Лбить: 179 words (наступить, дымить, выступить, возопить, язвить, гневить).
    conjugation!("лбить", 179, "~ить";
        infinitive: "~ить";
        non_past_first: "~лю", "~им";
        non_past_second: "~ишь", "~ите";
        non_past_third: "~ит", "~ят";
        past: "~ил", "~ила", "~ило", "~или";
        imperative: "~и", "~ите";
        participle: "~ящий", "~ивший", "~имый", "~лённый";
        gerund: "~я", "~ив|~ивши";
    ),
    // Ставиться: 172 words (нравиться, готовиться, знакомиться, ознакомиться, славиться, условиться).
    conjugation!("ставиться", 172, "~ить~я";
        infinitive: "~ить~я";
        non_past_first: "~лю~ь", "~им~я";
        non_past_second: "~ишь~я", "~ите~ь";
        non_past_third: "~ит~я", "~ят~я";
        past: "~ил~я", "~ила~ь", "~ило~ь", "~или~ь";
        imperative: "~ь~я", "~ьте~ь";
        participle: "~ящий~я", "~ивший~я", "~е~емый", "~~итый";
        gerund: "~я~ь", "~ивши~ь";
    ),
    // Ягчить: 172 words (решить, отличить, лишить, различить, дружить, ужесточить).
    conjugation!("ягчить", 172, "~ить";
        infinitive: "~ить";
        non_past_first: "~у", "~им";
        non_past_second: "~ишь", "~ите";
        non_past_third: "~ит", "~ат";
        past: "~ил", "~ила", "~ило", "~или";
        imperative: "~и", "~ите";
        participle: "~ащий", "~ивший", "~имый", "~ённый";
        gerund: "~а", "~ив|~ивши";
    ),
    // Ставить: 166 words (готовить, приспособить, славить, лукавить, обособить, расслабить).
    conjugation!("ставить", 166, "~ить";
        infinitive: "~ить";
        non_past_first: "~лю", "~им";
        non_past_second: "~ишь", "~ите";
        non_past_third: "~ит", "~ят";
        past: "~ил", "~ила", "~ило", "~или";
        imperative: "~ь", "~ьте";
        participle: "~ящий", "~ивший", "~имый", "~ленный";
        gerund: "~я", "~ив|~ивши";
    ),
    // Залипнуть: 156 words (распухнуть, сдохнуть, высохнуть, погибнуть, прилипнуть, прокиснуть).
    conjugation!("залипнуть", 156, "~нуть";
        infinitive: "~нуть";
        non_past_first: "~ну", "~нем";
        non_past_second: "~нешь", "~нете";
        non_past_third: "~нет", "~нут";
        past: "~", "~ла", "~ло", "~ли";
        imperative: "~ни", "~ните";
        participle: "~нущий", "~ший", "~емый", "~нутый";
        gerund: "~я", "~ши";
    ),
    // Становить: 126 words (уступить, любить, купить, ловить, топить, рубить).
    conjugation!("становить", 126, "~ить";
        infinitive: "~ить";
        non_past_first: "~лю", "~им";
        non_past_second: "~ишь", "~ите";
        non_past_third: "~ит", "~ят";
        past: "~ил", "~ила", "~ило", "~или";
        imperative: "~и", "~ите";
        participle: "~ящий", "~ивший", "~имый", "~ленный";
        gerund: "~я", "~ив|~ивши";
    ),
    // Тянуться: 125 words (чокнуться, тянуться, плюхнуться, вывернуться, шлёпнуться, апнуться).
    conjugation!("тянуться", 125, "~уть~я";
        infinitive: "~уть~я";
        non_past_first: "~у~ь", "~ем~я";
        non_past_second: "~ешь~я", "~ете~ь";
        non_past_third: "~ет~я", "~ут~я";
        past: "~ул~я", "~ула~ь", "~уло~ь", "~ули~ь";
        imperative: "~и~ь", "~ите~ь";
        participle: "~ущий~я", "~увший~я", "~е~емый", "~~итый";
        gerund: "~ая~ь", "~увши~ь";
    ),
    // Водить: 114 words (ходить, ездить, уходить, входить, вынудить, удить).
    conjugation!("водить", 114, "~дить";
        infinitive: "~дить";
        non_past_first: "~жу", "~дим";
        non_past_second: "~дишь", "~дите";
        non_past_third: "~дит", "~дят";
        past: "~дил", "~дила", "~дило", "~дили";
        imperative: "~ди", "~дите";
        participle: "~дящий", "~дивший", "~димый", "~женный";
        gerund: "~дя", "~див|~дивши";
    ),
    // Хватить: 111 words (платить, катить, хватить, чертить, ухватить, очертить).
    conjugation!("хватить", 111, "~тить";
        infinitive: "~тить";
        non_past_first: "~чу", "~тим";
        non_past_second: "~тишь", "~тите";
        non_past_third: "~тит", "~тят";
        past: "~тил", "~тила", "~тило", "~тили";
        imperative: "~ти", "~тите";
        participle: "~тящий", "~тивший", "~тимый", "~ченный";
        gerund: "~тя", "~тив|~тивши";
    ),
    // Гнездиться: 103 words (насладиться, гордиться, годиться, вводиться, плодиться, гнездиться).
    conjugation!("гнездиться", 103, "~дить~я";
        infinitive: "~дить~я";
        non_past_first: "~жу~ь", "~дим~я";
        non_past_second: "~дишь~я", "~дите~ь";
        non_past_third: "~дит~я", "~дят~я";
        past: "~дил~я", "~дила~ь", "~дило~ь", "~дили~ь";
        imperative: "~ди~ь", "~дите~ь";
        participle: "~дящий~я", "~дивший~я", "~е~емый", "~~итый";
        gerund: "~дя~ь", "~дивши~ь";
    ),
    // Отметить: 102 words (заметить, отметить, пометить, озаботить, пятить, лохматить).
    conjugation!("отметить", 102, "~тить";
        infinitive: "~тить";
        non_past_first: "~чу", "~тим";
        non_past_second: "~тишь", "~тите";
        non_past_third: "~тит", "~тят";
        past: "~тил", "~тила", "~тило", "~тили";
        imperative: "~ть", "~тьте";
        participle: "~тящий", "~тивший", "~тимый", "~ченный";
        gerund: "~тя", "~тив|~тивши";
    ),
    // Сложить: 96 words (положить, лечить, волочить, дрочить, сложить, иссушить).
    conjugation!("сложить", 96, "~ить";
        infinitive: "~ить";
        non_past_first: "~у", "~им";
        non_past_second: "~ишь", "~ите";
        non_past_third: "~ит", "~ат";
        past: "~ил", "~ила", "~ило", "~или";
        imperative: "~и", "~ите";
        participle: "~ащий", "~ивший", "~имый", "~енный";
        gerund: "~а", "~ив|~ивши";
    ),
    // Намить: 86 words (дыбить, швартовить, бесславить, голубить, гробить, дырявить).
    conjugation!("намить", 86, "~ить";
        infinitive: "~ить";
        non_past_first: "~лю", "~им";
        non_past_second: "~ишь", "~ите";
        non_past_third: "~ит", "~ят";
        past: "~ил", "~ила", "~ило", "~или";
        imperative: "~ь", "~ьте";
        participle: "~ящий", "~ивший", "~имый", "~ленный";
        gerund: "~я", "~ивши|~ив";
    ),
    // Стучать: 71 words (держать, жужжать, мычать, лежать, сдержать, дышать).
    conjugation!("стучать", 71, "~ать";
        infinitive: "~ать";
        non_past_first: "~у", "~им";
        non_past_second: "~ишь", "~ите";
        non_past_third: "~ит", "~ат";
        past: "~ал", "~ала", "~ало", "~али";
        imperative: "~и", "~ите";
        participle: "~ащий", "~авший", "~имый", "~анный";
        gerund: "~а", "~ав|~авши";
    ),
    // Крутиться: 69 words (катиться, хватиться, ухватиться, ютиться, платиться, кипятиться).
    conjugation!("крутиться", 69, "~тить~я";
        infinitive: "~тить~я";
        non_past_first: "~чу~ь", "~тим~я";
        non_past_second: "~тишь~я", "~тите~ь";
        non_past_third: "~тит~я", "~тят~я";
        past: "~тил~я", "~тила~ь", "~тило~ь", "~тили~ь";
        imperative: "~ти~ь", "~тите~ь";
        participle: "~тящий~я", "~тивший~я", "~е~емый", "~~итый";
        gerund: "~тя~ь", "~тивши~ь";
    ),
    // Третиться: 68 words (пятиться, озаботиться, горбатиться, жмотиться, выколотиться, обесцветиться).
    conjugation!("третиться", 68, "~тить~я";
        infinitive: "~тить~я";
        non_past_first: "~чу~ь", "~тим~я";
        non_past_second: "~тишь~я", "~тите~ь";
        non_past_third: "~тит~я", "~тят~я";
        past: "~тил~я", "~тила~ь", "~тило~ь", "~тили~ь";
        imperative: "~ть~я", "~тьте~ь";
        participle: "~тящий~я", "~тивший~я", "~е~емый", "~~итый";
        gerund: "~тя~ь", "~тивши~ь";
    ),
    // Сычевать: 63 words (сычевать, врачевать, глянцевать, гужевать, дрожжевать, лицевать).
    conjugation!("сычевать", 63, "~евать";
        infinitive: "~евать";
        non_past_first: "~ую", "~уем";
        non_past_second: "~уешь", "~уете";
        non_past_third: "~ует", "~уют";
        past: "~евал", "~евала", "~евало", "~евали";
        imperative: "~уй", "~уйте";
        participle: "~ующий", "~евавший", "~уемый", "~ованный";
        gerund: "~уя", "~евав|~евавши";
    ),
    // Гасить: 61 words (просить, уносить, плодоносить, вывесить, голосить, флексить).
    conjugation!("гасить", 61, "~сить";
        infinitive: "~сить";
        non_past_first: "~шу", "~сим";
        non_past_second: "~сишь", "~сите";
        non_past_third: "~сит", "~сят";
        past: "~сил", "~сила", "~сило", "~сили";
        imperative: "~си", "~сите";
        participle: "~сящий", "~сивший", "~симый", "~шенный";
        gerund: "~ся", "~сив|~сивши";
    ),
    // Перестроиться: 59 words (устроиться, выстроиться, пристроиться, перестроиться, встроиться, достроиться).
    conjugation!("перестроиться", 59, "~ить~я";
        infinitive: "~ить~я";
        non_past_first: "~ю~ь", "~им~я";
        non_past_second: "~ишь~я", "~ите~ь";
        non_past_third: "~ит~я", "~ят~я";
        past: "~ил~я", "~ила~ь", "~ило~ь", "~или~ь";
        imperative: "~й~я", "~йте~ь";
        participle: "~ящий~я", "~ивший~я", "~е~емый", "~~итый";
        gerund: "~я~ь", "~ивши~ь";
    ),
    // Ушеваться: 55 words (тушеваться, гужеваться, харчеваться, бичеваться, шприцеваться, фальцеваться).
    conjugation!("ушеваться", 55, "~евать~я";
        infinitive: "~евать~я";
        non_past_first: "~ую~ь", "~уем~я";
        non_past_second: "~уешь~я", "~уете~ь";
        non_past_third: "~ует~я", "~уют~я";
        past: "~евал~я", "~евала~ь", "~евало~ь", "~евали~ь";
        imperative: "~уй~я", "~уйте~ь";
        participle: "~ующий~я", "~евавший~я", "~е~емый", "~~итый";
        gerund: "~уя~ь", "~евавши~ь";
    ),
    // Елозить: 53 words (унизить, проказить, обезобразить, елозить, французить, мутузить).
    conjugation!("елозить", 53, "~зить";
        infinitive: "~зить";
        non_past_first: "~жу", "~зим";
        non_past_second: "~зишь", "~зите";
        non_past_third: "~зит", "~зят";
        past: "~зил", "~зила", "~зило", "~зили";
        imperative: "~зь", "~зьте";
        participle: "~зящий", "~зивший", "~зимый", "~женный";
        gerund: "~зя", "~зив|~зивши";
    ),
    // Очиститься: 53 words (проститься, очиститься, вместиться, посовеститься, чиститься, гоститься).
    conjugation!("очиститься", 53, "~~титься";
        infinitive: "~~титься";
        non_past_first: "~щу~ь", "~~тимся";
        non_past_second: "~~тишься", "~~титесь";
        non_past_third: "~~тится", "~~тятся";
        past: "~~тился", "~~тилась", "~~тилось", "~~тились";
        imperative: "~~тись", "~~титесь";
        participle: "~~тящийся", "~~тившийся", "~е~емый", "~~итый";
        gerund: "~~тясь", "~~тившись";
    ),
    // Озить: 51 words (возить, выразить, грузить, сгрузить, увозить, ввозить).
    conjugation!("озить", 51, "~зить";
        infinitive: "~зить";
        non_past_first: "~жу", "~зим";
        non_past_second: "~зишь", "~зите";
        non_past_third: "~зит", "~зят";
        past: "~зил", "~зила", "~зило", "~зили";
        imperative: "~зи", "~зите";
        participle: "~зящий", "~зивший", "~зимый", "~женный";
        gerund: "~зя", "~зив|~зивши";
    ),
    // Птать: 48 words (хохотать, щекотать, шептать, лепетать, хлопотать, метать).
    conjugation!("птать", 48, "~тать";
        infinitive: "~тать";
        non_past_first: "~чу", "~чем";
        non_past_second: "~чешь", "~чете";
        non_past_third: "~чет", "~чут";
        past: "~тал", "~тала", "~тало", "~тали";
        imperative: "~чи", "~чите";
        participle: "~чущий", "~тавший", "~тимый", "~танный";
        gerund: "~ча", "~тав|~тавши";
    ),
    // Перестроить: 45 words (клеить, устроить, подстроить, пристроить, перестроить, выстроить).
    conjugation!("перестроить", 45, "~ить";
        infinitive: "~ить";
        non_past_first: "~ю", "~им";
        non_past_second: "~ишь", "~ите";
        non_past_third: "~ит", "~ят";
        past: "~ил", "~ила", "~ило", "~или";
        imperative: "~й", "~йте";
        participle: "~ящий", "~ивший", "~имый", "~енный";
        gerund: "~я", "~ив|~ивши";
    ),
    // Таять: 44 words (баять, лаять, чуять, сеять, хаять, таять).
    conjugation!("таять", 44, "~ять";
        infinitive: "~ять";
        non_past_first: "~ю", "~ем";
        non_past_second: "~ешь", "~ете";
        non_past_third: "~ет", "~ют";
        past: "~ял", "~яла", "~яло", "~яли";
        imperative: "~й", "~йте";
        participle: "~ющий", "~явший", "~емый", "~янный";
        gerund: "~я", "~яв|~явши";
    ),
    // Осить: 44 words (хуесосить, дубасить, купоросить, тулумбасить, чудесить, фантомасить).
    conjugation!("осить", 44, "~сить";
        infinitive: "~сить";
        non_past_first: "~шу", "~сим";
        non_past_second: "~сишь", "~сите";
        non_past_third: "~сит", "~сят";
        past: "~сил", "~сила", "~сило", "~сили";
        imperative: "~сь", "~сьте";
        participle: "~сящий", "~сивший", "~симый", "~шенный";
        gerund: "~ся", "~сив|~сивши";
    ),
    // Глохнуть: 44 words (проникнуть, возникнуть, глохнуть, дрогнуть, потухнуть, погаснуть).
    conjugation!("глохнуть", 44, "~нуть";
        infinitive: "~нуть";
        non_past_first: "~ну", "~нем";
        non_past_second: "~нешь", "~нете";
        non_past_third: "~нет", "~нут";
        past: "~", "~ла", "~ло", "~ли";
        imperative: "~ни", "~ните";
        participle: "~нущий", "~ший|~нувший", "~емый", "~нутый";
        gerund: "~я", "~нув|~ши|~нувши";
    ),
    // Местить: 42 words (простить, растить, мстить, вместить, упростить, льстить).
    conjugation!("местить", 42, "~стить";
        infinitive: "~стить";
        non_past_first: "~щу", "~стим";
        non_past_second: "~стишь", "~стите";
        non_past_third: "~стит", "~стят";
        past: "~стил", "~стила", "~стило", "~стили";
        imperative: "~сти", "~стите";
        participle: "~стящий", "~стивший", "~стимый", "~щённый";
        gerund: "~стя", "~стив|~стивши";
    ),
    // Ядить: 41 words (щадить, отрядить, снарядить, разрядить, холодить, омолодить).
    conjugation!("ядить", 41, "~дить";
        infinitive: "~дить";
        non_past_first: "~жу", "~дим";
        non_past_second: "~дишь", "~дите";
        non_past_third: "~дит", "~дят";
        past: "~дил", "~дила", "~дило", "~дили";
        imperative: "~ди", "~дите";
        participle: "~дящий", "~дивший", "~димый", "~жённый";
        gerund: "~дя", "~див|~дивши";
    ),
    // Разбить: 41 words (сшить, разбить, сбить, свить, развить, разлить).
    conjugation!("разбить", 41, "~~ить";
        infinitive: "~~ить";
        non_past_first: "~о~ью", "~о~ьём";
        non_past_second: "~о~ьёшь", "~о~ьёте";
        non_past_third: "~о~ьёт", "~о~ьют";
        past: "~~ил", "~~ила", "~~ило", "~~или";
        imperative: "~~ей", "~~ейте";
        participle: "~ющий~я", "~~ивший", "~е~емый", "~~итый";
        gerund: "~я~ь", "~~ив|~~ивши";
    ),
    // Устить: 40 words (чистить, вырастить, очистить, гостить, честить, выхолостить).
    conjugation!("устить", 40, "~стить";
        infinitive: "~стить";
        non_past_first: "~щу", "~стим";
        non_past_second: "~стишь", "~стите";
        non_past_third: "~стит", "~стят";
        past: "~стил", "~стила", "~стило", "~стили";
        imperative: "~сти", "~стите";
        participle: "~стящий", "~стивший", "~стимый", "~щенный";
        gerund: "~стя", "~стив|~стивши";
    ),
    // Разнести: 39 words (изнести, принести, привезти, произнести, донести, перенести).
    conjugation!("разнести", 39, "~е~ти";
        infinitive: "~е~ти";
        non_past_first: "~е~у", "~е~ём";
        non_past_second: "~е~ёшь", "~е~ёте";
        non_past_third: "~е~ёт", "~е~ут";
        past: "~ё~", "~е~ла", "~е~ло", "~е~ли";
        imperative: "~е~и", "~е~ите";
        participle: "~е~ущий", "~ё~ший", "~е~омый", "~е~ённый";
        gerund: "~е~я", "~е~я|~ё~ши";
    ),
    // Ладить: 38 words (гладить, лихорадить, обезвредить, нудить, обезводить, обессудить).
    conjugation!("ладить", 38, "~дить";
        infinitive: "~дить";
        non_past_first: "~жу", "~дим";
        non_past_second: "~дишь", "~дите";
        non_past_third: "~дит", "~дят";
        past: "~дил", "~дила", "~дило", "~дили";
        imperative: "~дь", "~дьте";
        participle: "~дящий", "~дивший", "~димый", "~женный";
        gerund: "~дя", "~див|~дивши";
    ),
    // Ладиться: 38 words (чудиться, сгрудиться, наудиться, нудиться, гладиться, хороводиться).
    conjugation!("ладиться", 38, "~дить~я";
        infinitive: "~дить~я";
        non_past_first: "~жу~ь", "~дим~я";
        non_past_second: "~дишь~я", "~дите~ь";
        non_past_third: "~дит~я", "~дят~я";
        past: "~дил~я", "~дила~ь", "~дило~ь", "~дили~ь";
        imperative: "~дь~я", "~дьте~ь";
        participle: "~дящий~я", "~дивший~я", "~е~емый", "~~итый";
        gerund: "~дя~ь", "~дивши~ь";
    ),
    // Ржаться: 37 words (мчаться, сдержаться, спаться, дышаться, улежаться, лежаться).
    conjugation!("ржаться", 37, "~ать~я";
        infinitive: "~ать~я";
        non_past_first: "~у~ь", "~им~я";
        non_past_second: "~ишь~я", "~ите~ь";
        non_past_third: "~ит~я", "~ат~я";
        past: "~ал~я", "~ала~ь", "~ало~ь", "~али~ь";
        imperative: "~и~ь", "~ите~ь";
        participle: "~ащий~я", "~авший~я", "~е~емый", "~~итый";
        gerund: "~а~ь", "~авши~ь";
    ),
    // Носиться: 36 words (проситься, беситься, уноситься, выброситься, вируситься, вмеситься).
    conjugation!("носиться", 36, "~~иться";
        infinitive: "~~иться";
        non_past_first: "~шу~ь", "~~имся";
        non_past_second: "~~ишься", "~~итесь";
        non_past_third: "~~ится", "~~ятся";
        past: "~~ился", "~~илась", "~~илось", "~~ились";
        imperative: "~~ись", "~~итесь";
        participle: "~~ящийся", "~~ившийся", "~е~емый", "~~итый";
        gerund: "~~ясь", "~~ившись";
    ),
    // Сбиться: 35 words (влиться, сбиться, развиться, взвиться, разбиться, отбиться).
    conjugation!("сбиться", 35, "~~ить~я";
        infinitive: "~~ить~я";
        non_past_first: "~о~ью~ь", "~о~ьём~я";
        non_past_second: "~о~ьёшь~я", "~о~ьёте~ь";
        non_past_third: "~о~ьёт~я", "~о~ьют~я";
        past: "~~ил~я", "~~ила~ь", "~~ило~ь", "~~или~ь";
        imperative: "~~ей~я", "~~ейте~ь";
        participle: "~е~ущий~я", "~~ивший~я", "~~ить~я", "~о~~анный";
        gerund: "~е~я~ь", "~~ивши~ь";
    ),
    // Возбудить: 34 words (руководить, обсудить, вредить, возбудить, осудить, охладить).
    conjugation!("возбудить", 34, "~дить";
        infinitive: "~дить";
        non_past_first: "~жу", "~дим";
        non_past_second: "~дишь", "~дите";
        non_past_third: "~дит", "~дят";
        past: "~дил", "~дила", "~дило", "~дили";
        imperative: "~ди", "~дите";
        participle: "~дящий", "~дивший", "~димый", "~ждённый";
        gerund: "~дя", "~див|~дивши";
    ),
    // Возиться: 34 words (сразиться, возиться, грузиться, гомозиться, ввозиться, разиться).
    conjugation!("возиться", 34, "~зить~я";
        infinitive: "~зить~я";
        non_past_first: "~жу~ь", "~зим~я";
        non_past_second: "~зишь~я", "~зите~ь";
        non_past_third: "~зит~я", "~зят~я";
        past: "~зил~я", "~зила~ь", "~зило~ь", "~зили~ь";
        imperative: "~зи~ь", "~зите~ь";
        participle: "~зящий~я", "~зивший~я", "~е~емый", "~~итый";
        gerund: "~зя~ь", "~зивши~ь";
    ),
    // Кишеть: 34 words (висеть, храпеть, кипеть, зависеть, кишеть, шипеть).
    conjugation!("кишеть", 34, "~еть";
        infinitive: "~еть";
        non_past_first: "~лю", "~им";
        non_past_second: "~ишь", "~ите";
        non_past_third: "~ит", "~ят";
        past: "~ел", "~ела", "~ело", "~ели";
        imperative: "~и", "~ите";
        participle: "~ящий", "~евший", "~имый", "~енный";
        gerund: "~я", "~ев|~евши";
    ),
    // Развести: 33 words (провести, вести, перевести, увести, завести, произвести).
    conjugation!("развести", 33, "~ести";
        infinitive: "~ести";
        non_past_first: "~еду", "~едём";
        non_past_second: "~едёшь", "~едёте";
        non_past_third: "~едёт", "~едут";
        past: "~ёл", "~ела", "~ело", "~ели";
        imperative: "~еди", "~едите";
        participle: "~етущий", "~едший", "~имый", "~едённый";
        gerund: "~етя", "~едя|~едши";
    ),
    // Зти: 33 words (идти, соблюсти, потрясти, сотрясти, ебсти, обтрясти).
    conjugation!("зти", 33, "~ти";
        infinitive: "~ти";
        non_past_first: "~у", "~ём";
        non_past_second: "~ёшь", "~ёте";
        non_past_third: "~ёт", "~ут";
        past: "~", "~ла", "~ло", "~ли";
        imperative: "~и", "~ите";
        participle: "~ущий", "~ший", "~омый", "~ённый";
        gerund: "~я", "~ши";
    ),
    // Инуть: 32 words (кинуть, клюнуть, плюнуть, двинуть, глянуть, сдвинуть).
    conjugation!("инуть", 32, "~уть";
        infinitive: "~уть";
        non_past_first: "~у", "~ем";
        non_past_second: "~ешь", "~ете";
        non_past_third: "~ет", "~ут";
        past: "~ул", "~ула", "~уло", "~ули";
        imperative: "~ь", "~ьте";
        participle: "~ущий", "~увший", "~емый", "~утый";
        gerund: "~я", "~ув|~увши";
    ),
    // Левать: 32 words (плевать, клевать, горевать, чаевать, дневать, калевать).
    conjugation!("левать", 32, "~евать";
        infinitive: "~евать";
        non_past_first: "~юю", "~юем";
        non_past_second: "~юешь", "~юете";
        non_past_third: "~юет", "~юют";
        past: "~евал", "~евала", "~евало", "~евали";
        imperative: "~юй", "~юйте";
        participle: "~юющий", "~евавший", "~юемый", "~ёванный";
        gerund: "~юя", "~евав|~евавши";
    ),
    // Лизиться: 32 words (унизиться, вверзиться, навозиться, капризиться, вморозиться, обеззаразиться).
    conjugation!("лизиться", 32, "~зить~я";
        infinitive: "~зить~я";
        non_past_first: "~жу~ь", "~зим~я";
        non_past_second: "~зишь~я", "~зите~ь";
        non_past_third: "~зит~я", "~зят~я";
        past: "~зил~я", "~зила~ь", "~зило~ь", "~зили~ь";
        imperative: "~зь~я", "~зьте~ь";
        participle: "~зящий~я", "~зивший~я", "~е~емый", "~~итый";
        gerund: "~зя~ь", "~зивши~ь";
    ),
    // Переброситься: 31 words (выситься, наброситься, переброситься, вброситься, левкаситься, купороситься).
    conjugation!("переброситься", 31, "~~иться";
        infinitive: "~~иться";
        non_past_first: "~шу~ь", "~~имся";
        non_past_second: "~~ишься", "~~итесь";
        non_past_third: "~~ится", "~~ятся";
        past: "~~ился", "~~илась", "~~илось", "~~ились";
        imperative: "~~ься", "~~ьтесь";
        participle: "~~ящийся", "~~ившийся", "~е~емый", "~~итый";
        gerund: "~~ясь", "~~ившись";
    ),
    // Каяться: 31 words (каяться, маяться, измаяться, умаяться, замаяться, намаяться).
    conjugation!("каяться", 31, "~ять~я";
        infinitive: "~ять~я";
        non_past_first: "~ю~ь", "~ем~я";
        non_past_second: "~ешь~я", "~ете~ь";
        non_past_third: "~ет~я", "~ют~я";
        past: "~ял~я", "~яла~ь", "~яло~ь", "~яли~ь";
        imperative: "~й~я", "~йте~ь";
        participle: "~ющий~я", "~явший~я", "~е~емый", "~~итый";
        gerund: "~я~ь", "~явши~ь";
    ),
    // Птить: 30 words (золотить, мутить, кипятить, озолотить, франтить, грохотить).
    conjugation!("птить", 30, "~тить";
        infinitive: "~тить";
        non_past_first: "~чу", "~тим";
        non_past_second: "~тишь", "~тите";
        non_past_third: "~тит", "~тят";
        past: "~тил", "~тила", "~тило", "~тили";
        imperative: "~ти", "~тите";
        participle: "~тящий", "~тивший", "~тимый", "~чённый";
        gerund: "~тя", "~тив|~тивши";
    ),
    // Ебать: 28 words (врать, ебать, ржать, заржать, рвать, ждать).
    conjugation!("ебать", 28, "~~ать";
        infinitive: "~~ать";
        non_past_first: "~~у", "~~ём";
        non_past_second: "~~ёшь", "~~ёте";
        non_past_third: "~~ёт", "~~ут";
        past: "~~ал", "~~ала", "~~ало", "~~али";
        imperative: "~~и", "~~ите";
        participle: "~~ущий", "~~авший", "~е~омый", "~~анный";
        gerund: "~~я", "~~ав";
    ),
    // Хтеть: 28 words (лететь, хотеть, улететь, влететь, пыхтеть, ввертеть).
    conjugation!("хтеть", 28, "~теть";
        infinitive: "~теть";
        non_past_first: "~чу", "~тим";
        non_past_second: "~тишь", "~тите";
        non_past_third: "~тит", "~тят";
        past: "~тел", "~тела", "~тело", "~тели";
        imperative: "~ти", "~тите";
        participle: "~тящий", "~тевший", "~тимый", "~ченный";
        gerund: "~тя", "~тев|~тевши";
    ),
    // Сообразить: 26 words (вообразить, разить, сразить, сообразить, погрузить, преобразить).
    conjugation!("сообразить", 26, "~зить";
        infinitive: "~зить";
        non_past_first: "~жу", "~зим";
        non_past_second: "~зишь", "~зите";
        non_past_third: "~зит", "~зят";
        past: "~зил", "~зила", "~зило", "~зили";
        imperative: "~зи", "~зите";
        participle: "~зящий", "~зивший", "~зимый", "~жённый";
        gerund: "~зя", "~зив|~зивши";
    ),
    // Раться: 26 words (ебаться, стебаться, рваться, сраться, уебаться, ужраться).
    conjugation!("раться", 26, "~ать~я";
        infinitive: "~ать~я";
        non_past_first: "~у~ь", "~ём~я";
        non_past_second: "~ёшь~я", "~ёте~ь";
        non_past_third: "~ёт~я", "~ут~я";
        past: "~ал~я", "~ала~ь", "~ало~ь", "~али~ь";
        imperative: "~и~ь", "~ите~ь";
        participle: "~ущий~я", "~авший~я", "~е~емый", "~~итый";
        gerund: "~я~ь", "~авши~ь";
    ),
    // Сдавать: 26 words (узнавать, сдавать, знавать, сувать, создавать, давать).
    conjugation!("сдавать", 26, "~вать";
        infinitive: "~вать";
        non_past_first: "~ю", "~ём";
        non_past_second: "~ёшь", "~ёте";
        non_past_third: "~ёт", "~ют";
        past: "~вал", "~вала", "~вало", "~вали";
        imperative: "~вай", "~вайте";
        participle: "~ющий", "~вавший", "~ваемый", "~ванный";
        gerund: "~вая", "~вав|~вавши";
    ),
    // Атиться: 25 words (возмутиться, ощутиться, воплотиться, посвятиться, тяготиться, позлатиться).
    conjugation!("атиться", 25, "~тить~я";
        infinitive: "~тить~я";
        non_past_first: "~щу~ь", "~тим~я";
        non_past_second: "~тишь~я", "~тите~ь";
        non_past_third: "~тит~я", "~тят~я";
        past: "~тил~я", "~тила~ь", "~тило~ь", "~тили~ь";
        imperative: "~ти~ь", "~тите~ь";
        participle: "~тящий~я", "~тивший~я", "~е~емый", "~~итый";
        gerund: "~тя~ь", "~тивши~ь|~тя~ь";
    ),
    // Щитить: 24 words (посвятить, ощутить, воплотить, возмутить, восхитить, тяготить).
    conjugation!("щитить", 24, "~тить";
        infinitive: "~тить";
        non_past_first: "~щу", "~тим";
        non_past_second: "~тишь", "~тите";
        non_past_third: "~тит", "~тят";
        past: "~тил", "~тила", "~тило", "~тили";
        imperative: "~ти", "~тите";
        participle: "~тящий", "~тивший", "~тимый", "~щённый";
        gerund: "~тя", "~тив|~тивши";
    ),
    // Клепать: 23 words (зевать, оседлать, хлебать, седлать, верстать, сверстать).
    conjugation!("клепать", 23, "~е~ть";
        infinitive: "~е~ть";
        non_past_first: "~е~ю", "~е~ем";
        non_past_second: "~е~ешь", "~е~ете";
        non_past_third: "~е~ет", "~е~ют";
        past: "~е~л", "~е~ла", "~е~ло", "~е~ли";
        imperative: "~е~й", "~е~йте";
        participle: "~е~ющий", "~е~вший", "~е~емый", "~ё~нный";
        gerund: "~е~я", "~е~в|~е~вши";
    ),
    // Вывестись: 23 words (вестись, развестись, произвестись, свестись, довестись, привестись).
    conjugation!("вывестись", 23, "~е~тись";
        infinitive: "~е~тись";
        non_past_first: "~еду~ь", "~едём~я";
        non_past_second: "~едёшь~я", "~едёте~ь";
        non_past_third: "~едёт~я", "~едут~я";
        past: "~ёл~я", "~ела~ь", "~ело~ь", "~ели~ь";
        imperative: "~еди~ь", "~едите~ь";
        participle: "~етущий~я", "~едший~я", "~е~емый", "~~итый";
        gerund: "~етя~ь", "~едя~ь|~едши~ь";
    ),
    // Ястись: 22 words (соблюстись, ебстись, мястись, идтись, ебатись, йтись).
    conjugation!("ястись", 22, "~ти~ь";
        infinitive: "~ти~ь";
        non_past_first: "~у~ь", "~ём~я";
        non_past_second: "~ёшь~я", "~ёте~ь";
        non_past_third: "~ёт~я", "~ут~я";
        past: "~~я", "~ла~ь", "~ло~ь", "~ли~ь";
        imperative: "~и~ь", "~ите~ь";
        participle: "~ущий~я", "~ший~я", "~е~емый", "~~итый";
        gerund: "~я~ь", "~ши~ь";
    ),
    // Крыть: 20 words (мыть, рыть, выть, ныть, уныть, омыть).
    conjugation!("крыть", 20, "~ыть";
        infinitive: "~ыть";
        non_past_first: "~ою", "~оем";
        non_past_second: "~оешь", "~оете";
        non_past_third: "~оет", "~оют";
        past: "~ыл", "~ыла", "~ыло", "~ыли";
        imperative: "~ой", "~ойте";
        participle: "~оющий", "~ывший", "~оемый", "~ытый";
        gerund: "~оя", "~ыв|~ывши";
    ),
    // Гореться: 20 words (зреться, узреться, улицезреться, озреться, уреветься, смотреться).
    conjugation!("гореться", 20, "~еть~я";
        infinitive: "~еть~я";
        non_past_first: "~ю~ь", "~им~я";
        non_past_second: "~ишь~я", "~ите~ь";
        non_past_third: "~ит~я", "~ят~я";
        past: "~ел~я", "~ела~ь", "~ело~ь", "~ели~ь";
        imperative: "~и~ь", "~ите~ь";
        participle: "~ящий~я", "~евший~я", "~е~емый", "~~итый";
        gerund: "~я~ь", "~евши~ь";
    ),
    // Шить: 19 words (пить, бить, лить, вить, шить, увить).
    conjugation!("шить", 19, "~ить";
        infinitive: "~ить";
        non_past_first: "~ью", "~ьём";
        non_past_second: "~ьёшь", "~ьёте";
        non_past_third: "~ьёт", "~ьют";
        past: "~ил", "~ила", "~ило", "~или";
        imperative: "~ей", "~ейте";
        participle: "~ьющий", "~ивший", "~имый", "~итый";
        gerund: "~ья", "~ив|~ивши";
    ),
    // Виснуть: 19 words (гибнуть, дрыхнуть, гаснуть, сякнуть, виснуть, вязнуть).
    conjugation!("виснуть", 19, "~нуть";
        infinitive: "~нуть";
        non_past_first: "~ну", "~нем";
        non_past_second: "~нешь", "~нете";
        non_past_third: "~нет", "~нут";
        past: "~|~нул", "~ла", "~ло", "~ли";
        imperative: "~ни", "~ните";
        participle: "~нущий", "~нувший", "~емый", "~нутый";
        gerund: "~я", "~нув|~нувши";
    ),
    // Поехать: 18 words (поехать, ехать, приехать, уехать, въехать, проехать).
    conjugation!("поехать", 18, "~хать";
        infinitive: "~хать";
        non_past_first: "~ду", "~дем";
        non_past_second: "~дешь", "~дете";
        non_past_third: "~дет", "~дут";
        past: "~хал", "~хала", "~хало", "~хали";
        imperative: "~зжай", "~зжайте";
        participle: "~шущий", "~хавший", "~домый", "~ханный";
        gerund: "~ша", "~хав|~хавши";
    ),
    // Виться: 18 words (литься, биться, увиться, виться, вылиться, ушиться).
    conjugation!("виться", 18, "~ить~я";
        infinitive: "~ить~я";
        non_past_first: "~ью~ь", "~ьём~я";
        non_past_second: "~ьёшь~я", "~ьёте~ь";
        non_past_third: "~ьёт~я", "~ьют~я";
        past: "~ил~я", "~ила~ь", "~ило~ь", "~или~ь";
        imperative: "~ей~я", "~ейте~ь";
        participle: "~ьющий~я", "~ивший~я", "~е~емый", "~~итый";
        gerund: "~ья~ь", "~ивши~ь";
    ),
    // Лести: 18 words (подмести, взгнести, намести, нагнести, вмести, взмести).
    conjugation!("лести", 18, "~ести";
        infinitive: "~ести";
        non_past_first: "~ету", "~етём";
        non_past_second: "~етёшь", "~етёте";
        non_past_third: "~етёт", "~етут";
        past: "~ёл", "~ела", "~ело", "~ели";
        imperative: "~ети", "~етите";
        participle: "~етущий", "~ётший", "~имый", "~етённый";
        gerund: "~етя", "~етя|~ётши";
    ),
    // Сидеть: 18 words (бздеть, сидеть, глядеть, гудеть, бдеть, зудеть).
    conjugation!("сидеть", 18, "~деть";
        infinitive: "~деть";
        non_past_first: "~жу", "~дим";
        non_past_second: "~дишь", "~дите";
        non_past_third: "~дит", "~дят";
        past: "~дел", "~дела", "~дело", "~дели";
        imperative: "~ди", "~дите";
        participle: "~дящий", "~девший", "~димый", "~жённый";
        gerund: "~дя", "~дев|~девши";
    ),
    // Насесть: 17 words (сесть, засесть, присесть, осесть, подсесть, пересесть).
    conjugation!("насесть", 17, "~есть";
        infinitive: "~есть";
        non_past_first: "~яду", "~ядем";
        non_past_second: "~ядешь", "~ядете";
        non_past_third: "~ядет", "~ядут";
        past: "~ел", "~ела", "~ело", "~ели";
        imperative: "~ядь", "~ядьте";
        participle: "~ящий", "~евший", "~имый", "~тённый";
        gerund: "~я", "~ев";
    ),
    // Ласить: 17 words (гласить, оросить, бесить, вопросить, искусить, разбесить).
    conjugation!("ласить", 17, "~сить";
        infinitive: "~сить";
        non_past_first: "~шу", "~сим";
        non_past_second: "~сишь", "~сите";
        non_past_third: "~сит", "~сят";
        past: "~сил", "~сила", "~сило", "~сили";
        imperative: "~си", "~сите";
        participle: "~сящий", "~сивший", "~симый", "~шённый";
        gerund: "~ся", "~сив|~сивши";
    ),
    // Сдвинуться: 17 words (двинуться, кинуться, ринуться, сдвинуться, проклюнуться, вдвинуться).
    conjugation!("сдвинуться", 17, "~уть~я";
        infinitive: "~уть~я";
        non_past_first: "~у~ь", "~ем~я";
        non_past_second: "~ешь~я", "~ете~ь";
        non_past_third: "~ет~я", "~ут~я";
        past: "~ул~я", "~ула~ь", "~уло~ь", "~ули~ь";
        imperative: "~ь~я", "~ьте~ь";
        participle: "~ущий~я", "~увший~я", "~е~емый", "~~итый";
        gerund: "~ая~ь", "~увши~ь";
    ),
    // Унестись: 17 words (произнестись, донестись, отнестись, пронестись, понестись, перенестись).
    conjugation!("унестись", 17, "~е~ти~ь";
        infinitive: "~е~ти~ь";
        non_past_first: "~е~у~ь", "~е~ём~я";
        non_past_second: "~е~ёшь~я", "~е~ёте~ь";
        non_past_third: "~е~ёт~я", "~е~ут~я";
        past: "~ё~~я", "~е~ла~ь", "~е~ло~ь", "~е~ли~ь";
        imperative: "~е~и~ь", "~е~ите~ь";
        participle: "~е~ущий~я", "~ё~ший~я", "~е~ти~ь", "~о~~анный";
        gerund: "~е~я~ь", "~е~я~ь|~ё~ши~ь";
    ),
    // Гореть: 17 words (гореть, обозреть, призреть, воззреть, прозреть, лицезреть).
    conjugation!("гореть", 17, "~еть";
        infinitive: "~еть";
        non_past_first: "~ю", "~им";
        non_past_second: "~ишь", "~ите";
        non_past_third: "~ит", "~ят";
        past: "~ел", "~ела", "~ело", "~ели";
        imperative: "~и", "~ите";
        participle: "~ящий", "~евший", "~имый", "~енный";
        gerund: "~я", "~ев|~евши";
    ),
    // Оргнуться: 17 words (назябнуться, ввергнуться, воздвигнуться, оргнуться, вергнуться, никнуться).
    conjugation!("оргнуться", 17, "~нуть~я";
        infinitive: "~нуть~я";
        non_past_first: "~ну~ь", "~нем~я";
        non_past_second: "~нешь~я", "~нете~ь";
        non_past_third: "~нет~я", "~нут~я";
        past: "~~я", "~ла~ь", "~ло~ь", "~ли~ь";
        imperative: "~ни~ь", "~ните~ь";
        participle: "~нущий~я", "~ший~я", "~е~емый", "~~итый";
        gerund: "~я~ь", "~ши~ь";
    ),
    // Пасть: 17 words (класть, клясть, обокрасть, искласть, пасть, асть).
    conjugation!("пасть", 17, "~сть";
        infinitive: "~сть";
        non_past_first: "~ду", "~дём";
        non_past_second: "~дёшь", "~дёте";
        non_past_third: "~дёт", "~дут";
        past: "~л", "~ла", "~ло", "~ли";
        imperative: "~ди", "~дите";
        participle: "~дущий", "~вший", "~домый", "~денный";
        gerund: "~дя", "~в|~вши";
    ),
    // Рыться: 16 words (мыться, срыться, рыться, врыться, домыться, омыться).
    conjugation!("рыться", 16, "~ыть~я";
        infinitive: "~ыть~я";
        non_past_first: "~ою~ь", "~оем~я";
        non_past_second: "~оешь~я", "~оете~ь";
        non_past_third: "~оет~я", "~оют~я";
        past: "~ыл~я", "~ыла~ь", "~ыло~ь", "~ыли~ь";
        imperative: "~ой~я", "~ойте~ь";
        participle: "~оющий~я", "~ывший~я", "~е~емый", "~~итый";
        gerund: "~оя~ь", "~ывши~ь";
    ),
    // Плакать: 16 words (плакать, тыкать, оплакать, кликать, заплакать, мурлыкать).
    conjugation!("плакать", 16, "~кать";
        infinitive: "~кать";
        non_past_first: "~чу", "~чем";
        non_past_second: "~чешь", "~чете";
        non_past_third: "~чет", "~чут";
        past: "~кал", "~кала", "~кало", "~кали";
        imperative: "~чь", "~чьте";
        participle: "~чущий", "~кавший", "~тимый", "~канный";
        gerund: "~ча", "~кав|~кавши";
    ),
    // Вырвать: 16 words (жаждать, выебать, вырвать, выждать, глаголать, выжрать).
    conjugation!("вырвать", 16, "~ать";
        infinitive: "~ать";
        non_past_first: "~у", "~ем";
        non_past_second: "~ешь", "~ете";
        non_past_third: "~ет", "~ут";
        past: "~ал", "~ала", "~ало", "~али";
        imperative: "~и", "~ите";
        participle: "~ущий", "~авший", "~имый", "~анный";
        gerund: "~ая", "~ав|~авши";
    ),
    // Даваться: 16 words (сдаваться, вдаваться, узнаваться, знаваться, даваться, аваться).
    conjugation!("даваться", 16, "~вать~я";
        infinitive: "~вать~я";
        non_past_first: "~ю~ь", "~ём~я";
        non_past_second: "~ёшь~я", "~ёте~ь";
        non_past_third: "~ёт~я", "~ют~я";
        past: "~вал~я", "~вала~ь", "~вало~ь", "~вали~ь";
        imperative: "~вай~я", "~вайте~ь";
        participle: "~ющий~я", "~вавший~я", "~е~емый", "~~итый";
        gerund: "~вая~ь", "~вавши~ь";
    ),
    // Вернуть: 16 words (свернуть, отстегнуть, хлестнуть, черкнуть, хлебнуть, обернуть).
    conjugation!("вернуть", 16, "~е~уть";
        infinitive: "~е~уть";
        non_past_first: "~е~у", "~е~ём";
        non_past_second: "~е~ёшь", "~е~ёте";
        non_past_third: "~е~ёт", "~е~ут";
        past: "~е~ул", "~е~ула", "~е~уло", "~е~ули";
        imperative: "~е~и", "~е~ите";
        participle: "~е~ущий", "~е~увший", "~е~омый", "~ё~утый";
        gerund: "~е~я", "~е~ув|~е~увши";
    ),
    // Глядеться: 16 words (увидеться, вглядеться, сидеться, глядеться, пердеться, зудеться).
    conjugation!("глядеться", 16, "~деть~я";
        infinitive: "~деть~я";
        non_past_first: "~жу~ь", "~дим~я";
        non_past_second: "~дишь~я", "~дите~ь";
        non_past_third: "~дит~я", "~дят~я";
        past: "~дел~я", "~дела~ь", "~дело~ь", "~дели~ь";
        imperative: "~ди~ь", "~дите~ь";
        participle: "~дящий~я", "~девший~я", "~е~емый", "~~итый";
        gerund: "~дя~ь", "~девши~ь";
    ),
    // Азаться: 15 words (казаться, вязаться, ввязаться, увязаться, лизаться, нанизаться).
    conjugation!("азаться", 15, "~зать~я";
        infinitive: "~зать~я";
        non_past_first: "~жу~ь", "~жем~я";
        non_past_second: "~жешь~я", "~жете~ь";
        non_past_third: "~жет~я", "~жут~я";
        past: "~зал~я", "~зала~ь", "~зало~ь", "~зали~ь";
        imperative: "~жи~ь", "~жите~ь";
        participle: "~жущий~я", "~завший~я", "~е~емый", "~~итый";
        gerund: "~жа~ь", "~завши~ь";
    ),
    // Жевать: 15 words (бичевать, жевать, межевать, тушевать, линчевать, свежевать).
    conjugation!("жевать", 15, "~евать";
        infinitive: "~евать";
        non_past_first: "~ую", "~уем";
        non_past_second: "~уешь", "~уете";
        non_past_third: "~ует", "~уют";
        past: "~евал", "~евала", "~евало", "~евали";
        imperative: "~уй", "~уйте";
        participle: "~ующий", "~евавший", "~уемый", "~ёванный";
        gerund: "~уя", "~евав|~евавши";
    ),
    // Сыпаться: 15 words (сыпаться, истрепаться, затрепаться, щипаться, дрематься, отрепаться).
    conjugation!("сыпаться", 15, "~ать~я";
        infinitive: "~ать~я";
        non_past_first: "~лю~ь", "~лем~я";
        non_past_second: "~лешь~я", "~лете~ь";
        non_past_third: "~лет~я", "~лют~я";
        past: "~ал~я", "~ала~ь", "~ало~ь", "~али~ь";
        imperative: "~ли~ь", "~лите~ь";
        participle: "~лющий~я", "~авший~я", "~е~емый", "~~итый";
        gerund: "~ля~ь", "~авши~ь";
    ),
    // Разобрать: 14 words (вобрать, разобрать, подобрать, отобрать, ободрать, содрать).
    conjugation!("разобрать", 14, "~о~~ать";
        infinitive: "~о~~ать";
        non_past_first: "~~е~у", "~~е~ём";
        non_past_second: "~~е~ёшь", "~~е~ёте";
        non_past_third: "~~е~ёт", "~~е~ут";
        past: "~о~~ал", "~о~~ала", "~о~~ало", "~о~~али";
        imperative: "~~е~и", "~~е~ите";
        participle: "~е~ущий~я", "~о~~авший", "~о~~ать", "~о~~анный";
        gerund: "~е~я~ь", "~о~~ав|~о~~авши";
    ),
    // Пасться: 14 words (клясться, класться, есться, уесться, испрясться, накласться).
    conjugation!("пасться", 14, "~~ться";
        infinitive: "~~ться";
        non_past_first: "~ду~ь", "~дём~я";
        non_past_second: "~дёшь~я", "~дёте~ь";
        non_past_third: "~дёт~я", "~дут~я";
        past: "~л~я", "~ла~ь", "~ло~ь", "~ли~ь";
        imperative: "~ди~ь", "~дите~ь";
        participle: "~дущий~я", "~вший~я", "~е~емый", "~~итый";
        gerund: "~дя~ь", "~вши~ь";
    ),
    // Птаться: 14 words (ухохотаться, щекотаться, всклокотаться, птаться, хохотаться, етаться).
    conjugation!("птаться", 14, "~тать~я";
        infinitive: "~тать~я";
        non_past_first: "~чу~ь", "~чем~я";
        non_past_second: "~чешь~я", "~чете~ь";
        non_past_third: "~чет~я", "~чут~я";
        past: "~тал~я", "~тала~ь", "~тало~ь", "~тали~ь";
        imperative: "~чи~ь", "~чите~ь";
        participle: "~чущий~я", "~тавший~я", "~е~емый", "~~итый";
        gerund: "~ча~ь", "~тавши~ь";
    ),
    // Язать: 14 words (вырезать, увязать, ввязать, унизать, улизать, азать).
    conjugation!("язать", 14, "~зать";
        infinitive: "~зать";
        non_past_first: "~жу", "~жем";
        non_past_second: "~жешь", "~жете";
        non_past_third: "~жет", "~жут";
        past: "~зал", "~зала", "~зало", "~зали";
        imperative: "~жи", "~жите";
        participle: "~зающий", "~завший", "~заемый", "~занный";
        gerund: "~зая", "~зав|~завши";
    ),
    // Сдать: 13 words (дать, сдать, раздать, сдодать, создать, ить).
    conjugation!("сдать", 13, "~ть";
        infinitive: "~ть";
        non_past_first: "~м", "~дим";
        non_past_second: "~шь", "~дите";
        non_past_third: "~ст", "~дут";
        past: "~л", "~ла", "~ло|~ло", "~ли";
        imperative: "~й", "~йте";
        participle: "~ющий", "~вший", "~емый", "~нный";
        gerund: "~я", "~в|~вши";
    ),
    // Пахать: 13 words (отмахать, запахать, распахать, вспахать, выпахать, допахать).
    conjugation!("пахать", 13, "~хать";
        infinitive: "~хать";
        non_past_first: "~шу", "~шем";
        non_past_second: "~шешь", "~шете";
        non_past_third: "~шет", "~шут";
        past: "~хал", "~хала", "~хало", "~хали";
        imperative: "~ши", "~шите";
        participle: "~шущий", "~хавший", "~симый", "~ханный";
        gerund: "~ша", "~хав|~хавши";
    ),
    // Попереть: 13 words (потереть, простереть, утереть, попереть, опереть, перетереть).
    conjugation!("попереть", 13, "~е~еть";
        infinitive: "~е~еть";
        non_past_first: "~~у", "~~ём";
        non_past_second: "~~ёшь", "~~ёте";
        non_past_third: "~~ёт", "~~ут";
        past: "~ё~", "~ё~ла", "~ё~ло", "~ё~ли";
        imperative: "~~и", "~~ите";
        participle: "~~ущий", "~ё~ший", "~е~омый", "~ё~тый";
        gerund: "~~я", "~е~ев|~ё~ши";
    ),
    // Лыхаться: 13 words (напахаться, выпахаться, брехаться, допахаться, запахаться, припахаться).
    conjugation!("лыхаться", 13, "~хать~я";
        infinitive: "~хать~я";
        non_past_first: "~шу~ь", "~шем~я";
        non_past_second: "~шешь~я", "~шете~ь";
        non_past_third: "~шет~я", "~шут~я";
        past: "~хал~я", "~хала~ь", "~хало~ь", "~хали~ь";
        imperative: "~ши~ь", "~шите~ь";
        participle: "~шущий~я", "~хавший~я", "~е~емый", "~~итый";
        gerund: "~ша~ь", "~хавши~ь";
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
    // Тыть: 13 words (настрять, рять, стать, вять, тыть, встать).
    conjugation!("тыть", 13, "~ть";
        infinitive: "~ть";
        non_past_first: "~ну", "~нем";
        non_past_second: "~нешь", "~нете";
        non_past_third: "~нет", "~нут";
        past: "~л", "~ла", "~ло", "~ли";
        imperative: "~нь", "~ньте";
        participle: "~ющий", "~вший", "~емый", "~нный";
        gerund: "~я", "~в|~вши";
    ),
    // Рыскать: 13 words (искать, плескать, сыскать, рыскать, наплескать, оскать).
    conjugation!("рыскать", 13, "~скать";
        infinitive: "~скать";
        non_past_first: "~щу", "~щем";
        non_past_second: "~щешь", "~щете";
        non_past_third: "~щет", "~щут";
        past: "~скал", "~скала", "~скало", "~скали";
        imperative: "~щи", "~щите";
        participle: "~щущий", "~скавший", "~скаемый", "~сканный";
        gerund: "~ща", "~скав|~скавши";
    ),
    // Напеть: 12 words (петь, допеть, запеть, напеть, распеть, перепеть).
    conjugation!("напеть", 12, "~еть";
        infinitive: "~еть";
        non_past_first: "~ою", "~оём";
        non_past_second: "~оёшь", "~оёте";
        non_past_third: "~оёт", "~оют";
        past: "~ел", "~ела", "~ело", "~ели";
        imperative: "~ой", "~ойте";
        participle: "~оющий", "~евший", "~имый", "~етый";
        gerund: "~оя", "~ев|~евши";
    ),
    // Прискакать: 12 words (скакать, отскакать, заскакать, наскакать, прискакать, вскакать).
    conjugation!("прискакать", 12, "~кать";
        infinitive: "~кать";
        non_past_first: "~чу", "~чем";
        non_past_second: "~чешь", "~чете";
        non_past_third: "~чет", "~чут";
        past: "~кал", "~кала", "~кало", "~кали";
        imperative: "~чи", "~чите";
        participle: "~чущий", "~кавший", "~тимый", "~канный";
        gerund: "~ча", "~кав|~кавши";
    ),
    // Ечь: 12 words (вытечь, приволочь, выволочь, проистечь, вволочь, уволочь).
    conjugation!("ечь", 12, "~чь";
        infinitive: "~чь";
        non_past_first: "~ку", "~чём";
        non_past_second: "~чёшь", "~чёте";
        non_past_third: "~чёт", "~кут";
        past: "~к", "~кла", "~кло", "~кли";
        imperative: "~ки", "~ките";
        participle: "~гущий", "~кший", "~тимый", "~чённый";
        gerund: "~тя", "~кши";
    ),
    // Драть: 12 words (брать, драть, прать, обрать, наддрать, избрать).
    conjugation!("драть", 12, "~~ать";
        infinitive: "~~ать";
        non_past_first: "~е~у", "~е~ём";
        non_past_second: "~е~ёшь", "~е~ёте";
        non_past_third: "~е~ёт", "~е~ут";
        past: "~~ал", "~~ала", "~~ало", "~~али";
        imperative: "~е~и", "~е~ите";
        participle: "~е~ущий", "~~авший", "~е~омый", "~~анный";
        gerund: "~е~я", "~~ав|~~авши";
    ),
    // Воеваться: 12 words (циклеваться, шпаклеваться, шпатлеваться, штабелеваться, штемпелеваться, калеваться).
    conjugation!("воеваться", 12, "~евать~я";
        infinitive: "~евать~я";
        non_past_first: "~юю~ь", "~юем~я";
        non_past_second: "~юешь~я", "~юете~ь";
        non_past_third: "~юет~я", "~юют~я";
        past: "~евал~я", "~евала~ь", "~евало~ь", "~евали~ь";
        imperative: "~юй~я", "~юйте~ь";
        participle: "~юющий~я", "~евавший~я", "~е~емый", "~~итый";
        gerund: "~юя~ь", "~евавши~ь";
    ),
    // Истеть: 12 words (хрустеть, естеть, истеть, лестеть, вистеть, свистеть).
    conjugation!("истеть", 12, "~стеть";
        infinitive: "~стеть";
        non_past_first: "~щу", "~стим";
        non_past_second: "~стишь", "~стите";
        non_past_third: "~стит", "~стят";
        past: "~стел", "~стела", "~стело", "~стели";
        imperative: "~сти", "~стите";
        participle: "~стящий", "~стевший", "~стимый", "~щённый";
        gerund: "~стя", "~стев|~стевши";
    ),
    // Браться: 11 words (драться, браться, выдраться, повыбраться, обраться, избраться).
    conjugation!("браться", 11, "~~ать~я";
        infinitive: "~~ать~я";
        non_past_first: "~е~у~ь", "~е~ём~я";
        non_past_second: "~е~ёшь~я", "~е~ёте~ь";
        non_past_third: "~е~ёт~я", "~е~ут~я";
        past: "~~ал~я|~~ал~я", "~~ала~ь", "~~ало~ь|~~ало~ь", "~~али~ь|~~али~ь";
        imperative: "~е~и~ь", "~е~ите~ь";
        participle: "~е~ущий~я", "~~авший~я", "~~ать~я", "~о~~анный";
        gerund: "~е~я~ь", "~~авши~ь";
    ),
    // Зачесться: 11 words (учесться, расчесться, зачесться, почесться, обчесться, причесться).
    conjugation!("зачесться", 11, "~е~ться";
        infinitive: "~е~ться";
        non_past_first: "~ту~ь", "~тём~я";
        non_past_second: "~тёшь~я", "~тёте~ь";
        non_past_third: "~тёт~я", "~тут~я";
        past: "~ёл~я", "~ла~ь", "~ло~ь", "~ли~ь";
        imperative: "~ти~ь", "~тите~ь";
        participle: "~ющий~я", "~евший~я", "~е~емый", "~~итый";
        gerund: "~я~ь", "~тя~ь";
    ),
    // Рщить: 11 words (выпучить, плющить, орядочить, рщить, орщить, ращить).
    conjugation!("рщить", 11, "~ить";
        infinitive: "~ить";
        non_past_first: "~у", "~им";
        non_past_second: "~ишь", "~ите";
        non_past_third: "~ит", "~ат";
        past: "~ил", "~ила", "~ило", "~или";
        imperative: "~и|~ь", "~ьте";
        participle: "~ащий", "~ивший", "~имый", "~енный";
        gerund: "~а", "~ив|~ивши";
    ),
    // Честь: 11 words (прочесть, честь, вычесть, учесть, почесть, перечесть).
    conjugation!("честь", 11, "~есть";
        infinitive: "~есть";
        non_past_first: "~ту", "~тём";
        non_past_second: "~тёшь", "~тёте";
        non_past_third: "~тёт", "~тут";
        past: "~ёл", "~ла", "~ло", "~ли";
        imperative: "~ти", "~тите";
        participle: "~ющий", "~ётший", "~емый", "~тённый";
        gerund: "~я", "~тя|~ётши";
    ),
    // Вянуть: 11 words (вянуть, увянуть, обвянуть, повянуть, привянуть, призавянуть).
    conjugation!("вянуть", 11, "~нуть";
        infinitive: "~нуть";
        non_past_first: "~ну", "~нем";
        non_past_second: "~нешь", "~нете";
        non_past_third: "~нет", "~нут";
        past: "~л", "~ла", "~ло", "~ли";
        imperative: "~нь", "~ньте";
        participle: "~нущий", "~нувший", "~емый", "~нутый";
        gerund: "~я", "~нувши";
    ),
    // Рпеться: 11 words (шипеться, сипеться, рпеться, ерпеться, терпеться, кипеться).
    conjugation!("рпеться", 11, "~еть~я";
        infinitive: "~еть~я";
        non_past_first: "~лю~ь", "~им~я";
        non_past_second: "~ишь~я", "~ите~ь";
        non_past_third: "~ит~я", "~ят~я";
        past: "~ел~я", "~ела~ь", "~ело~ь", "~ели~ь";
        imperative: "~и~ь", "~ите~ь";
        participle: "~ящий~я", "~евший~я", "~е~емый", "~~итый";
        gerund: "~я~ь", "~евши~ь";
    ),
    // Писать: 11 words (плясать, вытесать, вычесать, исать, ясать, писать).
    conjugation!("писать", 11, "~сать";
        infinitive: "~сать";
        non_past_first: "~шу", "~шем";
        non_past_second: "~шешь", "~шете";
        non_past_third: "~шет", "~шут";
        past: "~сал", "~сала", "~сало", "~сали";
        imperative: "~ши", "~шите";
        participle: "~сающий", "~савший", "~симый", "~санный";
        gerund: "~сая", "~сав|~савши";
    ),
    // Ичься: 11 words (поднапрячься, перенапрячься, рячься, тичься, ичься, тричься).
    conjugation!("ичься", 11, "~чь~я";
        infinitive: "~чь~я";
        non_past_first: "~гу~ь", "~жём~я";
        non_past_second: "~жёшь~я", "~жёте~ь";
        non_past_third: "~жёт~я", "~гут~я";
        past: "~г~я", "~гла~ь", "~гло~ь", "~гли~ь";
        imperative: "~ги~ь", "~гите~ь";
        participle: "~гущий~я", "~гший~я", "~е~емый", "~~итый";
        gerund: "~жая~ь", "~гши~ь";
    ),
    // Вынести: 10 words (вынести, вывезти, вытрясти, повывезти, повынести, повыползти).
    conjugation!("вынести", 10, "~ти";
        infinitive: "~ти";
        non_past_first: "~у", "~ем";
        non_past_second: "~ешь", "~ете";
        non_past_third: "~ет", "~ут";
        past: "~", "~ла", "~ло", "~ли";
        imperative: "~и", "~ите";
        participle: "~ущий", "~ший", "~омый", "~енный";
        gerund: "~я", "~я";
    ),
    // Подобраться: 10 words (разобраться, подобраться, разодраться, вобраться, отобраться, изодраться).
    conjugation!("подобраться", 10, "~о~~ать~я";
        infinitive: "~о~~ать~я";
        non_past_first: "~~е~у~ь", "~~е~ём~я";
        non_past_second: "~~е~ёшь~я", "~~е~ёте~ь";
        non_past_third: "~~е~ёт~я", "~~е~ут~я";
        past: "~о~~ал~я", "~о~~ала~ь", "~о~~ало~ь|~о~~ало~ь", "~о~~али~ь|~о~~али~ь";
        imperative: "~~е~и~ь", "~~е~ите~ь";
        participle: "~о~~ать~я", "~о~~авший~я", "~о~~ать~я", "~о~~ать~я";
        gerund: "~о~~ать~я", "~о~~авши~ь";
    ),
    // Помять: 10 words (мять, помять, распять, замять, домять, вымять).
    conjugation!("помять", 10, "~ять";
        infinitive: "~ять";
        non_past_first: "~ну", "~нём";
        non_past_second: "~нёшь", "~нёте";
        non_past_third: "~нёт", "~нут";
        past: "~ял", "~яла", "~яло", "~яли";
        imperative: "~ни", "~ните";
        participle: "~нущий", "~явший", "~емый", "~ятый";
        gerund: "~ня", "~яв|~явши";
    ),
    // Влечься: 10 words (увлечься, влечься, печься, посечься, сечься, пресечься).
    conjugation!("влечься", 10, "~ечь~я";
        infinitive: "~ечь~я";
        non_past_first: "~еку~ь", "~ечём~я";
        non_past_second: "~ечёшь~я", "~ечёте~ь";
        non_past_third: "~ечёт~я", "~екут~я";
        past: "~ёк~я", "~екла~ь", "~екло~ь", "~екли~ь";
        imperative: "~еки~ь", "~еките~ь";
        participle: "~екущий~я", "~ёкший~я", "~е~емый", "~~итый";
        gerund: "~я~ь", "~ёкши~ь";
    ),
    // Лыться: 10 words (житься, ужиться, сжиться, вжиться, плыться, лыться).
    conjugation!("лыться", 10, "~ть~я";
        infinitive: "~ть~я";
        non_past_first: "~ву~ь", "~вём~я";
        non_past_second: "~вёшь~я", "~вёте~ь";
        non_past_third: "~вёт~я", "~вут~я";
        past: "~л~я|~л~я", "~ла~ь", "~ло~ь|~ло~ь", "~ли~ь";
        imperative: "~ви~ь", "~вите~ь";
        participle: "~ющий~я", "~вший~я", "~е~емый", "~~итый";
        gerund: "~я~ь", "~вши~ь";
    ),
    // Спеться: 10 words (петься, спеться, допеться, распеться, напеться, припеться).
    conjugation!("спеться", 10, "~еть~я";
        infinitive: "~еть~я";
        non_past_first: "~ою~ь", "~оём~я";
        non_past_second: "~оёшь~я", "~оёте~ь";
        non_past_third: "~оёт~я", "~оют~я";
        past: "~ел~я", "~ела~ь", "~ело~ь", "~ели~ь";
        imperative: "~ой~я", "~ойте~ь";
        participle: "~оющий~я", "~евший~я", "~е~емый", "~~итый";
        gerund: "~оя~ь", "~евши~ь";
    ),
    // Опереться: 10 words (опереться, стереться, отпереться, подтереться, втереться, впереться).
    conjugation!("опереться", 10, "~~е~еть~я";
        infinitive: "~~е~еть~я";
        non_past_first: "~о~~у~ь", "~о~~ём~я";
        non_past_second: "~о~~ёшь~я", "~о~~ёте~ь";
        non_past_third: "~о~~ёт~я", "~о~~ут~я";
        past: "~~ё~~я", "~~ё~ла~ь", "~~ё~ло~ь", "~~ё~ли~ь";
        imperative: "~о~~и~ь", "~о~~ите~ь";
        participle: "~~е~еть~я", "~~ё~ший~я", "~~е~еть~я", "~~е~еть~я";
        gerund: "~~е~еть~я", "~~ё~ши~ь";
    ),
    // Лететься: 10 words (хотеться, пыхтеться, ввертеться, вывертеться, ртеться, лететься).
    conjugation!("лететься", 10, "~теть~я";
        infinitive: "~теть~я";
        non_past_first: "~чу~ь", "~тим~я";
        non_past_second: "~тишь~я", "~тите~ь";
        non_past_third: "~тит~я", "~тят~я";
        past: "~тел~я", "~тела~ь", "~тело~ь", "~тели~ь";
        imperative: "~ти~ь", "~тите~ь";
        participle: "~тящий~я", "~тевший~я", "~е~емый", "~~итый";
        gerund: "~тя~ь", "~тевши~ь";
    ),
    // Тчевать: 10 words (выкорчевать, оттушевать, перетанцевать, потушевать, развальцевать, растушевать).
    conjugation!("тчевать", 10, "~евать";
        infinitive: "~евать";
        non_past_first: "~ую", "~уем";
        non_past_second: "~уешь", "~уете";
        non_past_third: "~ует", "~уют";
        past: "~евал", "~евала", "~евало", "~евали";
        imperative: "~уй", "~уйте";
        participle: "~ующий", "~евавший", "~уемый", "~еванный";
        gerund: "~уя", "~евав|~евавши";
    ),
    // Видеть: 9 words (видеть, увидеть, обидеть, ясновидеть, навидеть, ненавидеть).
    conjugation!("видеть", 9, "~деть";
        infinitive: "~деть";
        non_past_first: "~жу", "~дим";
        non_past_second: "~дишь", "~дите";
        non_past_third: "~дит", "~дят";
        past: "~дел", "~дела", "~дело", "~дели";
        imperative: "~дь", "~дьте";
        participle: "~дящий", "~девший", "~димый", "~денный";
        gerund: "~дя", "~дев|~девши";
    ),
    // Тичь: 9 words (мочь, вылечь, тичь, очь, ричь, стичь).
    conjugation!("тичь", 9, "~чь";
        infinitive: "~чь";
        non_past_first: "~гу", "~жем";
        non_past_second: "~жешь", "~жете";
        non_past_third: "~жет", "~гут";
        past: "~г", "~гла", "~гло", "~гли";
        imperative: "~ги", "~гите";
        participle: "~гущий", "~гший", "~заемый", "~женный";
        gerund: "~зая", "~гши";
    ),
    // Петать: 9 words (клеветать, петать, ежетать, репетать, леветать, роптать).
    conjugation!("петать", 9, "~тать";
        infinitive: "~тать";
        non_past_first: "~щу", "~щем";
        non_past_second: "~щешь", "~щете";
        non_past_third: "~щет", "~щут";
        past: "~тал", "~тала", "~тало", "~тали";
        imperative: "~щи", "~щите";
        participle: "~щущий", "~тавший", "~скаемый", "~танный";
        gerund: "~ща", "~тав|~тавши";
    ),
    // Обрести: 9 words (изобрести, выцвести, доцвести, промести, процвести, примести).
    conjugation!("обрести", 9, "~ести";
        infinitive: "~ести";
        non_past_first: "~ету", "~етём";
        non_past_second: "~етёшь", "~етёте";
        non_past_third: "~етёт", "~етут";
        past: "~ёл", "~ела", "~ело", "~ели";
        imperative: "~ети", "~етите";
        participle: "~етущий", "~етший", "~имый", "~етённый";
        gerund: "~етя", "~етя|~етши";
    ),
    // Сечь: 9 words (печь, течь, сечь, увлечь, влечь, надсечь).
    conjugation!("сечь", 9, "~ечь";
        infinitive: "~ечь";
        non_past_first: "~еку", "~ечём";
        non_past_second: "~ечёшь", "~ечёте";
        non_past_third: "~ечёт", "~екут";
        past: "~ёк", "~екла", "~екло", "~екли";
        imperative: "~еки", "~еките";
        participle: "~екущий", "~ёкший", "~екомый", "~ечённый";
        gerund: "~я", "~ёкши";
    ),
    // Ыпать: 9 words (дремать, сыпать, щипать, щепать, вытрепать, ущипать).
    conjugation!("ыпать", 9, "~ать";
        infinitive: "~ать";
        non_past_first: "~лю", "~лем";
        non_past_second: "~лешь", "~лете";
        non_past_third: "~лет", "~лют";
        past: "~ал", "~ала", "~ало", "~али";
        imperative: "~ли", "~лите";
        participle: "~лющий", "~авший", "~аемый", "~анный";
        gerund: "~ля", "~ав|~авши";
    ),
    // Чесаться: 9 words (чесаться, плясаться, счесаться, исаться, есаться, писаться).
    conjugation!("чесаться", 9, "~~аться";
        infinitive: "~~аться";
        non_past_first: "~шу~ь", "~шем~я";
        non_past_second: "~шешь~я", "~шете~ь";
        non_past_third: "~шет~я", "~шут~я";
        past: "~~ался", "~~алась", "~~алось", "~~ались";
        imperative: "~ши~ь", "~шите~ь";
        participle: "~шущий~я", "~~авшийся", "~е~емый", "~~итый";
        gerund: "~ша~ь", "~~авшись";
    ),
    // Статься: 9 words (одеться, надеться, вдеться, аться, статься, деться).
    conjugation!("статься", 9, "~ть~я";
        infinitive: "~ть~я";
        non_past_first: "~ну~ь", "~нем~я";
        non_past_second: "~нешь~я", "~нете~ь";
        non_past_third: "~нет~я", "~нут~я";
        past: "~л~я", "~ла~ь", "~ло~ь", "~ли~ь";
        imperative: "~нь~я", "~ньте~ь";
        participle: "~ющий~я", "~вший~я", "~е~емый", "~~итый";
        gerund: "~я~ь", "~вши~ь";
    ),
    // Уйти: 9 words (произойти, уйти, низойти, ойти, взойти, изойти).
    conjugation!("уйти", 9, "~йти";
        infinitive: "~йти";
        non_past_first: "~йду", "~йдём";
        non_past_second: "~йдёшь", "~йдёте";
        non_past_third: "~йдёт", "~йдут";
        past: "~шёл", "~шла", "~шло", "~шли";
        imperative: "~йди", "~йдите";
        participle: "~ющий", "~шедший", "~емый", "~йдённый";
        gerund: "~я", "~йдя|~шедши";
    ),
    // Резать: 9 words (урезать, врезать, срезать, вмазать, вынизать, резать).
    conjugation!("резать", 9, "~зать";
        infinitive: "~зать";
        non_past_first: "~жу", "~жем";
        non_past_second: "~жешь", "~жете";
        non_past_third: "~жет", "~жут";
        past: "~зал", "~зала", "~зало", "~зали";
        imperative: "~жь", "~жьте";
        participle: "~зающий", "~завший", "~заемый", "~занный";
        gerund: "~зая", "~зав|~завши";
    ),
    // Жить: 8 words (жить, плыть, ожить, выжить, сжить, лыть).
    conjugation!("жить", 8, "~ть";
        infinitive: "~ть";
        non_past_first: "~ву", "~вём";
        non_past_second: "~вёшь", "~вёте";
        non_past_third: "~вёт", "~вут";
        past: "~л", "~ла", "~ло", "~ли";
        imperative: "~ви", "~вите";
        participle: "~вущий", "~вший", "~емый", "~тый";
        gerund: "~вя", "~в|~вши";
    ),
    // Стереть: 8 words (стереть, подпереть, спереть, обтереть, втереть, впереть).
    conjugation!("стереть", 8, "~~е~еть";
        infinitive: "~~е~еть";
        non_past_first: "~о~~у", "~о~~ём";
        non_past_second: "~о~~ёшь", "~о~~ёте";
        non_past_third: "~о~~ёт", "~о~~ут";
        past: "~~ё~", "~~ё~ла", "~~ё~ло", "~~ё~ли";
        imperative: "~о~~и", "~о~~ите";
        participle: "~е~ущий~я", "~~ё~ший", "~~е~еть", "~~ё~тый";
        gerund: "~е~я~ь", "~~е~ев|~~ё~ши";
    ),
    // Добыть: 8 words (быть, прибыть, добыть, пребыть, пробыть, избыть).
    conjugation!("добыть", 8, "~ыть";
        infinitive: "~ыть";
        non_past_first: "~уду", "~удем";
        non_past_second: "~удешь", "~удете";
        non_past_third: "~удет", "~удут";
        past: "~ыл", "~ыла", "~ыло", "~ыли";
        imperative: "~удь", "~удьте";
        participle: "~оющий", "~ывший", "~оемый", "~ытый";
        gerund: "~оя", "~ыв|~ывши";
    ),
    // Врезаться: 8 words (врезаться, вмазаться, срезаться, урезаться, резаться, езаться).
    conjugation!("врезаться", 8, "~зать~я";
        infinitive: "~зать~я";
        non_past_first: "~жу~ь", "~жем~я";
        non_past_second: "~жешь~я", "~жете~ь";
        non_past_third: "~жет~я", "~жут~я";
        past: "~зал~я", "~зала~ь", "~зало~ь", "~зали~ь";
        imperative: "~жь~я", "~жьте~ь";
        participle: "~жущий~я", "~завший~я", "~е~емый", "~~итый";
        gerund: "~жа~ь", "~завши~ь";
    ),
    // Размять: 8 words (смять, измять, размять, подмять, вмять, обмять).
    conjugation!("размять", 8, "~~ять";
        infinitive: "~~ять";
        non_past_first: "~о~ну", "~о~нём";
        non_past_second: "~о~нёшь", "~о~нёте";
        non_past_third: "~о~нёт", "~о~нут";
        past: "~~ял", "~~яла", "~~яло", "~~яли";
        imperative: "~о~ни", "~о~ните";
        participle: "~ющий~я", "~~явший", "~е~емый", "~~ятый";
        gerund: "~я~ь", "~~яв|~~явши";
    ),
    // Равнять: 8 words (равнять, ровнять, сравнять, сровнять, уравнять, уровнять).
    conjugation!("равнять", 8, "~ять";
        infinitive: "~ять";
        non_past_first: "~яю", "~яем";
        non_past_second: "~яешь", "~яете";
        non_past_third: "~яет", "~яют";
        past: "~ял", "~яла", "~яло", "~яли";
        imperative: "~яй", "~яйте";
        participle: "~яющий", "~явший", "~яемый", "~енный";
        gerund: "~яя", "~яв|~явши";
    ),
    // Мяться: 8 words (замяться, мяться, помяться, вымяться, домяться, перемяться).
    conjugation!("мяться", 8, "~ять~я";
        infinitive: "~ять~я";
        non_past_first: "~ну~ь", "~нём~я";
        non_past_second: "~нёшь~я", "~нёте~ь";
        non_past_third: "~нёт~я", "~нут~я";
        past: "~ял~я", "~яла~ь", "~яло~ь", "~яли~ь";
        imperative: "~ни~ь", "~ните~ь";
        participle: "~нущий~я", "~явший~я", "~е~емый", "~~итый";
        gerund: "~ня~ь", "~явши~ь";
    ),
    // Зиждить: 8 words (зиждить, достелить, подстелить, простелить, взъемлить, исстелить).
    conjugation!("зиждить", 8, "~ить";
        infinitive: "~ить";
        non_past_first: "~ю", "~ем";
        non_past_second: "~ешь", "~ете";
        non_past_third: "~ет", "~ют";
        past: "~ил", "~ила", "~ило", "~или";
        imperative: "~и", "~ите";
        participle: "~ящий", "~ивший", "~имый", "~енный";
        gerund: "~я", "~ив|~ивши";
    ),
    // Пакоститься: 8 words (чихвоститься, пакоститься, выгоститься, невеститься, овеститься, веститься).
    conjugation!("пакоститься", 8, "~~титься";
        infinitive: "~~титься";
        non_past_first: "~щу~ь", "~~тимся";
        non_past_second: "~~тишься", "~~титесь";
        non_past_third: "~~тится", "~~тятся";
        past: "~~тился", "~~тилась", "~~тилось", "~~тились";
        imperative: "~~ться", "~~тьтесь";
        participle: "~~тящийся", "~~тившийся", "~е~емый", "~~итый";
        gerund: "~~тясь", "~~тившись";
    ),
    // Грестись: 8 words (выгрестись, отгрестись, подгрестись, соскрестись, поскрестись, рестись).
    conjugation!("грестись", 8, "~е~тись";
        infinitive: "~е~тись";
        non_past_first: "~ебу~ь", "~ебём~я";
        non_past_second: "~ебёшь~я", "~ебёте~ь";
        non_past_third: "~ебёт~я", "~ебут~я";
        past: "~ёб~я", "~ебла~ь", "~ебло~ь", "~ебли~ь";
        imperative: "~еби~ь", "~ебите~ь";
        participle: "~етущий~я", "~ёбший~я", "~е~емый", "~~итый";
        gerund: "~етя~ь", "~ебя~ь";
    ),
    // Лоть: 8 words (лоть, олоть, полоть, ороть, колоть, роть).
    conjugation!("лоть", 8, "~оть";
        infinitive: "~оть";
        non_past_first: "~ю", "~ем";
        non_past_second: "~ешь", "~ете";
        non_past_third: "~ет", "~ют";
        past: "~ол", "~ола", "~оло", "~оли";
        imperative: "~и", "~ите";
        participle: "~ющий", "~овший", "~емый", "~отый";
        gerund: "~я", "~ов|~овши";
    ),
    // Оесть: 7 words (есть, уесть, оесть, ъесть, надоесть, доесть).
    conjugation!("оесть", 7, "~сть";
        infinitive: "~сть";
        non_past_first: "~м", "~дим";
        non_past_second: "~шь", "~дите";
        non_past_third: "~ст", "~дят";
        past: "~л", "~ла", "~ло", "~ли";
        imperative: "~шь", "~шьте";
        participle: "~дящий", "~вший", "~домый", "~денный";
        gerund: "~дя", "~в|~вши";
    ),
    // Сковать: 7 words (ковать, совать, сковать, пломбировать, новать, ломбировать).
    conjugation!("сковать", 7, "~овать";
        infinitive: "~овать";
        non_past_first: "~ую", "~уём";
        non_past_second: "~уёшь", "~уёте";
        non_past_third: "~уёт", "~уют";
        past: "~овал", "~овала", "~овало", "~овали";
        imperative: "~уй", "~уйте";
        participle: "~ующий", "~овавший", "~уемый", "~ованный";
        gerund: "~уя", "~овав|~овавши";
    ),
    // Одеть: 7 words (надеть, одеть, деть, вдеть, сдеть, задеть).
    conjugation!("одеть", 7, "~ть";
        infinitive: "~ть";
        non_past_first: "~ну", "~нем";
        non_past_second: "~нешь", "~нете";
        non_past_third: "~нет", "~нут";
        past: "~л", "~ла", "~ло", "~ли";
        imperative: "~нь", "~ньте";
        participle: "~ющий", "~вший", "~емый", "~тый";
        gerund: "~я", "~в|~вши";
    ),
    // Разогнать: 7 words (подогнать, обогнать, отогнать, разогнать, согнать, вогнать).
    conjugation!("разогнать", 7, "~о~~ать";
        infinitive: "~о~~ать";
        non_past_first: "~~о~ю", "~~о~им";
        non_past_second: "~~о~ишь", "~~о~ите";
        non_past_third: "~~о~ит", "~~о~ят";
        past: "~о~~ал", "~о~~ала", "~о~~ало", "~о~~али";
        imperative: "~~о~и", "~~о~ите";
        participle: "~е~ущий~я", "~о~~авший", "~о~~ать", "~о~~анный";
        gerund: "~е~я~ь", "~о~~ав|~о~~авши";
    ),
    // Чесать: 7 words (чесать, тесать, очесать, счесать, утесать, пачесать).
    conjugation!("чесать", 7, "~есать";
        infinitive: "~есать";
        non_past_first: "~ешу", "~ешем";
        non_past_second: "~ешешь", "~ешете";
        non_past_third: "~ешет", "~ешут";
        past: "~есал", "~есала", "~есало", "~есали";
        imperative: "~еши", "~ешите";
        participle: "~ешущий", "~есавший", "~емый", "~ёсанный";
        gerund: "~еша", "~есав|~есавши";
    ),
    // Лезть: 7 words (лезть, вылезть, влезть, выгрызть, улезть, езть).
    conjugation!("лезть", 7, "~ть";
        infinitive: "~ть";
        non_past_first: "~у", "~ем";
        non_past_second: "~ешь", "~ете";
        non_past_third: "~ет", "~ут";
        past: "~", "~ла", "~ло", "~ли";
        imperative: "~ь", "~ьте";
        participle: "~ущий", "~ший", "~омый", "~енный";
        gerund: "~я", "~ши";
    ),
    // Выслать: 7 words (выслать, прислать, услать, переслать, заслать, наслать).
    conjugation!("выслать", 7, "~с~ать";
        infinitive: "~с~ать";
        non_past_first: "~ш~ю", "~ш~ём";
        non_past_second: "~ш~ёшь", "~ш~ёте";
        non_past_third: "~ш~ёт", "~ш~ют";
        past: "~с~ал", "~с~ала", "~с~ало", "~с~али";
        imperative: "~ш~и", "~ш~ите";
        participle: "~ющий~я", "~с~авший", "~е~емый", "~с~анный";
        gerund: "~я~ь", "~с~ав|~с~авши";
    ),
    // Отозваться: 7 words (отозваться, разогнаться, вогнаться, обозваться, отогнаться, подогнаться).
    conjugation!("отозваться", 7, "~о~~ать~я";
        infinitive: "~о~~ать~я";
        non_past_first: "~~о~ю~ь", "~~о~им~я";
        non_past_second: "~~о~ишь~я", "~~о~ите~ь";
        non_past_third: "~~о~ит~я", "~~о~ят~я";
        past: "~о~~ал~я", "~о~~ала~ь", "~о~~ало~ь|~о~~ало~ь", "~о~~али~ь|~о~~али~ь";
        imperative: "~~о~и~ь", "~~о~ите~ь";
        participle: "~о~~ать~я", "~о~~авший~я", "~о~~ать~я", "~о~~ать~я";
        gerund: "~о~~ать~я", "~о~~авши~ь";
    ),
    // Смяться: 7 words (измяться, смяться, размяться, вмяться, обмяться, подмяться).
    conjugation!("смяться", 7, "~~ять~я";
        infinitive: "~~ять~я";
        non_past_first: "~о~ну~ь", "~о~нём~я";
        non_past_second: "~о~нёшь~я", "~о~нёте~ь";
        non_past_third: "~о~нёт~я", "~о~нут~я";
        past: "~~ял~я", "~~яла~ь", "~~яло~ь", "~~яли~ь";
        imperative: "~о~ни~ь", "~о~ните~ь";
        participle: "~е~ущий~я", "~~явший~я", "~~ять~я", "~о~~анный";
        gerund: "~е~я~ь", "~~явши~ь";
    ),
    // Прячь: 7 words (перенапрячь, поднапрячь, перезапрячь, преднапрячь, небречь, рячь).
    conjugation!("прячь", 7, "~чь";
        infinitive: "~чь";
        non_past_first: "~гу", "~жём";
        non_past_second: "~жёшь", "~жёте";
        non_past_third: "~жёт", "~гут";
        past: "~г", "~гла", "~гло", "~гли";
        imperative: "~ги", "~гите";
        participle: "~гущий", "~гший", "~димый", "~жённый";
        gerund: "~дя", "~гши";
    ),
    // Ытить: 7 words (предвосхитить, сытить, хитить, ухитить, ытить, насытить).
    conjugation!("ытить", 7, "~тить";
        infinitive: "~тить";
        non_past_first: "~щу", "~тим";
        non_past_second: "~тишь", "~тите";
        non_past_third: "~тит", "~тят";
        past: "~тил", "~тила", "~тило", "~тили";
        imperative: "~ть", "~тьте";
        participle: "~тящий", "~тивший", "~тимый", "~щенный";
        gerund: "~тя", "~тив|~тивши";
    ),
    // Выслаться: 7 words (выслаться, дослаться, заслаться, переслаться, прислаться, наслаться).
    conjugation!("выслаться", 7, "~с~ать~я";
        infinitive: "~с~ать~я";
        non_past_first: "~ш~ю~ь", "~ш~ём~я";
        non_past_second: "~ш~ёшь~я", "~ш~ёте~ь";
        non_past_third: "~ш~ёт~я", "~ш~ют~я";
        past: "~с~ал~я", "~с~ала~ь", "~с~ало~ь", "~с~али~ь";
        imperative: "~ш~и~ь", "~ш~ите~ь";
        participle: "~е~ущий~я", "~с~авший~я", "~с~ать~я", "~о~~анный";
        gerund: "~е~я~ь", "~с~авши~ь";
    ),
    // Речься: 7 words (улечься, жечься, речься, остеречься, стеречься, лечься).
    conjugation!("речься", 7, "~ечь~я";
        infinitive: "~ечь~я";
        non_past_first: "~егу~ь", "~ежём~я";
        non_past_second: "~ежёшь~я", "~ежёте~ь";
        non_past_third: "~ежёт~я", "~егут~я";
        past: "~ёг~я", "~егла~ь", "~егло~ь", "~егли~ь";
        imperative: "~еги~ь", "~егите~ь";
        participle: "~егущий~я", "~ёгший~я", "~е~емый", "~~итый";
        gerund: "~я~ь", "~ёгши~ь";
    ),
    // Ызть: 7 words (вгрызть, ызть, грызть, рызть, зть, поднесть).
    conjugation!("ызть", 7, "~ть";
        infinitive: "~ть";
        non_past_first: "~у", "~ём";
        non_past_second: "~ёшь", "~ёте";
        non_past_third: "~ёт", "~ут";
        past: "~", "~ла", "~ло", "~ли";
        imperative: "~и", "~ите";
        participle: "~ущий", "~ший", "~омый", "~енный";
        gerund: "~я", "~ши";
    ),
    // Опорить: 7 words (мусорить, выверить, закупорить, расстопорить, заякорить, сахарить).
    conjugation!("опорить", 7, "~ить";
        infinitive: "~ить";
        non_past_first: "~ю", "~им";
        non_past_second: "~ишь", "~ите";
        non_past_third: "~ит", "~ят";
        past: "~ил", "~ила", "~ило", "~или";
        imperative: "~и|~ь", "~ьте";
        participle: "~ящий", "~ивший", "~имый", "~енный";
        gerund: "~я", "~ив|~ивши";
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
    // Вырасти: 6 words (расти, вырасти, врасти, урасти, повырасти, зарасти).
    conjugation!("вырасти", 6, "~а~ти";
        infinitive: "~а~ти";
        non_past_first: "~а~ту", "~а~тём";
        non_past_second: "~а~тёшь", "~а~тёте";
        non_past_third: "~а~тёт", "~а~тут";
        past: "~о~", "~о~ла", "~о~ло", "~о~ли";
        imperative: "~а~ти", "~а~тите";
        participle: "~а~тущий", "~о~ший", "~е~емый", "~о~тённый";
        gerund: "~а~тя", "~о~ши";
    ),
    // Даться: 6 words (сдаться, вдаться, даться, иться, создаться, родаться).
    conjugation!("даться", 6, "~ть~я";
        infinitive: "~ть~я";
        non_past_first: "~м~я", "~дим~я";
        non_past_second: "~шь~я", "~дите~ь";
        non_past_third: "~~тся", "~дут~я";
        past: "~л~я|~л~я", "~ла~ь", "~ло~ь|~ло~ь", "~ли~ь|~ли~ь";
        imperative: "~й~я", "~йте~ь";
        participle: "~ющий~я", "~вший~я", "~е~емый", "~~итый";
        gerund: "~я~ь", "~вши~ь";
    ),
    // Вылить: 6 words (вышить, вылить, вывить, повыбить, выпить, чайпить).
    conjugation!("вылить", 6, "~ить";
        infinitive: "~ить";
        non_past_first: "~ью", "~ьем";
        non_past_second: "~ьешь", "~ьете";
        non_past_third: "~ьет", "~ьют";
        past: "~ил", "~ила", "~ило", "~или";
        imperative: "~ей", "~ейте";
        participle: "~ьющий", "~ивший", "~имый", "~итый";
        gerund: "~ья", "~ив|~ивши";
    ),
    // Жать: 6 words (выжать, жать, ужать, нажать, прижать, зажать).
    conjugation!("жать", 6, "~ать";
        infinitive: "~ать";
        non_past_first: "~му", "~мём";
        non_past_second: "~мёшь", "~мёте";
        non_past_third: "~мёт", "~мут";
        past: "~ал", "~ала", "~ало", "~али";
        imperative: "~ми", "~мите";
        participle: "~мущий", "~авший", "~имый", "~атый";
        gerund: "~мя", "~ав|~авши";
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
    // Шибить: 6 words (ушибить, сшибить, вшибить, шибить, ибить, зашибить).
    conjugation!("шибить", 6, "~ить";
        infinitive: "~ить";
        non_past_first: "~у", "~ём";
        non_past_second: "~ёшь", "~ёте";
        non_past_third: "~ёт", "~ут";
        past: "~", "~ла", "~ло", "~ли";
        imperative: "~и", "~ите";
        participle: "~ащий", "~ивший", "~имый", "~ленный";
        gerund: "~а", "~ив|~ивши";
    ),
    // Плескаться: 6 words (плескаться, искаться, сыскаться, ыскаться, лескаться, оскаться).
    conjugation!("плескаться", 6, "~~каться";
        infinitive: "~~каться";
        non_past_first: "~щу~ь", "~щем~я";
        non_past_second: "~щешь~я", "~щете~ь";
        non_past_third: "~щет~я", "~щут~я";
        past: "~~кался", "~~калась", "~~калось", "~~кались";
        imperative: "~щи~ь", "~щите~ь";
        participle: "~щущий~я", "~~кавшийся", "~е~емый", "~~итый";
        gerund: "~ща~ь", "~~кавшись";
    ),
    // Тереться: 6 words (тереться, переться, вытереться, утереться, ростереться, запереться).
    conjugation!("тереться", 6, "~е~еть~я";
        infinitive: "~е~еть~я";
        non_past_first: "~~у~ь", "~~ём~я";
        non_past_second: "~~ёшь~я", "~~ёте~ь";
        non_past_third: "~~ёт~я", "~~ут~я";
        past: "~ё~~я", "~ё~ла~ь", "~ё~ло~ь", "~ё~ли~ь";
        imperative: "~~и~ь", "~~ите~ь";
        participle: "~~ущий~я", "~ё~ший~я", "~е~еть~я", "~о~~анный";
        gerund: "~е~я~ь", "~ё~ши~ь";
    ),
    // Стлать: 6 words (выдрать, повыдрать, повыбрать, стлать, тлать, выбрать).
    conjugation!("стлать", 6, "~~ать";
        infinitive: "~~ать";
        non_past_first: "~е~у", "~е~ем";
        non_past_second: "~е~ешь", "~е~ете";
        non_past_third: "~е~ет", "~е~ут";
        past: "~~ал", "~~ала", "~~ало", "~~али";
        imperative: "~е~и", "~е~ите";
        participle: "~е~ущий", "~~авший", "~е~омый", "~~анный";
        gerund: "~е~я", "~~ав|~~авши";
    ),
    // Хлестать: 6 words (хлестать, дристать, хвостать, нахлыстать, вистать, свистать).
    conjugation!("хлестать", 6, "~стать";
        infinitive: "~стать";
        non_past_first: "~щу", "~щем";
        non_past_second: "~щешь", "~щете";
        non_past_third: "~щет", "~щут";
        past: "~стал", "~стала", "~стало", "~стали";
        imperative: "~щи", "~щите";
        participle: "~щущий", "~ставший", "~скаемый", "~станный";
        gerund: "~ща", "~став|~ставши";
    ),
    // Обжечься: 6 words (обжечься, ожечься, разжечься, поджечься, сжечься, изжечься).
    conjugation!("обжечься", 6, "~~ечь~я";
        infinitive: "~~ечь~я";
        non_past_first: "~о~гу~ь", "~о~жём~я";
        non_past_second: "~о~жёшь~я", "~о~жёте~ь";
        non_past_third: "~о~жёт~я", "~о~гут~я";
        past: "~~ёг~я", "~о~гла~ь", "~о~гло~ь", "~о~гли~ь";
        imperative: "~о~ги~ь", "~о~гите~ь";
        participle: "~е~ущий~я", "~~ёгший~я", "~~ечь~я", "~о~~анный";
        gerund: "~е~я~ь", "~~ёгши~ь";
    ),
    // Исхлестаться: 6 words (хвостаться, расхлестаться, исхлестаться, нахлестаться, вистаться, свистаться).
    conjugation!("исхлестаться", 6, "~~таться";
        infinitive: "~~таться";
        non_past_first: "~щу~ь", "~щем~я";
        non_past_second: "~щешь~я", "~щете~ь";
        non_past_third: "~щет~я", "~щут~я";
        past: "~~тался", "~~талась", "~~талось", "~~тались";
        imperative: "~щи~ь", "~щите~ь";
        participle: "~щущий~я", "~~тавшийся", "~е~емый", "~~итый";
        gerund: "~ща~ь", "~~тавшись";
    ),
    // Ечься: 6 words (осечься, выволочься, вволочься, уволочься, ечься, волочься).
    conjugation!("ечься", 6, "~чь~я";
        infinitive: "~чь~я";
        non_past_first: "~ку~ь", "~чём~я";
        non_past_second: "~чёшь~я", "~чёте~ь";
        non_past_third: "~чёт~я", "~кут~я";
        past: "~к~я", "~кла~ь", "~кло~ь", "~кли~ь";
        imperative: "~ки~ь", "~ките~ь";
        participle: "~гущий~я", "~кший~я", "~е~емый", "~~итый";
        gerund: "~я~ь", "~кши~ь";
    ),
    // Стелиться: 6 words (зиждиться, взъемлиться, подстелиться, простелиться, стелиться, иждиться).
    conjugation!("стелиться", 6, "~ить~я";
        infinitive: "~ить~я";
        non_past_first: "~ю~ь", "~ем~я";
        non_past_second: "~ешь~я", "~ете~ь";
        non_past_third: "~ет~я", "~ют~я";
        past: "~ил~я", "~ила~ь", "~ило~ь", "~или~ь";
        imperative: "~и~ь", "~ите~ь";
        participle: "~ящий~я", "~ивший~я", "~е~емый", "~~итый";
        gerund: "~я~ь", "~ивши~ь";
    ),
    // Вырваться: 6 words (вырваться, выебаться, выткаться, глаголаться, жаждаться, саться).
    conjugation!("вырваться", 6, "~ать~я";
        infinitive: "~ать~я";
        non_past_first: "~у~ь", "~ем~я";
        non_past_second: "~ешь~я", "~ете~ь";
        non_past_third: "~ет~я", "~ут~я";
        past: "~ал~я", "~ала~ь", "~ало~ь", "~али~ь";
        imperative: "~и~ь", "~ите~ь";
        participle: "~ущий~я", "~авший~я", "~е~емый", "~~итый";
        gerund: "~ая~ь", "~авши~ь";
    ),
    // Лоться: 6 words (роться, колоться, лоться, ороться, пороться, полоться).
    conjugation!("лоться", 6, "~оть~я";
        infinitive: "~оть~я";
        non_past_first: "~ю~ь", "~ем~я";
        non_past_second: "~ешь~я", "~ете~ь";
        non_past_third: "~ет~я", "~ют~я";
        past: "~ол~я", "~ола~ь", "~оло~ь", "~оли~ь";
        imperative: "~и~ь", "~ите~ь";
        participle: "~ющий~я", "~овший~я", "~е~емый", "~~итый";
        gerund: "~я~ь", "~овши~ь";
    ),
    // Выгрести: 6 words (разгрести, выгрести, повыгрести, грести, скрести, крести).
    conjugation!("выгрести", 6, "~ести";
        infinitive: "~ести";
        non_past_first: "~ебу", "~ебём";
        non_past_second: "~ебёшь", "~ебёте";
        non_past_third: "~ебёт", "~ебут";
        past: "~ёб", "~ебла", "~ебло", "~ебли";
        imperative: "~еби", "~ебите";
        participle: "~етущий", "~ёбший", "~емый", "~ебённый";
        gerund: "~етя", "~ёбши";
    ),
    // Рякнуть: 6 words (нисповергнуть, рякнуть, провергнуть, оргнуть, верзнуть, ибнуть).
    conjugation!("рякнуть", 6, "~нуть";
        infinitive: "~нуть";
        non_past_first: "~ну", "~нем";
        non_past_second: "~нешь", "~нете";
        non_past_third: "~нет", "~нут";
        past: "~", "~ла", "~ло", "~ли";
        imperative: "~ни", "~ните";
        participle: "~нущий", "~ший|~нувший", "~емый", "~нутый";
        gerund: "~я", "~ши|~нув|~нувши";
    ),
    // Лгнуть: 6 words (горкнуть, дряхнуть, жухнуть, осмеркнуть, лгнуть, узнуть).
    conjugation!("лгнуть", 6, "~нуть";
        infinitive: "~нуть";
        non_past_first: "~ну", "~нем";
        non_past_second: "~нешь", "~нете";
        non_past_third: "~нет", "~нут";
        past: "~нул", "~ла", "~ло", "~ли";
        imperative: "~ни", "~ните";
        participle: "~нущий", "~нувший", "~емый", "~нутый";
        gerund: "~я", "~нув|~нувши";
    ),
    // Вгрызться: 6 words (вгрызться, грызться, лезться, ызться, рызться, верзться).
    conjugation!("вгрызться", 6, "~ть~я";
        infinitive: "~ть~я";
        non_past_first: "~у~ь", "~ём~я";
        non_past_second: "~ёшь~я", "~ёте~ь";
        non_past_third: "~ёт~я", "~ут~я";
        past: "~~я", "~ла~ь", "~ло~ь", "~ли~ь";
        imperative: "~и~ь", "~ите~ь";
        participle: "~ущий~я", "~ший~я", "~е~емый", "~~итый";
        gerund: "~я~ь", "~ши~ь";
    ),
    // Смотреть: 5 words (презреть, узреть, смотреть, усмотреть, мотреть).
    conjugation!("смотреть", 5, "~еть";
        infinitive: "~еть";
        non_past_first: "~ю", "~им";
        non_past_second: "~ишь", "~ите";
        non_past_third: "~ит", "~ят";
        past: "~ел", "~ела", "~ело", "~ели";
        imperative: "~и", "~ите";
        participle: "~ящий", "~евший", "~имый", "~енный";
        gerund: "~я", "~ев|~евши";
    ),
    // Бежать: 5 words (бежать, прибежать, забежать, обежать, ежать).
    conjugation!("бежать", 5, "~жать";
        infinitive: "~жать";
        non_past_first: "~гу", "~жим";
        non_past_second: "~жишь", "~жите";
        non_past_third: "~жит", "~гут";
        past: "~жал", "~жала", "~жало", "~жали";
        imperative: "~ги", "~гите";
        participle: "~гущий", "~жавший", "~емый", "~жённый";
        gerund: "~я", "~жав|~жавши";
    ),
    // Жечь: 5 words (жечь, выжечь, ожечь, повыжечь, прижечь).
    conjugation!("жечь", 5, "~ечь";
        infinitive: "~ечь";
        non_past_first: "~гу", "~жём";
        non_past_second: "~жёшь", "~жёте";
        non_past_third: "~жёт", "~гут";
        past: "~ёг", "~гла", "~гло", "~гли";
        imperative: "~ги", "~гите";
        participle: "~гущий", "~ёгший", "~екомый", "~жённый";
        gerund: "~дя", "~ёгши";
    ),
    // Идеть: 5 words (пердеть, издеть, идеть, ердеть, пиздеть).
    conjugation!("идеть", 5, "~деть";
        infinitive: "~деть";
        non_past_first: "~жу", "~дим";
        non_past_second: "~дишь", "~дите";
        non_past_third: "~дит", "~дят";
        past: "~дел", "~дела", "~дело", "~дели";
        imperative: "~ди", "~дите";
        participle: "~дящий", "~девший", "~димый", "~женный";
        gerund: "~дя", "~дев|~девши";
    ),
    // Мести: 5 words (гнести, плести, мести, цвести, рассвести).
    conjugation!("мести", 5, "~ести";
        infinitive: "~ести";
        non_past_first: "~ету", "~етём";
        non_past_second: "~етёшь", "~етёте";
        non_past_third: "~етёт", "~етут";
        past: "~ёл", "~ела", "~ело", "~ели";
        imperative: "~ети", "~етите";
        participle: "~етущий", "~ётший", "~имый", "~етённый";
        gerund: "~етя", "~ётши";
    ),
    // Стояться: 5 words (бояться, настояться, стояться, ояться, тояться).
    conjugation!("стояться", 5, "~ять~я";
        infinitive: "~ять~я";
        non_past_first: "~ю~ь", "~им~я";
        non_past_second: "~ишь~я", "~ите~ь";
        non_past_third: "~ит~я", "~ят~я";
        past: "~ял~я", "~яла~ь", "~яло~ь", "~яли~ь";
        imperative: "~й~я", "~йте~ь";
        participle: "~ящий~я", "~явший~я", "~е~емый", "~~итый";
        gerund: "~я~ь", "~явши~ь";
    ),
    // Вязать: 5 words (лизать, вязать, казать, низать, изать).
    conjugation!("вязать", 5, "~зать";
        infinitive: "~зать";
        non_past_first: "~жу", "~жем";
        non_past_second: "~жешь", "~жете";
        non_past_third: "~жет", "~жут";
        past: "~зал", "~зала", "~зало", "~зали";
        imperative: "~жи", "~жите";
        participle: "~жущий", "~завший", "~заемый", "~занный";
        gerund: "~жа", "~зав|~завши";
    ),
    // Отжать: 5 words (сжать, обжать, разжать, отжать, вжать).
    conjugation!("отжать", 5, "~~ать";
        infinitive: "~~ать";
        non_past_first: "~о~му", "~о~мём";
        non_past_second: "~о~мёшь", "~о~мёте";
        non_past_third: "~о~мёт", "~о~мут";
        past: "~~ал", "~~ала", "~~ало", "~~али";
        imperative: "~о~ми", "~о~мите";
        participle: "~~ущий", "~~авший", "~е~омый", "~~атый";
        gerund: "~~я", "~~ав|~~авши";
    ),
    // Видеться: 5 words (обидеться, видеться, пообидеться, бидеться, навидеться).
    conjugation!("видеться", 5, "~деть~я";
        infinitive: "~деть~я";
        non_past_first: "~жу~ь", "~дим~я";
        non_past_second: "~дишь~я", "~дите~ь";
        non_past_third: "~дит~я", "~дят~я";
        past: "~дел~я", "~дела~ь", "~дело~ь", "~дели~ь";
        imperative: "~дь~я", "~дьте~ь";
        participle: "~дящий~я", "~девший~я", "~е~емый", "~~итый";
        gerund: "~дя~ь", "~девши~ь";
    ),
    // Пыхать: 5 words (брехать, пыхать, сбрехать, обрехать, рехать).
    conjugation!("пыхать", 5, "~хать";
        infinitive: "~хать";
        non_past_first: "~шу", "~шем";
        non_past_second: "~шешь", "~шете";
        non_past_third: "~шет", "~шут";
        past: "~хал", "~хала", "~хало", "~хали";
        imperative: "~ши", "~шите";
        participle: "~шущий", "~хавший", "~симый", "~ханный";
        gerund: "~ша", "~хав|~хавши";
    ),
    // Сесться: 5 words (усесться, осесться, ссесться, обсесться, сесться).
    conjugation!("сесться", 5, "~е~ться";
        infinitive: "~е~ться";
        non_past_first: "~яду~ь", "~ядем~я";
        non_past_second: "~ядешь~я", "~ядете~ь";
        non_past_third: "~ядет~я", "~ядут~я";
        past: "~ел~я", "~ела~ь", "~ело~ь", "~ели~ь";
        imperative: "~ядь~я", "~ядьте~ь";
        participle: "~ящий~я", "~евший~я", "~е~емый", "~~итый";
        gerund: "~я~ь", "~евши~ь";
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
    // Обмереть: 5 words (вытереть, отпереть, обмереть, отмереть, повымереть).
    conjugation!("обмереть", 5, "~~е~еть";
        infinitive: "~~е~еть";
        non_past_first: "~о~~у", "~о~~ём";
        non_past_second: "~о~~ёшь", "~о~~ёте";
        non_past_third: "~о~~ёт", "~о~~ут";
        past: "~~е~", "~~е~ла", "~~е~ло", "~~е~ли";
        imperative: "~о~~и", "~о~~ите";
        participle: "~е~ущий~я", "~~е~ший", "~~е~еть", "~~е~тый";
        gerund: "~е~я~ь", "~~е~ев";
    ),
    // Унять: 5 words (унять, понять, занять, нанять, призанять).
    conjugation!("унять", 5, "~нять";
        infinitive: "~нять";
        non_past_first: "~йму", "~ймём";
        non_past_second: "~ймёшь", "~ймёте";
        non_past_third: "~ймёт", "~ймут";
        past: "~нял", "~няла", "~няло", "~няли";
        imperative: "~йми", "~ймите";
        participle: "~ющий", "~нявший", "~емый", "~нятый";
        gerund: "~я", "~няв|~нявши";
    ),
    // Лестись: 5 words (поплестись, проплестись, соплестись, лестись, обрестись).
    conjugation!("лестись", 5, "~е~тись";
        infinitive: "~е~тись";
        non_past_first: "~ету~ь", "~етём~я";
        non_past_second: "~етёшь~я", "~етёте~ь";
        non_past_third: "~етёт~я", "~етут~я";
        past: "~ёл~я", "~ела~ь", "~ело~ь", "~ели~ь";
        imperative: "~ети~ь", "~етите~ь";
        participle: "~етущий~я", "~ётший~я", "~е~емый", "~~итый";
        gerund: "~етя~ь", "~етя~ь|~ётши~ь";
    ),
    // Тыкаться: 5 words (тыкаться, плакаться, доскакаться, мурлыкаться, скакаться).
    conjugation!("тыкаться", 5, "~кать~я";
        infinitive: "~кать~я";
        non_past_first: "~чу~ь", "~чем~я";
        non_past_second: "~чешь~я", "~чете~ь";
        non_past_third: "~чет~я", "~чут~я";
        past: "~кал~я", "~кала~ь", "~кало~ь", "~кали~ь";
        imperative: "~чь~я", "~чьте~ь";
        participle: "~чущий~я", "~кавший~я", "~е~емый", "~~итый";
        gerund: "~ча~ь", "~кавши~ь";
    ),
    // Зачертить: 5 words (вскатить, докатить, дочертить, зачертить, подчертить).
    conjugation!("зачертить", 5, "~тить";
        infinitive: "~тить";
        non_past_first: "~чу", "~тим";
        non_past_second: "~тишь", "~тите";
        non_past_third: "~тит", "~тят";
        past: "~тил", "~тила", "~тило", "~тили";
        imperative: "~ти", "~тите";
        participle: "~тящий", "~тивший", "~тимый", "~танный";
        gerund: "~тя", "~тив|~тивши";
    ),
    // Бечь: 5 words (бечь, речь, еречь, остеречь, стеречь).
    conjugation!("бечь", 5, "~ечь";
        infinitive: "~ечь";
        non_past_first: "~егу", "~ежём";
        non_past_second: "~ежёшь", "~ежёте";
        non_past_third: "~ежёт", "~егут";
        past: "~ёг", "~егла", "~егло", "~егли";
        imperative: "~еги", "~егите";
        participle: "~егущий", "~ёгший", "~екомый", "~ежённый";
        gerund: "~я", "~ёгши";
    ),
    // Няться: 5 words (сняться, вняться, няться, одняться, обняться).
    conjugation!("няться", 5, "~ять~я";
        infinitive: "~ять~я";
        non_past_first: "~иму~ь", "~имем~я";
        non_past_second: "~имешь~я", "~имете~ь";
        non_past_third: "~имет~я", "~имут~я";
        past: "~ял~я|~ял~я", "~яла~ь", "~яло~ь|~яло~ь", "~яли~ь|~яли~ь";
        imperative: "~ими~ь", "~имите~ь";
        participle: "~яющий~я", "~явший~я", "~е~емый", "~~итый";
        gerund: "~яя~ь", "~явши~ь";
    ),
    // Рщиться: 5 words (ращиться, рщиться, орщиться, порщиться, орядочиться).
    conjugation!("рщиться", 5, "~ить~я";
        infinitive: "~ить~я";
        non_past_first: "~у~ь", "~им~я";
        non_past_second: "~ишь~я", "~ите~ь";
        non_past_third: "~ит~я", "~ат~я";
        past: "~ил~я", "~ила~ь", "~ило~ь", "~или~ь";
        imperative: "~и~ь|~ь~я", "~ьте~ь";
        participle: "~ащий~я", "~ивший~я", "~е~емый", "~~итый";
        gerund: "~а~ь", "~ивши~ь";
    ),
    // Ынуть: 5 words (ынуть, тынуть, стынуть, трянуть, остынуть).
    conjugation!("ынуть", 5, "~нуть";
        infinitive: "~нуть";
        non_past_first: "~ну", "~нем";
        non_past_second: "~нешь", "~нете";
        non_past_third: "~нет", "~нут";
        past: "~л", "~ла", "~ло", "~ли";
        imperative: "~нь", "~ньте";
        participle: "~нущий", "~вший", "~емый", "~нутый";
        gerund: "~я", "~в|~вши";
    ),
    // Обнять: 5 words (однять, обнять, приять, отнять, снять).
    conjugation!("обнять", 5, "~ять";
        infinitive: "~ять";
        non_past_first: "~иму", "~имем";
        non_past_second: "~имешь", "~имете";
        non_past_third: "~имет", "~имут";
        past: "~ял", "~яла", "~яло", "~яли";
        imperative: "~ими", "~имите";
        participle: "~нущий", "~явший", "~емый", "~ятый";
        gerund: "~ня", "~яв|~явши";
    ),
    // Трепать: 5 words (трепать, репать, лебать, истрепать, обтрепать).
    conjugation!("трепать", 5, "~е~ать";
        infinitive: "~е~ать";
        non_past_first: "~е~лю", "~е~лем";
        non_past_second: "~е~лешь", "~е~лете";
        non_past_third: "~е~лет", "~е~лют";
        past: "~е~ал", "~е~ала", "~е~ало", "~е~али";
        imperative: "~е~ли", "~е~лите";
        participle: "~е~лющий", "~е~авший", "~е~лемый", "~ё~анный";
        gerund: "~е~ля", "~е~ав|~е~авши";
    ),
    // Вигнуться: 5 words (ргнуться, верзнуться, вигнуться, рзнуться, овергнуться).
    conjugation!("вигнуться", 5, "~нуть~я";
        infinitive: "~нуть~я";
        non_past_first: "~ну~ь", "~нем~я";
        non_past_second: "~нешь~я", "~нете~ь";
        non_past_third: "~нет~я", "~нут~я";
        past: "~нул~я", "~ла~ь", "~ло~ь", "~ли~ь";
        imperative: "~ни~ь", "~ните~ь";
        participle: "~нущий~я", "~нувший~я|~ший~я", "~е~емый", "~~итый";
        gerund: "~я~ь", "~ши~ь";
    ),
    // Усориться: 5 words (обмохнатиться, купориться, усориться, йориться, упориться).
    conjugation!("усориться", 5, "~ить~я";
        infinitive: "~ить~я";
        non_past_first: "~ю~ь", "~им~я";
        non_past_second: "~ишь~я", "~ите~ь";
        non_past_third: "~ит~я", "~ят~я";
        past: "~ил~я", "~ила~ь", "~ило~ь", "~или~ь";
        imperative: "~ь~я|~и~ь", "~ьте~ь";
        participle: "~ящий~я", "~ивший~я", "~е~емый", "~~итый";
        gerund: "~я~ь", "~я~ь|~ивши~ь";
    ),
    // Лечь: 4 words (лечь, налечь, отлечь, улечь).
    conjugation!("лечь", 4, "~ечь";
        infinitive: "~ечь";
        non_past_first: "~ягу", "~яжем";
        non_past_second: "~яжешь", "~яжете";
        non_past_third: "~яжет", "~ягут";
        past: "~ёг", "~егла", "~егло", "~егли";
        imperative: "~яг", "~ягте";
        participle: "~егущий", "~ёгший", "~екомый", "~ежённый";
        gerund: "~я", "~ёгши";
    ),
    // Нести: 4 words (нести, везти, уебти, заебти).
    conjugation!("нести", 4, "~е~ти";
        infinitive: "~е~ти";
        non_past_first: "~е~у", "~е~ём";
        non_past_second: "~е~ёшь", "~е~ёте";
        non_past_third: "~е~ёт", "~е~ут";
        past: "~ё~", "~е~ла", "~е~ло", "~е~ли";
        imperative: "~е~и", "~е~ите";
        participle: "~е~ущий", "~ё~ший", "~е~омый", "~е~ённый";
        gerund: "~е~я", "~ё~ши";
    ),
    // Плестись: 4 words (плестись, местись, цвестись, гнестись).
    conjugation!("плестись", 4, "~е~тись";
        infinitive: "~е~тись";
        non_past_first: "~ету~ь", "~етём~я";
        non_past_second: "~етёшь~я", "~етёте~ь";
        non_past_third: "~етёт~я", "~етут~я";
        past: "~ёл~я", "~ела~ь", "~ело~ь", "~ели~ь";
        imperative: "~ети~ь", "~етите~ь";
        participle: "~етущий~я", "~ётший~я", "~е~емый", "~~итый";
        gerund: "~етя~ь", "~ётши~ь";
    ),
    // Жаться: 4 words (жаться, ужаться, выжаться, прижаться).
    conjugation!("жаться", 4, "~ать~я";
        infinitive: "~ать~я";
        non_past_first: "~му~ь", "~мём~я";
        non_past_second: "~мёшь~я", "~мёте~ь";
        non_past_third: "~мёт~я", "~мут~я";
        past: "~ал~я", "~ала~ь", "~ало~ь", "~али~ь";
        imperative: "~ми~ь", "~мите~ь";
        participle: "~мущий~я", "~авший~я", "~е~емый", "~~итый";
        gerund: "~мя~ь", "~авши~ь";
    ),
    // Вынестись: 4 words (вынестись, вывезтись, вытрястись, верзтись).
    conjugation!("вынестись", 4, "~ти~ь";
        infinitive: "~ти~ь";
        non_past_first: "~у~ь", "~ем~я";
        non_past_second: "~ешь~я", "~ете~ь";
        non_past_third: "~ет~я", "~ут~я";
        past: "~~я", "~ла~ь", "~ло~ь", "~ли~ь";
        imperative: "~и~ь", "~ите~ь";
        participle: "~ущий~я", "~ший~я", "~е~емый", "~~итый";
        gerund: "~я~ь", "~я~ь";
    ),
    // Леваться: 4 words (плеваться, клеваться, леваться, блеваться).
    conjugation!("леваться", 4, "~евать~я";
        infinitive: "~евать~я";
        non_past_first: "~юю~ь", "~юём~я";
        non_past_second: "~юёшь~я", "~юёте~ь";
        non_past_third: "~юёт~я", "~юют~я";
        past: "~евал~я", "~евала~ь", "~евало~ь", "~евали~ь";
        imperative: "~юй~я", "~юйте~ь";
        participle: "~юющий~я", "~евавший~я", "~е~емый", "~~итый";
        gerund: "~юя~ь", "~евавши~ь";
    ),
    // Ушибиться: 4 words (ушибиться, сшибиться, ибиться, шибиться).
    conjugation!("ушибиться", 4, "~ить~я";
        infinitive: "~ить~я";
        non_past_first: "~у~ь", "~ём~я";
        non_past_second: "~ёшь~я", "~ёте~ь";
        non_past_third: "~ёт~я", "~ут~я";
        past: "~~я", "~ла~ь", "~ло~ь", "~ли~ь";
        imperative: "~и~ь", "~ите~ь";
        participle: "~ащий~я", "~ивший~я", "~е~емый", "~~итый";
        gerund: "~а~ь", "~ивши~ь";
    ),
    // Испить: 4 words (расшить, испить, распить, исшить).
    conjugation!("испить", 4, "~с~ить";
        infinitive: "~с~ить";
        non_past_first: "~зо~ью", "~зо~ьём";
        non_past_second: "~зо~ьёшь", "~зо~ьёте";
        non_past_third: "~зо~ьёт", "~зо~ьют";
        past: "~с~ил", "~с~ила", "~с~ило", "~с~или";
        imperative: "~с~ей", "~с~ейте";
        participle: "~с~ящий", "~с~ивший", "~е~емый", "~с~итый";
        gerund: "~с~я", "~с~ив|~с~ивши";
    ),
    // Уебать: 4 words (уебать, заебать, роебать, тебать).
    conjugation!("уебать", 4, "~е~ать";
        infinitive: "~е~ать";
        non_past_first: "~е~у", "~е~ём";
        non_past_second: "~е~ёшь", "~е~ёте";
        non_past_third: "~е~ёт", "~е~ут";
        past: "~е~ал", "~е~ала", "~е~ало", "~е~али";
        imperative: "~е~и", "~е~ите";
        participle: "~е~лющий", "~е~авший", "~е~лемый", "~ё~анный";
        gerund: "~е~ля", "~е~ав|~е~авши";
    ),
    // Бороздить: 4 words (гвоздить, омоздить, громоздить, бороздить).
    conjugation!("бороздить", 4, "~здить";
        infinitive: "~здить";
        non_past_first: "~зжу", "~здим";
        non_past_second: "~здишь", "~здите";
        non_past_third: "~здит", "~здят";
        past: "~здил", "~здила", "~здило", "~здили";
        imperative: "~зди", "~здите";
        participle: "~здящий", "~здивший", "~здимый", "~ждённый";
        gerund: "~здя", "~здив|~здивши";
    ),
    // Хвостить: 4 words (чихвостить, чехвостить, шилохвостить, хвостить).
    conjugation!("хвостить", 4, "~стить";
        infinitive: "~стить";
        non_past_first: "~щу", "~стим";
        non_past_second: "~стишь", "~стите";
        non_past_third: "~стит", "~стят";
        past: "~стил", "~стила", "~стило", "~стили";
        imperative: "~сть", "~стьте";
        participle: "~стящий", "~стивший", "~стимый", "~щенный";
        gerund: "~стя", "~стив|~стивши";
    ),
    // Усять: 4 words (очинять, оязыковлять, усять, стеснять).
    conjugation!("усять", 4, "~ять";
        infinitive: "~ять";
        non_past_first: "~яю", "~яем";
        non_past_second: "~яешь", "~яете";
        non_past_third: "~яет", "~яют";
        past: "~ял", "~яла", "~яло", "~яли";
        imperative: "~яй", "~яйте";
        participle: "~яющий", "~явший", "~яемый", "~ённый";
        gerund: "~яя", "~яв|~явши";
    ),
    // Изъяться: 4 words (изъяться, подъяться, разъяться, отъяться).
    conjugation!("изъяться", 4, "~ъять~я";
        infinitive: "~ъять~я";
        non_past_first: "~ыму~ь", "~ымем~я";
        non_past_second: "~ымешь~я", "~ымете~ь";
        non_past_third: "~ымет~я", "~ымут~я";
        past: "~ъял~я", "~ъяла~ь", "~ъяло~ь", "~ъяли~ь";
        imperative: "~ыми~ь", "~ымите~ь";
        participle: "~ющий~я", "~ъявший~я", "~е~емый", "~~итый";
        gerund: "~я~ь", "~ъявши~ь";
    ),
    // Ытиться: 4 words (ухититься, сытиться, ытиться, насытиться).
    conjugation!("ытиться", 4, "~тить~я";
        infinitive: "~тить~я";
        non_past_first: "~щу~ь", "~тим~я";
        non_past_second: "~тишь~я", "~тите~ь";
        non_past_third: "~тит~я", "~тят~я";
        past: "~тил~я", "~тила~ь", "~тило~ь", "~тили~ь";
        imperative: "~ть~я", "~тьте~ь";
        participle: "~тящий~я", "~тивший~я", "~е~емый", "~~итый";
        gerund: "~тя~ь", "~тивши~ь";
    ),
    // Оять: 4 words (настоять, оять, стоять, тоять).
    conjugation!("оять", 4, "~ять";
        infinitive: "~ять";
        non_past_first: "~ю", "~им";
        non_past_second: "~ишь", "~ите";
        non_past_third: "~ит", "~ят";
        past: "~ял", "~яла", "~яло", "~яли";
        imperative: "~й", "~йте";
        participle: "~ящий", "~явший", "~яемый", "~янный";
        gerund: "~я", "~яв|~явши";
    ),
    // Рчать: 4 words (ичать, лчать, рчать, ршать).
    conjugation!("рчать", 4, "~ать";
        infinitive: "~ать";
        non_past_first: "~у", "~им";
        non_past_second: "~ишь", "~ите";
        non_past_third: "~ит", "~ат";
        past: "~ал", "~ала", "~ало", "~али";
        imperative: "~и", "~ите";
        participle: "~ащий", "~авший", "~аемый", "~анный";
        gerund: "~а", "~ав|~авши";
    ),
    // Еяться: 4 words (еяться, меяться, смеяться, пияться).
    conjugation!("еяться", 4, "~ять~я";
        infinitive: "~ять~я";
        non_past_first: "~ю~ь", "~ём~я";
        non_past_second: "~ёшь~я", "~ёте~ь";
        non_past_third: "~ёт~я", "~ют~я";
        past: "~ял~я", "~яла~ь", "~яло~ь", "~яли~ь";
        imperative: "~й~я", "~йте~ь";
        participle: "~ющий~я", "~явший~я", "~е~емый", "~~итый";
        gerund: "~я~ь", "~явши~ь";
    ),
    // Стричь: 4 words (ичь, тричь, стричь, бречь).
    conjugation!("стричь", 4, "~чь";
        infinitive: "~чь";
        non_past_first: "~гу", "~жём";
        non_past_second: "~жёшь", "~жёте";
        non_past_third: "~жёт", "~гут";
        past: "~г", "~гла", "~гло", "~гли";
        imperative: "~ги", "~гите";
        participle: "~гущий", "~гший", "~димый", "~женный";
        gerund: "~дя", "~гши";
    ),
    // Домить: 4 words (дрейфить, ведомить, домить, уведомить).
    conjugation!("домить", 4, "~ить";
        infinitive: "~ить";
        non_past_first: "~лю", "~им";
        non_past_second: "~ишь", "~ите";
        non_past_third: "~ит", "~ят";
        past: "~ил", "~ила", "~ило", "~или";
        imperative: "~и|~ь", "~ьте";
        participle: "~ящий", "~ивший", "~имый", "~ленный";
        gerund: "~я", "~ив|~ивши";
    ),
    // Измыслить: 4 words (измыслить, обмыслить, умыслить, ыслить).
    conjugation!("измыслить", 4, "~с~ить";
        infinitive: "~с~ить";
        non_past_first: "~с~ю", "~с~им";
        non_past_second: "~с~ишь", "~с~ите";
        non_past_third: "~с~ит", "~с~ят";
        past: "~с~ил", "~с~ила", "~с~ило", "~с~или";
        imperative: "~с~и", "~с~ите";
        participle: "~с~ящий", "~с~ивший", "~е~емый", "~ш~енный";
        gerund: "~с~я", "~с~ив|~с~ивши";
    ),
    // Петаться: 4 words (трепетаться, петаться, леветаться, репетаться).
    conjugation!("петаться", 4, "~тать~я";
        infinitive: "~тать~я";
        non_past_first: "~щу~ь", "~щем~я";
        non_past_second: "~щешь~я", "~щете~ь";
        non_past_third: "~щет~я", "~щут~я";
        past: "~тал~я", "~тала~ь", "~тало~ь", "~тали~ь";
        imperative: "~щи~ь", "~щите~ь";
        participle: "~щущий~я", "~тавший~я", "~е~емый", "~~итый";
        gerund: "~ща~ь", "~тавши~ь";
    ),
    // Ведомиться: 4 words (понадобиться, занадобиться, ведомиться, домиться).
    conjugation!("ведомиться", 4, "~ить~я";
        infinitive: "~ить~я";
        non_past_first: "~лю~ь", "~им~я";
        non_past_second: "~ишь~я", "~ите~ь";
        non_past_third: "~ит~я", "~ят~я";
        past: "~ил~я", "~ила~ь", "~ило~ь", "~или~ь";
        imperative: "~и~ь|~ь~я", "~ьте~ь";
        participle: "~ящий~я", "~ивший~я", "~е~емый", "~~итый";
        gerund: "~я~ь", "~ивши~ь";
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
        gerund: "~~я", "~ё~ши";
    ),
    // Нестись: 3 words (нестись, везтись, свезтись).
    conjugation!("нестись", 3, "~е~ти~ь";
        infinitive: "~е~ти~ь";
        non_past_first: "~е~у~ь", "~е~ём~я";
        non_past_second: "~е~ёшь~я", "~е~ёте~ь";
        non_past_third: "~е~ёт~я", "~е~ут~я";
        past: "~ё~~я", "~е~ла~ь", "~е~ло~ь", "~е~ли~ь";
        imperative: "~е~и~ь", "~е~ите~ь";
        participle: "~е~ущий~я", "~ё~ший~я", "~е~ти~ь", "~о~~анный";
        gerund: "~е~я~ь", "~ё~ши~ь";
    ),
    // Чтить: 3 words (чтить, тщить, чуждить).
    conjugation!("чтить", 3, "~ить";
        infinitive: "~ить";
        non_past_first: "~у", "~им";
        non_past_second: "~ишь", "~ите";
        non_past_third: "~ит", "~ят";
        past: "~ил", "~ила", "~ило", "~или";
        imperative: "~и", "~ите";
        participle: "~ящий", "~ивший", "~имый", "~ённый";
        gerund: "~я", "~ив|~ивши";
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
    // Соваться: 3 words (коваться, соваться, новаться).
    conjugation!("соваться", 3, "~овать~я";
        infinitive: "~овать~я";
        non_past_first: "~ую~ь", "~уём~я";
        non_past_second: "~уёшь~я", "~уёте~ь";
        non_past_third: "~уёт~я", "~уют~я";
        past: "~овал~я", "~овала~ь", "~овало~ь", "~овали~ь";
        imperative: "~уй~я", "~уйте~ь";
        participle: "~ующий~я", "~овавший~я", "~е~емый", "~~итый";
        gerund: "~уя~ь", "~овавши~ь";
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
    // Выдвинуться: 3 words (выдвинуться, унуться, кашлянуться).
    conjugation!("выдвинуться", 3, "~уть~я";
        infinitive: "~уть~я";
        non_past_first: "~у~ь", "~ем~я";
        non_past_second: "~ешь~я", "~ете~ь";
        non_past_third: "~ет~я", "~ут~я";
        past: "~ул~я", "~ула~ь", "~уло~ь", "~ули~ь";
        imperative: "~и~ь|~ь~я", "~ьте~ь";
        participle: "~ущий~я", "~увший~я", "~е~емый", "~~итый";
        gerund: "~ая~ь", "~увши~ь";
    ),
    // Уняться: 3 words (уняться, заняться, наняться).
    conjugation!("уняться", 3, "~нять~я";
        infinitive: "~нять~я";
        non_past_first: "~йму~ь", "~ймём~я";
        non_past_second: "~ймёшь~я", "~ймёте~ь";
        non_past_third: "~ймёт~я", "~ймут~я";
        past: "~нял~я|~нял~я", "~няла~ь", "~няло~ь", "~няли~ь";
        imperative: "~йми~ь", "~ймите~ь";
        participle: "~ющий~я", "~нявший~я", "~е~емый", "~~итый";
        gerund: "~я~ь", "~нявши~ь";
    ),
    // Истереть: 3 words (растереть, распереть, истереть).
    conjugation!("истереть", 3, "~с~е~еть";
        infinitive: "~с~е~еть";
        non_past_first: "~зо~~у", "~зо~~ём";
        non_past_second: "~зо~~ёшь", "~зо~~ёте";
        non_past_third: "~зо~~ёт", "~зо~~ут";
        past: "~с~ё~", "~с~ё~ла", "~с~ё~ло", "~с~ё~ли";
        imperative: "~зо~~и", "~зо~~ите";
        participle: "~е~ущий~я", "~с~ё~ший", "~с~е~еть", "~с~ё~тый";
        gerund: "~е~я~ь", "~с~е~ев|~с~ё~ши";
    ),
    // Лесить: 3 words (пылесосить, опасить, лесить).
    conjugation!("лесить", 3, "~сить";
        infinitive: "~сить";
        non_past_first: "~шу", "~сим";
        non_past_second: "~сишь", "~сите";
        non_past_third: "~сит", "~сят";
        past: "~сил", "~сила", "~сило", "~сили";
        imperative: "~сь", "~сьте";
        participle: "~сящий", "~сивший", "~симый", "~сенный";
        gerund: "~ся", "~сив|~сивши";
    ),
    // Колебать: 3 words (зыбать, выщипать, колебать).
    conjugation!("колебать", 3, "~ать";
        infinitive: "~ать";
        non_past_first: "~лю", "~лем";
        non_past_second: "~лешь", "~лете";
        non_past_third: "~лет", "~лют";
        past: "~ал", "~ала", "~ало", "~али";
        imperative: "~ли", "~лите";
        participle: "~лющий", "~авший", "~лемый", "~ленный";
        gerund: "~ля", "~ав|~авши";
    ),
    // Выклевать: 3 words (мухлевать, выклевать, штемпелевать).
    conjugation!("выклевать", 3, "~евать";
        infinitive: "~евать";
        non_past_first: "~юю", "~юем";
        non_past_second: "~юешь", "~юете";
        non_past_third: "~юет", "~юют";
        past: "~евал", "~евала", "~евало", "~евали";
        imperative: "~юй", "~юйте";
        participle: "~юющий", "~евавший", "~юемый", "~еванный";
        gerund: "~юя", "~евав|~евавши";
    ),
    // Выкласть: 3 words (повыпасть, выкласть, повыкрасть).
    conjugation!("выкласть", 3, "~сть";
        infinitive: "~сть";
        non_past_first: "~ду", "~дем";
        non_past_second: "~дешь", "~дете";
        non_past_third: "~дет", "~дут";
        past: "~л", "~ла", "~ло", "~ли";
        imperative: "~ди", "~дите";
        participle: "~дущий", "~вший", "~домый", "~денный";
        gerund: "~дя", "~в|~вши";
    ),
    // Оясаться: 3 words (вытесаться, оясаться, лясаться).
    conjugation!("оясаться", 3, "~~аться";
        infinitive: "~~аться";
        non_past_first: "~шу~ь", "~шем~я";
        non_past_second: "~шешь~я", "~шете~ь";
        non_past_third: "~шет~я", "~шут~я";
        past: "~~ался", "~~алась", "~~алось", "~~ались";
        imperative: "~шь~я", "~шьте~ь";
        participle: "~шущий~я", "~~авшийся", "~е~емый", "~~итый";
        gerund: "~ша~ь", "~~авшись";
    ),
    // Растереться: 3 words (растереться, истереться, распереться).
    conjugation!("растереться", 3, "~с~е~еть~я";
        infinitive: "~с~е~еть~я";
        non_past_first: "~зо~~у~ь", "~зо~~ём~я";
        non_past_second: "~зо~~ёшь~я", "~зо~~ёте~ь";
        non_past_third: "~зо~~ёт~я", "~зо~~ут~я";
        past: "~с~ё~~я", "~с~ё~ла~ь", "~с~ё~ло~ь", "~с~ё~ли~ь";
        imperative: "~зо~~и~ь", "~зо~~ите~ь";
        participle: "~с~е~еть~я", "~с~ё~ший~я", "~с~е~еть~я", "~с~е~еть~я";
        gerund: "~с~е~еть~я", "~с~ё~ши~ь";
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
    // Срастись: 3 words (срастись, врастись, растись).
    conjugation!("срастись", 3, "~а~ти~ь";
        infinitive: "~а~ти~ь";
        non_past_first: "~а~ту~ь", "~а~тём~я";
        non_past_second: "~а~тёшь~я", "~а~тёте~ь";
        non_past_third: "~а~тёт~я", "~а~тут~я";
        past: "~о~~я", "~о~ла~ь", "~о~ло~ь", "~о~ли~ь";
        imperative: "~а~ти~ь", "~а~тите~ь";
        participle: "~а~тущий~я", "~о~ший~я", "~а~ти~ь", "~о~~анный";
        gerund: "~а~тя~ь", "~о~ши~ь";
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
    // Харассить: 3 words (харассить, глонассить, лонассить).
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
    // Ятать: 3 words (ятать, рятать, прятать).
    conjugation!("ятать", 3, "~тать";
        infinitive: "~тать";
        non_past_first: "~чу", "~чем";
        non_past_second: "~чешь", "~чете";
        non_past_third: "~чет", "~чут";
        past: "~тал", "~тала", "~тало", "~тали";
        imperative: "~чь", "~чьте";
        participle: "~чущий", "~тавший", "~тимый", "~танный";
        gerund: "~ча", "~тав|~тавши";
    ),
    // Ождать: 3 words (кажать, ождать, возвращать).
    conjugation!("ождать", 3, "~ать";
        infinitive: "~ать";
        non_past_first: "~аю", "~аем";
        non_past_second: "~аешь", "~аете";
        non_past_third: "~ает", "~ают";
        past: "~ал", "~ала", "~ало", "~али";
        imperative: "~ай", "~айте";
        participle: "~ающий", "~авший", "~аемый", "~ённый";
        gerund: "~ая", "~ав|~авши";
    ),
    // Ятаться: 3 words (ятаться, рятаться, прятаться).
    conjugation!("ятаться", 3, "~тать~я";
        infinitive: "~тать~я";
        non_past_first: "~чу~ь", "~чем~я";
        non_past_second: "~чешь~я", "~чете~ь";
        non_past_third: "~чет~я", "~чут~я";
        past: "~тал~я", "~тала~ь", "~тало~ь", "~тали~ь";
        imperative: "~чь~я", "~чьте~ь";
        participle: "~чущий~я", "~тавший~я", "~е~емый", "~~итый";
        gerund: "~ча~ь", "~тавши~ь";
    ),
    // Одолеть: 3 words (олеть, одолеть, ечатлеть).
    conjugation!("одолеть", 3, "~еть";
        infinitive: "~еть";
        non_past_first: "~ею", "~еем";
        non_past_second: "~еешь", "~еете";
        non_past_third: "~еет", "~еют";
        past: "~ел", "~ела", "~ело", "~ели";
        imperative: "~ей", "~ейте";
        participle: "~ящий", "~евший", "~имый", "~ённый";
        gerund: "~я", "~ев|~евши";
    ),
    // Лочься: 3 words (лочься, олочься, толочься).
    conjugation!("лочься", 3, "~очь~я";
        infinitive: "~очь~я";
        non_past_first: "~ку~ь", "~чём~я";
        non_past_second: "~чёшь~я", "~чёте~ь";
        non_past_third: "~чёт~я", "~кут~я";
        past: "~ок~я", "~кла~ь", "~кло~ь", "~кли~ь";
        imperative: "~ки~ь", "~ките~ь";
        participle: "~кущий~я", "~окший~я", "~е~емый", "~~итый";
        gerund: "~я~ь", "~окши~ь";
    ),
    // Быться: 3 words (быться, забыться, добыться).
    conjugation!("быться", 3, "~ыть~я";
        infinitive: "~ыть~я";
        non_past_first: "~уду~ь", "~удем~я";
        non_past_second: "~удешь~я", "~удете~ь";
        non_past_third: "~удет~я", "~удут~я";
        past: "~ыл~я", "~ыла~ь", "~ыло~ь", "~ыли~ь";
        imperative: "~удь~я", "~удьте~ь";
        participle: "~оющий~я", "~ывший~я", "~е~емый", "~~итый";
        gerund: "~оя~ь", "~ывши~ь";
    ),
    // Ежаться: 3 words (бежаться, ежаться, азбежаться).
    conjugation!("ежаться", 3, "~жать~я";
        infinitive: "~жать~я";
        non_past_first: "~гу~ь", "~жим~я";
        non_past_second: "~жишь~я", "~жите~ь";
        non_past_third: "~жит~я", "~гут~я";
        past: "~жал~я", "~жала~ь", "~жало~ь", "~жали~ь";
        imperative: "~ги~ь", "~гите~ь";
        participle: "~жающий~я", "~жавший~я", "~е~емый", "~~итый";
        gerund: "~жая~ь", "~жавши~ь";
    ),
    // Истеться: 3 words (вистеться, свистеться, истеться).
    conjugation!("истеться", 3, "~~теться";
        infinitive: "~~теться";
        non_past_first: "~щу~ь", "~~тимся";
        non_past_second: "~~тишься", "~~титесь";
        non_past_third: "~~тится", "~~тятся";
        past: "~~телся", "~~телась", "~~телось", "~~телись";
        imperative: "~~тись", "~~титесь";
        participle: "~~тящийся", "~~тевшийся", "~е~емый", "~~итый";
        gerund: "~~тясь", "~~тевшись";
    ),
    // Заводить: 3 words (воспроизводить, ереводить, заводить).
    conjugation!("заводить", 3, "~одить";
        infinitive: "~одить";
        non_past_first: "~ожу", "~одим";
        non_past_second: "~одишь", "~одите";
        non_past_third: "~одит", "~одят";
        past: "~одил", "~одила", "~одило", "~одили";
        imperative: "~оди", "~одите";
        participle: "~одящий", "~одивший", "~одимый", "~едённый";
        gerund: "~одя", "~одив|~одивши";
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
    // Ться: 1 words (ться).
    conjugation!("ться", 1, "ть~я";
        infinitive: "ть~я";
        non_past_first: "ю~ь", "ем~я";
        non_past_second: "ешь~я", "ете~ь";
        non_past_third: "ет~я", "ют~я";
        past: "л~я", "ла~ь", "ло~ь", "ли~ь";
        imperative: "й~я", "йте~ь";
        participle: "ющий~я", "вший~я", "~емый", "~нный";
        gerund: "я~ь", "вши~ь";
    ),
    // Чься: 1 words (чься).
    conjugation!("чься", 1, "чь~я";
        infinitive: "чь~я";
        non_past_first: "~ю", "~ем";
        non_past_second: "~ешь", "~ете";
        non_past_third: "жет~я", "~ют";
        past: "~л", "~ла", "гло~ь", "~ли";
        imperative: "~й", "~йте";
        participle: "~ющий", "~вший", "~емый", "~нный";
        gerund: "~я", "~в";
    ),
];
