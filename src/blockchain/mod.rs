mod transaction;
mod block;

use transaction::Transaction;
use block::Block;

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub current_transactions: Vec<Transaction>,
    pub nodes: Vec<String>,
}

// TODO - imp Blockchain and genesis 