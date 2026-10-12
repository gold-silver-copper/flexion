//! Learning a [`Choice`]: ordered ending rules and the words they get wrong.
//!
//! Endings are considered from short to long. An ending becomes a rule when
//! it pays for itself (minimum description length): telling the values of
//! the words with that ending by the rule's own counts, rather than by the
//! broader rule that decides them now, saves more bits than writing the rule
//! down costs. The words the finished rules still get wrong, judged exactly
//! as the runtime reads them, are listed.

use crate::HashMap;
use flexion::{Choice, Label, Rule};
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::hash::Hash;

/// The longest ending a rule looks at, in letters.
pub const MAX_ENDING: usize = 8;

/// How a choice is learned.
#[derive(Debug, Clone, Copy)]
pub struct Settings {
    /// Bits a rule costs: what it must save to be kept.
    pub rule_bits: f64,
    /// Words a rule must cover.
    pub min_gain: u32,
    /// See [`flexion::Choice::read`].
    pub smoothing: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            rule_bits: 4.0,
            min_gain: 2,
            smoothing: 2.0,
        }
    }
}

/// Learn which value a word takes from `examples` (`(word, value)`; a word
/// may have several, homographs). `fits` says whether a word can take a
/// value at all (the runtime asks the same).
pub fn learn<'a, L: Label + Hash + Ord>(
    examples: &[(String, L)],
    settings: Settings,
    fits: impl Fn(&str, L) -> bool,
) -> Choice<'a, L> {
    // Every ending's counts.
    let mut counts: HashMap<String, BTreeMap<L, u32>> = HashMap::default();
    for (word, value) in examples {
        for ending in endings(word) {
            *counts
                .entry(ending.to_string())
                .or_default()
                .entry(*value)
                .or_default() += 1;
        }
    }
    let winner = |c: &BTreeMap<L, u32>| -> Option<L> {
        // Most words; of equal counts, the lowest value.
        c.iter()
            .max_by(|a, b| a.1.cmp(b.1).then_with(|| b.0.cmp(a.0)))
            .map(|(&l, _)| l)
    };
    let mut by_length: Vec<(&String, &BTreeMap<L, u32>)> = counts.iter().collect();
    by_length.sort_by(|a, b| {
        a.0.chars()
            .count()
            .cmp(&b.0.chars().count())
            .then_with(|| a.0.cmp(b.0))
    });
    // Kept rules, by ending.
    let mut kept: HashMap<&str, &BTreeMap<L, u32>> = HashMap::default();
    for (ending, c) in &by_length {
        if winner(c).is_none() {
            continue;
        }
        if ending.is_empty() {
            kept.insert(ending.as_str(), c);
            continue;
        }
        let total: u32 = c.values().sum();
        if total < settings.min_gain {
            continue;
        }
        // The rule deciding these words now: the longest kept ending.
        let Some(current) = endings(ending).skip(1).find_map(|e| kept.get(e).copied()) else {
            continue;
        };
        if saved_bits(c, current) >= settings.rule_bits {
            kept.insert(ending.as_str(), c);
        }
    }
    let mut rules: Vec<Rule<'a, L>> = kept
        .keys()
        .filter_map(|&ending| {
            let c = counts.get(ending)?;
            let mut list: Vec<(L, u32)> = c.iter().map(|(&l, &n)| (l, n)).collect();
            list.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
            Some(Rule {
                ending: Cow::Owned(ending.to_string()),
                counts: Cow::Owned(list),
            })
        })
        .collect();
    rules.sort_by(|a, b| flexion::by_ending(&a.ending, &b.ending));
    let mut choice = Choice {
        side: flexion::Side::End,
        rules: Cow::Owned(rules),
        exceptions: Cow::Owned(Vec::new()),
    };
    // The words the rules get wrong, read as the runtime reads them.
    let mut by_word: BTreeMap<&str, Vec<L>> = BTreeMap::new();
    for (word, value) in examples {
        let values = by_word.entry(word.as_str()).or_default();
        if !values.contains(value) {
            values.push(*value);
        }
    }
    let mut exceptions: Vec<(Cow<'a, str>, L)> = Vec::new();
    for (word, values) in by_word {
        let read = choice.read(word, settings.smoothing, |l| fits(word, l));
        let right = values.len() == 1 && read.first().map(|r| r.value) == values.first().copied();
        if !right {
            exceptions.extend(
                values
                    .into_iter()
                    .map(|v| (Cow::Owned(word.to_string()), v)),
            );
        }
    }
    exceptions.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));
    choice.exceptions = Cow::Owned(exceptions);
    choice
}

