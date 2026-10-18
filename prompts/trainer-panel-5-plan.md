# Prompt: the fifth panel's plan — one source of truth, a faster trainer, and the bugs it found

You own one PR on `~/code/flexion`. Branch from the latest `main`, at or after `98699c9`
("Merge pull request #21"). Commit this prompt as the PR's first commit.

**The panel.** A fifth panel of twelve met on 2026-10-18:
- six specialities: linguist, classicist, phonologist, ML, Rust engineer, data and
  evaluation;
- each with a Claude member and a Codex member, who split their material.

Its theme was set by the owner:
1. **One source of truth.** Every fact and every meaning is defined once, and everything
   else reads it.
2. **A dedicated performance pass** over the whole library.
3. Then errors and bytes.

**Correctness comes before speed.** The two performance changes that don't touch what is
learned (the test profile and the scheduler) go first, because they make every later
screen and CI run cheaper. Every code-level speed-up waits until the fixes are in: most
of them optimize code the fixes rewrite (`relations.rs`, `stress.rs`, the listing check),
and an exact speed-up of buggy code would only preserve the bug. Each speed-up is exact
against the corrected output: alone, it must not change an answer, so that any change in
a score is a fix's or an experiment's, never a speed-up's.

Before anyone proposed, the panel built an evidence pack:
- **Two inventories of truths held twice:**
  - 32 entries in the trainer, runtime and sound-law;
  - 51 in the tooling, configs, READMEs and tests.
- **Profiles** of the trainer and runtime.
- **The dev tenth of PR 21's grammar,** every form dumped beside the floor's.
- **The census read with examples.**
- **Answer samples.**
- **The ledger of past attempts.**

Members made 48 bets. The chair ran probes (four instrumented comparisons, five analyses,
inspections), the panel voted, and the chair decided the plan below. The record, in
`~/code/notes/projects/flexion/`:
- `trainer-panel-5-2026-10-18.md`: the decision. Read it first. It maps every inventory
  entry to an item or a reason.
- `-evidence.md`: the pack, with both inventories, the profiles and the probe results in
  full.
- `-dossier.md`: every bet.
- `-minutes.md`: votes, objections and 89 checks.
- `-ledger.md`: every mechanism tried since PR 10, PR 21's included.

The evidence itself is in `~/.cache/claude-tmp/panel5/`:
- `dump/<lang>-<part>.tsv`: **the dev baseline** (saved runs of PR 21's grammar,
  `try --quick`, all 75 parts, for `compare`);
- `dump/<lang>-<part>.forms.tsv`: every dev form, wanted against given, with the floor's
  form. The `right` column is letters only, accent aside;
- `dump/*.log`, `dump/*.mem`: the try output, and wall time and peak memory;
- `base/t9-pr21/`, `base/t9-main/`: tenth-9 saved runs of PR 21's final grammar and of
  main before it;
- `probes/`: every probe. `instrumented.diff` and `instrumented-probe.rs` are the scratch
  instrumentation of the relation, harmony, tolerance and tied-word comparisons: apply the
  diff in a scratch worktree, copy `instrumented-probe.rs` to
  `crates/flexion-train/src/probe.rs`, and switch each probe on with `PROBE_FIT`,
  `PROBE_HARMONY`, `PROBE_TOL` or `PROBE_TIED`. Never commit them;
- `prof/`, `sample.sh`, `traceable.py`, `prof.py`: the stack sampler. `perf` and `samply`
  are blocked here (`kernel.perf_event_paranoid = 2`); this samples with `eu-stack`
  through a launcher that allows tracing;
- `samples/`: 20 tenth-9 words per part (70 parts; five tiny parts have no tenth-9
  words);
- `probe-dump.diff`: the scratch patch to `xtask/src/main.rs` that makes `xtask try`
  write the forms dump when `PROBE_DUMP=<file>` is set. Apply it in a scratch worktree
  for witnesses; never commit it.

**This PR carries out the plan:** a performance pass, six items and two experiments. You
may change anything that serves it: runtime, trainer, configs, `sound-law`, generated
crates, tests, CI, xtask, APIs. The project is pre-release: no backwards compatibility, no
shims, no fallbacks. Delete what a better design replaces. A deleted copy is deleted, not
kept as a fallback or cross-check. An item that doesn't pay is dropped, with its numbers.
No prototypes or spikes: what lands is finished.

**Check the panel's evidence.** Where the code or a run says otherwise, follow the code
and log why.

**Never ask the owner anything.** Make every decision yourself, on evidence. That includes
linguistic rulings: research them and decide, citing the reference grammar. Keep working
until the PR is merged.

**This run is budgeted at about 12–13 hours of wall time,** the final pipeline included.
PR 21 took 21.5 hours: its final pipeline ran three times because of late findings (a
loss in a part outside an item's screen; a law reading a build mark, found only on tenth
9; bytes measured only at the end). §3 is how this run avoids that. Follow it exactly.

## 0. Where things stand

flexion learns each language's inflection from Kaikki/Wiktionary dumps. It prints each
language as a Rust crate that *is* the grammar: paradigms, ending rules, sound laws,
stem slots, relations, irregular lines, all readable. Twelve languages ship, each with
every part of speech it inflects. No cell is ever empty.

**Terms:**
- **Dev tenth:** tenth 2, scored by `cargo xtask try`. **Tenth 9:** the confirmation
  tenth, run once per PR. **README tenth:** tenth 0, scored by each build.
- **Floor:** the CoNLL–SIGMORPHON 2017 non-neural baseline, learned from the same words.
- **Paired comparison:** `cargo xtask compare a b`, a bootstrap over words, with the words
  better and worse.
- **Listed word:** a word the grammar names with its class. **Irregular line:** a word's
  own stems or forms.
- **Relation:** how one cell's form is told from another's, by a choice among *changes*
  (an old pattern and a new: `("tun~", "tue~")`).
- **Truth held twice:** two copies of one fact or meaning. The six kinds:
  1. a second implementation;
  2. two representations;
  3. a hand-kept copy;
  4. numbers written twice;
  5. tooling outside the repo;
  6. one policy in several places.
- **Exact:** crates byte-identical and saved runs identical.
- **Points:** the meeting's vote points (each member gave 5, 4, 3, 2, 1). **Inventory
  entries:** "1.3" is `inventory-trainer.md`'s numbering, "K1.2" is
  `inventory-tooling.md`'s; both are in full in `trainer-panel-5-2026-10-18-evidence.md`.
- **Canary set** and **heavy canary:** §3.

**Results on the README tenth** (PR 21's final build; the root README has the full
tables). Columns:
- **top-1** from the headword: for a closed class or a part under 30 held-out words,
  "reproduced N of N" and a new word's score;
- **in full:** the lemma typed with every mark;
- **principal parts:** the dictionary's principal parts given;
- **floor**, and **− floor:** grammar minus floor, paired, with its 95% interval;
- **irregular:** the share of training words with an irregular line;
- **wrong forms:** the count, then % class / marks / accent / stem / ending / variant /
  homograph;
- **by block:** top-1 averaged over the part's blocks.

