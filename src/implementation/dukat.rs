use crate::implementation::transaction::Transaction;
use crate::utils::hashable::{difficulty_bytes_as_u128, u128_bytes, u64_bytes, Hashable};
use chrono::prelude::*;
use std::fmt::{self, Debug, Formatter};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Dukat {
    pub index: u64,
    pub hash: Vec<u8>,
    pub time: i64,
    pub transactions: Vec<Transaction>,
    pub prev_hash: Vec<u8>,
    pub payload: String,
    pub difficulty: u64,
    pub nonce: u64,
}

impl Dukat {
    pub fn new(
        index: u64,
        transactions: Vec<Transaction>,
        prev_hash: Vec<u8>,
        payload: String,
        difficulty: u64,
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

    pub fn default_difficulty() -> u64 {
        0x000fffffffffffffffffffffffffffff
    }

    pub fn check_difficulty(hash: &Vec<u8>, difficulty: u64) -> bool {
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
            transactions: {:?}
            "#,
            &self.index,
            hex::encode(&self.hash),
            &self.time,
            &self.nonce,
            &self.payload,
            &self.transactions
        )
    }
}
