use crate::{
    consts::{GAS_OPTIONS, PORTFOLIO_SEED},
    cpi_utils::{create_instruction_data, EndpointQuoteParams, MessagingFee},
    errors::PortfolioError,
    state::Portfolio,
    xfer::XFER,
    *,
};

use anchor_lang::solana_program::{
    instruction::Instruction,
    program::{get_return_data, invoke},
};

#[derive(Accounts)]
#[instruction(params: QuoteParams)]
pub struct Quote<'info> {
    #[account(seeds = [PORTFOLIO_SEED], bump = portfolio.bump)]
    pub portfolio: Account<'info, Portfolio>,
    // #[account(seeds = [ENDPOINT_SEED], bump = endpoint.bump, seeds::program = ENDPOINT_ID)]
    // pub endpoint: Account<'info, EndpointSettings>,
    /// CHECK: the endpoint program
    pub endpoint_program: AccountInfo<'info>,
}

pub fn quote(ctx: &Context<Quote>, params: &QuoteParams) -> Result<MessagingFee> {
    let message = params.message.pack_xfer_message()?;

    // calling endpoint cpi
    let quote_params = EndpointQuoteParams {
        sender: ctx.accounts.portfolio.key(),
        dst_eid: params.dst_eid,
        receiver: params.receiver,
        message,
        pay_in_lz_token: false,
        options: GAS_OPTIONS.to_vec(),
    };
    let cpi_data = create_instruction_data(&quote_params, "quote");

    let accounts_metas: Vec<AccountMeta> = ctx
        .remaining_accounts
        .iter()
        .skip(1)
        .map(|account| AccountMeta {
            pubkey: *account.key,
            is_signer: account.is_signer,
            is_writable: account.is_writable,
        })
        .collect();

    // Invoke CPI
    invoke(
        &Instruction {
            program_id: ctx.accounts.endpoint_program.key(),
            accounts: accounts_metas,
            data: cpi_data,
        },
        ctx.remaining_accounts,
    )?;

    let return_data = get_return_data().ok_or(PortfolioError::LzQuoteError)?;

    // Deserialize the return data into MessagingReceipt
    MessagingFee::try_from_slice(&return_data.1).map_err(|_| error!(PortfolioError::LzQuoteError))

    //oapp::endpoint_cpi::quote(ENDPOINT_ID, ctx.remaining_accounts, quote_params)
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct QuoteParams {
    pub dst_eid: u32,
    pub receiver: [u8; 32],
    pub message: XFER,
}
