use chrono::prelude::*;
use std::fmt::{self, Debug, Formatter};

use crate::implementation::transaction::Transaction;
use crate::utils::hashable::{difficulty_bytes_as_u128, u128_bytes, u64_bytes, Hashable};

pub struct Dukat {
    pub index: u128,
    pub hash: Vec<u8>,
    pub time: DateTime<Utc>,
    pub transactions: Vec<Transaction>,
    pub prev_hash: Vec<u8>,
    pub payload: String,
    pub difficulty: u128,
    pub nonce: u64,
}

impl Dukat {
    pub fn new(
        index: u128,
        transactions: Vec<Transaction>,
        prev_hash: Vec<u8>,
        payload: String,
        difficulty: u128,
    ) -> Self {
        let mut block = Dukat {
            index,
            transactions,
            time: Utc::now(),
            hash: vec![0; 32],
            prev_hash,
            payload,
            difficulty,
            nonce: 0,
        };

        block.hash = Dukat::hash(&block);
        block
    }
    pub fn mine_block(&mut self) {
        for nonce_attempt in 0..(u64::max_value()) {
            self.nonce = nonce_attempt;
            let hash = self.hash();
            println!("Trying {}", nonce_attempt);
            if Dukat::check_difficulty(&hash, self.difficulty) {
                self.hash = hash;
                return;
            }
        }
    }

    pub fn check_difficulty(hash: &Vec<u8>, difficulty: u128) -> bool {
        println!(
            "difficulty {} and {}",
            difficulty,
            difficulty_bytes_as_u128(&hash),
        );
        difficulty > difficulty_bytes_as_u128(&hash)
    }
}

impl Hashable for Dukat {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend(&u128_bytes(&self.index));
        bytes.extend(&self.time.timestamp().to_be_bytes());
        bytes.extend(self.payload.as_bytes());
        bytes.extend(&u64_bytes(&self.nonce));
        bytes.extend(&u128_bytes(&self.difficulty));

        bytes
    }
}

impl Debug for Dukat {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            r#"Block[{}]: {},
            timestamp: {},
            nonce: {},
            payload: {}
            "#,
            &self.index,
            hex::encode(&self.hash),
            &self.time,
            &self.nonce,
            &self.payload
        )
    }
}
