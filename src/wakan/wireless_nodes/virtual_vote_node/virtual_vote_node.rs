use std::rc::Rc;

use ghx_constrained_delaunay::hashbrown::HashMap;

use super::VirtualVotePacket;
use crate::wakan::{
    wireless_nodes::virtual_vote_node::VirtualVote, NodeId, Radio, RecievedTime, Time,
    Transmission, WirelessNode,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VirtualVoteNode<MessageId, NodeAddress> {
    id: NodeId,
    virtual_vote: VirtualVote<MessageId, NodeAddress>,
}
impl<MessageId: std::hash::Hash + Ord + Clone, NodeAddress: std::cmp::PartialEq + Clone>
    WirelessNode<VirtualVotePacket<MessageId, NodeAddress>>
    for VirtualVoteNode<MessageId, NodeAddress>
{
    fn tick(
        &mut self,
        _now: Time,
        recieved_packets: Vec<(
            RecievedTime,
            Rc<VirtualVotePacket<MessageId, NodeAddress>>,
            Radio,
        )>,
    ) -> Result<Vec<Transmission<VirtualVotePacket<MessageId, NodeAddress>>>, String> {
        if recieved_packets.is_empty() {
            todo!()
        } else {
            let mut has_heards = HashMap::new();
            let mut messages = Vec::new();

            for (_, packet, _) in recieved_packets {
                let packet = &*packet;
                match packet {
                    VirtualVotePacket::Message { source, message_id } => messages.push(
                        VirtualVotePacket::new_message(message_id.clone(), source.clone()),
                    ),
                    VirtualVotePacket::HasHeard {
                        source,
                        message_id,
                        nodes_that_have_heard,
                        ready_to_drop,
                    } => {
                        match self.virtual_vote.handle_has_heard_packet(nodes_that_have_heard, *ready_to_drop, message_id.clone(), source.clone()) {
                            crate::wakan::wireless_nodes::virtual_vote_node::HandlePacketReturn::AtEase => (),
                            crate::wakan::wireless_nodes::virtual_vote_node::HandlePacketReturn::BroadCastHasHeard { message_id, mut new_have_heard, ready_to_drop } => {
                                if let Some(VirtualVotePacket::HasHeard { source:_, message_id:_, nodes_that_have_heard, ready_to_drop: my_ready_to_drop}) = has_heards.get_mut(&message_id) {
                                    nodes_that_have_heard.append(&mut new_have_heard);
                                    *my_ready_to_drop = *my_ready_to_drop || ready_to_drop;
                                } else {
                                    let msg = VirtualVotePacket::new_has_heard(message_id.clone(), nodes_that_have_heard.clone(), ready_to_drop, source.clone());
                                    has_heards.insert(message_id, msg);
                                }
                            },
                        }
                    }
                }
            }
            todo!()
        }
    }

    fn new(id: NodeId) -> Self {
        Self {
            id,
            virtual_vote: VirtualVote::new(),
        }
    }
}
