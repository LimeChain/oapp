use crate::*;
use oapp::endpoint::{
    instructions::QuoteParams as EndpointQuoteParams, state::EndpointSettings, ENDPOINT_SEED,
    ID as ENDPOINT_ID,
};

#[derive(Accounts)]
#[instruction(params: QuoteParams)]
pub struct Quote<'info> {
    pub sender: Signer<'info>,
    #[account(seeds = [ENDPOINT_SEED], bump = endpoint.bump, seeds::program = ENDPOINT_ID)]
    pub endpoint: Account<'info, EndpointSettings>,
}
impl<'info> Quote<'info> {
    pub fn apply(ctx: &Context<Quote>, params: &QuoteParams) -> Result<MessagingFee> {
        let message = msg_codec::encode(params.msg_type, ctx.accounts.endpoint.eid);

        // calling endpoint cpi
        let quote_params = EndpointQuoteParams {
            sender: ctx.accounts.sender.key(),
            dst_eid: params.dst_eid,
            receiver: params.receiver,
            message,
            pay_in_lz_token: params.pay_in_lz_token,
            options: params.options.clone(),
        };
        oapp::endpoint_cpi::quote(ENDPOINT_ID, ctx.remaining_accounts, quote_params)
    }
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct QuoteParams {
    pub dst_eid: u32,
    pub receiver: [u8; 32],
    pub msg_type: u8,
    pub options: Vec<u8>,
    pub pay_in_lz_token: bool,
}
