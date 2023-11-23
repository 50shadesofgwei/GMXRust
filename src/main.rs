pub mod contract_caller;

use crate::contract_caller::utils::structs::{OrderCalcInput, OrderCalcOutput};
use crate::contract_caller::order_builder::order_builder::calculate_order_params;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let test: OrderCalcInput = OrderCalcInput {
        order_type: 2,
        collateral_token: "USDC".to_string(),
        collateral_amount: "10000000000".to_string(),
        index_token: "ETH".to_string(),
        leverage_factor: 10.0,
    };

    let test_result: OrderCalcOutput = calculate_order_params(test).await?;

    // Use test_result for something, or print it out
    println!("{:?}", test_result);

    Ok(())
}