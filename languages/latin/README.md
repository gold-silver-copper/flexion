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
| noun | 15893 | 1 | 85 | 439 | 1251 | 1180 |
| adjective | 27464 | 1 | 45 | 190 | 458 | 2282 |
| verb | 6171 | 1 | 217 | 648 | 1166 | 5043 |
| pronoun | 86 | 1 | 22 | 15 | 61 | 1286 |
| numeral | 118 | 1 | 9 | 11 | 5 | 6 |

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
| noun | 1565 | 18407 | **65.24%** ± 2.4 | 68.78% | 0.048 | 93.67% | 98.33% | 54.53% | 99.72% |
| adjective | 2841 | 102282 | **81.74%** ± 1.4 | 82.82% | 0.051 | 99.17% | 99.47% | 61.20% | 99.98% |
| verb | 619 | 60971 | **70.33%** ± 3.6 | 80.43% | 0.030 | 85.37% | 97.49% | 60.65% | 99.23% |
| pronoun | 9 | 270 | **22.59%** ± 27.3 | 30.37% | 0.437 | 34.81% |  | 25.56% | 97.13% |
| numeral | 11 | 396 | **72.73%** ± 26.3 | 72.73% | 0.264 | 100.00% |  | 63.64% | 100.00% |
| | lines of grammar | bytes |
|---|---:|---:|
| noun | 10043 | 313775 |
| adjective | 10773 | 356661 |
| verb | 30029 | 1614713 |
| pronoun | 2337 | 69803 |
| numeral | 696 | 27060 |

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
