use crate::consts::{BRIDGE_SEED, GAS_OPTIONS, REMOTE_SEED};
use crate::state::{Bridge, Remote};
use crate::xfer::XFER;

use anchor_lang::prelude::*;
use hex::decode;
use oapp::endpoint::MessagingReceipt;
use oapp::endpoint::{
    instructions::{QuoteParams as EndpointQuoteParams, SendParams as EndpointSendParams},
    state::EndpointSettings,
    ENDPOINT_SEED, ID as ENDPOINT_ID,
};

#[derive(Accounts)]
#[instruction(params: SendParams)]
pub struct Send<'info> {
    #[account(
        seeds = [
            REMOTE_SEED,
            &bridge.key().to_bytes(),
            &params.dst_eid.to_be_bytes()
        ],
        bump = remote.bump
    )]
    pub remote: Account<'info, Remote>,
    #[account(seeds = [BRIDGE_SEED], bump = bridge.bump)]
    pub bridge: Account<'info, Bridge>,
    #[account(seeds = [ENDPOINT_SEED], bump = endpoint_program.bump, seeds::program = ENDPOINT_ID)]
    pub endpoint_program: Account<'info, EndpointSettings>,
}

pub fn send_message(ctx: Context<Send>, params: SendParams) -> Result<MessagingReceipt> {
    let options = decode(GAS_OPTIONS).unwrap(); //we provide const value

    //TODO: Calculate quote

    let message = params.message.pack_xfer_message()?;
    let send_params = EndpointSendParams {
        dst_eid: params.dst_eid,
        receiver: ctx.accounts.remote.address,
        message,
        options,
        native_fee: 0x11b24f,
        lz_token_fee: 0,
    };
    let seeds: &[&[u8]] = &[BRIDGE_SEED, &[ctx.accounts.bridge.bump]];

    let receipt = oapp::endpoint_cpi::send(
        ENDPOINT_ID,
        ctx.accounts.bridge.key(),
        ctx.remaining_accounts,
        seeds,
        send_params,
    )?;

    Ok(receipt)
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct SendParams {
    pub dst_eid: u32,
    pub message: XFER,
}
