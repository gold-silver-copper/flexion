//! Sound laws: changes where a stem meets its ending (Greek `γ+σ → +ξ`,
//! Russian `[к|г|х]+ы → +и`), of a letter of the ending by the nearest of
//! some letters before it (Sanskrit `न → ण` after `र`), or of a copied
//! letter (Greek `φ` copied as `π`: `πεφίληκα`), or where a literal before
//! the stem meets it (the Greek augment lengthens a vowel: `ἐ‹ἀ → ἠ`,
//! `ἤγον`). They are found
//! where a word's paradigm, or a more general one, would build its forms but
//! for such a change, and kept when they fix many forms and break few. The
//! words they explain join the more general paradigm, and paradigms left
//! without words go.

use crate::HashMap;
use flexion::pattern::{self, BOUNDARY, COPIED, FRONT};
use flexion::{Orthography, Paradigm};
use sound_law::{Map, Program, Rule, RuleSet};
use std::borrow::Cow;

/// What a law changes.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Change {
    /// Where stem meets ending: after one of `stem` (the stem's last
    /// letters), the ending's first letters `ending` and those of the stem
    /// become `to` (with the boundary): `ε+ο → +ου`. With `keep`, the stem's
    /// letter stays before `to`: `[к|г|х]+ы → +и`. With `earlier`, only
    /// where one of those letters (the vowels) comes earlier in the stem
    /// (`φιλε+ο` contracts, `πλε+ο` doesn't).
    Edge {
        stem: Vec<String>,
        ending: String,
        to: String,
        keep: bool,
        earlier: Vec<char>,
    },
    /// Along a tier: `letter` in the ending becomes `to` where the nearest
    /// of `on` and `off` before it is one of `on`, and none of `stop`
    /// follows it.
    Tier {
        letter: char,
        to: char,
        on: Vec<char>,
        off: Vec<char>,
        stop: Vec<char>,
    },
    /// A copy of `letter` (see [`pattern::COPY`]) is `to`.
    Copy { letter: char, to: char },
    /// Where the literal a form begins with meets the stem (see [`FRONT`]):
    /// the literal (as built, a copy marked) and the stem's first letters
    /// `stem` become `to`: `ἐ‹ἀ → ἠ`. A one-letter stem written without
    /// its marks (inside a compound, `συν|αρχω`) changes as well, into
    /// `plain`'s letters (`α → η`).
    Front {
        literal: String,
        stem: String,
        to: String,
        plain: Vec<(char, String)>,
    },
}

/// One learned law.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Law {
    pub change: Change,
    /// The words whose forms it gives.
    pub words: u32,
    /// Some of them.
    pub examples: Vec<String>,
    /// The dictionary's forms it changes from right to wrong.
    pub breaks: u32,
}

impl std::fmt::Display for Law {
    /// As a grammar states it: `ε+ο → +ου`, `[к|г|х]+ы → +и`, `न → ण after र,
    /// ष unless त, … comes between, not before ्`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let list = |l: &[char]| l.iter().map(char::to_string).collect::<Vec<_>>().join(", ");
        match &self.change {
            Change::Edge {
                stem,
                ending,
                to,
                earlier,
                ..
            } => {
                match stem.as_slice() {
                    [one] => write!(f, "{one}{BOUNDARY}{ending} → {to}")?,
                    many => write!(f, "[{}]{BOUNDARY}{ending} → {to}", many.join("|"))?,
                }
                if !earlier.is_empty() {
                    write!(f, " after a vowel earlier in the stem")?;
                }
                Ok(())
            }
            Change::Tier {
                letter,
                to,
                on,
                off,
                stop,
            } => {
                write!(f, "{letter} → {to} after {}", list(on))?;
                if !off.is_empty() {
                    write!(f, " unless {} comes between", list(off))?;
                }
                if !stop.is_empty() {
                    write!(f, ", not before {}", list(stop))?;
                }
                Ok(())
            }
            Change::Copy { letter, to } => write!(f, "copy of {letter} → {to}"),
            // A copy as the pattern writes it: `%ε‹ἀ → ἠ`.
            Change::Front {
                literal, stem, to, ..
            } => {
                let mut copy = false;
                for c in literal.chars() {
                    match (c, copy) {
                        (COPIED, _) => copy = true,
                        (_, true) => {
                            copy = false;
                            write!(f, "{}", pattern::COPY)?;
                        }
                        _ => write!(f, "{c}")?,
                    }
                }
                write!(f, "{FRONT}{stem} → {to}")
            }
        }
    }
}

impl Change {
    /// A change at the stem's edge made only where one of `vowels` comes
    /// earlier in the stem; `None` for any other.
    fn after(&self, vowels: &[char]) -> Option<Change> {
        match self {
            Change::Edge {
                stem,
                ending,
                to,
                keep,
                earlier,
            } if earlier.is_empty() && !vowels.is_empty() => Some(Change::Edge {
                stem: stem.clone(),
                ending: ending.clone(),
                to: to.clone(),
                keep: *keep,
                earlier: vowels.to_vec(),
            }),
            _ => None,
        }
    }

    /// What it makes of a form built marked at the stem's edge; `None` where
    /// it doesn't apply. A law on a tier runs as its compiled rule.
    fn rewrite(&self, built: &str) -> Option<String> {
        match self {
            Change::Edge {
                stem: letters,
                ending: first,
                to,
                keep,
                earlier,
            } => {
                let (stem, ending) = built.split_once(BOUNDARY)?;
                let rest = ending.strip_prefix(first.as_str())?;
                let letter = letters.iter().find(|l| stem.ends_with(l.as_str()))?;
                let head = stem.strip_suffix(letter.as_str())?;
                if !earlier.is_empty() && !head.chars().any(|c| earlier.contains(&c)) {
                    return None;
                }
                let kept = if *keep { letter.as_str() } else { "" };
                Some(format!("{head}{kept}{to}{rest}"))
            }
            Change::Tier { .. } => compile(std::slice::from_ref(self)).map(|p| p.apply(built)),
            Change::Copy { letter, to } => {
                let copy = format!("{COPIED}{letter}");
                built
                    .contains(&copy)
                    .then(|| built.replace(&copy, &to.to_string()))
            }
            Change::Front {
                literal,
                stem,
                to,
                plain,
            } => {
                let rest = built.strip_prefix(&format!("{literal}{FRONT}"))?;
                let mut letters = rest.chars();
                let first = letters.next()?;
                match plain.iter().find(|(c, _)| *c == first) {
                    Some((_, plain)) => Some(format!("{plain}{}", letters.as_str())),
                    None => rest
                        .strip_prefix(stem.as_str())
                        .map(|rest| format!("{to}{rest}")),
                }
            }
        }
    }

