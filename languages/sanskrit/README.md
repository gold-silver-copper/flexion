# sanskrit

Sanskrit inflection for **any** word, attested or not, in Devanagari: nouns and
adjectives in eight cases and three numbers, and verbs (present system with its
four moods, imperfect, future, conditional, perfect, aorist, infinitive,
gerund, participles), with a confidence for every answer.

```rust
use sanskrit::*;

assert_eq!(noun("देव", Case::Instrumental, Number::Singular), "देवेन");
assert_eq!(noun("देव", Case::Genitive, Number::Plural), "देवानाम्");
assert_eq!(adjective("नव", Case::Nominative, Gender::Feminine, Number::Singular), "नवा");
let cell = PresentCell {
    mood: Mood::Indicative,
    voice: Voice::Active,
    person: Person::First,
    number: Number::Singular,
};
assert_eq!(present("निन्दति", cell), "निन्दामि");
```

Nouns and adjectives are cited by their stem, as Sanskrit dictionaries do; a
gender marker helps (`देव m`). Verbs are cited by the third person singular
present; a citation adds the future, perfect and aorist (`नयति, नेष्यति, निनाय,
अनैषीत्`). Lookup is by Devanagari only, not by romanization, and the forms are
Classical: Vedic variants and accents are left out. Sandhi between words is out
of scope; forms carry the sandhi inside the word.

## How it works

This crate is Sanskrit inflection as a grammar, learned by
[flexion](https://github.com/gold-silver-copper/flexion) from the English Wiktionary's
Sanskrit entries. Each part of speech is a module under `src/`:

- `paradigms.rs`: how words inflect, one table per paradigm, named after a word that
  follows it, with a pattern per form (`~а`: the stem, then `а`);
- `rules.rs`: which paradigm a word takes, as rules on its ending, each with how many of
  the dictionary's words with that ending follow each paradigm, and the words no rule
  gets right, listed;
- `irregular.rs`: the forms no paradigm gives.

A word the dictionary lists gets its own forms. Any other word gets the paradigm its
ending suggests, with a probability that says how often such a guess is right, and the
reason (the rule and its counts).

## The grammar

| | words | paradigms | rules | listed words | irregular forms |
|---|---:|---:|---:|---:|---:|
| noun | 5023 | 55 | 268 | 1045 | 1510 |
| adjective | 1964 | 36 | 122 | 401 | 2658 |
| verb | 1014 | 41 | 133 | 440 | 8437 |
| pronoun | 15 | 1 | 1 | 11 | 308 |

*Words*: the dictionary's words the grammar is learned from. *Rules*: the endings that
choose a paradigm. *Listed words*: words no rule gets right, listed with their paradigm.
*Irregular forms*: forms no paradigm gives, listed.

## How well it does

A tenth of the words (by a hash of the word) was held out: the grammar scored here was
learned from the other nine tenths, and asked for every form the dictionary lists of
each held-out word. *Top-1*: its first answer is right; *top-3*: the right answer is among
its first three. *Calibration error*: how far its stated probabilities are, on average,
from how often it is right (0 is perfect).

| | held-out words | forms | top-1 | top-3 | calibration error |
|---|---:|---:|---:|---:|---:|
| noun | 576 | 13801 | **91.80%** | 98.46% | 0.022 |
| adjective | 190 | 13456 | **91.52%** | 98.89% | 0.027 |
| verb | 88 | 7237 | **87.61%** | 88.77% | 0.102 |
| pronoun | 3 | 207 | **67.15%** | 67.15% | 0.329 |


## Data and license

The code is MIT or Apache-2.0. The grammar under `src/` is derived from Wiktionary via
[Kaikki](https://kaikki.org/) and [wiktextract](https://github.com/tatuylonen/wiktextract),
and is licensed under [CC BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/); see
`DATA-LICENSE.md`.

## Regenerating

```sh
cd flexion
cargo xtask extract sanskrit
cargo xtask build sanskrit
```
