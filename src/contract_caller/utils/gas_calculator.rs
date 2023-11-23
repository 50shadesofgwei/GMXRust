use reqwest;
use ethers::core::types::U256;
use super::structs::GasPriceResponse;

pub async fn calculate_execution_fee(gas_estimate: u64) -> Result<u64, Box<dyn std::error::Error>> {
    // Fetch the current gas price
    let current_gas_price: U256 = get_current_gas_price().await?;

    // Calculate the execution fee
    let execution_fee: U256 = current_gas_price.checked_mul(U256::from(gas_estimate))
        .ok_or_else(|| "Execution fee overflow")?;

    // Convert U256 to u64 safely
    let execution_fee_u64: u64 = execution_fee.low_u64(); // Use low_u64 for values that fit into u64

    Ok(execution_fee_u64)
}

async fn get_current_gas_price() -> Result<U256, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let url = "https://eth-mainnet.alchemyapi.io/v2/yourAlchemyApiKey/eth_gasPrice";

    let response: reqwest::Response = client.get(url).send().await?;
    let gas_price_response: GasPriceResponse = response.json().await?;

    // Convert the hex gas price to U256
    let gas_price_wei = U256::from_str_radix(&gas_price_response.result[2..], 16)?; // Remove the "0x" prefix

    Ok(gas_price_wei)
}
