//! Outside yardsticks, through data adapters only: the SIGMORPHON 2023
//! shared task's lemma-disjoint test files (`xtask sigmorphon`), and
//! Albright & Hayes's (2003) English nonce verbs with speakers' answers
//! (`xtask nonce`). The mapping from UniMorph features to the configs'
//! Wiktionary tags lives here, not in the trainer.

use flexion_train::cells::Cells;
use flexion_train::extract::{Lexeme, PosData};
use flexion_train::spec::Language;
use flexion_train::{choose, learn, measure, script};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

/// UniMorph features (SIGMORPHON 2023's hierarchical notation, flattened)
/// as the Wiktionary tags the configs' cells are read by. A feature not
/// here makes its form unmappable; `a|b`: the configs name it either way.
const UNIMORPH: &[(&str, &[&str])] = &[
    ("NOM", &["nominative"]),
    ("ACC", &["accusative"]),
    ("GEN", &["genitive"]),
    ("DAT", &["dative"]),
    ("VOC", &["vocative"]),
    ("INS", &["instrumental"]),
    ("LOC", &["locative"]),
    ("ABL", &["ablative"]),
    ("ESS", &["prepositional"]),
    ("SG", &["singular"]),
    ("PL", &["plural"]),
    ("DU", &["dual"]),
    ("MASC", &["masculine"]),
    ("FEM", &["feminine"]),
    ("NEUT", &["neuter"]),
    ("ANIM", &["animate"]),
    ("INAN", &["inanimate"]),
    ("1", &["first-person"]),
    ("2", &["second-person"]),
    ("3", &["third-person"]),
    ("PRS", &["present"]),
    ("PST", &["past"]),
    ("FUT", &["future"]),
    ("IPFV", &["imperfect"]),
    ("PRF", &["perfect"]),
    ("PLPRF", &["pluperfect"]),
    ("AOR", &["aorist"]),
    ("IND", &["indicative"]),
    ("SBJV", &["subjunctive"]),
    ("OPT", &["optative"]),
    ("IMP", &["imperative"]),
    ("COND", &["conditional"]),
    ("ACT", &["active"]),
    ("MID", &["middle|middle-voice"]),
    ("PASS", &["passive"]),
    ("NFIN", &["infinitive"]),
    ("V.PTCP", &["participle"]),
    ("V.CVB", &["adverbial"]),
    ("LGSPEC1", &["short-form"]),
];

/// The parts of speech a UniMorph tag set may be, by its first feature, in
/// the order tried: a noun with gender (Sanskrit's shared-task "nouns" are
/// adjectives' tables) falls to the adjective's cells.
fn parts_of(features: &str) -> &'static [&'static str] {
    match features.split(';').next() {
        Some("N") => &["noun", "adjective"],
        Some("ADJ") => &["adjective"],
        Some("V" | "V.PTCP" | "V.CVB") => &["verb"],
        _ => &[],
    }
}

/// Whether a UniMorph feature names the part of speech only.
fn part_of(feature: &str) -> bool {
    matches!(feature, "N" | "ADJ" | "V")
}

/// Spelling conventions of the shared task's data that differ from the
/// dictionary's, as `(language, theirs at a word's end, ours)`: UniMorph's
/// Sanskrit writes a final `s` or `r` where Wiktionary writes the visarga.
const WORD_ENDS: &[(&str, &str, &str)] = &[("sanskrit", "स्", "ः"), ("sanskrit", "र्", "ः")];

