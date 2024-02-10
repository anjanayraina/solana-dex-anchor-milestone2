use anchor_lang::prelude::*;

declare_id!("XNiBJSgxaaUkYfX8outPevtBcmao6LV1UrTQiyJ2YJs");


pub mod liquidity_position_util {
    use super::*;
    pub fn update_price_state(ctx: Context<UpdatePriceState>) -> Result<()> {
        // Function logic goes here
        Ok(())
    }

    pub fn increase_liquidity_position(state : State , market_congif :&mut MarketConfig , parameter : IncreaseLiquidityPositionContext) -> (u128) {
        // call using MarketUtil , add after wards 
        let base_config = &mut market_congif.base_config;
        // let position_cache = &mut state.li
        return 100; 
    }


}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct IncreaseLiquidityPositionContext {

}

// Define your structs here as needed.



#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
struct MarketDescriptor {
    // Define fields according to your Solidity IMarketDescriptor
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
struct PriceFeed {
    // Define fields according to your Solidity IPriceFeed
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
struct GlobalLiquidityPosition {
    net_size: u128,
    liquidation_buffer_net_size: u128,
    // Additional fields...
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
    pub liquidity_positions : Vec<AccountToLiquidity>
    // Referral fees, liquidity positions, positions, and liquidation fund positions
    // would be managed through PDAs or alternative data structures.
}

// Assume these structs are defined elsewhere in your module.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]

pub struct AccountToLiquidity {
    account : Pubkey , 
    liquidity : u128, 
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct GlobalPosition {
    // Fields representing global position...
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