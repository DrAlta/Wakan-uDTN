use std::collections::BTreeMap;

use crate::wakan::{wireless_nodes::zillions_of_trees_node::structs::NeighborInfo, NodeId};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ZillionsOfTreesNode{
    pub id: NodeId,
    pub neighbors:BTreeMap<NodeId, NeighborInfo>,
}