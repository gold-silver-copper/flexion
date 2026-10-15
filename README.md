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
(letters only). *Against the floor*: the grammar's letters right less the floor's, word
by word on the same forms, with its 95% interval. *On a training word / new root*: top-1
of the held-out words that are a training word with a prefix or a compound's front, and of
the rest. *Any listed form*: top-1 where any form the dictionary lists in the cell counts.
*Homograph ceiling*: the most any grammar could get right (words typed alike get one
answer per cell). *Lemma spelling*: top-1 on the lemma's form where it isn't what was
typed. *Attested*: the shipped grammar on the dictionary's own words. This tenth was looked
at while the trainer was developed, up to PR #15; since PR #16 changes are judged on
another (`cargo xtask try` scores tenth 2, `--final` this one).

| language | part of speech | top-1 from the headword | lemma in full | principal parts given | features given | floor | against the floor | on a training word / new root | any listed form | homograph ceiling | lemma spelling | attested | calibration error |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| english | noun | **97.17%** (97.0–97.4) | 97.17% |  |  | 96.77% | +0.40 (+0.24…+0.56) | 98.68% / 95.73% | 97.97% | 99.88% | 0.00% (1) | 100.00% | 0.003 |
| english | verb | **97.17%** (96.8–97.6) | 97.17% | 99.54% |  | 97.79% | -0.62 (-0.93…-0.33) | 98.92% / 96.23% | 97.61% | 99.86% |  | 100.00% | 0.006 |
| english | adjective | **95.35%** (94.8–95.9) | 95.35% |  |  | 92.53% | +2.82 (+2.39…+3.25) | 99.05% / 93.61% | 96.30% | 99.94% |  | 100.00% | 0.005 |
| english | adverb | **99.01%** (98.4–99.5) | 99.01% |  |  | 98.34% | +0.66 (+0.25…+1.16) | 100.00% / 98.67% | 99.59% | 100.00% |  | 100.00% | 0.005 |
| russian | noun | **94.64%** (94.0–95.2) | 94.64% | 95.81% | 97.30% | 94.24% | +0.41 (-0.06…+0.91) | 97.06% / 93.82% | 94.99% | 99.82% |  | 100.00% | 0.007 |
| russian | adjective | **99.75%** (99.6–99.9) | 99.75% |  |  | 99.77% | -0.02 (-0.14…+0.08) | 99.71% / 99.77% | 99.75% | 99.94% |  | 100.00% | 0.003 |
| russian | verb | **95.76%** (95.0–96.4) | 95.76% | 98.88% | 95.49% | 95.92% | -0.16 (-0.70…+0.39) | 95.84% / 95.72% | 95.89% | 99.49% |  | 100.00% | 0.006 |
| latin | noun | **68.69%** (66.4–70.9) | 92.93% | 98.69% | 69.57% | 58.82% | +9.87 (+7.17…+12.54) | 75.91% / 66.47% | 68.74% | 99.46% | 68.09% (984) | 100.00% | 0.019 |
| latin | adjective | **84.05%** (82.8–85.3) | 99.23% | 99.47% |  | 71.72% | +12.32 (+10.56…+14.12) | 87.26% / 82.03% | 84.14% | 99.60% | 82.98% (2151) | 99.91% | 0.023 |
| latin | verb | **77.28%** (74.2–80.5) | 85.66% | 98.55% |  | 65.32% | +11.96 (+7.95…+16.09) | 86.26% / 73.46% | 77.42% | 98.45% | 88.85% (592) | 100.00% | 0.044 |
| ancient-greek | noun | **71.16%** (68.2–74.1) | 88.25% | 95.20% | 72.06% | 51.16% | +23.86 (+19.97…+27.52) | 77.49% / 69.68% | 71.18% | 99.55% | 75.47% (587) | 99.73% | 0.055 |
| ancient-greek | adjective | **77.17%** (73.7–80.4) | 90.10% | 97.49% |  | 52.29% | +25.98 (+21.47…+30.48) | 80.30% / 76.57% | 77.37% | 99.17% | 82.84% (303) | 99.93% | 0.044 |
| ancient-greek | verb | **65.28%** (60.5–70.1) | 76.10% | 80.89% |  | 46.03% | +20.69 (+14.70…+26.51) | 73.19% / 60.67% | 65.52% | 99.90% | 77.50% (160) | 100.00% | 0.071 |
| sanskrit | noun | **92.10%** (90.8–93.4) | 92.10% |  | 97.30% | 91.70% | +0.40 (-0.73…+1.48) | 94.30% / 92.03% | 92.65% | 97.83% |  | 100.00% | 0.015 |
| sanskrit | adjective | **91.91%** (89.6–94.0) | 91.91% |  |  | 89.95% | +1.96 (+0.32…+3.76) | 83.33% / 92.30% | 91.94% | 100.00% |  | 100.00% | 0.020 |
| sanskrit | verb | **89.97%** (85.4–94.1) | 89.97% | 90.05% |  | 89.93% | +0.04 (-2.21…+2.14) | 95.65% / 89.43% | 90.48% | 100.00% | 100.00% (2) | 100.00% | 0.060 |
| polish | noun | **93.37%** (92.9–93.9) | 93.37% | 95.71% | 96.17% | 93.23% | +0.13 (-0.24…+0.47) | 95.06% / 93.04% | 93.65% | 99.23% | 0.00% (5) | 100.00% | 0.004 |
| polish | adjective | **99.92%** (99.9–99.9) | 99.92% |  |  | 99.99% | -0.07 (-0.09…-0.04) | 99.94% / 99.91% | 99.92% | 100.00% |  | 100.00% | 0.001 |
| polish | verb | **96.62%** (95.8–97.4) | 96.62% | 97.88% | 96.67% | 96.78% | -0.16 (-0.73…+0.45) | 95.22% / 96.93% | 96.90% | 99.77% | 0.00% (7) | 100.00% | 0.012 |
| german | noun | **93.63%** (93.2–94.1) | 93.63% | 99.79% | 94.05% | 93.88% | -0.25 (-0.58…+0.08) | 96.70% / 89.92% | 94.43% | 99.69% |  | 100.00% | 0.007 |
| german | adjective | **98.76%** (98.2–99.3) | 98.76% | 99.86% |  | 99.00% | -0.24 (-0.74…+0.25) | 98.59% / 98.83% | 99.03% | 100.00% | 0.00% (28) | 100.00% | 0.003 |
| german | verb | **95.07%** (94.1–96.0) | 95.07% | 95.78% | 95.18% | 73.03% | +22.04 (+20.29…+23.86) | 95.91% / 94.14% | 95.17% | 99.26% | 0.00% (1) | 100.00% | 0.013 |
| spanish | noun | **98.62%** (98.3–98.9) | 98.62% |  | 98.62% | 98.62% | +0.00 (-0.20…+0.20) | 99.85% / 98.05% | 98.83% | 99.98% |  | 100.00% | 0.003 |
| spanish | adjective | **98.21%** (97.8–98.7) | 98.21% | 99.67% |  | 98.57% | -0.36 (-0.68…-0.04) | 99.68% / 97.70% | 98.32% | 100.00% |  | 100.00% | 0.005 |
| spanish | verb | **98.96%** (98.6–99.3) | 98.96% | 99.72% |  | 99.19% | -0.23 (-0.51…+0.01) | 99.28% / 98.75% | 98.98% | 99.98% | 0.00% (2) | 100.00% | 0.004 |
| french | noun | **97.57%** (97.1–98.0) | 97.57% |  | 97.57% | 97.01% | +0.56 (+0.21…+0.88) | 96.88% / 97.82% | 97.96% | 99.88% |  | 100.00% | 0.004 |
| french | adjective | **97.50%** (96.9–98.1) | 97.50% | 99.95% |  | 97.72% | -0.22 (-0.65…+0.24) | 98.85% / 97.12% | 97.57% | 100.00% |  | 100.00% | 0.009 |
| french | verb | **98.70%** (98.1–99.3) | 98.70% | 99.48% |  | 99.28% | -0.59 (-1.06…-0.22) | 99.63% / 98.40% | 98.84% | 100.00% |  | 100.00% | 0.004 |
| french | participle | **99.84%** (99.5–100.0) | 99.84% |  |  | 99.52% | +0.32 (-0.32…+0.96) | 100.00% / 99.80% | 99.84% | 100.00% |  | 100.00% | 0.005 |
| italian | noun | **95.82%** (95.3–96.3) | 95.82% |  | 96.39% | 95.09% | +0.74 (+0.31…+1.16) | 95.91% / 95.76% | 95.96% | 99.77% |  | 100.00% | 0.003 |
| italian | adjective | **98.46%** (98.1–98.8) | 98.46% | 99.78% |  | 98.46% | +0.00 (-0.29…+0.30) | 99.36% / 97.96% | 98.49% | 100.00% |  | 100.00% | 0.010 |
| italian | verb | **99.05%** (98.7–99.4) | 99.05% | 99.56% | 99.05% | 99.22% | -0.17 (-0.48…+0.13) | 98.93% / 99.12% | 99.12% | 99.96% |  | 100.00% | 0.005 |
| italian | participle | **100.00%** (100.0–100.0) | 100.00% |  |  | 100.00% | +0.00 (+0.00…+0.00) | 100.00% / 100.00% | 100.00% | 100.00% |  | 100.00% | 0.000 |
| finnish | noun | **97.89%** (97.7–98.1) | 97.89% | 99.22% |  | 98.29% | -0.40 (-0.60…-0.21) | 98.73% / 96.22% | 98.05% | 99.98% |  | 100.00% | 0.009 |
| finnish | adjective | **97.45%** (96.9–98.0) | 97.45% |  |  | 97.41% | +0.04 (-0.62…+0.72) | 99.15% / 96.40% | 97.52% | 100.00% |  | 100.00% | 0.010 |
| turkish | noun | **96.51%** (95.5–97.4) | 96.51% | 99.52% |  | 93.27% | +3.23 (+2.30…+4.24) | 97.63% / 96.39% | 96.55% | 99.88% |  | 100.00% | 0.012 |

