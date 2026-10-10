//! Head-line forms, shared by the Spanish, French and Italian configs.
//!
//! These languages' nouns, adjectives and determiners have no inflection
//! tables: Wiktionary lists their forms on the head line (`gratuit (feminine
//! gratuite, masculine plural gratuits, feminine plural gratuites)`), next to
//! derived words (diminutives, adverbs, relational adjectives) and a noun's
//! counterpart of the other sex (`chien`, feminine `chienne`). The helpers
//! here keep the inflected forms and write out the ones a head line leaves
//! implicit: `(invariable)`, and adjectives with one form for both genders.

use flexion_train::kaikki::{Entry, Form};
use flexion_train::spec::Group;

/// Tags that name an inflected form.
const GRAMMAR: &[&str] = &[
    "masculine",
    "feminine",
    "singular",
    "plural",
    "superlative",
    "before-noun",
    "before-vowel",
];

fn has(f: &Form, tag: &str) -> bool {
    f.tags.iter().any(|t| t == tag)
}

fn head(f: &Form) -> bool {
    f.source.is_none() && !has(f, "canonical")
}

fn form(word: &str, tags: &[&str]) -> Form {
    Form {
        form: word.to_string(),
        tags: tags.iter().map(|t| t.to_string()).collect(),
        ..Form::default()
    }
}

/// Keep the head-line forms that name a grammatical feature and whose other
/// tags are all `register` tags (soft or excluded ones, which the tag policy
/// handles): `chienne` (feminine) stays for now, `chiot` (diminutive) goes.
fn keep_inflected(entry: &mut Entry, register: &[&str]) {
    let grammar = |t: &String| GRAMMAR.contains(&t.as_str());
    entry.forms.retain(|f| {
        !head(f)
            || (f.tags.iter().any(grammar)
                && f.tags
                    .iter()
                    .all(|t| grammar(t) || register.contains(&t.as_str())))
    });
}

/// Whether the head line calls the word invariable (`città f (invariable)`).
fn invariable(entry: &Entry) -> bool {
    entry
        .head_templates
        .first()
        .is_some_and(|h| h.expansion.contains("invariable"))
}

/// The word after `label` in the head line's parenthesis, with notes in
/// nested parentheses left out (`plural (in most meanings) braccia f or …`
/// → `braccia`): for the few heads Kaikki doesn't split into forms. Only a
/// word that starts like the lemma counts (not `plural not attested`).
fn head_word(entry: &Entry, label: &str) -> Option<String> {
    let expansion = &entry.head_templates.first()?.expansion;
    let (_, inner) = expansion.split_once('(')?;
    let mut depth = 0usize;
    let mut flat = String::new();
    for c in inner.chars() {
        match c {
            '(' => depth += 1,
            ')' if depth == 0 => break,
            ')' => depth -= 1,
            _ if depth == 0 => flat.push(c),
            _ => {}
        }
    }
    let first = entry.word.chars().next()?;
    flat.split(',')
        .find_map(|item| {
            let rest = item.trim().strip_prefix(label)?;
            rest.starts_with(' ')
                .then(|| rest.split_whitespace().next())
                .flatten()
        })
        .filter(|w| w.starts_with(first))
        .map(str::to_string)
}

/// A noun's head line: its plural only. A gendered plural counts only when
/// there is no plain one (`uovo`, plural `uova` f), since next to a plain
/// plural it is the counterpart's (`abuela`, masculine plural `abuelos`). An
/// invariable noun's plural is the lemma.
pub fn noun(entry: &mut Entry, register: &[&str]) {
    keep_inflected(entry, register);
    let plain = |f: &Form| has(f, "plural") && !has(f, "masculine") && !has(f, "feminine");
    let any_plain = entry.forms.iter().any(|f| head(f) && plain(f));
    entry
        .forms
        .retain(|f| !head(f) || (has(f, "plural") && (!any_plain || plain(f))));
    if !entry.forms.iter().any(|f| head(f) && has(f, "plural")) {
        let plural = if invariable(entry) {
            Some(entry.word.clone())
        } else {
            head_word(entry, "plural")
        };
        if let Some(p) = plural {
            entry.forms.push(form(&p, &["plural"]));
        }
    }
}

