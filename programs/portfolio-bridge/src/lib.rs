use anchor_lang::prelude::*;
use anchor_lang::{declare_id, program};
use oapp::endpoint::{MessagingFee, MessagingReceipt};

mod consts;
mod errors;
mod events;
mod instructions;
mod state;
mod xfer;
use instructions::*;

declare_id!("DD12vMyLdwszDCAzLhsUPwBmzJXv611dUCPhqwpZQYG4");

#[program]
pub mod portfolio_bridge {
    use super::*;

    // pub fn init_bridge(ctx: Context<InitBridge>, params: InitBridgeParams) -> Result<()> {
    //     instructions::init_bridge(ctx, params)
    // }

    // pub fn set_remote(mut ctx: Context<SetRemote>, params: SetRemoteParams) -> Result<()> {
    //     instructions::set_remote(&mut ctx, &params)
    // }

    // pub fn send(ctx: Context<Send>, params: SendParams) -> Result<MessagingReceipt> {
    //     let receipt = instructions::send(ctx, params)?;
    //     Ok(receipt)
    // }

    // pub fn quote(mut ctx: Context<Quote>, params: QuoteParams) -> Result<()> {
    //     instructions::quote(&mut ctx, &params)
    // }
}
