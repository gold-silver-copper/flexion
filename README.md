# flexion

Point a trainer at a language's [Kaikki](https://kaikki.org/) Wiktionary dump and get a
Rust crate that **is that language's inflectional grammar**: its declensions and
conjugations as named tables, the rules that choose them with their evidence, its sound
laws, its accent system, its irregular words' principal parts. The crate inflects **any** word, attested or
not, and every answer comes with a calibrated probability, how settled it is, and why.

```rust
use russian::*;

assert_eq!(noun("книга", Case::Genitive, Number::Plural), "книг");
// A word the dictionary doesn't have: every candidate, how likely, and why.
for c in Noun::new("бокр").candidates((Case::Accusative, Number::Singular)) {
    println!("{c}");
}
// бокр (61%, regular): like приём; 5031 of 5179 words in -р
// бокра (39%, regular): like приём; 5031 of 5179 words in -р
// (inanimate, or animate: say which with `.animacy(…)`, and the guess is gone)
```

Twelve languages are built from the same trainer, with nothing language-specific in the
trainer or the runtime: [English](languages/english), [Russian](languages/russian),
[Latin](languages/latin), [Ancient Greek](languages/ancient-greek),
[Sanskrit](languages/sanskrit), [Polish](languages/polish), [German](languages/german),
[Spanish](languages/spanish), [French](languages/french), [Italian](languages/italian),
[Finnish](languages/finnish) and [Turkish](languages/turkish). Each is a config in
`crates/flexion-configs`; `cargo xtask build german` rebuilds one.

## How well it does

