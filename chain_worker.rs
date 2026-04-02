use crate::block_core::Block;
use std::time::SystemTime;

pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis = Block {
            height: 0,
            prev_hash: "0".to_string(),
            timestamp: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs(),
            tx_root: "genesis_root".to_string(),
            nonce: 0,
            hash: "genesis_hash".to_string(),
        };
        Blockchain { blocks: vec![genesis] }
    }

    pub fn last_block(&self) -> &Block {
        self.blocks.last().unwrap()
    }
}
