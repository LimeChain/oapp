use anchor_lang::{Accounts, AnchorDeserialize, AnchorSerialize, Key};
use anchor_lang::prelude::*;
use endpoint::ID as ENDPOINT_ID;
use solana_program::pubkey::Pubkey;
use crate::state::GlobalConfig;

#[derive(Accounts)]
#[instruction(params: RegisterOAppParams)]
pub struct RegisterOApp<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = GlobalConfig::LEN,
        seeds = [b"global_config"],
        bump
    )]
    pub global_config: Account<'info, GlobalConfig>,

    /// The vault account that will hold the deposited SOL.
    #[account(mut, seeds = [b"sol_vault"], bump)]
    pub sol_vault: SystemAccount<'info>,

    pub system_program: AccountInfo<'info>,
}

impl RegisterOApp<'_> {
    pub fn apply(ctx: &mut Context<RegisterOApp>, params: RegisterOAppParams) -> Result<()> {
        let global_config = &mut ctx.accounts.global_config;
        global_config.portfolio = params.portfolio;
        global_config.mainnet_rfq = params.mainnet_rfq;
        global_config.default_chain_id = 1;
        global_config.out_nonce = 0;

        let sol_vault = &mut ctx.accounts.sol_vault;

        msg!("Global config initialized");
        msg!("Registering OAPP");
        oapp::endpoint_cpi::register_oapp(
            ENDPOINT_ID,
            sol_vault.key(),
            ctx.remaining_accounts,
            &[b"sol_vault".as_ref(), &[ctx.bumps.sol_vault]],
            endpoint::instructions::RegisterOAppParams {
                delegate: ctx.accounts.authority.key(),
            },
        )?;

        Ok(())
    }
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct RegisterOAppParams {
    pub portfolio: Pubkey,
    pub mainnet_rfq: Pubkey,
}
