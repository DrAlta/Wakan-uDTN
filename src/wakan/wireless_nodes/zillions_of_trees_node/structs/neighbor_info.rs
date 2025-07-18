use std::{cmp::Ordering, collections::{BTreeMap, BTreeSet}};

use crate::wakan::{NodeId, Radio, Time};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NeighborInfo{
    pub queen: NodeId,
    pub princess: NodeId,

    pub flow: BTreeSet<NodeId>,
    pub tree: BTreeSet<NodeId>,

    pub first_seen: BTreeMap<Radio, Time>,
    pub last_seen: BTreeMap<Radio, Time>,

}
impl NeighborInfo{
    pub fn find_last_seen(&self) -> Option<(&Radio, &Time)> {
        self.last_seen
            .iter()
            .max_by(|(ar, at), (br, bt)| match at.cmp(bt) {
                Ordering::Less => Ordering::Less,
                Ordering::Equal => ar.cmp(br),
                Ordering::Greater => Ordering::Greater,
            })
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
    pub fn find_lowest_id_accessible_thru(&self) -> Option<NodeId> {
        self.flow.first().iter()
        .chain(self.tree.first().iter())
        .min_by(
            |a,b|
            a.0.cmp(b.0)
        ).cloned()
    }
}