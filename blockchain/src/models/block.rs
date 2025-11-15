//importando dempendências 
use chrono::prelude::*;
use serde::{Serialize,Deserialize};
//use super::blockchain::Blockchain;
use sha2::{Sha256, Digest};

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


//teste de mineração para ver se blockchainfunciona
impl Block{
}