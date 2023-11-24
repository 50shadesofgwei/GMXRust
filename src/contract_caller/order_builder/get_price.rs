use crate::contract_caller::utils::structs::{TokenPriceFromApiResponse, ApiResponse};
use reqwest;

pub async fn fetch_token_price(index_token: String) -> Result<TokenPriceFromApiResponse, Box<dyn std::error::Error>> {
    let url: &str = "https://arbitrum-api.gmxinfra.io/signed_prices/latest";
    
    // Get the raw response
    let response = reqwest::get(url).await?;
    let response_text = response.text().await?;
    println!("Response from API: {}", response_text);

    // Deserialize the response text to ApiResponse
    let response_json: ApiResponse = serde_json::from_str(&response_text)?;

    println!("Searching for token: {}", index_token);

    // Find the relevant price data for the specified token
    for price_data in response_json.signed_prices {
        if price_data.token_symbol == index_token {
            let min_price = price_data.min_price_full.unwrap_or_default();
            let max_price = price_data.max_price_full.unwrap_or_default();

            return Ok(TokenPriceFromApiResponse {
                token_symbol: price_data.token_symbol,
                min_price_full: min_price,
                max_price_full: max_price,
            });
        }
    }

    Err("Token not found in price data".into())
}

