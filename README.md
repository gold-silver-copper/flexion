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
was not learned from, asked for every form the dictionary lists. *Top-1 from the
headword*: the word typed as the dictionary's headword (Latin `rosa`, Greek `λύω`, no
vowel length), with a 95% margin. *Lemma in full*: typed with every mark the tables
write. *Principal parts given*: the dictionary's principal parts given too. *Features
given*: the word's gender (Russian: and animacy; verbs: aspect) given, as `.gender(…)`
gives it. *Seen* and *unseen cells*: the cells some training word of the word's paradigm
has, and the others (told from other cells), with their number of forms. *Floor*: the
exact non-neural baseline of the CoNLL–SIGMORPHON 2017 shared task. *Baseline*: the
version of these crates before PR #10 (a model, not a grammar), from the bare lemma.
*Attested*: the shipped grammar on the dictionary's own words.

| language | part of speech | top-1 from the headword | lemma in full | principal parts given | features given | seen cells | unseen cells | floor | baseline | attested | calibration error |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| english | noun | **98.58%** ± 0.2 | 98.58% | 100.00% |  | 98.58% (47466) |  | 98.39% | 97.14% | 100.00% | 0.001 |
| english | verb | **97.80%** ± 0.4 | 97.80% | 99.71% |  | 97.80% (20745) |  | 98.24% | 97.91% | 100.00% | 0.005 |
| english | adjective | **96.91%** ± 0.4 | 96.91% | 100.00% |  | 96.91% (18715) |  | 95.02% | 96.69% | 100.00% | 0.004 |
| english | adverb | **99.34%** ± 0.5 | 99.34% | 100.00% |  | 99.34% (3621) |  | 98.90% |  | 100.00% | 0.002 |
| russian | noun | **95.10%** ± 0.8 | 95.10% | 96.20% | 97.53% | 95.10% (30193) | 100.00% (8) | 94.72% | 84.48% | 100.00% | 0.007 |
| russian | adjective | **99.75%** ± 0.2 | 99.75% |  |  | 99.76% (171213) | 0.00% (12) | 99.78% | 96.60% | 100.00% | 0.003 |
| russian | verb | **96.07%** ± 1.1 | 96.07% | 99.00% | 95.80% | 96.09% (21829) | 79.31% (29) | 96.17% | 92.37% | 100.00% | 0.005 |
| latin | noun | **66.38%** ± 2.3 | 93.38% | 98.74% | 67.21% | 66.40% (18401) | 0.00% (6) | 59.45% | 68.91% | 100.00% | 0.032 |
| latin | adjective | **82.53%** ± 1.4 | 99.28% | 99.52% |  | 82.53% (102281) | 0.00% (1) | 71.85% | 87.22% | 100.00% | 0.025 |
| latin | verb | **70.47%** ± 3.6 | 85.25% | 98.56% |  | 70.44% (60850) | 87.60% (121) | 65.34% | 75.18% | 100.00% | 0.034 |
| ancient-greek | noun | **67.48%** ± 3.2 | 85.52% | 93.95% | 68.19% | 67.48% (11700) |  | 49.22% | 57.82% | 99.92% | 0.061 |
| ancient-greek | adjective | **72.07%** ± 4.0 | 85.73% | 94.63% |  | 72.09% (22426) | 40.00% (10) | 50.81% | 63.66% | 99.94% | 0.059 |
| ancient-greek | verb | **66.66%** ± 5.8 | 75.77% | 80.86% |  | 67.02% (63800) | 10.40% (404) | 46.07% | 54.26% | 100.00% | 0.081 |
| sanskrit | noun | **92.10%** ± 2.2 | 92.10% |  | 97.30% | 92.10% (13801) |  | 91.70% | 93.55% | 100.00% | 0.017 |
| sanskrit | adjective | **92.00%** ± 3.9 | 92.00% |  |  | 92.06% (13447) | 11.11% (9) | 89.95% | 91.21% | 100.00% | 0.022 |
| sanskrit | verb | **90.05%** ± 6.3 | 90.05% | 90.77% |  | 90.45% (6953) | 80.28% (284) | 90.01% | 90.13% | 100.00% | 0.054 |
| polish | noun | **93.91%** ± 0.7 | 93.91% | 96.09% | 96.49% | 93.92% (48466) | 62.50% (16) | 93.78% |  | 100.00% | 0.004 |
| polish | adjective | **99.95%** ± 0.1 | 99.95% |  |  | 99.95% (83790) |  | 99.99% |  | 100.00% | 0.001 |
| polish | verb | **96.66%** ± 1.0 | 96.66% | 97.99% | 96.82% | 96.67% (55069) | 93.75% (128) | 96.84% |  | 100.00% | 0.010 |
| german | noun | **94.22%** ± 0.6 | 94.22% | 98.62% | 94.59% | 94.22% (46353) |  | 94.71% |  | 100.00% | 0.005 |
| german | adjective | **98.78%** ± 0.6 | 98.78% | 99.82% |  | 98.78% (126633) |  | 99.01% |  | 100.00% | 0.004 |
| german | verb | **95.20%** ± 1.3 | 95.20% | 95.88% | 95.22% | 95.19% (51549) | 100.00% (50) | 73.53% |  | 100.00% | 0.013 |
| spanish | noun | **99.31%** ± 0.2 | 99.31% |  | 99.31% | 99.31% (12164) |  | 99.31% |  | 100.00% | 0.002 |
| spanish | adjective | **98.80%** ± 0.4 | 98.80% | 99.79% |  | 98.80% (9619) |  | 98.93% |  | 100.00% | 0.003 |
| spanish | verb | **98.90%** ± 0.7 | 98.90% | 99.71% |  | 98.90% (67865) |  | 99.20% |  | 100.00% | 0.004 |
| french | noun | **98.78%** ± 0.3 | 98.78% |  | 98.78% | 98.78% (8636) |  | 98.51% |  | 100.00% | 0.003 |
| french | adjective | **98.11%** ± 0.6 | 98.11% | 99.97% |  | 98.14% (7354) | 0.00% (2) | 98.29% |  | 100.00% | 0.006 |
| french | verb | **98.78%** ± 0.8 | 98.78% | 99.36% |  | 98.78% (35550) |  | 99.30% |  | 100.00% | 0.007 |
| french | participle | **99.88%** ± 0.3 | 99.88% |  |  | 99.88% (2508) |  | 99.64% |  | 100.00% | 0.004 |
| italian | noun | **97.91%** ± 0.4 | 97.91% |  | 98.19% | 97.91% (12784) |  | 97.54% |  | 100.00% | 0.003 |
| italian | adjective | **98.84%** ± 0.4 | 98.84% | 99.87% |  | 98.84% (10530) | 100.00% (5) | 98.84% |  | 100.00% | 0.007 |
| italian | verb | **97.42%** ± 0.9 | 97.42% | 98.13% | 99.05% | 97.42% (71585) | 100.00% (1) | 99.24% |  | 100.00% | 0.021 |
| italian | participle | **100.00%** ± 0.0 | 100.00% |  |  | 100.00% (3284) |  | 100.00% |  | 100.00% | 0.000 |
| finnish | noun | **92.49%** ± 0.5 | 92.49% |  |  | 92.51% (1912243) | 8.49% (365) | 98.30% |  | 93.26% | 0.011 |
| finnish | adjective | **97.50%** ± 0.7 | 97.50% |  |  | 97.50% (46113) | 33.33% (3) | 97.51% |  | 100.00% | 0.008 |
| turkish | noun | **96.53%** ± 1.1 | 96.53% |  |  | 96.53% (72174) |  | 93.37% |  | 100.00% | 0.012 |

