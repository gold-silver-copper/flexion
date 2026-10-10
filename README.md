# flexion

Describe a language's grammar, point the trainer at a [Kaikki](https://kaikki.org/)
Wiktionary dump, and get a Rust crate that inflects **any** word of that language
(attested or not) with a calibrated confidence for every answer.

This repository builds ten languages from the same trainer, with no language-specific
code in the trainer or the runtime:

| crate | held-out words (never seen in training), top-1 | attested words, top-1 |
|---|---|---|
| [`english`](languages/english) | nouns 97.15%, verbs 97.91% (99.53% given the past tense), adjectives 96.69%, adverbs 99.00% (old hand-written rules: 95.76%, 92.51%, 96.15%, 99.00%) | 99.96–100% (`english` 0.4: 98.89–99.43%) |
| [`russian`](languages/russian) | from the bare lemma: nouns 84.5%, adjectives 96.6% (participles are stressed as their verbs' tables write them), verbs 91.9% (stress is the hard part: letters alone are right 95.1%, 99.8%, 97.3%); given the dictionary's principal parts: nouns 95.3% (97.4% with gender/animacy markers), verbs 98.9%; with a stressed lemma, nouns 93.2%, adjectives 99.8% | 99.5–100% |
| [`latin`](languages/latin) | given the dictionary's principal parts: nouns 98.1% (98.5% with the gender marker), adjectives 99.6%, verbs 99.6%; from the bare lemma 59.1%, 82.0% (participles take their vowel length from their verbs' tables), 66.6% (vowel length is the hard part: letters alone are right 93.3%, 99.4%, 90.3%) | 99.6–100% |
| [`polish`](languages/polish) | from the bare lemma: nouns 93.9%, adjectives 99.98%, verbs 97.3%; given the principal parts: nouns 96.3% (98.2% with gender markers), verbs 98.4% | 99.9–100% |
| [`german`](languages/german) | from the bare lemma: nouns 94.9%, adjectives 99.1%, verbs 93.9% (separable verbs inflect through their base: `anstehen` → `steht an`); given the principal parts: nouns 98.4% (98.7% with the gender marker), verbs 99.2% | 99.9–100% |
| [`ancient-greek`](languages/ancient-greek) | given the dictionary's principal parts: nouns 96.0%, adjectives 98.5%, verbs 73.8%; with the lemma written in full (accents and vowel length): 81.4%, 86.8%, 53.6%; from the headword (accents, no vowel length): 42.6%, 47.0%, 35.8% (letters right 92.6%, 94.5%, 63.6%) | 99.8–100% (pronouns 97.9%) |
| [`sanskrit`](languages/sanskrit) | from the stem: nouns 93.0% (97.2% with the gender marker), adjectives 91.5%, verbs 91.0% (92.4% given future, perfect and aorist) | 99.9–100% |
| [`spanish`](languages/spanish) | from the bare lemma: nouns 98.7%, adjectives 98.6%, verbs 99.4% (prefixed verbs inflect like their base: `deshacer` → `deshizo`); given the principal parts: verbs 99.9% | 99.2–100% |
| [`french`](languages/french) | from the bare lemma: nouns 97.5%, adjectives 97.7%, verbs 99.1%, past participle agreement 99.7%; given the principal parts: verbs 99.8% | 99.99–100% |
| [`italian`](languages/italian) | from the bare lemma: nouns 95.9% (96.9% with the gender marker), adjectives 98.4%, verbs 99.2%, past participle agreement 100%; given the principal parts: verbs 99.7% | 99.99–100% |

Attested words are the ones the shipped model was trained on. The full reports (calibration,
per-cell numbers, made-up words, the largest groups of mistakes, size and speed) are in
each crate's README. The English model is 832 KB against `english` 0.4's 1026 KB of tables,
and a call takes about 130–160 ns, about twice as long as `english` 0.4's. The other models
are 0.3–7.4 MB (0.1–2.6 MB gzipped), except Russian's, 21.4 MB (7.7 MB gzipped: four times
the words, and the participle lexicon), and Ancient Greek's, 20.1 MB (3.9 MB gzipped), all
under crates.io's 10 MB limit for a packaged crate. Calls take 100–600 ns (Spanish, French
and Italian 100–250 ns), except Ancient Greek's: 450–900 ns for nouns and adjectives and
1.4 µs for verbs, whose 600-cell classes rarely fit an unseen verb at a deep node.

