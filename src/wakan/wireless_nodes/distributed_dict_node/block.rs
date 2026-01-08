#![allow(dead_code)]
use std::collections::BTreeMap;

use super::NodeAddress;
use super::Path;

/// A Block holds the list of nodes that "keep track" of that block.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Block {
    owners: Vec<NodeAddress>,
}

impl Block {
    pub fn add_owner(&mut self, addr: NodeAddress) {
        if !self.owners.contains(&addr) {
            self.owners.push(addr);
        }
    }

    /// Return a Path to any owner that we already know about in `tracked_nodes`.
    /// We try owners in order and return the first Path we find in `tracked_nodes`.
    pub fn get_path_to_a_owner(&self, tracked_nodes: &BTreeMap<NodeAddress, Path>) -> Option<Path> {
        for &owner in &self.owners {
            if let Some(path) = tracked_nodes.get(&owner) {
                return Some(path.clone());
            }
        }
        None
    }

    /// Helper: pick the owner closest by XOR to a target address (no path).
    pub fn best_owner_by_xor(&self, dest: NodeAddress) -> Option<NodeAddress> {
        self.owners.iter().copied().min_by_key(|&a| a ^ dest)
    }
}
