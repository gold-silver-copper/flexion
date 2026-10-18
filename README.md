<!-- Written by `cargo xtask readme` from `xtask/src/readme.md`: edit that, not this. -->
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
trainer or the runtime: [English](languages/english), [Russian](languages/russian), [Latin](languages/latin), [Ancient Greek](languages/ancient-greek), [Sanskrit](languages/sanskrit), [Polish](languages/polish), [German](languages/german), [Spanish](languages/spanish), [French](languages/french), [Italian](languages/italian), [Finnish](languages/finnish) and [Turkish](languages/turkish). Each is a config in
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
| english | verb | **97.23%** (96.8–97.6) | 97.23% | 99.57% |  | 97.79% | -0.56 (-0.87…-0.25) | 98.92% / 96.32% | 97.67% | 99.86% |  | 97.21% (3598) / 100.00% (3) / 97.36% (550) |  | 0.38% | 0.006 |
| english | adjective | **95.35%** (94.8–95.9) | 95.35% |  |  | 92.53% | +2.82 (+2.39…+3.25) | 99.05% / 93.61% | 96.30% | 99.94% |  | 95.59% (5574) / 75.00% (4) / 93.48% (661) | 99.06% (11894) | 0.06% | 0.005 |
| english | adverb | **99.01%** (98.4–99.5) | 99.01% |  |  | 98.34% | +0.66 (+0.25…+1.16) | 100.00% / 98.67% | 99.59% | 100.00% |  | 98.95% (1146) / – / 100.00% (61) | 99.67% (2398) | 0.11% | 0.005 |
| russian | noun | **94.64%** (94.0–95.2) | 94.64% | 95.73% | 97.30% | 94.72% | -0.09 (-0.52…+0.37) | 97.06% / 93.81% | 94.98% | 99.82% |  | 94.55% (2394) / 96.60% (181) / 90.08% (36) |  | 0.54% | 0.007 |
| russian | proper_noun | **90.77%** (88.5–93.0) | 90.77% | 92.59% | 96.72% | 93.60% | -2.83 (-4.13…-1.49) | 93.22% / 90.71% | 90.90% | 99.88% |  | 91.20% (331) / 100.00% (1) / 69.23% (8) |  | 2.19% | 0.025 |
| russian | adjective | **99.75%** (99.6–99.9) | 99.75% |  |  | 99.76% | -0.01 (-0.12…+0.09) | 99.71% / 99.77% | 99.76% | 99.94% |  | 99.13% (950) / 100.00% (2512) / 100.00% (4) |  | 0.07% | 0.002 |
| russian | verb | **95.74%** (95.0–96.4) | 95.74% | 98.90% | 95.47% | 96.65% | -0.91 (-1.43…-0.42) | 95.74% / 95.73% | 95.86% | 99.49% |  | 95.71% (1269) / 100.00% (29) / 81.44% (6) |  | 0.75% | 0.006 |
| russian | pronoun | **reproduced 1974 of 1974**; a new word 48.83% (42 words) | 54.26% |  |  | 54.26% | +0.00 (+0.00…+0.00) | 0.00% / 54.26% | 54.26% | 100.00% |  | 54.26% (4) / – / – |  | 30.95% | 0.290 |
| russian | numeral | **reproduced 2986 of 2986**; a new word 83.69% (77 words) | 94.12% |  |  | 100.00% | -5.88 (-32.53…+0.00) | 100.00% / 91.51% | 94.12% | 100.00% |  | 94.12% (4) / – / – |  | 24.68% | 0.006 |
| russian | adverb | **reproduced 185 of 185**; a new word 77.30% (185 words) | 66.67% |  |  | 83.33% | -16.67 (-33.33…+0.00) | 0.00% / 66.67% | 72.22% | 100.00% |  | 66.67% (18) / – / – |  | 11.89% | 0.310 |
| latin | noun | **71.83%** (69.6–74.1) | 92.93% | 98.70% | 72.64% | 59.09% | +12.74 (+10.05…+15.41) | 79.61% / 69.44% | 71.94% | 99.46% | 70.33% (984) | 71.91% (1379) / 83.25% (19) / 69.90% (167) |  | 1.16% | 0.036 |
| latin | proper_noun | **60.13%** (56.7–63.4) | 92.79% | 98.32% | 60.13% | 60.58% | -0.45 (-4.19…+3.41) | 58.77% / 60.25% | 60.62% | 99.32% | 47.79% (362) | 59.91% (801) / – / 65.22% (33) |  | 1.64% | 0.056 |
| latin | adjective | **87.82%** (86.6–89.0) | 99.23% | 99.48% |  | 71.87% | +15.95 (+14.14…+17.75) | 90.53% / 86.12% | 87.93% | 99.60% | 87.12% (2151) | 77.35% (981) / 93.97% (1684) / 87.18% (176) |  | 0.48% | 0.019 |
| latin | verb | **76.87%** (73.7–80.0) | 85.45% | 98.55% |  | 64.17% | +12.71 (+8.80…+16.80) | 83.68% / 73.94% | 77.00% | 98.43% | 88.01% (592) | 77.55% (541) / – / 72.34% (78) | 73.02% (16157) | 4.69% | 0.026 |
| latin | pronoun | **reproduced 2211 of 2211**; a new word 30.66% (80 words) | 19.08% |  |  | 25.00% | -18.66 (-47.89…+3.14) | 0.00% / 6.34% | 10.07% | 100.00% | 14.29% (7) | 8.42% (7) / – / 0.00% (2) |  | 73.75% | 0.230 |
| latin | numeral | **reproduced 3996 of 3996**; a new word 85.01% (118 words) | 100.00% |  |  | 72.73% | +0.00 (-36.36…+36.36) | 66.67% / 75.00% | 72.73% | 100.00% | 72.73% (11) | 62.50% (8) / – / 100.00% (3) |  | 0.85% | 0.303 |
| latin | adverb | **88.57%** (80.0–96.0) | 98.35% | 96.72% |  | 44.57% | +44.00 (+30.05…+57.71) | 100.00% / 88.17% | 89.14% | 100.00% | 88.89% (54) | 87.73% (58) / 100.00% (3) / 100.00% (2) |  | 5.56% | 0.075 |
| ancient-greek | noun | **74.96%** (72.1–77.8) | 89.14% | 96.10% | 75.96% | 51.28% | +25.29 (+21.30…+28.96) | 81.03% / 73.54% | 74.98% | 99.55% | 79.18% (586) | 74.79% (784) / 66.67% (3) / 82.32% (22) |  | 8.64% | 0.046 |
| ancient-greek | proper_noun | **60.28%** (55.1–65.6) | 86.95% | 91.55% | 61.72% | 47.42% | +14.45 (+8.29…+20.98) | 59.65% / 60.34% | 60.43% | 99.40% | 65.16% (221) | 60.67% (375) / – / 43.48% (8) |  | 5.14% | 0.076 |
| ancient-greek | adjective | **78.19%** (74.9–81.4) | 90.76% | 98.02% |  | 52.46% | +26.51 (+21.96…+30.76) | 82.37% / 77.38% | 78.39% | 99.17% | 84.39% (301) | 79.52% (349) / 74.94% (115) / 68.31% (10) |  | 2.78% | 0.039 |
| ancient-greek | verb | **68.30%** (63.8–72.7) | 75.88% | 81.95% |  | 44.20% | +25.27 (+19.32…+30.88) | 77.29% / 63.03% | 68.65% | 99.90% | 83.12% (160) | 68.86% (244) / 45.83% (3) / 55.25% (10) | 56.29% (4832) | 44.33% | 0.072 |
| ancient-greek | pronoun | **reproduced 1771 of 1771**; a new word 53.75% (58 words) | 40.16% |  |  | 41.96% | +7.08 (-8.14…+30.97) | 0.00% / 40.33% | 40.33% | 99.46% | 100.00% (1) | 37.37% (8) / 88.89% (1) / 0.00% (2) |  | 51.72% | 0.477 |
| ancient-greek | numeral | **reproduced 301 of 301**; a new word 66.78% (19 words) | 100.00% |  |  | 100.00% | +0.00 (+0.00…+0.00) | 0.00% / 100.00% | 100.00% | 100.00% |  | 100.00% (2) / – / – |  | 21.05% | 0.061 |
| sanskrit | noun | **92.10%** (90.8–93.4) | 92.10% |  | 97.30% | 91.70% | +0.40 (-0.73…+1.48) | 94.30% / 92.03% | 92.65% | 97.83% |  | 92.14% (563) / 100.00% (2) / 88.71% (11) |  | 1.57% | 0.015 |
| sanskrit | proper_noun | **95.15%** (90.0–99.1) | 95.15% |  | 96.34% | 86.48% | +8.67 (+3.40…+14.54) | 0.00% / 95.15% | 95.15% | 100.00% |  | 95.05% (48) / – / 100.00% (1) |  | 2.13% | 0.030 |
| sanskrit | adjective | **91.86%** (89.5–94.0) | 91.86% |  |  | 90.11% | +1.75 (+0.12…+3.52) | 83.33% / 92.24% | 91.89% | 100.00% |  | 91.55% (183) / 100.00% (6) / 100.00% (1) |  | 3.72% | 0.020 |
| sanskrit | numeral | **reproduced 1656 of 1656**; a new word 90.52% (65 words) | 63.64% |  |  | 92.42% | -28.79 (-55.13…-0.93) | 100.00% / 50.00% | 63.64% | 100.00% |  | 63.64% (9) / – / – |  | 7.69% | 0.349 |
| sanskrit | verb | **89.61%** (85.1–93.8) | 89.61% | 90.33% | 89.61% | 89.93% | -0.32 (-2.79…+2.02) | 95.65% / 89.03% | 90.12% | 100.00% | 100.00% (2) | 91.23% (71) / 55.52% (17) / – |  | 23.63% | 0.041 |
| sanskrit | pronoun | **reproduced 987 of 987**; a new word 68.90% (15 words) | 69.57% |  |  | 70.53% | -0.97 (-3.17…+0.00) | 0.00% / 69.57% | 69.57% | 100.00% |  | 69.57% (3) / – / – |  | 46.67% | 0.298 |
| polish | noun | **93.44%** (92.9–94.0) | 93.44% | 95.83% | 96.23% | 93.34% | +0.10 (-0.27…+0.46) | 95.17% / 93.11% | 93.72% | 99.23% |  | 92.46% (3110) / 98.91% (809) / 88.52% (62) |  | 1.80% | 0.002 |
| polish | proper_noun | **94.00%** (93.3–94.7) | 94.00% | 95.03% | 94.92% | 93.74% | +0.26 (-0.26…+0.81) | 98.13% / 93.37% | 94.19% | 97.11% |  | 94.12% (1760) / 38.46% (2) / 50.00% (4) |  | 1.36% | 0.005 |
| polish | adjective | **99.94%** (99.9–100.0) | 99.94% |  |  | 99.99% | -0.05 (-0.07…-0.03) | 99.97% / 99.93% | 99.94% | 100.00% |  | 99.92% (1147) / 100.00% (239) / 100.00% (8) |  | 0.36% | 0.000 |
| polish | verb | **97.17%** (96.5–97.9) | 97.17% | 98.38% | 97.33% | 97.66% | -0.49 (-0.99…+0.02) | 95.72% / 97.49% | 97.45% | 99.79% |  | 97.20% (1230) / 100.00% (2) / 93.03% (11) |  | 1.35% | 0.006 |
| polish | pronoun | **reproduced 1550 of 1550**; a new word 42.65% (28 words) | 81.64% |  |  | 81.64% | +0.00 (+0.00…+0.00) | 0.00% / 81.64% | 81.64% | 100.00% |  | 75.00% (4) / 100.00% (1) / 89.83% (1) |  | 60.71% | 0.656 |
| polish | numeral | **reproduced 189 of 189**; a new word 3.17% (5 words) | – | – | – | – | – | – | – | – | – | – | – | 0.00% | – |
| polish | adverb | **93.75%** (87.5–98.4) | 93.75% | 100.00% |  | 95.31% | -1.56 (-7.81…+3.12) | 100.00% / 93.65% | 93.75% | 100.00% |  | 93.65% (63) / – / 100.00% (1) |  | 5.23% | 0.068 |
| german | noun | **93.75%** (93.3–94.2) | 93.75% | 99.80% | 94.23% | 94.77% | -1.02 (-1.33…-0.73) | 96.86% / 90.00% | 94.53% | 99.69% |  | 93.57% (5249) / 98.90% (730) / 87.87% (344) |  | 0.49% | 0.008 |
| german | numeral | **reproduced 126 of 126**; a new word 100.00% (18 words) | 100.00% |  | 100.00% | 100.00% | +0.00 (+0.00…+0.00) | 0.00% / 100.00% | 100.00% | 100.00% |  | 100.00% (2) / – / – |  | 0.00% | 0.000 |
| german | proper_noun | **91.13%** (88.1–93.9) | 91.13% | 99.77% | 91.13% | 90.66% | +0.48 (-1.86…+2.78) | 100.00% / 90.64% | 92.72% | 99.21% |  | 90.98% (296) / 100.00% (1) / 95.00% (12) |  | 0.28% | 0.020 |
| german | adjective | **98.90%** (98.4–99.4) | 98.90% | 99.88% |  | 99.02% | -0.12 (-0.59…+0.35) | 98.86% / 98.92% | 99.18% | 100.00% |  | 98.85% (1410) / 100.00% (28) / 100.00% (49) | 96.90% (548) | 0.07% | 0.003 |
| german | verb | **95.21%** (94.2–96.1) | 95.21% | 96.02% | 95.32% | 73.16% | +22.05 (+20.30…+23.83) | 95.91% / 94.43% | 95.31% | 99.25% |  | 95.04% (936) / – / 99.83% (34) | 91.76% (9057) | 5.07% | 0.011 |
| german | determiner | **reproduced 532 of 532**; a new word 82.52% (36 words) | 40.00% |  |  | 40.00% | +0.00 (+0.00…+0.00) | 0.00% / 40.00% | 40.00% | 100.00% |  | 40.00% (3) / – / – |  | 13.89% | 0.598 |
| spanish | noun | **98.67%** (98.4–98.9) | 98.67% |  | 98.67% | 98.67% | +0.00 (-0.20…+0.20) | 99.85% / 98.12% | 98.88% | 99.98% |  | 98.74% (5245) / 100.00% (583) / 94.02% (251) |  | 0.11% | 0.003 |
| spanish | adjective | **98.50%** (98.1–98.9) | 98.50% | 99.67% |  | 98.60% | -0.10 (-0.35…+0.14) | 99.84% / 98.04% | 98.61% | 100.00% |  | 98.49% (2284) / 100.00% (68) / 97.12% (47) |  | 0.21% | 0.003 |
| spanish | verb | **98.99%** (98.6–99.3) | 98.99% | 99.74% |  | 99.28% | -0.30 (-0.59…-0.01) | 99.32% / 98.77% | 99.01% | 99.98% |  | 99.04% (879) / 100.00% (4) / 97.86% (46) |  | 1.68% | 0.004 |
| spanish | determiner | **reproduced 173 of 173**; a new word 80.35% (78 words) | 90.91% |  |  | 100.00% | -9.09 (-30.00…+0.00) | 100.00% / 90.48% | 90.91% | 100.00% | 0.00% (1) | 77.78% (4) / 100.00% (2) / 100.00% (3) |  | 11.54% | 0.094 |
| spanish | adverb | **reproduced 8 of 8**; a new word 12.50% (7 words) | – | – | – | – | – | – | – | – | – | – | – | 42.86% | – |
| french | noun | **97.59%** (97.1–98.0) | 97.59% |  | 97.59% | 97.03% | +0.56 (+0.23…+0.90) | 96.88% / 97.85% | 97.98% | 99.88% |  | 97.97% (3898) / 99.18% (245) / 86.78% (174) |  | 0.65% | 0.004 |
| french | adjective | **97.53%** (96.9–98.2) | 97.53% | 99.95% |  | 97.75% | -0.22 (-0.65…+0.24) | 98.85% / 97.16% | 97.61% | 100.00% |  | 97.94% (1781) / – / 85.06% (58) |  | 0.28% | 0.009 |
| french | verb | **98.70%** (98.1–99.3) | 98.70% | 99.48% |  | 99.28% | -0.59 (-1.06…-0.22) | 99.63% / 98.40% | 98.84% | 100.00% |  | 98.74% (724) / – / 97.08% (19) |  | 0.96% | 0.004 |
| french | participle | **99.84%** (99.5–100.0) | 99.84% |  |  | 99.52% | +0.32 (-0.32…+0.96) | 100.00% / 99.80% | 99.84% | 100.00% |  | 100.00% (1) / 99.84% (626) / – |  | 0.03% | 0.005 |
| french | determiner | **reproduced 103 of 103**; a new word 46.60% (49 words) | 46.15% |  |  | 69.23% | -23.08 (-64.29…+0.00) | 0.00% / 46.15% | 46.15% | 100.00% |  | 46.15% (6) / – / – |  | 22.45% | 0.538 |
| french | adverb | **reproduced 4 of 4**; a new word 0.00% (2 words) | – | – | – | – | – | – | – | – | – | – | – | 0.00% | – |
| italian | noun | **95.82%** (95.3–96.3) | 95.82% |  | 96.39% | 95.09% | +0.74 (+0.31…+1.16) | 95.91% / 95.76% | 95.96% | 99.77% |  | 95.85% (5862) / 98.57% (210) / 93.44% (320) |  | 0.19% | 0.003 |
| italian | adjective | **98.46%** (98.0–98.8) | 98.46% | 99.78% |  | 98.46% | +0.00 (-0.28…+0.28) | 99.36% / 97.96% | 98.49% | 100.00% |  | 98.48% (2514) / 100.00% (42) / 96.86% (75) |  | 0.19% | 0.010 |
| italian | verb | **99.05%** (98.7–99.4) | 99.05% | 99.53% | 99.05% | 99.22% | -0.17 (-0.48…+0.14) | 98.93% / 99.12% | 99.12% | 99.96% |  | 99.34% (1188) / 100.00% (9) / 95.12% (89) |  | 1.77% | 0.004 |
| italian | participle | **100.00%** (100.0–100.0) | 100.00% |  |  | 100.00% | +0.00 (+0.00…+0.00) | 100.00% / 100.00% | 100.00% | 100.00% |  | 100.00% (11) / 100.00% (810) / – |  | 0.02% | 0.000 |
| italian | determiner | **reproduced 117 of 117**; a new word 88.03% (45 words) | 81.82% |  |  | 86.36% | -4.55 (-13.64…+0.00) | 100.00% / 78.95% | 81.82% | 100.00% |  | 81.82% (8) / – / – |  | 13.33% | 0.182 |
| italian | adverb | **reproduced 10 of 10**; a new word 0.00% (8 words) | 0.00% | 0.00% |  | 0.00% | +0.00 (+0.00…+0.00) | 0.00% / 0.00% | 0.00% | 100.00% |  | 0.00% (1) / – / – |  | 75.00% | 1.000 |
| finnish | noun | **98.07%** (97.9–98.3) | 98.07% | 99.34% |  | 98.29% | -0.22 (-0.41…-0.04) | 98.94% / 96.33% | 98.24% | 99.98% |  | 98.13% (12649) / 100.00% (7) / 94.30% (213) |  | 1.01% | 0.010 |
| finnish | pronoun | **reproduced 1484 of 1484**; a new word 16.51% (75 words) | 6.42% |  |  | 9.63% | -3.21 (-5.92…-0.99) | 0.00% / 8.76% | 6.42% | 100.00% |  | 7.89% (7) / 0.00% (1) / 0.00% (1) |  | 69.33% | 0.342 |
| finnish | numeral | **reproduced 1873 of 1873**; a new word 31.50% (75 words) | 32.14% |  |  | 43.30% | -11.16 (-41.96…+19.73) | 0.00% / 36.18% | 32.14% | 100.00% |  | 23.62% (8) / – / 100.00% (1) |  | 44.00% | 0.288 |
| finnish | proper_noun | **93.82%** (92.5–95.1) | 93.82% | 99.01% |  | 91.77% | +2.05 (+0.72…+3.35) | 97.97% / 92.76% | 94.32% | 99.88% |  | 93.92% (911) / – / 75.61% (6) |  | 2.60% | 0.015 |
| finnish | adjective | **97.50%** (96.9–98.1) | 97.50% |  |  | 97.41% | +0.08 (-0.58…+0.76) | 99.24% / 96.42% | 97.54% | 100.00% |  | 97.42% (1570) / 98.54% (104) / 97.81% (47) |  | 0.83% | 0.011 |
| finnish | verb | **98.50%** (98.0–98.9) | 98.50% | 99.47% |  | 97.56% | +0.94 (+0.26…+1.69) | 99.43% / 98.27% | 98.50% | 99.93% |  | 98.50% (1103) / 100.00% (59) / 96.86% (57) | 98.97% (117603) | 3.97% | 0.003 |
| finnish | adverb | **96.67%** (94.5–98.6) | 96.67% | 98.57% |  | 97.39% | -0.71 (-1.67…+0.00) | 100.00% / 96.47% | 96.67% | 100.00% |  | 96.67% (211) / – / – |  | 2.32% | 0.017 |
| finnish | postposition | **reproduced 199 of 199**; a new word 81.91% (41 words) | 65.22% |  |  | 78.26% | -13.04 (-30.43…+0.00) | 0.00% / 65.22% | 65.22% | 100.00% |  | 65.22% (5) / – / – |  | 4.88% | 0.177 |
| turkish | noun | **96.62%** (95.7–97.5) | 96.62% | 99.52% |  | 93.27% | +3.35 (+2.45…+4.33) | 98.92% / 96.39% | 96.67% | 99.88% |  | 96.29% (977) / 100.00% (72) / 99.51% (16) |  | 1.11% | 0.013 |
| turkish | pronoun | **reproduced 133 of 133**; a new word 14.29% (13 words) | 0.00% |  |  | 90.91% | -90.91 (-90.91…-90.91) | 0.00% / 0.00% | 0.00% | 100.00% |  | 0.00% (1) / – / – |  | 61.54% | 1.000 |
| turkish | numeral | **reproduced 77 of 77**; a new word 37.66% (7 words) | 31.82% |  |  | 0.00% | +31.82 (+0.00…+63.64) | 0.00% / 31.82% | 31.82% | 100.00% |  | 31.82% (2) / – / – |  | 42.86% | 0.682 |
| turkish | proper_noun | **90.94%** (85.4–95.6) | 90.94% | 97.74% |  | 93.24% | -2.31 (-6.43…+1.59) | 100.00% / 90.64% | 91.20% | 100.00% |  | 90.94% (145) / – / – | 100.00% (23) | 1.97% | 0.046 |
| turkish | adjective | **reproduced 4116 of 4116**; a new word 98.13% (60 words, by tenth) | 86.75% | 100.00% |  | 81.93% | +4.82 (+0.00…+14.46) | 0.00% / 86.75% | 86.75% | 100.00% |  | 86.75% (3) / – / – |  | 1.11% | 0.115 |
| turkish | verb | **98.90%** (97.8–99.8) | 98.90% | 99.92% |  | 98.68% | +0.22 (-1.06…+1.38) | 98.63% / 99.14% | 98.90% | 100.00% |  | 98.73% (198) / 100.00% (30) / – | 99.53% (44149) | 6.33% | 0.006 |

