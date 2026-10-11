//! How close flexion's models are to what their data allows. Reads the lexeme
//! cache (`$FLEXION_CACHE`, filled by `xtask extract`) and the models
//! `curves.sh` keeps (`<lang>[-<pos>]-full.bin`, trained without the held-out
//! tenth):
//!
//! - `ceiling noise <lang>`: where two Wiktionary editions both have a word,
//!   how often their primary forms of a cell differ, per cell;
//! - `ceiling ending <lang> <pos> <stress|length|<inherent feature>>`: how well
//!   the lemma's stress, vowel length or an inherent feature can be predicted
//!   from its last (or first) k letters alone, on the held-out tenth, next to
//!   what the tree gets;
//! - `ceiling compare <lang> <pos> <predictions.tsv> [model.bin]`: another
//!   model's answers (`lemma, cell, primary, answer`, as `neural/seq2seq.py`
//!   writes them) next to the tree's on the same held-out cells, by cell,
//!   ending and word type (the tree: `model.bin`, else the full-data probe
//!   model).

use flexion::{Model, Options, Spelling};
use flexion_train::cells::Cells;
use flexion_train::extract::{self, Lexeme, PosData, Word};
use flexion_train::learn::same_form;
use flexion_train::spec::{Language, PartOfSpeech};
use flexion_train::{TEST_BUCKET, bucket};
use std::collections::{BTreeMap, HashMap};
use std::path::PathBuf;
use unicode_normalization::UnicodeNormalization;
use unicode_normalization::char::is_combining_mark;

const USAGE: &str = "usage: ceiling noise <lang> | ending <lang> <pos> <stress|length|<feature>> \
                     | compare <lang> <pos> <predictions.tsv> [model.bin]";

/// The deepest context the ending (or beginning) predictors read.
const MAX_DEPTH: usize = 12;
/// Interpolation weight of the parent context in the held-out log-loss.
const BACKOFF: f64 = 2.0;

fn main() {
    if let Err(e) = run() {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let arg = |i: usize| args.get(i).map(String::as_str).ok_or(USAGE);
    let lang = flexion_configs::by_name(arg(1)?).ok_or("no such language")?;
    let data = load(&lang)?;
    match arg(0)? {
        "noise" => noise(&lang, &data),
        "ending" => ending(&lang, &data, arg(2)?, arg(3)?),
        "compare" => compare(
            &lang,
            &data,
            arg(2)?,
            arg(3)?,
            args.get(4).map(String::as_str),
        ),
        _ => Err(USAGE.into()),
    }
}

fn cache_dir() -> PathBuf {
    match std::env::var("FLEXION_CACHE") {
        Ok(dir) => PathBuf::from(dir),
        Err(_) => PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| ".".into()))
            .join(".cache/flexion"),
    }
}

fn load(lang: &Language) -> Result<Vec<PosData>, String> {
    let path = cache_dir().join(format!("{}-lexemes.json", lang.name));
    let bytes = std::fs::read(&path).map_err(|e| format!("{}: {e}", path.display()))?;
    serde_json::from_slice(&bytes).map_err(|e| e.to_string())
}

/// The full-data probe model `curves.sh` kept for this part of speech.
fn model_bytes(lang: &Language, pos: &str) -> Result<Vec<u8>, String> {
    let dir = cache_dir();
    let own = dir.join(format!("{}-{pos}-full.bin", lang.name));
    let shared = dir.join(format!("{}-full.bin", lang.name));
    std::fs::read(&own)
        .or_else(|_| std::fs::read(&shared))
        .map_err(|e| format!("{}: {e} (run curves.sh first)", shared.display()))
}

fn part<'l>(lang: &'l Language, name: &str) -> Result<(usize, &'l PartOfSpeech), String> {
    lang.parts
        .iter()
        .enumerate()
        .find(|(_, p)| p.name == name)
        .ok_or(format!("{}: no part of speech {name}", lang.name))
}

