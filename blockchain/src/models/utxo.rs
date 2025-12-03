use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use super::transaction::Transaction;
use super::block::Block;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Outpoint {
    pub tx_id: [u8; 32],
    pub index: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UTXOSet {
    pub utxos: HashMap<Outpoint, crate::models::transaction::TxOutput>,
}

impl UTXOSet {
    pub fn new() -> Self {
        UTXOSet {
            utxos: HashMap::new(),
        }
    }

    pub fn reindex(&mut self, chain: &Vec<Block>) {
        self.utxos.clear();
        for block in chain {
            for tx in &block.transactions {
                self.update(tx);
            }
        }
    }

    pub fn update(&mut self, tx: &Transaction) {
        if !tx.is_coinbase() {
            for input in &tx.inputs {
                self.utxos.remove(&input.outpoint);
            }
        }

        for (index, output) in tx.outputs.iter().enumerate() {
            let outpoint = Outpoint {
                tx_id: tx.tx_id(),
                index: index as u32,
            };
            self.utxos.insert(outpoint, output.clone());
        }
    }

    pub fn calculate_balance(&self, pub_key_hash: &[u8]) -> u64 {
        let mut balance = 0;
        for output in self.utxos.values() {
            if output.pub_key_hash == pub_key_hash {
                balance += output.value;
            }
        }
        balance
    }

    pub fn find_spendable_outputs(
        &self, 
        pub_key_hash: &[u8], 
        amount: u64
    ) -> (u64, HashMap<Outpoint, crate::models::transaction::TxOutput>) {
        let mut accumulated = 0;
        let mut unspent_outputs = HashMap::new();

        for (outpoint, output) in &self.utxos {
            if output.pub_key_hash == pub_key_hash {
                accumulated += output.value;
                unspent_outputs.insert(outpoint.clone(), output.clone());

                if accumulated >= amount {
                    break;
                }
            }
        }
        (accumulated, unspent_outputs)
    }
}