    /// Whether it may apply where a build has `window` around its boundary:
    /// a law on a tier, wherever its letter is; on a copy, wherever it is
    /// copied.
    fn applies(&self, (before, after): &(String, String), built: &str) -> bool {
        match self {
            Change::Edge { stem, ending, .. } => {
                after.starts_with(ending.as_str())
                    && stem.iter().any(|l| before.ends_with(l.as_str()))
            }
            Change::Tier { letter, .. } => built.contains(*letter),
            Change::Copy { letter, .. } => built.contains(&format!("{COPIED}{letter}")),
            Change::Front { .. } => self.rewrite(built).is_some(),
        }
    }

    /// As `sound-law` states it, as rule `name`: what changes into what,
    /// what must come before, and what after stops it; and the letters a
    /// captured letter becomes, if it says so.
    fn rule(&self, name: &str) -> ([String; 4], Vec<(String, String)>) {
        let list = |l: &[char]| l.iter().map(char::to_string).collect::<Vec<_>>().join("|");
        let rule = match self {
            Change::Edge {
                stem,
                ending,
                to,
                keep,
                earlier,
            } => {
                let stem = stem.join("|");
                let (target, to) = if *keep {
                    (format!("{{x:[{stem}]}}"), format!("{{x}}{to}"))
                } else {
                    (stem, to.clone())
                };
                // A vowel, then none until the target.
                let before = if earlier.is_empty() {
                    String::new()
                } else {
                    let v = list(earlier);
                    format!("{{[{v}]}}{{![{v}]*}}")
                };
                [
                    format!("{target}{BOUNDARY}{ending}"),
                    to,
                    before,
                    String::new(),
                ]
            }
            // The letter is the ending's: the stem's are as the dictionary
            // writes them.
            Change::Tier {
                letter,
                to,
                on,
                off,
                stop,
            } => {
                let tier = list(&[on.as_slice(), off.as_slice()].concat());
                [
                    letter.to_string(),
                    to.to_string(),
                    format!(
                        "{{t:[{}]}}{{a:![{tier}]*}}{BOUNDARY}{{b:![{tier}]*}}",
                        list(on)
                    ),
                    if stop.is_empty() {
                        String::new()
                    } else {
                        format!("{{[{}]}}", list(stop))
                    },
                ]
            }
            Change::Copy { letter, to } => [
                format!("{COPIED}{letter}"),
                to.to_string(),
                String::new(),
                String::new(),
            ],
            // A capture, whose letters become those the law says.
            Change::Front {
                literal,
                stem,
                to,
                plain,
            } => {
                let map: Vec<(String, String)> = std::iter::once((stem.clone(), to.clone()))
                    .chain(plain.iter().map(|(c, p)| (c.to_string(), p.clone())))
                    .collect();
                let letters: Vec<&str> = map.iter().map(|(c, _)| c.as_str()).collect();
                let rule = [
                    format!("^{literal}{FRONT}{{x:[{}]}}", letters.join("|")),
                    format!("{{x|{name}}}"),
                    String::new(),
                    String::new(),
                ];
                return (rule, map);
            }
        };
        (rule, Vec::new())
    }
}

/// The laws compiled, in order.
pub fn compile(changes: &[Change]) -> Option<Program<'static>> {
    let texts: Vec<_> = changes
        .iter()
        .enumerate()
        .map(|(i, c)| {
            let name = format!("law{i}");
            let (rule, map) = c.rule(&name);
            (name, rule, map)
        })
        .collect();
    let entries: Vec<(&str, Vec<(&str, &str)>)> = texts
        .iter()
        .filter(|(_, _, map)| !map.is_empty())
        .map(|(name, _, map)| {
            let pairs = map.iter().map(|(a, b)| (a.as_str(), b.as_str())).collect();
            (name.as_str(), pairs)
        })
        .collect();
    let maps: Vec<Map<'_>> = entries
        .iter()
        .map(|(name, pairs)| Map::new(name, pairs))
        .collect();
    let rules: Vec<Rule<'_>> = texts
        .iter()
        .map(|(name, [target, to, before, stop], _)| {
            let rule = Rule::new(name, target, to).in_context(before, "");
            if stop.is_empty() {
                rule
            } else {
                rule.unless_context("", stop)
            }
        })
        .collect();
    RuleSet::new_with_maps(&[], &maps, &rules).compile().ok()
}

/// A word, as the learner sees it: its letters, and per cell its usual form
/// (`None` where the dictionary has none, or the cell is derived).
pub struct Word<'w> {
    pub letters: &'w str,
    pub forms: Vec<Option<&'w str>>,
}

/// The fewest words a law must give forms of.
pub(crate) const MIN_WORDS: usize = 5;
/// The fewest cells a law must give forms in.
const MIN_CELLS: usize = 2;
/// The most forms a law may break, as a share of the forms it fixes.
const BREAKS: f64 = 0.05;
/// The more general paradigms a word's paradigm is compared with.
const NEIGHBOURS: usize = 4;

/// One of the paradigms a word could take, and what it builds: per form
/// the word has, the form marked at the stem's edge, after the laws so far.
#[derive(Clone)]
struct Reading {
    paradigm: usize,
    builds: Vec<String>,
    /// How many of them are the word's forms.
    right: usize,
}

/// A build: its word, its reading, its place among the word's forms.
type At = (usize, usize, usize);

/// A law kept, the words whose forms it gives, and how many forms.
type Kept = (Law, Vec<usize>, usize);

