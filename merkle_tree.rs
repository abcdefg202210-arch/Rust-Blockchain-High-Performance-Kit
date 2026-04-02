use sha2::{Sha256, Digest};
use std::fmt::Write;

pub fn merkle_root(mut hashes: Vec<String>) -> String {
    if hashes.is_empty() {
        return "0".repeat(64);
    }
    while hashes.len() > 1 {
        let mut next = Vec::new();
        for i in (0..hashes.len()).step_by(2) {
            let left = &hashes[i];
            let right = if i+1 < hashes.len() { &hashes[i+1] } else { left };
            let mut h = Sha256::new();
            h.update(left.as_bytes());
            h.update(right.as_bytes());
            let mut s = String::new();
            for b in h.finalize() { write!(&mut s, "{:02x}", b).unwrap(); }
            next.push(s);
        }
        hashes = next;
    }
    hashes[0].clone()
}
