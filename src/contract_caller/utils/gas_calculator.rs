use reqwest;
use ethers::core::types::U256;
use super::structs::GasPriceResponse;
use std::env;

pub async fn calculate_execution_fee(gas_estimate: u64) -> Result<U256, Box<dyn std::error::Error>> {
    println!("Calculating execution fee...");

    // Fetch the current gas price
    println!("Fetching current gas price...");
    let current_gas_price: U256 = get_current_gas_price().await?;
    println!("Current gas price: {}", current_gas_price);

    // Calculate the execution fee
    println!("Calculating execution fee with gas estimate: {}", gas_estimate);
    let execution_fee: U256 = current_gas_price.checked_mul(U256::from(gas_estimate))
        .ok_or_else(|| "Execution fee overflow")?;

    println!("Calculated execution fee: {}", execution_fee);

    Ok(execution_fee)
}

pub async fn get_current_gas_price() -> Result<U256, Box<dyn std::error::Error>> {
    println!("Fetching current gas price...");

    // Use the Arbitrum API key from the environment variables
    let alchemy_api_key = env::var("ALCHEMY_ARBITRUM_API_KEY")
        .expect("ALCHEMY_ARBITRUM_API_KEY not set in the environment variables");

    let client = reqwest::Client::new();
    let url = format!("https://arb-mainnet.g.alchemy.com/v2/{}", alchemy_api_key);

    let request_body = r#"{"jsonrpc":"2.0","method":"eth_gasPrice","params":[],"id":0}"#;

    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .body(request_body)
        .send()
        .await?;
    println!("Received response from Alchemy");

    let response_text = response.text().await?;
    println!("Response from Alchemy: {}", response_text);

    // Parse the response_text into the GasPriceResponse struct
    let gas_price_response: GasPriceResponse = serde_json::from_str(&response_text)?;
    println!("Parsed response");

    // Convert the hex gas price to U256
    let gas_price_wei = U256::from_str_radix(&gas_price_response.result[2..], 16)?;
    println!("Converted gas price to U256");

    Ok(gas_price_wei)
}
