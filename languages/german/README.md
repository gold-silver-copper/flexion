# german

German inflection for **any** German word, attested or not: noun declension,
adjectives in all three declensions and three degrees, and verbs (every finite
form in main and subordinate clauses, imperative, infinitives, participles),
each with how sure the grammar is and why.

```rust
use german::*;

assert_eq!(noun("Mann", Case::Nominative, Number::Plural), "Männer");
assert_eq!(noun("haus", Case::Dative, Number::Plural), "Häusern");
assert_eq!(adjective("schön", Declension::Weak, Case::Nominative, Agreement::Masculine), "schöne");
assert_eq!(verb("geben", Tense::Present, Person::Third, Number::Singular), "gibt");
assert_eq!(participle("singen", Participle::Past), "gesungen");
```

A verb that begins with a prefix inflects as the rest of it does. A separable
prefix goes where the form puts it, an inseparable one stays in front, and the
participle's `ge-` goes:

```rust
use german::*;

assert_eq!(verb("anfangen", Tense::Preterite, Person::First, Number::Singular), "fing an");
assert_eq!(subordinate("anfangen", Tense::Present, Person::Third, Number::Singular), "anfängt");
assert_eq!(participle("anfangen", Participle::Past), "angefangen");
assert_eq!(participle("verfangen", Participle::Past), "verfangen");
```

Nouns are always written capitalized, however the word is typed.

## How it works

This crate is German inflection as a grammar, learned by
[flexion](https://github.com/gold-silver-copper/flexion) from the English Wiktionary's and the German Wiktionary's
German entries. Each part of speech is a module under `src/`, and has what its
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
| noun | 143828 | 167 | 88 | 7745 | 14744 | 252 |
| adjective | 14840 | 23 | 26 | 375 | 585 | 893 |
| verb | 9323 | 74 | 5 | 248 | 739 | 4500 |
| determiner | 37 | 3 | 0 | 6 | 9 | 16 |

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
| noun | 6323 | 40053 | **93.63%** (93.2–94.1) | 98.74% | 98.85% | 86.92% | 0.007 | 0.782 | 93.63% | 99.79% | 97.99 / 98.35 / 99.09 | 94.05% | 93.63% (40053) |  | 93.88% (0.00%) | -0.25 (-0.58…+0.08) | 94.43% | 99.69% | 96.70% / 89.92% |  | 100.00% |
| adjective | 1487 | 125205 | **98.76%** (98.2–99.3) | 99.80% | 99.80% | 96.50% | 0.003 | 0.140 | 98.76% | 99.86% | 99.42 / 99.41 / 99.41 |  | 98.76% (125205) |  | 99.00% (0.00%) | -0.24 (-0.74…+0.25) | 99.03% | 100.00% | 98.59% / 98.83% | 0.00% (28) | 100.00% |
| verb | 971 | 50629 | **95.07%** (94.1–96.0) | 98.22% | 98.39% | 85.58% | 0.013 | 1.037 | 95.07% | 95.78% | 95.55 / 95.61 / 95.68 | 95.18% | 95.07% (50629) |  | 73.03% (0.00%) | +22.04 (+20.29…+23.86) | 95.17% | 99.26% | 95.91% / 94.14% | 0.00% (1) | 100.00% |
| determiner | 3 | 45 | **40.00%** (0.0–100.0) | 73.33% | 73.33% | 33.33% | 0.598 | 16.619 | 40.00% |  | 73.81 / 69.23 / 66.67 |  | 40.00% (45) |  | 40.00% (0.00%) | +0.00 (+0.00…+0.00) | 40.00% | 100.00% | 0.00% / 40.00% |  | 100.00% |

| | lines of grammar | bytes |
|---|---:|---:|
| noun | 53537 | 1814780 |
| adjective | 3131 | 178524 |
| verb | 7165 | 371853 |
| determiner | 516 | 17832 |

## Data and license

The code is MIT or Apache-2.0. The grammar under `src/` is derived from Wiktionary via
[Kaikki](https://kaikki.org/) and [wiktextract](https://github.com/tatuylonen/wiktextract),
and is licensed under [CC BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/); see
`DATA-LICENSE.md`.

## Regenerating

```sh
cd flexion
cargo xtask extract german
cargo xtask build german
```
