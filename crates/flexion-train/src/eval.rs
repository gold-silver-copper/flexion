//! Held-out evaluation: words are split into tenths by a hash of their key;
//! one tenth is never trained on and scored, and how well the stated
//! probabilities match how often the answers are right is measured.

use crate::extract::Lexeme;
use crate::measure::{self, Kind};
use flexion::Label;

/// The tenth the READMEs report: scored once per build, never trained on,
/// and never what a change is judged on (see [`DEVELOP`]).
pub const TEST: u64 = 0;
/// The tenth the sharpness is fitted on when [`TEST`] is scored.
pub const DEV: u64 = 1;
/// The tenth a change is judged on (`try` scores it unless told otherwise;
/// its sharpness is fitted on [`dev_of`] it).
pub const DEVELOP: u64 = 2;

/// The tenth settings are chosen on when tenth `test` is scored: the next.
pub fn dev_of(test: u64) -> u64 {
    (test + 1) % 10
}

/// A stable tenth for a word: FNV-1a of its key, mod 10.
pub fn bucket(key: &str) -> u64 {
    hash(key) % 10
}

/// How a held-out word is typed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Input<'p> {
    /// In full, as the dictionary's table writes it (with every mark:
    /// accents, vowel length), and said to be so ([`flexion::Known::full`]).
    Full,
    /// As the dictionary's headword writes it (Latin `rosa`, Greek `λύω`:
    /// no vowel length), as people type it.
    Headword,
    /// In full, with the dictionary's forms of these cells (its principal
    /// parts) given; those cells aren't scored.
    Citation(&'p [usize]),
    /// As the headword, with the word's inherent features (its gender)
    /// given.
    Features,
    /// In full, with `k` of the word's forms given, chosen by a hash of the
    /// word and the cell (see [`given_cells`]); those cells aren't scored.
    Given(usize),
}

/// Scores for one input mode.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Score {
    /// Forms scored.
    pub forms: usize,
    /// Top candidate right.
    pub right: usize,
    /// Right among the top three.
    pub top3: usize,
    /// Summed over forms: one over the right answer's place among the
    /// candidates (0 where it isn't one).
    pub reciprocal: f64,
    /// Top candidate's letters right (its stress aside).
    pub letters: usize,
    /// Per probability tenth of the top candidate: `(forms, right, sum of
    /// probabilities)`.
    pub bins: [(usize, usize, f64); 10],
    /// `(forms, right)` on the cells the word's paradigm (its likeliest
    /// one) has some training word's form in, and on the others.
    pub seen: (usize, usize),
    pub unseen: (usize, usize),
    /// `(forms, right)` on the lemma's cell, where its form isn't the
    /// letters typed (vowel length the headword doesn't write).
    pub lemma: (usize, usize),
    /// Top candidate is one of the forms the dictionary lists, any of them.
    pub any: usize,
    /// The wrong forms by what went wrong, in [`Kind::ALL`]'s order.
    pub errors: [usize; Kind::ALL.len()],
    /// First candidates without the stress mark the right answer writes
    /// (Greek `ὀστουν` for `ὀστοῦν`).
    pub accentless: usize,
    /// The right answer among the candidates at all, at any place.
    pub among: usize,
    /// Summed over forms: −log₂ of the probability the grammar gives the
    /// right answer, at most its listing cost (see [`Outcome::bits`]).
    pub bits: f64,
    /// Words scored (with at least one form), and those with every scored
    /// form right.
    pub words: usize,
    pub whole: usize,
    /// Top candidate compatible with the length the dictionary states (see
    /// [`Outcome::compatible`]): a second score, never more right forms.
    pub compatible: usize,
}

/// What became of one scored form.
#[derive(Debug, Clone, PartialEq)]
pub struct Outcome {
    /// The word: its place among the lexemes scored.
    pub word: usize,
    pub cell: usize,
    /// The right answer's place among the candidates (0: the first), if it
    /// is one.
    pub rank: Option<usize>,
    /// The first candidate's probability.
    pub top: f32,
    /// The probability of the right answer: summed over the candidates that
    /// write it.
    pub probability: f32,
    /// The first candidate's letters are right (its stress aside).
    pub letters: bool,
    /// The cell is one the word's likeliest paradigm has training forms in.
    pub seen: bool,
    /// The cell is the lemma's (and its form isn't the letters typed).
    pub lemma: bool,
    /// The first candidate writes one of the forms the dictionary lists in
    /// the cell, not only its first.
    pub any: bool,
    /// −log₂ [`Outcome::probability`], at most the right answer's listing
    /// cost: [`LETTER_BITS`] per letter and one more for its end. A form no
    /// candidate writes costs its listing.
    pub bits: f64,
    /// What went wrong, if the first candidate isn't right.
    pub error: Option<Kind>,
    /// The first candidate, a word, has no stress mark where the right
    /// answer has one.
    pub accentless: bool,
    /// The first candidate is right, or is the right answer but for length
    /// marks on vowels it writes bare, each compatible with the length the
    /// word's table states of that vowel (see [`TableLength`]).
    pub compatible: bool,
    /// The same of its letters alone (stress aside), as the floor is
    /// scored.
    pub compatible_letters: bool,
}

impl Outcome {
    pub fn right(&self) -> bool {
        self.rank == Some(0)
    }
}

/// What a listed form costs per letter, in bits, in the log loss: one of 32
/// symbols. A fixed cost, the same for every language, so that held-out
/// bits compare across grammars and runs.
pub const LETTER_BITS: f64 = 5.0;

/// The cost of listing `form` whole: [`LETTER_BITS`] for each letter and
/// for its end.
pub fn listing_bits(form: &str) -> f64 {
    #[allow(clippy::cast_precision_loss)]
    let letters = form.chars().count() as f64;
    LETTER_BITS * (letters + 1.0)
}

