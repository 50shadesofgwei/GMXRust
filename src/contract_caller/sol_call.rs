use ethers::prelude::*;
use ethers::types::H160;
use dotenv::dotenv;
use std::env;
use std::sync::Arc;

abigen!{IERC721, "/Users/jfeasby/GMX Rust/GMX_Rust/src/contract_caller/abis/exchange_router_abi.json"}

pub async fn sol_call() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // Directly unwrap the address parsing result
    let contract_address_str: String = "0x7C68C7866A64FA2160F78EEaE12217FFbf871fa8".to_string();
    let contract_address: H160 = contract_address_str.parse()?;

    let rpc_url: String = env::var("PROVIDER_URL")?;

    let provider: Provider<Http> = Provider::<Http>::try_from(rpc_url.as_str())?;
    let provider: Arc<Provider<Http>> = Arc::new(provider);  // Wrap the provider in an Arc
    let contract: IERC721<_> = IERC721::new(contract_address, provider.clone());

    let function_name: &str = "multicall";
    let function_params = ();
    let result: Address = contract.method(function_name, function_params)?.call().await?;
    let result_string: String = result.to_string();
    println!("{}", result_string);

    Ok(())
}