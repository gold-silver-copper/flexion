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
| noun | 108776 | 1 | 204 | 1788 | 4664 | 2691 |
| adjective | 67544 | 1 | 41 | 459 | 1987 | 755 |
| verb | 46887 | 1 | 348 | 2242 | 4807 | 1705 |
| pronoun | 42 | 1 | 11 | 9 | 15 | 434 |
| numeral | 77 | 1 | 5 | 9 | 19 | 461 |

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
| noun | 2610 | 30201 | **94.55%** ± 0.9 | 99.06% | 0.008 | 94.94% | 96.08% | 92.65% | 99.79% |
| adjective | 3464 | 171100 | **98.62%** ± 0.4 | 99.21% | 0.022 | 99.19% |  | 99.63% | 99.87% |
| verb | 1300 | 21793 | **96.00%** ± 1.1 | 99.47% | 0.006 | 96.00% | 99.01% | 92.47% | 99.57% |
| pronoun | 4 | 192 | **55.21%** ± 48.7 | 55.21% | 0.307 | 55.21% |  | 55.21% | 100.00% |
| numeral | 4 | 156 | **94.23%** ± 22.8 | 94.23% | 0.058 | 94.23% |  | 100.00% | 100.00% |
| | lines of grammar | bytes |
|---|---:|---:|
| noun | 31227 | 1320403 |
| adjective | 7170 | 391384 |
| verb | 14415 | 793155 |
| pronoun | 1215 | 49271 |
| numeral | 1049 | 39483 |

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
