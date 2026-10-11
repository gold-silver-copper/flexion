# How close to the ceiling: results

Measured on 2026-10-10 with the Wiktionary dumps of 2026-10-02/03, before the
start-restoration model and the size price (the "shipped" trees below are the
models of that date). `README.md` here says how to rerun each part; raw outputs go
under `$FLEXION_CACHE/{curves,results,neural}/`.

All tree numbers are `xtask probe` numbers, scored on the held-out tenth exactly as
the reports score it (English Wiktionary's held-out words, their own entries, top-1 =
the primary form). The learning curves and section 3 use the default parameters;
section 4 also probes with the parameters each shipped build chose, which reproduces
the READMEs' held-out top-1 to within 0.3 points (default parameters are 1–11 points
lower: most for German verbs, which ship with the join model, and Latin verbs, which
ship with part models).

Noise: the held-out sets are 250–3,000 words, and a word's cells move together. A
word-level bootstrap (`ceiling compare`) gives 95% intervals for a difference between
two models of about ±1.2 points for Russian nouns (2,561 words), ±1.6 for Russian
verbs (1,251), ±2.5 for Latin nouns (1,523), ±4 for Latin verbs (604) and ±5 for
Greek verbs (254).

## Summary

| language | is the tree near its ceiling? | where the room is |
|---|---|---|
| Russian | Yes, for what the spelling's ending can tell. Nouns 84.3% from the bare lemma (shipped parameters), 93.2% with the lemma's stress given, 97.4% given the genitive and the gender/animacy markers. A transformer reading the whole word is worse (nouns −3.8, verbs −3.5 points). | Lemma stress is ~9 of the ~16 missing points on nouns: the best ending predictor gets it 84% right, the tree 87%; the word's beginning could add up to ~4 (either predictor right: 91%). Animacy (~2 points given citation): the ending tops out at 83.5%, the tree is there. More data still adds ~1.7 (nouns) and ~2.7 (verbs) points per doubling. Label noise ~0.5–0.8% per cell. |
| Latin | No, for nouns. 59.1% from the bare lemma, 98% given principal parts. | Vowel length inside the stem. The ending gets the lemma's length marks 59% right (nouns), the whole spelling 71%, and a transformer is **+10.6 points** on noun forms (prefixes `dē-`, `cōn-`; vowels before `ns`). Verbs: the shipped tree's part models already match the transformer (−2.6, within noise). More data: +2 (nouns) to +4.4 (verbs) per doubling. |
| Ancient Greek | No, for nouns. 42.6% from the bare lemma. | Short-vowel marks inside the stem: transformer **+20.4** on nouns; the whole spelling gets the lemma's length 65% right vs 48% from the ending. Verbs tie once an extraction artifact (bare `εἶμεν` for the perfect optative middle, 2.4% of cells) is set aside; the tree can't generalize perfect reduplication (8% right) or the augment after a preverb, the transformer can (38%). Curves still rising (+2 to +5 per doubling). |
| German | Yes. Nouns 94.9%, verbs 93.9% (shipped); the transformer is worse (−8.4, −2.0). | Data, slowly (+1 nouns, +1.2–1.5 verbs per doubling); label noise 0.15–0.75%. |
| Polish, Sanskrit | Probably (no neural run). | Nouns and verbs +0.5–1.4 per doubling; adjectives saturated. |
| English, French, Spanish, Italian | Yes: curves flat (≤0.3 points per doubling on nouns and adjectives). | Verbs +0.4–0.8 per doubling; Spanish verbs are within ~0.6 of their label noise. |

## 1. Learning curves

`tools/ceiling/curves.sh`: `xtask probe <lang> --fraction f`. Each cell: held-out top-1
% (log-loss, nats per cell). *Train lemmas* counts the training words at 100% (train +
dev tenths, every edition). *Last doubling* = top-1 at 100% minus at 50%.

