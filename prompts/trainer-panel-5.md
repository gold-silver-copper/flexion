# Prompt: the fifth panel — find what to do next, and write the prompt that does it

This run analyses and decides; it does not implement. You convene a fifth panel of
specialists on `~/code/flexion`, as it is after PR 21 (`main` at or after `98699c9`).
The panel reads the trainer, the runtime and the generated crates, and makes bets on
what to change, backed by evidence. You chair: you gather the evidence first, check
claims, run the probes the panel asks for, and decide a ranked plan.

At the end you deliver three things:
1. **The panel's record** in the notes (§3).
2. **The implementation prompt,** `~/code/flexion/prompts/trainer-panel-5-plan.md` (§5).
   A separate agent run carries it out in one PR. Write it so that run needs nothing
   else.
3. **A full report** to the owner of what the panel found and what you recommend (§9).

**This panel's theme: one source of truth.** Every fact and every meaning in the
project should be defined once, and everything else should read it. Where there are two
copies, they drift apart, and the drift is a bug no score shows until a confirmation
run finds it. The panel's first job is to find every place where flexion keeps two
sources of one truth, and to plan deleting the second wherever possible. It still
looks for errors and bytes, but a bet that removes a second copy ranks ahead of a
new mechanism.

The case that started this, PR 21's *spinting* bug:
- **How it works:** the trainer decides whether a candidate harmony law is right with
  its own letter-reader. That reader is `reads` and `Class::value` in
  `crates/flexion-train/src/laws/harmony.rs:139,149`. The crate ships the same law as
  a `sound_law` pattern, written by `stated()` in `crates/flexion-train/src/laws.rs`
  and run by the runtime.
- **The bug:** the runtime's forms carry the build's boundary marks (`unpin+Ting`). The
  trainer's reader saw plain words. So the trainer counted zero errors, and the shipped
  law wrote *unpinting*.
- **The fix (a528551):** it patched the shipped pattern to skip the marks. The two
  copies agree again, but there are still two.

**The kinds of duplicated truth.** This is the old meaning of "don't repeat yourself":
every piece of knowledge has one authoritative copy. It does not mean "no similar-looking
code". In flexion it takes six forms. Each kind below has the cases already found (read
from the code at `98699c9`, line numbers checked) and how that kind is removed. Bare
file names are under `crates/flexion-train/src/`. The panel's inventory (§2) starts
from these and finds the rest.

1. **A second implementation of a meaning:** the trainer predicts what the shipped
   grammar will do with its own code instead of running the shipped code.
   - The harmony reader above: `reads`/`Class::value`, `laws/harmony.rs:139,149`.
   - `Change::applies` (`laws.rs:306`), "a quick test before running" each candidate
     law. `try_law` (`laws.rs:1921`) compiles the real program but asks `applies`
     first. Where `applies` says no but the program would fire, the law is neither
     credited nor blamed there, and nothing checks the two agree.
   - Two answers to "is this the dictionary's form?": `same()` (`laws.rs:3027`, build
     marks stripped) and `eval::same_form` (`eval.rs:317`, letters and stress). They
     may differ for a reason; if so, the reason is stated where both are used.

   *Removed by:* deleting the second implementation and running the real code. Turn the
   candidate into its program once, then run it over all the candidate's words.
2. **One thing represented two ways.** The trainer's search builds forms without the
   build's boundary and copy marks; the shipped crate's forms carry them. Every law
   judged on the first and run on the second can drift, as *spinting* did.

   *The right shape already exists:* a law is a `Change`, its notation and its compiled
   program, and both `notation()` and `compile()` derive from one `stated()`
   (`laws.rs:566`). One source, derived views.

   *Removed by:* judging on the representation the crate will have.
3. **A hand-kept copy of what the code or data already knows.**
   - The saved build's file list, `SOURCE` in `saved.rs:48`, mirrors the module tree. It
     missed `marks.rs`; a test now catches a missing file, but the list could be
     derived.
   - `COSTS` in `xtask/src/lanes.rs:16` hardcodes each language's run minutes and peak
     memory, and is already stale. It says Finnish builds in 66 min and Turkish in 20;
     PR 21 measured 68–74 and 33. The scheduler that reads it orders the lanes.

   *Removed by:* deriving it. Runs record their own times and memory, and the scheduler
   reads the latest.
4. **Numbers written twice.**
   - The root README's prose was typed from its tables in PR 21 ("352 listed forms",
     "59.04% → 6.33%", the size paragraph). It's right today and wrong after the next
     rebuild.
   - Its "What it costs" table shows per-call times "last measured on the grammars of
     PR #14", beside PR 21's sizes.

   *Removed by:* generating the numbers, and wording the prose so it needs none it can't
   read from a table. A measurement that isn't rerun is dated, or dropped.
5. **Tooling outside the repo doing the repo's job.**
   - The root README's tables are written by `roottable.py` and `splice.py` in
     `~/.cache/claude-tmp/panel4-plan/`, while `xtask` writes the crate READMEs. Two
     writers of one document. In PR 21 the scripts merged two tables into one.
   - `bytes.py` and `cmp.sh` produce the bytes and screen reports the pipeline's
     decisions rest on.

   *Removed by:* moving them into `xtask`. Scheduling glue may stay a script.
