# Prompt: the third panel's plan — score as `try` scores, judge through the runtime, every part of speech

You own one PR on `~/code/flexion`. Branch from the latest `main`, at or after `9e8de97`
("Merge pull request #18"). Commit this prompt as the PR's first commit.

**The panel.** A third panel of fourteen read the trainer, the runtime and the generated
crates after PR 18, argued, and agreed on a ranked plan. It had seven specialities, each
with a Claude and a Codex member:
- a linguist;
- a classicist;
- a computational phonologist;
- a typologist for the missing parts of speech;
- an ML specialist;
- a Rust engineer;
- a data and evaluation specialist.

This PR carries out the whole plan. You may change anything if it serves that: runtime,
trainer, configs, `sound-law`, generated crates, tests, CI, xtask, APIs. The project is
pre-release: no backwards compatibility and no shims. Delete what a better design
replaces.

**One part is decided, not up to measurement:** each language ships every part of speech it
inflects (item 7). The other items land only where their measurement says so. An item
that doesn't pay is dropped, with its numbers. No prototypes or spikes: what lands is
finished.

**Check the panel's evidence.** The panel read code and answer samples; it did not change
anything. Where the code or a run says otherwise, follow the code and log why.

**Never ask the owner anything.** Make every decision yourself, on evidence. That includes
linguistic rulings: research them and decide, citing the reference grammar. Keep working
until the PR is merged.

## 0. Where things stand

flexion learns each language's inflection from Kaikki/Wiktionary dumps. It prints each
language as a Rust crate that *is* the grammar: paradigms, ending rules, sound laws, stem
slots, relations, irregular lines, all readable. Twelve languages ship.

**Read first:**
- the root `README.md`;
- a few generated crates (`languages/ancient-greek/src/verb/`,
  `languages/latin/src/verb/`, `languages/turkish/src/noun/`,
  `languages/finnish/src/noun/`, `languages/german/src/noun/`);
- the trainer (`crates/flexion-train/src/`), the runtime (`crates/flexion/src/`) and
  `crates/sound-law/`;
- the log, `~/code/notes/projects/flexion/grammar-crates.md`. Read the "PR 18" section
  closely; skim PR 12–16.

**The panel's record**, in `~/code/notes/projects/flexion/`:
- `trainer-panel-3-2026-10-15.md`: the decision. It has the plan with its evidence and
  file:line references, what was rejected or deferred and why, the open disagreements and
  how to settle them, and the facts the chair re-checked. Where this prompt is brief, it
  has the detail. Read it first.
- `-dossier.md`: all 14 analyses and 110 proposals, cited below as
  `<speciality>/<claude|codex> #n`.
- `-minutes.md`: the meeting: objections, endorsements, revisions.
- `-coverage.md`: the measured inventory of parts of speech in the dumps, with the Turkish
  and Finnish verb findings.
- `-report.md`: the chair's report to the owner.

