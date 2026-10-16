# Prompt: the second panel's plan — ship what is read, compare fairly, state things once

You own one PR on `~/code/flexion`. Branch from the latest `main`, at or after `492b3c8`
("Merge pull request #17").

A second panel of six specialists read the trainer and the generated crates after PR #16,
argued, and agreed on a ranked plan:
- four Claude members: a linguist, an ML specialist, a Rust engineer, and a data and
  evaluation specialist;
- two Codex members: a computational phonologist and a performance engineer.

This PR carries out the whole plan. You may change anything if it serves that: runtime,
trainer, configs, `sound-law`, generated crates, tests, CI, xtask, APIs. The project is
pre-release: no backwards compatibility and no shims. Delete what a better design
replaces.

**Order.** Work through §2 in the order given. The first items make sizes and comparisons
honest, and later items are judged by them.

**Keep only what pays.** An item stays only where its measurement says so. An item that
doesn't pay is dropped, with its numbers. No prototypes or spikes: what lands is
finished.

**Check the panel's evidence.** The panel read code; it did not run it. Where the code or
a run says otherwise, follow the code and log why.

## 0. Where things stand

flexion learns each language's inflection from Kaikki/Wiktionary dumps and prints it as a
Rust crate that *is* the grammar. Twelve languages ship.

**Read first:**
- the root `README.md`;
- a few generated crates (`languages/latin/src/noun/`, `languages/german/src/noun/`,
  `languages/spanish/src/verb/`, `languages/finnish/src/noun/`);
- the trainer (`crates/flexion-train/src/`), the runtime (`crates/flexion/src/`) and
  `crates/sound-law`;
- the log, `~/code/notes/projects/flexion/grammar-crates.md`. Read the "PR 16" section
  closely and skim PR 12–15: they say what was built, measured and dropped, and why.

**The panel's record**, in `~/code/notes/projects/flexion/`:
- `trainer-panel-2-2026-10-15.md`: the chair's decision. It holds the plan, the evidence
  (file:line), the rejections with the earlier measurements behind them, and the open
  disagreements. Where this prompt is brief, it has the detail. Read it first.
- `trainer-panel-2-2026-10-15-dossier.md`: all six analyses (44 proposals).
- `trainer-panel-2-2026-10-15-minutes.md`: each member's objections, endorsements and
  revisions.

**Results on the README tenth.** The table in the root README has top-1 from the
headword, against the floor, words built on a training word against new roots, and the
other columns. Some rows:
- Latin nouns 68.69 (92.93 typed in full, 98.69 with principal parts).
- Ancient Greek nouns, adjectives and verbs: 71.16, 77.17, 65.28.
- Below the floor beyond chance: English verbs −0.62, French verbs −0.59, Finnish nouns
  −0.40, Spanish adjectives −0.36, Polish adjectives −0.07.

**How decisions are judged.** On the development tenth (tenth 2), comparing the same
words before and after:
- `cargo xtask try <lang> --pos <part> --quick --save <file>`, then `cargo xtask compare`
  on two such files;
- `cargo xtask sweep <dir>` for every language, and `compare` on two sweep directories;
- `try --diagnose` for the class-choice diagnostics;
- `--final` scores the README tenth. Score it once, at the end.

**Size: the generated crates only.** The trainer and runtime may be as large as they need.

The crates total 19.85 MB and 498k lines. Per crate:

| crate | MB |
|---|---:|
| ancient-greek | 4.20 |
| russian | 2.97 |
| german | 2.46 |
| latin | 1.93 |
| polish | 1.73 |
| spanish | 1.67 |
| finnish | 1.56 |
| sanskrit | 0.90 |
| english | 0.89 |
| italian | 0.87 |
| french | 0.42 |
| turkish | 0.27 |

By file kind:

| file | MB |
|---|---:|
| relations.rs | 6.20 |
| rules.rs | 4.25 |
| paradigms.rs | 3.01 |
| irregular.rs | 1.90 |
| laws.rs | 1.08 |

