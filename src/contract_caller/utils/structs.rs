use ethers::prelude::*;
use serde::{Deserialize};

use super::hash_utils::hash_string;

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

pub struct MarketInfo {
    market: String,
    market_address: String,
    is_synthetic: Option<bool>,
}

pub enum Markets {
    ETH,
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
            Markets::ETH => MarketInfo {
                market: "ETH".to_string(),
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

    pub fn from_token_name(token_name: &str) -> Option<Self> {
        match token_name {
            "ETH" => Some(Markets::ETH),
            "WBTC" => Some(Markets::WBTC),
            "DOGE" => Some(Markets::DOGE),
            "SOL" => Some(Markets::SOL),
            "LTC" => Some(Markets::LTC),
            "UNI" => Some(Markets::UNI),
            "LINK" => Some(Markets::LINK),
            "ARB" => Some(Markets::ARB),
            "USDCE" => Some(Markets::USDCE),
            "USDT" => Some(Markets::USDT),
            "DAI" => Some(Markets::DAI),
            "XRP" => Some(Markets::XRP),
            _ => None,
        }
    }

    pub fn get_market_address(token_name: &str) -> Option<String> {
        Markets::from_token_name(token_name)
            .map(|market| market.info().market_address)
    }

    pub fn get_swap_path_for_collateral(collateral_token: &str) -> Vec<String> {
        if collateral_token != "USDC" {
            if let Some(market_address) = Markets::get_market_address(collateral_token) {
                vec![market_address]
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        }
    }
}

#[derive(Debug)]
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
    pub referral_code: String,
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
            referral_code: "0x0000000000000000000000000000000000000000000000000000000000000000".to_string(),
        }
    }
}

pub struct TokenInfo {
    pub name: &'static str,
    pub address: &'static str,
    pub decimals: u8,
}

pub enum Token {
    ETH,
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
            Token::ETH => TokenInfo { name: "ETH", address: "0x82aF49447D8a07e3bd95BD0d56f35241523fBab1", decimals: 18 },
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
            "ETH" => Some(Token::ETH),
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

    pub fn token_address_from_name(name: &str) -> Option<String> {
        Token::from_name(name)
            .map(|token| token.info().address.to_string())
    }
}

pub struct SimpleOrder {
    pub is_long: bool,
    pub index_token: String,
    pub collateral_amount: String,
    pub collateral_token: String,
    pub leverage_factor: f32,
}

pub struct SimpleClosePosition {
    pub reciever: String,
    pub index_token: String,
    pub collateral_token: String,

}

#[derive(Debug)]
pub struct MarketIncreaseOrderCalcOutput {
    pub is_long: bool,
    pub collateral_amount: U256,
    pub size_delta_usd: U256,
    pub initial_collateral_delta_amount: U256,
    pub trigger_price: U256,
    pub acceptable_price: U256,
    pub execution_fee: U256,
    pub min_output_amount: U256,
}

#[derive(Debug)]
pub struct MarketDecreaseOrderCalcOutput {
    pub size_delta_usd: U256,
    pub initial_collateral_delta_amount: U256,
    pub trigger_price: U256,
    pub acceptable_price: U256,
    pub execution_fee: U256,
    pub callback_gas_limit: U256,
    pub min_output_amount: U256,
}

#[derive(Debug, Clone)]
pub struct AddressesForMarketIncreaseOrder {
    pub receiver: String,
    pub callback_contract: String,
    pub ui_fee_receiver: String,
    pub market: String, 
    pub initial_collateral_token: String, 
    pub swap_path: Vec<String>, 
    pub referral_code: String, 
}

