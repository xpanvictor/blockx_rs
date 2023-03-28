
use crate::constants;
use chrono::prelude::Utc;

/// This is the block main file
/// It contains the block DS and implementations

/// A block structure
#[derive(Debug)]
pub struct Block {
    pub index: u32,
    // todo: get a better data
    pub data: String,
    pub timestamp: i64,
    pub hash: String,
    pub prev_hash: String
}

/// Implementations for block data structure
impl Block {
    pub fn new(data: &str) -> Block {
        Block {
            // generate index from db
            index: 1,
            data: String::from(data),
            timestamp: Utc::now().timestamp_millis(),
            hash: String::from("hey"),
            prev_hash: String:: from("hi")
        }
    }

    pub fn retrieve_genesis() -> Block {
        Block {
            data: String::from("Genesis"),
            index: 0,
            timestamp: constants::START_TIME,
            hash: String::from(constants::GEN_HASH),
            prev_hash: String::new()
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_genesis_block() {
        let retrieved_block = Block::retrieve_genesis();
        assert_eq!(retrieved_block.hash, constants::GEN_HASH);
        assert_eq!(retrieved_block.index, 0);
        assert_eq!(retrieved_block.timestamp, constants::START_TIME);
    }
}