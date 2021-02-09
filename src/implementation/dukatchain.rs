use chrono::prelude::*;

use crate::models::dukat::Dukat;
use crate::models::dukatchain::DukatChain;
use crate::models::transaction::Transaction;

impl DukatChain {
    fn add_block() {}
    fn get_last_block() {}
    fn add_transaction() {}
    fn add_genesis_block(&mut self) {
        let mut transactions: Vec<Transaction> = vec![];

        let genesis_transaction = Transaction {
            sender: String::from("Root"),
            reciever: String::from("Zero"),
            amount: 10,
            hash: 111,
            time: Utc::now(),
        };

        transactions.push(genesis_transaction);

        let genesis_block = Dukat {
            index: 0,
            hash: 11,
            time: Utc::now(),
            transactions,
            prev_hash: None,
        };

        self.chain.push(genesis_block)
    }
    // register node
    // resolve conficts
    // minePending
    // addGenesisBlock
    // isValid Chain
    // genereate KEys
    // chainJSON encode
}
