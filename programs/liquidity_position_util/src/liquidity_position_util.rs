use anchor_lang::prelude::*;



pub enum Rounding {
    Up,
    Down,
}

/// Returns the maximum of two values.
pub fn max(a: u128, b: u128) -> u128 {
    std::cmp::max(a, b)
}

/// Returns the minimum of two values.
pub fn min(a: u128, b: u128) -> u128 {
    std::cmp::min(a, b)
}

/// Calculates `a / b` with rounding up.
pub fn ceil_div(a: u128, b: u128) -> u128 {
    if b == 0 {
        panic!("Division by zero");
    }

    if a == 0 {
        0
    } else {
        (a - 1) / b + 1
    }
}

/// Calculates `x * y / denominator` with rounding down.
pub fn mul_div(x: u128, y: u128, denominator: u128) -> u128 {
    x.checked_mul(y).expect("Multiplication overflow")
     .checked_div(denominator).expect("Division overflow")
}

/// Calculates `x * y / denominator` with rounding up.
pub fn mul_div_up(x: u128, y: u128, denominator: u128) -> u128 {
    if denominator == 0 {
        panic!("Division by zero");
    }

    x.checked_mul(y).expect("Multiplication overflow")
     .checked_add(denominator - 1) // Add the denominator - 1 before division for rounding up
     .expect("Addition overflow")
     .checked_div(denominator).expect("Division overflow")
}

/// Calculates `x * y / denominator` with specific rounding.
pub fn mul_div_rounding(x: u128, y: u128, denominator: u128, rounding: Rounding) -> u128 {
    match rounding {
        Rounding::Up => mul_div_up(x, y, denominator),
        Rounding::Down => mul_div(x, y, denominator),
    }
}

