pub mod Utils {
    use sha256::{digest};

    pub fn hash(values: &str) -> String {
        digest(values)
    }
}