# italian

Italian inflection for **any** Italian word, attested or not: noun plurals,
adjective and participle agreement, and the whole simple conjugation
(indicative, subjunctive, imperative, gerund, participles), with a confidence
for every answer.

```rust
use italian::*;

assert_eq!(noun("uomo", Number::Plural), "uomini");
assert_eq!(noun("città", Number::Plural), "città");
assert_eq!(adjective("bello", Gender::Feminine, Number::Plural), "belle");
assert_eq!(verb("finire", Tense::Present, Person::First, Number::Singular), "finisco");
assert_eq!(verb("prendere", Tense::PastHistoric, Person::First, Number::Singular), "presi");
assert_eq!(verb("rifare", Tense::Present, Person::Third, Number::Singular), "rifà");
assert_eq!(agreeing_participle("scrivere", Gender::Feminine, Number::Singular), "scritta");
assert_eq!(before_noun("bello", Gender::Masculine, Number::Plural, "occhi").as_deref(), Some("begli"));
```

Forms are written in standard spelling, with an accent only on a final
stressed vowel (`città`, `prenderò`) and on the monosyllables that take one
(`dà`, `può`).

A citation gives principal parts the way a dictionary prints an irregular
verb: first person singular present, passato remoto and past participle
(`prendere, prendo, presi, preso`). Nouns take their gender as a marker
(`mano f`).

## How it works

This crate is Italian inflection as a grammar, learned by
[flexion](https://github.com/gold-silver-copper/flexion) from the English Wiktionary's
Italian entries. Each part of speech is a module under `src/`:

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
| noun | 61772 | 24 | 659 | 1997 | 3914 |
| adjective | 26291 | 15 | 221 | 458 | 1071 |
| verb | 12955 | 101 | 272 | 535 | 44889 |
| participle | 8087 | 2 | 4 | 2 | 3 |
| determiner | 44 | 5 | 4 | 6 | 5 |

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
| noun | 6392 | 12784 | **95.81%** | 99.67% | 0.011 |
| adjective | 2632 | 10535 | **98.20%** | 99.94% | 0.005 |
| verb | 1286 | 71586 | **95.80%** | 99.60% | 0.024 |
| participle | 821 | 3284 | **100.00%** | 100.00% | 0.000 |
| determiner | 7 | 26 | **96.15%** | 100.00% | 0.038 |


## Data and license

The code is MIT or Apache-2.0. The grammar under `src/` is derived from Wiktionary via
[Kaikki](https://kaikki.org/) and [wiktextract](https://github.com/tatuylonen/wiktextract),
and is licensed under [CC BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/); see
`DATA-LICENSE.md`.

## Regenerating

```sh
cd flexion
cargo xtask extract italian
cargo xtask build italian
```
