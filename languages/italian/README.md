# italian

Italian inflection for **any** Italian word, attested or not: noun plurals,
adjective and participle agreement, and the whole simple conjugation
(indicative, subjunctive, imperative, gerund, participles), each with how sure
the grammar is and why.

```rust
use italian::*;

assert_eq!(noun("uomo", Number::Plural), "uomini");
assert_eq!(noun("città", Number::Plural), "città");
assert_eq!(adjective("bello", Gender::Feminine, Number::Plural), "belle");
assert_eq!(verb("finire", Tense::Present, Person::First, Number::Singular), "finisco");
assert_eq!(verb("prendere", Tense::PastHistoric, Person::First, Number::Singular), "presi");
assert_eq!(agreeing_participle("scrivere", Gender::Feminine, Number::Singular), "scritta");
assert_eq!(before_noun("bello", Gender::Masculine, Number::Plural, "occhi").as_deref(), Some("begli"));
```

Forms are written in standard spelling, with an accent only on a final
stressed vowel (`città`, `prenderò`) and on the monosyllables that take one
(`dà`, `può`).

## How it works

This crate is Italian inflection as a grammar, learned by
[flexion](https://github.com/gold-silver-copper/flexion) from the English Wiktionary's
Italian entries. Each part of speech is a module under `src/`, and has what its
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
- `prefixes.rs`: prefixes a word inflects behind, separable or not;
- `compounds.rs`: whether a word written in parts inflects each part;
- `spellings.rs`: the words whose spelling their typed form doesn't show;
- `stems.rs`: the stem slots (the cells a word builds on one stem, learned from the
  words that put different stems in different cells) and the ending tables paradigms
  share over a slot, each named after its commonest word;
- `irregular.rs`: what no paradigm gives a word: the stem it lists for a slot, built
  with an ending table, and the forms no stem gives;
- `auxiliary.rs`: a word's auxiliary, which decides which cell some cells' forms
  come from.

A word the dictionary lists gets its own forms. Any other word gets the paradigm its
ending suggests, with a probability that says how often such a guess is right, and the
reason (the rule and its counts).

## The grammar

| | words | paradigms | sound laws | rules | listed words | irregular forms |
|---|---:|---:|---:|---:|---:|---:|
| noun | 61772 | 26 | 0 | 1020 | 2238 | 92 |
| adjective | 26291 | 12 | 1 | 322 | 484 | 48 |
| verb | 12955 | 120 | 19 | 447 | 5115 | 5709 |
| participle | 8087 | 2 | 0 | 6 | 2 | 3 |
| determiner | 44 | 4 | 0 | 5 | 8 | 6 |

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
| noun | 6392 | 12784 | **97.91%** ± 0.4 | 99.72% | 99.84% | 95.82% | 0.003 | 0.196 | 97.91% |  |  | 98.19% | 97.91% (12784) |  | 97.54% (0.00%) | 100.00% |
| adjective | 2632 | 10535 | **98.84%** ± 0.4 | 99.90% | 99.90% | 97.61% | 0.007 | 0.103 | 98.84% | 99.87% | 99.72 / 99.83 / 100.00 |  | 98.84% (10530) | 100.00% (5) | 98.84% (100.00%) | 100.00% |
| verb | 1286 | 71586 | **97.61%** ± 0.8 | 98.09% | 98.09% | 56.77% | 0.022 | 1.178 | 97.61% | 98.60% | 98.19 / 98.31 / 98.37 | 99.07% | 97.61% (71585) | 100.00% (1) | 99.24% (100.00%) | 100.00% |
| participle | 821 | 3284 | **100.00%** ± 0.0 | 100.00% | 100.00% | 100.00% | 0.000 | 0.000 | 100.00% |  | 100.00 / 100.00 / 0.00 |  | 100.00% (3284) |  | 100.00% (0.00%) | 100.00% |
| determiner | 7 | 26 | **96.15%** ± 14.2 | 100.00% | 100.00% | 85.71% | 0.038 | 0.962 | 96.15% |  | 94.44 / 91.67 / 0.00 |  | 96.15% (26) |  | 100.00% (0.00%) | 100.00% |

| | lines of grammar | bytes |
|---|---:|---:|
| noun | 6618 | 223322 |
| adjective | 2634 | 85361 |
| verb | 25302 | 952259 |
| participle | 438 | 15111 |
| determiner | 403 | 13296 |

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