/// [`learn`], for a choice that reads beginnings (see [`flexion::Side`]):
/// its rules are learned on the words read backwards.
pub fn learn_starts<'a, L: Label + Hash + Ord>(
    examples: &[(String, L)],
    settings: Settings,
    fits: impl Fn(&str, L) -> bool,
) -> Choice<'a, L> {
    let back = |s: &str| -> String { s.chars().rev().collect() };
    let reversed: Vec<(String, L)> = examples.iter().map(|(w, l)| (back(w), *l)).collect();
    let learned = learn(&reversed, settings, |w, l| fits(&back(w), l));
    let mut rules: Vec<Rule<'a, L>> = learned
        .rules
        .iter()
        .map(|r| Rule {
            ending: Cow::Owned(back(&r.ending)),
            counts: r.counts.clone(),
        })
        .collect();
    rules.sort_by(|a, b| a.ending.cmp(&b.ending));
    let mut exceptions: Vec<(Cow<'a, str>, L)> = learned
        .exceptions
        .iter()
        .map(|(w, l)| (Cow::Owned(back(w)), *l))
        .collect();
    exceptions.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));
    Choice {
        side: flexion::Side::Start,
        rules: Cow::Owned(rules),
        exceptions: Cow::Owned(exceptions),
    }
}

/// The bits saved telling the values of words counted `own` by their own
/// counts rather than by `broader`'s (each smoothed by half a word).
fn saved_bits<L: Ord>(own: &BTreeMap<L, u32>, broader: &BTreeMap<L, u32>) -> f64 {
    let n: f64 = own.values().map(|&x| f64::from(x)).sum();
    let m: f64 = broader.values().map(|&x| f64::from(x)).sum();
    #[allow(clippy::cast_precision_loss)]
    let k = (broader.len().max(own.len()) as f64) + 1.0;
    own.iter()
        .map(|(l, &x)| {
            let x = f64::from(x);
            let p_own = x / n;
            let q = (broader.get(l).map_or(0.0, |&y| f64::from(y)) + 0.5) / (m + 0.5 * k);
            x * (p_own / q).log2()
        })
        .sum()
}

/// A word's endings, longest first, down to the empty one: at most
/// [`MAX_ENDING`] letters (the whole word if it is shorter).
pub fn endings(word: &str) -> impl Iterator<Item = &str> {
    let starts: Vec<usize> = word
        .char_indices()
        .map(|(i, _)| i)
        .chain(std::iter::once(word.len()))
        .collect();
    let first = starts.len().saturating_sub(MAX_ENDING.saturating_add(1));
    starts
        .into_iter()
        .skip(first)
        .filter_map(move |i| word.get(i..))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn learns_rules_and_lists_the_rest() {
        let mut examples: Vec<(String, u16)> = Vec::new();
        for w in ["книга", "нога", "рука", "мука", "дорога"] {
            examples.push((w.to_string(), 1));
        }
        for w in ["стол", "дом", "кот", "нос", "сок", "рот"] {
            examples.push((w.to_string(), 0));
        }
        // A word in -а that isn't like книга.
        examples.push(("папа".to_string(), 2));
        let c = learn(&examples, Settings::default(), |_, _| true);
        let top = |w: &str| c.read(w, 2.0, |_| true).first().map(|r| r.value);
        assert_eq!(top("слуга"), Some(1));
        assert_eq!(top("мост"), Some(0));
        assert_eq!(top("папа"), Some(2));
        assert_eq!(c.listed("папа").collect::<Vec<_>>(), vec![2]);
        // Every example reads right.
        for (w, v) in &examples {
            assert_eq!(top(w), Some(*v), "{w}");
        }
        assert_eq!(
            endings("книга").collect::<Vec<_>>(),
            vec!["книга", "нига", "ига", "га", "а", ""]
        );
    }
}
