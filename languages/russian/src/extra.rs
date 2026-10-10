//! Personal and reflexive pronouns. Wiktionary keeps all of them in one table
//! whose tags don't distinguish singular from plural (`я` and `мы` are both
//! "first-person nominative") and mix romanizations in as forms, so this is a
//! small fixed table rather than learned data.

use crate::{Case, Number, Person};

/// The gender of a third person singular pronoun.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PronounGender {
    Masculine,
    Feminine,
    Neuter,
}

/// A personal pronoun, stressed. Third person forms after a preposition take
/// `н-` (`у него́`): pass `after_preposition: true` for those.
///
/// ```
/// use russian::*;
/// assert_eq!(personal_pronoun(Person::First, Number::Singular, PronounGender::Masculine, Case::Genitive, false), "меня́");
/// assert_eq!(personal_pronoun(Person::Third, Number::Plural, PronounGender::Masculine, Case::Genitive, true), "них");
/// ```
pub fn personal_pronoun(
    person: Person,
    number: Number,
    gender: PronounGender,
    case: Case,
    after_preposition: bool,
) -> &'static str {
    // nominative, genitive, dative, accusative, instrumental, prepositional
    let (plain, prep): ([&'static str; 6], [&'static str; 6]) = match (person, number, gender) {
        (Person::First, Number::Singular, _) => (
            ["я", "меня́", "мне", "меня́", "мной", "мне"],
            ["я", "меня́", "мне", "меня́", "мной", "мне"],
        ),
        (Person::Second, Number::Singular, _) => (
            ["ты", "тебя́", "тебе́", "тебя́", "тобо́й", "тебе́"],
            ["ты", "тебя́", "тебе́", "тебя́", "тобо́й", "тебе́"],
        ),
        (Person::Third, Number::Singular, PronounGender::Masculine) => (
            ["он", "его́", "ему́", "его́", "им", "нём"],
            ["он", "него́", "нему́", "него́", "ним", "нём"],
        ),
        (Person::Third, Number::Singular, PronounGender::Neuter) => (
            ["оно́", "его́", "ему́", "его́", "им", "нём"],
            ["оно́", "него́", "нему́", "него́", "ним", "нём"],
        ),
        (Person::Third, Number::Singular, PronounGender::Feminine) => (
            ["она́", "её", "ей", "её", "ей", "ней"],
            ["она́", "неё", "ней", "неё", "ней", "ней"],
        ),
        (Person::First, Number::Plural, _) => (
            ["мы", "нас", "нам", "нас", "на́ми", "нас"],
            ["мы", "нас", "нам", "нас", "на́ми", "нас"],
        ),
        (Person::Second, Number::Plural, _) => (
            ["вы", "вас", "вам", "вас", "ва́ми", "вас"],
            ["вы", "вас", "вам", "вас", "ва́ми", "вас"],
        ),
        (Person::Third, Number::Plural, _) => (
            ["они́", "их", "им", "их", "и́ми", "них"],
            ["они́", "них", "ним", "них", "ни́ми", "них"],
        ),
    };
    let row = if after_preposition { prep } else { plain };
    let [nom, gen_, dat, acc, ins, prp] = row;
    match case {
        Case::Nominative => nom,
        Case::Genitive => gen_,
        Case::Dative => dat,
        Case::Accusative => acc,
        Case::Instrumental => ins,
        Case::Prepositional => prp,
    }
}

/// The reflexive pronoun `себя́`. It has no nominative; asking for one
/// returns `себя́`, the form it is cited by.
pub fn reflexive_pronoun(case: Case) -> &'static str {
    match case {
        Case::Nominative | Case::Genitive | Case::Accusative => "себя́",
        Case::Dative | Case::Prepositional => "себе́",
        Case::Instrumental => "собо́й",
    }
}
