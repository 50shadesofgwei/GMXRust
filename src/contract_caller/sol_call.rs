use ethers::prelude::*;
use ethers::types::{H160, Address, U256};
use dotenv::dotenv;
use std::env;
use std::str::FromStr;
use std::sync::Arc;

use super::utils::local_signer::get_local_signer;
use super::utils::structs::OrderObject;
use super::utils::contract_addresses::Contracts;

use crate::contract_caller::connect_provider::connect_provider;

// Create contract instances from abis w/ abigen
abigen!{ 
    EXCHANGE_ROUTER, "/Users/jfeasby/GMX Rust/GMX_Rust/src/contract_caller/abis/exchange_router_abi.json";
    USDC_NATIVE, "/Users/jfeasby/GMX Rust/GMX_Rust/src/contract_caller/abis/usdc_arb_native_abi.json";
    VAULT, "/Users/jfeasby/GMX Rust/GMX_Rust/src/contract_caller/abis/vault_abi.json";
    WETH, "/Users/jfeasby/GMX Rust/GMX_Rust/src/contract_caller/abis/weth_abi.json";
    DAI, "/Users/jfeasby/GMX Rust/GMX_Rust/src/contract_caller/abis/dai_abi.json";
    ARB, "/Users/jfeasby/GMX Rust/GMX_Rust/src/contract_caller/abis/arb_abi.json";
    LINK, "/Users/jfeasby/GMX Rust/GMX_Rust/src/contract_caller/abis/link_abi.json";
    UNI, "/Users/jfeasby/GMX Rust/GMX_Rust/src/contract_caller/abis/uni_abi.json";
    USDCE, "/Users/jfeasby/GMX Rust/GMX_Rust/src/contract_caller/abis/usdce_abi.json";
    USDT, "/Users/jfeasby/GMX Rust/GMX_Rust/src/contract_caller/abis/usdt_abi.json";
    WBTC, "/Users/jfeasby/GMX Rust/GMX_Rust/src/contract_caller/abis/wbtc_abi.json";
    SOL, "/Users/jfeasby/GMX Rust/GMX_Rust/src/contract_caller/abis/wsol_abi.json";
}

pub async fn sol_call(order_object: OrderObject) -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // ---------------------------------------------------------
    //                      Initialisation
    // ---------------------------------------------------------

    let provider: Provider<Http> = connect_provider().await?;
    let arc_provider: Arc<Provider<Http>> = Arc::new(provider);
    let contracts: Contracts = Contracts::new(arc_provider);



    // Parse number values to U256
    let amount_u256: U256 = order_object.amount.parse()?;
    let size_delta_usd: U256 = order_object.size_delta_usd.parse()?;
    let initial_collateral_delta_amount: U256 = order_object.initial_collateral_delta_amount.parse()?;
    let trigger_price: U256 = order_object.trigger_price.parse()?;
    let acceptable_price: U256 = order_object.acceptable_price.parse()?;
    let execution_fee: U256 = order_object.execution_fee.parse()?;
    let callback_gas_limit: U256 = order_object.callback_gas_limit.parse()?;
    let min_output_amount: U256 = order_object.min_output_amount.parse()?;

    // Parse addresses
    let receiver: Address = order_object.receiver.parse()?;
    let callback_contract: Address = order_object.callback_contract.parse()?;
    let ui_fee_receiver: Address = order_object.ui_fee_receiver.parse()?;
    let market: Address = order_object.market.parse()?;
    let initial_collateral_token: Address = order_object.initial_collateral_token.parse()?;
    let swap_path: Vec<Address> = order_object.swap_path.iter().map(|s| s.parse()).collect::<Result<_, _>>()?;

    let referral_code_str: String = order_object.referral_code;
    let referral_code_h256: TxHash = H256::from_str(&referral_code_str)?;
    let referral_code_bytes: [u8; 32] = referral_code_h256.into();

    // Create the order object to be submitted to the chain
    let create_order_object: CreateOrderParams = CreateOrderParams {
        addresses: CreateOrderParamsAddresses {
            receiver,
            callback_contract,
            ui_fee_receiver,
            market,
            initial_collateral_token,
            swap_path,
        },
        numbers: CreateOrderParamsNumbers {
            size_delta_usd,
            initial_collateral_delta_amount,
            trigger_price,
            acceptable_price,
            execution_fee,
            callback_gas_limit,
            min_output_amount,
        },
        order_type: order_object.order_type,
        decrease_position_swap_type: order_object.decrease_position_swap_type,
        is_long: order_object.is_long,
        should_unwrap_native_token: order_object.should_unwrap_native_token,
        referral_code: referral_code_bytes,
    };


    // Build local wallet
    let wallet = get_local_signer()?;


    // ---------------------------------------------------------
    // ------------------------------------------------------
    //                   Multicall Builder
    // ------------------------------------------------------
    // ---------------------------------------------------------

    // ----------------------------------
    //            Tx0: Approvals
    // ----------------------------------

    let exchange_router_address: H160 = contracts.exchange_router_contract.address();
    let vault_contract_address: H160 = contracts.vault_contract.address();
    let tx0_builder = match order_object.initial_collateral_token.as_str() {
        "USDC" => contracts.usdc_contract.approve(exchange_router_address, amount_u256),
        "DAI" => contracts.dai_contract.approve(exchange_router_address, amount_u256),
        "WETH" => contracts.weth_contract.approve(exchange_router_address, amount_u256),
        "WBTC" => contracts.wbtc_contract.approve(exchange_router_address, amount_u256),
        "LINK" => contracts.link_contract.approve(exchange_router_address, amount_u256),
        "ARB" => contracts.arb_contract.approve(exchange_router_address, amount_u256),
        "UNI" => contracts.uni_contract.approve(exchange_router_address, amount_u256),
        "SOL" => contracts.sol_contract.approve(exchange_router_address, amount_u256),
        "USDT" => contracts.usdt_contract.approve(exchange_router_address, amount_u256),
        "USDCE" => contracts.usdce_contract.approve(exchange_router_address, amount_u256),
        _ => return Err("Unsupported collateral token".into()),
    };
    let tx0_bytes: Bytes = tx0_builder.calldata().unwrap();

    // ----------------------------------
    //            Tx1: Send Gas
    // ----------------------------------

    let weth_amount: U256 = execution_fee;

    // Encode the sendWnt transaction calldata
    let tx1_builder = contracts.exchange_router_contract.send_wnt(vault_contract_address, weth_amount);
    let tx1_bytes: Bytes = tx1_builder.calldata().unwrap();
    
    // ----------------------------------
    //         Tx2: Vault Deposit
    // ----------------------------------

    let token_address_str: String = order_object.initial_collateral_token;
    let token_address_h160: H160 = token_address_str.parse()?;
    let tx2_builder = contracts.exchange_router_contract.send_tokens(token_address_h160, vault_contract_address, amount_u256);
    let tx2_bytes: Bytes = tx2_builder.calldata().unwrap();

    // ----------------------------------
    //         Tx3: Create Order
    // ----------------------------------

    let tx3_builder = contracts.exchange_router_contract.create_order(create_order_object);
    let tx3_bytes: Bytes = tx3_builder.calldata().unwrap();


    // ----------------------------------
    //      Bundling & Tx Execution 
    // ----------------------------------

    let bundle: Vec<Bytes> = vec!(tx1_bytes, tx2_bytes, tx3_bytes);
    let multicall_tx_call = contracts.exchange_router_contract.multicall(bundle);

    // Sign and send the transaction directly using the wallet
    let receipt = multicall_tx_call.send().await?;

    Ok(())
}