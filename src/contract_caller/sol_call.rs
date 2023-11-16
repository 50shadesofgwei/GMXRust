use alloy_sol_types::SolCall;
use alloy_primitives::Address;
use ethers::prelude::*;
use ethers::types::H160;
use dotenv::dotenv;
use std::env;
use std::sync::Arc;

use crate::contract_caller::get_abi::get_abi;

abigen!{IERC721, "/Users/jfeasby/GMX Rust/GMX_Rust/src/contract_caller/abis/exchange_router_abi.json"}

pub async fn sol_call() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // Directly unwrap the address parsing result
    let contract_address: H160 = "0x7C68C7866A64FA2160F78EEaE12217FFbf871fa8".parse()?;

    let abi: alloy_json_abi::JsonAbi = get_abi()?;
    let rpc_url: String = env::var("PROVIDER_URL")?;

    let provider: Provider<Http> = Provider::<Http>::try_from(rpc_url.as_str())?;
    let provider = Arc::new(provider);  // Wrap the provider in an Arc
    let contract: IERC721<_> = IERC721::new(contract_address, provider.clone());

    Ok(())
}