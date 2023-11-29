use ethers::prelude::*;
use ethers::types::transaction::eip2718::TypedTransaction;
use ethers::types::{H160, Address, U256, TransactionRequest, NameOrAddress};
use dotenv::dotenv;
use anyhow::anyhow;
use std::str::FromStr;
use std::sync::Arc;

use super::utils::local_signer::get_local_signer;
use super::utils::structs::{OrderObject, Token};
use super::utils::contract_addresses::{Contracts, CreateOrderParams, CreateOrderParamsAddresses, CreateOrderParamsNumbers};

use crate::contract_caller::connect_provider::connect_provider;
use crate::contract_caller::utils::gas_calculator::get_current_gas_price;


pub async fn sol_call(order_object: OrderObject) -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // ---------------------------------------------------------
    //                      Initialisation
    // ---------------------------------------------------------

    let wallet = get_local_signer()?;
    let provider: Provider<Http> = connect_provider().await?;
    let arc_provider: Arc<Provider<Http>> = Arc::new(provider);
    let contracts: Contracts = Contracts::new(arc_provider.clone());
    let client: SignerMiddleware<Arc<Provider<Http>>, _> = SignerMiddleware::new(arc_provider.clone(), wallet.clone());



    // Parse number values to U256
    let amount_u256: U256 = U256::from_dec_str(&order_object.amount)
    .map_err(|e| format!("Error parsing amount to U256: {}", e))?;

    let size_delta_usd: U256 = U256::from_dec_str(&order_object.size_delta_usd)
        .map_err(|e| format!("Error parsing size_delta_usd to U256: {}", e))?;

    let initial_collateral_delta_amount: U256 = U256::from_dec_str(&order_object.initial_collateral_delta_amount)
        .map_err(|e| format!("Error parsing initial_collateral_delta_amount to U256: {}", e))?;

    let trigger_price: U256 = U256::from_dec_str(&order_object.trigger_price)
        .map_err(|e| format!("Error parsing trigger_price to U256: {}", e))?;

    let acceptable_price: U256 = U256::from_dec_str(&order_object.acceptable_price)
        .map_err(|e| format!("Error parsing acceptable_price to U256: {}", e))?;

    let execution_fee: U256 = U256::from_dec_str(&order_object.execution_fee)
        .map_err(|e| format!("Error parsing execution_fee to U256: {}", e))?;

    let callback_gas_limit: U256 = U256::from_dec_str(&order_object.callback_gas_limit)
        .map_err(|e| format!("Error parsing callback_gas_limit to U256: {}", e))?;

    let min_output_amount: U256 = U256::from_dec_str(&order_object.min_output_amount)
        .map_err(|e| format!("Error parsing min_output_amount to U256: {}", e))?;


    // Parse addresses with error handling
    let receiver = order_object.receiver.parse()
        .map_err(|e| format!("Error parsing receiver address: {}", e))?;
    let callback_contract = order_object.callback_contract.parse()
        .map_err(|e| format!("Error parsing callback_contract address: {}", e))?;
    let ui_fee_receiver = order_object.ui_fee_receiver.parse()
        .map_err(|e| format!("Error parsing ui_fee_receiver address: {}", e))?;
    let market = order_object.market.parse()
        .map_err(|e| format!("Error parsing market address: {}", e))?;
    let initial_collateral_token_str: String = Token::token_address_from_name(&order_object.initial_collateral_token)
    .ok_or("Unsupported collateral token")?;
    let initial_collateral_token: Address = initial_collateral_token_str.parse()
        .map_err(|e| format!("Error parsing initial_collateral_token address: {}", e))?;


    // Handle swap path parsing
    let swap_path = order_object.swap_path.iter().map(|s| s.parse())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Error parsing swap path addresses: {}", e))?;

    // Handle referral code conversion
    let referral_code_h256 = H256::from_str(&order_object.referral_code)
        .map_err(|e| format!("Error converting referral code to H256: {}", e))?;
    let referral_code_bytes = referral_code_h256.into();



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


    // ---------------------------------------------------------
    // ------------------------------------------------------
    //                   Multicall Builder
    // ------------------------------------------------------
    // ---------------------------------------------------------

    // ----------------------------------
    //            Tx0: Approvals
    // ----------------------------------

    let exchange_router_address: H160 = contracts.exchange_router_contract.address();
    let deposit_vault_contract_address: H160 = contracts.deposit_vault_contract.address();
    let order_vault_contract_address: H160 = contracts.order_vault_contract.address();
    let double_check: Option<String> = Token::token_address_from_name(order_object.initial_collateral_token.as_str());
    println!("TESTING: TOKEN ADDRESS {:?}", double_check);
    // let tx0_builder = match order_object.initial_collateral_token.as_str() {
    //     "USDC" => contracts.usdc_contract.approve(exchange_router_address, amount_u256),
    //     "DAI" => contracts.dai_contract.approve(exchange_router_address, amount_u256),
    //     "WETH" => contracts.weth_contract.approve(exchange_router_address, amount_u256),
    //     "WBTC" => contracts.wbtc_contract.approve(exchange_router_address, amount_u256),
    //     "LINK" => contracts.link_contract.approve(exchange_router_address, amount_u256),
    //     "ARB" => contracts.arb_contract.approve(exchange_router_address, amount_u256),
    //     "UNI" => contracts.uni_contract.approve(exchange_router_address, amount_u256),
    //     "SOL" => contracts.sol_contract.approve(exchange_router_address, amount_u256),
    //     "USDT" => contracts.usdt_contract.approve(exchange_router_address, amount_u256),
    //     "USDCE" => contracts.usdce_contract.approve(exchange_router_address, amount_u256),
    //     _ => return Err("Unsupported collateral token".into()),
    // };
    // let tx0_bytes: Bytes = tx0_builder.calldata()
    // .ok_or_else(|| anyhow!("Failed to build tx0 calldata"))?;

    // println!("tx0 ok");

    // ----------------------------------
    //            Tx1: Send Gas
    // ----------------------------------

    let weth_amount: U256 = execution_fee;
    println!("EXECUTION FEE = {}", weth_amount);

    // Encode the sendWnt transaction calldata
    let tx1_builder = contracts.exchange_router_contract.send_wnt(order_vault_contract_address, weth_amount);
    let tx1_bytes: Bytes = tx1_builder.calldata()
    .ok_or_else(|| anyhow!("Failed to build tx0 calldata"))?;

    println!("tx1 ok");
    
    // ----------------------------------
    //         Tx2: Vault Deposit
    // ----------------------------------

    let tx2_builder = contracts.exchange_router_contract.send_tokens(initial_collateral_token, order_vault_contract_address, amount_u256);
    println!("TEST: AMOUNT_U256 = {}", amount_u256);
    let tx2_bytes: Bytes = tx2_builder.calldata()
    .ok_or_else(|| anyhow!("Failed to build tx0 calldata"))?;

    println!("tx2 ok");

    // ----------------------------------
    //         Tx3: Create Order
    // ----------------------------------

    let tx3_builder = contracts.exchange_router_contract.create_order(create_order_object);
    let tx3_bytes: Bytes = tx3_builder.calldata()
    .ok_or_else(|| anyhow!("Failed to build tx0 calldata"))?;

    println!("tx3 ok");

    // ----------------------------------
    //      Bundling & Tx Execution 
    // ----------------------------------

    let bundle: Vec<Bytes> = vec!(tx1_bytes, tx2_bytes, tx3_bytes);

    let gas_estimate: U256 = U256::from(4000000);
    println!("Estimated Gas: {}", gas_estimate);
    let gas_limit: U256 = gas_estimate + 100000; // Buffer
    println!("GAS LIMIT = {}", gas_limit);
    let gas_price: U256 = get_current_gas_price().await?;
    let nonce: U256 = arc_provider.clone().get_transaction_count(wallet.clone().address(), None).await
    .map_err(|e| format!("Error fetching nonce: {}", e))?;

    // Step 1: Prepare the Transaction Request
    let tx_data = contracts.exchange_router_contract.multicall(bundle.clone()).calldata().unwrap();
    let tx_request: TransactionRequest = TransactionRequest {
        from: Some(wallet.address()),
        to: Some(NameOrAddress::Address(contracts.exchange_router_contract.address())),
        gas: Some(gas_limit),
        gas_price: Some(gas_price),
        nonce: Some(nonce),
        data: Some(tx_data.into()),
        value: execution_fee.into(),
        chain_id: Some(42161.into())
    };

    // Step 2: Convert into TypedTransaction + define gas
    let access_list: Vec<_> = Vec::new();
    let priority_fee: U256 = U256::from(2000000000);
    let max_fee_per_gas: U256 = gas_price + priority_fee;
    let typed_tx: Eip1559TransactionRequest = ethers::types::transaction::eip1559::Eip1559TransactionRequest {
        from: tx_request.from,
        to: tx_request.to,
        nonce: tx_request.nonce,
        max_priority_fee_per_gas: Some(priority_fee),
        max_fee_per_gas: Some(max_fee_per_gas),
        gas: tx_request.gas,
        value: tx_request.value,
        data: tx_request.data,
        access_list: ethers::types::transaction::eip2930::AccessList(access_list),
        chain_id: tx_request.chain_id,
    };
    
    let typed_tx: TypedTransaction = TypedTransaction::Eip1559(typed_tx);
    println!("TYPED TX = {:?}", typed_tx);

    // Step 3: Sign and Send the Transaction
    let pending_tx: PendingTransaction<'_, Http> = client.send_transaction(typed_tx, None).await?;
    let receipt: Option<TransactionReceipt> = pending_tx.confirmations(1).await?;

    println!("Transaction successful, receipt: {:?}", receipt);

    Ok(())
}