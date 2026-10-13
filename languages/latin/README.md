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
    verb("amo", Mood::Indicative, Voice::Active, Tense::Perfect, Person::Third, Number::Plural),
    "amāvērunt"
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
  follows it, with a pattern per form (`~а`: the stem, then `а`; `derived`: another
  cell's form, changed);
- `relations.rs`: which cell each cell's form is told from, and the rules on that
  form's ending that change it;
- `laws.rs`: sound laws where a stem meets its ending or a literal before it (along a
  tier, or on a copied letter), each with the words whose forms it gives and the forms it gets wrong;
- `rules.rs`: which paradigm a word takes, as rules on its ending, each with how many of
  the dictionary's words with that ending follow each paradigm, and the words no rule
  gets right, listed;
- `accent.rs`: per cell, which rule places the accent, and which words have which
  pattern;
- `prefixes.rs`: prefixes a word inflects behind, separable or not;
- `compounds.rs`: whether a word written in parts inflects each part;
- `marks.rs` and `spellings.rs`: how a word typed without its marks (vowel length,
  `ё`) is spelled;
- `irregular.rs`: the forms no paradigm gives;
- a module per principal part, where a part of speech is built on them: each cell
  belongs to one part's system and is inflected from that part.

A word the dictionary lists gets its own forms. Any other word gets the paradigm its
ending suggests, with a probability that says how often such a guess is right, and the
reason (the rule and its counts).

## The grammar

| | words | paradigms | sound laws | rules | listed words | irregular forms |
|---|---:|---:|---:|---:|---:|---:|
| noun | 15894 | 81 | 9 | 496 | 1298 | 363 |
| adjective | 27465 | 44 | 1 | 255 | 469 | 357 |
| verb | 6171 | 202 | 38 | 792 | 1157 | 627 |
| pronoun | 81 | 14 | 4 | 15 | 59 | 287 |
| numeral | 118 | 9 | 0 | 12 | 5 | 2 |

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
(its gender, as `.gender(…)` tells it). *Seen* and *unseen cells*: top-1 on the cells
some training word of the word's paradigm has a form in, and on the others, derived
from other cells (with the number of forms). *Floor*: the CoNLL–SIGMORPHON 2017
shared task's non-neural baseline (letters only), and on the unseen cells. *Attested*: top-1 of the shipped
grammar on the dictionary's own words, typed in full.

| | held-out words | forms | top-1 | top-3 | calibration error | lemma in full | principal parts given | features given | seen cells | unseen cells | floor (unseen) | attested |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| noun | 1565 | 18407 | **66.41%** ± 2.3 | 85.33% | 0.031 | 93.45% | 98.35% | 67.19% | 66.43% (18401) | 0.00% (6) | 59.45% (0.00%) | 100.00% |
| adjective | 2841 | 102282 | **82.54%** ± 1.4 | 93.04% | 0.014 | 99.28% | 99.51% |  | 82.54% (102281) | 0.00% (1) | 71.85% (0.00%) | 100.00% |
| verb | 619 | 60971 | **70.47%** ± 3.6 | 90.42% | 0.048 | 85.24% | 97.17% |  | 70.44% (60845) | 84.13% (126) | 65.34% (70.63%) | 100.00% |
| pronoun | 9 | 270 | **22.59%** ± 27.3 | 37.41% | 0.575 | 34.81% |  |  | 22.59% (270) |  | 25.56% (0.00%) | 100.00% |
| numeral | 11 | 396 | **72.73%** ± 26.3 | 100.00% | 0.240 | 100.00% |  |  | 72.73% (396) |  | 72.73% (0.00%) | 100.00% |

| | lines of grammar | bytes |
|---|---:|---:|
| noun | 13422 | 422381 |
| adjective | 11186 | 373351 |
| verb | 24606 | 1007611 |
| pronoun | 1268 | 48860 |
| numeral | 914 | 32202 |

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