6. **One policy or one kind of knowledge in several places.** Unverified; the panel
   checks before acting.
   - Three agreement thresholds of 0.9: `ROOT_SHARE` (`marks.rs:119`), `TWIN_SHARE`
     (`laws.rs:1233`) and `HEAD_SHARE` (`learn.rs:3033`). If they are one policy ("most
     of the evidence agrees"), they are one constant. If not, each says why it differs.
   - `xtask/src/editors.rs` reads per-language Wiktionary templates (`la-conj`,
     `grc-conj`). Per-language knowledge is meant to live in `crates/flexion-configs`,
     so this may be its second home.

   *Removed by:* one named constant, or one home for the knowledge.

**The fourth panel's process mostly worked; its implementation run still ran long.**
Panel 4 made every proposal a bet with a kill criterion, gathered evidence before round
1, kept a ledger of past attempts and ran probes between rounds. Its plan's top bets paid
on the rows they named. Turkish verbs gained +2.16 on tenth 9, the confirmation tenth
(defined in §0). Latin nouns gained +0.89 and the crates shrank 1.46 MB. Every item
ended with a verdict. But the implementation took about 21.5 hours against a 12-hour
budget. §0's "How PR 21 went" says where the time went. One of its three late
findings (*spinting*) and its README table bug were drift between two copies of one
truth. This panel keeps panel 4's process and changes it in these ways:
- **one source of truth first:** an inventory of every truth held twice (§2), a kind of
  bet for deleting a copy (§3), and those bets ahead of new mechanisms;
- **smaller plans:** at most six items plus two experiments;
- **screens that reach every part an item's code touches,** the slow Finnish and Turkish
  parts included;
- **bytes judged at every screen,** not found at the end;
- **a final pipeline that can't be restarted by a late finding;**
- **a dedicated performance pass** over the whole library: the trainer, the runtime,
  `sound-law`, `xtask` and the tests. It is profiled, not guessed. Changes that don't
  touch what is learned (test settings, scheduling) land first; code-level speed-ups land
  after the fixes, on the corrected code (lead 8).

**What this run may change:**
- the notes;
- the new prompt file;
- scratch files under `~/.cache/claude-tmp/panel5/`.

It may not change the trainer, runtime, configs or crates, make commits or open a PR.
The implementation run commits the new prompt as its first commit. Short measurements
are allowed under the caps in §7.

**Never ask the owner anything.** Make every decision yourself, on evidence. That
includes linguistic rulings: research them and decide. Nothing in the plan may say
"the owner's call" or "ask the owner".

## 0. Where things stand

**This section is a snapshot,** taken on 2026-10-18 at `98699c9`. Its tables, sizes and
times are copies of the README, the notes and the run logs, and copies go stale. Before
round 1, check out the latest `main`. If anything merged since `98699c9`:
- regenerate the results table and sizes from the README and `bytes.py`;
- read the log for what changed;
- correct this section's facts in the context file the members read.

Where the snapshot and the repo disagree, the repo wins.

flexion learns each language's inflection from Kaikki/Wiktionary dumps. It prints each
language as a Rust crate that *is* the grammar: paradigms, ending rules, sound laws, stem
slots, relations, irregular lines, all readable. Twelve languages ship, each with every
part of speech it inflects. No cell of any part is ever empty.

**Four panels came before.** Their records are in `~/code/notes/projects/flexion/`:
- `trainer-panel-2026-10-14.md` (PR 16);
- `trainer-panel-2-2026-10-15*.md` (PR 18);
- `trainer-panel-3-2026-10-15*.md` (PR 19);
- `trainer-panel-4-2026-10-16*.md` (PR 21): decision, dossier, minutes, evidence, ledger
  and report;
- `trainer-panel-4-plan-report-2026-10-18.md`: PR 21's final report, with every item's
  bet against its result.

The log of every measurement, including what was tried and dropped, is
`~/code/notes/projects/flexion/grammar-crates.md`. Read the PR 21 section ("PR 21: the
fourth panel's plan") closely, then PR 19's.

**Terms used below:**
- **dev tenth:** tenth 2, scored by `cargo xtask try`. Decisions are made on it.
- **tenth 9:** a tenth no decision looks at, run once per PR to confirm.
- **README tenth:** tenth 0. Each build scores it, and the READMEs report it.
- **floor:** the CoNLL–SIGMORPHON 2017 non-neural baseline, learned from the same words.
- **paired comparison:** `cargo xtask compare a b`, a bootstrap over words, giving a 95%
  interval and the words better and worse.
- **listed word:** a word the grammar names with its class.
- **irregular line:** a listed stem or form the rules don't give.
- **canary set:** the fixed set of fast parts every item is screened on.

**What PR 21 did** (numbers are top-1 from the headword, paired against main):
- **Item 1, the saved build.** The pipeline saves what it learned. Its key covers the
  data, the config and the trainer, runtime and `sound-law` source, but not the printer.
  A printer-only change reprints in minutes. Any trainer change still relearns every
  crate.
- **Item 2, one index and one table read** (exact): −0.58 MB, Greek verb tables 31%
  faster, every answer identical. The cell index is arithmetic over groups (the
  `flexion::relation!` macro).
- **Item 3, scores that say what they measure:**
  - the census kinds are now class, marks, accent, stem, ending, variant, homograph and
    gap. "Marks" counts only marks the language's orthography writes, so Finnish
    harmony errors are no longer "length";
  - closed classes and parts with fewer than 30 held-out words headline "reproduced N of
    N", beside a new word's score, each word left out of training in turn;
  - Greek has a second score, *compatible with stated length*;
  - each crate README has a table of its shipped grammar against the scored one.
- **Item 4, laws read the right letters.** A harmony class of vowels reads a tier of
  vowels (plus the marks between words). A copy of consonants reads the letter just
  before it.
  - Turkish verbs: irregular lines 59.04% → 6.33%, listed forms 32,526 → 352, +2.16 on
    tenth 9.
  - English verbs +0.16 on tenth 9 (`quizzes`, `foxproofs`).
- **Item 5, the Greek augment as one law** per front: Greek verbs +1.30 on the dev tenth,
  +1.61 with the principal parts.
- **Item 6, Latin root vowel lengths learned from every form** of every training word (90%
  agreement, two words at least): Latin nouns +0.89 on tenth 9. Adjectives gained +0.17,
  short of the +0.8 bet.
- **Item 7, Finnish compound heads for harmony:** nouns +0.17 on tenth 9. A last member
  is a head only where it fixes more training words than it breaks.
- **Item 8(c):** a closed class's listed words keep the dictionary's other spellings
  (Italian `il|lo|l'`).
- **Item 8, the French data repairs:** four adjectives gain their feminines, and `bien`'s
  superlative is `le mieux`.
- **E2, tied words vote by evidence:** German nouns +0.13, Polish nouns +0.14 on tenth 9.
- **E3, the typed Greek accent holds past a head:** Greek nouns +0.38 on the dev tenth.
- **Greek bare α, ι, υ in the listing judge:** a form compatible with the stated length
  counts as the dictionary's. The accent stays strict. −141 KB, strict top-1 identical.
- **Crates:** 15.97 → 14.51 MB.

**Dropped in PR 21, with numbers.** Each is in the log; don't revive one without a new
idea.
- **8(a), stem pieces anchored on a vowel:** Spanish verbs +0.10, but Finnish verbs −2.93
  on tenth 9 (their paradigms 102 → 198).
- **8(b), one-cell sound laws:** German nouns −0.17 and Turkish nouns −1.11 with the
  principal parts.
- **E1, Greek preverbs as one unit:** −0.10 on preverb lemmas against a +0.5 bet.
- **Item 4's other shapes:**
  - every consonant class tried first as an adjacent copy: Finnish verbs −1.02;
  - no blockers at all: the Turkish tier was unchanged;
  - "a class of mostly vowels": Finnish names −1.92;
  - harmony learned from last members only: Finnish nouns −0.21.
- **E2's wider votes:** Russian verbs −0.17. Item 7's heads that aren't words: English
  nouns −0.06.
- **8(c) for open classes:** +2.1 MB in Greek verbs, no answer changed.

**Not built in PR 21, or left open.** This panel settles each one: put it in the plan
with a probe behind it, or retire it with a reason.
- **The witnesses 8(a) and 8(b) were for, still wrong:**
  - Spanish `reasentar` → *reasento* (`reasiento`), `sarmentar` (`sarmiento`);
  - Polish `dziki`'s comparative;
  - Spanish `-ísimo` keeping a final vowel the superlative drops (`prudente`).
- **E1's witness:** `συρράπτω` → *ἐσύρραπτον*.
- **Latin roots, unanimity against 90% agreement:** not measured.
- **Deferred by panel 4:**
  - Sanskrit athematic presents, augment maps, the weak perfect and preverb junctions
    (probe P3: 12% of Sanskrit verb forms are built by no paradigm);
  - Russian names' animacy, per word (probe B2: 68% of the floor-right, grammar-wrong
    forms are accusatives of the wrong animacy);
  - Latin names' marks (probe B1: their stem marks are right 48% of the time), to be
    measured again after item 6;
  - kept prefixes for Polish and Russian (Russian вы-);
  - taking back listed forms a word's own line tells, and unlisting Greek accent classes.
    Their exactness checks compared first candidates only;
  - bootstrap by held-out key.
- **Finnish front `-nen` adjectives:** 8% still break harmony (`-mielinen` compounds whose
  last member isn't a word, loans like `geometrinen`).
- **Logged, not chased:**
  - Finnish proper nouns, −0.09 with the principal parts on tenth 9 (9 words);
  - the README tenth's dips: Latin verbs −0.62, German proper nouns −0.40, Sanskrit verbs
    −0.25. On tenth 9 the same rows are +0.46, 0.00 and +0.48.

**Results on the README tenth,** scored by PR 21's final build. The root README has the
full tables. Columns:
- **top-1:** from the headword. For a closed class or a part under 30 held-out words, the
  "reproduced" headline.
- **in full:** the lemma typed with every mark.
- **principal parts:** the dictionary's principal parts given.
- **floor**, and **− floor:** grammar minus floor, paired, with its 95% interval.
- **irregular:** the share of training words with an irregular line.
- **wrong forms:** the count, then the census as % of them, by item 3's kinds:
  - *class:* the right form was among the candidates, but another paradigm was chosen;
  - *marks:* the same letters once the orthography's marks are off;
  - *accent:* the right letters, with the accent elsewhere;
  - *stem* and *ending:* where the form goes wrong;
  - *variant:* another form the dictionary lists in the cell;
  - *homograph:* a word typed like another, whose answer is the other's.
- **by block:** top-1 averaged over the part's blocks of cells.

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

**Size, the generated crates only.** The trainer and runtime may be as large as they
need. The crates total 14.51 MB: 11.83 MB for the parts that shipped before PR 19, plus
2.68 MB for PR 19's new parts.

| crate | MB | main before PR 21 |
|---|---:|---:|
| ancient-greek | 3.35 | 3.50 |
| finnish | 2.11 | 2.26 |
| russian | 1.96 | 1.98 |
| latin | 1.50 | 1.43 |
| german | 1.23 | 1.25 |
| polish | 1.19 | 1.19 |
| turkish | 0.89 | 1.99 |
| sanskrit | 0.65 | 0.70 |
| spanish | 0.48 | 0.49 |
| english | 0.46 | 0.49 |
| italian | 0.42 | 0.43 |
| french | 0.26 | 0.27 |

- **By file kind:**
  - `rules.rs` 3.41 MB, the largest and untouched by PR 21;
  - `paradigms.rs` 2.96, `relations.rs` 2.21, `irregular.rs` 1.97 (2.84 before PR 21);
  - `stems.rs` 0.89, `mod.rs` 0.52, `marks.rs` 0.49, `spellings.rs` 0.44, `animacy.rs`
    0.43, `accent.rs` 0.37;
  - the rest under 0.25 each.
- **Greek verbs** are most of Greek: paradigms 0.6 MB (284 conjugations, 489 cells
  each), irregular forms 0.9 MB (25,869 forms), stems 0.23 MB, accent 0.18 MB.
- **The script:** `~/.cache/claude-tmp/panel4-plan/bytes.py` measures all of this, against
  main's numbers.

**How PR 21 went** (times from `~/.cache/claude-tmp/panel4-plan/state.md`, `fin/times`,
`fin-1/times` and the log):
- **Wall time** about 21.5 h (2026-10-17 06:40 to 2026-10-18 04:40), against a 12-hour
  budget. PR 19 took 18.6 h.
  - Items and screens: about 10 h;
  - the integration sweep and its fixes: 2.5 h;
  - the final pipeline: about 9 h, because it ran three times.
- **Two lanes worked.**
  - The canary set: 8 minutes.
  - The integration sweep, ten languages: 1 h 15 (the pooled sweep took 3.2 h one at a
    time in PR 19).
  - All twelve builds: about 1 h 40. Finnish, at 68–74 min, then Turkish, at 33, is the
    critical path, since they never run together.
  - Tenth 9: 76–78 min of runs.
- **Item agents in worktrees worked.** Seven items went to subagents, each in its own
  worktree, and their trainer runs went through one queue. Merging them cost little.
- **The final pipeline restarted three times, each on a late finding:**
  1. Item 8(a) cost Finnish verbs −2.93 on tenth 9. The integration sweep had already
     shown it (−3.07), but Finnish landed last and was never read. Finnish verbs, a slow
     part, weren't in item 8's screen.
  2. Tenth 9 found *spinting*. A sound law reading the letter just before it saw the
     build's boundary mark, not the stem's `n`. The trainer had judged the law on plain
     forms, so it reported no breaks: the trainer/runtime mismatch class again (PR 18's
     recurring bug).
  3. The bytes, measured only at the end, had grown 2.1 MB. Item 8(c) spelled out open
     classes' alternatives, and Greek verbs weren't in its screen.
- **Tenth 9 changed two decisions.** In PRs 16–19 it changed one in three. It runs beside
  the builds, so each finding cost a full rebuild.
- **Item 4 was re-screened seven times.** Each reshaping of which letters form a class
  moved a different language: German verbs, Finnish verbs, Spanish adjectives, Finnish
  names, Finnish nouns. Only the canary set plus a few named parts were screened each
  time.
- **Process slips:**
  - the job queue sorted priority `9` after `22`;
  - a helper script promoted the build jobs early;
  - a `git checkout` wiped an uncommitted edit;
  - wait loops were stacked, 21 shells at one point;
  - a stage's results sat unread for hours;
  - the README table script merged two tables.
- **The job-queue scripts** are in `~/.cache/claude-tmp/panel4-plan/`:
  - `worker.sh`, `enq.sh`: two lanes, priority by name, a lock for heavy jobs;
  - `screen.sh`, `cmp.sh`: paired compares of two run directories, flagging losses;
  - `bytes.py`, `roottable.py`, `splice.py`: bytes, and the README's tables by script;
  - `final.sh`.

  Their scheduling (lanes, locks, priorities) may stay a scratch script. But what the
  pipeline's results depend on doesn't belong outside the repo: the bytes report, the
  root README's tables and the paired-screen summary. `xtask` already writes the crate
  READMEs and runs `compare`. Moving those pieces there is a one-source bet (lead 1).

**How decisions are judged** (the tools):
- **`cargo xtask try <lang> --pos <part> --quick --save <file>`:** scores the dev tenth.
- **`cargo xtask compare a b`:** the paired bootstrap, with a row per word group and the
  words better and worse.
- **`sweep <dir> --pool`:** every language, compared as two directories.
- **`--fold 9`:** tenth 9. **`--final`:** the README tenth.
- **`cargo xtask inspect <lang> --pos <part> --word <w> --parts --holdout`:** one word's
  table, scored the way `try` scores it.
- **`try --diagnose`:** class-choice diagnostics.
- **`cargo xtask bench <lang> --pos <part>`:** time per call and per table.

**Leads.** These are what the numbers show, not a plan. Each needs checking before anyone
builds on it.
1. **Two sources of one truth.** The six kinds and their known cases are in the
   introduction; don't copy them here. What isn't known yet is where else they occur.
   Places not yet read with this question:
   - the trainer's stems, class choice, accent and marks searches (`stems.rs`,
     `choose.rs`, `marks.rs`, `learn.rs`): every place a candidate is judged by
     anything but the runtime's `grammar.table` or a compiled law. Scoring and
     `misbuilt` (`learn.rs:3967`) already go through `grammar.table`;
   - the printer (`print.rs`) against the runtime's macros: does it know a format the
     runtime also knows;
   - the configs, for one general rule written per language;
   - `xtask`, CI and the end-to-end test, for lists of languages, parts or tenths kept
     by hand;
   - tests that re-implement what they test instead of asking the code.

   The trainer's shortcuts exist for speed: the search tries many candidates. A bet that
   deletes one says what it costs in learn time, and how it keeps the cost down.
2. **Class choice is where the wrong forms are,** outside Latin and Greek. Item 3's census
   moved the Finnish harmony errors out of "length": Finnish nouns are now 57% class and
   28% ending.
   - The other main rows: Turkish verbs 94%, Russian proper nouns 88%, Italian nouns 85%,
     Sanskrit adjectives 83%, French adjectives 81%, English adjectives 79%, Polish
     adjectives 78%, French verbs and Finnish adjectives 75%, Russian verbs 75%, English
     verbs 74%, German nouns 67%.
   - Rows below the floor beyond chance: German nouns −1.02, Russian verbs −0.91, French
     verbs −0.59, English verbs −0.56, Spanish verbs −0.30, Finnish nouns −0.22, Polish
     adjectives −0.05. Also Russian proper nouns −2.83, Finnish pronouns −3.21 and Sanskrit
     numerals −28.79.
   - Many ways to choose better were tried and dropped (the ledger). E2's narrow vote paid
     (+0.13 and +0.14), and so did item 7's validated heads. Both read evidence the words
     already have, rather than adding a model.
3. **Marks in Latin and Greek.**
   - Latin nouns: 71.83 from the headword against 92.93 in full (marks are 76% of wrong
     forms); adjectives 92% marks; proper nouns 60.13, 83% marks.
   - Greek nouns: 58% marks; Greek proper nouns: 60.28, 74% marks.
   - Item 6's roots vote only where two training words share a root. A root no other word
     shares stays a guess.
4. **Greek verbs:**
   - 68.30 from the headword, 81.95 with the principal parts;
   - 44.33% of training verbs need an irregular line, 25,869 irregular forms (0.9 MB);
   - wrong forms: marks 42%, class 20%, stem 19%, ending 14%;
   - E1's preverb unit was dropped.
5. **Sanskrit verbs:** 89.61, 23.63% irregular, wrong forms 56% ending and 22% stem; by
   block 78.66. Panel 4 deferred the athematic stems with probe P3 behind them.
6. **Stems and endings in the Romance verbs:** Spanish verbs' wrong forms are 33% stem and
   23% ending, against the floor −0.30. 8(a)'s witnesses are still wrong.
7. **Size.** `rules.rs` (3.41 MB) is the largest file kind and no panel has audited it
   since PR 19. Greek is 23% of all bytes.
8. **Performance, across the whole library.** No panel has profiled flexion. Every run
   time in this prompt is a wall-clock total, and every speed-up so far (PR 19's
   threads, PR 21's saved build and table reads) was argued from source. The run times
   decide how many screens a run can afford, so a faster trainer pays back in every
   later item.
   - **Where the time is:**
     - the trainer's learn: Finnish's build takes about 70 minutes and is the pipeline's
       critical path, then Turkish 33, Russian 17, Greek 12;
     - `try` and `sweep`: the integration sweep took 1 h 15 on two lanes, tenth 9 about
       80 minutes of runs;
     - the runtime's calls and tables: the README's per-call times were last measured on
       PR #14's grammars. Only Greek verb tables were re-measured in PR 21 (33 ms a
       table, 79 µs a cell);
     - CI's test job, 15 minutes, almost all the end-to-end test and the crates'
       doctests;
     - the generated crates' compile time (1–6 s each, debug).
   - **What is known about the setup:**
     - the learner's threads are capped by a constant, `THREADS = 8`
       (`crates/flexion-train/src/par.rs:6`), and `par::map` may not nest;
     - `[profile.release]` sets only `opt-level = 3` (`Cargo.toml:35`): no LTO,
       default codegen units, no debug symbols for profiling;
     - only the sound-law stage and the whole saved build are cached
       (`stage.rs`, `saved.rs`). The saved build's key covers all trainer source, so
       any trainer change relearns every crate. PR 21's three final rebuilds were full
       learns.
   - **The tools:** `samply` is installed (`~/.cargo/bin/samply`). `cargo xtask bench`
     times the runtime, and the build logs print each stage's time.
   - **Possible bets,** for the profiles to confirm or kill:
     - hot loops in the learn (allocation, cloning strings, repeated parsing of the same
       forms, maps where an index would do);
     - a build profile for the trainer (LTO, codegen units);
     - a cache key per stage or per part, so a fix to one stage relearns only what reads
       it;
     - a faster `compare` and `sweep`;
     - a cheaper end-to-end test.
   - The pass is exact: the same crates, byte for byte, and the same saved runs. Cutting
     implementation time is a result here, not just build cost.
9. **Closed classes:** a new word's score, each word left out in turn, is Turkish
   pronouns 14.29%, Finnish pronouns 16.51%. They reproduce their own words 100%. Decide
   whether anything there is worth a bet, under the no-gaps rule (§6).

**Tried and dropped.** Don't revive these without a new idea and new evidence. The full
list, with numbers, is the panel 4 ledger
(`trainer-panel-4-2026-10-16-ledger.md`) plus PR 21's drops above. In short:
- **Before PR 19:**
  - gender as a hard filter;
  - global or far-left vowel-length contexts;
  - restoring marks when typed in full;
  - Russian stress via uc-prosody;
  - naive-Bayes beginning rules;
  - prefixes for every language;
  - `derived` cells wholesale;
  - fixed or every-listed-word compound heads;
  - derived cells from another slot;
  - choice-rule pricing;
  - the Chu–Liu/Edmonds tree;
  - calibration refits (proposed three times);
  - a global description-length objective;
  - a German umlaut law;
  - hard-EM beyond ties;
  - a minimum paradigm size;
  - accent through sound laws;
  - variant-aware listing;
  - per-slot class choice;
  - more than 4 readings per table;
  - an ending classifier for class choice.
- **PR 19:**
  - Sanskrit tense pages read into their root's verb;
  - vowel-length rules that skip a letter;
  - an ε → η augment law;
  - `shares: noun` for proper nouns;
  - learned English pronouns;
  - hyphenated `First` compounds;
  - an incremental settle;
  - the "informative" filter on every word;
  - a runtime gap fallback;
  - a fallback accent on listed stems.
- **Panel 4, rejected:**
  - harmony as a learned ending rule (killed by probe H1);
  - smoothing sweeps;
  - French vowel alternation as a stem operation;
  - Italian future spellings;
  - lemma-to-form overrides;
  - closed-class calibration by leaving each word out (it changes top-1).
- **PR 21:** 8(a), 8(b), E1, and item 4's, E2's, item 7's and 8(c)'s other shapes, as above.

## 1. Read first

Read these yourself before convening the panel. You chair, so you must be able to check
claims:
- PR 21's final report and the PR 21 log section;
- the fourth panel's decision, minutes, ledger and report;
- `panel-process-review-2026-10-16.md`, and §0's "How PR 21 went";
- the root README;
- at least four generated crates: one of Latin or Greek, one of Russian or Polish,
  Finnish or Turkish verbs, and a Romance language;
- the trainer (`crates/flexion-train/src/`), the runtime (`crates/flexion/src/`) and
  `crates/sound-law/`. PR 21 added `saved.rs`, `marks.rs` and `report.rs` to the trainer.
  Item 4 rewrote `laws/harmony.rs`.

**Literature.** Use services that need no identity: OpenAlex, arXiv, ACL Anthology. Never
send the owner's email address or other personal details to any outside service, and
never put them in a member's prompt.

## 2. The evidence pack, before round 1

Build these in `~/.cache/claude-tmp/panel5/`, under the caps in §7. Panel 4's scripts are
in `~/.cache/claude-tmp/panel4/`; reuse what still fits:
- `samples.sh`, `pick.py`;
- `evidence.py`, `dossier.py`, `minutes.py`, `tally.py`;
- the schemas;
- `codex1.sh`, `codex2.sh`.

PR 21's saved runs are the baseline. There is no need to rerun what they hold:
- tenth 9 per language: `~/.cache/claude-tmp/panel4-plan/fin-t9/`, with main's in
  `base/t9/`;
- the integration sweep: `int-sw/`, with main's in `base/sw/`.

They live in scratch, which isn't kept. First, check that they exist and were made by
`98699c9`'s trainer (the binary `xtask-final-228f7a9`; the merge changed no trainer
code). Then copy what this run reads into `~/.cache/claude-tmp/panel5/base/`. If any
are gone, or `main` has moved past `98699c9` in trainer code, rerun them on two lanes:
about 1 h 15 for the sweep and 80 min for tenth 9. That is approved.

