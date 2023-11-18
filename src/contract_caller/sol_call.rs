use ethers::prelude::*;
use ethers::types::H160;
use dotenv::dotenv;
use std::env;
use std::sync::Arc;

// Create contract instances from abis w/ abigen
abigen!{ 
    EXCHANGE_ROUTER, "/Users/jfeasby/GMX Rust/GMX_Rust/src/contract_caller/abis/exchange_router_abi.json";
    USDC_NATIVE, "/Users/jfeasby/GMX Rust/GMX_Rust/src/contract_caller/abis/usdc_arb_native_abi.json";
    VAULT, "/Users/jfeasby/GMX Rust/GMX_Rust/src/contract_caller/abis/vault_abi.json";
    WETH, "/Users/jfeasby/GMX Rust/GMX_Rust/src/contract_caller/abis/weth_abi.json";
}

pub async fn sol_call() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // ---------------------------------------------------------
    //                      Initialisation
    // ---------------------------------------------------------

    // Some necessary preamble for contract instances
    // Exchange Router
    let exchange_router_address_str: String = "0x7C68C7866A64FA2160F78EEaE12217FFbf871fa8".to_string();
    let exchange_router_address: H160 = exchange_router_address_str.parse()?;

    // USDC (Native)
    let usdc_native_address_str: String = "0xaf88d065e77c8cC2239327C5EDb3A432268e5831".to_string();
    let usdc_native_address: H160 = usdc_native_address_str.parse()?;

    // Vault
    let vault_address_str: String = "0x489ee077994B6658eAfA855C308275EAd8097C4A".to_string();
    let vault_address: H160 = vault_address_str.parse()?;

    // WETH (WNT)
    let weth_address_str: String = "0x82aF49447D8a07e3bd95BD0d56f35241523fBab1".to_string();
    let weth_address: H160 = weth_address_str.parse()?;


    // Initialise providers
    let rpc_url: String = env::var("PROVIDER_URL")?;
    let provider: Provider<Http> = Provider::<Http>::try_from(rpc_url.as_str())?;
    let provider: Arc<Provider<Http>> = Arc::new(provider);  // Wrap the provider in an Arc


    // Create contract instances
    let exchange_router_contract: EXCHANGE_ROUTER<_> = EXCHANGE_ROUTER::new(exchange_router_address, provider.clone());
    let usdc_native_contract: USDC_NATIVE<_> = USDC_NATIVE::new(usdc_native_address, provider.clone());
    let vault_contract: VAULT<_> = VAULT::new(vault_address, provider.clone());

    // Example vars
    // TODO: make these arguments passed to call builder func.
    let wnt_amount: &str = "amount_in_wei_here";
    let usdc_amount: &str = "10000000";
    let deposit_params: &str = "/* create your deposit params here */";



    // ---------------------------------------------------------
    //                    Multicall Builder
    // --------------------------------------------------------- 

    let approve_tx = usdc_native_contract
    .method::<_, ()>("approve", (router_address, usdc_amount))?
    .send()
    .await?;

    let send_wnt_data = exchange_router_contract
        .encode_function_data("sendWnt", (deposit_vault_address, wnt_amount))?;


    let function_name: &str = "multicall";
    let function_params = ();
    let result: Address = contract.method(function_name, function_params)?.call().await?;
    let result_string: String = result.to_string();
    println!("{}", result_string);

    Ok(())
}