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

| | words | paradigms | sound laws | rules | listed words | irregular forms |
|---|---:|---:|---:|---:|---:|---:|
| noun | 8133 | 92 | 6 | 316 | 1705 | 2844 |
| adjective | 4691 | 57 | 49 | 330 | 1044 | 4333 |
| verb | 2655 | 287 | 30 | 235 | 1537 | 33311 |
| pronoun | 58 | 19 | 0 | 14 | 36 | 439 |
| numeral | 19 | 7 | 0 | 4 | 5 | 27 |

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
before it, and of the rest. *Attested*: top-1 of the shipped grammar on the dictionary's
own words, typed in full. The held-out tenth was looked at while the trainer was being
developed (changes are judged on another tenth since PR #16).

| | held-out words | forms | top-1 | top-3 | among the candidates | every cell right | calibration error | log loss | lemma in full | principal parts given | 1 / 2 / 3 forms given | features given | seen cells | unseen cells | floor (unseen) | against the floor | any listed form | homograph ceiling | on a training word / new root | lemma spelling | attested |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| noun | 810 | 11498 | **71.16%** (68.2–74.1) | 88.61% | 91.36% | 61.73% | 0.055 | 5.185 | 88.25% | 95.20% | 93.49 / 94.75 / 95.24 | 72.06% | 71.16% (11498) |  | 51.16% (0.00%) | +23.86 (+19.97…+27.52) | 71.18% | 99.55% | 77.49% / 69.68% | 75.47% (587) | 99.73% |
| adjective | 476 | 22264 | **77.17%** (73.7–80.4) | 91.75% | 93.61% | 63.45% | 0.044 | 4.117 | 90.10% | 97.49% | 93.98 / 94.27 / 96.40 |  | 77.19% (22254) | 40.00% (10) | 52.29% (40.00%) | +25.98 (+21.47…+30.48) | 77.37% | 99.17% | 80.30% / 76.57% | 82.84% (303) | 99.93% |
| verb | 257 | 64136 | **65.28%** (60.5–70.1) | 79.14% | 83.82% | 18.68% | 0.071 | 10.164 | 76.10% | 80.89% | 78.99 / 79.06 / 79.21 |  | 65.34% (63986) | 36.67% (150) | 46.03% (25.33%) | +20.69 (+14.70…+26.51) | 65.52% | 99.90% | 73.19% / 60.67% | 77.50% (160) | 100.00% |
| pronoun | 11 | 367 | **40.33%** (10.3–69.3) | 44.14% | 44.14% | 9.09% | 0.550 | 12.397 | 40.16% |  | 41.69 / 41.86 / 42.04 |  | 42.41% (349) | 0.00% (18) | 41.96% (11.11%) | +6.81 (-8.25…+30.49) | 40.33% | 99.46% | 0.00% / 40.33% | 100.00% (1) | 100.00% |
| numeral | 2 | 30 | **100.00%** (100.0–100.0) | 100.00% | 100.00% | 100.00% | 0.030 | 0.044 | 100.00% |  | 100.00 / 100.00 / 100.00 |  | 100.00% (30) |  | 100.00% (0.00%) | +0.00 (+0.00…+0.00) | 100.00% | 100.00% | 0.00% / 100.00% |  | 100.00% |

| | lines of grammar | bytes |
|---|---:|---:|
| noun | 11814 | 541194 |
| adjective | 7511 | 400503 |
| verb | 37664 | 2980910 |
| pronoun | 1535 | 65857 |
| numeral | 1034 | 40647 |

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
