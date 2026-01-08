use std::collections::BTreeMap;

use crate::wakan::wireless_nodes::virtual_vote_node::MIN_NEEDED_TO_DROP;

type Set<T> = Vec<T>;

pub enum HandlePacketReturn<MessageId, NodeAddress> {
    AtEase,
    BroadCastHasHeard {
        message_id: MessageId,
        new_have_heard: Set<NodeAddress>,
        ready_to_drop: bool,
    },
}
use HandlePacketReturn::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MessageInfo<NodeAddress> {
    neighbors_ready_to_drop: Set<NodeAddress>,
    ready_to_drop: bool,
    has_heard: Set<NodeAddress>,
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VirtualVote<MessageId, NodeAddress> {
    messages: BTreeMap<MessageId, MessageInfo<NodeAddress>>,
}
impl<MessageId, NodeAddress> VirtualVote<MessageId, NodeAddress> {
    pub fn new() -> Self {
        VirtualVote {
            messages: BTreeMap::new(),
        }
    }
}
impl<MessageId, NodeAddress> VirtualVote<MessageId, NodeAddress> {
    pub fn handle_has_heard_packet(
        &mut self,
        nodes_that_have_heard: &Vec<NodeAddress>,
        ready_to_drop: bool,
        message_id: MessageId,
        source: NodeAddress,
    ) -> HandlePacketReturn<MessageId, NodeAddress>
    where
        MessageId: std::hash::Hash + PartialEq + Eq + Ord,
        NodeAddress: PartialEq + Clone,
    {
        let mut new_have_heard = Set::from([source.clone()]);
        if let Some(message_info) = self.messages.get_mut(&message_id) {
            if ready_to_drop {
                message_info.neighbors_ready_to_drop.push(source)
            }
            for node_address in nodes_that_have_heard {
                if !message_info.has_heard.contains(node_address) {
                    new_have_heard.push(node_address.clone())
                }
            }
            for newly_heard_has_heard in &new_have_heard {
                message_info.has_heard.push(newly_heard_has_heard.clone())
            }
            message_info.ready_to_drop =
                message_info.ready_to_drop || message_info.has_heard.len() >= MIN_NEEDED_TO_DROP;
            return BroadCastHasHeard {
                message_id,
                new_have_heard: new_have_heard,
                ready_to_drop: message_info.ready_to_drop,
            };
        }
        return AtEase;
        // ignore the message
    }
}
