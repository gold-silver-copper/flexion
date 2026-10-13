# Prompt: a universal core — one cost, stated-once structure, stem spaces, prosody

You own one PR on `~/code/flexion` (branch from the latest `main`, at or after `774b2a5`,
"Merge pull request #12"). It makes the trainer's core a better theory of morphology in
general: mechanisms every language uses, learned from the data, that make every generated
grammar smaller, more accurate and closer to how a linguist writes one. You may change
anything (runtime, trainer, configs, `sound-law`, generated crates, tests, CI, APIs) if it
serves that. The project is pre-release: no backwards compatibility, no shims; delete what a
better design replaces.

Work through §2 in the order given (it is the order of dependencies, and roughly of value),
all in this one PR. Each item is kept only where it pays, measured and logged; an item that
doesn't pay is dropped with its numbers. No prototypes or spikes: what lands is finished.

## 0. Where things stand

flexion learns each language's inflection from Kaikki/Wiktionary dumps and prints it as a
Rust crate that *is* the grammar. Read the root `README.md`, one generated crate
(`languages/ancient-greek/src/verb/`, `languages/russian/src/noun/`), the trainer
(`crates/flexion-train/src/`), the runtime (`crates/flexion/src/`) and `crates/sound-law`.
The previous prompts (`prompts/grammar-structure.md`, `prompts/stems-and-boundaries.md`)
and the log (`~/code/notes/projects/flexion/grammar-crates.md`, "PR 12") say what was built,
tried and dropped, and why.

Held-out top-1 from the headword (crate READMEs have the full reports):

| | nouns | adjectives | verbs |
|---|---:|---:|---:|
| English | 98.59 | 96.91 | 97.83 |
| Russian (no stress) | 95.08 | 99.75 | 96.05 |
| Latin | 66.41 (93.45 in full) | 82.54 (99.28) | 70.47 (85.24); ten folds 71.79 ± 0.90 |
| Ancient Greek | 67.50 (85.52) | 72.00 (85.65) | 63.22 (72.09) |
| Sanskrit | 92.13 | 91.25 | 88.14; ten folds 85.33 ± 1.54 |

Principal parts given: Greek verbs 74.38, Latin verbs 97.17. Top-3 from the headword: Greek
verbs 75.0, Latin nouns 85.3. Calibration error, worse than PR #12's base: Latin verbs 0.048
(0.031), Greek verbs 0.071 (0.065), pronouns (Russian 0.402, Greek 0.809). Attested ≥ 99.9%.
The model before PR #10 is still ahead on Latin verbs (75.18) and nouns (68.91), Sanskrit
nouns (93.55) and verbs (90.13; the 2017 floor 90.01). The "unseen cells" figures rest on
1–8 forms for most parts of speech: they measure almost nothing yet (see §6).

**Size means the generated crates only.** Trainer and runtime may be as large as they need;
no line budget on them. The generated grammars are the product: 11.4 MB, 243k lines.

| crate | bytes | lines | largest files |
|---|---:|---:|---|
| ancient-greek | 4,878,953 | 73,193 | verb `irregular.rs` 1.31 MB (46,243 forms), verb `paradigms.rs` 1.18 MB (141 one-word conjugations, 444 KB), verb `relations.rs` 409 KB, verb `accent.rs` 196 KB |
| russian | 2,892,346 | 67,290 | noun `animacy.rs` 631 KB (3,933 rules, 12,106 listed words), verb `rules.rs` 462 KB, noun `relations.rs` 441 KB, noun `rules.rs` 305 KB |
| latin | 1,927,691 | 54,172 | verb `relations.rs` 428 KB, verb `paradigms.rs` 371 KB |
| english | 898,992 | 29,615 | noun `rules.rs` 323 KB, noun `relations.rs` 188 KB |
| sanskrit | 846,272 | 19,125 | |

