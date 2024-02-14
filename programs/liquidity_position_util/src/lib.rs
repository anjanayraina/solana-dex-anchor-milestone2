use anchor_lang::prelude::*;
mod helper;
mod math;
mod liquidity_position_util;
mod position_util;
pub mod interfaces{
    pub mod IConfigurable;
pub mod IMarketLiquidityPosition; 
pub mod IMarketPosition;}
// use crate::interfaces::IConfigurable;
// use crate::interfaces::IMarketLiquidityPosition;
// use crate::interfaces::IMarketPosition;
// use crate::liquidity_position_util as helper_liquidity_position;
// use crate::position_util as helper_position_util;
// use crate::helper as other_helper;
// use crate::math as helper_math;

declare_id!("XNiBJSgxaaUkYfX8outPevtBcmao6LV1UrTQiyJ2YJs");


pub mod liquidity_position_uti {
    use super::*;
    pub fn update_price_state(ctx: Context<UpdatePriceState>) -> Result<()> {
        // Function logic goes here
        Ok(())
    }

    pub fn increase_liquidity_position(state : &mut State , market_congif :&mut MarketConfig , parameter : &mut IncreaseLiquidityPositionContext , index : usize) -> (u128) {
        // call using MarketUtil , add after wards 
        let base_config = &mut market_congif.base_config;
        let position_cache = &mut state.liquidity_positions.get(index).unwrap();
        let global_liquidity_positions = &mut state.global_liqudity_position;
        let mut reaized_pnl = 0;
        if true { //  add this after all the libraries are created 
            if true{ // check for liquidity delta 0 , if yes then revert 

            }

        }
        else {
            // reaized_pnl = _calculate_realized_pnl(global_liquidity_positions , position_cache); // uncomment this when the other libraries are created 
        }
        let mut margin_after_int = 0;
        margin_after_int = position_cache.margin + parameter.margin_delta;
        margin_after_int+=reaized_pnl;
        // revert if insufficient margin 
        let mut margin_after = margin_after_int;
        let mut liquidity_after = position_cache.liquidity;
        if parameter.liquidity_delta > 0 
        {
            liquidity_after+=parameter.liquidity_delta;
            // call to market util
            global_liquidity_positions.liquidity = global_liquidity_positions.liquidity + parameter.liquidity_delta;

        }

        // add validate position call 
        if let Some(position) = state.liquidity_positions.get_mut(index) {
            position.margin = margin_after;
            position.liquidity = liquidity_after;
            position.entry_unreaized_pnl_growth = global_liquidity_positions.entry_unreaized_pnl_growth;

        } else {
            // Handle the case where index is out of bounds
        }

        return 100; 
    }

    pub fn decrease_liquidity_position(
        state: &mut State,
        market_config: &MarketConfig,
        parameter: &DecreaseLiquidityPositionParameter,
    ) -> (u128, u128) {
        let position = state.liquidity_positions.get_mut(parameter.account_index).unwrap(); // Error if index is out of bounds
    
        if position.liquidity == 0 {
            // error statement 
                }
    
        if position.liquidity < parameter.liquidity_delta {
            // error statement 
        }
    
        let realized_pnl_delta = 0; // Placeholder for realized PnL calculation logic
    
        let margin_after_int = position.margin as i128 + parameter.margin_delta as i128 + realized_pnl_delta;
        if margin_after_int < 0 {
            return (0, 0 ) // Replace with a more specific error
        }
    
        let liquidity_after = position.liquidity.checked_sub(parameter.liquidity_delta).unwrap(); // Replace with a more specific error if subtraction underflows
    
        // decrease global liquidity
    
        // Apply changes to the position
        position.margin = margin_after_int as u128;
        position.liquidity = liquidity_after;
    
        
    
        return (position.margin, parameter.margin_delta); // Return updated margin and original margin delta
    }
    
