# Prompt: the fourth panel's plan — laws that read the right letters, a shipped grammar that is the scored one, scores that say what they measure

You own one PR on `~/code/flexion`. Branch from the latest `main`, at or after `95536ce`
("Merge pull request #20"). Commit this prompt as the PR's first commit.

**The panel.** A fourth panel of twelve met on 2026-10-16:
- six specialities: linguist, classicist, phonologist, ML, Rust engineer, data and
  evaluation;
- each with a Claude and a Codex member, who split their crates between them.

Before anyone proposed, the panel built an evidence pack:
- every main part run with every form written out;
- the census read with examples;
- the words the floor gets right and the grammar wrong;
- answer samples;
- the ledger of past attempts.

Members made 57 bets, each with a number and a kill criterion. The chair ran 24 probes,
the panel voted, and the chair decided the plan below. The record:
- `~/code/notes/projects/flexion/trainer-panel-4-2026-10-16.md`: the decision. Read it
  first;
- `-evidence.md`, `-dossier.md` (every bet, every probe), `-minutes.md` (votes,
  objections, checks), `-ledger.md` (every mechanism tried since PR 10, with numbers).

The evidence itself is in `~/.cache/claude-tmp/panel4/`:
- `dump/<lang>-<part>.forms.tsv`: every dev-tenth form, wanted against given, with the
  floor's form;
- `dump/<lang>-<part>.tsv`: saved runs for `compare`;
- `dump/*.log`: the full `try` output;
- `evidence/`: the census read;
- `probes/`: every probe's output and write-up;
- `samples/`;
- `src/`: a scratch copy of main whose `xtask try` writes the forms dump when
  `PROBE_DUMP=<file>` is set. Reuse it for witnesses; never commit it.

This PR carries out the plan: eight items and three experiments. You may change
anything if it serves that: runtime, trainer, configs, `sound-law`, generated crates,
tests, CI, xtask, APIs. The project is pre-release: no backwards compatibility and no
shims. Delete what a better design replaces. An item that doesn't pay is dropped, with
its numbers. No prototypes or spikes: what lands is finished.

**Check the panel's evidence.** Where the code or a run says otherwise, follow the code
and log why.

**Never ask the owner anything.** Make every decision yourself, on evidence. That includes
linguistic rulings: research them and decide, citing the reference grammar. Keep working
until the PR is merged.

**This run is budgeted at about 12–14 hours of wall time,** the final pipeline included.
The last two implementation runs took about 18 hours each. About 80% of that went to
full runs one after another, restarted when an unscreened item or an unchecked build
failure showed up late (`~/code/notes/projects/flexion/panel-process-review-2026-10-16.md`).
§3 is how this run avoids that. Follow it exactly.

## 0. Where things stand

**Terms:**
- **dev tenth:** tenth 2, scored by `try`. **Tenth 9:** an untouched confirmation tenth.
  **README tenth:** tenth 0, scored once with `--final`.
- **floor:** the SIGMORPHON non-neural baseline (`crates/flexion-train/src/baseline.rs`).
  It learns each cell from the lemma alone; the grammar is judged against it word by
  word.
- **paired comparison:** `cargo xtask compare a b`, a bootstrap over words, with words
  better and worse.
- **listed word:** a word the grammar names with its class. **Irregular line:** a word's
  own forms where no paradigm builds them.
- **harmony law and tier:** a law fills an ending's class letter from the nearest letter of
  its tier, the letters it looks back at. **Copy law:** a class whose value is that tier
  letter itself.
- **canary set:** the fixed screening set in §3.

**Results on the README tenth** (PR 19, `main`). Columns:
- **top-1:** from the headword. **in full:** the lemma typed with every mark.
  **principal parts:** the dictionary's principal parts given.
- **floor**, and **− floor:** the grammar minus the floor, paired.
- **irregular:** the share of training words with an irregular line.
- **wrong forms:** the count, then % class / length (any mark) / accent / stem / ending /
  variant.
- **by block:** top-1 averaged over blocks of cells (misleading, item 3).

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

