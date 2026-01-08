//! this node just sends out a beep when ever it's received node's isn't empty or
//! if it has id 1 and the now is 0
mod virtual_vote;
pub use virtual_vote::HandlePacketReturn;
pub use virtual_vote::VirtualVote;
mod virtual_vote_node;
pub use virtual_vote_node::VirtualVoteNode;
mod virtual_vote_packet;
pub use virtual_vote_packet::VirtualVotePacket;

mod frontend;

const MIN_NEEDED_TO_DROP: usize = 4;
