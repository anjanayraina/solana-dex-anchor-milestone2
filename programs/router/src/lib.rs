use anchor_lang::{
    prelude::*
};
const GOVERNOR_PUBKEY: Pubkey = Pubkey::new_from_array([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32]);


declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");


#[program]
mod router {
    use super::*;
    pub fn initilize(ctx: Context<SetData>, efc: Pubkey , reward_farm : Pubkey , fee_distributor : Pubkey) -> Result<()> {
        require!(ctx.accounts.authorized_account.key() == GOVERNOR_PUBKEY, MyError::Unauthorized);     
        let state =&mut ctx.accounts.state;
        require!(!state.initilized , MyError::AlreadyInitlized );
        state.efc= efc;
        state.reward_farm = reward_farm;
        state.fee_distributor = fee_distributor;
        
        Ok(())
    }

    
    pub fn add_plugin(ctx: Context<UpdateExecutor>, new_plugin: Pubkey) -> Result<()> {
        // Ensure the caller is the governor
        require!(ctx.accounts.authorized_account.key() == GOVERNOR_PUBKEY, MyError::CallerUnauthorized);

        // Add new executor to the list
        let governance_state = &mut ctx.accounts.state;
        governance_state.executors.push(new_plugin);
        Ok(())
    }

        // Function to update executor
    pub fn remove_plugin(ctx: Context<UpdateExecutor>, plugin_address: Pubkey) -> Result<()> {
            require!(ctx.accounts.authorized_account.key() == GOVERNOR_PUBKEY, MyError::CallerUnauthorized);
            let address_list = &mut ctx.accounts.state.executors;
            address_list.retain(|&x| x != plugin_address);
            // Logic to update executor
            Ok(())
        }

            
    pub fn add_liquidator(ctx: Context<UpdateExecutor>, new_liquidator: Pubkey) -> Result<()> {
        // Ensure the caller is the governor
        require!(ctx.accounts.authorized_account.key() == GOVERNOR_PUBKEY, MyError::CallerUnauthorized);

        // Add new executor to the list
        let governance_state = &mut ctx.accounts.state;
        governance_state.liquidators.push(new_liquidator);
        Ok(())
    }

        // Function to update executor
    pub fn remove_liquidator(ctx: Context<UpdateExecutor>, liquidator_address: Pubkey) -> Result<()> {
            require!(ctx.accounts.authorized_account.key() == GOVERNOR_PUBKEY, MyError::CallerUnauthorized);
            let address_list = &mut ctx.accounts.state.liquidators;
            address_list.retain(|&x| x != liquidator_address);
            // Logic to update executor
            Ok(())
        }

    pub fn plugin_transfer(ctx: Context<PluginTransfer>, amount: u128, from: Pubkey, to: Pubkey) -> Result<()> {
        let address_list = &mut ctx.accounts.state.executors;
        let user_pubkey = ctx.accounts.user.key();
        require!(address_list.contains(&user_pubkey) , MyError::CallerUnauthorized);
        // token transfer logic
        Ok(())
    }

    pub fn plugin_transfer_nft(ctx: Context<PluginTransferNFT>, from: Pubkey , to:Pubkey , tokenID:u128) -> Result<()> {
        let address_list = &mut ctx.accounts.state.executors;
        let user_pubkey = ctx.accounts.user.key();
        require!(address_list.contains(&user_pubkey) , MyError::CallerUnauthorized);
        
        //token transfer
        Ok(())
    }

    pub fn plugin_open_liquidity_position(ctx: Context<LiquidityPosition>, account: Pubkey, margin:u128, liquidity:u128 , pool : Pubkey ) -> Result<u128> {
        let address_list = &mut ctx.accounts.state.executors;
        let user_pubkey = ctx.accounts.user.key();
        require!(address_list.contains(&user_pubkey) , MyError::CallerUnauthorized); 
        /// external call to pool
        Ok(100)
    }

