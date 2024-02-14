use anchor_lang::prelude::*;

#[account]
pub struct GlobalPosition {
    pub long_size: u128,
    pub short_size: u128,
    pub max_size: u128,
    pub max_size_per_position: u128,
    pub long_funding_rate_growth_x96: i128, // Adjusted to i128 for compatibility
    pub short_funding_rate_growth_x96: i128, // Adjusted to i128 for compatibility
}

#[account]
pub struct PreviousGlobalFundingRate {
    pub long_funding_rate_growth_x96: i128, // Adjusted to i128 for compatibility
    pub short_funding_rate_growth_x96: i128, // Adjusted to i128 for compatibility
}

#[account]
pub struct GlobalFundingRateSample {
    pub last_adjust_funding_rate_time: u64,
    pub sample_count: u16,
    pub cumulative_premium_rate_x96: i128, // Adjusted to i128 for compatibility
}

#[account]
pub struct Position {
    pub margin: u128,
    pub size: u128,
    pub entry_price_x96: u128, // Adjusted to u128 for simplicity; consider using a custom type or handling for Q64.96 fixed-point numbers.
    pub entry_funding_rate_growth_x96: i128, // Adjusted to i128 for compatibility
}
