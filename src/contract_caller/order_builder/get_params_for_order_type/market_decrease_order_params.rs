use crate::contract_caller::utils::local_signer::get_local_signer;
use crate::contract_caller::utils::structs::{MarketDecreaseOrderCalcOutput, Token, TokenInfo, AddressesForMarketIncreaseOrder, OrderObject, Markets, SimpleClosePosition};
use ethers::signers::Signer;
use ethers::types::U256;
use crate::contract_caller::order_builder::get_price::fetch_token_price;
use crate::contract_caller::utils::gas_calculator::calculate_execution_fee;

pub async fn calculate_market_decrease_order_params(input: &SimpleClosePosition) -> Result<MarketDecreaseOrderCalcOutput, Box<dyn std::error::Error>> {
    const USD_SCALE_FACTOR: u32 = 30;

    let trigger_price: U256 = U256::from(0);
    let min_output_amount: U256 = U256::from(0);
    let estimated_gas: u64 = 3000000000000000;
    let estimated_gas_u256: U256 = U256::from(estimated_gas);
    let collateral_info: TokenInfo = Token::from_name(&input.collateral_token)
        .ok_or("Unsupported token")?
        .info();

    let collateral_amount_raw: U256 = U256::from_dec_str(&input.collateral_amount)?;
    let decimal_adjusted_value: U256 = collateral_amount_raw
    .checked_div(U256::exp10(collateral_info.decimals as usize))
    .ok_or("Conversion to USD value error")?;
    let price_output = fetch_token_price(input.index_token.clone()).await?;
    let acceptable_price: U256 = U256::from_dec_str(&price_output.min_price_full)?;
    
}