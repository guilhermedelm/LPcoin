

use serde::{Serialize,Deserialize};
use chrono::prelude::*;
use sha2::{Sha256, Digest};
use super::mempool::Mempool;

//lembrar de tornar funções privadas



//"Ponteiros" que apontam para Txoutputs(valores que você recebeu antes) que são usados como saldos na transferência
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct TxInputs{
    id_prev_tx: String,          //id da transação anterior(Txinput que originou ela)
    output_index: u32,   // Índice da saída na transação anterior
    signature: Vec<u8>,  // Assinatura provando propriedade
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct TxOutputs{
    value:u64,                    //valor da transação(em unidades da moeda)
    public_key_hash: String,      //chave pública de quem }

}




#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Transaction{
    pub id: String,                 //id da transação
    pub inputs: Vec<TxInputs>,       //vetor de TxInputs
    pub outputs:Vec<TxOutputs>,      //vetor de TxOutputs
    pub timestamp:DateTime<Utc>,
}

impl Transaction{

    //criar nova transaction
    pub fn new(inputs:Vec<TxInputs>, outputs:Vec<TxOutputs>) -> Self{
        let mut transaction = Transaction{
            id:  String::new(),
            inputs,
            outputs,
            timestamp:Utc::now(),
        };
        transaction.set_id();
        transaction

    }
    pub fn set_id(&mut self) -> String {
        let block_data = self.clone();
        //block_data.hash = String;
        let serialized_block_data = serde_json::to_string(&block_data).unwrap(); //converte dados da block_data para json para poder 

        let mut hasher = Sha256::new();                                          //inicia função de hash Sha256
        hasher.update(serialized_block_data);                                    //atualiza valor da função hash com serialized_block_data
        let result = hasher.finalize();                                          //finaliza função e retorna resultado
        self.id = format!("{:x}", result);
        format!("{:x}", result)
    }
}

//recompensa de trabalho
impl Transaction{
    pub fn coinbase(to:&str) -> Self{
        let value = 100;
        let outputs = vec![TxOutputs{value: value,public_key_hash: to.to_string()}];
        let mut tx = Transaction::new(vec![],outputs);
        tx
    }
}
