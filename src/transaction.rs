pub struct TxOut {
    pub address: String,
    pub amount: u64,
}

pub struct TxIn {

}

impl TxOut {
    pub fn new(address: &str, amount: u64) -> TxOut {
        TxOut {
            address: String::from(address),
            amount
        }
    }
}
