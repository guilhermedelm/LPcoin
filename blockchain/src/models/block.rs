//importando dempendências 
use chrono::prelude::*;
use serde::{Serialize,Deserialize};
use sha2::{Sha256, Digest};
use super::transaction::Transaction;
use super::mempool::Mempool;

//**********

#[derive(Serialize,Deserialize, Debug, Clone)]

//Criando a struct de cada bloco
pub struct Block{
    pub index: u64,
    pub timestamp: DateTime<Utc>,
    pub data: String,
    pub prev_hash: String,
    pub nonce: u64,
    pub hash: String,
    pub miner_key: String,
    pub transactions: Vec<Transaction>,
}

//Funções de Block

impl Block{
    pub fn calculate_hash(&self) -> String{
        let mut hasher = Sha256::new();
        hasher.update(self.index.to_string());
        hasher.update(self.timestamp.to_rfc3339());
        hasher.update(&self.data);
        hasher.update(&self.prev_hash);
        hasher.update(self.nonce.to_string());
        let result = hasher.finalize();
        format!("{:x}", result)
    }
}


impl Block {
    // Integração mempool-bloco: pega transações da mempool
    pub fn from_mempool(mempool: &mut Mempool, max_tx: usize) -> Vec<Transaction> {
        let transactions = mempool.select_for_block(max_tx);
        let tx_ids: Vec<[u8; 32]> = transactions.iter().map(|tx| tx.tx_id()).collect();
        mempool.remove_batch(&tx_ids);
        transactions
    }
}