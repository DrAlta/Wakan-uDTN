use std::collections::BTreeMap;

use qol::logy;

use crate::wakan::{
    wireless_nodes::scoms_tree_node::{
        find_lowest_id_lowest_accessible_thru_neighbor, types::NeighborInfo,
    },
    NodeId, Radio, ScomsTreeNode, ScomsTreePacket, Time, Transmission,
};

impl ScomsTreeNode {
    pub fn handle_beacon(
        &mut self,
        merge_trees_ka: &mut bool, // `merge_trees_ka` is part of the delay to give time for the lowest id in tree mergers to travel along the tree
        neighbors: &BTreeMap<NodeId, NodeId>,
        parent_maybe: Option<&NodeId>,
        transmissions: &mut Vec<Transmission<ScomsTreePacket>>,
        now: Time,
        source: &NodeId,
        recieved_time: &Time,
        radio: &Radio,
    ) {
        let mut old_lowest_id_known = source.clone();
        if let Some(parent) = parent_maybe {
            if parent == &self.id {
                if self.children.insert(source.clone()) {
                    logy!(
                        "trace-scoms-tree-node-handle-beacon",
                        "{:?} added {:?} as child",
                        self.id,
                        source
                    );
                }
            }
        }

        // Beacon packets advertise a node's presence and its known neighbors.
        // This helps build and update the network view.

        let lowest_accessible_thru;
        match self.neighbors.get_mut(source) {
            Some(neighbor_info) => {
                old_lowest_id_known = neighbor_info.lowest_id_known.clone();
                // This neighbor is already known.
                // Update the last time we saw it from this radio.
                neighbor_info
                    .last_seen
                    .insert(radio.clone(), recieved_time.clone());

                //find the lowest id reachable tho
                let lowest_known;
                // Update the cached view of this neighbor's neighbors
                (lowest_accessible_thru, lowest_known) =
                    find_lowest_id_lowest_accessible_thru_neighbor(
                        neighbors.iter(),
                        source,
                        &self.id,
                    );
                neighbor_info.neighbors_of_neighbor =
                    neighbors.iter().map(|(id, _lowest)| id.clone()).collect();
                neighbor_info.lowest_accessible_thru = lowest_accessible_thru.clone();
                neighbor_info.lowest_id_known = lowest_known;
            }
            None => {
                // We've never seen this neighbor before.
                // Initialize tracking info for it.

                // Record the first time we saw it (from this radio)
                let first_seen = BTreeMap::from([(radio.clone(), recieved_time.clone())]);

                let lowest_id_known;
                (lowest_accessible_thru, lowest_id_known) =
                    find_lowest_id_lowest_accessible_thru_neighbor(
                        neighbors.iter(),
                        source,
                        &self.id,
                    );
                let neighbors_of_neighbor =
                    neighbors.iter().map(|(id, _lowest)| id.clone()).collect();

                // Insert this new neighbor into our neighbor map
                self.neighbors.insert(
                    source.clone(),
                    NeighborInfo {
                        last_seen: first_seen.clone(),
                        first_seen,
                        neighbors_of_neighbor,
                        lowest_accessible_thru: lowest_accessible_thru.clone(),
                        lowest_id_known,
                    },
                );
            }
        }
        if lowest_accessible_thru > self.lowest_known_node_id {
            loop {
                if let Some(my_parent) = &self.parent_maybe {
                    if source == my_parent && self.lowest_known_dirty {
                        // this is a parent and lowestid known is dirty and the lowest id accesable thru
                        // them is still greater than the lowest id we knoe so set the flag to
                        // merge the trees
                        //println!("{} seting merger_trees_ka to true", self.id);
                        *merge_trees_ka = true;
                        break;
                    }
                }
                if self.lowest_known_node_id < old_lowest_id_known {
                    let packet = ScomsTreePacket::TreeMerge {
                        source: self.id.clone(),
                        new_root: self.lowest_known_node_id.clone(),
                    };
                    let transmittion = Transmission::new(now, packet, 0.into());
                    logy!("info", "{}: sending Treemerge", self.id);
                    transmissions.push(transmittion);
                }
                break;
            }
        }
    }
}