Score PR 21's final grammar on the dev tenth once, two lanes, before round 1. This is the
baseline every later compare uses. On two lanes it is about 1 h 15 plus Finnish and
Turkish.

1. **The ledger,** extended. Copy panel 4's ledger and add every mechanism PR 21 built or
   dropped, from the PR 21 log section, one line each: what it was, the rows it moved
   with the numbers, its verdict and why. Members get it with the context file. Each
   proposal names its nearest entry and what is new.
2. **Answer samples.** As panel 4 did: for every language and part, about 20 tenth-9
   words chosen at random across the word groups, plus words with a wrong cell. Each
   word's table as `try` scores it, next to the dump's.
3. **The census, read.** For every main row (nouns, adjectives, verbs, proper nouns), 20
   wrong forms of each kind over 5% of the row: the form given, the form wanted, and for
   *class*, the class chosen and the right one. Take them from the saved runs' `error`
   column.
4. **Floor-right, grammar-wrong.** For the rows below the floor, the words where the
   floor is right and the grammar wrong, with both answers. Group them by what the floor
   knew that the rules didn't. Panel 4 did this for seven rows (its evidence file); redo
   it on PR 21's grammar and say what moved.
5. **The sources-of-truth inventory** (lead 1), first and most important. Two read-only
   subagents split it:
   - one reads the trainer, the runtime and `sound-law`;
   - the other reads `xtask`, the configs, the READMEs, CI, the tests and the scratch
     scripts the pipeline used.

   They start from the introduction's known cases: confirm each, correct it, or strike
   it. Then they add every other truth held twice, sorted by the six kinds. For each:
   - its kind (1–6);
   - the two copies, with file:line;
   - which one should be the source, and why;
   - what deleting the other costs: learn time, bytes, lines;
   - whether any test guards their agreement today;
   - a witness where they disagree now: a word whose answer differs, found with
     `inspect` or a one-off comparison under 10 minutes. "None found" is a result too.
     For kind 1, the cheapest witness check is one candidate law run both ways
     (the trainer's predicate and the compiled program) over a part's training
     words, counting the places they disagree.

   Members get the inventory. A bet that deletes a copy cites its line.
6. **Bytes per file kind per part,** from `bytes.py`. For `rules.rs`, what its lines are:
   - endings, rules or exceptions;
   - how many rules choose the commonest class anyway;
   - how many are never the deciding rule for a training word.
7. **Timings and profiles** (lead 8).
   - **Stage times** for every part, from PR 21's build logs
     (`~/.cache/claude-tmp/panel4-plan/fin/build3-*.log`) and the sweep logs. Above all
     Finnish: which stage of its 70 minutes is which.
   - **Profiles.** A release build with debug symbols, then `samply record` (saved,
     not opened in a browser) on:
     - Finnish nouns, Turkish verbs and Greek verbs, each a `try --quick` learn;
     - one full build of a mid-sized crate (German or Russian);
     - a `compare` of two saved sweeps;
     - `cargo xtask bench` for Greek and Turkish verbs.

     One profile per lane at a time, under §7's caps. For each, the ten functions with
     the most time (self and total), the stage they belong to, and what they do.
   - **Memory:** each profiled run's peak, from `/usr/bin/time -v`.
   - **CI:** the test job's time per test binary, from the last run's log
     (`gh run view --log`).
8. **The canary set, revisited.** Panel 4's set is in `prompts/trainer-panel-4-plan.md`
   §3. Time it again. Then add a **heavy canary**: the slow parts (Finnish nouns, verbs
   and proper nouns, Turkish verbs, Greek verbs) as a second set, run on the heavy lane
   while the light canary runs on the other. Say how long the two take together.

Members can't run training; this pack is how they see real output. Put it in the
context file.

## 3. The panel

### Members

Six specialities, each with **two members**: a Claude subagent and a Codex agent (GPT-6
Astra, normal tier, not fast). Both get the same role prompt but **split the material.**
Each reads half the speciality's crates closely and skims the other half.

| speciality | Claude member reads closely | Codex member reads closely |
|---|---|---|
| linguist (paradigm structure; class choice in the modern languages) | German, Russian, Polish nouns and verbs; lead 2 | Spanish, French, Italian, English; leads 2 and 6, 8(a)'s and 8(b)'s witnesses |
| classicist (Latin, Greek, Sanskrit, as the reference grammars present them) | Ancient Greek verbs: lead 4, E1's witness | Latin names and roots, Sanskrit verbs: leads 3 and 5 |
| phonologist (laws, harmony, gradation, accent, marks) | Latin and Greek `marks.rs`, `accent.rs`; lead 3 | Finnish and Turkish laws after item 4, the Finnish `-nen` adjectives, the Spanish `-ísimo` laws |
| machine-learning specialist (class choice, the floor, calibration) | the nouns below the floor (German, Finnish) and Russian names' animacy | the verbs below the floor (Russian, English, French, Spanish) |
| Rust engineer, with performance (generated code, run time, the pipeline) | the crates' code and bytes: `rules.rs`, `paradigms.rs`, `relations.rs`, Greek; lead 7; the trainer's search shortcuts and what using the compiled law instead would cost (lead 1) | the profiles and the trainer's run time, `par.rs`, `saved.rs`, `stage.rs`, `xtask`, the end-to-end test: lead 8; the hand-kept lists and the scripts outside the repo (lead 1) |
| data and evaluation specialist (census, tenths, closed classes, the dumps) | the sources-of-truth inventory (lead 1): trainer, runtime, `sound-law`; the end-to-end test's reach | the inventory's other half: `xtask`, configs, READMEs, CI and tests; closed classes, lead 9 |

Each member says, with file:line in `languages/<lang>/src/` or the trainer:
- what a linguist would write differently;
- what is redundant, oversized, wrong or missing;
- **where their material keeps two sources of one truth,** adding to the inventory, and
  which copy should go.

Run the Codex members read-only, with a JSON schema for the answer. Panel 4 used the
model `gpt-6-astra`. Check the model list first (`codex` help or config) and use the
current equivalent on the normal tier, not fast:

```
timeout 5400 codex exec -m gpt-6-astra -c service_tier=default -s read-only -C ~/code/flexion \
  --skip-git-repo-check --output-schema <schema.json> -o <out.json> - < <prompt.md>
```

Run the Claude members as subagents, in parallel, no more than about six at once. Every
member is **read-only**:
- no edits in the repo, no training, no builds, nothing over a minute;
- read code, crates, READMEs, notes, the evidence pack and papers;
- cite evidence as file:line or as numbers from the pack, the READMEs and the log;
- mark guesses "unverified".

Budget each member about 30 minutes.

### Every proposal is a bet

Use one JSON schema for every member's answer. Each proposal has:
- **kind:** one of
  - a *bug* with a witness: a word and cell that come out wrong, and why;
  - an *exact change*: bytes or time fall, answers identical;
  - a *one-source change*: a second copy of a truth deleted, and its readers made to
    read the first. It names its inventory entry and kind. Answers are identical, or
    every word that changes is a witness of the copies' disagreement and is right
    afterwards. It states its cost in learn time;
  - an *experiment*: a change in what is learned, with a predicted gain.
- **the rows it moves,** and the predicted paired change on each, from the headword and
  with the principal parts given;
- **its reach:** every part its code touches, the slow parts included. That is its
  screening set. A proposal that changes a mechanism (harmony, class choice, listing)
  names every part that has the mechanism.
- **the command that measures it;**
- **a kill criterion:** the result that drops it;
- **cost:** implementation hours, run minutes, and **predicted crate bytes per part,**
  with a byte kill line where bytes could grow;
- **the nearest past attempt** in the ledger, and what is new. A repeat with nothing new
  is out before the meeting;
- **evidence:** file:line, and words from the pack.

At most five proposals per member. An experiment with no predicted number is out.

### Process

1. **Context file.** One file all members read first:
   - the project and where things are;
   - this prompt's §0;
   - the ledger;
   - the evidence pack's index;
   - the owner's constraints (§6);
   - the read-only rules;
   - the bet format;
   - each member's material.
2. **Round 1, analyses.** Each member writes:
   - a diagnosis;
   - findings in their crates, with file:line;
   - up to five bets.

   Each member also asks for up to two **probes**: runs under about 10 minutes whose
   answer would confirm or kill one of their bets.
3. **Probes.** You run the probes asked for (merge duplicates, about 10 in all, two lanes,
   about an hour) and add the results to the dossier. A bet its probe killed is
   withdrawn.
4. **Dossier.** All analyses and probe results in one file.
5. **Round 2, the meeting.** Each member reads the whole dossier and returns:
   - **votes:** five points to their best bet across the panel, four to the next, down to
     one;
   - **objections,** each tied to a bet, each naming the measurement that would settle
     it;
   - **checks:** at least three factual claims by other members, verified against the
     code, true or false with file:line. The Codex members check the exactness claims
     and the reach claims first: does the bet's screening set really cover every part
     its code touches?
   - their own bets revised or withdrawn.

   No restating: an endorsement with nothing new is a vote.
6. **Your decision.**
   - **Check the facts yourself.** Re-check, read-only, every fact the plan rests on. You
     may run a short probe to settle a contested claim.
   - **Follow the votes.** Panels 3 and 4's top-voted bets paid; items kept against the
     votes lost. An item below the vote median needs a written reason, with evidence.
   - **Write the decision:**
     - *what the panel agrees on:* the state of the crates, and where the errors and the
       bytes are;
     - *the plan:* **at most six items**, plus **at most two experiments**, which run
       last and are screened in 30 minutes or less each. Each item carries its bet:
       kind, rows, predicted change, reach, command, kill criterion, bytes, cost;
     - *the inventory:* every truth held twice, and for each: deleted by which item, or
       kept, with the reason and its measured cost;
     - *the performance pass* (decided, outside the six-item cap): the hot spots the
       profiles show, ranked by minutes saved on the final pipeline and the screens.
       Each spot is an exact bet with a predicted saving, its stage and its command. The
       pass has a time box (the panel sets it, about 2–3 hours of the run) and a target:
       the Finnish build, the light and heavy canaries, and CI's test job, each in
       minutes. Runtime calls are in the pass too, judged by `bench`;
     - *the open items above:* each one in the plan or retired, with a reason;
     - *rejected or deferred bets,* with reasons and the ledger entries behind them;
     - *open disagreements,* each with the measurement that settles it.
   - **Rank** by evidence and by what the owner values, in this order:
     1. correct forms;
     2. readable, minimal crates;
     3. build cost and implementation time.

     A correct output nobody scores counts as much as a score. **A bet that deletes a
     second copy ranks ahead of an equal bet that adds a mechanism.** The plan takes
     every one-source bet the inventory and the votes support before any experiment.
     Where a copy stays, the decision says why, with its measured cost.
   - **Size the plan** so its implementation fits in about 12 hours of wall time, the
     final pipeline included. PR 21's plan of eight items and three experiments took
     21.5. Estimate each item's hours, including its re-screens. Add the final pipeline
     at about 3.5 hours: builds on two lanes, plus tenth 9. Cut from the bottom.
   - **Count the relearns.** The saved build's key covers all the trainer's source, so
     every trainer change relearns every part it screens. One-source changes to the
     trainer cost a full screen each, the heavy canary included (about 30–40 minutes
     on two lanes). Plan them to land together where they touch the same stage, and
     screen them once.
   - **A learn-time ceiling.** Finnish's build, about 70 minutes, is the pipeline's
     critical path. No item may make any part's learn more than 20% slower than the run
     before it (after the performance pass, against the faster times), measured at its
     screen. The exception is a cost the plan accepts by name, with the hours it adds
     to the pipeline. This holds above all for one-source changes, which replace fast
     shortcuts with the real code. Their bets name the performance-pass spot that pays
     for them, if any.

**The record**, in `~/code/notes/projects/flexion/`, named with the date of the run:
- `trainer-panel-5-<date>.md`: the decision;
- `-dossier.md`: all analyses and probes;
- `-minutes.md`: the meeting, with votes and checks;
- `-ledger.md`: the ledger;
- `-evidence.md`: the pack's index and findings.

Write them in plain language. Define each term once.

## 4. Members' rules, shared

- No gaps and no attestation thresholds. A cell's rarity is status and reason, never an
  empty cell or a removed one.
- Every part of speech a language inflects ships.
- Nothing language-specific in the trainer or runtime.
- No "the owner decides".

## 5. The implementation prompt

Write `~/code/flexion/prompts/trainer-panel-5-plan.md`. Model it on
`prompts/trainer-panel-4-plan.md`: same plain language, every term defined once. It must
contain:

- **Where things stand,** as you measured it: the results table, sizes per crate and per
  file kind, run times.
- **Read first:**
  - the panel's record and the ledger;
  - the code it cites, with file:line;
  - the literature behind each linguistic choice, with page or section numbers.
- **The items, in order.**
  - Order: items that change the extracted data first. Then the performance changes that
    don't touch what is learned (test settings, scheduling), so every later screen is
    cheaper. Then the bugs and the one-source changes, then experiments. The code-level
    speed-ups come last, on the corrected code: an exact speed-up of code a fix will
    rewrite only preserves the bug. Each speed-up is exact against the corrected output.
  - For each item:
    - what to change, and the evidence;
    - its kind and bet: the rows, the predicted change, the kill criterion;
    - its reach (screening set), the slow parts included;
    - its byte line;
    - its cost.
  - Decided items are marked decided.
- **Not in this PR:** what was rejected, deferred or retired, with reasons.
- **What must stay true** (§6).
- **How the run works.** Copy §7's rules. Add that the run never asks the owner anything,
  and that its long runs are approved up front:
  - sweeps;
  - tenth-9 runs;
  - single-language learns of any length;
  - the rebuild of every crate.
- **Judging changes:** the tiered gates, written out as follows.
  1. **Freeze the baseline first.** Make every extraction change, re-extract, and score
     `main`'s binary on the new data on the dev tenth and tenth 9. All later compares are
     against this. A later re-extraction means scoring the baseline again.
  2. **Screen each item as it lands,** on two lanes:
     - the light canary, the heavy canary and every part in the item's reach;
     - from the headword and with the principal parts given;
     - paired against the previous item's run.

     An exact item instead shows identical saved runs. **Every screen reports bytes per
     part** against the previous run, read by script; a part over the item's byte line
     is a loss.
  3. **Keep or drop,** by these rules:
     - **exact:** kept if answers are identical and bytes or time fall;
     - **performance:** kept if every crate it reaches is byte-identical, saved runs are
       identical, and the time falls beyond noise. Time is the median of three runs at
       the same thread count, with the other lane idle; peak memory may not rise above
       16 GB. A change that alters output is not a performance change: it is judged as
       what it is;
     - **bug fix:** kept if no row loses beyond chance. A loss beyond chance is found with
       `inspect --parts --holdout`, then fixed or narrowed;
     - **experiment:** kept only if its target rows gain beyond chance and no row loses
       beyond chance by more than 0.1 points. Otherwise dropped, logged with its numbers;
     - **a correct output nobody scores:** kept if no row loses beyond chance by more
       than 0.1 and its bytes are under its line;
     - **one-source:** kept if answers are identical, or every changed word is wrong
       before and right after (each one a witness the two copies disagreed), and learn
       time grows no more than its stated line and the 20% ceiling. If the right copy is
       the one deleted, fix the copy kept, then delete. Never keep both.
  4. **A change to a mechanism re-screens every part that has the mechanism,** from the
     first screen, not the parts that motivated it. After a late fix, re-screen every
     part the changed code reaches, in both input modes.
  5. **No full run while anything is unmeasured.**
  6. **One integration sweep** once every item is in, against the baseline. It is read by
     a script that lists every row losing beyond chance and every part over its byte
     line, **for all twelve languages**. It is not finished until that list has every
     language in it. Each loss goes to a subagent that returns the cause, then is fixed
     or its item dropped. Rerun only the languages it touched.
  7. **Tenth 9 before the builds.** Run tenth 9 on the integration binary. Every loss
     beyond chance, or of ten words or more, gets its words read. The builds start only
     when tenth 9 is clean or every loss is accepted under the rules. In PR 21, two
     findings there each cost a full rebuild.
  8. **The pre-launch checklist:**
     - zero gaps in the sweep log;
     - Sanskrit, French, English and one crate per new kind of part built and tested;
     - fmt and clippy clean;
     - a clean tree;
     - the binary copied to a versioned name, never overwritten while a run uses it;
     - bytes per crate predicted from the sweep's learns.
  9. **The builds, once, on two lanes,** longest first: Finnish then Turkish on one lane,
     the other ten on the other.
  10. **After the builds,** a fix rebuilds only the crates it changes.
- **The README is written once,** from the final build, by code in the repo (`xtask`),
  not by scratch scripts. Every number and claim
  in its prose is read from the tables or the census, with the column checked. The
  script takes only the table it names, and its output is diffed against the old README
  before committing.
- **Agent hygiene:**
  - a run-state file at `~/.cache/claude-tmp/panel5-plan/state.md`: binary hashes, what is
    running, what is decided, which rows lost;
  - **one** background wait per stage, never stacked. Each wait exits on the stage's own
    completion marker, and the result is read the moment it lands;
  - queue priorities are two-digit (`05`, not `5`);
  - investigations and bisects go to subagents that return the conclusion;
  - never `git checkout` over uncommitted work: commit or stash first;
  - commit everything before pushing.
- **Git.** One PR off the latest `main`, with this prompt as its first commit. fmt and
  clippy before pushing. `gh pr merge --merge` once CI is green, then check out the latest
  `main`. No publishing.
- **Done means:**
  - every item ends with a verdict line in the log: built, with numbers; dropped, with
    numbers; or not built, with the reason. Nothing is silently skipped;
  - no significant loss anywhere unless accepted under the rules above;
  - total crate bytes reported per crate and per file kind;
  - every crate regenerated and compiling, README examples passing, CI green;
  - READMEs re-scored once on the README tenth;
  - the wall time of each stage, against the 12-hour budget;
  - every process, worktree and queue worker the run started is gone;
  - the run's local branches are deleted once merged. A dropped attempt the log cites
    keeps its commit hash in the log, not a branch. Scratch runs the next panel needs
    as its baseline (tenth 9, the integration sweep) are listed by path in the log.
- **Final report:**
  - the PR link;
  - a table of every item: its bet against the result, and its verdict;
  - per language and part of speech, before and after: top-1, typed in full, principal
    parts, floor and grammar − floor, census, irregular lines, calibration, Greek's
    compatible score;
  - crate bytes;
  - how each open disagreement was settled;
  - run times per stage, against the budget and PR 21's;
  - the performance pass: each stage's time and each part's learn before and after,
    the runtime's `bench` before and after, CI's test job, and the hot spots left;
  - how it was verified, with commands starting `cd ~/code/flexion`.

Before you finish, read the prompt once as the agent that will run it. Every number has a
source, every file:line is checked, and nothing assumes this conversation.

## 6. What must stay true

The plan, and the prompt that carries it out, respect these:
- **A crate is its grammar.** Named items: laws, stem slots, ending tables, paradigms,
  relations, heads, affixes, accent marks, phrases. Every rule carries its evidence. No
  probability tables, opaque ids or weights. Any classifier lives in the trainer only.
- **The end-to-end test** prints learned grammars, compiles them with `-D warnings` and
  compares every answer. It keeps passing and covers every new item.
- **Every word has every form in every cell,** attested or not (analytic forms as
  phrases). No defectiveness; never the lemma as a stand-in. Rarity or doubt goes into
  status and reason only.
- **Answers and output.**
  - Calibrated candidates, with a status and a reason on every answer.
  - Non-test code never panics: clippy denies unwrap, expect, indexing and panic.
  - Nothing is computed before the first call.
  - Output is byte-identical for the same input, across hash seeds and toolchains.
- **Nothing language-specific in the trainer or runtime.** Per-language things live in the
  configs. If `sound-law` or another of the project's crates lacks a feature, add it
  there; never work around it.
- **Code.** Idiomatic, type-driven Rust; clippy clean with `-D warnings`; README examples
  pass. Generated crates read like a grammar.
- **One source of truth.** Every fact and meaning is defined once, and everything else
  reads it.
  - The trainer predicts a form only through the runtime's or `sound_law`'s own code,
    on the same representation the crate will have, marks included.
  - Lists that mirror code or data are derived from it.
  - Numbers in prose come from the tables, by code in the repo.
  - A deleted copy is deleted, not kept as a fallback or a cross-check.
- **Trainer size is free; generated crate size matters.**
- **Pre-release:** no backwards compatibility, no shims; delete what a better design
  replaces.
- **Tests.** Few and meaningful. No golden, snapshot or parity tests. Each test needs a
  verified failure witness.

## 7. How you work

- **No `/tmp`.** Scratch goes in `~/.cache/claude-tmp/panel5/` (this run) and
  `~/.cache/claude-tmp/panel5-plan/` (the implementation run).
- **Resource caps:**
  - **two lanes:** at most two trainer processes at once, each with at most 8 threads,
    each under `systemd-run --user --scope -q -p MemoryMax=16G -p MemorySwapMax=0`;
  - Finnish and Turkish heavy learns never run at the same time;
  - `cargo` with `-j 4`, tests with `--test-threads 4`;
  - no more than about six panel members at once.
- **Approved up front for this run:**
  - the panel's Codex and subagent runs;
  - the evidence pack, including the dev-tenth baseline of PR 21's grammar and the
    profiles (each no longer than the run it profiles, one per lane);
  - the probes;
  - `try --quick` and `inspect` runs to settle a contested claim.

  Nothing else that runs over about 10 minutes. Before a long wait, tell the user what
  you're waiting on and for how long.
- **Clean up.** Kill everything you start, by PID. Never `pkill -f` or `pgrep -f | kill` a
  pattern that appears in your own command line: it kills your own shell.
- **Talking to the user.** Plain language, no internal labels without saying what they
  mean. Status reports say what is done, what is running, what is blocked, and when it
  will finish.

## 8. Done means

- **The evidence pack** is built and indexed, the extended ledger included.
- **The panel's record** is written: decision, dossier with probes, minutes with votes
  and checks, ledger, evidence. Every fact the plan rests on has been re-checked by you.
- **The sources-of-truth inventory** is complete, and every entry is planned for
  deletion or kept with a measured reason.
- **Every open item** from §0 is in the plan or retired, with a reason.
- **The implementation prompt** is at `prompts/trainer-panel-5-plan.md`. It is
  self-contained, at most six items plus two experiments, sized to about 12 hours, with
  the tiered gates written in. It is not committed.
- **Nothing in the repo changed** but that file. No process you started is still running.

## 9. Final report

The report is the deliverable the owner reads. Give it in full in your final message, and
save a copy next to the panel's record. Say plainly what you found and what you recommend:
- **The sources-of-truth inventory:** every truth held twice, which copy the plan
  deletes, the witnesses where copies disagree today, and what stays two, and why.
- **What the evidence pack showed:**
  - the census read;
  - the floor-right words, and what moved since panel 4;
  - where the bytes of `rules.rs` go;
  - where Finnish's 70 minutes go, and what the profiles show across the library.
- **What the panel found:**
  - the state of the crates, read as grammars;
  - where the errors and the bytes are;
  - what is wrong, redundant or missing, with examples from the generated code.
- **Who sat,** what each pair covered, and the votes.
- **The probes,** and which bets they confirmed or killed.
- **The recommended plan,** ranked. For each item:
  - what, and why;
  - the bet: kind, rows, predicted change, reach, kill criterion, bytes;
  - the evidence;
  - the cost in hours;
  - how it will be judged.
- **The open items,** each planned or retired.
- **Rejected or deferred,** with reasons.
- **Open disagreements,** and the measurement that settles each.
- **Risks,** and what you would cut first if the run has to be cut short.
- **Paths:** the record, the ledger, the report, and the implementation prompt.
