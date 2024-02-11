use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct TradingFeeState {
    pub trading_fee_rate: u32,
    pub referral_return_fee_rate: u32,
    pub referral_parent_return_fee_rate: u32,
    pub referral_token: Pubkey,
    pub referral_parent_token: Pubkey,
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