*Where the errors are*: the wrong first answers by what went wrong. *Class*: the right
form is among the candidates, from another paradigm or reading (the grammar chose
wrong); *marks*: the same letters but for a mark the spelling folds (vowel length, a breathing; not a letter written with a combining mark, as a Devanagari vowel sign);
*accent*: the right letters, the accent elsewhere; *stem* and *ending*: where it goes
wrong; *variant*: another of the forms the dictionary lists in the cell; *homograph*:
another dictionary word spelled alike has this form; *gap*: no form at all (none, in
every part). *By block*: top-1 averaged over the part's groups of cells (a Turkish
verb's negative abilitative forms count as much as its plain ones). *Reproduced*: the
shipped grammar asked again for its own words (a sample of them), and *complete*: of their
cells, those with a form. Closed classes (pronouns, numerals) are scored both ways:
their held-out tenth is a handful of words.

| language | part of speech | wrong forms | class | marks | accent | stem | ending | variant | homograph | gap | by block | reproduced | complete |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| english | noun | 671 | 63% | 0% | 0% | 4% | 2% | 28% | 2% | 0% | 97.17% | 100.00% | 100.00% |
| english | proper_noun | 28 | 46% | 0% | 0% | 4% | 0% | 50% | 0% | 0% | 99.35% | 100.00% | 100.00% |
| english | verb | 459 | 74% | 0% | 0% | 3% | 3% | 16% | 4% | 0% | 97.23% | 100.00% | 100.00% |
| english | adjective | 580 | 79% | 0% | 0% | 0% | 0% | 20% | 1% | 0% | 95.35% | 100.00% | 100.00% |
| english | adverb | 24 | 42% | 0% | 0% | 0% | 0% | 58% | 0% | 0% | 99.01% | 100.00% | 100.00% |
| russian | noun | 1482 | 81% | 0% | 0% | 2% | 7% | 6% | 3% | 0% | 94.64% | 100.00% | 100.00% |
| russian | proper_noun | 225 | 88% | 0% | 0% | 3% | 7% | 1% | 1% | 0% | 90.77% | 100.00% | 100.00% |
| russian | adjective | 412 | 70% | 0% | 0% | 0% | 5% | 0% | 25% | 0% | 98.11% | 100.00% | 100.00% |
| russian | verb | 876 | 75% | 0% | 0% | 8% | 5% | 3% | 9% | 0% | 96.02% | 99.98% | 100.00% |
| russian | pronoun | 86 | 95% | 0% | 0% | 0% | 5% | 0% | 0% | 0% | 54.26% | 100.00% | 100.00% |
| russian | numeral | 9 | 0% | 0% | 0% | 0% | 100% | 0% | 0% | 0% | 94.12% | 100.00% | 100.00% |
| russian | adverb | 6 | 17% | 0% | 0% | 67% | 0% | 17% | 0% | 0% | – | 100.00% | 100.00% |
| latin | noun | 5031 | 16% | 76% | 0% | 2% | 4% | 0% | 2% | 0% | 71.83% | 100.00% | 100.00% |
| latin | proper_noun | 2041 | 7% | 83% | 0% | 2% | 5% | 1% | 2% | 0% | 58.19% | 99.97% | 100.00% |
| latin | adjective | 12373 | 2% | 92% | 0% | 0% | 2% | 1% | 3% | 0% | 81.95% | 99.95% | 100.00% |
| latin | verb | 17832 | 46% | 38% | 0% | 5% | 4% | 1% | 6% | 0% | 77.03% | 100.00% | 100.00% |
| latin | pronoun | 251 | 24% | 15% | 0% | 8% | 49% | 4% | 0% | 0% | 6.34% | 100.00% | 100.00% |
| latin | numeral | 108 | 0% | 100% | 0% | 0% | 0% | 0% | 0% | 0% | 72.73% | 100.00% | 100.00% |
| latin | adverb | 20 | 0% | 90% | 0% | 0% | 5% | 5% | 0% | 0% | 88.57% | 100.00% | 100.00% |
| ancient-greek | noun | 2875 | 27% | 58% | 6% | 2% | 6% | 0% | 1% | 0% | 74.96% | 99.89% | 100.00% |
| ancient-greek | proper_noun | 800 | 14% | 74% | 4% | 2% | 6% | 0% | 0% | 0% | 60.28% | 99.88% | 100.00% |
| ancient-greek | adjective | 4835 | 28% | 58% | 3% | 4% | 4% | 1% | 1% | 0% | 80.85% | 99.84% | 100.00% |
| ancient-greek | verb | 21862 | 20% | 42% | 4% | 19% | 14% | 1% | 0% | 0% | 66.52% | 99.06% | 100.00% |
| ancient-greek | pronoun | 219 | 6% | 0% | 15% | 1% | 78% | 0% | 0% | 0% | 40.33% | 100.00% | 100.00% |
| ancient-greek | numeral | 0 | – | – | – | – | – | – | – | – | 100.00% | 100.00% | 100.00% |
| sanskrit | noun | 1090 | 50% | 0% | 0% | 10% | 6% | 7% | 27% | 0% | 92.10% | 100.00% | 100.00% |
| sanskrit | proper_noun | 57 | 33% | 0% | 0% | 26% | 40% | 0% | 0% | 0% | 95.15% | 100.00% | 100.00% |
| sanskrit | adjective | 1095 | 83% | 0% | 0% | 6% | 10% | 0% | 0% | 0% | 91.96% | 100.00% | 100.00% |
| sanskrit | numeral | 96 | 0% | 0% | 0% | 0% | 100% | 0% | 0% | 0% | 63.64% | 100.00% | 100.00% |
| sanskrit | verb | 746 | 17% | 0% | 0% | 22% | 56% | 5% | 0% | 0% | 78.66% | 100.00% | 100.00% |
| sanskrit | pronoun | 63 | 0% | 0% | 0% | 0% | 100% | 0% | 0% | 0% | 69.57% | 100.00% | 100.00% |
| polish | noun | 2911 | 69% | 0% | 0% | 3% | 12% | 4% | 11% | 0% | 93.44% | 99.99% | 100.00% |
| polish | proper_noun | 1284 | 39% | 0% | 0% | 2% | 7% | 3% | 48% | 0% | 94.00% | 100.00% | 100.00% |
| polish | adjective | 51 | 78% | 0% | 0% | 8% | 10% | 4% | 0% | 0% | 85.61% | 100.00% | 100.00% |
| polish | verb | 1518 | 67% | 0% | 0% | 5% | 11% | 10% | 8% | 0% | 96.82% | 100.00% | 100.00% |
| polish | pronoun | 65 | 0% | 0% | 0% | 0% | 100% | 0% | 0% | 0% | 81.64% | 100.00% | 100.00% |
| polish | numeral | – | – | – | – | – | – | – | – | – | – | 100.00% | 100.00% |
| polish | adverb | 8 | 0% | 0% | 0% | 0% | 100% | 0% | 0% | 0% | 93.75% | 100.00% | 100.00% |
| german | noun | 2503 | 67% | 0% | 0% | 4% | 12% | 12% | 4% | 0% | 93.75% | 100.00% | 100.00% |
| german | numeral | 0 | – | – | – | – | – | – | – | – | – | 100.00% | 100.00% |
| german | proper_noun | 112 | 71% | 0% | 0% | 0% | 7% | 18% | 4% | 0% | 91.13% | 100.00% | 100.00% |
| german | adjective | 1372 | 64% | 0% | 0% | 0% | 11% | 25% | 0% | 0% | 98.24% | 100.00% | 100.00% |
| german | verb | 2423 | 57% | 0% | 0% | 19% | 7% | 2% | 15% | 0% | 94.24% | 100.00% | 100.00% |
| german | determiner | 27 | 56% | 0% | 0% | 22% | 22% | 0% | 0% | 0% | 40.00% | 100.00% | 100.00% |
| spanish | noun | 81 | 74% | 0% | 0% | 5% | 4% | 16% | 1% | 0% | 98.67% | 100.00% | 100.00% |
| spanish | adjective | 108 | 81% | 0% | 0% | 5% | 7% | 7% | 0% | 0% | 98.57% | 100.00% | 100.00% |
| spanish | verb | 678 | 43% | 0% | 0% | 33% | 23% | 2% | 0% | 0% | 98.83% | 100.00% | 100.00% |
| spanish | determiner | 2 | 100% | 0% | 0% | 0% | 0% | 0% | 0% | 0% | – | 100.00% | 100.00% |
| spanish | adverb | – | – | – | – | – | – | – | – | – | – | 100.00% | 100.00% |
| french | noun | 104 | 75% | 0% | 0% | 5% | 1% | 16% | 3% | 0% | 97.59% | 100.00% | 100.00% |
| french | adjective | 136 | 81% | 0% | 0% | 3% | 13% | 3% | 0% | 0% | 97.57% | 100.00% | 100.00% |
| french | verb | 454 | 75% | 0% | 0% | 0% | 14% | 11% | 0% | 0% | 98.81% | 100.00% | 100.00% |
| french | participle | 3 | 100% | 0% | 0% | 0% | 0% | 0% | 0% | 0% | 99.84% | 100.00% | 100.00% |
| french | determiner | 7 | 0% | 0% | 0% | 29% | 71% | 0% | 0% | 0% | – | 100.00% | 100.00% |
| french | adverb | – | – | – | – | – | – | – | – | – | – | 100.00% | 100.00% |
| italian | noun | 267 | 85% | 0% | 0% | 6% | 1% | 3% | 5% | 0% | 95.82% | 100.00% | 100.00% |
| italian | adjective | 122 | 90% | 0% | 0% | 3% | 4% | 2% | 0% | 0% | 96.56% | 100.00% | 100.00% |
| italian | verb | 665 | 48% | 0% | 0% | 22% | 19% | 7% | 4% | 0% | 99.04% | 100.00% | 100.00% |
| italian | participle | 0 | – | – | – | – | – | – | – | – | 100.00% | 100.00% | 100.00% |
| italian | determiner | 4 | 25% | 0% | 0% | 50% | 25% | 0% | 0% | 0% | – | 100.00% | 100.00% |
| italian | adverb | 1 | 0% | 0% | 0% | 0% | 100% | 0% | 0% | 0% | – | 100.00% | 100.00% |
| finnish | noun | 36661 | 57% | 0% | 0% | 5% | 28% | 9% | 1% | 0% | 98.05% | 100.00% | 100.00% |
| finnish | pronoun | 175 | 4% | 0% | 0% | 14% | 82% | 0% | 0% | 0% | 6.42% | 100.00% | 100.00% |
| finnish | numeral | 152 | 18% | 0% | 0% | 27% | 55% | 0% | 0% | 0% | 32.14% | 100.00% | 100.00% |
| finnish | proper_noun | 7391 | 73% | 0% | 0% | 4% | 13% | 8% | 2% | 0% | 93.76% | 100.00% | 100.00% |
| finnish | adjective | 1112 | 75% | 0% | 0% | 2% | 21% | 2% | 0% | 0% | 97.42% | 100.00% | 100.00% |
| finnish | verb | 2945 | 54% | 0% | 0% | 0% | 41% | 0% | 5% | 0% | 98.33% | 100.00% | 100.00% |
| finnish | adverb | 14 | 64% | 0% | 0% | 14% | 21% | 0% | 0% | 0% | 96.67% | 100.00% | 100.00% |
| finnish | postposition | 8 | 75% | 0% | 0% | 0% | 25% | 0% | 0% | 0% | – | 100.00% | 100.00% |
| turkish | noun | 2402 | 73% | 0% | 0% | 11% | 7% | 1% | 7% | 0% | 96.93% | 100.00% | 100.00% |
| turkish | pronoun | 11 | 0% | 0% | 0% | 0% | 100% | 0% | 0% | 0% | – | 100.00% | 100.00% |
| turkish | numeral | 15 | 0% | 0% | 0% | 0% | 100% | 0% | 0% | 0% | – | 100.00% | 100.00% |
| turkish | proper_noun | 342 | 75% | 0% | 0% | 10% | 13% | 3% | 0% | 0% | 91.56% | 100.00% | 100.00% |
| turkish | adjective | 33 | 100% | 0% | 0% | 0% | 0% | 0% | 0% | 0% | 88.51% | 100.00% | 100.00% |
| turkish | verb | 2330 | 94% | 0% | 0% | 0% | 6% | 0% | 0% | 0% | 98.95% | 100.00% | 100.00% |

