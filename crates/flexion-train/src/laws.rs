//! Sound laws at the stem's edge: changes where a stem meets its ending
//! (Greek `ε+ο → +ου`, Russian `к+ы → к+и`). They are found where a more
//! general paradigm would build another paradigm's words but for such a
//! change, and kept when they fix many forms and break few. The words they
//! explain join the more general paradigm, and paradigms left without words
//! go.

use crate::HashMap;
use flexion::pattern::{self, BOUNDARY};
use flexion::{Orthography, Paradigm};
use sound_law::{Program, Rule, RuleSet};
use std::borrow::Cow;

/// One learned law: `from` (the stem's last letters, the boundary, the
/// ending's first) becomes `to`, in `sound-law`'s notation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Law {
    pub from: String,
    pub to: String,
    /// The words whose forms it gives.
    pub words: u32,
    /// Some of them.
    pub examples: Vec<String>,
    /// The dictionary's forms it changes from right to wrong.
    pub breaks: u32,
}

impl std::fmt::Display for Law {
    /// As a grammar states it: `ε+ο → +ου`; for a law on several letters,
    /// `[к|г|х]+ы → +и`; for one on a tier, `न → ण after र, ष unless त, …
    /// comes between, not before ्`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(t) = tier_parts(&self.from) {
            let off: Vec<&str> = t
                .tier
                .iter()
                .filter(|l| !t.on.contains(l))
                .copied()
                .collect();
            write!(f, "{} → {} after {}", t.letter, self.to, t.on.join(", "))?;
            if !off.is_empty() {
                write!(f, " unless {} comes between", off.join(", "))?;
            }
            if !t.stop.is_empty() {
                write!(f, ", not before {}", t.stop.join(", "))?;
            }
            return Ok(());
        }
        let plain = |s: &str| s.replace("{x:", "").replace("{x}", "").replace('}', "");
        write!(f, "{} → {}", plain(&self.from), plain(&self.to))
    }
}

/// A word, as the learner sees it: its letters, and per cell its usual form
/// (`None` where the dictionary has none, or the cell is referred).
pub struct Word<'w> {
    pub letters: &'w str,
    pub forms: Vec<Option<&'w str>>,
}

/// The fewest words a law must give forms of.
const MIN_WORDS: usize = 5;
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

/// A law kept, the words whose forms it gives, and how many forms.
type Kept = (Law, Vec<usize>, usize);

