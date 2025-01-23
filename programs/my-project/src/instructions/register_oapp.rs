use anchor_lang::{emit_cpi, event_cpi, Accounts, AnchorDeserialize, AnchorSerialize, Key};
use anchor_lang::prelude::*;
use cpi_helper::CpiContext;
use endpoint::OAPP_SEED;
use endpoint::events::OAppRegisteredEvent;
use endpoint::state::OAppRegistry;
use solana_program::pubkey::Pubkey;

pub fn register_oapp() {

}
#[event_cpi]
#[derive(CpiContext, Accounts)]
#[instruction(params: RegisterOAppParams)]
pub struct RegisterOApp<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    /// The PDA of the OApp
    pub oapp: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = 8 + OAppRegistry::INIT_SPACE,
        seeds = [OAPP_SEED, oapp.key.as_ref()],
        bump
    )]
    pub oapp_registry: Account<'info, OAppRegistry>,
    pub system_program: Program<'info, System>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct RegisterOAppParams {
    pub delegate: Pubkey,
}
