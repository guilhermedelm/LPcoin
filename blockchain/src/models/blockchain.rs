//importando dependências
use chrono::prelude::*;
use super::block::Block;
use super::transaction::Transaction;

//**********
type Blocks = Vec<Block>;
#[derive(Debug)]
pub struct Blockchain{
    pub genesis: Block,         //bloco genesis para ter como base
    pub chain: Blocks,          //vetor de blocos
    pub difficulty: u64,        //nonce para determinar dificuldade do PoW

}
impl Blockchain{
    //método para iniciar blockchain
    pub fn new(difficulty:u64) -> Self{
        let mut genesis = Block{
            index: 0,
            timestamp: Utc::now(),
            data:"0".to_string(),
            prev_hash: String::new(),
            nonce: 0,
            hash: String::new(),
            miner_key: "0".to_string(),

        };   

        let mut chain = Vec::new();

        genesis.hash = genesis.calculate_hash();

        chain.push(genesis.clone());
        //cria instância da blockchain(inicia block chain)
        let  blockchain = Blockchain{
            genesis,
            chain,
            difficulty,
        };    
        blockchain
    }
    
}


impl Blockchain{
    //função para adicionar bloco à blockchain
    pub fn add_block(&mut self,new_block:Block) {
       if self.validate_block(&new_block){
        self.chain.push(new_block.clone());
        let tx_coinbase = Transaction::coinbase(&new_block.miner_key.clone());
        

        
        }
    }

    //função para validar requisitos de um bloco 

    pub fn validate_block(&mut self,block_check:&Block) -> bool{
        if let Some(last_block) = self.chain.last(){
            if block_check.index != last_block.index + 1{
                println!("invalid index");
                false
            }
            
            else if block_check.prev_hash != last_block.hash{
                println!("invalid prev_hash");
                false
            }
            else if last_block.timestamp >= block_check.timestamp{
                println!("invalid timestamp");
                false
            }
            else{
                println!("passou");
                true
            }
            
        }
        else {
            println!("bloco vazio");
            false
        }
    }


        
}