Russian learns from the Russian Wiktionary too, four times the words of the English one.
From the bare lemma, the hard part is where the stress falls. A participle typed plain
(`ахнувший`) takes its stress from its verb's table, so adjectives reach 96.6%. Nouns
(84.5%) and verbs (91.9%) mostly miss on a lemma's stress that nothing in the spelling
shows, and on animacy, which decides whether the accusative looks like the genitive
(`адвоката`) or the nominative (`айсберг`). Given the nominative and genitive singular,
nouns reach 95.3%; with the markers a dictionary prints (`кот, кота m anim`), 97.4%.

A Latin lemma as people type it (`rosa`, `rex`) doesn't say which vowels are long, and
for a word the model never saw that can't be told from the spelling. So a bare unseen
lemma gets its letters right 90–99% of the time, but its macrons only 59% (nouns) to 67%
(verbs). Adjectives reach 82.0%, because many are participles whose verbs' tables write
them in full (`amatus` → `amātus`). The confidences say so, and the second guess is often
the right one: top-3 is 73–88%. Given the citation a dictionary prints (`rēx, rēgis m.`,
`amō, amāre, amāvī, amātum`), whose parts spell the stem in full, nouns reach 98.1% and
verbs 99.6%.

Ancient Greek is the same story twice over: a headword like `λύω` writes the accent but
not the vowel length the tables mark (`λῡ́ω`), so even its letters are right far more
often than its marks (participles take theirs from their verbs, so adjectives reach
47.0%). Given the principal parts, nouns reach 96.0% but verbs only 73.8% (the aim was
85%). A verb's principal parts each build their own tenses (the part models); what the
misses still need is an accent that depends on vowel lengths the parts don't always show
(`ἠγνοήμεθα`, `ἠγνοῆσθαι`, where the model writes `ἠγνόημεθα`, `ἠγνοήσθαι`), in cells that
only a few of the 2,700 verbs with tables attest. No other Wiktionary has Greek verb
tables the trainer could use.

Spanish, French and Italian need only their configs. Their remaining misses are mostly
words that don't inflect although their ending says they would: French adjectives and
Italian nouns that are invariable (`antisida`, `abbassalingua`, `allerta`), which nothing
in the spelling marks, keep French adjectives at 97.7% and Italian nouns at 95.9% (the aims
were 98% and 97%). Italian `-a` nouns also need their gender: masculine ones take `-i` (`il poeta` →
`poeti`), feminine ones `-e` (`la casa` → `case`); with the gender marker, Italian nouns
reach 96.9%.

```rust
assert_eq!(english::noun("child", english::Number::Plural), "children");
assert_eq!(english::noun("glorp", english::Number::Plural), "glorps");
assert_eq!(english::verb("lie, lied", english::VerbForm::Past), "lied");

use russian::{Case, Number};
assert_eq!(russian::noun("книга", Case::Genitive, Number::Plural), "кни́г");
assert_eq!(russian::noun("кот, кота m anim", Case::Accusative, Number::Singular), "кота́");

assert_eq!(latin::noun("rex", latin::Case::Genitive, latin::Number::Singular), "rēgis");
assert_eq!(latin::noun("rēx, rēgis m.", latin::Case::Dative, latin::Number::Plural), "rēgibus");

use polish::{AdjectiveCase, Agreement, Animacy};
assert_eq!(polish::noun("dom, domu", polish::Case::Locative, polish::Number::Singular), "domu");
assert_eq!(polish::adjective("dobry", AdjectiveCase::Nominative, Agreement::Virile, Animacy::Animate), "dobrzy");

assert_eq!(german::noun("haus", german::Case::Dative, german::Number::Plural), "Häusern");
assert_eq!(german::verb("anfangen", german::Tense::Preterite, german::Person::First, german::Number::Singular), "fing an");

assert_eq!(ancient_greek::noun("λογος", ancient_greek::Case::Genitive, ancient_greek::Number::Singular), "λόγου");

assert_eq!(sanskrit::noun("देव", sanskrit::Case::Instrumental, sanskrit::Number::Singular), "देवेन");

assert_eq!(spanish::noun("canción", spanish::Number::Plural), "canciones");
assert_eq!(spanish::verb("contar", spanish::Tense::Present, spanish::Person::First, spanish::Number::Singular), "cuento");

assert_eq!(french::verb("prendre", french::Tense::PastHistoric, french::Person::Third, french::Number::Singular), "prit");
assert_eq!(french::agreeing_participle("prendre", french::Gender::Feminine, french::Number::Plural), "prises");

assert_eq!(italian::noun("uomo", italian::Number::Plural), "uomini");
assert_eq!(italian::verb("finire", italian::Tense::Present, italian::Person::First, italian::Number::Singular), "finisco");
```

