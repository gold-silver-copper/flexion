# finnish

Finnish inflection for **any** Finnish word, attested or not: nouns, names,
adjectives, pronouns and numerals in fourteen cases, two numbers and the
possessive forms; verbs in every mood, person, tense and polarity, the
negatives and perfects the phrases they are (`en tule`, `olen tullut`), and
their infinitives and participles; postpositions with their possessive
suffixes; adverbs in their degrees. Each answer says how sure the grammar is
and why. Vowel harmony is a learned sound law: an ending is written once
(`-ssA`) and the law chooses `a` or `ä` from the word's vowels. A verb of two
words inflects its first (`pyydän anteeksi`, `kiinni otettu`).

```rust
use finnish::*;

assert_eq!(noun("talo", Case::Inessive, Number::Singular), "talossa");
assert_eq!(noun("kylä", Case::Inessive, Number::Singular), "kylässä");
let me = Person::FirstSingular;
assert_eq!(indicative("tulla", Tense::Present, Polarity::Affirmative, me), "tulen");
assert_eq!(indicative("tulla", Tense::Perfect, Polarity::Negative, Person::ThirdSingular), "ei ole tullut");
assert_eq!(indicative("pyytää anteeksi", Tense::Present, Polarity::Affirmative, me), "pyydän anteeksi");
assert_eq!(postposition("alla", Suffix::FirstSingular), "allani");
```

## How it works

