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
- `heads.rs`: the words a word ending in one inflects as, its front kept (compounds);
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
| noun | 60268 | 21 | 0 | 288 | 739 | 74 |
| adjective | 24552 | 15 | 18 | 245 | 525 | 41 |
| verb | 17975 | 162 | 24 | 1105 | 839 | 411 |
| determiner | 60 | 6 | 0 | 10 | 18 | 14 |

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
The lemma's own form isn't scored where it is the letters typed (it is the input); where
it isn't (vowel length the headword doesn't write) it is scored, and also apart (*lemma
spelling*). After top-1, its 95% interval, resampling words. *Lemma in full*: top-1 typed
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
shared task's non-neural baseline (letters only), and on the unseen cells. *Against the
floor*: the grammar's letters right less the floor's, on the same forms, with its 95%
interval (resampling words). *Any listed form*: top-1 where any of the forms the
dictionary lists in the cell counts. *Homograph ceiling*: the most any grammar could get
right, words typed alike getting one answer per cell. *On a training word / new root*:
top-1 of the held-out words that are a training word with a prefix or a compound's front
before it, and of the rest. *Attested*: top-1 of the shipped grammar on the dictionary's
own words, typed in full. The held-out tenth was looked at while the trainer was being
developed (changes are judged on another tenth since PR #16).

| | held-out words | forms | top-1 | top-3 | among the candidates | every cell right | calibration error | log loss | lemma in full | principal parts given | 1 / 2 / 3 forms given | features given | seen cells | unseen cells | floor (unseen) | against the floor | any listed form | homograph ceiling | on a training word / new root | lemma spelling | attested |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| noun | 6082 | 6082 | **98.62%** (98.3–98.9) | 99.75% | 99.82% | 98.62% | 0.003 | 0.124 | 98.62% |  |  | 98.62% | 98.62% (6082) |  | 98.62% (0.00%) | +0.00 (-0.20…+0.20) | 98.83% | 99.98% | 99.85% / 98.05% |  | 100.00% |
| adjective | 2402 | 7217 | **98.21%** (97.8–98.7) | 99.56% | 99.57% | 97.42% | 0.005 | 0.311 | 98.21% | 99.67% | 99.54 / 99.88 / 86.67 |  | 98.21% (7217) |  | 98.57% (0.00%) | -0.36 (-0.68…-0.04) | 98.32% | 100.00% | 99.68% / 97.70% |  | 100.00% |
| verb | 931 | 66936 | **98.96%** (98.6–99.3) | 99.37% | 99.39% | 94.95% | 0.004 | 0.344 | 98.96% | 99.72% | 99.13 / 99.27 / 99.33 |  | 98.96% (66936) |  | 99.19% (0.00%) | -0.23 (-0.51…+0.01) | 98.98% | 99.98% | 99.28% / 98.75% | 0.00% (2) | 100.00% |
| determiner | 4 | 12 | **83.33%** (66.7–100.0) | 100.00% | 100.00% | 50.00% | 0.125 | 0.586 | 83.33% |  | 75.00 / 100.00 / 0.00 |  | 83.33% (12) |  | 83.33% (0.00%) | +0.00 (+0.00…+0.00) | 100.00% | 100.00% | 0.00% / 83.33% |  | 100.00% |

| | lines of grammar | bytes |
|---|---:|---:|
| noun | 2523 | 77986 |
| adjective | 2576 | 91671 |
| verb | 43565 | 1414808 |
| determiner | 466 | 15483 |

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
