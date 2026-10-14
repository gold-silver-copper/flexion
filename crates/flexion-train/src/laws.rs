//! Sound laws: changes where a stem meets its ending (Greek `γ+σ → +ξ`,
//! Russian `[к|г|х]+ы → +и`), of a letter of the ending by the nearest of
//! some letters before it (Sanskrit `न → ण` after `र`), of a class letter
//! an ending writes by the nearest letter of a tier before it as written
//! (harmony: Turkish `-lAr`; or the letter itself, a copy: the Finnish
//! illative `-Vn`), or of a copied
//! letter (Greek `φ` copied as `π`: `πεφίληκα`), or where a literal before
//! the stem meets it (the Greek augment lengthens a vowel: `ἐ‹ἀ → ἠ`,
//! `ἤγον`), or inside a stem that alternates, by what follows it (Finnish
//! consonant gradation, `katu`, `kadun`). They are found where a word's
//! paradigm, or a more general one, would build its forms but for such a
//! change, and kept when they fix many
//! forms and break few, on the words they are learned from and on words
//! held out (Rasin & Katzir 2020). A change at the stem's edge that breaks
//! too many is tried again with a letter more of context on whichever side
//! best tells the forms it gives from those it breaks (Belth 2024 (11),
//! p. 8). The words they explain join the more general paradigm, and
//! paradigms left without words go.
//!
//! Which stems alternate is the word's (Finnish `auto`, `auton` keeps its
//! `t`): a more general paradigm has a twin whose patterns end in
//! [`ALTERNATES`] (a rule feature, Chomsky & Halle 1968 ch. 8; Koskenniemi
//! 1983's morphophonemes), and only its forms go through the laws inside
//! the stem. Such a law's context is a decision tree on the letters after
//! the change (Quinlan 1986), read as any letter, vowel or not, or letter
//! by letter, whichever gives as many forms; its shifts after different
//! letters are one law where one context makes them all (`t → d after
//! [a|o|u|h]; t → n after n; k → ∅ after k; …`), applying at once (a
//! changed letter is marked [`CHANGED`]: `voitto`, `voiton`, not
//! `voidon`).

mod harmony;

use crate::HashMap;
use flexion::pattern::{self, ALTERNATES, BOUNDARY, CHANGED, COPIED, FRONT};
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
    /// Inside a stem that alternates (its paradigm's patterns end in
    /// [`ALTERNATES`]): each of `shifts` where what follows it begins as one
    /// of `after` says, a set of letters per place (the stem's end `+`
    /// among them, `$` the form's end): Finnish consonant gradation, `t → d
    /// after [a|o|u|h]; t → n after n; … before [o|u]+n$` (`katu`, `kadun`,
    /// `ranta`, `rannan`; Koskenniemi 1983 pp. 73–75).
    Inner {
        shifts: Vec<Shift>,
        after: Vec<Vec<Vec<char>>>,
    },
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

