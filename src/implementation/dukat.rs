use crate::models::dukat::Dukat;
use chrono::prelude::*;
use sha2::{Digest, Sha256};

impl Dukat {
    pub fn calculate_hash(
        time: DateTime<Utc>,
        hash_transaction: String,
        prev_hash: String,
        nonce: String,
    ) -> String {
        let mut hasher = Sha256::new();

        let hash_string = format!(
            "{}{}{}{}",
            time.to_rfc2822(),
            hash_transaction,
            prev_hash,
            nonce
        );

        // write input message
        // hasher.update(b"hello world");

        // // read hash digest and consume hasher
        // let result = hasher.finalize();
        String::from("IDK")
    }
    fn mine_block() {}
}
