use crate::block_core::Block;

pub fn avg_block_time(blocks: &[Block]) -> u64 {
    if blocks.len() <= 1 { return 0; }
    let mut total = 0;
    for i in 1..blocks.len() {
        total += blocks[i].timestamp - blocks[i-1].timestamp;
    }
    total / (blocks.len() as u64 - 1)
}
