use sha2::{Sha256, Digest};
use std::fmt::Write;

#[derive(Debug, Clone)]
pub struct Block {
    pub height: u64,
    pub prev_hash: String,
    pub timestamp: u64,
    pub tx_root: String,
    pub nonce: u64,
    pub hash: String,
}

impl Block {
    pub fn compute_hash(&self) -> String {
        let input = format!(
            "{}{}{}{}{}",
            self.height, self.prev_hash, self.timestamp, self.tx_root, self.nonce
        );
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        let result = hasher.finalize();
        let mut s = String::new();
        for byte in result {
            write!(&mut s, "{:02x}", byte).unwrap();
        }
        s
    }
}