Where the grammar is weak or below the floor, and why:

- **Latin, from the headword.** A headword doesn't write vowel length (`fox` for `fōx`);
  the grammar restores it by rules on the letters around each vowel (`-ātiō`, `dē-`, a
  diphthong), learned from every part of speech's words and reading which part a word is
  (`quīngēnī`, a numeral; `indigenus`, an adjective), and reads the three likeliest
  spellings. On the development tenth that gave nouns +3.8 points, adjectives +2.1, verbs
  +8.6. But a stem's own lengths are lexical: `iūdex`, `bōlus` and `grūs` come back as
  `iudex`, `bolus`, `grus`. Typed in full the same grammars reach 92.9% (nouns), 99.2%
  (adjectives), 85.7% (verbs), and with the principal parts 98.5–99.5%.
- **Ancient Greek.** A circumflex on a diphthong's second letter (`ἀριθμοῦ`) was read as
  vowel length, which split declensions in two; read as the diphthong's, nouns went from
  133 to 87 declensions and +3.8 points, adjectives +3.1 (development tenth). The augment
  (a law where the form's first letters meet the stem: `ἐ‹ἀ → ἠ`), the perfect's
  reduplication (`%ε~κα`) and the augment behind a preverb (`ἀπο + ἐ → ἀπε`) are laws.
  What's left in the verbs is mostly stems: a verb's aorist or perfect stem (`ἀγγέλλω` →
  `ἠγγέλθην`) follows from no ending, so a new verb's is a guess; known verbs list it with
  the ending table it takes, and a principal part given fixes its stem (80.9%).
- **Sanskrit.** The `n` → `ṇ` rule is a law (`तर्काणाम्`), and a noun's gender, given,
  takes it from 92.1% to 97.3%. The verbs are small (a tenth is ~300 words).
- **Below the floor, beyond chance**: English verbs (−0.62), French verbs (−0.59),
  Finnish nouns (−0.40), Spanish adjectives (−0.36), Polish adjectives (−0.07); not looked
  into yet. Italian verbs were 1.6 points below: the polite imperative (`parli`) was split
  by the verb's auxiliary between two cells that are the same form, and the auxiliary,
  never decided from the headword, left it empty. The split is gone (99.05%, the floor's
  99.22% within chance). German nouns (−0.25, within chance) lose the plural cells: of 200
  wrong plurals, two in three are the plural class chosen wrong (a loanword's `-s`,
  `-ismus` → `-ismen`), one in fourteen umlaut.
- **Russian** has no stress any more (stripped from input, never written).

## The generated code

A language crate has a module per part of speech. Excerpts from `russian`:

A paradigm: a table of patterns, `~` for the stem, named after its commonest word, with
the gender its words share and the cells it takes from others marked `derived`. A
paradigm *like* another writes only the cells that differ (`_` for the rest):

```rust
    // Приём: 37,696 words (указ, зонт, кот, зонтик, бортник, бочаг).
    declension!("приём", 37696, "~", [gender: Masculine];
        nominative: "~", "~ы";
        genitive: "~а", "~ов";
        dative: "~у", "~ам";
        accusative: derived, derived;
        instrumental: "~ом", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~у", "~у";
    ),

    // Химия: 6,582 words (анархия, лесохимия, библия, биология, академия, армия). Like Склонность, except in 5 cells: nominative sg, genitive pl, accusative sg, instrumental sg, noun_second_case loc.
    declension!("химия", 6582, "~я", [gender: Feminine, animacy: Inanimate], like Склонность;
        nominative: "~я", _;
        genitive: _, "~й";
        accusative: "~ю", _;
        instrumental: "~ей|~ею", _;
        ...
```

Where a `derived` cell comes from (`relations.rs`): the accusative is the genitive or the
nominative, by the word's animacy. From `latin`, a relation that changes the form, with
the rules on its ending that choose the change (`0`, `1`: the changes in order):

```rust
    relation!(ACC_SG, [GEN_SG, NOM_SG], [], none, false, None),

    // IND_ACT_PERF_2_SG, from IND_ACT_PERF_1_SG.
    relation!(IND_ACT_PERF_2_SG, [IND_ACT_PERF_1_SG], [("~iī", "~īstī"), ("~ī", "~istī")], rules {
        "" => [1 4880, 0 30],
        "iī" => [0 30, 1 14],
        "vī" => [1 2809],
```

A cell no word (or too few) has is still built: by analogy with a pair of cells that
differ the same way. Greek has no future subjunctive; it is made from the future
optative as the perfect subjunctive is from the perfect optative (`λύσοιμι` → `λύσω`):

```rust
    // FUTURE_SUBJ_ACT_1_SG, from FUTURE_OPT_ACT_1_SG. Few or no words have it: changed as PERFECT_OPT_ACT_1_SG is into PERFECT_SUBJ_ACT_1_SG.
    relation!(FUTURE_SUBJ_ACT_1_SG, [FUTURE_OPT_ACT_1_SG], [("~αιην", "~ω"), ("~οιμῐ", "~ω")], rules {
```

The start of a word too. From `ancient-greek`: the perfect copies the stem's first letter
(`%`), the augment is a law where the form's first letters meet the stem (`‹`), and a
preverb meets the augment as the dictionary's compounds show:

```rust
    perfect_ind_act_1: "%ε~ηκᾰ", _, "%ε~ηκᾰμεν";

    // ἐ‹ἀ → ἠ: 92 words (ἀγω, ἀκουω, ἀρχω, ἀγαλλιαω), 0 forms wrong.

        ), // ανα + ἐ → ἀνε
```

Sound laws (`laws.rs`), each with the words whose forms it gives and the forms it gets
wrong (those are listed); from `russian` and `sanskrit`:

```rust
    // к+ы → к+и: 9,368 words (собака, зонтик, бортник, бредник), 5 forms wrong.
    // न → ण after र, ष, ृ unless च, ज, ट, ठ, ड, ढ, ण, त, थ, द, ध, न, ल, श, स comes between, not before ्: 1,182 words (वृक्ष, समुद्र, तरंग, तीर), 1 forms wrong.
```

The rules choosing a declension, each with its evidence (how many of the dictionary's
words with that ending take each declension):

```rust
        "ба" => [Полнота 241, Какао 10, Люлька 4, Порубка 1],
        "ьба" => [Полнота 32, Люлька 4, Порубка 1],
        "дьба" => [Полнота 6, Люлька 4],
```

From `latin`, how a vowel typed without its macron is written, by the letter before it
and the letters after it (`#` is the word's start or end):

```rust
        "#a" => ['a' 1684, 'ā' 97],
        "#act" => ['ā' 10, 'a' 3],
```

From `ancient-greek`, the law of limitation's facts, and an accent pattern (where each
cell's accent goes before the law moves it):

```rust
    prosody: Some(flexion::Prosody {
        diphthongs: &["αι", "ει", "οι", "υι", "αυ", "ευ", "ηυ", "ου", "ωυ"],
        long: "ηωἠἡὠὡᾀᾁᾐᾑᾠᾡᾱᾳῃῑῡῳ",
        short_finals: &["αι", "οι"],
        ...
    }),

    pattern!("νῠμφη", 1810;
        nominative: Stem, Stem, Stem;
        genitive: Stem, Stem, Ending;
        ...
```

And what no rule gives, as the dictionary writes it (a table: word, cell, form). A word
lists its principal parts; Latin `adiuvō` lists its perfect and its participle, and its
other perfect forms (`adiūvistī`, `adiūverant`) are told from them:

```text
adiuvo	18	adiūvī
adiuvo	175	adiūtus
```

## What it costs

A clean debug build of each crate, one at a time (one machine; CI's numbers are in its
compile-cost job), and one call (release: a word's candidates for one cell, over 30 words
and their first 40 cells; `cargo xtask bench <language> --pos <part>`, last measured on
the grammars of PR #14), on words the grammar was learned from and on held-out words:

| crate | source MB | build seconds | peak MB | `.rlib` MB | one call, nouns | one call, verbs |
|---|---:|---:|---:|---:|---:|---:|
| english | 0.9 | 3 | 566 | 46 | 9 / 9 µs | 10 / 10 µs |
| russian | 3.0 | 7 | 922 | 75 | 16 / 17 µs | 36 / 38 µs |
| latin | 1.9 | 6 | 816 | 70 | 21 / 23 µs | 32 / 40 µs |
| ancient-greek | 4.2 | 4 | 818 | 77 | 44 / 81 µs | 52 / 146 µs |
| sanskrit | 0.9 | 2 | 485 | 41 | 5 / 7 µs | 19 / 25 µs |
| polish | 1.7 | 4 | 755 | 61 | 13 / 25 µs | 13 / 17 µs |
| german | 2.5 | 9 | 1198 | 62 | 12 / 22 µs | 16 / 24 µs |
| spanish | 1.7 | 6 | 1135 | 47 | 4 / 4 µs | 9 / 12 µs |
| french | 0.4 | 1 | 464 | 45 | 4 / 4 µs | 6 / 6 µs |
| italian | 0.9 | 3 | 563 | 50 | 4 / 4 µs | 6 / 11 µs |
| finnish | 1.6 | 4 | 582 | 34 | 16 / 16 µs | adjectives 10 / 11 µs |
| turkish | 0.3 | 1 | 334 | 22 | 8 / 9 µs | adjectives 5 / 5 µs |

Ancient Greek's verbs, 624 cells each, are most of its size: their paradigms (0.9 MB,
287 conjugations) and their irregular forms (1.0 MB: 1,860 listed stems with the ending
table each builds on, and 33,311 forms no stem and table give). Nothing is computed
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
  words. Laws run in [`sound-law`](crates/sound-law).
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
  dictionary's words that the rules don't spell are listed, and the answer's probability
  says how much of the spelling was a guess. A circumflex is vowel length on a single
  vowel (`σῦς`), not on a diphthong's second letter (`ἀριθμοῦ`).
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