#[derive(Deserialize, Debug)]
pub struct PriceData {
    pub id: String,
    #[serde(rename = "minBlockNumber")]
    pub min_block_number: Option<u64>,
    #[serde(rename = "minBlockHash")]
    pub min_block_hash: Option<String>,
    #[serde(rename = "oracleDecimals")]
    pub oracle_decimals: Option<u8>,
    #[serde(rename = "tokenSymbol")]
    pub token_symbol: String,
    #[serde(rename = "tokenAddress")]
    pub token_address: String,
    #[serde(rename = "minPrice")]
    pub min_price: Option<String>,
    #[serde(rename = "maxPrice")]
    pub max_price: Option<String>,
    pub signer: Option<String>,
    pub signature: Option<String>,
    #[serde(rename = "signatureWithoutBlockHash")]
    pub signature_without_block_hash: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    #[serde(rename = "minBlockTimestamp")]
    pub min_block_timestamp: Option<u64>,
    #[serde(rename = "oracleKeeperKey")]
    pub oracle_keeper_key: Option<String>,
    #[serde(rename = "maxBlockTimestamp")]
    pub max_block_timestamp: Option<u64>,
    #[serde(rename = "maxBlockNumber")]
    pub max_block_number: Option<u64>,
    #[serde(rename = "maxBlockHash")]
    pub max_block_hash: Option<String>,
    #[serde(rename = "maxPriceFull")]
    pub max_price_full: Option<String>,
    #[serde(rename = "minPriceFull")]
    pub min_price_full: Option<String>,
    #[serde(rename = "oracleKeeperRecordId")]
    pub oracle_keeper_record_id: Option<String>,
    #[serde(rename = "oracleKeeperFetchType")]
    pub oracle_keeper_fetch_type: Option<String>,
    #[serde(rename = "oracleType")]
    pub oracle_type: Option<String>,
    pub blob: Option<String>,
}

#[derive(Deserialize)]
pub struct ApiResponse {
    #[serde(rename = "signedPrices")]
    pub signed_prices: Vec<PriceData>,
}

#[derive(Deserialize)]
pub struct TokenPriceFromApiResponse {
    pub token_symbol: String,
    pub min_price_full: String,
    pub max_price_full: String,
}

#[derive(Deserialize)]
pub struct GasPriceResponse {
    pub result: String,
}