fn pct(a: usize, n: usize) -> f64 {
    if n == 0 {
        0.0
    } else {
        100.0 * a as f64 / n as f64
    }
}

/// The held-out tenth, scored on the main dump's entries as the reports do.
fn held_out(d: &PosData) -> PosData {
    let mut main = d.main_only();
    main.lexemes.retain(|l| bucket(&l.key) == TEST_BUCKET);
    main
}

// ---------------------------------------------------------------- noise

#[derive(Default, Clone, Copy)]
struct Agreement {
    compared: usize,
    differ: usize,
    differ_letters: usize,
    /// Differ, and neither edition lists the other's primary form at all.
    conflict: usize,
}

impl Agreement {
    fn add(&mut self, o: &Agreement) {
        self.compared += o.compared;
        self.differ += o.differ;
        self.differ_letters += o.differ_letters;
        self.conflict += o.conflict;
    }

    fn row(&self, name: &str) -> String {
        format!(
            "| {name} | {} | {:.2}% | {:.2}% | {:.2}% | {:.2}% |",
            self.compared,
            pct(self.differ, self.compared),
            pct(self.differ - self.differ_letters, self.compared),
            pct(self.differ_letters, self.compared),
            pct(self.conflict, self.compared),
        )
    }
}

fn noise(lang: &Language, data: &[PosData]) -> Result<(), String> {
    let spec = flexion_train::spelling_spec(lang);
    let sp = flexion_train::view(&spec);
    let shapes = flexion_train::shapes(lang)?;
    let letters = |f: &str| sp.unmarked(&sp.read(f).base);
    for ((part, d), shape) in lang.parts.iter().zip(data).zip(&shapes) {
        if !d.lexemes.iter().any(|l| l.source > 0) {
            continue;
        }
        let cells = Cells::new(lang, part)?;
        let mut per_cell: Vec<Agreement> = vec![Agreement::default(); shape.cells];
        let (mut shared, mut any_differ) = (0usize, 0usize);
        for w in extract::words(d) {
            let main: Vec<&Lexeme> = w
                .lexemes
                .iter()
                .copied()
                .filter(|l| l.source == 0)
                .collect();
            let other: Vec<&Lexeme> = w.lexemes.iter().copied().filter(|l| l.source > 0).collect();
            if main.is_empty() || other.is_empty() {
                continue;
            }
            shared += 1;
            let mut differs = false;
            for (cell, slot) in per_cell.iter_mut().enumerate() {
                let (Some(a), Some(b)) = (first(&main, cell), first(&other, cell)) else {
                    continue;
                };
                slot.compared += 1;
                if same_form(&sp, a, b) {
                    continue;
                }
                differs = true;
                slot.differ += 1;
                if letters(a) != letters(b) {
                    slot.differ_letters += 1;
                }
                let listed = |ls: &[&Lexeme], f: &str| {
                    ls.iter()
                        .filter_map(|l| l.cells.get(cell))
                        .flatten()
                        .any(|g| same_form(&sp, g, f))
                };
                if !listed(&main, b) && !listed(&other, a) {
                    slot.conflict += 1;
                }
            }
            any_differ += usize::from(differs);
        }
        println!(
            "\n### {} {}: {shared} words in both editions, {:.2}% differ in some cell\n",
            lang.name,
            part.name,
            pct(any_differ, shared)
        );
        println!(
            "| cell | cells in both | differ | marks only | letters | neither lists the other's |"
        );
        println!("|---|---:|---:|---:|---:|---:|");
        let mut total = Agreement::default();
        let mut groups: Vec<(String, Agreement)> = Vec::new();
        for (cell, a) in per_cell.iter().enumerate() {
            if a.compared == 0 {
                continue;
            }
            let name = if cell == shape.lemma_cell {
                "citation form".to_string()
            } else {
                cells.group_of(cell).map_or("?", |g| g.name).to_string()
            };
            if cell != shape.lemma_cell {
                total.add(a);
            }
            match groups.iter_mut().find(|(n, _)| *n == name) {
                Some((_, g)) => g.add(a),
                None => groups.push((name, *a)),
            }
        }
        for (name, a) in &groups {
            println!("{}", a.row(name));
        }
        println!("{}", total.row("**all but the citation form**"));
        // The cells that differ most often, with enough support.
        let mut worst: Vec<(usize, &Agreement)> = per_cell
            .iter()
            .enumerate()
            .filter(|(c, a)| *c != shape.lemma_cell && a.compared >= 200)
            .collect();
        worst.sort_by(|x, y| {
            pct(y.1.differ, y.1.compared).total_cmp(&pct(x.1.differ, x.1.compared))
        });
        let shown: Vec<String> = worst
            .iter()
            .take(8)
            .map(|(c, a)| format!("{} {:.2}%", cells.name(*c), pct(a.differ, a.compared)))
            .collect();
        println!("\nMost often different: {}", shown.join(", "));
    }
    Ok(())
}

