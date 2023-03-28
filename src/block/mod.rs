
use crate::constants;
use chrono::prelude::Utc;
use crate::db::{DB};

/// This is the block main file
/// It contains the block DS and implementations

/// A block structure
#[derive(Debug)]
pub struct Block<'a> {
    pub index: u32,
    // todo: get a better data
    pub data: String,
    pub timestamp: i64,
    pub hash: String,
    pub prev_hash: &'a str
}

/// Implementations for block data structure
impl<'b> Block<'b> {
    pub fn new<'a>(data: &str, latest_block: &'a Block) -> Block<'a> {
        Block {
            // generate index from db
            index: latest_block.index + 1,
            data: String::from(data),
            timestamp: Utc::now().timestamp_millis(),
            hash: String::from("hey"),
            prev_hash: &latest_block.hash
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_genesis_block() {
        let db = DB::new();
        let retrieved_block: &Block = db.get_genesis().unwrap();
        assert_eq!(retrieved_block.hash, constants::GEN_HASH);
        assert_eq!(retrieved_block.index, 0);
        assert_eq!(retrieved_block.timestamp, constants::START_TIME);
    }

    #[test]
    fn generate_block() {
        let data = "hello world";
        let db = DB::new();
        let prev_block = db.latest_block().unwrap();
        let new_block = Block::new(data, &prev_block);

        assert_eq!(new_block.index, prev_block.index + 1);
        assert_eq!(new_block.prev_hash, prev_block.hash);
        assert_eq!(new_block.data, data);
    }
}