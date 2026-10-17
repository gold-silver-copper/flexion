//! A part of speech's grammar, and how it answers: which paradigm a word
//! follows, how it is stressed, and what its forms are, with candidates
//! that say how sure they are and why.

use crate::choice::{Choice, Reading, Status};
use crate::orthography::{Casing, Orthography, Written};
use crate::paradigm::{Paradigm, Relation};
use crate::pattern;
use crate::stems::{EndingTable, Slot};
use crate::stress::{self, Accent, Place};
use crate::{Candidate, Evidence, Label};
use std::borrow::Cow;

/// How [`Grammar::marks`] writes a word's ends.
pub const WORD_END: char = '#';

/// How much likelier a word's paradigm is to share its inherent feature
/// values than not: a paradigm states a value nineteen in twenty of its
/// words have.
const OTHER_VALUES: f32 = 0.05;

/// Readings of each kind kept when combining them into candidates.
const KEEP: usize = 4;

/// The paradigm readings a word's table is built from, the likeliest.
pub const READINGS: usize = 4;

/// What joins the parts of a word written in parts: a hyphen, or a space.
const JOINERS: [char; 2] = ['-', ' '];

/// The fewest letters before a compound's last part (see [`Grammar::head`]).
pub const FRONT_LETTERS: usize = 3;

/// How a word written in parts (`диван-кровать`) inflects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Compound {
    /// Only its last part, like a word of its own: `интернет-магазин`,
    /// `интернет-магазина`.
    Last,
    /// Each part, as the word it is: `диван-кровать`, `дивана-кровати`.
    Each,
    /// Only its first part, the rest kept, a word apart: `pyytää anteeksi`,
    /// `en pyydä anteeksi`.
    First,
}

impl Label for Compound {
    fn index(self) -> usize {
        self as usize
    }
    fn from_index(index: usize) -> Option<Self> {
        [Compound::Last, Compound::Each, Compound::First]
            .get(index)
            .copied()
    }
}

/// Prefixes a word inflects behind (German `aus-`, `ver-`): a word that
/// begins with one may inflect as the rest of it does. A separable prefix
/// goes where the cell puts it (`räuchre aus`, `ausgeräuchert`); an
/// inseparable one stays in front, and takes off the rest's form what the
/// cell says (`verhindert`, not `vergehindert`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Prefixes<'a> {
    /// Sorted by their letters.
    pub prefixes: Cow<'a, [Prefix<'a>]>,
    /// Words that do with their prefix what few words with it do, sorted.
    pub listed: Cow<'a, [(Cow<'a, str>, Separation)]>,
    /// Per cell, where a separable prefix goes.
    pub placement: Cow<'a, [Placement]>,
    /// Per cell, what an inseparable prefix takes off the front of the rest's
    /// form (the participle's `ge`).
    pub drops: Cow<'a, [Cow<'a, str>]>,
    /// How a prefix kept in front meets the rest's form: `(prefix, head,
    /// joined)`, the prefix (as it is looked up) and the form's first letter
    /// becoming `joined` (Greek `ἀπο` and the augment's `ἐ`: `ἀπε`,
    /// `ἀπέλυον`), sorted. Where none is listed, the prefix is written before
    /// the form.
    pub junctions: Cow<'a, [Junction<'a>]>,
}

/// How a prefix meets a form: `(prefix, head, joined)`.
type Junction<'a> = (Cow<'a, str>, Cow<'a, str>, Cow<'a, str>);

/// A prefix, and of the dictionary's words that begin with it, how many
/// separate it, how many keep it in front, of how many.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Prefix<'a> {
    pub letters: Cow<'a, str>,
    pub separable: u32,
    pub inseparable: u32,
    pub words: u32,
}

/// What a word does with the prefix it begins with.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Separation {
    /// Separates it (`räuchre aus`).
    Separable,
    /// Keeps it in front (`verhindert`).
    Inseparable,
    /// Isn't made of it (`rauschen`, not `raus-chen`).
    Neither,
}

/// Where a separable prefix goes in a cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Placement {
    /// Before the rest, written together: `ausgeräuchert`.
    Before,
    /// After it, as a word of its own: `räuchre aus`.
    After,
}

impl Prefixes<'_> {
    /// The rest of `form` (letters) kept behind `prefix` (looked up as `key`),
    /// as it is before they meet (see [`Prefixes::join`]); `None` where it
    /// isn't.
    pub fn unjoin(&self, key: &str, prefix: &str, form: &str) -> Option<String> {
        self.junctions
            .iter()
            .filter(|(k, _, joined)| k == key && form.starts_with(joined.as_ref()))
            .max_by_key(|(_, _, joined)| joined.len())
            .and_then(|(_, head, joined)| Some(format!("{head}{}", form.get(joined.len()..)?)))
            .or_else(|| form.strip_prefix(prefix).map(str::to_string))
    }

    /// The letters of `prefix` (looked up as `key`) kept in front of `rest`
    /// (letters), as they meet (see [`Prefixes::junctions`]), and where
    /// `rest`'s letters after its first start in them.
    pub fn join(&self, key: &str, prefix: &str, rest: &str) -> (String, usize) {
        let head_len = rest.chars().next().map_or(0, char::len_utf8);
        let (head, tail) = rest.split_at(head_len);
        let at = self
            .junctions
            .binary_search_by(|(p, h, _)| (p.as_ref(), h.as_ref()).cmp(&(key, head)))
            .ok()
            .and_then(|i| self.junctions.get(i));
        match at {
            Some((_, _, joined)) => (format!("{joined}{tail}"), joined.len()),
            None => (format!("{prefix}{rest}"), prefix.len() + head_len),
        }
    }

    /// The prefix the word looked up as `key` begins with (the longest, with
    /// at least three letters after it), its length in letters, and how
    /// likely the word is to separate it and to keep it in front.
    pub fn split(&self, key: &str) -> Option<(usize, f32, f32)> {
        let prefix = self
            .prefixes
            .iter()
            .filter(|p| {
                key.starts_with(p.letters.as_ref())
                    && key.chars().count() >= p.letters.chars().count() + 3
            })
            .max_by_key(|p| p.letters.len())?;
        let listed = self
            .listed
            .binary_search_by(|(w, _)| w.as_ref().cmp(key))
            .ok()
            .and_then(|i| self.listed.get(i))
            .map(|e| e.1);
        #[allow(clippy::cast_precision_loss)]
        let (separable, inseparable) = match listed {
            Some(Separation::Separable) => (1.0, 0.0),
            Some(Separation::Inseparable) => (0.0, 1.0),
            Some(Separation::Neither) => (0.0, 0.0),
            None => {
                let words = prefix.words as f32 + 3.0;
                (
                    (prefix.separable as f32 + 1.0) / words,
                    (prefix.inseparable as f32 + 1.0) / words,
                )
            }
        };
        Some((prefix.letters.chars().count(), separable, inseparable))
    }
}

/// The forms no rule gives, as a table: a line per word, sorted, its key
/// (see [`Orthography::key`]), then each listed stem's slot and the stem
/// with its ending table (`aorist`, `ἀγγειλ- like λυω`: see
/// [`crate::LIKE`]), then each listed cell's number and form, all
/// tab-separated, slots and cells in order.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Irregular<'a> {
    pub table: Cow<'a, str>,
}

/// A listed stem: `(word, slot, stem, table)`.
pub type ListedStem = (String, String, String, String);

