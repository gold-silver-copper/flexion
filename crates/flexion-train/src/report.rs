//! The README's report of a build: the grammar's size and how well it does,
//! from what the build learned and measured. Only formatting, so left out of
//! the key a saved build is kept under (see [`crate::saved`]): a change here
//! reprints from the saved build.

use crate::eval::{self, Score};
use crate::learn::Learned;
use crate::pipeline::{Compatible, Measured, law_sides};
use crate::spec::Language;
use std::fmt::Write as _;

/// The README's report: the grammar's size and how well it does.
pub fn report(lang: &Language, learned: &[Learned<'_>], measured: &[Measured]) -> String {
    let mut s = String::from("## The grammar\n\n");
    s.push_str("| | words | paradigms | sound laws | rules | listed words | irregular forms | listed by relations |\n|---|---:|---:|---:|---:|---:|---:|---:|\n");
    for (part, g) in lang.parts.iter().zip(learned) {
        let _ = writeln!(
            s,
            "| {} | {} | {} | {} | {} | {} | {} | {} |",
            part.name,
            g.stats.words,
            g.stats.paradigms,
            g.laws.len(),
            g.grammar.paradigm.rules.len(),
            g.grammar.paradigm.exceptions.len(),
            g.stats.irregular_forms,
            g.stats.relation_listed,
        );
    }
    s.push_str(
        "\n*Words*: the dictionary's words the grammar is learned from. *Sound laws*: changes\n\
         where a stem meets its ending (or farther, along a tier). *Rules*: the endings that\n\
         choose a paradigm. *Listed words*: words no rule gets right, listed with their paradigm.\n\
         *Irregular forms*: forms no paradigm gives, listed. *Listed by relations*: the words the\n\
         relations' rules get wrong, listed with the change they take.\n\n## How well it does\n\n\
         A tenth of the words (by a hash of the word) was held out: the grammar scored here was\n\
         learned from the other nine tenths, and asked for every form the dictionary lists of\n\
         each held-out word, typed as the dictionary's headword. *Top-1*: its first answer is\n\
         right; *top-3*: the right answer is among its first three. *Calibration error*: how far\n\
         its stated probabilities are, on average, from how often it is right (0 is perfect).\n\
         The lemma's own form isn't scored where it is the letters typed (it is the input); where\n\
         it isn't (vowel length the headword doesn't write) it is scored, and also apart (*lemma\n\
         spelling*). After top-1, its 95% interval, resampling words. *Lemma in full*: top-1 typed\n\
         with every mark the dictionary's table writes, and said to be so (`spelled_in_full`).\n\
         *Principal parts given*: top-1 of the other forms, given the dictionary's principal\n\
         parts. *Features given*: top-1 from the headword, told the word's inherent features\n\
         (its gender, as `.gender(…)` tells it). *Among the candidates*: the right answer is\n\
         among its answers at all. *Every cell right*: the share of words with every form right.\n\
         *Log loss*: bits per form, −log₂ of the probability it gives the right answer (a form\n\
         it doesn't give costs 5 bits a letter). *1 / 2 / 3 forms given*: top-1 of the other\n\
         forms, typed in full, given that many of the word's forms chosen at random.\n\
         *Seen* and *unseen cells*: top-1 on the cells some training word of the word's\n\
         paradigm has a form in, and on the others, derived\n\
         from other cells (with the number of forms). *Floor*: the CoNLL–SIGMORPHON 2017\n\
         shared task's non-neural baseline (letters only), and on the unseen cells. *Against the\n\
         floor*: the grammar's letters right less the floor's, on the same forms, with its 95%\n\
         interval (resampling words). *Any listed form*: top-1 where any of the forms the\n\
         dictionary lists in the cell counts. *Homograph ceiling*: the most any grammar could get\n\
         right, words typed alike getting one answer per cell. *On a training word / new root*:\n\
         top-1 of the held-out words that are a training word with a prefix or a compound's front\n\
         before it, and of the rest. *Own / form-of / alt-of*: top-1 of the held-out entries that\n\
         are words of their own, another word's form (a participle, a comparative) and another\n\
         word's spelling, with their number of words. *Phrases*: top-1 on the forms the dictionary\n\
         writes as phrases (the Latin perfect passive, `amātus sum`). *Irregular lines*: the share of the\n\
         dictionary's words the grammar lists a form or a stem of. The held-out tenth was looked at while the trainer was being\n\
         developed (changes are judged on another tenth since PR #16). A closed class (pronouns,\n\
         numerals, determiners, adpositions) and a part of speech with fewer than 30 held-out words\n\
         is judged first on its own words: its top-1 is *reproduced*, the shipped grammar's right\n\
         forms of the forms of its own words, then *a new word*, top-1 with each word left out of\n\
         training in turn (a tenth at a time where another part of speech's words teach it). A part\n\
         with no held-out words has no other score (–).\n\n",
    );
    s.push_str(
        "| | held-out words | forms | top-1 | top-3 | among the candidates | every cell right | calibration error | log loss | lemma in full | principal parts given | 1 / 2 / 3 forms given | features given | seen cells | unseen cells | floor (unseen) | against the floor | any listed form | homograph ceiling | on a training word / new root | lemma spelling | own / form-of / alt-of | phrases | irregular lines |\n|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|\n",
    );
    // `right` of `forms`, as a percentage with the count of forms.
    let share = |(forms, right): (usize, usize)| {
        if forms == 0 {
            String::new()
        } else {
            #[allow(clippy::cast_precision_loss)]
            let p = 100.0 * right as f64 / forms as f64;
            format!("{p:.2}% ({forms})")
        }
    };
    // Empty where nothing is scored (English nouns given their plural:
    // nothing is left).
    let accuracy = |s: &Option<Score>| {
        s.as_ref()
            .filter(|s| s.forms > 0)
            .map_or(String::new(), |s| format!("{:.2}%", 100.0 * s.accuracy()))
    };
    for m in measured.iter() {
        let p = m.headword.accuracy();
        let headline = match &m.small {
            Some(_) if m.reproduced.forms == 0 => "–".to_string(),
            Some(small) => format!(
                "**reproduced {} of {}**; a new word {}",
                m.reproduced.right,
                m.reproduced.forms,
                match small.left_out.forms {
                    0 => "–".to_string(),
                    _ => format!(
                        "{:.2}% ({} word{}{})",
                        100.0 * small.left_out.accuracy(),
                        small.left_out.words,
                        if small.left_out.words == 1 { "" } else { "s" },
                        if small.by_tenth { ", by tenth" } else { "" }
                    ),
                }
            ),
            None => format!(
                "**{:.2}%** ({:.1}–{:.1})",
                100.0 * p,
                100.0 * m.interval.0,
                100.0 * m.interval.1
            ),
        };
        if m.headword.forms == 0 {
            let none = vec!["–"; 19].join(" | ");
            let _ = writeln!(
                s,
                "| {} | {} | 0 | {headline} | {none} | {:.2}% |",
                m.name,
                m.test_words,
                100.0 * m.irregular_share
            );
            continue;
        }
        let given = if m.given.iter().all(|s| s.forms == 0) {
            String::new()
        } else {
            m.given
                .iter()
                .map(|s| format!("{:.2}", 100.0 * s.accuracy()))
                .collect::<Vec<_>>()
                .join(" / ")
        };
        let f = &m.against_floor;
        let _ = writeln!(
            s,
            "| {} | {} | {} | {headline} | {:.2}% | {:.2}% | {:.2}% | {:.3} | {:.3} | {:.2}% | {} | {} | {} | {} | {} | {:.2}% ({:.2}%) | {:+.2} ({:+.2}…{:+.2}) | {:.2}% | {:.2}% | {:.2}% / {:.2}% | {} | {} | {} | {:.2}% |",
            m.name,
            m.test_words,
            m.headword.forms,
            100.0 * m.headword.top3_accuracy(),
            100.0 * m.headword.among_accuracy(),
            100.0 * m.headword.whole_words(),
            m.headword.calibration_error(),
            m.headword.log_loss(),
            100.0 * m.full.accuracy(),
            accuracy(&m.citation),
            given,
            accuracy(&m.features),
            share(m.headword.seen),
            share(m.headword.unseen),
            100.0 * m.floor,
            100.0 * m.floor_unseen,
            100.0 * (f.b - f.a),
            100.0 * f.low,
            100.0 * f.high,
            100.0 * m.headword.any_accuracy(),
            100.0 * m.ceiling,
            100.0 * m.on_training.accuracy(),
            100.0 * m.new_root.accuracy(),
            share(m.headword.lemma),
            m.by_entry
                .iter()
                .map(|s| if s.words == 0 {
                    "–".to_string()
                } else {
                    format!("{:.2}% ({})", 100.0 * s.accuracy(), s.words)
                })
                .collect::<Vec<_>>()
                .join(" / "),
            share((m.phrases.forms, m.phrases.right)),
            100.0 * m.irregular_share,
        );
    }
    let compatible: Vec<(&Measured, &Compatible)> = measured
        .iter()
        .filter(|m| m.headword.forms > 0)
        .filter_map(|m| Some((m, m.compatible.as_ref()?)))
        .collect();
    if !compatible.is_empty() {
        s.push_str(
            "\n*Compatible with stated length*: the dictionary marks a vowel's length both ways,\n\
             long and short, so a vowel it writes with neither has its length unstated where the\n\
             word's table leaves that vowel bare in every form, and stated where the table marks it\n\
             elsewhere (a circumflex marks it long). An answer that is the dictionary's form but for a\n\
             length mark on a vowel the form leaves bare is compatible with it where the table never\n\
             states that vowel's length otherwise. A second score, for the grammar and the floor alike,\n\
             beside top-1, which stays the score: the forms it adds are not more forms right.\n\n\
             | | top-1 | compatible with stated length | floor | floor, compatible | against the floor, compatible |\n\
             |---|---:|---:|---:|---:|---:|\n",
        );
        for (m, c) in compatible {
            let f = &c.against_floor;
            let _ = writeln!(
                s,
                "| {} | {:.2}% | {:.2}% | {:.2}% | {:.2}% | {:+.2} ({:+.2}…{:+.2}) |",
                m.name,
                100.0 * m.headword.accuracy(),
                100.0 * m.headword.compatible_accuracy(),
                100.0 * m.floor,
                100.0 * c.floor,
                100.0 * (f.b - f.a),
                100.0 * f.low,
                100.0 * f.high,
            );
        }
    }
    s.push_str(
        "\n## Where the errors are\n\n\
         The held-out forms the grammar gets wrong from the headword, by what went wrong, as a share\n\
         of them: *class*, the right form is among its answers but another paradigm's comes first;\n\
         *marks*, the right letters but for a mark the spelling folds (a vowel's length, a\n\
         breathing; not a letter written with a combining mark, as a Devanagari vowel sign);\n\
         *accent*, the right letters with the accent wrong; *stem* and *ending*, wrong letters\n\
         inside the stem or after it; *variant*, another of the forms the dictionary lists comes\n\
         first; *homograph*, another dictionary word spelled the same has that form; *gap*, no form\n\
         at all. *By block*: the mean of each group of cells' top-1, so a group of many cells\n\
         doesn't outweigh the rest; a group with fewer than 30 held-out forms is left out of it and\n\
         named below. *Reproduced*: the shipped grammar on the dictionary's own words (at most\n\
         3,000), from the headword; *complete*: of those words' cells, the dictionary's or not, the\n\
         share with a form.\n\n\
         | | wrong forms | class | marks | accent | stem | ending | variant | homograph | gap | by block | reproduced | complete |\n\
         |---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|\n",
    );
    for m in measured {
        let errors = &m.headword.errors;
        let wrong: usize = errors.iter().sum();
        let of = |k: crate::measure::Kind| {
            if wrong == 0 {
                return "–".to_string();
            }
            let n = crate::measure::Kind::ALL
                .iter()
                .position(|&x| x == k)
                .and_then(|i| errors.get(i))
                .copied()
                .unwrap_or(0);
            #[allow(clippy::cast_precision_loss)]
            let share = 100.0 * n as f64 / wrong.max(1) as f64;
            format!("{share:.0}%")
        };
        use crate::measure::Kind as K;
        // `part` of `all`, as a percentage; – where there is none.
        let percent = |part: usize, all: usize| {
            #[allow(clippy::cast_precision_loss)]
            let p = 100.0 * part as f64 / all.max(1) as f64;
            if all == 0 {
                "–".to_string()
            } else {
                format!("{p:.2}%")
            }
        };
        let wrong = if m.headword.forms == 0 {
            "–".to_string()
        } else {
            wrong.to_string()
        };
        let _ = writeln!(
            s,
            "| {} | {wrong} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |",
            m.name,
            of(K::Paradigm),
            of(K::Marks),
            of(K::Accent),
            of(K::Stem),
            of(K::Ending),
            of(K::Variant),
            of(K::Homograph),
            of(K::Gap),
            m.by_block
                .map_or("–".to_string(), |b| format!("{:.2}%", 100.0 * b)),
            percent(m.reproduced.right, m.reproduced.forms),
            percent(m.complete.1, m.complete.0),
        );
    }
    let blocks: Vec<String> = measured
        .iter()
        .filter(|m| m.blocks.iter().filter(|(_, b)| b.forms > 0).count() > 1)
        .map(|m| format!("- {}: {}", m.name, blocks(&m.blocks)))
        .collect();
    if !blocks.is_empty() {
        s.push_str("\n*By block*, each group of cells' top-1 and held-out forms:\n\n");
        s.push_str(&blocks.join("\n"));
        s.push('\n');
    }
    s.push_str(
        "\n## The shipped grammar and the scored one\n\n\
         The grammar shipped is learned from every word; the one scored above from all but the\n\
         held-out tenth. Their sound laws (each with its tier) and irregular forms, side by side;\n\
         a law one has and the other doesn't is listed below the table.\n\n\
         | | sound laws, shipped / scored | the same laws | irregular forms, shipped / scored |\n\
         |---|---:|---:|---:|\n",
    );
    let mut differ = Vec::new();
    for (m, g) in measured.iter().zip(learned) {
        let shipped: Vec<String> = g.laws.iter().map(ToString::to_string).collect();
        let sides = law_sides(&shipped, &m.scored_laws);
        let same = sides.iter().filter(|(mark, _)| *mark == '=').count();
        let _ = writeln!(
            s,
            "| {} | {} / {} | {same} | {} / {} |",
            m.name,
            shipped.len(),
            m.scored_laws.len(),
            g.stats.irregular_forms,
            m.scored_irregular,
        );
        let only = |side: char| -> Vec<String> {
            sides
                .iter()
                .filter(|(mark, _)| *mark == side)
                .map(|(_, law)| format!("`{law}`"))
                .collect()
        };
        let (shipped_only, scored_only) = (only('+'), only('−'));
        if !shipped_only.is_empty() || !scored_only.is_empty() {
            let mut line = format!("- {}:", m.name);
            if !shipped_only.is_empty() {
                let _ = write!(
                    line,
                    " only the shipped grammar's: {}.",
                    shipped_only.join(", ")
                );
            }
            if !scored_only.is_empty() {
                let _ = write!(
                    line,
                    " Only the scored grammar's: {}.",
                    scored_only.join(", ")
                );
            }
            differ.push(line);
        }
    }
    if !differ.is_empty() {
        s.push('\n');
        s.push_str(&differ.join("\n"));
        s.push('\n');
    }
    s.push('\n');
    s
}

/// Each block of cells' top-1 and forms, those with fewer than
/// [`eval::BLOCK_FORMS`] (not in the mean) apart: `name 93.10% (2000), …;
/// under 30 forms: name 50.00% (4)`.
pub fn blocks(blocks: &[(String, Score)]) -> String {
    let each = |few: bool| -> Vec<String> {
        blocks
            .iter()
            .filter(|(_, b)| b.forms > 0 && (b.forms < eval::BLOCK_FORMS) == few)
            .map(|(name, b)| format!("{name} {:.2}% ({})", 100.0 * b.accuracy(), b.forms))
            .collect()
    };
    let (enough, few) = (each(false), each(true));
    let mut out = enough.join(", ");
    if !few.is_empty() {
        if !out.is_empty() {
            out.push_str("; ");
        }
        let _ = write!(
            out,
            "under {} forms (not in the mean): {}",
            eval::BLOCK_FORMS,
            few.join(", ")
        );
    }
    out
}
