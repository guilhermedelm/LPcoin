//importando dependências
use chrono::prelude::*;
use super::block::Block;
use super::transaction::Transaction;
use crate::models::Mempool;

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
            miner_key: vec![],
            transactions: Vec::new(),
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
    pub fn add_block(&mut self,new_block:Block) -> bool{
        if self.validate_block(&new_block){
            self.chain.push(new_block.clone());
            let tx_coinbase = Transaction::coinbase(new_block.miner_key.clone());
            return true
        };
        return false
        
    }

    //função para validar requisitos de um bloco 

    pub fn validate_block(&mut self,block_check:&Block) -> bool{
        if let Some(last_block) = self.chain.last(){
            if block_check.index != last_block.index + 1{
                print!("{}",block_check.index);
                print!("{}",last_block.index);
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

impl Blockchain{
 
    pub fn mine(&mut self, public_key:Vec<u8>) -> &Block {
        let id = self.chain.last().unwrap().index.clone() +1;
        let data = "00".to_string();//integrar com Mempool
        let pre_hash = self.chain.last().unwrap().hash.clone();

        let coinbase_tx = Transaction::coinbase(public_key.clone());
        let txs = vec![coinbase_tx];
        let data = serde_json::to_string(&txs).unwrap();

        let mut candidate = Block{
            index: id ,
            timestamp: Utc::now(),
            data:data,
            prev_hash: pre_hash,
            nonce: 0,
            hash: String::new(),
            miner_key: public_key,
            transactions: Vec::new(),
        };
        candidate.hash = candidate.calculate_hash();
    
        loop{
            if candidate.hash.chars().take_while(|&c| c == '0').count() >= self.difficulty.try_into().unwrap(){
               if self.add_block(candidate.clone()) == true{
                break
               }
               else{
                println!("invalid block");
                break 
               }
            }
            else{
                candidate.nonce += 1;
                }
        }
        
        //validate_block(&self,candidate)
        return self.chain.last().unwrap();
    }
}

impl Blockchain{
   pub fn mine_from_mempool(&mut self, miner_pub_key: Vec<u8>, mempool:&mut Mempool) -> &Block {
        let id = self.chain.last().unwrap().index.clone() +1;
        let prev_hash = self.chain.last().unwrap().hash.clone();
        let mut txs: Vec<Transaction> = mempool.transactions.values().cloned().collect();
        let coinbase_tx = Transaction::coinbase(miner_pub_key.clone());
        txs.push(coinbase_tx);
        let data = serde_json::to_string(&txs).unwrap();

        let mut candidate = Block{
            index: id ,
            timestamp: Utc::now(),
            data,
            prev_hash,
            nonce: 0,
            hash: String::new(),
            miner_key: miner_pub_key.clone(),
            transactions: txs.clone(),
        };

        loop {
            candidate.hash = candidate.calculate_hash();

            if candidate.hash.chars().take(self.difficulty as usize).all(|c| c == '0') {
                if self.add_block(candidate.clone()) {
                    for tx in txs.iter() {
                        mempool.remove(tx.tx_id());
                    }
                    break;
                } else {
                    println!("Invalid block after PoW");
                    break;
                }
            } else {
                candidate.nonce += 1;
            }
        }

        self.chain.last().unwrap()
    }
}