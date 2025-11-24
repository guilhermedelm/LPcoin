use transaction::TxOutputs;

pub struct UTXO{
    pub map: HashMap<String,Vec<TxOutputs>>,
}