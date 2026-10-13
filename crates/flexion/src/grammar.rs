//! A part of speech's grammar, and how it answers: which paradigm a word
//! follows, how it is stressed, and what its forms are, with candidates
//! that say how sure they are and why.

use crate::choice::{Choice, Reading, Status};
use crate::orthography::{Casing, Orthography, Written};
use crate::paradigm::{Paradigm, Relation};
use crate::pattern;
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

/// What joins the parts of a compound written in parts.
const JOINER: char = '-';

/// How a word written in parts (`диван-кровать`) inflects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Compound {
    /// Only its last part, like a word of its own: `интернет-магазин`,
    /// `интернет-магазина`.
    Last,
    /// Each part, as the word it is: `диван-кровать`, `дивана-кровати`.
    Each,
}

impl Label for Compound {
    fn index(self) -> usize {
        self as usize
    }
    fn from_index(index: usize) -> Option<Self> {
        [Compound::Last, Compound::Each].get(index).copied()
    }
}

/// Prefixes a word inflects behind (German `aus-`, `ver-`): a word that
/// begins with one may inflect as the rest of it does. A separable prefix
/// goes where the cell puts it (`räuchre aus`, `ausgeräuchert`); an
/// inseparable one stays in front, and takes off the rest's form what the
/// cell says (`verhindert`, not `vergehindert`).
#[derive(Debug, Clone, PartialEq, Eq)]
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
pub struct Prefix<'a> {
    pub letters: Cow<'a, str>,
    pub separable: u32,
    pub inseparable: u32,
    pub words: u32,
}

/// What a word does with the prefix it begins with.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
/// (see [`Orthography::key`]) then each listed cell's number and form, all
/// tab-separated, cells in order.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Irregular<'a> {
    pub table: Cow<'a, str>,
}