| language | part | top-1 | in full | principal parts | floor | − floor | irregular | wrong forms: class / marks / accent / stem / ending / variant / homograph | by block |
|---|---|---:|---:|---:|---:|---|---:|---|---:|
| english | noun | 97.17 | 97.17 | – | 96.78 | +0.39 (+0.24…+0.55) | 0.11% | 671: 63 / 0 / 0 / 4 / 2 / 28 / 2 | 97.17 |
| english | proper_noun | 99.35 | 99.35 | – | 99.28 | +0.07 (−0.12…+0.28) | 0.05% | 28: 46 / 0 / 0 / 4 / 0 / 50 / 0 | 99.35 |
| english | verb | 97.23 | 97.23 | 99.57 | 97.79 | −0.56 (−0.87…−0.25) | 0.38% | 459: 74 / 0 / 0 / 3 / 3 / 16 / 4 | 97.23 |
| english | adjective | 95.35 | 95.35 | – | 92.53 | +2.82 (+2.39…+3.25) | 0.06% | 580: 79 / 0 / 0 / 0 / 0 / 20 / 1 | 95.35 |
| english | adverb | 99.01 | 99.01 | – | 98.34 | +0.66 (+0.25…+1.16) | 0.11% | 24: 42 / 0 / 0 / 0 / 0 / 58 / 0 | 99.01 |
| russian | noun | 94.64 | 94.64 | 95.73 | 94.72 | −0.09 (−0.52…+0.37) | 0.54% | 1482: 81 / 0 / 0 / 2 / 7 / 6 / 3 | 94.64 |
| russian | proper_noun | 90.77 | 90.77 | 92.59 | 93.60 | −2.83 (−4.13…−1.49) | 2.19% | 225: 88 / 0 / 0 / 3 / 7 / 1 / 1 | 90.77 |
| russian | adjective | 99.75 | 99.75 | – | 99.76 | −0.01 (−0.12…+0.09) | 0.07% | 412: 70 / 0 / 0 / 0 / 5 / 0 / 25 | 98.11 |
| russian | verb | 95.74 | 95.74 | 98.90 | 96.65 | −0.91 (−1.43…−0.42) | 0.75% | 876: 75 / 0 / 0 / 8 / 5 / 3 / 9 | 96.02 |
| russian | pronoun | repr. 1974/1974; new 48.83 | 54.26 | – | 54.26 | +0.00 (+0.00…+0.00) | 30.95% | 86: 95 / 0 / 0 / 0 / 5 / 0 / 0 | 54.26 |
| russian | numeral | repr. 2986/2986; new 83.69 | 94.12 | – | 100.00 | −5.88 (−32.53…+0.00) | 24.68% | 9: 0 / 0 / 0 / 0 / 100 / 0 / 0 | 94.12 |
| russian | adverb | repr. 185/185; new 77.30 | 66.67 | – | 83.33 | −16.67 (−33.33…+0.00) | 11.89% | 6: 17 / 0 / 0 / 67 / 0 / 17 / 0 | – |
| latin | noun | 71.83 | 92.93 | 98.70 | 59.09 | +12.74 (+10.05…+15.41) | 1.16% | 5031: 16 / 76 / 0 / 2 / 4 / 0 / 2 | 71.83 |
| latin | proper_noun | 60.13 | 92.79 | 98.32 | 60.58 | −0.45 (−4.19…+3.41) | 1.64% | 2041: 7 / 83 / 0 / 2 / 5 / 1 / 2 | 58.19 |
| latin | adjective | 87.82 | 99.23 | 99.48 | 71.87 | +15.95 (+14.14…+17.75) | 0.48% | 12373: 2 / 92 / 0 / 0 / 2 / 1 / 3 | 81.95 |
| latin | verb | 76.87 | 85.45 | 98.55 | 64.17 | +12.71 (+8.80…+16.80) | 4.69% | 17832: 46 / 38 / 0 / 5 / 4 / 1 / 6 | 77.03 |
| latin | pronoun | repr. 2211/2211; new 30.66 | 19.08 | – | 25.00 | −18.66 (−47.89…+3.14) | 73.75% | 251: 24 / 15 / 0 / 8 / 49 / 4 / 0 | 6.34 |
| latin | numeral | repr. 3996/3996; new 85.01 | 100.00 | – | 72.73 | +0.00 (−36.36…+36.36) | 0.85% | 108: 0 / 100 / 0 / 0 / 0 / 0 / 0 | 72.73 |
| latin | adverb | 88.57 | 98.35 | 96.72 | 44.57 | +44.00 (+30.05…+57.71) | 5.56% | 20: 0 / 90 / 0 / 0 / 5 / 5 / 0 | 88.57 |
| ancient-greek | noun | 74.96 | 89.14 | 96.10 | 51.28 | +25.29 (+21.30…+28.96) | 8.64% | 2875: 27 / 58 / 6 / 2 / 6 / 0 / 1 | 74.96 |
| ancient-greek | proper_noun | 60.28 | 86.95 | 91.55 | 47.42 | +14.45 (+8.29…+20.98) | 5.14% | 800: 14 / 74 / 4 / 2 / 6 / 0 / 0 | 60.28 |
| ancient-greek | adjective | 78.19 | 90.76 | 98.02 | 52.46 | +26.51 (+21.96…+30.76) | 2.78% | 4835: 28 / 58 / 3 / 4 / 4 / 1 / 1 | 80.85 |
| ancient-greek | verb | 68.30 | 75.88 | 81.95 | 44.20 | +25.27 (+19.32…+30.88) | 44.33% | 21862: 20 / 42 / 4 / 19 / 14 / 1 / 0 | 66.52 |
| ancient-greek | pronoun | repr. 1771/1771; new 53.75 | 40.16 | – | 41.96 | +7.08 (−8.14…+30.97) | 51.72% | 219: 6 / 0 / 15 / 1 / 78 / 0 / 0 | 40.33 |
| ancient-greek | numeral | repr. 301/301; new 66.78 | 100.00 | – | 100.00 | +0.00 (+0.00…+0.00) | 21.05% | 0: – / – / – / – / – / – / – | 100.00 |
| sanskrit | noun | 92.10 | 92.10 | – | 91.70 | +0.40 (−0.73…+1.48) | 1.57% | 1090: 50 / 0 / 0 / 10 / 6 / 7 / 27 | 92.10 |
| sanskrit | proper_noun | 95.15 | 95.15 | – | 86.48 | +8.67 (+3.40…+14.54) | 2.13% | 57: 33 / 0 / 0 / 26 / 40 / 0 / 0 | 95.15 |
| sanskrit | adjective | 91.86 | 91.86 | – | 90.11 | +1.75 (+0.12…+3.52) | 3.72% | 1095: 83 / 0 / 0 / 6 / 10 / 0 / 0 | 91.96 |
| sanskrit | numeral | repr. 1656/1656; new 90.52 | 63.64 | – | 92.42 | −28.79 (−55.13…−0.93) | 7.69% | 96: 0 / 0 / 0 / 0 / 100 / 0 / 0 | 63.64 |
| sanskrit | verb | 89.61 | 89.61 | 90.33 | 89.93 | −0.32 (−2.79…+2.02) | 23.63% | 746: 17 / 0 / 0 / 22 / 56 / 5 / 0 | 78.66 |
| sanskrit | pronoun | repr. 987/987; new 68.90 | 69.57 | – | 70.53 | −0.97 (−3.17…+0.00) | 46.67% | 63: 0 / 0 / 0 / 0 / 100 / 0 / 0 | 69.57 |
| polish | noun | 93.44 | 93.44 | 95.83 | 93.34 | +0.10 (−0.27…+0.46) | 1.80% | 2911: 69 / 0 / 0 / 3 / 12 / 4 / 11 | 93.44 |
| polish | proper_noun | 94.00 | 94.00 | 95.03 | 93.74 | +0.26 (−0.26…+0.81) | 1.36% | 1284: 39 / 0 / 0 / 2 / 7 / 3 / 48 | 94.00 |
| polish | adjective | 99.94 | 99.94 | – | 99.99 | −0.05 (−0.07…−0.03) | 0.36% | 51: 78 / 0 / 0 / 8 / 10 / 4 / 0 | 85.61 |
| polish | verb | 97.17 | 97.17 | 98.38 | 97.66 | −0.49 (−0.99…+0.02) | 1.35% | 1518: 67 / 0 / 0 / 5 / 11 / 10 / 8 | 96.82 |
| polish | pronoun | repr. 1550/1550; new 42.65 | 81.64 | – | 81.64 | +0.00 (+0.00…+0.00) | 60.71% | 65: 0 / 0 / 0 / 0 / 100 / 0 / 0 | 81.64 |
| polish | numeral | repr. 189/189; new 3.17 | – | – | – | – | 0.00% | –: – / – / – / – / – / – / – | – |
| polish | adverb | 93.75 | 93.75 | 100.00 | 95.31 | −1.56 (−7.81…+3.12) | 5.23% | 8: 0 / 0 / 0 / 0 / 100 / 0 / 0 | 93.75 |
| german | noun | 93.75 | 93.75 | 99.80 | 94.77 | −1.02 (−1.33…−0.73) | 0.49% | 2503: 67 / 0 / 0 / 4 / 12 / 12 / 4 | 93.75 |
| german | numeral | repr. 126/126; new 100.00 | 100.00 | – | 100.00 | +0.00 (+0.00…+0.00) | 0.00% | 0: – / – / – / – / – / – / – | – |
| german | proper_noun | 91.13 | 91.13 | 99.77 | 90.66 | +0.48 (−1.86…+2.78) | 0.28% | 112: 71 / 0 / 0 / 0 / 7 / 18 / 4 | 91.13 |
| german | adjective | 98.90 | 98.90 | 99.88 | 99.02 | −0.12 (−0.59…+0.35) | 0.07% | 1372: 64 / 0 / 0 / 0 / 11 / 25 / 0 | 98.24 |
| german | verb | 95.21 | 95.21 | 96.02 | 73.16 | +22.05 (+20.30…+23.83) | 5.07% | 2423: 57 / 0 / 0 / 19 / 7 / 2 / 15 | 94.24 |
| german | determiner | repr. 532/532; new 82.52 | 40.00 | – | 40.00 | +0.00 (+0.00…+0.00) | 13.89% | 27: 56 / 0 / 0 / 22 / 22 / 0 / 0 | 40.00 |
| spanish | noun | 98.67 | 98.67 | – | 98.67 | +0.00 (−0.20…+0.20) | 0.11% | 81: 74 / 0 / 0 / 5 / 4 / 16 / 1 | 98.67 |
| spanish | adjective | 98.50 | 98.50 | 99.67 | 98.60 | −0.10 (−0.35…+0.14) | 0.21% | 108: 81 / 0 / 0 / 5 / 7 / 7 / 0 | 98.57 |
| spanish | verb | 98.99 | 98.99 | 99.74 | 99.28 | −0.30 (−0.59…−0.01) | 1.68% | 678: 43 / 0 / 0 / 33 / 23 / 2 / 0 | 98.83 |
| spanish | determiner | repr. 173/173; new 80.35 | 90.91 | – | 100.00 | −9.09 (−30.00…+0.00) | 11.54% | 2: 100 / 0 / 0 / 0 / 0 / 0 / 0 | – |
| spanish | adverb | repr. 8/8; new 12.50 | – | – | – | – | 42.86% | –: – / – / – / – / – / – / – | – |
| french | noun | 97.59 | 97.59 | – | 97.03 | +0.56 (+0.23…+0.90) | 0.65% | 104: 75 / 0 / 0 / 5 / 1 / 16 / 3 | 97.59 |
| french | adjective | 97.53 | 97.53 | 99.95 | 97.75 | −0.22 (−0.65…+0.24) | 0.28% | 136: 81 / 0 / 0 / 3 / 13 / 3 / 0 | 97.57 |
| french | verb | 98.70 | 98.70 | 99.48 | 99.28 | −0.59 (−1.06…−0.22) | 0.96% | 454: 75 / 0 / 0 / 0 / 14 / 11 / 0 | 98.81 |
| french | participle | 99.84 | 99.84 | – | 99.52 | +0.32 (−0.32…+0.96) | 0.03% | 3: 100 / 0 / 0 / 0 / 0 / 0 / 0 | 99.84 |
| french | determiner | repr. 103/103; new 46.60 | 46.15 | – | 69.23 | −23.08 (−64.29…+0.00) | 22.45% | 7: 0 / 0 / 0 / 29 / 71 / 0 / 0 | – |
| french | adverb | repr. 4/4; new 0.00 | – | – | – | – | 0.00% | –: – / – / – / – / – / – / – | – |
| italian | noun | 95.82 | 95.82 | – | 95.09 | +0.74 (+0.31…+1.16) | 0.19% | 267: 85 / 0 / 0 / 6 / 1 / 3 / 5 | 95.82 |
| italian | adjective | 98.46 | 98.46 | 99.78 | 98.46 | +0.00 (−0.28…+0.28) | 0.19% | 122: 90 / 0 / 0 / 3 / 4 / 2 / 0 | 96.56 |
| italian | verb | 99.05 | 99.05 | 99.53 | 99.22 | −0.17 (−0.48…+0.14) | 1.77% | 665: 48 / 0 / 0 / 22 / 19 / 7 / 4 | 99.04 |
| italian | participle | 100.00 | 100.00 | – | 100.00 | +0.00 (+0.00…+0.00) | 0.02% | 0: – / – / – / – / – / – / – | 100.00 |
| italian | determiner | repr. 117/117; new 88.03 | 81.82 | – | 86.36 | −4.55 (−13.64…+0.00) | 13.33% | 4: 25 / 0 / 0 / 50 / 25 / 0 / 0 | – |
| italian | adverb | repr. 10/10; new 0.00 | 0.00 | 0.00 | 0.00 | +0.00 (+0.00…+0.00) | 75.00% | 1: 0 / 0 / 0 / 0 / 100 / 0 / 0 | – |
| finnish | noun | 98.07 | 98.07 | 99.34 | 98.29 | −0.22 (−0.41…−0.04) | 1.01% | 36661: 57 / 0 / 0 / 5 / 28 / 9 / 1 | 98.05 |
| finnish | pronoun | repr. 1484/1484; new 16.51 | 6.42 | – | 9.63 | −3.21 (−5.92…−0.99) | 69.33% | 175: 4 / 0 / 0 / 14 / 82 / 0 / 0 | 6.42 |
| finnish | numeral | repr. 1873/1873; new 31.50 | 32.14 | – | 43.30 | −11.16 (−41.96…+19.73) | 44.00% | 152: 18 / 0 / 0 / 27 / 55 / 0 / 0 | 32.14 |
| finnish | proper_noun | 93.82 | 93.82 | 99.01 | 91.77 | +2.05 (+0.72…+3.35) | 2.60% | 7391: 73 / 0 / 0 / 4 / 13 / 8 / 2 | 93.76 |
| finnish | adjective | 97.50 | 97.50 | – | 97.41 | +0.08 (−0.58…+0.76) | 0.83% | 1112: 75 / 0 / 0 / 2 / 21 / 2 / 0 | 97.42 |
| finnish | verb | 98.50 | 98.50 | 99.47 | 97.56 | +0.94 (+0.26…+1.69) | 3.97% | 2945: 54 / 0 / 0 / 0 / 41 / 0 / 5 | 98.33 |
| finnish | adverb | 96.67 | 96.67 | 98.57 | 97.39 | −0.71 (−1.67…+0.00) | 2.32% | 14: 64 / 0 / 0 / 14 / 21 / 0 / 0 | 96.67 |
| finnish | postposition | repr. 199/199; new 81.91 | 65.22 | – | 78.26 | −13.04 (−30.43…+0.00) | 4.88% | 8: 75 / 0 / 0 / 0 / 25 / 0 / 0 | – |
| turkish | noun | 96.62 | 96.62 | 99.52 | 93.27 | +3.35 (+2.45…+4.33) | 1.11% | 2402: 73 / 0 / 0 / 11 / 7 / 1 / 7 | 96.93 |
| turkish | pronoun | repr. 133/133; new 14.29 | 0.00 | – | 90.91 | −90.91 (−90.91…−90.91) | 61.54% | 11: 0 / 0 / 0 / 0 / 100 / 0 / 0 | – |
| turkish | numeral | repr. 77/77; new 37.66 | 31.82 | – | 0.00 | +31.82 (+0.00…+63.64) | 42.86% | 15: 0 / 0 / 0 / 0 / 100 / 0 / 0 | – |
| turkish | proper_noun | 90.94 | 90.94 | 97.74 | 93.24 | −2.31 (−6.43…+1.59) | 1.97% | 342: 75 / 0 / 0 / 10 / 13 / 3 / 0 | 91.56 |
| turkish | adjective | repr. 4116/4116; new 98.13 | 86.75 | 100.00 | 81.93 | +4.82 (+0.00…+14.46) | 1.11% | 33: 100 / 0 / 0 / 0 / 0 / 0 / 0 | 88.51 |
| turkish | verb | 98.90 | 98.90 | 99.92 | 98.68 | +0.22 (−1.06…+1.38) | 6.33% | 2330: 94 / 0 / 0 / 0 / 6 / 0 / 0 | 98.95 |

