use anchor_lang::prelude::*;

// pub const MAX_TOKENS: usize = 20;
// pub const NATIVE_SYMBOL: [u8; 32] = [
//     83, 79, 76, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//     0,
// ];

// ACCOUNTS

#[account]
#[derive(InitSpace)]
pub struct Portfolio {
    pub admin: Pubkey,
    pub global_config: GlobalConfig,
    pub endpoint_program: Pubkey,
    pub bump: u8,
}

impl Portfolio {
    pub const LEN: usize = 8 + Portfolio::INIT_SPACE; // descriminator + sum of each field's len
}

#[account]
#[derive(InitSpace)]
pub struct Remote {
    pub address: [u8; 32],
    pub bump: u8,
}

impl Remote {
    pub const SIZE: usize = 8 + Self::INIT_SPACE;
}

use crate::consts::MAX_TOKENS;

// ACCOUNTS

#[derive(InitSpace, AnchorDeserialize, AnchorSerialize, Clone)]
pub struct GlobalConfig {
    pub allow_deposit: bool,
    pub program_paused: bool,
    pub native_deposits_restricted: bool,
    pub src_chain_id: u16,
    pub default_chain_id: u32, // Dexalot L1
}

impl GlobalConfig {
    pub const LEN: usize = 8 + // discriminator
        GlobalConfig::INIT_SPACE;
}

#[account]
pub struct Admin {}

impl Admin {
    pub const LEN: usize = 8; // discriminator_admin (u8)
}

#[account]
#[derive(InitSpace)]
pub struct BannedAccount {
    pub reason: BanReason,
}

impl BannedAccount {
    pub const LEN: usize = 8 + // discriminator
        BanReason::INIT_SPACE; // reason (BanReason)
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Eq, PartialEq, InitSpace)]
pub enum BanReason {
    NotBanned,
    Ofac,
    Abuse,
    Terms,
}

#[account]
pub struct TokenList {
    pub next_page: Option<Pubkey>,
    pub tokens: Vec<[u8; 32]>,
}

impl TokenList {
    pub const LEN: usize = 8 + // discriminator
        33 + // next_page (Option<Pubkey>)
        4 + // length of the Vec<Pubkey> (u32)
        MAX_TOKENS * 32; // tokens (Vec<[u8; 32]>)
}

#[account]
#[derive(Default, InitSpace)]
pub struct TokenDetails {
    pub decimals: u8,
    pub symbol: [u8; 32],
    pub token_address: Option<Pubkey>,
}

impl TokenDetails {
    pub const LEN: usize = 8 + TokenDetails::INIT_SPACE; // discriminator
}
