# Prompt: the third panel — find what to do next, and write the prompt that does it

This run analyses and decides; it does not implement. You convene a third panel of
specialists on `~/code/flexion`, as it is after PR 18 (`main` at or after `9e8de97`).
The panel reads the trainer, the runtime and the generated crates, proposes and argues.
You chair, check its claims, and decide a ranked plan.

At the end you deliver three things:
1. **The panel's record** in the notes (§2).
2. **The implementation prompt,** `~/code/flexion/prompts/trainer-panel-3-plan.md` (§4).
   A separate agent run carries it out in one PR. Write it so that run needs nothing
   else.
3. **A full report** to the owner of what the panel found and what you recommend (§8).

**One thing is already decided, not up to the panel:** each language must ship every part
of speech it inflects. Turkish and Finnish have no verbs today, and other parts of speech
are missing too (§0). This run measures the gap (§3), and the panel plans *how* to close
it, not *whether*. The implementation prompt includes closing it.

**What this run may change:**
- the notes;
- the new prompt file;
- scratch files under `~/.cache/claude-tmp/panel3/`.

It may not change the trainer, runtime, configs or crates, make commits or open a PR.
The implementation run commits the new prompt as its first commit. Short measurements
are allowed under the caps in §6: `try --quick`, `inspect`, reading the dumps.

**Never ask the owner anything.** Make every decision yourself, on evidence. That includes
linguistic rulings: research them and decide.

## 0. Where things stand

flexion learns each language's inflection from Kaikki/Wiktionary dumps. It prints each
language as a Rust crate that *is* the grammar: paradigms, ending rules, sound laws, stem
slots, relations, irregular lines, all readable. Twelve languages ship.

**Two panels came before.** Their records are in `~/code/notes/projects/flexion/`:
- `trainer-panel-2026-10-14.md` and `.json`: the first panel. PR 16 carried out its plan.
- `trainer-panel-2-2026-10-15.md`, `-dossier.md`, `-minutes.md`: the second panel. PR 18
  carried out its plan.

**What PR 18 did:**
- It removed relation data nothing reads, so relations went from 6.20 to 1.52 MB.
- Sound laws are printed in `sound-law`'s notation and compiled by a macro (laws.rs 1.08
  → 0.16 MB).
- Paradigms state their stem slots through shared ending tables.
- Latin and Greek analytic forms are built as phrases (`amātus sum`, `λελυμένος ὦ`).
- The sound-law stage became about 5× faster.
- Finnish harmony now applies to every spelling of a cell.
- A list of Latin and Greek roots tells vowel length.
- The floor (the baseline the grammar is judged against) learns from the same words as
  the grammar.
- Scores are split by word group:
  - own entries;
  - form-of entries (participles, comparatives);
  - alt-of entries (spelling variants).
- Class-choice diagnostics were added (`try --diagnose`).
- Gains: Latin from the headword +2 to +3 points, Greek nouns and adjectives +0.4,
  Finnish nouns +0.24. The one accepted loss: Polish nouns −0.06 without the Middle
  Polish tables.

The log has every measurement, including what was tried and dropped: read the "PR 18"
section of `~/code/notes/projects/flexion/grammar-crates.md` closely, and skim PR 12–16.

**Results on the README tenth**, scored once at the end of PR 18. The full table, with
more columns, is in the root README. Columns:
- **top-1**: from the headword.
- **in full**: the lemma typed with every mark.
- **principal parts**: the dictionary's principal parts given.
- **− floor**: grammar minus floor, a paired interval on the same words.
- **irregular lines**: the share of training words that need an irregular line.