impl Irregular<'_> {
    /// The table of `forms`: `(word, cell, form)`.
    pub fn new(forms: &[(String, usize, String)]) -> Irregular<'static> {
        let mut rows: Vec<(&str, usize, &str)> = forms
            .iter()
            .map(|(w, c, f)| (w.as_str(), *c, f.as_str()))
            .collect();
        rows.sort_unstable();
        rows.dedup_by(|a, b| a.0 == b.0 && a.1 == b.1);
        let mut table = String::new();
        for word in rows.chunk_by(|a, b| a.0 == b.0) {
            let Some((w, _, _)) = word.first() else {
                continue;
            };
            table.push_str(w);
            for (_, c, f) in word {
                table.push_str(&format!("\t{c}\t{f}"));
            }
            table.push('\n');
        }
        Irregular {
            table: Cow::Owned(table),
        }
    }

    /// How many forms the table lists.
    pub fn len(&self) -> usize {
        self.table.lines().map(|l| l.split('\t').count() / 2).sum()
    }

    pub fn is_empty(&self) -> bool {
        self.table.is_empty()
    }

    /// The form of `cell` for the word looked up as `word`, if listed.
    pub fn get(&self, word: &str, cell: u16) -> Option<&str> {
        let mut parts = self.line(word)?.split('\t').skip(1);
        while let (Some(c), Some(f)) = (parts.next(), parts.next()) {
            if c.parse::<u16>().ok() == Some(cell) {
                return Some(f);
            }
        }
        None
    }

    /// Whether the word looked up as `word` has listed forms.
    pub fn has(&self, word: &str) -> bool {
        self.line(word).is_some()
    }

    /// The word's line.
    fn line(&self, word: &str) -> Option<&str> {
        let table = self.table.as_ref();
        let bytes = table.as_bytes();
        let (mut low, mut high) = (0, table.len());
        while low < high {
            let middle = low + (high - low) / 2;
            let start = bytes
                .get(..middle)?
                .iter()
                .rposition(|&b| b == b'\n')
                .map_or(0, |i| i + 1)
                .max(low);
            let end = bytes
                .get(start..)?
                .iter()
                .position(|&b| b == b'\n')
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

/// The inflection of one part of speech, as learned from the dictionary.
#[derive(Debug, Clone)]
pub struct Grammar<'a, P: Label, A: Label = u8, F: Label = u8> {
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
    /// Which words written in parts inflect each part, read off the first
    /// part (`диван` of `диван-кровать`).
    pub compound: Option<Choice<'a, Compound>>,
    /// Mobile stress, where the language writes it.
    pub accent: Option<Accent<'a, A>>,
    /// Sound laws and spelling rules every built form goes through.
    pub laws: Option<sound_law::Program<'static>>,
    /// Words whose spelling their key doesn't show, as the dictionary writes
    /// them: `(key, letters)`, sorted. A caller who types `fox` means `fōx`;
    /// marks a paradigm writes (the `ō` of `ratiō`) aren't listed.
    pub spellings: Cow<'a, [(Cow<'a, str>, Cow<'a, str>)]>,
    /// How a letter typed without its mark is written, by the letter before
    /// it, it and the letters after it (rules on the beginning of that, the
    /// word's ends written [`WORD_END`]: Latin `tatio#` → `ā`, as in `-ātiō`;
    /// `#de` → `ē`, as in `dē-`; `ae` → `e`, a diphthong): the spelling of a
    /// word that isn't listed.
    pub marks: Option<Choice<'a, char>>,
    /// Forms no rule gives, as the dictionary writes them: `(word, cell,
    /// form)`, sorted by word and cell. They are the answer for that word.
    pub irregular: Irregular<'a>,
    /// How many words' worth a broader rule's counts add to a specific
    /// rule's (see [`Choice::read`]).
    pub smoothing: f32,
    /// The power candidates' probabilities are raised to (then made to sum
    /// to one again), fitted on held-out words so that what is said to be
    /// 80% likely is right about 80% of the time.
    pub sharpness: f32,
}

/// A reading's forms of the cells built so far: per cell, its forms with
/// where their endings start, by how likely.
type Built = std::collections::HashMap<usize, Vec<(Vec<(String, usize)>, f32)>>;

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
        self.sharpened(self.unsharpened(lemma, &[cell], known))
            .pop()
            .unwrap_or_default()
    }

    /// The forms of every cell for `lemma`, as [`Grammar::candidates`]
    /// gives each, read once.
    pub fn table(&self, lemma: &str, known: &Known<'_>) -> Vec<Vec<Candidate<'_>>> {
        let cells: Vec<usize> = (0..self.cells).collect();
        self.sharpened(self.unsharpened(lemma, &cells, known))
    }

    /// Each list of candidates sharpened and sorted, likeliest first.
    fn sharpened<'g>(&self, mut lists: Vec<Vec<Candidate<'g>>>) -> Vec<Vec<Candidate<'g>>> {
        for out in &mut lists {
            sharpen(out.iter_mut().map(|c| &mut c.probability), self.sharpness);
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
        let mut otherwise = None;
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
        let mut inflections = self.inflections(&word.letters, &key, known);
        if inflections.is_empty() {
            let none = Known {
                forms: &[],
                ..*known
            };
            inflections = self.inflections(&word.letters, &key, &none);
        }
        // The likeliest few readings, each with its accent classes and its
        // forms, cell by cell, built once.
        let mut readings: Vec<_> = inflections
            .into_iter()
            .take(KEEP)
            .map(|inf| {
                let classes = self.accent_classes(&word, &inf, cells);
                (inf, classes, Built::default())
            })
            .collect();
        // A word written in parts: as one word, or part by part.
        let (whole, each) = match (&self.compound, lemma.split_once(JOINER)) {
            (Some(compound), Some((first, rest))) => {
                let first_key = self.orthography.key(&self.orthography.read(first).letters);
                let readings = compound.read(&first_key, self.smoothing, |_| true);
                let p = |c: Compound| {
                    readings
                        .iter()
                        .find(|r| r.value == c)
                        .map_or(0.0, |r| r.probability)
                };
                let each = p(Compound::Each);
                let parts = (each > 0.0).then(|| {
                    (
                        top(self.unsharpened(first, cells, &Known::default())),
                        top(self.unsharpened(rest, cells, known)),
                    )
                });
                (p(Compound::Last), parts.map(|parts| (parts, each)))
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
        let listed_word = self.irregular.has(&key);
        let mut lists = Vec::with_capacity(cells.len());
        for (i, &cell) in cells.iter().enumerate() {
            if let Some(form) = self.irregular_form(&key, cell) {
                lists.push(vec![listed(crate::recase(form, casing), 1.0)]);
                continue;
            }
            // A form of a word whose forms are listed, but not this one: told
            // from the listed form of the cell it comes from.
            if let Some((letters, ending)) = listed_word
                .then(|| self.told_from_listed(&key, cell, self.relations.len()))
                .flatten()
            {
                let stresses = match readings.first() {
                    Some((inf, classes, _)) => {
                        self.stresses(inf.reading.value, classes, cell, &letters, ending)
                    }
                    None => vec![(None, 1.0)],
                };
                lists.push(
                    stresses
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
            if let Some(((firsts, rests), p)) = &each {
                for a in firsts.get(i).into_iter().flatten() {
                    for b in rests.get(i).into_iter().flatten() {
                        out.push(Candidate {
                            form: format!("{}{JOINER}{}", a.form, b.form),
                            also: Vec::new(),
                            probability: p * a.probability * b.probability,
                            status: b.status,
                            paradigm: b.paradigm,
                            evidence: b.evidence,
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
                let forms = self.build(inf.paradigm, &inf.stems, &key, known.inherent, cell, built);
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
                        };
                        add(&mut out, candidate);
                    }
                }
            }
            out.retain(|c| c.probability > 0.0);
            lists.push(out);
        }
        // The word spelled the next likeliest way, as likely as that is.
        if let Some((letters, p)) = otherwise {
            let typed = self.orthography.write(&letters, word.stress, casing);
            let full = Known {
                full: true,
                ..*known
            };
            for (out, more) in lists.iter_mut().zip(self.unsharpened(&typed, cells, &full)) {
                // A listed form is the answer however the word is spelled.
                if out
                    .iter()
                    .any(|c| c.status == Status::Listed && c.paradigm.is_empty())
                {
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

    /// The form `form` (written) of the word `rest` with `prefix` kept in
    /// front, as they meet. Its accent stays on the syllable it is on, counted
    /// from the end; one as far back as the law allows on the rest goes as
    /// far back as it allows on the whole word, but not before the prefix's
    /// last syllable, nor before an augment (`ἀπόλυε`, `ἀπῆλθον`).
    fn kept(&self, prefixes: &Prefixes<'_>, prefix: &str, rest: &str, form: &str) -> String {
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
        if let (Some(p), Some(s)) = (&o.prosody, written.stress) {
            let recessive = |letters: &str, long: bool, wall: usize| {
                let syllables = p.syllables(o, letters, long);
                p.accent(&syllables, wall, p.acute)
            };
            let long = [false, true]
                .into_iter()
                .find(|&long| recessive(&written.letters, long, 0) == Some(s));
            if let Some(long) = long {
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

    /// The letters of `cell` for the word looked up as `key`, told from the
    /// listed form of the cell it comes from (or one told so in turn, at
    /// most `steps` away), and where the change starts.
    fn told_from_listed(&self, key: &str, cell: usize, steps: usize) -> Option<(String, usize)> {
        let relation = self.relation(cell).filter(|r| r.listed)?;
        let from = relation.source()?;
        let source = match self.irregular_form(key, from) {
            Some(listed) => self.orthography.read(listed).letters,
            None => self.told_from_listed(key, from, steps.checked_sub(1)?)?.0,
        };
        self.changing(relation).told(&source, self.smoothing)
    }

    /// The relation whose changes `relation` makes: its own, or, for one by
    /// analogy with no changes of its own, those of the relation it is by
    /// analogy with.
    fn changing<'r>(&'r self, relation: &'r Relation<'a>) -> &'r Relation<'a> {
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
        paradigm: &Paradigm<'a>,
        stems: &[&str],
        key: &str,
        inherent: &[(u8, u8)],
        cell: usize,
        built: &mut Built,
    ) -> Vec<(Vec<(String, usize)>, f32)> {
        if let Some(forms) = built.get(&cell) {
            return forms.clone();
        }
        // A cell on the way to itself (none, as relations are learned) has
        // no form.
        built.insert(cell, Vec::new());
        let mut out = Vec::new();
        let forms = paradigm.built(&self.paradigms, self.laws.as_ref(), cell, stems);
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
                    None => choice
                        .read(key, self.smoothing, |_| true)
                        .into_iter()
                        .filter_map(|r| Some((*from.get(r.value.index())?, r.probability)))
                        .collect(),
                },
            };
            let relation = self.changing(relation);
            for (from, p) in sources {
                let from = usize::from(from);
                for (forms, q) in self.build(paradigm, stems, key, inherent, from, built) {
                    if relation.change.is_none() {
                        out.push((forms, p * q));
                        continue;
                    }
                    let Some((usual, ending)) = forms.first() else {
                        continue;
                    };
                    for (changed, at, r) in relation.apply(usual, self.smoothing) {
                        out.push((vec![(changed, (*ending).min(at))], p * q * r));
                    }
                }
            }
            // The likeliest few: along a line of cells each told from the
            // last, alternatives would multiply.
            out.sort_by(|a, b| b.1.total_cmp(&a.1));
            out.truncate(KEEP);
        }
        built.insert(cell, out.clone());
        out
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
        let agrees = |p: &Paradigm<'a>| {
            let Some(stems) = p.stems(&self.orthography, letters) else {
                return false;
            };
            given.iter().all(|(cell, form)| {
                self.build(p, &stems, key, inherent, *cell, &mut Built::default())
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
        if out.is_empty() {
            out.push((None, 1.0));
        }
        out
    }
}

/// The spelling of a word typed as `letters` (looked up as `key`) whose
/// unmarked letters `marks` marks as it says (see [`Grammar::marks`]), and how
/// likely it is.
pub fn restore(
    orthography: &Orthography<'_>,
    marks: &Choice<'_, char>,
    smoothing: f32,
    letters: &str,
    key: &str,
) -> (String, f32) {
    restorations(orthography, marks, smoothing, letters, key).0
}

/// The likeliest spelling (see [`restore`]), and the next likeliest, the
/// letter it is least sure of written otherwise, if that is at least a
/// tenth as likely.
fn restorations(
    o: &Orthography<'_>,
    marks: &Choice<'_, char>,
    smoothing: f32,
    letters: &str,
    key: &str,
) -> ((String, f32), Option<(String, f32)>) {
    let mut p = 1.0;
    let mut out: Vec<char> = Vec::with_capacity(letters.len());
    // The letter least sure of: where, what else, and how much less likely.
    let mut doubt: Option<(usize, char, f32)> = None;
    let mut before = WORD_END;
    for ((i, k), c) in key.char_indices().zip(letters.chars()) {
        let readings = if c == k && o.folded.iter().any(|e| e.1 == c) {
            let around = format!("{before}{}{WORD_END}", key.get(i..).unwrap_or(""));
            marks.read(&around, smoothing, |m| o.fold(m) == c)
        } else {
            Vec::new()
        };
        before = k;
        let [first, rest @ ..] = readings.as_slice() else {
            out.push(c);
            continue;
        };
        if let Some(second) = rest.first()
            && first.probability > 0.0
        {
            let ratio = second.probability / first.probability;
            if doubt.is_none_or(|d| ratio > d.2) {
                doubt = Some((out.len(), second.value, ratio));
            }
        }
        out.push(first.value);
        p *= first.probability;
    }
    let other = doubt.filter(|d| d.2 >= 0.1).map(|(at, other, ratio)| {
        let alt = out.iter().enumerate();
        let alt = alt.map(|(i, &c)| if i == at { other } else { c });
        (alt.collect(), p * ratio)
    });
    ((out.into_iter().collect(), p), other)
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
