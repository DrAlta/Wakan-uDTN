mod structs;
pub use structs::{
    range::{find_least_error_among_all, Case, Range},
    Graph, Node, NodeId, Radio, RawNode, Transmission, WakamSim,
};
mod traits;
pub use traits::{Cluster, Frontend, Parent, WirelessNode};
mod wireless_nodes;
pub use wireless_nodes::{
    scoms_tree_node1, scoms_tree_node2, BeepyNode, BeepyPacket, FloodNode, FloodPacket,
    LocalMinimumClusterNode, LocalMinimumClusterPacket, WakanNode, WakanPacket,
    ZillionsOfTreesNode,
};

pub const SIM_SIZE: usize = 25;

pub type Time = u64;

type RecievedTime = Time;
type ScheduledTransmitionTime = Time;

/// This is the signature of the API to Scedual a PAcket for TRansmition
pub fn schedule_transmission<Packet>(
    // time comes fist as it is less stable than packet which could be attemtped to be transmitter at multiple times
    when_to_transmit: ScheduledTransmitionTime,
    packet: Packet,
    transmitter: Radio,
) -> Result<(), Packet> {
    let _ = when_to_transmit;
    let _ = packet;
    let _ = transmitter;
    todo!()
}
