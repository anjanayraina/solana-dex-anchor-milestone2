use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnT");
const GOVERNOR_PUBKEY: Pubkey = Pubkey::new_from_array([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32]);

#[program]
mod position_router {
    use super::*;

    // Constructor equivalent in Anchor
    pub fn initialize(ctx: Context<Initialize>, min_execution_fee: u128 , usd : Pubkey , router : Pubkey , min_block_delayer_executor : u128 , min_time_delay : u128 , max_time_delay : u128 , execution_gas_limit : u128 ) -> Result<()> {
        require!(ctx.accounts.user.key() == GOVERNOR_PUBKEY, Errors::CallerUnauthorized);     
        let state =&mut ctx.accounts.state;
        require!(!state.initilized , Errors::AlreadyInitlized );       
        state.min_execution_fee = min_execution_fee;
        state.usd = usd;
        state.router = router;
        state.min_block_delayer_executor = min_block_delayer_executor;
        state.min_time_delay = min_time_delay;
        state.max_time_delay = max_time_delay;
        state.execution_gas_limit = execution_gas_limit;
        state.initilized = true;
        
        Ok(())
    }

    pub fn add_executor(ctx: Context<UpdateExecutor>, new_executor: Pubkey) -> Result<()> {
        // Ensure the caller is the governor
        require!(ctx.accounts.user.key() == GOVERNOR_PUBKEY, Errors::CallerUnauthorized);

        // Add new executor to the list
        let governance_state = &mut ctx.accounts.state;
        governance_state.executors.push(new_executor);
        Ok(())
    }

        // Function to update executor
    pub fn remove_executor(ctx: Context<UpdateExecutor>, executor: Pubkey) -> Result<()> {
            require!(ctx.accounts.user.key() == GOVERNOR_PUBKEY, Errors::CallerUnauthorized);
            let address_list = &mut ctx.accounts.state.executors;
            address_list.retain(|&x| x != executor);
            // Logic to update executor
            Ok(())
        }

    // Function to update delay values
    pub fn update_delay_values(ctx: Context<UpdateDelayValues> , min_block_delayer_executor : u128 , min_time_delay :u128, max_time_delay : u128 ) -> Result<()> {
        require!(ctx.accounts.user.key() == GOVERNOR_PUBKEY, Errors::CallerUnauthorized);
        let state =&mut ctx.accounts.state;
        state.min_block_delayer_executor = min_block_delayer_executor;
        state.min_time_delay = min_time_delay;
        state.max_time_delay = max_time_delay;
        Ok(())
    }

    // Function to update minimum execution fee
    pub fn update_min_execution_fee(ctx: Context<UpdateMinExecutionFee>, new_fee: u128) -> Result<()> {
        // Logic to update minimum execution fee
        require!(ctx.accounts.user.key() == GOVERNOR_PUBKEY, Errors::CallerUnauthorized);
        let state =&mut ctx.accounts.state;
        state.min_execution_fee = new_fee;
        emit!(MinExecutionFeeUpdated{
            min_execution_fee : new_fee ,
        });
        Ok(())
    }

    // Function to update execution gas limit
    pub fn update_execution_gas_limit(ctx: Context<UpdateExecutionGasLimit>, new_gas_limt : u128 ) -> Result<()> {
                require!(ctx.accounts.user.key() == GOVERNOR_PUBKEY, Errors::CallerUnauthorized);
                let state =&mut ctx.accounts.state;
                state.execution_gas_limit = new_gas_limt;
        Ok(())
    }

    // Function to create open liquidity position
    pub fn create_open_liquidity_position(ctx: Context<CreateOpenLiquidityPosition>, pool: Pubkey , margin : u128 , liquidity : u128 , value : u128  ) -> Result<u128> {
        // Logic to create open liquidity position
        // msg.value check 
        // external call to router 

        let state =&mut ctx.accounts.state;
        require!(state.min_execution_fee > value , Errors::InsufficientExecutionFee);
        let clock: Clock = Clock::get().unwrap();
        let clock2: Clock = Clock::get()?;
        let position = OpenLiquidityPositionRequest {
           account :  ctx.accounts.user.key(),
            pool : pool,
            blockNumber : clock.slot as u128,
            liquidity: liquidity,
            executionFee : value , // change it to msg.value after wards 
            margin : margin , 
            blockTime :  clock2.unix_timestamp as u128

        };
        let positions = &mut state.open_liquidity_position_requests;
        positions.push(position);
        emit!(OpenLiquidityPositionRequestEvent{    account :  ctx.accounts.user.key(),
            pool : pool,
            blockNumber : clock.slot as u128,
            liquidity: liquidity,
            executionFee : value , // change it to msg.value after wards 
            margin : margin , 
            blockTime :  clock2.unix_timestamp as u128});
        Ok((positions.len() as u128).into() )
    }

