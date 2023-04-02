use std::error::Error;
use std::fmt::format;
use crate::constants;
use chrono::prelude::Utc;
use crate::db::{DB};
use crate::utils::Utils;

/// This is the block main file
/// It contains the block DS and implementations

/// A block structure
// ? Can I try to implement this as inheritance
pub struct UnresolvedBlock<'a> {
    pub index: u32,
    data: &'a str,
    prev_hash: &'a str,
    timestamp: i64,
    pub difficulty: u32,
}
#[derive(Debug)]
pub struct Block<'a> {
    pub index: u32,
    pub data: String,
    pub timestamp: i64,
    pub prev_hash: &'a str,
    // nonce for block's proof of work
    pub nonce: u32,
    // non hash data
    pub hash: String,
    pub difficulty: u32,
}

pub struct BlockHashMeta(u32, String);

/// Implementations for block data structure
impl<'b> Block<'b> {
    pub fn new<'a>(data: &str, block_chain: &'a DB) -> Block<'a> {

        let latest_block = block_chain.latest_block().unwrap();
        let index = latest_block.index + 1;
        let timestamp = Utc::now().timestamp_millis();
        let prev_hash = &latest_block.hash;

        let unresolved_block = UnresolvedBlock {
            index,
            data,
            prev_hash,
            timestamp,
            difficulty: latest_block.difficulty,
        };
        // dynamically generate difficulty
        let difficulty = Utils::generate_difficulty(block_chain, &unresolved_block);

        let BlockHashMeta(nonce, hash) = Block::find_block_hash_meta(unresolved_block);

        Block {
            index,
            data: String::from(data),
            timestamp,
            hash,
            prev_hash,
            nonce,
            difficulty
        }
    }

    pub fn block_merge(nonce: u32, index: u32, data: &str, timestamp: i64, prev_hash: &str) -> String {
        format!("{}{}{}{}{}", nonce, index, data, timestamp, prev_hash)
    }

    fn find_block_hash_meta(block: UnresolvedBlock) -> BlockHashMeta {
        let mut nonce = 0;
        let mut count = 0;

        loop {
            let block_merge = Block::block_merge(
                nonce,
                block.index,
                &block.data,
                block.timestamp,
                block.prev_hash
            );
            let block_hash = Utils::hash(&block_merge);

            if Utils::scored_difficulty(&block_hash, block.difficulty) {
                break BlockHashMeta(nonce, block_hash)
            }

            count += 1;
            println!("Attempt {}", count);

            nonce += 1;
        }
    }

    pub fn validate_block(&self, prev_block: &Block) -> Result<(), &str> {
        let block_merge = Block::block_merge(
            self.nonce,
            self.index,
            &self.data,
            self.timestamp,
            self.prev_hash
        );
        // is prev hash the prev block's hash

        let timestamp_is_valid = prev_block.timestamp - 60 < self.timestamp
            &&
            self.timestamp - 60 < Utc::now().timestamp_millis();

        return if self.prev_hash != prev_block.hash {
            Err("New block doesn't have old block's hash")
        } // validate timestamp
        else if !timestamp_is_valid {
            Err("Invalid timestamp") // todo: better error message
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

        let block_merge = Block::block_merge(
            0,
            0,
            &retrieved_block.data,
            retrieved_block.timestamp,
            retrieved_block.prev_hash
        );
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
        let new_block = Block::new(data, &db);

        assert_eq!(new_block.index, prev_block.index + 1, "Mined block isn't one index higher than prev");
        assert_eq!(new_block.prev_hash, prev_block.hash, "Mined block's prev hash isn't prev block's hash!");
        assert_eq!(new_block.data, data, "Mined block's data malformed");
    }
}