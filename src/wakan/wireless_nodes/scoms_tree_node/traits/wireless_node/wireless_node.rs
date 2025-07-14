use std::{
    collections::{BTreeMap, BTreeSet},
    rc::Rc,
};

use crate::wakan::{
    wireless_nodes::scoms_tree_node::gen_next_heartbeat_time, NodeId, Radio, RecievedTime,
    ScomsTreeNode, ScomsTreePacket, Time, Transmission, WirelessNode,
};

impl WirelessNode<ScomsTreePacket> for ScomsTreeNode {
    fn tick(
        &mut self,
        now: Time,
        recieved_packets: Vec<(RecievedTime, Rc<ScomsTreePacket>, Radio)>,
    ) -> Result<Vec<Transmission<ScomsTreePacket>>, String> {
        // Create a list to store transmissions we might generate during this tick
        let mut transmissions = Vec::new();

        ////////////////////////////
        // Process each received packet
        for (recieved_time, packet, radio) in recieved_packets {
            match packet.as_ref() {
                // We're only handling Beacon packets here
                ScomsTreePacket::Beacon {
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
                ScomsTreePacket::new_beacon(
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
                            (neighbor_id.clone(), info.lowest_accessable_thru.clone())
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
            id,
            next_beacon,
            neighbors: BTreeMap::new(),
            parent_maybe: None,
            children: BTreeSet::new(),
        }
    }
}
