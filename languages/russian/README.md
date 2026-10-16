# russian

Russian inflection for **any** Russian word, attested or not: noun declension,
adjectives (long and short forms), verbs (non-past, past, imperative,
participles, gerunds), each with how sure the grammar is and why. Stress is
not written: it is stripped from what you type and never written in answers.

```rust
use russian::*;

assert_eq!(noun("книга", Case::Genitive, Number::Plural), "книг");
assert_eq!(noun("стол", Case::Genitive, Number::Singular), "стола");
assert_eq!(adjective("новый", Case::Dative, Agreement::Feminine, Animacy::Inanimate), "новой");
assert_eq!(non_past("читать", Person::Third, Number::Plural), "читают");
assert_eq!(past("читать", Agreement::Feminine), "читала");
```

A noun's accusative is its genitive if it is animate and its nominative if not.
The grammar knows which for the dictionary's words, and guesses for the others;
tell it, and the guess is gone:

```rust
use russian::*;

let бокр = Noun::new("бокр");
assert_eq!(бокр.form((Case::Genitive, Number::Singular)), "бокра");
let animate = бокр.clone().animacy(Animacy::Animate);
assert_eq!(animate.form((Case::Accusative, Number::Singular)), "бокра");
let inanimate = бокр.animacy(Animacy::Inanimate);
assert_eq!(inanimate.form((Case::Accusative, Number::Singular)), "бокр");

// Each candidate says how likely it is and why.
for c in Noun::new("бокр").candidates((Case::Accusative, Number::Singular)) {
    println!("{c}");
}
```

## How it works

This crate is Russian inflection as a grammar, learned by
[flexion](https://github.com/gold-silver-copper/flexion) from the English Wiktionary's and the Russian Wiktionary's
Russian entries. Each part of speech is a module under `src/`, and has what its
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
- `animacy.rs`: a word's animacy, which decides which cell some cells' forms
  come from.

A word the dictionary lists gets its own forms. Any other word gets the paradigm its
ending suggests, with a probability that says how often such a guess is right, and the
reason (the rule and its counts).

## The grammar

| | words | paradigms | sound laws | rules | listed words | irregular forms | listed by relations |
|---|---:|---:|---:|---:|---:|---:|---:|
| noun | 108878 | 188 | 17 | 1905 | 4491 | 1633 | 24 |
| adjective | 67690 | 42 | 4 | 463 | 1535 | 123 | 4 |
| verb | 46911 | 341 | 64 | 2771 | 4858 | 607 | 111 |
| pronoun | 42 | 10 | 0 | 10 | 13 | 373 | 2 |
| numeral | 77 | 4 | 4 | 11 | 19 | 85 | 2 |

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
| noun | 2611 | 27635 | **94.64%** (94.0–95.2) | 99.40% | 99.52% | 77.98% | 0.007 | 0.403 | 94.64% | 95.81% | 95.98 / 96.06 / 96.20 | 97.30% | 94.64% (27628) | 100.00% (7) | 94.72% (71.43%) | -0.08 (-0.52…+0.36) | 94.99% | 99.82% | 97.06% / 93.82% |  | 94.56% (2394) / 96.55% (181) / 90.08% (36) |  | 0.53% |
| adjective | 3466 | 167759 | **99.75%** (99.6–99.9) | 99.99% | 99.99% | 98.04% | 0.002 | 0.013 | 99.75% |  | 99.85 / 99.88 / 99.87 |  | 99.76% (167747) | 0.00% (12) | 99.76% (91.67%) | -0.01 (-0.12…+0.09) | 99.75% | 99.94% | 99.71% / 99.77% |  | 99.11% (950) / 100.00% (2512) / 100.00% (4) |  | 0.07% |
| verb | 1304 | 20554 | **95.76%** (95.0–96.4) | 99.40% | 99.43% | 86.73% | 0.006 | 0.411 | 95.76% | 98.88% | 96.62 / 97.12 / 97.94 | 95.49% | 95.78% (20517) | 83.78% (37) | 96.65% (56.76%) | -0.89 (-1.39…-0.40) | 95.89% | 99.49% | 95.84% / 95.72% |  | 95.74% (1269) / 100.00% (29) / 81.44% (6) |  | 0.75% |
| pronoun | 4 | 188 | **54.26%** (8.5–100.0) | 97.87% | 97.87% | 50.00% | 0.290 | 1.493 | 54.26% |  | 95.65 / 95.56 / 95.45 |  | 54.26% (188) |  | 54.26% (0.00%) | +0.00 (+0.00…+0.00) | 54.26% | 100.00% | 0.00% / 54.26% |  | 54.26% (4) / – / – |  | 30.95% |
| numeral | 4 | 153 | **94.12%** (67.5–100.0) | 94.12% | 94.12% | 75.00% | 0.006 | 2.198 | 94.12% |  | 93.96 / 93.79 / 93.62 |  | 94.12% (153) |  | 100.00% (0.00%) | -5.88 (-32.53…+0.00) | 94.12% | 100.00% | 100.00% / 91.51% |  | 94.12% (4) / – / – |  | 24.68% |

| | lines of grammar | bytes |
|---|---:|---:|
| noun | 28943 | 1243341 |
| adjective | 3747 | 203000 |
| verb | 13661 | 797402 |
| pronoun | 899 | 41806 |
| numeral | 927 | 37298 |

## Data and license

The code is MIT or Apache-2.0. The grammar under `src/` is derived from Wiktionary via
[Kaikki](https://kaikki.org/) and [wiktextract](https://github.com/tatuylonen/wiktextract),
and is licensed under [CC BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/); see
`DATA-LICENSE.md`.

## Regenerating

```sh
cd flexion
cargo xtask extract russian
cargo xtask build russian
```
