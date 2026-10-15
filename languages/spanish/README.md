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

| | words | paradigms | sound laws | rules | listed words | irregular forms | listed by relations |
|---|---:|---:|---:|---:|---:|---:|---:|
| noun | 60257 | 21 | 0 | 286 | 731 | 66 | 0 |
| adjective | 24530 | 15 | 19 | 242 | 525 | 41 | 1 |
| verb | 17957 | 159 | 24 | 1097 | 822 | 382 | 136 |
| determiner | 60 | 6 | 0 | 10 | 18 | 14 | 5 |

*Words*: the dictionary's words the grammar is learned from. *Sound laws*: changes
where a stem meets its ending (or farther, along a tier). *Rules*: the endings that
choose a paradigm. *Listed words*: words no rule gets right, listed with their paradigm.
*Irregular forms*: forms no paradigm gives, listed. *Listed by relations*: the words the
relations' rules get wrong, listed with the change they take.

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
before it, and of the rest. *Own / form-of / alt-of*: top-1 of the held-out entries that
are words of their own, another word's form (a participle, a comparative) and another
word's spelling, with their number of words. *Phrases*: top-1 on the forms the dictionary
writes as phrases (the Latin perfect passive, `amātus sum`). *Irregular lines*: the share of the
dictionary's words the grammar lists a form or a stem of. The held-out tenth was looked at while the trainer was being
developed (changes are judged on another tenth since PR #16).

| | held-out words | forms | top-1 | top-3 | among the candidates | every cell right | calibration error | log loss | lemma in full | principal parts given | 1 / 2 / 3 forms given | features given | seen cells | unseen cells | floor (unseen) | against the floor | any listed form | homograph ceiling | on a training word / new root | lemma spelling | own / form-of / alt-of | phrases | irregular lines |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| noun | 6079 | 6079 | **98.67%** (98.4–98.9) | 99.80% | 99.87% | 98.67% | 0.003 | 0.111 | 98.67% |  |  | 98.67% | 98.67% (6079) |  | 98.67% (0.00%) | +0.00 (-0.20…+0.20) | 98.88% | 99.98% | 99.85% / 98.12% |  | 98.74% (5245) / 100.00% (583) / 94.02% (251) |  | 0.11% |
| adjective | 2399 | 7208 | **98.24%** (97.8–98.7) | 99.54% | 99.56% | 97.46% | 0.005 | 0.315 | 98.24% | 99.67% | 99.54 / 99.88 / 86.67 |  | 98.24% (7208) |  | 98.60% (0.00%) | -0.36 (-0.68…-0.07) | 98.35% | 100.00% | 99.68% / 97.74% |  | 98.21% (2284) / 100.00% (68) / 97.12% (47) |  | 0.22% |
| verb | 929 | 66839 | **98.97%** (98.6–99.3) | 99.39% | 99.41% | 95.16% | 0.004 | 0.336 | 98.97% | 99.73% | 99.16 / 99.29 / 99.43 |  | 98.97% (66839) |  | 99.28% (0.00%) | -0.31 (-0.60…-0.02) | 98.99% | 99.98% | 99.28% / 98.77% |  | 99.02% (879) / 100.00% (4) / 97.86% (46) |  | 1.73% |
| determiner | 4 | 12 | **83.33%** (66.7–100.0) | 100.00% | 100.00% | 50.00% | 0.125 | 0.586 | 83.33% |  | 75.00 / 100.00 / 0.00 |  | 83.33% (12) |  | 83.33% (0.00%) | +0.00 (+0.00…+0.00) | 100.00% | 100.00% | 0.00% / 83.33% |  | 83.33% (2) / 100.00% (1) / 66.67% (1) |  | 15.00% |

| | lines of grammar | bytes |
|---|---:|---:|
| noun | 1606 | 52996 |
| adjective | 1511 | 53751 |
| verb | 7620 | 361835 |
| determiner | 450 | 14882 |

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