fn first<'a>(ls: &[&'a Lexeme], cell: usize) -> Option<&'a str> {
    ls.iter()
        .find_map(|l| l.cells.get(cell).and_then(|f| f.first()))
        .map(String::as_str)
}

// ---------------------------------------------------------------- ending

/// Per letter of a form: which of `marks` it carries (bit i for mark i),
/// with an implicitly stressed letter (Russian `ё`) counted as marked when
/// nothing else is.
fn pattern(form: &str, marks: &[char], implicit: Option<char>) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();
    let mut implied = None;
    let flag = |c: char| {
        marks
            .iter()
            .position(|&m| m == c)
            .map_or(0u8, |i| 1u8 << i.min(7))
    };
    for c in form.to_lowercase().nfc() {
        if is_combining_mark(c) {
            if let Some(last) = out.last_mut() {
                *last |= flag(c);
            }
            continue;
        }
        if Some(c) == implicit && implied.is_none() {
            implied = Some(out.len());
        }
        out.push(
            c.to_string()
                .nfd()
                .skip(1)
                .fold(0u8, |acc, m| acc | flag(m)),
        );
    }
    if let Some(i) = implied
        && out.iter().all(|&b| b & 1 == 0)
        && let Some(b) = out.get_mut(i)
    {
        *b |= 1;
    }
    out
}

/// A label for word `n` letters long, read from a training word's label
/// aligned at the end (or the start).
fn aligned(label: &[u8], n: usize, from_end: bool) -> Vec<u8> {
    if label.len() >= n {
        let skip = if from_end { label.len() - n } else { 0 };
        return label.iter().skip(skip).take(n).copied().collect();
    }
    let pad = std::iter::repeat_n(0u8, n - label.len());
    if from_end {
        pad.chain(label.iter().copied()).collect()
    } else {
        label.iter().copied().chain(pad).collect()
    }
}

struct Example {
    key: Vec<char>,
    label: Vec<u8>,
}

/// Label counts per context: the last (or first) `k` letters, `k ≤ MAX_DEPTH`.
struct Contexts {
    from_end: bool,
    /// Per-letter labels are aligned to the word; a feature value is not.
    per_letter: bool,
    counts: HashMap<(usize, String), BTreeMap<Vec<u8>, f64>>,
}

impl Contexts {
    fn context(&self, key: &[char], k: usize) -> Option<String> {
        if k > key.len() {
            return None;
        }
        let s: String = if self.from_end {
            key.iter().skip(key.len() - k).collect()
        } else {
            key.iter().take(k).collect()
        };
        Some(s)
    }

