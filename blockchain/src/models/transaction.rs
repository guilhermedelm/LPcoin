use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use secp256k1::{Secp256k1, Message, SecretKey, ecdsa::Signature, PublicKey};
use crate::models::utxo::Outpoint;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TxInput {
    pub outpoint: Outpoint,
    pub pub_key: Vec<u8>,
    pub signature: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TxOutput {
    pub value: u64,
    pub pub_key_hash: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub inputs: Vec<TxInput>,
    pub outputs: Vec<TxOutput>,
}

impl Transaction {
    pub fn new(inputs: Vec<TxInput>, outputs: Vec<TxOutput>) -> Self {
        Transaction { inputs, outputs }
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

    pub fn coinbase(to_address: &str) -> Self {
        let reward = 50;
        let output = TxOutput {
            value: reward,
            pub_key_hash: to_address.as_bytes().to_vec(),
        };
        Transaction::new(vec![], vec![output])
    }

    pub fn sign(&mut self, private_key: &SecretKey) {
        let secp = Secp256k1::new();
        let message_bytes = self.tx_id();
        let message = Message::from_digest_slice(&message_bytes).expect("Hash deve ter 32 bytes");

        let signature = secp.sign_ecdsa(&message, private_key);
        
        for input in &mut self.inputs {
            input.signature = signature.serialize_der().to_vec();
        }
    }

    pub fn verify(&self) -> bool {
        if self.is_coinbase() {
            return true;
        }

        let secp = Secp256k1::new();
        let message_bytes = self.tx_id();
        let message = Message::from_digest_slice(&message_bytes).expect("Hash deve ter 32 bytes");

        for input in &self.inputs {
            let signature = match Signature::from_der(&input.signature) {
                Ok(sig) => sig,
                Err(_) => return false,
            };

            let public_key = match PublicKey::from_slice(&input.pub_key) {
                Ok(pk) => pk,
                Err(_) => return false,
            };
            if secp.verify_ecdsa(&message, &signature, &public_key).is_err() {
                return false;
            }
        }
        true
    }

    pub fn is_coinbase(&self) -> bool {
        self.inputs.is_empty()
    }
}