use std::rc::Rc;

use qol::logy;

use crate::wakan::{NodeId, Radio, RecievedTime, Time, Transmission, WakanPacket, WirelessNode};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WakanNode {
    id: NodeId,
    count: u8,
}
impl WakanNode {
    fn send(&mut self, now: Time) -> Result<Vec<Transmission<WakanPacket>>, String> {
        let count = self.count;
        self.count = self.count.wrapping_add(1);
        Ok(vec![Transmission::new(
            now + 1 + (count as Time % 8),
            (self.id.clone(), count),
            0.into(),
        )])
    }
}
impl WirelessNode<WakanPacket> for WakanNode {
    fn tick(
        &mut self,
        now: Time,
        recieved_packets: Vec<(RecievedTime, Rc<WakanPacket>, Radio)>,
    ) -> Result<Vec<Transmission<WakanPacket>>, String> {
        if recieved_packets.is_empty() {
            if now == 0 {
                logy!("trace-wakan-node", "now == 0");
                self.send(now)
            } else {
                Ok(Vec::new())
            }
        } else {
            logy!("trace-wakan-node", "recieved_packets not empty");
            self.send(now)
        }
    }

    fn new(id: NodeId) -> Self {
        Self { id, count: 0 }
    }
}
