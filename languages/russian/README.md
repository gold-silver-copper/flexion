# russian

Russian inflection for **any** Russian word, attested or not: noun declension,
adjectives (long and short forms), verbs (non-past, past, imperative,
participles, gerunds), each with how sure the grammar is and why. Stress is
not written: it is stripped from what you type and never written in answers.

```rust
use russian::*;

assert_eq!(noun("книга", Case::Genitive, Number::Plural), "книг");
assert_eq!(noun("стол", Case::Genitive, Number::Singular), "стола");
assert_eq!(adjective("новый", Case::Dative, Agreement::Feminine, Animacy::Inanimate), "новой");
assert_eq!(non_past("читать", Person::Third, Number::Plural), "читают");
assert_eq!(past("читать", Agreement::Feminine), "читала");
```

A noun's accusative is its genitive if it is animate and its nominative if not.
The grammar knows which for the dictionary's words, and guesses for the others;
tell it, and the guess is gone:

```rust
use russian::*;

let бокр = Noun::new("бокр");
assert_eq!(бокр.form((Case::Genitive, Number::Singular)), "бокра");
let animate = бокр.clone().animacy(Animacy::Animate);
assert_eq!(animate.form((Case::Accusative, Number::Singular)), "бокра");
let inanimate = бокр.animacy(Animacy::Inanimate);
assert_eq!(inanimate.form((Case::Accusative, Number::Singular)), "бокр");

// Each candidate says how likely it is and why.
for c in Noun::new("бокр").candidates((Case::Accusative, Number::Singular)) {
    println!("{c}");
}
```

## How it works