## Layout

```
crates/flexion         the runtime every generated crate depends on (no dependencies)
crates/flexion-train   extract → learn → evaluate → emit
configs/               one language description per module: english.rs, russian.rs, latin.rs,
                       polish.rs, german.rs, ancient_greek.rs, sanskrit.rs, spanish.rs,
                       french.rs, italian.rs (and romance.rs: head-line helpers they share)
configs/extra/         hand-written helpers copied into a generated crate
languages/english      generated (don't edit by hand)
languages/russian      generated
languages/latin        generated
languages/polish       generated
languages/german       generated
languages/ancient-greek generated
languages/sanskrit     generated
languages/spanish      generated
languages/french       generated
languages/italian      generated
tools/english-baseline the published english 0.4, as a baseline for the English report
xtask/                 cargo xtask build | probe | extract
```

## How it learns

1. **Extract.** For every entry of a configured part of speech, the trainer keeps the
   plain forms of each cell in Wiktionary's listing order. Cells are named by the
   config: a group of cells is the product of some features (`Case × Number`), and each
   feature value is a set of Wiktionary tags (`genitive`, `plural`). Nonstandard,
   dialectal, dated and similar forms are dropped (the config says which tags those are).
2. **Edits.** Each (lemma, form) pair becomes an edit: a stripped start and an added
   prefix, a stripped and an added suffix (`city → cities` is "−y +ies", `beautiful →
   more beautiful` is the prefix "more ", `ἀκούω → ἤκουσα` turns the start `ἀ` into
   `ἠ`), and, for languages that write stress, where the stress goes (same vowel as the
   lemma, first vowel of the ending, last vowel of the stem, n-th syllable from the end
   of the word…) and with which mark (the Greek acute or circumflex). Stress is part of
   the edit, not part of the letters, so `рука́ → ру́ки` and `нога́ → но́ги` share one
   edit. A pair can be read several ways; each pair takes the reading the most pairs
   share, so an augment or a `ge-` becomes one edit across many words.
3. **Classes.** A lexeme's edits across all its cells, plus its inherent features
   (gender, animacy, aspect), form its **inflection class**. Classes are found in the
   data, not taken from a grammar book. Later-listed variants become extra observations
   with half the weight of the one before.
4. **Tree.** Observations are counted at every node of a tree over the lemmas, read from
   the last letter back to the start of the word; a syllable-count symbol and the word's
   first letter can be inserted at learned depths (English: `-er` vs `more`). Top-down, each node's counts are
   smoothed toward its nearest kept ancestor, and a node is kept only if it differs from
   that ancestor by a likelihood-ratio test. A whole-word node is kept only if dropping it
   would change one of the word's attested forms. So general patterns live near the root,
   exceptions deep down, and nothing is stored twice.
5. **Tune and measure.** A hash of the lemma splits the words into tenths. One tenth
   chooses the hyperparameters (smoothing, pruning, suffix support, the syllable and
   first-letter symbols) by the one-standard-error rule, and whether the join model and
   the part models (below) earn their place; another is held out and only ever scored.
   The shipped model is then trained on everything.
6. **Emit.** The crate gets typed functions per cell group, a settings value (stress
   marks or macrons on or off), the model as a binary blob, the config's hand-written
   helpers, and a README with the evaluation.

Marks that people leave out when they type (Latin macrons, Russian `ё`, Greek accents,
breathings and vowel length) are folded for lookup; a config names the combining marks
and the trainer finds every precomposed letter that carries them. Each class keeps a
hidden cell with the lemma written in full, and a separate one-cell **restoration model**
learns how a typed lemma is written in full (`rex → rēx`). Forms are weighed over its
readings, so a word whose vowel length can't be known gets a low confidence instead of a
confident guess. A principal part given in a citation spells the stem in full, so it
decides the reading itself.

A part of speech can also read its lemmas written in full from another part of speech's
tables: a verb's participles are adjectives, and the verb's table writes them with their
stress or vowel length. Those forms make a **lexicon**: a lemma typed with the marks left
out (`ахнувший`, `amatus`) and found there is read as written there (`а́хнувший`,
`amātus`), so only its other forms are left to predict. It holds every such form, not only
those the model would get wrong: a participle missing from it then means one the verbs
don't attest, so the model's confidence for the rest stays honest.

