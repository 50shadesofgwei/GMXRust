use ethers::prelude::*;
use std::sync::Arc;

abigen!{ 
    EXCHANGE_ROUTER, "/Users/jfeasby/GMX Rust/GMX_Rust/src/contract_caller/abis/exchange_router_abi.json";
    USDC_NATIVE, "/Users/jfeasby/GMX Rust/GMX_Rust/src/contract_caller/abis/usdc_arb_native_abi.json";
    ORDER_VAULT, "/Users/jfeasby/GMX Rust/GMX_Rust/src/contract_caller/abis/order_vault_abi.json";
    DEPOSIT_VAULT, "/Users/jfeasby/GMX Rust/GMX_Rust/src/contract_caller/abis/deposit_vault_abi.json";
    WETH, "/Users/jfeasby/GMX Rust/GMX_Rust/src/contract_caller/abis/weth_abi.json";
    DAI, "/Users/jfeasby/GMX Rust/GMX_Rust/src/contract_caller/abis/dai_abi.json";
    ARB, "/Users/jfeasby/GMX Rust/GMX_Rust/src/contract_caller/abis/arb_abi.json";
    LINK, "/Users/jfeasby/GMX Rust/GMX_Rust/src/contract_caller/abis/link_abi.json";
    UNI, "/Users/jfeasby/GMX Rust/GMX_Rust/src/contract_caller/abis/uni_abi.json";
    USDCE, "/Users/jfeasby/GMX Rust/GMX_Rust/src/contract_caller/abis/usdce_abi.json";
    USDT, "/Users/jfeasby/GMX Rust/GMX_Rust/src/contract_caller/abis/usdt_abi.json";
    WBTC, "/Users/jfeasby/GMX Rust/GMX_Rust/src/contract_caller/abis/wbtc_abi.json";
    SOL, "/Users/jfeasby/GMX Rust/GMX_Rust/src/contract_caller/abis/wsol_abi.json";
    GAS_UTILS, "/Users/jfeasby/GMX Rust/GMX_Rust/src/contract_caller/abis/gas_utils_abi.json";
}

abigen!{
    READER, "/Users/jfeasby/GMX Rust/GMX_Rust/src/contract_caller/abis/reader_abi.json";
}

pub struct Contracts {
    pub usdc_contract: USDC_NATIVE<Provider<Http>>,
    pub dai_contract: DAI<Provider<Http>>,
    pub weth_contract: WETH<Provider<Http>>,
    pub wbtc_contract: WBTC<Provider<Http>>,
    pub link_contract: LINK<Provider<Http>>,
    pub arb_contract: ARB<Provider<Http>>,
    pub uni_contract:UNI<Provider<Http>>,
    pub sol_contract: SOL<Provider<Http>>,
    pub usdt_contract: USDT<Provider<Http>>,
    pub usdce_contract: USDCE<Provider<Http>>,
    pub exchange_router_contract: EXCHANGE_ROUTER<Provider<Http>>,
    pub order_vault_contract: ORDER_VAULT<Provider<Http>>,
    pub deposit_vault_contract: DEPOSIT_VAULT<Provider<Http>>,
    pub gas_contract: GAS_UTILS<Provider<Http>>,
    pub reader_contract: READER<Provider<Http>>
}

impl Contracts {
    pub fn new(provider: Arc<Provider<Http>>) -> Self {
        Contracts {
            usdc_contract: USDC_NATIVE::<Provider<Http>>::new("0xaf88d065e77c8cC2239327C5EDb3A432268e5831".parse::<H160>().unwrap(), provider.clone()),
            dai_contract: DAI::<Provider<Http>>::new("0xDA10009cBd5D07dd0CeCc66161FC93D7c9000da1".parse::<H160>().unwrap(), provider.clone()),
            weth_contract: WETH::<Provider<Http>>::new("0x82aF49447D8a07e3bd95BD0d56f35241523fBab1".parse::<H160>().unwrap(), provider.clone()),
            wbtc_contract: WBTC::<Provider<Http>>::new("0x2f2a2543B76A4166549F7aaB2e75Bef0aefC5B0f".parse::<H160>().unwrap(), provider.clone()),
            link_contract: LINK::<Provider<Http>>::new("0xf97f4df75117a78c1A5a0DBb814Af92458539FB4".parse::<H160>().unwrap(), provider.clone()),
            arb_contract: ARB::<Provider<Http>>::new("0x912CE59144191C1204E64559FE8253a0e49E6548".parse::<H160>().unwrap(), provider.clone()),
            uni_contract: UNI::<Provider<Http>>::new("0xFa7F8980b0f1E64A2062791cc3b0871572f1F7f0".parse::<H160>().unwrap(), provider.clone()),
            sol_contract: SOL::<Provider<Http>>::new("0x53B56de645B9de6e5a40acE047D1c74E8B42Eccb".parse::<H160>().unwrap(), provider.clone()),
            usdt_contract: USDT::<Provider<Http>>::new("0xFd086bC7CD5C481DCC9C85ebE478A1C0b69FCbb9".parse::<H160>().unwrap(), provider.clone()),
            usdce_contract: USDCE::<Provider<Http>>::new("0xFF970A61A04b1cA14834A43f5dE4533eBDDB5CC8".parse::<H160>().unwrap(), provider.clone()),
            exchange_router_contract: EXCHANGE_ROUTER::<Provider<Http>>::new("0x7C68C7866A64FA2160F78EEaE12217FFbf871fa8".parse::<H160>().unwrap(), provider.clone()),
            order_vault_contract: ORDER_VAULT::<Provider<Http>>::new("0x31eF83a530Fde1B38EE9A18093A333D8Bbbc40D5".parse::<H160>().unwrap(), provider.clone()),
            deposit_vault_contract: DEPOSIT_VAULT::<Provider<Http>>::new("0xF89e77e8Dc11691C9e8757e84aaFbCD8A67d7A55".parse::<H160>().unwrap(), provider.clone()),
            gas_contract: GAS_UTILS::<Provider<Http>>::new("0x6Ee83F82757C5B10468855753F5374FFF826BDCB".parse::<H160>().unwrap(), provider.clone()),
            reader_contract: READER::<Provider<Http>>::new("0xf60becbba223eea9495da3f606753867ec10d139".parse::<H160>().unwrap(), provider.clone()),
        }
    }

    // Dynamically select and call approve function based on token
    pub async fn approve(&self, token: &str, spender: Address, amount: U256) -> Result<Bytes, Box<dyn std::error::Error>> {
        match token {
            "USDC" => Ok(self.usdc_contract.approve(spender, amount).calldata().unwrap()),
            "DAI" => Ok(self.dai_contract.approve(spender, amount).calldata().unwrap()),
            "WETH" => Ok(self.weth_contract.approve(spender, amount).calldata().unwrap()),
            "WBTC" => Ok(self.wbtc_contract.approve(spender, amount).calldata().unwrap()),
            "LINK" => Ok(self.link_contract.approve(spender, amount).calldata().unwrap()),
            "ARB" => Ok(self.arb_contract.approve(spender, amount).calldata().unwrap()),
            "UNI" => Ok(self.uni_contract.approve(spender, amount).calldata().unwrap()),
            "SOL" => Ok(self.sol_contract.approve(spender, amount).calldata().unwrap()),
            "USDT" => Ok(self.usdt_contract.approve(spender, amount).calldata().unwrap()),
            "USDCE" => Ok(self.usdce_contract.approve(spender, amount).calldata().unwrap()),
            _ => Err("Unsupported token".into()),
        }
    }
}