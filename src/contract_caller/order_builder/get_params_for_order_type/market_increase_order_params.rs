use crate::contract_caller::utils::local_signer::get_local_signer;
use crate::contract_caller::utils::structs::{MarketIncreaseOrderCalcOutput, Token, TokenInfo, AddressesForMarketIncreaseOrder, OrderObject, Markets, SimpleOrder};
use ethers::signers::Signer;
use ethers::types::U256;
use crate::contract_caller::order_builder::get_price::fetch_token_price;
use crate::contract_caller::utils::gas_calculator::calculate_execution_fee;

pub async fn calculate_market_increase_order_params(input: SimpleOrder) -> Result<MarketIncreaseOrderCalcOutput, Box<dyn std::error::Error>> {
    const USD_SCALE_FACTOR: u32 = 30; // GMX's scaling factor for USD values

    println!("Starting order parameter calculations...");

    let initial_collateral_delta_amount: U256 = U256::from(0);
    let trigger_price: U256 = U256::from(0);
    let min_output_amount: U256 = U256::from(0);

    let estimated_gas: u64 = 2500000;

    println!("Fetching token information...");
    let collateral_info: TokenInfo = Token::from_name(&input.collateral_token)
        .ok_or("Unsupported token")?
        .info();

    // Fetch the current price of the collateral token in USD
    let collateral_price_output = fetch_token_price(input.collateral_token.clone()).await?;
    let collateral_price: U256 = U256::from_dec_str(&collateral_price_output.min_price_full)?;

    println!("Converting and adjusting collateral amount...");
    let collateral_amount_raw: U256 = U256::from_dec_str(&input.collateral_amount)?;
    
    // Calculate the actual USD value of the collateral
    let actual_usd_value: U256 = if input.collateral_token == "USDC" {
        collateral_amount_raw // For USDC, the amount is already in USD
    } else {
        collateral_amount_raw
            .checked_mul(collateral_price)
            .and_then(|amount| amount.checked_div(U256::exp10(collateral_info.decimals as usize)))
            .ok_or("Conversion to USD value error")?
    };

    println!("Fetching and adjusting prices...");
    let price_output = fetch_token_price(input.index_token.clone()).await?;

    println!("Calculating acceptable price...");
    let acceptable_price: U256 = U256::from_dec_str(&price_output.min_price_full)?;

    println!("Applying leverage factor and adjusting for USD amount...");
    let leverage_as_u256: U256 = U256::from(input.leverage_factor as u64);
    let leveraged_usd_value: U256 = actual_usd_value.checked_mul(leverage_as_u256)
        .ok_or("Leverage application error")?;
    let size_delta_usd: U256 = leveraged_usd_value.checked_mul(U256::exp10(USD_SCALE_FACTOR as usize))
        .ok_or("Final USD scaling error")?;

    println!("Calculating execution fee...");
    let execution_fee = calculate_execution_fee(estimated_gas).await?;

    println!("Returning calculated order parameters...");
    Ok(MarketIncreaseOrderCalcOutput {
        collateral_amount: collateral_amount_raw, 
        size_delta_usd,
        initial_collateral_delta_amount,
        trigger_price,
        acceptable_price,
        execution_fee,
        min_output_amount
    })
}

pub fn get_addresses_for_market_increase_order(input: SimpleOrder) -> Result<AddressesForMarketIncreaseOrder, Box<dyn std::error::Error>> {
    let wallet = get_local_signer()?;
    let receiver: ethers::types::H160 = wallet.address();
    let default_order: OrderObject = OrderObject::default();
    let mut market: String = String::new();
    let swap_path: Vec<String> = Markets::get_swap_path_for_collateral(&input.collateral_token);

    if let Some(market_address) = Markets::get_market_address(&input.index_token) {
        println!("Market address for {}: {}", input.index_token, market_address);
        market = market_address;
    } else {
        println!("Market address not found for {}", input.index_token);
    }


    Ok(AddressesForMarketIncreaseOrder {
        receiver: receiver.to_string(),
        callback_contract: default_order.callback_contract,
        ui_fee_receiver: default_order.ui_fee_receiver,
        market,
        initial_collateral_token: input.collateral_token.to_string(),
        swap_path,
        referral_code: default_order.referral_code.to_vec(),
    })
}

pub fn create_full_order_object(
    simple_order: SimpleOrder,
    address_data: AddressesForMarketIncreaseOrder,
    calc_output: MarketIncreaseOrderCalcOutput,
) -> Result<OrderObject, Box<dyn std::error::Error>> {
    let referral_code: [u8; 32] = address_data.referral_code
        .try_into()
        .map_err(|_| "Referral code must be 32 bytes")?;

    Ok(OrderObject {
        is_long: simple_order.is_long,
        position_asset: address_data.initial_collateral_token.clone(),
        amount: calc_output.collateral_amount.to_string(),
        receiver: address_data.receiver,
        callback_contract: address_data.callback_contract,
        ui_fee_receiver: address_data.ui_fee_receiver,
        market: address_data.market,
        initial_collateral_token: address_data.initial_collateral_token,
        swap_path: address_data.swap_path,
        size_delta_usd: calc_output.size_delta_usd.to_string(),
        initial_collateral_delta_amount: calc_output.initial_collateral_delta_amount.to_string(),
        trigger_price: calc_output.trigger_price.to_string(),
        acceptable_price: calc_output.acceptable_price.to_string(),
        execution_fee: calc_output.execution_fee.to_string(),
        callback_gas_limit: "0".to_string(), 
        min_output_amount: calc_output.min_output_amount.to_string(),
        order_type: 2,
        decrease_position_swap_type: 0, 
        should_unwrap_native_token: false,
        referral_code,
    })
}
