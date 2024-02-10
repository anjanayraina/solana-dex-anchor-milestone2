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

pub fn calculate_premium_price_after(step : &mut SimulateMoveStep , reached : bool , size_usd : u128 ) -> (u128){
    
    if reached {
        return step.to.premium_rate_x96;
    }


    return 100;
}

pub fn update_price_state( parameter: UpdatePriceStateParameter ,  global_position : GlobalLiquidityPosition , mut price_state : PriceState) -> Result<()> {
    if parameter.size_delta == 0 {
        return err!(Errors::InvalidOperation);
    }

    let mut global_position_cache = global_position.clone();
    let mut price_state_cache = PriceStateCache {
        premium_rate_x96: price_state.premium_rate_x96,
        pending_vertex_index: price_state.pending_vertex_index,
        liquidation_vertex_index: parameter.liquidation_vertex_index,
        current_vertex_index: price_state.current_vertex_index,
        basis_index_price_x96: price_state.basis_index_price_x96 as u64,
    };

    let balanced = (global_position_cache.net_size | global_position_cache.liquidation_buffer_net_size) == 0;
    if balanced {
        price_state_cache.basis_index_price_x96 = parameter.index_price_x96;
    }

    let improve_balance = parameter.side == global_position_cache.side && !balanced;
    // Assuming `_update_price_state` is another function you'll implement
    let (trade_price_x96_times_size_total, size_left, total_buffer_used) = self::_update_price_state(
        &mut global_position_cache,
        &mut price_state,
        &mut price_state_cache,
        &parameter,
        improve_balance,
    );

    // Implement the logic to apply changes from price_state_cache back to the actual accounts
    // This might involve directly modifying ctx.accounts.price_state and ctx.accounts.global_position

    Ok(())
}

// Additional helper functions like `_update_price_state` would be defined here


pub fn _update_price_state(
    global_position_cache: &GlobalLiquidityPosition, 
    price_state: &mut PriceState,
    price_state_cache: &mut PriceStateCache, 
    parameter: &UpdatePriceStateParameter, 
    improve_balance: bool,
) -> (i128, u128, u128) {
    // Initialized with a default vertex
    let default_vertex = PriceVertex{
        size: 0,
        premium_rate_x96 : 0
    };

    let mut step = SimulateMoveStep {
        side: parameter.side,
        size_left: parameter.size_delta,
        index_price_x96: parameter.index_price_x96,
        basis_index_price_x96: price_state_cache.basis_index_price_x96,
        improve_balance,
        from: default_vertex.clone(),
        current: PriceVertex {
            size: global_position_cache.net_size,
            premium_rate_x96: price_state_cache.premium_rate_x96,
        },
        to: default_vertex.clone(), // Assuming a default implementation or placeholder
    };

    let mut trade_price_x96_times_size_total: i128 = 0;
    let mut total_buffer_used: u128 = 0;

    // Logic for adjusting the price state
    // This is a simplified placeholder logic. You will replace this with actual logic.
    if !step.improve_balance {
        // Simulate some adjustments based on your logic
        // For demonstration, let's iterate through some vertices as an example
        for i in price_state_cache.current_vertex_index..price_state.price_vertices.len() as u128 {
            let vertex = &price_state.price_vertices[(i-1) as usize];
            step.from = vertex.clone();
            step.to = (&price_state.price_vertices[i as usize]).clone();

            
            // Placeholder for simulate_move call - replace with actual logic
            // let (trade_price_x96, size_used, _, premium_rate_after_x96) = simulate_move(&step);
            let (trade_price_x96, size_used, _, premium_rate_after_x96) = simulate_move(&mut step);
            if size_used < step.size_left && !(parameter.liquidation && price_state_cache.liquidation_vertex_index == i as u8) {
                price_state_cache.current_vertex_index = i+1;
                step.current = step.to.clone();
            } 
            // Placeholder adjustments
            trade_price_x96_times_size_total += trade_price_x96*(size_used as i128 );
            total_buffer_used += size_used;
            
            
            step.size_left = step.size_left.saturating_sub(size_used);
        }

        if(step.size_left > 0){
            if(!parameter.liquidation) {
                return (-1 , 0 ,0); // throw error at this condition 
            }

            // Assuming `priceVertices` is accessible and `liquidationVertexIndex` is within bounds
    let vertex = price_state.price_vertices[price_state_cache.liquidation_vertex_index as usize].clone(); // Clone if necessary

    step.to = vertex.clone();
    step.from = vertex.clone();
    step.current = vertex; 
    let ( trade_price , _ , _ , _) = simulate_move(&mut step);
    total_buffer_used+=step.size_left;
    let liquidation_vertex_index = price_state_cache.liquidation_vertex_index;
    let liquidation_buffer_net_size_after = price_state.liquidation_buffer_net_sizes.get(liquidation_vertex_index as usize).unwrap() + step.size_left;
    price_state.liquidation_buffer_net_sizes[liquidation_vertex_index as usize] = liquidation_buffer_net_size_after;

        }
    }
// Else part for when the balance rate got better
else {
    // Logic when balance rate got better
    for (index, vertex) in price_state.price_vertices.iter().enumerate().rev() {
        let i = index ; // assuming the use of u8 for indexing aligns with your data sizes
        let mut buffer_size_after = *price_state.liquidation_buffer_net_sizes.get(i).unwrap();
        let mut size_used = 0;
        if(buffer_size_after > 0){
            let vertex = price_state.price_vertices[index as usize].clone();
            step.from = vertex.clone();
            step.to = vertex.clone();
            size_used = std::cmp::min(buffer_size_after, step.size_left);
            buffer_size_after-=size_used;
            price_state.liquidation_buffer_net_sizes[i] = buffer_size_after;
            total_buffer_used+=size_used;
            step.size_left-=size_used;
            let (trade_price_x96, size_used, _, premium_rate_after_x96) = simulate_move(&mut step);
            trade_price_x96_times_size_total+=trade_price_x96*(size_used as i128);

        }
        step.from = vertex.clone();
        step.to = if i > 0 {
            price_state.price_vertices[i as usize - 1].clone()
        } else {
            vertex.clone() // handle the edge case for i == 0
        };

        

        if size_used < step.size_left {
            step.from = price_state.price_vertices.get(i as usize).unwrap().clone();
            step.to = price_state.price_vertices.get(i-1 as usize).unwrap().clone();
            let (trade_price_x96, size_used, reached , premium_rate_after_x96) = simulate_move(&mut step);
            if reached {
                price_state_cache.current_vertex_index = (i-1) as u128;
                step.current = step.to;
            }
            step.size_left-=size_used;
            trade_price_x96_times_size_total+=trade_price_x96*(size_used as i128 );
            price_state_cache.premium_rate_x96 = premium_rate_after_x96;

        }

        // Update calculations
        // trade_price_x96_times_size_total += trade_price_x96 * size_used as i128;
        // total_buffer_used += size_used;
        // step.size_left = step.size_left.saturating_sub(size_used);
    }


}
// Assuming the rest of your function logic follows here


    // Return calculated values
    // These are placeholders; replace them with actual calculated values
    (trade_price_x96_times_size_total, step.size_left, total_buffer_used)
}

