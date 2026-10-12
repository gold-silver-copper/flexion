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

Five languages are built from the same trainer, with nothing language-specific in the
trainer or the runtime: [English](languages/english), [Russian](languages/russian),
[Latin](languages/latin), [Ancient Greek](languages/ancient-greek) and
[Sanskrit](languages/sanskrit). Configs for Polish, German, Spanish, French and Italian
are kept in `crates/flexion-configs`; `cargo xtask build german` builds one.

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
| english | noun | **98.60%** ± 0.1 | 98.60% | 100.00% |  | 98.60% (47466) |  | 98.39% | 97.14% | 100.00% | 0.001 |
| english | verb | **97.87%** ± 0.4 | 97.87% | 99.73% |  | 97.87% (20745) |  | 98.24% | 97.91% | 100.00% | 0.006 |
| english | adjective | **96.53%** ± 0.5 | 96.53% | 100.00% |  | 96.53% (18715) |  | 95.02% | 96.69% | 100.00% | 0.050 |
| english | adverb | **99.17%** ± 0.5 | 99.17% | 100.00% |  | 99.17% (3621) |  | 98.90% |  | 100.00% | 0.070 |
| russian | noun | **95.03%** ± 0.8 | 95.03% | 96.16% | 97.57% | 95.03% (30193) | 87.50% (8) | 94.72% | 84.48% | 99.96% | 0.004 |
| russian | adjective | **99.76%** ± 0.2 | 99.76% |  |  | 99.76% (171213) | 0.00% (12) | 99.78% | 96.60% | 100.00% | 0.002 |
| russian | verb | **96.04%** ± 1.1 | 96.04% | 98.93% | 95.74% | 96.06% (21827) | 83.87% (31) | 96.17% | 92.37% | 100.00% | 0.009 |
| latin | noun | **65.29%** ± 2.4 | 93.67% | 98.20% | 66.10% | 65.31% (18401) | 0.00% (6) | 59.70% | 68.91% | 100.00% | 0.048 |
| latin | adjective | **81.70%** ± 1.4 | 99.24% | 99.47% |  | 81.71% (102281) | 0.00% (1) | 71.95% | 87.22% | 100.00% | 0.052 |
| latin | verb | **70.53%** ± 3.6 | 85.26% | 95.78% |  | 70.50% (60859) | 85.71% (112) | 65.34% | 75.18% | 100.00% | 0.031 |
| ancient-greek | noun | **61.94%** ± 3.3 | 78.80% | 88.96% | 62.59% | 61.89% (11658) | 100.00% (15) | 45.17% | 57.82% | 100.00% | 0.120 |
| ancient-greek | adjective | **59.07%** ± 4.4 | 72.08% | 87.48% |  | 59.07% (22472) | 50.00% (12) | 39.77% | 63.66% | 100.00% | 0.100 |
| ancient-greek | verb | **46.06%** ± 6.0 | 53.64% | 52.15% |  | 46.28% (64145) | 3.53% (340) | 42.64% | 54.26% | 100.00% | 0.065 |
| sanskrit | noun | **91.90%** ± 2.2 | 91.90% |  | 97.16% | 91.90% (13801) |  | 91.70% | 93.55% | 100.00% | 0.013 |
| sanskrit | adjective | **91.64%** ± 3.9 | 91.64% |  |  | 91.69% (13449) | 0.00% (7) | 89.95% | 91.21% | 100.00% | 0.030 |
| sanskrit | verb | **87.74%** ± 6.9 | 87.74% | 85.44% |  | 88.01% (7173) | 57.81% (64) | 90.01% | 90.13% | 100.00% | 0.085 |

Where the grammar is below the baseline or the floor, and why:

- **Latin and Ancient Greek, from the headword.** A headword doesn't write vowel length
  (`fox` for `fōx`, `λυω` for `λῡω`); the grammar restores it by rules on the letters
  around each vowel (`-ātiō`, `dē-`, a diphthong), but a stem's own lengths are lexical:
  `iūdex`, `bōlus` and `grūs` come back as `iudex`, `bolus`, `grus`. Typed in full the
  same grammars reach 93.7% (Latin nouns), 99.2% (adjectives), 85.3% (verbs), and with
  the principal parts 96–99%. Latin adjectives also include participles, which the
  previous version spelled from their verbs' tables.
- **Ancient Greek verbs.** The augment goes after a preverb (`περιέφερον`, `ἐπετίθην`)
  and the perfect reduplicates (`βέβλεφα`); neither is learned yet, so such forms are
  listed for known verbs (as principal parts where their system follows from them) and
  missed for new ones.