**Size** (generated crates only; the trainer may be as large as it needs). Total 15.97 MB:
12.15 MB for the parts that shipped before PR 19, plus 3.82 MB of new parts. Per crate:
- ancient-greek 3.50;
- finnish 2.25, turkish 1.99, russian 1.98;
- latin 1.43, german 1.25, polish 1.19;
- sanskrit 0.70;
- english 0.49, spanish 0.49, italian 0.42, french 0.27.

By file kind across all parts:
- rules.rs 3.42, paradigms.rs 3.03, irregular.rs 2.84, relations.rs 2.29;
- mod.rs 1.03, stems.rs 0.93;
- the rest under 0.5 each.

Largest parts:
- Greek verbs 2.4 MB (irregular.rs 958 KB);
- Turkish verbs 1.58 MB (irregular.rs 750 KB, mod.rs 260 KB);
- Russian nouns 911 KB;
- German nouns 721 KB.

Per part: `~/.cache/claude-tmp/panel4/bytes-parts.md`. Script:
`~/.cache/claude-tmp/p3/bytes.py`.

**Run times:**
- **`try --quick` per part** (measured by the panel, with the dump's overhead):
  - most parts under 2 minutes;
  - German nouns 242 s, Russian adjectives 209 s, Finnish proper nouns 177 s;
  - Finnish verbs 442 s, Turkish verbs 542 s, Greek verbs 648 s;
  - Finnish nouns about 16 minutes (11 GB).

  All 44 main parts ran in 36 minutes on two lanes (`~/.cache/claude-tmp/panel4/dump/lane*.times`).
- **Pooled sweep (`sweep --pool`):** 3.2 h, one at a time.
- **Tenth 9, all languages:** 1.1 h.
- **Rebuilding all twelve crates:** 2.6–2.7 h (two rebuilds in PR 19). Finnish 66 min, Greek 21, Turkish 20, Russian
  12, German 12.
- **What is cached:** only the sound-law stage, and only for `try` and `sweep`
  (`crates/flexion-train/src/stage.rs`).
- **The machine:** 32 cores, 125 GB. Most runs peak near 4 GB; Finnish and Turkish at
  11–15 GB.

## 1. Read first

- **The decision**, `trainer-panel-4-2026-10-16.md`. Then its probe results (in the
  dossier's last section, and `~/.cache/claude-tmp/panel4/probes/summary.md`).
- **`panel-process-review-2026-10-16.md`:** why §3 is as it is.
- **The PR 19 section** of `~/code/notes/projects/flexion/grammar-crates.md`.
- **The code each item cites:**
  - harmony tiers: `crates/flexion-train/src/laws/harmony.rs` (module doc, `fit` at
    :832, `copies` at :821);
  - front laws: `crates/flexion-train/src/laws.rs` (`fronts` at :2576);
  - mark roots: `crates/flexion-train/src/learn.rs:1853` (`mark_roots`);
  - word fitting and ties: `learn.rs:3056–3102` (`builds`, `fit`);
  - the listing judge: `learn.rs:3564, 3621–3629`;
  - comparison: `crates/flexion-train/src/eval.rs:300` (`same_form`);
  - by block: `eval.rs:533–550`;
  - stems: `crates/flexion/src/paradigm.rs:234, 250`, `crates/flexion/src/stems.rs:49`;
  - the listed candidate: `crates/flexion/src/grammar.rs:776`;
  - the accent-class filter: `grammar.rs:1561–1571`;
  - cells: `crates/flexion-train/src/cells.rs:19, 28`, and the printer's per-cell index at
    `print.rs:1684`;
  - the printer: `crates/flexion-train/src/print.rs:376, 696, 1040`;
  - `xtask/src/bench.rs:92` (whole-table timing);
  - the build: `xtask/src/main.rs`.
- **The references:**
  - Belth 2024, 2025 and Burness & McMullin 2019 for tier learning, as the harmony
    module cites them;
  - Smyth §§ 428–461 (augment; temporal augment § 435);
  - Allen & Greenough §§ 10–12 (Latin quantity);
  - Karlsson, *Finnish: An Essential Grammar* (vowel harmony; in a compound the last
    member decides);
  - the RAE's *Nueva gramática* on -ísimo.

## 2. The items

**Run order:**
1. step 0 (§3);
2. items 1, 2, 3 (exact and diagnostic);
3. items 4, 8 (bugs);
4. items 5, 6, 7 (experiments);
5. E1, E2, E3.

Numbers are the decision's. Each item: what, the bet (rows, predicted change, kill), its
screening set beyond the canary set, and its cost. Items 1–8 are decided; their method
is free, their gate is not.

### Item 1. The pipeline keeps what it learned (exact)
- **What:**
  - Save each part's learned grammar and every other printer input (feature selection,
    views, sound-law evidence, measurements; `print.rs:376, 696, 1040`) under a key: a
    hash of the extracted data, the config, the trainer's and printer's source, the same
    way `stage.rs` keys the sound-law stage.
  - `cargo xtask build` reprints from the saved inputs when only the printer changed, and
    writes only the files whose bytes changed.
  - `sweep` and the build take a `--lanes 2` (or equal), running two parts or languages
    at once from one binary: Finnish and Turkish verbs never together, longest first.
- **Gate:** byte-identical crates printed from the cache against a fresh learn, for
  Sanskrit, French and Finnish. Saved runs identical from one lane and from two.
- **Kill:** any byte differs.
- **Expected:** a printer-only rebuild of all twelve in minutes. A cold build on two
  lanes takes at least Finnish's 66 minutes plus whatever doesn't fit beside it; that
  is an estimate, so measure it.
- **Cost:** 1.5 h.

### Item 2. One index, one table read: PR 19's item 5 (exact)
- **What:**
  - `Cell::index` written as arithmetic over the cell groups' sizes, not a match arm per
    cell. The printer emits the arms at `crates/flexion-train/src/print.rs:1684`
    (`let cell = match self {`): 547 KB in 75 functions today. The trainer's
    `GroupCells::index` (`crates/flexion-train/src/cells.rs:19, 28`) is the arithmetic to
    print;
  - the 75 identical relation-macro bodies become one macro in `flexion-macros`;
  - `Grammar::table` reads each reading's paradigm and stem slots once per word, not once
    per cell (`paradigm.rs:250`, `stems.rs:49`).
- **Baseline:** time `cargo xtask bench turkish --pos verb` and `bench ancient-greek --pos
  verb` (whole-table timing, `bench.rs:92`) before and after.
- **Gate:** the end-to-end test, and the canary set's saved runs identical.
- **Expected:** about −0.6 MB; Turkish verb table time down at least 20%.
- **Kill:** any answer differs.
- **Cost:** 1.5 h.

### Item 3. Scores that say what they measure (answers unchanged)
- **(a) The census by script.** The "length" kind is a wrong mark the orthography folds or
  drops (its `marks`), compared per script, not "any combining character". Devanagari
  vowel signs and virama, Russian й, Finnish and German ä stop counting as length.
  - Witnesses: `probes/D2-sanskrit-census.md` (only 4.5% of Sanskrit adjective "length"
    errors are length).
  - If a better name fits (`marks`), rename the kind everywhere.
- **(b) Empty rows and blocks.**
  - A row with no held-out words prints "–", not 0.00%.
  - By block shows each block's form count, and blocks under 30 forms are named apart,
    not averaged in.
  - Fix the −0.0 (the mean at `eval.rs:551` over no blocks: Rust's sum of an empty `f64`
    iterator is −0.0; Polish numerals print −0.00).
- **(c) Small parts.** For closed classes and any part under 30 held-out words, the
  headline is "reproduced N of N" (the grammar on its own words), with a new word's
  score beside it, measured by leaving each word out in turn. No calibration refit.
- **(d) Greek: compatible with stated length.**
  - A second, named score beside the strict one, applied to grammar and floor alike. A
    bare α/ι/υ in the dictionary counts as "length unstated" only where that word's table
    leaves the same stem vowel bare in every cell.
  - The strict score stays the headline and is never reported as a gain in correct forms.
  - `same_form`'s use in the trainer's listing judge (`learn.rs:3564, 3629`) is not
    changed here; the disagreement section below measures that apart.
- **(e) The shipped grammar against the scored one.** `build` prints, per part, the laws
  (with tiers) and the irregular-form count of the grammar it ships beside those of the
  README-tenth grammar. Any law or tier that differs is listed in the build log and the
  crate README's notes. This is how Turkish verbs' bad tier hid: the scored grammar had a
  vowel tier, the shipped one did not.
- **Gate:** every row's top-1 identical.
- **Cost:** 1.5 h.

### Item 4. Laws read the right letters (bug; the panel's first vote, 55 + 14 points)
- **What, generically** (`laws/harmony.rs`):
  - **A class's tier starts from letters of its kind.** For a class whose values are
    vowels (`Orthography::is_vowel`), the starting tier is the vowels, plus boundary
    symbols (space, hyphen) where the data shows them deciding. It is not the whole
    alphabet.
  - **Blockers:** the Finnish tiers' evidenced non-vowel letters (`-`, `z`, `ž` in
    `languages/finnish/src/adjective/laws.rs:24` and proper nouns) are re-examined. Keep
    those that decide within tolerance after the kind rule; log the rest.
  - **Copy laws:** a copy whose values are consonants (English `S` of s/x/z) reads only
    the letter just before the boundary. A copy of vowels (the Finnish illative `-Vn`)
    reads the nearest vowel.
  - Nothing names a language.
