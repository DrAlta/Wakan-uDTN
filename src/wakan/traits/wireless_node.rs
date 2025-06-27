use std::rc::Rc;

use crate::wakan::{NodeId, Radio, RecievedTime, Time, Transmission};

pub trait WirelessNode<Packet> {
    fn tick(
        &mut self,
        now: Time,
        recieved_packets: Vec<(RecievedTime, Rc<Packet>, Radio)>,
    ) -> Result<Vec<Transmission<Packet>>, String>;
    fn new(id: NodeId) -> Self;
}
