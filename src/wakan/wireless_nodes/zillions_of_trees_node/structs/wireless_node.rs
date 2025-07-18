use std::{collections::{BTreeMap, BTreeSet}, hash::{DefaultHasher, Hash, Hasher}};

use crate::wakan::WirelessNode;
use super::super::{MAX_AGE, ZillionsOfTreesNode, ZillionsOfTreesPacket};

impl WirelessNode<ZillionsOfTreesPacket> for ZillionsOfTreesNode{
    fn tick(
        &mut self,
        now: crate::wakan::Time,
        recieved_packets: Vec<(crate::wakan::RecievedTime, std::rc::Rc<ZillionsOfTreesPacket>, crate::wakan::Radio)>,
    ) -> Result<Vec<crate::wakan::Transmission<ZillionsOfTreesPacket>>, String> {
        let mut transmissions = Vec::new();

        // handle receptions
        for (recieved_time, packet_rc, radio) in recieved_packets{
            let packet = packet_rc.as_ref();
            match packet{
                ZillionsOfTreesPacket::Beacon { source: neighbor_id, neighbors: neighbors_neighbors, princess } => {
                    self.handle_beacon(neighbor_id, neighbors_neighbors, princess, recieved_time, &radio);
                }
            }
        }

        // update
        self.update(now);


        // handles transmittions
        if now >= self.next_beacon {
            let mut hasher = DefaultHasher::new();
            now.hash(&mut hasher);
            self.id.0.hash(&mut hasher);
            let hash = hasher.finish();
            self.next_beacon = now + (hash % MAX_AGE);
            transmissions.push(self.generate_beacon());
        }

        Ok(transmissions)
    }

    fn new(id: crate::wakan::NodeId) -> Self {
        let mut x = ZillionsOfTreesNode{ 
            id: id.clone(),
            queen: id.clone(),
            princess: id, 
            neighbors: BTreeMap::new(), 
            tree_neighbors: BTreeSet::new(),
            next_beacon: 0,
        };
        let mut hasher = DefaultHasher::new();
        x.id.0.hash(&mut hasher);
        let hash = hasher.finish();
        x.next_beacon = hash % MAX_AGE;
        x
    }
}