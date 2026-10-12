# Prompt: stems, boundaries, and the fixes the references point to

You own one PR on `~/code/flexion` (branch from the latest `main`, at or after `cba8fba`,
"Merge pull request #11"). It makes the learned grammars more accurate, smaller, and closer
to how a linguist writes a grammar. You may change anything (runtime, trainer, configs,
`sound-law`, generated crates, tests, CI, APIs) if it serves those aims. The project is
pre-release: no backwards compatibility, no deprecation shims. Delete what a better design
replaces.

## 0. Where things stand

flexion learns each language's inflection from Kaikki/Wiktionary dumps and prints it as a
Rust crate that *is* the grammar. Read the root `README.md` first. Then read a generated
crate (`languages/ancient-greek/src/verb/`, `languages/russian/src/noun/`), the trainer
(`crates/flexion-train/src/`: `learn.rs`, `choose.rs`, `laws.rs`, `relations.rs`,
`spec.rs`, `print.rs`), the runtime (`crates/flexion/src/`) and `crates/sound-law`.
PR #11's prompt (`prompts/grammar-structure.md`) and its log
(`~/code/notes/projects/flexion/grammar-crates.md`) say what was built and what was
tried and dropped.

Held-out top-1 from the headword (crate READMEs have the full reports):

| | nouns | adjectives | verbs |
|---|---:|---:|---:|
| English | 98.60 | 96.53 | 97.87 |
| Russian (no stress) | 95.03 | 99.76 | 96.04 |
| Latin | 65.29 (93.67 in full) | 81.70 (99.24) | 70.53 (85.26) |
| Ancient Greek | 61.94 (78.80) | 59.07 (72.08) | 46.06 (53.64) |
| Sanskrit | 91.90 | 91.64 | 87.74 |

The model before PR #10 (a model, not a grammar) is still ahead where it matters:
Latin 68.91 / 87.22 / 75.18, Greek 57.82 / 63.66 / 54.26, Sanskrit 93.55 / 91.21 / 90.13.
Principal parts given: Greek verbs 57.88, Latin verbs 97.59. Unseen cells: Greek verbs 3.5%
of 340 forms. The exact SIGMORPHON 2017 floor beats Sanskrit verbs (90.01) and Russian
adjectives (99.78). Calibration error: Greek 0.120 / 0.100 / 0.065, Sanskrit verbs 0.085.

Cost: Ancient Greek is 11.9 MB of source (verb `paradigms.rs` 5.4 MB, `irregular.rs`
2.0 MB, `accent.rs` 1.6 MB, `relations.rs` 0.4 MB), 8 s, 1.1 GB to compile. Russian
3.1 MB. One call: Sanskrit 15 µs (4 µs before its n→ṇ law), Greek 28 µs. Trainer plus
runtime: 9,149 lines.

