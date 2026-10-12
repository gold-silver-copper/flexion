# latin

Latin inflection for **any** Latin word, attested or not: nouns, adjectives and
participles, verbs (every finite form, imperatives, infinitives, participles,
gerund, supine), with long vowels marked, and a confidence for every answer.

```rust
use latin::*;

assert_eq!(noun("rosa", Case::Ablative, Number::Plural), "rosīs");
assert_eq!(noun("rex", Case::Genitive, Number::Singular), "rēgis");
assert_eq!(noun("rēx, rēgis m.", Case::Dative, Number::Plural), "rēgibus");
let cell = VerbCell {
    mood: Mood::Indicative,
    voice: Voice::Active,
    tense: Tense::Perfect,
    person: Person::Third,
    number: Number::Plural,
};
assert_eq!(verb("amo", cell), "amāvērunt");
assert_eq!(adjective("bonus", Case::Dative, Gender::Feminine, Number::Singular), "bonae");

// Without macrons:
let plain = Latin::default().macrons(false);
assert_eq!(plain.noun("rosa", Case::Ablative, Number::Plural), "rosis");
```

A citation gives principal parts the way a dictionary prints them: nouns
`rēx, rēgis m.` (nominative, genitive, gender), verbs `amō, amāre, amāvī, amātum`,
adjectives `bonus, bona, bonum`.

**Vowel length.** A word typed without macrons is found anyway, and an attested
word comes back with its long vowels marked. For a word the model never saw,
though, the length of its stem vowels can't be known from the plain spelling:
`thesaurus` doesn't say that its `e` is long. So give such words with macrons, or
as a citation (`thēsaurus, thēsaurī`), whose parts spell the stem in full. Parts
are taken as written, so write them with their macrons. The report below scores all
three ways.

## How it works

This crate is Latin inflection as a grammar, learned by
[flexion](https://github.com/gold-silver-copper/flexion) from the English Wiktionary's
Latin entries. Each part of speech is a module under `src/`:

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
| noun | 15893 | 83 | 400 | 1199 | 10694 |
| adjective | 27464 | 47 | 175 | 474 | 106084 |
| verb | 6171 | 181 | 472 | 949 | 36820 |
| pronoun | 86 | 22 | 16 | 60 | 1286 |
| numeral | 118 | 9 | 11 | 5 | 6 |

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
| noun | 1565 | 18407 | **93.25%** | 98.30% | 0.031 |
| adjective | 2841 | 102282 | **98.79%** | 99.64% | 0.024 |
| verb | 619 | 60971 | **87.02%** | 96.98% | 0.048 |
| pronoun | 9 | 270 | **34.81%** | 50.74% | 0.521 |
| numeral | 11 | 396 | **100.00%** | 100.00% | 0.000 |


## Data and license

The code is MIT or Apache-2.0. The grammar under `src/` is derived from Wiktionary via
[Kaikki](https://kaikki.org/) and [wiktextract](https://github.com/tatuylonen/wiktextract),
and is licensed under [CC BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/); see
`DATA-LICENSE.md`.

## Regenerating

```sh
cd flexion
cargo xtask extract latin
cargo xtask build latin
```
