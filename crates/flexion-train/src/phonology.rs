//! Learning a part of speech's sound laws from its edits.
//!
//! An edit is a string swap learned for every ending it occurs with, so one
//! process becomes many unrelated edits: the Polish locative `ręka → ręce`,
//! `noga → nodze`, `mucha → musze` are `-ka +ce`, `-ga +dze`, `-cha +sze`.
//! Those are one ending, `-a +e`, after a stem whose last segment changes by
//! one map (`k → c`, `g → dz`, `ch → sz`). The learner finds such regular
//! differences and rewrites the edits as the ending with a *trigger*
//! character, plus one rule per map that rewrites the segment before the
//! trigger: `-a +⟨T⟩e` with `{x:S} => {x|M} / _ ⟨T⟩`. Words of one class that
//! differed only by that segment now share their edits, and an unseen word
//! whose stem ends in a mapped segment gets the change.
//!
//! The rules are local (they rewrite a few letters next to the stem's edge,
//! the processes that are learnable from examples alone, Chandlee, Eyraud &
//! Heinz 2014), in three shapes:
//!
//! - **at the end**: the stem's last consonants, before the ending, into
//!   consonants (`k → c`, `t → ci` where a vowel letter marks softness);
//! - **across final consonants**: the stem's last vowel, before its final
//!   consonants and the ending (German umlaut, `Haus → Häuser`; only where
//!   stress isn't written, as it moves the vowel count of the stem);
//! - **at the start**: a prefix that copies the stem's first letter before a
//!   vowel (Greek reduplication, `λύω → λέλυκα`, `θύω → τέθυκα`).
//!
//! A map entry `X → Y` is kept when it is reliable where it could apply
//! (Albright & Hayes 2002): of the training pairs whose stem ends in `X`
//! before the ending, the share that change it to `Y`, by its lower
//! confidence bound, with the stems that keep `X` counted against it. A raw
//! edit is rewritten only if the rewritten edit and the rules give every
//! training pair of it exactly its form. Rules never touch the lemma cell or
//! the cells principal parts give, so nothing has to undo them.

use crate::edit::{Chooser, Stem};
use crate::extract::Word;
use crate::learn::{Shape, apply};
use flexion::tables::{EditSpec, PhonologySpec, RuleSpec};
use flexion::{Spelling, TRIGGER_FIRST, TRIGGER_LAST};
use std::collections::{BTreeMap, HashMap, HashSet};

/// Longest stem segment a rule rewrites, and longest replacement, in letters.
const MAX_SEGMENT: usize = 2;
const MAX_REPLACEMENT: usize = 3;
/// A map entry is kept when the lower confidence bound of its reliability
/// is at least this.
const MIN_CONFIDENCE: f64 = 0.5;
/// The bound's z (one-sided, about 84%).
const Z: f64 = 1.0;
/// Distinct words a map entry needs (a word's many forms count once).
const MIN_HITS: usize = 3;
/// Endings a map entry must be reliable under: a sound law recurs across
/// morphological contexts (Albright & Hayes's cross-context rules); a change
/// seen under one ending only is just that ending's edits.
const MIN_CONTEXTS: usize = 2;
/// Distinct segments a rule needs to be worth one: with fewer it merges no
/// edits.
const MIN_SEGMENTS: usize = 2;

/// Where a rule rewrites.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Shape3 {
    End,
    AcrossConsonants,
    Start,
}

/// A part of speech's learned sound laws.
pub struct Phonology {
    pub spec: PhonologySpec,
    program: flexion::sound_law::Program,
    /// Cells rules may rewrite.
    cells: Vec<bool>,
    /// Raw edit → rewritten, for the end shapes (whatever the lemma).
    suffix: HashMap<EditSpec, EditSpec>,
    /// (raw edit, the lemma's first letter) → rewritten, for the start shape.
    start: HashMap<(EditSpec, char), EditSpec>,
    /// One line per rule, for the report.
    pub described: Vec<String>,
}

