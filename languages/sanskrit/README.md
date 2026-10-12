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
  follows it, with a pattern per form (`~а`: the stem, then `а`; `referred`: another
  cell's form, as the word's feature says);
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

| | words | systems | paradigms | rules | listed words | irregular forms |
|---|---:|---:|---:|---:|---:|---:|
| noun | 5023 | 1 | 56 | 292 | 1058 | 1319 |
| adjective | 1964 | 1 | 36 | 127 | 413 | 2700 |
| verb | 1014 | 1 | 40 | 127 | 429 | 8458 |
| pronoun | 15 | 1 | 1 | 1 | 11 | 308 |

*Words*: the dictionary's words the grammar is learned from. *Systems*: the principal
parts a part of speech is built on (1: its lemma alone). *Rules*: the endings that
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
parts. *Floor*: the longest-suffix rule (the SIGMORPHON shared tasks' non-neural
baseline; letters only). *Attested*: top-1 of the shipped grammar on the dictionary's
own words, typed in full.

| | held-out words | forms | top-1 | top-3 | calibration error | lemma in full | principal parts given | floor | attested |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| noun | 576 | 13801 | **91.58%** ± 2.3 | 98.51% | 0.022 | 91.58% |  | 92.04% | 98.13% |
| adjective | 190 | 13456 | **90.21%** ± 4.2 | 98.36% | 0.039 | 90.21% |  | 91.12% | 99.90% |
| verb | 88 | 7237 | **87.61%** ± 6.9 | 88.77% | 0.101 | 87.61% | 87.94% | 72.49% | 99.99% |
| pronoun | 3 | 207 | **67.15%** ± 53.1 | 67.15% | 0.329 | 67.15% |  | 70.53% | 98.30% |
| | lines of grammar | bytes |
|---|---:|---:|
| noun | 4820 | 229573 |
| adjective | 4877 | 261267 |
| verb | 12471 | 727937 |
| pronoun | 821 | 33639 |

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
