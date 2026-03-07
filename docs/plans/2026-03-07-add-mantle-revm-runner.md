# Add Mantle REVM Runner Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add a new `mantle-revm` runner that benchmarks the same contracts as the existing `revm` runner so `evm-bench` can compare both implementations side by side.

**Architecture:** Keep `runners/revm` unchanged and add a sibling `runners/mantle-revm` crate with the same CLI contract and output format. Use Mantle REVM's public git repository at pinned commit `f13cb92c721681af99b4c5300d15351d2ab16d75`, deploy benchmark bytecode once, then rebuild a minimal benchmark database around the deployed runtime bytecode so the timed loop measures repeated calls only.

**Tech Stack:** Rust/Cargo, git worktrees, `revm`, Mantle `mantle-xyz/revm`, shell runner entry scripts

---

### Task 1: Prepare the isolated worktree

**Files:**
- Modify: `.gitignore`
- Modify: `docs/plans/2026-03-07-add-mantle-revm-runner.md`

**Step 1: Ignore the repo-local worktree directory**

Add:

```gitignore
.worktrees/
```

to `.gitignore` if it is not already ignored.

**Step 2: Create the feature worktree**

Run:

```bash
git worktree add .worktrees/mantle-revm -b feature/mantle-revm-runner
```

Expected: Git creates a new worktree rooted at `.worktrees/mantle-revm`.

**Step 3: Verify the baseline from the worktree**

Run:

```bash
cargo test
```

Expected: existing tests pass before implementation starts, or failures are reported before proceeding.

### Task 2: Scaffold the new runner crate

**Files:**
- Create: `runners/mantle-revm/Cargo.toml`
- Create: `runners/mantle-revm/entry.sh`
- Create: `runners/mantle-revm/runner.evm-bench.json`
- Modify: `Cargo.toml`

**Step 1: Write the metadata and entry script**

Create `runner.evm-bench.json` with:

```json
{
  "$schema": "../schema.json",
  "name": "mantle-revm",
  "entry": "entry.sh"
}
```

Create `entry.sh` to mirror the existing Rust runners:

```bash
#!/usr/bin/env bash
set -eo pipefail

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

cargo run --profile runner --manifest-path "$SCRIPT_DIR/Cargo.toml" -- "$@"
```

**Step 2: Add the runner manifest**

Create `runners/mantle-revm/Cargo.toml` with a pinned git dependency on Mantle REVM commit `f13cb92c721681af99b4c5300d15351d2ab16d75`, plus the smallest supporting dependencies needed for CLI parsing and logging.

**Step 3: Decide workspace inclusion**

Update root `Cargo.toml` to include `runners/mantle-revm/` in `[workspace].members` if that allows `cargo test` to cover the new runner cleanly. If workspace inclusion causes avoidable dependency friction, leave the new runner standalone and validate it with `--manifest-path`.

### Task 3: Add a failing testable seam for the runner logic

**Files:**
- Create: `runners/mantle-revm/src/main.rs`

**Step 1: Write failing tests first**

Add inline Rust tests that define the desired behavior for helper functions such as:

```rust
#[test]
fn parses_hex_inputs_without_prefix_noise() {
    let bytes = parse_hex_bytes("00ff").unwrap();
    assert_eq!(bytes.len(), 2);
}

#[test]
fn deploys_and_runs_minimal_contract() {
    let creation = "600160005560016000f3";
    let runtimes = run_benchmark_once(creation, "", 1).unwrap();
    assert_eq!(runtimes.len(), 1);
}
```

Use the smallest realistic contract sample that can be deployed and called by Mantle REVM.

**Step 2: Run the new runner tests to prove they fail**

Run:

```bash
cargo test --manifest-path runners/mantle-revm/Cargo.toml
```

Expected: FAIL because the helper functions or behavior are not implemented yet.

### Task 4: Implement the Mantle REVM benchmark loop

**Files:**
- Modify: `runners/mantle-revm/src/main.rs`

**Step 1: Add minimal helper functions**

Implement small functions for:
- reading and decoding the contract bytecode file
- decoding calldata
- deploying creation bytecode with Mantle REVM
- extracting the created address and runtime bytecode
- running repeated timed calls against a benchmark database seeded with that runtime bytecode

**Step 2: Wire helpers into the CLI entrypoint**

Implement `main()` so it:
- parses `--contract-code-path`, `--calldata`, and `--num-runs`
- deploys once
- times only the repeated call loop
- prints one millisecond timing per line

**Step 3: Keep failure modes explicit**

Use `expect`/`panic!` messages or `anyhow`-style error propagation so deployment failures, missing runtime bytecode, and failed calls are easy to diagnose during benchmark runs.

### Task 5: Make the failing tests pass

**Files:**
- Modify: `runners/mantle-revm/src/main.rs`

**Step 1: Re-run the targeted test command**

Run:

```bash
cargo test --manifest-path runners/mantle-revm/Cargo.toml
```

Expected: PASS after the implementation is complete.

**Step 2: Run a focused compile check**

Run:

```bash
cargo check --manifest-path runners/mantle-revm/Cargo.toml
```

Expected: PASS with no unresolved API mismatches against the pinned Mantle REVM commit.

### Task 6: Verify end-to-end benchmark comparison

**Files:**
- Inspect: `runners/revm/src/main.rs`
- Inspect: `runners/mantle-revm/src/main.rs`
- Inspect: `outputs/results/*`

**Step 1: Run a side-by-side smoke benchmark**

Run:

```bash
RUST_LOG=info cargo run -- --runners revm mantle-revm --benchmarks ten-thousand-hashes
```

Expected: both runners execute the same benchmark and appear in the output results.

**Step 2: Run the Mantle runner alone**

Run:

```bash
RUST_LOG=info cargo run -- --runners mantle-revm --benchmarks ten-thousand-hashes
```

Expected: `mantle-revm` succeeds independently and emits valid timing lines.

**Step 3: Run final regression verification**

Run:

```bash
cargo test
```

Expected: the repo test suite still passes with the new runner added.

Given the user asked me to carry this out now, I am using the in-session execution path rather than stopping for a separate plan handoff.
