use crate::wakan::{NodeId, Radio, RecievedTime, ScheduledTransmitionTime, Time, WirelessNode};

pub type WakanPacket = (NodeId, u8);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WakanNode{
    id: u64,
    count: u8 
}
impl WirelessNode<WakanPacket> for WakanNode {
    fn tick(
        &mut self, 
        now: Time,
         recieved_packets: Vec<(
            RecievedTime, 
            &WakanPacket, 
            Radio,
        )>
    ) -> Vec<(
        ScheduledTransmitionTime, 
        WakanPacket, 
        Radio
    )> {
        let _ = recieved_packets;
        let count = self.count;
        self.count += 1;
        vec![(
            now + (
                (
                    (count as Time % 8)
                    * 1000
                ) 
                / 8
            ), 
            (self.id, count), 
            0
        )]
    }
    
    fn new(id: NodeId) -> Self {
        Self{ id, count: 0 }
    }
} 