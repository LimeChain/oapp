use anchor_lang::prelude::*;
use endpoint::{MessagingReceipt, MessagingFee};
use anchor_lang::{declare_id, program};

mod instructions;
mod state;
mod xfer;

use instructions::*;


declare_id!("9Fmenbf7Qti4sG3hQWwifpAvGArtqtK9N96jdN19MX3u");

#[program]
pub mod portfolio_bridge {
    use super::*;

    pub fn initialize(ctx: Context<RegisterOApp>, params: RegisterOAppParams) -> Result<()> {
        register_oapp(ctx, params)
    }

    pub fn set_remote(ctx: Context<SetRemote>, params: SetRemoteParams) -> Result<()> {
        instructions::set_remote(ctx, params)
    }

    pub fn send(ctx: Context<Send>, params: SendParams) -> Result<MessagingReceipt> {
        let receipt = send_message(ctx, params)?;
        Ok(receipt)
    }

    pub fn quote(ctx: Context<Quote>, params: QuoteParams) -> Result<MessagingFee> {
        Ok(instructions::quote(ctx, params)?)
    }
}