# Add Mantle REVM Runner Design

**Date:** 2026-03-07

**Goal**

Add a new `mantle-revm` runner to `evm-bench` so the benchmark suite can compare upstream `revm` and Mantle's public REVM fork side by side without changing the existing `revm` runner.

**Scope**

- Keep `runners/revm` behavior and metadata unchanged.
- Add a sibling runner under `runners/mantle-revm`.
- Make the new runner consume the same benchmark inputs and emit the same newline-separated timing output as every other runner.
- Use the public `mantle-xyz/revm` repository, pinned to a reproducible commit, instead of a local absolute path.
- Run the same benchmark command against both `revm` and `mantle-revm` and verify both appear in results.

**Non-Goals**

- Replace or refactor the existing upstream `revm` runner.
- Vendor the entire Mantle REVM repository into this repo.
- Redesign benchmark discovery, output formatting, or benchmark contracts.
- Add Mantle-specific benchmark semantics beyond what is required to execute the existing suite fairly.

**Approach**

1. Add a new runner crate and metadata file at `runners/mantle-revm`.
2. Mirror the existing runner CLI so `evm-bench` can invoke the new runner without any framework changes.
3. Pin Mantle REVM to public git commit `f13cb92c721681af99b4c5300d15351d2ab16d75` for reproducibility.
4. Deploy the benchmark contract once, then extract the deployed runtime bytecode and rebuild a minimal benchmark database around that runtime so the timed loop covers repeated calls only.
5. Run a targeted benchmark smoke test with both `revm` and `mantle-revm` on the same benchmark to validate apples-to-apples comparison.

**Architecture**

The repository keeps two independent Rust runner crates: the existing upstream `revm` runner and a new `mantle-revm` runner. `evm-bench` already discovers runners from `runner.evm-bench.json`, so the new runner only needs to satisfy the established CLI and output contract.

The new Mantle runner should follow the same high-level lifecycle as the upstream runner: parse bytecode and calldata, deploy once, and then time repeated calls. Because Mantle REVM uses a newer API surface than the existing runner, the implementation should use Mantle's `Context`, `TxEnv`, `BenchmarkDB`, and mainnet builder APIs directly rather than trying to force the older upstream runner structure onto the new crate.

**Execution Flow**

1. Read the creation bytecode from `--contract-code-path` and calldata from `--calldata`.
2. Create a Mantle mainnet EVM and execute one contract creation transaction.
3. Resolve the created contract address and extract the deployed runtime bytecode from journal state.
4. Build a fresh benchmark EVM backed by Mantle's `BenchmarkDB` seeded with the deployed runtime bytecode.
5. Execute the same call transaction `num-runs` times, measuring only the per-call execution and printing one millisecond value per line.

**Dependency Strategy**

- Depend on Mantle REVM from `https://github.com/mantle-xyz/revm`.
- Pin the dependency to commit `f13cb92c721681af99b4c5300d15351d2ab16d75`, resolved on 2026-03-07.
- Prefer minimal direct dependencies from Mantle's public crates instead of local path overrides.
- Keep the new runner outside the root Cargo workspace if that avoids dependency conflicts with the existing upstream `revm` crate version; benchmark invocation only needs `cargo run --manifest-path`.

**Testing Strategy**

- Verify the repo setup in the worktree before implementation with a targeted baseline command.
- Add focused tests in `runners/mantle-revm/src/main.rs` for hex parsing and a minimal deploy-then-call path.
- Run a targeted Cargo test/check command for the new runner crate.
- Run a benchmark smoke test using a stable benchmark such as `ten-thousand-hashes` against both `revm` and `mantle-revm`.

**Error Handling**

- Fail fast on unreadable contract bytecode files or malformed hex calldata.
- Fail with explicit messages if deployment does not succeed, no created address is returned, runtime bytecode cannot be recovered, or timed execution returns a non-success result.
- Preserve the runner contract of writing only numeric timing lines to stdout for successful runs.

**Notes**

- A repo-local `.worktrees/` directory will be added to `.gitignore` before creating the isolated implementation branch.
- I am not committing the design doc separately because the session policy forbids unsolicited commits.
