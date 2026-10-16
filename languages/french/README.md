# french

French inflection for **any** French word, attested or not: noun plurals,
adjective and participle agreement, and the whole simple conjugation
(indicative, subjunctive, imperative, participles), each with how sure the
grammar is and why.

```rust
use french::*;

assert_eq!(noun("cheval", Number::Plural), "chevaux");
assert_eq!(adjective("beau", Gender::Feminine, Number::Singular), "belle");
assert_eq!(before_vowel("nouveau"), "nouvel");
assert_eq!(verb("appeler", Tense::Present, Person::First, Number::Singular), "appelle");
assert_eq!(verb("manger", Tense::Present, Person::First, Number::Plural), "mangeons");
assert_eq!(verb("prendre", Tense::PastHistoric, Person::Third, Number::Singular), "prit");
assert_eq!(agreeing_participle("prendre", Gender::Feminine, Number::Plural), "prises");
```

Accents and `ç` are letters: `pêcher` and `pécher` are two words.

## How it works

This crate is French inflection as a grammar, learned by
[flexion](https://github.com/gold-silver-copper/flexion) from the English Wiktionary's
French entries. Each part of speech is a module under `src/`, and has what its
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
| noun | 43084 | 15 | 0 | 266 | 624 | 280 | 0 |
| adjective | 18156 | 17 | 6 | 227 | 485 | 61 | 0 |
| verb | 7295 | 56 | 15 | 292 | 148 | 204 | 35 |
| participle | 6500 | 5 | 0 | 17 | 25 | 2 | 0 |
| determiner | 49 | 9 | 1 | 14 | 12 | 14 | 5 |

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
| noun | 4317 | 4317 | **97.59%** (97.1–98.0) | 99.63% | 99.77% | 97.59% | 0.004 | 0.222 | 97.59% |  |  | 97.59% | 97.59% (4317) |  | 97.03% (0.00%) | +0.56 (+0.23…+0.90) | 97.98% | 99.88% | 96.88% / 97.85% |  | 97.97% (3898) / 99.18% (245) / 86.78% (174) |  | 0.65% |
| adjective | 1839 | 5517 | **97.50%** (96.9–98.1) | 99.53% | 99.60% | 96.52% | 0.009 | 0.272 | 97.50% | 99.95% | 98.67 / 99.67 / 100.00 |  | 97.53% (5515) | 0.00% (2) | 97.72% (50.00%) | -0.22 (-0.65…+0.24) | 97.57% | 100.00% | 98.85% / 97.12% |  | 97.90% (1781) / – / 85.06% (58) |  | 0.28% |
| verb | 743 | 34807 | **98.70%** (98.1–99.3) | 99.81% | 99.81% | 97.04% | 0.004 | 0.157 | 98.70% | 99.48% | 99.22 / 99.40 / 99.52 |  | 98.70% (34807) |  | 99.28% (0.00%) | -0.59 (-1.06…-0.22) | 98.84% | 100.00% | 99.63% / 98.40% |  | 98.74% (724) / – / 97.08% (19) |  | 0.96% |
| participle | 627 | 1881 | **99.84%** (99.5–100.0) | 100.00% | 100.00% | 99.84% | 0.005 | 0.011 | 99.84% |  | 100.00 / 100.00 / 0.00 |  | 99.84% (1881) |  | 99.52% (0.00%) | +0.32 (-0.32…+0.96) | 99.84% | 100.00% | 100.00% / 99.80% |  | 100.00% (1) / 99.84% (626) / – |  | 0.03% |
| determiner | 6 | 13 | **46.15%** (14.3–85.7) | 46.15% | 46.15% | 50.00% | 0.538 | 15.385 | 46.15% |  | 57.14 / 66.67 / 0.00 |  | 46.15% (13) |  | 69.23% (0.00%) | -23.08 (-64.29…+0.00) | 46.15% | 100.00% | 0.00% / 46.15% |  | 46.15% (6) / – / – |  | 22.45% |

| | lines of grammar | bytes |
|---|---:|---:|
| noun | 1997 | 63040 |
| adjective | 1393 | 46841 |
| verb | 2845 | 114542 |
| participle | 446 | 14993 |
| determiner | 547 | 18441 |

## Data and license

The code is MIT or Apache-2.0. The grammar under `src/` is derived from Wiktionary via
[Kaikki](https://kaikki.org/) and [wiktextract](https://github.com/tatuylonen/wiktextract),
and is licensed under [CC BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/); see
`DATA-LICENSE.md`.

## Regenerating

```sh
cd flexion
cargo xtask extract french
cargo xtask build french
```
