//! Stem slots and ending tables: a word whose forms no paradigm gives lists
//! a stem per slot rather than its forms (`ἀγγέλλω`, aorist `ἀγγειλ-`), and
//! each of the slot's cells is built from that stem and an ending table
//! paradigms share (Bonami & Boyé 2002, 2003).

use crate::pattern;
use std::borrow::Cow;

/// A stem slot: the cells a word builds on one of its stems (the aorist's
/// cells on `λυσ-`), learned from the words that place different stems in
/// different cells.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Slot<'a> {
    /// What every one of its cells' names has and no other cell's has all
    /// of (`aorist_pass`); else `lemma`, the slot of the lemma's cell, or
    /// `rest`.
    pub name: Cow<'a, str>,
    /// Its cells, in order.
    pub cells: Cow<'a, [u16]>,
}

impl Slot<'_> {
    /// Where `cell` is among its cells.
    pub fn position(&self, cell: usize) -> Option<usize> {
        let cell = u16::try_from(cell).ok()?;
        self.cells.binary_search(&cell).ok()
    }
}

/// An ending table: what each cell of a slot adds to the slot's stem, as a
/// pattern with one stem piece (`ἐ~ᾰ`: the augment, the stem, the ending),
/// shared by the paradigms that build the slot alike; named after its
/// commonest word.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EndingTable<'a> {
    /// The commonest word built with it (`λυω`).
    pub name: Cow<'a, str>,
    /// How many of the dictionary's words are built with it.
    pub words: u32,
    /// Its slot (its place among its part of speech's).
    pub slot: u16,
    /// Per cell of the slot, its pattern (alternatives after `|`), a tab
    /// after each; nothing where its paradigms tell the cell from another.
    pub table: Cow<'a, str>,
}

impl EndingTable<'_> {
    /// The pattern of the slot's cell at `position`, if the table has one.
    pub fn pattern(&self, position: usize) -> Option<&str> {
        self.table
            .split('\t')
            .nth(position)
            .filter(|p| !p.is_empty())
    }

    /// The forms the table builds at the slot's cell at `position` on
    /// `stem`, the usual first, each with where its ending starts, through
    /// `laws` (see [`pattern::fill_marked`]).
    pub fn build(
        &self,
        position: usize,
        stem: &str,
        laws: Option<&sound_law::Program<'_>>,
    ) -> Vec<(String, usize)> {
        self.pattern(position)
            .map_or_else(Vec::new, |p| build(p, stem, laws))
    }
}

/// The forms `pattern` (with one stem piece; alternatives after `|`) builds
/// on `stem`, the usual first, each with where its ending starts, through
/// `laws` (see [`pattern::fill_marked`]).
pub fn build(
    pattern: &str,
    stem: &str,
    laws: Option<&sound_law::Program<'_>>,
) -> Vec<(String, usize)> {
    pattern::variants(pattern)
        .map(|v| match laws {
            None => (pattern::fill(v, &[stem]), pattern::ending_start(v, &[stem])),
            Some(laws) => pattern::unmarked(&laws.apply(&pattern::fill_marked(v, &[stem]))),
        })
        .collect()
}

/// `pattern` (with one stem piece; alternatives after `|`) with `stem` (a
/// pattern over a word's stem pieces) in its stem piece's place: an ending
/// table's pattern and a paradigm's stem for its slot (`ἐ~ᾰ`, `~σ`) make the
/// paradigm's pattern (`ἐ~σᾰ`).
pub fn compose(stem: &str, pattern: &str) -> String {
    let variants: Vec<String> = pattern::variants(pattern)
        .map(|v| v.replacen('~', stem, 1))
        .collect();
    variants.join("|")
}

/// The stem `pattern` (with one stem piece) builds `form` on (letters),
/// through `laws`: what the pattern leaves of the form, or that with its
/// first letter `lemma`'s (a template before the stem may change the
/// stem's first letter: the augment's `ἀ` → `ἠ`).
pub fn stem_of(
    pattern: &str,
    form: &str,
    lemma: &str,
    laws: Option<&sound_law::Program<'_>>,
) -> Option<String> {
    let builds = |stem: &str| {
        build(pattern, stem, laws)
            .first()
            .is_some_and(|(f, _)| f == form)
    };
    for v in pattern::variants(pattern) {
        // The pattern as written, and without what comes before its stem.
        let bare = v.find('~').and_then(|at| v.get(at..)).unwrap_or(v);
        for p in [v, bare] {
            let Some(&[left]) = pattern::bind(p, form).as_deref() else {
                continue;
            };
            let initial = lemma.chars().next();
            let swapped = initial.map(|i| {
                let rest: String = left.chars().skip(1).collect();
                format!("{i}{rest}")
            });
            let tried = std::iter::once(left.to_string()).chain(swapped);
            if let Some(stem) = tried.into_iter().find(|s| builds(s)) {
                return Some(stem);
            }
        }
    }
    None
}

