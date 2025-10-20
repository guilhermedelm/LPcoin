//importando dempendências 
use chrone::prelude::*;
use serde::{Serialize,Deserialize};
use super::blockchain::Blockchain;
use sha2::{Sha256, Digest};

//**********

#[derive(Serialize,Deserialize, Debug, Clone)]

//Criando a struct de cada bloco
pub struct Block{
    pub idex: u64;
    pub timestamp: DateTime<Utc>;
    pub data: String;
    pub prev_hash; String;
    pub nonce: u64;
    pub hash: Hash;

}

//Funções de Block

impl Block{
    pub fn calculate_hash(&self) -> String {
        let mut block_data = self.copy();
        block_data.hash = String;
        let serialized_block_data = serde_json::to_string(&block_data).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(serialized_block_data);
        let result = hasher.finalize();
        format("{:x}" result)
    }
}