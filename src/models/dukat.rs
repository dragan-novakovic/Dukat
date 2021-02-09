use crate::models::transaction::Transaction;
use chrono::prelude::*;
pub struct Dukat {
    pub index: i32,
    pub hash: i32,
    pub time: DateTime<Utc>,
    pub transactions: Vec<Transaction>,
    pub prev_hash: Option<i32>,
}
