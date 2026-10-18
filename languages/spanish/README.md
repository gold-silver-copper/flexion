# spanish

Spanish inflection for **any** Spanish word, attested or not: noun plurals,
adjective agreement, and the whole conjugation (every simple tense, both
imperfect subjunctives, voseo, imperatives, participles), each with how sure
the grammar is and why.

```rust
use spanish::*;

assert_eq!(noun("canción", Number::Plural), "canciones");
assert_eq!(noun("joven", Number::Plural), "jóvenes");
assert_eq!(adjective("feliz", Gender::Feminine, Number::Plural), "felices");
assert_eq!(verb("contar", Tense::Present, Person::First, Number::Singular), "cuento");
assert_eq!(verb("hacer", Tense::Preterite, Person::First, Number::Singular), "hice");
assert_eq!(vos("tener", Mood::Indicative), "tenés");
assert_eq!(participle("escribir", Gender::Feminine, Number::Plural), "escritas");
```

Written accents are part of the word: `papá` and `papa` are two words.

## How it works

This crate is Spanish inflection as a grammar, learned by
[flexion](https://github.com/gold-silver-copper/flexion) from the English Wiktionary's and the Spanish Wiktionary's
Spanish entries. Each part of speech is a module under `src/`, and has what its
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
- `heads.rs`: the words a word ending in one inflects as, its front kept (compounds);
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
| noun | 60256 | 21 | 1 | 286 | 730 | 65 | 0 |
| adjective | 24530 | 15 | 17 | 284 | 568 | 39 | 1 |
| verb | 17957 | 159 | 23 | 1095 | 797 | 381 | 136 |
| determiner | 78 | 7 | 0 | 10 | 15 | 12 | 4 |
| adverb | 7 | 2 | 0 | 1 | 6 | 4 | 0 |

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
| noun | 6079 | 6079 | **98.67%** (98.4–98.9) | 99.80% | 99.87% | 98.67% | 0.003 | 0.111 | 98.67% |  |  | 98.67% | 98.67% (6079) |  | 98.67% (0.00%) | +0.00 (-0.20…+0.20) | 98.88% | 99.98% | 99.85% / 98.12% |  | 98.74% (5245) / 100.00% (583) / 94.02% (251) |  | 0.11% |
| adjective | 2399 | 7208 | **98.50%** (98.1–98.9) | 99.81% | 99.82% | 97.79% | 0.003 | 0.180 | 98.50% | 99.67% | 99.67 / 99.88 / 86.67 |  | 98.50% (7208) |  | 98.60% (0.00%) | -0.10 (-0.35…+0.14) | 98.61% | 100.00% | 99.84% / 98.04% |  | 98.49% (2284) / 100.00% (68) / 97.12% (47) |  | 0.21% |
| verb | 929 | 66839 | **98.99%** (98.6–99.3) | 99.40% | 99.42% | 95.26% | 0.004 | 0.329 | 98.99% | 99.74% | 99.19 / 99.32 / 99.45 |  | 98.99% (66839) |  | 99.28% (0.00%) | -0.30 (-0.59…-0.01) | 99.01% | 99.98% | 99.32% / 98.77% |  | 99.04% (879) / 100.00% (4) / 97.86% (46) |  | 1.68% |
| determiner | 9 | 22 | **reproduced 173 of 173**; a new word 80.35% (78 words) | 95.45% | 100.00% | 88.89% | 0.094 | 0.621 | 90.91% |  | 100.00 / 100.00 / 0.00 |  | 90.91% (22) |  | 100.00% (0.00%) | -9.09 (-30.00…+0.00) | 90.91% | 100.00% | 100.00% / 90.48% | 0.00% (1) | 77.78% (4) / 100.00% (2) / 100.00% (3) |  | 11.54% |
| adverb | 0 | 0 | **reproduced 8 of 8**; a new word 12.50% (7 words) | – | – | – | – | – | – | – | – | – | – | – | – | – | – | – | – | – | – | – | 42.86% |

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
| noun | 81 | 74% | 0% | 0% | 5% | 4% | 16% | 1% | 0% | 98.67% | 100.00% | 100.00% |
| adjective | 108 | 81% | 0% | 0% | 5% | 7% | 7% | 0% | 0% | 98.57% | 100.00% | 100.00% |
| verb | 678 | 43% | 0% | 0% | 33% | 23% | 2% | 0% | 0% | 98.83% | 100.00% | 100.00% |
| determiner | 2 | 100% | 0% | 0% | 0% | 0% | 0% | 0% | 0% | – | 100.00% | 100.00% |
| adverb | – | – | – | – | – | – | – | – | – | – | 100.00% | 100.00% |

*By block*, each group of cells' top-1 and held-out forms:

- adjective: adjective 98.57% (7193); under 30 forms (not in the mean): superlative 58.33% (12), before_noun 100.00% (3)
- verb: verb 99.18% (50130), vos 99.78% (2784), imperative 97.44% (4640), negative_imperative 97.44% (4640), gerund 99.68% (929), participle 99.46% (3716)

## The shipped grammar and the scored one

The grammar shipped is learned from every word; the one scored above from all but the
held-out tenth. Their sound laws (each with its tier) and irregular forms, side by side;
a law one has and the other doesn't is listed below the table.

| | sound laws, shipped / scored | the same laws | irregular forms, shipped / scored |
|---|---:|---:|---:|
| noun | 1 / 0 | 0 | 65 / 60 |
| adjective | 17 / 19 | 13 | 39 / 38 |
| verb | 23 / 23 | 22 | 381 / 359 |
| determiner | 0 / 0 | 0 | 12 / 12 |
| adverb | 0 / 0 | 0 | 4 / 4 |

- noun: only the shipped grammar's: `E → i after a; e after i, none (the nearest of a, i before it, as written)`.
- adjective: only the shipped grammar's: `és+e → +ese`, `[s|h]+s → +`, `i+e → i+`, `ó+e → ó+`. Only the scored grammar's: `[s|o]+s → +`, `[i|o]+e → +`, `as+e → a+`, `os+e → o+`, `d+s → d+ after a vowel earlier in the stem`, `h+s → h+`.
- verb: only the shipped grammar's: `o → ue after [c|d|f|l|m|p|r|s|t|v]; o → üe after g in alternating stems (ˇ), before +.a$, +.a[n|s]$, +.o, b+[a|e|i|o|u|í|ü]$, b+[a|e|i|o|u|í|ü][n|s]$, b.+[a|e|i|o|u|í|ü]$, b.+[a|e|i|o|u|í|ü][n|s]$, [d|ñ]+[a|e|i|o|u|í|ü]$, l+a$, l+a[n|s]$, l+e$, l+e[n|s], l+o, l[d|l]+[a|e|i|o|u|í|ü]$, l[d|l]+[a|e|i|o|u|í|ü][n|s]$, lt+[a|e|i|o|u|í|ü]$, [n|s]+[a|e|i|o|u|í|ü]$, [n|s]+[a|e|i|o|u|í|ü][n|s]$, [n|s]t+[a|e|i|o|u|í|ü]$, [n|s]t+[a|e|i|o|u|í|ü][n|s]$, r+[a|e|i|o|u|í|ü]$, r+[^a|e|i|o|u|í|ü]a$, r+[^a|e|i|o|u|í|ü]a[n|s]$, r+[^a|e|i|o|u|í|ü]o, rd+a$, rd+a[n|s]$, rd+e$, rd+e[n|s], rd+o, rm+[a|e], rn+[a|e|i|o|u|í|ü]$, rn+[a|e|i|o|u|í|ü][n|s]$, rt+[a|e|i|o|u|í|ü]$, v+a$, v+a.$, v+e$`. Only the scored grammar's: `o → ue after [c|d|f|l|m|p|r|s|t|v] in alternating stems (ˇ), before +.a$, +.a[n|s]$, +.o, b+[a|e|i|o|u|í|ü]$, b+[a|e|i|o|u|í|ü][n|s]$, b.+[a|e|i|o|u|í|ü]$, b.+[a|e|i|o|u|í|ü][n|s]$, [d|ñ]+[a|e|i|o|u|í|ü]$, l+a$, l+a[n|s]$, l+e$, l+e[n|s], l+o, ld+[a|e|i|o|u|í|ü]$, ld+[a|e|i|o|u|í|ü][n|s]$, l[l|t]+[a|e|i|o|u|í|ü]$, n+[a|e|i|o|u|í|ü]$, n+[a|e|i|o|u|í|ü][n|s]$, nt+[a|e|i|o|u|í|ü]$, r+[a|e|i|o|u|í|ü]$, r+[^a|e|i|o|u|í|ü]a$, r+[^a|e|i|o|u|í|ü]a[n|s]$, rd+a$, rd+a[n|s]$, rd+e$, rd+e[n|s], rd+o, rm+[a|e], rn+[a|e|i|o|u|í|ü]$, rn+[a|e|i|o|u|í|ü][n|s]$, rt+[a|e|i|o|u|í|ü]$, s+[a|e|i|o|u|í|ü]$, st+[a|e|i|o|u|í|ü]$, st+[a|e|i|o|u|í|ü][n|s]$, v+a$, v+a.$, v+e$`.

| | lines of grammar | bytes |
|---|---:|---:|
| noun | 1023 | 42310 |
| adjective | 1120 | 47147 |
| verb | 7069 | 348053 |
| determiner | 417 | 13671 |
| adverb | 327 | 10913 |

## Data and license

The code is MIT or Apache-2.0. The grammar under `src/` is derived from Wiktionary via
[Kaikki](https://kaikki.org/) and [wiktextract](https://github.com/tatuylonen/wiktextract),
and is licensed under [CC BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/); see
`DATA-LICENSE.md`.

## Regenerating

```sh
cd flexion
cargo xtask extract spanish
cargo xtask build spanish
```
