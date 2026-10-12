//! Mobile stress (Russian): each word has a stress pattern saying, cell by
//! cell, whether the stem or the ending is stressed, and a stressed vowel
//! for when the stem is.

use crate::Label;
use crate::choice::Choice;
use crate::orthography::{Orthography, Stress};
use std::borrow::Cow;

/// Where a form is stressed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Place {
    /// On the stem: the word's own stressed vowel.
    Stem,
    /// On the ending: its first vowel, or the word's last vowel when the
    /// ending has none.
    Ending,
}

/// A stress pattern: per cell, stem or ending.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccentPattern<'a> {
    pub name: Cow<'a, str>,
    /// How many of the dictionary's words follow it.
    pub words: u32,
    /// One place per cell, in the part of speech's cell order.
    pub places: Cow<'a, [Place]>,
}

/// A stress class: a pattern, and which vowel a word of the class stresses
/// when it stresses its stem, counted from the lemma's last vowel (0 is the
/// last); `None` for a pattern that never stresses the stem.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct StressClass {
    /// Index into [`Accent::patterns`].
    pub pattern: u16,
    pub vowel: Option<u8>,
    /// How many of the dictionary's words are of this class.
    pub words: u32,
}

/// A part of speech's stress: its patterns, its classes (a pattern and a
/// stressed stem vowel), and which class a word is of.
#[derive(Debug, Clone, PartialEq)]
pub struct Accent<'a, A: Label> {
    pub patterns: Cow<'a, [AccentPattern<'a>]>,
    /// Indexed by `A`.
    pub classes: Cow<'a, [StressClass]>,
    pub class: Choice<'a, A>,
}

impl<'a, A: Label> Accent<'a, A> {
    pub fn class_of(&self, a: A) -> Option<&StressClass> {
        self.classes.get(a.index())
    }

    pub fn pattern_of(&self, class: &StressClass) -> Option<&AccentPattern<'a>> {
        self.patterns.get(usize::from(class.pattern))
    }
}

/// The stress of a form, `letters`, whose ending starts at byte `ending`:
/// on vowel `stem_vowel` (from the start) for [`Place::Stem`], on the
/// ending's first vowel for [`Place::Ending`]; the last vowel when the
/// place has none.
pub fn place(
    orthography: &Orthography<'_>,
    letters: &str,
    ending: usize,
    place: Place,
    stem_vowel: u8,
) -> Option<Stress> {
    let vowels: Vec<usize> = letters
        .char_indices()
        .filter(|&(_, c)| orthography.is_vowel(c))
        .map(|(i, _)| i)
        .collect();
    let last = vowels.len().checked_sub(1)?;
    let vowel = match place {
        Place::Stem => usize::from(stem_vowel).min(last),
        Place::Ending => vowels.iter().position(|&i| i >= ending).unwrap_or(last),
    };
    Some(Stress {
        vowel: u8::try_from(vowel).ok()?,
        kind: 0,
    })
}