/// The laws, and the paradigms and assignment once the words they explain
/// have joined more general paradigms.
pub fn learn<'s>(
    orthography: &Orthography<'_>,
    words: &[Word<'_>],
    paradigms: Vec<Paradigm<'s>>,
    assignment: Vec<u16>,
) -> (
    Vec<Law>,
    Option<Program<'static>>,
    Vec<Paradigm<'s>>,
    Vec<u16>,
) {
    let cells = paradigms
        .first()
        .map_or(0, |p| p.table.split('\t').count().saturating_sub(1));
    // A paradigm is more general than another when its lemma fixes fewer
    // letters (`~ω` takes `ποιεω` as well as `λυω`); of equally general
    // ones, the commoner.
    let rank: Vec<(usize, usize)> = paradigms
        .iter()
        .enumerate()
        .map(|(i, p)| (p.lemma.chars().filter(|&c| c != '~').count(), i))
        .collect();
    let rank = |p: usize| rank.get(p).copied().unwrap_or((usize::MAX, p));
    let neighbours = neighbours(&paradigms, cells, rank);
    let forms: Vec<Vec<&str>> = words
        .iter()
        .map(|w| w.forms.iter().flatten().copied().collect())
        .collect();
    let cells_of: Vec<Vec<usize>> = words
        .iter()
        .map(|w| {
            w.forms
                .iter()
                .enumerate()
                .filter_map(|(c, f)| f.map(|_| c))
                .collect()
        })
        .collect();
    let readings: Vec<Vec<Reading>> = words
        .iter()
        .zip(&assignment)
        .map(|(w, &own)| {
            let own = usize::from(own);
            std::iter::once(own)
                .chain(neighbours.get(own).into_iter().flatten().copied())
                .filter_map(|p| reading(orthography, &paradigms, p, w))
                .collect()
        })
        .collect();
    // Every change that would let a word's paradigm (where a change explains
    // the form), or a more general one (all its forms), build a form it has,
    // with the words it would do that for.
    let mut support: HashMap<Change, u32> = HashMap::default();
    for (rs, fs) in readings.iter().zip(&forms) {
        let Some(own) = rs.first() else { continue };
        let mut mine: Vec<Change> = own
            .builds
            .iter()
            .zip(fs)
            .filter(|(b, f)| !same(b, f))
            .flat_map(|(b, f)| explain(b, f))
            .collect();
        mine.extend(
            rs.iter()
                .skip(1)
                .filter_map(|r| options(own, r, fs))
                .flatten(),
        );
        mine.sort();
        mine.dedup();
        for edit in mine {
            *support.entry(edit).or_default() += 1;
        }
    }
    let mut candidates: Vec<(Change, u32)> = classes(&support);
    candidates.extend(support);
    candidates.extend(tiers(&readings, &forms));
    candidates.retain(|&(_, n)| n as usize >= MIN_WORDS);
    candidates.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    // Each change at the stem's edge also where a vowel comes earlier in the
    // stem, right after it: kept where the change alone breaks too much.
    let mut vowels: Vec<char> = words
        .iter()
        .flat_map(|w| w.letters.chars())
        .filter(|&c| orthography.is_vowel(c))
        .collect();
    vowels.sort_unstable();
    vowels.dedup();
    let candidates: Vec<(Change, u32)> = candidates
        .into_iter()
        .flat_map(|(change, n)| {
            let twin = change.after(&vowels).map(|c| (c, n));
            std::iter::once((change, n)).chain(twin)
        })
        .collect();
    let (laws, moved) = greedy(
        &candidates,
        readings,
        &forms,
        &cells_of,
        assignment.clone(),
        &rank,
    );
    let laws: Vec<Law> = laws
        .into_iter()
        .filter(|(l, users, forms)| pays(l, users, *forms))
        .map(|(mut law, users, _)| {
            law.words = u32::try_from(users.len()).unwrap_or(u32::MAX);
            law.examples = users
                .iter()
                .take(4)
                .filter_map(|&w| words.get(w).map(|w| w.letters.to_string()))
                .collect();
            law
        })
        .collect();
    let mut laws = laws;
    plain(orthography, &mut laws);
    let changes: Vec<Change> = laws.iter().map(|l| l.change.clone()).collect();
    let program = (!laws.is_empty()).then(|| compile(&changes)).flatten();
    // A word stays where it was unless, under the laws kept, the paradigm
    // it moved to builds its forms as well.
    let right = |p: u16, w: &Word<'_>| {
        reading(orthography, &paradigms, usize::from(p), w).map_or(0, |r| {
            r.builds
                .iter()
                .zip(w.forms.iter().flatten())
                .filter(|(b, f)| {
                    same(
                        &program
                            .as_ref()
                            .map_or_else(|| b.to_string(), |p| p.apply(b)),
                        f,
                    )
                })
                .count()
        })
    };
    let assignment: Vec<u16> = assignment
        .iter()
        .zip(&moved)
        .zip(words)
        .map(|((&was, &now), w)| {
            if now != was && right(now, w) >= right(was, w) {
                now
            } else {
                was
            }
        })
        .collect();
    let (paradigms, assignment) = drop_empty(paradigms, assignment);
    (laws, program, paradigms, assignment)
}