impl Score {
    pub fn accuracy(&self) -> f64 {
        ratio(self.right, self.forms)
    }

    /// The share of forms whose top candidate is compatible with the length
    /// the dictionary states (see [`Outcome::compatible`]).
    pub fn compatible_accuracy(&self) -> f64 {
        ratio(self.compatible, self.forms)
    }

    pub fn top3_accuracy(&self) -> f64 {
        ratio(self.top3, self.forms)
    }

    /// The share of forms whose right answer is among the candidates at all.
    pub fn among_accuracy(&self) -> f64 {
        ratio(self.among, self.forms)
    }

    /// The share of forms whose top candidate is any form the dictionary
    /// lists in the cell.
    pub fn any_accuracy(&self) -> f64 {
        ratio(self.any, self.forms)
    }

    /// The share of words with every scored form right.
    pub fn whole_words(&self) -> f64 {
        ratio(self.whole, self.words)
    }

    /// Held-out log loss: bits per form (see [`Outcome::bits`]).
    pub fn log_loss(&self) -> f64 {
        #[allow(clippy::cast_precision_loss)]
        let n = self.forms.max(1) as f64;
        self.bits / n
    }

    /// Tally outcomes (in word order, as [`outcomes`] gives them).
    pub fn of(outcomes: &[Outcome]) -> Score {
        let mut s = Score::default();
        let mut word: Option<(usize, bool)> = None;
        for o in outcomes {
            let right = o.right();
            s.forms += 1;
            s.right += usize::from(right);
            s.letters += usize::from(o.letters);
            s.top3 += usize::from(o.rank.is_some_and(|r| r < 3));
            s.among += usize::from(o.rank.is_some());
            if let Some(r) = o.rank {
                #[allow(clippy::cast_precision_loss)]
                let place = (r + 1) as f64;
                s.reciprocal += 1.0 / place;
            }
            let split = if o.seen { &mut s.seen } else { &mut s.unseen };
            split.0 += 1;
            split.1 += usize::from(right);
            if o.lemma {
                s.lemma.0 += 1;
                s.lemma.1 += usize::from(right);
            }
            s.any += usize::from(o.any);
            s.accentless += usize::from(o.accentless);
            s.compatible += usize::from(o.compatible);
            if let Some(slot) = o
                .error
                .and_then(|e| Kind::ALL.iter().position(|&k| k == e))
                .and_then(|i| s.errors.get_mut(i))
            {
                *slot += 1;
            }
            s.bits += o.bits;
            s.calibrate(f64::from(o.top), right);
            word = match word {
                Some((w, all)) if w == o.word => Some((w, all && right)),
                other => {
                    if let Some((_, all)) = other {
                        s.words += 1;
                        s.whole += usize::from(all);
                    }
                    Some((o.word, right))
                }
            };
        }
        if let Some((_, all)) = word {
            s.words += 1;
            s.whole += usize::from(all);
        }
        s
    }

    /// The wrong forms by what went wrong, as `kind count, …` with each
    /// kind's share of the wrong forms, commonest first (kinds with none
    /// left out).
    pub fn census(&self) -> String {
        let wrong: usize = self.errors.iter().sum();
        let mut kinds: Vec<(Kind, usize)> = Kind::ALL
            .iter()
            .copied()
            .zip(self.errors)
            .filter(|&(_, n)| n > 0)
            .collect();
        kinds.sort_by_key(|k| std::cmp::Reverse(k.1));
        kinds
            .iter()
            .map(|(k, n)| {
                #[allow(clippy::cast_precision_loss)]
                let share = 100.0 * *n as f64 / wrong.max(1) as f64;
                format!("{k} {n} ({share:.0}%)")
            })
            .collect::<Vec<_>>()
            .join(", ")
            + &if self.accentless > 0 {
                format!("; {} without the accent they need", self.accentless)
            } else {
                String::new()
            }
    }

    /// The mean reciprocal rank of the right answer.
    pub fn reciprocal_rank(&self) -> f64 {
        #[allow(clippy::cast_precision_loss)]
        let n = self.forms.max(1) as f64;
        self.reciprocal / n
    }

    /// Expected calibration error: how far, on average over forms, the stated
    /// probability of the top candidate is from how often it is right.
    pub fn calibration_error(&self) -> f64 {
        let mut e = 0.0;
        for &(n, right, p) in &self.bins {
            if n > 0 {
                #[allow(clippy::cast_precision_loss)]
                let (n_f, right_f) = (n as f64, right as f64);
                e += (right_f / n_f - p / n_f).abs() * n_f;
            }
        }
        #[allow(clippy::cast_precision_loss)]
        let total = self.forms.max(1) as f64;
        e / total
    }

    /// Count a form whose top candidate, said to be `p` likely, is `right`
    /// or not, in its probability tenth.
    fn calibrate(&mut self, p: f64, right: bool) {
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let bin = ((p * 10.0) as usize).min(9);
        if let Some(b) = self.bins.get_mut(bin) {
            b.0 += 1;
            b.1 += usize::from(right);
            b.2 += p;
        }
    }
}

fn ratio(a: usize, b: usize) -> f64 {
    #[allow(clippy::cast_precision_loss)]
    let r = a as f64 / b.max(1) as f64;
    r
}

/// Whether two written forms are the same word form: the same letters and
/// the same (main) stress. Marks the language's reading drops, and stress
/// marks before the main one (a compound's first part), don't count.
pub fn same_form(orthography: &flexion::Orthography<'_>, a: &str, b: &str) -> bool {
    let (x, y) = (orthography.read(a), orthography.read(b));
    x.letters == y.letters && x.stress == y.stress
}

