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
| english | proper_noun | **99.35%** (99.1–99.6) | 99.35% |  |  | 99.28% | +0.07 (-0.12…+0.28) | 99.47% / 99.33% | 99.68% | 100.00% |  | 99.37% (4311) / – / 90.00% (10) |  | 0.05% | 0.002 |
| english | verb | **97.17%** (96.8–97.6) | 97.17% | 99.54% |  | 97.79% | -0.62 (-0.93…-0.33) | 98.92% / 96.23% | 97.61% | 99.86% |  | 97.14% (3598) / 100.00% (3) / 97.36% (550) |  | 1.01% | 0.006 |
| english | adjective | **95.35%** (94.8–95.9) | 95.35% |  |  | 92.53% | +2.82 (+2.39…+3.25) | 99.05% / 93.61% | 96.30% | 99.94% |  | 95.59% (5574) / 75.00% (4) / 93.48% (661) | 99.06% (11894) | 0.06% | 0.005 |
| english | adverb | **99.01%** (98.4–99.5) | 99.01% |  |  | 98.34% | +0.66 (+0.25…+1.16) | 100.00% / 98.67% | 99.59% | 100.00% |  | 98.95% (1146) / – / 100.00% (61) | 99.67% (2398) | 0.11% | 0.005 |
| russian | noun | **94.65%** (94.0–95.2) | 94.65% | 95.81% | 97.30% | 94.72% | -0.08 (-0.51…+0.37) | 97.06% / 93.82% | 94.99% | 99.82% |  | 94.57% (2394) / 96.55% (181) / 90.08% (36) |  | 0.54% | 0.007 |
| russian | proper_noun | **90.86%** (88.5–93.0) | 90.86% | 92.69% | 96.88% | 93.60% | -2.75 (-4.11…-1.41) | 93.22% / 90.80% | 90.98% | 99.88% |  | 91.29% (331) / 100.00% (1) / 69.23% (8) |  | 2.19% | 0.025 |
| russian | adjective | **99.75%** (99.6–99.9) | 99.75% |  |  | 99.76% | -0.01 (-0.12…+0.09) | 99.71% / 99.77% | 99.75% | 99.94% |  | 99.11% (950) / 100.00% (2512) / 100.00% (4) |  | 0.07% | 0.002 |
| russian | verb | **95.76%** (95.0–96.4) | 95.76% | 98.88% | 95.49% | 96.65% | -0.89 (-1.39…-0.40) | 95.84% / 95.72% | 95.89% | 99.49% |  | 95.74% (1269) / 100.00% (29) / 81.44% (6) |  | 0.75% | 0.006 |
| russian | pronoun | **54.26%** (8.5–100.0) | 54.26% |  |  | 54.26% | +0.00 (+0.00…+0.00) | 0.00% / 54.26% | 54.26% | 100.00% |  | 54.26% (4) / – / – |  | 30.95% | 0.290 |
| russian | numeral | **94.12%** (67.5–100.0) | 94.12% |  |  | 100.00% | -5.88 (-32.53…+0.00) | 100.00% / 91.51% | 94.12% | 100.00% |  | 94.12% (4) / – / – |  | 24.68% | 0.006 |
| russian | adverb | **66.67%** (44.4–88.9) | 66.67% |  |  | 83.33% | -16.67 (-33.33…+0.00) | 0.00% / 66.67% | 72.22% | 100.00% |  | 66.67% (18) / – / – |  | 11.89% | 0.310 |
| latin | noun | **71.12%** (68.9–73.4) | 92.93% | 98.70% | 71.98% | 59.09% | +12.03 (+9.41…+14.72) | 77.96% / 69.02% | 71.22% | 99.46% | 69.72% (984) | 71.25% (1379) / 77.99% (19) / 69.27% (167) |  | 1.16% | 0.038 |
| latin | proper_noun | **59.84%** (56.4–63.2) | 92.75% | 98.24% | 59.84% | 60.58% | -0.74 (-4.64…+3.12) | 58.77% / 59.93% | 60.32% | 99.32% | 47.51% (362) | 59.75% (801) / – / 61.84% (33) |  | 1.63% | 0.054 |
| latin | adjective | **87.50%** (86.3–88.6) | 99.23% | 99.48% |  | 71.87% | +15.63 (+13.86…+17.46) | 89.98% / 85.95% | 87.61% | 99.60% | 86.66% (2151) | 76.94% (981) / 93.73% (1684) / 86.59% (176) |  | 0.48% | 0.022 |
| latin | verb | **77.49%** (74.4–80.6) | 84.92% | 98.59% |  | 64.17% | +13.32 (+9.35…+17.33) | 84.08% / 74.65% | 77.61% | 98.43% | 89.86% (592) | 78.26% (541) / – / 72.34% (78) | 74.64% (16157) | 4.69% | 0.027 |
| latin | pronoun | **6.34%** (0.0–16.1) | 19.08% |  |  | 25.00% | -18.66 (-47.89…+3.14) | 0.00% / 6.34% | 10.07% | 100.00% | 14.29% (7) | 8.42% (7) / – / 0.00% (2) |  | 73.75% | 0.230 |
| latin | numeral | **72.73%** (45.5–100.0) | 100.00% |  |  | 72.73% | +0.00 (-36.36…+36.36) | 66.67% / 75.00% | 72.73% | 100.00% | 72.73% (11) | 62.50% (8) / – / 100.00% (3) |  | 0.85% | 0.303 |
| latin | adverb | **85.14%** (75.8–93.3) | 98.35% | 96.72% |  | 44.57% | +40.57 (+25.58…+55.62) | 100.00% / 84.62% | 85.71% | 100.00% | 85.19% (54) | 84.05% (58) / 100.00% (3) / 100.00% (2) |  | 5.56% | 0.084 |
| ancient-greek | noun | **74.79%** (71.9–77.5) | 89.15% | 96.09% | 75.82% | 51.28% | +25.16 (+21.24…+28.91) | 81.35% / 73.25% | 74.81% | 99.55% | 78.50% (586) | 74.59% (784) / 66.67% (3) / 82.96% (22) |  | 8.64% | 0.048 |
| ancient-greek | proper_noun | **59.73%** (54.6–64.9) | 86.50% | 91.00% | 62.07% | 47.42% | +13.90 (+7.80…+20.37) | 56.73% / 60.01% | 59.88% | 99.40% | 63.80% (221) | 60.11% (375) / – / 43.48% (8) |  | 5.09% | 0.080 |
| ancient-greek | adjective | **78.02%** (74.8–81.2) | 90.76% | 98.02% |  | 52.46% | +26.37 (+21.84…+30.57) | 82.37% / 77.17% | 78.21% | 99.17% | 84.05% (301) | 79.52% (349) / 74.20% (115) / 68.31% (10) |  | 2.91% | 0.040 |
| ancient-greek | verb | **68.04%** (63.4–72.5) | 75.67% | 81.10% |  | 44.20% | +25.05 (+19.23…+30.66) | 77.94% / 62.22% | 68.50% | 99.90% | 82.50% (160) | 68.66% (244) / 45.83% (3) / 53.40% (10) | 57.62% (4832) | 46.74% | 0.084 |
| ancient-greek | pronoun | **40.33%** (10.3–69.3) | 40.16% |  |  | 41.96% | +6.81 (-8.25…+30.49) | 0.00% / 40.33% | 40.33% | 99.46% | 100.00% (1) | 37.37% (8) / 88.89% (1) / 0.00% (2) |  | 51.72% | 0.602 |
| ancient-greek | numeral | **100.00%** (100.0–100.0) | 100.00% |  |  | 100.00% | +0.00 (+0.00…+0.00) | 0.00% / 100.00% | 100.00% | 100.00% |  | 100.00% (2) / – / – |  | 21.05% | 0.061 |
| sanskrit | noun | **92.10%** (90.8–93.4) | 92.10% |  | 97.30% | 91.70% | +0.40 (-0.73…+1.48) | 94.30% / 92.03% | 92.65% | 97.83% |  | 92.14% (563) / 100.00% (2) / 88.71% (11) |  | 1.57% | 0.015 |
| sanskrit | proper_noun | **95.15%** (90.0–99.1) | 95.15% |  | 96.34% | 86.48% | +8.67 (+3.40…+14.54) | 0.00% / 95.15% | 95.15% | 100.00% |  | 95.05% (48) / – / 100.00% (1) |  | 2.13% | 0.030 |
| sanskrit | adjective | **91.91%** (89.6–94.0) | 91.91% |  |  | 90.11% | +1.81 (+0.17…+3.56) | 83.33% / 92.30% | 91.94% | 100.00% |  | 91.60% (183) / 100.00% (6) / 100.00% (1) |  | 3.82% | 0.020 |
| sanskrit | numeral | **63.64%** (29.8–99.1) | 63.64% |  |  | 92.42% | -28.79 (-55.13…-0.93) | 100.00% / 50.00% | 63.64% | 100.00% |  | 63.64% (9) / – / – |  | 7.69% | 0.349 |
| sanskrit | verb | **89.86%** (85.3–94.0) | 89.86% | 90.58% | 89.86% | 89.93% | -0.07 (-2.32…+2.05) | 95.65% / 89.31% | 90.37% | 100.00% | 100.00% (2) | 91.49% (71) / 55.52% (17) / – |  | 24.23% | 0.045 |
| sanskrit | pronoun | **69.57%** (0.0–100.0) | 69.57% |  |  | 70.53% | -0.97 (-3.17…+0.00) | 0.00% / 69.57% | 69.57% | 100.00% |  | 69.57% (3) / – / – |  | 46.67% | 0.298 |
| polish | noun | **93.42%** (92.9–93.9) | 93.42% | 95.79% | 96.25% | 93.34% | +0.08 (-0.29…+0.44) | 95.10% / 93.10% | 93.70% | 99.23% |  | 92.44% (3110) / 98.91% (809) / 88.39% (62) |  | 1.81% | 0.003 |
| polish | proper_noun | **94.01%** (93.3–94.7) | 94.01% | 95.06% | 94.99% | 93.74% | +0.26 (-0.25…+0.83) | 98.13% / 93.38% | 94.20% | 97.11% |  | 94.12% (1760) / 38.46% (2) / 50.00% (4) |  | 1.36% | 0.008 |
| polish | adjective | **99.92%** (99.9–99.9) | 99.92% |  |  | 99.99% | -0.07 (-0.09…-0.04) | 99.94% / 99.91% | 99.92% | 100.00% |  | 99.90% (1147) / 100.00% (239) / 100.00% (8) |  | 0.36% | 0.001 |
| polish | verb | **97.17%** (96.5–97.9) | 97.17% | 98.38% | 97.29% | 97.66% | -0.49 (-0.99…+0.02) | 95.72% / 97.49% | 97.45% | 99.79% |  | 97.20% (1230) / 100.00% (2) / 93.03% (11) |  | 1.35% | 0.009 |
| polish | pronoun | **81.64%** (50.0–99.4) | 81.64% |  |  | 81.64% | +0.00 (+0.00…+0.00) | 0.00% / 81.64% | 81.64% | 100.00% |  | 75.00% (4) / 100.00% (1) / 89.83% (1) |  | 60.71% | 0.656 |
| polish | numeral | **0.00%** (0.0–0.0) | 0.00% |  |  | 0.00% | +0.00 (+0.00…+0.00) | 0.00% / 0.00% | 0.00% | 0.00% |  | – / – / – |  | 0.00% | 0.000 |
| polish | adverb | **93.75%** (87.5–98.4) | 93.75% | 100.00% |  | 95.31% | -1.56 (-7.81…+3.12) | 100.00% / 93.65% | 93.75% | 100.00% |  | 93.65% (63) / – / 100.00% (1) |  | 5.23% | 0.068 |
| german | noun | **93.62%** (93.2–94.1) | 93.62% | 99.79% | 94.05% | 94.77% | -1.15 (-1.46…-0.85) | 96.70% / 89.91% | 94.43% | 99.69% |  | 93.44% (5249) / 98.79% (730) / 87.78% (344) |  | 0.53% | 0.007 |
| german | numeral | **100.00%** (100.0–100.0) | 100.00% |  | 100.00% | 100.00% | +0.00 (+0.00…+0.00) | 0.00% / 100.00% | 100.00% | 100.00% |  | 100.00% (2) / – / – |  | 0.00% | 0.000 |
| german | proper_noun | **91.53%** (88.5–94.3) | 91.53% | 99.77% | 91.53% | 90.66% | +0.87 (-1.57…+3.20) | 100.00% / 91.05% | 93.03% | 99.21% |  | 91.39% (296) / 100.00% (1) / 95.00% (12) |  | 0.28% | 0.028 |
| german | adjective | **98.79%** (98.2–99.3) | 98.79% | 99.88% |  | 99.02% | -0.23 (-0.73…+0.26) | 98.61% / 98.87% | 99.06% | 100.00% |  | 98.73% (1410) / 100.00% (28) / 100.00% (49) | 96.53% (548) | 0.07% | 0.006 |
| german | verb | **95.17%** (94.2–96.1) | 95.17% | 95.98% | 95.28% | 73.16% | +22.01 (+20.25…+23.80) | 95.91% / 94.35% | 95.27% | 99.25% |  | 95.00% (936) / – / 99.83% (34) | 91.76% (9057) | 5.11% | 0.012 |
| german | determiner | **40.00%** (0.0–100.0) | 40.00% |  |  | 40.00% | +0.00 (+0.00…+0.00) | 0.00% / 40.00% | 40.00% | 100.00% |  | 40.00% (3) / – / – |  | 13.89% | 0.598 |
| spanish | noun | **98.67%** (98.4–98.9) | 98.67% |  | 98.67% | 98.67% | +0.00 (-0.20…+0.20) | 99.85% / 98.12% | 98.88% | 99.98% |  | 98.74% (5245) / 100.00% (583) / 94.02% (251) |  | 0.11% | 0.003 |
| spanish | adjective | **98.50%** (98.1–98.9) | 98.50% | 99.67% |  | 98.60% | -0.10 (-0.35…+0.14) | 99.84% / 98.04% | 98.61% | 100.00% |  | 98.49% (2284) / 100.00% (68) / 97.12% (47) |  | 0.21% | 0.003 |
| spanish | verb | **98.97%** (98.6–99.3) | 98.97% | 99.73% |  | 99.28% | -0.31 (-0.60…-0.02) | 99.28% / 98.77% | 98.99% | 99.98% |  | 99.02% (879) / 100.00% (4) / 97.86% (46) |  | 1.73% | 0.004 |
| spanish | determiner | **90.91%** (70.0–100.0) | 90.91% |  |  | 100.00% | -9.09 (-30.00…+0.00) | 100.00% / 90.48% | 90.91% | 100.00% | 0.00% (1) | 77.78% (4) / 100.00% (2) / 100.00% (3) |  | 11.54% | 0.094 |
| spanish | adverb | **0.00%** (0.0–0.0) | 0.00% |  |  | 0.00% | +0.00 (+0.00…+0.00) | 0.00% / 0.00% | 0.00% | 0.00% |  | – / – / – |  | 42.86% | 0.000 |
| french | noun | **97.59%** (97.1–98.0) | 97.59% |  | 97.59% | 97.03% | +0.56 (+0.23…+0.90) | 96.88% / 97.85% | 97.98% | 99.88% |  | 97.97% (3898) / 99.18% (245) / 86.78% (174) |  | 0.65% | 0.004 |
| french | adjective | **97.50%** (96.9–98.1) | 97.50% | 99.95% |  | 97.72% | -0.22 (-0.65…+0.24) | 98.85% / 97.12% | 97.57% | 100.00% |  | 97.90% (1781) / – / 85.06% (58) |  | 0.28% | 0.009 |
| french | verb | **98.70%** (98.1–99.3) | 98.70% | 99.48% |  | 99.28% | -0.59 (-1.06…-0.22) | 99.63% / 98.40% | 98.84% | 100.00% |  | 98.74% (724) / – / 97.08% (19) |  | 0.96% | 0.004 |
| french | participle | **99.84%** (99.5–100.0) | 99.84% |  |  | 99.52% | +0.32 (-0.32…+0.96) | 100.00% / 99.80% | 99.84% | 100.00% |  | 100.00% (1) / 99.84% (626) / – |  | 0.03% | 0.005 |
| french | determiner | **46.15%** (14.3–85.7) | 46.15% |  |  | 69.23% | -23.08 (-64.29…+0.00) | 0.00% / 46.15% | 46.15% | 100.00% |  | 46.15% (6) / – / – |  | 22.45% | 0.538 |
| french | adverb | **0.00%** (0.0–0.0) | 0.00% |  |  | 0.00% | +0.00 (+0.00…+0.00) | 0.00% / 0.00% | 0.00% | 0.00% |  | – / – / – |  | 0.00% | 0.000 |
| italian | noun | **95.82%** (95.3–96.3) | 95.82% |  | 96.39% | 95.09% | +0.74 (+0.31…+1.16) | 95.91% / 95.76% | 95.96% | 99.77% |  | 95.85% (5862) / 98.57% (210) / 93.44% (320) |  | 0.19% | 0.003 |
| italian | adjective | **98.46%** (98.0–98.8) | 98.46% | 99.78% |  | 98.46% | +0.00 (-0.28…+0.28) | 99.36% / 97.96% | 98.49% | 100.00% |  | 98.48% (2514) / 100.00% (42) / 96.86% (75) |  | 0.19% | 0.010 |
| italian | verb | **99.05%** (98.7–99.4) | 99.05% | 99.53% | 99.05% | 99.22% | -0.17 (-0.48…+0.13) | 98.93% / 99.12% | 99.12% | 99.96% |  | 99.34% (1188) / 100.00% (9) / 95.12% (89) |  | 1.77% | 0.005 |
| italian | participle | **100.00%** (100.0–100.0) | 100.00% |  |  | 100.00% | +0.00 (+0.00…+0.00) | 100.00% / 100.00% | 100.00% | 100.00% |  | 100.00% (11) / 100.00% (810) / – |  | 0.02% | 0.000 |
| italian | determiner | **81.82%** (54.5–100.0) | 81.82% |  |  | 86.36% | -4.55 (-13.64…+0.00) | 100.00% / 78.95% | 81.82% | 100.00% |  | 81.82% (8) / – / – |  | 13.33% | 0.182 |
| italian | adverb | **0.00%** (0.0–0.0) | 0.00% | 0.00% |  | 0.00% | +0.00 (+0.00…+0.00) | 0.00% / 0.00% | 0.00% | 100.00% |  | 0.00% (1) / – / – |  | 75.00% | 1.000 |
| finnish | noun | **97.81%** (97.6–98.0) | 97.81% | 99.15% |  | 98.29% | -0.48 (-0.68…-0.29) | 98.56% / 96.30% | 97.98% | 99.98% |  | 97.87% (12649) / 100.00% (7) / 94.16% (213) |  | 1.34% | 0.009 |
| finnish | pronoun | **2.14%** (0.0–6.6) | 2.14% |  |  | 9.63% | -7.49 (-16.57…-1.39) | 0.00% / 2.92% | 2.14% | 100.00% |  | 2.63% (7) / 0.00% (1) / 0.00% (1) |  | 73.33% | 0.336 |
| finnish | numeral | **32.14%** (9.8–64.6) | 32.14% |  |  | 43.30% | -11.16 (-41.96…+19.73) | 0.00% / 36.18% | 32.14% | 100.00% |  | 23.62% (8) / – / 100.00% (1) |  | 44.00% | 0.288 |
| finnish | proper_noun | **93.86%** (92.6–95.1) | 93.86% | 99.03% |  | 91.77% | +2.10 (+0.76…+3.41) | 98.18% / 92.76% | 94.36% | 99.88% |  | 93.96% (911) / – / 75.61% (6) |  | 2.49% | 0.018 |
| finnish | adjective | **97.45%** (96.9–98.0) | 97.45% |  |  | 97.41% | +0.04 (-0.62…+0.72) | 99.15% / 96.40% | 97.52% | 100.00% |  | 97.37% (1570) / 98.54% (104) / 97.81% (47) |  | 0.85% | 0.009 |
| finnish | verb | **98.49%** (98.0–98.9) | 98.49% | 99.48% |  | 97.56% | +0.92 (+0.22…+1.68) | 99.35% / 98.27% | 98.49% | 99.93% |  | 98.48% (1103) / 100.00% (59) / 96.86% (57) | 98.94% (117603) | 3.97% | 0.003 |
| finnish | adverb | **96.67%** (94.5–98.6) | 96.67% | 98.57% |  | 97.39% | -0.71 (-1.67…+0.00) | 100.00% / 96.47% | 96.67% | 100.00% |  | 96.67% (211) / – / – |  | 2.32% | 0.016 |
| finnish | postposition | **65.22%** (21.7–100.0) | 65.22% |  |  | 78.26% | -13.04 (-30.43…+0.00) | 0.00% / 65.22% | 65.22% | 100.00% |  | 65.22% (5) / – / – |  | 4.88% | 0.177 |
| turkish | noun | **96.51%** (95.5–97.4) | 96.51% | 99.52% |  | 93.27% | +3.23 (+2.30…+4.24) | 97.63% / 96.39% | 96.55% | 99.88% |  | 96.17% (977) / 100.00% (72) / 99.51% (16) |  | 1.11% | 0.012 |
| turkish | pronoun | **0.00%** (0.0–0.0) | 0.00% |  |  | 90.91% | -90.91 (-90.91…-90.91) | 0.00% / 0.00% | 0.00% | 100.00% |  | 0.00% (1) / – / – |  | 61.54% | 1.000 |
| turkish | numeral | **31.82%** (0.0–63.6) | 31.82% |  |  | 0.00% | +31.82 (+0.00…+63.64) | 0.00% / 31.82% | 31.82% | 100.00% |  | 31.82% (2) / – / – |  | 42.86% | 0.682 |
| turkish | proper_noun | **90.94%** (85.4–95.6) | 90.94% | 97.74% |  | 93.24% | -2.31 (-6.43…+1.59) | 100.00% / 90.64% | 91.20% | 100.00% |  | 90.94% (145) / – / – | 100.00% (23) | 1.97% | 0.046 |
| turkish | adjective | **86.75%** (60.2–100.0) | 86.75% | 100.00% |  | 81.93% | +4.82 (+0.00…+14.46) | 0.00% / 86.75% | 86.75% | 100.00% |  | 86.75% (3) / – / – |  | 1.11% | 0.115 |
| turkish | verb | **98.65%** (97.4–99.6) | 98.65% | 99.72% |  | 98.68% | -0.03 (-1.42…+1.23) | 98.61% / 98.69% | 98.65% | 100.00% |  | 98.45% (198) / 100.00% (30) / – | 99.49% (44149) | 59.04% | 0.033 |

