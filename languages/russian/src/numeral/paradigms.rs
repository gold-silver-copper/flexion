//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's feature says):
/// `nominative_masculine` (anim, inan)
/// `nominative_feminine` (anim, inan)
/// `nominative_neuter` (anim, inan)
/// `nominative_plural` (anim, inan)
/// `genitive_masculine` (anim, inan)
/// `genitive_feminine` (anim, inan)
/// `genitive_neuter` (anim, inan)
/// `genitive_plural` (anim, inan)
/// `dative_masculine` (anim, inan)
/// `dative_feminine` (anim, inan)
/// `dative_neuter` (anim, inan)
/// `dative_plural` (anim, inan)
/// `accusative_masculine` (anim, inan)
/// `accusative_feminine` (anim, inan)
/// `accusative_neuter` (anim, inan)
/// `accusative_plural` (anim, inan)
/// `instrumental_masculine` (anim, inan)
/// `instrumental_feminine` (anim, inan)
/// `instrumental_neuter` (anim, inan)
/// `instrumental_plural` (anim, inan)
/// `prepositional_masculine` (anim, inan)
/// `prepositional_feminine` (anim, inan)
/// `prepositional_neuter` (anim, inan)
/// `prepositional_plural` (anim, inan)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal;
     nominative_masculine: $c0:tt, $c1:tt;
     nominative_feminine: $c2:tt, $c3:tt;
     nominative_neuter: $c4:tt, $c5:tt;
     nominative_plural: $c6:tt, $c7:tt;
     genitive_masculine: $c8:tt, $c9:tt;
     genitive_feminine: $c10:tt, $c11:tt;
     genitive_neuter: $c12:tt, $c13:tt;
     genitive_plural: $c14:tt, $c15:tt;
     dative_masculine: $c16:tt, $c17:tt;
     dative_feminine: $c18:tt, $c19:tt;
     dative_neuter: $c20:tt, $c21:tt;
     dative_plural: $c22:tt, $c23:tt;
     accusative_masculine: $c24:tt, $c25:tt;
     accusative_feminine: $c26:tt, $c27:tt;
     accusative_neuter: $c28:tt, $c29:tt;
     accusative_plural: $c30:tt, $c31:tt;
     instrumental_masculine: $c32:tt, $c33:tt;
     instrumental_feminine: $c34:tt, $c35:tt;
     instrumental_neuter: $c36:tt, $c37:tt;
     instrumental_plural: $c38:tt, $c39:tt;
     prepositional_masculine: $c40:tt, $c41:tt;
     prepositional_feminine: $c42:tt, $c43:tt;
     prepositional_neuter: $c44:tt, $c45:tt;
     prepositional_plural: $c46:tt, $c47:tt;
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
                flexion::form!($c19),
                flexion::form!($c20),
                flexion::form!($c21),
                flexion::form!($c22),
                flexion::form!($c23),
                flexion::form!($c24),
                flexion::form!($c25),
                flexion::form!($c26),
                flexion::form!($c27),
                flexion::form!($c28),
                flexion::form!($c29),
                flexion::form!($c30),
                flexion::form!($c31),
                flexion::form!($c32),
                flexion::form!($c33),
                flexion::form!($c34),
                flexion::form!($c35),
                flexion::form!($c36),
                flexion::form!($c37),
                flexion::form!($c38),
                flexion::form!($c39),
                flexion::form!($c40),
                flexion::form!($c41),
                flexion::form!($c42),
                flexion::form!($c43),
                flexion::form!($c44),
                flexion::form!($c45),
                flexion::form!($c46),
                flexion::form!($c47),
            ]),
        }
    };
}

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `один`: 42 words.
    Один,
    /// Like `пять`: 19 words.
    Пять,
    /// Like `пятеро`: 7 words.
    Пятеро,
    /// Like `пятьсот`: 5 words.
    Пятьсот,
    /// Like `пятьдесят`: 4 words.
    Пятьдесят,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 5] = [
        Declension::Один,
        Declension::Пять,
        Declension::Пятеро,
        Declension::Пятьсот,
        Declension::Пятьдесят,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 5] = [
    // Один: 42 words (один, два, три, четыре, сто, сорок).
    declension!("один", 42, "~";
        nominative_masculine: "~", "~";
        nominative_feminine: "~", "~";
        nominative_neuter: "~", "~";
        nominative_plural: "~|~ы", "~|~ы";
        genitive_masculine: "~а", "~а";
        genitive_feminine: "~а", "~а";
        genitive_neuter: "~а", "~а";
        genitive_plural: "~а|~ов", "~а|~ов";
        dative_masculine: "~у", "~у";
        dative_feminine: "~у", "~у";
        dative_neuter: "~у", "~у";
        dative_plural: "~у|~ам", "~у|~ам";
        accusative_masculine: "~", "~";
        accusative_feminine: "~", "~";
        accusative_neuter: "~", "~";
        accusative_plural: "~|~ы", "~|~ы";
        instrumental_masculine: "~ом", "~ом";
        instrumental_feminine: "~ом", "~ом";
        instrumental_neuter: "~ом", "~ом";
        instrumental_plural: "~ом|~ами", "~ом|~ами";
        prepositional_masculine: "~е", "~е";
        prepositional_feminine: "~е", "~е";
        prepositional_neuter: "~е", "~е";
        prepositional_plural: "~е|~ах", "~е|~ах";
    ),
    // Пять: 19 words (шесть, пять, семь, восемь, девять, десять).
    declension!("пять", 19, "~ь";
        nominative_masculine: "~ь", "~ь";
        nominative_feminine: "~ь", "~ь";
        nominative_neuter: "~ь", "~ь";
        nominative_plural: "~ь", "~ь";
        genitive_masculine: "~и", "~и";
        genitive_feminine: "~и", "~и";
        genitive_neuter: "~и", "~и";
        genitive_plural: "~и", "~и";
        dative_masculine: "~и", "~и";
        dative_feminine: "~и", "~и";
        dative_neuter: "~и", "~и";
        dative_plural: "~и", "~и";
        accusative_masculine: "~ь", "~ь";
        accusative_feminine: "~ь", "~ь";
        accusative_neuter: "~ь", "~ь";
        accusative_plural: "~ь", "~ь";
        instrumental_masculine: "~ью", "~ью";
        instrumental_feminine: "~ью", "~ью";
        instrumental_neuter: "~ью", "~ью";
        instrumental_plural: "~ью", "~ью";
        prepositional_masculine: "~и", "~и";
        prepositional_feminine: "~и", "~и";
        prepositional_neuter: "~и", "~и";
        prepositional_plural: "~и", "~и";
    ),
    // Пятеро: 7 words (четверо, шестеро, семеро, восьмеро, десятеро, пятеро).
    declension!("пятеро", 7, "~о";
        nominative_masculine: "~", "~";
        nominative_feminine: "~", "~";
        nominative_neuter: "~", "~";
        nominative_plural: "~о", "~о";
        genitive_masculine: "~а", "~а";
        genitive_feminine: "~а", "~а";
        genitive_neuter: "~а", "~а";
        genitive_plural: "~ых", "~ых";
        dative_masculine: "~у", "~у";
        dative_feminine: "~у", "~у";
        dative_neuter: "~у", "~у";
        dative_plural: "~ым", "~ым";
        accusative_masculine: "~", "~";
        accusative_feminine: "~", "~";
        accusative_neuter: "~", "~";
        accusative_plural: "~ых", "~о";
        instrumental_masculine: "~ом", "~ом";
        instrumental_feminine: "~ом", "~ом";
        instrumental_neuter: "~ом", "~ом";
        instrumental_plural: "~ыми", "~ыми";
        prepositional_masculine: "~е", "~е";
        prepositional_feminine: "~е", "~е";
        prepositional_neuter: "~е", "~е";
        prepositional_plural: "~ых", "~ых";
    ),
    // Пятьсот: 5 words (пятьсот, шестьсот, семьсот, восемьсот, девятьсот).
    declension!("пятьсот", 5, "~ь~о~";
        nominative_masculine: "~ь~о~", "~ь~о~";
        nominative_feminine: "~ь~о~", "~ь~о~";
        nominative_neuter: "~ь~о~", "~ь~о~";
        nominative_plural: "~ь~о~", "~ь~о~";
        genitive_masculine: "~ь~о~", "~ь~о~";
        genitive_feminine: "~ь~о~", "~ь~о~";
        genitive_neuter: "~ь~о~", "~ь~о~";
        genitive_plural: "~и~о~|~и~~а", "~и~о~|~и~~а";
        dative_masculine: "~ь~о~", "~ь~о~";
        dative_feminine: "~ь~о~", "~ь~о~";
        dative_neuter: "~ь~о~", "~ь~о~";
        dative_plural: "~и~~ам", "~и~~ам";
        accusative_masculine: "~ь~о~", "~ь~о~";
        accusative_feminine: "~ь~о~", "~ь~о~";
        accusative_neuter: "~ь~о~", "~ь~о~";
        accusative_plural: "~ь~о~", "~ь~о~";
        instrumental_masculine: "~ь~о~", "~ь~о~";
        instrumental_feminine: "~ь~о~", "~ь~о~";
        instrumental_neuter: "~ь~о~", "~ь~о~";
        instrumental_plural: "~ью~~ами", "~ью~~ами";
        prepositional_masculine: "~ь~о~", "~ь~о~";
        prepositional_feminine: "~ь~о~", "~ь~о~";
        prepositional_neuter: "~ь~о~", "~ь~о~";
        prepositional_plural: "~и~~ах", "~и~~ах";
    ),
    // Пятьдесят: 4 words (пятьдесят, шестьдесят, семьдесят, восемьдесят).
    declension!("пятьдесят", 4, "~ь~";
        nominative_masculine: "~ь~", "~ь~";
        nominative_feminine: "~ь~", "~ь~";
        nominative_neuter: "~ь~", "~ь~";
        nominative_plural: "~ь~", "~ь~";
        genitive_masculine: "~и~и", "~и~и";
        genitive_feminine: "~и~и", "~и~и";
        genitive_neuter: "~и~и", "~и~и";
        genitive_plural: "~и~и", "~и~и";
        dative_masculine: "~и~и", "~и~и";
        dative_feminine: "~и~и", "~и~и";
        dative_neuter: "~и~и", "~и~и";
        dative_plural: "~и~и", "~и~и";
        accusative_masculine: "~ь~", "~ь~";
        accusative_feminine: "~ь~", "~ь~";
        accusative_neuter: "~ь~", "~ь~";
        accusative_plural: "~ь~", "~ь~";
        instrumental_masculine: "~ью~ью", "~ью~ью";
        instrumental_feminine: "~ью~ью", "~ью~ью";
        instrumental_neuter: "~ью~ью", "~ью~ью";
        instrumental_plural: "~ью~ью", "~ью~ью";
        prepositional_masculine: "~и~и", "~и~и";
        prepositional_feminine: "~и~и", "~и~и";
        prepositional_neuter: "~и~и", "~и~и";
        prepositional_plural: "~и~и", "~и~и";
    ),
];
