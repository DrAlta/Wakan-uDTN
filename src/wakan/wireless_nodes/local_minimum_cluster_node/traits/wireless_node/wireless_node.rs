use std::{
    collections::{BTreeMap, BTreeSet},
    rc::Rc,
};

use crate::wakan::{NodeId, Radio, RecievedTime, Time, Transmission, WirelessNode};

use super::super::super::{gen_next_heartbeat_time, LocalMinimumClusterNode, LocalMinimumClusterPacket};

impl WirelessNode<LocalMinimumClusterPacket> for LocalMinimumClusterNode {
    fn tick(
        &mut self,
        now: Time,
        recieved_packets: Vec<(RecievedTime, Rc<LocalMinimumClusterPacket>, Radio)>,
    ) -> Result<Vec<Transmission<LocalMinimumClusterPacket>>, String> {
        // Create a list to store transmissions we might generate during this tick
        let mut transmissions = Vec::new();

        ////////////////////////////
        // Process each received packet
        for (recieved_time, packet, radio) in recieved_packets {
            match packet.as_ref() {
                // We're only handling Beacon packets here
                LocalMinimumClusterPacket::Beacon {
                    source,
                    neighbors,
                    parent_maybe,
                } => self.handle_beacon(
                    neighbors,
                    parent_maybe.as_ref(),
                    source,
                    &recieved_time,
                    &radio,
                ),
            }
        }

        ////////////////////////////
        // do any processing needed ths tick
        self.update(now);

        ////////////////////////////
        // send an packets

        // Check if it's time to send out a new beacon
        if now >= self.next_beacon {
            // Schedule the next heartbeat
            self.next_beacon = gen_next_heartbeat_time(self.next_beacon);

            // Construct a new Beacon packet advertising our own ID
            // and all known neighbors — this helps others learn about us.
            transmissions.push(Transmission::new(
                now + 1, // Add slight delay to allow channel scheduling
                LocalMinimumClusterPacket::new_beacon(
                    self.neighbors
                        .iter()
                        .filter(|&(neighbor_id, _)| {
                            if self.parent_maybe.as_ref() == Some(neighbor_id) {
                                return true;
                            } else {
                                self.children.contains(neighbor_id)
                            }
                        })
                        .map(|(neighbor_id, info)| {
                            (neighbor_id.clone(), info.lowest_accessible_thru.clone())
                        })
                        .collect(), // Share our known neighbor list
                    self.parent_maybe.clone(),
                    self.id.clone(), // Identify ourself as the sender
                ),
                0.into(), // Some radio/channel priority abstraction
            ));
        };

        ////////////////////////////
        // Return the list of transmissions we want to make this tick
        Ok(transmissions)
    }

    fn new(id: NodeId) -> Self {
        let next_beacon = gen_next_heartbeat_time(id.0 as Time);
        Self {
            lowest_known_node_id: id.clone(),
            id,
            next_beacon,
            neighbors: BTreeMap::new(),
            parent_maybe: None,
            children: BTreeSet::new(),
        }
    }
}
