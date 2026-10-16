//! Measuring grammars beyond held-out top-1, for every language alike:
//! learning curves and thinned tables, cells held out of training, words
//! built on a training word by a prefix, paired comparison of two runs, how
//! two groupings of words agree, and what kind of error a wrong answer is.
//! Nothing here is read by the learner.

use crate::eval::hash;
use crate::extract::Lexeme;
use unicode_normalization::UnicodeNormalization;

/// A word's letters with every mark taken off and in lowercase: what two
/// forms must share to be "the same letters" when comparing stems.
pub fn plain(s: &str) -> String {
    s.nfd()
        .filter(|c| !unicode_normalization::char::is_combining_mark(*c))
        .flat_map(char::to_lowercase)
        .collect()
}

/// The length, in letters, of the longest common beginning of `a` and `b`.
pub fn common_start(a: &str, b: &str) -> usize {
    a.chars().zip(b.chars()).take_while(|(x, y)| x == y).count()
}

/// Whether a word is kept when training on `percent` of the words: by a
/// hash of its key, so the same words are kept every run, and those kept at
/// 25% are kept at 50% too.
pub fn kept(key: &str, percent: u8) -> bool {
    hash(&format!("share\u{1f}{key}")) % 100 < u64::from(percent)
}

/// Take about `percent` of the cells out of a word's table (never the
/// lemma's), by a hash of the word and the cell.
pub fn thin(l: &mut Lexeme, percent: u8, lemma_cell: Option<usize>) {
    for (cell, forms) in l.cells.iter_mut().enumerate() {
        let h = hash(&format!("drop\u{1f}{}\u{1f}{cell}", l.key));
        if Some(cell) != lemma_cell && h % 100 < u64::from(percent) {
            forms.clear();
        }
    }
}

/// A word's stem as its own forms show it: the longest beginning of its
/// lemma that at least half of its other forms (marks aside) begin with.
pub fn word_stem(lemma: &str, forms: &[String]) -> usize {
    let lemma = plain(lemma);
    let mut shared: Vec<usize> = forms
        .iter()
        .map(|f| common_start(&lemma, &plain(f)))
        .collect();
    shared.sort_unstable_by(|a, b| b.cmp(a));
    let half = shared.len().div_ceil(2);
    half.checked_sub(1)
        .and_then(|i| shared.get(i))
        .copied()
        .unwrap_or(0)
}

/// A group of cells whose forms share a stem other than the lemma's: in
/// the training words where one of two of them leaves the lemma's stem
/// (`regō`: `rēxī`, `rēxeram`), mostly both do, and their forms begin
/// alike for longer than either does with the lemma.
#[derive(Debug, Clone, PartialEq)]
pub struct StemGroup {
    pub cells: Vec<usize>,
    /// How often, on average over its cells, a word's form doesn't begin
    /// with the word's stem (see [`word_stem`]): how far it is from the
    /// lemma's stem.
    pub off: f64,
    /// Training forms in its cells.
    pub forms: usize,
}

impl StemGroup {
    /// How many of its cells are, on average, off the lemma's stem: what
    /// the default probe maximizes.
    pub fn weight(&self) -> f64 {
        #[allow(clippy::cast_precision_loss)]
        let n = self.cells.len() as f64;
        n * self.off
    }
}

/// A word's forms per cell, for stem grouping: the form, plain; whether it
/// is off the word's stem (see [`word_stem`]); and how long a beginning it
/// shares with the lemma.
type Row = Vec<Option<(String, bool, usize)>>;

/// The training words a stem grouping is computed on, at most.
const SAMPLE: usize = 1000;
/// Two cells share a stem when, of the words where either leaves the
/// lemma's stem, at least this share have both leave it together.
const SAME_STEM: f64 = 0.5;

