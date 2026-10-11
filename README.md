# flexion

Describe a language's grammar, point the trainer at a [Kaikki](https://kaikki.org/)
Wiktionary dump, and get a Rust crate that inflects **any** word of that language
(attested or not) with a calibrated confidence for every answer.

This repository builds ten languages from the same trainer, with no language-specific
code in the trainer or the runtime:

| crate | held-out words (never seen in training), top-1 | attested words, top-1 |
|---|---|---|
| [`english`](languages/english) | nouns 97.14%, verbs 97.91% (99.53% given the past tense), adjectives 96.71%, adverbs 99.00% (old hand-written rules: 95.76%, 92.51%, 96.15%, 99.00%) | 99.96–100% (`english` 0.4: 98.89–99.43%) |
| [`russian`](languages/russian) | from the bare lemma: nouns 84.5%, adjectives 96.6% (participles are stressed as their verbs' tables write them), verbs 92.1% (stress is the hard part: letters alone are right 95.1%, 99.8%, 97.5%); given the dictionary's principal parts: nouns 95.3% (97.4% with gender/animacy markers), verbs 98.9%; with a stressed lemma, nouns 93.2%, adjectives 99.8% | 99.5–100% |
| [`latin`](languages/latin) | given the dictionary's principal parts: nouns 98.4% (98.7% with the gender marker), adjectives 99.6%, verbs 99.1%; from the bare lemma 68.9%, 86.8% (participles take their vowel length from their verbs' tables), 75.8% (vowel length is the hard part: letters alone are right 93.6%, 99.3%, 90.7%) | 99.6–100% |
| [`polish`](languages/polish) | from the bare lemma: nouns 94.1%, adjectives 99.98%, verbs 97.3%; given the principal parts: nouns 96.4% (98.4% with gender markers), verbs 98.3% | 99.9–100% |
| [`german`](languages/german) | from the bare lemma: nouns 94.9%, adjectives 99.2%, verbs 93.9% (separable verbs inflect through their base: `anstehen` → `steht an`); given the principal parts: nouns 98.4% (98.7% with the gender marker), verbs 99.1% | 99.9–100% |
| [`ancient-greek`](languages/ancient-greek) | given the dictionary's principal parts: nouns 96.5%, adjectives 98.4%, verbs 82.9%; with the lemma written in full (accents and vowel length): 80.2%, 86.1%, 65.6%; from the headword (accents, no vowel length): 57.8%, 63.9%, 54.6% (letters right 92.5%, 95.1%, 79.3%) | 99.7–100% (pronouns 97.9%) |
| [`sanskrit`](languages/sanskrit) | from the stem: nouns 93.4% (97.3% with the gender marker), adjectives 91.8%, verbs 90.0% (91.1% given future, perfect and aorist) | 99.9–100% |
| [`spanish`](languages/spanish) | from the bare lemma: nouns 98.7%, adjectives 98.6%, verbs 99.4% (prefixed verbs inflect like their base: `deshacer` → `deshizo`); given the principal parts: verbs 99.9% | 99.2–100% |
| [`french`](languages/french) | from the bare lemma: nouns 97.7%, adjectives 97.4%, verbs 99.1%, past participle agreement 99.0%; given the principal parts: verbs 99.8% | 99.99–100% |
| [`italian`](languages/italian) | from the bare lemma: nouns 95.9% (96.9% with the gender marker), adjectives 98.4%, verbs 99.2%, past participle agreement 100%; given the principal parts: verbs 99.7% | 99.99–100% |

Attested words are the ones the shipped model was trained on. The full reports (calibration,
per-cell numbers, made-up words, the largest groups of mistakes, where the bytes are, size
against fit, speed) are in each crate's README.