    pub fn plugin_close_liquidity_position(ctx: Context<LiquidityPosition>,  _positionID:u128 ,  _receiver:Pubkey ) -> Result<()> {
        let address_list = &mut ctx.accounts.state.executors;
        let user_pubkey = ctx.accounts.user.key();
        require!(address_list.contains(&user_pubkey) , MyError::CallerUnauthorized);   
        // extrernal call to pool
      Ok(())
    }

    pub fn plugin_adjust_liquidity_position_margin(ctx: Context<LiquidityPosition>, _pool:Pubkey,
        _positionID:u128,
        _marginDelta:u128,
        _receiver:Pubkey) -> Result<()> {
            let address_list = &mut ctx.accounts.state.executors;
            let user_pubkey = ctx.accounts.user.key();
            require!(address_list.contains(&user_pubkey) , MyError::CallerUnauthorized);
        // return a u128 value
      Ok(())
    }

    
      // Increase the liquidity of a risk buffer fund position
      pub fn plugin_increase_risk_buffer_fund_position(
        ctx: Context<RiskBufferFundPosition>, 
        account: Pubkey, 
        liquidity_delta: u128
    ) -> Result<()> {
        let address_list = &mut ctx.accounts.state.executors;
        let user_pubkey = ctx.accounts.user.key();
        require!(address_list.contains(&user_pubkey) , MyError::CallerUnauthorized);  
        Ok(())
    }

    
    // Decrease the liquidity of a risk buffer fund position
    pub fn plugin_decrease_risk_buffer_fund_position(
        ctx: Context<RiskBufferFundPosition>, 
        account: Pubkey, 
        liquidity_delta: u128, 
        receiver: Pubkey
    ) -> Result<()> {
        let address_list = &mut ctx.accounts.state.executors;
        let user_pubkey = ctx.accounts.user.key();
        require!(address_list.contains(&user_pubkey) , MyError::CallerUnauthorized);
        Ok(())
    }

    // Increase the margin/liquidity of a position
    pub fn plugin_increase_position(
        ctx: Context<PositionManagement>, 
        account: Pubkey, 
        side: bool, 
        margin_delta: u128, 
        size_delta: u128
    ) -> Result<u128> {
        // TODO: Implement access control, position increase logic
        let address_list = &mut ctx.accounts.state.executors;
        let user_pubkey = ctx.accounts.user.key();
        require!(address_list.contains(&user_pubkey) , MyError::CallerUnauthorized);    
        Ok(0) // Placeholder for trade price
    }

    // Decrease the margin/liquidity of a position
    pub fn plugin_decrease_position(
        ctx: Context<PositionManagement>, 
        account: Pubkey, 
        side: bool, 
        margin_delta: u128, 
        size_delta: u128, 
        receiver: Pubkey
    ) -> Result<u128> {
        // TODO: Implement access control, position decrease logic
        let address_list = &mut ctx.accounts.state.executors;
        let user_pubkey = ctx.accounts.user.key();
        require!(address_list.contains(&user_pubkey) , MyError::CallerUnauthorized);    
        Ok(0) // Placeholder for trade price
    }

    // Close a position by the liquidator
    pub fn plugin_close_position_by_liquidator(
        ctx: Context<PositionManagement>, 
        _pool: Pubkey, 
        side: bool, 
        size_delta: u128, 
        receiver: Pubkey
    ) -> Result<()> {
        // TODO: Implement access control for liquidator, position closing logic
        let address_list = &mut ctx.accounts.state.executors;
        let user_pubkey = ctx.accounts.user.key();
        require!(address_list.contains(&user_pubkey) , MyError::CallerUnauthorized);
        
        Ok(())
    }

        // Collect the referral fee
    pub fn plugin_collect_referral_fee(
            ctx: Context<PositionManagement>, 
            pool: Pubkey, 
            referral_token: u128, 
            receiver: Pubkey
        ) -> Result<()> {
            // TODO: Implement access control for liquidator, position closing logic
            let address_list = &mut ctx.accounts.state.executors;
            let user_pubkey = ctx.accounts.user.key();
            require!(address_list.contains(&user_pubkey) , MyError::CallerUnauthorized);        
            Ok(())
        }



  


}

