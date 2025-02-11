use crate::consts::{BRIDGE_SEED, SOL_VAULT_SEED};
use crate::state::{Bridge, GlobalConfig};
use anchor_lang::prelude::*;
use anchor_lang::{Accounts, AnchorDeserialize, AnchorSerialize, Key};
use oapp::endpoint::{instructions::RegisterOAppParams, ID as ENDPOINT_ID};
use solana_program::pubkey::Pubkey;

#[derive(Accounts)]
#[instruction(params: RegisterOAppParams)]
pub struct InitBridge<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = Bridge::LEN,
        seeds = [BRIDGE_SEED],
        bump
    )]
    pub bridge: Account<'info, Bridge>,

    #[account(mut, seeds = [SOL_VAULT_SEED], bump)]
    pub sol_vault: SystemAccount<'info>,
    pub system_program: AccountInfo<'info>,
}

pub fn init_bridge(ctx: Context<InitBridge>, params: InitBridgeParams) -> Result<()> {
    let bridge = &mut ctx.accounts.bridge;

    msg!("Bridge: {}", bridge.key());
    // init bridge
    bridge.endpoint_program = params.endpoint_program.key();
    bridge.admin = ctx.accounts.authority.key();
    bridge.bump = ctx.bumps.bridge;

    // init global config
    bridge.global_config.portfolio = params.portfolio;
    bridge.global_config.mainnet_rfq = params.mainnet_rfq;
    bridge.global_config.default_chain_id = params.default_chain_id;

    msg!("Global config initialized");
    msg!("Registering Portfolio Bridge with id {}", bridge.key());

    let seeds = &[BRIDGE_SEED, &[ctx.bumps.bridge]];
    let params = RegisterOAppParams {
        delegate: ctx.accounts.authority.key(),
    };
    oapp::endpoint_cpi::register_oapp(
        ENDPOINT_ID,
        bridge.key(),
        ctx.remaining_accounts,
        seeds,
        params,
    )?;

    Ok(())
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct InitBridgeParams {
    pub portfolio: Pubkey,
    pub mainnet_rfq: Pubkey,
    pub default_chain_id: u32,
    pub endpoint_program: Pubkey,
}
