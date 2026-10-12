//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's feature says):
/// `nominative` (sg)
/// `accusative` (sg)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal;
     nominative: $c0:tt;
     accusative: $c1:tt;
    ) => {
        Paradigm {
            name: ::std::borrow::Cow::Borrowed($name),
            words: $words,
            lemma: ::std::borrow::Cow::Borrowed($lemma),
            forms: ::std::borrow::Cow::Borrowed(&[flexion::form!($c0), flexion::form!($c1)]),
        }
    };
}

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `ядро`: 58,294 words.
    Ядро,
    /// Like `полнота`: 23,176 words.
    Полнота,
    /// Like `дух`: 13,610 words.
    Дух,
    /// Like `заря`: 7,747 words.
    Заря,
    /// Like `отец`: 2,128 words.
    Отец,
    /// Like `червь`: 1,023 words.
    Червь,
    /// Like `хуй`: 494 words.
    Хуй,
    /// Like `накладная`: 453 words.
    Накладная,
    /// Like `волчок`: 410 words.
    Волчок,
    /// Like `олимпиец`: 255 words.
    Олимпиец,
    /// Like `зелёный`: 220 words.
    Зелёный,
    /// Like `умелец`: 170 words.
    Умелец,
    /// Like `кормчий`: 110 words.
    Кормчий,
    /// Like `штрафной`: 65 words.
    Штрафной,
    /// Like `гусёк`: 38 words.
    Гусёк,
    /// Like `телепень`: 36 words.
    Телепень,
    /// Like `русский`: 33 words.
    Русский,
    /// Like `котище`: 26 words.
    Котище,
    /// Like `батько`: 21 words.
    Батько,
    /// Like `козёл`: 11 words.
    Козёл,
    /// Like `передняя`: 7 words.
    Передняя,
    /// Like `осётр`: 4 words.
    Осётр,
    /// Like `учащийся`: 4 words.
    Учащийся,
    /// Like `путы`: 4 words.
    Путы,
    /// Like `кто`: 3 words.
    Кто,
    /// Like `муравей`: 3 words.
    Муравей,
    /// Like `игры`: 3 words.
    Игры,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 27] = [
        Declension::Ядро,
        Declension::Полнота,
        Declension::Дух,
        Declension::Заря,
        Declension::Отец,
        Declension::Червь,
        Declension::Хуй,
        Declension::Накладная,
        Declension::Волчок,
        Declension::Олимпиец,
        Declension::Зелёный,
        Declension::Умелец,
        Declension::Кормчий,
        Declension::Штрафной,
        Declension::Гусёк,
        Declension::Телепень,
        Declension::Русский,
        Declension::Котище,
        Declension::Батько,
        Declension::Козёл,
        Declension::Передняя,
        Declension::Осётр,
        Declension::Учащийся,
        Declension::Путы,
        Declension::Кто,
        Declension::Муравей,
        Declension::Игры,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 27] = [
    // Ядро: 58,294 words (указ, зонт, палец, зонтик, бочаг, брашно).
    declension!("ядро", 58294, "~";
        nominative: "~";
        accusative: "~";
    ),
    // Полнота: 23,176 words (собака, кошка, буява, былица, быличка, масленица).
    declension!("полнота", 23176, "~а";
        nominative: "~а";
        accusative: "~у";
    ),
    // Дух: 13,610 words (кот, бортник, мэр, язык, лучник, любовник).
    declension!("дух", 13610, "~";
        nominative: "~";
        accusative: "~а";
    ),
    // Заря: 7,747 words (братыня, лесопильня, анархия, лесохимия, земля, неделя).
    declension!("заря", 7747, "~я";
        nominative: "~я";
        accusative: "~ю";
    ),
    // Отец: 2,128 words (отец, левантинец, древоточец, самозванец, разночинец, ньюйоркец).
    declension!("отец", 2128, "~е~";
        nominative: "~е~";
        accusative: "~~а";
    ),
    // Червь: 1,023 words (царь, олень, король, омуль, гость, пудель).
    declension!("червь", 1023, "~ь";
        nominative: "~ь";
        accusative: "~я";
    ),
    // Хуй: 494 words (гей, вертухай, хуй, гений, герой, попугай).
    declension!("хуй", 494, "~й";
        nominative: "~й";
        accusative: "~я";
    ),
    // Накладная: 453 words (деревообделочная, детская, гостиная, ванная, кладовая, прихожая).
    declension!("накладная", 453, "~ая";
        nominative: "~ая";
        accusative: "~ую";
    ),
    // Волчок: 410 words (щенок, бесёнок, подросток, ребёнок, телёнок, пастушок).
    declension!("волчок", 410, "~о~";
        nominative: "~о~";
        accusative: "~~а";
    ),
    // Олимпиец: 255 words (австриец, боец, австралиец, ариец, белогвардеец, китаец).
    declension!("олимпиец", 255, "~е~";
        nominative: "~е~";
        accusative: "~й~а";
    ),
    // Зелёный: 220 words (белый, чёрный, гласный, беспризорный, коридорный, правый).
    declension!("зелёный", 220, "~ый";
        nominative: "~ый";
        accusative: "~ого";
    ),
    // Умелец: 170 words (лев, телец, комсомолец, земледелец, бразилец, владелец).
    declension!("умелец", 170, "~е~";
        nominative: "~е~";
        accusative: "~ь~а";
    ),
    // Кормчий: 110 words (лесничий, рабочий, посторонний, нищий, военнослужащий, домашний).
    declension!("кормчий", 110, "~ий";
        nominative: "~ий";
        accusative: "~его";
    ),
    // Штрафной: 65 words (больной, голубой, часовой, вестовой, немой, глухой).
    declension!("штрафной", 65, "~й";
        nominative: "~й";
        accusative: "~го";
    ),
    // Гусёк: 38 words (мотылёк, хорёк, королёк, конёк, зверёк, малёк).
    declension!("гусёк", 38, "~ё~";
        nominative: "~ё~";
        accusative: "~ь~а";
    ),
    // Телепень: 36 words (оборотень, парень, пень, трутень, селезень, кремень).
    declension!("телепень", 36, "~е~ь";
        nominative: "~е~ь";
        accusative: "~~я";
    ),
    // Русский: 33 words (русский, полицейский, одинокий, голубенький, новенький, убогий).
    declension!("русский", 33, "~ий";
        nominative: "~ий";
        accusative: "~ого";
    ),
    // Котище: 26 words (дружище, волчище, козлище, человечище, мужчинище, мужичище).
    declension!("котище", 26, "~е";
        nominative: "~е";
        accusative: "~а|~у";
    ),
    // Батько: 21 words (сколько, столько, несколько, батько, соловейко, воронко).
    declension!("батько", 21, "~о";
        nominative: "~о";
        accusative: "~а";
    ),
    // Козёл: 11 words (пёс, козёл, орёл, осёл, бобёр, одёр).
    declension!("козёл", 11, "~ё~";
        nominative: "~ё~";
        accusative: "~~а";
    ),
    // Передняя: 7 words (передняя, преисподняя, ближняя, замужняя, незамужняя, несовершеннолетняя).
    declension!("передняя", 7, "~яя";
        nominative: "~яя";
        accusative: "~юю";
    ),
    // Осётр: 4 words (ксёндз, осётр, клёст, пётр).
    declension!("осётр", 4, "~ё~";
        nominative: "~ё~";
        accusative: "~е~а";
    ),
    // Учащийся: 4 words (учащийся, трудящийся, вольноопределяющийся, праздношатающийся).
    declension!("учащийся", 4, "~ий~";
        nominative: "~ий~";
        accusative: "~его~";
    ),
    // Путы: 4 words (путы, полминуты, полдюжины, буты).
    declension!("путы", 4, "~~ы";
        nominative: "~~ы";
        accusative: "~у~у";
    ),
    // Кто: 3 words (кто, кое-кто, никто).
    declension!("кто", 3, "~т~";
        nominative: "~т~";
        accusative: "~~го";
    ),
    // Муравей: 3 words (соловей, воробей, муравей).
    declension!("муравей", 3, "~ей";
        nominative: "~ей";
        accusative: "~ья";
    ),
    // Игры: 3 words (игры, гагры, пол-игры).
    declension!("игры", 3, "~ы";
        nominative: "~а";
        accusative: "~а";
    ),
];