One call (release, one cell's candidates): Greek verbs 67 µs (8.5 before PR #12), Greek
nouns 36, Russian verbs 22, Latin verbs 18, Sanskrit nouns 6 (target about 4).

Five more languages have configs (`crates/flexion-configs`: German, Polish, Spanish, French,
Italian) and extracted data in `~/.cache/flexion/grammar/`; they are not shipped.
`~/.cache/kaikki/` has dumps of some 150 more (Finnish, Turkish, Hungarian, Estonian, …).

**The dumps carry the editors' own analyses**, as inflection-template arguments and tags:
Greek `grc-conj` gives each verb's tense stems, Latin `la-conj` the class and perfect and
supine stems, Russian `ru-conj`/`ru-noun-table` Zaliznyak's class and stress letter (`6a`;
nouns: a 89%, b 7%), Finnish every table's class (`27*F/käsi`, about 51 noun types); tags
mark deponents (587 Latin verbs, 52 Greek) and Russian adjectival nouns (2,106). **Use these
only to score what is learned** (do the learned stem slots, stems, accent classes, features
agree with the editors'?), never as training input: the grammar must learn them itself.

Tried and dropped before (don't repeat without a new idea; numbers in the log):
naive-Bayes beginning rules; prefixes for every language (only where an augment shows the
boundary); principal parts as a fixed list; restoring Russian `ё`; Greek contraction as a
global law; marking cells `derived` wholesale; lumping minor rule values; Wilson & Li
pruning; gender as a hard filter; one global two-sided vowel-length context (la noun −1.3,
grc adj −4.4); restoring marks when typed in full (la verb pp −4.6); accent carried through
the sound laws; per-letter rule cost on every choice (la noun −3.5; kept at 1 bit on
relation and accent choices only); paradigms like others block by block (no gain while the
augment sat in the patterns — it is a law now); relation sources by three-letter endings;
sharpness kept at 1 unless another value is 10% better.

## 1. Read first

The references are in `~/code/notes/projects/flexion/references/`: 244 PDFs in `papers/`,
four older surveys (indexed by `README.md`; they cover only the first 104 papers), and four
notes written for this prompt on 2026-10-13, which are the brief:
- `2026-10-reread.md`: every citation below checked against the PDFs, with corrections, the
  order of work, and missing operational detail per item (its parts A–C are the summary);
- `2026-10-recent-structure.md`: description-length learning, stem spaces and inflection
  classes from data, syncretism, subregular learning, accent as marks;
- `2026-10-recent-inflection.md`: SIGMORPHON 2019–2023, evaluation design, classical
  languages, calibration;
- `2026-10-recent-typology.md`: UniMorph, agglutination and harmony, Russian stress, vowel
  length and diacritics, per-class calibration, the Finnish/Turkish data counted.
Where a note and a paper disagree, the paper wins. Read the papers behind an item before
building it and cite pages in the log ("p." printed page, "PDF p." file page).

## 2. What to do, in order

Every item is a general mechanism, learned the same way for every language and part of
speech, with nothing language-specific in the trainer or the runtime (configs may name a
language's facts). Where an item names Russian or Greek, that is an instance; build the
mechanism and let the data decide where it applies. Every search is deterministic (fixed tie
order; the papers' greedy-with-random-ties and annealing searches are not acceptable here).

**1. One cost for everything: description length.** Build the accounting first; items 2–7
are "kept where they pay" and have no other threshold (no paper gives one: Zwicky 1985
p. 384). Define it exactly:
- *grammar bits*: each printed item's token stream at log₂(choices) per token (Rasin et al.
  2021 fn 12: log₂ n, not rounded up) plus a fixed cost per item for its name and evidence;
  a counts table costs −log₂ of its Dirichlet-multinomial marginal (O'Donnell TR p. 26 eq. 7);
  listed stems and listed words are counted with the grammar (Walther & Sagot: counting the
  grammar alone picks the wrong stem-space analysis, 35,800 vs 51,400 bits);
- *data bits*: held-out −log₂ P(form | lemma, cell) under the runtime's own blend, counting
  forms a rule wrongly produces (Rasin, Shefi & Katzir 2020); cells a word lacks cost nothing
  (otherwise listing everything wins as data grows: Rasin et al. 2021 p. 48);
- *one weight* on the data part, the same for every language and item kind, chosen on the
  dev tenth and logged as a size–accuracy frontier (every paper weights it; the literature's
  4–5 bits per letter is 8–10× what cost Latin nouns 3.5 points in PR 12);
- a held-out top-1 guard on every structural merge (Beniamine et al. 2017 p. 481: the
  description-length optimum can predict worse), and a held-out check on sound laws, whose
  blocking contexts training-data bits never pay for (Rasin & Katzir 2020).
Use it for rules (Khalifa et al. 2023: an accuracy test beat the Tolerance Principle on
unseen Arabic words, 79.4% vs 69.2%, with 402 listed words instead of 2,305), paradigms
(the 141 one-word Greek conjugations against "like the parent, plus listed stems"), ending
tables, stem slots, referrals, relations, laws, listed words. Report bits and bytes per item
kind. The optimum is not the smallest grammar (O'Donnell TR p. 49); report it as found.

**2. Sound laws that generalize.** Laws come before stems: two stems a general law relates
are one stem (Bonami & Boyé 2002 n. 5; Pirrelli & Battista p. 323). Make the law learner
general enough for every language, extending `sound-law` where it lacks a feature (never
work around it):
- contexts grown outward from none, one letter at a time on whichever side separates the
  cases better, only while the law has too many exceptions (Belth's local learner) — a new
  basis for the dropped two-sided contexts;
- laws that read their own earlier output, left to right, so harmony runs through a chain of
  suffixes (Turkish `ev-ler-imiz-de`; Koskenniemi p. 77; Chandlee et al. 2015 p. 123; Heinz
  2018 pp. 36–38); triggers inside an ending; one law mapping target × trigger → output (a
  four-way harmony law is one law); a default for words with only neutral vowels; tiers
  learned by removing letters from the alphabet while keeping blockers;
- laws learned from forms that differ from their build in more than one letter (today
  `laws.rs` accepts only one-letter differences);
- an "unstressed" context, for laws that read stress (item 7).

**3. What is the same is stated once.** Two devices, not one (Brown 1998 pp. 98–102):
- *neutralization*: a form shared throughout a context is stated once on a shorter path,
  pointing nowhere — Greek middle = passive outside the aorist and future (about 300 of 624
  cells), Latin dative = ablative plural, the Russian adjective's animacy twins and
  masculine/neuter obliques (today 32 of 51 Russian adjective relations are identity copies;
  Corbett & Fraser's adjective node is 19 equations). Learn the feature order the
  statements use (number → case → gender → animacy for Russian; Pertsova 2009: each domain
  over feature values, fewest features first, with "elsewhere" exceptions);
- *directed referral*, only where the identity is one-way or class-specific: it resolves on
  the word's own source form, listed forms included (Evans & Gazdar 1996 p. 175), may cover
  a whole block (`du → pl`, Brown 1998 (5.6) p. 101), may point to a later cell (today only
  earlier cells; no paper supports that limit), never in a cycle. Direction is often
  undecidable (Hein & Murphy 2025 p. 410): try both, keep the cheaper.
A statement must hold across at least two ending tables and not already be made by a law
(Baerman 2004 ms pp. 3–5); share each referral's scope per paradigm class where that is
where it holds (Flexica 2022 borrowed the Polish instrumental for the dative everywhere
because they coincide in most feminines, and failed elsewhere). The same for accent places
(a default per cell, overrides per pattern) and relations. No published learner builds such
a hierarchy from data: say plainly the procedure is new.

**4. Stem spaces with shared ending tables.** A lexeme is a few stems in learned slots; a
cell is a slot, an optional template (augment, reduplication, preverb) and an ending from a
table paradigms share (Bonami & Boyé 2002, 2003; Finkel & Stump 2006 Tables 19/22–24 and
Finkel & Stump 2009 DHQ ¶45–60; Ackerman, Blevins & Malouf 2009 pp. 65–68, 76–80).
- Stems: the longest common subsequence of a word's forms, keeping tied alignments and
  choosing the one with the commonest endings (Beniamine & Guzmán Naranjo 2021); endings
  first from the commonest paradigm, stems as forms minus endings (Bonami & Boyé 2003
  pp. 106, 120). The stem/ending boundary is undecided (Beniamine p. 472): try maximal
  stems, root + theme and between, keep the cheapest by item 1.
- Slots: after undoing the laws, record which cells share a stem, drop the letters, and
  count those groupings across words; a grouping is a slot when the same cell split recurs
  in at least two different alternations, counted by root with compounds merged (Pirrelli &
  Battista p. 315; LeFevre, Elsner & Sims 2021). Slots are free cell sets, not feature
  defined (2003 p. 109); a slot may be built on another cell's whole form (Hungarian
  dative plural = nominative plural + `-nak`); slots form a tree where a listed stem fills
  its slot and every slot below it unless listed too (2002 p. 58); a repeated stem is
  listed twice, no conditional links (2003 pp. 116–117). Candidate stems are attested ones
  or letter-by-letter joins of them (Wang & Hayes).
- Ending tables: the recurring sets of endings over a slot's cells, named after their
  commonest word; split into case and number pieces only where that saves bits (Latin verbs
  yes, Latin adjectives no).
- An irregular word lists its stems, not its forms (Greek about six; English `go/went/gone`);
  forms built on no stem (French *sommes, êtes*) stay listed whole. Prefixed words take
  their base word's stems (Latin: uncertainty 0.279 → 0.079 bits, Pellegrini 2020).
- Keep whole-form relations as first-class items next to stems: the evidence for prediction
  is word to word (Bonami & Beniamine 2016 fn 1); stems are claimed for size and
  readability, measured, not for accuracy.
- Score against the editors (§0): Greek tense stems, Latin principal stems, Russian classes.
  Hand counts to compare: Latin 3 slots (Pirrelli & Battista Table 6) or 4 (Pellegrini
  2020), French 8 + 4, Italian 8, Sanskrit about 9–11. The only automatic slot-maker in the
  corpus came out twice the size of the hand grammar (DHQ ¶80).
Print a stem table and an ending table a reader recognizes; show them in the README.

**5. Relations by the paradigm's implicative structure.** Choose which cell tells which by
held-out bits: −log₂ P(true target | true source) with flexion's own relation rules as the
patterns, forms no rule produces charged their listing cost, every training word counted
once, cross-validated (Bonami & Beniamine 2016 eq. 11 p. 12; two sources eq. 16, n sources
eq. 18), **plus the relation's own rule bits** (the Russian dative from the prepositional
was a size failure, 13,848 vs 1,031 rule lines at equal top-1, which entropy alone would
not catch). Build the tree as a Chu-Liu-Edmonds arborescence over these costs (Cotterell et
al. 2019 pp. 332–333), rooted at the headword and given principal parts, with item 3's
statements as free edges; don't assume the headword is the best predictor (Latin's present
1sg is the worst: 0.917 bits, Pellegrini 2020; Latin's 254 verb cells fall into 15 groups
that predict each other perfectly — a yardstick for item 3). Test one source per paradigm
class against one per part of speech (Brown & Evans 2010 p. 236). Move off a parent only
when the gain beats the noise (Albright diss. pp. 118–126, §3.3 pp. 43–45). Two sources only
where the best single one is weak and only if they win held out (check the Latin present
participle against single present-system sources first). At runtime, combine every form the
caller gives through its relation to the target cell and name the deciding form in the
reason (Liu & Hulden 2020).

**6. Word features as referrals, across parts of speech.** Generalize what Russian animacy
does (a word feature chooses which cell a cell refers to) to any cell and to another part of
speech's grammar.
- Animacy, Network Morphology style: defaults at the top (acc = nom; animate plural and
  animate masculine singular → gen) overridden by a class's own accusative (Corbett & Fraser
  pp. 129–131; Brown 1998 (6.14) p. 155). Compare "default inanimate, animates listed"
  (Brown App. p. 246) with suffix rules on unseen animate nouns (agent suffixes predict it).
- Deponents: values none / full / perfect-only / present-only for Latin (Hippisley 2021
  sheets 20–21; Surrey's DATR account), per tense system for Greek; inferred like gender
  from the tables. Network Morphology leaves deponent passives undefined: not adopted here —
  the passive cells are generated, only the active cells refer.
- Adjectival nouns and participles refer to the adjective grammar (nearest model: Evans et
  al. 2003 p. 4's separate class index).
- Score each learned feature against the dump's tags (§0).

**7. Prosody as its own layer.** Where the data writes stress or accent, learn it over the
segmental grammar, as marks: stems accented on a given syllable, post-accenting, or
unaccented, endings marked too, joined by one combining rule ("leftmost accent wins";
Kiparsky 2010; Matushansky 2024; Halle 1979). The marks can be solved exactly: stems whose
stress moves are unaccented and show each ending's mark; every other stem gets the one mark
that fits; stems nothing fits are listed. Marks sit on stem slots (or Russian classes c and
d can't both be stated). Zaliznyak's letters then fall out and can be printed as names
(score them against the dump's letters; include b′ and f″, which Brown 1998's eight patterns
lack). Defaults: Greek recessive with named departures (Golston & Paulsen 2025); Russian
last stem syllable (Mołczanow 2019). Add an accent place "first syllable" (го́ловы, по́нял,
Vedic). Stress chooses endings too (Russian gen pl -ей vs zero, Brown & Hippisley 1994
pp. 56–57; nom pl -а́ with c), so accent class and paradigm are chosen jointly. Spelling laws
that read stress (unstressed о → е after sibilants, ё) run after stress placement (Swanson &
Tyers 2022 pp. 109–111). Russian answers may stay unstressed; say whether input headwords
carry stress. This must merge the 64 Russian paradigms that differ only by stress-driven
spelling. Floors on unseen Russian words: 83.9% (Hall & Sproat, unseen lemmas), 84–89%,
neural 88.7% (Ponomareva 2017).

**8. Spelling and paradigm chosen together.** A headword typed without marks is ambiguous in
spelling and paradigm at once: score candidate spellings jointly with the paradigms they lead
to (joint scoring cut new-word errors 30–40% in Arabic and Hebrew restoration), with the
syllable count as an explicit context symbol (Albright 2005 p. 15; the Latin −r/−s shares are
read off Fig. 4, p. 20) and the word's start; Wilson & Li's two-sided contexts mean left and
right generalized separately among rules with the same change (§2.2–2.3), so per paradigm,
not global. Score a Greek spelling against the accent the headword already carries: the
accent fixes the length of α, ι, υ almost without exception (about 83% are short; a final
one's length is fixed in 48% of headwords, agreeing with the dictionary 96.8% of the time;
Thörn Cleland & Cullhed 2026). Decide max vs sum over spellings giving the same form, and
use the same in item 10. Retire the fixed 1/10 second-spelling threshold. The bound is what
the spelling carries (Latin noun stem length 71% from the whole word vs 59% from the ending;
Winge 2015), not the typed-in-full number.

**9. Statuses from the right counts.** `productive()` today counts every dictionary word with
the ending — exactly the count Yang's Guide (p. 5) rejects. Count types per cell, attested
lemmas only, once per base stem (prefixed words merged; Gorman & Yang 2019 p. 14 fn 11), on
vocabularies of a few hundred to 2,000 words ranked by the commonness proxy (k = 250, 500,
1,000, 2,000), e ≤ N/ln N as real numbers, per node of the rule tree (Belth et al. 2021
pp. 2–4; Yang 2018 pp. 799, 802–803). Report how often the verdict holds across vocabularies
(stability, not correctness: Guide p. 10) and check verdicts against published ones (English
-ed; German feminine -(e)n; Polish gen sg none, gen pl -ów; Russian dental 1sg none). When two
analyses within a few bits give different forms, that is a source of `unsettled` too (Katzir
2025). Description length decides rule vs list; the Tolerance Principle only labels.

**10. Calibration per kind of candidate.** First check the scoring: the scorer accepts only
the first listed form, and pronoun cells list several — part of the pronouns' error may be
the measure. Then put every candidate kind on the same evidence scale, fit one sharpness and
one weight per part of speech by log loss on the dev tenth (from the headword), and
per-status deviations shrunk toward it (Goldwater & Johnson 2003 §2, §4.1), only where a
status has enough dev cases; listed forms as Zuraw's mixture (2000 pp. 51–53). Add a "none of
these" slot: today's rescaling keeps each list's sum, so one-candidate lists can never move.
Report log loss next to calibration error, and a smoothed calibration error (Błasiok 2023;
the 10-bin one reads low). Find first which status carries Latin verbs' 0.031 → 0.048; it
must get back to 0.031 or better, and nothing may be worse than PR #12's base.

**11. Universality checks.** The mechanisms are claims about morphology in general; test them
on languages they weren't tuned on.
- Before and after this PR, run `cargo xtask try` on the five unshipped languages (German,
  Polish, Spanish, French, Italian), every part of speech, and report them next to the
  shipped five. A significant paired loss on the same words (sign test) on any of them means
  the mechanism is overfitted: fix or drop it.
- Add Finnish nouns and adjectives (consonant gradation in 12% of forms, an answer key in
  Koskenniemi's four-stem split pp. 45–46, and the editors' class on every table) and Turkish
  nouns and adjectives (the last syllable's vowel predicts the locative ending in 98.6% of
  10,638 nouns; 148 loanword exceptions; stem changes k→ğ 1,331, t→d 177, vowel drop 136;
  84 cells = number × possessor × case). Turkish verbs are mis-tagged in the dumps; leave
  them. Config repairs only (Finnish possessive rows lose the noun's number; infinitive
  numbers parse as persons). Harmony must come out as item 2's laws, endings printed with
  class letters (`-lAr`, `-DA`; Belth 2023, 2025), gradation decided by description length
  (law, slots or split paradigms), with no code that names either language. Report case ×
  number cells apart from possessive cells; cap training tables generically (6,000 Finnish
  noun tables sufficed: Durrett & DeNero fn 6). Compare the learned Finnish classes with the
  editors' ~51 (an earlier learner found 258 paradigms). Don't ship either crate unless it
  is good; report what the core can't express yet.

**12. Speed.** Last, since items 1–8 change the call path. A call answers from the grammar
with nothing precomputed; still, one cell of a Greek verb costs 67 µs, eight times PR #12's
base, because the second-spelling reading and the preverb route rebuild what the first pass
built: share it. Targets: Greek verbs within 2× of 8.5 µs, Sanskrit nouns about 4 µs. Move
the release harness (`~/.cache/claude-tmp/sb/rt-bench/`) into the repo; time attested and
unseen words separately; a pure speed change must leave the candidate dump byte-identical.
No compiled transducers (they can blow up and are precomputed).

## 3. What must stay true

- A crate is its grammar: named items (laws, stem slots, ending tables, statements,
  referrals, paradigms, accent marks), evidence on every rule, no probability tables or
  opaque ids, no in-memory twin. The end-to-end test that prints learned grammars, compiles
  them with `-D warnings` and compares every answer keeps passing and covers every new item.
- Every word has every form in every cell, attested or not: no gaps, no defectiveness, no
  "no form", never the lemma as a stand-in. Rarity or doubt goes into status and reason only.
- Calibrated candidates with status and reason on every answer. No panics (clippy denies
  unwrap/expect/indexing/panic), nothing computed before the first call, byte-identical
  output for the same input.
- Nothing language-specific in trainer or runtime. If `sound-law` or another of the
  project's crates lacks a feature, add it there.
- Idiomatic, type-driven Rust; clippy clean with `-D warnings`; README examples pass.

## 4. How you work

- No `/tmp`; scratch in `~/.cache/claude-tmp/`. Never send the user's email address or other
  personal details to any outside service (for example a paper-lookup API); use services
  that need no identity (OpenAlex, arXiv, ACL Anthology).
- Cap parallelism (`-j 4`, `--test-threads 4`, at most 8 training threads). Build generated
  crates one at a time under `systemd-run --user --scope -p MemoryMax=16G -p
  MemorySwapMax=0` (1–20 minutes each). Nothing else over about 10 minutes, nothing using all
  cores or lots of RAM, without asking. Ten-fold runs: one language at a time; ask before any
  over 10 minutes (Greek verbs take about 25).
- `cargo xtask try <lang> --pos <part> --quick` is the fast loop; `--fold N` scores another
  tenth; `cargo xtask build <lang>` is the full build.
- Kill everything you start. Never `pkill -f` a pattern from your own command line; use PIDs.
- Tests: few and meaningful, no golden, snapshot or parity tests, each with a verified
  failure witness (break the code, see it fail, restore).
- Progress log: dated entries in `~/code/notes/projects/flexion/grammar-crates.md` under
  "PR 13", with numbers, what was tried and dropped, and the paper pages behind each choice.
- Subagents are allowed for independent measurement and audits; don't run memory-heavy
  builds in parallel.
- Talk to the user in plain language, with before/after code for API changes and
  copy-pasteable commands starting with `cd ~/code/flexion`. Say what you're waiting on and
  for how long before long waits.

## 5. Git

One PR from a branch off the latest `main`. fmt and clippy before pushing. Merge with
`gh pr merge --merge` once CI is green, then check out the latest `main`. No crates.io
publishing, tags or releases.

## 6. Measurement (build these early; the items are judged by them)

- **Before/after by paired bootstrap** on the same words, not by eye; the 95% margins in the
  READMEs stay.
- **Held-out cells, not only held-out words** (Kodner, Khalifa & Payne 2023): drop every
  training form of a set of cells that share a stem (the Latin perfect-stem tenses, the Greek
  aorist passive, the Russian past) and score those cells. This tests items 3–5 directly.
- **The right form among the candidates at all** next to top-1 and top-3 (Flexica 2022:
  84% among candidates vs 50% chosen, for Polish), and the share of words with every cell
  right.
- **Given forms**: accuracy with k random forms given (k = 1, 2, 3), not only principal parts.
- **Error audit by kind** on a sample of 200 errors per weak cell: wrong gold (Gorman et al.
  2019: nearly a quarter of "errors"), vowel length, accent, stem, ending, paradigm choice;
  over-regularization vs over-irregularization counted apart; which paradigm each wrong
  answer came from.
- **Learning curve** (25 / 50 / 100% of training words) and a run with half of each table's
  cells removed.
- **Words built on a training word by a prefix** reported apart (the hash split leaks them).
- **Agreement with the editors** (§0): stem slots, stems, accent classes, features.
- **Outside yardsticks, through a data adapter only**: SIGMORPHON 2023's lemma-disjoint data
  for the languages it shares with us (best published: Greek 61.7, Sanskrit 73.4, Russian
  91.9); English verb probabilities against Albright & Hayes's human nonce-verb ratings
  (their rule model beat a neural one on 6 of 8 measures); public nonce-word sets with human
  answers for English, German, Turkish and Finnish where they exist.

## 7. Done means

- Every item in §2 made, or measured and dropped with its numbers.
- The five crates regenerated, compiling, README examples passing, CI green.
- The generated crates at least a third smaller in total (bytes and lines) than 11.4 MB /
  243k lines, Ancient Greek under 3 MB, no crate larger than now.
- Held-out accuracy at least the §0 table everywhere within its margin (paired bootstrap),
  and no significant loss on the five unshipped languages; held-out-cells, unseen cells and
  principal parts given reported before and after. Attested ≥ 99.9%. Calibration no worse than
  PR #12's base anywhere.
- Speed targets of item 12 met, or the reason in numbers.
- READMEs (root and crates) updated with real excerpts of the new structure (a stem table, an
  ending table, a stated-once statement, a referral, accent marks, a harmony law) and the new
  numbers.

## 8. Final report

The PR link. Per language and part of speech (shipped five, unshipped five, Finnish, Turkish),
before and after: top-1 from the headword, in full, principal parts given, k forms given,
features given, seen, unseen and held-out cells, top-3, among-candidates, reciprocal rank,
floor, attested, calibration and log loss, with ten-fold margins where run and paired
significance. Agreement with the editors' analyses. Generated size per crate before and after
(bytes, lines, largest files), the description-length frontier chosen, build cost, call time.
What each §2 item gave, what was dropped and why, the error audit, what Finnish and Turkish
showed about the core, and what the references point to next. How you verified it, and how
the user can run it.
