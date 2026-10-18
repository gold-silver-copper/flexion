# german

German inflection for **any** German word, attested or not: noun declension,
adjectives in all three declensions and three degrees, and verbs (every finite
form in main and subordinate clauses, imperative, infinitives, participles),
each with how sure the grammar is and why.

```rust
use german::*;

assert_eq!(noun("Mann", Case::Nominative, Number::Plural), "Männer");
assert_eq!(noun("haus", Case::Dative, Number::Plural), "Häusern");
assert_eq!(adjective("schön", Declension::Weak, Case::Nominative, Agreement::Masculine), "schöne");
assert_eq!(verb("geben", Tense::Present, Person::Third, Number::Singular), "gibt");
assert_eq!(participle("singen", Participle::Past), "gesungen");
```

A verb that begins with a prefix inflects as the rest of it does. A separable
prefix goes where the form puts it, an inseparable one stays in front, and the
participle's `ge-` goes:

```rust
use german::*;

assert_eq!(verb("anfangen", Tense::Preterite, Person::First, Number::Singular), "fing an");
assert_eq!(subordinate("anfangen", Tense::Present, Person::Third, Number::Singular), "anfängt");
assert_eq!(participle("anfangen", Participle::Past), "angefangen");
assert_eq!(participle("verfangen", Participle::Past), "verfangen");
```

Nouns are always written capitalized, however the word is typed.

## How it works

