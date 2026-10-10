//! Turning a (lemma, form) pair into an edit the runtime can replay on other
//! words: the start stripped and a prefix added, a suffix stripped and one
//! added, and a stress rule (with the mark's kind) relative to that split.
//!
//! A pair can usually be read several ways (`λύω → ἔλυσα` is "add ἐ, ω → σα"
//! or "λυω → ἐλυσα"). The [`Chooser`] picks, for every pair of a part of
//! speech, the reading the most pairs share, so an augment or a `ge-` prefix
//! becomes one edit across many words instead of one per word.

use crate::extract::Word;
use flexion::Spelling;
use flexion::format::Stress;
use flexion::write::EditSpec;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

/// The lemma an edit starts from: its letters, stressed vowel and mark kind.
#[derive(Debug, Clone, Copy)]
pub struct Stem<'a> {
    pub letters: &'a str,
    pub stress: Option<usize>,
    pub kind: u8,
}

/// The longest start an edit may strip, and the longest prefix it may add,
/// in the alternative readings (the default reading finds longer prefixes,
/// like `more `).
const MAX_FRONT: usize = 3;
const MAX_PREFIX: usize = 6;

/// The default reading of `form` as an edit of `stem`. `lemma_cell` marks the
/// lemma's own cell, whose stress is described from the word's end so it can
/// be predicted for words the caller didn't stress.
pub fn edit(sp: &Spelling<'_>, stem: Stem<'_>, form: &str, lemma_cell: bool) -> EditSpec {
    let read = sp.read(form);
    let l: Vec<char> = stem.letters.chars().collect();
    let f: Vec<char> = read.base.chars().collect();
    let (front, prefix, kept) = default_split(&l, &f);
    make(sp, stem, &read, &l, &f, (front, prefix, kept), lemma_cell)
}

/// Every reading of `form` as an edit of `stem`, the default one first.
/// Alternatives are only looked for when the default keeps less than half
/// of the lemma (a change at the start hides the stem from it).
pub fn candidates(
    sp: &Spelling<'_>,
    stem: Stem<'_>,
    form: &str,
    lemma_cell: bool,
) -> Vec<EditSpec> {
    let read = sp.read(form);
    let l: Vec<char> = stem.letters.chars().collect();
    let f: Vec<char> = read.base.chars().collect();
    let default = default_split(&l, &f);
    let first = make(sp, stem, &read, &l, &f, default, lemma_cell);
    let mut out = vec![first.clone()];
    // The same split with the stress counted from the word's end.
    if !lemma_cell && let Some(e) = from_end(sp, &read, first) {
        out.push(e);
    }
    if lemma_cell || default.2.saturating_mul(2) >= l.len() {
        return out;
    }
    for front in 0..=MAX_FRONT.min(l.len()) {
        for prefix in 0..=MAX_PREFIX.min(f.len()) {
            if (front, prefix) == (default.0, default.1) {
                continue;
            }
            let kept = common(
                l.get(front..).unwrap_or(&[]),
                f.get(prefix..).unwrap_or(&[]),
            );
            if kept == 0 || kept <= default.2 {
                continue;
            }
            let e = make(sp, stem, &read, &l, &f, (front, prefix, kept), lemma_cell);
            if let Some(x) = from_end(sp, &read, e.clone())
                && !out.contains(&x)
            {
                out.push(x);
            }
            if !out.contains(&e) {
                out.push(e);
            }
        }
    }
    out
}

/// `e` with its stress counted from the end of the form, if the form is
/// stressed and that is a different rule.
fn from_end(sp: &Spelling<'_>, read: &flexion::Word<'_>, mut e: EditSpec) -> Option<EditSpec> {
    let s = read.stress?;
    let from_end = u8::try_from(sp.syllable_from_end([&read.base, "", ""], s)?).ok()?;
    if !sp.has_stress() || e.stress == Stress::Unmarked || e.stress == Stress::FromEnd(from_end) {
        return None;
    }
    e.stress = Stress::FromEnd(from_end);
    Some(e)
}

fn common(a: &[char], b: &[char]) -> usize {
    a.iter().zip(b).take_while(|(x, y)| x == y).count()
}

/// `(start stripped, prefix added, letters kept)` of the default reading: a
/// common start, or a long start of the lemma that reappears later in the
/// form (beautiful → more beautiful, machen → gemacht).
fn default_split(l: &[char], f: &[char]) -> (usize, usize, usize) {
    // Exact letters: folding (ё → е) is for lookup only. An edit that kept
    // a folded letter would write the lemma's letter where the form has
    // another (четыре → четырёх).
    let common = common(l, f);
    for k in (common.saturating_add(1).max(3)..=l.len()).rev() {
        let Some(needle) = l.get(..k) else {
            continue;
        };
        let found = (1..f.len().saturating_sub(k).saturating_add(1))
            .find(|&at| f.get(at..at.saturating_add(k)).is_some_and(|w| w == needle));
        if let Some(at) = found {
            return (0, at, k);
        }
    }
    (0, 0, common)
}

