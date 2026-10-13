//! Agreement with the dictionary editors' own analyses: what their
//! inflection templates and tags say about each word (Latin `la-conj`'s
//! class, Russian Zaliznyak types and stress letters, Greek `grc-conj`'s
//! tense stems, deponents, Russian adjectival nouns), read here, in the
//! measuring tool, and compared with how the learned grammar groups the
//! same words. The learner never sees any of it (see
//! [`flexion_train::extract::Lexeme::editors`]).

use flexion_train::extract::{Editors, Lexeme};
use flexion_train::learn::Learned;
use flexion_train::measure::{self, plain};
use flexion_train::spec::{Language, PartOfSpeech};
use std::collections::BTreeMap;

/// What the editors say about one word: `(kind, value)` pairs (`("class",
/// "3")`, `("deponent", "yes")`). Words the editors' templates don't
/// describe get none.
pub fn labels(lang: &str, pos: &str, e: &Editors) -> Vec<(&'static str, String)> {
    let mut out = Vec::new();
    // What a Latin template's `<…>` says: its declension, before any `.`.
    let angled = |v: &str| {
        let inside = v.split_once('<')?.1.split_once('>')?.0;
        let class = inside.split('.').next().unwrap_or(inside);
        Some(class.to_string())
    };
    match (lang, pos) {
        ("latin", "noun") => {
            if let Some(c) = e.template("la-ndecl").find_map(|a| angled(arg(a, "1")?)) {
                out.push(("declension", c));
            }
        }
        ("latin", "adjective") => {
            if let Some(a) = e.template("la-adecl").next()
                && let Some(v) = arg(a, "1")
            {
                // Without `<…>` the module reads the class off the lemma's
                // ending.
                let c = angled(v).unwrap_or_else(|| {
                    let p: Vec<char> = plain(v).chars().collect();
                    let tail: String = p.iter().skip(p.len().saturating_sub(2)).collect();
                    format!("by -{tail}")
                });
                out.push(("declension", c));
            }
        }
        ("latin", "verb") => {
            if let Some(a) = e.template("la-conj").next()
                && let Some(v) = arg(a, "1")
            {
                let class = v.split('.').next().unwrap_or(v).trim_end_matches('+');
                out.push(("conjugation", class.to_string()));
                out.push(("deponent", deponent(e)));
            }
        }
        ("ancient-greek", "verb") => {
            let firsts: Vec<&str> = e.template("grc-conj").filter_map(|a| arg(a, "1")).collect();
            let present = firsts.iter().find(|v| v.starts_with("pres"));
            let aorist = firsts.iter().find(|v| v.starts_with("aor"));
            let named = e
                .templates
                .iter()
                .find_map(|(n, _)| n.strip_prefix("grc-conj-"));
            if let Some(n) = named {
                out.push(("conjugation", n.to_string()));
            } else if present.is_some() || aorist.is_some() {
                let c = [present, aorist]
                    .iter()
                    .flatten()
                    .map(|s| **s)
                    .collect::<Vec<&str>>()
                    .join(" ");
                out.push(("conjugation", c));
            }
            if !firsts.is_empty() || named.is_some() {
                out.push(("deponent", deponent(e)));
            }
        }
        ("russian", "noun") => {
            let a = e
                .template("ru-noun+")
                .next()
                .or_else(|| e.template("ru-noun-table").next());
            if let Some(a) = a {
                let positional: Vec<&str> = (1..=4).map_while(|i| arg(a, &i.to_string())).collect();
                // `[stress letter] lemma declension …`: only the first may
                // be a letter (a declension `f` is the feminine's).
                let letter = positional.first().filter(|v| stress_letter(v));
                let rest = positional
                    .get(usize::from(letter.is_some())..)
                    .unwrap_or(&[]);
                let lemma = rest.first().copied();
                let declension = rest.get(1).copied().unwrap_or("");
                let stress = match letter {
                    Some(l) => (*l).to_string(),
                    None => lemma.map_or("a", default_letter).to_string(),
                };
                out.push(("stress", stress));
                let adjectival = if declension.starts_with('+') {
                    "yes"
                } else {
                    "no"
                };
                out.push(("adjectival", adjectival.to_string()));
            }
        }
        ("russian", "verb") => {
            if let Some(v) = e.template("ru-conj").find_map(|a| arg(a, "2")) {
                let v = v.split('+').next().unwrap_or(v);
                let digits: String = v.chars().take_while(|c| c.is_ascii_digit()).collect();
                let letter: String = v
                    .chars()
                    .skip_while(|c| c.is_ascii_digit() || *c == '°')
                    .collect();
                if !digits.is_empty() {
                    out.push(("zaliznyak", v.to_string()));
                    out.push(("type", digits));
                    out.push(("stress", letter));
                }
            }
        }
        ("russian", "adjective") => {
            if let Some(v) = e.template("ru-decl-adj").find_map(|a| arg(a, "2")) {
                let letter = v.split(':').next().unwrap_or(v);
                if stress_letter(letter) {
                    out.push(("short-form stress", letter.to_string()));
                }
            }
        }
        ("sanskrit", "noun" | "adjective") => {
            let found = e.templates.iter().find_map(|(n, a)| {
                let gender = n.strip_prefix("sa-decl-noun-")?;
                let stem = plain(arg(a, "1")?);
                let last = stem.chars().last()?;
                Some(format!("{gender}-{last}"))
            });
            if let Some(c) = found {
                out.push(("stem class", c));
            }
        }
        _ => {}
    }
    out
}

