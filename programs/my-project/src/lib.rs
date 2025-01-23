use anchor_lang::prelude::*;
use endpoint::MessagingReceipt;
use endpoint::instructions::oapp::SendParams;
use oapp::endpoint::instructions::RegisterOAppParams;

mod instructions;
mod state;

use instructions::*;
use state::{Admin, GlobalConfig, TokenList};

declare_id!("5midd7yfem3sFitDwsQVCUEwyUrcEkZdHjevVJqt1Gye");

#[program]
pub mod my_project {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // Initialize global config
        let global_config = &mut ctx.accounts.global_config;
        global_config.allow_deposit = true;
        global_config.program_paused = false;
        global_config.native_deposits_restricted = false;

        Ok(())
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

        oapp::endpoint_cpi::register_oapp(
            oapp::endpoint::ID,
            *ctx.program_id,
            &[],
            &[b"sol_vault".as_ref(), &[ctx.bumps.sol_vault]],
            RegisterOAppParams {
                delegate: params.portfolio,
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
