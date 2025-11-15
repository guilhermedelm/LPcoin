mod models;
//use chrono::prelude::*;
use models::{Block,Blockchain,Transaction,TxInputs,TxOutputs,Mempool,Wallet};           

fn main() {
    let lp_coin =Blockchain::new(0);
    printl!("Blockchain criada com bloco gênesis");
    let mut my_wallet = Wallet::create_wallet();
    

}