/// Argument `k` of a template.
fn arg<'a>(a: &'a [(String, String)], k: &str) -> Option<&'a str> {
    a.iter().find(|(n, _)| n == k).map(|(_, v)| v.as_str())
}

fn deponent(e: &Editors) -> String {
    if e.has_tag("semi-deponent") {
        "semi".into()
    } else if e.has_tag("deponent") {
        "yes".into()
    } else {
        "no".into()
    }
}

/// A Zaliznyak stress letter or several (`a`, `b'`, `d',f`).
fn stress_letter(v: &str) -> bool {
    !v.is_empty()
        && v.split(',').all(|p| {
            let mut c = p.chars();
            c.next().is_some_and(|l| "abcdef".contains(l)) && c.all(|x| x == '\'' || x == '′')
        })
}

/// The letter `ru-noun+` gives a noun when none is written: `b` when the
/// lemma is stressed on a final vowel, else `a`.
fn default_letter(lemma: &str) -> &'static str {
    use unicode_normalization::UnicodeNormalization;
    let chars: Vec<char> = lemma.nfd().collect();
    let vowels = "аеёиоуыэюяАЕЁИОУЫЭЮЯ";
    let last_vowel = chars.iter().rposition(|c| vowels.contains(*c));
    let ends_in_vowel = chars
        .iter()
        .rposition(|c| c.is_alphabetic())
        .is_some_and(|i| Some(i) == last_vowel);
    let stressed = last_vowel.is_some_and(|i| {
        chars.get(i + 1) == Some(&'\u{301}') || chars.get(i + 1) == Some(&'\u{308}')
    });
    if ends_in_vowel && stressed { "b" } else { "a" }
}

