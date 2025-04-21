//! Custom opcodes example
#![cfg_attr(not(test), warn(unused_crate_dependencies))]

use revm::{
    context::Context,
    database::CacheDB,
    database_interface::EmptyDB,
    handler::EvmTr,
    context_interface::result::{ExecutionResult, Output},
    primitives::{hex, Bytes, TxKind, bytes},
    ExecuteCommitEvm, ExecuteEvm, MainBuilder, MainContext,
};


// add_example.sol:
/*
    // SPDX-License-Identifier: MIT
    pragma solidity ^0.8.0;

    contract Add123And321 {
        function compute() external pure returns (uint256) {
            return 123 + 321;
        }
    }
*/

const BYTES: &str = include_str!("add_example.hex");

pub fn main() {
    // deploy contract with init code and runtime bytecode
    let bytecode = Bytes::from(hex::decode(BYTES).unwrap());
    let ctx = Context::mainnet()
        .modify_tx_chained(|tx| {
            tx.kind = TxKind::Create;
            tx.data = bytecode.clone();
        })
        .with_db(CacheDB::<EmptyDB>::default());

    let mut evm = ctx.build_mainnet();

    let ref_tx = evm.replay_commit().unwrap();
    let ExecutionResult::Success {
        output: Output::Create(_, Some(address)),
        ..
    } = ref_tx
    else {
        panic!("Failed to create contract")
    };

    println!("Created contract at {address}");
    // call contract with custom opcode
    evm.ctx().modify_tx(|tx| {
        tx.kind = TxKind::Call(address);
        tx.data = bytes!("1a43c338");
        tx.nonce += 1;
    });


    let ras = match evm.replay() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("EVM Failed: {e}");
            std::process::exit(1);
        }
    };
    

    let output_slice = ras.result.output().unwrap();
    println!("output: {output_slice:#?}");

}
