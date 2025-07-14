use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};

use crate::wakan::{NodeId, Radio, Time};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NeighborInfo {
    pub first_seen: BTreeMap<Radio, Time>,
    pub last_seen: BTreeMap<Radio, Time>,
    pub neighbors_of_neighbor: BTreeSet<NodeId>,
    pub lowest_accessable_thru: NodeId,
}
#[allow(dead_code)]
impl NeighborInfo {
    pub fn simple_new(first_seen: BTreeMap<Radio, Time>, lowest_accessable_thru: NodeId) -> Self {
        Self {
            last_seen: first_seen.clone(),
            first_seen,
            neighbors_of_neighbor: BTreeSet::new(),
            lowest_accessable_thru,
        }
    }
    pub fn new(
        first_seen: BTreeMap<Radio, Time>,
        last_seen: BTreeMap<Radio, Time>,
        neighbors_of_neighbor: BTreeSet<NodeId>,
        lowest_accessable_thru: NodeId,
    ) -> Self {
        Self {
            first_seen,
            last_seen,
            neighbors_of_neighbor,
            lowest_accessable_thru,
        }
    }
    pub fn find_oldest_time(&self) -> Option<Time> {
        Some(
            self.first_seen
                .iter()
                .min_by(|(_, a), (_, b)| a.cmp(b))?
                .1
                .clone(),
        )
    }
    pub fn find_oldest_radio_time(&self) -> Option<(&Radio, &Time)> {
        self.first_seen
            .iter()
            .min_by(|(ar, at), (br, bt)| match at.cmp(bt) {
                Ordering::Less => Ordering::Less,
                Ordering::Equal => ar.cmp(br),
                Ordering::Greater => Ordering::Greater,
            })
    }
}
