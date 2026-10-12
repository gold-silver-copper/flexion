//! The conjugations, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one conjugation: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's feature says):
/// `non_past_first` (sg)
/// `non_past_third` (pl)
/// `participle` (pres_act)
/// `gerund` (pres)
macro_rules! conjugation {
    ($name:literal, $words:literal, $lemma:literal;
     non_past_first: $c0:tt;
     non_past_third: $c1:tt;
     participle: $c2:tt;
     gerund: $c3:tt;
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
            ]),
        }
    };
}

/// Every conjugation learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Conjugation {
    /// Like `забью`: 16,700 words.
    Забью,
    /// Like `вьюсь`: 10,802 words.
    Вьюсь,
    /// Like `стою`: 2,778 words.
    Стою,
    /// Like `лезу`: 2,766 words.
    Лезу,
    /// Like `числюсь`: 1,870 words.
    Числюсь,
    /// Like `разойдусь`: 1,668 words.
    Разойдусь,
    /// Like `проскочу`: 1,453 words.
    Проскочу,
    /// Like `долблю`: 1,155 words.
    Долблю,
    /// Like `держусь`: 958 words.
    Держусь,
    /// Like `расправлюсь`: 790 words.
    Расправлюсь,
    /// Like `развожу`: 621 words.
    Развожу,
    /// Like `режу`: 575 words.
    Режу,
    /// Like `хвачу`: 564 words.
    Хвачу,
    /// Like `расхожусь`: 423 words.
    Расхожусь,
    /// Like `закачусь`: 357 words.
    Закачусь,
    /// Like `сношу`: 353 words.
    Сношу,
    /// Like `отпущу`: 255 words.
    Отпущу,
    /// Like `переброшусь`: 232 words.
    Переброшусь,
    /// Like `загружу`: 185 words.
    Загружу,
    /// Like `распущусь`: 181 words.
    Распущусь,
    /// Like `сближусь`: 122 words.
    Сближусь,
    /// Like `защищу`: 53 words.
    Защищу,
    /// Like `отдаю`: 52 words.
    Отдаю,
    /// Like `отдаюсь`: 45 words.
    Отдаюсь,
    /// Like `обращусь`: 43 words.
    Обращусь,
    /// Like `сдам`: 41 words.
    Сдам,
    /// Like `засыплю`: 41 words.
    Засыплю,
    /// Like `режусь`: 27 words.
    Режусь,
    /// Like `заем`: 23 words.
    Заем,
    /// Like `отдамся`: 20 words.
    Отдамся,
    /// Like `жду`: 20 words.
    Жду,
    /// Like `въемся`: 18 words.
    Въемся,
    /// Like `высру`: 14 words.
    Высру,
    /// Like `обыщу`: 11 words.
    Обыщу,
    /// Like `приемлю`: 10 words.
    Приемлю,
    /// Like `хнычу`: 6 words.
    Хнычу,
    /// Like `наморщусь`: 5 words.
    Наморщусь,
    /// Like `капаю`: 4 words.
    Капаю,
    /// Like `почту`: 4 words.
    Почту,
    /// Like `обгложу`: 3 words.
    Обгложу,
    /// Like `задгружусь`: 3 words.
    Задгружусь,
    /// Like `харашу`: 3 words.
    Харашу,
    /// Like `есть`: 1 words.
    Есть,
}

