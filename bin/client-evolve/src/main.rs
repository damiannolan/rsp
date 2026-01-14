#![no_main]
sp1_zkvm::entrypoint!(main);

use rsp_client_executor::{
    executor::{EvolveClientExecutor, DESERIALZE_INPUTS},
    io::{CommittedHeader, EvolveClientExecutorInput},
    utils::profile_report,
};
use std::sync::Arc;

pub fn main() {
    // Read the input.
    let input = profile_report!(DESERIALZE_INPUTS, {
        let input = sp1_zkvm::io::read_vec();
        bincode::deserialize::<EvolveClientExecutorInput>(&input).unwrap()
    });

    // Execute the block with evolve configuration.
    let chain_spec = Arc::new((&input.genesis).try_into().unwrap());
    let executor = EvolveClientExecutor::evolve(
        chain_spec,
        input.custom_beneficiary,
        &input.genesis,
    );
    let header = executor.execute(input).expect("failed to execute client");

    // Commit the block hash.
    sp1_zkvm::io::commit::<CommittedHeader>(&header.into());
}