**Build times** (`cargo xtask build`, one at a time):
- Finnish ~53 min, German ~59, Russian ~30, Polish ~19, Greek ~17, Latin ~11; the rest
  1–6 min;
- about 3.5 hours for all twelve.
- Learning the sound laws is 85–97% of Russian and German learn time. That "laws" timer
  also includes the rehome and inherent steps (`learn.rs:270–279`).

**Tried and dropped before.** Don't repeat these without a new idea; the numbers are in
the log and the decision file:
- pricing choice rules higher or per letter;
- building the whole relation tree at once (Chu–Liu/Edmonds);
- an interleaved two-sided vowel-length context;
- a calibration refit on pooled tenths;
- a global description-length objective to choose between laws, stems and classes;
- a German umlaut law;
- hard-EM reassignment beyond exact ties;
- a minimum on paradigm size;
- accent run through the sound laws.

## 1. Read first

- The decision file (§0), then the code it cites. Above all:
  - in `grammar.rs`: lines 23, 558, 677, 739, 857, 975–983, 1032 and 1161;
  - `paradigm.rs:173–198`;
  - `pipeline.rs:129`;
  - in `learn.rs`: lines 248, 270–279, 632, 706, 1471, 2037 and 3054–3058;
  - `stems.rs:779–784`;
  - `laws.rs:1323–1330, 1334, 1492, 2088–2091`;
  - `laws/harmony.rs:597`;
  - `eval.rs:346–356, 721–733`;
  - `print.rs:468–527, 850–866`;
  - the Latin, Polish, German and Romance configs.
- Literature: Hayes & Wilson 2008, and Rasin & Katzir, on stating phonological rules
  once; Stump 2001, on rule blocks and morphological conditions on affixes; Allen, *Vox
  Latina*, on hidden quantity.
- For papers, use services that need no identity: OpenAlex, arXiv, ACL Anthology. Never
  send the user's email address or other personal details to any outside service.

## 2. What to do, in order

### Item 1. Ship only what the runtime reads, and measure crate bytes everywhere

**Report bytes.** `try`, `--save` and `compare` (two runs or two sweeps) report:
- printed bytes per file kind;
- listed-entry counts, with relation-listed words included.

The crate README's grammar table counts relation-listed words too. From here on, crate
bytes are a measured, compared number in every experiment.

**A reachability pass at the end of `learn()`.** Decide what can be read by using the
runtime's own resolution (`Paradigm::pattern_in`, `Grammar::changing`,
`told_from_listed`), not copied logic. Three tiers:
- **(a) Exact.** A relation's whole change list and rules go when all of these hold:
  - no paradigm derives its cell;
  - it is `listed: false`;
  - no analogy relation borrows its changes.

  Keep the relation entry itself: `grammar.rs:739` reads that it exists, to flag
  unattested cells.
- **(b) Exact for dictionary input.** For a `listed: true` relation whose cell no
  paradigm derives, keep only the exception keys that listed words' source forms
  produce. Include forms reached through chains of relations and forms built on listed
  stems.
- **(c) Behavioural.** For a cell some paradigms derive, drop the exception keys that no
  deriving paradigm's training word has. Judge this tier alone, with the paired
  comparison. Count the held-out words whose source form hits a pruned key, and explain
  every changed answer, for example a reflexive verb's key reaching a plain verb.

**Checks:**
- In the trainer: every training word's answers are byte-identical before and after
  tiers (a) and (b).
- One end-to-end test: every printed relation payload has a reader.

**Panel's evidence** (check it):
- English NOUN_PL, 186 KB, is `listed: false`.
- Three German noun relations, most of 798 KB, are `listed: false`.
- Spanish IMPERATIVE_TU, 254 KB, is `listed: false`.
- Spanish USTED/NOSOTROS/USTEDES (about 770 KB) are `listed: true`, against 333
  irregular lines.

Expect at least about 1.2 MB less.

### Item 2. Fair comparisons

- **(a) Same data.** Train the floor on the same words as the grammar (`pipeline.rs:129`
  and xtask). Today the floor uses main-dump words only: about 62k German nouns against
  the grammar's 144k.
