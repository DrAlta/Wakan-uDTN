use std::collections::{BTreeMap, BTreeSet};

use crate::wakan::NodeId;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ZillionsOfTreesPacket {
    Beacon {
        source: NodeId,
        princess: NodeId,
        neighbors: BTreeMap<NodeId, BTreeSet<NodeId>>,
        //        neighbors: BTreeMap<NodeId, (LowestAccessible, HighestAccessible)>,
    },
}
