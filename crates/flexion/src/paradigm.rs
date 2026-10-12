//! Paradigms: how a group of words inflects, one pattern per cell; and
//! referrals, cells whose form is another cell's.

use crate::orthography::Orthography;
use crate::pattern::{self, Stems};
use std::borrow::Cow;

/// One way of inflecting: the lemma's pattern, and a pattern per cell (see
/// [`crate::pattern`]). Named after its commonest word. A paradigm can be
/// *like* another (its parent): the cells its own table leaves empty are the
/// parent's.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Paradigm<'a> {
    /// The commonest word that inflects this way (`книга`).
    pub name: Cow<'a, str>,
    /// How many of the dictionary's words inflect this way.
    pub words: u32,
    /// The lemma's pattern: which words can inflect this way at all
    /// (`~а` takes words in `-а`).
    pub lemma: Cow<'a, str>,
    /// The paradigm (its place among its part of speech's) this one is like.
    pub parent: Option<u16>,
    /// The values of its part of speech's inherent features its words have,
    /// as `(feature, value)` by their places: Sanskrit `साधन` is neuter.
    pub inherent: Cow<'a, [(u8, u8)]>,
    /// What it has in each cell, in the part of speech's cell order, a tab
    /// after each: a pattern (`~а`, alternatives after `|`), [`REFERRED`],
    /// or nothing, for the parent's.
    pub table: Cow<'a, str>,
}

/// What a paradigm's table writes for a referred cell.
pub const REFERRED: &str = "=";

/// What a paradigm has in a cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Form<'t> {
    /// A pattern (see [`crate::pattern`]): `~а`; alternatives after `|`.
    Pattern(&'t str),
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
    /// The stem pieces of a lemma (its letters) that can inflect this way,
    /// matching letters as `orthography` looks them up.
    pub fn stems<'w>(&self, orthography: &Orthography<'_>, lemma: &'w str) -> Option<Stems<'w>> {
        pattern::bind_by(&self.lemma, lemma, |a, b| orthography.same_letter(a, b))
    }

    /// Whether words with these inherent feature values (`(feature, value)`)
    /// can inflect this way.
    pub fn has(&self, values: &[(u8, u8)]) -> bool {
        values
            .iter()
            .all(|(f, v)| self.inherent.iter().all(|(g, w)| g != f || w == v))
    }

    /// What its own table has in `cell`; `None` where it is the parent's.
    pub fn own(&self, cell: usize) -> Option<Form<'_>> {
        match self.table.split('\t').nth(cell)? {
            "" => None,
            REFERRED => Some(Form::Referred),
            pattern => Some(Form::Pattern(pattern)),
        }
    }

    /// What it has in `cell`, its own or its parent's (`family`: its part of
    /// speech's paradigms).
    pub fn form<'s>(&'s self, family: &'s [Paradigm<'a>], cell: usize) -> Option<Form<'s>> {
        let mut paradigm = self;
        // Parents come before their children, so this ends.
        for _ in 0..=family.len() {
            if let Some(form) = paradigm.own(cell) {
                return Some(form);
            }
            paradigm = family.get(usize::from(paradigm.parent?))?;
        }
        None
    }

    /// The cell's pattern (all its alternatives); `None` for a referred
    /// cell.
    pub fn pattern<'s>(&'s self, family: &'s [Paradigm<'a>], cell: usize) -> Option<&'s str> {
        match self.form(family, cell)? {
            Form::Pattern(p) => Some(p),
            Form::Referred => None,
        }
    }

    /// Whether the paradigm takes the cell's form from another cell.
    pub fn refers(&self, family: &[Paradigm<'a>], cell: usize) -> bool {
        self.form(family, cell) == Some(Form::Referred)
    }

    /// The forms of `cell` for these stems, the usual one first, each with
    /// where its ending starts: the patterns filled, then `laws` applied at
    /// the stem's edge (see [`pattern::BOUNDARY`]).
    pub fn built(
        &self,
        family: &[Paradigm<'a>],
        laws: Option<&sound_law::Program<'_>>,
        cell: usize,
        stems: &[&str],
    ) -> Vec<(String, usize)> {
        let Some(p) = self.pattern(family, cell) else {
            return Vec::new();
        };
        pattern::variants(p)
            .map(|v| match laws {
                None => (pattern::fill(v, stems), pattern::ending_start(v, stems)),
                Some(laws) => {
                    let marked = laws.apply(&pattern::fill_marked(v, stems));
                    let ending = marked.find(pattern::BOUNDARY).unwrap_or(0);
                    (marked.replace(pattern::BOUNDARY, ""), ending)
                }
            })
            .collect()
    }
}
