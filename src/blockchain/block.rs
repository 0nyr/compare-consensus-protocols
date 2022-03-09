use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Block {
    pub index: u32,
    pub timestamp: u64,
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
}

impl Block {
    pub fn new(
        index: u32, 
        timestamp: Utc, 
        data: String, 
        previous_hash: String
    ) -> Block {
        let mut block = Block {
            index: index,
            timestamp: timestamp,
            data: data,
            previous_hash: previous_hash,
            hash: String::new(),
        };
        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let mut sha = Sha256::new();
        sha.input_str(&self.index.to_string() + &self.timestamp.to_string() + &self.data + &self.previous_hash);
        sha.result_str()
    }

    pub fn genesis(node_address: String) -> Self {
        // reward for mining a block
        let reward_transaction = Transaction::new(
            "0".to_string(),
            node_address.clone(),
            100
        ); 

        // add data to the block (text & reward transaction)
        let mut data = String::new();
        data.append("(Genesis block)");
        data.append(reward_transaction.to_string());

        // create the genesis block
        let mut genesis_block = Block::new(
            0, 
            Utc::now(), 
            data, 
            "0".to_string()
        );
        genesis_block.hash = genesis_block.calculate_hash();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_new() {
        let block = Block::new(0, 0, "data".to_string(), "previous_hash".to_string());
        assert_eq!(block.index, 0);
        assert_eq!(block.timestamp, 0);
        assert_eq!(block.data, "data");
        assert_eq!(block.previous_hash, "previous_hash");
        assert_eq!(block.hash, "cf057bbfb72640471fd910bcb67639c22df9f92470936cddc1ade0e2f2e7dc4f");
    }

    #[test]
    fn test_block_calculate_hash() {
        let block = Block::new(0, 0, "data".to_string(), "previous_hash".to_string());
        assert_eq!(block.calculate_hash(), "cf057bbfb72640471fd910bcb67639c22df9f92470936cddc1ade0e2");
}