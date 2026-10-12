//! The conjugations, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one conjugation: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's feature says):
/// `non_past_first` (pl)
/// `non_past_second` (sg, pl)
/// `non_past_third` (sg)
/// `imperative` (sg, pl)
/// `participle` (pres_pass)
macro_rules! conjugation {
    ($name:literal, $words:literal, $lemma:literal;
     non_past_first: $c0:tt;
     non_past_second: $c1:tt, $c2:tt;
     non_past_third: $c3:tt;
     imperative: $c4:tt, $c5:tt;
     participle: $c6:tt;
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
            ]),
        }
    };
}

/// Every conjugation learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Conjugation {
    /// Like `играешь`: 16,365 words.
    Играешь,
    /// Like `набираешься`: 10,518 words.
    Набираешься,
    /// Like `хватишь`: 4,791 words.
    Хватишь,
    /// Like `расходишься`: 3,256 words.
    Расходишься,
    /// Like `представишь`: 2,603 words.
    Представишь,
    /// Like `расправишься`: 1,621 words.
    Расправишься,
    /// Like `разведёшь`: 1,497 words.
    Разведёшь,
    /// Like `тянешь`: 1,473 words.
    Тянешь,
    /// Like `разойдёшься`: 878 words.
    Разойдёшься,
    /// Like `тянешься`: 608 words.
    Тянешься,
    /// Like `лезешь`: 302 words.
    Лезешь,
    /// Like `режешься`: 155 words.
    Режешься,
    /// Like `перестроишься`: 122 words.
    Перестроишься,
    /// Like `обтечёшь`: 105 words.
    Обтечёшь,
    /// Like `скуёшь`: 103 words.
    Скуёшь,
    /// Like `стоишь`: 98 words.
    Стоишь,
    /// Like `влечёшься`: 81 words.
    Влечёшься,
    /// Like `прожжёшь`: 72 words.
    Прожжёшь,
    /// Like `отдаёшь`: 71 words.
    Отдаёшь,
    /// Like `суёшься`: 68 words.
    Суёшься,
    /// Like `забьёшь`: 66 words.
    Забьёшь,
    /// Like `зажжёшься`: 56 words.
    Зажжёшься,
    /// Like `вьёшься`: 55 words.
    Вьёшься,
    /// Like `отдаёшься`: 45 words.
    Отдаёшься,
    /// Like `разобьёшь`: 44 words.
    Разобьёшь,
    /// Like `сдашь`: 41 words.
    Сдашь,
    /// Like `собьёшься`: 39 words.
    Собьёшься,
    /// Like `выставишь`: 29 words.
    Выставишь,
    /// Like `очистишься`: 23 words.
    Очистишься,
    /// Like `заешь`: 22 words.
    Заешь,
    /// Like `отдашься`: 20 words.
    Отдашься,
    /// Like `набежишь`: 20 words.
    Набежишь,
    /// Like `ляжешь`: 20 words.
    Ляжешь,
    /// Like `въешься`: 17 words.
    Въешься,
    /// Like `поедешь`: 16 words.
    Поедешь,
    /// Like `робишь`: 16 words.
    Робишь,
    /// Like `поможешь`: 15 words.
    Поможешь,
    /// Like `засыплешь`: 15 words.
    Засыплешь,
    /// Like `морщишься`: 13 words.
    Морщишься,
    /// Like `дремлешь`: 13 words.
    Дремлешь,
    /// Like `вымажешь`: 9 words.
    Вымажешь,
    /// Like `выпьешь`: 8 words.
    Выпьешь,
    /// Like `сыплешься`: 7 words.
    Сыплешься,
    /// Like `выдвинешься`: 5 words.
    Выдвинешься,
    /// Like `пробежишься`: 5 words.
    Пробежишься,
    /// Like `выпряжешься`: 5 words.
    Выпряжешься,
    /// Like `выльешься`: 5 words.
    Выльешься,
    /// Like `высечешь`: 4 words.
    Высечешь,
    /// Like `уляжешься`: 4 words.
    Уляжешься,
    /// Like `изопьёшь`: 4 words.
    Изопьёшь,
    /// Like `хочешь`: 3 words.
    Хочешь,
    /// Like `разъедешься`: 3 words.
    Разъедешься,
    /// Like `чтёшься`: 3 words.
    Чтёшься,
    /// Like `выпечешься`: 3 words.
    Выпечешься,
    /// Like `изопьёшься`: 3 words.
    Изопьёшься,
    /// Like `есть`: 1 words.
    Есть,
}