impl Irregular<'_> {
    /// The table of `forms`, `(word, cell, form)`, and `stems`.
    pub fn new(forms: &[(String, usize, String)], stems: &[ListedStem]) -> Irregular<'static> {
        let mut rows: Vec<(&str, usize, &str)> = forms
            .iter()
            .map(|(w, c, f)| (w.as_str(), *c, f.as_str()))
            .collect();
        rows.sort_unstable();
        rows.dedup_by(|a, b| a.0 == b.0 && a.1 == b.1);
        let mut stems: Vec<&ListedStem> = stems.iter().collect();
        stems.sort();
        stems.dedup_by(|a, b| a.0 == b.0 && a.1 == b.1);
        let mut words: Vec<&str> = rows
            .iter()
            .map(|r| r.0)
            .chain(stems.iter().map(|s| s.0.as_str()))
            .collect();
        words.sort_unstable();
        words.dedup();
        let mut table = String::new();
        let (mut r, mut s) = (0, 0);
        for w in words {
            table.push_str(w);
            while let Some((_, slot, stem, like)) = stems.get(s).filter(|e| e.0 == w) {
                table.push_str(&format!("\t{slot}\t{stem}{}{like}", crate::LIKE));
                s += 1;
            }
            while let Some((_, c, f)) = rows.get(r).filter(|e| e.0 == w) {
                table.push_str(&format!("\t{c}\t{f}"));
                r += 1;
            }
            table.push('\n');
        }
        Irregular {
            table: Cow::Owned(table),
        }
    }

    /// How many forms the table lists.
    pub fn len(&self) -> usize {
        self.entries()
            .filter(|(c, _)| c.parse::<u16>().is_ok())
            .count()
    }

    /// How many stems the table lists.
    pub fn stems(&self) -> usize {
        self.entries()
            .filter(|(c, _)| c.parse::<u16>().is_err())
            .count()
    }

    /// Every line's entries: `(cell, form)` and `(slot, stem with table)`.
    fn entries(&self) -> impl Iterator<Item = (&str, &str)> {
        self.table.lines().flat_map(|l| {
            let mut parts = l.split('\t').skip(1);
            std::iter::from_fn(move || Some((parts.next()?, parts.next()?)))
        })
    }

    pub fn is_empty(&self) -> bool {
        self.table.is_empty()
    }

    /// The form of `cell` for the word looked up as `word`, if listed.
    pub fn get(&self, word: &str, cell: u16) -> Option<&str> {
        listed_in(self.line(word)?, cell)
    }

    /// Whether the word looked up as `word` has listed forms.
    pub fn has(&self, word: &str) -> bool {
        self.line(word).is_some()
    }

    /// The word's line (a search through the whole table: a caller asking
    /// for several cells looks it up once).
    fn line(&self, word: &str) -> Option<&str> {
        let table = self.table.as_ref();
        let (mut low, mut high) = (0, table.len());
        while low < high {
            let middle = (low + (high - low) / 2..=high)
                .find(|&i| table.is_char_boundary(i))
                .unwrap_or(high);
            let start = table
                .get(..middle)?
                .rfind('\n')
                .map_or(0, |i| i + 1)
                .max(low);
            let end = table
                .get(start..)?
                .find('\n')
                .map_or(table.len(), |i| start + i);
            let line = table.get(start..end)?;
            match line.split('\t').next()?.cmp(word) {
                std::cmp::Ordering::Less => low = end + 1,
                std::cmp::Ordering::Greater => high = start,
                std::cmp::Ordering::Equal => return Some(line),
            }
        }
        None
    }
}

/// The stem a word's line of the irregular table lists for the slot named
/// `slot`, and its ending table's name.
fn stem_in<'l>(line: &'l str, slot: &str) -> Option<(&'l str, &'l str)> {
    let mut parts = line.split('\t').skip(1);
    while let (Some(s), Some(entry)) = (parts.next(), parts.next()) {
        if s == slot {
            return crate::stems::listed_stem(entry);
        }
    }
    None
}

/// The form of `cell` in a word's line of the irregular table, if listed.
fn listed_in(line: &str, cell: u16) -> Option<&str> {
    spellings_in(line, cell).and_then(|f| pattern::variants(f).next())
}

/// The forms `line` lists of `cell`, as the dictionary spells them: the
/// usual one first, others after `|` (Italian `i|gli`).
fn spellings_in(line: &str, cell: u16) -> Option<&str> {
    let mut parts = line.split('\t').skip(1);
    while let (Some(c), Some(f)) = (parts.next(), parts.next()) {
        if c.parse::<u16>().ok() == Some(cell) {
            return Some(f);
        }
    }
    None
}

/// What the caller knows about a word besides its lemma: forms of it, such
/// as principal parts, as `(cell, form)`; the values of its inherent
/// features (its gender, its animacy); and whether the lemma is spelled in
/// full. Readings that disagree are dropped.
#[derive(Debug, Clone, Copy, Default)]
pub struct Known<'q> {
    pub forms: &'q [(usize, &'q str)],
    /// Values of its inherent features, as `(feature, value)` by their
    /// places: the paradigms of other values are dropped, and a relation
    /// that depends on one takes the cell this value says.
    pub inherent: &'q [(u8, u8)],
    /// The lemma is spelled with every mark the dictionary writes (`fōx`,
    /// `ёж`, not `fox`, `еж`): none is hidden.
    pub full: bool,
}

/// Which words a word ending in one inflects as (compounds, Finnish
/// `aavameri` as `meri`): every listed one of at least `least` letters,
/// where the language's compounds do, but those `not`, whose own words
/// don't (Latin `capitālis`, `digitālis` don't inflect as `Ītalis`); and
/// those `words`, whose own compounds do, listed or not (one not listed
/// inflects as a word of its own: Finnish `arvomerkintä` as `merkintä`,
/// `arvomerkinnät`, its last member deciding its harmony).
#[derive(Debug, Clone, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Heads<'a> {
    pub least: Option<u8>,
    /// Sorted.
    pub not: Cow<'a, [Cow<'a, str>]>,
    /// Sorted.
    pub words: Cow<'a, [Cow<'a, str>]>,
}

impl Heads<'_> {
    /// No word inflects as a word it ends in.
    pub fn is_empty(&self) -> bool {
        self.least.is_none() && self.words.is_empty()
    }

    /// Whether a word ending in `head` (a listed word) inflects as it.
    pub fn inflects(&self, head: &str) -> bool {
        self.least.is_some_and(|least| {
            head.chars().count() >= usize::from(least) && !has(&self.not, head)
        }) || self.names(head)
    }

    /// Whether `head` is one of `words`: a word ending in it inflects as
    /// it, listed or not.
    pub fn names(&self, head: &str) -> bool {
        has(&self.words, head)
    }
}

/// Whether the sorted `words` have `word`.
fn has(words: &[Cow<'_, str>], word: &str) -> bool {
    words.binary_search_by(|w| w.as_ref().cmp(word)).is_ok()
}

/// How a letter typed without its mark is written: rules on the letter
/// before it, it, the letters after it and, after the word's end, the part
/// of speech's name (rules on the beginning of that, the word's ends written
/// [`WORD_END`]: Latin `tatio#` → `ā`, as in `-ātiō`; `#de` → `ē`, as in
/// `dē-`; `ae` → `e`, a diphthong; `eni#numeral` → `ē`, as in `quīngēnī`,
/// where adjectives in `-genus` have `e`). One language's rules serve every
/// part of speech, each read with its `part`. A root the rules don't tell
/// comes first (see [`Marks::roots`]).
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Marks<'a> {
    pub rules: Choice<'a, char>,
    /// Word beginnings whose letter third from their end nine in ten of the
    /// dictionary's words with a form beginning so (two or more) write one
    /// way the rules don't: `("iudi", 'ū')`, a word typed `iudicium` writes
    /// `ū`, as `iūdicō` and `iūdicātiō` do. Sorted. A part of speech they
    /// would mislead reads none.
    pub roots: Cow<'a, [(Cow<'a, str>, char)]>,
    pub part: Cow<'a, str>,
}

/// The fewest letters of a root (see [`Marks::roots`]).
pub const ROOT_LETTERS: usize = 4;

/// How likely a letter is as its root writes it: the share of the letters
/// roots decide on held-out Latin and Greek lemmas that they get right (244
/// of 250).
const ROOT_TRUST: f32 = 0.97;

impl Marks<'_> {
    /// How the root of `key` through the letter at char `i` and two past it
    /// writes that letter, if it is listed.
    pub fn root(&self, key: &str, i: usize) -> Option<char> {
        let end = i + 3;
        if end < ROOT_LETTERS {
            return None;
        }
        let root = match key.char_indices().nth(end) {
            Some((at, _)) => key.get(..at)?,
            None if key.chars().count() == end => key,
            None => return None,
        };
        let at = self.roots.binary_search_by(|(r, _)| r.as_ref().cmp(root));
        at.ok().and_then(|at| self.roots.get(at)).map(|(_, v)| *v)
    }
}

