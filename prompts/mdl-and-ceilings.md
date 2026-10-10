# Prompt: smaller, better-founded models (ceilings, MDL, factored classes)

One PR for `~/code/flexion` (GitHub `gold-silver-copper/flexion`, branch `main`). The ten
language crates are accurate but bigger than they need to be. Russian's model is 21.4 MB,
up from 7.0 MB once it learned from the Russian Wiktionary as well. Much of that weight
looks like noise. The structure of every model is decided by several separate knobs, and
none of them charges for size.

This PR does three things:

1. Measures how much better the models *could* get.
2. Replaces the ad-hoc structure choices with one principled objective that pays for every
   byte.
3. Tries the two modeling changes most likely to lower error on unseen words.

You own the PR fully: trainer, runtime, binary format, configs, tests, docs and APIs are
yours to change if it makes the project better. The notes below are starting points, not a
spec. Follow the measurements. No prototypes or spikes: what lands is finished, measured,
regenerated work. An experiment that doesn't earn its place is reported and left out.

Every trainer or runtime change must be **generic**: it must make sense for any language.
Language-specific knowledge goes in configs.

---

## 1. What exists

Read these first:

- `README.md`: architecture, "How it learns", the editions section, the results table.
- `prompts/` (all earlier prompts, especially `editions-and-hard-lemmas.md`). Their rules
  still apply unless this prompt says otherwise.
- `crates/flexion/src/format.rs` (binary format v6) and `crates/flexion/README.md`.
- `crates/flexion-train/src/learn.rs` (edits → classes → suffix tree → pruning; the
  trainer's copy of the runtime's choice), `pipeline.rs` (tuning grid, one-standard-error
  rule, adoption of joins and part models, the report), `lib.rs` (part models, restoration
  model, lexicon).
- Each `languages/<lang>/README.md`: the current numbers, which are the "before".
- `~/code/notes/projects/flexion/editions-and-hard-lemmas.md`: the last run's log.

How a model is shaped today:

- **Classes.** A word's inflection class is its whole paradigm: one edit per cell, plus
  inherent features. Russian nouns have about 4,000 classes; Ancient Greek verbs have
  625-cell classes, most seen once.
- **The tree.** A suffix tree over lemmas stores a class distribution per node.
- **The knobs that decide structure:**
  - a likelihood-ratio pruning threshold, minimum suffix support and smoothing α, tuned
    on the dev tenth by top-1 accuracy with the one-standard-error rule over a small grid;
  - "memorize a word if dropping its node changes an attested form";
  - "adopt a part model or the join model if it beats dev beyond the noise".

  Nothing weighs size.

Where Russian's 21.4 MB goes. Measured on the 22 MB build before edit records shrank to 32
bytes; the shares hold:

| part | ~MB | note |
|---|---|---|
| class distributions (8 bytes per entry) | 8.4 | nouns 411k entries (223k before the edition); each node keeps up to 256 classes |
| participle lexicon | 3.9 | 58k entries, each a key string plus a full written string |
| tree nodes | 3.8 | 10.8k memorized nouns (1.9k before the edition) |
| verb edits + join model | 2.9 | join classes brought verb edits from 9.5k to 46k |
| noun part model (from the genitive singular) | 2.4 | a whole second tree, used for **2 cells** |

The other sizes: Ancient Greek 20.1 MB (3.9 MB gzipped), Latin 7.4, German 5.9, Polish 2.1,
the rest under 1 MB.

## 2. Measure the ceiling

Before optimizing accuracy, find out how much there is to gain. Report:

- **Learning curves.** Per language and part of speech, held-out top-1 and log-loss when
  training on 10%, 25%, 50% and 100% of the training words. The shape says whether more
  data still helps or the curve has flattened. Russian has two editions' worth of data to
  test this with.
- **A noise floor.** Where two Wiktionary editions disagree on a shared word (2–3% of
  Russian cells), at least one is wrong or both are acceptable. Use that to estimate how
  much error the labels themselves carry.
