mod block;
mod constants;
mod db;
mod utils;
mod transaction;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    
}
