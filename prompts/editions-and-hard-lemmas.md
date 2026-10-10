# Prompt: learn from more Wiktionary editions, and close the hardest gaps

Two things in one PR for `~/code/flexion` (GitHub `gold-silver-copper/flexion`, branch
`main`):

1. **More data.** Today every flexion language learns from the English Wiktionary only.
   Let a language also learn from other Wiktionary editions (the Russian Wiktionary for
   Russian, the German one for German, the French one for French and for Latin, and
   so on), wherever doing so measurably improves it.
2. **The weakest numbers.** Raise the scores in the three places flexion is weakest:
   - Russian from the bare lemma (stress);
   - Latin from the bare lemma (vowel length);
   - Ancient Greek verbs and Ancient Greek from the headword.

They belong together. Part 1 is the cheapest lever for part 2: the Russian Wiktionary
writes stress on every form of hundreds of thousands of words. Measure part 1 first,
then work on whatever gap remains.

This is one PR. You own it fully: trainer, runtime, binary format, configs, hooks,
extras, tests, docs and APIs are yours to change if it makes the project better. The
notes below are starting points, not a spec. Follow the data and the measurements.
No prototypes or spikes: the PR delivers finished, regenerated crates.

Every trainer or runtime change must be **generic**: it must make sense for any
language. Language-specific knowledge goes in configs. A change that only one language
could ever use means the design is wrong. Rethink it.

---

## 1. What exists

Read these first:

- `README.md`: the architecture, "How it learns", the results table, and "What the
  next language would need".
- The earlier prompts in `prompts/` (`inflection-compiler.md`, `latin-polish.md`,
  `german-sanskrit-greek.md`, `spanish-french-italian.md`). Their rules (tests,
  resources, no panics, reports, ship bars) still apply unless this prompt says
  otherwise.
- `crates/flexion/README.md` and `crates/flexion/src/format.rs`: the model and the
  binary format.
- `crates/flexion-train/src/spec.rs`: `Language`, `Hooks`, `TagPolicy`, spelling and
  stress marks.
- `crates/flexion-train/src/kaikki.rs`: the dump reader.
- Each language's `languages/<lang>/README.md`: its current report. These are the
  "before" numbers.

The current weak spots (held-out words, top-1, from the root README):

| language | input | nouns | adjectives | verbs | letters right (no stress/length) |
|---|---|---|---|---|---|
| Russian | bare lemma | 81.5% | 90.4% | 88.1% | 94.9%, 99.8%, 96.8% |
| Russian | principal parts | 94.6% (96.9% with markers) | | 98.6% | |
| Russian | stressed lemma | 92.6% | 99.8% | | |
| Latin | bare lemma | 59.1% | 71.8% | 66.6% | 93.3%, 99.4%, 90.3% |
| Ancient Greek | principal parts | 96.0% | 98.5% | **73.8%** | |
| Ancient Greek | lemma in full | 81.4% | 86.9% | 53.6% | |
| Ancient Greek | headword | 42.6% | 42.1% | 35.8% | 92.7%, 94.5%, 63.6% |

In Russian and Latin the letters are almost always right. What fails is where the
stress falls (Russian) or which vowels are long (Latin). Ancient Greek verbs fail even
given the principal parts. Their classes have about 600 cells, and an unseen verb
rarely fits one at a deep node; verbs are also the slowest calls in the repo (1.3 µs).
The Ancient Greek model is 21.8 MB, 3.8 MB gzipped.

### A reference: morphe

`~/code/morphe` (GitHub `gold-silver-copper/morphe`) is a separate, untested
experiment built on the same idea. It already reads other editions (`extra_dumps`
and the `prepare_extra` hook in `crates/morphe-train/src/spec.rs`, used by the
configs in `configs/src/langs/`). Its notes are at
`~/code/notes/projects/morphe/progress.md`. Read it for the traps each edition
has. Don't trust it as correct: it was never measured. Don't modify that repo.

