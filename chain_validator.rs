use crate::block_core::Block;

pub fn is_chain_valid(blocks: &[Block]) -> bool {
    for i in 1..blocks.len() {
        let curr = &blocks[i];
        let prev = &blocks[i-1];
        if curr.hash != curr.compute_hash() { return false; }
        if curr.prev_hash != prev.hash { return false; }
    }
    true
}
