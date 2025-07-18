mod neighbor_info;
use std::collections::BTreeSet;

pub use neighbor_info::NeighborInfo;
mod wireless_node;
mod zillions_of_trees_node;
pub use zillions_of_trees_node::ZillionsOfTreesNode;
mod zillions_of_trees_packet;
pub use zillions_of_trees_packet::ZillionsOfTreesPacket;

use crate::wakan::NodeId;

type LowestAccessible = BTreeSet<NodeId>;
type HighestAccessible = BTreeSet<NodeId>;