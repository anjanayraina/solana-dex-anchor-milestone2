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
    pub max_size : u128,
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
    pub global_position : GlobalPosition,
    pub positions : Vec<Position> 
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

pub fn increase_position(
    state: &mut State, 
    market_config: &MarketConfig, 
    parameter: &IncreasePositionParameter,
    position_cache: &mut Position, 
    index : usize , 

) -> Result<u128> { 

    if position_cache.size == 0 {
        if parameter.size_delta == 0 {
            return err!(ErrorCode::PositionNotFound);
        }

        // market util call
    }
    let base_cfg = &market_config.base_config;



    validate_global_liquidity(state.global_liqudity_position.liquidity)?;


    let trading_fee_state = build_trading_fee_state(&market_config.fee_rate_config, parameter.account, 0, 0); // Placeholder for referral tokens

    let mut size_after = position_cache.size;
    let mut trade_price_x96 = 0u128; // Placeholder for actual trade price calculation
    let mut trading_fee :u128= 0;
    if parameter.size_delta > 0 {
        size_after = validate_increase_size(
            state.global_liqudity_position.net_size, 
            state.global_liqudity_position.max_size, 
            position_cache.size, 
            parameter.size_delta,
        )?;

        // Placeholder for index price calculation
        let index_price_x96 = 0u128; // Market Util call 

        // Placeholder for trade price calculation and updating price state
        trade_price_x96 = index_price_x96; 

        // Assuming a function to distribute fees
        let fee_param = &DistributeFeeParameter {
            market: parameter.market,
            account: parameter.account,
            size_delta: parameter.size_delta,
            trade_price_x96: trade_price_x96,
            trading_fee_state: trading_fee_state.clone(),
            liquidation_fee: 0,
        };
        let trading_fee = distribute_fee(
            &mut state.global_liqudity_position,
            &market_config.fee_rate_config , 
            fee_param
        );
    }

    let global_funding_growth = choose_previous_global_funding_rate_growth_x96(&state.global_position, parameter.side);
    
    let funding_fee = calculate_funding_fee(
        global_funding_growth, 
        position_cache.entry_funding_rate_growth_x96, 
        position_cache.size,
    );

    let margin_after = position_cache.margin as i128 + parameter.margin_delta as i128 + funding_fee - trading_fee as i128;

    // Calculate the new entry price
    let entry_price_after_x96 = calculate_next_entry_price_x96(
        parameter.side,
        position_cache.size,
        position_cache.entry_price_x96,
        parameter.size_delta,
        trade_price_x96,
    );

    let maintain_parameter: MaintainMarginRateParameter = MaintainMarginRateParameter{
        margin : margin_after , 
        side : parameter.side ,
        size : size_after , 
        entry_price_x96 : entry_price_after_x96 , 
        decrease_price_x96 : 0, // place holder to add market util 
        trading_fee_rate : trading_fee_state.trading_fee_rate , 
        liquidatable_position : false , 

    };
    validate_position_liquidate_maintain_margin_rate(base_cfg, &maintain_parameter );

    if parameter.size_delta > 0 {
        // market util call 
        increase_global_position(&mut state.global_position, parameter.side , parameter.size_delta);
    }

    let position: &mut Position = state.positions.get_mut(index).unwrap();
    position.margin = margin_after as u128;
    position.size = size_after;
    position_cache.entry_price_x96 = entry_price_after_x96;
    position_cache.entry_funding_rate_growth_x96 = 0; // Placeholder for actual update


    Ok(trade_price_x96) 
}
pub fn decrease_position(
    state: &mut State,
    market_config: &MarketConfig,
    parameter: &DecreasePositionParameter,
    position: &mut Position,
) -> Result<(u128, u128)> {
    // Validate position size before proceeding

    if  position.size == 0 {
        return Err(
        error!(ErrorCode::PositionNotFound)
        );
    }
    if position.size < parameter.size_delta {
        return Err(error!(ErrorCode::InsufficientSizeToDecrease));
    }

    // Placeholder for settle liquidity unrealized PnL and decrease index price calculation

    let mut trading_fee_state = build_trading_fee_state(&market_config.fee_rate_config, parameter.account, 0, 0); // Placeholder for referral tokens
    let mut trade_price_x96 = 100;
    let global_funding_rate_growth_x96 = choose_previous_global_funding_rate_growth_x96(&state.global_position, parameter.side);
    let mut trading_fee = 100;
    let mut funding_fee  = 100;
    let mut realized_pnl_delta = 100;
    if parameter.size_delta > 0 {
    trade_price_x96 = 100; // place holder for the PriceUtil call 
    let fee_param = &DistributeFeeParameter {
        market: parameter.market,
        account: parameter.account,
        size_delta: position.size,
        trade_price_x96: trade_price_x96,
        trading_fee_state: trading_fee_state.clone(),
        liquidation_fee: 0,
    };
    let trading_fee = distribute_fee(
        &mut state.global_liqudity_position,
        &market_config.fee_rate_config , 
        fee_param
    );
     funding_fee = calculate_funding_fee(
        global_funding_rate_growth_x96,
        position.entry_funding_rate_growth_x96,
        position.size,
    );
    // Calculate unrealized PnL delta based on the size decrease
    realized_pnl_delta = calculate_unrealized_pnl(
        parameter.side,
        parameter.size_delta,
        position.entry_price_x96,
        0, // Placeholder for current price, assuming this will be calculated or provided
    );
    }
    let margin_after = (position.margin as i128) + realized_pnl_delta + funding_fee - (trading_fee as i128) - (parameter.margin_delta as i128);
    if margin_after < 0 {
        return Err(error!(ErrorCode::InsufficientMargin));
    }

    let size_after = position.size - parameter.size_delta;

    // Update position's size and margin
 

    if size_after > 0 {
        // calls to marketutil and other libraries 
    }

    else {
        // Logic to delete the position or mark it as closed
    }

    // Adjust global position
    if parameter.size_delta > 0 {
    decrease_global_position(&mut state.global_position, parameter.side, parameter.size_delta);
    }
    position.size = size_after;
    position.margin = margin_after as u128;
    position.entry_funding_rate_growth_x96 = global_funding_rate_growth_x96;
    
    Ok((0, margin_after as u128)) // placeholder values 
}

