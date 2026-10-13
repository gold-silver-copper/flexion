//! Choices: what a word's ending says about it (which paradigm it follows,
//! where it is stressed, its gender), learned from the dictionary as an
//! ordered list of rules, with the words no rule gets right listed.

use crate::Label;
use std::borrow::Cow;

/// One learned choice: rules on the word's ending (the longest ending the
/// word has decides; the rule with the empty ending covers every word), and
/// the words the rules get wrong. A choice can read beginnings instead (see
/// [`Side`]).
#[derive(Debug, Clone, PartialEq)]
pub struct Choice<'a, L: Label> {
    /// Which end of a word its rules read.
    pub side: Side,
    /// Sorted by their endings read backwards (see [`by_ending`]), so rules
    /// on related endings stand together: `а`, `ка`, `ска`, `ца`.
    pub rules: Cow<'a, [Rule<'a, L>]>,
    /// Words the rules don't decide right, with what they are, sorted by word.
    pub exceptions: Cow<'a, [(Cow<'a, str>, L)]>,
}

/// Which end of a word a choice's rules read.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Side {
    /// Its ending (the paradigm of `книга` is told by `-а`).
    #[default]
    End,
    /// Its beginning (the length of a vowel is told by the vowel and what
    /// follows it: `āti`).
    Start,
}

/// One rule: the words with this ending (or beginning), and how they split.
#[derive(Debug, Clone, PartialEq)]
pub struct Rule<'a, L: Label> {
    pub ending: Cow<'a, str>,
    /// How many of the dictionary's words with this ending take each value,
    /// commonest first.
    pub counts: Cow<'a, [(L, u32)]>,
}

/// How sure a choice is about a word.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Status {
    /// The dictionary says so: the word is listed.
    Listed,
    /// A productive rule: few enough exceptions that speakers extend it to
    /// new words (Yang's Tolerance Principle).
    Regular,
    /// No rule here is productive: the words with this ending split too
    /// evenly, and speakers vary.
    Unsettled,
    /// Few or no words of the dictionary have the cell: the form is made by
    /// analogy with cells words do have.
    Unattested,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Status::Listed => "listed",
            Status::Regular => "regular",
            Status::Unsettled => "unsettled",
            Status::Unattested => "unattested",
        })
    }
}

/// One value a choice gives a word, with its probability and why.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Reading<'g, 'a, L: Label> {
    pub value: L,
    pub probability: f32,
    pub status: Status,
    /// The rule that decided, or `None` for a listed word.
    pub rule: Option<&'g Rule<'a, L>>,
}

impl<L: Label> std::fmt::Display for Reading<'_, '_, L> {
    /// `Книга (97%, regular): 1203 of 1210 words in -га`
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} ({:.0}%, {})",
            self.value,
            self.probability * 100.0,
            self.status
        )?;
        if let Some(r) = self.rule {
            let evidence = crate::Evidence {
                ending: &r.ending,
                count: r.count(self.value),
                total: r.total(),
            };
            write!(f, ": {evidence}")?;
        }
        Ok(())
    }
}

impl<'a, L: Label> Rule<'a, L> {
    /// How many words the rule covers.
    pub fn total(&self) -> u32 {
        self.counts.iter().map(|&(_, n)| n).sum()
    }

    /// How many of them take `value`.
    pub fn count(&self, value: L) -> u32 {
        self.counts
            .iter()
            .find(|&&(l, _)| l == value)
            .map_or(0, |&(_, n)| n)
    }

    /// Whether the rule's commonest value is productive: its exceptions are
    /// at most `N / ln N` of its `N` words (Yang 2016).
    pub fn productive(&self) -> bool {
        let total = self.total();
        let top = self.counts.first().map_or(0, |&(_, n)| n);
        f64::from(total.saturating_sub(top)) <= tolerance(total)
    }
}

/// The order of rules: by ending, read from its last letter back.
pub fn by_ending(a: &str, b: &str) -> std::cmp::Ordering {
    a.chars().rev().cmp(b.chars().rev())
}

/// The Tolerance Principle's threshold: a rule over `n` words stays
/// productive with up to `n / ln n` exceptions.
pub fn tolerance(n: u32) -> f64 {
    if n < 2 {
        return 0.0;
    }
    f64::from(n) / f64::from(n).ln()
}

impl<'a, L: Label> Choice<'a, L> {
    /// The values the dictionary lists for `word` (several for homographs).
    pub fn listed<'s>(&'s self, word: &'s str) -> impl Iterator<Item = L> + 's {
        let start = self.exceptions.partition_point(|(w, _)| w.as_ref() < word);
        self.exceptions
            .get(start..)
            .unwrap_or_default()
            .iter()
            .take_while(move |(w, _)| w.as_ref() == word)
            .map(|&(_, l)| l)
    }

