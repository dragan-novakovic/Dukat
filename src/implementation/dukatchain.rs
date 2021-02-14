// use chrono::prelude::*;
use std::fmt::{self, Debug, Formatter};

use crate::implementation::dukat::Dukat;
use crate::implementation::transaction::Transaction;
use crate::utils::hashable::Hashable;

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

        let genesis_block = Dukat::new(0, vec![], vec![0; 32], "GENESIS".to_owned(), 0, 0);
        self.chain.push(genesis_block)
    }

    pub fn add_block(block: Dukat) {
        //last hash

        //verify
    }

    pub fn verify(&self) -> bool {
        for (i, block) in self.chain.iter().enumerate() {
            if block.index != i as u128 {
                println!("Index mismatch {} != {}", &block.index, &i);
                return false;
            } else if !Dukat::check_difficulty(&block.hash(), block.difficulty) {
                println!("Difficulty Fail");
                return false;
            } else if i == 0 {
                // not genesis
                let prev_block = &self.chain[i - 1];
                if block.time <= prev_block.time {
                    println!("Time Failed");
                    return false;
                } else if block.prev_hash != prev_block.prev_hash {
                    println!("Hash mismatch");
                    return false;
                }
            } else {
                //genesis
                if block.prev_hash != vec![0; 32] {}
                println!("Gensis block invalid");
                return false;
            }
        }

        true
    }

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
