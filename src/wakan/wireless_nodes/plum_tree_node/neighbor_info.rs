use std::collections::BTreeSet;

use crate::wakan::{NodeId, Time};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NeighborInfo {
    pub last_seen: Time,
    pub neighbors_of_neighbor: BTreeSet<NodeId>,
}
#[allow(dead_code)]
impl NeighborInfo {
    pub fn simple_new(last_seen: Time) -> Self {
        Self {
            last_seen,
            neighbors_of_neighbor: BTreeSet::new(),
        }
    }
    pub fn new(last_seen: Time, neighbors_of_neighbor: BTreeSet<u64>) -> Self {
        Self {
            last_seen,
            neighbors_of_neighbor,
        }
    }
}
