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
  follows it, with a pattern per form (`~`: the stem, then what follows it;
  `derived`: another cell's form, changed);
- `relations.rs`: which cell each cell's form is told from, and the rules on that
  form's ending that change it;
- `laws.rs`: sound laws where a stem meets its ending or a literal before it (along a
  tier, on a copied letter, or on a class letter an ending writes: harmony), each
  with the words whose forms it gives and the forms it gets wrong;
- `rules.rs`: which paradigm a word takes, as rules on its ending, each with how many
  of the dictionary's words with that ending follow each paradigm, and the words no
  rule gets right, listed;
- `compounds.rs`: whether a word written in parts inflects each part;
- `spellings.rs`: the words whose spelling their typed form doesn't show;
- `stems.rs`: the stem slots (the cells a word builds on one stem, learned from the
  words that put different stems in different cells) and the ending tables paradigms
  share over a slot, each named after its commonest word;
- `irregular.rs`: what no paradigm gives a word: the stem it lists for a slot, built
  with an ending table, and the forms no stem gives.

A word the dictionary lists gets its own forms. Any other word gets the paradigm its
ending suggests, with a probability that says how often such a guess is right, and the
reason (the rule and its counts).

## The grammar

| | words | paradigms | sound laws | rules | listed words | irregular forms |
|---|---:|---:|---:|---:|---:|---:|
| noun | 43092 | 15 | 0 | 269 | 628 | 285 |
| adjective | 18164 | 19 | 6 | 226 | 491 | 62 |
| verb | 7295 | 56 | 16 | 292 | 149 | 204 |
| participle | 6504 | 5 | 1 | 17 | 23 | 0 |
| determiner | 49 | 9 | 1 | 14 | 12 | 14 |

*Words*: the dictionary's words the grammar is learned from. *Sound laws*: changes
where a stem meets its ending (or farther, along a tier). *Rules*: the endings that
choose a paradigm. *Listed words*: words no rule gets right, listed with their paradigm.
*Irregular forms*: forms no paradigm gives, listed.

## How well it does

A tenth of the words (by a hash of the word) was held out: the grammar scored here was
learned from the other nine tenths, and asked for every form the dictionary lists of
each held-out word, typed as the dictionary's headword. *Top-1*: its first answer is
right; *top-3*: the right answer is among its first three. *Calibration error*: how far
its stated probabilities are, on average, from how often it is right (0 is perfect).
The margin after top-1 is a 95% interval, counting words. *Lemma in full*: top-1 typed
with every mark the dictionary's table writes, and said to be so (`spelled_in_full`).
*Principal parts given*: top-1 of the other forms, given the dictionary's principal
parts. *Features given*: top-1 from the headword, told the word's inherent features
(its gender, as `.gender(…)` tells it). *Among the candidates*: the right answer is
among its answers at all. *Every cell right*: the share of words with every form right.
*Log loss*: bits per form, −log₂ of the probability it gives the right answer (a form
it doesn't give costs 5 bits a letter). *1 / 2 / 3 forms given*: top-1 of the other
forms, typed in full, given that many of the word's forms chosen at random.
*Seen* and *unseen cells*: top-1 on the cells some training word of the word's
paradigm has a form in, and on the others, derived
from other cells (with the number of forms). *Floor*: the CoNLL–SIGMORPHON 2017
shared task's non-neural baseline (letters only), and on the unseen cells. *Attested*: top-1 of the shipped
grammar on the dictionary's own words, typed in full.

| | held-out words | forms | top-1 | top-3 | among the candidates | every cell right | calibration error | log loss | lemma in full | principal parts given | 1 / 2 / 3 forms given | features given | seen cells | unseen cells | floor (unseen) | attested |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| noun | 4318 | 8636 | **98.78%** ± 0.3 | 99.80% | 99.87% | 97.57% | 0.003 | 0.116 | 98.78% |  |  | 98.78% | 98.78% (8636) |  | 98.51% (0.00%) | 100.00% |
| adjective | 1839 | 7356 | **98.12%** ± 0.6 | 99.65% | 99.70% | 96.52% | 0.006 | 0.204 | 98.12% | 99.97% | 99.11 / 99.84 / 100.00 |  | 98.15% (7354) | 0.00% (2) | 98.29% (50.00%) | 100.00% |
| verb | 743 | 35550 | **98.73%** ± 0.8 | 99.81% | 99.81% | 97.04% | 0.004 | 0.153 | 98.73% | 99.49% | 99.18 / 99.42 / 99.53 |  | 98.73% (35550) |  | 99.30% (0.00%) | 100.00% |
| participle | 627 | 2508 | **99.88%** ± 0.3 | 100.00% | 100.00% | 99.84% | 0.004 | 0.008 | 99.88% |  | 100.00 / 100.00 / 0.00 |  | 99.88% (2508) |  | 99.64% (0.00%) | 100.00% |
| determiner | 6 | 19 | **63.16%** ± 38.6 | 63.16% | 63.16% | 50.00% | 0.368 | 10.526 | 63.16% |  | 72.73 / 83.33 / 0.00 |  | 63.16% (19) |  | 78.95% (0.00%) | 100.00% |

| | lines of grammar | bytes |
|---|---:|---:|
| noun | 2738 | 81871 |
| adjective | 2347 | 75589 |
| verb | 3517 | 160251 |
| participle | 533 | 18457 |
| determiner | 540 | 18722 |

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
