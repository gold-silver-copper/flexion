# Prompt: add German, Sanskrit and Ancient Greek to flexion

Add three languages to `~/code/flexion` (GitHub `gold-silver-copper/flexion`, branch
`main`): **German**, **Sanskrit** and **Ancient Greek**. Each gets a config in
`configs/`, a generated crate in `languages/`, and the full evaluation report that every
flexion crate carries.

English, Russian, Latin and Polish needed only configs plus generic changes. These
three are the first languages that need things the system can't express yet: material
that changes after a prefix, changes at the start of a word, and an accent that is not
only placed but chosen. The point of this run is to add those as **generic** features
of the trainer (`crates/flexion-train`) and runtime (`crates/flexion`), designed once
for all three languages, not as three special cases. A feature that only one language
could ever use means the design is wrong. Rethink it.

This is one PR. You own it fully: configs, hooks, hand-written extras, trainer and
runtime design, the binary format, tests and docs are yours to decide, and you may
change any API if it makes the project better. The notes below are starting points, not
a spec. Follow the data and the measurements. No prototypes: the PR delivers three
finished crates.

---

## 1. What exists

Read these first:

- `README.md`: architecture, how learning works, how to add a language, and the
  "What the next language would need" section, which sketches this run.
- `prompts/inflection-compiler.md` and `prompts/latin-polish.md`: the earlier prompts.
  Their rules (tests, resources, no panics, reports, ship bars) still apply unless this
  prompt says otherwise.
- `crates/flexion/README.md` and `crates/flexion/src/format.rs`: the model and the
  binary format (version 4).
- `crates/flexion-train/src/spec.rs`: the `Language` description, the `features!`
  macro, `Hooks` (including `repair`), optional groups, `Group::from_head`, inherent
  features, citation markers, and spelling (`fold`, `strip_marks`, `unmark`,
  `marks_setting`).
- `configs/src/latin.rs`: the closest template for Ancient Greek and Sanskrit: marks
  folded for lookup and restored by a learned restoration model, principal-part
  citations, big verb tables split into groups.
- `configs/src/polish.rs`: the template for German (inherent gender, a `repair` hook,
  degree forms from the head line).
- `configs/src/russian.rs`: the stress model, which Ancient Greek's accent extends.
- `languages/latin/README.md`: what a finished report looks like.

How the pieces work now, in short:

- An edit is `prefix + (lemma minus a stripped suffix) + added suffix`, plus a rule
  for where the stress mark goes.
- A class is one edit per cell. A suffix tree over lemmas picks the class.
- Each class has a hidden cell with the lemma written in full. A separate one-cell
  restoration model reads a typed lemma as the lemma written in full (Latin `rex` →
  `rēx`), and forms are weighed over its readings.

Commands:

- `cargo xtask build <lang> --dump <path> --date <YYYY-MM-DD>`: the full pipeline,
  writing `languages/<lang>`.
- `cargo xtask extract <lang> --dump <path>`, then `cargo xtask probe <lang> [--pos …]
  [--alpha …] [--tau …] [--syllables …] [--support …] [--misses 1] [--attested 1]`,
  for experiments.

Dumps go into `~/.cache/flexion/dumps/`, never `/tmp`. The existing ones are dated
2026-10-03. Download the current Kaikki dumps for the three new languages and record
their dates. The URLs follow
`https://kaikki.org/dictionary/<Language>/kaikki.org-dictionary-<Language>.jsonl.gz`;
check the exact name for Ancient Greek on kaikki.org (the space in the language name
is handled differently in the path and in the file name). The German dump is large:
stream it, never load it whole.

Before writing a config, survey each dump. Find out:

- which `pos` values and inflection templates carry tables;
- which tag sets the table forms use;
- what the head templates look like;
- where inherent features (gender, conjugation class, verb stems) live;
- which bookkeeping, romanization, dialect or periphrastic forms leak into tables.

Earlier dumps had such traps: Russian participles filed under `verb` and romanizations
listed as forms; Polish conjugation tables with no first/second person tags and
`error-unrecognized-form` where the nominative was meant; Latin headwords without the
macrons the tables have. Expect similar ones and write them down.

## 2. The generic features this run needs

Design these once, for every language, and say in the final report how each one is
used by more than one language. Keep English, Russian, Latin and Polish exactly as
accurate as they are now (regenerate them and compare with their current READMEs).

- **Prefixes the inflection happens behind.** German separable verbs (`anfangen` →
  `fängt an`, `angefangen`, `anzufangen`), Greek preverbs (`ἀποβάλλω` → `ἀπέβαλον`,
  the augment goes after `ἀπο-` and its vowel elides), Sanskrit prefixes (`saṃgacchati`
  → `samagacchat`). In all three, the word is a prefix plus a verb, the verb inflects,
  and the prefix comes back in front (sometimes changed at the join) or as a separate
  word. Where the prefix list comes from (the data, a config list, Wiktionary's
  etymology or head templates) and how the join is learned are yours to decide. Check
  whether the data's tables already give the separated forms (`fängt an`) as one form
  with a space: the existing configs drop forms with spaces as periphrastic, and
  German must not drop these.
