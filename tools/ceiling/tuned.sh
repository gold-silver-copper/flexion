#!/usr/bin/env bash
# Probe the parts of speech the neural baseline is compared on with the
# parameters their shipped builds chose (smoothing, pruning, syllable walk,
# joins, part models), keeping each model as
# $FLEXION_CACHE/<lang>-<pos>-tuned.bin for `ceiling compare … <model>`.
# Run from the repository root after extract.sh; one probe at a time.
set -euo pipefail
export FLEXION_CACHE="${FLEXION_CACHE:-$HOME/.cache/flexion-ceiling}"
OUT="${OUT:-$FLEXION_CACHE/tuned}"
mkdir -p "$OUT"
X=target/release/xtask
while read -r lang pos params; do
  [ -n "$lang" ] || continue
  echo "== $lang $pos $(date +%T)" >&2
  # shellcheck disable=SC2086
  nice -n 10 $X probe "$lang" --pos "$pos" $params > "$OUT/$lang-$pos.log" 2> "$OUT/$lang-$pos.err"
  cp "$FLEXION_CACHE/$lang-lexemes.bin" "$FLEXION_CACHE/$lang-$pos-tuned.bin"
done <<'LIST'
russian noun --alpha 2 --tau 1 --support 1 --syllables 2 --parts 1
russian verb --alpha 0.5 --tau 1 --support 1 --joins 1 --parts 1
latin noun --alpha 0.5 --tau 1 --support 1 --parts 1
latin verb --alpha 2 --tau 1 --support 1 --parts 1
ancient-greek noun --alpha 0.5 --tau 2 --support 1
ancient-greek verb --alpha 0.5 --tau 1 --support 1 --syllables 2 --parts 1
german noun --alpha 0.5 --tau 1 --support 1 --parts 1
german verb --alpha 0.5 --tau 1 --support 1 --syllables 0 --joins 1 --parts 1
LIST
