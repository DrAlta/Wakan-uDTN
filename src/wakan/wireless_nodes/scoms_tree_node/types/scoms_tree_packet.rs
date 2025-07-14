use std::{collections::BTreeMap, fmt};

use crate::wakan::NodeId;

type LowestKnownId = NodeId;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ScomsTreePacket {
    Beacon {
        source: NodeId,
        neighbors: BTreeMap<NodeId, LowestKnownId>,
    },
}

impl ScomsTreePacket {
    pub fn new_beacon(source: NodeId, neighbors: BTreeMap<NodeId, LowestKnownId>) -> Self {
        Self::Beacon { source, neighbors }
    }
}

impl fmt::Display for ScomsTreePacket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScomsTreePacket::Beacon { source, neighbors } => {
                write!(f, "Beacon{{Source:{}, neighbors:{:?}}})", source, neighbors)
            }
        }
    }
}
