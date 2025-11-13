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
    //Função para calcular hash
    pub fn calculate_hash(&self) -> String {
        let block_data = self.clone();
        //block_data.hash = String;
        let serialized_block_data = serde_json::to_string(&block_data).unwrap(); //converte dados da block_data para json para poder 

        let mut hasher = Sha256::new();                                          //inicia função de hash Sha256
        hasher.update(serialized_block_data);                                    //atualiza valor da função hash com serialized_block_data
        let result = hasher.finalize();                                          //finaliza função e retorna resultado
        format!("{:x}", result)
    }
}


//teste de mineração para ver se blockchainfunciona
impl Block{

    pub fn mine(&mut self,difficulty:u64) -> (&Block, u64, String) {
        loop{
            //apenas contador para ver quantos números o nonce já incrementou
            if self.nonce % 1000000000 == 0{
                println!("bilhão")
            }
            if self.hash.chars().take_while(|&c| c == '0').count() >= difficulty.try_into().unwrap(){
                return (self,self.nonce,self.hash.clone());
            }
            else{
                self.nonce += 1;

            }
        }
    }
}