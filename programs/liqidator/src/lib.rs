use anchor_lang::prelude::*;
use anchor_lang::InstructionData;
use anchor_lang::solana_program;
use anchor_lang::solana_program::program::invoke;
use router::cpi::accounts::PositionManagement;
use router::program::Router;
use router::{self , ContractState};
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnR");
const GOVERNOR_PUBKEY: Pubkey = Pubkey::new_from_array([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32]);
#[program]
pub mod liquidator {
    use std::{default, sync::mpsc::Receiver};

    use super::*;

     
    pub fn initialize(ctx: Context<Initialize> , _router : Pubkey , _pool_factory : Pubkey , _efc:Pubkey) -> Result<()> {
        require!(ctx.accounts.authorized_account.key() == GOVERNOR_PUBKEY, MyError::CallerUnauthorized);
        let state =&mut ctx.accounts.state;
        require!(!state.initilized , MyError::AlreadyInitlized );
        state.initilized = true;
        state.router = _router;
        state.pool_factory = _pool_factory;
        state.efc = _efc;
        Ok(())
    }

    // Function to update price feed
    pub fn update_price_feed(ctx: Context<UpdatePriceFeed> , _price_feed : Pubkey) -> Result<()> {
        require!(ctx.accounts.authorized_account.key() == GOVERNOR_PUBKEY, MyError::CallerUnauthorized);
        // Logic to update price feed
        let state =&mut ctx.accounts.state;
        state.price_feed= _price_feed;

        Ok(())
    }

    pub fn add_executor(ctx: Context<GovernanceAction>, new_executor: Pubkey) -> Result<()> {
        // Ensure the caller is the governor
        require!(ctx.accounts.authorized_account.key() == GOVERNOR_PUBKEY, MyError::CallerUnauthorized);

        // Add new executor to the list
        let governance_state = &mut ctx.accounts.governance_state;
        governance_state.executors.push(new_executor);
        Ok(())
    }

    // Function to update executor
    pub fn remove_executor(ctx: Context<UpdateExecutor>, executor: Pubkey) -> Result<()> {
        require!(ctx.accounts.authorized_account.key() == GOVERNOR_PUBKEY, MyError::CallerUnauthorized);
        let address_list = &mut ctx.accounts.state.executors;
        address_list.retain(|&x| x != executor);
        // Logic to update executor
        Ok(())
    }

    // Function to liquidate liquidity position
    pub fn liquidate_liquidity_position(ctx: Context<LiquidateLiquidityPosition>, pool : Pubkey , _position_id : u64 , _fee_reciever : Pubkey) -> Result<()> {
        let address_list = &mut ctx.accounts.state.executors;
        let user_pubkey = ctx.accounts.user.key();
        require!(address_list.contains(&user_pubkey) , MyError::CallerUnauthorized);

        Ok(())
    }

    // Function to liquidate position
    pub fn liquidate_position(ctx: Context<LiquidatePosition>, token : Pubkey , account : Pubkey , side : bool , _fee_reciever : Pubkey  , pool : Pubkey ) -> Result<()> {
        // Logic to liquidate position
        let addresses: AccountInfo<'_> = ctx.accounts.authorized_account.clone();
        
        let user_pubkey = &ctx.accounts.user;
        let address_list = &ctx.accounts.state.executors;
        
        require!(address_list.contains(&user_pubkey.key()) , MyError::CallerUnauthorized);
        // let decrease_index_price = _choose_index_price(ctx ,token , size)?;
        let decrease_index_price=0;
        let _has_unrealized_profit:bool = false;
        let size = 100;
        if size ==0 ||  _has_unrealized_profit {
            // liquidate position 
           return Ok(())
        }
        let cpi_program = ctx.accounts.router_program.to_account_info();
        let cpi_accounts = PositionManagement{
            state : ctx.accounts.router_program.to_account_info(),
            authorized_account : addresses,
            user : ctx.accounts.router_program.to_account_info()
        };
        let receiver : Pubkey = Pubkey::default(); // change this to the program Pubkey that  you set in the end 
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        router::cpi::plugin_close_position_by_liquidator(cpi_ctx , pool ,  side , size , receiver);
        // external call to pool 


        Ok(())
    }


