mod models;
use chrono::prelude::*;
use models::{Block,Blockchain};           

fn main() {
    let lp_coin =Blockchain::new(0);
    println!("{:#?}", lp_coin.chain);
    
    println!("{:#?} ", Block {
        index: lp_coin.chain.last().unwrap().index + 1,
        timestamp: Utc::now(),
        data: "0".to_string(),
        prev_hash: lp_coin.chain.last().unwrap().hash.clone(),
        nonce: 0,
        hash: String::new(),
    }.mine(1));
}