- **The irreducible part.** For the gaps the earlier prompts named, estimate how much of
  each is predictable from spelling at all: Russian lemma stress and animacy, Latin and
  Greek vowel length. For example, measure the conditional entropy of the stress pattern
  given the word's ending at the depth the data supports, or see where accuracy saturates
  as context grows.
- **A neural baseline.** Train a small character-level sequence model (the standard kind
  in the SIGMORPHON inflection shared tasks) on exactly our split, for at least Russian
  nouns and verbs, Latin nouns and Ancient Greek verbs. Score it on the same held-out
  words. It is a measuring stick, not a shipped model and not a dependency of any crate.
  - Put its code where it fits: a `tools/` directory like `tools/english-baseline`, or
    the notes if it doesn't belong in the repo.
  - Keep it reproducible.
  - It runs on CPU, at most 8 threads. Each training run must stay within ~10 minutes,
    so size the model to fit.

  If the baseline is far ahead of the tree on unseen words, say where (which cells, which
  word types), because that is information the tree is leaving out.

These numbers decide how much of section 4 is worth doing. Put them in the root README
("How close to the ceiling") and the final report.

## 3. One objective that pays for structure

Replace the structure knobs with a **minimum description length** objective. A tree node,
a distribution entry, a memorized word, a join prefix, a part model or a lexicon entry is
kept only if it saves more bits describing the training data than it costs to store.
Bayesian suffix-tree methods do this in a principled way (context-tree weighting,
hierarchical Pitman–Yor or Dirichlet smoothing over the tree); read up and choose.

- **Tune on held-out log-loss as well as top-1.** Log-loss rewards honest confidences. If
  confidences still need it after that, add a small learned calibration map per part of
  speech, fitted on the dev tenth.
- **One size knob.** Expose a single knob for the size/accuracy trade-off (bits per stored
  parameter, or a size budget). Choose its default from a size-vs-accuracy curve you
  report for every language.
- **Count every part.** Part models, the join model and the lexicon enter the same
  accounting. A 2.4 MB part model that builds 2 cells must pay for itself or go.
- **Edition disagreements stay soft.** A word whose only exception comes from two
  editions disagreeing should not become a memorized word. Decide how disagreements
  enter, for example as soft observations, or with the editions weighted by a learned
  per-source reliability (Dawid–Skene-style).

Format work that loses nothing, whatever the objective decides:

- **Distribution entries from 8 to 4 bytes or fewer:** a narrow class id plus a quantized
  probability. Measure that quantization changes no top-1 answer, and calibration only
  within noise. Fall back where a part of speech has too many classes.
- **The lexicon as "key plus where the marks go"** (`а́хнувший` = `ахнувший` + acute on
  vowel 1; Latin macrons and Greek length marks likewise), instead of a second full string.
- **A size breakdown in every report**, like the table above, so the next person can see
  where the bytes are.

## 4. Two modeling changes worth trying

Do these after sections 2 and 3, and only as far as section 2 says there is room. Each is
adopted only if it wins on held-out top-1 and log-loss, without worse calibration or size.

- **Factored classes.** A Russian noun is described by a few independent pieces:
  declension type, stress pattern, stem alternation (Zaliznyak's `1a`, `3*b`). Learn such
  factors from the data: find groups of cells whose edits vary together, and predict each
  factor from the word's ending separately, conditioned on the others where they interact.
  Fewer, denser parameters should mean a smaller model and better guesses for rare words.
  Stress becomes its own prediction. Greek's 625-cell verb classes are the strongest case.
- **Evidence beyond the ending.** The tree reads a word only from its end, but stress
  (Russian, Greek) and vowel length (Latin) follow the root. Words that share a root share
  much of their behavior. The participle lexicon was one instance of this, and gave Russian
  adjectives +6 points. Generalize it:
  - find an unseen word's attested relatives (the same stem with other prefixes or
    suffixes, across parts of speech);
  - use their stress or length as evidence, for example as a second context read from the
    stem's start and combined with the suffix tree.

  This is the change most likely to move Russian nouns and verbs, and Latin and Greek from
  the bare lemma.

