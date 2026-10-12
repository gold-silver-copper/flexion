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
}

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

/// The forms no rule gives, as a table: a line per form, `word`, tab, the
/// cell's number, tab, `form`, sorted by word and cell. A word is as it is
/// looked up (see [`Orthography::key`]).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Irregular<'a> {
    pub table: Cow<'a, str>,
}

impl Irregular<'_> {
    /// The table of `forms`: `(word, cell, form)`.
    pub fn new(forms: &[(String, usize, String)]) -> Irregular<'static> {
        let mut lines: Vec<(&str, usize, &str)> = forms
            .iter()
            .map(|(w, c, f)| (w.as_str(), *c, f.as_str()))
            .collect();
        lines.sort_unstable();
        lines.dedup_by(|a, b| a.0 == b.0 && a.1 == b.1);
        let table = lines
            .iter()
            .map(|(w, c, f)| format!("{w}\t{c}\t{f}\n"))
            .collect();
        Irregular {
            table: Cow::Owned(table),
        }
    }

    /// Every form: `(word, cell, form)`.
    pub fn iter(&self) -> impl Iterator<Item = (&str, u16, &str)> {
        self.table.lines().filter_map(line)
    }

    /// How many forms the table lists.
    pub fn len(&self) -> usize {
        self.table.lines().count()
    }

    pub fn is_empty(&self) -> bool {
        self.table.is_empty()
    }

    /// The form of `cell` for the word looked up as `word`, if listed.
    pub fn get(&self, word: &str, cell: u16) -> Option<&str> {
        self.find(word, Some(cell))
    }

    /// Whether the word looked up as `word` has listed forms.
    pub fn has(&self, word: &str) -> bool {
        self.find(word, None).is_some()
    }

    /// A listed form of the word, of `cell` if given.
    fn find(&self, word: &str, cell: Option<u16>) -> Option<&str> {
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
            let (w, c, f) = line(table.get(start..end)?)?;
            match w
                .cmp(word)
                .then(cell.map_or(std::cmp::Ordering::Equal, |cell| c.cmp(&cell)))
            {
                std::cmp::Ordering::Less => low = end + 1,
                std::cmp::Ordering::Greater => high = start,
                std::cmp::Ordering::Equal => return Some(f),
            }
        }
        None
    }
}

/// A table line's word, cell and form.
fn line(line: &str) -> Option<(&str, u16, &str)> {
    let mut parts = line.splitn(3, '\t');
    let word = parts.next()?;
    let cell = parts.next()?.parse().ok()?;
    Some((word, cell, parts.next()?))
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
    /// Which paradigm a word follows.
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

/// One way of reading the word: a paradigm with the word's stems.
struct Inflection<'g, 'a, 'w, P: Label> {
    reading: Reading<'g, 'a, P>,
    paradigm: &'g Paradigm<'a>,
    stems: pattern::Stems<'w>,
}

