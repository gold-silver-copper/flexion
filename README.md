# flexion

Describe a language's grammar, point the trainer at a [Kaikki](https://kaikki.org/)
Wiktionary dump, and get a Rust crate that inflects **any** word of that language
(attested or not) with a calibrated confidence for every answer.

This repository builds two languages from the same trainer, with no language-specific
code in the trainer or the runtime:

| crate | held-out words (never seen in training), top-1 | attested words, top-1 |
|---|---|---|
| [`english`](languages/english) | nouns 97.15%, verbs 97.91%, adjectives 96.67%, adverbs 99.00% (old hand-written rules: 95.76%, 92.51%, 96.15%, 99.00%) | 99.96–100% (`english` 0.4: 98.89–99.43%) |
| [`russian`](languages/russian) | from the bare lemma: nouns 81.4%, adjectives 90.4%, verbs 87.2% (stress is the hard part: letters alone are right 94.7%, 99.8%, 95.8%); given the dictionary's principal parts: nouns 94.5% (96.8% with gender/animacy markers), verbs 98.1%; adjectives with a stressed lemma 99.7% | 99.8–100% |

Attested words are the ones the shipped model was trained on. The full reports (calibration,
per-cell numbers, made-up words, the largest groups of mistakes, size and speed) are in
each crate's README. The English model is 808 KB against `english` 0.4's 1026 KB of tables,
and a call takes 1.4–2.6× as long as `english` 0.4's (about 100–150 ns). The Russian model is
4.4 MB (1.8 MB gzipped), 200–370 ns per call.

```rust
assert_eq!(english::noun("child", english::Number::Plural), "children");
assert_eq!(english::noun("glorp", english::Number::Plural), "glorps");
assert_eq!(english::verb("lie, lied", english::VerbForm::Past), "lied");

use russian::{Case, Number};
assert_eq!(russian::noun("книга", Case::Genitive, Number::Plural), "кни́г");
assert_eq!(russian::noun("кот, кота m anim", Case::Accusative, Number::Singular), "кота́");
```

## Layout

```
crates/flexion         the runtime every generated crate depends on (no dependencies)
crates/flexion-train   extract → learn → evaluate → emit
configs/               one language description per module: english.rs, russian.rs
configs/extra/         hand-written helpers copied into a generated crate
languages/english      generated (don't edit by hand)
languages/russian      generated
tools/english-baseline the published english 0.4, as a baseline for the English report
xtask/                 cargo xtask build | probe | extract
```

## How it learns

1. **Extract.** For every entry of a configured part of speech, the trainer keeps the
   plain forms of each cell in Wiktionary's listing order. Cells are named by the
   config: a group of cells is the product of some features (`Case × Number`), and each
   feature value is a set of Wiktionary tags (`genitive`, `plural`). Nonstandard,
   dialectal, dated and similar forms are dropped (the config says which tags those are).
2. **Edits.** Each (lemma, form) pair becomes an edit: an added prefix, a stripped and
   an added suffix (`city → cities` is "−y +ies", `beautiful → more beautiful` is the
   prefix "more "), and, for languages that write stress, where the stress goes relative
   to that split (same vowel as the lemma, first vowel of the ending, last vowel of the
   stem, …). Stress is part of the edit, not part of the letters, so `рука́ → ру́ки` and
   `нога́ → но́ги` share one edit.
3. **Classes.** A lexeme's edits across all its cells, plus its inherent features
   (gender, animacy, aspect), form its **inflection class**. Classes are found in the
   data, not taken from a grammar book. Later-listed variants become extra observations
   with half the weight of the one before.
4. **Tree.** Observations are counted at every node of a tree over the lemmas, read from
   the last letter back to the start of the word; a syllable-count symbol can be inserted
   at a learned depth (English: `-er` vs `more`). Top-down, each node's counts are
   smoothed toward its nearest kept ancestor, and a node is kept only if it differs from
   that ancestor by a likelihood-ratio test. A whole-word node is kept only if dropping it
   would change one of the word's attested forms. So general patterns live near the root,
   exceptions deep down, and nothing is stored twice.
5. **Tune and measure.** A hash of the lemma splits the words into tenths. One tenth
   chooses the hyperparameters (smoothing, pruning, suffix support, the syllable symbol)
   by the one-standard-error rule; another is held out and only ever scored. The shipped
   model is then trained on everything.
6. **Emit.** The crate gets typed functions per cell group, a settings value (stress
   marks on or off), the model as a binary blob, the config's hand-written helpers, and a
   README with the evaluation.

At runtime a word is walked down the tree; the deepest node with a class that fits it
(the class's stripped suffixes must match) decides. A **citation** (`"lie, lied"`,
`"книга, книги"`) keeps only the classes that agree with the given principal parts, and
**markers** (`m`, `anim`, `impf`) weigh classes by the features they name.

## Adding a language

A config is a [`Language`](crates/flexion-train/src/spec.rs): features (with the
[`features!`](crates/flexion-train/src/spec.rs) macro), parts of speech and their cell
groups, citation format, spelling (case folding, stress mark, vowels, letter folding),
tag policy, and a small [`Hooks`](crates/flexion-train/src/spec.rs) impl for editorial
quirks (which lemmas and forms are admissible; inherent features parsed from the head
line). See [`configs/src/russian.rs`](configs/src/russian.rs). Then:

```sh
cd flexion
cargo xtask build <name> --dump kaikki.org-dictionary-<Language>.jsonl --date <dump date>
```

### What a third language would need

- **German**: a config like Russian's (case × number nouns with gender; strong/weak/mixed
  adjective tables; verb person × number × tense/mood), and `ß`/umlaut-aware folding in
  the spelling section. Umlaut plurals (`Mann → Männer`) and the `ge-…-t` participle are
  already expressible: an edit strips and adds at the end and may add a prefix. Separable
  prefixes (`anfangen → fängt an`) are not: they need an edit that moves material, which
  the runtime doesn't have.
- **Latin**: straightforward: five declensions and four conjugations are suffix edits;
  macrons as letters (or folded for lookup), and the four principal parts as the verb
  citation.
- **Turkish**: the trainer's whole-paradigm classes don't fit agglutination: a Turkish
  verb has thousands of cells. It would need per-suffix-slot learning (classes per
  morpheme slot, conditioned on the vowel and consonant classes the previous slot ends
  in), which is a trainer change.

## Commands

```sh
cd flexion
cargo xtask build english --dump ~/.cache/flexion/dumps/English.jsonl --date 2026-10-03
cargo xtask build russian --dump ~/.cache/flexion/dumps/Russian.jsonl --date 2026-10-03
cargo xtask extract russian --dump ~/.cache/flexion/dumps/Russian.jsonl   # cache lexemes
cargo xtask probe russian --pos noun --tau 2 --misses 1                   # experiment
```

Dumps: `https://kaikki.org/dictionary/<Language>/kaikki.org-dictionary-<Language>.jsonl.gz`.
A build streams the dump (it is never loaded whole) and takes about a minute per
language on 8 threads.

## License

Code: MIT or Apache-2.0. Generated models (`languages/*/src/model.bin`) and the test
fixtures are derived from Wiktionary and licensed CC BY-SA 4.0.
