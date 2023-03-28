use std::io;
use crate::constants;
use crate::block::Block;

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
}

// pub const std_db: DB = DB::new();
