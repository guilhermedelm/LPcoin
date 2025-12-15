use serde::{Serialize,Deserialize};
//use chrono::prelude::*;
//use sha2::{Sha256, Digest};
use super::transaction::Transaction;
use std::collections::HashMap;
use crate::models::utxo::Outpoint;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Mempool{
    pub transactions: HashMap<[u8; 32], Transaction>,
    pub max_size: usize,
}

impl Mempool{
    pub fn new() -> Self{
        Mempool { 
            transactions: HashMap::new(),
            max_size: 5000 
        }
    }

    fn is_outpoint_already_spent(&self, outpoint: &Outpoint) -> bool {
        self.transactions.values().any(|existing_tx| {
            existing_tx.inputs.iter().any(|input| {
                input.outpoint == *outpoint
            })
        })
    }

    pub fn add(&mut self, tx: Transaction) -> bool {
        for input in &tx.inputs {
            if self.is_outpoint_already_spent(&input.outpoint) {
                println!("❌ Double-spend detectado na mempool!");
                return false;
            }
        }

        let txid = tx.tx_id();
        self.transactions.insert(txid, tx);
        true
    }

    pub fn get(&self, tx_id: [u8; 32]) -> Option<&Transaction> {
        self.transactions.get(&tx_id)
    }

    pub fn select_for_block(&self, count: usize) -> Vec<Transaction> {
        self.transactions.values().take(count).cloned().collect()
    }

    pub fn remove_batch(&mut self, tx_ids: &[[u8; 32]]) {
        for tx_id in tx_ids {
            self.transactions.remove(tx_id);
        }
    }

    pub fn remove(&mut self, tx_id: [u8;32]){
        self.transactions.remove(&tx_id);
    }

    pub fn contains(&self, tx_id: [u8; 32]) -> bool{
        self.transactions.contains_key(&tx_id)
    }

    pub fn all(&self) -> Vec<Transaction>{
        self.transactions.values().cloned().collect()
    }
}