    pub fn cancel_open_liquidity_position(ctx: Context<CreateOpenLiquidityPosition>, index : usize , execution_fee_reciever : Pubkey) -> Result<bool>{
        // transfer eth out 
        let state =&mut ctx.accounts.state;
        let clock: Clock = Clock::get().unwrap();
        let clock2: Clock = Clock::get()?;
        let state =&mut ctx.accounts.state;
        let position = state.open_liquidity_position_requests.get(index);
        // _should_cancel(block_number : u128 , position_block_time : u128, account : Pubkey , block_timestamp : u128 , min_block_delayer_executor :u128  , sender : Pubkey)
        if let Some(position) = state.open_liquidity_position_requests.get(index) {
            // Now you can directly access fields of `position`
            let should_cancel = _should_cancel(position.blockNumber, position.blockTime, position.account, clock.unix_timestamp as u128, state.min_block_delayer_executor, ctx.accounts.user.key())?;
            require!(should_cancel , Errors::CallerUnauthorized);
            // Logic to remove from vector if needed
            // state.open_liquidity_position_requests.remove(index);
        } else {
            // Handle the case where there is no position at the index
            msg!("Position at index {} does not exist.", index);
        }
        emit!(OpenLiquidityPositionCancelled{index : index as u128 , 
        reciever : execution_fee_reciever});
        state.open_liquidity_position_requests.remove(index);
        Ok(true)
    }

    pub fn execute_open_liquidity_position(
        ctx: Context<CreateOpenLiquidityPosition>,
        index: usize,
        execution_fee_receiver: Pubkey,
    ) -> Result<bool> {
        

        // usdc transfer 
        // external call to plugin 
        // transfer out eth 
        let state =&mut ctx.accounts.state;
        let clock: Clock = Clock::get().unwrap();
        let clock2: Clock = Clock::get()?;
        let state =&mut ctx.accounts.state;
        let position = state.open_liquidity_position_requests.get(index);
        if let Some(position) = state.open_liquidity_position_requests.get(index) {
            // Now you can directly access fields of `position`
            let should_cancel = _should_cancel(position.blockNumber, position.blockTime, position.account, clock.unix_timestamp as u128, state.min_block_delayer_executor, ctx.accounts.user.key())?;
            require!(should_cancel , Errors::CallerUnauthorized);
            // Logic to remove from vector if needed
            // state.open_liquidity_position_requests.remove(index);
        } else {
            // Handle the case where there is no position at the index
            msg!("Position at index {} does not exist.", index);
        }
        state.open_liquidity_position_requests.remove(index.try_into().unwrap());
        emit!(OpenLiquidityPositionExecuted{index : index as u128 , 
            reciever : execution_fee_receiver});
        Ok(true)
    }

    pub fn create_close_liquidity_position(
        ctx: Context<CreateOpenLiquidityPosition>,
        pool: Pubkey,
        position_id: u128,
        receiver: Pubkey,
    ) -> Result<u128> {
        // Logic to create open liquidity position
        // msg.value check 
        // external call to router 
        let value = 100;
        let state =&mut ctx.accounts.state;
        let clock: Clock = Clock::get().unwrap();
        let clock2: Clock = Clock::get()?;
        let position = CloseLiquidityPositionRequest {
           account :  ctx.accounts.user.key(),
            pool : pool,
            positionID : position_id , 
            blockNumber : clock.slot as u128 ,
            receiver: receiver,
            executionFee : value , // change it to msg.value after wards
            blockTime :  clock2.unix_timestamp as u128

        };
        let positions = &mut state.close_liquidity_position_requests;
        positions.push(position);
        emit!(CloseLiquidityPositionRequestEvent{    account :  ctx.accounts.user.key(),
            pool : pool,
            blockNumber : clock.slot as u128,

            executionFee : value , // change it to msg.value after wards 
         
            blockTime :  clock2.unix_timestamp as u128});
        Ok((positions.len() as u128).into() ) 
    }

