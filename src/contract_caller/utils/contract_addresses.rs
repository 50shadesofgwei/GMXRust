use alloy_primitives::{address, fixed_bytes, Address, FixedBytes, I256, U256};
use lazy_static::lazy_static;

const USDC_STR: &str = "0xaf88d065e77c8cC2239327C5EDb3A432268e5831";
const WETH_STR: &str = "0x82aF49447D8a07e3bd95BD0d56f35241523fBab1";


lazy_static! {
    pub static ref USDC: Address = Address::parse_checksummed(USDC_STR, None).unwrap();
    pub static ref WETH: Address = Address::parse_checksummed(WETH_STR, None).unwrap();
}