| language | pos | train lemmas | mode | 10% | 25% | 50% | 100% | last doubling |
|---|---|---:|---|---:|---:|---:|---:|---:|
| russian | noun | 97822 | Lemma | 78.23 (0.821) | 79.88 (0.721) | 81.50 (0.667) | 83.20 (0.611) | +1.70 |
| russian | noun | | StressedLemma | 90.76 (0.439) | 91.60 (0.379) | 92.17 (0.351) | 93.03 (0.313) | +0.86 |
| russian | noun | | Citation | 93.61 (0.332) | 93.97 (0.286) | 94.49 (0.266) | 95.09 (0.227) | +0.60 |
| russian | adjective | 60792 | Lemma | 83.71 (0.462) | 88.02 (0.381) | 91.84 (0.287) | 96.09 (0.159) | +4.25 ¹ |
| russian | adjective | | StressedLemma | 99.74 (0.031) | 99.73 (0.029) | 99.86 (0.025) | 99.78 (0.026) | −0.08 |
| russian | verb | 42115 | Lemma | 74.88 (0.866) | 82.20 (0.667) | 86.41 (0.538) | 89.06 (0.450) | +2.65 |
| russian | verb | | StressedLemma | 90.50 (0.424) | 93.41 (0.305) | 94.86 (0.229) | 96.07 (0.172) | +1.21 |
| russian | verb | | Citation | 96.65 (0.262) | 97.87 (0.154) | 98.59 (0.101) | 98.83 (0.090) | +0.24 |
| latin | noun | 14370 | Lemma | 50.19 (4.073) | 53.45 (3.674) | 54.91 (3.275) | 56.87 (3.026) | +1.96 |
| latin | noun | | StressedLemma | 88.42 (0.725) | 88.65 (0.691) | 87.88 (0.690) | 87.10 (0.722) | −0.78 ² |
| latin | noun | | Citation | 97.79 (0.158) | 97.85 (0.144) | 97.98 (0.143) | 98.11 (0.127) | +0.13 |
| latin | adjective | 24695 | Lemma | 61.05 (3.342) | 67.39 (2.698) | 72.70 (2.177) | 80.90 (1.484) | +8.20 ¹ |
| latin | adjective | | StressedLemma | 96.38 (0.393) | 95.38 (0.458) | 95.07 (0.447) | 94.87 (0.435) | −0.20 ² |
| latin | verb | 5567 | Lemma | 45.41 (3.789) | 49.18 (3.299) | 53.94 (2.946) | 58.34 (2.601) | +4.40 |
| latin | verb | | StressedLemma | 70.39 (1.222) | 71.42 (1.109) | 77.03 (1.040) | 80.46 (0.976) | +3.43 |
| latin | verb | | Citation | 95.92 (0.382) | 98.18 (0.173) | 98.83 (0.120) | 99.09 (0.094) | +0.26 |
| ancient-greek | noun | 7358 | Lemma | 31.19 (5.974) | 35.72 (5.562) | 39.55 (5.287) | 40.13 (4.875) | +0.58 |
| ancient-greek | noun | | StressedLemma | 82.27 (1.305) | 83.68 (1.235) | 85.20 (1.161) | 83.12 (1.177) | −2.08 ² |
| ancient-greek | noun | | Citation | 92.49 (0.623) | 93.95 (0.492) | 94.86 (0.410) | 95.90 (0.321) | +1.04 |
| ancient-greek | adjective | 4228 | Lemma | 35.25 (5.941) | 37.38 (5.605) | 40.20 (5.350) | 44.36 (4.826) | +4.16 ¹ |
| ancient-greek | adjective | | StressedLemma | 87.06 (1.035) | 88.89 (0.809) | 89.59 (0.766) | 89.31 (0.818) | −0.28 |
| ancient-greek | adjective | | Citation | 95.66 (0.385) | 97.92 (0.177) | 98.00 (0.164) | 98.40 (0.134) | +0.40 |
| ancient-greek | verb | 2435 | Lemma | 25.06 (6.759) | 28.05 (6.468) | 30.68 (6.032) | 32.85 (5.649) | +2.17 |
| ancient-greek | verb | | StressedLemma | 45.93 (4.944) | 49.31 (4.495) | 51.58 (4.207) | 53.90 (3.886) | +2.32 |
| ancient-greek | verb | | Citation | 39.41 (5.582) | 47.27 (4.772) | 49.97 (4.490) | 54.93 (4.050) | +4.96 |
| german | noun | 129288 | Lemma | 90.59 (0.363) | 92.17 (0.290) | 93.23 (0.252) | 94.19 (0.227) | +0.96 |
| german | noun | | Citation | 97.46 (0.119) | 98.05 (0.090) | 98.25 (0.083) | 98.41 (0.076) | +0.16 |
| german | adjective | 13361 | Lemma | 98.00 (0.110) | 98.16 (0.086) | 98.59 (0.063) | 98.86 (0.049) | +0.27 |
| german | verb | 8383 | Lemma | 76.44 (1.380) | 79.77 (1.086) | 81.45 (0.867) | 82.69 (0.682) | +1.24 |
| german | verb | | Citation | 90.37 (0.847) | 94.16 (0.535) | 96.79 (0.311) | 98.26 (0.183) | +1.47 |
| polish | noun | 33752 | Lemma | 91.54 (0.307) | 92.96 (0.239) | 93.23 (0.218) | 93.92 (0.200) | +0.69 |
| polish | noun | | Citation | 94.80 (0.210) | 95.71 (0.163) | 95.92 (0.147) | 96.32 (0.131) | +0.40 |
| polish | adjective | 11858 | Lemma | 99.85 (0.008) | 99.95 (0.003) | 99.97 (0.001) | 99.97 (0.001) | +0.00 |
| polish | verb | 10882 | Lemma | 94.69 (0.261) | 95.52 (0.202) | 96.39 (0.169) | 96.86 (0.134) | +0.47 |
| polish | verb | | Citation | 96.89 (0.202) | 97.73 (0.138) | 97.92 (0.122) | 98.28 (0.092) | +0.36 |
| sanskrit | noun | 4509 | Lemma | 89.44 (0.459) | 90.82 (0.331) | 91.82 (0.274) | 92.29 (0.254) | +0.47 |
| sanskrit | adjective | 1778 | Lemma | 85.26 (0.503) | 89.57 (0.366) | 90.54 (0.272) | 90.98 (0.264) | +0.44 |
| sanskrit | verb | 927 | Lemma | 82.19 (1.525) | 85.54 (1.018) | 87.86 (0.960) | 89.23 (0.827) | +1.37 |
| english | noun | 205414 | Lemma | 96.58 (0.138) | 96.78 (0.122) | 96.89 (0.115) | 97.17 (0.110) | +0.28 |
| english | adjective | 55292 | Lemma | 95.31 (0.133) | 95.36 (0.127) | 95.46 (0.118) | 95.47 (0.116) | +0.01 |
| english | verb | 34275 | Lemma | 93.59 (0.204) | 95.23 (0.147) | 96.19 (0.121) | 96.98 (0.101) | +0.79 |
| english | verb | | Citation | 99.33 (0.035) | 99.49 (0.031) | 99.53 (0.027) | 99.57 (0.024) | +0.04 |
| french | noun | 38867 | Lemma | 97.40 (0.172) | 97.47 (0.160) | 97.49 (0.158) | 97.51 (0.152) | +0.02 |
| french | adjective | 16326 | Lemma | 96.34 (0.183) | 97.01 (0.143) | 97.39 (0.130) | 97.66 (0.102) | +0.27 |
| french | verb | 6557 | Lemma | 96.00 (0.203) | 97.22 (0.131) | 98.15 (0.076) | 98.92 (0.052) | +0.77 |
| spanish | noun | 54316 | Lemma | 98.05 (0.078) | 98.25 (0.072) | 98.49 (0.065) | 98.61 (0.061) | +0.12 |
| spanish | adjective | 22154 | Lemma | 98.29 (0.098) | 98.53 (0.085) | 98.53 (0.075) | 98.54 (0.073) | +0.01 |
| spanish | verb | 16148 | Lemma | 97.38 (0.125) | 97.82 (0.082) | 98.75 (0.060) | 99.27 (0.029) | +0.52 |
| italian | noun | 55489 | Lemma | 94.84 (0.235) | 95.40 (0.210) | 95.38 (0.199) | 95.94 (0.186) | +0.56 |
| italian | adjective | 23668 | Lemma | 98.10 (0.090) | 98.15 (0.074) | 98.16 (0.073) | 98.48 (0.066) | +0.32 |
| italian | verb | 11691 | Lemma | 97.38 (0.168) | 98.12 (0.104) | 98.42 (0.074) | 98.79 (0.059) | +0.37 |

