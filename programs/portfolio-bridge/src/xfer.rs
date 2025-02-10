use anchor_lang::prelude::*;

// Define the XFER struct
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct XFER {
    pub nonce: u64,           // uint64 -> u64
    pub transaction: Tx,      // IPortfolio.Tx -> Use an enum or struct for Tx
    pub trader: Pubkey,       // address -> Pubkey
    pub symbol: [u8; 32],     // bytes32 -> Fixed-size [u8; 32] array
    pub quantity: u64,        // uint256 -> u64
    pub timestamp: i64,       // uint256 -> i64
    pub customdata: [u8; 28], // bytes28 -> Fixed-size [u8; 28] array
}

impl XFER {
    // Constructor for the XFER struct
    pub fn new(
        transaction: Tx,
        trader: Pubkey,
        symbol: [u8; 32],
        quantity: u64,
        timestamp: i64,
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
        }
    }
}

pub fn pack_xfer_message(xfer: &XFER) -> Result<Vec<u8>> {
    let mut slot0 = [0u8; 32];
    slot0[..18].copy_from_slice(&xfer.customdata); // customdata: 18 bytes
    slot0[18..22].copy_from_slice(&xfer.timestamp.to_be_bytes()); // timestamp: 8 bytes
    slot0[22..30].copy_from_slice(&xfer.nonce.to_be_bytes()); // nonce: 8 bytes
    slot0[30] = xfer.transaction.clone() as u8; // transaction: 1 byte
    slot0[31] = XChainMsgType::XFER as u8; // message type: 1 byte

    let mut slot1 = [0u8; 32];
    slot1.copy_from_slice(&xfer.trader.to_bytes());

    let mut slot2 = [0u8; 32];
    slot2.copy_from_slice(&xfer.symbol);

    let mut slot3 = [0u8; 32];
    slot3[24..].copy_from_slice(&xfer.quantity.to_be_bytes()); // quantity: 8 bytes

    Ok([
        slot0.to_vec(),
        slot1.to_vec(),
        slot2.to_vec(),
        slot3.to_vec(),
    ]
        .concat())
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

    Ok(XFER {
        nonce,
        transaction,
        trader: trader.into(),
        symbol,
        quantity,
        timestamp: i64::from(timestamp),
        customdata,
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

#[repr(u8)]
pub enum XChainMsgType {
    XFER, // Add other types if needed
}

impl TryFrom<u8> for XChainMsgType {
    type Error = Error;

    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(XChainMsgType::XFER),
            _ => err!(AnchorError::XFERError)
        }
    }
}

#[error_code]
pub enum AnchorError {
    #[msg("XFER error occurred")]
    XFERError
}