    fn new(train: &[Example], from_end: bool, per_letter: bool) -> Contexts {
        let mut c = Contexts {
            from_end,
            per_letter,
            counts: HashMap::new(),
        };
        for e in train {
            for k in 0..=MAX_DEPTH {
                let Some(ctx) = c.context(&e.key, k) else {
                    break;
                };
                *c.counts
                    .entry((k, ctx))
                    .or_default()
                    .entry(e.label.clone())
                    .or_default() += 1.0;
            }
        }
        c
    }

    /// The label distribution at a context, for a word `n` letters long.
    fn at(&self, key: &[char], k: usize, n: usize) -> Option<BTreeMap<Vec<u8>, f64>> {
        let counts = self.counts.get(&(k, self.context(key, k)?))?;
        let mut out: BTreeMap<Vec<u8>, f64> = BTreeMap::new();
        for (label, c) in counts {
            let l = if self.per_letter {
                aligned(label, n, self.from_end)
            } else {
                label.clone()
            };
            *out.entry(l).or_default() += c;
        }
        Some(out)
    }

    /// The majority label reading at most `k` letters (backing off to the
    /// deepest context seen), and the depth used.
    fn predict(&self, key: &[char], k: usize, n: usize) -> (Option<Vec<u8>>, usize) {
        for depth in (0..=k.min(key.len())).rev() {
            if let Some(dist) = self.at(key, depth, n) {
                let best = dist
                    .into_iter()
                    .max_by(|a, b| a.1.total_cmp(&b.1).then(b.0.cmp(&a.0)))
                    .map(|(l, _)| l);
                return (best, depth);
            }
        }
        (None, 0)
    }

    /// The held-out probability of `truth`, interpolating each context with
    /// its parent's estimate (the root with a uniform share for the unseen).
    fn probability(&self, key: &[char], k: usize, truth: &[u8]) -> f64 {
        let n = truth.len();
        let mut p = 1e-6;
        for depth in 0..=k.min(key.len()) {
            let Some(dist) = self.at(key, depth, n) else {
                break;
            };
            let total: f64 = dist.values().sum();
            let c = dist.get(truth).copied().unwrap_or(0.0);
            p = (c + BACKOFF * p) / (total + BACKOFF);
        }
        p
    }
}

