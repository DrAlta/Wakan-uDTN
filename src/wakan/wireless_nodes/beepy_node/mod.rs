//! this node just sends out a beep when ever it's received node's isn't empty or 
//! if it has id 1 and the now is 0
mod beepy_node;
pub use beepy_node::BeepyNode;
mod beepy_packet;
pub use beepy_packet::BeepyPacket;
