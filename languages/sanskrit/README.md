# sanskrit

Sanskrit inflection for **any** word, attested or not, in Devanagari: nouns and
adjectives in eight cases and three numbers, and verbs (present system with its
four moods, imperfect, future, conditional, perfect, aorist, infinitive,
gerund, participles), each with how sure the grammar is and why.

```rust
use sanskrit::*;

assert_eq!(noun("देव", Case::Instrumental, Number::Singular), "देवेन");
assert_eq!(noun("देव", Case::Genitive, Number::Plural), "देवानाम्");
assert_eq!(adjective("नव", Case::Nominative, Gender::Feminine, Number::Singular), "नवा");

// A noun the dictionary lacks: by its ending masculine, unless you say otherwise.
assert_eq!(noun("सुकुल", Case::Nominative, Number::Singular), "सुकुलः");
let neuter = Noun::new("सुकुल").gender(Gender::Neuter);
assert_eq!(neuter.form((Case::Nominative, Number::Singular)), "सुकुलम्");
```

Nouns and adjectives are cited by their stem, as Sanskrit dictionaries do;
verbs by the third person singular present. The forms are Classical (Vedic
variants and accents are left out), with the sandhi inside the word.

## How it works

This crate is Sanskrit inflection as a grammar, learned by
[flexion](https://github.com/gold-silver-copper/flexion) from the English Wiktionary's
Sanskrit entries. Each part of speech is a module under `src/`, and has what its
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
| noun | 5023 | 55 | 4 | 283 | 889 | 199 |
| adjective | 1964 | 41 | 2 | 160 | 362 | 441 |
| verb | 1004 | 66 | 7 | 158 | 366 | 1266 |
| pronoun | 15 | 3 | 0 | 3 | 9 | 27 |

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
| noun | 576 | 13801 | **92.10%** (90.8–93.4) | 98.74% | 98.74% | 73.96% | 0.015 | 0.765 | 92.10% |  | 94.57 / 94.85 / 96.01 | 97.30% | 92.10% (13801) |  | 91.70% (0.00%) | +0.40 (-0.73…+1.48) | 92.65% | 97.83% | 94.30% / 92.03% |  | 100.00% |
| adjective | 190 | 13456 | **91.91%** (89.6–94.0) | 98.66% | 98.66% | 72.11% | 0.020 | 0.931 | 91.91% |  | 93.69 / 94.00 / 97.29 |  | 91.97% (13447) | 11.11% (9) | 89.95% (11.11%) | +1.96 (+0.32…+3.76) | 91.94% | 100.00% | 83.33% / 92.30% |  | 100.00% |
| verb | 88 | 7177 | **89.97%** (85.4–94.1) | 91.71% | 92.06% | 65.91% | 0.060 | 4.316 | 89.97% | 90.05% | 91.07 / 91.13 / 91.15 |  | 90.37% (6896) | 80.07% (281) | 89.93% (80.78%) | +0.04 (-2.21…+2.14) | 90.48% | 100.00% | 95.65% / 89.43% | 100.00% (2) | 100.00% |
| pronoun | 3 | 207 | **69.57%** (0.0–100.0) | 69.57% | 69.57% | 66.67% | 0.298 | 10.836 | 69.57% |  | 69.61 / 69.65 / 75.25 |  | 69.57% (207) |  | 70.53% (0.00%) | -0.97 (-3.17…+0.00) | 69.57% | 100.00% | 0.00% / 69.57% |  | 100.00% |

| | lines of grammar | bytes |
|---|---:|---:|
| noun | 7509 | 296748 |
| adjective | 3995 | 189656 |
| verb | 6428 | 343408 |
| pronoun | 914 | 37134 |

## Data and license

The code is MIT or Apache-2.0. The grammar under `src/` is derived from Wiktionary via
[Kaikki](https://kaikki.org/) and [wiktextract](https://github.com/tatuylonen/wiktextract),
and is licensed under [CC BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/); see
`DATA-LICENSE.md`.

## Regenerating

```sh
cd flexion
cargo xtask extract sanskrit
cargo xtask build sanskrit
```
