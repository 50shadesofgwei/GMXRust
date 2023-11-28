pub mod contract_caller;

use crate::contract_caller::sol_call::sol_call;
use crate::contract_caller::utils::structs::{SimpleOrder, OrderObject};
use crate::contract_caller::order_builder::get_params_for_order_type::market_increase_order_params::get_order_object_from_simple_order;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let test: SimpleOrder = SimpleOrder {
        is_long: true,
        collateral_token: "USDC".to_string(),
        collateral_amount: "5000000".to_string(),
        index_token: "ETH".to_string(),
        leverage_factor: 10.0,
    };

    let test_result: OrderObject = get_order_object_from_simple_order(&test).await?;
    let receipt = sol_call(test_result).await?;

    println!("{:?}", receipt);

    Ok(())
}