(The full table, with every Citation row, is `python3 tools/ceiling/curves_table.py`.)

¹ Russian, Latin and Greek adjectives read participles from the verbs' tables (the
lexicon): with fewer training verbs, fewer held-out adjectives are found there. Their
steep curves measure lexicon coverage, not generalization.
² Non-monotone: with the lemma written in full, Latin nouns and adjectives and Greek
nouns get slightly *worse* with more data (−0.8, −1.5, −2.1 points from the best
share). Default parameters keep more structure as data grows; this is the kind of
over-structure an MDL objective should fix, not a data limit.

What the curves say:

- **Flattened** (≤0.3 points per doubling, within noise): English nouns and adjectives,
  French nouns, Spanish nouns and adjectives, Polish adjectives, Russian adjectives in
  full, and given principal parts Latin nouns and adjectives, German nouns and most
  Romance cells. More data won't move these.
- **Still rising, ~0.5–1 point per doubling**: German nouns, Polish nouns and verbs,
  Sanskrit, Italian, English, French and Spanish verbs. Log-loss falls faster than
  top-1 everywhere, so confidences keep improving after top-1 stalls.
- **Still rising clearly, 1.5–5 points per doubling**: Russian nouns (+1.7) and verbs
  (+2.65) from the bare lemma, Latin nouns (+2.0) and verbs (+4.4), Greek verbs (+2.2
  bare, +5.0 with principal parts), German verbs (+1.2 bare, +1.5 with principal parts).
  In Russian most of the gain is in the lemma's stress: with the stress given the
  slope halves (+0.9 nouns, +1.2 verbs) and with principal parts it is small (+0.6,
  +0.24). Russian's second edition is already in the 100% point (98k noun lemmas vs
  26k from the English Wiktionary alone).