    pub fn cancel_close_liquidity_position(
        ctx: Context<CreateOpenLiquidityPosition>,
        index: usize,
        execution_fee_receiver: Pubkey,
    ) -> Result<bool> {
        // should cancel check to be added 
        // transfer eth out 
        let clock: Clock = Clock::get().unwrap();
        let state =&mut ctx.accounts.state;
        if let Some(position) = state.close_liquidity_position_requests.get(index) {
            // Now you can directly access fields of `position`
            let should_cancel = _should_cancel(position.blockNumber, position.blockTime, position.account, clock.unix_timestamp as u128, state.min_block_delayer_executor, ctx.accounts.user.key())?;
            require!(should_cancel , Errors::CallerUnauthorized);
            // Logic to remove from vector if needed
            // state.open_liquidity_position_requests.remove(index);
        } else {
            // Handle the case where there is no position at the index
            msg!("Position at index {} does not exist.", index);
        }
        state.close_liquidity_position_requests.remove(index.try_into().unwrap());
        Ok(true)
    }

    pub fn execute_close_liquidity_position(
        ctx: Context<CreateOpenLiquidityPosition>,
        index: usize,
        execution_fee_receiver: Pubkey,
    ) -> Result<bool> {
        // should execute check 
        // usdc transfer 
        // external call to plugin 
        // transfer out eth 
        let clock: Clock = Clock::get().unwrap();
        let state =&mut ctx.accounts.state;
        if let Some(position) = state.close_liquidity_position_requests.get(index) {
            // Now you can directly access fields of `position`
            let should_cancel = _should_cancel(position.blockNumber, position.blockTime, position.account, clock.unix_timestamp as u128, state.min_block_delayer_executor, ctx.accounts.user.key())?;
            require!(should_cancel , Errors::CallerUnauthorized);
            // Logic to remove from vector if needed
            // state.open_liquidity_position_requests.remove(index);
        } else {
            // Handle the case where there is no position at the index
            msg!("Position at index {} does not exist.", index);
        }
        state.close_liquidity_position_requests.remove(index.try_into().unwrap());
        
        Ok(true)
    }

    pub fn create_adjust_liquidity_position_margin(
        ctx: Context<CreateOpenLiquidityPosition>,
        pool: Pubkey, 
        position_id: u128, 
        margin_delta: u128, 
        receiver: Pubkey,
    ) -> Result<()> {
            // Logic to create open liquidity position
        // msg.value check 
        // external call to router 
        let value = 100;
        let state =&mut ctx.accounts.state;
        let clock: Clock = Clock::get().unwrap();
        let clock2: Clock = Clock::get()?;
        let position = AdjustLiquidityPositionMarginRequest {
           account :  ctx.accounts.user.key(),
            pool : pool,
            blockNumber : clock.slot as u128,
            executionFee : value , // change it to msg.value after wards 
           receiver : receiver,
            blockTime :  clock2.unix_timestamp as u128,
            positionID : position_id , 
            margin_delta : margin_delta , 

        };
        let positions = &mut state.adjust_liquidity_position_margin_requests;
        positions.push(position);
        Ok(()) 
        
    }

    pub fn cancel_adjust_liquidity_position_margin(
        ctx: Context<CreateOpenLiquidityPosition>,
        index: usize,
    ) -> Result<bool> {
              // should cancel check to be added 
        // transfer eth out 
        let state =&mut ctx.accounts.state;
        let clock: Clock = Clock::get().unwrap();
        let state =&mut ctx.accounts.state;
        if let Some(position) = state.adjust_liquidity_position_margin_requests.get(index) {
            // Now you can directly access fields of `position`

            let should_cancel = _should_cancel(position.blockNumber, position.blockTime, position.account, clock.unix_timestamp as u128, state.min_block_delayer_executor, ctx.accounts.user.key())?;
            if should_cancel{
                return Ok(false);
            }
            if position.account ==  Pubkey::default(){
                return Ok(true);
            }

            if position.margin_delta > 0 {
                // USDC transfer 
            } 
            // Logic to remove from vector if needed
            // state.open_liquidity_position_requests.remove(index);
        } else {
            // Handle the case where there is no position at the index
            msg!("Position at index {} does not exist.", index);
        }
        state.adjust_liquidity_position_margin_requests.remove(index.try_into().unwrap());
        Ok(true)

    }
    
