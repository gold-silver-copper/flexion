"""Learning-curve tables from the logs `curves.sh` writes.

usage: python3 curves_table.py <curves dir> <export dir>

Per language and main part of speech: held-out top-1 and log-loss (nats per
cell) at each training share, from the bare lemma (Lemma), the lemma written
in full (StressedLemma) and with principal parts (Citation), and the gain of
the last doubling (50% -> 100%).
"""

import re
import sys
from pathlib import Path

FRACTIONS = ["0.1", "0.25", "0.5", "1.0"]
MAIN = ["noun", "adjective", "verb"]
LINE = re.compile(r"^  (\w+): n (\d+) top1 ([\d.]+)% .* loss ([\d.]+)")


def parse(path):
    """{pos: {mode: (n, top1, loss)}}"""
    out, pos = {}, None
    for line in path.read_text().splitlines():
        if line.startswith("== "):
            pos = line[3:].split(":")[0]
            out[pos] = {}
        elif pos and (m := LINE.match(line)):
            mode, n, top1, loss = m.groups()
            if int(n) > 0:
                out[pos][mode] = (int(n), float(top1), float(loss))
    return out


def train_lemmas(export, lang, pos):
    path = export / lang / f"{pos}.tsv"
    if not path.exists():
        return None
    seen = set()
    with open(path, encoding="utf-8") as f:
        for line in f:
            split, lemma = line.split("\t", 2)[:2]
            if split != "test":
                seen.add(lemma)
    return len(seen)


def main():
    curves, export = Path(sys.argv[1]), Path(sys.argv[2])
    langs = sorted({re.sub(r"(-(noun|adjective|verb))?-[\d.]+\.log$", "", p.name)
                    for p in curves.glob("*.log")})
    print("| language | pos | train lemmas (100%) | mode | 10% | 25% | 50% | 100% | last doubling |")
    print("|---|---|---:|---|---:|---:|---:|---:|---:|")
    for lang in langs:
        runs = {}
        for f in FRACTIONS:
            merged = {}
            for p in [curves / f"{lang}-{f}.log", *[curves / f"{lang}-{q}-{f}.log" for q in MAIN]]:
                if p.exists():
                    merged.update(parse(p))
            runs[f] = merged
        for pos in MAIN:
            if pos not in runs["1.0"]:
                continue
            for mode in ["Lemma", "StressedLemma", "Citation"]:
                cells = [runs[f].get(pos, {}).get(mode) for f in FRACTIONS]
                if not all(cells):
                    continue
                shown = " | ".join(f"{c[1]:.2f} ({c[2]:.3f})" for c in cells)
                gain = cells[3][1] - cells[2][1]
                print(f"| {lang} | {pos} | {train_lemmas(export, lang, pos)} | {mode} | {shown} | "
                      f"{gain:+.2f} |")


if __name__ == "__main__":
    main()
