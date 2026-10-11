//! Training with fixed parameters instead of the tuned build's choices, for
//! experiments (`cargo xtask probe`) and the ceiling measurements
//! (`tools/ceiling`): every part of speech gets the same [`Params`].

use crate::extract::{PosData, Word};
use crate::learn::{Params, Stats};
use crate::spec::Language;
use crate::{DEV_BUCKET, PosPlan, TEST_BUCKET, bucket, extract, percentile};

/// How a probe trains.
#[derive(Debug, Clone)]
pub struct Probe {
    pub params: Params,
    /// Train on this share of the training words only.
    pub fraction: f64,
    /// Choose part-model sources only for this part of speech.
    pub only: Option<String>,
    /// Train on every word, the held-out ones too.
    pub attested: bool,
    /// Choose which cells principal parts build, on the dev tenth.
    pub parts: bool,
}

impl Probe {
    /// Read the probe flags from `args` (`--alpha 2 --tau 1 --support 1
    /// --syllables 2 --initial 0 --epsilon e --variant v --joins 1 --front 1
    /// --parts 1 --attested 1 --fraction 0.5 --pos noun`); others are ignored.
    pub fn from_args(args: &[String]) -> Result<Probe, String> {
        let flag = |name: &str| -> Option<&str> {
            args.windows(2)
                .find(|w| w.first().is_some_and(|f| f == name))
                .and_then(|w| w.get(1))
                .map(String::as_str)
        };
        let num = |name: &str| -> Result<Option<f64>, String> {
            flag(name)
                .map(|v| v.parse().map_err(|_| format!("bad {name}")))
                .transpose()
        };
        let mut params = Params::default();
        if let Some(a) = num("--alpha")? {
            params.alpha = a;
        }
        if let Some(t) = num("--tau")? {
            params.tau = t;
        }
        if let Some(e) = num("--epsilon")? {
            params.epsilon = e;
        }
        if let Some(v) = num("--variant")? {
            params.variant_decay = v;
        }
        if let Some(m) = flag("--support") {
            params.min_support = m.parse().map_err(|_| "bad --support")?;
        }
        if let Some(d) = flag("--syllables") {
            params.walk.syllable_depth = Some(d.parse().map_err(|_| "bad --syllables")?);
        }
        if let Some(d) = flag("--initial") {
            params.walk.initial_depth = Some(d.parse().map_err(|_| "bad --initial")?);
        }
        params.joins = flag("--joins").is_some();
        params.front_restore = flag("--front").is_some();
        Ok(Probe {
            params,
            fraction: num("--fraction")?.unwrap_or(1.0),
            only: flag("--pos").map(str::to_string),
            attested: flag("--attested").is_some(),
            parts: flag("--parts").is_some(),
        })
    }
}

/// Train every part of speech with the probe's parameters on the training
/// words (all words with `attested`).
pub fn train(
    lang: &Language,
    data: &[PosData],
    probe: &Probe,
) -> Result<(flexion::Built, Vec<Stats>), String> {
    let shapes = crate::shapes(lang)?;
    let spec = crate::spelling_spec(lang);
    let sp = crate::view(&spec);
    let params = probe.params;
    let mut plans: Vec<PosPlan> = Vec::new();
    for ((part, shape), d) in lang.parts.iter().zip(&shapes).zip(data) {
        let cells = crate::cells::Cells::new(lang, part)?;
        let citation: Vec<usize> = part
            .citation
            .iter()
            .filter_map(|(g, v)| cells.find(g, v))
            .collect();
        let skip = probe.only.as_deref().is_some_and(|o| o != part.name);
        let sources = if probe.parts && !skip {
            let marker_names = lang.markers(part);
            let markers: Vec<&[&str]> = marker_names.iter().map(Vec::as_slice).collect();
            let words = extract::words(d);
            let train: Vec<Word> = extract::words(d)
                .into_iter()
                .filter(|w| bucket(w.key) != TEST_BUCKET && bucket(w.key) != DEV_BUCKET)
                .collect();
            let dev: Vec<Word> = words
                .into_iter()
                .filter(|w| bucket(w.key) == DEV_BUCKET)
                .collect();
            let chooser = crate::edit::Chooser::new(&sp, &train, shape.lemma_cell);
            let s = crate::pipeline::choose_sources(
                &spec,
                shape,
                params,
                &train,
                &dev,
                &chooser,
                &citation,
                &markers,
                crate::pipeline::SIZE_PRICE,
            );
            eprintln!(
                "{}: {} cells from principal parts",
                part.name,
                s.iter().filter(|&&x| x > 0).count()
            );
            s
        } else {
            Vec::new()
        };
        plans.push(PosPlan {
            shape: shape.clone(),
            params,
            citation,
            sources,
        });
    }
    let share = (probe.fraction * 100.0).round() as u64;
    let attested = probe.attested;
    let (built, stats) = crate::train(lang, data, &plans, &|k| {
        attested || (bucket(k) != TEST_BUCKET && percentile(k) < share)
    });
    Ok((built.ok_or("the tables make no model")?, stats))
}