/// The length one word's table states of each of its vowels, for a
/// spelling that marks length both ways (`stated`: Greek macron and breve;
/// see [`crate::spec::Spelling::stated_length`]): the length marks it
/// writes on the vowel in any of its forms (a circumflex reads as long),
/// none where it leaves the vowel bare in every cell. A vowel is told by
/// its letter and the two before it, marks aside, so that a stem's vowel is
/// the same vowel in every cell (`βαλ` of `προβάλλω`, `προβάλοισθε`).
///
/// An answer that writes a length mark on a vowel the dictionary's form
/// leaves bare is compatible with the length the dictionary states where
/// the table never states that vowel otherwise: it leaves it bare in every
/// cell (the length unstated: a table typed with a bare stem), or marks it
/// so elsewhere (the pluperfect's bare `ἠγαπήμεθᾰ` of `ἀγαπάω`, whose
/// imperfect writes `ἠγᾰ́πων`). One the table marks the other way anywhere
/// is not: the length is stated, and the answer contradicts it.
#[derive(Debug, Clone)]
pub struct TableLength<'a> {
    stated: &'a str,
    /// Per vowel, the marks of `stated` the table writes on it, a bit each.
    vowels: crate::HashMap<[char; 3], u8>,
}

/// A form's letters (as read), each its base letter and its marks.
fn marked_letters(letters: &str) -> Vec<(char, String)> {
    use unicode_normalization::UnicodeNormalization;
    let mut out: Vec<(char, String)> = Vec::new();
    for c in letters.nfd() {
        match out.last_mut() {
            Some((_, marks)) if unicode_normalization::char::is_combining_mark(c) => marks.push(c),
            _ => out.push((c, String::new())),
        }
    }
    out
}

/// Each letter's key, as [`TableLength`] tells a vowel: the two letters
/// before it and its own, marks aside (`^` before the word).
fn letter_keys(letters: &[(char, String)]) -> impl Iterator<Item = [char; 3]> + '_ {
    let base = |i: Option<usize>| i.and_then(|i| letters.get(i)).map_or('^', |l| l.0);
    (0..letters.len()).map(move |i| {
        [
            base(i.checked_sub(2)),
            base(i.checked_sub(1)),
            base(Some(i)),
        ]
    })
}

impl<'a> TableLength<'a> {
    /// The length marks `forms` (a word's table) write on each vowel; none
    /// where `stated` is empty.
    pub fn of<'f>(
        orthography: &flexion::Orthography<'_>,
        stated: &'a str,
        forms: impl IntoIterator<Item = &'f str>,
    ) -> TableLength<'a> {
        let mut table = TableLength {
            stated,
            vowels: crate::HashMap::default(),
        };
        if stated.is_empty() {
            return table;
        }
        for form in forms {
            let letters = marked_letters(&orthography.read(form).letters);
            for (key, (base, marks)) in letter_keys(&letters).zip(&letters) {
                if orthography.is_vowel(*base) {
                    let bits = table.bits(marks);
                    *table.vowels.entry(key).or_default() |= bits;
                }
            }
        }
        table
    }

    /// The marks of `stated` among `marks`, a bit each.
    fn bits(&self, marks: &str) -> u8 {
        self.stated
            .chars()
            .zip(0u32..8)
            .filter(|&(m, _)| marks.contains(m))
            .fold(0, |bits, (_, i)| bits | 1u8.checked_shl(i).unwrap_or(0))
    }

    /// Whether `answer` is `want` (both letters, as read) but for length
    /// marks on vowels `want` writes bare, each compatible with the length
    /// the table states of that vowel (see [`TableLength`]).
    pub fn compatible(&self, want: &str, answer: &str) -> bool {
        if self.vowels.is_empty() {
            return false;
        }
        let (w, a) = (marked_letters(want), marked_letters(answer));
        let stated = |m: char| self.stated.contains(m);
        w.len() == a.len()
            && letter_keys(&w)
                .zip(w.iter().zip(&a))
                .all(|(key, ((wb, wm), (ab, am)))| {
                    wb == ab
                        && (wm == am
                            || (!wm.chars().any(stated)
                                && am.chars().filter(|&m| !stated(m)).eq(wm.chars())
                                && self
                                    .vowels
                                    .get(&key)
                                    .is_some_and(|&table| table & !self.bits(am) == 0)))
                })
    }
}

/// What a table's first answer `top` is against the dictionary's forms of
/// the cell (`listed`, the first the one scored), compared as
/// [`same_form`] compares them, as `try` scores it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verdict {
    /// No answer: a gap.
    Gap,
    /// The dictionary has no form to score it against.
    Unlisted,
    Right,
    /// Another of the forms the dictionary lists.
    Listed,
    Wrong,
}

/// See [`Verdict`].
pub fn verdict(
    orthography: &flexion::Orthography<'_>,
    top: Option<&str>,
    listed: &[String],
) -> Verdict {
    match (top, listed.first()) {
        (None, _) => Verdict::Gap,
        (Some(_), None) => Verdict::Unlisted,
        (Some(t), Some(want)) if same_form(orthography, t, want) => Verdict::Right,
        (Some(t), _) if listed.iter().any(|f| same_form(orthography, t, f)) => Verdict::Listed,
        _ => Verdict::Wrong,
    }
}

/// Score `grammar` on `lexemes` (held out), every cell the dictionary lists,
/// against its first form, typed as `input`. `seen`: per paradigm, the
/// cells its training words have forms in; `stated`, the marks that state
/// length both ways (see [`TableLength`]).
pub fn score<P: Label + Sync, A: Label + Sync, F: Label + Sync>(
    grammar: &flexion::Grammar<'_, P, A, F>,
    orthography: &flexion::Orthography<'_>,
    stated: &str,
    lexemes: &[&Lexeme],
    input: Input<'_>,
    seen: &[Vec<bool>],
) -> Score {
    Score::of(&outcomes(
        grammar,
        orthography,
        stated,
        lexemes,
        input,
        seen,
        |_| true,
    ))
}

