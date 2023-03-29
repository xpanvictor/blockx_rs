pub mod Utils {
    use sha256::{digest};

    pub fn hash(values: &str) -> String {
        digest(values)
    }

    pub fn scored_difficulty(hash: &str, difficulty: u32) -> bool {
        todo!()
    }
    
    pub fn generate_difficulty() -> u32 {
        todo!()
    }
}