    pub fn decrease_global_liquidity(
        global_liquidity_position: &mut GlobalLiquidityPosition, // mutable reference is necessary to modify the struct
        global_position: &GlobalPosition, // immutable reference since we're only reading from this struct
        liquidity_delta: u128,
    ) -> Result<()> {
        let liquidity_after = global_liquidity_position.liquidity.checked_sub(liquidity_delta).unwrap();
    
        if liquidity_after == 0 && (global_position.long_size | global_position.short_size) > 0 {
            
        }
    
        global_liquidity_position.liquidity = liquidity_after; // modifying the struct, hence &mut is necessary
    
        Ok(())
    }

}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct IncreaseLiquidityPositionContext {
    margin_delta : u128, 
    liquidity_delta : u128 , 
    
}

// Define your structs here as needed.

pub struct DecreaseLiquidityPositionParameter {
    pub account_index: usize,
    pub margin_delta: u128,
    pub liquidity_delta: u128,
    // Include other parameters as needed
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
struct MarketDescriptor {
    // Define fields according to your Solidity IMarketDescriptor
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
struct PriceFeed {
    // Define fields according to your Solidity IPriceFeed
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct GlobalLiquidityPosition {
    net_size: u128,
    liquidation_buffer_net_size: u128,
    liquidity : u128 , 
    entry_unreaized_pnl_growth : u128 , 
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct PriceState {
    premium_rate_x96: u128,
    pending_vertex_index: u8,
    current_vertex_index: u8,
    basis_index_price_x96: u128,
    // Additional fields...
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct UpdatePriceStateParameter {
    market: MarketDescriptor,
    account: Pubkey, // Address in Solana
    margin_delta: u128,
    liquidity_delta: u128,
    price_feed: PriceFeed,
    // Additional fields for Decrease and Liquidate...
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct PriceStateCache {
    premium_rate_x96: u128,
    pending_vertex_index: u8,
    liquidation_vertex_index: u8,
    current_vertex_index: u8,
    basis_index_price_x96: u64,
    // Additional fields...
}

#[derive(Accounts)]
pub struct UpdatePriceState {
    // Define other necessary accounts here
}



#[account]
pub struct MarketConfig {
    pub base_config: MarketBaseConfig,
    pub fee_rate_config: MarketFeeRateConfig,
    pub price_config: MarketPriceConfig,
}

#[account]
pub struct MarketBaseConfig {
    pub min_margin_per_liquidity_position: u64,
    pub max_leverage_per_liquidity_position: u32,
    pub liquidation_fee_rate_per_liquidity_position: u32,
    pub min_margin_per_position: u64,
    pub max_leverage_per_position: u32,
    pub liquidation_fee_rate_per_position: u32,
    pub max_position_liquidity: u128,
    pub max_position_value_rate: u32,
    pub max_size_rate_per_position: u32,
    pub liquidation_execution_fee: u64,
    pub interest_rate: u32,
    pub max_funding_rate: u32,
}

#[account]
pub struct MarketFeeRateConfig {
    pub trading_fee_rate: u32,
    pub protocol_fee_rate: u32,
    pub referral_return_fee_rate: u32,
    pub referral_parent_return_fee_rate: u32,
    pub referral_discount_rate: u32,
}

#[account]
pub struct VertexConfig {
    pub balance_rate: u32,
    pub premium_rate: u32,
}

#[account]
pub struct MarketPriceConfig {
    pub max_price_impact_liquidity: u128,
    pub liquidation_vertex_index: u8,
    pub vertices: [VertexConfig; 10],
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct State {
    // This would be stored directly in the State account.
    pub price_state: PriceState,
    pub usd_balance: u128,
    pub protocol_fee: u128,
    pub liquidity_positions : Vec<AccountToLiquidity> , 
    pub global_liqudity_position : GlobalLiquidityPosition,
    // Referral fees, liquidity positions, positions, and liquidation fund positions
    // would be managed through PDAs or alternative data structures.
}

// Assume these structs are defined elsewhere in your module.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]

pub struct AccountToLiquidity {
    account : Pubkey , 
    liquidity : u128, 
    margin : u128 , 
    entry_unreaized_pnl_growth : u128,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct GlobalPosition {
    // Fields representing global position...
    long_size: u128, 
    short_size : u128 , 
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct PreviousGlobalFundingRate {
    // Fields...
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct GlobalFundingRateSample {
    // Fields...
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct GlobalLiquidationFund {
    pub liquidation_fund: i128, // Assuming use of i128 for simplicity.
    pub liquidity: u128,
}