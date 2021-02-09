use chrono::prelude::*;

pub struct Transaction {
    pub sender: String,
    pub reciever: String,
    pub amount: i64,
    pub time: DateTime<Utc>,
    pub hash: String,
}
