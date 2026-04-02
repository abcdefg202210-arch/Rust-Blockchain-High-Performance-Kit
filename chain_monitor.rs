use crate::block_core::Block;

pub fn print_chain_info(blocks: &[Block]) {
    println!("=== Chain Monitor ===");
    println!("Height: {}", blocks.len());
    if let Some(last) = blocks.last() {
        println!("Latest Hash: {}", last.hash);
    }
}

pub fn has_fork(a: &[Block], b: &[Block]) -> bool {
    let min = a.len().min(b.len());
    for i in 0..min {
        if a[i].hash != b[i].hash { return true; }
    }
    false
}
