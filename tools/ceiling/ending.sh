#!/usr/bin/env bash
# The irreducible part: how predictable each lemma-level choice is from the
# spelling. Run from the repository root after extract.sh and curves.sh.
set -euo pipefail
export FLEXION_CACHE="${FLEXION_CACHE:-$HOME/.cache/flexion-ceiling}"
C=target/release/ceiling
for args in "russian noun stress" "russian noun Animacy" "russian noun Gender" \
            "russian adjective stress" "russian verb stress" "russian verb Aspect" \
            "latin noun length" "latin adjective length" "latin verb length" \
            "ancient-greek noun length" "ancient-greek adjective length" "ancient-greek verb length"; do
  nice -n 10 $C ending $args
done