fn ending(lang: &Language, data: &[PosData], pos: &str, target: &str) -> Result<(), String> {
    let spec = flexion_train::spelling_spec(lang);
    let sp = flexion_train::view(&spec);
    let shapes = flexion_train::shapes(lang)?;
    let (pi, part) = part(lang, pos)?;
    let shape = shapes.get(pi).ok_or("no shape")?;
    let d = data.get(pi).ok_or("no data")?;
    let bytes = model_bytes(lang, pos)?;
    let model = Model::from_bytes(&bytes).ok_or("model failed to load")?;
    let p = model.pos(pi).ok_or("no such part in the model")?;

    let marks: Vec<char> = match target {
        "stress" => lang.spelling.stress_marks.chars().collect(),
        "length" => vec!['\u{304}', '\u{306}'],
        _ => Vec::new(),
    };
    let implicit = lang
        .spelling
        .implicit
        .chars()
        .next()
        .filter(|_| target == "stress");
    let feature = part.inherent.iter().position(|f| *f == target);
    let per_letter = feature.is_none();
    if per_letter && marks.is_empty() {
        return Err(format!(
            "{target}: expected stress, length or one of {:?}",
            part.inherent
        ));
    }
    let values: Vec<&str> = feature
        .and_then(|f| part.inherent.get(f))
        .and_then(|name| lang.feature(name))
        .map(|f| f.values.iter().map(|v| v.name).collect())
        .unwrap_or_default();
    // The truth for a word, read from its first entry that has it.
    let label = |w: &Word| -> Option<Vec<u8>> {
        match feature {
            Some(f) => w
                .lexemes
                .iter()
                .find_map(|l| l.inherent.get(f).copied().filter(|&v| v != u8::MAX))
                .map(|v| vec![v]),
            None => w
                .primary(shape.lemma_cell)
                .map(|f| pattern(f, &marks, implicit)),
        }
    };
    let key_of = |w: &Word| -> Vec<char> { sp.read(w.lemma()).key.chars().collect() };
    let examples = |words: Vec<Word>| -> Vec<(Example, String)> {
        words
            .iter()
            .filter_map(|w| {
                let key = key_of(w);
                let label = label(w)?;
                (!per_letter || label.len() == key.len())
                    .then(|| (Example { key, label }, w.lemma().to_string()))
            })
            .collect()
    };
    let mut train_words = extract::words(d);
    train_words.retain(|w| bucket(w.key) != TEST_BUCKET);
    let train: Vec<Example> = examples(train_words).into_iter().map(|(e, _)| e).collect();
    let test_data = held_out(d);
    let test = examples(extract::words(&test_data));
    let n = test.len();

    // What the tree says, from the bare lemma.
    let mut tree_guess = Vec::with_capacity(n);
    for (_, lemma) in &test {
        let citation = p.citation(lemma, &[]);
        let guess = match feature {
            Some(f) => p.inherent(&citation, f).first().map(|&(v, _)| vec![v]),
            None => p
                .candidates(&citation, shape.lemma_cell, Options::default())
                .first()
                .map(|c| pattern(&c.form, &marks, implicit)),
        };
        tree_guess.push(guess);
    }
    let tree_right: Vec<bool> = test
        .iter()
        .zip(&tree_guess)
        .map(|((e, _), g)| g.as_deref() == Some(e.label.as_slice()))
        .collect();
    let tree = tree_right.iter().filter(|&&r| r).count();

    let describe = |l: &[u8]| -> String {
        if per_letter {
            l.iter()
                .map(|&b| {
                    if b == 0 {
                        '·'
                    } else {
                        char::from(b'0' + b.min(9))
                    }
                })
                .collect()
        } else {
            l.first()
                .and_then(|&v| values.get(usize::from(v)))
                .map_or("?".to_string(), |s| s.to_string())
        }
    };
    let mut marginal: BTreeMap<String, usize> = BTreeMap::new();
    for e in &train {
        *marginal.entry(describe(&e.label)).or_default() += 1;
    }
    println!(
        "\n### {} {pos}: {target} of the lemma from its spelling\n\n{} training words, {n} held-out words, {} distinct labels.\nThe tree (probe model, bare lemma): **{:.2}%**.\n",
        lang.name,
        train.len(),
        marginal.len(),
        pct(tree, n)
    );
    println!(
        "| letters read | from the end: right | log-loss (bits) | seen at full depth | from the start: right | either right |"
    );
    println!("|---:|---:|---:|---:|---:|---:|");
    let suffix = Contexts::new(&train, true, per_letter);
    let prefix = Contexts::new(&train, false, per_letter);
    let mut best = (0usize, 0usize);
    for k in 0..=MAX_DEPTH {
        let (mut right, mut seen, mut start_right, mut either) = (0, 0, 0, 0);
        let mut bits = 0.0;
        for (e, _) in &test {
            let len = e.label.len();
            let (guess, depth) = suffix.predict(&e.key, k, len);
            let ok = guess.as_deref() == Some(e.label.as_slice());
            right += usize::from(ok);
            seen += usize::from(depth == k.min(e.key.len()));
            bits -= suffix.probability(&e.key, k, &e.label).log2();
            let (from_start, _) = prefix.predict(&e.key, k, len);
            let ok_start = from_start.as_deref() == Some(e.label.as_slice());
            start_right += usize::from(ok_start);
            either += usize::from(ok || ok_start);
        }
        if right > best.1 {
            best = (k, right);
        }
        println!(
            "| {k} | {:.2}% | {:.3} | {:.1}% | {:.2}% | {:.2}% |",
            pct(right, n),
            bits / n.max(1) as f64,
            pct(seen, n),
            pct(start_right, n),
            pct(either, n)
        );
    }
    // Where the tree and the best ending predictor part ways, and (per
    // letter) whether the miss is in the last three letters or before them.
    let k = best.0;
    let (mut both, mut only_tree, mut only_ending) = (0, 0, 0);
    let (mut tail, mut head) = ([0usize; 2], [0usize; 2]);
    let split = |guess: Option<&[u8]>, truth: &[u8]| -> (bool, bool) {
        let cut = truth.len().saturating_sub(3);
        match guess {
            Some(g) if g.len() == truth.len() => (
                g.iter().skip(cut).eq(truth.iter().skip(cut)),
                g.iter().take(cut).eq(truth.iter().take(cut)),
            ),
            _ => (false, false),
        }
    };
    for ((e, _), (&t, tg)) in test.iter().zip(tree_right.iter().zip(&tree_guess)) {
        let (guess, _) = suffix.predict(&e.key, k, e.label.len());
        let s = guess.as_deref() == Some(e.label.as_slice());
        both += usize::from(s && t);
        only_tree += usize::from(t && !s);
        only_ending += usize::from(s && !t);
        for (i, g) in [tg.as_deref(), guess.as_deref()].into_iter().enumerate() {
            let (a, b) = split(g, &e.label);
            if let (Some(x), Some(y)) = (tail.get_mut(i), head.get_mut(i)) {
                *x += usize::from(a);
                *y += usize::from(b);
            }
        }
    }
    println!(
        "\nBest ending predictor: {k} letters, {:.2}%. Both right {:.2}%, only the tree {:.2}%, only the ending predictor {:.2}%.",
        pct(best.1, n),
        pct(both, n),
        pct(only_tree, n),
        pct(only_ending, n)
    );
    if per_letter {
        let [tt, te] = tail;
        let [ht, he] = head;
        println!(
            "Right in the last three letters / in the letters before them: tree {:.2}% / {:.2}%, ending predictor {:.2}% / {:.2}%.",
            pct(tt, n),
            pct(ht, n),
            pct(te, n),
            pct(he, n)
        );
    }
    Ok(())
}