impl Phonology {
    /// The edit to learn instead of `raw` for `lemma` in `cell`, if the
    /// rules rewrite it.
    pub fn factor(&self, cell: usize, lemma: &str, raw: &EditSpec) -> Option<EditSpec> {
        if !self.cells.get(cell).copied().unwrap_or(false) {
            return None;
        }
        if let Some(e) = self.suffix.get(raw) {
            return Some(e.clone());
        }
        let first = lemma.chars().next()?;
        self.start.get(&(raw.clone(), first)).cloned()
    }

    pub fn program(&self) -> &flexion::sound_law::Program {
        &self.program
    }
}

/// One (lemma, raw edit) observation's lemma, by index into the lemmas.
type Pairs = Vec<usize>;

/// How an end-shape decomposition reads a raw edit: the edit without the
/// segment (`front`, `prefix`, the ending's strip and add, stress, mark),
/// and the shape.
type Key = (Shape3, EditSpec);

/// Learn the sound laws of `words`' cells (all but `skip`) as `chooser`
/// reads their pairs. `None` when no rule earns a place.
pub fn learn(
    sp: &Spelling<'_>,
    shape: &Shape,
    words: &[Word],
    chooser: &Chooser,
    skip: &[usize],
) -> Option<Phonology> {
    let cells: Vec<bool> = (0..shape.cells)
        .map(|c| c != shape.lemma_cell && !skip.contains(&c) && !shape.unscored.contains(&c))
        .collect();
    // Every observation: raw edit → the lemmas it was seen with.
    let mut lemmas: Vec<String> = Vec::new();
    let mut lemma_index: HashMap<String, usize> = HashMap::new();
    let mut by_edit: HashMap<EditSpec, Pairs> = HashMap::new();
    let mut consonants: HashSet<char> = HashSet::new();
    for w in words {
        for lex in &w.lexemes {
            let letters = std::iter::once(&lex.full).chain(lex.cells.iter().flatten());
            for c in letters.flat_map(|w| w.chars()) {
                if c.is_alphabetic() && !sp.is_vowel(c) && !flexion::text::combining(c) {
                    consonants.insert(c);
                }
            }
            let li = *lemma_index.entry(lex.full.clone()).or_insert_with(|| {
                lemmas.push(lex.full.clone());
                lemmas.len().saturating_sub(1)
            });
            for (cell, forms) in lex.cells.iter().enumerate() {
                if !cells.get(cell).copied().unwrap_or(false) {
                    continue;
                }
                let stem = Stem {
                    letters: &lex.full,
                    stress: lex.lemma_stress,
                    kind: lex.lemma_kind,
                };
                for f in forms {
                    let e = chooser.edit(sp, stem, f, false);
                    by_edit.entry(e).or_default().push(li);
                }
            }
        }
    }
    // Distinct words per raw edit: hits and scope count words, not forms.
    let words_of: HashMap<&EditSpec, usize> = by_edit
        .iter()
        .map(|(e, pairs)| {
            let mut p = pairs.clone();
            p.sort_unstable();
            p.dedup();
            (e, p.len())
        })
        .collect();
    let stress = sp.has_stress();
    let vowels = |s: &str| sp.vowel_count(s);
    let all_vowels = |s: &[char]| !s.is_empty() && s.iter().all(|&c| sp.is_vowel(c));
    let is_consonant = |c: char| consonants.contains(&c);

    // End shapes: every way to read a raw edit as segment + ending.
    // key → segment → replacement → hits.
    let mut hits: HashMap<Key, BTreeMap<String, BTreeMap<String, usize>>> = HashMap::new();
    let mut reads: HashMap<EditSpec, Vec<(Key, String, String)>> = HashMap::new();
    for (e, pairs) in &by_edit {
        let s: Vec<char> = e.strip.chars().collect();
        let a: Vec<char> = e.add.chars().collect();
        let mut out = Vec::new();
        for i in 1..=MAX_SEGMENT.min(s.len()) {
            let (x, st) = s.split_at(i);
            for j in 0..=MAX_REPLACEMENT.min(a.len()) {
                let (y, at) = a.split_at(j);
                if x == y || (st.is_empty() && at.is_empty()) {
                    continue;
                }
                // A change of the stem's consonants into consonants (a
                // spelling may mark softness with a vowel letter after
                // them, Polish `t → ci`); vowel changes go across the final
                // consonants, and losing or gaining segments is what endings
                // do.
                if !x.iter().all(|&c| is_consonant(c))
                    || !y.first().is_some_and(|&c| is_consonant(c))
                {
                    continue;
                }
                let (x, y): (String, String) = (x.iter().collect(), y.iter().collect());
                if stress && vowels(&x) != vowels(&y) {
                    continue;
                }
                out.push(((Shape3::End, ending(e, st, at)), x, y));
            }
        }
        if !stress {
            // A vowel before final consonants: strip = X K st, add = Y K at.
            for i in 1..=2usize.min(s.len()) {
                let (x, rest) = s.split_at(i);
                if !all_vowels(x) {
                    continue;
                }
                let k_len = rest.iter().take_while(|&&c| is_consonant(c)).count();
                if k_len == 0 {
                    continue;
                }
                let (k, st) = rest.split_at(k_len);
                for j in 1..=2usize.min(a.len()) {
                    let (y, rest_a) = a.split_at(j);
                    if !all_vowels(y) || x == y || !rest_a.starts_with(k) {
                        continue;
                    }
                    let at = rest_a.get(k.len()..).unwrap_or(&[]);
                    out.push((
                        (Shape3::AcrossConsonants, ending(e, st, at)),
                        x.iter().collect(),
                        y.iter().collect(),
                    ));
                }
            }
        }
        let n = words_of.get(e).copied().unwrap_or(0);
        let _ = pairs;
        for (key, x, y) in &out {
            *hits
                .entry(key.clone())
                .or_default()
                .entry(x.clone())
                .or_default()
                .entry(y.clone())
                .or_default() += n;
        }
        reads.insert(e.clone(), out);
    }
    // Scope: the stems that keep the segment under the ending count against
    // the change.
    let mut keeps: HashMap<Key, BTreeMap<String, usize>> = HashMap::new();
    let promising = hits.iter().filter(|(_, segs)| {
        segs.values()
            .filter(|ys| ys.values().copied().max().unwrap_or(0) >= MIN_HITS)
            .count()
            >= MIN_SEGMENTS
    });
    let promising: Vec<Key> = promising.map(|(k, _)| k.clone()).collect();
    for key in &promising {
        let (shape3, ending) = key;
        let Some(pairs) = by_edit.get(ending) else {
            continue;
        };
        let segments: Vec<String> = hits
            .get(key)
            .map(|m| m.keys().cloned().collect())
            .unwrap_or_default();

        let counts = keeps.entry(key.clone()).or_default();
        let mut distinct = pairs.clone();
        distinct.sort_unstable();
        distinct.dedup();
        for &li in &distinct {
            let Some(lemma) = lemmas.get(li) else {
                continue;
            };
            let Some(end) = sp.folded_suffix_start(lemma, &ending.strip) else {
                continue;
            };
            let stem = lemma.get(..end).unwrap_or("");
            for x in &segments {
                let found = match shape3 {
                    Shape3::End => stem.ends_with(x.as_str()),
                    _ => {
                        let t = stem.trim_end_matches(|c: char| is_consonant(c));
                        t.len() < stem.len() && t.ends_with(x.as_str())
                    }
                };
                if found {
                    *counts.entry(x.clone()).or_default() += 1;
                }
            }
        }
    }
    // Each key's reliable entries, then only those reliable under several
    // endings.
    /// A key's reliable entries: `(segment, replacement, words)`.
    type Entries = Vec<(String, String, usize)>;
    let mut reliable: Vec<(Key, Entries)> = Vec::new();
    let mut contexts: HashMap<(Shape3, String, String), usize> = HashMap::new();
    for key in &promising {
        let Some(segs) = hits.get(key) else {
            continue;
        };
        let mut entries = Vec::new();
        for (x, ys) in segs {
            let Some((y, &n)) = ys.iter().max_by(|a, b| a.1.cmp(b.1).then(b.0.cmp(a.0))) else {
                continue;
            };
            let kept = keeps.get(key).and_then(|k| k.get(x)).copied().unwrap_or(0);
            let scope = ys.values().sum::<usize>().saturating_add(kept);
            if n >= MIN_HITS && lower_bound(n, scope) >= MIN_CONFIDENCE {
                *contexts.entry((key.0, x.clone(), y.clone())).or_default() += 1;
                entries.push((x.clone(), y.clone(), n));
            }
        }
        reliable.push((key.clone(), entries));
    }
    let mut maps: Vec<(Key, BTreeMap<String, String>, usize)> = Vec::new();
    for (key, entries) in reliable {
        let mut map = BTreeMap::new();
        let mut covered = 0usize;
        for (x, y, n) in entries {
            if contexts
                .get(&(key.0, x.clone(), y.clone()))
                .copied()
                .unwrap_or(0)
                >= MIN_CONTEXTS
            {
                map.insert(x, y);
                covered = covered.saturating_add(n);
            }
        }
        if map.len() >= MIN_SEGMENTS {
            maps.push((key, map, covered));
        }
    }
    maps.sort_by(|a, b| b.2.cmp(&a.2).then(a.0.cmp(&b.0)));
    // Each raw edit goes to the key that covers most pairs among those that
    // read it.
    let rank: HashMap<&Key, usize> = maps.iter().enumerate().map(|(i, m)| (&m.0, i)).collect();
    let mut assigned: BTreeMap<usize, Vec<(EditSpec, String)>> = BTreeMap::new();
    let mut sorted_edits: Vec<&EditSpec> = reads.keys().collect();
    sorted_edits.sort();
    for e in sorted_edits {
        let best = reads
            .get(e)
            .into_iter()
            .flatten()
            .filter_map(|(key, x, y)| {
                let r = *rank.get(key)?;
                let (_, map, _) = maps.get(r)?;
                (map.get(x) == Some(y)).then_some((r, x.clone()))
            });
        if let Some((r, x)) = best.min_by_key(|(r, _)| *r) {
            assigned.entry(r).or_default().push((e.clone(), x));
        }
    }

    // Start shape: a prefix that ends in a map of the lemma's first letter
    // and a vowel, after a fixed lead (the Greek pluperfect's augment,
    // `ἐλελύκη`). vowel → (first letter → copy) → hits, and the raw edits
    // read: `(edit, first letter, vowel, copy, lead)`.
    let mut start_hits: BTreeMap<char, BTreeMap<char, BTreeMap<String, usize>>> = BTreeMap::new();
    let mut start_reads: Vec<(EditSpec, char, char, String, String)> = Vec::new();
    for (e, pairs) in &by_edit {
        if !e.front.is_empty() {
            continue;
        }
        let p: Vec<char> = e.prefix.chars().collect();
        let Some((&v, rest)) = p.split_last() else {
            continue;
        };
        let copy_len = rest.iter().rev().take_while(|&&c| is_consonant(c)).count();
        if !sp.is_vowel(v) || copy_len == 0 {
            continue;
        }
        let (lead, copy) = rest.split_at(rest.len().saturating_sub(copy_len));
        let (lead, copy): (String, String) = (lead.iter().collect(), copy.iter().collect());
        let mut by_first: BTreeMap<char, usize> = BTreeMap::new();
        let mut distinct = pairs.clone();
        distinct.sort_unstable();
        distinct.dedup();
        for &li in &distinct {
            if let Some(c) = lemmas.get(li).and_then(|l| l.chars().next()) {
                *by_first.entry(c).or_default() += 1;
            }
        }
        for (c, n) in by_first {
            *start_hits
                .entry(v)
                .or_default()
                .entry(c)
                .or_default()
                .entry(copy.clone())
                .or_default() += n;
            start_reads.push((e.clone(), c, v, copy.clone(), lead.clone()));
        }
    }
    let mut start_maps: BTreeMap<char, BTreeMap<char, String>> = BTreeMap::new();
    for (v, firsts) in &start_hits {
        let mut map = BTreeMap::new();
        for (c, copies) in firsts {
            let Some((y, &n)) = copies.iter().max_by(|a, b| a.1.cmp(b.1).then(b.0.cmp(a.0))) else {
                continue;
            };
            let scope = copies.values().sum::<usize>();
            if n >= MIN_HITS && lower_bound(n, scope) >= MIN_CONFIDENCE {
                map.insert(*c, y.clone());
            }
        }
        if map.len() >= MIN_SEGMENTS {
            start_maps.insert(*v, map);
        }
    }

    // Triggers: one per map; maps of the same shape that never disagree
    // share one.
    let mut triggers: Vec<(Shape3, BTreeMap<String, String>, Option<char>)> = Vec::new();
    let mut trigger_of_rank: HashMap<usize, usize> = HashMap::new();
    for (r, edits) in &assigned {
        let Some((key, map, _)) = maps.get(*r) else {
            continue;
        };
        let used: BTreeMap<String, String> = edits
            .iter()
            .filter_map(|(_, x)| Some((x.clone(), map.get(x)?.clone())))
            .collect();
        if used.len() < MIN_SEGMENTS {
            continue;
        }
        let shared = triggers.iter().position(|(s, m, _)| {
            *s == key.0 && used.iter().all(|(x, y)| m.get(x).is_none_or(|z| z == y))
        });
        let t = match shared {
            Some(t) => {
                if let Some((_, m, _)) = triggers.get_mut(t) {
                    m.extend(used);
                }
                t
            }
            None => {
                triggers.push((key.0, used, None));
                triggers.len().saturating_sub(1)
            }
        };
        trigger_of_rank.insert(*r, t);
    }
    for (v, map) in &start_maps {
        let map: BTreeMap<String, String> = map
            .iter()
            .map(|(c, y)| (c.to_string(), y.clone()))
            .collect();
        triggers.push((Shape3::Start, map, Some(*v)));
    }
    let n_triggers = u32::from(TRIGGER_LAST).saturating_sub(u32::from(TRIGGER_FIRST));
    triggers.truncate(usize::try_from(n_triggers).unwrap_or(0));
    if triggers.is_empty() {
        return None;
    }
    let trigger_char = |t: usize| {
        u32::try_from(t)
            .ok()
            .and_then(|t| char::from_u32(u32::from(TRIGGER_FIRST).saturating_add(t)))
            .unwrap_or(TRIGGER_FIRST)
    };

    // The rewritten edits.
    let mut suffix: HashMap<EditSpec, EditSpec> = HashMap::new();
    for (r, edits) in &assigned {
        let (Some(&t), Some((key, _, _))) = (trigger_of_rank.get(r), maps.get(*r)) else {
            continue;
        };
        let (_, ending) = key;
        let tc = trigger_char(t);
        for (e, _) in edits {
            let mut f = ending.clone();
            f.add = format!("{tc}{}", ending.add);
            suffix.insert(e.clone(), f);
        }
    }
    let mut start: HashMap<(EditSpec, char), EditSpec> = HashMap::new();
    for (e, c, v, copy, lead) in &start_reads {
        let Some(t) = triggers
            .iter()
            .position(|(s, _, tv)| *s == Shape3::Start && *tv == Some(*v))
        else {
            continue;
        };
        if triggers.get(t).and_then(|(_, m, _)| m.get(&c.to_string())) != Some(copy) {
            continue;
        }
        let mut f = e.clone();
        f.prefix = format!("{lead}{}{v}", trigger_char(t));
        start.insert((e.clone(), *c), f);
    }

    let mut consonant_list: Vec<char> = consonants.into_iter().collect();
    consonant_list.sort_unstable();
    let build = |triggers: &[(Shape3, BTreeMap<String, String>, Option<char>)]| {
        rules(triggers, &consonant_list, &trigger_char)
    };
    let spec = build(&triggers);
    let program = spec.compile().ok()?;
    // Keep only rewrites that give every training pair exactly its form.
    let realizes = |raw: &EditSpec, factored: &EditSpec, lemma: &str| -> bool {
        let (Some((want, ws, wk)), Some((got, gs, gk))) = (
            apply(sp, lemma, raw, None, 0),
            apply(sp, lemma, factored, None, 0),
        ) else {
            return false;
        };
        flexion::realize(Some(&program), &got) == want && ws == gs && wk == gk
    };
    suffix.retain(|raw, factored| {
        by_edit.get(raw).is_some_and(|pairs| {
            pairs
                .iter()
                .all(|&li| lemmas.get(li).is_some_and(|l| realizes(raw, factored, l)))
        })
    });
    start.retain(|(raw, first), factored| {
        by_edit.get(raw).is_some_and(|pairs| {
            pairs.iter().all(|&li| {
                lemmas
                    .get(li)
                    .is_none_or(|l| !l.starts_with(*first) || realizes(raw, factored, l))
            })
        })
    });
    if suffix.is_empty() && start.is_empty() {
        return None;
    }
    // Describe what stayed.
    let mut described = Vec::new();
    for (t, (shape3, map, v)) in triggers.iter().enumerate() {
        let tc = trigger_char(t);
        let pairs: usize = suffix
            .iter()
            .filter(|(_, f)| f.add.starts_with(tc))
            .map(|(raw, _)| by_edit.get(raw).map_or(0, Vec::len))
            .sum::<usize>()
            .saturating_add(
                start
                    .iter()
                    .filter(|(_, f)| f.prefix.starts_with(tc))
                    .map(|((raw, first), _)| {
                        by_edit.get(raw).map_or(0, |p| {
                            p.iter()
                                .filter(|&&li| {
                                    lemmas.get(li).and_then(|l| l.chars().next()) == Some(*first)
                                })
                                .count()
                        })
                    })
                    .sum::<usize>(),
            );
        if pairs == 0 {
            continue;
        }
        let entries: Vec<String> = map.iter().map(|(x, y)| format!("{x}→{y}")).collect();
        let what = match (shape3, v) {
            (Shape3::End, _) => "the stem's last segment before the ending".to_string(),
            (Shape3::AcrossConsonants, _) => {
                "the stem's last vowel, across its final consonants".to_string()
            }
            (Shape3::Start, Some(v)) => format!("a copy of the stem's first letter, then {v}"),
            (Shape3::Start, None) => "the stem's start".to_string(),
        };
        described.push(format!(
            "{what}: {} ({pairs} training pairs)",
            entries.join(", ")
        ));
    }
    Some(Phonology {
        spec,
        program,
        cells,
        suffix,
        start,
        described,
    })
}

