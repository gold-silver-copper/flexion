# latin

Latin inflection for **any** Latin word, attested or not: nouns, adjectives and
participles, verbs (every finite form, imperatives, infinitives, participles,
gerund, supine), with long vowels marked, each with how sure the grammar is
and why.

```rust
use latin::*;

assert_eq!(noun("rosa", Case::Ablative, Number::Plural), "rosīs");
assert_eq!(noun("rex", Case::Genitive, Number::Singular), "rēgis");
assert_eq!(adjective("bonus", Case::Dative, Gender::Feminine, Number::Singular), "bonae");
assert_eq!(
    indicative("amo", Voice::Active, Tense::Perfect, Person::Third, Number::Plural),
    "amāvērunt"
);
assert_eq!(
    subjunctive("amo", Voice::Active, SubjunctiveTense::Imperfect, Person::First, Number::Singular),
    "amārem"
);
```

**Vowel length.** A word typed without macrons is found anyway, and comes back
with its long vowels marked. For a word the dictionary doesn't have, the
grammar restores the marks by rules on the letters around each vowel (`-ātiō`,
`dē-`), but a stem's own vowel lengths can't be told from its spelling, and its
answers say so. Type the lemma with its macrons and say so, and they are
certain:

```rust
use latin::*;

let word = Noun::new("thēsaurus").spelled_in_full();
assert_eq!(word.form((Case::Genitive, Number::Singular)), "thēsaurī");
```

## How it works