**Answer samples** the panel read are in `~/.cache/claude-tmp/panel3/samples/` (scratch: they
may be gone; item 1(a)'s `inspect` makes them again): 20 tenth-9 held-out words per
language and part, with both tables. Their RIGHT/WRONG labels compare
raw strings, not as `try` does (item 1 fixes `inspect`), so treat a WRONG label as an
upper bound.

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

**Run times** (one at a time, under the caps in §4):
- **One crate (`cargo xtask build`):** Finnish 43 min, Greek 27, German 13, Russian 13,
  Latin 6, Polish 6; the rest 1–3 min; about 2 hours for all twelve.
- **`cargo xtask sweep <dir> --pool`:** the pooled development tenths for every language,
  about 2 hours.
- **Tenth 9 for all twelve:** about 50 minutes.
- **German nouns trained in full:** 196 s, of which the sound-law stage's greedy step is
  116 s.

**Coverage today, measured by the chair** (entries with an inflection table; details in
`-coverage.md`).

| language | ships (dump parts of speech read) | in the dumps, not read |
|---|---|---|
| english | noun, verb, adjective, adverb | pronoun (no tables; head-line forms only), proper noun (head-line plurals only) |
| russian | noun (noun, pron), adjective (adj, verb), verb, pronoun (pron, det), numeral | proper noun 4,333 (+6,931 Russian edition), adverb degrees from the head line 223 |
| latin | noun, adjective (adj, verb), verb, pronoun (pron, det), numeral | proper noun 9,233, adverb degrees from the head line 630 |
| ancient-greek | noun, adjective (adj, verb), verb, pronoun (pron, det, article), numeral | proper noun 4,070 |
| sanskrit | noun, adjective, verb, pronoun (pron, det) | numeral 69, proper noun 641 |
| polish | noun (noun, pron), adjective (adj, verb), verb, pronoun (pron, det), numeral | proper noun 18,398; adverb degrees from the head line 1,478; the mobile person clitic on conjunctions and particles (`żebym`) |
| german | noun, adjective (adj, verb), verb, determiner (article, det, pron) | proper noun 3,346 (+17,564 German edition), numeral 21 (+155) |
| spanish | noun, adjective, verb, determiner (det, num, article) | pronoun 16 (+22 Spanish edition), adverb degrees (`bien`/`mejor`) |
| french | noun, adjective, verb, participle, determiner | adverb degrees (`bien`, `peu`); pronouns and names have no tables |
| italian | noun, adjective, verb, participle, determiner (det, num) | article 7 (`il/lo/l'`, `i/gli`), adverb degrees (`bene`/`meglio`); pronouns have no tables |
| finnish | noun, adjective | **verb 15,371** (`fi-conj-*`), proper noun 9,507, adverb 1,438 with tables (2,155 with comparison), postposition 165 (with possessive suffixes), pronoun 82, numeral 81, determiner 21 |
| turkish | noun, adjective | **verb 2,388** (`tr-conj`), proper noun 1,844, pronoun 20, numeral 7 |

**How decisions are judged.**
- **Two runs compared:** `cargo xtask try <lang> --pos <part> --quick --save <file>` scores
  the dev tenth (tenth 2). `cargo xtask compare a b` is a paired bootstrap on words, with
  a row per word group.
- **Every language:** `cargo xtask sweep <dir> --pool`, compared as two directories.
- **`--fold 9`:** an untouched tenth, to confirm.
- **`--final`:** the README tenth. Score it once, at the end.
- **One word:** `cargo xtask inspect <lang> --pos <part> --word <w> --parts --holdout` shows
  its table as `try` scores it, with the principal parts given and the word held out.
- **`try --diagnose`:** the class-choice diagnostics.

**Tried and dropped before.** Don't revive these without a new idea and new evidence; the
numbers are in the log and the panel records:
- gender as a hard filter;
- a single global, interleaved or far-left vowel-length context (it costs Latin);
- restoring marks when typed in full;
- Russian stress via the uc-prosody branch;
- naive-Bayes beginning rules;
- prefixes for every language;
- marking cells `derived` wholesale;
- a fixed 3-letter compound head;
- choosing compound heads among every listed word (made Russian `ванный` a head,
  adjectives −0.16);
- derived cells from another stem slot's cell;
- choice-rule pricing (higher, or per letter);
- the Chu–Liu/Edmonds relation tree;
- a calibration refit by log loss (worse in PR 13);
- a global description-length objective;
- a German umlaut law;
- hard-EM reassignment beyond exact ties;
- a minimum paradigm size;
- accent run through the sound laws;
- variant-aware listing;
- a per-stem-slot class choice (Polish nouns: 87.6% of forms against 93.1% chosen whole);
- more than 4 paradigm readings per table;
- reordering the extra editions' variants (the German config already puts `-es` first,
  `german.rs:513-525`).

The third panel also rejected or deferred these; the reasons are in the decision's §3:
- heads among all training words with short prefixes and one root one vote;
- German `-es`/`-s` as a class dimension;
- deleting animacy listings for bytes;
- a per-language laws stage;
- vowel-only harmony tiers everywhere;
- feminine and neuter cells for analytic phrases;
- pooling evidence across paradigms;
- a following-word context for Italian articles;
- edge classes in the harmony learner;
- config-nominated relation edges;
- sampling in relation learning;
- all-ten-tenth development scoring;
- a Greek α/ι/υ-long default;
- dropping duplicated edition tables (already de-duplicated, `learn.rs:125-138`);
- one Greek tense stem across moods modulo the augment;
- the reflexive and postfix layer;
- status derived from printed counts;
- independent Greek vowel features.

