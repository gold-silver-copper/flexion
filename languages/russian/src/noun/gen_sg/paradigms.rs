//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's animacy says):
/// `nominative` (pl)
/// `genitive` (sg, pl)
/// `dative` (sg, pl)
/// `accusative` (pl)
/// `instrumental` (sg, pl)
/// `prepositional` (sg, pl)
/// `noun_second_case` (part, loc)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal;
     nominative: $c0:tt;
     genitive: $c1:tt, $c2:tt;
     dative: $c3:tt, $c4:tt;
     accusative: $c5:tt;
     instrumental: $c6:tt, $c7:tt;
     prepositional: $c8:tt, $c9:tt;
     noun_second_case: $c10:tt, $c11:tt;
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
            ]),
        }
    };
}

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `приёма`: 28,106 words.
    Приёма,
    /// Like `очка`: 11,676 words.
    Очка,
    /// Like `скопления`: 9,069 words.
    Скопления,
    /// Like `склонности`: 7,553 words.
    Склонности,
    /// Like `полноты`: 7,433 words.
    Полноты,
    /// Like `стрелки`: 6,662 words.
    Стрелки,
    /// Like `химии`: 6,587 words.
    Химии,
    /// Like `какао`: 3,358 words.
    Какао,
    /// Like `олимпийца`: 2,661 words.
    Олимпийца,
    /// Like `хлопанья`: 2,659 words.
    Хлопанья,
    /// Like `шоколадницы`: 2,597 words.
    Шоколадницы,
    /// Like `дорожки`: 2,516 words.
    Дорожки,
    /// Like `сходства`: 2,212 words.
    Сходства,
    /// Like `тяги`: 2,136 words.
    Тяги,
    /// Like `уровня`: 2,108 words.
    Уровня,
    /// Like `товарища`: 944 words.
    Товарища,
    /// Like `выдачи`: 688 words.
    Выдачи,
    /// Like `змея`: 659 words.
    Змея,
    /// Like `гения`: 635 words.
    Гения,
    /// Like `чайки`: 474 words.
    Чайки,
    /// Like `второго`: 472 words.
    Второго,
    /// Like `барина`: 395 words.
    Барина,
    /// Like `ложа`: 343 words.
    Ложа,
    /// Like `накладной`: 342 words.
    Накладной,
    /// Like `ключа`: 317 words.
    Ключа,
    /// Like `червя`: 256 words.
    Червя,
    /// Like `люльки`: 255 words.
    Люльки,
    /// Like `бани`: 236 words.
    Бани,
    /// Like `русского`: 233 words.
    Русского,
    /// Like `батареи`: 195 words.
    Батареи,
    /// Like `постава`: 177 words.
    Постава,
    /// Like `горючего`: 157 words.
    Горючего,
    /// Like `рожка`: 151 words.
    Рожка,
    /// Like `фофудьи`: 148 words.
    Фофудьи,
    /// Like `орлёнка`: 126 words.
    Орлёнка,
    /// Like `зари`: 124 words.
    Зари,
    /// Like `подворотни`: 121 words.
    Подворотни,
    /// Like `овцы`: 114 words.
    Овцы,
    /// Like `жнивья`: 113 words.
    Жнивья,
    /// Like `спальни`: 85 words.
    Спальни,
    /// Like `хода`: 79 words.
    Хода,
    /// Like `помочи`: 79 words.
    Помочи,
    /// Like `мастерской`: 64 words.
    Мастерской,
    /// Like `внучонка`: 57 words.
    Внучонка,
    /// Like `съезжей`: 54 words.
    Съезжей,
    /// Like `ядра`: 52 words.
    Ядра,
    /// Like `пейзана`: 51 words.
    Пейзана,
    /// Like `положняка`: 51 words.
    Положняка,
    /// Like `свечи`: 49 words.
    Свечи,
    /// Like `каракули`: 43 words.
    Каракули,
    /// Like `хуища`: 39 words.
    Хуища,
    /// Like `моря`: 33 words.
    Моря,
    /// Like `пера`: 31 words.
    Пера,
    /// Like `ладьи`: 30 words.
    Ладьи,
    /// Like `щита`: 29 words.
    Щита,
    /// Like `вереи`: 29 words.
    Вереи,
    /// Like `порубки`: 28 words.
    Порубки,
    /// Like `деревни`: 27 words.
    Деревни,
    /// Like `звезды`: 23 words.
    Звезды,
    /// Like `члена`: 23 words.
    Члена,
    /// Like `маракуйи`: 23 words.
    Маракуйи,
    /// Like `валенка`: 22 words.
    Валенка,
    /// Like `устья`: 20 words.
    Устья,
    /// Like `ребра`: 20 words.
    Ребра,
    /// Like `хуя`: 20 words.
    Хуя,
    /// Like `носа`: 17 words.
    Носа,
    /// Like `зла`: 16 words.
    Зла,
    /// Like `анкары`: 16 words.
    Анкары,
    /// Like `бугая`: 15 words.
    Бугая,
    /// Like `бойни`: 15 words.
    Бойни,
    /// Like `губоцветных`: 15 words.
    Губоцветных,
    /// Like `племени`: 14 words.
    Племени,
    /// Like `котёночка`: 13 words.
    Котёночка,
    /// Like `мериноса`: 13 words.
    Мериноса,
    /// Like `кокцинеллиды`: 13 words.
    Кокцинеллиды,
    /// Like `сердца`: 12 words.
    Сердца,
    /// Like `гнезда`: 12 words.
    Гнезда,
    /// Like `счёта`: 11 words.
    Счёта,
    /// Like `усадьбы`: 10 words.
    Усадьбы,
    /// Like `кольца`: 10 words.
    Кольца,
    /// Like `рыльца`: 10 words.
    Рыльца,
    /// Like `дульца`: 10 words.
    Дульца,
    /// Like `дома`: 9 words.
    Дома,
    /// Like `духа`: 9 words.
    Духа,
    /// Like `роста`: 9 words.
    Роста,
    /// Like `калашникова`: 9 words.
    Калашникова,
    /// Like `куклы`: 8 words.
    Куклы,
    /// Like `покоя`: 8 words.
    Покоя,
    /// Like `пресмыкающегося`: 8 words.
    Пресмыкающегося,
    /// Like `герца`: 8 words.
    Герца,
    /// Like `плюхина`: 8 words.
    Плюхина,
    /// Like `плеча`: 7 words.
    Плеча,
    /// Like `камня`: 7 words.
    Камня,
    /// Like `череды`: 7 words.
    Череды,
    /// Like `весны`: 7 words.
    Весны,
    /// Like `королька`: 7 words.
    Королька,
    /// Like `боя`: 6 words.
    Боя,
    /// Like `стекла`: 6 words.
    Стекла,
    /// Like `татарина`: 6 words.
    Татарина,
    /// Like `чукчи`: 6 words.
    Чукчи,
    /// Like `метёлки`: 6 words.
    Метёлки,
    /// Like `коленца`: 6 words.
    Коленца,
    /// Like `исайи`: 6 words.
    Исайи,
    /// Like `круга`: 5 words.
    Круга,
    /// Like `кухни`: 5 words.
    Кухни,
    /// Like `хозяина`: 5 words.
    Хозяина,
    /// Like `чернорубашечника`: 5 words.
    Чернорубашечника,
    /// Like `любви`: 4 words.
    Любви,
    /// Like `опия`: 4 words.
    Опия,
    /// Like `того`: 4 words.
    Того,
    /// Like `судна`: 4 words.
    Судна,
    /// Like `ничьей`: 4 words.
    Ничьей,
    /// Like `щенка`: 3 words.
    Щенка,
    /// Like `носка`: 3 words.
    Носка,
    /// Like `земли`: 3 words.
    Земли,
    /// Like `пути`: 3 words.
    Пути,
    /// Like `семени`: 3 words.
    Семени,
    /// Like `лжи`: 3 words.
    Лжи,
    /// Like `смерти`: 3 words.
    Смерти,
    /// Like `киселя`: 3 words.
    Киселя,
    /// Like `епитимьи`: 3 words.
    Епитимьи,
    /// Like `дверцы`: 3 words.
    Дверцы,
    /// Like `получеловека`: 3 words.
    Получеловека,
    /// Like `морячки`: 3 words.
    Морячки,
    /// Like `сынули`: 3 words.
    Сынули,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 125] = [
        Declension::Приёма,
        Declension::Очка,
        Declension::Скопления,
        Declension::Склонности,
        Declension::Полноты,
        Declension::Стрелки,
        Declension::Химии,
        Declension::Какао,
        Declension::Олимпийца,
        Declension::Хлопанья,
        Declension::Шоколадницы,
        Declension::Дорожки,
        Declension::Сходства,
        Declension::Тяги,
        Declension::Уровня,
        Declension::Товарища,
        Declension::Выдачи,
        Declension::Змея,
        Declension::Гения,
        Declension::Чайки,
        Declension::Второго,
        Declension::Барина,
        Declension::Ложа,
        Declension::Накладной,
        Declension::Ключа,
        Declension::Червя,
        Declension::Люльки,
        Declension::Бани,
        Declension::Русского,
        Declension::Батареи,
        Declension::Постава,
        Declension::Горючего,
        Declension::Рожка,
        Declension::Фофудьи,
        Declension::Орлёнка,
        Declension::Зари,
        Declension::Подворотни,
        Declension::Овцы,
        Declension::Жнивья,
        Declension::Спальни,
        Declension::Хода,
        Declension::Помочи,
        Declension::Мастерской,
        Declension::Внучонка,
        Declension::Съезжей,
        Declension::Ядра,
        Declension::Пейзана,
        Declension::Положняка,
        Declension::Свечи,
        Declension::Каракули,
        Declension::Хуища,
        Declension::Моря,
        Declension::Пера,
        Declension::Ладьи,
        Declension::Щита,
        Declension::Вереи,
        Declension::Порубки,
        Declension::Деревни,
        Declension::Звезды,
        Declension::Члена,
        Declension::Маракуйи,
        Declension::Валенка,
        Declension::Устья,
        Declension::Ребра,
        Declension::Хуя,
        Declension::Носа,
        Declension::Зла,
        Declension::Анкары,
        Declension::Бугая,
        Declension::Бойни,
        Declension::Губоцветных,
        Declension::Племени,
        Declension::Котёночка,
        Declension::Мериноса,
        Declension::Кокцинеллиды,
        Declension::Сердца,
        Declension::Гнезда,
        Declension::Счёта,
        Declension::Усадьбы,
        Declension::Кольца,
        Declension::Рыльца,
        Declension::Дульца,
        Declension::Дома,
        Declension::Духа,
        Declension::Роста,
        Declension::Калашникова,
        Declension::Куклы,
        Declension::Покоя,
        Declension::Пресмыкающегося,
        Declension::Герца,
        Declension::Плюхина,
        Declension::Плеча,
        Declension::Камня,
        Declension::Череды,
        Declension::Весны,
        Declension::Королька,
        Declension::Боя,
        Declension::Стекла,
        Declension::Татарина,
        Declension::Чукчи,
        Declension::Метёлки,
        Declension::Коленца,
        Declension::Исайи,
        Declension::Круга,
        Declension::Кухни,
        Declension::Хозяина,
        Declension::Чернорубашечника,
        Declension::Любви,
        Declension::Опия,
        Declension::Того,
        Declension::Судна,
        Declension::Ничьей,
        Declension::Щенка,
        Declension::Носка,
        Declension::Земли,
        Declension::Пути,
        Declension::Семени,
        Declension::Лжи,
        Declension::Смерти,
        Declension::Киселя,
        Declension::Епитимьи,
        Declension::Дверцы,
        Declension::Получеловека,
        Declension::Морячки,
        Declension::Сынули,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 125] = [
    // Приёма: 28,106 words (указа, зонта, кота, узуса, скота, мэра).
    declension!("приёма", 28106, "~а";
        nominative: "~ы";
        genitive: "~а", "~ов";
        dative: "~у", "~ам";
        accusative: referred;
        instrumental: "~ом", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~а", "~е";
    ),
    // Очка: 11,676 words (зонтика, бортника, бочага, бредника, языка, гулага).
    declension!("очка", 11676, "~а";
        nominative: "~и";
        genitive: "~а", "~ов";
        dative: "~у", "~ам";
        accusative: referred;
        instrumental: "~ом", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~а", "~е";
    ),
    // Скопления: 9,069 words (произношения, лезвия, распятия, деревенения, лесонасаждения, ведения).
    declension!("скопления", 9069, "~я";
        nominative: "~я";
        genitive: "~я", "~й";
        dative: "~ю", "~ям";
        accusative: referred;
        instrumental: "~ем", "~ями";
        prepositional: "~и", "~ях";
        noun_second_case: "~я", "~и";
    ),
    // Склонности: 7,553 words (знати, веси, гласности, деревянности, лошади, матери).
    declension!("склонности", 7553, "~и";
        nominative: "~и";
        genitive: "~и", "~ей";
        dative: "~и", "~ям";
        accusative: referred;
        instrumental: "~ью", "~ями";
        prepositional: "~и", "~ях";
        noun_second_case: "~и", "~и";
    ),
    // Полноты: 7,433 words (буявы, ваты, работы, камеры, липы, серы).
    declension!("полноты", 7433, "~ы";
        nominative: "~ы";
        genitive: "~ы", "~";
        dative: "~е", "~ам";
        accusative: referred;
        instrumental: "~ой|~ою", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~ы", "~е";
    ),
    // Стрелки: 6,662 words (водки, цыпки, жилки, деревянки, лесопилки, доски).
    declension!("стрелки", 6662, "~~и";
        nominative: "~~и";
        genitive: "~~и", "~о~";
        dative: "~~е", "~~ам";
        accusative: referred;
        instrumental: "~~ой|~~ою", "~~ами";
        prepositional: "~~е", "~~ах";
        noun_second_case: "~~и", "~~е";
    ),
    // Химии: 6,587 words (анархии, лесохимии, библии, биологии, академии, армии).
    declension!("химии", 6587, "~и";
        nominative: "~и";
        genitive: "~и", "~й";
        dative: "~и", "~ям";
        accusative: referred;
        instrumental: "~ей|~ею", "~ями";
        prepositional: "~и", "~ях";
        noun_second_case: "~и", "~и";
    ),
    // Какао: 3,358 words (пальто, ничего, кое-чего, адажио, какао, полвторого).
    declension!("какао", 3358, "~";
        nominative: "~";
        genitive: "~", "~";
        dative: "~", "~";
        accusative: referred;
        instrumental: "~", "~";
        prepositional: "~", "~";
        noun_second_case: "~", "~";
    ),
    // Олимпийца: 2,661 words (пальца, левантинца, древоточца, марганца, месяца, самозванца).
    declension!("олимпийца", 2661, "~а";
        nominative: "~ы";
        genitive: "~а", "~ев";
        dative: "~у", "~ам";
        accusative: referred;
        instrumental: "~ем", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~а", "~е";
    ),
    // Хлопанья: 2,659 words (запястья, воскресенья, копья, варенья, воспитанья, отчаянья).
    declension!("хлопанья", 2659, "~ья";
        nominative: "~ья";
        genitive: "~ья", "~ий";
        dative: "~ью", "~ьям";
        accusative: referred;
        instrumental: "~ьем", "~ьями";
        prepositional: "~ье", "~ьях";
        noun_second_case: "~ья", "~ье";
    ),
    // Шоколадницы: 2,597 words (былицы, масленицы, царицы, лисицы, птицы, гусеницы).
    declension!("шоколадницы", 2597, "~ы";
        nominative: "~ы";
        genitive: "~ы", "~";
        dative: "~е", "~ам";
        accusative: referred;
        instrumental: "~ей|~ею", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~ы", "~е";
    ),
    // Дорожки: 2,516 words (кошки, былички, палочки, ручки, мошки, бабочки).
    declension!("дорожки", 2516, "~~и";
        nominative: "~~и";
        genitive: "~~и", "~е~";
        dative: "~~е", "~~ам";
        accusative: referred;
        instrumental: "~~ой|~~ою", "~~ами";
        prepositional: "~~е", "~~ах";
        noun_second_case: "~~и", "~~е";
    ),
    // Сходства: 2,212 words (буева, бёрда, лета, жала, глаза, горла).
    declension!("сходства", 2212, "~а";
        nominative: "~а";
        genitive: "~а", "~";
        dative: "~у", "~ам";
        accusative: referred;
        instrumental: "~ом", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~а", "~е";
    ),
    // Тяги: 2,136 words (собаки, муки, книги, керамики, ваги, республики).
    declension!("тяги", 2136, "~и";
        nominative: "~и";
        genitive: "~и", "~";
        dative: "~е", "~ам";
        accusative: referred;
        instrumental: "~ой|~ою", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~и", "~е";
    ),
    // Уровня: 2,108 words (бредня, сшивателя, апреля, июня, июля, оленя).
    declension!("уровня", 2108, "~я";
        nominative: "~и";
        genitive: "~я", "~ей";
        dative: "~ю", "~ям";
        accusative: referred;
        instrumental: "~ем", "~ями";
        prepositional: "~е", "~ях";
        noun_second_case: "~я", "~е";
    ),
    // Товарища: 944 words (харча, туша, треша, ража, товарища, дервиша).
    declension!("товарища", 944, "~а";
        nominative: "~и";
        genitive: "~а", "~ей";
        dative: "~у", "~ам";
        accusative: referred;
        instrumental: "~ем", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~а", "~е";
    ),
    // Выдачи: 688 words (ладоши, кожи, чаши, мульчи, грыжи, дикуши).
    declension!("выдачи", 688, "~и";
        nominative: "~и";
        genitive: "~и", "~";
        dative: "~е", "~ам";
        accusative: referred;
        instrumental: "~ей|~ею", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~и", "~е";
    ),
    // Змея: 659 words (мая, гея, вертухая, елея, героя, случая).
    declension!("змея", 659, "~я";
        nominative: "~и";
        genitive: "~я", "~ев";
        dative: "~ю", "~ям";
        accusative: referred;
        instrumental: "~ем", "~ями";
        prepositional: "~е", "~ях";
        noun_second_case: "~я", "~е";
    ),
    // Гения: 635 words (калия, меркурия, гения, бериллия, гелия, лития).
    declension!("гения", 635, "~я";
        nominative: "~и";
        genitive: "~я", "~ев";
        dative: "~ю", "~ям";
        accusative: referred;
        instrumental: "~ем", "~ями";
        prepositional: "~и", "~ях";
        noun_second_case: "~я", "~и";
    ),
    // Чайки: 474 words (копейки, скамейки, перестройки, ищейки, лайки, австрийки).
    declension!("чайки", 474, "~й~и";
        nominative: "~й~и";
        genitive: "~й~и", "~е~";
        dative: "~й~е", "~й~ам";
        accusative: referred;
        instrumental: "~й~ой|~й~ою", "~й~ами";
        prepositional: "~й~е", "~й~ах";
        noun_second_case: "~й~и", "~й~е";
    ),
    // Второго: 472 words (белого, чёрного, былого, гласного, беспризорного, коридорного).
    declension!("второго", 472, "~ого";
        nominative: "~ые";
        genitive: "~ого", "~ых";
        dative: "~ому", "~ым";
        accusative: referred;
        instrumental: "~ым", "~ыми";
        prepositional: "~ом", "~ых";
        noun_second_case: "~ого", "~ом";
    ),
    // Барина: 395 words (господина, англичанина, россиянина, опочанина, гражданина, новоангличанина).
    declension!("барина", 395, "~ина";
        nominative: "~е";
        genitive: "~ина", "~";
        dative: "~ину", "~ам";
        accusative: referred;
        instrumental: "~ином", "~ами";
        prepositional: "~ине", "~ах";
        noun_second_case: "~ина", "~ине";
    ),
    // Ложа: 343 words (солнца, веча, училища, кладбища, ложа, пристанища).
    declension!("ложа", 343, "~а";
        nominative: "~а";
        genitive: "~а", "~";
        dative: "~у", "~ам";
        accusative: referred;
        instrumental: "~ем", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~а", "~е";
    ),
    // Накладной: 342 words (деревообделочной, гостиной, ванной, кладовой, уборной, борзой).
    declension!("накладной", 342, "~ой";
        nominative: "~ые";
        genitive: "~ой", "~ых";
        dative: "~ой", "~ым";
        accusative: referred;
        instrumental: "~ой|~ою", "~ыми";
        prepositional: "~ой", "~ых";
        noun_second_case: "~ой", "~ой";
    ),
    // Ключа: 317 words (ежа, борща, карандаша, врача, сыча, ножа).
    declension!("ключа", 317, "~а";
        nominative: "~и";
        genitive: "~а", "~ей";
        dative: "~у", "~ам";
        accusative: referred;
        instrumental: "~ом", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~а", "~е";
    ),
    // Червя: 256 words (царя, рубля, кремля, дождя, дня, января).
    declension!("червя", 256, "~я";
        nominative: "~и";
        genitive: "~я", "~ей";
        dative: "~ю", "~ям";
        accusative: referred;
        instrumental: "~ём", "~ями";
        prepositional: "~е", "~ях";
        noun_second_case: "~я", "~е";
    ),
    // Люльки: 255 words (пеньки, пиписьки, кухоньки, батеньки, кальки, свистульки).
    declension!("люльки", 255, "~ь~и";
        nominative: "~ь~и";
        genitive: "~ь~и", "~е~";
        dative: "~ь~е", "~ь~ам";
        accusative: referred;
        instrumental: "~ь~ой|~ь~ою", "~ь~ами";
        prepositional: "~ь~е", "~ь~ах";
        noun_second_case: "~ь~и", "~ь~е";
    ),
    // Бани: 236 words (братыни, недели, бури, няни, княгини, тёти).
    declension!("бани", 236, "~и";
        nominative: "~и";
        genitive: "~и", "~ь";
        dative: "~е", "~ям";
        accusative: referred;
        instrumental: "~ей|~ею", "~ями";
        prepositional: "~е", "~ях";
        noun_second_case: "~и", "~е";
    ),
    // Русского: 233 words (русского, этого, сербскохорватского, английского, норвежского, китайского).
    declension!("русского", 233, "~ого";
        nominative: "~ие";
        genitive: "~ого", "~их";
        dative: "~ому", "~им";
        accusative: referred;
        instrumental: "~им", "~ими";
        prepositional: "~ом", "~их";
        noun_second_case: "~ого", "~ом";
    ),
    // Батареи: 195 words (статуи, стаи, шеи, идеи, затеи, феи).
    declension!("батареи", 195, "~и";
        nominative: "~и";
        genitive: "~и", "~й";
        dative: "~е", "~ям";
        accusative: referred;
        instrumental: "~ей|~ею", "~ями";
        prepositional: "~е", "~ях";
        noun_second_case: "~и", "~е";
    ),
    // Постава: 177 words (города, облака, пояса, адреса, бока, вечера).
    declension!("постава", 177, "~а";
        nominative: "~а";
        genitive: "~а", "~ов";
        dative: "~у", "~ам";
        accusative: referred;
        instrumental: "~ом", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~а", "~е";
    ),
    // Горючего: 157 words (лесничего, всего, чего, настоящего, рабочего, млекопитающего).
    declension!("горючего", 157, "~его";
        nominative: "~ие";
        genitive: "~его", "~их";
        dative: "~ему", "~им";
        accusative: referred;
        instrumental: "~им", "~ими";
        prepositional: "~ем", "~их";
        noun_second_case: "~его", "~ем";
    ),
    // Рожка: 151 words (семечка, яичка, рожка, окошка, зёрнышка, горлышка).
    declension!("рожка", 151, "~~а";
        nominative: "~~и";
        genitive: "~~а", "~е~";
        dative: "~~у", "~~ам";
        accusative: referred;
        instrumental: "~~ом", "~~ами";
        prepositional: "~~е", "~~ах";
        noun_second_case: "~~а", "~~е";
    ),
    // Фофудьи: 148 words (гостьи, авиаэскадрильи, эскадрильи, кельи, тортильи, колдуньи).
    declension!("фофудьи", 148, "~ьи";
        nominative: "~ьи";
        genitive: "~ьи", "~ий";
        dative: "~ье", "~ьям";
        accusative: referred;
        instrumental: "~ьей|~ьею", "~ьями";
        prepositional: "~ье", "~ьях";
        noun_second_case: "~ьи", "~ье";
    ),
    // Орлёнка: 126 words (телёнка, октябрёнка, поросёнка, жеребёнка, ягнёнка, козлёнка).
    declension!("орлёнка", 126, "~ёнка";
        nominative: "~ята";
        genitive: "~ёнка", "~ят";
        dative: "~ёнку", "~ятам";
        accusative: referred;
        instrumental: "~ёнком", "~ятами";
        prepositional: "~ёнке", "~ятах";
        noun_second_case: "~ёнка", "~ёнке";
    ),
    // Зари: 124 words (ступни, стези, простыни, тли, хуйни, брони).
    declension!("зари", 124, "~и";
        nominative: "~и";
        genitive: "~и", "~ей";
        dative: "~е", "~ям";
        accusative: referred;
        instrumental: "~ёй|~ёю", "~ями";
        prepositional: "~е", "~ях";
        noun_second_case: "~и", "~е";
    ),
    // Подворотни: 121 words (вишни, песни, бушни, пригоршни, башни, пекарни).
    declension!("подворотни", 121, "~~и";
        nominative: "~~и";
        genitive: "~~и", "~е~";
        dative: "~~е", "~~ям";
        accusative: referred;
        instrumental: "~~ей|~~ею", "~~ями";
        prepositional: "~~е", "~~ях";
        noun_second_case: "~~и", "~~е";
    ),
    // Овцы: 114 words (овцы, сосны, домны, царевны, гривны, кошмы).
    declension!("овцы", 114, "~~ы";
        nominative: "~~ы";
        genitive: "~~ы", "~е~";
        dative: "~~е", "~~ам";
        accusative: referred;
        instrumental: "~~ой|~~ою", "~~ами";
        prepositional: "~~е", "~~ах";
        noun_second_case: "~~ы", "~~е";
    ),
    // Жнивья: 113 words (питья, ружья, битья, вранья, сырья, жилья).
    declension!("жнивья", 113, "~ья";
        nominative: "~ья";
        genitive: "~ья", "~ей";
        dative: "~ью", "~ьям";
        accusative: referred;
        instrumental: "~ьём", "~ьями";
        prepositional: "~ье", "~ьях";
        noun_second_case: "~ья", "~ье";
    ),
    // Спальни: 85 words (лесопильни, спальни, читальни, колокольни, коптильни, наковальни).
    declension!("спальни", 85, "~ь~и";
        nominative: "~ь~и";
        genitive: "~ь~и", "~е~";
        dative: "~ь~е", "~ь~ям";
        accusative: referred;
        instrumental: "~ь~ей|~ь~ею", "~ь~ями";
        prepositional: "~ь~е", "~ь~ях";
        noun_second_case: "~ь~и", "~ь~е";
    ),
    // Хода: 79 words (укропа, самогона, града, народа, аппетита, яда).
    declension!("хода", 79, "~а";
        nominative: "~ы";
        genitive: "~а", "~ов";
        dative: "~у", "~ам";
        accusative: referred;
        instrumental: "~ом", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~у", "~е";
    ),
    // Помочи: 79 words (ночи, дитяти, мыши, мочи, туши, помочи).
    declension!("помочи", 79, "~и";
        nominative: "~и";
        genitive: "~и", "~ей";
        dative: "~и", "~ам";
        accusative: referred;
        instrumental: "~ью", "~ами";
        prepositional: "~и", "~ах";
        noun_second_case: "~и", "~и";
    ),
    // Мастерской: 64 words (детской, мастерской, мужской, русской, кондитерской, парикмахерской).
    declension!("мастерской", 64, "~ой";
        nominative: "~ие";
        genitive: "~ой", "~их";
        dative: "~ой", "~им";
        accusative: referred;
        instrumental: "~ой|~ою", "~ими";
        prepositional: "~ой", "~их";
        noun_second_case: "~ой", "~ой";
    ),
    // Внучонка: 57 words (медвежонка, индюшонка, крольчонка, внучонка, волчонка, мышонка).
    declension!("внучонка", 57, "~онка";
        nominative: "~ата";
        genitive: "~онка", "~ат";
        dative: "~онку", "~атам";
        accusative: referred;
        instrumental: "~онком", "~атами";
        prepositional: "~онке", "~атах";
        noun_second_case: "~онка", "~онке";
    ),
    // Съезжей: 54 words (передней, прихожей, преисподней, телеведущей, съезжей, неверующей).
    declension!("съезжей", 54, "~ей";
        nominative: "~ие";
        genitive: "~ей", "~их";
        dative: "~ей", "~им";
        accusative: referred;
        instrumental: "~ей|~ею", "~ими";
        prepositional: "~ей", "~их";
        noun_second_case: "~ей", "~ей";
    ),
    // Ядра: 52 words (брашна, говна, деревца, ядра, масла, числа).
    declension!("ядра", 52, "~~а";
        nominative: "~~а";
        genitive: "~~а", "~е~";
        dative: "~~у", "~~ам";
        accusative: referred;
        instrumental: "~~ом", "~~ами";
        prepositional: "~~е", "~~ах";
        noun_second_case: "~~а", "~~е";
    ),
    // Пейзана: 51 words (солдата, гусара, вольта, погона, волоса, ампера).
    declension!("пейзана", 51, "~а";
        nominative: "~ы";
        genitive: "~а", "~";
        dative: "~у", "~ам";
        accusative: referred;
        instrumental: "~ом", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~а", "~е";
    ),
    // Положняка: 51 words (страха, гороха, горошка, коньяка, лака, мака).
    declension!("положняка", 51, "~а";
        nominative: "~и";
        genitive: "~а", "~ов";
        dative: "~у", "~ам";
        accusative: referred;
        instrumental: "~ом", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~у", "~е";
    ),
    // Свечи: 49 words (левши, ханжи, вожжи, свечи, саранчи, лапши).
    declension!("свечи", 49, "~и";
        nominative: "~и";
        genitive: "~и", "~ей";
        dative: "~е", "~ам";
        accusative: referred;
        instrumental: "~ой|~ою", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~и", "~е";
    ),
    // Каракули: 43 words (бати, дяди, доли, ходули, сводни, пени).
    declension!("каракули", 43, "~и";
        nominative: "~и";
        genitive: "~и", "~ей";
        dative: "~е", "~ям";
        accusative: referred;
        instrumental: "~ей|~ею", "~ями";
        prepositional: "~е", "~ях";
        noun_second_case: "~и", "~е";
    ),
    // Хуища: 39 words (дружища, когтища, волчища, хуища, козлища, талантища).
    declension!("хуища", 39, "~а";
        nominative: "~и";
        genitive: "~а", "~";
        dative: "~у", "~ам";
        accusative: referred;
        instrumental: "~ем", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~а", "~е";
    ),
    // Моря: 33 words (моря, поля, учителя, якоря, флигеля, лагеря).
    declension!("моря", 33, "~я";
        nominative: "~я";
        genitive: "~я", "~ей";
        dative: "~ю", "~ям";
        accusative: referred;
        instrumental: "~ем", "~ями";
        prepositional: "~е", "~ях";
        noun_second_case: "~я", "~е";
    ),
    // Пера: 31 words (дерева, брата, пера, крыла, листа, мужа).
    declension!("пера", 31, "~а";
        nominative: "~ья";
        genitive: "~а", "~ьев";
        dative: "~у", "~ьям";
        accusative: referred;
        instrumental: "~ом", "~ьями";
        prepositional: "~е", "~ьях";
        noun_second_case: "~а", "~е";
    ),
    // Ладьи: 30 words (семьи, ладьи, статьи, судьи, скамьи, гольи).
    declension!("ладьи", 30, "~ьи";
        nominative: "~ьи";
        genitive: "~ьи", "~ей";
        dative: "~ье", "~ьям";
        accusative: referred;
        instrumental: "~ьёй|~ьёю", "~ьями";
        prepositional: "~ье", "~ьях";
        noun_second_case: "~ьи", "~ье";
    ),
    // Щита: 29 words (мира, льда, кона, пода, рта, льна).
    declension!("щита", 29, "~а";
        nominative: "~ы";
        genitive: "~а", "~ов";
        dative: "~у", "~ам";
        accusative: referred;
        instrumental: "~ом", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~а", "~у";
    ),
    // Вереи: 29 words (змеи, колеи, чешуи, швеи, струи, алии).
    declension!("вереи", 29, "~и";
        nominative: "~и";
        genitive: "~и", "~й";
        dative: "~е", "~ям";
        accusative: referred;
        instrumental: "~ёй|~ёю", "~ями";
        prepositional: "~е", "~ях";
        noun_second_case: "~и", "~е";
    ),
    // Порубки: 28 words (скоблянки, меньшевички, связистки, отклейки, порубки, самосброски).
    declension!("порубки", 28, "~и";
        nominative: "~и";
        genitive: "~и", "~а";
        dative: "~е", "~ам";
        accusative: referred;
        instrumental: "~ой|~ою", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~и", "~е";
    ),
    // Деревни: 27 words (торговли, сабли, капли, туфли, чапли, цапли).
    declension!("деревни", 27, "~~и";
        nominative: "~~и";
        genitive: "~~и", "~е~ь";
        dative: "~~е", "~~ям";
        accusative: referred;
        instrumental: "~~ей|~~ею", "~~ями";
        prepositional: "~~е", "~~ях";
        noun_second_case: "~~и", "~~е";
    ),
    // Звезды: 23 words (звезды, пчелы, жены, сестры, медсестры, бересты).
    declension!("звезды", 23, "~е~ы";
        nominative: "~ё~ы";
        genitive: "~е~ы", "~ё~";
        dative: "~е~е", "~ё~ам";
        accusative: referred;
        instrumental: "~е~ой|~е~ою", "~ё~ами";
        prepositional: "~е~е", "~ё~ах";
        noun_second_case: "~е~ы", "~е~е";
    ),
    // Члена: 23 words (клона, типа, члена, ката, кекса, баса).
    declension!("члена", 23, "~а";
        nominative: "~ы";
        genitive: "~а", "~ов";
        dative: "~у", "~ам";
        accusative: "~ов|~ы";
        instrumental: "~ом", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~а", "~е";
    ),
    // Маракуйи: 23 words (аллилуйи, секвойи, паранойи, папайи, пирайи, майи).
    declension!("маракуйи", 23, "~и";
        nominative: "~и";
        genitive: "~и", "~";
        dative: "~е", "~ям";
        accusative: referred;
        instrumental: "~ей|~ею", "~ями";
        prepositional: "~е", "~ях";
        noun_second_case: "~и", "~е";
    ),
    // Валенка: 22 words (ботинка, ведёрка, чулка, турка, валенка, глазка).
    declension!("валенка", 22, "~~а";
        nominative: "~~и";
        genitive: "~~а", "~о~";
        dative: "~~у", "~~ам";
        accusative: referred;
        instrumental: "~~ом", "~~ами";
        prepositional: "~~е", "~~ах";
        noun_second_case: "~~а", "~~е";
    ),
    // Устья: 20 words (платья, устья, подмастерья, подполья, уголья, отребья).
    declension!("устья", 20, "~я";
        nominative: "~я";
        genitive: "~я", "~ев";
        dative: "~ю", "~ям";
        accusative: referred;
        instrumental: "~ем", "~ями";
        prepositional: "~е", "~ях";
        noun_second_case: "~я", "~е";
    ),
    // Ребра: 20 words (ребра, весла, зерна, бедра, седла, ремесла).
    declension!("ребра", 20, "~е~~а";
        nominative: "~ё~~а";
        genitive: "~е~~а", "~ё~е~";
        dative: "~е~~у", "~ё~~ам";
        accusative: referred;
        instrumental: "~е~~ом", "~ё~~ами";
        prepositional: "~е~~е", "~ё~~ах";
        noun_second_case: "~е~~а", "~е~~е";
    ),
    // Хуя: 20 words (хуя, роя, слоя, буя, гая, пая).
    declension!("хуя", 20, "~я";
        nominative: "~и";
        genitive: "~я", "~ёв";
        dative: "~ю", "~ям";
        accusative: referred;
        instrumental: "~ем", "~ями";
        prepositional: "~е", "~ях";
        noun_second_case: "~я", "~е";
    ),
    // Носа: 17 words (часа, чада, пола, дуба, носа, сыра).
    declension!("носа", 17, "~а";
        nominative: "~ы";
        genitive: "~а", "~ов";
        dative: "~у", "~ам";
        accusative: referred;
        instrumental: "~ом", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~у", "~у";
    ),
    // Зла: 16 words (окна, зла, сукна, волокна, жалка, тягла).
    declension!("зла", 16, "~~а";
        nominative: "~~а";
        genitive: "~~а", "~о~";
        dative: "~~у", "~~ам";
        accusative: referred;
        instrumental: "~~ом", "~~ами";
        prepositional: "~~е", "~~ах";
        noun_second_case: "~~а", "~~е";
    ),
    // Анкары: 16 words (ковурмы, дисны, полумглы, мсты, ангары, анкары).
    declension!("анкары", 16, "~ы";
        nominative: "~ы";
        genitive: "~ы", "~а";
        dative: "~е", "~ам";
        accusative: referred;
        instrumental: "~ой|~ою", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~ы", "~е";
    ),
    // Бугая: 15 words (соловья, воробья, муравья, ручья, холуя, лишая).
    declension!("бугая", 15, "~я";
        nominative: "~и";
        genitive: "~я", "~ёв";
        dative: "~ю", "~ям";
        accusative: referred;
        instrumental: "~ём", "~ями";
        prepositional: "~е", "~ях";
        noun_second_case: "~я", "~е";
    ),
    // Бойни: 15 words (кофейни, бойни, водогрейни, воскобойни, двойни, маслобойни).
    declension!("бойни", 15, "~й~и";
        nominative: "~й~и";
        genitive: "~й~и", "~е~";
        dative: "~й~е", "~й~ям";
        accusative: referred;
        instrumental: "~й~ей|~й~ею", "~й~ями";
        prepositional: "~й~е", "~й~ях";
        noun_second_case: "~й~и", "~й~е";
    ),
    // Губоцветных: 15 words (вересковых, водяниковых, верескоцветных, буковых, букоцветных, мальпигиевых).
    declension!("губоцветных", 15, "~х";
        nominative: "~ы";
        genitive: "~х", "~ов";
        dative: "~м", "~ам";
        accusative: referred;
        instrumental: "~ми", "~ами";
        prepositional: "~х", "~ах";
        noun_second_case: "~х", "~х";
    ),
    // Племени: 14 words (имени, времени, знамени, бремени, вымени, пламени).
    declension!("племени", 14, "~е~и";
        nominative: "~е~а";
        genitive: "~е~и", "~ё~";
        dative: "~е~и", "~е~ам";
        accusative: referred;
        instrumental: "~е~ем", "~е~ами";
        prepositional: "~е~и", "~е~ах";
        noun_second_case: "~е~и", "~е~и";
    ),
    // Котёночка: 13 words (цыплёночка, котёночка, жеребёночка, козлёночка, утёночка, ягнёночка).
    declension!("котёночка", 13, "~ёноч~а";
        nominative: "~ят~и";
        genitive: "~ёноч~а", "~ято~";
        dative: "~ёноч~у", "~ят~ам";
        accusative: referred;
        instrumental: "~ёноч~ом", "~ят~ами";
        prepositional: "~ёноч~е", "~ят~ах";
        noun_second_case: "~ёноч~а", "~ёноч~е";
    ),
    // Мериноса: 13 words (квакера, виолончелиста, солитера, кастрата, бахмата, дидакта).
    declension!("мериноса", 13, "~а";
        nominative: "~ы";
        genitive: "~а", "~ов";
        dative: "~у", "~ам";
        accusative: "~ы";
        instrumental: "~ом", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~а", "~е";
    ),
    // Кокцинеллиды: 13 words (ламиины, иссиды, кокцинеллиды, алеохарины, циксииды, платигастриды).
    declension!("кокцинеллиды", 13, "~ы";
        nominative: "~ы";
        genitive: "~ы", "~";
        dative: "~е", "~ам";
        accusative: "~ы";
        instrumental: "~ой|~ою", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~ы", "~е";
    ),
    // Сердца: 12 words (сердца, полотенца, блюдца, оконца, волоконца, кружевца).
    declension!("сердца", 12, "~~а";
        nominative: "~~а";
        genitive: "~~а", "~е~";
        dative: "~~у", "~~ам";
        accusative: referred;
        instrumental: "~~ем", "~~ами";
        prepositional: "~~е", "~~ах";
        noun_second_case: "~~а", "~~е";
    ),
    // Гнезда: 12 words (озера, колеса, села, чела, решета, гнезда).
    declension!("гнезда", 12, "~е~а";
        nominative: "~ё~а";
        genitive: "~е~а", "~ё~";
        dative: "~е~у", "~ё~ам";
        accusative: referred;
        instrumental: "~е~ом", "~ё~ами";
        prepositional: "~е~е", "~ё~ах";
        noun_second_case: "~е~а", "~е~е";
    ),
    // Счёта: 11 words (мёда, счёта, шёлка, жёрнова, жёлоба, аэрожёлоба).
    declension!("счёта", 11, "~ё~а";
        nominative: "~е~а";
        genitive: "~ё~а", "~е~ов";
        dative: "~ё~у", "~е~ам";
        accusative: referred;
        instrumental: "~ё~ом", "~е~ами";
        prepositional: "~ё~е", "~е~ах";
        noun_second_case: "~ё~а", "~ё~е";
    ),
    // Усадьбы: 10 words (тьмы, судьбы, свадьбы, тюрьмы, усадьбы, пыльцы).
    declension!("усадьбы", 10, "~ь~ы";
        nominative: "~ь~ы";
        genitive: "~ь~ы", "~е~";
        dative: "~ь~е", "~ь~ам";
        accusative: referred;
        instrumental: "~ь~ой|~ь~ою", "~ь~ами";
        prepositional: "~ь~е", "~ь~ах";
        noun_second_case: "~ь~ы", "~ь~е";
    ),
    // Кольца: 10 words (письма, кольца, крыльца, сельца, автокольца, пихьма).
    declension!("кольца", 10, "~ь~а";
        nominative: "~ь~а";
        genitive: "~ь~а", "~е~";
        dative: "~ь~у", "~ь~ам";
        accusative: referred;
        instrumental: "~ь~ом", "~ь~ами";
        prepositional: "~ь~е", "~ь~ах";
        noun_second_case: "~ь~а", "~ь~е";
    ),
    // Рыльца: 10 words (зеркальца, щупальца, рыльца, сальца, мыльца, ебальца).
    declension!("рыльца", 10, "~ь~а";
        nominative: "~ь~а";
        genitive: "~ь~а", "~е~";
        dative: "~ь~у", "~ь~ам";
        accusative: referred;
        instrumental: "~ь~ем", "~ь~ами";
        prepositional: "~ь~е", "~ь~ах";
        noun_second_case: "~ь~а", "~ь~е";
    ),
    // Дульца: 10 words (весельца, дульца, жальца, зальца, одеяльца, седельца).
    declension!("дульца", 10, "~ь~а";
        nominative: "~ь~а";
        genitive: "~ь~а", "~ь~ев|~е~";
        dative: "~ь~у", "~ь~ам";
        accusative: referred;
        instrumental: "~ь~ем", "~ь~ами";
        prepositional: "~ь~е", "~ь~ах";
        noun_second_case: "~ь~а", "~ь~е";
    ),
    // Дома: 9 words (дома, леса, века, снега, меха, стога).
    declension!("дома", 9, "~а";
        nominative: "~а";
        genitive: "~а", "~ов";
        dative: "~у", "~ам";
        accusative: referred;
        instrumental: "~ом", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~у", "~у";
    ),
    // Духа: 9 words (воздуха, сока, духа, шага, мха, пуха).
    declension!("духа", 9, "~а";
        nominative: "~и";
        genitive: "~а", "~ов";
        dative: "~у", "~ам";
        accusative: referred;
        instrumental: "~ом", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~у", "~у";
    ),
    // Роста: 9 words (цвета, голоса, окорока, роста, сахара, веса).
    declension!("роста", 9, "~а";
        nominative: "~а";
        genitive: "~а", "~ов";
        dative: "~у", "~ам";
        accusative: referred;
        instrumental: "~ом", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~у", "~е";
    ),
    // Калашникова: 9 words (калашникова, баскова, топтыгина, шарыпова, обещалкина, сердечкина).
    declension!("калашникова", 9, "~а";
        nominative: "~ы";
        genitive: "~а", "~ых";
        dative: "~у", "~ым";
        accusative: referred;
        instrumental: "~ым", "~ыми";
        prepositional: "~е", "~ых";
        noun_second_case: "~а", "~е";
    ),
    // Куклы: 8 words (мглы, куклы, мошны, княжны, свёклы, чухны).
    declension!("куклы", 8, "~~ы";
        nominative: "~~ы";
        genitive: "~~ы", "~о~";
        dative: "~~е", "~~ам";
        accusative: referred;
        instrumental: "~~ой|~~ою", "~~ами";
        prepositional: "~~е", "~~ах";
        noun_second_case: "~~ы", "~~е";
    ),
    // Покоя: 8 words (гноя, покоя, инея, сельдерея, шалфея, перегноя).
    declension!("покоя", 8, "~я";
        nominative: "~и";
        genitive: "~я", "~ев";
        dative: "~ю", "~ям";
        accusative: referred;
        instrumental: "~ем", "~ями";
        prepositional: "~е", "~ях";
        noun_second_case: "~ю", "~е";
    ),
    // Пресмыкающегося: 8 words (учащегося, трудящегося, пресмыкающегося, случившегося, вольноопределяющегося, праздношатающегося).
    declension!("пресмыкающегося", 8, "~его~";
        nominative: "~ие~";
        genitive: "~его~", "~их~";
        dative: "~ему~", "~им~";
        accusative: referred;
        instrumental: "~им~", "~ими~";
        prepositional: "~ем~", "~их~";
        noun_second_case: "~его~", "~ем~";
    ),
    // Герца: 8 words (герца, гигагерца, килогерца, мегагерца, миллигерца, кольбаца).
    declension!("герца", 8, "~а";
        nominative: "~ы";
        genitive: "~а", "~";
        dative: "~у", "~ам";
        accusative: referred;
        instrumental: "~ем", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~а", "~е";
    ),
    // Плюхина: 8 words (мотовства, плюхина, плюхнова, бородина, дивеева, кабанова).
    declension!("плюхина", 8, "~а";
        nominative: "~а|~о";
        genitive: "~а|~о", "~о";
        dative: "~у|~о", "~ам|~о";
        accusative: referred;
        instrumental: "~ом|~о", "~ами|~о";
        prepositional: "~е|~о", "~ах|~о";
        noun_second_case: "~а|~о", "~е|~о";
    ),
    // Плеча: 7 words (яблока, плеча, сапога, брюха, лыка, амбаришка).
    declension!("плеча", 7, "~а";
        nominative: "~и";
        genitive: "~а", "~";
        dative: "~у", "~ам";
        accusative: referred;
        instrumental: "~ом", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~а", "~е";
    ),
    // Камня: 7 words (камня, корня, сечня, цветня, шашня, лютня).
    declension!("камня", 7, "~~я";
        nominative: "~~и|~е~и";
        genitive: "~~я|~е~я", "~~ей|~е~ей";
        dative: "~~ю|~е~ю", "~~ям|~е~ям";
        accusative: referred;
        instrumental: "~~ем|~е~ем", "~~ями|~е~ями";
        prepositional: "~~е|~е~е", "~~ях|~е~ях";
        noun_second_case: "~~я|~е~я", "~~е|~е~е";
    ),
    // Череды: 7 words (середы, железы, слезы, пелены, череды, желны).
    declension!("череды", 7, "~е~ы";
        nominative: "~е~ы";
        genitive: "~е~ы", "~ё~";
        dative: "~е~е", "~е~ам";
        accusative: referred;
        instrumental: "~е~ой|~е~ою", "~е~ами";
        prepositional: "~е~е", "~е~ах";
        noun_second_case: "~е~ы", "~е~е";
    ),
    // Весны: 7 words (десны, весны, метлы, ветлы, гидрометлы, блесны).
    declension!("весны", 7, "~е~~ы";
        nominative: "~ё~~ы";
        genitive: "~е~~ы", "~ё~е~";
        dative: "~е~~е", "~ё~~ам";
        accusative: referred;
        instrumental: "~е~~ой|~е~~ою", "~ё~~ами";
        prepositional: "~е~~е", "~ё~~ах";
        noun_second_case: "~е~~ы", "~е~~е";
    ),
    // Королька: 7 words (королька, франка, счётчика, макаронника, кержака, чирика).
    declension!("королька", 7, "~а";
        nominative: "~и";
        genitive: "~а", "~ов";
        dative: "~у", "~ам";
        accusative: "~ов|~и";
        instrumental: "~ом", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~а", "~е";
    ),
    // Боя: 6 words (чая, боя, края, клея, иван-чая, фиточая).
    declension!("боя", 6, "~я";
        nominative: "~и";
        genitive: "~я", "~ёв";
        dative: "~ю", "~ям";
        accusative: referred;
        instrumental: "~ем", "~ями";
        prepositional: "~е", "~ях";
        noun_second_case: "~ю", "~е";
    ),
    // Стекла: 6 words (стекла, бронестекла, стегна, наностекла, оргстекла, автостекла).
    declension!("стекла", 6, "~е~~а";
        nominative: "~ё~~а";
        genitive: "~е~~а", "~ё~о~";
        dative: "~е~~у", "~ё~~ам";
        accusative: referred;
        instrumental: "~е~~ом", "~ё~~ами";
        prepositional: "~е~~е", "~ё~~ах";
        noun_second_case: "~е~~а", "~е~~е";
    ),
    // Татарина: 6 words (болгарина, татарина, хазарина, угрина, долганина, хариджанина).
    declension!("татарина", 6, "~ина";
        nominative: "~ы";
        genitive: "~ина", "~";
        dative: "~ину", "~ам";
        accusative: referred;
        instrumental: "~ином", "~ами";
        prepositional: "~ине", "~ах";
        noun_second_case: "~ина", "~ине";
    ),
    // Чукчи: 6 words (юноши, чукчи, раджи, обжи, корчи, магараджи).
    declension!("чукчи", 6, "~и";
        nominative: "~и";
        genitive: "~и", "~ей";
        dative: "~е", "~ам";
        accusative: referred;
        instrumental: "~ей|~ею", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~и", "~е";
    ),
    // Метёлки: 6 words (чечётки, метёлки, кержанки, разведёнки, скалолазки, удмуртки).
    declension!("метёлки", 6, "~~и";
        nominative: "~~и";
        genitive: "~~и", "~о~";
        dative: "~~е", "~~ам";
        accusative: "~о~";
        instrumental: "~~ой|~~ою", "~~ами";
        prepositional: "~~е", "~~ах";
        noun_second_case: "~~и", "~~е";
    ),
    // Коленца: 6 words (болотца, коленца, ведёрца, корытца, гуменца, одонца).
    declension!("коленца", 6, "~~а";
        nominative: "~~а";
        genitive: "~~а", "~~ев|~е~";
        dative: "~~у", "~~ам";
        accusative: referred;
        instrumental: "~~ем", "~~ами";
        prepositional: "~~е", "~~ах";
        noun_second_case: "~~а", "~~е";
    ),
    // Исайи: 6 words (исайи, хориоидеи, зауреи, стеатореи, ципреи, персеи).
    declension!("исайи", 6, "~и";
        nominative: "~и";
        genitive: "~и", "~й";
        dative: "~е", "~ям";
        accusative: "~й";
        instrumental: "~ей|~ею", "~ями";
        prepositional: "~е", "~ях";
        noun_second_case: "~и", "~е";
    ),
    // Круга: 5 words (мозга, круга, верха, слуха, торга).
    declension!("круга", 5, "~а";
        nominative: "~и";
        genitive: "~а", "~ов";
        dative: "~у", "~ам";
        accusative: referred;
        instrumental: "~ом", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~а", "~у";
    ),
    // Кухни: 5 words (кухни, куфни, кормокухни, авиакухни, автокухни).
    declension!("кухни", 5, "~~и";
        nominative: "~~и";
        genitive: "~~и", "~о~ь";
        dative: "~~е", "~~ям";
        accusative: referred;
        instrumental: "~~ей|~~ею", "~~ями";
        prepositional: "~~е", "~~ях";
        noun_second_case: "~~и", "~~е";
    ),
    // Хозяина: 5 words (хозяина, домохозяина, квартирохозяина, сохозяина, судохозяина).
    declension!("хозяина", 5, "~ина";
        nominative: "~ева";
        genitive: "~ина", "~ев";
        dative: "~ину", "~евам";
        accusative: referred;
        instrumental: "~ином", "~евами";
        prepositional: "~ине", "~евах";
        noun_second_case: "~ина", "~ине";
    ),
    // Чернорубашечника: 5 words (чернорубашечника, австралопитека, питека, укропитека, козлятника).
    declension!("чернорубашечника", 5, "~а";
        nominative: "~и";
        genitive: "~а", "~ов";
        dative: "~у", "~ам";
        accusative: "~и";
        instrumental: "~ом", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~а", "~е";
    ),
    // Любви: 4 words (церкви, любви, нелюбви, лжецеркви).
    declension!("любви", 4, "~~и";
        nominative: "~~и";
        genitive: "~~и", "~~ей";
        dative: "~~и", "~~ям";
        accusative: referred;
        instrumental: "~о~ью", "~~ями";
        prepositional: "~~и", "~~ях";
        noun_second_case: "~~и", "~~и";
    ),
    // Опия: 4 words (гравия, опия, морфия, цикория).
    declension!("опия", 4, "~я";
        nominative: "~и";
        genitive: "~я", "~ев";
        dative: "~ю", "~ям";
        accusative: referred;
        instrumental: "~ем", "~ями";
        prepositional: "~и", "~ях";
        noun_second_case: "~ю", "~и";
    ),
    // Того: 4 words (кого, того, кое-кого, никого).
    declension!("того", 4, "~ого";
        nominative: "~то";
        genitive: "~ого", "~ого";
        dative: "~ому", "~ому";
        accusative: referred;
        instrumental: "~ем", "~ем";
        prepositional: "~ом", "~ом";
        noun_second_case: "~ого", "~ом";
    ),
    // Судна: 4 words (судна, агитсудна, авиасудна, фальшдна).
    declension!("судна", 4, "~на";
        nominative: "~а";
        genitive: "~на", "~ов";
        dative: "~ну", "~ам";
        accusative: referred;
        instrumental: "~ном", "~ами";
        prepositional: "~не", "~ах";
        noun_second_case: "~на", "~не";
    ),
    // Ничьей: 4 words (ничьей, третьей, девичьей, моей).
    declension!("ничьей", 4, "~ей";
        nominative: "~и";
        genitive: "~ей", "~их";
        dative: "~ей", "~им";
        accusative: referred;
        instrumental: "~ей|~ею", "~ими";
        prepositional: "~ей", "~их";
        noun_second_case: "~ей", "~ей";
    ),
    // Щенка: 3 words (щенка, внука, правнука).
    declension!("щенка", 3, "~ка";
        nominative: "~ки|~чата";
        genitive: "~ка", "~ков|~чат";
        dative: "~ку", "~кам|~чатам";
        accusative: referred;
        instrumental: "~ком", "~ками|~чатами";
        prepositional: "~ке", "~ках|~чатах";
        noun_second_case: "~ка", "~ке";
    ),
    // Носка: 3 words (носка, сяжка, черевичка).
    declension!("носка", 3, "~~а";
        nominative: "~~и";
        genitive: "~~а", "~~ов|~е~";
        dative: "~~у", "~~ам";
        accusative: referred;
        instrumental: "~~ом", "~~ами";
        prepositional: "~~е", "~~ах";
        noun_second_case: "~~а", "~~е";
    ),
    // Земли: 3 words (земли, петли, полупетли).
    declension!("земли", 3, "~~и";
        nominative: "~~и";
        genitive: "~~и", "~е~ь";
        dative: "~~е", "~~ям";
        accusative: referred;
        instrumental: "~~ёй|~~ёю", "~~ями";
        prepositional: "~~е", "~~ях";
        noun_second_case: "~~и", "~~е";
    ),
    // Пути: 3 words (пути, автопути, севморпути).
    declension!("пути", 3, "~и";
        nominative: "~и";
        genitive: "~и", "~ей";
        dative: "~и", "~ям";
        accusative: referred;
        instrumental: "~ём", "~ями";
        prepositional: "~и", "~ях";
        noun_second_case: "~и", "~и";
    ),
    // Семени: 3 words (семени, стремени, льносемени).
    declension!("семени", 3, "~е~и";
        nominative: "~е~а";
        genitive: "~е~и", "~я~";
        dative: "~е~и", "~е~ам";
        accusative: referred;
        instrumental: "~е~ем", "~е~ами";
        prepositional: "~е~и", "~е~ах";
        noun_second_case: "~е~и", "~е~и";
    ),
    // Лжи: 3 words (ржи, вши, лжи).
    declension!("лжи", 3, "~~и";
        nominative: "~~и";
        genitive: "~~и", "~~ей";
        dative: "~~и", "~~ам";
        accusative: referred;
        instrumental: "~о~ью", "~~ами";
        prepositional: "~~и", "~~ах";
        noun_second_case: "~~и", "~~и";
    ),
    // Смерти: 3 words (смерти, дряни, мрази).
    declension!("смерти", 3, "~и";
        nominative: "~и";
        genitive: "~и", "~ей";
        dative: "~и", "~ям";
        accusative: "~ей|~и";
        instrumental: "~ью", "~ями";
        prepositional: "~и", "~ях";
        noun_second_case: "~и", "~и";
    ),
    // Киселя: 3 words (киселя, щавеля, ревеня).
    declension!("киселя", 3, "~я";
        nominative: "~и";
        genitive: "~я", "~ей";
        dative: "~ю", "~ям";
        accusative: referred;
        instrumental: "~ём", "~ями";
        prepositional: "~е", "~ях";
        noun_second_case: "~ю", "~е";
    ),
    // Епитимьи: 3 words (ектеньи, паремьи, епитимьи).
    declension!("епитимьи", 3, "~ьи";
        nominative: "~ьи";
        genitive: "~ьи", "~ий";
        dative: "~ье", "~ьям";
        accusative: referred;
        instrumental: "~ьёй|~ьёю", "~ьями";
        prepositional: "~ье", "~ьях";
        noun_second_case: "~ьи", "~ье";
    ),
    // Дверцы: 3 words (дверцы, шпорцы, мадерцы).
    declension!("дверцы", 3, "~~ы";
        nominative: "~~ы";
        genitive: "~~ы", "~е~";
        dative: "~~е", "~~ам";
        accusative: referred;
        instrumental: "~~ей|~~ею", "~~ами";
        prepositional: "~~е", "~~ах";
        noun_second_case: "~~ы", "~~е";
    ),
    // Получеловека: 3 words (получеловека, обезьяночеловека, первочеловека).
    declension!("получеловека", 3, "~че~овека";
        nominative: "~~юди";
        genitive: "~че~овека", "~~юдей";
        dative: "~че~овеку", "~~юдям";
        accusative: referred;
        instrumental: "~че~овеком", "~~юдьми";
        prepositional: "~че~овеке", "~~юдях";
        noun_second_case: "~че~овека", "~че~овеке";
    ),
    // Морячки: 3 words (морячки, рыбачки, тестюшки).
    declension!("морячки", 3, "~~и";
        nominative: "~~и";
        genitive: "~~и", "~е~";
        dative: "~~е", "~~ам";
        accusative: "~~и";
        instrumental: "~~ой|~~ою", "~~ами";
        prepositional: "~~е", "~~ах";
        noun_second_case: "~~и", "~~е";
    ),
    // Сынули: 3 words (сынули, сани, игрули).
    declension!("сынули", 3, "~и";
        nominative: "~и";
        genitive: "~и", "~ь";
        dative: "~е", "~ям";
        accusative: "~и";
        instrumental: "~ей|~ею", "~ями";
        prepositional: "~е", "~ях";
        noun_second_case: "~и", "~е";
    ),
];