**Size, the generated crates only:** 14.51 MB.

| crate | MB |
|---|---:|
| ancient-greek | 3.35 |
| finnish | 2.11 |
| russian | 1.96 |
| latin | 1.50 |
| german | 1.23 |
| polish | 1.19 |
| turkish | 0.89 |
| sanskrit | 0.65 |
| spanish | 0.48 |
| english | 0.46 |
| italian | 0.42 |
| french | 0.26 |

By file kind:
- `rules.rs` 3.41 MB, of which 443,668 B is indentation and 256,800 B the word
  ` regular`;
- `paradigms.rs` 2.96, `relations.rs` 2.21, `irregular.rs` 1.97, `stems.rs` 0.89;
- `mod.rs` 0.52, `marks.rs` 0.49, `spellings.rs` 0.44, `animacy.rs` 0.43, `accent.rs` 0.37.

`~/.cache/claude-tmp/panel4-plan/bytes.py` measures this today. Item 5 moves it into
`xtask`.

**Run times** (measured by the panel; `~/.cache/claude-tmp/panel5/evidence/timings.md`,
`run-costs.md`, `profiles.md`):
- **Dev-tenth `try --quick`, all 75 parts:** 98 minutes of runs, about 55 on two lanes.
  - Finnish nouns 1,298 s, peak 10.8 GB; Finnish verbs 485 s, 7.2 GB.
  - Turkish verbs 466 s, **3.0 GB** (`xtask/src/lanes.rs` COSTS says 15).
  - Greek verbs 422 s, 1.2 GB; Turkish adjectives 701 s (they learn from the nouns'
    words too); Russian adjectives 261 s; German nouns 248 s.
- **Threads:** 3.2 of 8 busy on average.
- **A full rebuild (PR 21):** Finnish 4,106 s, then Turkish 1,967 s on one lane (they
  never ran together). Critical path 6,073 s. A build learns every part three times
  (8/10, 9/10, all), 87% of Finnish's time.
- **CI's test job:** 906 s, of which 806 s is one end-to-end test (`a_saved_build_prints…`),
  the trainer unoptimized there. At opt-level 2 it took 34 s (75 s with compiling).
