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

// Vowel length is marked on α, ι, υ, as Wiktionary's tables do, except
// where a breathing takes the letter (`ᾰ̓` is written `ἀ`).
assert_eq!(noun("ἄνθρωπος", Case::Genitive, Number::Singular), "ἀνθρώπου");
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
  follows it, with a pattern per form (`~`: the stem, then what follows it;
  `derived`: another cell's form, changed);
- `relations.rs`: which cell each cell's form is told from, and the rules on that
  form's ending that change it;
- `laws.rs`: sound laws where a stem meets its ending or a literal before it (along a
  tier, on a copied letter, or on a class letter an ending writes: harmony), each
  with the words whose forms it gives and the forms it gets wrong;
- `rules.rs`: which paradigm a word takes, as rules on its ending, each with how many
  of the dictionary's words with that ending follow each paradigm, and the words no
  rule gets right, listed;
- `accent.rs`: per cell, which rule places the accent, and which words have which
  pattern;
- `prefixes.rs`: prefixes a word inflects behind, separable or not;
- `heads.rs`: the words a word ending in one inflects as, its front kept (compounds);
- `marks.rs`: how a letter typed without its mark (vowel length) is written;
- `spellings.rs`: the words whose spelling their typed form doesn't show;
- `stems.rs`: the stem slots (the cells a word builds on one stem, learned from the
  words that put different stems in different cells) and the ending tables paradigms
  share over a slot, each named after its commonest word;
- `irregular.rs`: what no paradigm gives a word: the stem it lists for a slot, built
  with an ending table, and the forms no stem gives.

A word the dictionary lists gets its own forms. Any other word gets the paradigm its
ending suggests, with a probability that says how often such a guess is right, and the
reason (the rule and its counts).

## The grammar

| | words | paradigms | sound laws | rules | listed words | irregular forms | listed by relations |
|---|---:|---:|---:|---:|---:|---:|---:|
| noun | 8124 | 86 | 6 | 316 | 1697 | 2814 | 43 |
| adjective | 4674 | 46 | 49 | 330 | 1027 | 4149 | 47 |
| verb | 2655 | 286 | 31 | 234 | 1534 | 33861 | 462 |
| pronoun | 58 | 19 | 0 | 14 | 36 | 439 | 13 |
| numeral | 19 | 7 | 0 | 4 | 5 | 27 | 0 |

*Words*: the dictionary's words the grammar is learned from. *Sound laws*: changes
where a stem meets its ending (or farther, along a tier). *Rules*: the endings that
choose a paradigm. *Listed words*: words no rule gets right, listed with their paradigm.
*Irregular forms*: forms no paradigm gives, listed. *Listed by relations*: the words the
relations' rules get wrong, listed with the change they take.

## How well it does

A tenth of the words (by a hash of the word) was held out: the grammar scored here was
learned from the other nine tenths, and asked for every form the dictionary lists of
each held-out word, typed as the dictionary's headword. *Top-1*: its first answer is
right; *top-3*: the right answer is among its first three. *Calibration error*: how far
its stated probabilities are, on average, from how often it is right (0 is perfect).
The lemma's own form isn't scored where it is the letters typed (it is the input); where
it isn't (vowel length the headword doesn't write) it is scored, and also apart (*lemma
spelling*). After top-1, its 95% interval, resampling words. *Lemma in full*: top-1 typed
with every mark the dictionary's table writes, and said to be so (`spelled_in_full`).
*Principal parts given*: top-1 of the other forms, given the dictionary's principal
parts. *Features given*: top-1 from the headword, told the word's inherent features
(its gender, as `.gender(…)` tells it). *Among the candidates*: the right answer is
among its answers at all. *Every cell right*: the share of words with every form right.
*Log loss*: bits per form, −log₂ of the probability it gives the right answer (a form
it doesn't give costs 5 bits a letter). *1 / 2 / 3 forms given*: top-1 of the other
forms, typed in full, given that many of the word's forms chosen at random.
*Seen* and *unseen cells*: top-1 on the cells some training word of the word's
paradigm has a form in, and on the others, derived
from other cells (with the number of forms). *Floor*: the CoNLL–SIGMORPHON 2017
shared task's non-neural baseline (letters only), and on the unseen cells. *Against the
floor*: the grammar's letters right less the floor's, on the same forms, with its 95%
interval (resampling words). *Any listed form*: top-1 where any of the forms the
dictionary lists in the cell counts. *Homograph ceiling*: the most any grammar could get
right, words typed alike getting one answer per cell. *On a training word / new root*:
top-1 of the held-out words that are a training word with a prefix or a compound's front
before it, and of the rest. *Own / form-of / alt-of*: top-1 of the held-out entries that
are words of their own, another word's form (a participle, a comparative) and another
word's spelling, with their number of words. *Phrases*: top-1 on the forms the dictionary
writes as phrases (the Latin perfect passive, `amātus sum`). *Irregular lines*: the share of the
dictionary's words the grammar lists a form or a stem of. The held-out tenth was looked at while the trainer was being
developed (changes are judged on another tenth since PR #16).

| | held-out words | forms | top-1 | top-3 | among the candidates | every cell right | calibration error | log loss | lemma in full | principal parts given | 1 / 2 / 3 forms given | features given | seen cells | unseen cells | floor (unseen) | against the floor | any listed form | homograph ceiling | on a training word / new root | lemma spelling | own / form-of / alt-of | phrases | irregular lines |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| noun | 809 | 11483 | **71.90%** (68.9–74.9) | 88.69% | 91.39% | 62.42% | 0.056 | 5.145 | 88.36% | 95.17% | 93.63 / 94.89 / 95.27 | 72.81% | 71.90% (11483) |  | 51.28% (0.00%) | +24.36 (+20.33…+28.12) | 71.92% | 99.55% | 79.56% / 70.11% | 76.45% (586) | 71.60% (784) / 100.00% (3) / 78.46% (22) |  | 8.62% |
| adjective | 474 | 22171 | **77.29%** (74.0–80.6) | 92.13% | 94.01% | 63.50% | 0.046 | 3.934 | 90.48% | 97.88% | 94.39 / 94.68 / 96.92 |  | 77.31% (22161) | 40.00% (10) | 52.46% (40.00%) | +25.92 (+21.48…+30.22) | 77.49% | 99.17% | 80.30% / 76.70% | 83.06% (301) | 78.36% (349) / 74.75% (115) / 68.31% (10) |  | 2.91% |
| verb | 257 | 68968 | **65.45%** (60.7–70.1) | 77.87% | 83.01% | 18.68% | 0.057 | 11.207 | 74.89% | 80.52% | 78.26 / 78.21 / 78.29 |  | 65.60% (68730) | 19.75% (238) | 44.20% (15.97%) | +22.59 (+16.63…+28.50) | 65.82% | 99.90% | 73.49% / 60.72% | 78.12% (160) | 65.96% (244) / 45.83% (3) / 53.40% (10) | 56.62% (4832) | 52.39% |
| pronoun | 11 | 367 | **40.33%** (10.3–69.3) | 44.14% | 44.14% | 9.09% | 0.504 | 12.434 | 40.16% |  | 41.69 / 41.86 / 42.04 |  | 42.41% (349) | 0.00% (18) | 41.96% (11.11%) | +6.81 (-8.25…+30.49) | 40.33% | 99.46% | 0.00% / 40.33% | 100.00% (1) | 37.37% (8) / 88.89% (1) / 0.00% (2) |  | 53.45% |
| numeral | 2 | 30 | **100.00%** (100.0–100.0) | 100.00% | 100.00% | 100.00% | 0.030 | 0.045 | 100.00% |  | 100.00 / 100.00 / 100.00 |  | 100.00% (30) |  | 100.00% (0.00%) | +0.00 (+0.00…+0.00) | 100.00% | 100.00% | 0.00% / 100.00% |  | 100.00% (2) / – / – |  | 21.05% |

| | lines of grammar | bytes |
|---|---:|---:|
| noun | 9645 | 470626 |
| adjective | 6066 | 316370 |
| verb | 32318 | 2671287 |
| pronoun | 1477 | 64205 |
| numeral | 1025 | 40300 |

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