    pub fn execute_adjust_liquidity_position_margin(
        ctx: Context<CreateOpenLiquidityPosition>,
        index: usize,
    ) -> Result<bool> {
        // should execute check 
        // usdc transfer 
        // external call to plugin 
        // transfer out eth 
        let clock: Clock = Clock::get().unwrap();
        let state =&mut ctx.accounts.state;
        if let Some(position) = state.adjust_liquidity_position_margin_requests.get(index) {
            // Now you can directly access fields of `position`

            let should_cancel = _should_cancel(position.blockNumber, position.blockTime, position.account, clock.unix_timestamp as u128, state.min_block_delayer_executor, ctx.accounts.user.key())?;
            if should_cancel{
                return Ok(false);
            }
            if position.account ==  Pubkey::default(){
                return Ok(true);
            }

            if position.margin_delta > 0 {
                // USDC transfer 
            } 
            // Logic to remove from vector if needed
            // state.open_liquidity_position_requests.remove(index);
        } else {
            // Handle the case where there is no position at the index
            msg!("Position at index {} does not exist.", index);
        }
        state.adjust_liquidity_position_margin_requests.remove(index.try_into().unwrap());
        
        Ok(true)
    }

    pub fn create_increase_risk_buffer_fund_position(
        ctx: Context<CreateOpenLiquidityPosition>,
        pool: Pubkey, 
        liquidity_delta: u128,
    ) -> Result<u128> {
  // Logic to create open liquidity position
        // msg.value check 
        // external call to router 
        let value = 100;
        let state =&mut ctx.accounts.state;
        let clock: Clock = Clock::get().unwrap();
        let clock2: Clock = Clock::get()?;
        let position = IncreaseRiskBufferFundPositionRequest {
           account :  ctx.accounts.user.key(),
            pool : pool,
            blockNumber : clock.slot as u128 ,
            executionFee : value , // change it to msg.value after wards
            blockTime :  clock2.unix_timestamp as u128,
            liquidityDelta : liquidity_delta , 

        };
        let positions = &mut state.increase_risk_buffer_fund_position_request;
        positions.push(position);
        Ok((positions.len() as u128).into() ) 
    
        
    }

    pub fn cancel_increase_risk_buffer_fund_position(
        ctx: Context<CreateOpenLiquidityPosition>,
        index: usize,
    ) -> Result<bool> {
  // should cancel check to be added 
        // transfer eth out 

        let clock: Clock = Clock::get().unwrap();
        let state =&mut ctx.accounts.state;
        if let Some(position) = state.increase_risk_buffer_fund_position_request.get(index) {
            // Now you can directly access fields of `position`

            let should_cancel = _should_cancel(position.blockNumber, position.blockTime, position.account, clock.unix_timestamp as u128, state.min_block_delayer_executor, ctx.accounts.user.key())?;
            if should_cancel{
                return Ok(false);
            }
            if position.account ==  Pubkey::default(){
                return Ok(true);
            }
            // Logic to remove from vector if needed
            // state.open_liquidity_position_requests.remove(index);
        } else {
            // Handle the case where there is no position at the index
            msg!("Position at index {} does not exist.", index);
        }
        state.increase_risk_buffer_fund_position_request.remove(index.try_into().unwrap());
            
        Ok(true)
    }

    pub fn execute_increase_risk_buffer_fund_position(
        ctx: Context<CreateOpenLiquidityPosition>,
        index: usize,
    ) -> Result<bool> {
     // should execute check 
        // usdc transfer 
        // external call to plugin 
        // transfer out eth 
        let clock: Clock = Clock::get().unwrap();
        let state =&mut ctx.accounts.state;
        if let Some(position) = state.increase_risk_buffer_fund_position_request.get(index) {
            // Now you can directly access fields of `position`

            let should_cancel = _should_cancel(position.blockNumber, position.blockTime, position.account, clock.unix_timestamp as u128, state.min_block_delayer_executor, ctx.accounts.user.key())?;
            if should_cancel{
                return Ok(false);
            }
            if position.account ==  Pubkey::default(){
                return Ok(true);
            }
            // Logic to remove from vector if needed
            // state.open_liquidity_position_requests.remove(index);
        } else {
            // Handle the case where there is no position at the index
            msg!("Position at index {} does not exist.", index);
        }
        state.increase_risk_buffer_fund_position_request.remove(index.try_into().unwrap());
       
    
        Ok(true)
    }
    
