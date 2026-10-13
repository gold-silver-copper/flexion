//! Paradigms: how a group of words inflects, one pattern per cell; and
//! relations, cells whose form comes from another cell's.

use crate::choice::Choice;
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
    /// after each: a pattern (`~а`, alternatives after `|`), [`DERIVED`],
    /// or nothing, for the parent's (derived, in a paradigm like no other).
    pub table: Cow<'a, str>,
}

/// What a paradigm's table writes for a cell whose form comes from another
/// cell's (see [`Relation`]).
pub const DERIVED: &str = "=";

/// What a paradigm has in a cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Form<'t> {
    /// A pattern (see [`crate::pattern`]): `~а`; alternatives after `|`.
    Pattern(&'t str),
    /// Another cell's form, changed as the cell's [`Relation`] says: the
    /// accusative of `стол` is its nominative, of `кот` its genitive.
    Derived,
}

impl std::fmt::Display for Form<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Form::Pattern(p) => f.write_str(p),
            Form::Derived => f.write_str("derived"),
        }
    }
}

/// A cell whose form comes from another cell's form of the word: from which
/// cell (by the value of a feature of the word, when that decides), and how
/// the end of that form changes (Latin `amāvī` → `amāvistī`). Used where a
/// paradigm derives the cell, and where the word's form of that cell is
/// listed (a principal part) but this one isn't.
#[derive(Debug, Clone, PartialEq)]
pub struct Relation<'a> {
    pub cell: u16,
    /// The cell it comes from: one, or one per value of the feature (the
    /// Russian accusative is the genitive of an animate noun, the
    /// nominative of an inanimate one).
    pub from: Cow<'a, [u16]>,
    /// The changes of that form, as patterns `(old, new)` around the letters
    /// they keep (`~`): `("~ī", "~istī")`; the augment, `("~", "ἐ~")`.
    pub changes: Cow<'a, [(Cow<'a, str>, Cow<'a, str>)]>,
    /// Which change, read off that form's ending (its letters), of those
    /// fitting it with the longest beginning; `None`: the form itself.
    pub change: Option<Choice<'a, u16>>,
    /// It tells the form of a word whose form of the cell it comes from is
    /// listed (a principal part), rather than the word's paradigm.
    pub listed: bool,
    /// Few or no words have the cell: the change is the one cell `.0`'s form
    /// makes into cell `.1`'s, the two differing as `from` and `cell` do (a
    /// future subjunctive is the future made as the present subjunctive is
    /// made from the present indicative). Without changes of its own, it
    /// makes those of the relation telling `.1` from `.0`; with `.0` and
    /// `.1` the same, none.
    pub analogy: Option<(u16, u16)>,
}

impl Relation<'_> {
    /// The one cell it comes from, if it doesn't depend on a feature.
    pub fn source(&self) -> Option<usize> {
        match self.from.as_ref() {
            &[from] => Some(usize::from(from)),
            _ => None,
        }
    }

    /// The likeliest form `form` gives, and where its change starts.
    pub fn told(&self, form: &str, smoothing: f32) -> Option<(String, usize)> {
        self.apply(form, smoothing)
            .into_iter()
            .max_by(|a, b| a.2.total_cmp(&b.2))
            .map(|(f, at, _)| (f, at))
    }

    /// The forms `form` gives (its letters), each with where its change
    /// starts and how likely it is.
    pub fn apply(&self, form: &str, smoothing: f32) -> Vec<(String, usize, f32)> {
        let start = |p: &str| p.split('~').next().map_or(0, str::len);
        let fitting = |c: u16| {
            self.changes
                .get(usize::from(c))
                .filter(|(old, _)| pattern::bind(old, form).is_some())
                .map(|(old, _)| start(old))
        };
        // Of the changes fitting the form, those saying most of its start.
        let longest = (0..self.changes.len())
            .filter_map(|c| fitting(u16::try_from(c).ok()?))
            .max();
        let fits = |c: u16| fitting(c).is_some() && fitting(c) == longest;
        let readings = self
            .change
            .iter()
            .flat_map(|c| c.read(form, smoothing, fits));
        let out: Vec<(String, usize, f32)> = readings
            .filter_map(|r| {
                let (old, new) = self.changes.get(usize::from(r.value))?;
                let stems = pattern::bind(old, form)?;
                let at = pattern::ending_start(new, &stems);
                Some((pattern::fill(new, &stems), at, r.probability))
            })
            .collect();
        // No change, or none fits its ending: the form itself (as a guess).
        if out.is_empty() {
            return vec![(form.to_string(), form.len(), 1.0)];
        }
        out
    }
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

    /// What its own table has in `cell`; `None` where it is the parent's
    /// (at a paradigm like no other, derived).
    pub fn own(&self, cell: usize) -> Option<Form<'_>> {
        match self.table.split('\t').nth(cell)? {
            "" => None,
            DERIVED => Some(Form::Derived),
            pattern => Some(Form::Pattern(pattern)),
        }
    }

    /// What it has in `cell`, its own or its parent's (`family`: its part of
    /// speech's paradigms); a cell no paradigm up the line writes is derived.
    pub fn form<'s>(&'s self, family: &'s [Paradigm<'a>], cell: usize) -> Option<Form<'s>> {
        let mut paradigm = self;
        // Parents come before their children, so this ends.
        for _ in 0..=family.len() {
            if let Some(form) = paradigm.own(cell) {
                return Some(form);
            }
            match paradigm.parent {
                Some(p) => paradigm = family.get(usize::from(p))?,
                None => return Some(Form::Derived),
            }
        }
        None
    }

    /// The cell's pattern (all its alternatives); `None` for a derived
    /// cell.
    pub fn pattern<'s>(&'s self, family: &'s [Paradigm<'a>], cell: usize) -> Option<&'s str> {
        match self.form(family, cell)? {
            Form::Pattern(p) => Some(p),
            Form::Derived => None,
        }
    }

    /// Whether the paradigm takes the cell's form from another cell.
    pub fn derives(&self, family: &[Paradigm<'a>], cell: usize) -> bool {
        self.form(family, cell) == Some(Form::Derived)
    }

    /// The usual form of `cell` for these stems before any sound law, and
    /// where its ending starts; `None` for a derived cell.
    pub fn unlawed(
        &self,
        family: &[Paradigm<'a>],
        cell: usize,
        stems: &[&str],
    ) -> Option<(String, usize)> {
        let p = pattern::variants(self.pattern(family, cell)?).next()?;
        let marked = pattern::fill_marked(p, stems);
        let ending = marked.find(pattern::BOUNDARY).unwrap_or(0);
        Some((marked.replace(pattern::BOUNDARY, ""), ending))
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
