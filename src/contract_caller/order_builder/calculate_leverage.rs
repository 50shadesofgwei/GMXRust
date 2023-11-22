pub fn calculate_leveraged_amount(amount: String, leverage_factor: f32) -> Result<u64, Box<dyn std::error::Error>> {
    if leverage_factor >= 50.0 {
        return Err("Leverage factor must be less than 50".into());
    }

    let amount_int: u64 = amount.parse()?;
    let leveraged_amount: f32 = amount_int as f32 * leverage_factor;
    Ok(leveraged_amount as u64)
}