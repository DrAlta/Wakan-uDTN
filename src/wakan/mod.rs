mod sim;
pub use sim::WakamSim;
mod transmission;
pub use transmission::Transmission;
mod wireless_node;
pub use wireless_node::WirelessNode;
mod wireless_nodes;
pub use wireless_nodes::{
    BeepyNode, BeepyPacket, FloodNode, FloodPacket, PlumTreeNode, PlumTreePacket, WakanNode,
    WakanPacket,
};

pub type NodeId = u64;
pub type Radio = u8;
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
