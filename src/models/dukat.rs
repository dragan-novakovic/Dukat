use crate::models::transaction::Transaction;
use chrono::prelude::*;
pub struct Dukat {
    pub index: i128,
    pub hash: String,
    pub time: DateTime<Utc>,
    pub transactions: Vec<Transaction>,
    pub prev_hash: Option<String>,
}
