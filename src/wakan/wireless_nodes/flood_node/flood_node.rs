use std::{collections::BTreeSet, rc::Rc};

use qol::logy;

use crate::wakan::{
    wireless_nodes::flood_node::PacketId, FloodPacket, NodeId, Radio, RecievedTime, Time,
    Transmission, WirelessNode,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FloodNode {
    id: NodeId,
    count: u8,
    seen: BTreeSet<PacketId>,
}

impl WirelessNode<FloodPacket> for FloodNode {
    fn tick(
        &mut self,
        now: Time,
        recieved_packets: Vec<(RecievedTime, Rc<FloodPacket>, Radio)>,
    ) -> Result<Vec<Transmission<FloodPacket>>, String> {
        if recieved_packets.is_empty() {
            if now == 0 && self.id.0 == 1 {
                logy!("trace-flood-node", "now == 0 node 1 is starting");
                let count = self.count;
                self.count = self.count.wrapping_add(1);
                self.seen.insert(0);
                Ok(vec![Transmission::new(
                    now + 1 + (count as Time % 8),
                    FloodPacket::new(self.id.clone(), 0),
                    0.into(),
                )])
            } else {
                // we didn't receive anything and we aren't starting so send nothing
                Ok(Vec::new())
            }
        } else {
            logy!("trace-flood-node", "recieved_packets not empty");
            let send_packets: Vec<Transmission<FloodPacket>> = recieved_packets
                .into_iter()
                .filter_map(|(_, packet, _)| {
                    if self.seen.contains(&packet.1) {
                        logy!("trace-flood-node", "filtered out packet{}", packet.1);
                        return None;
                    };
                    self.seen.insert(packet.1);
                    let mut new_packet = packet.as_ref().clone();
                    new_packet.push(self.id.clone());
                    Some(Transmission::new(now + 1, new_packet, 0.into()))
                })
                .collect();
            if self.id.0 == 1 && !send_packets.is_empty() {
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
