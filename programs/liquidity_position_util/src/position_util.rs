use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;


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


#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct TradingFeeState {
    pub trading_fee_rate: u32,
    pub referral_return_fee_rate: u32,
    pub referral_parent_return_fee_rate: u32,
    pub referral_token: u128,
    pub referral_parent_token: u128,
}


#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct IncreasePositionParameter {
    pub market: Pubkey, 
    pub account: Pubkey,
    pub side: bool,
    pub margin_delta: u128,
    pub size_delta: u128,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct DecreasePositionParameter {
    pub market: Pubkey,
    pub account: Pubkey,
    pub side: bool,
    pub margin_delta: u128,
    pub size_delta: u128,
    pub receiver: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct LiquidatePositionParameter {
    pub market: Pubkey,
    pub account: Pubkey,
    pub side: bool,
    pub fee_receiver: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct MaintainMarginRateParameter {
    pub margin: i128, // Adjusted to i128
    pub side: bool,
    pub size: u128,
    pub entry_price_x96: u128,
    pub decrease_price_x96: u128,
    pub trading_fee_rate: u32,
    pub liquidatable_position: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct LiquidateParameter {
    pub market: Pubkey,
    pub account: Pubkey,
    pub side: bool,
    pub trade_price_x96: u128,
    pub decrease_index_price_x96: u128,
    pub required_funding_fee: i128, // Adjusted to i128
    pub fee_receiver: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct DistributeFeeParameter {
    pub market: Pubkey,
    pub account: Pubkey,
    pub size_delta: u128,
    pub trade_price_x96: u128,
    pub trading_fee_state: TradingFeeState,
    pub liquidation_fee: i128, // Adjusted to i128
}

pub struct MarketConfig {
    pub base_config: MarketBaseConfig,
    pub fee_rate_config: MarketFeeRateConfig,
    pub price_config: MarketPriceConfig,
}

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

pub struct MarketFeeRateConfig {
    pub trading_fee_rate: u32,
    pub protocol_fee_rate: u32,
    pub referral_return_fee_rate: u32,
    pub referral_parent_return_fee_rate: u32,
    pub referral_discount_rate: u32,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct VertexConfig {
    pub balance_rate: u32,
    pub premium_rate: u32,
}

pub struct MarketPriceConfig {
    pub max_price_impact_liquidity: u128,
    pub liquidation_vertex_index: u8,
    pub vertices: [VertexConfig; 10],
}

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
    pub entry_unrealized_pnl_growth_x64: i128,
}



pub struct GlobalPosition {
    pub long_size: u128,
    pub short_size: u128,
    pub max_size: u128,
    pub max_size_per_position: u128,
    pub long_funding_rate_growth_x96: i128, 
    pub short_funding_rate_growth_x96: i128, 
}


pub struct PreviousGlobalFundingRate {
    pub long_funding_rate_growth_x96: i128, 
    pub short_funding_rate_growth_x96: i128, 
}


pub struct GlobalFundingRateSample {
    pub last_adjust_funding_rate_time: u64,
    pub sample_count: u16,
    pub cumulative_premium_rate_x96: i128, 
}


pub struct Position {
    pub margin: u128,
    pub size: u128,
    pub entry_price_x96: u128, 
    pub entry_funding_rate_growth_x96: i128, 
}
fn calculate_trading_fee(size_delta: u128, trade_price_x96: u128, trading_fee_rate: u32) -> u128 {
    // Placeholder for trading fee calculation
    0
}
pub struct State {
   
    pub price_state: PriceState,
    pub usd_balance: u128,
    pub protocol_fee: u128,
    pub liquidity_positions : Vec<AccountToLiquidity> , 
    pub global_liqudity_position : GlobalLiquidityPosition,

}
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]

pub struct AccountToLiquidity {
    account : Pubkey , 
    liquidity : u128, 
    margin : u128 , 
    entry_unreaized_pnl_growth : u128,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct PriceState {
    premium_rate_x96: u128,
    pending_vertex_index: u8,
    current_vertex_index: u8,
    basis_index_price_x96: u128,
    // Additional fields...
}


/// Simplified version of _calculateFee
pub fn calculate_fee(
    state: &mut State,
    fee_rate_cfg: &MarketFeeRateConfig,
    parameter: &DistributeFeeParameter,
) -> (u128, u128) { // Returns tradingFee and liquidityFee
    let trading_fee = calculate_trading_fee(
        parameter.size_delta,
        parameter.trade_price_x96,
        parameter.trading_fee_state.trading_fee_rate,
    );

    if trading_fee == 0 {
        return (0, 0);
    }

    let protocol_fee = split_fee(trading_fee, fee_rate_cfg.protocol_fee_rate);
    state.protocol_fee += protocol_fee;

    let liquidity_fee = trading_fee - protocol_fee;

    // Assuming implementation for referral fee logic here
    let referral_fee = 0u128; // Placeholder for actual referral fee calculation
    let referral_parent_fee = 0u128; // Placeholder for actual referral parent fee calculation

    // Adjust liquidity fee based on referral fees
    let adjusted_liquidity_fee = liquidity_fee - (referral_fee + referral_parent_fee);

    (trading_fee, adjusted_liquidity_fee)
}
pub fn split_fee(trading_fee: u128, fee_rate: u32) -> u128 {
    let basis_points_divisor = 10000u128; // Assuming a basis point divisor of 10,000 for percentage calculations
    (trading_fee * fee_rate as u128) / basis_points_divisor
}

/// Chooses the previous global funding rate growth based on the position side.
pub fn choose_previous_global_funding_rate_growth_x96(
    previous_global_funding_rate: &PreviousGlobalFundingRate,
    is_long: bool,
) -> i128 { // Assuming simplified usage of i128 to represent fixed-point numbers
    if is_long {
        previous_global_funding_rate.long_funding_rate_growth_x96
    } else {
        previous_global_funding_rate.short_funding_rate_growth_x96
    }
}

/// Checks if the calculated liquidation price is acceptable based on the position side and entry price.
pub fn is_acceptable_liquidation_price_x96(
    is_long: bool,
    liquidation_price_x96: u128, // Simplified to u128 for consistency
    entry_price_x96: u128, // Simplified to u128 for consistency
) -> bool {
    (is_long && liquidation_price_x96 < entry_price_x96) || (!is_long && liquidation_price_x96 > entry_price_x96)
}
pub fn change_max_size(
    global_liquidity_position: &GlobalLiquidityPosition,
    base_cfg: &MarketBaseConfig,
    global_position: &mut GlobalPosition,
    index_price_x96: u128,
) {
    let min_liquidity = min(global_liquidity_position.liquidity, base_cfg.max_position_liquidity);
    let numerator = min_liquidity.checked_mul(base_cfg.max_position_value_rate as u128).unwrap();
    let bp :u128= 100;
    let denominator = bp.checked_mul(index_price_x96).unwrap();
    let max_size_after = numerator.checked_div(denominator).unwrap();
    let max_size_per_position_after = max_size_after.checked_mul(base_cfg.max_size_rate_per_position as u128 ).unwrap().checked_div(bp).unwrap(); // replace with the constants when done 

    global_position.max_size = max_size_after;
    global_position.max_size_per_position = max_size_per_position_after;
}

pub fn calculate_liquidation_price_x96(
    position_margin: u128, // Assuming margin is directly passed instead of the entire position for simplicity
    position_size: u128, // Directly pass size
    entry_price_x96: u128, // Directly pass entry price
    is_long: bool, // Simplify Side to a bool
    funding_fee: i128,
    liquidation_fee_rate: u32,
    trading_fee_rate: u32,
    liquidation_execution_fee: u64,
) -> u128 { // Return type simplified to u128
    let margin_after = if funding_fee >= 0 {
        position_margin.wrapping_add(funding_fee as u128)
    } else {
        position_margin.wrapping_sub((-funding_fee) as u128)
    };

    let (numerator_x96, denominator) = if is_long {
        ((10000 + liquidation_fee_rate) as u128, (10000 - trading_fee_rate) as u128)
    } else {
        ((10000 - liquidation_fee_rate) as u128, (10000 + trading_fee_rate) as u128)
    };

    let numerator_part2_x96 = if margin_after >= liquidation_execution_fee as u128 {
        margin_after - liquidation_execution_fee as u128
    } else {
        liquidation_execution_fee as u128 - margin_after
    };

    let total_numerator_x96 = numerator_x96 * entry_price_x96 * position_size + numerator_part2_x96 * 10000 * (1 << 96);
    let total_denominator = denominator * position_size;

    if is_long {
        total_numerator_x96 / total_denominator
    } else {
        (total_numerator_x96 + total_denominator - 1) / total_denominator // emulate ceilDiv
    }
}

/// Validates if increasing the position size would exceed the market's size limits.
pub fn validate_increase_size(
    max_size_per_position: u128, // Assume these are parameters directly passed
    max_size: u128,
    size_before: u128,
    size_delta: u128,
) -> Result<u128> { // Using Result to handle errors
    let size_after = size_before.checked_add(size_delta).ok_or_else(|| ErrorCode::Overflow)?;

    if size_after > max_size_per_position {
        return Err(error!(ErrorCode::SizeExceedsMaxSizePerPosition));
    }

    let total_size_after = size_after; // Assuming total size is calculated elsewhere or passed as a parameter

    if total_size_after > max_size {
        return Err(error!(ErrorCode::SizeExceedsMaxSize));
    }

    Ok(size_after)
}



fn build_trading_fee_state(fee_rate_cfg: &MarketFeeRateConfig, account: Pubkey , referral_token : u128 , referral_parent_token : u128) -> TradingFeeState {
    
   
    let trading_fee_rate = if referral_token == 0 {
        fee_rate_cfg.trading_fee_rate
    } else {
        let bp = 100; // change after you add the constants 
        let discounted_trading_fee_rate = ((fee_rate_cfg.trading_fee_rate as u128 * fee_rate_cfg.referral_discount_rate as u128) / bp as u128) as u32;
        discounted_trading_fee_rate
    };

    TradingFeeState {
        referral_token,
        referral_parent_token,
        trading_fee_rate,
        referral_return_fee_rate: fee_rate_cfg.referral_return_fee_rate,
        referral_parent_return_fee_rate: fee_rate_cfg.referral_parent_return_fee_rate,
    }
}

fn calculate_unrealized_pnl(side: bool, size: u128, entry_price_x96: u128, decrease_price_x96: u128) -> i128 {
    // Placeholder implementation
    0
}

fn calculate_maintenance_margin(size: u128, entry_price_x96: u128, decrease_price_x96: u128, liquidation_fee_rate_per_position: u32, trading_fee_rate: u32, liquidation_execution_fee: u64) -> u128 {
    // Placeholder implementation
    0
}

fn adjust_global_funding_rate(global_position: &mut GlobalPosition, long_rate_adjustment: i128, short_rate_adjustment: i128) {
    // Adjust the global funding rates based on the provided adjustments
    global_position.long_funding_rate_growth_x96 = global_position.long_funding_rate_growth_x96.checked_add(long_rate_adjustment).expect("Overflow in funding rate adjustment");
    global_position.short_funding_rate_growth_x96 = global_position.short_funding_rate_growth_x96.checked_add(short_rate_adjustment).expect("Overflow in funding rate adjustment");
    // Additional logic as needed...
}

pub fn validate_global_liquidity(global_liquidity: u128) -> Result<()> {
    if global_liquidity == 0 {
        // Replace `InsufficientGlobalLiquidity` with the actual error handling approach you prefer.
        return Err(ErrorCode::InsufficientGlobalLiquidity.into());
    }
    Ok(())
}

// Increases the size of the global position based on the side.
// `is_long` is `true` for long positions, and `false` for short positions.
pub fn increase_global_position(global_position: &mut GlobalPosition, is_long: bool, size: u128) {
    if is_long {
        global_position.long_size = global_position.long_size.checked_add(size).expect("Overflow in long size");
    } else {
        global_position.short_size = global_position.short_size.checked_add(size).expect("Overflow in short size");
    }
}


pub fn decrease_global_position(global_position: &mut GlobalPosition, is_long: bool, size: u128) {
    if is_long {
        global_position.long_size = global_position.long_size.checked_sub(size).expect("Underflow in long size");
    } else {
        global_position.short_size = global_position.short_size.checked_sub(size).expect("Underflow in short size");
    }
}

// Define the error code for insufficient global liquidity

pub fn adjust_funding_rate_by_liquidation(
    global_position: &mut GlobalPosition,
    side: bool, // true for Long, false for Short
    required_funding_fee: i128,
    adjusted_funding_fee: i128,
) -> i128 {
    let insufficient_funding_fee = adjusted_funding_fee - required_funding_fee;
    let opposite_size = if side { global_position.short_size } else { global_position.long_size };

    let liquidation_fund_loss = if opposite_size > 0 {
        let q96 = 1u128 << 96;
        let insufficient_funding_rate_growth_delta_x96 = mul_div(insufficient_funding_fee as u128, q96.try_into().unwrap(), opposite_size.try_into().unwrap());

        if side {
            // Adjust short funding rate for a long position liquidation
            adjust_global_funding_rate(global_position, 0, -( insufficient_funding_rate_growth_delta_x96 as i128)); // add it afyer creating funding rate util
        } else {
            // Adjust long funding rate for a short position liquidation
            adjust_global_funding_rate(global_position, - ( insufficient_funding_rate_growth_delta_x96 as i128), 0);
        }
        0
    } else {
        -insufficient_funding_fee
    };

    liquidation_fund_loss
}

pub fn validate_position_liquidate_maintain_margin_rate(base_cfg: &MarketBaseConfig, parameter: &MaintainMarginRateParameter) -> Result<()> {
    let unrealized_pnl = calculate_unrealized_pnl(parameter.side, parameter.size, parameter.entry_price_x96, parameter.decrease_price_x96);
    let maintenance_margin = calculate_maintenance_margin(parameter.size, parameter.entry_price_x96, parameter.decrease_price_x96, base_cfg.liquidation_fee_rate_per_position, parameter.trading_fee_rate, base_cfg.liquidation_execution_fee);
    let margin_after = parameter.margin.checked_add(unrealized_pnl).ok_or_else(|| ErrorCode::Overflow)?;

    if !parameter.liquidatable_position {
        if parameter.margin <= 0 || margin_after <= 0 || maintenance_margin >= margin_after as u128 {
            return Err(ErrorCode::MarginRateTooHigh.into());
        }
    } else {
        if parameter.margin > 0 && margin_after > 0 && maintenance_margin < margin_after as u128 {
            return Err(ErrorCode::MarginRateTooLow.into());
        }
    }

    Ok(())
}

#[error_code]
pub enum ErrorCode {
    #[msg("The margin rate is too high.")]
    MarginRateTooHigh,
    #[msg("The margin rate is too low.")]
    MarginRateTooLow,
    #[msg("Overflow occurred.")]
    Overflow,
    #[msg("Insufficient global liquidity.")]
    InsufficientGlobalLiquidity,
    #[msg("Size Excedded")] 
    SizeExceedsMaxSize , 
    #[msg("Size Excedded per position ")] 
    SizeExceedsMaxSizePerPosition , 

}