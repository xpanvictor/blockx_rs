use std::cell::Ref;
use std::ops::Deref;
use crate::constants;
use chrono::prelude::Utc;
use chrono::Timelike;
use crate::constants::MINUTE_IN_MILLISECOND;
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
pub struct Block {
    pub index: u32,
    pub data: String,
    pub timestamp: i64,
    pub prev_hash: String,
    // nonce for block's proof of work
    pub nonce: u32,
    // non hash data
    pub hash: String,
    pub difficulty: u32,
}

pub struct BlockHashMeta(u32, String);

/// Implementations for block data structure
impl<'a> Block {
    pub fn new(data: &str, block_chain_db: &'a mut DB) -> &'a Block {

        let latest_block = block_chain_db.latest_block().unwrap();
        let index = latest_block.index + 1;
        let timestamp = Utc::now().timestamp_millis();
        // let latest_block = latest_block.deref();
        let prev_hash = latest_block.hash.clone();

        let unresolved_block = UnresolvedBlock {
            index,
            data,
            prev_hash: &latest_block.hash,
            timestamp,
            difficulty: latest_block.difficulty,
        };
        // dynamically generate difficulty
        let difficulty = Utils::generate_difficulty(
            block_chain_db,
            &unresolved_block
        );

        let BlockHashMeta(nonce, hash) = Block::find_block_hash_meta(unresolved_block);

        // push to the chain
        let mined_block = Block {
            index,
            data: String::from(data),
            timestamp,
            hash,
            prev_hash: latest_block.hash.clone(),
            nonce,
            difficulty
        };

        block_chain_db
            .add_block(mined_block)
            .expect("Block not successful mined");

        block_chain_db.latest_block().unwrap()
    }

    pub fn block_merge(nonce: u32, index: u32, data: &str, timestamp: i64, prev_hash: &str) -> String {
        format!("{}{}{}{}{}", nonce, index, data, timestamp, prev_hash)
    }

    fn find_block_hash_meta(block: UnresolvedBlock) -> BlockHashMeta {
        let mut nonce = 0;
        // for metric analysis
        let mut count = 0;
        let start_time = Utc::now().timestamp();

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
                let time_used = Utc::now().timestamp() - start_time;
                println!("Total attempts: {count}, time used: {time_used}s");
                break BlockHashMeta(nonce, block_hash)
            }

            count += 1;

            nonce += 1;
        }

    }

    pub fn validate_block(&self, prev_block: &Block) -> Result<(), &str> {
        let block_merge = Block::block_merge(
            self.nonce,
            self.index,
            &self.data,
            self.timestamp,
            &self.prev_hash
        );
        // is prev hash the prev block's hash

        let timestamp_is_valid = prev_block.timestamp - MINUTE_IN_MILLISECOND < self.timestamp
            &&
            self.timestamp - MINUTE_IN_MILLISECOND < Utc::now().timestamp_millis();

        return if self.prev_hash != prev_block.hash {
            Err("New block doesn't have old block's hash")
        }
        // validate timestamp
        else if !timestamp_is_valid {
            Err("Invalid timestamp") // todo: better error message
        }
        // check if block is one index ahead last block
        else if self.index != prev_block.index + 1 {
            Err("New block's index is malformed")
        }
        // check if block's hash matches it's data
        else if Utils::hash(&block_merge) != self.hash {
            Err("Invalid block, hash doesn't correlate with data")
        }
        // check if block actually scored the difficulty
        else if !Utils::scored_difficulty(&self.hash, self.difficulty) {
            Err("Difficulty was not achieved, block is mischievous!")
        } else {
            Ok(())
        }
    }
}


#[cfg(test)]
mod tests {
    use std::ops::Deref;
    use super::*;

    #[test]
    fn generate_genesis_block() {
        let db = DB::new();
        let genesis_ref = db.get_genesis().unwrap();
        let retrieved_block: &Block = genesis_ref.deref();

        let block_merge = Block::block_merge(
            0,
            0,
            &retrieved_block.data,
            retrieved_block.timestamp,
            &retrieved_block.prev_hash
        );
        let block_hash = Utils::hash(&block_merge);

        assert_eq!(block_hash, constants::GEN_HASH, "Invalid generic hash, hasher faulty.");
        assert_eq!(retrieved_block.index, 0, "Genesis index must be zero");
        assert_eq!(retrieved_block.timestamp, constants::START_TIME);
    }

    #[test]
    fn generate_block() {
        let data = "hello world";
        let mut db = DB::new();

        let prev_block = db.latest_block().unwrap();
        // let new_block = Block::new(data, &mut db);
        //
        // assert_eq!(new_block.index, prev_block.index + 1, "Mined block isn't one index higher than prev");
        // assert_eq!(new_block.prev_hash, prev_block.hash, "Mined block's prev hash isn't prev block's hash!");
        // assert_eq!(new_block.data, data, "Mined block's data malformed");
    }

    #[test]
    fn validate_block() {
        let mut db = DB::new();
        let prev_block = db.latest_block().unwrap();
        let data = "hello world";
        let latest_block = Block::new(data, &mut db);

        // add latest block to chain
        let difficulty = 4;
        let timestamp = latest_block.timestamp + 3000;

        let new_block = UnresolvedBlock {
            index: latest_block.index + 1,
            prev_hash: &latest_block.hash,
            timestamp,
            data,
            difficulty
        };
        let block_hash = Block::find_block_hash_meta(new_block);

        // using a difficulty of 3 to test
        let broadcast_block = Block {
            nonce: block_hash.0,
            difficulty,
            prev_hash: latest_block.hash.clone(),
            hash: block_hash.1,
            timestamp,
            data: String::from(data),
            index: latest_block.index + 1
        };

        // check if add block validates
        let _add_res = db.add_block(broadcast_block);

    }
}