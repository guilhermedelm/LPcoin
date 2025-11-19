mod models;
//use chrono::prelude::*;
use models::{Block,Blockchain,Transaction,TxInputs,TxOutputs,Mempool,Wallet};           

fn main() {
    let mut lp_coin =Blockchain::new(0);
    print!("Blockchain criada com bloco gênesis");
    let mut my_wallet = Wallet::create_wallet();
    print!("carteira criada");
    let mine = Blockchain::mine(&mut lp_coin,"abc".to_string());
    println!("{:#?}",mine);
    
    

}