- **Witnesses:**
  - `languages/turkish/src/verb/laws.rs:19, 24`: law0's tier is `a|b|d|g|i|o|u|v|ü|ı`,
    "7,470 forms wrong", so zıplatmak → *zıplatmadıler*;
  - `languages/english/src/verb/laws.rs:20, 26`: foxproof → *foxproofx*, wizen → *wizenz*
    (`probes/p4b-english-laws.txt`).
- **Judged on the full-data learn,** because the dev grammar already had the good tier
  (`~/.cache/claude-tmp/panel4/dump/turkish-verb.log:10`):
  - learn Turkish verbs on every word before and after: `build turkish`, about 20
    minutes plus compiling. `try` always holds a tenth out (`--train-share` too), so it
    can't show this; with item 1, the build's learn is cached for reprints;
  - after: law0's tier is vowels only, and listed forms fall from 32,526 to under 10,000
    (irregular.rs under 150 KB);
  - item 3(e) shows no law differing between the shipped and README-tenth grammars.
- **On the dev tenth:**
  - English verbs +0.2 or better from the headword, foxproof and wizen right in both
    input modes;
  - Finnish nouns, verbs, adjectives and Sanskrit verbs identical or better (screening set
    beyond the canary set: Finnish nouns, Finnish verbs, Turkish verbs).
