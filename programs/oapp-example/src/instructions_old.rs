use crate::state::{Admin, GlobalConfig, TokenList};
use anchor_lang::{
    prelude::*,
    solana_program::{keccak::hash, system_program::ID as SYSTEM_ID},
};
use cpi_helper::CpiContext;
use endpoint::{
    self,
    cpi::accounts::{Clear, ClearCompose, Quote, RegisterOApp, Send, SendCompose, SetDelegate},
    instructions::{
        ClearComposeParams, ClearParams, QuoteParams, RegisterOAppParams, SendComposeParams,
        SendParams, SetDelegateParams,
    },
    ConstructCPIContext, MessagingFee, MessagingReceipt, COMPOSED_MESSAGE_HASH_SEED, ENDPOINT_SEED,
    NONCE_SEED, OAPP_SEED, PAYLOAD_HASH_SEED,
};

// pub fn register_oapp(
//     endpoint_program: Pubkey,
//     accounts: &[AccountInfo],
//     seeds: &[&[u8]],
//     params: RegisterOAppParams,
// ) -> Result<()> {
//     let cpi_ctx = RegisterOApp::construct_context(endpoint_program, accounts)?;
//     endpoint::cpi::register_oapp(cpi_ctx.with_signer(&[&seeds]), params)
// }
//
// pub fn set_delegate(
//     endpoint_program: Pubkey,
//     accounts: &[AccountInfo],
//     seeds: &[&[u8]],
//     params: SetDelegateParams,
// ) -> Result<()> {
//     let cpi_ctx = SetDelegate::construct_context(endpoint_program, accounts)?;
//     endpoint::cpi::set_delegate(cpi_ctx.with_signer(&[&seeds]), params)
// }