- **Edits at the start of the word.** The Greek augment is not only an added `ἐ-`:
  before a vowel it lengthens the vowel (`ἀκούω` → `ἤκουσα`, `ἐλπίζω` → `ἤλπιζον`).
  Reduplication (`λύω` → `λέλυκα`, Sanskrit `kṛ` → `cakāra`) copies the first
  consonant. So an edit needs to be able to strip and add at the start as well as the
  end, and a class must check that the word starts the way its edits need, as it
  already checks the end. Decide whether copying is a learned edit kind or falls out
  of start-edits learned per initial letter, and measure it.
- **Accent kinds.** The Greek accent moves like Russian stress, but it is also one of
  three marks (acute, grave, circumflex), and which one depends on vowel length and
  position (`δῶρον`, `δώρου`). The stress rule must be able to choose the mark as well
  as the vowel. Keep the design general (a set of marks a language writes on its
  stressed vowel), so it also covers Russian's single mark and Vedic accents if ever
  needed. Whether the grave (which in running text replaces a final acute) is output
  at all is a language decision: say what the Greek config does and why.

If a language needs anything else, the same rule applies: generic, measured, and
explained.

## 3. German

- **Parts of speech:** nouns, adjectives (strong, weak and mixed declension, plus
  comparative and superlative), verbs, pronouns and determiners with tables.
- **Nouns:** four cases × two numbers. Gender is inherent. Plurals are unpredictable,
  which is why dictionaries print them: the citation is `Mann, Männer m` (nominative
  singular, nominative plural, gender as a marker) or, where it helps, also the
  genitive singular (`Mann, -(e)s, Männer`). Decide which parts the citation takes,
  and measure.
- **Adjectives:** case × gender × number, plus the plural, in each of the three
  declension types; comparative and superlative (`schön, schöner, am schönsten`).
- **Verbs:** present, preterite, subjunctive I and II with person × number,
  imperative, infinitive, `zu`-infinitive, present and past participle. Compound tenses
  (`hat gemacht`, `wird machen`) are phrases, not forms. Strong verbs change their
  stem vowel (`singen, sang, gesungen`): the citation is the dictionary's principal
  parts (infinitive, preterite, past participle, optionally the third person singular
  present: `geben, gibt, gab, gegeben`). Separable verbs must come out right: `fängt
  an` in main clauses, and the joined form where the table gives one.
- **Spelling:** nouns are capitalized; lookup should find `Mann` from `mann` and keep
  the capital in the output. Decide how `ß`/`ss` and the umlauts are folded for lookup
  (they are distinct letters in output), and say why.
- Watch for spellings from before the 1996 reform (`daß`), Swiss and Austrian variants,
  and archaic forms (`dem Manne`): decide which are soft or excluded.

## 4. Sanskrit

Target: Classical Sanskrit as Wiktionary's tables give it.

- **Script:** output in Devanagari, the script Wiktionary uses for headwords. Decide
  whether lookup also accepts IAST romanization (`deva`), and, if so, how: this is
  where romanization stops being a trap and becomes a feature. Watch for
  romanizations filed as forms. Devanagari doesn't write the vowel `a` that follows a
  consonant, so anything that counts vowels or syllables (the syllable symbol in the
  tree, the stress rule) must count Devanagari syllables correctly. Make that a
  spelling setting, not a special case.
- **Parts of speech:** nouns, adjectives, verbs, pronouns, numerals with tables.
- **Nouns and adjectives:** eight cases × three numbers (singular, dual, plural).
  Gender is inherent for nouns. Wiktionary's lemma is often the stem (`देव`, cited as
  `deva-`), not the nominative: decide what the lemma is and say so in the docs.
- **Verbs:** whatever tense systems the tables give (present system with its ten
  classes, aorist, perfect, future, passive, and so on), person × number × voice
  (active, middle, passive). Expect partial tables: many verbs have only some tenses,
  so most groups are optional. The citation is the root plus the present third person
  singular (`गम्, गच्छति`), or whatever the dictionaries and the data support best:
  decide and measure.
- **Sandhi:** forms within a word already have their internal sandhi applied. External
  sandhi, between words, is out of scope.
- Vedic forms and accents: decide whether to exclude them or keep them as soft, and
  say why.
- **Data volume is the main risk.** Count the entries with full tables before writing
  the config. If a part of speech has too few lemmas for a meaningful held-out
  evaluation, say so in the report, still ship it with attested accuracy ≥99%, and
  don't pretend.

## 5. Ancient Greek

Target: Attic Greek as Wiktionary's tables give it, polytonic.

- **Parts of speech:** nouns, adjectives (with degree forms where attested), verbs,
  pronouns and articles, numerals that decline.
- **Nouns:** five cases (nominative, genitive, dative, accusative, vocative) × three
  numbers (singular, dual, plural). Gender is inherent. The citation is the dictionary
  format: `λόγος, λόγου m`.
