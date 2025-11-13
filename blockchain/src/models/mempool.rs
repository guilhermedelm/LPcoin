use serde::{Serialize,Deserialize};
use chrono::prelude::*;
use sha2::{Sha256, Digest};
use super::transaction::Transaction;
use std::collections::HashMap;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Mempool{
    pub transactions: HashMap<String ,Transaction>,
}

impl Mempool{
    pub fn add(&mut self,tx:Transaction) -> bool{
        let max_size = 5000;
        if self.transactions.len() > max_size{
            return false
        }
        //if !tx.is_valid(){
        //    return false          //falta implementar o método is_valid na transaction
        //}
        self.transactions.insert(tx.id.clone(),tx);
        true

    }
}