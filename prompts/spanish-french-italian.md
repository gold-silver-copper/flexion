# Prompt: add Spanish, French and Italian to flexion

Add three languages to `~/code/flexion` (GitHub `gold-silver-copper/flexion`, branch
`main`): **Spanish**, **French** and **Italian**. Each gets a config in `configs/`, a
generated crate in `languages/`, and the full evaluation report that every flexion crate
carries.

Seven languages are already built from the same trainer and runtime, with no
language-specific code in either. These three should need **only configs**. If one does
need a trainer or runtime change, make it generic (it must make sense for any language),
keep the seven existing languages at least as good as they are now, and explain the
change in the final report. A change that only one language could ever use means the
design is wrong. Rethink it.

This is one PR. You own it fully: configs, hooks, hand-written extras, generic trainer
and runtime changes, the binary format, tests and docs are yours to decide, and you may
change any API if it makes the project better. The notes below are starting points, not
a spec. Follow the data and the measurements. No prototypes: the PR delivers three
finished crates.

---

## 1. What exists

Read these first:

- `README.md`: architecture, how learning works, how to add a language, and the
  "What the next language would need" section.
- `prompts/inflection-compiler.md`, `prompts/latin-polish.md` and
  `prompts/german-sanskrit-greek.md`: the earlier prompts. Their rules (tests,
  resources, no panics, reports, ship bars) still apply unless this prompt says
  otherwise.
- `crates/flexion/README.md` and `crates/flexion/src/format.rs`: the model and the
  binary format (version 5).
- `crates/flexion-train/src/spec.rs`: the `Language` description, the `features!`
  macro, `Hooks` (including `repair`), optional groups, `Group::from_head`, inherent
  features, citation markers, capitalized parts of speech, and spelling (`fold`,
  `fold_marks`, `strip_marks`, `unmark`, `unmark_marks`, stress marks).
- `configs/src/german.rs`: the closest template (a Western European language with
  inherent gender, degree forms, a big verb table, a `repair` hook, joins for prefixed
  verbs, and hand-written personal pronouns in `configs/extra/german.rs`).
- `configs/src/latin.rs`: marks folded for lookup and restored by a learned model.
- `languages/german/README.md`: what a finished report looks like.

How the pieces work now, in short:

- An edit strips and adds at both ends of the lemma, with a stress rule. Each
  (lemma, form) pair takes the reading the most pairs share.
- A class is one edit per cell. A suffix tree over lemmas picks the class.
- A **restoration model** reads a typed lemma as the lemma written in full (Latin `rex`
  → `rēx`).
- A **join model** inflects unseen words made of a known prefix plus an attested word
  through that word (German `anstehen` → `steht an`).
- **Part models** build cells from given principal parts, where that beats the main
  model on the tuning tenth.
- The pipeline adopts joins and part models only when they do better on the tuning
  tenth beyond the noise, and without worse letters or calibration.

Commands:

- `cargo xtask build <lang> --dump <path> --date <YYYY-MM-DD>`: the full pipeline,
  writing `languages/<lang>`.
- `cargo xtask extract <lang> --dump <path>`, then `cargo xtask probe <lang> [--pos …]
  [--alpha …] [--tau …] [--syllables …] [--support …] [--joins 1] [--parts 1]
  [--misses 1] [--attested 1]`, for experiments. With `--misses 1` the probe also
  prints a per-group breakdown.

Dumps (2026-10-03, 60–95 MB gzipped each), into `~/.cache/flexion/dumps/`, never `/tmp`:

- `https://kaikki.org/dictionary/Spanish/kaikki.org-dictionary-Spanish.jsonl.gz`
- `https://kaikki.org/dictionary/French/kaikki.org-dictionary-French.jsonl.gz`
- `https://kaikki.org/dictionary/Italian/kaikki.org-dictionary-Italian.jsonl.gz`

The existing languages use dumps of the same date. Stream the dumps; never load one
whole.

Before writing a config, survey each dump. Find out:

- which `pos` values and inflection templates carry tables;
- which tag sets the table forms use;
- what the head templates look like;
- where inherent features (gender, conjugation class) live;
- which bookkeeping, dialect, archaic, periphrastic or clitic-attached forms leak into
  tables.

Earlier dumps had such traps:

- German adjective forms written with their article;
- Sanskrit tenses named only in section headers;
- Greek tables labelled by dialect only in their header;
- Polish verb tables with untagged persons.

Expect similar ones here. Look in particular at whether the Romance tables list forms
with clitic pronouns attached (`dámelo`, `dímelo`), reflexive forms (`se laver`,
`lavarse`), compound tenses, and regional variants (voseo, Latin American and
peninsular forms), and decide which are cells, which are soft, and which are left out.

## 2. Spanish

