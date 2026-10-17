//! Harmony: a letter of an ending that paradigms of one shape write
//! differently (`-lar`, `-ler`) is written once, as a class letter (`-lAr`),
//! and a law fills it from the nearest letter of a tier before it, as the law
//! has written it: the value passes from the stem to the first suffix and on
//! from suffix to suffix (Koskenniemi 1983 p. 77; Belth 2023 Table 4).
//!
//! The classes come from the tables' alternations (Ermolaeva 2018 §3): the
//! letters paradigms of one shape write at one place of one cell's ending
//! (not those the lemma ends in: the word shows them), the widest set first.
//! A class's tier is learned by taking letters off the alphabet (Burness &
//! McMullin 2019 §5.3; Belth 2025 pp. 45–46): every letter before the class
//! letter starts on it, and the letter after which its value is least
//! decided leaves it, one at a time, while one isn't decided; a letter that
//! decides one (a trigger, or a blocker) stays. "Decided" is Yang's
//! tolerance, at most n/ln n exceptions in n (Belth 2024 eq. 14), counting
//! a word once (a loan that breaks harmony is one exception, not one per
//! form). A letter that decides only what a letter before it would leaves
//! too. The tier is learned on half the words and checked on the other half
//! (Rasin & Katzir 2020: what a law must not do, the words it is learned
//! from never pay for). A class is kept when its law decides it within the
//! tolerance on both halves, writes at least two of its letters, and is along
//! a tier: a letter further back than the one just before decides most of its
//! places (else it is a law at the stem's edge, Belth 2024 p. 8). Classes
//! whose values agree are one class letter; classes one tier decides are one
//! law,
//! written in one pass. A paradigm writes the class letter where the law
//! gives its words' letter, breaking few; a class that makes no two endings
//! one goes; paradigms then alike are one.
//!
//! A class whose value is the tier letter itself is a copy (the Finnish
//! illative `-Vn`: `taloon`, `kylään`, `maahan`), a law however near its
//! letter is (no law at the stem's edge states it once): it copies each of
//! its letters (every vowel, for one of vowels), is a law of its own,
//! written last (it reads the others' letters, `talossaan` of `~ssAVn`),
//! and is tried on the places writing just its letters where wider ones
//! fail (`-ssA` writes `a` or `ä` after any vowel).

use super::{BREAKS, Change, Law, MIN_WORDS, Word, held_out};
use crate::HashMap;
use flexion::{Orthography, Paradigm, pattern};
use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet};

/// The most alternations tried as classes.
const CANDIDATES: usize = 24;

/// A place a paradigm's ending writes a letter others of its shape don't:
/// the paradigm, the cell, and the letter's place in the ending.
type Spot = (usize, usize, usize);

/// Where a letter of a word's form is: the word, and its byte in the form
/// of the spot's cell.
type Example = (usize, usize);

/// A pattern before its ending (`~` and what comes before it), and its
/// ending's letters.
type Split = (String, Vec<char>);

/// A tier, and whether it is read only just before (see [`reads`]).
type Reading<'t> = (&'t BTreeSet<char>, bool);

/// A form up to a place, the letter there, and the word.
type Seen<'w> = (&'w str, char, usize);

/// Per tier letter before a place (or none), the letters seen there, each
/// counted once a word: a word that breaks a law breaks it once however
/// many of its forms show it (a lexical exception is one, `saat`, `saatler`,
/// `saatlerde`).
type Units = BTreeMap<Option<char>, BTreeMap<char, u32>>;

/// What `seen` shows on `tier` (see [`Units`]), read as [`reads`] says.
fn units(seen: &[Seen<'_>], tier: &BTreeSet<char>, adjacent: bool) -> Units {
    let mut distinct: Vec<(Option<char>, char, usize)> = seen
        .iter()
        .map(|(before, at, w)| (reads(before, tier, adjacent), *at, *w))
        .collect();
    distinct.sort_unstable();
    distinct.dedup();
    let mut by = Units::new();
    for (t, at, _) in distinct {
        *by.entry(t).or_default().entry(at).or_default() += 1;
    }
    by
}

/// The value seen most often after each tier letter (and none); where no
/// word has none, the value seen most often of all.
fn values(by: &Units) -> Option<BTreeMap<Option<char>, char>> {
    let mut total: BTreeMap<char, u32> = BTreeMap::new();
    for values in by.values() {
        for (&v, &n) in values {
            *total.entry(v).or_default() += n;
        }
    }
    let mut map: BTreeMap<Option<char>, char> = by
        .iter()
        .filter_map(|(t, values)| Some((*t, commonest(values)?)))
        .collect();
    if let std::collections::btree_map::Entry::Vacant(none) = map.entry(None) {
        none.insert(commonest(&total)?);
    }
    Some(map)
}

/// How many of `by`'s words `map` gets wrong, and how many it has.
fn judged(by: &Units, map: &BTreeMap<Option<char>, char>) -> (u32, u32) {
    let mut n = 0;
    let mut wrong = 0;
    for (t, values) in by {
        let v = map.get(t).or_else(|| map.get(&None));
        for (x, &k) in values {
            n += k;
            if v != Some(x) {
                wrong += k;
            }
        }
    }
    (n, wrong)
}

/// A learned class: its letters, its tier (read just before it where
/// `adjacent`), and its value after each tier letter and after none.
struct Class {
    letters: Vec<char>,
    tier: BTreeSet<char>,
    adjacent: bool,
    map: BTreeMap<Option<char>, char>,
    /// Its letters, commonest first.
    common: Vec<char>,
}

/// The nearest of `tier` before the end of `before`.
fn nearest(before: &str, tier: &BTreeSet<char>) -> Option<char> {
    before.chars().rev().find(|c| tier.contains(c))
}

/// The tier letter a class reads before the end of `before`: the nearest,
/// or where `adjacent` the letter just before if it is of the tier.
fn reads(before: &str, tier: &BTreeSet<char>, adjacent: bool) -> Option<char> {
    if adjacent {
        before.chars().next_back().filter(|c| tier.contains(c))
    } else {
        nearest(before, tier)
    }
}

impl Class {
    /// Its value after `before`.
    fn value(&self, before: &str) -> Option<char> {
        let trigger = reads(before, &self.tier, self.adjacent);
        self.map
            .get(&trigger)
            .or_else(|| self.map.get(&None))
            .copied()
    }
}

/// Of paradigms of one shape, by a cell, one of its other spellings (its
/// place among them), the pattern before its ending and the ending's
/// length: each paradigm's ending.
type Spellings<'p> = BTreeMap<(String, usize, usize, &'p str, usize), Vec<(usize, Vec<char>)>>;