    /// What `word` is, most likely first: the listed values if the
    /// dictionary has the word, else the rules' counts. A specific rule's
    /// counts lean on the broader rules' by `smoothing` words' worth (a
    /// rule seen with few words says less). Only values `fits` accepts are
    /// given.
    pub fn read(
        &self,
        word: &str,
        smoothing: f32,
        fits: impl Fn(L) -> bool,
    ) -> Vec<Reading<'_, 'a, L>> {
        let listed: Vec<L> = self.listed(word).filter(|&l| fits(l)).collect();
        if !listed.is_empty() {
            #[allow(clippy::cast_precision_loss)]
            let p = 1.0 / listed.len() as f32;
            return listed
                .into_iter()
                .map(|value| Reading {
                    value,
                    probability: p,
                    status: Status::Listed,
                    rule: None,
                })
                .collect();
        }
        // The rules whose ending (or beginning) the word has, least specific
        // first.
        let cuts = word.char_indices().map(|(i, _)| i).chain([word.len()]);
        let mut chain: Vec<&Rule<'a, L>> = cuts
            .filter_map(|i| {
                let at = match self.side {
                    Side::End => {
                        let affix = word.get(i..)?;
                        self.rules.binary_search_by(|r| by_ending(&r.ending, affix))
                    }
                    Side::Start => {
                        let affix = word.get(..i)?;
                        self.rules
                            .binary_search_by(|r| r.ending.as_ref().cmp(affix))
                    }
                };
                self.rules.get(at.ok()?)
            })
            .collect();
        if self.side == Side::End {
            chain.reverse();
        }
        let Some(&decisive) = chain.last() else {
            return Vec::new();
        };
        // Smooth down the chain, broadest rule first.
        let mut dist: Vec<(L, f64)> = Vec::new();
        for rule in &chain {
            dist = blend(&rule.counts, &dist, smoothing);
        }
        dist.retain(|&(l, p)| p > 0.0 && fits(l));
        let mass: f64 = dist.iter().map(|&(_, p)| p).sum();
        if mass <= 0.0 {
            return Vec::new();
        }
        let status = if decisive.productive() {
            Status::Regular
        } else {
            Status::Unsettled
        };
        let mut out: Vec<Reading<'_, 'a, L>> = dist
            .into_iter()
            .map(|(value, p)| Reading {
                value,
                #[allow(clippy::cast_possible_truncation)]
                probability: (p / mass) as f32,
                status,
                rule: Some(decisive),
            })
            .collect();
        out.sort_by(|a, b| {
            b.probability
                .total_cmp(&a.probability)
                .then_with(|| a.value.index().cmp(&b.value.index()))
        });
        out
    }
}

/// How much of each of its counts a rule leaves to the broader rule
/// (absolute discounting): a rule of few words leans on the broader one
/// more than a rule of many.
pub const DISCOUNT: f64 = 0.5;

/// A rule's distribution: its `counts`, each less [`DISCOUNT`], and the
/// broader rule's distribution `parent` (none for the broadest) with the
/// weight that leaves, plus `smoothing` words' worth.
pub fn blend<L: Label>(counts: &[(L, u32)], parent: &[(L, f64)], smoothing: f32) -> Vec<(L, f64)> {
    let n: f64 = counts.iter().map(|&(_, c)| f64::from(c)).sum();
    if parent.is_empty() {
        return if n > 0.0 {
            counts.iter().map(|&(l, c)| (l, f64::from(c) / n)).collect()
        } else {
            Vec::new()
        };
    }
    let b = f64::from(smoothing);
    #[allow(clippy::cast_precision_loss)]
    let k = counts.iter().filter(|e| e.1 > 0).count() as f64;
    let denom = n + b;
    if denom <= 0.0 {
        return parent.to_vec();
    }
    let lean = (DISCOUNT * k + b) / denom;
    let mut out: Vec<(L, f64)> = parent.iter().map(|&(l, q)| (l, lean * q)).collect();
    for &(l, c) in counts {
        let own = (f64::from(c) - DISCOUNT).max(0.0) / denom;
        match out.iter_mut().find(|e| e.0 == l) {
            Some(e) => e.1 += own,
            None => out.push((l, own)),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn choice() -> Choice<'static, u16> {
        Choice {
            side: Side::End,
            rules: Cow::Owned(vec![
                Rule {
                    ending: "".into(),
                    counts: Cow::Owned(vec![(0, 100), (1, 40), (2, 50)]),
                },
                Rule {
                    ending: "а".into(),
                    counts: Cow::Owned(vec![(1, 30), (0, 2)]),
                },
                Rule {
                    ending: "ость".into(),
                    counts: Cow::Owned(vec![(2, 50), (0, 1)]),
                },
            ]),
            exceptions: Cow::Owned(vec![("путь".into(), 3)]),
        }
    }

    #[test]
    fn the_most_specific_rule_decides() {
        let c = choice();
        let r = c.read("радость", 2.0, |_| true);
        assert_eq!(r.first().map(|x| x.value), Some(2));
        assert_eq!(r.first().map(|x| x.status), Some(Status::Regular));
        assert_eq!(
            c.read("книга", 2.0, |_| true).first().map(|x| x.value),
            Some(1)
        );
        // A value the word can't take is never given.
        assert_eq!(
            c.read("книга", 2.0, |l| l != 1).first().map(|x| x.value),
            Some(0)
        );
        let listed = c.read("путь", 2.0, |_| true);
        assert_eq!(
            listed.first().map(|x| (x.value, x.status)),
            Some((3, Status::Listed))
        );
    }

    #[test]
    fn tolerance_follows_yang() {
        assert!((tolerance(1000) - 144.76).abs() < 0.01);
        let even = Rule {
            ending: "".into(),
            counts: Cow::Owned(vec![(0u16, 837), (1, 516)]),
        };
        // Polish masculine genitives in -a and -u: neither is productive.
        assert!(!even.productive());
    }
}
