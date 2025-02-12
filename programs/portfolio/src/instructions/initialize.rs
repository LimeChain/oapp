use crate::consts::{ADMIN_SEED, PORTFOLIO_SEED, SOL_VAULT_SEED};
use crate::cpi_utils::{create_instruction_data, RegisterOAppParams};
use crate::state::{Admin, GlobalConfig, Portfolio, TokenList};
use anchor_lang::solana_program::instruction::Instruction;
use anchor_lang::solana_program::program::invoke_signed;
use anchor_lang::{
    prelude::*, solana_program::pubkey::Pubkey, Accounts, AnchorDeserialize, AnchorSerialize, Key,
};

// #[derive(Accounts)]
// pub struct InitBridge<'info> {
//     #[account(mut)]
//     pub authority: Signer<'info>,

//     #[account(
//         init,
//         payer = authority,
//         space = Portfolio::LEN,
//         seeds = [BRIDGE_SEED],
//         bump
//     )]
//     pub portfolio: Account<'info, Portfolio>,

//     #[account(mut, seeds = [SOL_VAULT_SEED], bump)]
//     pub sol_vault: SystemAccount<'info>,
//     pub system_program: AccountInfo<'info>,
// }

pub fn initialize(ctx: Context<Initialize>, params: InitializeParams) -> Result<()> {
    let portfolio = &mut ctx.accounts.portfolio;

    // init portfolio
    portfolio.endpoint_program = params.endpoint_program.key();
    portfolio.admin = ctx.accounts.authority.key();
    portfolio.bump = ctx.bumps.portfolio;

    // init global config
    portfolio.global_config.default_chain_id = params.default_chain_id;
    portfolio.global_config.allow_deposit = true;
    portfolio.global_config.program_paused = false;
    portfolio.global_config.native_deposits_restricted = false;
    portfolio.global_config.src_chain_id = params.src_chain_id;

    // prepare CPI
    let register_params = RegisterOAppParams {
        delegate: ctx.accounts.authority.key(),
    };

    let seeds: &[&[&[u8]]] = &[&[PORTFOLIO_SEED, &[ctx.accounts.portfolio.bump]]];
    let cpi_data = create_instruction_data(&register_params, "register_oapp");

    let portfolio_key = ctx.accounts.portfolio.key();
    let accounts_metas: Vec<AccountMeta> = ctx
        .remaining_accounts
        .iter()
        .skip(1)
        .map(|account| AccountMeta {
            pubkey: *account.key,
            is_signer: account.key() == portfolio_key || account.is_signer,
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
    // oapp::endpoint_cpi::register_oapp(
    //     ENDPOINT_ID,
    //     bridge.key(),
    //     ctx.remaining_accounts,
    //     seeds,
    //     register_params,
    // )?;

    Ok(())
}

// pub fn initialize(ctx: Context<Initialize>, src_chain_id: u16) -> Result<()> {
//     // Initialize global config
//     let global_config = &mut ctx.accounts.global_config;
//     global_config.allow_deposit = true;
//     global_config.program_paused = false;
//     global_config.native_deposits_restricted = false;
//     global_config.src_chain_id = src_chain_id;

//     Ok(())
// }

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct InitializeParams {
    pub src_chain_id: u16,
    pub mainnet_rfq: Pubkey,
    pub default_chain_id: u32,
    pub endpoint_program: Pubkey,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = Portfolio::LEN,
        seeds = [PORTFOLIO_SEED],
        bump
    )]
    pub portfolio: Account<'info, Portfolio>,
    #[account(
        init,
        payer = authority,
        space = TokenList::LEN,
        seeds = [b"token_list", b"0".as_ref()],
        bump
    )]
    pub token_list: Account<'info, TokenList>,

    /// CHECK: SPL ATA authority account to sing transfers.
    #[account(
        init,
        payer = authority,
        seeds = [b"spl_vault"],
        bump,
        space = 0
    )]
    pub spl_vault: AccountInfo<'info>,

    #[account(
        init,
        payer = authority,
        space = Admin::LEN,
        seeds = [ADMIN_SEED, authority.key().as_ref()],
        bump
    )]
    pub admin: Account<'info, Admin>,

    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    /// CHECK: endpoint program,
    pub endpoint_program: AccountInfo<'info>,
}