## 1. Read first

- **The decision file (§0),** then the code it cites. Above all:
  - in the runtime, `grammar.rs`:
    - `kept` at 842–882, its `[false, true]` short-final-first test at 862;
    - `head` at 1263–1290;
    - the accent placement at 1454–1475;
    - the head path in `candidates` at 547;
  - in the runtime, `paradigm.rs:246` (per-cell text splitting) and `paradigm.rs:310-334`
    (precedence: own row, stem slot, parent);
  - in the trainer, `learn.rs`:
    - `misbuilt` at 3315–3345, its only runtime check (headword and full lemma, never
      principal parts given);
    - the stages after it, 743–795;
    - `untie` at 864–907;
    - `heads` at 2782;
    - `settle` at 2521–2540;
    - the edition de-duplication at 125–138;
  - in the trainer, also:
    - `extract.rs:362-392`: `Hooks::repair` sees every table form in order, before the
      tag policy;
    - `cells.rs` and `spec.rs:136-150`: a group is the full product of its features;
    - `eval.rs:253` (`same_form`), `eval.rs:325-461` (`outcomes`, `floor`);
    - `pipeline.rs:119-133`;
  - in xtask, `xtask/src/main.rs`, `inspect` at 1516–1640;
  - the configs `turkish.rs`, `finnish.rs`, `sanskrit.rs`, `ancient_greek.rs:150`,
    `latin.rs`, `german.rs`.
- **Literature, with sections:**
  - Smyth, *Greek Grammar*:
    - accent: §§ 163a, 169, 425–426;
    - augment: §§ 429–438;
    - prefixed verbs: §§ 449–451;
    - the paradigm tables: which moods each tense has;
  - Allen & Greenough, *Latin Grammar*: §§ 184–196 (the conjugations, with no future
    subjunctive);
  - Whitney, *Sanskrit Grammar*:
    - § 585 (the augment after the preverb);
    - §§ 602ff (present classes);
    - §§ 793–794 (the weak perfect);
  - Göksel & Kerslake, *Turkish: A Comprehensive Grammar*: verb suffix order and buffer
    consonants;
  - Karlsson, *Finnish: An Essential Grammar*, and the Kotus verb types 52–78: verb types
    and infinitives.

  For papers, use services that need no identity: OpenAlex, arXiv, ACL Anthology. Never
  send the owner's email address or other personal details to any outside service.

## 2. What to do, in order

The order puts measurement first and byte-identical work early, so that every later item
is judged and built faster. The decision file numbers the same items by rank; its number
is given in brackets.

### Item 1. Diagnostics that score as `try` scores [decision 1]

- **(a) `inspect`** prints both tables for a word:
  - from the headword (`Lexeme.lemma`, through `Grammar::table`, as `try` does; today it
    takes a simplified path without irregular lines);
  - with the dictionary's principal parts given;
  - each cell marked right or wrong by `eval::same_form` and the evaluator's cell mask;
  - also which lexeme it chose and which parts were supplied.
- **(b) An error census in `try`,** saved runs and the README. Each wrong form in exactly
  one category, by cell and word group:
  - class chosen wrong (an oracle paradigm builds it);
  - vowel length or accent;
  - stem fixed wrong;
  - a gap;
  - the dictionary itself inconsistent;
  - variant order;
  - other.
- **(c) Scoring.**
  - Large tables are also scored by word and by block of cells.
  - Small and closed parts are scored by reproducing every known table and by completing
    every cell. Their held-out rows stay, with their denominators.
  - Development uses the dev tenths only; tenth 9 and the README tenth stay reserved.
- **(d) A gap counter:** a cell where no construction route gives a form. Equality with the
  lemma is not the test; syncretism and invariant words are real.

**Check:** crates byte-identical. One test: a Russian form differing from the dictionary's
only by stress marks counts as right in `inspect`, as in `try`. Show it fails without the
change.

### Item 2. One judge through the runtime, on every input, and a closing gate [decision 2]

