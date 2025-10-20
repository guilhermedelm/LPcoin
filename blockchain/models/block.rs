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
    //Função para calcular hash
    pub fn calculate_hash(&self) -> String {
        let mut block_data = self.copy();
        block_data.hash = String;
        let serialized_block_data = serde_json::to_string(&block_data).unwrap(); //converte dados da block_data para json para poder 

        let mut hasher = Sha256::new();                                          //inicia função de hash Sha256
        hasher.update(serialized_block_data);                                    //atualiza valor da função hash com serialized_block_data
        let result = hasher.finalize();                                          //finaliza função e retorna resultado
        format("{:x}" result)
    }
}
