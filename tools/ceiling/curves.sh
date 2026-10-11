#!/usr/bin/env bash
# Learning curves: probe each language trained on 10/25/50/100% of the
# training words, one run at a time. Logs go to $OUT/<lang>[-<pos>]-<f>.log;
# the full-data model is kept as $FLEXION_CACHE/<lang>-full.bin for
# `ceiling ending` and `ceiling compare`. Run from the repository root after
# tools/ceiling/extract.sh.
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
      if [ "$f" = 1.0 ]; then cp "$FLEXION_CACHE/$lang-lexemes.bin" "$FLEXION_CACHE/$lang${p:+-$p}-full.bin"; fi
    done
  done
done
