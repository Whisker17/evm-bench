# evm-bench

[![CI](https://github.com/ziyadedher/evm-bench/actions/workflows/ci.yml/badge.svg)](https://github.com/ziyadedher/evm-bench/actions/workflows/ci.yml)

**evm-bench is a suite of Ethereum Virtual Machine (EVM) stress tests and benchmarks.**

evm-bench makes it easy to compare EVM performance in a scalable, standardized, and portable way.

|                         | evmone   | mantle-revm | pyrevm   | revm     | geth      | py-evm.pypy | ethereumjs | py-evm.cpython |
| ----------------------- | -------- | ----------- | -------- | -------- | --------- | ----------- | ---------- | -------------- |
| **sum**                 | 44.122ms | 64.627ms    | 71.714ms | 74.808ms | 146.950ms | 4.331s      | 13.372s    | 19.574s        |
| **relative**            | 1.000x   | 1.465x      | 1.625x   | 1.695x   | 3.331x    | 98.163x     | 303.072x   | 443.636x       |
| calls.deep-call-chain   | 55.083µs | 33.181µs    | 156.208µs | 157.277µs | 70.333µs  | 35.146ms    | 6.225ms    | 9.420ms        |
| calls.fanout-aggregate  | 148.166µs | 224.541µs   | 191.583µs | 371.124µs | 848.333µs | 74.955ms    | 15.968ms   | 44.491ms       |
| create.create-many      | 158.778µs | 109.291µs   | 177.458µs | 108.222µs | 456.333µs | 89.156ms    | 37.811ms   | 25.071ms       |
| create2.create2-many    | 136.000µs | 218.666µs   | 173.249µs | 229.153µs | 201.333µs | 76.546ms    | 28.867ms   | 28.407ms       |
| erc20.approval-transfer | 4.435ms  | 4.983ms     | 6.558ms  | 4.338ms  | 11.780ms  | 337.026ms   | 976.722ms  | 883.010ms      |
| erc20.mint              | 2.602ms  | 3.968ms     | 5.445ms  | 4.067ms  | 10.033ms  | 229.530ms   | 1.916s     | 982.955ms      |
| erc20.transfer          | 5.120ms  | 6.561ms     | 8.488ms  | 8.940ms  | 14.835ms  | 278.327ms   | 2.060s     | 1.316s         |
| erc721.mint-transfer    | 321.139µs | 1.077ms     | 522.902µs | 772.763µs | 921.666µs | 167.128ms   | 189.689ms  | 81.210ms       |
| hash.merkle-proof-batch | 186.092µs | 205.741µs   | 258.841µs | 724.991µs | 518.000µs | 81.779ms    | 30.500ms   | 55.343ms       |
| logging.log-burst       | 119.675µs | 141.341µs   | 175.674µs | 325.725µs | 645.800µs | 87.590ms    | 13.472ms   | 49.925ms       |
| memory.expand-copy      | 244.033µs | 185.491µs   | 270.158µs | 207.066µs | 534.800µs | 46.164ms    | 9.797ms    | 20.506ms       |
| snailtracer             | 26.958ms | 42.720ms    | 44.225ms | 50.416ms | 96.171ms  | 2.541s      | 7.564s     | 15.127s        |
| storage.read-many       | 333.666µs | 459.275µs   | 663.224µs | 367.449µs | 903.800µs | 24.416ms    | 62.511ms   | 58.496ms       |
| storage.write-many      | 187.458µs | 260.208µs   | 281.725µs | 423.758µs | 1.001ms   | 42.777ms    | 155.657ms  | 40.436ms       |
| ten-thousand-hashes     | 3.017ms  | 3.319ms     | 3.950ms  | 3.152ms  | 7.636ms   | 132.004ms   | 202.174ms  | 814.605ms      |
| vault.deposit-withdraw  | 100.569µs | 161.306µs   | 178.027µs | 207.014µs | 394.000µs | 87.648ms    | 102.225ms  | 36.567ms       |

Benchmarked on 2026-03-07 with:

- `evmone` `0.19.0`
- `mantle-revm` (`mantle-xyz/revm` `f13cb92c721681af99b4c5300d15351d2ab16d75`)
- `pyrevm` `0.3.7`
- `revm` `36.0.0`
- `go-ethereum` `v1.17.1`
- `@ethereumjs/common`, `@ethereumjs/evm`, `@ethereumjs/util`, `@ethereumjs/vm` `10.1.1`
- `py-evm` `0.12.1b1` (`cpython`, `pypy`)

This rerun used the full 16 benchmark × 8 runner result artifact at `outputs/results/2026-03-07-full-runner-matrix`.

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
