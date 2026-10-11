# Prompt: models as Rust code, with learned sound laws

One PR for `~/code/flexion` (GitHub `gold-silver-copper/flexion`, branch `main`). Start
from the latest `main`. It must already contain PR #8 (format v7, the start-restoration
model, the size price); if `git log main` doesn't show it, stop and ask.

Today every language crate ships its model as `src/model.bin`. A generated `lib.rs`
loads it with `include_bytes!` and `flexion::Model::from_bytes`, and the `flexion`
runtime decodes the bytes. Every edit the model knows is a plain string swap, learned
separately for every ending it occurs with.

This PR does three things, in this order:

0. **Move `sound-law` into this repo.** The sound-change engine lives today in
   `gold-silver-copper/ruthenian` (`crates/sound-law`). It becomes `crates/sound-law`
   here, with its history, and is deleted from `ruthenian`.
1. **Stage 1: models as Rust code.** The binary format goes away. The trainer writes
   each model as Rust source:
   - the suffix tree becomes `match` statements over the word's letters;
   - inflection classes become an `enum`;
   - the lexicon and the join prefixes become `match`es on strings.

   Answers don't change, and an audit proves it.
2. **Stage 2: learned sound laws.** The trainer learns each language's phonology as
   ordered rewrite rules: stem changes such as Polish `k → c` before a softening `-e`,
   or Greek perfect reduplication. It runs them with the
   [`sound-law`](https://github.com/gold-silver-copper/ruthenian/tree/master/crates/sound-law)
   engine and emits them as a `sound_laws!` block in each crate. Edits shrink to
   endings plus rules that every class shares. Rules are kept only where they win on
   held-out words.

Neither stage is about size or speed. Generated source, compiled size and call time may
all grow; report them, don't optimize them. Two things still bind:

- **Stage 1 gives the same answers as today.** Stage 2 changes answers only where held-out
  words get better.
- **Every crate compiles on CI** (GitHub's `ubuntu-latest`: 4 cores, 16 GB RAM) within
  the job's time limit.

You own the PR fully:

- In flexion: the trainer, runtime, emitter, configs, xtask, `tools/ceiling`, tests,
  docs and APIs.
- `sound-law`'s code, wherever it ends up.

The design below is a starting point, not a spec. No prototypes or spikes: what lands is
finished, regenerated work. The project is pre-release: delete the old format outright,
with no compatibility layer and no migration notes.

Every trainer, runtime or emitter change must be **generic**: no language-specific code
there. Language knowledge stays in `configs/`. Phonology rules are **learned from the
data**, not written into configs. At most, configs declare segment inventories (which
letters are vowels or consonants; vowels are already declared). Prefer rules that emerge
from the data over special cases.

---

## 1. What exists

Read these first.

In flexion:

- `README.md`: the architecture, "How it learns", "How close to the ceiling", the
  results table and the commands.
- `prompts/`: all earlier prompts, especially `mdl-and-ceilings.md`. Their working rules
  still apply unless this prompt says otherwise.
- `tools/ceiling/RESULTS.md`: where the models fall short and why. Sections 3 and 4 name
  what the tree can't generalize:
  - **Greek perfect reduplication:** the tree got 8.1% of perfect cells right, a
    character transformer 38.1%.
  - **The augment after a preverb:** `ἐξετελοῦμεν`, where the tree writes `ἐἐκτελοῦμεν`.
  - **Vowel length** inside stems.
- `crates/flexion/src/format.rs` (format v7) and `crates/flexion/README.md`.
- `crates/flexion/src/lib.rs`: the loader. It derives node offsets and builds the
  lexicon's hash index at load.
- `crates/flexion/src/pos.rs`: the engine. It covers:
  - the tree walk and the scan for fitting classes;
  - the restoration and start-restoration models (`full_lemmas`, `front_readings`);
  - citations, markers, joins, part models and the lexicon.
- `crates/flexion/src/write.rs`: the writer, including the per-section byte counts
  (`write_sized`) that the trainer's size price uses.
- `crates/flexion-train/src/edit.rs`: how a (lemma, form) pair becomes an edit, and how
  ambiguous pairs choose the reading most pairs share.
- `crates/flexion-train/src/emit.rs`: writes each language crate (`lib.rs`, README,
  `model.bin`).
- `crates/flexion-train/src/pipeline.rs`: the tuning grid and the adoption rules for
  joins, the start model and part models. **It scores every candidate by writing a blob
  and loading it with `Model::from_bytes`.** `xtask probe`, `configs/tests/end_to_end.rs`
  and `tools/ceiling` load blobs the same way.
- Each `languages/<lang>/README.md`: the current numbers, which are the "before".

In `gold-silver-copper/ruthenian`, `crates/sound-law`:

- `README.md` and `src/lib.rs`: the pattern language and its limits.
  - It has classes, captures, correspondence maps, anchors, and ordered rules whose
    matches are applied simultaneously.
  - It gives one deterministic result: no probabilities, no stress model.
- `src/engine.rs`, `src/compile.rs`: a backtracking matcher. A `RuleSet` is `const` data
  that `compile()` validates at runtime and turns into a `Program`.
- `tests/`, `UPSTREAM_CONFORMANCE.md`, `tests/fixtures/PROVENANCE.md`: its conformance
  suites. Brassica fixtures are BSD-3-Clause; keep their license and provenance.
- Nothing else in `ruthenian` uses it.

---

## 2. Step 0: move `sound-law` into flexion

The generated crates must build from the flexion repo alone, and `sound-law` is
developed with flexion from now on.

- **Make it `crates/sound-law` in flexion's workspace, with its git history.**
  - Split its history out of `ruthenian`, for example
    `git subtree split --prefix=crates/sound-law`, or `git filter-repo` on a fresh clone.
  - Merge that history into flexion under `crates/sound-law`, so `git log` there shows
    its past commits.
  - If that can't be done cleanly, copy it instead, and name the source commit in the
    commit message.
- **Bring all of it:**
  - its README and its `*.md` docs (`CORPUS_TOOLS.md`, `DIFFERENTIAL.md`,
    `UPSTREAM_CONFORMANCE.md`);
  - its tests, fixtures and their licenses and provenance;
  - its `corpus_inventory` binary.
- **Make it fit flexion's workspace.** Replace `ruthenian`'s inherited
  `edition.workspace`, license, authors and repository fields with flexion's. Its tests
  run in `cargo test --workspace`, and `cargo fmt` and `cargo clippy -D warnings` pass on
  it.
- **The no-panic lints** (`expect` and indexing denied) can be switched on for it at the
  start of stage 2 (section 4), together with the fixes they need. Until then it keeps
  its own lint settings, but it must land under the workspace's lints in this PR.
- **Delete it from `ruthenian`.** Open one PR in `gold-silver-copper/ruthenian` that:
  - removes `crates/sound-law` and its workspace entry;
  - updates anything that names it (`SOUND_LAW_PORTING_PROMPT.md`, docs, `Cargo.lock`)
    to point to flexion.

  Nothing in `ruthenian` uses the crate; confirm that first. Merge it with
  `gh pr merge --merge` once its CI is green.

Commit step 0 on its own, before stage 1.

---

## 3. Stage 1: models as Rust code

### Generated code

For each part of speech, and for each of its helper models (restoration,
start-restoration, part models), the emitter writes:

- **The classes**, as an `enum` with a doc comment naming one example word and its forms
  (`/// like книга: книги, книге, …`). It also writes what the engine needs per class:
  - the suffix and start it strips;
  - its inherent features;
  - its edit per cell, as a `match (class, cell)`.
- **The edits**, as Rust values, e.g.
  `Edit::suffix("y", "ies")`, or
  `Edit::suffix("а", "и").stress(Stress::FirstVowelOfEnding)`.
- **The tree**, as a function from the key (the folded word read from its last letter,
  ending in the boundary symbol) to the distributions along its path:
  - slice patterns, e.g. `['y', rest @ ..] => { out.push(&[(C1, 0.8), (C0, 0.2)]); match rest { … } }`;
  - a node whose label has several letters matches them together;
  - a node with a syllable-count or first-letter token becomes a `match` on that value
    at its depth.
- **The lexicon, the join prefixes and the part-model routing**, as `match`es.
- **The spelling** (folds, marks, vowels, diphthongs), as a `static`.

Example, from the tiny model in `crates/flexion/src/tests.rs`:

```rust
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum NounClass {
    /// like `cat`: cats
    C0,
    /// like `city`: cities
    C1,
    /// like `ox`: oxen
    C2,
}

pub(crate) fn noun_tree(key: &[char], out: &mut Vec<&'static [(NounClass, f32)]>) {
    use NounClass::*;
    out.push(&[(C0, 0.9), (C1, 0.1)]);
    match key {
        ['y', ..] => out.push(&[(C1, 0.8), (C0, 0.2)]),
        ['x', 'o', BOUNDARY, ..] => out.push(&[(C2, 1.0)]),
        _ => {}
    }
}
```

**One giant function will not compile for Russian.** Russian's noun tree has hundreds of
thousands of nodes, and Ancient Greek's verbs have hundreds of thousands of distribution
entries. rustc's time and memory grow faster than linearly with a function's size. Split
the tree into many functions, for example one per subtree up to some number of nodes,
calling each other. Split it across modules and files too, so no single file or function
is huge. Measure where the limits are; don't guess.

### The runtime

`flexion` becomes the engine over a trait, which the generated code implements. The trait
might look like this:

```rust
pub trait Pos {
    type Class: Class;
    fn tree(&self, key: &[char], out: &mut Vec<&'static [(Self::Class, f32)]>);
    fn restore(&self) -> Option<&dyn Restore>;
    fn restore_front(&self) -> Option<&dyn Restore>;
    fn lexicon(&self, folded: &str) -> Option<&'static str>;
    …
}
```

Choose your own shape: generics, `dyn` or a mix. Do what keeps the engine one piece of
code and the generated code plain.

- **Deleted:** `format.rs`, `write.rs`, `Model::from_bytes` and the checks for damaged
  files.
- **Kept:** the engine's behavior (`pos.rs`), every query and every confidence.
- **Unchanged:** each language crate's public API (`italian::noun("uomo",
  Number::Plural)`), and the README examples still compile and pass.

### The trainer

The trainer can't compile Rust for each of the thousands of models it scores while
tuning. It needs an **in-memory model**, its own tables of nodes, classes and edits,
that implements the same trait the generated code does. The engine scores that model
directly, with no blob round trip. Then:

- tuning, the reports, `xtask probe`, the end-to-end test and `tools/ceiling` all use the
  in-memory model;
- only the final, shipped model is emitted as code;
- the size price (`SIZE_PRICE`, `ln(log-loss) + 0.05·log2(bytes)`) still needs a size. Keep
  a deterministic size measure computed from the tables. The simplest is the byte count
  v7 would have written. Then **every tuning choice stays exactly what it is today**. If
  you change the measure, every choice that moves needs a reason in the report;
- each report's "Where the bytes are" table becomes whatever size is meaningful now
  (generated source, compiled size, the size measure). Your call; say what it measures.

### The audit

The generated code and the trainer's in-memory model are two implementations of one
model, so they must agree. Build an audit that runs both on every language, for:

- the held-out words;
- a sample of attested words;
- made-up words;
- citations and markers;
- every cell.

It compares the candidates, their order and their confidences (equal up to f32
rounding). The audit is the gate for both stages.

**At the end of stage 1:**

- run the audit on all ten languages;
- check that every held-out, attested and calibration number in the regenerated reports
  equals `main`'s, and explain any difference;
- commit stage 1 as its own commit, with all ten crates regenerated.

That commit is the baseline stage 2 is measured against.

This is an audit, not a golden test: it compares two live implementations, not stored
outputs. A small, fast version of it belongs in the test suite, e.g. the end-to-end
fixture model checked both ways.

---

## 4. Stage 2: learned sound laws

### Why

An edit today is a string swap, learned on its own for every ending it occurs with. So
one phonological process becomes many unrelated edits. Before, the Polish locative:

```rust
Edit::suffix("ka", "ce")    // ręka   → ręce
Edit::suffix("ga", "dze")   // noga   → nodze
Edit::suffix("ta", "cie")   // tata   → tacie
Edit::suffix("na", "nie")   // ściana → ścianie
```

Each is learned only from the training words ending that way. An unseen word whose
combination of ending and class no training word showed can't get it right. Operations
at the word's start (reduplication, the augment after a preverb) can't be generalized
at all by a tree that reads from the end: that is the gap `RESULTS.md` measured.

After, an edit is an ending that may carry a trigger the rules react to, and one rule
set serves every class and cell of the language:

```rust
// the ending: `ě` is a learned trigger, a softening e
Edit::suffix("a", "ě")

// emitted once per language
const PHONOLOGY: RuleSet<'static> = sound_laws! {
    classes { HARD = ["k", "g", "ch", "t", "d", "r", "n", "w"]; }
    maps { SOFTEN = ["k" => "c", "g" => "dz", "ch" => "sz", "t" => "ci",
                     "d" => "dzi", "r" => "rz", "n" => "ni", "w" => "wi"]; }
    rules {
        soften: "{c:HARD}" => "{c|SOFTEN}" / "" _ "ě";
        realize: "ě" => "e";
    }
};
```

Greek perfect reduplication is one capture rule on the perfect stem, e.g.
`"{c:C}" => "{c|DEASPIRATE}ε{c}" / "^" _ ""`, giving `λύω → λέλυκα` and `θύω → τέθυκα`.
The augment after a preverb works at the seam the join model already finds:
`ἐκ + ε → ἐξε`.

These rule texts are illustrations. What the trainer learns, how it writes the trigger
and the seam, and in what form are your call.

### What `sound-law` needs first

Step 0 (section 2) has already moved it here. Before stage 2 uses it:

- **Make it pass flexion's lints.** Non-test code must not panic. It has `expect`s in
  `src/engine.rs` (lines 173, 856, 861, 866 at the time of writing) and an index in
  `src/compile.rs:502`. Remove them, and put it under the workspace's
  `deny`/`forbid` lints.
- **Unicode.** `sound-law` matches Unicode scalars and declared multi-scalar segments.
  flexion works in graphemes with combining marks (Greek accents and breathings, Latin
  macrons, Russian stress). Decide how they meet: rules over folded text, segments
  declared as whole graphemes, or something better. Test it on Greek.
- **Compiling at runtime.** `RuleSet::compile()` returns a `Result`. Generated crates
  compile their rules once on first use. A rule set that fails to compile must be
  impossible to ship: the trainer compiles every rule set it emits, and a test checks
  each crate's.
- **Change `sound-law` itself where that's cleaner** than working around it: an API to
  build rule sets at runtime for the trainer, a way to emit them as `sound_laws!`, or
  `const` validation. Its own tests must keep passing.

### Learning the rules

The trainer learns each language's rules from its edits. Start from this, and change it
where the data says otherwise:

1. **Find regular differences.** Within one cell, find groups of edits that differ only
   by a consistent mapping of the stem's last segment or segments (`ka→ce`, `ga→dze`,
   `ta→cie`). Each such group suggests one ending (with a trigger) plus one mapping rule.
   The mapping is learned from the pairs, not declared.
2. **Share across cells.** A trigger and rule found in one cell is tried in every cell
   whose edits it explains: the dative and locative share Polish softening.
3. **Word-start operations.** Copying the first consonant (reduplication) and inserting
   after a seam (the augment after a join-model prefix) are rules anchored at `^` or at
   the seam, learned from the cells where they explain the edits.
4. **Order.** Rules apply in a learned order. Keep the order that explains the most
   training pairs.
5. **Re-learn the classes.** With the rules in place, each (lemma, form) pair is read as
   ending plus rules, so classes are counted over endings. The tree, tuning and the rest
   of the pipeline run unchanged on top.
6. **Keep a rule only if it earns its place**, the way joins and the start model do:
   - the size price must pick it;
   - it must win dev top-1 beyond one word-level standard error;
   - it must not make attested words worse.

   Rules cost bytes in the size measure. A language where no rule earns its place ships
   without a phonology block.

The trainer runs rules with the same `sound-law` engine the crates use, so the in-memory
model and the generated code still match and the audit still applies.

### Where to look for wins

Measure every language, and expect gains first here:

- **Ancient Greek verbs:** perfect and pluperfect reduplication, and the augment after a
  preverb (imperfect, aorist).
- **Polish nouns:** softening before `-e` in the dative and locative.
- **Russian:** consonant alternations in verbs (`писать → пишу`) and fleeting vowels.
- **German:** umlaut in plurals.
- **Latin:** verb stem changes.

Report each language's rules, how many training pairs each explains, and the held-out
change it brought. Where a rule was tried and didn't earn its place, report it too.

---

## 5. Done means

- **No binary format.** No `model.bin`, no `include_bytes!`/`include_str!` of model data,
  no `format.rs`/`write.rs`/`from_bytes`. All ten crates are regenerated with
  `cargo xtask build`, at the end of stage 1 and again at the end of stage 2.
- **Stage 1: same answers.** The audit passes on all ten languages, and the report
  numbers match `main`'s.
- **Stage 2: better or unchanged.**
  - Every language's held-out top-1 is at least the stage-1 baseline's within word-level
    noise, in every input mode.
  - Attested primary-form accuracy stays ≥99% per part of speech.
  - Calibration gets no worse beyond the noise.
  - The audit passes again on all ten languages.
  - Every number that moves has a cause in the report.
  - It's fine if few languages adopt rules, as long as you report what was tried.
- **`sound-law` moved and hardened:**
  - it lives in flexion's workspace as `crates/sound-law`, with its history (or a named
    source commit);
  - it has no panics in non-test code;
  - its conformance suites pass;
  - the `ruthenian` PR deleting its old copy is merged.
- **CI is green.** `cargo test --workspace`, `cargo fmt --all --check` and
  `cargo clippy --workspace --all-targets -- -D warnings` pass. Generated code is either
  formatted and clippy-clean, or excluded explicitly and narrowly (for example
  `#![allow(...)]` with a reason in the generated module, or `ignore` in `rustfmt.toml`).
  If CI's time or memory is the limit, fix the code generation: splitting, smaller
  functions. Don't drop languages from CI.
