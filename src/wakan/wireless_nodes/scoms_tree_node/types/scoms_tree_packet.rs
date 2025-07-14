use std::{collections::BTreeMap, fmt};

use crate::wakan::NodeId;

type LowestKnownId = NodeId;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ScomsTreePacket {
    Beacon {
        source: NodeId,
        parent_maybe: Option<NodeId>,
        neighbors: BTreeMap<NodeId, LowestKnownId>,
    },
}

impl ScomsTreePacket {
    pub fn new_beacon(
        neighbors: BTreeMap<NodeId, LowestKnownId>,
        parent: Option<NodeId>,
        source: NodeId,
    ) -> Self {
        Self::Beacon {
            source,
            neighbors,
            parent_maybe: parent,
        }
    }
}

impl fmt::Display for ScomsTreePacket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScomsTreePacket::Beacon {
                source,
                neighbors,
                parent_maybe: parent,
            } => {
                write!(
                    f,
                    "Beacon{{source:{}, parent:{:?}, neighbors:{:?}}})",
                    source, parent, neighbors
                )
            }
        }
    }
}
