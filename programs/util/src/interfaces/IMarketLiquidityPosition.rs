use anchor_lang::prelude::*;



pub struct GlobalLiquidityPosition {
    pub net_size: u128,
    pub liquidation_buffer_net_size: u128,
    // Assuming previousSPPriceX96 can be represented as u128 for simplicity,
    // but you may need to handle fixed-point arithmetic separately.
    pub previous_sp_price_x96: u128,
    pub side: bool,
    pub liquidity: u128,
    // Assuming unrealizedPnLGrowthX64 can be represented as i128 for simplicity,
    // actual fixed-point operations need to consider the scale factor.
    pub unrealized_pnl_growth_x64: i128,
}


pub struct LiquidityPosition {
    pub margin: u128,
    pub liquidity: u128,
    // Again, assuming entryUnrealizedPnLGrowthX64 as i128 for simplicity.
    pub entry_unrealized_pnl_growth_x64: i128,
}