/// `form` with [`WORD_ENDS`]' endings of `lang` written as ours.
fn conventional(lang: &str, form: &str) -> String {
    form.split(' ')
        .map(|w| {
            WORD_ENDS
                .iter()
                .filter(|(l, _, _)| *l == lang)
                .find_map(|(_, theirs, ours)| Some(format!("{}{ours}", w.strip_suffix(theirs)?)))
                .unwrap_or_else(|| w.to_string())
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// The Wiktionary tags a cell's forms carry: its group's and its values'.
fn cell_tags(cells: &Cells<'_>, cell: usize) -> BTreeSet<String> {
    let mut tags = BTreeSet::new();
    for g in cells
        .groups
        .iter()
        .filter(|g| (g.offset..g.offset + g.len()).contains(&cell))
    {
        tags.extend(g.group.all.iter().map(|t| (*t).to_string()));
        for (f, v) in g.features.iter().zip(g.values(cell)) {
            if let Some(v) = f.values.get(v) {
                tags.extend(v.all.iter().map(|t| (*t).to_string()));
            }
        }
    }
    tags
}

/// UniMorph features as Wiktionary tags, every way the configs may name
/// them; `None` if one has no mapping.
fn tags_of(features: &str) -> Option<Vec<Vec<String>>> {
    let mut ways: Vec<Vec<String>> = vec![Vec::new()];
    let verb = parts_of(features) == ["verb"];
    for (i, f) in features
        .split([';', ',', '(', ')'])
        .filter(|f| !f.is_empty())
        .enumerate()
    {
        // A verb's `NOM(…)` is agreement with its subject, no case.
        if (i == 0 && part_of(f)) || (verb && f == "NOM") {
            continue;
        }
        let (_, mapped) = UNIMORPH.iter().find(|(u, _)| *u == f)?;
        for tag in mapped.iter() {
            ways = ways
                .into_iter()
                .flat_map(|w| {
                    tag.split('|').map(move |t| {
                        let mut w = w.clone();
                        w.push(t.to_string());
                        w
                    })
                })
                .collect();
        }
    }
    Some(ways)
}

/// How a word is looked up: folded as the language folds case, then its
/// key.
fn key_of(lang: &Language, s: &script::Script, word: &str) -> String {
    let o = s.orthography();
    let w = if lang.spelling.fold_case {
        word.to_lowercase()
    } else {
        word.to_string()
    };
    o.key(&o.read(&w).letters)
}

/// `cargo xtask sigmorphon <lang> <file>`: score the grammar on a
/// SIGMORPHON 2023 file (`lemma`, `form`, UniMorph features, tab-separated;
/// the test files are lemma-disjoint from their training files) from each
/// lemma, on the forms whose features map to one of our cells (not
/// periphrastic ones: several words for a word of one). Two
/// grammars: the shipped one (every dictionary word, which may include the
/// test lemmas), and one learned without the test lemmas.
pub fn sigmorphon(lang: &Language, data: &[PosData], file: &Path) -> Result<(), String> {
    let text = std::fs::read_to_string(file).map_err(|e| format!("{}: {e}", file.display()))?;
    let script = script::Script::new(&lang.spelling);
    // Per part of speech: (lemma, cell, gold).
    let mut rows: BTreeMap<&str, Vec<(String, usize, String)>> = BTreeMap::new();
    let (mut total, mut unmapped) = (0usize, BTreeMap::<String, usize>::new());
    for line in text.lines() {
        let f: Vec<&str> = line.split('\t').collect();
        let (Some(lemma), Some(features), Some(form)) = (f.first(), f.get(1), f.get(2)) else {
            continue;
        };
        total += 1;
        // A form of several words for a word of one (Russian `будет
        // делать`) is periphrastic: no cell of ours.
        let periphrastic = form.contains(' ') && !lemma.contains(' ');
        // The first part of speech with a cell whose tags include every
        // one the features map to; failing that, the first with a cell the
        // tags match (the cells may leave a feature unsaid: Russian
        // adjectives' singular).
        let find = |strict: bool| {
            let ways = tags_of(features).filter(|_| !periphrastic)?;
            parts_of(features).iter().find_map(|&p| {
                let part = lang.parts.iter().find(|x| x.name == p)?;
                let cells = Cells::new(lang, part).ok()?;
                ways.iter().find_map(|tags| {
                    let cell = cells.match_tags(tags, &|_| true).into_iter().find(|&c| {
                        let have = cell_tags(&cells, c);
                        !strict || tags.iter().all(|t| have.contains(t))
                    })?;
                    Some((part.name, cell))
                })
            })
        };
        let cell = find(true).or_else(|| find(false));
        match cell {
            Some((p, c)) => {
                rows.entry(p)
                    .or_default()
                    .push(((*lemma).to_string(), c, (*form).to_string()))
            }
            None => *unmapped.entry((*features).to_string()).or_default() += 1,
        }
    }
    let n_unmapped: usize = unmapped.values().sum();
    println!(
        "{}: {total} forms, {} mapped to a cell, {n_unmapped} not (commonest: {})",
        file.display(),
        total - n_unmapped,
        unmapped
            .iter()
            .take(6)
            .map(|(f, n)| format!("{f} {n}"))
            .collect::<Vec<_>>()
            .join(", ")
    );
    for (pos, rows) in &rows {
        let Some((part, d)) = lang.parts.iter().zip(data).find(|(p, _)| p.name == *pos) else {
            continue;
        };
        let shape = flexion_train::shape(lang, part)?;
        let lemmas: BTreeSet<String> = rows.iter().map(|r| key_of(lang, &script, &r.0)).collect();
        let ours = d.lexemes.iter().filter(|l| lemmas.contains(&l.key)).count();
        let without: Vec<Lexeme> = d
            .lexemes
            .iter()
            .filter(|l| !lemmas.contains(&l.key))
            .cloned()
            .collect();
        for (name, lexemes) in [
            ("shipped", &d.lexemes),
            ("without the test lemmas", &without),
        ] {
            let learned = learn::learn(
                script.orthography(),
                &shape,
                lexemes,
                choose::Settings::default(),
            );
            let g = &learned.grammar;
            let o = &g.orthography;
            let (mut exact, mut letters, mut length, mut plain) = (0usize, 0usize, 0usize, 0usize);
            let mut wrong: Vec<String> = Vec::new();
            let mut tables: BTreeMap<&str, Vec<Vec<flexion::Candidate<'_>>>> = BTreeMap::new();
            let mut scored = 0usize;
            for (lemma, cell, gold) in rows {
                // Typed as we cite the word: the file's own form of our
                // lemma's cell where it has one (Sanskrit verbs are cited by
                // their root there, by the present there), which isn't scored.
                let cited = shape.lemma_cell.and_then(|lc| {
                    rows.iter()
                        .find(|(l, c, _)| l == lemma && *c == lc)
                        .map(|(_, _, f)| f.as_str())
                });
                if cited.is_some() && Some(*cell) == shape.lemma_cell {
                    continue;
                }
                scored += 1;
                let table = tables.entry(lemma.as_str()).or_insert_with(|| {
                    let w = cited.unwrap_or(lemma);
                    let w = if lang.spelling.fold_case {
                        w.to_lowercase()
                    } else {
                        w.to_string()
                    };
                    g.table(&w, &flexion::Known::default())
                });
                let answer = table
                    .get(*cell)
                    .and_then(|c| c.first())
                    .map_or("", |c| c.form.as_str());
                let gold = if lang.spelling.fold_case && !part.capitalized {
                    gold.to_lowercase()
                } else {
                    gold.clone()
                };
                let (gold, answer) = (
                    conventional(lang.name, &gold),
                    conventional(lang.name, answer),
                );
                let answer = answer.as_str();
                let right = nfc(answer) == nfc(&gold);
                if !right && wrong.len() < 4 {
                    wrong.push(format!("{lemma} cell {cell}: {gold} ≠ {answer}"));
                }
                exact += usize::from(right);
                let (ours, theirs) = (o.read(answer).letters, o.read(&gold).letters);
                letters += usize::from(ours == theirs);
                length += usize::from(unlengthened(&ours) == unlengthened(&theirs));
                plain += usize::from(measure::plain(answer) == measure::plain(&gold));
            }
            let pct = |n: usize| 100.0 * n as f64 / scored.max(1) as f64;
            println!(
                "  {pos}, {name}: {} lemmas ({ours} in our dictionary), {} forms: top-1 {:.2}% exact, {:.2}% stress aside, {:.2}% stress and vowel length aside, {:.2}% every mark aside",
                lemmas.len(),
                scored,
                pct(exact),
                pct(letters),
                pct(length),
                pct(plain)
            );
            println!("    for example: {}", wrong.join("; "));
        }
    }
    Ok(())
}

/// `s` without vowel-length marks (combining macron and breve): the
/// shared task's Greek writes them on some vowels and not others.
fn unlengthened(s: &str) -> String {
    use unicode_normalization::UnicodeNormalization;
    s.nfd()
        .filter(|c| !matches!(c, '\u{304}' | '\u{306}'))
        .nfc()
        .collect()
}

fn nfc(s: &str) -> String {
    use unicode_normalization::UnicodeNormalization;
    s.nfc().collect()
}

/// `cargo xtask nonce english <file>`: the English verb grammar's
/// probability for each past form of Albright & Hayes's (2003) nonce verbs
/// against speakers' production probabilities and ratings (Pearson and
/// Spearman), regular and irregular pasts apart, as their Table 4 reports
/// them. The file: `stem, past, regular (1/0), production probability, mean
/// rating`, tab-separated, `#` lines aside. The grammar learns from every
/// dictionary verb.
pub fn nonce(lang: &Language, data: &[PosData], file: &Path) -> Result<(), String> {
    let text = std::fs::read_to_string(file).map_err(|e| format!("{}: {e}", file.display()))?;
    let (part, d) = lang
        .parts
        .iter()
        .zip(data)
        .find(|(p, _)| p.name == "verb")
        .ok_or("no verbs")?;
    let cells = Cells::new(lang, part)?;
    let past = cells
        .match_tags(&["past".to_string()], &|_| true)
        .into_iter()
        .next()
        .ok_or("no past cell")?;
    let script = script::Script::new(&lang.spelling);
    let shape = flexion_train::shape(lang, part)?;
    let learned = learn::learn(
        script.orthography(),
        &shape,
        &d.lexemes,
        choose::Settings::default(),
    );
    let g = &learned.grammar;
    // (regular, ours, production, rating)
    let mut points: Vec<(bool, f64, f64, f64)> = Vec::new();
    for line in text.lines().filter(|l| !l.starts_with('#')) {
        let f: Vec<&str> = line.split('\t').collect();
        let (Some(stem), Some(form), Some(regular), Some(prod), Some(rating)) =
            (f.first(), f.get(1), f.get(2), f.get(3), f.get(4))
        else {
            continue;
        };
        let c = g.candidates(stem, past, &flexion::Known::default());
        let ours: f32 = c
            .iter()
            .filter(|c| c.form == *form)
            .map(|c| c.probability)
            .sum();
        let parse = |s: &str| {
            s.parse::<f64>()
                .map_err(|_| format!("bad number in {line}"))
        };
        points.push((
            *regular == "1",
            f64::from(ours),
            parse(prod)?,
            parse(rating)?,
        ));
    }
    for (name, regular) in [("regular", true), ("irregular", false)] {
        let p: Vec<&(bool, f64, f64, f64)> = points.iter().filter(|x| x.0 == regular).collect();
        let ours: Vec<f64> = p.iter().map(|x| x.1).collect();
        let prod: Vec<f64> = p.iter().map(|x| x.2).collect();
        let rating: Vec<f64> = p.iter().map(|x| x.3).collect();
        println!(
            "{name} pasts ({}): with production probability r = {:.3}, ρ = {:.3}; with ratings r = {:.3}, ρ = {:.3}",
            p.len(),
            pearson(&ours, &prod),
            pearson(&ranks(&ours), &ranks(&prod)),
            pearson(&ours, &rating),
            pearson(&ranks(&ours), &ranks(&rating)),
        );
    }
    Ok(())
}

fn pearson(x: &[f64], y: &[f64]) -> f64 {
    #[allow(clippy::cast_precision_loss)]
    let n = x.len().min(y.len()).max(1) as f64;
    let (mx, my) = (x.iter().sum::<f64>() / n, y.iter().sum::<f64>() / n);
    let (mut sxy, mut sxx, mut syy) = (0.0, 0.0, 0.0);
    for (a, b) in x.iter().zip(y) {
        sxy += (a - mx) * (b - my);
        sxx += (a - mx) * (a - mx);
        syy += (b - my) * (b - my);
    }
    if sxx > 0.0 && syy > 0.0 {
        sxy / (sxx * syy).sqrt()
    } else {
        0.0
    }
}

/// Ranks, ties sharing their mean rank.
fn ranks(x: &[f64]) -> Vec<f64> {
    let mut order: Vec<usize> = (0..x.len()).collect();
    order.sort_by(|&a, &b| {
        let (p, q) = (
            x.get(a).copied().unwrap_or(0.0),
            x.get(b).copied().unwrap_or(0.0),
        );
        p.total_cmp(&q)
    });
    let mut out = vec![0.0; x.len()];
    let mut i = 0;
    while i < order.len() {
        let value = |k: usize| order.get(k).and_then(|&o| x.get(o)).copied();
        let mut j = i;
        while j + 1 < order.len() && value(j + 1) == value(i) {
            j += 1;
        }
        #[allow(clippy::cast_precision_loss)]
        let mean = (i + j) as f64 / 2.0 + 1.0;
        for k in i..=j {
            if let Some(slot) = order.get(k).and_then(|&o| out.get_mut(o)) {
                *slot = mean;
            }
        }
        i = j + 1;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unimorph_features_become_wiktionary_tags_or_nothing() {
        assert_eq!(
            tags_of("N;ACC(PL)"),
            Some(vec![vec!["accusative".to_string(), "plural".to_string()]])
        );
        assert_eq!(tags_of("V.PTCP;MID;PST").map(|t| t.len()), Some(2));
        // A causative has no cell of ours.
        assert_eq!(tags_of("V;CAUS;FUT;ACT;NOM(PL,2)"), None);
        assert_eq!(
            tags_of("V;PRS;NOM(1,SG)").map(|t| t.concat().len()),
            Some(3)
        );
        assert_eq!(conventional("sanskrit", "देवास्"), "देवाः");
    }
}
