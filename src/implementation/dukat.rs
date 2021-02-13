use chrono::prelude::*;
use std::fmt::{self, Debug, Formatter};

use crate::implementation::transaction::Transaction;
use crate::utils::hashable::{u128_bytes, Hashable};

pub struct Dukat {
    pub index: u128,
    pub hash: Vec<u8>,
    pub time: DateTime<Utc>,
    pub transactions: Vec<Transaction>,
    pub prev_hash: Vec<u8>,
    pub payload: String,
}

impl Dukat {
    pub fn new(
        index: u128,
        transactions: Vec<Transaction>,
        prev_hash: Vec<u8>,
        payload: String,
    ) -> Self {
        Dukat {
            index,
            transactions,
            time: Utc::now(),
            hash: vec![0; 32],
            prev_hash,
            payload,
        }
    }
    pub fn _mine_block() {}
}

impl Hashable for Dukat {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend(&u128_bytes(&self.index));
        bytes.extend(&self.time.timestamp().to_be_bytes());
        bytes.extend(self.payload.as_bytes());

        bytes
    }
}

impl Debug for Dukat {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            r#"Block[{}]: {},
            timestamp: {},
            payload: {}"#,
            &self.index,
            hex::encode(&self.hash),
            &self.time,
            &self.payload
        )
    }
}