pub struct KeyHashes {
    wnt: String,
    nonce: String,
    fee_receiver: String,
    holding_address: String,
    in_strict_price_feed_mode: String,
    min_handle_execution_error_gas: String,
    min_additional_gas_for_execution: String,
    min_handle_execution_error_gas_to_forward: String,
    max_leverage: String,
    market_list: String,
    deposit_list: String,
    account_deposit_list: String,
    withdrawal_list: String,
    account_withdrawal_list: String,
    position_list: String,
    account_position_list: String,
    order_list: String,
    account_order_list: String,
    subaccount_list: String,
    create_deposit_feature_disabled: String,
    cancel_deposit_feature_disabled: String,
    execute_deposit_feature_disabled: String,
    create_order_feature_disabled: String,
    execute_order_feature_disabled: String,
    execute_adl_feature_disabled: String,
    update_order_feature_disabled: String,
    cancel_order_feature_disabled: String,
    claimable_fee_amount: String,
    claimable_funding_amount: String,
    claimable_collateral_amount: String,
    claimable_collateral_factor: String,
    claimable_collateral_time_divisor: String,
    claimable_ui_fee_amount: String,
    affiliate_reward: String,
    max_ui_fee_factor: String,
    is_market_disabled: String,
    max_swap_path_length: String,
    min_market_tokens_for_first_deposit: String,
    min_oracle_block_confirmations: String,
    max_oracle_price_age: String,
    max_oracle_ref_price_deviation_factor: String,
    min_oracle_signers: String,
    min_collateral_factor: String,
    min_collateral_factor_for_open_interest_multiplier: String,
    min_collateral_usd: String,
    min_position_size_usd: String,
    swap_fee_receiver_factor: String,
    token_transfer_gas_limit: String,
    native_token_transfer_gas_limit: String,
    max_callback_gas_limit: String,
    request_expiration_block_age: String,
    price_feed: String,
    price_feed_multiplier: String,
    price_feed_heartbeat_duration: String,
    realtime_feed_id: String,
    realtime_feed_multiplier: String,
    stable_price: String,
    oracle_type: String,
    open_interest: String,
    open_interest_in_tokens: String,
    collateral_sum: String,
    pool_amount: String,
    max_pool_amount: String,
    max_pool_amount_for_deposit: String,
    max_open_interest: String,
    position_impact_pool_amount: String,
    min_position_impact_pool_amount: String,
    position_impact_pool_distribution_rate: String,
    position_impact_pool_distributed_at: String,
    swap_impact_pool_amount: String,
    position_fee_receiver_factor: String,
    borrowing_fee_receiver_factor: String,
    swap_fee_factor: String,
    swap_impact_factor: String,
    swap_impact_exponent_factor: String,
    position_impact_factor: String,
    position_impact_exponent_factor: String,
    max_position_impact_factor: String,
    max_position_impact_factor_for_liquidations: String,
    position_fee_factor: String,
    reserve_factor: String,
    open_interest_reserve_factor: String,
    max_pnl_factor: String,
    max_pnl_factor_for_traders: String,
    max_pnl_factor_for_adl: String,
    min_pnl_factor_after_adl: String,
    max_pnl_factor_for_deposits: String,
    max_pnl_factor_for_withdrawals: String,
    latest_adl_block: String,
    is_adl_enabled: String,
    funding_factor: String,
    funding_exponent_factor: String,
    saved_funding_factor_per_second: String,
    funding_increase_factor_per_second: String,
    funding_decrease_factor_per_second: String,
    min_funding_factor_per_second: String,
    max_funding_factor_per_second: String,
    threshold_for_stable_funding: String,
    threshold_for_decrease_funding: String,
    funding_fee_amount_per_size: String,
    claimable_funding_amount_per_size: String,
    funding_updated_at: String,
    borrowing_factor: String,
    borrowing_exponent_factor: String,
    skip_borrowing_fee_for_smaller_side: String,
    estimated_gas_fee_base_amount: String,
    estimated_gas_fee_multiplier_factor: String,
    execution_gas_fee_base_amount: String,
    execution_gas_fee_multiplier_factor: String,
    deposit_gas_limit: String,
    withdrawal_gas_limit: String,
    single_swap_gas_limit: String,
    increase_order_gas_limit: String,
    decrease_order_gas_limit: String,
    swap_order_gas_limit: String,
    cumulative_borrowing_factor: String,
    cumulative_borrowing_factor_updated_at: String,
    virtual_token_id: String,
    virtual_market_id: String,
    virtual_inventory_for_swaps: String,
    virtual_inventory_for_positions: String,
    max_allowed_subaccount_action_count: String,
    subaccount_action_count: String,
    subaccount_auto_top_up_amount: String,
    subaccount_order_action: String,
}