- **Determinism.** The same dumps and config give byte-identical generated source,
  including the learned rules and their order.
- **No panics** in non-test code, generated code included. The workspace's
  `deny`/`forbid` lints (no `unwrap`, `expect`, indexing; the runtime also denies
  arithmetic side effects) cover the generated code. Slice patterns and `.get()` are
  fine.
- **Measured, not optimized.** For every language, report:
  - generated source size;
  - clean compile time and peak compile memory (locally and on CI);
  - compiled `.rlib` size;
  - call time against `main`, old and new alternating in one run.
- **Tests:**
  - the tiny model emitted as code answers like the in-memory one;
  - the audit on the fixture;
  - a rule learned from a small hand-made set of pairs, e.g. a softening map recovered
    from `ka→ce`, `ga→dze`, `ta→cie`;
  - a reduplication rule recovered the same way;
  - no golden or snapshot tests.
- **Docs.**
  - The root README's architecture and "How it learns" describe code generation and the
    phonology step.
  - `crates/flexion/README.md` describes the trait and the generated code, with a real
    excerpt.
  - `crates/sound-law/README.md` says how flexion uses it.

---

## 6. Rules for how you work

The earlier prompts' rules apply:

- No `/tmp`; use `~/.cache/claude-tmp/`.
- Cap parallelism: `-j 8`, `--test-threads 4`, at most 8 training threads.
- Nothing over ~10 minutes, all cores or lots of RAM without asking.
- Kill everything you started when done.
- Never `pkill -f` a pattern from your own command line; use PIDs or `pgrep -x`.
- Keep a progress log at `~/code/notes/projects/flexion/rust-codegen.md`.

