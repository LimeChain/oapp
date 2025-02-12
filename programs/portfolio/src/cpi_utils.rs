use anchor_lang::prelude::*;
use sha2::{Digest, Sha256};

pub fn create_instruction_data<T>(params: &T, instruction_name: &str) -> Vec<u8>
where
    T: Clone + AnchorSerialize + AnchorDeserialize,
{
    const PREFIX: &str = "global:";
    let full_len = PREFIX.len() + instruction_name.len();
    let mut instruction_full_name = Vec::with_capacity(full_len);
    instruction_full_name.extend_from_slice(PREFIX.as_bytes());
    instruction_full_name.extend_from_slice(instruction_name.as_bytes());

    let hash = Sha256::new()
        .chain_update(&instruction_full_name)
        .finalize();
    let discriminator = &hash[..8];

    let serialized_data = params.try_to_vec().unwrap();

    let mut instruction_data = Vec::with_capacity(8 + serialized_data.len());
    instruction_data.extend_from_slice(discriminator);
    instruction_data.extend_from_slice(&serialized_data);

    instruction_data
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct EndpointSendParams {
    pub dst_eid: u32,
    pub receiver: [u8; 32],
    pub message: Vec<u8>,
    pub options: Vec<u8>,
    pub native_fee: u64,
    pub lz_token_fee: u64,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct RegisterOAppParams {
    pub delegate: Pubkey,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct EndpointQuoteParams {
    pub sender: Pubkey,
    pub dst_eid: u32,
    pub receiver: [u8; 32],
    pub message: Vec<u8>,
    pub options: Vec<u8>,
    pub pay_in_lz_token: bool,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize, Default)]
pub struct MessagingFee {
    pub native_fee: u64,
    pub lz_token_fee: u64,
}
