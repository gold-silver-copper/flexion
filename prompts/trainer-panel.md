# Prompt: the trainer panel's plan — honest scores, then the class choice, length and twins

You own one PR on `~/code/flexion` (branch from the latest `main`, at or after `7bdd762`,
"Merge pull request #15"). A panel of four specialists (linguist, machine-learning
specialist, Rust engineer, data and evaluation specialist) read the trainer, argued, and
agreed on a ranked plan. This PR carries out the whole plan. You may change anything
(runtime, trainer, configs, `sound-law`, generated crates, tests, CI, xtask, APIs) if it
serves that. The project is pre-release: no backwards compatibility, no shims; delete what a
better design replaces.

Work through §2 in the order given: the first items make measurement trustworthy, and every
later item is judged by them. An item is kept only where it pays, measured by the protocol
of item 1 and logged. An item that doesn't pay is dropped with its numbers. No prototypes or
spikes: what lands is finished. The panel's evidence is a starting point, not gospel. Check
it, and where the code says otherwise, follow the code and log why.

## 0. Where things stand

flexion learns each language's inflection from Kaikki/Wiktionary dumps and prints it as a
Rust crate that *is* the grammar. Twelve languages ship. Read these first:
- the root `README.md`;
- one generated crate (`languages/ancient-greek/src/verb/`, `languages/finnish/src/noun/`);
- the trainer (`crates/flexion-train/src/`), the runtime (`crates/flexion/src/`) and
  `crates/sound-law`;
- the log `~/code/notes/projects/flexion/grammar-crates.md` (PR 12–15 sections): what was
  built, tried and dropped, and why.

**The panel's record** is in `~/code/notes/projects/flexion/`:
- `trainer-panel-2026-10-14.md`: the chair's decision. It has the plan, the evidence with
  file:line, what was rejected and why, and the open disagreements.
- `trainer-panel-2026-10-14.json`: each specialist's full proposals and meeting notes.

Read both before starting. Where this prompt is brief, they have the detail.

Held-out top-1 on the README's tenth, as measured today. Columns: from the headword; typed
in full; the floor (the CoNLL–SIGMORPHON 2017 baseline, learned per cell):

| | from the headword | in full | floor |
|---|---:|---:|---:|
| English noun / verb / adj / adv | 98.58 / 97.74 / 96.90 / 99.34 | same | 98.39 / 98.24 / 95.02 / 98.90 |
| Russian noun / adj / verb (no stress) | 95.10 / 99.75 / 96.02 | same | 94.72 / 99.78 / 96.17 |
| Latin noun / adj / verb | 66.36 / 82.53 / 70.47 | 93.37 / 99.28 / 85.25 | 59.45 / 71.85 / 65.34 |
| Ancient Greek noun / adj / verb | 67.48 / 72.28 / 66.60 | 85.52 / 86.14 / 75.74 | 49.22 / 50.81 / 46.07 |
| Sanskrit noun / adj / verb | 92.10 / 92.00 / 90.05 | same | 91.70 / 89.95 / 90.01 |
| Polish noun / adj / verb | 93.90 / 99.92 / 96.71 | same | 93.78 / 99.99 / 96.84 |
| German noun / adj / verb | 93.94 / 98.77 / 95.18 | same | 94.71 / 99.01 / 73.53 |
| Spanish noun / adj / verb | 99.31 / 98.66 / 98.91 | same | 99.31 / 98.93 / 99.20 |
| French noun / adj / verb / participle | 98.78 / 98.12 / 98.73 / 99.88 | same | 98.51 / 98.29 / 99.30 / 99.64 |
| Italian noun / adj / verb / participle | 97.91 / 98.84 / 97.61 / 100.00 | same | 97.54 / 98.84 / 99.24 / 100.00 |
| Finnish noun / adj | 97.75 / 97.64 | same | 98.30 / 97.51 |
| Turkish noun | 96.56 | same | 93.37 |

