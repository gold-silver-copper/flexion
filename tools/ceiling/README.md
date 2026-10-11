# ceiling

Measurements of how close flexion's models are to the best accuracy their data
allows. Nothing here is a dependency of any crate; it reads the lexeme cache and
the probe models `xtask` writes.

Use a cache of its own, so a trainer change elsewhere can't mix in:

```sh
export FLEXION_CACHE=$HOME/.cache/flexion-ceiling
```

Every script runs from the repository root. Logs and tables go under
`$FLEXION_CACHE`.

## 0. Data

Extract every language (with the other editions the shipped models read) and
export the evaluation splits as TSV (`$FLEXION_CACHE/export/<lang>/<pos>.tsv`).
About two minutes for all ten.

```sh
cd ~/code/flexion && export FLEXION_CACHE=$HOME/.cache/flexion-ceiling \
  && cargo build --release -p xtask -p ceiling -j 4 \
  && tools/ceiling/extract.sh            # or: tools/ceiling/extract.sh russian latin
```

## 1. Learning curves

`xtask probe --fraction f` for f = 1, 0.5, 0.25, 0.1 (default parameters, scored
on the held-out tenth); Ancient Greek one part of speech at a time. About 12
minutes for all ten, one probe at a time.

```sh
cd ~/code/flexion && export FLEXION_CACHE=$HOME/.cache/flexion-ceiling \
  && tools/ceiling/curves.sh \
  && python3 tools/ceiling/curves_table.py $FLEXION_CACHE/curves $FLEXION_CACHE/export
```

The table gives top-1 (log-loss in nats per cell) per training share and input
mode, and the gain of the last doubling.

## 2. Noise floor

Where the English Wiktionary and another edition both have a word (Russian,
German, Spanish), how often their primary forms of a cell differ: in marks only
(stress, length) or in letters, and how often neither edition lists the other's
form at all. Seconds.

```sh
cd ~/code/flexion && export FLEXION_CACHE=$HOME/.cache/flexion-ceiling \
  && for l in russian german spanish; do target/release/ceiling noise $l; done
```

## 3. The irreducible part

How well a lemma-level choice can be predicted from spelling alone: the
majority label among training words sharing the lemma's last k letters (backing
off to shorter endings), scored on the held-out tenth, for k = 0…12; the same
from the first k letters (evidence from the root); and how often either is
right (an upper bound for combining the two). Next to it, what the tree gets
from the bare lemma. Targets: Russian lemma stress, noun animacy and gender,
verb aspect; Latin and Ancient Greek vowel length of the lemma (macrons and
breves, every vowel). Seconds.

```sh
cd ~/code/flexion && export FLEXION_CACHE=$HOME/.cache/flexion-ceiling \
  && tools/ceiling/ending.sh
```

`target/release/ceiling ending <lang> <pos> <stress|length|<inherent feature>>`
runs one.

## 4. Neural baseline

A character-level transformer (the SIGMORPHON shared-task kind) trained on the
export's train split, checkpoint chosen on dev, scored on the same held-out
words with the same rule; see [`neural/README.md`](neural/README.md). About ten
minutes per part of speech.

The tree to compare it with is trained by `ceiling compare` itself, with the
probe flags after its arguments: none for default parameters, or the parameters
the shipped build chose (smoothing, pruning, syllable walk, joins, part models;
`tuned.sh` lists them and probes each). `ceiling compare` puts the two side by
side by cell, lemma ending, length and inherent feature, with a word-level
bootstrap interval for the difference:

```sh
cd ~/code/flexion && export FLEXION_CACHE=$HOME/.cache/flexion-ceiling \
  && tools/ceiling/neural/run.sh russian noun \
  && target/release/ceiling compare russian noun $FLEXION_CACHE/neural/russian-noun/predictions.tsv \
       --alpha 2 --tau 1 --support 1 --syllables 2 --parts 1
```

How much of a lemma's vowel length the *whole* spelling predicts (next to step
3's ending-only predictor): the same model trained to write the lemma in full,
one row per word, a few minutes each:

```sh
cd ~/code/flexion && export FLEXION_CACHE=$HOME/.cache/flexion-ceiling \
  && tools/ceiling/neural/run.sh latin noun --target lemma --steps 1500 --eval-every 150
```