/// The inflection of one part of speech, as learned from the dictionary.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Grammar<'a, P: Label, A: Label = u8, F: Label = u8> {
    /// Not serialized: it borrows the language's script, so whoever loads a
    /// grammar sets it again (it loads as [`Orthography::default`]).
    #[cfg_attr(feature = "serde", serde(skip))]
    pub orthography: Orthography<'a>,
    /// How many cells a word has.
    pub cells: usize,
    /// The cell the lemma itself fills, if any.
    pub lemma_cell: Option<usize>,
    /// Every word is written with a capital letter (German nouns).
    pub capitalized: bool,
    /// Indexed by `P`.
    pub paradigms: Cow<'a, [Paradigm<'a>]>,
    /// Which paradigm a word follows, by the word written after
    /// [`WORD_END`] (`#lux`: the whole word `lux`, not `pollux`).
    pub paradigm: Choice<'a, P>,
    /// Per cell (at most one each, by cell), where its form comes from when
    /// a paradigm derives it, or the word's form of the cell it comes from
    /// is listed.
    pub relations: Cow<'a, [Relation<'a>]>,
    /// The inherent feature relations depend on (its place), and which
    /// value of it a word has.
    pub feature: Option<(u8, Choice<'a, F>)>,
    /// Prefixes words inflect behind, where the part of speech has them.
    pub prefixes: Option<Prefixes<'a>>,
    /// Where words inflect their last part (compounds; see
    /// [`Grammar::head`]).
    pub heads: Heads<'a>,
    /// Which words written in parts inflect each part or only the first,
    /// read off the first part (`диван` of `диван-кровать`).
    pub compound: Option<Choice<'a, Compound>>,
    /// Where a word that inflects its first part keeps the rest, per cell:
    /// after it (Finnish `otan kiinni`) or before it (`kiinni ottava`).
    /// Empty: after it in every cell.
    pub kept: Cow<'a, [Placement]>,
    /// Mobile stress, where the language writes it.
    pub accent: Option<Accent<'a, A>>,
    /// Sound laws and spelling rules every built form goes through.
    pub laws: Option<sound_law::Program<'static>>,
    /// Words whose spelling their key doesn't show, as the dictionary writes
    /// them: `(key, letters)`, sorted. A caller who types `fox` means `fōx`;
    /// marks a paradigm writes (the `ō` of `ratiō`) aren't listed.
    pub spellings: Cow<'a, [(Cow<'a, str>, Cow<'a, str>)]>,
    /// How a letter typed without its mark is written (see [`Marks`]): the
    /// spelling of a word that isn't listed.
    pub marks: Option<Marks<'a>>,
    /// The stem slots: the cells words build on one stem each.
    pub slots: Cow<'a, [Slot<'a>]>,
    /// The ending tables slots' cells are built with, on a listed stem.
    pub tables: Cow<'a, [EndingTable<'a>]>,
    /// Forms no rule gives, as the dictionary writes them, and stems words
    /// list (see [`Irregular`]). They are the answer for that word.
    pub irregular: Irregular<'a>,
    /// How many words' worth a broader rule's counts add to a specific
    /// rule's (see [`Choice::read`]).
    pub smoothing: f32,
    /// The power candidates' probabilities are raised to (then made to sum
    /// to one again), fitted on held-out words so that what is said to be
    /// 80% likely is right about 80% of the time.
    pub sharpness: f32,
    /// How much of its probability an answer keeps, by its status (see
    /// [`Trust`]).
    pub trust: Trust,
}

/// A cell's forms, the usual first, each with where its ending starts.
type Forms = Vec<(String, usize)>;

/// A reading's paradigm and its forms of the cells built so far: its line's
/// tables, each read once; per cell, its forms with where their endings
/// start, by how likely; and the stems the forms the caller gives fix, per
/// slot: `(slot, ending table, stem)`.
struct Built<'g, 'a> {
    tables: crate::paradigm::Tables<'g, 'a>,
    cells: std::collections::HashMap<usize, Vec<(Forms, f32)>>,
    fixed: Vec<(usize, String, String)>,
}

/// An accent class a word may be of: how likely, its accent's place in
/// each cell, and the position (see [`stress::position`]) its stem accents.
type Class = (f32, Vec<Option<Place>>, usize);

/// One way of reading the word: a paradigm with the word's stems.
struct Inflection<'g, 'a, 'w, P: Label> {
    reading: Reading<'g, 'a, P>,
    paradigm: &'g Paradigm<'a>,
    stems: pattern::Stems<'w>,
}

