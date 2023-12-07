// Importing necessary crates
use reqwest;
use serde_json::Value;
use std::error::Error;

pub async fn api_caller() -> Result<(), Box<dyn Error>> {
    // The URL of the API endpoint
    let url: &str = "https://arbitrum-api.gmxinfra.io/actions";
    
    // The account parameter to be passed
    let account_param = "0x729fBbB8a11Cf2d564Ba5Fd913AEdEf1D9a6ea66";

    // Creating an instance of a client
    let client = reqwest::Client::new();

    // Making a GET request with the account parameter
    let response = client.get(url)
                         .query(&[("account", account_param)])
                         .send()
                         .await?;

    // Checking if the request was successful
    if response.status().is_success() {
        // Parsing the response body as JSON
        let body = response.text().await?;
        let json: Value = serde_json::from_str(&body)?;

        // Printing out the JSON response
        println!("Response JSON: {}", json);
    } else {
        // If the response was not successful, print out the status code
        println!("Response was not successful: {}", response.status());
    }

    Ok(())
}