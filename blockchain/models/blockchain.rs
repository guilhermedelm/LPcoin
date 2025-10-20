//importando dependências
use chrono::prelude::*;
use super::block::Block;
//**********
type Blocks = Vec<Block>;
[derive(Debug)]
pub struct Blockchain{
    pub genesis: Block;         //bloco genesis para ter como base
    pub chain: Blocks;          //vetor de blocos
    pub difficulty: u64;        //nonce para determinar dificuldade do PoW

}
impl Blockchain{

    pub fn new(difficulty:u64) -> Self{
        let mut genesis = Block{

        };    

        let mut chain = Vec::new();
        chain.push(genesis.clone());
        //cria instância da blockchain(inicia block chain)
        let blockchain = Blockchain{
            genesis,
            chain,
            difficulty,
        };
        blockchain.chain[0].hash.calculate_hash();      //calcular hash do bloco genesis
        blockchain
    }
    
}