pub mod Utils {
    use std::ops::Index;
    use sha256::{digest};
    use crate::block::UnresolvedBlock;
    use crate::constants::{BLOCK_GENERATION_INTERVAL, DIFFICULTY_ADJUSTMENT_INTERVAL};
    use crate::db::DB;

    pub fn hash(values: &str) -> String {
        digest(values)
    }

    pub fn scored_difficulty(hash: &str, difficulty: u32) -> bool {
        let difficulty_string = "0".repeat(difficulty as usize);
        hash.starts_with(&difficulty_string)
    }

    pub fn generate_difficulty(
        block_db: &DB,
        unresolved_block: &UnresolvedBlock
    ) -> u32 {

        let current_difficulty = unresolved_block.difficulty;

        // check if current block should be checked
        if unresolved_block.index > DIFFICULTY_ADJUSTMENT_INTERVAL &&
           unresolved_block.index % DIFFICULTY_ADJUSTMENT_INTERVAL == 0 &&
           unresolved_block.index != 0
        {
            // ! need access to past few gen_interval blocks
            let block_chain = &block_db.chain;
            let block_len = block_chain.len();
            let prev_adjusted_block = block_chain.index(
                block_len - (DIFFICULTY_ADJUSTMENT_INTERVAL as usize) - 1
            );
            let last_mined_block = block_db.latest_block().unwrap();

            let time_spent = last_mined_block.timestamp - prev_adjusted_block.timestamp;
            let time_expected = DIFFICULTY_ADJUSTMENT_INTERVAL as i64 * BLOCK_GENERATION_INTERVAL as i64;

            // adjust based on difference in half range of time expected
            return if time_spent > time_expected / 2 {
                current_difficulty - 1
            } else if time_spent < time_expected / 2 {
                current_difficulty + 1
            } else {
                current_difficulty
            }
        }
        current_difficulty
    }
}