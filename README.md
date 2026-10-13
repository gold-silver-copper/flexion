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
| english | noun | **98.59%** ± 0.2 | 98.59% | 100.00% |  | 98.59% (47466) |  | 98.39% | 97.14% | 100.00% | 0.001 |
| english | verb | **97.83%** ± 0.4 | 97.83% | 99.71% |  | 97.83% (20745) |  | 98.24% | 97.91% | 100.00% | 0.004 |
| english | adjective | **96.91%** ± 0.4 | 96.91% | 100.00% |  | 96.91% (18715) |  | 95.02% | 96.69% | 100.00% | 0.003 |
| english | adverb | **99.34%** ± 0.5 | 99.34% | 100.00% |  | 99.34% (3621) |  | 98.90% |  | 100.00% | 0.006 |
| russian | noun | **95.08%** ± 0.8 | 95.08% | 96.20% | 97.53% | 95.08% (30193) | 100.00% (8) | 94.72% | 84.48% | 100.00% | 0.007 |
| russian | adjective | **99.75%** ± 0.2 | 99.75% |  |  | 99.76% (171213) | 0.00% (12) | 99.78% | 96.60% | 100.00% | 0.002 |
| russian | verb | **96.05%** ± 1.1 | 96.05% | 98.92% | 95.79% | 96.07% (21829) | 79.31% (29) | 96.17% | 92.37% | 100.00% | 0.009 |
| latin | noun | **66.41%** ± 2.3 | 93.45% | 98.35% | 67.19% | 66.43% (18401) | 0.00% (6) | 59.45% | 68.91% | 100.00% | 0.031 |
| latin | adjective | **82.54%** ± 1.4 | 99.28% | 99.51% |  | 82.54% (102281) | 0.00% (1) | 71.85% | 87.22% | 100.00% | 0.014 |
| latin | verb | **70.47%** ± 3.6 | 85.24% | 97.17% |  | 70.44% (60845) | 84.13% (126) | 65.34% | 75.18% | 100.00% | 0.048 |
| ancient-greek | noun | **67.50%** ± 3.2 | 85.52% | 93.83% | 68.20% | 67.49% (11695) | 100.00% (5) | 49.22% | 57.82% | 99.92% | 0.061 |
| ancient-greek | adjective | **72.00%** ± 4.0 | 85.65% | 94.62% |  | 72.01% (22426) | 40.00% (10) | 50.81% | 63.66% | 99.94% | 0.060 |
| ancient-greek | verb | **63.22%** ± 5.9 | 72.09% | 74.38% |  | 63.41% (63654) | 41.09% (550) | 46.07% | 54.26% | 100.00% | 0.071 |
| sanskrit | noun | **92.13%** ± 2.2 | 92.13% |  | 97.03% | 92.13% (13801) |  | 91.70% | 93.55% | 100.00% | 0.012 |
| sanskrit | adjective | **91.25%** ± 4.0 | 91.25% |  |  | 91.29% (13449) | 0.00% (7) | 89.95% | 91.21% | 100.00% | 0.034 |
| sanskrit | verb | **88.14%** ± 6.8 | 88.14% | 88.40% |  | 88.47% (7210) | 0.00% (27) | 90.01% | 90.13% | 100.00% | 0.052 |

Where the grammar is below the baseline or the floor, and why:

- **Latin, from the headword.** A headword doesn't write vowel length (`fox` for `fōx`);
  the grammar restores it by rules on the letters around each vowel (`-ātiō`, `dē-`, a
  diphthong), and offers the second likeliest spelling too (top-3: nouns 85.3%, verbs
  90.4%), but a stem's own lengths are lexical: `iūdex`, `bōlus` and `grūs` come back as
  `iudex`, `bolus`, `grus`. Typed in full the same grammars reach 93.5% (nouns), 99.3%
  (adjectives), 85.2% (verbs), and with the principal parts 97–99%. Over all ten tenths
  the verbs score 71.8% ± 0.9 from the headword and 97.8% ± 0.3 with principal parts.
- **Ancient Greek verbs.** The augment (a law where the form's first letters meet the
  stem: `ἐ‹ἀ → ἠ`), the perfect's reduplication (a copied letter, `%ε~κα`) and the augment
  behind a preverb (`ἀπο + ἐ → ἀπε`) are learned now, and the accent can sit on a fixed
  syllable from the word's end (`λυθῇς`, `λελυμένος`). What's left is mostly stems: a verb's
  aorist or perfect stem (`ἀγγέλλω` → `ἠγγέλθην`) follows from no ending, so a new verb's
  is a guess, and known verbs list it.
- **Sanskrit.** The `n` → `ṇ` rule is a law (`तर्काणाम्`), and a noun's gender, given,
  takes it from 92.1% to 97.0%. The verbs are small (a tenth is ~300 words): over all ten
  tenths they score 85.3% ± 1.5, the tenths ranging from 82.6% to 89.3%. Few full perfect
  tables leave the reduplicated perfect unlearned.
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
compile-cost job), and one call (release: a word's candidates for one cell, over a few words and their first 40 cells):

| crate | source MB | build seconds | peak MB | `.rlib` MB | one call |
|---|---:|---:|---:|---:|---:|
| english | 0.9 | 3 | 573 | 41 | 10 µs (nouns), 5 µs (verbs) |
| russian | 2.9 | 6 | 881 | 66 | 16 µs (nouns), 22 µs (verbs) |
| latin | 1.9 | 4 | 774 | 57 | 12 µs (nouns), 18 µs (verbs) |
| ancient-greek | 4.9 | 4 | 835 | 71 | 36 µs (nouns), 67 µs (verbs) |
| sanskrit | 0.8 | 2 | 488 | 36 | 6 µs (nouns), 16 µs (verbs) |

Ancient Greek's verbs, 624 cells each, are most of its size: their irregular forms
(1.3 MB, mostly principal parts of verbs whose stems follow from no ending) and their
paradigms (1.2 MB). Nothing is computed before the first answer, and the same
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
  law can also change a copied letter (reduplication, `%`): Greek copies `φ` as `π`
  (`πεφίληκα`); or join a form's first literal with the stem's first letters (`‹`): the
  Greek augment lengthens a vowel (`ἐ‹ἀ → ἠ`, `ἐ‹αἰ → ᾐ`: `ἦγον`, `ᾔτουν`), and the
  same law takes the letter written without its breathing (inside a compound). A law
  is kept when it gives forms of five or more words in two or more cells and breaks
  fewer than one in twenty of the forms it gives; the words it explains
  join the more general paradigm, and the forms it breaks are listed. Laws run in
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
  a tenth of the words the grammar didn't learn from, typed as their headwords.

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
