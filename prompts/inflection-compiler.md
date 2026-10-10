# Prompt: a generic trainer that turns a language description and Wiktionary into an inflection crate (English and Russian)

Build a system that takes **a description of a language** (its features, cells,
citation format, spelling rules and extraction quirks) plus **a Kaikki
Wiktionary dump**, learns that language's inflection from the data, and writes
out **a finished Rust crate** that inflects any word of the language, attested
or not, with a calibrated confidence for every answer.

Deliver it with exactly two languages, **English and Russian**, both generated
by the same trainer with no language-specific code in the trainer. English has
an existing library to beat. Russian is the opposite kind of language (six
cases, stress, gender you can't see in the spelling, rich verb paradigms) and
proves the system is really generic.

This is one PR in a new repo (section 8). You own it fully: architecture,
names, crate boundaries, formats, tests and API are yours to decide. The design
below is the intended direction, not a spec. If measurements point somewhere
better, follow them and explain why in the final report. No prototypes or
spikes: the PR delivers the finished system and both finished crates.

---

## 1. What exists today

All under `~/code/linguistics/` (clean clones in sync with GitHub):

- **`english/`** (`gold-silver-copper/english`, published as `english` 0.4.0).
  `english-core` holds hand-written regular rules. `english` looks up
  Wiktionary-derived perfect-hash exception tables (`generated/*_phf.rs`)
  and falls back to the rules. `extractor` streams the Kaikki dump, ranks plain
  vs. soft-tagged forms (nonstandard, dialectal…), picks a canonical form per
  slot from head-template order, drops forms the rules predict, and numbers
  homographs and variants as `_<n>` keys (`die_2` → dice). The numbers change
  whenever the dump's forms change. `xtask` has `refresh-data`,
  `check-registry` and `accuracy`. Current numbers (README, 2026-07-02 dump):
  - any-key recall: 100% for nouns and verbs, 99.99% for adjectives and adverbs;
  - **bare-lemma** accuracy (the natural call returns the primary attested
    form): nouns 98.94%, verbs 99.50%, adjectives 99.48%, adverbs 99.03%;
  - about 90–165 ns per call, and about 25k lines of generated tables.

  The extractor's **editorial policy** is hard-won knowledge. Read it and
  carry it into the English config: what counts as a plain attested form, how
  listing order marks the canonical form, periphrastic adjectives, flat and
  suppletive adverbs, casing and acronyms, multi-word entries.
  `~/code/english` is an identical older clone: ignore it.
- **`church-slavonic/`**: `crates/church-slavonic-tools/src/sources/kaikki.rs`
  is a second Kaikki reader (entry schema, sentinel and table-form detection,
  "form of" handling, tag → case/number/person). Its typed cells with
  round-trip names (`crates/church-slavonic/src/cell.rs`) are a good model for
  generated cell types. Read both, don't modify them.
- **`ruthenian/crates/sound-law`**: a generic, dependency-free rewrite engine
  with declared segment classes. Its class notation is a candidate for
  declaring the sound classes the learner uses as features (vowels, consonants,
  soft/hard). Use it as a dependency or a model as you see fit. Don't modify it
  in place.

No dumps are on this machine. Download Kaikki's per-language JSONL for English
and Russian (kaikki.org; same entry schema as the raw dump, much smaller) into
a gitignored data directory or `~/.cache/`, never `/tmp`. Record the dump dates.

## 2. The shape of the system

```
trainer        extract  → learn  → evaluate → emit
               (Kaikki reader + language config → training pairs → model
                → held-out report → a complete crate directory)
runtime        tiny, dependency-free crate every generated crate depends on:
               table walk, applying edits, citation parsing, scored results
configs/       one language description each: english, russian
crates/        the generated crates, committed: english, russian
```

- **A generated crate is just types plus data**, on top of the shared runtime:
  feature enums, cell types, typed functions, trained tables, docs, and a
  README that includes its own evaluation report. Its only dependency is the
  runtime crate (and whatever tiny, justified dependency the runtime truly
  needs, e.g. Unicode normalization for Russian stress marks). No I/O, no
  serde, no model files read at runtime, no panics.
- **The trainer has no language names in it.** Everything language-specific
  lives in the config. A config is Rust: a declarative macro for the common part
  plus a hooks trait for editorial quirks the macro can't express. Not TOML:
  English's extractor showed that some editorial knowledge will always need a
  few lines of code. Starting sketch:

  ```rust
  language! {
      name: russian,
      wiktionary: "ru",
      features {
          Case   { Nominative: "nominative", Genitive: "genitive", Dative: "dative",
                   Accusative: "accusative", Instrumental: "instrumental",
                   Prepositional: "prepositional" }
          Number { Singular: "singular", Plural: "plural" }
          Gender { Masculine: "masculine", Feminine: "feminine", Neuter: "neuter" }
          Animacy { Animate: "animate", Inanimate: "inanimate" }
      }
      noun {
          cells: Case × Number,
          inherent: Gender, Animacy,          // predicted for unseen words too
          citation: [Nominative.Singular, Genitive.Singular],
      }
      sounds { V = ["а", "е", "ё", "и", "о", "у", "ы", "э", "ю", "я"]; }
      spelling { stress: combining_acute }
  }
  ```

  The config declares: features and values with the Wiktionary tags that mean
  them; cells per part of speech; inherent features (Russian gender, animacy,
  verb aspect); citation format (which cells are principal parts); sound
  classes for features; spelling normalization (stress marks, ё/е, casing);
  extraction hooks.
- **Commands** (names are yours): one to train and emit a language's crate from
  a dump, one to evaluate. Deterministic: same dump + same config → the same
  generated crate, byte for byte, and the same numbers.

## 3. The learner

### Edits

Each attested (lemma, cell, form) becomes an edit. Use a representation that
covers suffixes, prefixes and internal changes: **edit trees** (align the
two forms on their shared part and record the change before, inside and after
it), as spaCy and UDPipe do. English needs suffixes and internal changes
(sing → sang, man → men). Russian needs suffixes, fleeting vowels
(день → дня), consonant alternations (рука → руке, писать → пишу) and stress
moves.

### Inflection classes, not independent cells

Cells move together. In Russian, a noun's genitive singular and stress pattern
largely fix its other eleven forms. So:

- **Find inflection classes in the data:** groups of lemmas that make the same
  set of edits across all of a part of speech's cells. Wiktionary's own class
  annotations (e.g. Russian Zaliznyak indices in the declension templates) may
  be used to check or label the discovered classes. Prediction for unseen
  words must not depend on them, since a user's bare word won't carry one.
- **Predict the class from the word**, with a model that starts as a pruned
  tree over the word read from its end and adds features where they help:
  syllable count (English -er vs. more), sound classes, prefixes, compound
  heads, multi-word heads (passer-by → passers-by), capitalization, inherent
  features.
- **Prune** nodes whose prediction isn't meaningfully different from their
  parent's, by a principled criterion (minimum description length, or a
  significance test validated on held-out data), never per-word hand tuning.
  What survives is the general patterns near the root and the true exceptions
  deep down. Pruning replaces English's "drop what the rules predict" step.
