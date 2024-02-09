use anchor_lang::prelude::*;

declare_id!("3zdpR6aw81LKw7GQpDboSMU9e4aaB3MC6KoUr4kBSWQT");


#[program]
pub mod price_util {
    use super::*;

    // // Corresponds to `updatePriceState` in Solidity
    pub fn update_price_state(ctx: Context<UpdatePriceState>, global_position: GlobalLiquidityPosition , price_state : PriceState , parameter : UpdatePriceStateParameter) -> Result<()> {
        // Function logic goes here
        Ok(())
    }

    // Additional functions can be added here
}

pub fn calculate_premium_price_after(step : &mut SimulateMoveStep , reached : bool , size_usd : u128 ) -> Result<u128>{
    
    if reached {
        return Ok(step.to.premium_rate_x96);
    }


    Ok(100)
}

pub fn calculate_ax248_and_bx96(side : bool , from :  PriceVertex , to : PriceVertex) -> (u128 , i128){
    let mut to_new = to;
    let mut from_new = from;
    if from_new.size > to_new.size {
        let temp = from_new;
        from_new = to_new;
        to_new = temp;
    }

    if to_new.premium_rate_x96 >= from_new.premium_rate_x96 {
        return (0 ,0);
    }

    let size_delta = to_new.size - from_new.size;
    let ax348 = (to_new.premium_rate_x96 - from_new.premium_rate_x96) * 100 /size_delta;
    let mut bx96: i128 = 0;
    let numerator_part_1_x96 = (from_new.premium_rate_x96 as u128) *to_new.size; 
    let numerator_part_2_x96 = (to_new.premium_rate_x96 as u128) *from_new.size;
    if side {
        if numerator_part_1_x96 >= numerator_part_2_x96 {
            bx96 = ((numerator_part_1_x96 - numerator_part_2_x96) / size_delta) as i128 ;
        }

        else {
            bx96 = ((numerator_part_2_x96 - numerator_part_1_x96) / size_delta) as i128;
        }

    }

    else {
        if numerator_part_2_x96 >= numerator_part_1_x96 {
            bx96 = ((numerator_part_2_x96 - numerator_part_1_x96) / size_delta) as i128;
        }

        else {
            bx96 = -((numerator_part_2_x96 as i128 - numerator_part_1_x96 as i128) / size_delta as i128);
        }

    }

    return (ax348 , bx96);


    (100 , 100)
}

#[error_code]
pub enum Errors {
    #[msg("Unauthorized access")]
    CallerUnauthorized,
    #[msg("Invalid operation")]
    InvalidOperation,
    #[msg("Already Initilized")]
    AlreadyInitlized,
    #[msg("Insufficient execution fee")]
    InsufficientExecutionFee,
    #[msg("Cannot cancel")]
    CannotCancel,
}


#[derive(Accounts)]
pub struct UpdatePriceState<> {
    // Define the accounts needed for the update_price_state function
    // This includes accounts that will be read from or written to
}

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
    pub to: PriceVertex,
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