/// A word's likeliest paradigm and the editors' labels of it.
type Labelled = (Option<usize>, Vec<(&'static str, String)>);

/// The training words agreement is measured on, at most.
const TRAINING_SAMPLE: usize = 5000;

/// Print how the learned grammar groups words (each word's likeliest
/// paradigm, read from the word in full) against each of the editors'
/// labels: adjusted Rand index, purity and inverse purity, on training
/// words and held-out words; and, for held-out words, how often the
/// commonest label of their paradigm's training words is theirs.
pub fn report(
    lang: &Language,
    part: &PartOfSpeech,
    learned: &Learned<'_>,
    train: &[Lexeme],
    test: &[&Lexeme],
) {
    let g = &learned.grammar;
    let full = flexion::Known {
        full: true,
        ..flexion::Known::default()
    };
    let paradigm = |l: &Lexeme| {
        g.readings(&l.full, &full)
            .first()
            .map(|r| flexion::Label::index(r.value))
    };
    let mains: Vec<&Lexeme> = train.iter().filter(|l| l.source == 0).collect();
    let step = mains.len().div_ceil(TRAINING_SAMPLE).max(1);
    let sample: Vec<&Lexeme> = mains.into_iter().step_by(step).collect();
    let labelled = |words: &[&Lexeme]| -> Vec<Labelled> {
        words
            .iter()
            .map(|l| (paradigm(l), labels(lang.name, part.name, &l.editors)))
            .filter(|(_, ls)| !ls.is_empty())
            .collect()
    };
    let (tr, te) = (labelled(&sample), labelled(test));
    let mut kinds: Vec<&'static str> = Vec::new();
    for (_, ls) in tr.iter().chain(&te) {
        for (k, _) in ls {
            if !kinds.contains(k) {
                kinds.push(k);
            }
        }
    }
    for kind in kinds {
        let pairs = |rows: &[Labelled]| {
            rows.iter()
                .filter_map(|(p, ls)| {
                    let v = ls.iter().find(|(k, _)| *k == kind)?;
                    Some((p.unwrap_or(usize::MAX), v.1.clone()))
                })
                .collect::<Vec<(usize, String)>>()
        };
        let (a, b) = (pairs(&tr), pairs(&te));
        let values: std::collections::BTreeSet<&String> = a.iter().map(|x| &x.1).collect();
        if values.len() < 2 {
            continue;
        }
        let at = measure::agreement(&a);
        let ah = measure::agreement(&b);
        // Each paradigm's commonest label among training words.
        let mut by: BTreeMap<usize, BTreeMap<&String, usize>> = BTreeMap::new();
        for (p, v) in &a {
            *by.entry(*p).or_default().entry(v).or_default() += 1;
        }
        let commonest = |p: usize| {
            by.get(&p)
                .and_then(|m| m.iter().max_by(|x, y| x.1.cmp(y.1).then(y.0.cmp(x.0))))
                .map(|(v, _)| (*v).clone())
        };
        let told = b
            .iter()
            .filter(|(p, v)| commonest(*p).as_ref() == Some(v))
            .count();
        let mut counts: BTreeMap<&String, usize> = BTreeMap::new();
        for (_, v) in &a {
            *counts.entry(v).or_default() += 1;
        }
        let mut common: Vec<(&&String, &usize)> = counts.iter().collect();
        common.sort_by(|x, y| y.1.cmp(x.1).then(x.0.cmp(y.0)));
        let shown: Vec<String> = common
            .iter()
            .take(6)
            .map(|(v, n)| format!("{v} {n}"))
            .collect();
        println!(
            "  editors' {kind} ({} values: {}): training words {}: {} paradigms, adjusted Rand {:.3}, purity {:.3}, inverse purity {:.3}; held-out {}: adjusted Rand {:.3}, purity {:.3}, inverse purity {:.3}, paradigm's commonest {kind} right {:.2}%",
            values.len(),
            shown.join(", "),
            at.items,
            at.groups_a,
            at.rand,
            at.purity,
            at.inverse_purity,
            ah.items,
            ah.rand,
            ah.purity,
            ah.inverse_purity,
            100.0 * told as f64 / b.len().max(1) as f64,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn editors(templates: &[(&str, &[(&str, &str)])], tags: &[&str]) -> Editors {
        Editors {
            templates: templates
                .iter()
                .map(|(n, a)| {
                    (
                        n.to_string(),
                        a.iter()
                            .map(|(k, v)| (k.to_string(), v.to_string()))
                            .collect(),
                    )
                })
                .collect(),
            tags: tags.iter().map(|t| t.to_string()).collect(),
        }
    }

    #[test]
    fn the_editors_classes_are_read_from_their_templates() {
        let tango = editors(&[("la-conj", &[("1", "3.sigm"), ("2", "tangō")])], &[]);
        assert_eq!(
            labels("latin", "verb", &tango),
            vec![("conjugation", "3".into()), ("deponent", "no".into())]
        );
        let kot = editors(&[("ru-noun+", &[("1", "b"), ("a", "an")])], &[]);
        assert_eq!(
            labels("russian", "noun", &kot).first(),
            Some(&("stress", "b".into()))
        );
        // No letter written: an ending-stressed lemma is `b`.
        let vino = editors(&[("ru-noun+", &[("1", "вино́")])], &[]);
        assert_eq!(
            labels("russian", "noun", &vino).first(),
            Some(&("stress", "b".into()))
        );
        let stolovaya = editors(&[("ru-noun+", &[("1", "столо́вая"), ("2", "+")])], &[]);
        assert_eq!(
            labels("russian", "noun", &stolovaya).get(1),
            Some(&("adjectival", "yes".into()))
        );
        // A declension `f` is no stress letter.
        let tetrad = editors(&[("ru-noun+", &[("1", "тетра́дь"), ("2", "f")])], &[]);
        assert_eq!(
            labels("russian", "noun", &tetrad).first(),
            Some(&("stress", "a".into()))
        );
        let pisat = editors(&[("ru-conj", &[("1", "impf"), ("2", "6c+p")])], &[]);
        assert_eq!(
            labels("russian", "verb", &pisat).get(2),
            Some(&("stress", "c".into()))
        );
    }
}