impl KeyHashes {
    pub fn new() -> Self {
        HashedStrings {
            wnt: hash_string("WNT"),
            nonce: hash_string("NONCE"),
            fee_receiver: hash_string("FEE_RECEIVER"),
            holding_address: hash_string("HOLDING_ADDRESS"),
            in_strict_price_feed_mode: hash_string("IN_STRICT_PRICE_FEED_MODE"),
            min_handle_execution_error_gas: hash_string("MIN_HANDLE_EXECUTION_ERROR_GAS"),
            min_additional_gas_for_execution: hash_string("MIN_ADDITIONAL_GAS_FOR_EXECUTION"),
            min_handle_execution_error_gas_to_forward: hash_string("MIN_HANDLE_EXECUTION_ERROR_GAS_TO_FORWARD"),
            max_leverage: hash_string("MAX_LEVERAGE"),
            market_list: hash_string("MARKET_LIST"),
            deposit_list: hash_string("DEPOSIT_LIST"),
            account_deposit_list: hash_string("ACCOUNT_DEPOSIT_LIST"),
            withdrawal_list: hash_string("WITHDRAWAL_LIST"),
            account_withdrawal_list: hash_string("ACCOUNT_WITHDRAWAL_LIST"),
            position_list: hash_string("POSITION_LIST"),
            account_position_list: hash_string("ACCOUNT_POSITION_LIST"),
            order_list: hash_string("ORDER_LIST"),
            account_order_list: hash_string("ACCOUNT_ORDER_LIST"),
            subaccount_list: hash_string("SUBACCOUNT_LIST"),
            create_deposit_feature_disabled: hash_string("CREATE_DEPOSIT_FEATURE_DISABLED"),
            cancel_deposit_feature_disabled: hash_string("CANCEL_DEPOSIT_FEATURE_DISABLED"),
            execute_deposit_feature_disabled: hash_string("EXECUTE_DEPOSIT_FEATURE_DISABLED"),
            create_order_feature_disabled: hash_string("CREATE_ORDER_FEATURE_DISABLED"),
            execute_order_feature_disabled: hash_string("EXECUTE_ORDER_FEATURE_DISABLED"),
            execute_adl_feature_disabled: hash_string("EXECUTE_ADL_FEATURE_DISABLED"),
            update_order_feature_disabled: hash_string("UPDATE_ORDER_FEATURE_DISABLED"),
            cancel_order_feature_disabled: hash_string("CANCEL_ORDER_FEATURE_DISABLED"),
            claimable_fee_amount: hash_string("CLAIMABLE_FEE_AMOUNT"),
            claimable_funding_amount: hash_string("CLAIMABLE_FUNDING_AMOUNT"),
            claimable_collateral_amount: hash_string("CLAIMABLE_COLLATERAL_AMOUNT"),
            claimable_collateral_factor: hash_string("CLAIMABLE_COLLATERAL_FACTOR"),
            claimable_collateral_time_divisor: hash_string("CLAIMABLE_COLLATERAL_TIME_DIVISOR"),
            claimable_ui_fee_amount: hash_string("CLAIMABLE_UI_FEE_AMOUNT"),
            affiliate_reward: hash_string("AFFILIATE_REWARD"),
            max_ui_fee_factor: hash_string("MAX_UI_FEE_FACTOR"),
            is_market_disabled: hash_string("IS_MARKET_DISABLED"),
            max_swap_path_length: hash_string("MAX_SWAP_PATH_LENGTH"),
            min_market_tokens_for_first_deposit: hash_string("MIN_MARKET_TOKENS_FOR_FIRST_DEPOSIT"),
            min_oracle_block_confirmations: hash_string("MIN_ORACLE_BLOCK_CONFIRMATIONS"),
            max_oracle_price_age: hash_string("MAX_ORACLE_PRICE_AGE"),
            max_oracle_ref_price_deviation_factor: hash_string("MAX_ORACLE_REF_PRICE_DEVIATION_FACTOR"),
            min_oracle_signers: hash_string("MIN_ORACLE_SIGNERS"),
            min_collateral_factor: hash_string("MIN_COLLATERAL_FACTOR"),
            min_collateral_factor_for_open_interest_multiplier: hash_string("MIN_COLLATERAL_FACTOR_FOR_OPEN_INTEREST_MULTIPLIER"),
            min_collateral_usd: hash_string("MIN_COLLATERAL_USD"),
            min_position_size_usd: hash_string("MIN_POSITION_SIZE_USD"),
            swap_fee_receiver_factor: hash_string("SWAP_FEE_RECEIVER_FACTOR"),
            token_transfer_gas_limit: hash_string("TOKEN_TRANSFER_GAS_LIMIT"),
            native_token_transfer_gas_limit: hash_string("NATIVE_TOKEN_TRANSFER_GAS_LIMIT"),
            max_callback_gas_limit: hash_string("MAX_CALLBACK_GAS_LIMIT"),
            request_expiration_block_age: hash_string("REQUEST_EXPIRATION_BLOCK_AGE"),
            price_feed: hash_string("PRICE_FEED"),
            price_feed_multiplier: hash_string("PRICE_FEED_MULTIPLIER"),
            price_feed_heartbeat_duration: hash_string("PRICE_FEED_HEARTBEAT_DURATION"),
            realtime_feed_id: hash_string("REALTIME_FEED_ID"),
            realtime_feed_multiplier: hash_string("REALTIME_FEED_MULTIPLIER"),
            stable_price: hash_string("STABLE_PRICE"),
            oracle_type: hash_string("ORACLE_TYPE"),
            open_interest: hash_string("OPEN_INTEREST"),
            open_interest_in_tokens: hash_string("OPEN_INTEREST_IN_TOKENS"),
            collateral_sum: hash_string("COLLATERAL_SUM"),
            pool_amount: hash_string("POOL_AMOUNT"),
            max_pool_amount: hash_string("MAX_POOL_AMOUNT"),
            max_pool_amount_for_deposit: hash_string("MAX_POOL_AMOUNT_FOR_DEPOSIT"),
            max_open_interest: hash_string("MAX_OPEN_INTEREST"),
            position_impact_pool_amount: hash_string("POSITION_IMPACT_POOL_AMOUNT"),
            min_position_impact_pool_amount: hash_string("MIN_POSITION_IMPACT_POOL_AMOUNT"),
            position_impact_pool_distribution_rate: hash_string("POSITION_IMPACT_POOL_DISTRIBUTION_RATE"),
            position_impact_pool_distributed_at: hash_string("POSITION_IMPACT_POOL_DISTRIBUTED_AT"),
            swap_impact_pool_amount: hash_string("SWAP_IMPACT_POOL_AMOUNT"),
            position_fee_receiver_factor: hash_string("POSITION_FEE_RECEIVER_FACTOR"),
            borrowing_fee_receiver_factor: hash_string("BORROWING_FEE_RECEIVER_FACTOR"),
            swap_fee_factor: hash_string("SWAP_FEE_FACTOR"),
            swap_impact_factor: hash_string("SWAP_IMPACT_FACTOR"),
            swap_impact_exponent_factor: hash_string("SWAP_IMPACT_EXPONENT_FACTOR"),
            position_impact_factor: hash_string("POSITION_IMPACT_FACTOR"),
            position_impact_exponent_factor: hash_string("POSITION_IMPACT_EXPONENT_FACTOR"),
            max_position_impact_factor: hash_string("MAX_POSITION_IMPACT_FACTOR"),
            max_position_impact_factor_for_liquidations: hash_string("MAX_POSITION_IMPACT_FACTOR_FOR_LIQUIDATIONS"),
            position_fee_factor: hash_string("POSITION_FEE_FACTOR"),
            reserve_factor: hash_string("RESERVE_FACTOR"),
            open_interest_reserve_factor: hash_string("OPEN_INTEREST_RESERVE_FACTOR"),
            max_pnl_factor: hash_string("MAX_PNL_FACTOR"),
            max_pnl_factor_for_traders: hash_string("MAX_PNL_FACTOR_FOR_TRADERS"),
            max_pnl_factor_for_adl: hash_string("MAX_PNL_FACTOR_FOR_ADL"),
            min_pnl_factor_after_adl: hash_string("MIN_PNL_FACTOR_AFTER_ADL"),
            max_pnl_factor_for_deposits: hash_string("MAX_PNL_FACTOR_FOR_DEPOSITS"),
            max_pnl_factor_for_withdrawals: hash_string("MAX_PNL_FACTOR_FOR_WITHDRAWALS"),
            latest_adl_block: hash_string("LATEST_ADL_BLOCK"),
            is_adl_enabled: hash_string("IS_ADL_ENABLED"),
            funding_factor: hash_string("FUNDING_FACTOR"),
            funding_exponent_factor: hash_string("FUNDING_EXPONENT_FACTOR"),
            saved_funding_factor_per_second: hash_string("SAVED_FUNDING_FACTOR_PER_SECOND"),
            funding_increase_factor_per_second: hash_string("FUNDING_INCREASE_FACTOR_PER_SECOND"),
            funding_decrease_factor_per_second: hash_string("FUNDING_DECREASE_FACTOR_PER_SECOND"),
            min_funding_factor_per_second: hash_string("MIN_FUNDING_FACTOR_PER_SECOND"),
            max_funding_factor_per_second: hash_string("MAX_FUNDING_FACTOR_PER_SECOND"),
            threshold_for_stable_funding: hash_string("THRESHOLD_FOR_STABLE_FUNDING"),
            threshold_for_decrease_funding: hash_string("THRESHOLD_FOR_DECREASE_FUNDING"),
            funding_fee_amount_per_size: hash_string("FUNDING_FEE_AMOUNT_PER_SIZE"),
            claimable_funding_amount_per_size: hash_string("CLAIMABLE_FUNDING_AMOUNT_PER_SIZE"),
            funding_updated_at: hash_string("FUNDING_UPDATED_AT"),
            borrowing_factor: hash_string("BORROWING_FACTOR"),
            borrowing_exponent_factor: hash_string("BORROWING_EXPONENT_FACTOR"),
            skip_borrowing_fee_for_smaller_side: hash_string("SKIP_BORROWING_FEE_FOR_SMALLER_SIDE"),
            estimated_gas_fee_base_amount: hash_string("ESTIMATED_GAS_FEE_BASE_AMOUNT"),
            estimated_gas_fee_multiplier_factor: hash_string("ESTIMATED_GAS_FEE_MULTIPLIER_FACTOR"),
            execution_gas_fee_base_amount: hash_string("EXECUTION_GAS_FEE_BASE_AMOUNT"),
            execution_gas_fee_multiplier_factor: hash_string("EXECUTION_GAS_FEE_MULTIPLIER_FACTOR"),
            deposit_gas_limit: hash_string("DEPOSIT_GAS_LIMIT"),
            withdrawal_gas_limit: hash_string("WITHDRAWAL_GAS_LIMIT"),
            single_swap_gas_limit: hash_string("SINGLE_SWAP_GAS_LIMIT"),
            increase_order_gas_limit: hash_string("INCREASE_ORDER_GAS_LIMIT"),
            decrease_order_gas_limit: hash_string("DECREASE_ORDER_GAS_LIMIT"),
            swap_order_gas_limit: hash_string("SWAP_ORDER_GAS_LIMIT"),
            cumulative_borrowing_factor: hash_string("CUMULATIVE_BORROWING_FACTOR"),
            cumulative_borrowing_factor_updated_at: hash_string("CUMULATIVE_BORROWING_FACTOR_UPDATED_AT"),
            virtual_token_id: hash_string("VIRTUAL_TOKEN_ID"),
            virtual_market_id: hash_string("VIRTUAL_MARKET_ID"),
            virtual_inventory_for_swaps: hash_string("VIRTUAL_INVENTORY_FOR_SWAPS"),
            virtual_inventory_for_positions: hash_string("VIRTUAL_INVENTORY_FOR_POSITIONS"),
            max_allowed_subaccount_action_count: hash_string("MAX_ALLOWED_SUBACCOUNT_ACTION_COUNT"),
            subaccount_action_count: hash_string("SUBACCOUNT_ACTION_COUNT"),
            subaccount_auto_top_up_amount: hash_string("SUBACCOUNT_AUTO_TOP_UP_AMOUNT"),
            subaccount_order_action: hash_string("SUBACCOUNT_ORDER_ACTION"),
        }
    }
}