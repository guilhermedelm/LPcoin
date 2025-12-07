use serde::{Serialize,Deserialize};
use bincode;
use sha2::{Sha256, Digest};
use crate::models::utxo::{Outpoint, UTXOSet};
use std::collections::HashMap;
use secp256k1::{Secp256k1, SecretKey, PublicKey as SecpPublicKey, Message, ecdsa::{Signature as EcdsaSignature, self}};
use crate::models::mempool::Mempool;

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

    pub fn sign_input(&mut self, sk: &SecretKey) {
        let secp = Secp256k1::new();

        let hash = self.tx_id();
        let msg = Message::from_digest_slice(&hash).expect("hash 32 bytes");

        let pubkey = SecpPublicKey::from_secret_key(&secp, sk);
        let pubkey_bytes = pubkey.serialize().to_vec();

        for inp in &mut self.inputs {
            inp.pub_key = pubkey_bytes.clone();

            let sig: EcdsaSignature = secp.sign_ecdsa(&msg, sk);
            let compact = sig.serialize_compact();
            inp.signature = compact.to_vec();
        }
    }

    pub fn verify_signatures(&self) -> bool {
        let secp = Secp256k1::new();

        let hash = self.tx_id();
        let msg = match Message::from_digest_slice(&hash) {
            Ok(m) => m,
            Err(_) => return false,
        };

        for inp in &self.inputs {
            if inp.signature.len() != 64 { return false; }

            let sig = match ecdsa::Signature::from_compact(&inp.signature) {
                Ok(s) => s,
                Err(_) => return false,
            };

            let pubkey = match SecpPublicKey::from_slice(&inp.pub_key) {
                Ok(pk) => pk,
                Err(_) => return false,
            };

            if secp.verify_ecdsa(&msg, &sig, &pubkey).is_err() {
                return false;
            }
        }

        true
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

    pub fn create_and_submit_transaction(
        sender_secret: &SecretKey,
        to_pub_key_bytes: Vec<u8>,
        amount: u64,
        mempool: &mut Mempool,
        utxos: &UTXOSet,
    ) -> Result<Transaction, String> {

        let secp = Secp256k1::new();

        // -------------------------------------------------------
        // 1) Obter a public key do remetente a partir da SecretKey
        // -------------------------------------------------------
        let sender_pub_key = SecpPublicKey::from_secret_key(&secp, sender_secret);
        let sender_pub_bytes = sender_pub_key.serialize().to_vec();

        // hash do remetente (para bater com outputs pub_key_hash)
        let sender_pub_key_hash = Transaction::pub_key_hash(&sender_pub_bytes);

        // -------------------------------------------------------
        // 2) Selecionar UTXOs pertencentes ao remetente
        // -------------------------------------------------------
        let mut selected: Vec<(Outpoint, TxOutput)> = Vec::new();
        let mut total_accumulated = 0;

        for (outpoint, utxo) in utxos.iter() {
            // UTXO pertence ao remetente?
            if utxo.pub_key_hash != sender_pub_key_hash {
                continue;
            }

            // Evitar double-spend dentro da mempool
            let is_spent_in_mempool = mempool.transactions.values().any(|tx| {
                tx.inputs.iter().any(|i| i.outpoint == *outpoint)
            });
            if is_spent_in_mempool {
                continue;
            }

            total_accumulated += utxo.value;
            selected.push((outpoint.clone(), utxo.clone()));

            if total_accumulated >= amount {
                break;
            }
        }

        if total_accumulated < amount {
            return Err("Insufficient funds".into());
        }

        // -------------------------------------------------------
        // 3) Construir tx.inputs
        // -------------------------------------------------------
        let inputs: Vec<TxInput> = selected.iter().map(|(op, _)| {
            TxInput {
                outpoint: op.clone(),
                pub_key: Vec::new(),    // será preenchido por sign_inputs
                signature: Vec::new(),  // será preenchido por sign_inputs
            }
        }).collect();

        // -------------------------------------------------------
        // 4) Construir tx.outputs
        // -------------------------------------------------------
        let mut outputs = Vec::new();

        // output para destinatário
        outputs.push(TxOutput {
            value: amount,
            pub_key_hash: Transaction::pub_key_hash(&to_pub_key_bytes),
        });

        // troco para remetente
        let change = total_accumulated - amount;
        if change > 0 {
            outputs.push(TxOutput {
                value: change,
                pub_key_hash: sender_pub_key_hash.clone(),
            });
        }

        let mut tx = Transaction::new(inputs, outputs);

        tx.sign_input(sender_secret);

        if !tx.verify_signatures() {
            return Err("Signature verification failed".into());
        }

        // -------------------------------------------------------
        // 6) Validar contra UTXO set
        // -------------------------------------------------------
        if !tx.validate_tx(utxos, &sender_pub_bytes) {
            return Err("Transaction validation against UTXOSet failed".into());
        }

        // -------------------------------------------------------
        // 7) Adicionar na mempool
        // -------------------------------------------------------
        mempool.add(tx.clone());

        Ok(tx)
    }
}
