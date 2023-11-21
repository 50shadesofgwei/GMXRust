use ethers::prelude::*;

// Struct for 'addresses' parameter in 'createOrder'
pub struct CreateOrderParamsAddresses {
    pub receiver: Address,
    pub callback_contract: Address,
    pub ui_fee_receiver: Address,
    pub market: Address,
    pub initial_collateral_token: Address,
    pub swap_path: Vec<Address>,
}

// Struct for 'numbers' parameter in 'createOrder'
pub struct CreateOrderParamsNumbers {
    pub size_delta_usd: U256,
    pub initial_collateral_delta_amount: U256,
    pub trigger_price: U256,
    pub acceptable_price: U256,
    pub execution_fee: U256,
    pub callback_gas_limit: U256,
    pub min_output_amount: U256,
}

pub struct CreateOrderStruct {
        addresses: CreateOrderParamsAddresses,
        numbers: CreateOrderParamsNumbers,
        order_type: i32,
        decrease_position_swap_type: i32,
        is_long: bool,
        should_unwrap_native_token: bool,
        referral_code: [u8; 32],
}

pub struct OrderObject {
    pub is_long: bool,
    pub position_asset: String,
    pub amount: String,
    pub receiver: String,
    pub callback_contract: String,
    pub ui_fee_receiver: String,
    pub market: String,
    pub initial_collateral_token: String,
    pub swap_path: Vec<String>,
    pub size_delta_usd: String,
    pub initial_collateral_delta_amount: String,
    pub trigger_price: String,
    pub acceptable_price: String,
    pub execution_fee: String,
    pub callback_gas_limit: String,
    pub min_output_amount: String,
    pub order_type: u8,
    pub decrease_position_swap_type: u8,
    pub should_unwrap_native_token: bool,
    pub referral_code: [u8; 32],
}

pub enum OrderType {
    MarketSwap,
    LimitSwap,
    MarketIncrease,
    LimitIncrease,
    MarketDecrease,
    LimitDecrease,
    StopLossDecrease,
    Liquidation,
}

pub enum DecreasePositionSwapType {
    NoSwap,
    SwapPnlTokenToCollateralToken,
    SwapCollateralTokenToPnlToken
}

pub struct TokenInfo {
    name: &'static str,
    address: &'static str,
    decimals: u8,
}

pub enum Token {
    ETH,
    WETH,
    BTC,
    LINK,
    ARB,
    SOL,
    UNI,
    XRP,
    LTC,
    DOGE,
    USDC,
    USDCE,
    DAI,
    USDT
}

impl Token {
    pub fn info(&self) -> TokenInfo {
        match self {
            Token::ETH => TokenInfo { name: "ETH", address: "0x...EthAddress...", decimals: 18 },
            Token::WETH => TokenInfo { name: "WETH", address: "0x...WEthAddress...", decimals: 18 },
            Token::BTC => TokenInfo { name: "BTC", address: "0x...BtcAddress...", decimals: 8 },
            Token::LINK => TokenInfo { name: "LINK", address: "0x...LinkAddress...", decimals: 18 },
            Token::ARB => TokenInfo { name: "ARB", address: "0x...ArbAddress...", decimals: 18 },
            Token::SOL => TokenInfo { name: "SOL", address: "0x...SolAddress...", decimals: 9 },
            Token::UNI => TokenInfo { name: "UNI", address: "0x...UniAddress...", decimals: 18 },
            Token::XRP => TokenInfo { name: "XRP", address: "0x...XrpAddress...", decimals: 6 },
            Token::DOGE => TokenInfo { name: "DOGE", address: "0x...DogeAddress...", decimals: 8 },
            Token::LTC => TokenInfo { name: "LTC", address: "0x...LtcAddress...", decimals: 8 },
            Token::USDC => TokenInfo { name: "USDC", address: "0x...USDCAddress...", decimals: 18 },
            Token::USDCE => TokenInfo { name: "USDCE", address: "0x...USDCEAddress...", decimals: 6 },
            Token::DAI => TokenInfo { name: "DAI", address: "0x...DAIAddress...", decimals: 8 },
            Token::USDT => TokenInfo { name: "USDT", address: "0x...USDTAddress...", decimals: 8 },
        }
    }

    pub fn from_name(name: &str) -> Option<Token> {
        match name {
            "ETH" => Some(Token::ETH),
            "BTC" => Some(Token::BTC),
            "LINK" => Some(Token::LINK),
            "ARB" => Some(Token::ARB),
            "SOL" => Some(Token::SOL),
            "UNI" => Some(Token::UNI),
            "XRP" => Some(Token::XRP),
            "DOGE" => Some(Token::DOGE),
            "LTC" => Some(Token::LTC),
            "USDC" => Some(Token::USDC),
            "USDCE" => Some(Token::USDCE),
            "DAI" => Some(Token::DAI),
            "USDT" => Some(Token::USDT),
            _ => None,
        }
    }
}