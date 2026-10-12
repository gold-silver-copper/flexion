# flexion

Point a trainer at a language's [Kaikki](https://kaikki.org/) Wiktionary dump and get a
Rust crate that **is that language's inflectional grammar**: its declensions and
conjugations as named tables, the rules that choose them with their evidence, its accent
system, its prefixes, its irregular words. The crate inflects **any** word, attested or
not, and every answer comes with a calibrated probability, how settled it is, and why.

```rust
use russian::*;

assert_eq!(noun("книга", Case::Genitive, Number::Plural), "книг");
for c in Noun::new("бокр").candidates((Case::Genitive, Number::Singular)) {
    println!("{c}"); // бокра (…%, regular): like приём; … words in -р
}
```

Ten languages are built from the same trainer, with nothing language-specific in the
trainer or the runtime: [English](languages/english), [Russian](languages/russian),
[Latin](languages/latin), [Polish](languages/polish), [German](languages/german),
[Ancient Greek](languages/ancient-greek), [Sanskrit](languages/sanskrit),
[Spanish](languages/spanish), [French](languages/french) and [Italian](languages/italian).

RESULTS

## The generated code

A language crate has a module per part of speech. Excerpts from `russian`:

EXCERPTS

## How the grammar is learned

Everything below is learned from the dictionary's inflection tables, and kept only where
it pays for itself.

- **Paradigms.** A word's forms are aligned on the letters they all keep (the stem
  pieces); what's left is a pattern per form: `~а`, `~о~` (a vowel the other forms drop),
  `ge~t`. Words with the same patterns share a paradigm, named after its commonest word.
  A word missing some forms joins the fullest paradigm that builds the forms it has.
- **Choices.** Which paradigm a word takes is a list of rules on its ending: the longest
  rule the word has decides, with how many of the dictionary's words with that ending
  take each paradigm. A rule is kept only when it describes its words in fewer bits than
  the broader rule does, by more than the rule costs (minimum description length). Words
  no rule gets right are listed. A rule's status follows Yang's Tolerance Principle: a
  rule with few enough exceptions is productive (`regular`); otherwise speakers vary
  (`unsettled`).
- **Referrals.** A cell whose form is another cell's, depending on a feature of the word
  (Russian: the accusative is the genitive of animate nouns, the nominative of inanimate
  ones; Polish likewise for masculine animates), is found from the data and written
  `referred`; the feature becomes a choice of its own.
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
- **Principal parts.** A part of speech can be learned as the systems of its principal
  parts (Latin `amō, amāre, amāvī, amātum`): each cell is inflected from the part it is
  told from most regularly. It is used where it does as well on held-out words with
  fewer forms listed.
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
