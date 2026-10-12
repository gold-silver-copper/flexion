# polish

Polish inflection for **any** Polish word, attested or not: noun declension,
adjectives, and verbs (non-past, past and conditional by person and gender,
imperative, participles), each with how sure the grammar is and why.

```rust
use polish::*;

assert_eq!(noun("kot", Case::Genitive, Number::Singular), "kota");
assert_eq!(noun("ręka", Case::Locative, Number::Singular), "ręce");
assert_eq!(non_past("czytać", Person::First, Number::Singular), "czytam");
assert_eq!(past("czytać", Person::First, Agreement::Feminine), "czytałam");
```

Whether a masculine noun's genitive takes `-a` or `-u` can't be told from its
spelling; the grammar says how sure it is, and a form you know settles it:

```rust
use polish::*;

for c in Noun::new("blorp").candidates((Case::Genitive, Number::Singular)) {
    println!("{c}");
}
let blorp = Noun::new("blorp").given((Case::Genitive, Number::Singular), "blorpu");
assert_eq!(blorp.form((Case::Locative, Number::Singular)), "blorpie");
```

## How it works

This crate is Polish inflection as a grammar, learned by
[flexion](https://github.com/gold-silver-copper/flexion) from the English Wiktionary's
Polish entries. Each part of speech is a module under `src/`, and has what its
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
| noun | 37480 | 2 | 401 | 2991 | 11950 | 1054 |
| adjective | 13224 | 1 | 40 | 146 | 141 | 394 |
| verb | 12103 | 3 | 268 | 1280 | 1588 | 2558 |
| pronoun | 28 | 1 | 16 | 7 | 10 | 19 |
| numeral | 40 | 1 | 19 | 11 | 11 | 20 |

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
*Top-1, lemma in full*: typed with every mark the dictionary's table writes, and said
to be so (`spelled_in_full`).

| | held-out words | forms | top-1 | top-3 | calibration error | top-1, lemma in full |
|---|---:|---:|---:|---:|---:|---:|
| noun | 4010 | 48482 | **93.37%** | 98.91% | 0.004 | 93.37% |
| adjective | 1394 | 83790 | **99.98%** | 99.99% | 0.000 | 99.98% |
| verb | 1253 | 55197 | **96.79%** | 98.88% | 0.011 | 96.79% |
| pronoun | 6 | 360 | **81.94%** | 81.94% | 0.253 | 81.94% |
| numeral | 4 | 16 | **75.00%** | 75.00% | 0.299 | 75.00% |


## Data and license

The code is MIT or Apache-2.0. The grammar under `src/` is derived from Wiktionary via
[Kaikki](https://kaikki.org/) and [wiktextract](https://github.com/tatuylonen/wiktextract),
and is licensed under [CC BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/); see
`DATA-LICENSE.md`.

## Regenerating

```sh
cd flexion
cargo xtask extract polish
cargo xtask build polish
```