- **Kill:** a consonant left in the full-data Turkish vowel tier, or any screened row
  losing beyond chance.
- **Cost:** 1.5 h.

### Item 8. Impossible forms, one by one (bugs with witnesses)
Run it right after item 4. Each witness is right from the headword afterwards, or
explained in the log:
- **Stem pieces anchored to their vowel.** A stem change's inserted letter binds to the
  vowel it changes, not wherever two adjacent stem pieces split.
  - Spanish reasentar → *reasenito*, sarmentar → *sarmenito* (`probes/p4-spanish-laws.txt`,
    `p10-spanish-diphthong.txt`).
  - Polish dziki → *dzżizy*, from `languages/polish/src/adjective/paradigms.rs:323, 344`
    (`probes/p9-polish-comparative.txt`).
- **Spanish -ísimo.** A final vowel and a written accent before it drop, by laws learned
  from the superlatives the dictionary has: prudente → prudentísimo (not
  *prudenteísimo*), hábil → habilísimo (not *hábilísimo*;
  `probes/p9b-spanish-superlative.txt`).
- **The Italian article keeps its listed alternatives.** `lo` and `gli` appear in no
  generated file: `languages/italian/src/determiner/irregular.rs:14` lists only `i` and
  `la`, and `grammar.rs:776` returns only the listed candidate. The ordered spellings
  `il|lo|l'`, `i|gli` come back as the answer's alternatives.
- **French adjectives `obstiné` and `commissionné`.** Their adjective entries' head
  template is `fr-adj|mf`, which marks them the same in both genders
  (`~/.cache/flexion/dumps/French.jsonl` lines 97743 and 317094). That is an editors'
  error: the same words' participle entries (lines 97745, 317093) give obstinée and
  commissionnée. Repair it in the French config, in step 0. Either:
  - a rule: an adjective marked `mf` whose same-spelled participle entry gives a
    distinct feminine takes that feminine; count how many words it touches;
  - or the two entries listed with this provenance. Held out, the grammar already says obstinée
  (`probes/p5b-french-adj-holdout.txt`).