## 2. Part 1: other Wiktionary editions

Kaikki extracts every Wiktionary edition, not just the English one
(`https://kaikki.org/` lists them, e.g. `kaikki.org/ruwiktionary/`). Copies of
eleven editions' own-language dumps (dated 2026-10-10) are already in
`~/.cache/kaikki/<edition>/`:

- `ru/Русский.jsonl`
- `de/Deutsch.jsonl`
- `fr/Français.jsonl`
- `es/Español.jsonl`
- `it/Italiano.jsonl`
- `pl/językpolski.jsonl`
- also `cs`, `el`, `nl`, `pt`, `tr`

The German and French ones are over 3 GB. Use them, or download fresh ones into
`~/.cache/flexion/dumps/`. Never use `/tmp`. Latin, Ancient Greek and Sanskrit have no
own-language edition there. Other editions (French, Italian, German, Greek, Russian…)
cover them, so find which ones have real tables for them and download those.

Survey each edition before writing anything. Find out:

- how its tags look. Kaikki normalizes some editions to English tags; others keep
  `raw_tags` in their own language;
- which head templates and inflection classes it records. The Russian edition writes
  Zaliznyak's index (`сущ ru f a 1a`), which *is* the stress pattern;
- what bookkeeping, dialect or archaic forms leak in;
- how its entries split homographs and senses compared with the English edition;
- how it writes stress, vowel length, `ё` and accents.

Then build a generic way for a language's config to name extra editions and repair
their entries. Design it yourself; the requirements are:

- **Measured adoption.** An edition is used for a language only if it improves that
  language's held-out numbers beyond the noise, without worse attested accuracy or
  calibration. Report every edition you tried, including the rejected ones and why.
- **No leaks.** Choose the held-out and tuning lemmas over the union of the sources. A
  held-out lemma must be absent from *every* source, matched loosely enough to catch the
  same word written with or without stress marks, `ё`, macrons or accents.
- **Comparable numbers.** Report held-out accuracy two ways:
  - on the English-edition lemmas the current README tests on, so before and after
    compare directly;
  - on the full union.
- **Disagreements are data.** Where two editions give different forms for the same
  lemma and cell, measure how often by cell and part of speech, and decide what wins.
  It could be one edition, both as alternatives, or the attested majority. Say why.
  High disagreement in a cell usually means a tag mismatch or a different standard,
  not a fact.
- **Attribution.** Every generated crate's `DATA-LICENSE.md` and README name each
  edition it learned from (CC BY-SA 4.0).
- **Determinism.** The same dumps and config give a byte-identical model.

Try it on every language where an edition could plausibly help:

- Russian, German, French, Spanish, Polish and Italian, from their own editions;
- Latin, Ancient Greek and Sanskrit, from whichever editions cover them.

English already learns from the English edition.

## 3. Part 2: the weak spots

After part 1 has landed, look at the largest groups of mistakes (`cargo xtask probe <lang>
--misses 1`) and fix their causes. Some starting points, none of them required:

- **Russian stress.** More stressed data may be enough. If not:
  - the Zaliznyak index could feed a learned class or a citation marker;
  - stress patterns could be learned as their own layer over the letters.
- **Latin vowel length.** The restoration model reads `rex` as `rēx`. Editions that
  write macrons on headwords give it more to learn from. Some lengths can't be known
  from a typed lemma at all. There the bar is an honest, low confidence, not a
  confident guess.
- **Ancient Greek verbs.**
  - Find out why the given principal parts don't fix them. Is it the part models'
    coverage, augment and reduplication, dialect forms in the tables, or classes too
    big to generalize?
  - The headword case (accents, no vowel length) is worth more than its current 36–43%.
- **Ancient Greek size and speed.** Improve both if the fix falls out of the work above.
  Neither may get worse.

**Aims** (held-out top-1; aims, not bars): halve the gap between the current number and
the letters-only number in each row that has one. That means, from the bare lemma:

