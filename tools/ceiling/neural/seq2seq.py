"""A character-level transformer for inflection, as a measuring stick for flexion.

The standard model of the SIGMORPHON inflection shared tasks (Wu, Cotterell and
O'Donnell 2021): input = the cell's feature tags, then the lemma's characters;
output = the form's characters, stress and length marks included (text is
NFD-normalized, so a mark is its own character). It trains on the `train` rows
of `xtask export`, keeps the checkpoint with the best exact-match accuracy on a
fixed sample of `dev`, and scores `test` with flexion's rule: top-1 is right
when the greedy answer is the primary form. It also reports the log-loss of the
primary form (nats per cell, the probability floored at 1e-4, as `xtask probe`
reports it).

With `--target lemma` it learns the lemma written in full instead (one row per
word: Latin `rosa` -> `rosa`, `abactio` -> `abāctiō`), to measure how much of
vowel length the whole spelling predicts.

Writes `<out>/<lang>-<pos>[-lemma]/predictions.tsv` (lemma, cell, primary, answer,
probability of the primary form), `metrics.json` and `log.txt`.
"""

import argparse
import json
import math
import random
import sys
import time
import unicodedata
import warnings
from pathlib import Path

import torch
from torch import nn

warnings.filterwarnings("ignore", message="enable_nested_tensor")

PAD, BOS, EOS, UNK = 0, 1, 2, 3
SPECIALS = ["<pad>", "<s>", "</s>", "<unk>"]
MIN_PROBABILITY = 1e-4


def nfd(s):
    return unicodedata.normalize("NFD", s)


def same_form(a, b):
    """flexion's equality of forms, closely enough: letters and marks after
    case folding, a grave read as an acute, an acute on ё implied."""
    def norm(s):
        s = unicodedata.normalize("NFC", nfd(s.lower()).replace("\u0300", "\u0301"))
        # Russian ё is stressed unless another vowel is marked.
        if s.count("\u0301") == 1 and "ё\u0301" in s:
            s = s.replace("ё\u0301", "ё")
        return s
    return norm(a) == norm(b)


def letters(s):
    """The form's letters, every combining mark dropped."""
    return "".join(c for c in nfd(s.lower()) if not unicodedata.combining(c))


def read_split(path):
    rows = {"train": [], "dev": [], "test": []}
    with open(path, encoding="utf-8") as f:
        for line in f:
            cols = line.rstrip("\n").split("\t")
            if len(cols) < 5:
                continue
            split, lemma, full, cell, primary = cols[:5]
            rows[split].append((lemma, full, cell, primary))
    return rows


def length_marks(s):
    """Per letter, its macron and breve (vowel length), nothing else."""
    out = []
    for c in nfd(s.lower()):
        if c in "\u0304\u0306":
            if out:
                out[-1] += c
        elif not unicodedata.combining(c):
            out.append("")
    return out


class Vocab:
    def __init__(self, symbols):
        self.itos = SPECIALS + sorted(set(symbols) - set(SPECIALS))
        self.stoi = {s: i for i, s in enumerate(self.itos)}

    def encode(self, symbols):
        return [self.stoi.get(s, UNK) for s in symbols]

    def __len__(self):
        return len(self.itos)


def source_symbols(lemma, cell):
    return [f"[{t}]" for t in cell.split(".")] + list(nfd(lemma))


class Model(nn.Module):
    def __init__(self, n_src, n_tgt, d, heads, layers, ff, dropout, max_len):
        super().__init__()
        self.d = d
        self.src_emb = nn.Embedding(n_src, d, padding_idx=PAD)
        self.tgt_emb = nn.Embedding(n_tgt, d, padding_idx=PAD)
        self.pos = nn.Embedding(max_len, d)
        self.net = nn.Transformer(
            d_model=d,
            nhead=heads,
            num_encoder_layers=layers,
            num_decoder_layers=layers,
            dim_feedforward=ff,
            dropout=dropout,
            batch_first=True,
            norm_first=True,
        )
        self.out = nn.Linear(d, n_tgt)

    def embed(self, emb, x):
        positions = torch.arange(x.size(1), device=x.device).unsqueeze(0)
        return emb(x) * math.sqrt(self.d) + self.pos(positions)

    def encode(self, src):
        mask = src == PAD
        return self.net.encoder(self.embed(self.src_emb, src), src_key_padding_mask=mask), mask

    def decode(self, memory, src_mask, tgt):
        causal = nn.Transformer.generate_square_subsequent_mask(tgt.size(1), dtype=torch.bool)
        h = self.net.decoder(
            self.embed(self.tgt_emb, tgt),
            memory,
            tgt_mask=causal,
            tgt_key_padding_mask=tgt == PAD,
            memory_key_padding_mask=src_mask,
            tgt_is_causal=True,
        )
        return self.out(h)

    def forward(self, src, tgt_in):
        memory, src_mask = self.encode(src)
        return self.decode(memory, src_mask, tgt_in)


