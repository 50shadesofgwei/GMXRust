use reqwest;
use ethers::core::types::U256;
use super::structs::GasPriceResponse;

pub async fn calculate_execution_fee(gas_estimate: u64) -> Result<u64, Box<dyn std::error::Error>> {
    // Fetch the current gas price
    let current_gas_price: U256 = get_current_gas_price().await?;

    // Calculate the execution fee
    let execution_fee = gas_estimate * current_gas_price;

    Ok(execution_fee)
}

async fn get_current_gas_price() -> Result<U256, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let url = "https://eth-mainnet.alchemyapi.io/v2/yourAlchemyApiKey/eth_gasPrice";

    let response = client.get(url).send().await?;
    let gas_price_response: GasPriceResponse = response.json().await?;

    // Convert the hex gas price to U256
    let gas_price_wei = U256::from_str_radix(&gas_price_response.result[2..], 16)?; // Remove the "0x" prefix

    Ok(gas_price_wei)
}
