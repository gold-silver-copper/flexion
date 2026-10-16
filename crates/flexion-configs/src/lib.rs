//! One language description per module. A config is everything the trainer
//! knows about a language; adding a language means adding a module here.

pub mod ancient_greek;
pub mod english;
pub mod finnish;
pub mod french;
pub mod german;
pub mod italian;
pub mod latin;
pub mod polish;
pub mod romance;
pub mod russian;
pub mod sanskrit;
pub mod spanish;
pub mod turkish;

use flexion_train::spec::{Language, PartOfSpeech};

/// Adverbs in their degrees of comparison, from the head line (Latin
/// `bene`, `melius`, `optimē`): the language's `Degree` feature, the
/// positive the headword.
pub fn adverb() -> PartOfSpeech {
    PartOfSpeech {
        name: "adverb",
        doc: "Adverbs that compare (`melius`, `lepiej`).",
        wiktionary: vec!["adv"],
        templates: vec![],
        forms: flexion_train::spec::FormSource::All,
        groups: vec![flexion_train::spec::Group {
            name: "adverb",
            doc: "An adverb in a degree.",
            features: vec!["Degree"],
            all: vec![],
            any: vec![],
            none: vec![],
            from_head: false,
        }],
        lemma: Some(("adverb", vec!["Positive"])),
        citation: vec![("adverb", vec!["Comparative"])],
        inherent: vec![],
        capitalized: false,
        lemmas_from: vec![],
        max_words: None,
    }
}

/// `lang` with a small part `name` (pronouns, numerals) read from the
/// dictionary's `wiktionary` parts in the plain cells of its part `host`
/// (the group named after it), from tables of `templates` (the host's if
/// empty), after the host.
pub fn with_small(
    mut lang: Language,
    host: &str,
    name: &'static str,
    doc: &'static str,
    wiktionary: &[&'static str],
    templates: &[&'static str],
) -> Language {
    let Some(at) = lang.parts.iter().position(|p| p.name == host) else {
        return lang;
    };
    let Some(mut part) = lang.parts.get(at).map(|h| like(h, name, doc, wiktionary)) else {
        return lang;
    };
    part.groups.retain(|g| g.name == name);
    part.citation.clear();
    if !templates.is_empty() {
        part.templates = templates.to_vec();
    }
    lang.parts.insert(at + 1, part);
    lang
}

/// `lang` with its proper nouns (see [`proper_noun`]) after its nouns.
pub fn with_proper_nouns(mut lang: Language) -> Language {
    if let Some(at) = lang.parts.iter().position(|p| p.name == "noun")
        && let Some(names) = lang.parts.get(at).map(proper_noun)
    {
        lang.parts.insert(at + 1, names);
    }
    lang
}

/// A language's proper nouns, from the dictionary's names (`name`): in the
/// cells of its nouns, always capitalized, with rules of their own (names
/// choose otherwise than nouns: English names in a consonant and `y` take
/// `-ys`, `Kennedys`, 1,418 times against `-ies` 38).
pub fn proper_noun(noun: &PartOfSpeech) -> PartOfSpeech {
    PartOfSpeech {
        capitalized: true,
        ..like(
            noun,
            "proper_noun",
            "Proper nouns: names of people, places and peoples, declined as nouns are.",
            &["name"],
        )
    }
}

/// A part of speech `name` read from the dictionary's `wiktionary` parts,
/// in the cells of `host` (whose groups it takes, the host's own renamed
/// after it), with rules of its own.
pub fn like(
    host: &PartOfSpeech,
    name: &'static str,
    doc: &'static str,
    wiktionary: &[&'static str],
) -> PartOfSpeech {
    // Group names are a crate's functions: each the new part's own
    // (`noun_second_case` → `proper_noun_second_case`, `possessive` →
    // `proper_noun_possessive`).
    let rename = |group: &'static str| -> &'static str {
        if group == host.name {
            return name;
        }
        let rest = group
            .strip_prefix(host.name)
            .and_then(|r| r.strip_prefix('_'))
            .unwrap_or(group);
        Box::leak(format!("{name}_{rest}").into_boxed_str())
    };
    PartOfSpeech {
        name,
        doc,
        wiktionary: wiktionary.to_vec(),
        groups: host
            .groups
            .iter()
            .map(|g| flexion_train::spec::Group {
                name: rename(g.name),
                ..g.clone()
            })
            .collect(),
        lemma: host.lemma.clone().map(|(g, v)| (rename(g), v)),
        citation: host
            .citation
            .iter()
            .map(|(g, v)| (rename(g), v.clone()))
            .collect(),
        lemmas_from: vec![],
        max_words: None,
        ..host.clone()
    }
}

/// The languages whose crates are built and tested.
pub fn all() -> Vec<Language> {
    vec![
        english::language(),
        russian::language(),
        latin::language(),
        ancient_greek::language(),
        sanskrit::language(),
        polish::language(),
        german::language(),
        spanish::language(),
        french::language(),
        italian::language(),
        finnish::language(),
        turkish::language(),
    ]
}

/// A configured language by crate name.
pub fn by_name(name: &str) -> Option<Language> {
    all().into_iter().find(|l| l.name == name)
}