- **(b) Word groups.** Score form-of entries (participles, comparatives) and alt-of
  entries (spelling variants) as their own groups. 59% of Latin adjective test forms are
  form-of entries.
- **(c) A better column than "attested".** "Attested" is 99.7–100% everywhere. Replace it
  with the share of training words that need an irregular line.
- **(d) Pooled tenths.** `try --folds` pools several development tenths for rows under
  about 5,000 words. Each accepted change is confirmed once on a tenth no decision looked
  at.
- **(e) Tied best paradigms.** The oracle's "best paradigm" becomes the set of tied
  paradigms (`eval.rs:721–733` breaks ties by index).
- **(f) The extra editions.** After (a), re-decide each extra Wiktionary edition on the
  development tenth, learned with and without its words (German noun learns take about
  1,000 s each).

### Item 3. Clean the input, and list only real irregularities

**Configs:**
- drop Middle Polish (`pl-mid-decl-noun`, `polish.rs:82`) by checking `exclude` tags at
  the sense level;
- stop "plural only" becoming the form *only* (`romance.rs:84–89`);
- strip "ist / es ist / sind" from German predicative cells.

**Foreign tables.** Flag a word when none of its lemma-cell spellings has the headword's
letters. Print every flagged word to the log and read it, then drop the flagged words,
reporting the flagged, kept and dropped counts. Legitimate conventions must survive:
- Greek contracted-lemma verbs (687 of them);
- Sanskrit active lemmas.

The panel found, for example, 65 Polish verbs carrying another word's table, naming 33 of
179 paradigms.

**Variant-aware listing.** `misbuilt` (`learn.rs:3054–3058`) counts a cell as right when
the grammar's form matches any listed variant. Make sure a training word's first answer
doesn't silently change to its second variant.

**Naming.** Name each class after a member with no irregular line. Today 426 of 3,308
paradigms are named after a listed word.

### Item 4. Paradigms use their shared stem tables

The Latin third declension never uses its 4,119-word `ratiō` table:
- 53 `stem noun_sg` statements, 1 `stem noun_pl`;
- Ratiō, Amor and Carmen each spell out the same 11 endings.

**Steps:**
1. Log why `stems::state` rejects each table match, before changing anything. The cause
   is unproven.
2. The panel suspects two causes. Fix whichever the log shows, or what it shows instead:
   - tables are built before `inherit` (`learn.rs:632` vs `:706`);
   - exact equality fails on cells a paradigm derives (`stems.rs:779–784`).
3. Drop empty slot statements, those whose table is all `~`.
4. Name slots by their cells. Latin "noun_pl" today holds gen, dat, acc and abl sg.

Answers must not change.

### Item 5. Sound laws printed as sound laws

**A compile-time macro.** Add a small proc-macro to `sound-law` that compiles the existing
law notation into the same `CompiledRule` values the crates print today:
- same order and behaviour;
- a malformed law is a compile error; no silent fallback;
- laws.rs carries the notation instead of compiled structs.

The panel guessed about 1.08 MB → 0.2–0.3 MB (unverified).

**Doc examples from each crate.** Each crate's doc examples come from its own items. Today
every crate's header cites other languages (`print.rs:468–527`): Russian laws.rs cites the
Greek augment.

**Then, as a separate step judged on paired numbers:** merge laws that are one law, using
the existing class and map machinery:
- the Russian spelling rule ы → и after к г х ж ш ч щ, now six laws;
- the Greek temporal augment, now about 20 laws per vowel and context.

### Item 6. Analytic forms: build them as phrases (decided)

Latin's config drops periphrastic forms (`latin.rs:4–5`, `latin_word` at `latin.rs:280`).
The cells stay, filled by 74 analogy relations whose changes never fit
(`relations.rs:2116`: IND_PASS_PERF_3_SG from the active perfect with imperfect changes).
`Relation::apply` returns the source form when no change fits (`paradigm.rs:196–198`). So
the perfect passive very likely comes out as the active perfect: *amāvit* for "amātus
est". Run `amō` first and record every such cell's actual output.