/// The cells of a part of speech grouped by shared stems (see
/// [`StemGroup`]), the lemma's cell aside, largest weight first. Computed
/// from a fixed sample of `lexemes` (by a hash of the key); cells with
/// forms in fewer than 5% of the sampled words are left out.
pub fn stem_groups(lexemes: &[Lexeme], cells: usize, lemma_cell: Option<usize>) -> Vec<StemGroup> {
    let mut sample: Vec<&Lexeme> = lexemes.iter().filter(|l| l.source == 0).collect();
    sample.sort_by_key(|l| (hash(&l.key), l.order));
    sample.truncate(SAMPLE);
    // Per word and cell: its form, plain; whether it is off the word's
    // stem; and how long a beginning it shares with the lemma.
    let mut words: Vec<Row> = Vec::new();
    for l in &sample {
        let lemma = lemma_cell
            .and_then(|c| l.cells.get(c))
            .and_then(|f| f.first())
            .unwrap_or(&l.full);
        let firsts: Vec<String> = l
            .cells
            .iter()
            .enumerate()
            .filter(|&(c, _)| Some(c) != lemma_cell)
            .filter_map(|(_, f)| f.first().cloned())
            .collect();
        let stem = word_stem(lemma, &firsts);
        let lemma = plain(lemma);
        let row = (0..cells)
            .map(|c| {
                let f = l.cells.get(c)?.first()?;
                let p = plain(f);
                let shared = common_start(&lemma, &p);
                Some((p, shared < stem, shared))
            })
            .collect();
        words.push(row);
    }
    let has = |c: usize| {
        words
            .iter()
            .filter(|w| matches!(w.get(c), Some(Some(_))))
            .count()
    };
    let least = (words.len() / 20).max(2);
    let mut groups: Vec<Vec<usize>> = Vec::new();
    for c in (0..cells).filter(|&c| Some(c) != lemma_cell && has(c) >= least) {
        let joins = groups
            .iter()
            .position(|g| g.first().is_some_and(|&r| same_stem(&words, r, c)));
        match joins.and_then(|i| groups.get_mut(i)) {
            Some(g) => g.push(c),
            None => groups.push(vec![c]),
        }
    }
    let mut out: Vec<StemGroup> = groups
        .into_iter()
        .map(|cells| {
            let (mut n, mut off) = (0usize, 0usize);
            for w in &words {
                for &c in &cells {
                    if let Some(Some((_, o, _))) = w.get(c) {
                        n += 1;
                        off += usize::from(*o);
                    }
                }
            }
            #[allow(clippy::cast_precision_loss)]
            let off = off as f64 / n.max(1) as f64;
            StemGroup {
                cells,
                off,
                forms: n,
            }
        })
        .collect();
    out.sort_by(|a, b| {
        b.weight()
            .total_cmp(&a.weight())
            .then(b.cells.len().cmp(&a.cells.len()))
            .then(a.cells.cmp(&b.cells))
    });
    out
}

/// Whether cells `a` and `b` share a stem other than the lemma's: of the
/// words where either form is off the word's stem, at least [`SAME_STEM`]
/// (and at least ten words) have both off, beginning alike for longer than
/// either begins like the lemma.
fn same_stem(words: &[Row], a: usize, b: usize) -> bool {
    let (mut either, mut both) = (0usize, 0usize);
    for w in words {
        let (Some(Some((x, off_x, lx))), Some(Some((y, off_y, ly)))) = (w.get(a), w.get(b)) else {
            continue;
        };
        if *off_x || *off_y {
            either += 1;
            both += usize::from(*off_x && *off_y && common_start(x, y) > *lx.max(ly));
        }
    }
    #[allow(clippy::cast_precision_loss)]
    let share = both as f64 / either.max(1) as f64;
    both >= 10 && share >= SAME_STEM
}

/// The default held-out-cells probe, from the stem group of largest weight
/// with at least two cells (see [`stem_groups`]): with `all`, the whole
/// group less its cell with the most training forms, which stays in
/// training as the stem's witness; else every other cell of the group (the
/// second, fourth, …), the rest staying in training, so that what the held
/// cells share with the kept ones (the stem, endings by person and number)
/// can be learned and what they don't can't (Kodner, Khalifa & Payne 2023
/// keep 2 of the 6 Spanish future cells). Returns `(held, kept)`.
pub fn default_probe(
    groups: &[StemGroup],
    lexemes: &[Lexeme],
    all: bool,
) -> Option<(Vec<usize>, Vec<usize>)> {
    let g = groups.iter().find(|g| g.cells.len() >= 2)?;
    if !all {
        let half = |parity: usize| -> Vec<usize> {
            g.cells
                .iter()
                .enumerate()
                .filter(|(i, _)| i % 2 == parity)
                .map(|(_, &c)| c)
                .collect()
        };
        return Some((half(1), half(0)));
    }
    let count = |c: usize| {
        lexemes
            .iter()
            .filter(|l| l.cells.get(c).is_some_and(|f| !f.is_empty()))
            .count()
    };
    let witness = g
        .cells
        .iter()
        .copied()
        .max_by(|&a, &b| count(a).cmp(&count(b)).then(b.cmp(&a)))?;
    let held = g.cells.iter().copied().filter(|&c| c != witness).collect();
    Some((held, vec![witness]))
}

