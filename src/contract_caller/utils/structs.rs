use alloy_primitives::{hex, Address, U256};
use alloy_sol_types::{sol, SolEnum, SolType};

fn struct_builder() {

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

    sol! {
        struct CreateDepositParams {
            receiver: Address,
            callback_contract: Address,
            ui_fee_receiver: Address,
            market: Address,
            initial_long_token: Address,
            initial_short_token: Address,
            long_token_swap_path: Vec<Address>,
            short_token_swap_path: Vec<Address>,
            min_market_tokens: U256,
            should_unwrap_native_token: bool,
            execution_fee: U256,
            callback_gas_limit: U256,
        }
    }

    sol! {
        struct CreateOrderParams {
            addresses: CreateOrderParamsAddresses,
            numbers: CreateOrderParamsNumbers,
            order_type: OrderType, // Define OrderType enum based on your contract
            decrease_position_swap_type: DecreasePositionSwapType, // Define DecreasePositionSwapType enum based on your contract
            is_long: bool,
            should_unwrap_native_token: bool,
            referral_code: [u8; u32], // Or appropriate type for referral code
        }
    }

    sol! {
        struct CreateOrderParamsAddresses {
            receiver: Address,
            callbackContract: Address,
            uiFeeReceiver: Address,
            market: Address,
            initialCollateralToken: Address,
            swapPath: Vec<Address>,
        }
    }

    sol! {
        struct CreateOrderParamsNumbers {
            size_delta_usd: U256,
            initial_collateral_delta_amount: U256,
            trigger_price: U256,
            acceptable_price: U256,
            execution_fee: U256,
            callback_gas_limit: U256,
            min_output_amount: U256,
        }
    }
}