This crate is German inflection as a grammar, learned by
[flexion](https://github.com/gold-silver-copper/flexion) from the English Wiktionary's and the German Wiktionary's
German entries. Each part of speech is a module under `src/`, and has what its
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
| noun | 143818 | 160 | 49 | 8110 | 14034 | 275 | 0 |
| numeral | 18 | 1 | 1 | 1 | 0 | 0 | 0 |
| proper_noun | 2863 | 16 | 1 | 240 | 368 | 11 | 0 |
| adjective | 14839 | 22 | 5 | 446 | 179 | 16 | 0 |
| verb | 9316 | 68 | 4 | 248 | 725 | 4485 | 75 |
| determiner | 36 | 3 | 0 | 6 | 8 | 39 | 1 |

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
| noun | 6323 | 40053 | **93.75%** (93.3–94.2) | 98.78% | 98.90% | 87.06% | 0.008 | 0.756 | 93.75% | 99.80% | 98.03 / 98.42 / 99.10 | 94.23% | 93.75% (40053) |  | 94.77% (0.00%) | -1.02 (-1.33…-0.73) | 94.53% | 99.69% | 96.86% / 90.00% |  | 93.57% (5249) / 98.90% (730) / 87.87% (344) |  | 0.49% |
| numeral | 2 | 14 | **reproduced 126 of 126**; a new word 100.00% (18 words) | 100.00% | 100.00% | 100.00% | 0.000 | 0.000 | 100.00% |  | 100.00 / 100.00 / 100.00 | 100.00% | 100.00% (14) |  | 100.00% (0.00%) | +0.00 (+0.00…+0.00) | 100.00% | 100.00% | 0.00% / 100.00% |  | 100.00% (2) / – / – |  | 0.00% |
| proper_noun | 309 | 1263 | **91.13%** (88.1–93.9) | 98.97% | 99.29% | 86.73% | 0.020 | 0.701 | 91.13% | 99.77% | 97.90 / 97.67 / 98.52 | 91.13% | 91.13% (1263) |  | 90.66% (0.00%) | +0.48 (-1.86…+2.78) | 92.72% | 99.21% | 100.00% / 90.64% |  | 90.98% (296) / 100.00% (1) / 95.00% (12) |  | 0.28% |
| adjective | 1487 | 125177 | **98.90%** (98.4–99.4) | 99.80% | 99.84% | 98.59% | 0.003 | 0.124 | 98.90% | 99.88% | 99.48 / 99.48 / 99.47 |  | 98.90% (125177) |  | 99.02% (0.00%) | -0.12 (-0.59…+0.35) | 99.18% | 100.00% | 98.86% / 98.92% |  | 98.85% (1410) / 100.00% (28) / 100.00% (49) | 96.90% (548) | 0.07% |
| verb | 970 | 50575 | **95.21%** (94.2–96.1) | 98.32% | 98.50% | 85.67% | 0.011 | 0.970 | 95.21% | 96.02% | 95.57 / 95.62 / 95.66 | 95.32% | 95.21% (50575) |  | 73.16% (0.00%) | +22.05 (+20.30…+23.83) | 95.31% | 99.25% | 95.91% / 94.43% |  | 95.04% (936) / – / 99.83% (34) | 91.76% (9057) | 5.07% |
| determiner | 3 | 45 | **reproduced 532 of 532**; a new word 82.52% (36 words) | 73.33% | 73.33% | 33.33% | 0.598 | 16.615 | 40.00% |  | 69.05 / 69.23 / 69.44 |  | 40.00% (45) |  | 40.00% (0.00%) | +0.00 (+0.00…+0.00) | 40.00% | 100.00% | 0.00% / 40.00% |  | 40.00% (3) / – / – |  | 13.89% |

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
| noun | 2503 | 67% | 0% | 0% | 4% | 12% | 12% | 4% | 0% | 93.75% | 100.00% | 100.00% |
| numeral | 0 | – | – | – | – | – | – | – | – | – | 100.00% | 100.00% |
| proper_noun | 112 | 71% | 0% | 0% | 0% | 7% | 18% | 4% | 0% | 91.13% | 100.00% | 100.00% |
| adjective | 1372 | 64% | 0% | 0% | 0% | 11% | 25% | 0% | 0% | 98.24% | 100.00% | 100.00% |
| verb | 2423 | 57% | 0% | 0% | 19% | 7% | 2% | 15% | 0% | 94.24% | 100.00% | 100.00% |
| determiner | 27 | 56% | 0% | 0% | 22% | 22% | 0% | 0% | 0% | 40.00% | 100.00% | 100.00% |

*By block*, each group of cells' top-1 and held-out forms:

- adjective: adjective 99.79% (71376), comparative 98.54% (26352), superlative 96.90% (26352), predicative 97.72% (1097)
- verb: verb 92.65% (23184), subordinate 97.99% (23184), imperative 94.00% (1916), zu_infinitive 92.31% (351), participle 94.23% (1940)

## The shipped grammar and the scored one

The grammar shipped is learned from every word; the one scored above from all but the
held-out tenth. Their sound laws (each with its tier) and irregular forms, side by side;
a law one has and the other doesn't is listed below the table.

| | sound laws, shipped / scored | the same laws | irregular forms, shipped / scored |
|---|---:|---:|---:|
| noun | 49 / 47 | 30 | 275 / 270 |
| numeral | 1 / 1 | 1 | 0 / 0 |
| proper_noun | 1 / 1 | 0 | 11 / 11 |
| adjective | 5 / 4 | 2 | 16 / 26 |
| verb | 4 / 4 | 3 | 4485 / 7181 |
| determiner | 0 / 0 | 0 | 39 / 34 |

- noun: only the shipped grammar's: `um → ∅ after [b|c|d|e|g|i|k|l|m|n|r|s|t|u|v|ä]; us → ∅ after [b|c|d|e|g|i|k|l|m|n|p|r|s|t]; o → ∅ after [d|i|k|l|n|p|t|z]; is → ∅ after [l|s|x] in alternating stems (ˇ), before +[a|e|i|o|u|y|ä|ö|ü]$, +[a|e|i|o|u|y|ä|ö|ü]n`, `[hr|al|hl|ll|an]+n → +en`, `y+n → y+s`, `u+n → u+en`, `p+n → p+s`, `[x|hn|ür]+n → +en`, `on → ∅ after [h|i|k|l|n|o|r|t|x]; s → ∅ after [e|o]; ß → ss after [a|e|i|o]; s → ti after n; k → z after i in alternating stems (ˇ), before +[a|e|i|o|u|y|ä|ö|ü]$`, `[é|mi]+e → +s`, `ki+en → ki+s`, `[ok|fi]+en → +s`, `ok+e → ok+s`, `[w|fi|fa|ka|bi]+e → +s`, `[wn|bi]+en → +s`, `wn+n → wn+s`, `[wn|ü|ga|gn]+e → +s`, `[ü|gn|ku]+en → +s`, `[ic|ou|ru]+en → +s`, `[ja|wa|ic|ku|ou|ru|ba|hu]+e → +s`, `hu+en → hu+s`. Only the scored grammar's: `um → ∅ after [b|d|e|g|i|k|l|m|n|r|s|t|u|v|ä]; us → ∅ after [b|c|d|e|g|i|k|l|m|n|p|r|s|t]; o → ∅ after [d|i|k|l|n|t]; is → ∅ after [l|s|x] in alternating stems (ˇ), before +[a|e|i|o|u|y|ä|ö|ü]$, +[a|e|i|o|u|y|ä|ö|ü]n`, `o+en → o+s`, `[hr|hl]+n → +en`, `[y|p]+n → +s`, `[u|x|hn|ür]+n → +en`, `on → ∅ after [h|i|k|l|n|r|t|x]; s → ∅ after [e|o]; ß → ss after [a|e|i|o]; s → ti after n; k → z after i in alternating stems (ˇ), before +[a|e|i|o|u|y|ä|ö|ü]$`, `é+e → é+s`, `[ki|fi]+en → +s`, `ok+en → ok+s`, `fi+e → fi+s`, `[ok|w|fa|bi|gn|ka]+e → +s`, `bi+en → bi+s`, `[gn|wn|ü]+en → +s`, `[ü|ga]+e → +s`, `[ic|ku]+en → +s`, `[ja|wa|ic|wn|ku|ou|ru|xi]+e → +s`, `[ou|ru]+en → +s`.
- proper_noun: only the shipped grammar's: `[z|ls|ns|rs|ß|os]+s → +`. Only the scored grammar's: `[z|ls|ns|rs|os|ß]+s → +`.
- adjective: only the shipped grammar's: `[s|nt|ft|ht|st|kt|u|zt|mt|bt|ß|ut|it|id|at|ei|dt|sk|x|z|md|pt]+s → +es`, `äh+s → ä+chs after a vowel earlier in the stem`, `lt+s → lt+es after [a|h|i|l|o|u|ä|‹]`. Only the scored grammar's: `[s|nt|ft|ht|st|kt|u|zt|bt|mt|ß|ut|it|at|id|ei|dt|sk|x|z|md|pt]+s → +es`, `äh+es → ä+chs`.
- verb: only the shipped grammar's: `[h|i]+f → +`. Only the scored grammar's: `it+ → it+t after r`.

| | lines of grammar | bytes |
|---|---:|---:|
| noun | 13530 | 733757 |
| numeral | 378 | 12994 |
| proper_noun | 883 | 37613 |
| adjective | 2239 | 101762 |
| verb | 4826 | 296548 |
| determiner | 423 | 14999 |

## Data and license

The code is MIT or Apache-2.0. The grammar under `src/` is derived from Wiktionary via
[Kaikki](https://kaikki.org/) and [wiktextract](https://github.com/tatuylonen/wiktextract),
and is licensed under [CC BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/); see
`DATA-LICENSE.md`.

## Regenerating

```sh
cd flexion
cargo xtask extract german
cargo xtask build german
```
