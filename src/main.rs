pub mod contract_caller;

use crate::contract_caller::sol_call::sol_call;
use crate::contract_caller::utils::structs::{SimpleOrder, OrderObject};
use crate::contract_caller::order_builder::get_params_for_order_type::market_increase_order_params::get_order_object_from_simple_order;
use crate::contract_caller::utils::reader_interface::reader_functions::get_position_key;
use crate::contract_caller::utils::reader_interface::api_caller::api_caller;
use dotenv::dotenv;
use ethers::signers::Signer;
use std::env;
use ethers::prelude::LocalWallet;



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let test = api_caller().await?;

    // let address_str: &str = "0xa6D1FEda6fc70680816eF6A23faf5e454e2f9C09";
    // let address: H160 = address_str.parse().expect("Invalid H160 address");
    // let market_str: &str = "0x70d95587d40A2caf56bd97485aB3Eec10Bee6336";
    // let market: H160 = market_str.parse().expect("Invalid H160 address");
    // let collateral_str: &str = "0xaf88d065e77c8cC2239327C5EDb3A432268e5831";
    // let collateral: H160 = collateral_str.parse().expect("Invalid H160 address");

    // let test_hash: ethers::types::H256 = get_position_key(
    //     address,
    //     market,
    //     collateral,
    //     true
    // );

    // let test_hash_str: String = format!("{:x}", test_hash);

    // println!("Test Key Result = {}", test_hash_str);

    // let test: SimpleOrder = SimpleOrder {
    //     is_long: true,
    //     collateral_token: "ETH".to_string(),
    //     collateral_amount: "2053551709838329".to_string(),
    //     index_token: "WBTC".to_string(),
    //     leverage_factor: 10.0,
    // };

    // let test_result: OrderObject = get_order_object_from_simple_order(&test).await?;
    // let receipt = sol_call(test_result).await?;

    // println!("{:?}", receipt);

    Ok(())
}