Two more models are learned where the data supports them:

- **Joins.** Prefixes that, put before an attested word, make words that inflect through
  it: German `an` + `fangen` → `fängt an`, `angefangen`, `anzufangen`; Russian `за` +
  `падать`. Per prefix, the trainer learns per-cell edits from the attested word's forms
  to the joined word's, and how the prefix changes the word's inherent features (`на`
  makes an imperfective verb perfective). An unseen `anstehen` then inflects through
  `stehen`.
- **Part models.** Where a citation gives principal parts, a part model learns the cells
  from that part instead of the lemma (`ἔλυσα → ἐλύσαμεν`, `नेष्यति → नेष्यामि`). Which
  cells come from which part is decided on the tuning tenth.

At runtime a word is walked down the tree; the deepest node with a class that fits it
(the class's stripped suffixes and starts must match) decides. A stress the caller
writes on the lemma is a fact about the word: classes that stress their lemma elsewhere
are passed over while any class agrees. A **citation** (`"lie, lied"`, `"книга,
книги"`) keeps only the classes that agree with the given principal parts (a missing part
is written `—`), and **markers** (`m`, `anim`, `impf`) weigh classes by the features
they name.

## Other Wiktionary editions

The main dump is always the English Wiktionary's. A config can add other editions'
entries for the language (the Russian Wiktionary's Russian entries) from their own Kaikki
dumps: each edition has its own tags, tables and quirks, so the config's
[`Hooks::adapt`](crates/flexion-train/src/spec.rs) turns its entries into what the English
path produces, or skips them. Words both editions have keep both entries, the English one
first. The tenth a word falls in is a hash of its letters, so a held-out word is held out of
every edition. The held-out numbers count the English Wiktionary's held-out words on its
own entries, so they compare with a model that reads it alone; each report also gives the
score over every held-out word and how often the editions agree on the words they share.

An edition is used only where it measurably helps (more held-out words right, beyond the
noise, without worse calibration):

- **Russian**: the Russian Wiktionary, for nouns, adjectives (participles included) and
  verbs: four times the words, and the first forms agree with the English Wiktionary's on
  98–99% of the cells both have.
- **German**: the German Wiktionary, for nouns (82,000 more). Its adjectives made held-out
  adjectives slightly worse once the model was tuned, and its verbs would make the
  separable-verb model's confidences worse.
- **Spanish**: the Spanish Wiktionary, for verbs.
- **Not used**:
  - French and Italian: their own Wiktionaries agree with the English one but add no
    right answer once the model is tuned (French), or add almost no new words (Italian).
  - Polish: its 22,000 extra nouns don't help, because Polish noun misses are gender and
    animacy, which more words don't reveal.
  - Latin, Ancient Greek and Sanskrit have no own-language edition. The French, German,
    Russian and Czech Latin tables leave out many stem macrons, which teaches the model to
    leave them out too. The other editions' Ancient Greek tables write no vowel length, and
    none has Sanskrit tables.

## Adding a language

A config is a [`Language`](crates/flexion-train/src/spec.rs): features (with the
[`features!`](crates/flexion-train/src/spec.rs) macro), parts of speech and their cell
groups, citation format, spelling (case folding, stress marks and their kinds, vowels,
letters and marks folded for lookup or written plain by a setting, letters with an
inherent vowel, diphthongs), tag policy, and a small
[`Hooks`](crates/flexion-train/src/spec.rs) impl for editorial quirks: which lemmas and
forms are admissible, inherent features parsed from the head line, and `repair` for
tables whose tags are broken (Polish conjugation tables leave the first and second
person untagged; Greek and Sanskrit tables name their tense only in a header). Another
edition's entries go through `adapt` instead, which maps them onto what `repair` makes of
the English Wiktionary's. A part of speech can name groups of others whose forms are its
lemmas written in full (`lemmas_from`: Russian, Latin and Greek adjectives read the verbs'
participles). See
[`configs/src/russian.rs`](configs/src/russian.rs),
[`configs/src/german.rs`](configs/src/german.rs) and
[`configs/src/ancient_greek.rs`](configs/src/ancient_greek.rs). Then:

```sh
cd flexion
cargo xtask build <name> --dump kaikki.org-dictionary-<Language>.jsonl --date <dump date>
```

### What the next language would need

German, Ancient Greek and Sanskrit needed only configs plus generic changes: edits that
change the start of a word, the join model, part models, stress marks of several kinds
counted in syllables, letters with an inherent vowel, capitalized parts of speech, and a
lemma that is no form of the word. Spanish, French and Italian needed only configs: their
nouns and adjectives have no tables, so the configs read the head line (`gratuit (feminine
gratuite, masculine plural gratuits, …)`) and fill what it leaves implicit (invariable
words, one form for both genders). Their written accents are letters, not a stress model:
most lemmas write no accent, so an accent model can't tell a word typed without one from a
word that has none, and it did worse.

Since then the trainer reads other Wiktionary editions (an `adapt` hook per config maps
their tags and tables onto the English one's; each edition had its own traps, listed in its
config), a part of speech can take its lemmas written in full from another's tables (the
lexicon), and a class whose stress rule finds no vowel to stress in a word no longer fits
it (a rule learned as "the stem's second vowel" wrote `вливался` with no stress at all). A
language with a large Wiktionary of its own, with tagged tables, is worth an edition
adapter: Russian got four times the words. For the next ones:

- **Lithuanian, Serbo-Croatian**: pitch accents of several kinds on long and short
  vowels, which the mark kinds already express; the accent classes are the hard part,
  and they are what the tree learns.
- **Hebrew, Arabic**: root-and-pattern morphology. An edit only changes a word's start
  and end, so a pattern that interleaves vowels with the root (`kataba → kutiba`) is a
  whole-word edit. It needs edits that keep the root's consonants and replace what lies
  between them, which is a trainer and runtime change.
- **Turkish, Finnish**: the trainer's whole-paradigm classes don't fit agglutination: a
  Turkish verb has thousands of cells. It would need per-suffix-slot learning (classes
  per morpheme slot, conditioned on the vowel and consonant classes the previous slot
  ends in), which is a trainer change.

## Commands

```sh
cd flexion
cargo xtask build english --dump ~/.cache/flexion/dumps/English.jsonl --date 2026-10-03
cargo xtask build russian --dump ~/.cache/flexion/dumps/Russian.jsonl --edition ru=~/.cache/kaikki/ru/Русский.jsonl --date 2026-10-03
cargo xtask build latin --dump ~/.cache/flexion/dumps/Latin.jsonl --date 2026-10-03
cargo xtask build polish --dump ~/.cache/flexion/dumps/Polish.jsonl --date 2026-10-03
cargo xtask build german --dump ~/.cache/flexion/dumps/German.jsonl --edition de=~/.cache/kaikki/de/Deutsch.jsonl --date 2026-10-03
cargo xtask build ancient-greek --dump ~/.cache/flexion/dumps/AncientGreek.jsonl --date 2026-10-03
cargo xtask build sanskrit --dump ~/.cache/flexion/dumps/Sanskrit.jsonl --date 2026-10-03
cargo xtask build spanish --dump ~/.cache/flexion/dumps/Spanish.jsonl --edition es=~/.cache/kaikki/es/Español.jsonl --date 2026-10-03
cargo xtask build french --dump ~/.cache/flexion/dumps/French.jsonl --edition fr=~/.cache/kaikki/fr/Français.jsonl --date 2026-10-03
cargo xtask build italian --dump ~/.cache/flexion/dumps/Italian.jsonl --date 2026-10-03
cargo xtask extract russian --dump ~/.cache/flexion/dumps/Russian.jsonl \
    --edition ru=~/.cache/kaikki/ru/Русский.jsonl                       # cache lexemes
cargo xtask probe russian --pos noun --tau 2 --misses 1                   # experiment
cargo xtask probe ancient-greek --pos verb --parts 1 --joins 1            # with part models, joins
```

Dumps: `https://kaikki.org/dictionary/<Language>/kaikki.org-dictionary-<Language>.jsonl.gz`;
another edition's: `https://kaikki.org/<code>wiktionary/<Language>/kaikki.org-dictionary-<Language>.jsonl`
(the language named in that edition: `ruwiktionary/Русский`), read with `--edition <code>=<path>`.
A build streams the dump (it is never loaded whole) and takes one to ten minutes per
language on 8 threads (Ancient Greek's 600-cell verbs are the slowest).

## License

Code: MIT or Apache-2.0. Generated models (`languages/*/src/model.bin`) and the test
fixtures are derived from Wiktionary and licensed CC BY-SA 4.0.
