use ethers::abi::{AbiEncode, Token};
use ethers::types::Bytes;
use ethers::utils::keccak256;

pub fn encode_data(data_types: Vec<&str>, data_values: Vec<Token>) -> String {
    let encoded_bytes = ethers::abi::encode(&data_values);
    hex::encode(encoded_bytes)
}

pub fn hash_data(data_types: Vec<&str>, data_values: Vec<Token>) -> String {
    let encoded_bytes = ethers::abi::encode(&data_values);
    let hash = keccak256(encoded_bytes);
    hex::encode(hash)
}

pub fn hash_string(string: &str) -> String {
    hash_data(vec!["string"], vec![Token::String(string.to_string())])
}

pub fn keccak_string(string: &str) -> String {
    let hash = keccak256(string.as_bytes());
    hex::encode(hash)
}