**What:**
- **Every structural decision is accepted through the runtime's own `table()`,** for each
  input mode: headword, lemma in full, principal parts given. The decisions:
  - heads and untie;
  - listing and stem slots;
  - derived cells and prefixes;
  - the phrase cells.
- **A closing gate after the last stage of `learn()`:**
  - a bounded no-regression check of the training words against the grammar before each
    late transformation;
  - not a loop that lists until everything is right;
  - contradictory homographs reported apart.
- **Fix what the judge finds,** starting with the known cases:
  - **the trainer's head finder against `Grammar::head`:** listed heads only; the 3-letter
    front;
  - **`untie` against the head path;**
  - **the head length rule;**
  - **the separable-verb principal-parts bug:** given `zwischenspeichern`'s main-clause
    preterite `speicherte zwischen`, the subordinate preterite comes out
    `speicherte zwischenst`, where the headword alone gives `zwischenspeichertest`;
  - **the other principal-parts regressions in the samples:** German 3 cells, Finnish 5,
    Polish 4 (data/codex #3).

**Judged by:**
- the dev tenth from the headword and with principal parts given, by word group;
- tenth 9;
- a paired comparison against the run before.

Each fix gets one test with a verified failure witness.

### Item 3. Faster builds, byte-identical [decision 13]

**What:**
- `eval::outcomes`, `eval::floor` and the strata over threads, in order: 893 s of the
  7,138 s rebuild; Finnish 518 s.
- An incremental `settle` for listing: re-judge only the words whose listing changed, and
  the words built from them. Greek verbs: 403 s.
- The marks model built once per fold in pooled sweeps.
- The laws greedy step: index stem-edge changes, reuse the bitset and threads, skip laws
  whose builds didn't change.
- Fewer corpus copies between stages (Finnish peaks at 12.8 GB).

**Check:** crates byte-identical (`bench --dump`, the end-to-end test); the stage trace's
time and peak memory before and after, for Finnish, Greek, German and Russian.

### Item 4. A part's cells are the language's cells [decision 3]

**What:**
- A config group can restrict a feature's values, or list the combinations the language
  lacks. `cells.rs` numbers only real cells.
- The printer gives a restricted group its own value type, so a non-cell doesn't compile:

  Before:
  ```rust
  verb::Cell::Future(Mood::Subjunctive, Voice::Active, Person::First, Number::Singular)
  // compiles, and returns an invented form
  ```
  After:
  ```rust
  verb::Cell::Future(FutureMood::Optative, Voice::Active, Person::First, Number::Singular)
  // FutureMood has Indicative and Optative only
  ```
- `Group.none` filters source tags, not cells; it can't do this.

**Which cells a language lacks** is a grammarian's statement, cited in the config, never
inferred from zero attestations:
- **Latin:** no future or future-perfect subjunctive, 24 cells. The conjugation tables of
  A&G §§ 184–196 have none.
- **Greek:** no future subjunctive or imperative, no first-person imperative, about 135
  cells. Smyth's paradigms give the future only in the indicative, optative, infinitive
  and participle; `ancient_greek.rs:150` already says so.
- **Template blanks** (`-`) are positions, not cells.

This is not modelling defectiveness: no word of the language has these forms.

**Judged by:** the attested cells' scores, by paired comparison (they may move, because
relations and slots change); bytes per file kind; the gap counter.

### Item 5. Ready for 900-cell parts [decision 6]

**What, with output byte-identical:**
- **Runtime:** resolve each reading's paradigm once per `table()`, not once per cell
  (Greek verb tables take 81 ms today).
- **Trainer:** exact bounds and indexes for relation learning on large cell spaces. No
  sampling: it changes the 95% sameness decision.
- **Printer:**
  - print the cell schema and relation macros once (44 KB of repeated macros today);
  - compact per-cell scaffolding (about 420 bytes per cell today, so 928 Turkish verb
    cells would carry about 400 KB before any grammar).

**Check:** byte-identical crates, and the stage trace.

### Item 6. Bytes: print the grammar, not a list of lines [decision 8]

**What:**
- **(a) Listed words grouped by class** in `rules.rs`, and the same for animacy, gender
  and accent listings.
  - Expanded at compile time into today's sorted array, so lookup stays a binary search.
  - Measured: 2.39 MB of listed lines, about 0.8–1.2 MB grouped.
  - Nothing computed before the first call.