- **Adjectives:** case × gender × number, two- and three-ending types.
- **Verbs:** the full table Wiktionary gives: present, imperfect, future, aorist,
  perfect, pluperfect, future perfect; active, middle, passive; indicative,
  subjunctive, optative, imperative; infinitives and participles (citation forms; they
  decline as adjectives). Wiktionary splits verb tables by tense stem: expect most
  verbs to have only some of them, so most groups are optional. The citation is the
  six principal parts (`λύω, λύσω, ἔλυσα, λέλυκα, λέλυμαι, ἐλύθην`), any subset in
  order. Deponent verbs and contracted verbs (`τιμάω` → `τιμῶ`) must come out right.
- **Spelling:**
  - Breathings, accents, iota subscript, diaeresis and vowel-length marks
    (macron/breve, which some tables write) are part of the written form.
  - Lookup should find a word typed without accents or breathings (`logos` in Greek
    letters, `λογος`), and the output keeps them, with settings to drop the accent
    and the length marks.
  - Final sigma `ς`/`σ` is one letter for lookup.
  - Decide whether Latin-letter transliteration is accepted for lookup, as for
    Sanskrit.
- **Dialects:** Epic/Homeric, Ionic, Doric, Aeolic and Koine forms are not Attic.
  Decide which are soft or excluded.
- **Size:** dual numbers and big verb tables make this the largest model so far
  (Latin is 6.8 MB). Keep it under crates.io's 10 MB limit, and measure what the
  format can do about it if it doesn't fit.

## 6. Done means

For each language, the same bar as the existing four:

- every cell of every inflecting part of speech the language has, through a typed API
  and the scored functions, with citations and markers;
- the full report in the crate's README, generated by `cargo xtask build`;
- **attested primary-form accuracy at least 99%** for every part of speech with more
  than a handful of lemmas;
- calibration reasonable (ECE comparable to English and Russian) from the bare lemma
  and from the citation;
- packaged under crates.io's 10 MB limit;
- sub-microsecond calls (speed matters less than accuracy in this run, but don't let
  it grow past a microsecond for any language, old or new);
- held-out accuracy reported honestly from the bare lemma and from the dictionary
  citation (and with markers);
- made-up words in the report (a few per part of speech, judged by eye);
- an end-to-end test on a small committed fixture of real dump lines, like the
  existing ones in `configs/tests/`, covering the new generic features (a separable
  verb, an augmented and a reduplicated form, a circumflex accent, a Devanagari form);
- hand-written extras only where Wiktionary's data is broken or missing, documented in
  the config with the reason (as for the Russian personal pronouns).

**Aims** (top-1 on held-out words given the citation): German nouns ≥97% given
singular, plural and gender; German verbs ≥97% given the principal parts; Sanskrit
nouns ≥95% given the lemma and gender; Ancient Greek nouns ≥95% given nominative,
genitive and gender; Ancient Greek verbs ≥90% given the principal parts the data has.
If an aim is missed, explain which information the misses need and whether any data
could supply it.

Across the repo:

- English, Russian, Latin and Polish numbers don't get worse. Regenerate them if a
  generic change touches the trainer or runtime, and compare with their current
  READMEs;
- the root README's results table, its "How it learns" section (for the new edit
  kinds and accent marks), and its "What the next language would need" section are
  updated;
- `crates/flexion/README.md` and `format.rs` describe the new model features;
- `cargo fmt`, `cargo clippy --workspace --all-targets -- -D warnings` and the
  workspace tests pass;
- generated crates are deterministic (same dump + config → byte-identical model) and
  formatted by the build.

## 7. Rules for how you work

All the rules in section 7 of `prompts/inflection-compiler.md` apply. In short:

- No `/tmp`.
- Cap parallelism (`-j 8`, `--test-threads 4`).
- Stream dumps.
- Nothing over ~10 minutes without asking.
- No panics in non-test code.
- Few meaningful tests, no golden files.
- Keep a progress log at `~/code/notes/projects/flexion/german-sanskrit-greek.md`.
- Kill everything you started when done.

The machine is often busy with other work. Benchmark with the fastest-of-several method
`fastest_ns` already uses, compare old and new code in one interleaved run, and don't
trust a single timing. When a wait loop matches processes by name, make sure it can't
match its own command line.

## 8. Git

- Work on a branch, open one PR, and merge it with `gh pr merge --merge` once CI is
  green. Then check out the latest `main`.
- Don't publish to crates.io, create tags or releases. Leave the new crates ready to
  publish. The names `german`, `sanskrit` and `ancient-greek` were free on crates.io on
  2026-10-09. Check again, and pick other names and say so if one has been taken.
  Prefer `ancient-greek` to `greek`, which suggests Modern Greek.

## 9. Final report

- PR link.
- Per language, the headline table: held-out top-1 per part of speech (bare lemma,
  citation, citation and markers), attested accuracy, calibration, size, speed. Plus
  whether each bar and aim was met.
- Every trainer or runtime change: what it does, with before/after code for any API
  change, which of the languages use it (and why it is generic), and the before/after
  numbers of the four existing languages.
- The data traps found in each dump and how the config handles them.
- The API for each language, with a few real examples.
- The biggest remaining error groups per language.
- How you verified it, and copy-pasteable commands starting with `cd ~/code/flexion`
  to inflect a few words in each new language, rebuild each crate, and rerun its
  evaluation.
