use std::collections::HashSet;

pub struct Node {
    pub id: String,
    peers: HashSet<String>,
    block_cache: HashSet<String>,
}

impl Node {
    pub fn new(id: &str) -> Self {
        Node {
            id: id.to_string(),
            peers: HashSet::new(),
            block_cache: HashSet::new(),
        }
    }

    pub fn connect(&mut self, peer: &str) {
        self.peers.insert(peer.to_string());
    }

    pub fn broadcast(&self, hash: &str) {
        for p in &self.peers {
            println!("Node {} broadcast to {}: {}", self.id, p, hash);
        }
    }
}