impl<'a, P: Label, A: Label, F: Label> Grammar<'a, P, A, F> {
    /// The forms of `cell` for `lemma`, likeliest first, given what the
    /// caller knows (readings that disagree with it are dropped, unless all
    /// do).
    pub fn candidates(&self, lemma: &str, cell: usize, known: &Known<'_>) -> Vec<Candidate<'_>> {
        let given = self.informative(lemma, known);
        let known = Known {
            forms: &given,
            ..*known
        };
        self.sharpened(self.unsharpened(lemma, &[cell], &known))
            .pop()
            .unwrap_or_default()
    }

    /// The forms of every cell for `lemma`, as [`Grammar::candidates`]
    /// gives each, read once.
    pub fn table(&self, lemma: &str, known: &Known<'_>) -> Vec<Vec<Candidate<'_>>> {
        let given = self.informative(lemma, known);
        let known = Known {
            forms: &given,
            ..*known
        };
        let cells: Vec<usize> = (0..self.cells).collect();
        self.sharpened(self.unsharpened(lemma, &cells, &known))
    }

    /// The forms `known` gives that tell the grammar something. Of a word the
    /// grammar lists, the dictionary's own, a form it already gives first
    /// tells nothing, and changes nothing (Sanskrit `खादति` given its perfect
    /// `चखाद`, which its line lists, keeps its augment: `अखादम्`). Of
    /// another word, every given form tells which readings it can have.
    fn informative<'k>(&self, lemma: &str, known: &Known<'k>) -> Vec<(usize, &'k str)> {
        let o = &self.orthography;
        let key = o.key(&o.read(lemma).letters);
        if known.forms.is_empty() || !self.lists(&key) {
            return known.forms.to_vec();
        }
        let cells: Vec<usize> = known.forms.iter().map(|&(c, _)| c).collect();
        let bare = Known {
            forms: &[],
            ..*known
        };
        let firsts = self.sharpened(self.unsharpened(lemma, &cells, &bare));
        let same = |a: &str, b: &str| {
            let (a, b) = (o.read(a), o.read(b));
            a.letters == b.letters && a.stress == b.stress
        };
        known
            .forms
            .iter()
            .zip(&firsts)
            .filter(|((_, form), built)| !built.first().is_some_and(|c| same(&c.form, form)))
            .map(|(&given, _)| given)
            .collect()
    }

    /// Each list of candidates sharpened and sorted, likeliest first.
    fn sharpened<'g>(&self, mut lists: Vec<Vec<Candidate<'g>>>) -> Vec<Vec<Candidate<'g>>> {
        for out in &mut lists {
            sharpen(out.iter_mut().map(|c| &mut c.probability), self.sharpness);
            for c in out.iter_mut() {
                c.probability *= self.trust.of(c.status);
            }
            out.sort_by(|a, b| b.probability.total_cmp(&a.probability));
        }
        lists
    }

    /// [`Grammar::candidates`] of each of `cells`, before sharpening, in no
    /// order.
    fn unsharpened(
        &self,
        lemma: &str,
        cells: &[usize],
        known: &Known<'_>,
    ) -> Vec<Vec<Candidate<'_>>> {
        let mut word = self.orthography.read(lemma);
        let key = self.orthography.key(&word.letters);
        // A word typed without some of its marks: as listed (even typed in
        // full: the dictionary's headword may leave off marks its forms
        // have), or as its letters say.
        let mut spelled = 1.0;
        let mut otherwise = Vec::new();
        match (self.spelling(&word.letters, &key), &self.marks) {
            (Some(letters), _) => word.letters = letters.to_string(),
            (None, Some(marks)) if !known.full => {
                let o = &self.orthography;
                ((word.letters, spelled), otherwise) =
                    restorations(o, marks, self.smoothing, &word.letters, &key);
            }
            (None, _) => {}
        }
        let casing = if self.capitalized {
            Casing::Title
        } else {
            word.casing
        };
        // A word ending in a listed one (its head, `meri` of Finnish
        // `aavameri`): the head's forms, behind the letters before it. Not
        // where the word is typed with an accent its head's lemma doesn't
        // have there (`κατάβασις`, `βάσις`; `συναγωγή`, `γῆ`): its own
        // reading.
        let headed = self.head(&key, &word.letters).and_then(|(front, head)| {
            let Some(stress) = word.stress else {
                return Some((front, head.to_string(), None));
            };
            let before = u8::try_from(self.orthography.vowel_count(front)).ok()?;
            let vowel = stress.vowel.checked_sub(before)?;
            let stress = crate::Stress { vowel, ..stress };
            let typed = self.orthography.write(head, Some(stress), Casing::Lower);
            Some((front, typed, Some(stress)))
        });
        if let Some((front, head, typed)) = headed {
            let given: Vec<(usize, String)> = known
                .forms
                .iter()
                .filter_map(|&(c, f)| {
                    let letters = self.orthography.read(f).letters;
                    Some((c, letters.strip_prefix(front)?.to_string()))
                })
                .collect();
            let given: Vec<(usize, &str)> = given.iter().map(|(c, f)| (*c, f.as_str())).collect();
            let base = Known {
                forms: &given,
                ..*known
            };
            let lists = self.unsharpened(&head, cells, &base);
            let stress = |lists: &[Vec<Candidate<'_>>], at: usize| {
                let first = lists.get(at)?.first()?;
                Some(self.orthography.read(&first.form).stress)
            };
            let keeps = match (typed, self.lemma_cell) {
                (Some(typed), Some(lemma)) => {
                    let first = match cells.iter().position(|&c| c == lemma) {
                        Some(at) => stress(&lists, at),
                        None => stress(&self.unsharpened(&head, &[lemma], &base), 0),
                    };
                    first == Some(Some(typed))
                }
                _ => true,
            };
            if keeps {
                let joined =
                    |form: &str| crate::recase(&format!("{front}{}", form.to_lowercase()), casing);
                return lists
                    .into_iter()
                    .map(|list| {
                        list.into_iter()
                            .map(|c| Candidate {
                                form: joined(&c.form),
                                also: c.also.iter().map(|a| joined(a)).collect(),
                                ..c
                            })
                            .collect()
                    })
                    .collect();
            }
        }
        // The forms the caller gives, as letters.
        let given: Vec<(usize, String)> = known
            .forms
            .iter()
            .map(|&(c, f)| (c, self.orthography.read(f).letters))
            .collect();
        let mut inflections = self.inflections(&word.letters, &key, known);
        if inflections.is_empty() {
            let none = Known {
                forms: &[],
                ..*known
            };
            inflections = self.inflections(&word.letters, &key, &none);
        }
        // A word no paradigm's lemma fits (Greek `ἵ`, one letter): the
        // commonest paradigm's forms, the word its one stem, unsure. Every
        // word has every form.
        if inflections.is_empty()
            && let Some((value, paradigm)) = self
                .paradigms
                .iter()
                .enumerate()
                .max_by_key(|(i, p)| (p.words, std::cmp::Reverse(*i)))
                .and_then(|(i, p)| Some((P::from_index(i)?, p)))
        {
            inflections.push(Inflection {
                reading: Reading {
                    value,
                    probability: 1.0,
                    status: Status::Unsettled,
                    rule: None,
                },
                paradigm,
                stems: vec![word.letters.as_str()],
            });
        }
        // The likeliest few readings, each with its accent classes and its
        // forms, cell by cell, built once.
        let mut readings: Vec<_> = inflections
            .into_iter()
            .take(READINGS)
            .map(|inf| {
                let classes = self.accent_classes(&word, &inf, cells);
                let mut built = self.built(inf.paradigm);
                built.fixed = self.fixed(inf.paradigm, &inf.stems, &given, &word.letters);
                (inf, classes, built)
            })
            .collect();
        // A word written in parts: as one word, part by part, or its first
        // part with the rest kept.
        let parted = JOINERS
            .iter()
            .filter_map(|&j| Some((lemma.find(j)?, j)))
            .min()
            .and_then(|(_, j)| Some((lemma.split_once(j)?, j)));
        let (whole, parts) = match (&self.compound, parted) {
            (Some(compound), Some(((first, rest), joiner))) => {
                let first_key = self.orthography.key(&self.orthography.read(first).letters);
                let key = format!("{first_key}{joiner}");
                let readings = compound.read(&key, self.smoothing, |_| true);
                let p = |c: Compound| {
                    readings
                        .iter()
                        .find(|r| r.value == c)
                        .map_or(0.0, |r| r.probability)
                };
                let (each, kept) = (p(Compound::Each), p(Compound::First));
                // The forms given, of the first part: without the rest.
                let (tail, head) = (format!("{joiner}{rest}"), format!("{rest}{joiner}"));
                let given: Vec<(usize, &str)> = known
                    .forms
                    .iter()
                    .filter_map(|&(c, f)| {
                        Some((c, f.strip_suffix(&tail).or_else(|| f.strip_prefix(&head))?))
                    })
                    .collect();
                let base = Known {
                    forms: &given,
                    ..*known
                };
                let firsts = (each + kept > 0.0).then(|| {
                    let known = if kept > 0.0 { &base } else { &Known::default() };
                    top(self.unsharpened(first, cells, known))
                });
                let rests = (each > 0.0).then(|| top(self.unsharpened(rest, cells, known)));
                (
                    p(Compound::Last),
                    firsts.map(|firsts| (firsts, rests, joiner, rest, each, kept)),
                )
            }
            _ => (1.0, None),
        };
        // A word that begins with a prefix: the rest of it, the prefix where
        // the cell puts it; or a word of its own.
        let behind = self.prefixes.as_ref().and_then(|prefixes| {
            let (letters, separable, inseparable) = prefixes.split(&key)?;
            if separable + inseparable <= 0.0 {
                return None;
            }
            let at = word
                .letters
                .char_indices()
                .nth(letters)
                .map_or(word.letters.len(), |(i, _)| i);
            let (prefix, rest) = word.letters.split_at(at);
            // The forms given, kept behind the prefix: the rest's.
            let key_of = self.orthography.key(prefix);
            let given: Vec<(usize, String)> = known
                .forms
                .iter()
                .filter_map(|&(c, f)| {
                    let letters = self.orthography.read(f).letters;
                    Some((c, prefixes.unjoin(&key_of, prefix, &letters)?))
                })
                .collect();
            let given: Vec<(usize, &str)> = given.iter().map(|(c, f)| (*c, f.as_str())).collect();
            let base = Known {
                forms: &given,
                ..*known
            };
            let rests = top(self.unsharpened(rest, cells, &base));
            Some((prefixes, prefix, rest, separable, inseparable, rests))
        });
        let whole = whole * behind.as_ref().map_or(1.0, |b| 1.0 - b.3 - b.4);
        let line = self.irregular.line(&key);
        let mut lists = Vec::with_capacity(cells.len());
        // Where a word's line lists a form, or a stem the cell is built on:
        // its letters.
        let have = |c: usize| -> Option<String> {
            let line = line?;
            match listed_in(line, u16::try_from(c).ok()?) {
                Some(form) => Some(self.orthography.read(form).letters),
                None => self.stem_built(line, c)?.0.into_iter().next().map(|f| f.0),
            }
        };
        // The stress of a listed word's letters `letters` of `cell` (its
        // ending starting at `ending`), by its likeliest reading.
        let lead = line
            .and(readings.first())
            .map(|(inf, classes, _)| (inf.reading.value, classes.clone()));
        let stressed = |cell: usize, letters: &str, ending: usize| match &lead {
            Some((value, classes)) => self.stresses(*value, classes, cell, letters, ending),
            None => vec![(None, 1.0)],
        };
        for (i, &cell) in cells.iter().enumerate() {
            let listed_forms = line
                .zip(u16::try_from(cell).ok())
                .and_then(|(l, c)| spellings_in(l, c));
            if let Some(forms) = listed_forms {
                let mut spellings = pattern::variants(forms).map(|f| crate::recase(f, casing));
                let form = spellings.next().unwrap_or_default();
                lists.push(vec![Candidate {
                    also: spellings.collect(),
                    ..listed(form, 1.0)
                }]);
                continue;
            }
            // A cell of a slot whose stem the word lists: built on it, with
            // the slot's ending table.
            if let Some((forms, table, stem)) = line.and_then(|l| self.stem_built(l, cell))
                && let Some(((first, ending), also)) = forms.split_first()
            {
                let stresses = stressed(cell, first, *ending);
                lists.push(
                    stresses
                        .into_iter()
                        .map(|(stress, p)| {
                            let write = |f: &str| self.orthography.write(f, stress, casing);
                            Candidate {
                                form: write(first),
                                also: also.iter().map(|(f, _)| write(f)).collect(),
                                paradigm: table,
                                stem: Some(stem),
                                ..listed(String::new(), p)
                            }
                        })
                        .collect(),
                );
                continue;
            }
            // A form of a word whose forms are listed, but not this one: told
            // from the listed form of the cell it comes from.
            if let Some((letters, ending)) = line
                .is_some()
                .then(|| self.told_from_listed(&have, cell, self.relations.len()))
                .flatten()
            {
                lists.push(
                    stressed(cell, &letters, ending)
                        .into_iter()
                        .map(|(stress, p)| {
                            listed(self.orthography.write(&letters, stress, casing), p)
                        })
                        .collect(),
                );
                continue;
            }
            let mut out: Vec<Candidate<'_>> = Vec::new();
            if let Some((prefixes, prefix, rest, separable, inseparable, rests)) = &behind {
                let placement = prefixes
                    .placement
                    .get(cell)
                    .copied()
                    .unwrap_or(Placement::Before);
                let drop = prefixes.drops.get(cell).map_or("", |d| d.as_ref());
                for c in rests.get(i).cloned().unwrap_or_default() {
                    let separated = match placement {
                        Placement::Before => format!("{prefix}{}", c.form),
                        Placement::After => format!("{} {prefix}", c.form),
                    };
                    let kept = self.kept(
                        prefixes,
                        prefix,
                        rest,
                        c.form.strip_prefix(drop).unwrap_or(&c.form),
                        cell,
                    );
                    for (form, p) in [(separated, *separable), (kept, *inseparable)] {
                        if p <= 0.0 {
                            continue;
                        }
                        let candidate = Candidate {
                            form,
                            probability: p * c.probability,
                            also: Vec::new(),
                            ..c
                        };
                        add(&mut out, candidate);
                    }
                }
            }
            if let Some((firsts, rests, joiner, rest, each, kept)) = &parts {
                for a in firsts.get(i).into_iter().flatten() {
                    if *kept > 0.0 {
                        let with = |f: &str| match self.kept.get(cell) {
                            Some(Placement::Before) => format!("{rest}{joiner}{f}"),
                            _ => format!("{f}{joiner}{rest}"),
                        };
                        add(
                            &mut out,
                            Candidate {
                                form: with(&a.form),
                                also: a.also.iter().map(|f| with(f)).collect(),
                                probability: kept * a.probability,
                                ..a.clone()
                            },
                        );
                    }
                    let rests = rests.as_ref().and_then(|r| r.get(i)).into_iter().flatten();
                    for b in rests {
                        out.push(Candidate {
                            form: format!("{}{joiner}{}", a.form, b.form),
                            also: Vec::new(),
                            probability: each * a.probability * b.probability,
                            status: b.status,
                            paradigm: b.paradigm,
                            evidence: b.evidence,
                            stem: b.stem,
                        });
                    }
                }
            }
            // A cell few or no words of the dictionary have is told by analogy.
            let unattested = self.relation(cell).is_some_and(|r| r.analogy.is_some());
            for (inf, classes, built) in &mut readings {
                let evidence = inf.reading.rule.map(|r| Evidence {
                    ending: r.ending.as_ref(),
                    count: r.count(inf.reading.value),
                    total: r.total(),
                });
                let forms = self.build(&inf.stems, &key, known.inherent, cell, built);
                for (forms, p_cell) in forms {
                    let Some(((first, ending), also)) = forms.split_first() else {
                        continue;
                    };
                    let p = spelled * whole * p_cell * inf.reading.probability;
                    for (stress, p_stress) in
                        self.stresses(inf.reading.value, classes, cell, first, *ending)
                    {
                        let write = |f: &str| self.orthography.write(f, stress, casing);
                        let candidate = Candidate {
                            form: write(first),
                            also: also.iter().map(|(f, _)| write(f)).collect(),
                            probability: p * p_stress,
                            status: if unattested {
                                Status::Unattested
                            } else {
                                inf.reading.status
                            },
                            paradigm: inf.paradigm.name.as_ref(),
                            evidence,
                            stem: None,
                        };
                        add(&mut out, candidate);
                    }
                }
            }
            out.retain(|c| c.probability > 0.0);
            lists.push(out);
        }
        // The word spelled the next likeliest ways, as likely as each is.
        for (letters, p) in otherwise {
            let typed = self.orthography.write(&letters, word.stress, casing);
            let full = Known {
                full: true,
                ..*known
            };
            for (out, more) in lists.iter_mut().zip(self.unsharpened(&typed, cells, &full)) {
                // A listed form, or one built on a stem the word lists, is the
                // answer however the word is spelled.
                if out.iter().any(|c| {
                    c.status == Status::Listed && (c.paradigm.is_empty() || c.stem.is_some())
                }) {
                    continue;
                }
                for mut c in more {
                    c.probability *= p;
                    add(out, c);
                }
            }
        }
        lists
    }

    /// The form `form` (written) of `cell` of the word `rest` with `prefix`
    /// kept in front, as they meet. Its accent stays on the syllable it is
    /// on, counted from the end; in a cell whose accent recedes, one as far
    /// back as the law allows on the rest goes as far back as it allows on
    /// the whole word, but not before the prefix's last syllable, nor before
    /// an augment (`ἀπόλυε`, `ἀπῆλθον`; Smyth §§ 426, 450). How long the
    /// last syllable counts is the cell's (the optative's `-οι` is long,
    /// Smyth §169: `προφέροι`); a cell whose accent doesn't recede keeps it
    /// where it is (the infinitives and participles, Smyth §425:
    /// `προλῦσαι`).
    fn kept(
        &self,
        prefixes: &Prefixes<'_>,
        prefix: &str,
        rest: &str,
        form: &str,
        cell: usize,
    ) -> String {
        let o = &self.orthography;
        let written = o.read(form);
        let (letters, rest_at) = prefixes.join(&o.key(prefix), prefix, &written.letters);
        let from_end = written.stress.map(|s| {
            o.vowel_count(&written.letters)
                .saturating_sub(usize::from(s.vowel))
        });
        let mut stress = from_end.and_then(|n| {
            let vowel = o.vowel_count(&letters).checked_sub(n)?;
            Some(crate::Stress {
                vowel: u8::try_from(vowel).ok()?,
                ..written.stress?
            })
        });
        // The cell's accent: whether it recedes (as the commonest pattern
        // places it), and whether its final `-αι`, `-οι` counts as long.
        let accent = self.accent.as_ref();
        let recedes = accent
            .and_then(|a| a.usual(cell))
            .is_none_or(|place| place == crate::Place::Recessive);
        let long = accent.is_some_and(|a| {
            a.long_finals
                .binary_search(&u16::try_from(cell).unwrap_or(u16::MAX))
                .is_ok()
        });
        if let (Some(p), Some(s), true) = (&o.prosody, written.stress, recedes) {
            let recessive = |letters: &str, long: bool, wall: usize| {
                let syllables = p.syllables(o, letters, long);
                p.accent(&syllables, wall, p.acute)
            };
            if recessive(&written.letters, long, 0) == Some(s) {
                // The augment: the form begins otherwise than the word.
                let augmented = written.letters.chars().next() != rest.chars().next();
                let syllables = p.syllables(o, &letters, long);
                let boundary = if augmented {
                    rest_at
                } else {
                    prefix.len().min(rest_at)
                };
                let wall = syllables
                    .iter()
                    .rposition(|syl| syl.at < boundary)
                    .unwrap_or(0);
                stress = p.accent(&syllables, wall, p.acute);
            }
        }
        o.write(&letters, stress, written.casing)
    }

    /// The letters of `cell` for a word whose listed forms (or forms built on
    /// its listed stems) `listed` gives by cell, as letters, told from that
    /// of the cell it comes from (or one told so in turn, at most `steps`
    /// away), and where the change starts.
    fn told_from_listed(
        &self,
        listed: &impl Fn(usize) -> Option<String>,
        cell: usize,
        steps: usize,
    ) -> Option<(String, usize)> {
        let (relation, source) = self.listed_source(listed, cell, steps)?;
        relation.told(&source, self.smoothing, self.laws.as_ref())
    }

    /// The relation (its changes' owner: see [`Grammar::changing`]) that
    /// tells `cell` of a word whose forms are listed (`listed`), and the
    /// letters it reads: the listed form of the cell it comes from, or that
    /// cell's told in turn.
    fn listed_source(
        &self,
        listed: &impl Fn(usize) -> Option<String>,
        cell: usize,
        steps: usize,
    ) -> Option<(&Relation<'a>, String)> {
        let relation = self.relation(cell).filter(|r| r.listed)?;
        let from = relation.source()?;
        let source = match listed(from) {
            Some(letters) => letters,
            None => {
                self.told_from_listed(listed, from, steps.checked_sub(1)?)?
                    .0
            }
        };
        Some((self.changing(relation), source))
    }

    /// Of a word the grammar lists (looked up as `key`), for each cell no
    /// form or stem of its line gives: the cell whose relation's changes
    /// tell it, and the letters that relation reads. What a listed word can
    /// ask of the relations.
    pub fn listed_sources(&self, key: &str) -> Vec<(usize, String)> {
        let Some(line) = self.irregular.line(key) else {
            return Vec::new();
        };
        let have = |c: usize| -> Option<String> {
            match listed_in(line, u16::try_from(c).ok()?) {
                Some(form) => Some(self.orthography.read(form).letters),
                None => self.stem_built(line, c)?.0.into_iter().next().map(|f| f.0),
            }
        };
        (0..self.cells)
            .filter(|&c| have(c).is_none())
            .filter_map(|c| {
                let (relation, source) = self.listed_source(&have, c, self.relations.len())?;
                Some((usize::from(relation.cell), source))
            })
            .collect()
    }

    /// The forms (letters, each with where its ending starts) of `cell` for
    /// a word whose irregular `line` lists the stem of the cell's slot,
    /// built with the stem's ending table; the table's name, and the stem.
    fn stem_built<'l>(&'l self, line: &'l str, cell: usize) -> Option<(Forms, &'l str, &'l str)> {
        let (s, slot) = self
            .slots
            .iter()
            .enumerate()
            .find(|(_, s)| s.position(cell).is_some())?;
        let position = slot.position(cell)?;
        let (stem, name) = stem_in(line, &slot.name)?;
        let table = self
            .tables
            .iter()
            .find(|t| usize::from(t.slot) == s && t.name == name)?;
        let forms = table.build(position, stem, self.laws.as_ref());
        (!forms.is_empty()).then_some((forms, table.name.as_ref(), stem))
    }

    /// Its stem slots and ending tables, as paradigms state slots with.
    pub fn space(&self) -> crate::Space<'_, 'a> {
        crate::Space {
            slots: &self.slots,
            tables: &self.tables,
        }
    }

    /// A reading of `paradigm`, nothing built yet.
    fn built<'g>(&'g self, paradigm: &'g Paradigm<'a>) -> Built<'g, 'a> {
        Built {
            tables: crate::paradigm::Tables::new(paradigm, &self.paradigms, self.space()),
            cells: std::collections::HashMap::new(),
            fixed: Vec::new(),
        }
    }

    /// The slot `cell` is in, and its place among the slot's cells.
    fn slot_of(&self, cell: usize) -> Option<(usize, usize)> {
        self.slots
            .iter()
            .enumerate()
            .find_map(|(s, slot)| Some((s, slot.position(cell)?)))
    }

    /// The stems the forms the caller gives (`given`, as letters) fix, per
    /// slot, for a word `lemma` (letters) read as `paradigm`: `(slot, ending
    /// table, stem)`. A given form's stem is what the paradigm's own table
    /// for the slot (see [`crate::factor`]) leaves of it, where the table
    /// builds it; the stem's first letter may be the lemma's, where a
    /// template before the stem changed it (an augment). Another paradigm's
    /// table is that paradigm's reading's to use, not this one's; a form
    /// the paradigm builds itself fixes nothing.
    fn fixed(
        &self,
        paradigm: &Paradigm<'a>,
        stems: &[&str],
        given: &[(usize, String)],
        lemma: &str,
    ) -> Vec<(usize, String, String)> {
        let mut out: Vec<(usize, String, String)> = Vec::new();
        for (cell, form) in given {
            // A word written in parts has no one stem; nor has a form written
            // in two words where the paradigm writes one (a separated prefix:
            // German `richtete zugrunde` fixes no stem `zugrunderichtete` is
            // built on).
            let phrase = paradigm
                .pattern_in(&self.paradigms, self.space(), *cell)
                .is_some_and(|p| p.contains(' '));
            if lemma.contains('-') || form.contains('-') || (form.contains(' ') && !phrase) {
                continue;
            }
            // A form the paradigm builds fixes nothing it doesn't.
            let built = paradigm.built_in(
                &self.paradigms,
                self.space(),
                self.laws.as_ref(),
                *cell,
                stems,
            );
            if built.iter().any(|(b, _)| b == form) {
                continue;
            }
            let Some((s, position)) = self.slot_of(*cell) else {
                continue;
            };
            if out.iter().any(|f| f.0 == s) {
                continue;
            }
            let Some(slot) = self.slots.get(s) else {
                continue;
            };
            let owned: Vec<Option<Cow<'_, str>>> = slot
                .cells
                .iter()
                .map(|&c| paradigm.pattern_in(&self.paradigms, self.space(), usize::from(c)))
                .collect();
            let patterns: Vec<Option<&str>> = owned.iter().map(|p| p.as_deref()).collect();
            let found = crate::stems::factor(&patterns).and_then(|(_, table)| {
                let entry = |at: usize| table.split('\t').nth(at).filter(|p| !p.is_empty());
                // A cell the paradigm derives: its source's entry, the form
                // taken back through the relation (the Latin supine `iussum`,
                // from the infinitive `iussum esse`), where the paradigm
                // doesn't build that source form itself (Finnish `madon`
                // from `madot`, which `mato` builds: its stem alternates).
                let stem = match entry(position) {
                    Some(p) => crate::stems::stem_of(p, form, lemma, self.laws.as_ref())?,
                    None => {
                        let relation = self.changing(self.relation(*cell)?);
                        let source = self.relation(*cell)?.source()?;
                        let p = entry(slot.position(source)?)?;
                        let backs = relation.sources(form, self.smoothing, self.laws.as_ref());
                        let built = paradigm.built_in(
                            &self.paradigms,
                            self.space(),
                            self.laws.as_ref(),
                            source,
                            stems,
                        );
                        if backs.iter().any(|b| built.iter().any(|(f, _)| f == b)) {
                            return None;
                        }
                        backs.iter().find_map(|back| {
                            crate::stems::stem_of(p, back, lemma, self.laws.as_ref())
                        })?
                    }
                };
                Some((table, stem))
            });
            if let Some((table, stem)) = found {
                out.push((s, table, stem));
            }
        }
        out
    }

    /// The letters of `cell` the word looked up as `key` builds on the stem
    /// it lists for the cell's slot, if it lists one (the usual form).
    pub fn stem_form(&self, key: &str, cell: usize) -> Option<String> {
        let line = self.irregular.line(key)?;
        let (forms, _, _) = self.stem_built(line, cell)?;
        forms.into_iter().next().map(|f| f.0)
    }

    /// The relation whose changes `relation` makes: its own, or, for one by
    /// analogy with no changes of its own, those of the relation it is by
    /// analogy with.
    pub fn changing<'r>(&'r self, relation: &'r Relation<'a>) -> &'r Relation<'a> {
        match relation.analogy {
            Some((a, b)) if a != b && relation.change.is_none() => self
                .relation(usize::from(b))
                .filter(|r| r.from.as_ref() == [a])
                .unwrap_or(relation),
            _ => relation,
        }
    }

    /// The relation that gives `cell`.
    pub fn relation(&self, cell: usize) -> Option<&Relation<'a>> {
        let at = self
            .relations
            .binary_search_by_key(&cell, |r| usize::from(r.cell))
            .ok()?;
        self.relations.get(at)
    }

    /// What an inflection builds in `cell`, by how likely: its paradigm's
    /// forms, the usual first, each with where its ending starts; for a cell
    /// the paradigm derives, those of the cell it comes from (by the word's
    /// feature, given or chosen), changed. `built`: the cells built so far.
    fn build(
        &self,
        stems: &[&str],
        key: &str,
        inherent: &[(u8, u8)],
        cell: usize,
        built: &mut Built<'_, 'a>,
    ) -> Vec<(Vec<(String, usize)>, f32)> {
        if let Some(forms) = built.cells.get(&cell) {
            return forms.clone();
        }
        // A cell on the way to itself (none, as relations are learned) has
        // no form.
        built.cells.insert(cell, Vec::new());
        let mut out = Vec::new();
        // A cell of a slot a given form fixes the stem of: built on that
        // stem; else as the paradigm builds it.
        let on_given = (!built.fixed.is_empty())
            .then(|| built.tables.slot(cell))
            .flatten()
            .and_then(|(s, position)| {
                let (_, table, stem) = built.fixed.iter().find(|f| f.0 == s)?;
                let p = table.split('\t').nth(position).filter(|p| !p.is_empty())?;
                Some(crate::stems::build(p, stem, self.laws.as_ref()))
            });
        let forms = on_given.unwrap_or_else(|| built.tables.built(self.laws.as_ref(), cell, stems));
        if !forms.is_empty() {
            out.push((forms, 1.0));
        } else if let Some(relation) = self.relation(cell) {
            let sources: Vec<(u16, f32)> = match (relation.from.as_ref(), &self.feature) {
                ([from], _) => vec![(*from, 1.0)],
                (_, None) => Vec::new(),
                (from, Some((f, choice))) => match inherent.iter().find(|(g, _)| g == f) {
                    Some(&(_, v)) => from
                        .get(usize::from(v))
                        .map(|&c| (c, 1.0))
                        .into_iter()
                        .collect(),
                    None => {
                        let read: Vec<(u16, f32)> = choice
                            .read(key, self.smoothing, |_| true)
                            .into_iter()
                            .filter_map(|r| Some((*from.get(r.value.index())?, r.probability)))
                            .collect();
                        // A choice that knows nothing of the word: every
                        // cell alike, never no form.
                        if read.is_empty() {
                            #[allow(clippy::cast_precision_loss)]
                            let each = 1.0 / from.len().max(1) as f32;
                            from.iter().map(|&c| (c, each)).collect()
                        } else {
                            read
                        }
                    }
                },
            };
            let relation = self.changing(relation);
            for (from, p) in sources {
                let from = usize::from(from);
                for (forms, q) in self.build(stems, key, inherent, from, built) {
                    if relation.change.is_none() {
                        out.push((forms, p * q));
                        continue;
                    }
                    let Some((usual, ending)) = forms.first() else {
                        continue;
                    };
                    for (changed, at, r) in
                        relation.apply(usual, self.smoothing, self.laws.as_ref())
                    {
                        out.push((vec![(changed, (*ending).min(at))], p * q * r));
                    }
                }
            }
            // The likeliest few: along a line of cells each told from the
            // last, alternatives would multiply.
            out.sort_by(|a, b| b.1.total_cmp(&a.1));
            out.truncate(KEEP);
        }
        built.cells.insert(cell, out.clone());
        out
    }

    /// The letters of each cell's forms (the usual first) `lemma` (typed in
    /// full) would have if it took paradigm `paradigm`, accent aside; none
    /// where the paradigm doesn't fit it. For measuring how well paradigms
    /// build words apart from how well the grammar chooses one.
    pub fn built_as(&self, lemma: &str, paradigm: usize) -> Vec<Vec<String>> {
        let letters = self.orthography.read(lemma).letters;
        let key = self.orthography.key(&letters);
        let Some((p, stems)) = self
            .paradigms
            .get(paradigm)
            .and_then(|p| Some((p, p.stems(&self.orthography, &letters)?)))
        else {
            return vec![Vec::new(); self.cells];
        };
        let mut built = self.built(p);
        (0..self.cells)
            .map(|cell| {
                let forms = self.build(&stems, &key, &[], cell, &mut built);
                forms
                    .first()
                    .map(|(forms, _)| forms.iter().map(|(f, _)| f.clone()).collect())
                    .unwrap_or_default()
            })
            .collect()
    }

    /// The paradigms `lemma` can follow, likeliest first, each with how
    /// sure and why, given what the caller knows.
    pub fn readings(&self, lemma: &str, known: &Known<'_>) -> Vec<Reading<'_, 'a, P>> {
        let mut letters = self.orthography.read(lemma).letters;
        let key = self.orthography.key(&letters);
        if let Some(listed) = self.spelling(&letters, &key) {
            letters = listed.to_string();
        }
        let mut readings: Vec<Reading<'_, 'a, P>> = self
            .inflections(&letters, &key, known)
            .into_iter()
            .map(|i| i.reading)
            .collect();
        sharpen(
            readings.iter_mut().map(|r| &mut r.probability),
            self.sharpness,
        );
        readings
    }

    /// The dictionary's spelling of a word typed as `letters` without some
    /// of the marks it has (`fox` for `fōx`), if it lists one that agrees
    /// with the marks typed.
    pub fn spelling(&self, letters: &str, key: &str) -> Option<&str> {
        let at = self
            .spellings
            .binary_search_by(|(k, _)| k.as_ref().cmp(key))
            .ok()?;
        let listed = self.spellings.get(at).map(|(_, s)| s.as_ref())?;
        let agrees = listed.chars().count() == letters.chars().count()
            && letters
                .chars()
                .zip(listed.chars())
                .all(|(t, l)| t == l || self.orthography.fold(t) == t);
        agrees.then_some(listed)
    }

    /// Whether the grammar lists the word looked up as `key`: its forms, or
    /// its paradigm.
    pub fn lists(&self, key: &str) -> bool {
        self.irregular.has(key)
            || self
                .paradigm
                .listed(&format!("{WORD_END}{key}"))
                .next()
                .is_some()
    }

    /// The word a word the grammar doesn't list ends in (its head: a listed
    /// one whose compounds inflect as it, or one named, see [`Heads`]), the
    /// longest,
    /// behind at least [`FRONT_LETTERS`] letters: the letters before it and
    /// its letters (`letters` the word's, `key` how it is looked up). None
    /// for a word a prefix splits (see [`Prefixes`]): the prefix decides
    /// where its letters go.
    pub fn head<'w>(&self, key: &str, letters: &'w str) -> Option<(&'w str, &'w str)> {
        if self.lists(key) {
            return None;
        }
        self.head_unlisted(key, letters)
    }

    /// The head [`Grammar::head`] gives the word `key` once the grammar
    /// doesn't list it (the trainer asks before it stops listing a word).
    pub fn head_unlisted<'w>(&self, key: &str, letters: &'w str) -> Option<(&'w str, &'w str)> {
        if self.heads.is_empty()
            || self
                .prefixes
                .as_ref()
                .and_then(|p| p.split(key))
                .is_some_and(|(_, separable, inseparable)| separable + inseparable > 0.0)
        {
            return None;
        }
        let n = key.chars().count();
        let at = (FRONT_LETTERS..n).find(|&i| {
            key.char_indices()
                .nth(i)
                .and_then(|(b, _)| key.get(b..))
                .is_some_and(|head| {
                    self.heads.names(head) || (self.heads.inflects(head) && self.lists(head))
                })
        })?;
        let (b, _) = letters.char_indices().nth(at)?;
        Some(letters.split_at(b))
    }

    /// The listed form of `cell` for the word looked up as `key`, if any.
    pub fn irregular_form(&self, key: &str, cell: usize) -> Option<&str> {
        self.irregular.get(key, u16::try_from(cell).ok()?)
    }

    /// The likeliest form of `cell` for `lemma` (the lemma itself when the
    /// grammar has no reading of it).
    pub fn best(&self, lemma: &str, cell: usize, known: &Known<'_>) -> String {
        self.candidates(lemma, cell, known)
            .into_iter()
            .next()
            .map_or_else(|| lemma.to_string(), |c| c.form)
    }

    /// The paradigms the word `letters` (looked up as `key`) can follow,
    /// likeliest first, agreeing with the forms `known` gives; a paradigm
    /// whose words have other inherent feature values than it gives is less
    /// likely (by [`OTHER_VALUES`]).
    fn inflections<'g, 'w>(
        &'g self,
        letters: &'w str,
        key: &str,
        known: &Known<'_>,
    ) -> Vec<Inflection<'g, 'a, 'w, P>> {
        let inherent = known.inherent;
        let given: Vec<(usize, String)> = known
            .forms
            .iter()
            .map(|&(c, f)| (c, self.orthography.read(f).letters))
            .collect();
        // A form the word's own line gives (listed, or built on a stem it
        // lists) agrees with any paradigm: German `geschehen` lists
        // `geschieh-`, which its paradigm alone doesn't build.
        let line = self.irregular.line(key);
        let lines = |cell: usize, form: &str| {
            line.is_some_and(|l| {
                let listed = u16::try_from(cell).ok().and_then(|c| listed_in(l, c));
                listed.is_some_and(|f| self.orthography.read(f).letters == form)
                    || self
                        .stem_built(l, cell)
                        .is_some_and(|(forms, ..)| forms.iter().any(|(f, _)| f == form))
            })
        };
        let agrees = |p: &Paradigm<'a>| {
            let Some(stems) = p.stems(&self.orthography, letters) else {
                return false;
            };
            given.iter().all(|(cell, form)| {
                lines(*cell, form)
                    || self
                        .build(&stems, key, inherent, *cell, &mut self.built(p))
                        .iter()
                        .any(|(forms, _)| forms.iter().any(|(built, _)| built == form))
            })
        };
        let fits = |p: P| self.paradigms.get(p.index()).is_some_and(agrees);
        // Rules read the word from its end to its start (`#`).
        let mut readings = self
            .paradigm
            .read(&format!("{WORD_END}{key}"), self.smoothing, fits);
        if readings.is_empty() {
            // No rule knows a paradigm the word fits: every paradigm it does,
            // by how many words follow it.
            let fitting: Vec<(P, u32)> = self
                .paradigms
                .iter()
                .enumerate()
                .filter(|(_, p)| agrees(p))
                .filter_map(|(i, p)| Some((P::from_index(i)?, p.words.max(1))))
                .collect();
            let total: f64 = fitting.iter().map(|&(_, n)| f64::from(n)).sum();
            readings = fitting
                .into_iter()
                .map(|(value, n)| Reading {
                    value,
                    #[allow(clippy::cast_possible_truncation)]
                    probability: (f64::from(n) / total) as f32,
                    status: Status::Unsettled,
                    rule: None,
                })
                .collect();
            readings.sort_by(|a, b| b.probability.total_cmp(&a.probability));
        }
        for r in &mut readings {
            if self
                .paradigms
                .get(r.value.index())
                .is_some_and(|p| !p.has(inherent))
            {
                r.probability *= OTHER_VALUES;
            }
        }
        readings.sort_by(|a, b| b.probability.total_cmp(&a.probability));
        readings
            .into_iter()
            .filter_map(|reading| {
                let paradigm = self.paradigms.get(reading.value.index())?;
                let stems = paradigm.stems(&self.orthography, letters)?;
                Some(Inflection {
                    reading,
                    paradigm,
                    stems,
                })
            })
            .collect()
    }

    /// The accent classes of the word `word` read as `inf`, by how likely:
    /// those whose pattern puts the lemma's accent where the caller wrote it.
    fn accent_classes(
        &self,
        word: &Written,
        inf: &Inflection<'_, 'a, '_, P>,
        cells: &[usize],
    ) -> Vec<Class> {
        let Some(accent) = &self.accent else {
            return Vec::new();
        };
        let letters = word.letters.as_str();
        let o = &self.orthography;
        // Where the lemma's own ending starts: an accent the caller wrote
        // says which patterns the word can follow.
        let lemma_ending = pattern::ending_start(&inf.paradigm.lemma, &inf.stems);
        let lemma_finals = self
            .lemma_cell
            .map(|c| accent.finals(inf.reading.value.index(), c))
            .unwrap_or_default();
        let given = word.stress;
        let given_at = given.map(|g| stress::position(o, letters, g));
        let fits = |a: A| {
            accent.class_of(a).is_some_and(|class| {
                let place = self.lemma_cell.and_then(|c| accent.place(class.pattern, c));
                let (Some(g), Some(place)) = (given, place) else {
                    return true;
                };
                let at = given_at.unwrap_or(0);
                stress::place(o, letters, lemma_ending, lemma_finals, place, at) == Some(g)
            })
        };
        let mut classes = accent.class.read(&o.key(letters), self.smoothing, fits);
        classes.truncate(KEEP * KEEP);
        let positions = stress::positions(o, letters);
        classes
            .into_iter()
            .filter_map(|r| {
                let class = accent.class_of(r.value)?;
                // Each cell's place: all at once for a table, else the
                // cells asked for and the lemma's.
                let places = if cells.len() * 4 > self.cells {
                    accent.places(class.pattern)
                } else {
                    let mut places = vec![None; self.cells];
                    for &c in cells.iter().chain(&self.lemma_cell) {
                        if let Some(slot) = places.get_mut(c) {
                            *slot = accent.place(class.pattern, c);
                        }
                    }
                    places
                };
                let on_stem = self
                    .lemma_cell
                    .and_then(|c| places.get(c).copied().flatten())
                    == Some(Place::Stem);
                // The stem's accented syllable: the one written, when the
                // lemma's accent is on its stem; else the class's, from the
                // lemma's end.
                let stem = match given_at {
                    Some(at) if on_stem => Some(at),
                    _ => class
                        .vowel
                        .and_then(|v| positions.checked_sub(1)?.checked_sub(usize::from(v))),
                };
                Some((r.probability, places, stem.unwrap_or(0)))
            })
            .collect()
    }

    /// Where the form `form` of `cell` (its ending starting at byte
    /// `ending`) is stressed in a paradigm's accent `classes`, with
    /// probabilities. One unstressed reading when the language writes no
    /// stress.
    fn stresses(
        &self,
        paradigm: P,
        classes: &[Class],
        cell: usize,
        form: &str,
        ending: usize,
    ) -> Vec<(Option<crate::Stress>, f32)> {
        let Some(accent) = &self.accent else {
            return vec![(None, 1.0)];
        };
        let finals = accent.finals(paradigm.index(), cell);
        let mut out: Vec<(Option<crate::Stress>, f32)> = Vec::new();
        for (p, places, stem) in classes {
            let Some(place) = places.get(cell).copied().flatten() else {
                continue;
            };
            let s = stress::place(&self.orthography, form, ending, finals, place, *stem);
            match out.iter_mut().find(|e| e.0 == s) {
                Some(e) => e.1 += p,
                None => out.push((s, *p)),
            }
        }
        // A cell none of the word's classes places: where most words have
        // it, else as far back as the law allows. A form that needs an accent
        // never goes without one (Greek `ὀστοῦν`, not `ὀστουν`).
        if out.is_empty() {
            let place = accent.usual(cell).unwrap_or(crate::Place::Recessive);
            let stem = classes.first().map_or(0, |c| c.2);
            let s = stress::place(&self.orthography, form, ending, finals, place, stem);
            out.push((s, 1.0));
        }
        out
    }
}

