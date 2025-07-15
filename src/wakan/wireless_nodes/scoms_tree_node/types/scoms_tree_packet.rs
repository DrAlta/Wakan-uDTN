use std::{collections::BTreeMap, fmt};

use crate::wakan::NodeId;

type LowestKnownId = NodeId;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ScomsTreePacket {
    Beacon {
        source: NodeId,
        packet_id: u64,
        parent_maybe: Option<NodeId>,
        neighbors: BTreeMap<NodeId, LowestKnownId>,
    },
    TreeMerge {
        source: NodeId,
        packet_id: u64,
        new_root: NodeId,
    },
}

impl ScomsTreePacket {
    pub fn new_beacon(
        neighbors: BTreeMap<NodeId, LowestKnownId>,
        parent: Option<NodeId>,
        source: NodeId,
        packet_id: u64,
    ) -> Self {
        Self::Beacon {
            source,
            packet_id,
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
                packet_id,
            } => {
                write!(
                    f,
                    "Beacon{{source:{}, parent:{:?}, neighbors:{:?}, packet id: {}}})",
                    source, parent, neighbors, packet_id
                )
            }
            ScomsTreePacket::TreeMerge {
                source,
                new_root,
                packet_id,
            } => {
                write!(
                    f,
                    "TreeMerge{{source:{}, new_root:{}, packet id: {}}})",
                    source, new_root, packet_id
                )
            }
        }
    }
}
