# italian

Italian inflection for **any** Italian word, attested or not: noun plurals,
adjective and participle agreement, and the whole simple conjugation
(indicative, subjunctive, imperative, gerund, participles), each with how sure
the grammar is and why.

```rust
use italian::*;

assert_eq!(noun("uomo", Number::Plural), "uomini");
assert_eq!(noun("città", Number::Plural), "città");
assert_eq!(adjective("bello", Gender::Feminine, Number::Plural), "belle");
assert_eq!(verb("finire", Tense::Present, Person::First, Number::Singular), "finisco");
assert_eq!(verb("prendere", Tense::PastHistoric, Person::First, Number::Singular), "presi");
assert_eq!(agreeing_participle("scrivere", Gender::Feminine, Number::Singular), "scritta");
assert_eq!(before_noun("bello", Gender::Masculine, Number::Plural, "occhi").as_deref(), Some("begli"));
```

Forms are written in standard spelling, with an accent only on a final
stressed vowel (`città`, `prenderò`) and on the monosyllables that take one
(`dà`, `può`).

Left out: pronouns. Their dictionary entries give no forms but the
possessive (`io`, `mio`), which the determiners already decline.

## How it works

This crate is Italian inflection as a grammar, learned by
[flexion](https://github.com/gold-silver-copper/flexion) from the English Wiktionary's
Italian entries. Each part of speech is a module under `src/`, and has what its
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
| noun | 61768 | 26 | 0 | 1021 | 2234 | 88 | 0 |
| adjective | 26286 | 12 | 1 | 322 | 483 | 48 | 4 |
| verb | 12955 | 116 | 18 | 462 | 393 | 737 | 56 |
| participle | 8087 | 2 | 0 | 6 | 2 | 3 | 0 |
| determiner | 45 | 4 | 0 | 5 | 9 | 8 | 0 |
| adverb | 8 | 1 | 0 | 1 | 6 | 8 | 0 |

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
| noun | 6392 | 6392 | **95.82%** (95.3–96.3) | 99.44% | 99.67% | 95.82% | 0.003 | 0.387 | 95.82% |  |  | 96.39% | 95.82% (6392) |  | 95.09% (0.00%) | +0.74 (+0.31…+1.16) | 95.96% | 99.77% | 95.91% / 95.76% |  | 95.85% (5862) / 98.57% (210) / 93.44% (320) |  | 0.19% |
| adjective | 2631 | 7900 | **98.46%** (98.0–98.8) | 99.86% | 99.86% | 97.61% | 0.010 | 0.137 | 98.46% | 99.78% | 99.60 / 99.66 / 100.00 |  | 98.45% (7895) | 100.00% (5) | 98.46% (100.00%) | +0.00 (-0.28…+0.28) | 98.49% | 100.00% | 99.36% / 97.96% |  | 98.48% (2514) / 100.00% (42) / 96.86% (75) |  | 0.19% |
| verb | 1286 | 70300 | **99.05%** (98.7–99.4) | 99.57% | 99.57% | 96.35% | 0.004 | 0.233 | 99.05% | 99.53% | 99.30 / 99.45 / 99.48 | 99.05% | 99.05% (70300) |  | 99.22% (0.00%) | -0.17 (-0.48…+0.14) | 99.12% | 99.96% | 98.93% / 99.12% |  | 99.34% (1188) / 100.00% (9) / 95.12% (89) |  | 1.77% |
| participle | 821 | 2463 | **100.00%** (100.0–100.0) | 100.00% | 100.00% | 100.00% | 0.000 | 0.000 | 100.00% |  | 100.00 / 100.00 / 0.00 |  | 100.00% (2463) |  | 100.00% (0.00%) | +0.00 (+0.00…+0.00) | 100.00% | 100.00% | 100.00% / 100.00% |  | 100.00% (11) / 100.00% (810) / – |  | 0.02% |
| determiner | 8 | 22 | **reproduced 117 of 117**; a new word 88.03% (45 words) | 86.36% | 86.36% | 75.00% | 0.182 | 2.955 | 81.82% |  | 78.57 / 71.43 / 0.00 |  | 90.00% (20) | 0.00% (2) | 86.36% (0.00%) | -4.55 (-13.64…+0.00) | 81.82% | 100.00% | 100.00% / 78.95% |  | 81.82% (8) / – / – |  | 13.33% |
| adverb | 1 | 1 | **reproduced 10 of 10**; a new word 0.00% (8 words) | 0.00% | 0.00% | 0.00% | 1.000 | 55.000 | 0.00% | 0.00% |  |  | 0.00% (1) |  | 0.00% (0.00%) | +0.00 (+0.00…+0.00) | 0.00% | 100.00% | 0.00% / 0.00% |  | 0.00% (1) / – / – |  | 75.00% |

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
| noun | 267 | 85% | 0% | 0% | 6% | 1% | 3% | 5% | 0% | 95.82% | 100.00% | 100.00% |
| adjective | 122 | 90% | 0% | 0% | 3% | 4% | 2% | 0% | 0% | 96.56% | 100.00% | 100.00% |
| verb | 665 | 48% | 0% | 0% | 22% | 19% | 7% | 4% | 0% | 99.04% | 100.00% | 100.00% |
| participle | 0 | – | – | – | – | – | – | – | – | 100.00% | 100.00% | 100.00% |
| determiner | 4 | 25% | 0% | 0% | 50% | 25% | 0% | 0% | 0% | – | 100.00% | 100.00% |
| adverb | 1 | 0% | 0% | 0% | 0% | 100% | 0% | 0% | 0% | – | 100.00% | 100.00% |

*By block*, each group of cells' top-1 and held-out forms:

- adjective: adjective 98.48% (7844), superlative 94.64% (56)
- verb: verb 99.07% (53688), imperative 98.87% (6386), negative_imperative 99.15% (6388), gerund 99.45% (1283), participle 98.67% (2555)

## The shipped grammar and the scored one

The grammar shipped is learned from every word; the one scored above from all but the
held-out tenth. Their sound laws (each with its tier) and irregular forms, side by side;
a law one has and the other doesn't is listed below the table.

| | sound laws, shipped / scored | the same laws | irregular forms, shipped / scored |
|---|---:|---:|---:|
| noun | 0 / 0 | 0 | 88 / 82 |
| adjective | 1 / 3 | 1 | 48 / 41 |
| verb | 18 / 17 | 15 | 737 / 804 |
| participle | 0 / 0 | 0 | 3 / 3 |
| determiner | 0 / 0 | 0 | 8 / 8 |
| adverb | 0 / 0 | 0 | 8 / 7 |

- adjective: Only the scored grammar's: `i+h → +`, `[cc|nc]+i → +hi`.
- verb: only the shipped grammar's: `ec+e → ec+he after a vowel earlier in the stem`, `[l|n]+ss → +s`, `gl+en → gl+ien`. Only the scored grammar's: `ec+er → ec+her`, `l+ss → l+s`.

| | lines of grammar | bytes |
|---|---:|---:|
| noun | 2128 | 111245 |
| adjective | 1165 | 47117 |
| verb | 4841 | 197099 |
| participle | 382 | 13088 |
| determiner | 368 | 11933 |
| adverb | 320 | 10714 |

## Data and license

The code is MIT or Apache-2.0. The grammar under `src/` is derived from Wiktionary via
[Kaikki](https://kaikki.org/) and [wiktextract](https://github.com/tatuylonen/wiktextract),
and is licensed under [CC BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/); see
`DATA-LICENSE.md`.

## Regenerating

```sh
cd flexion
cargo xtask extract italian
cargo xtask build italian
```
