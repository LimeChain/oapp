use crate::{
    consts::{GAS_OPTIONS, PORTFOLIO_SEED},
    state::Portfolio,
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
    #[account(seeds = [PORTFOLIO_SEED], bump = portfolio.bump)]
    pub portfolio: Account<'info, Portfolio>,
    #[account(seeds = [ENDPOINT_SEED], bump = endpoint.bump, seeds::program = ENDPOINT_ID)]
    pub endpoint: Account<'info, EndpointSettings>,
}

pub fn quote(ctx: &Context<Quote>, params: &QuoteParams) -> Result<()> {
    msg!("Endpoint: {}", ctx.accounts.endpoint.key());
    // let message = params.message.pack_xfer_message()?;

    // let options = decode(GAS_OPTIONS).unwrap();

    // // calling endpoint cpi
    // let quote_params = EndpointQuoteParams {
    //     sender: ctx.accounts.bridge.key(),
    //     dst_eid: params.dst_eid,
    //     receiver: params.receiver,
    //     message,
    //     pay_in_lz_token: false,
    //     options,
    // };
    Ok(())
    // oapp::endpoint_cpi::quote(ENDPOINT_ID, ctx.remaining_accounts, quote_params)
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct QuoteParams {
    pub dst_eid: u32,
    pub receiver: [u8; 32],
    pub message: XFER,
}
