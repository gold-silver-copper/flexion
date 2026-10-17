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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
    /// or nothing, for its stem slot's (where [`Paradigm::stems`] states
    /// it), else the parent's (derived, in a paradigm like no other).
    pub table: Cow<'a, str>,
    /// Per stem slot of its part of speech (see [`crate::Slot`]), a tab
    /// after each: nothing, or the slot's stem as a pattern over the word's
    /// stem pieces and the ending table that builds the slot's cells on it
    /// (`~σ- like λυω`: the aorist of `λύω`'s stem `λυ` is `λυσ`, built as
    /// `λυω`'s table builds it). A cell its own table leaves empty is its
    /// slot's, where it states one.
    pub stems: Cow<'a, str>,
    /// The cell of its parent each of its cells reads where that is another
    /// (a deponent's active cells read as its parent's passive ones), by the
    /// cells' order; `None`: each the same cell.
    pub reads: Option<Cow<'a, [u16]>>,
    /// What it adds after each of its parent's patterns (Russian reflexives:
    /// `~я`, `~ь` after a vowel, the stem's last piece their `с`).
    pub affix: Option<Affix<'a>>,
}

/// What a paradigm adds after each of its parent's patterns: `pattern`, or
/// `other` where the parent's pattern ends in one of `letters`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Affix<'a> {
    pub pattern: Cow<'a, str>,
    pub other: Cow<'a, str>,
    pub letters: Cow<'a, str>,
}

impl Affix<'_> {
    /// `pattern` (its alternatives each) with the affix added.
    pub fn after(&self, pattern: &str) -> String {
        pattern
            .split('|')
            .map(|p| {
                let other = p
                    .chars()
                    .next_back()
                    .is_some_and(|c| self.letters.contains(c));
                format!("{p}{}", if other { &self.other } else { &self.pattern })
            })
            .collect::<Vec<_>>()
            .join("|")
    }
}

/// `pattern`, with the affixes met on the way up to it (innermost first)
/// added.
fn affixed<'s>(pattern: Cow<'s, str>, affixes: &[&Affix<'_>]) -> Cow<'s, str> {
    affixes.iter().fold(pattern, |p, a| Cow::Owned(a.after(&p)))
}

/// A part of speech's stem slots and ending tables, which paradigms state
/// slots with (see [`Paradigm::stems`]).
#[derive(Debug, Clone, Copy, Default)]
pub struct Space<'s, 'a> {
    pub slots: &'s [crate::Slot<'a>],
    pub tables: &'s [crate::EndingTable<'a>],
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

/// A cell whose form comes from another cell's form of the word: from which
/// cell (by the value of a feature of the word, when that decides), and how
/// the end of that form changes (Latin `amāvī` → `amāvistī`). Used where a
/// paradigm derives the cell, and where the word's form of that cell is
/// listed (a principal part) but this one isn't.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
    pub fn told(
        &self,
        form: &str,
        smoothing: f32,
        laws: Option<&sound_law::Program<'_>>,
    ) -> Option<(String, usize)> {
        self.apply(form, smoothing, laws)
            .into_iter()
            .max_by(|a, b| a.2.total_cmp(&b.2))
            .map(|(f, at, _)| (f, at))
    }

    /// The forms `form` gives (its letters), each with where its change
    /// starts and how likely it is: the change's new ending put where the
    /// old one was; one that writes a class letter (a capital, `~Im`) is
    /// run through `laws`, as a paradigm's form is.
    pub fn apply(
        &self,
        form: &str,
        smoothing: f32,
        laws: Option<&sound_law::Program<'_>>,
    ) -> Vec<(String, usize, f32)> {
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
                let (told, at) = match laws.filter(|_| new.chars().any(char::is_uppercase)) {
                    None => (
                        pattern::fill(new, &stems),
                        pattern::ending_start(new, &stems),
                    ),
                    Some(laws) => {
                        pattern::unmarked(&laws.apply(&pattern::fill_marked(new, &stems)))
                    }
                };
                Some((told, at, r.probability))
            })
            .collect();
        // No change, or none fits its ending: the form itself (as a guess).
        if out.is_empty() {
            return vec![(form.to_string(), form.len(), 1.0)];
        }
        out
    }

    /// The forms of the cell it comes from that it makes `form` of: a
    /// change's old ending put back where its new one is, where the rules
    /// would make that form `form` again (Latin `audiistī` from `audiī`
    /// through `-ī → -istī` is not one: an `-iī` perfect makes `audīstī`).
    pub fn sources(
        &self,
        form: &str,
        smoothing: f32,
        laws: Option<&sound_law::Program<'_>>,
    ) -> Vec<String> {
        if self.changes.is_empty() {
            return vec![form.to_string()];
        }
        let mut out: Vec<String> = Vec::new();
        for (old, new) in self.changes.iter() {
            let Some(pieces) = pattern::bind(new, form) else {
                continue;
            };
            let source = pattern::fill(old, &pieces);
            let again = self.told(&source, smoothing, laws);
            if again.is_some_and(|(again, _)| again == form) && !out.contains(&source) {
                out.push(source);
            }
        }
        out
    }
}