/// A paradigm's patterns over a slot's cells (`None` where it tells a cell
/// from another), factored into the slot's stem as a pattern over the
/// word's stem pieces (the pieces, what lies between them, and what every
/// ending begins with: `~σ` of `ἐ~σᾰ`, `~σω`) and an ending table: per
/// cell, its pattern on that stem (`ἐ~ᾰ`, `~ω`), alternatives after `|`, a
/// tab after each. `None` where no stem serves every cell.
pub fn factor(patterns: &[Option<&str>]) -> Option<(String, String)> {
    // Every alternative: its template, the stem's pieces with what lies
    // between them, and its ending.
    let mut core: Option<&str> = None;
    let mut split: Vec<Option<Vec<(&str, &str)>>> = Vec::with_capacity(patterns.len());
    for p in patterns {
        let Some(p) = p else {
            split.push(None);
            continue;
        };
        let mut alternatives = Vec::new();
        for v in pattern::variants(p) {
            let (first, last) = (v.find('~')?, v.rfind('~')?);
            let here = v.get(first..=last)?;
            if core.is_some_and(|c| c != here) {
                return None;
            }
            core = Some(here);
            alternatives.push((v.get(..first)?, v.get(last + 1..)?));
        }
        split.push(Some(alternatives));
    }
    let core = core?;
    // What every ending begins with joins the stem.
    let endings: Vec<&str> = split.iter().flatten().flatten().map(|e| e.1).collect();
    let first = endings.first()?;
    let mut shared = first.len();
    for e in &endings {
        shared = shared.min(
            first
                .char_indices()
                .zip(e.chars())
                .take_while(|((_, a), b)| a == b)
                .last()
                .map_or(0, |((i, a), _)| i + a.len_utf8()),
        );
    }
    let begins = first.get(..shared)?;
    let table: String = split
        .iter()
        .map(|cell| {
            let written = cell.as_ref().map_or(String::new(), |alternatives| {
                let parts: Vec<String> = alternatives
                    .iter()
                    .map(|(t, e)| format!("{t}~{}", e.get(shared..).unwrap_or("")))
                    .collect();
                parts.join("|")
            });
            format!("{written}\t")
        })
        .collect();
    Some((format!("{core}{begins}"), table))
}

/// What a listed word's line says of a slot: `(slot name, stem, table
/// name)`, written `slot` then `stem- like table` (see
/// [`crate::Irregular`]).
pub fn listed_stem(entry: &str) -> Option<(&str, &str)> {
    entry.split_once(LIKE)
}

/// Between a listed stem and its table's name in a listed word's line.
pub const LIKE: &str = "- like ";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_paradigms_slot_is_a_stem_and_an_ending_table() {
        // The aorist of `λύω`: `ἔλυσα`, `λύσω`; its stem keeps the `σ` every
        // ending begins with, and the table is what is left.
        let (stem, table) = factor(&[Some("ἐ~σα"), None, Some("~σω|~σωμι")]).expect("one stem");
        assert_eq!(stem, "~σ");
        assert_eq!(table, "ἐ~α\t\t~ω|~ωμι\t");
        // Cells whose stem pieces differ have no one stem.
        assert_eq!(factor(&[Some("~о~"), Some("~~а")]), None);
        // A listed stem builds the slot's cells; a form gives its stem back,
        // the stem's first letter the lemma's where the augment changed it.
        let t = EndingTable {
            name: Cow::Borrowed("λυω"),
            words: 1,
            slot: 0,
            table: Cow::Borrowed("ἐ~α\t\t~ω\t"),
        };
        assert_eq!(
            t.build(2, "ἀγγειλ", None),
            vec![("ἀγγειλω".to_string(), "ἀγγειλ".len())]
        );
        assert!(t.build(1, "ἀγγειλ", None).is_empty());
        let rules = [sound_law::Rule::new("augment", "ἐ‹ἀ", "ἠ")];
        let augment = sound_law::RuleSet::new(&[], &rules)
            .compile()
            .expect("a law");
        assert_eq!(
            t.build(0, "ἀγγειλ", Some(&augment))
                .first()
                .map(|f| f.0.as_str()),
            Some("ἠγγειλα")
        );
        assert_eq!(
            stem_of("ἐ~α", "ἠγγειλα", "ἀγγελλω", Some(&augment)).as_deref(),
            Some("ἀγγειλ")
        );
        assert_eq!(stem_of("ἐ~α", "ἠγγειλα", "ἀγγελλω", None), None);
    }

    #[test]
    fn a_paradigm_states_a_slot_with_a_stem_and_a_table() {
        let slots = [Slot {
            name: Cow::Borrowed("aorist"),
            cells: Cow::Borrowed(&[1, 2, 3]),
        }];
        let tables = [EndingTable {
            name: Cow::Borrowed("λυω"),
            words: 1,
            slot: 0,
            table: Cow::Borrowed("ἐ~α\t~ω|~ωμι\t\t"),
        }];
        let space = crate::Space {
            slots: &slots,
            tables: &tables,
        };
        let paradigm = |table: &'static str, stems: &'static str, parent| crate::Paradigm {
            name: Cow::Borrowed("λυω"),
            words: 1,
            lemma: Cow::Borrowed("~ω"),
            parent,
            inherent: Cow::Borrowed(&[]),
            table: Cow::Borrowed(table),
            stems: Cow::Borrowed(stems),
            reads: None,
            affix: None,
        };
        let family = [
            paradigm("~ω\tἐ~ξα\t~ξω\t~ξαι\t", "", None),
            // Like the first, but its aorist on `~σ` with `λυω`'s table; its
            // own cell wins, and a cell the table leaves out is its parent's.
            paradigm("\t\t~σω\t\t", "~σ- like λυω\t", Some(0)),
        ];
        let child = family.get(1).expect("a child");
        let at = |cell| {
            child
                .pattern_in(&family, space, cell)
                .map(|p| p.into_owned())
        };
        assert_eq!(at(1).as_deref(), Some("ἐ~σα"));
        assert_eq!(at(2).as_deref(), Some("~σω"));
        assert_eq!(at(3).as_deref(), Some("~ξαι"));
        assert_eq!(at(0).as_deref(), Some("~ω"));
        assert_eq!(compose("~σ", "~ω|~ωμι"), "~σω|~σωμι");
    }
}
