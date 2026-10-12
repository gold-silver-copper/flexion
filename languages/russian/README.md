# russian

Russian inflection for **any** Russian word, attested or not: noun declension,
adjectives (long and short forms), verbs (non-past, past, imperative,
participles, gerunds), with a confidence for every answer.

```rust
use russian::*;

assert_eq!(noun("книга", Case::Genitive, Number::Plural), "книг");
assert_eq!(noun("стол", Case::Genitive, Number::Singular), "стола");
assert_eq!(adjective("новый", Case::Dative, Agreement::Feminine, Animacy::Inanimate), "новой");
assert_eq!(non_past("читать", Person::Third, Number::Plural), "читают");
assert_eq!(past("читать", Agreement::Feminine), "читала");

// Gender is predicted too, for words the data never saw.
assert_eq!(gender("тетрадь")[0].0, Gender::Feminine);
```

A citation gives principal parts (nominative and genitive singular for nouns;
infinitive and first and second singular for verbs); the rest is predicted to
agree with them.

```rust
use russian::*;

assert_eq!(noun("путь, пути", Case::Instrumental, Number::Singular), "путём");

// Markers name what spelling can't show, as a dictionary prints them after the
// word: gender (`m`, `f`, `n`), animacy (`anim`, `inan`), aspect (`impf`, `pf`).
// They decide for a word the model doesn't know; an attested word keeps its own.
assert_eq!(noun("бокр, бокра m anim", Case::Accusative, Number::Singular), "бокра");
assert_eq!(noun("бокр, бокра m inan", Case::Accusative, Number::Singular), "бокр");
```

## How it works

This crate is Russian inflection as a grammar, learned by
[flexion](https://github.com/gold-silver-copper/flexion) from the English Wiktionary's and the Russian Wiktionary's
Russian entries. Each part of speech is a module under `src/`:

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
| noun | 108894 | 204 | 1754 | 4430 | 2675 |
| adjective | 67695 | 41 | 351 | 1639 | 997 |
| verb | 46911 | 348 | 2244 | 4831 | 1705 |
| pronoun | 42 | 11 | 9 | 15 | 434 |
| numeral | 77 | 5 | 9 | 19 | 461 |

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
| noun | 2611 | 30201 | **94.96%** | 99.45% | 0.003 |
| adjective | 3466 | 171225 | **99.76%** | 99.99% | 0.002 |
| verb | 1304 | 21858 | **96.00%** | 99.47% | 0.008 |
| pronoun | 4 | 192 | **55.21%** | 55.21% | 0.307 |
| numeral | 4 | 156 | **94.23%** | 94.23% | 0.058 |


## Data and license

The code is MIT or Apache-2.0. The grammar under `src/` is derived from Wiktionary via
[Kaikki](https://kaikki.org/) and [wiktextract](https://github.com/tatuylonen/wiktextract),
and is licensed under [CC BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/); see
`DATA-LICENSE.md`.

## Regenerating

```sh
cd flexion
cargo xtask extract russian
cargo xtask build russian
```
