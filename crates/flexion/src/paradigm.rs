//! Paradigms: how a group of words inflects, one pattern per cell; and
//! referrals, cells whose form is another cell's.

use crate::pattern::{self, Stems};
use std::borrow::Cow;

/// One way of inflecting: the lemma's pattern, and a pattern per cell (see
/// [`crate::pattern`]). Named after its commonest word.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Paradigm<'a> {
    /// The commonest word that inflects this way (`книга`).
    pub name: Cow<'a, str>,
    /// How many of the dictionary's words inflect this way.
    pub words: u32,
    /// The lemma's pattern: which words can inflect this way at all
    /// (`~а` takes words in `-а`).
    pub lemma: Cow<'a, str>,
    /// What it has in each cell, in the part of speech's cell order.
    pub forms: Cow<'a, [Form<'a>]>,
}

/// What a paradigm has in a cell.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Form<'a> {
    /// A pattern (see [`crate::pattern`]): `~а`; alternatives after `|`.
    Pattern(Cow<'a, str>),
    /// Another cell's form, which one the word's [`Referral`] says: the
    /// accusative of `стол` is its nominative, of `кот` its genitive.
    Referred,
}

impl std::fmt::Display for Form<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Form::Pattern(p) => f.write_str(p),
            Form::Referred => f.write_str("referred"),
        }
    }
}

/// A cell whose form, in the paradigms that refer it, is another cell's,
/// which one depending on a feature of the word (its animacy).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Referral<'a> {
    pub cell: u16,
    /// Per value of the feature, in its order, the cell whose form it is.
    pub to: Cow<'a, [u16]>,
}

impl<'a> Paradigm<'a> {
    /// The stem pieces of a lemma that can inflect this way.
    pub fn stems<'w>(&self, lemma: &'w str) -> Option<Stems<'w>> {
        pattern::bind(&self.lemma, lemma)
    }

    /// The cell's pattern (all its alternatives); `None` for a referred
    /// cell.
    pub fn pattern(&self, cell: usize) -> Option<&str> {
        match self.forms.get(cell)? {
            Form::Pattern(p) => Some(p),
            Form::Referred => None,
        }
    }

    /// Whether the paradigm takes the cell's form from another cell.
    pub fn refers(&self, cell: usize) -> bool {
        self.forms.get(cell) == Some(&Form::Referred)
    }

    /// The forms of `cell` for these stems, the usual one first.
    pub fn forms(&self, cell: usize, stems: &[&str]) -> Vec<String> {
        self.pattern(cell)
            .map(|p| {
                pattern::variants(p)
                    .map(|v| pattern::fill(v, stems))
                    .collect()
            })
            .unwrap_or_default()
    }
}
