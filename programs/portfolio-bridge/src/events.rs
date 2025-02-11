use anchor_lang::prelude::*;

use crate::state::BanReason;
use crate::xfer::Tx;

// BannedAccount events
// event BanStatusChanged(address indexed account, BanReason reason, bool banned);
#[event]
pub struct BanStatusChangedEvent {
    pub account: Pubkey,
    pub reason: BanReason,
    pub banned: bool,
}

// Admin events
#[event]
pub struct RoleGrantedEvent {
    pub role: [u8; 32],
    pub admin: Pubkey,
}

#[event]
pub struct RoleRevokedEvent {
    pub role: [u8; 32],
    pub admin: Pubkey,
}

// Portfolio events
#[event]
pub struct PortfolioUpdatedEvent {
    pub transaction: Tx,
    pub wallet: Pubkey,
    pub symbol: [u8; 32],
    pub quantity: u64,
    pub fee_charged: u64,
    pub total: u64,
    pub available: u64,
    pub wallet_other: Pubkey,
}

#[event]
pub struct ParameterUpdatedEvent {
    pub pair: [u8; 32],
    pub parameter: String,
    pub old_value: u64,
    pub new_value: u64,
}
