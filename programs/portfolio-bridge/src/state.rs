use anchor_lang::prelude::*;

// pub const MAX_TOKENS: usize = 20;
// pub const NATIVE_SYMBOL: [u8; 32] = [
//     83, 79, 76, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//     0,
// ];

// ACCOUNTS

#[derive(Clone, InitSpace, AnchorDeserialize, AnchorSerialize)]
pub struct GlobalConfig {
    pub portfolio: Pubkey,
    pub mainnet_rfq: Pubkey,
    pub default_chain_id: u32, // Dexalot L1
}

#[account]
#[derive(InitSpace)]
pub struct Bridge {
    pub admin: Pubkey,
    pub global_config: GlobalConfig,
    pub endpoint_program: Pubkey,
    pub sol_vault: Pubkey,
    pub bump: u8,
}

impl Bridge {
    pub const LEN: usize = 8 + Bridge::INIT_SPACE; // descriminator + sum of each field's len
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
