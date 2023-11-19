use ethers::prelude::*;
use ethers::types::H160;
use dotenv::dotenv;
use std::env;
use std::sync::Arc;

use crate::contract_caller::utils::structs::{AddressesStruct, NumbersStruct};

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
    let exchange_router_address_str: String = "0xFE98518C9c8F1c5a216E999816c2dE3199f295D2".to_string();
    let exchange_router_address: H160 = exchange_router_address_str.parse()?;

    // USDC (Native)
    let usdc_native_address_str: String = "0x04FC936a15352a1b15b3B9c56EA002051e3DB3e5".to_string();
    let usdc_native_address: H160 = usdc_native_address_str.parse()?;

    // Vault
    let vault_address_str: String = "0x82aFd2590814a7Ce3d7ea6b63F80481F8b227bA9".to_string();
    let vault_address: H160 = vault_address_str.parse()?;

    // // WETH (WNT)
    // let weth_address_str: String = "0x82aF49447D8a07e3bd95BD0d56f35241523fBab1".to_string();
    // let weth_address: H160 = weth_address_str.parse()?;


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
    let weth_amount: &str = "10000000000000000";
    let usdc_amount: &str = "10000000";
    let deposit_params: &str = "/* create deposit params here */";



    // ---------------------------------------------------------
    // ------------------------------------------------------
    //                   Multicall Builder
    // ------------------------------------------------------
    // ---------------------------------------------------------

    // ----------------------------------
    //            Tx1: Approve
    // ----------------------------------

    let approve_usdc = usdc_native_contract
        .encode_function_data("approve", (router_address, usdc_amount))?;

    // ----------------------------------
    //         Tx2: Vault Deposit
    // ----------------------------------

    let deposit_colleratal = exchange_router_contract
        .encode_function_data("approve", (router_address, usdc_amount))?;

    // ----------------------------------
    //         Tx3: Create Order
    // ----------------------------------

    // Structure the input for 'createOrder'
let addresses: AddressesStruct = AddressesStruct {
    receiver: "0x1f13a5dc44911ebd98ea1b55ab5b7b2a99acca14".parse()?,
    callback_contract: "0x0000000000000000000000000000000000000000".parse()?,
    ui_fee_receiver: "0x0000000000000000000000000000000000000000".parse()?,
    market: "0x70d95587d40a2caf56bd97485ab3eec10bee6336".parse()?,
    initial_collateral_token: "0x82af49447d8a07e3bd95bd0d56f35241523fbab1".parse()?,
    swap_path: vec!["0x6853ea96ff216fab11d2d930ce3c508556a4bdc4".parse()?],
};

let numbers = NumbersStruct {
    size_delta_usd: U256::from_dec_str("2431245426638617490489280000000000")?,
    initial_collateral_delta_amount: U256::from(0),
    trigger_price: U256::from(0),
    acceptable_price: U256::from_dec_str("1975900891694612")?,
    execution_fee: U256::from_dec_str("1292500000000000")?,
    callback_gas_limit: U256::from(0),
    min_output_amount: U256::from(0),
};

let order_type: i32 = 2;
let decrease_position_swap_type: i32 = 0;
let is_long: bool = true;
let should_unwrap_native_token: bool = true;
let referral_code: [u8; 32] = [0u8; 32]; // Assuming 32-byte zero array

// Encode 'createOrder' call
let create_order_payload = exchange_router_contract.method::<_, Bytes>("createOrder", (addresses, numbers, order_type, decrease_position_swap_type, is_long, should_unwrap_native_token, referral_code))?.calldata().await?;


    let function_name: &str = "multicall";
    let function_params = ();
    let result = exchange_router_contract.method(function_name, function_params)?.call().await?;
    let result_string: String = result.to_string();
    println!("{}", result_string);

    Ok(())
}