/// The cells given for `k` forms given ([`Input::Given`]): `k` of the
/// cells `l` has forms in (the lemma's aside), those with the lowest hash
/// of the word and the cell. None when the word has no more than `k` such
/// cells (nothing would be left to score).
pub fn given_cells(l: &Lexeme, k: usize, lemma_cell: Option<usize>) -> Option<Vec<usize>> {
    let mut cells: Vec<(u64, usize)> = l
        .cells
        .iter()
        .enumerate()
        .filter(|&(c, fs)| !fs.is_empty() && Some(c) != lemma_cell)
        .map(|(c, _)| (hash(&format!("{}\u{1f}{c}", l.key)), c))
        .collect();
    if cells.len() <= k {
        return None;
    }
    cells.sort_unstable();
    Some(cells.into_iter().take(k).map(|(_, c)| c).collect())
}

/// FNV-1a of a string.
pub fn hash(s: &str) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in s.bytes() {
        h ^= u64::from(b);
        h = h.wrapping_mul(0x0100_0000_01b3);
    }
    h
}

/// Whether the form `want` of cell `cell` is what was typed (`typed`): the
/// lemma's cell, the same letters. Its answer is the input itself, so it
/// isn't scored, by the grammar or the floor.
pub fn typed_back(
    orthography: &flexion::Orthography<'_>,
    lemma_cell: Option<usize>,
    cell: usize,
    want: &str,
    typed: &str,
) -> bool {
    lemma_cell == Some(cell) && orthography.read(want).letters == orthography.read(typed).letters
}

/// Every form of a word's table.
fn word_forms(l: &Lexeme) -> impl Iterator<Item = &str> {
    l.cells.iter().flatten().map(String::as_str)
}

/// Every scored form of `lexemes` (held out) in the cells `scored` accepts,
/// against the dictionary's first form, typed as `input`, word by word; the
/// lemma's cell only where its form isn't what was typed (see
/// [`typed_back`]). `seen`: per paradigm, the cells its training words have
/// forms in (empty: every cell counts as seen). `stated`: the marks that
/// state length both ways, for [`Outcome::compatible`] (empty: none).
#[allow(clippy::too_many_arguments)]
pub fn outcomes<P: Label + Sync, A: Label + Sync, F: Label + Sync>(
    grammar: &flexion::Grammar<'_, P, A, F>,
    orthography: &flexion::Orthography<'_>,
    stated: &str,
    lexemes: &[&Lexeme],
    input: Input<'_>,
    seen: &[Vec<bool>],
    scored: impl Fn(usize) -> bool + Sync,
) -> Vec<Outcome> {
    // Words spelled alike, for the homographs: per key, the scored words.
    let mut alike: crate::HashMap<&str, Vec<&Lexeme>> = crate::HashMap::default();
    for l in lexemes {
        alike.entry(l.key.as_str()).or_default().push(l);
    }
    let numbered: Vec<(usize, &&Lexeme)> = lexemes.iter().enumerate().collect();
    let each = crate::par::map(&numbered, |&(word, l)| {
        let mut out = Vec::new();
        let firsts: Vec<String> = l.cells.iter().filter_map(|f| f.first().cloned()).collect();
        let stem = measure::word_stem(&l.full, &firsts);
        let unstated = TableLength::of(orthography, stated, word_forms(l));
        let typed = match input {
            Input::Full | Input::Citation(_) | Input::Given(_) => l.full.clone(),
            Input::Headword | Input::Features => l.lemma.clone(),
        };
        let inherent: Vec<(u8, u8)> = l
            .inherent
            .iter()
            .enumerate()
            .filter(|&(_, &v)| input == Input::Features && v != u8::MAX)
            .filter_map(|(f, &v)| Some((u8::try_from(f).ok()?, v)))
            .collect();
        let parts: Vec<usize> = match input {
            Input::Citation(parts) => parts.to_vec(),
            Input::Given(k) => match given_cells(l, k, grammar.lemma_cell) {
                Some(cells) => cells,
                None => return out,
            },
            _ => Vec::new(),
        };
        let given: Vec<(usize, &str)> = parts
            .iter()
            .filter_map(|&c| Some((c, l.cells.get(c)?.first()?.as_str())))
            .collect();
        let known = flexion::Known {
            forms: &given,
            inherent: &inherent,
            full: !matches!(input, Input::Headword | Input::Features),
        };
        let has = grammar
            .readings(&typed, &known)
            .first()
            .and_then(|r| seen.get(r.value.index()));
        let table = grammar.table(&typed, &known);
        for (cell, forms) in l.cells.iter().enumerate() {
            let Some(want) = forms.first().filter(|want| {
                !parts.contains(&cell)
                    && scored(cell)
                    && !typed_back(orthography, grammar.lemma_cell, cell, want, &typed)
            }) else {
                continue;
            };
            let candidates = table.get(cell).map_or(&[][..], Vec::as_slice);
            let same = |form: &str| same_form(orthography, form, want);
            let probability: f32 = candidates
                .iter()
                .filter(|c| same(&c.form))
                .map(|c| c.probability)
                .sum();
            let listing = listing_bits(want);
            let bits = if probability > 0.0 {
                (-f64::from(probability).log2()).min(listing)
            } else {
                listing
            };
            let first = candidates.first();
            let rank = candidates.iter().position(|c| same(&c.form));
            let wanted = orthography.read(want);
            let answer = first.map(|c| orthography.read(&c.form));
            let letters = answer.as_ref().is_some_and(|a| a.letters == wanted.letters);
            let compatible_letters = answer.as_ref().is_some_and(|a| {
                a.letters == wanted.letters || unstated.compatible(&wanted.letters, &a.letters)
            });
            let any =
                first.is_some_and(|c| forms.iter().any(|f| same_form(orthography, &c.form, f)));
            let error = match first {
                _ if rank == Some(0) => None,
                None => Some(Kind::Gap),
                Some(_) if any => Some(Kind::Variant),
                Some(c)
                    if alike.get(l.key.as_str()).is_some_and(|ws| {
                        ws.iter().any(|w| {
                            !std::ptr::eq(*w, *l)
                                && w.cells
                                    .get(cell)
                                    .and_then(|f| f.first())
                                    .is_some_and(|f| same_form(orthography, &c.form, f))
                        })
                    }) =>
                {
                    Some(Kind::Homograph)
                }
                Some(c) => Some(measure::kind(
                    orthography,
                    want,
                    &c.form,
                    rank.is_some(),
                    stem,
                )),
            };
            out.push(Outcome {
                word,
                cell,
                rank,
                top: first.map_or(0.0, |c| c.probability),
                probability,
                letters,
                seen: has.is_none_or(|h| h.get(cell).copied().unwrap_or(false)),
                lemma: grammar.lemma_cell == Some(cell),
                any,
                bits,
                error,
                // A phrase's words carry their own (`μεμιασμένος ὦ`).
                accentless: first.is_some_and(|c| {
                    !c.form.contains(' ') && answer.as_ref().is_some_and(|a| a.stress.is_none())
                }) && wanted.stress.is_some(),
                compatible: compatible_letters
                    && answer.as_ref().is_some_and(|a| a.stress == wanted.stress),
                compatible_letters,
            });
        }
        out
    });
    each.into_iter().flatten().collect()
}