- **Where the time goes** (stack samples):
  - **Greek verbs:** `misbuilt` (the runtime's table for every training word) 69% of busy
    time; inside it `flexion/stress.rs::place` 33% self, `orthography.rs::write` 13%.
  - **Finnish nouns:** the sound-law engine's matcher about 36% self
    (`apply_rules_with_budget`, `find_next_match`, `match_at`, `pattern`, `atoms`,
    `push_ends`); inclusive (overlapping) shares: the engine 47%, `misbuilt` 47%, `fit` 33%.
  - **Turkish verbs:** `relations.rs::changes`/`change` about 30% self, about 40% of it
    allocation. After the learn, the relations-pruned audit takes about 41 s on one
    thread.
  - **German build:** the sound-law engine about half.

**How decisions are judged** (the tools):
- **`cargo xtask try <lang> --pos <part> --quick --save <file>`:** the dev tenth.
- **`cargo xtask compare a b`:** the paired comparison.
- **`--fold 9`:** tenth 9. **`--final`:** the README tenth.
- **`cargo xtask inspect <lang> --pos <part> --word <w> --parts --holdout`:** one word's
  table, scored the way `try` scores it.
- **`try --diagnose`:** class choice and vowel-length diagnostics.
- **`cargo xtask bench <lang> --pos <part>`:** time per call and per whole table.

## 1. Read first

- **The decision,** `trainer-panel-5-2026-10-18.md`, then the probe results
  (`~/.cache/claude-tmp/panel5/probes/summary.md` and `instrumented.md`, `analyses.md`).
- **PR 21's final report,** `trainer-panel-4-plan-report-2026-10-18.md`, and the PR 21
  section of `~/code/notes/projects/flexion/grammar-crates.md`.
- **The code each item cites** (bare file names are under `crates/flexion-train/src/`):
  - **relations:**
    - the trainer's fit: `relations.rs:205-209`; its application: `relations.rs:432-438`;
    - the runtime's fit and application: `crates/flexion/src/paradigm.rs:162-197`;
  - **prefixes and preverbs:**
    - `learn.rs:1592-1866` (`prefixes`; the join check at :1721-1723, the longest prefix
      at :1760-1766);
    - the runtime's `Prefixes::split` at `crates/flexion/src/grammar.rs:150-162`;
    - `Orthography::initial_only` at `crates/flexion/src/orthography.rs:263-288`;
  - **the short endings:** `crates/flexion/src/stress.rs:270-280`;
  - **harmony's stage order:** `laws.rs:755-761`;
  - **German prefix stacking:** `crates/flexion/src/grammar.rs:790-810`;
  - **a paradigm's word count:** `crates/flexion/src/paradigm.rs:19`, `print.rs:429, 726,
    835, 854`, `learn.rs:197`;
  - **`ROOT_TRUST`:** `crates/flexion/src/grammar.rs:409-416`; the measurement,
    `xtask/src/main.rs:1715`;
  - **tolerance:** `crates/flexion/src/choice.rs:124-127`; `laws/harmony.rs:862-870`;
  - **the root window and marks context:** `marks.rs:106, 160, 165`, `length.rs:18, 91`,
    `crates/flexion/src/grammar.rs:411, 422, 1752-1756`;
  - **the accepted-form test:** `learn.rs:2056-2061, 3995-4003`, `eval.rs:617-660`;
  - **`same_form`:** `crates/flexion/src/grammar.rs:572-575` against `eval.rs:317`;
  - **sharpen:** `eval.rs:954` against `crates/flexion/src/grammar.rs:1872`;
  - **build marks:** `crates/flexion/src/pattern.rs:164` and the re-listings in
    `laws.rs:413, 485, 2128, 2377, 2580, 2846`;
  - **cache keys:** `stage.rs:23`, `saved.rs:48`;
  - **tenths:** `eval.rs:19` (`dev_of`), `xtask/src/main.rs:369` (`POOL`);
  - **scheduling:** `xtask/src/lanes.rs:16-33`;
  - **the learners** (inventory K1.2): `pipeline.rs:184-215`; `xtask/src/main.rs:535-545,
    592-600, 646-666` (try), `:1056-1070` (small parts), `:1779-1790` (inspect);
    `xtask/src/audit.rs:46-72`; `xtask/src/bench.rs:30-50`; `xtask/src/yardsticks.rs:232-242`
    (sigmorphon), `:346-352` (nonce); `crates/flexion-configs/tests/end_to_end.rs:46-68,
    166-178`;
  - **Turkish adjectives' teacher words:** `crates/flexion-configs/src/turkish.rs:192`;
  - **hot paths:**
    - `crates/flexion/src/stress.rs` (`place`), `grammar.rs:1694-1700`;
    - `learn.rs:3440` (`fit`), `learn.rs:3967-4010` (`misbuilt`);
    - `eval.rs:711` (`completion`);
    - the pruned audit `xtask/src/main.rs:1180-1206`;
    - `relations.rs:405, 540`;
    - `crates/sound-law/src/engine.rs:858, 862`;
  - **the accent class:** `crates/flexion/src/grammar.rs:1631-1643`, `learn.rs:3825`.
- **The references:**
  - Smyth §§ 163a, 169 (accent and -εως), 428-461 (augment; temporal augment § 435;
    reduplication §§ 439-446), 1652-1660 (preverbs, elision before vowels);
  - Wiese, *The Phonology of German*, and Duden *Grammatik* on separable and inseparable
    prefixes and stacked particles (hinab-, voran-);
  - Yang 2016, *The Price of Linguistic Productivity* (the tolerance principle, n/ln n).

## 2. The items

**Cut order** if the run passes its budget before the integration set: drop E2, then
E1, then item 6, then item 5's census (c) and licence (e) parts, then performance (f)–(h),
then item 3(c). Log each cut with its reason.

**Run order:**
1. step 0 (§3);
2. performance part 1: (a) the test profile and (b) the scheduler. Neither touches what is
   learned;
3. items 1, 2, 3 (one-source changes that move answers, and the bugs);
4. items 4, 5, 6 (the one-source work that is exact or nearly so);
5. E1, E2;
6. performance part 2: (c)–(h), the code-level speed-ups, on the corrected code;
7. the integration set (§3).

Items run in parallel in worktrees where they don't touch the same code (§5), but their
screens queue on the two lanes in this order.

Each item gives what to change, its bet (rows, predicted change, kill), its **reach**
(every part its code touches, the slow parts included) and **byte line**, and its cost.
The performance pass and items 1–6 are decided: their method is free, their gate is not.

### The performance pass (exact; decided; two parts, time box 3 hours in all)

Part 1 is (a) and (b), first. Part 2 is (c)–(h), after the items and experiments, so it
speeds up the code that ships, not code the fixes replace.

Every change here leaves the crates byte-identical and the saved runs identical **to the
code it starts from**: for part 2, the corrected code after the items and experiments. Measure
each with stage timers (the build logs print stages; add timers where a stage isn't
timed), at the same thread count, with the other lane idle: the median of three runs for
anything under 5 minutes (a part's `try`, `bench`), and one run each before and after,
read by stage timer, for the long ones (Finnish nouns' `fit` and listing stages). Keep a
change only if its stage is faster beyond noise. Work in this order; stop at the time
box, and log what was left.

- **(a) The workspace optimized in tests.**
  - In the root `Cargo.toml`, `[profile.dev.package.flexion-train] opt-level = 2`, and the
    same for `flexion`, `sound-law`, `sound-law-macros`, `flexion-macros` and
    `flexion-configs`, keeping debug assertions and overflow checks. (Today
    `[profile.dev.package."*"] opt-level = 2` reaches dependencies only, not workspace
    crates.)
  - Measured by the panel with exactly those overrides for flexion-train, flexion,
    sound-law and flexion-configs (`--config 'profile.dev.package.<crate>.opt-level=2'`):
    the saved-build test 806 → 34 s, 75 s with compiling. Cargo still prints
    "unoptimized + debuginfo" for the test profile; that line doesn't mean the overrides
    failed.
  - **Bet:** CI's test job 906 → under 300 s.
  - **Kill:** any test lost or skipped.
- **(b) Scheduling from measured costs.**
  - Builds and tries record their wall seconds and peak memory (the children's peak RSS,
    as `~/.cache/claude-tmp/panel5/mem.py` measures it) beside `target/build-cache/`.
  - `lanes.rs` reads the latest instead of `COSTS`. **`COSTS` and every hand-kept heavy
    list are deleted** (inventory K3.1, K3.3).
  - A run with no record runs alone. Admission is by measured memory against a budget
    (20 GB per run, 40 GB for all), and Turkish runs beside Finnish. Builds get
    `MemoryMax=20G`: a Finnish build peaks at 15.9 GB, at the edge of 16G.
  - First simulate PR 21's twelve recorded build times with two and three lanes (the
    panel computed about 5,072 s and 4,106 s). Then validate the chosen schedule during
    this PR's one real rebuild (§3.11). Three lanes are allowed for that rebuild only if
    the simulation favours them; keep three only if no part's learn slows over 20% under
    contention. Otherwise the two-lane cap of §5 holds.
- **(c) The runtime's stress and writing path** (engineer-claude-2).
  - `stress::place` syllabifies once per form, not once per accent class.
  - The vowel-pair test doesn't allocate.
  - `Orthography::initial_only` returns early when the language has no initial marks and
    the word is NFC already.
  - **Bet:** the Greek verb whole-table bench about 40% faster (PR 21 measured 33 ms, the
    panel's sampled run 39 ms: measure your own baseline first), and the Greek verb learn
    about 25% faster.
  - **Kill:** the table under 15% faster.
- **(d) Work the answer doesn't need** (engineer-claude-1, without its completion half,
  which a probe killed: 8%, about 18 s).
  - `misbuilt` builds one table where `full` can't matter (no marks, `l.full == plain`).
  - `fit` stops a paradigm that can't beat the best so far. That is exact only if no
    fitted paradigm has a second variant that matches where its first doesn't: count it
    first, and don't build only the first variant unless the count is 0.
  - **Kill:** Finnish noun `fit` under 40% faster.
- **(e) The relations-pruned audit in parallel** (engineer-codex-2): about 41 s on one
  thread per Turkish try, in `par::map`. **Kill:** the Turkish verb try under 20 s faster.
- **(f) Relation scoring text prepared once** (engineer-codex-3).
  - Character boundaries and suffix keys are computed once for the scoring sample.
  - **Kill:** Turkish's relations stage under 25% faster.
- **(g) The sound-law engine's matching** (the chair's profile lead, unverified): the
  recursive matcher (`engine.rs:858` `pattern`, `:862` `atoms`, `find_next_match`,
  `match_at`) is about 36% of Finnish nouns' busy time and about half of the German build.
  Cut match work without changing results: for example a literal prefilter that skips
  positions a rule's first literal can't match, before `atoms` runs. **Kill** (the
  chair's line): under 10% off the laws stages.
- **(h) Release settings.** Try `lto = "thin"` and `codegen-units = 1`; keep them only if
  Finnish noun's learn is 4% faster and the crates byte-identical.

After part 1, measure the light and heavy canary and the Finnish build: those are the
times the items' 20% ceiling is measured against. After part 2, measure them again for
the integration set, tenth 9 and the rebuild.

### Item 1. One relation matcher (one-source; the panel's first, 60 points from all twelve)

**The copies** (inventory 1.3, 1.7):
- The trainer's relation learner accepts a change for a form if its old pattern binds
  (`relations.rs:205-209`).
- The runtime keeps only the binding changes with the longest literal before the stem
  (`crates/flexion/src/paradigm.rs:169-179`).
- The application body (`fill`, or the laws on `fill_marked`) is written in both
  (`relations.rs:432-438`, `paradigm.rs:186-197`, also `paradigm.rs:361-370` and
  `crates/flexion/src/stems.rs:75-84`).

**The witnesses** (probe 2: training pairs the two tell differently on relations the
runtime reads; in nearly every pair the trainer's form is the dictionary's and the
runtime's is wrong, and nothing lists them):

| part | pairs | witnesses |
|---|---|---|
| German verbs | 1,110 | tunken → *tueken*, landet → *ländet*, kämmten → *kammten* |
| Spanish verbs | about 23,490 | imperative_ustedes 12,221 of 15,943: `("aca~","acá~se")` captures whole classes; traiga → *tiraies* |
| Russian nouns | about 5,994 | |
| Finnish nouns | 274 | ajoissa → *akjoina* |
| Russian verbs | 40 | ездила → *ёзди* |
| English verbs | 1,611 | fishing → *foush* |
| German adjectives | 447 | absurder → *äbsurderer* |

**What:**
- One runtime function says which changes fit a form, and one applies a change. The
  trainer's relation learner, `Relation::apply` and the stems path all call them, and the
  copies are deleted.
- **The policy is settled by measurement, through the one function:**
  - **(a) bind-only:** the choice's rules decide among the changes that bind;
  - **(b) the longest literal before the stem.**
- Screen both. The meeting expects (a): every member objected that keeping the runtime's
  filter keeps the wrong forms, and a listed exception can't override it, because listed
  values pass through the same filter. Keep the winner; delete the other.
- Learn time: compute the admissible set once per distinct source form, not per (word,
  change). The relations stage is Turkish verbs' largest.

**Bet:**
- Rows (the chair's estimate; the bet's author forecast 0.00 before probe 2): principal
  parts given (relations are read when forms are given):
  - Spanish verbs +0.3;
  - German verbs +0.1;
  - Russian nouns +0.1;
  - English verbs, Russian verbs, Finnish nouns, German adjectives ≥ 0.
- From the headword: ±0.05 everywhere.
- The check that the copies agree: before the change, run the instrumented probe
  (`PROBE_FIT`) at the item's base under each policy and record the counts. After it,
  one function serves both, so add a test that the trainer's told form and the runtime's
  agree on a part's training pairs (it fails on the base: tunken).
- With one source cell supplied (`Known::forms`), these must be right:
  - tunken: pres_1_pl → pres_1_sg *tunke*;
  - landet: pres_2_pl → pres_2_sg *landest*;
  - ajoissa: ine_pl → ess_pl *ajoina*;
  - fishing: verb_prp → verb_past *fished*.

**Kill:**
- Spanish verbs with principal parts under +0.1 under both policies;
- any row losing beyond chance under the kept policy without a fix;
- any part's learn over +20%.

**Reach:** every part with relations: all 75, the heavy canary included.
**Byte line:** +0.5% per part.
**Cost:** about 3 h, plus two screens.

### Item 2. Greek preverbs by the runtime's spelling (one-source; 44 points)

**The copy:** the trainer proves a preverb by comparing a compound's form with its own
spelling of prefix + rest (`learn.rs:1721-1723`, `joining.join`). The runtime writes the
join through `Orthography::initial_only` (`orthography.rs:263-288`), which drops the
rest's breathing. So ὑπ+ἀκούω and συρ+ῥάπτω never prove, and no elided, aspirated or
συρ- preverb is learned.

**Witnesses** (inspect, dev): cells right from the headword:
- ὑπακούω 186 of 489;
- συρράπτω 180;
- διαρρέω 0 (102 with principal parts);

against ἐκδίδωμι 390 and ἀπατάω 366. The aorist *ὑπᾰ́κουσᾰ (right: ὑπήκουσᾰ) and
*ἐσῠ́ρρᾰπτον. Across 34 dev lemmas, 2,046 of 3,570 augmented forms are wrong.

**What, in two halves, screened apart:**
- **(i) The join through the runtime's spelling.** Every comparison of a compound's form
  with prefix + base goes through the runtime's written form. The trainer's own join
  spelling is deleted. The trainer's longest-prefix choice (`learn.rs:1760-1766`) becomes
  a call to `Prefixes::split`; the predicates agree today, so this part is exact.
- **(ii) The temporal augment counted:** the trainer's `augmented` map (`learn.rs:1653-1675`,
  not the runtime's `augmented` at `grammar.rs:1061`) counts a form whose first letter
  differs from the base's beyond the initial marks (syllabic augment, temporal augment,
  ρ-doubling), as its doc says.

ἡλόω and ὑστερέω are not compounds (ἧλος, ὕστερος). Their errors are the temporal
augment's, so they measure half (ii), not (i).

**Bet:**
- Greek verbs from the headword +0.6, with principal parts +0.4;
- the preverb-lemma group +2;
- about −50 KB (irregular.rs).

**Kill** (for each half separately, and for both together):
- Greek verbs under +0.2 from the headword, or the preverb group under +0.5;
- any Greek verb loss beyond chance with principal parts;
- a loss on non-compound lemmas over 100 forms (an ἀπατάω-type false split);
- any part outside Greek and Sanskrit verbs not byte-identical, or over 1 KB.

If Latin, Russian or Polish gain a prefix from half (ii), restrict the temporal-augment
count to vowel-initial heads.

**Reach:**
- every prefix-bearing part: Greek, Sanskrit, German and **Italian** verbs (Italian has
  `ri-`, `languages/italian/src/verb/prefixes.rs`);
- and, because the admission predicate changes, the prefix stage's output (admitted
  prefixes, stripped bases, augment owners) diffed across all 75 parts before any full
  screen. A part whose output changes is screened.

**Byte line:** the Greek verb crate must not grow.
**Cost:** about 3 h.

### Item 3. Bugs with witnesses (decided)

- **(a) Greek -εως counts short** (phonologist-claude-1, 29 points).
  - `Prosody`'s short-ending test folds the word's letters (ς → σ) but not the ending's
    (`stress.rs:274-276`; the config writes `εως`). So no -εως genitive is ever short.
  - Witnesses: κάθαρσις gen sg → *κᾰθᾰρσέως* (right: κᾰθᾰ́ρσεως), ὕπαρξις → *ὑπᾰρξέως*;
    0 of the dev tenth's 66 -εως forms right.
  - Fold the ending too. Add ς → σ to the stress test's orthography; that makes the
    existing test fail without the fix (the witness).
  - Bet: Greek nouns +0.35, proper nouns +0.2.
  - Kill: under 30 of the 50 dev -εως noun forms right (66 counts every Greek part).
  - Byte line: any Greek part over +2 KB.
- **(b) Harmony's stage order.**
  - A class letter is read before a later law fills it in: 158 built places in the dev
    learns (probe 1; the shipped crates, learned on every word, may differ slightly):
    - Finnish `body+IVn` → *bodyayn* (right: bodyaan), and every possessive 3 of a
      y-final loan;
    - Turkish `mazur gör+IrlEr` → *mazur görürlar*;
    - Turkish `metropol+imIz` → *metropolümiz*.
  - Order the harmony laws so a law reads letters the laws it depends on have written,
    or make the copy law read the filled letter. Generic: no language's letters named.
  - Bet: those witnesses right; Finnish nouns and Turkish verbs ≥ 0.
  - Kill: §3's bug-fix rule (no row loses beyond chance). Byte line: any part over 5 KB.
- **(c) A candidate printed above 100%.**
  - `inspect ancient-greek --pos verb --word ἐκδίδωμι`: "ἐκδῐ́δωμῐ (106%, listed)".
  - Find why a listed word's probability passes 1 (a sum over readings, a trust above 1,
    a listed reading added to a rule's), fix it, and add an assertion in the test that
    every candidate is at most 1.
  - Witness: that word. Bet: no candidate above 1 anywhere. Kill: §3's bug-fix rule.
- **(d) German stacked prefixes** (linguist-claude-2).
  - The runtime splits a prefix off, then splits the rest again (`grammar.rs:790-810`),
    and places each particle after the one before:
    - hinabklettern → *klettre ab hin*;
    - bevorzugen → *bege zu vor*;
    - about 260 dev forms, all wrong.
  - An inseparable prefix in front keeps every prefix in front. A separable prefix whose
    rest separates again writes the whole particle span after the verb as the lemma has
    it (hinab, durcheinander).
  - Generic; German is the one language with separable prefixes today.
  - Bet: German verbs +0.3 from the headword, +0.2 with principal parts.
  - Kill: under +0.1. Greek verbs must not change. Byte line: German verbs over +10 KB.

**Reach:**
- (a) Greek parts;
- (b) every part whose laws have an `agreements` block, derived from the crates, not this
  list. At HEAD:
  - Greek nouns and proper nouns;
  - English verbs;
  - Finnish adjectives, nouns, postpositions, pronouns, proper nouns and verbs;
  - German nouns;
  - Latin adverbs, nouns and verbs;
  - Polish adjectives and verbs;
  - Russian numerals;
  - Sanskrit adjectives and verbs;
  - Spanish nouns and verbs;
  - Turkish adjectives, nouns, proper nouns and verbs;
- (c) all;
- (d) the four prefix-bearing parts.
**Cost:** about 3 h together.

### Item 4. Truths held twice in the code (one-source; decided)

Each piece is its own commit, screened as it lands.

- **(a) A paradigm's word count, once** (engineer-claude-4).
  - The count is printed four times (`print.rs:429, 726, 835, 854`). Two copies mean
    different things: the paradigm sample's words (`learn.rs:197`) and the choice's empty
    rule (every word).
  - In Finnish nouns they disagree in 99 of 101 paradigms (Purkaminen 733 against 7,728),
    and the runtime's fallbacks read the sample's.
  - Delete `Paradigm::words`. The fallbacks read the choice's root rule, and the printed
    comments read one count.
  - Bet: Finnish nouns +0.00 to +0.02; every other part identical; about −15 KB.
  - Kill: a part other than Finnish nouns changes an answer that isn't a fallback word,
    or any part grows over 1 KB.
- **(b) `ROOT_TRUST` measured** (data-claude-4).
  - The runtime types 0.97, "244 of 250". Measured on PR 21's grammars ("letters roots
    write right"):

    | part | share |
    |---|---|
    | Latin nouns | 0.956 |
    | Latin adjectives | 0.954 |
    | Latin verbs | 0.933 |
    | Greek nouns | 0.879 |

  - The marks learner counts it on its held-out half and ships it in each part's `Marks`
    with its evidence line. The constant is deleted.
  - Bet: within ±0.05 top-1; the calibration error `try` prints within ±0.005.
  - Kill: a row loses more than 0.05, or any part grows over 2 KB.
- **(c) One tolerance function** (phonologist-codex-3).
  - `flexion::tolerates` (`choice.rs:126`, no exception allowed under two words) and
    `harmony::tolerated` (`harmony.rs:865`, under three) disagree at n = 2: in English,
    German, Spanish and Russian, 639 verdicts and 2,533 `regular` flags (764 and 3,091
    with Finnish and Turkish).
  - Keep `flexion::tolerates` and change its guard to n < 3 (no exception under three
    words: n/ln n ≥ n for n ≤ e). Delete `harmony::tolerated`.
  - Statuses change. Top-1 may move where trust differs by status (Russian unsettled
    0.996, Russian proper nouns' 0.736), so screen it.
  - Kill: a row loses beyond chance.
- **(d) Exact deletions** (each must leave crates and saved runs identical):
  - **the root window and the marks context** as runtime functions (`Marks::root_of`,
    `Marks::context`), read by `marks.rs` and `length.rs` (phonologist-claude-4,
    classicist-codex-1);
  - **one "is this the dictionary's form" test** in the runtime and one
    `eval::accepted(…)` for spell_out, misbuilt, outcomes and the end-to-end test
    (data-claude-3, classicist-codex-4). The accent stays strict;
  - **eval's sharpen copy** replaced by `flexion::sharpen` (ml-codex-1);
  - **the build marks** as one `pattern::MARKS` with `is_mark`, each "all but X" site
    saying why (inventory 3.3);
  - **the cache keys' file lists:**
    - `stage.rs:23` misses `lib.rs`, which sets HashMap order, and other files: derive
      it, or guard it with `saved.rs`'s test;
    - derive `saved.rs:48`'s list too;
  - **dated decision numbers** in comments (`learn.rs:2013`, `learn.rs:1262`): dated, or
    dropped.

**Reach:** (a) every part; (b) Latin and Greek; (c) every part; (d) every part.
**Gates:** (a), (b) and (c) move answers, so each is judged by its own kill line. (d) is
exact and judged by §3's exact rule.
**Cost:** about 3 h.

### Item 5. The pipeline's numbers and roles in the repo (one-source; decided)

- **(a) The root README is written by `xtask`.**
  - Its tables come from the crate builds' structured measurements, not from
    `~/.cache/claude-tmp/panel4-plan/roottable.py` and `splice.py`.
  - The bytes report (`panel4-plan/bytes.py`) and the paired-screen verdict
    (`panel4-plan/cmp.sh`'s loss/gain rule) become `xtask` commands.
  - Bytes are counted on the formatted files: the crate README's size table is counted
    before rustfmt today (Turkish verbs 9,040 lines against 10,569 on disk).
  - The README's prose carries no current number it doesn't read from a table:
    - current claims are generated or removed: Latin verbs 84.9% in full (85.45% now),
      `Приём` 37,690 (37,921 now), the renamed relations in the excerpts;
    - before/after history (Greek 74.8 / 78.0 / 68.0) stays, dated with its PR.
  - "What it costs" is regenerated with dates, or its per-call column measured again.
  - (Inventory K1.3, K2.2-K2.4, K3.5, K4.1-K4.3, K4.6, K4.7, K5.1, K5.4-K5.7.)
- **(b) Tenth roles in code.**
  - The dev tenth (2), the confirmation tenth (9), the README tenth (0) and the
    sharpness tenths are named in one place.
  - `sweep` gains `--fold 9` (or `--confirm`).
  - Fix the leak: `POOL = [2, 4, 6, 8]` with `dev_of(8) = 9` lets a non-quick pooled run
    fit its sharpness on tenth 9. A plan that reads a reserved tenth is an error.
  - The canary set is a named list in `xtask`.
  - (K5.2, K2.5, K5.5.)
- **(c) Census kinds named once** (data-claude-2).
  - One `name()` per kind: "class" everywhere ("paradigm" in saved runs today).
  - The README columns derive from `Kind::ALL`; `report.rs:227, 262-270` are deleted.
  - A new kind, *reading*, where the right form is another reading of the same paradigm
    (a prefix split, animacy, a second stem). In German verbs that is at least 575 of
    1,426 dev "class" forms, mostly separable-prefix readings.
  - Diagnostic only: no answer changes.
- **(d) The printer's exact savings:**
  - no indentation inside choice payloads: 443,668 B exact (engineer-codex-5);
  - `regular` as the paradigm choice's default, with the unsettled rules marked instead:
    199,270 B net (256,800 B of ` regular` less the ` unsettled` markers added;
    engineer-claude-3, revised).

  Printer-only, so the saved builds reprint without relearning. **Kill:** any token or
  answer differs.
- **(e) The generated `Cargo.toml`'s `include`** names `LICENSE-MIT` and `LICENSE-APACHE`,
  which no language crate has: write them, or name what exists (K3.6).
- **(f) The list of the twelve languages** comes from `flexion_configs::all()` wherever it
  is typed (K3.2).

**Bet:** answers identical. Crates about −0.64 MB. The root README reproduces today's
table values.
**Cost:** about 3.5 h.

### Item 6. One way to learn a part (one-source; decided, narrowed)

"Learn a part's grammar" is written eight times (inventory K1.2):
- the pipeline, `pipeline.rs:184-215`;
- try, `xtask/src/main.rs:535-545, 592-600, 646-666`, and its small parts, `:1056-1070`;
- inspect, `main.rs:1779-1790`;
- audit, `xtask/src/audit.rs:46-72`;
- bench, `xtask/src/bench.rs:30-50`;
- the yardsticks, `xtask/src/yardsticks.rs:232-242` (sigmorphon) and `:346-352` (nonce);
- the end-to-end test, `crates/flexion-configs/tests/end_to_end.rs:46-68, 166-178`.

Only the pipeline and try (both of its paths) add teacher words. Turkish adjectives learn
from the nouns' words (`turkish.rs:192`), so inspect, audit, bench, the yardsticks and
the end-to-end test learn a Turkish adjective grammar that isn't the one shipped or
scored.

**What:** one `learn_part` (corpus selection, teacher words, marks attached), called by
every one of them.

The deeper copy, the README's measurement against `try`'s (K1.1, 12-18 h), is not in
this PR. Log what the shared function makes possible.

**Bet:** shipped crates and saved runs identical. The Turkish adjective diagnostics change
to the shipped grammar's; log their numbers.
**Kill:** any shipped answer or byte differs.
**Cost:** about 2.5 h.

### Experiments (last; each screened in 30 minutes or less)

- **E1. The accent class read with the paradigm** (phonologist-claude-2).
  - Today the accent class rules read the word's letters alone
    (`crates/flexion/src/grammar.rs:1631-1643`).
  - Their context ends with the word's paradigm, as the marks rules' context ends with
    the part of speech (`marks.rs:106`). The trainer learns class rules on `key#paradigm`.
  - Witnesses:
    - νύμφη gen pl → *νῠ́μφων* (66%), with the right νῠμφῶν second (15%);
    - ἐπιφάνεια gen pl → *ἐπῐφᾰνείων*;
    - ἐλοιδόρουμεν, κῡμᾰ́νει.
  - Bet: Greek verbs +0.3 from the headword, nouns +0.15.
  - Kill: Greek verb accent errors not down 25% (576 → 430 dev), or any Greek row losing
    beyond chance.
  - Reach: every part with an accent model (the six Greek parts).
  - Byte line: +40 KB.
  - Cost: about 3 h.
- **E2. Vowel-initial reduplication as one copy law** (classicist-claude-2).
  - Greek verbs' laws state the augment's lengthening again and again in
    `languages/ancient-greek/src/verb/laws.rs:19-30`: law0, 1, 5, 8, 10, 15, 18 and 19,
    plus 12 and 20 (two other maps there, `ζ → ἐζ`, are not lengthening).
  - Front literals that differ only in the copied vowel pool into one law whose map is
    the augment's own.
  - Witnesses: *ὑεὑστέρηκα, *ἡεἡλόηκα (989 dev forms).
  - Bet: Greek verbs +0.4 from the headword, +0.3 with principal parts.
  - Kill: under +0.15; the 13 named lemmas' perfect cells not mostly right; or laws.rs
    not shorter.
  - Byte line: any part over +2 KB.
  - Reach: every part with copy (`%`, `pattern::COPY`) patterns: Greek verbs, Finnish
    adjectives and verbs, German verbs. Also Sanskrit verbs, for the augment laws (they
    have no `%`). (`°` is a build mark, `COPIED`, not the copy pattern.)
  - Cost: about 2.5 h.

### Open disagreements: measure each and log it

- **Item 1's policy:** (a) against (b), as above.
- **Two lanes or three:** performance (b).
- **`fit`'s early stop exactness:** performance (d)'s count.
- **Item 2's halves:** screened apart.
- **`ROOT_TRUST`:** the measured share per part against the constant.

### Not in this PR (the decision gives the reasons)

- **Kept, with measured reasons:**
  - the harmony reader as the tier search's statistic (0 disagreements in 8.0 M lawful
    places);
  - `Tables::pattern` (proved equal in PR 21);
  - the FNV copies (the held-out split must keep its hash).