/// The letters each law where a literal meets a one-letter stem (see
/// [`Change::Front`]) on a vowel also takes: its letter written with fewer
/// of its marks (`α`, `ᾰ` for `ἀ`: inside a compound), no other such law's,
/// and with no mark another law's letter has that it hasn't. What one becomes
/// lacks the marks it lacks (`ἐ‹ἀ → ἠ`, so `α → η`).
fn plain(orthography: &Orthography<'_>, laws: &mut [Law]) {
    use unicode_normalization::UnicodeNormalization;
    let marks = |c: char| -> Vec<char> { c.to_string().nfd().skip(1).collect() };
    let fronts: Vec<(String, String)> = laws
        .iter()
        .filter_map(|l| match &l.change {
            Change::Front { literal, stem, .. } => Some((literal.clone(), stem.clone())),
            _ => None,
        })
        .collect();
    // The marks some law's letter has: those the laws tell apart.
    let telling: Vec<char> = fronts
        .iter()
        .flat_map(|(_, stem)| stem.chars().flat_map(marks))
        .collect();
    for law in laws.iter_mut() {
        let Change::Front {
            literal,
            stem,
            to,
            plain,
        } = &mut law.change
        else {
            continue;
        };
        let mut letters = stem.chars();
        let (Some(s), None) = (letters.next(), letters.next()) else {
            continue;
        };
        let mine = marks(s);
        let base = orthography.fold(s);
        if !orthography.is_vowel(base) {
            continue;
        }
        let others = orthography
            .folded
            .iter()
            .filter(|(_, f)| *f == base)
            .map(|(c, _)| *c)
            .chain(std::iter::once(base));
        for c in others {
            let theirs = marks(c);
            let claimed = fronts
                .iter()
                .any(|(l, st)| l == literal && st.starts_with(c));
            let fewer = theirs
                .iter()
                .all(|m| mine.contains(m) || !telling.contains(m))
                && mine.iter().any(|m| !theirs.contains(m));
            if claimed || !fewer || plain.iter().any(|(p, _)| *p == c) {
                continue;
            }
            let lacks: Vec<char> = mine
                .iter()
                .filter(|m| !theirs.contains(m))
                .copied()
                .collect();
            let becomes: String = to
                .nfd()
                .filter(|m| !lacks.contains(m))
                .collect::<String>()
                .nfc()
                .collect();
            plain.push((c, becomes));
        }
        plain.sort();
    }
}

