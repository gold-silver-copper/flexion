# turkish

Turkish inflection for **any** Turkish word, attested or not: nouns, names,
adjectives, pronouns and numerals in six cases, two numbers and the
possessed forms; verbs in all 928 forms of the dictionary's tables (the
tenses with their copulas, the imperatives, participles and converbs, each
positive and negative, plain and abilitative), each with how sure the
grammar is and why. Vowel harmony is a learned sound law: an ending is
written once (`-lAr`, `-DA`) and the law chooses its vowel from the vowel
before it, as written, so it runs through a chain of suffixes
(`ev-ler-imiz-de`, `gel-me-yecek-ler`). A name keeps the apostrophe before
its endings (`Nepal'e`).

```rust
use turkish::*;

assert_eq!(noun("ev", Case::Locative, Number::Plural), "evlerde");
assert_eq!(noun("kitap", Case::Locative, Number::Plural), "kitaplarda");
let (yes, no, plain) = (Polarity::Positive, Polarity::Negative, Ability::Plain);
assert_eq!(finite("gelmek", yes, plain, Tense::Progressive, Copula::None, Subject::FirstSingular), "geliyorum");
assert_eq!(finite("gelmek", no, plain, Tense::Future, Copula::None, Subject::ThirdPlural), "gelmeyecekler");
assert_eq!(converb("gelmek", yes, plain, Converb::Doubled), "gele gele");
```

Left out: comparison with `daha` and `en`, which are words of their own; and
the predicative forms the noun tables end with (`evim` "I am a house",
`evdir`). Those endings are the copula, which attaches to any predicate (an
adjective, a case form: `evdeyim`; Göksel & Kerslake ch. 12), not forms of
the noun; the verb part writes the copula's forms.

## How it works

