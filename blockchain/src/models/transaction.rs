use serde::{Serialize,Deserialize};
use bincode;
use sha2::{Sha256, Digest};
use crate::models::utxo::{Outpoint, UTXOSet};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TxInput{
    pub outpoint: Outpoint,
    pub pub_key: Vec<u8>,
    pub signature: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
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
            data.extend(&inp.signature);
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

    pub fn pub_key_hash(pub_key: &Vec<u8>) -> Vec<u8>{
        let sha256_hash = Sha256::digest(pub_key);
        sha256_hash[12..32].to_vec()
    }

    pub fn coinbase(miner_pub_key: Vec<u8>) -> Self{
        let reward = 50;
        let output = TxOutput{
            value: reward,
            pub_key_hash: Transaction::pub_key_hash(&miner_pub_key),
        };
        Transaction::new(vec![], vec![output])
    }

    pub fn validate_tx(&self, utxo_set: &HashMap<Outpoint, TxOutput>, pub_key: &Vec<u8>) -> bool{
        let pub_key_hash = Transaction::pub_key_hash(pub_key);
        let mut input_sum = 0;
        let mut output_sum = 0;

        for input in &self.inputs{
            if let Some(prev_utxo) = utxo_set.get(&input.outpoint){
                if prev_utxo.pub_key_hash != pub_key_hash{
                    return false;
                }
                input_sum += prev_utxo.value;
            } else {
                return false;
            }
        }

        for output in &self.outputs {
            output_sum += output.value;
        }

        if input_sum < output_sum {
            return false;
        }

        true
    }

}