- **Smooth**, so tiny counts never give 0 or 1 confidence on unseen words.
- **Citations are evidence.** When the caller gives principal parts
  (`"книга, книги"`, `"lie, lied"`), they narrow the class distribution and
  every other cell's confidence follows. Parts the caller gives are output
  exactly.
- **Inherent features are predicted too:** `russian::gender("кофе")` returns
  a scored answer.
- Agglutinative languages (Turkish, Finnish) have too many cells for whole-class
  prediction and will need per-suffix-slot learning. They're out of scope.
  Don't build for them, but don't make choices that rule them out. Say in the
  report what they would need.

### Compactness

Train by counting or similarly cheap fitting, store as generated Rust tables,
evaluate by a table walk. No ML runtime dependency. A neural model is out of
scope. If you believe one would clearly win somewhere, measure the gap against
the compact model and put it in the report as a follow-up.

### Variants, homographs, closed classes

- Wiktionary lists variants in an editorial order without frequencies. Turn the
  order into weights by a simple, stated scheme, and document that variant
  confidences reflect Wiktionary's ordering, not measured usage.
- Plain forms only as main answers. Soft-tagged forms (nonstandard,
  dialectal, obsolete…) are either labelled low-weight alternatives or
  excluded. Decide which, and justify it.
- Homographs that inflect differently (lie: lay / lied) yield one honest
  distribution, not an arbitrary pick.
