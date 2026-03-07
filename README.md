# evm-bench

[![CI](https://github.com/ziyadedher/evm-bench/actions/workflows/ci.yml/badge.svg)](https://github.com/ziyadedher/evm-bench/actions/workflows/ci.yml)

**evm-bench is a suite of Ethereum Virtual Machine (EVM) stress tests and benchmarks.**

evm-bench makes it easy to compare EVM performance in a scalable, standardized, and portable way.

|                         | pyrevm   | revm     | geth      | ethereumjs | py-evm.cpython |
| ----------------------- | -------- | -------- | --------- | ---------- | -------------- |
| **sum**                 | 65.857ms | 79.743ms | 132.941ms | 12.749s    | 18.034s        |
| **relative**            | 1.000x   | 1.211x   | 2.019x    | 193.582x   | 273.841x       |
| erc20.approval-transfer | 6.271ms  | 6.335ms  | 11.738ms  | 1.009s     | 908.891ms      |
| erc20.mint              | 4.944ms  | 5.428ms  | 10.594ms  | 1.948s     | 930.605ms      |
| erc20.transfer          | 7.978ms  | 8.019ms  | 14.523ms  | 2.097s     | 1.324s         |
| snailtracer             | 42.849ms | 55.374ms | 88.801ms  | 7.491s     | 14.067s        |
| ten-thousand-hashes     | 3.815ms  | 4.588ms  | 7.286ms   | 204.837ms  | 804.090ms      |

Benchmarked on 2026-03-07 with:

- `pyrevm` `0.3.7`
- `revm` `36.0.0`
- `go-ethereum` `v1.17.1`
- `@ethereumjs/common`, `@ethereumjs/evm`, `@ethereumjs/util`, `@ethereumjs/vm` `10.1.1`
- `py-evm` `0.12.1b1`

This rerun used the result artifact at `outputs/results/2026-03-07-latest.evm-bench.results.json`.
`evmone` was updated to `0.19.0` but not included in this run because `cmake` was unavailable in the environment.
`py-evm.pypy` was not included because `pypy3` was unavailable in the environment.

To reproduce these results, check out [usage with the evm-bench suite below](#with-the-evm-bench-suite).

## Technical Overview

In evm-bench there are [benchmarks](/benchmarks) and [runners](/runners):

- [Benchmarks](/benchmarks) are expensive Solidity contracts paired with configuration.
- [Runners](/runners) are consistent platforms for deploying and calling arbitrary smart contracts.

The evm-bench framework can run any benchmark on any runner. The links above dive deeper into how to build new benchmarks or runners.

## Usage

### With the evm-bench suite

Simply cloning this repository and running `cargo run` will do the trick. You may need to install some dependencies for the benchmark build process and the runner execution.

### With another suite

evm-bench is meant to be used with the pre-developed suite of benchmarks and runners in this repository. However, it should work as an independent framework elsewhere.

See the CLI arguments for evm-bench to figure out how to set it up! Alternatively just reach out to me or post an issue.

## Development

Do it. Reach out to me if you wanna lend a hand but don't know where to start!