/// The fewest forms a block of cells is scored with to count in
/// [`by_block`]'s mean: fewer is a handful of words, named apart.
pub const BLOCK_FORMS: usize = 30;

/// Top-1 by block of cells (`blocks`, the part's groups of cells: a
/// Turkish verb's negative abilitative forms, its participles): each
/// block's score, and their mean over the blocks with at least
/// [`BLOCK_FORMS`] forms (none: no such block), so that a block of many
/// cells can't outweigh the rest.
pub fn by_block(
    outcomes: &[Outcome],
    blocks: &[std::ops::Range<usize>],
) -> (Option<f64>, Vec<Score>) {
    let scores: Vec<Score> = blocks
        .iter()
        .map(|b| {
            let mine: Vec<Outcome> = outcomes
                .iter()
                .filter(|o| b.contains(&o.cell))
                .cloned()
                .collect();
            Score::of(&mine)
        })
        .collect();
    let scored: Vec<f64> = scores
        .iter()
        .filter(|s| s.forms >= BLOCK_FORMS)
        .map(Score::accuracy)
        .collect();
    #[allow(clippy::cast_precision_loss)]
    let mean = (!scored.is_empty()).then(|| scored.iter().sum::<f64>() / scored.len() as f64);
    (mean, scores)
}

/// How complete `grammar`'s tables are for `lexemes` typed as their
/// headwords: every cell, the dictionary's or not, `(cells, with a form)`,
/// and the first few gaps (`word cell`). A cell without a form is a gap: no
/// way of building it gave one.
pub fn completion<P: Label, A: Label, F: Label>(
    grammar: &flexion::Grammar<'_, P, A, F>,
    lexemes: &[&Lexeme],
) -> (usize, usize, Vec<String>) {
    let known = flexion::Known::default();
    let mut gaps = Vec::new();
    let (cells, filled) = lexemes.iter().fold((0, 0), |(cells, filled), l| {
        let table = grammar.table(&l.lemma, &known);
        for cell in (0..grammar.cells).filter(|&c| table.get(c).is_none_or(Vec::is_empty)) {
            if gaps.len() < 6 {
                gaps.push(format!("{} {cell}", l.lemma));
            }
        }
        let with = table.iter().filter(|c| !c.is_empty()).count();
        (cells + grammar.cells, filled + with.min(grammar.cells))
    });
    (cells, filled, gaps)
}

/// One held-out form the floor was asked for: the word (its place among
/// those scored), the cell, whether the floor wrote its letters, and
/// whether they are compatible with the length the dictionary states (see
/// [`TableLength`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FloorOutcome {
    pub word: usize,
    pub cell: usize,
    pub right: bool,
    pub compatible: bool,
}