Tried and dropped before (don't repeat without a new idea; numbers in the log):
naive-Bayes beginning rules; prefixes for every language; principal-part systems;
restoring Russian `ё`; Greek contraction as a global law (one-syllable -έω verbs πλέω,
πνέω, ῥέω, χέω don't contract ε+ο: 1,439 of 3,317 forms broke); marking paradigm cells
`derived` (Greek verbs 46 → 43); lumping minor rule values; Wilson & Li pruning
(calibration error doubled: la noun 0.048 → 0.093); gender as a hard filter.

## 1. Read first

`~/code/notes/projects/flexion/after-pr11-ideas.md` is the result of rereading all 104
papers in `~/code/notes/projects/flexion/references/papers/` against the state above,
with page references and counts measured on the crates. It is the brief for this PR;
the four survey files next to `papers/` are the background (`references/README.md` indexes
them). Where the ideas file and a survey disagree, the ideas file is newer; where either
disagrees with a paper, the paper wins.

## 2. What to do

The aims say what must be true; how to get there is yours. The ideas file proposes
mechanisms; treat them as leads, not specifications. Measure each change on the held-out
words and keep it only where it pays (accuracy, calibration, or size at equal accuracy).
Say what each cost or gained.

**1. Bugs and data.**
- No cell answers the bare lemma as a stand-in, and no learning step reads a stand-in as
  evidence. Today `learn.rs` (~line 1137, `None => lemma.clone()`) writes the lemma
  pattern into cells where no word of the paradigm has a form and no relation tells it:
  56,486 of 131,031 printed Greek verb cells (1st-person dual, future subjunctive…), 9,612
  of 25,892 Latin. The words' own forms are untouched, but the filler is then treated as
  learned:
  - `laws.rs`: `neighbours()` compares tables cell by cell (shared filler looks alike,
    filler against a real pattern looks different), `stem_like()` counts filler as endings,
    and `options()` abandons a neighbour reading when a filler cell builds the lemma where
    the word has a form, so laws and merges are missed;
  - `inherit()` counts shared filler when choosing a parent;
  - the runtime's given-form check (`grammar.rs`, `inflections`) rejects a paradigm whose
    filler cell can't build a given principal part;
  - held-out words asked for those cells get the lemma (likely most of Greek's unseen-cell
    3.5%).
  First measure the effect (Greek and Latin verbs: laws, paradigms, top-1, principal parts
  given, unseen cells) with unlearned cells kept out of every comparison. Then make "this
  paradigm learned nothing here" a state the trainer and runtime both know, excluded from
  comparisons and checks, and answer those cells with a real form built by the grammar (a
  relation, an analogy across paradigms, a shared ending table), with a low status and its
  reason.
- The engine deliberately generates every cell for every word, including forms the
  dictionary marks as missing (`—`; skipped in `spec.rs` ~line 242 on purpose) or that are
  conventionally rejected: Russian `победить` has a 1sg, it is just rare. Keep it that way
  in every language: no defectiveness, no "no form" answers. Rarity or a gap mark may at
  most show in a candidate's status or reason, never remove the form.
- Greek periphrastic optative: Wiktionary prints two words and the extractor keeps the
  auxiliary (`εἶμεν`), 2.4% of held-out verb cells. Extract the right form.
- Inflected forms entered as headwords (`Γεγονετε`, `Τεθνηκατε`) don't train paradigms.
- Sanskrit back to about 4 µs/call: the engine (`sound-law/src/engine.rs`, `match_at`)
  evaluates the unbounded left context before testing the target, at every position. The
  law as learned is left-subsequential; run it as one pass, with the stem's state computed
  once at the `+` boundary.

**2. Greek under 5 MB of source and the tables halved everywhere, by stating things
once.**
- Accent patterns no word uses aren't printed (53 of 100 Greek verb patterns are empty).
  Accent is a default per cell plus overrides (an inherited accent dimension), not a full
  pattern per class: 935 of 29,328 cells differ from the per-cell majority.
- Cells identical in (nearly) every paradigm are stated once, with evidence and
  exceptions: Greek verbs 624 → about 305 cells (passive = middle outside the aorist and
  future); Russian adjectives 52 → about 17.
- A paradigm earns its place by description length, like a rule does; otherwise its words
  are "like P, plus these listed cells". 203 of 323 Greek verb paradigms have one word.
- Print only what differs (all-`_` rows are 0.97 MB; row labels 2.6 MB).
- A verb as a few stems, each with a shared ending table (Greek tense systems; Latin
  present / perfect / supine), so irregular verbs list stems, not forms, and a listed
  stem fixes every cell built on it. Learn the slots from the data (two cells are kept
  apart only where some words' stems differ there and the split recurs), and print named
  ending tables a reader recognizes.

**3. Operations at the start of the word.**
- Greek and Sanskrit preverbs as a boundary, learned from where the augment or
  reduplication lands inside the lemma (`ἐξ|ε|τέλουν`), not from every prefix. The simple
  verb inflects; laws at the boundary rejoin it (`ἐκ+ε → ἐξε`, `ἀπο+ε → ἀπε`). Evidence
  counts a verb family once, not once per compound.
- Reduplication as a copy element in patterns (the stem's first consonant + ε), with the
  laws that follow it (Grassmann's deaspiration). Extend `sound-law` or the pattern
  language where needed; don't work around it.
- Relations whose change can touch both edges of a form, so the augment and reduplication
  generalize to cells a paradigm's words lack.
- Greek accent placed before contraction, contraction after, by learned laws; contraction
  conditioned so πλέω and φιλέω both come out right (a condition on the root, or one flag
  per lemma, never a thousand listed forms).

**4. Relations chosen better.** Sources picked for held-out accuracy (Albright's
local bases, or a spanning tree on held-out cross-entropy), preferring sources on the same
stem; two sources where one isn't enough (Latin present participle from the infinitive and
the 1sg). Today's choices include the Greek perfect middle 1sg from the present imperative
2sg. Report unseen cells and principal-parts-given accuracy before and after.

**5. Vowel length and paradigm choice from the headword.** This is the largest gap
(Latin nouns 65 from the headword vs 94 typed in full). Restoration reads both sides of
the vowel and the word's start, can condition on the chosen paradigm and syllable count,
keeps several spellings as candidates scored jointly with the paradigm, and may cite a
dictionary relative ("ū as in cūra") where that pays. Paradigm choice can use a word-start
anchor and syllable count (Latin 3rd declension: monosyllables -r .21 vs -s .75). The
realistic bound is what the spelling carries (whole word predicts Latin noun stem length
71% vs 59% from the ending), not the typed-in-full number. Today `restore` returns one
spelling, so the right one is usually not among the candidates at all (Latin nouns top-3
is only 68.9 vs top-1 65.3).

**6. Russian structure.**
- Stress as its own learned dimension (accent classes learned from the stressed forms
  before they are stripped, chosen by prefix and root as well as ending), with the
  spelling it drives as laws (unstressed о → е after sibilants and ц, ё under stress).
  64 paradigms differ only by stress-driven spelling (ключ/товарищ, рвануть/тянуть).
  Stress need not be printed in answers; it must stop splitting paradigms.
- The accusative stated once (number, then animacy, then gender); animacy listed only
  where it changes a form (`animacy.rs` is 614 KB). Nouns that decline as adjectives refer
  to the adjective grammar. Deponents (Latin and Greek verbs with passive or middle form
  in active meaning) are a word feature inferred like gender, not separate paradigms; their
  other voice is still generated.

**7. Rules and calibration.**
- Prune rules by what the runtime reads: keep a rule only if it lowers the code length of
  its words against the smoothed chain the runtime uses (why Wilson & Li pruning failed is
  in the ideas file, §D).
- Back-off to the parent rule weighted by how many words and paradigms a rule covers, not
  a fixed 2; calibration fitted or reported per candidate class, not one power per part of
  speech.
- The Tolerance Principle on bounded vocabularies ranked by the existing commonness proxy
  (no token frequencies needed), reported as how often the verdict holds across them.

**8. Measurement.**
- All ten folds for the small sets (Greek, Latin, Sanskrit verbs), so margins shrink from
  about ±6 to about ±2.
- Reciprocal rank next to the existing top-1 and top-3 columns. Use the top-1/top-3 gap to
  aim the work: Latin nouns 65.3 / 68.9 and Greek verbs 46.1 / 52.3 mean the right form is
  mostly not among the candidates (generate more: spellings, augment, reduplication);
  Sanskrit nouns 91.9 / 98.7 and adjectives 91.6 / 98.7 mean it is there but ranked second
  (fix ranking, e.g. gender).
- A floor for unseen cells (a vote over relations), since the 2017 baseline scores ~0
  there.

## 3. What must stay true

- A crate is its grammar: named items, evidence on every rule, no probability tables or
  opaque ids, no in-memory twin. The end-to-end test that prints learned grammars, compiles
  them with `-D warnings` and compares every answer keeps passing and covers any new kind
  of item (copy elements, stems, boundaries, cells built by analogy).
- Every word has every form, attested or not (no gaps, no defectiveness); every answer has calibrated candidates with
  status and reason. No panics (clippy denies unwrap/expect/indexing/panic), no
  initialization, byte-identical output for the same input.
- Idiomatic, type-driven Rust; clippy clean with `-D warnings`; README examples pass.
- Nothing language-specific in the trainer or runtime: Greek augment, Russian stress and
  Latin length must come out of general mechanisms and configs.

## 4. How you work

- No `/tmp`; use `~/.cache/claude-tmp/`.
- Cap parallelism (`-j 4` for language crates, `--test-threads 4`, at most 8 training
  threads). Build generated crates one at a time under
  `systemd-run --user --scope -p MemoryMax=16G -p MemorySwapMax=0`. Nothing over about 10
  minutes, nothing using all cores or lots of RAM, without asking. `cargo xtask try <lang>
  --pos <part>` is the fast loop; `cargo xtask build <lang>` the full build. The ten-fold
  runs count: run them one language at a time, and ask before any that would exceed
  10 minutes.
- Kill everything you started when done. Never `pkill -f` a pattern from your own command
  line; use PIDs.
- Tests: few and meaningful, no golden, snapshot or parity tests, each guard with a
  verified failure witness (break the code, see it fail, restore).
- Progress log: dated entries in `~/code/notes/projects/flexion/grammar-crates.md`, with
  numbers and what was tried and dropped.
- Subagents are allowed; parallelize audits and measurement, not builds that compete for
  memory.
- Talk to the user in plain language, with before/after code for API changes and
  copy-pasteable commands starting with `cd ~/code/flexion`. Say what you're waiting on
  before long waits.

## 5. Git

One PR from a branch off the latest `main`. Run fmt and clippy before pushing. Merge with
`gh pr merge --merge` once CI is green, then check out the latest `main`. No crates.io
publishing, tags or releases.

## 6. Done means

- Every item in §2 made, or measured and dropped with its numbers.
- The five crates regenerated, compiling, README examples passing, CI green.
- No cell answers the bare lemma as a stand-in; every cell's form is built by the grammar.
- Held-out accuracy at least the §0 table everywhere within noise; Latin, Greek and
  Sanskrit at or above the pre-PR #10 model, or the reason they aren't, in numbers. Attested
  ≥ 99%. Calibration error not worse anywhere.
- Ancient Greek under 5 MB of source; Sanskrit back to about 4 µs/call.
- Trainer plus runtime no larger than 9,149 lines, with the code the new mechanisms replace
  deleted; smaller if you can.
- READMEs (root and crates) updated with real excerpts of the new structure (a stem table,
  a boundary law, a copy element, a cell built by analogy) and the new numbers.

## 7. Final report

The PR link. Per language and part of speech, before and after: top-1 from the headword, in
full, principal parts given, features given, seen and unseen cells, top-3 and reciprocal rank, floor,
attested, calibration, with ten-fold margins where run. Grammar size before and after
(paradigms, stems, rules, laws, listed words and forms, bytes, compile time and memory, µs
per call). Excerpts of the new items. Lines added and removed in trainer and runtime. What
was tried and dropped, with numbers. How to verify, as commands starting with
`cd ~/code/flexion`.