This crate is Latin inflection as a grammar, learned by
[flexion](https://github.com/gold-silver-copper/flexion) from the English Wiktionary's
Latin entries. Each part of speech is a module under `src/`, and has what its
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
| noun | 15883 | 95 | 20 | 497 | 1300 | 224 | 14 |
| proper_noun | 8787 | 83 | 10 | 395 | 1009 | 154 | 11 |
| adjective | 27458 | 66 | 11 | 298 | 516 | 212 | 34 |
| verb | 6168 | 200 | 41 | 867 | 1074 | 495 | 61 |
| pronoun | 80 | 12 | 0 | 18 | 63 | 843 | 39 |
| numeral | 118 | 9 | 0 | 12 | 5 | 6 | 0 |
| adverb | 612 | 13 | 1 | 35 | 49 | 23 | 0 |

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
| noun | 1565 | 17859 | **71.83%** (69.6–74.1) | 91.56% | 93.65% | 70.10% | 0.036 | 3.942 | 92.93% | 98.70% | 97.83 / 99.32 / 99.41 | 72.64% | 71.85% (17854) | 0.00% (5) | 59.09% (0.00%) | +12.74 (+10.05…+15.41) | 71.94% | 99.46% | 79.61% / 69.44% | 70.33% (984) | 71.91% (1379) / 83.25% (19) / 69.90% (167) |  | 1.16% |
| proper_noun | 834 | 5119 | **60.13%** (56.7–63.4) | 87.50% | 90.17% | 59.59% | 0.056 | 5.744 | 92.79% | 98.32% | 96.81 / 98.54 / 98.71 | 60.13% | 60.34% (5093) | 19.23% (26) | 60.58% (26.92%) | -0.45 (-4.19…+3.41) | 60.62% | 99.32% | 58.77% / 60.25% | 47.79% (362) | 59.91% (801) / – / 65.22% (33) |  | 1.64% |
| adjective | 2841 | 101595 | **87.82%** (86.6–89.0) | 97.42% | 97.55% | 86.77% | 0.019 | 1.818 | 99.23% | 99.48% | 99.52 / 99.61 / 99.61 |  | 87.82% (101594) | 0.00% (1) | 71.87% (0.00%) | +15.95 (+14.14…+17.75) | 87.93% | 99.60% | 90.53% / 86.12% | 87.12% (2151) | 77.35% (981) / 93.97% (1684) / 87.18% (176) |  | 0.48% |
| verb | 619 | 77103 | **76.87%** (73.7–80.0) | 94.48% | 97.27% | 69.14% | 0.026 | 2.640 | 85.45% | 98.55% | 95.69 / 95.81 / 96.12 |  | 76.95% (76845) | 52.33% (258) | 64.17% (25.97%) | +12.71 (+8.80…+16.80) | 77.00% | 98.43% | 83.68% / 73.94% | 88.01% (592) | 77.55% (541) / – / 72.34% (78) | 73.02% (16157) | 4.69% |
| pronoun | 9 | 268 | **reproduced 2211 of 2211**; a new word 30.66% (80 words) | 27.99% | 43.66% | 0.00% | 0.230 | 31.506 | 19.08% |  | 33.20 / 35.25 / 35.74 |  | 6.34% (268) |  | 25.00% (0.00%) | -18.66 (-47.89…+3.14) | 10.07% | 100.00% | 0.00% / 6.34% | 14.29% (7) | 8.42% (7) / – / 0.00% (2) |  | 73.75% |
| numeral | 11 | 396 | **reproduced 3996 of 3996**; a new word 85.01% (118 words) | 100.00% | 100.00% | 72.73% | 0.303 | 1.225 | 100.00% |  | 100.00 / 100.00 / 100.00 |  | 72.73% (396) |  | 72.73% (0.00%) | +0.00 (-36.36…+36.36) | 72.73% | 100.00% | 66.67% / 75.00% | 72.73% (11) | 62.50% (8) / – / 100.00% (3) |  | 0.85% |
| adverb | 63 | 175 | **88.57%** (80.0–96.0) | 98.86% | 98.86% | 87.30% | 0.075 | 1.182 | 98.35% | 96.72% | 98.28 / 0.00 / 0.00 |  | 88.57% (175) |  | 44.57% (0.00%) | +44.00 (+30.05…+57.71) | 89.14% | 100.00% | 100.00% / 88.17% | 88.89% (54) | 87.73% (58) / 100.00% (3) / 100.00% (2) |  | 5.56% |

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
| noun | 5031 | 16% | 76% | 0% | 2% | 4% | 0% | 2% | 0% | 71.83% | 100.00% | 100.00% |
| proper_noun | 2041 | 7% | 83% | 0% | 2% | 5% | 1% | 2% | 0% | 58.19% | 99.97% | 100.00% |
| adjective | 12373 | 2% | 92% | 0% | 0% | 2% | 1% | 3% | 0% | 81.95% | 99.95% | 100.00% |
| verb | 17832 | 46% | 38% | 0% | 5% | 4% | 1% | 6% | 0% | 77.03% | 100.00% | 100.00% |
| pronoun | 251 | 24% | 15% | 0% | 8% | 49% | 4% | 0% | 0% | 6.34% | 100.00% | 100.00% |
| numeral | 108 | 0% | 100% | 0% | 0% | 0% | 0% | 0% | 0% | 72.73% | 100.00% | 100.00% |
| adverb | 20 | 0% | 90% | 0% | 0% | 5% | 5% | 0% | 0% | 88.57% | 100.00% | 100.00% |

*By block*, each group of cells' top-1 and held-out forms:

- noun: noun 71.83% (17856); under 30 forms (not in the mean): noun_locative 66.67% (3)
- proper_noun: proper_noun 60.35% (4862), proper_noun_locative 56.03% (257)
- adjective: adjective 87.83% (101478), comparative 75.00% (64), superlative 83.02% (53)
- verb: indicative 76.78% (37461), subjunctive 76.52% (24990), present_imperative 79.17% (2160), future_imperative 79.49% (3828), infinitive 75.49% (3064), participle 75.92% (2110), gerund 79.90% (2468), supine 72.99% (1022)

## The shipped grammar and the scored one

The grammar shipped is learned from every word; the one scored above from all but the
held-out tenth. Their sound laws (each with its tier) and irregular forms, side by side;
a law one has and the other doesn't is listed below the table.

| | sound laws, shipped / scored | the same laws | irregular forms, shipped / scored |
|---|---:|---:|---:|
| noun | 20 / 18 | 16 | 224 / 223 |
| proper_noun | 10 / 9 | 8 | 154 / 137 |
| adjective | 11 / 10 | 9 | 212 / 218 |
| verb | 41 / 39 | 29 | 495 / 466 |
| pronoun | 0 / 0 | 0 | 843 / 665 |
| numeral | 0 / 0 | 0 | 6 / 6 |
| adverb | 1 / 0 | 0 | 23 / 27 |

- noun: only the shipped grammar's: `s → ∅ after [e|p|ā|ē|ō|ū]; ō → i after [d|g]; us → e after [d|l|m|n] in alternating stems (ˇ), before +[e|i|n|r|t|u|ē]`, `[r|n|l|x]+n → +`, `in+c → in+g`, `xi+e → x+e`. Only the scored grammar's: `s → ∅ after [e|p|ā|ē|ō|ū]; ō → i after [d|g]; us → e after [d|m|n] in alternating stems (ˇ), before +[e|i|n|r|t|u|ē]`, `[r|n|l]+n → +`.
- proper_noun: only the shipped grammar's: `ū+t → +unt`, `ēn+t → +ent`. Only the scored grammar's: `[ā|ē]+d → +t`.
- adjective: only the shipped grammar's: `ē → e after [c|d|i|l|m|p|r|t|u] in alternating stems (ˇ), before +.., .+..`, `p+t → p+`. Only the scored grammar's: `x → c after [ā|ī|ō]; ēs → ed after p; ex → ic after l; s → d after r in alternating stems (ˇ), before +.`.
- verb: only the shipped grammar's: `u → ū after [b|i|l|n|p|r|s|t]; er → ∅ after s in alternating stems (ˇ), before +.[e|i|u|ē|ī|ū], +.tu[m|s], +.t[ī|ū], .+[c|s|x]`, `n → ∅ after i; un → ū after [f|t]; um → ū after r; ō → i after n; ī → ē after s in alternating stems (ˇ), before +[c|t], +d.r[a|u|ā|ī|ō], +d.ri[m|n|t], +d.s., +e.[a|ā|ī], +e.i.., +i..[a|e|i|o|u|y|ā|ē|ī|ō|ū|ȳ], [^a|e|i|o|u|y|ā|ē|ī|ō|ū|ȳ]+.r[a|u|ā|ī|ō], [^a|e|i|o|u|y|ā|ē|ī|ō|ū|ȳ]+.ri[m|n|t], [^a|e|i|o|u|y|ā|ē|ī|ō|ū|ȳ]+.s., [^a|e|i|o|u|y|ā|ē|ī|ō|ū|ȳ]+.[u|ī|ū]`, `c+s → +x`, `ru+s → ru+`, `fī+ct → fī+x after a vowel earlier in the stem`, `g+c → +c`, `b+id → b+`, `oc+it → oc+t after a vowel earlier in the stem`, `qu+c → +c`, `qu+x → +x`, `ol+t → +ult`, `[ā|e]+t → +st after p before [u|ī|ū]`. Only the scored grammar's: `u → ū after [b|i|l|n|p|s|t]; u → ūc after r in alternating stems (ˇ), before +.[a|e|i|o|u|y|ā|ē|ī|ō|ū|ȳ], .+[c|s|x]`, `n → ∅ after i; un → ū after [f|t]; um → ū after r; ī → ē after s; ō → i after n in alternating stems (ˇ), before +[c|t], +d.r[a|u|ā|ī|ō], +d.ri[m|n|t], +d.s., +e.[a|ā|ī], +e.i.., +i..[a|e|i|o|u|y|ā|ē|ī|ō|ū|ȳ], [^a|e|i|o|u|y|ā|ē|ī|ō|ū|ȳ]+.r[a|u|ā|ī|ō], [^a|e|i|o|u|y|ā|ē|ī|ō|ū|ȳ]+.ri[m|n|t], [^a|e|i|o|u|y|ā|ē|ī|ō|ū|ȳ]+.s., [^a|e|i|o|u|y|ā|ē|ī|ō|ū|ȳ]+.[u|ī|ū]`, `in+t → i+t`, `on+s → +ōns`, `in+s → +ēv`, `l+id → l+u`, `oc+i → oc+ after a vowel earlier in the stem`, `s+s → s+īv`, `lī+xu → lī+ctu`, `lī+xū → lī+ctū`.
- adverb: only the shipped grammar's: `L → that letter; l after none (the letter just before it, if one of l, r)`.

| | lines of grammar | bytes |
|---|---:|---:|
| noun | 5879 | 210885 |
| proper_noun | 5085 | 179601 |
| adjective | 4940 | 180000 |
| verb | 10221 | 422021 |
| pronoun | 1265 | 60983 |
| numeral | 750 | 26493 |
| adverb | 657 | 23285 |

## Data and license

The code is MIT or Apache-2.0. The grammar under `src/` is derived from Wiktionary via
[Kaikki](https://kaikki.org/) and [wiktextract](https://github.com/tatuylonen/wiktextract),
and is licensed under [CC BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/); see
`DATA-LICENSE.md`.

## Regenerating

```sh
cd flexion
cargo xtask extract latin
cargo xtask build latin
```