    pub fn create_decrease_risk_buffer_fund_position(
        ctx: Context<CreateOpenLiquidityPosition>,
        pool: Pubkey, 
        liquidity_delta: u128, 
        receiver: Pubkey,
    ) -> Result<u128> {
        let value = 100;
        let state =&mut ctx.accounts.state;
        let clock: Clock = Clock::get().unwrap();
        let clock2: Clock = Clock::get()?;
        let position = DecreaseRiskBufferFundPositionRequest {
           account :  ctx.accounts.user.key(),
            pool : pool,
            receiver : receiver , 
            liquidityDelta : liquidity_delta , 
            blockTime : clock.unix_timestamp as u128 , 
            blockNumber : clock.slot as u128, 
            executionFee : 0 // add msg.value as the execution fee 
        };
        let positions = &mut state.decrease_risk_buffer_fund_position_request;
        positions.push(position);
        Ok((positions.len() as u128).into() ) 
    }

    pub fn cancel_decrease_risk_buffer_fund_position(
        ctx: Context<CreateOpenLiquidityPosition>,
        index: usize,
    ) -> Result<bool> {
        // Function logic here
        let clock: Clock = Clock::get().unwrap();
        let state =&mut ctx.accounts.state;
        if let Some(position) = state.decrease_risk_buffer_fund_position_request.get(index) {
            // Now you can directly access fields of `position`

            let should_cancel = _should_cancel(position.blockNumber, position.blockTime, position.account, clock.unix_timestamp as u128, state.min_block_delayer_executor, ctx.accounts.user.key())?;
            if should_cancel{
                return Ok(false);
            }
            if position.account ==  Pubkey::default(){
                return Ok(true);
            }
            // Logic to remove from vector if needed
            // state.open_liquidity_position_requests.remove(index);
        } else {
            // Handle the case where there is no position at the index
            msg!("Position at index {} does not exist.", index);
        }
        state.decrease_risk_buffer_fund_position_request.remove(index.try_into().unwrap());
        Ok(true) // Placeholder for the cancellation success status
    }
    
    pub fn execute_decrease_risk_buffer_fund_position(
        ctx: Context<CreateOpenLiquidityPosition>,
        index: usize,
    ) -> Result<bool> {
        let state =&mut ctx.accounts.state;
        let clock: Clock = Clock::get().unwrap();
        if let Some(position) = state.decrease_risk_buffer_fund_position_request.get(index) {
            // Now you can directly access fields of `position`

            let should_cancel = _should_cancel(position.blockNumber, position.blockTime, position.account, clock.unix_timestamp as u128, state.min_block_delayer_executor, ctx.accounts.user.key())?;
            if should_cancel{
                return Ok(false);
            }
            if position.account ==  Pubkey::default(){
                return Ok(true);
            }
            // Logic to remove from vector if needed
            // state.open_liquidity_position_requests.remove(index);
        } else {
            // Handle the case where there is no position at the index
            msg!("Position at index {} does not exist.", index);
        }
        state.decrease_risk_buffer_fund_position_request.remove(index.try_into().unwrap());
        Ok(true) // Placeholder for the cancellation success status
    }

    pub fn create_increase_position(
        ctx: Context<CreateOpenLiquidityPosition>,
        pool : Pubkey , 
        side: bool, // Assuming Side is a boolean for simplicity
        margin_delta: u128,
        size_delta: u128,
        acceptable_trade_price_x96: u128,
    ) -> Result<u128> {
       // Logic to create open liquidity position
        // msg.value check 
        // external call to router 
        let value = 100;
        let state =&mut ctx.accounts.state;
        let clock: Clock = Clock::get().unwrap();
        let clock2: Clock = Clock::get()?;
        let position = IncreasePositionRequest {
           account :  ctx.accounts.user.key(),
            pool : pool,
            side : side ,
            blockNumber : clock.slot as u128 ,
            sizeDelta : size_delta , 
            marginDelta : margin_delta , 
            acceptableTradePriceX96: acceptable_trade_price_x96,
            executionFee : value , // change it to msg.value after wards
            blockTime :  clock2.unix_timestamp as u128

        };
        let positions = &mut state.increase_position_request;
        positions.push(position);
        Ok(((positions.len() -1 ) as u128).into() ) 
    }

    pub fn cancel_increase_position(
        ctx: Context<CreateOpenLiquidityPosition>,
        index: usize, // Assuming index is passed to identify the request
    ) -> Result<bool> {
      // Function logic here
      let clock: Clock = Clock::get().unwrap();
      let state =&mut ctx.accounts.state;
      if let Some(position) = state.increase_position_request.get(index) {
          // Now you can directly access fields of `position`

          let should_cancel = _should_cancel(position.blockNumber, position.blockTime, position.account, clock.unix_timestamp as u128, state.min_block_delayer_executor, ctx.accounts.user.key())?;
          if should_cancel{
              return Ok(false);
          }
          if position.account ==  Pubkey::default(){
              return Ok(true);
          }

          if position.marginDelta > 0 {
            // perform the usdc transfer 
          }
          // Logic to remove from vector if needed
          // state.open_liquidity_position_requests.remove(index);
      } else {
          // Handle the case where there is no position at the index
          msg!("Position at index {} does not exist.", index);
      }
      state.increase_position_request.remove(index.try_into().unwrap());
      Ok(true) // Placeholder for the cancellation success status
    }

