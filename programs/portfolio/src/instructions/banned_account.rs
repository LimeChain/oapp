use crate::consts::{ADMIN_SEED, BANNED_ACCOUNT_SEED};
use crate::errors::PortfolioError;
use crate::events::BanStatusChangedEvent;
use crate::state::{BanReason, BannedAccount};
use anchor_lang::prelude::*;

pub fn ban_account(ctx: Context<BanAccount>, account: Pubkey, reason: BanReason) -> Result<()> {
    let admin = &ctx.accounts.admin;

    // CHECK: Verify that user is an admin by checking their PDA.
    require!(admin.owner == ctx.program_id, PortfolioError::Unauthorized);

    let banned_account = &mut ctx.accounts.banned_account;
    banned_account.reason = reason;

    emit!(BanStatusChangedEvent {
        account,
        reason,
        banned: true,
    });

    Ok(())
}

pub fn unban_account(ctx: Context<UnbanAccount>, account: Pubkey) -> Result<()> {
    let admin = &ctx.accounts.admin;

    // CHECK: Verify that user is an admin by checking their PDA.
    require!(admin.owner == ctx.program_id, PortfolioError::Unauthorized);

    emit!(BanStatusChangedEvent {
        account,
        reason: BanReason::NotBanned,
        banned: false,
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(account: Pubkey, reason: BanReason)]
pub struct BanAccount<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    /// CHECK: Validate that the authority is an admin by checking the admin PDA.
    #[account(
        seeds = [ADMIN_SEED, authority.key().as_ref()],
        bump
    )]
    pub admin: AccountInfo<'info>,

    /// Create or update the BannedAccount PDA derived from the `account` pubkey provided.
    /// The seeds ensure uniqueness per banned user pubkey.
    /// If `reason` is `NotBanned`, you could allow closing or just updating.
    #[account(
        init,
        payer = authority,
        space = BannedAccount::LEN,
        seeds = [BANNED_ACCOUNT_SEED, account.as_ref()],
        bump
    )]
    pub banned_account: Account<'info, BannedAccount>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(account: Pubkey)]
pub struct UnbanAccount<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    /// CHECK: Validate that the authority is an admin by checking the admin PDA.
    #[account(
        seeds = [ADMIN_SEED, authority.key().as_ref()],
        bump
    )]
    pub admin: AccountInfo<'info>,

    // The banned account for this user. We use `close` to return lamports to `receiver`.
    #[account(
        mut,
        close = receiver,
        seeds = [BANNED_ACCOUNT_SEED, account.as_ref()],
        bump
    )]
    pub banned_account: Account<'info, BannedAccount>,

    #[account(mut)]
    pub receiver: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}