Specific to this PR:

- **Compiling generated crates can eat memory.** This machine's terminal has been
  OOM-killed before. Compile big generated crates under a hard memory cap:
  `systemd-run --user --scope -p MemoryMax=16G -p MemorySwapMax=0 cargo build -p russian -j 4`.
  16 GB is CI's limit anyway. A blow-up should kill the build, not the machine.
- **Order the work so the risk shows early.** In stage 1, get the emitter working for one
  small language (Italian), then Russian and Ancient Greek, before polishing anything. In
  stage 2, try rule learning on Polish nouns and Greek verbs first. This is not a spike:
  it's the real code, done in the order that finds problems first.
- **Decide on tuned builds, not default-parameter probes.**
- The Russian build takes about 15 minutes; plan around it.
- Swapping the build tool under a running queue: `mv` a new copy into place; never
  overwrite the running file.

---

## 7. Git

- **flexion:** work on a branch, with step 0 (the move), stage 1 and stage 2 as separate
  commits, in that order (more commits inside each are fine). Open one PR, and merge it with `gh pr merge --merge` once
  CI is green. Then check out the latest `main`.
- **ruthenian:** one PR that deletes `crates/sound-law` and its workspace entry; merge it
  the same way.
- Don't publish to crates.io, create tags or releases.

---

## 8. Final report

- **Both PR links** (flexion, and the `ruthenian` deletion).
- **The move:** how `sound-law`'s history came over, and what changed to fit flexion's
  workspace.
- **What the generated code looks like.** Real excerpts from one language: a class enum,
  a piece of the tree, a lexicon match, and the learned `sound_laws!` block. Also say how
  the code is split into functions, modules and files.
- **Runtime API before/after**, as code: how a crate loads and queries its model before
  and after. Also the trainer's, xtask's and `sound-law`'s API changes.
- **The audit**, for both stages: what it compared, on how many words and cells per
  language, and the result. Stage 1's report numbers against `main`.
- **Stage 2 per language:**
  - the rules learned, each with an example, how many training pairs it explains, and
    whether it was adopted;
  - top-1 per part of speech and input mode against the stage-1 baseline, with
    word-level noise;
  - attested accuracy and calibration.

  Name the rules tried and rejected, and why.
- **Per language:** generated source size, compile time and memory (local and CI),
  `.rlib` size, call time against `main`.
- **What didn't work**, in either stage, with numbers.
- **How you verified it**, with copy-pasteable commands starting with `cd ~/code/flexion`:
  regenerate a crate, compile it under the memory cap, run the audit, show a language's
  learned rules.
