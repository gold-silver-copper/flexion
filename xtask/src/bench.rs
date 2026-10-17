//! `cargo xtask bench <language> [--pos <part>] [--dump <file>]`: how long one
//! call takes (one cell's candidates, release build), on words the grammar
//! was learned from and on held-out words apart, per part of speech. With
//! `--dump`, every candidate (form, probability bits, status) is written, so
//! that a change meant only to be faster can be checked to answer the same.

use flexion_train::extract::{Lexeme, PosData};
use flexion_train::spec::Language;
use flexion_train::{choose, eval, learn, script};
use std::fmt::Write as _;
use std::time::Instant;

/// Words timed per group, cells per word, and rounds.
const WORDS: usize = 30;
const CELLS: usize = 40;
const ROUNDS: usize = 5;

pub fn run(
    lang: &Language,
    data: &[PosData],
    pos: Option<&str>,
    dump: Option<&str>,
) -> Result<(), String> {
    let script = script::Script::new(&lang.spelling);
    let mut out = String::new();
    for (part, d) in lang.parts.iter().zip(data) {
        if pos.is_some_and(|p| p != part.name) {
            continue;
        }
        let mut shape = flexion_train::shape(lang, part)?;
        shape.marks = flexion_train::marks(
            lang,
            &script.orthography(),
            data,
            |l: &Lexeme| eval::bucket(&l.key) != eval::TEST,
            choose::Settings::default(),
        )?
        .map(|m| m.of(part.name));
        let train: Vec<Lexeme> = d
            .lexemes
            .iter()
            .filter(|l| eval::bucket(&l.key) != eval::TEST)
            .cloned()
            .collect();
        let learned = learn::learn(
            script.orthography(),
            &shape,
            &train,
            choose::Settings::default(),
        );
        let g = &learned.grammar;
        let group = |held: bool| -> Vec<&str> {
            d.lexemes
                .iter()
                .filter(|l| (eval::bucket(&l.key) == eval::TEST) == held && l.source == 0)
                .take(WORDS)
                .map(|l| l.lemma.as_str())
                .collect()
        };
        let cells: Vec<usize> = (0..g.cells.min(CELLS)).collect();
        let known = flexion::Known::default();
        let mut line = format!("{} {}:", lang.name, part.name);
        for (name, words) in [("attested", group(false)), ("held out", group(true))] {
            let t = Instant::now();
            let mut calls = 0u32;
            for _ in 0..ROUNDS {
                for w in &words {
                    for &c in &cells {
                        std::hint::black_box(g.candidates(w, c, &known));
                        calls += 1;
                    }
                }
            }
            let micros = t.elapsed().as_secs_f64() * 1e6 / f64::from(calls.max(1));
            let _ = write!(line, " {name} {micros:.1} µs");
            if dump.is_some() {
                for w in &words {
                    for &c in &cells {
                        for cand in g.candidates(w, c, &known) {
                            let _ = writeln!(
                                out,
                                "{} {w} {c} {} {:08x} {}",
                                part.name,
                                cand.form,
                                cand.probability.to_bits(),
                                cand.status
                            );
                        }
                    }
                }
            }
        }
        // A whole table, as `try` and a caller wanting every form ask.
        let words = group(true);
        let t = Instant::now();
        for _ in 0..ROUNDS {
            for w in &words {
                std::hint::black_box(g.table(w, &known));
            }
        }
        let rounds = u32::try_from(ROUNDS * words.len().max(1)).unwrap_or(u32::MAX);
        let micros = t.elapsed().as_secs_f64() * 1e6 / f64::from(rounds);
        let _ = write!(
            line,
            "; a whole table ({} cells), held out, {micros:.0} µs",
            g.cells
        );
        if dump.is_some() {
            for w in &words {
                for (c, list) in g.table(w, &known).iter().enumerate() {
                    for cand in list {
                        let _ = writeln!(
                            out,
                            "{} {w} table {c} {} {:08x} {}",
                            part.name,
                            cand.form,
                            cand.probability.to_bits(),
                            cand.status
                        );
                    }
                }
            }
        }
        println!("{line} per call");
    }
    if let Some(path) = dump {
        std::fs::write(path, out).map_err(|e| format!("{path}: {e}"))?;
    }
    Ok(())
}