// ---------------------------------------------------------------- compare

#[derive(Default, Clone, Copy)]
struct Pair {
    n: usize,
    tree: usize,
    other: usize,
    tree_letters: usize,
    other_letters: usize,
}

impl Pair {
    fn row(&self, name: &str) -> String {
        format!(
            "| {name} | {} | {:.2}% | {:.2}% | {:+.2} | {:.2}% | {:.2}% |",
            self.n,
            pct(self.tree, self.n),
            pct(self.other, self.n),
            pct(self.other, self.n) - pct(self.tree, self.n),
            pct(self.tree_letters, self.n),
            pct(self.other_letters, self.n),
        )
    }
}

fn table(title: &str, rows: &[(String, Pair)]) {
    println!("\n{title}\n");
    println!("| | cells | tree | neural | neural − tree | tree, letters | neural, letters |");
    println!("|---|---:|---:|---:|---:|---:|---:|");
    for (name, p) in rows {
        println!("{}", p.row(name));
    }
}

fn add(rows: &mut Vec<(String, Pair)>, name: &str, t: bool, o: bool, tl: bool, ol: bool) {
    let i = match rows.iter().position(|(n, _)| n == name) {
        Some(i) => i,
        None => {
            rows.push((name.to_string(), Pair::default()));
            rows.len() - 1
        }
    };
    if let Some((_, p)) = rows.get_mut(i) {
        p.n += 1;
        p.tree += usize::from(t);
        p.other += usize::from(o);
        p.tree_letters += usize::from(tl);
        p.other_letters += usize::from(ol);
    }
}

