# The fifth trainer panel: the decision (2026-10-18)

The fifth panel read `~/code/flexion` at `98699c9` (the merge of PR 21; the trainer is PR
21's final, 228f7a9) and decided what the next PR does. Its theme, set by the owner: **one
source of truth first** (every fact and meaning defined once, everything else reading
it), **then a dedicated performance pass**, then errors and bytes. The implementation
prompt is `~/code/flexion/prompts/trainer-panel-5-plan.md`.

The record:
- this decision;
- `trainer-panel-5-2026-10-18-dossier.md`: the twelve round-1 analyses, every bet, the
  probes;
- `trainer-panel-5-2026-10-18-minutes.md`: the meeting, its votes, objections and 89 checks;
- `trainer-panel-5-2026-10-18-ledger.md`: the ledger of past attempts, PR 21's added;
- `trainer-panel-5-2026-10-18-evidence.md`: the evidence pack's index and findings,
  including the two sources-of-truth inventories;
- `trainer-panel-5-2026-10-18-report.md`: the report to the owner.

Scratch, with every raw file: `~/.cache/claude-tmp/panel5/`.

## Terms

- **Dev tenth:** tenth 2, scored by `cargo xtask try`. **Tenth 9:** the confirmation
  tenth. **README tenth:** tenth 0.
- **Floor:** the SIGMORPHON non-neural baseline. **Paired comparison:** `xtask compare`,
  a bootstrap over words.
- **Listed word:** a word the grammar names with its class. **Irregular line:** a word's
  own stems or forms.
- **Relation:** how one cell's form is told from another's, by a choice among *changes*
  (`("tun~","tue~")`: the old pattern, the new).
- **Truth held twice:** two copies of one fact or meaning. The six kinds are in the
  panel's prompt: a second implementation, two representations, a hand-kept copy, numbers
  twice, tooling outside the repo, one policy in several places.
- **Canary set:** panel 4's fixed fast parts. **Heavy canary:** Finnish nouns, verbs and
  proper nouns, Turkish verbs, Greek verbs.
- **Exact:** crates byte-identical and saved runs identical.

## How the panel ran

- **Evidence first** (about 2.5 h, two lanes):
  - the dev tenth of PR 21's grammar for all 75 parts, every scored form dumped beside the
    floor's;
  - the census read and floor-right words;
  - two read-only inventories of truths held twice: 32 entries in the trainer, runtime and
    sound-law, and 51 in the tooling, configs, READMEs and tests;
  - where `rules.rs`'s 3.41 MB go;
  - timings from PR 21's logs;
  - stack-sample profiles of the trainer and runtime (eu-stack: perf and samply are
    blocked by the kernel setting);
  - 20 tenth-9 words per part;
  - the extended ledger (278 rows).
- **Twelve members:** six specialities, each a Claude subagent and a Codex agent
  (`gpt-6.1-sol`, newer than panel 4's `gpt-6-astra`), splitting their material. 48 bets,
  each with its reach, bytes, learn time and inventory entry.
- **Probes** between rounds:
  - inspection runs;
  - diagnostics;
  - an optimized end-to-end test;
  - four instrumented comparisons in a scratch worktree;
  - five no-training analyses.

  They killed three bets and confirmed four.
- **The meeting:** five votes each, objections, 89 checks (67 true, 22 partly or false).

## What the panel agrees on

1. **The trainer already asks the runtime for most answers.** Scoring, listing (`misbuilt`),
   settling and the laws' settle all run `grammar.table` or the compiled program. The
   truths held twice sit in the search's shortcuts, the trainer's own predicates beside
   the runtime's, hand-kept lists and numbers, and the tooling.
2. **One copy pair is live and large: which change of a relation fits a form.**
   - The trainer accepts any change that binds (`crates/flexion-train/src/relations.rs:205`).
   - The runtime keeps only the binding changes with the longest literal before the stem
     (`crates/flexion/src/paradigm.rs:169-179`).

   On the training pairs, nearly always with the runtime wrong, the two filters tell
   different forms:
   - German verbs 1,110 on read relations (*tunken* → *tueken*, *landet* → *ländet*);
   - Spanish verbs about 23,490 (imperative_ustedes 12,221 of 15,943);
   - Russian nouns about 5,994;
   - Finnish nouns 274 (*ajoissa* → *akjoina*);
   - Russian verbs 40;
   - English verbs (*fishing* → *foush*);
   - German adjectives 447.

   The trainer never lists these, because it thinks they are right.
3. **The harmony reader agrees with the shipped law.** Panel 5's prompt opened on it, but
   on 8.0 M places where the shipped laws reproduce the dictionary, the two copies never
   disagree. The live bug there is stage order: 158 places where a class letter is read
   before a later law fills it (Finnish *bodyayn*, Turkish *mazur görürlar*,
   *metropolümiz*).
4. **The runtime is the trainer's hot path.** `misbuilt` (the runtime's tables for every
   training word) is 69% of Greek verbs' learn and 47% of Finnish nouns'. The sound-law
   engine is about half of Finnish nouns' learn and of the German build. Greek's
   `stress::place` alone is 33%. Threads are 3.2 of 8 busy on average.
5. **Cheap wins:**
   - CI's end-to-end test runs the trainer unoptimized: 806 of the job's 906 s, 34 s at
     opt-level 2.
   - Turkish peaks at 3.0 GB, not the 15 GB `xtask/src/lanes.rs` COSTS gives, so it can
     run beside Finnish.
   - 443,668 bytes of `rules.rs` are indentation, and 256,800 the word ` regular`.
6. **Several numbers are held twice and already disagree.**
   - A paradigm's word count is printed four times; in Finnish nouns two copies mean
     different things (Purkaminen 733 against 7,728) and the runtime's fallbacks read the
     wrong one.
   - `ROOT_TRUST` is typed 0.97; measured, it is 0.879–0.956 per part.
   - The tolerance principle is coded twice and disagrees at n = 2: 639 verdicts, 2,533
     `regular` flags.
   - The confirmation tenth (9) exists only in scripts and prose. `POOL = [2, 4, 6, 8]`
     with `dev_of(8) = 9` lets a non-quick pooled run fit its sharpness on it.

## The plan

Ranked by the votes, with the theme and the probes. Every item's full bet is in the
implementation prompt.

### Performance pass (decided; outside the six-item cap; in two parts)

Revised after the panel, on the owner's direction: **correctness before speed.** Part 1,
(a) and (b), goes first: neither touches what is learned, and both make every later screen
and CI run cheaper. Part 2, (c)–(h), goes after the items and experiments. Most of those
spots optimize code the fixes rewrite (`relations.rs`, `stress.rs`, the listing check),
and an exact speed-up of buggy code only preserves the bug. Each speed-up stays exact,
but against the corrected output: alone it must not change an answer, so any score change
is a fix's, never a speed-up's.

Exact (crates and saved runs identical). Time box 3 h, in this order, each kept only if
measured beyond noise:
- **(a)** The workspace optimized in tests (engineer-codex-4, 16 points; data-claude-5).
  Measured: the saved-build test 806 → 34 s.
- **(b)** Scheduling from measured costs: `COSTS` deleted, runs record their own times and
  peak memory, Turkish beside Finnish (engineer-codex-1, 10 points). Two lanes, about
  6,073 → 5,072 s per rebuild; three lanes measured separately.
- **(c)** The runtime's stress and writing path (engineer-claude-2). Greek `place` 33%;
  kill under 15% on the Greek verb table bench.
- **(d)** `misbuilt` building one table where `full` can't matter, and `fit` stopping a
  paradigm that can't win (engineer-claude-1, its completion half killed). Each with
  stage timers.
- **(e)** The relations-pruned audit in parallel (engineer-codex-2): about 41 s on one
  thread per Turkish try.
- **(f)** Relation scoring text prepared once (engineer-codex-3): Turkish relations about
  30% of samples.
- **(g)** The sound-law engine's per-apply `pattern`/`atoms` work: the chair's profile
  lead, unverified.
- **(h)** Release LTO and codegen units: kept only if the learn is 4% faster.

### Items

1. **One relation matcher** (one-source; linguist-codex-2, **60 points, every member's
   first**).
   - The runtime owns which changes fit a form and how a change is applied (inventory 1.3,
     1.7), and the trainer calls it.
   - Both policies (bind-only and longest-literal) are screened through the one function.
     The meeting expects bind-only, the trainer's, to win, since the runtime's filter is
     the copy that is wrong. The loser is deleted.
2. **Greek preverbs by the runtime's spelling** (one-source; classicist-claude-1, **44**,
   with classicist-claude-3's `Prefixes::split`).
   - The two halves (the join's spelling; the temporal augment counted) are screened apart.
   - Reach: every prefix-bearing part (Greek, Sanskrit, German and Italian verbs), and the
     prefix stage's output diffed across all 75 parts.
3. **Bugs with witnesses** (decided):
   - **(a)** Greek -εως (phonologist-claude-1, **29**).
   - **(b)** Harmony stage order (phonologist-claude-3 revised).
   - **(c)** A candidate printed at 106% (ἐκδίδωμι; found by the chair).
   - **(d)** German stacked prefixes (linguist-claude-2: *klettre ab hin*, *bege zu vor*).
4. **Truths held twice in the code:** a paradigm's count once (engineer-claude-4),
   `ROOT_TRUST` measured (data-claude-4), one tolerance function (phonologist-codex-3),
   and the exact deletions:
   - the root window and marks context;
   - one accepted-form test;
   - eval's sharpen copy;
   - the build-mark set;
   - the cache keys' file lists.
5. **The pipeline's numbers and roles in the repo:**
   - the root README's tables, bytes and costs by `xtask`;
   - tenth roles in code and the `POOL` leak fixed;
   - census kinds named once;
   - the printer's exact byte savings (indentation, the `regular` default);
   - licence files in the generated manifest.
6. **One way to learn a part** (inventory K1.2, narrowed): one `learn_part` used by
   pipeline, try, inspect, bench, audit and the end-to-end test. Turkish adjectives'
   teacher words are the witness: six of eight paths learn a grammar that isn't shipped.

### Experiments (last; each screened in 30 minutes or less)

- **E1. Accent class read with the paradigm** (phonologist-claude-2). Witnesses νύμφη →
  *νῠ́μφων*, ἐπιφάνεια.
- **E2. Vowel-initial reduplication as one copy law** reading the augment's map
  (classicist-claude-2). It deletes nine statements of the lengthening.

Both are below the vote median. They are kept because:
- they target confirmed witnesses in Greek verbs, the largest error mass (26,660 dev
  wrong forms);
- their screens are cheap (Greek verbs 7 min);
- E2 is itself a one-source deletion.

**Cut order** if the run passes its budget before the integration sweep: E2, E1, item 6,
item 5's census and licence parts, performance spots (f)–(h), item 3(c).

## The inventory, entry by entry

Every entry is deleted by an item or kept with its reason.

**Trainer, runtime and sound-law** (`inventory-trainer.md`):

| entry | what | disposition |
|---|---|---|
| 1.1 | harmony reader vs shipped law | **kept as the search statistic**: probe 1 found 0 disagreements in 8.0 M lawful places. Its one live consequence, stage order, is item 3(b). A full rewrite (phonologist-codex-1, 12–20 h) is not worth it today. |
| 1.2 | `Change::applies` pre-filter vs program | **kept, deferred**: exact for every kind but Inner, where the gap is measured nowhere. The next panel gets a probe counting builds the program changes outside the index. |
| 1.3, 1.7 | relation fit and application | **item 1** |
| 1.4, 1.10 | accent-class eligibility, fallback stress test | **E1** covers the class choice; the fallback's byte-counting test goes with it if E1 lands, else deferred. |
| 1.5 | listed stem found without the runtime's `stem_of` (γαλλιᾱσ-) | **item 2** (same prefix/front path) if its screen shows it; else deferred with the witness. |
| 1.6 | telling listed forms without `changing` | **kept**: no live witness; `settle` re-runs the runtime, so only bytes are at stake. |
| 1.8, 1.9 | exceptions keyed by dictionary form; lemma spelling by `fill` | **kept**: accepted gaps, documented in code, checked by `try`. |
| 1.11 | `Tables::pattern` cache vs `pattern_in` | **kept**: PR 21 proved them equal over every cell of every crate. |
| 2.1 | the laws' text parsed by sound-law-macros, compiled by the trainer through the API | **deferred**: no witness. A notation parser in sound-law is the fix, about 120 lines. |
| 2.3, 2.4 | proposals on unmarked letters; prose as the law's price | **kept**: no wrong law can ship (`try_law` runs the program); pricing by prose is a style fix. |
| 2.5 | orthography/grammar structs printed by hand | **kept**: the compiler and the end-to-end test guard them. |
| 3.1, 3.2 | cache-key file lists | **item 4** |
| 3.3 | build marks re-listed | **item 4** |
| 3.4, 3.5 | root window, marks context | **item 4** |
| 3.6, 3.7, 3.8 | inherited cells, accent place names, small constants | **kept**: they agree today; next panel. |
| 4 | `ROOT_TRUST`; dated numbers in comments | **item 4**: measured and shipped; comments dated. |
| 5 | scripts, printed index formula | **item 5**: scripts; the formula is guarded by the end-to-end test, kept. |
| 6.1 | tolerance twice | **item 4** |
| 6.2 | the 0.9 shares | **kept apart, named**: `ROOT_SHARE` at 2–9 voters is already unanimity, so the open "unanimity against 0.9" item is **retired**. |
| 6.3 | accepted-form test inlined | **item 4** |
| 6.4 | FNV twice, two unescapers | **kept**: the split's exact hash must not move; the writers emit only what both accept. |

**Tooling, configs, READMEs, tests** (`inventory-tooling.md`):

| entries | disposition |
|---|---|
| K1.1 (README measure vs `try`) | **deferred**: a 12–18 h refactor (data-codex-1). Item 6 does its learning half. The next panel's first candidate. |
| K1.2 (learn a part, 8 copies) | **item 6** |
| K1.3, K5.7 (three README writers) | **item 5** |
| K1.4, K1.7, K6.7 (the end-to-end test's own logic) | **item 6** for the learning; the rest kept with a note. |
| K1.5, K1.6 | **kept**: low; they agree. |
| K2.1 (cell names in two schemes) | **item 5** |
| K2.2, K4.10 (a part's size three ways) | **item 5**: bytes counted on the formatted files. |
| K2.3, K2.4 (README headings and column meanings) | **item 5** |
| K2.5 (sharpness tenth) | **item 5**: tenth roles in code. |
| K2.6–K2.8 | **kept**: low, no disagreement. |
| K3.1, K3.3, K5.3, K5.8, K6.9 (COSTS, heavy set, recorders) | **performance (b)** |
| K3.2, K3.4 (language and command lists) | **item 5** derives the language list from `flexion_configs::all()`; command lists kept. |
| K3.5, K4.1–K4.3, K4.6, K4.7 (stale README text) | **item 5** |
| K3.6 (manifest `include`) | **item 5** |
| K3.7 (`SOURCE`) | **item 4** |
| K3.8, K4.4, K4.5, K4.8, K4.9 | **item 5** where they're in READMEs; the rest kept, dated. |
| K5.1, K5.4, K5.5, K5.6 (scripts) | **item 5**: tables, bytes and the paired verdict in `xtask`. The canary set becomes a named list in `xtask`. |
| K5.2 (the confirmation tenth) | **item 5** |
| K5.9 (the systemd wrapper) | **kept**: one string in `--wrap`'s doc and the plan. |
| K6.1 (`editors.rs`), K6.3–K6.6 (config copies) | **deferred**: no disagreement but Russian's vowels (third copy) and Sanskrit gender. Next panel, with the configs' shared helpers. |
| K6.2 (yardsticks) | **kept**: the SIGMORPHON conventions are the yardstick's, not the grammar's. |

## Rejected, deferred or retired

- **Killed by probes:**
  - **The longest ending decides the class** (linguist-claude-1): +0.09 German nouns,
    −0.05 Polish, under its +0.15 line.
  - **One training compound makes a Finnish head** (ml-claude-3): 5 of 85 words, against a
    confirm line of 35.
  - **Fusing completion with scoring** (engineer-claude-1c): 8%, about 18 s.
- **Withdrawn after the meeting:**
  - the Greek census "correction" (data-codex-2's numbers came from the dump's letters-only
    column);
  - choices as a new table macro (engineer-claude-3's macro half: anchored keys feed four
    readers);
  - data-codex-5 (merged into performance b).
- **Deferred, no votes, with their evidence:**
  - **Tied words abstain** (ml-claude-1): probe 4 shows the PR 21 vote already moves most
    of them (678 German nouns keep an arbitrary label).
  - **Stricter one-cell laws** (phonologist-codex-5, 8(b)'s witnesses): the 8(b)
    regressions stand.
  - **Anchors after a failed round trip** (linguist-codex-3, 8(a)'s witnesses): reasentar,
    sarmentar, requebrar still 59 of 73 cells from the headword.
  - **Separation per prefix as a choice** (linguist-claude-3): item 3(d) first.
  - **Scoring choice rules after the runtime's eligibility** (ml-codex-2): diagnostic first.
  - **A feature-told form's status** (ml-claude-2): the meeting found trust isn't 1 for
    every status, so it can change top-1; next panel.
  - **One listing decision by forms** (ml-claude-4): the checks found untie also protects
    accent and homographs, and heads.least is set in five parts, not three.
  - **Derived law-search filters** (phonologist-codex-2): no witness; 10–18 h.
  - **Harmony judged by the compiled program in full** (phonologist-codex-1,
    linguist-codex-1): agreement measured.
  - **Accent eligibility shared** (phonologist-codex-4): E1 first.
  - **Latin names' part-specific marks** (classicist-codex-3): B1's witness stands; next
    panel.
- **Sanskrit athematic presents** (classicist-codex-2): **deferred again**.
  - The diagnosis shows 11.5% of dev forms built by no paradigm, and the choice costs 3.5
    points.
  - But each sandhi law has at most 4 training words against the laws' minimum of 5, and
    the dev tenth is 97 words.
  - The next panel should measure it on folds 2–8 first.
- **Russian names' animacy per number** (ml-claude): **retired**. The floor wins only the
  plural accusative, and its answers are incoherent per word.
- **Latin roots, unanimity against 0.9:** **retired** (above).
- **Release profile bundled with scheduling** (engineer-claude-5): split; the profile is
  measured in performance (h).

## Open disagreements, and what settles each

- **The relation matcher's policy:** bind-only against longest-literal, both through the
  one function.
  - The screen: principal-parts-given tries on Spanish, German and Russian verbs, Russian
    and Finnish nouns, English verbs, German adjectives.
  - The witnesses with one source cell supplied: tunken → tunke, landet → landest,
    ajoissa → ajoina, fishing → fished.
  - The instrumented disagreement count must be 0 after.
- **Item 2's halves:** the join's spelling and the temporal augment, screened apart.
  ἡλόω and ὑστερέω are not compounds (linguist-claude's objection).
- **Two lanes or three:** performance (b) replays both, then validates the chosen
  schedule during the required rebuild.
- **`fit`'s early stop:** the exactness argument holds only if no fitted paradigm has a
  second variant matching where the first doesn't. Count it.
- **`ROOT_TRUST`:** the measured share per part against the constant, screened on Latin
  and Greek.

## Cost

About 12–13 hours of wall time, the final pipeline included. Items are implemented in
parallel worktrees, so wall time is set by the screens on two lanes:
- performance part 1 (the test profile and the scheduler) makes every screen cheaper; part 2 speeds up the integration set, tenth 9 and the rebuild;
- the heavy canary is about 35 minutes with Turkish beside Finnish;
- one integration set of 75 parts against the panel's dev baseline, about 1 h;
- tenth 9, about 75 minutes;
- the twelve builds, about 85 minutes after (b).