- Closed classes (pronouns, `be`, Russian pronouns and numerals) are
  memorized from Wiktionary like any other attested word. Hand-written tables
  only where Wiktionary's data is broken, recorded in the config with the reason.

## 4. Generated API

The same shape for every language, generated from the config. Sketch:

```rust
// Quick: total, never panics, unseen words inflect by prediction.
english::noun("cat", Number::Plural)                               // "cats"
russian::noun("книга", Case::Genitive, Number::Plural)             // "книг"

// Scored: every candidate with confidence, best first.
english::noun_scored("octopus", Number::Plural)
// [("octopuses", 0.6), ("octopi", 0.3), ("octopodes", 0.1)]
russian::noun_scored("кофе", Case::Genitive, Number::Singular)

// Citations pin principal parts; the rest is predicted given them.
english::verb("lie, lied", Tense::Past, …)                         // "lied"
russian::noun("путь, пути", Case::Instrumental, Number::Singular)

// Inherent features, scored.
russian::gender("тетрадь")                                         // [(Feminine, 0.9…), …]

// Settings for systematic choices, where the data supports them.
let ru = Russian::default().stress(false);
```

- Typed functions take positional features for small cells and a cell struct
  for big ones (Russian participles). No function with six positional enums.
- Citation format per language, documented in the generated crate: English
  verbs present, past, past participle; nouns singular, plural; adjectives
  positive, comparative, superlative. Russian nouns nominative + genitive
  singular; verbs infinitive + 1st and 2nd singular present/future, the
  traditional dictionary citation. Define multi-word lemmas and malformed input
  (never a panic).
- Input matching ignores what users don't type: Russian stress marks, and ё
  written as е. Output stress marks are a setting (on by default if the data
  supports it).
- English's `_<n>` keys are gone, with no compatibility layer. Pre-release.
- Region (UK/US) and similar settings exist only if Wiktionary's labels
  support them well enough to measure. Otherwise leave them out and say why.
- Keep English's useful helpers (`count`, possessives, capitalization) where
  they still make sense, as hand-written code in the English config or crate.
- **Out of scope:** analysis (form → lemma + cell) and any sentence-level or
  context-aware inflection. Don't build them, don't design for them, and don't
  discuss them in the report.

## 5. Coverage

- **English:** noun plural; verb third singular, past, past participle,
  present participle; adjective and adverb comparative and superlative
  (suffixal or periphrastic); pronouns and `be`.
- **Russian:** nouns (6 cases × 2 numbers, plus second genitive and second
  locative where attested, gender and animacy inherent, animacy-dependent
  accusative); adjectives (long forms in every case for m/f/n/plural, short
  forms, comparative); verbs (aspect inherent, non-past 6 forms, past
  m/f/n/plural, imperative, infinitive, the four participles, which then
  decline as adjectives, and both gerunds); pronouns; numerals as attested.
  Aspect pairs and other derivation are out of scope.

## 6. Evaluation (the core of the PR)

Run for both languages, from one command, written into each generated crate's
README:

- **Baseline first (English):** before replacing anything, download the dump,
  run the old `cargo xtask accuracy` in `~/code/linguistics/english`, and
  score the old library on the new held-out split (rules alone, and rules +
  tables). Record it.
- **Held-out split by lemma**, not by form, chosen deterministically (hash of
  the lemma, about 10%). Train without those lemmas, then score them. Report
  top-1 and top-3 per part of speech and per cell: from the bare lemma, and for
  Russian also from the nominative + genitive citation.
- **Calibration:** bucket predictions by confidence and report how often each
  bucket was right, plus expected calibration error.
- **Attested accuracy:** trained on everything, scored on every plain attested
  form, both "primary form from the natural call" and "reachable as any
  returned alternative".
- **Inherent features:** held-out accuracy of predicted Russian gender and
  animacy.
- **Error groups:** the largest clusters of held-out misses, with examples.
  Fix the ones that are feature, config or data-policy problems; explain the
  rest.
- **Made-up words:** a short list per language (glorp, spling; бузять,
  кракозябра…) with what a native speaker would expect, judged by eye in the
  report. Not an exact-match test.
- **Outside comparison (Russian, optional):** if `pymorphy3` installs cleanly
  into a virtualenv under `~/.cache/` (never system-wide), compare its
  predictions for unknown words with the Russian crate's on the held-out set.
