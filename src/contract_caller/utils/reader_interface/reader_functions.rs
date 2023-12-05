use ethers::abi::Token;
use ethers::types::{Address, H160, H256};

pub fn get_position_key(account: H160, market: H160, collateral_token: H160, is_long: bool) -> H256 {
    let data_values = vec![
        Token::Address(account),
        Token::Address(market),
        Token::Address(collateral_token),
        Token::Bool(is_long),
    ];

    let hash_hex = hash_data(vec!["address", "address", "address", "bool"], data_values);
    
    // Convert hex string to H256
    H256::from_slice(&hex::decode(hash_hex).expect("Invalid hex string"))
}