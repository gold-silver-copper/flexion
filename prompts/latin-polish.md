# Prompt: add Latin and Polish to flexion

Add two languages to `~/code/flexion` (GitHub `gold-silver-copper/flexion`, branch
`main`): **Latin** and **Polish**. Each gets a config in `configs/`, a generated crate
in `languages/`, and the full evaluation report that every flexion crate carries.

The point of this run is to test whether the system is really generic. Both
languages should need **only configs**: no language-specific code in the trainer
(`crates/flexion-train`) or the runtime (`crates/flexion`). If a language does need
a trainer or runtime change, make it generic (it must make sense for any
language), keep English and Russian at least as good as they are now, and explain
the change in the final report. A change that only one language could ever use
means the config is the wrong place to stop. Rethink the design rather than add a
special case.

This is one PR. You own it fully: configs, hooks, hand-written extras, generic
trainer and runtime changes, tests and docs are yours to decide. The notes below
are starting points, not a spec. Follow the data and the measurements. No
prototypes: the PR delivers two finished crates.

---

## 1. What exists

Read these first:

- `README.md`: architecture, how learning works, how to add a language.
- `prompts/inflection-compiler.md`: the original prompt. Its rules (tests,
  resources, no panics, reports, ship bars) still apply unless this prompt says
  otherwise.
- `configs/src/russian.rs` (with `configs/extra/russian.rs`): the closest
  template for Polish, and the model for Latin's case system.
- `configs/src/english.rs`: the model for hooks and periphrastic forms.
- `languages/russian/README.md`: what a finished report looks like.
- `crates/flexion-train/src/spec.rs`: the `Language` description, the
  `features!` macro, `Hooks`, optional groups, inherent features and citation
  markers.

Commands: `cargo xtask build <lang> --dump <path> --date <YYYY-MM-DD>` (the full
pipeline, writing `languages/<lang>`); `cargo xtask extract <lang> --dump <path>`
then `cargo xtask probe <lang> [--pos …] [--alpha …] [--tau …] [--syllables …]
[--support …] [--misses 1] [--attested 1]` for experiments.

Dumps (2026-10-03), into `~/.cache/flexion/dumps/`, never `/tmp`:

- `https://kaikki.org/dictionary/Latin/kaikki.org-dictionary-Latin.jsonl.gz` (~108 MB)
- `https://kaikki.org/dictionary/Polish/kaikki.org-dictionary-Polish.jsonl.gz` (~83 MB)

Before writing a config, survey each dump: which `pos` values and inflection
templates carry tables, which tag sets the table forms use, what the head
templates look like, where inherent features (gender, conjugation, aspect) live,
and which bookkeeping or romanization forms leak into tables. The Russian dump
had several such traps: participles filed under `verb`, romanizations as forms,
pre-reform spellings tagged `dated`, and a pronoun table without number tags.
Expect similar ones.

## 2. Latin

Target: Classical Latin as Wiktionary's tables give it (they mark long vowels with
macrons).

- **Parts of speech:** nouns, adjectives (with comparative and superlative
  where attested), verbs, pronouns and determiners with tables, numerals that
  decline.
- **Nouns:** six cases (nominative, genitive, dative, accusative, ablative,
  vocative) × two numbers, and the locative where attested (an optional group).
  Gender is inherent.
- **Adjectives:** case × gender × number; first/second- and third-declension
  types; degree forms where the tables give them.
- **Verbs:** the full table Wiktionary gives: active and passive voice;
  indicative and subjunctive in every tense; imperative; infinitives;
  participles (citation forms; they decline as adjectives); gerund, gerundive
  and supine. Deponent verbs (passive forms with active meaning) and defective
  verbs must come out right. Decide how the cells are grouped into functions so
  the API stays readable (cell structs for big groups).
- **Spelling:** macrons are part of the written form. Fold them for lookup, so
  `rosa` finds the attested word, but keep them in the output, with a setting to
  drop them (as Russian has for stress). Decide how to treat `j`/`i` and `v`/`u`
  spelling variants, and say why.
- **Citations:** the dictionary format: nouns `rēx, rēgis m.` (nominative,
  genitive, and gender as a marker); verbs the four principal parts
  `amō, amāre, amāvī, amātum`; adjectives `bonus, bona, bonum` or `fortis, forte`.
  Note that a Latin verb's lemma is its first-person singular present, not its
  infinitive.

## 3. Polish

