use crate::wakan::{DistributedDictNode, wireless_nodes::distributed_dict_node::{CROWN_SIZE, NodeAddress}};

impl DistributedDictNode {
    /// Check whether an address falls inside this node's canopy range.
    /// Adjust canopy size to your scheme; here we use 128 addresses as in your sketch.
    pub fn in_canopy(&self, addr: NodeAddress) -> bool {
        let start = self.canopy;
        let end = start.saturating_add(CROWN_SIZE);
        (start <= addr) && (addr < end)
    }

}