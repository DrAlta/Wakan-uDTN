use std::marker::PhantomData;
use ordered_f32::OrderedF32;
use crate::wakan::{NodeId, WirelessNode};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Node<P, N: WirelessNode<P>> {
    pub id: NodeId,
    pub wireless_node: N,
    pub x: OrderedF32,
    pub y: OrderedF32,
    pub outbound_links: Vec<NodeId>,
    pub inbound_links: Vec<NodeId>,

    phantom: PhantomData<P>,
}
impl<P, N: WirelessNode<P>> Node<P, N> {
    pub fn new(
        id: NodeId,
        x: f32,
        y: f32,
        outbound_links: Vec<NodeId>,
        inbound_links: Vec<NodeId>,
    ) -> Self {
        let wireless_node = WirelessNode::new(id);
        Node { id, wireless_node, x: x.into(), y: y.into(), outbound_links, inbound_links, phantom: PhantomData }
    }
}