This crate is Russian inflection as a grammar, learned by
[flexion](https://github.com/gold-silver-copper/flexion) from the English Wiktionary's and the Russian Wiktionary's
Russian entries. Each part of speech is a module under `src/`, and has what its
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
  with an ending table, and the forms no stem gives;
- `animacy.rs`: a word's animacy, which decides which cell some cells' forms
  come from.

A word the dictionary lists gets its own forms. Any other word gets the paradigm its
ending suggests, with a probability that says how often such a guess is right, and the
reason (the rule and its counts).

## The grammar

| | words | paradigms | sound laws | rules | listed words | irregular forms | listed by relations |
|---|---:|---:|---:|---:|---:|---:|---:|
| noun | 108878 | 181 | 10 | 1926 | 4160 | 2167 | 24 |
| proper_noun | 3013 | 38 | 2 | 148 | 162 | 158 | 5 |
| adjective | 67690 | 42 | 2 | 598 | 1104 | 123 | 4 |
| verb | 46911 | 341 | 50 | 2902 | 4672 | 605 | 111 |
| pronoun | 42 | 10 | 0 | 10 | 13 | 399 | 2 |
| numeral | 77 | 4 | 5 | 11 | 19 | 129 | 2 |
| adverb | 185 | 5 | 0 | 7 | 37 | 22 | 0 |

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
| noun | 2611 | 27635 | **94.64%** (94.0–95.2) | 99.36% | 99.46% | 77.79% | 0.007 | 0.429 | 94.64% | 95.73% | 95.87 / 95.94 / 96.08 | 97.30% | 94.64% (27628) | 100.00% (7) | 94.72% (85.71%) | -0.09 (-0.52…+0.37) | 94.98% | 99.82% | 97.06% / 93.81% |  | 94.55% (2394) / 96.60% (181) / 90.08% (36) |  | 0.54% |
| proper_noun | 340 | 2439 | **90.77%** (88.5–93.0) | 98.97% | 99.10% | 69.41% | 0.025 | 0.721 | 90.77% | 92.59% | 92.85 / 92.44 / 92.53 | 96.72% | 90.77% (2439) |  | 93.60% (0.00%) | -2.83 (-4.13…-1.49) | 90.90% | 99.88% | 93.22% / 90.71% |  | 91.20% (331) / 100.00% (1) / 69.23% (8) |  | 2.19% |
| adjective | 3466 | 167759 | **99.75%** (99.6–99.9) | 99.99% | 99.99% | 98.15% | 0.002 | 0.012 | 99.75% |  | 99.85 / 99.88 / 99.88 |  | 99.76% (167747) | 0.00% (12) | 99.76% (91.67%) | -0.01 (-0.12…+0.09) | 99.76% | 99.94% | 99.71% / 99.77% |  | 99.13% (950) / 100.00% (2512) / 100.00% (4) |  | 0.07% |
| verb | 1304 | 20554 | **95.74%** (95.0–96.4) | 99.36% | 99.39% | 86.96% | 0.006 | 0.416 | 95.74% | 98.90% | 96.62 / 97.13 / 97.98 | 95.47% | 95.76% (20525) | 82.76% (29) | 96.65% (44.83%) | -0.91 (-1.43…-0.42) | 95.86% | 99.49% | 95.74% / 95.73% |  | 95.71% (1269) / 100.00% (29) / 81.44% (6) |  | 0.75% |
| pronoun | 4 | 188 | **reproduced 1974 of 1974**; a new word 48.83% (42 words) | 97.87% | 97.87% | 50.00% | 0.290 | 1.493 | 54.26% |  | 95.65 / 95.56 / 95.45 |  | 54.26% (188) |  | 54.26% (0.00%) | +0.00 (+0.00…+0.00) | 54.26% | 100.00% | 0.00% / 54.26% |  | 54.26% (4) / – / – |  | 30.95% |
| numeral | 4 | 153 | **reproduced 2986 of 2986**; a new word 83.69% (77 words) | 94.12% | 94.12% | 75.00% | 0.006 | 2.198 | 94.12% |  | 93.96 / 93.79 / 93.62 |  | 94.12% (153) |  | 100.00% (0.00%) | -5.88 (-32.53…+0.00) | 94.12% | 100.00% | 100.00% / 91.51% |  | 94.12% (4) / – / – |  | 24.68% |
| adverb | 18 | 18 | **reproduced 185 of 185**; a new word 77.30% (185 words) | 72.22% | 72.22% | 66.67% | 0.310 | 10.347 | 66.67% |  |  |  | 66.67% (18) |  | 83.33% (0.00%) | -16.67 (-33.33…+0.00) | 72.22% | 100.00% | 0.00% / 66.67% |  | 66.67% (18) / – / – |  | 11.89% |

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
| noun | 1482 | 81% | 0% | 0% | 2% | 7% | 6% | 3% | 0% | 94.64% | 100.00% | 100.00% |
| proper_noun | 225 | 88% | 0% | 0% | 3% | 7% | 1% | 1% | 0% | 90.77% | 100.00% | 100.00% |
| adjective | 412 | 70% | 0% | 0% | 0% | 5% | 0% | 25% | 0% | 98.11% | 100.00% | 100.00% |
| verb | 876 | 75% | 0% | 0% | 8% | 5% | 3% | 9% | 0% | 96.02% | 99.98% | 100.00% |
| pronoun | 86 | 95% | 0% | 0% | 0% | 5% | 0% | 0% | 0% | 54.26% | 100.00% | 100.00% |
| numeral | 9 | 0% | 0% | 0% | 0% | 100% | 0% | 0% | 0% | 94.12% | 100.00% | 100.00% |
| adverb | 6 | 17% | 0% | 0% | 67% | 0% | 17% | 0% | 0% | – | 100.00% | 100.00% |

*By block*, each group of cells' top-1 and held-out forms:

- noun: noun 94.64% (27608); under 30 forms (not in the mean): noun_second_case 92.59% (27)
- adjective: adjective 99.86% (162902), short_adjective 96.36% (4857)
- verb: non_past 93.51% (7795), past 99.08% (5207), imperative 92.06% (2594), participle 97.28% (2909), gerund 98.19% (2049)

## The shipped grammar and the scored one

The grammar shipped is learned from every word; the one scored above from all but the
held-out tenth. Their sound laws (each with its tier) and irregular forms, side by side;
a law one has and the other doesn't is listed below the table.

| | sound laws, shipped / scored | the same laws | irregular forms, shipped / scored |
|---|---:|---:|---:|
| noun | 10 / 14 | 7 | 2167 / 2134 |
| proper_noun | 2 / 3 | 2 | 158 / 114 |
| adjective | 2 / 2 | 2 | 123 / 126 |
| verb | 50 / 50 | 32 | 605 / 574 |
| pronoun | 0 / 0 | 0 | 399 / 396 |
| numeral | 5 / 4 | 4 | 129 / 128 |
| adverb | 0 / 0 | 0 | 22 / 23 |

- noun: only the shipped grammar's: `[к|г|х|ш|ч|ж|щ]+ы → +и`, `иц+о → иц+е`, `зж+о → зж+е`. Only the scored grammar's: `О → о after е, и, л, р, т, ч, ю, none; е after ц, щ (the nearest of е, и, л, р, т, ц, ч, щ, ю before it, as written)`, `к+ы → к+и`, `е → ∅ after [б|в|г|д|ж|з|и|к|л|м|н|п|р|с|т|х|ч|ш|ы] in alternating stems (ˇ), before +[м|х], е.+., к+[а|и|о|у], л+е., л+я, н+е., н+ё, ц+.`, `[г|х|ш|ч|ж|щ]+ы → +и`, `к+ей → к+ой`, `к+е → к+о before [в|г]`, `[г|сц]+е → +о before [в|й|м]`.
- proper_noun: Only the scored grammar's: `[ч|ш|ц|ж|щ]+о → +е after [а|д|и|к|о|у|я]`.
- verb: only the shipped grammar's: `[ч|ж|ш]+ю → +у`, `[ч|ж|ш]+я → +а`, `[о|сл]+ь → +и`, `к → г after б, ж, и, м, я unless з, л, п, с, т comes between, not before а`, `[тр|др]+ь → +и`, `[мн|гл|хл|бл|тл]+ь → +и`, `н+й → +й`, `ни+ву → ни+ю`, `[гр|зл|кл]+ь → +и`, `ъе+ше → ъе+де`, `ъе+шу → ъе+ду`, `[в|р]+оё → +ё`, `ев+ой → ев+и`, `ев+ою → ев+у`, `цв+ёт → цв+ет`, `ъе+ши → ъе+зжай`, `ел+ё → ел+е after [т|щ]`, `он+ё → он+е after [а|о|с]т`. Only the scored grammar's: `ж+ю → ж+у`, `ж+я → ж+а`, `[о|сл|тр]+ь → +и`, `к → г after б, ж, и, м, у, я unless в, з, с, т comes between, not before а`, `др+ь → др+и`, `ис+и → и+ши after a vowel earlier in the stem`, `бл+у → бл+ю`, `мн+ь → мн+и`, `ор+л → ор+`, `ня+ну → +йму`, `ня+нь → +йми`, `ня+не → +ймё`, `[гл|бл]+ь → +и`, `[тл|гр|кл]+ь → +и`, `ж+ег → ж+г`, `зл+ь → зл+и`, `бл+юе → бл+юё`, `ел+ё → ел+е after [т|ц]`.
- numeral: only the shipped grammar's: `Ы → ы after е; и after о; е after none (the nearest of е, о before it, as written)`.

| | lines of grammar | bytes |
|---|---:|---:|
| noun | 14678 | 927471 |
| proper_noun | 1724 | 83813 |
| adjective | 2590 | 150561 |
| verb | 10425 | 685089 |
| pronoun | 842 | 38760 |
| numeral | 858 | 35300 |
| adverb | 380 | 13241 |

## Data and license

The code is MIT or Apache-2.0. The grammar under `src/` is derived from Wiktionary via
[Kaikki](https://kaikki.org/) and [wiktextract](https://github.com/tatuylonen/wiktextract),
and is licensed under [CC BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/); see
`DATA-LICENSE.md`.

## Regenerating

```sh
cd flexion
cargo xtask extract russian
cargo xtask build russian
```
