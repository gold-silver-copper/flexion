//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's animacy says):
/// `nominative` (sg, pl)
/// `genitive` (sg, pl)
/// `dative` (sg, pl)
/// `accusative` (sg, pl)
/// `instrumental` (sg, pl)
/// `prepositional` (sg, pl)
/// `noun_second_case` (part, loc)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal;
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
            ]),
        }
    };
}

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `приём`: 27,703 words.
    Приём,
    /// Like `ролик`: 10,035 words.
    Ролик,
    /// Like `скопление`: 9,054 words.
    Скопление,
    /// Like `склонность`: 7,474 words.
    Склонность,
    /// Like `полнота`: 7,414 words.
    Полнота,
    /// Like `стрелка`: 6,661 words.
    Стрелка,
    /// Like `химия`: 6,582 words.
    Химия,
    /// Like `какао`: 3,417 words.
    Какао,
    /// Like `хлопанье`: 2,658 words.
    Хлопанье,
    /// Like `шоколадница`: 2,595 words.
    Шоколадница,
    /// Like `дорожка`: 2,516 words.
    Дорожка,
    /// Like `сходство`: 2,327 words.
    Сходство,
    /// Like `агнец`: 2,158 words.
    Агнец,
    /// Like `тяга`: 2,131 words.
    Тяга,
    /// Like `медведь`: 1,983 words.
    Медведь,
    /// Like `рожок`: 1,046 words.
    Рожок,
    /// Like `товарищ`: 945 words.
    Товарищ,
    /// Like `выдача`: 690 words.
    Выдача,
    /// Like `змей`: 653 words.
    Змей,
    /// Like `гений`: 647 words.
    Гений,
    /// Like `барашек`: 488 words.
    Барашек,
    /// Like `чайка`: 474 words.
    Чайка,
    /// Like `барин`: 399 words.
    Барин,
    /// Like `накладная`: 342 words.
    Накладная,
    /// Like `ложе`: 327 words.
    Ложе,
    /// Like `зелёный`: 278 words.
    Зелёный,
    /// Like `ключ`: 277 words.
    Ключ,
    /// Like `люлька`: 257 words.
    Люлька,
    /// Like `баня`: 253 words.
    Баня,
    /// Like `олимпиец`: 250 words.
    Олимпиец,
    /// Like `червь`: 229 words.
    Червь,
    /// Like `счёты`: 203 words.
    Счёты,
    /// Like `батарея`: 194 words.
    Батарея,
    /// Like `лососёвые`: 192 words.
    Лососёвые,
    /// Like `русский`: 191 words.
    Русский,
    /// Like `отец`: 179 words.
    Отец,
    /// Like `умелец`: 174 words.
    Умелец,
    /// Like `постав`: 174 words.
    Постав,
    /// Like `фофудья`: 151 words.
    Фофудья,
    /// Like `латы`: 150 words.
    Латы,
    /// Like `молочко`: 148 words.
    Молочко,
    /// Like `мостки`: 144 words.
    Мостки,
    /// Like `заря`: 127 words.
    Заря,
    /// Like `орлёнок`: 124 words.
    Орлёнок,
    /// Like `подворотня`: 120 words.
    Подворотня,
    /// Like `второе`: 118 words.
    Второе,
    /// Like `кормчий`: 115 words.
    Кормчий,
    /// Like `овца`: 114 words.
    Овца,
    /// Like `жнивьё`: 110 words.
    Жнивьё,
    /// Like `камень`: 107 words.
    Камень,
    /// Like `нападки`: 99 words.
    Нападки,
    /// Like `гусёк`: 92 words.
    Гусёк,
    /// Like `ясли`: 86 words.
    Ясли,
    /// Like `спальня`: 85 words.
    Спальня,
    /// Like `ход`: 78 words.
    Ход,
    /// Like `помочь`: 77 words.
    Помочь,
    /// Like `достопримечательность`: 77 words.
    Достопримечательность,
    /// Like `абзац`: 75 words.
    Абзац,
    /// Like `кулички`: 74 words.
    Кулички,
    /// Like `штрафной`: 72 words.
    Штрафной,
    /// Like `мастерская`: 64 words.
    Мастерская,
    /// Like `ядро`: 63 words.
    Ядро,
    /// Like `внучонок`: 57 words.
    Внучонок,
    /// Like `свеча`: 56 words.
    Свеча,
    /// Like `пейзан`: 52 words.
    Пейзан,
    /// Like `дрова`: 52 words.
    Дрова,
    /// Like `съезжая`: 46 words.
    Съезжая,
    /// Like `каракуля`: 45 words.
    Каракуля,
    /// Like `чужое`: 43 words.
    Чужое,
    /// Like `хуище`: 40 words.
    Хуище,
    /// Like `мощи`: 38 words.
    Мощи,
    /// Like `делёж`: 36 words.
    Делёж,
    /// Like `положняк`: 35 words.
    Положняк,
    /// Like `дрязги`: 34 words.
    Дрязги,
    /// Like `горючее`: 33 words.
    Горючее,
    /// Like `ладья`: 30 words.
    Ладья,
    /// Like `верея`: 29 words.
    Верея,
    /// Like `кешью`: 29 words.
    Кешью,
    /// Like `порубка`: 28 words.
    Порубка,
    /// Like `шов`: 27 words.
    Шов,
    /// Like `егерь`: 27 words.
    Егерь,
    /// Like `деревня`: 27 words.
    Деревня,
    /// Like `пол-литра`: 26 words.
    Поллитра,
    /// Like `щит`: 24 words.
    Щит,
    /// Like `голец`: 24 words.
    Голец,
    /// Like `домашние`: 24 words.
    Домашние,
    /// Like `звезда`: 23 words.
    Звезда,
    /// Like `член`: 23 words.
    Член,
    /// Like `лицо`: 23 words.
    Лицо,
    /// Like `маракуйя`: 22 words.
    Маракуйя,
    /// Like `деньги`: 21 words.
    Деньги,
    /// Like `устье`: 21 words.
    Устье,
    /// Like `ребро`: 21 words.
    Ребро,
    /// Like `козёл`: 20 words.
    Козёл,
    /// Like `хуй`: 20 words.
    Хуй,
    /// Like `клин`: 19 words.
    Клин,
    /// Like `сенцы`: 19 words.
    Сенцы,
    /// Like `нос`: 18 words.
    Нос,
    /// Like `ископаемое`: 18 words.
    Ископаемое,
    /// Like `боёк`: 18 words.
    Боёк,
    /// Like `детище`: 17 words.
    Детище,
    /// Like `очко`: 16 words.
    Очко,
    /// Like `анкара`: 16 words.
    Анкара,
    /// Like `зло`: 15 words.
    Зло,
    /// Like `бойня`: 15 words.
    Бойня,
    /// Like `пень`: 15 words.
    Пень,
    /// Like `побои`: 15 words.
    Побои,
    /// Like `перо`: 14 words.
    Перо,
    /// Like `ебала`: 14 words.
    Ебала,
    /// Like `губоцветные`: 14 words.
    Губоцветные,
    /// Like `сердце`: 13 words.
    Сердце,
    /// Like `котёночек`: 13 words.
    Котёночек,
    /// Like `меринос`: 13 words.
    Меринос,
    /// Like `гнездо`: 12 words.
    Гнездо,
    /// Like `фекалии`: 12 words.
    Фекалии,
    /// Like `кольцо`: 12 words.
    Кольцо,
    /// Like `рыльце`: 12 words.
    Рыльце,
    /// Like `валенок`: 11 words.
    Валенок,
    /// Like `кипяток`: 11 words.
    Кипяток,
    /// Like `счёт`: 11 words.
    Счёт,
    /// Like `прения`: 11 words.
    Прения,
    /// Like `тельце`: 11 words.
    Тельце,
    /// Like `племя`: 10 words.
    Племя,
    /// Like `глаз`: 10 words.
    Глаз,
    /// Like `усадьба`: 10 words.
    Усадьба,
    /// Like `хлопья`: 10 words.
    Хлопья,
    /// Like `коленце`: 10 words.
    Коленце,
    /// Like `дом`: 9 words.
    Дом,
    /// Like `перешеек`: 9 words.
    Перешеек,
    /// Like `плечо`: 8 words.
    Плечо,
    /// Like `горе`: 8 words.
    Горе,
    /// Like `кукла`: 8 words.
    Кукла,
    /// Like `покой`: 8 words.
    Покой,
    /// Like `герц`: 8 words.
    Герц,
    /// Like `калашников`: 8 words.
    Калашников,
    /// Like `остриё`: 8 words.
    Остриё,
    /// Like `плюхино`: 8 words.
    Плюхино,
    /// Like `пол-юбки`: 8 words.
    Полюбки,
    /// Like `ложь`: 7 words.
    Ложь,
    /// Like `дух`: 7 words.
    Дух,
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
    /// Like `чужой`: 7 words.
    Чужой,
    /// Like `полгода`: 7 words.
    Полгода,
    /// Like `пол-июля`: 7 words.
    Полиюля,
    /// Like `пол-игры`: 7 words.
    Полигры,
    /// Like `бой`: 6 words.
    Бой,
    /// Like `кто`: 6 words.
    Кто,
    /// Like `млекопитающее`: 6 words.
    Млекопитающее,
    /// Like `татарин`: 6 words.
    Татарин,
    /// Like `осётр`: 6 words.
    Осётр,
    /// Like `чукча`: 6 words.
    Чукча,
    /// Like `метёлка`: 6 words.
    Метёлка,
    /// Like `макаронник`: 6 words.
    Макаронник,
    /// Like `ебеня`: 6 words.
    Ебеня,
    /// Like `скло`: 6 words.
    Скло,
    /// Like `исайя`: 6 words.
    Исайя,
    /// Like `пол-лета`: 6 words.
    Поллета,
    /// Like `круг`: 5 words.
    Круг,
    /// Like `огонь`: 5 words.
    Огонь,
    /// Like `кухня`: 5 words.
    Кухня,
    /// Like `дичь`: 5 words.
    Дичь,
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
    /// Like `получеловек`: 5 words.
    Получеловек,
    /// Like `ботиночек`: 5 words.
    Ботиночек,
    /// Like `чернорубашечник`: 5 words.
    Чернорубашечник,
    /// Like `полкроны`: 5 words.
    Полкроны,
    /// Like `тиса`: 5 words.
    Тиса,
    /// Like `копьё`: 4 words.
    Копьё,
    /// Like `опий`: 4 words.
    Опий,
    /// Like `судно`: 4 words.
    Судно,
    /// Like `улей`: 4 words.
    Улей,
    /// Like `учащийся`: 4 words.
    Учащийся,
    /// Like `бугай`: 4 words.
    Бугай,
    /// Like `капли`: 4 words.
    Капли,
    /// Like `пресмыкающееся`: 4 words.
    Пресмыкающееся,
    /// Like `ничья`: 4 words.
    Ничья,
    /// Like `яйки`: 4 words.
    Яйки,
    /// Like `горошек`: 4 words.
    Горошек,
    /// Like `сынуля`: 4 words.
    Сынуля,
    /// Like `дитятко`: 4 words.
    Дитятко,
    /// Like `земля`: 3 words.
    Земля,
    /// Like `путь`: 3 words.
    Путь,
    /// Like `угол`: 3 words.
    Угол,
    /// Like `сколько`: 3 words.
    Сколько,
    /// Like `чудо`: 3 words.
    Чудо,
    /// Like `средостениё`: 3 words.
    Средостениё,
    /// Like `смерть`: 3 words.
    Смерть,
    /// Like `кисель`: 3 words.
    Кисель,
    /// Like `епитимья`: 3 words.
    Епитимья,
    /// Like `дверца`: 3 words.
    Дверца,
    /// Like `солнышко`: 3 words.
    Солнышко,
    /// Like `морячка`: 3 words.
    Морячка,
    /// Like `пол-леса`: 3 words.
    Поллеса,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 204] = [
        Declension::Приём,
        Declension::Ролик,
        Declension::Скопление,
        Declension::Склонность,
        Declension::Полнота,
        Declension::Стрелка,
        Declension::Химия,
        Declension::Какао,
        Declension::Хлопанье,
        Declension::Шоколадница,
        Declension::Дорожка,
        Declension::Сходство,
        Declension::Агнец,
        Declension::Тяга,
        Declension::Медведь,
        Declension::Рожок,
        Declension::Товарищ,
        Declension::Выдача,
        Declension::Змей,
        Declension::Гений,
        Declension::Барашек,
        Declension::Чайка,
        Declension::Барин,
        Declension::Накладная,
        Declension::Ложе,
        Declension::Зелёный,
        Declension::Ключ,
        Declension::Люлька,
        Declension::Баня,
        Declension::Олимпиец,
        Declension::Червь,
        Declension::Счёты,
        Declension::Батарея,
        Declension::Лососёвые,
        Declension::Русский,
        Declension::Отец,
        Declension::Умелец,
        Declension::Постав,
        Declension::Фофудья,
        Declension::Латы,
        Declension::Молочко,
        Declension::Мостки,
        Declension::Заря,
        Declension::Орлёнок,
        Declension::Подворотня,
        Declension::Второе,
        Declension::Кормчий,
        Declension::Овца,
        Declension::Жнивьё,
        Declension::Камень,
        Declension::Нападки,
        Declension::Гусёк,
        Declension::Ясли,
        Declension::Спальня,
        Declension::Ход,
        Declension::Помочь,
        Declension::Достопримечательность,
        Declension::Абзац,
        Declension::Кулички,
        Declension::Штрафной,
        Declension::Мастерская,
        Declension::Ядро,
        Declension::Внучонок,
        Declension::Свеча,
        Declension::Пейзан,
        Declension::Дрова,
        Declension::Съезжая,
        Declension::Каракуля,
        Declension::Чужое,
        Declension::Хуище,
        Declension::Мощи,
        Declension::Делёж,
        Declension::Положняк,
        Declension::Дрязги,
        Declension::Горючее,
        Declension::Ладья,
        Declension::Верея,
        Declension::Кешью,
        Declension::Порубка,
        Declension::Шов,
        Declension::Егерь,
        Declension::Деревня,
        Declension::Поллитра,
        Declension::Щит,
        Declension::Голец,
        Declension::Домашние,
        Declension::Звезда,
        Declension::Член,
        Declension::Лицо,
        Declension::Маракуйя,
        Declension::Деньги,
        Declension::Устье,
        Declension::Ребро,
        Declension::Козёл,
        Declension::Хуй,
        Declension::Клин,
        Declension::Сенцы,
        Declension::Нос,
        Declension::Ископаемое,
        Declension::Боёк,
        Declension::Детище,
        Declension::Очко,
        Declension::Анкара,
        Declension::Зло,
        Declension::Бойня,
        Declension::Пень,
        Declension::Побои,
        Declension::Перо,
        Declension::Ебала,
        Declension::Губоцветные,
        Declension::Сердце,
        Declension::Котёночек,
        Declension::Меринос,
        Declension::Гнездо,
        Declension::Фекалии,
        Declension::Кольцо,
        Declension::Рыльце,
        Declension::Валенок,
        Declension::Кипяток,
        Declension::Счёт,
        Declension::Прения,
        Declension::Тельце,
        Declension::Племя,
        Declension::Глаз,
        Declension::Усадьба,
        Declension::Хлопья,
        Declension::Коленце,
        Declension::Дом,
        Declension::Перешеек,
        Declension::Плечо,
        Declension::Горе,
        Declension::Кукла,
        Declension::Покой,
        Declension::Герц,
        Declension::Калашников,
        Declension::Остриё,
        Declension::Плюхино,
        Declension::Полюбки,
        Declension::Ложь,
        Declension::Дух,
        Declension::Лапоть,
        Declension::Череда,
        Declension::Сиверко,
        Declension::Передняя,
        Declension::Семя,
        Declension::Стекло,
        Declension::Заём,
        Declension::Весна,
        Declension::Чужой,
        Declension::Полгода,
        Declension::Полиюля,
        Declension::Полигры,
        Declension::Бой,
        Declension::Кто,
        Declension::Млекопитающее,
        Declension::Татарин,
        Declension::Осётр,
        Declension::Чукча,
        Declension::Метёлка,
        Declension::Макаронник,
        Declension::Ебеня,
        Declension::Скло,
        Declension::Исайя,
        Declension::Поллета,
        Declension::Круг,
        Declension::Огонь,
        Declension::Кухня,
        Declension::Дичь,
        Declension::Боец,
        Declension::Козлы,
        Declension::Репей,
        Declension::Хозяин,
        Declension::Бега,
        Declension::Получеловек,
        Declension::Ботиночек,
        Declension::Чернорубашечник,
        Declension::Полкроны,
        Declension::Тиса,
        Declension::Копьё,
        Declension::Опий,
        Declension::Судно,
        Declension::Улей,
        Declension::Учащийся,
        Declension::Бугай,
        Declension::Капли,
        Declension::Пресмыкающееся,
        Declension::Ничья,
        Declension::Яйки,
        Declension::Горошек,
        Declension::Сынуля,
        Declension::Дитятко,
        Declension::Земля,
        Declension::Путь,
        Declension::Угол,
        Declension::Сколько,
        Declension::Чудо,
        Declension::Средостениё,
        Declension::Смерть,
        Declension::Кисель,
        Declension::Епитимья,
        Declension::Дверца,
        Declension::Солнышко,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 204] = [
    // Приём: 27,703 words (указ, зонт, кот, узус, скот, мэр).
    declension!("приём", 27703, "~";
        nominative: "~", "~ы";
        genitive: "~а", "~ов";
        dative: "~у", "~ам";
        accusative: referred, referred;
        instrumental: "~ом", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~а", "~е";
    ),
    // Ролик: 10,035 words (зонтик, бортник, бочаг, бредник, язык, гулаг).
    declension!("ролик", 10035, "~";
        nominative: "~", "~и";
        genitive: "~а", "~ов";
        dative: "~у", "~ам";
        accusative: referred, referred;
        instrumental: "~ом", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~а", "~е";
    ),
    // Скопление: 9,054 words (произношение, лезвие, распятие, деревенение, лесонасаждение, ведение).
    declension!("скопление", 9054, "~е";
        nominative: "~е", "~я";
        genitive: "~я", "~й";
        dative: "~ю", "~ям";
        accusative: referred, referred;
        instrumental: "~ем", "~ями";
        prepositional: "~и", "~ях";
        noun_second_case: "~я", "~и";
    ),
    // Склонность: 7,474 words (знать, весь, гласность, деревянность, дверь, ртуть).
    declension!("склонность", 7474, "~ь";
        nominative: "~ь", "~и";
        genitive: "~и", "~ей";
        dative: "~и", "~ям";
        accusative: referred, referred;
        instrumental: "~ью", "~ями";
        prepositional: "~и", "~ях";
        noun_second_case: "~и", "~и";
    ),
    // Полнота: 7,414 words (буява, вата, работа, камера, липа, сера).
    declension!("полнота", 7414, "~а";
        nominative: "~а", "~ы";
        genitive: "~ы", "~";
        dative: "~е", "~ам";
        accusative: "~у", referred;
        instrumental: "~ой|~ою", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~ы", "~е";
    ),
    // Стрелка: 6,661 words (водка, цыпка, жилка, деревянка, лесопилка, доска).
    declension!("стрелка", 6661, "~~а";
        nominative: "~~а", "~~и";
        genitive: "~~и", "~о~";
        dative: "~~е", "~~ам";
        accusative: "~~у", referred;
        instrumental: "~~ой|~~ою", "~~ами";
        prepositional: "~~е", "~~ах";
        noun_second_case: "~~и", "~~е";
    ),
    // Химия: 6,582 words (анархия, лесохимия, библия, биология, академия, армия).
    declension!("химия", 6582, "~я";
        nominative: "~я", "~и";
        genitive: "~и", "~й";
        dative: "~и", "~ям";
        accusative: "~ю", referred;
        instrumental: "~ей|~ею", "~ями";
        prepositional: "~и", "~ях";
        noun_second_case: "~и", "~и";
    ),
    // Какао: 3,417 words (пальто, брачущиеся, брачующиеся, адажио, какао, полпути).
    declension!("какао", 3417, "~";
        nominative: "~", "~";
        genitive: "~", "~";
        dative: "~", "~";
        accusative: referred, referred;
        instrumental: "~", "~";
        prepositional: "~", "~";
        noun_second_case: "~", "~";
    ),
    // Хлопанье: 2,658 words (запястье, воскресенье, варенье, воспитанье, отчаянье, уединенье).
    declension!("хлопанье", 2658, "~ье";
        nominative: "~ье", "~ья";
        genitive: "~ья", "~ий";
        dative: "~ью", "~ьям";
        accusative: referred, referred;
        instrumental: "~ьем", "~ьями";
        prepositional: "~ье", "~ьях";
        noun_second_case: "~ья", "~ье";
    ),
    // Шоколадница: 2,595 words (былица, масленица, царица, лисица, птица, гусеница).
    declension!("шоколадница", 2595, "~а";
        nominative: "~а", "~ы";
        genitive: "~ы", "~";
        dative: "~е", "~ам";
        accusative: "~у", referred;
        instrumental: "~ей|~ею", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~ы", "~е";
    ),
    // Дорожка: 2,516 words (кошка, быличка, палочка, ручка, мошка, бабочка).
    declension!("дорожка", 2516, "~~а";
        nominative: "~~а", "~~и";
        genitive: "~~и", "~е~";
        dative: "~~е", "~~ам";
        accusative: "~~у", referred;
        instrumental: "~~ой|~~ою", "~~ами";
        prepositional: "~~е", "~~ах";
        noun_second_case: "~~и", "~~е";
    ),
    // Сходство: 2,327 words (буево, бёрдо, око, лето, жало, горло).
    declension!("сходство", 2327, "~о";
        nominative: "~о", "~а";
        genitive: "~а", "~";
        dative: "~у", "~ам";
        accusative: referred, referred;
        instrumental: "~ом", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~а", "~е";
    ),
    // Агнец: 2,158 words (левантинец, древоточец, марганец, самозванец, разночинец, ньюйоркец).
    declension!("агнец", 2158, "~е~";
        nominative: "~е~", "~~ы";
        genitive: "~~а", "~~ев";
        dative: "~~у", "~~ам";
        accusative: referred, referred;
        instrumental: "~~ем", "~~ами";
        prepositional: "~~е", "~~ах";
        noun_second_case: "~~а", "~~е";
    ),
    // Тяга: 2,131 words (собака, мука, книга, керамика, вага, республика).
    declension!("тяга", 2131, "~а";
        nominative: "~а", "~и";
        genitive: "~и", "~";
        dative: "~е", "~ам";
        accusative: "~у", referred;
        instrumental: "~ой|~ою", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~и", "~е";
    ),
    // Медведь: 1,983 words (сшиватель, апрель, июнь, июль, олень, автомобиль).
    declension!("медведь", 1983, "~ь";
        nominative: "~ь", "~и";
        genitive: "~я", "~ей";
        dative: "~ю", "~ям";
        accusative: referred, referred;
        instrumental: "~ем", "~ями";
        prepositional: "~е", "~ях";
        noun_second_case: "~я", "~е";
    ),
    // Рожок: 1,046 words (щенок, бесёнок, лесок, замок, носок, подросток).
    declension!("рожок", 1046, "~о~";
        nominative: "~о~", "~~и";
        genitive: "~~а", "~~ов";
        dative: "~~у", "~~ам";
        accusative: referred, referred;
        instrumental: "~~ом", "~~ами";
        prepositional: "~~е", "~~ах";
        noun_second_case: "~~а", "~~е";
    ),
    // Товарищ: 945 words (харч, туш, треш, раж, товарищ, дервиш).
    declension!("товарищ", 945, "~";
        nominative: "~", "~и";
        genitive: "~а", "~ей";
        dative: "~у", "~ам";
        accusative: referred, referred;
        instrumental: "~ем", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~а", "~е";
    ),
    // Выдача: 690 words (ладоша, кожа, чаша, мульча, грыжа, дикуша).
    declension!("выдача", 690, "~а";
        nominative: "~а", "~и";
        genitive: "~и", "~";
        dative: "~е", "~ам";
        accusative: "~у", referred;
        instrumental: "~ей|~ею", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~и", "~е";
    ),
    // Змей: 653 words (май, гей, вертухай, елей, герой, случай).
    declension!("змей", 653, "~й";
        nominative: "~й", "~и";
        genitive: "~я", "~ев";
        dative: "~ю", "~ям";
        accusative: referred, referred;
        instrumental: "~ем", "~ями";
        prepositional: "~е", "~ях";
        noun_second_case: "~я", "~е";
    ),
    // Гений: 647 words (калий, меркурий, гений, бериллий, гелий, литий).
    declension!("гений", 647, "~й";
        nominative: "~й", "~и";
        genitive: "~я", "~ев";
        dative: "~ю", "~ям";
        accusative: referred, referred;
        instrumental: "~ем", "~ями";
        prepositional: "~и", "~ях";
        noun_second_case: "~я", "~и";
    ),
    // Барашек: 488 words (излишек, камешек, гривенничек, кусочек, орешек, камушек).
    declension!("барашек", 488, "~е~";
        nominative: "~е~", "~~и";
        genitive: "~~а", "~~ов";
        dative: "~~у", "~~ам";
        accusative: referred, referred;
        instrumental: "~~ом", "~~ами";
        prepositional: "~~е", "~~ах";
        noun_second_case: "~~а", "~~е";
    ),
    // Чайка: 474 words (копейка, скамейка, перестройка, ищейка, лайка, австрийка).
    declension!("чайка", 474, "~й~а";
        nominative: "~й~а", "~й~и";
        genitive: "~й~и", "~е~";
        dative: "~й~е", "~й~ам";
        accusative: "~й~у", referred;
        instrumental: "~й~ой|~й~ою", "~й~ами";
        prepositional: "~й~е", "~й~ах";
        noun_second_case: "~й~и", "~й~е";
    ),
    // Барин: 399 words (господин, англичанин, россиянин, опочанин, гражданин, новоангличанин).
    declension!("барин", 399, "~ин";
        nominative: "~ин", "~е";
        genitive: "~ина", "~";
        dative: "~ину", "~ам";
        accusative: referred, referred;
        instrumental: "~ином", "~ами";
        prepositional: "~ине", "~ах";
        noun_second_case: "~ина", "~ине";
    ),
    // Накладная: 342 words (деревообделочная, гостиная, ванная, кладовая, уборная, борзая).
    declension!("накладная", 342, "~ая";
        nominative: "~ая", "~ые";
        genitive: "~ой", "~ых";
        dative: "~ой", "~ым";
        accusative: "~ую", referred;
        instrumental: "~ой|~ою", "~ыми";
        prepositional: "~ой", "~ых";
        noun_second_case: "~ой", "~ой";
    ),
    // Ложе: 327 words (солнце, вече, училище, кладбище, ложе, пристанище).
    declension!("ложе", 327, "~е";
        nominative: "~е", "~а";
        genitive: "~а", "~";
        dative: "~у", "~ам";
        accusative: referred, referred;
        instrumental: "~ем", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~а", "~е";
    ),
    // Зелёный: 278 words (белый, чёрный, гласный, беспризорный, коридорный, левый).
    declension!("зелёный", 278, "~ый";
        nominative: "~ый", "~ые";
        genitive: "~ого", "~ых";
        dative: "~ому", "~ым";
        accusative: referred, referred;
        instrumental: "~ым", "~ыми";
        prepositional: "~ом", "~ых";
        noun_second_case: "~ого", "~ом";
    ),
    // Ключ: 277 words (борщ, карандаш, врач, сыч, нож, меч).
    declension!("ключ", 277, "~";
        nominative: "~", "~и";
        genitive: "~а", "~ей";
        dative: "~у", "~ам";
        accusative: referred, referred;
        instrumental: "~ом", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~а", "~е";
    ),
    // Люлька: 257 words (пенька, пиписька, кухонька, батенька, калька, свистулька).
    declension!("люлька", 257, "~ь~а";
        nominative: "~ь~а", "~ь~и";
        genitive: "~ь~и", "~е~";
        dative: "~ь~е", "~ь~ам";
        accusative: "~ь~у", referred;
        instrumental: "~ь~ой|~ь~ою", "~ь~ами";
        prepositional: "~ь~е", "~ь~ах";
        noun_second_case: "~ь~и", "~ь~е";
    ),
    // Баня: 253 words (братыня, неделя, буря, няня, княгиня, тётя).
    declension!("баня", 253, "~я";
        nominative: "~я", "~и";
        genitive: "~и", "~ь";
        dative: "~е", "~ям";
        accusative: "~ю", referred;
        instrumental: "~ей|~ею", "~ями";
        prepositional: "~е", "~ях";
        noun_second_case: "~и", "~е";
    ),
    // Олимпиец: 250 words (австриец, австралиец, ариец, белогвардеец, китаец, византиец).
    declension!("олимпиец", 250, "~е~";
        nominative: "~е~", "~й~ы";
        genitive: "~й~а", "~й~ев";
        dative: "~й~у", "~й~ам";
        accusative: referred, referred;
        instrumental: "~й~ем", "~й~ами";
        prepositional: "~й~е", "~й~ах";
        noun_second_case: "~й~а", "~й~е";
    ),
    // Червь: 229 words (царь, рубль, кремль, дождь, январь, февраль).
    declension!("червь", 229, "~ь";
        nominative: "~ь", "~и";
        genitive: "~я", "~ей";
        dative: "~ю", "~ям";
        accusative: referred, referred;
        instrumental: "~ём", "~ями";
        prepositional: "~е", "~ях";
        noun_second_case: "~я", "~е";
    ),
    // Счёты: 203 words (часы, роды, древоеды, щипцы, боеприпасы, французы).
    declension!("счёты", 203, "~ы";
        nominative: "~", "~ы";
        genitive: "~а", "~ов";
        dative: "~у", "~ам";
        accusative: referred, referred;
        instrumental: "~ом", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~а", "~е";
    ),
    // Батарея: 194 words (статуя, стая, шея, идея, затея, фея).
    declension!("батарея", 194, "~я";
        nominative: "~я", "~и";
        genitive: "~и", "~й";
        dative: "~е", "~ям";
        accusative: "~ю", referred;
        instrumental: "~ей|~ею", "~ями";
        prepositional: "~е", "~ях";
        noun_second_case: "~и", "~е";
    ),
    // Лососёвые: 192 words (данные, чаевые, присные, месячные, наличные, родные).
    declension!("лососёвые", 192, "~е";
        nominative: "~е", "~е";
        genitive: "~я", "~х";
        dative: "~ю", "~м";
        accusative: referred, referred;
        instrumental: "~ем", "~ми";
        prepositional: "~и", "~х";
        noun_second_case: "~я", "~и";
    ),
    // Русский: 191 words (русский, сербскохорватский, английский, норвежский, китайский, немецкий).
    declension!("русский", 191, "~ий";
        nominative: "~ий", "~ие";
        genitive: "~ого", "~их";
        dative: "~ому", "~им";
        accusative: referred, referred;
        instrumental: "~им", "~ими";
        prepositional: "~ом", "~их";
        noun_second_case: "~ого", "~ом";
    ),
    // Отец: 179 words (отец, свинец, ветер, купец, творец, дворец).
    declension!("отец", 179, "~е~";
        nominative: "~е~", "~~ы";
        genitive: "~~а", "~~ов";
        dative: "~~у", "~~ам";
        accusative: referred, referred;
        instrumental: "~~ом", "~~ами";
        prepositional: "~~е", "~~ах";
        noun_second_case: "~~а", "~~е";
    ),
    // Умелец: 174 words (палец, комсомолец, земледелец, бразилец, владелец, анголец).
    declension!("умелец", 174, "~е~";
        nominative: "~е~", "~ь~ы";
        genitive: "~ь~а", "~ь~ев";
        dative: "~ь~у", "~ь~ам";
        accusative: referred, referred;
        instrumental: "~ь~ем", "~ь~ами";
        prepositional: "~ь~е", "~ь~ах";
        noun_second_case: "~ь~а", "~ь~е";
    ),
    // Постав: 174 words (город, пояс, адрес, бок, вечер, номер).
    declension!("постав", 174, "~";
        nominative: "~", "~а";
        genitive: "~а", "~ов";
        dative: "~у", "~ам";
        accusative: referred, referred;
        instrumental: "~ом", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~а", "~е";
    ),
    // Фофудья: 151 words (гостья, авиаэскадрилья, эскадрилья, келья, тортилья, колдунья).
    declension!("фофудья", 151, "~ья";
        nominative: "~ья", "~ьи";
        genitive: "~ьи", "~ий";
        dative: "~ье", "~ьям";
        accusative: "~ью", referred;
        instrumental: "~ьей|~ьею", "~ьями";
        prepositional: "~ье", "~ьях";
        noun_second_case: "~ьи", "~ье";
    ),
    // Латы: 150 words (цимбалы, каникулы, ножницы, шахматы, именины, бакенбарды).
    declension!("латы", 150, "~ы";
        nominative: "~а", "~ы";
        genitive: "~ы", "~";
        dative: "~е", "~ам";
        accusative: "~у", referred;
        instrumental: "~ой", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~ы", "~е";
    ),
    // Молочко: 148 words (семечко, колено, батько, яичко, окошко, зёрнышко).
    declension!("молочко", 148, "~~о";
        nominative: "~~о", "~~и";
        genitive: "~~а", "~е~";
        dative: "~~у", "~~ам";
        accusative: referred, referred;
        instrumental: "~~ом", "~~ами";
        prepositional: "~~е", "~~ах";
        noun_second_case: "~~а", "~~е";
    ),
    // Мостки: 144 words (древпластики, духи, щипчики, мозги, падонки, очки).
    declension!("мостки", 144, "~и";
        nominative: "~и", "~и";
        genitive: "~и", "~ов";
        dative: "~и", "~ам";
        accusative: referred, referred;
        instrumental: "~и", "~ами";
        prepositional: "~и", "~ах";
        noun_second_case: "~и", "~и";
    ),
    // Заря: 127 words (ступня, стезя, простыня, тля, хуйня, броня).
    declension!("заря", 127, "~я";
        nominative: "~я", "~и";
        genitive: "~и", "~ей";
        dative: "~е", "~ям";
        accusative: "~ю", referred;
        instrumental: "~ёй|~ёю", "~ями";
        prepositional: "~е", "~ях";
        noun_second_case: "~и", "~е";
    ),
    // Орлёнок: 124 words (телёнок, октябрёнок, поросёнок, жеребёнок, ягнёнок, козлёнок).
    declension!("орлёнок", 124, "~ёнок";
        nominative: "~ёнок", "~ята";
        genitive: "~ёнка", "~ят";
        dative: "~ёнку", "~ятам";
        accusative: referred, referred;
        instrumental: "~ёнком", "~ятами";
        prepositional: "~ёнке", "~ятах";
        noun_second_case: "~ёнка", "~ёнке";
    ),
    // Подворотня: 120 words (вишня, песня, бушня, пригоршня, башня, пекарня).
    declension!("подворотня", 120, "~~я";
        nominative: "~~я", "~~и";
        genitive: "~~и", "~е~";
        dative: "~~е", "~~ям";
        accusative: "~~ю", referred;
        instrumental: "~~ей|~~ею", "~~ями";
        prepositional: "~~е", "~~ях";
        noun_second_case: "~~и", "~~е";
    ),
    // Второе: 118 words (былое, существительное, числительное, прилагательное, мороженое, приданое).
    declension!("второе", 118, "~ое";
        nominative: "~ое", "~ые";
        genitive: "~ого", "~ых";
        dative: "~ому", "~ым";
        accusative: referred, referred;
        instrumental: "~ым", "~ыми";
        prepositional: "~ом", "~ых";
        noun_second_case: "~ого", "~ом";
    ),
    // Кормчий: 115 words (лесничий, рабочий, посторонний, нищий, военнослужащий, домашний).
    declension!("кормчий", 115, "~ий";
        nominative: "~ий", "~ие";
        genitive: "~его", "~их";
        dative: "~ему", "~им";
        accusative: referred, referred;
        instrumental: "~им", "~ими";
        prepositional: "~ем", "~их";
        noun_second_case: "~его", "~ем";
    ),
    // Овца: 114 words (овца, сосна, домна, царевна, гривна, кошма).
    declension!("овца", 114, "~~а";
        nominative: "~~а", "~~ы";
        genitive: "~~ы", "~е~";
        dative: "~~е", "~~ам";
        accusative: "~~у", referred;
        instrumental: "~~ой|~~ою", "~~ами";
        prepositional: "~~е", "~~ах";
        noun_second_case: "~~ы", "~~е";
    ),
    // Жнивьё: 110 words (питьё, ружьё, битьё, враньё, сырьё, жильё).
    declension!("жнивьё", 110, "~ьё";
        nominative: "~ьё", "~ья";
        genitive: "~ья", "~ей";
        dative: "~ью", "~ьям";
        accusative: referred, referred;
        instrumental: "~ьём", "~ьями";
        prepositional: "~ье", "~ьях";
        noun_second_case: "~ья", "~ье";
    ),
    // Камень: 107 words (бредень, оборотень, камень, парень, полдень, кашель).
    declension!("камень", 107, "~е~ь";
        nominative: "~е~ь", "~~и";
        genitive: "~~я", "~~ей";
        dative: "~~ю", "~~ям";
        accusative: referred, referred;
        instrumental: "~~ем", "~~ями";
        prepositional: "~~е", "~~ях";
        noun_second_case: "~~я", "~~е";
    ),
    // Нападки: 99 words (лесопосадки, плавки, сутки, сливки, колготки, поминки).
    declension!("нападки", 99, "~~и";
        nominative: "~~а", "~~и";
        genitive: "~~и", "~о~";
        dative: "~~е", "~~ам";
        accusative: "~~у", referred;
        instrumental: "~~ой", "~~ами";
        prepositional: "~~е", "~~ах";
        noun_second_case: "~~и", "~~е";
    ),
    // Гусёк: 92 words (лёд, мотылёк, лён, хорёк, кошелёк, королёк).
    declension!("гусёк", 92, "~ё~";
        nominative: "~ё~", "~ь~и";
        genitive: "~ь~а", "~ь~ов";
        dative: "~ь~у", "~ь~ам";
        accusative: referred, referred;
        instrumental: "~ь~ом", "~ь~ами";
        prepositional: "~ь~е", "~ь~ах";
        noun_second_case: "~ь~а", "~ь~е";
    ),
    // Ясли: 86 words (люди, кости, черви, внутренности, ясли, новости).
    declension!("ясли", 86, "~и";
        nominative: "~и", "~и";
        genitive: "~и", "~ей";
        dative: "~и", "~ям";
        accusative: referred, referred;
        instrumental: "~и", "~ями";
        prepositional: "~и", "~ях";
        noun_second_case: "~и", "~и";
    ),
    // Спальня: 85 words (лесопильня, спальня, читальня, колокольня, коптильня, наковальня).
    declension!("спальня", 85, "~ь~я";
        nominative: "~ь~я", "~ь~и";
        genitive: "~ь~и", "~е~";
        dative: "~ь~е", "~ь~ям";
        accusative: "~ь~ю", referred;
        instrumental: "~ь~ей|~ь~ею", "~ь~ями";
        prepositional: "~ь~е", "~ь~ях";
        noun_second_case: "~ь~и", "~ь~е";
    ),
    // Ход: 78 words (укроп, самогон, град, народ, аппетит, яд).
    declension!("ход", 78, "~";
        nominative: "~", "~ы";
        genitive: "~а", "~ов";
        dative: "~у", "~ам";
        accusative: referred, referred;
        instrumental: "~ом", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~у", "~е";
    ),
    // Помочь: 77 words (ночь, мочь, тушь, помочь, печь, речь).
    declension!("помочь", 77, "~ь";
        nominative: "~ь", "~и";
        genitive: "~и", "~ей";
        dative: "~и", "~ам";
        accusative: referred, referred;
        instrumental: "~ью", "~ами";
        prepositional: "~и", "~ах";
        noun_second_case: "~и", "~и";
    ),
    // Достопримечательность: 77 words (лошадь, дочь, мать, сельдь, свекровь, достопримечательность).
    declension!("достопримечательность", 77, "~ь";
        nominative: "~ь", "~и";
        genitive: "~и", "~ей";
        dative: "~и", "~ям";
        accusative: "~ь", referred;
        instrumental: "~ью", "~ями";
        prepositional: "~и", "~ях";
        noun_second_case: "~и", "~и";
    ),
    // Абзац: 75 words (месяц, принц, абзац, кварц, шприц, плац).
    declension!("абзац", 75, "~";
        nominative: "~", "~ы";
        genitive: "~а", "~ев";
        dative: "~у", "~ам";
        accusative: referred, referred;
        instrumental: "~ем", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~а", "~е";
    ),
    // Кулички: 74 words (сумерки, бачки, шашки, кусачки, детишки, корточки).
    declension!("кулички", 74, "~~и";
        nominative: "~~а", "~~и";
        genitive: "~~и", "~е~";
        dative: "~~е", "~~ам";
        accusative: "~~у", referred;
        instrumental: "~~ой", "~~ами";
        prepositional: "~~е", "~~ах";
        noun_second_case: "~~и", "~~е";
    ),
    // Штрафной: 72 words (больной, голубой, часовой, золотой, вестовой, немой).
    declension!("штрафной", 72, "~ой";
        nominative: "~ой", "~ые";
        genitive: "~ого", "~ых";
        dative: "~ому", "~ым";
        accusative: referred, referred;
        instrumental: "~ым", "~ыми";
        prepositional: "~ом", "~ых";
        noun_second_case: "~ого", "~ом";
    ),
    // Мастерская: 64 words (детская, мастерская, мужская, русская, кондитерская, парикмахерская).
    declension!("мастерская", 64, "~ая";
        nominative: "~ая", "~ие";
        genitive: "~ой", "~их";
        dative: "~ой", "~им";
        accusative: "~ую", referred;
        instrumental: "~ой|~ою", "~ими";
        prepositional: "~ой", "~их";
        noun_second_case: "~ой", "~ой";
    ),
    // Ядро: 63 words (брашно, говно, деревцо, ядро, масло, вёдро).
    declension!("ядро", 63, "~~о";
        nominative: "~~о", "~~а";
        genitive: "~~а", "~е~";
        dative: "~~у", "~~ам";
        accusative: referred, referred;
        instrumental: "~~ом", "~~ами";
        prepositional: "~~е", "~~ах";
        noun_second_case: "~~а", "~~е";
    ),
    // Внучонок: 57 words (медвежонок, индюшонок, крольчонок, внучонок, волчонок, мышонок).
    declension!("внучонок", 57, "~онок";
        nominative: "~онок", "~ата";
        genitive: "~онка", "~ат";
        dative: "~онку", "~атам";
        accusative: referred, referred;
        instrumental: "~онком", "~атами";
        prepositional: "~онке", "~атах";
        noun_second_case: "~онка", "~онке";
    ),
    // Свеча: 56 words (левша, ханжа, вожжа, свеча, саранча, лапша).
    declension!("свеча", 56, "~а";
        nominative: "~а", "~и";
        genitive: "~и", "~ей";
        dative: "~е", "~ам";
        accusative: "~у", referred;
        instrumental: "~ой|~ою", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~и", "~е";
    ),
    // Пейзан: 52 words (солдат, цыган, гусар, вольт, погон, волос).
    declension!("пейзан", 52, "~";
        nominative: "~", "~ы";
        genitive: "~а", "~";
        dative: "~у", "~ам";
        accusative: referred, referred;
        instrumental: "~ом", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~а", "~е";
    ),
    // Дрова: 52 words (ребята, врата, дрова, уста, внучата, правнучата).
    declension!("дрова", 52, "~а";
        nominative: "~а", "~а";
        genitive: "~ы", "~";
        dative: "~е", "~ам";
        accusative: "~у", referred;
        instrumental: "~ой", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~ы", "~е";
    ),
    // Съезжая: 46 words (прихожая, телеведущая, съезжая, неверующая, купчая, составляющая).
    declension!("съезжая", 46, "~ая";
        nominative: "~ая", "~ие";
        genitive: "~ей", "~их";
        dative: "~ей", "~им";
        accusative: "~ую", referred;
        instrumental: "~ей|~ею", "~ими";
        prepositional: "~ей", "~их";
        noun_second_case: "~ей", "~ей";
    ),
    // Каракуля: 45 words (постеля, батя, дядя, доля, ходуля, сводня).
    declension!("каракуля", 45, "~я";
        nominative: "~я", "~и";
        genitive: "~и", "~ей";
        dative: "~е", "~ям";
        accusative: "~ю", referred;
        instrumental: "~ей|~ею", "~ями";
        prepositional: "~е", "~ях";
        noun_second_case: "~и", "~е";
    ),
    // Чужое: 43 words (многое, шампанское, лёгкое, членистоногое, жаркое, чужое).
    declension!("чужое", 43, "~ое";
        nominative: "~ое", "~ие";
        genitive: "~ого", "~их";
        dative: "~ому", "~им";
        accusative: referred, referred;
        instrumental: "~им", "~ими";
        prepositional: "~ом", "~их";
        noun_second_case: "~ого", "~ом";
    ),
    // Хуище: 40 words (дружище, когтище, волчище, хуище, козлище, третье).
    declension!("хуище", 40, "~е";
        nominative: "~е", "~и";
        genitive: "~а", "~";
        dative: "~у", "~ам";
        accusative: referred, referred;
        instrumental: "~ем", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~а", "~е";
    ),
    // Мощи: 38 words (вожжи, дрожжи, щи, клещи, пассатижи, мощи).
    declension!("мощи", 38, "~и";
        nominative: "~и", "~и";
        genitive: "~и", "~ей";
        dative: "~и", "~ам";
        accusative: referred, referred;
        instrumental: "~и", "~ами";
        prepositional: "~и", "~ах";
        noun_second_case: "~и", "~и";
    ),
    // Делёж: 36 words (пиздёж, чертёж, грабёж, платёж, мухлёж, кутёж).
    declension!("делёж", 36, "~ё~";
        nominative: "~ё~", "~е~и";
        genitive: "~е~а", "~е~ей";
        dative: "~е~у", "~е~ам";
        accusative: referred, referred;
        instrumental: "~е~ом", "~е~ами";
        prepositional: "~е~е", "~е~ах";
        noun_second_case: "~е~а", "~е~е";
    ),
    // Положняк: 35 words (страх, горох, коньяк, лак, мак, грог).
    declension!("положняк", 35, "~";
        nominative: "~", "~и";
        genitive: "~а", "~ов";
        dative: "~у", "~ам";
        accusative: referred, referred;
        instrumental: "~ом", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~у", "~е";
    ),
    // Дрязги: 34 words (брюки, ладоши, лыжи, вериги, пики, брызги).
    declension!("дрязги", 34, "~и";
        nominative: "~и", "~и";
        genitive: "~и", "~";
        dative: "~е", "~ам";
        accusative: "~у", referred;
        instrumental: "~ой", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~и", "~е";
    ),
    // Горючее: 33 words (настоящее, будущее, горючее, грядущее, прочее, болеутоляющее).
    declension!("горючее", 33, "~ее";
        nominative: "~ее", "~ие";
        genitive: "~его", "~их";
        dative: "~ему", "~им";
        accusative: referred, referred;
        instrumental: "~им", "~ими";
        prepositional: "~ем", "~их";
        noun_second_case: "~его", "~ем";
    ),
    // Ладья: 30 words (семья, ладья, статья, судья, скамья, голья).
    declension!("ладья", 30, "~ья";
        nominative: "~ья", "~ьи";
        genitive: "~ьи", "~ей";
        dative: "~ье", "~ьям";
        accusative: "~ью", referred;
        instrumental: "~ьёй|~ьёю", "~ьями";
        prepositional: "~ье", "~ьях";
        noun_second_case: "~ьи", "~ье";
    ),
    // Верея: 29 words (змея, колея, чешуя, швея, струя, алия).
    declension!("верея", 29, "~я";
        nominative: "~я", "~и";
        genitive: "~и", "~й";
        dative: "~е", "~ям";
        accusative: "~ю", referred;
        instrumental: "~ёй|~ёю", "~ями";
        prepositional: "~е", "~ях";
        noun_second_case: "~и", "~е";
    ),
    // Кешью: 29 words (порося, кешью, чапаев, гэта, мюсли, пол-москвы).
    declension!("кешью", 29, "~";
        nominative: "~", "~";
        genitive: "~", "~";
        dative: "~", "~";
        accusative: "~", "~";
        instrumental: "~", "~";
        prepositional: "~", "~";
        noun_second_case: "~", "~";
    ),
    // Порубка: 28 words (скоблянка, меньшевичка, связистка, отклейка, порубка, самосброска).
    declension!("порубка", 28, "~а";
        nominative: "~а", "~и";
        genitive: "~и", "~а";
        dative: "~е", "~ам";
        accusative: "~у", referred;
        instrumental: "~ой|~ою", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~и", "~е";
    ),
    // Шов: 27 words (свёкор, сон, хохол, бугор, посол, боз).
    declension!("шов", 27, "~о~";
        nominative: "~о~", "~~ы";
        genitive: "~~а", "~~ов";
        dative: "~~у", "~~ам";
        accusative: referred, referred;
        instrumental: "~~ом", "~~ами";
        prepositional: "~~е", "~~ах";
        noun_second_case: "~~а", "~~е";
    ),
    // Егерь: 27 words (учитель, якорь, флигель, лагерь, тополь, соболь).
    declension!("егерь", 27, "~ь";
        nominative: "~ь", "~я";
        genitive: "~я", "~ей";
        dative: "~ю", "~ям";
        accusative: referred, referred;
        instrumental: "~ем", "~ями";
        prepositional: "~е", "~ях";
        noun_second_case: "~я", "~е";
    ),
    // Деревня: 27 words (торговля, сабля, капля, туфля, чапля, цапля).
    declension!("деревня", 27, "~~я";
        nominative: "~~я", "~~и";
        genitive: "~~и", "~е~ь";
        dative: "~~е", "~~ям";
        accusative: "~~ю", referred;
        instrumental: "~~ей|~~ею", "~~ями";
        prepositional: "~~е", "~~ях";
        noun_second_case: "~~и", "~~е";
    ),
    // Поллитра: 26 words (пол-очка, пол-литра, пол-оборота, пол-августа, пол-автобуса, пол-аршина).
    declension!("пол-литра", 26, "~-~а";
        nominative: "~-~а", "~у~ы";
        genitive: "~у~а", "~у~ов";
        dative: "~у~у", "~у~ам";
        accusative: referred, referred;
        instrumental: "~у~ом", "~у~ами";
        prepositional: "~у~е", "~у~ах";
        noun_second_case: "~у~а", "~у~е";
    ),
    // Щит: 24 words (мир, кон, под, стан, зоб, щит).
    declension!("щит", 24, "~";
        nominative: "~", "~ы";
        genitive: "~а", "~ов";
        dative: "~у", "~ам";
        accusative: referred, referred;
        instrumental: "~ом", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~а", "~у";
    ),
    // Голец: 24 words (лев, телец, стрелец, жилец, делец, малец).
    declension!("голец", 24, "~е~";
        nominative: "~е~", "~ь~ы";
        genitive: "~ь~а", "~ь~ов";
        dative: "~ь~у", "~ь~ам";
        accusative: referred, referred;
        instrumental: "~ь~ом", "~ь~ами";
        prepositional: "~ь~е", "~ь~ах";
        noun_second_case: "~ь~а", "~ь~е";
    ),
    // Домашние: 24 words (немногие, сколькие, брюхоногие, близкие, древние, домашние).
    declension!("домашние", 24, "~ие";
        nominative: "~ее", "~ие";
        genitive: "~его", "~их";
        dative: "~ему", "~им";
        accusative: "~ее", referred;
        instrumental: "~им", "~ими";
        prepositional: "~ем", "~их";
        noun_second_case: "~его", "~ем";
    ),
    // Звезда: 23 words (звезда, пчела, жена, сестра, медсестра, береста).
    declension!("звезда", 23, "~е~а";
        nominative: "~е~а", "~ё~ы";
        genitive: "~е~ы", "~ё~";
        dative: "~е~е", "~ё~ам";
        accusative: "~е~у", referred;
        instrumental: "~е~ой|~е~ою", "~ё~ами";
        prepositional: "~е~е", "~ё~ах";
        noun_second_case: "~е~ы", "~е~е";
    ),
    // Член: 23 words (клон, тип, член, кат, кекс, бас).
    declension!("член", 23, "~";
        nominative: "~", "~ы";
        genitive: "~а", "~ов";
        dative: "~у", "~ам";
        accusative: "~а|~", "~ов|~ы";
        instrumental: "~ом", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~а", "~е";
    ),
    // Лицо: 23 words (существо, лицо, божество, чадо, чмо, падло).
    declension!("лицо", 23, "~о";
        nominative: "~о", "~а";
        genitive: "~а", "~";
        dative: "~у", "~ам";
        accusative: "~о", referred;
        instrumental: "~ом", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~а", "~е";
    ),
    // Маракуйя: 22 words (аллилуйя, секвойя, паранойя, папайя, пирайя, майя).
    declension!("маракуйя", 22, "~я";
        nominative: "~я", "~и";
        genitive: "~и", "~";
        dative: "~е", "~ям";
        accusative: "~ю", referred;
        instrumental: "~ей|~ею", "~ями";
        prepositional: "~е", "~ях";
        noun_second_case: "~и", "~е";
    ),
    // Деньги: 21 words (деньги, четвереньки, грабельки, хахоньки, ахоньки, ясельки).
    declension!("деньги", 21, "~ь~и";
        nominative: "~ь~а", "~ь~и";
        genitive: "~ь~и", "~е~";
        dative: "~ь~е", "~ь~ам";
        accusative: "~ь~у", referred;
        instrumental: "~ь~ой", "~ь~ами";
        prepositional: "~ь~е", "~ь~ах";
        noun_second_case: "~ь~и", "~ь~е";
    ),
    // Устье: 21 words (платье, устье, подмастерье, подполье, уголье, отребье).
    declension!("устье", 21, "~е";
        nominative: "~е", "~я";
        genitive: "~я", "~ев";
        dative: "~ю", "~ям";
        accusative: referred, referred;
        instrumental: "~ем", "~ями";
        prepositional: "~е", "~ях";
        noun_second_case: "~я", "~е";
    ),
    // Ребро: 21 words (сребро, серебро, ребро, весло, зерно, бедро).
    declension!("ребро", 21, "~е~~о";
        nominative: "~е~~о", "~ё~~а";
        genitive: "~е~~а", "~ё~е~";
        dative: "~е~~у", "~ё~~ам";
        accusative: referred, referred;
        instrumental: "~е~~ом", "~ё~~ами";
        prepositional: "~е~~е", "~ё~~ах";
        noun_second_case: "~е~~а", "~е~~е";
    ),
    // Козёл: 20 words (пёс, козёл, орёл, овёс, осёл, костёр).
    declension!("козёл", 20, "~ё~";
        nominative: "~ё~", "~~ы";
        genitive: "~~а", "~~ов";
        dative: "~~у", "~~ам";
        accusative: referred, referred;
        instrumental: "~~ом", "~~ами";
        prepositional: "~~е", "~~ах";
        noun_second_case: "~~а", "~~е";
    ),
    // Хуй: 20 words (хуй, рой, слой, буй, гай, пай).
    declension!("хуй", 20, "~й";
        nominative: "~й", "~и";
        genitive: "~я", "~ёв";
        dative: "~ю", "~ям";
        accusative: referred, referred;
        instrumental: "~ем", "~ями";
        prepositional: "~е", "~ях";
        noun_second_case: "~я", "~е";
    ),
    // Клин: 19 words (брат, лист, муж, клин, стул, струп).
    declension!("клин", 19, "~";
        nominative: "~", "~ья";
        genitive: "~а", "~ьев";
        dative: "~у", "~ьям";
        accusative: referred, referred;
        instrumental: "~ом", "~ьями";
        prepositional: "~е", "~ьях";
        noun_second_case: "~а", "~е";
    ),
    // Сенцы: 19 words (японцы, плоскогубцы, плоскозубцы, шлёпанцы, сланцы, святцы).
    declension!("сенцы", 19, "~ы";
        nominative: "~", "~ы";
        genitive: "~а", "~ев";
        dative: "~у", "~ам";
        accusative: referred, referred;
        instrumental: "~ем", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~а", "~е";
    ),
    // Нос: 18 words (час, чад, пол, дуб, нос, сыр).
    declension!("нос", 18, "~";
        nominative: "~", "~ы";
        genitive: "~а", "~ов";
        dative: "~у", "~ам";
        accusative: referred, referred;
        instrumental: "~ом", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~у", "~у";
    ),
    // Ископаемое: 18 words (насекомое, животное, земноводное, парнокопытное, ракообразное, позвоночное).
    declension!("ископаемое", 18, "~ое";
        nominative: "~ое", "~ые";
        genitive: "~ого", "~ых";
        dative: "~ому", "~ым";
        accusative: "~ое", referred;
        instrumental: "~ым", "~ыми";
        prepositional: "~ом", "~ых";
        noun_second_case: "~ого", "~ом";
    ),
    // Боёк: 18 words (паёк, чаёк, ручеёк, кофеёк, боёк, буёк).
    declension!("боёк", 18, "~ё~";
        nominative: "~ё~", "~й~и";
        genitive: "~й~а", "~й~ов";
        dative: "~й~у", "~й~ам";
        accusative: referred, referred;
        instrumental: "~й~ом", "~й~ами";
        prepositional: "~й~е", "~й~ах";
        noun_second_case: "~й~а", "~й~е";
    ),
    // Детище: 17 words (чудовище, чудище, детище, посмешище, уёбище, страшилище).
    declension!("детище", 17, "~е";
        nominative: "~е", "~а";
        genitive: "~а", "~";
        dative: "~у", "~ам";
        accusative: "~е", referred;
        instrumental: "~ем", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~а", "~е";
    ),
    // Очко: 16 words (колёсико, древко, очко, брюшко, личико, пивко).
    declension!("очко", 16, "~о";
        nominative: "~о", "~и";
        genitive: "~а", "~ов";
        dative: "~у", "~ам";
        accusative: referred, referred;
        instrumental: "~ом", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~а", "~е";
    ),
    // Анкара: 16 words (ковурма, дисна, полумгла, мста, ангара, анкара).
    declension!("анкара", 16, "~а";
        nominative: "~а", "~ы";
        genitive: "~ы", "~а";
        dative: "~е", "~ам";
        accusative: "~у", referred;
        instrumental: "~ой|~ою", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~ы", "~е";
    ),
    // Зло: 15 words (окно, зло, сукно, волокно, жалко, тягло).
    declension!("зло", 15, "~~о";
        nominative: "~~о", "~~а";
        genitive: "~~а", "~о~";
        dative: "~~у", "~~ам";
        accusative: referred, referred;
        instrumental: "~~ом", "~~ами";
        prepositional: "~~е", "~~ах";
        noun_second_case: "~~а", "~~е";
    ),
    // Бойня: 15 words (кофейня, бойня, водогрейня, воскобойня, двойня, маслобойня).
    declension!("бойня", 15, "~й~я";
        nominative: "~й~я", "~й~и";
        genitive: "~й~и", "~е~";
        dative: "~й~е", "~й~ям";
        accusative: "~й~ю", referred;
        instrumental: "~й~ей|~й~ею", "~й~ями";
        prepositional: "~й~е", "~й~ях";
        noun_second_case: "~й~и", "~й~е";
    ),
    // Пень: 15 words (день, ремень, пень, кремень, плетень, трудодень).
    declension!("пень", 15, "~е~ь";
        nominative: "~е~ь", "~~и";
        genitive: "~~я", "~~ей";
        dative: "~~ю", "~~ям";
        accusative: referred, referred;
        instrumental: "~~ём", "~~ями";
        prepositional: "~~е", "~~ях";
        noun_second_case: "~~я", "~~е";
    ),
    // Побои: 15 words (обои, побои, помои, прелиминарии, покои, фотообои).
    declension!("побои", 15, "~и";
        nominative: "~и", "~и";
        genitive: "~и", "~ев";
        dative: "~и", "~ям";
        accusative: referred, referred;
        instrumental: "~и", "~ями";
        prepositional: "~и", "~ях";
        noun_second_case: "~и", "~и";
    ),
    // Перо: 14 words (дерево, перо, крыло, сено, авиазвено, звено).
    declension!("перо", 14, "~о";
        nominative: "~о", "~ья";
        genitive: "~а", "~ьев";
        dative: "~у", "~ьям";
        accusative: referred, referred;
        instrumental: "~ом", "~ьями";
        prepositional: "~е", "~ьях";
        noun_second_case: "~а", "~е";
    ),
    // Ебала: 14 words (ебала, ламиина, иссида, кокцинеллида, алеохарина, циксиида).
    declension!("ебала", 14, "~а";
        nominative: "~а", "~ы";
        genitive: "~ы", "~";
        dative: "~е", "~ам";
        accusative: "~у", "~ы";
        instrumental: "~ой|~ою", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~ы", "~е";
    ),
    // Губоцветные: 14 words (вересковые, водяниковые, верескоцветные, буковые, букоцветные, мальпигиевые).
    declension!("губоцветные", 14, "~е";
        nominative: "~е", "~я";
        genitive: "~х", "~й";
        dative: "~м", "~ям";
        accusative: referred, referred;
        instrumental: "~ми", "~ями";
        prepositional: "~х", "~ях";
        noun_second_case: "~х", "~х";
    ),
    // Сердце: 13 words (сердце, деревце, полотенце, блюдце, оконце, волоконце).
    declension!("сердце", 13, "~~е";
        nominative: "~~е", "~~а";
        genitive: "~~а", "~е~";
        dative: "~~у", "~~ам";
        accusative: referred, referred;
        instrumental: "~~ем", "~~ами";
        prepositional: "~~е", "~~ах";
        noun_second_case: "~~а", "~~е";
    ),
    // Котёночек: 13 words (цыплёночек, котёночек, жеребёночек, козлёночек, утёночек, ягнёночек).
    declension!("котёночек", 13, "~ёноче~";
        nominative: "~ёноче~", "~ят~и";
        genitive: "~ёноч~а", "~ято~";
        dative: "~ёноч~у", "~ят~ам";
        accusative: referred, referred;
        instrumental: "~ёноч~ом", "~ят~ами";
        prepositional: "~ёноч~е", "~ят~ах";
        noun_second_case: "~ёноч~а", "~ёноч~е";
    ),
    // Меринос: 13 words (квакер, виолончелист, солитер, кастрат, бахмат, дидакт).
    declension!("меринос", 13, "~";
        nominative: "~", "~ы";
        genitive: "~а", "~ов";
        dative: "~у", "~ам";
        accusative: "~", "~ы";
        instrumental: "~ом", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~а", "~е";
    ),
    // Гнездо: 12 words (озеро, колесо, село, чело, решето, гнездо).
    declension!("гнездо", 12, "~е~о";
        nominative: "~е~о", "~ё~а";
        genitive: "~е~а", "~ё~";
        dative: "~е~у", "~ё~ам";
        accusative: referred, referred;
        instrumental: "~е~ом", "~ё~ами";
        prepositional: "~е~е", "~ё~ах";
        noun_second_case: "~е~а", "~е~е";
    ),
    // Фекалии: 12 words (химикалии, фекалии, авиалинии, гениталии, сатурналии, реалии).
    declension!("фекалии", 12, "~и";
        nominative: "~и", "~и";
        genitive: "~и", "~й";
        dative: "~и", "~ям";
        accusative: referred, referred;
        instrumental: "~и", "~ями";
        prepositional: "~и", "~ях";
        noun_second_case: "~и", "~и";
    ),
    // Кольцо: 12 words (письмо, дерьмо, кольцо, крыльцо, сельцо, автокольцо).
    declension!("кольцо", 12, "~ь~о";
        nominative: "~ь~о", "~ь~а";
        genitive: "~ь~а", "~е~";
        dative: "~ь~у", "~ь~ам";
        accusative: referred, referred;
        instrumental: "~ь~ом", "~ь~ами";
        prepositional: "~ь~е", "~ь~ах";
        noun_second_case: "~ь~а", "~ь~е";
    ),
    // Рыльце: 12 words (зеркальце, щупальце, дельце, рыльце, сальце, мыльце).
    declension!("рыльце", 12, "~ь~е";
        nominative: "~ь~е", "~ь~а";
        genitive: "~ь~а", "~е~";
        dative: "~ь~у", "~ь~ам";
        accusative: referred, referred;
        instrumental: "~ь~ем", "~ь~ами";
        prepositional: "~ь~е", "~ь~ах";
        noun_second_case: "~ь~а", "~ь~е";
    ),
    // Валенок: 11 words (ботинок, чулок, турок, валенок, глазок, зубок).
    declension!("валенок", 11, "~о~";
        nominative: "~о~", "~~и";
        genitive: "~~а", "~о~";
        dative: "~~у", "~~ам";
        accusative: referred, referred;
        instrumental: "~~ом", "~~ами";
        prepositional: "~~е", "~~ах";
        noun_second_case: "~~а", "~~е";
    ),
    // Кипяток: 11 words (песок, мох, кипяток, холодок, дымок, сахарок).
    declension!("кипяток", 11, "~о~";
        nominative: "~о~", "~~и";
        genitive: "~~а", "~~ов";
        dative: "~~у", "~~ам";
        accusative: referred, referred;
        instrumental: "~~ом", "~~ами";
        prepositional: "~~е", "~~ах";
        noun_second_case: "~~у", "~~е";
    ),
    // Счёт: 11 words (мёд, счёт, шёлк, жёрнов, жёлоб, аэрожёлоб).
    declension!("счёт", 11, "~ё~";
        nominative: "~ё~", "~е~а";
        genitive: "~ё~а", "~е~ов";
        dative: "~ё~у", "~е~ам";
        accusative: referred, referred;
        instrumental: "~ё~ом", "~е~ами";
        prepositional: "~ё~е", "~е~ах";
        noun_second_case: "~ё~а", "~ё~е";
    ),
    // Прения: 11 words (прения, условия, переживания, усилия, воспоминания, взаимоотношения).
    declension!("прения", 11, "~я";
        nominative: "~я", "~я";
        genitive: "~и", "~й";
        dative: "~и", "~ям";
        accusative: "~ю", referred;
        instrumental: "~ей", "~ями";
        prepositional: "~и", "~ях";
        noun_second_case: "~и", "~и";
    ),
    // Тельце: 11 words (тельце, весельце, дульце, жальце, зальце, одеяльце).
    declension!("тельце", 11, "~ь~е";
        nominative: "~ь~е", "~ь~а";
        genitive: "~ь~а", "~ь~ев|~е~";
        dative: "~ь~у", "~ь~ам";
        accusative: referred, referred;
        instrumental: "~ь~ем", "~ь~ами";
        prepositional: "~ь~е", "~ь~ах";
        noun_second_case: "~ь~а", "~ь~е";
    ),
    // Племя: 10 words (имя, время, знамя, вымя, пламя, племя).
    declension!("племя", 10, "~я";
        nominative: "~я", "~ена";
        genitive: "~ени", "~ён";
        dative: "~ени", "~енам";
        accusative: referred, referred;
        instrumental: "~енем", "~енами";
        prepositional: "~ени", "~енах";
        noun_second_case: "~ени", "~ени";
    ),
    // Глаз: 10 words (глаз, цвет, голос, окорок, рост, сахар).
    declension!("глаз", 10, "~";
        nominative: "~", "~а";
        genitive: "~а", "~ов";
        dative: "~у", "~ам";
        accusative: referred, referred;
        instrumental: "~ом", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~у", "~е";
    ),
    // Усадьба: 10 words (тьма, судьба, свадьба, тюрьма, усадьба, пыльца).
    declension!("усадьба", 10, "~ь~а";
        nominative: "~ь~а", "~ь~ы";
        genitive: "~ь~ы", "~е~";
        dative: "~ь~е", "~ь~ам";
        accusative: "~ь~у", referred;
        instrumental: "~ь~ой|~ь~ою", "~ь~ами";
        prepositional: "~ь~е", "~ь~ах";
        noun_second_case: "~ь~ы", "~ь~е";
    ),
    // Хлопья: 10 words (хлопья, лохмотья, объедья, брусья, коренья, стручья).
    declension!("хлопья", 10, "~я";
        nominative: "~я", "~я";
        genitive: "~и", "~ев";
        dative: "~и", "~ям";
        accusative: "~ю", referred;
        instrumental: "~ей", "~ями";
        prepositional: "~и", "~ях";
        noun_second_case: "~и", "~и";
    ),
    // Коленце: 10 words (болотце, коленце, ведёрце, корытце, долотце, донце).
    declension!("коленце", 10, "~~е";
        nominative: "~~е", "~~а";
        genitive: "~~а", "~~ев|~е~";
        dative: "~~у", "~~ам";
        accusative: referred, referred;
        instrumental: "~~ем", "~~ами";
        prepositional: "~~е", "~~ах";
        noun_second_case: "~~а", "~~е";
    ),
    // Дом: 9 words (дом, лес, век, снег, мех, стог).
    declension!("дом", 9, "~";
        nominative: "~", "~а";
        genitive: "~а", "~ов";
        dative: "~у", "~ам";
        accusative: referred, referred;
        instrumental: "~ом", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~у", "~у";
    ),
    // Перешеек: 9 words (перешеек, закраек, зашеек, опоек, отроек, ошеек).
    declension!("перешеек", 9, "~е~";
        nominative: "~е~", "~й~и";
        genitive: "~й~а", "~й~ов";
        dative: "~й~у", "~й~ам";
        accusative: referred, referred;
        instrumental: "~й~ом", "~й~ами";
        prepositional: "~й~е", "~й~ах";
        noun_second_case: "~й~а", "~й~е";
    ),
    // Плечо: 8 words (яблоко, молоко, млеко, веко, плечо, брюхо).
    declension!("плечо", 8, "~о";
        nominative: "~о", "~и";
        genitive: "~а", "~";
        dative: "~у", "~ам";
        accusative: referred, referred;
        instrumental: "~ом", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~а", "~е";
    ),
    // Горе: 8 words (море, горе, поле, перекати-поле, муде, биополе).
    declension!("горе", 8, "~е";
        nominative: "~е", "~я";
        genitive: "~я", "~ей";
        dative: "~ю", "~ям";
        accusative: referred, referred;
        instrumental: "~ем", "~ями";
        prepositional: "~е", "~ях";
        noun_second_case: "~я", "~е";
    ),
    // Кукла: 8 words (мгла, кукла, мошна, княжна, свёкла, чухна).
    declension!("кукла", 8, "~~а";
        nominative: "~~а", "~~ы";
        genitive: "~~ы", "~о~";
        dative: "~~е", "~~ам";
        accusative: "~~у", referred;
        instrumental: "~~ой|~~ою", "~~ами";
        prepositional: "~~е", "~~ах";
        noun_second_case: "~~ы", "~~е";
    ),
    // Покой: 8 words (гной, покой, иней, сельдерей, шалфей, перегной).
    declension!("покой", 8, "~й";
        nominative: "~й", "~и";
        genitive: "~я", "~ев";
        dative: "~ю", "~ям";
        accusative: referred, referred;
        instrumental: "~ем", "~ями";
        prepositional: "~е", "~ях";
        noun_second_case: "~ю", "~е";
    ),
    // Герц: 8 words (герц, гигагерц, килогерц, мегагерц, миллигерц, кольбац).
    declension!("герц", 8, "~";
        nominative: "~", "~ы";
        genitive: "~а", "~";
        dative: "~у", "~ам";
        accusative: referred, referred;
        instrumental: "~ем", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~а", "~е";
    ),
    // Калашников: 8 words (калашников, басков, топтыгин, обещалкин, сердечкин, собакин).
    declension!("калашников", 8, "~";
        nominative: "~", "~ы";
        genitive: "~а", "~ых";
        dative: "~у", "~ым";
        accusative: referred, referred;
        instrumental: "~ым", "~ыми";
        prepositional: "~е", "~ых";
        noun_second_case: "~а", "~е";
    ),
    // Остриё: 8 words (остриё, дитё, востриё, хулиганьё, гадьё, мудачьё).
    declension!("остриё", 8, "~ё";
        nominative: "~ё", "~я";
        genitive: "~я", "~ей";
        dative: "~ю", "~ям";
        accusative: "~ё", referred;
        instrumental: "~ём", "~ями";
        prepositional: "~е", "~ях";
        noun_second_case: "~я", "~е";
    ),
    // Плюхино: 8 words (мотовство, плюхино, плюхново, кабаново, клементьево, бахтино).
    declension!("плюхино", 8, "~о";
        nominative: "~о", "~а|~о";
        genitive: "~а|~о", "~о";
        dative: "~у|~о", "~ам|~о";
        accusative: referred, referred;
        instrumental: "~ом|~о", "~ами|~о";
        prepositional: "~е|~о", "~ах|~о";
        noun_second_case: "~а|~о", "~е|~о";
    ),
    // Полюбки: 8 words (пол-ёмкости, пол-линии, пол-ложки, пол-области, пол-юбки, пол-осьмухи).
    declension!("пол-юбки", 8, "~-~и";
        nominative: "~-~и", "~у~и";
        genitive: "~у~и", "~у~ей";
        dative: "~у~и", "~у~ям";
        accusative: referred, referred;
        instrumental: "~у~ью", "~у~ями";
        prepositional: "~у~и", "~у~ях";
        noun_second_case: "~у~и", "~у~и";
    ),
    // Ложь: 7 words (церковь, рожь, вошь, ложь, любовь, нелюбовь).
    declension!("ложь", 7, "~о~ь";
        nominative: "~о~ь", "~~и";
        genitive: "~~и", "~~ей";
        dative: "~~и", "~~ям";
        accusative: referred, referred;
        instrumental: "~о~ью", "~~ями";
        prepositional: "~~и", "~~ях";
        noun_second_case: "~~и", "~~и";
    ),
    // Дух: 7 words (воздух, сок, дух, шаг, пух, полк).
    declension!("дух", 7, "~";
        nominative: "~", "~и";
        genitive: "~а", "~ов";
        dative: "~у", "~ам";
        accusative: referred, referred;
        instrumental: "~ом", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~у", "~у";
    ),
    // Лапоть: 7 words (локоть, дёготь, ноготь, уголь, коготь, лапоть).
    declension!("лапоть", 7, "~о~ь";
        nominative: "~о~ь", "~~и";
        genitive: "~~я", "~~ей";
        dative: "~~ю", "~~ям";
        accusative: referred, referred;
        instrumental: "~~ем", "~~ями";
        prepositional: "~~е", "~~ях";
        noun_second_case: "~~я", "~~е";
    ),
    // Череда: 7 words (середа, железа, слеза, пелена, череда, желна).
    declension!("череда", 7, "~е~а";
        nominative: "~е~а", "~е~ы";
        genitive: "~е~ы", "~ё~";
        dative: "~е~е", "~е~ам";
        accusative: "~е~у", referred;
        instrumental: "~е~ой|~е~ою", "~е~ами";
        prepositional: "~е~е", "~е~ах";
        noun_second_case: "~е~ы", "~е~е";
    ),
    // Сиверко: 7 words (ведёрко, сиверко, железко, весёлко, одеялко, морозко).
    declension!("сиверко", 7, "~~о";
        nominative: "~~о", "~~и";
        genitive: "~~а", "~о~";
        dative: "~~у", "~~ам";
        accusative: referred, referred;
        instrumental: "~~ом", "~~ами";
        prepositional: "~~е", "~~ах";
        noun_second_case: "~~а", "~~е";
    ),
    // Передняя: 7 words (передняя, преисподняя, ближняя, замужняя, незамужняя, несовершеннолетняя).
    declension!("передняя", 7, "~яя";
        nominative: "~яя", "~ие";
        genitive: "~ей", "~их";
        dative: "~ей", "~им";
        accusative: "~юю", referred;
        instrumental: "~ей|~ею", "~ими";
        prepositional: "~ей", "~их";
        noun_second_case: "~ей", "~ей";
    ),
    // Семя: 7 words (бремя, темя, семя, стремя, беремя, льносемя).
    declension!("семя", 7, "~я";
        nominative: "~я", "~ена";
        genitive: "~ени", "~ян";
        dative: "~ени", "~енам";
        accusative: referred, referred;
        instrumental: "~енем", "~енами";
        prepositional: "~ени", "~енах";
        noun_second_case: "~ени", "~ени";
    ),
    // Стекло: 7 words (стекло, бронестекло, стегно, пекло, наностекло, оргстекло).
    declension!("стекло", 7, "~е~~о";
        nominative: "~е~~о", "~ё~~а";
        genitive: "~е~~а", "~ё~о~";
        dative: "~е~~у", "~ё~~ам";
        accusative: referred, referred;
        instrumental: "~е~~ом", "~ё~~ами";
        prepositional: "~е~~е", "~ё~~ах";
        noun_second_case: "~е~~а", "~е~~е";
    ),
    // Заём: 7 words (заём, наём, поднаём, перенаём, госзаём, сберзаём).
    declension!("заём", 7, "~ё~";
        nominative: "~ё~", "~й~ы";
        genitive: "~й~а", "~й~ов";
        dative: "~й~у", "~й~ам";
        accusative: referred, referred;
        instrumental: "~й~ом", "~й~ами";
        prepositional: "~й~е", "~й~ах";
        noun_second_case: "~й~а", "~й~е";
    ),
    // Весна: 7 words (десна, весна, метла, ветла, гидрометла, блесна).
    declension!("весна", 7, "~е~~а";
        nominative: "~е~~а", "~ё~~ы";
        genitive: "~е~~ы", "~ё~е~";
        dative: "~е~~е", "~ё~~ам";
        accusative: "~е~~у", referred;
        instrumental: "~е~~ой|~е~~ою", "~ё~~ами";
        prepositional: "~е~~е", "~ё~~ах";
        noun_second_case: "~е~~ы", "~е~~е";
    ),
    // Чужой: 7 words (глухой, чужой, костромской, нитакой, старшой, большой).
    declension!("чужой", 7, "~ой";
        nominative: "~ой", "~ие";
        genitive: "~ого", "~их";
        dative: "~ому", "~им";
        accusative: referred, referred;
        instrumental: "~им", "~ими";
        prepositional: "~ом", "~их";
        noun_second_case: "~ого", "~ом";
    ),
    // Полгода: 7 words (полчаса, полгода, полметра, полмиллиона, полкилометра, поллитра).
    declension!("полгода", 7, "~~а";
        nominative: "~~а", "~у~ы";
        genitive: "~у~а", "~у~ов";
        dative: "~у~у", "~у~ам";
        accusative: referred, referred;
        instrumental: "~у~ом", "~у~ами";
        prepositional: "~у~е", "~у~ах";
        noun_second_case: "~у~а", "~у~е";
    ),
    // Полиюля: 7 words (пол-апреля, пол-имения, пол-июля, пол-июня, пол-лагеря, пол-октября).
    declension!("пол-июля", 7, "~-~я";
        nominative: "~-~я", "~у~и";
        genitive: "~у~я", "~у~ей";
        dative: "~у~ю", "~у~ям";
        accusative: referred, referred;
        instrumental: "~у~ем", "~у~ями";
        prepositional: "~у~е", "~у~ях";
        noun_second_case: "~у~я", "~у~е";
    ),
    // Полигры: 7 words (пол-игры, пол-избы, пол-луковицы, пол-юрты, пол-ягоды, пол-луны).
    declension!("пол-игры", 7, "~-~ы";
        nominative: "~-~ы", "~у~ы";
        genitive: "~у~ы", "~у~";
        dative: "~у~е", "~у~ам";
        accusative: referred, referred;
        instrumental: "~у~ой", "~у~ами";
        prepositional: "~у~е", "~у~ах";
        noun_second_case: "~у~ы", "~у~е";
    ),
    // Бой: 6 words (чай, бой, край, клей, иван-чай, фиточай).
    declension!("бой", 6, "~й";
        nominative: "~й", "~и";
        genitive: "~я", "~ёв";
        dative: "~ю", "~ям";
        accusative: referred, referred;
        instrumental: "~ем", "~ями";
        prepositional: "~е", "~ях";
        noun_second_case: "~ю", "~е";
    ),
    // Кто: 6 words (что, кто, кое-кто, никто, ничто, кое-что).
    declension!("кто", 6, "~то";
        nominative: "~то", "~то";
        genitive: "~ого", "~ого";
        dative: "~ому", "~ому";
        accusative: referred, referred;
        instrumental: "~ем", "~ем";
        prepositional: "~ом", "~ом";
        noun_second_case: "~ого", "~ом";
    ),
    // Млекопитающее: 6 words (млекопитающее, окружающее, обезболивающее, иглокожее, кишечнодышащее, простейшее).
    declension!("млекопитающее", 6, "~ее";
        nominative: "~ее", "~ие";
        genitive: "~его", "~их";
        dative: "~ему", "~им";
        accusative: "~ее", referred;
        instrumental: "~им", "~ими";
        prepositional: "~ем", "~их";
        noun_second_case: "~его", "~ем";
    ),
    // Татарин: 6 words (болгарин, татарин, хазарин, угрин, долганин, хариджанин).
    declension!("татарин", 6, "~ин";
        nominative: "~ин", "~ы";
        genitive: "~ина", "~";
        dative: "~ину", "~ам";
        accusative: referred, referred;
        instrumental: "~ином", "~ами";
        prepositional: "~ине", "~ах";
        noun_second_case: "~ина", "~ине";
    ),
    // Осётр: 6 words (ксёндз, чёлн, осётр, черёд, клёст, пётр).
    declension!("осётр", 6, "~ё~";
        nominative: "~ё~", "~е~ы";
        genitive: "~е~а", "~е~ов";
        dative: "~е~у", "~е~ам";
        accusative: referred, referred;
        instrumental: "~е~ом", "~е~ами";
        prepositional: "~е~е", "~е~ах";
        noun_second_case: "~е~а", "~е~е";
    ),
    // Чукча: 6 words (юноша, чукча, раджа, обжа, корча, магараджа).
    declension!("чукча", 6, "~а";
        nominative: "~а", "~и";
        genitive: "~и", "~ей";
        dative: "~е", "~ам";
        accusative: "~у", referred;
        instrumental: "~ей|~ею", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~и", "~е";
    ),
    // Метёлка: 6 words (чечётка, метёлка, кержанка, разведёнка, скалолазка, удмуртка).
    declension!("метёлка", 6, "~~а";
        nominative: "~~а", "~~и";
        genitive: "~~и", "~о~";
        dative: "~~е", "~~ам";
        accusative: "~~у", "~о~";
        instrumental: "~~ой|~~ою", "~~ами";
        prepositional: "~~е", "~~ах";
        noun_second_case: "~~и", "~~е";
    ),
    // Макаронник: 6 words (франк, счётчик, макаронник, кержак, чирик, архистратиг).
    declension!("макаронник", 6, "~";
        nominative: "~", "~и";
        genitive: "~а", "~ов";
        dative: "~у", "~ам";
        accusative: "~а", "~ов";
        instrumental: "~ом", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~а", "~е";
    ),
    // Ебеня: 6 words (зеленя, ебеня, труселя, прохоря, чигиря, уборя).
    declension!("ебеня", 6, "~я";
        nominative: "~я", "~я";
        genitive: "~и", "~ей";
        dative: "~е", "~ям";
        accusative: "~ю", referred;
        instrumental: "~ёй", "~ями";
        prepositional: "~е", "~ях";
        noun_second_case: "~и", "~е";
    ),
    // Скло: 6 words (скло, ракло, шарыпово, запевало, карло, сткло).
    declension!("скло", 6, "~о";
        nominative: "~о", "~ы";
        genitive: "~а", "~";
        dative: "~у", "~ам";
        accusative: referred, referred;
        instrumental: "~ом", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~а", "~е";
    ),
    // Исайя: 6 words (исайя, хориоидея, заурея, стеаторея, ципрея, персея).
    declension!("исайя", 6, "~я";
        nominative: "~я", "~и";
        genitive: "~и", "~й";
        dative: "~е", "~ям";
        accusative: "~ю", "~й";
        instrumental: "~ей|~ею", "~ями";
        prepositional: "~е", "~ях";
        noun_second_case: "~и", "~е";
    ),
    // Поллета: 6 words (пол-лета, пол-лица, пол-лота, пол-окна, пол-училища, пол-ящика).
    declension!("пол-лета", 6, "~-~а";
        nominative: "~-~а", "~у~а";
        genitive: "~у~а", "~у~";
        dative: "~у~у", "~у~ам";
        accusative: referred, referred;
        instrumental: "~у~ом", "~у~ами";
        prepositional: "~у~е", "~у~ах";
        noun_second_case: "~у~а", "~у~е";
    ),
    // Круг: 5 words (мозг, круг, верх, слух, торг).
    declension!("круг", 5, "~";
        nominative: "~", "~и";
        genitive: "~а", "~ов";
        dative: "~у", "~ам";
        accusative: referred, referred;
        instrumental: "~ом", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~а", "~у";
    ),
    // Огонь: 5 words (огонь, угорь, ломоть, артогонь, стоп-огонь).
    declension!("огонь", 5, "~о~ь";
        nominative: "~о~ь", "~~и";
        genitive: "~~я", "~~ей";
        dative: "~~ю", "~~ям";
        accusative: referred, referred;
        instrumental: "~~ём", "~~ями";
        prepositional: "~~е", "~~ях";
        noun_second_case: "~~я", "~~е";
    ),
    // Кухня: 5 words (кухня, куфня, кормокухня, авиакухня, автокухня).
    declension!("кухня", 5, "~~я";
        nominative: "~~я", "~~и";
        genitive: "~~и", "~о~ь";
        dative: "~~е", "~~ям";
        accusative: "~~ю", referred;
        instrumental: "~~ей|~~ею", "~~ями";
        prepositional: "~~е", "~~ях";
        noun_second_case: "~~и", "~~е";
    ),
    // Дичь: 5 words (мышь, сволочь, дичь, бестолочь, водомышь).
    declension!("дичь", 5, "~ь";
        nominative: "~ь", "~и";
        genitive: "~и", "~ей";
        dative: "~и", "~ам";
        accusative: "~ь", referred;
        instrumental: "~ью", "~ами";
        prepositional: "~и", "~ах";
        noun_second_case: "~и", "~и";
    ),
    // Боец: 5 words (боец, хуец, политбоец, гаец, экс-боец).
    declension!("боец", 5, "~е~";
        nominative: "~е~", "~й~ы";
        genitive: "~й~а", "~й~ов";
        dative: "~й~у", "~й~ам";
        accusative: referred, referred;
        instrumental: "~й~ом", "~й~ами";
        prepositional: "~й~е", "~й~ах";
        noun_second_case: "~й~а", "~й~е";
    ),
    // Козлы: 5 words (ножны, бубны, козлы, люберцы, ливны).
    declension!("козлы", 5, "~~ы";
        nominative: "~~ы", "~~ы";
        genitive: "~у~ы", "~е~";
        dative: "~у~е", "~~ам";
        accusative: referred, referred;
        instrumental: "~у~ой", "~~ами";
        prepositional: "~у~е", "~~ах";
        noun_second_case: "~у~ы", "~у~е";
    ),
    // Репей: 5 words (соловей, воробей, муравей, ручей, репей).
    declension!("репей", 5, "~ей";
        nominative: "~ей", "~ьи";
        genitive: "~ья", "~ьёв";
        dative: "~ью", "~ьям";
        accusative: referred, referred;
        instrumental: "~ьём", "~ьями";
        prepositional: "~ье", "~ьях";
        noun_second_case: "~ья", "~ье";
    ),
    // Хозяин: 5 words (хозяин, домохозяин, квартирохозяин, сохозяин, судохозяин).
    declension!("хозяин", 5, "~ин";
        nominative: "~ин", "~ева";
        genitive: "~ина", "~ев";
        dative: "~ину", "~евам";
        accusative: referred, referred;
        instrumental: "~ином", "~евами";
        prepositional: "~ине", "~евах";
        noun_second_case: "~ина", "~ине";
    ),
    // Бега: 5 words (бега, торока, потроха, холода, севера).
    declension!("бега", 5, "~а";
        nominative: "~а", "~а";
        genitive: "~ы", "~ов";
        dative: "~е", "~ам";
        accusative: "~у", referred;
        instrumental: "~ой", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~ы", "~е";
    ),
    // Получеловек: 5 words (получеловек, обезьяночеловек, первочеловек, предчеловек, транс-человек).
    declension!("получеловек", 5, "~че~овек";
        nominative: "~че~овек", "~~юди";
        genitive: "~че~овека", "~~юдей";
        dative: "~че~овеку", "~~юдям";
        accusative: referred, referred;
        instrumental: "~че~овеком", "~~юдьми";
        prepositional: "~че~овеке", "~~юдях";
        noun_second_case: "~че~овека", "~че~овеке";
    ),
    // Ботиночек: 5 words (ботиночек, затылочек, катерочек, валеночек, чулочек).
    declension!("ботиночек", 5, "~е~";
        nominative: "~е~", "~~и";
        genitive: "~~а", "~е~";
        dative: "~~у", "~~ам";
        accusative: referred, referred;
        instrumental: "~~ом", "~~ами";
        prepositional: "~~е", "~~ах";
        noun_second_case: "~~а", "~~е";
    ),
    // Чернорубашечник: 5 words (чернорубашечник, австралопитек, питек, укропитек, козлятник).
    declension!("чернорубашечник", 5, "~";
        nominative: "~", "~и";
        genitive: "~а", "~ов";
        dative: "~у", "~ам";
        accusative: "~", "~и";
        instrumental: "~ом", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~а", "~е";
    ),
    // Полкроны: 5 words (полкроны, полстраны, полшколы, ползалупы, полпланеты).
    declension!("полкроны", 5, "~~ы";
        nominative: "~~ы", "~у~ы";
        genitive: "~у~ы", "~у~";
        dative: "~у~е", "~у~ам";
        accusative: referred, referred;
        instrumental: "~у~ой", "~у~ами";
        prepositional: "~у~е", "~у~ах";
        noun_second_case: "~у~ы", "~у~е";
    ),
    // Тиса: 5 words (линда, шила, тиса, драва, пуста).
    declension!("тиса", 5, "~а";
        nominative: "~а", "~а";
        genitive: "~ы", "~";
        dative: "~е", "~а";
        accusative: "~у", referred;
        instrumental: "~ой", "~а";
        prepositional: "~е", "~а";
        noun_second_case: "~ы", "~е";
    ),
    // Копьё: 4 words (копьё, тряпьё, тупьё, репьё).
    declension!("копьё", 4, "~ьё";
        nominative: "~ьё", "~ья";
        genitive: "~ья", "~ий";
        dative: "~ью", "~ьям";
        accusative: referred, referred;
        instrumental: "~ьём", "~ьями";
        prepositional: "~ье", "~ьях";
        noun_second_case: "~ья", "~ье";
    ),
    // Опий: 4 words (гравий, опий, морфий, цикорий).
    declension!("опий", 4, "~й";
        nominative: "~й", "~и";
        genitive: "~я", "~ев";
        dative: "~ю", "~ям";
        accusative: referred, referred;
        instrumental: "~ем", "~ями";
        prepositional: "~и", "~ях";
        noun_second_case: "~ю", "~и";
    ),
    // Судно: 4 words (судно, агитсудно, авиасудно, фальшдно).
    declension!("судно", 4, "~но";
        nominative: "~но", "~а";
        genitive: "~на", "~ов";
        dative: "~ну", "~ам";
        accusative: referred, referred;
        instrumental: "~ном", "~ами";
        prepositional: "~не", "~ах";
        noun_second_case: "~на", "~не";
    ),
    // Улей: 4 words (улей, жеребей, чирей, ирей).
    declension!("улей", 4, "~ей";
        nominative: "~ей", "~ьи";
        genitive: "~ья", "~ьев";
        dative: "~ью", "~ьям";
        accusative: referred, referred;
        instrumental: "~ьем", "~ьями";
        prepositional: "~ье", "~ьях";
        noun_second_case: "~ья", "~ье";
    ),
    // Учащийся: 4 words (учащийся, трудящийся, вольноопределяющийся, праздношатающийся).
    declension!("учащийся", 4, "~ий~";
        nominative: "~ий~", "~ие~";
        genitive: "~его~", "~их~";
        dative: "~ему~", "~им~";
        accusative: referred, referred;
        instrumental: "~им~", "~ими~";
        prepositional: "~ем~", "~их~";
        noun_second_case: "~его~", "~ем~";
    ),
    // Бугай: 4 words (холуй, лишай, бугай, валуй).
    declension!("бугай", 4, "~й";
        nominative: "~й", "~и";
        genitive: "~я", "~ёв";
        dative: "~ю", "~ям";
        accusative: referred, referred;
        instrumental: "~ём", "~ями";
        prepositional: "~е", "~ях";
        noun_second_case: "~я", "~е";
    ),
    // Капли: 4 words (грабли, капли, чапли, велотуфли).
    declension!("капли", 4, "~~и";
        nominative: "~~я", "~~и";
        genitive: "~~и", "~е~ь";
        dative: "~~е", "~~ям";
        accusative: "~~ю", referred;
        instrumental: "~~ей", "~~ями";
        prepositional: "~~е", "~~ях";
        noun_second_case: "~~и", "~~е";
    ),
    // Пресмыкающееся: 4 words (пресмыкающееся, случившееся, сбывшееся, несбывшееся).
    declension!("пресмыкающееся", 4, "~ее~";
        nominative: "~ее~", "~ие~";
        genitive: "~его~", "~их~";
        dative: "~ему~", "~им~";
        accusative: "~ее~", referred;
        instrumental: "~им~", "~ими~";
        prepositional: "~ем~", "~их~";
        noun_second_case: "~его~", "~ем~";
    ),
    // Ничья: 4 words (ничья, третья, девичья, моя).
    declension!("ничья", 4, "~я";
        nominative: "~я", "~и";
        genitive: "~ей", "~их";
        dative: "~ей", "~им";
        accusative: "~ю", referred;
        instrumental: "~ей|~ею", "~ими";
        prepositional: "~ей", "~их";
        noun_second_case: "~ей", "~ей";
    ),
    // Яйки: 4 words (обойки, валуйки, яйки, прощайки).
    declension!("яйки", 4, "~й~и";
        nominative: "~й~а", "~й~и";
        genitive: "~й~и", "~е~";
        dative: "~й~е", "~й~ам";
        accusative: "~й~у", referred;
        instrumental: "~й~ой", "~й~ами";
        prepositional: "~й~е", "~й~ах";
        noun_second_case: "~й~и", "~й~е";
    ),
    // Горошек: 4 words (горошек, лишек, песочек, чаёчек).
    declension!("горошек", 4, "~е~";
        nominative: "~е~", "~~и";
        genitive: "~~а", "~~ов";
        dative: "~~у", "~~ам";
        accusative: referred, referred;
        instrumental: "~~ом", "~~ами";
        prepositional: "~~е", "~~ах";
        noun_second_case: "~~у", "~~е";
    ),
    // Сынуля: 4 words (сынуля, саня, меля, игруля).
    declension!("сынуля", 4, "~я";
        nominative: "~я", "~и";
        genitive: "~и", "~ь";
        dative: "~е", "~ям";
        accusative: "~ю", "~и";
        instrumental: "~ей|~ею", "~ями";
        prepositional: "~е", "~ях";
        noun_second_case: "~и", "~е";
    ),
    // Дитятко: 4 words (дитятко, старичонко, золотко, мужичонко).
    declension!("дитятко", 4, "~~о";
        nominative: "~~о", "~~и";
        genitive: "~~а", "~о~";
        dative: "~~у", "~~ам";
        accusative: "~~о", referred;
        instrumental: "~~ом", "~~ами";
        prepositional: "~~е", "~~ах";
        noun_second_case: "~~а", "~~е";
    ),
    // Земля: 3 words (земля, петля, полупетля).
    declension!("земля", 3, "~~я";
        nominative: "~~я", "~~и";
        genitive: "~~и", "~е~ь";
        dative: "~~е", "~~ям";
        accusative: "~~ю", referred;
        instrumental: "~~ёй|~~ёю", "~~ями";
        prepositional: "~~е", "~~ях";
        noun_second_case: "~~и", "~~е";
    ),
    // Путь: 3 words (путь, автопуть, севморпуть).
    declension!("путь", 3, "~ь";
        nominative: "~ь", "~и";
        genitive: "~и", "~ей";
        dative: "~и", "~ям";
        accusative: referred, referred;
        instrumental: "~ём", "~ями";
        prepositional: "~и", "~ях";
        noun_second_case: "~и", "~и";
    ),
    // Угол: 3 words (рот, лоб, угол).
    declension!("угол", 3, "~о~";
        nominative: "~о~", "~~ы";
        genitive: "~~а", "~~ов";
        dative: "~~у", "~~ам";
        accusative: referred, referred;
        instrumental: "~~ом", "~~ами";
        prepositional: "~~е", "~~ах";
        noun_second_case: "~~а", "~~у";
    ),
    // Сколько: 3 words (сколько, столько, несколько).
    declension!("сколько", 3, "~о";
        nominative: "~о", "~о";
        genitive: "~а", "~их|~их";
        dative: "~у", "~им|~им";
        accusative: referred, referred;
        instrumental: "~ом", "~ими|~ими";
        prepositional: "~е", "~их|~их";
        noun_second_case: "~а", "~е";
    ),
    // Чудо: 3 words (чудо, древо, худо).
    declension!("чудо", 3, "~о";
        nominative: "~о", "~еса";
        genitive: "~а", "~ес";
        dative: "~у", "~есам";
        accusative: referred, referred;
        instrumental: "~ом", "~есами";
        prepositional: "~е", "~есах";
        noun_second_case: "~а", "~е";
    ),
    // Средостениё: 3 words (мумиё, средостениё, лезвеё).
    declension!("средостениё", 3, "~ё";
        nominative: "~ё", "~я";
        genitive: "~я", "~й";
        dative: "~ю", "~ям";
        accusative: referred, referred;
        instrumental: "~ём", "~ями";
        prepositional: "~е", "~ях";
        noun_second_case: "~я", "~е";
    ),
    // Смерть: 3 words (смерть, дрянь, мразь).
    declension!("смерть", 3, "~ь";
        nominative: "~ь", "~и";
        genitive: "~и", "~ей";
        dative: "~и", "~ям";
        accusative: referred, "~ей|~и";
        instrumental: "~ью", "~ями";
        prepositional: "~и", "~ях";
        noun_second_case: "~и", "~и";
    ),
    // Кисель: 3 words (кисель, щавель, ревень).
    declension!("кисель", 3, "~ь";
        nominative: "~ь", "~и";
        genitive: "~я", "~ей";
        dative: "~ю", "~ям";
        accusative: referred, referred;
        instrumental: "~ём", "~ями";
        prepositional: "~е", "~ях";
        noun_second_case: "~ю", "~е";
    ),
    // Епитимья: 3 words (ектенья, паремья, епитимья).
    declension!("епитимья", 3, "~ья";
        nominative: "~ья", "~ьи";
        genitive: "~ьи", "~ий";
        dative: "~ье", "~ьям";
        accusative: "~ью", referred;
        instrumental: "~ьёй|~ьёю", "~ьями";
        prepositional: "~ье", "~ьях";
        noun_second_case: "~ьи", "~ье";
    ),
    // Дверца: 3 words (дверца, шпорца, мадерца).
    declension!("дверца", 3, "~~а";
        nominative: "~~а", "~~ы";
        genitive: "~~ы", "~е~";
        dative: "~~е", "~~ам";
        accusative: "~~у", referred;
        instrumental: "~~ей|~~ею", "~~ами";
        prepositional: "~~е", "~~ах";
        noun_second_case: "~~ы", "~~е";
    ),
    // Солнышко: 3 words (солнышко, нещечко, студентишко).
    declension!("солнышко", 3, "~~о";
        nominative: "~~о", "~~и";
        genitive: "~~а", "~е~";
        dative: "~~у", "~~ам";
        accusative: "~~о", referred;
        instrumental: "~~ом", "~~ами";
        prepositional: "~~е", "~~ах";
        noun_second_case: "~~а", "~~е";
    ),
    // Морячка: 3 words (морячка, рыбачка, тестюшка).
    declension!("морячка", 3, "~~а";
        nominative: "~~а", "~~и";
        genitive: "~~и", "~е~";
        dative: "~~е", "~~ам";
        accusative: "~~у", "~~и";
        instrumental: "~~ой|~~ою", "~~ами";
        prepositional: "~~е", "~~ах";
        noun_second_case: "~~и", "~~е";
    ),
    // Поллеса: 3 words (пол-леса, пол-округа, пол-острова).
    declension!("пол-леса", 3, "~-~а";
        nominative: "~-~а", "~у~а";
        genitive: "~у~а", "~у~ов";
        dative: "~у~у", "~у~ам";
        accusative: referred, referred;
        instrumental: "~у~ом", "~у~ами";
        prepositional: "~у~е", "~у~ах";
        noun_second_case: "~у~а", "~у~е";
    ),
];
