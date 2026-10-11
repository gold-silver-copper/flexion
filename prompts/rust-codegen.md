# Prompt: models as Rust code (no binary format)

One PR for `~/code/flexion` (GitHub `gold-silver-copper/flexion`, branch `main`). Start
from the latest `main`. It must already contain the `mdl` PR (format v7, the
start-restoration model, the size price); if `git log main` doesn't show it, stop and ask.

Today every language crate ships its model as `src/model.bin`. A generated `lib.rs`
loads it with `include_bytes!` and `flexion::Model::from_bytes`, and the `flexion`
runtime decodes the bytes. **This PR removes the binary format completely.** The
trainer writes each model as Rust source code:

- the suffix tree becomes `match` statements over the word's letters;
- inflection classes become an `enum`;
- the lexicon and the join prefixes become `match`es on strings.

Nothing a crate ships is a byte blob, an `include_bytes!` or an `include_str!` of model
data. Anyone reading a language crate can follow how it decides a form.

**Size and speed are not goals for this PR.** Generated source, compiled size and
call time may all grow; report them, don't optimize them. Two things still bind:

- the models must give **the same answers** as today's;
- every crate must **compile on CI** (GitHub's `ubuntu-latest`: 4 cores, 16 GB RAM)
  within the job's time limit.

You own the PR fully. The trainer, runtime, emitter, configs, xtask, `tools/ceiling`,
tests, docs and APIs are yours to change if that makes the project better. The design
below is a starting point, not a spec. No prototypes or spikes: what lands is finished,
regenerated work. The project is pre-release: delete the old format outright, with no
compatibility layer and no migration notes.

Every trainer or runtime change must be **generic**: no language-specific code in the
trainer, runtime or emitter. Language knowledge stays in `configs/`.

---

## 1. What exists

Read these first:

- `README.md`: the architecture, "How it learns", the results table and the commands.
- `prompts/`, all earlier prompts, especially `mdl-and-ceilings.md`. Their working rules
  still apply unless this prompt says otherwise.
- `crates/flexion/src/format.rs` (format v7) and `crates/flexion/README.md`.
- `crates/flexion/src/lib.rs`: the loader. It derives node offsets and builds the
  lexicon's hash index at load.
- `crates/flexion/src/pos.rs`: the engine. It covers:
  - the tree walk and the scan for fitting classes;
  - the restoration and start-restoration models (`full_lemmas`, `front_readings`);
  - citations, markers, joins, part models and the lexicon.
- `crates/flexion/src/write.rs`: the writer, including the per-section byte counts
  (`write_sized`) that the trainer's size price uses.
- `crates/flexion-train/src/emit.rs`: writes each language crate (`lib.rs`, README,
  `model.bin`).
- `crates/flexion-train/src/pipeline.rs`: the tuning grid. **It scores every
  candidate by writing a blob and loading it with `Model::from_bytes`.** `xtask probe`,
  `configs/tests/end_to_end.rs` and `tools/ceiling` load blobs the same way.
- Each `languages/<lang>/README.md`: the current numbers, which are the "before".

---

## 2. The design to start from

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

### Same answers: the audit

The generated code and the trainer's in-memory model are two implementations of one
model, so they must agree. Build an audit that runs both on every language, for:

- the held-out words;
- a sample of attested words;
- made-up words;
- citations and markers;
- every cell.

It compares the candidates, their order and their confidences (equal up to f32
rounding). Run it on all ten languages and report the result. Also compare the new
crates' report numbers against `main`'s: every held-out, attested and calibration
number should be identical. Explain any difference.

This is an audit, not a golden test: it compares two live implementations, not stored
outputs. A small, fast version of it belongs in the test suite, e.g. the end-to-end
fixture model checked both ways.

---

## 3. Done means

- **No binary format.** No `model.bin`, no `include_bytes!`/`include_str!` of model data,
  no `format.rs`/`write.rs`/`from_bytes`. All ten crates are regenerated with
  `cargo xtask build`.
- **Same answers.** The audit passes on all ten languages, and the report numbers match
  `main`'s.
- **CI is green.** `cargo test --workspace`, `cargo fmt --all --check` and
  `cargo clippy --workspace --all-targets -- -D warnings` pass. Generated code is either
  formatted and clippy-clean, or excluded explicitly and narrowly (for example
  `#![allow(...)]` with a reason in the generated module, or `ignore` in `rustfmt.toml`).
  If CI's time or memory is the limit, fix the code generation: splitting, smaller
  functions. Don't drop languages from CI.
- **Determinism.** The same dumps and config give byte-identical generated source.
- **No panics** in non-test code, generated code included. The workspace's
  `deny`/`forbid` lints (no `unwrap`, `expect`, indexing; the runtime also denies
  arithmetic side effects) cover the generated code. Slice patterns and `.get()` are
  fine.
- **Measured, not optimized.** For every language, report:
  - generated source size;
  - clean compile time and peak compile memory (locally and on CI);
  - compiled `.rlib` size;
  - call time against `main`, old and new alternating in one run.
- **Tests.** A few meaningful ones (e.g. the tiny model emitted as code answers like the
  in-memory one; the audit on the fixture). No golden or snapshot tests.
- **Docs.** Rewrite the root README's architecture and "How it learns" parts that mention
  the blob. Rewrite `crates/flexion/README.md` around the trait and the generated code;
  show a generated excerpt there.

---

## 4. Rules for how you work

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
- **Order the work so the risk shows early.** Get the emitter working for one small
  language (Italian), then Russian and Ancient Greek, before polishing anything. This
  is not a spike: it's the real code, done in the order that finds problems first.
- The Russian build takes about 15 minutes; plan around it.
- Swapping the build tool under a running queue: `mv` a new copy into place; never
  overwrite the running file.

---

## 5. Git

- Work on a branch, open one PR, and merge it with `gh pr merge --merge` once CI is green.
  Then check out the latest `main`.
- Don't publish to crates.io, create tags or releases.

---

## 6. Final report

- PR link.
- **What the generated code looks like.** A real excerpt from one language (a class enum,
  a piece of the tree, a lexicon match), and how the code is split into functions,
  modules and files.
- **Runtime API before/after**, as code: how a crate loads and queries its model before
  and after. Also the trainer's and xtask's API changes.
- **The audit.** What it compared, on how many words and cells per language, and the
  result. Report numbers against `main`.
- **Per language:** generated source size, compile time and memory (local and CI),
  `.rlib` size, call time against `main`.
- **What didn't work** while getting the big languages to compile, with numbers.
- **How you verified it**, with copy-pasteable commands starting with `cd ~/code/flexion`:
  regenerate a crate, compile it under the memory cap, run the audit.