/// A build: its word, its reading, its place among the word's forms.
type At = (usize, usize, usize);

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
    // Every change that would let a word's paradigm, or a more general one,
    // build a form it has, and the words it would do that for.
    let mut support: HashMap<(String, String), u32> = HashMap::default();
    for (rs, fs) in readings.iter().zip(&forms) {
        let Some(own) = rs.first() else { continue };
        // Its own paradigm's wrong forms, each that a change explains; a
        // more general paradigm's, all of them.
        let mut mine: Vec<(String, String)> = own
            .builds
            .iter()
            .zip(fs)
            .filter(|(b, f)| !same(b, f))
            .filter_map(|(b, f)| b.split_once(BOUNDARY).map(|(s, e)| edits(s, e, f)))
            .flatten()
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
    let mut candidates: Vec<((String, String), u32)> = classes(&support);
    candidates.extend(support);
    candidates.extend(tiers(&readings, &forms));
    candidates.retain(|&(_, n)| n as usize >= MIN_WORDS);
    candidates.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    // The cell of each form a word has.
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
    let program = (!laws.is_empty())
        .then(|| {
            let rules: Vec<(String, String)> = laws
                .iter()
                .map(|l| (l.from.clone(), l.to.clone()))
                .collect();
            compile(&rules)
        })
        .flatten();
    // A word stays where it was unless, under the laws kept, the paradigm
    // it moved to builds its forms as well.
    let right = |p: u16, w: &Word<'_>| {
        reading(orthography, &paradigms, usize::from(p), w).map_or(0, |r| {
            r.builds
                .iter()
                .zip(w.forms.iter().flatten())
                .filter(|(b, f)| match &program {
                    Some(laws) => same(&laws.apply(b), f),
                    None => same(b, f),
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

/// What paradigm `p` builds for word `w`, if it can take it.
fn reading(
    orthography: &Orthography<'_>,
    paradigms: &[Paradigm<'_>],
    p: usize,
    w: &Word<'_>,
) -> Option<Reading> {
    let par = paradigms.get(p)?;
    let stems = par.stems(orthography, w.letters)?;
    let (builds, right) = w
        .forms
        .iter()
        .enumerate()
        .filter_map(|(cell, f)| Some((cell, (*f)?)))
        .fold((Vec::new(), 0), |(mut builds, right), (cell, form)| {
            let built = par
                .pattern(paradigms, cell)
                .and_then(|pat| pattern::variants(pat).next())
                .map(|v| pattern::fill_marked(v, &stems))
                .unwrap_or_default();
            let right = right + usize::from(same(&built, form));
            builds.push(built);
            (builds, right)
        });
    Some(Reading {
        paradigm: p,
        builds,
        right,
    })
}

/// The laws, tried in turn (the best supported first), and the assignment
/// after the words they explain moved. A law is kept when it fixes forms of
/// enough words (under their paradigm or a more general one) and breaks few
/// their paradigm gave; with each, the words whose forms it gives (by their
/// place in `forms`) and how many forms.
fn greedy(
    candidates: &[((String, String), u32)],
    mut readings: Vec<Vec<Reading>>,
    forms: &[Vec<&str>],
    cells_of: &[Vec<usize>],
    mut assignment: Vec<u16>,
    rank: &dyn Fn(usize) -> (usize, usize),
) -> (Vec<Kept>, Vec<u16>) {
    // Builds by the stem's last letter and the letters around their
    // boundary.
    let mut index: Index = HashMap::default();
    for (w, rs) in readings.iter().enumerate() {
        for (r, reading) in rs.iter().enumerate() {
            for (k, built) in reading.builds.iter().enumerate() {
                file(&mut index, built, (w, r, k));
            }
        }
    }
    let form = |w: usize, k: usize| forms.get(w).and_then(|f| f.get(k)).copied().unwrap_or("");
    let mut laws: Vec<(Law, Vec<At>)> = Vec::new();
    for ((from, to), _) in candidates {
        let tier = tier_letter(from);
        // A law on a tier runs as its compiled rule; one at the edge, as a
        // rewrite.
        let law = match tier {
            Some(_) => match compile(&[(from.clone(), to.clone())]) {
                Some(law) => Some(law),
                None => continue,
            },
            None => None,
        };
        let mut at: Vec<At> = match tier {
            // A law on a tier looks at every build with its letter.
            Some(x) => readings
                .iter()
                .enumerate()
                .flat_map(|(w, rs)| {
                    rs.iter().enumerate().flat_map(move |(r, reading)| {
                        reading
                            .builds
                            .iter()
                            .enumerate()
                            .filter(move |(_, b)| b.contains(x))
                            .map(move |(k, _)| (w, r, k))
                    })
                })
                .collect(),
            None => stem_ends(from)
                .iter()
                .filter_map(|c| index.get(c))
                .flatten()
                .filter(|(k, _)| applies(from, k))
                .flat_map(|(_, v)| v.iter().copied())
                .collect(),
        };
        at.sort_unstable();
        at.dedup();
        let mut changes: Vec<(At, String)> = Vec::new();
        let mut fixed: Vec<At> = Vec::new();
        let mut breaks = 0u32;
        for (w, r, k) in at {
            let Some(reading) = readings.get(w).and_then(|rs| rs.get(r)) else {
                continue;
            };
            // An entry the laws so far have moved elsewhere.
            let Some(built) = reading
                .builds
                .get(k)
                .filter(|b| tier.is_some() || applies(from, &window(b)))
            else {
                continue;
            };
            let after = match &law {
                Some(law) => law.apply(built),
                None => rewrite(from, to, built).unwrap_or_else(|| built.clone()),
            };
            if after == *built {
                continue;
            }
            let (was, is) = (same(built, form(w, k)), same(&after, form(w, k)));
            let own = assignment
                .get(w)
                .is_some_and(|&p| usize::from(p) == reading.paradigm);
            if own && was && !is {
                breaks += 1;
            }
            if !was && is {
                fixed.push((w, r, k));
            }
            changes.push(((w, r, k), after));
        }
        let mut fixed_words: Vec<usize> = fixed.iter().map(|a| a.0).collect();
        fixed_words.dedup();
        // A sound law applies wherever its letters meet: in more than one
        // cell.
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
            let Some(reading) = readings.get_mut(w).and_then(|rs| rs.get_mut(r)) else {
                continue;
            };
            let Some(built) = reading.builds.get_mut(k) else {
                continue;
            };
            reading.right = (reading.right + usize::from(same(&after, want)))
                .saturating_sub(usize::from(same(built, want)));
            file(&mut index, &after, (w, r, k));
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
        laws.push((
            Law {
                from: from.clone(),
                to: to.clone(),
                words: 0,
                examples: Vec::new(),
                breaks,
            },
            fixed,
        ));
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

/// What a law at the stem's edge makes of a build, as its compiled rule
/// does, but quicker: the letters around the boundary replaced. `None` where
/// it doesn't apply.
fn rewrite(from: &str, to: &str, built: &str) -> Option<String> {
    let (a, b) = from.split_once(BOUNDARY)?;
    let (stem, ending) = built.split_once(BOUNDARY)?;
    let rest = ending.strip_prefix(b)?;
    let (head, kept) = match a.strip_prefix("{x:[").and_then(|a| a.strip_suffix("]}")) {
        Some(letters) => {
            let letter = letters.split('|').find(|l| stem.ends_with(l))?;
            (stem.strip_suffix(letter)?, letter)
        }
        None => (stem.strip_suffix(a)?, ""),
    };
    Some(format!("{head}{}{rest}", to.replace("{x}", kept)))
}

/// Builds by the stem's last letter, then the letters around their
/// boundary.
type Index = HashMap<char, HashMap<(String, String), Vec<At>>>;

fn file(index: &mut Index, built: &str, at: At) {
    let around = window(built);
    let last = around.0.chars().last().unwrap_or(BOUNDARY);
    index
        .entry(last)
        .or_default()
        .entry(around)
        .or_default()
        .push(at);
}

/// The stem's last letters a law at the stem's edge applies after.
fn stem_ends(from: &str) -> Vec<char> {
    let a = from.split_once(BOUNDARY).map_or("", |(a, _)| a);
    a.strip_prefix("{x:[")
        .and_then(|a| a.strip_suffix("]}"))
        .map_or_else(|| vec![a], |a| a.split('|').collect())
        .iter()
        .map(|a| a.chars().last().unwrap_or(BOUNDARY))
        .collect()
}

/// The letters around a build's boundary: up to two before, two after.
fn window(built: &str) -> (String, String) {
    let (stem, ending) = built.split_once(BOUNDARY).unwrap_or(("", built));
    let mut before: Vec<char> = stem.chars().rev().take(2).collect();
    before.reverse();
    (
        before.into_iter().collect(),
        ending.chars().take(2).collect(),
    )
}

/// Whether the law changing `from` (`a+b`) applies where a build's boundary
/// has `window` around it.
fn applies(from: &str, (before, after): &(String, String)) -> bool {
    let Some((a, b)) = from.split_once(BOUNDARY) else {
        return false;
    };
    let letters = a.strip_prefix("{x:[").and_then(|a| a.strip_suffix("]}"));
    after.starts_with(b)
        && match letters {
            Some(letters) => letters.split('|').any(|a| before.ends_with(a)),
            None => before.ends_with(a),
        }
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
                    paradigms
                        .get(p)
                        .is_some_and(|p| pattern::pieces(&p.lemma) == pattern::pieces(&me.lemma))
                })
                .filter(|&p| {
                    paradigms
                        .get(p)
                        .is_some_and(|p| stem_like(&p.lemma, &me.lemma, mine))
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
    let (Some(general), Some(lemma)) = (end(general), end(lemma)) else {
        return false;
    };
    let Some(moved) = lemma.strip_suffix(general.as_str()) else {
        return false;
    };
    let begins = cells
        .iter()
        .filter(|c| !c.is_empty())
        .filter(|c| end(c).is_some_and(|e| e.starts_with(moved)))
        .count();
    let all = cells.iter().filter(|c| !c.is_empty()).count();
    moved.is_empty() || 2 * begins >= all
}

/// The changes at the stem's edge that would let reading `r` build the forms
/// the word's `own` reading builds: every way of explaining each (see
/// [`edits`]). `None` when one can't be explained so, or none differs.
fn options(own: &Reading, r: &Reading, forms: &[&str]) -> Option<Vec<(String, String)>> {
    let mut out: Vec<(String, String)> = Vec::new();
    for ((built, mine), want) in r.builds.iter().zip(&own.builds).zip(forms) {
        if same(built, want) || !same(mine, want) {
            continue;
        }
        let (stem, ending) = built.split_once(BOUNDARY)?;
        let ways = edits(stem, ending, want);
        if ways.is_empty() {
            return None;
        }
        out.extend(ways);
    }
    (!out.is_empty()).then_some(out)
}

/// The changes of the stem's last letters (one or two) and the ending's
/// first (none to two) that turn `stem` + `ending` into `want`, as
/// `(from, to)`: `("ε+ο", "+ου")`. What stays of the stem keeps the boundary
/// after it.
fn edits(stem: &str, ending: &str, want: &str) -> Vec<(String, String)> {
    let stem: Vec<char> = stem.chars().collect();
    let ending: Vec<char> = ending.chars().collect();
    let want: Vec<char> = want.chars().collect();
    let mut out = Vec::new();
    for k in 1..=2usize {
        for m in 0..=2usize {
            let (Some(head), Some(a), Some(b), Some(tail)) = (
                stem.len().checked_sub(k).and_then(|h| stem.get(..h)),
                stem.len().checked_sub(k).and_then(|h| stem.get(h..)),
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
            let from: String = a.iter().chain(&[BOUNDARY]).chain(b).collect();
            let to: String = kept_part.iter().chain(&[BOUNDARY]).chain(rest).collect();
            out.push((from, to));
        }
    }
    out
}

/// The same change of an ending after one letter or another (`к+ы → к+и`,
/// `г+ы → г+и`), as one law on those letters (`{x:[к|г]}+ы → {x}+и`), with
/// the words of all of them.
fn classes(support: &HashMap<(String, String), u32>) -> Vec<((String, String), u32)> {
    let mut by_change: HashMap<(&str, &str), Vec<(u32, &str)>> = HashMap::default();
    for ((from, to), &n) in support {
        let (Some((a, b)), Some((kept, rest))) =
            (from.split_once(BOUNDARY), to.split_once(BOUNDARY))
        else {
            continue;
        };
        if a.chars().count() == 1 && a == kept {
            by_change.entry((b, rest)).or_default().push((n, a));
        }
    }
    by_change
        .into_iter()
        .filter(|(_, letters)| letters.len() > 1)
        .map(|((b, rest), mut letters)| {
            letters.sort_by(|x, y| y.cmp(x));
            let alternatives: Vec<&str> = letters.iter().map(|l| l.1).collect();
            let from = format!("{{x:[{}]}}{BOUNDARY}{b}", alternatives.join("|"));
            let to = format!("{{x}}{BOUNDARY}{rest}");
            ((from, to), letters.iter().map(|l| l.0).sum())
        })
        .collect()
}

/// Laws on a tier: one letter of an ending becomes another (`न` → `ण`) where
/// the nearest of some letters before it (the tier) is one of a few (`र`,
/// `ष`, `ृ`; not `त`, `च`, …): Sanskrit's n → ṇ, or a vowel harmony. Found
/// where the change is away from the stem's edge (a change next to it is a
/// law at the edge). As candidates, with the words that have the change.
fn tiers(readings: &[Vec<Reading>], forms: &[Vec<&str>]) -> Vec<((String, String), u32)> {
    // A build and its form, as letters, and where the ending starts.
    let pairs = || {
        readings.iter().zip(forms).flat_map(|(rs, fs)| {
            rs.iter().flat_map(move |r| {
                r.builds.iter().zip(fs).filter_map(|(b, f)| {
                    let edge = b.chars().position(|c| c == BOUNDARY)?;
                    let built: Vec<char> = b.chars().filter(|&c| c != BOUNDARY).collect();
                    let form: Vec<char> = f.chars().collect();
                    (built.len() == form.len()).then_some((built, form, edge))
                })
            })
        })
    };
    // The changes of one letter of the ending, and the words with them.
    let mut changes: HashMap<(char, char), Vec<usize>> = HashMap::default();
    for (w, rs) in readings.iter().enumerate() {
        let fs = forms.get(w).map_or(&[][..], Vec::as_slice);
        for r in rs {
            for (b, f) in r.builds.iter().zip(fs) {
                let (Some(edge), built) = (
                    b.chars().position(|c| c == BOUNDARY),
                    b.chars().filter(|&c| c != BOUNDARY),
                ) else {
                    continue;
                };
                let differ: Vec<(usize, (char, char))> = built
                    .zip(f.chars())
                    .enumerate()
                    .filter(|(_, (a, b))| a != b)
                    .collect();
                if let [(at, change)] = differ.as_slice()
                    && *at > edge
                    && b.chars().count() == f.chars().count() + 1
                {
                    let words = changes.entry(*change).or_default();
                    if words.last() != Some(&w) {
                        words.push(w);
                    }
                }
            }
        }
    }
    let mut out: Vec<((String, String), u32)> = changes
        .into_iter()
        .filter(|(_, words)| words.len() >= MIN_WORDS)
        .filter_map(|((x, y), words)| {
            // Each place an ending has `x`, by the letters before it (the
            // nearest first), and whether the dictionary has `y` there.
            let mut examples: HashMap<Example, u32> = HashMap::default();
            for (built, form, edge) in pairs() {
                if built
                    .iter()
                    .zip(&form)
                    .any(|(a, b)| a != b && (*a, *b) != (x, y))
                {
                    continue;
                }
                for (i, (&a, &b)) in built.iter().zip(&form).enumerate().skip(edge) {
                    if a == x {
                        let before: Vec<char> = built
                            .get(..i)
                            .unwrap_or_default()
                            .iter()
                            .rev()
                            .copied()
                            .collect();
                        let next = built.get(i + 1).copied();
                        *examples.entry((before, next, b == y)).or_default() += 1;
                    }
                }
            }
            let [on, off, stop] = tier(&examples)?;
            let list = |letters: &[char]| {
                letters
                    .iter()
                    .map(char::to_string)
                    .collect::<Vec<String>>()
                    .join("|")
            };
            let all: Vec<char> = on.iter().chain(&off).copied().collect();
            let mut from = format!("{{t:[{}]}}{{gap:![{}]*}}{x}", list(&on), list(&all));
            if !stop.is_empty() {
                from.push_str(&format!("{{!:[{}]}}", list(&stop)));
            }
            Some((
                (from, y.to_string()),
                u32::try_from(words.len()).unwrap_or(u32::MAX),
            ))
        })
        .collect();
    out.sort();
    out
}

/// A place an ending has the letter a law on a tier may change: the letters
/// before it (the nearest first), the one after it, and whether the
/// dictionary changes it.
type Example = (Vec<char>, Option<char>, bool);

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
                let changes = before
                    .iter()
                    .find(|c| on.contains(c) || off.contains(c))
                    .is_some_and(|c| on.contains(c))
                    && next.is_none_or(|n| !stop.contains(&n));
                changes == *y
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
                let free = if side == 2 {
                    !next[2].contains(&l)
                } else {
                    !next[0].contains(&l) && !next[1].contains(&l)
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
            Some((score, next)) if score >= best + MIN_WORDS as u32 => {
                best = score;
                sets = next;
            }
            _ => break,
        }
    }
    for set in &mut sets {
        set.sort_unstable();
    }
    (!sets[0].is_empty()).then_some(sets)
}

/// The letter a law on a tier changes; `None` for a law at the stem's edge.
fn tier_letter(from: &str) -> Option<char> {
    tier_parts(from).map(|t| t.letter)
}

/// A law on a tier, read: `{t:[र|ष]}{gap:![र|ष|त]*}न{!:[्]}`.
struct TierLaw<'a> {
    /// The letters that make the change.
    on: Vec<&'a str>,
    /// The tier: the nearest of these before the letter decides.
    tier: Vec<&'a str>,
    letter: char,
    /// The letters after it that stop it.
    stop: Vec<&'a str>,
}

fn tier_parts(from: &str) -> Option<TierLaw<'_>> {
    let rest = from.strip_prefix("{t:[")?;
    let (on, rest) = rest.split_once("]}{gap:![")?;
    let (tier, rest) = rest.split_once("]*}")?;
    let mut chars = rest.chars();
    let letter = chars.next()?;
    let stop = chars
        .as_str()
        .strip_prefix("{!:[")
        .and_then(|s| s.strip_suffix("]}"))
        .map_or_else(Vec::new, |s| s.split('|').collect());
    Some(TierLaw {
        on: on.split('|').collect(),
        tier: tier.split('|').collect(),
        letter,
        stop,
    })
}

/// Whether a form built marked at the stem's edge is `form`.
fn same(built: &str, form: &str) -> bool {
    built.chars().filter(|&c| c != BOUNDARY).eq(form.chars())
}

/// The rules of `laws` compiled, in order.
pub fn compile(laws: &[(String, String)]) -> Option<Program<'static>> {
    let rules: Vec<RuleText> = laws
        .iter()
        .enumerate()
        .map(|(i, (from, to))| {
            let name = format!("law{i}");
            match tier_parts(from) {
                Some(t) => RuleText {
                    name,
                    target: t.letter.to_string(),
                    to: to.clone(),
                    // The letter is the ending's: the stem's are as the
                    // dictionary writes them.
                    before: format!(
                        "{{t:[{on}]}}{{a:![{tier}]*}}{BOUNDARY}{{b:![{tier}]*}}",
                        on = t.on.join("|"),
                        tier = t.tier.join("|")
                    ),
                    stop: if t.stop.is_empty() {
                        String::new()
                    } else {
                        format!("{{[{}]}}", t.stop.join("|"))
                    },
                },
                None => RuleText {
                    name,
                    target: from.clone(),
                    to: to.clone(),
                    before: String::new(),
                    stop: String::new(),
                },
            }
        })
        .collect();
    let rules: Vec<Rule<'_>> = rules
        .iter()
        .map(|r| {
            let rule = Rule::new(&r.name, &r.target, &r.to).in_context(&r.before, "");
            if r.stop.is_empty() {
                rule
            } else {
                rule.unless_context("", &r.stop)
            }
        })
        .collect();
    RuleSet::new(&[], &rules).compile().ok()
}

/// A law as `sound-law` states a rule: its name, what it changes into what,
/// what must come before, and what after stops it.
struct RuleText {
    name: String,
    target: String,
    to: String,
    before: String,
    stop: String,
}

/// Whether pattern `p` builds what `q` does but for a change of the stem's
/// last letters and the ending's first two at most: the same before the
/// last stem piece, and `q`'s ending is `p`'s, its first letters changed
/// (`~ομεν`, `~ουμεν`).
fn near(p: &str, q: &str) -> bool {
    if p == q {
        return true;
    }
    let (Some((p_head, p_end)), Some((q_head, q_end))) = (p.rsplit_once('~'), q.rsplit_once('~'))
    else {
        return false;
    };
    let (p_end, q_end): (Vec<char>, Vec<char>) = (p_end.chars().collect(), q_end.chars().collect());
    p_head == q_head
        && (0..=2).any(|m| {
            p_end
                .get(m..)
                .is_some_and(|rest| q_end.ends_with(rest) && q_end.len() <= rest.len() + 4)
        })
}

/// The paradigms that still have words, and the assignment renumbered.
fn drop_empty<'s>(
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
        let finds = |stem, ending, want, from: &str, to: &str| {
            edits(stem, ending, want).contains(&(from.to_string(), to.to_string()))
        };
        // Greek contraction: ποιε + ομεν → ποιουμεν.
        assert!(finds("ποιε", "ομεν", "ποιουμεν", "ε+ο", "+ου"));
        // The Russian spelling rule: стрелк + ы → стрелки.
        assert!(finds("стрелк", "ы", "стрелки", "к+ы", "к+и"));
        let program = compile(&[("ε+ο".into(), "+ου".into())]).expect("compiles");
        assert_eq!(program.apply("ποιε+ομεν"), "ποι+ουμεν");
        // The same change after several letters is one law on all of them.
        let mut support = HashMap::default();
        support.insert(("к+ы".to_string(), "к+и".to_string()), 3);
        support.insert(("г+ы".to_string(), "г+и".to_string()), 2);
        let class = classes(&support);
        assert_eq!(
            class,
            [(("{x:[к|г]}+ы".to_string(), "{x}+и".to_string()), 5)]
        );
        assert!(applies("{x:[к|г]}+ы", &window("книг+ы")));
        let program = compile(&[class[0].0.clone()]).expect("compiles");
        assert_eq!(program.apply("книг+ы"), "книг+и");
        // The learner's quick rewrite agrees with the compiled rules.
        for (from, to, built) in [
            ("{x:[к|г]}+ы", "{x}+и", "книг+ы"),
            ("ε+ο", "+ου", "ποιε+ομεν"),
            ("γ+σ", "+ξ", "λεγ+σω"),
        ] {
            let program = compile(&[(from.into(), to.into())]).expect("compiles");
            assert_eq!(rewrite(from, to, built), Some(program.apply(built)));
        }
    }
}