- Size of the generated tables, packaged crate size (crates.io's limit is
  10 MB), and speed per call.

**Ship bars:**

- **English** replaces the old design only if all of these hold:
  - held-out top-1 is at least as good as the old rules on every part of speech;
  - attested accuracy (both kinds) is at least the old numbers;
  - calibration is reasonable;
  - tables are at most about 2× the old size;
  - calls are at most about 2× slower.

  If a hand-written English rule still measurably wins for some cell, it may
  stay in the English config as a hook or feature. The measurement decides,
  not habit.
- **Russian:**
  - attested primary-form accuracy is at least 99%;
  - calibration is reasonable;
  - it packages under 10 MB;
  - sub-microsecond calls;
  - held-out accuracy is reported honestly. Aim for at least 95% top-1 on
    nouns given the nominative + genitive citation, and explain if it's missed.
- If a bar isn't met, say so plainly with the numbers. Don't lower it.

**Generality audit** (part of the PR): grep the trainer and runtime for anything
English- or Russian-specific and move it into configs. Then write down, for a
third language (Latin, German, Turkish), what its config would need and whether
anything would require trainer changes.

## 7. Rules for how you work

- Never write to `/tmp` (it's RAM-backed). Use `~/.cache/claude-tmp/` or a
  gitignored directory in the repo.
- The machine has 32 cores and 125 GB RAM, and a runaway run has OOM-killed the
  terminal before. Cap parallelism (`cargo … -j 8`, `--test-threads 8`).
  Stream dumps line by line, never load one whole. Training and evaluation
  must each finish in under about 10 minutes per language. If one can't, ask
  the user before running it. Downloading dumps is fine.
- Non-test code must not panic: no `unwrap`, `expect`, indexing or `panic!`,
  enforced with workspace-level clippy `deny` lints. That includes generated
  code.
- Tests: few and meaningful. No snapshot, golden-file or exact-parity tests
  that freeze output. CI has no dumps: use a small committed fixture of real
  Kaikki lines per language, enough to train, emit and exercise a tiny crate
  end to end.
- Idiomatic Rust, correctness through simplicity. Prefer behavior that comes
  out of the data over special-case code.
- Subagents and workflows are allowed: parallelize feature experiments and
  audits under the resource caps.
- Keep a progress log at `~/code/notes/projects/<repo-name>/progress.md` (what's
  done, current numbers, decisions and their reasons, what's next), updated
  after every milestone, so the work can be resumed if interrupted.
- When you finish, kill every process, tmux session and background shell you
  started.

## 8. Repo and Git

- Create a new repo under `gold-silver-copper` with a short name you choose
  (check GitHub and crates.io availability). Build it in `~/code/<repo-name>/`
  and copy this prompt to its `prompts/inflection-compiler.md`. Push an initial
  commit with the skeleton and CI to `main`, do the work on a branch, open one
  PR, and merge it with `gh pr merge --merge` once CI is green. Then check out
  the latest `main`.
- The generated English crate is the successor to the published `english`
  crate: same crate name, breaking version bump. Choose the Russian crate's
  name after checking crates.io. Leave every crate ready to publish: metadata,
  `cargo package --list` checked so no dump data gets packaged, and Wiktionary
  attribution plus the CC BY-SA data license stated for every generated crate.
- Do **not** publish to crates.io, run `cargo publish`, create tags or
  releases, or touch the existing `english`, `church-slavonic` and `ruthenian`
  repos. The user does that.

## 9. Final report

- Repo URL and PR link.
- Per language, a table of numbers: held-out top-1/top-3 per part of speech
  (and, for English, the old rules on the same split); attested accuracy (both
  kinds); calibration; inherent-feature accuracy (Russian); size; speed;
  whether the ship bar was met. Plus the pymorphy comparison if run.
- The system as built: config format, edit representation, how classes are
  found, features kept and dropped with their measured effect, the pruning
  criterion, the variant weighting.
- The generated API, with English before/after code.
- The generality audit and what a third language would need.
- The biggest remaining error groups per language, and what would fix them.
- How you verified it in practice, and copy-pasteable commands starting with
  `cd ~/code/<repo-name>` to: inflect a few English and Russian words with
  confidence, regenerate a crate from a dump, and rerun the evaluation.
