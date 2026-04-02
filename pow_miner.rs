use crate::block_core::Block;

const DIFFICULTY: usize = 4;

pub fn mine_block(mut block: Block) -> Block {
    let target = "0".repeat(DIFFICULTY);
    loop {
        let hash = block.compute_hash();
        if hash.starts_with(&target) {
            block.hash = hash;
            return block;
        }
        block.nonce += 1;
    }
}
