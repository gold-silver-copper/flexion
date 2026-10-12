# ancient-greek

Ancient Greek (Attic) inflection for **any** word, attested or not: nouns,
adjectives, and verbs in every tense, mood and voice, with accents, breathings
and vowel length, and a confidence for every answer.

```rust
use ancient_greek::*;

assert_eq!(noun("λόγος", Case::Genitive, Number::Plural), "λόγων");
assert_eq!(noun("δωρον", Case::Genitive, Number::Singular), "δώρου");
// Vowel length is marked on α, ι, υ, as Wiktionary's tables do:
assert_eq!(adjective("ἀγαθός", Case::Dative, Gender::Feminine, Number::Singular), "ᾰ̓γᾰθῇ");
let cell = PresentCell {
    mood: Mood::Indicative,
    voice: Voice::Active,
    person: Person::Third,
    number: Number::Plural,
};
assert_eq!(present("λύω", cell), "λῡ́ουσῐ");

// Without accents or length marks:
let plain = AncientGreek::default().accents(false).length_marks(false);
assert_eq!(plain.noun("δῶρον", Case::Nominative, Number::Plural), "δωρα");
```

A word is found whether or not it is typed with accents, breathings, iota
subscripts or length marks; the output writes them. A citation gives principal
parts the way a dictionary prints them: nouns with the genitive and gender
(`λόγος, λόγου m`), adjectives with the feminine and neuter (`ἀγαθός, ἀγαθή,
ἀγαθόν`), verbs with the six principal parts (`λύω, λύσω, ἔλυσα, λέλυκα,
λέλυμαι, ἐλύθην`), or as many as are given.

## How it works

This crate is Ancient Greek inflection as a grammar, learned by
[flexion](https://github.com/gold-silver-copper/flexion) from the English Wiktionary's
Ancient Greek entries. Each part of speech is a module under `src/`:

- `paradigms.rs`: how words inflect, one table per paradigm, named after a word that
  follows it, with a pattern per form (`~а`: the stem, then `а`);
- `rules.rs`: which paradigm a word takes, as rules on its ending, each with how many of
  the dictionary's words with that ending follow each paradigm, and the words no rule
  gets right, listed;
- `irregular.rs`: the forms no paradigm gives.

A word the dictionary lists gets its own forms. Any other word gets the paradigm its
ending suggests, with a probability that says how often such a guess is right, and the
reason (the rule and its counts).

## The grammar

| | words | paradigms | rules | listed words | irregular forms |
|---|---:|---:|---:|---:|---:|
| noun | 8165 | 163 | 676 | 2835 | 9202 |
| adjective | 4699 | 105 | 512 | 2208 | 16327 |
| verb | 2689 | 329 | 441 | 2405 | 232304 |
| pronoun | 58 | 27 | 12 | 36 | 526 |
| numeral | 19 | 8 | 4 | 5 | 24 |

*Words*: the dictionary's words the grammar is learned from. *Rules*: the endings that
choose a paradigm. *Listed words*: words no rule gets right, listed with their paradigm.
*Irregular forms*: forms no paradigm gives, listed.

## How well it does

A tenth of the words (by a hash of the word) was held out: the grammar scored here was
learned from the other nine tenths, and asked for every form the dictionary lists of
each held-out word. *Top-1*: its first answer is right; *top-3*: the right answer is among
its first three. *Calibration error*: how far its stated probabilities are, on average,
from how often it is right (0 is perfect).

| | held-out words | forms | top-1 | top-1, lemma typed without stress | top-3 | calibration error |
|---|---:|---:|---:|---:|---:|---:|
| noun | 815 | 11673 | **78.74%** | 13.73% | 88.91% | 0.045 |
| adjective | 477 | 22484 | **71.76%** | 10.98% | 82.94% | 0.060 |
| verb | 261 | 64485 | **53.38%** | 17.42% | 58.99% | 0.150 |
| pronoun | 11 | 375 | **26.67%** | 1.87% | 38.67% | 0.637 |
| numeral | 2 | 30 | **100.00%** | 0.00% | 100.00% | 0.000 |


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
