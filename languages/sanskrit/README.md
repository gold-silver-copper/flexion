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
| adjective | 1964 | 41 | 2 | 134 | 348 | 413 | 41 |
| numeral | 65 | 4 | 0 | 8 | 6 | 44 | 0 |
| verb | 1003 | 64 | 5 | 157 | 287 | 1254 | 199 |
| pronoun | 15 | 3 | 0 | 3 | 8 | 28 | 0 |

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
developed (changes are judged on another tenth since PR #16). A closed class (pronouns,
numerals, determiners, adpositions) and a part of speech with fewer than 30 held-out words
is judged first on its own words: its top-1 is *reproduced*, the shipped grammar's right
forms of the forms of its own words, then *a new word*, top-1 with each word left out of
training in turn (a tenth at a time where another part of speech's words teach it). A part
with no held-out words has no other score (–).

| | held-out words | forms | top-1 | top-3 | among the candidates | every cell right | calibration error | log loss | lemma in full | principal parts given | 1 / 2 / 3 forms given | features given | seen cells | unseen cells | floor (unseen) | against the floor | any listed form | homograph ceiling | on a training word / new root | lemma spelling | own / form-of / alt-of | phrases | irregular lines |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| noun | 576 | 13801 | **92.10%** (90.8–93.4) | 98.74% | 98.74% | 73.96% | 0.015 | 0.765 | 92.10% |  | 94.62 / 94.88 / 96.04 | 97.30% | 92.10% (13801) |  | 91.70% (0.00%) | +0.40 (-0.73…+1.48) | 92.65% | 97.83% | 94.30% / 92.03% |  | 92.14% (563) / 100.00% (2) / 88.71% (11) |  | 1.57% |
| proper_noun | 49 | 1176 | **95.15%** (90.0–99.1) | 96.77% | 96.77% | 83.67% | 0.030 | 1.705 | 95.15% |  | 95.12 / 95.64 / 96.11 | 96.34% | 95.15% (1176) |  | 86.48% (0.00%) | +8.67 (+3.40…+14.54) | 95.15% | 100.00% | 0.00% / 95.15% |  | 95.05% (48) / – / 100.00% (1) |  | 2.13% |
| adjective | 190 | 13456 | **91.86%** (89.5–94.0) | 98.60% | 98.60% | 72.11% | 0.020 | 0.976 | 91.86% |  | 93.47 / 93.76 / 97.04 |  | 91.92% (13447) | 11.11% (9) | 90.11% (11.11%) | +1.75 (+0.12…+3.52) | 91.89% | 100.00% | 83.33% / 92.24% |  | 91.55% (183) / 100.00% (6) / 100.00% (1) |  | 3.72% |
| numeral | 9 | 264 | **reproduced 1656 of 1656**; a new word 90.52% (65 words) | 63.64% | 63.64% | 66.67% | 0.349 | 19.623 | 63.64% |  | 63.14 / 62.60 / 62.45 |  | 63.64% (264) |  | 92.42% (0.00%) | -28.79 (-55.13…-0.93) | 63.64% | 100.00% | 100.00% / 50.00% |  | 63.64% (9) / – / – |  | 7.69% |
| verb | 88 | 7177 | **89.61%** (85.1–93.8) | 91.24% | 91.36% | 64.77% | 0.041 | 4.651 | 89.61% | 90.33% | 90.99 / 91.29 / 91.42 | 89.61% | 90.23% (7060) | 52.14% (117) | 89.93% (38.46%) | -0.32 (-2.79…+2.02) | 90.12% | 100.00% | 95.65% / 89.03% | 100.00% (2) | 91.23% (71) / 55.52% (17) / – |  | 23.63% |
| pronoun | 3 | 207 | **reproduced 987 of 987**; a new word 68.90% (15 words) | 69.57% | 69.57% | 66.67% | 0.298 | 10.836 | 69.57% |  | 69.61 / 69.65 / 75.25 |  | 69.57% (207) |  | 70.53% (0.00%) | -0.97 (-3.17…+0.00) | 69.57% | 100.00% | 0.00% / 69.57% |  | 69.57% (3) / – / – |  | 46.67% |

## Where the errors are

The held-out forms the grammar gets wrong from the headword, by what went wrong, as a share
of them: *class*, the right form is among its answers but another paradigm's comes first;
*marks*, the right letters but for a mark the spelling folds (a vowel's length, a
breathing; not a letter written with a combining mark, as a Devanagari vowel sign);
*accent*, the right letters with the accent wrong; *stem* and *ending*, wrong letters
inside the stem or after it; *variant*, another of the forms the dictionary lists comes
first; *homograph*, another dictionary word spelled the same has that form; *gap*, no form
at all. *By block*: the mean of each group of cells' top-1, so a group of many cells
doesn't outweigh the rest; a group with fewer than 30 held-out forms is left out of it and
named below. *Reproduced*: the shipped grammar on the dictionary's own words (at most
3,000), from the headword; *complete*: of those words' cells, the dictionary's or not, the
share with a form.

