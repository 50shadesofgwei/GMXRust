use alloy_json_abi::JsonAbi;

fn get_abi() {
    let path = "GMX_Rust/src/contract_caller/abis/exchange_router_abi.json";

    match fs::read_to_string(path) {
        Ok(json) => {
            match serde_json::from_str::<JsonAbi>(&json) {
                Ok(abi) => {
                    // TODO
                },
                Err(e) => eprintln!("Failed to parse JSON: {}", e),
            }
        },
        Err(e) => eprintln!("Failed to read file: {}", e),
    }
}