impl Conjugation {
    /// Every conjugation, in order.
    pub const ALL: [Conjugation; 43] = [
        Conjugation::Забью,
        Conjugation::Вьюсь,
        Conjugation::Стою,
        Conjugation::Лезу,
        Conjugation::Числюсь,
        Conjugation::Разойдусь,
        Conjugation::Проскочу,
        Conjugation::Долблю,
        Conjugation::Держусь,
        Conjugation::Расправлюсь,
        Conjugation::Развожу,
        Conjugation::Режу,
        Conjugation::Хвачу,
        Conjugation::Расхожусь,
        Conjugation::Закачусь,
        Conjugation::Сношу,
        Conjugation::Отпущу,
        Conjugation::Переброшусь,
        Conjugation::Загружу,
        Conjugation::Распущусь,
        Conjugation::Сближусь,
        Conjugation::Защищу,
        Conjugation::Отдаю,
        Conjugation::Отдаюсь,
        Conjugation::Обращусь,
        Conjugation::Сдам,
        Conjugation::Засыплю,
        Conjugation::Режусь,
        Conjugation::Заем,
        Conjugation::Отдамся,
        Conjugation::Жду,
        Conjugation::Въемся,
        Conjugation::Высру,
        Conjugation::Обыщу,
        Conjugation::Приемлю,
        Conjugation::Хнычу,
        Conjugation::Наморщусь,
        Conjugation::Капаю,
        Conjugation::Почту,
        Conjugation::Обгложу,
        Conjugation::Задгружусь,
        Conjugation::Харашу,
        Conjugation::Есть,
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

pub(super) static CONJUGATIONS: [Paradigm<'static>; 43] = [
    // Забью: 16,700 words (баю, знаю, играю, презираю, делаю, уступаю).
    conjugation!("забью", 16700, "~ю";
        non_past_first: "~ю";
        non_past_third: "~ют";
        participle: "~ющий";
        gerund: "~я";
    ),
    // Вьюсь: 10,802 words (отпочковываюсь, изъясняюсь, облокачиваюсь, тусуюсь, используюсь, моюсь).
    conjugation!("вьюсь", 10802, "~ю~ь";
        non_past_first: "~ю~ь";
        non_past_third: "~ют~я";
        participle: "~ющий~я";
        gerund: "~я~ь";
    ),
    // Стою: 2,778 words (жалю, ужалю, презрю, говорю, сгорю, курю).
    conjugation!("стою", 2778, "~ю";
        non_past_first: "~ю";
        non_past_third: "~ят";
        participle: "~ящий";
        gerund: "~я";
    ),
    // Лезу: 2,766 words (беру, возьму, достигну, иду, стану, выковырну).
    conjugation!("лезу", 2766, "~у";
        non_past_first: "~у";
        non_past_third: "~ут";
        participle: "~ущий";
        gerund: "~я";
    ),
    // Числюсь: 1,870 words (изъяснюсь, беспокоюсь, боюсь, ссорюсь, соединюсь, присоединюсь).
    conjugation!("числюсь", 1870, "~ю~ь";
        non_past_first: "~ю~ь";
        non_past_third: "~ят~я";
        participle: "~ящий~я";
        gerund: "~я~ь";
    ),
    // Разойдусь: 1,668 words (ебусь, рехнусь, стебусь, берегусь, поднимусь, крадусь).
    conjugation!("разойдусь", 1668, "~у~ь";
        non_past_first: "~у~ь";
        non_past_third: "~ут~я";
        participle: "~ущий~я";
        gerund: "~я~ь";
    ),
    // Проскочу: 1,453 words (держу, учу, кончу, увеличу, закончу, значу).
    conjugation!("проскочу", 1453, "~у";
        non_past_first: "~у";
        non_past_third: "~ат";
        participle: "~ащий";
        gerund: "~а";
    ),
    // Долблю: 1,155 words (сплю, уступлю, забавлю, люблю, полюблю, возлюблю).
    conjugation!("долблю", 1155, "~лю";
        non_past_first: "~лю";
        non_past_third: "~ят";
        participle: "~ящий";
        gerund: "~я";
    ),
    // Держусь: 958 words (учусь, кончусь, увеличусь, страшусь, случусь, научусь).
    conjugation!("держусь", 958, "~у~ь";
        non_past_first: "~у~ь";
        non_past_third: "~ат~я";
        participle: "~ащий~я";
        gerund: "~а~ь";
    ),
    // Расправлюсь: 790 words (нравлюсь, понравлюсь, готовлюсь, появлюсь, тороплюсь, направлюсь).
    conjugation!("расправлюсь", 790, "~лю~ь";
        non_past_first: "~лю~ь";
        non_past_third: "~ят~я";
        participle: "~ящий~я";
        gerund: "~я~ь";
    ),
    // Развожу: 621 words (щажу, бзжу, набзжу, вижу, сижу, происхожу).
    conjugation!("развожу", 621, "~жу";
        non_past_first: "~жу";
        non_past_third: "~дят";
        participle: "~дящий";
        gerund: "~дя";
    ),
    // Режу: 575 words (хохочу, скажу, могу, пишу, смогу, помогу).
    conjugation!("режу", 575, "~у";
        non_past_first: "~у";
        non_past_third: "~ут";
        participle: "~ущий";
        gerund: "~а";
    ),
    // Хвачу: 564 words (лечу, хочу, полечу, захочу, отвечу, встречу).
    conjugation!("хвачу", 564, "~чу";
        non_past_first: "~чу";
        non_past_third: "~тят";
        participle: "~тящий";
        gerund: "~тя";
    ),
    // Расхожусь: 423 words (нахожусь, тружусь, сажусь, наслажусь, развожусь, рожусь).
    conjugation!("расхожусь", 423, "~жу~ь";
        non_past_first: "~жу~ь";
        non_past_third: "~дят~я";
        participle: "~дящий~я";
        gerund: "~дя~ь";
    ),
    // Закачусь: 357 words (охочусь, захочусь, облокочусь, забочусь, встречусь, расплачусь).
    conjugation!("закачусь", 357, "~чу~ь";
        non_past_first: "~чу~ь";
        non_past_third: "~тят~я";
        participle: "~тящий~я";
        gerund: "~тя~ь";
    ),
    // Сношу: 353 words (ношу, спрошу, крашу, приношу, приглашу, брошу).
    conjugation!("сношу", 353, "~шу";
        non_past_first: "~шу";
        non_past_third: "~сят";
        participle: "~сящий";
        gerund: "~ся";
    ),
    // Отпущу: 255 words (блещу, допущу, пропущу, крещу, свищу, чищу).
    conjugation!("отпущу", 255, "~щу";
        non_past_first: "~щу";
        non_past_third: "~стят";
        participle: "~стящий";
        gerund: "~стя";
    ),
    // Переброшусь: 232 words (вышусь, соотношусь, соглашусь, брошусь, возвышусь, отношусь).
    conjugation!("переброшусь", 232, "~шу~ь";
        non_past_first: "~шу~ь";
        non_past_third: "~~ятся";
        participle: "~~ящийся";
        gerund: "~~ясь";
    ),
    // Загружу: 185 words (воображу, лажу, заморожу, привожу, морожу, скольжу).
    conjugation!("загружу", 185, "~жу";
        non_past_first: "~жу";
        non_past_third: "~зят";
        participle: "~зящий";
        gerund: "~зя";
    ),
    // Распущусь: 181 words (крещусь, спущусь, сгущусь, опущусь, помещусь, прощусь).
    conjugation!("распущусь", 181, "~щу~ь";
        non_past_first: "~щу~ь";
        non_past_third: "~~тятся";
        participle: "~~тящийся";
        gerund: "~~тясь";
    ),
    // Сближусь: 122 words (сражусь, приближусь, сконфужусь, вожусь, выражусь, ближусь).
    conjugation!("сближусь", 122, "~жу~ь";
        non_past_first: "~жу~ь";
        non_past_third: "~зят~я";
        participle: "~зящий~я";
        gerund: "~зя~ь";
    ),
    // Защищу: 53 words (запрещу, прекращу, посещу, защищу, похищу, сокращу).
    conjugation!("защищу", 53, "~щу";
        non_past_first: "~щу";
        non_past_third: "~тят";
        participle: "~тящий";
        gerund: "~тя";
    ),
    // Отдаю: 52 words (даю, воссоздаю, создаю, издаю, устаю, предаю).
    conjugation!("отдаю", 52, "~ю";
        non_past_first: "~ю";
        non_past_third: "~ют";
        participle: "~ющий";
        gerund: "~вая";
    ),
    // Отдаюсь: 45 words (издаюсь, остаюсь, сдаюсь, расстаюсь, удаюсь, раздаюсь).
    conjugation!("отдаюсь", 45, "~ю~ь";
        non_past_first: "~ю~ь";
        non_past_third: "~ют~я";
        participle: "~ющий~я";
        gerund: "~вая~ь";
    ),
    // Обращусь: 43 words (превращусь, обращусь, прекращусь, возвращусь, смущусь, возмущусь).
    conjugation!("обращусь", 43, "~щу~ь";
        non_past_first: "~щу~ь";
        non_past_third: "~тят~я";
        participle: "~тящий~я";
        gerund: "~тя~ь";
    ),
    // Сдам: 41 words (дам, воссоздам, создам, издам, предам, выдам).
    conjugation!("сдам", 41, "~м";
        non_past_first: "~м";
        non_past_third: "~дут";
        participle: "~дящий";
        gerund: "~дя";
    ),
    // Засыплю: 41 words (засыплю, сыплю, насыплю, высыплю, посыплю, щиплю).
    conjugation!("засыплю", 41, "~лю";
        non_past_first: "~лю";
        non_past_third: "~лют";
        participle: "~лющий";
        gerund: "~ля";
    ),
    // Режусь: 27 words (кажусь, прячусь, мечусь, пишусь, топчусь, плещусь).
    conjugation!("режусь", 27, "~у~ь";
        non_past_first: "~у~ь";
        non_past_third: "~ут~я";
        participle: "~ущий~я";
        gerund: "~а~ь";
    ),
    // Заем: 23 words (ем, съем, поем, надоем, выем, разъем).
    conjugation!("заем", 23, "~м";
        non_past_first: "~м";
        non_past_third: "~дят";
        participle: "~дящий";
        gerund: "~дя";
    ),
    // Отдамся: 20 words (издамся, удамся, сдамся, раздамся, выдамся, дамся).
    conjugation!("отдамся", 20, "~м~";
        non_past_first: "~м~";
        non_past_third: "~дут~";
        participle: "~дящий~";
        gerund: "~я~ь";
    ),
    // Жду: 20 words (жду, подожду, жажду, выжду, пережду, обожду).
    conjugation!("жду", 20, "~у";
        non_past_first: "~у";
        non_past_third: "~ут";
        participle: "~ущий";
        gerund: "~ая";
    ),
    // Въемся: 18 words (наемся, въемся, объемся, приемся, взъемся, заемся).
    conjugation!("въемся", 18, "~м~";
        non_past_first: "~м~";
        non_past_third: "~дят~";
        participle: "~дящий~";
        gerund: "~я~ь";
    ),
    // Высру: 14 words (сру, обосру, насру, посру, высру, засру).
    conjugation!("высру", 14, "~~у";
        non_past_first: "~~у";
        non_past_third: "~~ут";
        participle: "~~ущий";
        gerund: "~~я";
    ),
    // Обыщу: 11 words (отыщу, разыщу, обыщу, взыщу, подыщу, сыщу).
    conjugation!("обыщу", 11, "~щу";
        non_past_first: "~щу";
        non_past_third: "~щут";
        participle: "~щущий";
        gerund: "~ская";
    ),
    // Приемлю: 10 words (задремлю, внемлю, приемлю, восприемлю, предприемлю, передремлю).
    conjugation!("приемлю", 10, "~е~лю";
        non_past_first: "~е~лю";
        non_past_third: "~е~лют";
        participle: "~е~лющий";
        gerund: "~е~ля";
    ),
    // Хнычу: 6 words (тычу, мурлычу, хнычу, захнычу, нычу, утычу).
    conjugation!("хнычу", 6, "~чу";
        non_past_first: "~чу|~каю";
        non_past_third: "~чут|~кают";
        participle: "~чущий|~кающий";
        gerund: "~ча|~кая";
    ),
    // Наморщусь: 5 words (встопорщусь, чтусь, наморщусь, изморщусь, узрусь).
    conjugation!("наморщусь", 5, "~у~ь";
        non_past_first: "~у~ь";
        non_past_third: "~ят~я";
        participle: "~ящий~я";
        gerund: "~я~ь";
    ),
    // Капаю: 4 words (капаю, крапаю, стебаю, щепаю).
    conjugation!("капаю", 4, "~аю";
        non_past_first: "~аю|~лю";
        non_past_third: "~ают|~лют";
        participle: "~ающий|~лющий";
        gerund: "~ая|~ля";
    ),
    // Почту: 4 words (почту, лихачу, отчужду, тщу).
    conjugation!("почту", 4, "~у";
        non_past_first: "~у";
        non_past_third: "~ят";
        participle: "~ящий";
        gerund: "~я";
    ),
    // Обгложу: 3 words (гложу, обгложу, сгложу).
    conjugation!("обгложу", 3, "~жу";
        non_past_first: "~жу|~даю";
        non_past_third: "~жут|~дают";
        participle: "~жущий";
        gerund: "~жа";
    ),
    // Задгружусь: 3 words (задгружусь, перезадгружусь, самозадгружусь).
    conjugation!("задгружусь", 3, "~д~жу~ь";
        non_past_first: "~д~жу~ь";
        non_past_third: "~~зят~я";
        participle: "~д~жу~ь";
        gerund: "~д~жу~ь";
    ),
    // Харашу: 3 words (оглонашу, харашу, глонашу).
    conjugation!("харашу", 3, "~шу";
        non_past_first: "~шу";
        non_past_third: "~ссят";
        participle: "~ссящий";
        gerund: "~сся";
    ),
    // Есть: 1 words (есть).
    conjugation!("есть", 1, "есть";
        non_past_first: "есть|есмь|буду";
        non_past_third: "есть|суть|будут";
        participle: "сущий";
        gerund: "будучи";
    ),
];