impl<'a> Paradigm<'a> {
    /// The stem pieces of a lemma (its letters) that can inflect this way,
    /// matching letters as `orthography` looks them up.
    pub fn stems<'w>(&self, orthography: &Orthography<'_>, lemma: &'w str) -> Option<Stems<'w>> {
        let o = orthography;
        pattern::bind_by(&self.lemma, lemma, |a, b| a == b || o.fold(a) == o.fold(b))
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
        form(self.table.split('\t').nth(cell)?)
    }

    /// What it states of `cell` through the cell's stem slot (see
    /// [`Paradigm::stems`]), if it states the slot.
    pub fn stated(&self, space: Space<'_, '_>, cell: usize) -> Option<String> {
        let (s, slot) = space
            .slots
            .iter()
            .enumerate()
            .find(|(_, slot)| slot.position(cell).is_some())?;
        let position = slot.position(cell)?;
        let entry = self.stems.split('\t').nth(s).filter(|e| !e.is_empty())?;
        let (stem, name) = crate::stems::listed_stem(entry)?;
        let table = space
            .tables
            .iter()
            .find(|t| usize::from(t.slot) == s && t.name == name)?;
        Some(crate::stems::compose(stem, table.pattern(position)?))
    }

    /// The cell's pattern (all its alternatives) as its own table and its
    /// parents' write it (`family`: its part of speech's paradigms), without
    /// what they state through stem slots (see [`Paradigm::pattern_in`]);
    /// `None` for a derived cell (and one no paradigm up the line writes).
    pub fn pattern<'s>(&'s self, family: &'s [Paradigm<'a>], cell: usize) -> Option<Cow<'s, str>> {
        up(self, family, cell, |_, p, cell| {
            p.own(cell).map(|f| f.pattern().map(Cow::Borrowed))
        })
    }

    /// The cell of its parent its `cell` reads (see [`Paradigm::reads`]).
    pub fn read(&self, cell: usize) -> usize {
        self.reads
            .as_ref()
            .and_then(|r| r.get(cell))
            .map_or(cell, |&c| usize::from(c))
    }

    /// The cell's pattern (all its alternatives): its own table's, else its
    /// stem slot's (in `space`), else its parent's (`family`: its part of
    /// speech's paradigms), and so on up the line; `None` for a derived cell
    /// (and one no paradigm up the line writes).
    pub fn pattern_in<'s>(
        &'s self,
        family: &'s [Paradigm<'a>],
        space: Space<'_, '_>,
        cell: usize,
    ) -> Option<Cow<'s, str>> {
        up(self, family, cell, |_, p, cell| match p.own(cell) {
            Some(f) => Some(f.pattern().map(Cow::Borrowed)),
            None => p.stated(space, cell).map(|s| Some(Cow::Owned(s))),
        })
    }

    /// The forms of `cell` for these stems, the usual one first, each with
    /// where its ending starts: the patterns filled, then `laws` applied at
    /// the stem's edge (see [`pattern::BOUNDARY`]) and to copied letters
    /// (see [`pattern::COPIED`]). None for a derived cell.
    pub fn built(
        &self,
        family: &[Paradigm<'a>],
        laws: Option<&sound_law::Program<'_>>,
        cell: usize,
        stems: &[&str],
    ) -> Vec<(String, usize)> {
        self.built_in(family, Space::default(), laws, cell, stems)
    }

    /// [`Paradigm::built`], with what paradigms state through the stem slots
    /// of `space` (see [`Paradigm::pattern_in`]).
    pub fn built_in(
        &self,
        family: &[Paradigm<'a>],
        space: Space<'_, '_>,
        laws: Option<&sound_law::Program<'_>>,
        cell: usize,
        stems: &[&str],
    ) -> Vec<(String, usize)> {
        self.pattern_in(family, space, cell)
            .map_or_else(Vec::new, |p| filled(&p, laws, stems))
    }
}

impl<'t> Form<'t> {
    /// Its pattern; `None` for a derived cell.
    fn pattern(self) -> Option<&'t str> {
        match self {
            Form::Pattern(pattern) => Some(pattern),
            Form::Derived => None,
        }
    }
}