Other figures:
- Top-3 from the headword: German nouns 98.82, Polish nouns 98.72, Latin nouns 86.49,
  Finnish nouns 98.91.
- Finnish held-out nouns built on a training word score 98.6%; the rest score 96.0%.
- Principal parts given: Latin about 98.6, Greek verbs 80.8.

**These numbers are inflated and noisy; item 1 replaces them.** The headword cell is
scored as a prediction even though its answer is the typed word, for both the grammar and
the floor. The ± is not a paired test. `try` defaults to the very tenth the README reports.

**Size means the generated crates only.** The trainer and runtime may be as large as they
need. Generated crates: 21.3 MB, about 534k lines.

| crate | MB | lines |
|---|---:|---:|
| ancient-greek | 4.57 | 78,264 |
| russian | 3.02 | 69,355 |
| german | 2.88 | 82,321 |
| latin | 2.01 | 56,428 |
| polish | 1.80 | 51,834 |
| spanish | 1.68 | 52,453 |
| finnish | 1.59 | 38,228 |
| italian | 1.32 | 36,721 |
| english | 0.89 | 28,536 |
| sanskrit | 0.88 | 20,090 |
| french | 0.41 | 12,106 |
| turkish | 0.27 | 8,073 |

Build times (`cargo xtask build`, one at a time):
- Finnish ~50 min, German ~40, Russian ~30, Polish ~19, Greek ~18;
- the rest 1–11 min;
- about 3 h for all twelve.