fn make(
    sp: &Spelling<'_>,
    stem: Stem<'_>,
    read: &flexion::Word<'_>,
    l: &[char],
    f: &[char],
    (front_len, plen, kept_len): (usize, usize, usize),
    lemma_cell: bool,
) -> EditSpec {
    let collect = |s: Option<&[char]>| s.map(|c| c.iter().collect::<String>()).unwrap_or_default();
    let front = collect(l.get(..front_len));
    let prefix = collect(f.get(..plen));
    let kept = collect(l.get(front_len..front_len.saturating_add(kept_len)));
    let strip = collect(l.get(front_len.saturating_add(kept_len)..));
    let add = collect(f.get(plen.saturating_add(kept_len)..));
    // The lemma's stress counted from the kept stem's start.
    let vf = sp.vowel_count(&front);
    let lemma_stress = stem.stress.and_then(|s| s.checked_sub(vf));
    let stress = rule(sp, &prefix, &kept, lemma_stress, read.stress, lemma_cell);
    let stress = if sp.place(stress, lemma_stress, &prefix, &kept, &add) == read.stress {
        stress
    } else {
        Stress::Unmarked
    };
    // The lemma's own mark (0), or the form's (kind + 1).
    let same = !lemma_cell && stem.stress.is_some() && read.kind == stem.kind;
    let mark = if sp.stress_kinds() <= 1 || read.stress.is_none() || same {
        0
    } else {
        read.kind.saturating_add(1)
    };
    EditSpec {
        front,
        prefix,
        strip,
        add,
        stress,
        mark,
    }
}

fn rule(
    sp: &Spelling<'_>,
    prefix: &str,
    kept: &str,
    lemma_stress: Option<usize>,
    stress: Option<usize>,
    lemma_cell: bool,
) -> Stress {
    let Some(s) = stress else {
        return Stress::Unmarked;
    };
    let vp = sp.vowel_count(prefix);
    let vk = sp.vowel_count(kept);
    let small = |n: usize| u8::try_from(n).ok();
    if !lemma_cell
        && let Some(ls) = lemma_stress
        && ls < vk
        && s == vp.saturating_add(ls)
    {
        return Stress::SameAsLemma;
    }
    if s >= vp.saturating_add(vk) {
        return small(s - vp - vk).map_or(Stress::Unmarked, Stress::Added);
    }
    if s >= vp {
        let from_start = s - vp;
        let from_end = vk.saturating_sub(1).saturating_sub(from_start);
        if !lemma_cell && from_end != 0 && from_start == 0 {
            return Stress::KeptFromStart(0);
        }
        return small(from_end).map_or(Stress::Unmarked, Stress::KeptFromEnd);
    }
    small(s).map_or(Stress::Unmarked, Stress::Prefix)
}

/// The reading chosen for each (lemma, form) pair of a part of speech, where
/// it isn't the default one.
#[derive(Default)]
pub struct Chooser<'a> {
    picks: HashMap<(&'a str, &'a str), EditSpec>,
}

fn digest(e: &EditSpec) -> u64 {
    let mut h = std::collections::hash_map::DefaultHasher::new();
    e.hash(&mut h);
    h.finish()
}