## 2. Noise floor

`ceiling noise <lang>`: on words both editions have, per cell, how often the English
Wiktionary's primary form and the other edition's differ (`same_form`, flexion's
equality), split into marks only (stress, ё) and letters, and how often neither edition
lists the other's form at all (a real conflict; otherwise both list both and only the
order differs).

| language, pos | words in both | cells compared | differ | marks only | letters | neither lists the other's | lemma (citation form) differs |
|---|---:|---:|---:|---:|---:|---:|---:|
| Russian noun | 22,987 | 266,441 | 1.60% | 0.89% | 0.71% | 0.72% | 0.72% |
| Russian adjective | 22,461 | 1,104,301 | 0.89% | 0.62% | 0.27% | 0.26% | 0.77% |
| Russian verb | 12,134 | 198,604 | 1.30% | 0.79% | 0.52% | 0.41% | 0.58% |
| German noun | 36,736 | 274,173 | 1.48% | 0.00% | 1.48% | 0.28% | 0.00% |
| Spanish verb | 7,339 | 534,213 | 0.16% | 0.00% | 0.16% | 0.12% | 0.00% |

Cells that differ most: Russian `noun.acc.pl` 3.9% and `noun.acc.sg` 2.3% (animacy
disagreements), short adjectives 3.9% (5.5% plural), `participle.past.pass` 3.1%;
German `noun.gen.sg` 5.1% (`-s`/`-es` order); Spanish imperatives 0.4–0.7%.

If each edition were wrong independently at rate e, they would differ at about 2e, so
**label noise per cell ≈ half the "differ" rate**: Russian 0.45–0.8% (0.15–0.35% if only
real conflicts count), German nouns 0.15–0.75%, Spanish verbs 0.06–0.08%. That is a
ceiling of roughly 99–99.5% top-1 even for a perfect model, and it matters only where
the models already stand at 98–99.5% (Spanish, Italian, French verbs; every language
given principal parts). For Russian, Latin and Greek from the bare lemma, label noise is
a small part of the 10–60 point gap.

