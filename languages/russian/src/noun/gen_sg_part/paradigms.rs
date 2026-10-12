//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's feature says):
/// `genitive` (sg)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal;
     genitive: $c0:tt;
    ) => {
        Paradigm {
            name: ::std::borrow::Cow::Borrowed($name),
            words: $words,
            lemma: ::std::borrow::Cow::Borrowed($lemma),
            forms: ::std::borrow::Cow::Borrowed(&[flexion::form!($c0)]),
        }
    };
}

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `приём`: 39,952 words.
    Приём,
    /// Like `стрелка`: 12,822 words.
    Стрелка,
    /// Like `скопление`: 11,741 words.
    Скопление,
    /// Like `полнота`: 10,211 words.
    Полнота,
    /// Like `заря`: 7,697 words.
    Заря,
    /// Like `склонность`: 7,635 words.
    Склонность,
    /// Like `какао`: 3,358 words.
    Какао,
    /// Like `отец`: 2,847 words.
    Отец,
    /// Like `ядро`: 2,702 words.
    Ядро,
    /// Like `червь`: 2,240 words.
    Червь,
    /// Like `бой`: 1,342 words.
    Бой,
    /// Like `рожок`: 1,271 words.
    Рожок,
    /// Like `коленце`: 429 words.
    Коленце,
    /// Like `накладная`: 406 words.
    Накладная,
    /// Like `зелёный`: 276 words.
    Зелёный,
    /// Like `олимпиец`: 264 words.
    Олимпиец,
    /// Like `второе`: 218 words.
    Второе,
    /// Like `умелец`: 198 words.
    Умелец,
    /// Like `русский`: 191 words.
    Русский,
    /// Like `жнивьё`: 125 words.
    Жнивьё,
    /// Like `камень`: 120 words.
    Камень,
    /// Like `кормчий`: 115 words.
    Кормчий,
    /// Like `гусёк`: 92 words.
    Гусёк,
    /// Like `штрафной`: 79 words.
    Штрафной,
    /// Like `пол-литра`: 52 words.
    Поллитра,
    /// Like `съезжая`: 46 words.
    Съезжая,
    /// Like `осётр`: 42 words.
    Осётр,
    /// Like `полгода`: 29 words.
    Полгода,
    /// Like `боёк`: 25 words.
    Боёк,
    /// Like `козёл`: 20 words.
    Козёл,
    /// Like `племя`: 17 words.
    Племя,
    /// Like `губоцветные`: 15 words.
    Губоцветные,
    /// Like `огонь`: 12 words.
    Огонь,
    /// Like `репей`: 9 words.
    Репей,
    /// Like `ложь`: 7 words.
    Ложь,
    /// Like `передняя`: 7 words.
    Передняя,
    /// Like `учащийся`: 4 words.
    Учащийся,
    /// Like `пресмыкающееся`: 4 words.
    Пресмыкающееся,
    /// Like `ничья`: 4 words.
    Ничья,
    /// Like `пол-узла`: 4 words.
    Полузла,
    /// Like `это`: 3 words.
    Это,
    /// Like `дочь`: 3 words.
    Дочь,
    /// Like `что`: 3 words.
    Что,
    /// Like `кто`: 3 words.
    Кто,
    /// Like `пальто`: 3 words.
    Пальто,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 45] = [
        Declension::Приём,
        Declension::Стрелка,
        Declension::Скопление,
        Declension::Полнота,
        Declension::Заря,
        Declension::Склонность,
        Declension::Какао,
        Declension::Отец,
        Declension::Ядро,
        Declension::Червь,
        Declension::Бой,
        Declension::Рожок,
        Declension::Коленце,
        Declension::Накладная,
        Declension::Зелёный,
        Declension::Олимпиец,
        Declension::Второе,
        Declension::Умелец,
        Declension::Русский,
        Declension::Жнивьё,
        Declension::Камень,
        Declension::Кормчий,
        Declension::Гусёк,
        Declension::Штрафной,
        Declension::Поллитра,
        Declension::Съезжая,
        Declension::Осётр,
        Declension::Полгода,
        Declension::Боёк,
        Declension::Козёл,
        Declension::Племя,
        Declension::Губоцветные,
        Declension::Огонь,
        Declension::Репей,
        Declension::Ложь,
        Declension::Передняя,
        Declension::Учащийся,
        Declension::Пресмыкающееся,
        Declension::Ничья,
        Declension::Полузла,
        Declension::Это,
        Declension::Дочь,
        Declension::Что,
        Declension::Кто,
        Declension::Пальто,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 45] = [
    // Приём: 39,952 words (указ, зонт, кот, зонтик, бортник, бочаг).
    declension!("приём", 39952, "~";
        genitive: "~а";
    ),
    // Стрелка: 12,822 words (собака, кошка, быличка, водка, палочка, цыпка).
    declension!("стрелка", 12822, "~а";
        genitive: "~и";
    ),
    // Скопление: 11,741 words (произношение, лезвие, запястье, распятие, деревенение, лесонасаждение).
    declension!("скопление", 11741, "~е";
        genitive: "~я";
    ),
    // Полнота: 10,211 words (буява, былица, масленица, царица, вата, лисица).
    declension!("полнота", 10211, "~а";
        genitive: "~ы";
    ),
    // Заря: 7,697 words (братыня, лесопильня, анархия, лесохимия, земля, неделя).
    declension!("заря", 7697, "~я";
        genitive: "~и";
    ),
    // Склонность: 7,635 words (знать, весь, гласность, деревянность, лошадь, дверь).
    declension!("склонность", 7635, "~ь";
        genitive: "~и";
    ),
    // Какао: 3,358 words (адажио, какао, полпути, полдороги, полвторого, пенни).
    declension!("какао", 3358, "~";
        genitive: "~";
    ),
    // Отец: 2,847 words (отец, левантинец, древоточец, горошек, свинец, марганец).
    declension!("отец", 2847, "~е~";
        genitive: "~~а";
    ),
    // Ядро: 2,702 words (брашно, буево, бёрдо, яблоко, озеро, око).
    declension!("ядро", 2702, "~о";
        genitive: "~а";
    ),
    // Червь: 2,240 words (царь, сшиватель, рубль, кремль, дождь, январь).
    declension!("червь", 2240, "~ь";
        genitive: "~я";
    ),
    // Бой: 1,342 words (чай, май, калий, гей, меркурий, вертухай).
    declension!("бой", 1342, "~й";
        genitive: "~я";
    ),
    // Рожок: 1,271 words (щенок, бесёнок, ботинок, лесок, замок, носок).
    declension!("рожок", 1271, "~о~";
        genitive: "~~а";
    ),
    // Коленце: 429 words (сердце, солнце, вече, училище, кладбище, деревце).
    declension!("коленце", 429, "~е";
        genitive: "~а";
    ),
    // Накладная: 406 words (деревообделочная, детская, гостиная, ванная, кладовая, уборная).
    declension!("накладная", 406, "~ая";
        genitive: "~ой";
    ),
    // Зелёный: 276 words (белый, чёрный, гласный, беспризорный, коридорный, левый).
    declension!("зелёный", 276, "~ый";
        genitive: "~ого";
    ),
    // Олимпиец: 264 words (австриец, боец, австралиец, ариец, белогвардеец, китаец).
    declension!("олимпиец", 264, "~е~";
        genitive: "~й~а";
    ),
    // Второе: 218 words (былое, настоящее, насекомое, существительное, числительное, многое).
    declension!("второе", 218, "~е";
        genitive: "~го";
    ),
    // Умелец: 198 words (палец, лев, телец, комсомолец, земледелец, бразилец).
    declension!("умелец", 198, "~е~";
        genitive: "~ь~а";
    ),
    // Русский: 191 words (русский, сербскохорватский, английский, норвежский, китайский, немецкий).
    declension!("русский", 191, "~ий";
        genitive: "~ого";
    ),
    // Жнивьё: 125 words (копьё, питьё, ружьё, битьё, мумиё, враньё).
    declension!("жнивьё", 125, "~ё";
        genitive: "~я";
    ),
    // Камень: 120 words (бредень, день, оборотень, камень, парень, кашель).
    declension!("камень", 120, "~е~ь";
        genitive: "~~я";
    ),
    // Кормчий: 115 words (лесничий, рабочий, посторонний, нищий, военнослужащий, домашний).
    declension!("кормчий", 115, "~ий";
        genitive: "~его";
    ),
    // Гусёк: 92 words (лёд, мотылёк, лён, хорёк, кошелёк, королёк).
    declension!("гусёк", 92, "~ё~";
        genitive: "~ь~а";
    ),
    // Штрафной: 79 words (больной, голубой, часовой, золотой, вестовой, немой).
    declension!("штрафной", 79, "~й";
        genitive: "~го";
    ),
    // Поллитра: 52 words (пол-очка, пол-литра, пол-оборота, пол-августа, пол-автобуса, пол-аршина).
    declension!("пол-литра", 52, "~-~";
        genitive: "~у~";
    ),
    // Съезжая: 46 words (прихожая, телеведущая, съезжая, неверующая, купчая, составляющая).
    declension!("съезжая", 46, "~ая";
        genitive: "~ей";
    ),
    // Осётр: 42 words (ксёндз, чёлн, пиздёж, чертёж, осётр, грабёж).
    declension!("осётр", 42, "~ё~";
        genitive: "~е~а";
    ),
    // Полгода: 29 words (полчаса, полгода, полвека, полметра, полмиллиона, полминуты).
    declension!("полгода", 29, "~~";
        genitive: "~у~";
    ),
    // Боёк: 25 words (заём, наём, паёк, чаёк, ручеёк, поднаём).
    declension!("боёк", 25, "~ё~";
        genitive: "~й~а";
    ),
    // Козёл: 20 words (пёс, козёл, орёл, овёс, осёл, костёр).
    declension!("козёл", 20, "~ё~";
        genitive: "~~а";
    ),
    // Племя: 17 words (имя, время, знамя, бремя, вымя, пламя).
    declension!("племя", 17, "~я";
        genitive: "~ени";
    ),
    // Губоцветные: 15 words (вересковые, водяниковые, верескоцветные, буковые, букоцветные, мальпигиевые).
    declension!("губоцветные", 15, "~е";
        genitive: "~х";
    ),
    // Огонь: 12 words (огонь, локоть, дёготь, угорь, ноготь, уголь).
    declension!("огонь", 12, "~о~ь";
        genitive: "~~я";
    ),
    // Репей: 9 words (соловей, воробей, муравей, ручей, улей, репей).
    declension!("репей", 9, "~ей";
        genitive: "~ья";
    ),
    // Ложь: 7 words (церковь, рожь, вошь, ложь, любовь, нелюбовь).
    declension!("ложь", 7, "~о~ь";
        genitive: "~~и";
    ),
    // Передняя: 7 words (передняя, преисподняя, ближняя, замужняя, незамужняя, несовершеннолетняя).
    declension!("передняя", 7, "~яя";
        genitive: "~ей";
    ),
    // Учащийся: 4 words (учащийся, трудящийся, вольноопределяющийся, праздношатающийся).
    declension!("учащийся", 4, "~ий~";
        genitive: "~его~";
    ),
    // Пресмыкающееся: 4 words (пресмыкающееся, случившееся, сбывшееся, несбывшееся).
    declension!("пресмыкающееся", 4, "~е~";
        genitive: "~го~";
    ),
    // Ничья: 4 words (ничья, третья, девичья, моя).
    declension!("ничья", 4, "~я";
        genitive: "~ей";
    ),
    // Полузла: 4 words (пол-узла, пол-университета, пол-урока, пол-училища).
    declension!("пол-узла", 4, "~-~~";
        genitive: "~~у~";
    ),
    // Это: 3 words (это, то, третье).
    declension!("это", 3, "~";
        genitive: "~го";
    ),
    // Дочь: 3 words (дочь, мать, яжмать).
    declension!("дочь", 3, "~ь";
        genitive: "~ери";
    ),
    // Что: 3 words (что, ничто, кое-что).
    declension!("что", 3, "~т~";
        genitive: "~ег~";
    ),
    // Кто: 3 words (кто, кое-кто, никто).
    declension!("кто", 3, "~т~";
        genitive: "~~го";
    ),
    // Пальто: 3 words (пальто, косово, фирово).
    declension!("пальто", 3, "~о";
        genitive: "~о|~а";
    ),
];
