use crate::{
    consts::{BRIDGE_SEED, GAS_OPTIONS},
    state::Bridge,
    xfer::XFER,
    *,
};

use hex::decode;
use oapp::endpoint::{
    instructions::QuoteParams as EndpointQuoteParams, state::EndpointSettings, ENDPOINT_SEED,
    ID as ENDPOINT_ID,
};

#[derive(Accounts)]
#[instruction(params: QuoteParams)]
pub struct Quote<'info> {
    #[account(seeds = [BRIDGE_SEED], bump = bridge.bump)]
    pub bridge: Account<'info, Bridge>,
    #[account(seeds = [ENDPOINT_SEED], bump = endpoint.bump, seeds::program = ENDPOINT_ID)]
    pub endpoint: Account<'info, EndpointSettings>,
}

pub fn quote(ctx: &Context<Quote>, params: &QuoteParams) -> Result<MessagingFee> {
    let message = params.message.pack_xfer_message()?;

    let options = decode(GAS_OPTIONS).unwrap();

    // calling endpoint cpi
    let quote_params = EndpointQuoteParams {
        sender: ctx.accounts.bridge.key(),
        dst_eid: params.dst_eid,
        receiver: params.receiver,
        message,
        pay_in_lz_token: false,
        options,
    };

    oapp::endpoint_cpi::quote(ENDPOINT_ID, ctx.remaining_accounts, quote_params)
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct QuoteParams {
    pub dst_eid: u32,
    pub receiver: [u8; 32],
    pub message: XFER,
}