## 3. The irreducible part

`ceiling ending <lang> <pos> <target>`: the best predictor that sees only the lemma's
last k letters (majority label among training words with that ending, backing off to
shorter ones; scored on the held-out words), and the same from the first k letters
(evidence from the root or prefix). The label is the whole lemma's pattern: Russian,
which letter carries the stress; Latin and Greek, which letters carry a macron or breve;
or the inherent feature's value. *Either right* is the share where the ending or the
beginning predictor is right: an upper bound for combining the two. Full tables per k:
`$FLEXION_CACHE/results/ending.md`.

| | ending only: k=1 / 3 / 6 / 12 letters | best (k) | log-loss at best k (bits) | beginning only, best | either right | **the tree** |
|---|---|---:|---:|---:|---:|---:|
| Russian noun, lemma stress | 47.1 / 71.7 / 82.3 / 84.3 | 84.3 (12) | 0.89 | 51.6 | 91.2 | **87.0** |
| Russian noun, animacy | 75.0 / 80.9 / 83.0 / 83.6 | 83.6 (9) | 0.54 | 73.4 | 94.0 | **83.5** |
| Russian noun, gender | 94.5 / 97.1 / 96.6 / 96.4 | 97.1 (3) | 0.15 | 50.6 | 98.6 | **97.2** |
| Russian adjective, lemma stress | 43.4 / 49.2 / 73.4 / 89.8 | 89.8 (12) | 0.60 | 65.9 | 96.0 | **96.0** ¹ |
| Russian verb, lemma stress | 61.4 / 55.8 / 80.8 / 93.5 | 93.5 (12) | 0.43 | 74.3 | 98.2 | **92.3** |
| Russian verb, aspect | 45.1 / 71.6 / 78.3 / 72.6 | 79.2 (5) | 0.72 | 80.9 | 96.9 | **78.1** |
| Latin noun, lemma length | 39.9 / 51.9 / 59.2 / 59.1 | 59.2 (6) | 2.92 | 40.8 | 73.2 | **60.2** |
| Latin adjective, lemma length | 24.4 / 50.6 / 64.3 / 71.4 | 71.5 (11) | 2.58 | 26.5 | 77.9 | **81.1** ¹ |
| Latin verb, lemma length | 58.6 / 65.7 / 73.8 / 74.7 | 74.7 (7) | 2.05 | 18.5 | 77.3 | **69.2** |
| Greek noun, lemma length | 28.5 / 37.7 / 47.2 / 48.0 | 48.0 (8) | 6.43 | 32.3 | 59.2 | **44.5** |
| Greek adjective, lemma length | 28.9 / 31.4 / 41.8 / 42.3 | 42.3 (9) | 6.24 | 38.0 | 60.1 | **46.7** ¹ |
| Greek verb, lemma length | 40.9 / 49.6 / 55.5 / 55.9 | 56.3 (7) | 3.98 | 43.3 | 69.3 | **57.1** |

¹ The tree also reads participles from the verbs' tables (the lexicon), which an
ending predictor can't.

Where the length or stress goes wrong, by position (best ending predictor vs tree):
the last three letters are right for 90–100% of words everywhere; the misses are in the
stem. Latin nouns: right in the last three letters 94% (tree 94%), before them 64%
(tree 65%). Greek nouns: 89% / 54% (tree 90% / 50%). Russian noun stress: 90% / 87%
(tree 91% / 88%).

**The whole spelling, not just the ending.** The same neural model as in section 4,
trained to write the lemma in full from the lemma as typed (`seq2seq.py --target
lemma`, one row per word, 1500 steps, ~3–5 minutes each), shows how much of the lemma's
vowel length the *whole* spelling predicts (*length right*: every macron and breve in
place):

| | train words | ending only (best k) | tree | neural, whole word: length right | (all letters right too) |
|---|---:|---:|---:|---:|---:|
| Latin noun | 12,833 | 59.2 | 60.2 | **71.4** | 71.3 |
| Latin adjective | 21,949 | 71.5 | 81.1 | **86.4** | 86.4 |
| Latin verb | 4,996 | 74.7 | 69.2 | **77.5** | 76.8 |
| Greek noun | 6,559 | 48.0 | 44.5 | **64.6** | 64.4 |
| Greek adjective | 3,736 | 42.3 | 46.7 | **56.5** | 54.8 |
| Greek verb | 2,175 | 56.3 | 57.1 | 48.0 | 45.7 |

