use std::collections::{BTreeMap, BTreeSet};

use crate::wakan::{wireless_nodes::zillions_of_trees_node::structs::{HighestAccessible, LowestAccessible}, NodeId};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ZillionsOfTreesPacket{
    Beacon {
        source: NodeId,
        neighbors: BTreeMap<NodeId, BTreeSet<NodeId>>,
//        neighbors: BTreeMap<NodeId, (LowestAccessible, HighestAccessible)>,
    },
}