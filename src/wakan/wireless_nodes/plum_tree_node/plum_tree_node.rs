use std::{collections::BTreeMap, hash::Hash};

use crate::wakan::{wireless_nodes::plum_tree_node::NeighborInfo, NodeId, Time};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PlumTreeNode {
    pub(super) id: NodeId,
    pub(super) next_beacon: Time,
    pub(super) neighbors: BTreeMap<NodeId, NeighborInfo>,
    pub(super) parent: Option<NodeId>,
}

impl PlumTreeNode {
    pub fn find_oldest_neighbor(&self) -> Option<&NodeId> {
        let x = self
            .neighbors
            .iter()
            .filter_map(|(neighbor_id, neighbor_info)| {
                Some((neighbor_id, neighbor_info.find_oldest_time()?))
            })
            .min_by(|(_, a), (_, b)| a.cmp(b))?;
        Some(x.0)
    }
}
