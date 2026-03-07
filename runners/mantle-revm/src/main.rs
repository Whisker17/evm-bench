#![cfg_attr(not(test), warn(unused_crate_dependencies))]

use anyhow::{bail, Context as _, Result};
use clap::Parser;
use revm::{
    context::{Context as RevmContext, TxEnv},
    database::CacheDB,
    database_interface::EmptyDB,
    primitives::{hex, Bytes, TxKind},
    ExecuteCommitEvm, MainBuilder, MainContext,
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

    let mut run_times = Vec::with_capacity(num_runs as usize);

    for run_index in 0..num_runs {
        let transaction = TxEnv::builder_for_bench()
            .nonce(run_index + 1)
            .kind(TxKind::Call(created_address))
            .data(calldata.clone())
            .build_fill();
        let started_at = Instant::now();
        let result = evm.transact_commit(transaction).context("mantle-revm failed to execute call transaction")?;
        let elapsed = started_at.elapsed();

        if !result.is_success() {
            bail!("mantle-revm call transaction did not succeed: {result:?}");
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

    #[test]
    fn deploys_and_runs_stateful_contract() {
        let creation_code =
            parse_hex_bytes("6006600c60003960066000f3600160005500").unwrap();
        let run_times = run_benchmark_from_bytes(creation_code, Bytes::default(), 1).unwrap();

        assert_eq!(run_times.len(), 1);
        assert!(run_times[0].is_finite());
        assert!(run_times[0] >= 0.0);
    }
}
