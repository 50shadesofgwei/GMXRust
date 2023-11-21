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
    WETH,
    BTC,
    WBTC,
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
            Token::WETH => TokenInfo { name: "WETH", address: "0x82aF49447D8a07e3bd95BD0d56f35241523fBab1", decimals: 18 },
            Token::BTC => TokenInfo { name: "BTC", address: "0x47904963fc8b2340414262125aF798B9655E58Cd", decimals: 8 }, // Synthetic
            Token::WBTC => TokenInfo { name: "WBTC", address: "0x2f2a2543B76A4166549F7aaB2e75Bef0aefC5B0f", decimals: 8 },
            Token::LINK => TokenInfo { name: "LINK", address: "0xf97f4df75117a78c1A5a0DBb814Af92458539FB4", decimals: 18 },
            Token::ARB => TokenInfo { name: "ARB", address: "0x912CE59144191C1204E64559FE8253a0e49E6548", decimals: 18 },
            Token::SOL => TokenInfo { name: "SOL", address: "0x2bcC6D6CdBbDC0a4071e48bb3B969b06B3330c07", decimals: 9 },
            Token::UNI => TokenInfo { name: "UNI", address: "0xFa7F8980b0f1E64A2062791cc3b0871572f1F7f0", decimals: 18 },
            Token::XRP => TokenInfo { name: "XRP", address: "0xc14e065b0067dE91534e032868f5Ac6ecf2c6868", decimals: 6 }, // Synthetic
            Token::DOGE => TokenInfo { name: "DOGE", address: "0xC4da4c24fd591125c3F47b340b6f4f76111883d8", decimals: 8 }, // Synthetic
            Token::LTC => TokenInfo { name: "LTC", address: "0xB46A094Bc4B0adBD801E14b9DB95e05E28962764", decimals: 8 }, // Synthetic
            Token::USDC => TokenInfo { name: "USDC", address: "0xaf88d065e77c8cC2239327C5EDb3A432268e5831", decimals: 6 },
            Token::USDCE => TokenInfo { name: "USDCE", address: "0xFF970A61A04b1cA14834A43f5dE4533eBDDB5CC8", decimals: 6 },
            Token::DAI => TokenInfo { name: "DAI", address: "0xDA10009cBd5D07dd0CeCc66161FC93D7c9000da1", decimals: 18 },
            Token::USDT => TokenInfo { name: "USDT", address: "0xFd086bC7CD5C481DCC9C85ebE478A1C0b69FCbb9", decimals: 6 },
        }
    }

    pub fn from_name(name: &str) -> Option<Token> {
        match name {
            "WETH" => Some(Token::WETH),
            "BTC" => Some(Token::BTC),
            "WBTC" => Some(Token::WBTC),
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