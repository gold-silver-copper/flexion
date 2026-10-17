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
| noun | 8124 | 86 | 4 | 316 | 1710 | 1999 | 60 |
| proper_noun | 3793 | 68 | 0 | 192 | 728 | 758 | 26 |
| adjective | 4674 | 46 | 3 | 330 | 1027 | 3599 | 84 |
| verb | 2655 | 286 | 26 | 234 | 1531 | 27750 | 425 |
| pronoun | 58 | 19 | 0 | 14 | 35 | 405 | 13 |
| numeral | 19 | 7 | 0 | 4 | 5 | 20 | 0 |

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
| noun | 809 | 11483 | **74.79%** (71.9–77.5) | 91.47% | 94.21% | 62.79% | 0.048 | 3.768 | 89.15% | 96.09% | 94.55 / 95.84 / 96.10 | 75.82% | 74.79% (11483) |  | 51.28% (0.00%) | +25.16 (+21.24…+28.91) | 74.81% | 99.55% | 81.35% / 73.25% | 78.50% (586) | 74.59% (784) / 66.67% (3) / 82.96% (22) |  | 8.64% |
| proper_noun | 383 | 2014 | **59.73%** (54.6–64.9) | 80.54% | 83.57% | 54.31% | 0.080 | 8.602 | 86.50% | 91.00% | 94.11 / 95.82 / 96.30 | 62.07% | 59.73% (2014) |  | 47.42% (0.00%) | +13.90 (+7.80…+20.37) | 59.88% | 99.40% | 56.73% / 60.01% | 63.80% (221) | 60.11% (375) / – / 43.48% (8) |  | 5.09% |
| adjective | 474 | 22171 | **78.02%** (74.8–81.2) | 93.37% | 95.03% | 63.71% | 0.040 | 3.396 | 90.76% | 98.02% | 94.65 / 94.93 / 97.06 |  | 78.03% (22161) | 40.00% (10) | 52.46% (40.00%) | +26.37 (+21.84…+30.57) | 78.21% | 99.17% | 82.37% / 77.17% | 84.05% (301) | 79.52% (349) / 74.20% (115) / 68.31% (10) |  | 2.91% |
| verb | 257 | 68968 | **68.04%** (63.4–72.5) | 79.85% | 83.50% | 23.35% | 0.084 | 10.861 | 75.67% | 81.10% | 78.59 / 78.54 / 78.77 |  | 68.19% (68730) | 23.11% (238) | 44.20% (15.97%) | +25.05 (+19.23…+30.66) | 68.50% | 99.90% | 77.94% / 62.22% | 82.50% (160) | 68.66% (244) / 45.83% (3) / 53.40% (10) | 57.62% (4832) | 46.74% |
| pronoun | 11 | 367 | **40.33%** (10.3–69.3) | 44.14% | 44.14% | 9.09% | 0.602 | 12.382 | 40.16% |  | 41.69 / 41.86 / 42.04 |  | 42.41% (349) | 0.00% (18) | 41.96% (11.11%) | +6.81 (-8.25…+30.49) | 40.33% | 99.46% | 0.00% / 40.33% | 100.00% (1) | 37.37% (8) / 88.89% (1) / 0.00% (2) |  | 51.72% |
| numeral | 2 | 30 | **100.00%** (100.0–100.0) | 100.00% | 100.00% | 100.00% | 0.061 | 0.091 | 100.00% |  | 100.00 / 100.00 / 100.00 |  | 100.00% (30) |  | 100.00% (0.00%) | +0.00 (+0.00…+0.00) | 100.00% | 100.00% | 0.00% / 100.00% |  | 100.00% (2) / – / – |  | 21.05% |

## Where the errors are

The held-out forms the grammar gets wrong from the headword, by what went wrong, as a share
of them: *class*, the right form is among its answers but another paradigm's comes first;
*length* and *accent*, the right letters with a vowel's length or the accent wrong; *stem*
and *ending*, wrong letters inside the stem or after it; *variant*, another of the forms
the dictionary lists comes first; *homograph*, another dictionary word spelled the same has
that form; *gap*, no form at all. *By block*: the mean of each group of cells' top-1, so a
group of many cells doesn't outweigh the rest. *Reproduced*: the shipped grammar on the
dictionary's own words (at most 3,000), from the headword; *complete*: of those words'
cells, the dictionary's or not, the share with a form.

| | wrong forms | class | length | accent | stem | ending | variant | homograph | gap | by block | reproduced | complete |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| noun | 2895 | 27% | 59% | 6% | 2% | 6% | 0% | 1% | 0% | 74.79% | 99.67% | 100.00% |
| proper_noun | 811 | 15% | 74% | 4% | 2% | 5% | 0% | 0% | 0% | 59.73% | 99.89% | 100.00% |
| adjective | 4874 | 28% | 60% | 4% | 4% | 3% | 1% | 1% | 0% | 80.60% | 99.85% | 100.00% |
| verb | 22043 | 18% | 42% | 4% | 19% | 17% | 1% | 0% | 0% | 66.21% | 99.94% | 100.00% |
| pronoun | 219 | 6% | 0% | 14% | 1% | 78% | 0% | 0% | 0% | 40.33% | 100.00% | 100.00% |
| numeral | 0 | 0% | 0% | 0% | 0% | 0% | 0% | 0% | 0% | 100.00% | 100.00% | 100.00% |

| | lines of grammar | bytes |
|---|---:|---:|
| noun | 6154 | 340503 |
| proper_noun | 3979 | 206010 |
| adjective | 4368 | 253089 |
| verb | 29239 | 2418002 |
| pronoun | 1487 | 63454 |
| numeral | 1012 | 39327 |

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