*Where the errors are*: the wrong first answers by what went wrong. *Class*: the right
form is among the candidates, from another paradigm or reading (the grammar chose
wrong); *length*: the same letters once marks are off (vowel length, a breathing);
*accent*: the right letters, the accent elsewhere; *stem* and *ending*: where it goes
wrong; *variant*: another of the forms the dictionary lists in the cell; *homograph*:
another dictionary word spelled alike has this form; *gap*: no form at all (none, in
every part). *By block*: top-1 averaged over the part's groups of cells (a Turkish
verb's negative abilitative forms count as much as its plain ones). *Reproduced*: the
shipped grammar asked again for its own words (3,000 at most), and *complete*: of their
cells, those with a form. Closed classes (pronouns, numerals) are scored both ways:
their held-out tenth is a handful of words.

| language | part of speech | wrong forms | class | length | accent | stem | ending | variant | homograph | gap | by block | reproduced | complete |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| english | noun | 671 | 63% | 0% | 0% | 4% | 2% | 28% | 2% | 0% | 97.17% | 100.00% | 100.00% |
| english | proper_noun | 28 | 46% | 0% | 0% | 4% | 0% | 50% | 0% | 0% | 99.35% | 100.00% | 100.00% |
| english | verb | 469 | 74% | 0% | 0% | 3% | 3% | 15% | 4% | 0% | 97.17% | 100.00% | 100.00% |
| english | adjective | 580 | 79% | 0% | 0% | 0% | 0% | 20% | 1% | 0% | 95.35% | 100.00% | 100.00% |
| english | adverb | 24 | 42% | 0% | 0% | 0% | 0% | 58% | 0% | 0% | 99.01% | 100.00% | 100.00% |
| russian | noun | 1479 | 81% | 3% | 0% | 1% | 5% | 6% | 3% | 0% | 93.62% | 100.00% | 100.00% |
| russian | proper_noun | 223 | 86% | 5% | 0% | 3% | 3% | 1% | 1% | 0% | 90.86% | 100.00% | 100.00% |
| russian | adjective | 420 | 69% | 5% | 0% | 0% | 1% | 0% | 24% | 0% | 98.02% | 100.00% | 100.00% |
| russian | verb | 871 | 64% | 14% | 0% | 7% | 3% | 3% | 9% | 0% | 96.04% | 100.00% | 100.00% |
| russian | pronoun | 86 | 91% | 5% | 0% | 0% | 5% | 0% | 0% | 0% | 54.26% | 100.00% | 100.00% |
| russian | numeral | 9 | 0% | 0% | 0% | 0% | 100% | 0% | 0% | 0% | 94.12% | 100.00% | 100.00% |
| russian | adverb | 6 | 17% | 0% | 0% | 67% | 0% | 17% | 0% | 0% | 66.67% | 100.00% | 100.00% |
| latin | noun | 5158 | 15% | 77% | 0% | 2% | 4% | 0% | 2% | 0% | 68.89% | 100.00% | 100.00% |
| latin | proper_noun | 2056 | 8% | 83% | 0% | 2% | 4% | 1% | 2% | 0% | 57.85% | 99.93% | 100.00% |
| latin | adjective | 12698 | 2% | 92% | 0% | 0% | 2% | 1% | 3% | 0% | 81.84% | 99.95% | 100.00% |
| latin | verb | 17359 | 49% | 35% | 0% | 5% | 4% | 1% | 6% | 0% | 77.90% | 100.00% | 100.00% |
| latin | pronoun | 251 | 24% | 15% | 0% | 8% | 49% | 4% | 0% | 0% | 6.34% | 100.00% | 100.00% |
| latin | numeral | 108 | 0% | 100% | 0% | 0% | 0% | 0% | 0% | 0% | 72.73% | 100.00% | 100.00% |
| latin | adverb | 26 | 0% | 92% | 0% | 0% | 4% | 4% | 0% | 0% | 85.14% | 100.00% | 100.00% |
| ancient-greek | noun | 2895 | 27% | 59% | 6% | 2% | 6% | 0% | 1% | 0% | 74.79% | 99.67% | 100.00% |
| ancient-greek | proper_noun | 811 | 15% | 74% | 4% | 2% | 5% | 0% | 0% | 0% | 59.73% | 99.89% | 100.00% |
| ancient-greek | adjective | 4874 | 28% | 60% | 4% | 4% | 3% | 1% | 1% | 0% | 80.60% | 99.85% | 100.00% |
| ancient-greek | verb | 22043 | 18% | 42% | 4% | 19% | 17% | 1% | 0% | 0% | 66.21% | 99.94% | 100.00% |
| ancient-greek | pronoun | 219 | 6% | 0% | 14% | 1% | 78% | 0% | 0% | 0% | 40.33% | 100.00% | 100.00% |
| ancient-greek | numeral | 0 | 0% | 0% | 0% | 0% | 0% | 0% | 0% | 0% | 100.00% | 100.00% | 100.00% |
| sanskrit | noun | 1090 | 25% | 27% | 0% | 10% | 4% | 7% | 27% | 0% | 92.10% | 100.00% | 100.00% |
| sanskrit | proper_noun | 57 | 19% | 16% | 0% | 26% | 39% | 0% | 0% | 0% | 95.15% | 100.00% | 100.00% |
| sanskrit | adjective | 1088 | 20% | 67% | 0% | 6% | 7% | 0% | 0% | 0% | 39.00% | 100.00% | 100.00% |
| sanskrit | numeral | 96 | 0% | 79% | 0% | 0% | 21% | 0% | 0% | 0% | 63.64% | 100.00% | 100.00% |
| sanskrit | verb | 728 | 9% | 28% | 0% | 24% | 35% | 5% | 0% | 0% | 82.00% | 100.00% | 100.00% |
| sanskrit | pronoun | 63 | 0% | 0% | 0% | 0% | 100% | 0% | 0% | 0% | 69.57% | 100.00% | 100.00% |
| polish | noun | 2920 | 68% | 2% | 0% | 3% | 11% | 4% | 11% | 0% | 93.42% | 99.99% | 100.00% |
| polish | proper_noun | 1283 | 38% | 2% | 0% | 2% | 6% | 3% | 48% | 0% | 94.01% | 100.00% | 100.00% |
| polish | adjective | 67 | 66% | 15% | 0% | 7% | 9% | 3% | 0% | 0% | 80.00% | 100.00% | 100.00% |
| polish | verb | 1518 | 63% | 8% | 0% | 5% | 7% | 10% | 8% | 0% | 96.82% | 100.00% | 100.00% |
| polish | pronoun | 65 | 0% | 88% | 0% | 0% | 12% | 0% | 0% | 0% | 81.64% | 100.00% | 100.00% |
| polish | numeral | 0 | 0% | 0% | 0% | 0% | 0% | 0% | 0% | 0% | -0.00% | 100.00% | 100.00% |
| polish | adverb | 8 | 0% | 0% | 0% | 0% | 100% | 0% | 0% | 0% | 93.75% | 100.00% | 100.00% |
| german | noun | 2554 | 62% | 6% | 0% | 4% | 12% | 13% | 4% | 0% | 93.62% | 100.00% | 100.00% |
| german | numeral | 0 | 0% | 0% | 0% | 0% | 0% | 0% | 0% | 0% | 100.00% | 100.00% | 100.00% |
| german | proper_noun | 107 | 67% | 0% | 0% | 0% | 7% | 18% | 8% | 0% | 91.53% | 100.00% | 100.00% |
| german | adjective | 1519 | 32% | 42% | 0% | 0% | 3% | 23% | 0% | 0% | 98.04% | 100.00% | 100.00% |
| german | verb | 2443 | 57% | 1% | 0% | 19% | 6% | 2% | 15% | 0% | 94.21% | 100.00% | 100.00% |
| german | determiner | 27 | 56% | 0% | 0% | 22% | 22% | 0% | 0% | 0% | 40.00% | 100.00% | 100.00% |
| spanish | noun | 81 | 69% | 6% | 0% | 5% | 2% | 16% | 1% | 0% | 98.67% | 100.00% | 100.00% |
| spanish | adjective | 108 | 81% | 3% | 0% | 5% | 5% | 7% | 0% | 0% | 85.63% | 100.00% | 100.00% |
| spanish | verb | 688 | 23% | 25% | 0% | 34% | 16% | 2% | 0% | 0% | 98.81% | 100.00% | 100.00% |
| spanish | determiner | 2 | 100% | 0% | 0% | 0% | 0% | 0% | 0% | 0% | 90.91% | 100.00% | 100.00% |
| spanish | adverb | 0 | 0% | 0% | 0% | 0% | 0% | 0% | 0% | 0% | -0.00% | 100.00% | 100.00% |
| french | noun | 104 | 75% | 0% | 0% | 5% | 1% | 16% | 3% | 0% | 97.59% | 100.00% | 100.00% |
| french | adjective | 138 | 81% | 0% | 0% | 3% | 13% | 3% | 0% | 0% | 48.77% | 100.00% | 100.00% |
| french | verb | 454 | 67% | 8% | 0% | 0% | 14% | 11% | 0% | 0% | 98.81% | 100.00% | 100.00% |
| french | participle | 3 | 100% | 0% | 0% | 0% | 0% | 0% | 0% | 0% | 99.84% | 100.00% | 100.00% |
| french | determiner | 7 | 0% | 0% | 0% | 29% | 71% | 0% | 0% | 0% | 46.15% | 100.00% | 100.00% |
| french | adverb | 0 | 0% | 0% | 0% | 0% | 0% | 0% | 0% | 0% | -0.00% | 100.00% | 100.00% |
| italian | noun | 267 | 85% | 0% | 0% | 6% | 0% | 3% | 5% | 0% | 95.82% | 100.00% | 100.00% |
| italian | adjective | 122 | 90% | 0% | 0% | 3% | 4% | 2% | 0% | 0% | 96.56% | 100.00% | 100.00% |
| italian | verb | 667 | 48% | 1% | 0% | 22% | 19% | 7% | 4% | 0% | 99.03% | 100.00% | 100.00% |
| italian | participle | 0 | 0% | 0% | 0% | 0% | 0% | 0% | 0% | 0% | 100.00% | 100.00% | 100.00% |
| italian | determiner | 4 | 25% | 0% | 0% | 50% | 25% | 0% | 0% | 0% | 81.82% | 100.00% | 100.00% |
| italian | adverb | 1 | 0% | 0% | 0% | 0% | 100% | 0% | 0% | 0% | 0.00% | 100.00% | 100.00% |
| finnish | noun | 41613 | 35% | 31% | 0% | 4% | 21% | 8% | 1% | 0% | 97.79% | 100.00% | 100.00% |
| finnish | pronoun | 183 | 5% | 4% | 0% | 13% | 78% | 0% | 0% | 0% | 2.14% | 100.00% | 100.00% |
| finnish | numeral | 152 | 18% | 0% | 0% | 27% | 55% | 0% | 0% | 0% | 32.14% | 100.00% | 100.00% |
| finnish | proper_noun | 7336 | 68% | 8% | 0% | 3% | 12% | 8% | 2% | 0% | 93.80% | 100.00% | 100.00% |
| finnish | adjective | 1132 | 27% | 52% | 0% | 2% | 16% | 3% | 0% | 0% | 97.31% | 100.00% | 100.00% |
| finnish | verb | 2978 | 54% | 5% | 0% | 0% | 36% | 0% | 5% | 0% | 98.32% | 100.00% | 100.00% |
| finnish | adverb | 14 | 64% | 0% | 0% | 14% | 21% | 0% | 0% | 0% | 96.67% | 100.00% | 100.00% |
| finnish | postposition | 8 | 75% | 0% | 0% | 0% | 25% | 0% | 0% | 0% | 65.22% | 100.00% | 100.00% |
| turkish | noun | 2485 | 67% | 7% | 0% | 11% | 7% | 1% | 7% | 0% | 96.83% | 100.00% | 100.00% |
| turkish | pronoun | 11 | 0% | 0% | 0% | 0% | 100% | 0% | 0% | 0% | 0.00% | 100.00% | 100.00% |
| turkish | numeral | 15 | 0% | 0% | 0% | 0% | 100% | 0% | 0% | 0% | 31.82% | 100.00% | 100.00% |
| turkish | proper_noun | 342 | 75% | 0% | 0% | 10% | 13% | 3% | 0% | 0% | 91.56% | 100.00% | 100.00% |
| turkish | adjective | 33 | 100% | 0% | 0% | 0% | 0% | 0% | 0% | 0% | 88.51% | 100.00% | 100.00% |
| turkish | verb | 2851 | 92% | 0% | 0% | 1% | 7% | 0% | 0% | 0% | 98.78% | 100.00% | 100.00% |