/// One change of [`Change::Inner`]: after one of the letters, the first
/// string becomes the second (`t → d` after `[a|o|u|h]`).
pub type Shift = (Vec<char>, String, String);

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
                    let sep = if k == 0 { "" } else { "; " };
                    // A copy: `V → that letter; a after none`.
                    let mine = map.iter().filter(|e| e.0 == *class);
                    if mine.clone().all(|&(_, t, v)| t.is_none_or(|t| t == v)) {
                        let none = mine.filter(|e| e.1.is_none()).map(|e| e.2);
                        write!(f, "{sep}{class} → that letter")?;
                        for v in none {
                            write!(f, "; {v} after none")?;
                        }
                        continue;
                    }
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
                    write!(f, "{sep}{class} → {}", values.join("; "))?;
                }
                write!(f, " (the nearest of {} before it, as written)", list(tier))
            }
            Change::Copy { letter, to } => write!(f, "copy of {letter} → {to}"),
            // `t → d after [a|o|u|h], before [o|u]+n$, [o|u]+ss`.
            Change::Inner { shifts, after } => {
                // `.`: any letter; `[^a|e]`: any but these.
                let set = |s: &[char]| match s {
                    ['!'] => ".".to_string(),
                    ['!', but @ ..] => format!("[^{}]", list(but).replace(", ", "|")),
                    [one] => one.to_string(),
                    many => format!("[{}]", list(many).replace(", ", "|")),
                };
                let or = |s: &str| if s.is_empty() { "∅".to_string() } else { s.to_string() };
                let shifts: Vec<String> = shifts
                    .iter()
                    .map(|(left, from, to)| format!("{} → {} after {}", or(from), or(to), set(left)))
                    .collect();
                write!(f, "{} in alternating stems ({ALTERNATES})", shifts.join("; "))?;
                let paths: Vec<String> = after
                    .iter()
                    .map(|p| p.iter().map(|s| set(s)).collect())
                    .collect();
                if !paths.is_empty() {
                    write!(f, ", before {}", paths.join(", "))?;
                }
                Ok(())
            }
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
            Change::Inner { shifts, .. } => {
                built.ends_with(ALTERNATES)
                    && built.split_once(BOUNDARY).is_some_and(|(stem, _)| {
                        shifts.iter().any(|(_, from, _)| stem.contains(from.as_str()))
                    })
            }
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
            // One shift: its letters; several: a capture, which the law's
            // map gives by the letter before it (as it was where no shift
            // has that letter before it). Before each path's sets, then the
            // rest of the form, which an alternating stem's mark ends.
            Change::Inner { shifts, after } => {
                let (target, to, before, agree) = match shifts.as_slice() {
                    [(left, from, to)] => (
                        from.clone(),
                        format!("{to}{CHANGED}"),
                        format!("{{[{}]}}", list(left)),
                        Vec::new(),
                    ),
                    many => {
                        let mut froms: Vec<&str> = many.iter().map(|s| s.1.as_str()).collect();
                        froms.sort_unstable();
                        froms.dedup();
                        let mut lefts: Vec<char> = many.iter().flat_map(|s| s.0.clone()).collect();
                        lefts.sort_unstable();
                        lefts.dedup();
                        let agree: Vec<(String, String, String)> = froms
                            .iter()
                            .flat_map(|&f| lefts.iter().map(move |&l| (f, l)))
                            .map(|(f, l)| {
                                let to = many
                                    .iter()
                                    .find(|s| s.1 == f && s.0.contains(&l))
                                    .map_or(f.to_string(), |s| format!("{}{CHANGED}", s.2));
                                (f.to_string(), l.to_string(), to)
                            })
                            .collect();
                        (
                            format!("{{x:[{}]}}", froms.join("|")),
                            format!("{{x|{name}[l]}}"),
                            format!("{{l:[{}]}}", list(&lefts)),
                            agree,
                        )
                    }
                };
                let rights: Vec<String> = after
                    .iter()
                    .map(|path| {
                        let mut right = String::new();
                        for set in path {
                            match set.as_slice() {
                                ['$'] => break,
                                [BOUNDARY] => right.push(BOUNDARY),
                                ['!', but @ ..] => {
                                    let marks = [BOUNDARY, ALTERNATES, CHANGED, COPIED, FRONT];
                                    let but: Vec<char> = marks.iter().chain(but).copied().collect();
                                    right.push_str(&format!("{{![{}]}}", list(&but)));
                                }
                                letters => right.push_str(&format!("{{[{}]}}", list(letters))),
                            }
                        }
                        // The change is in the stem: the stem's end follows.
                        if path.last().is_some_and(|s| s.as_slice() == ['$']) {
                            right.push(ALTERNATES);
                        } else {
                            if !path.iter().any(|s| s.as_slice() == [BOUNDARY]) {
                                right.push_str(&format!("{{![{BOUNDARY}]*}}{BOUNDARY}"));
                            }
                            right.push_str(&format!("{{![{ALTERNATES}]*}}{ALTERNATES}"));
                        }
                        right.push('$');
                        right
                    })
                    .collect();
                // No context: nowhere (the mark ends a form once).
                let (after, or) = match rights.split_first() {
                    Some((first, rest)) => (first.clone(), rest.to_vec()),
                    None => (format!("{ALTERNATES}{ALTERNATES}$"), Vec::new()),
                };
                return Stated {
                    target,
                    to,
                    before,
                    after,
                    or,
                    agree,
                    ..Stated::default()
                };
            }
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
    /// What may follow instead of `after`.
    or: Vec<String>,
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
    let ors: Vec<Vec<&str>> = texts
        .iter()
        .map(|(_, s)| s.or.iter().map(String::as_str).collect())
        .collect();
    let rules: Vec<Rule<'_>> = texts
        .iter()
        .zip(&ors)
        .map(|((name, s), or)| {
            let rule = Rule::new(name, &s.target, &s.to)
                .in_context(&s.before, &s.after)
                .or_before(or);
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
    // ones, the commoner. A paradigm's twin whose stems alternate is as
    // general as it.
    let mut rank: Vec<(usize, usize)> = paradigms
        .iter()
        .enumerate()
        .map(|(i, p)| (p.lemma.chars().filter(|&c| c != '~').count(), i))
        .collect();
    let mut neighbours = neighbours(&paradigms, cells, |p| {
        rank.get(p).copied().unwrap_or((usize::MAX, p))
    });
    let originals = paradigms.len();
    let (paradigms, twins) = twins(orthography, words, paradigms, &assignment, pre, &rank);
    for (twin, of, _) in &twins {
        rank.push(rank.get(*of).copied().unwrap_or((usize::MAX, *twin)));
    }
    for (twin, _, from) in &twins {
        for &p in from {
            if let Some(n) = neighbours.get_mut(p) {
                n.push(*twin);
            }
        }
    }
    let rank = |p: usize| rank.get(p).copied().unwrap_or((usize::MAX, p));
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
            // A twin only where its stems alternate as the word's do: it
            // builds every form, or does but for a change inside the stem,
            // some so.
            let alternate = |r: &Reading| {
                let mut wrong = r
                    .builds
                    .iter()
                    .zip(w.forms.iter().flatten())
                    .filter(|(b, f)| !same(b, f))
                    .peekable();
                wrong.peek().is_some() && wrong.all(|(b, f)| !inners(b, f).is_empty())
            };
            std::iter::once(own)
                .chain(neighbours.get(own).into_iter().flatten().copied())
                .filter_map(|p| reading(orthography, &paradigms, p, w, pre))
                .filter(|r| r.paradigm < originals || alternate(r))
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
    // A change inside a stem after several letters is tried after them all
    // only.
    let grouped = lefts(&support);
    let pair = |c: &Change| match c {
        Change::Inner { shifts, .. } => match shifts.as_slice() {
            [(_, from, to)] => Some((from.clone(), to.clone())),
            _ => None,
        },
        _ => None,
    };
    let after_several: crate::HashSet<(String, String)> =
        grouped.iter().filter_map(|(c, _)| pair(c)).collect();
    support.retain(|c, _| pair(c).is_none_or(|p| !after_several.contains(&p)));
    candidates.extend(grouped);
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
        &Halves {
            held: &held,
            vowels: &vowels,
            cells: &cells_of,
        },
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
    // A law inside alternating stems pays in bits (see `crate::bits`): the
    // grammar is shorter with it, its paradigms that have words, its laws
    // and the forms it gets wrong (listed) counted, than without it.
    let alphabet: crate::HashSet<char> = forms.iter().flatten().flat_map(|f| f.chars()).collect();
    let prices = crate::bits::Prices::new(alphabet.len(), cells);
    let all: usize = forms.iter().map(Vec::len).sum();
    let letters: usize = forms.iter().flatten().map(|f| f.chars().count()).sum();
    #[allow(clippy::cast_precision_loss)]
    let listed = prices.cell + prices.letter * (letters as f64 / all.max(1) as f64 + 1.0);
    let cost = |laws: &[Law], assignment: &[u16], right: usize| -> f64 {
        let mut used = vec![false; paradigms.len()];
        for &a in assignment {
            if let Some(u) = used.get_mut(usize::from(a)) {
                *u = true;
            }
        }
        let tables: f64 = paradigms
            .iter()
            .zip(&used)
            .filter(|(_, u)| **u)
            .map(|(p, _)| {
                let cells: f64 = p.table.split('\t').map(|c| prices.text(c)).sum();
                crate::bits::ITEM + cells
            })
            .sum();
        let stated: f64 = laws
            .iter()
            .map(|l| crate::bits::ITEM + prices.text(&l.to_string()))
            .sum();
        #[allow(clippy::cast_precision_loss)]
        let wrong = all.saturating_sub(right) as f64 * listed;
        tables + stated + wrong
    };
    let inner: Vec<Change> = laws
        .iter()
        .map(|l| l.change.clone())
        .filter(|c| matches!(c, Change::Inner { .. }))
        .collect();
    for change in inner {
        let without: Vec<Law> = laws
            .iter()
            .filter(|l| l.change != change)
            .cloned()
            .collect();
        let (p, a, t) = settle(&without);
        if cost(&without, &a, t) <= cost(&laws, &assignment, total) {
            (laws, program, assignment, total) = (without, p, a, t);
        }
    }
    let before: Vec<String> = paradigms
        .iter()
        .take(originals)
        .map(|p| p.name.to_string())
        .collect();
    let (mut paradigms, assignment) = drop_empty(paradigms, assignment);
    // A twin with words is named after the word a paradigm it took the
    // words of was named after (commonest first), else its first word no
    // paradigm is named after.
    let mut names: crate::HashSet<String> =
        paradigms.iter().map(|p| p.name.to_string()).collect();
    for (p, par) in paradigms.iter_mut().enumerate() {
        if !par.lemma.ends_with(ALTERNATES) {
            continue;
        }
        let mine: Vec<&str> = assignment
            .iter()
            .zip(words)
            .filter(|(a, _)| usize::from(**a) == p)
            .map(|(_, w)| w.letters)
            .collect();
        let named = before.iter().find(|n| mine.contains(&n.as_str()));
        let first = mine.iter().find(|n| !names.contains(**n));
        if let Some(name) = named
            .map(String::as_str)
            .filter(|n| !names.contains(*n))
            .or(first.copied())
        {
            names.insert(name.to_string());
            par.name = Cow::Owned(name.to_string());
        }
    }
    (laws, program, paradigms, assignment)
}

/// A twin (see [`twins`]): its place, the paradigm it is a twin of, and
/// the paradigms it may take the words of.
type Twin = (usize, usize, Vec<usize>);

/// The words of a paradigm its more general paradigms are tried on, to find
/// those whose twin with alternating stems builds them (see [`twins`]).
const SAMPLE: usize = 3;
/// The share of those words' forms such a twin must build, or build but
/// for a change inside the stem.
const TWIN_SHARE: f64 = 0.9;

/// Twins of more general paradigms whose stems alternate (see
/// [`ALTERNATES`]): every pattern ends in the mark, and the laws that hold
/// only for such stems change them. A paradigm gets a twin where, for its
/// first words, the twin builds nearly every form, or does but for one
/// change inside the stem (`veto`, `vedon` of `taso`, `tason`: `t → d`),
/// some forms differing so. The paradigms with the twins after them, and
/// per twin its place, the paradigm it is a twin of, and the paradigms it
/// may take the words of (at most [`NEIGHBOURS`] each, the closest).
fn twins<'s>(
    orthography: &Orthography<'_>,
    words: &[Word<'_>],
    mut paradigms: Vec<Paradigm<'s>>,
    assignment: &[u16],
    pre: Option<&Program<'_>>,
    rank: &[(usize, usize)],
) -> (Vec<Paradigm<'s>>, Vec<Twin>) {
    let marked = |p: &Paradigm<'_>| -> Paradigm<'s> {
        let mark = |cell: &str| -> String {
            if cell.is_empty() || cell == flexion::DERIVED {
                return cell.to_string();
            }
            let variants: Vec<String> = pattern::variants(cell)
                .map(|v| format!("{v}{ALTERNATES}"))
                .collect();
            variants.join("|")
        };
        let table: Vec<String> = p.table.split('\t').map(mark).collect();
        Paradigm {
            name: Cow::Owned(format!("{}{ALTERNATES}", p.name)),
            words: 0,
            lemma: Cow::Owned(format!("{}{ALTERNATES}", p.lemma)),
            parent: None,
            inherent: Cow::Owned(p.inherent.to_vec()),
            table: Cow::Owned(table.join("\t")),
            stems: Cow::Owned(p.stems.to_string()),
            reads: None,
        }
    };
    let originals = paradigms.len();
    // Of paradigms of one stem piece: an alternating stem is the word's.
    let one = |q: usize| paradigms.get(q).is_some_and(|p| pattern::pieces(&p.lemma) == 1);
    let twins: Vec<Paradigm<'s>> = paradigms.iter().map(marked).collect();
    let mut members: Vec<Vec<usize>> = vec![Vec::new(); originals];
    for (w, &p) in assignment.iter().enumerate() {
        if let Some(m) = members.get_mut(usize::from(p))
            && m.len() < SAMPLE
        {
            m.push(w);
        }
    }
    // Per paradigm, the twins building its first words' forms, closest
    // first.
    let mut close: Vec<Vec<((usize, usize), usize)>> = vec![Vec::new(); originals];
    let all: Vec<Paradigm<'s>> = paradigms.iter().cloned().chain(twins.iter().cloned()).collect();
    for (p, mine) in members.iter().enumerate() {
        let Some(&me) = rank.get(p) else { continue };
        for q in (0..originals).filter(|&q| one(q) && rank.get(q).is_some_and(|r| *r < me)) {
            let (mut right, mut changed, mut forms) = (0usize, 0usize, 0usize);
            for &w in mine {
                let Some(word) = words.get(w) else { continue };
                let Some(r) = reading(orthography, &all, originals + q, word, pre) else {
                    forms = usize::MAX / 2;
                    break;
                };
                for (built, want) in r.builds.iter().zip(word.forms.iter().flatten()) {
                    forms += 1;
                    if same(built, want) {
                        right += 1;
                    } else if !inners(built, want).is_empty() {
                        changed += 1;
                    }
                }
            }
            #[allow(clippy::cast_precision_loss)]
            let enough = (right + changed) as f64 >= TWIN_SHARE * forms as f64;
            if changed > 0
                && forms > 0
                && enough
                && let Some(c) = close.get_mut(p)
            {
                c.push(((right + changed, right), q));
            }
        }
    }
    let mut from: Vec<Vec<usize>> = vec![Vec::new(); originals];
    for (p, mut c) in close.into_iter().enumerate() {
        c.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
        for (_, q) in c.into_iter().take(NEIGHBOURS) {
            if let Some(f) = from.get_mut(q) {
                f.push(p);
            }
        }
    }
    let mut out = Vec::new();
    for (q, (twin, from)) in twins.into_iter().zip(from).enumerate() {
        if !from.is_empty() {
            out.push((paradigms.len(), q, from));
            paradigms.push(twin);
        }
    }
    (paradigms, out)
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
    halves: &Halves<'_>,
) -> (Vec<Kept>, [Vec<u16>; 2]) {
    let held = halves.held;
    // Builds by the letters around their boundary.
    let mut index: HashMap<(String, String), Vec<At>> = HashMap::default();
    let mut alternating: HashMap<String, Vec<At>> = HashMap::default();
    for (w, rs) in readings.iter().enumerate() {
        for (r, reading) in rs.iter().enumerate() {
            for (k, built) in reading.builds.iter().enumerate() {
                index.entry(window(built)).or_default().push((w, r, k));
                if built.ends_with(ALTERNATES) {
                    alternating.entry(stem_end(built)).or_default().push((w, r, k));
                }
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
            // A shift inside a stem learns its context first (it gives
            // nothing without), and joins a law of such shifts whose context
            // gives as many of its forms (consonant gradation: one law).
            let mut into = None;
            if let Change::Inner { shifts, after } = &change
                && let [shift] = shifts.as_slice()
                && after.is_empty()
            {
                let own = context(&change, &alternating, &readings, forms, halves);
                let gives = own.as_ref().map_or(0, |own| {
                    try_law(own, &index, &alternating, &readings, forms, &assignment)
                        .filter(|t| t.kept(cells_of, held))
                        .map_or(0, |t| t.fixed.len())
                });
                for (j, (law, _)) in laws.iter().enumerate() {
                    let Change::Inner {
                        shifts: theirs,
                        after: paths,
                    } = &law.change
                    else {
                        continue;
                    };
                    // Not one that changes the same letters after the same
                    // letter, and none inserting.
                    let clash = |s: &Shift| s.1 == shift.1 && s.0.iter().any(|l| shift.0.contains(l));
                    let inserts = |s: &Shift| s.1.is_empty();
                    if theirs.iter().any(|s| clash(s) || inserts(s)) || inserts(shift) {
                        continue;
                    }
                    let trial = Change::Inner {
                        shifts: vec![shift.clone()],
                        after: paths.clone(),
                    };
                    let t = try_law(&trial, &index, &alternating, &readings, forms, &assignment);
                    #[allow(clippy::cast_precision_loss)]
                    let enough = |t: &Tried| t.fixed.len() as f64 >= (1.0 - BREAKS) * gives as f64;
                    if let Some(t) = t.filter(|t| t.kept(cells_of, held) && enough(t)) {
                        (into, tried, change) = (Some(j), Some((t, true)), trial);
                        break;
                    }
                }
                if into.is_none() {
                    match own {
                        Some(c) => change = c,
                        None => continue,
                    }
                }
            }
            for _ in 0..=if growing { GROWN } else { 0 } {
                if into.is_some() {
                    break;
                }
                let Some(t) = try_law(&change, &index, &alternating, &readings, forms, &assignment)
                else {
                    break;
                };
                let kept = t.kept(cells_of, held);
                let grown = (!kept && growing)
                    .then(|| grow(&change, &t, &readings, halves))
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
                if after.ends_with(ALTERNATES) {
                    alternating.entry(stem_end(&after)).or_default().push((w, r, k));
                }
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
            // Joined: the law makes its shift too.
            if let Some((law, theirs)) = into.and_then(|j| laws.get_mut(j)) {
                if let (Change::Inner { shifts, .. }, Change::Inner { shifts: mine, .. }) =
                    (&mut law.change, change)
                {
                    shifts.extend(mine);
                }
                law.breaks = law.breaks.saturating_add(breaks);
                theirs.extend(fixed);
                continue;
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
    users.sort_unstable();
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

/// Which words laws are learned from and which they are checked on (see
/// [`held_out`]), the vowels their letters have, and per word the cells of
/// its forms.
struct Halves<'a> {
    held: &'a [bool],
    vowels: &'a [char],
    cells: &'a [Vec<usize>],
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
    alternating: &HashMap<String, Vec<At>>,
    readings: &[Vec<Reading>],
    forms: &[Vec<&str>],
    assignment: &[u16],
) -> Option<Tried> {
    let program = compile(std::slice::from_ref(change))?;
    let form = |w: usize, k: usize| forms.get(w).and_then(|f| f.get(k)).copied().unwrap_or("");
    let inner = matches!(change, Change::Inner { .. });
    let mut at: Vec<At> = if let Change::Inner { shifts, .. } = change {
        let near = |end: &String| {
            shifts.iter().any(|(left, from, _)| {
                left.iter().any(|l| end.contains(&format!("{l}{from}")))
            })
        };
        alternating
            .iter()
            .filter(|(end, _)| near(end))
            .flat_map(|(_, v)| v.iter().copied())
            .collect()
    } else {
        index
            .iter()
            .filter(|(k, _)| !matches!(change, Change::Edge { .. }) || change.applies(k, ""))
            .flat_map(|(_, v)| v.iter().copied())
            .collect()
    };
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
        // A law inside alternating stems breaks the forms of the twins
        // too: the words it is for may join them.
        if (own || inner) && was && !is {
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
    halves: &Halves<'_>,
) -> Option<Change> {
    let held = halves.held;
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

/// The most letters after a change inside a stem its context reads.
const DEPTH: usize = 6;
/// The most letters between a change inside a stem and the stem's end.
const NEAR: usize = 2;

/// A change inside alternating stems (see [`Change::Inner`]) with the
/// context it is made in: what follows it where it gives forms and not
/// where it breaks them, in the builds of alternating stems (`alternating`
/// finds them by their stem's end) of the words not held out. The letters
/// after the change (the stem's end `+` among them, `$` the form's end)
/// are read one place at a time while they don't yet tell the two apart (a
/// decision tree, Quinlan 1986), the places where they do kept as paths.
/// Finnish weak grades: `kadun` (`katu+nˇ`), `kadussa`, not `katuun`,
/// `katumme`. `None` where no path gives enough forms.
fn context(
    change: &Change,
    alternating: &HashMap<String, Vec<At>>,
    readings: &[Vec<Reading>],
    forms: &[Vec<&str>],
    halves: &Halves<'_>,
) -> Option<Change> {
    let Change::Inner { shifts, .. } = change else {
        return None;
    };
    let [(left, from, to)] = shifts.as_slice() else {
        return None;
    };
    let (from, to): (Vec<char>, Vec<char>) = (from.chars().collect(), to.chars().collect());
    let mut examples: Vec<(Vec<char>, bool, usize, usize)> = Vec::new();
    let near = |end: &String| {
        left.iter().any(|l| {
            let mut x = String::from(*l);
            x.extend(&from);
            end.contains(&x)
        })
    };
    for &(w, r, k) in alternating
        .iter()
        .filter(|(end, _)| near(end))
        .flat_map(|(_, v)| v)
    {
        let (Some(built), Some(want)) = (
            readings.get(w).and_then(|rs| rs.get(r)?.builds.get(k)),
            forms.get(w).and_then(|f| f.get(k)),
        ) else {
            continue;
        };
        if halves.held.get(w) == Some(&true) {
            continue;
        }
        let seq: Vec<char> = built
            .chars()
            .filter(|&c| c != COPIED && c != FRONT)
            .map(|c| if c == ALTERNATES { '$' } else { c })
            .collect();
        let Some(edge) = seq.iter().position(|&c| c == BOUNDARY) else {
            continue;
        };
        // Its last place in the stem, near the stem's end, not one a law
        // has changed.
        let Some(at) = (1..=edge.saturating_sub(from.len())).rev().find(|&i| {
            seq.get(i - 1).is_some_and(|c| left.contains(c))
                && seq.get(i..i + from.len()) == Some(from.as_slice())
                && seq.get(i + from.len()) != Some(&CHANGED)
        }) else {
            continue;
        };
        if edge - at - from.len() > NEAR {
            continue;
        }
        let (Some(head), Some(rest)) = (seq.get(..at), seq.get(at + from.len()..)) else {
            continue;
        };
        let letters = |s: &mut dyn Iterator<Item = &char>| -> String {
            s.filter(|&&c| c != BOUNDARY && c != '$' && c != CHANGED).collect()
        };
        let was = letters(&mut seq.iter()) == **want;
        let is = letters(&mut head.iter().chain(&to).chain(rest)) == **want;
        if was != is {
            let cell = halves.cells.get(w).and_then(|c| c.get(k)).copied().unwrap_or(k);
            examples.push((rest.iter().take(DEPTH).copied().collect(), is, w, cell));
        }
    }
    let places: Vec<Place<'_>> = examples
        .iter()
        .map(|(after, gives, word, cell)| Place {
            after,
            gives: *gives,
            word: *word,
            cell: *cell,
        })
        .collect();
    let mut after = Vec::new();
    flatten(&decide(&places, 0, halves.vowels), &mut Vec::new(), &mut after);
    (!after.is_empty()).then(|| Change::Inner {
        shifts: shifts.clone(),
        after,
    })
}

/// A place a change inside a stem could be made (see [`context`]): what
/// follows it, whether the change gives the form there (else breaks it),
/// and the form's word and cell.
#[derive(Clone, Copy)]
struct Place<'a> {
    after: &'a [char],
    gives: bool,
    word: usize,
    cell: usize,
}

/// A node of the tree [`context`] grows: the change made here, not made, or
/// decided by the next letter (a set of letters each).
#[derive(PartialEq)]
enum Node {
    Made,
    Not,
    By(Vec<(Vec<char>, Node)>),
}

/// Whether letter `c` is one of `set` (a set `!` and letters: any letter
/// but those, the form's end and marks not letters).
fn within(set: &[char], c: char) -> bool {
    match set {
        ['!', but @ ..] => c != '$' && !pattern::is_mark(c) && !but.contains(&c),
        letters => letters.contains(&c),
    }
}

/// The letters of either set (see [`within`]).
fn union(a: &[char], b: &[char]) -> Vec<char> {
    let mut out: Vec<char> = match (a, b) {
        (['!', x @ ..], ['!', y @ ..]) => std::iter::once('!')
            .chain(x.iter().copied().filter(|c| y.contains(c)))
            .collect(),
        (['!', but @ ..], letters) | (letters, ['!', but @ ..]) => std::iter::once('!')
            .chain(but.iter().copied().filter(|c| !letters.contains(c)))
            .collect(),
        (x, y) => x.iter().chain(y).copied().collect(),
    };
    out.sort_unstable();
    out.dedup();
    out
}

/// The places of `places` whose letter at `at` is one of `set`.
fn among<'a>(places: &[Place<'a>], at: usize, set: &[char]) -> Vec<Place<'a>> {
    let has = |p: &&Place<'_>| p.after.get(at).is_some_and(|&c| within(set, c));
    places.iter().filter(has).copied().collect()
}

impl Node {
    /// How many forms it gives.
    fn gives(&self, places: &[Place<'_>], at: usize) -> usize {
        match self {
            Node::Made => places.iter().filter(|p| p.gives).count(),
            Node::Not => 0,
            Node::By(children) => children
                .iter()
                .map(|(set, n)| n.gives(&among(places, at, set), at + 1))
                .sum(),
        }
    }
}

/// Whether the change may be made at all of `places`: it breaks few of the
/// forms it gives (as [`pays`] asks), and, in each cell, the forms of few
/// of the words it gives forms of (a lexical exception), never a cell's
/// forms it gives none of (Finnish `vetoineen`, among the weak `vedoin`).
fn settled(places: &[Place<'_>]) -> bool {
    let gives = places.iter().filter(|p| p.gives).count();
    #[allow(clippy::cast_precision_loss)]
    let few = |breaks: usize, gives: usize| breaks as f64 <= BREAKS * gives as f64;
    if !few(places.len() - gives, gives) {
        return false;
    }
    let mut by: std::collections::BTreeMap<usize, (Vec<usize>, Vec<usize>)> = Default::default();
    for p in places {
        let (g, b) = by.entry(p.cell).or_default();
        if p.gives { g } else { b }.push(p.word);
    }
    by.into_values().all(|(mut g, mut b)| {
        g.sort_unstable();
        g.dedup();
        b.sort_unstable();
        b.dedup();
        b.is_empty() || (!g.is_empty() && (b.len() <= 1 || few(b.len(), g.len())))
    })
}

/// The tree on `places` from place `at`: made where [`settled`] and giving
/// at least [`MIN_WORDS`] forms; not where it gives fewer, or [`DEPTH`]
/// letters don't tell. A place is read as any letter, as a vowel (of
/// `vowels`) or any other letter, or letter by letter (the stem's end and
/// the form's end apart each time): the coarsest that gives as many as
/// any.
fn decide(places: &[Place<'_>], at: usize, vowels: &[char]) -> Node {
    let gives = places.iter().filter(|p| p.gives).count();
    if gives < MIN_WORDS {
        return Node::Not;
    }
    if settled(places) {
        return Node::Made;
    }
    if at >= DEPTH {
        return Node::Not;
    }
    let split = |sets: Vec<Vec<char>>| -> Node {
        let children: Vec<(Vec<char>, Node)> = sets
            .into_iter()
            .filter(|set| !set.is_empty())
            .map(|set| {
                let node = decide(&among(places, at, &set), at + 1, vowels);
                (set, node)
            })
            .filter(|(_, n)| *n != Node::Not)
            .collect();
        if children.is_empty() {
            Node::Not
        } else {
            Node::By(children)
        }
    };
    let mut letters: Vec<char> = places.iter().filter_map(|p| p.after.get(at).copied()).collect();
    letters.sort_unstable();
    letters.dedup();
    let marks: Vec<char> = letters
        .iter()
        .copied()
        .filter(|&c| c == '$' || c == BOUNDARY)
        .collect();
    let apart = || marks.iter().map(|&c| vec![c]);
    let consonants: Vec<char> = std::iter::once('!').chain(vowels.iter().copied()).collect();
    let readings = [
        apart().chain([vec!['!']]).collect(),
        apart().chain([vowels.to_vec(), consonants]).collect(),
        letters.iter().map(|&c| vec![c]).collect(),
    ]
    .map(split);
    let given: Vec<usize> = readings.iter().map(|n| n.gives(places, at)).collect();
    let most = given.iter().copied().max().unwrap_or(0);
    let coarsest = given.iter().position(|&g| g == most).unwrap_or(0);
    readings.into_iter().nth(coarsest).unwrap_or(Node::Not)
}

/// The paths to where `node` makes the change, after `prefix`: sets whose
/// rest is the same one set (the stem's end and the form's end apart).
fn flatten(node: &Node, prefix: &mut Vec<Vec<char>>, out: &mut Vec<Vec<Vec<char>>>) {
    match node {
        Node::Not => {}
        Node::Made => out.push(prefix.clone()),
        Node::By(children) => {
            let alone = |c: &char| *c == '$' || *c == BOUNDARY;
            let mut groups: Vec<(Vec<char>, &Node)> = Vec::new();
            for (set, n) in children {
                let apart = set.iter().any(alone);
                match groups
                    .iter_mut()
                    .find(|(g, m)| *m == n && !apart && !g.iter().any(alone))
                {
                    Some((g, _)) => *g = union(g, set),
                    None => groups.push((set.clone(), n)),
                }
            }
            for (mut set, n) in groups {
                set.sort_unstable();
                prefix.push(set);
                flatten(n, prefix, out);
                prefix.pop();
            }
        }
    }
}

/// Whether a law pays for itself: it gives `forms` of enough `words`, and
/// breaks few.
#[allow(clippy::cast_precision_loss)]
fn pays(words: usize, breaks: u32, forms: usize) -> bool {
    words >= MIN_WORDS && f64::from(breaks) <= BREAKS * forms as f64
}

/// The letters of a build's stem a change inside it may touch, with the one
/// before them: the last [`NEAR`] and four more.
fn stem_end(built: &str) -> String {
    let stem: Vec<char> = built
        .split_once(BOUNDARY)
        .map_or("", |s| s.0)
        .chars()
        .filter(|&c| c != COPIED && c != FRONT)
        .collect();
    let skip = stem.len().saturating_sub(NEAR + 4);
    stem.get(skip..).unwrap_or_default().iter().collect()
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
        .chain(inners(built, want))
        .collect()
}

/// The change inside an alternating stem (see [`Change::Inner`]) that
/// turns `built` into `want`: the one place they differ, of at most two
/// letters each, in the stem, after the letter before it (`veto+nˇ`,
/// `vedon`: `t → d` after `e`; `paikk+anˇ`, `paikan`: `k → ∅` after `k`).
/// Its context is learned later (see [`paths`]).
fn inners(built: &str, want: &str) -> Vec<Change> {
    if !built.ends_with(ALTERNATES) {
        return Vec::new();
    }
    let letters: Vec<char> = built.chars().filter(|&c| !pattern::is_mark(c)).collect();
    let Some(edge) = built
        .chars()
        .filter(|&c| c == BOUNDARY || !pattern::is_mark(c))
        .position(|c| c == BOUNDARY)
    else {
        return Vec::new();
    };
    let want: Vec<char> = want.chars().collect();
    let head = letters.iter().zip(&want).take_while(|(a, b)| a == b).count();
    let most = letters.len().min(want.len()) - head;
    let tail = letters
        .iter()
        .rev()
        .zip(want.iter().rev())
        .take(most)
        .take_while(|(a, b)| a == b)
        .count();
    let (Some(from), Some(to), Some(&left)) = (
        letters.get(head..letters.len() - tail),
        want.get(head..want.len() - tail),
        head.checked_sub(1).and_then(|h| letters.get(h)),
    ) else {
        return Vec::new();
    };
    // In the stem, at most [`NEAR`] letters before its end.
    let end = letters.len() - tail;
    let inside = end <= edge && edge - end <= NEAR;
    if !inside || from.len() > 2 || to.len() > 2 || from == to {
        return Vec::new();
    }
    vec![Change::Inner {
        shifts: vec![(vec![left], from.iter().collect(), to.iter().collect())],
        after: Vec::new(),
    }]
}

/// The changes of the stem's last letters (one or two) and the ending's
/// first (none to two) that turn `built` into `want`: `ε+ο → +ου`. What
/// stays of the stem keeps the boundary after it.
fn edits(built: &str, want: &str) -> Vec<Change> {
    let built: String = built
        .chars()
        .filter(|&c| c != COPIED && c != FRONT && c != ALTERNATES && c != CHANGED)
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

/// The same change inside alternating stems after one letter or another
/// (`t → d` after `a`, after `u`), as one law after those letters (`t → d`
/// after `[a|u]`), with the words of all of them.
fn lefts(support: &HashMap<Change, u32>) -> Vec<(Change, u32)> {
    let mut by_change: HashMap<(&str, &str), Vec<(u32, char)>> = HashMap::default();
    for (change, &n) in support {
        if let Change::Inner { shifts, .. } = change
            && let [(left, from, to)] = shifts.as_slice()
            && let [l] = left.as_slice()
            && n as usize >= MIN_WORDS
        {
            by_change.entry((from, to)).or_default().push((n, *l));
        }
    }
    by_change
        .into_iter()
        .filter(|(_, letters)| letters.len() > 1)
        .map(|((from, to), mut letters)| {
            letters.sort_unstable_by(|x, y| y.cmp(x));
            let n = letters.iter().map(|l| l.0).sum();
            let mut left: Vec<char> = letters.into_iter().map(|l| l.1).collect();
            left.sort_unstable();
            let change = Change::Inner {
                shifts: vec![(left, from.to_string(), to.to_string())],
                after: Vec::new(),
            };
            (change, n)
        })
        .collect()
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

    #[test]
    fn learns_a_change_inside_alternating_stems() {
        // Finnish consonant gradation: `veto`, `vedon` (`taso`'s twin
        // builds `veto+nˇ`); a plain paradigm's builds have no such law.
        let found = inners("veto+nˇ", "vedon");
        let bare = Change::Inner {
            shifts: vec![(vec!['e'], "t".into(), "d".into())],
            after: Vec::new(),
        };
        assert_eq!(found, std::slice::from_ref(&bare));
        assert!(inners("veto+n", "vedon").is_empty());
        assert!(inners("paikk+anˇ", "paikan").contains(&Change::Inner {
            shifts: vec![(vec!['k'], "k".into(), String::new())],
            after: Vec::new(),
        }));
        // Where it gives forms (a closed syllable) and where it breaks them
        // (an open one, the illative's long vowel, a possessive `-mme`).
        let mut examples: Vec<(String, bool)> = Vec::new();
        for v in ['o', 'u', 'a'] {
            for (rest, weak) in [
                ("+n$", true),
                ("+ssa$", true),
                ("+lla$", true),
                ("+t$", true),
                ("+issa$", true),
                ("+$", false),
                ("+a$", false),
                ("+na$", false),
                ("+mme$", false),
                ("+nsa$", false),
                ("+" , false),
            ] {
                let rest = if rest == "+" { format!("+{v}n$") } else { rest.to_string() };
                for _ in 0..3 {
                    examples.push((format!("{v}{rest}"), weak));
                }
            }
        }
        let seqs: Vec<(Vec<char>, bool)> =
            examples.iter().map(|(e, y)| (e.chars().collect(), *y)).collect();
        let seqs: Vec<Place<'_>> = seqs
            .iter()
            .enumerate()
            .map(|(i, (e, y))| Place {
                after: e,
                gives: *y,
                word: i,
                cell: 0,
            })
            .collect();
        let mut after = Vec::new();
        let vowels: Vec<char> = "aeiouyäö".chars().collect();
        flatten(&decide(&seqs, 0, &vowels), &mut Vec::new(), &mut after);
        // With the geminate's shift, in one law: `kukka`, `kukan`.
        let law = Change::Inner {
            after,
            shifts: vec![
                (vec!['a', 'e', 'u'], "t".into(), "d".into()),
                (vec!['k'], "k".into(), String::new()),
            ],
        };
        let program = compile(std::slice::from_ref(&law)).expect("compiles");
        for (built, after) in [
            ("katu+nˇ", "kadun"),
            ("katu+issaˇ", "kaduissa"),
            ("katu+ˇ", "katu"),
            ("katu+unˇ", "katuun"),
            ("katu+mmeˇ", "katumme"),
            ("katu+nsaˇ", "katunsa"),
            ("katu+naˇ", "katuna"),
            // Not in an alternating stem, nor in the ending.
            ("katu+n", "katun"),
            ("ka+tunˇ", "katun"),
            ("kukka+nˇ", "kukan"),
            ("kukka+naˇ", "kukkana"),
        ] {
            let built_now = program.apply(built);
            assert_eq!(pattern::unmarked(&built_now).0, after, "{law:?} on {built}");
        }
    }
}