    pub fn _choose_index_price(ctx: Context<GovernanceAction> , token : Pubkey , side : bool ) -> Result<u64>{
        if side{

        }
    Ok(100)
    }

    pub fn _choose_funding_rate_growth(ctx: Context<GovernanceAction> , pool : Pubkey , side : bool) -> Result<u64>{
        if side{

        }
    Ok(100)
    }

    pub fn _require_liquidatable(ctx: Context<GovernanceAction> , token : Pubkey , side : bool , _account : Pubkey , _margin:u64 , _size : u64 , _enrty_price_x96 :u64, _decrease_price_x96:u64) -> Result<u64>{
        if side{

        }
    Ok(100)
    }

    pub fn _has_unrealized_profit(ctx: Context<GovernanceAction> , _entry_price: u64 , side : bool , _index_price :u64 ) -> Result<bool>{
        if side{
            return Ok(_index_price > _entry_price);
        }
    
    return Ok(_entry_price > _index_price);
    }


    // Additional functions as per your contract
}

// Contract state to hold the list of authorized addresses
#[account]
pub struct State {
    pub executors: Vec<Pubkey>,
    router : Pubkey,
    pool_factory : Pubkey,
    efc : Pubkey,
    price_feed : Pubkey,
    initilized: bool,
}

// #[account]
// pub struct Executors {
//     pub executor_address: Pubkey,
//     pub isActive: bool,
// }
#[derive(Accounts)]
pub struct GovernanceAction<'info> {
    // The account performing the action. Must be the governor.
    /// CHECK
    #[account(signer)]
    pub authorized_account: AccountInfo<'info>,
    // The governance state account.
    #[account(mut)]
    pub governance_state: Account<'info, State>,
    pub user: Signer<'info>,
}


// Context struct for Initialize function
#[derive(Accounts)]
pub struct Initialize<'info> {
     // Adjust space as needed
    /// CHECK
    #[account(signer)]
    pub authorized_account: AccountInfo<'info>,
    pub state: Account<'info, State>,
    pub user: Signer<'info>,

}

// Context struct for UpdatePriceFeed function
#[derive(Accounts)]
pub struct UpdatePriceFeed<'info> {
     // Adjust space as needed
    /// CHECK
    #[account(signer)]
    pub authorized_account: AccountInfo<'info>,
    pub state: Account<'info, State>,
    pub user: Signer<'info>,
}

// Context struct for UpdateExecutor function
#[derive(Accounts)]
pub struct UpdateExecutor<'info> {
    /// CHECK
    #[account(signer)]
    pub authorized_account: AccountInfo<'info>,
    pub state: Account<'info, State>,
    pub user: Signer<'info>,
}

// Context struct for LiquidateLiquidityPosition function
#[derive(Accounts)]
pub struct LiquidateLiquidityPosition<'info> {
        /// CHECK
    #[account(signer)]
    pub authorized_account: AccountInfo<'info>,
    pub state: Account<'info, State>,
    pub user: Signer<'info>,
}

// Context struct for LiquidatePosition function
#[derive(Accounts)]
pub struct LiquidatePosition<'info> {
        /// CHECK
        
        pub authorized_account: AccountInfo<'info>,

        pub state: Account<'info, State>,
        pub user: Signer<'info>,
        pub router_program: Program<'info, Router>,
}

#[derive(Accounts)]
pub struct ClosePositionByLiquidatorCpiContext<'info> {
    
    pub router_program: Program<'info, Router>,
}


// Custom errors
#[error_code]
pub enum MyError {
    #[msg("Unauthorized access")]
    CallerUnauthorized,
    #[msg("Invalid operation")]
    InvalidOperation,
    #[msg("Program Already initilized")]
    AlreadyInitlized,
    // Add other custom errors
}
