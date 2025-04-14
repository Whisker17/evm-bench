# evm-bench

[![CI](https://github.com/ziyadedher/evm-bench/actions/workflows/ci.yml/badge.svg)](https://github.com/ziyadedher/evm-bench/actions/workflows/ci.yml)

**evm-bench is a suite of Ethereum Virtual Machine (EVM) stress tests and benchmarks.**

evm-bench makes it easy to compare EVM performance in a scalable, standardized, and portable way.

|                         | revm     | pyrevm   | geth      | ethereumjs | py-evm.pypy | py-evm.cpython | evmone |
| ----------------------- | -------- | -------- | --------- | ---------- | ----------- | -------------- | ------ |
| **sum**                 | 58.966ms | 79.591ms | 149.229ms | 6.392s     | N/A         | N/A            | N/A    |
| **relative**            | 1.000x   | 1.350x   | 2.531x    | 108.406x   | N/A         | N/A            | N/A    |
| erc20.approval-transfer | 5.952ms  | 7.818ms  | 12.693ms  | 462.744ms  |             |                |        |
| erc20.mint              | 3.560ms  | 6.333ms  | 11.060ms  | 467.701ms  |             |                |        |
| erc20.transfer          | 6.877ms  | 9.899ms  | 16.548ms  | 650.097ms  |             |                |        |
| snailtracer             | 40.542ms | 52.818ms | 102.535ms | 4.525s     |             |                |        |
| ten-thousand-hashes     | 2.036ms  | 2.723ms  | 6.394ms   | 286.452ms  |             |                |        |

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