impl Conjugation {
    /// Every conjugation, in order.
    pub const ALL: [Conjugation; 56] = [
        Conjugation::Играешь,
        Conjugation::Набираешься,
        Conjugation::Хватишь,
        Conjugation::Расходишься,
        Conjugation::Представишь,
        Conjugation::Расправишься,
        Conjugation::Разведёшь,
        Conjugation::Тянешь,
        Conjugation::Разойдёшься,
        Conjugation::Тянешься,
        Conjugation::Лезешь,
        Conjugation::Режешься,
        Conjugation::Перестроишься,
        Conjugation::Обтечёшь,
        Conjugation::Скуёшь,
        Conjugation::Стоишь,
        Conjugation::Влечёшься,
        Conjugation::Прожжёшь,
        Conjugation::Отдаёшь,
        Conjugation::Суёшься,
        Conjugation::Забьёшь,
        Conjugation::Зажжёшься,
        Conjugation::Вьёшься,
        Conjugation::Отдаёшься,
        Conjugation::Разобьёшь,
        Conjugation::Сдашь,
        Conjugation::Собьёшься,
        Conjugation::Выставишь,
        Conjugation::Очистишься,
        Conjugation::Заешь,
        Conjugation::Отдашься,
        Conjugation::Набежишь,
        Conjugation::Ляжешь,
        Conjugation::Въешься,
        Conjugation::Поедешь,
        Conjugation::Робишь,
        Conjugation::Поможешь,
        Conjugation::Засыплешь,
        Conjugation::Морщишься,
        Conjugation::Дремлешь,
        Conjugation::Вымажешь,
        Conjugation::Выпьешь,
        Conjugation::Сыплешься,
        Conjugation::Выдвинешься,
        Conjugation::Пробежишься,
        Conjugation::Выпряжешься,
        Conjugation::Выльешься,
        Conjugation::Высечешь,
        Conjugation::Уляжешься,
        Conjugation::Изопьёшь,
        Conjugation::Хочешь,
        Conjugation::Разъедешься,
        Conjugation::Чтёшься,
        Conjugation::Выпечешься,
        Conjugation::Изопьёшься,
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

pub(super) static CONJUGATIONS: [Paradigm<'static>; 56] = [
    // Играешь: 16,365 words (баешь, знаешь, играешь, презираешь, делаешь, уступаешь).
    conjugation!("играешь", 16365, "~ешь";
        non_past_first: "~ем";
        non_past_second: "~ешь", "~ете";
        non_past_third: "~ет";
        imperative: "~й", "~йте";
        participle: "~емый";
    ),
    // Набираешься: 10,518 words (отпочковываешься, изъясняешься, облокачиваешься, тусуешься, используешься, моешься).
    conjugation!("набираешься", 10518, "~ешь~я";
        non_past_first: "~ем~я";
        non_past_second: "~ешь~я", "~ете~ь";
        non_past_third: "~ет~я";
        imperative: "~й~я", "~йте~ь";
        participle: "~ешь~я";
    ),
    // Хватишь: 4,791 words (летишь, спишь, держишь, презришь, уступишь, щадишь).
    conjugation!("хватишь", 4791, "~шь";
        non_past_first: "~м";
        non_past_second: "~шь", "~те";
        non_past_third: "~т";
        imperative: "~", "~те";
        participle: "~мый";
    ),
    // Расходишься: 3,256 words (изъяснишься, учишься, кончишься, находишься, трудишься, садишься).
    conjugation!("расходишься", 3256, "~шь~я";
        non_past_first: "~м~я";
        non_past_second: "~шь~я", "~те~ь";
        non_past_third: "~т~я";
        imperative: "~~ь", "~те~ь";
        participle: "~шь~я";
    ),
    // Представишь: 2,603 words (жалишь, ужалишь, забавишь, восславишь, увидишь, лазишь).
    conjugation!("представишь", 2603, "~ишь";
        non_past_first: "~им";
        non_past_second: "~ишь", "~ите";
        non_past_third: "~ит";
        imperative: "~ь", "~ьте";
        participle: "~имый";
    ),
    // Расправишься: 1,621 words (нравишься, понравишься, увеличишься, ссоришься, готовишься, высишься).
    conjugation!("расправишься", 1621, "~ишь~я";
        non_past_first: "~им~я";
        non_past_second: "~ишь~я", "~ите~ь";
        non_past_third: "~ит~я";
        imperative: "~ь~я", "~ьте~ь";
        participle: "~ишь~я";
    ),
    // Разведёшь: 1,497 words (берёшь, возьмёшь, идёшь, взбзднёшь, произойдёшь, врёшь).
    conjugation!("разведёшь", 1497, "~ёшь";
        non_past_first: "~ём";
        non_past_second: "~ёшь", "~ёте";
        non_past_third: "~ёт";
        imperative: "~и", "~ите";
        participle: "~омый";
    ),
    // Тянешь: 1,473 words (достигнешь, хохочешь, выковырнешь, скажешь, пишешь, расскажешь).
    conjugation!("тянешь", 1473, "~ешь";
        non_past_first: "~ем";
        non_past_second: "~ешь", "~ете";
        non_past_third: "~ет";
        imperative: "~и", "~ите";
        participle: "~емый";
    ),
    // Разойдёшься: 878 words (ебёшься, рехнёшься, стебёшься, крадёшься, найдёшься, повернёшься).
    conjugation!("разойдёшься", 878, "~ёшь~я";
        non_past_first: "~ём~я";
        non_past_second: "~ёшь~я", "~ёте~ь";
        non_past_third: "~ёт~я";
        imperative: "~и~ь", "~ите~ь";
        participle: "~ёшь~я";
    ),
    // Тянешься: 608 words (борешься, поднимешься, кажешься, колеблешься, грохнешься, окажешься).
    conjugation!("тянешься", 608, "~ешь~я";
        non_past_first: "~ем~я";
        non_past_second: "~ешь~я", "~ете~ь";
        non_past_third: "~ет~я";
        imperative: "~и~ь", "~ите~ь";
        participle: "~ешь~я";
    ),
    // Лезешь: 302 words (станешь, сядешь, устанешь, застрянешь, прячешь, спрячешь).
    conjugation!("лезешь", 302, "~ешь";
        non_past_first: "~ем";
        non_past_second: "~ешь", "~ете";
        non_past_third: "~ет";
        imperative: "~ь", "~ьте";
        participle: "~емый";
    ),
    // Режешься: 155 words (останешься, расстанешься, оденешься, двинешься, забудешься, прячешься).
    conjugation!("режешься", 155, "~ешь~я";
        non_past_first: "~ем~я";
        non_past_second: "~ешь~я", "~ете~ь";
        non_past_third: "~ет~я";
        imperative: "~ь~я", "~ьте~ь";
        participle: "~ешь~я";
    ),
    // Перестроишься: 122 words (беспокоишься, боишься, успокоишься, приклеишься, строишься, забоишься).
    conjugation!("перестроишься", 122, "~ишь~я";
        non_past_first: "~им~я";
        non_past_second: "~ишь~я", "~ите~ь";
        non_past_third: "~ит~я";
        imperative: "~й~я", "~йте~ь";
        participle: "~ишь~я";
    ),
    // Обтечёшь: 105 words (печёшь, испечёшь, спечёшь, стечёшь, течёшь, сволочёшь).
    conjugation!("обтечёшь", 105, "~чёшь";
        non_past_first: "~чём";
        non_past_second: "~чёшь", "~чёте";
        non_past_third: "~чёт";
        imperative: "~ки", "~ките";
        participle: "~комый";
    ),
    // Скуёшь: 103 words (куёшь, поёшь, блюёшь, плюёшь, подкуёшь, споёшь).
    conjugation!("скуёшь", 103, "~ёшь";
        non_past_first: "~ём";
        non_past_second: "~ёшь", "~ёте";
        non_past_third: "~ёт";
        imperative: "~й", "~йте";
        participle: "~омый";
    ),
    // Стоишь: 98 words (стоишь, беспокоишь, строишь, клеишь, устроишь, построишь).
    conjugation!("стоишь", 98, "~ишь";
        non_past_first: "~им";
        non_past_second: "~ишь", "~ите";
        non_past_third: "~ит";
        imperative: "~й", "~йте";
        participle: "~имый";
    ),
    // Влечёшься: 81 words (отречёшься, увлечёшься, растечёшься, отвлечёшься, стечёшься, влечёшься).
    conjugation!("влечёшься", 81, "~чёшь~я";
        non_past_first: "~чём~я";
        non_past_second: "~чёшь~я", "~чёте~ь";
        non_past_third: "~чёт~я";
        imperative: "~ки~ь", "~ките~ь";
        participle: "~чёшь~я";
    ),
    // Прожжёшь: 72 words (жжёшь, сожжёшь, бережёшь, подожжёшь, лжёшь, пренебрежёшь).
    conjugation!("прожжёшь", 72, "~жёшь";
        non_past_first: "~жём";
        non_past_second: "~жёшь", "~жёте";
        non_past_third: "~жёт";
        imperative: "~ги", "~гите";
        participle: "~гомый";
    ),
    // Отдаёшь: 71 words (даёшь, воссоздаёшь, создаёшь, издаёшь, устаёшь, предаёшь).
    conjugation!("отдаёшь", 71, "~ёшь";
        non_past_first: "~ём";
        non_past_second: "~ёшь", "~ёте";
        non_past_third: "~ёт";
        imperative: "~вай", "~вайте";
        participle: "~ваемый";
    ),
    // Суёшься: 68 words (смеёшься, засмеёшься, куёшься, рассмеёшься, посмеёшься, оснуёшься).
    conjugation!("суёшься", 68, "~ёшь~я";
        non_past_first: "~ём~я";
        non_past_second: "~ёшь~я", "~ёте~ь";
        non_past_third: "~ёт~я";
        imperative: "~й~я", "~йте~ь";
        participle: "~ёшь~я";
    ),
    // Забьёшь: 66 words (пьёшь, бьёшь, льёшь, вьёшь, шьёшь, побьёшь).
    conjugation!("забьёшь", 66, "~ьёшь";
        non_past_first: "~ьём";
        non_past_second: "~ьёшь", "~ьёте";
        non_past_third: "~ьёт";
        imperative: "~ей", "~ейте";
        participle: "~емый";
    ),
    // Зажжёшься: 56 words (бережёшься, напряжёшься, зажжёшься, остережёшься, обожжёшься, стрижёшься).
    conjugation!("зажжёшься", 56, "~жёшь~я";
        non_past_first: "~жём~я";
        non_past_second: "~жёшь~я", "~жёте~ь";
        non_past_third: "~жёт~я";
        imperative: "~ги~ь", "~гите~ь";
        participle: "~жёшь~я";
    ),
    // Вьёшься: 55 words (льёшься, бьёшься, добьёшься, напьёшься, нальёшься, пробьёшься).
    conjugation!("вьёшься", 55, "~ьёшь~я";
        non_past_first: "~ьём~я";
        non_past_second: "~ьёшь~я", "~ьёте~ь";
        non_past_third: "~ьёт~я";
        imperative: "~ей~я", "~ейте~ь";
        participle: "~ьёшь~я";
    ),
    // Отдаёшься: 45 words (издаёшься, остаёшься, сдаёшься, расстаёшься, удаёшься, раздаёшься).
    conjugation!("отдаёшься", 45, "~ёшь~я";
        non_past_first: "~ём~я";
        non_past_second: "~ёшь~я", "~ёте~ь";
        non_past_third: "~ёт~я";
        imperative: "~вай~я", "~вайте~ь";
        participle: "~ёшь~я";
    ),
    // Разобьёшь: 44 words (сошьёшь, разобьёшь, собьёшь, совьёшь, разовьёшь, разольёшь).
    conjugation!("разобьёшь", 44, "~о~ьёшь";
        non_past_first: "~о~ьём";
        non_past_second: "~о~ьёшь", "~о~ьёте";
        non_past_third: "~о~ьёт";
        imperative: "~~ей", "~~ейте";
        participle: "~о~ьёшь";
    ),
    // Сдашь: 41 words (дашь, воссоздашь, создашь, издашь, предашь, выдашь).
    conjugation!("сдашь", 41, "~шь";
        non_past_first: "~дим";
        non_past_second: "~шь", "~дите";
        non_past_third: "~ст";
        imperative: "~й", "~йте";
        participle: "~домый";
    ),
    // Собьёшься: 39 words (вольёшься, собьёшься, разовьёшься, взовьёшься, разобьёшься, отобьёшься).
    conjugation!("собьёшься", 39, "~о~ьёшь~я";
        non_past_first: "~о~ьём~я";
        non_past_second: "~о~ьёшь~я", "~о~ьёте~ь";
        non_past_third: "~о~ьёт~я";
        imperative: "~~ей~я", "~~ейте~ь";
        participle: "~о~ьёшь~я";
    ),
    // Выставишь: 29 words (мусоришь, дрейфишь, упорядочишь, выбросишь, морщишь, выставишь).
    conjugation!("выставишь", 29, "~ишь";
        non_past_first: "~им";
        non_past_second: "~ишь", "~ите";
        non_past_third: "~ит";
        imperative: "~и|~ь", "~ьте";
        participle: "~имый";
    ),
    // Очистишься: 23 words (портишься, испортишься, корчишься, скорчишься, очистишься, горбишься).
    conjugation!("очистишься", 23, "~ишь~я";
        non_past_first: "~им~я";
        non_past_second: "~ишь~я", "~ите~ь";
        non_past_third: "~ит~я";
        imperative: "~и~ь|~ь~я", "~ите~ь|~ьте~ь";
        participle: "~ишь~я";
    ),
    // Заешь: 22 words (ешь, съешь, надоешь, выешь, разъешь, заешь).
    conjugation!("заешь", 22, "~шь";
        non_past_first: "~дим";
        non_past_second: "~шь", "~дите";
        non_past_third: "~ст";
        imperative: "~шь", "~шьте";
        participle: "~домый";
    ),
    // Отдашься: 20 words (издашься, удашься, сдашься, раздашься, выдашься, дашься).
    conjugation!("отдашься", 20, "~шь~я";
        non_past_first: "~дим~я";
        non_past_second: "~шь~я", "~дите~ь";
        non_past_third: "~~тся";
        imperative: "~й~я", "~йте~ь";
        participle: "~шь~я";
    ),
    // Набежишь: 20 words (бежишь, побежишь, избежишь, выбежишь, убежишь, сбежишь).
    conjugation!("набежишь", 20, "~ж~шь";
        non_past_first: "~ж~м";
        non_past_second: "~ж~шь", "~ж~те";
        non_past_third: "~ж~т";
        imperative: "~г~", "~г~те";
        participle: "~ж~шь";
    ),
    // Ляжешь: 20 words (ляжешь, приляжешь, заляжешь, обляжешь, проляжешь, наляжешь).
    conjugation!("ляжешь", 20, "~жешь";
        non_past_first: "~жем";
        non_past_second: "~жешь", "~жете";
        non_past_third: "~жет";
        imperative: "~г", "~гте";
        participle: "~емый";
    ),
    // Въешься: 17 words (наешься, въешься, объешься, приешься, взъешься, заешься).
    conjugation!("въешься", 17, "~шь~я";
        non_past_first: "~дим~я";
        non_past_second: "~шь~я", "~дите~ь";
        non_past_third: "~~тся";
        imperative: "~шь~я", "~шьте~ь";
        participle: "~шь~я";
    ),
    // Поедешь: 16 words (поедешь, приедешь, уедешь, въедешь, проедешь, выедешь).
    conjugation!("поедешь", 16, "~дешь";
        non_past_first: "~дем";
        non_past_second: "~дешь", "~дете";
        non_past_third: "~дет";
        imperative: "~зжай", "~зжайте";
        participle: "~емый";
    ),
    // Робишь: 16 words (чистишь, портишь, корчишь, горбишь, робишь, бесчестишь).
    conjugation!("робишь", 16, "~ишь";
        non_past_first: "~им";
        non_past_second: "~ишь", "~ите";
        non_past_third: "~ит";
        imperative: "~и|~ь", "~ите|~ьте";
        participle: "~имый";
    ),
    // Поможешь: 15 words (можешь, сможешь, поможешь, выжжешь, выстрижешь, изнеможешь).
    conjugation!("поможешь", 15, "~жешь";
        non_past_first: "~жем";
        non_past_second: "~жешь", "~жете";
        non_past_third: "~жет";
        imperative: "~ги", "~гите";
        participle: "~гомый";
    ),
    // Засыплешь: 15 words (засыплешь, сыплешь, насыплешь, высыплешь, посыплешь, просыплешь).
    conjugation!("засыплешь", 15, "~лешь";
        non_past_first: "~лем|~ем";
        non_past_second: "~лешь|~ешь", "~лете|~ете";
        non_past_third: "~лет|~ет";
        imperative: "~ь", "~ьте";
        participle: "~лемый";
    ),
    // Морщишься: 13 words (таращишься, морщишься, понадобишься, осведомишься, поморщишься, сморщишься).
    conjugation!("морщишься", 13, "~ишь~я";
        non_past_first: "~им~я";
        non_past_second: "~ишь~я", "~ите~ь";
        non_past_third: "~ит~я";
        imperative: "~и~ь|~ь~я", "~ьте~ь";
        participle: "~ишь~я";
    ),
    // Дремлешь: 13 words (дремлешь, задремлешь, внемлешь, приемлешь, восприемлешь, предприемлешь).
    conjugation!("дремлешь", 13, "~емлешь";
        non_past_first: "~емлем";
        non_past_second: "~емлешь", "~емлете";
        non_past_third: "~емлет";
        imperative: "~емли", "~емлите";
        participle: "~емлемый";
    ),
    // Вымажешь: 9 words (выдвинешь, вылезешь, вырежешь, высунешь, кашлянешь, выплюнешь).
    conjugation!("вымажешь", 9, "~ешь";
        non_past_first: "~ем";
        non_past_second: "~ешь", "~ете";
        non_past_third: "~ет";
        imperative: "~и|~ь", "~ьте";
        participle: "~емый";
    ),
    // Выпьешь: 8 words (выпьешь, вышьешь, выбьешь, выльешь, подвыпьешь, вывьешь).
    conjugation!("выпьешь", 8, "~ь~шь";
        non_past_first: "~ь~м";
        non_past_second: "~ь~шь", "~ь~те";
        non_past_third: "~ь~т";
        imperative: "~~й", "~~йте";
        participle: "~ь~шь";
    ),
    // Сыплешься: 7 words (просыплешься, рассыплешься, посыплешься, сыплешься, осыплешься, высыплешься).
    conjugation!("сыплешься", 7, "~лешь~я";
        non_past_first: "~лем~я|~ем~я";
        non_past_second: "~лешь~я|~ешь~я", "~лете~ь|~ете~ь";
        non_past_third: "~лет~я|~ет~я";
        imperative: "~ь~я", "~ьте~ь";
        participle: "~лешь~я";
    ),
    // Выдвинешься: 5 words (высунешься, выдвинешься, грянешься, откашлянешься, самовыдвинешься).
    conjugation!("выдвинешься", 5, "~ешь~я";
        non_past_first: "~ем~я";
        non_past_second: "~ешь~я", "~ете~ь";
        non_past_third: "~ет~я";
        imperative: "~и~ь", "~ьте~ь";
        participle: "~ешь~я";
    ),
    // Пробежишься: 5 words (разбежишься, сбежишься, пробежишься, поразбежишься, оббежишься).
    conjugation!("пробежишься", 5, "~ж~шь~я";
        non_past_first: "~ж~м~я";
        non_past_second: "~ж~шь~я", "~ж~те~ь";
        non_past_third: "~ж~т~я";
        imperative: "~г~~ь", "~г~те~ь";
        participle: "~ж~шь~я";
    ),
    // Выпряжешься: 5 words (разнеможешься, выпряжешься, выстрижешься, занеможешься, переможешься).
    conjugation!("выпряжешься", 5, "~жешь~я";
        non_past_first: "~жем~я";
        non_past_second: "~жешь~я", "~жете~ь";
        non_past_third: "~жет~я";
        imperative: "~ги~ь", "~гите~ь";
        participle: "~жешь~я";
    ),
    // Выльешься: 5 words (выбьешься, выльешься, выпьешься, вывьешься, повыбьешься).
    conjugation!("выльешься", 5, "~ь~шь~я";
        non_past_first: "~ь~м~я";
        non_past_second: "~ь~шь~я", "~~йте~ь";
        non_past_third: "~ь~т~я";
        imperative: "~~й~я", "~~йте~ь";
        participle: "~ь~шь~я";
    ),
    // Высечешь: 4 words (высечешь, вытечешь, выволочешь, выпечешь).
    conjugation!("высечешь", 4, "~чешь";
        non_past_first: "~чем";
        non_past_second: "~чешь", "~чете";
        non_past_third: "~чет";
        imperative: "~ки", "~ките";
        participle: "~комый";
    ),
    // Уляжешься: 4 words (уляжешься, поуляжешься, разляжешься, приуляжешься).
    conjugation!("уляжешься", 4, "~жешь~я";
        non_past_first: "~жем~я";
        non_past_second: "~жешь~я", "~жете~ь";
        non_past_third: "~жет~я";
        imperative: "~г~я", "~гте~ь";
        participle: "~жешь~я";
    ),
    // Изопьёшь: 4 words (разошьёшь, изопьёшь, разопьёшь, изошьёшь).
    conjugation!("изопьёшь", 4, "~зо~ьёшь";
        non_past_first: "~зо~ьём";
        non_past_second: "~зо~ьёшь", "~зо~ьёте";
        non_past_third: "~зо~ьёт";
        imperative: "~с~ей", "~с~ейте";
        participle: "~зо~ьёшь";
    ),
    // Хочешь: 3 words (хочешь, захочешь, расхочешь).
    conjugation!("хочешь", 3, "~чешь";
        non_past_first: "~тим";
        non_past_second: "~чешь", "~тите";
        non_past_third: "~чет";
        imperative: "~ти", "~тите";
        participle: "~емый";
    ),
    // Разъедешься: 3 words (съедешься, разъедешься, проедешься).
    conjugation!("разъедешься", 3, "~дешь~я";
        non_past_first: "~дем~я";
        non_past_second: "~дешь~я", "~дете~ь";
        non_past_third: "~дет~я";
        imperative: "~зжай~я", "~зжайте~ь";
        participle: "~дешь~я";
    ),
    // Чтёшься: 3 words (чтёшься, прядёшься, накладёшься).
    conjugation!("чтёшься", 3, "~ёшь~я";
        non_past_first: "~ём~я";
        non_past_second: "~ёшь~я", "~ите~ь";
        non_past_third: "~ёт~я";
        imperative: "~и~ь", "~ите~ь";
        participle: "~ёшь~я";
    ),
    // Выпечешься: 3 words (выволочешься, выпечешься, высечешься).
    conjugation!("выпечешься", 3, "~чешь~я";
        non_past_first: "~чем~я";
        non_past_second: "~чешь~я", "~чете~ь";
        non_past_third: "~чет~я";
        imperative: "~ки~ь", "~ките~ь";
        participle: "~чешь~я";
    ),
    // Изопьёшься: 3 words (разопьёшься, разошьёшься, изопьёшься).
    conjugation!("изопьёшься", 3, "~зо~ьёшь~я";
        non_past_first: "~зо~ьём~я";
        non_past_second: "~зо~ьёшь~я", "~зо~ьёте~ь";
        non_past_third: "~зо~ьёт~я";
        imperative: "~с~ей~я", "~с~ейте~ь";
        participle: "~зо~ьёшь~я";
    ),
    // Есть: 1 words (есть).
    conjugation!("есть", 1, "есть";
        non_past_first: "есть|есмы|будем";
        non_past_second: "есть|еси|есь|будешь", "есть|есте|будете";
        non_past_third: "есть|будет";
        imperative: "будь|буди", "будьте";
        participle: "есть";
    ),
];
