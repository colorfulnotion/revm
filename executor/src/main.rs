use std::fs::File;
use std::path::Path;
use std::sync::Arc;

use rsp_client_executor::{
    executor::{EthClientExecutor},
    io::EthClientExecutorInput,
};

pub fn main() {
    let file_path = Path::new("22185220_0xa00dab9400617f82f08394572d461e88ffb0d68d592da2dcd999eb11780560de_0xe4046a78df17bac62ac8be07754aa876b9e497ec157ca67ae8c65b9382e1f8aa.bin");
    let file = File::open(file_path).unwrap();

    let input: EthClientExecutorInput = bincode::deserialize_from(file).unwrap();
    println!("Deserialized input:\n{:#?}", input);

    
    let executor = EthClientExecutor::eth(
        Arc::new((&input.genesis).try_into().unwrap()),
        input.custom_beneficiary,
    );
    let header = executor.execute(input).expect("failed to execute client");
    let block_hash = header.hash_slow();

    // Commit the block hash.
    // sp1_zkvm::io::commit(&block_hash);
    println!("Block hash: {:?}", block_hash);

    println!("Execution completed successfully.");
}
