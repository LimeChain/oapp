use anchor_lang::prelude::*;
use endpoint::{MessagingReceipt, ID as ENDPOINT_ID, ENDPOINT_SEED};
use oapp::endpoint_cpi::send;
use crate::instructions::{quote, QuoteParams};
use crate::xfer::{pack_xfer_message, XFER};

#[derive(Accounts)]
#[instruction(params: SendParams)]
pub struct Send<'info> {
    #[account(signer)]
    pub sender: AccountInfo<'info>,
    pub endpoint_program: AccountInfo<'info>,
}

pub fn send_message(ctx: Context<Send>, params: SendParams) -> Result<MessagingReceipt> {
    // let quote_params = QuoteParams {
    //     dst_eid: 0,
    //     receiver: params.receiver,
    //     msg_type: 0,
    //     options: params.options.clone(),
    //     pay_in_lz_token: false,
    // };
    let par = endpoint::instructions::oapp::SendParams{
        dst_eid: params.dst_eid,
        receiver: params.receiver,
        message: pack_xfer_message(&params.message)?,
        options: params.options,
        native_fee: 500,// quote()?.native_fee,
        lz_token_fee: params.lz_token_fee,
    };
    let receipt = send(
        ENDPOINT_ID,
        ctx.accounts.sender.key(),
        ctx.remaining_accounts,
        &[ENDPOINT_SEED],
        par,
    )?;

    Ok(receipt)
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct SendParams {
    pub dst_eid: u32,
    pub receiver: [u8; 32],
    pub message: XFER,
    pub options: Vec<u8>,
    pub native_fee: u64,
    pub lz_token_fee: u64,
}
