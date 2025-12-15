use chrono::prelude::*;
use super::block::Block;
use super::transaction::Transaction;
use super::utxo::UTXOSet;
use crate::models::Mempool;

type Blocks = Vec<Block>;

#[derive(Debug)]
pub struct Blockchain {
    pub genesis: Block,
    pub chain: Blocks,
    pub difficulty: u64,
    pub utxo: UTXOSet,
}

impl Blockchain {
    pub fn new(difficulty: u64) -> Self {
        let mut genesis = Block {
            index: 0,
            timestamp: Utc::now(),
            data: "Genesis Block".to_string(),
            prev_hash: String::from("0"),
            nonce: 0,
            hash: String::new(),
            miner_key: String::from("genesis"),
            transactions: Vec::new(),
        };

        let tx_genesis = Transaction::coinbase("genesis_wallet");
        genesis.transactions.push(tx_genesis);
        
        genesis.hash = genesis.calculate_hash();

        let mut chain = Vec::new();
        chain.push(genesis.clone());

        let mut utxo = UTXOSet::new();
        utxo.reindex(&chain);

        Blockchain {
            genesis,
            chain,
            difficulty,
            utxo,
        }
    }

    pub fn add_block(&mut self, new_block: Block) -> bool {
        if !self.validate_block(&new_block) {
            return false;
        }

        // snapshot temporário
        let mut temp_utxo = self.utxo.clone();

        for tx in &new_block.transactions {
            if !tx.validate_against_utxo(&temp_utxo) {
                println!("❌ Transação inválida economicamente");
                return false;
            }

            temp_utxo.update(tx);
        }

        // commit final
        self.utxo = temp_utxo;
        self.chain.push(new_block);

        true
    }

    pub fn validate_block(&self, block_check: &Block) -> bool {
        let last_block = self.chain.last().unwrap();

        if block_check.index != last_block.index + 1 {
            println!("❌ Erro: Index inválido");
            return false;
        } 
        if block_check.prev_hash != last_block.hash {
            println!("❌ Erro: Hash anterior não bate");
            return false;
        } 
        if block_check.hash != block_check.calculate_hash() {
            println!("❌ Erro: Hash do bloco inválido");
            return false;
        }

        for (i, tx) in block_check.transactions.iter().enumerate() {
            if !tx.verify() {
                println!("❌ Erro: Transação #{} tem assinatura inválida", i);
                return false;
            }
        }

        true
    }

   pub fn mine_from_mempool(
        &mut self,
        miner_address: String,
        mempool: &mut Mempool
    ) -> Option<Block> {

        let real_txs = mempool.select_for_block(20);

        let id = self.chain.last().unwrap().index + 1;
        let prev_hash = self.chain.last().unwrap().hash.clone();

        let mut txs = real_txs.clone();

        // ✅ UMA coinbase por bloco
        let coinbase_tx = Transaction::coinbase(&miner_address);
        txs.insert(0, coinbase_tx);

        let mut candidate = Block {
            index: id,
            timestamp: Utc::now(),
            data: "Mempool Block".to_string(),
            prev_hash,
            nonce: 0,
            hash: String::new(),
            miner_key: miner_address.clone(),
            transactions: txs.clone(),
        };

        println!("⛏️  Iniciando mineração do bloco #{}...", id);

        loop {
            candidate.hash = candidate.calculate_hash();
            let target = "0".repeat(self.difficulty as usize);

            if candidate.hash.starts_with(&target) {

                if self.add_block(candidate.clone()) {

                    let tx_ids: Vec<[u8; 32]> = real_txs
                        .iter()
                        .map(|tx| tx.tx_id())
                        .collect();

                    mempool.remove_batch(&tx_ids);

                    return Some(candidate);
                } else {
                    println!("❌ Falha ao adicionar bloco minerado.");
                    return None;
                }
            }

            candidate.nonce += 1;
        }
    }
}