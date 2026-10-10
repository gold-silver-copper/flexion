//! `english` 0.4 (hand-written rules plus Wiktionary exception tables) as a
//! baseline: the same question the learned model answers, asked of the old
//! library. Cells use the learned crate's names (`noun.pl`, `verb.past`, …).

use english_core::EnglishCore;
use english_core::grammar::{Degree, Form, Number, Person, Tense};
use english_old::English;

/// Which part of the old library to ask.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Old {
    /// `english-core` alone: the regular rules, no tables.
    Rules,
    /// `english`: tables first, then the rules (the bare-lemma call).
    Library,
}

/// The old library's answer for a cell, or `None` for cells it has no notion of.
pub fn inflect(which: Old, cell: &str, lemma: &str) -> Option<String> {
    let verb = |tense: Tense, form: Form| match which {
        Old::Rules => EnglishCore::verb(lemma, &Person::Third, &Number::Singular, &tense, &form),
        Old::Library => English::verb(lemma, &Person::Third, &Number::Singular, &tense, &form),
    };
    Some(match (cell, which) {
        ("noun.pl", Old::Rules) => EnglishCore::pluralize_noun(lemma),
        ("noun.pl", Old::Library) => English::noun(lemma, &Number::Plural),
        ("verb.3sg", _) => verb(Tense::Present, Form::Finite),
        ("verb.past", _) => verb(Tense::Past, Form::Finite),
        ("verb.prp", _) => verb(Tense::Present, Form::Participle),
        ("verb.pp", _) => verb(Tense::Past, Form::Participle),
        ("adjective.comp", Old::Rules) => EnglishCore::comparative(lemma),
        ("adjective.sup", Old::Rules) => EnglishCore::superlative(lemma),
        ("adjective.comp", Old::Library) => English::adj(lemma, &Degree::Comparative),
        ("adjective.sup", Old::Library) => English::adj(lemma, &Degree::Superlative),
        ("adverb.comp", Old::Rules) => EnglishCore::comparative_adverb(lemma),
        ("adverb.sup", Old::Rules) => EnglishCore::superlative_adverb(lemma),
        ("adverb.comp", Old::Library) => English::adverb(lemma, &Degree::Comparative),
        ("adverb.sup", Old::Library) => English::adverb(lemma, &Degree::Superlative),
        _ => return None,
    })
}
