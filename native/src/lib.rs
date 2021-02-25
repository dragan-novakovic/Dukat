mod implementation;
mod utils;

use implementation::dukat::Dukat;
use implementation::dukatchain::DukatChain;
use implementation::transaction::Transaction;
use neon::prelude::*;
use std::convert::TryInto;

declare_types! {
    /// JS class wrapping Employee records.
    pub class JsDukatChain for DukatChain {
        init(mut cx) {
            Ok(DukatChain::new())
        }
    }
}
// Export the class
register_module!(mut m, {
    // <JsEmployee> tells neon what class we are exporting
    // "Employee" is the name of the export that the class is exported as
    m.export_class::<JsDukatChain>("DukatChain")?;
    Ok(())
});

/*



fn main() {

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
