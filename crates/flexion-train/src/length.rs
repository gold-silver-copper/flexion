//! How much of a word's vowel length (the marks its key leaves off) its
//! letters tell: a diagnostic, never in a crate. Per letter that may take a
//! mark, the value training words show most often in the longest context
//! around it they share: first the word's root (another word beginning with
//! the same letters through two past it: `iūdex` of `iūdicō`), then letters
//! on either side (up to [`LEFT`] before it, [`RIGHT`] after) with the part
//! of speech. Each kind of cue can be left out, to see what it adds.

use crate::HashMap;
use flexion::{Orthography, WORD_END};
use std::hash::{Hash, Hasher};

/// The most letters read before a letter.
pub const LEFT: usize = 4;
/// The most letters read after a letter.
pub const RIGHT: usize = 6;

/// A word: how it is typed without marks, how the dictionary spells it,
/// and its part of speech.
#[derive(Debug, Clone, Copy)]
pub struct Spelled<'a> {
    pub key: &'a str,
    pub letters: &'a str,
    pub part: &'a str,
}

/// A kind of cue.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cue {
    Left,
    /// The letters before it but the nearest (the marks rules read that
    /// one).
    FarLeft,
    Right,
    Part,
    Root,
}

/// Every kind of cue.
pub const CUES: [Cue; 5] = [Cue::Left, Cue::FarLeft, Cue::Right, Cue::Part, Cue::Root];

/// The values seen in each context, by its hash.
#[derive(Default)]
pub struct Model {
    seen: HashMap<u64, Vec<(char, u32)>>,
}

/// A context: a root (a word's beginning, see [`flexion::Marks::root_of`]),
/// or letters around a letter, with the part of speech or not.
#[derive(Hash)]
enum Context<'a> {
    Root(&'a str),
    Around(&'a [char], char, &'a [char], Option<&'a str>),
}

fn hash(context: &Context<'_>) -> u64 {
    let mut h = crate::Fnv::default();
    context.hash(&mut h);
    h.finish()
}

/// Each letter that may take a mark: its place among the key's letters, the
/// letter typed, and the letter spelled.
fn markable(o: &Orthography<'_>, w: &Spelled<'_>) -> Vec<(usize, char, char)> {
    let spelled: Vec<char> = w.letters.chars().collect();
    w.key
        .chars()
        .enumerate()
        .filter(|&(_, k)| o.folded.iter().any(|e| e.1 == k))
        .filter_map(|(i, k)| Some((i, k, *spelled.get(i)?)))
        .collect()
}

/// The contexts of the letter at `i` of `w`'s key (`ended`: the key with
/// `#` at either end), the longest first, as `without` leaves them.
fn contexts<'a>(
    w: &Spelled<'a>,
    ended: &'a [char],
    i: usize,
    without: Option<Cue>,
) -> Vec<Context<'a>> {
    let mut out = Vec::new();
    let part = w.part;
    if without != Some(Cue::Root)
        && let Some(root) = flexion::Marks::root_of(w.key, i)
    {
        out.push(Context::Root(root));
    }
    // `ended` has `#` before the key's first letter.
    let (i, key) = (i + 1, ended);
    let Some(&letter) = key.get(i) else {
        return out;
    };
    let lefts = match without {
        Some(Cue::Left) => 0,
        Some(Cue::FarLeft) => 1,
        _ => LEFT,
    };
    let rights = if without == Some(Cue::Right) {
        0
    } else {
        RIGHT
    };
    let mut around: Vec<(usize, usize)> = (0..=lefts)
        .flat_map(|l| (0..=rights).map(move |r| (l, r)))
        .filter(|&(l, r)| l <= i && i + 1 + r <= key.len())
        .collect();
    around.sort_by_key(|&(l, r)| std::cmp::Reverse((l + r, r)));
    for (l, r) in around {
        let (Some(left), Some(right)) = (key.get(i - l..i), key.get(i + 1..i + 1 + r)) else {
            continue;
        };
        if without != Some(Cue::Part) {
            out.push(Context::Around(left, letter, right, Some(part)));
        }
        out.push(Context::Around(left, letter, right, None));
    }
    out
}

/// The key's letters with `#` at either end.
fn ended(key: &str) -> Vec<char> {
    std::iter::once(WORD_END)
        .chain(key.chars())
        .chain([WORD_END])
        .collect()
}

impl Model {
    /// What `words` show, in every context.
    pub fn learn(o: &Orthography<'_>, words: &[Spelled<'_>]) -> Model {
        let mut model = Model::default();
        for w in words {
            let key = ended(w.key);
            for (i, _, value) in markable(o, w) {
                for c in contexts(w, &key, i, None) {
                    let values = model.seen.entry(hash(&c)).or_default();
                    match values.iter_mut().find(|(v, _)| *v == value) {
                        Some((_, n)) => *n += 1,
                        None => values.push((value, 1)),
                    }
                }
            }
        }
        model
    }

    /// The word's spelling, as the model reads it without `without`.
    pub fn spell(&self, o: &Orthography<'_>, w: &Spelled<'_>, without: Option<Cue>) -> String {
        let key = ended(w.key);
        let mut out: Vec<char> = w.key.chars().collect();
        for (i, typed, _) in markable(o, w) {
            let value = contexts(w, &key, i, without)
                .iter()
                .find_map(|c| self.seen.get(&hash(c)))
                .and_then(|values| {
                    // The commonest; of as common, the letter as typed.
                    values
                        .iter()
                        .max_by_key(|&&(v, n)| (n, v == typed, std::cmp::Reverse(v)))
                        .map(|&(v, _)| v)
                });
            if let (Some(slot), Some(v)) = (out.get_mut(i), value) {
                *slot = v;
            }
        }
        out.into_iter().collect()
    }
}
