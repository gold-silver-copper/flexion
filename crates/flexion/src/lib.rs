#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

mod choice;
mod grammar;
mod orthography;
mod paradigm;
pub mod pattern;
mod stress;

pub use choice::{Choice, Reading, Rule, Side, Status, by_ending, tolerance};
pub use grammar::{
    Compound, Grammar, Irregular, Known, Placement, Prefix, Prefixes, Separation, WORD_END,
    restore, sharpen,
};
pub use orthography::{Casing, Orthography, Stress, Written, recase};
pub use paradigm::{DERIVED, Form, Paradigm, Relation};
pub use sound_law;
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
    /// `книги (97%, regular): like книга; 1203 of 1210 words in -га`
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({:.0}%, {}): like {}",
            self.form,
            self.probability * 100.0,
            self.status,
            self.paradigm
        )?;
        if let Some(e) = self.evidence {
            write!(f, "; {e}")?;
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
        rules { $( $ending:literal => [ $( $value:tt $count:literal ),* $(,)? ] $(+ $rest:literal)? ),* $(,)? }
        listed { $( $word:literal => $listed:tt ),* $(,)? }
    ) => {
        $crate::choice!(@ $crate::Side::End; $( $ending => [ $( $value $count ),* ] (0 $(+ $rest)?) ),*; $( $word => $listed ),*)
    };
    (
        starts { $( $ending:literal => [ $( $value:tt $count:literal ),* $(,)? ] $(+ $rest:literal)? ),* $(,)? }
        listed { $( $word:literal => $listed:tt ),* $(,)? }
    ) => {
        $crate::choice!(@ $crate::Side::Start; $( $ending => [ $( $value $count ),* ] (0 $(+ $rest)?) ),*; $( $word => $listed ),*)
    };
    (@ $side:expr; $( $ending:literal => [ $( $value:tt $count:literal ),* ] ($($rest:tt)*) ),*; $( $word:literal => $listed:tt ),*) => {
        $crate::Choice {
            side: $side,
            rules: ::std::borrow::Cow::Borrowed(&[ $(
                $crate::Rule {
                    ending: ::std::borrow::Cow::Borrowed($ending),
                    counts: ::std::borrow::Cow::Borrowed(&[ $( ($value, $count) ),* ]),
                    rest: $($rest)*,
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

/// A cell of a paradigm table, as text: a pattern (`"~а"`), `derived` (see
/// [`Relation`]), or `_` for the parent's.
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
}
