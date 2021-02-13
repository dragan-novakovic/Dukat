// use chrono::prelude::*;
use std::fmt::{self, Debug, Formatter};

use crate::implementation::dukat::Dukat;
use crate::implementation::transaction::Transaction;

pub struct DukatChain {
    pub chain: Vec<Dukat>,
    pub pending_transactions: Vec<Transaction>,
}

impl DukatChain {
    pub fn new() -> Self {
        let mut chain = DukatChain {
            chain: vec![],
            pending_transactions: vec![],
        };

        DukatChain::add_genesis_block(&mut chain);

        chain
    }

    fn add_genesis_block(&mut self) {
        let mut transactions: Vec<Transaction> = vec![];

        let genesis_transaction = Transaction::new("Root".to_owned(), "First".to_owned(), 1);
        transactions.push(genesis_transaction);

        let genesis_block = Dukat::new(0, vec![], vec![0; 32], "GENESIS".to_owned());
        self.chain.push(genesis_block)
    }

    pub fn _add_block() {}
    pub fn _add_pending_transaction() {}

    // register node
    // resolve conficts
    // minePending
    // addGenesisBlock
    // isValid Chain
    // genereate KEys
    // chainJSON encode
}

impl Debug for DukatChain {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "BlockChain:\n {:#?}", &self.chain)
    }
}