| | wrong forms | class | marks | accent | stem | ending | variant | homograph | gap | by block | reproduced | complete |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| noun | 1090 | 50% | 0% | 0% | 10% | 6% | 7% | 27% | 0% | 92.10% | 100.00% | 100.00% |
| proper_noun | 57 | 33% | 0% | 0% | 26% | 40% | 0% | 0% | 0% | 95.15% | 100.00% | 100.00% |
| adjective | 1095 | 83% | 0% | 0% | 6% | 10% | 0% | 0% | 0% | 91.96% | 100.00% | 100.00% |
| numeral | 96 | 0% | 0% | 0% | 0% | 100% | 0% | 0% | 0% | 63.64% | 100.00% | 100.00% |
| verb | 746 | 17% | 0% | 0% | 22% | 56% | 5% | 0% | 0% | 78.66% | 100.00% | 100.00% |
| pronoun | 63 | 0% | 0% | 0% | 0% | 100% | 0% | 0% | 0% | 69.57% | 100.00% | 100.00% |

*By block*, each group of cells' top-1 and held-out forms:

- adjective: adjective 91.96% (13440); under 30 forms (not in the mean): comparative 12.50% (8), superlative 12.50% (8)
- verb: present 94.22% (4515), imperfect 87.93% (1143), future 92.31% (351), conditional 85.00% (360), perfect 66.33% (401), aorist 37.65% (162), participle 87.20% (211); under 30 forms (not in the mean): infinitive 94.12% (17), gerund 88.24% (17)

## The shipped grammar and the scored one

The grammar shipped is learned from every word; the one scored above from all but the
held-out tenth. Their sound laws (each with its tier) and irregular forms, side by side;
a law one has and the other doesn't is listed below the table.

| | sound laws, shipped / scored | the same laws | irregular forms, shipped / scored |
|---|---:|---:|---:|
| noun | 4 / 1 | 1 | 199 / 283 |
| proper_noun | 0 / 0 | 0 | 72 / 51 |
| adjective | 2 / 4 | 0 | 413 / 397 |
| numeral | 0 / 0 | 0 | 44 / 61 |
| verb | 5 / 4 | 1 | 1254 / 1135 |
| pronoun | 0 / 0 | 0 | 28 / 12 |

- noun: only the shipped grammar's: `्म+् → ्म+ after a vowel earlier in the stem`, `्म+्न → ्म+न`, `्म+्ण → ्म+ण`.
- adjective: only the shipped grammar's: `A → न after ं, च, ज, ट, ठ, ड, ढ, ण, त, थ, द, ध, न, ल, ळ, श, स, none; ण after र, ष, ृ, ॄ (the nearest of ं, च, ज, ट, ठ, ड, ढ, ण, त, थ, द, ध, न, र, ल, ळ, श, ष, स, ृ, ॄ before it, as written)`, `D → ि after क, ख, ण, न, म, य, र, ल, श, ा, ो, none (the nearest of क, ख, ण, न, म, य, र, ल, श, ा, ो before it, as written)`. Only the scored grammar's: `A → श after ं; न after ख, ज, ट, ठ, ण, त, थ, द, ध, न, ल, श, स, none; ण after र, ष, ृ, ॄ (the nearest of ं, ख, ज, ट, ठ, ण, त, थ, द, ध, न, र, ल, श, ष, स, ृ, ॄ before it, as written)`, `B → न after ज, ट, ठ, ण, त, थ, द, ध, न, ल, श, स, ी, none; ण after र, ष (the nearest of ज, ट, ठ, ण, त, थ, द, ध, न, र, ल, श, ष, स, ी before it, as written)`, `D → ि after क, ख, ण, म, य, र, ल, श, ा, ो, none (the nearest of क, ख, ण, म, य, र, ल, श, ा, ो before it, as written)`, `ण → न after ढ, ळ`.
- verb: only the shipped grammar's: `A → न after क, ग, च, ज, ट, ठ, ण, त, द, ध, श, स, none; ण after र, ष, ृ (the nearest of क, ग, च, ज, ट, ठ, ण, त, द, ध, र, श, ष, स, ृ before it, as written)`, `B → ष after क, र, ि, ी, ु, े, ो; स after त, म, व, स, ा; ध after द; ज after ष, ृ, none (the nearest of क, त, द, म, र, व, ष, स, ा, ि, ी, ु, ृ, े, ो before it, as written)`, `अ‹अ → आ, इ → ऐ, उ → औ, उद → उदा, उप → उपा, ऊ → औ, ऋ → आर्, ऋच → अर्च, ओ → औ, सम → समा`, `[ा|े|ो|ू|ए]+ा → +`. Only the scored grammar's: `A → न after क, ग, च, ज, ट, ठ, ण, त, द, ध, श, none; ण after र, ष, ृ (the nearest of क, ग, च, ज, ट, ठ, ण, त, द, ध, र, श, ष, ृ before it, as written)`, `अ‹अ → आ, इ → ऐ, उ → औ, उद → उदा, उप → उपा, ऊ → औ, ऋ → आर्, ऋच → अर्च, ओ → औ, वी → व्य, सम → समा`, `[ा|े|्|ो|ू|ए]+ा → +`.

| | lines of grammar | bytes |
|---|---:|---:|
| noun | 2143 | 107705 |
| proper_noun | 1085 | 47036 |
| adjective | 2923 | 138676 |
| numeral | 889 | 35224 |
| verb | 5085 | 274556 |
| pronoun | 787 | 30287 |

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
