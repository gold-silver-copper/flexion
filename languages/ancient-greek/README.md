# ancient-greek

Ancient Greek (Attic) inflection for **any** word, attested or not: nouns,
adjectives, and verbs in every tense, mood and voice, with accents, breathings
and vowel length, each with how sure the grammar is and why.

```rust
use ancient_greek::*;

assert_eq!(noun("λόγος", Case::Genitive, Number::Plural), "λόγων");
assert_eq!(noun("δῶρον", Case::Genitive, Number::Singular), "δώρου");
assert_eq!(present("λύω", Mood::Indicative, Voice::Active, Person::Third, Number::Plural), "λῡ́ουσῐ");
```

The accent is placed by rule. A noun keeps it on the syllable its lemma has it
on, as far as the law of limitation allows: on the third syllable from the end
only before a short last syllable, and a long next-to-last syllable before a
short last one takes the circumflex. A verb's accent recedes as far as the law
allows.

```rust
use ancient_greek::*;

// Vowel length is marked on α, ι, υ, as Wiktionary's tables do.
assert_eq!(noun("ἄνθρωπος", Case::Genitive, Number::Singular), "ᾰ̓νθρώπου");
assert_eq!(noun("δῶρον", Case::Nominative, Number::Plural), "δῶρᾰ");
```

A word is found whether or not it is typed with breathings, iota subscripts or
length marks. For a word the dictionary doesn't have, vowel length is restored
by rules on the letters around each vowel; type the lemma with its marks and
say so (`spelled_in_full`), and none is a guess.

## How it works

This crate is Ancient Greek inflection as a grammar, learned by
[flexion](https://github.com/gold-silver-copper/flexion) from the English Wiktionary's
Ancient Greek entries. Each part of speech is a module under `src/`, and has what its
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
| noun | 8165 | 1 | 163 | 676 | 2835 | 9202 |
| adjective | 4699 | 1 | 105 | 512 | 2208 | 16247 |
| verb | 2689 | 1 | 328 | 434 | 2409 | 232954 |
| pronoun | 58 | 1 | 27 | 12 | 36 | 526 |
| numeral | 19 | 1 | 8 | 4 | 5 | 24 |

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
| noun | 815 | 11673 | **61.88%** | 71.98% | 0.118 | 78.74% |
| adjective | 477 | 22484 | **58.78%** | 70.88% | 0.093 | 71.76% |
| verb | 261 | 64485 | **45.92%** | 50.95% | 0.069 | 53.46% |
| pronoun | 11 | 375 | **2.40%** | 2.40% | 0.631 | 26.67% |
| numeral | 2 | 30 | **100.00%** | 100.00% | 0.284 | 100.00% |

## Data and license

The code is MIT or Apache-2.0. The grammar under `src/` is derived from Wiktionary via
[Kaikki](https://kaikki.org/) and [wiktextract](https://github.com/tatuylonen/wiktextract),
and is licensed under [CC BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/); see
`DATA-LICENSE.md`.

## Regenerating

```sh
cd flexion
cargo xtask extract ancient-greek
cargo xtask build ancient-greek
```
