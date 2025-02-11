use anchor_lang::prelude::*;

use crate::consts::{ADMIN_SEED, PORTFOLIO_SEED};
use crate::errors::PortfolioError;
use crate::state::Portfolio;

pub fn set_allow_deposit(ctx: Context<WriteConfig>, allow_deposit: bool) -> Result<()> {
    let admin = &ctx.accounts.admin;

    require!(admin.owner == ctx.program_id, PortfolioError::Unauthorized);

    ctx.accounts.portfolio.global_config.allow_deposit = allow_deposit;

    Ok(())
}

pub fn set_paused(ctx: Context<WriteConfig>, paused: bool) -> Result<()> {
    let admin = &ctx.accounts.admin;

    require!(admin.owner == ctx.program_id, PortfolioError::Unauthorized);

    ctx.accounts.portfolio.global_config.program_paused = paused;

    Ok(())
}

pub fn set_native_deposits_restricted(
    ctx: Context<WriteConfig>,
    native_deposits_restricted: bool,
) -> Result<()> {
    let admin = &ctx.accounts.admin;

    require!(admin.owner == ctx.program_id, PortfolioError::Unauthorized);

    ctx.accounts
        .portfolio
        .global_config
        .native_deposits_restricted = native_deposits_restricted;

    Ok(())
}

#[derive(Accounts)]
pub struct WriteConfig<'info> {
    pub authority: Signer<'info>,
    #[account(
        mut,
        seeds = [PORTFOLIO_SEED],
        bump
    )]
    pub portfolio: Account<'info, Portfolio>,

    /// CHECK: Used to check if authority is admin
    #[account(
        seeds = [ADMIN_SEED, authority.key().as_ref()],
        bump
    )]
    pub admin: AccountInfo<'info>,
}
