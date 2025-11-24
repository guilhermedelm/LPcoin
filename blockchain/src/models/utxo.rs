use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Outpoint{
    pub tx_id: [u8;32],
    pub index: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct UTXO{
    pub outpoint: Outpoint,
    pub amount: u64,
    pub pub_key_hash: Vec<u8>,
}