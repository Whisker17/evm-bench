## Benchmarks

evm-bench benchmarks are (typically) expensive Solidity contracts paired with configuration.

Benchmarks are built independently of any runner using `solc` running in Docker. The evm-bench framework picks up on benchmarks by scanning for `benchmark.evm-bench.json` files, which have [a schema](schema.json). That schema has more information on the structure of benchmark metadata file.

The benchmark suite now aims to cover several distinct EVM execution categories:

- storage-heavy workloads
- memory growth and return-data construction
- hash-heavy workloads
- event/log-heavy workloads
- deep and wide external call patterns
- contract creation with both `CREATE` and `CREATE2`
- application-style state transitions such as ERC20, ERC721, and vault accounting

### Developing a new benchmark

You want to first start off by creating a new Solidity contract. This can be whatever you want it to be, but presumably it will be expensive to run in an EVM. Then figure out the calldata you need to execute your benchmark. _Do not_ have your benchmark be in the constructor. The constructor is not benchmarked by runners. The runners will benchmark the time it takes to call the contract with the calldata you supply.

All you need now is a new `benchmark.evm-bench.json` file somewhere under this directory (since this is where the tool scans for benchmarks by default). Use the other benchmarks here as an example! Create a new folder and add resources under that folder. Note that if you plan to share resources among benchmarks (e.g. a shared Solidity library), make sure the benchmark metadata has the correct build context. See benchmarks under [`erc20`](erc20) for an example of this.

Keep new benchmarks deterministic and self-contained. Avoid dependencies on live chain data, timestamps, randomness, external addresses, or protocol deployments that are not compiled as part of the benchmark build context. Put the measured workload inside `Benchmark()` itself. Constructors may set up static state for later reads, but constructor work is not part of the benchmarked call.

Once you have your benchmark, it's time to test! Consider running the evm-bench framework with a single runner ([`revm`](../runners/revm) is the most stable in my experience) against your new benchmark to start, then move on to running it on all runners. It would look something like `RUST_LOG=info cargo run -- --runners revm --benchmarks <my_new_benchmark_name>`, if you need more information about logs you can tweak `RUST_LOG`.