- **French `bien`.** Its superlative is answered `bien`, the lemma standing in, which the
  no-gaps rule forbids (`probes/p8b-french-adverb.txt`). Build the adverb superlative as
  the language does: the phrase *le mieux*, as the config's phrase cells allow.
- **Screening set beyond the canary set:** Spanish adjectives, Polish adjectives, Italian
  determiners, French adverbs.
- **Expected:**
  - Spanish verbs +0.05 to +0.15;
  - Polish adjectives' gap to the floor about 0;
  - Spanish adjectives +0.05.
- **Kill** (per fix): any canary row loses.
- **Cost:** 2 h.

### Item 5. The Greek augment as one law per front literal (experiment; 37 points)
- **What** (`laws.rs`, `fronts` at :2576):
  - learn the augment, vowel reduplication and the temporal augment as one pooled front
    law per literal (`ἐ‹V → V̄`), not one law per vowel and place;
  - a relation's change at a form's front goes through the front laws too
    (Sanskrit *अअविष्यध्वम्*, 106 dev forms).
- **Break test:** the 5% test stays per left context and per input vowel, so ε → η
  (breaks 9%, PR 19) stays out.
- **Witnesses:** ὑστερέω → *ἐὑστέρουν*, αὐξάνω → *ἐαὔξᾰνον*, ἐνοικέω → *ἐνεοίκουν*
  (`probes/p1-greek-augment.txt`). Smyth §§ 428–438.
