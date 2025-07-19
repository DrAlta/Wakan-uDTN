use std::collections::{BTreeMap, BTreeSet};

use crate::wakan::{wireless_nodes::zillions_of_trees_node::structs::NeighborInfo, NodeId, Time};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ZillionsOfTreesNode{
    // node consts
    pub id: NodeId,

    // tree
    pub queen: NodeId,
    pub princess: NodeId,
    pub neighbors: BTreeMap<NodeId, NeighborInfo>,
    pub tree_neighbors:  BTreeSet<NodeId>,
    pub parent_maybe: Option<NodeId>,

    // bookkeeping
    pub next_beacon: Time,
}
