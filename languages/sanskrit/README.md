# sanskrit

Sanskrit inflection for **any** word, attested or not, in Devanagari: nouns and
adjectives in eight cases and three numbers, and verbs (present system with its
four moods, imperfect, future, conditional, perfect, aorist, infinitive,
gerund, participles), each with how sure the grammar is and why.

```rust
use sanskrit::*;

assert_eq!(noun("देव", Case::Instrumental, Number::Singular), "देवेन");
assert_eq!(noun("देव", Case::Genitive, Number::Plural), "देवानाम्");
assert_eq!(adjective("नव", Case::Nominative, Gender::Feminine, Number::Singular), "नवा");

// A noun the dictionary lacks: by its ending masculine, unless you say otherwise.
assert_eq!(noun("सुकुल", Case::Nominative, Number::Singular), "सुकुलः");
let neuter = Noun::new("सुकुल").gender(Gender::Neuter);
assert_eq!(neuter.form((Case::Nominative, Number::Singular)), "सुकुलम्");
```

Nouns and adjectives are cited by their stem, as Sanskrit dictionaries do;
verbs by the third person singular present. The forms are Classical (Vedic
variants and accents are left out), with the sandhi inside the word.

## How it works

This crate is Sanskrit inflection as a grammar, learned by
[flexion](https://github.com/gold-silver-copper/flexion) from the English Wiktionary's
Sanskrit entries. Each part of speech is a module under `src/`, and has what its
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
- `prefixes.rs`: prefixes a word inflects behind, separable or not;
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
| noun | 5023 | 55 | 4 | 283 | 889 | 199 | 3 |
| proper_noun | 610 | 25 | 0 | 64 | 89 | 72 | 9 |
| adjective | 1964 | 41 | 2 | 160 | 361 | 435 | 43 |
| numeral | 65 | 4 | 0 | 8 | 6 | 27 | 0 |
| verb | 1003 | 64 | 5 | 159 | 346 | 1273 | 207 |
| pronoun | 15 | 3 | 0 | 3 | 8 | 26 | 0 |

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
| noun | 576 | 13801 | **92.10%** (90.8–93.4) | 98.74% | 98.74% | 73.96% | 0.015 | 0.765 | 92.10% |  | 94.62 / 94.88 / 96.04 | 97.30% | 92.10% (13801) |  | 91.70% (0.00%) | +0.40 (-0.73…+1.48) | 92.65% | 97.83% | 94.30% / 92.03% |  | 92.14% (563) / 100.00% (2) / 88.71% (11) |  | 1.57% |
| proper_noun | 49 | 1176 | **95.15%** (90.0–99.1) | 96.77% | 96.77% | 83.67% | 0.030 | 1.705 | 95.15% |  | 95.12 / 95.64 / 96.11 | 96.34% | 95.15% (1176) |  | 86.48% (0.00%) | +8.67 (+3.40…+14.54) | 95.15% | 100.00% | 0.00% / 95.15% |  | 95.05% (48) / – / 100.00% (1) |  | 2.13% |
| adjective | 190 | 13456 | **91.91%** (89.6–94.0) | 98.66% | 98.66% | 72.11% | 0.020 | 0.931 | 91.91% |  | 93.71 / 94.00 / 97.29 |  | 91.97% (13447) | 11.11% (9) | 90.11% (11.11%) | +1.81 (+0.17…+3.56) | 91.94% | 100.00% | 83.33% / 92.30% |  | 91.60% (183) / 100.00% (6) / 100.00% (1) |  | 3.82% |
| numeral | 9 | 264 | **63.64%** (29.8–99.1) | 63.64% | 63.64% | 66.67% | 0.349 | 19.623 | 63.64% |  | 63.14 / 62.60 / 62.45 |  | 63.64% (264) |  | 92.42% (0.00%) | -28.79 (-55.13…-0.93) | 63.64% | 100.00% | 100.00% / 50.00% |  | 63.64% (9) / – / – |  | 7.69% |
| verb | 88 | 7177 | **89.86%** (85.3–94.0) | 91.49% | 91.61% | 64.77% | 0.045 | 4.490 | 89.86% | 90.58% | 91.24 / 91.54 / 91.68 | 89.86% | 90.26% (6896) | 80.07% (281) | 89.93% (80.78%) | -0.07 (-2.32…+2.05) | 90.37% | 100.00% | 95.65% / 89.31% | 100.00% (2) | 91.49% (71) / 55.52% (17) / – |  | 24.23% |
| pronoun | 3 | 207 | **69.57%** (0.0–100.0) | 69.57% | 69.57% | 66.67% | 0.298 | 10.836 | 69.57% |  | 69.61 / 69.65 / 75.25 |  | 69.57% (207) |  | 70.53% (0.00%) | -0.97 (-3.17…+0.00) | 69.57% | 100.00% | 0.00% / 69.57% |  | 69.57% (3) / – / – |  | 46.67% |

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
| noun | 1090 | 25% | 27% | 0% | 10% | 4% | 7% | 27% | 0% | 92.10% | 100.00% | 100.00% |
| proper_noun | 57 | 19% | 16% | 0% | 26% | 39% | 0% | 0% | 0% | 95.15% | 100.00% | 100.00% |
| adjective | 1088 | 20% | 67% | 0% | 6% | 7% | 0% | 0% | 0% | 39.00% | 100.00% | 100.00% |
| numeral | 96 | 0% | 79% | 0% | 0% | 21% | 0% | 0% | 0% | 63.64% | 100.00% | 100.00% |
| verb | 728 | 9% | 28% | 0% | 24% | 35% | 5% | 0% | 0% | 82.00% | 100.00% | 100.00% |
| pronoun | 63 | 0% | 0% | 0% | 0% | 100% | 0% | 0% | 0% | 69.57% | 100.00% | 100.00% |

| | lines of grammar | bytes |
|---|---:|---:|
| noun | 2191 | 109813 |
| proper_noun | 1133 | 49144 |
| adjective | 3109 | 151131 |
| numeral | 986 | 40184 |
| verb | 5326 | 292308 |
| pronoun | 884 | 35953 |

## Data and license

The code is MIT or Apache-2.0. The grammar under `src/` is derived from Wiktionary via
[Kaikki](https://kaikki.org/) and [wiktextract](https://github.com/tatuylonen/wiktextract),
and is licensed under [CC BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/); see
`DATA-LICENSE.md`.

## Regenerating

```sh
cd flexion
cargo xtask extract sanskrit
cargo xtask build sanskrit
```