| | nouns | adjectives | verbs |
|---|---|---|---|
| Russian | ≥88% | ≥95% | ≥92% |
| Latin | ≥76% | ≥85% | ≥78% |
| Ancient Greek, from the headword | ≥67% | ≥68% | ≥49% |

For Ancient Greek verbs given principal parts, aim for ≥85%. If an aim is missed,
explain which information the misses need and whether any data could supply it.

## 4. Done means

- Every language that adopted an edition is regenerated with `cargo xtask build`, with
  its full report, now naming its sources.
- **No language gets worse.** Rebuild all ten. Compare every held-out, attested,
  calibration, size and speed number with the current README. Every number that moves,
  even by a few hundredths of a point, needs a cause in the report.
- Every existing bar still holds:
  - attested primary-form accuracy ≥99% per part of speech;
  - reasonable calibration (ECE);
  - each crate packages under crates.io's 10 MB limit;
  - sub-microsecond calls everywhere except Ancient Greek, which must not get slower.
- Tests:
  - an end-to-end test on a small committed fixture of real lines from a second
    edition, following the pattern in `configs/tests/`. It should show at least that
    the extra edition's words get learned, and that a held-out word appears in no
    source;
  - a few meaningful tests for any new trainer logic. No golden or snapshot tests.
- Docs:
  - the root README's results table, "How it learns" and "What the next language would
    need" are updated;
  - `crates/flexion/README.md` and `format.rs` describe any format change.
- `cargo fmt`, `cargo clippy --workspace --all-targets -- -D warnings` and the
  workspace tests pass.

## 5. Rules for how you work

All the rules of the earlier prompts apply:

- No `/tmp`.
- Cap parallelism (`-j 8`, `--test-threads 4`).
- Stream dumps; never load one whole. Some editions are 3 GB.
- Nothing over ~10 minutes, all cores or lots of RAM without asking.
- No panics in non-test code.
- Few meaningful tests, no golden files.
- Keep a progress log at `~/code/notes/projects/flexion/editions-and-hard-lemmas.md`.
- Kill everything you started when done.

Lessons from earlier runs:

- **Shared machine.** Benchmark with the fastest-of-several method `fastest_ns` uses,
  compare old and new code in one interleaved run, and don't trust a single timing.
- **Wait loops.** A wait loop that matches processes by name must not match its own
  command line. Write PIDs to a file instead.
- **Pruning must match the runtime.** The trainer's copy of the runtime's choice (in
  `learn.rs`) decides which words keep their own node. Mirror any runtime change there,
  or attested accuracy drops quietly.
- **Rebuilding mid-run.** Configs are compiled into the build tool. Rebuild it before a
  queue of language builds starts, not during one.
- **Simple beats clever.** In the Spanish run, a stress-mark model that looked right in
  theory scored worse than treating accents as letters, and it was reverted. Measure
  each idea against the simplest alternative, and delete what doesn't earn its place.

## 6. Git

- Work on a branch, open one PR, and merge it with `gh pr merge --merge` once CI is
  green. Then check out the latest `main`.
- Don't publish to crates.io, create tags or releases.

## 7. Final report

- PR link.
- Per language: the editions tried, which were adopted, and why. Entries added, and the
  disagreement rates between editions.
- Per language, before/after tables of held-out top-1 per part of speech and input
  (bare lemma, citation, markers, stressed or full lemma), on the old test lemmas and
  on the union. Also attested accuracy, calibration, size and speed.
- Whether each aim was met. For a miss: what information the remaining errors need.
- Every trainer, runtime or format change: what it does, why it is generic, which
  languages use it, and before/after code for any API change.
- The data traps found in each edition and how the configs handle them.
- The biggest remaining error groups for Russian, Latin and Ancient Greek.
- How you verified it, with copy-pasteable commands starting with `cd ~/code/flexion`
  to inflect a few words, rebuild a crate, and rerun its evaluation.