impl<'a> Chooser<'a> {
    /// Count every reading of every pair in `words`, then give each pair the
    /// reading the most pairs share (the default one on a tie). Two passes
    /// over the pairs, so only the counts are kept in between.
    pub fn new(sp: &Spelling<'_>, words: &[Word<'a>], lemma_cell: usize) -> Chooser<'a> {
        let pairs = || {
            words
                .iter()
                .flat_map(|w| w.lexemes.iter())
                .flat_map(move |lex| {
                    lex.cells
                        .iter()
                        .enumerate()
                        .filter(move |(cell, _)| *cell != lemma_cell)
                        .flat_map(move |(_, forms)| forms.iter().map(move |f| (*lex, f)))
                })
        };
        let stem = |lex: &'a crate::extract::Lexeme| Stem {
            letters: &lex.full,
            stress: lex.lemma_stress,
            kind: lex.lemma_kind,
        };
        let mut counts: HashMap<u64, u32> = HashMap::new();
        for (lex, form) in pairs() {
            for e in candidates(sp, stem(lex), form, false) {
                *counts.entry(digest(&e)).or_default() += 1;
            }
        }
        let mut picks = HashMap::new();
        for (lex, form) in pairs() {
            let c = candidates(sp, stem(lex), form, false);
            if c.len() < 2 {
                continue;
            }
            let count = |e: &EditSpec| counts.get(&digest(e)).copied().unwrap_or(0);
            let mut best = 0usize;
            for (i, e) in c.iter().enumerate() {
                if c.get(best).is_some_and(|b| count(e) > count(b)) {
                    best = i;
                }
            }
            if best > 0
                && let Some(e) = c.into_iter().nth(best)
            {
                picks.insert((lex.full.as_str(), form.as_str()), e);
            }
        }
        Chooser { picks }
    }

    /// The chosen edit for a pair.
    pub fn edit(
        &self,
        sp: &Spelling<'_>,
        stem: Stem<'_>,
        form: &str,
        lemma_cell: bool,
    ) -> EditSpec {
        if !lemma_cell && let Some(e) = self.picks.get(&(stem.letters, form)) {
            return e.clone();
        }
        edit(sp, stem, form, lemma_cell)
    }

    /// Whether any pair's chosen reading strips the start of its lemma.
    pub fn strips_fronts(&self) -> bool {
        self.picks.values().any(|e| !e.front.is_empty())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use flexion::SpellingStrings;

    fn en() -> Spelling<'static> {
        Spelling::new(
            true,
            false,
            SpellingStrings {
                vowels: "aeiouy",
                ..SpellingStrings::default()
            },
        )
    }

    fn ru() -> Spelling<'static> {
        Spelling::new(
            true,
            true,
            SpellingStrings {
                stress_marks: "\u{301}",
                vowels: "аеёиоуыэюя",
                implicit: "ё",
                fold_from: "ё",
                fold_to: "е",
                strip_marks: "\u{300}",
                ..SpellingStrings::default()
            },
        )
    }

    fn e(prefix: &str, strip: &str, add: &str, stress: Stress) -> EditSpec {
        EditSpec {
            front: String::new(),
            prefix: prefix.into(),
            strip: strip.into(),
            add: add.into(),
            stress,
            mark: 0,
        }
    }

    fn plain(s: &str) -> Stem<'_> {
        Stem {
            letters: s,
            stress: None,
            kind: 0,
        }
    }

    fn stressed(s: &str, at: usize) -> Stem<'_> {
        Stem {
            letters: s,
            stress: Some(at),
            kind: 0,
        }
    }

    #[test]
    fn suffix_and_prefix_edits() {
        let sp = en();
        assert_eq!(
            edit(&sp, plain("city"), "cities", false),
            e("", "y", "ies", Stress::Unmarked)
        );
        assert_eq!(
            edit(&sp, plain("sing"), "sang", false),
            e("", "ing", "ang", Stress::Unmarked)
        );
        assert_eq!(
            edit(&sp, plain("beautiful"), "more beautiful", false),
            e("more ", "", "", Stress::Unmarked)
        );
        assert_eq!(
            edit(&sp, plain("good"), "better", false),
            e("", "good", "better", Stress::Unmarked)
        );
    }

    #[test]
    fn readings_that_change_the_start() {
        let sp = en();
        let c = candidates(&sp, plain("singen"), "gesungen", false);
        assert!(c.contains(&e("ge", "ingen", "ungen", Stress::Unmarked)));
        let c = candidates(&sp, plain("akouo"), "ekousa", false);
        assert!(
            c.iter()
                .any(|x| x.front == "a" && x.prefix == "e" && x.strip == "o")
        );
    }

    #[test]
    fn folded_letters_are_still_edits() {
        // ё folds to е for lookup, but an edit must write the form's own letter.
        let ed = edit(&ru(), stressed("сестра", 1), "сёстры", false);
        assert_eq!((ed.strip.as_str(), ed.add.as_str()), ("естра", "ёстры"));
    }

    #[test]
    fn stress_rules() {
        let sp = ru();
        // соба́ка → соба́ке: stress stays where the lemma has it.
        assert_eq!(
            edit(&sp, stressed("собака", 1), "соба\u{301}ке", false).stress,
            Stress::SameAsLemma
        );
        // стол → стола́: on the ending.
        assert_eq!(
            edit(&sp, stressed("стол", 0), "стола\u{301}", false).stress,
            Stress::Added(0)
        );
        // рука́ → ру́ки: onto the stem's last vowel.
        assert_eq!(
            edit(&sp, stressed("рука", 1), "ру\u{301}ки", false).stress,
            Stress::KeptFromEnd(0)
        );
        // голова́ → го́ловы: back to the stem's first vowel.
        assert_eq!(
            edit(&sp, stressed("голова", 2), "го\u{301}ловы", false).stress,
            Stress::KeptFromStart(0)
        );
        // The lemma cell counts from the word's end.
        assert_eq!(
            edit(&sp, plain("собака"), "соба\u{301}ка", true).stress,
            Stress::KeptFromEnd(1)
        );
    }
}
