# The fifth panel's plan: where the run stopped (2026-10-18)

The owner stopped the run mid-way. This records what is merged on this branch and what is in
progress on side branches, so the run can resume from here.

The plan is `prompts/trainer-panel-5-plan.md`, and the panel's decision is
`prompts/trainer-panel-5-decision.md`. The full record is in the notes repo, under
`projects/flexion/trainer-panel-5-2026-10-18*.md`; the progress log is
`grammar-crates.md`, section "PR 22".

## Merged on this branch (`panel-five`)

| commit | what | verdict |
|---|---|---|
| 5e6598c | the panel's prompt and the plan | – |
| 4bd3264 | performance (a): the workspace's own crates at opt-level 2 in dev and test builds. The end-to-end test, 15 tests, runs in 35 s (72 s with compiling); CI's saved-build test alone took 806 s | built |
| dc125e6 | performance (b): runs scheduled from measured costs (`target/build-cache/runs.tsv`), `COSTS` and the heavy-language lists deleted, `cargo xtask schedule`. On PR 21's twelve build times: 5,074 s on two lanes (6,073 before), 4,107 on three | built |
| 84c35ad, f102947, fb8e4a0, 74a9eba | item 5 (a), (e), (f): the root README written by `cargo xtask readme` (reproduces all 231 rows of the four root tables on PR 21's builds); `xtask bytes`; `xtask compare <dir> <dir>` verdicts; licence files in every crate; the language list derived | built |

No crate under `languages/` changed: no rebuild has run.

## In progress on side branches (pushed, unmerged, mostly unscreened)

Each was built in its own worktree off this branch. Their screens were queued and stopped part
way: no verdicts yet unless stated.

| branch | item | state |
|---|---|---|
| `p5-item1` | 1: one relation matcher, two policies (a) bind-only and (b) longest literal | 4 commits; screens of both policies started (`panel5-plan/item1/screen-a`, `-a2`, `-b`), not finished |
| `p5-item2` | 2: Greek preverbs by the runtime's spelling, halves (i) and (ii) | 3 commits; the prefix-stage dump across 75 parts and the screens of (i), (ii), both: queued, not run |
| `p5-item3` | 3 (a) Greek -εως short, (b) harmony stage order, (c) the 106% candidate | 3 commits, the last a WIP commit of uncommitted work; (b)'s screen not finished |
| `p5-item3d` | 3 (d) German stacked prefixes | 1 commit (5773020) with a unit test; screen queued, not run |
| `p5-item4` | 4 (d) exact deletions, (a) one word count, (b) ROOT_TRUST measured, (c) one tolerance function | 4 commits (542ac8b, 494de32, 0fa09df, 38ac114); exact check of (d) and the combined screen of (a)+(b)+(c) queued, not run |
| `p5-item5b` | 5 (b) tenth roles and the POOL leak, (c) census kinds named once with a new "reading" kind, (d) printer savings (predicted rules.rs −706 KB, all choice files −1.17 MB) | 3 commits (445e985, 28de4e7, 3171db6); checks queued, not run. Note: `try --fold 9` becomes `--confirm`. Conflicts with item 5a in `report.rs` (resolve to 5b's `Kind::ALL` columns in 5a's errors-table function) |
| `p5-item6` | 6: one way to learn a part | 1 commit; checks not finished |

Not started: E1, E2, performance part 2 ((c)–(h)), the integration set, tenth 9, the rebuild.

## To resume

- **Scratch** is `~/.cache/claude-tmp/panel5-plan/`. Keep it until the run resumes. It holds:
  - the baselines (`base/dev`, `base/t9`);
  - the queue (`q/`, stopped: `q/stop` exists);
  - the helpers (`screen.sh`, `cmp.sh`, `AGENT.md`);
  - each item's partial screens;
  - the run state, `state.md`.
- **The worktrees** `wt-item*` are still in place; one is checked out on each branch above.
- **Restart the queue:**
  ```
  cd ~/.cache/claude-tmp/panel5-plan && rm q/stop && setsid nohup ./worker.sh 1 & setsid nohup ./worker.sh 2 &
  ```
  Jobs that were running when stopped are left in `q/running/`: move them back to
  `q/pending/` first.
- **Then judge the items** in the plan's run order and merge them one at a time.
