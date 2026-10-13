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
| noun | 5023 | 41 | 8 | 280 | 890 | 514 |
| adjective | 1964 | 33 | 5 | 158 | 416 | 498 |
| verb | 1004 | 34 | 21 | 146 | 407 | 1515 |
| pronoun | 15 | 1 | 0 | 1 | 11 | 32 |

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
| noun | 576 | 13801 | **92.13%** ± 2.2 | 98.72% | 0.012 | 92.13% |  | 97.03% | 92.13% (13801) |  | 91.70% (0.00%) | 100.00% |
| adjective | 190 | 13456 | **91.25%** ± 4.0 | 98.73% | 0.034 | 91.25% |  |  | 91.29% (13449) | 0.00% (7) | 89.95% (0.00%) | 100.00% |
| verb | 88 | 7237 | **88.14%** ± 6.8 | 90.55% | 0.052 | 88.14% | 88.40% |  | 88.47% (7210) | 0.00% (27) | 90.01% (0.00%) | 100.00% |
| pronoun | 3 | 207 | **67.15%** ± 53.1 | 67.15% | 0.329 | 67.15% |  |  | 67.15% (207) |  | 70.53% (0.00%) | 100.00% |

| | lines of grammar | bytes |
|---|---:|---:|
| noun | 7053 | 282699 |
| adjective | 4058 | 187981 |
| verb | 5344 | 310520 |
| pronoun | 897 | 35144 |

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
