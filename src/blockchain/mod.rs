pub struct Blockchain {
    pub chain: Vec<Block>,
    pub current_transactions: Vec<Transaction>,
    pub nodes: Vec<String>,
}

// TODO - imp Blockchain and genesis 