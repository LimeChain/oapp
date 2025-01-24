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