| language | part | top-1 | in full | principal parts | floor | − floor | irregular lines |
|---|---|---:|---:|---:|---:|---|---:|
| english | noun | 97.17 | 97.17 | – | 96.78 | +0.39 (+0.24…+0.55) | 0.11% |
| english | verb | 97.17 | 97.17 | 99.54 | 97.79 | −0.62 (−0.93…−0.33) | 1.01% |
| english | adjective | 95.35 | 95.35 | – | 92.53 | +2.82 (+2.39…+3.25) | 0.06% |
| english | adverb | 99.01 | 99.01 | – | 98.34 | +0.66 (+0.25…+1.16) | 0.11% |
| russian | noun | 94.64 | 94.64 | 95.81 | 94.72 | −0.08 (−0.52…+0.36) | 0.53% |
| russian | adjective | 99.75 | 99.75 | – | 99.76 | −0.01 (−0.12…+0.09) | 0.07% |
| russian | verb | 95.76 | 95.76 | 98.88 | 96.65 | −0.89 (−1.39…−0.40) | 0.75% |
| latin | noun | 70.07 | 92.93 | 98.70 | 59.09 | +10.98 (+8.20…+13.69) | 1.16% |
| latin | adjective | 87.26 | 99.23 | 99.48 | 71.87 | +15.40 (+13.68…+17.15) | 0.48% |
| latin | verb | 76.85 | 84.92 | 98.59 | 64.17 | +12.68 (+8.60…+16.87) | 4.69% |
| ancient-greek | noun | 71.90 | 88.36 | 95.17 | 51.28 | +24.36 (+20.33…+28.12) | 8.62% |
| ancient-greek | adjective | 77.29 | 90.48 | 97.88 | 52.46 | +25.92 (+21.48…+30.22) | 2.91% |
| ancient-greek | verb | 65.45 | 74.89 | 80.52 | 44.20 | +22.59 (+16.63…+28.50) | 52.39% |
| sanskrit | noun | 92.10 | 92.10 | – | 91.70 | +0.40 (−0.73…+1.48) | 1.57% |
| sanskrit | adjective | 91.91 | 91.91 | – | 89.95 | +1.96 (+0.32…+3.76) | 3.87% |
| sanskrit | verb | 89.86 | 89.86 | 90.58 | 89.93 | −0.07 (−2.32…+2.05) | 24.23% |
| polish | noun | 93.42 | 93.42 | 95.79 | 93.34 | +0.08 (−0.29…+0.44) | 1.81% |
| polish | adjective | 99.92 | 99.92 | – | 99.99 | −0.07 (−0.09…−0.04) | 0.36% |
| polish | verb | 97.14 | 97.14 | 98.24 | 97.62 | −0.48 (−0.99…+0.02) | 1.39% |
| german | noun | 93.63 | 93.63 | 99.79 | 94.77 | −1.15 (−1.46…−0.85) | 0.51% |
| german | adjective | 98.79 | 98.79 | 99.88 | 99.02 | −0.23 (−0.73…+0.26) | 0.11% |
| german | verb | 95.17 | 95.17 | 95.88 | 73.16 | +22.01 (+20.25…+23.80) | 5.12% |
| spanish | noun | 98.67 | 98.67 | – | 98.67 | +0.00 (−0.20…+0.20) | 0.11% |
| spanish | adjective | 98.50 | 98.50 | 99.67 | 98.60 | −0.10 (−0.35…+0.14) | 0.22% |
| spanish | verb | 98.97 | 98.97 | 99.73 | 99.28 | −0.31 (−0.60…−0.02) | 1.73% |
| french | noun | 97.59 | 97.59 | – | 97.03 | +0.56 (+0.23…+0.90) | 0.65% |
| french | adjective | 97.50 | 97.50 | 99.95 | 97.72 | −0.22 (−0.65…+0.24) | 0.28% |
| french | verb | 98.70 | 98.70 | 99.48 | 99.28 | −0.59 (−1.06…−0.22) | 0.96% |
| french | participle | 99.84 | 99.84 | – | 99.52 | +0.32 (−0.32…+0.96) | 0.03% |
| italian | noun | 95.82 | 95.82 | – | 95.09 | +0.74 (+0.31…+1.16) | 0.19% |
| italian | adjective | 98.46 | 98.46 | 99.78 | 98.46 | +0.00 (−0.28…+0.28) | 0.19% |
| italian | verb | 99.05 | 99.05 | 99.56 | 99.22 | −0.17 (−0.48…+0.13) | 1.77% |
| italian | participle | 100.00 | 100.00 | – | 100.00 | +0.00 | 0.02% |
| finnish | noun | 97.81 | 97.81 | 99.15 | 98.29 | −0.48 (−0.68…−0.29) | 1.34% |
| finnish | adjective | 97.45 | 97.45 | – | 97.41 | +0.04 (−0.62…+0.72) | 0.90% |
| turkish | noun | 96.51 | 96.51 | 99.52 | 93.27 | +3.23 (+2.30…+4.24) | 1.11% |

