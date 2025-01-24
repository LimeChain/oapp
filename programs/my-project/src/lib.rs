use anchor_lang::prelude::*;
use endpoint::instructions::oapp::SendParams;
use endpoint::MessagingReceipt;
use oapp::endpoint::instructions::RegisterOAppParams;

mod instructions;
mod state;

use instructions::*;
use state::GlobalConfig;

declare_id!("GG9GMa3Y7ow2j9jRgbTusBHc57VUh55G4wfbVskhjkbh");

#[program]
pub mod my_project {
    use super::*;

    pub fn initialize(mut ctx: Context<Initialize>, params: InitParams) -> Result<()> {
        Initialize::apply(&mut ctx, params)
    }

    pub fn send(ctx: Context<Send>, params: SendParams) -> Result<MessagingReceipt> {
        let receipt = send_message(ctx, params)?;
        Ok(receipt)
    }
}

#[derive(Accounts)]
#[instruction(params: InitParams)]
pub struct Initialize<'info> {
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

impl Initialize<'_> {
    pub fn apply(ctx: &mut Context<Initialize>, params: InitParams) -> Result<()> {
        let global_config = &mut ctx.accounts.global_config;
        global_config.portfolio = params.portfolio;
        global_config.mainnet_rfq = params.mainnet_rfq;
        global_config.default_chain_id = 1;
        global_config.out_nonce = 0;

        msg!("Global config initialized");
        msg!("Registering OAPP");
        oapp::endpoint_cpi::register_oapp(
            oapp::endpoint::ID,
            *ctx.program_id,
            ctx.remaining_accounts,
            &[b"sol_vault".as_ref(), &[ctx.bumps.sol_vault]],
            RegisterOAppParams {
                delegate: ctx.accounts.authority.key(),
            },
        )?;

        Ok(())
    }
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct InitParams {
    pub portfolio: Pubkey,
    pub mainnet_rfq: Pubkey,
}
