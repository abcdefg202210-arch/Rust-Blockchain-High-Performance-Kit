use sha2::{Sha256, Digest};
use std::fmt::Write;

#[derive(Debug, Clone)]
pub struct Tx {
    pub txid: String,
    pub from: String,
    pub to: String,
    pub amount: f64,
    pub timestamp: u64,
}

impl Tx {
    pub fn new(from: String, to: String, amount: f64) -> Self {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
        let data = format!("{}{}{}{}", from, to, amount, timestamp);
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        let mut txid = String::new();
        for byte in hasher.finalize() {
            write!(&mut txid, "{:02x}", byte).unwrap();
        }
        Tx { txid, from, to, amount, timestamp }
    }
}