/// Whether a word (its key) is built on a training word: it is some
/// training word's key with letters in front, either one of the grammar's
/// learned `prefixes` (the rest at least two letters), or at least
/// [`flexion::FRONT_LETTERS`] letters (a compound's front) before a
/// training word of at least four.
pub fn prefixed(key: &str, training: &crate::HashSet<&str>, prefixes: &[&str]) -> bool {
    key.char_indices().skip(1).any(|(i, _)| {
        let (front, rest) = key.split_at(i);
        let (nf, nr) = (front.chars().count(), rest.chars().count());
        training.contains(rest)
            && ((prefixes.contains(&front) && nr >= 2) || (nf >= flexion::FRONT_LETTERS && nr >= 4))
    })
}

/// One word's result in two runs: forms right in run a and in run b, of the
/// same forms.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Paired {
    pub forms: usize,
    pub a: usize,
    pub b: usize,
}

/// A paired comparison of two runs on the same words.
#[derive(Debug, Clone, PartialEq)]
pub struct Comparison {
    pub words: usize,
    pub forms: usize,
    /// Top-1 of each run, over the shared forms.
    pub a: f64,
    pub b: f64,
    /// The 95% interval of b − a, by resampling words.
    pub low: f64,
    pub high: f64,
    /// Words b does better on, and worse.
    pub better: usize,
    pub worse: usize,
    /// The sign test's two-sided p-value over those words.
    pub p: f64,
}

/// Resamples in [`compare`]'s bootstrap.
const RESAMPLES: usize = 2000;

/// Compare two runs word by word: the paired bootstrap 95% interval of the
/// top-1 difference, resampling words (not forms) with a fixed seed, and a
/// sign test over the words whose share of right forms differs.
pub fn compare(words: &[Paired]) -> Comparison {
    let forms: usize = words.iter().map(|w| w.forms).sum();
    let ra: usize = words.iter().map(|w| w.a).sum();
    let rb: usize = words.iter().map(|w| w.b).sum();
    let share = |r: usize, n: usize| {
        #[allow(clippy::cast_precision_loss)]
        let s = r as f64 / n.max(1) as f64;
        s
    };
    let mut seed: u64 = 0x9e37_79b9_7f4a_7c15;
    let mut next = move || {
        // xorshift64*
        seed ^= seed >> 12;
        seed ^= seed << 25;
        seed ^= seed >> 27;
        seed.wrapping_mul(0x2545_f491_4f6c_dd1d)
    };
    let mut diffs = Vec::with_capacity(RESAMPLES);
    if !words.is_empty() {
        let n = words.len() as u64;
        for _ in 0..RESAMPLES {
            let (mut f, mut d) = (0usize, 0i64);
            for _ in 0..words.len() {
                let i = usize::try_from(next() % n).unwrap_or(0);
                if let Some(w) = words.get(i) {
                    f += w.forms;
                    d += i64::try_from(w.b).unwrap_or(0) - i64::try_from(w.a).unwrap_or(0);
                }
            }
            #[allow(clippy::cast_precision_loss)]
            diffs.push(d as f64 / f.max(1) as f64);
        }
    }
    diffs.sort_by(f64::total_cmp);
    let at = |q: f64| {
        #[allow(
            clippy::cast_precision_loss,
            clippy::cast_possible_truncation,
            clippy::cast_sign_loss
        )]
        let i = ((diffs.len().saturating_sub(1)) as f64 * q).round() as usize;
        diffs.get(i).copied().unwrap_or(0.0)
    };
    // A word is better in b when its share right is higher (same forms).
    let better = words.iter().filter(|w| w.b > w.a).count();
    let worse = words.iter().filter(|w| w.b < w.a).count();
    Comparison {
        words: words.len(),
        forms,
        a: share(ra, forms),
        b: share(rb, forms),
        low: at(0.025),
        high: at(0.975),
        better,
        worse,
        p: sign_test(better, worse),
    }
}

/// The 95% interval of one run's top-1, resampling words as [`compare`]
/// does: `words` is `(forms, right)` per word.
pub fn interval(words: &[(usize, usize)]) -> (f64, f64) {
    let paired: Vec<Paired> = words
        .iter()
        .map(|&(forms, right)| Paired {
            forms,
            a: 0,
            b: right,
        })
        .collect();
    let c = compare(&paired);
    (c.low, c.high)
}

