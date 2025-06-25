use std::fmt;

use crate::wakan::{wireless_nodes::flood_node::PacketId, NodeId};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FloodPacket(pub Vec<NodeId>, pub PacketId);

impl FloodPacket{
    pub fn new(source:NodeId, packet_id: u8) -> Self {
        Self(vec![source], packet_id)
    }
    pub fn push(&mut self, node_id:NodeId) {
        self.0.push(node_id)
    }
}

impl fmt::Display for FloodPacket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{Packet: {}, Path:{:?}}})", self.1, self.0,)
    }
}
