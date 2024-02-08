use anchor_lang::prelude::*;

declare_id!("3zdpR6aw81LKw7GQpDboSMU9e4aaB3MC6KoUr4kBSWQT");


#[program]
pub mod price_util {
    use super::*;

    // // Corresponds to `updatePriceState` in Solidity
    // pub fn update_price_state(ctx: Context<UpdatePriceState>, parameter: UpdatePriceStateParameter) -> Result<()> {
    //     // Function logic goes here
    //     Ok(())
    // }

    // Additional functions can be added here
}

// #[derive(Accounts)]
// pub struct UpdatePriceState<'info> {
//     // Define the accounts needed for the update_price_state function
//     // This includes accounts that will be read from or written to
// }

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct UpdatePriceStateParameter {
    // Translated from the Solidity struct
    // In this case, you might need to replace IMarketDescriptor with Pubkey or another suitable type
    pub market: Pubkey,
    pub side: bool, // Side might be an enum you need to define based on your application logic
    pub size_delta: u128,
    pub index_price_x96: u64,
    pub liquidation_vertex_index: u8,
    pub liquidation: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct SimulateMoveStep {
    // Similarly translate fields from Solidity to Rust
    pub side: bool,
    pub size_left: u128,
    pub index_price_x96: u64,
    pub basis_index_price_x96: u64,
    pub improve_balance: bool,
    // Convert addresses to Pubkey or other suitable types
    pub from: Pubkey,
    pub current: Pubkey,
    pub to: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct PriceStateCache {
    pub premium_rate_x96: u128,
    pub pending_vertex_index: u8,
    pub liquidation_vertex_index: u8,
    pub current_vertex_index: u8,
    pub basis_index_price_x96: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct GlobalLiquidityPosition {
    /// The size of the net position held by all LPs
    pub net_size: u128,
    /// The size of the net position held by all LPs in the liquidation buffer
    pub liquidation_buffer_net_size: u128,
    /// The Previous Settlement Point Price, as a Q64.96
    pub previous_sp_price_x96: u128, // Adjusted from uint160 for simplicity
    /// The side of the position (Long or Short)
    pub side: bool,
    /// The total liquidity of all LPs
    pub liquidity: u128,
    /// The accumulated unrealized Profit and Loss (PnL) growth per liquidity unit, as a Q192.64
    pub unrealized_pnl_growth_x64: i128, // Note: Check if i128 suffices or if a custom type is needed
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct LiquidityPosition {
    /// The margin of the position
    pub margin: u128,
    /// The liquidity (value) of the position
    pub liquidity: u128,
    /// The snapshot of `GlobalLiquidityPosition.realizedProfitGrowthX64` at the time of the position was opened
    pub entry_unrealized_pnl_growth_x64: i128,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct PriceVertex {
    pub size: u128,
    pub premium_rate_x96: u128,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct PriceState {
    pub premium_rate_x96: u128,
    pub pending_vertex_index: u8,
    pub current_vertex_index: u8,
    pub basis_index_price_x96: u128, // Adjusted for simplicity
    pub price_vertices: [PriceVertex; 10],
    pub liquidation_buffer_net_sizes: [u128; 10],
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct GlobalLiquidationFund {
    pub liquidation_fund: i128, // Note: Solana doesn't directly support int256
    pub liquidity: u128, // Adjusted for simplicity
}

// Simplified representation without mappings
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct State {
    pub price_state: PriceState,
    pub usd_balance: u128,
    pub protocol_fee: u128,
    // Omitting mappings - consider Solana accounts for these
    pub global_liquidity_position: GlobalLiquidityPosition,
    // Other fields...
}


