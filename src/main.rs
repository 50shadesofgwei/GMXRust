pub mod contract_caller;

use alloy_primitives::Address;
use contract_caller::sol_call::sol_call;
use dotenv::dotenv;
use ethers::{utils::{Anvil, self}, signers::LocalWallet};

#[tokio::main]
async fn main() {
    dotenv().ok();
    match sol_call().await {
        Ok(_) => println!("Successfully read contract."),
        Err(e) => println!("Error reading contract: {}", e),
    }
}