/// Each form of `test` (typed as their headwords) the shared task's
/// non-neural baseline (see [`crate::baseline`]) is asked for, learned from
/// `train`'s headwords and forms, and whether it is right: the cells
/// [`outcomes`] scores from the headword. Letters only: stress and accents
/// aside. `stated`: as [`outcomes`] takes it.
pub fn floor(
    orthography: &flexion::Orthography<'_>,
    stated: &str,
    train: &[&Lexeme],
    test: &[&Lexeme],
    lemma_cell: Option<usize>,
) -> Vec<FloorOutcome> {
    let letters = |s: &str| orthography.read(s).letters;
    let lemmas: Vec<String> = train.iter().map(|l| letters(&l.lemma)).collect();
    let tests: Vec<String> = test.iter().map(|l| letters(&l.lemma)).collect();
    let unstated: Vec<TableLength<'_>> = test
        .iter()
        .map(|l| TableLength::of(orthography, stated, word_forms(l)))
        .collect();
    let cells = train.iter().chain(test).map(|l| l.cells.len()).max();
    let mut out = Vec::new();
    // A cell at a time: its rules alone are small.
    for cell in 0..cells.unwrap_or(0) {
        let forms: Vec<(&str, String)> = train
            .iter()
            .zip(&lemmas)
            .filter_map(|(l, lemma)| Some((lemma.as_str(), letters(l.cells.get(cell)?.first()?))))
            .collect();
        let baseline =
            crate::baseline::Baseline::learn(forms.iter().map(|(l, f)| (*l, f.as_str())));
        let asked: Vec<(usize, &&Lexeme, &String, &TableLength<'_>)> = test
            .iter()
            .zip(&tests)
            .zip(&unstated)
            .enumerate()
            .map(|(word, ((l, lemma), unstated))| (word, l, lemma, unstated))
            .collect();
        let found = crate::par::map(&asked, |&(word, l, lemma, unstated)| {
            let want = l
                .cells
                .get(cell)
                .and_then(|forms| forms.first())
                .filter(|want| !typed_back(orthography, lemma_cell, cell, want, &l.lemma))?;
            let (wrote, want) = (baseline.inflect(lemma), letters(want));
            let right = wrote == want;
            Some(FloorOutcome {
                word,
                cell,
                right,
                compatible: right || unstated.compatible(&want, &wrote),
            })
        });
        out.extend(found.into_iter().flatten());
    }
    out.sort_unstable_by_key(|o| (o.word, o.cell));
    out
}

/// The grammar (its top candidate's letters, as the floor is scored) and
/// the floor paired word by word on the forms both were asked for, of the
/// cells `cell` accepts: `a` the floor's right forms, `b` the grammar's;
/// with `compatible`, those compatible with the length the dictionary
/// states, for both.
pub fn against_floor(
    outcomes: &[Outcome],
    floor: &[FloorOutcome],
    cell: impl Fn(usize) -> bool,
    compatible: bool,
) -> Vec<crate::measure::Paired> {
    let floor: crate::HashMap<(usize, usize), bool> = floor
        .iter()
        .map(|f| {
            (
                (f.word, f.cell),
                if compatible { f.compatible } else { f.right },
            )
        })
        .collect();
    let mut words: Vec<crate::measure::Paired> = Vec::new();
    let mut last = None;
    for o in outcomes.iter().filter(|o| cell(o.cell)) {
        let Some(&right) = floor.get(&(o.word, o.cell)) else {
            continue;
        };
        if last != Some(o.word) {
            words.push(crate::measure::Paired::default());
            last = Some(o.word);
        }
        if let Some(w) = words.last_mut() {
            w.forms += 1;
            w.a += usize::from(right);
            w.b += usize::from(if compatible {
                o.compatible_letters
            } else {
                o.letters
            });
        }
    }
    words
}

/// The share of `outcomes`' forms the floor gets right, of all and of the
/// cells the grammar's paradigm has no training form in.
pub fn floor_shares(outcomes: &[Outcome], floor: &[FloorOutcome]) -> (f64, f64) {
    let unseen: crate::HashSet<(usize, usize)> = outcomes
        .iter()
        .filter(|o| !o.seen)
        .map(|o| (o.word, o.cell))
        .collect();
    let (mut all, mut off) = ((0, 0), (0, 0));
    for f in floor {
        all = (all.0 + 1, all.1 + usize::from(f.right));
        if unseen.contains(&(f.word, f.cell)) {
            off = (off.0 + 1, off.1 + usize::from(f.right));
        }
    }
    (ratio(all.1, all.0), ratio(off.1, off.0))
}

/// The share of the floor's forms compatible with the length the
/// dictionary states (see [`FloorOutcome::compatible`]).
pub fn floor_compatible(floor: &[FloorOutcome]) -> f64 {
    ratio(floor.iter().filter(|f| f.compatible).count(), floor.len())
}

/// The share of `outcomes`' forms any grammar could get right: words typed
/// alike (homographs) get one answer per cell, so of each typed word's
/// forms in a cell, only the commonest can be.
pub fn ceiling(
    orthography: &flexion::Orthography<'_>,
    lexemes: &[&Lexeme],
    outcomes: &[Outcome],
) -> f64 {
    let mut counts: crate::HashMap<(String, usize), crate::HashMap<String, usize>> =
        crate::HashMap::default();
    for o in outcomes {
        let Some(l) = lexemes.get(o.word) else {
            continue;
        };
        let Some(want) = l.cells.get(o.cell).and_then(|f| f.first()) else {
            continue;
        };
        let read = orthography.read(want);
        let typed = orthography.read(&l.lemma).letters;
        *counts
            .entry((typed, o.cell))
            .or_default()
            .entry(format!("{}\u{1f}{:?}", read.letters, read.stress))
            .or_default() += 1;
    }
    let best: usize = counts
        .values()
        .map(|forms| forms.values().copied().max().unwrap_or(0))
        .sum();
    ratio(best, outcomes.len())
}

/// Per word, `(forms, right)` of `outcomes` (in word order).
pub fn by_word(outcomes: &[Outcome]) -> Vec<(usize, usize)> {
    let mut words: Vec<(usize, usize)> = Vec::new();
    let mut last = None;
    for o in outcomes {
        if last != Some(o.word) {
            words.push((0, 0));
            last = Some(o.word);
        }
        if let Some(w) = words.last_mut() {
            w.0 += 1;
            w.1 += usize::from(o.right());
        }
    }
    words
}

/// The sharpness and the trust (see [`flexion::Grammar::sharpness`],
/// [`flexion::Trust`]) that make `grammar`'s stated probabilities fit how
/// often its answers are right on `words` (held out, typed as their
/// headwords): the power, of a grid from 0.5 to 4, by calibration error;
/// then each status's trust, where at least [`TRUSTED`] of the forms have
/// an answer of that status first (else all of it).
pub fn fit_sharpness<P: Label + Sync, A: Label + Sync, F: Label + Sync>(
    grammar: &flexion::Grammar<'_, P, A, F>,
    orthography: &flexion::Orthography<'_>,
    words: &[&Lexeme],
) -> (f32, flexion::Trust) {
    // Every scored form's answers: (probability, right, status) each.
    // From the headword, as the grammar is most often asked (and scored).
    let each = crate::par::map(words, |l| {
        let mut forms: Vec<Vec<(f32, bool, flexion::Status)>> = Vec::new();
        let table = grammar.table(&l.lemma, &flexion::Known::default());
        for (cell, fs) in l.cells.iter().enumerate() {
            let Some(want) = fs.first() else {
                continue;
            };
            let c = table.get(cell).map_or(&[][..], Vec::as_slice);
            let scored = c.iter().map(|c| {
                // Undo the grammar's own trust: fit on what it was given.
                let kept = grammar.trust.of(c.status);
                let p = if kept > 0.0 {
                    c.probability / kept
                } else {
                    c.probability
                };
                (p, same_form(orthography, &c.form, want), c.status)
            });
            forms.push(scored.collect());
        }
        forms
    });
    let forms: Vec<Vec<(f32, bool, flexion::Status)>> = each.into_iter().flatten().collect();
    // The power: of a grid, the one whose answers' stated probabilities are
    // nearest how often they are right (calibration error).
    let sharpened = |f: &[(f32, bool, flexion::Status)], power: f32| -> Vec<f32> {
        let total: f32 = f.iter().map(|x| x.0).sum();
        let sharp: f32 = f.iter().map(|x| x.0.powf(power)).sum();
        let scale = if sharp > 0.0 { total / sharp } else { 0.0 };
        f.iter().map(|x| x.0.powf(power) * scale).collect()
    };
    let error = |power: f32| {
        let mut s = Score {
            forms: forms.len(),
            ..Score::default()
        };
        for f in &forms {
            let (Some(&(_, right, _)), Some(&p)) = (f.first(), sharpened(f, power).first()) else {
                continue;
            };
            s.calibrate(f64::from(p), right);
        }
        s.calibration_error()
    };
    let grid = [0.5f32, 0.75, 1.0, 1.25, 1.5, 1.75, 2.0, 2.5, 3.0, 3.5, 4.0];
    let power = grid
        .into_iter()
        .min_by(|&a, &b| error(a).total_cmp(&error(b)))
        .unwrap_or(1.0);
    // Each status's trust: where its first answers are right less often
    // than they say, as often as they are (at least 0.3); never more.
    let mut trust = flexion::Trust::FULL;
    for status in [
        flexion::Status::Regular,
        flexion::Status::Unsettled,
        flexion::Status::Unattested,
    ] {
        let (mut n, mut said, mut right) = (0usize, 0.0f64, 0.0f64);
        for f in forms
            .iter()
            .filter(|f| f.first().is_some_and(|x| x.2 == status))
        {
            let (Some(&(_, ok, _)), Some(&p)) = (f.first(), sharpened(f, power).first()) else {
                continue;
            };
            n += 1;
            said += f64::from(p);
            right += f64::from(u8::from(ok));
        }
        if n < TRUSTED || said <= 0.0 {
            continue;
        }
        // A share; f32 is plenty.
        #[allow(clippy::cast_possible_truncation)]
        let w = (right / said).clamp(0.3, 1.0) as f32;
        match status {
            flexion::Status::Regular => trust.regular = w,
            flexion::Status::Unsettled => trust.unsettled = w,
            _ => trust.unattested = w,
        }
    }
    (power, trust)
}

/// The fewest held-out forms whose first answer has a status for that
/// status's trust to be fitted.
pub const TRUSTED: usize = 100;

/// One held-out word typed in full, against the paradigm that builds the
/// most of its forms (the oracle: it reads the answers, so it is a
/// diagnostic, never a grammar's).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Oracle {
    /// The word's place among those scored.
    pub word: usize,
    /// Forms scored (the lemma's aside where it is what was typed).
    pub forms: usize,
    /// Forms whose letters the grammar's first candidate gets right.
    pub grammar: usize,
    /// The paradigms that build the most of them (tied, all of them), and
    /// how many; the grammar's likeliest paradigm.
    pub best: Vec<usize>,
    pub oracle: usize,
    pub chosen: Option<usize>,
    /// Per paradigm, how many of the forms it builds.
    pub built: Vec<usize>,
}

