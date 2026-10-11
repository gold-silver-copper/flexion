#!/usr/bin/env bash
# Learning curves: probe each language trained on 10/25/50/100% of the
# training words, one run at a time. Logs go to $OUT/<lang>[-<pos>]-<f>.log.
# Run from the repository root after tools/ceiling/extract.sh.
set -euo pipefail
export FLEXION_CACHE="${FLEXION_CACHE:-$HOME/.cache/flexion-ceiling}"
OUT="${OUT:-$FLEXION_CACHE/curves}"
mkdir -p "$OUT"
X=target/release/xtask
langs="${*:-russian latin ancient-greek polish german english sanskrit spanish french italian}"
for lang in $langs; do
  pos=("")
  [ "$lang" = ancient-greek ] && pos=(noun adjective verb)
  for f in 1.0 0.5 0.25 0.1; do
    for p in "${pos[@]}"; do
      name="$lang${p:+-$p}-$f"
      echo "== $name $(date +%T)" >&2
      nice -n 10 $X probe "$lang" --fraction "$f" ${p:+--pos "$p"} > "$OUT/$name.log" 2> "$OUT/$name.err"
    done
  done
done
