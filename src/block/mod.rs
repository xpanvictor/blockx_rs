use std::error::Error;
use std::fmt::format;
use crate::constants;
use chrono::prelude::Utc;
use crate::db::{DB};
use crate::utils::Utils;

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

        let index = latest_block.index + 1;
        let timestamp = Utc::now().timestamp_millis();
        let prev_hash = &latest_block.hash;

        let block_merge = Block::block_merge(index, data, timestamp, prev_hash);
        let block_hash = Utils::hash(&block_merge);

        Block {
            index,
            data: String::from(data),
            timestamp,
            hash: block_hash,
            prev_hash: &latest_block.hash
        }
    }

    pub fn block_merge(index: u32, data: &str, timestamp: i64, prev_hash: &str) -> String {
        format!("{}{}{}{}", index, data, timestamp, prev_hash)
    }

    pub fn validate_block(&self, prev_block: &Block) -> Result<(), &str> {
        let block_merge = Block::block_merge(self.index, &self.data, self.timestamp, self.prev_hash);
        // is prev hash the prev block's hash
        return if self.prev_hash != prev_block.hash {
            Err("New block doesn't have old block's hash")
        } else if self.index != prev_block.index + 1 {
            Err("New block's index is malformed")
        } else if Utils::hash(&block_merge) != self.hash {
            Err("Invalid block, hash doesn't correlate with data")
        } else {
            Ok(())
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

        let block_merge = Block::block_merge(0, &retrieved_block.data, retrieved_block.timestamp, retrieved_block.prev_hash);
        let block_hash = Utils::hash(&block_merge);

        assert_eq!(block_hash, constants::GEN_HASH, "Invalid generic hash, hasher faulty.");
        assert_eq!(retrieved_block.index, 0, "Genesis index must be zero");
        assert_eq!(retrieved_block.timestamp, constants::START_TIME);
    }

    #[test]
    fn generate_block() {
        let data = "hello world";
        let db = DB::new();
        let prev_block = db.latest_block().unwrap();
        let new_block = Block::new(data, &prev_block);

        assert_eq!(new_block.index, prev_block.index + 1, "Mined block isn't one index higher than prev");
        assert_eq!(new_block.prev_hash, prev_block.hash, "Mined block's prev hash isn't prev block's hash!");
        assert_eq!(new_block.data, data, "Mined block's data malformed");
    }
}