/// The spelling of a word typed as `letters` (looked up as `key`) whose
/// unmarked letters `marks` marks as it says (see [`Grammar::marks`]), and how
/// likely it is.
pub fn restore(
    orthography: &Orthography<'_>,
    marks: &Marks<'_>,
    smoothing: f32,
    letters: &str,
    key: &str,
) -> (String, f32) {
    restorations(orthography, marks, smoothing, letters, key).0
}

/// How likely another spelling must be, against the likeliest, to be read
/// too (lower than a tenth: Latin top-3 +1 to +1.2 points).
const SECOND_SPELLING: f32 = 0.03;

/// Spellings kept while reading a word's letters, and read at most (the
/// likeliest and two more).
const SPELLINGS: usize = 3;

/// The likeliest spelling (see [`restore`]), and the next likeliest ones,
/// each at least [`SECOND_SPELLING`] as likely: the [`SPELLINGS`] likeliest
/// over every letter's marks at once (a beam), so that a word unsure of two
/// letters is read with either written otherwise.
fn restorations(
    o: &Orthography<'_>,
    marks: &Marks<'_>,
    smoothing: f32,
    letters: &str,
    key: &str,
) -> ((String, f32), Vec<(String, f32)>) {
    let mut beam: Vec<(String, f32)> = vec![(String::with_capacity(letters.len()), 1.0)];
    let mut before = WORD_END;
    for (n, ((i, k), c)) in key.char_indices().zip(letters.chars()).enumerate() {
        let mut readings: Vec<(char, f32)> = if c == k && o.folded.iter().any(|e| e.1 == c) {
            let rest = key.get(i..).unwrap_or("");
            let around = format!("{before}{rest}{WORD_END}{}", marks.part);
            (marks
                .rules
                .read(&around, smoothing, |m| o.fold(m) == c)
                .iter())
            .map(|r| (r.value, r.probability))
            .collect()
        } else {
            Vec::new()
        };
        // Its root's letter first, the rules' others after.
        if let Some(v) = marks.root(key, n).filter(|_| !readings.is_empty()) {
            let others: f32 = readings.iter().filter(|r| r.0 != v).map(|r| r.1).sum();
            readings.retain(|r| r.0 != v);
            for r in &mut readings {
                r.1 = if others > 0.0 {
                    r.1 * (1.0 - ROOT_TRUST) / others
                } else {
                    0.0
                };
            }
            let trust = if readings.is_empty() { 1.0 } else { ROOT_TRUST };
            readings.insert(0, (v, trust));
        }
        before = k;
        if readings.is_empty() {
            for (s, _) in &mut beam {
                s.push(c);
            }
            continue;
        }
        let mut next: Vec<(String, f32)> = beam
            .iter()
            .flat_map(|(s, p)| {
                readings.iter().map(move |r| {
                    let mut s = s.clone();
                    s.push(r.0);
                    (s, p * r.1)
                })
            })
            .collect();
        next.sort_by(|a, b| b.1.total_cmp(&a.1));
        next.truncate(SPELLINGS);
        beam = next;
    }
    let mut beam = beam.into_iter();
    let first = beam.next().unwrap_or_default();
    let others = beam
        .filter(|(_, p)| first.1 > 0.0 && p / first.1 >= SECOND_SPELLING)
        .collect();
    (first, others)
}

