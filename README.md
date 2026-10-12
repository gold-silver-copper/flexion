# flexion

Point a trainer at a language's [Kaikki](https://kaikki.org/) Wiktionary dump and get a
Rust crate that **is that language's inflectional grammar**: its declensions and
conjugations as named tables, the rules that choose them with their evidence, its accent
system, its prefixes, its irregular words. The crate inflects **any** word, attested or
not, and every answer comes with a calibrated probability, how settled it is, and why.

```rust
use russian::*;

assert_eq!(noun("книга", Case::Genitive, Number::Plural), "книг");
// A word the dictionary doesn't have: every candidate, how likely, and why.
for c in Noun::new("бокр").candidates((Case::Accusative, Number::Singular)) {
    println!("{c}");
}
// бокр (60%, regular): like приём; 5019 of 5179 words in -р
// бокра (39%, regular): like приём; 5019 of 5179 words in -р
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
write. *Principal parts given*: the dictionary's principal parts given too. *Floor*: the
longest-suffix rule, the SIGMORPHON shared tasks' non-neural baseline. *Baseline*: the
previous version of these crates (a model, not a grammar), from the bare lemma.
*Attested*: the shipped grammar on the dictionary's own words.

| language | part of speech | top-1 from the headword | lemma in full | principal parts given | floor | baseline | attested | calibration error |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| english | noun | **98.60%** ± 0.1 | 98.60% | 100.00% | 94.52% | 97.14% | 100.00% | 0.001 |
| english | verb | **97.87%** ± 0.4 | 97.87% | 99.72% | 96.29% | 97.91% | 100.00% | 0.005 |
| english | adjective | **96.59%** ± 0.5 | 96.59% | 100.00% | 36.21% | 96.69% | 100.00% | 0.024 |
| english | adverb | **99.17%** ± 0.5 | 99.17% | 100.00% | 33.66% |  | 100.00% | 0.037 |
| russian | noun | **94.96%** ± 0.8 | 94.96% | 96.11% | 92.57% | 84.48% | 100.00% | 0.003 |
| russian | adjective | **99.76%** ± 0.2 | 99.76% |  | 99.67% | 96.60% | 100.00% | 0.002 |
| russian | verb | **96.00%** ± 1.1 | 96.00% | 99.00% | 92.49% | 92.37% | 100.00% | 0.008 |
| latin | noun | **65.24%** ± 2.4 | 93.67% | 98.33% | 54.53% | 68.91% | 100.00% | 0.048 |
| latin | adjective | **81.70%** ± 1.4 | 99.24% | 99.47% | 61.20% | 87.22% | 100.00% | 0.052 |
| latin | verb | **70.33%** ± 3.6 | 85.37% | 97.49% | 60.65% | 75.18% | 100.00% | 0.030 |
| ancient-greek | noun | **61.88%** ± 3.3 | 78.74% | 89.46% | 42.88% | 57.82% | 100.00% | 0.118 |
| ancient-greek | adjective | **58.78%** ± 4.4 | 71.76% | 87.42% | 38.51% | 63.66% | 100.00% | 0.093 |
| ancient-greek | verb | **46.11%** ± 6.0 | 53.57% | 56.99% | 43.86% | 54.26% | 100.00% | 0.075 |
| sanskrit | noun | **91.58%** ± 2.3 | 91.58% |  | 92.04% | 93.55% | 100.00% | 0.022 |
| sanskrit | adjective | **90.21%** ± 4.2 | 90.21% |  | 91.12% | 91.21% | 100.00% | 0.039 |
| sanskrit | verb | **87.61%** ± 6.9 | 87.61% | 87.94% | 72.49% | 90.13% | 100.00% | 0.101 |

Where the grammar is below the baseline, and why:

- **Latin and Ancient Greek, from the headword.** A headword doesn't write vowel length
  (`fox` for `fōx`, `λυω` for `λῡω`); the grammar restores it by rules on the letters
  around each vowel (`-ātiō`, `dē-`, a diphthong), but a stem's own lengths are lexical:
  `iūdex`, `bōlus` and `grūs` come back as `iudex`, `bolus`, `grus`. Typed in full the
  same grammars reach 93.7% (Latin nouns), 99.2% (adjectives), 85.4% (verbs), and with
  the principal parts 97–99%. Latin adjectives also include participles, which the
  previous version spelled from their verbs' tables.
- **Ancient Greek verbs.** The augment goes after a preverb (`περιέφερον`, `ἐπετίθην`)
  and the perfect reduplicates (`βέβλεφα`); neither is learned yet, so such forms are
  listed for known verbs and missed for new ones.
- **Sanskrit.** A stem doesn't say its gender (`कुल` is neuter: `कुलम्`), and the rule that
  turns `n` into `ṇ` after `r`, `ṣ` or `ṛ` (`तर्काणाम्`) is not learned yet; within the
  margin for adjectives.
- **Russian** has no stress any more (stripped from input, never written), so its
  numbers aren't comparable with the baseline's, which placed stress.

## The generated code

A language crate has a module per part of speech. Excerpts from `russian`:

A paradigm: a table of patterns, `~` for the stem, named after its commonest word, with
the cells the word's animacy decides marked `referred`:

```rust
    // Приём: 27,734 words (указ, зонт, кот, узус, скот, мэр).
    declension!("приём", 27734, "~";
        nominative: "~", "~ы";
        genitive: "~а", "~ов";
        dative: "~у", "~ам";
        accusative: referred, referred;
        instrumental: "~ом", "~ами";
        prepositional: "~е", "~ах";
        noun_second_case: "~а", "~е";
    ),
```

The referral itself, and the rules choosing a declension, each with its evidence (how
many of the dictionary's words with that ending take each declension):

```rust
pub(super) static REFERRALS: [Referral<'static>; 2] = [
    Referral { cell: ACC_SG, to: Cow::Borrowed(&[GEN_SG, NOM_SG]) },  // animate, inanimate
    Referral { cell: ACC_PL, to: Cow::Borrowed(&[GEN_PL, NOM_PL]) },
];

        "ба" => [Полнота 241, Какао 10, Усадьба 4, Анкара 1],
        "ьба" => [Полнота 32, Усадьба 4, Анкара 1],
        "дьба" => [Полнота 6, Усадьба 4],
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

And what no rule gives, as the dictionary writes it (a table: word, cell, form):

```text
автомат-манипулятор	7	автоматы-манипуляторы
```

## What it costs

A clean debug build of each crate, one at a time (one machine; CI's numbers are in its
compile-cost job), and one call:

| crate | source MB | build seconds | peak MB | `.rlib` MB | one call |
|---|---:|---:|---:|---:|---:|
| english | 2.9 | 10 | 949 | 102 | 11 µs |
| russian | 2.6 | 6 | 726 | 62 | 16–18 µs |
| latin | 2.4 | 7 | 1,303 | 52 | 8 µs |
| ancient-greek | 21.3 | 29 | 11,109 | 134 | 35 µs |
| sanskrit | 1.3 | 2 | 426 | 33 | 4 µs |

Ancient Greek's verbs, 548 cells each, are most of its size. Nothing is computed before
the first answer, and the same entries always give the same crate, byte for byte.

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
  take each paradigm (its main ones; the rest lumped, `+ N`). A rule is kept only when it
  says something the broader rule doesn't (another paradigm, or another productivity:
  Wilson & Li's pruning) and describes its words in fewer bits than the broader rule
  does, by more than the rule costs (minimum description length). Words no rule gets
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
