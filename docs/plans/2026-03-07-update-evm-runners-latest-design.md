# Update EVM Runners to Latest Stable Versions Design

**Date:** 2026-03-07

**Goal**

Upgrade every runner under `runners/` to the latest stable upstream version that can be supported in this repository, rerun the benchmark suite, and refresh `README.md` with the new numbers and version details.

**Scope**

- Update version pins for `revm`, `geth`, `ethereumjs`, `pyrevm`, `py-evm`, and `evmone`.
- Make the smallest runner code changes required by upstream API changes.
- Rerun the benchmark suite and capture new timings.
- Update the top-level `README.md` to reflect the refreshed benchmark data.

**Non-Goals**

- Redesign the benchmark framework.
- Change benchmark definitions unless an upstream compatibility issue requires a minimal fix.
- Optimize runner implementations beyond what is needed to restore compatibility.

**Approach**

1. Resolve the latest stable versions from primary upstream sources for each runner dependency.
2. Update the runner manifests and lockfiles with minimal edits.
3. Rebuild each runner and fix compatibility issues locally in the affected runner implementation.
4. Run the benchmark suite from the worktree and record the output artifact.
5. Update `README.md` with the new benchmark table, plus version context and reproduction notes if needed.

**Compatibility Strategy**

- Prefer stable releases over prereleases.
- Keep package groups compatible where upstream packages move in lockstep, especially the `@ethereumjs/*` packages.
- If a runner has no usable stable release path, document the blocker explicitly instead of silently leaving stale claims in the README.

**Validation**

- Baseline project verification: `cargo test`
- Post-upgrade verification:
  - `cargo test`
  - Targeted build or install checks for changed runners
  - Full benchmark rerun with `cargo run --release`

**Notes**

- A repo-local worktree is used for implementation so edits and build outputs stay isolated.
- I am not committing the design doc separately because the session policy forbids unsolicited commits.