*Compatible with stated length* (Ancient Greek): the dictionary marks a vowel's length both
ways, so a bare α, ι or υ whose table never marks it otherwise leaves its length unstated, and
an answer that is the dictionary's form but for a length mark there is compatible with it. A
second score, for the grammar and the floor alike; top-1 stays the score.

| language | part of speech | top-1 | compatible with stated length | floor | floor, compatible | against the floor, compatible |
|---|---|---:|---:|---:|---:|---:|
| ancient-greek | noun | 74.96% | 80.97% | 51.28% | 55.25% | +27.39 (+23.79…+30.89) |
| ancient-greek | proper_noun | 60.28% | 78.35% | 47.42% | 58.69% | +21.45 (+16.26…+26.92) |
| ancient-greek | adjective | 78.19% | 84.96% | 52.46% | 57.70% | +28.08 (+23.65…+32.43) |
| ancient-greek | verb | 68.30% | 74.83% | 44.20% | 46.20% | +29.88 (+24.47…+35.06) |
| ancient-greek | pronoun | 40.33% | 40.33% | 41.96% | 41.96% | +7.08 (-8.14…+30.97) |
| ancient-greek | numeral | 100.00% | 100.00% | 100.00% | 100.00% | +0.00 (+0.00…+0.00) |

*The shipped grammar and the scored one*: the grammar a crate ships is learned on every word,
the one scored here on all but this tenth. Their sound laws (with tiers) and irregular forms side
by side; a law in one and not the other is listed in the crate's README.

