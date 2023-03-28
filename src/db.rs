use std::io;
use crate::constants;
use crate::block::Block;
use utils::Utils;
use crate::utils::Utils;

pub struct DB<'a> {
    chain: Vec<Block<'a>>,
}

impl DB<'static> {
    pub fn new() -> DB<'static> {
        DB {
            chain: vec![
                Block {
                    data: String::from("Genesis"),
                    index: 0,
                    timestamp: constants::START_TIME,
                    hash: String::from(constants::GEN_HASH),
                    prev_hash: ""
                }
            ]
        }
    }

    pub fn add_block(&mut self, block: Block<'static>) -> Result<&Block, io::Error> {
        // verify block
        block
            .validate_block(self.latest_block().unwrap())
            .unwrap_or_else(|err|{
                panic!("Operational: {}", err)
            });
        // add block to chain
        self.chain.push(block);
        Ok(self.chain.last().unwrap())
    }

    pub fn latest_block(&self) -> Result<&Block, io::Error> {
        Ok(self.chain.last().unwrap())
    }

    pub fn get_genesis(&self) -> Result<&Block, io::Error> {
        Ok(self.chain.first().unwrap())
    }

    pub fn validate_chain(&self, cross_db: DB) -> Result<(), &str> {
        let genesis_block = self.chain.first().unwrap();
        let cross_genesis = cross_db.chain.first().unwrap();
        let cross_block_merge = Block::block_merge(
            cross_genesis.index,
            &cross_genesis.data,
            cross_genesis.timestamp,
            cross_genesis.prev_hash
        );

        return if genesis_block.hash != cross_genesis.hash {
            Err("Genesis block not the same")
        }else if Utils::hash(&cross_block_merge) != cross_genesis.hash{
            Err("Genesis block's data doesnt correlate with hash")
        } else {
            Ok(())
        }
    }
}

// pub const std_db: DB = DB::new();
