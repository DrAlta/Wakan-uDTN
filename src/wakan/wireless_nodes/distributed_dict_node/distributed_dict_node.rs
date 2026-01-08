use std::rc::Rc;

use crate::wakan::{
    wireless_nodes::distributed_dict_node::DistributedDict, DistributedDictPacket, NodeId, Radio,
    RecievedTime, Time, Transmission, WirelessNode,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DistributedDictNode {
    //sim stuff
    pub(super) id: NodeId,

    pub(super) distributed_dict: DistributedDict,

    pub(super) need_to_broadcast_hlu: bool,
}
impl WirelessNode<DistributedDictPacket> for DistributedDictNode {
    fn tick(
        &mut self,
        now: Time,
        recieved_packets: Vec<(RecievedTime, Rc<DistributedDictPacket>, Radio)>,
    ) -> Result<Vec<Transmission<DistributedDictPacket>>, String> {
        todo!("{now:?}{recieved_packets:?}")
    }

    fn new(id: NodeId) -> Self {
        DistributedDictNode {
            distributed_dict: DistributedDict::new(id.0.into()),
            id,
            need_to_broadcast_hlu: true,
        }
    }
}