/// A noun's gender from its head line (`canción f`, `artista m or f by
/// sense`): one tag per gender named before the parenthesis.
pub fn noun_gender(entry: &Entry) -> Vec<String> {
    let Some(h) = entry.head_templates.first() else {
        return Vec::new();
    };
    let before = h.expansion.split('(').next().unwrap_or("");
    let rest = before.strip_prefix(entry.word.as_str()).unwrap_or(before);
    let words: Vec<&str> = rest.split_whitespace().collect();
    [("m", "masculine"), ("f", "feminine")]
        .iter()
        .filter(|(short, _)| words.contains(short))
        .map(|(_, tag)| tag.to_string())
        .collect()
}

/// An adjective's (or determiner's) head line: gender and number forms,
/// the superlative, and the masculine singular before a noun or a vowel.
///
/// - `before a noun buen` (Spanish writes it as one form string) becomes
///   `buen`, tagged `before-noun`.
/// - A plural without a gender is both genders' plural.
/// - An adjective with one form for both genders (`grande`, `facile`) lists
///   no feminine singular: it is the lemma. An invariable one's every form is.
pub fn adjective(entry: &mut Entry, register: &[&str]) {
    for f in entry.forms.iter_mut().filter(|f| head(f)) {
        if let Some(w) = f.form.strip_prefix("before a noun ") {
            f.form = w.to_string();
            f.tags = vec!["before-noun".to_string()];
        }
    }
    keep_inflected(entry, register);
    for f in entry.forms.iter_mut().filter(|f| head(f)) {
        if has(f, "plural") && !has(f, "masculine") && !has(f, "feminine") {
            f.tags.push("masculine".to_string());
            f.tags.push("feminine".to_string());
        }
    }
    let gendered = |e: &Entry| {
        e.forms
            .iter()
            .any(|f| head(f) && (has(f, "plural") || has(f, "feminine")))
    };
    if !gendered(entry) {
        if invariable(entry) {
            let w = entry.word.clone();
            entry
                .forms
                .push(form(&w, &["masculine", "feminine", "plural"]));
        } else if let Some(p) = head_word(entry, "plural") {
            entry
                .forms
                .push(form(&p, &["masculine", "feminine", "plural"]));
        }
    }
    let feminine = entry
        .forms
        .iter()
        .any(|f| head(f) && has(f, "feminine") && !has(f, "plural"));
    let epicene = entry
        .forms
        .iter()
        .any(|f| head(f) && has(f, "feminine") && has(f, "masculine"));
    if !feminine && epicene {
        let w = entry.word.clone();
        entry.forms.push(form(&w, &["feminine"]));
    }
}

/// Whether a determiner's head line inflects it for gender (a feminine or a
/// masculine plural form), read before [`adjective`] fills the implicit
/// forms: pronouns like `ella` or `menos` list only a plural or other
/// words, and are left out.
pub fn agrees(entry: &Entry) -> bool {
    entry.forms.iter().any(|f| {
        head(f)
            && ((has(f, "feminine") && !has(f, "masculine"))
                || (has(f, "masculine") && has(f, "plural")))
    })
}

/// A cell group with no `any` tags.
pub fn group(
    name: &'static str,
    doc: &'static str,
    features: &[&'static str],
    all: &[&'static str],
    none: &[&'static str],
) -> Group {
    Group {
        name,
        doc,
        features: features.to_vec(),
        all: all.to_vec(),
        any: vec![],
        none: none.to_vec(),
        optional: false,
        from_head: false,
    }
}

/// A verb table form written with separate pronouns (`me quejo`, `si lava`,
/// `se ne va`): the verb alone, when every word before it is one of
/// `pronouns`. Other multi-word forms are left for `form_ok` to reject.
pub fn strip_pronouns(entry: &mut Entry, pronouns: &[&str]) {
    for f in entry.forms.iter_mut().filter(|f| f.source.is_some()) {
        let mut words: Vec<&str> = f.form.split(' ').collect();
        let Some(last) = words.pop() else {
            continue;
        };
        if !words.is_empty() && words.iter().all(|w| pronouns.contains(w)) {
            f.form = last.to_string();
        }
    }
}

/// Keep only an entry's first inflection table: Spanish verb entries add
/// a table of clitic forms (`hacerme`) and one of the pronominal verb
/// (`hacerse`: `me hago`, `hazte`) without saying so in the tags.
pub fn first_table(entry: &mut Entry) {
    let mut tables = 0usize;
    entry.forms.retain(|f| {
        if f.source.is_some() && has(f, "inflection-template") {
            tables += 1;
        }
        f.source.is_none() || tables <= 1
    });
}
