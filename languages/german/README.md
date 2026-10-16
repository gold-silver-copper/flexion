# german

German inflection for **any** German word, attested or not: noun declension,
adjectives in all three declensions and three degrees, and verbs (every finite
form in main and subordinate clauses, imperative, infinitives, participles),
each with how sure the grammar is and why.

```rust
use german::*;

assert_eq!(noun("Mann", Case::Nominative, Number::Plural), "Männer");
assert_eq!(noun("haus", Case::Dative, Number::Plural), "Häusern");
assert_eq!(adjective("schön", Declension::Weak, Case::Nominative, Agreement::Masculine), "schöne");
assert_eq!(verb("geben", Tense::Present, Person::Third, Number::Singular), "gibt");
assert_eq!(participle("singen", Participle::Past), "gesungen");
```

A verb that begins with a prefix inflects as the rest of it does. A separable
prefix goes where the form puts it, an inseparable one stays in front, and the
participle's `ge-` goes:

```rust
use german::*;

assert_eq!(verb("anfangen", Tense::Preterite, Person::First, Number::Singular), "fing an");
assert_eq!(subordinate("anfangen", Tense::Present, Person::Third, Number::Singular), "anfängt");
assert_eq!(participle("anfangen", Participle::Past), "angefangen");
assert_eq!(participle("verfangen", Participle::Past), "verfangen");
```

Nouns are always written capitalized, however the word is typed.

## How it works

This crate is German inflection as a grammar, learned by
[flexion](https://github.com/gold-silver-copper/flexion) from the English Wiktionary's and the German Wiktionary's
German entries. Each part of speech is a module under `src/`, and has what its
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
| noun | 143818 | 162 | 88 | 7749 | 14735 | 254 | 0 |
| adjective | 14839 | 22 | 26 | 284 | 178 | 892 | 0 |
| verb | 9316 | 68 | 5 | 250 | 732 | 4484 | 91 |
| determiner | 36 | 3 | 0 | 6 | 8 | 15 | 1 |

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
| noun | 6323 | 40053 | **93.63%** (93.2–94.1) | 98.74% | 98.85% | 86.92% | 0.007 | 0.782 | 93.63% | 99.79% | 98.04 / 98.43 / 99.10 | 94.05% | 93.63% (40053) |  | 94.77% (0.00%) | -1.15 (-1.46…-0.85) | 94.43% | 99.69% | 96.70% / 89.92% |  | 93.44% (5249) / 98.79% (730) / 87.78% (344) |  | 0.51% |
| adjective | 1487 | 125177 | **98.79%** (98.2–99.3) | 99.80% | 99.84% | 98.45% | 0.005 | 0.134 | 98.79% | 99.88% | 99.44 / 99.44 / 99.43 |  | 98.79% (125079) | 100.00% (98) | 99.02% (100.00%) | -0.23 (-0.73…+0.26) | 99.06% | 100.00% | 98.61% / 98.87% |  | 98.73% (1410) / 100.00% (28) / 100.00% (49) | 96.53% (548) | 0.11% |
| verb | 970 | 50575 | **95.17%** (94.2–96.1) | 98.32% | 98.50% | 85.67% | 0.012 | 0.970 | 95.17% | 95.88% | 95.52 / 95.59 / 95.66 | 95.28% | 95.17% (50575) |  | 73.16% (0.00%) | +22.01 (+20.25…+23.80) | 95.27% | 99.25% | 95.91% / 94.35% |  | 95.00% (936) / – / 99.83% (34) | 91.76% (9057) | 5.12% |
| determiner | 3 | 45 | **40.00%** (0.0–100.0) | 73.33% | 73.33% | 33.33% | 0.598 | 16.615 | 40.00% |  | 69.05 / 69.23 / 69.44 |  | 40.00% (45) |  | 40.00% (0.00%) | +0.00 (+0.00…+0.00) | 40.00% | 100.00% | 0.00% / 40.00% |  | 40.00% (3) / – / – |  | 13.89% |

| | lines of grammar | bytes |
|---|---:|---:|
| noun | 25298 | 968931 |
| adjective | 2395 | 130500 |
| verb | 5272 | 310749 |
| determiner | 451 | 15888 |

## Data and license

The code is MIT or Apache-2.0. The grammar under `src/` is derived from Wiktionary via
[Kaikki](https://kaikki.org/) and [wiktextract](https://github.com/tatuylonen/wiktextract),
and is licensed under [CC BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/); see
`DATA-LICENSE.md`.

## Regenerating

```sh
cd flexion
cargo xtask extract german
cargo xtask build german
```
