pub mod Utils {
    use sha256::{digest};
    use crate::block::UnresolvedBlock;
    use crate::constants::BLOCK_GENERATION_INTERVAL;

    pub fn hash(values: &str) -> String {
        digest(values)
    }

    pub fn scored_difficulty(hash: &str, difficulty: u32) -> bool {
        todo!()
    }

    pub fn generate_difficulty(block: &UnresolvedBlock) -> u32 {
        // check if current block should be checked
        if block.index % BLOCK_GENERATION_INTERVAL == 0 && block.index != 0{
            // ! need access to past few gen_interval blocks
            todo!()
        }
        block.difficulty
    }
}