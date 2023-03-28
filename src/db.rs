use std::io;
use crate::block::Block;

struct DB {
    chain: Vec<Block>,
}

impl DB {
    fn new() -> DB {
        DB {
            chain: vec![]
        }
    }
    fn add_block(&mut self, block: Block) -> Result<&Block, io::Error> {
        // verify block
        // add block to chain
        self.chain.push(block);
        Ok(self.chain.last().unwrap())
    }
}