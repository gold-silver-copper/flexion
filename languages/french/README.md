# french

French inflection for **any** French word, attested or not: noun plurals,
adjective and participle agreement, and the whole simple conjugation
(indicative, subjunctive, imperative, participles), each with how sure the
grammar is and why.

```rust
use french::*;

assert_eq!(noun("cheval", Number::Plural), "chevaux");
assert_eq!(adjective("beau", Gender::Feminine, Number::Singular), "belle");
assert_eq!(before_vowel("nouveau"), "nouvel");
assert_eq!(verb("appeler", Tense::Present, Person::First, Number::Singular), "appelle");
assert_eq!(verb("manger", Tense::Present, Person::First, Number::Plural), "mangeons");
assert_eq!(verb("prendre", Tense::PastHistoric, Person::Third, Number::Singular), "prit");
assert_eq!(agreeing_participle("prendre", Gender::Feminine, Number::Plural), "prises");
```

Accents and `ç` are letters: `pêcher` and `pécher` are two words.

## How it works

This crate is French inflection as a grammar, learned by
[flexion](https://github.com/gold-silver-copper/flexion) from the English Wiktionary's
French entries. Each part of speech is a module under `src/`, and has what its
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
| noun | 43092 | 1 | 24 | 274 | 676 | 284 |
| adjective | 18164 | 3 | 33 | 315 | 763 | 54 |
| verb | 7295 | 4 | 214 | 743 | 586 | 650 |
| participle | 6504 | 1 | 6 | 20 | 22 | 0 |
| determiner | 49 | 1 | 7 | 12 | 14 | 30 |

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
| noun | 4318 | 8636 | **98.81%** | 99.81% | 0.002 | 98.81% |
| adjective | 1839 | 7356 | **98.21%** | 99.73% | 0.003 | 98.21% |
| verb | 743 | 35550 | **98.53%** | 99.52% | 0.004 | 98.53% |
| participle | 627 | 2508 | **99.76%** | 100.00% | 0.005 | 99.76% |
| determiner | 6 | 19 | **63.16%** | 63.16% | 0.187 | 63.16% |


## Data and license

The code is MIT or Apache-2.0. The grammar under `src/` is derived from Wiktionary via
[Kaikki](https://kaikki.org/) and [wiktextract](https://github.com/tatuylonen/wiktextract),
and is licensed under [CC BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/); see
`DATA-LICENSE.md`.

## Regenerating

```sh
cd flexion
cargo xtask extract french
cargo xtask build french
```
