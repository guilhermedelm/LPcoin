mod models;

use models::{Blockchain, Transaction, Mempool, Wallet, TxOutput, TxInput};
use std::io::{self, Write};

fn main() {
    println!("🚀 Iniciando LPCOIN Node v1.1 (Correção de Saldo)...");

    let mut lp_coin = Blockchain::new(2);
    let mut mempool = Mempool::new();
    let miner_wallet = Wallet::create_wallet();
    let miner_addr = hex::encode(miner_wallet.public_key.serialize());
    let ladeira_wallet = Wallet::create_wallet();
    let ladeira_addr = hex::encode(ladeira_wallet.public_key.serialize());

    println!("✅ Sistema Iniciado!");
    println!("🔑 Minha Carteira: {}", miner_addr);
    println!("🔑 Carteira Ladeira: {}", ladeira_addr);
    
    loop {
        println!("\n================ MENU LPCOIN ==================");
        println!("1. ⛏️  Minerar Bloco (Processar Mempool)");
        println!("2. 💰 Consultar Saldos (UTXO)");
        println!("3. 💸 Enviar Transação (Eu -> Ladeira)");
        println!("4. 📦 Ver Mempool (Pendentes)");
        println!("0. ❌ Sair");
        print!("Escolha: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Erro ao ler");
        
        match input.trim() {
            "1" => {
                println!("⛏️  Minerando...");
                let new_block = lp_coin.mine_from_mempool(miner_addr.clone(), &mut mempool);
                match new_block {
                    Some(block) => {
                        println!("✅ Bloco #{} minerado! Hash: {}", block.index, block.hash);
                    },
                    None => println!("⚠️ Nenhum bloco minerado."),
                }
            },
            "2" => {

                let my_balance = lp_coin.utxo.calculate_balance(miner_addr.as_bytes());
                let ladeira_balance = lp_coin.utxo.calculate_balance(ladeira_addr.as_bytes());
                
                println!("\n💰 SALDOS ATUAIS:");
                println!("   Eu (Minerador): {} LPC", my_balance);
                println!("   Ladeira:          {} LPC", ladeira_balance);
            },
            "3" => {
                println!("\n💸 Configurando transação...");
                print!("Digite o valor para enviar ao Amigo: ");
                io::stdout().flush().unwrap();
                let mut amount_str = String::new();
                io::stdin().read_line(&mut amount_str).expect("Erro ao ler valor");

                let amount_to_send: u64 = amount_str.trim().parse().unwrap_or(0);

                if amount_to_send == 0 {
                    println!("❌ Valor inválido! Digite um número maior que zero.");
                    continue;
                }


                let my_pub_key_bytes = miner_wallet.public_key.serialize(); 

                let (acc, valid_outputs) = lp_coin.utxo.find_spendable_outputs(miner_addr.as_bytes(), amount_to_send);

                if acc < amount_to_send {
                    println!("❌ Erro: Saldo insuficiente!");
                    println!("   Você tem: {} LPC", acc);
                    println!("   Tentou enviar: {} LPC", amount_to_send);
                } else {
                    let mut inputs = Vec::new();
                    for (outpoint, _output) in valid_outputs {
                        let input = TxInput {
                            outpoint,
                            pub_key: my_pub_key_bytes.to_vec(),
                            signature: Vec::new(),
                        };
                        inputs.push(input);
                    }

                    let mut outputs = Vec::new();

                    outputs.push(TxOutput {
                        value: amount_to_send,
                        pub_key_hash: ladeira_addr.as_bytes().to_vec(),
                    });

                    if acc > amount_to_send {
                        outputs.push(TxOutput {
                            value: acc - amount_to_send,
                            pub_key_hash: miner_addr.as_bytes().to_vec(),
                        });
                    }
                    let mut tx = Transaction::new(inputs, outputs);
                    tx.sign(&miner_wallet.private_key);
                    
                    if mempool.add(tx.clone()) {
                        println!("✅ Transação de {} LPC enviada para Mempool!", amount_to_send);
                        println!("   ID: {:?}", hex::encode(tx.tx_id()));
                        println!("   (Minere um bloco para confirmar)");
                    } else {
                         println!("❌ Erro ao adicionar na Mempool.");
                    }
                }
            },
            "4" => {
                 println!("\n📦 Mempool:");
                 for (id, _) in &mempool.transactions {
                     println!("   TxID: {}", hex::encode(id));
                 }
            }
            "0" => break,
            _ => println!("Opção inválida"),
        }
    }
}