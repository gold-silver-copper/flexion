# ancient-greek

Ancient Greek (Attic) inflection for **any** word, attested or not: nouns,
adjectives, and verbs in every tense, mood and voice, with accents, breathings
and vowel length, each with how sure the grammar is and why.

```rust
use ancient_greek::*;

assert_eq!(noun("λόγος", Case::Genitive, Number::Plural), "λόγων");
assert_eq!(noun("δῶρον", Case::Genitive, Number::Singular), "δώρου");
assert_eq!(present("λύω", Mood::Indicative, Voice::Active, Person::Third, Number::Plural), "λῡ́ουσῐ");
```

The accent is placed by rule. A noun keeps it on the syllable its lemma has it
on, as far as the law of limitation allows: on the third syllable from the end
only before a short last syllable, and a long next-to-last syllable before a
short last one takes the circumflex. A verb's accent recedes as far as the law
allows.

```rust
use ancient_greek::*;

// Vowel length is marked on α, ι, υ, as Wiktionary's tables do, except
// where a breathing takes the letter (`ᾰ̓` is written `ἀ`).
assert_eq!(noun("ἄνθρωπος", Case::Genitive, Number::Singular), "ἀνθρώπου");
assert_eq!(noun("δῶρον", Case::Nominative, Number::Plural), "δῶρᾰ");
```

A word is found whether or not it is typed with breathings, iota subscripts or
length marks. For a word the dictionary doesn't have, vowel length is restored
by rules on the letters around each vowel; type the lemma with its marks and
say so (`spelled_in_full`), and none is a guess.

## How it works