- **(b) A syncretism stated once.** An ending table may write `derived` where the
  paradigms stating it derive the cell, and a child's row equal to what it inherits is
  dropped.
  - Rows that only repeat `derived` are 331 KB today (Greek verbs 203 of 578 KB).
- **(c) Readable names:**
  - stem slots named after the principal part or tense they hold
    (`stem perfect: ~āv- like agitō`), not `rest___________`;
  - "like X, except in N cells" counts only rows that differ.

**Judged by:** answers identical (the end-to-end test compares every answer), and bytes per
file kind.

### Item 7. Every part of speech (decided)

**Data gate for every new part.** An audit runs on the repaired entries (after `repair`),
gating on two things:
- two forms in one cell of one table;
- a table position mapped to no cell.

Print every dropped form's count to the log and read a sample.

**(a) Turkish verbs.**
- **Extraction:**
  - a `verb` part reading `tr-conj`, the first table only;
  - `bilmek`'s second table is the `-ebilmek` suffix table;
  - spaced multiword verbs allowed (490).
- **Tags by position.** `TurkishHooks::repair` retags every form by its position. The tags
  are wrong but the 940 positions never move (2,381 of 2,381 verbs). The trainer gets no
  layout reader: the hook already sees the forms in order, before the tag policy.
- **The layout:**
  - generated by loops: polarity × ability × tense/aspect/mood × copula × person, then
    converbs, participles and verbal nouns. No 940-line list;
  - re-derived and validated against `gelmek`, `bilmek` and the template;
  - anchors as tests, in zero-based positions over the lexical forms: the infinitive at
    209, the participle at 211;
  - positions 204 and 206 are the formal 2sg and 2pl, not duplicates;
  - **928 cells is the panel's count.** The template, not the panel, is the authority.
- **Dropped:**
  - template blanks (`-`), which are not cells;
  - the 26 cells where the dictionary writes non-forms (`atabilarım`, `olabilurum`; 80
    verbs), logged.
- **Laws:**
  - the aorist vowel (`-r`/`-Ir`/`-Ar`) is lexical, so it is a principal part;
  - buffer consonants (`-y-`, `-n-`, `-s-`) and edge laws before harmony (`söylüyor`) are
    learned as laws;
  - the d/t at a suffix's start is an edge law the adjective part already learned, not a
    harmony class.

**(b) Finnish verbs.**
- **Extraction:**
  - a `verb` part reading `fi-conj-*`, forms whose source is `conjugation`, the first
    table;
  - entries carrying only participle declensions (2,729) not read;
  - spaced verbs allowed.
- **Tags:**
  - the finite forms (positions 0–139) are tagged right;
  - the non-finite block (140–181) is retagged by position in `repair`, anchored on the
    sub-table markers (the layout holds over 11,980 verbs);
  - the colloquial block (`error-unrecognized-form`: `tuun`, `tuut`) dropped.
- **Phrases.** Negatives (`en tule`, `älä tule`) and perfects (`olen tullut`) are phrases
  built from cells, with PR 18's phrase machinery.
- **First-word verbs.** Spaced verbs that inflect their first word (215; `pyytää anteeksi`)
  need a `Compound::First` with real boundary and routing support in the runtime, not only
  an enum value.
