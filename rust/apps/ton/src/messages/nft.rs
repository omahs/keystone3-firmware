use alloc::{format, string::{String, ToString}};
use third_party::hex;

use super::traits::ParseCell;


pub const NFT_TRANSFER: u32 = 0x5fcc3d14;

pub enum NFTMessage {
    NFTTransferMessage(NFTTransferMessage)
}

impl ParseCell for NFTMessage {
    fn parse(cell: &crate::vendor::cell::ArcCell) -> Result<Self, crate::vendor::cell::TonCellError>
    where
        Self: Sized {
        cell.parse_fully(|parser| {
            let op_code = parser.load_u32(32)?;
            match op_code {
                NFT_TRANSFER => {
                    NFTTransferMessage::parse(cell).map(NFTMessage::NFTTransferMessage)
                }
                _ => Err(crate::vendor::cell::TonCellError::InternalError(format!(
                    "Invalid Op Code: {:X}",
                    op_code
                ))),
            }
        })
    }
}
 
pub struct NFTTransferMessage {
    query_id: String,
    new_owner_address: String,
    response_address: String,
    custom_payload: Option<String>,
    forward_ton_amount: String,
    forward_payload: Option<String>,
}

impl ParseCell for NFTTransferMessage {
    fn parse(cell: &crate::vendor::cell::ArcCell) -> Result<Self, crate::vendor::cell::TonCellError>
    where
        Self: Sized {
        cell.parse_fully(|parser| {
            let _op_code = parser.load_u32(32)?;
            let query_id = parser.load_u64(64)?.to_string();
            let new_owner_address = parser.load_address()?.to_base64_std();
            let response_address = parser.load_address()?.to_base64_std();
            let mut ref_index = 0;
            let custom_payload = if parser.load_bit()? {
                let payload = Some(hex::encode(cell.reference(ref_index)?.data.clone()));
                ref_index = ref_index + 1;
                payload
            } else {
                None
            };
            let forward_ton_amount = parser.load_coins()?.to_string();
            let forward_payload = if parser.load_bit()? {
                let payload = Some(hex::encode(cell.reference(ref_index)?.data.clone()));
                ref_index = ref_index + 1;
                payload
            } else {
                None
            };

            Ok(Self {
                query_id,
                new_owner_address,
                response_address,
                custom_payload,
                forward_ton_amount,
                forward_payload,
            })
        })
    }
}