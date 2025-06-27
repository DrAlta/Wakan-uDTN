use std::{
    collections::{BTreeMap, BTreeSet},
    rc::Rc,
};

use crate::wakan::{
    wireless_nodes::plum_tree_node::{gen_next_heartbeat_time, NeighborInfo},
    NodeId, PlumTreeNode, PlumTreePacket, Radio, RecievedTime, Time, Transmission, WirelessNode,
};

impl WirelessNode<PlumTreePacket> for PlumTreeNode {
    fn tick(
        &mut self,
        now: Time,
        recieved_packets: Vec<(RecievedTime, Rc<PlumTreePacket>, Radio)>,
    ) -> Result<Vec<Transmission<PlumTreePacket>>, String> {
        // Create a list to store transmissions we might generate during this tick
        let mut transmissions = Vec::new();

        // Process each received packet
        for (recieved_time, packet, radio) in recieved_packets {
            match packet.as_ref() {
                // We're only handling Beacon packets here
                PlumTreePacket::Beacon { source, neighbors } => {
                    self.handle_beacon(source, neighbors, &recieved_time, &radio)
                }
            }
        }
        let cutoff = now - 25;
        let mut neighbors_that_went_down = Vec::new();
        self.neighbors.retain(|neighbor_id, neighbor_info| {
            neighbor_info.last_seen.retain(|_, time| *time >= cutoff);
            if neighbor_info.last_seen.is_empty() {
                neighbors_that_went_down.push(neighbor_id.clone());
                false
            } else {
                true
            }
        });
        // Check if it's time to send out a new beacon
        if now >= self.next_beacon {
            // Schedule the next heartbeat
            self.next_beacon = gen_next_heartbeat_time(self.next_beacon);

            // Construct a new Beacon packet advertising our own ID
            // and all known neighbors — this helps others learn about us.
            transmissions.push(Transmission::new(
                now + 1, // Add slight delay to allow channel scheduling
                PlumTreePacket::new_beacon(
                    self.id.clone(),                                    // Identify ourself as the sender
                    self.neighbors.keys().map(|x| x.clone()).collect(), // Share our known neighbor list
                ),
                0.into(), // Some radio/channel priority abstraction
            ));
        };

        // Return the list of transmissions we want to make this tick
        Ok(transmissions)
    }

    fn new(id: NodeId) -> Self {
        let next_beacon = gen_next_heartbeat_time(id.0 as Time);
        Self {
            id,
            next_beacon,
            neighbors: BTreeMap::new(),
            parent: None,
        }
    }
}

impl PlumTreeNode {
    fn handle_beacon(
        &mut self,
        source: &NodeId,
        neighbors: &BTreeSet<NodeId>,
        recieved_time: &Time,
        radio: &Radio,
    ) {
        // Beacon packets advertise a node's presence and its known neighbors.
        // This helps build and update the network view.

        match self.neighbors.get_mut(source) {
            Some(neighbor_info) => {
                // This neighbor is already known.
                // Update the last time we saw it from this radio.
                neighbor_info
                    .last_seen
                    .insert(radio.clone(), recieved_time.clone());

                // Update the cached view of this neighbor's neighbors
                neighbor_info.neighbors_of_neighbor = neighbors.clone();
            }
            None => {
                // We've never seen this neighbor before.
                // Initialize tracking info for it.

                // Record the first time we saw it (from this radio)
                let first_seen = BTreeMap::from([(radio.clone(), recieved_time.clone())]);

                // Copy its advertised neighbors
                let neighbors_of_neighbor = neighbors.clone();

                // Insert this new neighbor into our neighbor map
                self.neighbors.insert(
                    source.clone(),
                    NeighborInfo {
                        last_seen: first_seen.clone(),
                        first_seen,
                        neighbors_of_neighbor,
                    },
                );
            }
        }
    }
}
