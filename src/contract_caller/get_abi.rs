use alloy_json_abi::JsonAbi;
use std::{fs::read_to_string, io::{self, ErrorKind}};

pub fn get_abi() -> Result<JsonAbi, io::Error> {
    let path: &str = "/Users/jfeasby/GMX Rust/GMX_Rust/src/contract_caller/abis/exchange_router_abi.json";

    let json: String = read_to_string(path)?;

    match serde_json::from_str::<JsonAbi>(&json) {
        Ok(abi) => Ok(abi),
        Err(e) => {
            eprintln!("Failed to parse JSON: {}", e);
            Err(io::Error::new(ErrorKind::InvalidData, "Failed to parse JSON"))
        }
    }
}