#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

mod choice;
mod grammar;
mod orthography;
mod paradigm;
pub mod pattern;
mod stems;
mod stress;

pub use choice::{Choice, DISCOUNT, Reading, Rule, Side, Status, blend, by_ending, tolerates};
pub use grammar::{
    Compound, FRONT_LETTERS, Grammar, Heads, Irregular, Known, ListedStem, Marks, Placement,
    Prefix, Prefixes, READINGS, ROOT_LETTERS, Separation, Trust, WORD_END, restore, sharpen,
};
pub use orthography::{Casing, Orthography, Stress, Written, recase};
pub use paradigm::{Affix, DERIVED, Form, Paradigm, Relation, Space};
pub use sound_law;
#[doc(hidden)]
pub use sound_law_macros::compiled as __compiled_laws;

/// Sound laws in `sound_law`'s notation (the body
/// [`sound_law::sound_laws!`] reads), compiled when the crate that writes
/// them is built: a `sound_law::Program` constant. A law that doesn't
/// compile is a compile error.
///
/// ```
/// const LAWS: flexion::sound_law::Program<'static> = flexion::laws! {
///     classes {}
///     rules {
///         law0: "к+ы" => "к+и";
///     }
/// };
/// assert_eq!(LAWS.apply("рук+ы"), "рук+и");
/// ```
#[macro_export]
macro_rules! laws {
    ($($body:tt)*) => {
        $crate::__compiled_laws!($crate::sound_law; $($body)*)
    };
}
pub use stems::{EndingTable, LIKE, Slot, compose, factor};
pub use stress::{
    Accent, AccentPattern, Finals, Place, Prosody, StressClass, Syllable, place as stress_place,
    position as stress_position, positions as stress_positions,
};

use std::fmt;

/// A value a [`Choice`] gives: a paradigm, a stress pattern, a vowel.
/// Generated crates use their own enums (`Declension::Книга`); the trainer
/// uses plain numbers.
pub trait Label: Copy + Eq + fmt::Debug + 'static {
    /// The value's place in its grammar's table.
    fn index(self) -> usize;
    /// The value at a place in the table.
    fn from_index(index: usize) -> Option<Self>;
}

impl Label for u8 {
    fn index(self) -> usize {
        usize::from(self)
    }
    fn from_index(index: usize) -> Option<Self> {
        u8::try_from(index).ok()
    }
}

/// A letter: what [`Grammar::marks`](crate::Grammar::marks) gives.
impl Label for char {
    fn index(self) -> usize {
        self as usize
    }
    fn from_index(index: usize) -> Option<Self> {
        u32::try_from(index).ok().and_then(char::from_u32)
    }
}

impl Label for u16 {
    fn index(self) -> usize {
        usize::from(self)
    }
    fn from_index(index: usize) -> Option<Self> {
        u16::try_from(index).ok()
    }
}

/// One possible form, how likely it is, and why.
#[derive(Debug, Clone, PartialEq)]
pub struct Candidate<'g> {
    /// The form, written as the language writes it (stress marks included).
    pub form: String,
    /// Other forms the same reading allows in the cell (`ня́нею` beside
    /// `ня́ней`).
    pub also: Vec<String>,
    /// How likely this form is, from 0 to 1.
    pub probability: f32,
    /// Whether the dictionary lists the word, a productive rule gives it, or
    /// the words like it disagree.
    pub status: Status,
    /// The paradigm, named after its commonest word.
    pub paradigm: &'g str,
    /// The rule that chose the paradigm, when no listing did.
    pub evidence: Option<Evidence<'g>>,
    /// The listed stem it is built on, for a word that lists the stem of the
    /// cell's slot (`paradigm` is then the ending table's name).
    pub stem: Option<&'g str>,
}

/// The evidence behind a rule: of the dictionary's `total` words ending in
/// `ending`, `count` inflect this way.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Evidence<'g> {
    pub ending: &'g str,
    pub count: u32,
    pub total: u32,
}

impl fmt::Display for Evidence<'_> {
    /// `1203 of 1210 words in -га`
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} of {} words", self.count, self.total)?;
        if !self.ending.is_empty() {
            write!(f, " in -{}", self.ending)?;
        }
        Ok(())
    }
}

impl fmt::Display for Candidate<'_> {
    /// `книги (97%, regular): like книга; 1203 of 1210 words in -га`; a
    /// listed form: `adiūvistī (100%, listed)`; a form built on a listed
    /// stem: `ἤγγειλᾰ (100%, listed): like λυω; stem ἀγγειλ-`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({:.0}%, {})",
            self.form,
            self.probability * 100.0,
            self.status
        )?;
        if !self.paradigm.is_empty() {
            write!(f, ": like {}", self.paradigm)?;
        }
        if let Some(e) = self.evidence {
            write!(f, "; {e}")?;
        }
        if let Some(stem) = self.stem {
            write!(f, "; stem {stem}-")?;
        }
        Ok(())
    }
}