pub fn liquidate_position(
    state: &mut State,
    market_config: &MarketConfig,
    position: &mut Position, 
    trading_fee_state: &TradingFeeState,
    parameter: &LiquidateParameter,
) -> Result<()> {
    if position.size == 0 {
        return err!(ErrorCode::PositionNotFound);
    }
    let base_cfg = &market_config.base_config;
    let liquidation_execution_fee = base_cfg.liquidation_execution_fee;
    let liquidation_fee_rate = base_cfg.liquidation_fee_rate_per_position;

    let (liquidation_price_x96, adjusted_funding_fee) = calculate_liquidation_price_x96(
        position,
        &state.global_position,
        parameter.side,
        parameter.required_funding_fee,
        liquidation_fee_rate,
        trading_fee_state.trading_fee_rate,
        liquidation_execution_fee,
    );

    let liquidation_fee = calculate_liquidation_fee(
        position.size,
        position.entry_price_x96,
        liquidation_fee_rate,
    );
    let mut liquidation_fund_delta = liquidation_fee as i128;

    // Adjust the funding rate by liquidation if needed

    if parameter.required_funding_fee != adjusted_funding_fee {
        liquidation_fund_delta += adjust_funding_rate_by_liquidation(
            &mut state.global_position,
            parameter.side,
            parameter.required_funding_fee,
            adjusted_funding_fee,
        );
    }

    // Calculate the difference if the liquidation price differs from the trade price
    liquidation_fund_delta += calculate_unrealized_pnl(
        parameter.side,
        position.size,
        liquidation_price_x96,
        parameter.trade_price_x96,
    );

    // Distribute the fee
    // global_liquidity_position: &mut GlobalLiquidityPosition,
    // fee_rate_cfg: &MarketFeeRateConfig,
    // parameter: &DistributeFeeParameter,
    let fee_param = &DistributeFeeParameter {
        market: parameter.market,
        account: parameter.account,
        size_delta: position.size,
        trade_price_x96: liquidation_price_x96,
        trading_fee_state: trading_fee_state.clone(),
        liquidation_fee: liquidation_fund_delta,
    };
    let trading_fee = distribute_fee(
        &mut state.global_liqudity_position,
        &market_config.fee_rate_config , 
        fee_param
    );

    // Decrease the global position
    decrease_global_position(&mut state.global_position, parameter.side, position.size);


    // add delete the position 
    Ok(())
}