- **Sanskrit.** The `n` → `ṇ` rule is a law now (`तर्काणाम्`), and a noun's gender, given,
  takes it from 91.9% to 97.2%; the verbs are within the margin of the baseline and the
  floor.
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
    // ACC_SG, from GEN_SG or NOM_SG (by animacy, in order).
    Relation { cell: ACC_SG, from: Cow::Borrowed(&[GEN_SG, NOM_SG]), changes: flexion::changes![], change: None, .. },

    // IND_ACT_PERF_2_SG, from IND_ACT_PERF_1_SG.
    Relation {
        cell: IND_ACT_PERF_2_SG,
        from: Cow::Borrowed(&[IND_ACT_PERF_1_SG]),
        changes: flexion::changes![("iī", "īstī"), ("ī", "istī")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 4880, 0 31],
            "iī" => [0 31, 1 14],
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
compile-cost job), and one call (release, a word's form):

| crate | source MB | build seconds | peak MB | `.rlib` MB | one call |
|---|---:|---:|---:|---:|---:|
| english | 2.8 | 8 | 926 | 57 | 9 µs |
| russian | 3.1 | 6 | 861 | 61 | 17 µs |
| latin | 2.6 | 4 | 694 | 50 | 9 µs |
| ancient-greek | 11.9 | 8 | 1,119 | 82 | 28 µs |
| sanskrit | 1.2 | 2 | 491 | 33 | 15 µs |

Ancient Greek's verbs, 548 cells each, are most of its size: their paradigms (5.4 MB) and
accent patterns (1.6 MB). Nothing is computed before the first answer, and the same
entries always give the same crate, byte for byte.

## How the grammar is learned

Everything below is learned from the dictionary's inflection tables, and kept only where
it pays for itself.

- **Paradigms.** A word's forms are aligned on the letters they all keep (the stem
  pieces); what's left is a pattern per form: `~а`, `~о~` (a vowel the other forms drop),
  `ge~t`. Words with the same patterns share a paradigm, named after its commonest word
  that no sense marks vulgar or offensive (the dumps give no word frequencies: commonest
  means in the most dictionaries, with the most senses). A word missing some forms joins
  the fullest paradigm that builds the forms it has. A paradigm states the inherent
  features its words share (`[gender: Neuter]`), and is *like* the commoner paradigm it
  shares the most cells with: it writes only the cells that differ (`_` for the rest).
- **Sound laws.** A built form marks where its stem meets its ending (`+`). Where a word's
  paradigm, or a more general one, would build its forms but for a change there, the
  change is a law: Russian `[к|г|х|ш|ч|ж|щ]+ы → +и`, Greek `γ+σ → +ξ`, `κ+θ → +χθ`. A law
  can also change a letter of the ending by the nearest of some letters before it (a
  tier): Sanskrit `न → ण` after `र`, `ष`, `ृ` unless a dental, palatal or retroflex comes
  between. Which letters make the change, block it, or stop it after it, is learned. A
  law is kept when it gives forms of five or more words in two or more cells and breaks
  fewer than one in twenty of the forms it gives; the words it explains join the more
  general paradigm, and the forms it breaks are listed. Laws run in
  [`sound-law`](crates/sound-law).
- **Relations.** Each cell's form is told from another cell's (the one that predicts it
  best: a tree on the lemma's cell), its end changed by rules read off that form's ending
  (Latin `amāvī` → `amāvistī`). Where a feature of the word decides which cell (Russian:
  the accusative is the genitive of animate nouns, the nominative of inanimate ones), the
  relation is by that feature, and the feature is a choice of its own. A paradigm takes
  the cells none of its words has from its relations (`derived`), and a word whose forms
  are listed lists only its principal parts: its other forms are told from them, where
  that does as well as its paradigm.
- **Choices.** Which paradigm a word takes is a list of rules on its ending: the longest
  rule the word has decides, with how many of the dictionary's words with that ending
  take each paradigm. A rule is kept only when it describes its words in fewer bits
  than the broader rule does, by more than the rule costs (minimum description length). Words no rule gets
  right are listed. A rule's status follows Yang's Tolerance Principle: a rule with few
  enough exceptions is productive (`regular`); otherwise speakers vary (`unsettled`).
  (The principle wants a core of frequent words; with no frequencies in the dumps it is
  applied to every word of the rule.)
- **Compounds and prefixes.** A word written in parts inflects its last part or each part
  (Russian `диван-кровать`). German verbs inflect behind their prefixes: separable ones
  go where the form puts them (`fing an`), inseparable ones stay in front and take off
  the participle's `ge-` (`verhindert`). The prefixes, and what the participle drops, are
  learned.
- **Accent.** Where a language writes it (Ancient Greek), each cell has a rule placing the
  accent (where the lemma has it, on the ending, or as far back as allowed) and the law
  of limitation moves it and picks acute or circumflex by the syllables' length. Which
  endings take the circumflex, and where a final `-οι` counts long (the optative), are
  learned.
- **Spelling.** A word is found however it is typed: without vowel length (`rosa`,
  `λυω`), without `ё`. Rules on the letters around each vowel restore the marks (`-ātiō`,
  `dē-`), the dictionary's words that they don't spell are listed, and the answer's
  probability says how much of the spelling was a guess.
- **Irregular forms.** Whatever no rule gives is listed, as the dictionary writes it.
- **Calibration.** Probabilities are sharpened by one power per part of speech, fitted on
  a tenth of the words the grammar didn't learn from.

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