- **Laws:**
  - gradation and harmony as in the nouns;
  - `tulla`/`tullut` is consonant-stem assimilation (the ending's consonant copies the
    stem's last), a law of its own, not gradation.

**(c) Parts that share another part's grammar.**
- **The mechanism.** A part can declare `shares: noun`: it reuses the host part's
  paradigms, laws and ending tables, and learns its own choice rules and listed words.
  Where it needs a paradigm the host lacks, it adds one.
- **Proper nouns:**
  - a separate public part in every language that declines them;
  - the counts in §0, with the editions' names;
  - names choose differently from nouns: English names in consonant+y take `-ys` 1,418
    times against `-ies` 38, so they learn their own rules and don't shift the noun
    part's rules or calibration;
  - casing kept: an acronym (`NATO`) must not uppercase its suffix;
  - Turkish keeps the apostrophe before suffixes as written;
  - a singular-only name gets every plural cell, marked unattested. Most Polish and
    Finnish names have plural tables anyway.
- **Also sharing the noun grammar:**
  - Turkish adjectives (53.82% alone, where nouns are 96.51%);
  - numerals that decline as nouns.

**(d) Closed classes and small parts.**
- **Ship:**
  - Finnish pronouns, numerals and determiners, postpositions with possessive suffixes
    (`alla`/`allani`), and adverbs with comparison;
  - Turkish pronouns and numerals;
  - Sanskrit numerals (the dump leaves their genders untagged: repair);
  - German numerals (`de-ndecl` is the German noun-declension template, not
    "indeclinable");
  - Spanish pronouns (`ese`, `esto`);
  - Italian articles as ordered spellings of the gender × number cells, the citation form
    first (`il|lo|l'`, `i|gli`). No following-word context;
  - comparative adverbs from the head line in Latin, Polish and Russian, and the
    suppletive degrees of French `bien`/`peu`, Italian `bene`/`meglio`, Spanish
    `bien`/`mejor`.
- **Leave out, with the reason in the README:**
  - **The Polish mobile person clitic** (`żebym`, `kiedyś`). The same six-form table sits
    on conjunctions, adverbs and particles: it attaches to the clause's first word and is
    not inflection of those words.
  - **Turkish `daha`/`en` comparison:** periphrastic, free words.
  - **English, French and Italian pronouns,** unless their head lines give case forms.
    Check; the dumps have no tables.
- **How they are judged:** item 1(c) (reproduction and completion), with held-out rows
  where they exist. A part's grammar is not capped at the size of its word list,
  because productive families inside closed classes must generalize.

**Each new part gets:**
- its config;
- a module like the others;
- its README rows (top-1, floor, principal parts where the dictionary gives them,
  reproduction and completion);
- the end-to-end test's coverage.

Its bytes and build time are reported apart from the existing parts.

### Item 8. Greek: the runtime writes possible Greek [decision 4]

Each step is measured alone, in this order:
- **(a) `Grammar::kept` takes the cell.**
  - It re-accents a compound only where the class's place for that cell is recessive.
  - It counts finals as the cell says (`Accent::finals`): the optative 3sg `-οι` is long
    (Smyth §169).
  - Infinitives and participles aren't recessive (§425).
  - The wall stays: not before the augment, nor before the prefix's last syllable (§426).
  - Today `προφέροι` comes out `πρόφεροι`.
  - The aorist infinitive has 408 irregular lines (342 compounds) and the optative 3sg
    209 (200).
- **(b) The typed accent constrains length and class.**
  - A form that needs an accent never comes out without one (60 of 300 sampled Greek noun
    forms do today: `ὀστουν`).
  - Proclitics and enclitics stay as the language writes them.
- **(c) Breathings and augments.**
  - Breathings stay word-initial in every join (134 sampled forms like
    `προοἰσθήσεσθαι` today).
  - Add the missing temporal-augment environments: ε-initial `ἐ → ἠ` and `ἑ → ἡ` in the
    imperfect and aorist (`ἤλπιζον`), and `αὐ/εὐ → ηὐ`.
  - The other vowels' augment laws already exist.
- **(d) The `-εως`/`-εων` genitives:** a scoped exception to accent limitation (Smyth
  §163a), not a global syllable merge. They are 1,004 of the 2,136 Greek noun irregular
  forms.
- **(e) Then Greek preverbs as one morpheme,** with elision and junction laws (Smyth
  §§ 449–451).
  - The learner never finds elided prefixes because it compares marked letters exactly
    (`learn.rs:1455-1466`).
  - 29,597 of 33,829 Greek irregular forms (925 KB) are on prefixed verbs. That is a
    candidate population, not a forecast: measure what (a)–(d) leave, then stage this by
    mechanism.

**Judged by:**
- Greek nouns, adjectives and verbs from the headword and with principal parts given;
- the count of accentless forms on words that need one;
- the gap counter;
- `irregular.rs` bytes.

### Item 9. Sanskrit verbs by root [decision 5]

**What:**
- **Tense pages.** A perfect, aorist, future or benedictive page with a table goes into its
  root's verb. 265 of the 275 such pages share their root with a present-verb entry. The
  rest are kept as citation entry points.
- **Derivative conjugations** (causatives, passives, desideratives) stay their own verbs.
- **The present class** (Whitney §§ 602ff) is read as an inherent feature.
- **Laws:**
  - the vṛddhi augment as one map (`ऊहति` → `औहम्`, not `अऊहम्`);
  - the augment after the preverb (§ 585);
  - the weak perfect stem (§§ 793–794: `नुनुदिव`, not `नुनोदिव`).

**Why:** 293 Sanskrit verb entries are form-of tense pages learned as verbs. They give
classes named after a perfect (`वेश`) or an aorist (`अग्रहीत्`), and 103 of the 244
irregular lines.

**Judged by:** Sanskrit verbs, pooled tenths, from the headword and with principal parts
given; the irregular-line share; the class count.

### Item 10. Laws stated once [decision 9]

**Exact canonicalization:** laws that are one law are printed once, with answers
identical. Examples:
- Russian `ы → и` after velars and hushers (7 laws);
- the Greek augment's per-vowel laws;
- copy deaspiration (3 laws).

**Widening a law** to letters training didn't see is a separate experiment, judged on
paired numbers. Not the German superlative: `-est` depends on stress
(`am typischsten` 756 times against 6).

### Item 11. Finnish harmony in compounds [decision 10]

**What:**
- A generic config option names the dump's compound templates and their argument slots.
  110k of 175k Finnish noun and adjective entries carry one (`maanpiiri` = `maa|piiri`).
  Their last-member boundary trains the heads, so held-out compounds inflect through their
  last member.
- Paradigms that differ only by the harmony letter class are folded
  (`kansainvälinen` into `tasainen`).
- Harmony domains are added only for what heads miss.

**Why:** 31% of sampled Finnish headword errors are the harmony letter.

**Judged by:** Finnish nouns (−0.48 against the floor) and adjectives, from the headword
and with principal parts given, dev tenth and tenth 9.

### Item 12. Vowel length: rules that can skip the left letter [decision 11]

**What:**
- Marks contexts may back off past the left letter, so "long before ns" is one rule, not
  132. The one-left-letter rules stay; dropping the left letter outright costs Latin.
- Spelling facts that always hold (Greek final ς, initial ῥ) become orthography laws.
- Any smoothing is fitted on the tenth sharpness is fitted on, never on the dev tenth.

**Why:** PR 18's diagnostic classifier beat the rules by 4.8 points on Latin lemma
spelling (78.93 against 74.15).

**Judged by:** Latin and Greek from the headword (lemma spelling and top-1), and
`marks.rs` bytes.

### Item 13. Repair confirmed data errors, with provenance [decision 12]

**What:**
- Spanish `Poco`: its feminine plural is taught as `-os|-as`. An unspecified plural is read
  by the headword's gender.
- Russian `документировать`: a present participle sits in its past-passive cell.
- The source errors listed by the classicists (classicist/codex #4).

Each repair goes in a config, logged with its count.

**Judged by:** paired comparison per affected part.

## 3. What must stay true

- **A crate is its grammar.** Named items: laws, stem slots, ending tables, paradigms,
  relations, heads, affixes, accent marks, phrases. Every rule carries its evidence. No
  probability tables, opaque ids or weights. Any classifier lives in the trainer only.
- **The end-to-end test** prints learned grammars, compiles them with `-D warnings` and
  compares every answer. It keeps passing and covers every new item: restricted cells,
  grouped listings, shared parts, positional repairs, `Compound::First`.
- **Every word has every form in every cell the language has,** attested or not (analytic
  forms as phrases). No defectiveness; never the lemma as a stand-in. Rarity or doubt
  goes into status and reason only.
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

## 4. How you work

- **No `/tmp`.** Put scratch files in `~/.cache/claude-tmp/`.
- **Resource caps:**
  - parallelism: `-j 4`, `--test-threads 4`, at most 8 training threads;
  - every trainer process runs under
    `systemd-run --user --scope -q -p MemoryMax=16G -p MemorySwapMax=0`;
  - generated crates are built one at a time;
  - one memory-heavy run at a time.
- **Never ask the owner anything.** The long runs this plan needs are approved up front:
  - sweeps (about 2 hours pooled);
  - tenth-9 runs (about 50 minutes for all);
  - single-language learns of any length;
  - the rebuild of every crate (about 2 hours today; more with the new parts).

  Before each, tell the user what you're waiting on and for how long.
- **Fast loops.** `try --quick` on small, quick languages first (Sanskrit, Turkish, French,
  English, Spanish, Italian). Run Finnish, German and Russian to confirm.
- **Judging changes.** Confirm every accepted change three ways before moving on. PR 18's
  bugs hid from the dev tenth scored from the headword:
  - the dev tenth, from the headword and with principal parts given, by word group;
  - tenth 9;
  - a paired `compare` against the run before it.

  Where a row loses, find the words with `inspect --parts --holdout`, then fix the cause or
  drop the change.
- **Clean up.** Kill everything you start, by PID. Never `pkill -f` or `pgrep -f | kill` a
  pattern that appears in your own command line: it kills your own shell.
- **Tests.** Few and meaningful. No golden, snapshot or parity tests. Each test needs a
  verified failure witness: break the code, see it fail, restore.
- **Progress log.** Dated entries in `~/code/notes/projects/flexion/grammar-crates.md`,
  under a new section for this PR. Log numbers, what was tried and dropped, and why, with
  the reference-grammar sections behind each linguistic choice.
- **Subagents** are allowed for independent measurement and audits. Don't run memory-heavy
  builds in parallel.
- **Commits.** Commit as the git config's identity only. Never override it, and pass the
  same rule to any subagent that commits.
- **Talking to the user.** Plain language, before/after code for API changes, and
  copy-pasteable commands starting with `cd ~/code/flexion`. Status reports say what is
  done, what is running, what is blocked, and when it will finish.

## 5. Git

One PR from a branch off the latest `main`. Run fmt and clippy before pushing. Merge with
`gh pr merge --merge` once CI is green, then check out the latest `main`. No crates.io
publishing, tags or releases.

## 6. Done means

- **Every item in §2** is made, or measured and dropped with its numbers. Item 7 is made
  for every part of speech in §0's coverage table, or the part is left out with its
  reason in the log and the README.
- **Each open disagreement** in the decision's §4 has the measurement that settles it,
  logged. The narrow heads-and-families experiment runs if item 2 has landed; otherwise it
  is logged for the next panel.
- **Before/after for every language and part of speech:**
  - on the pooled dev tenths and on tenth 9;
  - from the headword and with principal parts given, by word group;
  - with the error census;
  - crate bytes per file kind.
- **No significant loss anywhere** (paired bootstrap), unless accepted for a stated reason.
- **Size.** The parts of speech shipped today stay no larger than 13.79 MB in total, and
  none grows unless accepted for a stated gain in correct forms. Item 6(a) alone should
  take about 1.2 MB off. New parts are reported apart, with what they cost.
- **Correctness:**
  - no invented cells (item 4);
  - no accentless Greek form on a word that needs an accent;
  - no breathing inside a Greek word;
  - the gap counter at zero on every part.
- **All crates** regenerated, compiling, built one at a time, with README examples
  passing. CI green.
- **READMEs** (root and crates):
  - re-scored once on the README tenth;
  - the coverage table, the error census and the below-floor list redone;
  - real excerpts of what is new: a Turkish verb's relations, a Finnish verb, a proper-noun
    part sharing its noun grammar, a grouped listing, a restricted cell enum.

## 7. Final report

- **The PR link.**
- **Coverage:** per language, the parts of speech before and after, with what was left out
  and why.
- **Per language and part of speech,** before and after:
  - top-1 from the headword, typed in full, principal parts given;
  - the floor and grammar − floor with its interval;
  - the word groups;
  - the error census;
  - the irregular-line share;
  - calibration;
  - the new parts with their reproduction and completion scores.
- **Crate bytes** per crate and per file kind, before and after; the new parts apart.
- **What each item gave,** what was dropped and why, and how each open disagreement was
  settled.
- **Run time:** per stage, before and after, for Finnish, Greek, German and Russian.
- **Verification:** how you verified it, and how the user can run it.
