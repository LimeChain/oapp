use std::collections::HashSet;

use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::consts::{MAX_TOKENS, NATIVE_SYMBOL};
use crate::errors::{PortfolioError, TokenListError};
use crate::events::ParameterUpdatedEvent;
use crate::state::{Admin, GlobalConfig, TokenDetails, TokenList};

pub fn add_token<'info>(
    ctx: Context<'_, '_, 'info, 'info, AddToken<'info>>,
    symbol: [u8; 32],
    token_address: Option<Pubkey>,
    decimals: u8,
) -> Result<()> {
    // Validate that the signer is an admin.
    let admin = &ctx.accounts.admin;
    require_keys_eq!(*admin.owner, *ctx.program_id, PortfolioError::Unauthorized);

    // require that remaining accounts are not empty
    require!(
        !ctx.remaining_accounts.is_empty(),
        PortfolioError::AccountsNotProvided
    );

    // Populate the token details account.
    let token_details = &mut ctx.accounts.token_details;
    token_details.decimals = decimals;
    token_details.token_address = token_address;
    token_details.symbol = symbol;

    // Iterate over the remaining accounts to update the token list.
    for token_list_info in ctx.remaining_accounts.iter() {
        let mut token_list = Account::<TokenList>::try_from(token_list_info)?;
        // Check if the token is already in the list.
        let tokens_map = token_list
            .tokens
            .iter()
            .copied()
            .collect::<HashSet<[u8; 32]>>();

        require!(
            !tokens_map.contains(&symbol),
            TokenListError::TokenAlreadyAdded
        );

        // If there's space, add the token.
        if token_list.tokens.len() < MAX_TOKENS {
            token_list.tokens.push(symbol);
            // Exit account iteration after adding the token.
            token_list.exit(&crate::ID)?;
            break;
        } else if token_list.next_page.is_some() {
            // If there's no space, but there's a next page, continue.
            continue;
        } else {
            // If there's no space and no next page return PageFull error.
            // The off-chain client should handle this error and provide a way to add more tokens.
            return Err(TokenListError::TokenListFull.into());
        }
    }

    emit!(ParameterUpdatedEvent {
        pair: symbol,
        parameter: "P-ADDTOKEN".to_owned(),
        old_value: 0,
        new_value: 1
    });

    msg!("Token added successfully");

    Ok(())
}

pub fn add_native_token<'info>(
    ctx: Context<'_, '_, 'info, 'info, AddNativeToken<'info>>,
) -> Result<()> {
    // Validate that the signer is an admin.
    let admin = &ctx.accounts.admin;

    require_keys_eq!(*admin.owner, *ctx.program_id, PortfolioError::Unauthorized);

    // require that remaining accounts are not empty
    require!(
        !ctx.remaining_accounts.is_empty(),
        PortfolioError::AccountsNotProvided
    );

    let sol_details = &mut ctx.accounts.token_details;

    // Initialize token details for SOL
    sol_details.symbol = NATIVE_SYMBOL; // Fixed symbol for SOL
    sol_details.decimals = 9;
    sol_details.token_address = None;

    // Iterate over the remaining accounts to update the token list.
    for token_list_info in ctx.remaining_accounts.iter() {
        let mut token_list = Account::<TokenList>::try_from(token_list_info)?;
        // Check if the token is already in the list.
        let tokens_map = token_list
            .tokens
            .iter()
            .copied()
            .collect::<HashSet<[u8; 32]>>();

        require!(
            !tokens_map.contains(&NATIVE_SYMBOL),
            TokenListError::TokenAlreadyAdded
        );

        // If there's space, add the token.
        if token_list.tokens.len() < MAX_TOKENS {
            token_list.tokens.push(NATIVE_SYMBOL);
            // Exit account iteration after adding the token.
            token_list.exit(&crate::ID)?;
            break;
        } else if token_list.next_page.is_some() {
            // If there's no space, but there's a next page, continue.
            continue;
        } else {
            // If there's no space and no next page return PageFull error.
            // The off-chain client should handle this error and provide a way to add more tokens.
            return Err(TokenListError::TokenListFull.into());
        }
    }

    emit!(ParameterUpdatedEvent {
        pair: NATIVE_SYMBOL,
        parameter: "P-ADDTOKEN".to_owned(),
        old_value: 0,
        new_value: 1
    });

    Ok(())
}

