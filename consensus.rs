use crate::block_core::Block;
use crate::chain_validator;

pub fn resolve_conflicts(chains: Vec<Vec<Block>>) -> Vec<Block> {
    let mut best = chains[0].clone();
    for chain in chains {
        if chain.len() > best.len() && chain_validator::is_chain_valid(&chain) {
            best = chain;
        }
    }
    best
}