def pad(seqs):
    width = max(len(s) for s in seqs)
    return torch.tensor([s + [PAD] * (width - len(s)) for s in seqs], dtype=torch.long)


@torch.no_grad()
def greedy(model, src, max_len):
    memory, src_mask = model.encode(src)
    out = torch.full((src.size(0), 1), BOS, dtype=torch.long)
    done = torch.zeros(src.size(0), dtype=torch.bool)
    for _ in range(max_len):
        logits = model.decode(memory, src_mask, out)[:, -1]
        nxt = logits.argmax(-1).masked_fill(done, PAD)
        out = torch.cat([out, nxt.unsqueeze(1)], dim=1)
        done |= nxt == EOS
        if bool(done.all()):
            break
    return out[:, 1:]


@torch.no_grad()
def score_primary(model, src, tgt):
    """Per target (EOS-terminated): its log-probability, and whether every
    token is the argmax given the right prefix, which is exactly when greedy
    decoding writes it."""
    tgt_in = torch.cat([torch.full((tgt.size(0), 1), BOS, dtype=torch.long), tgt[:, :-1]], dim=1)
    tgt_in = tgt_in.masked_fill(tgt_in == EOS, PAD)
    logp = model(src, tgt_in).log_softmax(-1)
    real = tgt != PAD
    picked = logp.gather(-1, tgt.unsqueeze(-1)).squeeze(-1)
    exact = ((logp.argmax(-1) == tgt) | ~real).all(-1)
    return (picked * real).sum(-1), exact


def length(e):
    return len(e[0]) + len(e[1])


def chunks(examples, batch):
    """(indices, examples) batches of similar length, so little is padding."""
    order = sorted(range(len(examples)), key=lambda i: length(examples[i]))
    for i in range(0, len(order), batch):
        idx = order[i : i + batch]
        yield idx, [examples[j] for j in idx]


def exact_rate(model, examples, batch=1024):
    model.eval()
    hits = 0
    for _, chunk in chunks(examples, batch):
        _, exact = score_primary(model, pad([e[0] for e in chunk]), pad([e[1] for e in chunk]))
        hits += int(exact.sum())
    return hits / max(1, len(examples))


def predict(model, examples, tvocab, max_out, batch=512):
    """Greedy answers and the primary form's probability, for (src, tgt, …)."""
    model.eval()
    answers, probs = [""] * len(examples), [0.0] * len(examples)
    for idx, chunk in chunks(examples, batch):
        src = pad([e[0] for e in chunk])
        for i, row in zip(idx, greedy(model, src, max_out).tolist()):
            chars = []
            for t in row:
                if t in (EOS, PAD):
                    break
                chars.append(tvocab.itos[t])
            answers[i] = unicodedata.normalize("NFC", "".join(chars))
        lp, _ = score_primary(model, src, pad([e[1] for e in chunk]))
        for i, x in zip(idx, lp.tolist()):
            probs[i] = math.exp(x)
    return answers, probs


def batches(examples, batch, rng, pool=64):
    """Endless shuffled training batches: each pool of `pool` batches' worth
    of examples is sorted by length and cut into batches, which are shuffled."""
    while True:
        order = list(range(len(examples)))
        rng.shuffle(order)
        for i in range(0, len(order), batch * pool):
            part = sorted(order[i : i + batch * pool], key=lambda j: length(examples[j]))
            cut = [part[j : j + batch] for j in range(0, len(part), batch)]
            rng.shuffle(cut)
            for idx in cut:
                yield [examples[j] for j in idx]


