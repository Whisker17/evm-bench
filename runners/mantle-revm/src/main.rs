#![cfg_attr(not(test), warn(unused_crate_dependencies))]

use anyhow::{bail, Context as _, Result};
use clap::Parser;
use revm::{
    context::{Context as RevmContext, TxEnv},
    context_interface::ContextTr,
    database::CacheDB,
    database_interface::{BENCH_CALLER, EmptyDB},
    interpreter::{
        host::DummyHost,
        instruction_table,
        interpreter::{EthInterpreter, ExtBytecode},
        CallInput, InputsImpl, Interpreter, SharedMemory,
    },
    primitives::{hardfork::SpecId, hex, Address, Bytes, U256},
    state::Bytecode,
    Database, ExecuteCommitEvm, MainBuilder, MainContext,
};
use std::{
    fs,
    path::{Path, PathBuf},
    time::Instant,
};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    contract_code_path: PathBuf,

    #[arg(long)]
    calldata: String,

    #[arg(short, long, default_value_t = 1)]
    num_runs: u64,
}

fn parse_hex_bytes(input: &str) -> Result<Bytes> {
    let trimmed = input.trim();
    let trimmed = trimmed
        .strip_prefix("0x")
        .or_else(|| trimmed.strip_prefix("0X"))
        .unwrap_or(trimmed);
    Ok(hex::decode(trimmed)?.into())
}

fn load_contract_code(path: &Path) -> Result<Bytes> {
    let raw = fs::read_to_string(path)
        .with_context(|| format!("failed to read contract bytecode from {}", path.display()))?;
    parse_hex_bytes(&raw)
}

fn run_benchmark_from_bytes(creation_code: Bytes, calldata: Bytes, num_runs: u64) -> Result<Vec<f64>> {
    let (contract_address, runtime_bytecode) = deploy_contract(creation_code)?;
    benchmark_runtime(contract_address, runtime_bytecode, calldata, num_runs)
}

fn deploy_contract(creation_code: Bytes) -> Result<(Address, Bytecode)> {
    let mut evm = RevmContext::mainnet()
        .with_db(CacheDB::<EmptyDB>::default())
        .build_mainnet();

    let deployment = TxEnv::builder_for_bench()
        .create()
        .data(creation_code)
        .build_fill();

    let deployment_result = evm
        .transact_commit(deployment)
        .context("mantle-revm failed to execute deployment transaction")?;

    if !deployment_result.is_success() {
        bail!("mantle-revm deployment did not succeed: {deployment_result:?}");
    }

    let created_address = deployment_result
        .created_address()
        .context("mantle-revm deployment did not return a created address")?;

    let runtime_bytecode = evm
        .db_mut()
        .basic(created_address)
        .context("mantle-revm could not load created account from database")?
        .context("mantle-revm created account missing from database")?
        .code
        .context("mantle-revm created account missing runtime bytecode")?;

    Ok((created_address, runtime_bytecode))
}

fn benchmark_runtime(
    contract_address: Address,
    runtime_bytecode: Bytecode,
    calldata: Bytes,
    num_runs: u64,
) -> Result<Vec<f64>> {
    let table = instruction_table::<EthInterpreter, DummyHost>();
    let mut host = DummyHost;
    let mut run_times = Vec::with_capacity(num_runs as usize);

    for _ in 0..num_runs {
        let mut interpreter = Interpreter::<EthInterpreter>::new(
            SharedMemory::new(),
            ExtBytecode::new(runtime_bytecode.clone()),
            InputsImpl {
                target_address: contract_address,
                bytecode_address: Some(contract_address),
                caller_address: BENCH_CALLER,
                input: CallInput::Bytes(calldata.clone()),
                call_value: U256::ZERO,
            },
            false,
            SpecId::default(),
            u64::MAX,
        );

        let started_at = Instant::now();
        let action = interpreter.run_plain(&table, &mut host);
        let elapsed = started_at.elapsed();

        if !action.is_return() {
            bail!("mantle-revm interpreter returned unexpected action: {action:?}");
        }

        let result = action
            .instruction_result()
            .context("mantle-revm interpreter returned without an instruction result")?;

        if !result.is_ok() {
            bail!("mantle-revm interpreter failed with {result:?}");
        }

        run_times.push(elapsed.as_secs_f64() * 1000.0);
    }

    Ok(run_times)
}

fn main() -> Result<()> {
    let args = Args::parse();
    let creation_code = load_contract_code(&args.contract_code_path)?;
    let calldata = parse_hex_bytes(&args.calldata)?;
    for millis in run_benchmark_from_bytes(creation_code, calldata, args.num_runs)? {
        println!("{millis}");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn parses_hex_with_optional_prefix_and_whitespace() {
        let bytes = parse_hex_bytes("  0x00ff  ").unwrap();
        assert_eq!(bytes.as_ref(), &[0x00, 0xff]);
    }

    #[test]
    fn loads_creation_code_from_file() {
        let file = NamedTempFile::new().unwrap();
        std::fs::write(file.path(), "6001600c60003960016000f300\n").unwrap();

        let bytes = load_contract_code(file.path()).unwrap();

        assert_eq!(bytes.as_ref(), &[0x60, 0x01, 0x60, 0x0c, 0x60, 0x00, 0x39, 0x60, 0x01, 0x60, 0x00, 0xf3, 0x00]);
    }

    #[test]
    fn deploys_and_runs_minimal_contract() {
        let creation_code = parse_hex_bytes("6001600c60003960016000f300").unwrap();
        let run_times = run_benchmark_from_bytes(creation_code, Bytes::default(), 1).unwrap();

        assert_eq!(run_times.len(), 1);
        assert!(run_times[0].is_finite());
        assert!(run_times[0] >= 0.0);
    }
}