Every language now has every part of speech its dictionary inflects: proper nouns
(English names choose `-ys`, `Kennedys`, where nouns choose `-ies`, so they have
grammars of their own), adverbs in their degrees, pronouns and numerals where the
tables give them, Italian articles as their spellings in order (`il|lo|l'`), Finnish
verbs and postpositions, Turkish verbs (928 forms each). Each crate's README says what
it leaves out and why (the Polish mobile person endings, which attach to any word
beginning a clause; English and French personal pronouns, a hand-written table, as the
dictionary gives only some of their forms).

Where the grammar is weak or below the floor, and why:

- **Latin, from the headword.** A headword doesn't write vowel length (`fox` for `fōx`);
  the grammar restores it by rules on the letters around each vowel (`-ātiō`, `dē-`, a
  diphthong), learned from every part of speech's words and reading which part a word is
  (`quīngēnī`, a numeral; `indigenus`, an adjective), and reads the three likeliest
  spellings. But a stem's own lengths are lexical, so a root whose training words (two or
  more) all write a vowel one way the rules don't is listed and read first (`ROOTS` in
  `marks.rs`): on the development tenth nouns +2.0 points, adjectives +2.9, verbs +0.7, and
  +2.0, +2.6, +1.4 on a tenth no decision looked at. A root no other word shares stays a
  guess. The vowel-length rules learn from every part's words, proper nouns and adverbs
  too, which took nouns from 70.1% to 71.1%, adjectives from 87.3% to 87.5% and verbs
  from 76.9% to 77.5% on this tenth (on the development tenths verbs lost 0.41, within
  chance, and on tenth 9 0.61, within chance). Typed in full the same grammars reach
  92.9% (nouns), 99.2% (adjectives), 84.9% (verbs), and with the principal parts
  98.6–99.5%. The perfect passive is the phrase it
  is (`amātus sum`, `amātum esse`, `hortātus sum`), 74.6% right as typed from the headword.
