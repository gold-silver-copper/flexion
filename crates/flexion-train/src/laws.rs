//! Sound laws: changes where a stem meets its ending (Greek `γ+σ → +ξ`,
//! Russian `[к|г|х]+ы → +и`), of a letter of the ending by the nearest of
//! some letters before it (Sanskrit `न → ण` after `र`), of a class letter
//! an ending writes by the nearest letter of a tier before it as written
//! (harmony: Turkish `-lAr`), or of a copied
//! letter (Greek `φ` copied as `π`: `πεφίληκα`), or where a literal before
//! the stem meets it (the Greek augment lengthens a vowel: `ἐ‹ἀ → ἠ`,
//! `ἤγον`). They are found where a word's paradigm, or a more general one,
//! would build its forms but for such a change, and kept when they fix many
//! forms and break few, on the words they are learned from and on words
//! held out (Rasin & Katzir 2020). A change at the stem's edge that breaks
//! too many is tried again with a letter more of context on whichever side
//! best tells the forms it gives from those it breaks (Belth 2024 (11),
//! p. 8). The words they explain join the more general paradigm, and
//! paradigms left without words go.

mod harmony;

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
    /// (`φιλε+ο` contracts, `πλε+ο` doesn't). With `left` and `right`,
    /// only between those letters, a set per letter, the nearest first.
    Edge {
        stem: Vec<String>,
        ending: String,
        to: String,
        keep: bool,
        earlier: Vec<char>,
        left: Vec<Vec<char>>,
        right: Vec<Vec<char>>,
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
    /// Harmony: each class letter an ending writes (`A` of `-lAr`) is,
    /// after the nearest of `tier` before it as written (or none), what
    /// `map` says: `(class letter, tier letter, value)`.
    Harmony {
        tier: Vec<char>,
        map: Vec<(char, Option<char>, char)>,
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
                left,
                right,
                ..
            } => {
                match stem.as_slice() {
                    [one] => write!(f, "{one}{BOUNDARY}{ending} → {to}")?,
                    many => write!(f, "[{}]{BOUNDARY}{ending} → {to}", many.join("|"))?,
                }
                if !earlier.is_empty() {
                    write!(f, " after a vowel earlier in the stem")?;
                }
                // The letters around it, as they come.
                let sets = |sets: &mut dyn Iterator<Item = &Vec<char>>| -> String {
                    sets.map(|s| match s.as_slice() {
                        [one] => one.to_string(),
                        many => {
                            let letters: Vec<String> = many.iter().map(char::to_string).collect();
                            format!("[{}]", letters.join("|"))
                        }
                    })
                    .collect()
                };
                if !left.is_empty() {
                    write!(f, " after {}", sets(&mut left.iter().rev()))?;
                }
                if !right.is_empty() {
                    write!(f, " before {}", sets(&mut right.iter()))?;
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
            // `A → a after a, ı, o, u; e after e, i, ö, ü, none`.
            Change::Harmony { tier, map } => {
                let mut classes: Vec<char> = map.iter().map(|e| e.0).collect();
                classes.dedup();
                for (k, class) in classes.iter().enumerate() {
                    let mut values: Vec<(char, Vec<String>)> = Vec::new();
                    for &(_, t, v) in map.iter().filter(|e| e.0 == *class) {
                        let t = t.map_or("none".to_string(), String::from);
                        match values.iter_mut().find(|(w, _)| *w == v) {
                            Some((_, ts)) => ts.push(t),
                            None => values.push((v, vec![t])),
                        }
                    }
                    let values: Vec<String> = values
                        .iter()
                        .map(|(v, ts)| format!("{v} after {}", ts.join(", ")))
                        .collect();
                    let sep = if k == 0 { "" } else { "; " };
                    write!(f, "{sep}{class} → {}", values.join("; "))?;
                }
                write!(f, " (the nearest of {} before it, as written)", list(tier))
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
        let mut twin = self.clone();
        match &mut twin {
            Change::Edge { earlier, .. } if earlier.is_empty() && !vowels.is_empty() => {
                *earlier = vowels.to_vec();
                Some(twin)
            }
            _ => None,
        }
    }

    /// Whether its rule may change a build with `window` around its
    /// boundary, a quick test before running it: a law on a tier, wherever
    /// its letter is; on a copy, wherever it is copied.
    fn applies(&self, (before, after): &(String, String), built: &str) -> bool {
        match self {
            Change::Edge { stem, ending, .. } => {
                after.starts_with(ending.as_str())
                    && stem.iter().any(|l| before.ends_with(l.as_str()))
            }
            Change::Tier { letter, .. } => built.contains(*letter),
            Change::Harmony { map, .. } => map.iter().any(|e| built.contains(e.0)),
            Change::Copy { letter, .. } => built.contains(&format!("{COPIED}{letter}")),
            Change::Front { literal, stem, .. } => {
                built.starts_with(&format!("{literal}{FRONT}{stem}"))
            }
        }
    }

    /// As `sound-law` states it, as rule `name`.
    fn rule(&self, name: &str) -> Stated {
        let list = |l: &[char]| l.iter().map(char::to_string).collect::<Vec<_>>().join("|");
        let rule = match self {
            Change::Edge {
                stem,
                ending,
                to,
                keep,
                earlier,
                left,
                right,
            } => {
                let stem = stem.join("|");
                let (target, to) = if *keep {
                    (format!("{{x:[{stem}]}}"), format!("{{x}}{to}"))
                } else {
                    (stem, to.clone())
                };
                // A vowel, then none until the target.
                let mut before = if earlier.is_empty() {
                    String::new()
                } else {
                    let v = list(earlier);
                    format!("{{[{v}]}}{{![{v}]*}}")
                };
                let set = |s: &Vec<char>| format!("{{[{}]}}", list(s));
                before.extend(left.iter().rev().map(set));
                return Stated {
                    target: format!("{target}{BOUNDARY}{ending}"),
                    to,
                    before,
                    after: right.iter().map(set).collect(),
                    ..Stated::default()
                };
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
            // The class letter, the nearest tier letter before it as
            // written (or none), and the gap between: read on the output.
            Change::Harmony { tier, map } => {
                let mut classes: Vec<char> = map.iter().map(|e| e.0).collect();
                classes.dedup();
                let t = list(tier);
                return Stated {
                    target: format!("{{x:[{}]}}", list(&classes)),
                    to: format!("{{x|{name}[t]}}"),
                    before: format!("{{t:[{t}]?}}{{g:![{t}]*}}"),
                    agree: map
                        .iter()
                        .map(|(x, t, v)| {
                            (
                                x.to_string(),
                                t.map_or(String::new(), String::from),
                                v.to_string(),
                            )
                        })
                        .collect(),
                    iterative: true,
                    ..Stated::default()
                };
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
                return Stated {
                    target: format!("^{literal}{FRONT}{{x:[{}]}}", letters.join("|")),
                    to: format!("{{x|{name}}}"),
                    map,
                    ..Stated::default()
                };
            }
        };
        let [target, to, before, stop] = rule;
        Stated {
            target,
            to,
            before,
            stop,
            ..Stated::default()
        }
    }
}

/// A law as `sound-law` states it: what changes into what, what must come
/// before, and what after stops it; the letters a captured letter becomes
/// (`{x|law}`), or becomes by the letter captured before it
/// (`{x|law[t]}`); and whether it reads its own output.
#[derive(Default)]
struct Stated {
    target: String,
    to: String,
    before: String,
    after: String,
    stop: String,
    map: Vec<(String, String)>,
    agree: Vec<(String, String, String)>,
    iterative: bool,
}

/// The laws compiled, in order.
pub fn compile(changes: &[Change]) -> Option<Program<'static>> {
    let texts: Vec<(String, Stated)> = changes
        .iter()
        .enumerate()
        .map(|(i, c)| {
            let name = format!("law{i}");
            let stated = c.rule(&name);
            (name, stated)
        })
        .collect();
    let pairs: Vec<Vec<(&str, &str)>> = texts
        .iter()
        .map(|(_, s)| {
            s.map
                .iter()
                .map(|(a, b)| (a.as_str(), b.as_str()))
                .collect()
        })
        .collect();
    let triples: Vec<Vec<(&str, &str, &str)>> = texts
        .iter()
        .map(|(_, s)| {
            let agree = s.agree.iter();
            agree
                .map(|(a, k, b)| (a.as_str(), k.as_str(), b.as_str()))
                .collect()
        })
        .collect();
    let maps: Vec<Map<'_>> = texts
        .iter()
        .zip(pairs.iter().zip(&triples))
        .filter_map(
            |((name, _), (pairs, triples))| match (pairs.is_empty(), triples.is_empty()) {
                (false, _) => Some(Map::new(name, pairs)),
                (true, false) => Some(Map::keyed(name, triples)),
                (true, true) => None,
            },
        )
        .collect();
    let rules: Vec<Rule<'_>> = texts
        .iter()
        .map(|(name, s)| {
            let rule = Rule::new(name, &s.target, &s.to).in_context(&s.before, &s.after);
            let rule = if s.stop.is_empty() {
                rule
            } else {
                rule.unless_context("", &s.stop)
            };
            if s.iterative { rule.iterative() } else { rule }
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
    // Harmony first: the other laws see its class letters filled.
    let (harmony, paradigms, assignment) =
        harmony::learn(orthography, words, paradigms, assignment);
    let filled: Vec<Change> = harmony.iter().map(|l| l.change.clone()).collect();
    let pre = (!filled.is_empty()).then(|| compile(&filled)).flatten();
    let pre = pre.as_ref();
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
                .filter_map(|p| reading(orthography, &paradigms, p, w, pre))
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
    let held: Vec<bool> = words.iter().map(|w| held_out(w.letters)).collect();
    let (laws, moved) = greedy(
        &candidates,
        readings,
        &forms,
        &cells_of,
        assignment.clone(),
        &rank,
        &held,
    );
    let kept = laws
        .into_iter()
        .filter(|(l, users, forms)| pays(users.len(), l.breaks, *forms))
        .map(|(mut law, users, _)| {
            law.words = u32::try_from(users.len()).unwrap_or(u32::MAX);
            law.examples = users
                .iter()
                .take(4)
                .filter_map(|&w| words.get(w).map(|w| w.letters.to_string()))
                .collect();
            law
        });
    let mut laws: Vec<Law> = harmony.into_iter().chain(kept).collect();
    plain(orthography, &mut laws);
    // A word stays where it was unless, under the laws kept, the paradigm
    // it moved to builds its forms as well.
    let right = |program: Option<&Program<'_>>, p: u16, w: &Word<'_>| {
        reading(orthography, &paradigms, usize::from(p), w, None).map_or(0, |r| {
            let built = |b: &String| program.map_or_else(|| b.clone(), |p| p.apply(b));
            let forms = w.forms.iter().flatten();
            r.builds
                .iter()
                .zip(forms)
                .filter(|(b, f)| same(&built(b), f))
                .count()
        })
    };
    let settle = |laws: &[Law]| {
        let changes: Vec<Change> = laws.iter().map(|l| l.change.clone()).collect();
        let program = (!laws.is_empty()).then(|| compile(&changes)).flatten();
        // Where the laws last moved it, else where the plain laws did.
        let [plain, last] = &moved;
        let assignment: Vec<u16> = assignment
            .iter()
            .zip(plain.iter().zip(last))
            .zip(words)
            .map(|((&was, (&mid, &now)), w)| {
                let right = |p| right(program.as_ref(), p, w);
                [now, mid]
                    .into_iter()
                    .find(|&p| p != was && right(p) >= right(was))
                    .unwrap_or(was)
            })
            .collect();
        let total: usize = assignment
            .iter()
            .zip(words)
            .map(|(&p, w)| right(program.as_ref(), p, w))
            .sum();
        (program, assignment, total)
    };
    // A law grown context must pay in the end too, with the words where
    // they settled: dropped where the grammar builds as many forms without
    // it.
    let (mut program, mut assignment, mut total) = settle(&laws);
    let grown: Vec<Change> = laws
        .iter()
        .map(|l| l.change.clone())
        .filter(|c| matches!(c, Change::Edge { left, right, .. } if !left.is_empty() || !right.is_empty()))
        .collect();
    for change in grown {
        let without: Vec<Law> = laws
            .iter()
            .filter(|l| l.change != change)
            .cloned()
            .collect();
        let (p, a, t) = settle(&without);
        if t >= total {
            (laws, program, assignment, total) = (without, p, a, t);
        }
    }
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

/// What paradigm `p` builds for word `w`, if it can take it, through the
/// laws `pre` (harmony) if any.
fn reading(
    orthography: &Orthography<'_>,
    paradigms: &[Paradigm<'_>],
    p: usize,
    w: &Word<'_>,
    pre: Option<&Program<'_>>,
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
                .map(|v| {
                    let built = pattern::fill_marked(v, &stems);
                    pre.map_or_else(|| built.clone(), |p| p.apply(&built))
                })
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
/// letters meet), and breaks few their paradigm gave, on the words `held`
/// out too (see [`Tried::kept`]); one at the stem's edge that breaks too
/// many grows context first (see [`grow`]). With each, the words whose
/// forms it gives (by their place in `forms`) and how many forms; and the
/// words' paradigms after the plain laws, and after all.
fn greedy(
    candidates: &[(Change, u32)],
    mut readings: Vec<Vec<Reading>>,
    forms: &[Vec<&str>],
    cells_of: &[Vec<usize>],
    mut assignment: Vec<u16>,
    rank: &dyn Fn(usize) -> (usize, usize),
    held: &[bool],
) -> (Vec<Kept>, [Vec<u16>; 2]) {
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
    let mut credited: Vec<Kept> = Vec::new();
    let mut plain = Vec::new();
    // Every change as it is first; then those at the stem's edge that broke
    // too many, grown a letter of context at a time while they do (Belth
    // 2024 (11), p. 8): a law with context comes after the laws without.
    // Each pass's laws are credited with the words they gave when it ends.
    let mut later: Vec<usize> = Vec::new();
    for growing in [false, true] {
        let todo = if growing {
            std::mem::take(&mut later)
        } else {
            (0..candidates.len()).collect()
        };
        for i in todo {
            let Some((change, _)) = candidates.get(i) else {
                continue;
            };
            let mut change = change.clone();
            let mut tried = None;
            for _ in 0..=if growing { GROWN } else { 0 } {
                let Some(t) = try_law(&change, &index, &readings, forms, &assignment) else {
                    break;
                };
                let kept = t.kept(cells_of, held);
                let grown = (!kept && growing)
                    .then(|| grow(&change, &t, &readings, held))
                    .flatten();
                tried = Some((t, kept));
                match grown {
                    Some(g) => change = g,
                    None => break,
                }
            }
            let growable = |t: &Tried| t.growable(cells_of);
            if !growing
                && matches!(&tried, Some((t, false)) if growable(t))
                && matches!(change, Change::Edge { .. })
            {
                later.push(i);
            }
            let Some((
                Tried {
                    changes,
                    fixed,
                    broken,
                },
                true,
            )) = tried
            else {
                continue;
            };
            let breaks = u32::try_from(broken.len()).unwrap_or(u32::MAX);
            let mut fixed_words: Vec<usize> = fixed.iter().map(|a| a.0).collect();
            fixed_words.dedup();
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
                change,
                words: 0,
                examples: Vec::new(),
                breaks,
            };
            laws.push((law, fixed));
        }
        credited.extend(
            laws.drain(..)
                .map(|l| credit(l, &readings, &assignment, forms)),
        );
        if !growing {
            plain = assignment.clone();
        }
    }
    (credited, [plain, assignment])
}

/// What a law did for the words that moved, once its pass is over: the
/// forms it fixed (`fixed`) that their new paradigm still builds, and their
/// words.
fn credit(
    (law, fixed): (Law, Vec<At>),
    readings: &[Vec<Reading>],
    assignment: &[u16],
    forms: &[Vec<&str>],
) -> Kept {
    let form = |w: usize, k: usize| forms.get(w).and_then(|f| f.get(k)).copied().unwrap_or("");
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
}

/// Whether a word is in the half laws are checked on, not learned from
/// (Rasin & Katzir 2020: what a law must not do, the words it was learned
/// from never pay for).
pub(crate) fn held_out(letters: &str) -> bool {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    letters.hash(&mut h);
    h.finish() % 2 == 1
}

/// How many letters of context a law at the stem's edge may grow.
const GROWN: usize = 2;

/// A law tried on the builds: what it makes of each it changes, those it
/// makes the dictionary's form (`fixed`), and those it makes wrong in the
/// word's own paradigm (`broken`).
struct Tried {
    changes: Vec<(At, String)>,
    fixed: Vec<At>,
    broken: Vec<At>,
}

impl Tried {
    /// Whether the law is kept: it gives forms in more than one cell, pays
    /// for itself, and on the words held out (see [`held_out`]) breaks no
    /// more forms than the tolerance allows.
    fn kept(&self, cells_of: &[Vec<usize>], held: &[bool]) -> bool {
        let (words, cells) = self.gives(cells_of);
        let breaks = u32::try_from(self.broken.len()).unwrap_or(u32::MAX);
        let dev = |l: &[At]| l.iter().filter(|a| held.get(a.0) == Some(&true)).count();
        let (fixed, broken) = (dev(&self.fixed), dev(&self.broken));
        cells >= MIN_CELLS
            && pays(words, breaks, self.fixed.len())
            && harmony::tolerated(
                u32::try_from(fixed + broken).unwrap_or(u32::MAX),
                u32::try_from(broken).unwrap_or(u32::MAX),
            )
    }
}

impl Tried {
    /// How many words and cells it gives forms of.
    fn gives(&self, cells_of: &[Vec<usize>]) -> (usize, usize) {
        let mut words: Vec<usize> = self.fixed.iter().map(|a| a.0).collect();
        words.dedup();
        let mut cells: Vec<usize> = self
            .fixed
            .iter()
            .filter_map(|&(w, _, k)| cells_of.get(w)?.get(k).copied())
            .collect();
        cells.sort_unstable();
        cells.dedup();
        (words.len(), cells.len())
    }

    /// Whether context could make it a law: it gives enough forms, and
    /// breaks some (context only ever gives fewer).
    fn growable(&self, cells_of: &[Vec<usize>]) -> bool {
        let (words, cells) = self.gives(cells_of);
        words >= MIN_WORDS && cells >= MIN_CELLS && !self.broken.is_empty()
    }
}

/// What law `change` does to the builds (`index` finds them by the letters
/// around their boundary): `None` if it doesn't compile.
fn try_law(
    change: &Change,
    index: &HashMap<(String, String), Vec<At>>,
    readings: &[Vec<Reading>],
    forms: &[Vec<&str>],
    assignment: &[u16],
) -> Option<Tried> {
    let program = compile(std::slice::from_ref(change))?;
    let form = |w: usize, k: usize| forms.get(w).and_then(|f| f.get(k)).copied().unwrap_or("");
    let mut at: Vec<At> = index
        .iter()
        .filter(|(k, _)| !matches!(change, Change::Edge { .. }) || change.applies(k, ""))
        .flat_map(|(_, v)| v.iter().copied())
        .collect();
    at.sort_unstable();
    at.dedup();
    let mut tried = Tried {
        changes: Vec::new(),
        fixed: Vec::new(),
        broken: Vec::new(),
    };
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
        let after = program.apply(built);
        if after == *built {
            continue;
        }
        let (was, is) = (same(built, form(w, k)), same(&after, form(w, k)));
        let own = assignment
            .get(w)
            .is_some_and(|&p| usize::from(p) == reading.paradigm);
        if own && was && !is {
            tried.broken.push((w, r, k));
        }
        if !was && is {
            tried.fixed.push((w, r, k));
        }
        tried.changes.push(((w, r, k), after));
    }
    Some(tried)
}

/// A law at the stem's edge that breaks too many forms, with one more
/// letter of context: on the side (before its stem letters, or after its
/// ending's) whose letters best tell the forms it gives from those it
/// breaks, the letters there where it breaks few (one in twenty of what
/// it gives, as [`pays`] asks), learned on the words not held out. `None`
/// for any other law, or where no letter helps.
fn grow(
    change: &Change,
    tried: &Tried,
    readings: &[Vec<Reading>],
    held: &[bool],
) -> Option<Change> {
    let Change::Edge {
        stem,
        ending,
        earlier,
        left,
        right,
        ..
    } = change
    else {
        return None;
    };
    if !earlier.is_empty() {
        return None;
    }
    let built = |&(w, r, k): &At| -> Option<&str> {
        readings.get(w)?.get(r)?.builds.get(k).map(String::as_str)
    };
    // The letter `n` past the context on a side of a build's change.
    let letter = |b: &str, before: bool| -> Option<char> {
        let (head, tail) = b.split_once(BOUNDARY)?;
        if before {
            let alt = stem.iter().find(|s| head.ends_with(s.as_str()))?;
            head.chars().rev().nth(alt.chars().count() + left.len())
        } else {
            tail.chars().nth(ending.chars().count() + right.len())
        }
    };
    let mut best: Option<(usize, bool, Vec<char>)> = None;
    for before in [true, false] {
        let mut counts: std::collections::BTreeMap<char, (usize, usize)> = Default::default();
        let learned = |a: &&At| held.get(a.0) != Some(&true);
        for a in tried.fixed.iter().filter(learned) {
            if let Some(c) = built(a).and_then(|b| letter(b, before)) {
                counts.entry(c).or_default().0 += 1;
            }
        }
        for a in tried.broken.iter().filter(learned) {
            if let Some(c) = built(a).and_then(|b| letter(b, before)) {
                counts.entry(c).or_default().1 += 1;
            }
        }
        #[allow(clippy::cast_precision_loss)]
        let set: Vec<char> = counts
            .iter()
            .filter(|(_, (f, b))| *f > 0 && *b as f64 <= BREAKS * *f as f64)
            .map(|(&c, _)| c)
            .collect();
        let gives: usize = set.iter().filter_map(|c| counts.get(c)).map(|e| e.0).sum();
        if !set.is_empty() && best.as_ref().is_none_or(|b| gives > b.0) {
            best = Some((gives, before, set));
        }
    }
    let (_, before, set) = best?;
    let mut grown = change.clone();
    if let Change::Edge { left, right, .. } = &mut grown {
        if before {
            left.push(set)
        } else {
            right.push(set)
        }
    }
    Some(grown)
}

/// Whether a law pays for itself: it gives `forms` of enough `words`, and
/// breaks few.
#[allow(clippy::cast_precision_loss)]
fn pays(words: usize, breaks: u32, forms: usize) -> bool {
    words >= MIN_WORDS && f64::from(breaks) <= BREAKS * forms as f64
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
                left: Vec::new(),
                right: Vec::new(),
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
                left: Vec::new(),
                right: Vec::new(),
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
    // The changes of a letter of the ending past its first, and the words
    // with them: a form whose ending differs from its build only by that
    // change, in one letter or more, whatever its stem's edge does (another
    // law's).
    let only = |built: &[char], form: &[char], edge: usize| -> Option<(char, char)> {
        let mut differ = built
            .iter()
            .zip(form)
            .skip(edge + 1)
            .filter(|(a, b)| a != b)
            .map(|(a, b)| (*a, *b));
        let first = differ.next()?;
        differ.all(|d| d == first).then_some(first)
    };
    let mut changes: HashMap<(char, char), Vec<usize>> = HashMap::default();
    for (w, built, form, edge) in &pairs {
        if let Some(change) = only(built, form, *edge) {
            let words = changes.entry(change).or_default();
            if words.last() != Some(w) {
                words.push(*w);
            }
        }
    }
    let changes: Vec<((char, char), usize)> = changes
        .into_iter()
        .filter(|(_, words)| words.len() >= MIN_WORDS)
        .map(|(c, words)| (c, words.len()))
        .collect();
    // Each change's examples, in one pass: every place a letter it changes
    // stands in the ending, in a form whose ending no other change touches.
    let mut by_letter: HashMap<char, Vec<usize>> = HashMap::default();
    for (k, ((letter, _), _)) in changes.iter().enumerate() {
        by_letter.entry(*letter).or_default().push(k);
    }
    let mut examples: Vec<HashMap<Example, u32>> = vec![HashMap::default(); changes.len()];
    for (_, built, form, edge) in &pairs {
        let theirs = only(built, form, *edge);
        for (i, (&a, &b)) in built.iter().zip(form).enumerate().skip(*edge) {
            let Some(ks) = by_letter.get(&a) else {
                continue;
            };
            let mut before: Option<Vec<char>> = None;
            for &k in ks {
                let Some(((_, to), _)) = changes.get(k) else {
                    continue;
                };
                if theirs.is_some_and(|c| c != (a, *to)) {
                    continue;
                }
                let before = before.get_or_insert_with(|| {
                    built
                        .get(..i)
                        .unwrap_or_default()
                        .iter()
                        .rev()
                        .copied()
                        .collect()
                });
                if let Some(e) = examples.get_mut(k) {
                    *e.entry((before.clone(), built.get(i + 1).copied(), b == *to))
                        .or_default() += 1;
                }
            }
        }
    }
    let mut out: Vec<(Change, u32)> = changes
        .into_iter()
        .zip(examples)
        .filter_map(|(((letter, to), words), examples)| {
            let [on, off, stop] = tier(&examples)?;
            let words = u32::try_from(words).unwrap_or(u32::MAX);
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
    let mut sets: [Vec<char>; 3] = Default::default();
    loop {
        // How many it predicts right, and how many more with each letter
        // added on a side: added on the tier, a letter before the nearest
        // tier letter becomes the nearest; added to `stop`, it stops the
        // change where it follows.
        let [on, off, stop] = &sets;
        let mut right = 0i64;
        let mut more: HashMap<(char, usize), i64> = HashMap::default();
        for ((before, next, y), n) in &examples {
            let n = i64::from(*n);
            let at = before
                .iter()
                .position(|c| on.contains(c) || off.contains(c));
            let stopped = next.is_some_and(|x| stop.contains(&x));
            let was = at
                .and_then(|i| before.get(i))
                .is_some_and(|c| on.contains(c))
                && !stopped;
            let ok = |p: bool| i64::from(p == *y);
            right += n * ok(was);
            let mut first: Vec<char> = Vec::new();
            for &c in before.get(..at.unwrap_or(before.len())).unwrap_or_default() {
                if !first.contains(&c) {
                    first.push(c);
                    *more.entry((c, 0)).or_default() += n * (ok(!stopped) - ok(was));
                    *more.entry((c, 1)).or_default() += n * (ok(false) - ok(was));
                }
            }
            if let Some(x) = next.filter(|x| !stop.contains(x)) {
                *more.entry((x, 2)).or_default() += n * (ok(false) - ok(was));
            }
        }
        // The best step, the first of the best in letter and side order.
        let mut step: Option<(i64, char, usize)> = None;
        for &l in &letters {
            for side in 0..3 {
                // A letter is on the tier on one side, and stops it or not.
                let free = if side == 2 {
                    !stop.contains(&l)
                } else {
                    !on.contains(&l) && !off.contains(&l)
                };
                let score = right + more.get(&(l, side)).copied().unwrap_or(0);
                if free && step.is_none_or(|s| score > s.0) {
                    step = Some((score, l, side));
                }
            }
        }
        #[allow(clippy::cast_possible_wrap)]
        let enough = MIN_WORDS as i64;
        match step {
            Some((score, l, side)) if score >= right + enough => {
                if let Some(set) = sets.get_mut(side) {
                    set.push(l);
                }
            }
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
            left: Vec::new(),
            right: Vec::new(),
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
            left: Vec::new(),
            right: Vec::new(),
        };
        assert_eq!(class, [(velar.clone(), 5)]);
        // Contraction where a vowel comes earlier in the stem: not in `πλε`.
        let vowels: Vec<char> = "αεηιουω".chars().collect();
        let contract = edge("ε", "ο", "+ου").after(&vowels).expect("an edge law");
        // Context grown on each side: English `-es` after a hushing letter
        // only before `s` (`march+s`, not `march+ing`).
        let grown = Change::Edge {
            stem: vec!["h".into()],
            ending: String::new(),
            to: "h+e".into(),
            keep: false,
            earlier: Vec::new(),
            left: vec![vec!['c', 's']],
            right: vec![vec!['s']],
        };
        let law = |change: &Change| Law {
            change: change.clone(),
            words: 0,
            examples: Vec::new(),
            breaks: 0,
        };
        assert_eq!(law(&grown).to_string(), "h+ → h+e after [c|s] before s");
        for (change, built, after) in [
            (grown.clone(), "march+s", "march+es"),
            (grown.clone(), "march+ing", "march+ing"),
            (grown, "sigh+s", "sigh+s"),
            (velar, "книг+ы", "книг+и"),
            (edge("γ", "σ", "+ξ"), "λεγ+σω", "λε+ξω"),
            (contract.clone(), "φιλε+ομεν", "φιλ+ουμεν"),
            (contract, "πλε+ομεν", "πλε+ομεν"),
            (plain.clone(), "ἐ°φεφιλη+κη", "ἐπεφιλη+κη"),
            (plain, "°λελυ+κα", "°λελυ+κα"),
            (augment.clone(), "ἐ‹ἀγ+ον", "ἠγ+ον"),
            (augment, "ἐ‹λυ+ον", "ἐ‹λυ+ον"),
            (perfect.clone(), "°ἀε‹ἀρ+χα", "ἠρ+χα"),
            // The pluperfect's literal is another.
            (perfect, "ἐ°ἀε‹ἀρ+χειν", "ἐ°ἀε‹ἀρ+χειν"),
            // A stem written without its breathing, inside a compound.
            (unbreathed.clone(), "ἐ‹ᾰρχ+ον", "ηρχ+ον"),
            (unbreathed, "ἐ‹ἀρχ+ον", "ἠρχ+ον"),
        ] {
            let program = compile(std::slice::from_ref(&change)).expect("compiles");
            assert_eq!(program.apply(built), after, "{change:?} on {built}");
        }
    }
}