- **Judged:**
  - Greek verbs from the headword +1.0 predicted, with principal parts +0.4;
  - the compare is split into preverb-initial lemmas (the crate's prefixes) and the rest,
    and the item is credited only with the rest;
  - Sanskrit verbs +0.2.
- **Kill:** under +0.5 on the rest, or any Greek row losing.
- **Screening set beyond the canary set:** Greek verbs (about 11 min), Sanskrit verbs.
- **Cost:** 2 h.

### Item 6. Latin vowel length learned from every form (experiment; 33 + 9 points)
- **What** (`learn.rs:1853`, `mark_roots`):
  - learn a root's vowel lengths from every form of every training lexeme, not only its
    lemma spelling;
  - form-of pages vote under the lexeme they are a form of, one vote per lexeme;
  - the agreement threshold is 0.9, fixed before tenth 9 (unanimity also measured on
    the dev tenth and logged);
  - a part whose net on the dev tenth is negative takes no roots;
  - in Greek, a bare α/ι/υ casts no vote.
- **Evidence:** `probes/D3-latin-roots-every-form.md`. Leak-free: Latin nouns +0.79,
  adjectives +0.83 (174 fixed against 24 broken, and 820 against 0).
- **Judged:** Latin nouns and adjectives +0.8 each, verbs and proper nouns 0 or better;
  confirmed on tenth 9.
- **Kill:** nouns under +0.5 on either tenth, or any Latin or Greek row losing.
- **Cost:** 1.5 h.

### Item 7. Finnish compound domains for harmony, nouns first (experiment; PR 19's item 11)
- **What:**
  - a word is split at a boundary only where training evidence validates it: both parts
    are training headwords, and no held-out word's own template is used;
  - harmony then reads the last member;
  - the harmony twins (`languages/finnish/src/adjective/paradigms.rs:428–444`,
    `tasainen` against `kansainvälinen`, "like Tasainen, except in 14 cells") become one
    paradigm writing the class letter where the split makes the law hold;
  - adjective superlatives (`puna+isin`) are not split.
- **Evidence:** `probes/H2-finnish-compound-domain.md`. Nouns +2,742 forms recovered and
  135 broken; adjectives 70 and 0.
- **Judged:**
  - Finnish nouns from the headword +0.10 predicted (kill below +0.05);
  - adjectives +0.15;
  - Finnish verbs and proper nouns identical or better;
  - screening set: Finnish nouns (16 min), Finnish verbs.
- **Cost:** 2 h.

### Experiments (last; each screened in 30 minutes or less)
- **E1. Greek preverbs as one unit** (PR 19's item 8(e)).
  - **What:** a preverb's allomorphs (ἀπο-/ἀπ-/ἀφ-, συν-/συμ-/συρ-, ὑπο-/ὑπ-/ὑφ-) are one
    morpheme, told by junction laws, so the augment goes after it (συρράπτω → συνέρραπτον).
    277 of the 315 ὑ-initial verbs are ὑπ- compounds.
  - **Judged:** Greek verbs +0.5 on preverb-initial lemmas; irregular forms 27,718 →
    about 23,000.
  - **Kill:** under +0.3, or Greek nouns losing.
- **E2. Tied words vote by evidence.**
  - **What:** a word several classes fit equally (`learn.rs:3094`, today the lowest
    index) gives its vote to the class whose training words most often have the word's
    missing cells. Singular-only German `-smus` and Polish `-wanie` nouns now inflate the
    plural-less class (`probes/T1-ties.md`).
  - **Not exact:** answers can change (the meeting's checks).
  - **Judged:** German nouns +0.12, Polish nouns +0.12.
  - **Kill:** both under +0.05, or any row losing.
- **E3. The typed accent chooses the stress class.**
  - **What:** a filter already exists. `crates/flexion/src/grammar.rs:1561–1571` keeps
    only the stress classes that put the given stress where the headword has it. Yet the
    witnesses' answers come out "listed" with the wrong accent, so find the path that
    bypasses it (a listed word's own class, the spellings, or `given` unset from the
    headword) and make the headword's accent hold on every path.
  - **Witnesses:** Τιμόθεος → *Τῑμοθεός*, κατάβασις → *κᾰτᾰβᾰ́σῐς*
    (`probes/p2-greek-heads.txt`, `p2b-greek-heads-proper.txt`).
  - **Judged:** Greek proper nouns +0.8, nouns +0.2.
  - **Kill:** under +0.2 on proper nouns, or any Greek row losing.

### Open disagreements: measure each and log it
- **Greek bare dichrona in the trainer.** Two Greek runs (verbs and nouns):
  - (a) item 3(d) as specified;
  - (b) the same rule also in the listing judge.

  Report irregular forms, irregular.rs bytes and strict and compatible top-1 for each.
  Land (b) only if strict top-1 doesn't fall. Also check that grammar − floor keeps its
  sign and significance under both scores on tenths 2 and 9.
- **The tier rule's scope:** item 4's Finnish and Sanskrit controls settle it.
- **Latin roots, unanimity against 0.9:** item 6 logs both.

### Not in this PR (the decision gives the reasons)
- **Sanskrit athematic stems, augment maps, weak perfect and preverb junctions:** deferred.
  Probe P3: 12% of Sanskrit verb forms are built by no paradigm. Next panel.
- **Kept prefixes for Polish and Russian; Russian вы-; words through listed stems not
  voting.**
- **Russian names' animacy, per word:** probe B2.
- **Latin names' marks:** probe B1; measure again after item 6.
- **Taking back listed forms a word's own line tells; unlisting Greek accent classes.**
  Exactness isn't shown: `misbuilt` compares first candidates only.
- **Smoothing sweeps, calibration refits, harmony as an ending rule (killed by H1),
  French vowel alternation, Italian futures, lemma-to-form overrides, bootstrap by key.**
- **Retired:** PR 19's item 6(b) and "heads and families, narrowly".

## 3. Judging changes: the tiered gates

1. **Step 0, freeze the baseline.**
   - Make every extraction change first: item 8's French repairs, and any other change to
     what the configs read. Re-extract.
   - Build `main`'s binary (`git worktree` or a copy of `main`, under
     `~/.cache/claude-tmp/panel4-plan/`) and score it on the new data:
     - a pooled sweep (`sweep <dir> --pool`; it scores the headword and the principal
       parts);
     - tenth 9 for every language;
     - the canary set's saved runs.

     All three on two lanes, run by hand until item 1's lanes exist: two shell loops of
     `try` calls, one per lane.
   - Every later compare is against these. A later re-extraction means scoring the
     baseline again.
2. **The canary set.** Each part runs `try <lang> --pos <part> --quick --save
   <dir>/<lang>-<part>.tsv` (one directory per screen, so `compare` pairs files by name),
   about 13 minutes on two lanes.
   - **Lane 1:** German nouns, Russian nouns, Latin verbs, Russian verbs, Spanish verbs,
     Turkish nouns, Latin adjectives.
   - **Lane 2:** Polish nouns, Italian verbs, Greek adjectives, Greek nouns, Finnish
     adjectives, English nouns, German verbs, English verbs, Latin nouns, French verbs,
     Sanskrit verbs, Latin proper nouns, Russian proper nouns, English proper nouns,
     Sanskrit numerals, Latin pronouns, Finnish pronouns.
3. **Screen each item as it lands** (20 minutes or less): the canary set plus the item's
   screening set, compared (`compare`) against the previous item's runs, from the
   headword and with the principal parts given. An exact item shows identical saved runs
   instead.
4. **Keep or drop:**
   - **exact:** kept if answers are identical and bytes or time fall;
   - **bug fix:** kept if no row loses beyond chance. A loss beyond chance is found with
     `inspect --parts --holdout`, then fixed or narrowed;
   - **experiment:** kept only if its target rows gain beyond chance and no row loses
     beyond chance by more than 0.1 points. Otherwise dropped, logged with its numbers;
   - **a correct output nobody scores** (a phrase, a cell): kept if no row loses beyond
     chance by more than 0.1, with the losing words inspected.
5. **A late fix re-screens.** A fix after an item's screen re-screens every part the
   changed code reaches, in both input modes.
6. **No full run while anything is unmeasured.** No integration sweep, tenth-9 run or
   rebuild while any item is unscreened.
7. **One integration sweep,** once every item is in: the pooled sweep against step 0's,
   on two lanes.
   - A loss there goes to a subagent that returns its cause.
   - The item is fixed or dropped, and only the languages it touched are rerun.
8. **The pre-launch checklist** before the final pipeline:
   - zero gaps in the sweep log (`grep -c ' 0 gaps'` against the parts);
   - Sanskrit, French, English and one crate per new kind of part built and tested;
   - `cargo fmt --all --check` and clippy clean;
   - a clean tree;
   - the binary copied to a versioned name (`xtask-<short hash>`), never overwritten
     while a run uses it.
9. **The final pipeline, once, on two lanes:**
   - lane 1: the twelve crate builds, longest first (Finnish first);
   - lane 2: tenth 9 for every language, then the README tenth (the crate READMEs,
     `xtask readme` or the build's own scoring).

   Tenth 9 confirms. A loss seen only there, not beyond chance or on fewer than 10 words,
   is logged, not chased, unless the words show a bug.
10. **After the pipeline,** a fix rebuilds only the crates it changes: item 1's cache
    makes a printer-only fix minutes.

**The README is written once,** by script, from the final build. The root tables come from
`~/.cache/claude-tmp/p3/roottable.py` (adapt it to item 3's columns). Every number and
claim in the prose is read from the tables or the census, with its column checked
against the header. In PR 19 two claims misread a column and needed a follow-up PR.

## 4. What must stay true

- **A crate is its grammar.** Named items: laws, stem slots, ending tables, paradigms,
  relations, heads, affixes, accent marks, phrases. Every rule carries its evidence. No
  probability tables, opaque ids or weights. Any classifier lives in the trainer only.
- **The end-to-end test** prints learned grammars, compiles them with `-D warnings` and
  compares every answer. It keeps passing and covers every new item: a vowel-kind tier,
  an adjacent consonant copy, a pooled front law, an anchored stem change, a validated
  compound split.
- **Every word has every form in every cell,** attested or not (analytic forms as
  phrases). No defectiveness; never the lemma as a stand-in. Rarity or doubt goes into
  status and reason only.
- **Answers and output.**
  - Calibrated candidates, with a status and a reason on every answer.
  - Non-test code never panics: clippy denies unwrap, expect, indexing and panic.
  - Nothing is computed before the first call.
  - Output is byte-identical for the same input, across hash seeds, toolchains and lane
    counts.
- **Nothing language-specific in the trainer or runtime.** Per-language things live in the
  configs. If `sound-law` or another of the project's crates lacks a feature, add it
  there; never work around it.
- **Code.** Idiomatic, type-driven Rust; clippy clean with `-D warnings`; README examples
  pass. Generated crates read like a grammar.
- **Trainer size is free; generated crate size matters.**
- **Tests.** Few and meaningful. No golden, snapshot or parity tests. Each test needs a
  verified failure witness: break the code, see it fail, restore.

## 5. How you work

- **No `/tmp`.** Scratch goes in `~/.cache/claude-tmp/panel4-plan/`.
- **Resource caps:**
  - **two lanes:** at most two trainer processes at once, each with at most 8 threads
    (`par::THREADS`), each under `systemd-run --user --scope -q -p MemoryMax=16G -p
    MemorySwapMax=0`;
  - Finnish and Turkish verb learns never run at the same time;
  - `cargo` with `-j 4`, tests with `--test-threads 4`;
  - generated crates compile one at a time.
- **Approved up front, never ask:**
  - sweeps (3.2 h pooled one at a time; about half on two lanes, an estimate);
  - tenth-9 runs (1.1 h one at a time; about half on two lanes, an estimate);
  - single-language learns of any length;
  - the rebuild of every crate.

  Before each long wait, tell the user what you're waiting on and for how long.
- **The run-state file:** `~/.cache/claude-tmp/panel4-plan/state.md`. It holds:
  - the binary hashes;
  - what is running, with PIDs;
  - every item's verdict so far;
  - which rows lost.

  Update it at each step; after a context compaction, read it first.
- **Watch each stage as it lands.** Use a background watcher per stage that ends when the
  stage's output exists. Read results as soon as they land; never sit idle while a
  result is on disk. No foreground sleep loops.
- **Subagents** take investigations, bisects and "why did row X lose", and return the
  cause. They also take audits, read-only checks and data probes over the forms dumps.
  They never run memory-heavy work beside your two lanes.
- **Clean up.** Kill everything you start, by PID. Never `pkill -f` or `pgrep -f | kill` a
  pattern that appears in your own command line: it kills your own shell.
- **Progress log:** dated entries in `~/code/notes/projects/flexion/grammar-crates.md`,
  under a new section "PR 21: the fourth panel's plan". Every item ends with a verdict
  line:
  - built, with numbers;
  - dropped, with numbers;
  - not built, with the reason.

  Nothing is silently skipped.
- **Commits.** Commit as the git config's identity only (gsc <stephen.korzen@gmail.com>).
  Never override it (no `-c user.*`, `GIT_AUTHOR_*`, `--author`), and pass the same rule
  to any subagent that commits. Commit everything before pushing.
- **Talking to the user.** Plain language, before/after code for API changes, and
  copy-pasteable commands starting with `cd ~/code/flexion`. Status reports say what is
  done, what is running, what is blocked, and when it will finish.

## 6. Git

One PR from a branch off the latest `main`, with this prompt as its first commit. Run fmt
and clippy before pushing. Merge with `gh pr merge --merge` once CI is green, then check
out the latest `main`. No crates.io publishing, tags or releases.

## 7. Done means

- **Every item and experiment** ends with a verdict line in the log: built, dropped or not
  built, each with its numbers or reason. Every open disagreement has its measurement
  logged.
- **No significant loss anywhere** (paired bootstrap) unless accepted under §3's rules.
- **Turkish verbs:** the shipped grammar's vowel tier holds vowels only, and item 3(e)
  shows no law differing between the shipped and README-tenth grammars in any part (or
  each difference is explained).
- **Bytes** per crate and per file kind, before and after; the parts that shipped before
  PR 19 and the new parts apart.
- **Every crate** regenerated, compiling, with README examples passing. CI green.
- **READMEs** re-scored once on the README tenth, by script, with item 3's honest rows and
  census.
- **The wall time** of every stage, against the 12–14 hour budget.

## 8. Final report

- **The PR link.**
- **A table of every item and experiment:** its bet (rows, predicted change) against the
  result, and its verdict.
- **Per language and part of speech,** before and after:
  - top-1 from the headword, typed in full, principal parts given;
  - the floor and grammar − floor;
  - the census (with item 3's kinds);
  - irregular lines;
  - calibration;
  - Greek's compatible-with-stated-length score beside the strict one.
- **Crate bytes.**
- **How each open disagreement was settled.**
- **Run times,** per stage, against the budget and against PR 19's.
- **Verification:** how you verified it, and how the user can run it, with commands
  starting `cd ~/code/flexion`.
