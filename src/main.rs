mod implementation;
mod utils;

use implementation::dukatchain::DukatChain;

fn main() {
    println!("Hello, world!");
    let blockchain = DukatChain::new();

    dbg!(blockchain);
}
