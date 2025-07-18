use std::collections::BTreeSet;

use crate::wakan::NodeId;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NeighborInfo{
    pub high_accessible_trough: BTreeSet<NodeId>,
    pub low_accessible_trough: BTreeSet<NodeId>,
}