Where the grammar is below the baseline or the floor, and why:

- **Latin, from the headword.** A headword doesn't write vowel length (`fox` for `fōx`);
  the grammar restores it by rules on the letters around each vowel (`-ātiō`, `dē-`, a
  diphthong), and offers the second likeliest spelling too (top-3: nouns 86.5%, verbs
  91.6%), but a stem's own lengths are lexical: `iūdex`, `bōlus` and `grūs` come back as
  `iudex`, `bolus`, `grus`. Typed in full the same grammars reach 93.4% (nouns), 99.3%
  (adjectives), 85.3% (verbs), and with the principal parts 98.6–99.5%. Over all ten tenths
  the verbs scored 71.8% ± 0.9 from the headword and 97.8% ± 0.3 with principal parts
  (PR #12's grammar; one tenth now: 70.5% and 98.6%).
- **Ancient Greek verbs.** The augment (a law where the form's first letters meet the
  stem: `ἐ‹ἀ → ἠ`), the perfect's reduplication (a copied letter, `%ε~κα`) and the augment
  behind a preverb (`ἀπο + ἐ → ἀπε`) are learned now, and the accent can sit on a fixed
  syllable from the word's end (`λυθῇς`, `λελυμένος`). What's left is mostly stems: a verb's
  aorist or perfect stem (`ἀγγέλλω` → `ἠγγέλθην`) follows from no ending, so a new verb's
  is a guess; known verbs list it with the ending table it takes, and a principal part
  given fixes its stem (80.9% with them, 74.4% before stems were learned).
- **Sanskrit.** The `n` → `ṇ` rule is a law (`तर्काणाम्`), and a noun's gender, given,
  takes it from 92.1% to 97.3%. The verbs are small (a tenth is ~300 words): over all ten
  tenths they score 87.8% ± 1.6, the tenths ranging from 84.5% to 91.3%. Few full perfect
  tables leave the reduplicated perfect unlearned.
- **Finnish nouns** are learned from the 6,000 commonest (Durrett & DeNero found that
  enough), but the dictionary has ~116,000: the floor, learned from all of them, knows
  the last part of far more compounds (98.3% against 92.5%), and the dictionary's own
  words beyond the 6,000 aren't listed, so 6.7% of their forms come back wrong.
- **Italian verbs** (97.4% against the floor's 99.2%), and by less than a point Spanish
  and French verbs, German nouns and Polish verbs, are below the floor: not looked into
  yet.
- **Russian** has no stress any more (stripped from input, never written), so its
  numbers aren't comparable with the baseline's, which placed stress.

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
and their first 40 cells; `cargo xtask bench <language> --pos <part>`), on words the
grammar was learned from and on held-out words:

| crate | source MB | build seconds | peak MB | `.rlib` MB | one call, nouns | one call, verbs |
|---|---:|---:|---:|---:|---:|---:|
| english | 0.9 | 3 | 548 | 42 | 9 / 9 µs | 10 / 10 µs |
| russian | 3.0 | 6 | 912 | 69 | 16 / 17 µs | 36 / 38 µs |
| latin | 2.0 | 5 | 785 | 61 | 21 / 23 µs | 32 / 40 µs |
| ancient-greek | 4.7 | 6 | 855 | 73 | 44 / 81 µs | 52 / 146 µs |
| sanskrit | 0.9 | 2 | 441 | 38 | 5 / 7 µs | 19 / 25 µs |
| polish | 1.7 | 4 | 693 | 55 | 13 / 25 µs | 13 / 17 µs |
| german | 2.7 | 11 | 1024 | 64 | 12 / 22 µs | 16 / 24 µs |
| spanish | 1.6 | 6 | 794 | 43 | 4 / 4 µs | 9 / 12 µs |
| french | 0.4 | 2 | 339 | 40 | 4 / 4 µs | 6 / 6 µs |
| italian | 1.4 | 3 | 506 | 49 | 4 / 4 µs | 6 / 11 µs |
| finnish | 1.1 | 2 | 484 | 26 | 16 / 16 µs | adjectives 10 / 11 µs |
| turkish | 0.3 | 1 | 226 | 20 | 8 / 9 µs | adjectives 5 / 5 µs |

Ancient Greek's verbs, 624 cells each, are most of its size: their paradigms (1.2 MB,
292 conjugations) and their irregular forms (1.0 MB: 1,827 listed stems with the ending
table each builds on, and 32,105 forms no stem and table give). Nothing is computed
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
  ACT_AS_PASS`). A paradigm of more than one word is kept where it costs fewer bits than
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
  print with class letters (`~lAr`). Contexts grow one letter at a time while a law
  breaks too much, and every law is checked on held-out words. Laws run in
  [`sound-law`](crates/sound-law).
- **Relations.** Each cell's form is told from another cell's (the one that predicts it
  best: a tree on the lemma's cell), its end changed by rules read off that form's ending
  (Latin `amāvī` → `amāvistī`). Where a feature of the word decides which cell (Russian:
  the accusative is the genitive of animate nouns, the nominative of inanimate ones), the
  relation is by that feature, and the feature is a choice of its own. A paradigm takes
  the cells none of its words has from its relations (`derived`), and a word whose forms
  are listed lists only its principal parts: its other forms are told from them, where
  that does as well as its paradigm. A cell too few words have is told by analogy: from
  the cell that differs from it as another pair of cells does, with that pair's change.
  Every word gets a form in every cell, attested or not.
- **Choices.** Which paradigm a word takes is a list of rules on its ending: the longest
  rule the word has decides, with how many of the dictionary's words with that ending
  take each paradigm. A rule is kept only when it describes its words in fewer bits
  than the broader rule does, by more than the rule costs (minimum description length). Words no rule gets
  right are listed. A rule's status follows Yang's Tolerance Principle, judged as a learner
  meets it: on the 250, 500, 1,000 and 2,000 commonest words (in the most dictionaries,
  with the most senses) that have its ending; a rule with few enough exceptions there is
  productive (`regular`, printed on the rule); otherwise speakers vary (`unsettled`).
- **Compounds and prefixes.** A word written in parts inflects its last part or each part
  (Russian `диван-кровать`). German verbs inflect behind their prefixes: separable ones
  go where the form puts them (`fing an`), inseparable ones stay in front and take off
  the participle's `ge-` (`verhindert`). The prefixes, and what the participle drops, are
  learned.
- **Accent.** Where a language writes it (Ancient Greek), each cell has a rule placing the
  accent (where the lemma has it, on the ending, as far back as allowed, or on the word's
  last or next-to-last syllable) and the law
  of limitation moves it and picks acute or circumflex by the syllables' length. Which
  endings take the circumflex, and where a final `-οι` counts long (the optative), are
  learned.
- **Spelling.** Words are read in Unicode's composed form (NFC), so a letter typed with
  its marks in any order is the same letter. A word is found however it is typed: without vowel length (`rosa`,
  `λυω`), without `ё`. Rules on the letters around each vowel restore the marks (`-ātiō`,
  `dē-`), the dictionary's words that they don't spell are listed, and the answer's
  probability says how much of the spelling was a guess.
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
