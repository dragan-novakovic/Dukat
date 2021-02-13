use chrono::prelude::*;

pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: i64,
    pub time: DateTime<Utc>,
    pub hash: Vec<u8>,
}

impl Transaction {
    pub fn new(sender: String, receiver: String, amount: i64) -> Self {
        Transaction {
            sender,
            receiver,
            amount,
            time: Utc::now(),
            hash: vec![0; 32],
        }
    }
    pub fn _sign_transactions() {}
}
