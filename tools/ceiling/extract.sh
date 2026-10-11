#!/usr/bin/env bash
# Fill a separate lexeme cache for the ceiling measurements, then export the
# evaluation splits as TSV. Run from the repository root.
set -euo pipefail
export FLEXION_CACHE="${FLEXION_CACHE:-$HOME/.cache/flexion-ceiling}"
D=$HOME/.cache/flexion/dumps
K=$HOME/.cache/kaikki
X=target/release/xtask
langs="${*:-russian latin ancient-greek polish german english sanskrit spanish french italian}"
for lang in $langs; do
  case $lang in
    russian) args=(--dump $D/Russian.jsonl --edition ru=$K/ru/Русский.jsonl) ;;
    german) args=(--dump $D/German.jsonl --edition de=$K/de/Deutsch.jsonl) ;;
    spanish) args=(--dump $D/Spanish.jsonl --edition es=$K/es/Español.jsonl) ;;
    ancient-greek) args=(--dump $D/AncientGreek.jsonl) ;;
    *) args=(--dump $D/${lang^}.jsonl) ;;
  esac
  echo "== $lang" >&2
  nice -n 10 $X extract "$lang" "${args[@]}"
  nice -n 10 $X export "$lang"
done
