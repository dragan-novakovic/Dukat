struct DukatChain {
    chain: Vec<Dukat>,
}

struct Dukat {
    index: i32,
    hash: i32,
    time: i32,
    transactions: Vec<Transaction>,
    prevHash: Option<i32>,
}

struct Transaction {
    sender: String,
    reciever: String,
    amount: i64,
    time: i32,
    hash: i32,
}

fn main() {
    println!("Hello, world!");
}
