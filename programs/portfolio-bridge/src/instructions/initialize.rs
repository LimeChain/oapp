use crate::consts::{ADMIN_SEED, PORTFOLIO_SEED, SOL_VAULT_SEED};
use crate::state::{Admin, GlobalConfig, Portfolio, TokenList};
use anchor_lang::prelude::*;
use anchor_lang::{Accounts, AnchorDeserialize, AnchorSerialize, Key};
use oapp::endpoint::{instructions::RegisterOAppParams, ID as ENDPOINT_ID};
use solana_program::pubkey::Pubkey;

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

    msg!("Global config initialized");

    let seeds = &[PORTFOLIO_SEED, &[ctx.bumps.portfolio]];
    let params = RegisterOAppParams {
        delegate: ctx.accounts.authority.key(),
    };
    oapp::endpoint_cpi::register_oapp(
        ENDPOINT_ID,
        portfolio.key(),
        ctx.remaining_accounts,
        seeds,
        params,
    )?;

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
}
