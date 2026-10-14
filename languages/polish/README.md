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
| noun | 37480 | 325 | 48 | 1879 | 7541 | 965 |
| adjective | 13224 | 20 | 17 | 156 | 259 | 71 |
| verb | 12103 | 182 | 34 | 695 | 2585 | 592 |
| pronoun | 28 | 6 | 0 | 6 | 18 | 52 |
| numeral | 40 | 16 | 0 | 12 | 16 | 93 |

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
The margin after top-1 is a 95% interval, counting words. *Lemma in full*: top-1 typed
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
shared task's non-neural baseline (letters only), and on the unseen cells. *Attested*: top-1 of the shipped
grammar on the dictionary's own words, typed in full.

| | held-out words | forms | top-1 | top-3 | among the candidates | every cell right | calibration error | log loss | lemma in full | principal parts given | 1 / 2 / 3 forms given | features given | seen cells | unseen cells | floor (unseen) | attested |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| noun | 4010 | 48482 | **93.90%** ± 0.7 | 98.72% | 98.83% | 76.43% | 0.004 | 0.701 | 93.90% | 96.10% | 95.24 / 95.77 / 96.03 | 96.48% | 93.91% (48466) | 62.50% (16) | 93.78% (25.00%) | 100.00% |
| adjective | 1394 | 83790 | **99.92%** ± 0.1 | 99.99% | 99.99% | 97.70% | 0.000 | 0.011 | 99.92% |  | 99.93 / 99.93 / 99.93 |  | 99.92% (83790) |  | 99.99% (0.00%) | 100.00% |
| verb | 1253 | 55197 | **96.71%** ± 1.0 | 98.78% | 98.78% | 90.42% | 0.009 | 0.807 | 96.71% | 97.92% | 97.82 / 97.96 / 98.53 | 96.76% | 96.72% (55046) | 94.70% (151) | 96.84% (91.39%) | 100.00% |
| pronoun | 6 | 360 | **81.94%** ± 30.8 | 81.94% | 81.94% | 50.00% | 0.654 | 5.721 | 81.94% |  | 81.92 / 97.70 / 97.66 |  | 81.94% (360) |  | 81.94% (0.00%) | 100.00% |
| numeral | 4 | 16 | **75.00%** ± 42.4 | 75.00% | 100.00% | 75.00% | 0.444 | 1.506 | 75.00% |  | 100.00 / 100.00 / 100.00 |  | 66.67% (12) | 100.00% (4) | 100.00% (100.00%) | 100.00% |

| | lines of grammar | bytes |
|---|---:|---:|
| noun | 33804 | 1145488 |
| adjective | 1931 | 84623 |
| verb | 9161 | 399994 |
| pronoun | 912 | 35872 |
| numeral | 1027 | 40882 |

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
