# finnish

Finnish inflection for **any** Finnish word, attested or not: nouns, names,
adjectives, pronouns and numerals in fourteen cases, two numbers and the
possessive forms; verbs in every mood, person, tense and polarity, the
negatives and perfects the phrases they are (`en tule`, `olen tullut`), and
their infinitives and participles; postpositions with their possessive
suffixes; adverbs in their degrees. Each answer says how sure the grammar is
and why. Vowel harmony is a learned sound law: an ending is written once
(`-ssA`) and the law chooses `a` or `ä` from the word's vowels. A verb of two
words inflects its first (`pyydän anteeksi`, `kiinni otettu`).

```rust
use finnish::*;

assert_eq!(noun("talo", Case::Inessive, Number::Singular), "talossa");
assert_eq!(noun("kylä", Case::Inessive, Number::Singular), "kylässä");
let me = Person::FirstSingular;
assert_eq!(indicative("tulla", Tense::Present, Polarity::Affirmative, me), "tulen");
assert_eq!(indicative("tulla", Tense::Perfect, Polarity::Negative, Person::ThirdSingular), "ei ole tullut");
assert_eq!(indicative("pyytää anteeksi", Tense::Present, Polarity::Affirmative, me), "pyydän anteeksi");
assert_eq!(postposition("alla", Suffix::FirstSingular), "allani");
```

## How it works

