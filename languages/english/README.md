# english

English inflection for **any** English word, attested or not: noun plurals, verb
forms, and comparatives and superlatives, each with a confidence.

```rust
use english::*;

assert_eq!(noun("cat", Number::Plural), "cats");
assert_eq!(noun("child", Number::Plural), "children");
assert_eq!(verb("go", VerbForm::Past), "went");
assert_eq!(adjective("good", Degree::Comparative), "better");
assert_eq!(adjective("beautiful", Degree::Superlative), "most beautiful");

// Words nobody wrote down inflect by the patterns learned from those who did.
assert_eq!(noun("glorp", Number::Plural), "glorps");

// Every candidate, with the model's confidence.
let plurals = noun_scored("octopus", Number::Plural);
assert_eq!(plurals[0].form, "octopuses");

// A citation pins principal parts: here, which `lie` you mean.
assert_eq!(verb("lie", VerbForm::Past), "lay");
assert_eq!(verb("lie, lied", VerbForm::Past), "lied");
assert_eq!(verb("lie, lied", VerbForm::PastParticiple), "lied");
```

Agreement, pronouns and possessives are hand-written helpers on top:

```rust
use english::*;

assert_eq!(conjugate("be", Tense::Present, Person::First, Number::Singular), "am");
assert_eq!(conjugate("walk", Tense::Present, Person::Third, Number::Singular), "walks");
assert_eq!(pronoun(Person::Third, Number::Singular, Gender::Feminine, Case::Accusative), "her");
assert_eq!(count_with_number("mouse", 3), "3 mice");
assert_eq!(add_possessive("dogs"), "dogs'");
```

## How it works

This crate is English inflection as a grammar, learned by
[flexion](https://github.com/gold-silver-copper/flexion) from the English Wiktionary's
English entries. Each part of speech is a module under `src/`:

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
| noun | 228072 | 109 | 1403 | 4782 | 5803 |
| verb | 38230 | 88 | 472 | 1211 | 2790 |
| adjective | 61454 | 68 | 4990 | 19951 | 22855 |
| adverb | 12620 | 28 | 838 | 3869 | 4613 |

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
| noun | 23733 | 47466 | **97.64%** | 99.79% | 0.010 |
| verb | 4151 | 20745 | **96.62%** | 99.72% | 0.016 |
| adjective | 6239 | 18715 | **96.62%** | 99.50% | 0.052 |
| adverb | 1207 | 3621 | **99.17%** | 99.70% | 0.070 |


## Data and license

The code is MIT or Apache-2.0. The grammar under `src/` is derived from Wiktionary via
[Kaikki](https://kaikki.org/) and [wiktextract](https://github.com/tatuylonen/wiktextract),
and is licensed under [CC BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/); see
`DATA-LICENSE.md`.

## Regenerating

```sh
cd flexion
cargo xtask extract english
cargo xtask build english
```
