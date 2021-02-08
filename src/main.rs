struct DukatChain {
    chain: Vec<Dukat>,
    pending_transactions: Vec<Transaction>,
    // difficulty
    // minerReward
    // blockSize
    // self.nodes = set() ?
}

impl DukatChain {
    fn add_block() {}
    fn get_last_block() {}
    fn add_transaction() {}
    // register node
    // resolve conficts
    // minePending
    // addGenesisBlock
    // isValid Chain
    // genereate KEys
    // chainJSON encode
}

struct Dukat {
    index: i32,
    hash: i32,
    time: i32,
    transactions: Vec<Transaction>,
    prev_hash: Option<i32>,
}

impl Dukat {
    fn calculate_hash() {}
    fn mine_block() {}
}

struct Transaction {
    sender: String,
    reciever: String,
    amount: i64,
    time: i32,
    hash: i32,
}

impl Transaction {
    fn sign_transactions() {}
}

fn main() {
    println!("Hello, world!");
}