#[account] 
pub struct ContractState {

    efc : Pubkey,
    reward_farm : Pubkey , 
    fee_distributor : Pubkey ,
    initilized : bool,
    executors : Vec<Pubkey>,
    liquidators : Vec<Pubkey>,         

}

#[derive(Accounts)] 
pub struct UpdateExecutor<'info> {
          // Adjust space as needed
    /// CHECK
    #[account(signer)]
    pub authorized_account: AccountInfo<'info>,
    pub state: Account<'info, ContractState>,
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct PluginTransfer<'info> {
      // Adjust space as needed
    /// CHECK
    #[account(signer)]
    pub authorized_account: AccountInfo<'info>,
    pub state: Account<'info, ContractState>,
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct PluginTransferNFT<'info>  {
     // Adjust space as needed
    /// CHECK
    #[account(signer)]
    pub authorized_account: AccountInfo<'info>,
    pub state: Account<'info, ContractState>,
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct LiquidityPosition<'info>  {
     // Adjust space as needed
    /// CHECK
    #[account(signer)]
    pub authorized_account: AccountInfo<'info>,
    pub state: Account<'info, ContractState>,
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct RiskBufferFundPosition<'info>  {
     // Adjust space as needed
    /// CHECK
    #[account(signer)]
    pub authorized_account: AccountInfo<'info>,
    pub state: Account<'info, ContractState>,
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct PositionManagement<'info>  {
     // Adjust space as needed
    /// CHECK
    
    pub authorized_account: AccountInfo<'info>,
    pub state: Account<'info, ContractState>,
    pub user: Signer<'info>,

}

#[derive(Accounts)]
pub struct SetDataContext<'info>  {
     // Adjust space as needed
    /// CHECK
    #[account(mut)]
    pub authorized_account: AccountInfo<'info>,
    pub state: Account<'info, ContractState>,
    pub user: Signer<'info>,
}
#[derive(Accounts)]
pub struct ReferralManagement<'info> {
     // Adjust space as needed
    /// CHECK
    #[account(signer)]
    pub authorized_account: AccountInfo<'info>,
    pub state: Account<'info, ContractState>,
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct RewardManagement<'info>  {
     // Adjust space as needed
    /// CHECK
    #[account(signer)]
    pub authorized_account: AccountInfo<'info>,
    pub state: Account<'info, ContractState>,
    pub user: Signer<'info>,
}


#[account]
pub struct Pool {
    // Fields representing the state of the pool
}

#[account]
#[derive(Default)]
pub struct MyAccount {
    data: u128
}


#[derive(Accounts)]
pub struct SetData<'info> {
     // Adjust space as needed
    /// CHECK
    #[account(signer)]
    pub authorized_account: AccountInfo<'info>,
    pub state: Account<'info, ContractState>,
    pub user: Signer<'info>,
}

#[error_code]
pub enum MyError {
    #[msg("Unauthorized Caller")]
    Unauthorized
    ,
    #[msg["Unauthorsized caller"]]
    CallerUnauthorized,
    #[msg("Owner Mismatch")]
    OwnerMismatch , 
    #[msg("Program already initlized")]
    AlreadyInitlized,
}


// Context struct for the restricted_function
#[derive(Accounts)]
pub struct RestrictedFunction<'info> {
        /// CHECK: The caller account is marked as a signer, ensuring that the transaction is signed by the caller.

    // The account of the caller, which must sign the transaction
    #[account(signer)]
    pub caller: AccountInfo<'info>,
}



// Custom error definitions for the program
#[error_code]
pub enum SimpleAccessControlError {
    // Error when an unauthorized account tries to call the function
    #[msg("The caller is not authorized to perform this action")]
    Unauthorized,
    #[msg("Program already initlized")]
    AlreadyInitlized,
}

