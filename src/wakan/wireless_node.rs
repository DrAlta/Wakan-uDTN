use crate::wakan::{NodeId, Radio, RecievedTime, ScheduledTransmitionTime, Time};

pub trait WirelessNode<Packet> {
    fn tick(
        &mut self,
        now: Time,
        recieved_packets: Vec<(RecievedTime, &Packet, Radio)>,
    ) -> Result<Vec<(ScheduledTransmitionTime, Packet, Radio)>, String>;
    fn new(id: NodeId) -> Self;
}