fn calculate_reached_and_size_used(step: &SimulateMoveStep) -> (bool, u128) {
    let size_cost = if step.improve_balance {
        step.current.size.saturating_sub(step.to.size)
    } else {
        step.to.size.saturating_sub(step.current.size)
    };
    let reached = step.size_left >= size_cost;
    let size_used = if reached { size_cost } else { step.size_left };

    (reached, size_used)
}


pub fn simulate_move(step: &mut SimulateMoveStep) -> (i128, u128, bool, u128) {
    let (reached, size_used) = calculate_reached_and_size_used(step);
    let premium_rate_after_x96 = calculate_premium_price_after(step, reached, size_used);
    let premium_rate_before_x96 = step.current.premium_rate_x96;

    let (price_delta_x96_down, price_delta_x96_up) = (100, 100 );   //add the multdiv library    mul_div_2(step.basis_index_price_x96, premium_rate_before_x96 + premium_rate_after_x96, Q96 << 1);

    let trade_price_x96 = if step.side { 
        if step.improve_balance {
            (step.index_price_x96 as i128 - price_delta_x96_down as i128).max(0)
        } else {
            (step.index_price_x96 + price_delta_x96_up) as i128
        }
    } else { 
        if step.improve_balance {
            (step.index_price_x96 + price_delta_x96_down) as i128
        } else {
            (step.index_price_x96 as i128 - price_delta_x96_up as i128).max(0)
        }
    };

    (trade_price_x96, size_used, reached, premium_rate_after_x96)
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

pub fn calculate_market_price_x96(global_side : bool , side : bool , index_price : u128 , basis_index_price : u128 ,premium_rate : u128) -> (u128 ) {
    let price_delta_down = 100 ; // add multdiv here after library creation 
    let  price_delta_up = 100;
    let mut market_price =10;
    if global_side {
        market_price = if side {
            if index_price > price_delta_down { index_price - price_delta_down } else { 0 }
        } else {
            if index_price > price_delta_up { index_price - price_delta_up } else { 0 }
        };
    } else {
        market_price = if side {
            index_price + price_delta_up
        } else {
            index_price + price_delta_down
        };
    }

    market_price
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
    pub from: PriceVertex,
    pub current: PriceVertex,
    pub to: PriceVertex,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct PriceStateCache {
    pub premium_rate_x96: u128,
    pub pending_vertex_index: u8,
    pub liquidation_vertex_index: u8,
    pub current_vertex_index: u128,
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
    pub current_vertex_index: u128,
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


