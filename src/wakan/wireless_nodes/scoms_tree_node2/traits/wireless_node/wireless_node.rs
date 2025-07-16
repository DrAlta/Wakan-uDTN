use std::{
    collections::{BTreeMap, BTreeSet},
    rc::Rc,
};

use qol::logy;

use crate::wakan::{NodeId, Radio, RecievedTime, Time, Transmission, WirelessNode};

use super::super::super::{gen_next_heartbeat_time, ScomsTreeNode, ScomsTreePacket};

impl WirelessNode<ScomsTreePacket> for ScomsTreeNode {
    fn tick(
        &mut self,
        now: Time,
        recieved_packets: Vec<(RecievedTime, Rc<ScomsTreePacket>, Radio)>,
    ) -> Result<Vec<Transmission<ScomsTreePacket>>, String> {
        // Create a list to store transmissions we might generate during this tick
        let mut transmissions = Vec::new();

        let mut merge_trees_ka = false;
        let mut heard_new_root_from_announcment_ka = false;

        ////////////////////////////
        // Process each received packet
        for (recieved_time, packet, radio) in recieved_packets {
            match packet.as_ref() {
                ScomsTreePacket::TreeMerge {
                    source,
                    new_root,
                    packet_id,
                } => {
                    logy!("debug", "\n\n{}:TreeMerge{source}:{packet_id}", self.id);
                    if let Some(my_parent) = &self.parent_maybe {
                        if source == my_parent {
                            let info = self.neighbors.get_mut(my_parent).unwrap();
                            if &info.lowest_accessible_thru <= new_root {
                                logy!(
                                    "info",
                                    "{}:found a new lowest ID from {}'s new root announcement in packet:{packet_id}",
                                    self.id,
                                    source,
                                );
                                heard_new_root_from_announcment_ka = true;
                                info.lowest_accessible_thru = new_root.clone();
                            } else {
                                logy!(
                                    "error",
                                    "{}:{} announced{packet_id} he found a new lowest id but it was higher than what we knew was their lowest id",
                                    self.id,
                                    source,
                                )
                            }
                        }
                    }
                }
                ScomsTreePacket::Beacon {
                    source,
                    neighbors,
                    parent_maybe,
                    packet_id,
                } => self.handle_beacon(
                    &mut merge_trees_ka,
                    neighbors,
                    parent_maybe.as_ref(),
                    &mut transmissions,
                    now,
                    *packet_id,
                    source,
                    &recieved_time,
                    &radio,
                ),
            }
        }

        ////////////////////////////
        // do any processing needed ths tick
        self.update(
            merge_trees_ka,
            heard_new_root_from_announcment_ka,
            &mut transmissions,
            now,
        );

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
                            (neighbor_id.clone(), info.lowest_accessible_thru.clone())
                        })
                        .collect(), // Share our known neighbor list
                    self.parent_maybe.clone(),
                    self.id.clone(), // Identify ourself as the sender
                    {
                        let x = self.send_packet_count.clone();
                        self.send_packet_count += 1;
                        x
                    },
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
            lowest_known_dirty: false,
            send_packet_count: 0,
        }
    }
}