    pub fn execute_increase_position(
        ctx: Context<CreateOpenLiquidityPosition>,
        index: usize,
    ) -> Result<bool> {
        let clock: Clock = Clock::get().unwrap();
        let state =&mut ctx.accounts.state;
        if let Some(position) = state.increase_position_request.get(index) {
            // Now you can directly access fields of `position`
  
            let should_cancel = _should_cancel(position.blockNumber, position.blockTime, position.account, clock.unix_timestamp as u128, state.min_block_delayer_executor, ctx.accounts.user.key())?;
            if should_cancel{
                return Ok(false);
            }
            if position.account ==  Pubkey::default(){
                return Ok(true);
            }
  
            if position.marginDelta > 0 {
              // perform the usdc transfer 
            }
            // Logic to remove from vector if needed
            // state.open_liquidity_position_requests.remove(index);
        } else {
            // Handle the case where there is no position at the index
            msg!("Position at index {} does not exist.", index);
        }
        state.increase_position_request.remove(index.try_into().unwrap());
        Ok(true) // Placeholder for the cancellation success status
    }

    pub fn create_decrease_position(ctx: Context<CreateOpenLiquidityPosition>, margin_delta: u128, size_delta: u128, acceptable_trade_price_x96: u128, receiver: Pubkey , side : bool , pool : Pubkey ) -> Result<u128> {
      // Logic to create open liquidity position
        // msg.value check 
        // external call to router 
        let value = 100;
        let state =&mut ctx.accounts.state;
        let clock: Clock = Clock::get().unwrap();
        let clock2: Clock = Clock::get()?;
        let position = DecreasePositionRequest {
           account :  ctx.accounts.user.key(),
            pool : pool,
            side : side ,
            blockNumber : clock.slot as u128 ,
            sizeDelta : size_delta , 
            marginDelta : margin_delta , 
            acceptableTradePriceX96: acceptable_trade_price_x96,
            executionFee : value , // change it to msg.value after wards
            blockTime :  clock2.unix_timestamp as u128

        };
        let positions = &mut state.decrease_position_request;
        positions.push(position);
        Ok((positions.len() as u128).into() ) 
    }

    pub fn cancel_decrease_position(ctx: Context<CreateOpenLiquidityPosition>, index: usize, execution_fee_receiver: Pubkey) -> Result<bool> {
        let clock: Clock = Clock::get().unwrap();
        let state =&mut ctx.accounts.state;
        if let Some(position) = state.decrease_position_request.get(index) {
            // Now you can directly access fields of `position`
  
            let should_cancel = _should_cancel(position.blockNumber, position.blockTime, position.account, clock.unix_timestamp as u128, state.min_block_delayer_executor, ctx.accounts.user.key())?;
            if should_cancel{
                return Ok(false);
            }
            if position.account ==  Pubkey::default(){
                return Ok(true);
            }
  
            if position.marginDelta > 0 {
              // perform the usdc transfer 
            }
            // Logic to remove from vector if needed
            // state.open_liquidity_position_requests.remove(index);
        } else {
            // Handle the case where there is no position at the index
            msg!("Position at index {} does not exist.", index);
        }
      state.decrease_position_request.remove(index.try_into().unwrap());
      Ok(true) // Placeholder for the cancellation success status
    }

    pub fn execute_decrease_position(ctx: Context<CreateOpenLiquidityPosition>, index: usize, execution_fee_receiver: Pubkey) -> Result<bool> {
        let clock: Clock = Clock::get().unwrap();
        let state =&mut ctx.accounts.state;
        if let Some(position) = state.decrease_position_request.get(index) {
            // Now you can directly access fields of `position`
  
            let should_cancel = _should_cancel(position.blockNumber, position.blockTime, position.account, clock.unix_timestamp as u128, state.min_block_delayer_executor, ctx.accounts.user.key())?;
            if should_cancel{
                return Ok(false);
            }
            if position.account ==  Pubkey::default(){
                return Ok(true);
            }
  
            if position.marginDelta > 0 {
              // perform the usdc transfer 
            }

            // if position.acceptable_trade_price !=0 {
            //     _validate_trade_price_X96(position.side , )
            // }
            // Logic to remove from vector if needed
            // state.open_liquidity_position_requests.remove(index);
        } else {
            // Handle the case where there is no position at the index
            msg!("Position at index {} does not exist.", index);
        }
      state.decrease_position_request.remove(index.try_into().unwrap());
      Ok(true) // Placeholder for the cancellation success status
    }
    

}

