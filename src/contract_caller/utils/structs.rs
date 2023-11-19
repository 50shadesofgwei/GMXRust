use alloy_sol_types::sol;
use ethers::prelude::*;

// Struct for 'addresses' parameter in 'createOrder'
pub struct AddressesStruct {
    pub receiver: Address,
    pub callback_contract: Address,
    pub ui_fee_receiver: Address,
    pub market: Address,
    pub initial_collateral_token: Address,
    pub swap_path: Vec<Address>,
}

// Struct for 'numbers' parameter in 'createOrder'
pub struct NumbersStruct {
    pub size_delta_usd: U256,
    pub initial_collateral_delta_amount: U256,
    pub trigger_price: U256,
    pub acceptable_price: U256,
    pub execution_fee: U256,
    pub callback_gas_limit: U256,
    pub min_output_amount: U256,
}


pub fn struct_builder() {

    enum OrderType {
        MarketSwap,
        LimitSwap,
        MarketIncrease,
        LimitIncrease,
        MarketDecrease,
        LimitDecrease,
        StopLossDecrease,
        Liquidation,
    }

    enum DecreasePositionSwapType {
        NoSwap,
        SwapPnlTokenToCollateralToken,
        SwapCollateralTokenToPnlToken
    }

    sol! {
        struct CreateDepositParams {
            address receiver;
            address callback_contract;
            address ui_fee_receiver;
            address market;
            address initial_long_token;
            address initial_short_token;
            address[] long_token_swap_path;
            address[] short_token_swap_path;
            uint256 min_market_tokens;
            bool should_unwrap_native_token;
            uint256 execution_fee;
            uint256 callback_gas_limit;
        }
    }
    
    sol! {
        struct CreateOrderParamsAddresses {
            address receiver;
            address callbackContract;
            address uiFeeReceiver;
            address market;
            address initialCollateralToken;
            address[] swapPath;
        }
    }
    
    sol! {
        struct CreateOrderParamsNumbers {
            uint256 size_delta_usd;
            uint256 initial_collateral_delta_amount;
            uint256 trigger_price;
            uint256 acceptable_price;
            uint256 execution_fee;
            uint256 callback_gas_limit;
            uint256 min_output_amount;
        }
    }
    
}