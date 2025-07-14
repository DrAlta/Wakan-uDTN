use std::collections::BTreeMap;

use crate::wakan::{wireless_nodes::scoms_tree_node::{find_lowest_id_lowest_accessable_thru_neighbor, types::NeighborInfo}, NodeId, Radio, ScomsTreeNode, Time};

impl ScomsTreeNode {
    pub fn handle_beacon(
        &mut self,
        source: &NodeId,
        neighbors: &BTreeMap<NodeId, NodeId>,
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

                //find the lowest id reachable tho

                // Update the cached view of this neighbor's neighbors
                let lowest_accesssable_thru= find_lowest_id_lowest_accessable_thru_neighbor(neighbors.iter(), source, &self.id);
                neighbor_info.neighbors_of_neighbor = neighbors.iter().map(
                    |(id, _lowest)|
                    id.clone()
                ).collect();
                neighbor_info.lowest_accessable_thru = lowest_accesssable_thru;
            }
            None => {
                // We've never seen this neighbor before.
                // Initialize tracking info for it.

                // Record the first time we saw it (from this radio)
                let first_seen = BTreeMap::from([(radio.clone(), recieved_time.clone())]);


                let lowest_accessable_thru= find_lowest_id_lowest_accessable_thru_neighbor(neighbors.iter(), source, &self.id);
                let neighbors_of_neighbor = neighbors.iter().map(
                    |(id, _lowest)|
                    id.clone()
                ).collect();


                // Insert this new neighbor into our neighbor map
                self.neighbors.insert(
                    source.clone(),
                    NeighborInfo {
                        last_seen: first_seen.clone(),
                        first_seen,
                        neighbors_of_neighbor,
                        lowest_accessable_thru,
                        
                    },
                );
            }
        }
    }
}