pub fn _validate_trade_price_X96(side : bool , trade_price : u128 , acceptable_trade_price : u128) -> Result<()> {
    if (side && trade_price > acceptable_trade_price) || (!side && trade_price < acceptable_trade_price){
        return err!(Errors::InvalidOperation)
    }

    Ok(())
}

pub fn _should_execute_or_cancel(position_block_number : u128 , position_block_time : u128 , account : Pubkey , sender : Pubkey , min_block_delayer_executor :u128 , block_timestamp : u128   ) -> Result<bool> {
    let is_execute_call =  true ;// placeholder call , add the only executor check 
    require!(account == sender , Errors::CallerUnauthorized);
    if (position_block_time + min_block_delayer_executor ) > block_timestamp {
        return err!(Errors::CallerUnauthorized)
    }

    
    Ok(true)
}

pub fn _should_execute(block_number : u128 , position_block_time : u128, account : Pubkey , block_timestamp : u128 , min_block_delayer_executor :u128 , max_time_delay : u128 , sender : Pubkey) -> Result<bool> {
    if (position_block_time + max_time_delay) <= block_timestamp {
        return err!(Errors::CallerUnauthorized)
    }

    return _should_execute_or_cancel(block_number , position_block_time , account , sender , min_block_delayer_executor , block_timestamp);
}

pub fn _should_cancel(block_number : u128 , position_block_time : u128, account : Pubkey , block_timestamp : u128 , min_block_delayer_executor :u128  , sender : Pubkey) -> Result<bool> {
    return _should_execute_or_cancel(block_number , position_block_time , account , sender , min_block_delayer_executor , block_timestamp);
}

