# spanish

Spanish inflection for **any** Spanish word, attested or not: noun plurals,
adjective agreement, and the whole conjugation (every simple tense, both
imperfect subjunctives, voseo, imperatives, participles), each with how sure
the grammar is and why.

```rust
use spanish::*;

assert_eq!(noun("canción", Number::Plural), "canciones");
assert_eq!(noun("joven", Number::Plural), "jóvenes");
assert_eq!(adjective("feliz", Gender::Feminine, Number::Plural), "felices");
assert_eq!(verb("contar", Tense::Present, Person::First, Number::Singular), "cuento");
assert_eq!(verb("hacer", Tense::Preterite, Person::First, Number::Singular), "hice");
assert_eq!(vos("tener", Mood::Indicative), "tenés");
assert_eq!(participle("escribir", Gender::Feminine, Number::Plural), "escritas");
```

Written accents are part of the word: `papá` and `papa` are two words.

## How it works

This crate is Spanish inflection as a grammar, learned by
[flexion](https://github.com/gold-silver-copper/flexion) from the English Wiktionary's and the Spanish Wiktionary's
Spanish entries. Each part of speech is a module under `src/`, and has what its
grammar needs of:

- `paradigms.rs`: how words inflect, one table per paradigm, named after a word that
  follows it, with a pattern per form (`~а`: the stem, then `а`; `referred`: another
  cell's form, as the word's feature says);
- `rules.rs`: which paradigm a word takes, as rules on its ending, each with how many of
  the dictionary's words with that ending follow each paradigm, and the words no rule
  gets right, listed;
- `accent.rs`: per cell, which rule places the accent, and which words have which
  pattern;
- `prefixes.rs`: prefixes a word inflects behind, separable or not;
- `compounds.rs`: whether a word written in parts inflects each part;
- `marks.rs` and `spellings.rs`: how a word typed without its marks (vowel length,
  `ё`) is spelled;
- `irregular.rs`: the forms no paradigm gives;
- a module per principal part, where a part of speech is built on them: each cell
  belongs to one part's system and is inflected from that part.

A word the dictionary lists gets its own forms. Any other word gets the paradigm its
ending suggests, with a probability that says how often such a guess is right, and the
reason (the rule and its counts).

## The grammar

| | words | systems | paradigms | rules | listed words | irregular forms |
|---|---:|---:|---:|---:|---:|---:|
| noun | 60268 | 1 | 23 | 244 | 732 | 76 |
| adjective | 24552 | 3 | 41 | 656 | 1307 | 53 |
| verb | 17975 | 1 | 179 | 831 | 873 | 5331 |
| determiner | 60 | 1 | 12 | 7 | 18 | 0 |

*Words*: the dictionary's words the grammar is learned from. *Systems*: the principal
parts a part of speech is built on (1: its lemma alone). *Rules*: the endings that
choose a paradigm. *Listed words*: words no rule gets right, listed with their paradigm.
*Irregular forms*: forms no paradigm gives, listed.

## How well it does

A tenth of the words (by a hash of the word) was held out: the grammar scored here was
learned from the other nine tenths, and asked for every form the dictionary lists of
each held-out word, typed as the dictionary's headword. *Top-1*: its first answer is
right; *top-3*: the right answer is among its first three. *Calibration error*: how far
its stated probabilities are, on average, from how often it is right (0 is perfect).
*Top-1, lemma in full*: typed with every mark the dictionary's table writes, and said
to be so (`spelled_in_full`).

| | held-out words | forms | top-1 | top-3 | calibration error | top-1, lemma in full |
|---|---:|---:|---:|---:|---:|---:|
| noun | 6082 | 12164 | **99.30%** | 99.89% | 0.001 | 99.30% |
| adjective | 2402 | 9619 | **98.87%** | 99.82% | 0.003 | 98.87% |
| verb | 931 | 67865 | **98.81%** | 99.32% | 0.004 | 98.81% |
| determiner | 4 | 16 | **87.50%** | 100.00% | 0.062 | 87.50% |


## Data and license

The code is MIT or Apache-2.0. The grammar under `src/` is derived from Wiktionary via
[Kaikki](https://kaikki.org/) and [wiktextract](https://github.com/tatuylonen/wiktextract),
and is licensed under [CC BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/); see
`DATA-LICENSE.md`.

## Regenerating

```sh
cd flexion
cargo xtask extract spanish
cargo xtask build spanish
```
