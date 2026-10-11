#!/usr/bin/env bash
# Train and score the neural baseline on one language's part of speech.
#
#   tools/ceiling/neural/run.sh [--local] <lang> <pos> [seq2seq.py options]
#
# Reads $FLEXION_CACHE/export/<lang>/<pos>.tsv (from `xtask export`) and writes
# $OUT/<lang>-<pos>/ (default OUT=$FLEXION_CACHE/neural). By default it runs in
# a CPU-only container (8 CPUs, no network, the export mounted read-only, a
# read-only image with an in-memory /tmp, nothing written outside the output
# directory; the image is rebuilt when this directory changes). `--local` runs the same script with $PYTHON (default
# python3), which must have requirements.txt installed.
set -euo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
export FLEXION_CACHE="${FLEXION_CACHE:-$HOME/.cache/flexion}"
EXPORT="${EXPORT:-$FLEXION_CACHE/export}"
OUT="${OUT:-$FLEXION_CACHE/neural}"
IMAGE="${IMAGE:-flexion-neural}"

local_python=""
if [ "${1:-}" = "--local" ]; then
  shift
  local_python="${PYTHON:-python3}"
fi
[ $# -ge 2 ] || { sed -n '2,12p' "$0"; exit 2; }
[ -f "$EXPORT/$1/$2.tsv" ] || { echo "no $EXPORT/$1/$2.tsv: run xtask export $1" >&2; exit 1; }
mkdir -p "$OUT"

if [ -n "$local_python" ]; then
  exec nice -n 10 "$local_python" "$here/seq2seq.py" "$@" --export "$EXPORT" --out "$OUT" --threads 8
fi

# Docker: the current shell may predate its docker group membership.
docker_cmd() {
  if docker info >/dev/null 2>&1; then "$@"; else
    printf '%q ' "$@" | newgrp docker
  fi
}
# Cached layers make this quick when nothing changed.
docker_cmd docker build -q -t "$IMAGE" "$here" >/dev/null
docker_cmd docker run --rm --cpus 8 --network none --read-only --tmpfs /tmp \
  --user "$(id -u):$(id -g)" \
  -v "$EXPORT:/export:ro" -v "$OUT:/out" \
  "$IMAGE" "$@" --export /export --out /out --threads 8