/// Held-out words and cells scored for [`oracle`], at most: every paradigm
/// is tried on each word (at most 1,000 words).
pub const ORACLE_CELLS: usize = 100_000;

/// [`Oracle`] of an even sample of `lexemes` (at most [`ORACLE_CELLS`] of
/// words and cells), typed in full: letters only, as the floor.
pub fn oracle<P: Label + Sync, A: Label + Sync, F: Label + Sync>(
    grammar: &flexion::Grammar<'_, P, A, F>,
    lexemes: &[&Lexeme],
) -> Vec<Oracle> {
    let o = &grammar.orthography;
    let words = (ORACLE_CELLS / grammar.cells.max(1)).clamp(1, 1000);
    let step = lexemes.len().div_ceil(words).max(1);
    let sample: Vec<(usize, &Lexeme)> = lexemes.iter().copied().enumerate().step_by(step).collect();
    let known = flexion::Known {
        full: true,
        ..flexion::Known::default()
    };
    crate::par::map(&sample, |&(word, l)| {
        let wanted: Vec<Option<String>> = l
            .cells
            .iter()
            .enumerate()
            .map(|(cell, forms)| {
                let want = forms.first()?;
                (!typed_back(o, grammar.lemma_cell, cell, want, &l.full))
                    .then(|| o.read(want).letters)
            })
            .collect();
        let forms = wanted.iter().flatten().count();
        let right = |built: &[Option<String>]| {
            wanted
                .iter()
                .zip(built)
                .filter(|(w, b)| w.is_some() && w == b)
                .count()
        };
        let table: Vec<Option<String>> = grammar
            .table(&l.full, &known)
            .iter()
            .map(|c| c.first().map(|c| o.read(&c.form).letters))
            .collect();
        let first = |forms: Vec<Vec<String>>| -> Vec<Option<String>> {
            forms.into_iter().map(|f| f.into_iter().next()).collect()
        };
        let counts: Vec<usize> = (0..grammar.paradigms.len())
            .map(|p| right(&first(grammar.built_as(&l.full, p))))
            .collect();
        let oracle = counts.iter().copied().max().unwrap_or(0);
        // Every paradigm building that many: the best, tied.
        let best: Vec<usize> = (0..counts.len())
            .filter(|&p| oracle > 0 && counts.get(p) == Some(&oracle))
            .collect();
        Oracle {
            word,
            forms,
            grammar: right(&table),
            best,
            oracle,
            chosen: grammar
                .readings(&l.full, &known)
                .first()
                .map(|r| r.value.index()),
            built: counts,
        }
    })
}

