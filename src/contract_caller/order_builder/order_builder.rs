use crate::contract_caller::utils::structs::{OrderCalcInput, OrderCalcOutput, Token, TokenInfo, PriceData, TokenPriceFromApiResponse};
use ethers::types::U256;
use reqwest;
use crate::contract_caller::order_builder::get_price::fetch_token_price;
use crate::contract_caller::utils::gas_calculator::calculate_execution_fee;

pub async fn calculate_order_params(input: OrderCalcInput) -> Result<OrderCalcOutput, Box<dyn std::error::Error>> {

    let price_output: TokenPriceFromApiResponse;
    let initial_collateral_delta_amount: U256 = U256::from(0);
    let trigger_price: U256 = U256::from(0);
    let min_output_amount: U256 = U256::from(0);

    let estimated_gas: u64 = 2500000;


    // Fetch the decimals for the collateral/index tokens
    let collateral_info: TokenInfo = Token::from_name(&input.collateral_token)
        .ok_or("Unsupported token")?
        .info();

    let index_info: TokenInfo = Token::from_name(&input.index_token)
        .ok_or("Unsupported token")?
        .info();

    // Convert and adjust collateral amount
    let collateral_amount_raw: U256 = U256::from_dec_str(&input.collateral_amount)?;
    let collateral_amount: U256 = collateral_amount_raw.checked_mul(U256::exp10(collateral_info.decimals as usize))
    .ok_or("Decimal adjustment error for collateral amount")?;

    // Fetch and adjust prices
    let price_output: TokenPriceFromApiResponse = fetch_token_price(input.index_token.clone()).await?;

    let min_price_raw: U256 = U256::from_dec_str(&price_output.min_price_full)?;
    let acceptable_price: U256 = min_price_raw.checked_mul(U256::exp10(index_info.decimals as usize))
        .ok_or("Decimal adjustment error for min price")?;

    // Apply leverage factor and adjust for USD amount
    let leverage_as_u256: U256 = U256::from(input.leverage_factor as u64);
    let size_usd_raw: U256 = collateral_amount * leverage_as_u256;
    let size_delta_usd: U256 = size_usd_raw.checked_mul(U256::exp10(30))
        .ok_or("Decimal adjustment error for size in USD")?;

    // Execution fee calcs
    let execution_fee = calculate_execution_fee(estimated_gas).await?;


    // Create and return the OrderObject with calculated values
    Ok(OrderCalcOutput {
        collateral_amount, 
        size_delta_usd,
        initial_collateral_delta_amount,
        trigger_price,
        acceptable_price,
        execution_fee,
        min_output_amount
    })
}