| language | part of speech | sound laws, shipped / scored | the same laws | irregular forms, shipped / scored |
|---|---|---:|---:|---:|
| english | noun | 0 / 0 | 0 | 242 / 230 |
| english | proper_noun | 0 / 0 | 0 | 20 / 19 |
| english | verb | 16 / 14 | 10 | 147 / 134 |
| english | adjective | 4 / 4 | 4 | 41 / 39 |
| english | adverb | 2 / 2 | 2 | 16 / 16 |
| russian | noun | 10 / 14 | 7 | 2167 / 2134 |
| russian | proper_noun | 2 / 3 | 2 | 158 / 114 |
| russian | adjective | 2 / 2 | 2 | 123 / 126 |
| russian | verb | 50 / 50 | 32 | 605 / 574 |
| russian | pronoun | 0 / 0 | 0 | 399 / 396 |
| russian | numeral | 5 / 4 | 4 | 129 / 128 |
| russian | adverb | 0 / 0 | 0 | 22 / 23 |
| latin | noun | 20 / 18 | 16 | 224 / 223 |
| latin | proper_noun | 10 / 9 | 8 | 154 / 137 |
| latin | adjective | 11 / 10 | 9 | 212 / 218 |
| latin | verb | 41 / 39 | 29 | 495 / 466 |
| latin | pronoun | 0 / 0 | 0 | 843 / 665 |
| latin | numeral | 0 / 0 | 0 | 6 / 6 |
| latin | adverb | 1 / 0 | 0 | 23 / 27 |
| ancient-greek | noun | 5 / 5 | 1 | 1992 / 1596 |
| ancient-greek | proper_noun | 1 / 1 | 1 | 680 / 645 |
| ancient-greek | adjective | 3 / 3 | 2 | 3578 / 1983 |
| ancient-greek | verb | 22 / 19 | 11 | 25869 / 23431 |
| ancient-greek | pronoun | 0 / 0 | 0 | 415 / 289 |
| ancient-greek | numeral | 0 / 0 | 0 | 22 / 22 |
| sanskrit | noun | 4 / 1 | 1 | 199 / 283 |
| sanskrit | proper_noun | 0 / 0 | 0 | 72 / 51 |
| sanskrit | adjective | 2 / 4 | 0 | 413 / 397 |
| sanskrit | numeral | 0 / 0 | 0 | 44 / 61 |
| sanskrit | verb | 5 / 4 | 1 | 1254 / 1135 |
| sanskrit | pronoun | 0 / 0 | 0 | 28 / 12 |
| polish | noun | 39 / 39 | 33 | 880 / 858 |
| polish | proper_noun | 48 / 46 | 36 | 294 / 302 |
| polish | adjective | 17 / 17 | 16 | 71 / 54 |
| polish | verb | 30 / 31 | 19 | 150 / 250 |
| polish | pronoun | 0 / 0 | 0 | 53 / 6 |
| polish | numeral | 0 / 0 | 0 | 0 / 0 |
| polish | adverb | 5 / 5 | 5 | 37 / 37 |
| german | noun | 49 / 47 | 30 | 275 / 270 |
| german | numeral | 1 / 1 | 1 | 0 / 0 |
| german | proper_noun | 1 / 1 | 0 | 11 / 11 |
| german | adjective | 5 / 4 | 2 | 16 / 26 |
| german | verb | 4 / 4 | 3 | 4485 / 7181 |
| german | determiner | 0 / 0 | 0 | 39 / 34 |
| spanish | noun | 1 / 0 | 0 | 65 / 60 |
| spanish | adjective | 17 / 19 | 13 | 39 / 38 |
| spanish | verb | 23 / 23 | 22 | 381 / 359 |
| spanish | determiner | 0 / 0 | 0 | 12 / 12 |
| spanish | adverb | 0 / 0 | 0 | 4 / 4 |
| french | noun | 0 / 0 | 0 | 280 / 255 |
| french | adjective | 5 / 5 | 3 | 61 / 58 |
| french | verb | 15 / 17 | 13 | 204 / 197 |
| french | participle | 0 / 0 | 0 | 2 / 2 |
| french | determiner | 1 / 1 | 1 | 14 / 12 |
| french | adverb | 0 / 0 | 0 | 0 / 0 |
| italian | noun | 0 / 0 | 0 | 88 / 82 |
| italian | adjective | 1 / 3 | 1 | 48 / 41 |
| italian | verb | 18 / 17 | 15 | 737 / 804 |
| italian | participle | 0 / 0 | 0 | 3 / 3 |
| italian | determiner | 0 / 0 | 0 | 8 / 8 |
| italian | adverb | 0 / 0 | 0 | 8 / 7 |
| finnish | noun | 22 / 22 | 15 | 8352 / 8711 |
| finnish | pronoun | 2 / 2 | 0 | 406 / 339 |
| finnish | numeral | 0 / 0 | 0 | 271 / 250 |
| finnish | proper_noun | 23 / 25 | 9 | 1387 / 1238 |
| finnish | adjective | 11 / 9 | 8 | 662 / 434 |
| finnish | verb | 27 / 21 | 17 | 5539 / 5133 |
| finnish | adverb | 0 / 0 | 0 | 69 / 65 |
| finnish | postposition | 1 / 1 | 1 | 4 / 5 |
| turkish | noun | 4 / 4 | 3 | 149 / 127 |
| turkish | pronoun | 0 / 0 | 0 | 18 / 23 |
| turkish | numeral | 0 / 0 | 0 | 5 / 6 |
| turkish | proper_noun | 3 / 3 | 3 | 72 / 56 |
| turkish | adjective | 4 / 4 | 3 | 99 / 127 |
| turkish | verb | 8 / 7 | 4 | 352 / 337 |

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
  spellings. But a stem's own lengths are lexical, so a root its training words agree on
  (two or more, nine in ten) where the rules spell it otherwise is listed and read first
  (`ROOTS` in `marks.rs`). Every form of every training word votes on its root, one vote a
  word (a form-of page with the word whose table lists it), not only its lemma: on the
  development tenth nouns +0.61, on a tenth no decision looked at +0.89; adjectives +0.2
  (PR #21). A root no other word shares stays a guess. The vowel-length rules learn from
  every part's words, proper nouns and adverbs too, which took nouns from 70.1% to 71.1%,
  adjectives from 87.3% to 87.5% and verbs from 76.9% to 77.5% on this tenth (PR #19; on
  the development tenths verbs lost 0.41, within chance, and on tenth 9 0.61, within
  chance). Typed in full the grammars reach 92.93% (nouns), 99.23% (adjectives)
  and 85.45% (verbs), and with the principal parts 98.70%, 99.48% and
  98.55%. The perfect passive is the phrase it is (`amātus sum`, `amātum esse`,
  `hortātus sum`): the verbs' *phrases*, 73.02% right as typed from the headword.
- **Ancient Greek.** A circumflex on a diphthong's second letter (`ἀριθμοῦ`) was read as
  vowel length, which split declensions in two; read as the diphthong's, nouns went from
  133 to 87 declensions and +3.8 points, adjectives +3.1 (development tenth, PR #16). The augment
  (a law where the form's first letters meet the stem: `ἐ‹ἀ → ἠ`), the perfect's
  reduplication (`%ε~κα`) and the augment behind a preverb (`ἀπο + ἐ → ἀπε`) are laws.
  The augment is one law for every vowel it meets, those few verbs show too (`ὑστέρουν`,
  `ηὔξανον`, `ἐνῴκουν`): verbs +1.3 points from the headword on the development tenth
  (PR #21).
  A dictionary leaves a short α, ι or υ bare as often as it writes its breve; a second
  score, *compatible with stated length* (its table above), counts an answer that differs
  from such a form only by a length mark the word's table never contradicts. Top-1 stays
  the score. The perfect middle-passive subjunctive and optative are phrases (`λελυμένος ὦ`), each
  word keeping its own accent. What's left in the verbs is mostly stems: a verb's aorist
  or perfect stem (`ἀγγέλλω` → `ἠγγέλθην`) follows from no ending, so a new verb's is a
  guess; known verbs list it with the ending table it takes, and a principal part given
  fixes its stem (81.95%). A preverb kept in front moves the accent back only in a cell
  whose accent recedes, counting the optative's `-οι` long (`προφέροι`; Smyth §§ 169,
  425–426); a breathing stays at the word's start (`προοισθήσεσθαι`); `-εως`, `-εων`
  count short for the accent (`πόλεως`, § 163a); a form that needs an accent always has
  one. With the futures' and imperatives' invented cells gone (PR #19), nouns went from
  71.9% to 74.8%, adjectives from 77.3% to 78.0%, verbs from 65.5% to 68.0%, and the
  verbs' irregular lines from 52.4% to 46.7%.
- **Turkish verbs.** Their suffix vowels follow the vowel before them as written: one law
  on the vowels and the space (` a e i o u ö ü ı`). Learned on every word it used to take
  `b`, `d`, `g`, `v` for triggers (some paradigms write a `t` where the others write `a` or
  `e`, and a class with a consonant read every letter); 32,526 forms were listed to patch
  it, and the grammar shipped was not the one scored. Since PR #21 a class of vowels reads
  vowels: the verbs' irregular lines went from 59.04% to 6.33% (now 6.33%, with
  352 listed forms).
- **Sanskrit.** The `n` → `ṇ` rule is a law (`तर्काणाम्`), and a noun's gender, given,
  takes it from 92.10% to 97.30%. The verbs are few (a tenth is
  88 words). Reading a root's tense pages (a perfect's, an aorist's) into its verb was tried in PR #19: fewer
  classes (59 → 34), but 1.9 points lost, so they stay apart.
- **Closed classes** (pronouns, numerals, determiners, postpositions) and parts with fewer
  than 30 held-out words are a handful of words each, mostly listed: their headline is what
  their grammar does with its own words (*reproduced N of N*), beside a new word's score,
  each word left out of training in turn (Turkish pronouns 14.29%, Finnish pronouns
  16.51%).
- **Polish nouns** lost Middle Polish (`pl-mid-decl-noun`) tables, which aren't the modern
  language's (PR #18): on the development tenth −0.06 from the headword and −0.14 with the
  principal parts, the one loss this change was kept with.
- **Below the floor, beyond chance**, the floor learning from the same words as the
  grammar: English verbs (−0.56), Russian proper nouns (−2.83), Russian verbs (−0.91), Sanskrit numerals (−28.79), Polish adjectives (−0.05), German nouns (−1.02), Spanish verbs (−0.30), French verbs (−0.59), Finnish nouns (−0.22), Finnish pronouns (−3.21) and Turkish pronouns (−90.91). It isn't the declension chosen by the ending: in PR #18 a
  classifier reading endings up to eight letters chose no better by a point on German and
  Finnish nouns, English and French verbs or Spanish adjectives, and of 200 wrong German
  plurals, two in three were the plural class chosen wrong (a loanword's `-s`).
- **Russian** has no stress any more (stripped from input, never written).

## The generated code

A language crate has a module per part of speech. The excerpts below are read from the
crates by `cargo xtask readme`, which writes this README. From `russian`:

A paradigm: named after its commonest word, with the gender its words share, the stem
slots it builds through a shared ending table (`stems.rs`), then the cells that differ
from them, `~` for the stem, `derived` for a cell it takes from another. A paradigm
*like* another writes only what differs (`_` for the rest):

```rust
    // Приём: 37,921 words (указ, зонт, кот, зонтик, бортник, бочаг).
    declension!("приём", 37921, "~", [gender: Masculine];
    stem pl: "~- like приём";
    stem gen_sg: "~- like приём";
    nom: "~", _;
    noun_second_case: derived, _;
    ),
    // Полнота: 12,531 words (собака, буява, былица, масленица, царица, вата). Like Приём, except in 2 cells: gen_ pl, noun_second_case loc.
    declension!("полнота", 12531, "~а", [gender: Feminine], like Приём;
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
    // indicative_act_perf_1_sg like modulor: 561 words.
        table: Cow::Borrowed(concat!(
            "~us sum\t~ī sumus\t",
            "~us es\t~ī estis\t",
            "~us est\t~ī sunt\t",
            ...
            "~um esse\t",
```

Where a `derived` cell comes from (`relations.rs`): from `russian`, the accusative is the
genitive or the nominative, by the word's animacy. From `latin`, a relation that changes
the form, with the rules on its ending that choose the change (`0`, `1`, `2`: the changes
in order):

```rust
    relation!(ACC_SG, [GEN_SG, NOM_SG], [], none, false, None),

    relation!(ACT_PERF_2_SG, [ACT_PERF_1_SG], [("~iī", "~īstī"), ("~sum", "~es"), ("~ī", "~istī")], rules {
        "" => [2 4877, 1 563, 0 30],
        "m" => [1 563],
        "um" => [1 563],
        "ī" => [2 4877, 0 30],
        "iī" => [0 30, 2 14],
```

A cell no word (or too few) has is still built: by analogy with a pair of cells that
differ the same way. Few Greek verbs have a first person dual in the dictionary; it is
made from the first person plural as the second person's dual is from its plural:

```rust
    // IND_ACT_1_DU, from IND_ACT_1_PL. Few or no words have it: changed as IND_ACT_2_PL is into IND_ACT_2_DU.
    relation!(IND_ACT_1_DU, [IND_ACT_1_PL], [("~ε", "~ον")], rules {
        "" => [0 1855],
    } listed {}, true, Some((IND_ACT_2_PL, IND_ACT_2_DU))),
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
(`%`; a copy is written `°`, and a copied `φ` is `π`), and where the form's first letters
meet the stem (`‹`) a law lengthens the vowel, by a map; a preverb meets the augment as
the dictionary's compounds show:

```rust
        law1 = ["ἀ" => "ἠ", "α" => "η"];
    ...
        // %ε‹ἀ → ἠ (α → η): 16 words (ἀγω, ἀγᾰπᾰω, ἀκουω, ἀρκεω), 0 forms wrong.
        law1: "^°ἀε‹{x:[ἀ|α]}" => "{x|law1}";
        // copy of φ → π: 33 words (φοβεω, φῐλεω, φενᾱκῐζω, φῠω), 0 forms wrong.
        law2: "°φ" => "π";

    perfect_subj_pass_1: "%ε~γμένος ὦ", _, "%ε~γμένοι ὦμεν";
    perfect_ind_act_1: "%ε~κᾰ", _, _;

        ), // ανα + ἐ → ἀνε
```

The rules choosing a declension, each with its evidence (how many of the dictionary's
words with that ending take each declension), then the words no rule gets right, listed
by the declension they take (sorted into one lookup array when the crate is built):

```rust
        "ба" => [Полнота 239, Какао 10, Люлька 6, Порубка 1] regular,
        "жба" => [Полнота 30] regular,
        "роба" => [Полнота 13] regular,
    ...
    listed {
        Приём => [
            "#абдалхан", "#авиаспорт", "#ага-хан", "#аер", "#азиат", "#айршир", "#алихан", "#аллес",
```

Proper nouns have grammars of their own, as they choose otherwise: from `english`,
a name in `-y` takes `-s` (`Kennedys`), where a noun takes `-ies`:

```rust
        "y" => [Hopewell 3091, Kelly 31, Douglas 1, Miles 1] regular,
        "ay" => [Hopewell 228] regular,

    // Hopewell: 39,422 words (december, april, june, september, october, november).
    declension!("hopewell", 39422, "~";
    proper_noun: "~", "~s";
    ),
```

A part's cells are the language's. From `latin`, the subjunctive has no future:

```rust
/// Tense of a subjunctive: Latin has no future one (A&G §§ 184–196).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
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
    relation!(CONVERB_POS_PLAIN_DOUBLED, [OPTATIVE_POS_PLAIN___3SG], [("~", "~ Ala"), ("~", "~ AldırA"), ("~", "~ AnlAyA …
        "" => [35 1879, 4 186, 0 43, 30 33, 37 17, 33 15, 6 12, 15 12, 39 12, 19 10, 40 7, 23 6, 7 5, 17 4, 29 4, 32 4,  …
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
        "#a" => ['a' 7368, 'ā' 338],
        "#act" => ['ā' 28, 'a' 14],
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

    // 1, `νῠμφη`: 1,907 words. Like 0 (`σημειον`), except in 1 cell.
    pattern!("νῠμφη", 1907, like 0;
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

The crates' source, as they are now, per crate (MB):

| crate | MB |
|---|---:|
| ancient-greek | 3.35 |
| finnish | 2.11 |
| russian | 1.96 |
| latin | 1.50 |
| german | 1.23 |
| polish | 1.19 |
| turkish | 0.89 |
| sanskrit | 0.65 |
| spanish | 0.48 |
| english | 0.46 |
| italian | 0.42 |
| french | 0.26 |

They are 14.51 MB in all (15.97 before PR #21), Turkish verbs 0.55 MB (1.62 before
PR #21, when their vowel law came to read only vowels). Ancient Greek's verbs, 489 cells
each, are most of its size: their paradigms (0.59 MB, 284 conjugations) and
their irregular forms (0.90 MB: 25,869 forms no paradigm and listed stem
give, 27,750 before PR #21). Nothing is computed before the first answer, and the same entries
always give the same crate, byte for byte.

A clean debug build of each crate, one at a time (one machine, `-j 4`; measured on PR
#21's crates; CI's numbers are in its compile-cost job; every build's compiler peaked under
0.92 GB on PR #19's), and one call (release: a word's candidates for one cell, over 30
words and their first 40 cells; `cargo xtask bench <language> --pos <part>`, measured on
the grammars of PR #14), on words the grammar was learned from and on held-out words:

| language | build seconds | `.rlib` MB | one call, nouns | one call, verbs |
|---|---:|---:|---:|---:|
| english | 2 | 52 | 9 / 9 µs | 10 / 10 µs |
| russian | 6 | 86 | 16 / 17 µs | 36 / 38 µs |
| latin | 5 | 94 | 21 / 23 µs | 32 / 40 µs |
| ancient-greek | 4 | 86 | 44 / 81 µs | 52 / 146 µs |
| sanskrit | 2 | 55 | 5 / 7 µs | 19 / 25 µs |
| polish | 4 | 76 | 13 / 25 µs | 13 / 17 µs |
| german | 5 | 69 | 12 / 22 µs | 16 / 24 µs |
| spanish | 2 | 48 | 4 / 4 µs | 9 / 12 µs |
| french | 1 | 53 | 4 / 4 µs | 6 / 6 µs |
| italian | 2 | 55 | 4 / 4 µs | 6 / 11 µs |
| finnish | 5 | 84 | 16 / 16 µs | – |
| turkish | 3 | 56 | 8 / 9 µs | – |

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
  (Finnish nouns: 126,530), its paradigms, relations and laws are learned from the
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

- `crates/flexion`: The runtime of learned inflection grammars: paradigms, the rules that choose them, stress and sound laws, with candidates that explain themselves.
- `crates/flexion-configs`: What flexion-train knows about each language: its features, cells and Wiktionary's tags.
- `crates/flexion-macros`: A generated grammar's listed words, written grouped by what they are and sorted when the crate is built.
- `crates/flexion-train`: Learn a language's inflection grammar from Wiktionary and write it as a Rust crate.
- `crates/sound-law`: A declarative, capture-aware sound-change engine.
- `crates/sound-law-macros`: Sound laws written in sound-law's notation, compiled when the crate that writes them is built.
- `xtask`: Learn, write and check the language crates: cargo xtask <command>.
- `languages/*`: the generated crates.

## Building a language

```sh
cd flexion
cargo xtask extract russian   # read ~/.cache/flexion/dumps/Russian.jsonl
cargo xtask build russian     # learn, measure, and write languages/russian
cargo xtask build all --lanes 2   # every language, two at a time, then this README
cargo xtask readme            # this README, from the saved builds and the crates
```

A build saves what it learned and measured in `target/build-cache/`: built again with only
the printer changed, it reprints from there without learning again.

## License

The code is MIT or Apache-2.0. The generated grammars are derived from Wiktionary and are
licensed under CC BY-SA 4.0 (see each crate's `DATA-LICENSE.md`).
