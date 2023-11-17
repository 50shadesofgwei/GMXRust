use alloy_primitives::{Address};
use alloy_sol_macro::sol;
use alloy_sol_types::sol_data;

type U256 = alloy_sol_types::sol_data::Uint<256>;


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
            Address receiver;
            Address callback_contract;
            Address ui_fee_receiver;
            Address market;
            Address initial_long_token;
            Address initial_short_token;
            Address[] long_token_swap_path;
            Address[] short_token_swap_path;
            U256 min_market_tokens; // Changed from U256 to Uint<256>
            bool should_unwrap_native_token;
            U256 execution_fee; // Changed from U256 to Uint<256>
            U256 callback_gas_limit; // Changed from U256 to Uint<256>
        }
    }
    
    sol! {
        struct CreateOrderParams {
            CreateOrderParamsAddresses addresses;
            CreateOrderParamsNumbers numbers;
            OrderType order_type; // Define OrderType enum based on your contract
            DecreasePositionSwapType decrease_position_swap_type; // Define DecreasePositionSwapType enum based on your contract
            bool is_long;
            bool should_unwrap_native_token;
            [u8; u32] referral_code; // Or appropriate type for referral code
        }
    }
    
    sol! {
        struct CreateOrderParamsAddresses {
            Address receiver;
            Address callbackContract;
            Address uiFeeReceiver;
            Address market;
            Address initialCollateralToken;
            Vec<Address> swapPath;
        }
    }
    
    sol! {
        struct CreateOrderParamsNumbers {
            U256 size_delta_usd;
            U256 initial_collateral_delta_amount;
            U256 trigger_price;
            U256 acceptable_price;
            U256 execution_fee;
            U256 callback_gas_limit;
            U256 min_output_amount;
        }
    }
    
}