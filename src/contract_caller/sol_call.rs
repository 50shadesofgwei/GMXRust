use ethers::prelude::*;
use ethers::types::H160;
use dotenv::dotenv;
use std::env;
use std::sync::Arc;

// Create contract instances w/ abigen
abigen!{ 
    EXCHANGE_ROUTER, "/Users/jfeasby/GMX Rust/GMX_Rust/src/contract_caller/abis/exchange_router_abi.json";
    USDC_NATIVE, "/Users/jfeasby/GMX Rust/GMX_Rust/src/contract_caller/abis/usdc_arb_native_abi.json"
}

pub async fn sol_call() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // Directly unwrap the address parsing result
    let exchange_router_address_str: String = "0x7C68C7866A64FA2160F78EEaE12217FFbf871fa8".to_string();
    let exchange_router_address: H160 = exchange_router_address_str.parse()?;

    let usdc_native_address_str: String = "0xaf88d065e77c8cC2239327C5EDb3A432268e5831".to_string();
    let usdc_native_address: H160 = usdc_native_address_str.parse()?;

    // Initialise providers
    let rpc_url: String = env::var("PROVIDER_URL")?;
    let provider: Provider<Http> = Provider::<Http>::try_from(rpc_url.as_str())?;
    let provider: Arc<Provider<Http>> = Arc::new(provider);  // Wrap the provider in an Arc

    // Create contract instances
    let exchange_router_contract: EXCHANGE_ROUTER<_> = EXCHANGE_ROUTER::new(exchange_router_address, provider.clone());
    let usdc_native_contract: USDC_NATIVE<_> = USDC_NATIVE::new(usdc_native_address, provider.clone());

    let function_name: &str = "multicall";
    let function_params = ();
    let result: Address = contract.method(function_name, function_params)?.call().await?;
    let result_string: String = result.to_string();
    println!("{}", result_string);

    Ok(())
}