def main():
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("lang")
    ap.add_argument("pos")
    ap.add_argument("--export", required=True, help="the export directory (<cache>/export)")
    ap.add_argument("--out", required=True, help="where results go")
    ap.add_argument("--target", choices=["forms", "lemma"], default="forms",
                    help="forms: every scored cell; lemma: the lemma written in full "
                         "(vowel length restored), one row per word")
    ap.add_argument("--steps", type=int, default=3000)
    ap.add_argument("--batch", type=int, default=256)
    ap.add_argument("--d-model", type=int, default=256)
    ap.add_argument("--layers", type=int, default=3)
    ap.add_argument("--heads", type=int, default=4)
    ap.add_argument("--ff", type=int, default=1024)
    ap.add_argument("--dropout", type=float, default=0.0)
    ap.add_argument("--lr", type=float, default=2e-3)
    ap.add_argument("--warmup", type=int, default=300)
    ap.add_argument("--label-smoothing", type=float, default=0.0)
    ap.add_argument("--eval-every", type=int, default=300)
    ap.add_argument("--dev-sample", type=int, default=10000)
    ap.add_argument("--bf16", action=argparse.BooleanOptionalAction, default=True,
                    help="train in bfloat16 autocast (faster on CPUs with bf16 support)")
    ap.add_argument("--seed", type=int, default=1)
    ap.add_argument("--threads", type=int, default=8)
    ap.add_argument("--max-minutes", type=float, default=10.0,
                    help="stop training early (and say so) past this many minutes")
    args = ap.parse_args()

    torch.set_num_threads(args.threads)
    torch.manual_seed(args.seed)
    torch.use_deterministic_algorithms(True)
    rng = random.Random(args.seed)
    t0 = time.time()
    suffix = "-lemma" if args.target == "lemma" else ""
    out_dir = Path(args.out) / f"{args.lang}-{args.pos}{suffix}"
    out_dir.mkdir(parents=True, exist_ok=True)
    log_file = open(out_dir / "log.txt", "w", encoding="utf-8")

    def log(msg):
        line = f"[{time.time() - t0:7.1f}s] {msg}"
        print(line, flush=True)
        log_file.write(line + "\n")
        log_file.flush()

    rows = read_split(Path(args.export) / args.lang / f"{args.pos}.tsv")
    if args.target == "lemma":
        # One row per word: the lemma as typed -> the lemma written in full.
        rows = {k: sorted({(l, "lemma", f) for l, f, _, _ in v}) for k, v in rows.items()}
    else:
        rows = {k: [(l, c, p) for l, _, c, p in v] for k, v in rows.items()}
    log(f"{args.lang} {args.pos}: " + ", ".join(f"{k} {len(v)} pairs" for k, v in rows.items()))
    svocab = Vocab(s for lemma, cell, _ in rows["train"] for s in source_symbols(lemma, cell))
    tvocab = Vocab(c for _, _, form in rows["train"] for c in nfd(form))
    max_len = 2 + max(
        max(len(source_symbols(l, c)), len(nfd(f))) for split in rows.values() for l, c, f in split
    )

    def example(row):
        lemma, cell, form = row
        return (svocab.encode(source_symbols(lemma, cell)), tvocab.encode(list(nfd(form))) + [EOS], row)

    train = [example(r) for r in rows["train"]]
    dev = [example(r) for r in rows["dev"]]
    rng.shuffle(dev)
    dev = dev[: args.dev_sample]
    test = [example(r) for r in rows["test"]]
    max_out = max(len(e[1]) for e in train) + 4

    model = Model(len(svocab), len(tvocab), args.d_model, args.heads, args.layers, args.ff,
                  args.dropout, max_len + 8)
    params = sum(p.numel() for p in model.parameters())
    log(f"vocab {len(svocab)}/{len(tvocab)}, {params / 1e6:.2f}M parameters, "
        f"{args.steps} steps of {args.batch} (~{args.steps * args.batch / max(1, len(train)):.2f} epochs)")
    opt = torch.optim.AdamW(model.parameters(), lr=args.lr, betas=(0.9, 0.98), weight_decay=0.01)

    def lr_at(step):
        warm = min(1.0, (step + 1) / args.warmup)
        progress = max(0.0, (step - args.warmup) / max(1, args.steps - args.warmup))
        return args.lr * warm * 0.5 * (1 + math.cos(math.pi * min(1.0, progress)))

    loss_fn = nn.CrossEntropyLoss(ignore_index=PAD, label_smoothing=args.label_smoothing)
    best = (-1.0, 0, None)
    stream = batches(train, args.batch, rng)
    stopped_early = False
    for step in range(args.steps):
        batch = next(stream)
        src = pad([e[0] for e in batch])
        tgt = pad([[BOS] + e[1] for e in batch])
        tgt_in = tgt[:, :-1].masked_fill(tgt[:, :-1] == EOS, PAD)
        model.train()
        with torch.autocast("cpu", dtype=torch.bfloat16, enabled=args.bf16):
            logits = model(src, tgt_in)
        loss = loss_fn(logits.float().reshape(-1, logits.size(-1)), tgt[:, 1:].reshape(-1))
        for g in opt.param_groups:
            g["lr"] = lr_at(step)
        opt.zero_grad()
        loss.backward()
        nn.utils.clip_grad_norm_(model.parameters(), 1.0)
        opt.step()
        last = step + 1 == args.steps
        over = (time.time() - t0) / 60 > args.max_minutes
        if (step + 1) % args.eval_every == 0 or last or over:
            acc = exact_rate(model, dev)
            log(f"step {step + 1}: loss {loss.item():.4f}, dev top-1 {100 * acc:.2f}%")
            if acc > best[0]:
                best = (acc, step + 1, {k: v.clone() for k, v in model.state_dict().items()})
        if over:
            log(f"stopped at step {step + 1}: past {args.max_minutes} minutes")
            stopped_early = True
            break
    train_minutes = (time.time() - t0) / 60
    model.load_state_dict(best[2])
    log(f"best dev top-1 {100 * best[0]:.2f}% at step {best[1]}; scoring {len(test)} test pairs")

    answers, probs = predict(model, test, tvocab, max_out)
    per_cell = {}
    top1 = top1_letters = top1_length = 0
    nats = 0.0
    with open(out_dir / "predictions.tsv", "w", encoding="utf-8") as f:
        for (_, _, (lemma, cell, primary)), answer, p in zip(test, answers, probs):
            ok = same_form(answer, primary)
            top1 += ok
            top1_letters += letters(answer) == letters(primary)
            top1_length += length_marks(answer) == length_marks(primary)
            nats -= math.log(max(p, MIN_PROBABILITY))
            c = per_cell.setdefault(cell, [0, 0])
            c[0] += ok
            c[1] += 1
            f.write(f"{lemma}\t{cell}\t{primary}\t{answer}\t{p:.6f}\n")
    n = max(1, len(test))
    metrics = {
        "lang": args.lang,
        "pos": args.pos,
        "test_pairs": len(test),
        "test_lemmas": len({e[2][0] for e in test}),
        "top1": 100 * top1 / n,
        "letters": 100 * top1_letters / n,
        "length_marks": 100 * top1_length / n,
        "loss": nats / n,
        "dev_top1": 100 * best[0],
        "best_step": best[1],
        "train_pairs": len(train),
        "train_minutes": train_minutes,
        "stopped_early": stopped_early,
        "parameters": params,
        "args": vars(args),
        "per_cell": {k: [v[0], v[1]] for k, v in sorted(per_cell.items())},
    }
    (out_dir / "metrics.json").write_text(json.dumps(metrics, indent=1, ensure_ascii=False))
    log(f"test top-1 {metrics['top1']:.2f}%, letters {metrics['letters']:.2f}%, "
        f"length marks {metrics['length_marks']:.2f}%, "
        f"loss {metrics['loss']:.4f} over {len(test)} pairs ({metrics['test_lemmas']} lemmas)")
    log_file.close()


if __name__ == "__main__":
    sys.exit(main())