**Size: the generated crates only.** The trainer and runtime may be as large as they
need. The crates total 13.79 MB.

| crate | MB | | file kind | MB |
|---|---:|---|---|---:|
| ancient-greek | 3.71 | | rules.rs | 4.21 |
| russian | 2.34 | | paradigms.rs | 2.37 |
| german | 1.46 | | irregular.rs | 1.98 |
| latin | 1.24 | | relations.rs | 1.52 |
| polish | 1.24 | | stems.rs | 0.63 |
| finnish | 1.12 | | animacy.rs | 0.63 |
| sanskrit | 0.63 | | mod.rs | 0.55 |
| english | 0.62 | | accent.rs | 0.45 |
| spanish | 0.50 | | marks.rs | 0.32 |
| italian | 0.46 | | spellings.rs | 0.32 |
| french | 0.27 | | laws.rs | 0.16 |
| turkish | 0.19 | | | |

**Run times** (one at a time, under the caps in §6):
- **One crate (`cargo xtask build`):** Finnish 43 min, Greek 27, German 13, Russian 13,
  Latin 6, Polish 6; the rest 1–3 min; about 2 hours for all twelve.
- **`cargo xtask sweep <dir> --pool`:** the pooled development tenths for every language,
  about 2 hours.
- **Tenth 9 for all twelve:** about 50 minutes.
- **German nouns trained in full:** 196 s, of which the sound-law stage's greedy step is
  116 s.

**Coverage: parts of speech shipped, and what the dumps hold besides.** The right-hand
column is a rough count. It counts entries in the English Wiktionary dump
(`~/.cache/flexion/dumps/<Language>.jsonl`) with at least three forms outside the head
line and the table labels. It overcounts: alternative spellings, romanizations and English
"more X" comparatives count too. §3 measures it properly.

| language | ships | in the dump, not shipped (rough entries) |
|---|---|---|
| ancient-greek | noun, adjective, verb, pronoun, numeral | adverb 186, determiner (the article) 57, proper noun 4,191 |
| english | noun, adjective, verb, adverb | pronoun 108, numeral 54, proper noun 3,745 |
| finnish | noun, adjective | **verb 15,380**, adverb 1,455, postposition 165, pronoun 85, numeral 81, determiner 21, proper noun 9,507 |
| french | noun, adjective, verb, participle, determiner | pronoun 31, proper noun 191 |
| german | noun, adjective, verb, determiner | pronoun 102, adverb 64, numeral 23, article 12, proper noun 5,364 |
| italian | noun, adjective, verb, participle, determiner | pronoun 59, article 7 |
| latin | noun, adjective, verb, pronoun, numeral | adverb 579, determiner 51, proper noun 9,275 |
| polish | noun, adjective, verb, pronoun, numeral | adverb 196, proper noun 18,723 |
| russian | noun, adjective, verb, pronoun, numeral | adverb 407, determiner 29, proper noun 4,538 |
| sanskrit | noun, adjective, verb, pronoun | numeral 69, determiner 8, proper noun 654 |
| spanish | noun, adjective, verb, determiner | pronoun 56, numeral 9, proper noun 41 |
| turkish | noun, adjective | **verb 2,396**, pronoun 21, numeral 7, proper noun 1,850 |

**Why the verbs are missing:**
- **Turkish.** The config leaves verb tables out (`crates/flexion-configs/src/turkish.rs`,
  module doc). It says the dump shifts the rows' tags: `gelirim`, the first person, is
  tagged third.
- **Finnish.** No reason is written anywhere. The dump has about 15,000 verbs with
  tables.

**How decisions are judged:**
- **Comparing two versions:** `cargo xtask try <lang> --pos <part> --quick --save <file>`
  scores the development tenth (tenth 2); `cargo xtask compare a b` is a paired bootstrap
  on words, with a row per word group.
