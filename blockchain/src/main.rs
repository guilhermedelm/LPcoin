mod models;
//use chrono::prelude::*;
use models::{Block,Blockchain,Transaction,TxInputs,TxOutputs,Mempool,Wallet};
use std::io;         

fn main() {
    let mut lp_coin =Blockchain::new(0);
    print!("Blockchain criada com bloco gênesis");
    let mut my_wallet = Wallet::create_wallet();
    print!("carteira criada");
    let mine = Blockchain::mine(&mut lp_coin,"abc".to_string());
    println!("{:#?}",mine);
    
    let mut action = String::new();
    println!("1-Minerar bloco\n2-Criar transação\n3-Sair");
    io::stdin()
        .read_line(&mut action)
        .expect("Failed to read line");

}