/// What paradigm `p` builds for word `w`, if it can take it.
fn reading(
    orthography: &Orthography<'_>,
    paradigms: &[Paradigm<'_>],
    p: usize,
    w: &Word<'_>,
) -> Option<Reading> {
    let par = paradigms.get(p)?;
    let stems = par.stems(orthography, w.letters)?;
    let builds: Vec<String> = w
        .forms
        .iter()
        .enumerate()
        .filter(|(_, f)| f.is_some())
        .map(|(cell, _)| {
            par.pattern(paradigms, cell)
                .and_then(|pat| pattern::variants(pat).next())
                .map(|v| pattern::fill_marked(v, &stems))
                .unwrap_or_default()
        })
        .collect();
    let right = builds
        .iter()
        .zip(w.forms.iter().flatten())
        .filter(|(b, f)| same(b, f))
        .count();
    Some(Reading {
        paradigm: p,
        builds,
        right,
    })
}

/// The laws, tried in turn (the best supported first), and the assignment
/// after the words they explain moved. A law is kept when it fixes forms of
/// enough words, in more than one cell (a sound law applies wherever its
/// letters meet), and breaks few their paradigm gave; with each, the words
/// whose forms it gives (by their place in `forms`) and how many forms.
fn greedy(
    candidates: &[(Change, u32)],
    mut readings: Vec<Vec<Reading>>,
    forms: &[Vec<&str>],
    cells_of: &[Vec<usize>],
    mut assignment: Vec<u16>,
    rank: &dyn Fn(usize) -> (usize, usize),
) -> (Vec<Kept>, Vec<u16>) {
    // Builds by the letters around their boundary.
    let mut index: HashMap<(String, String), Vec<At>> = HashMap::default();
    for (w, rs) in readings.iter().enumerate() {
        for (r, reading) in rs.iter().enumerate() {
            for (k, built) in reading.builds.iter().enumerate() {
                index.entry(window(built)).or_default().push((w, r, k));
            }
        }
    }
    let form = |w: usize, k: usize| forms.get(w).and_then(|f| f.get(k)).copied().unwrap_or("");
    let mut laws: Vec<(Law, Vec<At>)> = Vec::new();
    for (change, _) in candidates {
        // A law on a tier runs as its rule, once compiled.
        let program = match change {
            Change::Tier { .. } => match compile(std::slice::from_ref(change)) {
                Some(p) => Some(p),
                None => continue,
            },
            Change::Edge { .. } | Change::Copy { .. } | Change::Front { .. } => None,
        };
        let mut at: Vec<At> = index
            .iter()
            .filter(|(k, _)| !matches!(change, Change::Edge { .. }) || change.applies(k, ""))
            .flat_map(|(_, v)| v.iter().copied())
            .collect();
        at.sort_unstable();
        at.dedup();
        let mut changes: Vec<(At, String)> = Vec::new();
        let mut fixed: Vec<At> = Vec::new();
        let mut breaks = 0u32;
        for (w, r, k) in at {
            let Some(reading) = readings.get(w).and_then(|rs| rs.get(r)) else {
                continue;
            };
            // Not an entry the laws so far have moved elsewhere.
            let Some(built) = reading
                .builds
                .get(k)
                .filter(|b| change.applies(&window(b), b))
            else {
                continue;
            };
            let after = match &program {
                Some(p) => p.apply(built),
                None => change.rewrite(built).unwrap_or_else(|| built.clone()),
            };
            if after == *built {
                continue;
            }
            let (was, is) = (same(built, form(w, k)), same(&after, form(w, k)));
            let own = assignment
                .get(w)
                .is_some_and(|&p| usize::from(p) == reading.paradigm);
            breaks += u32::from(own && was && !is);
            if !was && is {
                fixed.push((w, r, k));
            }
            changes.push(((w, r, k), after));
        }
        let mut fixed_words: Vec<usize> = fixed.iter().map(|a| a.0).collect();
        fixed_words.dedup();
        let mut fixed_cells: Vec<usize> = fixed
            .iter()
            .filter_map(|&(w, _, k)| cells_of.get(w)?.get(k).copied())
            .collect();
        fixed_cells.sort_unstable();
        fixed_cells.dedup();
        #[allow(clippy::cast_precision_loss)]
        let worth = fixed_words.len() >= MIN_WORDS
            && fixed_cells.len() >= MIN_CELLS
            && f64::from(breaks) <= BREAKS * fixed.len() as f64;
        if !worth {
            continue;
        }
        for ((w, r, k), after) in changes {
            let want = form(w, k);
            let Some(built) =
                readings
                    .get_mut(w)
                    .and_then(|rs| rs.get_mut(r))
                    .and_then(|reading| {
                        reading.right = (reading.right + usize::from(same(&after, want)))
                            .saturating_sub(usize::from(same(reading.builds.get(k)?, want)));
                        reading.builds.get_mut(k)
                    })
            else {
                continue;
            };
            index.entry(window(&after)).or_default().push((w, r, k));
            *built = after;
        }
        // A word whose forms a more general paradigm now builds as well as
        // its own joins it.
        for &w in &fixed_words {
            let (Some(rs), Some(own)) = (readings.get(w), assignment.get_mut(w)) else {
                continue;
            };
            let mine = rs
                .iter()
                .find(|r| r.paradigm == usize::from(*own))
                .map_or(0, |r| r.right);
            let best = rs
                .iter()
                .filter(|r| r.right >= mine)
                .map(|r| r.paradigm)
                .min_by_key(|&p| rank(p));
            if let Some(p) = best.filter(|&p| rank(p) < rank(usize::from(*own)))
                && let Ok(p) = u16::try_from(p)
            {
                *own = p;
            }
        }
        let law = Law {
            change: change.clone(),
            words: 0,
            examples: Vec::new(),
            breaks,
        };
        laws.push((law, fixed));
    }
    // What each law did for the words that moved: forms it fixed that their
    // new paradigm still builds.
    let laws = laws
        .into_iter()
        .map(|(law, fixed)| {
            let used: Vec<At> = fixed
                .into_iter()
                .filter(|&(w, r, k)| {
                    readings
                        .get(w)
                        .and_then(|rs| rs.get(r))
                        .is_some_and(|reading| {
                            assignment
                                .get(w)
                                .is_some_and(|&p| usize::from(p) == reading.paradigm)
                                && reading.builds.get(k).is_some_and(|b| same(b, form(w, k)))
                        })
                })
                .collect();
            let mut users: Vec<usize> = used.iter().map(|a| a.0).collect();
            users.dedup();
            (law, users, used.len())
        })
        .collect();
    (laws, assignment)
}

/// Whether a law pays for itself: it gives `forms` of enough words
/// (`users`) and breaks few.
#[allow(clippy::cast_precision_loss)]
fn pays(law: &Law, users: &[usize], forms: usize) -> bool {
    users.len() >= MIN_WORDS && f64::from(law.breaks) <= BREAKS * forms as f64
}

/// The letters around a build's boundary: up to two before, two after.
fn window(built: &str) -> (String, String) {
    let (stem, ending) = built.split_once(BOUNDARY).unwrap_or(("", built));
    let skip = stem.chars().count().saturating_sub(2);
    (
        stem.chars().skip(skip).collect(),
        ending.chars().take(2).collect(),
    )
}

/// Per paradigm, the more general paradigms with the same number of stem
/// pieces whose cells could build its cells but for a change at the stem's
/// edge (at least half of them), closest first.
fn neighbours(
    paradigms: &[Paradigm<'_>],
    cells: usize,
    rank: impl Fn(usize) -> (usize, usize),
) -> Vec<Vec<usize>> {
    let tables: Vec<Vec<&str>> = paradigms
        .iter()
        .map(|p| {
            (0..cells)
                .map(|c| {
                    p.pattern(paradigms, c)
                        .and_then(|p| pattern::variants(p).next())
                        .unwrap_or("")
                })
                .collect()
        })
        .collect();
    (0..paradigms.len())
        .map(|q| {
            let (Some(mine), Some(me)) = (tables.get(q), paradigms.get(q)) else {
                return Vec::new();
            };
            let mut close: Vec<((usize, usize), usize)> = (0..paradigms.len())
                .filter(|&p| rank(p) < rank(q))
                .filter(|&p| {
                    paradigms.get(p).is_some_and(|p| {
                        pattern::pieces(&p.lemma) == pattern::pieces(&me.lemma)
                            && stem_like(&p.lemma, &me.lemma, mine)
                    })
                })
                .filter_map(|p| {
                    let theirs = tables.get(p)?;
                    let same = theirs.iter().zip(mine).filter(|(a, b)| a == b).count();
                    let near = theirs.iter().zip(mine).filter(|(a, b)| near(a, b)).count();
                    (2 * near >= cells).then_some(((near, same), p))
                })
                .collect();
            close.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
            close.into_iter().take(NEIGHBOURS).map(|(_, p)| p).collect()
        })
        .collect()
}

/// Whether what a paradigm with lemma `general` takes as stem and the one
/// with lemma `lemma` (and `cells`) as ending is stem: the letters that
/// begin most of its endings (`γ` of `~γω`, `~γεις`, `~γον`; not `о` of
/// `~о`, `~а`, `~у`).
fn stem_like(general: &str, lemma: &str, cells: &[&str]) -> bool {
    let end = |p: &str| p.rsplit_once('~').map(|(_, e)| e.to_string());
    let Some(moved) = end(lemma)
        .zip(end(general))
        .and_then(|(l, g)| l.strip_suffix(g.as_str()).map(str::to_string))
    else {
        return false;
    };
    let had: Vec<&&str> = cells.iter().filter(|c| !c.is_empty()).collect();
    let begins = had
        .iter()
        .filter(|c| end(c).is_some_and(|e| e.starts_with(&moved)))
        .count();
    moved.is_empty() || 2 * begins >= had.len()
}

/// Whether pattern `p` builds what `q` does but for a change of the stem's
/// last letters and the ending's first two at most, and of its first
/// letters and the last two of a literal before it: the same between the
/// first stem piece and the last, `q`'s ending is `p`'s, its first letters
/// changed (`~ομεν`, `~ουμεν`), and its beginning `p`'s, its last letters
/// changed (`ἐ~ον`, `ἠ~ον`).
fn near(p: &str, q: &str) -> bool {
    // Before the first stem piece, between it and the last, and after.
    let parts = |s: &str| {
        let (head, end) = s.rsplit_once('~')?;
        let (start, middle) = head.split_once('~').unwrap_or((head, ""));
        let chars = |s: &str| s.chars().collect::<Vec<char>>();
        Some((chars(start), middle.to_string(), chars(end)))
    };
    let (Some((p_start, p_middle, p_end)), Some((q_start, q_middle, q_end))) = (parts(p), parts(q))
    else {
        return p == q;
    };
    let end = (0..=2).any(|m| {
        p_end
            .get(m..)
            .is_some_and(|rest| q_end.ends_with(rest) && q_end.len() <= rest.len() + 4)
    });
    let start = (0..=2).any(|m| {
        p_start
            .get(..p_start.len().saturating_sub(m))
            .is_some_and(|rest| q_start.starts_with(rest) && q_start.len() <= rest.len() + 4)
    });
    p_middle == q_middle && start && end
}

/// The changes at the stem's edge that would let reading `r` build the forms
/// the word's `own` reading builds: every way of explaining each (see
/// [`edits`]). `None` when one can't be explained so, or none differs.
fn options(own: &Reading, r: &Reading, forms: &[&str]) -> Option<Vec<Change>> {
    let mut out: Vec<Change> = Vec::new();
    for ((built, mine), want) in r.builds.iter().zip(&own.builds).zip(forms) {
        if same(built, want) || !same(mine, want) {
            continue;
        }
        let ways = explain(built, want);
        if ways.is_empty() {
            return None;
        }
        out.extend(ways);
    }
    (!out.is_empty()).then_some(out)
}

/// The changes that turn `built` into `want`: at the stem's edge (see
/// [`edits`]), of a copy (see [`copies`]), or where a literal before the
/// stem meets it (see [`fronts`]).
fn explain(built: &str, want: &str) -> Vec<Change> {
    edits(built, want)
        .into_iter()
        .chain(copies(built, want))
        .chain(fronts(built, want))
        .collect()
}

/// The changes of the stem's last letters (one or two) and the ending's
/// first (none to two) that turn `built` into `want`: `ε+ο → +ου`. What
/// stays of the stem keeps the boundary after it.
fn edits(built: &str, want: &str) -> Vec<Change> {
    let built: String = built
        .chars()
        .filter(|&c| c != COPIED && c != FRONT)
        .collect();
    let Some((stem, ending)) = built.split_once(BOUNDARY) else {
        return Vec::new();
    };
    let (stem, ending, want): (Vec<char>, Vec<char>, Vec<char>) = (
        stem.chars().collect(),
        ending.chars().collect(),
        want.chars().collect(),
    );
    let mut out = Vec::new();
    for k in 1..=2usize {
        for m in 0..=2usize {
            let Some(h) = stem.len().checked_sub(k) else {
                continue;
            };
            let (Some(head), Some(a), Some(b), Some(tail)) = (
                stem.get(..h),
                stem.get(h..),
                ending.get(..m),
                ending.get(m..),
            ) else {
                continue;
            };
            let Some(middle) = (want.starts_with(head) && want.ends_with(tail))
                .then(|| want.get(head.len()..want.len().checked_sub(tail.len())?))
                .flatten()
            else {
                continue;
            };
            let kept = a.iter().zip(middle).take_while(|(x, y)| x == y).count();
            let (Some(kept_part), Some(rest)) = (middle.get(..kept), middle.get(kept..)) else {
                continue;
            };
            out.push(Change::Edge {
                stem: vec![a.iter().collect()],
                ending: b.iter().collect(),
                to: kept_part.iter().chain(&[BOUNDARY]).chain(rest).collect(),
                keep: false,
                earlier: Vec::new(),
            });
        }
    }
    out
}

/// The changes of copied letters that `want` has where `built` copies
/// another (`°φεφιλ+ηκα`, `πεφιληκα`: a copy of `φ` is `π`).
fn copies(built: &str, want: &str) -> Vec<Change> {
    let mut want = want.chars();
    let mut out = Vec::new();
    let mut copied = false;
    for c in built.chars().filter(|&c| c != BOUNDARY && c != FRONT) {
        if c == COPIED {
            copied = true;
            continue;
        }
        match want.next() {
            Some(to) if copied && to != c => out.push(Change::Copy { letter: c, to }),
            Some(_) => {}
            None => break,
        }
        copied = false;
    }
    out
}

/// The changes where the literal a form begins with meets the stem (see
/// [`FRONT`]) that turn `built` into `want`: the literal and the stem's
/// first letters (one or two) become what `want` has there (`ἐ‹ἀγ+ον`,
/// `ἠγον`: `ἐ‹ἀ → ἠ`).
fn fronts(built: &str, want: &str) -> Vec<Change> {
    let Some((literal, stem)) = built.split_once(FRONT) else {
        return Vec::new();
    };
    let (stem, want): (Vec<char>, Vec<char>) = (stem.chars().collect(), want.chars().collect());
    let mut out = Vec::new();
    for m in 1..=2usize {
        let (Some(first), Some(rest)) = (stem.get(..m), stem.get(m..)) else {
            continue;
        };
        if first.iter().any(|&c| pattern::is_mark(c)) {
            continue;
        }
        let rest: Vec<char> = rest
            .iter()
            .copied()
            .filter(|&c| !pattern::is_mark(c))
            .collect();
        let Some(to) = want
            .len()
            .checked_sub(rest.len())
            .filter(|_| want.ends_with(&rest))
            .and_then(|end| want.get(..end))
        else {
            continue;
        };
        let was: String = literal
            .chars()
            .filter(|&c| c != COPIED)
            .chain(first.iter().copied())
            .collect();
        let to: String = to.iter().collect();
        if to != was {
            out.push(Change::Front {
                literal: literal.to_string(),
                stem: first.iter().collect(),
                to,
                plain: Vec::new(),
            });
        }
    }
    out
}

/// The same change of an ending after one letter or another (`к+ы → к+и`,
/// `г+ы → г+и`), as one law on those letters (`[к|г]+ы → +и`), with the
/// words of all of them.
fn classes(support: &HashMap<Change, u32>) -> Vec<(Change, u32)> {
    let mut by_change: HashMap<(&str, &str), Vec<(u32, &str)>> = HashMap::default();
    for (change, &n) in support {
        if let Change::Edge {
            stem,
            ending,
            to,
            keep: false,
            ..
        } = change
            && let [a] = stem.as_slice()
            && a.chars().count() == 1
            && let Some(rest) = to.strip_prefix(a.as_str())
        {
            by_change.entry((ending, rest)).or_default().push((n, a));
        }
    }
    by_change
        .into_iter()
        .filter(|(_, letters)| letters.len() > 1)
        .map(|((ending, rest), mut letters)| {
            letters.sort_by(|x, y| y.cmp(x));
            let change = Change::Edge {
                stem: letters.iter().map(|l| l.1.to_string()).collect(),
                ending: ending.to_string(),
                to: rest.to_string(),
                keep: true,
                earlier: Vec::new(),
            };
            (change, letters.iter().map(|l| l.0).sum())
        })
        .collect()
}

/// A place an ending has the letter a law on a tier may change: the letters
/// before it (the nearest first), the one after it, and whether the
/// dictionary changes it.
type Example = (Vec<char>, Option<char>, bool);

/// Laws on a tier: one letter of an ending becomes another (`न` → `ण`) where
/// the nearest of some letters before it decides (see [`tier`]). Found where
/// the change is away from the stem's edge (a change next to it is a law at
/// the edge). As candidates, with the words that have the change.
fn tiers(readings: &[Vec<Reading>], forms: &[Vec<&str>]) -> Vec<(Change, u32)> {
    // Each build and its form, as letters, and where the ending starts.
    let pairs: Vec<(usize, Vec<char>, Vec<char>, usize)> = readings
        .iter()
        .zip(forms)
        .enumerate()
        .flat_map(|(w, (rs, fs))| {
            rs.iter().flat_map(move |r| {
                r.builds.iter().zip(fs).filter_map(move |(b, f)| {
                    let edge = b
                        .chars()
                        .filter(|&c| c != COPIED && c != FRONT)
                        .position(|c| c == BOUNDARY)?;
                    let built: Vec<char> = b.chars().filter(|&c| !pattern::is_mark(c)).collect();
                    let form: Vec<char> = f.chars().collect();
                    (built.len() == form.len()).then_some((w, built, form, edge))
                })
            })
        })
        .collect();
    // The changes of one letter of the ending past its first, and the words
    // with them.
    let mut changes: HashMap<(char, char), Vec<usize>> = HashMap::default();
    for (w, built, form, edge) in &pairs {
        let differ: Vec<(usize, (char, char))> = built
            .iter()
            .zip(form)
            .enumerate()
            .filter(|(_, (a, b))| a != b)
            .map(|(i, (a, b))| (i, (*a, *b)))
            .collect();
        if let [(at, change)] = differ.as_slice()
            && at > edge
        {
            let words = changes.entry(*change).or_default();
            if words.last() != Some(w) {
                words.push(*w);
            }
        }
    }
    let mut out: Vec<(Change, u32)> = changes
        .into_iter()
        .filter(|(_, words)| words.len() >= MIN_WORDS)
        .filter_map(|((letter, to), words)| {
            let mut examples: HashMap<Example, u32> = HashMap::default();
            for (_, built, form, edge) in &pairs {
                if built
                    .iter()
                    .zip(form)
                    .any(|(a, b)| a != b && (*a, *b) != (letter, to))
                {
                    continue;
                }
                for (i, (&a, &b)) in built.iter().zip(form).enumerate().skip(*edge) {
                    if a == letter {
                        let before: Vec<char> = built
                            .get(..i)
                            .unwrap_or_default()
                            .iter()
                            .rev()
                            .copied()
                            .collect();
                        *examples
                            .entry((before, built.get(i + 1).copied(), b == to))
                            .or_default() += 1;
                    }
                }
            }
            let [on, off, stop] = tier(&examples)?;
            let words = u32::try_from(words.len()).unwrap_or(u32::MAX);
            Some((
                Change::Tier {
                    letter,
                    to,
                    on,
                    off,
                    stop,
                },
                words,
            ))
        })
        .collect();
    out.sort();
    out
}

/// What decides whether the change happens: the letters that make it, the
/// letters that don't (with them, the tier: the nearest of them before the
/// letter decides), and the letters after it that stop it. Grown a letter
/// at a time, each time the one that predicts the most `examples` right,
/// while it predicts at least [`MIN_WORDS`] more. `None` when no letter
/// makes it.
fn tier(examples: &HashMap<Example, u32>) -> Option<[Vec<char>; 3]> {
    let mut examples: Vec<(&Example, u32)> = examples.iter().map(|(e, &n)| (e, n)).collect();
    examples.sort_unstable();
    let mut letters: Vec<char> = examples
        .iter()
        .flat_map(|((before, next, _), _)| before.iter().chain(next))
        .copied()
        .collect();
    letters.sort_unstable();
    letters.dedup();
    let right = |[on, off, stop]: &[Vec<char>; 3]| -> u32 {
        examples
            .iter()
            .filter(|((before, next, y), _)| {
                let decides = before.iter().find(|c| on.contains(c) || off.contains(c));
                (decides.is_some_and(|c| on.contains(c)) && next.is_none_or(|n| !stop.contains(&n)))
                    == *y
            })
            .map(|e| e.1)
            .sum()
    };
    let mut sets: [Vec<char>; 3] = Default::default();
    let mut best = right(&sets);
    loop {
        let mut step: Option<(u32, [Vec<char>; 3])> = None;
        for &l in &letters {
            for side in 0..3 {
                let mut next = sets.clone();
                // A letter is on the tier on one side, and stops it or not.
                let [on, off, stop] = &next;
                let free = if side == 2 {
                    !stop.contains(&l)
                } else {
                    !on.contains(&l) && !off.contains(&l)
                };
                let Some(set) = next.get_mut(side).filter(|_| free) else {
                    continue;
                };
                set.push(l);
                let score = right(&next);
                if step.as_ref().is_none_or(|s| score > s.0) {
                    step = Some((score, next));
                }
            }
        }
        match step {
            Some((score, next)) if score >= best + MIN_WORDS as u32 => (best, sets) = (score, next),
            _ => break,
        }
    }
    for set in &mut sets {
        set.sort_unstable();
    }
    let [on, ..] = &sets;
    (!on.is_empty()).then_some(sets)
}

/// Whether a form built marked (at the stem's edge, and its copies) is
/// `form`.
fn same(built: &str, form: &str) -> bool {
    built
        .chars()
        .filter(|&c| !pattern::is_mark(c))
        .eq(form.chars())
}

/// The paradigms that still have words, and the assignment renumbered.
pub(crate) fn drop_empty<'s>(
    paradigms: Vec<Paradigm<'s>>,
    assignment: Vec<u16>,
) -> (Vec<Paradigm<'s>>, Vec<u16>) {
    let mut count: Vec<u32> = vec![0; paradigms.len()];
    for &a in &assignment {
        if let Some(n) = count.get_mut(usize::from(a)) {
            *n += 1;
        }
    }
    // Commonest first, as everywhere.
    let mut order: Vec<usize> = (0..paradigms.len())
        .filter(|&p| count.get(p).is_some_and(|&n| n > 0))
        .collect();
    order.sort_by(|&a, &b| count.get(b).cmp(&count.get(a)).then(a.cmp(&b)));
    let mut new_of: Vec<Option<u16>> = vec![None; paradigms.len()];
    for (new, &old) in order.iter().enumerate() {
        if let Some(slot) = new_of.get_mut(old) {
            *slot = u16::try_from(new).ok();
        }
    }
    let mut kept: Vec<Option<Paradigm<'s>>> = paradigms.into_iter().map(Some).collect();
    let paradigms = order
        .iter()
        .filter_map(|&old| {
            let mut p = kept.get_mut(old)?.take()?;
            p.words = count.get(old).copied().unwrap_or(0);
            p.name = Cow::Owned(p.name.into_owned());
            Some(p)
        })
        .collect();
    let assignment = assignment
        .into_iter()
        .map(|a| new_of.get(usize::from(a)).copied().flatten().unwrap_or(0))
        .collect();
    (paradigms, assignment)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_the_change_at_the_stem_edge() {
        let edge = |stem: &str, ending: &str, to: &str| Change::Edge {
            stem: vec![stem.to_string()],
            ending: ending.to_string(),
            to: to.to_string(),
            keep: false,
            earlier: Vec::new(),
        };
        // Greek contraction: ποιε + ομεν → ποιουμεν.
        assert!(edits("ποιε+ομεν", "ποιουμεν").contains(&edge("ε", "ο", "+ου")));
        // The Russian spelling rule: стрелк + ы → стрелки.
        assert!(edits("стрелк+ы", "стрелки").contains(&edge("к", "ы", "к+и")));
        // A copied letter changed: φιλη, its perfect built φεφιληκα, is
        // πεφιληκα.
        let plain = Change::Copy {
            letter: 'φ',
            to: 'π',
        };
        assert_eq!(
            copies("°φεφιλ+ηκα", "πεφιληκα"),
            std::slice::from_ref(&plain)
        );
        assert!(copies("°λελυ+κα", "λελυκα").is_empty());
        // The augment of a vowel-initial stem: ἐ + ἀγ is ἠγ.
        let front = |literal: &str, stem: &str, to: &str| Change::Front {
            literal: literal.to_string(),
            stem: stem.to_string(),
            to: to.to_string(),
            plain: Vec::new(),
        };
        let augment = front("ἐ", "ἀ", "ἠ");
        assert!(fronts("ἐ‹ἀγ+ον", "ἠγον").contains(&augment));
        let perfect = front("°ἀε", "ἀ", "ἠ");
        assert!(fronts("°ἀε‹ἀρ+χα", "ἠρχα").contains(&perfect));
        let unbreathed = Change::Front {
            literal: "ἐ".into(),
            stem: "ἀ".into(),
            to: "ἠ".into(),
            plain: vec![('α', "η".into()), ('ᾰ', "η".into())],
        };
        assert_eq!(unbreathed.rewrite("ἐ‹ᾰρχ+ον").as_deref(), Some("ηρχ+ον"));
        let law = Law {
            change: perfect.clone(),
            words: 0,
            examples: Vec::new(),
            breaks: 0,
        };
        assert_eq!(law.to_string(), "%ε‹ἀ → ἠ");
        // The same change after several letters is one law on all of them.
        let mut support = HashMap::default();
        support.insert(edge("к", "ы", "к+и"), 3);
        support.insert(edge("г", "ы", "г+и"), 2);
        let class = classes(&support);
        let velar = Change::Edge {
            stem: vec!["к".into(), "г".into()],
            ending: "ы".into(),
            to: "+и".into(),
            keep: true,
            earlier: Vec::new(),
        };
        assert_eq!(class, [(velar.clone(), 5)]);
        // The learner's rewrite agrees with the compiled rules.
        // Contraction where a vowel comes earlier in the stem: not in `πλε`.
        let vowels: Vec<char> = "αεηιουω".chars().collect();
        let contract = edge("ε", "ο", "+ου").after(&vowels).expect("an edge law");
        assert_eq!(contract.rewrite("πλε+ομεν"), None);
        for (change, built) in [
            (velar, "книг+ы"),
            (edge("ε", "ο", "+ου"), "ποιε+ομεν"),
            (edge("γ", "σ", "+ξ"), "λεγ+σω"),
            (contract.clone(), "φιλε+ομεν"),
            (contract.clone(), "ποιε+ομεν"),
            (contract, "πλε+ομεν"),
            (plain.clone(), "ἐ°φεφιλη+κη"),
            (plain, "°λελυ+κα"),
            (augment.clone(), "ἐ‹ἀγ+ον"),
            (augment, "ἐ‹λυ+ον"),
            (perfect.clone(), "°ἀε‹ἀρ+χα"),
            // The pluperfect's literal is another.
            (perfect, "ἐ°ἀε‹ἀρ+χειν"),
            // A stem written without its breathing, inside a compound.
            (unbreathed.clone(), "ἐ‹ᾰρχ+ον"),
            (unbreathed, "ἐ‹ἀρχ+ον"),
        ] {
            let program = compile(std::slice::from_ref(&change)).expect("compiles");
            assert_eq!(
                change.rewrite(built).unwrap_or_else(|| built.to_string()),
                program.apply(built)
            );
        }
    }
}
