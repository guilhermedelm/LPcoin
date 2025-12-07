use serde::{Serialize,Deserialize};
//use chrono::prelude::*;
//use sha2::{Sha256, Digest};
use super::transaction::Transaction;
use std::collections::HashMap;


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

    pub fn add(&mut self, tx: Transaction) -> bool{
        let tx_id = tx.tx_id();

        if self.transactions.len() >= self.max_size{ //mempool cheia
            return false;
        }

        if self.contains(tx_id){
            return false;
        }

        for existing in self.transactions.values() {
            for inp in &existing.inputs {
                for new_inp in &tx.inputs {
                    if inp.outpoint == new_inp.outpoint {
                        // conflito: outpoint já gasto por tx na mempool
                        return false;
                    }
                }
            }
        }

        self.transactions.insert(tx_id, tx);
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