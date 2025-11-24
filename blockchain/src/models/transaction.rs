use serde::{Serialize,Deserialize};
use bincode;
use sha2::{Sha256, Digest};
use crate::models::utxo::Outpoint;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TxInput{
    pub outpoint: Outpoint,
    pub pub_key: Vec<u8>,
    pub signature: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TxOutput{
    pub value: u64,
    pub pub_key_hash: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction{
    pub inputs: Vec<TxInput>,
    pub outputs: Vec<TxOutput>,

}

impl Transaction{
    pub fn new(inputs:Vec<TxInput>, outputs:Vec<TxOutput>) -> Self{
        Transaction{
            inputs,
            outputs,
        }
    }

    pub fn tx_id(&self) -> [u8; 32] {
        let mut data = Vec::new();

        for inp in &self.inputs {
            data.extend(&inp.outpoint.tx_id);
            data.extend(&inp.outpoint.index.to_be_bytes());
            data.extend(&inp.pub_key);
        }

        for out in &self.outputs {
            data.extend(&out.value.to_be_bytes());
            data.extend(&out.pub_key_hash);
        }

        let hash = Sha256::digest(&data);
        let mut id = [0u8; 32];
        id.copy_from_slice(&hash);
        id
    }

    pub fn coinbase(to:&str) -> Self{
        let reward = 50;
        let output = TxOutput{
            value: reward,
            pub_key_hash: to.as_bytes().to_vec(),
        };
        Transaction::new(vec![], vec![output])
    }

}
