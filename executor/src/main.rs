use std::fs::File;
use std::path::Path;
use std::sync::Arc;

use rsp_client_executor::{
    executor::EthClientExecutor,
    io::EthClientExecutorInput,
};

use alloy_primitives::FixedBytes;

fn parse_filename(path: &Path) -> (String, String) {
    let filename = path.file_name().unwrap().to_str().unwrap();
    let parts: Vec<&str> = filename.split('_').collect();
    assert!(parts.len() >= 3, "unexpected filename format");

    let block_hash = parts[1].to_string();

    let state_root = parts[2]
        .strip_suffix(".bin")
        .unwrap_or(parts[2])
        .to_string();

    (block_hash, state_root)
}

pub fn main() {
    let file_path = Path::new("22185220_0xa00dab9400617f82f08394572d461e88ffb0d68d592da2dcd999eb11780560de_0xe4046a78df17bac62ac8be07754aa876b9e497ec157ca67ae8c65b9382e1f8aa.bin");
    let (expected_block_hash, expected_state_root) = parse_filename(file_path);

    let file = File::open(file_path).unwrap();
    let mut input: EthClientExecutorInput = bincode::deserialize_from(file).unwrap();
    // println!("Deserialized input:\n{:#?}", input);

    let executor = EthClientExecutor::eth(
        Arc::new((&input.genesis).try_into().unwrap()),
        input.custom_beneficiary,
    );

    match executor.execute(input.clone()) {
        Ok(header) => {
            let block_hash = format!("{:?}", header.hash_slow());
            let state_root = format!("{:?}", header.state_root);
    
            println!("Expected block hash: {}", expected_block_hash);
            println!("Actual   block hash: {}", block_hash);
            println!("Expected state root: {}", expected_state_root);
            println!("Actual   state root: {}", state_root);
    
            if block_hash.contains(&expected_block_hash) && state_root.contains(&expected_state_root) {
                println!("Execution result: SUCCESS");
            } else {
                println!("Execution result: FAILED");
            }
        }
        Err(e) => {
            println!("Execution failed: {:?}", e);
        }
        Err(_) => {
            println!("Execution panicked");
        }
    }

    // Demonstrate block validation failure
    let zero = FixedBytes::<32>::ZERO;
    input.current_block.header.parent_hash = zero;

    match executor.execute(input) {
        Ok(header) => {
            let block_hash = format!("{:?}", header.hash_slow());
            let state_root = format!("{:?}", header.state_root);
    
            println!("Expected block hash: {}", expected_block_hash);
            println!("Actual   block hash: {}", block_hash);
            println!("Expected state root: {}", expected_state_root);
            println!("Actual   state root: {}", state_root);
    
            if block_hash.contains(&expected_block_hash) && state_root.contains(&expected_state_root) {
                println!("Execution result: SUCCESS");
            } else {
                println!("Execution result: FAILED");
            }
        }
        Err(e) => {
            println!("Execution failed: {:?}", e);
        }
        Err(_) => {
            println!("Execution panicked");
        }
    }
    

}