**The decision, already made: build these cells as phrases.** Do not ask about it. The
linguistic case, which you should check and cite in the PR:
- **Latin has no one-word perfect-system passive.** Every reference grammar states it
  inside the conjugation as the perfect passive participle, agreeing with the subject,
  plus a form of *esse*:
  - perfect, pluperfect and future perfect: *amātus sum / eram / erō*;
  - subjunctive: *amātus sim / essem*;
  - the infinitives: perfect passive *amātus esse*, future active *amātūrus esse*, future
    passive *amātum īrī* (the supine with *īrī*);
  - deponents use the same forms in their active sense (*hortātus sum*).

  Sources: Allen & Greenough §§ 184–196, Gildersleeve & Lodge §§ 116, 128 and 248–250.
- **The dictionary agrees.** Wiktionary's own `la-conj` tables put them in the paradigm,
  as "tāctus + present active indicative of sum".
- **So these cells are forms of the verb.** Removing them would model defectiveness, which
  the owner's rule forbids. Leaving them filled by a broken analogy returns wrong forms.
- **Agreement, by the tables' convention:** masculine nominative, singular in singular
  cells (*amātus est*) and plural in plural cells (*amātī sunt*). Confirm what the tables
  write, the infinitives included (*amātus esse* or *amātum esse*), and follow them.

**The mechanism: generic, declared in the config.** A config states the language's
analytic cells. Each is "this cell is the form of that cell (the participle), with this
ending change for agreement, followed by this word": for example `("~us", "~us est")`,
`("~us", "~ī sunt")`. The trainer turns each into a fixed relation with one change, not a
learned one. Paradigms leave the cell derived, and the runtime's existing relation code
builds it. Nothing Latin-specific goes in the trainer or runtime.

**In the crate,** the relation reads as what it is, and its doc says these are analytic
forms. A word whose participle is listed gets its phrase from that listing, like any
derived cell. Verbs with no supine in the dictionary (intransitives: *veniō* has only the
impersonal *ventum est*) still get the cells, from the participle the grammar builds:
every word gets every form.

**Ancient Greek, the same principle.** Find the forms Smyth gives as analytic, mainly:
- the perfect and pluperfect middle-passive 3rd plural of consonant stems (*λελειμμένοι
  εἰσί*);
- the perfect middle-passive subjunctive and optative (*λελυμένος ὦ / εἴην*);
- the periphrastic perfect active subjunctive and optative (*λελυκὼς ὦ*).

Check what Wiktionary's `grc-conj` tables write for them, and declare them in the config
the same way. Greek's other never-attested cells (180 of 624: duals, the future perfect)
are synthetic forms the dictionary simply lacks. Keep generating them by analogy, and
check that analogy gives a sane form for a sample. Where it can't, say so with examples.

**Done means:** every analytic cell of `amō`, `moneō`, `regō`, `hortor` and Greek `λύω`
gives the grammar-book phrase, with the before/after outputs in the PR. Scoring: these
cells aren't scored today (no single-word dictionary form). Score them where the tables
give the phrase, and report them as their own group.

### Item 7. The sound-law stage: trace, cache for experiments, then exact speedups

**Trace inside `laws::learn`:** harmony, twins, candidates, the greedy loop, settle, and
rehome/inherent.

**A stage cache, for `try` and `sweep` only.** Never for `build`, `--final` or the README
run. The key covers:
- the stage's full inputs, inherent feature values included;
- a hash of the stage's own source.

**Then make it faster without changing its output.** Pick what the trace shows matters:
- indexed lookups instead of key scans;
- one compiled program per change, not one per try (`laws.rs:1492`), and no stale index
  entries (`:1323–1330`);
- parallel independent applications.

No speculative parallel greedy search: accepting a law reassigns words (`laws.rs:1334`).

**Gate.** Output byte-identical: the reseed test, the end-to-end print comparison, and
`compare` showing zero difference. Report the stage's time before and after for Russian
verbs and German nouns.

