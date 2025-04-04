use std::{
    fs::File,
    path::Path,
};

use eyre::Result;
use bincode::{DefaultOptions, Options};
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct ClientExecutorInput {
    current_block: BlockData,
    parent_state: StateData,
}

#[derive(Serialize, Deserialize, Debug)]
struct BlockData {
    number: u64,
}

#[derive(Serialize, Deserialize, Debug)]
struct StateData {
    state_root: String,
}

/// Loads the client input from the given bin file using a custom bincode configuration.
/// The capacity is set to 512 MB here.
fn load_client_input<P>(path: &Path) -> Result<P>
where
    P: for<'de> Deserialize<'de>,
{
    let file = File::open(path)?;
    // Increase capacity to 512 MB
    let config = DefaultOptions::new().with_limit(512 * 1024 * 1024);
    let input = config.deserialize_from(file)?;
    Ok(input)
}

#[tokio::main]
async fn main() -> Result<()> {
    // Replace with the actual path to your bin file.
    let input_path = Path::new("22185220_0xa00dab9400617f82f08394572d461e88ffb0d68d592da2dcd999eb11780560de_0xe4046a78df17bac62ac8be07754aa876b9e497ec157ca67ae8c65b9382e1f8aa.bin");

    // Load the client input from the bin file.
    let client_input: ClientExecutorInput = load_client_input(input_path)?;
    println!("Loaded client input: {:#?}", client_input);

    // Convert the deserialized structure to pretty-printed JSON.
    let json_output = serde_json::to_string_pretty(&client_input)?;
    println!("JSON version of the bin file:\n{}", json_output);

    Ok(())
}