/// The two-sided exact sign test: the chance of a split at least as uneven
/// as `x` to `y` if each word were as likely to go either way.
pub fn sign_test(x: usize, y: usize) -> f64 {
    let n = x + y;
    if n == 0 {
        return 1.0;
    }
    let k = x.min(y);
    // ln n! for 0..=n.
    let mut ln_fact = Vec::with_capacity(n + 1);
    let mut acc = 0.0f64;
    ln_fact.push(0.0);
    for i in 1..=n {
        #[allow(clippy::cast_precision_loss)]
        let f = i as f64;
        acc += f.ln();
        ln_fact.push(acc);
    }
    let lf = |i: usize| ln_fact.get(i).copied().unwrap_or(0.0);
    #[allow(clippy::cast_precision_loss)]
    let half = n as f64 * std::f64::consts::LN_2;
    let tail: f64 = (0..=k)
        .map(|i| (lf(n) - lf(i) - lf(n - i) - half).exp())
        .sum();
    (2.0 * tail).min(1.0)
}

/// How two groupings of the same items agree: the adjusted Rand index (1:
/// the same grouping; about 0: no better than chance), purity (each group
/// of `a` counted by its commonest label of `b`) and inverse purity (the
/// reverse), and how many groups each has.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Agreement {
    pub items: usize,
    pub groups_a: usize,
    pub groups_b: usize,
    pub rand: f64,
    pub purity: f64,
    pub inverse_purity: f64,
}

/// The agreement of grouping `a` with grouping `b` over `pairs` (`(a's
/// label, b's label)` per item).
pub fn agreement<A: Ord + Clone, B: Ord + Clone>(pairs: &[(A, B)]) -> Agreement {
    use std::collections::BTreeMap;
    let mut table: BTreeMap<(A, B), usize> = BTreeMap::new();
    let mut ra: BTreeMap<A, usize> = BTreeMap::new();
    let mut rb: BTreeMap<B, usize> = BTreeMap::new();
    for (x, y) in pairs {
        *table.entry((x.clone(), y.clone())).or_default() += 1;
        *ra.entry(x.clone()).or_default() += 1;
        *rb.entry(y.clone()).or_default() += 1;
    }
    #[allow(clippy::cast_precision_loss)]
    let pairs_of = |n: usize| (n as f64) * (n as f64 - 1.0) / 2.0;
    let n = pairs.len();
    let index: f64 = table.values().map(|&v| pairs_of(v)).sum();
    let sa: f64 = ra.values().map(|&v| pairs_of(v)).sum();
    let sb: f64 = rb.values().map(|&v| pairs_of(v)).sum();
    let expected = if n > 1 { sa * sb / pairs_of(n) } else { 0.0 };
    let max = (sa + sb) / 2.0;
    let rand = if (max - expected).abs() > f64::EPSILON {
        (index - expected) / (max - expected)
    } else {
        1.0
    };
    let mut best_a: BTreeMap<&A, usize> = BTreeMap::new();
    let mut best_b: BTreeMap<&B, usize> = BTreeMap::new();
    for ((x, y), &v) in &table {
        let e = best_a.entry(x).or_default();
        *e = (*e).max(v);
        let e = best_b.entry(y).or_default();
        *e = (*e).max(v);
    }
    #[allow(clippy::cast_precision_loss)]
    let share = |s: usize| s as f64 / n.max(1) as f64;
    Agreement {
        items: n,
        groups_a: ra.len(),
        groups_b: rb.len(),
        rand,
        purity: share(best_a.values().sum()),
        inverse_purity: share(best_b.values().sum()),
    }
}

/// What kind of error a wrong answer is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Kind {
    /// The right letters, the stress or accent elsewhere.
    Accent,
    /// The same letters once marks are taken off: vowel length (or another
    /// mark: a breathing, a diaeresis).
    Length,
    /// The right form is among the candidates, from another paradigm or
    /// reading: the grammar chose wrong.
    Paradigm,
    /// It goes wrong inside the word's stem.
    Stem,
    /// It goes wrong after the stem: the ending.
    Ending,
}

impl std::fmt::Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Kind::Accent => "accent",
            Kind::Length => "length",
            Kind::Paradigm => "paradigm",
            Kind::Stem => "stem",
            Kind::Ending => "ending",
        })
    }
}

