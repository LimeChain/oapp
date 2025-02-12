pub const GAS_OPTIONS: [u8; 22] = [
    0, 3, 1, 0, 17, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 26, 128,
];
pub const PORTFOLIO_SEED: &[u8] = b"Portfolio";
pub const SOL_VAULT_SEED: &[u8] = b"SolVault";
pub const REMOTE_SEED: &[u8] = b"Remote";
pub const ADMIN_SEED: &[u8] = b"Admin";
pub const BANNED_ACCOUNT_SEED: &[u8] = b"Banned";

pub const ENDPOINT_SEEDS: &[u8] = &[69, 110, 100, 112, 111, 105, 110, 116];
pub const MAX_TOKENS: usize = 20;
pub const NATIVE_SYMBOL: [u8; 32] = [
    83, 79, 76, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0,
];