- **Parts of speech:** nouns, adjectives (with the superlative in `-ísimo` where
  attested; comparatives are periphrastic), verbs, pronouns and determiners with tables,
  numerals that inflect.
- **Nouns:** singular and plural. Gender is inherent, including nouns of both genders
  and invariable plurals (`la crisis, las crisis`). The written accent comes and goes
  with the plural (`canción` → `canciones`, `joven` → `jóvenes`): it is part of the
  letters, and the edits must produce it.
- **Adjectives:** gender × number, including the forms that shorten before a noun
  (`buen`, `gran`) if the data gives them.
- **Verbs:** the full table Wiktionary gives:
  - indicative present, imperfect, preterite, future and conditional;
  - subjunctive present, imperfect (both the `-ra` and the `-se` series), and future;
  - imperative (affirmative, and negative where the table gives it);
  - infinitive, gerund, past participle (with its gender and number forms where the
    table gives them).

  Compound tenses (`he hablado`) are phrases. Decide how `vos` and `vosotros` forms
  enter the cells. Stem-changing verbs (`contar` → `cuento`, `pedir` → `pido`), orthographic
  changes (`sacar` → `saqué`, `coger` → `cojo`) and irregular verbs must come out
  right, and verbs with prefixes should inflect through their base (`deshacer` →
  `deshice` like `hacer` → `hice`).
- **Spelling:** written accents (`á é í ó ú`) and `ñ`, `ü` are part of the written form.
  Decide whether lookup folds the accents (people often leave them out: `cancion`) and
  restores them with the restoration model, and say why. `ñ` is a letter of its own.
- **Citations:** Spanish dictionaries cite only the infinitive. Decide whether a
  citation with principal parts (for example infinitive, first person present,
  preterite, participle: `hacer, hago, hice, hecho`) is worth offering, measure it, and
  document what the crate accepts. Nouns take gender as a marker (`mapa m`).

## 3. French

- **Parts of speech:** nouns, adjectives, verbs, pronouns and determiners with tables,
  numerals that inflect.
- **Nouns:** singular and plural, with irregular plurals (`cheval` → `chevaux`,
  `œil` → `yeux`) and invariable ones. Gender is inherent.
- **Adjectives:** masculine and feminine, singular and plural, plus the masculine
  singular used before a vowel (`beau` → `bel`, `nouveau` → `nouvel`, `vieux` →
  `vieil`) where the data gives it.
- **Verbs:** the full table Wiktionary gives:
  - indicative present, imperfect, passé simple, future and conditional;
  - subjunctive present and imperfect;
  - imperative;
  - infinitive, present participle, past participle (with its gender and number forms).

  Compound tenses (`j'ai parlé`) are phrases. Spelling changes inside the stem
  (`appeler` → `appelle`, `acheter` → `achète`, `manger` → `mangeons`, `placer` →
  `plaçons`) and irregular verbs (`aller`, `être`, `faire`, `prendre`) must come out
  right, and prefixed verbs should inflect through their base (`refaire`, `défaire`
  like `faire`; `reprendre` like `prendre`). Decide how pronominal verbs (`se laver`,
  `s'en aller`) are cited and inflected.
- **Spelling:** accents (`é è ê ë à â î ï ô û ù ç`) and ligatures (`œ`, `æ`) are part of
  the written form. Decide what lookup folds, and say why. Elision (`j'`, `l'`) belongs
  to the sentence, not the word; keep it out.
- **Citations:** dictionaries cite the infinitive. As for Spanish, decide whether
  principal parts help (for example infinitive, first person present, passé simple,
  participle: `prendre, prends, pris, pris`), measure, and document it.

## 4. Italian

- **Parts of speech:** nouns, adjectives (with superlatives in `-issimo` where attested),
  verbs, pronouns and determiners with tables, numerals that inflect.
- **Nouns:** singular and plural, with irregular and mixed-gender plurals (`uomo` →
  `uomini`, `uovo` → `uova`, `braccio` → `braccia`) and invariable ones (`città`, `re`).
  Gender is inherent.
- **Adjectives:** gender × number, plus the forms that shorten before a noun (`bello` →
  `bel`, `begli`, `bei`; `buono` → `buon`) where the data gives them.
- **Verbs:** the full table Wiktionary gives:
  - indicative present, imperfect, passato remoto, future and conditional;
  - subjunctive present and imperfect;
  - imperative;
  - infinitive, gerund, present and past participle (with its gender and number forms).

  Compound tenses are phrases. `-isc-` verbs (`finire` → `finisco`), irregular passati
  remoti (`prendere` → `presi`, `scrivere` → `scrissi`) and irregular verbs must come
  out right, and prefixed verbs should inflect through their base (`rifare` like
  `fare`).
