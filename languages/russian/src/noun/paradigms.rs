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
    /// Like `иём`: 17,249 words.
    Иём,
    /// Like `бик`: 6,167 words.
    Бик,
    /// Like `онность`: 5,038 words.
    Онность,
    /// Like `пление`: 4,302 words.
    Пление,
    /// Like `игра`: 4,260 words.
    Игра,
    /// Like `ванка`: 3,714 words.
    Ванка,
    /// Like `ния`: 3,699 words.
    Ния,
    /// Like `као`: 2,648 words.
    Као,
    /// Like `коладница`: 1,683 words.
    Коладница,
    /// Like `сходство`: 1,665 words.
    Сходство,
    /// Like `вонец`: 1,636 words.
    Вонец,
    /// Like `опанье`: 1,527 words.
    Опанье,
    /// Like `рожка`: 1,378 words.
    Рожка,
    /// Like `ведь`: 1,310 words.
    Ведь,
    /// Like `тяга`: 1,210 words.
    Тяга,
    /// Like `варищ`: 753 words.
    Варищ,
    /// Like `рожок`: 654 words.
    Рожок,
    /// Like `сдача`: 477 words.
    Сдача,
    /// Like `ний`: 471 words.
    Ний,
    /// Like `змей`: 429 words.
    Змей,
    /// Like `барашек`: 353 words.
    Барашек,
    /// Like `подин`: 305 words.
    Подин,
    /// Like `кладная`: 288 words.
    Кладная,
    /// Like `двойка`: 276 words.
    Двойка,
    /// Like `лёный`: 234 words.
    Лёный,
    /// Like `дище`: 214 words.
    Дище,
    /// Like `ключ`: 200 words.
    Ключ,
    /// Like `оня`: 194 words.
    Оня,
    /// Like `импиец`: 183 words.
    Импиец,
    /// Like `осёвые`: 182 words.
    Осёвые,
    /// Like `счёты`: 167 words.
    Счёты,
    /// Like `ский`: 157 words.
    Ский,
    /// Like `рвь`: 144 words.
    Рвь,
    /// Like `арея`: 135 words.
    Арея,
    /// Like `шпилька`: 132 words.
    Шпилька,
    /// Like `латы`: 130 words.
    Латы,
    /// Like `умелец`: 120 words.
    Умелец,
    /// Like `фудья`: 106 words.
    Фудья,
    /// Like `зги`: 106 words.
    Зги,
    /// Like `мечко`: 103 words.
    Мечко,
    /// Like `вна`: 102 words.
    Вна,
    /// Like `вое`: 99 words.
    Вое,
    /// Like `т-пылесос`: 96 words.
    Тпылесос,
    /// Like `постав`: 92 words.
    Постав,
    /// Like `мчий`: 91 words.
    Мчий,
    /// Like `воротня`: 83 words.
    Воротня,
    /// Like `дьяволёнок`: 81 words.
    Дьяволёнок,
    /// Like `бень`: 78 words.
    Бень,
    /// Like `отец`: 77 words.
    Отец,
    /// Like `заря`: 71 words.
    Заря,
    /// Like `роп`: 69 words.
    Роп,
    /// Like `бри`: 68 words.
    Бри,
    /// Like `ивки`: 68 words.
    Ивки,
    /// Like `рафной`: 65 words.
    Рафной,
    /// Like `лички`: 63 words.
    Лички,
    /// Like `зац`: 62 words.
    Зац,
    /// Like `очь`: 57 words.
    Очь,
    /// Like `топримечательность`: 57 words.
    Топримечательность,
    /// Like `альня`: 56 words.
    Альня,
    /// Like `ерская`: 55 words.
    Ерская,
    /// Like `ер-тральщик`: 52 words.
    Ертральщик,
    /// Like `лёд`: 49 words.
    Лёд,
    /// Like `вел`: 42 words.
    Вел,
    /// Like `ядро`: 41 words.
    Ядро,
    /// Like `ата`: 41 words.
    Ата,
    /// Like `жое`: 40 words.
    Жое,
    /// Like `учонок`: 37 words.
    Учонок,
    /// Like `езжая`: 37 words.
    Езжая,
    /// Like `каракуля`: 36 words.
    Каракуля,
    /// Like `жьё`: 35 words.
    Жьё,
    /// Like `еча`: 34 words.
    Еча,
    /// Like `ещи`: 34 words.
    Ещи,
    /// Like `жняк`: 33 words.
    Жняк,
    /// Like `ючее`: 29 words.
    Ючее,
    /// Like `к-охотник`: 29 words.
    Кохотник,
    /// Like `ейта-пикколо`: 29 words.
    Ейтапикколо,
    /// Like `шью`: 28 words.
    Шью,
    /// Like `-литра`: 26 words.
    Литра,
    /// Like `щит`: 25 words.
    Щит,
    /// Like `лище`: 25 words.
    Лище,
    /// Like `руя`: 24 words.
    Руя,
    /// Like `член`: 23 words.
    Член,
    /// Like `ашние`: 23 words.
    Ашние,
    /// Like `к-птицеед`: 23 words.
    Кптицеед,
    /// Like `язги`: 22 words.
    Язги,
    /// Like `центировка`: 20 words.
    Центировка,
    /// Like `ел-хранитель`: 20 words.
    Елхранитель,
    /// Like `егерь`: 18 words.
    Егерь,
    /// Like `ребро`: 18 words.
    Ребро,
    /// Like `шов`: 17 words.
    Шов,
    /// Like `нцы`: 17 words.
    Нцы,
    /// Like `боёк`: 17 words.
    Боёк,
    /// Like `льё`: 17 words.
    Льё,
    /// Like `бедь-шипун`: 17 words.
    Бедьшипун,
    /// Like `нос`: 16 words.
    Нос,
    /// Like `звезда`: 16 words.
    Звезда,
    /// Like `цапля`: 16 words.
    Цапля,
    /// Like `акуйя`: 16 words.
    Акуйя,
    /// Like `копаемое`: 16 words.
    Копаемое,
    /// Like `пка-ушанка`: 16 words.
    Пкаушанка,
    /// Like `лицо`: 15 words.
    Лицо,
    /// Like `кота`: 15 words.
    Кота,
    /// Like `лёж`: 15 words.
    Лёж,
    /// Like `ка-ёжка`: 15 words.
    Каёжка,
    /// Like `клин`: 14 words.
    Клин,
    /// Like `ребье`: 14 words.
    Ребье,
    /// Like `зёл`: 13 words.
    Зёл,
    /// Like `бои`: 13 words.
    Бои,
    /// Like `дья`: 13 words.
    Дья,
    /// Like `т-скоба`: 13 words.
    Тскоба,
    /// Like `иня-мать`: 13 words.
    Инямать,
    /// Like `т-извещение`: 13 words.
    Тизвещение,
    /// Like `имя`: 12 words.
    Имя,
    /// Like `очко`: 12 words.
    Очко,
    /// Like `уёбище`: 12 words.
    Уёбище,
    /// Like `требитель-бомбардировщик`: 12 words.
    Требительбомбардировщик,
    /// Like `метра`: 12 words.
    Метра,
    /// Like `а-лоцман`: 12 words.
    Алоцман,
    /// Like `лев`: 11 words.
    Лев,
    /// Like `яток`: 11 words.
    Яток,
    /// Like `тёночек`: 11 words.
    Тёночек,
    /// Like `урма`: 11 words.
    Урма,
    /// Like `ельки`: 11 words.
    Ельки,
    /// Like `дце`: 11 words.
    Дце,
    /// Like `аник-водитель`: 11 words.
    Аникводитель,
    /// Like `-корреспондент`: 11 words.
    Корреспондент,
    /// Like `пень`: 10 words.
    Пень,
    /// Like `ловия`: 10 words.
    Ловия,
    /// Like `калии`: 10 words.
    Калии,
    /// Like `бро`: 10 words.
    Бро,
    /// Like `едья`: 10 words.
    Едья,
    /// Like `ква-река`: 10 words.
    Кварека,
    /// Like `тье-жакет`: 10 words.
    Тьежакет,
    /// Like `гнездо`: 9 words.
    Гнездо,
    /// Like `хуй`: 9 words.
    Хуй,
    /// Like `льце`: 9 words.
    Льце,
    /// Like `гол-связка`: 9 words.
    Голсвязка,
    /// Like `инос`: 9 words.
    Инос,
    /// Like `атр-студия`: 9 words.
    Атрстудия,
    /// Like `дом`: 8 words.
    Дом,
    /// Like `зубок`: 8 words.
    Зубок,
    /// Like `перо`: 8 words.
    Перо,
    /// Like `вер`: 8 words.
    Вер,
    /// Like `бойня`: 8 words.
    Бойня,
    /// Like `шеек`: 8 words.
    Шеек,
    /// Like `н-камикадзе`: 8 words.
    Нкамикадзе,
    /// Like `дюжины`: 8 words.
    Дюжины,
    /// Like `в-призрак`: 8 words.
    Впризрак,
    /// Like `-юбки`: 8 words.
    Юбки,
    /// Like `ечо`: 7 words.
    Ечо,
    /// Like `дух`: 7 words.
    Дух,
    /// Like `кла`: 7 words.
    Кла,
    /// Like `кой`: 7 words.
    Кой,
    /// Like `жой`: 7 words.
    Жой,
    /// Like `счёт`: 7 words.
    Счёт,
    /// Like `дняя`: 7 words.
    Дняя,
    /// Like `сьмо`: 7 words.
    Сьмо,
    /// Like `тце`: 7 words.
    Тце,
    /// Like `тье-майка`: 7 words.
    Тьемайка,
    /// Like `-июля`: 7 words.
    Июля,
    /// Like `-игры`: 7 words.
    Игры,
    /// Like `мка-пакет`: 7 words.
    Мкапакет,
    /// Like `зло`: 6 words.
    Зло,
    /// Like `жнивьё`: 6 words.
    Жнивьё,
    /// Like `ашников`: 6 words.
    Ашников,
    /// Like `ложь`: 6 words.
    Ложь,
    /// Like `ерко`: 6 words.
    Ерко,
    /// Like `сло-качалка`: 6 words.
    Слокачалка,
    /// Like `мьё`: 6 words.
    Мьё,
    /// Like `-лета`: 6 words.
    Лета,
    /// Like `бой`: 5 words.
    Бой,
    /// Like `йга`: 5 words.
    Йга,
    /// Like `горе`: 5 words.
    Горе,
    /// Like `круг`: 5 words.
    Круг,
    /// Like `зболивающее`: 5 words.
    Зболивающее,
    /// Like `кча`: 5 words.
    Кча,
    /// Like `зеленя`: 5 words.
    Зеленя,
    /// Like `пики`: 5 words.
    Пики,
    /// Like `коть`: 5 words.
    Коть,
    /// Like `злы`: 5 words.
    Злы,
    /// Like `пей`: 5 words.
    Пей,
    /// Like `а-ловушка`: 5 words.
    Аловушка,
    /// Like `бега`: 5 words.
    Бега,
    /// Like `а-малаша`: 5 words.
    Амалаша,
    /// Like `джак-болеро`: 5 words.
    Джакболеро,
    /// Like `вь-морковь`: 5 words.
    Вьморковь,
    /// Like `д-сад`: 5 words.
    Дсад,
    /// Like `тье-фрак`: 5 words.
    Тьефрак,
    /// Like `фий`: 4 words.
    Фий,
    /// Like `дичь`: 4 words.
    Дичь,
    /// Like `весна`: 4 words.
    Весна,
    /// Like `улей`: 4 words.
    Улей,
    /// Like `ящийся`: 4 words.
    Ящийся,
    /// Like `фья`: 4 words.
    Фья,
    /// Like `арин`: 4 words.
    Арин,
    /// Like `горошек`: 4 words.
    Горошек,
    /// Like `сынуля`: 4 words.
    Сынуля,
    /// Like `мыкающееся`: 4 words.
    Мыкающееся,
    /// Like `од-богоносец`: 4 words.
    Одбогоносец,
    /// Like `ерочек`: 4 words.
    Ерочек,
    /// Like `рубашечник`: 4 words.
    Рубашечник,
    /// Like `ач-интерн`: 4 words.
    Ачинтерн,
    /// Like `-пилот`: 4 words.
    Пилот,
    /// Like `ельце`: 4 words.
    Ельце,
    /// Like `тко`: 4 words.
    Тко,
    /// Like `юхино`: 4 words.
    Юхино,
    /// Like `ск-кузнецкий`: 4 words.
    Сккузнецкий,
    /// Like `прея`: 4 words.
    Прея,
    /// Like `адьба`: 3 words.
    Адьба,
    /// Like `угол`: 3 words.
    Угол,
    /// Like `боец`: 3 words.
    Боец,
    /// Like `желна`: 3 words.
    Желна,
    /// Like `чёлн`: 3 words.
    Чёлн,
    /// Like `дверца`: 3 words.
    Дверца,
    /// Like `макаронник`: 3 words.
    Макаронник,
    /// Like `шай`: 3 words.
    Шай,
    /// Like `рть`: 3 words.
    Рть,
    /// Like `чья`: 3 words.
    Чья,
    /// Like `вела`: 3 words.
    Вела,
    /// Like `тиса`: 3 words.
    Тиса,
    /// Like `уйки`: 3 words.
    Уйки,
    /// Like `хня`: 3 words.
    Хня,
    /// Like `едостениё`: 3 words.
    Едостениё,
    /// Like `иум`: 3 words.
    Иум,
    /// Like `ясли`: 3 words.
    Ясли,
    /// Like `лнышко`: 3 words.
    Лнышко,
    /// Like `ушка-пердушка`: 3 words.
    Ушкапердушка,
    /// Like `к-одиночка`: 3 words.
    Кодиночка,
    /// Like `мка-тележка`: 3 words.
    Мкатележка,
    /// Like `б-двойка`: 3 words.
    Бдвойка,
    /// Like `гопа`: 3 words.
    Гопа,
    /// Like `ведь-губач`: 3 words.
    Ведьгубач,
    /// Like `ц-регент`: 3 words.
    Црегент,
    /// Like `тан-шутиха`: 3 words.
    Таншутиха,
    /// Like `тье-ампир`: 3 words.
    Тьеампир,
    /// Like `-леса`: 3 words.
    Леса,
    /// Like `на-изгой`: 3 words.
    Наизгой,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 237] = [
        Declension::Иём,
        Declension::Бик,
        Declension::Онность,
        Declension::Пление,
        Declension::Игра,
        Declension::Ванка,
        Declension::Ния,
        Declension::Као,
        Declension::Коладница,
        Declension::Сходство,
        Declension::Вонец,
        Declension::Опанье,
        Declension::Рожка,
        Declension::Ведь,
        Declension::Тяга,
        Declension::Варищ,
        Declension::Рожок,
        Declension::Сдача,
        Declension::Ний,
        Declension::Змей,
        Declension::Барашек,
        Declension::Подин,
        Declension::Кладная,
        Declension::Двойка,
        Declension::Лёный,
        Declension::Дище,
        Declension::Ключ,
        Declension::Оня,
        Declension::Импиец,
        Declension::Осёвые,
        Declension::Счёты,
        Declension::Ский,
        Declension::Рвь,
        Declension::Арея,
        Declension::Шпилька,
        Declension::Латы,
        Declension::Умелец,
        Declension::Фудья,
        Declension::Зги,
        Declension::Мечко,
        Declension::Вна,
        Declension::Вое,
        Declension::Тпылесос,
        Declension::Постав,
        Declension::Мчий,
        Declension::Воротня,
        Declension::Дьяволёнок,
        Declension::Бень,
        Declension::Отец,
        Declension::Заря,
        Declension::Роп,
        Declension::Бри,
        Declension::Ивки,
        Declension::Рафной,
        Declension::Лички,
        Declension::Зац,
        Declension::Очь,
        Declension::Топримечательность,
        Declension::Альня,
        Declension::Ерская,
        Declension::Ертральщик,
        Declension::Лёд,
        Declension::Вел,
        Declension::Ядро,
        Declension::Ата,
        Declension::Жое,
        Declension::Учонок,
        Declension::Езжая,
        Declension::Каракуля,
        Declension::Жьё,
        Declension::Еча,
        Declension::Ещи,
        Declension::Жняк,
        Declension::Ючее,
        Declension::Кохотник,
        Declension::Ейтапикколо,
        Declension::Шью,
        Declension::Литра,
        Declension::Щит,
        Declension::Лище,
        Declension::Руя,
        Declension::Член,
        Declension::Ашние,
        Declension::Кптицеед,
        Declension::Язги,
        Declension::Центировка,
        Declension::Елхранитель,
        Declension::Егерь,
        Declension::Ребро,
        Declension::Шов,
        Declension::Нцы,
        Declension::Боёк,
        Declension::Льё,
        Declension::Бедьшипун,
        Declension::Нос,
        Declension::Звезда,
        Declension::Цапля,
        Declension::Акуйя,
        Declension::Копаемое,
        Declension::Пкаушанка,
        Declension::Лицо,
        Declension::Кота,
        Declension::Лёж,
        Declension::Каёжка,
        Declension::Клин,
        Declension::Ребье,
        Declension::Зёл,
        Declension::Бои,
        Declension::Дья,
        Declension::Тскоба,
        Declension::Инямать,
        Declension::Тизвещение,
        Declension::Имя,
        Declension::Очко,
        Declension::Уёбище,
        Declension::Требительбомбардировщик,
        Declension::Метра,
        Declension::Алоцман,
        Declension::Лев,
        Declension::Яток,
        Declension::Тёночек,
        Declension::Урма,
        Declension::Ельки,
        Declension::Дце,
        Declension::Аникводитель,
        Declension::Корреспондент,
        Declension::Пень,
        Declension::Ловия,
        Declension::Калии,
        Declension::Бро,
        Declension::Едья,
        Declension::Кварека,
        Declension::Тьежакет,
        Declension::Гнездо,
        Declension::Хуй,
        Declension::Льце,
        Declension::Голсвязка,
        Declension::Инос,
        Declension::Атрстудия,
        Declension::Дом,
        Declension::Зубок,
        Declension::Перо,
        Declension::Вер,
        Declension::Бойня,
        Declension::Шеек,
        Declension::Нкамикадзе,
        Declension::Дюжины,
        Declension::Впризрак,
        Declension::Юбки,
        Declension::Ечо,
        Declension::Дух,
        Declension::Кла,
        Declension::Кой,
        Declension::Жой,
        Declension::Счёт,
        Declension::Дняя,
        Declension::Сьмо,
        Declension::Тце,
        Declension::Тьемайка,
        Declension::Июля,
        Declension::Игры,
        Declension::Мкапакет,
        Declension::Зло,
        Declension::Жнивьё,
        Declension::Ашников,
        Declension::Ложь,
        Declension::Ерко,
        Declension::Слокачалка,
        Declension::Мьё,
        Declension::Лета,
        Declension::Бой,
        Declension::Йга,
        Declension::Горе,
        Declension::Круг,
        Declension::Зболивающее,
        Declension::Кча,
        Declension::Зеленя,
        Declension::Пики,
        Declension::Коть,
        Declension::Злы,
        Declension::Пей,
        Declension::Аловушка,
        Declension::Бега,
        Declension::Амалаша,
        Declension::Джакболеро,
        Declension::Вьморковь,
        Declension::Дсад,
        Declension::Тьефрак,
        Declension::Фий,
        Declension::Дичь,
        Declension::Весна,
        Declension::Улей,
        Declension::Ящийся,
        Declension::Фья,
        Declension::Арин,
        Declension::Горошек,
        Declension::Сынуля,
        Declension::Мыкающееся,
        Declension::Одбогоносец,
        Declension::Ерочек,
        Declension::Рубашечник,
        Declension::Ачинтерн,
        Declension::Пилот,
        Declension::Ельце,
        Declension::Тко,
        Declension::Юхино,
        Declension::Сккузнецкий,
        Declension::Прея,
        Declension::Адьба,
        Declension::Угол,
        Declension::Боец,
        Declension::Желна,
        Declension::Чёлн,
        Declension::Дверца,
        Declension::Макаронник,
        Declension::Шай,
        Declension::Рть,
        Declension::Чья,
        Declension::Вела,
        Declension::Тиса,
        Declension::Уйки,
        Declension::Хня,
        Declension::Едостениё,
        Declension::Иум,
        Declension::Ясли,
        Declension::Лнышко,
        Declension::Ушкапердушка,
        Declension::Кодиночка,
        Declension::Мкатележка,
        Declension::Бдвойка,
        Declension::Гопа,
        Declension::Ведьгубач,
        Declension::Црегент,
        Declension::Таншутиха,
        Declension::Тьеампир,
        Declension::Леса,
        Declension::Наизгой,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 237] = [
    // Иём: 17,249 words (указ, зонт, кот, узус, скот, мэр).
    declension!("иём", 17249, "~";
        nominative: "~", "~ы";
        genitive: "~а", "~ов";
        dative: "~у", "~ам";
        accusative: referred, referred;
        instrumental: "~ом", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~а", "~е";
    ),
    // Бик: 6,167 words (бортник, язык, лук, вторник, гонг, жук).
    declension!("бик", 6167, "~";
        nominative: "~", "~и";
        genitive: "~а", "~ов";
        dative: "~у", "~ам";
        accusative: referred, referred;
        instrumental: "~ом", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~а", "~е";
    ),
    // Онность: 5,038 words (знать, весь, дверь, ртуть, путь, ель).
    declension!("онность", 5038, "~ь";
        nominative: "~ь", "~и";
        genitive: "~и", "~ей";
        dative: "~и", "~ям";
        accusative: referred, referred;
        instrumental: "~ью", "~ями";
        prepositional: "~и", "~ях";
        noun_second_case: "~и", "~и";
    ),
    // Пление: 4,302 words (мнение, учение, уединение, умиление, мгновение, движение).
    declension!("пление", 4302, "~е";
        nominative: "~е", "~я";
        genitive: "~я", "~й";
        dative: "~ю", "~ям";
        accusative: referred, referred;
        instrumental: "~ем", "~ями";
        prepositional: "~и", "~ях";
        noun_second_case: "~я", "~и";
    ),
    // Игра: 4,260 words (вата, камера, липа, сера, арба, лиса).
    declension!("игра", 4260, "~а";
        nominative: "~а", "~ы";
        genitive: "~ы", "~";
        dative: "~е", "~ам";
        accusative: "~у", referred;
        instrumental: "~ой|~ою", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~ы", "~е";
    ),
    // Ванка: 3,714 words (цыпка, юбка, ёлка, щека, цыганка, скалка).
    declension!("ванка", 3714, "~~а";
        nominative: "~~а", "~~и";
        genitive: "~~и", "~о~";
        dative: "~~е", "~~ам";
        accusative: "~~у", referred;
        instrumental: "~~ой|~~ою", "~~ами";
        prepositional: "~~е", "~~ах";
        noun_second_case: "~~и", "~~е";
    ),
    // Ния: 3,699 words (библия, талия, фтизиатрия, философия, магия, эйфория).
    declension!("ния", 3699, "~я";
        nominative: "~я", "~и";
        genitive: "~и", "~й";
        dative: "~и", "~ям";
        accusative: "~ю", referred;
        instrumental: "~ей|~ею", "~ями";
        prepositional: "~и", "~ях";
        noun_second_case: "~и", "~и";
    ),
    // Као: 2,648 words (что, кто, они, эссе, кофе, алоэ).
    declension!("као", 2648, "~";
        nominative: "~", "~";
        genitive: "~", "~";
        dative: "~", "~";
        accusative: referred, referred;
        instrumental: "~", "~";
        prepositional: "~", "~";
        noun_second_case: "~", "~";
    ),
    // Коладница: 1,683 words (царица, птица, пятница, улица, пшеница, цареубийца).
    declension!("коладница", 1683, "~а";
        nominative: "~а", "~ы";
        genitive: "~ы", "~";
        dative: "~е", "~ам";
        accusative: "~у", referred;
        instrumental: "~ей|~ею", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~ы", "~е";
    ),
    // Сходство: 1,665 words (око, лето, жало, нёбо, вино, яйцо).
    declension!("сходство", 1665, "~о";
        nominative: "~о", "~а";
        genitive: "~а", "~";
        dative: "~у", "~ам";
        accusative: referred, referred;
        instrumental: "~ом", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~а", "~е";
    ),
    // Вонец: 1,636 words (марганец, засранец, аароновец, лондонец, японец, младенец).
    declension!("вонец", 1636, "~е~";
        nominative: "~е~", "~~ы";
        genitive: "~~а", "~~ев";
        dative: "~~у", "~~ам";
        accusative: referred, referred;
        instrumental: "~~ем", "~~ами";
        prepositional: "~~е", "~~ах";
        noun_second_case: "~~а", "~~е";
    ),
    // Опанье: 1,527 words (уединенье, умиленье, мненье, сиденье, здоровье, счастье).
    declension!("опанье", 1527, "~ье";
        nominative: "~ье", "~ья";
        genitive: "~ья", "~ий";
        dative: "~ью", "~ьям";
        accusative: referred, referred;
        instrumental: "~ьем", "~ьями";
        prepositional: "~ье", "~ьях";
        noun_second_case: "~ья", "~ье";
    ),
    // Рожка: 1,378 words (бабочка, девочка, ёлочка, папочка, батюшка, улыбочка).
    declension!("рожка", 1378, "~~а";
        nominative: "~~а", "~~и";
        genitive: "~~и", "~е~";
        dative: "~~е", "~~ам";
        accusative: "~~у", referred;
        instrumental: "~~ой|~~ою", "~~ами";
        prepositional: "~~е", "~~ах";
        noun_second_case: "~~и", "~~е";
    ),
    // Ведь: 1,310 words (сшиватель, апрель, июнь, июль, чернобыль, авось).
    declension!("ведь", 1310, "~ь";
        nominative: "~ь", "~и";
        genitive: "~я", "~ей";
        dative: "~ю", "~ям";
        accusative: referred, referred;
        instrumental: "~ем", "~ями";
        prepositional: "~е", "~ях";
        noun_second_case: "~я", "~е";
    ),
    // Тяга: 1,210 words (книга, вага, река, азбука, бумага, нога).
    declension!("тяга", 1210, "~а";
        nominative: "~а", "~и";
        genitive: "~и", "~";
        dative: "~е", "~ам";
        accusative: "~у", referred;
        instrumental: "~ой|~ою", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~и", "~е";
    ),
    // Варищ: 753 words (харч, туш, треш, раж, путч, марш).
    declension!("варищ", 753, "~";
        nominative: "~", "~и";
        genitive: "~а", "~ей";
        dative: "~у", "~ам";
        accusative: referred, referred;
        instrumental: "~ем", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~а", "~е";
    ),
    // Рожок: 654 words (щенок, лесок, носок, подросток, цветок, пирожок).
    declension!("рожок", 654, "~о~";
        nominative: "~о~", "~~и";
        genitive: "~~а", "~~ов";
        dative: "~~у", "~~ам";
        accusative: referred, referred;
        instrumental: "~~ом", "~~ами";
        prepositional: "~~е", "~~ах";
        noun_second_case: "~~а", "~~е";
    ),
    // Сдача: 477 words (кожа, чаша, папаша, мрежа, тёща, пища).
    declension!("сдача", 477, "~а";
        nominative: "~а", "~и";
        genitive: "~и", "~";
        dative: "~е", "~ам";
        accusative: "~у", referred;
        instrumental: "~ей|~ею", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~и", "~е";
    ),
    // Ний: 471 words (иттрий, самарий, иттербий, кюрий, эйнштейний, сценарий).
    declension!("ний", 471, "~й";
        nominative: "~й", "~и";
        genitive: "~я", "~ев";
        dative: "~ю", "~ям";
        accusative: referred, referred;
        instrumental: "~ем", "~ями";
        prepositional: "~и", "~ях";
        noun_second_case: "~я", "~и";
    ),
    // Змей: 429 words (май, гей, елей, герой, случай, еврей).
    declension!("змей", 429, "~й";
        nominative: "~й", "~и";
        genitive: "~я", "~ев";
        dative: "~ю", "~ям";
        accusative: referred, referred;
        instrumental: "~ем", "~ями";
        prepositional: "~е", "~ях";
        noun_second_case: "~я", "~е";
    ),
    // Барашек: 353 words (орешек, листочек, ребёночек, сыночек, цветочек, ящичек).
    declension!("барашек", 353, "~е~";
        nominative: "~е~", "~~и";
        genitive: "~~а", "~~ов";
        dative: "~~у", "~~ам";
        accusative: referred, referred;
        instrumental: "~~ом", "~~ами";
        prepositional: "~~е", "~~ах";
        noun_second_case: "~~а", "~~е";
    ),
    // Подин: 305 words (египтянин, южанин, христианин, христадельфианин, дворянин, ельчанин).
    declension!("подин", 305, "~ин";
        nominative: "~ин", "~е";
        genitive: "~ина", "~";
        dative: "~ину", "~ам";
        accusative: referred, referred;
        instrumental: "~ином", "~ами";
        prepositional: "~ине", "~ах";
        noun_second_case: "~ина", "~ине";
    ),
    // Кладная: 288 words (уборная, борзая, знакомая, скорая, щитовая, пятая).
    declension!("кладная", 288, "~ая";
        nominative: "~ая", "~ые";
        genitive: "~ой", "~ых";
        dative: "~ой", "~ым";
        accusative: "~ую", referred;
        instrumental: "~ой|~ою", "~ыми";
        prepositional: "~ой", "~ых";
        noun_second_case: "~ой", "~ой";
    ),
    // Двойка: 276 words (ищейка, балалайка, хозяйка, уклейка, змейка, злодейка).
    declension!("двойка", 276, "~й~а";
        nominative: "~й~а", "~й~и";
        genitive: "~й~и", "~е~";
        dative: "~й~е", "~й~ам";
        accusative: "~й~у", referred;
        instrumental: "~й~ой|~й~ою", "~й~ами";
        prepositional: "~й~е", "~й~ах";
        noun_second_case: "~й~и", "~й~е";
    ),
    // Лёный: 234 words (учёный, согласный, знакомый, пьяный, православный, убитый).
    declension!("лёный", 234, "~ый";
        nominative: "~ый", "~ые";
        genitive: "~ого", "~ых";
        dative: "~ому", "~ым";
        accusative: referred, referred;
        instrumental: "~ым", "~ыми";
        prepositional: "~ом", "~ых";
        noun_second_case: "~ого", "~ом";
    ),
    // Дище: 214 words (вече, училище, ложе, убежище, зрелище, днище).
    declension!("дище", 214, "~е";
        nominative: "~е", "~а";
        genitive: "~а", "~";
        dative: "~у", "~ам";
        accusative: referred, referred;
        instrumental: "~ем", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~а", "~е";
    ),
    // Ключ: 200 words (борщ, врач, сыч, нож, меч, этаж).
    declension!("ключ", 200, "~";
        nominative: "~", "~и";
        genitive: "~а", "~ей";
        dative: "~у", "~ам";
        accusative: referred, referred;
        instrumental: "~ом", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~а", "~е";
    ),
    // Оня: 194 words (буря, няня, княгиня, тётя, пуля, яблоня).
    declension!("оня", 194, "~я";
        nominative: "~я", "~и";
        genitive: "~и", "~ь";
        dative: "~е", "~ям";
        accusative: "~ю", referred;
        instrumental: "~ей|~ею", "~ями";
        prepositional: "~е", "~ях";
        noun_second_case: "~и", "~е";
    ),
    // Импиец: 183 words (убиец, таец, гвардеец, зеец, беец, бхилаец).
    declension!("импиец", 183, "~е~";
        nominative: "~е~", "~й~ы";
        genitive: "~й~а", "~й~ев";
        dative: "~й~у", "~й~ам";
        accusative: referred, referred;
        instrumental: "~й~ем", "~й~ами";
        prepositional: "~й~е", "~й~ах";
        noun_second_case: "~й~а", "~й~е";
    ),
    // Осёвые: 182 words (жвалоносные, ивовые, мышиные, озимые, членистохоботные, сцифоидные).
    declension!("осёвые", 182, "~е";
        nominative: "~е", "~е";
        genitive: "~х", "~х";
        dative: "~м", "~м";
        accusative: referred, referred;
        instrumental: "~ми", "~ми";
        prepositional: "~х", "~х";
        noun_second_case: "~х", "~х";
    ),
    // Счёты: 167 words (часы, роды, щипцы, весы, азы, усы).
    declension!("счёты", 167, "~ы";
        nominative: "~ы", "~ы";
        genitive: "~ы", "~ов";
        dative: "~ы", "~ам";
        accusative: referred, referred;
        instrumental: "~ы", "~ами";
        prepositional: "~ы", "~ах";
        noun_second_case: "~ы", "~ы";
    ),
    // Ский: 157 words (египетский, белорусский, японский, шведский, пьемонтский, убогий).
    declension!("ский", 157, "~ий";
        nominative: "~ий", "~ие";
        genitive: "~ого", "~их";
        dative: "~ому", "~им";
        accusative: referred, referred;
        instrumental: "~им", "~ими";
        prepositional: "~ом", "~их";
        noun_second_case: "~ого", "~ом";
    ),
    // Рвь: 144 words (царь, январь, февраль, нуль, конь, кобель).
    declension!("рвь", 144, "~ь";
        nominative: "~ь", "~и";
        genitive: "~я", "~ей";
        dative: "~ю", "~ям";
        accusative: referred, referred;
        instrumental: "~ём", "~ями";
        prepositional: "~е", "~ях";
        noun_second_case: "~я", "~е";
    ),
    // Арея: 135 words (стая, шея, идея, фея, ономатопея, ахинея).
    declension!("арея", 135, "~я";
        nominative: "~я", "~и";
        genitive: "~и", "~й";
        dative: "~е", "~ям";
        accusative: "~ю", referred;
        instrumental: "~ей|~ею", "~ями";
        prepositional: "~е", "~ях";
        noun_second_case: "~и", "~е";
    ),
    // Шпилька: 132 words (шпилька, нянька, дяденька, дядька, монополька, ноженька).
    declension!("шпилька", 132, "~ь~а";
        nominative: "~ь~а", "~ь~и";
        genitive: "~ь~и", "~е~";
        dative: "~ь~е", "~ь~ам";
        accusative: "~ь~у", referred;
        instrumental: "~ь~ой|~ь~ою", "~ь~ами";
        prepositional: "~ь~е", "~ь~ах";
        noun_second_case: "~ь~и", "~ь~е";
    ),
    // Латы: 130 words (ноты, вилы, соты, путы, чары, узы).
    declension!("латы", 130, "~ы";
        nominative: "~ы", "~ы";
        genitive: "~ы", "~";
        dative: "~ы", "~ам";
        accusative: referred, referred;
        instrumental: "~ы", "~ами";
        prepositional: "~ы", "~ах";
        noun_second_case: "~ы", "~ы";
    ),
    // Умелец: 120 words (земледелец, умелец, скиталец, сычуанец, аомынец, ковелец).
    declension!("умелец", 120, "~е~";
        nominative: "~е~", "~ь~ы";
        genitive: "~ь~а", "~ь~ев";
        dative: "~ь~у", "~ь~ам";
        accusative: referred, referred;
        instrumental: "~ь~ем", "~ь~ами";
        prepositional: "~ь~е", "~ь~ах";
        noun_second_case: "~ь~а", "~ь~е";
    ),
    // Фудья: 106 words (бомбилья, лгунья, ждунья, ефросинья, хрипунья, цапунья).
    declension!("фудья", 106, "~ья";
        nominative: "~ья", "~ьи";
        genitive: "~ьи", "~ий";
        dative: "~ье", "~ьям";
        accusative: "~ью", referred;
        instrumental: "~ьей|~ьею", "~ьями";
        prepositional: "~ье", "~ьях";
        noun_second_case: "~ьи", "~ье";
    ),
    // Зги: 106 words (духи, щипчики, очки, сиги, укронаци, ессентуки).
    declension!("зги", 106, "~и";
        nominative: "~и", "~и";
        genitive: "~и", "~ов";
        dative: "~и", "~ам";
        accusative: referred, referred;
        instrumental: "~и", "~ами";
        prepositional: "~и", "~ах";
        noun_second_case: "~и", "~и";
    ),
    // Мечко: 103 words (яичко, яблочко, пятнышко, ушко, гнёздышко, ядрышко).
    declension!("мечко", 103, "~~о";
        nominative: "~~о", "~~и";
        genitive: "~~а", "~е~";
        dative: "~~у", "~~ам";
        accusative: referred, referred;
        instrumental: "~~ом", "~~ами";
        prepositional: "~~е", "~~ах";
        noun_second_case: "~~а", "~~е";
    ),
    // Вна: 102 words (овца, царевна, хрипотца, ивановна, дмитриевна, львовна).
    declension!("вна", 102, "~~а";
        nominative: "~~а", "~~ы";
        genitive: "~~ы", "~е~";
        dative: "~~е", "~~ам";
        accusative: "~~у", referred;
        instrumental: "~~ой|~~ою", "~~ами";
        prepositional: "~~е", "~~ах";
        noun_second_case: "~~ы", "~~е";
    ),
    // Вое: 99 words (множимое, уменьшаемое, второе, означаемое, рвотное, ожидаемое).
    declension!("вое", 99, "~ое";
        nominative: "~ое", "~ые";
        genitive: "~ого", "~ых";
        dative: "~ому", "~ым";
        accusative: referred, referred;
        instrumental: "~ым", "~ыми";
        prepositional: "~ом", "~ых";
        noun_second_case: "~ого", "~ом";
    ),
    // Тпылесос: 96 words (радор-ретривер, лёт-снаряд, столет-пулемёт, гон-ресторан, т-провокатор, т-пылесос).
    declension!("т-пылесос", 96, "~~";
        nominative: "~~", "~ы~ы";
        genitive: "~а~а", "~ов~ов";
        dative: "~у~у", "~ам~ам";
        accusative: referred, referred;
        instrumental: "~ом~ом", "~ами~ами";
        prepositional: "~е~е", "~ах~ах";
        noun_second_case: "~а~а", "~е~е";
    ),
    // Постав: 92 words (город, пояс, бок, остров, берег, юг).
    declension!("постав", 92, "~";
        nominative: "~", "~а";
        genitive: "~а", "~ов";
        dative: "~у", "~ам";
        accusative: referred, referred;
        instrumental: "~ом", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~а", "~е";
    ),
    // Мчий: 91 words (рабочий, умерший, умирающий, уцелевший, зрячий, сничий).
    declension!("мчий", 91, "~ий";
        nominative: "~ий", "~ие";
        genitive: "~его", "~их";
        dative: "~ему", "~им";
        accusative: referred, referred;
        instrumental: "~им", "~ими";
        prepositional: "~ем", "~их";
        noun_second_case: "~его", "~ем";
    ),
    // Воротня: 83 words (дворня, овчарня, псарня, мыловарня, мытня, яишня).
    declension!("воротня", 83, "~~я";
        nominative: "~~я", "~~и";
        genitive: "~~и", "~е~";
        dative: "~~е", "~~ям";
        accusative: "~~ю", referred;
        instrumental: "~~ей|~~ею", "~~ями";
        prepositional: "~~е", "~~ях";
        noun_second_case: "~~и", "~~е";
    ),
    // Дьяволёнок: 81 words (телёнок, цыплёнок, дьяволёнок, цыганёнок, львёнок, опёнок).
    declension!("дьяволёнок", 81, "~ёнок";
        nominative: "~ёнок", "~ята";
        genitive: "~ёнка", "~ят";
        dative: "~ёнку", "~ятам";
        accusative: referred, referred;
        instrumental: "~ёнком", "~ятами";
        prepositional: "~ёнке", "~ятах";
        noun_second_case: "~ёнка", "~ёнке";
    ),
    // Бень: 78 words (полдень, кашель, стебель, трутень, тигель, слизень).
    declension!("бень", 78, "~е~ь";
        nominative: "~е~ь", "~~и";
        genitive: "~~я", "~~ей";
        dative: "~~ю", "~~ям";
        accusative: referred, referred;
        instrumental: "~~ем", "~~ями";
        prepositional: "~~е", "~~ях";
        noun_second_case: "~~я", "~~е";
    ),
    // Отец: 77 words (отец, дворец, узел, вдовец, дятел, овен).
    declension!("отец", 77, "~е~";
        nominative: "~е~", "~~ы";
        genitive: "~~а", "~~ов";
        dative: "~~у", "~~ам";
        accusative: referred, referred;
        instrumental: "~~ом", "~~ами";
        prepositional: "~~е", "~~ах";
        noun_second_case: "~~а", "~~е";
    ),
    // Заря: 71 words (тля, заря, лыжня, петля, пятерня, гэбня).
    declension!("заря", 71, "~я";
        nominative: "~я", "~и";
        genitive: "~и", "~ей";
        dative: "~е", "~ям";
        accusative: "~ю", referred;
        instrumental: "~ёй|~ёю", "~ями";
        prepositional: "~е", "~ях";
        noun_second_case: "~и", "~е";
    ),
    // Роп: 69 words (град, яд, род, год, воз, брод).
    declension!("роп", 69, "~";
        nominative: "~", "~ы";
        genitive: "~а", "~ов";
        dative: "~у", "~ам";
        accusative: referred, referred;
        instrumental: "~ом", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~у", "~е";
    ),
    // Бри: 68 words (люди, сани, вини, сени, буби, бели).
    declension!("бри", 68, "~и";
        nominative: "~и", "~и";
        genitive: "~и", "~ей";
        dative: "~и", "~ям";
        accusative: referred, referred;
        instrumental: "~и", "~ями";
        prepositional: "~и", "~ях";
        noun_second_case: "~и", "~и";
    ),
    // Ивки: 68 words (шмотки, жмурки, цыпки, нитянки, яйчоночки, посадки).
    declension!("ивки", 68, "~~и";
        nominative: "~~и", "~~и";
        genitive: "~~и", "~о~";
        dative: "~~и", "~~ам";
        accusative: referred, referred;
        instrumental: "~~и", "~~ами";
        prepositional: "~~и", "~~ах";
        noun_second_case: "~~и", "~~и";
    ),
    // Рафной: 65 words (золотой, гнедой, портной, рядовой, хромой, домовой).
    declension!("рафной", 65, "~ой";
        nominative: "~ой", "~ые";
        genitive: "~ого", "~ых";
        dative: "~ому", "~ым";
        accusative: referred, referred;
        instrumental: "~ым", "~ыми";
        prepositional: "~ом", "~ых";
        noun_second_case: "~ого", "~ом";
    ),
    // Лички: 63 words (цыпочки, пятнашки, щишки, салочки, саночки, плавочки).
    declension!("лички", 63, "~~и";
        nominative: "~~и", "~~и";
        genitive: "~~и", "~е~";
        dative: "~~и", "~~ам";
        accusative: referred, referred;
        instrumental: "~~и", "~~ами";
        prepositional: "~~и", "~~ах";
        noun_second_case: "~~и", "~~и";
    ),
    // Зац: 62 words (плац, фриц, поц, герц, шлиц, шпиц).
    declension!("зац", 62, "~";
        nominative: "~", "~ы";
        genitive: "~а", "~ев";
        dative: "~у", "~ам";
        accusative: referred, referred;
        instrumental: "~ем", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~а", "~е";
    ),
    // Очь: 57 words (ночь, мочь, тушь, печь, речь, течь).
    declension!("очь", 57, "~ь";
        nominative: "~ь", "~и";
        genitive: "~и", "~ей";
        dative: "~и", "~ам";
        accusative: referred, referred;
        instrumental: "~ью", "~ами";
        prepositional: "~и", "~ах";
        noun_second_case: "~и", "~и";
    ),
    // Топримечательность: 57 words (дочь, мать, свекровь, моль, макрель, модель).
    declension!("топримечательность", 57, "~ь";
        nominative: "~ь", "~и";
        genitive: "~и", "~ей";
        dative: "~и", "~ям";
        accusative: "~ь", referred;
        instrumental: "~ью", "~ями";
        prepositional: "~и", "~ях";
        noun_second_case: "~и", "~и";
    ),
    // Альня: 56 words (ожидальня, умывальня, гвоздильня, швальня, мыльня, мяльня).
    declension!("альня", 56, "~ь~я";
        nominative: "~ь~я", "~ь~и";
        genitive: "~ь~и", "~е~";
        dative: "~ь~е", "~ь~ям";
        accusative: "~ь~ю", referred;
        instrumental: "~ь~ей|~ь~ею", "~ь~ями";
        prepositional: "~ь~е", "~ь~ях";
        noun_second_case: "~ь~и", "~ь~е";
    ),
    // Ерская: 55 words (учительская, убогая, глухая, дворницкая, сухая, швейцарская).
    declension!("ерская", 55, "~ая";
        nominative: "~ая", "~ие";
        genitive: "~ой", "~их";
        dative: "~ой", "~им";
        accusative: "~ую", referred;
        instrumental: "~ой|~ою", "~ими";
        prepositional: "~ой", "~их";
        noun_second_case: "~ой", "~ой";
    ),
    // Ертральщик: 52 words (шер-гинеколог, од-подрядчик, лёт-заправщик, ер-тральщик, п-охотник, т-доставщик).
    declension!("ер-тральщик", 52, "~~";
        nominative: "~~", "~ы~и";
        genitive: "~а~а", "~ов~ов";
        dative: "~у~у", "~ам~ам";
        accusative: referred, referred;
        instrumental: "~ом~ом", "~ами~ами";
        prepositional: "~е~е", "~ах~ах";
        noun_second_case: "~а~а", "~е~е";
    ),
    // Лёд: 49 words (лёд, лён, князёк, зятёк, царёк, ячменёк).
    declension!("лёд", 49, "~ё~";
        nominative: "~ё~", "~ь~и";
        genitive: "~ь~а", "~ь~ов";
        dative: "~ь~у", "~ь~ам";
        accusative: referred, referred;
        instrumental: "~ь~ом", "~ь~ами";
        prepositional: "~ь~е", "~ь~ах";
        noun_second_case: "~ь~а", "~ь~е";
    ),
    // Вел: 42 words (цыган, погон, ампер, янычар, мокасин, ватт).
    declension!("вел", 42, "~";
        nominative: "~", "~ы";
        genitive: "~а", "~";
        dative: "~у", "~ам";
        accusative: referred, referred;
        instrumental: "~ом", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~а", "~е";
    ),
    // Ядро: 41 words (ядро, пятно, умишко, тябло, рядно, ятро).
    declension!("ядро", 41, "~~о";
        nominative: "~~о", "~~а";
        genitive: "~~а", "~е~";
        dative: "~~у", "~~ам";
        accusative: referred, referred;
        instrumental: "~~ом", "~~ами";
        prepositional: "~~е", "~~ах";
        noun_second_case: "~~а", "~~е";
    ),
    // Ата: 41 words (уста, белила, чернила, удила, перила, права).
    declension!("ата", 41, "~а";
        nominative: "~а", "~а";
        genitive: "~ы", "~";
        dative: "~е", "~ам";
        accusative: "~у", referred;
        instrumental: "~ой", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~ы", "~е";
    ),
    // Жое: 40 words (многое, членистоногое, уютненькое, знаменское, панское, гкое).
    declension!("жое", 40, "~ое";
        nominative: "~ое", "~ие";
        genitive: "~ого", "~их";
        dative: "~ому", "~им";
        accusative: referred, referred;
        instrumental: "~им", "~ими";
        prepositional: "~ом", "~их";
        noun_second_case: "~ого", "~ом";
    ),
    // Учонок: 37 words (мышонок, ежонок, сычонок, ужонок, ячонок, вежонок).
    declension!("учонок", 37, "~онок";
        nominative: "~онок", "~ата";
        genitive: "~онка", "~ат";
        dative: "~онку", "~атам";
        accusative: referred, referred;
        instrumental: "~онком", "~атами";
        prepositional: "~онке", "~атах";
        noun_second_case: "~онка", "~онке";
    ),
    // Езжая: 37 words (зрячая, хожая, ведущая, езжая, верующая, чая).
    declension!("езжая", 37, "~ая";
        nominative: "~ая", "~ие";
        genitive: "~ей", "~их";
        dative: "~ей", "~им";
        accusative: "~ую", referred;
        instrumental: "~ей|~ею", "~ими";
        prepositional: "~ей", "~их";
        noun_second_case: "~ей", "~ей";
    ),
    // Каракуля: 36 words (батя, дядя, доля, ходуля, пеня, распря).
    declension!("каракуля", 36, "~я";
        nominative: "~я", "~и";
        genitive: "~и", "~ей";
        dative: "~е", "~ям";
        accusative: "~ю", referred;
        instrumental: "~ей|~ею", "~ями";
        prepositional: "~е", "~ях";
        noun_second_case: "~и", "~е";
    ),
    // Жьё: 35 words (сырьё, вторсырьё, жульё, жраньё, рваньё, лганьё).
    declension!("жьё", 35, "~ьё";
        nominative: "~ьё", "~ья";
        genitive: "~ья", "~ей";
        dative: "~ью", "~ьям";
        accusative: referred, referred;
        instrumental: "~ьём", "~ьями";
        prepositional: "~ье", "~ьях";
        noun_second_case: "~ья", "~ье";
    ),
    // Еча: 34 words (паша, г-жа, моча, межа, ржа, ямча).
    declension!("еча", 34, "~а";
        nominative: "~а", "~и";
        genitive: "~и", "~ей";
        dative: "~е", "~ам";
        accusative: "~у", referred;
        instrumental: "~ой|~ою", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~и", "~е";
    ),
    // Ещи: 34 words (щи, мощи, гачи, ивацевичи, кириши, жжи).
    declension!("ещи", 34, "~и";
        nominative: "~и", "~и";
        genitive: "~и", "~ей";
        dative: "~и", "~ам";
        accusative: referred, referred;
        instrumental: "~и", "~ами";
        prepositional: "~и", "~ах";
        noun_second_case: "~и", "~и";
    ),
    // Жняк: 33 words (лак, мак, грог, ток, бак, срок).
    declension!("жняк", 33, "~";
        nominative: "~", "~и";
        genitive: "~а", "~ов";
        dative: "~у", "~ам";
        accusative: referred, referred;
        instrumental: "~ом", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~у", "~е";
    ),
    // Ючее: 29 words (означающее, тоящее, ущее, ючее, ядущее, чее).
    declension!("ючее", 29, "~ее";
        nominative: "~ее", "~ие";
        genitive: "~его", "~их";
        dative: "~ему", "~им";
        accusative: referred, referred;
        instrumental: "~им", "~ими";
        prepositional: "~ем", "~их";
        noun_second_case: "~его", "~ем";
    ),
    // Кохотник: 29 words (к-волк, овик-затейник, ик-ядерщик, к-охотник, уг-лущильник, ик-семицветик).
    declension!("к-охотник", 29, "~~";
        nominative: "~~", "~и~и";
        genitive: "~а~а", "~ов~ов";
        dative: "~у~у", "~ам~ам";
        accusative: referred, referred;
        instrumental: "~ом~ом", "~ами~ами";
        prepositional: "~е~е", "~ах~ах";
        noun_second_case: "~а~а", "~е~е";
    ),
    // Ейтапикколо: 29 words (рамма-дополнение, лопа-гну, кета-носитель, ейта-пикколо, ла-молот, дада-высотомер).
    declension!("ейта-пикколо", 29, "~а~";
        nominative: "~а~", "~ы~ы";
        genitive: "~ы~а", "~~ов";
        dative: "~е~у", "~ам~ам";
        accusative: "~у~", referred;
        instrumental: "~ой~ом|~ою~ом", "~ами~ами";
        prepositional: "~е~е", "~ах~ах";
        noun_second_case: "~ы~а", "~е~е";
    ),
    // Шью: 28 words (гэта, мюсли, татпуруша, сю-сю, ося, шью).
    declension!("шью", 28, "~";
        nominative: "~", "~";
        genitive: "~", "~";
        dative: "~", "~";
        accusative: "~", "~";
        instrumental: "~", "~";
        prepositional: "~", "~";
        noun_second_case: "~", "~";
    ),
    // Литра: 26 words (-очка, -литра, -оборота, -августа, -автобуса, -аршина).
    declension!("-литра", 26, "-~а";
        nominative: "-~а", "у~ы";
        genitive: "у~а", "у~ов";
        dative: "у~у", "у~ам";
        accusative: referred, referred;
        instrumental: "у~ом", "у~ами";
        prepositional: "у~е", "у~ах";
        noun_second_case: "у~а", "у~е";
    ),
    // Щит: 25 words (мир, кон, сад, под, стан, зоб).
    declension!("щит", 25, "~";
        nominative: "~", "~ы";
        genitive: "~а", "~ов";
        dative: "~у", "~ам";
        accusative: referred, referred;
        instrumental: "~ом", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~а", "~у";
    ),
    // Лище: 25 words (дружище, жучище, лгунище, лбище, ртище, дядище).
    declension!("лище", 25, "~е";
        nominative: "~е", "~и";
        genitive: "~а", "~";
        dative: "~у", "~ам";
        accusative: referred, referred;
        instrumental: "~ем", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~а", "~е";
    ),
    // Руя: 24 words (змея, колея, швея, алия, жалея, верея).
    declension!("руя", 24, "~я";
        nominative: "~я", "~и";
        genitive: "~и", "~й";
        dative: "~е", "~ям";
        accusative: "~ю", referred;
        instrumental: "~ёй|~ёю", "~ями";
        prepositional: "~е", "~ях";
        noun_second_case: "~и", "~е";
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
    // Ашние: 23 words (млекопитающие, многие, лькие, хоногие, изкие, евние).
    declension!("ашние", 23, "~ие";
        nominative: "~ее", "~ие";
        genitive: "~его", "~их";
        dative: "~ему", "~им";
        accusative: "~ее", referred;
        instrumental: "~им", "~ими";
        prepositional: "~ем", "~их";
        noun_second_case: "~его", "~ем";
    ),
    // Кптицеед: 23 words (к-птицеед, жник-мультипликатор, к-скакун, к-муравьед, утник-ретранслятор, фик-дизайнер).
    declension!("к-птицеед", 23, "~~";
        nominative: "~~", "~и~ы";
        genitive: "~а~а", "~ов~ов";
        dative: "~у~у", "~ам~ам";
        accusative: referred, referred;
        instrumental: "~ом~ом", "~ами~ами";
        prepositional: "~е~е", "~ах~ах";
        noun_second_case: "~а~а", "~е~е";
    ),
    // Язги: 22 words (лыжи, мытищи, сцаки, ссаки, юки, доши).
    declension!("язги", 22, "~и";
        nominative: "~и", "~и";
        genitive: "~и", "~";
        dative: "~и", "~ам";
        accusative: referred, referred;
        instrumental: "~и", "~ами";
        prepositional: "~и", "~ах";
        noun_second_case: "~и", "~и";
    ),
    // Центировка: 20 words (отклейка, самосброска, вшивка, лычка, мга, стирашка).
    declension!("центировка", 20, "~а";
        nominative: "~а", "~и";
        genitive: "~и", "~а";
        dative: "~е", "~ам";
        accusative: "~у", referred;
        instrumental: "~ой|~ою", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~и", "~е";
    ),
    // Елхранитель: 20 words (ел-хранитель, мат-наполнитель, н-смеситель, уд-охладитель, актор-размножитель, вол-заполнитель).
    declension!("ел-хранитель", 20, "~~ь";
        nominative: "~~ь", "~ы~и";
        genitive: "~а~я", "~ов~ей";
        dative: "~у~ю", "~ам~ям";
        accusative: referred, referred;
        instrumental: "~ом~ем", "~ами~ями";
        prepositional: "~е~е", "~ах~ях";
        noun_second_case: "~а~я", "~е~е";
    ),
    // Егерь: 18 words (учитель, лагерь, соболь, егерь, штабель, игель).
    declension!("егерь", 18, "~ь";
        nominative: "~ь", "~я";
        genitive: "~я", "~ей";
        dative: "~ю", "~ям";
        accusative: referred, referred;
        instrumental: "~ем", "~ями";
        prepositional: "~е", "~ях";
        noun_second_case: "~я", "~е";
    ),
    // Ребро: 18 words (стекло, стегно, ребро, весло, зерно, бедро).
    declension!("ребро", 18, "~е~~о";
        nominative: "~е~~о", "~ё~~а";
        genitive: "~е~~а", "~ё~е~";
        dative: "~е~~у", "~ё~~ам";
        accusative: referred, referred;
        instrumental: "~е~~ом", "~ё~~ами";
        prepositional: "~е~~е", "~ё~~ах";
        noun_second_case: "~е~~а", "~е~~е";
    ),
    // Шов: 17 words (свёкор, сон, бугор, посол, боз, ров).
    declension!("шов", 17, "~о~";
        nominative: "~о~", "~~ы";
        genitive: "~~а", "~~ов";
        dative: "~~у", "~~ам";
        accusative: referred, referred;
        instrumental: "~~ом", "~~ами";
        prepositional: "~~е", "~~ах";
        noun_second_case: "~~а", "~~е";
    ),
    // Нцы: 17 words (японцы, овалогубцы, сходцы, губцы, зубцы, ёпанцы).
    declension!("нцы", 17, "~ы";
        nominative: "~ы", "~ы";
        genitive: "~ы", "~ев";
        dative: "~ы", "~ам";
        accusative: referred, referred;
        instrumental: "~ы", "~ами";
        prepositional: "~ы", "~ах";
        noun_second_case: "~ы", "~ы";
    ),
    // Боёк: 17 words (заём, наём, паёк, чаёк, боёк, буёк).
    declension!("боёк", 17, "~ё~";
        nominative: "~ё~", "~й~и";
        genitive: "~й~а", "~й~ов";
        dative: "~й~у", "~й~ам";
        accusative: referred, referred;
        instrumental: "~й~ом", "~й~ами";
        prepositional: "~й~е", "~й~ах";
        noun_second_case: "~й~а", "~й~е";
    ),
    // Льё: 17 words (мытьё, нытьё, шмотьё, гнутьё, знатьё, мятьё).
    declension!("льё", 17, "~ьё";
        nominative: "~ьё", "~ья";
        genitive: "~ья", "~ий";
        dative: "~ью", "~ьям";
        accusative: referred, referred;
        instrumental: "~ьём", "~ьями";
        prepositional: "~ье", "~ьях";
        noun_second_case: "~ья", "~ье";
    ),
    // Бедьшипун: 17 words (мобиль-битумовоз, мобиль-кран, мобиль-мусоровоз, мобиль-фургон, мобиль-цементовоз, меритель-сигнализатор).
    declension!("бедь-шипун", 17, "~ь~";
        nominative: "~ь~", "~и~ы";
        genitive: "~я~а", "~ей~ов";
        dative: "~ю~у", "~ям~ам";
        accusative: referred, referred;
        instrumental: "~ем~ом", "~ями~ами";
        prepositional: "~е~е", "~ях~ах";
        noun_second_case: "~я~а", "~е~е";
    ),
    // Нос: 16 words (час, чад, пол, дуб, нос, сыр).
    declension!("нос", 16, "~";
        nominative: "~", "~ы";
        genitive: "~а", "~ов";
        dative: "~у", "~ам";
        accusative: referred, referred;
        instrumental: "~ом", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~у", "~у";
    ),
    // Звезда: 16 words (звезда, пчела, жена, сестра, верста, кинозвезда).
    declension!("звезда", 16, "~е~а";
        nominative: "~е~а", "~ё~ы";
        genitive: "~е~ы", "~ё~";
        dative: "~е~е", "~ё~ам";
        accusative: "~е~у", referred;
        instrumental: "~е~ой|~е~ою", "~ё~ами";
        prepositional: "~е~е", "~ё~ах";
        noun_second_case: "~е~ы", "~е~е";
    ),
    // Цапля: 16 words (цапля, ебля, путля, говля, бля, пля).
    declension!("цапля", 16, "~~я";
        nominative: "~~я", "~~и";
        genitive: "~~и", "~е~ь";
        dative: "~~е", "~~ям";
        accusative: "~~ю", referred;
        instrumental: "~~ей|~~ею", "~~ями";
        prepositional: "~~е", "~~ях";
        noun_second_case: "~~и", "~~е";
    ),
    // Акуйя: 16 words (майя, райя, гуйя, хойя, олея, илуйя).
    declension!("акуйя", 16, "~я";
        nominative: "~я", "~и";
        genitive: "~и", "~";
        dative: "~е", "~ям";
        accusative: "~ю", referred;
        instrumental: "~ей|~ею", "~ями";
        prepositional: "~е", "~ях";
        noun_second_case: "~и", "~е";
    ),
    // Копаемое: 16 words (екомое, тное, мноводное, окопытное, кообразное, звоночное).
    declension!("копаемое", 16, "~ое";
        nominative: "~ое", "~ые";
        genitive: "~ого", "~ых";
        dative: "~ому", "~ым";
        accusative: "~ое", referred;
        instrumental: "~ым", "~ыми";
        prepositional: "~ом", "~ых";
        noun_second_case: "~ого", "~ом";
    ),
    // Пкаушанка: 16 words (пка-ушанка, чка-теньковка, пка-невидимка, авка-портниха, тека-передвижка, тавка-продажа).
    declension!("пка-ушанка", 16, "~~а~~а";
        nominative: "~~а~~а", "~~и~~и";
        genitive: "~~и~~и", "~о~~о~";
        dative: "~~е~~е", "~~ам~~ам";
        accusative: "~~у~~у", referred;
        instrumental: "~~ой~~ой|~~ою~~ою", "~~ами~~ами";
        prepositional: "~~е~~е", "~~ах~~ах";
        noun_second_case: "~~и~~и", "~~е~~е";
    ),
    // Лицо: 15 words (лицо, чадо, чмо, мазло, высокопревосходительство, ссыкло).
    declension!("лицо", 15, "~о";
        nominative: "~о", "~а";
        genitive: "~а", "~";
        dative: "~у", "~ам";
        accusative: "~о", referred;
        instrumental: "~ом", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~а", "~е";
    ),
    // Кота: 15 words (кета, псяра, валера, зяма, мса, кота).
    declension!("кота", 15, "~а";
        nominative: "~а", "~ы";
        genitive: "~ы", "~";
        dative: "~е", "~ам";
        accusative: "~у", "~ы";
        instrumental: "~ой|~ою", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~ы", "~е";
    ),
    // Лёж: 15 words (бздёж, здёж, тёж, лёж, платёж, ндёж).
    declension!("лёж", 15, "~ё~";
        nominative: "~ё~", "~е~и";
        genitive: "~е~а", "~е~ей";
        dative: "~е~у", "~е~ам";
        accusative: referred, referred;
        instrumental: "~е~ом", "~е~ами";
        prepositional: "~е~е", "~е~ах";
        noun_second_case: "~е~а", "~е~е";
    ),
    // Каёжка: 15 words (ка-летяга, ла-рыба, ла-лопата, енна-мачта, ка-стенка, дина-говядина).
    declension!("ка-ёжка", 15, "~а~а";
        nominative: "~а~а", "~ы~ы";
        genitive: "~ы~ы", "~~";
        dative: "~е~е", "~ам~ам";
        accusative: "~у~у", referred;
        instrumental: "~ой~ой|~ою~ою", "~ами~ами";
        prepositional: "~е~е", "~ах~ах";
        noun_second_case: "~ы~ы", "~е~е";
    ),
    // Клин: 14 words (брат, лист, муж, клин, стул, кол).
    declension!("клин", 14, "~";
        nominative: "~", "~ья";
        genitive: "~а", "~ьев";
        dative: "~у", "~ьям";
        accusative: referred, referred;
        instrumental: "~ом", "~ьями";
        prepositional: "~е", "~ьях";
        noun_second_case: "~а", "~е";
    ),
    // Ребье: 14 words (отрепье, тье, мастерье, полье, ребье, еколье).
    declension!("ребье", 14, "~е";
        nominative: "~е", "~я";
        genitive: "~я", "~ев";
        dative: "~ю", "~ям";
        accusative: referred, referred;
        instrumental: "~ем", "~ями";
        prepositional: "~е", "~ях";
        noun_second_case: "~я", "~е";
    ),
    // Зёл: 13 words (пёс, орёл, овёс, осёл, одёр, сябёр).
    declension!("зёл", 13, "~ё~";
        nominative: "~ё~", "~~ы";
        genitive: "~~а", "~~ов";
        dative: "~~у", "~~ам";
        accusative: referred, referred;
        instrumental: "~~ом", "~~ами";
        prepositional: "~~е", "~~ах";
        noun_second_case: "~~а", "~~е";
    ),
    // Бои: 13 words (обои, бои, мои, лиминарии, кои, бореи).
    declension!("бои", 13, "~и";
        nominative: "~и", "~и";
        genitive: "~и", "~ев";
        dative: "~и", "~ям";
        accusative: referred, referred;
        instrumental: "~и", "~ями";
        prepositional: "~и", "~ях";
        noun_second_case: "~и", "~и";
    ),
    // Дья: 13 words (яхья, мья, дья, амья, нья, уфья).
    declension!("дья", 13, "~ья";
        nominative: "~ья", "~ьи";
        genitive: "~ьи", "~ей";
        dative: "~ье", "~ьям";
        accusative: "~ью", referred;
        instrumental: "~ьёй|~ьёю", "~ьями";
        prepositional: "~ье", "~ьях";
        noun_second_case: "~ьи", "~ье";
    ),
    // Тскоба: 13 words (т-скоба, гон-цистерна, мент-камера, н-стрела, н-укосина, цеп-цистерна).
    declension!("т-скоба", 13, "~~а";
        nominative: "~~а", "~ы~ы";
        genitive: "~а~ы", "~ов~";
        dative: "~у~е", "~ам~ам";
        accusative: "~~у", referred;
        instrumental: "~ом~ой|~ом~ою", "~ами~ами";
        prepositional: "~е~е", "~ах~ах";
        noun_second_case: "~а~ы", "~е~е";
    ),
    // Инямать: 13 words (ания-поставщик, ания-производитель, анизация-заказчик, анизация-разработчик, иня-мать, рея-салон).
    declension!("иня-мать", 13, "~я~";
        nominative: "~я~", "~и~и";
        genitive: "~и~а", "~й~ов";
        dative: "~и~у", "~ям~ам";
        accusative: "~ю~", referred;
        instrumental: "~ей~ом|~ею~ом", "~ями~ами";
        prepositional: "~и~е", "~ях~ах";
        noun_second_case: "~и~а", "~и~е";
    ),
    // Тизвещение: 13 words (т-извещение, прос-утверждение, он-уведомление, гон-общежитие, сказ-воспоминание, ман-воспоминание).
    declension!("т-извещение", 13, "~~е";
        nominative: "~~е", "~ы~я";
        genitive: "~а~я", "~ов~й";
        dative: "~у~ю", "~ам~ям";
        accusative: referred, referred;
        instrumental: "~ом~ем", "~ами~ями";
        prepositional: "~е~и", "~ах~ях";
        noun_second_case: "~а~я", "~е~и";
    ),
    // Имя: 12 words (имя, знамя, вымя, темя, семя, рамя).
    declension!("имя", 12, "~я";
        nominative: "~я", "~ена";
        genitive: "~ени", "~ён";
        dative: "~ени", "~енам";
        accusative: referred, referred;
        instrumental: "~енем", "~енами";
        prepositional: "~ени", "~енах";
        noun_second_case: "~ени", "~ени";
    ),
    // Очко: 12 words (очко, озерко, сико, евко, чико, вко).
    declension!("очко", 12, "~о";
        nominative: "~о", "~и";
        genitive: "~а", "~ов";
        dative: "~у", "~ам";
        accusative: referred, referred;
        instrumental: "~ом", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~а", "~е";
    ),
    // Уёбище: 12 words (уёбище, идолище, овище, мешище, шилище, боёбище).
    declension!("уёбище", 12, "~е";
        nominative: "~е", "~а";
        genitive: "~а", "~";
        dative: "~у", "~ам";
        accusative: "~е", referred;
        instrumental: "~ем", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~а", "~е";
    ),
    // Требительбомбардировщик: 12 words (требитель-бомбардировщик, итель-механик, сарь-водопроводчик, сарь-механик, сарь-сантехник, сарь-сборщик).
    declension!("требитель-бомбардировщик", 12, "~ь~";
        nominative: "~ь~", "~и~и";
        genitive: "~я~а", "~ей~ов";
        dative: "~ю~у", "~ям~ам";
        accusative: referred, referred;
        instrumental: "~ем~ом", "~ями~ами";
        prepositional: "~е~е", "~ях~ах";
        noun_second_case: "~я~а", "~е~е";
    ),
    // Метра: 12 words (часа, века, метра, миллиона, слова, километра).
    declension!("метра", 12, "~а";
        nominative: "~а", "у~ы";
        genitive: "у~а", "у~ов";
        dative: "у~у", "у~ам";
        accusative: referred, referred;
        instrumental: "у~ом", "у~ами";
        prepositional: "у~е", "у~ах";
        noun_second_case: "у~а", "у~е";
    ),
    // Алоцман: 12 words (а-удильщик, а-ангел, а-кит, а-лоцман, а-странник, а-клоун).
    declension!("а-лоцман", 12, "а~";
        nominative: "а~", "ы~ы";
        genitive: "ы~а", "~ов";
        dative: "е~у", "ам~ам";
        accusative: "у~а", referred;
        instrumental: "ой~ом|ою~ом", "ами~ами";
        prepositional: "е~е", "ах~ах";
        noun_second_case: "ы~а", "е~е";
    ),
    // Лев: 11 words (лев, гнилец, котелец, елец, жилец, гролев).
    declension!("лев", 11, "~е~";
        nominative: "~е~", "~ь~ы";
        genitive: "~ь~а", "~ь~ов";
        dative: "~ь~у", "~ь~ам";
        accusative: referred, referred;
        instrumental: "~ь~ом", "~ь~ами";
        prepositional: "~ь~е", "~ь~ах";
        noun_second_case: "~ь~а", "~ь~е";
    ),
    // Яток: 11 words (песок, мох, бачок, сырок, ледок, жирок).
    declension!("яток", 11, "~о~";
        nominative: "~о~", "~~и";
        genitive: "~~а", "~~ов";
        dative: "~~у", "~~ам";
        accusative: referred, referred;
        instrumental: "~~ом", "~~ами";
        prepositional: "~~е", "~~ах";
        noun_second_case: "~~у", "~~е";
    ),
    // Тёночек: 11 words (цыплёночек, опёночек, телёночек, цыганёночек, тёночек, лёночек).
    declension!("тёночек", 11, "~ёноче~";
        nominative: "~ёноче~", "~ят~и";
        genitive: "~ёноч~а", "~ято~";
        dative: "~ёноч~у", "~ят~ам";
        accusative: referred, referred;
        instrumental: "~ёноч~ом", "~ят~ами";
        prepositional: "~ёноч~е", "~ят~ах";
        noun_second_case: "~ёноч~а", "~ёноч~е";
    ),
    // Урма: 11 words (полумгла, мста, ангара, анкара, шабала, шантропа).
    declension!("урма", 11, "~а";
        nominative: "~а", "~ы";
        genitive: "~ы", "~а";
        dative: "~е", "~ам";
        accusative: "~у", referred;
        instrumental: "~ой|~ою", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~ы", "~е";
    ),
    // Ельки: 11 words (ахоньки, оченьки, еньки, ельки, хоньки, ульки).
    declension!("ельки", 11, "~ь~и";
        nominative: "~ь~а", "~ь~и";
        genitive: "~ь~и", "~е~";
        dative: "~ь~е", "~ь~ам";
        accusative: "~ь~у", referred;
        instrumental: "~ь~ой", "~ь~ами";
        prepositional: "~ь~е", "~ь~ах";
        noun_second_case: "~ь~и", "~ь~е";
    ),
    // Дце: 11 words (дце, евце, тенце, юдце, нце, оконце).
    declension!("дце", 11, "~~е";
        nominative: "~~е", "~~а";
        genitive: "~~а", "~е~";
        dative: "~~у", "~~ам";
        accusative: referred, referred;
        instrumental: "~~ем", "~~ами";
        prepositional: "~~е", "~~ах";
        noun_second_case: "~~а", "~~е";
    ),
    // Аникводитель: 11 words (отник-собиратель, аник-водитель, ник-смотритель, лог-любитель, ёмник-распределитель, утник-спасатель).
    declension!("аник-водитель", 11, "~~ь";
        nominative: "~~ь", "~и~и";
        genitive: "~а~я", "~ов~ей";
        dative: "~у~ю", "~ам~ям";
        accusative: referred, referred;
        instrumental: "~ом~ем", "~ами~ями";
        prepositional: "~е~е", "~ах~ях";
        noun_second_case: "~а~я", "~е~е";
    ),
    // Корреспондент: 11 words (-корреспондент, -кредитор, -эмитент, -должник, -агент, -андеррайтер).
    declension!("-корреспондент", 11, "~";
        nominative: "~", "и~ы";
        genitive: "а~а", "ов~ов";
        dative: "у~у", "ам~ам";
        accusative: referred, referred;
        instrumental: "ом~ом", "ами~ами";
        prepositional: "е~е", "ах~ах";
        noun_second_case: "а~а", "е~е";
    ),
    // Пень: 10 words (день, ремень, пень, кремень, плетень, журавель).
    declension!("пень", 10, "~е~ь";
        nominative: "~е~ь", "~~и";
        genitive: "~~я", "~~ей";
        dative: "~~ю", "~~ям";
        accusative: referred, referred;
        instrumental: "~~ём", "~~ями";
        prepositional: "~~е", "~~ях";
        noun_second_case: "~~я", "~~е";
    ),
    // Ловия: 10 words (усилия, ловия, живания, поминания, отношения, мочия).
    declension!("ловия", 10, "~я";
        nominative: "~я", "~я";
        genitive: "~и", "~й";
        dative: "~и", "~ям";
        accusative: "~ю", referred;
        instrumental: "~ей", "~ями";
        prepositional: "~и", "~ях";
        noun_second_case: "~и", "~и";
    ),
    // Калии: 10 words (икалии, калии, линии, италии, урналии, алии).
    declension!("калии", 10, "~и";
        nominative: "~и", "~и";
        genitive: "~и", "~й";
        dative: "~и", "~ям";
        accusative: referred, referred;
        instrumental: "~и", "~ями";
        prepositional: "~и", "~ях";
        noun_second_case: "~и", "~и";
    ),
    // Бро: 10 words (миро, ебро, дро, бро, тро, бло).
    declension!("бро", 10, "~о";
        nominative: "~о", "~о";
        genitive: "~а", "~о";
        dative: "~у", "~о";
        accusative: referred, referred;
        instrumental: "~ом", "~о";
        prepositional: "~е", "~о";
        noun_second_case: "~а", "~е";
    ),
    // Едья: 10 words (ошмётья, ошмотья, опья, хмотья, едья, усья).
    declension!("едья", 10, "~я";
        nominative: "~я", "~я";
        genitive: "~я", "~ев";
        dative: "~я", "~ям";
        accusative: referred, referred;
        instrumental: "~я", "~ями";
        prepositional: "~я", "~ях";
        noun_second_case: "~я", "~я";
    ),
    // Кварека: 10 words (убица-пушка, ба-повитуха, па-вспышка, стница-стремянка, рамма-оболочка, ма-бабочка).
    declension!("ква-река", 10, "~а~~а";
        nominative: "~а~~а", "~ы~~и";
        genitive: "~ы~~и", "~~е~";
        dative: "~е~~е", "~ам~~ам";
        accusative: "~у~~у", referred;
        instrumental: "~ой~~ой", "~ами~~ами";
        prepositional: "~е~~е", "~ах~~ах";
        noun_second_case: "~ы~~и", "~е~~е";
    ),
    // Тьежакет: 10 words (тье-костюм, тение-индикатор, тение-хозяин, тье-баллон, тье-жакет, тье-жилет).
    declension!("тье-жакет", 10, "~е~";
        nominative: "~е~", "~я~ы";
        genitive: "~я~а", "~ев~ов";
        dative: "~ю~у", "~ям~ам";
        accusative: referred, referred;
        instrumental: "~ем~ом", "~ями~ами";
        prepositional: "~е~е", "~ях~ах";
        noun_second_case: "~я~а", "~е~е";
    ),
    // Гнездо: 9 words (озеро, село, чело, гнездо, сверло, шето).
    declension!("гнездо", 9, "~е~о";
        nominative: "~е~о", "~ё~а";
        genitive: "~е~а", "~ё~";
        dative: "~е~у", "~ё~ам";
        accusative: referred, referred;
        instrumental: "~е~ом", "~ё~ами";
        prepositional: "~е~е", "~ё~ах";
        noun_second_case: "~е~а", "~е~е";
    ),
    // Хуй: 9 words (хуй, рой, слой, буй, гай, пай).
    declension!("хуй", 9, "~й";
        nominative: "~й", "~и";
        genitive: "~я", "~ёв";
        dative: "~ю", "~ям";
        accusative: referred, referred;
        instrumental: "~ем", "~ями";
        prepositional: "~е", "~ях";
        noun_second_case: "~я", "~е";
    ),
    // Льце: 9 words (щупальце, мыльце, жужжальце, ркальце, льце, вильце).
    declension!("льце", 9, "~ь~е";
        nominative: "~ь~е", "~ь~а";
        genitive: "~ь~а", "~е~";
        dative: "~ь~у", "~ь~ам";
        accusative: referred, referred;
        instrumental: "~ь~ем", "~ь~ами";
        prepositional: "~ь~е", "~ь~ах";
        noun_second_case: "~ь~а", "~ь~е";
    ),
    // Голсвязка: 9 words (гол-связка, ибр-пробка, пан-заслонка, ос-форсунка, фон-трубка, аф-колонка).
    declension!("гол-связка", 9, "~~~а";
        nominative: "~~~а", "~ы~~и";
        genitive: "~а~~и", "~ов~о~";
        dative: "~у~~е", "~ам~~ам";
        accusative: "~~~у", referred;
        instrumental: "~ом~~ой|~ом~~ою", "~ами~~ами";
        prepositional: "~е~~е", "~ах~~ах";
        noun_second_case: "~а~~и", "~е~~е";
    ),
    // Инос: 9 words (хайр, акер, олончелист, трат, дакт, стоед).
    declension!("инос", 9, "~";
        nominative: "~", "~ы";
        genitive: "~а", "~ов";
        dative: "~у", "~ам";
        accusative: "~", "~ы";
        instrumental: "~ом", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~а", "~е";
    ),
    // Атрстудия: 9 words (т-рекламация, жир-амфибия, атр-студия, ман-биография, ман-утопия, ман-фантазия).
    declension!("атр-студия", 9, "~~я";
        nominative: "~~я", "~ы~и";
        genitive: "~а~и", "~ов~й";
        dative: "~у~и", "~ам~ям";
        accusative: "~~ю", referred;
        instrumental: "~ом~ей|~ом~ею", "~ами~ями";
        prepositional: "~е~и", "~ах~ях";
        noun_second_case: "~а~и", "~е~и";
    ),
    // Дом: 8 words (дом, лес, век, снег, мех, стог).
    declension!("дом", 8, "~";
        nominative: "~", "~а";
        genitive: "~а", "~ов";
        dative: "~у", "~ам";
        accusative: referred, referred;
        instrumental: "~ом", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~у", "~у";
    ),
    // Зубок: 8 words (ботинок, чулок, турок, зубок, катанок, тюрок).
    declension!("зубок", 8, "~о~";
        nominative: "~о~", "~~и";
        genitive: "~~а", "~о~";
        dative: "~~у", "~~ам";
        accusative: referred, referred;
        instrumental: "~~ом", "~~ами";
        prepositional: "~~е", "~~ах";
        noun_second_case: "~~а", "~~е";
    ),
    // Перо: 8 words (перо, шило, сило, ыло, звено, ено).
    declension!("перо", 8, "~о";
        nominative: "~о", "~ья";
        genitive: "~а", "~ьев";
        dative: "~у", "~ьям";
        accusative: referred, referred;
        instrumental: "~ом", "~ьями";
        prepositional: "~е", "~ьях";
        noun_second_case: "~а", "~е";
    ),
    // Вер: 8 words (глаз, цвет, окорок, рост, вес, вер).
    declension!("вер", 8, "~";
        nominative: "~", "~а";
        genitive: "~а", "~ов";
        dative: "~у", "~ам";
        accusative: referred, referred;
        instrumental: "~ом", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~у", "~е";
    ),
    // Бойня: 8 words (двойня, грейня, кобойня, бойня, хладобойня, мойня).
    declension!("бойня", 8, "~й~я";
        nominative: "~й~я", "~й~и";
        genitive: "~й~и", "~е~";
        dative: "~й~е", "~й~ям";
        accusative: "~й~ю", referred;
        instrumental: "~й~ей|~й~ею", "~й~ями";
        prepositional: "~й~е", "~й~ях";
        noun_second_case: "~й~и", "~й~е";
    ),
    // Шеек: 8 words (закраек, отроек, ошеек, окраек, подхуек, шеек).
    declension!("шеек", 8, "~е~";
        nominative: "~е~", "~й~и";
        genitive: "~й~а", "~й~ов";
        dative: "~й~у", "~й~ам";
        accusative: referred, referred;
        instrumental: "~й~ом", "~й~ами";
        prepositional: "~й~е", "~й~ах";
        noun_second_case: "~й~а", "~й~е";
    ),
    // Нкамикадзе: 8 words (н-камикадзе, тов-на-дону, аф-купе, анкфурт-на-майне, уб-казино, лёт-такси).
    declension!("н-камикадзе", 8, "~~";
        nominative: "~~", "~ы~";
        genitive: "~а~", "~ов~";
        dative: "~у~", "~ам~";
        accusative: referred, referred;
        instrumental: "~ом~", "~ами~";
        prepositional: "~е~", "~ах~";
        noun_second_case: "~а~", "~е~";
    ),
    // Дюжины: 8 words (минуты, дюжины, кроны, страны, школы, страницы).
    declension!("дюжины", 8, "~ы";
        nominative: "~ы", "у~ы";
        genitive: "у~ы", "у~";
        dative: "у~е", "у~ам";
        accusative: referred, referred;
        instrumental: "у~ой", "у~ами";
        prepositional: "у~е", "у~ах";
        noun_second_case: "у~ы", "у~е";
    ),
    // Впризрак: 8 words (д-призрак, д-заповедник, д-памятник, д-спутник, в-призрак, д-миллионник).
    declension!("в-призрак", 8, "~~";
        nominative: "~~", "~а~и";
        genitive: "~а~а", "~ов~ов";
        dative: "~у~у", "~ам~ам";
        accusative: referred, referred;
        instrumental: "~ом~ом", "~ами~ами";
        prepositional: "~е~е", "~ах~ах";
        noun_second_case: "~а~а", "~е~е";
    ),
    // Юбки: 8 words (-ёмкости, -линии, -ложки, -области, -юбки, -осьмухи).
    declension!("-юбки", 8, "-~и";
        nominative: "-~и", "у~и";
        genitive: "у~и", "у~ей";
        dative: "у~и", "у~ям";
        accusative: referred, referred;
        instrumental: "у~ью", "у~ями";
        prepositional: "у~и", "у~ях";
        noun_second_case: "у~и", "у~и";
    ),
    // Ечо: 7 words (яблоко, млеко, веко, брюхо, лыко, ечо).
    declension!("ечо", 7, "~о";
        nominative: "~о", "~и";
        genitive: "~а", "~";
        dative: "~у", "~ам";
        accusative: referred, referred;
        instrumental: "~ом", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~а", "~е";
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
    // Кла: 7 words (мгла, княжна, чухна, секс-кукла, кла, шна).
    declension!("кла", 7, "~~а";
        nominative: "~~а", "~~ы";
        genitive: "~~ы", "~о~";
        dative: "~~е", "~~ам";
        accusative: "~~у", referred;
        instrumental: "~~ой|~~ою", "~~ами";
        prepositional: "~~е", "~~ах";
        noun_second_case: "~~ы", "~~е";
    ),
    // Кой: 7 words (гной, иней, двобой, кой, фей, дерей).
    declension!("кой", 7, "~й";
        nominative: "~й", "~и";
        genitive: "~я", "~ев";
        dative: "~ю", "~ям";
        accusative: referred, referred;
        instrumental: "~ем", "~ями";
        prepositional: "~е", "~ях";
        noun_second_case: "~ю", "~е";
    ),
    // Жой: 7 words (глухой, жой, тромской, акой, шой, ьшой).
    declension!("жой", 7, "~ой";
        nominative: "~ой", "~ие";
        genitive: "~ого", "~их";
        dative: "~ому", "~им";
        accusative: referred, referred;
        instrumental: "~им", "~ими";
        prepositional: "~ом", "~их";
        noun_second_case: "~ого", "~ом";
    ),
    // Счёт: 7 words (мёд, счёт, шёлк, жёрнов, жёлоб, т-счёт).
    declension!("счёт", 7, "~ё~";
        nominative: "~ё~", "~е~а";
        genitive: "~ё~а", "~е~ов";
        dative: "~ё~у", "~е~ам";
        accusative: referred, referred;
        instrumental: "~ё~ом", "~е~ами";
        prepositional: "~ё~е", "~е~ах";
        noun_second_case: "~ё~а", "~ё~е";
    ),
    // Дняя: 7 words (дняя, исподняя, ижняя, ужняя, мужняя, овершеннолетняя).
    declension!("дняя", 7, "~яя";
        nominative: "~яя", "~ие";
        genitive: "~ей", "~их";
        dative: "~ей", "~им";
        accusative: "~юю", referred;
        instrumental: "~ей|~ею", "~ими";
        prepositional: "~ей", "~их";
        noun_second_case: "~ей", "~ей";
    ),
    // Сьмо: 7 words (сьмо, ыльцо, льцо, кольцо, хьмо, ольцо).
    declension!("сьмо", 7, "~ь~о";
        nominative: "~ь~о", "~ь~а";
        genitive: "~ь~а", "~е~";
        dative: "~ь~у", "~ь~ам";
        accusative: referred, referred;
        instrumental: "~ь~ом", "~ь~ами";
        prepositional: "~ь~е", "~ь~ах";
        noun_second_case: "~ь~а", "~ь~е";
    ),
    // Тце: 7 words (тце, ьце, дёрце, ытце, альце, отце).
    declension!("тце", 7, "~~е";
        nominative: "~~е", "~~а";
        genitive: "~~а", "~~ев|~е~";
        dative: "~~у", "~~ам";
        accusative: referred, referred;
        instrumental: "~~ем", "~~ами";
        prepositional: "~~е", "~~ах";
        noun_second_case: "~~а", "~~е";
    ),
    // Тьемайка: 7 words (тье-колонна, тье-труба, тье-сорочка, тье-майка, тье-пачка, тье-футболка).
    declension!("тье-майка", 7, "~е~~а";
        nominative: "~е~~а", "~я~~и";
        genitive: "~я~~и", "~ев~е~";
        dative: "~ю~~е", "~ям~~ам";
        accusative: "~е~~у", referred;
        instrumental: "~ем~~ой|~ем~~ою", "~ями~~ами";
        prepositional: "~е~~е", "~ях~~ах";
        noun_second_case: "~я~~и", "~е~~е";
    ),
    // Июля: 7 words (-апреля, -имения, -июля, -июня, -лагеря, -октября).
    declension!("-июля", 7, "-~я";
        nominative: "-~я", "у~и";
        genitive: "у~я", "у~ей";
        dative: "у~ю", "у~ям";
        accusative: referred, referred;
        instrumental: "у~ем", "у~ями";
        prepositional: "у~е", "у~ях";
        noun_second_case: "у~я", "у~е";
    ),
    // Игры: 7 words (-игры, -избы, -луковицы, -юрты, -ягоды, -луны).
    declension!("-игры", 7, "-~ы";
        nominative: "-~ы", "у~ы";
        genitive: "у~ы", "у~";
        dative: "у~е", "у~ам";
        accusative: referred, referred;
        instrumental: "у~ой", "у~ами";
        prepositional: "у~е", "у~ах";
        noun_second_case: "у~ы", "у~е";
    ),
    // Мкапакет: 7 words (тка-киллер, тка-предшественник, йка-хохотун, вка-плакат, мка-пакет, равка-вызов).
    declension!("мка-пакет", 7, "~~а~";
        nominative: "~~а~", "~~и~ы";
        genitive: "~~и~а", "~о~~ов";
        dative: "~~е~у", "~~ам~ам";
        accusative: "~~у~", referred;
        instrumental: "~~ой~ом|~~ою~ом", "~~ами~ами";
        prepositional: "~~е~е", "~~ах~ах";
        noun_second_case: "~~и~а", "~~е~е";
    ),
    // Зло: 6 words (окно, зло, тягло, кно, лко, волокно).
    declension!("зло", 6, "~~о";
        nominative: "~~о", "~~а";
        genitive: "~~а", "~о~";
        dative: "~~у", "~~ам";
        accusative: referred, referred;
        instrumental: "~~ом", "~~ами";
        prepositional: "~~е", "~~ах";
        noun_second_case: "~~а", "~~е";
    ),
    // Жнивьё: 6 words (жнивьё, дитё, риё, триё, ходовьё, гавьё).
    declension!("жнивьё", 6, "~ё";
        nominative: "~ё", "~я";
        genitive: "~я", "~ев";
        dative: "~ю", "~ям";
        accusative: referred, referred;
        instrumental: "~ём", "~ями";
        prepositional: "~е", "~ях";
        noun_second_case: "~я", "~е";
    ),
    // Ашников: 6 words (басков, ашников, тыгин, щалкин, дечкин, бакин).
    declension!("ашников", 6, "~";
        nominative: "~", "~ы";
        genitive: "~а", "~ых";
        dative: "~у", "~ым";
        accusative: referred, referred;
        instrumental: "~ым", "~ыми";
        prepositional: "~е", "~ых";
        noun_second_case: "~а", "~е";
    ),
    // Ложь: 6 words (рожь, вошь, ложь, рковь, любовь, церковь).
    declension!("ложь", 6, "~о~ь";
        nominative: "~о~ь", "~~и";
        genitive: "~~и", "~~ей";
        dative: "~~и", "~~ям";
        accusative: referred, referred;
        instrumental: "~о~ью", "~~ями";
        prepositional: "~~и", "~~ях";
        noun_second_case: "~~и", "~~и";
    ),
    // Ерко: 6 words (дёрко, ерко, езко, ёлко, еялко, озко).
    declension!("ерко", 6, "~~о";
        nominative: "~~о", "~~и";
        genitive: "~~а", "~о~";
        dative: "~~у", "~~ам";
        accusative: referred, referred;
        instrumental: "~~ом", "~~ами";
        prepositional: "~~е", "~~ах";
        noun_second_case: "~~а", "~~е";
    ),
    // Слокачалка: 6 words (сло-качалка, сло-коляска, еяло-грелка, сло-вертушка, сло-люлька, сло-каталка).
    declension!("сло-качалка", 6, "~~о~~а";
        nominative: "~~о~~а", "~~а~~и";
        genitive: "~~а~~и", "~е~~о~";
        dative: "~~у~~е", "~~ам~~ам";
        accusative: "~~о~~у", referred;
        instrumental: "~~ом~~ой", "~~ами~~ами";
        prepositional: "~~е~~е", "~~ах~~ах";
        noun_second_case: "~~а~~и", "~~е~~е";
    ),
    // Мьё: 6 words (гвоздьё, мьё, лиганьё, дьё, мадьё, рдьё).
    declension!("мьё", 6, "~ё";
        nominative: "~ё", "~е";
        genitive: "~я", "~е";
        dative: "~ю", "~е";
        accusative: "~ё", referred;
        instrumental: "~ём", "~е";
        prepositional: "~е", "~е";
        noun_second_case: "~я", "~е";
    ),
    // Лета: 6 words (-лета, -лица, -лота, -окна, -училища, -ящика).
    declension!("-лета", 6, "-~а";
        nominative: "-~а", "у~а";
        genitive: "у~а", "у~";
        dative: "у~у", "у~ам";
        accusative: referred, referred;
        instrumental: "у~ом", "у~ами";
        prepositional: "у~е", "у~ах";
        noun_second_case: "у~а", "у~е";
    ),
    // Бой: 5 words (чай, бой, край, клей, иван-чай).
    declension!("бой", 5, "~й";
        nominative: "~й", "~и";
        genitive: "~я", "~ёв";
        dative: "~ю", "~ям";
        accusative: referred, referred;
        instrumental: "~ем", "~ями";
        prepositional: "~е", "~ях";
        noun_second_case: "~ю", "~е";
    ),
    // Йга: 5 words (мука, уха, тыща, туха, йга).
    declension!("йга", 5, "~а";
        nominative: "~а", "~и";
        genitive: "~и", "~";
        dative: "~е", "~ам";
        accusative: "~у", "~";
        instrumental: "~ой|~ою", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~и", "~е";
    ),
    // Горе: 5 words (море, горе, поле, муде, кати-поле).
    declension!("горе", 5, "~е";
        nominative: "~е", "~я";
        genitive: "~я", "~ей";
        dative: "~ю", "~ям";
        accusative: referred, referred;
        instrumental: "~ем", "~ями";
        prepositional: "~е", "~ях";
        noun_second_case: "~я", "~е";
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
    // Зболивающее: 5 words (млекопитающее, ружающее, зболивающее, кожее, шечнодышащее).
    declension!("зболивающее", 5, "~ее";
        nominative: "~ее", "~ие";
        genitive: "~его", "~их";
        dative: "~ему", "~им";
        accusative: "~ее", referred;
        instrumental: "~им", "~ими";
        prepositional: "~ем", "~их";
        noun_second_case: "~его", "~ем";
    ),
    // Кча: 5 words (обжа, оша, кча, джа, араджа).
    declension!("кча", 5, "~а";
        nominative: "~а", "~и";
        genitive: "~и", "~ей";
        dative: "~е", "~ам";
        accusative: "~у", referred;
        instrumental: "~ей|~ею", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~и", "~е";
    ),
    // Зеленя: 5 words (зеленя, чигиря, уборя, селя, хоря).
    declension!("зеленя", 5, "~я";
        nominative: "~я", "~я";
        genitive: "~и", "~ей";
        dative: "~е", "~ям";
        accusative: "~ю", referred;
        instrumental: "~ёй", "~ями";
        prepositional: "~е", "~ях";
        noun_second_case: "~и", "~е";
    ),
    // Пики: 5 words (пики, интервики, салоники, пагитики, оники).
    declension!("пики", 5, "~и";
        nominative: "~и", "~и";
        genitive: "~и", "~";
        dative: "~е", "~ам";
        accusative: "~у", referred;
        instrumental: "~ой", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~и", "~е";
    ),
    // Коть: 5 words (вихорь, коть, готь, моть, огонь).
    declension!("коть", 5, "~о~ь";
        nominative: "~о~ь", "~~и";
        genitive: "~~я", "~~ей";
        dative: "~~ю", "~~ям";
        accusative: referred, referred;
        instrumental: "~~ем", "~~ями";
        prepositional: "~~е", "~~ях";
        noun_second_case: "~~я", "~~е";
    ),
    // Злы: 5 words (жны, бны, злы, ерцы, вны).
    declension!("злы", 5, "~~ы";
        nominative: "~~а", "~~ы";
        genitive: "~~ы", "~е~";
        dative: "~~е", "~~ам";
        accusative: "~~у", referred;
        instrumental: "~~ой", "~~ами";
        prepositional: "~~е", "~~ах";
        noun_second_case: "~~ы", "~~е";
    ),
    // Пей: 5 words (вей, обей, авей, чей, пей).
    declension!("пей", 5, "~ей";
        nominative: "~ей", "~ьи";
        genitive: "~ья", "~ьёв";
        dative: "~ью", "~ьям";
        accusative: referred, referred;
        instrumental: "~ьём", "~ьями";
        prepositional: "~ье", "~ьях";
        noun_second_case: "~ья", "~ье";
    ),
    // Аловушка: 5 words (а-ловушка, а-яга, а-гадюка, а-растяжка, а-вонючка).
    declension!("а-ловушка", 5, "а~~а";
        nominative: "а~~а", "ы~~и";
        genitive: "ы~~и", "~е~";
        dative: "е~~е", "ам~~ам";
        accusative: "у~~у", referred;
        instrumental: "ой~~ой|ою~~ою", "ами~~ами";
        prepositional: "е~~е", "ах~~ах";
        noun_second_case: "ы~~и", "е~~е";
    ),
    // Бега: 5 words (бега, торока, холода, севера, роха).
    declension!("бега", 5, "~а";
        nominative: "~а", "~а";
        genitive: "~и", "~ов";
        dative: "~е", "~ам";
        accusative: "~у", referred;
        instrumental: "~ой", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~и", "~е";
    ),
    // Амалаша: 5 words (а-пила, а-рыба, а-зебра, а-труба, а-малаша).
    declension!("а-малаша", 5, "а~а";
        nominative: "а~а", "ы~ы";
        genitive: "ы~ы", "~";
        dative: "е~е", "ам~ам";
        accusative: "у~у", referred;
        instrumental: "ой~ой|ою~ою", "ами~ами";
        prepositional: "е~е", "ах~ах";
        noun_second_case: "ы~ы", "е~е";
    ),
    // Джакболеро: 5 words (чик-с-пальчик, сомольск-на-амуре, авянск-на-кубани, джак-болеро, ксандровск-на-мурмане).
    declension!("джак-болеро", 5, "~~";
        nominative: "~~", "~и~";
        genitive: "~а~", "~ов~";
        dative: "~у~", "~ам~";
        accusative: referred, referred;
        instrumental: "~ом~", "~ами~";
        prepositional: "~е~", "~ах~";
        noun_second_case: "~а~", "~е~";
    ),
    // Вьморковь: 5 words (голь-моголь, дкость-теплоноситель, итель-испытатель, вь-морковь, итель-округлитель).
    declension!("вь-морковь", 5, "~ь~ь";
        nominative: "~ь~ь", "~и~и";
        genitive: "~я~я", "~ей~ей";
        dative: "~ю~ю", "~ям~ям";
        accusative: referred, referred;
        instrumental: "~ем~ем", "~ями~ями";
        prepositional: "~е~е", "~ях~ях";
        noun_second_case: "~я~я", "~е~е";
    ),
    // Дсад: 5 words (д-сад, д-побратим, пел-арлекин, ессор-консультант, к-волкодав).
    declension!("д-сад", 5, "~~";
        nominative: "~~", "~а~ы";
        genitive: "~а~а", "~ов~ов";
        dative: "~у~у", "~ам~ам";
        accusative: referred, referred;
        instrumental: "~ом~ом", "~ами~ами";
        prepositional: "~е~е", "~ах~ах";
        noun_second_case: "~а~а", "~е~е";
    ),
    // Тьефрак: 5 words (приятие-субподрядчик, тье-корсаж, тье-купальник, тье-пиджак, тье-фрак).
    declension!("тье-фрак", 5, "~е~";
        nominative: "~е~", "~я~и";
        genitive: "~я~а", "~ев~ов";
        dative: "~ю~у", "~ям~ам";
        accusative: referred, referred;
        instrumental: "~ем~ом", "~ями~ами";
        prepositional: "~е~е", "~ях~ах";
        noun_second_case: "~я~а", "~е~е";
    ),
    // Фий: 4 words (гравий, опий, фий, орий).
    declension!("фий", 4, "~й";
        nominative: "~й", "~и";
        genitive: "~я", "~ев";
        dative: "~ю", "~ям";
        accusative: referred, referred;
        instrumental: "~ем", "~ями";
        prepositional: "~и", "~ях";
        noun_second_case: "~ю", "~и";
    ),
    // Дичь: 4 words (мышь, дичь, лочь, толочь).
    declension!("дичь", 4, "~ь";
        nominative: "~ь", "~и";
        genitive: "~и", "~ей";
        dative: "~и", "~ам";
        accusative: "~ь", referred;
        instrumental: "~ью", "~ами";
        prepositional: "~и", "~ах";
        noun_second_case: "~и", "~и";
    ),
    // Весна: 4 words (десна, весна, метла, ветла).
    declension!("весна", 4, "~е~~а";
        nominative: "~е~~а", "~ё~~ы";
        genitive: "~е~~ы", "~ё~е~";
        dative: "~е~~е", "~ё~~ам";
        accusative: "~е~~у", referred;
        instrumental: "~е~~ой|~е~~ою", "~ё~~ами";
        prepositional: "~е~~е", "~ё~~ах";
        noun_second_case: "~е~~ы", "~е~~е";
    ),
    // Улей: 4 words (улей, чирей, ирей, ребей).
    declension!("улей", 4, "~ей";
        nominative: "~ей", "~ьи";
        genitive: "~ья", "~ьев";
        dative: "~ью", "~ьям";
        accusative: referred, referred;
        instrumental: "~ьем", "~ьями";
        prepositional: "~ье", "~ьях";
        noun_second_case: "~ья", "~ье";
    ),
    // Ящийся: 4 words (учащийся, ящийся, определяющийся, здношатающийся).
    declension!("ящийся", 4, "~ий~";
        nominative: "~ий~", "~ие~";
        genitive: "~его~", "~их~";
        dative: "~ему~", "~им~";
        accusative: referred, referred;
        instrumental: "~им~", "~ими~";
        prepositional: "~ем~", "~их~";
        noun_second_case: "~его~", "~ем~";
    ),
    // Фья: 4 words (ектенья, епитимья, фья, емья).
    declension!("фья", 4, "~ья";
        nominative: "~ья", "~ьи";
        genitive: "~ьи", "~ий";
        dative: "~ье", "~ьям";
        accusative: "~ью", referred;
        instrumental: "~ьёй|~ьёю", "~ьями";
        prepositional: "~ье", "~ьях";
        noun_second_case: "~ьи", "~ье";
    ),
    // Арин: 4 words (хазарин, гарин, арин, иджанин).
    declension!("арин", 4, "~ин";
        nominative: "~ин", "~ы";
        genitive: "~ина", "~";
        dative: "~ину", "~ам";
        accusative: referred, referred;
        instrumental: "~ином", "~ами";
        prepositional: "~ине", "~ах";
        noun_second_case: "~ина", "~ине";
    ),
    // Горошек: 4 words (горошек, лишек, чаёчек, очек).
    declension!("горошек", 4, "~е~";
        nominative: "~е~", "~~и";
        genitive: "~~а", "~~ов";
        dative: "~~у", "~~ам";
        accusative: referred, referred;
        instrumental: "~~ом", "~~ами";
        prepositional: "~~е", "~~ах";
        noun_second_case: "~~у", "~~е";
    ),
    // Сынуля: 4 words (сынуля, саня, меля, руля).
    declension!("сынуля", 4, "~я";
        nominative: "~я", "~и";
        genitive: "~и", "~ь";
        dative: "~е", "~ям";
        accusative: "~ю", "~и";
        instrumental: "~ей|~ею", "~ями";
        prepositional: "~е", "~ях";
        noun_second_case: "~и", "~е";
    ),
    // Мыкающееся: 4 words (мыкающееся, чившееся, ывшееся, бывшееся).
    declension!("мыкающееся", 4, "~ее~";
        nominative: "~ее~", "~ие~";
        genitive: "~его~", "~их~";
        dative: "~ему~", "~им~";
        accusative: "~ее~", referred;
        instrumental: "~им~", "~ими~";
        prepositional: "~ем~", "~их~";
        noun_second_case: "~его~", "~ем~";
    ),
    // Одбогоносец: 4 words (од-богоносец, сист-ленинец, атор-республиканец, ин-афганец).
    declension!("од-богоносец", 4, "~~е~";
        nominative: "~~е~", "~ы~~ы";
        genitive: "~а~~а", "~ов~~ев";
        dative: "~у~~у", "~ам~~ам";
        accusative: referred, referred;
        instrumental: "~ом~~ем", "~ами~~ами";
        prepositional: "~е~~е", "~ах~~ах";
        noun_second_case: "~а~~а", "~е~~е";
    ),
    // Ерочек: 4 words (иночек, тылочек, ерочек, еночек).
    declension!("ерочек", 4, "~е~";
        nominative: "~е~", "~~и";
        genitive: "~~а", "~е~";
        dative: "~~у", "~~ам";
        accusative: referred, referred;
        instrumental: "~~ом", "~~ами";
        prepositional: "~~е", "~~ах";
        noun_second_case: "~~а", "~~е";
    ),
    // Рубашечник: 4 words (рубашечник, стралопитек, питек, лятник).
    declension!("рубашечник", 4, "~";
        nominative: "~", "~и";
        genitive: "~а", "~ов";
        dative: "~у", "~ам";
        accusative: "~", "~и";
        instrumental: "~ом", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~а", "~е";
    ),
    // Ачинтерн: 4 words (ач-интерн, ач-консультант, ач-педиатр, щ-тренчкот).
    declension!("ач-интерн", 4, "~~";
        nominative: "~~", "~и~ы";
        genitive: "~а~а", "~ей~ов";
        dative: "~у~у", "~ам~ам";
        accusative: referred, referred;
        instrumental: "~ом~ом", "~ами~ами";
        prepositional: "~е~е", "~ах~ах";
        noun_second_case: "~а~а", "~е~е";
    ),
    // Пилот: 4 words (-пилот, -гастролёр, -экскурсовод, -перемат).
    declension!("-пилот", 4, "~";
        nominative: "~", "ы~ы";
        genitive: "а~а", "ов~ов";
        dative: "у~у", "ам~ам";
        accusative: referred, referred;
        instrumental: "ом~ом", "ами~ами";
        prepositional: "е~е", "ах~ах";
        noun_second_case: "а~а", "е~е";
    ),
    // Ельце: 4 words (ельце, еяльце, кольце, гощупальце).
    declension!("ельце", 4, "~ь~е";
        nominative: "~ь~е", "~ь~а";
        genitive: "~ь~а", "~ь~ев|~е~";
        dative: "~ь~у", "~ь~ам";
        accusative: referred, referred;
        instrumental: "~ь~ем", "~ь~ами";
        prepositional: "~ь~е", "~ь~ах";
        noun_second_case: "~ь~а", "~ь~е";
    ),
    // Тко: 4 words (тятко, ичонко, тко, жичонко).
    declension!("тко", 4, "~~о";
        nominative: "~~о", "~~и";
        genitive: "~~а", "~о~";
        dative: "~~у", "~~ам";
        accusative: "~~о", referred;
        instrumental: "~~ом", "~~ами";
        prepositional: "~~е", "~~ах";
        noun_second_case: "~~а", "~~е";
    ),
    // Юхино: 4 words (юхино, юхново, аново, ментьево).
    declension!("юхино", 4, "~о";
        nominative: "~о", "~а|~о";
        genitive: "~а|~о", "~о";
        dative: "~у|~о", "~ам|~о";
        accusative: referred, referred;
        instrumental: "~ом|~о", "~ами|~о";
        prepositional: "~е|~о", "~ах|~о";
        noun_second_case: "~а|~о", "~е|~о";
    ),
    // Сккузнецкий: 4 words (ск-кузнецкий, енск-шахтинский, ксандровск-сахалинский, енск-уральский).
    declension!("ск-кузнецкий", 4, "~~ий";
        nominative: "~~ий", "~и~ие";
        genitive: "~а~ого", "~ов~их";
        dative: "~у~ому", "~ам~им";
        accusative: referred, referred;
        instrumental: "~ом~им", "~ами~ими";
        prepositional: "~е~ом", "~ах~их";
        noun_second_case: "~а~ого", "~е~ом";
    ),
    // Прея: 4 words (иоидея, урея, аторея, прея).
    declension!("прея", 4, "~я";
        nominative: "~я", "~и";
        genitive: "~и", "~й";
        dative: "~е", "~ям";
        accusative: "~ю", "~й";
        instrumental: "~ей|~ею", "~ями";
        prepositional: "~е", "~ях";
        noun_second_case: "~и", "~е";
    ),
    // Адьба: 3 words (тьма, адьба, рьма).
    declension!("адьба", 3, "~ь~а";
        nominative: "~ь~а", "~ь~ы";
        genitive: "~ь~ы", "~е~";
        dative: "~ь~е", "~ь~ам";
        accusative: "~ь~у", referred;
        instrumental: "~ь~ой|~ь~ою", "~ь~ами";
        prepositional: "~ь~е", "~ь~ах";
        noun_second_case: "~ь~ы", "~ь~е";
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
    // Боец: 3 words (боец, хуец, гаец).
    declension!("боец", 3, "~е~";
        nominative: "~е~", "~й~ы";
        genitive: "~й~а", "~й~ов";
        dative: "~й~у", "~й~ам";
        accusative: referred, referred;
        instrumental: "~й~ом", "~й~ами";
        prepositional: "~й~е", "~й~ах";
        noun_second_case: "~й~а", "~й~е";
    ),
    // Желна: 3 words (пелена, желна, шева).
    declension!("желна", 3, "~е~а";
        nominative: "~е~а", "~е~ы";
        genitive: "~е~ы", "~ё~";
        dative: "~е~е", "~е~ам";
        accusative: "~е~у", referred;
        instrumental: "~е~ой|~е~ою", "~е~ами";
        prepositional: "~е~е", "~е~ах";
        noun_second_case: "~е~ы", "~е~е";
    ),
    // Чёлн: 3 words (ксёндз, чёлн, пётр).
    declension!("чёлн", 3, "~ё~";
        nominative: "~ё~", "~е~ы";
        genitive: "~е~а", "~е~ов";
        dative: "~е~у", "~е~ам";
        accusative: referred, referred;
        instrumental: "~е~ом", "~е~ами";
        prepositional: "~е~е", "~е~ах";
        noun_second_case: "~е~а", "~е~е";
    ),
    // Дверца: 3 words (дверца, орца, дерца).
    declension!("дверца", 3, "~~а";
        nominative: "~~а", "~~ы";
        genitive: "~~ы", "~е~";
        dative: "~~е", "~~ам";
        accusative: "~~у", referred;
        instrumental: "~~ей|~~ею", "~~ами";
        prepositional: "~~е", "~~ах";
        noun_second_case: "~~ы", "~~е";
    ),
    // Макаронник: 3 words (счётчик, макаронник, архистратиг).
    declension!("макаронник", 3, "~";
        nominative: "~", "~и";
        genitive: "~а", "~ов";
        dative: "~у", "~ам";
        accusative: "~а|~", "~ов|~и";
        instrumental: "~ом", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~а", "~е";
    ),
    // Шай: 3 words (бугай, луй, шай).
    declension!("шай", 3, "~й";
        nominative: "~й", "~и";
        genitive: "~я", "~ёв";
        dative: "~ю", "~ям";
        accusative: referred, referred;
        instrumental: "~ём", "~ями";
        prepositional: "~е", "~ях";
        noun_second_case: "~я", "~е";
    ),
    // Рть: 3 words (мразь, рть, янь).
    declension!("рть", 3, "~ь";
        nominative: "~ь", "~и";
        genitive: "~и", "~ей";
        dative: "~и", "~ям";
        accusative: referred, "~ей|~и";
        instrumental: "~ью", "~ями";
        prepositional: "~и", "~ях";
        noun_second_case: "~и", "~и";
    ),
    // Чья: 3 words (моя, чья, ичья).
    declension!("чья", 3, "~я";
        nominative: "~я", "~и";
        genitive: "~ей", "~их";
        dative: "~ей", "~им";
        accusative: "~ю", referred;
        instrumental: "~ей|~ею", "~ими";
        prepositional: "~ей", "~их";
        noun_second_case: "~ей", "~ей";
    ),
    // Вела: 3 words (шала, вора, вела).
    declension!("вела", 3, "~а";
        nominative: "~а", "~ы";
        genitive: "~ы", "~";
        dative: "~е", "~ам";
        accusative: "~у", "~";
        instrumental: "~ой|~ою", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~ы", "~е";
    ),
    // Тиса: 3 words (шила, тиса, драва).
    declension!("тиса", 3, "~а";
        nominative: "~а", "~а";
        genitive: "~ы", "~";
        dative: "~е", "~а";
        accusative: "~у", referred;
        instrumental: "~ой", "~а";
        prepositional: "~е", "~а";
        noun_second_case: "~ы", "~е";
    ),
    // Уйки: 3 words (яйки, уйки, щайки).
    declension!("уйки", 3, "~й~и";
        nominative: "~й~а", "~й~и";
        genitive: "~й~и", "~е~";
        dative: "~й~е", "~й~ам";
        accusative: "~й~у", referred;
        instrumental: "~й~ой", "~й~ами";
        prepositional: "~й~е", "~й~ах";
        noun_second_case: "~й~и", "~й~е";
    ),
    // Хня: 3 words (хня, фня, кухня).
    declension!("хня", 3, "~~я";
        nominative: "~~я", "~~и";
        genitive: "~~и", "~о~ь";
        dative: "~~е", "~~ям";
        accusative: "~~ю", referred;
        instrumental: "~~ей|~~ею", "~~ями";
        prepositional: "~~е", "~~ях";
        noun_second_case: "~~и", "~~е";
    ),
    // Едостениё: 3 words (миё, едостениё, звеё).
    declension!("едостениё", 3, "~ё";
        nominative: "~ё", "~я";
        genitive: "~я", "~й";
        dative: "~ю", "~ям";
        accusative: referred, referred;
        instrumental: "~ём", "~ями";
        prepositional: "~е", "~ях";
        noun_second_case: "~я", "~е";
    ),
    // Иум: 3 words (дест, иум, хад).
    declension!("иум", 3, "~";
        nominative: "~", "~ы";
        genitive: "~а", "~ов";
        dative: "~у", "~ам";
        accusative: referred, "~ов";
        instrumental: "~ом", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~а", "~е";
    ),
    // Ясли: 3 words (ясли, мысли, сли).
    declension!("ясли", 3, "~~и";
        nominative: "~~и", "~~и";
        genitive: "~~и", "~~ей";
        dative: "~~и", "~~ям";
        accusative: referred, referred;
        instrumental: "~~и", "~~ями";
        prepositional: "~~и", "~~ях";
        noun_second_case: "~~и", "~~и";
    ),
    // Лнышко: 3 words (лнышко, щечко, ентишко).
    declension!("лнышко", 3, "~~о";
        nominative: "~~о", "~~и";
        genitive: "~~а", "~е~";
        dative: "~~у", "~~ам";
        accusative: "~~о", referred;
        instrumental: "~~ом", "~~ами";
        prepositional: "~~е", "~~ах";
        noun_second_case: "~~а", "~~е";
    ),
    // Ушкапердушка: 3 words (ушка-пердушка, гушка-квакушка, лочка-выручалочка).
    declension!("ушка-пердушка", 3, "~~а~~а";
        nominative: "~~а~~а", "~~и~~и";
        genitive: "~~и~~и", "~е~~е~";
        dative: "~~е~~е", "~~ам~~ам";
        accusative: "~~у~~у", referred;
        instrumental: "~~ой~~ой|~~ою~~ою", "~~ами~~ами";
        prepositional: "~~е~~е", "~~ах~~ах";
        noun_second_case: "~~и~~и", "~~е~~е";
    ),
    // Кодиночка: 3 words (щник-крошка, к-одиночка, ряк-самоучка).
    declension!("к-одиночка", 3, "~~~а";
        nominative: "~~~а", "~и~~и";
        genitive: "~а~~и", "~ов~е~";
        dative: "~у~~е", "~ам~~ам";
        accusative: "~а~~у", referred;
        instrumental: "~ом~~ой|~ом~~ою", "~ами~~ами";
        prepositional: "~е~~е", "~ах~~ах";
        noun_second_case: "~а~~и", "~е~~е";
    ),
    // Мкатележка: 3 words (енка-монашка, оловка-пеструшка, мка-тележка).
    declension!("мка-тележка", 3, "~~а~~а";
        nominative: "~~а~~а", "~~и~~и";
        genitive: "~~и~~и", "~о~~е~";
        dative: "~~е~~е", "~~ам~~ам";
        accusative: "~~у~~у", referred;
        instrumental: "~~ой~~ой|~~ою~~ою", "~~ами~~ами";
        prepositional: "~~е~~е", "~~ах~~ах";
        noun_second_case: "~~и~~и", "~~е~~е";
    ),
    // Бдвойка: 3 words (б-двойка, тюм-двойка, тюм-тройка).
    declension!("б-двойка", 3, "~~й~а";
        nominative: "~~й~а", "~ы~й~и";
        genitive: "~а~й~и", "~ов~е~";
        dative: "~у~й~е", "~ам~й~ам";
        accusative: "~~й~у", referred;
        instrumental: "~ом~й~ой|~ом~й~ою", "~ами~й~ами";
        prepositional: "~е~й~е", "~ах~й~ах";
        noun_second_case: "~а~й~и", "~е~й~е";
    ),
    // Гопа: 3 words (лаба, гопа, два).
    declension!("гопа", 3, "~а";
        nominative: "~а", "~а";
        genitive: "~ы", "~а";
        dative: "~е", "~а";
        accusative: "~у", referred;
        instrumental: "~ой|~ою", "~а";
        prepositional: "~е", "~а";
        noun_second_case: "~ы", "~е";
    ),
    // Ведьгубач: 3 words (мобиль-тягач, итель-лихач, ведь-губач).
    declension!("ведь-губач", 3, "~ь~";
        nominative: "~ь~", "~и~и";
        genitive: "~я~а", "~ей~ей";
        dative: "~ю~у", "~ям~ам";
        accusative: referred, referred;
        instrumental: "~ем~ом", "~ями~ами";
        prepositional: "~е~е", "~ях~ах";
        noun_second_case: "~я~а", "~е~е";
    ),
    // Црегент: 3 words (ц-консорт, ц-регент, анц-иосиф).
    declension!("ц-регент", 3, "~~";
        nominative: "~~", "~ы~ы";
        genitive: "~а~а", "~ев~ов";
        dative: "~у~у", "~ам~ам";
        accusative: referred, referred;
        instrumental: "~ем~ом", "~ами~ами";
        prepositional: "~е~е", "~ах~ах";
        noun_second_case: "~а~а", "~е~е";
    ),
    // Таншутиха: 3 words (уб-дискотека, тан-шутиха, гон-библиотека).
    declension!("тан-шутиха", 3, "~~а";
        nominative: "~~а", "~ы~и";
        genitive: "~а~и", "~ов~";
        dative: "~у~е", "~ам~ам";
        accusative: "~~у", referred;
        instrumental: "~ом~ой|~ом~ою", "~ами~ами";
        prepositional: "~е~е", "~ах~ах";
        noun_second_case: "~а~и", "~е~е";
    ),
    // Тьеампир: 3 words (тье-ампир, тье-бюстье, тье-стрейч).
    declension!("тье-ампир", 3, "~е~";
        nominative: "~е~", "~я~";
        genitive: "~я~", "~ев~";
        dative: "~ю~", "~ям~";
        accusative: referred, referred;
        instrumental: "~ем~", "~ями~";
        prepositional: "~е~", "~ях~";
        noun_second_case: "~я~", "~е~";
    ),
    // Леса: 3 words (-леса, -округа, -острова).
    declension!("-леса", 3, "-~а";
        nominative: "-~а", "у~а";
        genitive: "у~а", "у~ов";
        dative: "у~у", "у~ам";
        accusative: referred, referred;
        instrumental: "у~ом", "у~ами";
        prepositional: "у~е", "у~ах";
        noun_second_case: "у~а", "у~е";
    ),
    // Наизгой: 3 words (ола-лицей, ната-музей, на-изгой).
    declension!("на-изгой", 3, "~а~й";
        nominative: "~а~й", "~ы~и";
        genitive: "~ы~я", "~~ев";
        dative: "~е~ю", "~ам~ям";
        accusative: "~у~й", referred;
        instrumental: "~ой~ем|~ою~ем", "~ами~ями";
        prepositional: "~е~е", "~ах~ях";
        noun_second_case: "~ы~я", "~е~е";
    ),
];