pub fn calculate_liquidation_price_x96(
    position: &Position,
    global_funding_rate: &GlobalPosition,
    is_long: bool,
    funding_fee: i128,
    liquidation_fee_rate: u32,
    trading_fee_rate: u32,
    liquidation_execution_fee: u64,
) -> (u128, i128) {
    // Assuming margin is stored as u128 in Position
    let margin_int256 = position.margin as i128 + funding_fee;

    // Adjusted funding fee, initialized to the input funding fee
    let mut adjusted_funding_fee = funding_fee;

    // Placeholder for liquidation price calculation
    let mut liquidation_price_x96: u128 = 0;

    if margin_int256 > 0 {
        // Placeholder logic for calculating the liquidation price
        // This should be replaced with the actual logic based on your application's needs
        liquidation_price_x96 = _calculate_liquidation_price_x96(
            position.margin,
            position.size,
            position.entry_price_x96,
            is_long,
            funding_fee,
            liquidation_fee_rate,
            trading_fee_rate,
            liquidation_execution_fee,
        );

        // Assuming a function to check if the liquidation price is acceptable
        if is_acceptable_liquidation_price_x96(
            is_long,
            liquidation_price_x96,
            position.entry_price_x96,
        ) {
            return (liquidation_price_x96, funding_fee);
        }
    }

    // Placeholder logic to adjust the funding fee based on previous global funding rate
    // This should involve calculating a new funding fee based on the difference in funding rates
    // and the position size, similar to the Solidity logic

    adjusted_funding_fee = calculate_funding_fee(
        choose_previous_global_funding_rate_growth_x96(global_funding_rate, is_long),
        position.entry_funding_rate_growth_x96,
        position.size,
    );

    // Recalculate liquidation price with adjusted funding fee
    liquidation_price_x96 = _calculate_liquidation_price_x96(
        position.margin,
        position.size,
        position.entry_price_x96,
        is_long,
        adjusted_funding_fee,
        liquidation_fee_rate,
        trading_fee_rate,
        liquidation_execution_fee,
    );

    // Return the calculated liquidation price and adjusted funding fee
    (liquidation_price_x96, adjusted_funding_fee)
}



pub fn distribute_fee(
    global_liquidity_position: &mut GlobalLiquidityPosition,
    fee_rate_cfg: &MarketFeeRateConfig,
    parameter: &DistributeFeeParameter,
) -> u128 {
    let trading_fee = calculate_trading_fee(parameter.size_delta, parameter.trade_price_x96, fee_rate_cfg.trading_fee_rate);

    let liquidity_fee = 0; // Placeholder for liquidity fee calculation logic

    if trading_fee == 0 && parameter.liquidation_fee == 0 {
        return 0;
    }

    // Example logic for updating global liquidity position based on liquidation fee
    // This is a simplified example; actual logic will depend on your application's requirements
    if parameter.liquidation_fee != 0 {
        let liquidation_fund_after = parameter.liquidation_fee; 
    }

    // Update global liquidity position's unrealized PnL growth
    let unrealized_pnl_growth_after_x64 = global_liquidity_position.unrealized_pnl_growth_x64 +
       ( ((liquidity_fee as u128) << 64) as i128) / ( global_liquidity_position.liquidity as i128) ;
    global_liquidity_position.unrealized_pnl_growth_x64 = unrealized_pnl_growth_after_x64;

    trading_fee
}
pub fn calculate_next_entry_price_x96(
    is_long: bool,
    size_before: u128,
    entry_price_before_x96: u128,
    size_delta: u128,
    trade_price_x96: u128,
) -> u128 {
    if size_before == 0 && size_delta == 0 {
        return 0
    } else if size_before == 0 {
        return trade_price_x96
    } else if size_delta == 0 {
        return entry_price_before_x96
    } else {
        let liquidity_after_x96 = size_before
            .checked_mul(entry_price_before_x96)
            .unwrap()
            .checked_add(size_delta.checked_mul(trade_price_x96).unwrap())
            .unwrap();
        let size_after = size_before.checked_add(size_delta).unwrap();

        if is_long {
            return ceil_div(liquidity_after_x96, size_after)
        } else {
            return liquidity_after_x96 / size_after
        }
    }
}

