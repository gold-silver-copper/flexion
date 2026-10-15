# latin

Latin inflection for **any** Latin word, attested or not: nouns, adjectives and
participles, verbs (every finite form, imperatives, infinitives, participles,
gerund, supine), with long vowels marked, each with how sure the grammar is
and why.

```rust
use latin::*;

assert_eq!(noun("rosa", Case::Ablative, Number::Plural), "rosīs");
assert_eq!(noun("rex", Case::Genitive, Number::Singular), "rēgis");
assert_eq!(adjective("bonus", Case::Dative, Gender::Feminine, Number::Singular), "bonae");
assert_eq!(
    verb("amo", Mood::Indicative, Voice::Active, Tense::Perfect, Person::Third, Number::Plural),
    "amāvērunt"
);
```

**Vowel length.** A word typed without macrons is found anyway, and comes back
with its long vowels marked. For a word the dictionary doesn't have, the
grammar restores the marks by rules on the letters around each vowel (`-ātiō`,
`dē-`), but a stem's own vowel lengths can't be told from its spelling, and its
answers say so. Type the lemma with its macrons and say so, and they are
certain:

```rust
use latin::*;

let word = Noun::new("thēsaurus").spelled_in_full();
assert_eq!(word.form((Case::Genitive, Number::Singular)), "thēsaurī");
```

## How it works

This crate is Latin inflection as a grammar, learned by
[flexion](https://github.com/gold-silver-copper/flexion) from the English Wiktionary's
Latin entries. Each part of speech is a module under `src/`, and has what its
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
- `marks.rs`: how a letter typed without its mark (vowel length) is written;
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
| noun | 15894 | 101 | 20 | 497 | 1319 | 234 |
| adjective | 27465 | 72 | 12 | 252 | 503 | 200 |
| verb | 6171 | 198 | 54 | 790 | 1258 | 364 |
| pronoun | 81 | 14 | 0 | 17 | 64 | 362 |
| numeral | 118 | 9 | 0 | 12 | 5 | 5 |

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
| noun | 1565 | 17859 | **68.69%** (66.4–70.9) | 90.58% | 92.19% | 66.90% | 0.019 | 4.835 | 92.93% | 98.69% | 97.72 / 99.10 / 99.22 | 69.57% | 68.71% (17854) | 0.00% (5) | 58.82% (0.00%) | +9.87 (+7.17…+12.54) | 68.74% | 99.46% | 75.91% / 66.47% | 68.09% (984) | 100.00% |
| adjective | 2841 | 101595 | **84.05%** (82.8–85.3) | 96.06% | 96.19% | 83.07% | 0.023 | 2.613 | 99.23% | 99.47% | 99.58 / 99.63 / 99.64 |  | 84.05% (101594) | 0.00% (1) | 71.72% (0.00%) | +12.32 (+10.56…+14.12) | 84.14% | 99.60% | 87.26% / 82.03% | 82.98% (2151) | 99.91% |
| verb | 619 | 60947 | **77.28%** (74.2–80.5) | 95.33% | 97.29% | 67.85% | 0.044 | 2.371 | 85.66% | 98.55% | 96.25 / 96.43 / 96.60 |  | 77.26% (60826) | 87.60% (121) | 65.32% (73.55%) | +11.96 (+7.95…+16.09) | 77.42% | 98.45% | 86.26% / 73.46% | 88.85% (592) | 100.00% |
| pronoun | 9 | 268 | **6.34%** (0.0–16.1) | 23.51% | 43.66% | 0.00% | 0.216 | 31.488 | 19.08% |  | 30.43 / 31.15 / 31.91 |  | 6.34% (268) |  | 25.00% (0.00%) | -18.66 (-47.89…+3.14) | 9.70% | 100.00% | 0.00% / 6.34% | 14.29% (7) | 100.00% |
| numeral | 11 | 396 | **72.73%** (45.5–100.0) | 100.00% | 100.00% | 72.73% | 0.173 | 1.067 | 100.00% |  | 100.00 / 100.00 / 100.00 |  | 72.73% (396) |  | 72.73% (0.00%) | +0.00 (-36.36…+36.36) | 72.73% | 100.00% | 66.67% / 75.00% | 72.73% (11) | 100.00% |

| | lines of grammar | bytes |
|---|---:|---:|
| noun | 10587 | 346442 |
| adjective | 7321 | 270557 |
| verb | 22535 | 944867 |
| pronoun | 1301 | 49034 |
| numeral | 834 | 30402 |

## Data and license

The code is MIT or Apache-2.0. The grammar under `src/` is derived from Wiktionary via
[Kaikki](https://kaikki.org/) and [wiktextract](https://github.com/tatuylonen/wiktextract),
and is licensed under [CC BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/); see
`DATA-LICENSE.md`.

## Regenerating

```sh
cd flexion
cargo xtask extract latin
cargo xtask build latin
```
