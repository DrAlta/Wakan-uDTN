use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet},
    hash::Hash,
};

use crate::wakan::{NodeId, Time};

use super::super::NeighborInfo;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ScomsTreeNode {
    pub(in super::super::super) id: NodeId,
    pub(in super::super::super) next_beacon: Time,
    pub(in super::super::super) neighbors: BTreeMap<NodeId, NeighborInfo>,
    pub(in super::super::super) parent_maybe: Option<NodeId>,
    pub(in super::super::super) children: BTreeSet<NodeId>,
    pub(in super::super::super) lowest_known_node_id: NodeId,
}

impl ScomsTreeNode {
    pub fn find_lowest_id_accessible_by_neighbor(&self) -> Option<(&NodeId, &NodeId)> {
        let x = self
            .neighbors
            .iter()
            .map(|(neighbor_id, neighbor_info)| {
                (
                    neighbor_id,
                    &neighbor_info.lowest_accessible_thru,
                    neighbor_info.find_oldest_time(),
                )
            })
            .min_by(
                |(_, a_lowest, a_age), (_, b_lowest, b_age)| match a_lowest.cmp(b_lowest) {
                    Ordering::Less => Ordering::Less,
                    Ordering::Equal => a_age.cmp(b_age),
                    Ordering::Greater => Ordering::Greater,
                },
            )?;

        Some((x.0, x.1))
    }
    pub fn find_oldest_neighbor_that_the_lowest_id_can_be_accessed_thru(&self) -> Option<&NodeId> {
        let x = self
            .neighbors
            .iter()
            .filter_map(|(neighbor_id, neighbor_info)| {
                Some((
                    neighbor_id,
                    &neighbor_info.lowest_accessible_thru,
                    neighbor_info.find_oldest_time()?,
                ))
            })
            .min_by(
                |(_, a_lowest, a_age), (_, b_lowest, b_age)| match a_lowest.cmp(b_lowest) {
                    Ordering::Less => Ordering::Less,
                    Ordering::Equal => a_age.cmp(b_age),
                    Ordering::Greater => Ordering::Greater,
                },
            )?;

        if x.0 .0 < self.id.0 {
            Some(x.0)
        } else {
            None
        }
    }
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