#[account]
pub struct ContractState {
    usd : Pubkey,
    router : Pubkey ,
    min_execution_fee :u128,
    min_block_delayer_executor : u128 ,
    min_time_delay :u128, 
    max_time_delay :u128, 
    execution_gas_limit :u128 ,
    executors : Vec<Pubkey> ,
    pub open_liquidity_position_requests : Vec<OpenLiquidityPositionRequest>,
    pub close_liquidity_position_requests : Vec<CloseLiquidityPositionRequest>,
    pub adjust_liquidity_position_margin_requests : Vec<AdjustLiquidityPositionMarginRequest>,
    pub increase_risk_buffer_fund_position_request : Vec<IncreaseRiskBufferFundPositionRequest>,
    pub decrease_risk_buffer_fund_position_request : Vec<DecreaseRiskBufferFundPositionRequest>,
    pub increase_position_request : Vec<IncreasePositionRequest>,
    pub decrease_position_request : Vec<DecreasePositionRequest>,
    initilized : bool,



}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct OpenLiquidityPositionRequest{
    account : Pubkey, 
    blockNumber : u128,
    pool : Pubkey  ,
    blockTime : u128 ,
    margin : u128,
    liquidity : u128,
    executionFee : u128 ,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct CloseLiquidityPositionRequest{
    account: Pubkey,
    pool: Pubkey ,
    positionID: u128,
    receiver: Pubkey,
    executionFee: u128,
    blockNumber: u128,
    blockTime: u128 , 
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct AdjustLiquidityPositionMarginRequest{
    account: Pubkey,
    pool: Pubkey ,
    positionID: u128,
    receiver: Pubkey,
    executionFee: u128,
    blockNumber: u128,
    blockTime: u128 , 
    margin_delta : u128 , 
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct IncreaseRiskBufferFundPositionRequest{
    account: Pubkey,
    pool: Pubkey,
    liquidityDelta: u128,
    executionFee: u128,
    blockNumber: u128,
    blockTime: u128 , 
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct DecreaseRiskBufferFundPositionRequest{
    account: Pubkey,
    pool: Pubkey,
    liquidityDelta: u128,
    receiver: Pubkey,
    blockNumber : u128 , 
    executionFee : u128, 
    blockTime : u128 ,

}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct IncreasePositionRequest{
    account : Pubkey,
    pool : Pubkey,
    side : bool,
    marginDelta : u128,
    sizeDelta : u128 ,
    blockNumber : u128,
    blockTime :u128,
    acceptableTradePriceX96 : u128,
    executionFee:u128,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct DecreasePositionRequest{
    account : Pubkey,
    pool : Pubkey,
    side : bool,
    marginDelta : u128,
    sizeDelta : u128 ,
    blockNumber : u128,
    blockTime :u128,
    acceptableTradePriceX96 : u128,
    executionFee:u128,
}
// Context for Initialize function
#[derive(Accounts)]
pub struct Initialize<'info> {
    /// CHECK
    pub state: Account<'info, ContractState>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateExecutor<'info> {
    /// CHECK
    pub state: Account<'info, ContractState>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// Context for UpdatePositionExecutor function
#[derive(Accounts)]
pub struct UpdatePositionExecutor<'info> {
    /// CHECK
    #[account(mut)]
    pub state: Account<'info, ContractState>,
    // Include other accounts as needed, such as the signer to authorize the update
    /// CHECK
    #[account(signer)]
    pub user: AccountInfo<'info>,
}

// ... Additional Context structs for other functions

// PositionRouter state account


// Struct corresponding to IncreasePositionRequest in Solidity
#[derive(Accounts)]
pub struct IncreasePositionRequestContext<'info> {
    // Fields from your Solidity struct...
    /// CHECK
    #[account(mut)]
    pub position_router_state: Account<'info, ContractState>,
    // Include other accounts as needed, such as the signer to authorize the update
    /// CHECK
    #[account(signer)]
    pub authority: AccountInfo<'info>,
}

// Struct corresponding to DecreasePositionRequest in Solidity
#[derive(Accounts)]
pub struct DecreasePositionRequestContext<'info> {
    /// CHECK
    // Fields from your Solidity struct...
    #[account(mut)]
    pub position_router_state: Account<'info, ContractState>,
    // Include other accounts as needed, such as the signer to authorize the update
    /// CHECK
    #[account(signer)]
    pub authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CreateOpenLiquidityPosition<'info> {
    /// CHECK
    // Fields from your Solidity struct...
    #[account(mut)]
    pub state: Account<'info, ContractState>,
    // Include other accounts as needed, such as the signer to authorize the update
    /// CHECK
    #[account(signer)]
    pub user: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdateDelayValues<'info> {
    /// CHECK
    // Fields from your Solidity struct...
    #[account(mut)]
    pub state: Account<'info, ContractState>,
    // Include other accounts as needed, such as the signer to authorize the update
    /// CHECK
    #[account(signer)]
    pub user: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdateMinExecutionFee<'info> {
    /// CHECK
    // Fields from your Solidity struct...
    #[account(mut)]
    pub state: Account<'info, ContractState>,
    // Include other accounts as needed, such as the signer to authorize the update
    /// CHECK
    #[account(signer)]
    pub user: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdateExecutionGasLimit<'info> {
    /// CHECK
    // Fields from your Solidity struct...
    #[account(mut)]
    pub state: Account<'info, ContractState>,
    // Include other accounts as needed, such as the signer to authorize the update
    /// CHECK
    #[account(signer)]
    pub user: AccountInfo<'info>,
}
// ... Additional data structures as per your contract

// Custom errors
#[error_code]
pub enum Errors {
    #[msg("Unauthorized access")]
    CallerUnauthorized,
    #[msg("Invalid operation")]
    InvalidOperation,
    // ... Define other custom errors
    #[msg("Already Initilized")]
    AlreadyInitlized,
    #[msg("Insufficient execution fee")]
    InsufficientExecutionFee,
}

#[event]
pub struct MinExecutionFeeUpdated {
    min_execution_fee : u128 , 
}

#[event]
pub struct OpenLiquidityPositionRequestEvent {
    account :  Pubkey,
    pool : Pubkey,
    blockNumber : u128,
    liquidity: u128,
    executionFee : u128 ,  
    margin : u128, 
    blockTime :  u128,
}

#[event]
pub struct  OpenLiquidityPositionCancelled{
    reciever : Pubkey,
    index : u128,
}

#[event]
pub struct  OpenLiquidityPositionExecuted{
    reciever : Pubkey,
    index : u128,
}
#[event]
pub struct CloseLiquidityPositionRequestEvent {
    account :  Pubkey,
    pool : Pubkey,
    blockNumber : u128,
    executionFee : u128 ,  
    blockTime :  u128,
}