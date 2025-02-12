use anchor_lang::prelude::*;

use crate::consts::ADMIN_SEED;
use crate::errors::PortfolioError;
use crate::events::{RoleGrantedEvent, RoleRevokedEvent};
use crate::state::Admin;

// Instruction to add a new admin
pub fn add_admin(ctx: Context<AddAdmin>, account: Pubkey) -> Result<()> {
    let admin = &ctx.accounts.admin;

    //CHECK: Verify that user is an admin by checking their PDA.
    require!(admin.owner == ctx.program_id, PortfolioError::Unauthorized);

    emit!(RoleGrantedEvent {
        role: [0; 32],
        admin: account,
    });

    Ok(())
}

// Instruction to remove an admin
pub fn remove_admin(ctx: Context<RemoveAdmin>, account: Pubkey) -> Result<()> {
    let admin = &ctx.accounts.admin;

    require!(admin.owner == ctx.program_id, PortfolioError::Unauthorized);

    emit!(RoleRevokedEvent {
        role: [0; 32],
        admin: account,
    });

    Ok(())
}

// Context for creating a new admin
#[derive(Accounts)]
#[instruction(account: Pubkey)]
pub struct AddAdmin<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    /// CHECK: Verify that user is an admin by checking their PDA.
    #[account(
        seeds = [ADMIN_SEED, authority.key().as_ref()],
        bump
    )]
    pub admin: AccountInfo<'info>,

    #[account(
        init,
        payer = authority,
        space = Admin::LEN,
        seeds = [ADMIN_SEED, account.as_ref()],
        bump
    )]
    pub new_admin: Account<'info, Admin>,

    pub system_program: Program<'info, System>,
}

// Context for removing an existing admin
#[derive(Accounts)]
#[instruction(account: Pubkey)]
pub struct RemoveAdmin<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    /// CHECK: Verify that user is an admin by checking their PDA.
    #[account(
        seeds = [ADMIN_SEED, authority.key().as_ref()],
        bump
    )]
    pub admin: AccountInfo<'info>,

    #[account(mut)]
    pub receiver: SystemAccount<'info>,

    #[account(
        mut,
        close = receiver, // Refund lamports to the receiver
        seeds = [ADMIN_SEED, account.as_ref()],
        bump
    )]
    pub admin_to_remove: Account<'info, Admin>,

    pub system_program: Program<'info, System>,
}