// pub fn quote(
//     endpoint_program: Pubkey,
//     accounts: &[AccountInfo],
//     params: QuoteParams,
// ) -> Result<MessagingFee> {
//     let cpi_ctx = Quote::construct_context(endpoint_program, accounts)?;
//     let result = endpoint::cpi::quote(cpi_ctx, params)?;
//     Ok(result.get())
// }
//
// pub fn clear(
//     endpoint_program: Pubkey,
//     accounts: &[AccountInfo],
//     seeds: &[&[u8]],
//     params: ClearParams,
// ) -> Result<[u8; 32]> {
//     let cpi_ctx = Clear::construct_context(endpoint_program, accounts)?;
//     let result = endpoint::cpi::clear(cpi_ctx.with_signer(&[&seeds]), params)?;
//     Ok(result.get())
// }
//
// pub fn send_compose(
//     endpoint_program: Pubkey,
//     accounts: &[AccountInfo],
//     seeds: &[&[u8]],
//     params: SendComposeParams,
// ) -> Result<()> {
//     let cpi_ctx = SendCompose::construct_context(endpoint_program, accounts)?;
//     endpoint::cpi::send_compose(cpi_ctx.with_signer(&[&seeds]), params)
// }
//
// pub fn clear_compose(
//     endpoint_program: Pubkey,
//     accounts: &[AccountInfo],
//     seeds: &[&[u8]],
//     params: ClearComposeParams,
// ) -> Result<()> {
//     let cpi_ctx = ClearCompose::construct_context(endpoint_program, accounts)?;
//     endpoint::cpi::clear_compose(cpi_ctx.with_signer(&[&seeds]), params)
// }
//
// pub fn get_accounts_for_clear(
//     endpoint_program: Pubkey,
//     receiver: &Pubkey,
//     src_eid: u32,
//     sender: &[u8; 32],
//     nonce: u64,
// ) -> Vec<LzAccount> {
//     let (nonce_account, _) = Pubkey::find_program_address(
//         &[NONCE_SEED, &receiver.to_bytes(), &src_eid.to_be_bytes(), sender],
//         &endpoint_program,
//     );
//
//     let (payload_hash_account, _) = Pubkey::find_program_address(
//         &[
//             PAYLOAD_HASH_SEED,
//             &receiver.to_bytes(),
//             &src_eid.to_be_bytes(),
//             sender,
//             &nonce.to_be_bytes(),
//         ],
//         &endpoint_program,
//     );
//
//     let (oapp_registry_account, _) =
//         Pubkey::find_program_address(&[OAPP_SEED, &receiver.to_bytes()], &endpoint_program);
//     let (event_authority_account, _) =
//         Pubkey::find_program_address(&[EVENT_SEED], &endpoint_program);
//     let (endpoint_settings_account, _) =
//         Pubkey::find_program_address(&[ENDPOINT_SEED], &endpoint_program);
//
//     vec![
//         LzAccount { pubkey: endpoint_program, is_signer: false, is_writable: false },
//         LzAccount { pubkey: *receiver, is_signer: false, is_writable: false },
//         LzAccount { pubkey: oapp_registry_account, is_signer: false, is_writable: false },
//         LzAccount { pubkey: nonce_account, is_signer: false, is_writable: true },
//         LzAccount { pubkey: payload_hash_account, is_signer: false, is_writable: true },
//         LzAccount { pubkey: endpoint_settings_account, is_signer: false, is_writable: true },
//         LzAccount { pubkey: event_authority_account, is_signer: false, is_writable: false },
//         LzAccount { pubkey: endpoint_program, is_signer: false, is_writable: false },
//     ]
// }
//
// pub fn get_accounts_for_send_compose(
//     endpoint_program: Pubkey,
//     from: &Pubkey,
//     to: &Pubkey,
//     guid: &[u8; 32],
//     index: u16,
//     composed_message: &[u8],
// ) -> Vec<LzAccount> {
//     let (composed_message_account, _) = Pubkey::find_program_address(
//         &[
//             COMPOSED_MESSAGE_HASH_SEED,
//             &from.to_bytes(),
//             &to.to_bytes(),
//             &guid[..],
//             &index.to_be_bytes(),
//             &hash(composed_message).to_bytes(),
//         ],
//         &endpoint_program,
//     );
//
//     let (event_authority_account, _) =
//         Pubkey::find_program_address(&[EVENT_SEED], &endpoint_program);
//
//     vec![
//         LzAccount { pubkey: endpoint_program, is_signer: false, is_writable: false },
//         LzAccount { pubkey: *from, is_signer: false, is_writable: false },
//         LzAccount { pubkey: Pubkey::default(), is_signer: true, is_writable: true },
//         LzAccount { pubkey: composed_message_account, is_signer: false, is_writable: true },
//         LzAccount { pubkey: SYSTEM_ID, is_signer: false, is_writable: false },
//         LzAccount { pubkey: event_authority_account, is_signer: false, is_writable: false },
//         LzAccount { pubkey: endpoint_program, is_signer: false, is_writable: false },
//     ]
// }
//
// pub fn get_accounts_for_clear_compose(
//     endpoint_program: crate::EVENT_SEED,
//     from: &Pubkey,
//     to: &Pubkey,
//     guid: &[u8; 32],
//     index: u16,
//     composed_message: &[u8],
// ) -> Vec<LzAccount> {
//     let (composed_message_account, _) = Pubkey::find_program_address(
//         &[
//             COMPOSED_MESSAGE_HASH_SEED,
//             &from.to_bytes(),
//             &to.to_bytes(),
//             &guid[..],
//             &index.to_be_bytes(),
//             &hash(composed_message).to_bytes(),
//         ],
//         &endpoint_program,
//     );
//
//     let (event_authority_account, _) =
//         Pubkey::find_program_address(&[EVENT_SEED], &endpoint_program);
//
//     vec![
//         LzAccount { pubkey: endpoint_program, is_signer: false, is_writable: false },
//         LzAccount { pubkey: *to, is_signer: false, is_writable: false },
//         LzAccount { pubkey: composed_message_account, is_signer: false, is_writable: true },
//         LzAccount { pubkey: event_authority_account, is_signer: false, is_writable: false },
//         LzAccount { pubkey: endpoint_program, is_signer: false, is_writable: false },
//     ]
// }
