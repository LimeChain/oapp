use anchor_lang::prelude::*;
use endpoint::instructions::oapp::SendParams;
use endpoint::{MessagingReceipt, ID as ENDPOINT_ID, ENDPOINT_SEED};
use oapp::endpoint_cpi::send;

pub fn send_message(ctx: Context<Send>, params: SendParams) -> Result<MessagingReceipt> {
    let receipt = send(
        ENDPOINT_ID,
        ctx.accounts.sender.key(),
        ctx.remaining_accounts,
        &[ENDPOINT_SEED],
        params,
    )?;

    Ok(receipt)
}

#[derive(Accounts)]
#[instruction(params: SendParams)]
pub struct Send<'info> {
    #[account(signer)]
    pub sender: AccountInfo<'info>,
    pub endpoint_program: AccountInfo<'info>,
}
