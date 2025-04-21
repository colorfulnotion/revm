// transfer example
use revm::database::{BenchmarkDB, BENCH_CALLER, BENCH_TARGET};
use revm::{
    bytecode::Bytecode,
    primitives::{TxKind, U256},
    Context, ExecuteEvm, MainBuilder, MainContext,
};

fn main() {
    let mut evm = Context::mainnet()
        .with_db(BenchmarkDB::new_bytecode(Bytecode::new()))
        .modify_tx_chained(|tx| {
            // Execution globals block hash/gas_limit/coinbase/timestamp..
            tx.caller = BENCH_CALLER;
            tx.kind = TxKind::Call(BENCH_TARGET);
            tx.value = U256::from(911);
        })
        .build_mainnet();

    let _res_and_state = evm.replay();

    println!("replay: _res_and_state: {:?}", _res_and_state);
}