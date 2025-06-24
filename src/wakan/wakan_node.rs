use qol::logy;

use crate::wakan::{NodeId, Radio, RecievedTime, ScheduledTransmitionTime, Time, WirelessNode};

pub type WakanPacket = (NodeId, u8);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WakanNode {
    id: u64,
    count: u8,
}
impl WakanNode {
    fn send(&mut self, now: Time,) -> Result<Vec<(ScheduledTransmitionTime, WakanPacket, Radio)>, String> {
        let count = self.count;
        self.count =self.count.wrapping_add(1);
        Ok(vec![(
            now + 1 + (count as Time % 8),
            (self.id, count),
            0,
        )])
    }
}
impl WirelessNode<WakanPacket> for WakanNode {
    fn tick(
        &mut self,
        now: Time,
        recieved_packets: Vec<(RecievedTime, &WakanPacket, Radio)>,
    ) -> Result<Vec<(ScheduledTransmitionTime, WakanPacket, Radio)>, String> {
        if recieved_packets.is_empty() {
            if now == 0{
                logy!("trace-wakan-node", "now == 0");
                self.send(now)
            }else{
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
