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

pub struct MarketInfo {
    market: String,
    market_address: String,
    is_synthetic: Option<bool>,
}

pub enum Markets {
    WETH,
    WBTC,
    DOGE,
    SOL,
    LTC,
    UNI,
    LINK,
    ARB,
    USDCE,
    USDT,
    DAI,
    XRP,
}

impl Markets {
    pub fn info(&self) -> MarketInfo {
        match self {
            Markets::WETH => MarketInfo {
                market: "WETH".to_string(),
                market_address: "0x70d95587d40A2caf56bd97485aB3Eec10Bee6336".to_string(),
                is_synthetic: Some(false),
            },
            Markets::WBTC => MarketInfo {
                market: "WBTC".to_string(),
                market_address: "0x47c031236e19d024b42f8AE6780E44A573170703".to_string(),
                is_synthetic: Some(false),
            },
            Markets::DOGE => MarketInfo {
                market: "DOGE".to_string(),
                market_address: "0x6853EA96FF216fAb11D2d930CE3C508556A4bdc4".to_string(),
                is_synthetic: Some(true),
            },
            Markets::SOL => MarketInfo {
                market: "SOL".to_string(),
                market_address: "0x09400D9DB990D5ed3f35D7be61DfAEB900Af03C9".to_string(),
                is_synthetic: Some(false),
            },
            Markets::LTC => MarketInfo {
                market: "LTC".to_string(),
                market_address: "0xD9535bB5f58A1a75032416F2dFe7880C30575a41".to_string(),
                is_synthetic: Some(true),
            },
            Markets::UNI => MarketInfo {
                market: "UNI".to_string(),
                market_address: "0xc7Abb2C5f3BF3CEB389dF0Eecd6120D451170B50".to_string(),
                is_synthetic: Some(false),
            },
            Markets::LINK => MarketInfo {
                market: "LINK".to_string(),
                market_address: "0x7f1fa204bb700853D36994DA19F830b6Ad18455C".to_string(),
                is_synthetic: Some(false),
            },
            Markets::ARB => MarketInfo {
                market: "ARB".to_string(),
                market_address: "0xC25cEf6061Cf5dE5eb761b50E4743c1F5D7E5407".to_string(),
                is_synthetic: Some(false),
            },
            Markets::USDCE => MarketInfo {
                market: "USDCE".to_string(),
                market_address: "0x9C2433dFD71096C435Be9465220BB2B189375eA7".to_string(),
                is_synthetic: Some(false),
            },
            Markets::USDT => MarketInfo {
                market: "USDT".to_string(),
                market_address: "0xB686BcB112660343E6d15BDb65297e110C8311c4".to_string(),
                is_synthetic: Some(false),
            },
            Markets::DAI => MarketInfo {
                market: "DAI".to_string(),
                market_address: "0xe2fEDb9e6139a182B98e7C2688ccFa3e9A53c665".to_string(),
                is_synthetic: Some(false),
            },
            Markets::XRP => MarketInfo {
                market: "XRP".to_string(),
                market_address: "0x0CCB4fAa6f1F1B30911619f1184082aB4E25813c".to_string(),
                is_synthetic: Some(true),
            },
        }
    }
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

impl Default for OrderObject {
    fn default() -> Self {
        OrderObject {
            is_long: true,
            position_asset: "".to_string(),
            amount: "".to_string(),
            receiver: "".to_string(),
            callback_contract: "0x0000000000000000000000000000000000000000".to_string(),
            ui_fee_receiver: "0x0000000000000000000000000000000000000000".to_string(),
            market: "0x0000000000000000000000000000000000000000".to_string(),
            initial_collateral_token: "0x0000000000000000000000000000000000000000".to_string(),
            swap_path: Vec::new(),
            size_delta_usd: "".to_string(),
            initial_collateral_delta_amount: "".to_string(),
            trigger_price: "".to_string(),
            acceptable_price: "".to_string(),
            execution_fee: "".to_string(),
            callback_gas_limit: "".to_string(),
            min_output_amount: "".to_string(),
            order_type: 0,
            decrease_position_swap_type: 0,
            should_unwrap_native_token: false,
            referral_code: [0; 32],
        }
    }
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

pub struct SimpleOrder {
    pub is_long: bool,
    pub index_token: String,
    pub amount: String,
    pub collateral_token: String,
    pub leverage_factor: f32,
}