This crate is Ancient Greek inflection as a grammar, learned by
[flexion](https://github.com/gold-silver-copper/flexion) from the English Wiktionary's
Ancient Greek entries. Each part of speech is a module under `src/`, and has what its
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
- `accent.rs`: per cell, which rule places the accent, and which words have which
  pattern;
- `prefixes.rs`: prefixes a word inflects behind, separable or not;
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
| noun | 8124 | 86 | 5 | 316 | 1715 | 1992 | 61 |
| proper_noun | 3793 | 68 | 1 | 200 | 702 | 680 | 27 |
| adjective | 4674 | 46 | 3 | 330 | 1008 | 3578 | 84 |
| verb | 2655 | 284 | 22 | 273 | 1440 | 25869 | 426 |
| pronoun | 58 | 19 | 0 | 14 | 35 | 415 | 13 |
| numeral | 19 | 7 | 0 | 4 | 5 | 22 | 0 |

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
| noun | 809 | 11483 | **74.96%** (72.1–77.8) | 91.35% | 94.10% | 62.92% | 0.046 | 3.790 | 89.14% | 96.10% | 94.53 / 95.81 / 96.07 | 75.96% | 74.96% (11483) |  | 51.28% (0.00%) | +25.29 (+21.30…+28.96) | 74.98% | 99.55% | 81.03% / 73.54% | 79.18% (586) | 74.79% (784) / 66.67% (3) / 82.32% (22) |  | 8.64% |
| proper_noun | 383 | 2014 | **60.28%** (55.1–65.6) | 80.73% | 83.37% | 55.09% | 0.076 | 8.590 | 86.95% | 91.55% | 94.40 / 95.82 / 96.30 | 61.72% | 60.28% (2014) |  | 47.42% (0.00%) | +14.45 (+8.29…+20.98) | 60.43% | 99.40% | 59.65% / 60.34% | 65.16% (221) | 60.67% (375) / – / 43.48% (8) |  | 5.14% |
| adjective | 474 | 22171 | **78.19%** (74.9–81.4) | 93.33% | 95.03% | 63.92% | 0.039 | 3.402 | 90.76% | 98.02% | 94.65 / 94.93 / 97.06 |  | 78.21% (22161) | 40.00% (10) | 52.46% (40.00%) | +26.51 (+21.96…+30.76) | 78.39% | 99.17% | 82.37% / 77.38% | 84.39% (301) | 79.52% (349) / 74.94% (115) / 68.31% (10) |  | 2.78% |
| verb | 257 | 68968 | **68.30%** (63.8–72.7) | 81.20% | 84.33% | 24.12% | 0.072 | 10.236 | 75.88% | 81.95% | 78.99 / 78.94 / 79.29 |  | 68.43% (68730) | 29.83% (238) | 44.20% (15.97%) | +25.27 (+19.32…+30.88) | 68.65% | 99.90% | 77.29% / 63.03% | 83.12% (160) | 68.86% (244) / 45.83% (3) / 55.25% (10) | 56.29% (4832) | 44.33% |
| pronoun | 11 | 367 | **reproduced 1771 of 1771**; a new word 53.75% (58 words) | 44.14% | 44.14% | 9.09% | 0.477 | 12.382 | 40.16% |  | 41.69 / 41.86 / 42.04 |  | 44.71% (331) | 0.00% (36) | 41.96% (50.00%) | +7.08 (-8.14…+30.97) | 40.33% | 99.46% | 0.00% / 40.33% | 100.00% (1) | 37.37% (8) / 88.89% (1) / 0.00% (2) |  | 51.72% |
| numeral | 2 | 30 | **reproduced 301 of 301**; a new word 66.78% (19 words) | 100.00% | 100.00% | 100.00% | 0.061 | 0.091 | 100.00% |  | 100.00 / 100.00 / 100.00 |  | 100.00% (30) |  | 100.00% (0.00%) | +0.00 (+0.00…+0.00) | 100.00% | 100.00% | 0.00% / 100.00% |  | 100.00% (2) / – / – |  | 21.05% |

*Compatible with stated length*: the dictionary marks a vowel's length both ways,
long and short, so a vowel it writes with neither has its length unstated where the
word's table leaves that vowel bare in every form, and stated where the table marks it
elsewhere (a circumflex marks it long). An answer that is the dictionary's form but for a
length mark on a vowel the form leaves bare is compatible with it where the table never
states that vowel's length otherwise. A second score, for the grammar and the floor alike,
beside top-1, which stays the score: the forms it adds are not more forms right.

| | top-1 | compatible with stated length | floor | floor, compatible | against the floor, compatible |
|---|---:|---:|---:|---:|---:|
| noun | 74.96% | 80.97% | 51.28% | 55.25% | +27.39 (+23.79…+30.89) |
| proper_noun | 60.28% | 78.35% | 47.42% | 58.69% | +21.45 (+16.26…+26.92) |
| adjective | 78.19% | 84.96% | 52.46% | 57.70% | +28.08 (+23.65…+32.43) |
| verb | 68.30% | 74.83% | 44.20% | 46.20% | +29.88 (+24.47…+35.06) |
| pronoun | 40.33% | 40.33% | 41.96% | 41.96% | +7.08 (-8.14…+30.97) |
| numeral | 100.00% | 100.00% | 100.00% | 100.00% | +0.00 (+0.00…+0.00) |

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
| noun | 2875 | 27% | 58% | 6% | 2% | 6% | 0% | 1% | 0% | 74.96% | 99.89% | 100.00% |
| proper_noun | 800 | 14% | 74% | 4% | 2% | 6% | 0% | 0% | 0% | 60.28% | 99.88% | 100.00% |
| adjective | 4835 | 28% | 58% | 3% | 4% | 4% | 1% | 1% | 0% | 80.85% | 99.84% | 100.00% |
| verb | 21862 | 20% | 42% | 4% | 19% | 14% | 1% | 0% | 0% | 66.52% | 99.06% | 100.00% |
| pronoun | 219 | 6% | 0% | 15% | 1% | 78% | 0% | 0% | 0% | 40.33% | 100.00% | 100.00% |
| numeral | 0 | – | – | – | – | – | – | – | – | 100.00% | 100.00% | 100.00% |

*By block*, each group of cells' top-1 and held-out forms:

- adjective: adjective 78.01% (21052), comparative 82.49% (337), superlative 82.30% (339), adverb 80.59% (443)
- verb: present 79.11% (16394), present_imperative 79.11% (4116), imperfect 64.41% (4959), future 66.33% (7824), aorist 67.11% (12765), aorist_imperative 67.92% (3192), perfect 57.13% (10464), perfect_imperative 58.14% (2616), pluperfect 54.32% (2128), future_perfect 66.67% (240), infinitive 68.85% (2135), participle 69.18% (2135)

## The shipped grammar and the scored one

The grammar shipped is learned from every word; the one scored above from all but the
held-out tenth. Their sound laws (each with its tier) and irregular forms, side by side;
a law one has and the other doesn't is listed below the table.

| | sound laws, shipped / scored | the same laws | irregular forms, shipped / scored |
|---|---:|---:|---:|
| noun | 5 / 5 | 1 | 1992 / 1596 |
| proper_noun | 1 / 1 | 1 | 680 / 645 |
| adjective | 3 / 3 | 2 | 3578 / 1983 |
| verb | 22 / 19 | 11 | 25869 / 23431 |
| pronoun | 0 / 0 | 0 | 415 / 289 |
| numeral | 0 / 0 | 0 | 22 / 22 |

- noun: only the shipped grammar's: `Ᾰ → ᾱ after ε; ᾰ after η, ι, κ, ο, π, ω, ᾰ, ᾱ, ῐ, ῑ, ῠ, none (the nearest of ε, η, ι, κ, ο, π, ω, ᾰ, ᾱ, ῐ, ῑ, ῠ before it, as written)`, `[γ|ρῠ]+κ → +γ`, `[ῑ|ῡ|ἰ]+τ → +ν`, `[ε|ᾰ]+ν → +ς after a vowel earlier in the stem`. Only the scored grammar's: `Ᾰ → ᾱ after ε; ᾰ after η, ι, κ, ο, ω, ᾰ, ᾱ, ῐ, ῑ, ῠ, none (the nearest of ε, η, ι, κ, ο, ω, ᾰ, ᾱ, ῐ, ῑ, ῠ before it, as written)`, `ο+ες → ο+ι`, `γ+κ → γ+γ`, `[ῑ|ἰ]+τ → +ν`.
- adjective: only the shipped grammar's: `[ι|ητ|κτ|ικ|πτ|ηρ|ιν|ωτ|ηλ|ργ|υρ|ων|δν|ωρ|δρ|κρ|υλ|λλ|ρτ|σσ|υτ|ωπ|ῡμ|ημ|ην|θρ|ιρ|μν|ντ|πν|τρ|ιμ|γγ|πρ|ρφ|υκ|ωγ|ῡσ|ιτ|λμ|μβ|νδ|χν|ωλ|ᾱτ|ῡλ|ῡχ]+ωτ → +οτ`. Only the scored grammar's: `[ι|ητ|κτ|ικ|πτ|ηρ|ιν|ωτ|ηλ|ργ|υρ|δν|ων|ωρ|κρ|δρ|σσ|υλ|λλ|ρτ|ωπ|ῡμ|ντ|πν|υτ|ημ|θρ|ιρ|μν|ην|ιμ|τρ|ῡσ|λμ|πρ|ρφ|υκ|χν|ᾱτ|ῡχ]+ωτ → +οτ`.
- verb: only the shipped grammar's: `ἐ‹α → η, αἰ → ᾐ, αἱ → ᾑ, γῡ → ἐγυ, ε → ε, εὐ → ηὐ, κι → ἐκῐ, λῠ → ἐλυ, ο → ο, οχ → ωχ, οἰ → ᾠ, υ → υ, χυ → ἐκχυ, ἀ → ἠ, ἁ → ἡ, ἐθ → εἰθ, ἐκ → ἐξε, ἐμ → ἐνε, ἐξ → ἐξ, ἐπ → ἐπ, ἐφ → ἐφ, ἐχ → εἰχ, ἑ → εἱ, ἑρ → εἰρ, ἑψ → ἡψ, ἠ → ἠ, ἡ → ἡ, ἰ → ἰ, ἱ → ἱ, ὀ → ὠ, ὁ → ὡ, ὐ → εὐ, ὑ → ὑ, ὠ → ὠ, ὠθ → ἐωθ, ὠν → ἐων, ᾀ → ᾐ, ᾱ → ἀε, ῐ → ῑ, ῥ → ἐρρ (αι → ῃ, αϊ → ῃ, εθ → ειθ, εκ → εξε, εμ → ενε, εξ → εξ, επ → επ, ερ → εἰρ, ευ → ηυ, εφ → εφ, εχ → ειχ, εψ → ηψ, εϋ → ηυ, η → η, ι → ι, οι → ῳ, οϊ → ῳ, ω → ω, ωθ → εωθ, ων → εων, ϊ → ι, ϋ → ευ, ᾳ → ῃ)`, `ἐ%ε‹ἀ → ἠ, ἀν → ἀν, ἀτ → ητ (α → η, αν → αν, ατ → ητ)`, `%ε‹ἐ → ἠ, ἐθ → εἰθ, ἐκ → ἐκδε, ἐμ → ἐμημ, ἐν → ἐνδε, ἐξ → ἐξ, ἐχ → ἐσχ (ε → η, εθ → ειθ, εκ → εκδε, εμ → εμημ, εν → ενδε, εξ → εξ, εχ → εσχ)`, `κ+σ → +ξ`, `%ε‹ὑ → ὑ (υ → υ, ϋ → υ)`, `ζ+θ → +σθ`, `ν+σ → ν+`, `%ε‹α → η, αἰ → ᾐ, αἱ → ᾑ (αι → ῃ, αϊ → ῃ)`, `ἐ%ε‹α → η, αἰ → ᾐ, αἱ → ᾑ (αι → ῃ, αϊ → ῃ)`, `ἐ%ε‹ὑ → ὑ (υ → υ, ϋ → υ)`, `[υ|ι|ε|ῡ|ῠ|ῑ|ᾱ|ὑ|ω]+θ → +σθ after [α|ζ|ξ]`. Only the scored grammar's: `ἐ‹α → η, αἰ → ᾐ, αἱ → ᾑ, γῡ → ἐγυ, ε → ε, εὐ → ηὐ, λῠ → ἐλυ, οχ → ωχ, οἰ → ᾠ, οὐ → ἐου, υ → υ, χυ → ἐκχυ, ϊ → ϊ, ἀ → ἠ, ἀφ → ἀφ, ἁ → ἡ, ἐθ → εἰθ, ἐκ → ἐξε, ἐμ → ἐνε, ἐξ → ἐξ, ἐπ → ἐπ, ἐφ → ἐφ, ἐχ → εἰχ, ἑ → εἱ, ἑρ → εἰρ, ἑψ → ἡψ, ἠ → ἠ, ἡ → ἡ, ἰ → ἰ, ἱ → ἱ, ὀ → ὠ, ὁ → ὡ, ὐ → εὐ, ὑ → ὑ, ὠ → ὠ, ὠθ → ἐωθ, ὠν → ἐων, ᾀ → ᾐ, ῐ → ῑ, ῥ → ἐρρ, ῥῠ → ἐρρυ (αι → ῃ, αφ → αφ, εθ → ειθ, εκ → εξε, εμ → ενε, εξ → εξ, επ → επ, ερ → εἰρ, ευ → ηυ, εφ → εφ, εχ → ειχ, εψ → ηψ, η → η, ι → ι, ο → ω, οι → ῳ, ου → εου, ω → ω, ωθ → εωθ, ων → εων, ᾳ → ῃ)`, `ἐ%ε‹ἀ → ἠ (α → η)`, `%ε‹ἐ → ἠ, ἐθ → εἰθ, ἐκ → ἐκδε, ἐμ → ἐμημ, ἐν → ἐἐν, ἐξ → ἐξ, ἐχ → ἐσχ (ε → η, εθ → ειθ, εκ → εκδε, εμ → εμημ, εν → εεν, εξ → εξ, εχ → εσχ)`, `ὀ‹δ → ὁδ, χ → ὠχ, ἰ → οἰ (ι → οι)`, `ὠ‹δ → ὡδ, ἰ → ᾠ (ι → ῳ)`, `%ε‹ὑ → ὑ (υ → υ)`, `%ε‹ὁ → ὡ (ο → ω)`, `ἐ%ε‹ὑ → ὑ (υ → υ)`.

| | lines of grammar | bytes |
|---|---:|---:|
| noun | 6168 | 341027 |
| proper_noun | 4000 | 206885 |
| adjective | 4325 | 250197 |
| verb | 28565 | 2276916 |
| pronoun | 1417 | 61373 |
| numeral | 942 | 35864 |

## Data and license

The code is MIT or Apache-2.0. The grammar under `src/` is derived from Wiktionary via
[Kaikki](https://kaikki.org/) and [wiktextract](https://github.com/tatuylonen/wiktextract),
and is licensed under [CC BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/); see
`DATA-LICENSE.md`.

## Regenerating

```sh
cd flexion
cargo xtask extract ancient-greek
cargo xtask build ancient-greek
```
