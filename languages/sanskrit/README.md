# sanskrit

Sanskrit inflection for **any** word, attested or not, in Devanagari: nouns and
adjectives in eight cases and three numbers, and verbs (present system with its
four moods, imperfect, future, conditional, perfect, aorist, infinitive,
gerund, participles), each with how sure the grammar is and why.

```rust
use sanskrit::*;

assert_eq!(noun("देव", Case::Instrumental, Number::Singular), "देवेन");
assert_eq!(noun("देव", Case::Genitive, Number::Plural), "देवानाम्");
assert_eq!(adjective("नव", Case::Nominative, Gender::Feminine, Number::Singular), "नवा");

// A noun the dictionary lacks: by its ending masculine, unless you say otherwise.
assert_eq!(noun("सुकुल", Case::Nominative, Number::Singular), "सुकुलः");
let neuter = Noun::new("सुकुल").gender(Gender::Neuter);
assert_eq!(neuter.form((Case::Nominative, Number::Singular)), "सुकुलम्");
```

Nouns and adjectives are cited by their stem, as Sanskrit dictionaries do;
verbs by the third person singular present. The forms are Classical (Vedic
variants and accents are left out), with the sandhi inside the word.

## How it works

This crate is Sanskrit inflection as a grammar, learned by
[flexion](https://github.com/gold-silver-copper/flexion) from the English Wiktionary's
Sanskrit entries. Each part of speech is a module under `src/`, and has what its
grammar needs of:

- `paradigms.rs`: how words inflect, one table per paradigm, named after a word that
  follows it, with a pattern per form (`~а`: the stem, then `а`; `derived`: another
  cell's form, changed);
- `relations.rs`: which cell each cell's form is told from, and the rules on that
  form's ending that change it;
- `laws.rs`: sound laws where a stem meets its ending or a literal before it (along a
  tier, on a copied letter, or on a class letter an ending writes: harmony), each with the
  words whose forms it gives and the forms it gets wrong;
- `rules.rs`: which paradigm a word takes, as rules on its ending, each with how many of
  the dictionary's words with that ending follow each paradigm, and the words no rule
  gets right, listed;
- `accent.rs`: per cell, which rule places the accent, and which words have which
  pattern;
- `prefixes.rs`: prefixes a word inflects behind, separable or not;
- `compounds.rs`: whether a word written in parts inflects each part;
- `marks.rs` and `spellings.rs`: how a word typed without its marks (vowel length,
  `ё`) is spelled;
- `stems.rs`: the stem slots (the cells a word builds on one stem, learned from the
  words that put different stems in different cells) and the ending tables paradigms
  share over a slot (`ἐ~ᾰ`, `~ω`), each named after its commonest word;
- `irregular.rs`: what no paradigm gives a word: the stem it lists for a slot, built
  with an ending table (`aorist`, `ἀγγειλ- like λυω`), and the forms no stem gives;
- a module per principal part, where a part of speech is built on them: each cell
  belongs to one part's system and is inflected from that part.

A word the dictionary lists gets its own forms. Any other word gets the paradigm its
ending suggests, with a probability that says how often such a guess is right, and the
reason (the rule and its counts).

## The grammar

| | words | paradigms | sound laws | rules | listed words | irregular forms |
|---|---:|---:|---:|---:|---:|---:|
| noun | 5023 | 56 | 4 | 285 | 889 | 227 |
| adjective | 1964 | 40 | 1 | 157 | 399 | 451 |
| verb | 1004 | 66 | 7 | 158 | 367 | 1196 |
| pronoun | 15 | 3 | 0 | 3 | 9 | 27 |

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
| noun | 576 | 13801 | **92.10%** ± 2.2 | 98.74% | 98.74% | 73.96% | 0.017 | 0.766 | 92.10% |  | 94.57 / 94.85 / 96.01 | 97.30% | 92.10% (13801) |  | 91.70% (0.00%) | 100.00% |
| adjective | 190 | 13456 | **92.00%** ± 3.9 | 98.84% | 98.84% | 73.16% | 0.022 | 0.872 | 92.00% |  | 93.95 / 94.25 / 97.53 |  | 92.06% (13447) | 11.11% (9) | 89.95% (11.11%) | 100.00% |
| verb | 88 | 7237 | **90.05%** ± 6.3 | 91.75% | 92.12% | 65.91% | 0.054 | 4.270 | 90.05% | 90.77% | 91.59 / 91.72 / 91.73 |  | 90.45% (6953) | 80.28% (284) | 90.01% (80.99%) | 100.00% |
| pronoun | 3 | 207 | **69.57%** ± 52.1 | 69.57% | 69.57% | 66.67% | 0.298 | 10.836 | 69.57% |  | 75.98 / 75.62 / 75.25 |  | 69.57% (207) |  | 70.53% (0.00%) | 100.00% |

| | lines of grammar | bytes |
|---|---:|---:|
| noun | 7542 | 298783 |
| adjective | 4209 | 196380 |
| verb | 6784 | 368060 |
| pronoun | 903 | 36589 |

## Data and license

The code is MIT or Apache-2.0. The grammar under `src/` is derived from Wiktionary via
[Kaikki](https://kaikki.org/) and [wiktextract](https://github.com/tatuylonen/wiktextract),
and is licensed under [CC BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/); see
`DATA-LICENSE.md`.

## Regenerating

```sh
cd flexion
cargo xtask extract sanskrit
cargo xtask build sanskrit
```
