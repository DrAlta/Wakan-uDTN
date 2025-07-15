use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::wakan::{Node, NodeId, WirelessNode};

#[derive(Deserialize, Serialize, Debug)]
pub struct RawNode {
    pub id: NodeId,
    pub x: f32,
    pub y: f32,
    pub outbound_links: BTreeSet<NodeId>,
}

impl<P, N: WirelessNode<P>> From<Node<P, N>> for RawNode {
    fn from(value: Node<P, N>) -> Self {
        RawNode {
            id: value.id,
            x: value.x.into(),
            y: value.y.into(),
            outbound_links: value.outbound_links,
        }
    }
}

impl<P, N: WirelessNode<P>> From<&Node<P, N>> for RawNode {
    fn from(value: &Node<P, N>) -> Self {
        RawNode {
            id: value.id.clone(),
            x: value.x.into(),
            y: value.y.into(),
            outbound_links: value.outbound_links.clone(),
        }
    }
}
