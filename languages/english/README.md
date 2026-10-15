# english

English inflection for **any** English word, attested or not: noun plurals,
verb forms, comparatives and superlatives, each with how sure the grammar is
and why.

```rust
use english::*;

assert_eq!(noun("cat", Number::Plural), "cats");
assert_eq!(noun("child", Number::Plural), "children");
assert_eq!(verb("go", VerbForm::Past), "went");
assert_eq!(adjective("good", Degree::Comparative), "better");
```

A word nobody wrote down inflects by the rules learned from the words that
were. Every answer can come with its alternatives, how likely each is, and why:

```rust
use english::*;

let plurals = Noun::new("glorp").candidates(Number::Plural);
assert_eq!(plurals[0].form, "glorps");
println!("{}", plurals[0]);
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
English entries. Each part of speech is a module under `src/`, and has what its
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
| noun | 228072 | 84 | 0 | 2233 | 5726 | 249 |
| verb | 38230 | 83 | 31 | 1124 | 1668 | 202 |
| adjective | 61454 | 13 | 4 | 465 | 2507 | 16 |
| adverb | 12620 | 4 | 2 | 81 | 169 | 16 |

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
| noun | 23733 | 23734 | **97.17%** (97.0–97.4) | 99.67% | 99.77% | 97.17% | 0.003 | 0.234 | 97.17% |  |  |  | 97.17% (23734) |  | 96.77% (0.00%) | +0.40 (+0.24…+0.56) | 97.97% | 99.88% | 98.68% / 95.73% | 0.00% (1) | 100.00% |
| verb | 4151 | 16594 | **97.17%** (96.8–97.6) | 99.73% | 99.74% | 94.97% | 0.006 | 0.193 | 97.17% | 99.54% | 98.65 / 99.18 / 99.66 |  | 97.17% (16594) |  | 97.79% (0.00%) | -0.62 (-0.93…-0.33) | 97.61% | 99.86% | 98.92% / 96.23% |  | 100.00% |
| adjective | 6239 | 12476 | **95.35%** (94.8–95.9) | 99.95% | 99.97% | 95.34% | 0.005 | 0.177 | 95.35% |  | 99.98 / 0.00 / 0.00 |  | 95.35% (12476) |  | 92.53% (0.00%) | +2.82 (+2.39…+3.25) | 96.30% | 99.94% | 99.05% / 93.61% |  | 100.00% |
| adverb | 1207 | 2414 | **99.01%** (98.4–99.5) | 100.00% | 100.00% | 99.01% | 0.005 | 0.105 | 99.01% |  | 100.00 / 0.00 / 0.00 |  | 99.01% (2414) |  | 98.34% (0.00%) | +0.66 (+0.25…+1.16) | 99.59% | 100.00% | 100.00% / 98.67% |  | 100.00% |

| | lines of grammar | bytes |
|---|---:|---:|
| noun | 16084 | 502579 |
| verb | 6671 | 228476 |
| adjective | 3612 | 111546 |
| adverb | 681 | 24667 |

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
