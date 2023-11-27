use ethers::prelude::*;
use std::env;
use std::error::Error;

pub async fn connect_provider() -> Result<Provider<Http>, Box<dyn Error>> {
    let provider_url: String = match env::var("PROVIDER_URL") {
        Ok(value) => value,
        Err(e) => {
            println!("Couldn't read PROVIDER_URL ({})", e);
            return Err(e.into());
        },
    };
    
    let provider: Provider<Http> = Provider::<Http>::try_from(provider_url.as_str())?;
    let block_number: U64 = provider.get_block_number().await?;
    println!("Connected to provider at block number: {block_number}");

    Ok(provider)
}