- **Parts of speech:** nouns, adjectives (with degree forms), verbs, pronouns
  and numerals with tables.
- **Nouns:** seven cases (nominative, genitive, dative, accusative,
  instrumental, locative, vocative) × two numbers. Inherent gender with the
  three-way masculine (personal, animate, inanimate), feminine, neuter.
- **Agreement:** adjectives, past-tense verbs and participles agree in the
  singular by gender, and in the plural by **masculine personal vs. everything
  else** (`dobrzy panowie`, `dobre psy`). Model it as the data and Polish grammar
  have it, not as a copy of Russian's agreement feature.
- **Verbs:** present (imperfective) / future (perfective) non-past;
  past tense with person and gender (`czytałem`, `czytałaś`, `czytaliśmy`);
  conditional; imperative; infinitive; impersonal forms; participles (decline as
  adjectives); adverbial participles; verbal noun. Aspect is inherent.
  Periphrastic forms (imperfective future `będę czytać`) are not forms of the verb.
- **Spelling:** no stress model: stress is fixed and not written. Polish
  diacritics (`ą ę ó ł ż ź ć ń ś`) are distinct letters. Don't fold them for
  lookup.
- **Citations:** nouns nominative + genitive singular (`dom, domu`). The
  masculine genitive `-a`/`-u` is unpredictable, which is exactly why
  dictionaries print it. Verbs: infinitive + first and second person singular
  non-past, plus an aspect marker.

## 4. Done means

For each language, the same bar as English and Russian:

- every cell of every inflecting part of speech the language has, through a
  typed API and the scored functions, with citations and markers;
- the full report in the crate's README, generated by `cargo xtask build`;
- **attested primary-form accuracy at least 99%** for every part of speech with
  more than a handful of lemmas;
- calibration reasonable (ECE comparable to English and Russian);
- packaged under crates.io's 10 MB limit;
- sub-microsecond calls;
- held-out accuracy reported honestly from the bare lemma and from the
  dictionary citation (and with markers). **Aim** for at least 97% top-1 on
  Latin nouns and verbs given the citation, and at least 95% on Polish nouns given
  nominative + genitive. If an aim is missed, explain which information the
  misses need and whether any data could supply it;
- made-up words in the report (a few per part of speech, judged by eye);
- an end-to-end test on a small committed fixture of real dump lines, like the
  existing ones in `configs/tests/`;
- hand-written extras only where Wiktionary's data is broken or missing,
  documented in the config with the reason (as for the Russian personal
  pronouns).

Across the repo:

- English and Russian numbers don't get worse. Regenerate them if a generic
  change touches the trainer or runtime, and compare with their current
  READMEs;
- the root README's results table and "what a third language would need"
  section are updated;
- `cargo fmt`, `cargo clippy --workspace --all-targets -- -D warnings` and the
  workspace tests pass;
- generated crates are deterministic (same dump + config → byte-identical
  model) and formatted by the build.

## 5. Rules for how you work

All the rules in section 7 of `prompts/inflection-compiler.md` apply. In short:

- No `/tmp`.
- Cap parallelism (`-j 8`, `--test-threads 4`).
- Stream dumps.
- Nothing over ~10 minutes without asking.
- No panics in non-test code.
- Few meaningful tests, no golden files.
- Keep a progress log at `~/code/notes/projects/flexion/latin-polish.md`.
- Kill everything you started when done.

Beware that the machine is often busy with other work: benchmark with the
fastest-of-several method `fastest_ns` already uses, and don't trust a single
timing.

## 6. Git

- Work on a branch, open one PR, and merge it with `gh pr merge --merge` once CI
  is green. Then check out the latest `main`.
- Don't publish to crates.io, create tags or releases. Leave the new crates
  ready to publish (`latin` and `polish` 0.1.0 if those names are free on
  crates.io; otherwise pick names and say so).

## 7. Final report

- PR link.
- Per language, the headline table: held-out top-1 per part of speech (bare
  lemma, citation, citation and markers), attested accuracy, calibration, size,
  speed. Plus whether each bar and aim was met.
- Every trainer or runtime change, with why it's generic, and English/Russian
  before/after numbers if it touched them.
- The data traps found in each dump and how the config handles them.
- The API for each language, with a few real examples.
- The biggest remaining error groups per language.
- How you verified it, and copy-pasteable commands starting with
  `cd ~/code/flexion` to inflect a few Latin and Polish words, rebuild each
  crate, and rerun its evaluation.
