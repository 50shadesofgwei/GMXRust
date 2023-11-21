use ethers::prelude::{LocalWallet};
use std::env;
use dotenv::dotenv;


pub fn get_local_signer() -> Result<LocalWallet, Box<dyn std::error::Error>> {
    dotenv().ok();
    let key: String = env::var("TEST_WALLET_PRIV_KEY")?;
    let wallet = key.parse::<LocalWallet>()?;

    Ok(wallet)
}