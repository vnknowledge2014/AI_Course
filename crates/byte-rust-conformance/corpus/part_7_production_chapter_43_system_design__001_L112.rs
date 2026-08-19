// filename: src/main.rs

use std::collections::BTreeMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

struct ConsistentHash {
    ring: BTreeMap<u64, String>,
    virtual_nodes: u32,
}

impl ConsistentHash {
    fn new(virtual_nodes: u32) -> Self {
        ConsistentHash { ring: BTreeMap::new(), virtual_nodes }
    }

    fn add_node(&mut self, node: &str) {
        for i in 0..self.virtual_nodes {
            let key = hash_key(&format!("{}:{}", node, i));
            self.ring.insert(key, node.into());
        }
    }

    fn remove_node(&mut self, node: &str) {
        for i in 0..self.virtual_nodes {
            let key = hash_key(&format!("{}:{}", node, i));
            self.ring.remove(&key);
        }
    }

    fn get_node(&self, key: &str) -> Option<&String> {
        if self.ring.is_empty() { return None; }
        let hash = hash_key(key);
        // Find next node clockwise on the ring
        self.ring.range(hash..).next()
            .or_else(|| self.ring.iter().next()) // wrap around
            .map(|(_, node)| node)
    }
}

fn hash_key(key: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);
    hasher.finish()
}

fn main() {
    let mut ring = ConsistentHash::new(100); // 100 virtual nodes per server

    ring.add_node("server-1");
    ring.add_node("server-2");
    ring.add_node("server-3");

    // Route requests
    let keys = ["user:100", "user:200", "user:300", "order:1", "order:2"];
    println!("=== 3 servers ===");
    for key in &keys {
        println!("  {} → {}", key, ring.get_node(key).unwrap());
    }

    // Add server (minimal disruption!)
    ring.add_node("server-4");
    println!("\n=== 4 servers (added server-4) ===");
    for key in &keys {
        println!("  {} → {}", key, ring.get_node(key).unwrap());
    }
    // Most keys stay on same server!
}
