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