### Item 8. Finnish harmony on every spelling of a cell

`laws/harmony.rs:597` returns a variant unchanged when its ending length differs from the
first spelling's. That leaves seven Finnish twin classes (511 words; Ykkönen restates 19
possessive cells).

**The fix:**
- find the harmony letters in each variant separately;
- keep the variants' order;
- count support by word;
- then merge with the existing machinery.

Ykkönen's `~selleen|~sellensä` has no harmony letter in its first variant: handle it.

**Must not:**
- invent combinations no word has;
- grow the 1,078 forms the A-law already gets wrong.

**Judge it** on paired numbers, with possessive cells reported apart. Finnish nouns are
−0.40 against the floor.

### Item 9. Vowel length: measure what is recoverable first

**Step 0 (diagnostic, trainer-only).** A per-vowel classifier over:
- left context of 1–4 letters;
- right context of 1–6 letters;
- the part of speech;
- a "root seen in another training word" cue.

Report lemma-spelling accuracy against the marks rules, with each kind of cue left out.
Report separately the errors on keys whose dictionary entries conflict or carry no marks
(981 of the 3,466 Latin spelling exceptions have none).

**Step 1, only if left context pays.** 1–2 left letters as a back-off, read only after the
right-context rule has decided. `Choice::read` and the existing rules stay as they are.

**Step 2, only if the root cue adds something.** A learned root list. Weigh its crate
bytes.

**Judge** on Latin nouns and Greek nouns (the rows that lost before), pooled across tenths
for Greek, and by word group (item 2b).

### Item 10. Class choice where the grammar is below the floor

**A stronger diagnostic classifier:**
- endings up to 8 letters;
- a whole-word anchor;
- scored on forms, not only words;
- a word-beginnings ablation on every row.

**Run `--diagnose`** where the grammar is weak: German, Polish and Finnish nouns, English
and French verbs, Spanish adjectives, Greek verbs.

**Measure KEEP = 4, 8, 16.** The runtime builds forms from only the top 4 paradigm
readings (`grammar.rs:23`, `.take(KEEP)` at `:558`). For each, report the probability mass
kept, top-1, top-3, and µs per call.

**Gate.** Build a per-slot choice (one choice per stem slot, for classes that are products
of independent choices: Polish Ciąg vs Język) only if one of these holds:
- the stronger classifier beats the rules by a point or more on a weak row;
- a per-slot diagnostic wins.

Otherwise log the numbers and stop.

### Not in this PR

The panel rejected or deferred these. The reasons are in the decision file:
- choice-rule pricing;
- the global relation tree;
- interleaved marks context;
- calibration refit;
- hard-EM beyond ties;
- a choice cost in the keep test;
- German umlaut;
- Romance clitics and written accent (re-argue on readability after item 1);
- the affix layer as a named, priced morpheme;
- per-cell law support and inner boundaries;
- Greek contraction (first split Greek verb errors by which principal parts were
  supplied);
- syllable weight;
- whole-grammar pricing of laws against stems;
- harmony domains;
- gender counts and word beginnings in the choice;
- a heads de-duplication pass;
- compact text storage of relation exceptions;
- runtime table offsets;
- a shared prepared corpus;
- fit, rehome and relation/slot speedups beyond what item 7's trace shows;
- speculative parallel greedy search;
- an editors' audit for all twelve languages;
- trimming cells by attestation.

Add a line to the log if something you measure makes one urgent.

## 3. What must stay true

- **A crate is its grammar.** Named items (laws, stem slots, ending tables, paradigms,
  relations, heads, affixes, accent marks), evidence on every rule. No probability
  tables, opaque ids or weights. Any classifier lives in the trainer only.
- **The end-to-end test** prints learned grammars, compiles them with `-D warnings` and
  compares every answer. It keeps passing and covers every new item: the law macro, the
  pruned relations, slot tables, harmony variants.
