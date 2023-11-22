use crate::contract_caller::utils::structs::{OrderCalcInput, OrderCalcOutput, Token, TokenInfo, PriceData, TokenPriceFromApiResponse};
use ethers::types::U256;
use reqwest;
use crate::contract_caller::order_builder::get_price::fetch_token_price;

pub async fn calculate_order_params(input: OrderCalcInput) -> Result<OrderCalcOutput, Box<dyn std::error::Error>> {
    // Fetch the decimals for the collateral/index tokens
    let collateral_info: TokenInfo = Token::from_name(&input.collateral_token)
        .ok_or("Unsupported token")?
        .info();
    let collateral_decimals: u8 = collateral_info.decimals;

    let index_info: TokenInfo = Token::from_name(&input.index_token)
    .ok_or("Unsupported token")?
    .info();
    let index_decimals: u8 = index_info.decimals;

    let price_output: TokenPriceFromApiResponse = fetch_token_price(input.index_token).await?;


    // Create and return the OrderObject with calculated values
    Ok(OrderCalcOutput {
        // Fill in the fields based on calculations
        // ...
    })
}
