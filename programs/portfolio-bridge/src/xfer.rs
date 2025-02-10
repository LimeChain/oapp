use anchor_lang::prelude::*;

// Define the XFER struct
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct XFER {
    pub nonce: u64,           // uint64 -> u64
    pub transaction: Tx,      // IPortfolio.Tx -> Use an enum or struct for Tx
    pub trader: [u8; 20],     // address -> Pubkey
    pub symbol: [u8; 32],     // bytes32 -> Fixed-size [u8; 32] array
    pub quantity: [u64; 4],   // uint256 -> u64
    pub timestamp: u32,       // uint256 -> i64
    pub customdata: [u8; 28], // bytes28 -> Fixed-size [u8; 28] array
    pub message_type: XChainMsgType,
}

impl XFER {
    // Constructor for the XFER struct
    pub fn new(
        transaction: Tx,
        trader: [u8; 20],
        symbol: [u8; 32],
        quantity: [u64; 4],
        timestamp: u32,
        customdata: [u8; 28],
    ) -> Self {
        XFER {
            nonce: 0,
            transaction,
            trader,
            symbol,
            quantity,
            timestamp,
            customdata,
            message_type: XChainMsgType::XFER,
        }
    }
    pub fn pack_xfer_message(&self) -> Result<Vec<u8>> {
        // * slot0: trader(20), nonce(8), transaction(2), XChainMsgType(2)
        //     * slot1: symbol(32)
        // * slot2: quantity(32)
        // * slot3: customdata(28), timestamp(4)
        let mut slot0 = [0u8; 32];
        slot0[0..20].copy_from_slice(&self.trader);
        slot0[20..28].copy_from_slice(&self.nonce.to_be_bytes());
        slot0[28..30]
            .copy_from_slice(convert_u16_to_two_u8s_be(self.transaction.clone() as u16).as_slice());
        slot0[30..32]
            .copy_from_slice(convert_u16_to_two_u8s_be(self.message_type.clone() as u16).as_slice());
        // slot0[..28].copy_from_slice(&xfer.customdata); // customdata: 18 bytes
        // slot0[28..36].copy_from_slice(&xfer.timestamp.to_be_bytes()); // timestamp: 8 bytes
        // slot0[36..44].copy_from_slice(&xfer.nonce.to_be_bytes()); // nonce: 8 bytes
        // slot0[44] = xfer.transaction.clone() as u8; // transaction: 1 byte
        // slot0[45] = XChainMsgType::XFER as u8; // message type: 1 byte
    
        let mut slot1 = [0u8; 32];
        slot1.copy_from_slice(&self.symbol);
    
        let mut slot2 = [0u8; 32];
        slot2.copy_from_slice(convert(&self.quantity).as_slice());
    
        let mut slot3 = [0u8; 32];
        slot3[0..28].copy_from_slice(&self.customdata);
        slot3[28..32].copy_from_slice(self.timestamp.to_be_bytes().as_slice());
    
        Ok([
            slot0.to_vec(),
            slot1.to_vec(),
            slot2.to_vec(),
            slot3.to_vec(),
        ]
        .concat())
    }
    
}


fn convert_u16_to_two_u8s_be(integer: u16) -> [u8; 2] {
    [(integer >> 8) as u8, integer as u8]
}

pub fn convert(data: &[u64; 4]) -> [u8; 32] {
    let mut res = [0; 32];
    for i in 0..4 {
        res[4 * i..][..8].copy_from_slice(&data[i].to_le_bytes());
    }
    res
}

pub fn unpack_xfer_message(payload: &[u8]) -> Result<XFER> {
    if payload.len() != 128 {
        return err!(AnchorError::XFERError);
    }

    let slot0 = &payload[0..32];
    let slot1 = &payload[32..64];
    let slot2 = &payload[64..96];
    let slot3 = &payload[96..128];

    let customdata = {
        let mut data = [0u8; 28];
        data.copy_from_slice(&slot0[..28]);
        data
    };
    let timestamp = i32::from_be_bytes(slot0[18..22].try_into().unwrap());
    let nonce = u64::from_be_bytes(slot0[22..30].try_into().unwrap());
    let transaction = slot0[30];
    let message_type = slot0[31];

    XChainMsgType::try_from(message_type)?;

    let trader = {
        let mut data = [0u8; 32];
        data.copy_from_slice(slot1);
        data
    };

    let symbol = {
        let mut data = [0u8; 32];
        data.copy_from_slice(slot2);
        data
    };

    let quantity = u64::from_be_bytes(slot3[24..].try_into().unwrap());
    let transaction = Tx::try_from(transaction)?;
    !unimplemented!("withdraw called");
    // Ok(XFER {
    //     nonce,
    //     transaction,
    //     trader: trader.into(),
    //     symbol,
    //     quantity,
    //     timestamp: i64::from(timestamp),
    //     customdata,
    // })
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
#[repr(u16)]
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
    AutoFill, // 10
    CcTrade,  // Cross Chain Trade.
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

#[repr(u16)]
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum XChainMsgType {
    XFER, // Add other types if needed
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::xfer::Tx::CcTrade;

    #[test]
    fn test_pack_unpack() {
        let message = XFER {
            nonce: 5,
            transaction: CcTrade,
            trader: [
                189, 65, 233, 24, 30, 195, 207, 32, 141, 172, 96, 226, 204, 251, 242, 142, 185,
                188, 9, 214,
            ]
            .try_into()
            .unwrap(),
            symbol: [
                83, 79, 76, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0,
            ],
            quantity: [10000, 0, 0, 0],
            timestamp: 1620000000,
            customdata: [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            message_type: XChainMsgType::XFER,
        };
        let result = message.pack_xfer_message();
        assert!(result.is_ok());
        let msg = [
            189, 65, 233, 24, 30, 195, 207, 32, 141, 172, 96, 226, 204, 251, 242, 142, 185, 188, 9,
            214, 0, 0, 0, 0, 0, 0, 0, 5, 0, 11, 0, 0, 83, 79, 76, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 39, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 96, 143, 61, 0,
        ];
        // let msg = [
        //     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 96, 143, 61, 0, 0, 0, 0, 0, 0, 0, 0,
        //     5, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 189, 65, 233, 24, 30, 195, 207, 32, 141, 172, 96, 226, 204, 251, 242, 142, 185,
        //     188, 9, 214, 83, 79, 76, 0, 0, 0, 0, 0,
        //     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        //     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 16,
        // ];
        assert_eq!(result.unwrap().to_vec(), msg);
    }
}