/// A name that names no value of a feature: `"ablative".parse::<Case>()`
/// in a language without an ablative.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnknownValue {
    /// The feature, `Case`.
    pub feature: &'static str,
    /// The name given.
    pub name: String,
}

impl fmt::Display for UnknownValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "no {} is called {:?}", self.feature, self.name)
    }
}

impl std::error::Error for UnknownValue {}

/// A [`Choice`] written as data: its rules (`ending => [value count, …]`,
/// sorted by ending read backwards; or `starts { beginning => … }`, sorted)
/// and its listed words (`word => value`, sorted).
///
/// ```
/// use flexion::{Choice, choice};
/// static GENDER: Choice<'static, u8> = choice! {
///     rules {
///         "" => [0 120, 1 80],
///         "а" => [1 75, 0 5],
///     }
///     listed {
///         "папа" => 0,
///     }
/// };
/// assert_eq!(GENDER.read("мама", 2.0, |_| true)[0].value, 1);
/// assert_eq!(GENDER.read("папа", 2.0, |_| true)[0].value, 0);
/// ```
#[macro_export]
macro_rules! choice {
    (
        rules { $( $ending:literal => [ $( $value:tt $count:literal ),* $(,)? ] $($regular:ident)? ),* $(,)? }
        listed { $( $word:literal => $listed:tt ),* $(,)? }
    ) => {
        $crate::choice!(@ $crate::Side::End; $( $ending => [ $( $value $count ),* ] ($($regular)?) ),*; $( $word => $listed ),*)
    };
    (
        starts { $( $ending:literal => [ $( $value:tt $count:literal ),* $(,)? ] $($regular:ident)? ),* $(,)? }
        listed { $( $word:literal => $listed:tt ),* $(,)? }
    ) => {
        $crate::choice!(@ $crate::Side::Start; $( $ending => [ $( $value $count ),* ] ($($regular)?) ),*; $( $word => $listed ),*)
    };
    (@regular regular) => { true };
    (@regular) => { false };
    (@ $side:expr; $( $ending:literal => [ $( $value:tt $count:literal ),* ] ($($regular:ident)?) ),*; $( $word:literal => $listed:tt ),*) => {
        $crate::Choice {
            side: $side,
            rules: ::std::borrow::Cow::Borrowed(&[ $(
                $crate::Rule {
                    ending: ::std::borrow::Cow::Borrowed($ending),
                    counts: ::std::borrow::Cow::Borrowed(&[ $( ($value, $count) ),* ]),
                    regular: $crate::choice!(@regular $($regular)?),
                }
            ),* ]),
            exceptions: ::std::borrow::Cow::Borrowed(&[ $(
                (::std::borrow::Cow::Borrowed($word), $listed)
            ),* ]),
        }
    };
}

/// A relation's changes written as data: `(old ending, new ending)`.
#[macro_export]
macro_rules! changes {
    ( $( ($old:literal, $new:literal) ),* $(,)? ) => {
        ::std::borrow::Cow::Borrowed(&[ $( (::std::borrow::Cow::Borrowed($old), ::std::borrow::Cow::Borrowed($new)) ),* ])
    };
}

/// Spellings written as data: `key => letters`, sorted by key.
#[macro_export]
macro_rules! spellings {
    ( $( $key:literal => $letters:literal ),* $(,)? ) => {
        &[ $( (::std::borrow::Cow::Borrowed($key), ::std::borrow::Cow::Borrowed($letters)) ),* ]
    };
}

/// Roots and how they write their letter third from the end, as
/// `Marks::roots` keeps them.
#[macro_export]
macro_rules! roots {
    ( $( $root:literal => $letter:literal ),* $(,)? ) => {
        &[ $( (::std::borrow::Cow::Borrowed($root), $letter) ),* ]
    };
}

/// A list of words, as `Heads::words` keeps them.
#[macro_export]
macro_rules! words {
    ( $( $word:literal ),* $(,)? ) => {
        &[ $( ::std::borrow::Cow::Borrowed($word) ),* ]
    };
}

/// A cell of a paradigm table, as text: a pattern (`"~а"`), `derived` (see
/// [`Relation`]), an accent's place (`Stem`), or `_` for the parent's.
#[macro_export]
macro_rules! cell {
    // `concat!` takes literals only: this is `DERIVED`.
    (derived) => {
        "="
    };
    (_) => {
        ""
    };
    ($pattern:literal) => {
        $pattern
    };
    ($place:ident) => {
        stringify!($place)
    };
}