- **Spelling:** Italian writes accents only on final stressed vowels (`città`, `perché`)
  and the grave/acute distinction on `e`/`o` there. Stress elsewhere is not written: no
  stress model. Decide what lookup folds.
- **Citations:** Italian dictionaries print the infinitive and, for irregular verbs,
  the first person present, the passato remoto and the participle (`prendere, prendo,
  presi, preso`): use them as principal parts if they help, and measure. Nouns take
  gender as a marker.

## 5. Done means

For each language, the same bar as the existing seven:

- every cell of every inflecting part of speech the language has, through a typed API
  and the scored functions, with citations and markers;
- the full report in the crate's README, generated by `cargo xtask build`;
- **attested primary-form accuracy at least 99%** for every part of speech with more
  than a handful of lemmas;
- calibration reasonable (ECE comparable to English and German);
- packaged under crates.io's 10 MB limit;
- sub-microsecond calls;
- held-out accuracy reported honestly from the bare lemma and from the citation (and
  with markers);
- made-up words in the report (a few per part of speech, judged by eye);
- an end-to-end test on a small committed fixture of real dump lines, like the existing
  ones in `configs/tests/`, covering for each language at least an irregular plural, a
  stem-changing or irregular verb, and a prefixed verb inflected through its base;
- hand-written extras only where Wiktionary's data is broken or missing, documented in
  the config with the reason (as for the German personal pronouns).

**Aims** (top-1 on held-out words): from the bare lemma, nouns ≥97% and adjectives ≥98%
in all three languages, and verbs ≥95%; verbs given principal parts, if you offer them,
≥98%. Spanish and Italian plurals are nearly regular, so their nouns should do better
than the aim; French plurals are written by rules with many exceptions. If an aim is
missed, explain which information the misses need and whether any data could supply it.

Across the repo:

- the numbers of English, Russian, Latin, Polish, German, Ancient Greek and Sanskrit
  don't get worse. Regenerate them if a generic change touches the trainer or runtime,
  compare each table with its current README, and account for every number that moves.
  A change of a few hundredths of a point still needs a cause in the report;
- the root README's results table, "How it learns" (if the design changes) and "What the
  next language would need" sections are updated;
- `crates/flexion/README.md` and `format.rs` describe any new model features;
- `cargo fmt`, `cargo clippy --workspace --all-targets -- -D warnings` and the
  workspace tests pass;
- generated crates are deterministic (same dump + config → byte-identical model) and
  formatted by the build.

## 6. Rules for how you work

All the rules in section 7 of `prompts/inflection-compiler.md` apply. In short:

- No `/tmp`.
- Cap parallelism (`-j 8`, `--test-threads 4`).
- Stream dumps.
- Nothing over ~10 minutes without asking.
- No panics in non-test code.
- Few meaningful tests, no golden files.
- Keep a progress log at `~/code/notes/projects/flexion/spanish-french-italian.md`.
- Kill everything you started when done.

Lessons from the last run:

- **Shared machine.** The machine is often busy with other work. Benchmark with the
  fastest-of-several method `fastest_ns` already uses, compare old and new code in one
  interleaved run, and don't trust a single timing.
- **Wait loops.** When a wait loop matches processes by name, make sure it can't match
  its own command line (or the shell running it): write PIDs to a file instead.
- **Pruning must match the runtime.** The trainer's copy of the runtime's choice (in
  `learn.rs`) decides which words keep their own node. Any runtime change to how a
  class is chosen must be mirrored there, or attested accuracy drops quietly.
- **Rebuilding mid-run.** Configs are compiled into the build tool. Rebuild it before a
  language's build starts, not while a queue of builds is running, or the queue mixes
  binaries.

## 7. Git

- Work on a branch, open one PR, and merge it with `gh pr merge --merge` once CI is
  green. Then check out the latest `main`.
- Don't publish to crates.io, create tags or releases. Leave the new crates ready to
  publish. The names `spanish`, `french` and `italian` were free on crates.io on
  2026-10-10; check again, and pick other names and say so if one has been taken.

## 8. Final report

- PR link.
- Per language, the headline table: held-out top-1 per part of speech (bare lemma,
  citation, citation and markers), attested accuracy, calibration, size, speed. Plus
  whether each bar and aim was met.
- Every trainer or runtime change: what it does, with before/after code for any API
  change, which languages use it (and why it is generic), and the before/after numbers
  of the seven existing languages.
- The data traps found in each dump and how the config handles them.
- The decisions this prompt leaves open (accent folding, clitic and reflexive forms,
  regional variants, principal parts), what each config does, and why.
- The API for each language, with a few real examples.
- The biggest remaining error groups per language.
- How you verified it, and copy-pasteable commands starting with `cd ~/code/flexion`
  to inflect a few words in each new language, rebuild each crate, and rerun its
  evaluation.
