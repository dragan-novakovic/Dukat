use crate::models::dukat::Dukat;
use crate::models::transaction::Transaction;
pub struct DukatChain {
    pub chain: Vec<Dukat>,
    pub pending_transactions: Vec<Transaction>,
    // difficulty
    // minerReward
    // blockSize
    // self.nodes = set() ?
}
