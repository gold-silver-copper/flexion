//! The README's report of a build: the grammar's size and how well it does,
//! from what the build learned and measured. Only formatting, so left out of
//! the key a saved build is kept under (see [`crate::saved`]): a change here
//! reprints from the saved build.

use crate::eval::Score;
use crate::learn::Learned;
use crate::pipeline::Measured;
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
         developed (changes are judged on another tenth since PR #16).\n\n",
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
            "| {} | {} | {} | **{:.2}%** ({:.1}–{:.1}) | {:.2}% | {:.2}% | {:.2}% | {:.3} | {:.3} | {:.2}% | {} | {} | {} | {} | {} | {:.2}% ({:.2}%) | {:+.2} ({:+.2}…{:+.2}) | {:.2}% | {:.2}% | {:.2}% / {:.2}% | {} | {} | {} | {:.2}% |",
            m.name,
            m.test_words,
            m.headword.forms,
            100.0 * p,
            100.0 * m.interval.0,
            100.0 * m.interval.1,
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
    s.push_str(
        "\n## Where the errors are\n\n\
         The held-out forms the grammar gets wrong from the headword, by what went wrong, as a share\n\
         of them: *class*, the right form is among its answers but another paradigm's comes first;\n\
         *length* and *accent*, the right letters with a vowel's length or the accent wrong; *stem*\n\
         and *ending*, wrong letters inside the stem or after it; *variant*, another of the forms\n\
         the dictionary lists comes first; *homograph*, another dictionary word spelled the same has\n\
         that form; *gap*, no form at all. *By block*: the mean of each group of cells' top-1, so a\n\
         group of many cells doesn't outweigh the rest. *Reproduced*: the shipped grammar on the\n\
         dictionary's own words (at most 3,000), from the headword; *complete*: of those words'\n\
         cells, the dictionary's or not, the share with a form.\n\n\
         | | wrong forms | class | length | accent | stem | ending | variant | homograph | gap | by block | reproduced | complete |\n\
         |---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|\n",
    );
    for m in measured {
        let errors = &m.headword.errors;
        let wrong: usize = errors.iter().sum();
        let of = |k: crate::measure::Kind| {
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
        #[allow(clippy::cast_precision_loss)]
        let complete = 100.0 * m.complete.1 as f64 / m.complete.0.max(1) as f64;
        let _ = writeln!(
            s,
            "| {} | {wrong} | {} | {} | {} | {} | {} | {} | {} | {} | {:.2}% | {:.2}% | {complete:.2}% |",
            m.name,
            of(K::Paradigm),
            of(K::Length),
            of(K::Accent),
            of(K::Stem),
            of(K::Ending),
            of(K::Variant),
            of(K::Homograph),
            of(K::Gap),
            100.0 * m.by_block,
            100.0 * m.reproduced.accuracy(),
        );
    }
    s.push('\n');
    s
}