- **Ancient Greek.** A circumflex on a diphthong's second letter (`ἀριθμοῦ`) was read as
  vowel length, which split declensions in two; read as the diphthong's, nouns went from
  133 to 87 declensions and +3.8 points, adjectives +3.1 (development tenth). The augment
  (a law where the form's first letters meet the stem: `ἐ‹ἀ → ἠ`), the perfect's
  reduplication (`%ε~κα`) and the augment behind a preverb (`ἀπο + ἐ → ἀπε`) are laws.
  The perfect middle-passive subjunctive and optative are phrases (`λελυμένος ὦ`), each
  word keeping its own accent. What's left in the verbs is mostly stems: a verb's aorist
  or perfect stem (`ἀγγέλλω` → `ἠγγέλθην`) follows from no ending, so a new verb's is a
  guess; known verbs list it with the ending table it takes, and a principal part given
  fixes its stem (81.1%). A preverb kept in front moves the accent back only in a cell
  whose accent recedes, counting the optative's `-οι` long (`προφέροι`; Smyth §§ 169,
  425–426); a breathing stays at the word's start (`προοισθήσεσθαι`); `-εως`, `-εων`
  count short for the accent (`πόλεως`, § 163a); a form that needs an accent always has
  one. With the futures' and imperatives' invented cells gone, nouns went from 71.9% to
  74.8%, adjectives from 77.3% to 78.0%, verbs from 65.5% to 68.0%, and the verbs'
  irregular lines from 52.4% to 46.7%.
