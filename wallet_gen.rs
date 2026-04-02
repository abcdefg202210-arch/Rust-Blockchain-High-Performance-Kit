use rand::Rng;
use std::fmt::Write;

pub fn new_address() -> String {
    let mut bytes = [0u8; 20];
    rand::thread_rng().fill(&mut bytes);
    let mut s = String::from("0x");
    for b in bytes { write!(&mut s, "{:02x}", b).unwrap(); }
    s
}
