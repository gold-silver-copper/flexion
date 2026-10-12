# Prompt: grammar crates, from scratch

One PR for `~/code/flexion` (GitHub `gold-silver-copper/flexion`). Start completely from
scratch:

1. Check out the latest `main` and create the branch `grammar-crates` from it.
2. Make its first commit wipe the tree: `git rm -r` every tracked file except this prompt
   (`prompts/grammar-crates.md`). Commit it as "Start from scratch".
3. Build everything new on top of that commit.

The branch keeps `main`'s history, so the PR can merge, but none of its code. The old code
stays readable in git (`main`, and the unmerged branch `grammar-models`). It is reference
reading, not a base. A piece may come back only when it fits the new design and meets its
standards. Even then it is re-justified, not carried over by default.

---

## 0. Why

flexion learns the inflection of ten languages from Wiktionary (via Kaikki dumps):
- English, Russian, Latin, Polish, German, Ancient Greek, Sanskrit, Spanish, French and
  Italian;
- it emits one Rust crate per language, with typed functions per cell.

Today a generated crate is a statistical model written out as Rust:
- a suffix tree whose nodes hold probability distributions over thousands of opaque
  inflection classes;
- vectors of edit numbers;
- an in-memory twin of the model inside the trainer, which the generated code must match
  bit for bit.

Russian is 52 MB of source, most of it rows of class ids and 16-bit probabilities. It is
accurate, but you can't read the language in it.

The user, in their words:

> I feel we should get rid of the in memory model completely cause its clashing with my
> goals. I want the generated rust libraries for each language to be very human readable,
> not just weights in tables. I want them to really expose the phonology and grammar of a
> language. … You are trying to shoehorn model weights into rust code. I want you to be
> generating beautiful rust libraries that expose grammatical truths through code using
> machine learning techniques.

> I still want some sort of confidence scores or different candidates. Just done in a
> smarter more human way.

And earlier, still standing:

> I don't want these models to just regurgitate dictionaries, I want them to be able to
> also predict words which were never recorded but in theory were possible. So yes it's
> okay to confidently answer with a word the dictionary lacks if it grammatically makes
> sense.

The user has also said it's fine to produce forms that native speakers avoid. There is no
"no such form" answer.

The goal of this PR:
- the machine learning **discovers** a language's grammar from the data;
- the generated crate **is** that grammar, written as Rust a linguist would recognize;
- the user's hand-written Ruthenian grammar (below) is the model of the tone.

You own the PR fully: architecture, crates, trainer, configs, API, tests, CI, reports. This
is the real thing, not a prototype or a spike. The audits are part of the PR. What follows
says what must be true at the end and what is known. The design is yours.

## 1. What to read first

- **The research references:** `~/code/notes/projects/flexion/references/`. Start with
  `README.md`, then the four surveys:
  - `rule-induction.md`: Albright & Hayes's Minimal Generalization Learner with exact
    confidence formulas; Yang's Tolerance Principle; "no productive rule" gaps; pruning.
  - `paradigm-theory.md`: implicative entropy and Qumin; principal parts; Network
    Morphology default inheritance; stem spaces and morphomes; syncretism as referral;
    defectiveness.
  - `phonology-learning.md`: rule synthesis (Ellis 2022, SyPhon); MDL "keep a rule only if
    it pays"; subregular learnability; rule ordering by peeling; MaxEnt and Harmonic Grammar
    probabilities; Zuraw's listed-forms-first.
  - `paradigm-learning.md`: Hulden-style abstract paradigms; tiered confidence; evaluation
    splits and baselines; Zaliznyak stress letters; Greek recessive and persistent accent;
    finite-state grammar layout.

  These are a map, not a recipe. Use what earns its place, and record what you tried.
- **The user's Ruthenian grammar:** `https://github.com/gold-silver-copper/ruthenian`
  (`crates/ruthenian-core`). It is hand-written, and the generated crates should read like
  it. Its laws:
  - derive state and never store it;
  - one rule decides each thing;
  - endings tables per declension;
  - the soft series derived from the hard one;
  - seam rules as ordered rewrites;
  - marks on the lemma for what its ending can't tell.
