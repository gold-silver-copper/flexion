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
| noun | 228072 | 84 | 0 | 2233 | 5726 | 249 |
| verb | 38230 | 85 | 16 | 1109 | 1348 | 127 |
| adjective | 61454 | 10 | 7 | 466 | 2502 | 16 |
| adverb | 12620 | 4 | 1 | 81 | 169 | 12 |

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
The margin after top-1 is a 95% interval, counting words. *Lemma in full*: top-1 typed
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
shared task's non-neural baseline (letters only), and on the unseen cells. *Attested*: top-1 of the shipped
grammar on the dictionary's own words, typed in full.

| | held-out words | forms | top-1 | top-3 | among the candidates | every cell right | calibration error | log loss | lemma in full | principal parts given | 1 / 2 / 3 forms given | features given | seen cells | unseen cells | floor (unseen) | attested |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| noun | 23733 | 47466 | **98.58%** ± 0.2 | 99.84% | 99.88% | 97.17% | 0.001 | 0.119 | 98.58% | 100.00% |  |  | 98.58% (47466) |  | 98.39% (0.00%) | 100.00% |
| verb | 4151 | 20745 | **97.80%** ± 0.4 | 99.79% | 99.81% | 95.06% | 0.005 | 0.145 | 97.80% | 99.71% | 98.98 / 99.46 / 99.82 |  | 97.80% (20745) |  | 98.24% (0.00%) | 100.00% |
| adjective | 6239 | 18715 | **96.91%** ± 0.4 | 99.98% | 99.99% | 95.35% | 0.004 | 0.122 | 96.91% | 100.00% | 100.00 / 0.00 / 0.00 |  | 96.91% (18715) |  | 95.02% (0.00%) | 100.00% |
| adverb | 1207 | 3621 | **99.34%** ± 0.5 | 99.94% | 100.00% | 99.01% | 0.002 | 0.072 | 99.34% | 100.00% | 100.00 / 0.00 / 0.00 |  | 99.34% (3621) |  | 98.90% (0.00%) | 100.00% |

| | lines of grammar | bytes |
|---|---:|---:|
| noun | 16098 | 500941 |
| verb | 6046 | 200034 |
| adjective | 3547 | 111390 |
| adverb | 653 | 22936 |

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
