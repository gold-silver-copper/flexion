# Prompt: grammar structure — inheritance, derived cells, sound laws, gender

You own one PR on `~/code/flexion` (branch from the latest `main`, at or after `97b338a`,
"Merge pull request #10"). It makes the learned grammars more accurate, smaller, and closer
to how a linguist writes a grammar. You may change anything (runtime, trainer, configs,
generated crates, tests, CI, APIs) if it serves those three aims. The project is
pre-release: no backwards compatibility, no deprecation shims. Delete what a better design
replaces.

## 0. Where things stand

flexion learns each language's inflection from Kaikki/Wiktionary dumps and prints it as a
Rust crate that *is* the grammar. Read the root `README.md` first: it explains how a
grammar is learned and shows excerpts. Then read a generated crate (`languages/russian`,
`languages/ancient-greek`) and the trainer (`crates/flexion-train/src/learn.rs`,
`systems.rs`, `choose.rs`, `print.rs`) and runtime (`crates/flexion/src/`).

Five languages are built and tested: English, Russian, Latin, Ancient Greek, Sanskrit.
Configs for Polish, German, Spanish, French and Italian stay in `crates/flexion-configs`
(`flexion_configs::every()`), unbuilt; don't build or ship them in this PR.

Current held-out top-1 (from the headword; the crate READMEs have full reports):

| | nouns | adjectives | verbs |
|---|---:|---:|---:|
| English | 98.60 | 96.59 | 97.87 |
| Russian (no stress) | 94.96 | 99.76 | 96.00 |
| Latin | 65.24 (93.67 in full) | 81.70 (99.24) | 70.33 (85.37) |
| Ancient Greek | 61.88 (78.74) | 58.78 (71.76) | 46.11 (53.57) |
| Sanskrit | 91.58 | 90.21 | 87.61 |

Old-model baseline for comparison: Latin 68.91 / 87.22 / 75.18; Greek 57.82 / 63.66 /
54.26; Sanskrit 93.55 / 91.21 / 90.13. Attested accuracy is 100% everywhere; keep it ≥ 99%.

Cost per crate (clean debug build): English 2.9 MB source, 10 s, 0.9 GB peak; Russian
2.6 MB, 6 s, 0.7 GB; Latin 2.4 MB, 7 s, 1.3 GB; Sanskrit 1.3 MB, 2 s, 0.4 GB; **Ancient
Greek 21.3 MB, 29 s, 11 GB**: its verb paradigm tables (329 paradigms × 548 cells) and
listed irregular forms.

Known causes of loss: Latin and Greek vowel length a headword doesn't write (lexical stem
lengths); Greek augment after a preverb (`περιέφερον`) and reduplication (`βέβλεφα`);
Sanskrit gender (`कुल` is neuter: `कुलम्`) and the n→ṇ rule (`तर्काणाम्`).

