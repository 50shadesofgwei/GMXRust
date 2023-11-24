pub mod contract_caller;

use crate::contract_caller::utils::structs::{MarketIncreaseOrderCalcInput, MarketIncreaseOrderCalcOutput};
use crate::contract_caller::order_builder::get_params_for_order_type::market_increase_order_params::calculate_market_increase_order_params;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let test: MarketIncreaseOrderCalcInput = MarketIncreaseOrderCalcInput {
        collateral_token: "LINK".to_string(),
        collateral_amount: "10000000000000000000".to_string(),
        index_token: "ETH".to_string(),
        leverage_factor: 10.0,
    };

    let test_result: MarketIncreaseOrderCalcOutput = calculate_market_increase_order_params(test).await?;

    // Use test_result for something, or print it out
    println!("{:?}", test_result);

    Ok(())
}