impl<'a, P: Label, A: Label, F: Label> Grammar<'a, P, A, F> {
    pub fn paradigm_of(&self, p: P) -> Option<&Paradigm<'a>> {
        self.paradigms.get(p.index())
    }

    /// The forms of `cell` for `lemma`, likeliest first, given what the
    /// caller knows (readings that disagree with it are dropped, unless all
    /// do).
    pub fn candidates(&self, lemma: &str, cell: usize, known: &Known<'_>) -> Vec<Candidate<'_>> {
        let mut out = self.unsharpened(lemma, cell, known);
        sharpen(out.iter_mut().map(|c| &mut c.probability), self.sharpness);
        out.sort_by(|a, b| b.probability.total_cmp(&a.probability));
        out
    }

    /// [`Grammar::candidates`] before sharpening, in no order.
    fn unsharpened(&self, lemma: &str, cell: usize, known: &Known<'_>) -> Vec<Candidate<'_>> {
        let mut word = self.orthography.read(lemma);
        let key = self.orthography.key(&word.letters);
        // A word typed without some of its marks: as listed, or as its
        // letters say.
        let mut spelled = 1.0;
        if !known.full {
            match self.spelling(&word.letters, &key) {
                Some(letters) => word.letters = letters.to_string(),
                None => (word.letters, spelled) = self.restore(&word.letters, &key),
            }
        }
        let casing = if self.capitalized {
            Casing::Title
        } else {
            word.casing
        };
        if let Some(form) = self.irregular_form(&key, cell) {
            return vec![listed(crate::recase(form, casing), 1.0)];
        }
        let given = self.given(known);
        let mut inflections = self.inflections(&word.letters, &key, &given, known.inherent);
        if inflections.is_empty() {
            inflections = self.inflections(&word.letters, &key, &[], known.inherent);
        }
        // A form of a word whose forms are listed, but not this one: told
        // from the listed form of the cell it comes from.
        if let Some((letters, ending)) = self.from_listed(&key, cell, 0) {
            let stresses = match inflections.first() {
                Some(inf) => self.stresses(&word, inf, cell, &letters, ending),
                None => vec![(None, 1.0)],
            };
            return stresses
                .into_iter()
                .map(|(stress, p)| listed(self.orthography.write(&letters, stress, casing), p))
                .collect();
        }
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
                (p(Compound::Last), Some((first, rest, p(Compound::Each))))
            }
            _ => (1.0, None),
        };
        let mut out: Vec<Candidate<'_>> = Vec::new();
        // A word that begins with a prefix: the rest of it, the prefix where
        // the cell puts it; or a word of its own.
        let whole = match self
            .prefixes
            .as_ref()
            .and_then(|p| Some((p, p.split(&key)?)))
        {
            Some((prefixes, (letters, separable, inseparable)))
                if separable + inseparable > 0.0 =>
            {
                let at = word
                    .letters
                    .char_indices()
                    .nth(letters)
                    .map_or(word.letters.len(), |(i, _)| i);
                let (prefix, rest) = word.letters.split_at(at);
                let placement = prefixes
                    .placement
                    .get(cell)
                    .copied()
                    .unwrap_or(Placement::Before);
                let drop = prefixes.drops.get(cell).map_or("", |d| d.as_ref());
                let base = Known {
                    forms: &[],
                    ..*known
                };
                for c in top(self.unsharpened(rest, cell, &base)) {
                    let separated = match placement {
                        Placement::Before => format!("{prefix}{}", c.form),
                        Placement::After => format!("{} {prefix}", c.form),
                    };
                    let kept = format!("{prefix}{}", c.form.strip_prefix(drop).unwrap_or(&c.form));
                    for (form, p) in [(separated, separable), (kept, inseparable)] {
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
                whole * (1.0 - separable - inseparable)
            }
            _ => whole,
        };
        if let Some((first, rest, p)) = each.filter(|e| e.2 > 0.0) {
            let firsts = top(self.unsharpened(first, cell, &Known::default()));
            let rests = top(self.unsharpened(rest, cell, known));
            for a in &firsts {
                for b in &rests {
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
        for (inf, (forms, p_cell)) in inflections.iter().take(KEEP).flat_map(|inf| {
            self.build(inf.paradigm, &inf.stems, &key, known.inherent, cell, 0)
                .into_iter()
                .map(move |b| (inf, b))
        }) {
            let Some(((first, ending), also)) = forms.split_first() else {
                continue;
            };
            let also: Vec<&String> = also.iter().map(|(f, _)| f).collect();
            let evidence = inf.reading.rule.map(|r| Evidence {
                ending: r.ending.as_ref(),
                count: r.count(inf.reading.value),
                total: r.total(),
            });
            for (stress, p_stress) in self.stresses(&word, inf, cell, first, *ending) {
                let write = |f: &str| self.orthography.write(f, stress, casing);
                let candidate = Candidate {
                    form: write(first),
                    also: also.iter().map(|f| write(f)).collect(),
                    probability: spelled * whole * p_cell * inf.reading.probability * p_stress,
                    status: inf.reading.status,
                    paradigm: inf.paradigm.name.as_ref(),
                    evidence,
                };
                add(&mut out, candidate);
            }
        }
        out.retain(|c| c.probability > 0.0);
        out
    }

    /// The letters of `cell` for the word looked up as `key`, told from the
    /// listed form of the cell it comes from (or one told so in turn), and
    /// where the change starts.
    pub fn from_listed(&self, key: &str, cell: usize, depth: usize) -> Option<(String, usize)> {
        if depth == 0 && !self.irregular.has(key) {
            return None;
        }
        let relation = self
            .relation(cell)
            .filter(|r| r.listed && depth < self.cells)?;
        let from = relation.source()?;
        let source = match self.irregular_form(key, from) {
            Some(listed) => self.orthography.read(listed).letters,
            None => self.from_listed(key, from, depth + 1)?.0,
        };
        relation.told(&source, self.smoothing)
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
    /// feature, given or chosen), changed.
    fn build(
        &self,
        paradigm: &Paradigm<'a>,
        stems: &[&str],
        key: &str,
        inherent: &[(u8, u8)],
        cell: usize,
        depth: usize,
    ) -> Vec<(Vec<(String, usize)>, f32)> {
        if !paradigm.derives(&self.paradigms, cell) {
            let forms = paradigm.built(&self.paradigms, self.laws.as_ref(), cell, stems);
            return vec![(forms, 1.0)];
        }
        let Some(relation) = self.relation(cell).filter(|_| depth < self.cells) else {
            return Vec::new();
        };
        let sources: Vec<(u16, f32)> = match relation.from.as_ref() {
            [from] => vec![(*from, 1.0)],
            from => {
                let Some((f, choice)) = &self.feature else {
                    return Vec::new();
                };
                match inherent.iter().find(|(g, _)| g == f) {
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
                }
            }
        };
        let mut out = Vec::new();
        for (from, p) in sources {
            for (forms, q) in
                self.build(paradigm, stems, key, inherent, usize::from(from), depth + 1)
            {
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
        out
    }

    /// The paradigms `lemma` can follow, likeliest first, each with how
    /// sure and why, given what the caller knows.
    pub fn readings(&self, lemma: &str, known: &Known<'_>) -> Vec<Reading<'_, 'a, P>> {
        let mut letters = self.orthography.read(lemma).letters;
        let key = self.orthography.key(&letters);
        if let Some(listed) = self.spelling(&letters, &key).filter(|_| !known.full) {
            letters = listed.to_string();
        }
        let mut readings: Vec<Reading<'_, 'a, P>> = self
            .inflections(&letters, &key, &self.given(known), known.inherent)
            .into_iter()
            .map(|i| i.reading)
            .collect();
        sharpen(
            readings.iter_mut().map(|r| &mut r.probability),
            self.sharpness,
        );
        readings
    }

    /// The forms the caller gave, read.
    fn given(&self, known: &Known<'_>) -> Vec<(usize, Written)> {
        known
            .forms
            .iter()
            .map(|&(c, f)| (c, self.orthography.read(f)))
            .collect()
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

    /// The spelling of a word typed as `letters` (looked up as `key`) whose
    /// unmarked letters [`Grammar::marks`] marks as it says, and how likely
    /// it is.
    pub fn restore(&self, letters: &str, key: &str) -> (String, f32) {
        match &self.marks {
            Some(marks) => restore(&self.orthography, marks, self.smoothing, letters, key),
            None => (letters.to_string(), 1.0),
        }
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
    /// likeliest first, agreeing with `given` forms; a paradigm whose words
    /// have other `inherent` feature values than these is less likely (by
    /// [`OTHER_VALUES`]).
    fn inflections<'g, 'w>(
        &'g self,
        letters: &'w str,
        key: &str,
        given: &[(usize, Written)],
        inherent: &[(u8, u8)],
    ) -> Vec<Inflection<'g, 'a, 'w, P>> {
        let agrees = |p: &Paradigm<'a>| {
            let Some(stems) = p.stems(&self.orthography, letters) else {
                return false;
            };
            given.iter().all(|(cell, form)| {
                self.build(p, &stems, key, inherent, *cell, 0)
                    .iter()
                    .any(|(forms, _)| forms.iter().any(|(built, _)| *built == form.letters))
            })
        };
        let fits = |p: P| self.paradigm_of(p).is_some_and(agrees);
        let mut readings = self.paradigm.read(key, self.smoothing, fits);
        if readings.is_empty() {
            // No rule knows a paradigm the word fits: every paradigm it does,
            // by how many words follow it.
            readings = self.fallback(agrees);
        }
        for r in &mut readings {
            if self.paradigm_of(r.value).is_some_and(|p| !p.has(inherent)) {
                r.probability *= OTHER_VALUES;
            }
        }
        readings.sort_by(|a, b| b.probability.total_cmp(&a.probability));
        readings
            .into_iter()
            .filter_map(|reading| {
                let paradigm = self.paradigm_of(reading.value)?;
                let stems = paradigm.stems(&self.orthography, letters)?;
                Some(Inflection {
                    reading,
                    paradigm,
                    stems,
                })
            })
            .collect()
    }

    fn fallback(&self, agrees: impl Fn(&Paradigm<'a>) -> bool) -> Vec<Reading<'_, 'a, P>> {
        let fitting: Vec<(P, u32)> = self
            .paradigms
            .iter()
            .enumerate()
            .filter(|(_, p)| agrees(p))
            .filter_map(|(i, p)| Some((P::from_index(i)?, p.words.max(1))))
            .collect();
        let total: f64 = fitting.iter().map(|&(_, n)| f64::from(n)).sum();
        let mut out: Vec<Reading<'_, 'a, P>> = fitting
            .into_iter()
            .map(|(value, n)| Reading {
                value,
                #[allow(clippy::cast_possible_truncation)]
                probability: (f64::from(n) / total) as f32,
                status: Status::Unsettled,
                rule: None,
            })
            .collect();
        out.sort_by(|a, b| b.probability.total_cmp(&a.probability));
        out
    }

    /// Where the form of `cell` is stressed, with probabilities, for an
    /// inflection whose cell ending starts at byte `ending`. One unstressed
    /// reading when the language writes no stress.
    fn stresses(
        &self,
        word: &Written,
        inf: &Inflection<'_, 'a, '_, P>,
        cell: usize,
        form: &str,
        ending: usize,
    ) -> Vec<(Option<crate::Stress>, f32)> {
        let Some(accent) = &self.accent else {
            return vec![(None, 1.0)];
        };
        let letters = word.letters.as_str();
        let o = &self.orthography;
        let positions = stress::positions(o, letters);
        // Where the lemma's own ending starts: an accent the caller wrote
        // says which patterns the word can follow, and where its stem is
        // accented.
        let lemma_ending = pattern::ending_start(&inf.paradigm.lemma, &inf.stems);
        let paradigm = inf.reading.value.index();
        let lemma_finals = self
            .lemma_cell
            .map(|c| accent.finals(paradigm, c))
            .unwrap_or_default();
        let finals = accent.finals(paradigm, cell);
        let class_place = |class: &crate::StressClass<'_>| {
            let c = self.lemma_cell?;
            accent.pattern_of(class)?.place(c)
        };
        let given = word.stress;
        let given_at = given.map(|g| stress::position(o, letters, g));
        // A class fits an accent the caller wrote when its pattern puts the
        // lemma's accent there.
        let fits = |a: A| {
            accent.class_of(a).is_some_and(|class| {
                let (Some(g), Some(place)) = (given, class_place(class)) else {
                    return true;
                };
                let at = given_at.unwrap_or(0);
                stress::place(o, letters, lemma_ending, lemma_finals, place, at) == Some(g)
            })
        };
        let classes = accent
            .class
            .read(&self.orthography.key(letters), self.smoothing, fits);
        let mut out: Vec<(Option<crate::Stress>, f32)> = Vec::new();
        // The form's letters, to place the stress in.
        for reading in classes.iter().take(KEEP * KEEP) {
            let Some(class) = accent.class_of(reading.value) else {
                continue;
            };
            let Some(place) = accent.pattern_of(class).and_then(|p| p.place(cell)) else {
                continue;
            };
            // The stem's accented syllable: the one written, when the
            // lemma's accent is on its stem; else the class's, from the
            // lemma's end.
            let k = match given_at {
                Some(at) if class_place(class) == Some(Place::Stem) => Some(at),
                _ => class
                    .vowel
                    .and_then(|v| positions.checked_sub(1)?.checked_sub(usize::from(v))),
            };
            let s = stress::place(o, form, ending, finals, place, k.unwrap_or(0));
            match out.iter_mut().find(|e| e.0 == s) {
                Some(e) => e.1 += reading.probability,
                None => out.push((s, reading.probability)),
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
    let o = orthography;
    let mut p = 1.0;
    let mut out = String::with_capacity(letters.len().saturating_add(8));
    let mut before = WORD_END;
    for ((i, k), c) in key.char_indices().zip(letters.chars()) {
        let markable = c == k && o.folded.iter().any(|e| e.1 == c);
        let around = format!("{before}{}{WORD_END}", key.get(i..).unwrap_or(""));
        before = k;
        let reading = markable
            .then(|| {
                marks
                    .read(&around, smoothing, |m| o.fold(m) == c)
                    .into_iter()
                    .next()
            })
            .flatten();
        match reading {
            Some(r) => {
                out.push(r.value);
                p *= r.probability;
            }
            None => out.push(c),
        }
    }
    (out, p)
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

/// The likeliest few of `candidates`.
fn top(mut candidates: Vec<Candidate<'_>>) -> Vec<Candidate<'_>> {
    candidates.sort_by(|a, b| b.probability.total_cmp(&a.probability));
    candidates.truncate(KEEP - 1);
    candidates
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