This crate is Turkish inflection as a grammar, learned by
[flexion](https://github.com/gold-silver-copper/flexion) from the English Wiktionary's
Turkish entries. Each part of speech is a module under `src/`, and has what its
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
- `compounds.rs`: whether a word written in parts inflects each part;
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
| noun | 10345 | 22 | 4 | 278 | 608 | 149 | 35 |
| pronoun | 13 | 2 | 0 | 1 | 11 | 18 | 0 |
| numeral | 7 | 2 | 0 | 2 | 4 | 5 | 0 |
| proper_noun | 1423 | 9 | 3 | 81 | 164 | 72 | 4 |
| adjective | 10402 | 22 | 4 | 281 | 608 | 99 | 93 |
| verb | 2368 | 35 | 8 | 175 | 313 | 352 | 0 |

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
| noun | 1065 | 71109 | **96.62%** (95.7–97.5) | 99.33% | 99.33% | 93.33% | 0.013 | 0.492 | 96.62% | 99.52% | 97.80 / 98.13 / 98.25 |  | 96.62% (71109) |  | 93.27% (0.00%) | +3.35 (+2.45…+4.33) | 96.67% | 99.88% | 98.92% / 96.39% |  | 96.29% (977) / 100.00% (72) / 99.51% (16) |  | 1.11% |
| pronoun | 1 | 11 | **reproduced 133 of 133**; a new word 14.29% (13 words) | 0.00% | 0.00% | 0.00% | 1.000 | 36.364 | 0.00% |  | 0.00 / 0.00 / 0.00 |  | 0.00% (11) |  | 90.91% (0.00%) | -90.91 (-90.91…-90.91) | 0.00% | 100.00% | 0.00% / 0.00% |  | 0.00% (1) / – / – |  | 61.54% |
| numeral | 2 | 22 | **reproduced 77 of 77**; a new word 37.66% (7 words) | 31.82% | 31.82% | 0.00% | 0.682 | 26.364 | 31.82% |  | 30.00 / 33.33 / 31.25 |  | 31.82% (22) |  | 0.00% (0.00%) | +31.82 (+0.00…+63.64) | 31.82% | 100.00% | 0.00% / 31.82% |  | 31.82% (2) / – / – |  | 42.86% |
| proper_noun | 145 | 3773 | **90.94%** (85.4–95.6) | 97.96% | 97.96% | 88.28% | 0.046 | 1.609 | 90.94% | 97.74% | 92.23 / 95.87 / 95.54 |  | 94.76% (3473) | 46.67% (300) | 93.24% (91.67%) | -2.31 (-6.43…+1.59) | 91.20% | 100.00% | 100.00% / 90.64% |  | 90.94% (145) / – / – | 100.00% (23) | 1.97% |
| adjective | 3 | 249 | **reproduced 4116 of 4116**; a new word 98.13% (60 words, by tenth) | 100.00% | 100.00% | 66.67% | 0.115 | 0.490 | 86.75% | 100.00% | 86.59 / 86.42 / 86.25 |  | 86.75% (249) |  | 81.93% (0.00%) | +4.82 (+0.00…+14.46) | 86.75% | 100.00% | 0.00% / 86.75% |  | 86.75% (3) / – / – |  | 1.11% |
| verb | 228 | 211212 | **98.90%** (97.8–99.8) | 99.94% | 99.94% | 84.21% | 0.006 | 0.136 | 98.90% | 99.92% | 99.49 / 99.49 / 99.49 |  | 98.90% (211212) |  | 98.68% (0.00%) | +0.22 (-1.06…+1.38) | 98.90% | 100.00% | 98.63% / 99.14% |  | 98.73% (198) / 100.00% (30) / – | 99.53% (44149) | 6.33% |

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
| noun | 2402 | 73% | 0% | 0% | 11% | 7% | 1% | 7% | 0% | 96.93% | 100.00% | 100.00% |
| pronoun | 11 | 0% | 0% | 0% | 0% | 100% | 0% | 0% | 0% | – | 100.00% | 100.00% |
| numeral | 15 | 0% | 0% | 0% | 0% | 100% | 0% | 0% | 0% | – | 100.00% | 100.00% |
| proper_noun | 342 | 75% | 0% | 0% | 10% | 13% | 3% | 0% | 0% | 91.56% | 100.00% | 100.00% |
| adjective | 33 | 100% | 0% | 0% | 0% | 0% | 0% | 0% | 0% | 88.51% | 100.00% | 100.00% |
| verb | 2330 | 94% | 0% | 0% | 0% | 6% | 0% | 0% | 0% | 98.95% | 100.00% | 100.00% |

*By block*, each group of cells' top-1 and held-out forms:

- noun: noun 97.40% (11637), possessive 96.47% (59472)
- proper_noun: proper_noun 94.73% (1517), proper_noun_possessive 88.39% (2256)
- adjective: adjective 90.91% (33), adjective_possessive 86.11% (216)
- verb: finite 98.85% (131184), past 99.23% (16416), conditional 99.23% (16416), optative 99.01% (16416), imperative 99.14% (7296), infinitive 99.12% (684), verbal_noun 99.23% (912), participle 99.01% (2736), relative 99.11% (10944), converb 97.51% (8208)

## The shipped grammar and the scored one

The grammar shipped is learned from every word; the one scored above from all but the
held-out tenth. Their sound laws (each with its tier) and irregular forms, side by side;
a law one has and the other doesn't is listed below the table.

| | sound laws, shipped / scored | the same laws | irregular forms, shipped / scored |
|---|---:|---:|---:|
| noun | 4 / 4 | 3 | 149 / 127 |
| pronoun | 0 / 0 | 0 | 18 / 23 |
| numeral | 0 / 0 | 0 | 5 / 6 |
| proper_noun | 3 / 3 | 3 | 72 / 56 |
| adjective | 4 / 4 | 3 | 99 / 127 |
| verb | 8 / 7 | 4 | 352 / 337 |

- noun: only the shipped grammar's: `[ol|ul]+i → +ü`. Only the scored grammar's: `ol+i → ol+ü`.
- adjective: only the shipped grammar's: `[ol|ul]+i → +ü`. Only the scored grammar's: `ol+i → ol+ü`.
- verb: only the shipped grammar's: `[ş|t|k|p|ç|s]+d → +t`, `[r|l|y|ğ|z|n|v|ı]+tı → +dı`, `[y|r|ğ|n|m|l|z|v|u]+tu → +du`, `[s|ç|ğ]+ır → +ar`. Only the scored grammar's: `[ş|t|k|p|s|ç]+d → +t`, `[y|r|l|z|n|ğ|v|m|u|d]+t → +d`, `[s|y]+ır → +ar`.

| | lines of grammar | bytes |
|---|---:|---:|
| noun | 2393 | 98703 |
| pronoun | 422 | 13854 |
| numeral | 392 | 12854 |
| proper_noun | 1260 | 64921 |
| adjective | 2415 | 106468 |
| verb | 9040 | 535939 |

## Data and license

The code is MIT or Apache-2.0. The grammar under `src/` is derived from Wiktionary via
[Kaikki](https://kaikki.org/) and [wiktextract](https://github.com/tatuylonen/wiktextract),
and is licensed under [CC BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/); see
`DATA-LICENSE.md`.

## Regenerating

```sh
cd flexion
cargo xtask extract turkish
cargo xtask build turkish
```
