pub mod contract_caller;

use contract_caller::connect_provider::connect_provider;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    match connect_provider().await {
        Ok(_) => println!("Successfully connected to the provider."),
        Err(e) => println!("Error connecting to the provider: {}", e),
    }
}