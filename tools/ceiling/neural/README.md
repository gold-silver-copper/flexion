# Neural baseline

A measuring stick for flexion's suffix tree: a small character-level
transformer encoder-decoder, the standard model of the SIGMORPHON inflection
shared tasks. It is not shipped and nothing depends on it.

- **Input:** the cell's feature tags (`[noun] [gen] [pl]`), then the lemma's
  characters as typed (no stress or length marks it doesn't type).
- **Output:** the form's characters, stress and length marks included (text is
  NFD-normalized, so each mark is its own symbol).
- **Data:** `xtask export`'s split. It trains on `train` (every edition the
  model reads), picks the checkpoint with the best exact-match accuracy on a
  fixed sample of `dev`, and scores `test`: the English Wiktionary's held-out
  words on their own entries, exactly the cells the reports score.
- **Score:** top-1 is right when the greedy answer is the primary form (case
  folded, a grave read as an acute, as flexion compares forms); *letters*
  ignores every mark; *loss* is the mean log-loss of the primary form in nats
  per cell, its probability floored at 1e-4 (the probe's `loss`).
- **Defaults:** d_model 256, 3+3 layers, 4 heads, feed-forward 1024 (5.6M
  parameters), batch 256 (batches of similar length), 3000 steps (AdamW, lr
  2e-3, 300 warm-up steps, cosine decay; no dropout or label smoothing, which
  cost more CPU time than they gave back in ten minutes), bfloat16 autocast
  for training, seed 1, 8 threads. A run stops early, and says so, past
  `--max-minutes` (10); a run cut short that way depends on the machine's
  load, so pass a `--steps` that fits instead. Every option is a flag of
  `seq2seq.py`.

## Data

```sh
cd ~/code/flexion && export FLEXION_CACHE=$HOME/.cache/flexion-ceiling \
  && cargo xtask extract russian --dump ~/.cache/flexion/dumps/Russian.jsonl \
       --edition ru=$HOME/.cache/kaikki/ru/Русский.jsonl \
  && cargo xtask export russian
```

This writes `$FLEXION_CACHE/export/russian/<pos>.tsv`. `tools/ceiling/extract.sh`
does it for every language, with the editions the shipped models read.

## Run

In a container (CPU only, `--cpus 8`, no network, a read-only image with an
in-memory `/tmp`, the export mounted read-only, writing only to the output
directory; the image is rebuilt, from cache, whenever this directory changes):

```sh
cd ~/code/flexion && export FLEXION_CACHE=$HOME/.cache/flexion-ceiling \
  && tools/ceiling/neural/run.sh russian noun
```

If the shell predates the user's `docker` group membership, the script runs
the Docker commands through `newgrp docker`.

Without Docker, the same script with a Python that has `requirements.txt`
installed (`python -m pip install -r tools/ceiling/neural/requirements.txt`):

```sh
cd ~/code/flexion && export FLEXION_CACHE=$HOME/.cache/flexion-ceiling \
  && PYTHON=$HOME/.cache/nb-venv/bin/python tools/ceiling/neural/run.sh --local russian noun
```

Options after the part of speech go to `seq2seq.py`, e.g. `--steps 600` for a
short run, or `--target lemma` to learn the lemma written in full from the
lemma as typed (one row per word: how much of vowel length the whole spelling
predicts; output in `<lang>-<pos>-lemma/`). Both paths run the identical script with the pinned torch and give
the same numbers for the same seed on the same machine.

**Runtime:** 7–10 minutes of training (3000 steps at 0.15–0.2 s on 8 threads
of a Ryzen AI Max+ 395 shared with other work; Ancient Greek verbs, with longer
forms, 1800 steps in 7 minutes), plus 1–5 minutes of greedy decoding of the
test cells (Ancient Greek verbs, 65k cells, take longest).

**Results** go to `$OUT/<lang>-<pos>/` (default `OUT=$FLEXION_CACHE/neural`):
`metrics.json` (top-1, letters, loss, per-cell counts, the options),
`predictions.tsv` (`lemma, cell, primary, answer, probability of the primary`)
and `log.txt`.

To put the answers next to the tree's on the same cells (by cell, lemma
ending, length and inherent feature), with the probe model from
`tools/ceiling/curves.sh`:

```sh
cd ~/code/flexion && export FLEXION_CACHE=$HOME/.cache/flexion-ceiling \
  && target/release/ceiling compare russian noun $FLEXION_CACHE/neural/russian-noun/predictions.tsv
```