- **The old system** (reference only):
  - `main` (PR #9, merge `3459e44`);
  - branch `grammar-models` (unmerged, the newer state);
  - its progress log, `~/code/notes/projects/flexion/grammar-models.md`.

  What it learned that still holds:
  - Kaikki extraction details are hard-won. Read `crates/flexion-train/src/extract.rs`,
    `kaikki.rs` and `configs/src/*.rs`: tag filters, multiword forms, editions, principal
    parts, markers.
  - Filling a cell a word's entry lacks must follow words that *end alike*. Taking the
    commonest pattern instead gave German *-t* adjectives the superlative *-ste* and lost
    2 points.
  - Ties between ways of reading a form must keep the word's start untouched. Otherwise
    *reprendre* stopped inflecting like *prendre*.
  - The Greek augment (ε→η, ἀ→ἠ, ἀνα→ἀνε) and reduplication are learnable rules.
  - Ideas measured and rejected, with numbers, are in the root README's "Ideas that didn't
    pay": factoring stress out of a probability tree, stem spaces as class vectors, the
    head rule as a runtime split. They were rejected *as size optimizations of that
    tree*. In a grammar they may be exactly right, so judge them afresh.
  - `crates/sound-law` is the user's own sound-change engine. Port it if it serves.
    Whatever the grammar needs from it, add to it rather than working around it.
- **Data:**
  - Kaikki dumps: `~/.cache/flexion/dumps/*.jsonl` (Russian and Spanish also use
    `~/.cache/kaikki/ru/Русский.jsonl` and `~/.cache/kaikki/es/Español.jsonl`; German uses
    `~/.cache/kaikki/de/Deutsch.jsonl`).
  - Dump date: 2026-10-03.
  - Lexeme caches from the old pipeline are in `~/.cache/flexion/`. Use them or not.
- **Baseline to report against:** the `grammar-models` branch, held-out top-1 on the bare
  lemma. Measure = the old compact size measure; source = generated source.

| language | measure | source | noun | adjective | verb |
|---|---:|---:|---:|---:|---:|
| English | 496 KB | 2.9 MB | 97.14% | 96.69% | 97.91% |
| French | 283 KB | 1.3 MB | 97.68% | 97.35% | 99.06% |
| German | 3.5 MB | 14.8 MB | 94.82% | 98.66% | 93.59% |
| Italian | 316 KB | 1.5 MB | 95.86% | 98.39% | 99.28% |
| Spanish | 433 KB | 1.7 MB | 98.72% | 98.56% | 99.42% |
| Polish | 889 KB | 3.7 MB | 94.19% | 99.98% | 97.11% |
| Sanskrit | 705 KB | 2.3 MB | 93.55% | 91.21% | 90.13% |
| Latin | 5.1 MB | 21.4 MB | 68.91% | 87.22% | 75.18% |
| Ancient Greek | 16.2 MB | 45.4 MB | 57.82% | 63.66% | 54.26% |
| Russian | 12.2 MB | 51.6 MB | 84.48% | 96.60% | 92.37% |

  Attested accuracy (forms the dictionary lists, for words the model was trained on) is
  99.5–100% everywhere. The old scoring is in `crates/flexion-train/src/eval.rs` on those
  branches.

## 2. What a generated crate is

It is the language's inflectional grammar as code. A linguist opening `src/` should
recognize the language: its declensions and conjugations, its sound laws, its stress
system, its irregular words. That reader should not see numbered classes or probability
rows. Roughly, and only as a sketch of the spirit:

```rust
/// Second declension: feminines in -а/-я (рука, нога, неделя). 31,004 nouns.
pub const SECOND: Paradigm = paradigm! {
    parent: NOUN,                       // dat/ins/loc pl -ам/-ами/-ах from the root
    sg: [nom "а", gen "ы", dat "е", acc "у", ins "ой", loc "е"],
    pl: [nom "ы", gen "", acc => if animate { gen } else { nom }],
};

/// Which declension a noun takes. Most specific first; each rule with its evidence.
pub fn declension(noun: &Lemma) -> Choice<&'static Paradigm> {
    rules! {
        "-ость" => THIRD,              // 5,120 of 5,124 nouns
        "-а" | "-я" => SECOND,         // 31,004 of 31,390
        …
    }
}

sound_laws! {
    /// к г х → ч ж ш before the diminutive -ек.
    …
}

/// Stress: Zaliznyak's patterns. Lexical in Russian, so "a unless listed".
pub enum Accent { A, B, B1, C, D, D1, E, F, F1, F2 }
static ACCENT: &[(&str, Accent)] = &[("рука", C), ("нога", C), …];

/// What no rule covers, and why.
static IRREGULAR: &[Irregular] = &[ … ];
```

What must be true:

- **Grammar, not weights.**
  - The generated code is made of named, documented linguistic objects: paradigms (ending
    tables, ideally a default-inheritance tree with overrides and referrals for
    syncretism), stems, rules choosing a paradigm, sound laws, a stress or accent system,
    and exceptions.
  - Every rule and table carries its evidence in a comment or a field: how many words it
    covers, how many it gets wrong, an example or two.
  - No suffix trees of numbers, no class ids, no probability tables standing in for
    grammar.
  - A word list is fine where the language is lexical. Russian stress letters, Polish
    genitive *-a/-u*, and principal parts are honest grammar.
- **No in-memory twin.**
  - The trainer learns a grammar value of the same types the crates use (the runtime
    crate defines them) and prints that value as Rust.
  - There is one representation, so no bit-exact audit between two models is needed.
  - There must still be a test that a generated crate answers as the grammar it was
    printed from. By construction is fine, if you can show it.
- **Every word has every form.** The dictionary's silence is not absence. A word whose
  entry lacks a cell gets the form its grammar gives. Where the grammar has no productive
  answer, you still give an answer, marked as such (see confidence).
- **Confidence and candidates, in human terms.**
  - Every query can return its candidates. Each candidate carries:
    - a probability;
    - a status: listed exception / regular from the paradigm / derived by a rule /
      "no productive rule: speakers vary";
    - a reason a person can read: which rule or paradigm, its coverage, a similar word.
  - Example: "*gled*: like *bleed → bled* (6 of 7 verbs in -eed), 0.79; regular *gleeded*
    0.87".
  - Confidence should sharpen when the caller gives more, such as principal parts, gender,
    or a stressed lemma.
  - The probabilities must be calibrated: things said at 0.8 should be right about 80% of
    the time on held-out words.
  - The references give several sound ways to get there (best-rule confidence and
    normalization; the Tolerance Principle; MaxEnt over named constraints; implicative
    entropy).
- **Stress and accent as their own dimension**, not multiplied into paradigms. Russian: the
  Zaliznyak patterns are in Wiktionary's templates as labels. Greek: accent placed by the
  rules (recessive, persistent, contraction, law of limitation).
- **A typed API per language** (enums for case, number, person and so on) as today, plus
  the candidate and explanation API. Show the API before and after as code in the PR.
- **Lean on the type system; write idiomatic Rust.** The types should carry the grammar,
  so a wrong question doesn't compile. In the generated crates, the runtime and the
  trainer alike:
  - grammatical categories are enums: case, number, gender, person, tense, mood, voice,
    aspect, animacy, and the stress pattern;
  - a cell is a typed combination of them, and only combinations the language has exist
    (a Russian noun cell can't take a person);
  - paradigms, stems, rules and sound laws are named items with their own types;
  - a stem slot is not a plain `usize`, and a stressed lemma is not a plain `&str`;
  - use newtypes where a bare string or number would let values mix;
  - make illegal states unrepresentable, and use exhaustive `match` over these enums;
  - use traits for what parts of speech or languages share, and the standard traits
    (`Display`, `FromStr`, `Iterator`, `Default`) where they fit;
  - errors are typed values, not strings;
  - no stringly-typed tags or index arithmetic where a type would do;
  - follow the Rust API guidelines (naming, documentation, `#[must_use]` where it matters,
    no needless allocation);
  - keep the generated code something a Rust programmer would have written by hand, and
    reads with pleasure.
- **Rust standards:**
  - no `unsafe`;
  - no panics in non-test code, generated code included (deny `unwrap`, `expect`, indexing
    and `panic` by clippy lint);
  - no initialization: no `OnceLock`, `LazyLock` or build step before the first answer;
  - deterministic output: the same dumps and configs give byte-identical crates;
  - generated code stays within what rustc handles (see section 5).

## 3. What is measured

Readability is the point. Accuracy, calibration and cost are measured and reported
honestly, not assumed:

- **Held-out accuracy** per language, part of speech and input mode (bare lemma, lemma
  with marks, citation with principal parts).
  - Use lemma-disjoint splits, as the references describe.
  - Report against the baseline table above and against a simple non-neural floor (the
    SIGMORPHON "NonNeur" longest-suffix rule baseline).
  - Losing accuracy to gain readability is allowed. Every point lost needs a stated cause
    and an example, and the size of the exceptions list should be the lever for getting it
    back.
- **Attested accuracy stays ≥ 99%** on the forms the dictionary lists, for the words the
  crate was trained on. This is what the exceptions list is for.
- **Calibration** of the candidate probabilities, on held-out words.
- **The grammar's size in human terms**, per language and part of speech:
  - number of paradigms, rules, sound laws and stress patterns;
  - exceptions listed;
  - lines of generated code;
  - plus generated source bytes, compile time and peak memory, `.rlib` size, and call
    time.
- **Coverage of the grammar:** how many words each paradigm and rule covers, so the reader
  sees that the long tail is short.

## 4. Order of work

Make risk show early:

1. **The grammar types, the trainer's skeleton, the printer, and Russian nouns end to
   end.** That means paradigms, the stress letters, the rules choosing a paradigm, sound
   laws, exceptions, candidates with reasons, and calibration.
   - Put a real excerpt of the generated Russian noun module in the progress log as soon
     as it exists, then keep going.
2. **Latin verbs:** principal parts and stem slots, the perfect and supine stems.
3. **Ancient Greek:** the accent module and the augment and reduplication rules.
4. **The other seven languages,** then every part of speech each config covers.

## 5. Rules for how you work

- No `/tmp`; use `~/.cache/claude-tmp/`.
- **Cap parallelism:** `-j 8`, `--test-threads 4`, at most 8 training threads.
- Nothing over about 10 minutes, nothing using all cores, nothing using lots of RAM,
  without asking. Builds of the ten languages are part of the PR; run them one at a time,
  under the memory cap.
- **Compile generated crates under a hard memory cap:**
  `systemd-run --user --scope -p MemoryMax=16G -p MemorySwapMax=0 cargo build -p russian -j 4`.
  This machine's terminal has been OOM-killed before.
- What rustc taught the last PR:
  - one slice literal or one const-fn call per item, by the hundred thousand, costs rustc
    gigabytes;
  - very deep `match` nesting overflows its stack;
  - a 50,000-arm string `match` is too much.

  A real grammar should be far smaller than the old trees, but watch compile memory
  anyway.
- **Kill everything you started when done.** Never `pkill -f` a pattern from your own
  command line, and don't wait on `pgrep -f` with your own pattern; use PIDs or `pgrep -x`.
- **Tests:** few and meaningful, in a consistent style.
  - No golden, snapshot or pixel-exact tests. They freeze old behavior.
  - Every guard needs a verified failure witness: break the code, see the test fail,
    restore it.
  - At least:
    - a tiny grammar learned and printed as code answers as the grammar value does;
    - a word missing a cell still gets a form;
    - a candidate list carries probabilities and reasons;
    - calibration is checked on fixture data.
- **CI is wiped with the tree; rebuild it** (`.github/workflows`):
  - `cargo test --workspace`;
  - `cargo fmt --all --check`;
  - `cargo clippy --workspace --all-targets -- -D warnings`;
  - a compile-cost job reporting each language crate's build time and peak memory.

  Before pushing, run fmt and clippy locally. Earlier PRs failed CI on formatting.
- **Keep the licenses** (MIT or Apache-2.0) and the language crate names (`english`,
  `russian`, `latin`, `polish`, `german`, `ancient-greek`, `sanskrit`, `spanish`,
  `french`, `italian`). They are published crates.
- **Progress log:** `~/code/notes/projects/flexion/grammar-crates.md`. Dated entries:
  decisions, numbers, what was tried and dropped.
- **Talking to the user:**
  - plain language, no internal labels;
  - before/after code for API changes;
  - a copy-pasteable command starting with `cd ~/code/flexion` for anything to run;
  - before any long wait, say what you are waiting on and roughly how long.

## 6. Git

- Branch `grammar-crates` from the latest `main`; the first commit wipes the tree, as
  described at the top. One PR.
- Merge it with `gh pr merge --merge` once CI is green, then check out the latest `main`.
- Don't publish to crates.io, tag, or make releases.

## 7. Done means

- All ten language crates are generated from the dumps by the new trainer (one command per
  language, documented). They compile, and their README examples pass.
- The crates, the runtime and the trainer are idiomatic Rust that leans on the type system:
  typed grammatical categories and cells, named grammar items, no stringly-typed tags.
  Clippy is clean with `-D warnings`.
- A reader can open any crate's `src/` and find:
  - named paradigms;
  - the rules choosing them, with evidence;
  - sound laws;
  - the stress or accent system where the language has one;
  - an exceptions list, each entry saying what is irregular.
- No probability tables or opaque class ids anywhere.
- No in-memory twin. The test from section 5 shows a crate answers as its grammar.
- Every query can return calibrated candidates with status and reasons. Every word has
  every form.
- Attested accuracy ≥ 99% per part of speech.
- Held-out accuracy reported against the baseline table and the non-neural floor, with
  causes for every loss.
- CI green; deterministic output; no panics; no initialization.
- Docs:
  - the root README explains how a grammar is learned and what a generated crate contains,
    with real excerpts;
  - each crate's README shows its grammar's shape and its numbers.

## 8. Final report

- The PR link.
- **Per language:**
  - the grammar's shape: paradigms, rules, sound laws, stress patterns, exceptions, lines
    of code;
  - real excerpts (a paradigm, a choosing rule, a sound law, the stress system, a few
    exceptions, one candidate list with its reasons).
- **Accuracy per language and part of speech**, against the baseline and the floor, with
  noise. Attested accuracy, calibration, and the cause of every loss.
- **Cost:** generated source, compile time and memory (locally and on CI), `.rlib` size,
  call time.
- **API before/after**, as code.
- **What was tried and didn't work**, with numbers.
- **How to verify:** copy-pasteable commands starting with `cd ~/code/flexion`.
