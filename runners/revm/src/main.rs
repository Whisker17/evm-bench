use clap::Parser;
use revm::{
    context::{result::ExecutionResult, Context, TxEnv},
    database::{CacheDB, EmptyDB},
    primitives::{address, hardfork::SpecId, hex, Bytes, TxKind},
    ExecuteCommitEvm, MainBuilder, MainContext,
};
use std::{fs, path::PathBuf, time::Instant};

/// Revolutionary EVM (revm) runner interface
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the hex contract code to deploy and run
    #[arg(long)]
    contract_code_path: PathBuf,

    /// Hex of calldata to use when calling the contract
    #[arg(long)]
    calldata: String,

    /// Number of times to run the benchmark
    #[arg(short, long, default_value_t = 1)]
    num_runs: u8,
}

fn main() {
    let args = Args::parse();

    let creation_code_hex =
        fs::read_to_string(args.contract_code_path).expect("failed to read code path");
    let creation_code: Bytes =
        hex::decode(creation_code_hex.trim()).expect("could not hex decode contract code").into();
    let calldata: Bytes =
        hex::decode(args.calldata.trim()).expect("could not hex decode calldata").into();

    let caller = address!("1000000000000000000000000000000000000001");
    let spec_id = SpecId::OSAKA;

    let ctx = Context::mainnet()
        .modify_cfg_chained(|cfg| {
            cfg.set_spec_and_mainnet_gas_params(spec_id);
            cfg.tx_gas_limit_cap = Some(u64::MAX);
        })
        .with_db(CacheDB::<EmptyDB>::default());
    let mut evm = ctx.build_mainnet();

    let deploy_result = evm
        .transact_commit(
            TxEnv {
                caller,
                kind: TxKind::Create,
                data: creation_code,
                nonce: 0,
                ..TxEnv::new_bench()
            },
        )
        .expect("failed to deploy contract");

    let created_address = match deploy_result {
        ExecutionResult::Success { .. } => deploy_result
            .created_address()
            .expect("missing created address after successful deployment"),
        _ => panic!("failed creating contract: {deploy_result:#?}"),
    };

    for run_index in 0..args.num_runs {
        let timer = Instant::now();
        let call_result = evm
            .transact_commit(
                TxEnv {
                    caller,
                    kind: TxKind::Call(created_address),
                    data: calldata.clone(),
                    nonce: 1 + u64::from(run_index),
                    ..TxEnv::new_bench()
                },
            )
            .expect("failed to execute call transaction");
        let dur = timer.elapsed();

        assert!(
            matches!(call_result, ExecutionResult::Success { .. }),
            "transaction failed: {call_result:#?}"
        );

        println!("{}", dur.as_secs_f64() * 1000.0);
    }
}
