# flexion

Describe a language's grammar, point the trainer at a [Kaikki](https://kaikki.org/)
Wiktionary dump, and get a Rust crate that inflects **any** word of that language
(attested or not) with a calibrated confidence for every answer.

This repository builds four languages from the same trainer, with no language-specific
code in the trainer or the runtime:

| crate | held-out words (never seen in training), top-1 | attested words, top-1 |
|---|---|---|
| [`english`](languages/english) | nouns 97.15%, verbs 97.91%, adjectives 96.67%, adverbs 99.00% (old hand-written rules: 95.76%, 92.51%, 96.15%, 99.00%) | 99.96–100% (`english` 0.4: 98.89–99.43%) |
| [`russian`](languages/russian) | from the bare lemma: nouns 81.4%, adjectives 90.4%, verbs 87.2% (stress is the hard part: letters alone are right 94.9%, 99.8%, 96.3%); given the dictionary's principal parts: nouns 94.5% (96.8% with gender/animacy markers), verbs 98.1%; adjectives with a stressed lemma 99.7% | 99.8–100% |
| [`latin`](languages/latin) | given the dictionary's principal parts: nouns 98.0% (98.3% with the gender marker), adjectives 99.6%, verbs 97.7%; from the bare lemma 59.1%, 71.8%, 66.5% (vowel length is the hard part: letters alone are right 93.3%, 99.4%, 90.2%) | 99.7–100% |
| [`polish`](languages/polish) | from the bare lemma: nouns 93.9%, adjectives 99.98%, verbs 96.5%; given the principal parts: nouns 96.3% (98.2% with gender markers), verbs 97.8% | 99.9–100% |

Attested words are the ones the shipped model was trained on. The full reports (calibration,
per-cell numbers, made-up words, the largest groups of mistakes, size and speed) are in
each crate's README. The English model is 818 KB against `english` 0.4's 1026 KB of tables,
and a call takes about 150 ns, two to three times as long as `english` 0.4's. Russian is
4.6 MB (1.9 MB gzipped), Latin 6.8 MB (2.5 MB gzipped) and Polish 1.9 MB (0.8 MB gzipped).
Every call takes under a microsecond.

Russian nouns given only the nominative and genitive singular reach 94.5%, short of the 95%
the project aimed for. Nearly all remaining misses need facts spelling doesn't carry: the
accusative follows animacy (`адвоката` vs `айсберг`), and some plurals move the stress
(`ба́сы` vs `басы́`). Given the markers a dictionary prints with the word (`кот, кота m anim`),
nouns reach 96.8%.

A Latin lemma as people type it (`rosa`, `rex`) doesn't say which vowels are long, and
for a word the model never saw that can't be told from the spelling. So a bare unseen
lemma gets its letters right 90–99% of the time, but its macrons only 59–72%. The
confidences say so, and the second guess is often the right one: top-3 is 73–81%. Given
the citation a dictionary prints (`rēx, rēgis m.`, `amō, amāre, amāvī, amātum`), whose
parts spell the stem in full, nouns reach 98.0% and verbs 97.7%.

```rust
assert_eq!(english::noun("child", english::Number::Plural), "children");
assert_eq!(english::noun("glorp", english::Number::Plural), "glorps");
assert_eq!(english::verb("lie, lied", english::VerbForm::Past), "lied");

use russian::{Case, Number};
assert_eq!(russian::noun("книга", Case::Genitive, Number::Plural), "кни́г");
assert_eq!(russian::noun("кот, кота m anim", Case::Accusative, Number::Singular), "кота́");

assert_eq!(latin::noun("rex", latin::Case::Genitive, latin::Number::Singular), "rēgis");
assert_eq!(latin::noun("rēx, rēgis m.", latin::Case::Dative, latin::Number::Plural), "rēgibus");

use polish::{AdjectiveCase, Agreement, Animacy};
assert_eq!(polish::noun("dom, domu", polish::Case::Locative, polish::Number::Singular), "domu");
assert_eq!(polish::adjective("dobry", AdjectiveCase::Nominative, Agreement::Virile, Animacy::Animate), "dobrzy");
```

## Layout