- **Every language at once:** `sweep <dir> --pool`, compared as two directories.
- **`--fold 9`:** an untouched tenth, used to confirm.
- **`--final`:** scores the README tenth. Score it once, at the end.
- **`cargo xtask inspect <lang> --pos <part> --word <w> --parts --holdout`:** one word's
  table, scored the way `try` scores it, with the principal parts given and the word held
  out.
- **`try --diagnose`:** the class-choice diagnostics.

**Leads the panel should weigh.** They are what PR 18 saw, not a plan:
- **Coverage.** The missing parts of speech above, Finnish and Turkish verbs first. Both
  are agglutinative, with large tables:
  - Finnish verbs have consonant gradation, like its nouns;
  - Turkish verbs have vowel harmony and stacked suffixes.

  These are what the crates' laws, stem slots and relations exist for.
- **The largest remaining errors:**
  - **Vowel length.** Latin and Greek from the headword against typed in full: Latin
    nouns 70.07 vs 92.93, Greek nouns 71.90 vs 88.36.
  - **Greek verbs** score 65.45, and 80.52 even with principal parts given. 52% of
    training verbs need an irregular line.
  - **Sanskrit verbs:** 24% need an irregular line.
- **Below the floor beyond chance:**
  - German nouns −1.15: two in three wrong plurals are the plural class chosen wrong (a
    loanword's `-s`, `-ismus` → `-ismen`), one in fourteen the umlaut.
  - Russian verbs −0.89.
  - English verbs −0.62.
  - French verbs −0.59.
  - Finnish nouns −0.48.
  - Spanish verbs −0.31.
  - Polish adjectives −0.07.

  The ending classifier chooses no better than the rules on these rows, and building
  tables from more than 4 paradigm readings changes top-1 by at most 0.02 (PR 18,
  item 10).
- **Size.** `rules.rs`, the paradigm-choice rules and listed words, is now the largest
  file kind (31%). Ancient Greek is 27% of all bytes.
- **One bug class turned up three times in PR 18:** the trainer judges something on one
  set of words, or by one path, while the runtime applies it more widely or by another
  path. Each one showed only on tenth 9 or with principal parts given, never on the dev
  tenth from the headword:
  - compound heads were judged only on words with irregular forms but used for every
    listed word;
  - a derived cell's given form fixed the wrong stem;
  - prefixes were learned from the auxiliary in analytic phrases.
- **Noise.** The README tenth disagrees with the development tenth by up to half a point
  on small rows. Latin verbs: −0.43 on the README tenth, +0.77 on the development tenth,
  +1.36 on tenth 9.

**Tried and dropped before.** Don't revive these without a new idea and new evidence; the
numbers are in the log and the earlier panel records:
- gender as a hard filter;
- a single global, interleaved or far-left vowel-length context (it costs Latin);
- restoring marks when typed in full;
- Russian stress via the uc-prosody branch;
- naive-Bayes beginning rules;
- prefixes for every language;
- marking cells `derived` wholesale;
- a fixed 3-letter compound head;
- choosing compound heads among every listed word (made Russian `ванный` a head, adjectives
  −0.16);
- derived cells from another stem slot's cell;
- choice-rule pricing (higher, or per letter);
- the Chu–Liu/Edmonds relation tree;
- a calibration refit;
- a global description-length objective;
- a German umlaut law;
- hard-EM reassignment beyond exact ties;
- a minimum paradigm size;
- accent run through the sound laws;
- variant-aware listing;
- a per-stem-slot class choice (Polish nouns: 87.6% of forms against 93.1% chosen whole);
- more than 4 paradigm readings per table.

## 1. Read first

Read these yourself before convening the panel:
- the root README;
- the "PR 18" log section;
- the second panel's decision file and minutes;
- at least four generated crates (one each of Latin or Greek, Russian or Polish, German or
  Finnish, and a Romance language);
- the trainer (`crates/flexion-train/src/`), the runtime (`crates/flexion/src/`) and
  `crates/sound-law/`.

You chair, so you must be able to check claims.

**Literature.** Use services that need no identity: OpenAlex, arXiv, ACL Anthology. Never
send the owner's email address or other personal details to any outside service, and
never put them in a member's prompt.

## 2. The panel

### Members

Seven specialities. Each has **two members**: a Claude subagent and a Codex agent (GPT-6
Astra, normal tier, not fast). Both get the same role prompt, and in round 1 each works
independently, without the other's answer. That makes 14 members.

| speciality | the generated crates they must read, as main material |
|---|---|
| linguist (morphology and paradigm structure) | German, Russian, Polish, Spanish, French, Italian, English |
| classicist (Latin, Ancient Greek and Sanskrit inflection, as the reference grammars and dictionaries present it) | Latin, Ancient Greek, Sanskrit |
| computational phonologist (sound laws, harmony, gradation, accent, vowel length) | Finnish, Turkish, Russian, Ancient Greek |
| typologist for the missing parts of speech (§3) | Finnish and Turkish nouns, and the dumps' Finnish and Turkish verb tables. Also every language's pronoun, numeral, determiner, adverb and proper-noun tables, sampled |
| machine-learning specialist | any two crates, to tie the choice rules and listed words back to `choose.rs` and `learn.rs` |
| Rust engineer, with performance (the generated code as Rust: readability, idiom, size, compile time; and the trainer's run time) | all twelve crates' `mod.rs`, `paradigms.rs`, `relations.rs` and `laws.rs`, skimmed. One large crate (Greek or Russian) read closely |
| data and evaluation specialist | the answer samples (below) against the dump's tables, every crate; and the README tables |

So at least eight members read the generated code as their main material: the linguist,
classicist, phonologist and typologist pairs. Every crate is read closely by at least one
Claude member and one Codex member. Each member says, with file:line in
`languages/<lang>/src/`:
- what a linguist would write differently;
- what is redundant, oversized, wrong or missing.

Run the Codex members read-only, with a JSON schema for the answer:

```
codex exec -m gpt-6-astra -c service_tier=default -s read-only -C ~/code/flexion \
  --skip-git-repo-check --output-schema <schema.json> -o <out.json> - < <prompt.md>
```

Run the Claude members as subagents. Run the members in parallel, no more than about six
at once. Every member is **read-only**:
- no edits in the repo, no training, no builds, nothing over a minute;
- read code, crates, READMEs, notes and papers;
- cite evidence as file:line or as numbers from the READMEs and the log;
- mark guesses "unverified".

Put the panel's scratch files in `~/.cache/claude-tmp/panel3/`.

### Process

1. **Answer samples.** Before round 1, prepare samples in `~/.cache/claude-tmp/panel3/samples/`.
   For every language and shipped part of speech, take about 20 held-out words, chosen
   at random and across the word groups. Write each word's table as `try` scores it
   (`inspect --parts --holdout`) next to the dump's table. Include the wrong cells.
   Members can't run training, so this is how they see real output.
2. **Context file.** Write one context file all members read first:
   - the project and where things are;
   - this prompt's §0 (results, sizes, times, leads, tried-and-dropped);
   - the owner's constraints (§5);
   - the read-only rules;
   - coverage as decided: every language ships every part of speech it inflects. Members
     propose how to model the missing parts, not whether to. Give them §3's inventory;
   - each member's assigned crates (the table above), with the instruction to analyse the
     generated code, not only the trainer:
     - is it the grammar a linguist would write?
     - what is redundant, oversized, wrong or missing?
     - where does the size go?
     - which trainer code made what the crates show?
   - Members must not re-propose what PR 16 or PR 18 did. They build on it, and say with
     evidence where a deferred or dropped item should come back.
3. **Round 1, analyses.** Each member writes:
   - a diagnosis;
   - what they found in their assigned crates, with file:line;
   - up to eight proposals, each with the problem (with evidence), the change, the
     expected gain, the cost (crate bytes, build time, complexity) and the risks.

   The typologist pair also writes, per missing part of speech:
   - its cells and features;
   - which dump template feeds it;
   - what the extraction must repair (Turkish's shifted tags);
   - what the existing machinery covers and what it lacks.

   Use one JSON schema for every member's answer.
4. **Dossier.** Collect all analyses into one dossier.
5. **Round 2, the meeting.** Each member reads the whole dossier, 14 analyses, its twin's
   included, and returns:
   - their top five proposals across the panel;
   - objections, each tied to a proposal, with evidence;
   - endorsements, with any sharpening;
   - their own proposals revised or withdrawn.

   Members check contested claims against the code.
6. **Your decision.** Re-check every fact the plan rests on yourself, read-only, against
   the code, the crates and the log. You may run short diagnostics (`try --quick` on small
   parts, `inspect`) to settle a contested claim. Then write the decision:
   - **What the panel agrees on:** the state of the crates and where the error and the
     bytes are.
   - **The plan:** items ranked, the missing parts of speech among them. For each:
     - what to change;
     - why, with evidence;
     - how it is judged (the measurement, the rows, the gate that decides whether it
       lands);
     - the expected effect on accuracy and on crate bytes.
   - **What was rejected or deferred,** with reasons and the earlier measurements behind
     them.
   - **Open disagreements,** each with the measurement that will settle it.

   Rank by evidence and by what the owner values, in this order:
   1. correct forms;
   2. readable, minimal crates;
   3. build cost.

   Take a correct output nobody scores, like PR 18's analytic forms, as seriously as a
   score.

**The record**, in `~/code/notes/projects/flexion/`, named with the date of the run:
- `trainer-panel-3-<date>.md`: the decision;
- `-dossier.md`: all analyses;
- `-minutes.md`: the meeting.

Write them in plain language. Define each term once (relation, listed word, floor, paired
comparison, dev tenth).

Size the plan to one PR: what one run can finish, measure and land.

## 3. Coverage: what is missing, measured

Do this before the panel's round 1, so members argue from real counts.

**The rule the plan must meet.** Each language ships every part of speech it inflects that
the dumps give tables for:
- nouns, adjectives and verbs;
- pronouns, numerals, determiners and articles;
- adverbs with degrees of comparison;
- postpositions with possessive suffixes (Finnish);
- proper nouns where they inflect.

A part may be left out only where the language doesn't inflect it (English prepositions)
or the dumps have no tables, with the reason given. A part already folded into a shipped
one (German articles under determiners) is said so, not duplicated. Every new part follows
the no-gaps rule: every word gets every form in every cell.

**Measure:**
1. **Inventory.** For each language, count per part of speech the entries with real
   inflection tables, by template. Use every edition the language's config reads, not only
   the English one. Show a sample table for each missing part. This replaces the rough
   table in §0.
2. **Turkish verbs.** Find out exactly how the dump shifts the verb rows' tags, from a
   sample of tables against the Wiktionary template's layout. Say whether a repair in the
   Turkish config's hooks can fix it. `gelirim` is the first person singular.
3. **Finnish verbs.** Find why they were never added: config, extraction, size or build
   time. Estimate what adding them costs, in cells per verb and in learn time against
   Finnish nouns' 43 minutes.
4. **Each missing part:** its cells and features, and what the existing machinery covers
   and what it lacks. These go to the typologist pair and the whole panel.

Log the inventory in the notes, with the record.

## 4. The implementation prompt

Write `~/code/flexion/prompts/trainer-panel-3-plan.md`. Model it on
`prompts/trainer-panel-2.md`, the prompt that carried out the second panel's plan: same
shape, same plain language, every term defined once. It must contain:
- **Where things stand,** as you measured it: the results table, sizes per crate and per
  file kind, run times, the coverage inventory.
- **Read first:**
  - the panel's record;
  - the code it cites, with file:line;
  - the literature behind each linguistic choice, with page or section numbers.
- **The items, in order:**
  - the missing parts of speech, every one, Finnish and Turkish verbs first;
  - the panel's ranked plan.

  For each item: what to change, the evidence, how it is judged, its gate, and the
  expected effect on accuracy and bytes. Decided items are marked decided.
- **Not in this PR:** what was rejected or deferred, with reasons.
- **What must stay true** (§5).
- **How the run works.** Copy §6's rules. Add that it never asks the owner anything and
  that its long runs are approved up front: sweeps, tenth-9 runs, single-language learns
  of any length, and the rebuild of every crate.
- **Judging changes:**
  - Every accepted change is confirmed three ways: on the development tenth (from the
    headword and with principal parts given, by word group), on tenth 9, and by a paired
    `compare` against the run before it. PR 18's bugs hid from the dev tenth scored from
    the headword.
  - Where a row loses, the run finds the words with `inspect --parts --holdout` and fixes
    the cause or drops the change.
- **Git.** One PR off the latest `main`, with this prompt as its first commit. fmt and
  clippy before pushing. `gh pr merge --merge` once CI is green, then check out the latest
  `main`. No publishing.
- **Done means:**
  - every item made, or measured and dropped with its numbers;
  - every language ships every part of speech it inflects, or the reason is in the log and
    README;
  - no significant loss anywhere (paired bootstrap) unless accepted for a stated reason;
  - the parts of speech shipped today stay no larger than 13.79 MB in total and none
    grows, unless accepted for a stated gain in correct forms. New parts are reported
    apart;
  - every crate regenerated and compiling, README examples passing, CI green;
  - READMEs re-scored once on the README tenth, with excerpts of what is new.
- **Final report:**
  - the PR link;
  - coverage before and after;
  - per language and part of speech, before and after: top-1, typed in full, principal
    parts, floor and grammar − floor, word groups, irregular lines, calibration;
  - crate bytes;
  - what each item gave;
  - how each open disagreement was settled;
  - how it was verified, with commands starting `cd ~/code/flexion`.

Before you finish, read the prompt once as the agent that will run it. Every number has a
source, every file:line is checked, and nothing assumes this conversation.

## 5. What must stay true

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
- **Trainer size is free; generated crate size matters.**
- **Pre-release:** no backwards compatibility, no shims; delete what a better design
  replaces.
- **Tests.** Few and meaningful. No golden, snapshot or parity tests. Each test needs a
  verified failure witness.

## 6. How you work

- **No `/tmp`.** Put scratch files in `~/.cache/claude-tmp/panel3/`.
- **Resource caps:**
  - parallelism: `-j 4`, at most 8 training threads;
  - every trainer process runs under
    `systemd-run --user --scope -q -p MemoryMax=16G -p MemorySwapMax=0`;
  - one memory-heavy run at a time;
  - no more than about six panel members at once.
- **Approved up front:**
  - the panel's Codex and subagent runs;
  - the answer samples;
  - the coverage inventory;
  - `try --quick` and `inspect` runs to settle a contested claim.

  Nothing else that runs over about 10 minutes. Before a long wait, tell the user what
  you're waiting on and for how long.
- **Clean up.** Kill everything you start, by PID. Never `pkill -f` or `pgrep -f | kill` a
  pattern that appears in your own command line: it kills your own shell.
- **Talking to the user.** Plain language, no internal labels without saying what they
  mean. Status reports say what is done, what is running, what is blocked, and when it
  will finish.

## 7. Done means

- **The coverage inventory** is measured and logged (§3), with the Turkish and Finnish
  verb findings.
- **The panel's record** is written: decision, dossier, minutes. Every fact the plan rests
  on has been re-checked by you.
- **The implementation prompt** is at `prompts/trainer-panel-3-plan.md`. It is
  self-contained and covers every missing part of speech and the whole ranked plan. It is
  not committed.
- **Nothing in the repo changed** but that file. No process you started is still running.

## 8. Final report

The report is the deliverable the owner reads. Give it in full in your final message, and
save a copy next to the panel's record. Say plainly what you found and what you recommend:
- **Coverage.**
  - Per language: the parts of speech shipped, those missing, with real counts and sample
    tables.
  - Why Turkish and Finnish verbs are missing, and what adding each one takes.
- **What the panel found:**
  - the state of the crates, read as grammars;
  - where the errors and the bytes are;
  - what is wrong, redundant or missing, with examples from the generated code.
- **Who sat,** and where the Claude and Codex members of a pair agreed or differed.
- **The recommended plan,** ranked. For each item:
  - what, and why;
  - the evidence;
  - the expected gain in accuracy and bytes;
  - the cost (run time, complexity);
  - how it will be judged.
- **Rejected or deferred,** with reasons.
- **Open disagreements,** and the measurement that settles each.
- **Risks,** and what you would do first if the run has to be cut short.
- **Paths:** the record, the report, and the implementation prompt.
