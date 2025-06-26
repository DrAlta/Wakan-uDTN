use std::{collections::BTreeSet, fmt};

use crate::wakan::NodeId;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PlumTreePacket {
    Beacon {
        source: NodeId,
        neighbors: BTreeSet<NodeId>,
    },
}

impl PlumTreePacket {
    pub fn new_beacon(source: NodeId, neighbors: BTreeSet<NodeId>) -> Self {
        Self::Beacon { source, neighbors }
    }
}

impl fmt::Display for PlumTreePacket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PlumTreePacket::Beacon { source, neighbors } => {
                write!(f, "Beacon{{Source:{}, neighbors:{:?}}})", source, neighbors)
            }
        }
    }
}
