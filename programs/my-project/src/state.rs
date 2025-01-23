use anchor_lang::prelude::*;

pub const MAX_TOKENS: usize = 20;
pub const NATIVE_SYMBOL: [u8; 32] = [
    83, 79, 76, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0,
];

// ACCOUNTS
#[account]
pub struct GlobalConfig {
    pub portfolio: Pubkey,
    pub mainnet_rfq: Pubkey,
    pub default_chain_id: u32, // Dexalot L1
    pub out_nonce: u64,        // Outgoing messages nonce
}

impl GlobalConfig {
    pub const LEN: usize = 8 + 32 + 32 + 4 + 8; // descriminator + sum of each field's len
}

#[account]
pub struct Admin {}

impl Admin {
    pub const LEN: usize = 8; // discriminator_admin (u8)
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
#[derive(Default)]
pub struct TokenDetails {
    pub decimals: u8,
    pub symbol: [u8; 32],
    pub token_address: Option<Pubkey>,
}

impl TokenDetails {
    pub const LEN: usize = 8 + // discriminator
        1 + // decimals (1 byte)
        32 + // symbol (32 bytes)
        33; // Option tag (1 byte) + Pubkey (32 bytes)
}
