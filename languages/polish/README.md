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

| | words | paradigms | sound laws | rules | listed words | irregular forms | listed by relations |
|---|---:|---:|---:|---:|---:|---:|---:|
| noun | 37278 | 304 | 41 | 1970 | 7581 | 882 | 41 |
| adjective | 13224 | 20 | 17 | 156 | 258 | 71 | 0 |
| verb | 12038 | 149 | 31 | 705 | 812 | 414 | 10 |
| pronoun | 28 | 6 | 0 | 6 | 18 | 52 | 3 |
| numeral | 40 | 15 | 0 | 12 | 17 | 144 | 0 |

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
| noun | 3981 | 44404 | **93.42%** (92.9–93.9) | 98.71% | 98.81% | 76.81% | 0.003 | 0.713 | 93.42% | 95.79% | 94.89 / 95.42 / 95.64 | 96.25% | 93.45% (44372) | 56.25% (32) | 93.34% (59.38%) | +0.08 (-0.29…+0.44) | 93.70% | 99.23% | 95.10% / 93.10% |  | 92.44% (3110) / 98.91% (809) / 88.39% (62) |  | 1.81% |
| adjective | 1394 | 82398 | **99.92%** (99.9–99.9) | 99.99% | 99.99% | 97.70% | 0.001 | 0.011 | 99.92% |  | 99.93 / 99.93 / 99.93 |  | 99.92% (82398) |  | 99.99% (0.00%) | -0.07 (-0.09…-0.04) | 99.92% | 100.00% | 99.94% / 99.91% |  | 99.90% (1147) / 100.00% (239) / 100.00% (8) |  | 0.36% |
| verb | 1246 | 53640 | **97.14%** (96.4–97.8) | 99.30% | 99.30% | 90.61% | 0.009 | 0.487 | 97.14% | 98.24% | 98.10 / 98.24 / 98.78 | 97.27% | 97.15% (53488) | 94.74% (152) | 97.62% (92.11%) | -0.48 (-0.99…+0.02) | 97.43% | 99.77% | 95.62% / 97.48% |  | 97.17% (1233) / 100.00% (2) / 93.03% (11) |  | 1.39% |
| pronoun | 6 | 354 | **81.64%** (50.0–99.4) | 81.64% | 81.64% | 50.00% | 0.656 | 5.799 | 81.64% |  | 97.70 / 97.66 / 97.62 |  | 81.64% (354) |  | 81.64% (0.00%) | +0.00 (+0.00…+0.00) | 81.64% | 100.00% | 0.00% / 81.64% |  | 75.00% (4) / 100.00% (1) / 89.83% (1) |  | 60.71% |
| numeral | 4 | 16 | **75.00%** (25.0–100.0) | 75.00% | 100.00% | 75.00% | 0.459 | 1.434 | 75.00% |  | 100.00 / 100.00 / 100.00 |  | 66.67% (12) | 100.00% (4) | 100.00% (100.00%) | -25.00 (-75.00…+0.00) | 75.00% | 100.00% | 0.00% / 75.00% |  | 75.00% (4) / – / – |  | 22.50% |

| | lines of grammar | bytes |
|---|---:|---:|
| noun | 21953 | 821939 |
| adjective | 1960 | 73741 |
| verb | 5402 | 245084 |
| pronoun | 916 | 36354 |
| numeral | 974 | 40459 |

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