```
crates/flexion         the runtime every generated crate depends on (no dependencies)
crates/flexion-train   extract → learn → evaluate → emit
configs/               one language description per module: english.rs, russian.rs, latin.rs, polish.rs
configs/extra/         hand-written helpers copied into a generated crate
languages/english      generated (don't edit by hand)
languages/russian      generated
languages/latin        generated
languages/polish       generated
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
   marks or macrons on or off), the model as a binary blob, the config's hand-written
   helpers, and a README with the evaluation.

Marks that people leave out when they type (Latin macrons, Russian `ё`) are folded for
lookup. Each class keeps a hidden cell with the lemma written in full, and a separate
one-cell **restoration model** learns how a typed lemma is written in full (`rex →
rēx`). Forms are weighed over its readings, so a word whose vowel length can't be known
gets a low confidence instead of a confident guess. A principal part given in a citation
spells the stem in full, so it decides the reading itself.

At runtime a word is walked down the tree; the deepest node with a class that fits it
(the class's stripped suffixes must match) decides. A **citation** (`"lie, lied"`,
`"книга, книги"`) keeps only the classes that agree with the given principal parts, and
**markers** (`m`, `anim`, `impf`) weigh classes by the features they name.

## Adding a language

A config is a [`Language`](crates/flexion-train/src/spec.rs): features (with the
[`features!`](crates/flexion-train/src/spec.rs) macro), parts of speech and their cell
groups, citation format, spelling (case folding, stress mark, vowels, letter folding),
tag policy, and a small [`Hooks`](crates/flexion-train/src/spec.rs) impl for editorial
quirks: which lemmas and forms are admissible, inherent features parsed from the head
line, and `repair` for tables whose tags are broken (Polish conjugation tables leave the
first and second person untagged). See [`configs/src/russian.rs`](configs/src/russian.rs)
and [`configs/src/polish.rs`](configs/src/polish.rs). Then:

```sh
cd flexion
cargo xtask build <name> --dump kaikki.org-dictionary-<Language>.jsonl --date <dump date>
```

### What the next language would need

Latin and Polish needed only configs plus generic changes to the trainer and runtime
(marks folded for lookup and restored from a learned model, a hook to repair broken tags,
degree forms read from the head line, template names matched by prefix). For the next
ones:

- **German**: a config like Polish's (case × number nouns with gender; strong/weak/mixed
  adjective tables; verb person × number × tense/mood), and `ß`/umlaut-aware folding in
  the spelling section. Umlaut plurals (`Mann → Männer`) and the `ge-…-t` participle are
  already expressible: an edit strips and adds at the end and may add a prefix. Separable
  prefixes (`anfangen → fängt an`) are not: they need an edit that moves material, which
  the runtime doesn't have.
- **Ancient Greek**: the closest to Latin and Russian together: macron-like length marks
  and a written pitch accent that moves (the stress model covers one accent position, but
  not the three accent kinds), and verbs whose augment and reduplication are prefix edits
  that depend on the stem (`λύω → ἔλυσα, λέλυκα`). The accent kinds need a stress rule
  that also chooses the mark, which is a runtime change.
- **Turkish**: the trainer's whole-paradigm classes don't fit agglutination: a Turkish
  verb has thousands of cells. It would need per-suffix-slot learning (classes per
  morpheme slot, conditioned on the vowel and consonant classes the previous slot ends
  in), which is a trainer change.

## Commands

```sh
cd flexion
cargo xtask build english --dump ~/.cache/flexion/dumps/English.jsonl --date 2026-10-03
cargo xtask build russian --dump ~/.cache/flexion/dumps/Russian.jsonl --date 2026-10-03
cargo xtask build latin --dump ~/.cache/flexion/dumps/Latin.jsonl --date 2026-10-03
cargo xtask build polish --dump ~/.cache/flexion/dumps/Polish.jsonl --date 2026-10-03
cargo xtask extract russian --dump ~/.cache/flexion/dumps/Russian.jsonl   # cache lexemes
cargo xtask probe russian --pos noun --tau 2 --misses 1                   # experiment
```

Dumps: `https://kaikki.org/dictionary/<Language>/kaikki.org-dictionary-<Language>.jsonl.gz`.
A build streams the dump (it is never loaded whole) and takes one to two minutes per
language on 8 threads.

## License

Code: MIT or Apache-2.0. Generated models (`languages/*/src/model.bin`) and the test
fixtures are derived from Wiktionary and licensed CC BY-SA 4.0.