/// Calculates `x * y / denominator` with both rounding down and up.
pub fn mul_div2(x: u128, y: u128, denominator: u128) -> (u128, u128) {
    let result = mul_div(x, y, denominator);
    let result_up = if x.checked_mul(y).expect("Multiplication overflow")
                     .checked_rem(denominator).expect("Remainder calculation overflow") > 0 {
        result + 1
    } else {
        result
    };

    (result, result_up)
}



    pub fn update_price_state(ctx: Context<UpdatePriceState>) -> Result<()> {
        // Function logic goes here
        Ok(())
    }

    pub fn increase_liquidity_position(state : &mut State , market_congif :&mut MarketConfig , parameter : &mut IncreaseLiquidityPositionContext , index : usize , position_cache : &mut Position ) -> Result<u128> {
        // call using MarketUtil , add after wards 
        let base_config = &mut market_congif.base_config;
        let position_cache = &mut state.liquidity_positions.get(index).unwrap();
        let global_liquidity_positions = &mut state.global_liqudity_position;
        let mut reaized_pnl = 0;
        if position_cache.liquidity == 0  { //  add this after all the libraries are created 
            if parameter.liquidity_delta == 0 { 
                // check for liquidity delta 0 , if yes then revert 
                return err!(ErrorCode::LiquidityPositionNotFound);
                // market util call 
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

        Ok(margin_after_int)
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
    
        let liquidity_after = position.liquidity.checked_sub(parameter.liquidity_delta).unwrap(); 
    
        // decrease global liquidity
    
        // Apply changes to the position
        position.margin = margin_after_int as u128;
        position.liquidity = liquidity_after;
    
        
    
        return (position.margin, parameter.margin_delta); 
    }
    
    pub fn decrease_global_liquidity(
        global_liquidity_position: &mut GlobalLiquidityPosition, 
        global_position: &GlobalPosition, 
        liquidity_delta: u128,
    ) -> Result<()> {
        let liquidity_after = global_liquidity_position.liquidity.checked_sub(liquidity_delta).unwrap();
    
        if liquidity_after == 0 && (global_position.long_size | global_position.short_size) > 0 {
            
        }
    
        global_liquidity_position.liquidity = liquidity_after; 
    
        Ok(())
    }

    pub fn _decrease_global_liquidity(
        global_liquidity_position: &mut GlobalLiquidityPosition,
        global_position: &GlobalPosition,
        liquidity_delta: u128,
    ) -> Result<()> {
        if global_liquidity_position.liquidity < liquidity_delta {
            return err!(ErrorCode::Underflow);
        }
        let liquidity_after = global_liquidity_position.liquidity.checked_sub(liquidity_delta)
            .ok_or(ErrorCode::Underflow)?;
        
        if liquidity_after == 0 && (global_position.long_size | global_position.short_size) > 0 {
            return err!(ErrorCode::LastLiquidityPositionCannotBeClosed);
        }
        
        global_liquidity_position.liquidity = liquidity_after;
        
        Ok(())
    }

    pub fn validate_liquidity_position_risk_rate(
        base_cfg: &MarketBaseConfig,
        margin: i128,
        liquidity: u128,
        liquidatable_position: bool,
    ) -> Result<()> {
        let maintenance_margin = ((liquidity as u128)
            .checked_mul(base_cfg.liquidation_fee_rate_per_liquidity_position as u128).unwrap()
            / 10_000)  //  add the actual bp from constants when it gets added 
            .checked_add(base_cfg.liquidation_execution_fee as u128).unwrap();
        
        if !liquidatable_position {
            if margin < 0 || (maintenance_margin as i128) >= margin {
                return err!(ErrorCode::RiskRateTooHigh);
            }
        } else {
            if margin >= 0 && (maintenance_margin as i128) < margin {
                return err!(ErrorCode::RiskRateTooLow);
            }
        }
    
        Ok(())
    }

    pub fn calculate_realized_pnl(
        global_liquidity_position: &GlobalLiquidityPosition,
        position_cache: &LiquidityPosition,
    ) -> Result<i128> {
        let unrealized_pnl_growth_delta_x64 = global_liquidity_position.entry_unreaized_pnl_growth;
            - position_cache.entry_unrealized_pnl_growth_x64;
    
        let realized_pnl = if unrealized_pnl_growth_delta_x64 >= 0 {
            mul_div(
                unrealized_pnl_growth_delta_x64 as u128,
                position_cache.liquidity as u128,
                100, // change it to q64 when constants are added 
            ) as i128
        } else {
            -(mul_div_up(
                unrealized_pnl_growth_delta_x64 as u128,
                position_cache.liquidity as u128,
                100, // Constants::Q64 equivalent
            ) as i128)
        };
    
        Ok(realized_pnl)
    }
    



#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct IncreaseLiquidityPositionContext {
    margin_delta : u128, 
    liquidity_delta : u128 , 
    
}

pub struct DecreaseLiquidityPositionParameter {
    pub account_index: usize,
    pub margin_delta: u128,
    pub liquidity_delta: u128,
    // Include other parameters as needed
}
pub struct LiquidityPosition {
    pub margin: u128,
    pub liquidity: u128,
    pub entry_unrealized_pnl_growth_x64: i128,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
struct MarketDescriptor {
   
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
struct PriceFeed {
    
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

pub struct Position {
    pub margin: u128,
    pub size: u128,
    pub entry_price_x96: u128, 
    pub entry_funding_rate_growth_x96: i128, 
}

#[error_code]
pub enum ErrorCode {
    #[msg("The margin rate is too high.")]
    MarginRateTooHigh,
    #[msg("The margin rate is too low.")]
    MarginRateTooLow,
    #[msg("Overflow occurred.")]
    Overflow,
    #[msg("Underflow occurred.")]
    Underflow,
    #[msg("Insufficient global liquidity.")]
    InsufficientGlobalLiquidity,
    #[msg("Size Excedded")] 
    SizeExceedsMaxSize , 
    #[msg("Size Excedded per position ")] 
    SizeExceedsMaxSizePerPosition , 
    #[msg("InsufficientSizeToDecrease")]
    InsufficientSizeToDecrease , 
    #[msg("InsufficientMargin")]
    InsufficientMargin,
    #[msg("LiquidityPositionNotFound")]
    LiquidityPositionNotFound,
    #[msg("LastLiquidityPositionCannotBeClosed")] 
    LastLiquidityPositionCannotBeClosed,
    #[msg("RiskRateTooHigh")] 
    RiskRateTooHigh,
    #[msg("RiskRateTooLow")] 
    RiskRateTooLow,


}