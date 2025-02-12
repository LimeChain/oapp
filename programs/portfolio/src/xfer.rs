use anchor_lang::prelude::*;

/// Rust equivalent of Dexalot's XFER Solidity struct
/// The specific type mappings are left as comments next to the struct
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct XFER {
    pub nonce: u64,                  // uint64 -> u64
    pub transaction: Tx,             // IPortfolio.Tx -> Tx (u8 repr)
    pub trader: Pubkey,              // address -> Pubkey
    pub symbol: [u8; 32],            // bytes32 -> [u8; 32]
    pub quantity: [u64; 4],          // uint256 -> [u64; 4]
    pub timestamp: u32,              // uint256 -> u32
    pub custom_data: [u8; 18],       // bytes18 -> [u8; 18]
    pub message_type: XChainMsgType, // IPortfolio.XChainMsgType -> XChainMsgType (u8 repr)
}

impl XFER {
    pub fn new(
        transaction: Tx,
        trader: Pubkey,
        symbol: [u8; 32],
        quantity: [u64; 4],
        timestamp: u32,
        custom_data: [u8; 18],
    ) -> Self {
        XFER {
            nonce: 0,
            transaction,
            trader,
            symbol,
            quantity,
            timestamp,
            custom_data,
            message_type: XChainMsgType::XFER,
        }
    }

    /// Packs a given XFER struct into exactly 4 groups of 32 byte arrays
    pub fn pack_xfer_message(&self) -> Result<Vec<u8>> {
        let mut slot0 = [0u8; 32]; // 18 (custom_data) | 4 (timestamp) | 8 (nonce)  | 1 (Tx) | 1 (XChainMsgType)
        let mut slot1 = [0u8; 32]; // 32 (trader)
        let mut slot2 = [0u8; 32]; // 32 (symbol)
        let mut slot3 = [0u8; 32]; // 32 (quantity)

        slot0[0..18].copy_from_slice(&self.custom_data);
        slot0[18..22].copy_from_slice(&self.timestamp.to_be_bytes());
        slot0[22..30].copy_from_slice(&self.nonce.to_be_bytes());
        slot0[30] = self.transaction.clone() as u8;
        slot0[31] = self.message_type.clone() as u8;

        slot1.copy_from_slice(&self.trader.to_bytes());
        slot2.copy_from_slice(&self.symbol);
        slot3.copy_from_slice(convert_u64_4_to_u8_32_be(&self.quantity).as_slice());

        Ok([
            slot0.to_vec(),
            slot1.to_vec(),
            slot2.to_vec(),
            slot3.to_vec(),
        ]
        .concat())
    }
}

/// Unpacks a given byte array of exactly 128 bytes into an XFER struct
pub fn unpack_xfer_message(payload: &[u8]) -> Result<XFER> {
    if payload.len() != 128 {
        return err!(AnchorError::XFERError);
    }

    let mut slot0 = [0u8; 32];
    slot0.copy_from_slice(&payload[0..32]);
    let mut slot1 = [0u8; 32];
    slot1.copy_from_slice(&payload[32..64]);
    let mut slot2 = [0u8; 32];
    slot2.copy_from_slice(&payload[64..96]);
    let mut slot3 = [0u8; 32];
    slot3.copy_from_slice(&payload[96..128]);

    let mut custom_data = [0u8; 18];
    custom_data.copy_from_slice(&slot0[..18]);
    let timestamp = u32::from_be_bytes(slot0[18..22].try_into().unwrap());
    let nonce = u64::from_be_bytes(slot0[22..30].try_into().unwrap());
    let transaction = Tx::try_from(slot0[30])?;
    let message_type = XChainMsgType::try_from(slot0[31])?;

    let trader = Pubkey::new_from_array(slot1);
    let symbol = slot2;
    let mut quantity = [0u64; 4];
    quantity.copy_from_slice(&convert_u8_32_be_to_u64_4(&slot3));

    Ok(XFER {
        nonce,
        transaction,
        trader,
        symbol,
        quantity,
        timestamp,
        custom_data,
        message_type,
    })
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum Tx {
    Withdraw,
    Deposit,
    Execution,
    IncreaseAvail,
    DecreaseAvail,
    IxferSent,    // Subnet Sent. I for Internal to Subnet
    IxferRec,     // Subnet Received. I for Internal to Subnet
    RecoverFunds, // Obsolete as of 2/1/2024 CD
    AddGas,
    RemoveGas,
    AutoFill,
    CcTrade, // Cross Chain Trade.
    ConvertFrom,
    ConvertTo,
}

impl TryFrom<u8> for Tx {
    type Error = Error;

    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Tx::Withdraw),
            1 => Ok(Tx::Deposit),
            2 => Ok(Tx::Execution),
            3 => Ok(Tx::IncreaseAvail),
            4 => Ok(Tx::DecreaseAvail),
            5 => Ok(Tx::IxferSent),
            6 => Ok(Tx::IxferRec),
            7 => Ok(Tx::RecoverFunds),
            8 => Ok(Tx::AddGas),
            9 => Ok(Tx::RemoveGas),
            10 => Ok(Tx::AutoFill),
            11 => Ok(Tx::CcTrade),
            12 => Ok(Tx::ConvertFrom),
            13 => Ok(Tx::ConvertTo),
            _ => err!(AnchorError::XFERError),
        }
    }
}

