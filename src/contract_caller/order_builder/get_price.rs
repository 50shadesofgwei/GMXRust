use crate::contract_caller::utils::structs::{TokenPriceFromApiResponse, PriceData};
use reqwest;

pub async fn fetch_token_price(index_token: String) -> Result<TokenPriceFromApiResponse, Box<dyn std::error::Error>> {
    let response: Vec<PriceData> = reqwest::get("https://arbitrum-api.gmxinfra.io/signed_prices/latest")
        .await?
        .json::<Vec<PriceData>>()
        .await?;

    for price_data in response {
        if price_data.token_symbol == index_token {
            let min_price = price_data.min_price_full
                .as_ref()
                .and_then(|p| p.parse::<String>().ok())
                .ok_or("Failed to parse min_price_full")?;

            let max_price = price_data.max_price_full
                .as_ref()
                .and_then(|p| p.parse::<String>().ok())
                .ok_or("Failed to parse max_price_full")?;

            return Ok(TokenPriceFromApiResponse {
                token_symbol: price_data.token_symbol,
                min_price_full: min_price,
                max_price_full: max_price,
            });
        }
    }

    Err("Token not found in price data".into())
}
