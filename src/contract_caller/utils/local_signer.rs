use ethers::{prelude::{LocalWallet}, signers::Signer};
use std::env;
use dotenv::dotenv;


pub fn get_local_signer() -> Result<LocalWallet, Box<dyn std::error::Error>> {
    dotenv().ok();
    let chain_id: u64 = 42161;
    let key: String = env::var("TEST_WALLET_PRIV_KEY")?;
    let wallet = key.parse::<LocalWallet>()?;
    let wallet_with_chain_id = wallet.with_chain_id(chain_id);

    Ok(wallet_with_chain_id)
}