#[repr(u8)]
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum XChainMsgType {
    XFER,
}

impl TryFrom<u8> for XChainMsgType {
    type Error = Error;

    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(XChainMsgType::XFER),
            _ => err!(AnchorError::XFERError),
        }
    }
}

#[error_code]
pub enum AnchorError {
    #[msg("XFER error occurred")]
    XFERError,
}

fn convert_u64_4_to_u8_32_be(data: &[u64; 4]) -> [u8; 32] {
    let mut res = [0u8; 32];
    for i in 0..4 {
        res[8 * i..8 * (i + 1)].copy_from_slice(&data[i].to_be_bytes());
    }
    res
}

fn convert_u8_32_be_to_u64_4(data: &[u8; 32]) -> [u64; 4] {
    let mut res = [0u64; 4];
    for i in 0..4 {
        res[i] = u64::from_be_bytes(data[8 * i..8 * (i + 1)].try_into().unwrap());
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::prelude::*;

    #[test]
    fn test_pack_xfer_message() {
        let transaction = Tx::Deposit;
        let trader = Pubkey::new_unique();
        let symbol = *b"BTCUSD";
        let mut padded_symbol = [0u8; 32];
        padded_symbol[..symbol.len()].copy_from_slice(&symbol);
        let quantity = [100u64, 200u64, 300u64, 400u64];
        let timestamp = 1627545600;
        let custom_data = [0u8; 18];
        let xfer = XFER::new(
            transaction,
            trader,
            padded_symbol,
            quantity,
            timestamp,
            custom_data,
        );

        let packed_message = xfer.pack_xfer_message().unwrap();
        assert_eq!(packed_message.len(), 128);
    }

    #[test]
    fn test_pack_unpack_xfer_message() {
        let transaction = Tx::Deposit;
        let trader = Pubkey::new_unique();
        let symbol = *b"BTCUSD";
        let mut padded_symbol = [0u8; 32];
        padded_symbol[..symbol.len()].copy_from_slice(&symbol);
        let quantity = [100u64, 200u64, 300u64, 400u64];
        let timestamp = 1627545600;
        let custom_data = [0u8; 18];
        let xfer = XFER::new(
            transaction,
            trader,
            padded_symbol,
            quantity,
            timestamp,
            custom_data,
        );

        let packed_message = xfer.pack_xfer_message().unwrap();
        let unpacked_xfer = unpack_xfer_message(&packed_message).unwrap();

        assert_eq!(xfer.custom_data, unpacked_xfer.custom_data);
        assert_eq!(xfer.transaction, unpacked_xfer.transaction);
        assert_eq!(xfer.trader, unpacked_xfer.trader);
        assert_eq!(xfer.quantity, unpacked_xfer.quantity);
        assert_eq!(xfer.timestamp, unpacked_xfer.timestamp);
        assert_eq!(xfer.message_type, unpacked_xfer.message_type);
        assert_eq!(xfer.symbol, unpacked_xfer.symbol); // Most important assertion as we are checking the unpacked symbol with the original pre-padding symbol
        assert_eq!(xfer.nonce, unpacked_xfer.nonce);
    }

    #[test]
    fn test_convert_u64_4_to_u8_32_be() {
        let input = [
            0x0102030405060708,
            0x090a0b0c0d0e0f10,
            0x1112131415161718,
            0x191a1b1c1d1e1f20,
        ];
        let expected_output: [u8; 32] = [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31, 32,
        ];
        let result = convert_u64_4_to_u8_32_be(&input);
        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_convert_u8_32_be_to_u64_4() {
        let input: [u8; 32] = [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31, 32,
        ];
        let expected_output = [
            0x0102030405060708,
            0x090a0b0c0d0e0f10,
            0x1112131415161718,
            0x191a1b1c1d1e1f20,
        ];
        let result = convert_u8_32_be_to_u64_4(&input);
        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_round_trip_conversion() {
        let original_u64_array = [
            0x0102030405060708,
            0x090a0b0c0d0e0f10,
            0x1112131415161718,
            0x191a1b1c1d1e1f20,
        ];
        let u8_array = convert_u64_4_to_u8_32_be(&original_u64_array);
        let back_to_u64_array = convert_u8_32_be_to_u64_4(&u8_array);
        assert_eq!(original_u64_array, back_to_u64_array);
    }
}
