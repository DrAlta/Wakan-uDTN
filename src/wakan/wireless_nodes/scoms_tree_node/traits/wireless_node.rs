use std::{
    collections::BTreeMap,
    rc::Rc,
};

use qol::logy;

use crate::wakan::{
    wireless_nodes::scoms_tree_node::{gen_next_heartbeat_time, MAX_AGE},
    NodeId, ScomsTreeNode, ScomsTreePacket, Radio, RecievedTime, Time, Transmission, WirelessNode,
};

impl WirelessNode<ScomsTreePacket> for ScomsTreeNode {
    fn tick(
        &mut self,
        now: Time,
        recieved_packets: Vec<(RecievedTime, Rc<ScomsTreePacket>, Radio)>,
    ) -> Result<Vec<Transmission<ScomsTreePacket>>, String> {
        // Create a list to store transmissions we might generate during this tick
        let mut transmissions = Vec::new();

        // Process each received packet
        for (recieved_time, packet, radio) in recieved_packets {
            match packet.as_ref() {
                // We're only handling Beacon packets here
                ScomsTreePacket::Beacon { source, neighbors } => {
                    self.handle_beacon(source, neighbors, &recieved_time, &radio)
                }
            }
        }
        // cutoff is the limit for consider nodes offline if we haven't heard from them 
        // on or after cutoff then we cosider they have gone offline
        let cutoff = now.saturating_sub(MAX_AGE);
        
        // neighbors_that_went_down will hold the ids of neighbors that went down
        let mut ids_neighbors_that_went_down = Vec::new();
        // we will only retain the naighbors that on online still
        self.neighbors.retain(
            |neighbor_id, neighbor_info|
            {
                // remove all times we heard them that was before the cutoff
                neighbor_info.last_seen.retain(|_, time| *time >= cutoff);
                // if there is no last_seen times left then they are offline
                if neighbor_info.last_seen.is_empty() {
                    // add theri id to list of neighbors that when down
                    ids_neighbors_that_went_down.push(neighbor_id.clone());
                    // return false so that it won't be retained
                    false
                } else {
                    // we had a last_seen left over after filtering out all the 
                    // ones before the cutoff, so return true to retain it
                    true
                }
            }
        );
        // update parent if needed
        // find out if we need a new_parent?
        let need_new_parent_ka = if let Some(parent_id) = &self.parent {
            let x = ids_neighbors_that_went_down.contains(parent_id);
            if x {
                logy!("trace-scoms-tree-node", "{:?}'s' parent, {parent_id:?}, went offline", self.id);
            };
            x
        } else {
            true
        };

        // if we need a new parent look for one
        if need_new_parent_ka {
            if self.parent.is_some() {
                logy!("info", "resetting {:?}'s parent", self.id);
            };
            // find the oldest neighbor we know of the neighbors that have the lowest ID accessable thru them.
            if let Some(oldest_id)= self.find_oldest_neighbor_that_the_lowest_id_can_be_accessed_thru(){
                // if we found out set it to are new parent
                logy!("trace-scoms-tree-node", "{:?} set its parent to {:?}", self.id, oldest_id);
                self.parent = Some(oldest_id.clone());
            }
        }

        // Check if it's time to send out a new beacon
        if now >= self.next_beacon {
            // Schedule the next heartbeat
            self.next_beacon = gen_next_heartbeat_time(self.next_beacon);

            // Construct a new Beacon packet advertising our own ID
            // and all known neighbors — this helps others learn about us.
            transmissions.push(Transmission::new(
                now + 1, // Add slight delay to allow channel scheduling
                ScomsTreePacket::new_beacon(
                    self.id.clone(),                                    // Identify ourself as the sender
                    self.neighbors.iter().map(
                        |(neighbor_id, info)| 
                        (neighbor_id.clone(), info.lowest_accessable_thru.clone())
                    ).collect(), // Share our known neighbor list
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
