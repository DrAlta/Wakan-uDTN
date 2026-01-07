use std::{collections::{BTreeMap, BTreeSet}, rc::Rc};

use crate::wakan::{DistributedDictPacket, NodeId, Radio, RecievedTime, Time, Transmission, WirelessNode, wireless_nodes::distributed_dict_node::{LEVEL_BUCKETS, LEVELS, NodeAddress, ROOT_SIZE, StartOfAddressRange, distributed_dict_node::{Block, Path}}};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DistributedDictNode {
    //sim stuff
    pub (super) id: NodeId,

    // Dict stuff
    pub (super) root: [Block; ROOT_SIZE],
    pub (super) levels: [[(StartOfAddressRange, Block); LEVEL_BUCKETS]; LEVELS],
    pub (super) canopy: StartOfAddressRange,
    pub (super) tracked_nodes: BTreeMap<NodeAddress, Path>,

    // Dict Update
    pub (super) _my_address: NodeAddress,
    // A list of one-hop neighbors.
    pub (super) neighbors: BTreeSet<NodeAddress>,
    // The current maximum number of hops for discovery messages.
    pub (super) hop_limit: u8,
    // Timestamp of the last received HLU packet that either increased or confirmed the hop_limit.
    pub (super) last_hlu_received_time: Time,
    // Timestamp of the last time this node broadcast an HLU packet.
    pub (super) last_hlu_broadcast_time: Time,
    // EMA of receiving HLU packet that either increased or confirmed the hop_limit
    pub (super) hlu_mean_interval: Time,
    //  EMA of the squared differences from the mean.
    pub (super) hlu_variance: u32,

    pub (super) time_of_next_hlu_broadcast: Time,
    pub (super) need_to_broadcast_hlu: bool,
    // station stuff
}
/*
impl DistributedDictNode {
    fn send(&mut self, now: Time) -> Result<Vec<Transmission<DistributedDictPacket>>, String> {
        let count = self.count;
        self.count = self.count.wrapping_add(1);
        Ok(vec![Transmission::new(
            now + 1 + (count as Time % 8),
            DistributedDictPacket::new(self.id.clone(), count),
            0.into(),
        )])
    }
}
*/
impl WirelessNode<DistributedDictPacket> for DistributedDictNode {
    fn tick(
        &mut self,
        now: Time,
        recieved_packets: Vec<(RecievedTime, Rc<DistributedDictPacket>, Radio)>,
    ) -> Result<Vec<Transmission<DistributedDictPacket>>, String> {
        /*
        if recieved_packets.is_empty() {
            if now == 0 && self.id.0 == 1 {
                logy!("trace-beepy-node", "we are node 0 and now == 0");
                self.send(now)
            } else {
                Ok(Vec::new())
            }
        } else {
            logy!("trace-beepy-node", "recieved_packets not empty");
            self.send(now)
        }*/
        todo!("{now:?}{recieved_packets:?}")
    }

    fn new(id: NodeId) -> Self {
        DistributedDictNode::new(id.clone(), id.0.into(), 0)
    }
}