/// The kind of a wrong `answer` for `gold`: `same_letters` says whether the
/// orthography reads the same letters in both (stress aside); `among`
/// whether `gold` is among the candidates; `stem` is the word's stem length
/// (see [`word_stem`]).
pub fn kind(gold: &str, answer: &str, same_letters: bool, among: bool, stem: usize) -> Kind {
    let (g, a) = (plain(gold), plain(answer));
    if same_letters {
        Kind::Accent
    } else if g == a {
        Kind::Length
    } else if among {
        Kind::Paradigm
    } else if common_start(&g, &a) < stem {
        Kind::Stem
    } else {
        Kind::Ending
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_word_s_stem_is_what_half_its_forms_begin_with() {
        let forms = |f: &[&str]| f.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        // regō: most forms keep reg-, the perfect system has rēx-.
        let rego = forms(&[
            "regis", "regit", "regēbam", "rēxī", "rēxit", "rēctum", "regere",
        ]);
        assert_eq!(word_stem("regō", &rego), 3);
        assert_eq!(kind("rēxī", "regī", false, false, 3), Kind::Stem);
        assert_eq!(kind("книги", "книгы", false, false, 4), Kind::Ending);
        assert_eq!(kind("rēxī", "rexī", false, false, 3), Kind::Length);
    }

    #[test]
    fn cells_that_leave_the_lemma_s_stem_together_are_one_group() {
        // Cells: lemma, present 2 sg, 3 sg, 1 pl, perfect 1 sg, 2 sg. Half the
        // words have a perfect stem of their own (`reg-ō`, `rēx-ī`).
        let word = |i: usize, own: bool| {
            let root = format!(
                "t{}r",
                "abcdefghijklmnopqrstuvwxyz".chars().nth(i).unwrap_or('z')
            );
            let perfect = if own {
                format!("q{root}s")
            } else {
                format!("{root}av")
            };
            let cells = [
                format!("{root}o"),
                format!("{root}as"),
                format!("{root}at"),
                format!("{root}amus"),
                format!("{perfect}i"),
                format!("{perfect}isti"),
            ];
            Lexeme {
                lemma: format!("{root}o"),
                full: format!("{root}o"),
                key: format!("{root}o"),
                cells: cells.iter().map(|f| vec![f.clone()]).collect(),
                inherent: Vec::new(),
                order: i,
                senses: 1,
                source: 0,
                vulgar: false,
                editors: crate::extract::Editors::default(),
                of: crate::extract::Of::Own,
            }
        };
        let words: Vec<Lexeme> = (0..24).map(|i| word(i, i % 2 == 0)).collect();
        let groups = stem_groups(&words, 6, Some(0));
        assert_eq!(groups.first().map(|g| g.cells.clone()), Some(vec![4, 5]));
        assert_eq!(
            default_probe(&groups, &words, false),
            Some((vec![5], vec![4]))
        );
    }

    #[test]
    fn the_same_grouping_agrees_fully_and_a_split_one_less() {
        let same = agreement(&[(1, 'a'), (1, 'a'), (2, 'b'), (2, 'b')]);
        assert!((same.rand - 1.0).abs() < 1e-9);
        let split = agreement(&[(1, 'a'), (2, 'a'), (3, 'b'), (4, 'b')]);
        assert!(split.rand < 1.0);
        assert!((split.purity - 1.0).abs() < 1e-9);
        assert!((split.inverse_purity - 0.5).abs() < 1e-9);
    }

    #[test]
    fn a_clear_gain_is_significant_and_no_change_is_not() {
        let gain: Vec<Paired> = (0..100)
            .map(|i| Paired {
                forms: 10,
                a: 7,
                b: if i % 5 == 0 { 7 } else { 8 },
            })
            .collect();
        let c = compare(&gain);
        assert!(c.low > 0.0 && c.p < 0.001, "{c:?}");
        let none: Vec<Paired> = (0..100)
            .map(|i| Paired {
                forms: 10,
                a: 7,
                b: if i % 2 == 0 { 8 } else { 6 },
            })
            .collect();
        let c = compare(&none);
        assert!(c.low < 0.0 && c.high > 0.0 && c.p > 0.5, "{c:?}");
    }

    #[test]
    fn a_word_on_a_training_word_with_a_prefix_is_found() {
        let training: crate::HashSet<&str> = ["happy", "duco", "ago"].into_iter().collect();
        assert!(prefixed("overhappy", &training, &[]));
        assert!(prefixed("adduco", &training, &["ad"]));
        // Too short to tell from a chance ending: two letters in front, or
        // a word of three.
        assert!(!prefixed("unhappy", &training, &[]));
        assert!(!prefixed("drago", &training, &[]));
        assert!(!prefixed("happy", &training, &[]));
    }
}