- **Sanskrit.** The `n` → `ṇ` rule is a law (`तर्काणाम्`), and a noun's gender, given,
  takes it from 92.1% to 97.3%. The verbs are small (a tenth is ~300 words). Reading a
  root's tense pages (a perfect's, an aorist's) into its verb was tried: fewer classes
  (59 → 34), but 1.9 points lost, so they stay apart.
- **Closed classes** (pronouns, numerals, determiners, postpositions) are a handful of
  held-out words each, mostly listed: their grammar reproduces every one of its own words
  (*reproduced* in the table above), and a word it hasn't seen is a guess (Finnish
  pronouns, Turkish pronouns).
- **Polish nouns** lost Middle Polish (`pl-mid-decl-noun`) tables, which aren't the modern
  language's: on the development tenth −0.06 from the headword and −0.14 with the
  principal parts, the one loss this change was kept with.
- **Below the floor, beyond chance**, now that the floor learns from the same words as
  the grammar (every edition's, not the main dump's only): German nouns (−1.15), Russian
  verbs (−0.89), English verbs (−0.62), French verbs (−0.59), Finnish nouns (−0.48),
  Spanish verbs (−0.31), Polish adjectives (−0.07), and of the new parts Russian proper
  nouns (−2.75), Finnish pronouns (−7.49, 9 words), Sanskrit numerals (−28.79, 9
  words, four in five wrong forms the class chosen wrong) and Turkish pronouns (one
  held-out word, its 11 forms all wrong in the stem); Spanish adjectives are within
  chance of it (−0.10). Finnish verbs are above it (+0.92). The grammar's own top-1 on these rows is what it was
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
    stem gen_sg: "~- like приём";
    nom: "~", _;
    noun_second_case: derived, _;
    ),
    // Полнота: 12,546 words (собака, буява, былица, масленица, царица, вата). Like Приём, except in 2 cells: gen_ pl, noun_second_case loc.
    declension!("полнота", 12546, "~а", [gender: Feminine], like Приём;
    stem lemma: "~- like полнота";
    stem gen_sg: "~- like полнота";
    gen_: _, "~";
    noun_second_case: _, derived;
    ),
