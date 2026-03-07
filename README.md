# evm-bench

[![CI](https://github.com/ziyadedher/evm-bench/actions/workflows/ci.yml/badge.svg)](https://github.com/ziyadedher/evm-bench/actions/workflows/ci.yml)

**evm-bench is a suite of Ethereum Virtual Machine (EVM) stress tests and benchmarks.**

evm-bench makes it easy to compare EVM performance in a scalable, standardized, and portable way.

|                         | evmone   | pyrevm   | revm      | mantle-revm | geth      | py-evm.pypy | py-evm.cpython | ethereumjs |
| ----------------------- | -------- | -------- | --------- | ----------- | --------- | ----------- | -------------- | ---------- |
| **sum**                 | 40.877ms | 66.338ms | 101.316ms | 104.709ms   | 130.696ms | 3.211s      | 9.114s         | 14.783s    |
| **relative**            | 1.000x   | 1.623x   | 2.479x    | 2.562x      | 3.197x    | 78.551x     | 222.951x       | 361.634x   |
| erc20.approval-transfer | 4.348ms  | 6.257ms  | 8.836ms   | 10.104ms    | 10.988ms  | 224.399ms   | 431.895ms      | 1.161s     |
| erc20.mint              | 2.512ms  | 4.929ms  | 7.175ms   | 8.703ms     | 9.161ms   | 208.458ms   | 457.878ms      | 2.414s     |
| erc20.transfer          | 5.061ms  | 8.271ms  | 9.842ms   | 13.195ms    | 15.013ms  | 260.275ms   | 623.743ms      | 2.310s     |
| snailtracer             | 26.023ms | 42.996ms | 68.329ms  | 64.806ms    | 88.245ms  | 2.393s      | 7.201s         | 8.693s     |
| ten-thousand-hashes     | 2.932ms  | 3.884ms  | 7.135ms   | 7.901ms     | 7.290ms   | 124.704ms   | 399.052ms      | 203.540ms  |

Benchmarked on 2026-03-07 with:

- `evmone` `0.19.0`
- `pyrevm` `0.3.7`
- `revm` `36.0.0`
- `mantle-revm` (`mantle-xyz/revm` `f13cb92`)
- `go-ethereum` `v1.17.1`
- `@ethereumjs/common`, `@ethereumjs/evm`, `@ethereumjs/util`, `@ethereumjs/vm` `10.1.1`
- `py-evm` `0.12.1b1` on CPython 3.13 and PyPy 3.10

This comprehensive rerun includes all current runners on `develop` and uses the result artifact at `outputs/results/2026-03-07-full-benchmark-develop-complete.evm-bench.results.json`.

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