/// Calculate the liquidity (value) of a position.
pub fn calculate_liquidity(size: u128, price_x96: u128) -> u128 {
    let q96   = 100 ; // change this too 

    size.checked_mul(price_x96).unwrap().checked_div(q96).unwrap()
}

/// Calculate the unrealized PnL of a position based on entry price.
pub fn calculate_unrealized_pnl(
    is_long: bool,
    size: u128,
    entry_price_x96: u128,
    price_x96: u128,
) -> i128 {
    let bp :u128= 100; // add when adding constanst 
    let q96   = 100 ; // change this too 
    if is_long {
        if entry_price_x96 > price_x96 {
            -(size.checked_mul(entry_price_x96 - price_x96).unwrap().checked_div(q96).unwrap() as i128)
        } else {
            size.checked_mul(price_x96 - entry_price_x96).unwrap().checked_div(q96).unwrap() as i128
        }
    } else {
        if entry_price_x96 < price_x96 {
            -(size.checked_mul(price_x96 - entry_price_x96).unwrap().checked_div(q96).unwrap() as i128)
        } else {
            size.checked_mul(entry_price_x96 - price_x96).unwrap().checked_div(q96).unwrap() as i128
        }
    }
}

/// Calculate the liquidation fee.
pub fn calculate_liquidation_fee(size: u128, entry_price_x96: u128, liquidation_fee_rate: u32) -> u128 {
    let bp :u128= 100; // add when adding constanst 
    let q96   = 100 ; // change this too 
    let numerator = size.checked_mul(liquidation_fee_rate as u128).unwrap();
    let denominator = bp.checked_mul(q96).unwrap();
    numerator.checked_mul(entry_price_x96).unwrap().checked_div(denominator).unwrap()
}

/// Calculate the funding fee of a position.
pub fn calculate_funding_fee(global_funding_rate_growth_x96: i128, position_funding_rate_growth_x96: i128, position_size: u128) -> i128 {
    let delta_x96 = global_funding_rate_growth_x96 - position_funding_rate_growth_x96;
    let bp = 100; // add when adding constanst 
    let q96   = 100 ; // change this too 
    if delta_x96 >= 0 {
        (u128::try_from(delta_x96).unwrap().checked_mul(position_size).unwrap().checked_div(q96).unwrap() as i128)
    } else {
        -(u128::try_from(-delta_x96).unwrap().checked_mul(position_size).unwrap().checked_div(q96).unwrap() as i128)
    }
}

/// Calculate the maintenance margin.
pub fn calculate_maintenance_margin(size: u128, entry_price_x96: u128, index_price_x96: u128, liquidation_fee_rate: u32, trading_fee_rate: u32, liquidation_execution_fee: u64) -> u128 {
    let fee_part = entry_price_x96.checked_mul(liquidation_fee_rate as u128).unwrap()
        .checked_add(index_price_x96.checked_mul(trading_fee_rate as u128).unwrap()).unwrap();
    let bp = 100; // add when adding constanst 
    let q96   = 100 ; // change this too 
    let scaled_fee = fee_part.checked_div(bp).unwrap();
    let margin_without_fee = size.checked_mul(scaled_fee).unwrap().checked_div(q96).unwrap();
    margin_without_fee.checked_add(liquidation_execution_fee as u128).unwrap()
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
    previous_global_funding_rate: &GlobalPosition,
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


pub fn _calculate_liquidation_price_x96(
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
    #[msg("InsufficientSizeToDecrease")]
    InsufficientSizeToDecrease , 
    #[msg("InsufficientMargin")]
    InsufficientMargin,
    #[msg("Invalid Position")]
    PositionNotFound ,



}