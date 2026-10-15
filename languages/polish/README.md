# polish

Polish inflection for **any** Polish word, attested or not: noun declension,
adjectives, and verbs (non-past, past and conditional by person and gender,
imperative, participles), each with how sure the grammar is and why.

```rust
use polish::*;

assert_eq!(noun("kot", Case::Genitive, Number::Singular), "kota");
assert_eq!(noun("ręka", Case::Locative, Number::Singular), "ręce");
assert_eq!(non_past("czytać", Person::First, Number::Singular), "czytam");
assert_eq!(past("czytać", Person::First, Agreement::Feminine), "czytałam");
```

Whether a masculine noun's genitive takes `-a` or `-u` can't be told from its
spelling; the grammar says how sure it is, and a form you know settles it:

```rust
use polish::*;

for c in Noun::new("blorp").candidates((Case::Genitive, Number::Singular)) {
    println!("{c}");
}
let blorp = Noun::new("blorp").given((Case::Genitive, Number::Singular), "blorpu");
assert_eq!(blorp.form((Case::Locative, Number::Singular)), "blorpie");
```

## How it works

This crate is Polish inflection as a grammar, learned by
[flexion](https://github.com/gold-silver-copper/flexion) from the English Wiktionary's
Polish entries. Each part of speech is a module under `src/`, and has what its
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
  with an ending table, and the forms no stem gives;
- `gender.rs`: a word's gender, which decides which cell some cells' forms
  come from.

A word the dictionary lists gets its own forms. Any other word gets the paradigm its
ending suggests, with a probability that says how often such a guess is right, and the
reason (the rule and its counts).

## The grammar

| | words | paradigms | sound laws | rules | listed words | irregular forms |
|---|---:|---:|---:|---:|---:|---:|
| noun | 37480 | 326 | 40 | 1878 | 7441 | 957 |
| adjective | 13224 | 20 | 17 | 156 | 258 | 71 |
| verb | 12103 | 179 | 34 | 697 | 900 | 593 |
| pronoun | 28 | 6 | 0 | 6 | 18 | 52 |
| numeral | 40 | 15 | 0 | 12 | 17 | 144 |

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
| noun | 4010 | 44544 | **93.37%** (92.9–93.9) | 98.61% | 98.72% | 76.41% | 0.004 | 0.760 | 93.37% | 95.71% | 94.78 / 95.31 / 95.53 | 96.17% | 93.37% (44530) | 71.43% (14) | 93.23% (14.29%) | +0.13 (-0.24…+0.47) | 93.65% | 99.23% | 95.06% / 93.04% | 0.00% (5) | 100.00% |
| adjective | 1394 | 82398 | **99.92%** (99.9–99.9) | 99.99% | 99.99% | 97.70% | 0.001 | 0.011 | 99.92% |  | 99.93 / 99.93 / 99.93 |  | 99.92% (82398) |  | 99.99% (0.00%) | -0.07 (-0.09…-0.04) | 99.92% | 100.00% | 99.94% / 99.91% |  | 100.00% |
| verb | 1253 | 53951 | **96.62%** (95.8–97.4) | 98.76% | 98.76% | 90.26% | 0.012 | 0.821 | 96.62% | 97.88% | 97.74 / 97.93 / 98.52 | 96.67% | 96.62% (53801) | 94.67% (150) | 96.78% (91.33%) | -0.16 (-0.73…+0.45) | 96.90% | 99.77% | 95.22% / 96.93% | 0.00% (7) | 100.00% |
| pronoun | 6 | 354 | **81.64%** (50.0–99.4) | 81.64% | 81.64% | 50.00% | 0.656 | 5.799 | 81.64% |  | 81.61 / 97.66 / 97.62 |  | 81.64% (354) |  | 81.64% (0.00%) | +0.00 (+0.00…+0.00) | 81.64% | 100.00% | 0.00% / 81.64% |  | 100.00% |
| numeral | 4 | 16 | **75.00%** (25.0–100.0) | 75.00% | 100.00% | 75.00% | 0.459 | 1.434 | 75.00% |  | 100.00 / 100.00 / 100.00 |  | 66.67% (12) | 100.00% (4) | 100.00% (100.00%) | -25.00 (-75.00…+0.00) | 75.00% | 100.00% | 0.00% / 75.00% |  | 100.00% |

| | lines of grammar | bytes |
|---|---:|---:|
| noun | 33691 | 1136794 |
| adjective | 1937 | 85937 |
| verb | 7427 | 335745 |
| pronoun | 934 | 37100 |
| numeral | 983 | 40785 |

## Data and license

The code is MIT or Apache-2.0. The grammar under `src/` is derived from Wiktionary via
[Kaikki](https://kaikki.org/) and [wiktextract](https://github.com/tatuylonen/wiktextract),
and is licensed under [CC BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/); see
`DATA-LICENSE.md`.

## Regenerating

```sh
cd flexion
cargo xtask extract polish
cargo xtask build polish
```