Held-out words: a tenth of each part of speech (by a hash of the word) that the grammar
was not learned from, asked for every form the dictionary lists; the lemma's own form only
where it isn't what was typed (Latin `rex` asks for `rēx`). *Top-1 from the headword*: the
word typed as the dictionary's headword (Latin `rosa`, Greek `λύω`, no vowel length), with
its 95% interval (resampling words). *Lemma in full*: typed with every mark the tables
write. *Principal parts given*: the dictionary's principal parts given too. *Features
given*: the word's gender (Russian: and animacy; verbs: aspect) given, as `.gender(…)`
gives it. *Floor*: the exact non-neural baseline of the CoNLL–SIGMORPHON 2017 shared task
(letters only), learned from the same words as the grammar. *Against the floor*: the
grammar's letters right less the floor's, word by word on the same forms, with its 95%
interval. *On a training word / new root*: top-1 of the held-out words that are a training
word with a prefix or a compound's front, and of the rest. *Any listed form*: top-1 where
any form the dictionary lists in the cell counts. *Homograph ceiling*: the most any
grammar could get right (words typed alike get one answer per cell). *Lemma spelling*:
top-1 on the lemma's form where it isn't what was typed. *Own / form-of / alt-of*: top-1
of the words with an entry of their own, of those whose entry is a form of another word (a
participle, a comparative), and of spelling variants. *Phrases*: top-1 on the forms the
dictionary writes with a space (Latin `amātus sum`). *Irregular lines*: the share of the
training words the grammar lists a form or a stem of. This tenth was looked at while the
trainer was developed, up to PR #15; since PR #16 changes are judged on others (`cargo
xtask try` scores tenth 2, `--final` this one), and each PR's changes are confirmed once
on a tenth no decision looked at.

| language | part of speech | top-1 from the headword | lemma in full | principal parts given | features given | floor | against the floor | on a training word / new root | any listed form | homograph ceiling | lemma spelling | own / form-of / alt-of | phrases | irregular lines | calibration error |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| english | noun | **97.17%** (96.9–97.4) | 97.17% |  |  | 96.78% | +0.39 (+0.24…+0.55) | 98.68% / 95.73% | 97.98% | 99.88% |  | 97.34% (19552) / 90.00% (60) / 96.50% (4120) |  | 0.11% | 0.003 |
| english | verb | **97.17%** (96.8–97.6) | 97.17% | 99.54% |  | 97.79% | -0.62 (-0.93…-0.33) | 98.92% / 96.23% | 97.61% | 99.86% |  | 97.14% (3598) / 100.00% (3) / 97.36% (550) |  | 1.01% | 0.006 |
| english | adjective | **95.35%** (94.8–95.9) | 95.35% |  |  | 92.53% | +2.82 (+2.39…+3.25) | 99.05% / 93.61% | 96.30% | 99.94% |  | 95.59% (5574) / 75.00% (4) / 93.48% (661) | 99.06% (11894) | 0.06% | 0.005 |
| english | adverb | **99.01%** (98.4–99.5) | 99.01% |  |  | 98.34% | +0.66 (+0.25…+1.16) | 100.00% / 98.67% | 99.59% | 100.00% |  | 98.95% (1146) / – / 100.00% (61) | 99.67% (2398) | 0.11% | 0.005 |
| russian | noun | **94.64%** (94.0–95.2) | 94.64% | 95.81% | 97.30% | 94.72% | -0.08 (-0.52…+0.36) | 97.06% / 93.82% | 94.99% | 99.82% |  | 94.56% (2394) / 96.55% (181) / 90.08% (36) |  | 0.53% | 0.007 |
| russian | adjective | **99.75%** (99.6–99.9) | 99.75% |  |  | 99.76% | -0.01 (-0.12…+0.09) | 99.71% / 99.77% | 99.75% | 99.94% |  | 99.11% (950) / 100.00% (2512) / 100.00% (4) |  | 0.07% | 0.002 |
| russian | verb | **95.76%** (95.0–96.4) | 95.76% | 98.88% | 95.49% | 96.65% | -0.89 (-1.39…-0.40) | 95.84% / 95.72% | 95.89% | 99.49% |  | 95.74% (1269) / 100.00% (29) / 81.44% (6) |  | 0.75% | 0.006 |
| latin | noun | **70.07%** (67.8–72.3) | 92.93% | 98.70% | 70.95% | 59.09% | +10.98 (+8.20…+13.69) | 76.53% / 68.08% | 70.12% | 99.46% | 70.53% (984) | 70.26% (1379) / 72.73% (19) / 68.17% (167) |  | 1.16% | 0.025 |
| latin | adjective | **87.26%** (86.1–88.4) | 99.23% | 99.48% |  | 71.87% | +15.40 (+13.68…+17.15) | 90.01% / 85.54% | 87.36% | 99.60% | 86.75% (2151) | 75.63% (981) / 94.27% (1684) / 84.87% (176) |  | 0.48% | 0.016 |
| latin | verb | **76.85%** (73.7–80.1) | 84.92% | 98.59% |  | 64.17% | +12.68 (+8.60…+16.87) | 84.24% / 73.67% | 76.97% | 98.43% | 88.85% (592) | 77.52% (541) / – / 72.34% (78) | 74.02% (16157) | 4.69% | 0.041 |
| ancient-greek | noun | **71.90%** (68.9–74.9) | 88.36% | 95.17% | 72.81% | 51.28% | +24.36 (+20.33…+28.12) | 79.56% / 70.11% | 71.92% | 99.55% | 76.45% (586) | 71.60% (784) / 100.00% (3) / 78.46% (22) |  | 8.62% | 0.056 |
| ancient-greek | adjective | **77.29%** (74.0–80.6) | 90.48% | 97.88% |  | 52.46% | +25.92 (+21.48…+30.22) | 80.30% / 76.70% | 77.49% | 99.17% | 83.06% (301) | 78.36% (349) / 74.75% (115) / 68.31% (10) |  | 2.91% | 0.046 |
| ancient-greek | verb | **65.45%** (60.7–70.1) | 74.89% | 80.52% |  | 44.20% | +22.59 (+16.63…+28.50) | 73.49% / 60.72% | 65.82% | 99.90% | 78.12% (160) | 65.96% (244) / 45.83% (3) / 53.40% (10) | 56.62% (4832) | 52.39% | 0.057 |
| sanskrit | noun | **92.10%** (90.8–93.4) | 92.10% |  | 97.30% | 91.70% | +0.40 (-0.73…+1.48) | 94.30% / 92.03% | 92.65% | 97.83% |  | 92.14% (563) / 100.00% (2) / 88.71% (11) |  | 1.57% | 0.015 |
| sanskrit | adjective | **91.91%** (89.6–94.0) | 91.91% |  |  | 89.95% | +1.96 (+0.32…+3.76) | 83.33% / 92.30% | 91.94% | 100.00% |  | 91.60% (183) / 100.00% (6) / 100.00% (1) |  | 3.87% | 0.020 |
| sanskrit | verb | **89.86%** (85.3–94.0) | 89.86% | 90.58% |  | 89.93% | -0.07 (-2.32…+2.05) | 95.65% / 89.31% | 90.37% | 100.00% | 100.00% (2) | 91.49% (71) / 55.52% (17) / – |  | 24.23% | 0.045 |
| polish | noun | **93.42%** (92.9–93.9) | 93.42% | 95.79% | 96.25% | 93.34% | +0.08 (-0.29…+0.44) | 95.10% / 93.10% | 93.70% | 99.23% |  | 92.44% (3110) / 98.91% (809) / 88.39% (62) |  | 1.81% | 0.003 |
| polish | adjective | **99.92%** (99.9–99.9) | 99.92% |  |  | 99.99% | -0.07 (-0.09…-0.04) | 99.94% / 99.91% | 99.92% | 100.00% |  | 99.90% (1147) / 100.00% (239) / 100.00% (8) |  | 0.36% | 0.001 |
| polish | verb | **97.14%** (96.4–97.8) | 97.14% | 98.24% | 97.27% | 97.62% | -0.48 (-0.99…+0.02) | 95.62% / 97.48% | 97.43% | 99.77% |  | 97.17% (1233) / 100.00% (2) / 93.03% (11) |  | 1.39% | 0.009 |
| german | noun | **93.63%** (93.2–94.1) | 93.63% | 99.79% | 94.05% | 94.77% | -1.15 (-1.46…-0.85) | 96.70% / 89.92% | 94.43% | 99.69% |  | 93.44% (5249) / 98.79% (730) / 87.78% (344) |  | 0.51% | 0.007 |
| german | adjective | **98.79%** (98.2–99.3) | 98.79% | 99.88% |  | 99.02% | -0.23 (-0.73…+0.26) | 98.61% / 98.87% | 99.06% | 100.00% |  | 98.73% (1410) / 100.00% (28) / 100.00% (49) | 96.53% (548) | 0.11% | 0.005 |
| german | verb | **95.17%** (94.2–96.1) | 95.17% | 95.88% | 95.28% | 73.16% | +22.01 (+20.25…+23.80) | 95.91% / 94.35% | 95.27% | 99.25% |  | 95.00% (936) / – / 99.83% (34) | 91.76% (9057) | 5.12% | 0.012 |
| spanish | noun | **98.67%** (98.4–98.9) | 98.67% |  | 98.67% | 98.67% | +0.00 (-0.20…+0.20) | 99.85% / 98.12% | 98.88% | 99.98% |  | 98.74% (5245) / 100.00% (583) / 94.02% (251) |  | 0.11% | 0.003 |
| spanish | adjective | **98.50%** (98.1–98.9) | 98.50% | 99.67% |  | 98.60% | -0.10 (-0.35…+0.14) | 99.84% / 98.04% | 98.61% | 100.00% |  | 98.49% (2284) / 100.00% (68) / 97.12% (47) |  | 0.22% | 0.003 |
| spanish | verb | **98.97%** (98.6–99.3) | 98.97% | 99.73% |  | 99.28% | -0.31 (-0.60…-0.02) | 99.28% / 98.77% | 98.99% | 99.98% |  | 99.02% (879) / 100.00% (4) / 97.86% (46) |  | 1.73% | 0.004 |
| french | noun | **97.59%** (97.1–98.0) | 97.59% |  | 97.59% | 97.03% | +0.56 (+0.23…+0.90) | 96.88% / 97.85% | 97.98% | 99.88% |  | 97.97% (3898) / 99.18% (245) / 86.78% (174) |  | 0.65% | 0.004 |
| french | adjective | **97.50%** (96.9–98.1) | 97.50% | 99.95% |  | 97.72% | -0.22 (-0.65…+0.24) | 98.85% / 97.12% | 97.57% | 100.00% |  | 97.90% (1781) / – / 85.06% (58) |  | 0.28% | 0.009 |
| french | verb | **98.70%** (98.1–99.3) | 98.70% | 99.48% |  | 99.28% | -0.59 (-1.06…-0.22) | 99.63% / 98.40% | 98.84% | 100.00% |  | 98.74% (724) / – / 97.08% (19) |  | 0.96% | 0.004 |
| french | participle | **99.84%** (99.5–100.0) | 99.84% |  |  | 99.52% | +0.32 (-0.32…+0.96) | 100.00% / 99.80% | 99.84% | 100.00% |  | 100.00% (1) / 99.84% (626) / – |  | 0.03% | 0.005 |
| italian | noun | **95.82%** (95.3–96.3) | 95.82% |  | 96.39% | 95.09% | +0.74 (+0.31…+1.16) | 95.91% / 95.76% | 95.96% | 99.77% |  | 95.85% (5862) / 98.57% (210) / 93.44% (320) |  | 0.19% | 0.003 |
| italian | adjective | **98.46%** (98.0–98.8) | 98.46% | 99.78% |  | 98.46% | +0.00 (-0.28…+0.28) | 99.36% / 97.96% | 98.49% | 100.00% |  | 98.48% (2514) / 100.00% (42) / 96.86% (75) |  | 0.19% | 0.010 |
| italian | verb | **99.05%** (98.7–99.4) | 99.05% | 99.56% | 99.05% | 99.22% | -0.17 (-0.48…+0.13) | 98.93% / 99.12% | 99.12% | 99.96% |  | 99.34% (1188) / 100.00% (9) / 95.12% (89) |  | 1.77% | 0.005 |
| italian | participle | **100.00%** (100.0–100.0) | 100.00% |  |  | 100.00% | +0.00 (+0.00…+0.00) | 100.00% / 100.00% | 100.00% | 100.00% |  | 100.00% (11) / 100.00% (810) / – |  | 0.02% | 0.000 |
| finnish | noun | **97.81%** (97.6–98.0) | 97.81% | 99.15% |  | 98.29% | -0.48 (-0.68…-0.29) | 98.56% / 96.30% | 97.98% | 99.98% |  | 97.87% (12649) / 100.00% (7) / 94.16% (213) |  | 1.34% | 0.009 |
| finnish | adjective | **97.45%** (96.9–98.0) | 97.45% |  |  | 97.41% | +0.04 (-0.62…+0.72) | 99.15% / 96.40% | 97.52% | 100.00% |  | 97.37% (1570) / 98.54% (104) / 97.81% (47) |  | 0.90% | 0.010 |
| turkish | noun | **96.51%** (95.5–97.4) | 96.51% | 99.52% |  | 93.27% | +3.23 (+2.30…+4.24) | 97.63% / 96.39% | 96.55% | 99.88% |  | 96.17% (977) / 100.00% (72) / 99.51% (16) |  | 1.11% | 0.012 |

Where the grammar is weak or below the floor, and why:

- **Latin, from the headword.** A headword doesn't write vowel length (`fox` for `fōx`);
  the grammar restores it by rules on the letters around each vowel (`-ātiō`, `dē-`, a
  diphthong), learned from every part of speech's words and reading which part a word is
  (`quīngēnī`, a numeral; `indigenus`, an adjective), and reads the three likeliest
  spellings. But a stem's own lengths are lexical, so a root whose training words (two or
  more) all write a vowel one way the rules don't is listed and read first (`ROOTS` in
  `marks.rs`): on the development tenth nouns +2.0 points, adjectives +2.9, verbs +0.7, and
  +2.0, +2.6, +1.4 on a tenth no decision looked at. A root no other word shares stays a
  guess. Typed in full the same grammars reach 92.9% (nouns), 99.2% (adjectives), 84.9%
  (verbs), and with the principal parts 98.6–99.5%. The perfect passive is the phrase it
  is (`amātus sum`, `amātum esse`, `hortātus sum`), 74.0% right as typed from the headword.
- **Ancient Greek.** A circumflex on a diphthong's second letter (`ἀριθμοῦ`) was read as
  vowel length, which split declensions in two; read as the diphthong's, nouns went from
  133 to 87 declensions and +3.8 points, adjectives +3.1 (development tenth). The augment
  (a law where the form's first letters meet the stem: `ἐ‹ἀ → ἠ`), the perfect's
  reduplication (`%ε~κα`) and the augment behind a preverb (`ἀπο + ἐ → ἀπε`) are laws.
  The perfect middle-passive subjunctive and optative are phrases (`λελυμένος ὦ`), each
  word keeping its own accent. What's left in the verbs is mostly stems: a verb's aorist
  or perfect stem (`ἀγγέλλω` → `ἠγγέλθην`) follows from no ending, so a new verb's is a
  guess; known verbs list it with the ending table it takes, and a principal part given
  fixes its stem (80.5%).
- **Sanskrit.** The `n` → `ṇ` rule is a law (`तर्काणाम्`), and a noun's gender, given,
  takes it from 92.1% to 97.3%. The verbs are small (a tenth is ~300 words).
- **Polish nouns** lost Middle Polish (`pl-mid-decl-noun`) tables, which aren't the modern
  language's: on the development tenth −0.06 from the headword and −0.14 with the
  principal parts, the one loss this change was kept with.
- **Below the floor, beyond chance**, now that the floor learns from the same words as
  the grammar (every edition's, not the main dump's only): German nouns (−1.15), Russian
  verbs (−0.89), English verbs (−0.62), French verbs (−0.59), Finnish nouns (−0.48),
  Spanish verbs (−0.31), Polish adjectives (−0.07); Spanish adjectives (−0.36 before) are
  now within chance of it (−0.10). The grammar's own top-1 on these rows is what it was
  (Finnish nouns −0.08 on this tenth, +0.24 on the development tenth);
  the floor rose with its words (German nouns 93.88% → 94.77%, Russian verbs 95.92% →
  96.65%). It isn't the declension chosen by the ending: a classifier reading endings up
  to eight letters, and a short word whole, chooses no better by a point on German and
  Finnish nouns, English and French verbs or Spanish adjectives (German nouns 87.2% of
  words against the rules' 89.1%, Spanish adjectives 99.2% against 98.7%; development
  tenth), and building tables from 8 or 16 readings instead of 4 changes top-1 by at most
  0.02. Of 200 wrong German plurals, two in
  three are the plural class chosen wrong (a loanword's `-s`, `-ismus` → `-ismen`), one in
  fourteen umlaut.
- **Russian** has no stress any more (stripped from input, never written).

## The generated code

A language crate has a module per part of speech. Excerpts from `russian`:

A paradigm: named after its commonest word, with the gender its words share, the stem
slots it builds through a shared ending table (`stems.rs`), then the cells that differ
from them, `~` for the stem, `derived` for a cell it takes from another. A paradigm
*like* another writes only what differs (`_` for the rest):

```rust
    // Приём: 37,690 words (указ, зонт, кот, зонтик, бортник, бочаг).
    declension!("приём", 37690, "~", [gender: Masculine];
    stem pl: "~- like приём";
    stem rest: "~- like приём";
    nom: "~", _;
    noun_second_case: derived, _;
    ),
    // Полнота: 12,546 words (собака, буява, былица, масленица, царица, вата). Like Приём, except in 2 cells: gen_ pl, noun_second_case loc.
    declension!("полнота", 12546, "~а", [gender: Feminine], like Приём;
    stem lemma: "~- like полнота";
    stem rest: "~- like полнота";
    gen_: _, "~";
    noun_second_case: _, derived;
    ),
