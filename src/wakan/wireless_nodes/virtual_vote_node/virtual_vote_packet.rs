use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VirtualVotePacket<MessageId, NodeAddress> {
    Message {
        source: NodeAddress,
        message_id: MessageId,
    },
    HasHeard {
        source: NodeAddress,
        message_id: MessageId,
        nodes_that_have_heard: Vec<NodeAddress>,
        ready_to_drop: bool,
    },
}

impl<MessageId, NodeAddress> VirtualVotePacket<MessageId, NodeAddress> {
    pub fn new_message(message_id: MessageId, source: NodeAddress) -> Self {
        VirtualVotePacket::Message { source, message_id }
    }
    pub fn new_has_heard(
        message_id: MessageId,
        nodes_that_have_heard: Vec<NodeAddress>,
        ready_to_drop: bool,
        source: NodeAddress,
    ) -> Self {
        VirtualVotePacket::HasHeard {
            source,
            message_id,
            nodes_that_have_heard,
            ready_to_drop,
        }
    }
}

impl<MessageId: fmt::Display, NodeAddress: fmt::Display> fmt::Display
    for VirtualVotePacket<MessageId, NodeAddress>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VirtualVotePacket::Message { source, message_id } => {
                write!(f, "{{source:{source}, message_id:{message_id}}})")
            }
            VirtualVotePacket::HasHeard {
                source,
                message_id,
                nodes_that_have_heard,
                ready_to_drop,
            } => {
                let vecna: qol::Vecna<_> = nodes_that_have_heard.into();
                write!(f, "{{source:{source}, message_id:{message_id}, nodes_that_have_heard:{vecna}, ready_to_drop:{ready_to_drop}}})")
            }
        }
    }
}
