//! this does simple flooding, rebroadcasting any packet it hasn't already received
//! node with id 1 wil broadcast a new packet when now is 0.
mod flood_node;
pub use flood_node::FloodNode;
mod flood_packet;
pub use flood_packet::FloodPacket;
type PacketId = u8;
