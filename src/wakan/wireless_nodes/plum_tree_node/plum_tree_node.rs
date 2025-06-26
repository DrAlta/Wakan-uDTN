use std::{
    collections::BTreeMap,
    hash::{DefaultHasher, Hash, Hasher},
    rc::Rc,
};

use super::PlumTreePacket;
use crate::wakan::{
    wireless_nodes::plum_tree_node::NeighborInfo, NodeId, Radio, RecievedTime, Time, Transmission,
    WirelessNode,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PlumTreeNode {
    id: u64,
    next_beacon: Time,
    neighbors: BTreeMap<NodeId, NeighborInfo>,
}

impl WirelessNode<PlumTreePacket> for PlumTreeNode {
    fn tick(
        &mut self,
        now: Time,
        recieved_packets: Vec<(RecievedTime, Rc<PlumTreePacket>, Radio)>,
    ) -> Result<Vec<Transmission<PlumTreePacket>>, String> {
        let mut transmissions = Vec::new();

        for (recieved_time, packet, _radio) in recieved_packets {
            match packet.as_ref() {
                PlumTreePacket::Beacon { source, neighbors } => {
                    match self.neighbors.get_mut(source) {
                        Some(neighbor_info) => {
                            neighbor_info.last_seen = recieved_time;
                            neighbor_info.neighbors_of_neighbor = neighbors.clone();
                        }
                        None => {
                            let last_seen = recieved_time;
                            let neighbors_of_neighbor = neighbors.clone();
                            self.neighbors.insert(
                                *source,
                                NeighborInfo {
                                    last_seen,
                                    neighbors_of_neighbor,
                                },
                            );
                        }
                    }
                }
            }
        }
        if now >= self.next_beacon {
            self.next_beacon = gen_next_heartbeat_time(self.next_beacon);
            transmissions.push(
                (
                    now + 1,
                    PlumTreePacket::new_beacon(
                        self.id,
                        self.neighbors.keys().map(|x| *x).collect(),
                    ),
                    0,
                )
                    .into(),
            );
        };
        Ok(transmissions)
    }
    fn new(id: NodeId) -> Self {
        let next_beacon = gen_next_heartbeat_time(id as Time);
        Self {
            id,
            next_beacon,
            neighbors: BTreeMap::new(),
        }
    }
}

fn gen_next_heartbeat_time(time: Time) -> Time {
    let mut hasher = DefaultHasher::new();
    time.hash(&mut hasher);
    let hash = hasher.finish();
    (hash % 29) as Time + 5
}
