use std::hash::{DefaultHasher, Hash, Hasher};

use super::PlumTreePacket;
use crate::wakan::{NodeId, Radio, RecievedTime, ScheduledTransmitionTime, Time, WirelessNode};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PlumTreeNode {
    id: u64,
    next_beacon: Time,
}
impl WirelessNode<PlumTreePacket> for PlumTreeNode {
    fn tick(
        &mut self,
        now: Time,
        _recieved_packets: Vec<(RecievedTime, &PlumTreePacket, Radio)>,
    ) -> Result<Vec<(ScheduledTransmitionTime, PlumTreePacket, Radio)>, String> {
        if now >= self.next_beacon {
            self.next_beacon = gen_next_heartbeat_time(self.next_beacon);
            todo!("send out beacon");
        };
        todo!()
    }
    fn new(id: NodeId) -> Self {
        let next_beacon = gen_next_heartbeat_time(id as Time);
        Self { id, next_beacon }
    }
}

fn gen_next_heartbeat_time(time: Time) -> Time {
    let mut hasher = DefaultHasher::new();
    time.hash(&mut hasher);
    let hash = hasher.finish();
    (hash % 29) as Time
}