This crate is Finnish inflection as a grammar, learned by
[flexion](https://github.com/gold-silver-copper/flexion) from the English Wiktionary's
Finnish entries. Each part of speech is a module under `src/`, and has what its
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
| noun | 126530 | 100 | 21 | 4380 | 5593 | 10168 | 25 |
| pronoun | 75 | 11 | 1 | 16 | 62 | 366 | 32 |
| numeral | 75 | 23 | 0 | 9 | 48 | 266 | 140 |
| proper_noun | 9040 | 83 | 15 | 647 | 1354 | 1389 | 131 |
| adjective | 17353 | 84 | 9 | 615 | 624 | 680 | 16 |
| verb | 12219 | 103 | 26 | 443 | 655 | 5220 | 264 |
| adverb | 2152 | 14 | 0 | 65 | 104 | 69 | 1 |
| postposition | 41 | 6 | 1 | 8 | 7 | 4 | 0 |

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
| noun | 12869 | 1899864 | **97.81%** (97.6–98.0) | 99.10% | 99.12% | 94.94% | 0.009 | 0.805 | 97.81% | 99.15% | 98.74 / 98.77 / 98.90 |  | 97.82% (1899576) | 57.29% (288) | 98.29% (98.96%) | -0.48 (-0.68…-0.29) | 97.98% | 99.98% | 98.56% / 96.30% |  | 97.87% (12649) / 100.00% (7) / 94.16% (213) |  | 1.34% |
| pronoun | 9 | 187 | **2.14%** (0.0–6.6) | 3.74% | 6.95% | 0.00% | 0.336 | 50.334 | 2.14% |  | 9.55 / 8.88 / 18.12 |  | 2.16% (185) | 0.00% (2) | 9.63% (0.00%) | -7.49 (-16.57…-1.39) | 2.14% | 100.00% | 0.00% / 2.92% |  | 2.63% (7) / 0.00% (1) / 0.00% (1) |  | 73.33% |
| numeral | 9 | 224 | **32.14%** (9.8–64.6) | 44.20% | 44.20% | 22.22% | 0.288 | 47.482 | 32.14% |  | 54.42 / 54.37 / 54.31 |  | 32.14% (224) |  | 43.30% (0.00%) | -11.16 (-41.96…+19.73) | 32.14% | 100.00% | 0.00% / 36.18% |  | 23.62% (8) / – / 100.00% (1) |  | 44.00% |
| proper_noun | 917 | 119536 | **93.86%** (92.6–95.1) | 98.80% | 98.80% | 86.70% | 0.018 | 0.968 | 93.86% | 99.03% | 97.62 / 98.05 / 98.32 |  | 94.04% (119245) | 22.68% (291) | 91.77% (69.07%) | +2.10 (+0.76…+3.41) | 94.36% | 99.88% | 98.18% / 92.76% |  | 93.96% (911) / – / 75.61% (6) |  | 2.49% |
| adjective | 1721 | 44395 | **97.45%** (96.9–98.0) | 99.33% | 99.33% | 94.71% | 0.009 | 0.500 | 97.45% |  | 98.64 / 98.70 / 98.87 |  | 97.45% (44392) | 33.33% (3) | 97.41% (66.67%) | +0.04 (-0.62…+0.72) | 97.52% | 100.00% | 99.15% / 96.40% |  | 97.37% (1570) / 98.54% (104) / 97.81% (47) |  | 0.85% |
| verb | 1219 | 196574 | **98.49%** (98.0–98.9) | 99.37% | 99.37% | 93.44% | 0.003 | 0.497 | 98.49% | 99.48% | 99.01 / 99.02 / 99.03 |  | 98.49% (196426) | 97.97% (148) | 97.56% (2.70%) | +0.92 (+0.22…+1.68) | 98.49% | 99.93% | 99.35% / 98.27% |  | 98.48% (1103) / 100.00% (59) / 96.86% (57) | 98.94% (117603) | 3.97% |
| adverb | 211 | 421 | **96.67%** (94.5–98.6) | 98.81% | 98.81% | 94.31% | 0.016 | 0.692 | 96.67% | 98.57% | 98.57 / 0.00 / 0.00 |  | 96.67% (421) |  | 97.39% (0.00%) | -0.71 (-1.67…+0.00) | 96.67% | 100.00% | 100.00% / 96.47% |  | 96.67% (211) / – / – |  | 2.32% |
| postposition | 5 | 23 | **65.22%** (21.7–100.0) | 91.30% | 91.30% | 60.00% | 0.177 | 4.182 | 65.22% |  | 83.33 / 92.31 / 87.50 |  | 65.22% (23) |  | 78.26% (0.00%) | -13.04 (-30.43…+0.00) | 65.22% | 100.00% | 0.00% / 65.22% |  | 65.22% (5) / – / – |  | 4.88% |

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
| noun | 41613 | 35% | 31% | 0% | 4% | 21% | 8% | 1% | 0% | 97.79% | 100.00% | 100.00% |
| pronoun | 183 | 5% | 4% | 0% | 13% | 78% | 0% | 0% | 0% | 2.14% | 100.00% | 100.00% |
| numeral | 152 | 18% | 0% | 0% | 27% | 55% | 0% | 0% | 0% | 32.14% | 100.00% | 100.00% |
| proper_noun | 7336 | 68% | 8% | 0% | 3% | 12% | 8% | 2% | 0% | 93.80% | 100.00% | 100.00% |
| adjective | 1132 | 27% | 52% | 0% | 2% | 16% | 3% | 0% | 0% | 97.31% | 100.00% | 100.00% |
| verb | 2978 | 54% | 5% | 0% | 0% | 36% | 0% | 5% | 0% | 98.32% | 100.00% | 100.00% |
| adverb | 14 | 64% | 0% | 0% | 14% | 21% | 0% | 0% | 0% | 96.67% | 100.00% | 100.00% |
| postposition | 8 | 75% | 0% | 0% | 0% | 25% | 0% | 0% | 0% | 65.22% | 100.00% | 100.00% |

| | lines of grammar | bytes |
|---|---:|---:|
| noun | 13895 | 834876 |
| pronoun | 1154 | 42950 |
| numeral | 1559 | 61058 |
| proper_noun | 8233 | 407685 |
| adjective | 4684 | 194402 |
| verb | 10228 | 625622 |
| adverb | 642 | 23340 |
| postposition | 459 | 17032 |

## Data and license

The code is MIT or Apache-2.0. The grammar under `src/` is derived from Wiktionary via
[Kaikki](https://kaikki.org/) and [wiktextract](https://github.com/tatuylonen/wiktextract),
and is licensed under [CC BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/); see
`DATA-LICENSE.md`.

## Regenerating

```sh
cd flexion
cargo xtask extract finnish
cargo xtask build finnish
```