The neural restorer also garbles letters of some words (Greek: only 59–86% of lemmas
come out letter-perfect), so these are lower bounds. With enough words, reading the
whole lemma beats the best ending predictor by 12 points (Latin nouns) and 16 (Greek
nouns); Greek verbs (2,175 training words) are too few for it.

What this says:

- **Russian lemma stress and animacy:** the ending saturates by 8–10 letters at 84%
  (stress) and 83.5% (animacy); the tree is at or above that (87%, 83.5%). The tree
  uses the ending about as well as it can be used. What remains needs evidence the
  ending doesn't carry: the word's beginning alone gets stress right 52% of the time
  but disagrees usefully with the ending (either right 91%; animacy 94%), so related
  words sharing a root or prefix (section 4's "evidence beyond the ending") could add
  a few points of stress and up to ~10 of animacy at best. Gender is solved by the last
  2–3 letters (97%).
- **Russian verb stress** from the ending (93.5%) is a bit above the tree (92.3%) and
  the beginning helps a lot (either 98%): prefixes (`вы́-` in perfectives) decide much of
  it. Aspect: the ending tops out at 79%, the beginning (prefix) at 81%, together 97%.
- **Latin and Greek vowel length:** the ending decides the last syllables almost
  perfectly but the stem's lengths only 50–65% of the time. The ending saturates at
  59% (Latin nouns), 75% (Latin verbs), 48% (Greek nouns), 56% (Greek verbs); the tree
  is at that level. The whole spelling carries more: 71% (Latin nouns), 86% (Latin
  adjectives), 65% (Greek nouns). So roughly a third of the stem-length errors are
  predictable from the word's other letters (prefixes such as `dē-`, `cōn-`, `prō-`;
  a vowel before `ns`/`nf`; Greek stems), and the rest is information the spelling
  doesn't carry at all (a word's own vowel lengths, which only a dictionary knows).

## 4. Neural baseline

`tools/ceiling/neural/` (README there): a character-level transformer encoder-decoder
(input: cell tags + lemma characters; output: the form's characters with every mark;
NFD text), d_model 256, 3+3 layers, 4 heads, 5.6M parameters, batch 256, AdamW lr
2e-3 with warm-up and cosine decay, bf16 autocast, no dropout, seed 1, 8 threads,
trained on the export's train split (every edition), the checkpoint chosen on 10,000
dev cells, scored greedily on the same held-out cells as the tree with flexion's
`same_form` (`ceiling compare`). All training pairs are used, in shuffled
length-sorted batches; the step budget is what fits in ~10 CPU minutes:

| | training pairs | steps (epochs) | training time | best dev step |
|---|---:|---:|---:|---:|
| Russian noun | 944,709 | 3000 (0.81) | 9.8 min | 3000 |
| Russian verb | 574,731 | 2400 (1.07) | 8.4 min | 2400 |
| Latin noun | 138,677 | 3000 (5.5) | 9.4 min | 3000 |
| Latin verb | 478,320 | 2400 (1.28) | 8.9 min | 2400 |
| Greek noun | 88,153 | 3000 (8.7) | 9.8 min | 2400 |
| Greek verb | 530,335 | 1800 (0.87) | 6.8 min | 1800 |
| German noun | 737,466 | 2200 (0.76) | 8.6 min | 2100 |
| German verb | 387,936 | 3000 (1.98) | 10.0 min | 3000 |

