use core::str;
use std::hash::Hash;
use std::collections::HashMap;

use serde::{Serialize,Deserialize};

use crate::models::{TxOutput, utxo};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Outpoint{
    pub tx_id: [u8;32],
    pub index: u32,
}

pub type UTXOSet = HashMap<Outpoint, TxOutput>;