## 5. Done means

- **Sizes.** Every crate is smaller than now, or explained. Aim for Russian under 10 MB
  raw and every other language at least 30% smaller.
- **Accuracy.** No language's held-out top-1, attested accuracy or calibration gets worse
  beyond the noise. Use word-level noise: bootstrap over lemmas, not cells, since a word's
  cells move together. Every number that moves needs a cause in the report.
- **Bars.** Every existing bar still holds:
  - attested primary-form accuracy ≥99% per part of speech;
  - reasonable calibration;
  - each crate packages under 10 MB;
  - sub-microsecond calls, with Ancient Greek verbs no slower than now (1.4–1.6 µs).
  Measure speed with old and new code alternating in one run.
- **Determinism.** The same dumps and config give a byte-identical model.
- **Ceiling numbers** (section 2) are in the root README and the final report.
- **Rebuilt.** All ten crates are regenerated with `cargo xtask build`, with the size
  breakdown in each report.
- **Tests:**
  - a few meaningful tests for the new objective and format (e.g. a node that doesn't pay
    for itself is dropped; quantized entries round-trip);
  - no golden or snapshot tests.
- **Checks.** `cargo fmt`, `cargo clippy --workspace --all-targets -- -D warnings`, the
  workspace tests **and every generated crate's doctests** (`cargo test -p <lang> --doc`)
  pass.
- **Docs.** The root README's "How it learns" is rewritten to the new design, and
  `crates/flexion/README.md` and `format.rs` describe the new format.

## 6. Rules for how you work

All earlier rules apply:

- No `/tmp`.
- Cap parallelism (`-j 8`, `--test-threads 4`, at most 8 training threads).
- Stream dumps.
- Nothing over ~10 minutes, all cores or lots of RAM without asking. The Russian build
  already takes about 13 minutes, so plan around it (probes, subsets, or ask).
- No panics in non-test code.
- Keep a progress log at `~/code/notes/projects/flexion/mdl-and-ceilings.md`.
- Kill everything you started when done.

Lessons from the last runs:

- **Decide on tuned builds, not default-parameter probes.** Two edition adoptions that
  looked good in probes were worse in the tuned build.
- **Shared machine.** Benchmark old and new code in one interleaved run, alternating which
  goes first.
- **Matching processes.** Never `pkill -f` a pattern that appears in your own shell's
  command line (it killed the shell last time). Kill by PID or `pgrep -x <name>`.
- **Swapping the build tool.** Configs are compiled into the build tool. To swap it under a
  running queue, `mv` a new copy into place; never overwrite the running file.
- **Pruning must match the runtime.** The trainer's copy of the runtime's choice
  (`learn.rs`) must change with the runtime.

## 7. Git

- Work on a branch, open one PR, and merge it with `gh pr merge --merge` once CI is green.
  Then check out the latest `main`.
- Don't publish to crates.io, create tags or releases.

## 8. Final report

- PR link.
- **Ceiling.** Learning curves, noise floor, irreducible-error estimates and the neural
  baseline's scores next to the tree's, and what they say about where accuracy can still
  come from.
- **The objective.** What replaced which knobs, with the size-vs-accuracy curve per
  language and the chosen default.
- **Per language, before/after:** size (raw, gzipped, and the breakdown), held-out top-1
  and log-loss per part of speech and input mode, attested accuracy, calibration, speed.
- **Section 4.** What was tried, what was adopted, and the numbers either way.
- **Every format, trainer or runtime change.** Why it is generic, with before/after code
  for any API change.
- **How you verified it**, with copy-pasteable commands starting with `cd ~/code/flexion`
  to rebuild a crate, rerun its evaluation, and rerun the ceiling measurements.
