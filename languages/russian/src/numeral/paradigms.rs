//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (`[gender: Neuter]`, if
/// they share them), the declension it is like (if any), then a row of patterns
/// per line (`derived`: the form comes from another cell's, see
/// `relations.rs`; `_`: the form is the declension's it is like):
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
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident)?;
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
            parent: like!($($parent)?),
            inherent: ::std::borrow::Cow::Borrowed(&[]),
            table: ::std::borrow::Cow::Borrowed(concat!(
                flexion::cell!($c0), "\t",
                flexion::cell!($c1), "\t",
                flexion::cell!($c2), "\t",
                flexion::cell!($c3), "\t",
                flexion::cell!($c4), "\t",
                flexion::cell!($c5), "\t",
                flexion::cell!($c6), "\t",
                flexion::cell!($c7), "\t",
                flexion::cell!($c8), "\t",
                flexion::cell!($c9), "\t",
                flexion::cell!($c10), "\t",
                flexion::cell!($c11), "\t",
                flexion::cell!($c12), "\t",
                flexion::cell!($c13), "\t",
                flexion::cell!($c14), "\t",
                flexion::cell!($c15), "\t",
                flexion::cell!($c16), "\t",
                flexion::cell!($c17), "\t",
                flexion::cell!($c18), "\t",
                flexion::cell!($c19), "\t",
                flexion::cell!($c20), "\t",
                flexion::cell!($c21), "\t",
                flexion::cell!($c22), "\t",
                flexion::cell!($c23), "\t",
                flexion::cell!($c24), "\t",
                flexion::cell!($c25), "\t",
                flexion::cell!($c26), "\t",
                flexion::cell!($c27), "\t",
                flexion::cell!($c28), "\t",
                flexion::cell!($c29), "\t",
                flexion::cell!($c30), "\t",
                flexion::cell!($c31), "\t",
                flexion::cell!($c32), "\t",
                flexion::cell!($c33), "\t",
                flexion::cell!($c34), "\t",
                flexion::cell!($c35), "\t",
                flexion::cell!($c36), "\t",
                flexion::cell!($c37), "\t",
                flexion::cell!($c38), "\t",
                flexion::cell!($c39), "\t",
                flexion::cell!($c40), "\t",
                flexion::cell!($c41), "\t",
                flexion::cell!($c42), "\t",
                flexion::cell!($c43), "\t",
                flexion::cell!($c44), "\t",
                flexion::cell!($c45), "\t",
                flexion::cell!($c46), "\t",
                flexion::cell!($c47), "\t",
            )),
        }
    };
}

/// The declension a declension is like, by its name.
macro_rules! like {
    () => {
        None
    };
    ($parent:ident) => {
        Some(Declension::$parent as u16)
    };
}

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `один`: 61 words.
    Один,
    /// Like `пятеро`: 7 words.
    Пятеро,
    /// Like `пятьсот`: 5 words.
    Пятьсот,
    /// Like `пятьдесят`: 4 words.
    Пятьдесят,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 4] = [
        Declension::Один,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 4] = [
    // Один: 61 words (шесть, один, два, три, четыре, пять).
    declension!("один", 61, "~";
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
    // Пятеро: 7 words (четверо, шестеро, семеро, восьмеро, десятеро, пятеро).
    declension!("пятеро", 7, "~о";
        nominative_masculine: derived, "~о";
        nominative_feminine: derived, derived;
        nominative_neuter: derived, derived;
        nominative_plural: "~о", "~о";
        genitive_masculine: derived, derived;
        genitive_feminine: derived, derived;
        genitive_neuter: derived, derived;
        genitive_plural: "~ых", "~ых";
        dative_masculine: derived, derived;
        dative_feminine: derived, derived;
        dative_neuter: derived, derived;
        dative_plural: "~ым", "~ым";
        accusative_masculine: derived, derived;
        accusative_feminine: derived, derived;
        accusative_neuter: derived, derived;
        accusative_plural: "~ых", "~о";
        instrumental_masculine: derived, derived;
        instrumental_feminine: derived, derived;
        instrumental_neuter: derived, derived;
        instrumental_plural: "~ыми", "~ыми";
        prepositional_masculine: derived, derived;
        prepositional_feminine: derived, derived;
        prepositional_neuter: derived, derived;
        prepositional_plural: "~ых", "~ых";
    ),
    // Пятьсот: 5 words (пятьсот, шестьсот, семьсот, восемьсот, девятьсот).
    declension!("пятьсот", 5, "~ь~о~";
        nominative_masculine: derived, "~ь~о~";
        nominative_feminine: derived, derived;
        nominative_neuter: derived, derived;
        nominative_plural: "~ь~о~", "~ь~о~";
        genitive_masculine: derived, derived;
        genitive_feminine: derived, derived;
        genitive_neuter: derived, derived;
        genitive_plural: "~и~о~|~и~~а", "~и~о~|~и~~а";
        dative_masculine: derived, derived;
        dative_feminine: derived, derived;
        dative_neuter: derived, derived;
        dative_plural: "~и~~ам", "~и~~ам";
        accusative_masculine: derived, derived;
        accusative_feminine: derived, derived;
        accusative_neuter: derived, derived;
        accusative_plural: "~ь~о~", "~ь~о~";
        instrumental_masculine: derived, derived;
        instrumental_feminine: derived, derived;
        instrumental_neuter: derived, derived;
        instrumental_plural: "~ью~~ами", "~ью~~ами";
        prepositional_masculine: derived, derived;
        prepositional_feminine: derived, derived;
        prepositional_neuter: derived, derived;
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