/// What a paradigm's table writes in a cell: nothing (the parent's), a
/// pattern, or [`DERIVED`].
fn form(entry: &str) -> Option<Form<'_>> {
    match entry {
        "" => None,
        DERIVED => Some(Form::Derived),
        pattern => Some(Form::Pattern(pattern)),
    }
}

/// `pattern`'s forms (see [`Paradigm::built`]).
fn filled(
    pattern: &str,
    laws: Option<&sound_law::Program<'_>>,
    stems: &[&str],
) -> Vec<(String, usize)> {
    pattern::variants(pattern)
        .map(|v| match laws {
            None => (pattern::fill(v, stems), pattern::ending_start(v, stems)),
            Some(laws) => pattern::unmarked(&laws.apply(&pattern::fill_marked(v, stems))),
        })
        .collect()
}

/// The pattern of `cell` up `paradigm`'s line: `has` says what the
/// paradigm `level` steps up (the cell it reads there) has: a pattern,
/// `Some(None)` where it derives the cell, `None` where it is its parent's.
/// The affixes met on the way are added.
fn up<'s, 'a>(
    paradigm: &'s Paradigm<'a>,
    family: &'s [Paradigm<'a>],
    cell: usize,
    mut has: impl FnMut(usize, &'s Paradigm<'a>, usize) -> Option<Option<Cow<'s, str>>>,
) -> Option<Cow<'s, str>> {
    let (mut p, mut cell) = (paradigm, cell);
    let mut affixes: Vec<&Affix<'_>> = Vec::new();
    // Parents come before their children, so this ends.
    for level in 0..=family.len() {
        if let Some(found) = has(level, p, cell) {
            return found.map(|pattern| affixed(pattern, &affixes));
        }
        cell = p.read(cell);
        affixes.splice(0..0, p.affix.as_ref());
        p = family.get(usize::from(p.parent?))?;
    }
    None
}

/// A paradigm's line (it, the one it is like, and so on up), each table
/// and stated stem slot read once, the first time a cell needs it: what
/// [`Paradigm::pattern_in`] tells, for a word's many cells.
pub(crate) struct Tables<'s, 'a> {
    paradigm: &'s Paradigm<'a>,
    family: &'s [Paradigm<'a>],
    space: Space<'s, 'a>,
    /// Up the line, each paradigm read.
    levels: Vec<Level<'s>>,
    /// Per cell, its stem slot and its place among the slot's cells.
    slots: Option<Vec<Option<(usize, usize)>>>,
}

/// One paradigm of a line, read: its own table's cells, and per stem slot
/// it states, the stem and its ending table's patterns.
struct Level<'s> {
    own: Vec<&'s str>,
    stated: Vec<Option<(&'s str, Vec<&'s str>)>>,
}

impl<'s> Level<'s> {
    fn new(p: &'s Paradigm<'_>, space: Space<'s, '_>) -> Level<'s> {
        let mut stated: Vec<Option<(&str, Vec<&str>)>> = p
            .stems
            .split('\t')
            .enumerate()
            .map(|(s, entry)| {
                let (stem, name) = crate::stems::listed_stem(entry)?;
                let table = space
                    .tables
                    .iter()
                    .find(|t| usize::from(t.slot) == s && t.name == name)?;
                Some((stem, table.table.split('\t').collect()))
            })
            .collect();
        if stated.iter().all(Option::is_none) {
            stated.clear();
        }
        Level {
            own: p.table.split('\t').collect(),
            stated,
        }
    }
}

