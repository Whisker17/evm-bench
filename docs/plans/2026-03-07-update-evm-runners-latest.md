# Update EVM Runners to Latest Stable Versions Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Upgrade every runner dependency to the latest stable supported version, rerun the benchmark suite, and refresh `README.md` with the new benchmark results.

**Architecture:** Resolve current upstream versions first, then update runner manifests one ecosystem at a time so compatibility fixes stay localized. Validate each changed runner with the smallest relevant build command before running the full benchmark suite and updating the published results table.

**Tech Stack:** Rust/Cargo, Go modules, npm/TypeScript, Poetry/Python, CMake/C++, git worktrees

---

### Task 1: Record current runner version sources

**Files:**
- Modify: `docs/plans/2026-03-07-update-evm-runners-latest.md`
- Modify: `runners/revm/Cargo.toml`
- Modify: `runners/geth/go.mod`
- Modify: `runners/ethereumjs/package.json`
- Modify: `runners/pyrevm/pyproject.toml`
- Modify: `runners/py-evm/pyproject.toml`
- Modify: `runners/evmone/CMakeLists.txt`

**Step 1: Check primary upstream sources for the latest stable releases**

Run official-source lookups for:
- `revm`
- `go-ethereum`
- `@ethereumjs/common`, `@ethereumjs/evm`, `@ethereumjs/util`, `@ethereumjs/vm`
- `pyrevm`
- `py-evm`
- `evmone`

**Step 2: Record the target versions**

Write down the selected stable versions before changing manifests so later README updates can cite the exact toolchain used.

### Task 2: Update manifest pins with minimal changes

**Files:**
- Modify: `runners/revm/Cargo.toml`
- Modify: `runners/geth/go.mod`
- Modify: `runners/ethereumjs/package.json`
- Modify: `runners/pyrevm/pyproject.toml`
- Modify: `runners/py-evm/pyproject.toml`
- Modify: `runners/evmone/CMakeLists.txt`

**Step 1: Make the manifest changes**

Update the version constraints only where needed:
- Rust crate pin in `runners/revm/Cargo.toml`
- Go module requirement in `runners/geth/go.mod`
- npm package versions in `runners/ethereumjs/package.json`
- Poetry dependencies in `runners/pyrevm/pyproject.toml` and `runners/py-evm/pyproject.toml`
- CPM package reference in `runners/evmone/CMakeLists.txt`

**Step 2: Refresh lockfiles or resolved dependencies if the ecosystem requires them**

Use the package manager’s normal update command rather than hand-editing generated files.

### Task 3: Fix runner compatibility regressions

**Files:**
- Modify: `runners/revm/src/main.rs`
- Modify: `runners/geth/runner.go`
- Modify: `runners/ethereumjs/runner.ts`
- Modify: `runners/pyrevm/runner.py`
- Modify: `runners/py-evm/runner.py`
- Modify: `runners/evmone/runner.cpp`

**Step 1: Run the smallest build command that exercises each changed runner**

Suggested commands:
- `cargo check --manifest-path runners/revm/Cargo.toml`
- `go build ./...` from `runners/geth`
- `npm install` and `npm run nodejs-runner -- --help` from `runners/ethereumjs`
- `poetry install` for `runners/pyrevm`
- `poetry install` for `runners/py-evm`
- `cmake -S . -B build && cmake --build build` from `runners/evmone`

**Step 2: Apply the smallest code changes required by upstream API drift**

Keep fixes local to the relevant runner and avoid incidental refactors.

**Step 3: Re-run the targeted build command**

Do not move on until the runner builds or the blocker is clearly documented.

### Task 4: Re-run the benchmark suite

**Files:**
- Modify: `README.md`
- Inspect: `outputs/results/*`

**Step 1: Run the benchmark command from the worktree**

Run: `cargo run --release`

**Step 2: Capture the output result file and table**

Use the newest artifact under `outputs/results/` and the CLI table output as the source of truth for the README refresh.

### Task 5: Refresh published documentation

**Files:**
- Modify: `README.md`

**Step 1: Replace the benchmark table with the newly measured results**

Update the summary rows and each benchmark row using the fresh output.

**Step 2: Add or refresh version context**

List the runner versions used for the new benchmark run if the README does not already make them clear.

**Step 3: Run final verification**

Run:
- `cargo test`
- `cargo run --release -- --display`

Expected:
- tests pass
- display command shows the newest results without parsing errors

Given the user asked me to carry this out now, I am using the in-session execution path rather than stopping for a separate plan handoff.
