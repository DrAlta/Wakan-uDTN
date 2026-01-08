/*
I had the idea for a distributed dictionary for the routing on Wakan. basically the address space is divides in 128 chucks that are kept track of by a node, then those node are groups into 128 that a node keeps track ok to each node is keeping tracks of 128 * number_of_level. for reason the lowest level is 256 root nodes then 128 node for each level above that it's 8 levels so it's only 1028 nodes that each node has to keep track off then it forwards the packet to the highs level node tht it knows of their cover the range of the address space that the address the packet it addressed to is in.

nodes have a max_hop variable and they rebroadcast announcement that have traveled less than that number of hops. Nodes can broadcast as request for announcements that requested max_hops decreases with each forward when a node receives one of these, it checks if its current max hop is less than the requested one and if sets it's mac_hop to the requested and transmits the request to its neighbors. their are mechanism so that the node's max hops decreases so that if it doesn't keep getting requests it'll go back to the default.

the announcement rebroadcast thing is because a node might to far way from a node that keeps tack of a block.

I'm still working on the intralevel routing. nodes should keep tack of nodes that keep track of the same blocks as it does

my idea was that is doesn't need to keep track of the full path to the nodes in the block just the path along nodes that keep track of that block to the closests one of those to the node which would keep track of the path from itself to the node

this has the advantage as a node only needs minimum amount of memory to participate but if is does have more memory available to it is have use that additional memory to improve the routing by keeping track of more blocks

*/

//! mini version 4*4*4*4= 256
//!
mod block;
pub use block::Block;
mod distributed_dict;
pub use distributed_dict::DistributedDict;
mod distributed_dict_node;
pub use distributed_dict_node::DistributedDictNode;
mod distributed_dict_packet;
pub use distributed_dict_packet::DistributedDictPacket;
mod hop_limit_update;
pub use hop_limit_update::HopLimitUpdate;
mod path;
pub use path::Path;
// public
pub type NodeAddress = u64;

// private
type StartOfAddressRange = u64;

const DEFAULT_HOP_LIMIT: u8 = 2;
const HLU_SMOOTHING_NUMERATOR: u64 = 1;
const HLU_SMOOTHING_DENOMINATOR: u64 = 10;

const CROWN_SIZE: u64 = 128;
const ROOT_SIZE: usize = 256;
const LEVELS: usize = 7;
const LEVEL_BUCKETS: usize = 128;
