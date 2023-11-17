use alloy_sol_types::sol;

pub fn func_builder() {

    sol! {
        function createOrder(
        BaseOrderUtils.CreateOrderParams calldata params
    ) external payable nonReentrant returns (bytes32) {
        address account = msg.sender;

        return orderHandler.createOrder(
            account,
            params
        );
    }
    }
}