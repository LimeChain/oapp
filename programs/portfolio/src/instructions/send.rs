use crate::{
    consts::{GAS_OPTIONS, PORTFOLIO_SEED, REMOTE_SEED},
    cpi_utils::{create_instruction_data, EndpointSendParams},
    state::{Portfolio, Remote},
    xfer::XFER,
};

use anchor_lang::{
    prelude::*,
    solana_program::{instruction::Instruction, program::invoke_signed},
};

#[derive(Accounts)]
#[instruction(params: SendParams)]
pub struct Send<'info> {
    #[account(
        seeds = [
            REMOTE_SEED,
            &params.dst_eid.to_be_bytes()
        ],
        bump = remote.bump
    )]
    pub remote: Account<'info, Remote>,
    #[account(seeds = [PORTFOLIO_SEED], bump = portfolio.bump)]
    pub portfolio: Account<'info, Portfolio>,
    /// CHECK: the endpoint program
    pub endpoint_program: AccountInfo<'info>,
}

pub fn send<'info>(ctx: Context<'_, '_, '_, 'info, Send<'info>>, params: SendParams) -> Result<()> {
    //TODO: Calculate quote

    let message = params.message.pack_xfer_message()?;

    let send_params = EndpointSendParams {
        dst_eid: params.dst_eid,
        receiver: ctx.accounts.remote.address,
        message,
        options: GAS_OPTIONS.to_vec(),
        native_fee: 0x11b24f,
        lz_token_fee: 0,
    };
    let seeds: &[&[&[u8]]] = &[&[PORTFOLIO_SEED, &[ctx.accounts.portfolio.bump]]];

    let cpi_data = create_instruction_data(&send_params, "send");

    let bridge_key = ctx.accounts.portfolio.key();
    let accounts_metas: Vec<AccountMeta> = ctx
        .remaining_accounts
        .iter()
        .skip(1)
        .map(|account| AccountMeta {
            pubkey: *account.key,
            is_signer: account.key() == bridge_key || account.is_signer,
            is_writable: account.is_writable,
        })
        .collect();

    // Invoke CPI
    invoke_signed(
        &Instruction {
            program_id: ctx.accounts.endpoint_program.key(),
            accounts: accounts_metas,
            data: cpi_data,
        },
        ctx.remaining_accounts,
        seeds,
    )?;

    Ok(())
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct SendParams {
    pub dst_eid: u32,
    pub message: XFER,
}