// Remove a token from the system:
// - Validates caller is admin
// - Removes token from TokenList
// - Closes TokenDetails PDA
pub fn remove_token<'info>(
    ctx: Context<'_, '_, 'info, 'info, RemoveToken<'info>>,
    symbol: [u8; 32],
) -> Result<()> {
    // Validate remaining accounts are not empty
    require!(
        !ctx.remaining_accounts.is_empty(),
        PortfolioError::AccountsNotProvided
    );

    // Check the program is paused
    let global_config = &ctx.accounts.global_config;
    require!(
        global_config.program_paused,
        PortfolioError::ProgramNotPaused
    );

    for token_list_info in ctx.remaining_accounts.iter() {
        let mut token_list = Account::<TokenList>::try_from(token_list_info)?;

        // deserialize the token list into a hashset
        let mut tokens_map = token_list
            .tokens
            .iter()
            .copied()
            .collect::<HashSet<[u8; 32]>>();

        if !tokens_map.contains(&symbol) && token_list.next_page.is_none() {
            return Err(TokenListError::TokenNotFound.into());
        } else if !tokens_map.contains(&symbol) && token_list.next_page.is_some() {
            continue;
        } else {
            tokens_map.remove(&symbol);
            token_list.tokens = tokens_map.into_iter().collect();
            // Serialize the updated data back into the account
            token_list.exit(&crate::ID)?;
        }
    }

    emit!(ParameterUpdatedEvent {
        pair: symbol,
        parameter: "P-REMOVETOKEN".to_owned(),
        old_value: 1,
        new_value: 0
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(symbol: [u8; 32])]
pub struct AddToken<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    /// Confirm there is an admin account
    /// CHECK: Used to check if authority/signer is admin
    #[account(
        seeds = [b"admin", authority.key().as_ref()],
        bump
    )]
    pub admin: AccountInfo<'info>,

    /// The Program Account
    /// CHECK: Used to set the authority for the associated token account
    #[account(
        constraint = spl_vault.owner == __program_id,
        seeds = [b"spl_vault"],
        bump,
    )]
    pub spl_vault: AccountInfo<'info>,

    /// The PDA storing details for native SOL.
    /// Seeds: [b"token_details", symbol.as_ref()]
    /// Space = TokenDetails::LEN
    #[account(
        init,
        payer = authority,
        space = TokenDetails::LEN,
        seeds = [b"token_details", symbol.as_ref()],
        bump
    )]
    pub token_details: Account<'info, TokenDetails>,

    /// The token mint for the supported token
    pub token_mint: Account<'info, Mint>,

    /// This creates the ATA for the `token_details` PDA and sets the program as its authority.
    /// This ATA will hold tokens deposited by users.
    #[account(
        init,
        payer = authority,
        constraint = spl_vault.owner == __program_id,
        associated_token::mint = token_mint,
        associated_token::authority = spl_vault,
    )]
    pub spl_token_account: Account<'info, TokenAccount>,

    /// Programs & Sysvars
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(Accounts)]
pub struct AddNativeToken<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    /// Confirm there is an admin account
    /// CHECK: Used to check if authority/signer is admin
    #[account(
        seeds = [b"admin", authority.key().as_ref()],
        bump
    )]
    pub admin: AccountInfo<'info>,

    /// The PDA storing details for native SOL.
    /// Seeds: [b"token_details", NATIVE_SYMBOL.as_ref()]
    /// Space = TokenDetails::LEN
    #[account(
        init,
        payer = authority,
        space = TokenDetails::LEN,
        seeds = [b"token_details", NATIVE_SYMBOL.as_ref()],
        bump
    )]
    pub token_details: Account<'info, TokenDetails>,

    /// Programs & Sysvars
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(symbol: [u8; 32])]
pub struct RemoveToken<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    // Verify that user is an admin by checking their PDA.
    #[account(
        seeds = [b"admin", authority.key().as_ref()],
        bump
    )]
    pub admin: Account<'info, Admin>,

    #[account(
        seeds = [b"global_config"],
        bump
    )]
    pub global_config: Account<'info, GlobalConfig>,

    // Close the token_details account. This requires that:
    // - token_details matches the symbol being removed (you can add a constraint to check this)
    // - The close attribute sends lamports back to receiver when this account is closed.
    #[account(
        mut,
        close = receiver,
        seeds = [b"token_details", symbol.as_ref()],
        bump
    )]
    pub token_details: Account<'info, TokenDetails>,

    #[account(mut)]
    pub receiver: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}
