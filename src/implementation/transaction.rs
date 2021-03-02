use crate::utils::hashable::{u64_bytes, Hashable};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Debug, Formatter};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
    pub time: i64,
    pub hash: Vec<u8>,
}

impl Transaction {
    pub fn new(sender: String, receiver: String, amount: u64) -> Self {
        Transaction {
            sender,
            receiver,
            amount,
            time: Utc::timestamp(Utc::now()),
            hash: vec![0; 32],
        }
    }
    pub fn _sign_transactions(&mut self, _key: String, _sender_key: String) {
        // TO-DO
    }
}

impl Hashable for Transaction {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend(self.sender.as_bytes());
        bytes.extend(self.receiver.as_bytes());
        bytes.extend(&u64_bytes(&self.amount));
        bytes.extend(&self.time.timestamp().to_be_bytes());

        bytes
    }
}

impl Debug for Transaction {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            r#"Tx:,
            sender: {},
            receiver: {},
            amount: {},
            timestamp: {}
            "#,
            &self.sender, &self.receiver, &self.amount, &self.time
        )
    }
}
