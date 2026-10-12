//! Principal parts: a part of speech whose forms are built on several stems,
//! each cited in a dictionary (Latin `amō, amāre, amāvī, amātum`; Greek
//! `λύω, λύσω, ἔλυσα, λέλυκα, λέλυμαι, ἐλύθην`). Each cell belongs to the
//! system of one part and is inflected from that part's form; a word's parts
//! are what the caller gives, else what the part's own grammar gives for its
//! lemma (which lists the dictionary's parts it doesn't predict).

use crate::grammar::Known;
use crate::{Candidate, Label, Status};

/// What inflects words: a [`crate::Grammar`], or [`Systems`] of them.
pub trait Inflect<F: Label> {
    /// How many cells a word has.
    fn cells(&self) -> usize;

    /// The forms of `cell` for `lemma`, likeliest first, given what the
    /// caller knows.
    fn candidates(&self, lemma: &str, cell: usize, known: &Known<'_, F>) -> Vec<Candidate<'_>>;

    /// The likeliest form of `cell` for `lemma` (the lemma itself when there
    /// is no reading of it).
    fn best(&self, lemma: &str, cell: usize, known: &Known<'_, F>) -> String {
        self.candidates(lemma, cell, known)
            .into_iter()
            .next()
            .map_or_else(|| lemma.to_string(), |c| c.form)
    }
}

/// A part of speech built on principal parts.
pub struct Systems<'a, F: Label> {
    /// How many cells a word has.
    pub cells: usize,
    /// Per principal part, the lemma's first: its system.
    pub parts: &'a [System<'a, F>],
    /// The power the candidates' probabilities are raised to (see
    /// [`crate::Grammar::sharpness`]).
    pub sharpness: f32,
}

/// One principal part's system.
pub struct System<'a, F: Label> {
    /// What the system is called (`aorist`).
    pub name: &'a str,
    /// The cell the part is; `None` for the lemma's own system.
    pub part: Option<u16>,
    /// The system's cells, in the part of speech's numbering; its grammar
    /// numbers them in this order.
    pub cells: &'a [u16],
    /// Gives the part (its one cell) of a lemma: the dictionary's, listed,
    /// or predicted.
    pub from_lemma: Option<&'a (dyn Inflect<F> + Sync)>,
    /// Inflects the system's cells from the part.
    pub grammar: &'a (dyn Inflect<F> + Sync),
}

/// Part forms kept when combining them with their systems' forms.
const KEEP: usize = 3;

impl<F: Label> Systems<'_, F> {
    /// The system `cell` belongs to, and the cell's place in it.
    fn system_of(&self, cell: usize) -> Option<(usize, &System<'_, F>, usize)> {
        let cell = u16::try_from(cell).ok()?;
        self.parts.iter().enumerate().find_map(|(k, s)| {
            let i = s.cells.iter().position(|&c| c == cell)?;
            Some((k, s, i))
        })
    }

    /// The forms of part `k` of `lemma`, likeliest first, with
    /// probabilities: given, or as the part's grammar gives it.
    pub fn part(&self, lemma: &str, k: usize, known: &Known<'_, F>) -> Vec<(String, f32, Status)> {
        let Some(system) = self.parts.get(k) else {
            return Vec::new();
        };
        let Some(cell) = system.part else {
            return vec![(lemma.to_string(), 1.0, Status::Listed)];
        };
        if let Some(&(_, form)) = known.forms.iter().find(|(c, _)| *c == usize::from(cell)) {
            return vec![(form.to_string(), 1.0, Status::Listed)];
        }
        let Some(predict) = system.from_lemma else {
            return Vec::new();
        };
        let lemma_only = Known {
            forms: &[],
            ..*known
        };
        predict
            .candidates(lemma, 0, &lemma_only)
            .into_iter()
            .take(KEEP)
            .map(|c| (c.form, c.probability, c.status))
            .collect()
    }
}

impl<F: Label> Inflect<F> for Systems<'_, F> {
    fn cells(&self) -> usize {
        self.cells
    }

    fn candidates(&self, lemma: &str, cell: usize, known: &Known<'_, F>) -> Vec<Candidate<'_>> {
        let Some((k, system, i)) = self.system_of(cell) else {
            return Vec::new();
        };
        // What the caller gives of this system's cells, in its numbering.
        let given: Vec<(usize, &str)> = known
            .forms
            .iter()
            .filter_map(|&(c, f)| {
                let c = u16::try_from(c).ok()?;
                Some((system.cells.iter().position(|&x| x == c)?, f))
            })
            .collect();
        let mut out: Vec<Candidate<'_>> = Vec::new();
        for (part, p, status) in self.part(lemma, k, known) {
            // A listed or given part is spelled in full.
            let inner = Known {
                forms: &given,
                full: known.full || status == Status::Listed,
                ..*known
            };
            for c in system.grammar.candidates(&part, i, &inner) {
                let probability = p * c.probability;
                let status = if status == Status::Listed {
                    c.status
                } else {
                    status.max(c.status)
                };
                match out.iter_mut().find(|o| o.form == c.form) {
                    Some(o) => o.probability += probability,
                    None => out.push(Candidate {
                        probability,
                        status,
                        ..c
                    }),
                }
            }
        }
        // No part to build on: the lemma, as if it were the part.
        if out.is_empty() {
            let inner = Known {
                forms: &given,
                ..*known
            };
            out = system
                .grammar
                .candidates(lemma, i, &inner)
                .into_iter()
                .map(|c| Candidate {
                    status: Status::Unsettled,
                    ..c
                })
                .collect();
        }
        crate::sharpen(out.iter_mut().map(|c| &mut c.probability), self.sharpness);
        out.sort_by(|a, b| b.probability.total_cmp(&a.probability));
        out
    }
}