/// The 2.5th and 97.5th percentiles of (neural − tree) top-1, in points,
/// resampling words (a word's cells move together).
fn bootstrap(words: &[(usize, usize, usize)]) -> (f64, f64) {
    let mut state: u64 = 0x9e37_79b9_7f4a_7c15;
    let mut next = move || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        state
    };
    let n = words.len() as u64;
    if n == 0 {
        return (0.0, 0.0);
    }
    let mut diffs: Vec<f64> = (0..1000)
        .map(|_| {
            let (mut cells, mut tree, mut other) = (0usize, 0usize, 0usize);
            for _ in 0..n {
                let i = usize::try_from(next() % n).unwrap_or(0);
                if let Some(&(c, t, o)) = words.get(i) {
                    cells += c;
                    tree += t;
                    other += o;
                }
            }
            pct(other, cells) - pct(tree, cells)
        })
        .collect();
    diffs.sort_by(f64::total_cmp);
    let at = |q: usize| diffs.get(q).copied().unwrap_or(0.0);
    (at(25), at(974))
}

fn compare(
    lang: &Language,
    data: &[PosData],
    pos: &str,
    path: &str,
    model_path: Option<&str>,
) -> Result<(), String> {
    let spec = flexion_train::spelling_spec(lang);
    let sp: Spelling = flexion_train::view(&spec);
    let shapes = flexion_train::shapes(lang)?;
    let (pi, part) = part(lang, pos)?;
    let shape = shapes.get(pi).ok_or("no shape")?;
    let d = data.get(pi).ok_or("no data")?;
    let cells = Cells::new(lang, part)?;
    let bytes = match model_path {
        Some(m) => std::fs::read(m).map_err(|e| format!("{m}: {e}"))?,
        None => model_bytes(lang, pos)?,
    };
    let model = Model::from_bytes(&bytes).ok_or("model failed to load")?;
    let p = model.pos(pi).ok_or("no such part in the model")?;
    let text = std::fs::read_to_string(path).map_err(|e| format!("{path}: {e}"))?;
    let answers: HashMap<(&str, &str), &str> = text
        .lines()
        .filter_map(|l| {
            let mut c = l.split('\t');
            let (lemma, cell, _, answer) = (c.next()?, c.next()?, c.next()?, c.next()?);
            Some(((lemma, cell), answer))
        })
        .collect();
    let letters = |f: &str| sp.unmarked(&sp.read(f).base);
    let features: Vec<(&str, Vec<&str>)> = part
        .inherent
        .iter()
        .map(|name| {
            let values = lang
                .feature(name)
                .map(|f| f.values.iter().map(|v| v.name).collect())
                .unwrap_or_default();
            (*name, values)
        })
        .collect();

    let test_data = held_out(d);
    let (mut total, mut by_group, mut by_cell, mut by_ending, mut by_feature, mut by_length) = (
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    );
    let mut per_word: Vec<(usize, usize, usize)> = Vec::new();
    let mut words_tree_only: Vec<String> = Vec::new();
    let mut words_other_only: Vec<String> = Vec::new();
    let (mut missing, mut both_wrong, mut either) = (0usize, 0usize, 0usize);
    for (word_index, w) in extract::words(&test_data).into_iter().enumerate() {
        let citation = p.citation(w.lemma(), &[]);
        let key: Vec<char> = sp.read(w.lemma()).key.chars().collect();
        let end: String = key.iter().skip(key.len().saturating_sub(2)).collect();
        let syllables = sp.read(w.lemma()).groups;
        let inherent: Vec<String> = features
            .iter()
            .enumerate()
            .filter_map(|(f, (name, values))| {
                let v = w.lexemes.first()?.inherent.get(f).copied()?;
                Some(format!(
                    "{name} {}",
                    values.get(usize::from(v)).copied().unwrap_or("?")
                ))
            })
            .collect();
        let mut word = (0usize, 0usize, 0usize);
        for cell in (0..cells.total).filter(|c| !shape.unscored.contains(c)) {
            let Some(primary) = w.primary(cell) else {
                continue;
            };
            let name = cells.name(cell);
            let Some(&other) = answers.get(&(w.lemma(), name.as_str())) else {
                missing += 1;
                continue;
            };
            let cands = p.candidates(&citation, cell, Options::default());
            let tree = cands.first().map_or("", |c| c.form.as_str());
            let (t, o) = (
                same_form(&sp, tree, primary),
                same_form(&sp, other, primary),
            );
            let (tl, ol) = (
                letters(tree) == letters(primary),
                letters(other) == letters(primary),
            );
            word = (word.0 + 1, word.1 + usize::from(t), word.2 + usize::from(o));
            add(&mut total, "all", t, o, tl, ol);
            add(
                &mut by_group,
                cells.group_of(cell).map_or("?", |g| g.name),
                t,
                o,
                tl,
                ol,
            );
            add(&mut by_cell, &name, t, o, tl, ol);
            add(&mut by_ending, &format!("-{end}"), t, o, tl, ol);
            add(
                &mut by_length,
                &format!("{} syllables", syllables.min(5)),
                t,
                o,
                tl,
                ol,
            );
            for f in &inherent {
                add(&mut by_feature, f, t, o, tl, ol);
            }
            both_wrong += usize::from(!t && !o);
            either += usize::from(t || o);
            // One example per word, from every 7th word, for variety.
            let example = format!(
                "{} {name}: {primary} (tree {tree}, neural {other})",
                w.lemma()
            );
            let list = match (t, o) {
                (true, false) => Some(&mut words_tree_only),
                (false, true) => Some(&mut words_other_only),
                _ => None,
            };
            if let Some(list) = list
                && list.len() < 15
                && word_index % 7 == 0
                && !list
                    .iter()
                    .any(|e| e.starts_with(&format!("{} ", w.lemma())))
            {
                list.push(example);
            }
        }
        if word.0 > 0 {
            per_word.push(word);
        }
    }
    let n = total.first().map_or(0, |(_, p)| p.n);
    println!(
        "\n### {} {pos}: neural vs tree on the same held-out cells\n\n{n} cells ({missing} without a neural answer). Either right: {:.2}%; both wrong: {:.2}%.",
        lang.name,
        pct(either, n),
        pct(both_wrong, n)
    );
    let (lo, hi) = bootstrap(&per_word);
    println!(
        "Neural − tree, 95% interval from 1000 resamples of the {} words: {lo:+.2} to {hi:+.2} points.",
        per_word.len()
    );
    table("Overall and per group:", &[total, by_group].concat());
    by_cell.sort_by(|a, b| a.0.cmp(&b.0));
    if by_cell.len() <= 40 {
        table("Per cell:", &by_cell);
    } else {
        // Many cells: the ones where the two differ most.
        by_cell.sort_by(|a, b| {
            let gap = |p: &Pair| (pct(p.other, p.n) - pct(p.tree, p.n)).abs() * (p.n as f64).sqrt();
            gap(&b.1).total_cmp(&gap(&a.1))
        });
        by_cell.truncate(20);
        table(
            "The 20 cells where the two differ most (by gap × √n):",
            &by_cell,
        );
    }
    by_ending.sort_by_key(|e| std::cmp::Reverse(e.1.n));
    by_ending.truncate(15);
    table("Per lemma ending (the 15 most common):", &by_ending);
    by_length.sort_by(|a, b| a.0.cmp(&b.0));
    table("Per lemma length (syllables; 5 = 5 or more):", &by_length);
    if !by_feature.is_empty() {
        by_feature.sort_by(|a, b| a.0.cmp(&b.0));
        table("Per inherent feature:", &by_feature);
    }
    println!("\nOnly the neural model right, e.g.:");
    for e in &words_other_only {
        println!("- {e}");
    }
    println!("\nOnly the tree right, e.g.:");
    for e in &words_tree_only {
        println!("- {e}");
    }
    Ok(())
}