- **Deferred:**
  - the README measurement against `try` (K1.1);
  - the laws' text parser in sound-law (2.1);
  - derived law-search filters (1.2);
  - Sanskrit athematic presents (measure on folds 2–8 first);
  - Latin names' marks;
  - tied words abstaining;
  - one-cell laws and stem-piece anchors (PR 21's 8(a) and 8(b) witnesses);
  - separation per prefix as a choice;
  - eligibility-conditioned rule scoring;
  - a feature-told form's status;
  - one listing decision by forms;
  - configs' shared helpers and `editors.rs`'s per-language knowledge.
- **Killed by the panel's probes:**
  - the longest ending decides the class;
  - one training compound makes a Finnish head;
  - fusing completion with scoring.
- **Retired:**
  - Russian names' animacy per number;
  - Latin roots' unanimity against 0.9 (with 2–9 voters, 0.9 is unanimity).

## 3. Judging changes: the tiered gates

1. **Step 0, the baseline.**
   - No extraction changes are planned. The dev baseline is the panel's
     `~/.cache/claude-tmp/panel5/dump/*.tsv` (PR 21's grammar, all 75 parts,
     `try --quick`), and tenth 9 is `base/t9-pr21/`.
   - Check that they exist. Copy them into `~/.cache/claude-tmp/panel5-plan/base/` first,
     since scratch isn't kept. If they are gone, rerun them with main's binary on two
     lanes: about 55 minutes and 75 minutes.
   - If you change what the configs read, re-extract and score the baseline again.
2. **The canary set** (light) and **the heavy canary.**
   - **Light, lane 1:** German nouns, Russian nouns, Latin verbs, Russian verbs, Spanish
     verbs, Turkish nouns, Latin adjectives.
   - **Light, lane 2:** Polish nouns, Italian verbs, Greek adjectives, Greek nouns,
     Finnish adjectives, English nouns, German verbs, English verbs, Latin nouns, French
     verbs, Sanskrit verbs, Latin proper nouns, Russian proper nouns, English proper
     nouns, Sanskrit numerals, Latin pronouns, Finnish pronouns.
   - **Heavy:** Finnish nouns (about 22 min), Finnish verbs (8), Finnish proper nouns (4),
     Turkish verbs (8), Greek verbs (7).

   Each part runs `try <lang> --pos <part> --quick --save <dir>/<lang>-<part>.tsv`, one
   directory per screen. Before performance (b), Finnish and Turkish never run together.
   After it, the measured scheduler decides.
3. **Screen each item as it lands:**
   - the light and heavy canary plus every part in the item's reach;
   - from the headword and with the principal parts given;
   - paired (`compare`) against the previous kept run.

   **Every screen reports bytes per part against the previous run, by script.** A part
   over the item's byte line is a loss. An exact item shows identical saved runs and
   identical crates instead.
4. **Keep or drop:**
   - **exact and performance:** kept if answers and bytes are identical and the time falls
     beyond noise;
   - **one-source:** kept if answers are identical, or every changed word is wrong before
     and right after (each a witness of the copies' disagreement), and learn time stays
     under its line and the 20% ceiling. If the deleted copy was the right one, fix the
     copy you keep, then delete. Never keep both;
   - **which gate governs:** an item that states its own bet and kill (items 1, 2, 3, 4(a)–(c),
     E1, E2) is judged by them and by "no row loses beyond chance". This rule's
     every-changed-word test governs only the pieces marked exact (performance, 4(d), 5,
     6), where any change at all is a failure;
   - **bug fix:** kept if no row loses beyond chance. A loss beyond chance is found with
     `inspect --parts --holdout`, then fixed or narrowed;
   - **experiment:** kept only if its target rows gain beyond chance and no row loses
     beyond chance by more than 0.1 points. Otherwise dropped, logged with its numbers.
5. **A change to a mechanism re-screens every part that has the mechanism,** from its
   first screen, not only the parts that motivated it. A late fix re-screens every part
   its code reaches, in both input modes.
6. **The learn-time ceiling.** No item may make any part's learn more than 20% slower than
   the run before it (after performance part 1, against its times), unless the
   log accepts the cost by name with the hours it adds.
7. **No full run while anything is unmeasured.**
8. **The integration set,** once every item is in.
   - All 75 parts, `try --quick`, against step 0's dev baseline, on two lanes (about an
     hour after performance part 2).
   - Read by a script that lists every row losing beyond chance and every part over its
     byte line, **for all twelve languages**. It isn't finished until every language is in
     that list.
   - Each loss goes to a subagent that returns its cause, then is fixed or its item
     dropped. Rerun only the parts it touched.
9. **Tenth 9 before the builds.**
   - Tenth 9 for every language on the integration binary, against `base/t9-pr21/`.
   - Every loss beyond chance, or of ten words or more, gets its words read.
   - The builds start only when tenth 9 is clean or every loss is accepted under the
     rules.
10. **The pre-launch checklist:**
    - zero gaps;
    - Sanskrit, French, English built and tested;
    - fmt and clippy clean;
    - a clean tree;
    - the binary copied to a versioned name, never overwritten while a run uses it;
    - bytes per crate predicted from the integration learns.
11. **The builds, once,** by `xtask build all` with performance (b)'s scheduler, longest
    first.
12. **After the builds,** a fix rebuilds only the crates it changes. A printer-only fix
    reprints from the saved builds in minutes.

**The README is written once,** by `xtask` (item 5), from the final build. Every number in
its prose is read from a table, by code.

## 4. What must stay true

- **One source of truth.**
  - Every fact and meaning is defined once.
  - The trainer predicts a form only through the runtime's or `sound_law`'s own code, on
    the representation the crate will have, marks included.
  - Lists that mirror code or data are derived.
  - Numbers in prose come from code in the repo.
  - A deleted copy is not kept as a fallback or a cross-check.
- **A crate is its grammar.** Named items: laws, stem slots, ending tables, paradigms,
  relations, heads, affixes, accent marks, phrases. Every rule carries its evidence. No
  probability tables, opaque ids or weights. Any classifier lives in the trainer only.
- **The end-to-end test** prints learned grammars, compiles them with `-D warnings` and
  compares every answer. It keeps passing and covers every new item, and it now asserts
  every candidate's probability is at most 1.
- **Every word has every form in every cell,** attested or not (analytic forms as
  phrases). No defectiveness; never the lemma as a stand-in. Rarity or doubt goes into
  status and reason only.
- **Answers and output.**
  - Calibrated candidates, with a status and a reason on every answer.
  - Non-test code never panics: clippy denies unwrap, expect, indexing and panic.
  - Nothing is computed before the first call.
  - Output is byte-identical for the same input, across hash seeds, toolchains, lane
    counts and build profiles.
- **Nothing language-specific in the trainer or runtime.** Per-language things live in the
  configs. If `sound-law` or another of the project's crates lacks a feature, add it
  there; never work around it.
- **Code.** Idiomatic, type-driven Rust; clippy clean with `-D warnings`; README examples
  pass. Generated crates read like a grammar.
- **Trainer size is free; generated crate size matters.**
- **Tests.** Few and meaningful. No golden, snapshot or parity tests. Each test needs a
  verified failure witness: break the code, see it fail, restore.

## 5. How you work

- **No `/tmp`.** Scratch goes in `~/.cache/claude-tmp/panel5-plan/`. Tests run with
  `TMPDIR` there.
- **Resource caps:**
  - **two lanes:** at most two trainer processes at once, each with at most 8 threads,
    each under `systemd-run --user --scope -q -p MemoryMax=16G -p MemorySwapMax=0`
    (builds `MemoryMax=20G`: a Finnish build peaks at 15.9 GB). The one exception is
    performance (b)'s validation of three lanes during the final rebuild, if its
    simulation favours them;
  - before performance (b) lands, Finnish and Turkish heavy jobs never run together;
  - `cargo` with `-j 4`, tests with `--test-threads 4`;
  - generated crates compile one at a time.
- **Approved up front, never ask:**
  - sweeps and integration sets;
  - tenth-9 runs;
  - single-language learns of any length;
  - profiling runs;
  - the rebuild of every crate.

  Before each long wait, tell the user what you're waiting on and for how long.
- **The job queue.** PR 21's two-lane queue is in `~/.cache/claude-tmp/panel4-plan/`
  (`worker.sh`, `enq.sh`), and panel 5's copy is in `~/.cache/claude-tmp/panel5/`
  (stopped: `q/stop` exists, both workers are gone).
  - To use it, copy both scripts into `~/.cache/claude-tmp/panel5-plan/`, change their
    hard-coded `P`/`Q` paths, create `q/{pending,running,done}`, and start `worker.sh 1`
    and `worker.sh 2` in the background.
  - Priorities are two digits (`05`, not `5`).
  - Never run a script that moves jobs between priorities.
  - After performance (b), queue through `xtask`'s scheduler where it fits.
- **Worktrees for parallel items.** Implementation subagents each work in their own
  worktree on their own branch, and say when their code is ready. Their trainer runs go
  through the same queue. Merge into the PR branch one item at a time, screening each.
  - Never `git checkout` over uncommitted work: commit or stash first.
  - Subagents that commit must commit as the git config's identity only (gsc
    <stephen.korzen@gmail.com>). No `-c user.*`, `GIT_AUTHOR_*` or `--author`. Tell them
    so.
