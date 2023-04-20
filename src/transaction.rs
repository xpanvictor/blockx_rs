pub struct TxOut {
    pub address: String,
    pub amount: u64,
}

pub struct TxIn {
    pub txId: String,
    pub txIndex: u32,
    pub amount: u64
}

impl TxOut {
    pub fn new(address: &str, amount: u64) -> TxOut {
        TxOut {
            address: String::from(address),
            amount
        }
    }
}

pub struct Transaction {
    pub id: String,
    pub txIns: Vec<TxIn>,
    pub txOuts: Vec<TxOut>
}

impl Transaction {
    fn get_transaction_id(&self) -> String {
        let mut id = self.id.clone();
        // sum up data in txIns
        let txIn_iter = self.txIns.iter();
        // lol, must you use iters, :)`
        id = txIn_iter.map(|txIn|
            format!("{}{}{}", txIn.txId, txIn.txIndex, txIn.amount)
        )
    }
}