/// Of the words the paradigm choice lists (its exceptions), those the rules
/// alone give a paradigm that builds every one of the word's cells as the
/// listed one does: listed for nothing. `(tied, listed)`.
pub fn ties<P: Label + Sync, A: Label + Sync, F: Label + Sync>(
    grammar: &flexion::Grammar<'_, P, A, F>,
) -> (usize, usize) {
    let mut rules = grammar.paradigm.clone();
    rules.exceptions = std::borrow::Cow::Borrowed(&[]);
    let listed: Vec<(&str, usize)> = grammar
        .paradigm
        .exceptions
        .iter()
        .map(|(w, p)| (w.trim_start_matches(flexion::WORD_END), p.index()))
        .collect();
    let tied = crate::par::map(&listed, |&(key, p)| {
        let ruled = rules
            .read(
                &format!("{}{key}", flexion::WORD_END),
                grammar.smoothing,
                |_| true,
            )
            .first()
            .map(|r| r.value.index());
        ruled.is_some_and(|q| q != p && grammar.built_as(key, q) == grammar.built_as(key, p))
    });
    (tied.iter().filter(|&&t| t).count(), listed.len())
}

/// The outcomes whose right answer the dictionary writes as a phrase (with
/// a space: the Latin perfect passive, `amātus sum`).
pub fn phrases(outcomes: &[Outcome], lexemes: &[&Lexeme]) -> Vec<Outcome> {
    outcomes
        .iter()
        .filter(|o| {
            lexemes
                .get(o.word)
                .and_then(|l| l.cells.get(o.cell)?.first())
                .is_some_and(|f| f.contains(' '))
        })
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_form_is_scored_as_the_orthography_reads_it() {
        // Russian as shipped: the dictionary writes stress, the grammar
        // doesn't, and the orthography drops the acute when it reads.
        let russian = flexion::Orthography {
            lowercase: true,
            vowels: "аеиоуыэюяё",
            stress_marks: "",
            stressed: &[],
            implicit: "",
            dropped: "\u{301}\u{300}",
            mark_monosyllables: false,
            folded: &[('ё', 'е')],
            prosody: None,
            initial: "",
        };
        let listed = |forms: &[&str]| forms.iter().map(ToString::to_string).collect::<Vec<_>>();
        assert_eq!(
            verdict(&russian, Some("оный"), &listed(&["о\u{301}ный"])),
            Verdict::Right
        );
        assert_eq!(
            verdict(
                &russian,
                Some("ко\u{301}нь"),
                &listed(&["кон", "ко\u{301}нь"])
            ),
            Verdict::Listed
        );
        assert_eq!(verdict(&russian, None, &listed(&["конь"])), Verdict::Gap);
    }

    #[test]
    fn a_length_mark_is_compatible_where_the_table_never_states_otherwise() {
        let greek = flexion::Orthography {
            lowercase: true,
            vowels: "αεηιουω",
            stress_marks: "",
            stressed: &[],
            implicit: "",
            dropped: "",
            mark_monosyllables: false,
            folded: &[],
            prosody: None,
            initial: "",
        };
        let stated = "\u{304}\u{306}";
        // A table typed with a bare stem: `βαλ`'s α is unstated.
        let bare = TableLength::of(&greek, stated, ["προβαλλω", "προβαλλομεν", "προβαλλειν"]);
        assert!(bare.compatible("προβαλοισθε", "προβᾰλοισθε"));
        assert!(bare.compatible("προβαλοισθε", "προβᾱλοισθε"));
        // Other letters must still be right.
        assert!(!bare.compatible("προβαλοισθε", "προβᾰλοισθα"));
        // The table writes `γᾰπ` short elsewhere, after the augment too: the
        // pluperfect's bare one is short, and long contradicts it.
        let marked = TableLength::of(&greek, stated, ["ἀγᾰπαω", "ἠγᾰπων", "ἠγαπημεθᾰ"]);
        assert!(marked.compatible("ἠγαπημεθᾰ", "ἠγᾰπημεθᾰ"));
        assert!(!marked.compatible("ἠγαπημεθᾰ", "ἠγᾱπημεθᾰ"));
        // A spelling that doesn't mark length both ways states it bare.
        let latin = TableLength::of(&greek, "", ["rosa", "rosam"]);
        assert!(!latin.compatible("rosa", "rōsa"));
    }
}
