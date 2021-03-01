mod implementation;
mod utils;

use implementation::dukat::Dukat;
use implementation::dukatchain::DukatChain;
use implementation::transaction::Transaction;
use std::convert::TryInto;

use wasm_bindgen::prelude::*;

// import from web

#[wasm_bindgen]
extern "C" {
    fn alert(s: String);
}

// export to js
#[wasm_bindgen]
pub fn greet(name: String) {
    unsafe {
        alert(format!("Hello, {}!", name));
    }
}

#[wasm_bindgen]
pub fn init() -> DukatChain {
    let mut blockchain = DukatChain::new();

    let first_transaction = Transaction::new("Player1".to_owned(), "Player2".to_owned(), 100);
    let second_transaction = Transaction::new("Player2".to_owned(), "Player3".to_owned(), 150);

    // create new block with Transactions
    let mut first_block = Dukat::new(
        (blockchain.chain.len() as i32).try_into().unwrap(),
        vec![first_transaction, second_transaction],
        blockchain.chain.last().unwrap().hash.clone(),
        "First Interaction".to_owned(),
        Dukat::default_difficulty(),
    );

    // Mine it
    first_block.mine_block();

    // add it to blockchain
    blockchain.add_block(first_block);

    dbg!(blockchain);

    blockchain
}
