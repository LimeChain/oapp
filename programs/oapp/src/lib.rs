use anchor_lang::prelude::*;
use endpoint::instructions::oapp::SendParams;
use endpoint::{MessagingReceipt, MessagingFee};

mod instructions;
mod state;
mod msg_codec;

use instructions::*;

declare_id!("GG9GMa3Y7ow2j9jRgbTusBHc57VUh55G4wfbVskhjkbh");
const REMOTE_SEED: &[u8] = b"Remote";

#[program]
pub mod my_project {
    use super::*;

    pub fn initialize(mut ctx: Context<RegisterOApp>, params: RegisterOAppParams) -> Result<()> {
        RegisterOApp::apply(&mut ctx, params)
    }

    pub fn set_remote(mut ctx: Context<SetRemote>, params: SetRemoteParams) -> Result<()> {
        SetRemote::apply(&mut ctx, &params)
    }

    pub fn send(ctx: Context<Send>, params: SendParams) -> Result<MessagingReceipt> {
        let receipt = send_message(ctx, params)?;
        Ok(receipt)
    }

    pub fn quote(ctx: Context<Quote>, params: QuoteParams) -> Result<MessagingFee> {
        Quote::apply(&ctx, &params)
    }
}