Every run but Greek and German nouns was still improving on dev when it stopped, so
these are lower bounds for this kind of model. Reproducibility: the container path
(`run.sh`, Docker, `--cpus 8`) and the local path (`run.sh --local`, the same pinned
torch 2.14.1+cpu) gave byte-identical predictions and metrics for the same seed on a
300-step Russian-nouns run. (`seq2seq.py`'s own top-1 in `metrics.json` is up to 0.7
points stricter than flexion's `same_form` on Russian, which ignores a second stress
mark in hyphenated compounds; the tables use `ceiling compare`, i.e. flexion's rule.)

The tree is shown two ways: *probe* (default parameters, the curves' setting) and
*shipped* (`tools/ceiling/tuned.sh`: the same probe with the parameters the shipped
build chose: smoothing, pruning, syllable walk, joins, part models; it reproduces the
README's held-out top-1 to within 0.3 points). The difference and its interval are
against the shipped tree.

| | held-out words / cells | tree, probe | tree, shipped | neural | neural − shipped tree (95% interval, words resampled) | either right | letters right: tree / neural |
|---|---:|---:|---:|---:|---:|---:|---:|
| Russian noun | 2561 / 27124 | 83.20 | 84.25 | 80.43 | −3.82 (−5.03 … −2.53) | 89.8 | 95.0 / 92.9 |
| Russian verb | 1251 / 19729 | 89.06 | 91.89 | 88.38 | −3.51 (−4.96 … −2.02) | 96.5 | 97.1 / 95.0 |
| Latin noun | 1523 / 16448 | 56.87 | 59.08 | 69.66 | **+10.57** (+7.91 … +13.17) | 81.0 | 93.3 / 89.7 |
| Latin verb | 604 / 58798 | 58.34 | 66.55 | 63.99 | −2.56 (−7.06 … +1.30) | 81.6 | 90.3 / 73.1 |
| Greek noun | 807 / 10792 | 40.13 | 42.61 | 62.98 | **+20.38** (+16.84 … +24.17) | 72.0 | 84.1 / 79.8 |
| Greek verb | 254 / 64665 | 32.85 | 35.81 | 37.97 | +2.16 (−2.52 … +6.84) | 55.3 | 56.6 / 46.4 |
| German noun | 6239 / 39525 | 94.19 | 94.92 | 86.48 | −8.44 (−9.11 … −7.82) | 96.5 | – |
| German verb | 940 / 49015 | 82.69 | 93.89 | 91.85 | −2.04 (−3.35 … −0.72) | 97.8 | – |

Log-loss of the primary form (nats per cell), probe tree / neural: Russian noun 0.61 /
0.69, Russian verb 0.45 / 0.36, Latin noun 3.03 / 1.35, Latin verb 2.60 / 1.87, Greek
noun 4.88 / 1.80, Greek verb 5.65 / 3.56, German noun 0.23 / 0.47, German verb 0.68 /
0.36. (Shipped-parameter trees: 0.59, 0.36, 2.94, 2.37, 4.92, 5.73, 0.22, 0.37.)

Where they differ (`$FLEXION_CACHE/results/compare-*.md`, `*-tuned.md`
against the shipped tree):

- **Russian nouns** (tree better by 3.8): the neural model wins on some root-driven
  stress (`абью́зерша`, `календари́`, `като́ды`) but loses on long words (5+ syllables
  −7.8: it garbles long stems), on animacy-driven accusatives (acc.pl −6.3) and on most
  endings. Against the default-parameter tree it was better on fleeting vowels (`-ок`
  +17.8); the shipped tree's syllable walk already closes that (−4.3).
- **Russian verbs** (tree better by 3.5, every group): the neural model's wins are
  isolated words with a stressed prefix (`вы́двинусь`, `вы́качусь`), which the tree
  can't see from the end; it loses most on one-syllable stems (−14.6).
- **Latin nouns** (neural +10.6): the tree has the letters right more often (93.3% vs
  89.7%); the neural model has the **macrons** right far more often, on prefixes and
  stems the ending can't show (`cōnsessōrēs`, `dēceptiōnēs`, `crētae`, `cūriōsum`;
  `-ix` +57 points, `-or` +20, `-us` +16).
- **Latin verbs** (tree better by 2.6, within noise): the shipped tree's part models
  bring it from 58.3% to 66.6%; it has far more letters right (90.3% vs 73.1%), the
  neural model more marks right (`dēfundimus`, `dīlūcēscimus`, `cōnsuādēmus`): either
  right 81.6%.
- **Greek nouns** (neural +20.4): the tree's letters are better (84.1% vs 79.8%); the
  neural model writes the stem's short-vowel marks the tables print (`βῐβλῐ́ω`,
  `ᾰ̓δελφᾱ́`, `γῠμνᾰσταί`) where the tree leaves them unmarked; it wins on every common
  ending.
- **Greek verbs** (neural +2.2, within noise): 1,520 held-out cells (2.4%) are perfect
  optative middle/passive forms whose primary form is the bare auxiliary (`εἶμεν`,
  `εἶτε`, `εἶεν`…; Wiktionary prints the form periphrastically and the extractor keeps
  only the auxiliary). The neural model gets all of them, the tree none (it writes
  `κελεεἶμεν`). Without them the two tie (tree 36.7%, neural 36.5%). Underneath, they
  are right on different cells: the tree on the present (+10.5) and on letters overall
  (57% vs 46%); the neural model on the perfect (tree 8.1%, neural 38.1%:
  reduplication copies the first consonant, which a tree anchored at the end can't
  generalize), the imperfect (+10.7: the augment goes after a preverb, `ἐξετελοῦμεν`,
  `ἐπεστρᾰτεύομεν`, where the tree writes `ἐἐκτελοῦμεν`) and the pluperfect.
- **German nouns** (tree better by 8.4): long compounds, whose last part decides the
  plural, are what a tree reading from the end is made for; the neural model garbles
  them (`ausführungsverzögerun`) and loses on every cell.
- **German verbs** (tree better by 2.0): the default-parameter tree (no joins) was
  9.2 points *behind*, all of it in main-clause forms of separable verbs (`biege ab`;
  tree 69%, neural 89%). The shipped tree's join model splits separable prefixes and
  reaches 90.4% there; the neural model now trails in every group.

What the neural baseline says about what the tree leaves out:

- **Russian and German**: nothing systematic. The shipped tree beats a transformer
  that reads the whole word, in every group. The remaining Russian errors are lemma
  stress and animacy that the spelling barely carries (section 3); the two models are
  wrong on different words (either right: 89.8% nouns, 96.5% verbs), and the neural
  model's wins are words whose stress follows a prefix or root, so a second context
  read from the word's beginning is where a gain could come from.
- **Latin and Greek nouns**: the tree's letters are better, but it misses vowel length
  (and Greek short-vowel marks) inside the stem, which the whole spelling predicts much
  better than the ending (section 3: 71% vs 59% for Latin nouns, 65% vs 48% for Greek
  nouns). A lemma-length restorer that reads the whole word, with the tree's endings
  for the rest, could gain roughly 10–20 points on nouns from the bare lemma.
- **Greek verbs**: perfect reduplication and augment placement are operations at the
  word's beginning that the tree can't generalize; and the `εἶμεν` cells are an
  extraction artifact worth fixing (2.4% of held-out verb cells).

## Since then

The Greek extraction artifact is fixed: periphrastic optatives no longer keep the bare
auxiliary. The start-restoration model (a restoration tree read from the word's start)
took most of the Latin and Greek noun gap: from the bare lemma, Latin nouns 59.1% →
68.9% and Greek nouns 42.6% → 57.8%, against the transformer's 69.7% and 63.0%.

Greek verbs learn sound laws (`crates/flexion-train/src/phonology.rs`), reduplication
among them, with the aspirate copied plain as the transformer had found it must be: the
perfect goes from 26.8% to 38.6% of held-out cells right (the transformer: 38.1%), the
pluperfect from 21.0% to 32.9%, all verb cells from 52.4% to 54.6%.

Two more rule shapes were tried and dropped. One rewrites the ending's first letter
after some of the stem's last letters (Russian `ы → и` after `к г х ж ш щ ч`). The
other reads a stem letter the edit keeps (`lipa → lipie` as `p → pi`), which learns
Polish's second palatalization whole. Each rule had to merge classes to stay. They
merged few: Russian nouns 3,762 → 3,706 classes and Polish nouns 1,373 → 1,370, with
the tables no smaller and training 3.6 times slower. These languages' classes are told
apart by stress pattern and gender far more than by these spellings.