- **Every word has every form in every cell,** attested or not (analytic ones as phrases,
  item 6). No defectiveness; never the lemma as a stand-in. Rarity or doubt goes into
  status and reason only.
- **Answers and output.** Calibrated candidates with a status and a reason on every
  answer. No panics (clippy denies unwrap/expect/indexing/panic). Nothing is computed
  before the first call. Output is byte-identical for the same input, across hash seeds
  and toolchains.
- **Nothing language-specific in the trainer or runtime.** If `sound-law` or another of
  the project's crates lacks a feature, add it there.
- **Code.** Idiomatic, type-driven Rust; clippy clean with `-D warnings`; README examples
  pass.

## 4. How you work

- No `/tmp`; put scratch files in `~/.cache/claude-tmp/`.
- **Resource caps.**
  - Parallelism: `-j 4`, `--test-threads 4`, at most 8 training threads.
  - Run every trainer process under
    `systemd-run --user --scope -q -p MemoryMax=16G -p MemorySwapMax=0`.
  - Build generated crates one at a time.
  - **Never ask the owner anything.** Make every decision yourself, on evidence, and keep
    working until the PR is merged.
  - The long runs this plan needs are approved up front: sweeps (about 75 min with
    `--diagnose`), single-language learns of any length, and the rebuild of all twelve
    crates (about 3.5 h). Run them capped as above, one memory-heavy run at a time.
- **Fast loops.** `try --quick` on small, quick languages first (Sanskrit, Turkish,
  French, English, Spanish, Italian). Run Finnish, German and Russian only to confirm.
- **Clean up.** Kill everything you start, by PID. Never `pkill -f` a pattern that matches
  your own command line.
- **Tests.** Few and meaningful. No golden, snapshot or parity tests. Each test needs a
  verified failure witness: break the code, see it fail, restore.
- **Progress log.** Dated entries in `~/code/notes/projects/flexion/grammar-crates.md`
  under "PR 18", with numbers, what was tried and dropped, and why.
- **Subagents** are allowed for independent measurement and audits. Don't run
  memory-heavy builds in parallel.
- **Talking to the user.** Plain language, before/after code for API changes, and
  copy-pasteable commands starting with `cd ~/code/flexion`. Before a long wait, say what
  you're waiting on and for how long.

## 5. Git

One PR from a branch off the latest `main`. Run fmt and clippy before pushing. Merge with
`gh pr merge --merge` once CI is green, then check out the latest `main`. No crates.io
publishing, tags or releases.

## 6. Done means

- **Every item in §2** is made, or measured and dropped with its numbers. The gated parts
  of items 2f, 4, 5, 8, 9 and 10 are decided by the measurements they name. Item 6's
  analytic cells are built as phrases, for Latin and Greek.
- **Before/after for every language and part of speech,** on development tenths (pooled
  where item 2d says), with no significant loss anywhere (paired bootstrap). Crate bytes
  are reported per file kind in every comparison.
- **Size.** Generated crates smaller than 19.85 MB in total. Relations at least 1.2 MB
  smaller. laws.rs smaller with the macro. No crate larger than now.
- **The floor** is trained on the same words as the grammar; the README's below-floor list
  is redone.
- **All twelve crates** regenerated and compiling; README examples pass; CI green.
- **READMEs** (root and crates):
  - re-scored once on the README tenth;
  - real excerpts of what's new: a law in its notation, a third-declension stem table,
    the new word-group columns;
  - each crate's docs citing its own examples.

## 7. Final report

- **The PR link.**
- **Per language and part of speech,** before and after:
  - top-1 from the headword, the floor (now on the same words) and grammar − floor with
    its interval;
  - the word groups (form-of, alt-of, on a training word, new root);
  - the irregular-line share; principal parts given; calibration.
- **Crate bytes** per crate and per file kind, before and after.
- **What each item gave,** what was dropped and why, item 6's analytic forms (before and
  after for the sample verbs, with the grammar references), and how each open
  disagreement in the decision file was settled.
- **The sound-law stage's time** before and after, by sub-stage.
- **Verification:** how you verified it, and how the user can run it.
