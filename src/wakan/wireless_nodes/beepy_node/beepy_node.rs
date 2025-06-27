use std::rc::Rc;

use qol::logy;

use super::BeepyPacket;
use crate::wakan::{NodeId, Radio, RecievedTime, Time, Transmission, WirelessNode};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BeepyNode {
    id: NodeId,
    count: u8,
}
impl BeepyNode {
    fn send(&mut self, now: Time) -> Result<Vec<Transmission<BeepyPacket>>, String> {
        let count = self.count;
        self.count = self.count.wrapping_add(1);
        Ok(vec![Transmission::new(
            now + 1 + (count as Time % 8),
            BeepyPacket::new(self.id.clone(), count),
            0.into(),
        )])
    }
}
impl WirelessNode<BeepyPacket> for BeepyNode {
    fn tick(
        &mut self,
        now: Time,
        recieved_packets: Vec<(RecievedTime, Rc<BeepyPacket>, Radio)>,
    ) -> Result<Vec<Transmission<BeepyPacket>>, String> {
        if recieved_packets.is_empty() {
            if now == 0 {
                logy!("trace-beepy-node", "now == 0");
                self.send(now)
            } else {
                Ok(Vec::new())
            }
        } else {
            logy!("trace-beepy-node", "recieved_packets not empty");
            self.send(now)
        }
    }

    fn new(id: NodeId) -> Self {
        Self { id, count: 0 }
    }
}
