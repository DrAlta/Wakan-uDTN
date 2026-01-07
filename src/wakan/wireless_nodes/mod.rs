//!
//! * every node is a parent
//! ** is is parent tree with grandchildren that are with a lower id than itself
mod beepy_node;
pub use beepy_node::{BeepyNode, BeepyPacket};
mod distributed_dict_node;
pub use distributed_dict_node::{DistributedDictNode, DistributedDictPacket};
mod flood_node;
pub use flood_node::{FloodNode, FloodPacket};
mod local_minimum_cluster_node;
pub use local_minimum_cluster_node::{LocalMinimumClusterNode, LocalMinimumClusterPacket};
pub mod scoms_tree_node1;
pub mod scoms_tree_node2;
mod wakan_node;
pub use wakan_node::{WakanNode, WakanPacket};
mod zillions_of_trees_node;
pub use zillions_of_trees_node::ZillionsOfTreesNode;
