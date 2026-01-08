use super::NodeAddress;

/// Path is the intra-block chain of nodes that keep track of a block.
/// The first element (if any) is the next hop from this node.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Path {
    hops: Vec<NodeAddress>,
}

impl Path {
    pub fn new() -> Self {
        Self { hops: Vec::new() }
    }
    pub fn from_raw(hops: Vec<NodeAddress>) -> Self {
        Path { hops }
    }

    pub fn push(&mut self, addr: NodeAddress) {
        self.hops.push(addr);
    }

    pub fn first_hop(&self) -> Option<NodeAddress> {
        self.hops.first().copied()
    }
}
