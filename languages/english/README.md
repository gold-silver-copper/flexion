# english

English inflection for **any** English word, attested or not: noun plurals,
verb forms, comparatives and superlatives, each with how sure the grammar is
and why.

```rust
use english::*;

assert_eq!(noun("cat", Number::Plural), "cats");
assert_eq!(noun("child", Number::Plural), "children");
assert_eq!(verb("go", VerbForm::Past), "went");
assert_eq!(adjective("good", Degree::Comparative), "better");
```

A word nobody wrote down inflects by the rules learned from the words that
were. Every answer can come with its alternatives, how likely each is, and why:

```rust
use english::*;

let plurals = Noun::new("glorp").candidates(Number::Plural);
assert_eq!(plurals[0].form, "glorps");
println!("{}", plurals[0]);
```

Agreement, pronouns and possessives are hand-written helpers on top:

```rust
use english::*;

assert_eq!(conjugate("be", Tense::Present, Person::First, Number::Singular), "am");
assert_eq!(conjugate("walk", Tense::Present, Person::Third, Number::Singular), "walks");
assert_eq!(pronoun(Person::Third, Number::Singular, Gender::Feminine, Case::Accusative), "her");
assert_eq!(count_with_number("mouse", 3), "3 mice");
assert_eq!(add_possessive("dogs"), "dogs'");
```

The personal pronouns are a fixed table written by hand (`pronoun`): the
dictionary's head lines give only some of their forms (`he` lists `himself`
and not `him`), so a grammar learned from them would guess the rest.

## How it works

This crate is English inflection as a grammar, learned by
[flexion](https://github.com/gold-silver-copper/flexion) from the English Wiktionary's
English entries. Each part of speech is a module under `src/`, and has what its
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
| noun | 228065 | 84 | 0 | 2233 | 5719 | 242 | 0 |
| proper_noun | 42526 | 5 | 0 | 93 | 273 | 20 | 0 |
| verb | 38229 | 83 | 30 | 1124 | 1664 | 200 | 18 |
| adjective | 61453 | 13 | 4 | 465 | 2506 | 41 | 1 |
| adverb | 12620 | 4 | 2 | 81 | 169 | 16 | 1 |

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
| noun | 23732 | 23732 | **97.17%** (96.9–97.4) | 99.68% | 99.77% | 97.17% | 0.003 | 0.233 | 97.17% |  |  |  | 97.17% (23732) |  | 96.78% (0.00%) | +0.39 (+0.24…+0.55) | 97.98% | 99.88% | 98.68% / 95.73% |  | 97.34% (19552) / 90.00% (60) / 96.50% (4120) |  | 0.11% |
| proper_noun | 4321 | 4321 | **99.35%** (99.1–99.6) | 99.98% | 99.98% | 99.35% | 0.002 | 0.052 | 99.35% |  |  |  | 99.35% (4321) |  | 99.28% (0.00%) | +0.07 (-0.12…+0.28) | 99.68% | 100.00% | 99.47% / 99.33% |  | 99.37% (4311) / – / 90.00% (10) |  | 0.05% |
| verb | 4151 | 16594 | **97.17%** (96.8–97.6) | 99.73% | 99.74% | 94.97% | 0.006 | 0.193 | 97.17% | 99.54% | 98.69 / 99.24 / 99.71 |  | 97.17% (16594) |  | 97.79% (0.00%) | -0.62 (-0.93…-0.33) | 97.61% | 99.86% | 98.92% / 96.23% |  | 97.14% (3598) / 100.00% (3) / 97.36% (550) |  | 1.01% |
| adjective | 6239 | 12476 | **95.35%** (94.8–95.9) | 99.95% | 99.97% | 95.34% | 0.005 | 0.177 | 95.35% |  | 99.98 / 0.00 / 0.00 |  | 95.35% (12476) |  | 92.53% (0.00%) | +2.82 (+2.39…+3.25) | 96.30% | 99.94% | 99.05% / 93.61% |  | 95.59% (5574) / 75.00% (4) / 93.48% (661) | 99.06% (11894) | 0.06% |
| adverb | 1207 | 2414 | **99.01%** (98.4–99.5) | 100.00% | 100.00% | 99.01% | 0.005 | 0.105 | 99.01% |  | 100.00 / 0.00 / 0.00 |  | 99.01% (2414) |  | 98.34% (0.00%) | +0.66 (+0.25…+1.16) | 99.59% | 100.00% | 100.00% / 98.67% |  | 98.95% (1146) / – / 100.00% (61) | 99.67% (2398) | 0.11% |

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
| noun | 671 | 63% | 0% | 0% | 4% | 2% | 28% | 2% | 0% | 97.17% | 100.00% | 100.00% |
| proper_noun | 28 | 46% | 0% | 0% | 4% | 0% | 50% | 0% | 0% | 99.35% | 100.00% | 100.00% |
| verb | 469 | 74% | 0% | 0% | 3% | 3% | 15% | 4% | 0% | 97.17% | 100.00% | 100.00% |
| adjective | 580 | 79% | 0% | 0% | 0% | 0% | 20% | 1% | 0% | 95.35% | 100.00% | 100.00% |
| adverb | 24 | 42% | 0% | 0% | 0% | 0% | 58% | 0% | 0% | 99.01% | 100.00% | 100.00% |

| | lines of grammar | bytes |
|---|---:|---:|
| noun | 4439 | 224502 |
| proper_noun | 541 | 21272 |
| verb | 3626 | 144465 |
| adjective | 1332 | 68115 |
| adverb | 534 | 20468 |

## Data and license

The code is MIT or Apache-2.0. The grammar under `src/` is derived from Wiktionary via
[Kaikki](https://kaikki.org/) and [wiktextract](https://github.com/tatuylonen/wiktextract),
and is licensed under [CC BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/); see
`DATA-LICENSE.md`.

## Regenerating

```sh
cd flexion
cargo xtask extract english
cargo xtask build english
```
