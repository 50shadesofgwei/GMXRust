use crate::contract_caller::utils::structs::{TokenPriceFromApiResponse, ApiResponse};
use reqwest;

pub async fn fetch_token_price(index_token: String) -> Result<TokenPriceFromApiResponse, Box<dyn std::error::Error>> {
    let url: &str = "https://arbitrum-api.gmxinfra.io/signed_prices/latest";
    let response: ApiResponse = reqwest::get(url).await?.json().await?;

    println!("Searching for token: {}", index_token);

    // Find the relevant price data for the specified token
    for price_data in response.signed_prices {
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