Each crate's model is Rust code (see [The generated code](#the-generated-code)): 2–5 MB of
source for English and the Romance languages and Sanskrit, 8 MB for Polish, 22 MB for
German, 30 MB for Latin, 63 MB for Ancient Greek and 67 MB for Russian (0.2–8.4 MB
gzipped, under crates.io's 10 MB limit for a packaged crate). What the tuning prices is the
size of the tables underneath: 0.3–4.9 MB, Russian's 11.8 MB and Ancient Greek's 17.4 MB.
A clean build of all ten crates takes under a minute on 4 cores and peaks at 12 GB.

Russian learns from the Russian Wiktionary too, four times the words of the English one.
From the bare lemma, the hard part is where the stress falls. A participle typed plain
(`ахнувший`) takes its stress from its verb's table, so adjectives reach 96.6%. Nouns
(84.5%) and verbs (92.1%) mostly miss on a lemma's stress that nothing in the spelling
shows, and on animacy, which decides whether the accusative looks like the genitive
(`адвоката`) or the nominative (`айсберг`). Given the nominative and genitive singular,
nouns reach 95.3%; with the markers a dictionary prints (`кот, кота m anim`), 97.4%.

A Latin lemma as people type it (`rosa`, `rex`) doesn't say which vowels are long. A bare
unseen lemma gets its letters right 91–99% of the time, but its macrons less often. The
restoration models read the lemma from both ends, the start for prefixes like `cōn-` and
`dē-`, which brings nouns to 68.9%, adjectives to 86.8% (many are participles whose verbs'
tables write them in full: `amatus` → `amātus`) and verbs to 75.8%. The rest is a word's own
vowel lengths, which its spelling doesn't show. The confidences say so, and the second guess
is often the right one: top-3 is 83–93%. Given the citation a dictionary prints (`rēx, rēgis
m.`, `amō, amāre, amāvī, amātum`), whose parts spell the stem in full, nouns reach 98.4% and
verbs 99.1%.

Ancient Greek is the same story twice over: a headword like `λύω` writes the accent but
not the vowel length the tables mark (`λῡ́ω`), so even its letters are right far more
often than its marks. Reading the lemma from both ends brings nouns to 57.8% and adjectives
to 63.9%. Verbs learn their sound laws (reduplication, and the consonant changes before
the future's `-σ-` and the aorist passive's `-θ-`), which brings them to 54.6% (the
perfect from 27% to 39%). Given the principal parts, nouns reach 96.5% and verbs 82.9%
(the aim was 85%). A verb's principal parts each build their own tenses (the part models);
what the misses still need is an accent that depends on vowel lengths the parts don't always
show (`ἠγνοήμεθα`, where the model writes `ἠγνόημεθα`) and the augment after a preverb, in
cells that only a few of the 2,700 verbs with tables attest. No other
Wiktionary has Greek verb tables the trainer could use.

Spanish, French and Italian need only their configs. Their remaining misses are mostly
words that don't inflect although their ending says they would: French adjectives and
Italian nouns that are invariable (`antisida`, `abbassalingua`, `allerta`), which nothing
in the spelling marks, keep French adjectives at 97.4% and Italian nouns at 95.9% (the aims
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
crates/flexion         the runtime every generated crate depends on (and only on sound-law)
crates/flexion-train   extract → learn → evaluate → write the crate's code
crates/sound-law       a sound-change engine (moved here from gold-silver-copper/ruthenian)
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
tools/audit            every generated crate answers exactly as the model it was written from
tools/codegen-check    the same, as a test, on code generated from the test fixtures
tools/ceiling          how close the models are to what their data allows
xtask/                 cargo xtask build | emit | probe | extract | export | tune
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
   would change one of the word's attested forms, and it holds only that word's classes
   (smoothing it toward its parent only cost bytes: a word's own forms decide). So general
   patterns live near the root, exceptions deep down, and nothing is stored twice.
5. **Tune and measure.** A hash of the lemma splits the words into tenths. One tenth
   chooses the hyperparameters (smoothing, pruning, suffix support, the syllable and
   first-letter symbols) and whether the join model, the start-restoration model and the
   part models (below) earn their place; another is held out and only ever scored.
   Every setting is scored by top-1 and by **log-loss** (minus the log of the probability
   the model gave the right form, in nats per cell), with standard errors counted over
   words, since a word's cells move together. Among the settings within one standard
   error of the best dev top-1, the build takes the one with the lowest log-loss once
   **each doubling of the model's size costs 5% of it** (`--size-price 0.05`: it
   minimizes `ln(log-loss) + 0.05 · log2(bytes)`). That is the one knob trading size for
   fit, and it pays for everything the model stores: the tree, the restoration models,
   the join model, the part models. A price relative to the log-loss treats Greek, which
   loses 5 nats a cell, and Spanish, which loses 0.1, alike. The join and start-restoration
   models also cost time on every call, so they are added only if the price picks them
   and they win top-1 beyond the noise. A part model is kept if the nats it saves over
   the main model across all cells pay for its bytes and it loses no top-1. Each crate's
   report shows where the bytes are and the size-against-fit curve. The shipped model is
   then trained on everything.
6. **Emit.** The crate gets typed functions per cell group, a settings value (stress
   marks or macrons on or off), the model as Rust code, the config's hand-written
   helpers, and a README with the evaluation.

Marks that people leave out when they type (Latin macrons, Russian `ё`, Greek accents,
breathings and vowel length) are folded for lookup; a config names the combining marks
and the trainer finds every precomposed letter that carries them. Each class keeps a
hidden cell with the lemma written in full, and a separate one-cell **restoration model**
learns how a typed lemma is written in full (`rex → rēx`). Forms are weighed over its
readings, so a word whose vowel length can't be known gets a low confidence instead of a
confident guess. A principal part given in a citation spells the stem in full, so it
decides the reading itself.

The restoration model reads the end of the word, but in Latin and Greek much of what it
has to restore sits at the start: a prefix (`cōn-`, `dē-`, `ἐκ-`) or a vowel before `ns`.
So these languages also learn a **start-restoration model**: the same kind of tree, over
lemmas written backwards, which learns only the marks in the first half of the word. At
runtime each reading of the end model is joined with the best reading of the start model
(start from one, middle as typed, end from the other). It is skipped when the caller
types marks or principal parts, which already say how the word is written. From the bare
lemma, Latin nouns go from 59.1% to 68.9%, Greek nouns from 42.6% to 57.8%.

A part of speech can also read its lemmas written in full from another part of speech's
tables: a verb's participles are adjectives, and the verb's table writes them with their
stress or vowel length. Those forms make a **lexicon**: a lemma typed with the marks left
out (`ахнувший`, `amatus`) and found there is read as written there (`а́хнувший`,
`amātus`), so only its other forms are left to predict. It holds every such form, not only
those the model would get wrong: a participle missing from it then means one the verbs
don't attest, so the model's confidence for the rest stays honest.

Most of that lexicon repeats what the verb model already knows, so the build also tries
**deriving** it: a plain lemma is read backwards through the edits of the verb's participle
cells to a verb the model attests (the verbs that lend a participle are marked as attested
in the verb tree), and the verb model writes the participle, stress and length included.
The lexicon then keeps only the exceptions: the participles derivation writes differently
from the tables, and an empty entry for each one it would find that the tables don't
attest, so the answers stay those of the stored lexicon. The build keeps whichever model is
smaller, and all three languages with such a lexicon derive it now:

| | stored entries | exceptions (blocks among them) | tables |
|---|---:|---:|---:|
| Russian | 57,877 | 31,823 (29,340) | 13.19 → 11.50 MB |
| Ancient Greek | 17,196 | 7,601 (753) | 16.13 → 15.81 MB |
| Latin | 6,819 | 1,933 (46) | 5.43 → 5.36 MB |

The marks are the price, and they take most of the saving back: on the training tenths, Russian's
lexicon shrinks by 483 KB while its verb tree grows by 392 KB (Latin: 60 KB and 43 KB).

Three more models are learned where the data supports them:

- **Joins.** Prefixes that, put before an attested word, make words that inflect through
  it: German `an` + `fangen` → `fängt an`, `angefangen`, `anzufangen`; Russian `за` +
  `падать`. Per prefix, the trainer learns per-cell edits from the attested word's forms
  to the joined word's, and how the prefix changes the word's inherent features (`на`
  makes an imperfective verb perfective). An unseen `anstehen` then inflects through
  `stehen`.
- **Part models.** Where a citation gives principal parts, a part model learns the cells
  from that part instead of the lemma (`ἔλυσα → ἐλύσαμεν`, `नेष्यति → नेष्यामि`). Which
  cells come from which part is decided on the tuning tenth.
- **Sound laws.** An edit is a string swap learned for every ending it occurs with, so one
  process becomes many unrelated edits. Where edits differ only by a regular change at the
  stem's edge, the trainer rewrites them as one edit that carries an invisible trigger
  and one rule, run by [`sound-law`](crates/sound-law): Polish `ręka → ręce`, `noga →
  nodze` become `-a → ⟨T⟩e` with `k → c, g → dz / _⟨T⟩`. It looks for three local shapes
  (local processes are the ones learnable from examples alone, Chandlee, Eyraud & Heinz
  2014): the stem's last consonants before the ending (`k → c`), its last vowel across
  final consonants (German umlaut), and a copy of its first consonant before a vowel
  (Greek reduplication, `λύω → λέλυκα`, `θύω → τέθυκα`). A map entry is kept when it is
  reliable where it could apply, counting the stems that keep the segment against it
  (hits over scope by its lower confidence bound, Albright & Hayes 2002), when at least
  three words and two endings show it, and only for edits it reproduces on every
  training pair. Rules never touch the lemma cell or the cells principal parts give.

  Rules at the stem's end make the model smaller but rarely change an answer: the suffix
  tree already reads those letters, so the right class is found either way (Polish nouns'
  learned palatalization, German nouns' umlaut and Latin verbs' `-s-` perfects leave
  every number as it was). A rule at the word's start is what a tree read from the end
  can't learn: Ancient Greek verbs learn reduplication (with the aspirate copied plain,
  `θ → τ`, `φ → π`, `χ → κ`) and the consonant changes of the future, aorist and perfect,
  which take the perfect from 27% to 39% of held-out cells right and verbs from 52.4% to
  54.6%. Sound laws are kept for a part of speech when the size price picks them (they make
  the model smaller, and often its log-loss lower) and they lose no dev top-1 against the
  setting without them, on the same words, beyond the standard error of the difference (a
  paired test: two models this close share most answers).

At runtime a word is walked down the tree; the deepest node with a class that fits it
(the class's stripped suffixes and starts must match) decides. A stress the caller
writes on the lemma is a fact about the word: classes that stress their lemma elsewhere
are passed over while any class agrees. A **citation** (`"lie, lied"`, `"книга,
книги"`) keeps only the classes that agree with the given principal parts (a missing part
is written `—`), and **markers** (`m`, `anim`, `impf`) weigh classes by the features
they name.

## The generated code

A crate's model is Rust code under `src/model/`, one module per part of speech (and per
restoration, start-restoration and part model), each implementing the runtime's
[`PosModel`](crates/flexion/src/tables.rs) trait:

- the **suffix tree** is functions of `match`es over the word's symbols, read from its last
  letter back; each node on the way adds its distribution over classes:

  ```rust
  /// Words ending -а.
  pub(crate) fn n1(s: &[char], out: &mut Path<'_>) -> bool {
      super::push(out, 1);
      match s {
          [] => true,
          ['а', s @ ..] => {
              super::push(out, 32);
              match s { … }
          }
          …
          _ => false,
      }
  }
  ```

  A node is stored as its class counts and the node it is smoothed toward, in flat tables
  of numbers per file (`KINDS`, `PARENTS`, `STARTS`, `CLASSES`, `WEIGHTS`): a slice
  literal or a constructor call per node made rustc take 7.5 GB for Russian, the flat
  tables 3.5 GB. `push` computes a node's distribution on first use,
  `(n_c + α·q_c) / (N + α)` with `q` the parent's, cut below ε
  (`flexion::tables::smooth_node`, which the trainer's in-memory tables use too). Russian's
  nodes hold 0.30 million counts where they held 0.75 million probabilities: most of a
  smoothed distribution is its parent's;

- the **classes** and **edits** are statics, each class with a word of it and its first
  forms (`// 3324: like рука-манипулятор: рука-манипулятор, руки-манипуляторы, …`); classes
  share their per-group edit vectors as the tables do;
- the **lexicon** and the **join prefixes** are `match`es on strings.

The trainer never compiles this code while it tunes: it scores an in-memory model of the
same tables (the runtime's `Tables`), and writes the code from it once the build is done.
So the two must agree, and [`tools/audit`](tools/audit) checks it: for every language it
retrains the shipped model from the lexeme cache and the plans the build saved, then asks
both the same questions (the held-out words in every input mode, a sample of attested
words, made-up words; every cell with marks on and off, whether each cell exists, every
inherent feature), and requires the answers to be equal bit for bit. On the shipped crates
that is 1.6 million answers (French) to 11.4 million (Ancient Greek) per language, 46.7
million in all, every one equal.
[`tools/codegen-check`](tools/codegen-check) does the same on the test fixtures, in the test
suite.

rustc puts limits on what code can look like, and the generator stays inside them: a tree
function holds at most 256 nodes and nests at most 6 `match`es (a long word's chain of
nodes overflowed rustc's stack), a file at most 8,192 nodes or 20,000 table entries, and a
string `match` at most 500 arms (Russian's 56,000-word participle lexicon as one `match`
overflowed it too). Generated crates build no unit-test harness (`test = false`): their
tests are their README examples, and the model isn't compiled twice.

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

## How close to the ceiling

[`tools/ceiling`](tools/ceiling) measures how much better any model could do, four ways:

- **Learning curves**: each language trained on 10%, 25%, 50% and 100% of its words.
- **The noise in the labels**: where two editions both have a word, how often they
  disagree.
- **What the ending can tell**: the best predictor that sees only a word's last letters.
- **A neural baseline**: a 5.6M-parameter character transformer, run in Docker on
  8 CPU threads for about 10 minutes per part of speech, scored on the same held-out
  cells.

| language | near its ceiling? | where the room is |
|---|---|---|
| Russian | Yes, for what a lemma's spelling can tell. | The lemma's stress: the best ending-only predictor gets it 84% right on nouns and the tree 87%. A transformer reading the whole word is worse than the tree (nouns −3.8 points, verbs −3.5). More data adds about 1.7 (nouns) and 2.7 (verbs) points per doubling. |
| Latin, Ancient Greek | Closer, for nouns. | Vowel length inside the stem. The transformer beat the tree by 10.6 points on Latin nouns and 20.4 on Greek nouns, mostly on prefixes and stems the ending can't show. The start-restoration model closes most of that gap: Latin nouns 59.1% → 68.9% (transformer 69.7%), Greek nouns 42.6% → 57.8% (63.0%). The rest of the stem's lengths isn't in the spelling at all: even the whole word predicts a Latin noun's lengths only 71% of the time. |
| German | Yes. | The transformer is worse (nouns −8.4, verbs −2.0). More data adds about 1 point per doubling. |
| Polish, Sanskrit | Probably (no neural run). | About 0.5–1.4 points per doubling of the data. |
| English, French, Spanish, Italian | Yes. | Nouns and adjectives gain at most 0.3 points per doubling; verbs 0.4–0.8. |

Label noise is about half the rate at which two editions disagree: 0.5–0.8% of cells
for Russian, 0.15–0.75% for German nouns, under 0.1% for Spanish verbs. So even a
perfect model would stop at about 99–99.5% top-1. That limit matters only where models
already stand at 98–99.5%: Romance verbs, and every language given principal parts.
The full measurements are in [`tools/ceiling/RESULTS.md`](tools/ceiling/RESULTS.md).

These were tried and are not used, because they made models bigger or no better:

- **A size cost per tree node** (keep a node if its likelihood gain beats its size in
  bits). It made the same trees as the likelihood-ratio test, or worse ones for German
  and Latin.
- **Cutting the rarest classes from each node's distribution.** Those tails are rare
  words' real inflections, and citations need them.
- **Word neighbours sharing a root**, for Russian stress. At best they added 0.8 points,
  and they would need a large index of words.

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
cargo xtask build french --dump ~/.cache/flexion/dumps/French.jsonl --date 2026-10-03
cargo xtask build italian --dump ~/.cache/flexion/dumps/Italian.jsonl --date 2026-10-03
cargo xtask extract russian --dump ~/.cache/flexion/dumps/Russian.jsonl \
    --edition ru=~/.cache/kaikki/ru/Русский.jsonl                       # cache lexemes
cargo xtask probe russian --pos noun --tau 2 --misses 1                   # experiment
cargo xtask probe ancient-greek --pos verb --parts 1 --joins 1            # with part models, joins
cargo xtask probe latin --pos noun --front 1                               # with the start-restoration model
cargo xtask tune russian --pos noun                                       # the grid: size, top-1, log-loss
cargo xtask emit russian                                                  # rewrite the model's code from the last build's plans
cargo run --release -p audit -- russian                                   # the generated code answers as the model
cargo xtask export russian                                                # train/dev/test splits as TSV
```

Dumps: `https://kaikki.org/dictionary/<Language>/kaikki.org-dictionary-<Language>.jsonl.gz`;
another edition's: `https://kaikki.org/<code>wiktionary/<Language>/kaikki.org-dictionary-<Language>.jsonl`
(the language named in that edition: `ruwiktionary/Русский`), read with `--edition <code>=<path>`.
A build streams the dump (it is never loaded whole) and takes one to fifteen minutes per
language on 8 threads (Russian's four editions' worth of words and Ancient Greek's
600-cell verbs are the slowest). `--size-price` (default 0.05) sets the size knob;
`cargo xtask tune <lang>` shows what other prices would choose.

## License

Code: MIT or Apache-2.0. Generated models (`languages/*/src/model/`) and the test
fixtures are derived from Wiktionary and licensed CC BY-SA 4.0.
