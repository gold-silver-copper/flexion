//! `cargo xtask audit <lang> --pos <p> [--cells <cells>|weak] [--n 200]`: a
//! sample of wrong answers on the held-out tenth (from the headword), one
//! per line, tab-separated, with what kind of error each is.

use flexion_train::eval;
use flexion_train::extract::{Lexeme, PosData};
use flexion_train::measure::{self, Kind};
use flexion_train::spec::Language;
use flexion_train::{choose, learn, script};
use std::collections::BTreeMap;

/// Cells scored by at least this many forms count for `weak`.
const WEAK_FORMS: usize = 20;
/// How many of the weakest cells `weak` takes.
const WEAK_CELLS: usize = 5;

pub struct Options<'a> {
    pub pos: &'a str,
    /// Cells: `weak`, or numbers and names as `try --hold-cells` takes them;
    /// none: every cell.
    pub cells: Option<&'a str>,
    pub n: usize,
    pub fold: u64,
}

struct Wrong {
    word: String,
    cell: usize,
    gold: String,
    answer: String,
    paradigm: String,
    status: String,
    rank: Option<usize>,
    gold_paradigm: String,
    kind: Kind,
    direction: &'static str,
}

pub fn run(lang: &Language, data: &[PosData], o: &Options<'_>) -> Result<(), String> {
    let (part, d) = lang
        .parts
        .iter()
        .zip(data)
        .find(|(p, _)| p.name == o.pos)
        .ok_or("no such part of speech")?;
    let names = crate::cell_names(lang, part)?;
    let mut shape = flexion_train::shape(lang, part)?;
    let script = script::Script::new(&lang.spelling);
    shape.marks = flexion_train::marks(
        lang,
        &script.orthography(),
        data,
        |l: &Lexeme| eval::bucket(&l.key) != o.fold,
        choose::Settings::default(),
    )?;
    let train: Vec<Lexeme> = d
        .lexemes
        .iter()
        .filter(|l| eval::bucket(&l.key) != o.fold)
        .cloned()
        .collect();
    let test: Vec<&Lexeme> = d
        .lexemes
        .iter()
        .filter(|l| eval::bucket(&l.key) == o.fold && l.source == 0)
        .collect();
    let learned = learn::learn(
        script.orthography(),
        &shape,
        &train,
        choose::Settings::default(),
    );
    let g = &learned.grammar;
    let orth = &g.orthography;
    let words: BTreeMap<&str, u32> = g
        .paradigms
        .iter()
        .map(|p| (p.name.as_ref(), p.words))
        .collect();
    let size = |p: &str| words.get(p).copied().unwrap_or(0);
    // Per cell: (forms, right); and every wrong answer.
    let mut per_cell: BTreeMap<usize, (usize, usize)> = BTreeMap::new();
    let mut wrong: Vec<Wrong> = Vec::new();
    for l in &test {
        let table = g.table(&l.lemma, &flexion::Known::default());
        let lemma = shape
            .lemma_cell
            .and_then(|c| l.cells.get(c))
            .and_then(|f| f.first())
            .unwrap_or(&l.full);
        let firsts: Vec<String> = l
            .cells
            .iter()
            .enumerate()
            .filter(|&(c, _)| Some(c) != shape.lemma_cell)
            .filter_map(|(_, f)| f.first().cloned())
            .collect();
        let stem = measure::word_stem(lemma, &firsts);
        for (cell, forms) in l.cells.iter().enumerate() {
            let Some(gold) = forms.first() else { continue };
            let candidates = table.get(cell).map_or(&[][..], Vec::as_slice);
            let rank = candidates
                .iter()
                .position(|c| eval::same_form(orth, &c.form, gold));
            let tally = per_cell.entry(cell).or_default();
            tally.0 += 1;
            tally.1 += usize::from(rank == Some(0));
            if rank == Some(0) {
                continue;
            }
            let first = candidates.first();
            let answer = first.map_or("", |c| c.form.as_str());
            let paradigm = first.map_or("", |c| c.paradigm);
            let gold_paradigm = rank
                .and_then(|r| candidates.get(r))
                .map_or("", |c| c.paradigm);
            let same_letters = orth.read(answer).letters == orth.read(gold).letters;
            let kind = measure::kind(gold, answer, same_letters, rank.is_some(), stem);
            let direction = match rank {
                Some(_) if gold_paradigm == paradigm => "-",
                Some(_) if size(paradigm) > size(gold_paradigm) => "over-regular",
                Some(_) => "over-irregular",
                None if first.is_some_and(|c| c.status == flexion::Status::Regular) => {
                    "over-regular"
                }
                None => "-",
            };
            wrong.push(Wrong {
                word: l.full.clone(),
                cell,
                gold: gold.clone(),
                answer: answer.to_string(),
                paradigm: paradigm.to_string(),
                status: first.map_or(String::new(), |c| c.status.to_string()),
                rank,
                gold_paradigm: gold_paradigm.to_string(),
                kind,
                direction,
            });
        }
    }
    let chosen: Vec<usize> = match o.cells {
        None => (0..names.len()).collect(),
        Some("weak") => {
            let mut cells: Vec<(f64, usize)> = per_cell
                .iter()
                .filter(|(_, (n, _))| *n >= WEAK_FORMS)
                .map(|(&c, &(n, r))| (r as f64 / n as f64, c))
                .collect();
            cells.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)));
            cells.into_iter().take(WEAK_CELLS).map(|(_, c)| c).collect()
        }
        Some(spec) => crate::held_cells(spec, &names, &train, shape.lemma_cell)?,
    };
    let mut sample: Vec<&Wrong> = wrong.iter().filter(|w| chosen.contains(&w.cell)).collect();
    let all = sample.len();
    sample.sort_by_key(|w| (eval::hash(&format!("{}\u{1f}{}", w.word, w.cell)), w.cell));
    sample.truncate(o.n);
    sample.sort_by(|a, b| a.cell.cmp(&b.cell).then(a.word.cmp(&b.word)));
    println!(
        "word\tcell\tcell name\tgold\tanswer\tanswer's paradigm\tstatus\tgold's place\tgold's paradigm\tkind\tdirection"
    );
    for w in &sample {
        println!(
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            w.word,
            w.cell,
            names.get(w.cell).map_or("", String::as_str),
            w.gold,
            w.answer,
            w.paradigm,
            w.status,
            w.rank.map_or("-".to_string(), |r| (r + 1).to_string()),
            w.gold_paradigm,
            w.kind,
            w.direction
        );
    }
    let mut kinds: BTreeMap<String, usize> = BTreeMap::new();
    let mut directions: BTreeMap<&str, usize> = BTreeMap::new();
    for w in &sample {
        *kinds.entry(w.kind.to_string()).or_default() += 1;
        *directions.entry(w.direction).or_default() += 1;
    }
    let shown: Vec<String> = chosen
        .iter()
        .map(|c| {
            let (n, r) = per_cell.get(c).copied().unwrap_or((0, 0));
            format!(
                "{} {:.1}% of {n}",
                names.get(*c).map_or("", String::as_str),
                100.0 * r as f64 / n.max(1) as f64
            )
        })
        .collect();
    println!(
        "# {} of {all} wrong answers in {} cells ({})",
        sample.len(),
        chosen.len(),
        if chosen.len() <= 12 {
            shown.join("; ")
        } else {
            "every cell".to_string()
        },
    );
    println!("# by kind: {kinds:?}");
    println!("# by direction: {directions:?}");
    Ok(())
}
