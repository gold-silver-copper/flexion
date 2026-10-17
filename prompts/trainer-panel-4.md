# Prompt: the fourth panel — find what to do next, and write the prompt that does it

This run analyses and decides; it does not implement. You convene a fourth panel of
specialists on `~/code/flexion`, as it is after PR 19 (`main` at or after `95536ce`).
The panel reads the trainer, the runtime and the generated crates, and makes bets on
what to change, backed by evidence. You chair: you gather the evidence first, check
claims, run the probes the panel asks for, and decide a ranked plan.

At the end you deliver three things:
1. **The panel's record** in the notes (§3).
2. **The implementation prompt,** `~/code/flexion/prompts/trainer-panel-4-plan.md` (§5).
   A separate agent run carries it out in one PR. Write it so that run needs nothing
   else.
3. **A full report** to the owner of what the panel found and what you recommend (§9).

**This panel works differently from the last three.** A review of their sessions is in
`~/code/notes/projects/flexion/panel-process-review-2026-10-16.md`; read it first. In
short:
- **What paid:** bugs found by reading real output, exact changes (fewer bytes or less
  time, answers identical) and coverage. Mechanisms argued from source alone gave little
  or lost.
- **Repeats:** every panel re-proposed things already measured and dropped.
- **Plan size:** panel 3's plan was too big for one run, and one item vanished without
  a verdict.
- **Where implementation time went:** each of the last two implementation runs took
  about 18 hours. About 80% went to the final stretch: full runs one after another,
  restarted when an unscreened item or an unchecked build failure showed up late.

So this panel:
- gathers evidence before round 1 (§2);
- makes every proposal a bet with a number and a kill criterion (§3);
- checks proposals against a ledger of past attempts;
- runs probes between rounds;
- caps the plan;
- writes an implementation prompt with tiered gates and two run lanes (§5).

**What this run may change:**
- the notes;
- the new prompt file;
- scratch files under `~/.cache/claude-tmp/panel4/`.

It may not change the trainer, runtime, configs or crates, make commits or open a PR.
The implementation run commits the new prompt as its first commit. Short measurements
are allowed under the caps in §7.

**Never ask the owner anything.** Make every decision yourself, on evidence. That
includes linguistic rulings: research them and decide. Nothing in the plan may say
"the owner's call" or "ask the owner".

## 0. Where things stand

flexion learns each language's inflection from Kaikki/Wiktionary dumps. It prints each
language as a Rust crate that *is* the grammar: paradigms, ending rules, sound laws, stem
slots, relations, irregular lines, all readable. Twelve languages ship. Since PR 19 every
language ships every part of speech it inflects, and no cell of any part is ever empty.

**Three panels came before.** Their records are in `~/code/notes/projects/flexion/`:
- `trainer-panel-2026-10-14.md` (PR 16);
- `trainer-panel-2-2026-10-15*.md` (PR 18);
- `trainer-panel-3-2026-10-15*.md` (PR 19), with its coverage inventory and report.

The log of every measurement, including what was tried and dropped, is
`~/code/notes/projects/flexion/grammar-crates.md`. Read the PR 19 section closely, and
PR 16 and 18.

**What PR 19 did:**
- **Error census.** Each wrong form gets a kind in `try`, saved runs and the READMEs.
- **Reproduced and complete.** Each part is scored on its own words asked again, and on
  how many cells get a form.
- **One check through the runtime.** It asks for the dictionary's own words typed three
  ways: from the headword, in full, and with the principal parts given. It counts the
  forms that giving the parts breaks; German verbs had 783.
- **Faster runs** over threads.
- **Real cells.** Latin indicative and subjunctive. Greek futures in indicative and
  optative only.
- **Listed words grouped by class** in `rules.rs`.
- **New parts:** Turkish and Finnish verbs, plus proper nouns, adverbs, pronouns,
  numerals and postpositions wherever the dumps have tables.
- **Compounds and phrases:**
  - a word written in parts inflects its first part where the parts are spaced
    (`Compound::First`);
  - Greek accent kept per cell;
  - Greek breathings stay at the start of the word;
  - `-εως` counts short for the accent.
- **Laws stated once** across those already taken in.
- **No gaps.** A paradigm with nothing in its root cell takes the nearest cell's
  pattern, and a word no paradigm fits takes the commonest one.