impl<'s, 'a> Tables<'s, 'a> {
    /// `paradigm`'s line in `family`, with the stem slots of `space`.
    pub fn new(
        paradigm: &'s Paradigm<'a>,
        family: &'s [Paradigm<'a>],
        space: Space<'s, 'a>,
    ) -> Tables<'s, 'a> {
        Tables {
            paradigm,
            family,
            space,
            levels: Vec::new(),
            slots: None,
        }
    }

    /// The stem slot `cell` is in, and its place among the slot's cells.
    pub fn slot(&mut self, cell: usize) -> Option<(usize, usize)> {
        slot(&mut self.slots, self.space, cell)
    }

    /// [`Paradigm::pattern_in`] of `cell`.
    pub fn pattern(&mut self, cell: usize) -> Option<Cow<'s, str>> {
        let Tables {
            paradigm,
            family,
            space,
            levels,
            slots,
        } = self;
        up(paradigm, family, cell, |level, p, cell| {
            if levels.len() == level {
                levels.push(Level::new(p, *space));
            }
            let read = levels.get(level)?;
            if let Some(f) = read.own.get(cell).copied().and_then(form) {
                return Some(f.pattern().map(Cow::Borrowed));
            }
            if read.stated.is_empty() {
                return None;
            }
            let (s, position) = slot(slots, *space, cell)?;
            let (stem, table) = read.stated.get(s)?.as_ref()?;
            let pattern = table.get(position).filter(|p| !p.is_empty())?;
            Some(Some(Cow::Owned(crate::stems::compose(stem, pattern))))
        })
    }

    /// [`Paradigm::built_in`] of `cell`.
    pub fn built(
        &mut self,
        laws: Option<&sound_law::Program<'_>>,
        cell: usize,
        stems: &[&str],
    ) -> Vec<(String, usize)> {
        self.pattern(cell)
            .map_or_else(Vec::new, |p| filled(&p, laws, stems))
    }
}

/// The stem slot `cell` is in (the first, as [`Paradigm::stated`] finds
/// it), and its place among the slot's cells: `slots`, every cell's, read
/// from `space` the first time.
fn slot(
    slots: &mut Option<Vec<Option<(usize, usize)>>>,
    space: Space<'_, '_>,
    cell: usize,
) -> Option<(usize, usize)> {
    let slots = slots.get_or_insert_with(|| {
        let mut out: Vec<Option<(usize, usize)>> = Vec::new();
        for (s, slot) in space.slots.iter().enumerate() {
            for (position, &c) in slot.cells.iter().enumerate() {
                let c = usize::from(c);
                if out.len() <= c {
                    out.resize(c + 1, None);
                }
                if let Some(at @ None) = out.get_mut(c) {
                    *at = Some((s, position));
                }
            }
        }
        out
    });
    slots.get(cell).copied().flatten()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::choice::{Rule, Side};

    #[test]
    fn a_source_is_a_form_the_relation_makes_the_given_one_of() {
        // The Latin perfect's second person from its first: `-ī → -istī`,
        // but `-iī → -īstī` after `i`.
        let relation = Relation {
            cell: 1,
            from: Cow::Borrowed(&[0]),
            changes: Cow::Owned(vec![
                (Cow::Borrowed("~ī"), Cow::Borrowed("~istī")),
                (Cow::Borrowed("~iī"), Cow::Borrowed("~īstī")),
            ]),
            change: Some(Choice {
                side: Side::End,
                rules: Cow::Owned(vec![
                    Rule {
                        ending: "".into(),
                        counts: Cow::Owned(vec![(0, 100)]),
                        regular: true,
                    },
                    Rule {
                        ending: "iī".into(),
                        counts: Cow::Owned(vec![(1, 30)]),
                        regular: true,
                    },
                ]),
                exceptions: Cow::Borrowed(&[]),
            }),
            listed: true,
            analogy: None,
        };
        let sources = |form| relation.sources(form, 2.0, None);
        assert_eq!(sources("amāvistī"), ["amāvī"]);
        assert_eq!(sources("audīstī"), ["audiī"]);
        // `audiī` fits `-ī → -istī`, but makes `audīstī`.
        assert!(sources("audiistī").is_empty());
    }
}
