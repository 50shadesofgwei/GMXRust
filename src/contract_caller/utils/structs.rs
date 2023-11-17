use alloy_primitives::{hex, Address, U256};
use alloy_sol_types::{sol, SolEnum, SolType};

fn struct_builder() {
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
        CreateOrderParamsAddresses addresses;
        CreateOrderParamsNumbers numbers;
        Order.OrderType orderType;
        Order.DecreasePositionSwapType decreasePositionSwapType;
        bool isLong;
        bool shouldUnwrapNativeToken;
        bytes32 referralCode;
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