/// The places of a cell's other spellings (not of its usual one's shape)
/// where paradigms of one shape write different letters (Finnish
/// `~selleen|~sellensa`, `~selleen|~sellensä`): by paradigm, cell,
/// spelling and place in its ending, the class of `chosen` that gives the
/// letter there, if one gives it on nearly all the words the paradigm
/// builds that spelling of (each word once). Not the letters the lemma
/// ends in.
fn other_spellings(
    orthography: &Orthography<'_>,
    words: &[Word<'_>],
    paradigms: &[Paradigm<'_>],
    members: &[Vec<usize>],
    chosen: &BTreeMap<Spot, (usize, u32)>,
    classes: &[Class],
) -> BTreeMap<(usize, usize, usize, usize), usize> {
    let used: BTreeSet<usize> = chosen.values().map(|(k, _)| *k).collect();
    // Per paradigm and cell, its own spellings but the usual one's shape.
    let mut groups: Spellings<'_> = BTreeMap::new();
    for (p, par) in paradigms.iter().enumerate() {
        let shape: String = (par.lemma.chars())
            .map(|c| {
                if c == '~' || c == pattern::COPY {
                    c
                } else {
                    '.'
                }
            })
            .collect();
        for (c, cell) in par.table.split('\t').enumerate() {
            let mut variants = pattern::variants(cell).map(|v| v.rsplit_once('~'));
            let Some(Some((head, end))) = variants.next() else {
                continue;
            };
            let first = (head, end.chars().count());
            for (i, v) in variants.enumerate() {
                let Some((h, e)) = v else { continue };
                let e: Vec<char> = e.chars().collect();
                if (h, e.len()) == first {
                    continue;
                }
                let key = (shape.clone(), c, i + 1, h, e.len());
                groups.entry(key).or_default().push((p, e));
            }
        }
    }
    let mut out = BTreeMap::new();
    for ((_, c, i, head, len), ps) in groups.iter().filter(|(_, ps)| ps.len() > 1) {
        for at in 0..*len {
            let mut set: Vec<char> = ps.iter().filter_map(|(_, e)| e.get(at).copied()).collect();
            set.sort_unstable();
            set.dedup();
            if set.len() < 2 {
                continue;
            }
            for (p, e) in ps {
                let (Some(&letter), Some(par)) = (e.get(at), paradigms.get(*p)) else {
                    continue;
                };
                // Not the letters the lemma ends in: the word shows them.
                let lemma: Vec<char> =
                    (par.lemma.rsplit_once('~')).map_or(Vec::new(), |(_, e)| e.chars().collect());
                if at < lemma.len() && e.starts_with(&lemma) {
                    continue;
                }
                let ending: String = e.iter().collect();
                let spelling = format!("{head}~{ending}");
                // The letters before the place in each word's form, once a
                // word.
                let befores: Vec<String> = members
                    .get(*p)
                    .into_iter()
                    .flatten()
                    .filter_map(|&w| {
                        let stems = par.stems(orthography, words.get(w)?.letters)?;
                        let form: Vec<char> = pattern::fill(&spelling, &stems).chars().collect();
                        let before = form.len().checked_sub(len - at)?;
                        Some(form.get(..before)?.iter().collect())
                    })
                    .collect();
                if befores.is_empty() {
                    continue;
                }
                let fits = used.iter().find(|&&k| {
                    let Some(class) = classes.get(k).filter(|cl| cl.letters.contains(&letter))
                    else {
                        return false;
                    };
                    let wrong = (befores.iter())
                        .filter(|b| class.value(b) != Some(letter))
                        .count();
                    #[allow(clippy::cast_precision_loss)]
                    let ok = wrong as f64 <= BREAKS * befores.len() as f64;
                    ok
                });
                if let Some(&k) = fits {
                    out.insert((*p, *c, *i, at), k);
                }
            }
        }
    }
    out
}

/// A class's examples: its letters, and what its places show on the words
/// it is learned from and those it is checked on (see [`held_out`]).
struct Pool<'w> {
    letters: Vec<char>,
    learned: Vec<Seen<'w>>,
    held: Vec<Seen<'w>>,
}

/// The harmony laws, and the paradigms with their class letters (those
/// then alike merged) and the assignment.
pub(super) fn learn<'s>(
    orthography: &Orthography<'_>,
    words: &[Word<'_>],
    paradigms: Vec<Paradigm<'s>>,
    assignment: Vec<u16>,
) -> (Vec<Law>, Vec<Paradigm<'s>>, Vec<u16>) {
    let vowel = &|c: char| orthography.is_vowel(c);
    let cells = paradigms
        .first()
        .map_or(0, |p| p.table.split('\t').count().saturating_sub(1));
    // Per paradigm and cell, its usual pattern, split before its ending.
    let usual: Vec<Vec<Option<Split>>> = paradigms
        .iter()
        .map(|p| {
            (0..cells)
                .map(|c| {
                    let pat = p.pattern(&paradigms, c)?;
                    let v = pattern::variants(&pat).next()?;
                    let (head, end) = v.rsplit_once('~')?;
                    Some((head.to_string(), end.chars().collect()))
                })
                .collect()
        })
        .collect();
    let mut members: Vec<Vec<usize>> = vec![Vec::new(); paradigms.len()];
    for (w, &p) in assignment.iter().enumerate() {
        if let Some(m) = members.get_mut(usize::from(p)) {
            m.push(w);
        }
    }
    // Paradigms of one shape (their lemmas' letters aside), by cell, head
    // and ending length.
    let shape = |lemma: &str| -> String {
        lemma
            .chars()
            .map(|c| {
                if c == '~' || c == pattern::COPY {
                    c
                } else {
                    '.'
                }
            })
            .collect()
    };
    let mut groups: BTreeMap<(String, usize, &str, usize), Vec<usize>> = BTreeMap::new();
    for (p, par) in paradigms.iter().enumerate() {
        for (c, u) in usual.get(p).into_iter().flatten().enumerate() {
            if let Some((head, end)) = u {
                groups
                    .entry((shape(&par.lemma), c, head, end.len()))
                    .or_default()
                    .push(p);
            }
        }
    }
    // The places where they differ, with the letters written there: not
    // the letters a paradigm's lemma ends in, which the word itself shows
    // (`~i` of `kedi`, `~isi`).
    let given = |p: usize, end: &[char], at: usize| {
        let lemma: Vec<char> = paradigms
            .get(p)
            .and_then(|par| par.lemma.rsplit_once('~'))
            .map_or(Vec::new(), |(_, e)| e.chars().collect());
        at < lemma.len() && end.starts_with(&lemma)
    };
    let mut spots: BTreeMap<Spot, Vec<char>> = BTreeMap::new();
    for (&(_, c, _, len), ps) in groups.iter().filter(|(_, ps)| ps.len() > 1) {
        for at in 0..len {
            let letters: Vec<(usize, char)> = ps
                .iter()
                .filter_map(|&p| {
                    let end = &usual.get(p)?.get(c)?.as_ref()?.1;
                    let letter = *end.get(at)?;
                    (!given(p, end, at)).then_some((p, letter))
                })
                .collect();
            let mut set: Vec<char> = letters.iter().map(|e| e.1).collect();
            set.sort_unstable();
            set.dedup();
            if set.len() > 1 {
                for &(p, _) in &letters {
                    spots.insert((p, c, at), set.clone());
                }
            }
        }
    }
    if spots.is_empty() {
        return (Vec::new(), paradigms, assignment);
    }
    // Each spot's examples: the words its paradigm builds that form of.
    let mut examples: BTreeMap<Spot, Vec<Example>> = BTreeMap::new();
    for (p, par) in paradigms.iter().enumerate() {
        let mine: Vec<&(usize, usize, usize)> = spots.keys().filter(|s| s.0 == p).collect();
        if mine.is_empty() {
            continue;
        }
        for &w in members.get(p).into_iter().flatten() {
            let Some(word) = words.get(w) else { continue };
            let Some(stems) = par.stems(orthography, word.letters) else {
                continue;
            };
            for &&(_, c, at) in &mine {
                let (Some(Some(form)), Some(Some((head, end)))) =
                    (word.forms.get(c), usual.get(p).and_then(|u| u.get(c)))
                else {
                    continue;
                };
                let ending: String = end.iter().collect();
                if pattern::fill(&format!("{head}~{ending}"), &stems) != *form {
                    continue;
                }
                let places: Vec<usize> = form.char_indices().map(|(i, _)| i).collect();
                let Some(&byte) = (places.len() + at)
                    .checked_sub(end.len())
                    .and_then(|k| places.get(k))
                else {
                    continue;
                };
                examples.entry((p, c, at)).or_default().push((w, byte));
            }
        }
    }
    let seen = |spot: &Spot, e: &Example| -> Option<Seen<'_>> {
        let form: &str = words.get(e.0)?.forms.get(spot.1)?.as_ref()?;
        let at = form.get(e.1..)?.chars().next()?;
        Some((form.get(..e.1)?, at, e.0))
    };
    // The alternations, those whose letters cover the most examples first:
    // a class is the widest set one law decides (`ı i u ü`, not `ı i`).
    let mut sizes: BTreeMap<&Vec<char>, usize> = BTreeMap::new();
    for (spot, letters) in &spots {
        *sizes.entry(letters).or_default() += examples.get(spot).map_or(0, Vec::len);
    }
    let mut candidates: Vec<(&Vec<char>, usize)> = sizes
        .keys()
        .map(|&set| {
            let within = sizes
                .iter()
                .filter(|(l, _)| l.iter().all(|c| set.contains(c)));
            (set, within.map(|(_, n)| n).sum())
        })
        .collect();
    candidates.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
    let mut pools: Vec<Pool<'_>> = Vec::new();
    // Whether each is a copy: a law of its own.
    let mut copying: Vec<bool> = Vec::new();
    let mut claimed: BTreeSet<Spot> = BTreeSet::new();
    for (letters, _) in candidates.into_iter().take(CANDIDATES) {
        // The places writing these letters or fewer; failing that, those
        // writing just these (a copy of every vowel, `-Vn`, is no law of the
        // places writing `a` or `ä` after any, `-ssA`).
        for exact in [false, true] {
            let pool: Vec<&Spot> = spots
                .iter()
                .filter(|(s, l)| {
                    !claimed.contains(*s)
                        && if exact {
                            *l == letters
                        } else {
                            l.iter().all(|c| letters.contains(c))
                        }
                })
                .map(|(s, _)| s)
                .collect();
            let mut users: Vec<usize> = pool
                .iter()
                .flat_map(|s| examples.get(s).into_iter().flatten().map(|e| e.0))
                .collect();
            users.sort_unstable();
            users.dedup();
            if users.len() < MIN_WORDS {
                continue;
            }
            // Learned on half the words, checked on the other half: a law
            // whose blocking letters only the words it was learned from pay
            // for fails on words it hasn't seen (Rasin & Katzir 2020).
            let mut candidate = Pool {
                letters: letters.clone(),
                learned: Vec::new(),
                held: Vec::new(),
            };
            for s in &pool {
                for e in examples.get(*s).into_iter().flatten() {
                    let (Some(x), Some(w)) = (seen(s, e), words.get(e.0)) else {
                        continue;
                    };
                    if held_out(w.letters) {
                        candidate.held.push(x);
                    } else {
                        candidate.learned.push(x);
                    }
                }
            }
            // Just these letters: only as a copy.
            let copy = fit(&[&candidate], vowel)
                .map(|(tier, adjacent)| copies(&candidate, &tier, adjacent));
            if copy.is_some_and(|copy| !exact || copy) {
                claimed.extend(pool.into_iter().copied());
                pools.push(candidate);
                copying.push(copy == Some(true));
                break;
            }
        }
    }
    // Classes one tier decides are one law, written in one pass: what one
    // writes the next reads (`-lArI`: `I` reads the `a` or `e` just written).
    let mut laws_of: Vec<Vec<usize>> = Vec::new();
    let copy = |k: &usize| copying.get(*k).copied().unwrap_or(false);
    for k in 0..pools.len() {
        let joins = laws_of.iter().position(|ks| {
            if copy(&k) || ks.iter().any(copy) {
                return false;
            }
            let together: Vec<&Pool<'_>> = ks
                .iter()
                .chain([&k])
                .filter_map(|&i| pools.get(i))
                .collect();
            fit(&together, vowel).is_some()
        });
        match joins.and_then(|j| laws_of.get_mut(j)) {
            Some(ks) => ks.push(k),
            None => laws_of.push(vec![k]),
        }
    }
    let mut vowels: Vec<char> = words
        .iter()
        .flat_map(|w| w.letters.chars())
        .filter(|&c| orthography.is_vowel(c))
        .collect();
    vowels.sort_unstable();
    vowels.dedup();
    let mut classes: Vec<Class> = Vec::new();
    for ks in &laws_of {
        let together: Vec<&Pool<'_>> = ks.iter().filter_map(|&i| pools.get(i)).collect();
        let Some((tier, adjacent)) = fit(&together, vowel) else {
            continue;
        };
        for pool in together {
            // Its values from all its words, on the tier learned.
            let all: Vec<Seen<'_>> = pool.learned.iter().chain(&pool.held).copied().collect();
            let by = units(&all, &tier, adjacent);
            let Some(mut map) = values(&by) else { continue };
            // A copy of vowels copies every vowel, those no ending shows it
            // after too.
            let mut tier = tier.clone();
            let mut letters = pool.letters.clone();
            if copies(pool, &tier, adjacent) {
                if letters.iter().all(|&c| orthography.is_vowel(c)) {
                    tier.extend(&vowels);
                    let more: Vec<char> = vowels
                        .iter()
                        .copied()
                        .filter(|v| !letters.contains(v))
                        .collect();
                    letters.extend(more);
                }
                map.extend(tier.iter().map(|&t| (Some(t), t)));
            }
            let mut total: BTreeMap<char, u32> = BTreeMap::new();
            for (&v, &n) in by.values().flat_map(|v| v.iter()) {
                *total.entry(v).or_default() += n;
            }
            let mut common: Vec<(u32, char)> = total.iter().map(|(&c, &n)| (n, c)).collect();
            common.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
            // A class whose values are another's on the same tier is that
            // class: one letter.
            let value = |m: &BTreeMap<Option<char>, char>, t: Option<char>| {
                m.get(&t).or_else(|| m.get(&None)).copied()
            };
            // Copies are one class whatever a word with no tier letter
            // takes (`-Vn`, `-ssAAn`: few words show it).
            let copied =
                |m: &BTreeMap<Option<char>, char>| m.iter().all(|(t, v)| t.is_none_or(|t| t == *v));
            let alike = |c: &Class| {
                let none = !(copied(&c.map) && copied(&map));
                let triggers = tier.iter().map(|&t| Some(t)).chain(none.then_some(None));
                c.tier == tier
                    && c.adjacent == adjacent
                    && triggers
                        .into_iter()
                        .all(|t| value(&c.map, t) == value(&map, t))
            };
            if let Some(same) = classes.iter_mut().find(|c| alike(c)) {
                for l in &letters {
                    if !same.letters.contains(l) {
                        same.letters.push(*l);
                    }
                }
                continue;
            }
            classes.push(Class {
                letters,
                tier,
                adjacent,
                map,
                common: common.into_iter().map(|(_, c)| c).collect(),
            });
        }
    }
    if classes.is_empty() {
        return (Vec::new(), paradigms, assignment);
    }
    // Each paradigm's spots its words' letters the law gives: with the
    // class, the forms it gets wrong there.
    let mut chosen: BTreeMap<Spot, (usize, u32)> = BTreeMap::new();
    for spot in spots.keys() {
        let Some(letter) = usual
            .get(spot.0)
            .and_then(|u| u.get(spot.1)?.as_ref()?.1.get(spot.2).copied())
        else {
            continue;
        };
        let es: Vec<Seen<'_>> = examples
            .get(spot)
            .into_iter()
            .flatten()
            .filter_map(|e| seen(spot, e))
            .collect();
        if es.is_empty() {
            continue;
        }
        let fits = classes.iter().enumerate().find_map(|(k, class)| {
            if !class.letters.contains(&letter) {
                return None;
            }
            let wrong = es
                .iter()
                .filter(|(before, at, _)| class.value(before) != Some(*at))
                .count();
            #[allow(clippy::cast_precision_loss)]
            let ok = wrong as f64 <= BREAKS * es.len() as f64;
            ok.then(|| (k, u32::try_from(wrong).unwrap_or(u32::MAX)))
        });
        if let Some(f) = fits {
            chosen.insert(*spot, f);
        }
    }
    // A class that makes no two endings of a shape one states nothing once:
    // it goes, and the others are counted again without it.
    loop {
        let mut credit: BTreeMap<usize, usize> = BTreeMap::new();
        for (&(_, c, _, _), ps) in &groups {
            let ends = |rewrite: bool| -> BTreeSet<Vec<char>> {
                ps.iter()
                    .filter_map(|&p| {
                        let mut end = usual.get(p)?.get(c)?.as_ref()?.1.clone();
                        if rewrite {
                            for (at, l) in end.iter_mut().enumerate() {
                                if let Some((k, _)) = chosen.get(&(p, c, at)) {
                                    *l = char::from_u32(0xE000 + u32::try_from(*k).ok()?)?;
                                }
                            }
                        }
                        Some(end)
                    })
                    .collect()
            };
            let one = ends(false).len().saturating_sub(ends(true).len());
            if one > 0 {
                let used: BTreeSet<usize> = ps
                    .iter()
                    .flat_map(|&p| chosen.range((p, c, 0)..(p, c + 1, 0)).map(|(_, (k, _))| *k))
                    .collect();
                for k in used {
                    *credit.entry(k).or_default() += one;
                }
            }
        }
        let used: BTreeSet<usize> = chosen.values().map(|(k, _)| *k).collect();
        let idle: Vec<usize> = used
            .into_iter()
            .filter(|k| !credit.contains_key(k))
            .collect();
        if idle.is_empty() {
            break;
        }
        chosen.retain(|_, (k, _)| !idle.contains(k));
    }
    // The class letters: a letter of the class written as a capital (the
    // commonest first), or another Latin capital, none a word writes. A
    // class with none left is no class.
    let mut used: BTreeSet<char> = words
        .iter()
        .flat_map(|w| {
            w.letters
                .chars()
                .chain(w.forms.iter().flatten().flat_map(|f| f.chars()))
        })
        .collect();
    // Copies are named after the other classes (those have their own
    // letters' capitals); a copy of the vowel before it is `V`, as grammars
    // write it.
    let copy = |class: &Class| class.map.iter().all(|(t, v)| t.is_none_or(|t| t == *v));
    let mut order: Vec<usize> = (0..classes.len()).collect();
    order.sort_by_key(|&k| classes.get(k).is_some_and(copy));
    let mut names: Vec<Option<char>> = vec![None; classes.len()];
    for k in order {
        let Some(class) = classes.get(k) else {
            continue;
        };
        let capitals = class.common.iter().filter_map(|&c| {
            let mut upper = c.to_uppercase();
            match (upper.next(), upper.next()) {
                (Some(u), None) if u != c => Some(u),
                _ => None,
            }
        });
        let vowels = copy(class) && class.letters.iter().all(|&c| orthography.is_vowel(c));
        let name = vowels
            .then_some('V')
            .into_iter()
            .chain(capitals)
            .chain('A'..='Z')
            .find(|c| !used.contains(c));
        if let (Some(name), Some(slot)) = (name, names.get_mut(k)) {
            used.insert(name);
            *slot = Some(name);
        }
    }
    chosen.retain(|_, (k, _)| names.get(*k).copied().flatten().is_some());
    let more = other_spellings(orthography, words, &paradigms, &members, &chosen, &classes);
    // The tables rewritten.
    let mut paradigms = paradigms;
    for (p, par) in paradigms.iter_mut().enumerate() {
        let mine: Vec<(&Spot, &(usize, u32))> = chosen.iter().filter(|(s, _)| s.0 == p).collect();
        if mine.is_empty() {
            continue;
        }
        let mut table: Vec<String> = par.table.split('\t').map(str::to_string).collect();
        for (c, cell) in table.iter_mut().enumerate() {
            let Some(Some((head, end))) = usual.get(p).and_then(|u| u.get(c)) else {
                continue;
            };
            let here: Vec<(usize, char, &Class)> = mine
                .iter()
                .filter(|(s, _)| s.1 == c)
                .filter_map(|(s, (k, _))| Some((s.2, (*names.get(*k)?)?, classes.get(*k)?)))
                .collect();
            let others = more.range((p, c, 0, 0)..(p, c + 1, 0, 0));
            if here.is_empty() && others.clone().next().is_none() {
                continue;
            }
            let rewritten: Vec<String> = pattern::variants(cell)
                .enumerate()
                .map(|(i, v)| {
                    let Some((h, e)) = v.rsplit_once('~') else {
                        return v.to_string();
                    };
                    let mut e: Vec<char> = e.chars().collect();
                    if h != head.as_str() || e.len() != end.len() {
                        // Another spelling: its own places.
                        for (&(_, _, _, at), k) in others.clone().filter(|(s, _)| s.2 == i) {
                            if let (Some(l), Some(Some(name))) = (e.get_mut(at), names.get(*k)) {
                                *l = *name;
                            }
                        }
                    } else {
                        for &(at, name, class) in &here {
                            if let Some(l) = e.get_mut(at).filter(|l| class.letters.contains(l)) {
                                *l = name;
                            }
                        }
                    }
                    format!("{h}~{}", e.into_iter().collect::<String>())
                })
                .collect();
            *cell = rewritten.join("|");
        }
        par.table = Cow::Owned(table.join("\t"));
    }
    // Paradigms now alike are one: the commoner.
    let mut first: HashMap<(String, String), u16> = HashMap::default();
    let mut into: Vec<u16> = Vec::with_capacity(paradigms.len());
    for (p, par) in paradigms.iter().enumerate() {
        let me = u16::try_from(p).unwrap_or(u16::MAX);
        let key = (par.lemma.to_string(), par.table.to_string());
        into.push(*first.entry(key).or_insert(me));
    }
    let assignment: Vec<u16> = assignment
        .iter()
        .map(|&a| into.get(usize::from(a)).copied().unwrap_or(a))
        .collect();
    let (paradigms, assignment) = super::drop_empty(paradigms, assignment);
    // One law per tier: its classes, with their evidence.
    let mut tiers: Vec<(Reading<'_>, Vec<usize>)> = Vec::new();
    for (k, class) in classes.iter().enumerate() {
        let tier = (&class.tier, class.adjacent);
        match tiers.iter_mut().find(|(t, _)| *t == tier) {
            Some((_, ks)) => ks.push(k),
            None => tiers.push((tier, vec![k])),
        }
    }
    let mut laws: Vec<Law> = tiers
        .into_iter()
        .filter_map(|((tier, adjacent), ks)| {
            let mine: Vec<(&Spot, &(usize, u32))> =
                chosen.iter().filter(|(_, (k, _))| ks.contains(k)).collect();
            if mine.is_empty() {
                return None;
            }
            let mut users: Vec<usize> = mine
                .iter()
                .flat_map(|(s, _)| examples.get(s).into_iter().flatten().map(|e| e.0))
                .collect();
            users.sort_unstable();
            users.dedup();
            let map = ks
                .iter()
                .filter_map(|&k| Some(((*names.get(k)?)?, classes.get(k)?)))
                .flat_map(|(name, class)| {
                    let triggers = tier.iter().map(|&t| Some(t)).chain([None]);
                    triggers.filter_map(move |t| {
                        let v = class.map.get(&t).or_else(|| class.map.get(&None))?;
                        Some((name, t, *v))
                    })
                })
                .collect();
            Some(Law {
                change: Change::Harmony {
                    tier: tier.iter().copied().collect(),
                    map,
                    adjacent,
                },
                words: u32::try_from(users.len()).unwrap_or(u32::MAX),
                examples: users
                    .iter()
                    .take(4)
                    .filter_map(|&w| words.get(w).map(|w| w.letters.to_string()))
                    .collect(),
                breaks: mine.iter().map(|(_, (_, b))| b).sum(),
            })
        })
        .collect();
    // A copy reads the letters before it as the other laws write them
    // (`~ssAVn`, `talossaan`): it comes last.
    let copies = |l: &Law| match &l.change {
        Change::Harmony { map, .. } => map.iter().all(|&(_, t, v)| t.is_none_or(|t| t == v)),
        _ => false,
    };
    laws.sort_by_key(copies);
    (laws, paradigms, assignment)
}

/// The value seen most often (the first in order of those as often).
fn commonest(values: &BTreeMap<char, u32>) -> Option<char> {
    values
        .iter()
        .max_by(|a, b| a.1.cmp(b.1).then(b.0.cmp(a.0)))
        .map(|(&v, _)| v)
}

/// Whether `exceptions` in `n` are few enough to state a rule over them
/// (Yang's tolerance: at most n/ln n; with fewer than three, none).
#[allow(clippy::cast_precision_loss)]
pub(super) fn tolerated(n: u32, exceptions: u32) -> bool {
    if n < 3 {
        exceptions == 0
    } else {
        f64::from(exceptions) <= f64::from(n) / f64::from(n).ln()
    }
}

/// Whether `pool`'s class is, on `tier`, the tier letter it reads itself
/// (of two tier letters at least).
fn copies(pool: &Pool<'_>, tier: &BTreeSet<char>, adjacent: bool) -> bool {
    values(&units(&pool.learned, tier, adjacent)).is_some_and(|map| {
        let mut tiered = map.iter().filter_map(|(t, v)| Some(((*t)?, *v)));
        tiered.clone().count() > 1 && tiered.all(|(t, v)| t == v)
    })
}

/// The tier one law decides `pools`' classes on, and whether it reads only
/// the letter just before them, learned on their words not held out:
/// `None` unless it decides every class within the tolerance on the words
/// learned from and on those held out, writes at least two of each class's
/// letters, and is not local (see the module documentation).
///
/// A class of vowels (`vowel` says which letters are) reads vowels: its
/// tier starts from the vowels and the marks between words (a space, a
/// hyphen) its words show, not from every letter, since on enough words a
/// consonant passes for a trigger by chance (Turkish verbs' `b`, `d`, `g`,
/// `v`, which took the place of `e` and `ö`). A consonant comes back only as
/// a blocker its words show deciding: on enough words, within the
/// tolerance, and with fewer wrong than without it on the words learned
/// from and no more on those held out. A copy of consonants (English `S`,
/// `quizzes`) reads only the letter just before it: what it copies is the
/// stem's last letter, never one further back (`foxproofs`).
fn fit(pools: &[&Pool<'_>], vowel: &dyn Fn(char) -> bool) -> Option<(BTreeSet<char>, bool)> {
    let vowels = pools.iter().all(|p| p.letters.iter().all(|&c| vowel(c)));
    let seen: BTreeSet<char> = pools
        .iter()
        .flat_map(|p| p.learned.iter().flat_map(|s| s.0.chars()))
        .collect();
    let mut tier: BTreeSet<char> = seen
        .iter()
        .copied()
        .filter(|&c| !vowels || vowel(c) || !c.is_alphabetic())
        .collect();
    let after = |tier: &BTreeSet<char>| -> Vec<Units> {
        pools
            .iter()
            .map(|p| units(&p.learned, tier, false))
            .collect()
    };
    // The letter whose value after it is least decided: of those not
    // tolerated, for any class, the one with the most exceptions (Belth's
    // learner deletes the letters that mispredict). One at a time, since
    // what a letter decides shows only once the letters between it and the
    // class letter are off the tier.
    let undecided = |bys: &[Units]| -> Option<char> {
        bys.iter()
            .flat_map(|by| by.iter())
            .filter_map(|(t, values)| {
                let n: u32 = values.values().sum();
                let most = values.values().max().copied().unwrap_or(0);
                (!tolerated(n, n - most)).then_some((n - most, (*t)?))
            })
            .max_by(|a, b| a.0.cmp(&b.0).then(b.1.cmp(&a.1)))
            .map(|(_, t)| t)
    };
    let mut local = true;
    while let Some(off) = undecided(&after(&tier)) {
        local = false;
        tier.remove(&off);
    }
    // A class that is the nearest tier letter itself (a copy: the Finnish
    // illative `-Vn`, `taloon`, `kylään`, `maahan`) is a law however near
    // that letter is (no law at the stem's edge states it once), and copies
    // each of its letters, those few words show too.
    let copy = pools.iter().all(|p| copies(p, &tier, false));
    // A copy is a law of its own: no other class's letters are its tier.
    if copy && pools.len() > 1 {
        return None;
    }
    let adjacent = copy && pools.iter().all(|p| p.letters.iter().all(|&c| !vowel(c)));
    if copy {
        tier = pools
            .iter()
            .flat_map(|p| p.letters.iter().copied())
            .collect();
    }
    // A letter that decides what a letter before it would decide anyway
    // leaves too (`j` of `garaj`, after `a`): the smallest tier that decides
    // as well (the seldom first). Not one whose words, with none before it,
    // would change what a word with none takes (`o` of `kol`, where few
    // words show the default).
    let wrong = |bys: &[Units]| -> u32 {
        bys.iter()
            .flat_map(|by| by.values())
            .map(|v| v.values().sum::<u32>() - v.values().max().copied().unwrap_or(0))
            .sum()
    };
    let default = |bys: &[Units]| -> Vec<Option<char>> {
        bys.iter()
            .map(|by| by.get(&None).and_then(commonest))
            .collect()
    };
    let bys = after(&tier);
    let mut order: Vec<(u32, char)> = tier
        .iter()
        .map(|&t| {
            let n = bys
                .iter()
                .filter_map(|by| by.get(&Some(t)))
                .flat_map(|v| v.values());
            (n.sum(), t)
        })
        .collect();
    order.sort_unstable();
    let mut errors = wrong(&bys);
    let mut none = default(&bys);
    for (_, t) in order.into_iter().filter(|_| !copy) {
        let mut fewer = tier.clone();
        fewer.remove(&t);
        let bys = after(&fewer);
        let now = default(&bys);
        let kept = none.iter().zip(&now).all(|(a, b)| a.is_none() || a == b);
        if !kept {
            continue;
        }
        let e = wrong(&bys);
        if e <= errors {
            (tier, errors, none) = (fewer, e, now);
        }
    }
    // The blockers: consonants back on a tier of vowels where their words
    // show them deciding (the commonest first).
    if vowels && !copy {
        let held = |tier: &BTreeSet<char>| -> u32 {
            pools
                .iter()
                .filter_map(|p| {
                    let map = values(&units(&p.learned, tier, false))?;
                    Some(judged(&units(&p.held, tier, false), &map).1)
                })
                .sum()
        };
        let mut blockers: Vec<(u32, char)> = seen
            .iter()
            .filter(|&&c| !tier.contains(&c))
            .map(|&c| {
                let mut more = tier.clone();
                more.insert(c);
                let n = after(&more)
                    .iter()
                    .filter_map(|by| by.get(&Some(c)))
                    .flat_map(|v| v.values())
                    .sum::<u32>();
                (n, c)
            })
            .collect();
        blockers.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
        let mut missed = held(&tier);
        for (_, c) in blockers {
            let mut more = tier.clone();
            more.insert(c);
            let bys = after(&more);
            let mine: Vec<&BTreeMap<char, u32>> =
                bys.iter().filter_map(|by| by.get(&Some(c))).collect();
            let n: u32 = mine.iter().flat_map(|v| v.values()).sum();
            let decides = mine.iter().all(|v| {
                let n: u32 = v.values().sum();
                tolerated(n, n - v.values().max().copied().unwrap_or(0))
            });
            let (e, m) = (wrong(&bys), held(&more));
            if decides && n as usize >= MIN_WORDS && e < errors && m <= missed {
                (tier, errors, missed) = (more, e, m);
            }
        }
    }
    // A law is along a tier where a letter further back than the one just
    // before decides most places; else it is local: one at the stem's edge
    // (Belth 2024 p. 8), or none at all. (A tier of vowels decides at once
    // where its consonants never started on it: only the places tell.)
    let all = pools.iter().flat_map(|p| &p.learned);
    let (far, n) = all.fold((0usize, 0usize), |(far, n), (before, _, _)| {
        let just = before
            .chars()
            .next_back()
            .is_some_and(|c| tier.contains(&c));
        let back = !just && nearest(before, &tier).is_some();
        (far + usize::from(back), n + 1)
    });
    if tier.is_empty() || (!copy && ((local && !vowels) || 2 * far <= n)) {
        return None;
    }
    let maps: Vec<BTreeMap<Option<char>, char>> = pools
        .iter()
        .map(|p| values(&units(&p.learned, &tier, adjacent)))
        .collect::<Option<Vec<_>>>()?;
    let decides = pools.iter().zip(&maps).all(|(pool, map)| {
        let outputs: BTreeSet<&char> = map.values().collect();
        let fits = |seen: &[Seen<'_>]| {
            let (n, wrong) = judged(&units(seen, &tier, adjacent), map);
            n > 0 && tolerated(n, wrong)
        };
        outputs.len() > 1
            && outputs.iter().all(|c| pool.letters.contains(c))
            && fits(&pool.learned)
            && fits(&pool.held)
    });
    decides.then_some((tier, adjacent))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neutral_vowels_leave_the_tier_and_take_the_default() {
        // Finnish inessive `-ssA`: back after `a o u`, front after `ä ö y`;
        // `e i` decide nothing (`kahvissa`, `käsissä`), and a word with only
        // them takes the front value (`tiessä`). Two-syllable words of every
        // pair of vowels, in enough frames that what a letter decides shows.
        let back = ['a', 'o', 'u'];
        let front = ['ä', 'ö', 'y'];
        let vowels = ['a', 'o', 'u', 'ä', 'ö', 'y', 'e', 'i'];
        let mut words: Vec<(String, char)> = Vec::new();
        let frames = [
            ["k", "l"],
            ["t", "p"],
            ["m", "r"],
            ["s", "n"],
            ["h", "v"],
            ["p", "k"],
            ["l", "t"],
            ["v", "s"],
        ];
        for frame in frames {
            for v in vowels {
                for w in vowels {
                    let word = format!("{}{v}{}{w}", frame[0], frame[1]);
                    // The last vowel that isn't `e` or `i` decides; none: front.
                    let last = [w, v].into_iter().find(|c| !matches!(c, 'e' | 'i'));
                    let value = match last {
                        Some(c) if back.contains(&c) => 'a',
                        Some(c) if front.contains(&c) => 'ä',
                        _ => 'ä',
                    };
                    words.push((word, value));
                }
            }
        }
        let forms: Vec<String> = words.iter().map(|(w, _)| format!("{w}ss")).collect();
        let mut pool = Pool {
            letters: vec!['a', 'ä'],
            learned: Vec::new(),
            held: Vec::new(),
        };
        for (i, (form, (_, value))) in forms.iter().zip(&words).enumerate() {
            let seen = (form.as_str(), *value, i);
            // A third held out, across the vowels.
            if i % 3 == 0 {
                pool.held.push(seen);
            } else {
                pool.learned.push(seen);
            }
        }
        let vowel = |c: char| "aeiouyäö".contains(c);
        let (tier, _) = fit(&[&pool], &vowel).expect("a law on a tier");
        for c in ['a', 'o', 'u', 'ä', 'ö', 'y'] {
            assert!(tier.contains(&c), "{c} decides: {tier:?}");
        }
        for c in ['e', 'i', 's', 't', 'k'] {
            assert!(!tier.contains(&c), "{c} decides nothing: {tier:?}");
        }
        let all: Vec<Seen<'_>> = pool.learned.iter().chain(&pool.held).copied().collect();
        let map = values(&units(&all, &tier, false)).expect("values");
        assert_eq!(map.get(&None), Some(&'ä'), "{map:?}");
    }

    #[test]
    fn a_cell_s_other_spelling_takes_the_class_letter_too() {
        // Finnish allative with a possessive suffix, written two ways
        // (`talolleen` or `talollensa`): back and front words' paradigms
        // differ only in the second spelling's last letter. The inessive
        // `-ssA` makes the class; with its letter in that spelling too, they
        // are one paradigm.
        let orthography = Orthography {
            lowercase: true,
            vowels: "aeiouyäö",
            stress_marks: "",
            stressed: &[],
            implicit: "",
            dropped: "",
            mark_monosyllables: false,
            folded: &[],
            prosody: None,
            initial: "",
        };
        let consonants = ["k", "t", "p", "m", "s", "n", "h", "v", "l", "r"];
        // Fewer front words than back, as in Finnish: the tier is the back
        // vowels, the front value where there is none (the tier keeps the
        // commoner letters).
        let mut lexicon: Vec<(String, [String; 3], u16)> = Vec::new();
        for (i, c) in consonants.iter().enumerate() {
            for d in consonants {
                let front: &[char] = if i % 2 == 0 { &['ä', 'ö'] } else { &[] };
                for (vowels, a, p) in [(&['a', 'o', 'u'][..], 'a', 0), (front, 'ä', 1)] {
                    for &v in vowels {
                        let word = format!("{c}{v}{d}{v}");
                        let forms = [word.clone(), format!("{word}ss{a}"), format!("{word}lleen")];
                        lexicon.push((word, forms, p));
                    }
                }
            }
        }
        let words: Vec<Word<'_>> = lexicon
            .iter()
            .map(|(w, forms, _)| Word {
                letters: w,
                forms: forms.iter().map(|f| Some(f.as_str())).collect(),
            })
            .collect();
        let paradigm = |name: &'static str, table: &'static str| Paradigm {
            name: Cow::Borrowed(name),
            words: 0,
            lemma: Cow::Borrowed("~"),
            parent: None,
            inherent: Cow::Borrowed(&[]),
            table: Cow::Borrowed(table),
            stems: Cow::Borrowed(""),
            reads: None,
            affix: None,
        };
        let paradigms = vec![
            paradigm("talo", "~\t~ssa\t~lleen|~llensa\t"),
            paradigm("kylä", "~\t~ssä\t~lleen|~llensä\t"),
        ];
        let assignment: Vec<u16> = lexicon.iter().map(|w| w.2).collect();
        let (laws, paradigms, assignment) = learn(&orthography, &words, paradigms, assignment);
        assert!(!laws.is_empty());
        let tables: Vec<&str> = paradigms.iter().map(|p| p.table.as_ref()).collect();
        assert_eq!(tables, ["~\t~ssA\t~lleen|~llensA\t"]);
        assert!(assignment.iter().all(|&a| a == 0));
    }

    #[test]
    fn a_copy_of_the_vowel_before_is_a_law_however_near() {
        // The Finnish illative `-Vn`: `taloon`, `kylään`, `katuun`; the
        // vowel copied is just before the class letter, as a law at the
        // stem's edge would read it, but no one such law states it.
        let vowels = ['a', 'e', 'i', 'o', 'u', 'y', 'ä', 'ö'];
        let frames = [["k", "l"], ["t", "p"], ["m", "r"], ["s", "n"], ["h", "v"]];
        let mut forms: Vec<(String, char)> = Vec::new();
        for frame in frames {
            for v in vowels {
                for w in vowels {
                    forms.push((format!("{}{v}{}{w}", frame[0], frame[1]), w));
                }
            }
        }
        let mut pool = Pool {
            letters: vowels.to_vec(),
            learned: Vec::new(),
            held: Vec::new(),
        };
        for (i, (form, value)) in forms.iter().enumerate() {
            let seen = (form.as_str(), *value, i);
            if i % 3 == 0 {
                pool.held.push(seen);
            } else {
                pool.learned.push(seen);
            }
        }
        let vowel = |c: char| vowels.contains(&c);
        let (tier, adjacent) = fit(&[&pool], &vowel).expect("a copy");
        assert!(!adjacent, "a copy of vowels reads the nearest");
        let all: Vec<Seen<'_>> = pool.learned.iter().chain(&pool.held).copied().collect();
        let map = values(&units(&all, &tier, adjacent)).expect("values");
        for v in vowels {
            assert_eq!(map.get(&Some(v)), Some(&v), "{map:?}");
        }
    }
}
