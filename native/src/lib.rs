mod implementation;
mod utils;

use implementation::dukat::Dukat;
use implementation::dukatchain::DukatChain;
use implementation::transaction::Transaction;
use neon::prelude::*;
use std::convert::TryInto;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

register_module!(mut cx, { cx.export_function("hello", hello) });

/*



fn main() {


    let mut blockchain = DukatChain::new();
    // get seed?

    let first_transaction = Transaction::new("Player1".to_owned(), "Player2".to_owned(), 100);
    let second_transaction = Transaction::new("Player2".to_owned(), "Player3".to_owned(), 150);

    // create new block with Transactions
    let mut first_block = Dukat::new(
        (blockchain.chain.len() as i32).try_into().unwrap(),
        vec![first_transaction, second_transaction],
        blockchain.chain.last().unwrap().hash.clone(),
        "First Interaction".to_owned(),
        difficulty,
    );

    // Mine it
    first_block.mine_block();

    // add it to blockchain
    blockchain.add_block(first_block);

    dbg!(blockchain);
}




*/