- **The run-state file:** `~/.cache/claude-tmp/panel5-plan/state.md`. It holds:
  - binary hashes;
  - what is running, with PIDs;
  - every item's verdict so far;
  - which rows lost.

  Update it at each step; after a context compaction, read it first.
- **One background wait per stage, never stacked.** Each wait exits on its stage's own
  completion marker. Read the result the moment it lands. No foreground sleep loops.
- **Subagents** take investigations, bisects and "why did row X lose", and return the
  cause. They also take audits, read-only checks and data probes over the forms dumps.
  They never run memory-heavy work beside your two lanes.
- **Clean up.** Kill everything you start, by PID. Never `pkill -f` or `pgrep -f | kill` a
  pattern that appears in your own command line: it kills your own shell. Remove every
  worktree you made.
- **Progress log:** dated entries in `~/code/notes/projects/flexion/grammar-crates.md`,
  under a new section "PR 22: the fifth panel's plan" (use the PR's real number). Every
  item ends with a verdict line:
  - built, with numbers;
  - dropped, with numbers;
  - not built, with the reason.

  Nothing is silently skipped.
- **Talking to the user.** Plain language, before/after code for API changes, and
  copy-pasteable commands starting with `cd ~/code/flexion`. Status reports say what is
  done, what is running, what is blocked, and when it will finish.

## 6. Git

One PR from a branch off the latest `main`, with this prompt as its first commit. Run fmt
and clippy before pushing. Merge with `gh pr merge --merge` once CI is green, then check
out the latest `main`. No crates.io publishing, tags or releases.

## 7. Done means

- **Every item and experiment, and every performance spot,** ends with a verdict line in
  the log: built, dropped or not built, each with its numbers or reason. Every open
  disagreement has its measurement logged.
- **Every inventory entry** the decision assigns to this PR is deleted, or logged as kept
  with its measured reason.
- **No significant loss anywhere** (paired bootstrap) unless accepted under §3's rules.
- **Bytes** per crate and per file kind, before and after, by `xtask`.
- **Every crate** regenerated, compiling, with README examples passing. CI green, and
  CI's test job under 5 minutes.
- **READMEs** written once by `xtask` from the final build.
- **The wall time** of every stage, against the 12–13 hour budget and PR 21's.
- **Clean:** every process, worktree and queue worker the run started is gone. The run's
  local branches are deleted once merged; a dropped attempt keeps its commit hash in the
  log, not a branch. The scratch runs the next panel needs (tenth 9, the integration set)
  are listed by path in the log.

## 8. Final report

- **The PR link.**
- **A table of every item, experiment and performance spot:** its bet against the result,
  and its verdict.
- **The inventory:** every entry this PR touched, deleted or kept, and why.
- **Per language and part of speech,** before and after:
  - top-1 from the headword, typed in full, principal parts given;
  - the floor and grammar − floor;
  - the census;
  - irregular lines;
  - calibration;
  - Greek's compatible score.
- **Crate bytes.**
- **Performance,** before and after:
  - every stage's time and each part's learn;
  - the runtime's `bench`;
  - CI's test job;
  - the hot spots left.
- **How each open disagreement was settled.**
- **Run times per stage,** against the budget and PR 21's.
- **Verification:** how you verified it, and how the user can run it, with commands
  starting `cd ~/code/flexion`.
