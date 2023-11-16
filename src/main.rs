pub mod contract_caller;

use alloy_primitives::Address;
use contract_caller::connect_provider::connect_provider;
use dotenv::dotenv;
use ethers::{utils::{Anvil, self}, signers::LocalWallet};

#[tokio::main]
async fn main() {
    dotenv().ok();
    match connect_provider().await {
        Ok(_) => println!("Successfully connected to the provider."),
        Err(e) => println!("Error connecting to the provider: {}", e),
    }

    let contract_address: Result<Address, alloy_primitives::hex::FromHexError> = "0x7C68C7866A64FA2160F78EEaE12217FFbf871fa8".parse::<Address>(); // ExchangeRouter adr.
    let anvil: utils::AnvilInstance = Anvil::new().spawn();
    let wallet: LocalWallet = anvil.keys()[0].clone().into();

    
}