```

An ending table: a slot's cells, in order, built on one stem. From `latin`, the third
declension's, which the consonant stems take (`stem gen_sg: "~- like ratiō"`: a slot is
named after the principal part it holds), and a word
that lists its stem with it (`irregular.rs`: `anas`, `anatis`… as the dictionary
writes it, `anit-`):

```rust
    // gen_sg like ratiō: 4,121 words.
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
anas	gen_sg	anit- like ratiō
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
forms it gives and the forms it gets wrong (those are listed). Laws that are one law are
stated once: Russian `ы` → `и` after a velar or a hushing letter (seven laws, one per
letter, as learned), a captured letter (`{x}`) kept:

```rust
        // [к|г|х|ш|ч|ж|щ]+ы → +и: 12,996 words (собака, зонтик, бортник, бредник), 17 forms wrong.
        law0: "{x:[к|г|х|ш|ч|ж|щ]}+ы" => "{x}+и";
        // [щ|ц|ч|ш|ж|к|г|х]+ям → +ам: 84 words (ночь, помочь, печь, рожь), 6 forms wrong.
        law2: "{x:[щ|ц|ч|ш|ж|к|г|х]}+ям" => "{x}+ам";
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
words with that ending take each declension), then the words no rule gets right, listed
by the declension they take (sorted into one lookup array when the crate is built):

```rust
        "ба" => [Полнота 241, Какао 10, Люлька 4, Порубка 1] regular,
        "ьба" => [Полнота 32, Люлька 4, Порубка 1] regular,
        "дьба" => [Полнота 6, Люлька 4] regular,
    ...
    listed {
        Приём => [
            "#абдалхан", "#авиаспорт", "#ага-хан", "#аер", "#азиат", "#айршир", "#алихан", "#аллес",
```

Proper nouns have grammars of their own, as they choose otherwise: from `english`,
a name in `-y` takes `-s` (`Kennedys`), where a noun takes `-ies`:

```rust
        "y" => [Hopewell 3091, Kelly 31, Douglas 1, Miles 1] regular,
        "ey" => [Hopewell 1366] regular,

    // Hopewell: 39,422 words (december, april, june, september, october, november).
    declension!("hopewell", 39422, "~";
    proper_noun: "~", "~s";
    ),
```

A part's cells are the language's. From `latin`, the subjunctive has no future:

```rust
/// Tense of a subjunctive: Latin has no future one (A&G §§ 184–196).
pub enum SubjunctiveTense {
    Present,
    Imperfect,
    Perfect,
    Pluperfect,
}
```

From `turkish`, a verb's 928 forms; the doubled converb (`gele gele`) is the optative's
third person written twice, a change of its own (`~` → `~ ~`), beside the verbs of
words whose second word is another verb's (`~ gElE`):

```rust
    // CONVERB_POS_PLAIN_DOUBLED, from OPTATIVE_POS_PLAIN___3SG.
    relation!(CONVERB_POS_PLAIN_DOUBLED, [OPTATIVE_POS_PLAIN___3SG], [("~", "~ EdE"), ..., ("~", "~ ~"), ...], rules {
        "" => [36 1879, 0 186, 31 33, 27 29, ...
```

From `finnish`, a verb of two words inflects its first and keeps the other, after it
(`pyydän anteeksi`, `en pyydä anteeksi`) or before it in the participles (`anteeksi
pyytävä`, `kiinni otettu`), as the dictionary's verbs put it, cell by cell:

```rust
pub(super) static KEPT: [flexion::Placement; 163] = [
    flexion::Placement::After,  // PRES_AFF_1SG
    ...
    flexion::Placement::Before, // PARTICIPLE_PRES_ACT
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
        short_endings: &["εως", "εων"],
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

A clean debug build of each crate, one at a time (one machine, `-j 4`; CI's numbers
are in its compile-cost job; every build's compiler peaked under 0.92 GB), and one call
(release: a word's candidates for one cell, over 30 words and their first 40 cells;
`cargo xtask bench <language> --pos <part>`, last measured on the grammars of PR #14),
on words the grammar was learned from and on held-out words:

| crate | source MB | build seconds | `.rlib` MB | one call, nouns | one call, verbs |
|---|---:|---:|---:|---:|---:|
| english | 0.5 | 2 | 54 | 9 / 9 µs | 10 / 10 µs |
| russian | 2.0 | 6 | 89 | 16 / 17 µs | 36 / 38 µs |
| latin | 1.4 | 5 | 92 | 21 / 23 µs | 32 / 40 µs |
| ancient-greek | 3.5 | 4 | 89 | 44 / 81 µs | 52 / 146 µs |
| sanskrit | 0.7 | 2 | 56 | 5 / 7 µs | 19 / 25 µs |
| polish | 1.2 | 4 | 77 | 13 / 25 µs | 13 / 17 µs |
| german | 1.3 | 5 | 71 | 12 / 22 µs | 16 / 24 µs |
| spanish | 0.5 | 2 | 49 | 4 / 4 µs | 9 / 12 µs |
| french | 0.3 | 2 | 54 | 4 / 4 µs | 6 / 6 µs |
| italian | 0.4 | 2 | 56 | 4 / 4 µs | 6 / 11 µs |
| finnish | 2.3 | 6 | 86 | 16 / 16 µs | – |
| turkish | 2.0 | 3 | 60 | 8 / 9 µs | – |

The parts of speech the crates had before this PR's new ones went from 13.79 MB to
12.16 MB of source (the words no rule gets right listed by what they take, laws that
are one law stated once, no cells the language hasn't); the new parts add 3.84 MB,
most of it Turkish verbs (1.6 MB: 928 forms a verb) and Finnish verbs and names.
Ancient Greek's verbs, 489 cells each (624 before the futures' and imperatives'
invented cells went), are most of its size: their paradigms (0.6 MB, 286 conjugations)
and their irregular forms (1.0 MB: 2,956 listed stems with the ending table each builds
on, and 27,718 forms no stem and table give, 33,829 before). Nothing is computed
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
