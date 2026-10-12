# english

English inflection for **any** English word, attested or not: noun plurals,
verb forms, comparatives and superlatives, each with how sure the grammar is
and why.

```rust
use english::*;

assert_eq!(noun("cat", Number::Plural), "cats");
assert_eq!(noun("child", Number::Plural), "children");
assert_eq!(verb("go", VerbForm::Past), "went");
assert_eq!(adjective("good", Degree::Comparative), "better");
```

A word nobody wrote down inflects by the rules learned from the words that
were. Every answer can come with its alternatives, how likely each is, and why:

```rust
use english::*;

let plurals = Noun::new("glorp").candidates(Number::Plural);
assert_eq!(plurals[0].form, "glorps");
println!("{}", plurals[0]);
```

Agreement, pronouns and possessives are hand-written helpers on top:

```rust
use english::*;

assert_eq!(conjugate("be", Tense::Present, Person::First, Number::Singular), "am");
assert_eq!(conjugate("walk", Tense::Present, Person::Third, Number::Singular), "walks");
assert_eq!(pronoun(Person::Third, Number::Singular, Gender::Feminine, Case::Accusative), "her");
assert_eq!(count_with_number("mouse", 3), "3 mice");
assert_eq!(add_possessive("dogs"), "dogs'");
```

## How it works

This crate is English inflection as a grammar, learned by
[flexion](https://github.com/gold-silver-copper/flexion) from the English Wiktionary's
English entries. Each part of speech is a module under `src/`, and has what its
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
| noun | 228072 | 1 | 130 | 2373 | 6934 | 253 |
| verb | 38230 | 3 | 294 | 2388 | 3370 | 230 |
| adjective | 61454 | 1 | 65 | 5012 | 20155 | 22737 |
| adverb | 12620 | 1 | 28 | 838 | 3869 | 4613 |

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
| noun | 23733 | 47466 | **98.60%** | 99.84% | 0.001 | 98.60% |
| verb | 4151 | 20745 | **97.78%** | 99.80% | 0.003 | 97.78% |
| adjective | 6239 | 18715 | **96.53%** | 99.55% | 0.050 | 96.53% |
| adverb | 1207 | 3621 | **99.17%** | 99.70% | 0.070 | 99.17% |


## Data and license

The code is MIT or Apache-2.0. The grammar under `src/` is derived from Wiktionary via
[Kaikki](https://kaikki.org/) and [wiktextract](https://github.com/tatuylonen/wiktextract),
and is licensed under [CC BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/); see
`DATA-LICENSE.md`.

## Regenerating

```sh
cd flexion
cargo xtask extract english
cargo xtask build english
```