This crate is Finnish inflection as a grammar, learned by
[flexion](https://github.com/gold-silver-copper/flexion) from the English Wiktionary's
Finnish entries. Each part of speech is a module under `src/`, and has what its
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
| noun | 126530 | 101 | 22 | 4346 | 5039 | 8352 | 19 |
| pronoun | 75 | 11 | 2 | 16 | 61 | 406 | 29 |
| numeral | 75 | 23 | 0 | 9 | 48 | 271 | 139 |
| proper_noun | 9040 | 84 | 23 | 734 | 1138 | 1387 | 132 |
| adjective | 17353 | 71 | 11 | 636 | 594 | 662 | 18 |
| verb | 12219 | 102 | 27 | 443 | 652 | 5539 | 258 |
| adverb | 2152 | 14 | 0 | 65 | 103 | 69 | 1 |
| postposition | 41 | 6 | 1 | 8 | 7 | 4 | 0 |

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
| noun | 12869 | 1899864 | **98.07%** (97.9–98.3) | 99.28% | 99.29% | 95.48% | 0.010 | 0.629 | 98.07% | 99.34% | 98.96 / 98.99 / 99.11 |  | 98.08% (1899576) | 57.29% (288) | 98.29% (98.96%) | -0.22 (-0.41…-0.04) | 98.24% | 99.98% | 98.94% / 96.33% |  | 98.13% (12649) / 100.00% (7) / 94.30% (213) |  | 1.01% |
| pronoun | 9 | 187 | **reproduced 1484 of 1484**; a new word 16.51% (75 words) | 7.49% | 10.16% | 0.00% | 0.342 | 49.294 | 6.42% |  | 8.43 / 7.69 / 17.50 |  | 6.49% (185) | 0.00% (2) | 9.63% (0.00%) | -3.21 (-5.92…-0.99) | 6.42% | 100.00% | 0.00% / 8.76% |  | 7.89% (7) / 0.00% (1) / 0.00% (1) |  | 69.33% |
| numeral | 9 | 224 | **reproduced 1873 of 1873**; a new word 31.50% (75 words) | 44.20% | 44.20% | 22.22% | 0.288 | 47.482 | 32.14% |  | 54.42 / 54.37 / 54.31 |  | 32.14% (224) |  | 43.30% (0.00%) | -11.16 (-41.96…+19.73) | 32.14% | 100.00% | 0.00% / 36.18% |  | 23.62% (8) / – / 100.00% (1) |  | 44.00% |
| proper_noun | 917 | 119536 | **93.82%** (92.5–95.1) | 98.92% | 98.92% | 87.57% | 0.015 | 0.874 | 93.82% | 99.01% | 97.60 / 98.00 / 98.18 |  | 93.99% (119245) | 22.68% (291) | 91.77% (69.07%) | +2.05 (+0.72…+3.35) | 94.32% | 99.88% | 97.97% / 92.76% |  | 93.92% (911) / – / 75.61% (6) |  | 2.60% |
| adjective | 1721 | 44395 | **97.50%** (96.9–98.1) | 99.40% | 99.40% | 94.83% | 0.011 | 0.451 | 97.50% |  | 98.65 / 98.70 / 98.91 |  | 97.50% (44392) | 33.33% (3) | 97.41% (66.67%) | +0.08 (-0.58…+0.76) | 97.54% | 100.00% | 99.24% / 96.42% |  | 97.42% (1570) / 98.54% (104) / 97.81% (47) |  | 0.83% |
| verb | 1219 | 196574 | **98.50%** (98.0–98.9) | 99.39% | 99.39% | 93.44% | 0.003 | 0.475 | 98.50% | 99.47% | 99.02 / 99.03 / 99.04 |  | 98.50% (196426) | 97.97% (148) | 97.56% (2.70%) | +0.94 (+0.26…+1.69) | 98.50% | 99.93% | 99.43% / 98.27% |  | 98.50% (1103) / 100.00% (59) / 96.86% (57) | 98.97% (117603) | 3.97% |
| adverb | 211 | 421 | **96.67%** (94.5–98.6) | 98.81% | 98.81% | 94.31% | 0.017 | 0.691 | 96.67% | 98.57% | 98.57 / 0.00 / 0.00 |  | 96.67% (421) |  | 97.39% (0.00%) | -0.71 (-1.67…+0.00) | 96.67% | 100.00% | 100.00% / 96.47% |  | 96.67% (211) / – / – |  | 2.32% |
| postposition | 5 | 23 | **reproduced 199 of 199**; a new word 81.91% (41 words) | 91.30% | 91.30% | 60.00% | 0.177 | 4.182 | 65.22% |  | 83.33 / 92.31 / 87.50 |  | 65.22% (23) |  | 78.26% (0.00%) | -13.04 (-30.43…+0.00) | 65.22% | 100.00% | 0.00% / 65.22% |  | 65.22% (5) / – / – |  | 4.88% |

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
| noun | 36661 | 57% | 0% | 0% | 5% | 28% | 9% | 1% | 0% | 98.05% | 100.00% | 100.00% |
| pronoun | 175 | 4% | 0% | 0% | 14% | 82% | 0% | 0% | 0% | 6.42% | 100.00% | 100.00% |
| numeral | 152 | 18% | 0% | 0% | 27% | 55% | 0% | 0% | 0% | 32.14% | 100.00% | 100.00% |
| proper_noun | 7391 | 73% | 0% | 0% | 4% | 13% | 8% | 2% | 0% | 93.76% | 100.00% | 100.00% |
| adjective | 1112 | 75% | 0% | 0% | 2% | 21% | 2% | 0% | 0% | 97.42% | 100.00% | 100.00% |
| verb | 2945 | 54% | 0% | 0% | 0% | 41% | 0% | 5% | 0% | 98.33% | 100.00% | 100.00% |
| adverb | 14 | 64% | 0% | 0% | 14% | 21% | 0% | 0% | 0% | 96.67% | 100.00% | 100.00% |
| postposition | 8 | 75% | 0% | 0% | 0% | 25% | 0% | 0% | 0% | – | 100.00% | 100.00% |

*By block*, each group of cells' top-1 and held-out forms:

- noun: noun 98.03% (306074), possessive 98.08% (1593790)
- proper_noun: proper_noun 93.67% (19218), proper_noun_possessive 93.84% (100318)
- adjective: adjective 97.50% (42985), comparative 97.02% (705), superlative 97.73% (705)
- verb: indicative 98.42% (67986), conditional 98.10% (34008), imperative 98.94% (14556), perfect_imperative 99.45% (7272), potential 99.50% (33944), long_infinitive 99.13% (6068), second_infinitive 98.85% (2427), second_infinitive_possessive 99.21% (6068), second_infinitive_instructive 99.34% (1213), third_infinitive 96.37% (7270), third_infinitive_passive 99.09% (1206), fourth_infinitive 96.37% (1213), fifth_infinitive 96.37% (6059), participle 97.49% (7284)

## The shipped grammar and the scored one

The grammar shipped is learned from every word; the one scored above from all but the
held-out tenth. Their sound laws (each with its tier) and irregular forms, side by side;
a law one has and the other doesn't is listed below the table.

| | sound laws, shipped / scored | the same laws | irregular forms, shipped / scored |
|---|---:|---:|---:|
| noun | 22 / 22 | 15 | 8352 / 8711 |
| pronoun | 2 / 2 | 0 | 406 / 339 |
| numeral | 0 / 0 | 0 | 271 / 250 |
| proper_noun | 23 / 25 | 9 | 1387 / 1238 |
| adjective | 11 / 9 | 8 | 662 / 434 |
| verb | 27 / 21 | 17 | 5539 / 5133 |
| adverb | 0 / 0 | 0 | 69 / 65 |
| postposition | 1 / 1 | 1 | 4 / 5 |

- noun: only the shipped grammar's: `A → a after a, o, u; ä after y, ä, ö, none (the nearest of a, o, u, y, ä, ö before it, as written)`, `k → ∅ after [a|i|k|l|o|r|u|ä]; t → ∅ after t; t → d after [a|e|h|i|o|u|y|ä]; t → n after n; p → ∅ after p; p → v after [a|i|r|u|ä]; k → g after n; t → l after l; t → r after r; p → m after m in alternating stems (ˇ), before +[a|i|ä][k|l|t], +[a|i|ä]n$, +[a|i|ä]s.., +ei.$, +ei.[^a|e|i|o|u|y|ä|ö], +e[k|l|t], +es.., +o.[d|k|l|s|t], +o.n$, +ö.[d|h|k|l|s|t], e+i.[^a|e|i|o|u|y|ä|ö], e+[k|l], e+s.., [o|u|ö]+i.$, [o|u|ö]+i.[^a|e|i|o|u|y|ä|ö], [o|u|ö]+[k|l|t], [o|u|ö]+n$, [o|u|ö]+s.., y+i.[^a|e|i|o|u|y|ä|ö], y+[k|l|t], y+s..`, `∅ → t after t in alternating stems (ˇ), before [a|e]+[a|e|i|o|u|y|ä|ö], i+.[a|e|i|o|u|y|ä|ö]`, `[e|ä|a]+ii → +isii`, `de+i → +tei`, `[e|ä]+iä → +itä`, `r+d → r+r`. Only the scored grammar's: `A → ä after -, y, ä, ö, none; a after a, o, u (the nearest of -, a, o, u, y, ä, ö before it, as written)`, `k → ∅ after [a|e|i|k|l|r|u|ä]; t → ∅ after t; t → d after [a|e|h|i|o|u|ä]; t → n after n; p → ∅ after p; p → v after [a|i|r|u|ä]; k → g after n; t → l after l; t → r after r; p → m after m in alternating stems (ˇ), before +[a|i|ä][k|l|t], +[a|i|ä]n$, +[a|i|ä]s.., +ei.$, +ei.[^a|e|i|o|u|y|ä|ö], +e[k|l|t], +es.., +o.[d|k|l|s|t], +o.n$, +ö.[d|h|k|l|s|t], e+i.[^a|e|i|o|u|y|ä|ö], e+[k|l], e+s.., [o|u|ö]+i.$, [o|u|ö]+i.[^a|e|i|o|u|y|ä|ö], [o|u|ö]+[k|l|t], [o|u|ö]+n$, [o|u|ö]+s.., y+i.[^a|e|i|o|u|y|ä|ö], y+[k|l|t], y+s..`, `[e|a]+ii → +isii`, `∅ → t after t in alternating stems (ˇ), before [a|e]+[a|e|i|o|u|y|ä|ö], i+.e, i+.i.`, `de+id → +teid`, `de+it → +teit`, `e+iä → e+itä`.
- pronoun: only the shipped grammar's: `A → a after o, u; ä after none (the nearest of o, u before it, as written)`, `V → that letter; a after none (the nearest of a, e, i, o, u, y, ä, ö before it, as written)`. Only the scored grammar's: `A → a after o, u; ä after ä, none (the nearest of o, u, ä before it, as written)`, `E → ä after e; e after o; u after none (the nearest of e, o before it, as written)`.
- proper_noun: only the shipped grammar's: `A → a after a, o, u, å; ä after y, ä, ö, none (the nearest of a, o, u, y, ä, å, ö before it, as written)`, `I → i after -, c, i, none; e after p, ä (the nearest of -, c, i, p, ä before it, as written)`, `E → that letter; a after none (the nearest of a, e, i, m, ä before it, as written)`, `k → ∅ after [k|l|o|ä]; t → d after [e|h|i|o|u]; t → n after n; p → ∅ after p; k → g after n; t → r after r; p → m after m; p → v after [a|r]; t → l after l in alternating stems (ˇ), before +[a|i|ä][k|l|t], +[a|i|ä]n$, +[a|i|ä]s.., +ei.$, +ei.[^a|e|i|o|u|y|ä|å|ö], +e[k|l|t], +en$, +es.., +o.[d|k|l|s|t], +o.n$, +ö.[d|h|k|l|s|t], i+[k|l|t], i+s.., [o|u|ö]+i.$, [o|u|ö]+i.[^a|e|i|o|u|y|ä|å|ö], [o|u|ö]+[k|l|t], [o|u|ö]+n$, [o|u|ö]+s..`, `[n|r|t|l|k|d|s|m|v|g|h|š|x|f|c|b|ž|j]+ks → +iks`, `[n|r|t|l|k|d|s|m|v|g|h|š|x|f|c|b|ž]+mm → +imm`, `[n|r|t|l|k|d|s|m|v|g|h|š|x|f|c|b|ž]+nn → +inn`, `[n|r|t|l|k|d|s|m|v|g|h|š|x|f|c|b|ž]+ns → +ins`, `g → k after n; n → t after n in alternating stems (ˇ), before .+[a|e|i|o|u|y|ä|å|ö]`, `[ni|si|ti|mi|li]+h → +`, `iv+ea → iv+eä`, `[ji|vi]+h → +`, `∅ → p after p in alternating stems (ˇ), before a+[a|e|i|o|u|y|ä|å|ö]`, `[r|n|d|l|s|p|m|h|g]+ei → +ii before [d|m]`. Only the scored grammar's: `A → a after a, o, u; ä after y, ä, ö, none (the nearest of a, o, u, y, ä, ö before it, as written)`, `I → i after c, i, w, none; e after p, ä (the nearest of c, i, p, w, ä before it, as written)`, `k → ∅ after [k|l|o|ä]; t → d after [e|h|i|o|u]; t → n after n; p → ∅ after p; k → g after n; t → r after r; p → m after m; t → l after l; p → v after r in alternating stems (ˇ), before +[a|i|ä][k|l|t], +[a|i|ä]n$, +[a|i|ä]s.., +ei.$, +ei.[^a|e|i|o|u|y|ä|å|ö], +e[k|l|t], +en$, +es.., +o.[d|k|l|s|t], +o.n$, i+[k|l|t], i+s.., [o|u|ö]+i.$, [o|u|ö]+i.[^a|e|i|o|u|y|ä|å|ö], [o|u|ö]+[k|l|t], [o|u|ö]+n$, [o|u|ö]+s..`, `[n|r|t|l|k|d|m|s|v|g|h|š|x|f|b|ž|j|c]+ks → +iks`, `[n|r|t|l|k|d|m|s|v|g|h|š|x|f|b|ž|j|c]+ll → +ill`, `[n|r|t|l|k|d|m|s|v|g|h|š|x|f|b|ž|c]+mm → +imm`, `[n|r|t|l|k|d|m|s|v|g|h|š|x|f|b|ž|c]+nn → +inn`, `[n|r|t|l|k|d|m|s|v|g|h|š|x|f|b|ž|j|c]+lt → +ilt`, `[n|r|t|l|k|d|m|s|v|g|h|š|x|f|b|ž|c]+ns → +ins`, `ti+h → ti+`, `n → t after n in alternating stems (ˇ), before .+[a|e|i|o|u|y|ä|å|ö]`, `oo+a → oo+ta after a vowel earlier in the stem`, `oo+o → oo+see`, `[n|r|v|k|t|d|s|x|g|f|b]+o → +ii before [m|s]`, `on+a → on+ia after [b|e|l|m|n|r|s|t]`, `[r|n|d|l|p|m|h|g]+ei → +ii before [d|m]`.
- adjective: only the shipped grammar's: `t → ∅ after t; t → l after l; p → ∅ after p in alternating stems (ˇ), before +[a|i][^a|e|i|o|u|y|ä|ö]$, +[a|i][^a|e|i|o|u|y|ä|ö].., +e.[^a|e|i|o|u|y|ä|ö]$, +e.[^a|e|i|o|u|y|ä|ö].[a|e|i|o|u|y|ä|ö], +o...[a|e|i|o|u|y|ä|ö], .+i.$, .+i..[a|e|i|o|u|y|ä|ö], .+[k|l|m|s|t], .+n$`, `i+e → +e`, `kä+i → k+i after t`. Only the scored grammar's: `t → ∅ after t; t → d after [h|i|ä]; t → l after l; p → ∅ after p in alternating stems (ˇ), before +[a|i][^a|e|i|o|u|y|ä|ö]$, +[a|i][^a|e|i|o|u|y|ä|ö].., +e.[^a|e|i|o|u|y|ä|ö]$, +e.[^a|e|i|o|u|y|ä|ö].[a|e|i|o|u|y|ä|ö], +o...[a|e|i|o|u|y|ä|ö], .+i.$, .+i..[a|e|i|o|u|y|ä|ö], .+[k|l|m|s|t], .+n$`.
- verb: only the shipped grammar's: `t → ∅ after t; t → d after [a|h|i|o|u|y|ä]; t → n after n; t → r after r; t → l after l; p → v after [i|o|r]; k → g after n in alternating stems (ˇ), before +[a|ä]$, +[a|ä][^a|e|i|o|u|y|ä|ö]$, +[a|ä][^a|e|i|o|u|y|ä|ö][^a|e|i|o|u|y|ä|ö], +e, +i.$, +i..[a|e|i|o|u|y|ä|ö], i+m.[a|e|i|o|u|y|ä|ö]$, i+n$, i+t, [u|y]+$, [u|y]+i.$, [u|y]+i..[a|e|i|o|u|y|ä|ö], [u|y]+m.[a|e|i|o|u|y|ä|ö]$, [u|y]+n$, [u|y]+t`, `∅ → k after [a|i|k|l|o|r|u] in alternating stems (ˇ), before e+[a|s|ä], e+n[a|e|i|o|u|y|ä|ö], ä+.$, ä+.[i|m|s|v], ä+.n$, ä+.t$, ä+.t[^a|e|i|o|u|y|ä|ö]`, `d → t after [a|h|i|u]; n → t after n; r → t after r; l → t after l; j → k after [l|r]; v → p after [a|i|l]; g → k after n; m → p after m in alternating stems (ˇ), before .+[a|i|s|ä], .+n[a|e|i|o|u|y|ä|ö], ..+[a|e|i|o|u|y|ä|ö]`, `k → ∅ after [a|i|k|l|r|ä]; p → m after m in alternating stems (ˇ), before a+.nu$, i+m.[a|e|i|o|u|y|ä|ö]$, i+n$, i+t, [o|u]+$, [o|u]+i.$, [o|u]+i..[a|e|i|o|u|y|ä|ö], [o|u]+m.[a|e|i|o|u|y|ä|ö]$, [o|u]+n$, [o|u]+t$, [o|u]+t.e, y+$, y+i.$, y+i..[a|e|i|o|u|y|ä|ö], y+m.[a|e|i|o|u|y|ä|ö]$, y+n$, y+t`, `aa+a → a+kaa`, `aa+i → a+kai`, `aa+s → a+kas`, `o+a → +koa after [a|i]`, `o+s → +kos after [a|i]`, `pi+ → +i before ˇ`. Only the scored grammar's: `t → ∅ after t; t → d after [a|h|i|o|u|y|ä]; t → n after n; t → r after r; t → l after l; p → v after [i|o|r]; k → g after n in alternating stems (ˇ), before +[a|ä]$, +[a|ä][^a|e|i|o|u|y|ä|ö]$, +[a|ä][^a|e|i|o|u|y|ä|ö][^a|e|i|o|u|y|ä|ö], +e, +i.$, +i..[a|e|i|o|u|y|ä|ö], i+[^a|e|i|o|u|y|ä|ö]$, i+[^a|e|i|o|u|y|ä|ö][^a|e|i|o|u|y|ä|ö], [u|y]+$, [u|y]+i.$, [u|y]+i..[a|e|i|o|u|y|ä|ö], [u|y]+m.[a|e|i|o|u|y|ä|ö]$, [u|y]+n$, [u|y]+t`, `∅ → k after [a|i|k|o|r|u] in alternating stems (ˇ), before e+[a|s|ä], e+n[a|e|i|o|u|y|ä|ö], ä+.$, ä+.[i|m|s|v], ä+.n$, ä+.t$, ä+.t[^a|e|i|o|u|y|ä|ö]`, `d → t after [a|h|i|u]; n → t after n; r → t after r; l → t after l; j → k after [l|r]; g → k after n; m → p after m; v → p after a in alternating stems (ˇ), before .+[a|i|s|ä], .+n[a|e|i|o|u|y|ä|ö], ..+[a|e|i|o|u|y|ä|ö]`, `ko+t → +ot after [a|k]`.

| | lines of grammar | bytes |
|---|---:|---:|
| noun | 13187 | 745092 |
| pronoun | 1093 | 41609 |
| numeral | 1507 | 58931 |
| proper_noun | 8161 | 397307 |
| adjective | 4547 | 189532 |
| verb | 9909 | 604459 |
| adverb | 616 | 22455 |
| postposition | 430 | 15976 |

## Data and license

The code is MIT or Apache-2.0. The grammar under `src/` is derived from Wiktionary via
[Kaikki](https://kaikki.org/) and [wiktextract](https://github.com/tatuylonen/wiktextract),
and is licensed under [CC BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/); see
`DATA-LICENSE.md`.

## Regenerating

```sh
cd flexion
cargo xtask extract finnish
cargo xtask build finnish
```
