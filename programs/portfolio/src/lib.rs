use anchor_lang::prelude::*;
use anchor_lang::{declare_id, program};
use cpi_utils::MessagingFee;
mod consts;
mod cpi_utils;
mod errors;
mod events;
mod instructions;
mod state;
mod xfer;
use instructions::*;

declare_id!("CUmdZmnaTZh8g7oFPbQxh3GHPtSVz9Wyw1RXxmUeUxeQ");

#[program]
pub mod portfolio {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, params: InitializeParams) -> Result<()> {
        instructions::initialize(ctx, params)
    }

    pub fn set_remote(mut ctx: Context<SetRemote>, params: SetRemoteParams) -> Result<()> {
        instructions::set_remote(&mut ctx, &params)
    }

    pub fn send<'info>(
        ctx: Context<'_, '_, '_, 'info, Send<'info>>,
        params: SendParams,
    ) -> Result<()> {
        instructions::send(ctx, params)
    }

    pub fn quote(mut ctx: Context<Quote>, params: QuoteParams) -> Result<MessagingFee> {
        instructions::quote(&mut ctx, &params)
    }
}