**Tried and dropped before.** Don't repeat these without a new idea; the numbers are in the
log and the panel record:
- gender as a hard filter;
- one global two-sided vowel-length context (Latin nouns −1.3, Greek adjectives −4.4);
- restoring marks when typed in full;
- Russian stress via the uc-prosody branch (lost Russian verb points);
- naive-Bayes beginning rules;
- prefixes for every language;
- marking cells `derived` wholesale;
- a fixed 3-letter compound head (Latin nouns −8.6, Russian nouns −2.4);
- derived cells from a cell of another stem slot (doubled Greek verbs' irregular forms).

## 1. Read first

- The panel record (§0), then the code it cites: `eval.rs` (`outcomes`, `floor`, around
  lines 249, 343–346, 405–433), `pipeline.rs:211`, `xtask/src/main.rs:68`,
  `script.rs:98–112`, `lib.rs:4–9`, `laws.rs:1417`, `extract.rs:471`,
  `learn.rs:1513–1540, 2254–2310, 2369`, `stems.rs:335`, `grammar.rs:1331–1373`, and
  `measure.rs` (`compare` already does a word-level paired bootstrap).
- Literature behind the items:
  - Item 1: Gorman et al. 2019, on gold errors in SIGMORPHON data; Kodner, Khalifa & Payne
    2023, on lemma-disjoint and cell-disjoint splits.
  - Item 8: Stump 2001, *Inflectional Morphology*, on rule blocks and outer affixes.
  - Item 6: Allen 1978, *Vox Latina*, and Allen 1987, *Vox Graeca*, on hidden quantity and
    the written marks.
  - Item 5: Bonami & Beniamine 2016, on implicative entropy and which cells predict the
    class.
- For papers, use services that need no identity: OpenAlex, arXiv, ACL Anthology. Never
  send the user's email address or other personal details to any outside service.

## 2. What to do, in order

### Item 1. Honest scoring: one protocol change, one re-baseline

**Lemma cell.** In `eval::outcomes` and `eval::floor`, skip a cell when its first form,
read through the orthography, has the same letters as the typed input. Do this for both
the grammar and the floor. Where adding marks to the headword is a real task (Latin and
Greek vowel length), report that cell as its own "lemma spelling" column.

**Paired uncertainty.**
- Replace the README margin (`pipeline.rs:211`) with the word-level bootstrap from
  `measure::compare`.
- Make `eval::floor` return per-word outcomes. Report grammar − floor as a paired interval
  on the same words, with a sign-test p-value.
- Hoist the floor's `unseen` readings call out of the per-cell loop.

**A sealed test tenth.**
- `try` defaults to a development tenth that differs from the tenth sharpness is fitted on.
  `--final` scores the test tenth.
- Every decision in this PR is made on development tenths. The test tenth is scored once,
  at the end, for the READMEs.
- Say in the README that the test tenth was looked at by earlier PRs.

**New columns:**
- "any listed form counts": a cell with alternatives is right if any of them is given;
- the homograph ceiling: forms no grammar can get right, because two entries spelled the
  same have different tables;
- "built on a training word" vs "new root". Split by the grammar's own compound-head and
  prefix routes, not a suffix guess;
- in `try`, a per-cell grammar-vs-floor table with paired intervals, so each below-floor
  row says which cells lose and whether the loss is real.

**Re-baseline.** Run the new protocol on the grammars as they are now (before items 3–8) on
the development tenths, every language and part of speech. That is the "before" every later
item is compared with. Log it.

**One test.** The lemma cell is skipped when it equals the typed input and scored when it
differs (a Latin headword typed without marks). Show it fails without the change.

### Item 2. Reproducible output, and dead code deleted

**A fixed hasher.**
- Replace `BuildHasherDefault<DefaultHasher>` (`lib.rs:9`) with a small fixed in-crate
  hasher. Its algorithm can't change with the Rust release, as std's may.
- Give the hasher a seed that only a test can change.
- Move `laws::held_out`'s own `DefaultHasher::new()` (`laws.rs:1417`) onto `eval::hash`.
  That one is a data split, and the seed must not move it.

**One test.** Print a fixture crate under two hash seeds and assert the output is
identical. Use the Finnish fixture, which exercises laws, gradation and heads. This catches
any hidden dependence on hash order. If the output differs, fix the order-dependence; don't
weaken the test.

**Delete:**
- `Lexeme.weight`, `soft_entry_weight` and `soft_sense`: `weight` is set at
  `extract.rs:471` and never read;
- `eval::held_out_bits`, which has no caller;
- the Greek config comment that promises soft entries "train with less weight".

**Check.** The crates change once. Confirm with item 1's paired comparison that the shift
is noise.

### Item 3. Greek: a circumflex on a diphthong's second letter isn't vowel length

In `Orthography::read` (`script.rs:98–112`), a circumflexed vowel outside `long_vowels`
gets a macron, whatever letter comes before it. So `οῦ` and `αῖ` read as long `ῡ` and `ῑ`.
That splits Greek paradigms into twins (ἀρῐθμος, 507 words, vs τρᾰγος), and every
end-accented word becomes a coin flip between them. In the generated crates, 36 of 138
noun paradigms, 10 of 72 adjective paradigms and 123 of 293 verb paradigms carry such
cells.

**The fix.**
- A circumflexed vowel that is the second letter of a configured diphthong reads as the
  plain letter.
- A diaeresis marks the two vowels as separate syllables, so it keeps the long reading.
- The diphthongs come from the config; nothing language-specific goes into the trainer.

**Check:**
- The end-accented genitives still get `ῦ` back from `Accent::circumflex` once the twins
  merge.
- Twins merge: the Greek paradigm count falls.
- Top-1 on Greek nouns and adjectives moves (the panel guessed +1–3, unverified).

**One unit test.** `ἀριθμοῦ` reads with no macron, and a diaeresis keeps the vowels
separate.

### Item 4. Tooling: where the time goes, and every language in one command

**Per-stage trace.** Record wall time and peak memory (VmHWM from `/proc/self/status`) after
each stage of `learn()` and `pipeline::build`. Print them in `try` and `build`.

**`xtask sweep`.**
- Run `try --quick --save` for every language and part of speech, one after another, under
  the thread cap.
- Compare two sweeps as one paired table: per row, the difference with its interval, and
  the time and memory ratios.
- Print an estimated total duration before starting.

**Parallel map.**
- Replace the four copy-pasted thread blocks (`learn.rs:2369`, `stems.rs:335` and the
  others) with one work-sharing parallel map that has a thread cap.
- Use it only at the top level of a stage, never nested.
- It must fix the latent bug there: `filter_map(|h| h.join().ok())` silently drops a failed
  chunk, and results are matched to words by position. A failure is an error, never a
  shift.

**Three learns per part of speech.** A build learns three times per part of speech
(`pipeline.rs`: fit, train, all). Report what that costs from the trace. Leave the decision
to the user: scoring with the 8/10 grammar would save about a third of build time.

**Check.** Output stays byte-identical. Use `bench --dump` and the end-to-end test.

### Item 5. Wrong class chosen, or forms built wrong?

These are diagnostics. They decide the later items, and they never ship in a crate.

- **An oracle-paradigm column**, evaluation only: each held-out word scored with the learned
  paradigm that builds most of its forms. Label it clearly as a diagnostic, since it uses
  the answer table. Grammar against oracle is the class-choice error; oracle against 100 is
  the building error.
- **Dictionary-style principal parts** in the configs, where they're missing:
  - Finnish: gen sg, part sg, part pl, ill sg;
  - German nouns: gen sg (beside the plural);
  - Turkish: acc sg.
- **A ceiling classifier**, trainer-only and never shipped: regularized logistic regression
  (or boosted trees) predicting the paradigm.
  - Features: the word's last and first letters at several lengths, word length, vowel
    count (syllables, from the orthography's vowels), and the inherent features where the
    dictionary gives them (gender, animacy).
  - Run ablations, removing one feature group at a time. Report how far it beats the
    shipped ending rules, and which features carry that.
  - Report with homograph keys left out, because the labels are noisy.
- **A tie count**: listed exception words that have a tied paradigm the rules already
  predict.

**What follows from the results.** All of these are in scope for this PR:
- Where gender or syllable count carry real signal in the ablation, add them as readable
  conditions of choice rules. Print them as rules, never weights. A word given without the
  feature still gets every form.
- Where the tie count is large, reassign words among exact ties. Move a word to the class
  its ending predicts only when that class builds every one of its cells identically,
  unattested ones included. Fix the "fullest candidate" placement (`learn.rs:1513–1540`) in
  the same change.
- Where neither shows signal, log the numbers and leave them out.

The panel deferred the gender and syllable-count conditions because, from the bare
headword, the runtime already averages over a guessed gender. Judge them on paired dev
numbers.

### Item 6. Vowel length: one model per language, and a beam

This is the largest accuracy lever: Latin nouns 66.4 from the headword vs 93.4 typed in
full. 139 of 200 sampled Latin noun errors are vowel length.

**One model per language.**
- Learn vowel-length restoration once per language, from the lemmas and stems of every
  part of speech.
- Print it once at the language level, not per part of speech. Mind crate size.
- Allow 2–3 letters of left context, kept by the existing leave-one-out test.
- This needs a language-level stage before the per-part loop (`pipeline.rs:53`).

**A runtime beam.** Today the runtime flips only the least-sure vowel
(`grammar.rs:1331–1373`). Replace that with a top-3 beam over all vowels, so the
alternatives reflect the combined doubt. The flip alone moved Latin nouns' top-3 from 85.28
to 86.51 (log).

**Step 2: a root inventory, only if it pays.**
- Hidden quantity (`iūdex`, `lūx`) is lexical, and no context rule recovers it. First
  measure how often a held-out word's root has its length shown in a training word of
  another part of speech.
- Add an inventory of root spellings (`ū as in iūdicō`) only if that rate is high, and only
  if what it gains is worth its crate bytes. The panel's ML specialist estimated 100–300 KB
  per crate.

**Soft entries.** Settle the open question on paired dev numbers: train the marks on
Classical-only entries vs all entries.

**In the README,** say that a held-out noun's sibling verb may be in training.

### Item 7. Compounds and prefixes: decide per head word

**Per-head decisions.**
- Today one global minimum head length must hold for 90% of words (`learn.rs:2254–2310`).
  Italian and Spanish verbs come out at `heads: Some(8)`, the cap `HEAD_MOST`, so the head
  route is nearly off.
- German nouns and verbs, French verbs, and Polish nouns and verbs learn `None`. That is
  most of the below-floor rows, and the floor gets those compounds for free by matching
  long endings.
- Replace the global length with a decision per head word ("compounds of *fare* inflect as
  *fare*"). Keep a head only where the leave-one-out test holds. Print the heads as a
  readable list.

**Italian prefixes.** Find out why the prefix learner keeps only `ri`
(`languages/italian/src/verb/prefixes.rs`) and drops dis-, pre-, contra-, bene- and male-.
Ending rules can't tell `-fare` or `-dire` verbs apart:
- `rules.rs:36`: `"fare" => [Girare 90, Fare 21, Disfare 3]`;
- `rules.rs:250`: `"dire" => [Uscire 101, Dire 14, …]`.

**Gate.** Start once item 1's per-cell, per-stratum floor table shows where the floor wins:
on words built on a training word, or on new roots.

**What the panel disagreed on.** Item 1's tables decide which of these explains each
language's floor gap:
- Italian verbs: the head length, the prefix learner, or the order of stored alternatives
  (`~etti|~ei`);
- German nouns: missing umlaut, or compounds the floor memorizes.

### Item 8. Twin paradigms from missing operations

**An outer affix layer.**
- Where every form of a class is another class's form plus a constant affix, state it as
  "like X, with -ся". A law may choose the affix's variants, as Russian `-ся` after a
  consonant and `-сь` after a vowel.
- Today Russian has 136 of 342 verb paradigms as reflexive copies, Spanish 87 of 161, and
  Italian 38 of 120.
- Handle verbs that are only reflexive (бояться has no plain base), and participles, which
  put the affix after the adjective ending.
- Guard against false layers in other languages.

**A stem-vowel operation in `sound-law`.**
- One operation changes the stressed or last full vowel of the stem, triggered per cell.
- It covers German umlaut (14 noun paradigms are umlaut twins), ablaut where it pays, and
  the written accent Spanish and Italian enclitics need (`acomódate`).
- It also unlocks the Romance half of the affix layer.
- Extend `sound-law` rather than working around it.

**Gate and judgement.**
- Judge each part on paired dev numbers.
- Do the umlaut part only once item 1 shows umlaut errors, not compound errors, make up
  German's gap.
- Expect mainly smaller, more readable crates. Don't claim a closed floor gap unless the
  paired interval says so.

### Not in this PR

The panel rejected or postponed these; reasons are in the decision file:
- a sweep over the hand-set constants;
- corpus frequency lists;
- Russian stress;
- splitting `learn()` into typed stages;
- borrowed lexemes;
- a calibration refit;
- literature splits;
- a second-edition-only unseen-word set;
- a hard-example sample for Finnish;
- Finnish harmony twins;
- a "same length in all forms" rule.

Add a line to the log if something you measure makes one of them urgent, but don't do it
here.

## 3. What must stay true

- **A crate is its grammar.** Named items (laws, stem slots, ending tables, paradigms,
  relations, heads, accent marks), evidence on every rule, no probability tables, opaque
  ids or weights. Any classifier lives in the trainer only.
- **The end-to-end test** prints learned grammars, compiles them with `-D warnings` and
  compares every answer. It keeps passing and covers every new item: the affix layer,
  per-head lists, the language-level length model, any new choice conditions.
- **Every word has every form in every cell,** attested or not: no gaps, no defectiveness,
  never the lemma as a stand-in. Rarity or doubt goes into status and reason only.
- **Answers and output.** Calibrated candidates with a status and a reason on every answer.
  No panics (clippy denies unwrap/expect/indexing/panic). Nothing is computed before the
  first call. Output is byte-identical for the same input, now across toolchains too
  (item 2).
- **Nothing language-specific in the trainer or runtime.** If `sound-law` or another of the
  project's crates lacks a feature, add it there.
- **Code.** Idiomatic, type-driven Rust; clippy clean with `-D warnings`; README examples
  pass.

## 4. How you work

- No `/tmp`; put scratch files in `~/.cache/claude-tmp/`.
- **Resource caps.**
  - Parallelism: `-j 4`, `--test-threads 4`, at most 8 training threads.
  - Run every trainer process under
    `systemd-run --user --scope -q -p MemoryMax=16G -p MemorySwapMax=0`.
  - Build generated crates one at a time.
  - Nothing else over about 10 minutes, nothing using all cores, nothing with lots of RAM,
    without asking.
  - Ten-fold runs: one language at a time, and ask before any over 10 minutes.
  - A full `xtask sweep` or a rebuild of all twelve crates takes hours: ask first, and say
    roughly how long.
- **Fast loops.** `cargo xtask try <lang> --pos <part> --quick` (after item 1: a development
  tenth). Develop on small, quick languages first (Sanskrit, Turkish, French, English), and
  run Finnish, German and Russian only to confirm.
- **Clean up.** Kill everything you start, by PID. Never `pkill -f` a pattern that matches
  your own command line.
- **Tests.** Few and meaningful. No golden, snapshot or parity tests. Each test needs a
  verified failure witness: break the code, see it fail, restore.
- **Progress log.** Dated entries in `~/code/notes/projects/flexion/grammar-crates.md` under
  "PR 16", with numbers, what was tried and dropped, and the paper pages behind each
  choice.
- **Subagents** are allowed for independent measurement and audits. Don't run memory-heavy
  builds in parallel.
- **Talking to the user.** Plain language, before/after code for API changes, and
  copy-pasteable commands starting with `cd ~/code/flexion`. Before a long wait, say what
  you're waiting on and for how long.

## 5. Git

One PR from a branch off the latest `main`. Run fmt and clippy before pushing. Merge with
`gh pr merge --merge` once CI is green, then check out the latest `main`. No crates.io
publishing, tags or releases.

## 6. Done means

- **Every item in §2** is made, or measured and dropped with its numbers. The gated parts of
  items 5, 7 and 8 are decided by the diagnostics they name.
- **Measurement.** The new protocol is in place: lemma cell, sealed test tenth, paired
  intervals, grammar − floor per row and per cell, the new columns, the stage trace and
  `xtask sweep`.
- **Before/after**, on development tenths under the new protocol, for every language and
  part of speech, with no significant loss anywhere (paired bootstrap). Every below-floor
  row is either explained by its per-cell table or closed.
- **The panel's open disagreements** each have the measurement that settles them, logged.
- **All twelve crates** regenerated and compiling; README examples pass; CI green.
- **Size.** Total generated size no larger than 21.3 MB. Greek smaller than 4.57 MB.
  Russian, Spanish and Italian verb paradigm counts lower if item 8 lands.
- **READMEs** (root and crates) re-baselined on the sealed test tenth, scored once. They
  include real excerpts of anything new (an affix layer, a per-head list, the length model)
  and a note that earlier PRs looked at that tenth.

## 7. Final report

- **The PR link.**
- **Per language and part of speech**, before and after, on development tenths and then
  the final test tenth:
  - top-1 from the headword, lemma cell excluded, with the lemma-spelling column where it
    applies;
  - in full; principal parts given;
  - "any listed form" and the homograph ceiling;
  - built on a training word vs new root;
  - top-3; the floor, and grammar − floor with its paired interval;
  - attested; calibration.
- **Diagnostics.** The oracle-paradigm and ceiling-classifier results, with the ablation:
  which features carry the class.
- **What each item gave,** what was dropped and why, and how each open disagreement was
  settled.
- **Cost.** Generated size per crate before and after (bytes, lines, paradigm counts). Per
  stage time and peak memory for Finnish, German and Russian.
- **Verification.** How you verified it, and how the user can run it.
