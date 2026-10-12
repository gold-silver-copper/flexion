# polish

Polish inflection for **any** Polish word, attested or not: noun declension,
adjectives, and verbs (non-past, past and conditional by person and gender,
imperative, participles), with a confidence for every answer.

```rust
use polish::*;

assert_eq!(noun("kot", Case::Genitive, Number::Singular), "kota");
assert_eq!(noun("ręka", Case::Locative, Number::Singular), "ręce");
assert_eq!(adjective("dobry", AdjectiveCase::Nominative, Agreement::Virile, Animacy::Animate), "dobrzy");
assert_eq!(non_past("czytać", Person::First, Number::Singular), "czytam");
assert_eq!(past("czytać", Person::First, Agreement::Feminine), "czytałam");
```

A citation gives principal parts the way a dictionary prints them: nouns
nominative and genitive singular (`dom, domu`: whether a masculine noun takes
`-a` or `-u` can't be predicted), verbs infinitive and first and second person
singular; markers name gender and aspect (`kot, kota manim`, `czytać impf`).

## How it works

This crate is Polish inflection as a grammar, learned by
[flexion](https://github.com/gold-silver-copper/flexion) from the English Wiktionary's
Polish entries. Each part of speech is a module under `src/`:

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
| noun | 37480 | 319 | 1507 | 5818 | 14603 |
| adjective | 13224 | 37 | 132 | 143 | 446 |
| verb | 12103 | 101 | 335 | 660 | 8499 |
| pronoun | 28 | 16 | 7 | 10 | 19 |
| numeral | 40 | 19 | 11 | 11 | 20 |

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
| noun | 4010 | 48482 | **92.80%** | 98.65% | 0.017 |
| adjective | 1394 | 83790 | **99.94%** | 99.99% | 0.000 |
| verb | 1253 | 55197 | **96.85%** | 98.69% | 0.015 |
| pronoun | 6 | 360 | **81.94%** | 81.94% | 0.253 |
| numeral | 4 | 16 | **75.00%** | 75.00% | 0.299 |


## Data and license

The code is MIT or Apache-2.0. The grammar under `src/` is derived from Wiktionary via
[Kaikki](https://kaikki.org/) and [wiktextract](https://github.com/tatuylonen/wiktextract),
and is licensed under [CC BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/); see
`DATA-LICENSE.md`.

## Regenerating

```sh
cd flexion
cargo xtask extract polish
cargo xtask build polish
```
