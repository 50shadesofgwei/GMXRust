use crate::contract_caller::utils::structs::{OrderCalcInput, OrderCalcOutput, Token, TokenInfo, PriceData, TokenPriceFromApiResponse};
use ethers::types::U256;
use reqwest;
use crate::contract_caller::order_builder::get_price::fetch_token_price;

pub async fn calculate_order_params(input: OrderCalcInput) -> Result<OrderCalcOutput, Box<dyn std::error::Error>> {

    let mut price_output: TokenPriceFromApiResponse;

    // Fetch the decimals for the collateral/index tokens
    let collateral_info: TokenInfo = Token::from_name(&input.collateral_token)
        .ok_or("Unsupported token")?
        .info();
    let collateral_decimals: u8 = collateral_info.decimals;

    let index_info: TokenInfo = Token::from_name(&input.index_token)
        .ok_or("Unsupported token")?
        .info();
    let index_decimals: u8 = index_info.decimals;

    if input.collateral_token != input.index_token {
        price_output = fetch_token_price(input.index_token.clone()).await?;
    } else {
        // Handle the case where collateral token is the same as index token
        price_output = fetch_token_price(input.collateral_token.clone()).await?;
    }

    // Convert and adjust collateral amount
    let collateral_amount_raw: U256 = U256::from_dec_str(&input.collateral_amount)?;
    let collateral_amount: U256 = collateral_amount_raw.checked_mul(U256::exp10(collateral_info.decimals as usize))
    .ok_or("Decimal adjustment error for collateral amount")?;

    // Fetch and adjust prices
    let price_output: TokenPriceFromApiResponse = fetch_token_price(input.index_token.clone()).await?;

    let min_price_raw: U256 = U256::from_dec_str(&price_output.min_price_full)?;
    let min_price: U256 = min_price_raw.checked_mul(U256::exp10(index_info.decimals as usize))
        .ok_or("Decimal adjustment error for min price")?;

    // Apply leverage factor and adjust for USD amount
    let leverage_as_u256: U256 = U256::from(input.leverage_factor as u64);
    let size_usd_raw: U256 = collateral_amount * leverage_as_u256;
    let size_usd: U256 = size_usd_raw.checked_mul(U256::exp10(30))
        .ok_or("Decimal adjustment error for size in USD")?;


    // Create and return the OrderObject with calculated values
    Ok()
}
