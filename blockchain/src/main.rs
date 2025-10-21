mod models;

use models::{Blockchain};           //tirar aviso chato se for usar colocar Block , 

fn main() {
    let lp_coin =Blockchain::new(0);
    println!("{:#?}", lp_coin.chain);
    
}
