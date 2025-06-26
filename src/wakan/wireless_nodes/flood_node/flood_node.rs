use std::collections::BTreeSet;

use qol::logy;

use crate::wakan::{
    wireless_nodes::flood_node::PacketId, FloodPacket, NodeId, Radio, RecievedTime,
    ScheduledTransmitionTime, Time, WirelessNode,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FloodNode {
    id: u64,
    count: u8,
    seen: BTreeSet<PacketId>,
}

impl WirelessNode<FloodPacket> for FloodNode {
    fn tick(
        &mut self,
        now: Time,
        recieved_packets: Vec<(RecievedTime, &FloodPacket, Radio)>,
    ) -> Result<Vec<(ScheduledTransmitionTime, FloodPacket, Radio)>, String> {
        if recieved_packets.is_empty() {
            if now == 0 && self.id == 1 {
                logy!("trace-flood-node", "now == 0 node 1 is starting");
                let count = self.count;
                self.count = self.count.wrapping_add(1);
                self.seen.insert(0);
                Ok(vec![(
                    now + 1 + (count as Time % 8),
                    FloodPacket::new(self.id, 0),
                    0,
                )])
            } else {
                // we didn't receive anything and we aren't starting so send nothing
                Ok(Vec::new())
            }
        } else {
            logy!("trace-flood-node", "recieved_packets not empty");
            let send_packets: Vec<(u64, FloodPacket, u8)> = recieved_packets
                .into_iter()
                .filter_map(|(_, packet, _)| {
                    if self.seen.contains(&packet.1) {
                        logy!("trace-flood-node", "filtered out packet{}", packet.1);
                        return None;
                    };
                    self.seen.insert(packet.1);
                    let mut new_packet = packet.clone();
                    new_packet.push(self.id);
                    Some((now + 1, new_packet, 0))
                })
                .collect();
            if self.id == 1 && !send_packets.is_empty() {
                logy!("info", "node 1 is sending stuff");
            }
            Ok(send_packets)
        }
    }

    fn new(id: NodeId) -> Self {
        Self {
            id,
            count: 0,
            seen: BTreeSet::new(),
        }
    }
}
