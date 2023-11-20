use alloy_sol_types::abi::token;
use ethers::prelude::*;
use ethers::types::{H160, Address, U256};
use dotenv::dotenv;
use std::env;
use std::sync::Arc;

use crate::contract_caller::sol_call::exchange_router::{CreateOrderParamsAddresses, CreateOrderParamsNumbers};

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
    let exchange_router_address: Address = exchange_router_address_str.parse()?;

    // USDC (Native)
    let usdc_native_address_str: String = "0xaf88d065e77c8cC2239327C5EDb3A432268e5831".to_string();
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
    let usdc_native_contract: USDC_NATIVE<Provider<Http>> = USDC_NATIVE::new(usdc_native_address, provider.clone());
    let vault_contract: VAULT<_> = VAULT::new(vault_address, provider.clone());

    // Example vars
    // TODO: make these arguments passed to call builder func.
    let weth_amount: &str = "10000000000000000";
    let usdc_amount_str: &str = "10000000";
    let usdc_amount: U256 = U256::from_dec_str(usdc_amount_str)
        .expect("Invalid number format for USDC amount");



    // ---------------------------------------------------------
    // ------------------------------------------------------
    //                   Multicall Builder
    // ------------------------------------------------------
    // ---------------------------------------------------------

    // ----------------------------------
    //            Tx1: Approve
    // ----------------------------------

    let tx1_builder = usdc_native_contract.approve(exchange_router_address, usdc_amount);
    let tx1_bytes: Bytes = tx1_builder.calldata().unwrap();
    
    // ----------------------------------
    //         Tx2: Vault Deposit
    // ----------------------------------

    let token_reciever_address_str: String = "0x31ef83a530fde1b38ee9a18093a333d8bbbc40d5".to_string();
    let token_reciever_address: H160 = token_reciever_address_str.parse()?;
    let tx2_builder = exchange_router_contract.send_tokens(usdc_native_address, token_reciever_address, usdc_amount);
    let tx2_bytes: Bytes = tx2_builder.calldata().unwrap();

    // ----------------------------------
    //         Tx3: Create Order
    // ----------------------------------

    // Structure the input for 'createOrder'
    let addresses: CreateOrderParamsAddresses = CreateOrderParamsAddresses {
        receiver: "0xa6d1feda6fc70680816ef6a23faf5e454e2f9c09".parse()?,
        callback_contract: "0x0000000000000000000000000000000000000000".parse()?,
        ui_fee_receiver: "0x0000000000000000000000000000000000000000".parse()?,
        market: "0x70d95587d40a2caf56bd97485ab3eec10bee6336".parse()?,
        initial_collateral_token: "0x82af49447d8a07e3bd95bd0d56f35241523fbab1".parse()?,
        swap_path: vec!["0x6853ea96ff216fab11d2d930ce3c508556a4bdc4".parse()?],
    };

    let numbers: CreateOrderParamsNumbers = CreateOrderParamsNumbers {
        size_delta_usd: U256::from_dec_str("2431245426638617490489280000000000")?,
        initial_collateral_delta_amount: U256::from(0),
        trigger_price: U256::from(0),
        acceptable_price: U256::from_dec_str("1975900891694612")?,
        execution_fee: U256::from_dec_str("1292500000000000")?,
        callback_gas_limit: U256::from(0),
        min_output_amount: U256::from(0),
    };

    let order_type: u8 =  u8::from(2);
    let decrease_position_swap_type: u8 = u8::from(0);
    let is_long: bool = true;
    let should_unwrap_native_token: bool = true;
    let referral_code: [u8; 32] = [0u8; 32]; // Assuming 32-byte zero array i.e. no refferal code

    let create_order_object: CreateOrderParams = CreateOrderParams {
        addresses,
        numbers,
        order_type,
        decrease_position_swap_type,
        is_long,
        should_unwrap_native_token,
        referral_code,
    };

    let tx3_builder = exchange_router_contract.create_order(create_order_object);
    let tx3_bytes: Bytes = tx3_builder.calldata().unwrap();


    // ----------------------------------
    //      Bundling & Tx Execution 
    // ----------------------------------

    let bundle: Vec<Bytes> = vec!(tx1_bytes, tx2_bytes, tx3_bytes);
    let bundle_executor = exchange_router_contract.multicall(bundle).call().await?;

    Ok(())
}