- **Gains:**
  - Greek nouns +2.9, adjectives +0.7, verbs +2.6 from the headword;
  - Latin nouns +1.0, adjectives +0.2, verbs +0.6;
  - Turkish adjectives +20 (they now learn from the nouns' words too);
  - crates 13.79 → 12.15 MB for the parts that already shipped, and the new parts add
    3.82 MB.

**Not built in PR 19, or left open.** This panel must settle each one: put it in the plan
with a probe behind it, or retire it with a reason.
- **Item 5, "ready for 900-cell parts"** (left out of PR 19's report by mistake):
  - the runtime resolves a reading's paradigm once per cell rather than once per table
    (Greek verb tables took 81 ms);
  - the printer repeats per-cell scaffolding (~420 bytes per cell) and the relation
    macros (44 KB).
- **Item 6(b):** an ending table writing `derived`.
- **Item 8(e):** Greek preverbs as one unit. Greek verbs still have 27,718 irregular
  forms, against 33,829 before PR 19.
- **Item 9's Sanskrit laws:** the vṛddhi augment, the augment after a preverb, the weak
  perfect stem.
- **Item 11:** Finnish harmony through compound templates.
- **The open disagreement "heads and families, narrowly":** it needs word families, a
  base plus a prefix licensed by junction laws.

**Results on the README tenth,** scored at the end of PR 19. The root README has the full
table. Columns:
- **top-1:** the right form first, from the headword.
- **in full:** the lemma typed with every mark.
- **principal parts:** the dictionary's principal parts given.
- **floor:** the baseline the grammar is judged against.
- **− floor:** grammar minus floor, a paired interval on the same words.
- **irregular:** the share of training words that need an irregular line.
- **wrong forms:** the count, then the census as % of them:
  - *class*: the right form was among the candidates, but another paradigm or reading
    was chosen;
  - *length*: the same letters once marks are off. Despite the name, it counts every
    mark: Finnish ä/a, German umlaut, Russian ё;
  - *accent*: the right letters, with the accent elsewhere;
  - *stem* and *ending*: where the form goes wrong;
  - *variant*: another form the dictionary lists in that cell.
- **by block:** top-1 averaged over the part's blocks (groups of cells).

| language | part | top-1 | in full | principal parts | floor | − floor | irregular | wrong forms: class / length / accent / stem / ending / variant | by block |
|---|---|---:|---:|---:|---:|---|---:|---|---:|
| english | noun | 97.17 | 97.17 | – | 96.78 | +0.39 (+0.24…+0.55) | 0.11% | 671: 63 / 0 / 0 / 4 / 2 / 28 | 97.17 |
| english | proper_noun | 99.35 | 99.35 | – | 99.28 | +0.07 (−0.12…+0.28) | 0.05% | 28: 46 / 0 / 0 / 4 / 0 / 50 | 99.35 |
| english | verb | 97.17 | 97.17 | 99.54 | 97.79 | −0.62 (−0.93…−0.33) | 1.01% | 469: 74 / 0 / 0 / 3 / 3 / 15 | 97.17 |
| english | adjective | 95.35 | 95.35 | – | 92.53 | +2.82 (+2.39…+3.25) | 0.06% | 580: 79 / 0 / 0 / 0 / 0 / 20 | 95.35 |
| english | adverb | 99.01 | 99.01 | – | 98.34 | +0.66 (+0.25…+1.16) | 0.11% | 24: 42 / 0 / 0 / 0 / 0 / 58 | 99.01 |
| russian | noun | 94.65 | 94.65 | 95.81 | 94.72 | −0.08 (−0.51…+0.37) | 0.54% | 1479: 81 / 3 / 0 / 1 / 5 / 6 | 93.62 |
| russian | proper_noun | 90.86 | 90.86 | 92.69 | 93.60 | −2.75 (−4.11…−1.41) | 2.19% | 223: 86 / 5 / 0 / 3 / 3 / 1 | 90.86 |
| russian | adjective | 99.75 | 99.75 | – | 99.76 | −0.01 (−0.12…+0.09) | 0.07% | 420: 69 / 5 / 0 / 0 / 1 / 0 | 98.02 |
| russian | verb | 95.76 | 95.76 | 98.88 | 96.65 | −0.89 (−1.39…−0.40) | 0.75% | 871: 64 / 14 / 0 / 7 / 3 / 3 | 96.04 |
| russian | pronoun | 54.26 | 54.26 | – | 54.26 | +0.00 (+0.00…+0.00) | 30.95% | 86: 91 / 5 / 0 / 0 / 5 / 0 | 54.26 |
| russian | numeral | 94.12 | 94.12 | – | 100.00 | −5.88 (−32.53…+0.00) | 24.68% | 9: 0 / 0 / 0 / 0 / 100 / 0 | 94.12 |
| russian | adverb | 66.67 | 66.67 | – | 83.33 | −16.67 (−33.33…+0.00) | 11.89% | 6: 17 / 0 / 0 / 67 / 0 / 17 | 66.67 |
| latin | noun | 71.12 | 92.93 | 98.70 | 59.09 | +12.03 (+9.41…+14.72) | 1.16% | 5158: 15 / 77 / 0 / 2 / 4 / 0 | 68.89 |
| latin | proper_noun | 59.84 | 92.75 | 98.24 | 60.58 | −0.74 (−4.64…+3.12) | 1.63% | 2056: 8 / 83 / 0 / 2 / 4 / 1 | 57.85 |
| latin | adjective | 87.50 | 99.23 | 99.48 | 71.87 | +15.63 (+13.86…+17.46) | 0.48% | 12698: 2 / 92 / 0 / 0 / 2 / 1 | 81.84 |
| latin | verb | 77.49 | 84.92 | 98.59 | 64.17 | +13.32 (+9.35…+17.33) | 4.69% | 17359: 49 / 35 / 0 / 5 / 4 / 1 | 77.90 |
| latin | pronoun | 6.34 | 19.08 | – | 25.00 | −18.66 (−47.89…+3.14) | 73.75% | 251: 24 / 15 / 0 / 8 / 49 / 4 | 6.34 |
| latin | numeral | 72.73 | 100.00 | – | 72.73 | +0.00 (−36.36…+36.36) | 0.85% | 108: 0 / 100 / 0 / 0 / 0 / 0 | 72.73 |
| latin | adverb | 85.14 | 98.35 | 96.72 | 44.57 | +40.57 (+25.58…+55.62) | 5.56% | 26: 0 / 92 / 0 / 0 / 4 / 4 | 85.14 |
| ancient-greek | noun | 74.79 | 89.15 | 96.09 | 51.28 | +25.16 (+21.24…+28.91) | 8.64% | 2895: 27 / 59 / 6 / 2 / 6 / 0 | 74.79 |
| ancient-greek | proper_noun | 59.73 | 86.50 | 91.00 | 47.42 | +13.90 (+7.80…+20.37) | 5.09% | 811: 15 / 74 / 4 / 2 / 5 / 0 | 59.73 |
| ancient-greek | adjective | 78.02 | 90.76 | 98.02 | 52.46 | +26.37 (+21.84…+30.57) | 2.91% | 4874: 28 / 60 / 4 / 4 / 3 / 1 | 80.60 |
| ancient-greek | verb | 68.04 | 75.67 | 81.10 | 44.20 | +25.05 (+19.23…+30.66) | 46.74% | 22043: 18 / 42 / 4 / 19 / 17 / 1 | 66.21 |
| ancient-greek | pronoun | 40.33 | 40.16 | – | 41.96 | +6.81 (−8.25…+30.49) | 51.72% | 219: 6 / 0 / 14 / 1 / 78 / 0 | 40.33 |
| ancient-greek | numeral | 100.00 | 100.00 | – | 100.00 | +0.00 (+0.00…+0.00) | 21.05% | 0: 0 / 0 / 0 / 0 / 0 / 0 | 100.00 |
| sanskrit | noun | 92.10 | 92.10 | – | 91.70 | +0.40 (−0.73…+1.48) | 1.57% | 1090: 25 / 27 / 0 / 10 / 4 / 7 | 92.10 |
| sanskrit | proper_noun | 95.15 | 95.15 | – | 86.48 | +8.67 (+3.40…+14.54) | 2.13% | 57: 19 / 16 / 0 / 26 / 39 / 0 | 95.15 |
| sanskrit | adjective | 91.91 | 91.91 | – | 90.11 | +1.81 (+0.17…+3.56) | 3.82% | 1088: 20 / 67 / 0 / 6 / 7 / 0 | 39.00 |
| sanskrit | numeral | 63.64 | 63.64 | – | 92.42 | −28.79 (−55.13…−0.93) | 7.69% | 96: 0 / 79 / 0 / 0 / 21 / 0 | 63.64 |
| sanskrit | verb | 89.86 | 89.86 | 90.58 | 89.93 | −0.07 (−2.32…+2.05) | 24.23% | 728: 9 / 28 / 0 / 24 / 35 / 5 | 82.00 |
| sanskrit | pronoun | 69.57 | 69.57 | – | 70.53 | −0.97 (−3.17…+0.00) | 46.67% | 63: 0 / 0 / 0 / 0 / 100 / 0 | 69.57 |
| polish | noun | 93.42 | 93.42 | 95.79 | 93.34 | +0.08 (−0.29…+0.44) | 1.81% | 2920: 68 / 2 / 0 / 3 / 11 / 4 | 93.42 |
| polish | proper_noun | 94.01 | 94.01 | 95.06 | 93.74 | +0.26 (−0.25…+0.83) | 1.36% | 1283: 38 / 2 / 0 / 2 / 6 / 3 | 94.01 |
| polish | adjective | 99.92 | 99.92 | – | 99.99 | −0.07 (−0.09…−0.04) | 0.36% | 67: 66 / 15 / 0 / 7 / 9 / 3 | 80.00 |
| polish | verb | 97.17 | 97.17 | 98.38 | 97.66 | −0.49 (−0.99…+0.02) | 1.35% | 1518: 63 / 8 / 0 / 5 / 7 / 10 | 96.82 |
| polish | pronoun | 81.64 | 81.64 | – | 81.64 | +0.00 (+0.00…+0.00) | 60.71% | 65: 0 / 88 / 0 / 0 / 12 / 0 | 81.64 |
| polish | numeral | 0.00 | 0.00 | – | 0.00 | +0.00 (+0.00…+0.00) | 0.00% | 0: 0 / 0 / 0 / 0 / 0 / 0 | -0.00 |
| polish | adverb | 93.75 | 93.75 | 100.00 | 95.31 | −1.56 (−7.81…+3.12) | 5.23% | 8: 0 / 0 / 0 / 0 / 100 / 0 | 93.75 |
| german | noun | 93.62 | 93.62 | 99.79 | 94.77 | −1.15 (−1.46…−0.85) | 0.53% | 2554: 62 / 6 / 0 / 4 / 12 / 13 | 93.62 |
| german | numeral | 100.00 | 100.00 | – | 100.00 | +0.00 (+0.00…+0.00) | 0.00% | 0: 0 / 0 / 0 / 0 / 0 / 0 | 100.00 |
| german | proper_noun | 91.53 | 91.53 | 99.77 | 90.66 | +0.87 (−1.57…+3.20) | 0.28% | 107: 67 / 0 / 0 / 0 / 7 / 18 | 91.53 |
| german | adjective | 98.79 | 98.79 | 99.88 | 99.02 | −0.23 (−0.73…+0.26) | 0.07% | 1519: 32 / 42 / 0 / 0 / 3 / 23 | 98.04 |
| german | verb | 95.17 | 95.17 | 95.98 | 73.16 | +22.01 (+20.25…+23.80) | 5.11% | 2443: 57 / 1 / 0 / 19 / 6 / 2 | 94.21 |
| german | determiner | 40.00 | 40.00 | – | 40.00 | +0.00 (+0.00…+0.00) | 13.89% | 27: 56 / 0 / 0 / 22 / 22 / 0 | 40.00 |
| spanish | noun | 98.67 | 98.67 | – | 98.67 | +0.00 (−0.20…+0.20) | 0.11% | 81: 69 / 6 / 0 / 5 / 2 / 16 | 98.67 |
| spanish | adjective | 98.50 | 98.50 | 99.67 | 98.60 | −0.10 (−0.35…+0.14) | 0.21% | 108: 81 / 3 / 0 / 5 / 5 / 7 | 85.63 |
| spanish | verb | 98.97 | 98.97 | 99.73 | 99.28 | −0.31 (−0.60…−0.02) | 1.73% | 688: 23 / 25 / 0 / 34 / 16 / 2 | 98.81 |
| spanish | determiner | 90.91 | 90.91 | – | 100.00 | −9.09 (−30.00…+0.00) | 11.54% | 2: 100 / 0 / 0 / 0 / 0 / 0 | 90.91 |
| spanish | adverb | 0.00 | 0.00 | – | 0.00 | +0.00 (+0.00…+0.00) | 42.86% | 0: 0 / 0 / 0 / 0 / 0 / 0 | -0.00 |
| french | noun | 97.59 | 97.59 | – | 97.03 | +0.56 (+0.23…+0.90) | 0.65% | 104: 75 / 0 / 0 / 5 / 1 / 16 | 97.59 |
| french | adjective | 97.50 | 97.50 | 99.95 | 97.72 | −0.22 (−0.65…+0.24) | 0.28% | 138: 81 / 0 / 0 / 3 / 13 / 3 | 48.77 |
| french | verb | 98.70 | 98.70 | 99.48 | 99.28 | −0.59 (−1.06…−0.22) | 0.96% | 454: 67 / 8 / 0 / 0 / 14 / 11 | 98.81 |
| french | participle | 99.84 | 99.84 | – | 99.52 | +0.32 (−0.32…+0.96) | 0.03% | 3: 100 / 0 / 0 / 0 / 0 / 0 | 99.84 |
| french | determiner | 46.15 | 46.15 | – | 69.23 | −23.08 (−64.29…+0.00) | 22.45% | 7: 0 / 0 / 0 / 29 / 71 / 0 | 46.15 |
| french | adverb | 0.00 | 0.00 | – | 0.00 | +0.00 (+0.00…+0.00) | 0.00% | 0: 0 / 0 / 0 / 0 / 0 / 0 | -0.00 |
| italian | noun | 95.82 | 95.82 | – | 95.09 | +0.74 (+0.31…+1.16) | 0.19% | 267: 85 / 0 / 0 / 6 / 0 / 3 | 95.82 |
| italian | adjective | 98.46 | 98.46 | 99.78 | 98.46 | +0.00 (−0.28…+0.28) | 0.19% | 122: 90 / 0 / 0 / 3 / 4 / 2 | 96.56 |
| italian | verb | 99.05 | 99.05 | 99.53 | 99.22 | −0.17 (−0.48…+0.13) | 1.77% | 667: 48 / 1 / 0 / 22 / 19 / 7 | 99.03 |
| italian | participle | 100.00 | 100.00 | – | 100.00 | +0.00 (+0.00…+0.00) | 0.02% | 0: 0 / 0 / 0 / 0 / 0 / 0 | 100.00 |
| italian | determiner | 81.82 | 81.82 | – | 86.36 | −4.55 (−13.64…+0.00) | 13.33% | 4: 25 / 0 / 0 / 50 / 25 / 0 | 81.82 |
| italian | adverb | 0.00 | 0.00 | 0.00 | 0.00 | +0.00 (+0.00…+0.00) | 75.00% | 1: 0 / 0 / 0 / 0 / 100 / 0 | 0.00 |
| finnish | noun | 97.81 | 97.81 | 99.15 | 98.29 | −0.48 (−0.68…−0.29) | 1.34% | 41613: 35 / 31 / 0 / 4 / 21 / 8 | 97.79 |
| finnish | pronoun | 2.14 | 2.14 | – | 9.63 | −7.49 (−16.57…−1.39) | 73.33% | 183: 5 / 4 / 0 / 13 / 78 / 0 | 2.14 |
| finnish | numeral | 32.14 | 32.14 | – | 43.30 | −11.16 (−41.96…+19.73) | 44.00% | 152: 18 / 0 / 0 / 27 / 55 / 0 | 32.14 |
| finnish | proper_noun | 93.86 | 93.86 | 99.03 | 91.77 | +2.10 (+0.76…+3.41) | 2.49% | 7336: 68 / 8 / 0 / 3 / 12 / 8 | 93.80 |
| finnish | adjective | 97.45 | 97.45 | – | 97.41 | +0.04 (−0.62…+0.72) | 0.85% | 1132: 27 / 52 / 0 / 2 / 16 / 3 | 97.31 |
| finnish | verb | 98.49 | 98.49 | 99.48 | 97.56 | +0.92 (+0.22…+1.68) | 3.97% | 2978: 54 / 5 / 0 / 0 / 36 / 0 | 98.32 |
| finnish | adverb | 96.67 | 96.67 | 98.57 | 97.39 | −0.71 (−1.67…+0.00) | 2.32% | 14: 64 / 0 / 0 / 14 / 21 / 0 | 96.67 |
| finnish | postposition | 65.22 | 65.22 | – | 78.26 | −13.04 (−30.43…+0.00) | 4.88% | 8: 75 / 0 / 0 / 0 / 25 / 0 | 65.22 |
| turkish | noun | 96.51 | 96.51 | 99.52 | 93.27 | +3.23 (+2.30…+4.24) | 1.11% | 2485: 67 / 7 / 0 / 11 / 7 / 1 | 96.83 |
| turkish | pronoun | 0.00 | 0.00 | – | 90.91 | −90.91 (−90.91…−90.91) | 61.54% | 11: 0 / 0 / 0 / 0 / 100 / 0 | 0.00 |
| turkish | numeral | 31.82 | 31.82 | – | 0.00 | +31.82 (+0.00…+63.64) | 42.86% | 15: 0 / 0 / 0 / 0 / 100 / 0 | 31.82 |
| turkish | proper_noun | 90.94 | 90.94 | 97.74 | 93.24 | −2.31 (−6.43…+1.59) | 1.97% | 342: 75 / 0 / 0 / 10 / 13 / 3 | 91.56 |
| turkish | adjective | 86.75 | 86.75 | 100.00 | 81.93 | +4.82 (+0.00…+14.46) | 1.11% | 33: 100 / 0 / 0 / 0 / 0 / 0 | 88.51 |
| turkish | verb | 98.65 | 98.65 | 99.72 | 98.68 | −0.03 (−1.42…+1.23) | 59.04% | 2851: 92 / 0 / 0 / 1 / 7 / 0 | 98.78 |

**Size, the generated crates only.** The trainer and runtime may be as large as they
need. The crates total 15.97 MB: 12.15 MB for the parts that shipped before PR 19, plus
3.82 MB of new parts.

| crate | MB | of which new parts |
|---|---:|---:|
| ancient-greek | 3.50 | 0.21 |
| finnish | 2.25 | 1.19 |
| turkish | 1.99 | 1.73 |
| russian | 1.98 | 0.10 |
| latin | 1.43 | 0.20 |
| german | 1.25 | 0.05 |
| polish | 1.19 | 0.20 |
| sanskrit | 0.70 | 0.09 |
| english, spanish | 0.49 each | 0.02, 0.01 |
| italian | 0.42 | 0.01 |
| french | 0.27 | 0.01 |

- **By file kind** (the parts that shipped before PR 19):
  - `rules.rs` 3.09 MB, `paradigms.rs` 2.38, `irregular.rs` 1.82, `relations.rs` 1.49;
  - `stems.rs` 0.65, `mod.rs` 0.56, `animacy.rs` 0.41, `marks.rs` 0.39, `accent.rs` 0.33;
  - the rest under 0.32 each.
- **The largest new parts:** Turkish verbs 1.62 MB (928 cells), Finnish verbs 0.64,
  Finnish proper nouns 0.41.
- **The script:** `~/.cache/claude-tmp/p3/bytes.py` measures all of this.

**Run times** (PR 19, one run at a time):
- **The pooled sweep** (every language's development tenths): 3.2 h.
- **Tenth 9 for all twelve languages:** about 1 h.
- **Rebuilding all twelve crates:** 2.7 h.
  - Finnish 66 min, Greek 21, Turkish 20, Russian 12, German 12;
  - Latin and Polish about 6 each, the rest 1–3.
- **What is cached:** only the sound-law stage, keyed by a hash of its inputs and
  source, and only for `try` and `sweep` (`crates/flexion-train/src/stage.rs`). A fix to
  the printer alone means relearning every crate.
- **The implementation runs' wall time:** PR 16 6.5 h, PR 18 18.3 h, PR 19 18.6 h. In the
  last two, 78–79% went to the final checking and rebuilding.
- **The machine:** 32 cores and 125 GB. Most trainer runs peak near 4 GB; Finnish and
  Turkish peak at 11–15 GB.

**How decisions are judged** (the tools):
- **`cargo xtask try <lang> --pos <part> --quick --save <file>`:** scores the development
  tenth (tenth 2).
- **`cargo xtask compare a b`:** a paired bootstrap on words, with a row per word group.
  It also lists the words that got better and worse.
- **`sweep <dir> --pool`:** every language at once, compared as two directories.
- **`--fold 9`:** an untouched tenth, used to confirm.
- **`--final`:** scores the README tenth, once, at the end.
- **`cargo xtask inspect <lang> --pos <part> --word <w> --parts --holdout`:** one word's
  table, scored the way `try` scores it.
- **`try --diagnose`:** class-choice diagnostics.

**Leads.** These are what the numbers above show, not a plan. Each one needs checking
before anyone builds on it.
1. **Class choice is where most wrong forms are** outside Latin and Greek. Some shares:
   - Russian nouns 81%, Italian nouns 85%, Turkish verbs 92%;
   - English adjectives 79%, English verbs 74%;
   - the seven rows below the floor beyond chance: German nouns −1.15 (62% class),
     Russian verbs −0.89 (64%), English verbs −0.62 (74%), French verbs −0.59 (67%),
     Finnish nouns −0.48 (35%, and 31% marks), Spanish verbs −0.31 (23%, and stem 34%),
     Polish adjectives −0.07 (66%).

   On those rows the floor (a nearest-ending table) chooses better than the grammar's
   rules. Many ways to choose better were tried and dropped (see the list at the end of
   this section). The direct evidence hasn't been read yet: the words the floor gets
   right and the grammar gets wrong. `compare` lists them.
2. **Marks.**
   - Latin: adjectives 92% of wrong forms, nouns 77%, proper nouns 83%. Latin nouns score
     71.12 from the headword against 92.93 typed in full.
   - Greek: 42–74% of wrong forms.
   - The same kind is large where there is no vowel length: Finnish nouns 31%, German
     adjectives 42%, Russian verbs 14%, Sanskrit numerals 79%. Find what these are
     (harmony? umlaut? ё?). If it is harmony or umlaut, it is a bug, not a lead on length.
3. **Greek verbs** score 68.04 from the headword and 81.10 with the principal parts.
   46.74% of training verbs need an irregular line. Wrong forms: stem 19%, ending 17%.
   See item 8(e) above.
4. **Sanskrit verbs:** 24% need an irregular line; wrong forms stem 24%, ending 35%.
   Merging tense pages lost −1.88 and was dropped.
5. **"By block" far below top-1.**
   - The rows: Sanskrit adjectives 39.00 against 91.91, French adjectives 48.77 against
     97.50, Polish adjectives 80.00 against 99.92, Spanish adjectives 85.63 against
     98.50, Latin adjectives 81.84 against 87.50.
   - Either a block of cells (comparatives? one gender?) is mostly wrong, or the scoring
     is off. Find which.
   - Rows whose README tenth has no words print 0.00% (Polish numerals, Spanish and
     French adverbs), which reads as total failure.
6. **Proper nouns.** Latin 59.84 and Greek 59.73 (marks: 83% and 74% of their wrong
   forms). Russian 90.86, −2.75 against the floor.
7. **Closed classes** (pronouns, numerals, determiners, postpositions).
   - Each held-out tenth is a handful of words; a new pronoun is a guess. Latin pronouns
     score 6.34, Finnish 2.14, Turkish 0.00.
   - Their grammars reproduce their own words at 100%.
   - Decide how they should be judged and reported, and whether anything should change.
     Respect the no-gaps rule (§6).
8. **Size.** Turkish verbs carry 928 cells of scaffolding: see item 5 above. Greek is
   22% of all bytes.
9. **Pipeline time.** The final rebuild is 2.7 h even when only the printer changed.
   Possible fixes:
   - a cache of learned grammars, so printing reruns in minutes;
   - building only the crates whose learned grammar changed;
   - two lanes.

   Each is a candidate item. Here, cutting implementation time is a result, not just
   build cost.

**Tried and dropped.** Don't revive these without a new idea and new evidence. The
numbers are in the log and the panel records:
- **From earlier PRs:**
  - gender as a hard filter;
  - a single global, interleaved or far-left vowel-length context;
  - restoring marks when typed in full;
  - Russian stress via the uc-prosody branch;
  - naive-Bayes beginning rules;
  - prefixes for every language;
  - marking cells `derived` wholesale;
  - a fixed 3-letter compound head;
  - compound heads among every listed word (Russian adjectives −0.16);
  - derived cells from another stem slot's cell;
  - choice-rule pricing;
  - the Chu–Liu/Edmonds relation tree;
  - a calibration refit (proposed three times);
  - a global description-length objective;
  - a German umlaut law (umlaut is 7% of German plural errors);
  - hard-EM beyond exact ties;
  - a minimum paradigm size;
  - accent through the sound laws;
  - variant-aware listing;
  - a per-stem-slot class choice;
  - more than 4 paradigm readings per table;
  - an ending classifier for class choice (no better by a point).
- **From PR 19:**
  - Sanskrit tense pages read into their root's verb (−1.88);
  - vowel-length rules that may skip the letter before (Greek −11.02);
  - an ε → η augment law (it breaks about 9% of forms);
  - proper nouns sharing the nouns' grammar (`shares: noun`);
  - learned English pronouns;
  - hyphenated `First` compounds (French −0.12);
  - an incremental settle (not exact);
  - the "informative" filter on every word (Polish −0.16);
  - a runtime fallback for gaps;
  - a fallback accent on the listed-stem path (no change).

## 1. Read first

Read these yourself before convening the panel. You chair, so you must be able to check
claims:
- `panel-process-review-2026-10-16.md`;
- the root README;
- the PR 19 log section;
- the third panel's decision, minutes and report;
- at least four generated crates (one each of Latin or Greek, Russian or Polish, Finnish
  or Turkish verbs, and a Romance language);
- the trainer (`crates/flexion-train/src/`), the runtime (`crates/flexion/src/`) and
  `crates/sound-law/`.

**Literature.** Use services that need no identity: OpenAlex, arXiv, ACL Anthology. Never
send the owner's email address or other personal details to any outside service, and
never put them in a member's prompt.

## 2. The evidence pack, before round 1

Build these in `~/.cache/claude-tmp/panel4/`, under the caps in §7. Panel 3's scripts are
in `~/.cache/claude-tmp/panel3/`; reuse what still fits:
- `samples.sh`, `picks.tsv`, `pick.py`;
- `dossier.py`, `minutes.py`;
- the schemas;
- `codex1.sh`, `codex2.sh`.

1. **The ledger of past attempts.** One file, from `grammar-crates.md` (PR 11–19) and
   the three panel records. One line per mechanism tried:
   - what it was, and the PR;
   - the rows it moved, with the numbers;
   - its verdict: kept, dropped or never built;
   - why.

   Members get it with the context file. Each proposal names its nearest entry and what is
   new.
2. **Answer samples.** As panel 3 did (`samples.sh`): for every language and part of
   speech, about 20 held-out tenth-9 words, chosen at random across the word groups, plus
   words with a wrong cell. Each word's table as `try` scores it, next to the dump's.
3. **The census, read.** For every main row (nouns, adjectives, verbs, proper nouns),
   take 20 wrong forms of each kind over 5% of the row: the form given, the form wanted,
   and for *class*, the class chosen and the right one. Get them from saved `try` runs
   (`--save`, the `error` column), not by hand.
4. **Floor-right, grammar-wrong.** For the seven rows below the floor, the words where the
   floor is right and the grammar wrong (`compare`), with both answers, grouped by what
   the floor knew that the rules didn't.
5. **By block, audited.** For lead 5's rows, top-1 per block, the block that drags them
   down, and why. Also whether empty rows are scored as 0.00.
6. **The "length" kind outside length languages.** Twenty examples each from Finnish
   nouns, German adjectives and Russian verbs, and what they are.
7. **Helpers** (read-only subagents, each under 30 minutes):
   - *trainer/runtime mismatches:* where the trainer judges on one set of words or one
     path and the runtime applies another, PR 18's recurring bug class;
   - *stage timings* per part, from the PR 19 sweep log `~/.cache/claude-tmp/p3/sweep-p3.log` (saved runs in `sweep-p3/` and `t9-p3/`);
   - *bytes per file kind per part,* new parts included;
   - *dead code and duplicate helpers.*
8. **The canary set.** Time `try --quick` per part, from the PR 19 sweep logs or by running
   it. Then pick a fixed set of fast parts, together under about 20 minutes on two lanes,
   that exercises every mechanism:
   - vowel length;
   - accent;
   - harmony;
   - gradation;
   - phrases;
   - compounds;
   - stem slots;
   - relations;
   - listed words;
   - a closed class;
   - a proper noun.

   The implementation prompt screens every item on it (§5).

Members can't run training; this pack is how they see real output. Put it in the
context file.

## 3. The panel

### Members

Six specialities. Each has **two members**: a Claude subagent and a Codex agent (GPT-6
Astra, normal tier, not fast). Both get the same role prompt, but **split the material:**
each reads closely half of the speciality's crates, so the pair covers it all without
duplicating. In panel 3 the twins mostly wrote the same proposals. Each member skims
the other half.

| speciality | Claude member reads closely | Codex member reads closely |
|---|---|---|
| linguist (morphology, paradigm structure; the class-choice errors of the modern languages) | German, Russian, Polish | Spanish, French, Italian, English |
| classicist (Latin, Greek, Sanskrit, as the reference grammars present them) | Ancient Greek (verbs first, item 8(e)) | Latin and Sanskrit (verbs, item 9's laws) |
| phonologist (laws, harmony, gradation, accent, marks) | Latin and Greek `marks.rs`, `accent.rs`; leads 2 and 6 | Finnish and Turkish laws and harmony, German umlaut, Russian; item 11 |
| machine-learning specialist (class choice, the floor, calibration) | the nouns below the floor (German, Finnish) and evidence item 4 | the verbs below the floor (Russian, English, French, Spanish) |
| Rust engineer, with performance (generated code, item 5, run time, the pipeline) | the crates' code and bytes: Turkish and Finnish verbs, Greek | the trainer's run time, `stage.rs`, `xtask` build and sweep: lead 9 |
| data and evaluation specialist (census, by block, closed classes, proper nouns, the dumps) | closed classes and proper nouns in every language | the census and by-block scoring, lead 5, empty rows |

Each member says, with file:line in `languages/<lang>/src/` or the trainer:
- what a linguist would write differently;
- what is redundant, oversized, wrong or missing.

Run the Codex members read-only, with a JSON schema for the answer:

```
timeout 5400 codex exec -m gpt-6-astra -c service_tier=default -s read-only -C ~/code/flexion \
  --skip-git-repo-check --output-schema <schema.json> -o <out.json> - < <prompt.md>
```

Run the Claude members as subagents. Run the members in parallel, no more than about six
at once. Every member is **read-only**:
- no edits in the repo, no training, no builds, nothing over a minute;
- read code, crates, READMEs, notes, the evidence pack and papers;
- cite evidence as file:line or as numbers from the pack, the READMEs and the log;
- mark guesses "unverified".

Budget each member about 30 minutes.

### Every proposal is a bet

Use one JSON schema for every member's answer. Each proposal has:
- **kind:** one of
  - a *bug* with a witness: a word and cell that come out wrong, and why;
  - an *exact change*: bytes or time fall, answers identical;
  - an *experiment*: a change in what is learned, with a predicted gain.
- **the rows it moves,** and the predicted paired change on each, from the headword and
  with the principal parts given;
- **the screening set:** the parts its code reaches, beyond the canary set;
- **the command that measures it;**
- **a kill criterion:** the result that drops it;
- **cost:** implementation hours, run minutes, crate bytes;
- **the nearest past attempt** in the ledger, and what is new. A repeat with nothing new
  is out before the meeting;
- **evidence:** file:line, and words from the pack.

At most six proposals per member. An experiment with no predicted number is out.

### Process

1. **Context file.** One file all members read first:
   - the project and where things are;
   - this prompt's §0;
   - the ledger;
   - the evidence pack's index;
   - the owner's constraints (§6);
   - the read-only rules;
   - the bet format;
   - each member's material.
2. **Round 1, analyses.** Each member writes:
   - a diagnosis;
   - findings in their crates, with file:line;
   - up to six bets.

   Each member also asks for up to two **probes**: runs under about 10 minutes (`try
   --quick` on a part, `inspect` on some words, a grep over saved runs) whose answer would
   confirm or kill one of their bets.
3. **Probes.** You run the probes asked for (merge duplicates, about 10 in all, two lanes,
   about an hour) and add the results to the dossier. A bet its probe killed is
   withdrawn.
4. **Dossier.** All analyses and probe results in one file.
5. **Round 2, the meeting.** Each member reads the whole dossier and returns:
   - **votes:** five points to their best bet across the panel, four to the next, down to
     one;
   - **objections,** each tied to a bet. Each names the measurement that would settle it;
   - **checks:** at least three factual claims by other members, verified against the
     code, true or false with file:line. The Codex members check the engineer's and the
     ML specialist's exactness claims first;
   - their own bets revised or withdrawn.

   No restating: an endorsement with nothing new is a vote.
6. **Your decision.**
   - **Check the facts yourself.** Re-check, read-only, every fact the plan rests on. You
     may run a short probe to settle a contested claim.
   - **Follow the votes.** In panel 3 every top-voted bet paid, and the two items the chair
     kept against the votes both lost. An item below the vote median needs a written
     reason, with evidence.
   - **Write the decision:**
     - *what the panel agrees on:* the state of the crates, and where the errors and the
       bytes are;
     - *the plan:* **at most eight items**, plus **at most three experiments**, which run
       last and are screened in 30 minutes or less each. Each item carries its bet:
       kind, rows, predicted change, screening set, command, kill criterion, cost;
     - *the not-built items above:* each one in the plan or retired, with a reason;
     - *rejected or deferred bets,* with reasons and the ledger entries behind them;
     - *open disagreements,* each with the measurement that settles it.
   - **Rank** by evidence and by what the owner values, in this order:
     1. correct forms;
     2. readable, minimal crates;
     3. build cost and implementation time.

     A correct output nobody scores counts as much as a score.
   - **Size the plan** so its implementation fits in about 12 hours of wall time, the
     final pipeline included (about 4 hours on two lanes: §5). Estimate each item's hours,
     and cut from the bottom.

**The record**, in `~/code/notes/projects/flexion/`, named with the date of the run:
- `trainer-panel-4-<date>.md`: the decision;
- `-dossier.md`: all analyses and probes;
- `-minutes.md`: the meeting, with votes and checks;
- `-ledger.md`: the ledger.

Write them in plain language. Define each term once (relation, listed word, floor, paired
comparison, dev tenth, canary set).

## 4. Members' rules, shared

- No gaps and no attestation thresholds. A cell's rarity is status and reason, never an
  empty cell or a removed one.
- Every part of speech a language inflects ships.
- Nothing language-specific in the trainer or runtime.
- No "the owner decides".

## 5. The implementation prompt

Write `~/code/flexion/prompts/trainer-panel-4-plan.md`. Model it on
`prompts/trainer-panel-3-plan.md`: same plain language, every term defined once. It must
contain:

- **Where things stand,** as you measured it: the results table, sizes per crate and per
  file kind, run times.
- **Read first:**
  - the panel's record and the ledger;
  - the code it cites, with file:line;
  - the literature behind each linguistic choice, with page or section numbers.
- **The items, in order.**
  - Order: exact and diagnostic items first, then bugs, then experiments. Items that change
    the extracted data come first of all (see "freeze the baseline").
  - For each item:
    - what to change, and the evidence;
    - its kind and bet: the rows, the predicted change, the kill criterion;
    - its screening set;
    - its cost.
  - Decided items are marked decided.
- **Not in this PR:** what was rejected, deferred or retired, with reasons.
- **What must stay true** (§6).
- **How the run works.** Copy §7's rules. Add that the run never asks the owner anything,
  and that its long runs are approved up front:
  - sweeps;
  - tenth-9 runs;
  - single-language learns of any length;
  - the rebuild of every crate.
- **Judging changes:** the tiered gates, written out as follows.
  1. **Freeze the baseline first.**
     - Make every extraction change.
     - Re-extract.
     - Score `main`'s binary on the new data: a pooled sweep, from the headword and with
       the principal parts given. All later compares are against this.
     - A later re-extraction means scoring the baseline again.
  2. **Screen each item as it lands** (≤ 20 minutes): `try --quick` on its screening set
     plus the canary set, from the headword and with the principal parts given, paired
     against the previous item's run. An exact item instead shows identical saved runs on
     the canary set.
  3. **Keep or drop,** by these rules:
     - **exact:** kept if answers are identical and bytes or time fall;
     - **bug fix:** kept if no row loses beyond chance. A loss beyond chance is found with
       `inspect --parts --holdout`, then fixed or narrowed;
     - **experiment:** kept only if its target rows gain beyond chance and no row loses
       beyond chance by more than 0.1 points. Otherwise dropped, logged with its numbers;
     - **a correct output nobody scores** (new cells, phrases): kept if no row loses beyond
       chance by more than 0.1, with the losing words inspected.
  4. **Re-screen after a late fix.** A fix made after an item was screened re-screens
     every part the changed code reaches, in both input modes.
  5. **No full run while anything is unmeasured.** No sweep, tenth-9 run or rebuild while
     any item is unscreened.
  6. **One integration sweep** once every item is in, against the baseline. A loss there
     is investigated (by a subagent that returns the cause) and fixed or the item
     dropped. Then rerun only the languages it touched.
  7. **The pre-launch checklist,** before the final pipeline:
     - zero gaps in the sweep log;
     - Sanskrit, French, English and one crate per new kind of part built and tested;
     - fmt and clippy clean;
     - a clean tree;
     - the binary copied to a versioned name, never overwritten while a run uses it.
  8. **The final pipeline, once, on two lanes:**
     - the crate builds, longest first (Finnish first), in one lane;
     - tenth 9, then the README tenth, in the other.

     Tenth 9 is a confirmation. A loss seen only there, not beyond chance or under 10
     words, is logged, not chased, unless the words show a bug.
  9. **After the pipeline,** a fix rebuilds only the crates it changes.
- **The README is written once,** by script from the final build. Every number and claim
  in its prose is read from the tables or the census, with the column checked.
- **Agent hygiene:**
  - a run-state file at `~/.cache/claude-tmp/panel4-plan/state.md`: binary hashes, what
    is running, what is decided, which rows lost;
  - a watcher on each stage, reading results as they land, never idle while a result
    sits on disk;
  - investigations and bisects go to subagents that return the conclusion;
  - commit everything before pushing.
- **Git.** One PR off the latest `main`, with this prompt as its first commit. fmt and
  clippy before pushing. `gh pr merge --merge` once CI is green, then check out the latest
  `main`. No publishing.
- **Done means:**
  - every item ends with a verdict line in the log: built, with numbers; dropped, with
    numbers; or not built, with the reason. Nothing is silently skipped;
  - no significant loss anywhere unless accepted under the rules above;
  - total crate bytes reported per crate and per file kind;
  - every crate regenerated and compiling, README examples passing, CI green;
  - READMEs re-scored once on the README tenth;
  - the wall time of each stage, against the 12-hour budget.
- **Final report:**
  - the PR link;
  - a table of every item: its bet against the result;
  - per language and part of speech, before and after: top-1, typed in full, principal
    parts, floor and grammar − floor, census, irregular lines, calibration;
  - crate bytes;
  - how each open disagreement was settled;
  - run times;
  - how it was verified, with commands starting `cd ~/code/flexion`.

Before you finish, read the prompt once as the agent that will run it. Every number has a
source, every file:line is checked, and nothing assumes this conversation.

## 6. What must stay true

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

## 7. How you work

- **No `/tmp`.** Scratch goes in `~/.cache/claude-tmp/panel4/` (this run) and
  `~/.cache/claude-tmp/panel4-plan/` (the implementation run).
- **Resource caps:**
  - **two lanes:** at most two trainer processes at once, each with at most 8 threads,
    each under `systemd-run --user --scope -q -p MemoryMax=16G -p MemorySwapMax=0`;
  - Finnish and Turkish verb learns never run at the same time;
  - `cargo` with `-j 4`;
  - no more than about six panel members at once.
- **Approved up front for this run:**
  - the panel's Codex and subagent runs;
  - the evidence pack;
  - the probes;
  - `try --quick` and `inspect` runs to settle a contested claim.

  Nothing else that runs over about 10 minutes. Before a long wait, tell the user what
  you're waiting on and for how long.
- **Clean up.** Kill everything you start, by PID. Never `pkill -f` or `pgrep -f | kill` a
  pattern that appears in your own command line: it kills your own shell.
- **Talking to the user.** Plain language, no internal labels without saying what they
  mean. Status reports say what is done, what is running, what is blocked, and when it
  will finish.

## 8. Done means

- **The evidence pack** is built and indexed, the ledger included.
- **The panel's record** is written: decision, dossier with probes, minutes with votes
  and checks, ledger. Every fact the plan rests on has been re-checked by you.
- **Every not-built item and open disagreement** from §0 is in the plan or retired, with a
  reason.
- **The implementation prompt** is at `prompts/trainer-panel-4-plan.md`. It is
  self-contained, at most eight items plus three experiments, sized to about 12 hours,
  with the tiered gates written in. It is not committed.
- **Nothing in the repo changed** but that file. No process you started is still running.

## 9. Final report

The report is the deliverable the owner reads. Give it in full in your final message, and
save a copy next to the panel's record. Say plainly what you found and what you recommend:
- **What the evidence pack showed:**
  - the census read;
  - the floor-right words;
  - the by-block audit;
  - what the "length" kind is outside length languages.
- **What the panel found:**
  - the state of the crates, read as grammars;
  - where the errors and the bytes are;
  - what is wrong, redundant or missing, with examples from the generated code.
- **Who sat,** what each pair covered, and the votes.
- **The probes,** and which bets they confirmed or killed.
- **The recommended plan,** ranked. For each item:
  - what, and why;
  - the bet: kind, rows, predicted change, kill criterion;
  - the evidence;
  - the expected gain in accuracy and bytes;
  - the cost in hours;
  - how it will be judged.
- **The not-built items,** each planned or retired.
- **Rejected or deferred,** with reasons.
- **Open disagreements,** and the measurement that settles each.
- **Risks,** and what you would cut first if the run has to be cut short.
- **Paths:** the record, the ledger, the report, and the implementation prompt.
