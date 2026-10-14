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
- `spellings.rs`: the words whose spelling their typed form doesn't show;
- `stems.rs`: the stem slots (the cells a word builds on one stem, learned from the
  words that put different stems in different cells) and the ending tables paradigms
  share over a slot, each named after its commonest word;
- `irregular.rs`: what no paradigm gives a word: the stem it lists for a slot, built
  with an ending table, and the forms no stem gives;
- `auxiliary.rs`: a word's auxiliary, which decides which cell some cells' forms
  come from;
- `gender.rs`: a word's gender, which decides which cell some cells' forms
  come from.

A word the dictionary lists gets its own forms. Any other word gets the paradigm its
ending suggests, with a probability that says how often such a guess is right, and the
reason (the rule and its counts).

## The grammar

| | words | paradigms | sound laws | rules | listed words | irregular forms |
|---|---:|---:|---:|---:|---:|---:|
| noun | 143828 | 172 | 44 | 7315 | 15823 | 283 |
| adjective | 14840 | 23 | 25 | 375 | 1159 | 894 |
| verb | 9323 | 75 | 5 | 254 | 744 | 4433 |
| determiner | 37 | 3 | 0 | 6 | 9 | 16 |

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
| noun | 6323 | 46353 | **94.22%** ± 0.6 | 98.90% | 98.99% | 86.56% | 0.005 | 0.669 | 94.22% | 98.62% | 98.29 / 98.65 / 99.29 | 94.59% | 94.22% (46353) |  | 94.71% (0.00%) | 100.00% |
| adjective | 1487 | 126633 | **98.78%** ± 0.6 | 99.80% | 99.81% | 96.44% | 0.004 | 0.141 | 98.78% | 99.82% | 99.35 / 99.34 / 99.41 |  | 98.78% (126633) |  | 99.01% (0.00%) | 100.00% |
| verb | 971 | 51599 | **95.20%** ± 1.3 | 98.28% | 98.47% | 85.68% | 0.013 | 1.014 | 95.20% | 95.88% | 95.72 / 95.82 / 95.88 | 95.22% | 95.19% (51549) | 100.00% (50) | 73.53% (48.00%) | 100.00% |
| determiner | 3 | 48 | **43.75%** ± 56.1 | 75.00% | 75.00% | 33.33% | 0.560 | 15.581 | 43.75% |  | 75.56 / 69.05 / 66.67 |  | 43.75% (48) |  | 43.75% (0.00%) | 100.00% |

| | lines of grammar | bytes |
|---|---:|---:|
| noun | 61260 | 2033334 |
| adjective | 3457 | 188038 |
| verb | 7164 | 373510 |
| determiner | 518 | 17964 |

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