/// `e` with its strip and add replaced by the ending's.
fn ending(e: &EditSpec, st: &[char], at: &[char]) -> EditSpec {
    EditSpec {
        front: e.front.clone(),
        prefix: e.prefix.clone(),
        strip: st.iter().collect(),
        add: at.iter().collect(),
        stress: e.stress,
        mark: e.mark,
    }
}

/// The lower confidence bound of `hits` out of `scope` (Mikheev 1997, as
/// Albright & Hayes 2002 use it).
fn lower_bound(hits: usize, scope: usize) -> f64 {
    let n = scope.max(1) as f64;
    let p = (hits as f64 + 0.5) / (n + 1.0);
    p - Z * (p * (1.0 - p) / n).sqrt()
}

/// A pattern literal: `sound-law`'s metacharacters escaped.
fn literal(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        if matches!(c, '{' | '}' | '^' | '$' | '#' | '\\') {
            out.push('\\');
        }
        out.push(c);
    }
    out
}

/// The rules for `triggers`, the trigger cleanup last.
fn rules(
    triggers: &[(Shape3, BTreeMap<String, String>, Option<char>)],
    consonants: &[char],
    trigger_char: &dyn Fn(usize) -> char,
) -> PhonologySpec {
    let mut spec = PhonologySpec::default();
    let needs_consonants = triggers.iter().any(|t| t.0 == Shape3::AcrossConsonants);
    if needs_consonants && !consonants.is_empty() {
        spec.classes.push((
            "CONSONANT".into(),
            consonants.iter().map(char::to_string).collect(),
        ));
    }
    for (t, (shape3, map, v)) in triggers.iter().enumerate() {
        let tc = trigger_char(t).to_string();
        let class = format!("S{t}");
        let mapping = format!("M{t}");
        spec.classes
            .push((class.clone(), map.keys().cloned().collect()));
        spec.maps.push((
            mapping.clone(),
            map.iter().map(|(x, y)| (x.clone(), y.clone())).collect(),
        ));
        let rule = match (shape3, v) {
            (Shape3::End, _) => RuleSpec {
                name: format!("end_{t}"),
                target: format!("{{x:{class}}}"),
                replacement: format!("{{x|{mapping}}}"),
                left: String::new(),
                right: literal(&tc),
            },
            (Shape3::AcrossConsonants, _) => RuleSpec {
                name: format!("across_{t}"),
                target: format!("{{x:{class}}}"),
                replacement: format!("{{x|{mapping}}}"),
                left: String::new(),
                right: format!("{{k:CONSONANT*}}{}", literal(&tc)),
            },
            (Shape3::Start, v) => {
                let v = v.map(|v| literal(&v.to_string())).unwrap_or_default();
                RuleSpec {
                    name: format!("start_{t}"),
                    target: format!("{}{v}{{x:{class}}}", literal(&tc)),
                    replacement: format!("{{x|{mapping}}}{v}{{x}}"),
                    left: String::new(),
                    right: String::new(),
                }
            }
        };
        spec.rules.push(rule);
    }
    spec.classes.push((
        "TRIGGER".into(),
        (0..triggers.len())
            .map(|t| trigger_char(t).to_string())
            .collect(),
    ));
    spec.rules.push(RuleSpec {
        name: "cleanup".into(),
        target: "{t:TRIGGER}".into(),
        replacement: String::new(),
        left: String::new(),
        right: String::new(),
    });
    spec
}

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used, clippy::indexing_slicing)]

    use super::*;

    use crate::extract::{Lexeme, PosData};
    use flexion::SpellingStrings;

    fn lexeme(lemma: &str, forms: &[&str]) -> Lexeme {
        let mut cells: Vec<Vec<String>> = vec![vec![lemma.to_string()]];
        cells.extend(forms.iter().map(|f| vec![f.to_string()]));
        cells.push(vec![lemma.to_string()]);
        Lexeme {
            lemma: lemma.into(),
            base: lemma.into(),
            full: lemma.into(),
            key: lemma.into(),
            lemma_stress: None,
            lemma_kind: 0,
            cells,
            inherent: Vec::new(),
            weight: 1.0,
            order: 0,
            source: 0,
        }
    }

    /// Learn from `(lemma, forms)` with cells [lemma, forms…, citation].
    fn learn_from(entries: &[(&str, &[&str])]) -> (Phonology, Vec<EditSpec>) {
        let sp = Spelling::new(
            false,
            false,
            SpellingStrings {
                vowels: "aeiouyąęóεαιουηω",
                ..Default::default()
            },
        );
        let data = PosData {
            lexemes: entries.iter().map(|(l, f)| lexeme(l, f)).collect(),
            editions: Vec::new(),
        };
        let words = crate::extract::words(&data);
        let cells = entries.first().map_or(0, |(_, f)| f.len()) + 2;
        let shape = Shape {
            cells,
            inherent: 0,
            lemma_cell: cells - 1,
            groups: vec![cells],
            unscored: vec![0, cells - 1],
            optional: vec![false; cells],
            capitalized: false,
        };
        let chooser = Chooser::new(&sp, &words, shape.lemma_cell);
        let p = learn(&sp, &shape, &words, &chooser, &[]).expect("learns");
        // The raw edits of the first entry's forms, rewritten.
        let (lemma, forms) = entries[0];
        let stem = Stem {
            letters: lemma,
            stress: None,
            kind: 0,
        };
        let factored = forms
            .iter()
            .enumerate()
            .map(|(i, f)| {
                let raw = chooser.edit(&sp, stem, f, false);
                p.factor(i + 1, lemma, &raw).expect("rewritten")
            })
            .collect();
        (p, factored)
    }

    fn form(p: &Phonology, lemma: &str, e: &EditSpec) -> String {
        let sp = Spelling::new(false, false, SpellingStrings::default());
        let (built, _, _) = apply(&sp, lemma, e, None, 0).expect("applies");
        flexion::realize(Some(p.program()), &built).into_owned()
    }

    #[test]
    fn a_softening_map_is_learned_across_endings() {
        // Two endings that soften the stem's last consonant: k → c, g → dz.
        let (p, factored) = learn_from(&[
            ("ręka", &["ręce", "ręcem"]),
            ("rzeka", &["rzece", "rzecem"]),
            ("matka", &["matce", "matcem"]),
            ("noga", &["nodze", "nodzem"]),
            ("droga", &["drodze", "drodzem"]),
            ("waga", &["wadze", "wadzem"]),
            ("lipa", &["lipe", "lipem"]),
        ]);
        let laws = format!("{:?}", p.spec.maps);
        assert!(
            laws.contains("(\"k\", \"c\")") && laws.contains("(\"g\", \"dz\")"),
            "{laws}"
        );
        // One edit for every such word, unseen ones too.
        assert_eq!(form(&p, "mąka", &factored[0]), "mące");
        assert_eq!(form(&p, "mąka", &factored[1]), "mącem");
        assert_eq!(form(&p, "pogoga", &factored[0]), "pogodze");
    }

    #[test]
    fn reduplication_is_learned_with_its_map() {
        // A prefix that copies the first consonant before `e`; `f` copies
        // as `p`.
        let (p, factored) = learn_from(&[
            ("pato", &["pepatka"]),
            ("pilo", &["pepilka"]),
            ("pumo", &["pepumka"]),
            ("tako", &["tetakka"]),
            ("timo", &["tetimka"]),
            ("tuso", &["tetuska"]),
            ("fano", &["pefanka"]),
            ("fila", &["pefilka"]),
            ("fumo", &["pefumka"]),
        ]);
        assert_eq!(form(&p, "pato", &factored[0]), "pepatka");
        // Unseen words.
        assert_eq!(form(&p, "tero", &factored[0]), "teterka");
        assert_eq!(form(&p, "foro", &factored[0]), "peforka");
    }

    #[test]
    fn lower_bound_trusts_many_more_than_few() {
        assert!(lower_bound(990, 1000) > lower_bound(5, 5));
        assert!(lower_bound(1, 10) < MIN_CONFIDENCE);
    }

    #[test]
    fn rules_compile_and_rewrite_before_the_trigger() {
        let t = |i: usize| char::from_u32(u32::from(TRIGGER_FIRST) + i as u32).expect("trigger");
        let soften: BTreeMap<String, String> = [("k", "c"), ("g", "dz"), ("ch", "sz")]
            .into_iter()
            .map(|(a, b)| (a.to_string(), b.to_string()))
            .collect();
        let copy: BTreeMap<String, String> = [("λ", "λ"), ("θ", "τ")]
            .into_iter()
            .map(|(a, b)| (a.to_string(), b.to_string()))
            .collect();
        let spec = rules(
            &[
                (Shape3::End, soften, None),
                (Shape3::Start, copy, Some('ε')),
            ],
            &[],
            &t,
        );
        let p = spec.compile().expect("compiles");
        let (t0, t1) = (t(0), t(1));
        assert_eq!(p.apply(&format!("ręk{t0}e")), "ręce");
        assert_eq!(p.apply(&format!("nog{t0}e")), "nodze");
        assert_eq!(p.apply(&format!("much{t0}e")), "musze");
        // A stem the map doesn't name keeps its letters.
        assert_eq!(p.apply(&format!("lip{t0}e")), "lipe");
        assert_eq!(p.apply(&format!("{t1}ελυκα")), "λελυκα");
        assert_eq!(p.apply(&format!("{t1}εθυκα")), "τεθυκα");
    }
}
