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
    indicative("amo", Voice::Active, Tense::Perfect, Person::Third, Number::Plural),
    "amāvērunt"
);
assert_eq!(
    subjunctive("amo", Voice::Active, SubjunctiveTense::Imperfect, Person::First, Number::Singular),
    "amārem"
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

| | words | paradigms | sound laws | rules | listed words | irregular forms | listed by relations |
|---|---:|---:|---:|---:|---:|---:|---:|
| noun | 15883 | 95 | 20 | 496 | 1300 | 224 | 14 |
| proper_noun | 8787 | 83 | 10 | 235 | 948 | 153 | 11 |
| adjective | 27458 | 67 | 11 | 256 | 461 | 212 | 34 |
| verb | 6168 | 200 | 41 | 793 | 1264 | 495 | 61 |
| pronoun | 80 | 12 | 0 | 18 | 63 | 362 | 45 |
| numeral | 118 | 9 | 0 | 12 | 5 | 5 | 0 |
| adverb | 612 | 13 | 1 | 35 | 49 | 23 | 0 |

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
| noun | 1565 | 17859 | **71.12%** (68.9–73.4) | 91.50% | 93.38% | 69.33% | 0.038 | 4.110 | 92.93% | 98.70% | 97.83 / 99.32 / 99.41 | 71.98% | 71.14% (17854) | 0.00% (5) | 59.09% (0.00%) | +12.03 (+9.41…+14.72) | 71.22% | 99.46% | 77.96% / 69.02% | 69.72% (984) | 71.25% (1379) / 77.99% (19) / 69.27% (167) |  | 1.16% |
| proper_noun | 834 | 5119 | **59.84%** (56.4–63.2) | 87.15% | 90.27% | 59.23% | 0.054 | 5.756 | 92.75% | 98.24% | 96.74 / 98.45 / 98.71 | 59.84% | 59.89% (5098) | 47.62% (21) | 60.58% (57.14%) | -0.74 (-4.64…+3.12) | 60.32% | 99.32% | 58.77% / 59.93% | 47.51% (362) | 59.75% (801) / – / 61.84% (33) |  | 1.63% |
| adjective | 2841 | 101595 | **87.50%** (86.3–88.6) | 97.03% | 97.16% | 86.45% | 0.022 | 2.004 | 99.23% | 99.48% | 99.52 / 99.61 / 99.61 |  | 87.50% (101594) | 0.00% (1) | 71.87% (0.00%) | +15.63 (+13.86…+17.46) | 87.61% | 99.60% | 89.98% / 85.95% | 86.66% (2151) | 76.94% (981) / 93.73% (1684) / 86.59% (176) |  | 0.48% |
| verb | 619 | 77103 | **77.49%** (74.4–80.6) | 95.65% | 97.46% | 68.17% | 0.027 | 2.481 | 84.92% | 98.59% | 95.37 / 95.48 / 95.80 |  | 77.57% (76833) | 54.44% (270) | 64.17% (33.33%) | +13.32 (+9.35…+17.33) | 77.61% | 98.43% | 84.08% / 74.65% | 89.86% (592) | 78.26% (541) / – / 72.34% (78) | 74.64% (16157) | 4.69% |
| pronoun | 9 | 268 | **6.34%** (0.0–16.1) | 27.99% | 43.66% | 0.00% | 0.230 | 31.506 | 19.08% |  | 33.20 / 35.25 / 35.74 |  | 6.34% (268) |  | 25.00% (0.00%) | -18.66 (-47.89…+3.14) | 10.07% | 100.00% | 0.00% / 6.34% | 14.29% (7) | 8.42% (7) / – / 0.00% (2) |  | 73.75% |
| numeral | 11 | 396 | **72.73%** (45.5–100.0) | 100.00% | 100.00% | 72.73% | 0.303 | 1.223 | 100.00% |  | 100.00 / 100.00 / 100.00 |  | 72.73% (396) |  | 72.73% (0.00%) | +0.00 (-36.36…+36.36) | 72.73% | 100.00% | 66.67% / 75.00% | 72.73% (11) | 62.50% (8) / – / 100.00% (3) |  | 0.85% |
| adverb | 63 | 175 | **85.14%** (75.8–93.3) | 97.14% | 97.14% | 84.13% | 0.084 | 2.097 | 98.35% | 96.72% | 98.28 / 0.00 / 0.00 |  | 85.14% (175) |  | 44.57% (0.00%) | +40.57 (+25.58…+55.62) | 85.71% | 100.00% | 100.00% / 84.62% | 85.19% (54) | 84.05% (58) / 100.00% (3) / 100.00% (2) |  | 5.56% |

## Where the errors are

The held-out forms the grammar gets wrong from the headword, by what went wrong, as a share
of them: *class*, the right form is among its answers but another paradigm's comes first;
*length* and *accent*, the right letters with a vowel's length or the accent wrong; *stem*
and *ending*, wrong letters inside the stem or after it; *variant*, another of the forms
the dictionary lists comes first; *homograph*, another dictionary word spelled the same has
that form; *gap*, no form at all. *By block*: the mean of each group of cells' top-1, so a
group of many cells doesn't outweigh the rest. *Reproduced*: the shipped grammar on the
dictionary's own words (at most 3,000), from the headword; *complete*: of those words'
cells, the dictionary's or not, the share with a form.

| | wrong forms | class | length | accent | stem | ending | variant | homograph | gap | by block | reproduced | complete |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| noun | 5158 | 15% | 77% | 0% | 2% | 4% | 0% | 2% | 0% | 68.89% | 100.00% | 100.00% |
| proper_noun | 2056 | 8% | 83% | 0% | 2% | 4% | 1% | 2% | 0% | 57.85% | 99.93% | 100.00% |
| adjective | 12698 | 2% | 92% | 0% | 0% | 2% | 1% | 3% | 0% | 81.84% | 99.95% | 100.00% |
| verb | 17359 | 49% | 35% | 0% | 5% | 4% | 1% | 6% | 0% | 77.90% | 100.00% | 100.00% |
| pronoun | 251 | 24% | 15% | 0% | 8% | 49% | 4% | 0% | 0% | 6.34% | 100.00% | 100.00% |
| numeral | 108 | 0% | 100% | 0% | 0% | 0% | 0% | 0% | 0% | 72.73% | 100.00% | 100.00% |
| adverb | 26 | 0% | 92% | 0% | 0% | 4% | 4% | 0% | 0% | 85.14% | 100.00% | 100.00% |

| | lines of grammar | bytes |
|---|---:|---:|
| noun | 6106 | 218378 |
| proper_noun | 5090 | 177378 |
| adjective | 5004 | 181967 |
| verb | 10345 | 440000 |
| pronoun | 1336 | 48889 |
| numeral | 814 | 29809 |
| adverb | 700 | 24640 |

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