Tried and dropped in PR #10 (don't repeat without a new idea): naive-Bayes beginning
rules (−0.6 pt German verbs: beginnings and endings aren't independent); prefixes for
every language (−2.5 pt Russian nouns; now only where a dictionary writes a prefix apart);
choosing principal-part systems when "nearly as good and smaller" (lost Russian's animacy
referral; now only when more accurate); restoring Russian `ё` (adjectives below the floor).

## 1. Read first

`~/code/notes/projects/flexion/references/`: four notes files, each entry checked against
the full paper (PDFs in `papers/`). The README there indexes them. Most relevant:

- `paradigm-theory.md`: Network Morphology (default inheritance, overrides, referral;
  #7–9, the hierarchy is a graph with shared nodes, not a tree), implicative entropy and
  which cell predicts which (#1–4, Cotterell et al.'s spanning tree), principal parts
  (#5), stem spaces (#12), syncretism and its direction (#13; Russian acc = gen only in the
  animate plural and the singular of animate masculine o-stems).
- `rule-induction.md`: Albright & Hayes minimal generalization and confidence (#1, #6:
  generality is a starting rank, not a tie-break; one pseudo-exception per rule), Yang's
  Tolerance Principle (#7: on a frequency core of a few hundred to ~2,000 words, compared
  as a real number), "no productive rule" per context (#8), Wilson & Li pruning (#14),
  paradigm choice is where errors come from (#13).
- `phonology-learning.md`: rule learners need an exceptions list (none tolerate
  exceptions); the Sanskrit n→ṇ rule is not a local rewrite (it needs a tier: the last
  relevant letter, skipping others); description length breaks on gaps; Ellis et al.'s
  rule grammar (copy indices for reduplication).
- `paradigm-learning.md`: the exact SIGMORPHON 2017 baseline (from its code); unseen
  cells cost ~50 points, unseen lemmas < 2; Greek accent references.

## 2. What to do

Seven changes. The aims say what must be true; how to get there is yours. Measure each on
the held-out tenth and keep it only where it pays (accuracy, or size at equal accuracy).
Say what each cost or gained.

**1. Paradigms in an inheritance hierarchy.** Learn a hierarchy (Network Morphology):
shared patterns stated once on a parent, each paradigm stating only what differs, with
sharing across branches where the data has it. Print it so a reader sees "like X, except
…" with the exceptions named and counted. The aim: Greek and Latin verb tables shrink by a
large factor (target: Ancient Greek under 5 MB of source and under 4 GB peak to compile),
and every crate reads like a grammar book's declension chapter.

**2. One mechanism for "this form comes from that form".** Referrals, principal-part
systems, prefixes, compounds and fallback cells are one idea: a form is another form (of
the same word, or of another word) changed somehow, sometimes under a condition (a
feature, a prefix). Replace the separate mechanisms with one learned, typed relation, and
delete the code it replaces. Use it to fill cells a word's dictionary entry lacks from
the cell that best predicts them (implicative structure; Cotterell's spanning tree),
instead of borrowing from a similar paradigm. Report accuracy on seen and unseen cells
separately.

**3. Stem changes as sound laws.** Learn the changes that split otherwise identical
paradigms (Russian fleeting vowel `-ок/-ка`, Latin `rēx/rēg-`, Greek augment and
reduplication, Sanskrit n→ṇ) as named rules in `sound-law`, and merge the paradigms they
split. Extend `sound-law` where it lacks something (a tier rule for n→ṇ; copying for
reduplication) rather than working around it. A rule needs its evidence (words it
applies to, exceptions), and words it gets wrong stay listed.

**4. Gender (and other inherent features) chosen and used.** Learn each part of speech's
inherent features (gender; already animacy for Russian) as choices with evidence, let a
word's feature narrow its paradigm, and let the caller give it
(`Noun::new("कुल").gender(Gender::Neuter)`). Measure the gain when the feature is given
and when it isn't.

**5. Leaner, truer rules.** A rule lists its main outcomes and lumps the rest; prune
rules that say what a broader rule already says (Wilson & Li); apply the Tolerance
Principle on a frequency-ranked core, with "no productive rule" per context; name
paradigms after common words (frequency, not sense counts; no vulgar exemplars where a
common neutral word follows the paradigm).

**6. Irregular words as stems.** List a word's odd stem or principal part, not every
form it produces, wherever that reproduces the dictionary's forms; keep full forms only
for what no stem explains.

**7. Measurement.** Make the floor the exact SIGMORPHON 2017 baseline (rules keyed to the
cell, prefix and suffix rules separate) and report seen vs unseen cells.

## 3. What must stay true

- A crate is its grammar: named items, evidence on every rule, no probability tables or
  opaque ids, no in-memory twin (the trainer builds the values the crate holds). The
  end-to-end test that prints learned grammars, compiles them with `-D warnings` and
  compares every answer must keep passing, and cover whatever new kinds of item you add.
- Every word has every form; every answer has calibrated candidates with status and
  reason. No panics (clippy denies unwrap/expect/indexing/panic), no initialization,
  byte-identical output for the same input (the determinism test stays).
- Idiomatic, type-driven Rust; clippy clean with `-D warnings`; README examples pass.

## 4. How you work

- No `/tmp`; use `~/.cache/claude-tmp/`.
- Cap parallelism (`-j 4` for language crates, `--test-threads 4`, at most 8 training
  threads). Compile and build generated crates one at a time under
  `systemd-run --user --scope -p MemoryMax=16G -p MemorySwapMax=0`. Nothing over about 10
  minutes, nothing using all cores or lots of RAM, without asking. `cargo xtask try <lang>
  --pos <part>` is the fast loop; full builds are `cargo xtask build <lang>`.
- Kill everything you started when done. Never `pkill -f` a pattern from your own command
  line; use PIDs.
- Tests: few and meaningful, no golden or snapshot tests, each guard with a verified
  failure witness (break the code, see it fail, restore).
- Progress log: `~/code/notes/projects/flexion/grammar-crates.md`, dated entries with
  numbers and what was tried and dropped.
- Talk to the user in plain language, with before/after code for API changes and
  copy-pasteable commands starting with `cd ~/code/flexion`. Say what you're waiting on
  before long waits.

## 5. Git

One PR from a branch off the latest `main`. Run fmt and clippy before pushing. Merge with
`gh pr merge --merge` once CI is green, then check out the latest `main`. No crates.io
publishing, tags or releases.

## 6. Done means

- All seven changes made, or a change measured and dropped with its numbers.
- The five crates regenerated, compiling, README examples passing, CI green.
- Held-out accuracy at least as good as the table in §0 for every language and part of
  speech within noise, and better where the causes in §0 were addressed; attested ≥ 99%.
- Ancient Greek within the size and memory target in change 1, or the reason it isn't.
- Trainer plus runtime smaller in lines than at `97b338a`, with the mechanisms change 2
  replaces deleted.
- READMEs (root and crates) updated with real excerpts of the new structure and the new
  numbers.

## 7. Final report

The PR link; per language and part of speech, accuracy before and after (headword, in
full, principal parts given, seen vs unseen cells, floor, attested, calibration);
grammar size before and after (paradigms, rules, sound laws, listed words, irregular
forms, lines and bytes of source, compile time and memory); excerpts of a paradigm with
its parent, a sound law with its evidence, and a derived cell; the lines of code removed
and added in trainer and runtime; what was tried and dropped, with numbers; how to verify,
as commands starting with `cd ~/code/flexion`.