/// A form the dictionary lists.
fn listed(form: String, probability: f32) -> Candidate<'static> {
    Candidate {
        form,
        also: Vec::new(),
        probability,
        status: Status::Listed,
        paradigm: "",
        evidence: None,
        stem: None,
    }
}

/// `candidate` added to `out`: to the one of the same form, if any.
fn add<'g>(out: &mut Vec<Candidate<'g>>, candidate: Candidate<'g>) {
    match out.iter_mut().find(|c| c.form == candidate.form) {
        Some(c) => c.probability += candidate.probability,
        None => out.push(candidate),
    }
}

/// The likeliest few candidates of each list.
fn top(mut lists: Vec<Vec<Candidate<'_>>>) -> Vec<Vec<Candidate<'_>>> {
    for list in &mut lists {
        list.sort_by(|a, b| b.probability.total_cmp(&a.probability));
        list.truncate(KEEP - 1);
    }
    lists
}

/// Raise probabilities to `power` and make them sum to what they summed to
/// before.
/// How much of its probability an answer of each status keeps, fitted on
/// held-out words: the rest is "none of these" (a cell's answers may sum to
/// less than one, where answers of that kind are often wrong). A listed form
/// keeps all of it.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Trust {
    pub regular: f32,
    pub unsettled: f32,
    pub unattested: f32,
}

impl Trust {
    /// Every answer keeps all its probability.
    pub const FULL: Trust = Trust {
        regular: 1.0,
        unsettled: 1.0,
        unattested: 1.0,
    };

    /// What an answer of `status` keeps.
    pub fn of(&self, status: Status) -> f32 {
        match status {
            Status::Listed => 1.0,
            Status::Regular => self.regular,
            Status::Unsettled => self.unsettled,
            Status::Unattested => self.unattested,
        }
    }
}

pub fn sharpen<'p>(probabilities: impl Iterator<Item = &'p mut f32>, power: f32) {
    if (power - 1.0).abs() < f32::EPSILON {
        return;
    }
    let mut all: Vec<&mut f32> = probabilities.collect();
    let before: f32 = all.iter().map(|p| **p).sum();
    for p in &mut all {
        **p = p.powf(power);
    }
    let after: f32 = all.iter().map(|p| **p).sum();
    if after > 0.0 {
        for p in &mut all {
            **p *= before / after;
        }
    }
}
