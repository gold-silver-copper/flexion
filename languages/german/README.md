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
| noun | 143828 | 2 | 233 | 9870 | 18601 | 222 |
| adjective | 14840 | 3 | 61 | 440 | 1991 | 1698 |
| verb | 9323 | 1 | 64 | 225 | 681 | 7999 |
| determiner | 37 | 1 | 4 | 7 | 9 | 43 |

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
| noun | 6323 | 46353 | **94.90%** | 99.22% | 0.004 | 94.90% |
| adjective | 1487 | 126633 | **98.66%** | 99.35% | 0.005 | 98.66% |
| verb | 971 | 51599 | **94.81%** | 97.88% | 0.021 | 94.81% |
| determiner | 3 | 48 | **43.75%** | 75.00% | 0.560 | 43.75% |


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