```

An ending table: a slot's cells, in order, built on one stem. From `latin`, the third
declension's, which the consonant stems take (`stem rest: "~- like ratiō"`), and a word
that lists its stem with it (`irregular.rs`: `anas`, `anatis`… as the dictionary
writes it, `anit-`):

```rust
    // rest like ratiō: 4,121 words.
    EndingTable {
        name: Cow::Borrowed("ratiō"),
        words: 4121,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~ēs\t",
            "~is\t~um\t",
            "~ī\t~ibus\t",
            "~em\t~ēs\t",
            "~e\t~ibus\t",
            "~ēs\t",
            "\t\t",
        )),
    },
```

```text
anas	rest	anit- like ratiō
```

An analytic form is the phrase it is, learned like any other form: a Latin deponent's
perfect system (`modulātus sum` … `modulātum esse`), the participle agreeing in number:

```rust
    // rest_ like modulor: 561 words.
        table: Cow::Borrowed(concat!(
            "~us sum\t~ī sumus\t",
            "~us es\t~ī estis\t",
            "~us est\t~ī sunt\t",
            ...
            "~um esse\t",
```

Where a `derived` cell comes from (`relations.rs`): the accusative is the genitive or the
nominative, by the word's animacy. From `latin`, a relation that changes the form, with
the rules on its ending that choose the change (`0`, `1`, `2`: the changes in order):

```rust
    relation!(ACC_SG, [GEN_SG, NOM_SG], [], none, false, None),

    relation!(IND_ACT_PERF_2_SG, [IND_ACT_PERF_1_SG], [("~iī", "~īstī"), ("~sum", "~es"), ("~ī", "~istī")], rules {
        "" => [2 4877, 1 563, 0 30],
        "m" => [1 563],
        "um" => [1 563],
        "ī" => [2 4877, 0 30],
        "iī" => [0 30, 2 14],
```

A cell no word (or too few) has is still built: by analogy with a pair of cells that
differ the same way. Greek has no future subjunctive; it is made from the future
indicative as the perfect subjunctive is from the perfect indicative:

```rust
    // FUTURE_SUBJ_ACT_1_SG, from FUTURE_IND_ACT_1_SG. Few or no words have it: changed as PERFECT_IND_ACT_1_SG is into PERFECT_SUBJ_ACT_1_SG.
    relation!(FUTURE_SUBJ_ACT_1_SG, [FUTURE_IND_ACT_1_SG], [("~ᾰ", "~ω")], rules {
        "" => [0 1124],
    } listed {}, true, Some((PERFECT_IND_ACT_1_SG, PERFECT_SUBJ_ACT_1_SG))),
```

Sound laws (`laws.rs`) are written in `sound-law`'s notation (`"target" => "replacement"
/ "before" _ "after"`) and compiled when the crate is built, each with the words whose
forms it gives and the forms it gets wrong (those are listed). One law stated once:
Russian `ы` → `и` after a velar or a hushing letter, a captured letter (`{x}`) kept:

```rust
        // к+ы → к+и: 9,373 words (собака, зонтик, бортник, бредник), 5 forms wrong.
        law0: "к+ы" => "к+и";
        // [щ|ц|ч|ш|ж|к|г|х]+ям → +ам: 84 words (ночь, помочь, печь, рожь), 6 forms wrong.
        law6: "{x:[щ|ц|ч|ш|ж|к|г|х]}+ям" => "{x}+ам";
```

Sanskrit `n` → `ṇ`, with what may come between and what may not follow:

```rust
        // न → ण after र, ष, ृ unless च, ज, ट, ठ, ड, ढ, ण, त, थ, द, ध, न, ल, श, स comes between, not before ्: 1,181 words (वृक्ष, समुद्र, तरंग, तीर), 1 forms wrong.
        law0: "न" => "ण" / "{t:[र|ष|ृ]}{a:![र|ष|ृ|च|ज|ट|ठ|ड|ढ|ण|त|थ|द|ध|न|ल|श|स]*}+{b:![र|ष|ृ|च|ज|ट|ठ|ड|ढ|ण|त|थ|द|ध|न|ल|श|स]*}" _ "" unless "" _ "{[्]}";
```

The start of a word too. From `ancient-greek`: the perfect copies the stem's first letter
(`%`), the augment is a law where the form's first letters meet the stem (`‹`), its
lengthening a map, and a preverb meets the augment as the dictionary's compounds show:

```rust
    maps {
        law0 = ["ἀ" => "ἠ", "ᾰ" => "η", "ᾱ" => "η", "ᾳ" => "η"];
    ...
        // ἐ‹ἀ → ἠ (ᾰ → η, ᾱ → η, ᾳ → η): 88 words (ἀγω, ἀγᾰπᾰω, ἀκουω, ἀγᾰθοποιεω), 0 forms wrong.
        law0: "^ἐ‹{x:[ἀ|ᾰ|ᾱ|ᾳ]}" => "{x|law0}";

    perfect_ind_act_1: "%ε~κᾰ", _, _;
    perfect_subj_pass_1: "%ε~γμένος ὦ", _, "%ε~γμένοι ὦμεν";

        ), // ανα + ἐ → ἀνε
```

The rules choosing a declension, each with its evidence (how many of the dictionary's
words with that ending take each declension):

```rust
        "ба" => [Полнота 241, Какао 10, Люлька 4, Порубка 1] regular,
        "ьба" => [Полнота 32, Люлька 4, Порубка 1] regular,
        "дьба" => [Полнота 6, Люлька 4] regular,
```

From `latin`, how a vowel typed without its macron is written, by the letter before it
and the letters after it (`#` is the word's start or end), and, read first, a root whose
words all write a vowel one way the rules don't (`ROOTS`):

```rust
        "#a" => ['a' 6208, 'ā' 289],
        "#act" => ['ā' 26, 'a' 11],
    ...
    "abbati" => 'ā',
```

From `ancient-greek`, the law of limitation's facts, and an accent pattern (where each
cell's accent goes before the law moves it):

```rust
    prosody: Some(flexion::Prosody {
        diphthongs: &["αι", "ει", "οι", "υι", "αυ", "ευ", "ηυ", "ου", "ωυ"],
        hiatus: "ϊϋ",
        long: "ηωἠἡὠὡᾀᾁᾐᾑᾠᾡᾱᾳῃῑῡῳ",
        short_finals: &["αι", "οι"],
        ...
    }),

    // 1, `νῠμφη`: 1,865 words. Like 0 (`σημειον`), except in 1 cell.
    pattern!("νῠμφη", 1865, like 0;
    gen_: _, _, Ending;
    ),
```

And what no rule gives, as the dictionary writes it (a table: word, then its listed
stems and forms by cell). Latin `adiuvō`'s perfect `adiūvī` follows from no rule on
`adiuvō`:

```text
adiuvo	18	adiūvī	20	adiūvistī	22	adiūvit
```

## What it costs

A clean debug build of each crate, one at a time (one machine; CI's numbers are in its
compile-cost job), and one call (release: a word's candidates for one cell, over 30 words
and their first 40 cells; `cargo xtask bench <language> --pos <part>`, last measured on
the grammars of PR #14), on words the grammar was learned from and on held-out words:

| crate | source MB | build seconds | peak MB | `.rlib` MB | one call, nouns | one call, verbs |
|---|---:|---:|---:|---:|---:|---:|
| english | 0.6 | 2 | 500 | 44 | 9 / 9 µs | 10 / 10 µs |
| russian | 2.3 | 6 | 829 | 71 | 16 / 17 µs | 36 / 38 µs |
| latin | 1.2 | 4 | 702 | 66 | 21 / 23 µs | 32 / 40 µs |
| ancient-greek | 3.7 | 4 | 753 | 74 | 44 / 81 µs | 52 / 146 µs |
| sanskrit | 0.6 | 1 | 441 | 39 | 5 / 7 µs | 19 / 25 µs |
| polish | 1.2 | 4 | 668 | 58 | 13 / 25 µs | 13 / 17 µs |
| german | 1.5 | 5 | 931 | 54 | 12 / 22 µs | 16 / 24 µs |
| spanish | 0.5 | 2 | 457 | 40 | 4 / 4 µs | 9 / 12 µs |
| french | 0.3 | 1 | 421 | 45 | 4 / 4 µs | 6 / 6 µs |
| italian | 0.5 | 2 | 468 | 47 | 4 / 4 µs | 6 / 11 µs |
| finnish | 1.1 | 2 | 529 | 32 | 16 / 16 µs | adjectives 10 / 11 µs |
| turkish | 0.2 | 1 | 324 | 22 | 8 / 9 µs | adjectives 5 / 5 µs |

Ancient Greek's verbs, 624 cells each, are most of its size: their paradigms (0.6 MB,
286 conjugations) and their irregular forms (1.1 MB: 2,863 listed stems with the ending
table each builds on, and 33,829 forms no stem and table give). Nothing is computed
before the first answer, and the same entries always give the same crate, byte for byte.

## How the grammar is learned

Everything below is learned from the dictionary's inflection tables, the same way for
every language, and kept only where it pays for itself: what an item costs to write
down (its letters and references, in bits) against what it saves in listing the words
it describes (`crates/flexion-train/src/bits.rs`).

- **Paradigms.** A word's forms are aligned on the letters they all keep (the stem
  pieces); what's left is a pattern per form: `~а`, `~о~` (a vowel the other forms drop),
  `ge~t`. Words with the same patterns share a paradigm, named after its commonest word
  that no sense marks vulgar or offensive (the dumps give no word frequencies: commonest
  means in the most dictionaries, with the most senses). A word missing some forms joins
  the fullest paradigm that builds the forms it has. A paradigm states the inherent
  features its words share (`[gender: Neuter]`), and is *like* the commoner paradigm it
  shares the most cells with: it writes only the cells that differ (`_` for the rest). It
  may be like it through a *view*: one feature's value read as another (a Latin deponent
  is like `agitō` with its active cells read as the passive ones: `like Agitō as
  ACT_AS_PASS`), or with something added after every pattern it reads there (an
  *affix*: a Russian reflexive is its plain verb with `~я`, `~ь` after a vowel, the stem's
  last piece their `с`: `like Играть with "~ь", "~я" after "йлмть"`). A paradigm of
  more than one word is kept where it costs fewer bits than
  listing the forms the next paradigm doesn't build; a word whose table only another
  paradigm nearly builds is listed there. A cell whose form is another cell's for nearly
  every word is that cell's, stated once (Greek middle = passive outside the aorist and
  future).
- **Stems and ending tables.** A part of speech's cells fall into stem slots, learned
  from where words' stems change together (Greek verbs: the twelve tense stems the
  dictionary's own conjugation template has). Per slot, the ending tables paradigms share
  are named after their commonest word; a word the paradigms don't build lists one stem
  per slot and the table that builds on it, not every form (`αγοραζω  perfect  ἠγορᾰσ-
  like ἐλπω`: ἀγοράζω builds its perfect on `ἠγορᾰσ-` with ἐλπω's endings). A form the
  caller gives fixes its slot's stem.
- **Sound laws.** A built form marks where its stem meets its ending (`+`). Where a word's
  paradigm, or a more general one, would build its forms but for a change there, the
  change is a law: Russian `[к|г|х|ш|ч|ж|щ]+ы → +и`, Greek `γ+σ → +ξ`, `κ+θ → +χθ`. A law
  can also change a letter of the ending by the nearest of some letters before it (a
  tier): Sanskrit `न → ण` after `र`, `ष`, `ृ` unless a dental, palatal or retroflex comes
  between. Which letters make the change, block it, or stop it after it, is learned. A
  law can also change a copied letter (reduplication, `%`): Greek copies `φ` as `π`
  (`πεφίληκα`); or join a form's first literal with the stem's first letters (`‹`): the
  Greek augment lengthens a vowel (`ἐ‹ἀ → ἠ`, `ἐ‹αἰ → ᾐ`: `ἦγον`, `ᾔτουν`), and the
  same law takes the letter written without its breathing (inside a compound). A law
  is kept when it gives forms of five or more words in two or more cells and breaks
  fewer than one in twenty of the forms it gives; the words it explains
  join the more general paradigm, and the forms it breaks are listed. A law may read its
  own earlier output and map a class of letters by the class letter that triggers it, so
  vowel harmony through stacked suffixes is one law: Turkish `I` → ı/i/u/ü and `A` →
  a/e by the word's last vowel, Finnish `A` → a/ä with `e`, `i` transparent; endings then
  print with class letters (`~lAr`). A class can copy the vowel before it: Finnish `V`,
  the illative `talo-Vn` → `taloon`, `maa-hVn` → `maahan`, and the possessive `-Vn`. A
  law can change a letter inside the stem, by what follows it, in the stems a paradigm
  marks as alternating (`ˇ`): Finnish consonant gradation (`katu`, `kadun`: `t → d`
  before a closed syllable; `kukka`, `kukan`) and its inverse (`hammas`, `hampaan`) are
  such laws, so `katu`, `itu`, `veto` and `otto` (`oton`) share a paradigm. Contexts grow
  one letter at a time while a law breaks too much, and every law is checked on held-out
  words. Laws run in [`sound-law`](crates/sound-law), printed in its notation and compiled
  when the crate is built (a law that doesn't compile is a compile error). A class letter
  goes on every spelling a cell lists (Finnish `~sellAAn|~sellAnsA`), not its first only.
- **Relations.** Each cell's form is told from another cell's (the one that predicts it
  best: a tree on the lemma's cell), its end changed by rules read off that form's ending
  (Latin `amāvī` → `amāvistī`). They are learned after the sound laws, and a change whose
  new ending is written in class letters goes through them: Turkish "my" is one change,
  `+Im` (`evim`, `kızım`, `gölüm`, `okulum`), where it was a change per vowel. A
  paradigm's cell that a relation with one change tells from a cell of the same stem
  slot is `derived` rather than written where that costs fewer bits (Finnish `talossani`
  is `talossa` and `-ni`, for every noun). Where a feature of the word decides which
  cell (Russian: the accusative is the genitive of animate nouns, the nominative of
  inanimate ones), the relation is by that feature, and the feature is a choice of its
  own; where one cell gives the form whatever the feature is, the feature has no say
  (Italian `parli`, the polite imperative, is the subjunctive's third person for every
  verb). A paradigm takes the cells none of its words has from its relations (`derived`),
  and a word whose forms are listed lists only its principal parts: its other forms are
  told from them, where that does as well as its paradigm. A cell too few words have is
  told by analogy: from the cell that differs from it as another pair of cells does,
  with that pair's change. Every word gets a form in every cell, attested or not.
- **Every word.** Where a part of speech has far more words than its structure needs
  (Finnish nouns: ~114,000), its paradigms, relations and laws are learned from the
  commonest (6,000), word by word; then every word takes the paradigm that builds the
  most of its forms, teaches the rules that choose one, and lists what the grammar still
  gets wrong. Every dictionary word gets its own forms.
- **Choices.** Which paradigm a word takes is a list of rules on its ending: the longest
  rule the word has decides, with how many of the dictionary's words with that ending
  take each paradigm. A rule is kept only when it describes its words in fewer bits
  than the broader rule does, by more than the rule costs (minimum description length). Words no rule gets
  right are listed. A rule's status follows Yang's Tolerance Principle, judged as a learner
  meets it: on the 250, 500, 1,000 and 2,000 commonest words (in the most dictionaries,
  with the most senses) that have its ending; a rule with few enough exceptions there is
  productive (`regular`, printed on the rule); otherwise speakers vary (`unsettled`).
- **Compounds and prefixes.** A word written in parts inflects its last part or each
  part (Russian `диван-кровать`). A word the grammar doesn't list that ends in a listed
  word takes that word's listing, its front kept (Finnish `aavameri` inflects as `meri`,
  and its forms aren't listed): every listed word from some length on, where nine in ten
  of the dictionary's words ending in an irregular word that long inflect as it (Finnish
  five letters; none in English, Russian or German), and, of the shorter ones, each word
  whose own compounds do (`heads.rs`: German `Schein` for `Fahrschein`, `Wein` for
  `Schaumwein`). German verbs inflect behind their prefixes: separable ones go where the
  form puts them (`fing an`), inseparable ones stay in front and take off the
  participle's `ge-` (`verhindert`). The prefixes, and what the participle drops, are
  learned.
- **Accent.** Where a language writes it (Ancient Greek), each cell has a rule placing the
  accent (where the lemma has it, on the ending, as far back as allowed, or on the word's
  last or next-to-last syllable) and the law
  of limitation moves it and picks acute or circumflex by the syllables' length. Which
  endings take the circumflex, and where a final `-οι` counts long (the optative), are
  learned.
- **Spelling.** Words are read in Unicode's composed form (NFC), so a letter typed with
  its marks in any order is the same letter. A word is found however it is typed:
  without vowel length (`rosa`, `λυω`), without `ё`. Rules on the letters around each
  vowel restore the marks (`-ātiō`, `dē-`), learned once for the language from every
  part of speech's words (`marks.rs`: a root's length shown in a verb tells it in its
  noun), and the three likeliest spellings are read, over every vowel at once; the
  dictionary's words that the rules don't spell are listed, a root whose words (two or
  more) all write a vowel one way the rules don't is listed and read first (Latin words
  beginning `amico` write `amīc-`), and the answer's probability
  says how much of the spelling was a guess. A circumflex is vowel length on a single
  vowel (`σῦς`), not on a diphthong's second letter (`ἀριθμοῦ`).
- **Phrases.** A form the dictionary writes as a phrase is learned like any other: Latin
  `amātus sum`, the participle agreeing in number (`amātī sumus`), Greek `λελυμένος ὦ`,
  each word keeping its own accent. Where the dictionary's table only gives the recipe
  (the Latin perfect passive: the participle and a form of `sum`), the forms are written
  out from it (Allen & Greenough §§184–196; Smyth §§599, 696).
- **Irregular forms.** Whatever no rule gives is listed, as the dictionary writes it.
- **Calibration.** Probabilities are sharpened by one power per part of speech, fitted on
  a tenth of the words the grammar didn't learn from, typed as their headwords; an answer
  of a status whose answers are right less often than they say keeps only the share it
  earns (`trust`), the rest is "none of these".

The trainer builds the same values the crates hold, and prints them; a test prints
grammars learned from real entries, compiles them, and checks that they answer exactly
as the grammars they were printed from.

## Layout

- `crates/flexion`: the runtime: the grammar's types and how they answer.
- `crates/flexion-train`: the trainer: extraction from Kaikki dumps, learning, evaluation,
  printing.
- `crates/flexion-configs`: what the trainer knows about each language: its features,
  cells and Wiktionary's tags, and its README's introduction.
- `crates/sound-law`: sound-law programs.
- `languages/*`: the generated crates.
- `xtask`: `cargo xtask extract|build|readme|try|inspect <language>`.

## Building a language

```sh
cd flexion
cargo xtask extract russian   # read ~/.cache/flexion/dumps/Russian.jsonl
cargo xtask build russian     # learn, measure, and write languages/russian
```

## License

The code is MIT or Apache-2.0. The generated grammars are derived from Wiktionary and are
licensed under CC BY-SA 4.0 (see each crate's `DATA-LICENSE.md`).
